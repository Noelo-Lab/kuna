// Function: usage @ 0x44b3

void usage(int param_1)

{
  FILE *pFVar1;
  undefined8 uVar2;
  char *pcVar3;
  
  uVar2 = DAT_00127608;
  if (param_1 == 0) {
    pcVar3 = (char *)gettext(
                            "Usage: %s [OPTION]... [-T] SOURCE DEST\n  or:  %s [OPTION]... SOURCE... DIRECTORY\n  or:  %s [OPTION]... -t DIRECTORY SOURCE...\n"
                            );
    printf(pcVar3,uVar2,uVar2,uVar2);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.\n");
    fputs_unlocked(pcVar3,pFVar1);
    FUN_00104199();
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -a, --archive                same as -dR --preserve=all\n      --attributes-only        don\'t copy the file data, just the attributes\n      --backup[=CONTROL]       make a backup of each existing destination file\n  -b                           like --backup but does not accept an argument\n      --copy-contents          copy contents of special files when recursive\n  -d                           same as --no-dereference --preserve=links\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -f, --force                  if an existing destination file cannot be\n                                 opened, remove it and try again (this option\n                                 is ignored when the -n option is also used)\n  -i, --interactive            prompt before overwrite (overrides a previous -n\n                                  option)\n  -H                           follow command-line symbolic links in SOURCE\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -l, --link                   hard link files instead of copying\n  -L, --dereference            always follow symbolic links in SOURCE\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -n, --no-clobber             do not overwrite an existing file (overrides\n                                 a previous -i option)\n  -P, --no-dereference         never follow symbolic links in SOURCE\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -p                           same as --preserve=mode,ownership,timestamps\n      --preserve[=ATTR_LIST]   preserve the specified attributes (default:\n                                 mode,ownership,timestamps), if possible\n                                 additional attributes: context, links, xattr,\n                                 all\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "      --no-preserve=ATTR_LIST  don\'t preserve the specified attributes\n      --parents                use full source file name under DIRECTORY\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -R, -r, --recursive          copy directories recursively\n      --reflink[=WHEN]         control clone/CoW copies. See below\n      --remove-destination     remove each existing destination file before\n                                 attempting to open it (contrast with --force)\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "      --sparse=WHEN            control creation of sparse files. See below\n      --strip-trailing-slashes  remove any trailing slashes from each SOURCE\n                                 argument\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -s, --symbolic-link          make symbolic links instead of copying\n  -S, --suffix=SUFFIX          override the usual backup suffix\n  -t, --target-directory=DIRECTORY  copy all SOURCE arguments into DIRECTORY\n  -T, --no-target-directory    treat DEST as a normal file\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -u, --update                 copy only when the SOURCE file is newer\n                                 than the destination file or when the\n                                 destination file is missing\n  -v, --verbose                explain what is being done\n  -x, --one-file-system        stay on this file system\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -Z                           set SELinux security context of destination\n                                 file to default type\n      --context[=CTX]          like -Z, or if CTX is specified then set the\n                                 SELinux or SMACK security context to CTX\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("      --help        display this help and exit\n");
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("      --version     output version information and exit\n");
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "\nBy default, sparse SOURCE files are detected by a crude heuristic and the\ncorresponding DEST file is made sparse as well.  That is the behavior\nselected by --sparse=auto.  Specify --sparse=always to create a sparse DEST\nfile whenever the SOURCE file contains a long enough sequence of zero bytes.\nUse --sparse=never to inhibit creation of sparse files.\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "\nWhen --reflink[=always] is specified, perform a lightweight copy, where the\ndata blocks are copied only when modified.  If this is not possible the copy\nfails, or if --reflink=auto is specified, fall back to a standard copy.\nUse --reflink=never to ensure a standard copy is performed.\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    FUN_001041ca();
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "\nAs a special case, cp makes a backup of SOURCE when the force and backup\noptions are given and SOURCE and DEST are the same name for an existing,\nregular file.\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    FUN_0010421c(&DAT_0011f592);
  }
  else {
    pcVar3 = (char *)gettext("Try \'%s --help\' for more information.\n");
    fprintf(stderr,pcVar3,uVar2);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: re_protect @ 0x4793

undefined8 re_protect(char *param_1,int param_2,long param_3,long param_4,long param_5)

{
  undefined4 uVar1;
  undefined4 uVar2;
  long lVar3;
  long lVar4;
  char *pcVar5;
  void *pvVar6;
  char cVar7;
  int iVar8;
  size_t sVar9;
  ulong uVar10;
  undefined8 uVar11;
  int *piVar12;
  undefined8 uVar13;
  undefined1 *puVar14;
  long in_FS_OFFSET;
  timespec tVar15;
  undefined1 auVar16 [16];
  undefined1 auStack_b8 [8];
  long local_b0;
  long local_a8;
  long local_a0;
  int local_94;
  char *local_90;
  undefined4 local_7c;
  long local_78;
  char *local_70;
  size_t local_68;
  void *local_60;
  void *local_58;
  char *local_50;
  timespec local_48;
  undefined1 local_38 [16];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_b0 = param_5;
  local_a8 = param_4;
  local_a0 = param_3;
  local_94 = param_2;
  local_90 = param_1;
  local_70 = param_1;
  sVar9 = strlen(param_1);
  local_68 = sVar9 + 1;
  uVar10 = ((sVar9 + 0x18) / 0x10) * 0x10;
  for (puVar14 = auStack_b8; puVar14 != auStack_b8 + -(uVar10 & 0xfffffffffffff000);
      puVar14 = puVar14 + -0x1000) {
    *(undefined8 *)(puVar14 + -8) = *(undefined8 *)(puVar14 + -8);
  }
  lVar3 = -(ulong)((uint)uVar10 & 0xfff);
  if ((uVar10 & 0xfff) != 0) {
    *(undefined8 *)(puVar14 + ((ulong)((uint)uVar10 & 0xfff) - 8) + lVar3) =
         *(undefined8 *)(puVar14 + ((ulong)((uint)uVar10 & 0xfff) - 8) + lVar3);
  }
  sVar9 = local_68;
  pcVar5 = local_70;
  local_60 = (void *)((ulong)(puVar14 + lVar3 + 0xf) & 0xfffffffffffffff0);
  *(undefined8 *)(puVar14 + lVar3 + -8) = 0x104895;
  local_58 = memcpy((void *)((ulong)(puVar14 + lVar3 + 0xf) & 0xfffffffffffffff0),pcVar5,sVar9);
  local_50 = (char *)((long)local_58 + (local_a0 - (long)local_90));
  local_78 = local_a8;
  do {
    lVar4 = local_78;
    if (local_78 == 0) {
      uVar13 = 1;
LAB_00104b33:
      if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        *(undefined8 *)(puVar14 + lVar3 + -8) = 0x104b47;
        __stack_chk_fail();
      }
      return uVar13;
    }
    *(undefined1 *)((long)local_58 + *(long *)(local_78 + 0x98)) = 0;
    if (*(char *)(local_b0 + 0x1f) != '\0') {
      *(undefined8 *)(puVar14 + lVar3 + -8) = 0x1048f9;
      tVar15 = (timespec)FUN_00119e57(lVar4);
      lVar4 = local_78;
      local_48 = tVar15;
      *(undefined8 *)(puVar14 + lVar3 + -8) = 0x10490d;
      auVar16 = FUN_00119e8b(lVar4);
      pcVar5 = local_50;
      iVar8 = local_94;
      local_38 = auVar16;
      *(undefined8 *)(puVar14 + lVar3 + -8) = 0x10492f;
      iVar8 = utimensat(iVar8,pcVar5,&local_48,0);
      pvVar6 = local_58;
      if (iVar8 != 0) {
        *(undefined8 *)(puVar14 + lVar3 + -8) = 0x104944;
        uVar13 = FUN_00118a0f(4,pvVar6);
        *(undefined8 *)(puVar14 + lVar3 + -8) = 0x104956;
        uVar11 = gettext("failed to preserve times for %s");
        *(undefined8 *)(puVar14 + lVar3 + -8) = 0x10495e;
        piVar12 = __errno_location();
        iVar8 = *piVar12;
        *(undefined8 *)(puVar14 + lVar3 + -8) = 0x104977;
        error(0,iVar8,uVar11,uVar13);
        uVar13 = 0;
        goto LAB_00104b33;
      }
    }
    pcVar5 = local_50;
    iVar8 = local_94;
    if (*(char *)(local_b0 + 0x1d) != '\0') {
      uVar1 = *(undefined4 *)(local_78 + 0x20);
      uVar2 = *(undefined4 *)(local_78 + 0x1c);
      *(undefined8 *)(puVar14 + lVar3 + -8) = 0x1049b3;
      iVar8 = FUN_001112c3(iVar8,pcVar5,uVar2,uVar1);
      lVar4 = local_b0;
      if (iVar8 != 0) {
        *(undefined8 *)(puVar14 + lVar3 + -8) = 0x1049ca;
        cVar7 = FUN_0010dc1f(lVar4);
        pcVar5 = local_50;
        pvVar6 = local_58;
        iVar8 = local_94;
        if (cVar7 != '\x01') {
          *(undefined8 *)(puVar14 + lVar3 + -8) = 0x1049e2;
          uVar13 = FUN_00118a0f(4,pvVar6);
          *(undefined8 *)(puVar14 + lVar3 + -8) = 0x1049f4;
          uVar11 = gettext("failed to preserve ownership for %s");
          *(undefined8 *)(puVar14 + lVar3 + -8) = 0x1049fc;
          piVar12 = __errno_location();
          iVar8 = *piVar12;
          *(undefined8 *)(puVar14 + lVar3 + -8) = 0x104a15;
          error(0,iVar8,uVar11,uVar13);
          uVar13 = 0;
          goto LAB_00104b33;
        }
        uVar1 = *(undefined4 *)(local_78 + 0x20);
        *(undefined8 *)(puVar14 + lVar3 + -8) = 0x104a3e;
        local_7c = FUN_001112c3(iVar8,pcVar5,0xffffffff,uVar1);
      }
    }
    pcVar5 = local_50;
    pvVar6 = local_58;
    iVar8 = local_94;
    if (*(char *)(local_b0 + 0x1e) == '\0') {
      if (*(char *)(local_78 + 0x90) != '\0') {
        uVar1 = *(undefined4 *)(local_78 + 0x18);
        *(undefined8 *)(puVar14 + lVar3 + -8) = 0x104ab0;
        iVar8 = FUN_00111260(iVar8,pcVar5,uVar1);
        pvVar6 = local_58;
        if (iVar8 != 0) {
          *(undefined8 *)(puVar14 + lVar3 + -8) = 0x104ac5;
          uVar13 = FUN_00118a0f(4,pvVar6);
          *(undefined8 *)(puVar14 + lVar3 + -8) = 0x104ad7;
          uVar11 = gettext("failed to preserve permissions for %s");
          *(undefined8 *)(puVar14 + lVar3 + -8) = 0x104adf;
          piVar12 = __errno_location();
          iVar8 = *piVar12;
          *(undefined8 *)(puVar14 + lVar3 + -8) = 0x104af8;
          error(0,iVar8,uVar11,uVar13);
          uVar13 = 0;
          goto LAB_00104b33;
        }
      }
    }
    else {
      uVar1 = *(undefined4 *)(local_78 + 0x18);
      *(undefined8 *)(puVar14 + lVar3 + -8) = 0x104a74;
      iVar8 = FUN_0010ec57(pcVar5,0xffffffff,pvVar6,0xffffffff,uVar1);
      if (iVar8 != 0) {
        uVar13 = 0;
        goto LAB_00104b33;
      }
    }
    *(undefined1 *)((long)local_58 + *(long *)(local_78 + 0x98)) = 0x2f;
    local_78 = *(long *)(local_78 + 0xa0);
  } while( true );
}




// Function: make_dir_parents_private @ 0x4b50

undefined8
make_dir_parents_private(char *param_1,ulong param_2,int param_3,char *param_4,__dev_t *param_5,char *param_6,
            long param_7)

{
  long lVar1;
  long lVar2;
  char *pcVar3;
  void *pvVar4;
  char *pcVar5;
  char cVar6;
  int iVar7;
  undefined4 uVar8;
  uint uVar9;
  undefined8 uVar10;
  size_t sVar11;
  ulong uVar12;
  int *piVar13;
  undefined8 uVar14;
  undefined1 *puVar15;
  undefined1 *puVar16;
  undefined1 *puVar17;
  long in_FS_OFFSET;
  undefined1 auStack_1f8 [8];
  long local_1f0;
  char *local_1e8;
  __dev_t *local_1e0;
  char *local_1d8;
  int local_1cc;
  ulong local_1c8;
  char *local_1c0;
  char local_1ad;
  uint local_1ac;
  int local_1a8;
  uint local_1a4;
  uint local_1a0;
  uint local_19c;
  char *local_198;
  char *local_190;
  __dev_t *local_188;
  ulong local_180;
  char *local_178;
  size_t local_170;
  void *local_168;
  void *local_160;
  char *local_158;
  void *local_150;
  stat local_148;
  stat local_b8;
  long local_20;
  
  puVar15 = auStack_1f8;
  local_1f0 = param_7;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_1e8 = param_6;
  local_1e0 = param_5;
  local_1d8 = param_4;
  local_1cc = param_3;
  local_1c8 = param_2;
  local_1c0 = param_1;
  local_180 = FUN_0011158b(param_1);
  *local_1e0 = 0;
  if (local_1c8 < local_180) {
    local_178 = local_1c0;
    sVar11 = strlen(local_1c0);
    local_170 = sVar11 + 1;
    uVar12 = ((sVar11 + 0x18) / 0x10) * 0x10;
    for (; puVar15 != auStack_1f8 + -(uVar12 & 0xfffffffffffff000); puVar15 = puVar15 + -0x1000) {
      *(undefined8 *)(puVar15 + -8) = *(undefined8 *)(puVar15 + -8);
    }
    lVar1 = -(ulong)((uint)uVar12 & 0xfff);
    puVar16 = puVar15 + lVar1;
    if ((uVar12 & 0xfff) != 0) {
      *(undefined8 *)(puVar15 + ((ulong)((uint)uVar12 & 0xfff) - 8) + lVar1) =
           *(undefined8 *)(puVar15 + ((ulong)((uint)uVar12 & 0xfff) - 8) + lVar1);
    }
    sVar11 = local_170;
    pcVar3 = local_178;
    local_168 = (void *)((ulong)(puVar15 + lVar1 + 0xf) & 0xfffffffffffffff0);
    *(undefined8 *)(puVar15 + lVar1 + -8) = 0x104cba;
    local_160 = memcpy((void *)((ulong)(puVar15 + lVar1 + 0xf) & 0xfffffffffffffff0),pcVar3,sVar11);
    local_158 = (char *)(local_1c8 + (long)local_160);
    uVar12 = ((local_180 + 0x18) / 0x10) * 0x10;
    for (; puVar16 != puVar15 + (lVar1 - (uVar12 & 0xfffffffffffff000)); puVar16 = puVar16 + -0x1000
        ) {
      *(undefined8 *)(puVar16 + -8) = *(undefined8 *)(puVar16 + -8);
    }
    lVar1 = -(ulong)((uint)uVar12 & 0xfff);
    puVar17 = puVar16 + lVar1;
    if ((uVar12 & 0xfff) != 0) {
      *(undefined8 *)(puVar16 + ((ulong)((uint)uVar12 & 0xfff) - 8) + lVar1) =
           *(undefined8 *)(puVar16 + ((ulong)((uint)uVar12 & 0xfff) - 8) + lVar1);
    }
    pvVar4 = local_160;
    uVar12 = local_180;
    local_150 = (void *)((ulong)(puVar16 + lVar1 + 0xf) & 0xfffffffffffffff0);
    *(undefined8 *)(puVar16 + lVar1 + -8) = 0x104d8c;
    memcpy((void *)((ulong)(puVar16 + lVar1 + 0xf) & 0xfffffffffffffff0),pvVar4,uVar12);
    iVar7 = local_1cc;
    *(undefined1 *)((long)local_150 + local_180) = 0;
    for (local_198 = (char *)(local_1c8 + (long)local_150); pcVar3 = local_198, *local_198 == '/';
        local_198 = local_198 + 1) {
    }
    *(undefined8 *)(puVar16 + lVar1 + -8) = 0x104df0;
    iVar7 = fstatat(iVar7,pcVar3,&local_148,0);
    pvVar4 = local_150;
    if (iVar7 == 0) {
      if ((local_148.st_mode & 0xf000) != 0x4000) {
        *(undefined8 *)(puVar16 + lVar1 + -8) = 0x1054c3;
        uVar10 = FUN_00118a0f(4,pvVar4);
        *(undefined8 *)(puVar16 + lVar1 + -8) = 0x1054d5;
        uVar14 = gettext("%s exists but is not a directory");
        *(undefined8 *)(puVar16 + lVar1 + -8) = 0x1054ef;
        error(0,0,uVar14,uVar10);
        uVar10 = 0;
        goto LAB_00105505;
      }
      *local_1e8 = '\0';
    }
    else {
      for (local_190 = local_158; *local_190 == '/'; local_190 = local_190 + 1) {
      }
      local_198 = local_190;
      while( true ) {
        pcVar3 = local_190;
        *(undefined8 *)(puVar16 + lVar1 + -8) = 0x105486;
        local_190 = strchr(pcVar3,0x2f);
        pcVar3 = local_198;
        iVar7 = local_1cc;
        if (local_190 == (char *)0x0) break;
        *local_190 = '\0';
        *(undefined8 *)(puVar16 + lVar1 + -8) = 0x104e5b;
        iVar7 = fstatat(iVar7,pcVar3,&local_148,0);
        pcVar3 = local_158;
        local_1ad = iVar7 != 0;
        if (((((bool)local_1ad) || (*(char *)(local_1f0 + 0x1d) != '\0')) ||
            (*(char *)(local_1f0 + 0x1e) != '\0')) || (*(char *)(local_1f0 + 0x1f) != '\0')) {
          *(undefined8 *)(puVar16 + lVar1 + -8) = 0x104eb9;
          iVar7 = stat(pcVar3,&local_b8);
          if (iVar7 == 0) {
            if ((local_b8.st_mode & 0xf000) == 0x4000) {
              local_1a8 = 0;
            }
            else {
              local_1a8 = 0x14;
            }
          }
          else {
            *(undefined8 *)(puVar16 + lVar1 + -8) = 0x104ec2;
            piVar13 = __errno_location();
            local_1a8 = *piVar13;
          }
          pcVar3 = local_158;
          if (local_1a8 != 0) {
            *(undefined8 *)(puVar16 + lVar1 + -8) = 0x104f07;
            uVar10 = FUN_00118a0f(4,pcVar3);
            *(undefined8 *)(puVar16 + lVar1 + -8) = 0x104f19;
            uVar14 = gettext("failed to get attributes of %s");
            iVar7 = local_1a8;
            *(undefined8 *)(puVar16 + lVar1 + -8) = 0x104f36;
            error(0,iVar7,uVar14,uVar10);
            uVar10 = 0;
            puVar17 = puVar16 + lVar1;
            goto LAB_00105505;
          }
          *(undefined8 *)(puVar16 + lVar1 + -8) = 0x104f4a;
          local_188 = (__dev_t *)FUN_0011b941(0xa8);
          *local_188 = local_b8.st_dev;
          local_188[1] = local_b8.st_ino;
          local_188[2] = local_b8.st_nlink;
          local_188[3] = CONCAT44(local_b8.st_uid,local_b8.st_mode);
          local_188[4] = local_b8._32_8_;
          local_188[5] = local_b8.st_rdev;
          local_188[6] = local_b8.st_size;
          local_188[7] = local_b8.st_blksize;
          local_188[8] = local_b8.st_blocks;
          local_188[9] = local_b8.st_atim.tv_sec;
          local_188[10] = local_b8.st_atim.tv_nsec;
          local_188[0xb] = local_b8.st_mtim.tv_sec;
          local_188[0xc] = local_b8.st_mtim.tv_nsec;
          local_188[0xd] = local_b8.st_ctim.tv_sec;
          local_188[0xe] = local_b8.st_ctim.tv_nsec;
          local_188[0xf] = local_b8.__unused[0];
          local_188[0x10] = local_b8.__unused[1];
          local_188[0x11] = local_b8.__unused[2];
          local_188[0x13] = (long)local_190 - (long)local_160;
          *(undefined1 *)(local_188 + 0x12) = 0;
          local_188[0x14] = *local_1e0;
          *local_1e0 = (__dev_t)local_188;
        }
        pcVar3 = local_158;
        pvVar4 = local_160;
        cVar6 = local_1ad;
        lVar2 = local_1f0;
        if (local_1ad == '\0') {
          uVar8 = 0;
        }
        else {
          uVar8 = (undefined4)local_188[3];
        }
        *(undefined8 *)(puVar16 + lVar1 + -8) = 0x105097;
        cVar6 = FUN_001084c6(pcVar3,pvVar4,uVar8,cVar6,lVar2);
        pvVar4 = local_160;
        pcVar3 = local_198;
        iVar7 = local_1cc;
        if (cVar6 != '\x01') {
          uVar10 = 0;
          puVar17 = puVar16 + lVar1;
          goto LAB_00105505;
        }
        if (local_1ad == '\0') {
          if ((local_148.st_mode & 0xf000) != 0x4000) {
            *(undefined8 *)(puVar16 + lVar1 + -8) = 0x10539a;
            uVar10 = FUN_00118a0f(4,pvVar4);
            *(undefined8 *)(puVar16 + lVar1 + -8) = 0x1053ac;
            uVar14 = gettext("%s exists but is not a directory");
            *(undefined8 *)(puVar16 + lVar1 + -8) = 0x1053c6;
            error(0,0,uVar14,uVar10);
            uVar10 = 0;
            puVar17 = puVar16 + lVar1;
            goto LAB_00105505;
          }
          *local_1e8 = '\0';
        }
        else {
          *local_1e8 = '\x01';
          local_1a4 = (uint)local_188[3];
          if (*(char *)(local_1f0 + 0x1d) == '\x01') {
            local_1ac = 0x3f;
          }
          else if (*(char *)(local_1f0 + 0x1e) == '\0') {
            local_1ac = 0;
          }
          else {
            local_1ac = 0x12;
          }
          local_1ac = local_1ac & local_1a4;
          uVar9 = local_1a4;
          if (*(char *)(local_1f0 + 0x20) != '\0') {
            uVar9 = 0x1ff;
          }
          uVar9 = ~local_1ac & uVar9 & 0xfff;
          local_1a0 = uVar9;
          *(undefined8 *)(puVar16 + lVar1 + -8) = 0x105167;
          iVar7 = mkdirat(iVar7,pcVar3,uVar9);
          pcVar5 = local_158;
          pvVar4 = local_160;
          pcVar3 = local_1d8;
          if (iVar7 != 0) {
            *(undefined8 *)(puVar16 + lVar1 + -8) = 0x10517f;
            uVar10 = FUN_00118a0f(4,pvVar4);
            *(undefined8 *)(puVar16 + lVar1 + -8) = 0x105191;
            uVar14 = gettext("cannot make directory %s");
            *(undefined8 *)(puVar16 + lVar1 + -8) = 0x105199;
            piVar13 = __errno_location();
            iVar7 = *piVar13;
            *(undefined8 *)(puVar16 + lVar1 + -8) = 0x1051b2;
            error(0,iVar7,uVar14,uVar10);
            uVar10 = 0;
            puVar17 = puVar16 + lVar1;
            goto LAB_00105505;
          }
          if (local_1d8 != (char *)0x0) {
            *(undefined8 *)(puVar16 + lVar1 + -8) = 0x1051eb;
            printf(pcVar3,pcVar5,pvVar4);
          }
          pcVar3 = local_198;
          iVar7 = local_1cc;
          *(undefined8 *)(puVar16 + lVar1 + -8) = 0x10520b;
          iVar7 = fstatat(iVar7,pcVar3,&local_148,0x100);
          pvVar4 = local_160;
          if (iVar7 != 0) {
            *(undefined8 *)(puVar16 + lVar1 + -8) = 0x105223;
            uVar10 = FUN_00118a0f(4,pvVar4);
            *(undefined8 *)(puVar16 + lVar1 + -8) = 0x105235;
            uVar14 = gettext("failed to get attributes of %s");
            *(undefined8 *)(puVar16 + lVar1 + -8) = 0x10523d;
            piVar13 = __errno_location();
            iVar7 = *piVar13;
            *(undefined8 *)(puVar16 + lVar1 + -8) = 0x105256;
            error(0,iVar7,uVar14,uVar10);
            uVar10 = 0;
            puVar17 = puVar16 + lVar1;
            goto LAB_00105505;
          }
          if (*(char *)(local_1f0 + 0x1e) != '\x01') {
            if ((~local_148.st_mode & local_1ac) != 0) {
              *(undefined8 *)(puVar16 + lVar1 + -8) = 0x105289;
              uVar9 = FUN_0010dcaf();
              local_1ac = local_1ac & ~uVar9;
            }
            if (((~local_148.st_mode & local_1ac) != 0) || ((local_148.st_mode & 0x1c0) != 0x1c0)) {
              *(__mode_t *)(local_188 + 3) = local_148.st_mode | local_1ac;
              *(undefined1 *)(local_188 + 0x12) = 1;
            }
          }
          pcVar3 = local_198;
          iVar7 = local_1cc;
          uVar9 = local_148.st_mode | 0x1c0;
          local_19c = uVar9;
          if (uVar9 != local_148.st_mode) {
            *(undefined8 *)(puVar16 + lVar1 + -8) = 0x10531b;
            iVar7 = FUN_00111260(iVar7,pcVar3,uVar9);
            pvVar4 = local_160;
            if (iVar7 != 0) {
              *(undefined8 *)(puVar16 + lVar1 + -8) = 0x105337;
              uVar10 = FUN_00118a0f(4,pvVar4);
              *(undefined8 *)(puVar16 + lVar1 + -8) = 0x105349;
              uVar14 = gettext("setting permissions for %s");
              *(undefined8 *)(puVar16 + lVar1 + -8) = 0x105351;
              piVar13 = __errno_location();
              iVar7 = *piVar13;
              *(undefined8 *)(puVar16 + lVar1 + -8) = 0x10536a;
              error(0,iVar7,uVar14,uVar10);
              uVar10 = 0;
              puVar17 = puVar16 + lVar1;
              goto LAB_00105505;
            }
          }
        }
        pvVar4 = local_160;
        lVar2 = local_1f0;
        if ((*local_1e8 != '\x01') &&
           ((*(long *)(local_1f0 + 0x28) != 0 || (*(char *)(local_1f0 + 0x33) != '\0')))) {
          *(undefined8 *)(puVar16 + lVar1 + -8) = 0x105425;
          cVar6 = FUN_00108750(pvVar4,0,lVar2);
          if ((cVar6 != '\x01') && (*(char *)(local_1f0 + 0x34) != '\0')) {
            uVar10 = 0;
            puVar17 = puVar16 + lVar1;
            goto LAB_00105505;
          }
        }
        *local_190 = '/';
        do {
          local_190 = local_190 + 1;
        } while (*local_190 == '/');
      }
    }
    uVar10 = 1;
    puVar17 = puVar16 + lVar1;
  }
  else {
    uVar10 = 1;
    puVar17 = auStack_1f8;
  }
LAB_00105505:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar10;
  }
                    /* WARNING: Subroutine does not return */
  *(undefined8 *)(puVar17 + -8) = 0x105519;
  __stack_chk_fail();
}




// Function: do_copy @ 0x5522

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

byte do_copy(int param_1,undefined8 *param_2,long param_3,char param_4,ulong *param_5)

{
  long lVar1;
  ulong *puVar2;
  undefined4 uVar3;
  void *pvVar4;
  char *pcVar5;
  char cVar6;
  byte bVar7;
  int iVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  int *piVar11;
  size_t sVar12;
  ulong uVar13;
  void *pvVar14;
  long lVar15;
  char *pcVar16;
  undefined1 *puVar17;
  ulong **ppuVar18;
  long in_FS_OFFSET;
  ulong *local_178;
  long local_170;
  undefined8 *local_168;
  char local_160;
  int local_15c;
  byte local_14c;
  undefined1 local_14b;
  bool local_14a;
  char local_149;
  undefined4 local_148;
  int local_144;
  undefined4 local_140;
  int local_13c;
  void *local_138;
  char *local_130;
  void *local_128;
  char *local_120;
  long local_118;
  char *local_110;
  char *local_108;
  char *local_100;
  size_t local_f8;
  void *local_f0;
  char *local_e8;
  char *local_e0;
  size_t local_d8;
  void *local_d0;
  void *local_c8;
  void *local_c0;
  stat local_b8;
  long local_20;
  
  ppuVar18 = &local_178;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_14c = 0;
  local_14a = true;
  local_178 = param_5;
  local_170 = param_3;
  local_168 = param_2;
  local_160 = param_4;
  local_15c = param_1;
  if (param_1 <= (int)(uint)(param_3 == 0)) {
    if (param_1 < 1) {
      uVar9 = gettext("missing file operand");
      error(0,0,uVar9);
    }
    else {
      uVar9 = FUN_00118a0f(4,*param_2);
      uVar10 = gettext("missing destination file operand after %s");
      error(0,0,uVar10,uVar9);
    }
    FUN_001044b3(1);
  }
  local_b8.st_mode = 0;
  local_148 = 0xffffff9c;
  if (local_160 != '\0') {
    if (local_170 != 0) {
      uVar9 = gettext("cannot combine --target-directory (-t) and --no-target-directory (-T)");
      error(1,0,uVar9);
    }
    if (local_15c < 3) goto LAB_0010581f;
    uVar9 = FUN_00118a0f(4,local_168[2]);
    uVar10 = gettext("extra operand %s");
    error(0,0,uVar10,uVar9);
    FUN_001044b3(1);
  }
  if (local_170 != 0) {
    local_148 = FUN_00119f57(local_170,&local_b8);
    cVar6 = FUN_00119edf(local_148);
    if (cVar6 == '\x01') goto LAB_0010581f;
    uVar9 = FUN_00118a0f(4,local_170);
    uVar10 = gettext("target directory %s");
    piVar11 = __errno_location();
    error(1,*piVar11,uVar10,uVar9);
  }
  local_118 = local_168[(long)local_15c + -1];
  local_140 = FUN_00119f57(local_118,&local_b8);
  cVar6 = FUN_00119edf(local_140);
  if (cVar6 == '\0') {
    piVar11 = __errno_location();
    local_13c = *piVar11;
    if (local_13c == 2) {
      local_14c = 1;
    }
    if (2 < local_15c) {
      uVar9 = FUN_00118a0f(4,local_118);
      uVar10 = gettext("target %s");
      error(1,local_13c,uVar10,uVar9);
    }
  }
  else {
    local_148 = local_140;
    local_170 = local_118;
    local_15c = local_15c + -1;
  }
LAB_0010581f:
  if (local_170 == 0) {
    local_110 = (char *)*local_168;
    local_120 = (char *)local_168[1];
    if (DAT_00127121 != '\0') {
      uVar9 = gettext("with --parents, the destination must be a directory");
      error(0,0,uVar9);
      FUN_001044b3(1);
    }
    if (((((*(char *)((long)local_178 + 0x16) != '\0') && ((int)*local_178 != 0)) &&
         (iVar8 = strcmp(local_110,local_120), iVar8 == 0)) &&
        ((local_14c != 1 &&
         ((local_b8.st_mode != 0 || (iVar8 = stat(local_120,&local_b8), iVar8 == 0)))))) &&
       ((local_b8.st_mode & 0xf000) == 0x8000)) {
      local_120 = (char *)FUN_0011024d(0xffffff9c,local_120,(int)*local_178);
      _DAT_00127148 = local_178[1];
      _DAT_00127150 = local_178[2];
      _DAT_00127158 = local_178[3];
      _DAT_00127160 = local_178[4];
      _DAT_00127168 = local_178[5];
      _DAT_00127170 = local_178[6];
      _DAT_00127178 = local_178[7];
      _DAT_00127180 = local_178[8];
      _DAT_00127188 = local_178[9];
      _DAT_00127190 = local_178[10];
      _DAT_00127140 = *local_178 & 0xffffffff00000000;
      local_178 = (ulong *)&DAT_00127140;
    }
    local_14a = (bool)FUN_0010daea(local_110,local_120,0xffffff9c,local_120,-(uint)local_14c,
                                   local_178,&local_130,0);
    ppuVar18 = &local_178;
  }
  else {
    if (1 < local_15c) {
      FUN_0010a549(local_178);
      FUN_0010a5a6(local_178);
    }
    for (local_144 = 0; local_144 < local_15c; local_144 = local_144 + 1) {
      local_149 = '\x01';
      local_130 = (char *)0x0;
      pcVar16 = (char *)local_168[local_144];
      local_108 = pcVar16;
      if (DAT_00127122 != '\0') {
        *(undefined8 *)((long)ppuVar18 + -8) = 0x1058b3;
        FUN_001116ab(pcVar16);
      }
      pcVar16 = local_108;
      if (DAT_00127121 == '\0') {
        *(undefined8 *)((long)ppuVar18 + -8) = 0x105a7a;
        pcVar16 = (char *)FUN_00110369(pcVar16);
        local_100 = pcVar16;
        *(undefined8 *)((long)ppuVar18 + -8) = 0x105a90;
        sVar12 = strlen(pcVar16);
        local_f8 = sVar12 + 1;
        uVar13 = ((sVar12 + 0x18) / 0x10) * 0x10;
        for (puVar17 = (undefined1 *)ppuVar18;
            puVar17 != (undefined1 *)((long)ppuVar18 + -(uVar13 & 0xfffffffffffff000));
            puVar17 = puVar17 + -0x1000) {
          *(undefined8 *)(puVar17 + -8) = *(undefined8 *)(puVar17 + -8);
        }
        lVar1 = -(ulong)((uint)uVar13 & 0xfff);
        ppuVar18 = (ulong **)(puVar17 + lVar1);
        if ((uVar13 & 0xfff) != 0) {
          *(undefined8 *)(puVar17 + ((ulong)((uint)uVar13 & 0xfff) - 8) + lVar1) =
               *(undefined8 *)(puVar17 + ((ulong)((uint)uVar13 & 0xfff) - 8) + lVar1);
        }
        sVar12 = local_f8;
        pcVar16 = local_100;
        local_f0 = (void *)((ulong)(puVar17 + lVar1 + 0xf) & 0xfffffffffffffff0);
        *(undefined8 *)(puVar17 + lVar1 + -8) = 0x105b4a;
        pcVar16 = memcpy((void *)((ulong)(puVar17 + lVar1 + 0xf) & 0xfffffffffffffff0),pcVar16,
                         sVar12);
        local_e8 = pcVar16;
        *(undefined8 *)(puVar17 + lVar1 + -8) = 0x105b60;
        FUN_001116ab(pcVar16);
        pcVar16 = local_e8;
        *(undefined8 *)(puVar17 + lVar1 + -8) = 0x105b79;
        iVar8 = strcmp(pcVar16,"..");
        lVar15 = local_170;
        pcVar16 = local_e8 + (iVar8 == 0);
        local_e8 = pcVar16;
        *(undefined8 *)(puVar17 + lVar1 + -8) = 0x105ba8;
        local_128 = (void *)FUN_00111da5(lVar15,pcVar16,&local_130);
      }
      else {
        local_e0 = local_108;
        *(undefined8 *)((long)ppuVar18 + -8) = 0x1058df;
        sVar12 = strlen(pcVar16);
        local_d8 = sVar12 + 1;
        uVar13 = ((sVar12 + 0x18) / 0x10) * 0x10;
        for (puVar17 = (undefined1 *)ppuVar18;
            puVar17 != (undefined1 *)((long)ppuVar18 + -(uVar13 & 0xfffffffffffff000));
            puVar17 = puVar17 + -0x1000) {
          *(undefined8 *)(puVar17 + -8) = *(undefined8 *)(puVar17 + -8);
        }
        lVar1 = -(ulong)((uint)uVar13 & 0xfff);
        ppuVar18 = (ulong **)(puVar17 + lVar1);
        if ((uVar13 & 0xfff) != 0) {
          *(undefined8 *)(puVar17 + ((ulong)((uint)uVar13 & 0xfff) - 8) + lVar1) =
               *(undefined8 *)(puVar17 + ((ulong)((uint)uVar13 & 0xfff) - 8) + lVar1);
        }
        sVar12 = local_d8;
        pcVar16 = local_e0;
        local_d0 = (void *)((ulong)(puVar17 + lVar1 + 0xf) & 0xfffffffffffffff0);
        *(undefined8 *)(puVar17 + lVar1 + -8) = 0x105999;
        pvVar14 = memcpy((void *)((ulong)(puVar17 + lVar1 + 0xf) & 0xfffffffffffffff0),pcVar16,
                         sVar12);
        local_c8 = pvVar14;
        *(undefined8 *)(puVar17 + lVar1 + -8) = 0x1059af;
        FUN_001116ab(pvVar14);
        pvVar14 = local_c8;
        lVar15 = local_170;
        *(undefined8 *)(puVar17 + lVar1 + -8) = 0x1059cf;
        pvVar14 = (void *)FUN_00111da5(lVar15,pvVar14,&local_130);
        uVar3 = local_148;
        if (*(char *)((long)local_178 + 0x3c) == '\0') {
          pcVar16 = (char *)0x0;
        }
        else {
          pcVar16 = "%s -> %s\n";
        }
        lVar15 = (long)local_130 - (long)pvVar14;
        local_128 = pvVar14;
        *(ulong **)(puVar17 + lVar1 + -0x10) = local_178;
        *(undefined8 *)(puVar17 + lVar1 + -0x18) = 0x105a3a;
        local_149 = FUN_00104b50(pvVar14,lVar15,uVar3,pcVar16,&local_138,&local_14c);
        for (; *local_130 == '/'; local_130 = local_130 + 1) {
        }
      }
      pcVar5 = local_108;
      pvVar14 = local_128;
      pcVar16 = local_130;
      uVar3 = local_148;
      bVar7 = local_14c;
      puVar2 = local_178;
      if (local_149 == '\x01') {
        *(undefined8 *)((long)ppuVar18 + -8) = 0;
        *(undefined1 **)((long)ppuVar18 + -0x10) = &local_14b;
        *(undefined8 *)((long)ppuVar18 + -0x18) = 0x105c08;
        bVar7 = FUN_0010daea(pcVar5,pvVar14,uVar3,pcVar16,bVar7,puVar2);
        pvVar4 = local_128;
        pcVar16 = local_130;
        pvVar14 = local_138;
        uVar3 = local_148;
        puVar2 = local_178;
        local_14a = (bVar7 & local_14a) != 0;
        if (DAT_00127121 != '\0') {
          *(undefined8 *)((long)ppuVar18 + -8) = 0x105c5b;
          bVar7 = FUN_00104793(pvVar4,uVar3,pcVar16,pvVar14,puVar2);
          local_14a = (bVar7 & local_14a) != 0;
        }
      }
      else {
        local_14a = false;
      }
      if (DAT_00127121 != '\0') {
        while (pvVar14 = local_138, local_138 != (void *)0x0) {
          local_c0 = local_138;
          local_138 = *(void **)((long)local_138 + 0xa0);
          *(undefined8 *)((long)ppuVar18 + -8) = 0x105cb1;
          free(pvVar14);
        }
      }
      pvVar14 = local_128;
      *(undefined8 *)((long)ppuVar18 + -8) = 0x105ccc;
      free(pvVar14);
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_14a;
  }
                    /* WARNING: Subroutine does not return */
  *(undefined8 *)((long)ppuVar18 + -8) = 0x105f08;
  __stack_chk_fail();
}




// Function: cp_option_init @ 0x5f11

void cp_option_init(long param_1)

{
  char *pcVar1;
  
  FUN_0010dbcb(param_1);
  *(undefined1 *)(param_1 + 0x14) = 1;
  *(undefined4 *)(param_1 + 4) = 1;
  *(undefined1 *)(param_1 + 0x15) = 0;
  *(undefined1 *)(param_1 + 0x16) = 0;
  *(undefined1 *)(param_1 + 0x17) = 0;
  *(undefined4 *)(param_1 + 8) = 4;
  *(undefined1 *)(param_1 + 0x18) = 0;
  *(undefined1 *)(param_1 + 0x19) = 0;
  *(undefined1 *)(param_1 + 0x1c) = 0;
  *(undefined4 *)(param_1 + 0x44) = 1;
  *(undefined1 *)(param_1 + 0x1d) = 0;
  *(undefined1 *)(param_1 + 0x30) = 0;
  *(undefined1 *)(param_1 + 0x1e) = 0;
  *(undefined1 *)(param_1 + 0x1f) = 0;
  *(undefined1 *)(param_1 + 0x20) = 0;
  *(undefined1 *)(param_1 + 0x33) = 0;
  *(undefined1 *)(param_1 + 0x34) = 0;
  *(undefined8 *)(param_1 + 0x28) = 0;
  *(undefined1 *)(param_1 + 0x35) = 0;
  *(undefined1 *)(param_1 + 0x37) = 0;
  *(undefined1 *)(param_1 + 0x36) = 0;
  *(undefined1 *)(param_1 + 0x31) = 1;
  *(undefined1 *)(param_1 + 0x32) = 0;
  *(undefined1 *)(param_1 + 0x38) = 0;
  *(undefined4 *)(param_1 + 0xc) = 2;
  *(undefined1 *)(param_1 + 0x3a) = 0;
  *(undefined1 *)(param_1 + 0x39) = 0;
  *(undefined4 *)(param_1 + 0x10) = 0;
  *(undefined1 *)(param_1 + 0x3d) = 0;
  *(undefined1 *)(param_1 + 0x3b) = 0;
  *(undefined1 *)(param_1 + 0x3c) = 0;
  pcVar1 = getenv("POSIXLY_CORRECT");
  *(bool *)(param_1 + 0x3e) = pcVar1 != (char *)0x0;
  *(undefined8 *)(param_1 + 0x48) = 0;
  *(undefined8 *)(param_1 + 0x50) = 0;
  return;
}




// Function: decode_preserve_arg @ 0x606f

void decode_preserve_arg(undefined8 param_1,long param_2,char param_3)

{
  char *__ptr;
  char *pcVar1;
  long lVar2;
  char *local_20;
  char *local_18;
  
  __ptr = (char *)FUN_0011bf48(param_1);
  local_20 = __ptr;
  do {
    pcVar1 = strchr(local_20,0x2c);
    local_18 = pcVar1;
    if (pcVar1 != (char *)0x0) {
      local_18 = pcVar1 + 1;
      *pcVar1 = '\0';
    }
    if (param_3 == '\0') {
      pcVar1 = "--no-preserve";
    }
    else {
      pcVar1 = "--preserve";
    }
    lVar2 = FUN_0010f72a(pcVar1,local_20,&PTR_DAT_00126840,&DAT_0011fb60,4,PTR_FUN_00127028,1);
    switch(*(undefined4 *)(&DAT_0011fb60 + lVar2 * 4)) {
    case 0:
      *(char *)(param_2 + 0x1e) = param_3;
      *(bool *)(param_2 + 0x20) = param_3 == '\0';
      break;
    case 1:
      *(char *)(param_2 + 0x1f) = param_3;
      break;
    case 2:
      *(char *)(param_2 + 0x1d) = param_3;
      break;
    case 3:
      *(char *)(param_2 + 0x30) = param_3;
      break;
    case 4:
      *(char *)(param_2 + 0x34) = param_3;
      *(char *)(param_2 + 0x33) = param_3;
      break;
    case 5:
      *(char *)(param_2 + 0x35) = param_3;
      *(char *)(param_2 + 0x36) = param_3;
      break;
    case 6:
      *(char *)(param_2 + 0x1e) = param_3;
      *(char *)(param_2 + 0x1f) = param_3;
      *(char *)(param_2 + 0x1d) = param_3;
      *(char *)(param_2 + 0x30) = param_3;
      *(bool *)(param_2 + 0x20) = param_3 == '\0';
      if (DAT_00127120 != '\0') {
        *(char *)(param_2 + 0x33) = param_3;
      }
      *(char *)(param_2 + 0x35) = param_3;
      break;
    default:
                    /* WARNING: Subroutine does not return */
      abort();
    }
    local_20 = local_18;
  } while (local_18 != (char *)0x0);
  free(__ptr);
  return;
}




// Function: main @ 0x6276

byte main(int param_1,undefined8 *param_2)

{
  bool bVar1;
  byte bVar2;
  int iVar3;
  long lVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  int *piVar7;
  long in_FS_OFFSET;
  undefined1 local_9f;
  undefined1 local_9e;
  long local_98;
  long local_90;
  long local_88;
  long local_80;
  undefined4 local_78;
  int local_74;
  int local_70;
  int local_6c;
  undefined1 local_64;
  undefined1 local_63;
  undefined1 local_62;
  char local_61;
  undefined1 local_5c;
  undefined1 local_5b;
  undefined1 local_5a;
  undefined1 local_59;
  long local_50;
  undefined1 local_48;
  undefined1 local_47;
  undefined1 local_46;
  char local_45;
  char local_44;
  undefined1 local_43;
  undefined1 local_41;
  char local_40;
  char local_3e;
  undefined1 local_3d;
  undefined1 local_3c;
  int local_34;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  bVar1 = false;
  local_98 = 0;
  local_90 = 0;
  local_9f = 0;
  local_88 = 0;
  local_9e = 0;
  local_80 = 0;
  FUN_00116d05(*param_2);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  FUN_0011d610(FUN_00111312);
  iVar3 = is_selinux_enabled();
  DAT_00127120 = 0 < iVar3;
  FUN_00105f11(&local_78);
LAB_001067ab:
  do {
    while( true ) {
      iVar3 = getopt_long(param_1,param_2,"abdfHilLnprst:uvxPRS:TZ",&PTR_s_archive_00126480,0);
      if (iVar3 == -1) {
        if ((local_61 != '\0') && (local_3e != '\0')) {
          uVar5 = gettext("cannot make both hard and symbolic links");
          error(0,0,uVar5);
          FUN_001044b3(1);
        }
        if (local_70 == 2) {
          local_3d = 0;
        }
        if ((bVar1) && (local_70 == 2)) {
          uVar5 = gettext("options --backup and --no-clobber are mutually exclusive");
          error(0,0,uVar5);
          FUN_001044b3(1);
        }
        if ((local_34 == 2) && (local_6c != 2)) {
          uVar5 = gettext("--reflink can be used only with --sparse=auto");
          error(0,0,uVar5);
          FUN_001044b3(1);
        }
        if (bVar1) {
          uVar5 = gettext("backup type");
          local_78 = FUN_0011030b(uVar5,local_90);
        }
        else {
          local_78 = 0;
        }
        FUN_0010f869(local_98);
        if (local_74 == 1) {
          if ((local_40 == '\0') || (local_61 == '\x01')) {
            local_74 = 4;
          }
          else {
            local_74 = 2;
          }
        }
        if (local_40 != '\0') {
          local_64 = local_9f;
        }
        if (((local_50 != 0) || (local_80 != 0)) && (local_44 != '\x01')) {
          local_45 = '\0';
        }
        if ((local_45 != '\0') && ((local_50 != 0 || (local_80 != 0)))) {
          uVar5 = gettext("cannot set target context and preserve it");
          error(1,0,uVar5);
        }
        if ((local_44 != '\0') && (DAT_00127120 != '\x01')) {
          uVar5 = gettext("cannot preserve security context without an SELinux-enabled kernel");
          error(1,0,uVar5);
        }
        if ((local_80 != 0) && (iVar3 = setfscreatecon(local_80), iVar3 < 0)) {
          uVar5 = FUN_00118e56(local_80);
          uVar6 = gettext("failed to set default file creation context to %s");
          piVar7 = __errno_location();
          error(1,*piVar7,uVar6,uVar5);
        }
        FUN_0010df3d();
        bVar2 = FUN_00105522(param_1 - optind,param_2 + optind,local_88,local_9e,&local_78);
        if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
          __stack_chk_fail();
        }
        return bVar2 ^ 1;
      }
      if (iVar3 < 0x89) break;
switchD_001063cd_caseD_49:
      FUN_001044b3(1);
    }
    if (iVar3 < 0x48) {
      if (iVar3 == -0x83) {
LAB_0010674f:
        FUN_0011b6df(stdout,&DAT_0011f592,"GNU coreutils",PTR_DAT_00127020,"Torbjorn Granlund",
                     "David MacKenzie","Jim Meyering",0);
                    /* WARNING: Subroutine does not return */
        exit(0);
      }
      if (iVar3 == -0x82) {
        FUN_001044b3(0);
        goto LAB_0010674f;
      }
      goto switchD_001063cd_caseD_49;
    }
    switch(iVar3) {
    case 0x48:
      local_74 = 3;
      break;
    default:
      goto switchD_001063cd_caseD_49;
    case 0x4c:
      local_74 = 4;
      break;
    case 0x50:
      local_74 = 2;
      break;
    case 0x52:
    case 0x72:
      local_40 = '\x01';
      break;
    case 0x53:
      bVar1 = true;
      local_98 = optarg;
      break;
    case 0x54:
      local_9e = 1;
      break;
    case 0x5a:
      if (DAT_00127120 == '\0') {
        if (optarg != 0) {
          uVar5 = gettext("warning: ignoring --context; it requires an SELinux-enabled kernel");
          error(0,0,uVar5);
        }
      }
      else if (optarg == 0) {
        local_50 = selabel_open(0,0,0);
        if (local_50 == 0) {
          uVar5 = gettext("warning: ignoring --context");
          piVar7 = __errno_location();
          error(0,*piVar7,uVar5);
        }
      }
      else {
        local_80 = optarg;
      }
      break;
    case 0x61:
      local_74 = 2;
      local_48 = 1;
      local_5b = 1;
      local_5a = 1;
      local_59 = 1;
      local_46 = 1;
      if (DAT_00127120 != '\0') {
        local_45 = '\x01';
      }
      local_43 = 1;
      local_41 = 1;
      local_40 = '\x01';
      break;
    case 0x62:
      bVar1 = true;
      if (optarg != 0) {
        local_90 = optarg;
      }
      break;
    case 100:
      local_48 = 1;
      local_74 = 2;
      break;
    case 0x66:
      local_62 = 1;
      break;
    case 0x69:
      local_70 = 3;
      break;
    case 0x6c:
      local_61 = '\x01';
      break;
    case 0x6e:
      local_70 = 2;
      break;
    case 0x73:
      local_3e = '\x01';
      break;
    case 0x74:
      if (local_88 != 0) {
        uVar5 = gettext("multiple target directories specified");
        error(1,0,uVar5);
      }
      local_88 = optarg;
      break;
    case 0x75:
      local_3d = 1;
      break;
    case 0x76:
      local_3c = 1;
      break;
    case 0x78:
      local_5c = 1;
      break;
    case 0x80:
      local_47 = 0;
      break;
    case 0x81:
      local_9f = 1;
      break;
    case 0x82:
      FUN_0010606f(optarg,&local_78,0);
      break;
    case 0x83:
      DAT_00127121 = 1;
      break;
    case 0x84:
      if (optarg != 0) {
        FUN_0010606f(optarg,&local_78,1);
        local_46 = 1;
        break;
      }
    case 0x70:
      local_5b = 1;
      local_5a = 1;
      local_59 = 1;
      local_46 = 1;
      break;
    case 0x85:
      if (optarg == 0) {
        local_34 = 2;
      }
      else {
        lVar4 = FUN_0010f72a("--reflink",optarg,&PTR_DAT_00126460,&DAT_0011e3b0,4,PTR_FUN_00127028,1
                            );
        local_34 = *(int *)(&DAT_0011e3b0 + lVar4 * 4);
      }
      break;
    case 0x86:
      lVar4 = FUN_0010f72a("--sparse",optarg,&PTR_s_never_00126440,&DAT_0011e3a0,4,PTR_FUN_00127028,
                           1);
      local_6c = *(int *)(&DAT_0011e3a0 + lVar4 * 4);
      break;
    case 0x87:
      DAT_00127122 = 1;
      break;
    case 0x88:
      goto switchD_001063cd_caseD_88;
    }
  } while( true );
switchD_001063cd_caseD_88:
  local_63 = 1;
  goto LAB_001067ab;
}




// Function: follow_fstatat @ 0x6cbb

int follow_fstatat(int param_1,char *param_2,stat *param_3,int param_4)

{
  long lVar1;
  int iVar2;
  long in_FS_OFFSET;
  
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = fstatat(param_1,param_2,param_3,param_4);
  if (lVar1 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return iVar2;
}




// Function: punch_hole @ 0x6d36

int punch_hole(int param_1,__off_t param_2,__off_t param_3)

{
  char cVar1;
  int *piVar2;
  int local_c;
  
  local_c = fallocate(param_1,3,param_2,param_3);
  if (local_c < 0) {
    piVar2 = __errno_location();
    cVar1 = FUN_00106bb9(*piVar2);
    if ((cVar1 == '\0') && (piVar2 = __errno_location(), *piVar2 != 0x26)) {
      return local_c;
    }
    local_c = 0;
  }
  return local_c;
}




// Function: create_hole @ 0x6d9e

undefined8 create_hole(int param_1,undefined8 param_2,char param_3,long param_4)

{
  int iVar1;
  __off_t _Var2;
  undefined8 uVar3;
  undefined8 uVar4;
  int *piVar5;
  
  _Var2 = lseek(param_1,param_4,1);
  if (_Var2 < 0) {
    uVar3 = FUN_00118a0f(4,param_2);
    uVar4 = gettext("cannot lseek %s");
    piVar5 = __errno_location();
    error(0,*piVar5,uVar4,uVar3);
    uVar3 = 0;
  }
  else {
    if ((param_3 != '\0') && (iVar1 = FUN_00106d36(param_1,_Var2 - param_4,param_4), iVar1 < 0)) {
      uVar3 = FUN_00118a0f(4,param_2);
      uVar4 = gettext("error deallocating %s");
      piVar5 = __errno_location();
      error(0,*piVar5,uVar4,uVar3);
      return 0;
    }
    uVar3 = 1;
  }
  return uVar3;
}




// Function: sparse_copy @ 0x6ea8

undefined8
sparse_copy(int param_1,undefined4 param_2,long *param_3,ulong param_4,ulong param_5,
            undefined1 param_6,char param_7,undefined8 param_8,undefined8 param_9,ulong param_10,
            long *param_11,char *param_12)

{
  long lVar1;
  bool bVar2;
  bool bVar3;
  char cVar4;
  int iVar5;
  undefined8 uVar6;
  long lVar7;
  int *piVar8;
  undefined8 uVar9;
  ulong uVar10;
  ulong uVar11;
  undefined8 uVar12;
  long in_FS_OFFSET;
  char local_74;
  ulong local_70;
  long local_68;
  ulong local_60;
  ulong local_58;
  void *local_50;
  void *local_48;
  
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  *param_12 = '\0';
  *param_11 = 0;
  if ((param_5 == 0) && (param_7 != '\0')) {
    while (param_10 != 0) {
      uVar10 = param_10;
      if (0x7fffffffbfffffff < param_10) {
        uVar10 = 0x7fffffffc0000000;
      }
      local_68 = copy_file_range(param_1,0,param_2,0,uVar10,0);
      if (local_68 == 0) {
        if (*param_11 != 0) {
          uVar12 = 1;
          goto LAB_001074ea;
        }
        break;
      }
      if (local_68 < 0) {
        piVar8 = __errno_location();
        if (*piVar8 == 0x26) break;
        piVar8 = __errno_location();
        cVar4 = FUN_00106bb9(*piVar8);
        if ((((cVar4 != '\0') || (piVar8 = __errno_location(), *piVar8 == 0x16)) ||
            (piVar8 = __errno_location(), *piVar8 == 9)) ||
           (((piVar8 = __errno_location(), *piVar8 == 0x12 ||
             (piVar8 = __errno_location(), *piVar8 == 0x1a)) ||
            ((piVar8 = __errno_location(), *piVar8 == 1 && (*param_11 == 0)))))) break;
        piVar8 = __errno_location();
        if (*piVar8 != 4) {
          uVar12 = FUN_00118941(1,4,param_9);
          uVar9 = FUN_00118941(0,4,param_8);
          uVar6 = gettext("error copying %s to %s");
          piVar8 = __errno_location();
          error(0,*piVar8,uVar6,uVar9,uVar12);
          uVar12 = 0;
          goto LAB_001074ea;
        }
        local_68 = 0;
      }
      param_10 = param_10 - local_68;
      *param_11 = *param_11 + local_68;
    }
  }
  local_74 = '\0';
  local_70 = 0;
LAB_001074a7:
  do {
    if (param_10 == 0) {
LAB_001074b5:
      if ((local_74 == '\0') ||
         (cVar4 = FUN_00106d9e(param_2,param_9,param_6,local_70), cVar4 == '\x01')) {
        uVar12 = 1;
      }
      else {
        uVar12 = 0;
      }
      goto LAB_001074ea;
    }
    if (*param_3 == 0) {
      iVar5 = getpagesize();
      lVar7 = FUN_0011b8b1((long)iVar5,param_4);
      *param_3 = lVar7;
    }
    local_50 = (void *)*param_3;
    uVar10 = param_4;
    if (param_10 < param_4) {
      uVar10 = param_10;
    }
    local_60 = read(param_1,local_50,uVar10);
    if ((long)local_60 < 0) break;
    if (local_60 == 0) goto LAB_001074b5;
    param_10 = param_10 - local_60;
    *param_11 = *param_11 + local_60;
    local_58 = param_4;
    local_48 = local_50;
    if (param_5 != 0) {
      local_58 = param_5;
    }
    for (; cVar4 = local_74, local_60 != 0; local_60 = local_60 - local_58) {
      uVar10 = local_60;
      if (local_58 <= local_60) {
        uVar10 = local_58;
      }
      if ((param_5 != 0) && (uVar10 != 0)) {
        local_74 = FUN_00106b03(local_50,uVar10);
      }
      if ((local_74 == cVar4) || (local_70 == 0)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      if (((local_60 == uVar10) && (local_74 != '\x01')) || (uVar10 == 0)) {
        bVar3 = true;
      }
      else {
        bVar3 = false;
      }
      local_58 = uVar10;
      if ((bVar2) || (bVar3)) {
        if (!bVar2) {
          local_70 = uVar10 + local_70;
        }
        if (cVar4 == '\x01') {
          cVar4 = FUN_00106d9e(param_2,param_9,param_6,local_70);
          if (cVar4 != '\x01') {
            uVar12 = 0;
            goto LAB_001074ea;
          }
        }
        else {
          uVar11 = FUN_00114bff(param_2,local_48,local_70);
          if (uVar11 != local_70) {
            uVar12 = FUN_00118a0f(4,param_9);
            uVar9 = gettext("error writing %s");
            piVar8 = __errno_location();
            error(0,*piVar8,uVar9,uVar12);
            uVar12 = 0;
            goto LAB_001074ea;
          }
        }
        local_48 = local_50;
        local_70 = uVar10;
        if (bVar3) {
          if (uVar10 == 0) {
            local_60 = 0;
          }
          if (bVar2) {
            local_58 = 0;
          }
          else {
            local_70 = 0;
          }
        }
      }
      else {
        local_70 = uVar10 + local_70;
        if (local_70 + 0x8000000000000000 < uVar10) {
          uVar12 = FUN_00118a0f(4,param_8);
          uVar9 = gettext("overflow reading %s");
          error(0,0,uVar9,uVar12);
          uVar12 = 0;
          goto LAB_001074ea;
        }
      }
      local_50 = (void *)((long)local_50 + local_58);
    }
    *param_12 = local_74;
  } while( true );
  piVar8 = __errno_location();
  if (*piVar8 != 4) {
    uVar12 = FUN_00118a0f(4,param_8);
    uVar9 = gettext("error reading %s");
    piVar8 = __errno_location();
    error(0,*piVar8,uVar9,uVar12);
    uVar12 = 0;
LAB_001074ea:
    if (lVar1 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    return uVar12;
  }
  goto LAB_001074a7;
}




// Function: clone_file @ 0x750c

void clone_file(int param_1,uint param_2)

{
  ioctl(param_1,0x40049409,(ulong)param_2);
  return;
}




// Function: write_zeros @ 0x7533

undefined8 write_zeros(undefined4 param_1,ulong param_2)

{
  ulong uVar1;
  ulong uVar2;
  ulong local_28;
  
  local_28 = param_2;
  if ((DAT_001271c0 == (undefined *)0x0) &&
     (DAT_001271c0 = calloc(DAT_00127010,1), DAT_001271c0 == (undefined *)0x0)) {
    DAT_001271c0 = &DAT_001271e0;
    DAT_00127010 = 0x400;
  }
  while( true ) {
    if (local_28 == 0) {
      return 1;
    }
    uVar1 = DAT_00127010;
    if (local_28 <= DAT_00127010) {
      uVar1 = local_28;
    }
    uVar2 = FUN_00114bff(param_1,DAT_001271c0,uVar1);
    if (uVar1 != uVar2) break;
    local_28 = local_28 - uVar1;
  }
  return 0;
}




// Function: lseek_copy @ 0x75ed

undefined8
lseek_copy(int param_1,int param_2,undefined8 param_3,undefined8 param_4,undefined8 param_5,
            long param_6,__off_t param_7,int param_8,undefined1 param_9,undefined8 param_10,
            undefined8 param_11)

{
  char cVar1;
  byte bVar2;
  int iVar3;
  __off_t _Var4;
  undefined8 uVar5;
  undefined8 uVar6;
  int *piVar7;
  long in_FS_OFFSET;
  long local_90;
  char local_5a;
  char local_59;
  long local_58;
  long local_50;
  long local_48;
  long local_40;
  __off_t local_38;
  long local_30;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_50 = 0;
  local_48 = 0;
  local_40 = 0;
  local_59 = '\x01';
  local_90 = param_6;
  do {
    _Var4 = param_7;
    if (local_90 < 0) {
LAB_001078f7:
      param_7 = _Var4;
      if ((local_40 < param_7) || (local_59 != '\0')) {
        if (param_8 == 1) {
          bVar2 = FUN_00107533(param_2,param_7 - local_40);
          bVar2 = bVar2 ^ 1;
        }
        else {
          iVar3 = ftruncate(param_2,param_7);
          bVar2 = iVar3 != 0;
        }
        if (bVar2 != 0) {
          uVar5 = FUN_00118a0f(4,param_11);
          uVar6 = gettext("failed to extend %s");
          piVar7 = __errno_location();
          error(0,*piVar7,uVar6,uVar5);
          uVar5 = 0;
          goto LAB_00107a71;
        }
      }
      if (((param_8 == 3) && (local_40 < param_7)) &&
         (iVar3 = FUN_00106d36(param_2,local_40,param_7 - local_40), iVar3 < 0)) {
        uVar5 = FUN_00118a0f(4,param_11);
        uVar6 = gettext("error deallocating %s");
        piVar7 = __errno_location();
        error(0,*piVar7,uVar6,uVar5);
        uVar5 = 0;
      }
      else {
        uVar5 = 1;
      }
      goto LAB_00107a71;
    }
    local_38 = lseek(param_1,local_90,4);
    _Var4 = local_38;
    if (local_38 < 0) {
      piVar7 = __errno_location();
      if (*piVar7 != 6) break;
      local_38 = param_7;
      _Var4 = local_38;
      if (param_7 <= local_90) {
        param_7 = lseek(param_1,0,2);
        if (param_7 < 0) break;
        _Var4 = param_7;
        if (param_7 <= local_90) goto LAB_001078f7;
      }
    }
    local_38 = _Var4;
    if (param_7 < local_38) {
      param_7 = local_38;
    }
    _Var4 = lseek(param_1,local_90,0);
    if (_Var4 < 0) break;
    local_59 = '\0';
    local_30 = (local_90 - local_50) - local_48;
    if (local_30 != 0) {
      if (param_8 == 1) {
        cVar1 = FUN_00107533(param_2,local_30);
        if (cVar1 != '\x01') {
          uVar5 = FUN_00118ba1(0,3,param_11);
          uVar6 = gettext("%s: write failed");
          piVar7 = __errno_location();
          error(0,*piVar7,uVar6,uVar5);
          uVar5 = 0;
          goto LAB_00107a71;
        }
      }
      else {
        cVar1 = FUN_00106d9e(param_2,param_11,param_8 == 3,local_30);
        if (cVar1 != '\x01') {
          uVar5 = 0;
          goto LAB_00107a71;
        }
        local_59 = '\x01';
      }
    }
    local_48 = local_38 - local_90;
    local_50 = local_90;
    uVar5 = param_5;
    if (param_8 == 1) {
      uVar5 = 0;
    }
    local_28 = local_48;
    cVar1 = FUN_00106ea8(param_1,param_2,param_3,param_4,uVar5,1,param_9,param_10,param_11,local_48,
                         &local_58,&local_5a);
    if (cVar1 != '\x01') {
      uVar5 = 0;
      goto LAB_00107a71;
    }
    local_40 = local_90 + local_58;
    if (local_58 != 0) {
      local_59 = local_5a;
    }
    _Var4 = local_40;
    if (local_58 < local_28) goto LAB_001078f7;
    local_90 = lseek(param_1,local_40,3);
  } while ((-1 < local_90) || (piVar7 = __errno_location(), *piVar7 == 6));
  uVar5 = FUN_00118a0f(4,param_10);
  uVar6 = gettext("cannot lseek %s");
  piVar7 = __errno_location();
  error(0,*piVar7,uVar6,uVar5);
  uVar5 = 0;
LAB_00107a71:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: is_ancestor @ 0x7a8e

undefined8 is_ancestor(long *param_1,undefined8 *param_2)

{
  undefined8 *local_18;
  
  local_18 = param_2;
  while( true ) {
    if (local_18 == (undefined8 *)0x0) {
      return 0;
    }
    if ((local_18[1] == param_1[1]) && (local_18[2] == *param_1)) break;
    local_18 = (undefined8 *)*local_18;
  }
  return 1;
}




// Function: errno_unsupported @ 0x7ae9

undefined4 errno_unsupported(int param_1)

{
  undefined4 uVar1;
  
  if ((param_1 == 0x5f) || (param_1 == 0x3d)) {
    uVar1 = 1;
  }
  else {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: copy_attr_error @ 0x7b11

void copy_attr_error(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8,
                 undefined8 param_9,undefined8 param_10,undefined8 param_11,undefined8 param_12,
                 undefined8 param_13,undefined8 param_14)

{
  char in_AL;
  char cVar1;
  int *piVar2;
  long in_FS_OFFSET;
  undefined4 local_d8;
  undefined4 local_d4;
  undefined1 *local_d0;
  undefined1 *local_c8;
  long local_c0;
  undefined1 local_b8 [16];
  undefined8 local_a8;
  undefined8 local_a0;
  undefined8 local_98;
  undefined8 local_90;
  undefined8 local_88;
  undefined8 local_78;
  undefined8 local_68;
  undefined8 local_58;
  undefined8 local_48;
  undefined8 local_38;
  undefined8 local_28;
  undefined8 local_18;
  
  if (in_AL != '\0') {
    local_88 = param_1;
    local_78 = param_2;
    local_68 = param_3;
    local_58 = param_4;
    local_48 = param_5;
    local_38 = param_6;
    local_28 = param_7;
    local_18 = param_8;
  }
  local_c0 = *(long *)(in_FS_OFFSET + 0x28);
  local_a8 = param_11;
  local_a0 = param_12;
  local_98 = param_13;
  local_90 = param_14;
  piVar2 = __errno_location();
  cVar1 = FUN_00107ae9(*piVar2);
  if (cVar1 != '\x01') {
    piVar2 = __errno_location();
    local_d8 = 0x10;
    local_d4 = 0x30;
    local_d0 = &stack0x00000008;
    local_c8 = local_b8;
    FUN_0011ae51(0,*piVar2,param_10,&local_d8);
  }
  if (local_c0 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: copy_attr_allerror @ 0x7c09

void copy_attr_allerror(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8,
                 undefined8 param_9,undefined8 param_10,undefined8 param_11,undefined8 param_12,
                 undefined8 param_13,undefined8 param_14)

{
  char in_AL;
  int *piVar1;
  long in_FS_OFFSET;
  undefined4 local_d8;
  undefined4 local_d4;
  undefined1 *local_d0;
  undefined1 *local_c8;
  long local_c0;
  undefined1 local_b8 [16];
  undefined8 local_a8;
  undefined8 local_a0;
  undefined8 local_98;
  undefined8 local_90;
  undefined8 local_88;
  undefined8 local_78;
  undefined8 local_68;
  undefined8 local_58;
  undefined8 local_48;
  undefined8 local_38;
  undefined8 local_28;
  undefined8 local_18;
  
  if (in_AL != '\0') {
    local_88 = param_1;
    local_78 = param_2;
    local_68 = param_3;
    local_58 = param_4;
    local_48 = param_5;
    local_38 = param_6;
    local_28 = param_7;
    local_18 = param_8;
  }
  local_c0 = *(long *)(in_FS_OFFSET + 0x28);
  local_a8 = param_11;
  local_a0 = param_12;
  local_98 = param_13;
  local_90 = param_14;
  piVar1 = __errno_location();
  local_d8 = 0x10;
  local_d4 = 0x30;
  local_d0 = &stack0x00000008;
  local_c8 = local_b8;
  FUN_0011ae51(0,*piVar1,param_10,&local_d8);
  if (local_c0 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: copy_attr_quote @ 0x7cec

void copy_attr_quote(undefined8 param_1,undefined8 param_2)

{
  FUN_00118a0f(4,param_2);
  return;
}




// Function: copy_attr_free @ 0x7d13

void copy_attr_free(void)

{
  return;
}




// Function: check_selinux_attr @ 0x7d26

undefined8 check_selinux_attr(char *param_1,undefined8 param_2)

{
  int iVar1;
  
  iVar1 = strncmp(param_1,"security.selinux",0x10);
  if ((iVar1 != 0) && (iVar1 = attr_copy_check_permissions(param_1,param_2), iVar1 != 0)) {
    return 1;
  }
  return 0;
}




// Function: copy_attr @ 0x7d7e

bool copy_attr(undefined8 param_1,int param_2,undefined8 param_3,int param_4,long param_5)

{
  bool bVar1;
  bool bVar2;
  int iVar3;
  code *pcVar4;
  code **ppcVar5;
  long in_FS_OFFSET;
  code *local_28;
  code *local_20;
  code *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if ((*(char *)(param_5 + 0x31) == '\x01') && (*(char *)(param_5 + 0x36) == '\0')) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  if ((bVar1) || (*(char *)(param_5 + 0x37) == '\x01')) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  if ((*(char *)(param_5 + 0x33) == '\0') && (*(long *)(param_5 + 0x28) == 0)) {
    pcVar4 = (code *)0x0;
  }
  else {
    pcVar4 = FUN_00107d26;
  }
  if ((bVar1) || (bVar2)) {
    if (bVar1) {
      local_28 = FUN_00107c09;
    }
    else {
      local_28 = FUN_00107b11;
    }
    local_20 = FUN_00107cec;
    local_18 = FUN_00107d13;
    ppcVar5 = &local_28;
  }
  else {
    ppcVar5 = (code **)0x0;
  }
  if ((param_2 < 0) || (param_4 < 0)) {
    iVar3 = attr_copy_file(param_1,param_3,pcVar4,ppcVar5);
  }
  else {
    iVar3 = attr_copy_fd(param_1,param_2,param_3,param_4,pcVar4,ppcVar5);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return iVar3 == 0;
}




// Function: copy_dir @ 0x7ee6

byte copy_dir(undefined8 param_1,long param_2,undefined4 param_3,long param_4,undefined1 param_5
                 ,undefined8 param_6,undefined8 param_7,undefined8 *param_8,char *param_9,
                 char *param_10)

{
  byte bVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  int *piVar4;
  size_t sVar5;
  long in_FS_OFFSET;
  char local_9d;
  char local_9c;
  undefined1 local_9b;
  byte local_9a;
  char local_99;
  char *local_98;
  char *local_90;
  void *local_88;
  void *local_80;
  undefined8 local_78;
  undefined8 local_70;
  undefined8 local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined8 local_40;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_78 = *param_8;
  local_70 = param_8[1];
  local_68 = param_8[2];
  local_60 = param_8[3];
  local_58 = param_8[4];
  local_50 = param_8[5];
  local_48 = param_8[6];
  local_40 = param_8[7];
  local_38 = param_8[8];
  local_30 = param_8[9];
  local_28 = param_8[10];
  local_9a = 1;
  local_90 = (char *)FUN_001199a1(param_1,2);
  if (local_90 == (char *)0x0) {
    uVar2 = FUN_00118a0f(4,param_1);
    uVar3 = gettext("cannot access %s");
    piVar4 = __errno_location();
    error(0,*piVar4,uVar3,uVar2);
    bVar1 = 0;
  }
  else {
    if (*(int *)((long)param_8 + 4) == 3) {
      local_78 = CONCAT44(2,(undefined4)local_78);
    }
    local_99 = '\0';
    for (local_98 = local_90; *local_98 != '\0'; local_98 = local_98 + sVar5 + 1) {
      local_88 = (void *)FUN_00111da5(param_1,local_98,0);
      local_80 = (void *)FUN_00111da5(param_2,local_98,0);
      local_9c = *param_9;
      bVar1 = FUN_0010abda(local_88,local_80,param_3,(param_4 - param_2) + (long)local_80,param_5,
                           param_6,param_7,&local_78,0,&local_9c,&local_9d,&local_9b);
      local_9a = (bVar1 & local_9a) != 0;
      *param_10 = local_9d != '\0' || *param_10 != '\0';
      free(local_80);
      free(local_88);
      if (local_9d != '\0') break;
      local_99 = local_9c != '\0' || local_99 != '\0';
      sVar5 = strlen(local_98);
    }
    free(local_90);
    *param_9 = local_99;
    bVar1 = local_9a;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: set_owner @ 0x820f

int set_owner(long param_1,undefined8 param_2,undefined4 param_3,undefined8 param_4,int param_5,
                long param_6,char param_7,long param_8)

{
  __uid_t __owner;
  __gid_t __group;
  char cVar1;
  uint uVar2;
  int iVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  int *piVar6;
  
  __owner = *(__uid_t *)(param_6 + 0x1c);
  __group = *(__gid_t *)(param_6 + 0x20);
  if ((param_7 != '\x01') &&
     (((*(char *)(param_1 + 0x1e) != '\0' || (*(char *)(param_1 + 0x18) != '\0')) ||
      (*(char *)(param_1 + 0x39) != '\0')))) {
    if ((*(char *)(param_1 + 0x1e) == '\0') && (*(char *)(param_1 + 0x18) == '\0')) {
      uVar2 = *(uint *)(param_1 + 0x10);
    }
    else {
      uVar2 = *(uint *)(param_6 + 0x18);
    }
    if ((((uint)(ushort)(~(ushort)uVar2 | 0xe00) & *(uint *)(param_8 + 0x18) & 0xfff) != 0) &&
       (iVar3 = FUN_00116e81(param_2,param_5,*(uint *)(param_8 + 0x18) & uVar2 & 0x1c0), iVar3 != 0)
       ) {
      cVar1 = FUN_0010dc67(param_1);
      if (cVar1 != '\x01') {
        uVar4 = FUN_00118a0f(4,param_2);
        uVar5 = gettext("clearing permissions for %s");
        piVar6 = __errno_location();
        error(0,*piVar6,uVar5,uVar4);
      }
      return -(uint)*(byte *)(param_1 + 0x32);
    }
  }
  if (param_5 == -1) {
    iVar3 = FUN_001112c3(param_3,param_4,__owner,__group);
    if (iVar3 == 0) {
      return 1;
    }
    piVar6 = __errno_location();
    if ((*piVar6 == 1) || (piVar6 = __errno_location(), *piVar6 == 0x16)) {
      piVar6 = __errno_location();
      iVar3 = *piVar6;
      FUN_001112c3(param_3,param_4,0xffffffff,__group);
      piVar6 = __errno_location();
      *piVar6 = iVar3;
    }
  }
  else {
    iVar3 = fchown(param_5,__owner,__group);
    if (iVar3 == 0) {
      return 1;
    }
    piVar6 = __errno_location();
    if ((*piVar6 == 1) || (piVar6 = __errno_location(), *piVar6 == 0x16)) {
      piVar6 = __errno_location();
      iVar3 = *piVar6;
      fchown(param_5,0xffffffff,__group);
      piVar6 = __errno_location();
      *piVar6 = iVar3;
    }
  }
  cVar1 = FUN_0010dc1f(param_1);
  if (cVar1 != '\x01') {
    uVar4 = FUN_00118a0f(4,param_2);
    uVar5 = gettext("failed to preserve ownership for %s");
    piVar6 = __errno_location();
    error(0,*piVar6,uVar5,uVar4);
    if (*(char *)(param_1 + 0x32) != '\0') {
      return -1;
    }
  }
  return 0;
}




// Function: set_author @ 0x84b0

void set_author(void)

{
  return;
}




// Function: set_process_security_ctx @ 0x84c6

undefined8
set_process_security_ctx(undefined8 param_1,undefined8 param_2,undefined4 param_3,char param_4,long param_5)

{
  bool bVar1;
  bool bVar2;
  char cVar3;
  int iVar4;
  int *piVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  long in_FS_OFFSET;
  undefined8 local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (*(char *)(param_5 + 0x33) == '\0') {
    if ((*(long *)(param_5 + 0x28) != 0) && (param_4 != '\0')) {
      iVar4 = FUN_0010e549(*(undefined8 *)(param_5 + 0x28),param_2,param_3);
      if (iVar4 < 0) {
        piVar5 = __errno_location();
        cVar3 = FUN_00106c97(*piVar5);
        if (cVar3 != '\x01') {
          uVar6 = FUN_00118a0f(4,param_2);
          uVar7 = gettext("failed to set default file creation context for %s");
          piVar5 = __errno_location();
          error(0,*piVar5,uVar7,uVar6);
        }
      }
    }
  }
  else {
    if ((*(char *)(param_5 + 0x31) == '\x01') && (*(char *)(param_5 + 0x34) == '\0')) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    if ((bVar1) || (*(char *)(param_5 + 0x37) == '\x01')) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    iVar4 = FUN_00119d8b(param_1,&local_28);
    if (iVar4 < 0) {
      if (bVar1) {
LAB_00108646:
        uVar6 = FUN_00118a0f(4,param_1);
        uVar7 = gettext("failed to get security context of %s");
        piVar5 = __errno_location();
        error(0,*piVar5,uVar7,uVar6);
      }
      else if (bVar2) {
        piVar5 = __errno_location();
        cVar3 = FUN_00107ae9(*piVar5);
        if (cVar3 != '\x01') goto LAB_00108646;
      }
      if (*(char *)(param_5 + 0x34) != '\0') {
        uVar6 = 0;
        goto LAB_00108733;
      }
    }
    else {
      iVar4 = setfscreatecon(local_28);
      if (iVar4 < 0) {
        if (bVar1) {
LAB_001085b3:
          uVar6 = FUN_00118e56(local_28);
          uVar7 = gettext("failed to set default file creation context to %s");
          piVar5 = __errno_location();
          error(0,*piVar5,uVar7,uVar6);
        }
        else if (bVar2) {
          piVar5 = __errno_location();
          cVar3 = FUN_00107ae9(*piVar5);
          if (cVar3 != '\x01') goto LAB_001085b3;
        }
        if (*(char *)(param_5 + 0x34) != '\0') {
          freecon(local_28);
          uVar6 = 0;
          goto LAB_00108733;
        }
      }
      freecon(local_28);
    }
  }
  uVar6 = 1;
LAB_00108733:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar6;
}




// Function: set_file_security_ctx @ 0x8750

undefined8 set_file_security_ctx(undefined8 param_1,undefined1 param_2,long param_3)

{
  bool bVar1;
  bool bVar2;
  char cVar3;
  int *piVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  
  if ((*(char *)(param_3 + 0x31) == '\x01') && (*(char *)(param_3 + 0x34) == '\0')) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  if ((bVar1) || (*(char *)(param_3 + 0x37) == '\x01')) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  cVar3 = FUN_0010ead7(*(undefined8 *)(param_3 + 0x28),param_1,param_2);
  if (cVar3 == '\x01') {
    return 1;
  }
  if (!bVar1) {
    if (!bVar2) {
      return 0;
    }
    piVar4 = __errno_location();
    cVar3 = FUN_00107ae9(*piVar4);
    if (cVar3 == '\x01') {
      return 0;
    }
  }
  uVar5 = FUN_00118941(0,4,param_1);
  uVar6 = gettext("failed to set the security context of %s");
  piVar4 = __errno_location();
  error(0,*piVar4,uVar6,uVar5);
  return 0;
}




// Function: fchmod_or_lchmod @ 0x8868

void fchmod_or_lchmod(int param_1,undefined4 param_2,undefined8 param_3,__mode_t param_4)

{
  if (param_1 < 0) {
    FUN_00111260(param_2,param_3,param_4);
  }
  else {
    fchmod(param_1,param_4);
  }
  return;
}




// Function: infer_scantype @ 0x88ae

undefined8 infer_scantype(int param_1,long param_2,long *param_3)

{
  char cVar1;
  long lVar2;
  __off_t _Var3;
  int *piVar4;
  
  if ((*(uint *)(param_2 + 0x18) & 0xf000) == 0x8000) {
    lVar2 = *(long *)(param_2 + 0x30);
    if (lVar2 < 0) {
      lVar2 = lVar2 + 0x1ff;
    }
    if (*(long *)(param_2 + 0x40) < lVar2 >> 9) {
      _Var3 = lseek(param_1,0,3);
      *param_3 = _Var3;
      if ((*param_3 < 0) && (piVar4 = __errno_location(), *piVar4 != 6)) {
        piVar4 = __errno_location();
        if (*piVar4 != 0x16) {
          piVar4 = __errno_location();
          cVar1 = FUN_00106bb9(*piVar4);
          if (cVar1 != '\x01') {
            return 0;
          }
        }
        return 2;
      }
      return 3;
    }
  }
  return 1;
}




// Function: copy_reg @ 0x896f

undefined1
copy_reg(undefined8 param_1,char *param_2,int param_3,char *param_4,long param_5,uint param_6,
            uint param_7,char *param_8,__dev_t *param_9)

{
  char cVar1;
  undefined1 uVar2;
  int iVar3;
  uint uVar4;
  undefined8 uVar5;
  int *piVar6;
  char *__format;
  ssize_t sVar7;
  size_t sVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined4 uVar11;
  ulong uVar12;
  long in_FS_OFFSET;
  char local_1dd;
  undefined1 local_1dc;
  char local_1db;
  char local_1da;
  char local_1d9;
  int local_1d8;
  int local_1d4;
  uint local_1d0;
  uint local_1cc;
  int local_1c8;
  undefined4 local_1c4;
  uint local_1c0;
  undefined4 local_1bc;
  uint local_1b8;
  int local_1b4;
  undefined8 local_1b0;
  undefined8 local_1a8;
  __off_t local_1a0;
  ulong local_198;
  ulong local_190;
  ulong local_188;
  ulong local_180;
  undefined1 local_178 [16];
  undefined1 local_168 [16];
  stat local_158;
  stat local_c8;
  char local_31;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_1b0 = 0;
  local_1d0 = (uint)param_9[3];
  local_1dc = 1;
  local_1db = *(char *)(param_5 + 0x31);
  local_1da = *(char *)(param_5 + 0x35) != '\0';
  if (*(int *)(param_5 + 4) == 2) {
    uVar10 = 0x20000;
  }
  else {
    uVar10 = 0;
  }
  local_1c8 = FUN_00111785(param_1,uVar10);
  if (local_1c8 < 0) {
    uVar10 = FUN_00118a0f(4,param_1);
    uVar5 = gettext("cannot open %s for reading");
    piVar6 = __errno_location();
    error(0,*piVar6,uVar5,uVar10);
    uVar2 = 0;
    goto LAB_00109af9;
  }
  iVar3 = fstat(local_1c8,&local_c8);
  if (iVar3 == 0) {
    if ((param_9[1] == local_c8.st_ino) && (*param_9 == local_c8.st_dev)) {
      if (*param_8 == '\x01') {
LAB_00108dca:
        if (*param_8 == '\0') {
          local_1cc = 0;
          param_7 = 0;
        }
        else {
          if ((local_1da == '\0') || (*(char *)(param_5 + 0x1b) == '\x01')) {
            local_1c0 = 0;
          }
          else {
            local_1c0 = 0x80;
          }
          local_1c0 = local_1c0 | ~param_7 & param_6;
          local_1cc = ~param_6 & local_1c0;
          local_1bc = 0x41;
          local_1d8 = FUN_00116b35(param_3,param_4,0xc1,local_1c0);
          piVar6 = __errno_location();
          local_1d4 = *piVar6;
          if ((((local_1d8 < 0) && (local_1d4 == 0x11)) && (*(char *)(param_5 + 0x18) != '\x01')) &&
             (sVar7 = readlinkat(param_3,param_4,&local_31,1), -1 < sVar7)) {
            if (*(char *)(param_5 + 0x3e) == '\0') {
              uVar10 = FUN_00118a0f(4,param_2);
              uVar5 = gettext("not writing through dangling symlink %s");
              error(0,0,uVar5,uVar10);
              local_1dc = 0;
              goto LAB_00109a84;
            }
            local_1d8 = FUN_00116b35(param_3,param_4,local_1bc,local_1c0);
            piVar6 = __errno_location();
            local_1d4 = *piVar6;
          }
          if (((local_1d8 < 0) && (local_1d4 == 0x15)) &&
             ((*param_2 != '\0' && (sVar8 = strlen(param_2), param_2[sVar8 - 1] == '/')))) {
            local_1d4 = 0x14;
          }
        }
        if (local_1d8 < 0) {
          uVar10 = FUN_00118a0f(4,param_2);
          uVar5 = gettext("cannot create regular file %s");
          error(0,local_1d4,uVar5,uVar10);
          local_1dc = 0;
          goto LAB_00109a84;
        }
        if ((local_1db != '\0') && (*(int *)(param_5 + 0x44) != 0)) {
          iVar3 = FUN_0010750c(local_1d8,local_1c8);
          if (iVar3 == 0) {
            local_1db = '\0';
          }
          else if (*(int *)(param_5 + 0x44) == 2) {
            uVar10 = FUN_00118941(1,4,param_1);
            uVar5 = FUN_00118941(0,4,param_2);
            uVar9 = gettext("failed to clone %s from %s");
            piVar6 = __errno_location();
            error(0,*piVar6,uVar9,uVar5,uVar10);
            local_1dc = 0;
            goto LAB_00109a22;
          }
        }
        if ((*(char *)(param_5 + 0x1d) == '\0' && local_1db == '\0') && local_1cc == 0) {
          local_158.st_mode = 0;
        }
        else {
          iVar3 = fstat(local_1d8,&local_158);
          if (iVar3 != 0) {
            uVar10 = FUN_00118a0f(4,param_2);
            uVar5 = gettext("cannot fstat %s");
            piVar6 = __errno_location();
            error(0,*piVar6,uVar5,uVar10);
            local_1dc = 0;
            goto LAB_00109a22;
          }
        }
        local_1b8 = local_158.st_mode | local_1cc;
        if ((local_1b8 != local_158.st_mode) &&
           (iVar3 = FUN_00108868(local_1d8,param_3,param_4,local_1b8), iVar3 != 0)) {
          local_1cc = 0;
        }
        if (local_1db == '\0') {
LAB_00109666:
          if (*(char *)(param_5 + 0x1f) != '\0') {
            local_178 = FUN_00119e57(param_9);
            local_168 = FUN_00119e8b(param_9);
            iVar3 = FUN_001118c1(local_1d8,param_3,param_4,local_178,0);
            if (iVar3 != 0) {
              uVar10 = FUN_00118a0f(4,param_2);
              uVar5 = gettext("preserving times for %s");
              piVar6 = __errno_location();
              error(0,*piVar6,uVar5,uVar10);
              if (*(char *)(param_5 + 0x32) != '\0') {
                local_1dc = 0;
                goto LAB_00109a22;
              }
            }
          }
          if ((*(char *)(param_5 + 0x1d) != '\0') &&
             ((*(__uid_t *)((long)param_9 + 0x1c) != local_158.st_uid ||
              ((__gid_t)param_9[4] != local_158.st_gid)))) {
            iVar3 = FUN_0010820f(param_5,param_2,param_3,param_4,local_1d8,param_9,*param_8,
                                 &local_158);
            if (iVar3 == -1) {
              local_1dc = 0;
              goto LAB_00109a22;
            }
            if (iVar3 == 0) {
              local_1d0 = local_1d0 & 0xfffff1ff;
            }
          }
          if (local_1da != '\0') {
            cVar1 = FUN_00107d7e(param_1,local_1c8,param_2,local_1d8,param_5);
            if ((cVar1 != '\x01') && (*(char *)(param_5 + 0x36) != '\0')) {
              local_1dc = 0;
            }
          }
          FUN_001084b0(param_2,local_1d8,param_9);
          if ((*(char *)(param_5 + 0x1e) == '\0') && (*(char *)(param_5 + 0x18) == '\0')) {
            if (*(char *)(param_5 + 0x39) == '\0') {
              if ((*(char *)(param_5 + 0x20) == '\0') || (*param_8 == '\0')) {
                if (param_7 != 0 || local_1cc != 0) {
                  uVar4 = FUN_0010dcaf();
                  if ((param_7 & ~uVar4) != 0 || local_1cc != 0) {
                    uVar4 = FUN_0010dcaf();
                    iVar3 = FUN_00108868(local_1d8,param_3,param_4,~uVar4 & param_6);
                    if (iVar3 != 0) {
                      uVar10 = FUN_00118a0f(4,param_2);
                      uVar5 = gettext("preserving permissions for %s");
                      piVar6 = __errno_location();
                      error(0,*piVar6,uVar5,uVar10);
                      if (*(char *)(param_5 + 0x32) != '\0') {
                        local_1dc = 0;
                      }
                    }
                  }
                }
              }
              else {
                uVar4 = FUN_0010dcaf();
                iVar3 = FUN_0010ed24(param_2,local_1d8,~uVar4 & 0x1b6);
                if (iVar3 != 0) {
                  local_1dc = 0;
                }
              }
            }
            else {
              iVar3 = FUN_0010ed24(param_2,local_1d8,*(undefined4 *)(param_5 + 0x10));
              if (iVar3 != 0) {
                local_1dc = 0;
              }
            }
          }
          else {
            iVar3 = FUN_0010ec57(param_1,local_1c8,param_2,local_1d8,local_1d0);
            if ((iVar3 != 0) && (*(char *)(param_5 + 0x32) != '\0')) {
              local_1dc = 0;
            }
          }
        }
        else {
          local_198 = FUN_00106bc9();
          if ((local_158.st_blksize < 1) || (0x2000000000000000 < (ulong)local_158.st_blksize)) {
            local_158.st_blksize = 0x200;
          }
          local_190 = local_158.st_blksize;
          local_1b4 = FUN_001088ae(local_1c8,&local_c8,&local_1a8);
          if (local_1b4 == 0) {
            uVar10 = FUN_00118a0f(4,param_1);
            uVar5 = gettext("cannot lseek %s");
            piVar6 = __errno_location();
            error(0,*piVar6,uVar5,uVar10);
            local_1dc = 0;
          }
          else {
            if (((local_158.st_mode & 0xf000) == 0x8000) &&
               ((*(int *)(param_5 + 0xc) == 3 ||
                ((*(int *)(param_5 + 0xc) == 2 && (local_1b4 != 1)))))) {
              local_1d9 = '\x01';
            }
            else {
              local_1d9 = '\0';
            }
            FUN_00111711(local_1c8,0,0,2);
            if (local_1d9 != '\x01') {
              local_188 = 0x7fffffffffffffff;
              uVar10 = FUN_00106bc9();
              local_180 = FUN_00110429(uVar10,local_198,local_188);
              if (((local_c8.st_mode & 0xf000) == 0x8000) && ((ulong)local_c8.st_size < local_198))
              {
                local_198 = local_c8.st_size + 1;
              }
              local_198 = (local_198 + local_180) - 1;
              local_198 = local_198 - local_198 % local_180;
              if ((local_198 == 0) || (local_188 < local_198)) {
                local_198 = local_180;
              }
            }
            local_1dd = '\0';
            if (local_1b4 == 3) {
              if (local_1d9 == '\0') {
                uVar11 = 1;
              }
              else {
                uVar11 = *(undefined4 *)(param_5 + 0xc);
              }
              cVar1 = FUN_001075ed(local_1c8,local_1d8,&local_1b0,local_198,local_190,local_1a8,
                                   local_c8.st_size,uVar11,*(int *)(param_5 + 0x44) != 0,param_1,
                                   param_2);
            }
            else {
              uVar12 = local_190;
              if (local_1d9 == '\0') {
                uVar12 = 0;
              }
              cVar1 = FUN_00106ea8(local_1c8,local_1d8,&local_1b0,local_198,uVar12,
                                   *(int *)(param_5 + 0xc) == 3,*(int *)(param_5 + 0x44) != 0,
                                   param_1,param_2,0xffffffffffffffff,&local_1a0,&local_1dd);
            }
            if (cVar1 == '\x01') {
              if ((local_1dd == '\0') || (iVar3 = ftruncate(local_1d8,local_1a0), -1 < iVar3))
              goto LAB_00109666;
              uVar10 = FUN_00118a0f(4,param_2);
              uVar5 = gettext("failed to extend %s");
              piVar6 = __errno_location();
              error(0,*piVar6,uVar5,uVar10);
              local_1dc = 0;
            }
            else {
              local_1dc = 0;
            }
          }
        }
      }
      else {
        if (local_1db == '\0') {
          local_1c4 = 1;
        }
        else {
          local_1c4 = 0x201;
        }
        local_1d8 = FUN_00116b35(param_3,param_4,local_1c4);
        piVar6 = __errno_location();
        local_1d4 = *piVar6;
        if ((local_1d8 < 0) ||
           ((*(long *)(param_5 + 0x28) == 0 && (*(char *)(param_5 + 0x33) == '\0')))) {
LAB_00108c5f:
          if (((local_1d8 < 0) && (local_1d4 != 2)) && (*(char *)(param_5 + 0x16) != '\0')) {
            iVar3 = unlinkat(param_3,param_4,0);
            if (iVar3 == 0) {
              if (*(char *)(param_5 + 0x3c) != '\0') {
                uVar10 = FUN_00118a0f(4,param_2);
                __format = (char *)gettext("removed %s\n");
                printf(__format,uVar10);
              }
            }
            else {
              piVar6 = __errno_location();
              if (*piVar6 != 2) {
                uVar10 = FUN_00118a0f(4,param_2);
                uVar5 = gettext("cannot remove %s");
                piVar6 = __errno_location();
                error(0,*piVar6,uVar5,uVar10);
                local_1dc = 0;
                goto LAB_00109a84;
              }
            }
            local_1d4 = 2;
          }
          if ((local_1d8 < 0) && (local_1d4 == 2)) {
            if (*(long *)(param_5 + 0x28) != 0) {
              cVar1 = FUN_001084c6(param_1,param_2,param_6,1,param_5);
              if (cVar1 != '\x01') {
                local_1dc = 0;
                goto LAB_00109a84;
              }
            }
            *param_8 = '\x01';
          }
          goto LAB_00108dca;
        }
        cVar1 = FUN_00108750(param_2,0,param_5);
        if ((cVar1 == '\x01') || (*(char *)(param_5 + 0x34) == '\0')) goto LAB_00108c5f;
        local_1dc = 0;
      }
LAB_00109a22:
      iVar3 = close(local_1d8);
      if (iVar3 < 0) {
        uVar10 = FUN_00118a0f(4,param_2);
        uVar5 = gettext("failed to close %s");
        piVar6 = __errno_location();
        error(0,*piVar6,uVar5,uVar10);
        local_1dc = 0;
      }
    }
    else {
      uVar10 = FUN_00118a0f(4,param_1);
      uVar5 = gettext("skipping file %s, as it was replaced while being copied");
      error(0,0,uVar5,uVar10);
      local_1dc = 0;
    }
  }
  else {
    uVar10 = FUN_00118a0f(4,param_1);
    uVar5 = gettext("cannot fstat %s");
    piVar6 = __errno_location();
    error(0,*piVar6,uVar5,uVar10);
    local_1dc = 0;
  }
LAB_00109a84:
  iVar3 = close(local_1c8);
  if (iVar3 < 0) {
    uVar10 = FUN_00118a0f(4,param_1);
    uVar5 = gettext("failed to close %s");
    piVar6 = __errno_location();
    error(0,*piVar6,uVar5,uVar10);
    local_1dc = 0;
  }
  FUN_0010eda5(local_1b0);
  uVar2 = local_1dc;
LAB_00109af9:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: same_file_ok @ 0x9b18

/* WARNING: Restarted to delay deadcode elimination for space: stack */

bool same_file_ok(char *param_1,stat *param_2,int param_3,char *param_4,stat *param_5,int *param_6,
                 undefined1 *param_7)

{
  char cVar1;
  int iVar2;
  char *__ptr;
  long in_FS_OFFSET;
  bool bVar3;
  stat *local_160;
  stat *local_158;
  stat local_148;
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if ((param_2->st_ino == param_5->st_ino) && (param_2->st_dev == param_5->st_dev)) {
    bVar3 = true;
  }
  else {
    bVar3 = false;
  }
  *param_7 = 0;
  if ((bVar3) && (*(char *)((long)param_6 + 0x17) != '\0')) {
    *param_7 = 1;
    bVar3 = true;
    goto LAB_0010a360;
  }
  if (param_6[1] == 2) {
    local_160 = param_2;
    local_158 = param_5;
    if (((param_2->st_mode & 0xf000) == 0xa000) && ((param_5->st_mode & 0xf000) == 0xa000)) {
      cVar1 = FUN_00119370(0xffffff9c,param_1,param_3,param_4);
      if (cVar1 != '\x01') {
        if (*param_6 != 0) {
          bVar3 = true;
          goto LAB_0010a360;
        }
        if (bVar3) {
          *param_7 = 1;
          bVar3 = (char)param_6[6] == '\0';
          goto LAB_0010a360;
        }
      }
      bVar3 = cVar1 == '\0';
      goto LAB_0010a360;
    }
  }
  else {
    if (!bVar3) {
      bVar3 = true;
      goto LAB_0010a360;
    }
    iVar2 = fstatat(param_3,param_4,&local_148,0x100);
    if ((iVar2 != 0) || (iVar2 = lstat(param_1,&local_b8), iVar2 != 0)) {
      bVar3 = true;
      goto LAB_0010a360;
    }
    local_160 = &local_b8;
    local_158 = &local_148;
    if ((local_b8.st_ino == local_148.st_ino) && (local_b8.st_dev == local_148.st_dev)) {
      bVar3 = true;
    }
    else {
      bVar3 = false;
    }
    if ((((local_b8.st_mode & 0xf000) == 0xa000) && ((local_148.st_mode & 0xf000) == 0xa000)) &&
       (*(char *)((long)param_6 + 0x15) != '\0')) {
      bVar3 = true;
      goto LAB_0010a360;
    }
  }
  if (*param_6 != 0) {
    if (bVar3) {
      cVar1 = FUN_00119370(0xffffff9c,param_1,param_3,param_4);
      bVar3 = cVar1 == '\0';
    }
    else if (((((char)param_6[6] == '\x01') || (param_6[1] == 2)) ||
             ((local_160->st_mode & 0xf000) != 0xa000)) || ((local_158->st_mode & 0xf000) == 0xa000)
            ) {
      bVar3 = true;
    }
    else {
      bVar3 = false;
    }
    goto LAB_0010a360;
  }
  if (((char)param_6[6] != '\0') || (*(char *)((long)param_6 + 0x15) != '\0')) {
    if ((local_158->st_mode & 0xf000) == 0xa000) {
      bVar3 = true;
      goto LAB_0010a360;
    }
    if (((bVar3) && (1 < local_158->st_nlink)) &&
       (cVar1 = FUN_00119370(0xffffff9c,param_1,param_3,param_4), cVar1 != '\x01')) {
      bVar3 = (char)param_6[6] == '\0';
      goto LAB_0010a360;
    }
  }
  if (((local_160->st_mode & 0xf000) != 0xa000) && ((local_158->st_mode & 0xf000) != 0xa000)) {
    if ((local_160->st_ino != local_158->st_ino) || (local_160->st_dev != local_158->st_dev)) {
      bVar3 = true;
      goto LAB_0010a360;
    }
    if (*(char *)((long)param_6 + 0x17) != '\0') {
      *param_7 = 1;
      bVar3 = true;
      goto LAB_0010a360;
    }
  }
  if ((((char)param_6[6] == '\0') || ((param_2->st_mode & 0xf000) != 0xa000)) ||
     ((local_158->st_nlink < 2 || (__ptr = canonicalize_file_name(param_1), __ptr == (char *)0x0))))
  {
    if ((*(char *)((long)param_6 + 0x3a) == '\0') || ((local_158->st_mode & 0xf000) != 0xa000)) {
      if (param_6[1] == 2) {
        if ((local_160->st_mode & 0xf000) == 0xa000) {
          iVar2 = stat(param_1,&local_b8);
          if (iVar2 != 0) {
            bVar3 = true;
            goto LAB_0010a360;
          }
        }
        else {
          local_b8.st_dev = local_160->st_dev;
          local_b8.st_ino = local_160->st_ino;
          local_b8.st_nlink = local_160->st_nlink;
          local_b8.st_mode = local_160->st_mode;
          local_b8.st_uid = local_160->st_uid;
          local_b8.st_gid = local_160->st_gid;
          local_b8.__pad0 = local_160->__pad0;
          local_b8.st_rdev = local_160->st_rdev;
          local_b8.st_size = local_160->st_size;
          local_b8.st_blksize = local_160->st_blksize;
          local_b8.st_blocks = local_160->st_blocks;
          local_b8.st_atim.tv_sec = (local_160->st_atim).tv_sec;
          local_b8.st_atim.tv_nsec = (local_160->st_atim).tv_nsec;
          local_b8.st_mtim.tv_sec = (local_160->st_mtim).tv_sec;
          local_b8.st_mtim.tv_nsec = (local_160->st_mtim).tv_nsec;
          local_b8.st_ctim.tv_sec = (local_160->st_ctim).tv_sec;
          local_b8.st_ctim.tv_nsec = (local_160->st_ctim).tv_nsec;
          local_b8.__unused[0] = local_160->__unused[0];
          local_b8.__unused[2] = local_160->__unused[2];
          local_b8.__unused[1] = local_160->__unused[1];
        }
        if ((local_158->st_mode & 0xf000) == 0xa000) {
          iVar2 = fstatat(param_3,param_4,&local_148,0);
          if (iVar2 != 0) {
            bVar3 = true;
            goto LAB_0010a360;
          }
        }
        else {
          local_148.st_dev = local_158->st_dev;
          local_148.st_ino = local_158->st_ino;
          local_148.st_nlink = local_158->st_nlink;
          local_148.st_mode = local_158->st_mode;
          local_148.st_uid = local_158->st_uid;
          local_148.st_gid = local_158->st_gid;
          local_148.__pad0 = local_158->__pad0;
          local_148.st_rdev = local_158->st_rdev;
          local_148.st_size = local_158->st_size;
          local_148.st_blksize = local_158->st_blksize;
          local_148.st_blocks = local_158->st_blocks;
          local_148.st_atim.tv_sec = (local_158->st_atim).tv_sec;
          local_148.st_atim.tv_nsec = (local_158->st_atim).tv_nsec;
          local_148.st_mtim.tv_sec = (local_158->st_mtim).tv_sec;
          local_148.st_mtim.tv_nsec = (local_158->st_mtim).tv_nsec;
          local_148.st_ctim.tv_sec = (local_158->st_ctim).tv_sec;
          local_148.st_ctim.tv_nsec = (local_158->st_ctim).tv_nsec;
          local_148.__unused[0] = local_158->__unused[0];
          local_148.__unused[2] = local_158->__unused[2];
          local_148.__unused[1] = local_158->__unused[1];
        }
        if ((local_b8.st_ino != local_148.st_ino) || (local_b8.st_dev != local_148.st_dev)) {
          bVar3 = true;
          goto LAB_0010a360;
        }
        if (*(char *)((long)param_6 + 0x17) != '\0') {
          *param_7 = (local_158->st_mode & 0xf000) != 0xa000;
          bVar3 = true;
          goto LAB_0010a360;
        }
      }
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
  }
  else {
    cVar1 = FUN_00119370(0xffffff9c,__ptr,param_3,param_4);
    bVar3 = cVar1 == '\0';
    free(__ptr);
  }
LAB_0010a360:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: writable_destination @ 0xa37a

undefined4 writable_destination(int param_1,char *param_2,uint param_3)

{
  char cVar1;
  int iVar2;
  
  if ((((param_3 & 0xf000) != 0xa000) && (cVar1 = FUN_0011b86c(), cVar1 == '\0')) &&
     (iVar2 = faccessat(param_1,param_2,2,0x200), iVar2 != 0)) {
    return 0;
  }
  return 1;
}




// Function: overwrite_ok @ 0xa3d5

void overwrite_ok(long param_1,undefined8 param_2,undefined4 param_3,undefined8 param_4,long param_5
                 )

{
  uint uVar1;
  undefined8 uVar2;
  char cVar3;
  undefined8 uVar4;
  char *pcVar5;
  long in_FS_OFFSET;
  undefined1 local_3c;
  undefined1 auStack_3b [9];
  undefined1 local_32;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  cVar3 = FUN_0010a37a(param_3,param_4,*(undefined4 *)(param_5 + 0x18));
  if (cVar3 == '\x01') {
    uVar4 = FUN_00118a0f(4,param_2);
    uVar2 = DAT_00127608;
    pcVar5 = (char *)gettext("%s: overwrite %s? ");
    fprintf(stderr,pcVar5,uVar2,uVar4);
  }
  else {
    FUN_00111bad(*(undefined4 *)(param_5 + 0x18),&local_3c);
    local_32 = 0;
    uVar1 = *(uint *)(param_5 + 0x18);
    uVar4 = FUN_00118a0f(4,param_2);
    uVar2 = DAT_00127608;
    if (((*(char *)(param_1 + 0x18) == '\0') && (*(char *)(param_1 + 0x15) == '\0')) &&
       (*(char *)(param_1 + 0x16) == '\0')) {
      pcVar5 = (char *)gettext("%s: unwritable %s (mode %04lo, %s); try anyway? ");
    }
    else {
      pcVar5 = (char *)gettext("%s: replace %s, overriding mode %04lo (%s)? ");
    }
    fprintf(stderr,pcVar5,uVar2,uVar4,(ulong)(uVar1 & 0xfff),auStack_3b);
  }
  FUN_0011c30f();
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: dest_info_init @ 0xa549

void dest_info_init(long param_1)

{
  undefined8 uVar1;
  
  uVar1 = FUN_0011563d(0x3d,0,FUN_001167c7,FUN_0011674b,FUN_00116893);
  *(undefined8 *)(param_1 + 0x48) = uVar1;
  if (*(long *)(param_1 + 0x48) == 0) {
    FUN_0011bf79();
  }
  return;
}




// Function: src_info_init @ 0xa5a6

void src_info_init(long param_1)

{
  undefined8 uVar1;
  
  uVar1 = FUN_0011563d(0x3d,0,FUN_0011671d,FUN_0011674b,FUN_00116893);
  *(undefined8 *)(param_1 + 0x50) = uVar1;
  if (*(long *)(param_1 + 0x50) == 0) {
    FUN_0011bf79();
  }
  return;
}




// Function: abandon_move @ 0xa603

undefined4
abandon_move(long param_1,undefined8 param_2,undefined4 param_3,undefined8 param_4,long param_5)

{
  char cVar1;
  undefined4 uVar2;
  
  if (*(char *)(param_1 + 0x18) == '\0') {
                    /* WARNING: Subroutine does not return */
    __assert_fail("x->move_mode","src/copy.c",0x710,"abandon_move");
  }
  if (*(int *)(param_1 + 8) == 2) {
LAB_0010a6ca:
    uVar2 = 1;
  }
  else {
    if (*(int *)(param_1 + 8) == 3) {
LAB_0010a6a5:
      cVar1 = FUN_0010a3d5(param_1,param_2,param_3,param_4,param_5);
      if (cVar1 != '\x01') goto LAB_0010a6ca;
    }
    else if ((*(int *)(param_1 + 8) == 4) && (*(char *)(param_1 + 0x3d) != '\0')) {
      cVar1 = FUN_0010a37a(param_3,param_4,*(undefined4 *)(param_5 + 0x18));
      if (cVar1 != '\x01') goto LAB_0010a6a5;
    }
    uVar2 = 0;
  }
  return uVar2;
}




// Function: emit_verbose @ 0xa6db

void emit_verbose(undefined8 param_1,undefined8 param_2,long param_3)

{
  undefined8 uVar1;
  undefined8 uVar2;
  char *__format;
  
  uVar1 = FUN_00118941(1,4,param_2);
  uVar2 = FUN_00118941(0,4,param_1);
  printf("%s -> %s",uVar2,uVar1);
  if (param_3 != 0) {
    uVar1 = FUN_00118a0f(4,param_3);
    __format = (char *)gettext(" (backup: %s)");
    printf(__format,uVar1);
  }
  putchar_unlocked(10);
  return;
}




// Function: restore_default_fscreatecon_or_die @ 0xa788

void restore_default_fscreatecon_or_die(void)

{
  int iVar1;
  undefined8 uVar2;
  int *piVar3;
  
  iVar1 = setfscreatecon(0);
  if (iVar1 != 0) {
    uVar2 = gettext("failed to restore the default file creation context");
    piVar3 = __errno_location();
    error(1,*piVar3,uVar2);
  }
  return;
}




// Function: subst_suffix @ 0xa7d7

void subst_suffix(void *param_1,long param_2,char *param_3)

{
  size_t __n;
  size_t sVar1;
  void *__dest;
  
  __n = param_2 - (long)param_1;
  sVar1 = strlen(param_3);
  __dest = (void *)FUN_0011b967(sVar1 + 1 + __n);
  memcpy((void *)(__n + (long)__dest),param_3,sVar1 + 1);
  memcpy(__dest,param_1,__n);
  return;
}




// Function: create_hard_link @ 0xa85d

undefined8
create_hard_link(void *param_1,undefined4 param_2,undefined8 param_3,undefined8 param_4,
            undefined4 param_5,undefined8 param_6,undefined1 param_7,char param_8,char param_9)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  char *__format;
  undefined8 uVar4;
  void *local_30;
  void *local_20;
  
  if (param_9 == '\0') {
    uVar4 = 0;
  }
  else {
    uVar4 = 0x400;
  }
  iVar1 = FUN_0010e069(param_2,param_3,param_5,param_6,uVar4,param_7,0xffffffffffffffff);
  if (iVar1 < 1) {
    if ((iVar1 < 0) && (param_8 != '\0')) {
      uVar4 = FUN_00118a0f(4,param_4);
      __format = (char *)gettext("removed %s\n");
      printf(__format,uVar4);
    }
    uVar4 = 1;
  }
  else {
    local_20 = (void *)0x0;
    local_30 = param_1;
    if (param_1 == (void *)0x0) {
      local_30 = (void *)FUN_0010a7d7(param_4,param_6,param_3);
      local_20 = local_30;
    }
    uVar4 = FUN_00118941(1,4,local_30);
    uVar2 = FUN_00118941(0,4,param_4);
    uVar3 = gettext("cannot create hard link %s to %s");
    error(0,iVar1,uVar3,uVar2,uVar4);
    free(local_20);
    uVar4 = 0;
  }
  return uVar4;
}




// Function: should_dereference @ 0xa9cc

undefined4 should_dereference(long param_1,char param_2)

{
  undefined4 uVar1;
  
  if ((*(int *)(param_1 + 4) == 4) || ((*(int *)(param_1 + 4) == 3 && (param_2 != '\0')))) {
    uVar1 = 1;
  }
  else {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: source_is_dst_backup @ 0xaa08

undefined4 source_is_dst_backup(char *param_1,__dev_t *param_2,int param_3,char *param_4)

{
  int iVar1;
  undefined4 uVar2;
  size_t sVar3;
  char *pcVar4;
  size_t __n;
  size_t sVar5;
  long in_FS_OFFSET;
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  sVar3 = strlen(param_1);
  pcVar4 = (char *)FUN_00110369(param_4);
  __n = strlen(pcVar4);
  sVar5 = strlen(DAT_001275e8);
  if (((sVar3 == sVar5 + __n) && (iVar1 = memcmp(param_1,pcVar4,__n), iVar1 == 0)) &&
     (iVar1 = strcmp(param_1 + __n,DAT_001275e8), pcVar4 = DAT_001275e8, iVar1 == 0)) {
    sVar3 = strlen(param_4);
    pcVar4 = (char *)FUN_0010a7d7(param_4,param_4 + sVar3,pcVar4);
    iVar1 = fstatat(param_3,pcVar4,&local_b8,0);
    free(pcVar4);
    if (((iVar1 == 0) && (param_2[1] == local_b8.st_ino)) && (*param_2 == local_b8.st_dev)) {
      uVar2 = 1;
    }
    else {
      uVar2 = 0;
    }
  }
  else {
    uVar2 = 0;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: copy_internal @ 0xabda

undefined1
copy_internal(char *param_1,char *param_2,int param_3,char *param_4,uint param_5,__dev_t *param_6,
            undefined8 param_7,int *param_8,char param_9,char *param_10,undefined1 *param_11,
            byte *param_12)

{
  long lVar1;
  bool bVar2;
  stat *psVar3;
  size_t __n;
  __dev_t _Var4;
  __uid_t _Var5;
  __gid_t _Var6;
  __off_t _Var7;
  byte bVar8;
  char cVar9;
  undefined1 uVar10;
  int iVar11;
  uint uVar12;
  int iVar13;
  int *piVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  size_t sVar18;
  ulong uVar19;
  void *pvVar20;
  char *pcVar21;
  char *pcVar22;
  char *pcVar23;
  undefined1 *puVar24;
  undefined1 *puVar25;
  undefined1 *puVar26;
  undefined1 *puVar27;
  undefined1 *puVar28;
  long in_FS_OFFSET;
  undefined1 auStack_3a8 [8];
  byte *local_3a0;
  undefined1 *local_398;
  char *local_390;
  char local_384;
  int *local_380;
  undefined8 local_378;
  __dev_t *local_370;
  char *local_368;
  uint local_360;
  int local_35c;
  char *local_358;
  char *local_350;
  bool local_344;
  char local_343;
  char local_342;
  undefined1 local_341;
  char local_340;
  char local_33f;
  char local_33e;
  char local_33d;
  byte local_33c;
  char local_33b;
  byte local_33a;
  char local_339;
  uint local_338;
  uint local_334;
  uint local_330;
  int local_32c;
  int local_328;
  int local_324;
  undefined4 local_320;
  undefined4 local_31c;
  undefined4 local_318;
  uint local_314;
  uint local_310;
  uint local_30c;
  int local_308;
  uint local_304;
  int local_300;
  uint local_2fc;
  long local_2f8;
  void *local_2f0;
  stat *local_2e8;
  char *local_2e0;
  char *local_2d8;
  undefined8 local_2d0;
  undefined8 local_2c8;
  char *local_2c0;
  size_t local_2b8;
  size_t local_2b0;
  void *local_2a8;
  char *local_2a0;
  char *local_298;
  char *local_290;
  undefined8 *local_288;
  char *local_280;
  __dev_t local_278;
  undefined8 local_270;
  ulong local_268;
  uint local_260;
  __uid_t local_25c;
  __gid_t local_258;
  __dev_t local_250;
  undefined8 local_248;
  stat local_1e8;
  stat local_158;
  undefined1 local_c8 [152];
  long local_30;
  
  puVar27 = auStack_3a8;
  puVar24 = auStack_3a8;
  puVar25 = auStack_3a8;
  puVar28 = auStack_3a8;
  local_378 = param_7;
  local_380 = param_8;
  local_384 = param_9;
  local_390 = param_10;
  local_398 = param_11;
  local_3a0 = param_12;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_342 = '\0';
  local_2f8 = 0;
  local_2f0 = (void *)0x0;
  local_340 = '\0';
  local_33f = '\0';
  local_33e = '\0';
  local_344 = 0 < (int)param_5;
  *param_11 = 0;
  local_32c = param_8[0x10];
  local_370 = param_6;
  local_368 = param_4;
  local_360 = param_5;
  local_35c = param_3;
  local_358 = param_2;
  local_350 = param_1;
  if ((char)param_8[6] != '\0') {
    if (local_32c < 0) {
      iVar11 = FUN_00118ece(0xffffff9c,param_1,param_3,param_4,1);
      if (iVar11 == 0) {
        local_32c = 0;
      }
      else {
        piVar14 = __errno_location();
        local_32c = *piVar14;
      }
    }
    local_344 = local_32c == 0;
    *local_3a0 = local_344;
    local_360 = (uint)*local_3a0;
  }
  if (local_32c == 0) {
    bVar8 = *(byte *)((long)local_380 + 0x3f) ^ 1;
  }
  else if ((local_32c == 0x11) && (local_380[2] == 2)) {
    bVar8 = 0;
  }
  else {
    bVar8 = 1;
  }
  if (bVar8 != 0) {
    local_2d8 = local_368;
    local_324 = local_35c;
    local_2e0 = local_358;
    if (local_32c != 0) {
      local_324 = -100;
      local_2d8 = local_350;
      local_2e0 = local_350;
    }
    if (local_380[1] == 2) {
      local_320 = 0x100;
    }
    else {
      local_320 = 0;
    }
    iVar11 = FUN_00106cbb(local_324,local_2d8,&local_278,local_320);
    if (iVar11 != 0) {
      uVar15 = FUN_00118a0f(4,local_2e0);
      uVar16 = gettext("cannot stat %s");
      piVar14 = __errno_location();
      error(0,*piVar14,uVar16,uVar15);
      puVar28 = auStack_3a8;
      uVar10 = 0;
      goto LAB_0010d964;
    }
    local_338 = local_260;
    if (((local_260 & 0xf000) == 0x4000) && ((char)local_380[0xe] != '\x01')) {
      uVar15 = FUN_00118a0f(4,local_350);
      if (*(char *)((long)local_380 + 0x19) == '\x01') {
        uVar16 = gettext("omitting directory %s");
      }
      else {
        uVar16 = gettext("-r not specified; omitting directory %s");
      }
      error(0,0,uVar16,uVar15);
      puVar28 = auStack_3a8;
      uVar10 = 0;
      goto LAB_0010d964;
    }
  }
  if ((local_384 != '\0') && (*(long *)(local_380 + 0x14) != 0)) {
    if (((local_338 & 0xf000) != 0x4000) &&
       ((*local_380 == 0 &&
        (cVar9 = FUN_00111a7b(*(undefined8 *)(local_380 + 0x14),local_350,&local_278), cVar9 != '\0'
        )))) {
      uVar15 = FUN_00118a0f(4,local_350);
      uVar16 = gettext("warning: source file %s specified more than once");
      error(0,0,uVar16,uVar15);
      puVar28 = auStack_3a8;
      uVar10 = 1;
      goto LAB_0010d964;
    }
    FUN_001119db(*(undefined8 *)(local_380 + 0x14),local_350,&local_278);
  }
  local_33c = FUN_0010a9cc(local_380,local_384);
  puVar26 = auStack_3a8;
  if ((int)local_360 < 1) {
    if ((local_32c != 0x11) || (local_380[2] != 2)) {
      if (((((local_338 & 0xf000) == 0x8000) ||
           ((((char)local_380[5] == '\x01' && ((local_338 & 0xf000) != 0x4000)) &&
            ((local_338 & 0xf000) != 0xa000)))) &&
          ((((char)local_380[6] == '\0' && (*(char *)((long)local_380 + 0x3a) == '\0')) &&
           (*(char *)((long)local_380 + 0x17) == '\0')))) &&
         ((*local_380 == 0 && (*(char *)((long)local_380 + 0x15) == '\0')))) {
        local_33b = '\0';
      }
      else {
        local_33b = '\x01';
      }
      if (local_33b == '\0') {
        local_31c = 0;
      }
      else {
        local_31c = 0x100;
      }
      if ((local_33b == '\x01') || (-1 < (int)local_360)) {
        iVar11 = FUN_00106cbb(local_35c,local_368,&local_1e8,local_31c);
        if (iVar11 == 0) {
          local_33e = local_33b;
          local_32c = 0x11;
        }
        else {
          piVar14 = __errno_location();
          if ((*piVar14 != 0x28) || (*(char *)((long)local_380 + 0x16) == '\0')) {
            piVar14 = __errno_location();
            if (*piVar14 != 2) {
              uVar15 = FUN_00118a0f(4,local_358);
              uVar16 = gettext("cannot stat %s");
              piVar14 = __errno_location();
              error(0,*piVar14,uVar16,uVar15);
              uVar10 = 0;
              goto LAB_0010d964;
            }
            local_344 = true;
          }
        }
      }
      else {
        local_344 = true;
      }
    }
    puVar26 = auStack_3a8;
    if (local_32c != 0x11) goto LAB_0010bbf1;
    local_343 = '\0';
    if ((local_380[2] != 2) &&
       (cVar9 = FUN_00109b18(local_350,&local_278,local_35c,local_368,&local_1e8,local_380,
                             &local_343), cVar9 != '\x01')) {
      uVar15 = FUN_00118941(1,4,local_358);
      uVar16 = FUN_00118941(0,4,local_350);
      uVar17 = gettext("%s and %s are the same file");
      error(0,0,uVar17,uVar16,uVar15);
      puVar28 = auStack_3a8;
      uVar10 = 0;
      goto LAB_0010d964;
    }
    if ((*(char *)((long)local_380 + 0x3b) == '\0') || ((local_338 & 0xf000) == 0x4000)) {
LAB_0010b410:
      if ((char)local_380[6] == '\0') {
        if (((local_338 & 0xf000) != 0x4000) &&
           ((local_380[2] == 2 ||
            ((local_380[2] == 3 &&
             (cVar9 = FUN_0010a3d5(local_380,local_358,local_35c,local_368,&local_1e8),
             cVar9 != '\x01')))))) {
          puVar28 = auStack_3a8;
          uVar10 = 1;
          goto LAB_0010d964;
        }
      }
      else {
        cVar9 = FUN_0010a603(local_380,local_358,local_35c,local_368,&local_1e8);
        if (cVar9 != '\0') {
          if (local_3a0 != (byte *)0x0) {
            *local_3a0 = 1;
          }
          puVar28 = auStack_3a8;
          uVar10 = 1;
          goto LAB_0010d964;
        }
      }
      if (local_343 != '\0') {
        puVar28 = auStack_3a8;
        uVar10 = 1;
        goto LAB_0010d964;
      }
      if ((local_1e8.st_mode & 0xf000) != 0x4000) {
        if (((local_338 & 0xf000) == 0x4000) && (((char)local_380[6] == '\0' || (*local_380 == 0))))
        {
          uVar15 = FUN_00118941(1,4,local_350);
          uVar16 = FUN_00118941(0,4,local_358);
          uVar17 = gettext("cannot overwrite non-directory %s with directory %s");
          error(0,0,uVar17,uVar16,uVar15);
          puVar28 = auStack_3a8;
          uVar10 = 0;
          goto LAB_0010d964;
        }
        if (((local_384 != '\0') && (*local_380 != 3)) &&
           (cVar9 = FUN_00111a7b(*(undefined8 *)(local_380 + 0x12),local_368,&local_1e8),
           cVar9 != '\0')) {
          uVar15 = FUN_00118941(1,4,local_350);
          uVar16 = FUN_00118941(0,4,local_358);
          uVar17 = gettext("will not overwrite just-created %s with %s");
          error(0,0,uVar17,uVar16,uVar15);
          puVar28 = auStack_3a8;
          uVar10 = 0;
          goto LAB_0010d964;
        }
      }
      if ((((local_338 & 0xf000) != 0x4000) && ((local_1e8.st_mode & 0xf000) == 0x4000)) &&
         (((char)local_380[6] == '\0' || (*local_380 == 0)))) {
        uVar15 = FUN_00118a0f(4,local_358);
        uVar16 = gettext("cannot overwrite directory %s with non-directory");
        error(0,0,uVar16,uVar15);
        puVar28 = auStack_3a8;
        uVar10 = 0;
        goto LAB_0010d964;
      }
      if ((((char)local_380[6] != '\0') && ((local_260 & 0xf000) == 0x4000)) &&
         (((local_1e8.st_mode & 0xf000) != 0x4000 && (*local_380 == 0)))) {
        uVar15 = FUN_00118ba1(0,3,local_358);
        uVar16 = FUN_00118ba1(0,3,local_350);
        uVar17 = gettext("cannot move directory onto non-directory: %s -> %s");
        error(0,0,uVar17,uVar16,uVar15);
        puVar28 = auStack_3a8;
        uVar10 = 0;
        goto LAB_0010d964;
      }
      if (*local_380 != 0) {
        local_2d0 = FUN_00110369(local_350);
        cVar9 = FUN_00106aa4(local_2d0);
        if ((cVar9 != '\x01') &&
           (((char)local_380[6] != '\0' || ((local_1e8.st_mode & 0xf000) != 0x4000)))) {
          if ((*local_380 != 3) &&
             (cVar9 = FUN_0010aa08(local_2d0,&local_278,local_35c,local_368), cVar9 != '\0')) {
            if ((char)local_380[6] == '\0') {
              local_2c8 = gettext("backing up %s might destroy source;  %s not copied");
            }
            else {
              local_2c8 = gettext("backing up %s might destroy source;  %s not moved");
            }
            uVar15 = FUN_00118941(1,4,local_350);
            uVar16 = FUN_00118941(0,4,local_358);
            error(0,0,local_2c8,uVar16,uVar15);
            puVar28 = auStack_3a8;
            uVar10 = 0;
            goto LAB_0010d964;
          }
          local_2c0 = (char *)FUN_0011021f(local_35c,local_368,*local_380);
          if (local_2c0 == (char *)0x0) {
            piVar14 = __errno_location();
            if (*piVar14 != 2) {
              uVar15 = FUN_00118a0f(4,local_358);
              uVar16 = gettext("cannot backup %s");
              piVar14 = __errno_location();
              error(0,*piVar14,uVar16,uVar15);
              puVar28 = auStack_3a8;
              uVar10 = 0;
              goto LAB_0010d964;
            }
          }
          else {
            local_2b8 = (long)local_368 - (long)local_358;
            sVar18 = strlen(local_2c0);
            local_2b0 = sVar18 + 1;
            uVar19 = ((sVar18 + 1 + local_2b8 + 0x17) / 0x10) * 0x10;
            for (; puVar24 != auStack_3a8 + -(uVar19 & 0xfffffffffffff000);
                puVar24 = puVar24 + -0x1000) {
              *(undefined8 *)(puVar24 + -8) = *(undefined8 *)(puVar24 + -8);
            }
            lVar1 = -(ulong)((uint)uVar19 & 0xfff);
            puVar25 = puVar24 + lVar1;
            if ((uVar19 & 0xfff) != 0) {
              *(undefined8 *)(puVar24 + ((ulong)((uint)uVar19 & 0xfff) - 8) + lVar1) =
                   *(undefined8 *)(puVar24 + ((ulong)((uint)uVar19 & 0xfff) - 8) + lVar1);
            }
            __n = local_2b0;
            sVar18 = local_2b8;
            pcVar23 = local_358;
            local_2f0 = (void *)((ulong)(puVar24 + lVar1 + 0xf) & 0xfffffffffffffff0);
            *(undefined8 *)(puVar24 + lVar1 + -8) = 0x10ba03;
            pvVar20 = mempcpy((void *)((ulong)(puVar24 + lVar1 + 0xf) & 0xfffffffffffffff0),pcVar23,
                              sVar18);
            pcVar23 = local_2c0;
            *(undefined8 *)(puVar24 + lVar1 + -8) = 0x10ba1b;
            memcpy(pvVar20,pcVar23,__n);
            pcVar23 = local_2c0;
            *(undefined8 *)(puVar24 + lVar1 + -8) = 0x10ba2a;
            free(pcVar23);
          }
          local_344 = true;
          puVar26 = puVar25;
          goto LAB_0010bbf1;
        }
      }
      puVar26 = auStack_3a8;
      if ((((local_1e8.st_mode & 0xf000) != 0x4000) &&
          (puVar26 = auStack_3a8, (char)local_380[6] != '\x01')) &&
         ((*(char *)((long)local_380 + 0x15) != '\0' ||
          ((puVar26 = auStack_3a8, *(char *)((long)local_380 + 0x31) != '\0' &&
           ((((char)local_380[0xc] != '\0' && (1 < local_1e8.st_nlink)) ||
            ((puVar26 = auStack_3a8, local_380[1] == 2 &&
             (puVar26 = auStack_3a8, (local_260 & 0xf000) != 0x8000)))))))))) {
        iVar11 = unlinkat(local_35c,local_368,0);
        if ((iVar11 != 0) && (piVar14 = __errno_location(), *piVar14 != 2)) {
          uVar15 = FUN_00118a0f(4,local_358);
          uVar16 = gettext("cannot remove %s");
          piVar14 = __errno_location();
          error(0,*piVar14,uVar16,uVar15);
          puVar28 = auStack_3a8;
          uVar10 = 0;
          goto LAB_0010d964;
        }
        local_344 = true;
        puVar26 = auStack_3a8;
        if ((char)local_380[0xf] != '\0') {
          uVar15 = FUN_00118a0f(4,local_358);
          pcVar23 = (char *)gettext("removed %s\n");
          printf(pcVar23,uVar15);
          puVar26 = auStack_3a8;
        }
      }
      goto LAB_0010bbf1;
    }
    if ((*(char *)((long)local_380 + 0x1f) == '\0') ||
       (((char)local_380[6] == '\x01' && (local_1e8.st_dev == local_278)))) {
      local_318 = 0;
    }
    else {
      local_318 = 1;
    }
    iVar11 = FUN_0011a52e(local_35c,local_368,&local_1e8,&local_278,local_318);
    if (iVar11 < 0) goto LAB_0010b410;
    if (local_3a0 != (byte *)0x0) {
      *local_3a0 = 1;
    }
    local_2f8 = FUN_0010de9c(local_368,local_270,local_278);
    if ((local_2f8 == 0) ||
       (cVar9 = FUN_0010a85d(0,local_35c,local_2f8,local_358,local_35c,local_368,1,
                             (char)local_380[0xf],local_33c), cVar9 == '\x01')) {
      puVar28 = auStack_3a8;
      uVar10 = 1;
      goto LAB_0010d964;
    }
  }
  else {
LAB_0010bbf1:
    puVar28 = puVar26;
    iVar11 = local_35c;
    pcVar23 = local_368;
    if ((((local_384 != '\0') && (*(long *)(local_380 + 0x12) != 0)) &&
        ((char)local_380[6] != '\x01')) && (*local_380 == 0)) {
      local_33d = '\x01';
      if (local_33e == '\0') {
        *(undefined8 *)(puVar28 + -8) = 0x10bc79;
        iVar11 = fstatat(iVar11,pcVar23,(stat *)local_c8,0x100);
        if (iVar11 == 0) {
          local_2e8 = (stat *)local_c8;
        }
        else {
          local_33d = '\0';
        }
      }
      else {
        local_2e8 = &local_1e8;
      }
      psVar3 = local_2e8;
      pcVar23 = local_368;
      if ((local_33d != '\0') && ((local_2e8->st_mode & 0xf000) == 0xa000)) {
        uVar15 = *(undefined8 *)(local_380 + 0x12);
        *(undefined8 *)(puVar28 + -8) = 0x10bcdf;
        cVar9 = FUN_00111a7b(uVar15,pcVar23,psVar3);
        pcVar23 = local_358;
        if (cVar9 != '\0') {
          *(undefined8 *)(puVar28 + -8) = 0x10bcfc;
          uVar15 = FUN_00118941(1,4,pcVar23);
          pcVar23 = local_350;
          *(undefined8 *)(puVar28 + -8) = 0x10bd18;
          uVar16 = FUN_00118941(0,4,pcVar23);
          *(undefined8 *)(puVar28 + -8) = 0x10bd2a;
          uVar17 = gettext("will not copy %s through just-created symlink %s");
          *(undefined8 *)(puVar28 + -8) = 0x10bd47;
          error(0,0,uVar17,uVar16,uVar15);
          uVar10 = 0;
          goto LAB_0010d964;
        }
      }
    }
    pvVar20 = local_2f0;
    pcVar22 = local_350;
    pcVar23 = local_358;
    if ((((char)local_380[0xf] != '\0') && ((char)local_380[6] != '\x01')) &&
       ((local_338 & 0xf000) != 0x4000)) {
      *(undefined8 *)(puVar28 + -8) = 0x10bda4;
      FUN_0010a6db(pcVar22,pcVar23,pvVar20);
    }
    uVar15 = local_270;
    _Var4 = local_278;
    pcVar23 = local_368;
    if (local_32c == 0) {
      local_2f8 = 0;
    }
    else if (((char)local_380[0xe] == '\0') || ((local_338 & 0xf000) != 0x4000)) {
      if (((char)local_380[6] == '\0') || (local_268 != 1)) {
        if ((((char)local_380[0xc] != '\0') && (*(char *)((long)local_380 + 0x17) != '\x01')) &&
           ((1 < local_268 || (((local_384 != '\0' && (local_380[1] == 3)) || (local_380[1] == 4))))
           )) {
          *(undefined8 *)(puVar28 + -8) = 0x10beeb;
          local_2f8 = FUN_0010de9c(pcVar23,uVar15,_Var4);
        }
      }
      else {
        *(undefined8 *)(puVar28 + -8) = 0x10be6d;
        local_2f8 = FUN_0010de23(uVar15,_Var4);
      }
    }
    else if (local_384 == '\0') {
      *(undefined8 *)(puVar28 + -8) = 0x10be2c;
      local_2f8 = FUN_0010de23(uVar15,_Var4);
    }
    else {
      *(undefined8 *)(puVar28 + -8) = 0x10be07;
      local_2f8 = FUN_0010de9c(pcVar23,uVar15,_Var4);
    }
    lVar1 = local_2f8;
    pcVar21 = local_350;
    pcVar22 = local_358;
    iVar11 = local_35c;
    pcVar23 = local_368;
    puVar27 = puVar28;
    if (local_2f8 == 0) {
LAB_0010c172:
      pcVar22 = local_350;
      iVar11 = local_35c;
      pcVar23 = local_368;
      if ((char)local_380[6] != '\0') {
        if (local_32c == 0x11) {
          *(undefined8 *)(puVar28 + -8) = 0x10c1af;
          iVar11 = renameat(-100,pcVar22,iVar11,pcVar23);
          if (iVar11 == 0) {
            local_32c = 0;
          }
          else {
            *(undefined8 *)(puVar28 + -8) = 0x10c1b8;
            piVar14 = __errno_location();
            local_32c = *piVar14;
          }
        }
        pcVar22 = local_358;
        iVar11 = local_35c;
        pcVar23 = local_368;
        if (local_32c == 0) {
          if ((char)local_380[0xf] != '\0') {
            *(undefined8 *)(puVar28 + -8) = 0x10c1f2;
            pcVar23 = (char *)gettext("renamed ");
            *(undefined8 *)(puVar28 + -8) = 0x10c1ff;
            printf(pcVar23);
            pvVar20 = local_2f0;
            pcVar22 = local_350;
            pcVar23 = local_358;
            *(undefined8 *)(puVar28 + -8) = 0x10c21f;
            FUN_0010a6db(pcVar22,pcVar23,pvVar20);
          }
          pcVar23 = local_358;
          piVar14 = local_380;
          if (*(long *)(local_380 + 10) != 0) {
            *(undefined8 *)(puVar28 + -8) = 0x10c24a;
            FUN_00108750(pcVar23,1,piVar14);
          }
          pcVar23 = local_368;
          if (local_3a0 != (byte *)0x0) {
            *local_3a0 = 1;
          }
          if ((local_384 != '\0') && (*(char *)((long)local_380 + 0x3f) != '\x01')) {
            uVar15 = *(undefined8 *)(local_380 + 0x12);
            *(undefined8 *)(puVar28 + -8) = 0x10c29d;
            FUN_001119db(uVar15,pcVar23,&local_278);
          }
          uVar10 = 1;
          goto LAB_0010d964;
        }
        if (local_32c == 0x16) {
          *(undefined8 *)(puVar28 + -8) = 0x10c2c9;
          uVar15 = FUN_00118941(1,4,DAT_001271a8);
          *(undefined8 *)(puVar28 + -8) = 0x10c2e5;
          uVar16 = FUN_00118941(0,4,DAT_001271a0);
          *(undefined8 *)(puVar28 + -8) = 0x10c2f7;
          uVar17 = gettext("cannot move %s to a subdirectory of itself, %s");
          *(undefined8 *)(puVar28 + -8) = 0x10c314;
          error(0,0,uVar17,uVar16,uVar15);
          *local_398 = 1;
          uVar10 = 1;
          goto LAB_0010d964;
        }
        if (local_32c != 0x12) {
          *(undefined8 *)(puVar28 + -8) = 0x10c34e;
          uVar15 = FUN_00118941(1,4,pcVar22);
          pcVar23 = local_350;
          *(undefined8 *)(puVar28 + -8) = 0x10c36a;
          uVar16 = FUN_00118941(0,4,pcVar23);
          *(undefined8 *)(puVar28 + -8) = 0x10c37c;
          uVar17 = gettext("cannot move %s to %s");
          iVar11 = local_32c;
          *(undefined8 *)(puVar28 + -8) = 0x10c39c;
          error(0,iVar11,uVar17,uVar16,uVar15);
          *(undefined8 *)(puVar28 + -8) = 0x10c3b5;
          FUN_0010dda4(local_270,local_278);
          uVar10 = 0;
          goto LAB_0010d964;
        }
        if ((local_338 & 0xf000) == 0x4000) {
          iVar13 = 0x200;
        }
        else {
          iVar13 = 0;
        }
        *(undefined8 *)(puVar28 + -8) = 0x10c3f4;
        iVar11 = unlinkat(iVar11,pcVar23,iVar13);
        if (iVar11 != 0) {
          *(undefined8 *)(puVar28 + -8) = 0x10c401;
          piVar14 = __errno_location();
          pcVar23 = local_358;
          if (*piVar14 != 2) {
            *(undefined8 *)(puVar28 + -8) = 0x10c425;
            uVar15 = FUN_00118941(1,4,pcVar23);
            pcVar23 = local_350;
            *(undefined8 *)(puVar28 + -8) = 0x10c441;
            uVar16 = FUN_00118941(0,4,pcVar23);
            *(undefined8 *)(puVar28 + -8) = 0x10c453;
            uVar17 = gettext("inter-device move failed: %s to %s; unable to remove target");
            *(undefined8 *)(puVar28 + -8) = 0x10c45b;
            piVar14 = __errno_location();
            iVar11 = *piVar14;
            *(undefined8 *)(puVar28 + -8) = 0x10c477;
            error(0,iVar11,uVar17,uVar16,uVar15);
            *(undefined8 *)(puVar28 + -8) = 0x10c490;
            FUN_0010dda4(local_270,local_278);
            uVar10 = 0;
            goto LAB_0010d964;
          }
        }
        if (((char)local_380[0xf] != '\0') && ((local_338 & 0xf000) != 0x4000)) {
          *(undefined8 *)(puVar28 + -8) = 0x10c4ca;
          pcVar23 = (char *)gettext("copied ");
          *(undefined8 *)(puVar28 + -8) = 0x10c4d7;
          printf(pcVar23);
          pvVar20 = local_2f0;
          pcVar22 = local_350;
          pcVar23 = local_358;
          *(undefined8 *)(puVar28 + -8) = 0x10c4f7;
          FUN_0010a6db(pcVar22,pcVar23,pvVar20);
        }
        local_344 = true;
      }
      uVar12 = local_338;
      bVar2 = local_344;
      pcVar22 = local_350;
      pcVar23 = local_358;
      piVar14 = local_380;
      local_314 = local_338;
      if (*(char *)((long)local_380 + 0x39) != '\0') {
        local_314 = local_380[4];
      }
      local_314 = local_314 & 0xfff;
      if (*(char *)((long)local_380 + 0x1d) == '\x01') {
        local_330 = 0x3f;
      }
      else if ((local_338 & 0xf000) == 0x4000) {
        local_330 = 0x12;
      }
      else {
        local_330 = 0;
      }
      local_330 = local_330 & local_314;
      local_341 = 1;
      *(undefined8 *)(puVar28 + -8) = 0x10c5a9;
      cVar9 = FUN_001084c6(pcVar22,pcVar23,uVar12,bVar2,piVar14);
      pcVar21 = local_350;
      pcVar22 = local_358;
      iVar11 = local_35c;
      pcVar23 = local_368;
      uVar15 = local_378;
      piVar14 = local_380;
      if (cVar9 != '\x01') {
        uVar10 = 0;
        goto LAB_0010d964;
      }
      if ((local_338 & 0xf000) == 0x4000) {
        *(undefined8 *)(puVar28 + -8) = 0x10c5e9;
        cVar9 = FUN_00107a8e(&local_278,uVar15);
        pcVar23 = local_350;
        puVar26 = puVar28;
        if (cVar9 == '\0') {
          for (; puVar26 != puVar28; puVar26 = puVar26 + -0x1000) {
            *(undefined8 *)(puVar26 + -8) = *(undefined8 *)(puVar26 + -8);
          }
          puVar27 = puVar26 + -0x20;
          *(undefined8 *)(puVar26 + -8) = *(undefined8 *)(puVar26 + -8);
          pcVar22 = local_358;
          iVar11 = local_35c;
          pcVar23 = local_368;
          piVar14 = local_380;
          local_288 = (undefined8 *)((ulong)(puVar26 + -0x11) & 0xfffffffffffffff0);
          *local_288 = local_378;
          local_288[1] = local_270;
          local_288[2] = local_278;
          if ((local_344 == false) && ((local_1e8.st_mode & 0xf000) == 0x4000)) {
            local_330 = 0;
            if ((*(long *)(local_380 + 10) != 0) || (*(char *)((long)local_380 + 0x33) != '\0')) {
              *(undefined8 *)(puVar26 + -0x28) = 0x10c997;
              cVar9 = FUN_00108750(pcVar22,0,piVar14);
              if ((cVar9 != '\x01') && ((char)local_380[0xd] != '\0')) goto LAB_0010d821;
            }
LAB_0010c9b4:
            bVar2 = local_344;
            pcVar21 = local_350;
            pcVar22 = local_358;
            iVar11 = local_35c;
            pcVar23 = local_368;
            if ((((char)local_380[7] == '\0') || (local_370 == (__dev_t *)0x0)) ||
               (puVar28 = puVar26 + -0x20, *local_370 == local_278)) {
              *(undefined1 **)(puVar26 + -0x28) = local_398;
              *(char **)(puVar26 + -0x30) = local_390;
              *(int **)(puVar26 + -0x38) = local_380;
              *(undefined8 **)(puVar26 + -0x40) = local_288;
              *(undefined8 *)(puVar26 + -0x48) = 0x10ca39;
              local_341 = FUN_00107ee6(pcVar21,pcVar22,iVar11,pcVar23,bVar2,&local_278);
              puVar28 = puVar26 + -0x20;
            }
            goto LAB_0010d20a;
          }
          uVar12 = ~local_330 & local_314;
          local_304 = uVar12;
          *(undefined8 *)(puVar26 + -0x28) = 0x10c73e;
          iVar13 = mkdirat(iVar11,pcVar23,uVar12);
          pcVar22 = local_358;
          iVar11 = local_35c;
          pcVar23 = local_368;
          if (iVar13 == 0) {
            *(undefined8 *)(puVar26 + -0x28) = 0x10c7ae;
            iVar13 = fstatat(iVar11,pcVar23,&local_1e8,0x100);
            pcVar22 = local_358;
            iVar11 = local_35c;
            pcVar23 = local_368;
            if (iVar13 == 0) {
              if ((local_1e8.st_mode & 0x1c0) != 0x1c0) {
                local_334 = local_1e8.st_mode;
                local_342 = '\x01';
                uVar12 = local_1e8.st_mode | 0x1c0;
                *(undefined8 *)(puVar26 + -0x28) = 0x10c84b;
                iVar11 = FUN_00111260(iVar11,pcVar23,uVar12);
                pcVar23 = local_358;
                if (iVar11 != 0) {
                  *(undefined8 *)(puVar26 + -0x28) = 0x10c863;
                  uVar15 = FUN_00118a0f(4,pcVar23);
                  *(undefined8 *)(puVar26 + -0x28) = 0x10c875;
                  uVar16 = gettext("setting permissions for %s");
                  *(undefined8 *)(puVar26 + -0x28) = 0x10c87d;
                  piVar14 = __errno_location();
                  iVar11 = *piVar14;
                  *(undefined8 *)(puVar26 + -0x28) = 0x10c896;
                  error(0,iVar11,uVar16,uVar15);
                  puVar27 = puVar26 + -0x20;
                  goto LAB_0010d821;
                }
              }
              pcVar23 = local_368;
              if (*local_390 != '\x01') {
                *(undefined8 *)(puVar26 + -0x28) = 0x10c8cc;
                FUN_0010de9c(pcVar23,local_1e8.st_ino,local_1e8.st_dev);
                *local_390 = '\x01';
              }
              pcVar22 = local_350;
              pcVar23 = local_358;
              if ((char)local_380[0xf] != '\0') {
                if ((char)local_380[6] == '\0') {
                  *(undefined8 *)(puVar26 + -0x28) = 0x10c951;
                  FUN_0010a6db(pcVar22,pcVar23,0);
                }
                else {
                  *(undefined8 *)(puVar26 + -0x28) = 0x10c90c;
                  uVar15 = FUN_00118a0f(4,pcVar23);
                  *(undefined8 *)(puVar26 + -0x28) = 0x10c91e;
                  pcVar23 = (char *)gettext("created directory %s\n");
                  *(undefined8 *)(puVar26 + -0x28) = 0x10c92e;
                  printf(pcVar23,uVar15);
                }
              }
              goto LAB_0010c9b4;
            }
            *(undefined8 *)(puVar26 + -0x28) = 0x10c7c6;
            uVar15 = FUN_00118a0f(4,pcVar22);
            *(undefined8 *)(puVar26 + -0x28) = 0x10c7d8;
            uVar16 = gettext("cannot stat %s");
            *(undefined8 *)(puVar26 + -0x28) = 0x10c7e0;
            piVar14 = __errno_location();
            iVar11 = *piVar14;
            *(undefined8 *)(puVar26 + -0x28) = 0x10c7f9;
            error(0,iVar11,uVar16,uVar15);
            puVar27 = puVar26 + -0x20;
          }
          else {
            *(undefined8 *)(puVar26 + -0x28) = 0x10c756;
            uVar15 = FUN_00118a0f(4,pcVar22);
            *(undefined8 *)(puVar26 + -0x28) = 0x10c768;
            uVar16 = gettext("cannot create directory %s");
            *(undefined8 *)(puVar26 + -0x28) = 0x10c770;
            piVar14 = __errno_location();
            iVar11 = *piVar14;
            *(undefined8 *)(puVar26 + -0x28) = 0x10c789;
            error(0,iVar11,uVar16,uVar15);
            puVar27 = puVar26 + -0x20;
          }
        }
        else {
          *(undefined8 *)(puVar28 + -8) = 0x10c601;
          uVar15 = FUN_00118a0f(4,pcVar23);
          *(undefined8 *)(puVar28 + -8) = 0x10c613;
          uVar16 = gettext("cannot copy cyclic symbolic link %s");
          *(undefined8 *)(puVar28 + -8) = 0x10c62d;
          error(0,0,uVar16,uVar15);
        }
      }
      else {
        if (*(char *)((long)local_380 + 0x3a) == '\0') {
          if (*(char *)((long)local_380 + 0x17) == '\0') {
            if (((local_338 & 0xf000) != 0x8000) &&
               (((char)local_380[5] == '\0' || ((local_338 & 0xf000) == 0xa000)))) {
              if ((local_338 & 0xf000) != 0x1000) {
                if ((((local_338 & 0xf000) == 0x6000) || ((local_338 & 0xf000) == 0x2000)) ||
                   ((local_338 & 0xf000) == 0xc000)) {
                  uVar12 = ~local_330 & local_338;
                  local_310 = uVar12;
                  *(undefined8 *)(puVar28 + -8) = 0x10cec5;
                  iVar11 = mknodat(iVar11,pcVar23,uVar12,local_250);
                  pcVar23 = local_358;
                  if (iVar11 == 0) goto LAB_0010d20a;
                  *(undefined8 *)(puVar28 + -8) = 0x10cee1;
                  uVar15 = FUN_00118a0f(4,pcVar23);
                  *(undefined8 *)(puVar28 + -8) = 0x10cef3;
                  uVar16 = gettext("cannot create special file %s");
                  *(undefined8 *)(puVar28 + -8) = 0x10cefb;
                  piVar14 = __errno_location();
                  iVar11 = *piVar14;
                  *(undefined8 *)(puVar28 + -8) = 0x10cf14;
                  error(0,iVar11,uVar16,uVar15);
                }
                else if ((local_338 & 0xf000) == 0xa000) {
                  *(undefined8 *)(puVar28 + -8) = 0x10cf4b;
                  pcVar21 = (char *)FUN_0010eded(pcVar21,local_248);
                  pcVar22 = local_350;
                  iVar11 = local_35c;
                  pcVar23 = local_368;
                  local_33f = '\x01';
                  local_2a0 = pcVar21;
                  if (pcVar21 == (char *)0x0) {
                    *(undefined8 *)(puVar28 + -8) = 0x10cf77;
                    uVar15 = FUN_00118a0f(4,pcVar22);
                    *(undefined8 *)(puVar28 + -8) = 0x10cf89;
                    uVar16 = gettext("cannot read symbolic link %s");
                    *(undefined8 *)(puVar28 + -8) = 0x10cf91;
                    piVar14 = __errno_location();
                    iVar11 = *piVar14;
                    *(undefined8 *)(puVar28 + -8) = 0x10cfaa;
                    error(0,iVar11,uVar16,uVar15);
                  }
                  else {
                    uVar10 = *(undefined1 *)((long)local_380 + 0x16);
                    *(undefined8 *)(puVar28 + -8) = 0x10cfdf;
                    local_328 = FUN_0010e27f(pcVar21,iVar11,pcVar23,uVar10,0xffffffff);
                    _Var7 = local_1e8.st_size;
                    pcVar23 = local_2a0;
                    if (((0 < local_328) && (*(char *)((long)local_380 + 0x3b) != '\0')) &&
                       ((local_344 != true && ((local_1e8.st_mode & 0xf000) == 0xa000)))) {
                      *(undefined8 *)(puVar28 + -8) = 0x10d043;
                      sVar18 = strlen(pcVar23);
                      iVar11 = local_35c;
                      pcVar23 = local_368;
                      if (_Var7 == sVar18) {
                        *(undefined8 *)(puVar28 + -8) = 0x10d069;
                        pcVar22 = (char *)FUN_0010f0b6(iVar11,pcVar23,local_1e8.st_size);
                        pcVar23 = local_2a0;
                        local_298 = pcVar22;
                        if (pcVar22 != (char *)0x0) {
                          *(undefined8 *)(puVar28 + -8) = 0x10d093;
                          iVar11 = strcmp(pcVar22,pcVar23);
                          pcVar23 = local_298;
                          if (iVar11 == 0) {
                            local_328 = 0;
                          }
                          *(undefined8 *)(puVar28 + -8) = 0x10d0b0;
                          free(pcVar23);
                        }
                      }
                    }
                    pcVar23 = local_2a0;
                    *(undefined8 *)(puVar28 + -8) = 0x10d0bf;
                    free(pcVar23);
                    pcVar23 = local_358;
                    if (local_328 < 1) {
                      if (*(char *)((long)local_380 + 0x33) != '\0') {
                        *(undefined8 *)(puVar28 + -8) = 0x10d124;
                        FUN_0010a788();
                      }
                      _Var6 = local_258;
                      _Var5 = local_25c;
                      iVar11 = local_35c;
                      pcVar23 = local_368;
                      if (*(char *)((long)local_380 + 0x1d) != '\0') {
                        *(undefined8 *)(puVar28 + -8) = 0x10d157;
                        iVar11 = FUN_001112c3(iVar11,pcVar23,_Var5,_Var6);
                        piVar14 = local_380;
                        if (iVar11 != 0) {
                          *(undefined8 *)(puVar28 + -8) = 0x10d16e;
                          cVar9 = FUN_0010dc1f(piVar14);
                          if (cVar9 != '\x01') {
                            *(undefined8 *)(puVar28 + -8) = 0x10d188;
                            uVar15 = gettext("failed to preserve ownership for %s");
                            *(undefined8 *)(puVar28 + -8) = 0x10d190;
                            piVar14 = __errno_location();
                            pcVar23 = local_358;
                            iVar11 = *piVar14;
                            *(undefined8 *)(puVar28 + -8) = 0x10d1b0;
                            error(0,iVar11,uVar15,pcVar23);
                            if (*(char *)((long)local_380 + 0x32) != '\0') goto LAB_0010d821;
                          }
                        }
                      }
                      goto LAB_0010d20a;
                    }
                    *(undefined8 *)(puVar28 + -8) = 0x10d0dc;
                    uVar15 = FUN_00118a0f(4,pcVar23);
                    *(undefined8 *)(puVar28 + -8) = 0x10d0ee;
                    uVar16 = gettext("cannot create symbolic link %s");
                    iVar11 = local_328;
                    *(undefined8 *)(puVar28 + -8) = 0x10d10b;
                    error(0,iVar11,uVar16,uVar15);
                  }
                }
                else {
                  *(undefined8 *)(puVar28 + -8) = 0x10d1d8;
                  uVar15 = FUN_00118a0f(4,pcVar21);
                  *(undefined8 *)(puVar28 + -8) = 0x10d1ea;
                  uVar16 = gettext("%s has unknown file type");
                  *(undefined8 *)(puVar28 + -8) = 0x10d204;
                  error(0,0,uVar16,uVar15);
                }
                goto LAB_0010d821;
              }
              uVar12 = ~local_330 & local_338;
              local_30c = uVar12;
              *(undefined8 *)(puVar28 + -8) = 0x10cdd8;
              iVar13 = mknodat(iVar11,pcVar23,uVar12,0);
              iVar11 = local_35c;
              pcVar23 = local_368;
              if (iVar13 != 0) {
                uVar12 = local_30c & 0xffffefff;
                *(undefined8 *)(puVar28 + -8) = 0x10ce02;
                iVar11 = mkfifoat(iVar11,pcVar23,uVar12);
                pcVar23 = local_358;
                if (iVar11 != 0) {
                  *(undefined8 *)(puVar28 + -8) = 0x10ce1e;
                  uVar15 = FUN_00118a0f(4,pcVar23);
                  *(undefined8 *)(puVar28 + -8) = 0x10ce30;
                  uVar16 = gettext("cannot create fifo %s");
                  *(undefined8 *)(puVar28 + -8) = 0x10ce38;
                  piVar14 = __errno_location();
                  iVar11 = *piVar14;
                  *(undefined8 *)(puVar28 + -8) = 0x10ce51;
                  error(0,iVar11,uVar16,uVar15);
                  goto LAB_0010d821;
                }
              }
              goto LAB_0010d20a;
            }
            local_340 = '\x01';
            uVar12 = local_314 & 0x1ff;
            *(__dev_t **)(puVar28 + -0x10) = &local_278;
            *(bool **)(puVar28 + -0x18) = &local_344;
            *(ulong *)(puVar28 + -0x20) = (ulong)local_330;
            *(undefined8 *)(puVar28 + -0x28) = 0x10cd7b;
            cVar9 = FUN_0010896f(pcVar21,pcVar22,iVar11,pcVar23,piVar14,uVar12);
          }
          else {
            if ((*(char *)((long)local_380 + 0x16) == '\0') && (local_380[2] != 3)) {
              local_33a = 0;
            }
            else {
              local_33a = 1;
            }
            uVar19 = (ulong)local_33a;
            *(ulong *)(puVar28 + -0x10) = (ulong)local_33c;
            *(undefined8 *)(puVar28 + -0x18) = 0;
            *(ulong *)(puVar28 + -0x20) = uVar19;
            *(undefined8 *)(puVar28 + -0x28) = 0x10ccd6;
            cVar9 = FUN_0010a85d(pcVar21,0xffffff9c,pcVar21,pcVar22,iVar11,pcVar23);
          }
          if (cVar9 == '\x01') goto LAB_0010d20a;
          goto LAB_0010d821;
        }
        local_33f = '\x01';
        if (*local_350 != '/') {
          *(undefined8 *)(puVar28 + -8) = 0x10ca83;
          pcVar23 = (char *)FUN_00111559(pcVar23);
          local_290 = pcVar23;
          if (local_35c == -100) {
            *(undefined8 *)(puVar28 + -8) = 0x10caac;
            iVar11 = strcmp(".",pcVar23);
            if (iVar11 != 0) goto LAB_0010cab0;
LAB_0010cb17:
            local_339 = '\x01';
          }
          else {
LAB_0010cab0:
            *(undefined8 *)(puVar28 + -8) = 0x10cac9;
            iVar13 = stat(".",&local_158);
            pcVar23 = local_290;
            iVar11 = local_35c;
            if (iVar13 != 0) goto LAB_0010cb17;
            *(undefined8 *)(puVar28 + -8) = 0x10caed;
            iVar11 = fstatat(iVar11,pcVar23,(stat *)local_c8,0);
            if ((iVar11 != 0) ||
               ((local_158.st_ino == local_c8._8_8_ && (local_158.st_dev == local_c8._0_8_))))
            goto LAB_0010cb17;
            local_339 = '\0';
          }
          pcVar23 = local_290;
          *(undefined8 *)(puVar28 + -8) = 0x10cb3f;
          free(pcVar23);
          pcVar23 = local_358;
          if (local_339 != '\x01') {
            *(undefined8 *)(puVar28 + -8) = 0x10cb66;
            uVar15 = FUN_00118ba1(0,3,pcVar23);
            *(undefined8 *)(puVar28 + -8) = 0x10cb78;
            uVar16 = gettext("%s: can make relative symbolic links only in current directory");
            *(undefined8 *)(puVar28 + -8) = 0x10cb92;
            error(0,0,uVar16,uVar15);
            goto LAB_0010d821;
          }
        }
        pcVar22 = local_350;
        iVar11 = local_35c;
        pcVar23 = local_368;
        uVar10 = *(undefined1 *)((long)local_380 + 0x16);
        *(undefined8 *)(puVar28 + -8) = 0x10cbc7;
        local_308 = FUN_0010e27f(pcVar22,iVar11,pcVar23,uVar10,0xffffffff);
        pcVar23 = local_350;
        if (local_308 < 1) {
LAB_0010d20a:
          pcVar23 = local_358;
          piVar14 = local_380;
          if (((local_344 != true) && ((char)local_380[5] != '\x01')) &&
             (((local_338 & 0xf000) != 0x4000 &&
              ((*(long *)(local_380 + 10) != 0 || (*(char *)((long)local_380 + 0x33) != '\0')))))) {
            *(undefined8 *)(puVar28 + -8) = 0x10d276;
            cVar9 = FUN_00108750(pcVar23,0,piVar14);
            if ((cVar9 != '\x01') && (puVar27 = puVar28, (char)local_380[0xd] != '\0'))
            goto LAB_0010d821;
          }
          iVar11 = local_35c;
          pcVar23 = local_368;
          if ((local_384 != '\0') && (*(long *)(local_380 + 0x12) != 0)) {
            *(undefined8 *)(puVar28 + -8) = 0x10d2c9;
            iVar11 = fstatat(iVar11,pcVar23,(stat *)local_c8,0x100);
            pcVar23 = local_368;
            if (iVar11 == 0) {
              uVar15 = *(undefined8 *)(local_380 + 0x12);
              *(undefined8 *)(puVar28 + -8) = 0x10d2f1;
              FUN_001119db(uVar15,pcVar23,local_c8);
            }
          }
          uVar10 = local_341;
          if (((*(char *)((long)local_380 + 0x17) == '\0') || ((local_338 & 0xf000) == 0x4000)) &&
             (local_340 == '\0')) {
            if (*(char *)((long)local_380 + 0x1f) != '\0') {
              *(undefined8 *)(puVar28 + -8) = 0x10d355;
              local_c8._0_16_ = FUN_00119e57(&local_278);
              *(undefined8 *)(puVar28 + -8) = 0x10d372;
              local_c8._16_16_ = FUN_00119e8b(&local_278);
              iVar11 = local_35c;
              pcVar23 = local_368;
              if (local_33f == '\0') {
                iVar13 = 0;
              }
              else {
                iVar13 = 0x100;
              }
              local_300 = iVar13;
              *(undefined8 *)(puVar28 + -8) = 0x10d3bc;
              iVar11 = utimensat(iVar11,pcVar23,(timespec *)local_c8,iVar13);
              pcVar23 = local_358;
              if (iVar11 != 0) {
                *(undefined8 *)(puVar28 + -8) = 0x10d3d4;
                uVar15 = FUN_00118a0f(4,pcVar23);
                *(undefined8 *)(puVar28 + -8) = 0x10d3e6;
                uVar16 = gettext("preserving times for %s");
                *(undefined8 *)(puVar28 + -8) = 0x10d3ee;
                piVar14 = __errno_location();
                iVar11 = *piVar14;
                *(undefined8 *)(puVar28 + -8) = 0x10d407;
                error(0,iVar11,uVar16,uVar15);
                if (*(char *)((long)local_380 + 0x32) != '\0') {
                  uVar10 = 0;
                  goto LAB_0010d964;
                }
              }
            }
            pcVar22 = local_358;
            iVar11 = local_35c;
            pcVar23 = local_368;
            piVar14 = local_380;
            if (((local_33f != '\x01') && (*(char *)((long)local_380 + 0x1d) != '\0')) &&
               ((local_344 != false ||
                ((local_25c != local_1e8.st_uid || (local_258 != local_1e8.st_gid)))))) {
              uVar19 = (ulong)local_344;
              *(stat **)(puVar28 + -8) = &local_1e8;
              *(ulong *)(puVar28 + -0x10) = uVar19;
              *(undefined8 *)(puVar28 + -0x18) = 0x10d4b4;
              iVar11 = FUN_0010820f(piVar14,pcVar22,iVar11,pcVar23,0xffffffff,&local_278);
              if (iVar11 == -1) {
                uVar10 = 0;
                goto LAB_0010d964;
              }
              if (iVar11 == 0) {
                local_338 = local_338 & 0xfffff1ff;
              }
            }
            pcVar22 = local_350;
            pcVar23 = local_358;
            piVar14 = local_380;
            if (*(char *)((long)local_380 + 0x35) != '\0') {
              *(undefined8 *)(puVar28 + -8) = 0x10d511;
              cVar9 = FUN_00107d7e(pcVar22,0xffffffff,pcVar23,0xffffffff,piVar14);
              if ((cVar9 != '\x01') && (*(char *)((long)local_380 + 0x36) != '\0')) {
                uVar10 = 0;
                goto LAB_0010d964;
              }
            }
            pcVar23 = local_358;
            uVar10 = local_341;
            if (local_33f == '\0') {
              *(undefined8 *)(puVar28 + -8) = 0x10d561;
              FUN_001084b0(pcVar23,0xffffffff,&local_278);
              uVar12 = local_338;
              pcVar22 = local_350;
              pcVar23 = local_358;
              if ((*(char *)((long)local_380 + 0x1e) == '\0') && ((char)local_380[6] == '\0')) {
                if (*(char *)((long)local_380 + 0x39) == '\0') {
                  if (((char)local_380[8] == '\0') || (local_344 == false)) {
                    if (local_330 != 0) {
                      *(undefined8 *)(puVar28 + -8) = 0x10d6a5;
                      uVar12 = FUN_0010dcaf();
                      iVar11 = local_35c;
                      pcVar23 = local_368;
                      local_330 = local_330 & ~uVar12;
                      if ((local_330 != 0) && (local_342 != '\x01')) {
                        if (local_344 != false) {
                          *(undefined8 *)(puVar28 + -8) = 0x10d6f7;
                          iVar11 = fstatat(iVar11,pcVar23,&local_1e8,0x100);
                          pcVar23 = local_358;
                          if (iVar11 != 0) {
                            *(undefined8 *)(puVar28 + -8) = 0x10d70f;
                            uVar15 = FUN_00118a0f(4,pcVar23);
                            *(undefined8 *)(puVar28 + -8) = 0x10d721;
                            uVar16 = gettext("cannot stat %s");
                            *(undefined8 *)(puVar28 + -8) = 0x10d729;
                            piVar14 = __errno_location();
                            iVar11 = *piVar14;
                            *(undefined8 *)(puVar28 + -8) = 0x10d742;
                            error(0,iVar11,uVar16,uVar15);
                            uVar10 = 0;
                            goto LAB_0010d964;
                          }
                        }
                        local_334 = local_1e8.st_mode;
                        if ((~local_1e8.st_mode & local_330) != 0) {
                          local_342 = '\x01';
                        }
                      }
                    }
                    iVar11 = local_35c;
                    pcVar23 = local_368;
                    uVar10 = local_341;
                    if (local_342 != '\0') {
                      uVar12 = local_334 | local_330;
                      *(undefined8 *)(puVar28 + -8) = 0x10d7a3;
                      iVar11 = FUN_00111260(iVar11,pcVar23,uVar12);
                      pcVar23 = local_358;
                      uVar10 = local_341;
                      if (iVar11 != 0) {
                        *(undefined8 *)(puVar28 + -8) = 0x10d7bb;
                        uVar15 = FUN_00118a0f(4,pcVar23);
                        *(undefined8 *)(puVar28 + -8) = 0x10d7cd;
                        uVar16 = gettext("preserving permissions for %s");
                        *(undefined8 *)(puVar28 + -8) = 0x10d7d5;
                        piVar14 = __errno_location();
                        iVar11 = *piVar14;
                        *(undefined8 *)(puVar28 + -8) = 0x10d7ee;
                        error(0,iVar11,uVar16,uVar15);
                        uVar10 = local_341;
                        if (*(char *)((long)local_380 + 0x32) != '\0') {
                          uVar10 = 0;
                        }
                      }
                    }
                  }
                  else {
                    if (((local_338 & 0xf000) == 0x4000) || ((local_338 & 0xf000) == 0xc000)) {
                      local_2fc = 0x1ff;
                    }
                    else {
                      local_2fc = 0x1b6;
                    }
                    *(undefined8 *)(puVar28 + -8) = 0x10d661;
                    uVar12 = FUN_0010dcaf();
                    pcVar23 = local_358;
                    uVar12 = ~uVar12 & local_2fc;
                    *(undefined8 *)(puVar28 + -8) = 0x10d681;
                    iVar11 = FUN_0010ed24(pcVar23,0xffffffff,uVar12);
                    uVar10 = local_341;
                    if (iVar11 != 0) {
                      uVar10 = 0;
                    }
                  }
                }
                else {
                  iVar11 = local_380[4];
                  *(undefined8 *)(puVar28 + -8) = 0x10d5fa;
                  iVar11 = FUN_0010ed24(pcVar23,0xffffffff,iVar11);
                  uVar10 = local_341;
                  if (iVar11 != 0) {
                    uVar10 = 0;
                  }
                }
              }
              else {
                *(undefined8 *)(puVar28 + -8) = 0x10d5a8;
                iVar11 = FUN_0010ec57(pcVar22,0xffffffff,pcVar23,0xffffffff,uVar12);
                uVar10 = local_341;
                if ((iVar11 != 0) && (*(char *)((long)local_380 + 0x32) != '\0')) {
                  uVar10 = 0;
                }
              }
            }
          }
          goto LAB_0010d964;
        }
        *(undefined8 *)(puVar28 + -8) = 0x10cbf3;
        uVar15 = FUN_00118941(1,4,pcVar23);
        pcVar23 = local_358;
        *(undefined8 *)(puVar28 + -8) = 0x10cc0f;
        uVar16 = FUN_00118941(0,4,pcVar23);
        *(undefined8 *)(puVar28 + -8) = 0x10cc21;
        uVar17 = gettext("cannot create symbolic link %s to %s");
        iVar11 = local_308;
        *(undefined8 *)(puVar28 + -8) = 0x10cc41;
        error(0,iVar11,uVar17,uVar16,uVar15);
      }
    }
    else if ((local_338 & 0xf000) == 0x4000) {
      *(undefined8 *)(puVar28 + -8) = 0x10bf37;
      cVar9 = FUN_00119370(0xffffff9c,pcVar21,iVar11,lVar1);
      lVar1 = local_2f8;
      iVar11 = local_35c;
      pcVar23 = local_368;
      if (cVar9 == '\0') {
        *(undefined8 *)(puVar28 + -8) = 0x10bfcf;
        cVar9 = FUN_00119370(iVar11,pcVar23,iVar11,lVar1);
        lVar1 = local_2f8;
        pcVar22 = local_358;
        pcVar23 = local_368;
        if (cVar9 != '\0') {
          *(undefined8 *)(puVar28 + -8) = 0x10bfe7;
          uVar15 = FUN_00118a0f(4,DAT_001271a0);
          *(undefined8 *)(puVar28 + -8) = 0x10bff9;
          uVar16 = gettext("warning: source directory %s specified more than once");
          *(undefined8 *)(puVar28 + -8) = 0x10c013;
          error(0,0,uVar16,uVar15);
          if (((char)local_380[6] != '\0') && (local_3a0 != (byte *)0x0)) {
            *local_3a0 = 1;
          }
          uVar10 = 1;
          goto LAB_0010d964;
        }
        if ((local_380[1] == 4) || ((local_384 != '\0' && (local_380[1] == 3)))) goto LAB_0010c172;
        *(undefined8 *)(puVar28 + -8) = 0x10c08f;
        pvVar20 = (void *)FUN_0010a7d7(pcVar22,pcVar23,lVar1);
        local_2a8 = pvVar20;
        *(undefined8 *)(puVar28 + -8) = 0x10c0af;
        uVar15 = FUN_00118941(1,4,pvVar20);
        pcVar23 = local_358;
        *(undefined8 *)(puVar28 + -8) = 0x10c0cb;
        uVar16 = FUN_00118941(0,4,pcVar23);
        *(undefined8 *)(puVar28 + -8) = 0x10c0dd;
        uVar17 = gettext("will not create hard link %s to directory %s");
        *(undefined8 *)(puVar28 + -8) = 0x10c0fa;
        error(0,0,uVar17,uVar16,uVar15);
        pvVar20 = local_2a8;
        *(undefined8 *)(puVar28 + -8) = 0x10c109;
        free(pvVar20);
      }
      else {
        *(undefined8 *)(puVar28 + -8) = 0x10bf54;
        uVar15 = FUN_00118941(1,4,DAT_001271a8);
        *(undefined8 *)(puVar28 + -8) = 0x10bf70;
        uVar16 = FUN_00118941(0,4,DAT_001271a0);
        *(undefined8 *)(puVar28 + -8) = 0x10bf82;
        uVar17 = gettext("cannot copy a directory, %s, into itself, %s");
        *(undefined8 *)(puVar28 + -8) = 0x10bf9f;
        error(0,0,uVar17,uVar16,uVar15);
        *local_398 = 1;
      }
    }
    else {
      bVar8 = *(byte *)(local_380 + 0xf);
      *(ulong *)(puVar28 + -0x10) = (ulong)local_33c;
      *(ulong *)(puVar28 + -0x18) = (ulong)bVar8;
      *(undefined8 *)(puVar28 + -0x20) = 1;
      *(undefined8 *)(puVar28 + -0x28) = 0x10c159;
      cVar9 = FUN_0010a85d(0,iVar11,lVar1,pcVar22,iVar11,pcVar23);
      if (cVar9 == '\x01') {
        uVar10 = 1;
        goto LAB_0010d964;
      }
    }
  }
LAB_0010d821:
  if (*(char *)((long)local_380 + 0x33) != '\0') {
    *(undefined8 *)(puVar27 + -8) = 0x10d835;
    FUN_0010a788();
  }
  if (local_2f8 == 0) {
    *(undefined8 *)(puVar27 + -8) = 0x10d858;
    FUN_0010dda4(local_270,local_278);
  }
  iVar11 = local_35c;
  pcVar23 = local_368;
  if (local_2f0 != (void *)0x0) {
    pcVar22 = (char *)((long)local_2f0 + ((long)local_368 - (long)local_358));
    local_280 = pcVar22;
    *(undefined8 *)(puVar27 + -8) = 0x10d8a9;
    iVar11 = renameat(iVar11,pcVar22,iVar11,pcVar23);
    pcVar23 = local_358;
    if (iVar11 == 0) {
      if ((char)local_380[0xf] != '\0') {
        *(undefined8 *)(puVar27 + -8) = 0x10d91e;
        uVar15 = FUN_00118941(1,4,pcVar23);
        pvVar20 = local_2f0;
        *(undefined8 *)(puVar27 + -8) = 0x10d93a;
        uVar16 = FUN_00118941(0,4,pvVar20);
        *(undefined8 *)(puVar27 + -8) = 0x10d94c;
        pcVar23 = (char *)gettext("%s -> %s (unbackup)\n");
        *(undefined8 *)(puVar27 + -8) = 0x10d95f;
        printf(pcVar23,uVar16,uVar15);
      }
    }
    else {
      *(undefined8 *)(puVar27 + -8) = 0x10d8c1;
      uVar15 = FUN_00118a0f(4,pcVar23);
      *(undefined8 *)(puVar27 + -8) = 0x10d8d3;
      uVar16 = gettext("cannot un-backup %s");
      *(undefined8 *)(puVar27 + -8) = 0x10d8db;
      piVar14 = __errno_location();
      iVar11 = *piVar14;
      *(undefined8 *)(puVar27 + -8) = 0x10d8f4;
      error(0,iVar11,uVar16,uVar15);
    }
  }
  puVar28 = puVar27;
  uVar10 = 0;
LAB_0010d964:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    *(undefined8 *)(puVar28 + -8) = 0x10d978;
    __stack_chk_fail();
  }
  return uVar10;
}




// Function: valid_options @ 0xd983

undefined8 valid_options(uint *param_1)

{
  if (3 < *param_1) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("VALID_BACKUP_TYPE (co->backup_type)","src/copy.c",0xc0f,"valid_options");
  }
  if (((param_1[3] != 1) && (param_1[3] != 2)) && (param_1[3] != 3)) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("VALID_SPARSE_MODE (co->sparse_mode)","src/copy.c",0xc10,"valid_options");
  }
  if (((param_1[0x11] != 0) && (param_1[0x11] != 1)) && (param_1[0x11] != 2)) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("VALID_REFLINK_MODE (co->reflink_mode)","src/copy.c",0xc11,"valid_options");
  }
  if ((*(char *)((long)param_1 + 0x17) == '\x01') && (*(char *)((long)param_1 + 0x3a) == '\x01')) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("!(co->hard_link && co->symbolic_link)","src/copy.c",0xc12,"valid_options");
  }
  if ((param_1[0x11] == 2) && (param_1[3] != 2)) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("! (co->reflink_mode == REFLINK_ALWAYS && co->sparse_mode != SPARSE_AUTO)",
                  "src/copy.c",0xc13,"valid_options");
  }
  return 1;
}




// Function: copy @ 0xdaea

void copy(undefined8 param_1,undefined8 param_2,undefined4 param_3,undefined8 param_4,
                 undefined4 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined1 local_11;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0010d983(param_6);
  if (cVar1 == '\0') {
                    /* WARNING: Subroutine does not return */
    __assert_fail("valid_options (options)","src/copy.c",0xc2a,"copy");
  }
  local_11 = 0;
  DAT_001271a0 = param_1;
  DAT_001271a8 = param_2;
  FUN_0010abda(param_1,param_2,param_3,param_4,param_5,0,0,param_6,1,&local_11,param_7,param_8);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: cp_options_default @ 0xdbcb

void cp_options_default(void *param_1)

{
  __uid_t _Var1;
  
  memset(param_1,0,0x58);
  _Var1 = geteuid();
  *(bool *)((long)param_1 + 0x1b) = _Var1 == 0;
  *(undefined1 *)((long)param_1 + 0x1a) = *(undefined1 *)((long)param_1 + 0x1b);
  *(undefined4 *)((long)param_1 + 0x40) = 0xffffffff;
  return;
}




// Function: chown_failure_ok @ 0xdc1f

undefined4 chown_failure_ok(long param_1)

{
  int *piVar1;
  
  piVar1 = __errno_location();
  if (((*piVar1 == 1) || (piVar1 = __errno_location(), *piVar1 == 0x16)) &&
     (*(char *)(param_1 + 0x1a) != '\x01')) {
    return 1;
  }
  return 0;
}




// Function: owner_failure_ok @ 0xdc67

undefined4 owner_failure_ok(long param_1)

{
  int *piVar1;
  
  piVar1 = __errno_location();
  if (((*piVar1 == 1) || (piVar1 = __errno_location(), *piVar1 == 0x16)) &&
     (*(char *)(param_1 + 0x1b) != '\x01')) {
    return 1;
  }
  return 0;
}




// Function: cached_umask @ 0xdcaf

__mode_t cached_umask(void)

{
  if (DAT_00127018 == 0xffffffff) {
    DAT_00127018 = umask(0);
    umask(DAT_00127018);
  }
  return DAT_00127018;
}




// Function: src_to_dest_hash @ 0xdce7

ulong src_to_dest_hash(ulong *param_1,ulong param_2)

{
  return *param_1 % param_2;
}




// Function: src_to_dest_compare @ 0xdd14

undefined4 src_to_dest_compare(long *param_1,long *param_2)

{
  undefined4 uVar1;
  
  if ((*param_1 == *param_2) && (param_1[1] == param_2[1])) {
    uVar1 = 1;
  }
  else {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: src_to_dest_free @ 0xdd6d

void src_to_dest_free(void *param_1)

{
  free(*(void **)((long)param_1 + 0x10));
  free(param_1);
  return;
}




// Function: forget_created @ 0xdda4

void forget_created(undefined8 param_1,undefined8 param_2)

{
  long lVar1;
  long in_FS_OFFSET;
  undefined8 local_28;
  undefined8 local_20;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_18 = 0;
  local_28 = param_1;
  local_20 = param_2;
  lVar1 = FUN_001163ee(DAT_001275e0,&local_28);
  if (lVar1 != 0) {
    FUN_0010dd6d(lVar1);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: src_to_dest_lookup @ 0xde23

undefined8 src_to_dest_lookup(undefined8 param_1,undefined8 param_2)

{
  long lVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined8 local_28;
  undefined8 local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_28 = param_1;
  local_20 = param_2;
  lVar1 = FUN_00114f88(DAT_001275e0,&local_28);
  if (lVar1 == 0) {
    uVar2 = 0;
  }
  else {
    uVar2 = *(undefined8 *)(lVar1 + 0x10);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: remember_copied @ 0xde9c

undefined8 remember_copied(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  
  puVar1 = (undefined8 *)FUN_0011b941(0x18);
  uVar2 = FUN_0011bf48(param_1);
  puVar1[2] = uVar2;
  *puVar1 = param_2;
  puVar1[1] = param_3;
  puVar3 = (undefined8 *)FUN_0011637e(DAT_001275e0,puVar1);
  if (puVar3 == (undefined8 *)0x0) {
    FUN_0011bf79();
  }
  if (puVar3 == puVar1) {
    uVar2 = 0;
  }
  else {
    FUN_0010dd6d(puVar1);
    uVar2 = puVar3[2];
  }
  return uVar2;
}




// Function: hash_init @ 0xdf3d

void hash_init(void)

{
  DAT_001275e0 = FUN_0011563d(0x67,0,FUN_0010dce7,FUN_0010dd14,FUN_0010dd6d);
  if (DAT_001275e0 == 0) {
    FUN_0011bf79();
  }
  return;
}




// Function: samedir_template @ 0xdf8a

void * samedir_template(void *param_1,void *param_2)

{
  long lVar1;
  ulong __size;
  char *__dest;
  void *local_20;
  
  lVar1 = FUN_00110369(param_1);
  __size = (lVar1 - (long)param_1) + 9;
  local_20 = param_2;
  if ((__size < 0x101) || (local_20 = malloc(__size), local_20 != (void *)0x0)) {
    __dest = mempcpy(local_20,param_1,lVar1 - (long)param_1);
    strcpy(__dest,"CuXXXXXX");
  }
  return local_20;
}




// Function: try_link @ 0xe021

void try_link(char *param_1,int *param_2)

{
  linkat(*param_2,*(char **)(param_2 + 2),param_2[4],param_1,param_2[5]);
  return;
}




// Function: force_linkat @ 0xe069

int force_linkat(int param_1,char *param_2,int param_3,char *param_4,int param_5,char param_6,
                int param_7)

{
  int iVar1;
  int *piVar2;
  char *__old;
  long in_FS_OFFSET;
  int local_144;
  int local_138 [2];
  char *local_130;
  int local_128;
  int local_124;
  char local_118 [264];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_7 < 0) {
    iVar1 = linkat(param_1,param_2,param_3,param_4,param_5);
    if (iVar1 == 0) {
      param_7 = 0;
    }
    else {
      piVar2 = __errno_location();
      param_7 = *piVar2;
    }
  }
  if ((param_6 == '\x01') && (param_7 == 0x11)) {
    __old = (char *)FUN_0010df8a(param_4,local_118);
    if (__old == (char *)0x0) {
      piVar2 = __errno_location();
      param_7 = *piVar2;
    }
    else {
      local_138[0] = param_1;
      local_130 = param_2;
      local_128 = param_3;
      local_124 = param_5;
      iVar1 = FUN_0011a191(__old,0,local_138,FUN_0010e021,6);
      if (iVar1 == 0) {
        iVar1 = renameat(param_3,__old,param_3,param_4);
        if (iVar1 == 0) {
          local_144 = -1;
        }
        else {
          piVar2 = __errno_location();
          local_144 = *piVar2;
        }
        unlinkat(param_3,__old,0);
      }
      else {
        piVar2 = __errno_location();
        local_144 = *piVar2;
      }
      param_7 = local_144;
      if (__old != local_118) {
        free(__old);
      }
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return param_7;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: try_symlink @ 0xe245

void try_symlink(char *param_1,undefined8 *param_2)

{
  symlinkat((char *)*param_2,*(int *)(param_2 + 1),param_1);
  return;
}




// Function: force_symlinkat @ 0xe27f

int force_symlinkat(char *param_1,int param_2,char *param_3,char param_4,int param_5)

{
  int iVar1;
  int *piVar2;
  char *__old;
  long in_FS_OFFSET;
  int local_154;
  int local_134;
  char *local_128;
  int local_120;
  char local_118 [264];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_154 = param_5;
  if (param_5 < 0) {
    iVar1 = symlinkat(param_1,param_2,param_3);
    if (iVar1 == 0) {
      local_154 = 0;
    }
    else {
      piVar2 = __errno_location();
      local_154 = *piVar2;
    }
  }
  if ((param_4 == '\x01') && (local_154 == 0x11)) {
    __old = (char *)FUN_0010df8a(param_3,local_118);
    if (__old == (char *)0x0) {
      piVar2 = __errno_location();
      local_154 = *piVar2;
    }
    else {
      local_128 = param_1;
      local_120 = param_2;
      iVar1 = FUN_0011a191(__old,0,&local_128,FUN_0010e245,6);
      if (iVar1 == 0) {
        iVar1 = renameat(param_2,__old,param_2,param_3);
        if (iVar1 == 0) {
          local_134 = -1;
        }
        else {
          piVar2 = __errno_location();
          local_134 = *piVar2;
          unlinkat(param_2,__old,0);
        }
      }
      else {
        piVar2 = __errno_location();
        local_134 = *piVar2;
      }
      local_154 = local_134;
      if (__old != local_118) {
        free(__old);
      }
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_154;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: computecon @ 0xe441

undefined4 computecon(undefined8 param_1,undefined4 param_2,undefined8 param_3)

{
  short sVar1;
  int iVar2;
  int *piVar3;
  long in_FS_OFFSET;
  undefined4 local_30;
  undefined8 local_28;
  undefined8 local_20;
  void *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_28 = 0;
  local_20 = 0;
  local_30 = 0xffffffff;
  local_18 = (void *)FUN_00111559(param_1);
  if ((((local_18 != (void *)0x0) && (iVar2 = getcon(&local_28), -1 < iVar2)) &&
      (iVar2 = FUN_00119d4e(local_18,&local_20), -1 < iVar2)) &&
     (sVar1 = mode_to_security_class(param_2), sVar1 != 0)) {
    local_30 = security_compute_create(local_28,local_20,sVar1,param_3);
  }
  piVar3 = __errno_location();
  iVar2 = *piVar3;
  free(local_18);
  freecon(local_28);
  freecon(local_20);
  piVar3 = __errno_location();
  *piVar3 = iVar2;
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_30;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: defaultcon @ 0xe549

undefined4 defaultcon(undefined8 param_1,char *param_2,undefined4 param_3)

{
  int iVar1;
  int *piVar2;
  long in_FS_OFFSET;
  char *local_68;
  undefined4 local_50;
  undefined8 local_48;
  undefined8 local_40;
  long local_38;
  long local_30;
  char *local_28;
  long local_20;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_50 = 0xffffffff;
  local_48 = 0;
  local_40 = 0;
  local_38 = 0;
  local_30 = 0;
  local_28 = (char *)0x0;
  local_68 = param_2;
  if ((*param_2 == '/') ||
     (local_68 = (char *)FUN_001111d3(param_2,2), local_28 = local_68, local_68 != (char *)0x0)) {
    iVar1 = selabel_lookup(param_1,&local_48,local_68,param_3);
    if (iVar1 < 0) {
      piVar2 = __errno_location();
      if (*piVar2 == 2) {
        piVar2 = __errno_location();
        *piVar2 = 0x3d;
      }
    }
    else {
      iVar1 = FUN_0010e441(local_68,param_3,&local_40);
      if ((((-1 < iVar1) && (local_38 = context_new(local_48), local_38 != 0)) &&
          (local_30 = context_new(local_40), local_30 != 0)) &&
         (((local_20 = context_type_get(local_38), local_20 != 0 &&
           (iVar1 = context_type_set(local_30,local_20), iVar1 == 0)) &&
          (local_18 = context_str(local_30), local_18 != 0)))) {
        local_50 = setfscreatecon(local_18);
      }
    }
  }
  piVar2 = __errno_location();
  iVar1 = *piVar2;
  context_free(local_38);
  context_free(local_30);
  freecon(local_48);
  freecon(local_40);
  free(local_28);
  piVar2 = __errno_location();
  *piVar2 = iVar1;
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_50;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: restorecon_private @ 0xe72c

undefined4 restorecon_private(long param_1,char *param_2)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  long in_FS_OFFSET;
  undefined4 local_e8;
  undefined8 local_d8;
  long local_d0;
  long local_c8;
  long local_c0;
  long local_b8;
  long local_b0;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_e8 = 0xffffffff;
  local_d8 = 0;
  local_d0 = 0;
  local_c8 = 0;
  local_c0 = 0;
  if (param_1 == 0) {
    iVar1 = getfscreatecon(&local_d0);
    if (iVar1 < 0) {
      local_e8 = 0xffffffff;
    }
    else if (local_d0 == 0) {
      piVar3 = __errno_location();
      *piVar3 = 0x3d;
      local_e8 = 0xffffffff;
    }
    else {
      local_e8 = lsetfilecon(param_2,local_d0);
      piVar3 = __errno_location();
      iVar1 = *piVar3;
      freecon(local_d0);
      piVar3 = __errno_location();
      *piVar3 = iVar1;
    }
  }
  else {
    iVar1 = open(param_2,0x20000);
    if ((iVar1 != -1) || (piVar3 = __errno_location(), *piVar3 == 0x28)) {
      if (iVar1 == -1) {
        iVar2 = lstat(param_2,&local_a8);
      }
      else {
        iVar2 = fstat(iVar1,&local_a8);
      }
      if (-1 < iVar2) {
        iVar2 = selabel_lookup(param_1,&local_d8,param_2,local_a8.st_mode);
        if (iVar2 < 0) {
          piVar3 = __errno_location();
          if (*piVar3 == 2) {
            piVar3 = __errno_location();
            *piVar3 = 0x3d;
          }
        }
        else {
          local_c8 = context_new(local_d8);
          if (local_c8 != 0) {
            if (iVar1 == -1) {
              iVar2 = FUN_00119d8b(param_2,&local_d0);
            }
            else {
              iVar2 = FUN_00119dc8(iVar1,&local_d0);
            }
            if ((((-1 < iVar2) && (local_c0 = context_new(local_d0), local_c0 != 0)) &&
                (local_b8 = context_type_get(local_c8), local_b8 != 0)) &&
               ((iVar2 = context_type_set(local_c0,local_b8), iVar2 == 0 &&
                (local_b0 = context_str(local_c0), local_b0 != 0)))) {
              if (iVar1 == -1) {
                local_e8 = lsetfilecon(param_2,local_b0);
              }
              else {
                local_e8 = fsetfilecon(iVar1,local_b0);
              }
            }
          }
        }
      }
    }
    piVar3 = __errno_location();
    iVar2 = *piVar3;
    if (iVar1 != -1) {
      close(iVar1);
    }
    context_free(local_c8);
    context_free(local_c0);
    freecon(local_d8);
    freecon(local_d0);
    piVar3 = __errno_location();
    *piVar3 = iVar2;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_e8;
}




// Function: restorecon @ 0xead7

bool restorecon(undefined8 param_1,char *param_2,char param_3)

{
  int iVar1;
  int *piVar2;
  long lVar3;
  long lVar4;
  long in_FS_OFFSET;
  bool bVar5;
  char *local_68;
  int local_48;
  char *local_40;
  char *local_28;
  undefined8 local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_40 = (char *)0x0;
  local_68 = param_2;
  if ((*param_2 == '/') ||
     (local_68 = (char *)FUN_001111d3(param_2,2), local_40 = local_68, local_68 != (char *)0x0)) {
    if (param_3 == '\x01') {
      local_28 = local_68;
      local_20 = 0;
      lVar3 = FUN_0011bfb9(&local_28,0x10,0);
      local_48 = 0;
      while (lVar4 = FUN_00112fac(lVar3), lVar4 != 0) {
        iVar1 = FUN_0010e72c(param_1,*(undefined8 *)(lVar3 + 0x20));
        if (iVar1 < 0) {
          piVar2 = __errno_location();
          local_48 = *piVar2;
        }
      }
      piVar2 = __errno_location();
      if (*piVar2 != 0) {
        piVar2 = __errno_location();
        local_48 = *piVar2;
      }
      iVar1 = FUN_00112af2(lVar3);
      if (iVar1 != 0) {
        piVar2 = __errno_location();
        local_48 = *piVar2;
      }
      free(local_40);
      bVar5 = local_48 == 0;
    }
    else {
      iVar1 = FUN_0010e72c(param_1,local_68);
      bVar5 = iVar1 != -1;
      piVar2 = __errno_location();
      iVar1 = *piVar2;
      free(local_40);
      piVar2 = __errno_location();
      *piVar2 = iVar1;
    }
  }
  else {
    bVar5 = false;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: dir_name @ 0x11559

long dir_name(undefined8 param_1)

{
  long lVar1;
  
  lVar1 = FUN_00111610(param_1);
  if (lVar1 == 0) {
    FUN_0011bf79();
  }
  return lVar1;
}




// Function: hash_get_n_buckets @ 0x14c84

undefined8 hash_get_n_buckets(long param_1)

{
  return *(undefined8 *)(param_1 + 0x10);
}




// Function: hash_get_n_buckets_used @ 0x14c9a

undefined8 hash_get_n_buckets_used(long param_1)

{
  return *(undefined8 *)(param_1 + 0x18);
}




// Function: hash_get_n_entries @ 0x14cb0

undefined8 hash_get_n_entries(long param_1)

{
  return *(undefined8 *)(param_1 + 0x20);
}




// Function: hash_get_max_bucket_length @ 0x14cc6

ulong hash_get_max_bucket_length(undefined8 *param_1)

{
  long *local_28;
  ulong local_20;
  long *local_18;
  ulong local_10;
  
  local_20 = 0;
  for (local_28 = (long *)*param_1; local_28 < (long *)param_1[1]; local_28 = local_28 + 2) {
    if (*local_28 != 0) {
      local_18 = local_28;
      local_10 = 1;
      while (local_18 = (long *)local_18[1], local_18 != (long *)0x0) {
        local_10 = local_10 + 1;
      }
      if (local_20 < local_10) {
        local_20 = local_10;
      }
    }
  }
  return local_20;
}




// Function: hash_table_ok @ 0x14d48

undefined8 hash_table_ok(undefined8 *param_1)

{
  undefined8 uVar1;
  long *local_28;
  long local_20;
  long local_18;
  long *local_10;
  
  local_20 = 0;
  local_18 = 0;
  for (local_28 = (long *)*param_1; local_28 < (long *)param_1[1]; local_28 = local_28 + 2) {
    if (*local_28 != 0) {
      local_10 = local_28;
      local_20 = local_20 + 1;
      do {
        local_18 = local_18 + 1;
        local_10 = (long *)local_10[1];
      } while (local_10 != (long *)0x0);
    }
  }
  if ((local_20 == param_1[3]) && (local_18 == param_1[4])) {
    uVar1 = 1;
  }
  else {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: hash_print_statistics @ 0x14de6

void hash_print_statistics(undefined8 param_1,FILE *param_2)

{
  undefined8 uVar1;
  ulong uVar2;
  ulong uVar3;
  undefined8 uVar4;
  
  uVar1 = FUN_00114cb0(param_1);
  uVar2 = FUN_00114c84(param_1);
  uVar3 = FUN_00114c9a(param_1);
  uVar4 = FUN_00114cc6(param_1);
  fprintf(param_2,"# entries:         %lu\n",uVar1);
  fprintf(param_2,"# buckets:         %lu\n",uVar2);
  fprintf(param_2,"# buckets used:    %lu (%.2f%%)\n",(DAT_00120808 * (double)uVar3) / (double)uVar2
          ,uVar3);
  fprintf(param_2,"max bucket length: %lu\n",uVar4);
  return;
}




// Function: safe_hasher @ 0x14f2d

long safe_hasher(long *param_1,undefined8 param_2)

{
  ulong uVar1;
  
  uVar1 = (*(code *)param_1[6])(param_2,param_1[2]);
  if ((ulong)param_1[2] <= uVar1) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  return uVar1 * 0x10 + *param_1;
}




// Function: hash_lookup @ 0x14f88

long hash_lookup(long param_1,long param_2)

{
  char cVar1;
  long *local_18;
  
  local_18 = (long *)FUN_00114f2d(param_1,param_2);
  if (*local_18 != 0) {
    for (; local_18 != (long *)0x0; local_18 = (long *)local_18[1]) {
      if ((param_2 == *local_18) ||
         (cVar1 = (**(code **)(param_1 + 0x38))(param_2,*local_18), cVar1 != '\0')) {
        return *local_18;
      }
    }
  }
  return 0;
}




// Function: hash_get_first @ 0x1501f

long hash_get_first(undefined8 *param_1)

{
  long lVar1;
  long *local_10;
  
  if (param_1[4] == 0) {
    lVar1 = 0;
  }
  else {
    local_10 = (long *)*param_1;
    while( true ) {
      if ((long *)param_1[1] <= local_10) {
                    /* WARNING: Subroutine does not return */
        abort();
      }
      if (*local_10 != 0) break;
      local_10 = local_10 + 2;
    }
    lVar1 = *local_10;
  }
  return lVar1;
}




// Function: hash_get_next @ 0x1507f

long hash_get_next(long param_1,long param_2)

{
  long *local_18;
  long *local_10;
  
  local_18 = (long *)FUN_00114f2d(param_1,param_2);
  local_10 = local_18;
  while ((param_2 != *local_10 || (local_10[1] == 0))) {
    local_10 = (long *)local_10[1];
    if (local_10 == (long *)0x0) {
      do {
        local_18 = local_18 + 2;
        if (*(long **)(param_1 + 8) <= local_18) {
          return 0;
        }
      } while (*local_18 == 0);
      return *local_18;
    }
  }
  return *(long *)local_10[1];
}




// Function: hash_get_entries @ 0x1511d

ulong hash_get_entries(undefined8 *param_1,long param_2,ulong param_3)

{
  ulong local_20;
  long *local_18;
  long *local_10;
  
  local_20 = 0;
  local_18 = (long *)*param_1;
  do {
    if ((long *)param_1[1] <= local_18) {
      return local_20;
    }
    if (*local_18 != 0) {
      for (local_10 = local_18; local_10 != (long *)0x0; local_10 = (long *)local_10[1]) {
        if (param_3 <= local_20) {
          return local_20;
        }
        *(long *)(local_20 * 8 + param_2) = *local_10;
        local_20 = local_20 + 1;
      }
    }
    local_18 = local_18 + 2;
  } while( true );
}




// Function: hash_do_for_each @ 0x151bd

long hash_do_for_each(undefined8 *param_1,code *param_2,undefined8 param_3)

{
  char cVar1;
  long local_20;
  long *local_18;
  long *local_10;
  
  local_20 = 0;
  local_18 = (long *)*param_1;
  do {
    if ((long *)param_1[1] <= local_18) {
      return local_20;
    }
    if (*local_18 != 0) {
      for (local_10 = local_18; local_10 != (long *)0x0; local_10 = (long *)local_10[1]) {
        cVar1 = (*param_2)(*local_10,param_3);
        if (cVar1 != '\x01') {
          return local_20;
        }
        local_20 = local_20 + 1;
      }
    }
    local_18 = local_18 + 2;
  } while( true );
}




// Function: hash_string @ 0x15255

ulong hash_string(byte *param_1,ulong param_2)

{
  byte *local_20;
  ulong local_10;
  
  local_10 = 0;
  for (local_20 = param_1; *local_20 != 0; local_20 = local_20 + 1) {
    local_10 = ((ulong)*local_20 + local_10 * 0x1f) % param_2;
  }
  return local_10;
}




// Function: is_prime @ 0x152af

undefined8 is_prime(ulong param_1)

{
  undefined8 local_18;
  undefined8 local_10;
  
  local_10 = 9;
  for (local_18 = 3; (local_10 < param_1 && (param_1 % local_18 != 0)); local_18 = local_18 + 2) {
    local_10 = local_10 + (local_18 + 1) * 4;
  }
  return CONCAT71((int7)(param_1 % local_18 >> 8),param_1 % local_18 != 0);
}




// Function: next_prime @ 0x1531e

ulong next_prime(ulong param_1)

{
  char cVar1;
  ulong local_10;
  
  local_10 = param_1;
  if (param_1 < 10) {
    local_10 = 10;
  }
  local_10 = local_10 | 1;
  while( true ) {
    if (local_10 == 0xffffffffffffffff) {
      return 0xffffffffffffffff;
    }
    cVar1 = FUN_001152af(local_10);
    if (cVar1 == '\x01') break;
    local_10 = local_10 + 2;
  }
  return local_10;
}




// Function: hash_reset_tuning @ 0x15369

void hash_reset_tuning(undefined4 *param_1)

{
  *param_1 = 0;
  param_1[1] = DAT_00120810;
  param_1[2] = DAT_00120814;
  param_1[3] = DAT_00120818;
  *(undefined1 *)(param_1 + 4) = 0;
  return;
}




// Function: raw_hasher @ 0x153bf

ulong raw_hasher(undefined8 param_1,ulong param_2)

{
  ulong uVar1;
  
  uVar1 = FUN_0011c5bc(param_1,3);
  return uVar1 % param_2;
}




// Function: raw_comparator @ 0x153fa

bool raw_comparator(long param_1,long param_2)

{
  return param_1 == param_2;
}




// Function: check_tuning @ 0x15417

undefined8 check_tuning(long param_1)

{
  float *pfVar1;
  undefined8 uVar2;
  
  pfVar1 = *(float **)(param_1 + 0x28);
  if (pfVar1 == (float *)&DAT_00120780) {
    uVar2 = 1;
  }
  else if ((((pfVar1[2] <= DAT_0012081c) || (DAT_00120810 - DAT_0012081c <= pfVar1[2])) ||
           (pfVar1[3] <= DAT_0012081c + DAT_00120810)) ||
          (((*pfVar1 < 0.0 || (pfVar1[1] <= *pfVar1 + DAT_0012081c)) ||
           ((DAT_00120810 < pfVar1[1] || (pfVar1[2] <= *pfVar1 + DAT_0012081c)))))) {
    *(undefined **)(param_1 + 0x28) = &DAT_00120780;
    uVar2 = 0;
  }
  else {
    uVar2 = 1;
  }
  return uVar2;
}




// Function: compute_bucket_size @ 0x15524

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulong compute_bucket_size(ulong param_1,long param_2)

{
  ulong uVar1;
  float fVar2;
  ulong local_20;
  
  local_20 = param_1;
  if (*(char *)(param_2 + 0x10) != '\x01') {
    fVar2 = (float)param_1 / *(float *)(param_2 + 8);
    if (_DAT_00120820 <= fVar2) {
      return 0;
    }
    if (_DAT_00120824 <= fVar2) {
      local_20 = (long)(fVar2 - _DAT_00120824) ^ 0x8000000000000000;
    }
    else {
      local_20 = (ulong)fVar2;
    }
  }
  uVar1 = FUN_0011531e(local_20);
  if ((long)(uVar1 << 3) < 0 || uVar1 >> 0x3d != 0) {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: hash_initialize @ 0x1563d

long * hash_initialize(undefined8 param_1,undefined *param_2,code *param_3,code *param_4,long param_5)

{
  char cVar1;
  long *__ptr;
  long lVar2;
  void *pvVar3;
  code *local_38;
  code *local_30;
  undefined *local_28;
  
  local_30 = param_3;
  if (param_3 == (code *)0x0) {
    local_30 = FUN_001153bf;
  }
  local_38 = param_4;
  if (param_4 == (code *)0x0) {
    local_38 = FUN_001153fa;
  }
  __ptr = malloc(0x50);
  if (__ptr != (long *)0x0) {
    local_28 = param_2;
    if (param_2 == (undefined *)0x0) {
      local_28 = &DAT_00120780;
    }
    __ptr[5] = (long)local_28;
    cVar1 = FUN_00115417(__ptr);
    if (cVar1 == '\x01') {
      lVar2 = FUN_00115524(param_1,local_28);
      __ptr[2] = lVar2;
      if (__ptr[2] != 0) {
        pvVar3 = calloc(__ptr[2],0x10);
        *__ptr = (long)pvVar3;
        if (*__ptr != 0) {
          __ptr[1] = *__ptr + __ptr[2] * 0x10;
          __ptr[3] = 0;
          __ptr[4] = 0;
          __ptr[6] = (long)local_30;
          __ptr[7] = (long)local_38;
          __ptr[8] = param_5;
          __ptr[9] = 0;
          return __ptr;
        }
      }
    }
    free(__ptr);
  }
  return (long *)0x0;
}




// Function: hash_clear @ 0x157b2

void hash_clear(undefined8 *param_1)

{
  undefined8 *puVar1;
  long *local_20;
  undefined8 *local_18;
  
  for (local_20 = (long *)*param_1; local_20 < (long *)param_1[1]; local_20 = local_20 + 2) {
    if (*local_20 != 0) {
      local_18 = (undefined8 *)local_20[1];
      while (local_18 != (undefined8 *)0x0) {
        if (param_1[8] != 0) {
          (*(code *)param_1[8])(*local_18);
        }
        *local_18 = 0;
        puVar1 = (undefined8 *)local_18[1];
        local_18[1] = param_1[9];
        param_1[9] = local_18;
        local_18 = puVar1;
      }
      if (param_1[8] != 0) {
        (*(code *)param_1[8])(*local_20);
      }
      *local_20 = 0;
      local_20[1] = 0;
    }
  }
  param_1[3] = 0;
  param_1[4] = 0;
  return;
}




// Function: hash_free @ 0x158bd

void hash_free(ulong *param_1)

{
  void *pvVar1;
  long *local_20;
  long *local_18;
  
  if ((param_1[8] != 0) && (param_1[4] != 0)) {
    for (local_20 = (long *)*param_1; local_20 < (long *)param_1[1]; local_20 = local_20 + 2) {
      if (*local_20 != 0) {
        for (local_18 = local_20; local_18 != (long *)0x0; local_18 = (long *)local_18[1]) {
          (*(code *)param_1[8])(*local_18);
        }
      }
    }
  }
  for (local_20 = (long *)*param_1; local_20 < param_1[1];
      local_20 = (long *)((long)local_20 + 0x10)) {
    local_18 = *(void **)((long)local_20 + 8);
    while (local_18 != (void *)0x0) {
      pvVar1 = *(void **)((long)local_18 + 8);
      free(local_18);
      local_18 = pvVar1;
    }
  }
  local_18 = (void *)param_1[9];
  while (local_18 != (void *)0x0) {
    pvVar1 = *(void **)((long)local_18 + 8);
    free(local_18);
    local_18 = pvVar1;
  }
  free((void *)*param_1);
  free(param_1);
  return;
}




// Function: allocate_entry @ 0x159ec

void * allocate_entry(long param_1)

{
  undefined8 local_10;
  
  if (*(long *)(param_1 + 0x48) == 0) {
    local_10 = malloc(0x10);
  }
  else {
    local_10 = *(void **)(param_1 + 0x48);
    *(undefined8 *)(param_1 + 0x48) = *(undefined8 *)((long)local_10 + 8);
  }
  return local_10;
}




// Function: free_entry @ 0x15a3b

void free_entry(long param_1,undefined8 *param_2)

{
  *param_2 = 0;
  param_2[1] = *(undefined8 *)(param_1 + 0x48);
  *(undefined8 **)(param_1 + 0x48) = param_2;
  return;
}




// Function: hash_find_entry @ 0x15a75

long hash_find_entry(long param_1,long param_2,undefined8 *param_3,char param_4)

{
  long *plVar1;
  long lVar2;
  char cVar3;
  long lVar4;
  long *local_38;
  
  local_38 = (long *)FUN_00114f2d(param_1,param_2);
  *param_3 = local_38;
  if (*local_38 == 0) {
    lVar4 = 0;
  }
  else if ((param_2 == *local_38) ||
          (cVar3 = (**(code **)(param_1 + 0x38))(param_2,*local_38), cVar3 != '\0')) {
    lVar4 = *local_38;
    if (param_4 != '\0') {
      if (local_38[1] == 0) {
        *local_38 = 0;
      }
      else {
        plVar1 = (long *)local_38[1];
        lVar2 = plVar1[1];
        *local_38 = *plVar1;
        local_38[1] = lVar2;
        FUN_00115a3b(param_1,plVar1);
      }
    }
  }
  else {
    for (; local_38[1] != 0; local_38 = (long *)local_38[1]) {
      if ((param_2 == *(long *)local_38[1]) ||
         (cVar3 = (**(code **)(param_1 + 0x38))(param_2,*(undefined8 *)local_38[1]), cVar3 != '\0'))
      {
        lVar4 = *(long *)local_38[1];
        if (param_4 == '\0') {
          return lVar4;
        }
        lVar2 = local_38[1];
        local_38[1] = *(long *)(lVar2 + 8);
        FUN_00115a3b(param_1,lVar2);
        return lVar4;
      }
    }
    lVar4 = 0;
  }
  return lVar4;
}




// Function: transfer_entries @ 0x15c0e

undefined8 transfer_entries(long param_1,undefined8 *param_2,char param_3)

{
  long lVar1;
  long *plVar2;
  long *plVar3;
  long *local_38;
  long *local_30;
  
  local_38 = (long *)*param_2;
  do {
    if ((long *)param_2[1] <= local_38) {
      return 1;
    }
    if (*local_38 != 0) {
      local_30 = (long *)local_38[1];
      while (local_30 != (long *)0x0) {
        lVar1 = *local_30;
        plVar3 = (long *)FUN_00114f2d(param_1,lVar1);
        plVar2 = (long *)local_30[1];
        if (*plVar3 == 0) {
          *plVar3 = lVar1;
          *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
          FUN_00115a3b(param_1,local_30);
          local_30 = plVar2;
        }
        else {
          local_30[1] = plVar3[1];
          plVar3[1] = (long)local_30;
          local_30 = plVar2;
        }
      }
      lVar1 = *local_38;
      local_38[1] = 0;
      if (param_3 == '\0') {
        plVar2 = (long *)FUN_00114f2d(param_1,lVar1);
        if (*plVar2 == 0) {
          *plVar2 = lVar1;
          *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
        }
        else {
          plVar3 = (long *)FUN_001159ec(param_1);
          if (plVar3 == (long *)0x0) {
            return 0;
          }
          *plVar3 = lVar1;
          plVar3[1] = plVar2[1];
          plVar2[1] = (long)plVar3;
        }
        *local_38 = 0;
        param_2[3] = param_2[3] + -1;
      }
    }
    local_38 = local_38 + 2;
  } while( true );
}




// Function: hash_rehash @ 0x15de2

undefined8 hash_rehash(undefined8 *param_1,undefined8 param_2)

{
  char cVar1;
  size_t __nmemb;
  undefined8 uVar2;
  long in_FS_OFFSET;
  void *local_68;
  void *local_60;
  size_t local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined8 local_40;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 local_28;
  undefined8 local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  __nmemb = FUN_00115524(param_2,param_1[5]);
  if (__nmemb == 0) {
    uVar2 = 0;
  }
  else if (__nmemb == param_1[2]) {
    uVar2 = 1;
  }
  else {
    local_68 = calloc(__nmemb,0x10);
    if (local_68 == (void *)0x0) {
      uVar2 = 0;
    }
    else {
      local_60 = (void *)((long)local_68 + __nmemb * 0x10);
      local_50 = 0;
      local_48 = 0;
      local_40 = param_1[5];
      local_38 = param_1[6];
      local_30 = param_1[7];
      local_28 = param_1[8];
      local_20 = param_1[9];
      local_58 = __nmemb;
      cVar1 = FUN_00115c0e(&local_68,param_1,0);
      if (cVar1 == '\0') {
        param_1[9] = local_20;
        cVar1 = FUN_00115c0e(param_1,&local_68,1);
        if (cVar1 == '\x01') {
          cVar1 = FUN_00115c0e(param_1,&local_68,0);
          if (cVar1 == '\x01') {
            free(local_68);
            uVar2 = 0;
            goto LAB_00115ff7;
          }
        }
                    /* WARNING: Subroutine does not return */
        abort();
      }
      free((void *)*param_1);
      *param_1 = local_68;
      param_1[1] = local_60;
      param_1[2] = local_58;
      param_1[3] = local_50;
      param_1[9] = local_20;
      uVar2 = 1;
    }
  }
LAB_00115ff7:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: hash_insert_if_absent @ 0x1600d

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 hash_insert_if_absent(long param_1,long param_2,long *param_3)

{
  char cVar1;
  undefined8 uVar2;
  ulong uVar3;
  long lVar4;
  long in_FS_OFFSET;
  float fVar5;
  long *local_30;
  long local_28;
  long local_20;
  long *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 == 0) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  local_28 = FUN_00115a75(param_1,param_2,&local_30,0);
  if (local_28 != 0) {
    if (param_3 != (long *)0x0) {
      *param_3 = local_28;
    }
    uVar2 = 0;
    goto LAB_00116368;
  }
  if (((float)*(ulong *)(param_1 + 0x10) * *(float *)(*(long *)(param_1 + 0x28) + 8) <
       (float)*(ulong *)(param_1 + 0x18)) &&
     (FUN_00115417(param_1),
     (float)*(ulong *)(param_1 + 0x10) * *(float *)(*(long *)(param_1 + 0x28) + 8) <
     (float)*(ulong *)(param_1 + 0x18))) {
    local_20 = *(long *)(param_1 + 0x28);
    if (*(char *)(local_20 + 0x10) == '\0') {
      fVar5 = *(float *)(local_20 + 8) *
              *(float *)(local_20 + 0xc) * (float)*(ulong *)(param_1 + 0x10);
    }
    else {
      fVar5 = (float)*(ulong *)(param_1 + 0x10) * *(float *)(local_20 + 0xc);
    }
    if (_DAT_00120820 <= fVar5) {
      uVar2 = 0xffffffff;
      goto LAB_00116368;
    }
    if (_DAT_00120824 <= fVar5) {
      uVar3 = (long)(fVar5 - _DAT_00120824) ^ 0x8000000000000000;
    }
    else {
      uVar3 = (ulong)fVar5;
    }
    cVar1 = FUN_00115de2(param_1,uVar3);
    if (cVar1 != '\x01') {
      uVar2 = 0xffffffff;
      goto LAB_00116368;
    }
    lVar4 = FUN_00115a75(param_1,param_2,&local_30,0);
    if (lVar4 != 0) {
                    /* WARNING: Subroutine does not return */
      abort();
    }
  }
  if (*local_30 == 0) {
    *local_30 = param_2;
    *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
    *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
    uVar2 = 1;
  }
  else {
    local_18 = (long *)FUN_001159ec(param_1);
    if (local_18 == (long *)0x0) {
      uVar2 = 0xffffffff;
    }
    else {
      *local_18 = param_2;
      local_18[1] = local_30[1];
      local_30[1] = (long)local_18;
      *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
      uVar2 = 1;
    }
  }
LAB_00116368:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: hash_insert @ 0x1637e

undefined8 hash_insert(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_0011600d(param_1,param_2,&local_18);
  if (iVar1 == -1) {
    param_2 = 0;
  }
  else if (iVar1 == 0) {
    param_2 = local_18;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return param_2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_remove @ 0x163ee

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

long hash_remove(long param_1,undefined8 param_2)

{
  char cVar1;
  long lVar2;
  long in_FS_OFFSET;
  float fVar3;
  long *local_40;
  void *local_38;
  long local_30;
  long local_28;
  ulong local_20;
  void *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_30 = FUN_00115a75(param_1,param_2,&local_40,1);
  if (local_30 == 0) {
    lVar2 = 0;
  }
  else {
    *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + -1;
    lVar2 = local_30;
    if (((*local_40 == 0) &&
        (*(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + -1,
        (float)*(ulong *)(param_1 + 0x18) <
        (float)*(ulong *)(param_1 + 0x10) * **(float **)(param_1 + 0x28))) &&
       (FUN_00115417(param_1), lVar2 = local_30,
       (float)*(ulong *)(param_1 + 0x18) <
       (float)*(ulong *)(param_1 + 0x10) * **(float **)(param_1 + 0x28))) {
      local_28 = *(long *)(param_1 + 0x28);
      if (*(char *)(local_28 + 0x10) == '\0') {
        fVar3 = *(float *)(local_28 + 8) *
                *(float *)(local_28 + 4) * (float)*(ulong *)(param_1 + 0x10);
        if (_DAT_00120824 <= fVar3) {
          local_20 = (long)(fVar3 - _DAT_00120824) ^ 0x8000000000000000;
        }
        else {
          local_20 = (ulong)fVar3;
        }
      }
      else {
        fVar3 = (float)*(ulong *)(param_1 + 0x10) * *(float *)(local_28 + 4);
        if (_DAT_00120824 <= fVar3) {
          local_20 = (long)(fVar3 - _DAT_00120824) ^ 0x8000000000000000;
        }
        else {
          local_20 = (ulong)fVar3;
        }
      }
      cVar1 = FUN_00115de2(param_1,local_20);
      lVar2 = local_30;
      if (cVar1 != '\x01') {
        local_38 = *(void **)(param_1 + 0x48);
        while (local_38 != (void *)0x0) {
          local_18 = *(void **)((long)local_38 + 8);
          free(local_38);
          local_38 = local_18;
        }
        *(undefined8 *)(param_1 + 0x48) = 0;
        lVar2 = local_30;
      }
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar2;
}




// Function: hash_delete @ 0x166f4

void hash_delete(undefined8 param_1,undefined8 param_2)

{
  FUN_001163ee(param_1,param_2);
  return;
}




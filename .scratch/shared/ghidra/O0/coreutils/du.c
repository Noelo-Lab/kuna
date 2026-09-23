// Function: duinfo_init @ 0x40da

void duinfo_init(undefined8 *param_1)

{
  *param_1 = 0;
  param_1[1] = 0;
  param_1[2] = 0x8000000000000000;
  param_1[3] = 0xffffffffffffffff;
  return;
}




// Function: duinfo_set @ 0x411a

void duinfo_set(undefined8 *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  *param_1 = param_2;
  param_1[1] = 1;
  param_1[2] = param_3;
  param_1[3] = param_4;
  return;
}




// Function: duinfo_add @ 0x415c

void duinfo_add(ulong *param_1,long *param_2)

{
  int iVar1;
  ulong uVar2;
  
  uVar2 = *param_2 + *param_1;
  if (uVar2 < *param_1) {
    uVar2 = 0xffffffffffffffff;
  }
  *param_1 = uVar2;
  param_1[1] = param_1[1] + param_2[1];
  iVar1 = FUN_0011489d(param_1[2],param_1[3],param_2[2],param_2[3]);
  if (iVar1 < 0) {
    uVar2 = param_2[3];
    param_1[2] = param_2[2];
    param_1[3] = uVar2;
  }
  return;
}




// Function: usage @ 0x41ff

void usage(int param_1)

{
  FILE *pFVar1;
  undefined8 uVar2;
  char *pcVar3;
  
  uVar2 = DAT_001211d0;
  if (param_1 == 0) {
    pcVar3 = (char *)gettext(
                            "Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n"
                            );
    printf(pcVar3,uVar2,uVar2);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "Summarize device usage of the set of FILEs, recursively for directories.\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    FUN_00103d79();
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -0, --null            end each output line with NUL, not newline\n  -a, --all             write counts for all files, not just directories\n      --apparent-size   print apparent sizes rather than device usage; although\n                          the apparent size is usually smaller, it may be\n                          larger due to holes in (\'sparse\') files, internal\n                          fragmentation, indirect blocks, and the like\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -B, --block-size=SIZE  scale sizes by SIZE before printing them; e.g.,\n                           \'-BM\' prints sizes in units of 1,048,576 bytes;\n                           see SIZE format below\n  -b, --bytes           equivalent to \'--apparent-size --block-size=1\'\n  -c, --total           produce a grand total\n  -D, --dereference-args  dereference only symlinks that are listed on the\n                          command line\n  -d, --max-depth=N     print the total for a directory (or file, with --all)\n                          only if it is N or fewer levels below the command\n                          line argument;  --max-depth=0 is the same as\n                          --summarize\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "      --files0-from=F   summarize device usage of the\n                          NUL-terminated file names specified in file F;\n                          if F is -, then read names from standard input\n  -H                    equivalent to --dereference-args (-D)\n  -h, --human-readable  print sizes in human readable format (e.g., 1K 234M 2G)\n      --inodes          list inode usage information instead of block usage\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -k                    like --block-size=1K\n  -L, --dereference     dereference all symbolic links\n  -l, --count-links     count sizes many times if hard linked\n  -m                    like --block-size=1M\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -P, --no-dereference  don\'t follow any symbolic links (this is the default)\n  -S, --separate-dirs   for directories do not include size of subdirectories\n      --si              like -h, but use powers of 1000 not 1024\n  -s, --summarize       display only a total for each argument\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -t, --threshold=SIZE  exclude entries smaller than SIZE if positive,\n                          or entries greater than SIZE if negative\n      --time            show time of the last modification of any file in the\n                          directory, or any of its subdirectories\n      --time=WORD       show time as WORD instead of modification time:\n                          atime, access, use, ctime or status\n      --time-style=STYLE  show times using STYLE, which can be:\n                            full-iso, long-iso, iso, or +FORMAT;\n                            FORMAT is interpreted like in \'date\'\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -X, --exclude-from=FILE  exclude files that match any pattern in FILE\n      --exclude=PATTERN    exclude files that match PATTERN\n  -x, --one-file-system    skip directories on different file systems\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("      --help        display this help and exit\n");
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("      --version     output version information and exit\n");
    fputs_unlocked(pcVar3,pFVar1);
    FUN_00103ddb(&DAT_0011a151);
    FUN_00103daa();
    FUN_00103e10(&DAT_0011a154);
  }
  else {
    pcVar3 = (char *)gettext("Try \'%s --help\' for more information.\n");
    fprintf(stderr,pcVar3,uVar2);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: hash_ins @ 0x43fb

bool hash_ins(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  int iVar1;
  
  iVar1 = FUN_00106bcf(param_1,param_3,param_2);
  if (iVar1 < 0) {
    FUN_00115992();
  }
  return iVar1 != 0;
}




// Function: show_date @ 0x4441

void show_date(undefined8 param_1,undefined8 param_2,ulong param_3,undefined8 param_4)

{
  long lVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined8 local_a8;
  ulong local_a0;
  undefined8 local_98;
  undefined8 local_90;
  char *local_80;
  undefined1 local_78 [64];
  undefined1 local_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_a8 = param_2;
  local_a0 = param_3;
  local_98 = param_4;
  local_90 = param_1;
  lVar1 = FUN_00114623(param_4,&local_a8,local_78);
  if (lVar1 == 0) {
    local_80 = (char *)FUN_001040a7(local_a8,local_38);
    uVar2 = FUN_0011401b(local_80);
    uVar3 = gettext("time %s is out of range");
    error(0,0,uVar3,uVar2);
    fputs_unlocked(local_80,stdout);
  }
  else {
    FUN_001080da(stdout,local_90,local_78,local_98,local_a0 & 0xffffffff);
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: print_only_size @ 0x4559

void print_only_size(long param_1)

{
  FILE *__stream;
  char *__s;
  long in_FS_OFFSET;
  undefined1 local_2b8 [664];
  long local_20;
  
  __stream = stdout;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 == -1) {
    __s = (char *)gettext("Infinity");
  }
  else {
    __s = (char *)FUN_0010f661(param_1,local_2b8,DAT_00121148,1,DAT_00121170);
  }
  fputs_unlocked(__s,__stream);
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: print_size @ 0x45f2

void print_size(undefined8 *param_1,undefined8 param_2)

{
  undefined8 uVar1;
  
  if (DAT_0012114c == '\0') {
    uVar1 = *param_1;
  }
  else {
    uVar1 = param_1[1];
  }
  FUN_00104559(uVar1);
  if (DAT_0012114d != '\0') {
    putchar_unlocked(9);
    FUN_00104441(DAT_00121160,param_1[2],param_1[3],DAT_00121168);
  }
  if (DAT_0012113c == '\0') {
    uVar1 = 10;
  }
  else {
    uVar1 = 0;
  }
  printf("\t%s%c",param_2,uVar1);
  fflush_unlocked(stdout);
  return;
}




// Function: fill_mount_table @ 0x46a8

void fill_mount_table(void)

{
  long lVar1;
  int iVar2;
  long in_FS_OFFSET;
  long local_b8;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_b8 = FUN_00111107(0);
  while (local_b8 != 0) {
    if (((*(byte *)(local_b8 + 0x28) & 2) == 0) && ((*(byte *)(local_b8 + 0x28) & 1) == 0)) {
      iVar2 = stat(*(char **)(local_b8 + 8),&local_a8);
      if (iVar2 == 0) {
        FUN_001043fb(DAT_00121128,local_a8.st_ino,local_a8.st_dev);
      }
    }
    lVar1 = *(long *)(local_b8 + 0x30);
    FUN_00111d9d(local_b8);
    local_b8 = lVar1;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: mount_point_in_fts_cycle @ 0x4795

undefined8 mount_point_in_fts_cycle(long *param_1)

{
  long *plVar1;
  int iVar2;
  long *local_20;
  
  plVar1 = (long *)*param_1;
  local_20 = param_1;
  if (DAT_00121128 == 0) {
    DAT_00121128 = FUN_00106936();
    if (DAT_00121128 == 0) {
      FUN_00115992();
    }
    FUN_001046a8();
  }
  for (; (local_20 != (long *)0x0 && (local_20 != plVar1)); local_20 = (long *)local_20[1]) {
    iVar2 = FUN_00106c4b(DAT_00121128,local_20[0xe],local_20[0xf]);
    if (0 < iVar2) {
      return 1;
    }
  }
  return 0;
}




// Function: process_file @ 0x4831

undefined1 process_file(long param_1,long param_2)

{
  ushort uVar1;
  char cVar2;
  long *plVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  ulong uVar7;
  long lVar8;
  long in_FS_OFFSET;
  bool bVar9;
  undefined1 auVar10 [16];
  undefined1 local_a6;
  char local_a5;
  ulong local_a0;
  ulong local_68;
  ulong local_60;
  undefined8 local_58;
  undefined8 local_50;
  ulong local_48;
  ulong local_40;
  undefined8 local_38;
  undefined8 local_30;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_a6 = 1;
  uVar6 = *(undefined8 *)(param_2 + 0x38);
  plVar3 = (long *)(param_2 + 0x70);
  uVar1 = *(ushort *)(param_2 + 0x68);
  if (uVar1 == 4) {
    uVar4 = FUN_00113bd4(4,uVar6);
    uVar5 = gettext("cannot read directory %s");
    error(0,*(undefined4 *)(param_2 + 0x40),uVar5,uVar4);
    local_a6 = 0;
  }
  else if (uVar1 != 6) {
    local_a5 = FUN_001076f0(DAT_00121178,uVar6);
    if (local_a5 != '\x01') {
      if (uVar1 == 0xb) {
        FUN_0010c3cb(param_1,param_2,1);
        lVar8 = FUN_0010bd55(param_1);
        if (lVar8 != param_2) {
                    /* WARNING: Subroutine does not return */
          __assert_fail("e == ent","src/du.c",0x20f,"process_file");
        }
        uVar1 = *(ushort *)(param_2 + 0x68);
      }
      if ((uVar1 == 10) || (uVar1 == 0xd)) {
        uVar6 = FUN_00113bd4(4,uVar6);
        uVar4 = gettext("cannot access %s");
        error(0,*(undefined4 *)(param_2 + 0x40),uVar4,uVar6);
        local_a6 = 0;
        goto LAB_00105041;
      }
      if ((((*(uint *)(param_1 + 0x48) & 0x40) != 0) && (0 < *(long *)(param_2 + 0x58))) &&
         (*(long *)(param_1 + 0x18) != *plVar3)) {
        local_a5 = '\x01';
      }
    }
    if ((local_a5 != '\0') ||
       (((DAT_0012113a != '\x01' &&
         ((DAT_0012113b != '\0' ||
          (((*(uint *)(param_2 + 0x88) & 0xf000) != 0x4000 && (1 < *(ulong *)(param_2 + 0x80)))))))
        && (cVar2 = FUN_001043fb(DAT_00121120,*(undefined8 *)(param_2 + 0x78),*plVar3),
           cVar2 != '\x01')))) {
      if (uVar1 == 1) {
        FUN_0010c3cb(param_1,param_2,4);
        lVar8 = FUN_0010bd55(param_1);
        if (lVar8 != param_2) {
                    /* WARNING: Subroutine does not return */
          __assert_fail("e == ent","src/du.c",0x230,"process_file");
        }
      }
      local_a6 = 1;
      goto LAB_00105041;
    }
    if (uVar1 == 7) {
      uVar4 = FUN_00113d66(0,3,uVar6);
      error(0,*(undefined4 *)(param_2 + 0x40),&DAT_0011a1ba,uVar4);
      local_a6 = 0;
    }
    else if (uVar1 < 8) {
      if (uVar1 == 1) {
        local_a6 = 1;
        goto LAB_00105041;
      }
      if (uVar1 == 2) {
        cVar2 = FUN_00115a4d(param_1,param_2);
        if ((cVar2 == '\0') || (cVar2 = FUN_00104795(param_2), cVar2 == '\x01')) {
          local_a6 = 1;
        }
        else {
          uVar6 = FUN_00113d66(0,3,uVar6);
          uVar4 = gettext(
                         "WARNING: Circular directory structure.\nThis almost certainly means that you have a corrupted file system.\nNOTIFY YOUR SYSTEM MANAGER.\nThe following directory is part of the cycle:\n  %s\n"
                         );
          error(0,0,uVar4,uVar6);
          local_a6 = 0;
        }
        goto LAB_00105041;
      }
    }
  }
  if (DAT_00121150 == 0) {
    auVar10 = FUN_001140c7(plVar3);
  }
  else if (DAT_00121150 == 2) {
    auVar10 = FUN_00114093(plVar3);
  }
  else {
    auVar10 = FUN_001140ad(plVar3);
  }
  if (DAT_00121139 == '\0') {
    lVar8 = *(long *)(param_2 + 0xb0) << 9;
  }
  else {
    lVar8 = *(long *)(param_2 + 0xa0);
    if (lVar8 < 0) {
      lVar8 = 0;
    }
  }
  FUN_0010411a(&local_68,lVar8,auVar10._0_8_,auVar10._8_8_);
  uVar7 = *(ulong *)(param_2 + 0x58);
  local_48 = local_68;
  local_40 = local_60;
  local_38 = local_58;
  local_30 = local_50;
  if (DAT_001211a0 == 0) {
    DAT_001211a0 = uVar7 + 10;
    DAT_001211a8 = FUN_00115838(DAT_001211a0,0x40);
  }
  else if (uVar7 != DAT_00121130) {
    if (DAT_00121130 < uVar7) {
      local_a0 = DAT_00121130;
      if (DAT_001211a0 <= uVar7) {
        DAT_001211a8 = FUN_00115307(DAT_001211a8,uVar7,0x80);
        DAT_001211a0 = uVar7 * 2;
        local_a0 = DAT_00121130;
      }
      while (local_a0 = local_a0 + 1, local_a0 <= uVar7) {
        FUN_001040da(local_a0 * 0x40 + DAT_001211a8);
        FUN_001040da(local_a0 * 0x40 + DAT_001211a8 + 0x20);
      }
    }
    else {
      if (uVar7 != DAT_00121130 - 1) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("level == prev_level - 1","src/du.c",0x27e,"process_file");
      }
      FUN_0010415c(&local_48,DAT_00121130 * 0x40 + DAT_001211a8);
      if (DAT_0012113e != '\x01') {
        FUN_0010415c(&local_48,DAT_00121130 * 0x40 + DAT_001211a8 + 0x20);
      }
      FUN_0010415c(uVar7 * 0x40 + DAT_001211a8 + 0x20,DAT_00121130 * 0x40 + DAT_001211a8);
      FUN_0010415c(uVar7 * 0x40 + DAT_001211a8 + 0x20,DAT_00121130 * 0x40 + DAT_001211a8 + 0x20);
    }
  }
  DAT_00121130 = uVar7;
  if ((DAT_0012113e != '\x01') || ((uVar1 != 6 && (uVar1 != 4)))) {
    FUN_0010415c(uVar7 * 0x40 + DAT_001211a8,&local_68);
  }
  FUN_0010415c(&DAT_00121180,&local_68);
  if (((((uVar1 == 6) || (uVar1 == 4)) && (uVar7 <= DAT_00121010)) ||
      ((DAT_00121138 != '\0' && (uVar7 <= DAT_00121010)))) || (uVar7 == 0)) {
    uVar7 = local_48;
    if (DAT_0012114c != '\0') {
      uVar7 = local_40;
    }
    if ((long)DAT_00121140 < 0) {
      bVar9 = uVar7 <= -DAT_00121140;
    }
    else {
      bVar9 = DAT_00121140 <= uVar7;
    }
    if (bVar9) {
      FUN_001045f2(&local_48,uVar6);
    }
  }
LAB_00105041:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_a6;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: du_files @ 0x505b

bool du_files(long *param_1,undefined4 param_2)

{
  byte bVar1;
  int iVar2;
  long lVar3;
  long lVar4;
  int *piVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  bool local_29;
  
  local_29 = true;
  if (*param_1 != 0) {
    lVar3 = FUN_001159d2(param_1,param_2,0);
    while( true ) {
      lVar4 = FUN_0010bd55(lVar3);
      if (lVar4 == 0) break;
      bVar1 = FUN_00104831(lVar3,lVar4);
      local_29 = (bVar1 & local_29) != 0;
    }
    piVar5 = __errno_location();
    if (*piVar5 != 0) {
      uVar6 = FUN_00113d66(0,3,*(undefined8 *)(lVar3 + 0x20));
      uVar7 = gettext("fts_read failed: %s");
      piVar5 = __errno_location();
      error(0,*piVar5,uVar7,uVar6);
      local_29 = false;
    }
    DAT_00121130 = 0;
    iVar2 = FUN_0010b89b(lVar3);
    if (iVar2 != 0) {
      uVar6 = gettext("fts_close failed");
      piVar5 = __errno_location();
      error(0,*piVar5,uVar6);
      local_29 = false;
    }
  }
  return local_29;
}




// Function: main @ 0x5197

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

byte main(int param_1,undefined8 *param_2)

{
  uint uVar1;
  bool bVar2;
  bool bVar3;
  byte bVar4;
  int iVar5;
  char *pcVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  long lVar9;
  int *piVar10;
  long in_FS_OFFSET;
  bool local_93;
  undefined4 local_90;
  uint local_8c;
  uint local_88;
  int local_84;
  int local_80;
  int local_7c;
  uint local_78;
  undefined4 uStack_74;
  char *local_70;
  long local_68;
  long local_60;
  char *local_58;
  undefined8 *local_50;
  char *local_48;
  undefined8 local_40;
  undefined8 local_38;
  undefined8 local_30;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  bVar3 = false;
  local_93 = true;
  local_70 = (char *)0x0;
  local_8c = 8;
  local_88 = 0x10;
  bVar2 = false;
  local_38 = FUN_001040cc(&DAT_0011a2b7);
  local_30 = 0;
  FUN_00111fda(*param_2);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  FUN_001186c0(FUN_00106762);
  DAT_00121178 = FUN_00106ebe();
  pcVar6 = getenv("DU_BLOCK_SIZE");
  FUN_001102cd(pcVar6,&DAT_00121148,&DAT_00121170);
LAB_001052a5:
  local_90 = 0xffffffff;
  local_84 = getopt_long(param_1,param_2,"0abd:chHklmst:xB:DLPSX:",&PTR_DAT_00120680,&local_90);
  if (local_84 != -1) {
    if (0x87 < local_84) {
switchD_0010535f_caseD_31:
      local_93 = false;
      goto LAB_001052a5;
    }
    if (local_84 < 0x30) {
      if (local_84 == -0x83) {
LAB_00105760:
        FUN_0011517a(stdout,&DAT_0011a154,"GNU coreutils",PTR_DAT_00121018,"Torbjorn Granlund",
                     "David MacKenzie","Paul Eggert","Jim Meyering",0);
                    /* WARNING: Subroutine does not return */
        exit(0);
      }
      if (local_84 == -0x82) {
        FUN_001041ff(0);
        goto LAB_00105760;
      }
      goto switchD_0010535f_caseD_31;
    }
    switch(local_84) {
    case 0x30:
      DAT_0012113c = 1;
      break;
    default:
      goto switchD_0010535f_caseD_31;
    case 0x42:
      local_7c = FUN_001102cd(optarg,&DAT_00121148,&DAT_00121170);
      if (local_7c == 0) break;
      FUN_001160c2(local_7c,local_90,(int)(char)local_84,&PTR_DAT_00120680,optarg);
    case 0x44:
    case 0x48:
      local_88 = 0x11;
      break;
    case 0x4c:
      local_88 = 2;
      break;
    case 0x50:
      local_88 = 0x10;
      break;
    case 0x53:
      DAT_0012113e = 1;
      break;
    case 0x58:
      iVar5 = FUN_00107dac(FUN_001077e3,DAT_00121178,optarg,0x10000000,10);
      if (iVar5 != 0) {
        uVar7 = FUN_00113d66(0,3,optarg);
        piVar10 = __errno_location();
        error(0,*piVar10,&DAT_0011a1ba,uVar7);
        local_93 = false;
      }
      break;
    case 0x61:
      DAT_00121138 = '\x01';
      break;
    case 0x62:
      DAT_00121139 = '\x01';
      DAT_00121148 = 0;
      DAT_00121170 = 1;
      break;
    case 99:
      DAT_0012113d = '\x01';
      break;
    case 100:
      iVar5 = FUN_001161e3(optarg,0,0,&local_78,&DAT_0011934b);
      if (iVar5 == 0) {
        bVar3 = true;
        DAT_00121010 = CONCAT44(uStack_74,local_78);
      }
      else {
        uVar7 = FUN_0011401b(optarg);
        uVar8 = gettext("invalid maximum depth %s");
        error(0,0,uVar8,uVar7);
        local_93 = false;
      }
      break;
    case 0x68:
      DAT_00121148 = 0xb0;
      DAT_00121170 = 1;
      break;
    case 0x6b:
      DAT_00121148 = 0;
      DAT_00121170 = 0x400;
      break;
    case 0x6c:
      DAT_0012113a = '\x01';
      break;
    case 0x6d:
      DAT_00121148 = 0;
      DAT_00121170 = 0x100000;
      break;
    case 0x73:
      bVar2 = true;
      break;
    case 0x74:
      local_80 = FUN_00115b9a(optarg,0,0,&DAT_00121140,"kKmMGTPEZY0");
      if (local_80 != 0) {
        FUN_001160c2(local_80,local_90,(int)(char)local_84,&PTR_DAT_00120680,optarg);
      }
      if ((DAT_00121140 == 0) && (*optarg == '-')) {
        uVar7 = gettext("invalid --threshold argument \'-0\'");
        error(1,0,uVar7);
        goto switchD_0010535f_caseD_78;
      }
      break;
    case 0x78:
switchD_0010535f_caseD_78:
      local_8c = local_8c | 0x40;
      break;
    case 0x80:
      DAT_00121139 = '\x01';
      break;
    case 0x81:
      FUN_001077e3(DAT_00121178,optarg,0x10000000);
      break;
    case 0x82:
      local_70 = optarg;
      break;
    case 0x83:
      DAT_00121148 = 0x90;
      DAT_00121170 = 1;
      break;
    case 0x85:
      DAT_0012114d = '\x01';
      if (optarg == (char *)0x0) {
        DAT_00121150 = 0;
      }
      else {
        lVar9 = FUN_001063c9("--time",optarg,&PTR_s_atime_001209c0,&DAT_001194a0,4,PTR_FUN_00121020,
                             1);
        DAT_00121150 = *(undefined4 *)(&DAT_001194a0 + lVar9 * 4);
      }
      pcVar6 = getenv("TZ");
      DAT_00121168 = FUN_001141a3(pcVar6);
      break;
    case 0x86:
      DAT_00121158 = optarg;
      break;
    case 0x87:
      goto switchD_0010535f_caseD_87;
    }
    goto LAB_001052a5;
  }
  if (local_93 != true) {
    FUN_001041ff(1);
  }
  if ((DAT_00121138 != '\0') && (bVar2)) {
    uVar7 = gettext("cannot both summarize and show all entries");
    error(0,0,uVar7);
    FUN_001041ff(1);
  }
  if (((bVar2) && (bVar3)) && (DAT_00121010 == 0)) {
    uVar7 = gettext("warning: summarizing is the same as using --max-depth=0");
    error(0,0,uVar7);
  }
  if (((bVar2) && (bVar3)) && (DAT_00121010 != 0)) {
    local_60 = DAT_00121010;
    uVar7 = gettext("warning: summarizing conflicts with --max-depth=%lu");
    error(0,0,uVar7,local_60);
    FUN_001041ff(1);
  }
  if (bVar2) {
    DAT_00121010 = 0;
  }
  if (DAT_0012114c != '\0') {
    if (DAT_00121139 != '\0') {
      uVar7 = gettext("warning: options --apparent-size and -b are ineffective with --inodes");
      error(0,0,uVar7);
    }
    DAT_00121170 = 1;
  }
  if (DAT_0012114d != '\0') {
    if (DAT_00121158 == (char *)0x0) {
      DAT_00121158 = getenv("TIME_STYLE");
      if ((DAT_00121158 == (char *)0x0) || (iVar5 = strcmp(DAT_00121158,"locale"), iVar5 == 0)) {
        DAT_00121158 = "long-iso";
      }
      else if (*DAT_00121158 == '+') {
        local_58 = strchr(DAT_00121158,10);
        if (local_58 != (char *)0x0) {
          *local_58 = '\0';
        }
      }
      else {
        while (iVar5 = strncmp(DAT_00121158,"posix-",DAT_0011a750), iVar5 == 0) {
          DAT_00121158 = DAT_00121158 + DAT_0011a750;
        }
      }
    }
    if (*DAT_00121158 == '+') {
      DAT_00121160 = DAT_00121158 + 1;
    }
    else {
      lVar9 = FUN_001063c9("time style",DAT_00121158,&PTR_s_full_iso_00120a00,&DAT_001194d0,4,
                           PTR_FUN_00121020,1);
      uVar1 = *(uint *)(&DAT_001194d0 + lVar9 * 4);
      if (uVar1 == 2) {
        DAT_00121160 = "%Y-%m-%d";
      }
      else if (uVar1 < 3) {
        if (uVar1 == 0) {
          DAT_00121160 = "%Y-%m-%d %H:%M:%S.%N %z";
        }
        else if (uVar1 == 1) {
          DAT_00121160 = "%Y-%m-%d %H:%M";
        }
      }
    }
  }
  if (local_70 == (char *)0x0) {
    if (optind < param_1) {
      local_50 = param_2 + optind;
    }
    else {
      local_50 = &local_38;
    }
    local_68 = FUN_00106508(local_50);
    if ((optind + 1 < param_1) || (local_88 == 2)) {
      DAT_0012113b = '\x01';
    }
    else {
      DAT_0012113b = '\0';
    }
  }
  else {
    if (optind < param_1) {
      uVar7 = FUN_0011401b(param_2[optind]);
      uVar8 = gettext("extra operand %s");
      error(0,0,uVar8,uVar7);
      uVar7 = gettext("file operands cannot be combined with --files0-from");
      fprintf(stderr,"%s\n",uVar7);
      FUN_001041ff(1);
    }
    iVar5 = strcmp(local_70,"-");
    if ((iVar5 != 0) && (lVar9 = FUN_0010abea(local_70,&DAT_0011a50a,stdin), lVar9 == 0)) {
      uVar7 = FUN_00113bd4(4,local_70);
      uVar8 = gettext("cannot open %s for reading");
      piVar10 = __errno_location();
      error(1,*piVar10,uVar8,uVar7);
    }
    local_68 = FUN_0010655d(stdin);
    DAT_0012113b = '\x01';
  }
  if (local_68 == 0) {
    FUN_00115992();
  }
  DAT_00121120 = FUN_00106936();
  if (DAT_00121120 == 0) {
    FUN_00115992();
  }
  if ((DAT_0012113a != '\0') || (DAT_0012113b != '\x01')) {
    local_8c = local_8c | 0x100;
  }
  local_8c = local_8c | local_88;
  while( true ) {
    bVar2 = false;
    local_48 = (char *)FUN_001065ca(local_68,&local_78);
    if (local_48 == (char *)0x0) break;
    if (((local_70 != (char *)0x0) && (iVar5 = strcmp(local_70,"-"), iVar5 == 0)) &&
       (iVar5 = strcmp(local_48,"-"), iVar5 == 0)) {
      uVar7 = FUN_00113bd4(4,local_48);
      uVar8 = gettext("when reading file names from stdin, no file name of %s allowed");
      error(0,0,uVar8,uVar7);
      bVar2 = true;
    }
    if (*local_48 == '\0') {
      if (local_70 == (char *)0x0) {
        uVar7 = gettext("invalid zero-length file name");
        error(0,0,&DAT_0011a1ba,uVar7);
      }
      else {
        local_40 = FUN_001066b5(local_68);
        uVar7 = gettext("invalid zero-length file name");
        uVar8 = FUN_00113d66(0,3,local_70);
        error(0,0,"%s:%lu: %s",uVar8,local_40,uVar7);
      }
      bVar2 = true;
    }
    if (bVar2) {
      local_93 = false;
    }
    else {
      _DAT_001211b0 = local_48;
      bVar4 = FUN_0010505b(&DAT_001211b0,local_8c);
      local_93 = (bVar4 & local_93) != 0;
    }
  }
  if (local_78 == 4) {
    uVar7 = FUN_00113d66(0,3,local_70);
    uVar8 = gettext("%s: read error");
    piVar10 = __errno_location();
    error(0,*piVar10,uVar8,uVar7);
    local_93 = false;
LAB_00105f1c:
    FUN_001066f3(local_68);
    FUN_001069c8(DAT_00121120);
    if (DAT_00121128 != 0) {
      FUN_001069c8(DAT_00121128);
    }
    if ((local_70 != (char *)0x0) &&
       (((iVar5 = ferror_unlocked(stdin), iVar5 != 0 || (iVar5 = FUN_00107ea0(stdin), iVar5 != 0))
        && (local_93 != false)))) {
      uVar7 = FUN_00113bd4(4,local_70);
      uVar8 = gettext("error reading %s");
      error(1,0,uVar8,uVar7);
    }
    if (DAT_0012113d != '\0') {
      uVar7 = gettext("total");
      FUN_001045f2(&DAT_00121180,uVar7);
    }
    if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    return local_93 ^ 1;
  }
  if (local_78 < 5) {
    if (local_78 == 2) goto LAB_00105f1c;
    if (local_78 == 3) {
      FUN_00115992();
    }
  }
                    /* WARNING: Subroutine does not return */
  __assert_fail("!\"unexpected error code from argv_iter\"","src/du.c",0x439,"main");
switchD_0010535f_caseD_87:
  DAT_0012114c = '\x01';
  goto LAB_001052a5;
}




// Function: hash_get_n_buckets @ 0xd9a8

undefined8 hash_get_n_buckets(long param_1)

{
  return *(undefined8 *)(param_1 + 0x10);
}




// Function: hash_get_n_buckets_used @ 0xd9be

undefined8 hash_get_n_buckets_used(long param_1)

{
  return *(undefined8 *)(param_1 + 0x18);
}




// Function: hash_get_n_entries @ 0xd9d4

undefined8 hash_get_n_entries(long param_1)

{
  return *(undefined8 *)(param_1 + 0x20);
}




// Function: hash_get_max_bucket_length @ 0xd9ea

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




// Function: hash_table_ok @ 0xda6c

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




// Function: hash_print_statistics @ 0xdb0a

void hash_print_statistics(undefined8 param_1,FILE *param_2)

{
  undefined8 uVar1;
  ulong uVar2;
  ulong uVar3;
  undefined8 uVar4;
  
  uVar1 = FUN_0010d9d4(param_1);
  uVar2 = FUN_0010d9a8(param_1);
  uVar3 = FUN_0010d9be(param_1);
  uVar4 = FUN_0010d9ea(param_1);
  fprintf(param_2,"# entries:         %lu\n",uVar1);
  fprintf(param_2,"# buckets:         %lu\n",uVar2);
  fprintf(param_2,"# buckets used:    %lu (%.2f%%)\n",(DAT_0011aac8 * (double)uVar3) / (double)uVar2
          ,uVar3);
  fprintf(param_2,"max bucket length: %lu\n",uVar4);
  return;
}




// Function: safe_hasher @ 0xdc51

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




// Function: hash_lookup @ 0xdcac

long hash_lookup(long param_1,long param_2)

{
  char cVar1;
  long *local_18;
  
  local_18 = (long *)FUN_0010dc51(param_1,param_2);
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




// Function: hash_get_first @ 0xdd43

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




// Function: hash_get_next @ 0xdda3

long hash_get_next(long param_1,long param_2)

{
  long *local_18;
  long *local_10;
  
  local_18 = (long *)FUN_0010dc51(param_1,param_2);
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




// Function: hash_get_entries @ 0xde41

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




// Function: hash_do_for_each @ 0xdee1

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




// Function: hash_string @ 0xdf79

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




// Function: is_prime @ 0xdfd3

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




// Function: next_prime @ 0xe042

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
    cVar1 = FUN_0010dfd3(local_10);
    if (cVar1 == '\x01') break;
    local_10 = local_10 + 2;
  }
  return local_10;
}




// Function: hash_reset_tuning @ 0xe08d

void hash_reset_tuning(undefined4 *param_1)

{
  *param_1 = 0;
  param_1[1] = DAT_0011aad0;
  param_1[2] = DAT_0011aad4;
  param_1[3] = DAT_0011aad8;
  *(undefined1 *)(param_1 + 4) = 0;
  return;
}




// Function: raw_hasher @ 0xe0e3

ulong raw_hasher(undefined8 param_1,ulong param_2)

{
  ulong uVar1;
  
  uVar1 = FUN_001175f6(param_1,3);
  return uVar1 % param_2;
}




// Function: raw_comparator @ 0xe11e

bool raw_comparator(long param_1,long param_2)

{
  return param_1 == param_2;
}




// Function: check_tuning @ 0xe13b

undefined8 check_tuning(long param_1)

{
  float *pfVar1;
  undefined8 uVar2;
  
  pfVar1 = *(float **)(param_1 + 0x28);
  if (pfVar1 == (float *)&DAT_0011aa40) {
    uVar2 = 1;
  }
  else if ((((pfVar1[2] <= DAT_0011aadc) || (DAT_0011aad0 - DAT_0011aadc <= pfVar1[2])) ||
           (pfVar1[3] <= DAT_0011aadc + DAT_0011aad0)) ||
          (((*pfVar1 < 0.0 || (pfVar1[1] <= *pfVar1 + DAT_0011aadc)) ||
           ((DAT_0011aad0 < pfVar1[1] || (pfVar1[2] <= *pfVar1 + DAT_0011aadc)))))) {
    *(undefined **)(param_1 + 0x28) = &DAT_0011aa40;
    uVar2 = 0;
  }
  else {
    uVar2 = 1;
  }
  return uVar2;
}




// Function: compute_bucket_size @ 0xe248

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulong compute_bucket_size(ulong param_1,long param_2)

{
  ulong uVar1;
  float fVar2;
  ulong local_20;
  
  local_20 = param_1;
  if (*(char *)(param_2 + 0x10) != '\x01') {
    fVar2 = (float)param_1 / *(float *)(param_2 + 8);
    if (_DAT_0011aae0 <= fVar2) {
      return 0;
    }
    if (DAT_0011aae4 <= fVar2) {
      local_20 = (long)(fVar2 - DAT_0011aae4) ^ 0x8000000000000000;
    }
    else {
      local_20 = (ulong)fVar2;
    }
  }
  uVar1 = FUN_0010e042(local_20);
  if ((long)(uVar1 << 3) < 0 || uVar1 >> 0x3d != 0) {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: hash_initialize @ 0xe361

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
    local_30 = FUN_0010e0e3;
  }
  local_38 = param_4;
  if (param_4 == (code *)0x0) {
    local_38 = FUN_0010e11e;
  }
  __ptr = malloc(0x50);
  if (__ptr != (long *)0x0) {
    local_28 = param_2;
    if (param_2 == (undefined *)0x0) {
      local_28 = &DAT_0011aa40;
    }
    __ptr[5] = (long)local_28;
    cVar1 = FUN_0010e13b(__ptr);
    if (cVar1 == '\x01') {
      lVar2 = FUN_0010e248(param_1,local_28);
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




// Function: hash_clear @ 0xe4d6

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




// Function: hash_free @ 0xe5e1

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




// Function: allocate_entry @ 0xe710

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




// Function: free_entry @ 0xe75f

void free_entry(long param_1,undefined8 *param_2)

{
  *param_2 = 0;
  param_2[1] = *(undefined8 *)(param_1 + 0x48);
  *(undefined8 **)(param_1 + 0x48) = param_2;
  return;
}




// Function: hash_find_entry @ 0xe799

long hash_find_entry(long param_1,long param_2,undefined8 *param_3,char param_4)

{
  long *plVar1;
  long lVar2;
  char cVar3;
  long lVar4;
  long *local_38;
  
  local_38 = (long *)FUN_0010dc51(param_1,param_2);
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
        FUN_0010e75f(param_1,plVar1);
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
        FUN_0010e75f(param_1,lVar2);
        return lVar4;
      }
    }
    lVar4 = 0;
  }
  return lVar4;
}




// Function: transfer_entries @ 0xe932

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
        plVar3 = (long *)FUN_0010dc51(param_1,lVar1);
        plVar2 = (long *)local_30[1];
        if (*plVar3 == 0) {
          *plVar3 = lVar1;
          *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
          FUN_0010e75f(param_1,local_30);
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
        plVar2 = (long *)FUN_0010dc51(param_1,lVar1);
        if (*plVar2 == 0) {
          *plVar2 = lVar1;
          *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
        }
        else {
          plVar3 = (long *)FUN_0010e710(param_1);
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




// Function: hash_rehash @ 0xeb06

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
  __nmemb = FUN_0010e248(param_2,param_1[5]);
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
      cVar1 = FUN_0010e932(&local_68,param_1,0);
      if (cVar1 == '\0') {
        param_1[9] = local_20;
        cVar1 = FUN_0010e932(param_1,&local_68,1);
        if (cVar1 == '\x01') {
          cVar1 = FUN_0010e932(param_1,&local_68,0);
          if (cVar1 == '\x01') {
            free(local_68);
            uVar2 = 0;
            goto LAB_0010ed1b;
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
LAB_0010ed1b:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: hash_insert_if_absent @ 0xed31

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
  local_28 = FUN_0010e799(param_1,param_2,&local_30,0);
  if (local_28 != 0) {
    if (param_3 != (long *)0x0) {
      *param_3 = local_28;
    }
    uVar2 = 0;
    goto LAB_0010f08c;
  }
  if (((float)*(ulong *)(param_1 + 0x10) * *(float *)(*(long *)(param_1 + 0x28) + 8) <
       (float)*(ulong *)(param_1 + 0x18)) &&
     (FUN_0010e13b(param_1),
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
    if (_DAT_0011aae0 <= fVar5) {
      uVar2 = 0xffffffff;
      goto LAB_0010f08c;
    }
    if (DAT_0011aae4 <= fVar5) {
      uVar3 = (long)(fVar5 - DAT_0011aae4) ^ 0x8000000000000000;
    }
    else {
      uVar3 = (ulong)fVar5;
    }
    cVar1 = FUN_0010eb06(param_1,uVar3);
    if (cVar1 != '\x01') {
      uVar2 = 0xffffffff;
      goto LAB_0010f08c;
    }
    lVar4 = FUN_0010e799(param_1,param_2,&local_30,0);
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
    local_18 = (long *)FUN_0010e710(param_1);
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
LAB_0010f08c:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: hash_insert @ 0xf0a2

undefined8 hash_insert(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_0010ed31(param_1,param_2,&local_18);
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




// Function: hash_remove @ 0xf112

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
  local_30 = FUN_0010e799(param_1,param_2,&local_40,1);
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
       (FUN_0010e13b(param_1), lVar2 = local_30,
       (float)*(ulong *)(param_1 + 0x18) <
       (float)*(ulong *)(param_1 + 0x10) * **(float **)(param_1 + 0x28))) {
      local_28 = *(long *)(param_1 + 0x28);
      if (*(char *)(local_28 + 0x10) == '\0') {
        fVar3 = *(float *)(local_28 + 8) *
                *(float *)(local_28 + 4) * (float)*(ulong *)(param_1 + 0x10);
        if (DAT_0011aae4 <= fVar3) {
          local_20 = (long)(fVar3 - DAT_0011aae4) ^ 0x8000000000000000;
        }
        else {
          local_20 = (ulong)fVar3;
        }
      }
      else {
        fVar3 = (float)*(ulong *)(param_1 + 0x10) * *(float *)(local_28 + 4);
        if (DAT_0011aae4 <= fVar3) {
          local_20 = (long)(fVar3 - DAT_0011aae4) ^ 0x8000000000000000;
        }
        else {
          local_20 = (ulong)fVar3;
        }
      }
      cVar1 = FUN_0010eb06(param_1,local_20);
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




// Function: hash_delete @ 0xf418

void hash_delete(undefined8 param_1,undefined8 param_2)

{
  FUN_0010f112(param_1,param_2);
  return;
}




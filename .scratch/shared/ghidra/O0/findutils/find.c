// Function: left_dir @ 0x73b9

void left_dir(void)

{
  if (((DAT_00149010 & 0x200) != 0) && (-1 < DAT_00149018)) {
    close(DAT_00149018);
    DAT_00149018 = -1;
  }
  return;
}




// Function: inside_dir @ 0x73f4

void inside_dir(int param_1)

{
  if ((DAT_00149010 & 0x200) != 0) {
    if ((param_1 != -100) && (param_1 < 0)) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("dir_fd == AT_FDCWD || dir_fd >= 0","ftsfind.c",0x6b,"inside_dir");
    }
    DAT_001499d8 = param_1;
    if (DAT_00149018 < 0) {
      if (param_1 == -100) {
        DAT_00149018 = -100;
      }
      else if (param_1 < 0) {
        if ((DAT_00149018 < 0) && (param_1 < 0)) {
                    /* WARNING: Subroutine does not return */
          __assert_fail("curr_fd >= 0 || dir_fd >= 0","ftsfind.c",0x7d,"inside_dir");
        }
      }
      else {
        DAT_00149018 = FUN_0011d639(param_1);
      }
    }
  }
  return;
}




// Function: get_fts_info_name @ 0x74c2

char * get_fts_info_name(uint param_1)

{
  char *pcVar1;
  
  switch(param_1) {
  default:
    sprintf(&DAT_00149918,"[%d]",(ulong)param_1);
    pcVar1 = &DAT_00149918;
    break;
  case 1:
    pcVar1 = "FTS_D";
    break;
  case 2:
    pcVar1 = "FTS_DC";
    break;
  case 3:
    pcVar1 = "FTS_DEFAULT";
    break;
  case 4:
    pcVar1 = "FTS_DNR";
    break;
  case 5:
    pcVar1 = "FTS_DOT";
    break;
  case 6:
    pcVar1 = "FTS_DP";
    break;
  case 7:
    pcVar1 = "FTS_ERR";
    break;
  case 8:
    pcVar1 = "FTS_F";
    break;
  case 9:
    pcVar1 = "FTS_INIT";
    break;
  case 10:
    pcVar1 = "FTS_NS";
    break;
  case 0xb:
    pcVar1 = "FTS_NSOK";
    break;
  case 0xc:
    pcVar1 = "FTS_SL";
    break;
  case 0xd:
    pcVar1 = "FTS_SLNONE";
    break;
  case 0xe:
    pcVar1 = "FTS_W";
  }
  return pcVar1;
}




// Function: visit @ 0x75b5

void visit(long param_1,long param_2,undefined8 param_3)

{
  undefined8 uVar1;
  
  if ((*(short *)(param_2 + 0x68) == 10) || (*(short *)(param_2 + 0x68) == 0xb)) {
    DAT_001499c4 = 0;
  }
  else {
    DAT_001499c4 = 1;
  }
  DAT_001499d0 = *(undefined8 *)(param_2 + 0x30);
  DAT_001499d8 = *(undefined4 *)(param_1 + 0x2c);
  uVar1 = FUN_0010d7ff();
  FUN_00110873(*(undefined8 *)(param_2 + 0x38),param_3,uVar1);
  if (DAT_001499e0 != '\0') {
    FUN_0011fc0f(param_1,param_2,4);
  }
  return;
}




// Function: partial_quotearg_n @ 0x7664

undefined8 partial_quotearg_n(undefined4 param_1,long param_2,long param_3,undefined4 param_4)

{
  undefined1 uVar1;
  undefined8 uVar2;
  
  if (param_3 == 0) {
    uVar2 = FUN_0012f861(param_1,param_4,&DAT_00138110);
  }
  else {
    uVar1 = *(undefined1 *)(param_3 + param_2);
    *(undefined1 *)(param_3 + param_2) = 0;
    uVar2 = FUN_0012f861(param_1,param_4,param_2);
    *(undefined1 *)(param_2 + param_3) = uVar1;
  }
  return uVar2;
}




// Function: issue_loop_warning @ 0x76ea

void issue_loop_warning(long *param_1)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  
  if ((*(uint *)(param_1 + 0x11) & 0xf000) == 0xa000) {
    uVar1 = FUN_0011096a(0,param_1[7]);
    uVar2 = gettext(
                   "Symbolic link %s is part of a loop in the directory hierarchy; we have already visited the directory to which it points."
                   );
    error(0,0,uVar2,uVar1);
  }
  else {
    uVar1 = FUN_00107664(1,*(undefined8 *)(*param_1 + 0x38),*(undefined8 *)(*param_1 + 0x48),
                         DAT_001499ac);
    uVar2 = FUN_0011096a(0,param_1[7]);
    uVar3 = gettext("File system loop detected; %s is part of the same file system loop as %s.");
    error(0,0,uVar3,uVar2,uVar1);
  }
  return;
}




// Function: symlink_loop @ 0x77d0

undefined4 symlink_loop(undefined8 param_1)

{
  int iVar1;
  undefined4 uVar2;
  int *piVar3;
  long in_FS_OFFSET;
  undefined1 local_a8 [152];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = (*DAT_00149990)(param_1,local_a8);
  if (iVar1 != 0) {
    piVar3 = __errno_location();
    if (*piVar3 == 0x28) {
      uVar2 = 1;
      goto LAB_00107839;
    }
  }
  uVar2 = 0;
LAB_00107839:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: consider_visiting @ 0x7852

void consider_visiting(long param_1,long param_2)

{
  uint uVar1;
  char cVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  ulong uVar7;
  ulong uVar8;
  long in_FS_OFFSET;
  uint local_e4;
  uint local_e0;
  uint local_dc;
  undefined8 local_d8;
  undefined8 local_d0;
  undefined8 local_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined8 local_b0;
  undefined8 local_a8;
  undefined8 local_a0;
  undefined8 local_98;
  undefined8 local_90;
  undefined8 local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 local_70;
  undefined8 local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 local_50;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if ((DAT_00149980 & 4) != 0) {
    uVar3 = FUN_0012f861(2,DAT_001499ac,*(undefined8 *)(param_2 + 0x30));
    uVar4 = FUN_0012f861(1,DAT_001499ac,*(undefined8 *)(param_2 + 0x38));
    uVar1 = DAT_00149014;
    uVar7 = *(ulong *)(param_2 + 0x58);
    uVar5 = FUN_001074c2(*(undefined2 *)(param_2 + 0x68));
    uVar6 = FUN_0012f861(0,DAT_001499ac,*(undefined8 *)(param_2 + 0x38));
    fprintf(stderr,
            "consider_visiting (early): %s: fts_info=%-6s, fts_level=%2d, prev_depth=%d fts_path=%s, fts_accpath=%s\n"
            ,uVar6,uVar5,uVar7 & 0xffffffff,(ulong)uVar1,uVar4,uVar3);
  }
  if (*(short *)(param_2 + 0x68) == 6) {
    FUN_001073b9();
  }
  else if (((long)(int)DAT_00149014 < *(long *)(param_2 + 0x58)) || (*(long *)(param_2 + 0x58) == 0)
          ) {
    FUN_001073b9();
  }
  FUN_001073f4(*(undefined4 *)(param_1 + 0x2c));
  DAT_00149014 = (uint)*(undefined8 *)(param_2 + 0x58);
  local_d0 = *(undefined8 *)(param_2 + 0x78);
  if (*(short *)(param_2 + 0x68) == 7) {
    FUN_00110a13(*(undefined4 *)(param_2 + 0x40),*(undefined8 *)(param_2 + 0x38));
    goto LAB_00108017;
  }
  if (*(short *)(param_2 + 0x68) == 4) {
    FUN_00110a13(*(undefined4 *)(param_2 + 0x40),*(undefined8 *)(param_2 + 0x38));
    if (DAT_00149940 != '\0') goto LAB_00108017;
  }
  else {
    if (*(short *)(param_2 + 0x68) == 2) {
      FUN_001076ea(param_2);
      DAT_001499e4 = 1;
      goto LAB_00108017;
    }
    if (*(short *)(param_2 + 0x68) == 0xd) {
      cVar2 = FUN_001077d0(*(undefined8 *)(param_2 + 0x30));
      if (cVar2 != '\0') {
        FUN_00110a13(0x28,*(undefined8 *)(param_2 + 0x38));
        goto LAB_00108017;
      }
    }
    else if (*(short *)(param_2 + 0x68) == 10) {
      if (*(long *)(param_2 + 0x58) == 0) {
        FUN_00110a13(*(undefined4 *)(param_2 + 0x40),*(undefined8 *)(param_2 + 0x38));
        goto LAB_00108017;
      }
      cVar2 = FUN_001077d0(*(undefined8 *)(param_2 + 0x30));
      if (cVar2 != '\0') {
        FUN_00110a13(0x28,*(undefined8 *)(param_2 + 0x38));
        goto LAB_00108017;
      }
      FUN_00110a13(*(undefined4 *)(param_2 + 0x40),*(undefined8 *)(param_2 + 0x38));
    }
  }
  if ((*(short *)(param_2 + 0x68) == 0xb) || (*(short *)(param_2 + 0x68) == 10)) {
    if (DAT_001499c4 == 1) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("!state.have_stat","ftsfind.c",0x17c,"consider_visiting");
    }
    if ((*(short *)(param_2 + 0x68) != 0xb) && (DAT_001499c8 != 0)) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("ent->fts_info == FTS_NSOK || state.type == 0","ftsfind.c",0x17d,
                    "consider_visiting");
    }
    local_e4 = DAT_001499c8;
  }
  else {
    DAT_001499c4 = 1;
    DAT_001499c5 = 1;
    local_d8 = *(undefined8 *)(param_2 + 0x70);
    local_d0 = *(undefined8 *)(param_2 + 0x78);
    local_c8 = *(undefined8 *)(param_2 + 0x80);
    local_c0 = *(undefined8 *)(param_2 + 0x88);
    local_b8 = *(undefined8 *)(param_2 + 0x90);
    local_b0 = *(undefined8 *)(param_2 + 0x98);
    local_a8 = *(undefined8 *)(param_2 + 0xa0);
    local_a0 = *(undefined8 *)(param_2 + 0xa8);
    local_98 = *(undefined8 *)(param_2 + 0xb0);
    local_90 = *(undefined8 *)(param_2 + 0xb8);
    local_88 = *(undefined8 *)(param_2 + 0xc0);
    local_80 = *(undefined8 *)(param_2 + 200);
    local_78 = *(undefined8 *)(param_2 + 0xd0);
    local_70 = *(undefined8 *)(param_2 + 0xd8);
    local_68 = *(undefined8 *)(param_2 + 0xe0);
    local_60 = *(undefined8 *)(param_2 + 0xe8);
    local_50 = *(undefined8 *)(param_2 + 0xf8);
    local_58 = *(undefined8 *)(param_2 + 0xf0);
    local_e4 = (uint)local_c0;
    DAT_001499c8 = (uint)local_c0;
    if ((uint)local_c0 == 0) {
      uVar3 = FUN_0012f861(0,DAT_001499ac,*(undefined8 *)(param_2 + 0x38));
      uVar4 = gettext("WARNING: file %s appears to have mode 0000");
      error(0,0,uVar4,uVar3);
    }
  }
  DAT_001499c0 = (undefined4)*(undefined8 *)(param_2 + 0x58);
  if ((local_e4 == 0) ||
     (cVar2 = FUN_0010fe38(&local_e4,*(undefined8 *)(param_2 + 0x38),param_2 + 0x100,&local_d8,0),
     cVar2 == '\x01')) {
    local_e0 = 0;
    if (((local_e4 & 0xf000) == 0x4000) ||
       (((*(short *)(param_2 + 0x68) == 1 || (*(short *)(param_2 + 0x68) == 6)) ||
        (*(short *)(param_2 + 0x68) == 2)))) {
      local_dc = 1;
    }
    else {
      local_dc = 0;
    }
    if ((local_dc == 0) || (*(short *)(param_2 + 0x68) != 0xb)) {
      if ((-1 < DAT_00149944) &&
         (((long)DAT_00149944 <= *(long *)(param_2 + 0x58) &&
          (FUN_0011fc0f(param_1,param_2,4), (long)DAT_00149944 < *(long *)(param_2 + 0x58))))) {
        local_e0 = 1;
      }
      if ((*(short *)(param_2 + 0x68) == 1) && (DAT_00149940 != '\x01')) {
        local_e0 = 1;
      }
      else if ((*(short *)(param_2 + 0x68) == 6) && (DAT_00149940 != '\0')) {
        local_e0 = 1;
      }
      else if (*(long *)(param_2 + 0x58) < (long)DAT_00149948) {
        local_e0 = 1;
      }
      if ((DAT_00149980 & 4) != 0) {
        uVar8 = (ulong)DAT_001499c5;
        uVar7 = (ulong)DAT_001499c4;
        uVar3 = FUN_001074c2(*(undefined2 *)(param_2 + 0x68));
        uVar4 = FUN_0012f861(0,DAT_001499ac,*(undefined8 *)(param_2 + 0x38));
        fprintf(stderr,
                "consider_visiting (late): %s: fts_info=%-6s, isdir=%d ignore=%d have_stat=%d have_type=%d \n"
                ,uVar4,uVar3,(ulong)local_dc,(ulong)local_e0,uVar7,uVar8);
      }
      if (local_e0 == 0) {
        FUN_001075b5(param_1,param_2,&local_d8);
      }
      if (*(short *)(param_2 + 0x68) == 6) {
        DAT_001499e0 = 0;
      }
    }
    else {
      FUN_0011fc0f(param_1,param_2,1);
    }
  }
LAB_00108017:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: find @ 0x803a

undefined8 find(char *param_1)

{
  int iVar1;
  size_t sVar2;
  long lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  int *piVar6;
  long lVar7;
  long in_FS_OFFSET;
  int local_4c;
  char *local_38;
  undefined8 local_30;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  sVar2 = strlen(param_1);
  DAT_001499dc = (undefined4)sVar2;
  FUN_001073f4(0xffffff9c);
  local_30 = 0;
  if (DAT_00149988 == 2) {
    DAT_00149010 = DAT_00149010 | 0x11;
  }
  else if (DAT_00149988 < 3) {
    if (DAT_00149988 == 0) {
      DAT_00149010 = DAT_00149010 | 0x10;
    }
    else if (DAT_00149988 == 1) {
      DAT_00149010 = DAT_00149010 | 3;
    }
  }
  if (DAT_0014994d != '\0') {
    DAT_00149010 = DAT_00149010 | 0x40;
  }
  local_38 = param_1;
  lVar3 = FUN_0011ec55(&local_38,DAT_00149010,0);
  if (lVar3 == 0) {
    uVar4 = FUN_0011096a(0,param_1);
    uVar5 = gettext("cannot search %s");
    piVar6 = __errno_location();
    error(0,*piVar6,uVar5,uVar4);
    DAT_001499e4 = 1;
  }
  else {
    local_4c = -0x80000000;
    while( true ) {
      piVar6 = __errno_location();
      *piVar6 = 0;
      lVar7 = FUN_0011f599(lVar3);
      if (lVar7 == 0) break;
      if ((DAT_001499e8 != '\0') && (local_4c != (int)*(undefined8 *)(lVar7 + 0x58))) {
        FUN_0010f6cb();
      }
      local_4c = (int)*(undefined8 *)(lVar7 + 0x58);
      DAT_001499f8 = 0;
      DAT_001499c4 = 0;
      DAT_001499c5 = *(int *)(lVar7 + 0x88) != 0;
      if ((bool)DAT_001499c5) {
        DAT_001499c8 = *(undefined4 *)(lVar7 + 0x88);
      }
      else {
        DAT_001499c8 = 0;
      }
      FUN_00107852(lVar3,lVar7);
    }
    piVar6 = __errno_location();
    if (*piVar6 != 0) {
      uVar4 = FUN_0011096a(0,param_1);
      piVar6 = __errno_location();
      error(0,*piVar6,"failed to read file names from file system at or below %s",uVar4);
      DAT_001499e4 = 1;
      uVar4 = 0;
      goto LAB_001082ca;
    }
    iVar1 = FUN_0011f0df(lVar3);
    if (iVar1 != 0) {
      uVar4 = gettext("failed to restore working directory after searching %s");
      piVar6 = __errno_location();
      error(0,*piVar6,uVar4,param_1);
      DAT_001499e4 = 1;
      uVar4 = 0;
      goto LAB_001082ca;
    }
  }
  uVar4 = 1;
LAB_001082ca:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: process_all_startpoints @ 0x82e7

ulong process_all_startpoints(int param_1,undefined8 *param_2)

{
  char cVar1;
  bool bVar2;
  int iVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  ulong uVar6;
  int *piVar7;
  size_t sVar8;
  long in_FS_OFFSET;
  byte local_17a;
  uint local_178;
  int local_174;
  FILE *local_170;
  undefined8 local_168;
  long local_160;
  char *local_158;
  undefined8 local_150;
  stat local_148;
  stat local_b8;
  undefined2 local_22;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if ((param_1 < 1) || (cVar1 = FUN_0010ff96(*param_2,1), cVar1 == '\x01')) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  local_170 = (FILE *)0x0;
  local_168 = 0;
  if (DAT_001499b0 == (char *)0x0) {
    if (!bVar2) {
      local_22 = 0x2e;
      uVar6 = FUN_0010803a(&local_22);
      goto LAB_001088d3;
    }
    local_160 = FUN_0011d0aa(param_2);
  }
  else {
    if (bVar2) {
      uVar4 = FUN_0011096a(0,*param_2);
      uVar5 = gettext("extra operand %s");
      error(0,0,uVar5,uVar4);
      uVar4 = gettext("file operands cannot be combined with -files0-from");
      error(1,0,&DAT_001383f3,uVar4);
    }
    iVar3 = strcmp(DAT_001499b0,"-");
    if (iVar3 == 0) {
      if (DAT_001499b8 != '\0') {
        uVar4 = gettext(
                       "option -files0-from reading from standard input cannot be combined with -ok, -okdir"
                       );
        error(1,0,&DAT_0013844c,uVar4);
      }
      uVar4 = gettext("(standard input)");
      local_168 = FUN_0011096a(0,uVar4);
      local_170 = stdin;
    }
    else {
      local_168 = FUN_0011096a(0,DAT_001499b0);
      local_170 = fopen(DAT_001499b0,"r");
      if (local_170 == (FILE *)0x0) {
        uVar4 = gettext("cannot open %s for reading");
        piVar7 = __errno_location();
        error(1,*piVar7,uVar4,local_168);
      }
      local_174 = fileno(local_170);
      if (local_174 < 0) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("fd >= 0","ftsfind.c",0x262,"process_all_startpoints");
      }
      if ((((DAT_001499b8 != '\0') && (iVar3 = fstat(local_174,&local_148), iVar3 == 0)) &&
          (iVar3 = fstat(0,&local_b8), iVar3 == 0)) &&
         ((local_148.st_ino == local_b8.st_ino && (local_148.st_dev == local_b8.st_dev)))) {
        uVar4 = gettext(
                       "option -files0-from: standard input must not refer to the same file when combined with -ok, -okdir"
                       );
        error(1,0,"%s: %s\n",uVar4,local_168);
      }
      FUN_0011d5ba(local_174,1);
    }
    local_160 = FUN_0011d0ff(local_170);
  }
  if (local_160 == 0) {
    FUN_001321fa();
  }
  local_17a = 1;
  do {
    while( true ) {
      local_158 = (char *)FUN_0011d16c(local_160,&local_178);
      if (local_158 == (char *)0x0) {
        if (local_178 == 4) {
          uVar4 = gettext("%s: read error");
          piVar7 = __errno_location();
          error(0,*piVar7,uVar4,local_168);
          DAT_001499e4 = 1;
          local_17a = 0;
          goto LAB_00108852;
        }
        if (local_178 < 5) {
          if (local_178 == 2) goto LAB_00108852;
          if (local_178 == 3) {
            FUN_001321fa();
          }
        }
                    /* WARNING: Subroutine does not return */
        __assert_fail("!\"unexpected error code from argv_iter\"","ftsfind.c",0x2a3,
                      "process_all_startpoints");
      }
      if (*local_158 != '\0') break;
      if (DAT_001499b0 == (char *)0x0) {
        uVar4 = FUN_0011096a(0,local_158);
        error(0,2,&DAT_001383f3,uVar4);
      }
      else {
        local_150 = FUN_0011d257(local_160);
        uVar4 = gettext("invalid zero-length file name");
        error(0,0,"%s:%lu: %s",local_168,local_150,uVar4);
      }
      DAT_001499e4 = 1;
      local_17a = 0;
    }
    if ((DAT_001499b0 == (char *)0x0) && (cVar1 = FUN_0010ff96(local_158,1), cVar1 != '\0'))
    goto LAB_00108852;
    sVar8 = strlen(local_158);
    DAT_001499dc = (undefined4)sVar8;
    cVar1 = FUN_0010803a(local_158);
  } while (cVar1 == '\x01');
  local_17a = 0;
LAB_00108852:
  FUN_0011d295(local_160);
  if (((local_17a != 0) && (DAT_001499b0 != (char *)0x0)) &&
     ((iVar3 = ferror(local_170), iVar3 != 0 || (iVar3 = FUN_0011dad5(local_170), iVar3 != 0)))) {
    uVar4 = gettext("error reading %s");
    error(1,0,uVar4,local_168);
  }
  uVar6 = (ulong)local_17a;
LAB_001088d3:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar6;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: main @ 0x88ed

undefined4 main(int param_1,long *param_2)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  int *piVar4;
  char *pcVar5;
  
  if (*param_2 == 0) {
    FUN_0012dd35(&DAT_0013856a);
  }
  else {
    FUN_0012dd35(*param_2);
  }
  FUN_0010f78b();
  DAT_001499f8 = 0;
  DAT_001499e4 = 0;
  DAT_001499e8 = 0;
  DAT_001499d8 = 0xffffff9c;
  cVar1 = FUN_0011b5f4();
  if (cVar1 != '\0') {
    FUN_0011b20a();
  }
  DAT_001499f0 = FUN_00110be9(&DAT_0013856f);
  if (DAT_001499f0 == 0) {
    uVar3 = gettext("Failed to initialize shared-file hash table");
    piVar4 = __errno_location();
    error(1,*piVar4,uVar3);
  }
  FUN_001106bd(&DAT_00149940);
  setlocale(6,"");
  bindtextdomain("findutils","/usr/local/share/locale");
  textdomain("findutils");
  iVar2 = FUN_00137150(FUN_0011d697);
  if (iVar2 != 0) {
    uVar3 = gettext("The atexit library function failed");
    piVar4 = __errno_location();
    error(1,*piVar4,uVar3);
  }
  iVar2 = FUN_001103e8(param_1,param_2);
  if ((DAT_00149980 & 2) != 0) {
    DAT_00149990 = FUN_0010fd39;
  }
  if ((DAT_00149980 & 0x80) != 0) {
    pcVar5 = ctime(&DAT_00149968);
    fprintf(stderr,"cur_day_start = %s",pcVar5);
  }
  uVar3 = FUN_0010dd20(param_1,param_2,iVar2);
  cVar1 = FUN_001082e7(param_1 - iVar2,param_2 + iVar2);
  if (cVar1 != '\0') {
    FUN_0010ae53(uVar3);
    FUN_0010fa38();
  }
  return DAT_001499e4;
}




// Function: is_fts_enabled @ 0x8b12

undefined8 is_fts_enabled(undefined4 *param_1)

{
  *param_1 = DAT_00149010;
  return 1;
}




// Function: ts_difference @ 0x8b31

double ts_difference(time_t param_1,long param_2,time_t param_3,long param_4)

{
  double dVar1;
  
  dVar1 = difftime(param_1,param_3);
  return dVar1 + DAT_001389e8 * (double)(param_2 - param_4);
}




// Function: compare_ts @ 0x8ba9

undefined8 compare_ts(long param_1,long param_2,long param_3,long param_4)

{
  undefined8 uVar1;
  double dVar2;
  
  if ((param_1 == param_3) && (param_2 == param_4)) {
    uVar1 = 0;
  }
  else {
    dVar2 = (double)FUN_00108b31(param_1,param_2,param_3,param_4);
    if (0.0 <= dVar2) {
      uVar1 = 1;
    }
    else {
      uVar1 = 0xffffffff;
    }
  }
  return uVar1;
}




// Function: pred_timewindow @ 0x8c32

uint pred_timewindow(undefined8 param_1,undefined8 param_2,long param_3,int param_4)

{
  int iVar1;
  uint uVar2;
  double dVar3;
  
  uVar2 = *(uint *)(param_3 + 0x3c);
  if (uVar2 == 2) {
    dVar3 = (double)FUN_00108b31(param_1,param_2,*(undefined8 *)(param_3 + 0x40),
                                 *(undefined8 *)(param_3 + 0x48));
    if ((dVar3 <= 0.0) || ((double)param_4 < dVar3)) {
      uVar2 = 0;
    }
    else {
      uVar2 = 1;
    }
  }
  else {
    if (2 < uVar2) {
LAB_00108d27:
                    /* WARNING: Subroutine does not return */
      __assert_fail("0","pred.c",0x8a,"pred_timewindow");
    }
    if (uVar2 == 0) {
      iVar1 = FUN_00108ba9(param_1,param_2,*(undefined8 *)(param_3 + 0x40),
                           *(undefined8 *)(param_3 + 0x48));
      uVar2 = (uint)(0 < iVar1);
    }
    else {
      if (uVar2 != 1) goto LAB_00108d27;
      uVar2 = FUN_00108ba9(param_1,param_2,*(undefined8 *)(param_3 + 0x40),
                           *(undefined8 *)(param_3 + 0x48));
      uVar2 = uVar2 >> 0x1f;
    }
  }
  return uVar2;
}




// Function: pred_amin @ 0x8d51

void pred_amin(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined1 auVar1 [16];
  
  auVar1 = FUN_00130a4c(param_2);
  FUN_00108c32(auVar1._0_8_,auVar1._8_8_,param_3,0x3c);
  return;
}




// Function: pred_and @ 0x8d9c

undefined8 pred_and(undefined8 param_1,undefined8 param_2,long param_3)

{
  char cVar1;
  undefined8 uVar2;
  
  if ((*(long *)(param_3 + 0x110) != 0) &&
     (cVar1 = FUN_00110873(param_1,param_2,*(undefined8 *)(param_3 + 0x110)), cVar1 == '\0')) {
    return 0;
  }
  uVar2 = FUN_00110873(param_1,param_2,*(undefined8 *)(param_3 + 0x118));
  return uVar2;
}




// Function: pred_anewer @ 0x8e0d

bool pred_anewer(undefined8 param_1,undefined8 param_2,long param_3)

{
  int iVar1;
  undefined1 auVar2 [16];
  
  if (*(int *)(param_3 + 0x3c) != 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0xa6,"pred_anewer");
  }
  auVar2 = FUN_00130a4c(param_2);
  iVar1 = FUN_00108ba9(auVar2._0_8_,auVar2._8_8_,*(undefined8 *)(param_3 + 0x40),
                       *(undefined8 *)(param_3 + 0x48));
  return 0 < iVar1;
}




// Function: pred_atime @ 0x8e96

void pred_atime(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined1 auVar1 [16];
  
  auVar1 = FUN_00130a4c(param_2);
  FUN_00108c32(auVar1._0_8_,auVar1._8_8_,param_3,0x15180);
  return;
}




// Function: pred_closeparen @ 0x8ee1

undefined8 pred_closeparen(void)

{
  return 1;
}




// Function: pred_cmin @ 0x8efc

void pred_cmin(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined1 auVar1 [16];
  
  auVar1 = FUN_00130a66(param_2);
  FUN_00108c32(auVar1._0_8_,auVar1._8_8_,param_3,0x3c);
  return;
}




// Function: pred_cnewer @ 0x8f47

bool pred_cnewer(undefined8 param_1,undefined8 param_2,long param_3)

{
  int iVar1;
  undefined1 auVar2 [16];
  
  if (*(int *)(param_3 + 0x3c) != 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",199,"pred_cnewer");
  }
  auVar2 = FUN_00130a66(param_2);
  iVar1 = FUN_00108ba9(auVar2._0_8_,auVar2._8_8_,*(undefined8 *)(param_3 + 0x40),
                       *(undefined8 *)(param_3 + 0x48));
  return 0 < iVar1;
}




// Function: pred_comma @ 0x8fd0

void pred_comma(undefined8 param_1,undefined8 param_2,long param_3)

{
  if (*(long *)(param_3 + 0x110) != 0) {
    FUN_00110873(param_1,param_2,*(undefined8 *)(param_3 + 0x110));
  }
  FUN_00110873(param_1,param_2,*(undefined8 *)(param_3 + 0x118));
  return;
}




// Function: pred_ctime @ 0x9036

void pred_ctime(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined1 auVar1 [16];
  
  auVar1 = FUN_00130a66(param_2);
  FUN_00108c32(auVar1._0_8_,auVar1._8_8_,param_3,0x15180);
  return;
}




// Function: perform_delete @ 0x9081

undefined8 perform_delete(int param_1)

{
  int iVar1;
  undefined4 extraout_var;
  
  iVar1 = unlinkat(DAT_001499d8,DAT_001499d0,param_1);
  return CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),iVar1 == 0);
}




// Function: pred_delete @ 0x90b1

undefined8 pred_delete(undefined8 param_1,long param_2)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  int *piVar4;
  undefined8 uVar5;
  int local_1c;
  
  iVar2 = strcmp(DAT_001499d0,".");
  if (iVar2 == 0) {
    uVar3 = 1;
  }
  else {
    local_1c = 0;
    if ((DAT_001499c4 != '\0') && ((*(uint *)(param_2 + 0x18) & 0xf000) == 0x4000)) {
      local_1c = 0x200;
    }
    cVar1 = FUN_00109081(local_1c);
    if (cVar1 == '\0') {
      piVar4 = __errno_location();
      if ((*piVar4 == 2) && (DAT_0014994e != '\0')) {
        piVar4 = __errno_location();
        *piVar4 = 0;
        uVar3 = 1;
      }
      else {
        piVar4 = __errno_location();
        if ((*piVar4 == 0x15) && ((local_1c == 0 && (cVar1 = FUN_00109081(0x200), cVar1 != '\0'))))
        {
          return 1;
        }
        uVar3 = FUN_0011096a(0,param_1);
        uVar5 = gettext("cannot delete %s");
        piVar4 = __errno_location();
        error(0,*piVar4,uVar5,uVar3);
        DAT_001499e4 = 1;
        uVar3 = 0;
      }
    }
    else {
      uVar3 = 1;
    }
  }
  return uVar3;
}




// Function: pred_empty @ 0x91f4

ulong pred_empty(undefined8 param_1,long param_2)

{
  int iVar1;
  int *piVar2;
  undefined8 uVar3;
  ulong uVar4;
  DIR *__dirp;
  byte local_2d;
  dirent *local_28;
  
  if ((*(uint *)(param_2 + 0x18) & 0xf000) == 0x4000) {
    local_2d = 1;
    piVar2 = __errno_location();
    *piVar2 = 0;
    iVar1 = FUN_0012704e(DAT_001499d8,DAT_001499d0,0x90900);
    if (iVar1 < 0) {
      uVar3 = FUN_0011096a(0,param_1);
      piVar2 = __errno_location();
      error(0,*piVar2,&DAT_0013869a,uVar3);
      DAT_001499e4 = 1;
      uVar4 = 0;
    }
    else {
      __dirp = fdopendir(iVar1);
      if (__dirp == (DIR *)0x0) {
        uVar3 = FUN_0011096a(0,param_1);
        piVar2 = __errno_location();
        error(0,*piVar2,&DAT_0013869a,uVar3);
        DAT_001499e4 = 1;
        close(iVar1);
        uVar4 = 0;
      }
      else {
        piVar2 = __errno_location();
        *piVar2 = 0;
        local_28 = readdir(__dirp);
        while (local_28 != (dirent *)0x0) {
          if ((local_28->d_name[0] != '.') ||
             ((local_28->d_name[1] != '\0' &&
              ((local_28->d_name[1] != '.' || (local_28->d_name[2] != '\0')))))) {
            local_2d = 0;
            break;
          }
          local_28 = readdir(__dirp);
        }
        piVar2 = __errno_location();
        if (*piVar2 == 0) {
          iVar1 = closedir(__dirp);
          if (iVar1 == 0) {
            uVar4 = (ulong)local_2d;
          }
          else {
            uVar3 = FUN_0011096a(0,param_1);
            piVar2 = __errno_location();
            error(0,*piVar2,&DAT_0013869a,uVar3);
            DAT_001499e4 = 1;
            uVar4 = 0;
          }
        }
        else {
          uVar3 = FUN_0011096a(0,param_1);
          piVar2 = __errno_location();
          error(0,*piVar2,&DAT_0013869a,uVar3);
          DAT_001499e4 = 1;
          closedir(__dirp);
          uVar4 = 0;
        }
      }
    }
  }
  else if ((*(uint *)(param_2 + 0x18) & 0xf000) == 0x8000) {
    uVar4 = CONCAT71((int7)((ulong)*(long *)(param_2 + 0x30) >> 8),*(long *)(param_2 + 0x30) == 0);
  }
  else {
    uVar4 = 0;
  }
  return uVar4;
}




// Function: pred_exec @ 0x9466

void pred_exec(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0010b300(param_1,param_2,param_3);
  return;
}




// Function: pred_execdir @ 0x9497

void pred_execdir(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0010b300(DAT_001499d0,param_2,param_3);
  return;
}




// Function: pred_false @ 0x94cb

undefined8 pred_false(void)

{
  return 0;
}




// Function: pred_fls @ 0x94e6

undefined8 pred_fls(undefined8 param_1,undefined8 param_2,long param_3)

{
  FUN_0011b7b3(param_1,DAT_001499d8,DAT_001499d0,param_2,DAT_00149958,DAT_0014997c,
               *(undefined1 *)(param_3 + 0x28),*(undefined8 *)(param_3 + 0x40));
  return 1;
}




// Function: pred_fprint @ 0x954f

undefined8 pred_fprint(undefined8 param_1,undefined8 param_2,long param_3)

{
  FUN_0011c643(*(undefined8 *)(param_3 + 0x40),*(undefined8 *)(param_3 + 0x58),
               *(undefined1 *)(param_3 + 0x50),&DAT_0013869d,param_1);
  return 1;
}




// Function: pred_fprint0 @ 0x959f

undefined8 pred_fprint0(char *param_1,undefined8 param_2,long param_3)

{
  FILE *__stream;
  
  __stream = *(FILE **)(param_3 + 0x40);
  fputs(param_1,__stream);
  putc(0,__stream);
  return 1;
}




// Function: pred_fstype @ 0x95ee

bool pred_fstype(undefined8 param_1,undefined8 param_2,long param_3)

{
  char *__s2;
  int iVar1;
  char *__s1;
  
  __s2 = *(char **)(param_3 + 0x38);
  __s1 = (char *)FUN_00113a18(param_2,param_1);
  iVar1 = strcmp(__s1,__s2);
  return iVar1 == 0;
}




// Function: pred_gid @ 0x9643

undefined8 pred_gid(undefined8 param_1,long param_2,long param_3)

{
  uint uVar1;
  
  uVar1 = *(uint *)(param_3 + 0x38);
  if (uVar1 == 2) {
    if ((ulong)*(uint *)(param_2 + 0x20) == *(ulong *)(param_3 + 0x40)) {
      return 1;
    }
  }
  else if (uVar1 < 3) {
    if (uVar1 == 0) {
      if (*(ulong *)(param_3 + 0x40) < (ulong)*(uint *)(param_2 + 0x20)) {
        return 1;
      }
    }
    else if ((uVar1 == 1) && ((ulong)*(uint *)(param_2 + 0x20) < *(ulong *)(param_3 + 0x40))) {
      return 1;
    }
  }
  return 0;
}




// Function: pred_group @ 0x96d8

bool pred_group(undefined8 param_1,long param_2,long param_3)

{
  return *(int *)(param_3 + 0x38) == *(int *)(param_2 + 0x20);
}




// Function: pred_ilname @ 0x970c

void pred_ilname(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_00109975(param_1,param_2,param_3,1);
  return;
}




// Function: pred_name_common @ 0x973f

bool pred_name_common(undefined8 param_1,char *param_2,int param_3)

{
  int iVar1;
  char *__name;
  
  __name = (char *)FUN_0011d85c(param_1);
  FUN_0011da6f(__name);
  iVar1 = fnmatch(param_2,__name,param_3);
  free(__name);
  return iVar1 == 0;
}




// Function: pred_iname @ 0x97a2

void pred_iname(undefined8 param_1,undefined8 param_2,long param_3)

{
  FUN_0010973f(param_1,*(undefined8 *)(param_3 + 0x38),0x10);
  return;
}




// Function: pred_inum @ 0x97d8

undefined8 pred_inum(undefined8 param_1,long param_2,long param_3)

{
  uint uVar1;
  
  uVar1 = *(uint *)(param_3 + 0x38);
  if (uVar1 == 2) {
    if (*(long *)(param_2 + 8) == *(long *)(param_3 + 0x40)) {
      return 1;
    }
  }
  else if (uVar1 < 3) {
    if (uVar1 == 0) {
      if (*(ulong *)(param_3 + 0x40) < *(ulong *)(param_2 + 8)) {
        return 1;
      }
    }
    else if ((uVar1 == 1) && (*(ulong *)(param_2 + 8) < *(ulong *)(param_3 + 0x40))) {
      return 1;
    }
  }
  return 0;
}




// Function: pred_ipath @ 0x986a

bool pred_ipath(char *param_1,undefined8 param_2,long param_3)

{
  int iVar1;
  
  iVar1 = fnmatch(*(char **)(param_3 + 0x38),param_1,0x10);
  return iVar1 == 0;
}




// Function: pred_links @ 0x98b0

undefined8 pred_links(undefined8 param_1,long param_2,long param_3)

{
  uint uVar1;
  
  uVar1 = *(uint *)(param_3 + 0x38);
  if (uVar1 == 2) {
    if (*(long *)(param_2 + 0x10) == *(long *)(param_3 + 0x40)) {
      return 1;
    }
  }
  else if (uVar1 < 3) {
    if (uVar1 == 0) {
      if (*(ulong *)(param_3 + 0x40) < *(ulong *)(param_2 + 0x10)) {
        return 1;
      }
    }
    else if ((uVar1 == 1) && (*(ulong *)(param_2 + 0x10) < *(ulong *)(param_3 + 0x40))) {
      return 1;
    }
  }
  return 0;
}




// Function: pred_lname @ 0x9942

void pred_lname(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_00109975(param_1,param_2,param_3,0);
  return;
}




// Function: match_lname @ 0x9975

undefined1 match_lname(undefined8 param_1,long param_2,long param_3,char param_4)

{
  int iVar1;
  char *__name;
  int *piVar2;
  undefined1 local_11;
  
  local_11 = 0;
  if ((*(uint *)(param_2 + 0x18) & 0xf000) == 0xa000) {
    __name = (char *)FUN_0011d06d(DAT_001499d8,DAT_001499d0);
    if (__name == (char *)0x0) {
      piVar2 = __errno_location();
      FUN_00110a13(*piVar2,param_1);
      DAT_001499e4 = 1;
    }
    else {
      if (param_4 == '\0') {
        iVar1 = 0;
      }
      else {
        iVar1 = 0x10;
      }
      iVar1 = fnmatch(*(char **)(param_3 + 0x38),__name,iVar1);
      if (iVar1 == 0) {
        local_11 = 1;
      }
    }
    free(__name);
  }
  return local_11;
}




// Function: pred_ls @ 0x9a33

void pred_ls(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_001094e6(param_1,param_2,param_3);
  return;
}




// Function: pred_mmin @ 0x9a64

void pred_mmin(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined1 auVar1 [16];
  
  auVar1 = FUN_00130a80(param_2);
  FUN_00108c32(auVar1._0_8_,auVar1._8_8_,param_3,0x3c);
  return;
}




// Function: pred_mtime @ 0x9aaf

void pred_mtime(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined1 auVar1 [16];
  
  auVar1 = FUN_00130a80(param_2);
  FUN_00108c32(auVar1._0_8_,auVar1._8_8_,param_3,0x15180);
  return;
}




// Function: pred_name @ 0x9afa

void pred_name(undefined8 param_1,undefined8 param_2,long param_3)

{
  FUN_0010973f(param_1,*(undefined8 *)(param_3 + 0x38),0);
  return;
}




// Function: pred_negate @ 0x9b30

bool pred_negate(undefined8 param_1,undefined8 param_2,long param_3)

{
  char cVar1;
  
  cVar1 = FUN_00110873(param_1,param_2,*(undefined8 *)(param_3 + 0x118));
  return cVar1 == '\0';
}




// Function: pred_newer @ 0x9b79

bool pred_newer(undefined8 param_1,undefined8 param_2,long param_3)

{
  int iVar1;
  undefined1 auVar2 [16];
  
  if (*(int *)(param_3 + 0x3c) != 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0x26d,"pred_newer");
  }
  auVar2 = FUN_00130a80(param_2);
  iVar1 = FUN_00108ba9(auVar2._0_8_,auVar2._8_8_,*(undefined8 *)(param_3 + 0x40),
                       *(undefined8 *)(param_3 + 0x48));
  return 0 < iVar1;
}




// Function: pred_newerXY @ 0x9c02

bool pred_newerXY(undefined8 param_1,undefined8 param_2,long param_3)

{
  bool bVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined1 auVar5 [16];
  undefined8 local_28;
  undefined8 local_20;
  
  auVar5._8_8_ = local_20;
  auVar5._0_8_ = local_28;
  bVar1 = false;
  if (*(int *)(param_3 + 0x3c) != 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0x277,"pred_newerXY");
  }
  switch(*(undefined4 *)(param_3 + 0x38)) {
  case 0:
    auVar5 = FUN_00130a4c(param_2);
    bVar1 = true;
    break;
  case 1:
    auVar5 = FUN_00130a9a(param_2);
    bVar1 = true;
    if (auVar5._8_8_ < 0) {
      uVar3 = FUN_0011096a(0,param_1);
      uVar4 = gettext("WARNING: cannot determine birth time of file %s");
      error(0,0,uVar4,uVar3);
      return false;
    }
    break;
  case 2:
    auVar5 = FUN_00130a66(param_2);
    bVar1 = true;
    break;
  case 3:
    auVar5 = FUN_00130a80(param_2);
    bVar1 = true;
    break;
  case 4:
    if (*(int *)(param_3 + 0x38) != 4) {
      return false;
    }
                    /* WARNING: Subroutine does not return */
    __assert_fail("pred_ptr->args.reftime.xval != XVAL_TIME","pred.c",0x27c,"pred_newerXY");
  }
  local_20 = auVar5._8_8_;
  local_28 = auVar5._0_8_;
  if (bVar1) {
    iVar2 = FUN_00108ba9(local_28,local_20,*(undefined8 *)(param_3 + 0x40),
                         *(undefined8 *)(param_3 + 0x48));
    return 0 < iVar2;
  }
                    /* WARNING: Subroutine does not return */
  __assert_fail("collected","pred.c",0x29b,"pred_newerXY");
}




// Function: pred_nogroup @ 0x9dd5

bool pred_nogroup(undefined8 param_1,long param_2)

{
  group *pgVar1;
  
  pgVar1 = getgrgid(*(__gid_t *)(param_2 + 0x20));
  return pgVar1 == (group *)0x0;
}




// Function: pred_nouser @ 0x9e03

bool pred_nouser(undefined8 param_1,long param_2)

{
  passwd *ppVar1;
  
  ppVar1 = getpwuid(*(__uid_t *)(param_2 + 0x1c));
  return ppVar1 == (passwd *)0x0;
}




// Function: is_ok @ 0x9e31

void is_ok(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  char *__format;
  undefined8 uVar2;
  int *piVar3;
  
  FUN_0011e1a8(stdout);
  __format = (char *)gettext("< %s ... %s > ? ");
  iVar1 = fprintf(stderr,__format,param_1,param_2);
  if (iVar1 < 0) {
    uVar2 = gettext("Failed to write prompt for -ok");
    piVar3 = __errno_location();
    error(1,*piVar3,uVar2);
  }
  FUN_0011e1a8(stderr);
  FUN_00132d7c();
  return;
}




// Function: pred_ok @ 0x9ece

undefined8 pred_ok(undefined8 param_1,undefined8 param_2,long param_3)

{
  char cVar1;
  undefined8 uVar2;
  
  cVar1 = FUN_00109e31(**(undefined8 **)(param_3 + 0xe8),param_1);
  if (cVar1 == '\0') {
    uVar2 = 0;
  }
  else {
    uVar2 = FUN_0010b300(param_1,param_2,param_3);
  }
  return uVar2;
}




// Function: pred_okdir @ 0x9f27

undefined8 pred_okdir(undefined8 param_1,undefined8 param_2,long param_3)

{
  char cVar1;
  undefined8 uVar2;
  
  cVar1 = FUN_00109e31(**(undefined8 **)(param_3 + 0xe8),param_1);
  if (cVar1 == '\0') {
    uVar2 = 0;
  }
  else {
    uVar2 = FUN_0010b300(DAT_001499d0,param_2,param_3);
  }
  return uVar2;
}




// Function: pred_openparen @ 0x9f83

undefined8 pred_openparen(void)

{
  return 1;
}




// Function: pred_or @ 0x9f9e

undefined8 pred_or(undefined8 param_1,undefined8 param_2,long param_3)

{
  char cVar1;
  undefined8 uVar2;
  
  if ((*(long *)(param_3 + 0x110) != 0) &&
     (cVar1 = FUN_00110873(param_1,param_2,*(undefined8 *)(param_3 + 0x110)), cVar1 == '\x01')) {
    return 1;
  }
  uVar2 = FUN_00110873(param_1,param_2,*(undefined8 *)(param_3 + 0x118));
  return uVar2;
}




// Function: pred_path @ 0xa012

bool pred_path(char *param_1,undefined8 param_2,long param_3)

{
  int iVar1;
  
  iVar1 = fnmatch(*(char **)(param_3 + 0x38),param_1,0);
  return iVar1 == 0;
}




// Function: pred_perm @ 0xa058

undefined4 pred_perm(undefined8 param_1,long param_2,long param_3)

{
  uint uVar1;
  uint uVar2;
  uint uVar3;
  undefined4 uVar4;
  
  uVar1 = *(uint *)(param_2 + 0x18);
  uVar2 = *(uint *)(param_3 + 0xc + ((long)(int)(uint)((uVar1 & 0xf000) == 0x4000) + 0xc) * 4);
  uVar3 = *(uint *)(param_3 + 0x38);
  if (uVar3 == 2) {
    uVar4 = CONCAT31((int3)((uVar1 & 0xfff) >> 8),uVar2 == (uVar1 & 0xfff));
  }
  else {
    if (2 < uVar3) {
LAB_0010a0f3:
                    /* WARNING: Subroutine does not return */
      abort();
    }
    if (uVar3 == 0) {
      uVar4 = CONCAT31((int3)((uVar1 & uVar2) >> 8),uVar2 == (uVar1 & uVar2));
    }
    else {
      if (uVar3 != 1) goto LAB_0010a0f3;
      if (uVar2 == 0) {
        uVar4 = 1;
      }
      else {
        uVar4 = CONCAT31((int3)((uVar1 & uVar2) >> 8),(uVar1 & uVar2) != 0);
      }
    }
  }
  return uVar4;
}




// Function: pred_executable @ 0xa0fa

undefined8 pred_executable(void)

{
  int iVar1;
  undefined4 extraout_var;
  
  iVar1 = faccessat(DAT_001499d8,DAT_001499d0,1,0);
  return CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),iVar1 == 0);
}




// Function: pred_readable @ 0xa137

undefined8 pred_readable(void)

{
  int iVar1;
  undefined4 extraout_var;
  
  iVar1 = faccessat(DAT_001499d8,DAT_001499d0,4,0);
  return CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),iVar1 == 0);
}




// Function: pred_writable @ 0xa174

undefined8 pred_writable(void)

{
  int iVar1;
  undefined4 extraout_var;
  
  iVar1 = faccessat(DAT_001499d8,DAT_001499d0,2,0);
  return CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),iVar1 == 0);
}




// Function: pred_print @ 0xa1b1

undefined8 pred_print(undefined8 param_1,undefined8 param_2,long param_3)

{
  FUN_0011c643(*(undefined8 *)(param_3 + 0x40),*(undefined8 *)(param_3 + 0x58),
               *(undefined1 *)(param_3 + 0x50),&DAT_0013869d,param_1);
  return 1;
}




// Function: pred_print0 @ 0xa201

void pred_print0(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0010959f(param_1,param_2,param_3);
  return;
}




// Function: pred_prune @ 0xa232

undefined8 pred_prune(undefined8 param_1,long param_2)

{
  if (DAT_00149940 != '\0') {
    if (DAT_001499c4 == '\0') {
                    /* WARNING: Subroutine does not return */
      __assert_fail("state.have_stat","pred.c",0x355,"pred_prune");
    }
    if ((param_2 != 0) && ((*(uint *)(param_2 + 0x18) & 0xf000) == 0x4000)) {
      DAT_001499e0 = 1;
    }
  }
  return 1;
}




// Function: pred_quit @ 0xa2b0

void pred_quit(void)

{
  FUN_0010fa38();
                    /* WARNING: Subroutine does not return */
  exit(DAT_001499e4);
}




// Function: pred_regex @ 0xa2da

bool pred_regex(char *param_1,undefined8 param_2,long param_3)

{
  int iVar1;
  size_t sVar2;
  
  sVar2 = strlen(param_1);
  iVar1 = re_match(*(re_pattern_buffer **)(param_3 + 0x38),param_1,(int)sVar2,0,(re_registers *)0x0)
  ;
  return (int)sVar2 == iVar1;
}




// Function: pred_size @ 0xa336

undefined8 pred_size(undefined8 param_1,long param_2,long param_3)

{
  uint uVar1;
  ulong uVar2;
  
  uVar2 = (ulong)(*(long *)(param_2 + 0x30) % (long)*(int *)(param_3 + 0x3c) != 0) +
          *(long *)(param_2 + 0x30) / (long)*(int *)(param_3 + 0x3c);
  uVar1 = *(uint *)(param_3 + 0x38);
  if (uVar1 == 2) {
    if (uVar2 == *(ulong *)(param_3 + 0x40)) {
      return 1;
    }
  }
  else if (uVar1 < 3) {
    if (uVar1 == 0) {
      if (*(ulong *)(param_3 + 0x40) < uVar2) {
        return 1;
      }
    }
    else if ((uVar1 == 1) && (uVar2 < *(ulong *)(param_3 + 0x40))) {
      return 1;
    }
  }
  return 0;
}




// Function: pred_samefile @ 0xa3f7

undefined4 pred_samefile(undefined8 param_1,long *param_2,long param_3)

{
  int iVar1;
  undefined4 uVar2;
  
  if ((param_2[1] == 0) || (param_2[1] == *(long *)(param_3 + 0x38))) {
    iVar1 = FUN_0010f27d(param_1,DAT_001499d0,param_2);
    if (iVar1 == 0) {
      if ((param_2[1] == *(long *)(param_3 + 0x38)) && (*param_2 == *(long *)(param_3 + 0x40))) {
        uVar2 = 1;
      }
      else {
        uVar2 = 0;
      }
    }
    else {
      uVar2 = 0;
    }
  }
  else {
    uVar2 = 0;
  }
  return uVar2;
}




// Function: pred_true @ 0xa497

undefined8 pred_true(void)

{
  return 1;
}




// Function: pred_type @ 0xa4b2

undefined8 pred_type(undefined8 param_1,long param_2,long param_3)

{
  undefined8 uVar1;
  uint local_10;
  uint local_c;
  
  local_c = 7;
  if (DAT_001499c5 != '\0') {
    if (DAT_001499c8 == 0) {
      uVar1 = 0;
    }
    else {
      if (DAT_001499c4 == '\0') {
        local_10 = DAT_001499c8;
      }
      else {
        local_10 = *(uint *)(param_2 + 0x18);
      }
      local_10 = local_10 & 0xf000;
      if (local_10 == 0xc000) {
        local_c = 6;
      }
      else if (local_10 < 0xc001) {
        if (local_10 == 0xa000) {
          local_c = 4;
        }
        else if (local_10 < 0xa001) {
          if (local_10 == 0x8000) {
            local_c = 3;
          }
          else if (local_10 < 0x8001) {
            if (local_10 == 0x6000) {
              local_c = 0;
            }
            else if (local_10 < 0x6001) {
              if (local_10 == 0x4000) {
                local_c = 2;
              }
              else if (local_10 < 0x4001) {
                if (local_10 == 0x1000) {
                  local_c = 5;
                }
                else if (local_10 == 0x2000) {
                  local_c = 1;
                }
              }
            }
          }
        }
      }
      if ((local_c == 7) || (*(char *)(param_3 + 0x38 + (ulong)local_c) == '\0')) {
        uVar1 = 0;
      }
      else {
        uVar1 = 1;
      }
    }
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __assert_fail("state.have_type","pred.c",0x3d1,"pred_type");
}




// Function: pred_uid @ 0xa600

undefined8 pred_uid(undefined8 param_1,long param_2,long param_3)

{
  uint uVar1;
  
  uVar1 = *(uint *)(param_3 + 0x38);
  if (uVar1 == 2) {
    if ((ulong)*(uint *)(param_2 + 0x1c) == *(ulong *)(param_3 + 0x40)) {
      return 1;
    }
  }
  else if (uVar1 < 3) {
    if (uVar1 == 0) {
      if (*(ulong *)(param_3 + 0x40) < (ulong)*(uint *)(param_2 + 0x1c)) {
        return 1;
      }
    }
    else if ((uVar1 == 1) && ((ulong)*(uint *)(param_2 + 0x1c) < *(ulong *)(param_3 + 0x40))) {
      return 1;
    }
  }
  return 0;
}




// Function: pred_used @ 0xa695

undefined8 pred_used(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  int iVar1;
  undefined8 uVar2;
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  undefined8 local_38;
  undefined8 local_30;
  
  auVar3 = FUN_00130a4c(param_2);
  auVar4 = FUN_00130a66(param_2);
  iVar1 = FUN_00108ba9(auVar3._0_8_,auVar3._8_8_,auVar4._0_8_,auVar4._8_8_);
  if (iVar1 < 0) {
    uVar2 = 0;
  }
  else {
    local_38 = auVar4._0_8_ - auVar3._0_8_;
    local_30 = auVar4._8_8_ - auVar3._8_8_;
    if (local_30 < 0) {
      local_30 = local_30 + 1000000000;
      local_38 = local_38 + -1;
    }
    uVar2 = FUN_00108c32(local_38,local_30,param_3,0x15180);
  }
  return uVar2;
}




// Function: pred_user @ 0xa75a

bool pred_user(undefined8 param_1,long param_2,long param_3)

{
  return *(int *)(param_3 + 0x38) == *(int *)(param_2 + 0x1c);
}




// Function: pred_xtype @ 0xa78e

undefined8 pred_xtype(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  code *local_c0;
  undefined1 local_b8 [152];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0010fe03();
  if (cVar1 == '\0') {
    local_c0 = FUN_0010fc21;
  }
  else {
    local_c0 = FUN_0010fcc0;
  }
  FUN_0010f26e(local_b8);
  iVar2 = (*local_c0)(DAT_001499d0,local_b8);
  if (iVar2 == 0) {
    uVar4 = FUN_0010a4b2(param_1,local_b8,param_3);
  }
  else {
    cVar1 = FUN_0010fe03();
    if (cVar1 != '\0') {
      piVar3 = __errno_location();
      if (*piVar3 == 2) {
        uVar4 = FUN_0010a4b2(param_1,param_2,param_3);
        goto LAB_0010a8be;
      }
    }
    uVar4 = FUN_0011096a(0,param_1);
    piVar3 = __errno_location();
    error(0,*piVar3,&DAT_0013869a,uVar4);
    DAT_001499e4 = 1;
    uVar4 = 0;
  }
LAB_0010a8be:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar4;
}




// Function: pred_context @ 0xa8d8

bool pred_context(undefined8 param_1,undefined8 param_2,long param_3)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  int *piVar4;
  long in_FS_OFFSET;
  bool bVar5;
  char *local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = (*DAT_001499a0)(DAT_001499d8,DAT_001499d0,&local_28);
  if (iVar1 < 0) {
    uVar2 = FUN_0011096a(0,param_1);
    uVar3 = gettext("getfilecon failed: %s");
    piVar4 = __errno_location();
    error(0,*piVar4,uVar3,uVar2);
    bVar5 = false;
  }
  else {
    iVar1 = fnmatch(*(char **)(param_3 + 0x38),local_28,0);
    bVar5 = iVar1 == 0;
    freecon(local_28);
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return bVar5;
}




// Function: blank_rtrim @ 0xa9cd

char * blank_rtrim(char *param_1,char *param_2)

{
  int iVar1;
  size_t sVar2;
  int local_c;
  
  if (param_1 == (char *)0x0) {
    param_2 = (char *)0x0;
  }
  else {
    strcpy(param_2,param_1);
    sVar2 = strlen(param_2);
    local_c = (int)sVar2;
    do {
      iVar1 = local_c;
      local_c = iVar1 + -1;
      if (local_c < 0) break;
    } while ((param_2[local_c] == ' ') || (param_2[local_c] == '\t'));
    param_2[iVar1] = '\0';
  }
  return param_2;
}




// Function: print_list @ 0xaa62

void print_list(FILE *param_1,long param_2)

{
  undefined8 uVar1;
  long in_FS_OFFSET;
  long local_120;
  undefined1 local_118 [264];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  for (local_120 = param_2; local_120 != 0; local_120 = *(long *)(local_120 + 0x108)) {
    uVar1 = FUN_0010a9cd(*(undefined8 *)(local_120 + 8),local_118);
    fprintf(param_1,"[%s] ",uVar1);
  }
  fprintf(param_1,"\n");
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: print_parenthesised @ 0xab30

void print_parenthesised(FILE *param_1,long *param_2)

{
  bool bVar1;
  
  bVar1 = false;
  if (param_2 != (long *)0x0) {
    if ((((code *)*param_2 == FUN_00109f9e) || ((code *)*param_2 == FUN_00108d9c)) &&
       (param_2[0x22] == 0)) {
      print_parenthesised(param_1,param_2[0x23]);
    }
    else {
      if ((param_2[0x22] != 0) || (param_2[0x23] != 0)) {
        bVar1 = true;
      }
      if (bVar1) {
        fprintf(param_1,"%s",&DAT_0013879d);
      }
      FUN_0010ac38(param_1,param_2);
      if (bVar1) {
        fprintf(param_1,"%s",&DAT_001387a1);
      }
    }
  }
  return;
}




// Function: print_optlist @ 0xac38

void print_optlist(FILE *param_1,long param_2)

{
  char *pcVar1;
  char *pcVar2;
  char *pcVar3;
  
  if (param_2 != 0) {
    FUN_0010ab30(param_1,*(undefined8 *)(param_2 + 0x110));
    if (*(char *)(param_2 + 0x1c) == '\0') {
      pcVar2 = "";
    }
    else {
      pcVar2 = "[need inum] ";
    }
    if (*(char *)(param_2 + 0x1b) == '\0') {
      pcVar3 = "";
    }
    else {
      pcVar3 = "[need type] ";
    }
    if (*(char *)(param_2 + 0x1a) == '\0') {
      pcVar1 = "";
    }
    else {
      pcVar1 = "[call stat] ";
    }
    fprintf(param_1,"%s%s%s",pcVar1,pcVar3,pcVar2);
    FUN_0010c2dc(param_1,param_2);
    fprintf(param_1," [est success rate %.4g] ",(double)*(float *)(param_2 + 0x24));
    if ((DAT_00149980 & 0x40) != 0) {
      fprintf(param_1,"[real success rate %lu/%lu",*(undefined8 *)(param_2 + 0x128),
              *(undefined8 *)(param_2 + 0x120));
      if (*(long *)(param_2 + 0x120) == 0) {
        fprintf(param_1,"=_] ");
      }
      else {
        fprintf(param_1,"=%.4g] ",
                (double)*(ulong *)(param_2 + 0x128) / (double)*(ulong *)(param_2 + 0x120));
      }
    }
    FUN_0010ab30(param_1,*(undefined8 *)(param_2 + 0x118));
  }
  return;
}




// Function: show_success_rates @ 0xae53

void show_success_rates(undefined8 param_1)

{
  if ((DAT_00149980 & 0x40) != 0) {
    fprintf(stderr,"Predicate success rates after completion:\n");
    FUN_0010ac38(stderr,param_1);
    fprintf(stderr,"\n");
  }
  return;
}




// Function: pred_sanity_check @ 0xaec7

void pred_sanity_check(long *param_1)

{
  long *local_10;
  
  local_10 = param_1;
  while( true ) {
    if (local_10 == (long *)0x0) {
      return;
    }
    if (*local_10 == 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("p->pred_func != NULL","pred.c",0x51e,"pred_sanity_check");
    }
    if (local_10[0x26] == 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("p->parser_entry != NULL","pred.c",0x521,"pred_sanity_check");
    }
    if ((*(long *)(local_10[0x26] + 0x18) != 0) && (*(long *)(local_10[0x26] + 0x18) != *local_10))
    break;
    switch(*(undefined4 *)local_10[0x26]) {
    case 0:
    case 2:
      if (*(int *)local_10[0x26] == 0) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("p->parser_entry->type != ARG_OPTION","pred.c",0x53a,"pred_sanity_check");
      }
      if (*(int *)local_10[0x26] == 2) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("p->parser_entry->type != ARG_POSITIONAL_OPTION","pred.c",0x53b,
                      "pred_sanity_check");
      }
      break;
    case 1:
    case 3:
    case 4:
    case 5:
      if (*(char *)((long)local_10 + 0x19) == '\x01') {
                    /* WARNING: Subroutine does not return */
        __assert_fail("!p->no_default_print","pred.c",0x553,"pred_sanity_check");
      }
      if ((char)local_10[3] == '\x01') {
                    /* WARNING: Subroutine does not return */
        __assert_fail("!p->side_effects","pred.c",0x554,"pred_sanity_check");
      }
      break;
    case 6:
      if ((char)local_10[3] == '\0') {
                    /* WARNING: Subroutine does not return */
        __assert_fail("p->side_effects","pred.c",0x53f,"pred_sanity_check");
      }
      if ((((code *)*local_10 != FUN_0010a232) && ((code *)*local_10 != FUN_0010a2b0)) &&
         (*(char *)((long)local_10 + 0x19) == '\0')) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("p->no_default_print","pred.c",0x545,"pred_sanity_check");
      }
    }
    local_10 = (long *)local_10[0x21];
  }
                    /* WARNING: Subroutine does not return */
  __assert_fail("p->parser_entry->pred_func == p->pred_func","pred.c",0x52a,"pred_sanity_check");
}




// Function: initialize_wd_for_exec @ 0xb184

bool initialize_wd_for_exec(long param_1,undefined4 param_2,undefined8 param_3)

{
  undefined4 *puVar1;
  undefined4 uVar2;
  undefined8 uVar3;
  bool bVar4;
  
  uVar3 = FUN_00131bc2(0x10);
  *(undefined8 *)(param_1 + 0xc0) = uVar3;
  *(undefined8 *)(*(long *)(param_1 + 0xc0) + 8) = 0;
  puVar1 = *(undefined4 **)(param_1 + 0xc0);
  uVar2 = FUN_0012704e(param_2,param_3,0);
  *puVar1 = uVar2;
  bVar4 = -1 < **(int **)(param_1 + 0xc0);
  if (bVar4) {
    FUN_0011d5ba(**(undefined4 **)(param_1 + 0xc0),1);
  }
  return bVar4;
}




// Function: record_exec_dir @ 0xb22b

ulong record_exec_dir(long param_1)

{
  byte bVar1;
  char *pcVar2;
  void *__ptr;
  ulong uVar3;
  
  if (*(int *)(param_1 + 0x98) == 0) {
    if (*(int *)(param_1 + 0x98) != 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("!execp->state.todo","exec.c",0x47,"record_exec_dir");
    }
    pcVar2 = strchr(DAT_001499d0,0x2f);
    if (pcVar2 == (char *)0x0) {
      uVar3 = FUN_0010b184(param_1,DAT_001499d8,&DAT_00138a0a);
    }
    else {
      __ptr = (void *)FUN_0011d9d4(DAT_001499d0);
      bVar1 = FUN_0010b184(param_1,DAT_001499d8,__ptr);
      free(__ptr);
      uVar3 = (ulong)bVar1;
    }
  }
  else {
    uVar3 = 1;
  }
  return uVar3;
}




// Function: impl_pred_exec @ 0xb300

undefined1 impl_pred_exec(char *param_1,undefined8 param_2,undefined8 *param_3)

{
  char cVar1;
  char cVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  int *piVar5;
  size_t sVar6;
  size_t sVar7;
  undefined1 local_46;
  int local_44;
  char *local_40;
  char *local_38;
  undefined *local_30;
  undefined8 local_28;
  
  local_40 = (char *)0x0;
  cVar1 = FUN_00110933(*param_3);
  if (cVar1 == '\0') {
    if (param_3[0x1f] != DAT_00149a00) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("execp->wd_for_exec == initial_wd","exec.c",0x94,"impl_pred_exec");
    }
    local_30 = (undefined *)0x0;
    local_28 = 0;
    local_38 = param_1;
  }
  else {
    cVar2 = FUN_0010b22b(param_3 + 7);
    if (cVar2 != '\x01') {
      uVar3 = FUN_0011096a(0,param_1);
      uVar4 = gettext("Failed to save working directory in order to run a command on %s");
      piVar5 = __errno_location();
      error(1,*piVar5,uVar4,uVar3);
    }
    local_40 = (char *)FUN_0011d85c(DAT_001499d0);
    local_38 = local_40;
    if (*local_40 == '/') {
      local_30 = (undefined *)0x0;
      local_28 = 0;
    }
    else {
      local_30 = &DAT_00138a51;
      local_28 = 2;
    }
  }
  if (*(char *)(param_3 + 7) == '\0') {
    for (local_44 = 0; local_44 < *(int *)(param_3 + 0x1e); local_44 = local_44 + 1) {
      sVar6 = strlen(local_38);
      sVar7 = strlen(*(char **)((long)local_44 * 8 + param_3[0x1d]));
      FUN_00119af7(param_3 + 8,param_3 + 0x13,*(undefined8 *)((long)local_44 * 8 + param_3[0x1d]),
                   sVar7,local_30,local_28,local_38,sVar6,0);
    }
    FUN_00119f5a(param_3 + 8,param_3 + 0x13);
    if ((*(uint *)(param_3 + 0x20) & 0x7f) == 0) {
      if ((*(int *)(param_3 + 0x20) >> 8 & 0xffU) == 0) {
        local_46 = 1;
      }
      else {
        local_46 = 0;
      }
    }
    else {
      local_46 = 0;
    }
    if (cVar1 != '\0') {
      FUN_0012fe52(param_3[0x1f]);
    }
  }
  else {
    sVar6 = strlen(local_38);
    FUN_0011a1b8(param_3 + 8,param_3 + 0x13,local_38,sVar6 + 1,local_30,local_28,0);
    if (*(int *)(param_3 + 0x1a) != 0) {
      DAT_001499e8 = 1;
    }
    local_46 = 1;
  }
  if (local_40 != (char *)0x0) {
    if (cVar1 == '\0') {
                    /* WARNING: Subroutine does not return */
      __assert_fail("local","exec.c",0xd0,"impl_pred_exec");
    }
    free(local_40);
  }
  return local_46;
}




// Function: prep_child_for_exec @ 0xb615

undefined1 prep_child_for_exec(char param_1,int *param_2)

{
  int iVar1;
  undefined8 uVar2;
  int *piVar3;
  undefined *puVar4;
  undefined *puVar5;
  long in_FS_OFFSET;
  undefined1 local_3b;
  undefined8 local_3a;
  undefined2 local_32;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_3b = 1;
  if (param_1 != '\0') {
    local_3a = 0x6c756e2f7665642f;
    local_32 = 0x6c;
    iVar1 = close(0);
    if (iVar1 < 0) {
      uVar2 = gettext("Cannot close standard input");
      piVar3 = __errno_location();
      error(0,*piVar3,uVar2);
      local_3b = 0;
    }
    else {
      iVar1 = FUN_0011e034(&local_3a,0);
      if (iVar1 < 0) {
        uVar2 = FUN_0011096a(0,&local_3a);
        piVar3 = __errno_location();
        error(0,*piVar3,&DAT_00138a9b,uVar2);
      }
    }
  }
  iVar1 = FUN_0012fe17(param_2);
  if (iVar1 != 0) {
    if ((*param_2 < 0) && (*(long *)(param_2 + 2) != 0)) {
      puVar5 = *(undefined **)(param_2 + 2);
    }
    else {
      puVar5 = &DAT_00138a9e;
    }
    if ((*param_2 < 0) && (*(long *)(param_2 + 2) != 0)) {
      puVar4 = &DAT_00138a9f;
    }
    else {
      puVar4 = &DAT_00138a9e;
    }
    uVar2 = gettext("Failed to change directory%s%s");
    piVar3 = __errno_location();
    error(0,*piVar3,uVar2,puVar4,puVar5);
    local_3b = 0;
  }
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_3b;
}




// Function: launch @ 0xb7ae

undefined8 launch(undefined8 param_1,char *param_2,undefined8 param_3,char **param_4)

{
  char cVar1;
  __pid_t __pid;
  __pid_t _Var2;
  uint uVar3;
  undefined8 uVar4;
  int *piVar5;
  undefined8 uVar6;
  int local_2c;
  
  if ((DAT_00149980 & 0x20) != 0) {
    fprintf(stderr,"DebugExec: launching process (argc=%lu):",*(long *)(param_2 + 0x60) + -1);
    for (local_2c = 0; (ulong)(long)local_2c < *(long *)(param_2 + 0x60) - 1U;
        local_2c = local_2c + 1) {
      uVar4 = FUN_0011096a(0,*(undefined8 *)((long)local_2c * 8 + *(long *)(param_2 + 0x68)));
      fprintf(stderr," %s",uVar4);
    }
    fprintf(stderr,"\n");
  }
  FUN_0011e1a8(stdout);
  FUN_0011e1a8(stderr);
  if (DAT_0014901c != 0) {
    DAT_0014901c = 0;
    signal(0x11,(__sighandler_t)0x0);
  }
  __pid = fork();
  if (__pid == -1) {
    uVar4 = gettext("cannot fork");
    piVar5 = __errno_location();
    error(1,*piVar5,uVar4);
  }
  if (__pid == 0) {
    if (*(long *)(param_2 + 0xc0) == 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("NULL != execp->wd_for_exec","exec.c",0x141,"launch");
    }
    cVar1 = FUN_0010b615(param_2[0xbc],*(undefined8 *)(param_2 + 0xc0));
    if (cVar1 == '\x01') {
      cVar1 = FUN_0011b5f4();
      if (cVar1 != '\0') {
        FUN_0011b61e();
      }
      cVar1 = FUN_0011a94d(param_4);
      if (cVar1 == '\0') {
        execvp(*param_4,param_4);
      }
      else {
        piVar5 = __errno_location();
        *piVar5 = 7;
      }
      uVar4 = FUN_0011096a(0,*param_4);
      piVar5 = __errno_location();
      error(0,*piVar5,&DAT_00138a9b,uVar4);
                    /* WARNING: Subroutine does not return */
      _exit(1);
    }
                    /* WARNING: Subroutine does not return */
    _exit(1);
  }
  while (_Var2 = waitpid(__pid,(int *)(param_2 + 200),0), _Var2 == -1) {
    piVar5 = __errno_location();
    if (*piVar5 != 4) {
      uVar4 = FUN_0011096a(0,*param_4);
      uVar6 = gettext("error waiting for %s");
      piVar5 = __errno_location();
      error(0,*piVar5,uVar6,uVar4);
      DAT_001499e4 = 1;
      return 0;
    }
  }
  if ('\0' < (char)(((byte)*(undefined4 *)(param_2 + 200) & 0x7f) + 1) >> 1) {
    uVar3 = *(uint *)(param_2 + 200);
    uVar4 = FUN_0012f861(0,DAT_001499ac,*param_4);
    uVar6 = gettext("%s terminated by signal %d");
    error(0,0,uVar6,uVar4,uVar3 & 0x7f);
    if (*param_2 != '\0') {
      DAT_001499e4 = 1;
    }
    return 1;
  }
  uVar3 = *(int *)(param_2 + 200) >> 8 & 0xff;
  if ((DAT_00149980 & 0x20) != 0) {
    fprintf(stderr,"DebugExec: process (PID=%ld) terminated with exit status: %d\n",(long)__pid,
            (ulong)uVar3);
  }
  if (uVar3 == 0) {
    return 1;
  }
  if (*param_2 != '\0') {
    DAT_001499e4 = 1;
  }
  return 1;
}




// Function: matches_start_point @ 0xbb9d

undefined8 matches_start_point(char *param_1,char param_2)

{
  int iVar1;
  undefined8 uVar2;
  undefined4 extraout_var;
  int local_14;
  ulong local_10;
  
  local_14 = 0;
  if (param_2 != '\0') {
    local_14 = 0x10;
  }
  if (DAT_00149a28 == 0) {
    iVar1 = fnmatch(param_1,".",local_14);
    uVar2 = CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),iVar1 == 0);
  }
  else {
    for (local_10 = 0; local_10 < DAT_00149a28; local_10 = local_10 + 1) {
      iVar1 = fnmatch(param_1,*(char **)(local_10 * 8 + DAT_00149a20),local_14);
      if (iVar1 == 0) {
        return 1;
      }
    }
    uVar2 = 0;
  }
  return uVar2;
}




// Function: get_expr @ 0xbc44

long get_expr(long *param_1,short param_2,long param_3)

{
  long lVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  long local_28;
  
  local_28 = 0;
  lVar1 = *param_1;
  if (*param_1 == 0) {
    uVar2 = gettext("invalid expression");
    error(1,0,uVar2);
  }
  switch(*(undefined4 *)(*param_1 + 0x10)) {
  case 0:
    uVar2 = gettext("invalid expression");
    error(1,0,uVar2);
  case 3:
    uVar2 = *(undefined8 *)(lVar1 + 8);
    uVar3 = gettext(
                   "invalid expression; you have used a binary operator \'%s\' with nothing before it."
                   );
    error(1,0,uVar3,uVar2);
  case 5:
    if (param_3 == 0) {
      uVar2 = *(undefined8 *)(lVar1 + 8);
      uVar3 = gettext("invalid expression: expected expression before closing parentheses \'%s\'.");
      error(1,0,uVar3,uVar2);
    }
    if (((*(int *)(param_3 + 0x10) == 2) || (*(int *)(param_3 + 0x10) == 3)) &&
       (*(char *)(lVar1 + 0x29) != '\x01')) {
      uVar2 = *(undefined8 *)(param_3 + 8);
      uVar3 = gettext("expected an expression between \'%s\' and \')\'");
      error(1,0,uVar3,uVar2);
    }
    if (*(char *)(*param_1 + 0x29) != '\0') {
      uVar2 = *(undefined8 *)(param_3 + 8);
      uVar3 = gettext("expected an expression after \'%s\'");
      error(1,0,uVar3,uVar2);
    }
    uVar2 = gettext("invalid expression; you have too many \')\'");
    error(1,0,uVar2);
  case 1:
    local_28 = *param_1;
    *param_1 = *(long *)(*param_1 + 0x108);
    break;
  case 2:
    local_28 = *param_1;
    *param_1 = *(long *)(*param_1 + 0x108);
    uVar2 = get_expr(param_1,4,local_28);
    *(undefined8 *)(local_28 + 0x118) = uVar2;
    break;
  case 4:
    if ((*(long *)(*param_1 + 0x108) == 0) ||
       (*(char *)(*(long *)(*param_1 + 0x108) + 0x29) != '\0')) {
      uVar2 = *(undefined8 *)(lVar1 + 8);
      uVar3 = gettext(
                     "invalid expression; expected to find a \')\' but didn\'t see one. Perhaps you need an extra predicate after \'%s\'"
                     );
      error(1,0,uVar3,uVar2);
    }
    lVar1 = *param_1;
    *param_1 = *(long *)(*param_1 + 0x108);
    if (*(int *)(*param_1 + 0x10) == 5) {
      if (*(char *)(lVar1 + 0x29) != '\0') {
        uVar2 = *(undefined8 *)(*param_1 + 8);
        uVar3 = gettext("invalid expression: expected expression before closing parentheses \'%s\'."
                       );
        error(1,0,uVar3,uVar2);
      }
      uVar2 = gettext("invalid expression; empty parentheses are not allowed.");
      error(1,0,uVar2);
    }
    local_28 = get_expr(param_1,0,lVar1);
    if ((*param_1 == 0) || (*(int *)(*param_1 + 0x10) != 5)) {
      uVar2 = gettext(
                     "invalid expression; I was expecting to find a \')\' somewhere but did not see one."
                     );
      error(1,0,uVar2);
    }
    *param_1 = *(long *)(*param_1 + 0x108);
    break;
  default:
    uVar2 = gettext("oops -- invalid expression type!");
    error(1,0,uVar2);
  }
  if (((*param_1 != 0) && ((int)param_2 < *(int *)(*param_1 + 0x14))) &&
     (local_28 = FUN_0010c08f(param_1,local_28,(int)param_2), local_28 == 0)) {
    uVar2 = gettext("invalid expression");
    error(1,0,uVar2);
  }
  return local_28;
}




// Function: scan_rest @ 0xc08f

long scan_rest(long *param_1,long param_2,short param_3)

{
  uint uVar1;
  undefined4 uVar2;
  long lVar3;
  undefined8 uVar4;
  long local_28;
  
  if ((*param_1 == 0) || (local_28 = param_2, *(int *)(*param_1 + 0x10) == 5)) {
    local_28 = 0;
  }
  else {
    while (((*param_1 != 0 && ((int)param_3 < *(int *)(*param_1 + 0x14))) &&
           (uVar1 = *(uint *)(*param_1 + 0x10), uVar1 != 5))) {
      if (uVar1 < 6) {
        if (uVar1 == 4) {
LAB_0010c115:
          uVar4 = gettext("invalid expression");
          error(1,0,uVar4);
        }
        else {
          if (4 < uVar1) goto LAB_0010c1a7;
          if (uVar1 < 3) goto LAB_0010c115;
          if (uVar1 != 3) goto LAB_0010c1a7;
        }
        lVar3 = *param_1;
        *(long *)(*param_1 + 0x110) = local_28;
        local_28 = *param_1;
        *param_1 = *(long *)(*param_1 + 0x108);
        uVar4 = FUN_0010bc44(param_1,(int)(short)*(undefined4 *)(local_28 + 0x14),lVar3);
        *(undefined8 *)(local_28 + 0x118) = uVar4;
      }
      else {
LAB_0010c1a7:
        uVar2 = *(undefined4 *)(*param_1 + 0x10);
        uVar4 = gettext("oops -- invalid expression type (%d)!");
        error(1,0,uVar4,uVar2);
      }
    }
  }
  return local_28;
}




// Function: predicate_is_cost_free @ 0xc209

undefined4 predicate_is_cost_free(long *param_1)

{
  undefined4 uVar1;
  
  if (((((code *)*param_1 == FUN_00109afa) || ((code *)*param_1 == FUN_0010a012)) ||
      ((code *)*param_1 == FUN_001097a2)) || ((code *)*param_1 == FUN_0010986a)) {
    uVar1 = 1;
  }
  else if (DAT_001499a8 == 0) {
    uVar1 = 0;
  }
  else if ((((code *)*param_1 == FUN_00108d9c) || ((code *)*param_1 == FUN_00109b30)) ||
          (((code *)*param_1 == FUN_00108fd0 || ((code *)*param_1 == FUN_00109f9e)))) {
    uVar1 = 0;
  }
  else {
    uVar1 = CONCAT31((int3)((uint)(int)param_1[4] >> 8),(int)param_1[4] == 0);
  }
  return uVar1;
}




// Function: print_predicate @ 0xc2dc

void print_predicate(FILE *param_1,long param_2)

{
  if (*(long *)(param_2 + 0x30) == 0) {
    fprintf(param_1,"%s",*(undefined8 *)(param_2 + 8));
  }
  else {
    fprintf(param_1,"%s %s",*(undefined8 *)(param_2 + 8),*(undefined8 *)(param_2 + 0x30));
  }
  return;
}




// Function: predlist_init @ 0xc34d

void predlist_init(undefined8 *param_1)

{
  param_1[1] = 0;
  *param_1 = param_1[1];
  return;
}




// Function: predlist_insert @ 0xc377

void predlist_insert(long *param_1,long param_2,undefined8 *param_3)

{
  *param_3 = *(undefined8 *)(param_2 + 0x110);
  *(long *)(param_2 + 0x110) = *param_1;
  *param_1 = param_2;
  if (param_1[1] == 0) {
    param_1[1] = *param_1;
  }
  return;
}




// Function: pred_cost_compare @ 0xc3e1

undefined8 pred_cost_compare(long param_1,long param_2,char param_3)

{
  undefined8 uVar1;
  
  if (*(int *)(param_1 + 0x20) == *(int *)(param_2 + 0x20)) {
    if (*(float *)(param_1 + 0x24) == *(float *)(param_2 + 0x24)) {
      uVar1 = 0;
    }
    else if (param_3 == '\0') {
      if (*(float *)(param_2 + 0x24) <= *(float *)(param_1 + 0x24)) {
        uVar1 = 0xffffffff;
      }
      else {
        uVar1 = 1;
      }
    }
    else if (*(float *)(param_2 + 0x24) <= *(float *)(param_1 + 0x24)) {
      uVar1 = 1;
    }
    else {
      uVar1 = 0xffffffff;
    }
  }
  else if (*(uint *)(param_1 + 0x20) < *(uint *)(param_2 + 0x20)) {
    uVar1 = 0xffffffff;
  }
  else {
    uVar1 = 1;
  }
  return uVar1;
}




// Function: predlist_merge_sort @ 0xc49b

void predlist_merge_sort(long *param_1,undefined8 param_2)

{
  long lVar1;
  int iVar2;
  long in_FS_OFFSET;
  long local_38;
  long local_28;
  long local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (*param_1 != 0) {
    if ((DAT_00149980 & 8) != 0) {
      fprintf(stderr,"%s:\n","predlist before merge sort");
      FUN_0010ea9f(stderr,*param_1,2);
    }
    FUN_0010d838(*param_1);
    FUN_0010c34d(&local_28);
    while (*param_1 != 0) {
      lVar1 = *param_1;
      *param_1 = *(long *)(*param_1 + 0x110);
      *(undefined8 *)(lVar1 + 0x110) = 0;
      local_38 = local_28;
      while ((local_38 != 0 &&
             (iVar2 = FUN_0010c3e1(*(undefined8 *)(local_38 + 0x118),*(undefined8 *)(lVar1 + 0x118),
                                   *(int *)(local_38 + 0x14) != 2), iVar2 < 0))) {
        local_38 = *(long *)(local_38 + 0x110);
      }
      if (local_38 == 0) {
        *(long *)(lVar1 + 0x110) = local_28;
        local_28 = lVar1;
        if (local_20 == 0) {
          local_20 = lVar1;
        }
      }
      else {
        *(undefined8 *)(lVar1 + 0x110) = *(undefined8 *)(local_38 + 0x110);
        if (*(long *)(lVar1 + 0x110) == 0) {
          local_20 = lVar1;
        }
        *(long *)(local_38 + 0x110) = lVar1;
      }
    }
    if ((DAT_00149980 & 8) != 0) {
      fprintf(stderr,"%s:\n","predlist after merge sort");
      FUN_0010ea9f(stderr,local_28,2);
    }
    FUN_0010d838(local_28);
    FUN_0010d3fd(local_28,local_20,param_2);
    FUN_0010c34d(param_1);
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: merge_lists @ 0xc6e0

void merge_lists(long param_1,int param_2,undefined8 param_3,undefined8 param_4,undefined8 param_5)

{
  int local_c;
  
  DAT_00149a38 = FUN_0010c49b;
  FUN_0010c49b(param_3,param_5);
  (*DAT_00149a38)(param_4,param_5);
  for (local_c = 0; local_c < param_2; local_c = local_c + 1) {
    (*DAT_00149a38)((long)local_c * 0x10 + param_1,param_5);
  }
  return;
}




// Function: subtree_has_side_effects @ 0xc77a

undefined4 subtree_has_side_effects(long param_1)

{
  char cVar1;
  undefined4 uVar2;
  
  if (param_1 == 0) {
    uVar2 = 0;
  }
  else if (((*(char *)(param_1 + 0x18) == '\0') &&
           (cVar1 = subtree_has_side_effects(*(undefined8 *)(param_1 + 0x110)), cVar1 == '\0')) &&
          (cVar1 = subtree_has_side_effects(*(undefined8 *)(param_1 + 0x118)), cVar1 == '\0')) {
    uVar2 = 0;
  }
  else {
    uVar2 = 1;
  }
  return uVar2;
}




// Function: worst_cost @ 0xc7e3

uint worst_cost(long param_1)

{
  uint uVar1;
  uint uVar2;
  
  if (param_1 == 0) {
    uVar1 = 0;
  }
  else {
    uVar1 = worst_cost(*(undefined8 *)(param_1 + 0x110));
    uVar2 = worst_cost(*(undefined8 *)(param_1 + 0x118));
    if (uVar1 <= uVar2) {
      uVar1 = uVar2;
    }
    if (uVar1 < *(uint *)(param_1 + 0x20)) {
      uVar1 = *(uint *)(param_1 + 0x20);
    }
  }
  return uVar1;
}




// Function: perform_arm_swap @ 0xc856

void perform_arm_swap(long param_1)

{
  undefined8 uVar1;
  
  uVar1 = *(undefined8 *)(*(long *)(param_1 + 0x110) + 0x118);
  *(undefined8 *)(*(long *)(param_1 + 0x110) + 0x118) = *(undefined8 *)(param_1 + 0x118);
  *(undefined8 *)(param_1 + 0x118) = uVar1;
  return;
}




// Function: consider_arm_swap @ 0xc8a7

undefined8 consider_arm_swap(long *param_1)

{
  float fVar1;
  float fVar2;
  bool bVar3;
  char cVar4;
  long *plVar5;
  long *plVar6;
  int local_30;
  int local_2c;
  char *local_20;
  
  local_20 = (char *)0x0;
  if ((int)param_1[2] != 3) {
    local_20 = "Not a binary operation";
  }
  if ((local_20 == (char *)0x0) && ((param_1[0x22] == 0 || (param_1[0x23] == 0)))) {
    local_20 = "Doesn\'t have two arms";
  }
  if ((local_20 == (char *)0x0) && (*(long *)(param_1[0x22] + 0x118) == 0)) {
    local_20 = "Left arm has no child on RHS";
  }
  plVar5 = param_1 + 0x23;
  plVar6 = (long *)(param_1[0x22] + 0x118);
  if ((local_20 == (char *)0x0) && (cVar4 = FUN_0010c77a(*plVar6), cVar4 != '\0')) {
    local_20 = "Left subtree has side-effects";
  }
  if ((local_20 == (char *)0x0) && (cVar4 = FUN_0010c77a(*plVar5), cVar4 != '\0')) {
    local_20 = "Right subtree has side-effects";
  }
  if (local_20 == (char *)0x0) {
    local_30 = FUN_0010c7e3(*plVar6);
    local_2c = FUN_0010c7e3(*plVar5);
    if (local_30 < local_2c) {
      local_20 = "efficient as-is";
    }
  }
  if (local_20 == (char *)0x0) {
    if (local_30 == local_2c) {
      fVar1 = *(float *)(*plVar6 + 0x24);
      fVar2 = *(float *)(*plVar5 + 0x24);
      if ((DAT_00149980 & 8) != 0) {
        fprintf(stderr,"Success rates: l=%f, r=%f\n",(double)fVar1,(double)fVar2);
      }
      if ((code *)*param_1 == FUN_00109f9e) {
        bVar3 = fVar2 < fVar1;
        if (fVar1 <= fVar2) {
          local_20 = "Operation is OR; right success rate >= left";
        }
      }
      else if ((code *)*param_1 == FUN_00108d9c) {
        bVar3 = fVar1 < fVar2;
        if (fVar2 <= fVar1) {
          local_20 = "Operation is AND; right success rate <= left";
        }
      }
      else {
        bVar3 = false;
        local_20 = "Not \'AND\' or \'OR\'";
      }
    }
    else {
      bVar3 = true;
    }
    if (bVar3) {
      if ((DAT_00149980 & 8) != 0) {
        fprintf(stderr,"Performing arm swap on:\n");
        FUN_0010ea9f(stderr,param_1,0);
      }
      FUN_0010c856(param_1);
      return 1;
    }
  }
  if ((DAT_00149980 & 8) != 0) {
    fprintf(stderr,"Not an arm swap candidate (%s):\n",local_20);
    FUN_0010ea9f(stderr,param_1,0);
  }
  return 0;
}




// Function: do_arm_swaps @ 0xcb9f

undefined1 do_arm_swaps(long param_1)

{
  bool bVar1;
  char cVar2;
  
  if (param_1 != 0) {
    do {
      bVar1 = false;
      cVar2 = FUN_0010c8a7(param_1);
      if (((cVar2 != '\0') ||
          (cVar2 = do_arm_swaps(*(undefined8 *)(param_1 + 0x110)), cVar2 != '\0')) ||
         (cVar2 = do_arm_swaps(*(undefined8 *)(param_1 + 0x118)), cVar2 != '\0')) {
        bVar1 = true;
      }
    } while (bVar1);
  }
  return 0;
}




// Function: opt_expr @ 0xcc0f

undefined1 opt_expr(long *param_1)

{
  uint uVar1;
  code *pcVar2;
  long lVar3;
  char cVar4;
  bool bVar5;
  undefined1 uVar6;
  undefined8 uVar7;
  long in_FS_OFFSET;
  undefined1 local_12a;
  int local_128;
  undefined4 local_124;
  int local_120;
  long local_118;
  long *local_110;
  undefined8 local_f8;
  undefined8 local_f0;
  undefined8 local_e8;
  undefined8 local_e0;
  undefined1 local_d8 [184];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_f8 = 0;
  local_f0 = 0;
  local_e8 = 0;
  local_e0 = 0;
  local_12a = 0;
  if ((param_1 != (long *)0x0) && (*param_1 != 0)) {
    for (local_128 = 0; local_128 < 0xb; local_128 = local_128 + 1) {
      FUN_0010c34d(local_d8 + (long)local_128 * 0x10);
    }
    local_124 = 3;
    local_110 = param_1;
    for (local_118 = *param_1; *(long *)(local_118 + 0x110) != 0;
        local_118 = *(long *)(local_118 + 0x110)) {
      local_110 = (long *)(local_118 + 0x110);
      local_124 = *(undefined4 *)(local_118 + 0x14);
    }
    if (*(int *)(local_118 + 0x10) != 3) {
      FUN_0010d2d0(local_118,local_124,local_110);
    }
    if ((DAT_00149980 & 9) != 0) {
      fprintf(stderr,"Normalized Eval Tree:\n");
      FUN_0010ea9f(stderr,*param_1,0);
    }
    local_120 = 0;
    local_110 = param_1;
    if ((*param_1 != 0) && (*(int *)(*param_1 + 0x10) == 3)) {
      local_120 = *(int *)(*param_1 + 0x14);
    }
LAB_0010d216:
    do {
      local_118 = *local_110;
      if (local_118 == 0) goto code_r0x0010d235;
      if ((*(int *)(local_118 + 0x10) == 3) && (local_120 != *(int *)(local_118 + 0x14))) {
        local_118 = FUN_0010d2d0(local_118,local_120,local_110);
      }
      uVar1 = *(uint *)(*(long *)(local_118 + 0x118) + 0x10);
      pcVar2 = (code *)**(long **)(local_118 + 0x118);
      if (uVar1 == 3) {
        lVar3 = *(long *)(local_118 + 0x118);
        uVar6 = opt_expr(local_118 + 0x118);
        *(undefined1 *)(lVar3 + 0x18) = uVar6;
      }
      else if (uVar1 < 4) {
        if (uVar1 < 2) {
          if ((local_120 != 1) && (*(char *)(*(long *)(local_118 + 0x118) + 0x18) != '\x01')) {
            cVar4 = FUN_0010c209(*(undefined8 *)(local_118 + 0x118));
            if (cVar4 != '\0') {
              if ((DAT_00149980 & 8) != 0) {
                fprintf(stderr,"-O%d: promoting cheap predicate ",(ulong)DAT_001499a8);
                FUN_0010c2dc(stderr,*(undefined8 *)(local_118 + 0x118));
                fprintf(stderr," into name_list\n");
              }
              FUN_0010c377(&local_e8,local_118,local_110);
              goto LAB_0010d216;
            }
            if (pcVar2 == FUN_0010a2da) {
              FUN_0010c377(&local_f8,local_118,local_110);
              goto LAB_0010d216;
            }
            if (((DAT_001499a8 < 2) ||
                (((*(int *)(*(long *)(local_118 + 0x118) + 0x20) != 2 &&
                  (*(int *)(*(long *)(local_118 + 0x118) + 0x20) != 1)) ||
                 (*(char *)(*(long *)(local_118 + 0x118) + 0x1a) == '\x01')))) && (DAT_001499a8 < 3)
               ) {
              bVar5 = false;
            }
            else {
              bVar5 = true;
            }
            if (bVar5) {
              if ((DAT_00149980 & 8) != 0) {
                fprintf(stderr,"-O%d: categorising predicate ",(ulong)DAT_001499a8);
                FUN_0010c2dc(stderr,*(undefined8 *)(local_118 + 0x118));
                uVar7 = FUN_0010e95d(*(undefined4 *)(*(long *)(local_118 + 0x118) + 0x20));
                fprintf(stderr," by cost (%s)\n",uVar7);
              }
              FUN_0010c377(local_d8 + (ulong)*(uint *)(*(long *)(local_118 + 0x118) + 0x20) * 0x10,
                           local_118,local_110);
              goto LAB_0010d216;
            }
          }
        }
        else {
          if (uVar1 != 2) goto LAB_0010d181;
          lVar3 = *(long *)(local_118 + 0x118);
          uVar6 = opt_expr(*(long *)(local_118 + 0x118) + 0x118);
          *(undefined1 *)(lVar3 + 0x18) = uVar6;
        }
      }
      else {
LAB_0010d181:
        uVar7 = gettext("oops -- invalid expression type!");
        error(1,0,uVar7);
      }
      if (*(char *)(*(long *)(local_118 + 0x118) + 0x18) != '\0') {
        FUN_0010c6e0(local_d8,0xb,&local_e8,&local_f8,local_110);
        local_12a = 1;
      }
      local_110 = (long *)(local_118 + 0x110);
    } while( true );
  }
  local_12a = 0;
LAB_0010d276:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_12a;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
code_r0x0010d235:
  FUN_0010c6e0(local_d8,0xb,&local_e8,&local_f8,local_110);
  goto LAB_0010d276;
}




// Function: constrain_rate @ 0xd290

void constrain_rate(void)

{
  return;
}




// Function: set_new_parent @ 0xd2d0

undefined8 * set_new_parent(long param_1,uint param_2,undefined8 *param_3)

{
  undefined8 *puVar1;
  undefined4 uVar2;
  
  puVar1 = (undefined8 *)FUN_0013205a(0x138);
  *(undefined4 *)(puVar1 + 2) = 3;
  *(uint *)((long)puVar1 + 0x14) = param_2;
  *(undefined4 *)(puVar1 + 4) = 0;
  if (param_2 == 3) {
    *puVar1 = FUN_00108d9c;
    puVar1[1] = &DAT_00139094;
    uVar2 = FUN_0010d290(*(undefined4 *)(param_1 + 0x24));
    *(undefined4 *)((long)puVar1 + 0x24) = uVar2;
  }
  else if (param_2 < 4) {
    if (param_2 == 1) {
      *puVar1 = FUN_00108fd0;
      puVar1[1] = &DAT_0013908f;
      *(undefined4 *)((long)puVar1 + 0x24) = DAT_00139684;
    }
    else if (param_2 == 2) {
      *puVar1 = FUN_00109f9e;
      puVar1[1] = &DAT_00139091;
      uVar2 = FUN_0010d290(*(undefined4 *)(param_1 + 0x24));
      *(undefined4 *)((long)puVar1 + 0x24) = uVar2;
    }
  }
  puVar1[0x23] = param_1;
  *param_3 = puVar1;
  return puVar1;
}




// Function: merge_pred @ 0xd3fd

void merge_pred(undefined8 param_1,long param_2,undefined8 *param_3)

{
  *(undefined8 *)(param_2 + 0x110) = *param_3;
  *param_3 = param_1;
  return;
}




// Function: check_sorted @ 0xd431

undefined8 check_sorted(long param_1,ulong param_2,long param_3,code *param_4)

{
  int iVar1;
  ulong local_18;
  
  local_18 = 1;
  while( true ) {
    if (param_2 <= local_18) {
      return 1;
    }
    iVar1 = (*param_4)(local_18 * param_3 + param_1,(local_18 - 1) * param_3 + param_1);
    if (iVar1 < 0) break;
    iVar1 = (*param_4)((local_18 - 1) * param_3 + param_1,local_18 * param_3 + param_1);
    if (0 < iVar1) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("result <= 0","tree.c",0x3f5,"check_sorted");
    }
    local_18 = local_18 + 1;
  }
  return 0;
}




// Function: cost_table_comparison @ 0xd52e

void cost_table_comparison(undefined8 *param_1,undefined8 *param_2)

{
  long in_FS_OFFSET;
  undefined8 local_20;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_20 = *param_1;
  local_18 = *param_2;
  memcmp(&local_20,&local_18,8);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: get_pred_cost @ 0xd5a5

uint get_pred_cost(long *param_1)

{
  long lVar1;
  char cVar2;
  void *pvVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  uint local_48;
  uint local_44;
  long local_38 [3];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (*(char *)((long)param_1 + 0x1a) == '\0') {
    if (*(char *)((long)param_1 + 0x1c) == '\0') {
      if (*(char *)((long)param_1 + 0x1b) == '\0') {
        local_48 = 0;
      }
      else {
        local_48 = 2;
      }
    }
    else {
      local_48 = 1;
    }
  }
  else {
    local_48 = 3;
  }
  if (((code *)*param_1 == FUN_00109466) || ((code *)*param_1 == FUN_00109497)) {
    if ((char)param_1[7] == '\0') {
      local_44 = 8;
    }
    else {
      local_44 = 7;
    }
  }
  else if ((code *)*param_1 == FUN_0011376d) {
    local_44 = *(uint *)(param_1 + 4);
  }
  else {
    if (DAT_00149a30 == 0) {
      qsort(&PTR_FUN_00149020,0x3b,0x10,FUN_0010d52e);
      cVar2 = FUN_0010d431(&PTR_FUN_00149020,0x3b,0x10,FUN_0010d52e);
      if (cVar2 != '\x01') {
        error(1,0,"failed to sort the costlookup array");
      }
      DAT_00149a30 = 1;
    }
    local_38[0] = *param_1;
    pvVar3 = bsearch(local_38,&PTR_FUN_00149020,0x3b,0x10,FUN_0010d52e);
    if (pvVar3 == (void *)0x0) {
      lVar1 = param_1[1];
      uVar4 = gettext(
                     "warning: there is no entry in the predicate evaluation cost table for predicate %s; please report this as a bug"
                     );
      error(0,0,uVar4,lVar1);
      local_44 = 10;
    }
    else {
      local_44 = *(uint *)((long)pvVar3 + 8);
    }
  }
  if (local_48 < local_44) {
    local_48 = local_44;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_48;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: estimate_costs @ 0xd7ac

void estimate_costs(long param_1)

{
  undefined4 uVar1;
  
  if (param_1 != 0) {
    estimate_costs(*(undefined8 *)(param_1 + 0x118));
    estimate_costs(*(undefined8 *)(param_1 + 0x110));
    uVar1 = FUN_0010d5a5(param_1);
    *(undefined4 *)(param_1 + 0x20) = uVar1;
  }
  return;
}




// Function: get_eval_tree @ 0xd7ff

undefined8 get_eval_tree(void)

{
  return DAT_00149a10;
}




// Function: getrate @ 0xd810

undefined4 getrate(long param_1)

{
  undefined4 uVar1;
  
  uVar1 = DAT_00139684;
  if (param_1 != 0) {
    uVar1 = *(undefined4 *)(param_1 + 0x24);
  }
  return uVar1;
}




// Function: calculate_derived_rates @ 0xd838

void calculate_derived_rates(long *param_1)

{
  float fVar1;
  undefined4 uVar2;
  float local_c;
  
  if (param_1 == (long *)0x0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("NULL != p","tree.c",0x484,"calculate_derived_rates");
  }
  if (param_1[0x23] != 0) {
    calculate_derived_rates(param_1[0x23]);
  }
  if (param_1[0x22] != 0) {
    calculate_derived_rates(param_1[0x22]);
  }
  if ((int)param_1[2] == 5) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("p->p_type != CLOSE_PAREN","tree.c",0x48b,"calculate_derived_rates");
  }
  if ((int)param_1[2] == 4) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("p->p_type != OPEN_PAREN","tree.c",0x48c,"calculate_derived_rates");
  }
  switch((int)param_1[2]) {
  case 0:
    if (param_1[0x23] != 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("NULL == p->pred_right","tree.c",0x491,"calculate_derived_rates");
    }
    if (param_1[0x22] != 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("NULL == p->pred_left","tree.c",0x492,"calculate_derived_rates");
    }
    break;
  case 1:
    if (param_1[0x23] != 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("NULL == p->pred_right","tree.c",0x496,"calculate_derived_rates");
    }
    if (param_1[0x22] != 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("NULL == p->pred_left","tree.c",0x497,"calculate_derived_rates");
    }
    break;
  case 2:
    if ((code *)*param_1 != FUN_00109b30) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("pred_is (p, pred_negate)","tree.c",0x49c,"calculate_derived_rates");
    }
    if (param_1[0x22] != 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("NULL == p->pred_left","tree.c",0x49d,"calculate_derived_rates");
    }
    *(float *)((long)param_1 + 0x24) = DAT_00139684 - *(float *)(param_1[0x23] + 0x24);
    break;
  case 3:
    if ((code *)*param_1 == FUN_00108d9c) {
      fVar1 = (float)FUN_0010d810(param_1[0x23]);
      local_c = (float)FUN_0010d810(param_1[0x22]);
      local_c = local_c * fVar1;
    }
    else if ((code *)*param_1 == FUN_00108fd0) {
      local_c = DAT_00139684;
    }
    else {
      if ((code *)*param_1 != FUN_00109f9e) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("0","tree.c",0x4b4,"calculate_derived_rates");
      }
      fVar1 = (float)FUN_0010d810(param_1[0x23]);
      local_c = (float)FUN_0010d810(param_1[0x22]);
      local_c = local_c + fVar1;
    }
    uVar2 = FUN_0010d290(local_c);
    *(undefined4 *)((long)param_1 + 0x24) = uVar2;
    break;
  case 4:
  case 5:
    *(float *)((long)param_1 + 0x24) = DAT_00139684;
    break;
  default:
                    /* WARNING: Subroutine does not return */
    __assert_fail("0","tree.c",0x4c0,"calculate_derived_rates");
  }
  return;
}




// Function: check_normalization @ 0xdc43

void check_normalization(long param_1,char param_2)

{
  if ((param_2 != '\0') && (*(int *)(param_1 + 0x10) != 3)) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("BI_OP == p->p_type","tree.c",0x4ce,"check_normalization");
  }
  if (*(long *)(param_1 + 0x110) != 0) {
    if (*(int *)(*(long *)(param_1 + 0x110) + 0x10) != 3) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("BI_OP == p->pred_left->p_type","tree.c",0x4d3,"check_normalization");
    }
    check_normalization(*(undefined8 *)(param_1 + 0x110),0);
  }
  if (*(long *)(param_1 + 0x118) != 0) {
    check_normalization(*(undefined8 *)(param_1 + 0x118),0);
  }
  return;
}




// Function: build_expression_tree @ 0xdd20

undefined8 build_expression_tree(int param_1,long param_2,int param_3)

{
  long lVar1;
  long *__ptr;
  char cVar2;
  int iVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  int local_5c;
  int local_58;
  int local_54;
  long *local_50;
  long local_48;
  long local_40;
  long local_38;
  long local_30;
  int *local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_00149a08 = (long *)0x0;
  DAT_00149a20 = param_2 + (long)param_3 * 8;
  local_5c = param_1;
  for (local_58 = param_3; local_58 < local_5c; local_58 = local_58 + 1) {
    cVar2 = FUN_0010ff96(*(undefined8 *)(param_2 + (long)local_58 * 8),1);
    if (cVar2 == '\x01') break;
    DAT_00149a28 = DAT_00149a28 + 1;
  }
  local_48 = FUN_0011437a(&DAT_00139215);
  local_40 = FUN_0011437a(&DAT_00139217);
  local_38 = FUN_0011437a("print");
  if (local_48 == 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("entry_open != NULL","tree.c",0x4f6,"build_expression_tree");
  }
  if (local_40 == 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("entry_close != NULL","tree.c",0x4f7,"build_expression_tree");
  }
  if (local_38 == 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("entry_print != NULL","tree.c",0x4f8,"build_expression_tree");
  }
  FUN_001168b2(local_48,param_2,&local_5c);
  DAT_00149a18[1] = (long)&DAT_00139215;
  *(undefined1 *)((long)DAT_00149a08 + 0x29) = 1;
  FUN_00114262(param_2,local_5c,DAT_00149a18,DAT_00149a08);
  FUN_0010aec7(DAT_00149a18);
  while (local_58 < local_5c) {
    DAT_001499f8 = 0;
    cVar2 = FUN_0010ff96(*(undefined8 *)(param_2 + (long)local_58 * 8),0);
    if (cVar2 != '\x01') {
      uVar5 = *(undefined8 *)(param_2 + (long)local_58 * 8);
      uVar4 = gettext("paths must precede expression: `%s\'");
      error(0,0,uVar4,uVar5);
      iVar3 = access(*(char **)(param_2 + (long)local_58 * 8),0);
      if (iVar3 == 0) {
        lVar1 = DAT_00149a18[1];
        uVar5 = gettext("possible unquoted pattern after predicate `%s\'?");
        error(0,0,uVar5,lVar1);
      }
                    /* WARNING: Subroutine does not return */
      exit(1);
    }
    local_30 = *(long *)(param_2 + (long)local_58 * 8);
    local_28 = (int *)FUN_0011437a(local_30);
    if (local_28 == (int *)0x0) {
      uVar5 = gettext("unknown predicate `%s\'");
      error(1,0,uVar5,local_30);
    }
    if (*local_28 != 4) {
      local_58 = local_58 + 1;
    }
    local_54 = local_58;
    cVar2 = (**(code **)(local_28 + 4))(local_28,param_2,&local_58);
    if (cVar2 != '\x01') {
      if (*(long *)(param_2 + (long)local_58 * 8) != 0) {
        if ((*local_28 == 4) && (local_54 == local_58)) {
          uVar5 = gettext("invalid predicate `%s\'");
          error(1,0,uVar5,local_30);
        }
        uVar5 = *(undefined8 *)(param_2 + (long)local_58 * 8);
        uVar4 = gettext("invalid argument `%s\' to `%s\'");
        error(1,0,uVar4,uVar5,local_30);
      }
      uVar5 = gettext("missing argument to `%s\'");
      error(1,0,uVar5,local_30);
    }
    DAT_00149a18[1] = local_30;
    if (local_54 == local_58) {
      DAT_00149a18[6] = 0;
    }
    else {
      DAT_00149a18[6] = *(long *)((long)local_54 * 8 + param_2);
    }
    FUN_0010aec7(DAT_00149a18);
    FUN_0010aec7(DAT_00149a08);
  }
  FUN_00114287(param_2,local_5c,DAT_00149a18,DAT_00149a08);
  __ptr = DAT_00149a08;
  if (DAT_00149a08[0x21] == 0) {
    local_50 = DAT_00149a08;
    DAT_00149a08 = (long *)DAT_00149a08[0x21];
    DAT_00149a18 = DAT_00149a08;
    free(__ptr);
    FUN_00116eb3(local_38,param_2,&local_5c);
    DAT_00149a18[1] = (long)"-print";
    FUN_0010aec7(DAT_00149a18);
    FUN_0010aec7(DAT_00149a08);
  }
  else {
    cVar2 = FUN_0010ff58(DAT_00149a08[0x21]);
    if (cVar2 == '\x01') {
      FUN_00114833(local_40,param_2,&local_5c);
      DAT_00149a18[1] = (long)&DAT_00139217;
      *(undefined1 *)((long)DAT_00149a18 + 0x29) = 1;
      FUN_0010aec7(DAT_00149a18);
      FUN_00116eb3(local_38,param_2,&local_5c);
      DAT_00149a18[1] = (long)"-print";
      *(undefined1 *)((long)DAT_00149a18 + 0x29) = 1;
      FUN_0010aec7(DAT_00149a18);
      FUN_0010aec7(DAT_00149a08);
    }
    else {
      local_50 = DAT_00149a08;
      DAT_00149a08 = (long *)DAT_00149a08[0x21];
      FUN_0010aec7(DAT_00149a08);
      free(local_50);
    }
  }
  if ((DAT_00149980 & 9) != 0) {
    fprintf(stderr,"Predicate List:\n");
    FUN_0010aa62(stderr,DAT_00149a08);
  }
  FUN_00113f89(DAT_00149a08);
  FUN_0010aec7(DAT_00149a08);
  local_50 = DAT_00149a08;
  DAT_00149a10 = FUN_0010bc44(&local_50,0,0);
  FUN_0010d838(DAT_00149a10);
  if (local_50 != (long *)0x0) {
    if ((code *)*local_50 == FUN_00108ee1) {
      uVar5 = gettext("you have too many \')\'");
      error(1,0,uVar5);
    }
    if (local_50[1] != 0) {
      lVar1 = local_50[1];
      uVar5 = gettext("unexpected extra predicate \'%s\'");
      error(1,0,uVar5,lVar1);
    }
    uVar5 = gettext("unexpected extra predicate");
    error(1,0,uVar5);
  }
  if ((DAT_00149980 & 9) != 0) {
    fprintf(stderr,"Eval Tree:\n");
    FUN_0010ea9f(stderr,DAT_00149a10,0);
  }
  FUN_0010d7ac(DAT_00149a10);
  FUN_0010cc0f(&DAT_00149a10);
  FUN_0010dc43(DAT_00149a10,1);
  FUN_0010cb9f(DAT_00149a10);
  FUN_0010dc43(DAT_00149a10,1);
  if ((DAT_00149980 & 9) != 0) {
    fprintf(stderr,"Optimized Eval Tree:\n");
    FUN_0010ea9f(stderr,DAT_00149a10,0);
    fprintf(stderr,"Optimized command line:\n");
    FUN_0010ac38(stderr,DAT_00149a10);
    fprintf(stderr,"\n");
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return DAT_00149a10;
}




// Function: init_pred_perf @ 0xe60c

void init_pred_perf(long param_1)

{
  *(undefined8 *)(param_1 + 0x128) = 0;
  *(undefined8 *)(param_1 + 0x120) = *(undefined8 *)(param_1 + 0x128);
  return;
}




// Function: get_new_pred_noarg @ 0xe644

long get_new_pred_noarg(undefined8 param_1)

{
  long lVar1;
  
  lVar1 = FUN_0010e67d(param_1);
  if (lVar1 != 0) {
    *(undefined8 *)(lVar1 + 0x30) = 0;
  }
  return lVar1;
}




// Function: get_new_pred @ 0xe67d

long get_new_pred(int *param_1)

{
  long lVar1;
  long lVar2;
  
  if (*param_1 == 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("entry->type != ARG_OPTION","tree.c",0x5cd,"get_new_pred");
  }
  if (*param_1 != 2) {
    lVar2 = FUN_0013205a(0x138);
    lVar1 = lVar2;
    if (DAT_00149a08 != 0) {
      *(long *)(DAT_00149a18 + 0x108) = lVar2;
      lVar1 = DAT_00149a08;
    }
    DAT_00149a08 = lVar1;
    DAT_00149a18 = lVar2;
    *(int **)(lVar2 + 0x130) = param_1;
    *(undefined4 *)(DAT_00149a18 + 0x10) = 0;
    *(undefined4 *)(DAT_00149a18 + 0x14) = 0;
    *(undefined1 *)(DAT_00149a18 + 0x1a) = 1;
    *(undefined1 *)(DAT_00149a18 + 0x1b) = 1;
    *(undefined4 *)(DAT_00149a18 + 0x20) = 10;
    *(char **)(DAT_00149a18 + 0x30) = "ThisShouldBeSetToSomethingElse";
    *(undefined1 *)(DAT_00149a18 + 0x28) = DAT_0014994f;
    *(undefined4 *)(DAT_00149a18 + 0x24) = DAT_00139684;
    FUN_0010e60c(DAT_00149a18);
    return DAT_00149a18;
  }
                    /* WARNING: Subroutine does not return */
  __assert_fail("entry->type != ARG_POSITIONAL_OPTION","tree.c",0x5ce,"get_new_pred");
}




// Function: get_new_pred_chk_op @ 0xe7dd

long get_new_pred_chk_op(undefined8 param_1,undefined8 param_2)

{
  uint uVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  long lVar4;
  
  if (DAT_00149a40 == 0) {
    DAT_00149a40 = FUN_0011437a(&DAT_0013942f);
  }
  if (DAT_00149a40 == 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("entry_and != NULL","tree.c",0x5f8,"get_new_pred_chk_op");
  }
  if (DAT_00149a18 != 0) {
    uVar1 = *(uint *)(DAT_00149a18 + 0x10);
    if (uVar1 != 5) {
      if (5 < uVar1) goto LAB_0010e92c;
      if (uVar1 == 0) {
        uVar2 = gettext("oops -- invalid default insertion of and!");
        error(1,0,uVar2);
      }
      else if (uVar1 != 1) goto LAB_0010e92c;
    }
    puVar3 = (undefined8 *)FUN_0010e644(DAT_00149a40);
    *puVar3 = FUN_00108d9c;
    puVar3[1] = &DAT_00139094;
    *(undefined4 *)(puVar3 + 2) = 3;
    *(undefined4 *)((long)puVar3 + 0x14) = 3;
    *(undefined1 *)((long)puVar3 + 0x1a) = 0;
    *(undefined1 *)((long)puVar3 + 0x1b) = 0;
    *(undefined1 *)((long)puVar3 + 0x1c) = 0;
    puVar3[6] = 0;
    puVar3[7] = 0;
    *(undefined1 *)(puVar3 + 3) = 0;
    *(undefined1 *)((long)puVar3 + 0x19) = 0;
  }
LAB_0010e92c:
  lVar4 = FUN_0010e67d(param_1);
  *(undefined8 *)(lVar4 + 0x30) = param_2;
  *(undefined8 *)(lVar4 + 0x130) = param_1;
  return lVar4;
}




// Function: cost_name @ 0xe95d

char * cost_name(int param_1)

{
  uint local_10;
  
  local_10 = 0;
  while( true ) {
    if (10 < local_10) {
      return "unknown";
    }
    if (param_1 == *(int *)(&DAT_001493e0 + (ulong)local_10 * 0x10)) break;
    local_10 = local_10 + 1;
  }
  return (&PTR_s_Nothing_001493e8)[(ulong)local_10 * 2];
}




// Function: type_name @ 0xe9bd

undefined * type_name(short param_1)

{
  int local_c;
  
  local_c = 0;
  while ((*(short *)(&DAT_00149520 + (long)local_c * 0x10) != -1 &&
         (param_1 != *(short *)(&DAT_00149520 + (long)local_c * 0x10)))) {
    local_c = local_c + 1;
  }
  return (&PTR_DAT_00149528)[(long)local_c * 2];
}




// Function: prec_name @ 0xea2e

undefined * prec_name(short param_1)

{
  int local_c;
  
  local_c = 0;
  while ((*(short *)(&DAT_001494a0 + (long)local_c * 0x10) != -1 &&
         (param_1 != *(short *)(&DAT_001494a0 + (long)local_c * 0x10)))) {
    local_c = local_c + 1;
  }
  return (&PTR_DAT_001494a8)[(long)local_c * 2];
}




// Function: print_tree @ 0xea9f

void print_tree(FILE *param_1,long param_2,int param_3)

{
  float fVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined *puVar4;
  bool bVar5;
  int local_20;
  
  if (param_2 != 0) {
    for (local_20 = 0; local_20 < param_3; local_20 = local_20 + 1) {
      fprintf(param_1,"    ");
    }
    fprintf(param_1,"pred=[");
    FUN_0010c2dc(param_1,param_2);
    uVar2 = FUN_0010ea2e((int)(short)*(undefined4 *)(param_2 + 0x14));
    uVar3 = FUN_0010e9bd((int)(short)*(undefined4 *)(param_2 + 0x10));
    fprintf(param_1,"] type=%s prec=%s",uVar3,uVar2);
    if (*(char *)(param_2 + 0x18) == '\0') {
      puVar4 = &DAT_0013955d;
    }
    else {
      puVar4 = &DAT_0013955c;
    }
    fVar1 = *(float *)(param_2 + 0x24);
    uVar2 = FUN_0010e95d(*(undefined4 *)(param_2 + 0x20));
    fprintf(param_1," cost=%s est_success_rate=%#.4g %sside effects ",(double)fVar1,uVar2,puVar4);
    if (((*(char *)(param_2 + 0x1a) != '\0') || (*(char *)(param_2 + 0x1b) != '\0')) ||
       (*(char *)(param_2 + 0x1c) != '\0')) {
      fprintf(param_1,"Needs ");
      bVar5 = *(char *)(param_2 + 0x1a) != '\0';
      if (bVar5) {
        fprintf(param_1,"stat");
      }
      if (*(char *)(param_2 + 0x1c) != '\0') {
        if (bVar5) {
          puVar4 = &DAT_0013908f;
        }
        else {
          puVar4 = &DAT_0013955c;
        }
        fprintf(param_1,"%sinode",puVar4);
        bVar5 = true;
      }
      if (*(char *)(param_2 + 0x1b) != '\0') {
        if (bVar5) {
          puVar4 = &DAT_0013908f;
        }
        else {
          puVar4 = &DAT_0013955c;
        }
        fprintf(param_1,"%stype",puVar4);
      }
    }
    fprintf(param_1,"\n");
    for (local_20 = 0; local_20 < param_3; local_20 = local_20 + 1) {
      fprintf(param_1,"    ");
    }
    if ((*(long *)(param_2 + 0x110) == 0) && (*(long *)(param_2 + 0x118) == 0)) {
      fprintf(param_1,"no children.\n");
    }
    else {
      if (*(long *)(param_2 + 0x110) == 0) {
        fprintf(param_1,"no left.\n");
      }
      else {
        fprintf(param_1,"left:\n");
        print_tree(param_1,*(undefined8 *)(param_2 + 0x110),param_3 + 1);
      }
      for (local_20 = 0; local_20 < param_3; local_20 = local_20 + 1) {
        fprintf(param_1,"    ");
      }
      if (*(long *)(param_2 + 0x118) == 0) {
        fprintf(param_1,"no right.\n");
      }
      else {
        fprintf(param_1,"right:\n");
        print_tree(param_1,*(undefined8 *)(param_2 + 0x118),param_3 + 1);
      }
    }
  }
  return;
}




// Function: insert_primary_withpred @ 0xee5a

undefined8 * insert_primary_withpred(long param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0010e7dd(param_1,param_3);
  *puVar1 = param_2;
  puVar1[1] = *(undefined8 *)(param_1 + 8);
  puVar1[7] = 0;
  *(undefined4 *)(puVar1 + 2) = 1;
  *(undefined4 *)((long)puVar1 + 0x14) = 0;
  return puVar1;
}




// Function: insert_primary @ 0xeecc

void insert_primary(long param_1,undefined8 param_2)

{
  if (*(long *)(param_1 + 0x18) == 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("entry->pred_func != NULL","util.c",0x7b,"insert_primary");
  }
  FUN_0010ee5a(param_1,*(undefined8 *)(param_1 + 0x18),param_2);
  return;
}




// Function: insert_primary_noarg @ 0xef32

void insert_primary_noarg(undefined8 param_1)

{
  FUN_0010eecc(param_1,0);
  return;
}




// Function: show_valid_debug_options @ 0xef55

void show_valid_debug_options(int param_1)

{
  FILE *__stream;
  char *__s;
  undefined *puVar1;
  ulong local_20;
  
  __stream = stdout;
  __s = (char *)gettext("Valid arguments for -D:\n");
  fputs(__s,__stream);
  if (param_1 == 0) {
    for (local_20 = 0; local_20 < 9; local_20 = local_20 + 1) {
      if (local_20 == 0) {
        puVar1 = &DAT_001398cd;
      }
      else {
        puVar1 = &DAT_001398ca;
      }
      fprintf(stdout,"%s%s",puVar1,(&PTR_DAT_001495a0)[local_20 * 3]);
    }
  }
  else {
    for (local_20 = 0; local_20 < 9; local_20 = local_20 + 1) {
      fprintf(stdout,"%-10s %s\n",(&PTR_DAT_001495a0)[local_20 * 3],
              (&PTR_s_Show_diagnostic_information_rela_001495b0)[local_20 * 3]);
    }
  }
  return;
}




// Function: usage @ 0xf071

void usage(int param_1)

{
  FILE *pFVar1;
  undefined8 uVar2;
  char *pcVar3;
  
  uVar2 = DAT_0014a230;
  if (param_1 != 0) {
    pcVar3 = (char *)gettext("Try \'%s --help\' for more information.\n");
    fprintf(stderr,pcVar3,uVar2);
                    /* WARNING: Subroutine does not return */
    exit(param_1);
  }
  pcVar3 = (char *)gettext(
                          "Usage: %s [-H] [-L] [-P] [-Olevel] [-D debugopts] [path...] [expression]\n"
                          );
  fprintf(stdout,pcVar3,uVar2);
  pFVar1 = stdout;
  pcVar3 = (char *)gettext(
                          "\nDefault path is the current directory; default expression is -print.\nExpression may consist of: operators, options, tests, and actions.\n"
                          );
  fputs(pcVar3,pFVar1);
  pFVar1 = stdout;
  pcVar3 = (char *)gettext(
                          "\nOperators (decreasing precedence; -and is implicit where no others are given):\n      ( EXPR )   ! EXPR   -not EXPR   EXPR1 -a EXPR2   EXPR1 -and EXPR2\n      EXPR1 -o EXPR2   EXPR1 -or EXPR2   EXPR1 , EXPR2\n"
                          );
  fputs(pcVar3,pFVar1);
  pFVar1 = stdout;
  pcVar3 = (char *)gettext(
                          "\nPositional options (always true):\n      -daystart -follow -nowarn -regextype -warn\n"
                          );
  fputs(pcVar3,pFVar1);
  pFVar1 = stdout;
  pcVar3 = (char *)gettext(
                          "\nNormal options (always true, specified before other expressions):\n      -depth -files0-from FILE -maxdepth LEVELS -mindepth LEVELS\n       -mount -noleaf -xdev -ignore_readdir_race -noignore_readdir_race\n"
                          );
  fputs(pcVar3,pFVar1);
  pFVar1 = stdout;
  pcVar3 = (char *)gettext(
                          "\nTests (N can be +N or -N or N):\n      -amin N -anewer FILE -atime N -cmin N -cnewer FILE -context CONTEXT\n      -ctime N -empty -false -fstype TYPE -gid N -group NAME -ilname PATTERN\n      -iname PATTERN -inum N -iwholename PATTERN -iregex PATTERN\n      -links N -lname PATTERN -mmin N -mtime N -name PATTERN -newer FILE\n      -nouser -nogroup -path PATTERN -perm [-/]MODE -regex PATTERN\n      -readable -writable -executable\n      -wholename PATTERN -size N[bcwkMG] -true -type [bcdpflsD] -uid N\n      -used N -user NAME -xtype [bcdpfls]\n"
                          );
  fputs(pcVar3,pFVar1);
  pFVar1 = stdout;
  pcVar3 = (char *)gettext(
                          "\nActions:\n      -delete -print0 -printf FORMAT -fprintf FILE FORMAT -print \n      -fprint0 FILE -fprint FILE -ls -fls FILE -prune -quit\n      -exec COMMAND ; -exec COMMAND {} + -ok COMMAND ;\n      -execdir COMMAND ; -execdir COMMAND {} + -okdir COMMAND ;\n"
                          );
  fputs(pcVar3,pFVar1);
  pFVar1 = stdout;
  pcVar3 = (char *)gettext("\nOther common options:\n");
  fputs(pcVar3,pFVar1);
  pFVar1 = stdout;
  pcVar3 = (char *)gettext("      --help                   display this help and exit\n");
  fputs(pcVar3,pFVar1);
  pFVar1 = stdout;
  pcVar3 = (char *)gettext("      --version                output version information and exit\n\n")
  ;
  fputs(pcVar3,pFVar1);
  FUN_0010ef55(0);
  pFVar1 = stdout;
  pcVar3 = (char *)gettext("\nUse \'-D help\' for a description of the options, or see find(1)\n\n")
  ;
  fputs(pcVar3,pFVar1);
  FUN_0011d008(stdout,DAT_0014a230);
                    /* WARNING: Subroutine does not return */
  exit(0);
}




// Function: set_stat_placeholders @ 0xf26e

void set_stat_placeholders(void)

{
  return;
}




// Function: get_statinfo @ 0xf27d

undefined8 get_statinfo(undefined8 param_1,undefined8 param_2,long param_3)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  int *piVar4;
  
  if (DAT_001499c4 != '\x01') {
    FUN_0010f26e(param_3);
    iVar1 = (*DAT_00149990)(param_2,param_3);
    if (iVar1 != 0) {
      if ((DAT_0014994e != '\x01') || (piVar4 = __errno_location(), *piVar4 != 2)) {
        piVar4 = __errno_location();
        FUN_00110a13(*piVar4,param_1);
      }
      return 0xffffffff;
    }
    if (*(int *)(param_3 + 0x18) == 0) {
      uVar2 = FUN_0012f861(0,DAT_001499ac,param_2);
      uVar3 = gettext("WARNING: file %s appears to have mode 0000");
      error(0,0,uVar3,uVar2);
      DAT_001499e4 = 1;
    }
  }
  DAT_001499c8 = *(undefined4 *)(param_3 + 0x18);
  DAT_001499c5 = 1;
  DAT_001499c4 = 1;
  return 0;
}




// Function: get_info @ 0xf388

undefined8 get_info(undefined8 param_1,long param_2,long param_3)

{
  bool bVar1;
  int iVar2;
  
  bVar1 = false;
  if ((*(char *)(param_3 + 0x1a) == '\0') || (DAT_001499c4 == '\x01')) {
    if ((*(char *)(param_3 + 0x1b) == '\0') || (DAT_001499c5 == '\x01')) {
      if (*(char *)(param_3 + 0x1c) != '\0') {
        if (*(long *)(param_2 + 8) == 0) {
          bVar1 = true;
        }
        else if ((DAT_001499c5 != '\x01') || ((*(uint *)(param_2 + 0x18) & 0xf000) == 0x4000)) {
          bVar1 = true;
        }
      }
    }
    else {
      bVar1 = true;
    }
  }
  else {
    bVar1 = true;
  }
  if ((bVar1) && (iVar2 = FUN_0010f27d(param_1,DAT_001499d0,param_2), iVar2 != 0)) {
    return 0xffffffff;
  }
  return 0;
}




// Function: check_nofollow @ 0xf45a

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 check_nofollow(void)

{
  double dVar1;
  int iVar2;
  undefined4 extraout_var;
  undefined4 extraout_var_00;
  undefined8 uVar3;
  long in_FS_OFFSET;
  utsname local_198;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = uname(&local_198);
  if (iVar2 == 0) {
    dVar1 = (double)(*(code *)PTR_atof_00148fb8)(local_198.release);
    iVar2 = strcmp("Linux",local_198.sysname);
    if (iVar2 == 0) {
      uVar3 = CONCAT71((int7)(CONCAT44(extraout_var,iVar2) >> 8),_DAT_0013a450 <= (float)dVar1);
      goto LAB_0010f52a;
    }
    iVar2 = strcmp("FreeBSD",local_198.sysname);
    if (iVar2 == 0) {
      uVar3 = CONCAT71((int7)(CONCAT44(extraout_var_00,iVar2) >> 8),_DAT_0013a454 <= (float)dVar1);
      goto LAB_0010f52a;
    }
  }
  uVar3 = 1;
LAB_0010f52a:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar3;
}




// Function: exec_cb @ 0xf540

undefined8 exec_cb(long param_1)

{
  FUN_00119f5a(param_1 + 8,param_1 + 0x60);
  return 0;
}




// Function: do_exec @ 0xf57a

void do_exec(long param_1)

{
  FUN_0011a9dd(*(undefined8 *)(param_1 + 0xc0),FUN_0010f540,param_1);
  if (*(long *)(param_1 + 0xc0) != DAT_00149a00) {
    FUN_0012fe52(*(undefined8 *)(param_1 + 0xc0));
    free(*(void **)(param_1 + 0xc0));
    *(undefined8 *)(param_1 + 0xc0) = 0;
  }
  return;
}




// Function: do_complete_pending_execdirs @ 0xf5fa

void do_complete_pending_execdirs(long *param_1)

{
  if (param_1 != (long *)0x0) {
    if (DAT_001499e8 == '\0') {
                    /* WARNING: Subroutine does not return */
      __assert_fail("state.execdirs_outstanding","util.c",0x18c,"do_complete_pending_execdirs");
    }
    do_complete_pending_execdirs(param_1[0x22]);
    if (((((code *)*param_1 == FUN_00109497) || ((code *)*param_1 == FUN_00109f27)) &&
        ((char)param_1[7] != '\0')) && ((int)param_1[0x1a] != 0)) {
      FUN_0010f57a(param_1 + 7);
    }
    do_complete_pending_execdirs(param_1[0x23]);
  }
  return;
}




// Function: complete_pending_execdirs @ 0xf6cb

void complete_pending_execdirs(void)

{
  undefined8 uVar1;
  
  if (DAT_001499e8 != '\0') {
    uVar1 = FUN_0010d7ff();
    FUN_0010f5fa(uVar1);
    DAT_001499e8 = '\0';
  }
  return;
}




// Function: complete_pending_execs @ 0xf6f5

void complete_pending_execs(long *param_1)

{
  if (param_1 != (long *)0x0) {
    complete_pending_execs(param_1[0x22]);
    if ((((code *)*param_1 == FUN_00109466) && ((char)param_1[7] != '\0')) &&
       ((int)param_1[0x1a] != 0)) {
      FUN_00119f5a(param_1 + 8,param_1 + 0x13);
    }
    complete_pending_execs(param_1[0x23]);
  }
  return;
}




// Function: record_initial_cwd @ 0xf78b

void record_initial_cwd(void)

{
  int iVar1;
  undefined8 uVar2;
  int *piVar3;
  undefined *puVar4;
  undefined *puVar5;
  
  DAT_00149a00 = (int *)FUN_00131bc2(0x10);
  iVar1 = FUN_0012fd99(DAT_00149a00);
  if (iVar1 != 0) {
    if ((*DAT_00149a00 < 0) && (*(long *)(DAT_00149a00 + 2) != 0)) {
      puVar5 = *(undefined **)(DAT_00149a00 + 2);
    }
    else {
      puVar5 = &DAT_001398cd;
    }
    if ((*DAT_00149a00 < 0) && (*(long *)(DAT_00149a00 + 2) != 0)) {
      puVar4 = &DAT_0013a034;
    }
    else {
      puVar4 = &DAT_001398cd;
    }
    uVar2 = gettext("Failed to save initial working directory%s%s");
    piVar3 = __errno_location();
    error(1,*piVar3,uVar2,puVar4,puVar5);
  }
  return;
}




// Function: cleanup_initial_cwd @ 0xf861

void cleanup_initial_cwd(void)

{
  int iVar1;
  undefined8 uVar2;
  int *piVar3;
  undefined *puVar4;
  undefined *puVar5;
  
  iVar1 = FUN_0012fe17(DAT_00149a00);
  if (iVar1 == 0) {
    FUN_0012fe52(DAT_00149a00);
    free(DAT_00149a00);
    DAT_00149a00 = (int *)0x0;
    return;
  }
  if ((*DAT_00149a00 < 0) && (*(long *)(DAT_00149a00 + 2) != 0)) {
    puVar5 = *(undefined **)(DAT_00149a00 + 2);
  }
  else {
    puVar5 = &DAT_001398cd;
  }
  if ((*DAT_00149a00 < 0) && (*(long *)(DAT_00149a00 + 2) != 0)) {
    puVar4 = &DAT_0013a034;
  }
  else {
    puVar4 = &DAT_001398cd;
  }
  uVar2 = gettext("Failed to restore initial working directory%s%s");
  piVar3 = __errno_location();
  error(0,*piVar3,uVar2,puVar4,puVar5);
                    /* WARNING: Subroutine does not return */
  _exit(1);
}




// Function: traverse_tree @ 0xf959

void traverse_tree(long param_1,code *param_2)

{
  if (*(long *)(param_1 + 0x110) != 0) {
    traverse_tree(*(undefined8 *)(param_1 + 0x110),param_2);
  }
  (*param_2)(param_1);
  if (*(long *)(param_1 + 0x118) != 0) {
    traverse_tree(*(undefined8 *)(param_1 + 0x118),param_2);
  }
  return;
}




// Function: undangle_file_pointers @ 0xf9d1

void undangle_file_pointers(long *param_1)

{
  if (((((code *)*param_1 == FUN_0010954f) || ((code *)*param_1 == FUN_0011376d)) ||
      ((code *)*param_1 == FUN_001094e6)) || ((code *)*param_1 == FUN_0010959f)) {
    param_1[8] = 0;
  }
  return;
}




// Function: cleanup @ 0xfa38

void cleanup(void)

{
  char cVar1;
  int iVar2;
  long lVar3;
  int *piVar4;
  
  lVar3 = FUN_0010d7ff();
  if (lVar3 != 0) {
    FUN_0010f959(lVar3,FUN_0010f6f5);
    FUN_0010f6cb();
  }
  FUN_00110cac(DAT_001499f0);
  if (lVar3 != 0) {
    FUN_0010f959(lVar3,FUN_0010f9d1);
  }
  FUN_0010f861();
  cVar1 = FUN_0011b5f4();
  if (cVar1 != '\0') {
    FUN_0011b61e();
    FUN_0011b5c4();
  }
  iVar2 = FUN_0011e1a8(stdout);
  if (iVar2 == -1) {
    piVar4 = __errno_location();
    FUN_00110a77(*piVar4,"standard output");
  }
  return;
}




// Function: fallback_stat @ 0xfae2

int fallback_stat(char *param_1,stat *param_2,int param_3)

{
  int *piVar1;
  
  piVar1 = __errno_location();
  if ((*piVar1 == 2) || (*piVar1 == 0x14)) {
    if ((DAT_00149980 & 2) != 0) {
      fprintf(stderr,"fallback_stat(): stat(%s) failed; falling back on lstat()\n",param_1);
    }
    param_3 = fstatat(DAT_001499d8,param_1,param_2,0x100);
  }
  return param_3;
}




// Function: optionh_stat @ 0xfb5c

int optionh_stat(char *param_1,stat *param_2)

{
  int iVar1;
  
  if ((DAT_001499d8 != -100) && (DAT_001499d8 < 0)) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("state.cwd_dir_fd >= 0","util.c",0x25d,"optionh_stat");
  }
  FUN_0010f26e(param_2);
  if (DAT_001499c0 == 0) {
    iVar1 = fstatat(DAT_001499d8,param_1,param_2,0);
    if (iVar1 == 0) {
      iVar1 = 0;
    }
    else {
      iVar1 = FUN_0010fae2(param_1,param_2,iVar1);
    }
  }
  else {
    iVar1 = fstatat(DAT_001499d8,param_1,param_2,0x100);
  }
  return iVar1;
}




// Function: optionl_stat @ 0xfc21

undefined8 optionl_stat(char *param_1,stat *param_2)

{
  int iVar1;
  undefined8 uVar2;
  
  if ((DAT_001499d8 != -100) && (DAT_001499d8 < 0)) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("state.cwd_dir_fd >= 0","util.c",0x27c,"optionl_stat");
  }
  FUN_0010f26e(param_2);
  iVar1 = fstatat(DAT_001499d8,param_1,param_2,0);
  if (iVar1 == 0) {
    uVar2 = 0;
  }
  else {
    uVar2 = FUN_0010fae2(param_1,param_2,iVar1);
  }
  return uVar2;
}




// Function: optionp_stat @ 0xfcc0

void optionp_stat(char *param_1,stat *param_2)

{
  if ((DAT_001499d8 < 0) && (DAT_001499d8 != -100)) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("(state.cwd_dir_fd >= 0) || (state.cwd_dir_fd==AT_FDCWD)","util.c",0x28d,
                  "optionp_stat");
  }
  FUN_0010f26e(param_2);
  fstatat(DAT_001499d8,param_1,param_2,0x100);
  return;
}




// Function: debug_stat @ 0xfd39

void debug_stat(undefined8 param_1,undefined8 param_2)

{
  DAT_00149a48 = DAT_00149a48 + 1;
  fprintf(stderr,"debug_stat (%s)\n",param_1);
  if (DAT_00149988 == 2) {
    FUN_0010fb5c(param_1,param_2);
  }
  else {
    if (2 < DAT_00149988) {
LAB_0010fdd9:
                    /* WARNING: Subroutine does not return */
      __assert_fail("0","util.c",0x2a5,"debug_stat");
    }
    if (DAT_00149988 == 0) {
      FUN_0010fcc0(param_1,param_2);
    }
    else {
      if (DAT_00149988 != 1) goto LAB_0010fdd9;
      FUN_0010fc21(param_1,param_2);
    }
  }
  return;
}




// Function: following_links @ 0xfe03

undefined4 following_links(void)

{
  undefined4 uVar1;
  
  if (DAT_00149988 == 1) {
    uVar1 = 1;
  }
  else if (DAT_00149988 == 2) {
    uVar1 = CONCAT31((int3)((uint)DAT_001499c0 >> 8),DAT_001499c0 == 0);
  }
  else {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: digest_mode @ 0xfe38

undefined8
digest_mode(uint *param_1,undefined8 param_2,undefined8 param_3,long param_4,char param_5)

{
  char cVar1;
  int iVar2;
  
  if (*param_1 == 0) {
    if (param_5 == '\0') {
      iVar2 = FUN_0010f27d(param_2,param_3,param_4);
      if (iVar2 != 0) {
        return 0;
      }
      *param_1 = *(uint *)(param_4 + 0x18);
      DAT_001499c8 = *param_1;
      DAT_001499c5 = 1;
    }
    else {
      DAT_001499c4 = 0;
      DAT_001499c5 = 0;
      DAT_001499c8 = 0;
    }
  }
  else if (((*param_1 & 0xf000) == 0xa000) && (cVar1 = FUN_0010fe03(), cVar1 != '\0')) {
    iVar2 = FUN_0010f27d(param_2,param_3,param_4);
    if (iVar2 != 0) {
      return 0;
    }
    DAT_001499c8 = *(uint *)(param_4 + 0x18);
    *param_1 = DAT_001499c8;
    DAT_001499c5 = 1;
  }
  else {
    DAT_001499c5 = 1;
    DAT_001499c8 = *param_1;
    *(uint *)(param_4 + 0x18) = DAT_001499c8;
  }
  return 1;
}




// Function: default_prints @ 0xff58

undefined8 default_prints(long param_1)

{
  long local_10;
  
  local_10 = param_1;
  while( true ) {
    if (local_10 == 0) {
      return 1;
    }
    if (*(char *)(local_10 + 0x19) != '\0') break;
    local_10 = *(long *)(local_10 + 0x108);
  }
  return 0;
}




// Function: looks_like_expression @ 0xff96

bool looks_like_expression(undefined1 *param_1,char param_2)

{
  bool bVar1;
  
  switch(*param_1) {
  case 0x21:
  case 0x28:
    if (param_1[1] == '\0') {
      bVar1 = true;
    }
    else {
      bVar1 = false;
    }
    break;
  default:
    bVar1 = false;
    break;
  case 0x29:
  case 0x2c:
    if (param_1[1] == '\0') {
      bVar1 = param_2 == '\0';
    }
    else {
      bVar1 = false;
    }
    break;
  case 0x2d:
    if (param_1[1] == '\0') {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
  }
  return bVar1;
}




// Function: process_debug_options @ 0x1004b

void process_debug_options(char *param_1)

{
  bool bVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  char *local_40;
  char *local_38;
  ulong local_30;
  char local_22 [2];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_40 = (char *)0x0;
  local_22[0] = ',';
  local_22[1] = '\0';
  bVar1 = true;
  local_38 = strtok_r(param_1,local_22,&local_40);
  do {
    if (local_38 == (char *)0x0) {
      if (bVar1) {
        uVar4 = gettext("Empty argument to the -D option.");
        error(0,0,uVar4);
                    /* WARNING: Subroutine does not return */
        FUN_0010f071(1);
      }
      if ((DAT_00149980 & 0x10) != 0) {
        FUN_0010ef55(1);
                    /* WARNING: Subroutine does not return */
        exit(0);
      }
      if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return;
    }
    bVar1 = false;
    for (local_30 = 0; local_30 < 9; local_30 = local_30 + 1) {
      iVar2 = strcmp((&PTR_DAT_001495a0)[local_30 * 3],local_38);
      if (iVar2 == 0) {
        DAT_00149980 = (long)*(int *)(&DAT_001495a8 + local_30 * 0x18) | DAT_00149980;
        break;
      }
    }
    if (8 < local_30) {
      uVar4 = FUN_0012f861(0,DAT_001499ac,param_1);
      uVar3 = gettext("Ignoring unrecognised debug flag %s");
      error(0,0,uVar3,uVar4);
    }
    local_38 = strtok_r((char *)0x0,local_22,&local_40);
  } while( true );
}




// Function: process_optimisation_option @ 0x101fd

void process_optimisation_option(byte *param_1)

{
  int iVar1;
  undefined8 uVar2;
  ushort **ppuVar3;
  int *piVar4;
  long in_FS_OFFSET;
  byte *local_30;
  ulong local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (*param_1 == 0) {
    uVar2 = gettext("The -O option must be immediately followed by a decimal integer");
    error(1,0,uVar2);
  }
  ppuVar3 = __ctype_b_loc();
  if (((*ppuVar3)[*param_1] & 0x800) == 0) {
    uVar2 = gettext("Please specify a decimal number immediately after -O");
    error(1,0,uVar2);
  }
  piVar4 = __errno_location();
  iVar1 = *piVar4;
  piVar4 = __errno_location();
  *piVar4 = 0;
  local_28 = strtoul((char *)param_1,(char **)&local_30,10);
  if ((local_28 == 0) && (param_1 == local_30)) {
    uVar2 = gettext("Please specify a decimal number immediately after -O");
    error(1,0,uVar2);
  }
  if (*local_30 != 0) {
    uVar2 = gettext("Invalid optimisation level %s");
    error(1,0,uVar2,param_1);
  }
  if (local_28 == 0xffffffffffffffff) {
    piVar4 = __errno_location();
    if (*piVar4 != 0) {
      uVar2 = gettext("Invalid optimisation level %s");
      piVar4 = __errno_location();
      error(1,*piVar4,uVar2,param_1);
    }
  }
  if (0xffff < local_28) {
    uVar2 = gettext(
                   "Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate."
                   );
    error(1,0,uVar2,local_28);
  }
  DAT_001499a8 = (undefined2)local_28;
  piVar4 = __errno_location();
  *piVar4 = iVar1;
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: process_leading_options @ 0x103e8

int process_leading_options(int param_1,long param_2)

{
  int iVar1;
  undefined8 uVar2;
  int local_10;
  
  local_10 = 1;
  do {
    if (param_1 <= local_10) {
      return local_10;
    }
    iVar1 = strcmp("-H",*(char **)(param_2 + (long)local_10 * 8));
    if (iVar1 == 0) {
      FUN_001141b0(2);
    }
    else {
      iVar1 = strcmp("-L",*(char **)(param_2 + (long)local_10 * 8));
      if (iVar1 == 0) {
        FUN_001141b0(1);
      }
      else {
        iVar1 = strcmp("-P",*(char **)(param_2 + (long)local_10 * 8));
        if (iVar1 == 0) {
          FUN_001141b0(0);
        }
        else {
          iVar1 = strcmp("--",*(char **)(param_2 + (long)local_10 * 8));
          if (iVar1 == 0) {
            return local_10 + 1;
          }
          iVar1 = strcmp("-D",*(char **)(param_2 + (long)local_10 * 8));
          if (iVar1 == 0) {
            if (param_1 <= local_10 + 1) {
              uVar2 = gettext("Missing argument after the -D option.");
              error(0,0,uVar2);
                    /* WARNING: Subroutine does not return */
              FUN_0010f071(1);
            }
            FUN_0011004b(*(undefined8 *)(param_2 + ((long)local_10 + 1) * 8));
            local_10 = local_10 + 1;
          }
          else {
            iVar1 = strncmp("-O",*(char **)(param_2 + (long)local_10 * 8),2);
            if (iVar1 != 0) {
              return local_10;
            }
            FUN_001101fd(*(long *)(param_2 + (long)local_10 * 8) + 2);
          }
        }
      }
    }
    local_10 = local_10 + 1;
  } while( true );
}




// Function: now @ 0x10601

time_t now(void)

{
  int iVar1;
  long in_FS_OFFSET;
  timeval local_28;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = gettimeofday(&local_28,(__timezone_ptr_t)0x0);
  if ((iVar1 != 0) && (local_28.tv_sec = time((time_t *)0x0), local_28.tv_sec == -1)) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("t != (time_t)-1","util.c",0x3d2,"now");
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_28.tv_sec;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: set_option_defaults @ 0x106bd

void set_option_defaults(undefined1 *param_1)

{
  undefined1 uVar1;
  int iVar2;
  char *pcVar3;
  undefined8 uVar4;
  undefined1 auVar5 [16];
  
  pcVar3 = getenv("POSIXLY_CORRECT");
  if (pcVar3 == (char *)0x0) {
    param_1[0x11] = 0;
  }
  else {
    param_1[0x11] = 1;
  }
  uVar1 = FUN_0010f45a();
  param_1[0x58] = uVar1;
  *(undefined4 *)(param_1 + 0x5c) = 0;
  iVar2 = isatty(0);
  if (iVar2 == 0) {
    param_1[0x10] = 0;
    param_1[0xf] = 0;
  }
  else {
    param_1[0x10] = 1;
    param_1[0xf] = 0;
  }
  if (param_1[0x11] != '\0') {
    param_1[0x10] = 0;
  }
  *param_1 = 1;
  param_1[1] = 0;
  *(undefined4 *)(param_1 + 8) = 0xffffffff;
  *(undefined4 *)(param_1 + 4) = *(undefined4 *)(param_1 + 8);
  auVar5 = FUN_00110601();
  *(undefined1 (*) [16])(param_1 + 0x18) = auVar5;
  *(long *)(param_1 + 0x28) = *(long *)(param_1 + 0x18) + -0x15180;
  *(undefined8 *)(param_1 + 0x30) = *(undefined8 *)(param_1 + 0x20);
  param_1[0x38] = 0;
  param_1[0xd] = 0;
  param_1[0xe] = 0;
  if (param_1[0x11] == '\0') {
    *(undefined4 *)(param_1 + 0x3c) = 0x400;
  }
  else {
    *(undefined4 *)(param_1 + 0x3c) = 0x200;
  }
  *(undefined8 *)(param_1 + 0x40) = 0;
  *(undefined2 *)(param_1 + 0x68) = 2;
  pcVar3 = getenv("FIND_BLOCK_SIZE");
  if (pcVar3 != (char *)0x0) {
    uVar4 = gettext(
                   "The environment variable FIND_BLOCK_SIZE is not supported, the only thing that affects the block size is the POSIXLY_CORRECT environment variable"
                   );
    error(1,0,uVar4);
  }
  param_1[0xc] = 0;
  FUN_001141b0(0);
  *(undefined4 *)(param_1 + 0x6c) = 8;
  *(undefined8 *)(param_1 + 0x70) = 0;
  param_1[0x78] = 0;
  return;
}




// Function: apply_predicate @ 0x10873

bool apply_predicate(undefined8 param_1,undefined8 param_2,undefined8 *param_3)

{
  char cVar1;
  int iVar2;
  
  param_3[0x24] = param_3[0x24] + 1;
  if ((((*(char *)((long)param_3 + 0x1a) != '\0') || (*(char *)((long)param_3 + 0x1b) != '\0')) ||
      (*(char *)((long)param_3 + 0x1c) != '\0')) &&
     (iVar2 = FUN_0010f388(param_1,param_2,param_3), iVar2 != 0)) {
    return false;
  }
  cVar1 = (*(code *)*param_3)(param_1,param_2,param_3);
  if (cVar1 != '\0') {
    param_3[0x25] = param_3[0x25] + 1;
  }
  return cVar1 != '\0';
}




// Function: is_exec_in_local_dir @ 0x10933

undefined4 is_exec_in_local_dir(code *param_1)

{
  undefined4 uVar1;
  
  if ((param_1 == FUN_00109497) || (param_1 == FUN_00109f27)) {
    uVar1 = 1;
  }
  else {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: safely_quote_err_filename @ 0x1096a

void safely_quote_err_filename(undefined4 param_1,undefined8 param_2)

{
  FUN_0012f861(param_1,DAT_001499ac,param_2);
  return;
}




// Function: report_file_err @ 0x10995

void report_file_err(undefined4 param_1,undefined4 param_2,char param_3,undefined8 param_4)

{
  undefined8 uVar1;
  
  if ((param_3 != '\x01') || (DAT_001499f8 != '\x01')) {
    uVar1 = FUN_0011096a(0,param_4);
    error(param_1,param_2,&DAT_0013a3d2,uVar1);
    DAT_001499e4 = 1;
  }
  if (param_3 != '\0') {
    DAT_001499f8 = '\x01';
  }
  return;
}




// Function: nonfatal_target_file_error @ 0x10a13

void nonfatal_target_file_error(undefined4 param_1,undefined8 param_2)

{
  FUN_00110995(0,param_1,1,param_2);
  return;
}




// Function: fatal_target_file_error @ 0x10a44

void fatal_target_file_error(undefined4 param_1,undefined8 param_2)

{
  FUN_00110995(1,param_1,1,param_2);
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: nonfatal_nontarget_file_error @ 0x10a77

void nonfatal_nontarget_file_error(undefined4 param_1,undefined8 param_2)

{
  FUN_00110995(0,param_1,0,param_2);
  return;
}




// Function: fatal_nontarget_file_error @ 0x10aa8

void fatal_nontarget_file_error(undefined4 param_1,undefined8 param_2)

{
  DAT_001499f8 = 0;
  FUN_00110995(1,param_1,0,param_2);
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: entry_comparator @ 0x10ae2

undefined4 entry_comparator(long *param_1,long *param_2)

{
  undefined4 uVar1;
  
  if ((param_1[1] == param_2[1]) && (*param_1 == *param_2)) {
    uVar1 = 1;
  }
  else {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: entry_free @ 0x10b3b

void entry_free(void *param_1)

{
  undefined8 uVar1;
  int iVar2;
  int *piVar3;
  
  if (*(long *)((long)param_1 + 0x18) != 0) {
    iVar2 = FUN_0011dad5(*(undefined8 *)((long)param_1 + 0x18));
    if (iVar2 != 0) {
      uVar1 = *(undefined8 *)((long)param_1 + 0x10);
      piVar3 = __errno_location();
      FUN_00110aa8(*piVar3,uVar1);
    }
  }
  free(*(void **)((long)param_1 + 0x10));
  free(param_1);
  return;
}




// Function: entry_hashfunc @ 0x10bb1

ulong entry_hashfunc(ulong *param_1,ulong param_2)

{
  return (param_1[1] ^ *param_1) % param_2;
}




// Function: sharefile_init @ 0x10be9

long * sharefile_init(char *param_1)

{
  long *__ptr;
  char *pcVar1;
  long lVar2;
  
  __ptr = malloc(0x10);
  if (__ptr != (long *)0x0) {
    pcVar1 = strdup(param_1);
    *__ptr = (long)pcVar1;
    if (*__ptr == 0) {
      free(__ptr);
    }
    else {
      lVar2 = FUN_00121bb1(0xb,0,FUN_00110bb1,FUN_00110ae2,FUN_00110b3b);
      __ptr[1] = lVar2;
      if (__ptr[1] != 0) {
        return __ptr;
      }
      free((void *)*__ptr);
      free(__ptr);
    }
  }
  return (long *)0x0;
}




// Function: sharefile_destroy @ 0x10cac

void sharefile_destroy(undefined8 *param_1)

{
  free((void *)*param_1);
  FUN_00121e31(param_1[1]);
  return;
}




// Function: sharefile_fopen @ 0x10ce6

__dev_t sharefile_fopen(undefined8 *param_1,char *param_2)

{
  int iVar1;
  __dev_t *__ptr;
  __dev_t _Var2;
  char *pcVar3;
  long lVar4;
  int *piVar5;
  long in_FS_OFFSET;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  __ptr = malloc(0x20);
  if (__ptr == (__dev_t *)0x0) {
    _Var2 = 0;
  }
  else {
    pcVar3 = strdup(param_2);
    __ptr[2] = (__dev_t)pcVar3;
    if (__ptr[2] == 0) {
      free(__ptr);
      _Var2 = 0;
    }
    else {
      _Var2 = FUN_0011e49e(param_2,*param_1);
      __ptr[3] = _Var2;
      if (__ptr[3] == 0) {
        FUN_00110b3b(__ptr);
        _Var2 = 0;
      }
      else {
        iVar1 = fileno((FILE *)__ptr[3]);
        if (iVar1 < 0) {
                    /* WARNING: Subroutine does not return */
          __assert_fail("fd >= 0","sharefile.c",0xa6,"sharefile_fopen");
        }
        FUN_0011d5ba(iVar1,1);
        iVar1 = fstat(iVar1,&local_a8);
        if (iVar1 < 0) {
          FUN_00110b3b(__ptr);
          _Var2 = 0;
        }
        else {
          *__ptr = local_a8.st_dev;
          __ptr[1] = local_a8.st_ino;
          lVar4 = FUN_001214fc(param_1[1],__ptr);
          if (lVar4 == 0) {
            lVar4 = FUN_001228f2(param_1[1],__ptr);
            if (lVar4 == 0) {
              piVar5 = __errno_location();
              iVar1 = *piVar5;
              FUN_00110b3b(__ptr);
              piVar5 = __errno_location();
              *piVar5 = iVar1;
              _Var2 = 0;
            }
            else {
              _Var2 = __ptr[3];
            }
          }
          else {
            FUN_00110b3b(__ptr);
            _Var2 = *(__dev_t *)(lVar4 + 0x18);
          }
        }
      }
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return _Var2;
}




// Function: make_segment @ 0x10f4b

/* WARNING: Removing unreachable block (ram,0x00111122) */

long make_segment(long *param_1,char *param_2,int param_3,int param_4,char param_5,char param_6,
                 long param_7)

{
  char *pcVar1;
  long lVar2;
  undefined8 uVar3;
  uint local_24;
  char *local_20;
  
  local_24 = 0;
  if (param_5 == '{') {
                    /* WARNING: Subroutine does not return */
    __assert_fail("format_char != \'{\'","print.c",0x4e,"make_segment");
  }
  if (param_5 == '[') {
                    /* WARNING: Subroutine does not return */
    __assert_fail("format_char != \'[\'","print.c",0x4f,"make_segment");
  }
  if (param_5 == '(') {
                    /* WARNING: Subroutine does not return */
    __assert_fail("format_char != \'(\'","print.c",0x50,"make_segment");
  }
  lVar2 = FUN_00131bc2(0x20);
  *param_1 = lVar2;
  *(int *)*param_1 = param_4;
  *(char *)(*param_1 + 4) = param_5;
  *(char *)(*param_1 + 5) = param_6;
  *(undefined8 *)(*param_1 + 0x18) = 0;
  *(int *)(*param_1 + 0x10) = param_3;
  lVar2 = *param_1;
  uVar3 = FUN_00131bc2((long)param_3 + 2);
  *(undefined8 *)(lVar2 + 8) = uVar3;
  pcVar1 = *(char **)(lVar2 + 8);
  strncpy(pcVar1,param_2,(long)param_3);
  pcVar1 = pcVar1 + param_3;
  if ((param_4 == 0) || (param_4 == 1)) {
    if (param_5 != '\0') {
                    /* WARNING: Subroutine does not return */
      __assert_fail("0 == format_char","print.c",0x61,"make_segment");
    }
    if (param_6 != '\0') {
                    /* WARNING: Subroutine does not return */
      __assert_fail("0 == aux_format_char","print.c",0x62,"make_segment");
    }
    *pcVar1 = '\0';
    lVar2 = *param_1;
  }
  else {
    if (param_4 != 2) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("kind == KIND_FORMAT","print.c",0x69,"make_segment");
    }
    local_20 = pcVar1;
    switch(param_5) {
    case '%':
      local_20 = pcVar1 + 1;
      *pcVar1 = '%';
      break;
    case 'A':
    case 'B':
    case 'C':
    case 'F':
    case 'M':
    case 'T':
    case 'a':
    case 'c':
    case 'g':
    case 's':
    case 't':
    case 'u':
      *(undefined1 *)(param_7 + 0x1a) = 1;
      local_24 = 3;
      local_20 = pcVar1 + 1;
      *pcVar1 = 's';
      break;
    case 'D':
    case 'G':
    case 'U':
    case 'b':
    case 'k':
    case 'n':
      *(undefined1 *)(param_7 + 0x1a) = 1;
      local_24 = 3;
      local_20 = pcVar1 + 1;
      *pcVar1 = 's';
      break;
    case 'H':
      local_20 = pcVar1 + 1;
      *pcVar1 = 's';
      break;
    case 'P':
    case 'f':
    case 'h':
    case 'p':
      local_20 = pcVar1 + 1;
      *pcVar1 = 's';
      break;
    case 'S':
      *(undefined1 *)(param_7 + 0x1a) = 1;
      local_24 = 3;
      local_20 = pcVar1 + 1;
      *pcVar1 = 'g';
      break;
    case 'Y':
      *(undefined1 *)(param_7 + 0x1a) = 1;
      local_24 = 2;
      local_20 = pcVar1 + 1;
      *pcVar1 = 's';
      break;
    case 'Z':
      local_24 = 5;
      local_20 = pcVar1 + 1;
      *pcVar1 = 's';
      break;
    case 'd':
      local_20 = pcVar1 + 1;
      *pcVar1 = 'd';
      break;
    case 'i':
      *(undefined1 *)(param_7 + 0x1c) = 1;
      local_24 = 1;
      local_20 = pcVar1 + 1;
      *pcVar1 = 's';
      break;
    case 'l':
      *(undefined1 *)(param_7 + 0x1a) = 1;
      local_24 = 4;
      local_20 = pcVar1 + 1;
      *pcVar1 = 's';
      break;
    case 'm':
      local_20 = pcVar1 + 1;
      *pcVar1 = 'o';
      *(undefined1 *)(param_7 + 0x1a) = 1;
      local_24 = 3;
      break;
    case 'y':
      *(undefined1 *)(param_7 + 0x1b) = 1;
      local_24 = 2;
      local_20 = pcVar1 + 1;
      *pcVar1 = 's';
    }
    *local_20 = '\0';
    if (*(uint *)(param_7 + 0x20) < local_24) {
      *(uint *)(param_7 + 0x20) = local_24;
    }
    lVar2 = *param_1;
  }
  return lVar2 + 0x18;
}




// Function: is_octal_char @ 0x1133c

undefined4 is_octal_char(char param_1)

{
  undefined4 uVar1;
  
  if ((param_1 < '0') || ('7' < param_1)) {
    uVar1 = 0;
  }
  else {
    uVar1 = 1;
  }
  return uVar1;
}




// Function: parse_octal_escape @ 0x11366

int parse_octal_escape(long param_1,long *param_2)

{
  char cVar1;
  int iVar2;
  int iVar3;
  undefined8 local_20;
  
  local_20 = 0;
  iVar2 = 0;
  for (iVar3 = 0; iVar3 < 3; iVar3 = iVar3 + 1) {
    cVar1 = FUN_0011133c((int)*(char *)(local_20 + param_1));
    if (cVar1 == '\0') break;
    iVar2 = (int)*(char *)(local_20 + param_1) + iVar2 * 8 + -0x30;
    local_20 = local_20 + 1;
  }
  *param_2 = local_20 + -1;
  return iVar2;
}




// Function: parse_escape_char @ 0x113f2

undefined1 parse_escape_char(undefined1 param_1)

{
  undefined1 local_9;
  
  local_9 = 0;
  switch(param_1) {
  case 0x5c:
    local_9 = 0x5c;
    break;
  case 0x61:
    local_9 = 7;
    break;
  case 0x62:
    local_9 = 8;
    break;
  case 0x66:
    local_9 = 0xc;
    break;
  case 0x6e:
    local_9 = 10;
    break;
  case 0x72:
    local_9 = 0xd;
    break;
  case 0x74:
    local_9 = 9;
    break;
  case 0x76:
    local_9 = 0xb;
  }
  return local_9;
}




// Function: get_format_flags_length @ 0x11467

long get_format_flags_length(long param_1)

{
  char *pcVar1;
  ushort **ppuVar2;
  long local_10;
  
  local_10 = 0;
  do {
    local_10 = local_10 + 1;
    if (*(char *)(local_10 + param_1) == '\0') break;
    pcVar1 = strchr("-+ #",(int)*(char *)(local_10 + param_1));
  } while (pcVar1 != (char *)0x0);
  while (ppuVar2 = __ctype_b_loc(), ((*ppuVar2)[*(byte *)(local_10 + param_1)] & 0x800) != 0) {
    local_10 = local_10 + 1;
  }
  if (*(char *)(local_10 + param_1) == '.') {
    do {
      local_10 = local_10 + 1;
      ppuVar2 = __ctype_b_loc();
    } while (((*ppuVar2)[*(byte *)(local_10 + param_1)] & 0x800) != 0);
  }
  return local_10;
}




// Function: get_format_specifer_length @ 0x11545

undefined8 get_format_specifer_length(char param_1)

{
  char *pcVar1;
  undefined8 uVar2;
  
  pcVar1 = strchr("abcdDfFgGhHiklmMnpPsStuUyYZ%",(int)param_1);
  if (pcVar1 == (char *)0x0) {
    pcVar1 = strchr("ABCT",(int)param_1);
    if (pcVar1 == (char *)0x0) {
      uVar2 = 0;
    }
    else {
      uVar2 = 2;
    }
  }
  else {
    uVar2 = 1;
  }
  return uVar2;
}




// Function: insert_fprintf @ 0x1159f

undefined8 insert_fprintf(undefined8 *param_1,undefined8 param_2,char *param_3)

{
  char cVar1;
  undefined8 uVar2;
  char *pcVar3;
  long in_FS_OFFSET;
  long local_58;
  char *local_50;
  char *local_48;
  undefined8 *local_40;
  long local_38;
  long local_30;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_50 = param_3;
  local_28 = FUN_0010ee5a(param_2,FUN_0011376d,param_3);
  *(undefined1 *)(local_28 + 0x19) = 1;
  *(undefined1 *)(local_28 + 0x18) = *(undefined1 *)(local_28 + 0x19);
  uVar2 = param_1[1];
  *(undefined8 *)(local_28 + 0x38) = *param_1;
  *(undefined8 *)(local_28 + 0x40) = uVar2;
  uVar2 = param_1[3];
  *(undefined8 *)(local_28 + 0x48) = param_1[2];
  *(undefined8 *)(local_28 + 0x50) = uVar2;
  *(undefined8 *)(local_28 + 0x58) = param_1[4];
  *(undefined1 *)(local_28 + 0x1b) = 0;
  *(undefined1 *)(local_28 + 0x1a) = 0;
  *(undefined4 *)(local_28 + 0x20) = 0;
  local_40 = (undefined8 *)(local_28 + 0x38);
  *local_40 = 0;
  local_48 = local_50;
  do {
    if (*local_48 == '\0') {
      if (local_50 < local_48) {
        FUN_00110f4b(local_40,local_50,(int)local_48 - (int)local_50,0,0,0,local_28);
      }
LAB_00111afa:
      if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
        return 1;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    if ((*local_48 == '\\') && (local_48[1] == 'c')) {
      FUN_00110f4b(local_40,local_50,(int)local_48 - (int)local_50,1,0,0,local_28);
      if ((*(char *)(local_28 + 0x1a) != '\0') && (*(uint *)(local_28 + 0x20) < 3)) {
        *(undefined4 *)(local_28 + 0x20) = 3;
      }
      goto LAB_00111afa;
    }
    if (*local_48 == '\\') {
      local_38 = 1;
      if (local_48[1] == '\0') {
        uVar2 = gettext("warning: escape `\\\' followed by nothing at all");
        error(0,0,uVar2);
        local_38 = local_38 + -1;
      }
      else {
        cVar1 = FUN_0011133c((int)local_48[1]);
        if (cVar1 == '\0') {
          cVar1 = FUN_001113f2((int)local_48[local_38]);
          if (cVar1 == '\0') {
            cVar1 = local_48[local_38];
            uVar2 = gettext("warning: unrecognized escape `\\%c\'");
            error(0,0,uVar2,(int)cVar1);
            local_48 = local_48 + local_38;
            goto LAB_00111aa1;
          }
          *local_48 = cVar1;
        }
        else {
          local_58 = 0;
          cVar1 = FUN_00111366(local_48 + local_38,&local_58);
          *local_48 = cVar1;
          local_38 = local_38 + local_58;
        }
      }
      local_40 = (undefined8 *)
                 FUN_00110f4b(local_40,local_50,((int)local_48 - (int)local_50) + 1,0,0,0,local_28);
      local_50 = local_48 + local_38 + 1;
      local_48 = local_48 + local_38;
    }
    else if (*local_48 == '%') {
      if (local_48[1] == '\0') {
        uVar2 = gettext("error: %s at end of format string");
        error(1,0,uVar2,local_48);
      }
      if (local_48[1] == '%') {
        local_30 = 1;
      }
      else {
        local_30 = FUN_00111467(local_48);
      }
      local_48 = local_48 + local_30;
      local_30 = FUN_00111545((int)*local_48);
      if ((local_30 == 0) || (local_48[local_30 + -1] == '\0')) {
        pcVar3 = strchr("{[(",(int)*local_48);
        if (pcVar3 != (char *)0x0) {
          cVar1 = *local_48;
          uVar2 = gettext("error: the format directive `%%%c\' is reserved for future use");
          error(1,0,uVar2,(int)cVar1);
        }
        if ((local_30 == 2) && (local_48[1] == '\0')) {
          cVar1 = *local_48;
          uVar2 = gettext(
                         "warning: format directive `%%%c\' should be followed by another character"
                         );
          error(0,0,uVar2,(int)cVar1);
        }
        else {
          cVar1 = *local_48;
          uVar2 = gettext("warning: unrecognized format directive `%%%c\'");
          error(0,0,uVar2,(int)cVar1);
        }
        local_40 = (undefined8 *)
                   FUN_00110f4b(local_40,local_50,((int)local_48 + 1) - (int)local_50,0,0,0,local_28
                               );
      }
      else {
        if (local_30 == 2) {
          cVar1 = local_48[1];
        }
        else {
          cVar1 = '\0';
        }
        local_40 = (undefined8 *)
                   FUN_00110f4b(local_40,local_50,(int)local_48 - (int)local_50,2,(int)*local_48,
                                (int)cVar1,local_28);
        local_48 = local_48 + local_30 + -1;
      }
      local_50 = local_48 + 1;
    }
LAB_00111aa1:
    local_48 = local_48 + 1;
  } while( true );
}




// Function: scan_for_digit_differences @ 0x11b14

undefined8 scan_for_digit_differences(long param_1,long param_2,long *param_3,long *param_4)

{
  bool bVar1;
  ushort **ppuVar2;
  undefined8 uVar3;
  long local_10;
  
  bVar1 = false;
  local_10 = 0;
  do {
    if ((*(char *)(local_10 + param_1) == '\0') || (*(char *)(local_10 + param_2) == '\0')) {
      if ((*(char *)(local_10 + param_1) == '\0') && (*(char *)(local_10 + param_2) == '\0')) {
        uVar3 = 1;
      }
      else {
        uVar3 = 0;
      }
      return uVar3;
    }
    if (*(char *)(local_10 + param_1) != *(char *)(local_10 + param_2)) {
      ppuVar2 = __ctype_b_loc();
      if ((((*ppuVar2)[*(byte *)(local_10 + param_1)] & 0x800) == 0) ||
         (ppuVar2 = __ctype_b_loc(), ((*ppuVar2)[*(byte *)(local_10 + param_2)] & 0x800) == 0)) {
        return 0;
      }
      if (bVar1) {
        if (local_10 - *param_3 != *param_4) {
          return 0;
        }
        *param_4 = *param_4 + 1;
      }
      else {
        *param_3 = local_10;
        *param_4 = 1;
        bVar1 = true;
      }
    }
    local_10 = local_10 + 1;
  } while( true );
}




// Function: do_time_format @ 0x11c89

long do_time_format(char *param_1,tm *param_2,void *param_3,size_t param_4)

{
  undefined4 uVar1;
  undefined4 uVar2;
  char cVar3;
  size_t sVar4;
  ushort **ppuVar5;
  long in_FS_OFFSET;
  long local_98;
  long local_90;
  char *local_88;
  size_t local_80;
  size_t local_78;
  char *local_70;
  long local_68;
  size_t local_60;
  tm local_58;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_88 = (char *)0x0;
  sVar4 = strlen(param_1);
  local_88 = (char *)FUN_00131bc2(sVar4 + 2);
  *local_88 = '_';
  sVar4 = strlen(param_1);
  memcpy(local_88 + 1,param_1,sVar4 + 1);
  uVar1 = param_2->tm_sec;
  uVar2 = param_2->tm_min;
  local_58.tm_hour = param_2->tm_hour;
  local_58.tm_mday = param_2->tm_mday;
  local_58.tm_mon = param_2->tm_mon;
  local_58.tm_year = param_2->tm_year;
  local_58.tm_wday = param_2->tm_wday;
  local_58.tm_yday = param_2->tm_yday;
  local_58.tm_isdst = param_2->tm_isdst;
  local_58._36_4_ = *(undefined4 *)&param_2->field_0x24;
  local_58.tm_gmtoff = param_2->tm_gmtoff;
  local_58.tm_zone = param_2->tm_zone;
  if ((int)uVar1 < 0xb) {
    local_58.tm_sec = uVar1 + 0xb;
  }
  else {
    local_58.tm_sec = uVar1 + -0xb;
  }
  local_58.tm_min = uVar2;
  if (DAT_00149a60 == (char *)0x0) {
    DAT_00149a68 = 1;
    DAT_00149a60 = (char *)FUN_00131bc2(1);
  }
  while( true ) {
    local_80 = strftime(DAT_00149a60,DAT_00149a68,local_88,param_2);
    if ((local_80 != 0) && (local_80 < DAT_00149a68)) break;
    DAT_00149a60 = (char *)FUN_00131dbf(DAT_00149a60,&DAT_00149a68,1);
  }
  local_98 = 0;
  local_90 = 0;
  local_78 = param_4 + local_80 + 1;
  DAT_00149a60 = (char *)FUN_00131c2c(DAT_00149a60,local_78);
  DAT_00149a68 = local_78;
  local_70 = (char *)FUN_00131bc2(local_78);
  strftime(local_70,DAT_00149a68,local_88,&local_58);
  cVar3 = FUN_00111b14(DAT_00149a60,local_70,&local_98,&local_90);
  if ((cVar3 != '\0') && (local_90 == 2)) {
    ppuVar5 = __ctype_b_loc();
    if (((*ppuVar5)[*(byte *)((long)DAT_00149a60 + local_90 + local_98)] & 0x800) == 0) {
      local_68 = local_90 + local_98;
      local_60 = (local_80 - local_68) + 1;
      if (local_78 != local_60 + local_68 + param_4) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("end_of_seconds + ns_size + suffix_len == final_len","print.c",0x225,
                      "do_time_format");
      }
      memmove((void *)((long)DAT_00149a60 + param_4 + local_68),
              (void *)((long)DAT_00149a60 + local_68),local_60);
      memcpy((void *)((long)DAT_00149a60 + local_90 + local_98),param_3,param_4);
    }
  }
  free(local_88);
  free(local_70);
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return (long)DAT_00149a60 + 1;
}




// Function: format_date @ 0x1201f

char * format_date(long param_1,undefined8 param_2,int param_3)

{
  long lVar1;
  size_t sVar2;
  char *pcVar3;
  long in_FS_OFFSET;
  long local_a8;
  undefined8 local_a0;
  int local_98;
  int local_94;
  char *local_90;
  tm *local_88;
  char *local_80;
  long local_78;
  size_t local_70;
  char *local_68;
  ulong local_60;
  char local_54;
  undefined1 local_53;
  undefined1 local_52;
  char local_48 [40];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_98 = 0;
  local_94 = 0;
  local_a8 = param_1;
  local_a0 = param_2;
  if (param_3 == 0x2b) {
    strcpy(&local_54,"%Y-%m-%d+%T");
    local_94 = 1;
  }
  else {
    local_54 = '%';
    local_53 = (undefined1)param_3;
    local_52 = 0;
    if ((param_3 - 0x40U < 0x19) && ((0x1180001UL >> ((byte)(param_3 - 0x40U) & 0x3f) & 1) != 0)) {
      local_94 = 1;
    }
    else {
      local_94 = 0;
    }
  }
  if (local_94 == 0) {
    local_98 = 0;
    local_48[0] = '\0';
  }
  else {
    local_98 = snprintf(local_48,0x20,".%09ld0",local_a0);
    if (0x1f < local_98) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("charsprinted < NS_BUF_LEN","print.c",0x299,"format_date");
    }
  }
  if (((param_3 == 0x40) || (local_88 = localtime(&local_a8), local_88 == (tm *)0x0)) ||
     (local_80 = (char *)FUN_00111c89(&local_54,local_88,local_48,(long)local_98), pcVar3 = local_80
     , local_80 == (char *)0x0)) {
    local_78 = local_a8;
    lVar1 = local_a8;
    if (local_a8 < 0) {
      lVar1 = -local_a8;
    }
    local_90 = (char *)FUN_00122eb1(lVar1,&DAT_00149a81,0,1,1);
    if (local_90 < &DAT_00149a81) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("p > buf","print.c",0x2b9,"format_date");
    }
    if ((char *)0x149d96 < local_90) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("p < (buf + (sizeof buf))","print.c",0x2ba,"format_date");
    }
    if (local_a8 < 0) {
      local_90 = local_90 + -1;
      *local_90 = '-';
    }
    pcVar3 = local_90;
    if (local_94 != 0) {
      local_70 = strlen(local_90);
      local_68 = local_90 + (local_70 - 0x149a80);
      if ((char *)0x316 < local_68) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("sizeof buf > used","print.c",0x2c7,"format_date");
      }
      local_60 = 0x316 - (long)local_68;
      sVar2 = strlen(local_48);
      if (local_60 <= sVar2) {
        error(0,0,"charsprinted=%ld but remaining=%lu: ns_buf=%s",(long)local_98,local_60,local_48);
      }
      sVar2 = strlen(local_48);
      if (local_60 <= sVar2) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("strlen (ns_buf) < remaining","print.c",0x2d0,"format_date");
      }
      strcat(local_90,local_48);
      pcVar3 = local_90;
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return pcVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: ctime_format @ 0x123c8

undefined * ctime_format(time_t param_1,undefined8 param_2)

{
  undefined *puVar1;
  time_t local_28;
  undefined8 local_20;
  int local_14;
  tm *local_10;
  
  local_28 = param_1;
  local_20 = param_2;
  local_10 = localtime(&local_28);
  if (local_10 == (tm *)0x0) {
    puVar1 = (undefined *)FUN_0011201f(local_28,local_20,0x40);
  }
  else {
    if (local_10->tm_wday < 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("ptm->tm_wday >= 0","print.c",0x2ed,"ctime_format");
    }
    if (6 < local_10->tm_wday) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("ptm->tm_wday < 7","print.c",0x2ee,"ctime_format");
    }
    if (local_10->tm_mon < 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("ptm->tm_mon >= 0","print.c",0x2ef,"ctime_format");
    }
    if (0xb < local_10->tm_mon) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("ptm->tm_mon < 12","print.c",0x2f0,"ctime_format");
    }
    if (local_10->tm_hour < 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("ptm->tm_hour >= 0","print.c",0x2f1,"ctime_format");
    }
    if (0x17 < local_10->tm_hour) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("ptm->tm_hour < 24","print.c",0x2f2,"ctime_format");
    }
    if (0x3b < local_10->tm_min) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("ptm->tm_min < 60","print.c",0x2f3,"ctime_format");
    }
    if (0x3d < local_10->tm_sec) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("ptm->tm_sec <= 61","print.c",0x2f4,"ctime_format");
    }
    local_14 = snprintf(&DAT_00149da0,0x400,"%3s %3s %2d %02d:%02d:%02d.%09ld0 %04d",
                        (&PTR_DAT_00149680)[local_10->tm_wday],(&PTR_DAT_001496c0)[local_10->tm_mon]
                        ,(ulong)(uint)local_10->tm_mday,(ulong)(uint)local_10->tm_hour,
                        (ulong)(uint)local_10->tm_min,(ulong)(uint)local_10->tm_sec,local_20,
                        (ulong)(local_10->tm_year + 0x76c));
    if (0x3ff < local_14) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("nout < TIME_BUF_LEN","print.c",0x302,"ctime_format");
    }
    puVar1 = &DAT_00149da0;
  }
  return puVar1;
}




// Function: file_sparseness @ 0x12693

double file_sparseness(long param_1)

{
  double dVar1;
  
  if (*(long *)(param_1 + 0x30) == 0) {
    dVar1 = DAT_0013aca8;
    if ((*(long *)(param_1 + 0x40) != 0) && (dVar1 = DAT_0013acb8, *(long *)(param_1 + 0x40) < 0)) {
      dVar1 = DAT_0013acb0;
    }
  }
  else {
    dVar1 = (DAT_0013acc0 * (double)*(long *)(param_1 + 0x40)) / (double)*(long *)(param_1 + 0x30);
  }
  return dVar1;
}




// Function: checked_fprintf @ 0x1272c

void checked_fprintf(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8,
                 long param_9,char *param_10,undefined8 param_11,undefined8 param_12,
                 undefined8 param_13,undefined8 param_14)

{
  undefined8 uVar1;
  char in_AL;
  int iVar2;
  int *piVar3;
  long in_FS_OFFSET;
  undefined4 local_e8;
  undefined4 local_e4;
  undefined1 *local_e0;
  undefined1 *local_d8;
  long local_d0;
  undefined1 local_c8 [16];
  undefined8 local_b8;
  undefined8 local_b0;
  undefined8 local_a8;
  undefined8 local_a0;
  undefined8 local_98;
  undefined8 local_88;
  undefined8 local_78;
  undefined8 local_68;
  undefined8 local_58;
  undefined8 local_48;
  undefined8 local_38;
  undefined8 local_28;
  
  if (in_AL != '\0') {
    local_98 = param_1;
    local_88 = param_2;
    local_78 = param_3;
    local_68 = param_4;
    local_58 = param_5;
    local_48 = param_6;
    local_38 = param_7;
    local_28 = param_8;
  }
  local_d0 = *(long *)(in_FS_OFFSET + 0x28);
  local_e8 = 0x10;
  local_e4 = 0x30;
  local_e0 = &stack0x00000008;
  local_d8 = local_c8;
  local_b8 = param_11;
  local_b0 = param_12;
  local_a8 = param_13;
  local_a0 = param_14;
  iVar2 = vfprintf(*(FILE **)(param_9 + 8),param_10,&local_e8);
  if (iVar2 < 0) {
    uVar1 = *(undefined8 *)(param_9 + 0x10);
    piVar3 = __errno_location();
    FUN_00110a77(*piVar3,uVar1);
  }
  if (local_d0 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: checked_print_quoted @ 0x12839

void checked_print_quoted(long param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 uVar1;
  int iVar2;
  int *piVar3;
  
  iVar2 = FUN_0011c643(*(undefined8 *)(param_1 + 8),*(undefined8 *)(param_1 + 0x20),
                       *(undefined1 *)(param_1 + 0x18),param_2,param_3);
  if (iVar2 < 0) {
    uVar1 = *(undefined8 *)(param_1 + 0x10);
    piVar3 = __errno_location();
    FUN_00110a77(*piVar3,uVar1);
  }
  return;
}




// Function: checked_fwrite @ 0x128a9

void checked_fwrite(void *param_1,size_t param_2,ulong param_3,long param_4)

{
  undefined8 uVar1;
  size_t sVar2;
  int *piVar3;
  
  sVar2 = fwrite(param_1,param_2,param_3,*(FILE **)(param_4 + 8));
  if (sVar2 < param_3) {
    uVar1 = *(undefined8 *)(param_4 + 0x10);
    piVar3 = __errno_location();
    FUN_00110a77(*piVar3,uVar1);
  }
  return;
}




// Function: checked_fflush @ 0x12910

void checked_fflush(long param_1)

{
  undefined8 uVar1;
  int iVar2;
  int *piVar3;
  
  iVar2 = FUN_0011e1a8(*(undefined8 *)(param_1 + 8));
  if (iVar2 != 0) {
    uVar1 = *(undefined8 *)(param_1 + 0x10);
    piVar3 = __errno_location();
    FUN_00110a77(*piVar3,uVar1);
  }
  return;
}




// Function: mode_to_filetype @ 0x12955

undefined * mode_to_filetype(int param_1)

{
  undefined *puVar1;
  
  if (param_1 == 0x8000) {
    puVar1 = &DAT_0013aa33;
  }
  else if (param_1 == 0x4000) {
    puVar1 = &DAT_0013aa35;
  }
  else if (param_1 == 0xa000) {
    puVar1 = &DAT_0013aa37;
  }
  else if (param_1 == 0xc000) {
    puVar1 = &DAT_0013aa39;
  }
  else if (param_1 == 0x6000) {
    puVar1 = &DAT_0013aa3b;
  }
  else if (param_1 == 0x2000) {
    puVar1 = &DAT_0013aa3d;
  }
  else if (param_1 == 0x1000) {
    puVar1 = &DAT_0013aa3f;
  }
  else {
    puVar1 = &DAT_0013aa41;
  }
  return puVar1;
}




// Function: do_fprintf @ 0x129e7

void do_fprintf(undefined8 param_1,uint *param_2,void *param_3,undefined8 *param_4)

{
  uint uVar1;
  int iVar2;
  size_t sVar3;
  int *piVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long in_FS_OFFSET;
  undefined1 auVar7 [16];
  undefined8 local_390;
  char *local_388;
  char *local_380;
  void *local_378;
  passwd *local_370;
  char *local_368;
  group *local_360;
  void *local_358;
  void *local_350;
  stat local_348;
  undefined1 local_2b8 [10];
  undefined1 local_2ae;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = *param_2;
  if (uVar1 == 2) {
    switch((char)param_2[1]) {
    case '\0':
      uVar5 = gettext("error: %s at end of format string");
      error(1,0,uVar5,&DAT_0013aa65);
      break;
    case '%':
      FUN_001128a9(*(undefined8 *)(param_2 + 2),1,(long)(int)param_2[4],param_1);
      break;
    case 'D':
      uVar5 = FUN_00122eb1(*param_4,local_2b8,0,1,1);
      FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),uVar5);
      break;
    case 'F':
      uVar5 = FUN_00113a18(param_4,param_3);
      FUN_00112839(param_1,*(undefined8 *)(param_2 + 2),uVar5);
      break;
    case 'H':
      local_350 = (void *)FUN_00131bc2((long)(DAT_001499dc + 1));
      memcpy(local_350,param_3,(long)DAT_001499dc);
      *(undefined1 *)((long)local_350 + (long)DAT_001499dc) = 0;
      FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),local_350);
      free(local_350);
      break;
    case 'M':
      FUN_0011e472(param_4,local_2b8);
      local_2ae = 0;
      FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),local_2b8);
      break;
    case 'P':
      if (DAT_001499c0 < 1) {
        local_388 = "";
      }
      else {
        local_388 = (char *)((long)param_3 + (long)DAT_001499dc);
        if (*local_388 == '/') {
          local_388 = local_388 + 1;
        }
      }
      FUN_00112839(param_1,*(undefined8 *)(param_2 + 2),local_388);
      break;
    case 'S':
      uVar5 = FUN_00112693(param_4);
      FUN_0011272c(uVar5,param_1,*(undefined8 *)(param_2 + 2));
      break;
    case 'Y':
      if ((*(uint *)(param_4 + 3) & 0xf000) == 0xa000) {
        iVar2 = fstatat(DAT_001499d8,DAT_001499d0,&local_348,0);
        if (iVar2 == 0) {
          uVar5 = FUN_00112955(local_348.st_mode & 0xf000);
          FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),uVar5);
        }
        else {
          piVar4 = __errno_location();
          if ((*piVar4 == 2) || (piVar4 = __errno_location(), *piVar4 == 0x14)) {
            FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),&DAT_0013aa46);
          }
          else {
            piVar4 = __errno_location();
            if (*piVar4 == 0x28) {
              FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),&DAT_0013aa48);
            }
            else {
              FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),&DAT_0013aa4a);
              uVar5 = FUN_0011096a(0,param_3);
              piVar4 = __errno_location();
              error(0,*piVar4,&DAT_0013aa4c,uVar5);
            }
          }
        }
      }
      else {
        uVar5 = FUN_00112955(*(uint *)(param_4 + 3) & 0xf000);
        FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),uVar5);
      }
      break;
    case 'Z':
      iVar2 = (*DAT_001499a0)(DAT_001499d8,DAT_001499d0,&local_390);
      if (iVar2 < 0) {
        FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),&DAT_0013aa45);
        uVar5 = FUN_0011096a(0,param_3);
        uVar6 = gettext("getfilecon failed: %s");
        piVar4 = __errno_location();
        error(0,*piVar4,uVar6,uVar5);
        DAT_001499e4 = 1;
      }
      else {
        FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),local_390);
        freecon(local_390);
      }
      break;
    case 'a':
      auVar7 = FUN_00130a4c(param_4);
      uVar5 = FUN_001123c8(auVar7._0_8_,auVar7._8_8_);
      FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),uVar5);
      break;
    case 'b':
      uVar5 = FUN_00122eb1(param_4[8],local_2b8,0,0x200,0x200);
      FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),uVar5);
      break;
    case 'c':
      auVar7 = FUN_00130a66(param_4);
      uVar5 = FUN_001123c8(auVar7._0_8_,auVar7._8_8_);
      FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),uVar5);
      break;
    case 'd':
      FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),DAT_001499c0);
      break;
    case 'f':
      local_358 = (void *)FUN_0011d85c(param_3);
      FUN_00112839(param_1,*(undefined8 *)(param_2 + 2),local_358);
      free(local_358);
      break;
    case 'g':
      local_360 = getgrgid(*(__gid_t *)(param_4 + 4));
      if (local_360 != (group *)0x0) {
        *(undefined1 *)((long)(int)param_2[4] + *(long *)(param_2 + 2)) = 0x73;
        FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),local_360->gr_name);
        break;
      }
    case 'G':
      uVar5 = FUN_00122eb1(*(undefined4 *)(param_4 + 4),local_2b8,0,1,1);
      FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),uVar5);
      break;
    case 'h':
      local_368 = (char *)FUN_001321c9(param_3);
      sVar3 = strlen(local_368);
      for (local_380 = local_368 + (sVar3 - 1); (local_368 <= local_380 && (*local_380 == '/'));
          local_380 = local_380 + -1) {
      }
      if ((local_368 < local_380) && (local_380[1] == '/')) {
        local_380[1] = '\0';
      }
      local_380 = strrchr(local_368,0x2f);
      if (local_380 == (char *)0x0) {
        FUN_00112839(param_1,*(undefined8 *)(param_2 + 2),&DAT_0013aa43);
      }
      else {
        *local_380 = '\0';
        FUN_00112839(param_1,*(undefined8 *)(param_2 + 2),local_368);
      }
      free(local_368);
      break;
    case 'i':
      uVar5 = FUN_00122eb1(param_4[1],local_2b8,0,1,1);
      FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),uVar5);
      break;
    case 'k':
      uVar5 = FUN_00122eb1(param_4[8],local_2b8,0,0x200,0x400);
      FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),uVar5);
      break;
    case 'l':
      local_378 = (void *)0x0;
      if (((*(uint *)(param_4 + 3) & 0xf000) == 0xa000) &&
         (local_378 = (void *)FUN_0011d06d(DAT_001499d8,DAT_001499d0), local_378 == (void *)0x0)) {
        piVar4 = __errno_location();
        FUN_00110a13(*piVar4,param_3);
        DAT_001499e4 = 1;
      }
      if (local_378 == (void *)0x0) {
        FUN_00112839(param_1,*(undefined8 *)(param_2 + 2),&DAT_0013aa45);
      }
      else {
        FUN_00112839(param_1,*(undefined8 *)(param_2 + 2),local_378);
      }
      free(local_378);
      break;
    case 'm':
      FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),*(uint *)(param_4 + 3) & 0xfff);
      break;
    case 'n':
      uVar5 = FUN_00122eb1(param_4[2],local_2b8,0,1,1);
      FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),uVar5);
      break;
    case 'p':
      FUN_00112839(param_1,*(undefined8 *)(param_2 + 2),param_3);
      break;
    case 's':
      uVar5 = FUN_00122eb1(param_4[6],local_2b8,0,1,1);
      FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),uVar5);
      break;
    case 't':
      auVar7 = FUN_00130a80(param_4);
      uVar5 = FUN_001123c8(auVar7._0_8_,auVar7._8_8_);
      FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),uVar5);
      break;
    case 'u':
      local_370 = getpwuid(*(__uid_t *)((long)param_4 + 0x1c));
      if (local_370 != (passwd *)0x0) {
        *(undefined1 *)((long)(int)param_2[4] + *(long *)(param_2 + 2)) = 0x73;
        FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),local_370->pw_name);
        break;
      }
    case 'U':
      uVar5 = FUN_00122eb1(*(undefined4 *)((long)param_4 + 0x1c),local_2b8,0,1,1);
      FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),uVar5);
      break;
    case 'y':
      uVar5 = FUN_00112955(*(uint *)(param_4 + 3) & 0xf000);
      FUN_0011272c(param_1,*(undefined8 *)(param_2 + 2),uVar5);
    }
  }
  else if (uVar1 < 3) {
    if (uVar1 == 0) {
      FUN_001128a9(*(undefined8 *)(param_2 + 2),1,(long)(int)param_2[4],param_1);
    }
    else if (uVar1 == 1) {
      FUN_001128a9(*(undefined8 *)(param_2 + 2),1,(long)(int)param_2[4],param_1);
      FUN_00112910(param_1);
    }
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: pred_fprintf @ 0x1376d

undefined8 pred_fprintf(undefined8 param_1,undefined8 param_2,long param_3)

{
  char cVar1;
  byte bVar2;
  undefined8 *puVar3;
  undefined8 uVar4;
  undefined1 auVar5 [16];
  int *local_28;
  undefined8 local_18;
  undefined8 local_10;
  
  puVar3 = (undefined8 *)(param_3 + 0x38);
  local_28 = (int *)*puVar3;
  do {
    if (local_28 == (int *)0x0) {
      return 1;
    }
    if ((*local_28 == 2) && (*(char *)((long)local_28 + 5) != '\0')) {
      cVar1 = (char)local_28[1];
      if (cVar1 == 'T') {
        auVar5 = FUN_00130a80(param_2);
        bVar2 = 1;
      }
      else {
        if ('T' < cVar1) {
LAB_00113897:
                    /* WARNING: Subroutine does not return */
          __assert_fail("0","print.c",0x518,"pred_fprintf");
        }
        if (cVar1 == 'C') {
          auVar5 = FUN_00130a66(param_2);
          bVar2 = 1;
        }
        else {
          if ('C' < cVar1) goto LAB_00113897;
          if (cVar1 == 'A') {
            auVar5 = FUN_00130a4c(param_2);
            bVar2 = 1;
          }
          else {
            if (cVar1 != 'B') goto LAB_00113897;
            auVar5 = FUN_00130a9a(param_2);
            if (*(char *)((long)local_28 + 5) == '@') {
              bVar2 = 1;
            }
            else {
              bVar2 = (byte)~auVar5[0xf] >> 7;
            }
          }
        }
      }
      local_10 = auVar5._8_8_;
      local_18 = auVar5._0_8_;
      if (bVar2 == 0) {
        FUN_0011272c(puVar3,*(undefined8 *)(local_28 + 2),&DAT_0013aa45);
      }
      else {
        uVar4 = FUN_0011201f(local_18,local_10,(int)*(char *)((long)local_28 + 5));
        FUN_0011272c(puVar3,*(undefined8 *)(local_28 + 2),uVar4);
      }
    }
    else {
      FUN_001129e7(puVar3,local_28,param_1,param_2);
    }
    local_28 = *(int **)(local_28 + 6);
  } while( true );
}




// Function: free_file_system_list @ 0x1395f

void free_file_system_list(long param_1)

{
  long lVar1;
  undefined8 local_20;
  
  local_20 = param_1;
  while (local_20 != 0) {
    lVar1 = *(long *)(local_20 + 0x30);
    FUN_00126f67(local_20);
    local_20 = lVar1;
  }
  return;
}




// Function: get_file_system_list @ 0x1399c

long get_file_system_list(char param_1)

{
  if (((DAT_0014a1a0 != 0) && (DAT_0014a1a8 != '\x01')) && (param_1 != '\0')) {
    FUN_0011395f(DAT_0014a1a0);
    DAT_0014a1a0 = 0;
  }
  if (DAT_0014a1a0 == 0) {
    DAT_0014a1a0 = FUN_001262d1(param_1);
    DAT_0014a1a8 = param_1;
  }
  return DAT_0014a1a0;
}




// Function: filesystem_type @ 0x13a18

void * filesystem_type(long *param_1,undefined8 param_2)

{
  void *pvVar1;
  
  if (DAT_0014a1b0 != (void *)0x0) {
    if ((DAT_0014a1b8 != '\0') && (*param_1 == DAT_0014a1c0)) {
      return DAT_0014a1b0;
    }
    free(DAT_0014a1b0);
  }
  DAT_0014a1c0 = *param_1;
  pvVar1 = (void *)FUN_00113be0(param_1,param_2,&DAT_0014a1b8);
  DAT_0014a1b0 = pvVar1;
  return pvVar1;
}




// Function: is_used_fs_type @ 0x13aa6

undefined8 is_used_fs_type(char *param_1)

{
  int iVar1;
  undefined8 uVar2;
  long local_18;
  
  iVar1 = strcmp("afs",param_1);
  if (iVar1 == 0) {
    uVar2 = 1;
  }
  else {
    local_18 = FUN_0011399c(0);
    if (local_18 == 0) {
      uVar2 = 1;
    }
    else {
      for (; local_18 != 0; local_18 = *(long *)(local_18 + 0x30)) {
        iVar1 = strcmp(param_1,*(char **)(local_18 + 0x18));
        if (iVar1 == 0) {
          return 1;
        }
      }
      uVar2 = 0;
    }
  }
  return uVar2;
}




// Function: set_fstype_devno @ 0x13b3b

undefined8 set_fstype_devno(long param_1)

{
  int iVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined8 local_a8 [19];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (*(long *)(param_1 + 0x20) == -1) {
    FUN_0010f26e(local_a8);
    iVar1 = (*DAT_00149990)(*(undefined8 *)(param_1 + 8),local_a8);
    if (iVar1 == 0) {
      *(undefined8 *)(param_1 + 0x20) = local_a8[0];
      uVar2 = 0;
    }
    else {
      uVar2 = 0xffffffff;
    }
  }
  else {
    uVar2 = 0;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: file_system_type_uncached @ 0x13be0

long file_system_type_uncached(long *param_1,undefined8 param_2,undefined8 param_3)

{
  int iVar1;
  undefined8 uVar2;
  long local_28;
  long local_20;
  long local_18;
  
  local_20 = 0;
  local_28 = FUN_0011399c(1);
  if (local_28 == 0) {
    uVar2 = gettext("Cannot read mounted file system list");
    error(1,0,uVar2);
  }
  local_18 = 0;
  for (; local_28 != 0; local_28 = *(long *)(local_28 + 0x30)) {
    iVar1 = strcmp(*(char **)(local_28 + 0x18),"ignore");
    if (((iVar1 != 0) && (iVar1 = FUN_00113b3b(local_28), iVar1 == 0)) &&
       (*(long *)(local_28 + 0x20) == *param_1)) {
      local_20 = local_28;
    }
  }
  if (local_20 != 0) {
    local_18 = FUN_001321c9(*(undefined8 *)(local_20 + 0x18));
  }
  *(bool *)param_3 = local_18 != 0;
  if (local_18 == 0) {
    uVar2 = gettext("unknown");
    local_18 = FUN_001321c9(uVar2);
  }
  return local_18;
}




// Function: get_mounted_devices @ 0x13cfc

void * get_mounted_devices(long *param_1)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined8 local_40;
  long local_38;
  long local_30;
  void *local_28;
  long local_20;
  void *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_40 = 0;
  local_38 = 0;
  local_28 = (void *)0x0;
  local_30 = FUN_001262d1(0);
  local_20 = local_30;
  for (; local_30 != 0; local_30 = *(long *)(local_30 + 0x30)) {
    local_18 = (void *)FUN_0011ac29(local_28,(local_38 + 1) * 8,&local_40);
    if (local_18 == (void *)0x0) {
      free(local_28);
      local_28 = (void *)0x0;
    }
    else {
      local_28 = local_18;
      iVar1 = FUN_00113b3b(local_30);
      if (iVar1 == 0) {
        *(undefined8 *)(local_38 * 8 + (long)local_28) = *(undefined8 *)(local_30 + 0x20);
        local_38 = local_38 + 1;
      }
    }
  }
  FUN_0011395f(local_20);
  if (local_28 != (void *)0x0) {
    *param_1 = local_38;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_28;
}




// Function: fallback_getfilecon @ 0x13e1c

ulong fallback_getfilecon(undefined4 param_1,undefined8 param_2,undefined8 param_3,uint param_4)

{
  int *piVar1;
  ulong uVar2;
  
  piVar1 = __errno_location();
  if ((*piVar1 == 2) || (*piVar1 == 0x14)) {
    if ((DAT_00149980 & 2) != 0) {
      fprintf(stderr,"fallback_getfilecon(): getfilecon(%s) failed; falling back on lgetfilecon()\n"
              ,param_2);
    }
    uVar2 = FUN_0013011a(param_1,param_2,param_3);
  }
  else {
    uVar2 = (ulong)param_4;
  }
  return uVar2;
}




// Function: optionh_getfilecon @ 0x13e94

undefined8 optionh_getfilecon(undefined4 param_1,undefined8 param_2,undefined8 param_3)

{
  int iVar1;
  undefined8 uVar2;
  
  if (DAT_001499c0 == 0) {
    iVar1 = FUN_0012fe8c(param_1,param_2,param_3);
    if (iVar1 == 0) {
      uVar2 = 0;
    }
    else {
      uVar2 = FUN_00113e1c(param_1,param_2,param_3,iVar1);
    }
  }
  else {
    uVar2 = FUN_0013011a(param_1,param_2,param_3);
  }
  return uVar2;
}




// Function: optionl_getfilecon @ 0x13f08

undefined8 optionl_getfilecon(undefined4 param_1,undefined8 param_2,undefined8 param_3)

{
  int iVar1;
  undefined8 uVar2;
  
  iVar1 = FUN_0012fe8c(param_1,param_2,param_3);
  if (iVar1 == 0) {
    uVar2 = 0;
  }
  else {
    uVar2 = FUN_00113e1c(param_1,param_2,param_3,iVar1);
  }
  return uVar2;
}




// Function: optionp_getfilecon @ 0x13f5b

void optionp_getfilecon(undefined4 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0013011a(param_1,param_2,param_3);
  return;
}




// Function: check_option_combinations @ 0x13f89

void check_option_combinations(long *param_1)

{
  bool bVar1;
  bool bVar2;
  undefined8 uVar3;
  long *local_20;
  
  bVar2 = false;
  bVar1 = false;
  for (local_20 = param_1; local_20 != (long *)0x0; local_20 = (long *)local_20[0x21]) {
    if ((code *)*local_20 == FUN_001090b1) {
      bVar2 = true;
    }
    else if ((code *)*local_20 == FUN_0010a232) {
      bVar1 = true;
    }
  }
  if (((bVar1) && (bVar2)) && (DAT_00149941 != '\x01')) {
    uVar3 = gettext(
                   "The -delete action automatically turns on -depth, but -prune does nothing when -depth is in effect.  If you want to carry on anyway, just explicitly use the -depth option."
                   );
    error(1,0,uVar3);
  }
  return;
}




// Function: get_noop @ 0x14033

undefined * get_noop(void)

{
  int local_c;
  
  if (DAT_0014a1d0 == (undefined *)0x0) {
    local_c = 0;
    while ((&PTR_DAT_00147de8)[(long)local_c * 4] != (undefined *)0x0) {
      if (*(int *)(&DAT_00147de0 + (long)local_c * 0x20) == 1) {
        DAT_0014a1d0 = &DAT_00147de0 + (long)local_c * 0x20;
        return &DAT_00147de0 + (long)local_c * 0x20;
      }
      local_c = local_c + 1;
    }
  }
  return DAT_0014a1d0;
}




// Function: get_stat_Ytime @ 0x140b3

ulong get_stat_Ytime(undefined8 param_1,char param_2,undefined1 (*param_3) [16])

{
  ulong uVar1;
  undefined1 auVar2 [16];
  
  if (param_2 == 'm') {
    auVar2 = FUN_00130a80(param_1);
    *param_3 = auVar2;
    uVar1 = 1;
  }
  else {
    if ('m' < param_2) {
LAB_00114186:
                    /* WARNING: Subroutine does not return */
      __assert_fail("0","parser.c",0x1f2,"get_stat_Ytime");
    }
    if (param_2 == 'c') {
      auVar2 = FUN_00130a66(param_1);
      *param_3 = auVar2;
      uVar1 = 1;
    }
    else {
      if ('c' < param_2) goto LAB_00114186;
      if (param_2 == 'B') {
        auVar2 = FUN_00130a9a(param_1);
        *param_3 = auVar2;
        uVar1 = ~*(ulong *)(*param_3 + 8) >> 0x3f;
      }
      else {
        if (param_2 != 'a') goto LAB_00114186;
        auVar2 = FUN_00130a4c(param_1);
        *param_3 = auVar2;
        uVar1 = 1;
      }
    }
  }
  return uVar1;
}




// Function: set_follow_state @ 0x141b0

void set_follow_state(uint param_1)

{
  if (param_1 == 2) {
    DAT_00149990 = FUN_0010fb5c;
    DAT_001499a0 = FUN_00113e94;
    DAT_0014994c = 1;
  }
  else if (param_1 < 3) {
    if (param_1 == 0) {
      DAT_00149990 = FUN_0010fcc0;
      DAT_001499a0 = FUN_00113f5b;
    }
    else if (param_1 == 1) {
      DAT_00149990 = FUN_0010fc21;
      DAT_001499a0 = FUN_00113f08;
      DAT_0014994c = 1;
    }
  }
  DAT_00149988 = param_1;
  if ((DAT_00149980 & 2) != 0) {
    DAT_00149990 = FUN_0010fd39;
  }
  return;
}




// Function: parse_begin_user_args @ 0x14262

void parse_begin_user_args(void)

{
  DAT_0014a1c8 = 0;
  return;
}




// Function: parse_end_user_args @ 0x14287

void parse_end_user_args(void)

{
  return;
}




// Function: should_issue_warnings @ 0x142a1

undefined1 should_issue_warnings(void)

{
  undefined1 uVar1;
  
  uVar1 = DAT_00149950;
  if (DAT_00149951 != '\0') {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: found_parser @ 0x142c4

int * found_parser(long param_1,int *param_2)

{
  long lVar1;
  char cVar2;
  undefined8 uVar3;
  
  if (*param_2 != 2) {
    if (*param_2 == 1) {
      param_2 = (int *)0x0;
    }
    else if (*param_2 == 0) {
      if ((DAT_0014a1c8 != 0) && (cVar2 = FUN_001142a1(), lVar1 = DAT_0014a1c8, cVar2 != '\0')) {
        uVar3 = gettext(
                       "warning: you have specified the global option %s after the argument %s, but global options are not positional, i.e., %s affects tests specified before it as well as those specified after it.  Please specify global options before other arguments."
                       );
        error(0,0,uVar3,param_1,lVar1,param_1);
      }
    }
    else if (DAT_0014a1c8 == 0) {
      DAT_0014a1c8 = param_1;
    }
  }
  return param_2;
}




// Function: find_parser @ 0x1437a

undefined8 find_parser(char *param_1)

{
  int iVar1;
  size_t sVar2;
  undefined8 uVar3;
  char *local_20;
  int local_14;
  
  iVar1 = strncmp("-newer",param_1,6);
  if ((iVar1 == 0) && (sVar2 = strlen(param_1), sVar2 == 8)) {
    uVar3 = FUN_001142c4(param_1,&DAT_00147dc0);
    return uVar3;
  }
  local_20 = param_1;
  if (*param_1 == '-') {
    local_20 = param_1 + 1;
  }
  local_14 = 0;
  while( true ) {
    if ((&PTR_DAT_00147de8)[(long)local_14 * 4] == (undefined *)0x0) {
      return 0;
    }
    iVar1 = strcmp((&PTR_DAT_00147de8)[(long)local_14 * 4],local_20);
    if (iVar1 == 0) break;
    local_14 = local_14 + 1;
  }
  uVar3 = FUN_001142c4(param_1,&DAT_00147de0 + (long)local_14 * 0x20);
  return uVar3;
}




// Function: estimate_file_age_success_rate @ 0x1446f

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 estimate_file_age_success_rate(float param_1)

{
  undefined4 uVar1;
  
  uVar1 = DAT_0013c02c;
  if (((DAT_0013c028 <= param_1) && (uVar1 = DAT_0013c034, DAT_0013c030 <= param_1)) &&
     (uVar1 = DAT_0013c03c, _DAT_0013c038 < param_1)) {
    uVar1 = DAT_0013c034;
  }
  return uVar1;
}




// Function: estimate_timestamp_success_rate @ 0x144ce

void estimate_timestamp_success_rate(long param_1)

{
  FUN_0011446f((float)(int)((DAT_00149968 - param_1) / 0x15180));
  return;
}




// Function: collect_arg_nonconst @ 0x14528

undefined8 collect_arg_nonconst(long param_1,int *param_2,undefined8 *param_3)

{
  undefined8 uVar1;
  
  if ((param_1 == 0) || (*(long *)(param_1 + (long)*param_2 * 8) == 0)) {
    *param_3 = 0;
    uVar1 = 0;
  }
  else {
    *param_3 = *(undefined8 *)(param_1 + (long)*param_2 * 8);
    *param_2 = *param_2 + 1;
    uVar1 = 1;
  }
  return uVar1;
}




// Function: collect_arg @ 0x145ab

undefined1 collect_arg(undefined8 param_1,undefined8 param_2,undefined8 *param_3)

{
  undefined1 uVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_00114528(param_1,param_2,&local_18);
  *param_3 = local_18;
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar1;
}




// Function: collect_arg_stat_info @ 0x14611

undefined8
collect_arg_stat_info(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 *param_4)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  undefined8 local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_001145ab(param_1,param_2,&local_28);
  if (cVar1 == '\0') {
    *param_4 = 0;
    uVar4 = 0;
  }
  else {
    *param_4 = local_28;
    iVar2 = (*DAT_00149990)(local_28,param_3);
    if (iVar2 != 0) {
      piVar3 = __errno_location();
      FUN_00110a44(*piVar3,local_28);
    }
    uVar4 = 1;
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar4;
}




// Function: parse_and @ 0x146c4

undefined8 parse_and(undefined8 param_1)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0010e644(param_1);
  *puVar1 = FUN_00108d9c;
  *(undefined4 *)(puVar1 + 2) = 3;
  *(undefined4 *)((long)puVar1 + 0x14) = 3;
  *(undefined1 *)((long)puVar1 + 0x1b) = 0;
  *(undefined1 *)((long)puVar1 + 0x1a) = *(undefined1 *)((long)puVar1 + 0x1b);
  return 1;
}




// Function: parse_anewer @ 0x1472e

bool parse_anewer(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long lVar2;
  long in_FS_OFFSET;
  undefined4 uVar3;
  undefined1 auVar4 [16];
  undefined8 local_c8;
  long local_c0;
  undefined1 local_b8 [88];
  undefined8 local_60;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010f26e(local_b8);
  cVar1 = FUN_00114611(param_2,param_3,local_b8,&local_c8);
  if (cVar1 != '\0') {
    lVar2 = FUN_0010eecc(param_1,local_c8);
    *(undefined4 *)(lVar2 + 0x38) = 0;
    local_c0 = lVar2;
    auVar4 = FUN_00130a80(local_b8);
    *(undefined1 (*) [16])(lVar2 + 0x40) = auVar4;
    *(undefined4 *)(local_c0 + 0x3c) = 0;
    uVar3 = FUN_001144ce(local_60);
    *(undefined4 *)(local_c0 + 0x24) = uVar3;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return cVar1 != '\0';
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_closeparen @ 0x14833

undefined8 parse_closeparen(undefined8 param_1)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0010e644(param_1);
  *puVar1 = FUN_00108ee1;
  *(undefined4 *)(puVar1 + 2) = 5;
  *(undefined4 *)((long)puVar1 + 0x14) = 0;
  *(undefined1 *)((long)puVar1 + 0x1b) = 0;
  *(undefined1 *)((long)puVar1 + 0x1a) = *(undefined1 *)((long)puVar1 + 0x1b);
  return 1;
}




// Function: parse_cnewer @ 0x1489d

bool parse_cnewer(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long lVar2;
  long in_FS_OFFSET;
  undefined4 uVar3;
  undefined1 auVar4 [16];
  undefined8 local_c8;
  long local_c0;
  undefined1 local_b8 [88];
  undefined8 local_60;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010f26e(local_b8);
  cVar1 = FUN_00114611(param_2,param_3,local_b8,&local_c8);
  if (cVar1 != '\0') {
    lVar2 = FUN_0010eecc(param_1,local_c8);
    *(undefined4 *)(lVar2 + 0x38) = 2;
    local_c0 = lVar2;
    auVar4 = FUN_00130a80(local_b8);
    *(undefined1 (*) [16])(lVar2 + 0x40) = auVar4;
    *(undefined4 *)(local_c0 + 0x3c) = 0;
    uVar3 = FUN_001144ce(local_60);
    *(undefined4 *)(local_c0 + 0x24) = uVar3;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return cVar1 != '\0';
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_comma @ 0x149a2

undefined8 parse_comma(undefined8 param_1)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0010e644(param_1);
  *puVar1 = FUN_00108fd0;
  *(undefined4 *)(puVar1 + 2) = 3;
  *(undefined4 *)((long)puVar1 + 0x14) = 1;
  *(undefined1 *)((long)puVar1 + 0x1b) = 0;
  *(undefined1 *)((long)puVar1 + 0x1a) = *(undefined1 *)((long)puVar1 + 0x1b);
  *(undefined4 *)((long)puVar1 + 0x24) = DAT_0013c030;
  return 1;
}




// Function: parse_daystart @ 0x14a1d

undefined8 parse_daystart(void)

{
  tm *ptVar1;
  long lVar2;
  
  if (DAT_00149978 != '\x01') {
    DAT_00149968 = DAT_00149968 + 0x15180;
    DAT_00149970 = 0;
    ptVar1 = localtime(&DAT_00149968);
    if (ptVar1 == (tm *)0x0) {
      lVar2 = DAT_00149968 % 0x15180;
    }
    else {
      lVar2 = (long)(ptVar1->tm_hour * 0xe10 + ptVar1->tm_sec + ptVar1->tm_min * 0x3c);
    }
    DAT_00149968 = DAT_00149968 - lVar2;
    DAT_00149978 = '\x01';
  }
  return 1;
}




// Function: parse_delete @ 0x14afc

undefined8 parse_delete(undefined8 param_1)

{
  long lVar1;
  
  lVar1 = FUN_0010ef32(param_1);
  *(undefined1 *)(lVar1 + 0x19) = 1;
  *(undefined1 *)(lVar1 + 0x18) = *(undefined1 *)(lVar1 + 0x19);
  DAT_00149940 = 0;
  *(undefined1 *)(lVar1 + 0x1b) = 0;
  *(undefined1 *)(lVar1 + 0x1a) = *(undefined1 *)(lVar1 + 0x1b);
  *(undefined4 *)(lVar1 + 0x24) = DAT_0013c030;
  return 1;
}




// Function: parse_depth @ 0x14b71

void parse_depth(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  DAT_00149940 = 0;
  DAT_00149941 = 1;
  FUN_00117bae(param_1,param_2,param_3);
  return;
}




// Function: parse_d @ 0x14bb0

void parse_d(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  undefined8 uVar2;
  
  cVar1 = FUN_001142a1();
  if (cVar1 != '\0') {
    uVar2 = gettext(
                   "warning: the -d option is deprecated; please use -depth instead, because the latter is a POSIX-compliant feature."
                   );
    error(0,0,uVar2);
  }
  FUN_00114b71(param_1,param_2,param_3);
  return;
}




// Function: parse_empty @ 0x14c10

undefined8 parse_empty(undefined8 param_1)

{
  long lVar1;
  
  lVar1 = FUN_0010ef32(param_1);
  *(undefined4 *)(lVar1 + 0x24) = DAT_0013c02c;
  return 1;
}




// Function: parse_exec @ 0x14c50

void parse_exec(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011893a("-exec",param_1,param_2,param_3);
  return;
}




// Function: parse_execdir @ 0x14c88

void parse_execdir(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011893a("-execdir",param_1,param_2,param_3);
  return;
}




// Function: insert_false @ 0x14cc0

undefined8 insert_false(void)

{
  undefined8 uVar1;
  long lVar2;
  
  uVar1 = FUN_0011437a("false");
  lVar2 = FUN_0010ef32(uVar1);
  *(undefined1 *)(lVar2 + 0x1b) = 0;
  *(undefined1 *)(lVar2 + 0x1a) = *(undefined1 *)(lVar2 + 0x1b);
  *(undefined1 *)(lVar2 + 0x19) = 0;
  *(undefined1 *)(lVar2 + 0x18) = *(undefined1 *)(lVar2 + 0x19);
  *(undefined4 *)(lVar2 + 0x24) = 0;
  return 1;
}




// Function: parse_false @ 0x14d31

void parse_false(void)

{
  FUN_00114cc0();
  return;
}




// Function: parse_files0_from @ 0x14d50

bool parse_files0_from(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_001145ab(param_2,param_3,&local_18);
  if (cVar1 != '\0') {
    DAT_001499b0 = local_18;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return cVar1 != '\0';
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: insert_fls @ 0x14dbf

undefined8 insert_fls(undefined8 param_1,long param_2)

{
  long lVar1;
  
  lVar1 = FUN_0010ef32(param_1);
  if (param_2 == 0) {
    FUN_00119a85(lVar1 + 0x38);
  }
  else {
    FUN_00119964(param_2,lVar1 + 0x38);
  }
  *(undefined1 *)(lVar1 + 0x19) = 1;
  *(undefined1 *)(lVar1 + 0x18) = *(undefined1 *)(lVar1 + 0x19);
  *(undefined4 *)(lVar1 + 0x24) = DAT_0013c030;
  return 1;
}




// Function: parse_fls @ 0x14e42

undefined8 parse_fls(undefined8 param_1,undefined8 param_2,int *param_3)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_001145ab(param_2,param_3,&local_18);
  if (cVar1 != '\0') {
    cVar1 = FUN_00114dbf(param_1,local_18);
    if (cVar1 != '\0') {
      uVar2 = 1;
      goto LAB_00114eb6;
    }
    *param_3 = *param_3 + -1;
  }
  uVar2 = 0;
LAB_00114eb6:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: parse_follow @ 0x14ecc

void parse_follow(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_001141b0(1);
  FUN_00117bae(param_1,param_2,param_3);
  return;
}




// Function: parse_fprint @ 0x14f07

bool parse_fprint(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_20;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_001145ab(param_2,param_3,&local_20);
  if (cVar1 != '\0') {
    local_18 = FUN_0010eecc(param_1,local_20);
    FUN_00119964(local_20,local_18 + 0x38);
    *(undefined1 *)(local_18 + 0x19) = 1;
    *(undefined1 *)(local_18 + 0x18) = *(undefined1 *)(local_18 + 0x19);
    *(undefined1 *)(local_18 + 0x1b) = 0;
    *(undefined1 *)(local_18 + 0x1a) = *(undefined1 *)(local_18 + 0x1b);
    *(undefined4 *)(local_18 + 0x24) = DAT_0013c030;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return cVar1 != '\0';
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: insert_fprint @ 0x14fd8

undefined8 insert_fprint(undefined8 param_1,long param_2)

{
  long lVar1;
  
  lVar1 = FUN_0010eecc(param_1,param_2);
  if (param_2 == 0) {
    FUN_00119a85(lVar1 + 0x38);
  }
  else {
    FUN_00119964(param_2,lVar1 + 0x38);
  }
  *(undefined1 *)(lVar1 + 0x19) = 1;
  *(undefined1 *)(lVar1 + 0x18) = *(undefined1 *)(lVar1 + 0x19);
  *(undefined1 *)(lVar1 + 0x1b) = 0;
  *(undefined1 *)(lVar1 + 0x1a) = *(undefined1 *)(lVar1 + 0x1b);
  *(undefined4 *)(lVar1 + 0x24) = DAT_0013c030;
  return 1;
}




// Function: parse_fprint0 @ 0x15079

undefined8 parse_fprint0(undefined8 param_1,undefined8 param_2,int *param_3)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_001145ab(param_2,param_3,&local_18);
  if (cVar1 != '\0') {
    cVar1 = FUN_00114fd8(param_1,local_18);
    if (cVar1 != '\0') {
      uVar2 = 1;
      goto LAB_001150ed;
    }
    *param_3 = *param_3 + -1;
  }
  uVar2 = 0;
LAB_001150ed:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: estimate_fstype_success_rate @ 0x15103

void estimate_fstype_success_rate(char *param_1)

{
  int iVar1;
  char *__s2;
  long in_FS_OFFSET;
  undefined4 uVar2;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = stat("/",&local_a8);
  uVar2 = DAT_0013c030;
  if (iVar1 == 0) {
    __s2 = (char *)FUN_00113a18(&local_a8,&DAT_0013b1c9);
    iVar1 = strcmp(param_1,__s2);
    uVar2 = DAT_0013c034;
    if (iVar1 == 0) {
      uVar2 = DAT_0013c040;
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail(uVar2);
}




// Function: parse_fstype @ 0x151c2

undefined8 parse_fstype(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined4 uVar3;
  undefined8 local_20;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_001145ab(param_2,param_3,&local_20);
  if (cVar1 == '\0') {
    uVar2 = 0;
  }
  else {
    if (1 < DAT_001499a8) {
      cVar1 = FUN_00113aa6(local_20);
      if (cVar1 == '\0') {
        if ((DAT_00149980 & 8) != 0) {
          fprintf(stderr,"-fstype %s can never succeed, substituting -false\n",local_20);
        }
        uVar2 = FUN_00114cc0();
        goto LAB_001152a3;
      }
    }
    local_18 = FUN_0010eecc(param_1,local_20);
    *(undefined8 *)(local_18 + 0x38) = local_20;
    uVar3 = FUN_00115103(local_20);
    *(undefined4 *)(local_18 + 0x24) = uVar3;
    uVar2 = 1;
  }
LAB_001152a3:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: parse_gid @ 0x152b9

undefined8 parse_gid(undefined8 param_1,undefined8 param_2,int *param_3)

{
  long lVar1;
  undefined8 uVar2;
  undefined4 uVar3;
  
  lVar1 = FUN_001197bf(param_2,param_3,param_1);
  if (lVar1 == 0) {
    *param_3 = *param_3 + -1;
    uVar2 = 0;
  }
  else {
    uVar3 = DAT_0013c048;
    if (*(ulong *)(lVar1 + 0x40) < 100) {
      uVar3 = DAT_0013c044;
    }
    *(undefined4 *)(lVar1 + 0x24) = uVar3;
    uVar2 = 1;
  }
  return uVar2;
}




// Function: parse_group @ 0x15339

undefined8 parse_group(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  int iVar2;
  size_t sVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long in_FS_OFFSET;
  undefined4 uVar7;
  __gid_t local_44;
  char *local_38;
  group *local_30;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_001145ab(param_2,param_3,&local_38);
  if (cVar1 == '\0') {
    uVar6 = 0;
    goto LAB_00115537;
  }
  local_30 = getgrnam(local_38);
  endgrent();
  if (local_30 == (group *)0x0) {
    sVar3 = strspn(local_38,"0123456789");
    iVar2 = (int)sVar3;
    if (iVar2 != 0) {
      if (local_38[iVar2] == '\0') {
        local_44 = FUN_0011cd12(local_38,DAT_001499ac);
        goto LAB_001154e1;
      }
      uVar6 = FUN_0012f861(1,DAT_001499ac,local_38 + iVar2);
      uVar4 = FUN_0012f861(0,DAT_001499ac,local_38);
      uVar5 = gettext(
                     "%s is not the name of an existing group and it does not look like a numeric group ID because it has the unexpected suffix %s"
                     );
      error(1,0,uVar5,uVar4,uVar6);
    }
    if (*local_38 != '\0') {
      uVar6 = FUN_0012f861(0,DAT_001499ac,local_38);
      uVar4 = gettext("%s is not the name of an existing group");
      error(1,0,uVar4,uVar6);
    }
    uVar6 = gettext("argument to -group is empty, but should be a group name");
    error(1,0,uVar6);
  }
  else {
    local_44 = local_30->gr_gid;
  }
LAB_001154e1:
  local_28 = FUN_0010eecc(param_1,local_38);
  *(__gid_t *)(local_28 + 0x38) = local_44;
  uVar7 = DAT_0013c048;
  if (*(ulong *)(local_28 + 0x40) < 100) {
    uVar7 = DAT_0013c044;
  }
  *(undefined4 *)(local_28 + 0x24) = uVar7;
  uVar6 = 1;
LAB_00115537:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar6;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_help @ 0x15554

void parse_help(void)

{
                    /* WARNING: Subroutine does not return */
  FUN_0010f071(0);
}




// Function: estimate_pattern_match_rate @ 0x15576

undefined4 estimate_pattern_match_rate(char *param_1,int param_2)

{
  char *pcVar1;
  undefined4 uVar2;
  
  pcVar1 = strpbrk(param_1,"*?[");
  if ((pcVar1 != (char *)0x0) ||
     ((uVar2 = DAT_0013c028, param_2 != 0 &&
      (pcVar1 = strpbrk(param_1,"."), uVar2 = DAT_0013c028, pcVar1 != (char *)0x0)))) {
    uVar2 = DAT_0013c04c;
  }
  return uVar2;
}




// Function: parse_ilname @ 0x155d9

bool parse_ilname(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  float fVar2;
  undefined8 local_20;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_001145ab(param_2,param_3,&local_20);
  if (cVar1 != '\0') {
    local_18 = FUN_0010eecc(param_1,local_20);
    *(undefined8 *)(local_18 + 0x38) = local_20;
    fVar2 = (float)FUN_00115576(local_20,0);
    *(float *)(local_18 + 0x24) = fVar2 * DAT_0013c028;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return cVar1 != '\0';
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: fnmatch_sanitycheck @ 0x15686

undefined1 fnmatch_sanitycheck(void)

{
  int iVar1;
  undefined8 uVar2;
  
  if (DAT_0014a1d8 != '\x01') {
    iVar1 = fnmatch("foo","foo",0);
    if (((iVar1 != 0) || (iVar1 = fnmatch("Foo","foo",0), iVar1 == 0)) ||
       (iVar1 = fnmatch("Foo","foo",0x10), iVar1 != 0)) {
      uVar2 = gettext("sanity check of the fnmatch() library function failed.");
      error(1,0,uVar2);
    }
    DAT_0014a1d8 = 1;
    return 1;
  }
  return 1;
}




// Function: check_name_arg @ 0x1573c

undefined8 check_name_arg(undefined8 param_1,undefined8 param_2,char *param_3)

{
  char cVar1;
  char *pcVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  
  cVar1 = FUN_001142a1();
  if (cVar1 != '\0') {
    pcVar2 = strchr(param_3,0x2f);
    if (pcVar2 != (char *)0x0) {
      uVar3 = FUN_0011096a(2,param_2);
      uVar4 = FUN_0011096a(1,&DAT_0013b1c9);
      uVar5 = FUN_0011096a(0,param_1);
      uVar6 = gettext(
                     "warning: %s matches against basenames only, but the given pattern contains a directory separator (%s), thus the expression will evaluate to false all the time.  Did you mean %s?"
                     );
      error(0,0,uVar6,uVar5,uVar4,uVar3);
    }
  }
  return 1;
}




// Function: parse_iname @ 0x157fa

undefined8 parse_iname(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined4 uVar3;
  undefined8 local_20;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00115686();
  cVar1 = FUN_001145ab(param_2,param_3,&local_20);
  if (cVar1 != '\0') {
    cVar1 = FUN_0011573c("-iname","-iwholename",local_20);
    if (cVar1 != '\0') {
      local_18 = FUN_0010eecc(param_1,local_20);
      *(undefined1 *)(local_18 + 0x1b) = 0;
      *(undefined1 *)(local_18 + 0x1a) = *(undefined1 *)(local_18 + 0x1b);
      *(undefined8 *)(local_18 + 0x38) = local_20;
      uVar3 = FUN_00115576(local_20,0);
      *(undefined4 *)(local_18 + 0x24) = uVar3;
      uVar2 = 1;
      goto LAB_001158cb;
    }
  }
  uVar2 = 0;
LAB_001158cb:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: parse_inum @ 0x158e1

bool parse_inum(undefined8 param_1,undefined8 param_2,int *param_3)

{
  long lVar1;
  
  lVar1 = FUN_001197bf(param_2,param_3,param_1);
  if (lVar1 == 0) {
    *param_3 = *param_3 + -1;
  }
  else {
    *(undefined4 *)(lVar1 + 0x24) = DAT_0013c050;
    *(undefined1 *)(lVar1 + 0x1c) = 1;
    *(undefined1 *)(lVar1 + 0x1a) = 0;
    *(undefined1 *)(lVar1 + 0x1b) = 0;
  }
  return lVar1 != 0;
}




// Function: parse_iregex @ 0x15961

void parse_iregex(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011723e(param_2,param_3,param_1,DAT_0014999c | 0x400000);
  return;
}




// Function: parse_links @ 0x1599c

undefined8 parse_links(undefined8 param_1,undefined8 param_2,int *param_3)

{
  long lVar1;
  undefined8 uVar2;
  
  lVar1 = FUN_001197bf(param_2,param_3,param_1);
  if (lVar1 == 0) {
    *param_3 = *param_3 + -1;
    uVar2 = 0;
  }
  else {
    if (*(long *)(lVar1 + 0x40) == 1) {
      *(undefined4 *)(lVar1 + 0x24) = DAT_0013c044;
    }
    else if (*(long *)(lVar1 + 0x40) == 2) {
      *(undefined4 *)(lVar1 + 0x24) = DAT_0013c02c;
    }
    else {
      *(undefined4 *)(lVar1 + 0x24) = DAT_0013c054;
    }
    uVar2 = 1;
  }
  return uVar2;
}




// Function: parse_lname @ 0x15a46

bool parse_lname(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  float fVar2;
  undefined8 local_20;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00115686();
  cVar1 = FUN_001145ab(param_2,param_3,&local_20);
  if (cVar1 != '\0') {
    local_18 = FUN_0010eecc(param_1,local_20);
    *(undefined8 *)(local_18 + 0x38) = local_20;
    fVar2 = (float)FUN_00115576(local_20,0);
    *(float *)(local_18 + 0x24) = fVar2 * DAT_0013c028;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return cVar1 != '\0';
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_ls @ 0x15af8

void parse_ls(undefined8 param_1)

{
  FUN_00114dbf(param_1,0);
  return;
}




// Function: insert_depthspec @ 0x15b23

undefined8 insert_depthspec(undefined8 param_1,long param_2,int *param_3,int *param_4)

{
  char cVar1;
  int iVar2;
  size_t sVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  char *local_30;
  undefined8 local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_28 = *(undefined8 *)(param_2 + (long)*param_3 * 8 + -8);
  cVar1 = FUN_001145ab(param_2,param_3,&local_30);
  if (cVar1 != '\0') {
    sVar3 = strspn(local_30,"0123456789");
    if ((0 < (int)sVar3) && (local_30[(int)sVar3] == '\0')) {
      iVar2 = FUN_0011cd12(local_30,DAT_001499ac);
      *param_4 = iVar2;
      if (-1 < *param_4) {
        uVar4 = FUN_00117bae(param_1,param_2,param_3);
        goto LAB_00115c49;
      }
    }
    uVar4 = FUN_0012f861(0,DAT_001499ac,local_30);
    uVar5 = gettext("Expected a positive decimal integer argument to %s, but got %s");
    error(1,0,uVar5,local_28,uVar4);
  }
  uVar4 = 0;
LAB_00115c49:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar4;
}




// Function: parse_maxdepth @ 0x15c63

void parse_maxdepth(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_00115b23(param_1,param_2,param_3,&DAT_00149944);
  return;
}




// Function: parse_mindepth @ 0x15c98

void parse_mindepth(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_00115b23(param_1,param_2,param_3,&DAT_00149948);
  return;
}




// Function: do_parse_xmin @ 0x15ccd

undefined8
do_parse_xmin(undefined8 param_1,undefined8 param_2,undefined4 *param_3,undefined4 param_4)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined4 uVar3;
  undefined8 local_48;
  long local_40;
  long local_38;
  undefined8 local_30;
  undefined4 local_28;
  undefined4 uStack_24;
  undefined8 local_20;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  uVar3 = *param_3;
  cVar1 = FUN_001145ab(param_2,param_3,&local_48);
  if (cVar1 != '\0') {
    local_30 = DAT_00149970;
    local_38 = DAT_00149968 + 0x15180;
    local_28 = param_4;
    cVar1 = FUN_0011916a(DAT_0013c058,local_48,&local_28,local_38,DAT_00149970,
                         "arithmetic overflow while converting %s minutes to a number of seconds");
    if (cVar1 != '\0') {
      local_40 = FUN_0010eecc(param_1,local_48);
      *(ulong *)(local_40 + 0x38) = CONCAT44(uStack_24,local_28);
      *(undefined8 *)(local_40 + 0x40) = local_20;
      *(undefined8 *)(local_40 + 0x48) = local_18;
      uVar3 = FUN_001144ce(local_20);
      *(undefined4 *)(local_40 + 0x24) = uVar3;
      uVar2 = 1;
      goto LAB_00115dda;
    }
    *param_3 = uVar3;
  }
  uVar2 = 0;
LAB_00115dda:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: parse_amin @ 0x15df0

void parse_amin(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_00115ccd(param_1,param_2,param_3,0);
  return;
}




// Function: parse_cmin @ 0x15e23

void parse_cmin(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_00115ccd(param_1,param_2,param_3,2);
  return;
}




// Function: parse_mmin @ 0x15e56

void parse_mmin(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_00115ccd(param_1,param_2,param_3,3);
  return;
}




// Function: parse_name @ 0x15e89

undefined8 parse_name(undefined8 param_1,undefined8 param_2,undefined4 *param_3)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined4 uVar3;
  undefined8 local_20;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  uVar3 = *param_3;
  cVar1 = FUN_001145ab(param_2,param_3,&local_20);
  if (cVar1 != '\0') {
    FUN_00115686();
    cVar1 = FUN_0011573c("-name","-wholename",local_20);
    if (cVar1 != '\0') {
      local_18 = FUN_0010eecc(param_1,local_20);
      *(undefined1 *)(local_18 + 0x1b) = 0;
      *(undefined1 *)(local_18 + 0x1a) = *(undefined1 *)(local_18 + 0x1b);
      *(undefined8 *)(local_18 + 0x38) = local_20;
      uVar3 = FUN_00115576(local_20,0);
      *(undefined4 *)(local_18 + 0x24) = uVar3;
      uVar2 = 1;
      goto LAB_00115f6c;
    }
    *param_3 = uVar3;
  }
  uVar2 = 0;
LAB_00115f6c:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: parse_negate @ 0x15f82

undefined8 parse_negate(undefined8 param_1)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0010e7dd(param_1,0);
  *puVar1 = FUN_00109b30;
  *(undefined4 *)(puVar1 + 2) = 2;
  *(undefined4 *)((long)puVar1 + 0x14) = 4;
  *(undefined1 *)((long)puVar1 + 0x1b) = 0;
  *(undefined1 *)((long)puVar1 + 0x1a) = *(undefined1 *)((long)puVar1 + 0x1b);
  return 1;
}




// Function: parse_newer @ 0x15ff1

bool parse_newer(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long lVar2;
  long in_FS_OFFSET;
  undefined4 uVar3;
  undefined1 auVar4 [16];
  undefined8 local_c8;
  long local_c0;
  undefined1 local_b8 [88];
  undefined8 local_60;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010f26e(local_b8);
  cVar1 = FUN_00114611(param_2,param_3,local_b8,&local_c8);
  if (cVar1 != '\0') {
    lVar2 = FUN_0010eecc(param_1,local_c8);
    local_c0 = lVar2;
    auVar4 = FUN_00130a80(local_b8);
    *(undefined1 (*) [16])(lVar2 + 0x40) = auVar4;
    *(undefined4 *)(local_c0 + 0x38) = 3;
    *(undefined4 *)(local_c0 + 0x3c) = 0;
    uVar3 = FUN_001144ce(local_60);
    *(undefined4 *)(local_c0 + 0x24) = uVar3;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return cVar1 != '\0';
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_newerXY @ 0x160f6

undefined8 parse_newerXY(undefined8 param_1,long param_2,int *param_3)

{
  char cVar1;
  char cVar2;
  int iVar3;
  undefined8 uVar4;
  size_t sVar5;
  char *pcVar6;
  undefined8 uVar7;
  long *plVar8;
  int *piVar9;
  long in_FS_OFFSET;
  undefined4 uVar10;
  undefined1 local_b8 [146];
  char local_26 [6];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if ((param_2 == 0) || (*(long *)(param_2 + (long)*param_3 * 8) == 0)) {
    uVar4 = 0;
    goto LAB_00116733;
  }
  sVar5 = strlen(*(char **)(param_2 + (long)*param_3 * 8));
  if (sVar5 != 8) {
    uVar4 = 0;
    goto LAB_00116733;
  }
  builtin_strncpy(local_26,"aBcmt",6);
  iVar3 = strncmp("-newer",*(char **)(param_2 + (long)*param_3 * 8),6);
  if (iVar3 != 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("0 == strncmp (\"-newer\", argv[*arg_ptr], 6)","parser.c",0x613,"parse_newerXY");
  }
  cVar2 = *(char *)(*(long *)(param_2 + (long)*param_3 * 8) + 6);
  cVar1 = *(char *)(*(long *)(param_2 + (long)*param_3 * 8) + 7);
  if ((cVar2 == 'B') || (cVar1 == 'B')) {
    uVar4 = gettext("This system does not provide a way to find the birth time of a file.");
    error(0,0,uVar4);
    uVar4 = 0;
    goto LAB_00116733;
  }
  if (cVar2 != 't') {
    pcVar6 = strchr(local_26,(int)cVar2);
    if (pcVar6 != (char *)0x0) {
      pcVar6 = strchr(local_26,(int)cVar1);
      if (pcVar6 != (char *)0x0) {
        if (*(long *)(param_2 + (long)(*param_3 + 1) * 8) == 0) {
          uVar4 = FUN_0012f861(0,DAT_001499ac,*(undefined8 *)(param_2 + (long)*param_3 * 8));
          uVar7 = gettext("The %s test needs an argument");
          error(1,0,uVar7,uVar4);
        }
        *param_3 = *param_3 + 1;
        plVar8 = (long *)FUN_0010eecc(param_1,*(undefined8 *)(param_2 + (long)*param_3 * 8));
        if (cVar2 == 'm') {
          *(undefined4 *)(plVar8 + 7) = 3;
        }
        else {
          if ('m' < cVar2) {
LAB_00116427:
            pcVar6 = strchr(local_26,(int)cVar2);
            if (pcVar6 == (char *)0x0) {
                    /* WARNING: Subroutine does not return */
              __assert_fail("strchr (validchars, x)","parser.c",0x64b,"parse_newerXY");
            }
                    /* WARNING: Subroutine does not return */
            __assert_fail("0","parser.c",0x64c,"parse_newerXY");
          }
          if (cVar2 == 'c') {
            *(undefined4 *)(plVar8 + 7) = 2;
          }
          else {
            if ('c' < cVar2) goto LAB_00116427;
            if (cVar2 == 'B') {
              *(undefined4 *)(plVar8 + 7) = 1;
            }
            else {
              if (cVar2 != 'a') goto LAB_00116427;
              *(undefined4 *)(plVar8 + 7) = 0;
            }
          }
        }
        if (cVar1 == 't') {
          cVar2 = FUN_0012dcb0(plVar8 + 8,*(undefined8 *)(param_2 + (long)*param_3 * 8),
                               &DAT_00149958);
          if (cVar2 != '\x01') {
            uVar4 = FUN_0012f861(0,DAT_001499ac,*(undefined8 *)(param_2 + (long)*param_3 * 8));
            uVar7 = gettext("I cannot figure out how to interpret %s as a date or time");
            error(1,0,uVar7,uVar4);
            goto LAB_00116544;
          }
        }
        else {
LAB_00116544:
          FUN_0010f26e(local_b8);
          iVar3 = (*DAT_00149990)(*(undefined8 *)(param_2 + (long)*param_3 * 8),local_b8);
          if (iVar3 != 0) {
            uVar4 = *(undefined8 *)(param_2 + (long)*param_3 * 8);
            piVar9 = __errno_location();
            FUN_00110a44(*piVar9,uVar4);
          }
          iVar3 = FUN_001140b3(local_b8,(int)cVar1,plVar8 + 8);
          if (iVar3 == 0) {
            uVar4 = FUN_0011096a(0,*(undefined8 *)(param_2 + (long)*param_3 * 8));
            uVar7 = gettext("Cannot obtain birth time of file %s");
            error(1,0,uVar7,uVar4);
          }
        }
        *(undefined4 *)((long)plVar8 + 0x3c) = 0;
        uVar10 = FUN_001144ce(plVar8[8]);
        *(undefined4 *)((long)plVar8 + 0x24) = uVar10;
        *param_3 = *param_3 + 1;
        if (*plVar8 == 0) {
                    /* WARNING: Subroutine does not return */
          __assert_fail("our_pred->pred_func != NULL","parser.c",0x66f,"parse_newerXY");
        }
        if ((code *)*plVar8 != FUN_00109c02) {
                    /* WARNING: Subroutine does not return */
          __assert_fail("our_pred->pred_func == pred_newerXY","parser.c",0x670,"parse_newerXY");
        }
        if (*(char *)((long)plVar8 + 0x1a) == '\0') {
                    /* WARNING: Subroutine does not return */
          __assert_fail("our_pred->need_stat","parser.c",0x671,"parse_newerXY");
        }
        uVar4 = 1;
        goto LAB_00116733;
      }
    }
  }
  uVar4 = 0;
LAB_00116733:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar4;
}




// Function: parse_noleaf @ 0x1674d

void parse_noleaf(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  DAT_0014994c = 1;
  FUN_00117bae(param_1,param_2,param_3);
  return;
}




// Function: parse_nogroup @ 0x16785

undefined8 parse_nogroup(undefined8 param_1)

{
  long lVar1;
  
  lVar1 = FUN_0010eecc(param_1,0);
  *(undefined4 *)(lVar1 + 0x24) = DAT_0013c060;
  return 1;
}




// Function: parse_nouser @ 0x167ca

undefined8 parse_nouser(undefined8 param_1)

{
  long lVar1;
  
  lVar1 = FUN_0010ef32(param_1);
  *(undefined4 *)(lVar1 + 0x24) = DAT_0013c054;
  return 1;
}




// Function: parse_nowarn @ 0x1680a

void parse_nowarn(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  DAT_00149950 = 0;
  FUN_00117bae(param_1,param_2,param_3);
  return;
}




// Function: parse_ok @ 0x16842

void parse_ok(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011893a(&DAT_0013b600,param_1,param_2,param_3);
  return;
}




// Function: parse_okdir @ 0x1687a

void parse_okdir(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011893a("-okdir",param_1,param_2,param_3);
  return;
}




// Function: parse_openparen @ 0x168b2

undefined8 parse_openparen(undefined8 param_1)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0010e7dd(param_1,0);
  *puVar1 = FUN_00109f83;
  *(undefined4 *)(puVar1 + 2) = 4;
  *(undefined4 *)((long)puVar1 + 0x14) = 0;
  *(undefined1 *)((long)puVar1 + 0x1b) = 0;
  *(undefined1 *)((long)puVar1 + 0x1a) = *(undefined1 *)((long)puVar1 + 0x1b);
  return 1;
}




// Function: parse_or @ 0x16921

undefined8 parse_or(undefined8 param_1)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0010e644(param_1);
  *puVar1 = FUN_00109f9e;
  *(undefined4 *)(puVar1 + 2) = 3;
  *(undefined4 *)((long)puVar1 + 0x14) = 2;
  *(undefined1 *)((long)puVar1 + 0x1b) = 0;
  *(undefined1 *)((long)puVar1 + 0x1a) = *(undefined1 *)((long)puVar1 + 0x1b);
  return 1;
}




// Function: is_feasible_path_argument @ 0x1698b

undefined8 is_feasible_path_argument(char *param_1,undefined1 param_2)

{
  char cVar1;
  char *pcVar2;
  undefined8 uVar3;
  
  pcVar2 = strrchr(param_1,0x2f);
  if ((pcVar2 == (char *)0x0) || (pcVar2[1] != '\0')) {
    uVar3 = 1;
  }
  else {
    cVar1 = FUN_0010bb9d(param_1,param_2);
    if (cVar1 == '\0') {
      uVar3 = 0;
    }
    else {
      uVar3 = 1;
    }
  }
  return uVar3;
}




// Function: insert_path_check @ 0x169f6

undefined8
insert_path_check(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
            code *param_5)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined4 uVar3;
  undefined8 local_30;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00115686();
  cVar1 = FUN_001145ab(param_2,param_3,&local_30);
  if (cVar1 == '\0') {
    uVar2 = 0;
  }
  else {
    local_28 = FUN_0010ee5a(param_1,param_5,local_30);
    *(undefined1 *)(local_28 + 0x1b) = 0;
    *(undefined1 *)(local_28 + 0x1a) = *(undefined1 *)(local_28 + 0x1b);
    *(undefined8 *)(local_28 + 0x38) = local_30;
    uVar3 = FUN_00115576(local_30,0);
    *(undefined4 *)(local_28 + 0x24) = uVar3;
    if ((DAT_00149951 != '\x01') &&
       (cVar1 = FUN_0011698b(local_30,param_5 == FUN_0010986a), cVar1 != '\x01')) {
      uVar2 = gettext("warning: -%s %s will not match anything because it ends with /.");
      error(0,0,uVar2,param_4,local_30);
      *(undefined4 *)(local_28 + 0x24) = DAT_0013c064;
    }
    uVar2 = 1;
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: parse_path @ 0x16b4b

void parse_path(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_001169f6(param_1,param_2,param_3,&DAT_0013ae8c,FUN_0010a012);
  return;
}




// Function: parse_wholename @ 0x16b8a

void parse_wholename(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_001169f6(param_1,param_2,param_3,"wholename",FUN_0010a012);
  return;
}




// Function: parse_ipath @ 0x16bc9

void parse_ipath(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_001169f6(param_1,param_2,param_3,"ipath",FUN_0010986a);
  return;
}




// Function: parse_iwholename @ 0x16c08

void parse_iwholename(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_001169f6(param_1,param_2,param_3,"iwholename",FUN_0010986a);
  return;
}




// Function: parse_perm @ 0x16c47

undefined8 parse_perm(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char *pcVar1;
  char cVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  undefined4 local_4c;
  int local_48;
  undefined4 local_44;
  char *local_40;
  void *local_38;
  long local_30;
  int local_28;
  int local_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  cVar2 = FUN_001145ab(param_2,param_3,&local_40);
  if (cVar2 == '\x01') {
    if (*local_40 == '-') {
      local_48 = 1;
      local_44 = 0;
      local_4c = DAT_0013c048;
    }
    else if (*local_40 == '/') {
      local_48 = 1;
      local_44 = 1;
      local_4c = DAT_0013c034;
    }
    else {
      local_48 = 0;
      local_44 = 2;
      local_4c = DAT_0013c02c;
    }
    local_38 = (void *)FUN_00125a35(local_40 + local_48);
    if ((local_38 == (void *)0x0) ||
       (((*local_40 == '+' && ('/' < local_40[1])) && (local_40[1] < '8')))) {
      uVar3 = FUN_0012f861(0,DAT_001499ac,local_40);
      uVar4 = gettext("invalid mode %s");
      error(1,0,uVar4,uVar3);
    }
    local_28 = FUN_00125f2e(0,0,0,local_38,0);
    local_24 = FUN_00125f2e(0,1,0,local_38,0);
    free(local_38);
    pcVar1 = local_40;
    if (((*local_40 == '/') && (local_28 == 0)) && (local_24 == 0)) {
      uVar3 = gettext(
                     "warning: you have specified a mode pattern %s (which is equivalent to /000). The meaning of -perm /000 has now been changed to be consistent with -perm -000; that is, while it used to match no files, it now matches all files."
                     );
      error(0,0,uVar3,pcVar1);
      local_44 = 0;
      local_4c = DAT_0013c068;
    }
    local_30 = FUN_0010eecc(param_1,local_40);
    *(undefined4 *)(local_30 + 0x24) = local_4c;
    *(undefined4 *)(local_30 + 0x38) = local_44;
    memcpy((void *)(local_30 + 0x3c),&local_28,8);
    uVar3 = 1;
  }
  else {
    uVar3 = 0;
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar3;
}




// Function: parse_print @ 0x16eb3

undefined8 parse_print(undefined8 param_1)

{
  long lVar1;
  
  lVar1 = FUN_0010ef32(param_1);
  *(undefined1 *)(lVar1 + 0x19) = 1;
  *(undefined1 *)(lVar1 + 0x18) = *(undefined1 *)(lVar1 + 0x19);
  *(undefined1 *)(lVar1 + 0x1b) = 0;
  *(undefined1 *)(lVar1 + 0x1a) = *(undefined1 *)(lVar1 + 0x1b);
  FUN_00119a85(lVar1 + 0x38);
  return 1;
}




// Function: parse_print0 @ 0x16f20

void parse_print0(undefined8 param_1)

{
  FUN_00114fd8(param_1,0);
  return;
}




// Function: parse_printf @ 0x16f4b

undefined8 parse_printf(undefined8 param_1,undefined8 param_2,undefined4 *param_3)

{
  undefined4 uVar1;
  char cVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined8 local_40;
  undefined1 local_38 [40];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = *param_3;
  cVar2 = FUN_00114528(param_2,param_3,&local_40);
  if (cVar2 == '\0') {
    uVar3 = 0;
  }
  else {
    FUN_00119a85(local_38);
    cVar2 = FUN_0011159f(local_38,param_1,local_40);
    if (cVar2 == '\0') {
      *param_3 = uVar1;
      uVar3 = 0;
    }
    else {
      uVar3 = 1;
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar3;
}




// Function: parse_fprintf @ 0x16fef

undefined8 parse_fprintf(undefined8 param_1,undefined8 param_2,undefined4 *param_3)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined4 local_4c;
  undefined8 local_48;
  undefined8 local_40;
  undefined1 local_38 [40];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_4c = *param_3;
  cVar1 = FUN_001145ab(param_2,param_3,&local_48);
  if (cVar1 != '\0') {
    cVar1 = FUN_00114528(param_2,param_3,&local_40);
    if (cVar1 != '\0') {
      FUN_00119964(local_48,local_38);
      local_4c = *param_3;
      cVar1 = FUN_0011159f(local_38,param_1,local_40);
      if (cVar1 != '\0') {
        uVar2 = 1;
        goto LAB_001170a1;
      }
    }
  }
  *param_3 = local_4c;
  uVar2 = 0;
LAB_001170a1:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: parse_prune @ 0x170b7

undefined8 parse_prune(undefined8 param_1)

{
  long lVar1;
  
  lVar1 = FUN_0010ef32(param_1);
  if (DAT_00149940 != '\x01') {
    *(undefined1 *)(lVar1 + 0x1b) = 0;
    *(undefined1 *)(lVar1 + 0x1a) = *(undefined1 *)(lVar1 + 0x1b);
  }
  *(undefined1 *)(lVar1 + 0x18) = 1;
  *(undefined1 *)(lVar1 + 0x19) = 0;
  return 1;
}




// Function: parse_quit @ 0x1711b

undefined8 parse_quit(undefined8 param_1)

{
  long lVar1;
  
  lVar1 = FUN_0010ef32(param_1);
  *(undefined1 *)(lVar1 + 0x1b) = 0;
  *(undefined1 *)(lVar1 + 0x1a) = *(undefined1 *)(lVar1 + 0x1b);
  *(undefined1 *)(lVar1 + 0x18) = 1;
  *(undefined1 *)(lVar1 + 0x19) = 0;
  *(undefined4 *)(lVar1 + 0x24) = DAT_0013c030;
  return 1;
}




// Function: parse_regextype @ 0x17182

undefined8 parse_regextype(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_001145ab(param_2,param_3,&local_18);
  if (cVar1 == '\0') {
    uVar2 = 0;
  }
  else {
    DAT_0014999c = FUN_0011ca8f(local_18);
    uVar2 = FUN_00117bae(param_1,param_2,param_3);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: parse_regex @ 0x1720a

void parse_regex(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011723e(param_2,param_3,param_1,DAT_0014999c);
  return;
}




// Function: insert_regex @ 0x1723e

undefined8 insert_regex(undefined8 param_1,undefined8 param_2,undefined8 param_3,int param_4)

{
  char *pcVar1;
  char cVar2;
  uchar *puVar3;
  size_t __length;
  undefined8 uVar4;
  long in_FS_OFFSET;
  undefined4 uVar5;
  char *local_40;
  long local_38;
  re_pattern_buffer *local_30;
  char *local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  cVar2 = FUN_001145ab(param_1,param_2,&local_40);
  if (cVar2 == '\0') {
    uVar4 = 0;
  }
  else {
    local_38 = FUN_0010ee5a(param_3,FUN_0010a2da,local_40);
    *(undefined1 *)(local_38 + 0x1b) = 0;
    *(undefined1 *)(local_38 + 0x1a) = *(undefined1 *)(local_38 + 0x1b);
    local_30 = (re_pattern_buffer *)FUN_00131bc2(0x40);
    *(re_pattern_buffer **)(local_38 + 0x38) = local_30;
    local_30->allocated = 100;
    puVar3 = (uchar *)FUN_00131bc2(local_30->allocated);
    local_30->buffer = puVar3;
    local_30->fastmap = (char *)0x0;
    re_set_syntax((long)param_4);
    local_30->syntax = (long)param_4;
    local_30->translate = (uchar *)0x0;
    __length = strlen(local_40);
    local_28 = re_compile_pattern(local_40,__length,local_30);
    pcVar1 = local_40;
    if (local_28 != (char *)0x0) {
      uVar4 = gettext("failed to compile regular expression \'%s\': %s");
      error(1,0,uVar4,pcVar1,local_28);
    }
    uVar5 = FUN_00115576(local_40,1);
    *(undefined4 *)(local_38 + 0x24) = uVar5;
    uVar4 = 1;
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar4;
}




// Function: parse_size @ 0x173d3

undefined8 parse_size(undefined8 param_1,long param_2,int *param_3)

{
  char cVar1;
  undefined8 uVar2;
  size_t sVar3;
  long in_FS_OFFSET;
  undefined4 uVar4;
  char local_4d;
  int local_4c;
  int local_48;
  int local_44;
  long local_40;
  char *local_38;
  long local_30;
  char local_22 [2];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_48 = 0x200;
  if ((param_2 == 0) || (*(long *)(param_2 + (long)*param_3 * 8) == 0)) {
    uVar2 = 0;
    goto LAB_00117764;
  }
  local_38 = *(char **)(param_2 + (long)*param_3 * 8);
  sVar3 = strlen(local_38);
  local_44 = (int)sVar3;
  if (local_44 == 0) {
    uVar2 = gettext("invalid null argument to -size");
    error(1,0,uVar2);
  }
  local_4d = local_38[(long)local_44 + -1];
  if (local_4d == 'w') {
    local_48 = 2;
    local_38[(long)local_44 + -1] = '\0';
  }
  else if (local_4d < 'x') {
    if (local_4d == 'k') {
      local_48 = 0x400;
      local_38[(long)local_44 + -1] = '\0';
    }
    else {
      if ('k' < local_4d) goto LAB_001175d2;
      if (local_4d == 'c') {
        local_48 = 1;
        local_38[(long)local_44 + -1] = '\0';
      }
      else {
        if ('c' < local_4d) goto LAB_001175d2;
        if (local_4d == 'b') {
          local_48 = 0x200;
          local_38[(long)local_44 + -1] = '\0';
        }
        else {
          if ('b' < local_4d) goto LAB_001175d2;
          if (local_4d == 'M') {
            local_48 = 0x100000;
            local_38[(long)local_44 + -1] = '\0';
          }
          else {
            if ('M' < local_4d) goto LAB_001175d2;
            if (local_4d < ':') {
              if (local_4d < '0') goto LAB_001175d2;
              local_4d = '\0';
            }
            else {
              if (local_4d != 'G') goto LAB_001175d2;
              local_48 = 0x40000000;
              local_38[(long)local_44 + -1] = '\0';
            }
          }
        }
      }
    }
  }
  else {
LAB_001175d2:
    cVar1 = *(char *)((long)local_44 + -1 + *(long *)(param_2 + (long)*param_3 * 8));
    uVar2 = gettext("invalid -size type `%c\'");
    error(1,0,uVar2,(int)cVar1);
  }
  cVar1 = FUN_00119722(local_38,&local_40,&local_4c);
  if (cVar1 != '\x01') {
    local_22[0] = local_4d;
    local_22[1] = 0;
    uVar2 = gettext("Invalid argument `%s%s\' to -size");
    error(1,0,uVar2,local_38,local_22);
  }
  local_30 = FUN_0010eecc(param_1,local_38);
  *(int *)(local_30 + 0x38) = local_4c;
  *(int *)(local_30 + 0x3c) = local_48;
  *(long *)(local_30 + 0x40) = local_40;
  *(undefined1 *)(local_30 + 0x1a) = 1;
  *(undefined1 *)(local_30 + 0x1b) = 0;
  if (local_4c == 0) {
    uVar4 = DAT_0013c06c;
    if (0x5000 < (ulong)(local_40 * local_48)) {
      uVar4 = DAT_0013c028;
    }
    *(undefined4 *)(local_30 + 0x24) = uVar4;
  }
  else if (local_4c == 1) {
    uVar4 = DAT_0013c028;
    if (0x5000 < (ulong)(local_40 * local_48)) {
      uVar4 = DAT_0013c06c;
    }
    *(undefined4 *)(local_30 + 0x24) = uVar4;
  }
  else {
    *(undefined4 *)(local_30 + 0x24) = DAT_0013c02c;
  }
  *param_3 = *param_3 + 1;
  uVar2 = 1;
LAB_00117764:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_samefile @ 0x1777e

undefined8 parse_samefile(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  int *piVar4;
  long in_FS_OFFSET;
  int local_160;
  undefined4 local_15c;
  undefined8 local_158;
  long local_150;
  __dev_t local_148;
  __ino_t local_140;
  __nlink_t local_138;
  uint local_130;
  __uid_t _Stack_12c;
  __gid_t local_128;
  int iStack_124;
  __dev_t local_120;
  __off_t local_118;
  __blksize_t local_110;
  __blkcnt_t local_108;
  __time_t local_100;
  long local_f8;
  __time_t local_f0;
  long local_e8;
  __time_t local_e0;
  long local_d8;
  long local_d0;
  long local_c8;
  long local_c0;
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010f26e(&local_148);
  cVar1 = FUN_00114611(param_2,param_3,&local_148,&local_158);
  if (cVar1 == '\x01') {
    FUN_0010f26e(&local_b8);
    local_15c = 0;
    if (DAT_00149988 == 0) {
      if (DAT_00149998 == '\0') {
        if ((local_130 & 0xf000) == 0xa000) {
          local_160 = -2;
        }
        else {
          local_160 = -1;
        }
      }
      else {
        local_15c = 0x20000;
        local_160 = -1;
      }
    }
    else {
      local_160 = -1;
    }
    if (local_160 == -3) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("fd != -3","parser.c",0x8bf,"parse_samefile");
    }
    if ((local_160 == -1) && (local_160 = FUN_0011b433(local_158,local_15c), -1 < local_160)) {
      iVar2 = fstat(local_160,&local_b8);
      uVar3 = local_158;
      if (iVar2 != 0) {
        piVar4 = __errno_location();
        FUN_00110a44(*piVar4,uVar3);
      }
      iVar2 = (*DAT_00149990)(local_158,&local_148);
      uVar3 = local_158;
      if (iVar2 != 0) {
        piVar4 = __errno_location();
        FUN_00110a44(*piVar4,uVar3);
      }
      if ((DAT_00149988 == 0) && (DAT_00149998 != '\x01')) {
        if ((local_130 & 0xf000) == 0xa000) {
          close(local_160);
          local_160 = -1;
        }
        else if ((local_148 != local_b8.st_dev) || (local_140 != local_b8.st_ino)) {
          close(local_160);
          local_160 = -1;
        }
      }
      else {
        local_148 = local_b8.st_dev;
        local_140 = local_b8.st_ino;
        local_138 = local_b8.st_nlink;
        local_130 = local_b8.st_mode;
        _Stack_12c = local_b8.st_uid;
        local_128 = local_b8.st_gid;
        iStack_124 = local_b8.__pad0;
        local_120 = local_b8.st_rdev;
        local_118 = local_b8.st_size;
        local_110 = local_b8.st_blksize;
        local_108 = local_b8.st_blocks;
        local_100 = local_b8.st_atim.tv_sec;
        local_f8 = local_b8.st_atim.tv_nsec;
        local_f0 = local_b8.st_mtim.tv_sec;
        local_e8 = local_b8.st_mtim.tv_nsec;
        local_e0 = local_b8.st_ctim.tv_sec;
        local_d8 = local_b8.st_ctim.tv_nsec;
        local_d0 = local_b8.__unused[0];
        local_c8 = local_b8.__unused[1];
        local_c0 = local_b8.__unused[2];
      }
    }
    local_150 = FUN_0010eecc(param_1,local_158);
    *(__ino_t *)(local_150 + 0x38) = local_140;
    *(__dev_t *)(local_150 + 0x40) = local_148;
    *(int *)(local_150 + 0x48) = local_160;
    *(undefined1 *)(local_150 + 0x1b) = 0;
    *(undefined1 *)(local_150 + 0x1a) = 1;
    *(undefined4 *)(local_150 + 0x24) = DAT_0013c02c;
    uVar3 = 1;
  }
  else {
    uVar3 = 0;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_true @ 0x17b57

undefined8 parse_true(undefined8 param_1)

{
  long lVar1;
  
  lVar1 = FUN_0010ef32(param_1);
  *(undefined1 *)(lVar1 + 0x1b) = 0;
  *(undefined1 *)(lVar1 + 0x1a) = *(undefined1 *)(lVar1 + 0x1b);
  *(undefined4 *)(lVar1 + 0x24) = DAT_0013c030;
  return 1;
}




// Function: parse_noop @ 0x17bae

void parse_noop(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 uVar1;
  
  uVar1 = FUN_00114033();
  FUN_00117b57(uVar1,param_2,param_3);
  return;
}




// Function: parse_accesscheck @ 0x17be3

undefined8 parse_accesscheck(undefined8 param_1)

{
  long *plVar1;
  
  plVar1 = (long *)FUN_0010ef32(param_1);
  *(undefined1 *)((long)plVar1 + 0x1b) = 0;
  *(undefined1 *)((long)plVar1 + 0x1a) = *(undefined1 *)((long)plVar1 + 0x1b);
  *(undefined1 *)((long)plVar1 + 0x19) = 0;
  *(undefined1 *)(plVar1 + 3) = *(undefined1 *)((long)plVar1 + 0x19);
  if ((code *)*plVar1 == FUN_0010a0fa) {
    *(undefined4 *)((long)plVar1 + 0x24) = DAT_0013c048;
  }
  else {
    *(undefined4 *)((long)plVar1 + 0x24) = DAT_0013c06c;
  }
  return 1;
}




// Function: parse_type @ 0x17c77

void parse_type(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_001183bf(param_2,param_3,param_1,FUN_0010a4b2);
  return;
}




// Function: parse_uid @ 0x17cac

undefined8 parse_uid(undefined8 param_1,undefined8 param_2,int *param_3)

{
  long lVar1;
  undefined8 uVar2;
  undefined4 uVar3;
  
  lVar1 = FUN_001197bf(param_2,param_3,param_1);
  if (lVar1 == 0) {
    *param_3 = *param_3 + -1;
    uVar2 = 0;
  }
  else {
    uVar3 = DAT_0013c048;
    if (*(ulong *)(lVar1 + 0x40) < 100) {
      uVar3 = DAT_0013c044;
    }
    *(undefined4 *)(lVar1 + 0x24) = uVar3;
    uVar2 = 1;
  }
  return uVar2;
}




// Function: parse_used @ 0x17d2c

undefined8 parse_used(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined4 uVar3;
  undefined8 local_60;
  char *local_58;
  long local_50;
  undefined8 local_48;
  undefined8 local_40;
  undefined8 local_38;
  long local_30;
  undefined8 local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_58 = "arithmetic overflow while converting %s days to a number of seconds";
  cVar1 = FUN_001145ab(param_2,param_3,&local_60);
  if (cVar1 != '\0') {
    local_48 = 0;
    local_40 = 0;
    cVar1 = FUN_0011916a(DAT_0013c070,local_60,&local_38,0,0,local_58);
    if (cVar1 != '\0') {
      local_50 = FUN_0010eecc(param_1,local_60);
      *(undefined8 *)(local_50 + 0x38) = local_38;
      *(long *)(local_50 + 0x40) = local_30;
      *(undefined8 *)(local_50 + 0x48) = local_28;
      uVar3 = FUN_0011446f((float)(local_30 / 0x15180));
      *(undefined4 *)(local_50 + 0x24) = uVar3;
      uVar2 = 1;
      goto LAB_00117e73;
    }
    uVar2 = gettext("Invalid argument %s to -used");
    error(1,0,uVar2,local_60);
  }
  uVar2 = 0;
LAB_00117e73:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: parse_user @ 0x17e8d

undefined8 parse_user(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined4 uVar4;
  __uid_t local_44;
  char *local_40;
  passwd *local_38;
  size_t local_30;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_001145ab(param_2,param_3,&local_40);
  if (cVar1 == '\0') {
    uVar3 = 0;
  }
  else {
    local_38 = getpwnam(local_40);
    endpwent();
    if (local_38 == (passwd *)0x0) {
      local_30 = strspn(local_40,"0123456789");
      if ((local_30 == 0) || (local_40[local_30] != '\0')) {
        if (*local_40 != '\0') {
          uVar3 = FUN_0012f861(0,DAT_001499ac,local_40);
          uVar2 = gettext("%s is not the name of a known user");
          error(1,0,uVar2,uVar3);
        }
        uVar3 = gettext("The argument to -user should not be empty");
        error(1,0,uVar3);
      }
      else {
        local_44 = FUN_0011cd12(local_40,DAT_001499ac);
      }
    }
    else {
      local_44 = local_38->pw_uid;
    }
    local_28 = FUN_0010eecc(param_1,local_40);
    *(__uid_t *)(local_28 + 0x38) = local_44;
    uVar4 = DAT_0013c048;
    if (*(uint *)(local_28 + 0x38) < 100) {
      uVar4 = DAT_0013c044;
    }
    *(undefined4 *)(local_28 + 0x24) = uVar4;
    uVar3 = 1;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_version @ 0x1802c

/* WARNING: Removing unreachable block (ram,0x0011819b) */

void parse_version(void)

{
  char cVar1;
  int iVar2;
  char *pcVar3;
  long in_FS_OFFSET;
  uint local_14;
  undefined8 local_10;
  
  local_10 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  FUN_0011b6b6(&DAT_0013b89a);
  pcVar3 = (char *)gettext("Features enabled: ");
  printf(pcVar3);
  printf("D_TYPE ");
  if (DAT_00149998 == '\0') {
    pcVar3 = "disabled";
  }
  else {
    pcVar3 = "enabled";
  }
  printf("O_NOFOLLOW(%s) ",pcVar3);
  printf("LEAF_OPTIMISATION ");
  iVar2 = is_selinux_enabled();
  if (0 < iVar2) {
    printf("SELINUX ");
  }
  local_14 = 0;
  cVar1 = FUN_00108b12(&local_14);
  if (cVar1 != '\0') {
    printf("FTS(");
    if ((local_14 & 0x200) != 0) {
      printf("FTS_CWDFD");
    }
    printf(") ");
  }
  printf("CBO(level=%d) ",(ulong)DAT_001499a8);
  printf("\n");
                    /* WARNING: Subroutine does not return */
  exit(0);
}




// Function: parse_context @ 0x181cd

undefined8 parse_context(undefined8 param_1,long param_2,int *param_3)

{
  int iVar1;
  undefined8 uVar2;
  long lVar3;
  
  if ((param_2 == 0) || (*(long *)(param_2 + (long)*param_3 * 8) == 0)) {
    uVar2 = 0;
  }
  else {
    iVar1 = is_selinux_enabled();
    if (iVar1 < 1) {
      uVar2 = gettext("invalid predicate -context: SELinux is not enabled.");
      error(1,0,uVar2);
    }
    lVar3 = FUN_0010eecc(param_1,0);
    *(undefined4 *)(lVar3 + 0x24) = DAT_0013c02c;
    *(undefined1 *)(lVar3 + 0x1a) = 0;
    *(undefined8 *)(lVar3 + 0x38) = *(undefined8 *)(param_2 + (long)*param_3 * 8);
    *param_3 = *param_3 + 1;
    uVar2 = 1;
  }
  return uVar2;
}




// Function: parse_xdev @ 0x182aa

void parse_xdev(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  DAT_0014994d = 1;
  FUN_00117bae(param_1,param_2,param_3);
  return;
}




// Function: parse_ignore_race @ 0x182e2

void parse_ignore_race(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  DAT_0014994e = 1;
  FUN_00117bae(param_1,param_2,param_3);
  return;
}




// Function: parse_noignore_race @ 0x1831a

void parse_noignore_race(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  DAT_0014994e = 0;
  FUN_00117bae(param_1,param_2,param_3);
  return;
}




// Function: parse_warn @ 0x18352

void parse_warn(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  DAT_00149950 = 1;
  FUN_00117bae(param_1,param_2,param_3);
  return;
}




// Function: parse_xtype @ 0x1838a

void parse_xtype(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_001183bf(param_2,param_3,param_1,FUN_0010a78e);
  return;
}




// Function: insert_type @ 0x183bf

undefined8 insert_type(undefined8 param_1,undefined8 param_2,undefined8 param_3,code *param_4)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  uint local_40;
  float local_3c;
  char *local_38;
  char *local_30;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_4 == FUN_0010a78e) {
    local_30 = "-xtype";
  }
  else {
    local_30 = "-type";
  }
  cVar1 = FUN_001145ab(param_1,param_2,&local_38);
  if (cVar1 == '\x01') {
    if (*local_38 == '\0') {
      uVar2 = gettext("Arguments to %s should contain at least one letter");
      error(1,0,uVar2,local_30);
    }
    local_28 = FUN_0010ee5a(param_3,param_4,local_38);
    *(undefined4 *)(local_28 + 0x24) = 0;
    if (param_4 == FUN_0010a78e) {
      *(undefined1 *)(local_28 + 0x1a) = 1;
      *(undefined1 *)(local_28 + 0x1b) = 0;
    }
    else {
      *(undefined1 *)(local_28 + 0x1a) = 0;
      *(undefined1 *)(local_28 + 0x1b) = 1;
    }
    while (*local_38 != '\0') {
      local_3c = DAT_0013c02c;
      switch(*local_38) {
      case 'D':
        cVar1 = *local_38;
        uVar2 = gettext(
                       "%s %c is not supported because Solaris doors are not supported on the platform find was compiled on."
                       );
        error(1,0,uVar2,local_30,(int)cVar1);
      default:
        local_40 = 0;
        cVar1 = *local_38;
        uVar2 = gettext("Unknown argument to %s: %c");
        error(1,0,uVar2,local_30,(int)cVar1);
        break;
      case 'b':
        local_40 = 0;
        local_3c = DAT_0013c078;
        break;
      case 'c':
        local_40 = 1;
        local_3c = DAT_0013c07c;
        break;
      case 'd':
        local_40 = 2;
        local_3c = DAT_0013c080;
        break;
      case 'f':
        local_40 = 3;
        local_3c = DAT_0013c084;
        break;
      case 'l':
        local_40 = 4;
        local_3c = DAT_0013c088;
        break;
      case 'p':
        local_40 = 5;
        local_3c = DAT_0013c08c;
        break;
      case 's':
        local_40 = 6;
        local_3c = DAT_0013c090;
      }
      if (*(char *)(local_28 + 0x38 + (ulong)local_40) != '\0') {
        cVar1 = *local_38;
        uVar2 = gettext("Duplicate file type \'%c\' in the argument list to %s.");
        error(1,0,uVar2,(int)cVar1,local_30);
      }
      *(float *)(local_28 + 0x24) = *(float *)(local_28 + 0x24) + local_3c;
      *(undefined1 *)(local_28 + 0x38 + (ulong)local_40) = 1;
      local_38 = local_38 + 1;
      if (*local_38 != '\0') {
        if (*local_38 != ',') {
          uVar2 = gettext("Must separate multiple arguments to %s using: \',\'");
          error(1,0,uVar2,local_30);
        }
        local_38 = local_38 + 1;
        if (*local_38 == '\0') {
          uVar2 = gettext(
                         "Last file type in list argument to %s is missing, i.e., list is ending on: \',\'"
                         );
          error(1,0,uVar2,local_30);
        }
      }
    }
    uVar2 = 1;
  }
  else {
    uVar2 = 0;
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: stream_is_tty @ 0x1876f

undefined8 stream_is_tty(FILE *param_1)

{
  int iVar1;
  undefined8 uVar2;
  undefined4 extraout_var;
  
  iVar1 = fileno(param_1);
  if (iVar1 == -1) {
    uVar2 = 0;
  }
  else {
    iVar1 = isatty(iVar1);
    uVar2 = CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),iVar1 != 0);
  }
  return uVar2;
}




// Function: check_path_safety @ 0x187ac

void check_path_safety(undefined8 param_1)

{
  char cVar1;
  undefined8 uVar2;
  char *pcVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  long local_48;
  size_t local_40;
  char *local_38;
  undefined *local_30;
  char *local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_38 = getenv("PATH");
  local_30 = &DAT_0013bba5;
  if (local_38 != (char *)0x0) {
    FUN_0011cf56(local_38,&DAT_0013bba5,1,&local_48,&local_40);
    do {
      if ((local_40 == 0) || ((local_40 == 1 && (local_38[local_48] == '.')))) {
        uVar2 = gettext(
                       "The current directory is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove the current directory from your $PATH (that is, remove \".\", doubled colons, or leading or trailing colons)"
                       );
        error(1,0,uVar2,param_1);
      }
      if (local_38[local_48] != '/') {
        local_28 = strndup(local_38 + local_48,local_40);
        pcVar3 = local_28;
        if (local_28 == (char *)0x0) {
          pcVar3 = local_38 + local_48;
        }
        uVar2 = FUN_0011096a(0,pcVar3);
        uVar4 = gettext(
                       "The relative path %s is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove that entry from $PATH"
                       );
        error(1,0,uVar4,uVar2,param_1);
      }
      cVar1 = FUN_0011cf56(local_38,local_30,0,&local_48,&local_40);
    } while (cVar1 != '\0');
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: insert_exec_ok @ 0x1893a

undefined8 insert_exec_ok(undefined8 param_1,long param_2,long param_3,int *param_4)

{
  int iVar1;
  code *pcVar2;
  bool bVar3;
  bool bVar4;
  uint uVar5;
  int iVar6;
  undefined8 uVar7;
  void *__ptr;
  long lVar8;
  size_t sVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  long in_FS_OFFSET;
  int local_90;
  int local_8c;
  int local_84;
  char *local_78;
  undefined *local_70;
  char local_48 [24];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  pcVar2 = *(code **)(param_2 + 0x18);
  if ((param_3 == 0) || (*(long *)(param_3 + (long)*param_4 * 8) == 0)) {
    uVar7 = 0;
  }
  else {
    __ptr = (void *)FUN_0010ee5a(param_2,pcVar2,"(some -exec* arguments)");
    *(undefined1 *)((long)__ptr + 0x19) = 1;
    *(undefined1 *)((long)__ptr + 0x18) = *(undefined1 *)((long)__ptr + 0x19);
    *(undefined1 *)((long)__ptr + 0x1a) = 0;
    *(undefined1 *)((long)__ptr + 0x1b) = *(undefined1 *)((long)__ptr + 0x1a);
    *(undefined8 *)((long)__ptr + 0xf8) = 0;
    if ((pcVar2 == FUN_00109f27) || (pcVar2 == FUN_00109ece)) {
      bVar3 = false;
      DAT_001499b8 = 1;
      *(undefined1 *)((long)__ptr + 0xf4) = 1;
    }
    else {
      bVar3 = true;
      *(undefined1 *)((long)__ptr + 0xf4) = 0;
    }
    if ((pcVar2 == FUN_00109497) || (pcVar2 == FUN_00109f27)) {
      *(undefined8 *)((long)__ptr + 0xf8) = 0;
      DAT_0014994e = 0;
      FUN_001187ac(param_1);
    }
    else {
      if (DAT_00149a00 == 0) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("NULL != initial_wd","parser.c",0xb79,"insert_exec_ok");
      }
      *(long *)((long)__ptr + 0xf8) = DAT_00149a00;
    }
    *(undefined1 *)((long)__ptr + 0x38) = 0;
    iVar1 = *param_4;
    bVar4 = false;
    local_84 = 0;
    local_78 = (char *)0x0;
    local_90 = iVar1;
    while( true ) {
      if ((*(long *)(param_3 + (long)local_90 * 8) == 0) ||
         ((**(char **)(param_3 + (long)local_90 * 8) == ';' &&
          (*(char *)(*(long *)(param_3 + (long)local_90 * 8) + 1) == '\0')))) goto LAB_00118cc6;
      if ((bVar3) &&
         (((**(char **)(param_3 + (long)local_90 * 8) == '+' &&
           (*(char *)(*(long *)(param_3 + (long)local_90 * 8) + 1) == '\0')) && (bVar4)))) break;
      bVar4 = false;
      lVar8 = FUN_00124cc3(*(undefined8 *)(param_3 + (long)local_90 * 8),&DAT_0013bd78);
      if (lVar8 != 0) {
        bVar4 = true;
        local_78 = *(char **)(param_3 + (long)local_90 * 8);
        local_84 = local_84 + 1;
        if ((local_90 == 0) && ((pcVar2 == FUN_00109497 || (pcVar2 == FUN_00109f27)))) {
          uVar7 = gettext(
                         "You may not use {} within the utility name for -execdir and -okdir, because this is a potential security problem."
                         );
          error(1,0,uVar7);
        }
      }
      local_90 = local_90 + 1;
    }
    *(undefined1 *)((long)__ptr + 0x38) = 1;
LAB_00118cc6:
    if ((local_90 == iVar1) || (*(long *)(param_3 + (long)local_90 * 8) == 0)) {
      *param_4 = local_90;
      free(__ptr);
      uVar7 = 0;
    }
    else {
      if (*(char *)((long)__ptr + 0x38) != '\0') {
        if (pcVar2 == FUN_00109497) {
          local_70 = &DAT_0013bdf2;
        }
        else {
          local_70 = &DAT_0013bdf6;
        }
        if (1 < local_84) {
          uVar7 = gettext("Only one instance of {} is supported with -exec%s ... +");
          error(1,0,uVar7,local_70);
        }
        sVar9 = strlen(local_78);
        if (sVar9 != 2) {
          uVar5 = snprintf(local_48,0x13,"-exec%s ... {} +",local_70);
          if (0x13 < uVar5) {
                    /* WARNING: Subroutine does not return */
            __assert_fail("needed <= MsgBufSize","parser.c",0xbc6,"insert_exec_ok");
          }
          uVar7 = FUN_0012f861(2,DAT_001499ac,local_78);
          uVar10 = FUN_0012f861(1,DAT_001499ac,&DAT_0013bd78);
          uVar11 = FUN_0012f861(0,DAT_001499ac,local_48);
          uVar12 = gettext("In %s the %s must appear by itself, but you specified %s");
          error(1,0,uVar12,uVar11,uVar10,uVar7);
        }
      }
      iVar6 = FUN_0011a58c((long)__ptr + 0x40,0x800);
      if ((iVar6 != 0) && (iVar6 - 1U < 2)) {
        uVar7 = gettext("The environment is too large for exec().");
        error(1,0,uVar7);
      }
      FUN_0011a6e3((long)__ptr + 0x40);
      *(code **)((long)__ptr + 0x80) = FUN_0010b7ae;
      if (*(char *)((long)__ptr + 0x38) == '\0') {
        *(int *)((long)__ptr + 0xf0) = local_90 - iVar1;
        *(undefined **)((long)__ptr + 0x70) = &DAT_0013bd78;
        sVar9 = strlen(*(char **)((long)__ptr + 0x70));
        *(size_t *)((long)__ptr + 0x68) = sVar9;
        *(undefined8 *)((long)__ptr + 0x88) = 0;
        *(undefined8 *)((long)__ptr + 0x90) = 0;
        uVar7 = FUN_00131bc2((long)*(int *)((long)__ptr + 0xf0) << 3);
        *(undefined8 *)((long)__ptr + 0xe8) = uVar7;
        FUN_0011a742((long)__ptr + 0x40,(long)__ptr + 0x98,(long)__ptr + 0x38);
        for (local_8c = 0; local_8c < *(int *)((long)__ptr + 0xf0); local_8c = local_8c + 1) {
          *(undefined8 *)(*(long *)((long)__ptr + 0xe8) + (long)local_8c * 8) =
               *(undefined8 *)((long)(iVar1 + local_8c) * 8 + param_3);
        }
      }
      else {
        *(undefined8 *)((long)__ptr + 0xe8) = 0;
        *(undefined8 *)((long)__ptr + 0x70) = 0;
        *(undefined8 *)((long)__ptr + 0x68) = 0;
        *(undefined8 *)((long)__ptr + 0x88) = 0;
        *(undefined8 *)((long)__ptr + 0x90) = 0;
        *(long *)((long)__ptr + 0x78) = (long)((local_90 - iVar1) + -1);
        FUN_0011a742((long)__ptr + 0x40,(long)__ptr + 0x98,(long)__ptr + 0x38);
        for (local_8c = iVar1; local_8c < local_90 + -1; local_8c = local_8c + 1) {
          sVar9 = strlen(*(char **)(param_3 + (long)local_8c * 8));
          FUN_0011a1b8((long)__ptr + 0x40,(long)__ptr + 0x98,
                       *(undefined8 *)(param_3 + (long)local_8c * 8),sVar9 + 1,0,0,1);
        }
      }
      if (*(long *)(param_3 + (long)local_90 * 8) == 0) {
        *param_4 = local_90;
      }
      else {
        *param_4 = local_90 + 1;
      }
      uVar7 = 1;
    }
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar7;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: get_relative_timestamp @ 0x1916a

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8
get_relative_timestamp(double param_1,undefined8 param_2,long param_3,long param_4,long param_5,
            undefined8 param_6)

{
  uint uVar1;
  char cVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined8 local_30;
  double local_28;
  double local_20;
  double local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_30 = param_2;
  cVar2 = FUN_001196ab(&local_30,param_3 + 4);
  if (cVar2 == '\0') {
    uVar3 = 0;
  }
  else {
    uVar1 = *(uint *)(param_3 + 4);
    if ((uVar1 != 2) && (uVar1 < 3)) {
      if (uVar1 == 0) {
        *(undefined4 *)(param_3 + 4) = 1;
      }
      else if (uVar1 == 1) {
        *(undefined4 *)(param_3 + 4) = 0;
      }
    }
    cVar2 = FUN_0013223a(local_30,0,&local_28,PTR_strtod_00148fe8);
    if (cVar2 == '\0') {
      uVar3 = 0;
    }
    else {
      local_18 = modf(local_28 * param_1,&local_20);
      local_18 = _DAT_0013c098 * local_18;
      if ((double)DAT_0013c000 <= local_18) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("nanosec < nanosec_per_sec","parser.c",0xc4d,"get_relative_timestamp");
      }
      *(long *)(param_3 + 8) = (long)((double)param_4 - local_20);
      if (local_20 < 0.0 != param_4 < *(long *)(param_3 + 8)) {
        error(1,0,param_6,local_30);
      }
      *(long *)(param_3 + 0x10) = (long)((double)param_5 - local_18);
      if ((double)param_5 < local_18) {
        *(long *)(param_3 + 0x10) = *(long *)(param_3 + 0x10) + DAT_0013c000;
        *(long *)(param_3 + 8) = *(long *)(param_3 + 8) + -1;
      }
      uVar3 = 1;
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar3;
}




// Function: parse_time @ 0x1938f

/* WARNING: Removing unreachable block (ram,0x0011946c) */

undefined8 parse_time(undefined8 param_1,undefined8 param_2,undefined4 *param_3)

{
  char cVar1;
  undefined8 uVar2;
  undefined *puVar3;
  char *pcVar4;
  undefined *puVar5;
  long in_FS_OFFSET;
  undefined4 uVar6;
  int local_70;
  undefined4 local_6c;
  undefined8 local_68;
  long local_60;
  undefined8 local_58;
  undefined8 local_50;
  long local_48;
  long local_40;
  long local_38;
  undefined8 local_30;
  undefined4 local_28;
  int iStack_24;
  undefined8 local_20;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_58 = gettext("arithmetic overflow while converting %s days to a number of seconds");
  local_6c = *param_3;
  cVar1 = FUN_001145ab(param_2,param_3,&local_68);
  if (cVar1 == '\x01') {
    local_50 = local_68;
    local_38 = DAT_00149968;
    local_30 = DAT_00149970;
    cVar1 = FUN_001196ab(&local_68,&local_70);
    if ((cVar1 != '\0') && (local_70 == 1)) {
      local_48 = local_38 + 0x1517f;
      local_38 = local_38 + 0x1517f;
    }
    local_68 = local_50;
    cVar1 = FUN_0011916a(DAT_0013c070,local_50,&local_28,local_38,local_30,local_58);
    if (cVar1 == '\x01') {
      local_40 = FUN_0010eecc(param_1,local_50);
      *(ulong *)(local_40 + 0x38) = CONCAT44(iStack_24,local_28);
      *(undefined8 *)(local_40 + 0x40) = local_20;
      *(undefined8 *)(local_40 + 0x48) = local_18;
      uVar6 = FUN_001144ce(local_20);
      *(undefined4 *)(local_40 + 0x24) = uVar6;
      if ((DAT_00149980 & 1) != 0) {
        fprintf(stderr,"inserting %s\n",*(undefined8 *)(local_40 + 8));
        if (iStack_24 == 0) {
          puVar5 = &DAT_0013bf35;
        }
        else if (iStack_24 == 1) {
          puVar5 = &DAT_0013bf32;
        }
        else if (iStack_24 == 2) {
          puVar5 = &DAT_0013bf2c;
        }
        else {
          puVar5 = &DAT_0013bf2f;
        }
        if (iStack_24 == 0) {
          puVar3 = &DAT_0013bf40;
        }
        else if (iStack_24 == 1) {
          puVar3 = &DAT_0013bf3d;
        }
        else if (iStack_24 == 2) {
          puVar3 = &DAT_0013bf38;
        }
        else {
          puVar3 = &DAT_0013bf3b;
        }
        fprintf(stderr,"    type: %s    %s  ",puVar3,puVar5);
        local_60 = *(long *)(local_40 + 0x40);
        pcVar4 = ctime(&local_60);
        fprintf(stderr,"%lu %s",*(undefined8 *)(local_40 + 0x40),pcVar4);
        if (iStack_24 == 2) {
          local_60 = *(long *)(local_40 + 0x40) + 0x15180;
          pcVar4 = ctime(&local_60);
          fprintf(stderr,"                 <  %lu %s",local_60,pcVar4);
        }
      }
      uVar2 = 1;
    }
    else {
      *param_3 = local_6c;
      uVar2 = 0;
    }
  }
  else {
    uVar2 = 0;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: get_comp_type @ 0x196ab

undefined8 get_comp_type(long *param_1,undefined4 *param_2)

{
  if (*(char *)*param_1 == '+') {
    *param_2 = 0;
    *param_1 = *param_1 + 1;
  }
  else if (*(char *)*param_1 == '-') {
    *param_2 = 1;
    *param_1 = *param_1 + 1;
  }
  else {
    *param_2 = 2;
  }
  return 1;
}




// Function: get_num @ 0x19722

bool get_num(long param_1,undefined8 param_2,long param_3)

{
  char cVar1;
  int iVar2;
  long in_FS_OFFSET;
  bool bVar3;
  long local_20;
  undefined1 local_18 [8];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_20 = param_1;
  if (param_1 == 0) {
    bVar3 = false;
  }
  else {
    if (param_3 != 0) {
      cVar1 = FUN_001196ab(&local_20,param_3);
      if (cVar1 != '\x01') {
        bVar3 = false;
        goto LAB_001197a9;
      }
    }
    iVar2 = FUN_0013291e(local_20,local_18,10,param_2,&DAT_0013bdf6);
    bVar3 = iVar2 == 0;
  }
LAB_001197a9:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return bVar3;
}




// Function: insert_num @ 0x197bf

long insert_num(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  undefined *puVar2;
  long lVar3;
  undefined *puVar4;
  long in_FS_OFFSET;
  int local_2c;
  undefined8 local_28;
  undefined8 local_20;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_001145ab(param_1,param_2,&local_28);
  if ((cVar1 == '\0') || (cVar1 = FUN_00119722(local_28,&local_20,&local_2c), cVar1 == '\0')) {
    lVar3 = 0;
  }
  else {
    local_18 = FUN_0010eecc(param_3,local_28);
    *(int *)(local_18 + 0x38) = local_2c;
    *(undefined8 *)(local_18 + 0x40) = local_20;
    lVar3 = local_18;
    if ((DAT_00149980 & 1) != 0) {
      fprintf(stderr,"inserting %s\n",*(undefined8 *)(local_18 + 8));
      if (local_2c == 0) {
        puVar4 = &DAT_0013bf35;
      }
      else if (local_2c == 1) {
        puVar4 = &DAT_0013bf32;
      }
      else if (local_2c == 2) {
        puVar4 = &DAT_0013bf7a;
      }
      else {
        puVar4 = &DAT_0013bf2f;
      }
      if (local_2c == 0) {
        puVar2 = &DAT_0013bf40;
      }
      else if (local_2c == 1) {
        puVar2 = &DAT_0013bf3d;
      }
      else if (local_2c == 2) {
        puVar2 = &DAT_0013bf38;
      }
      else {
        puVar2 = &DAT_0013bf3b;
      }
      fprintf(stderr,"    type: %s    %s  ",puVar2,puVar4);
      fprintf(stderr,"%lu\n",*(undefined8 *)(local_18 + 0x40));
      lVar3 = local_18;
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: open_output_file @ 0x19964

void open_output_file(char *param_1,undefined8 *param_2)

{
  undefined1 uVar1;
  int iVar2;
  undefined8 uVar3;
  int *piVar4;
  
  *param_2 = 0;
  uVar3 = FUN_0012de20(0);
  param_2[4] = uVar3;
  iVar2 = strcmp(param_1,"/dev/stderr");
  if (iVar2 == 0) {
    param_2[1] = stderr;
    uVar3 = gettext("standard error");
    param_2[2] = uVar3;
  }
  else {
    iVar2 = strcmp(param_1,"/dev/stdout");
    if (iVar2 == 0) {
      param_2[1] = stdout;
      uVar3 = gettext("standard output");
      param_2[2] = uVar3;
    }
    else {
      uVar3 = FUN_00110ce6(DAT_001499f0,param_1);
      param_2[1] = uVar3;
      param_2[2] = param_1;
      if (param_2[1] == 0) {
        piVar4 = __errno_location();
        FUN_00110aa8(*piVar4,param_1);
      }
    }
  }
  uVar1 = FUN_0011876f(param_2[1]);
  *(undefined1 *)(param_2 + 3) = uVar1;
  return;
}




// Function: open_stdout @ 0x19a85

void open_stdout(undefined8 param_1)

{
  FUN_00119964("/dev/stdout",param_1);
  return;
}




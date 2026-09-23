// Function: main @ 0x7370

undefined4 main(int param_1,undefined8 *param_2)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  char *pcVar4;
  int *piVar5;
  undefined *puVar6;
  
  puVar6 = (undefined *)*param_2;
  if (puVar6 == (undefined *)0x0) {
    puVar6 = &DAT_0012b132;
  }
  FUN_00122cb0(puVar6);
  FUN_0010d970();
  DAT_0013e618 = 0;
  DAT_0013e604 = 0;
  DAT_0013e608 = 0;
  DAT_0013e5f8 = 0xffffff9c;
  cVar1 = FUN_00115dd0();
  if (cVar1 != '\0') {
    FUN_00115c30();
  }
  DAT_0013e610 = FUN_0010e4d0("w");
  if (DAT_0013e610 != 0) {
    FUN_0010de90(&DAT_0013e620);
    setlocale(6,"");
    bindtextdomain("findutils","/usr/local/share/locale");
    textdomain("findutils");
    iVar2 = FUN_0012a3c0(FUN_001175a0);
    if (iVar2 == 0) {
      iVar2 = FUN_0010dd00(param_1,param_2);
      if ((DAT_0013e660 & 2) != 0) {
        DAT_0013e670 = FUN_0010db70;
      }
      if ((DAT_0013e660 & 0x80) != 0) {
        pcVar4 = ctime(&DAT_0013e648);
        __fprintf_chk(stderr,1,"cur_day_start = %s",pcVar4);
      }
      uVar3 = FUN_0010c6f0(param_1,param_2,iVar2);
      cVar1 = FUN_00108210(param_1 - iVar2,param_2 + iVar2);
      if (cVar1 != '\0') {
        FUN_00109ee0(uVar3);
        FUN_0010e360();
      }
      return DAT_0013e604;
    }
    uVar3 = dcgettext(0,"The atexit library function failed",5);
    piVar5 = __errno_location();
                    /* WARNING: Subroutine does not return */
    error(1,*piVar5,uVar3);
  }
  uVar3 = dcgettext(0,"Failed to initialize shared-file hash table",5);
  piVar5 = __errno_location();
                    /* WARNING: Subroutine does not return */
  error(1,*piVar5,uVar3);
}




// Function: symlink_loop @ 0x7640

bool symlink_loop(undefined8 param_1)

{
  int iVar1;
  int *piVar2;
  long in_FS_OFFSET;
  bool bVar3;
  undefined1 auStack_a8 [152];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = (*DAT_0013e670)(param_1,auStack_a8);
  bVar3 = false;
  if (iVar1 != 0) {
    piVar2 = __errno_location();
    bVar3 = *piVar2 == 0x28;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: inside_dir @ 0x76a0

void inside_dir(int param_1)

{
  if ((DAT_0013e018._1_1_ & 2) == 0) {
    return;
  }
  if ((param_1 != -100) && (param_1 < 0)) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("dir_fd == AT_FDCWD || dir_fd >= 0","ftsfind.c",0x6b,"inside_dir");
  }
  DAT_0013e5f8 = param_1;
  if (-1 < DAT_0013e010) {
    return;
  }
  if (param_1 == -100) {
    DAT_0013e010 = 0xffffff9c;
    return;
  }
  DAT_0013e010 = FUN_00117560();
  return;
}




// Function: left_dir @ 0x7720

void left_dir(void)

{
  if (((DAT_0013e018._1_1_ & 2) != 0) && (-1 < DAT_0013e010)) {
    close(DAT_0013e010);
    DAT_0013e010 = 0xffffffff;
    return;
  }
  return;
}




// Function: get_fts_info_name @ 0x7750

char * get_fts_info_name(undefined4 param_1)

{
  switch(param_1) {
  default:
    __sprintf_chk(&DAT_0013e598,1,0xe,&DAT_0012b07d,param_1);
    return &DAT_0013e598;
  case 1:
    return "FTS_D";
  case 2:
    return "FTS_DC";
  case 3:
    return "FTS_DEFAULT";
  case 4:
    return "FTS_DNR";
  case 5:
    return "FTS_DOT";
  case 6:
    return "FTS_DP";
  case 7:
    return "FTS_ERR";
  case 8:
    return "FTS_F";
  case 9:
    return "FTS_INIT";
  case 10:
    return "FTS_NS";
  case 0xb:
    return "FTS_NSOK";
  case 0xc:
    return "FTS_SL";
  case 0xd:
    return "FTS_SLNONE";
  case 0xe:
    return "FTS_W";
  }
}




// Function: visit @ 0x7880

void visit(long param_1,long param_2,undefined8 param_3)

{
  undefined8 uVar1;
  
  DAT_0013e5f0 = *(undefined8 *)(param_2 + 0x30);
  DAT_0013e5e4 = 1 < (ushort)(*(short *)(param_2 + 0x68) - 10U);
  DAT_0013e5f8 = *(undefined4 *)(param_1 + 0x2c);
  uVar1 = FUN_0010b5e0();
  FUN_0010e2a0(*(undefined8 *)(param_2 + 0x38),param_3,uVar1);
  if (DAT_0013e600 == '\0') {
    return;
  }
  FUN_0011a190(param_1,param_2,4);
  return;
}




// Function: issue_loop_warning @ 0x7950

void issue_loop_warning(long *param_1)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  
  if ((*(uint *)(param_1 + 0x11) & 0xf000) != 0xa000) {
    uVar1 = FUN_00107900(*(undefined8 *)(*param_1 + 0x38),*(undefined8 *)(*param_1 + 0x48),
                         DAT_0013e68c);
    uVar2 = FUN_0010dfb0(0,param_1[7]);
    uVar3 = dcgettext(0,"File system loop detected; %s is part of the same file system loop as %s.",
                      5);
                    /* WARNING: Subroutine does not return */
    error(0,0,uVar3,uVar2,uVar1);
  }
  uVar1 = FUN_0010dfb0(0,param_1[7]);
  uVar2 = dcgettext(0,
                    "Symbolic link %s is part of a loop in the directory hierarchy; we have already visited the directory to which it points."
                    ,5);
                    /* WARNING: Subroutine does not return */
  error(0,0,uVar2,uVar1);
}




// Function: consider_visiting @ 0x7a00

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void consider_visiting(long param_1,long param_2)

{
  int iVar1;
  undefined1 uVar2;
  undefined1 uVar3;
  char cVar4;
  short sVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  uint uVar10;
  bool bVar11;
  undefined8 uVar12;
  long in_FS_OFFSET;
  uint local_dc;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined8 uStack_50;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (((byte)DAT_0013e660 & 4) != 0) {
    uVar8 = FUN_00124e50(2,DAT_0013e68c,*(undefined8 *)(param_2 + 0x30));
    uVar9 = FUN_00124e50(1,DAT_0013e68c,*(undefined8 *)(param_2 + 0x38));
    iVar1 = DAT_0013e014;
    uVar12 = *(undefined8 *)(param_2 + 0x58);
    uVar6 = FUN_00107750(*(undefined2 *)(param_2 + 0x68));
    uVar7 = FUN_00124e50(0,DAT_0013e68c,*(undefined8 *)(param_2 + 0x38));
    __fprintf_chk(stderr,1,
                  "consider_visiting (early): %s: fts_info=%-6s, fts_level=%2d, prev_depth=%d fts_path=%s, fts_accpath=%s\n"
                  ,uVar7,uVar6,uVar12,iVar1,uVar9,uVar8);
  }
  if (((*(short *)(param_2 + 0x68) == 6) || ((long)DAT_0013e014 < *(long *)(param_2 + 0x58))) ||
     (*(long *)(param_2 + 0x58) == 0)) {
    FUN_00107720();
  }
  FUN_001076a0(*(undefined4 *)(param_1 + 0x2c));
  uStack_d0 = *(undefined8 *)(param_2 + 0x78);
  sVar5 = *(short *)(param_2 + 0x68);
  DAT_0013e014 = (int)*(long *)(param_2 + 0x58);
  if (sVar5 == 7) {
LAB_00107e90:
    FUN_0010e060(*(undefined4 *)(param_2 + 0x40),*(undefined8 *)(param_2 + 0x38));
    goto LAB_00107c38;
  }
  if (sVar5 == 4) {
    FUN_0010e060(*(undefined4 *)(param_2 + 0x40),*(undefined8 *)(param_2 + 0x38));
    if (DAT_0013e620 != '\0') goto LAB_00107c38;
    goto LAB_00107e31;
  }
  if (sVar5 == 2) {
    FUN_00107950(param_2);
    DAT_0013e604 = 1;
    goto LAB_00107c38;
  }
  if (sVar5 == 0xd) {
    cVar4 = FUN_00107640(*(undefined8 *)(param_2 + 0x30));
    if (cVar4 != '\0') {
      FUN_0010e060(0x28,*(undefined8 *)(param_2 + 0x38));
      goto LAB_00107c38;
    }
LAB_00107e31:
    sVar5 = *(short *)(param_2 + 0x68);
  }
  else if (sVar5 == 10) {
    if (*(long *)(param_2 + 0x58) == 0) goto LAB_00107e90;
    cVar4 = FUN_00107640(*(undefined8 *)(param_2 + 0x30));
    if (cVar4 != '\0') {
      FUN_0010e060(0x28,*(undefined8 *)(param_2 + 0x38));
      goto LAB_00107c38;
    }
    FUN_0010e060(*(undefined4 *)(param_2 + 0x40),*(undefined8 *)(param_2 + 0x38));
    goto LAB_00107e31;
  }
  if ((ushort)(sVar5 - 10U) < 2) {
    if (DAT_0013e5e4 != '\0') {
                    /* WARNING: Subroutine does not return */
      __assert_fail("!state.have_stat","ftsfind.c",0x17c,"consider_visiting");
    }
    local_dc = DAT_0013e5e8;
    if ((sVar5 != 0xb) && (DAT_0013e5e8 != 0)) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("ent->fts_info == FTS_NSOK || state.type == 0","ftsfind.c",0x17d,
                    "consider_visiting");
    }
    DAT_0013e5e0 = (undefined4)*(undefined8 *)(param_2 + 0x58);
    if (DAT_0013e5e8 != 0) goto LAB_00107b66;
    sVar5 = *(short *)(param_2 + 0x68);
LAB_00107d4b:
    if (((ushort)(sVar5 - 1U) < 2) || (sVar5 == 6)) goto LAB_00107ba8;
    uVar12 = 0;
    if (-1 < DAT_0013e624) goto LAB_00107bc5;
    uVar10 = (uint)CONCAT71(DAT_0013e660._1_7_,(byte)DAT_0013e660);
    bVar11 = false;
LAB_00107bf5:
    if (*(long *)(param_2 + 0x58) < (long)DAT_0013e628) {
      if ((uVar10 & 4) != 0) goto LAB_00107d91;
    }
    else {
      if ((uVar10 & 4) != 0) goto LAB_00107d96;
LAB_00107c12:
      if (bVar11 == false) {
        FUN_00107880(param_1,param_2,&local_d8);
      }
    }
    if (*(short *)(param_2 + 0x68) != 6) goto LAB_00107c38;
  }
  else {
    local_d8 = *(undefined8 *)(param_2 + 0x70);
    uStack_d0 = *(undefined8 *)(param_2 + 0x78);
    local_c8 = *(undefined8 *)(param_2 + 0x80);
    uStack_c0 = *(undefined8 *)(param_2 + 0x88);
    _DAT_0013e5e4 = 0x101;
    local_b8 = *(undefined8 *)(param_2 + 0x90);
    uStack_b0 = *(undefined8 *)(param_2 + 0x98);
    local_a8 = *(undefined8 *)(param_2 + 0xa0);
    uStack_a0 = *(undefined8 *)(param_2 + 0xa8);
    local_98 = *(undefined8 *)(param_2 + 0xb0);
    uStack_90 = *(undefined8 *)(param_2 + 0xb8);
    local_88 = *(undefined8 *)(param_2 + 0xc0);
    uStack_80 = *(undefined8 *)(param_2 + 200);
    local_78 = *(undefined8 *)(param_2 + 0xd0);
    uStack_70 = *(undefined8 *)(param_2 + 0xd8);
    local_68 = *(undefined8 *)(param_2 + 0xe0);
    uStack_60 = *(undefined8 *)(param_2 + 0xe8);
    local_58 = *(undefined8 *)(param_2 + 0xf0);
    uStack_50 = *(undefined8 *)(param_2 + 0xf8);
    local_dc = (uint)uStack_c0;
    DAT_0013e5e8 = (uint)uStack_c0;
    if ((uint)uStack_c0 == 0) {
      uVar12 = FUN_00124e50(0,DAT_0013e68c,*(undefined8 *)(param_2 + 0x38));
      uVar8 = dcgettext(0,"WARNING: file %s appears to have mode 0000",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar8,uVar12);
    }
    DAT_0013e5e0 = (undefined4)*(undefined8 *)(param_2 + 0x58);
LAB_00107b66:
    DAT_0013e5e8 = local_dc;
    cVar4 = FUN_0010e150(&local_dc,*(undefined8 *)(param_2 + 0x38),param_2 + 0x100,&local_d8,0);
    if (cVar4 == '\0') goto LAB_00107c38;
    sVar5 = *(short *)(param_2 + 0x68);
    if ((local_dc & 0xf000) != 0x4000) goto LAB_00107d4b;
LAB_00107ba8:
    if (sVar5 == 0xb) {
      FUN_0011a190(param_1,param_2,1);
      goto LAB_00107c38;
    }
    uVar12 = 1;
    bVar11 = false;
    if (-1 < DAT_0013e624) {
LAB_00107bc5:
      if (*(long *)(param_2 + 0x58) < (long)DAT_0013e624) {
        bVar11 = false;
      }
      else {
        FUN_0011a190(param_1,param_2,4);
        sVar5 = *(short *)(param_2 + 0x68);
        bVar11 = (long)DAT_0013e624 < *(long *)(param_2 + 0x58);
      }
    }
    uVar10 = (uint)CONCAT71(DAT_0013e660._1_7_,(byte)DAT_0013e660);
    if (sVar5 == 1) {
      if (DAT_0013e620 != '\0') goto LAB_00107bf5;
      if (((byte)DAT_0013e660 & 4) == 0) goto LAB_00107c38;
LAB_00107d91:
      bVar11 = true;
LAB_00107d96:
      uVar3 = DAT_0013e5e5;
      uVar2 = DAT_0013e5e4;
      uVar8 = FUN_00107750();
      uVar9 = FUN_00124e50(0,DAT_0013e68c,*(undefined8 *)(param_2 + 0x38));
      __fprintf_chk(stderr,1,
                    "consider_visiting (late): %s: fts_info=%-6s, isdir=%d ignore=%d have_stat=%d have_type=%d \n"
                    ,uVar9,uVar8,uVar12,bVar11,uVar2,uVar3);
      goto LAB_00107c12;
    }
    if ((sVar5 != 6) || (DAT_0013e620 == '\0')) goto LAB_00107bf5;
    if (((byte)DAT_0013e660 & 4) != 0) goto LAB_00107d91;
  }
  DAT_0013e600 = 0;
LAB_00107c38:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: find @ 0x7fe0

undefined8 find(char *param_1)

{
  int iVar1;
  int iVar2;
  size_t sVar3;
  long lVar4;
  int *piVar5;
  long lVar6;
  char *pcVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  long in_FS_OFFSET;
  char *local_48;
  undefined8 local_40;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  sVar3 = strlen(param_1);
  DAT_0013e5fc = (undefined4)sVar3;
  FUN_001076a0(0xffffff9c);
  local_40 = 0;
  if (DAT_0013e668 == 1) {
    DAT_0013e018 = DAT_0013e018 | 3;
  }
  else if (DAT_0013e668 == 2) {
    DAT_0013e018 = DAT_0013e018 | 0x11;
  }
  else if (DAT_0013e668 == 0) {
    DAT_0013e018 = DAT_0013e018 | 0x10;
  }
  if (DAT_0013e62d != '\0') {
    DAT_0013e018 = DAT_0013e018 | 0x40;
  }
  iVar2 = -0x80000000;
  local_48 = param_1;
  lVar4 = FUN_001195e0(&local_48,DAT_0013e018,0);
  piVar5 = __errno_location();
  if (lVar4 == 0) {
    uVar8 = FUN_0010dfb0(0,param_1);
    uVar9 = dcgettext(0,"cannot search %s",5);
                    /* WARNING: Subroutine does not return */
    error(0,*piVar5,uVar9,uVar8);
  }
  while( true ) {
    *piVar5 = 0;
    lVar6 = FUN_00119b40(lVar4);
    if (lVar6 == 0) break;
    iVar1 = (int)*(undefined8 *)(lVar6 + 0x58);
    if ((DAT_0013e608 != '\0') && (iVar2 != iVar1)) {
      FUN_0010d940();
      iVar1 = *(int *)(lVar6 + 0x58);
    }
    iVar2 = iVar1;
    DAT_0013e5e8 = *(int *)(lVar6 + 0x88);
    DAT_0013e618 = 0;
    DAT_0013e5e4 = 0;
    DAT_0013e5e5 = DAT_0013e5e8 != 0;
    FUN_00107a00(lVar4);
  }
  if (*piVar5 == 0) {
    iVar2 = FUN_00119950(lVar4);
    if (iVar2 == 0) {
      if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
        return 1;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    pcVar7 = (char *)dcgettext(0,"failed to restore working directory after searching %s",5);
  }
  else {
    param_1 = (char *)FUN_0010dfb0(0,param_1);
    pcVar7 = "failed to read file names from file system at or below %s";
  }
                    /* WARNING: Subroutine does not return */
  error(0,*piVar5,pcVar7,param_1);
}




// Function: process_all_startpoints @ 0x8210

undefined4 process_all_startpoints(int param_1,undefined8 *param_2)

{
  FILE *pFVar1;
  char cVar2;
  int iVar3;
  int iVar4;
  undefined4 uVar5;
  undefined8 uVar6;
  long lVar7;
  char *__s;
  size_t sVar8;
  undefined8 uVar9;
  int *piVar10;
  undefined8 uVar11;
  long in_FS_OFFSET;
  FILE *local_180;
  int local_16c;
  stat local_168;
  stat local_d8;
  undefined2 local_42;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if ((param_1 < 1) || (cVar2 = FUN_0010dca0(*param_2,1), cVar2 != '\0')) {
    pFVar1 = DAT_0013e690;
    if (DAT_0013e690 == (FILE *)0x0) {
      local_42 = 0x2e;
      uVar5 = FUN_00107fe0(&local_42);
      goto LAB_00108351;
    }
    iVar3 = strcmp((char *)DAT_0013e690,"-");
    if (iVar3 == 0) {
      if (DAT_0013e698 != '\0') {
        uVar6 = dcgettext(0,
                          "option -files0-from reading from standard input cannot be combined with -ok, -okdir"
                          ,5);
                    /* WARNING: Subroutine does not return */
        error(1,0,&DAT_0012f889,uVar6);
      }
      uVar6 = dcgettext(0,"(standard input)",5);
      uVar6 = FUN_0010dfb0(0,uVar6);
      local_180 = stdin;
    }
    else {
      uVar6 = FUN_0010dfb0(0,pFVar1);
      local_180 = fopen((char *)DAT_0013e690,"r");
      if (local_180 == (FILE *)0x0) {
        uVar11 = dcgettext(0,"cannot open %s for reading",5);
        piVar10 = __errno_location();
                    /* WARNING: Subroutine does not return */
        error(1,*piVar10,uVar11,uVar6);
      }
      iVar3 = fileno(local_180);
      if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("fd >= 0","ftsfind.c",0x262,"process_all_startpoints");
      }
      if ((((DAT_0013e698 != '\0') && (iVar4 = fstat(iVar3,&local_168), iVar4 == 0)) &&
          (iVar4 = fstat(0,&local_d8), iVar4 == 0)) &&
         ((local_168.st_ino == local_d8.st_ino && (local_168.st_dev == local_d8.st_dev)))) {
        uVar11 = dcgettext(0,
                           "option -files0-from: standard input must not refer to the same file when combined with -ok, -okdir"
                           ,5);
                    /* WARNING: Subroutine does not return */
        error(1,0,"%s: %s\n",uVar11,uVar6);
      }
      FUN_001174f0(iVar3,1);
    }
    lVar7 = FUN_001171e0(local_180);
  }
  else {
    local_180 = DAT_0013e690;
    if (DAT_0013e690 != (FILE *)0x0) {
      uVar6 = FUN_0010dfb0(0,*param_2);
      uVar11 = dcgettext(0,"extra operand %s",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar11,uVar6);
    }
    uVar6 = 0;
    lVar7 = FUN_001171b0(param_2);
  }
  if (lVar7 == 0) {
LAB_0010866d:
                    /* WARNING: Subroutine does not return */
    FUN_00126d50();
  }
  uVar5 = 1;
  do {
    __s = (char *)FUN_00117220(lVar7,&local_16c);
    if (__s == (char *)0x0) {
      if (local_16c == 3) goto LAB_0010866d;
      if (local_16c == 4) {
        uVar11 = dcgettext(0,"%s: read error",5);
        piVar10 = __errno_location();
                    /* WARNING: Subroutine does not return */
        error(0,*piVar10,uVar11,uVar6);
      }
      if (local_16c != 2) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("!\"unexpected error code from argv_iter\"","ftsfind.c",0x2a3,
                      "process_all_startpoints");
      }
      goto LAB_00108316;
    }
    if (*__s == '\0') {
      if (DAT_0013e690 == (FILE *)0x0) {
        uVar6 = FUN_0010dfb0(0,__s);
                    /* WARNING: Subroutine does not return */
        error(0,2,&DAT_0013155a,uVar6);
      }
      uVar11 = FUN_001172c0(lVar7);
      uVar9 = dcgettext(0,"invalid zero-length file name",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,"%s:%lu: %s",uVar6,uVar11,uVar9);
    }
    if ((DAT_0013e690 == (FILE *)0x0) && (cVar2 = FUN_0010dca0(__s,1), cVar2 != '\0'))
    goto LAB_00108316;
    sVar8 = strlen(__s);
    DAT_0013e5fc = (undefined4)sVar8;
    cVar2 = FUN_00107fe0(__s);
  } while (cVar2 != '\0');
  uVar5 = 0;
LAB_00108316:
  FUN_001172e0(lVar7);
  if ((((char)uVar5 != '\0') && (DAT_0013e690 != (FILE *)0x0)) &&
     ((iVar3 = ferror(local_180), iVar3 != 0 || (iVar3 = FUN_00117850(local_180), iVar3 != 0)))) {
    uVar11 = dcgettext(0,"error reading %s",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar11,uVar6);
  }
LAB_00108351:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar5;
}




// Function: is_fts_enabled @ 0x86d0

undefined8 is_fts_enabled(undefined4 *param_1)

{
  *param_1 = DAT_0013e018;
  return 1;
}




// Function: pred_prune @ 0x86f0

undefined8 pred_prune(undefined8 param_1,long param_2)

{
  if (DAT_0013e620 != '\0') {
    if (DAT_0013e5e4 == '\0') {
                    /* WARNING: Subroutine does not return */
      __assert_fail("state.have_stat","pred.c",0x355,"pred_prune");
    }
    if ((param_2 != 0) && ((*(uint *)(param_2 + 0x18) & 0xf000) == 0x4000)) {
      DAT_0013e600 = 1;
      return 1;
    }
  }
  return 1;
}




// Function: ts_difference @ 0x8750

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

double ts_difference(time_t param_1,long param_2,time_t param_3,long param_4)

{
  double dVar1;
  
  dVar1 = difftime(param_1,param_3);
  return (double)(param_2 - param_4) * _DAT_0012b960 + dVar1;
}




// Function: compare_ts @ 0x8790

undefined8 compare_ts(long param_1,long param_2,long param_3,long param_4)

{
  undefined8 uVar1;
  double dVar2;
  
  if ((param_1 == param_3) && (param_2 == param_4)) {
    return 0;
  }
  dVar2 = (double)FUN_00108750();
  uVar1 = 0xffffffff;
  if (0.0 <= dVar2) {
    uVar1 = 1;
  }
  return uVar1;
}




// Function: pred_timewindow @ 0x87d0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

uint pred_timewindow(undefined8 param_1,undefined8 param_2,long param_3,int param_4)

{
  uint uVar1;
  int iVar2;
  double dVar3;
  
  iVar2 = *(int *)(param_3 + 0x3c);
  if (iVar2 == 1) {
    uVar1 = FUN_00108790(param_1,param_2,*(undefined8 *)(param_3 + 0x40),
                         *(undefined8 *)(param_3 + 0x48));
    uVar1 = uVar1 >> 0x1f;
  }
  else {
    if (iVar2 != 2) {
      if (iVar2 != 0) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("0","pred.c",0x8a,"pred_timewindow");
      }
      iVar2 = FUN_00108790(param_1,param_2,*(undefined8 *)(param_3 + 0x40),
                           *(undefined8 *)(param_3 + 0x48));
      return (uint)(0 < iVar2);
    }
    dVar3 = (double)FUN_00108750(param_1,param_2,*(undefined8 *)(param_3 + 0x40),
                                 *(undefined8 *)(param_3 + 0x48));
    uVar1 = 0;
    if (_DAT_0012b968 < dVar3) {
      return (uint)(dVar3 <= (double)param_4);
    }
  }
  return uVar1;
}




// Function: pred_and @ 0x8870

void pred_and(undefined8 param_1,undefined8 param_2,long param_3)

{
  char cVar1;
  
  if (*(long *)(param_3 + 0x110) != 0) {
    cVar1 = FUN_0010e2a0();
    if (cVar1 == '\0') {
      return;
    }
  }
  FUN_0010e2a0(param_1,param_2,*(undefined8 *)(param_3 + 0x118));
  return;
}




// Function: pred_or @ 0x88c0

void pred_or(undefined8 param_1,undefined8 param_2,long param_3)

{
  char cVar1;
  
  if (*(long *)(param_3 + 0x110) != 0) {
    cVar1 = FUN_0010e2a0();
    if (cVar1 != '\0') {
      return;
    }
  }
  FUN_0010e2a0(param_1,param_2,*(undefined8 *)(param_3 + 0x118));
  return;
}




// Function: perform_delete @ 0x8910

bool perform_delete(int param_1)

{
  int iVar1;
  
  iVar1 = unlinkat(DAT_0013e5f8,DAT_0013e5f0,param_1);
  return iVar1 == 0;
}




// Function: pred_name_common @ 0x8940

bool pred_name_common(undefined8 param_1,char *param_2,int param_3)

{
  int iVar1;
  char *__name;
  
  __name = (char *)FUN_001176f0();
  FUN_00117810(__name);
  iVar1 = fnmatch(param_2,__name,param_3);
  free(__name);
  return iVar1 == 0;
}




// Function: is_ok @ 0x8990

void is_ok(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  undefined8 uVar2;
  int *piVar3;
  
  FUN_00117bc0(stdout);
  uVar2 = dcgettext(0,"< %s ... %s > ? ",5);
  iVar1 = __fprintf_chk(stderr,1,uVar2,param_1,param_2);
  if (-1 < iVar1) {
    FUN_00117bc0(stderr);
    FUN_001276e0();
    return;
  }
  uVar2 = dcgettext(0,"Failed to write prompt for -ok",5);
  piVar3 = __errno_location();
                    /* WARNING: Subroutine does not return */
  error(1,*piVar3,uVar2);
}




// Function: blank_rtrim @ 0x8a40

char * blank_rtrim(char *param_1,char *param_2)

{
  char *pcVar1;
  size_t sVar2;
  int iVar3;
  char *pcVar4;
  
  if (param_1 == (char *)0x0) {
    return (char *)0x0;
  }
  strcpy(param_2,param_1);
  sVar2 = strlen(param_2);
  iVar3 = (int)sVar2 + -1;
  if (iVar3 < 0) {
    pcVar4 = param_2 + (int)sVar2;
  }
  else {
    pcVar1 = param_2 + iVar3;
    do {
      pcVar4 = pcVar1;
      if ((*pcVar4 != ' ') && (*pcVar4 != '\t')) {
        pcVar4 = param_2 + (iVar3 + 1);
        break;
      }
      iVar3 = iVar3 + -1;
      pcVar1 = pcVar4 + -1;
    } while (iVar3 != -1);
  }
  *pcVar4 = '\0';
  return param_2;
}




// Function: pred_amin @ 0x8b70

void pred_amin(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined1 auVar1 [16];
  
  auVar1 = FUN_00125c40(param_2);
  FUN_001087d0(auVar1._0_8_,auVar1._8_8_,param_3,0x3c);
  return;
}




// Function: pred_anewer @ 0x8ba0

bool pred_anewer(undefined8 param_1,undefined8 param_2,long param_3)

{
  int iVar1;
  undefined1 auVar2 [16];
  
  if (*(int *)(param_3 + 0x3c) == 0) {
    auVar2 = FUN_00125c40(param_2);
    iVar1 = FUN_00108790(auVar2._0_8_,auVar2._8_8_,*(undefined8 *)(param_3 + 0x40),
                         *(undefined8 *)(param_3 + 0x48));
    return 0 < iVar1;
  }
                    /* WARNING: Subroutine does not return */
  __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0xa6,"pred_anewer");
}




// Function: pred_atime @ 0x8c00

void pred_atime(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined1 auVar1 [16];
  
  auVar1 = FUN_00125c40(param_2);
  FUN_001087d0(auVar1._0_8_,auVar1._8_8_,param_3,0x15180);
  return;
}




// Function: pred_cmin @ 0x8c40

void pred_cmin(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined1 auVar1 [16];
  
  auVar1 = FUN_00125c50(param_2);
  FUN_001087d0(auVar1._0_8_,auVar1._8_8_,param_3,0x3c);
  return;
}




// Function: pred_cnewer @ 0x8c70

bool pred_cnewer(undefined8 param_1,undefined8 param_2,long param_3)

{
  int iVar1;
  undefined1 auVar2 [16];
  
  if (*(int *)(param_3 + 0x3c) == 0) {
    auVar2 = FUN_00125c50(param_2);
    iVar1 = FUN_00108790(auVar2._0_8_,auVar2._8_8_,*(undefined8 *)(param_3 + 0x40),
                         *(undefined8 *)(param_3 + 0x48));
    return 0 < iVar1;
  }
                    /* WARNING: Subroutine does not return */
  __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",199,"pred_cnewer");
}




// Function: pred_comma @ 0x8cd0

void pred_comma(undefined8 param_1,undefined8 param_2,long param_3)

{
  if (*(long *)(param_3 + 0x110) != 0) {
    FUN_0010e2a0();
  }
  FUN_0010e2a0(param_1,param_2,*(undefined8 *)(param_3 + 0x118));
  return;
}




// Function: pred_ctime @ 0x8d10

void pred_ctime(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined1 auVar1 [16];
  
  auVar1 = FUN_00125c50(param_2);
  FUN_001087d0(auVar1._0_8_,auVar1._8_8_,param_3,0x15180);
  return;
}




// Function: pred_delete @ 0x8d40

char pred_delete(undefined8 param_1,long param_2)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  
  iVar2 = strcmp(DAT_0013e5f0,".");
  if (iVar2 != 0) {
    iVar2 = 0;
    if (DAT_0013e5e4 != '\0') {
      iVar2 = (uint)((*(uint *)(param_2 + 0x18) & 0xf000) == 0x4000) << 9;
    }
    cVar1 = FUN_00108910(iVar2);
    if (cVar1 == '\0') {
      piVar3 = __errno_location();
      cVar1 = DAT_0013e62e;
      if (*piVar3 == 2) {
        if (DAT_0013e62e != '\0') {
          *piVar3 = 0;
          return cVar1;
        }
      }
      else if (((*piVar3 == 0x15) && (iVar2 == 0)) && (cVar1 = FUN_00108910(0x200), cVar1 != '\0'))
      {
        return '\x01';
      }
      uVar4 = FUN_0010dfb0(0,param_1);
      uVar5 = dcgettext(0,"cannot delete %s",5);
                    /* WARNING: Subroutine does not return */
      error(0,*piVar3,uVar5,uVar4);
    }
  }
  return '\x01';
}




// Function: pred_empty @ 0x8e50

bool pred_empty(undefined8 param_1,long param_2)

{
  uint uVar1;
  int iVar2;
  int *piVar3;
  DIR *__dirp;
  dirent *pdVar4;
  undefined8 uVar5;
  bool bVar6;
  
  uVar1 = *(uint *)(param_2 + 0x18) & 0xf000;
  if (uVar1 != 0x4000) {
    bVar6 = false;
    if (uVar1 == 0x8000) {
      bVar6 = *(long *)(param_2 + 0x30) == 0;
    }
    return bVar6;
  }
  piVar3 = __errno_location();
  uVar5 = DAT_0013e5f0;
  *piVar3 = 0;
  iVar2 = FUN_0011e690(DAT_0013e5f8,uVar5,0x90900);
  if (-1 < iVar2) {
    __dirp = fdopendir(iVar2);
    if (__dirp == (DIR *)0x0) {
      uVar5 = FUN_0010dfb0(0,param_1);
                    /* WARNING: Subroutine does not return */
      error(0,*piVar3,&DAT_0013155a,uVar5);
    }
    *piVar3 = 0;
    do {
      pdVar4 = readdir(__dirp);
      if (pdVar4 == (dirent *)0x0) {
        bVar6 = true;
        goto LAB_00108f03;
      }
    } while ((pdVar4->d_name[0] == '.') &&
            ((pdVar4->d_name[1] == '\0' || (*(short *)(pdVar4->d_name + 1) == 0x2e))));
    bVar6 = false;
LAB_00108f03:
    if (*piVar3 != 0) {
      uVar5 = FUN_0010dfb0(0,param_1);
                    /* WARNING: Subroutine does not return */
      error(0,*piVar3,&DAT_0013155a,uVar5);
    }
    iVar2 = closedir(__dirp);
    if (iVar2 == 0) {
      return bVar6;
    }
  }
  uVar5 = FUN_0010dfb0(0,param_1);
                    /* WARNING: Subroutine does not return */
  error(0,*piVar3,&DAT_0013155a,uVar5);
}




// Function: pred_execdir @ 0x8ff0

void pred_execdir(void)

{
  FUN_0010a390(DAT_0013e5f0);
  return;
}




// Function: pred_false @ 0x9000

undefined8 pred_false(void)

{
  return 0;
}




// Function: pred_fls @ 0x9010

undefined8 pred_fls(undefined8 param_1,undefined8 param_2,long param_3)

{
  FUN_00116170(param_1,DAT_0013e5f8,DAT_0013e5f0,param_2,DAT_0013e638,DAT_0013e65c,
               *(undefined1 *)(param_3 + 0x28),*(undefined8 *)(param_3 + 0x40));
  return 1;
}




// Function: pred_fprint @ 0x9050

undefined8 pred_fprint(undefined8 param_1,undefined8 param_2,long param_3)

{
  FUN_00116960(*(undefined8 *)(param_3 + 0x40),*(undefined8 *)(param_3 + 0x58),
               *(undefined1 *)(param_3 + 0x50),&DAT_0012f889,param_1);
  return 1;
}




// Function: pred_fprint0 @ 0x9090

undefined8 pred_fprint0(char *param_1,undefined8 param_2,long param_3)

{
  FILE *__stream;
  
  __stream = *(FILE **)(param_3 + 0x40);
  fputs(param_1,__stream);
  putc(0,__stream);
  return 1;
}




// Function: pred_fstype @ 0x90c0

bool pred_fstype(undefined8 param_1,undefined8 param_2,long param_3)

{
  char *__s2;
  char *__s1;
  int iVar1;
  
  __s2 = *(char **)(param_3 + 0x38);
  __s1 = (char *)FUN_001104c0(param_2,param_1);
  iVar1 = strcmp(__s1,__s2);
  return iVar1 == 0;
}




// Function: pred_gid @ 0x90f0

ulong pred_gid(undefined8 param_1,long param_2,long param_3,undefined8 param_4,
                  undefined8 param_5)

{
  int iVar1;
  undefined7 uVar2;
  
  iVar1 = *(int *)(param_3 + 0x38);
  uVar2 = (undefined7)((ulong)param_5 >> 8);
  if (iVar1 == 1) {
    return CONCAT71(uVar2,(ulong)*(uint *)(param_2 + 0x20) < *(ulong *)(param_3 + 0x40)) &
           0xffffffff;
  }
  if (iVar1 != 2) {
    if (iVar1 != 0) {
      return 0;
    }
    return (ulong)(*(ulong *)(param_3 + 0x40) < (ulong)*(uint *)(param_2 + 0x20));
  }
  return CONCAT71(uVar2,(ulong)*(uint *)(param_2 + 0x20) == *(ulong *)(param_3 + 0x40)) & 0xffffffff
  ;
}




// Function: pred_group @ 0x9140

undefined4 pred_group(undefined8 param_1,long param_2,long param_3)

{
  return CONCAT31((int3)((uint)*(int *)(param_2 + 0x20) >> 8),
                  *(int *)(param_3 + 0x38) == *(int *)(param_2 + 0x20));
}




// Function: pred_ilname @ 0x9150

void pred_ilname(undefined8 param_1,long param_2,undefined8 param_3)

{
  FUN_00108ad0(param_1,*(undefined4 *)(param_2 + 0x18),param_3,1);
  return;
}




// Function: pred_iname @ 0x9170

void pred_iname(undefined8 param_1,undefined8 param_2,long param_3)

{
  FUN_00108940(param_1,*(undefined8 *)(param_3 + 0x38),0x10);
  return;
}




// Function: pred_inum @ 0x9190

ulong pred_inum(undefined8 param_1,long param_2,long param_3,undefined8 param_4,
                  undefined8 param_5)

{
  int iVar1;
  undefined7 uVar2;
  
  iVar1 = *(int *)(param_3 + 0x38);
  uVar2 = (undefined7)((ulong)param_5 >> 8);
  if (iVar1 == 1) {
    return CONCAT71(uVar2,*(ulong *)(param_2 + 8) < *(ulong *)(param_3 + 0x40)) & 0xffffffff;
  }
  if (iVar1 != 2) {
    if (iVar1 != 0) {
      return 0;
    }
    return (ulong)(*(ulong *)(param_3 + 0x40) < *(ulong *)(param_2 + 8));
  }
  return CONCAT71(uVar2,*(long *)(param_2 + 8) == *(long *)(param_3 + 0x40)) & 0xffffffff;
}




// Function: pred_ipath @ 0x91e0

bool pred_ipath(char *param_1,undefined8 param_2,long param_3)

{
  int iVar1;
  
  iVar1 = fnmatch(*(char **)(param_3 + 0x38),param_1,0x10);
  return iVar1 == 0;
}




// Function: pred_links @ 0x9210

ulong pred_links(undefined8 param_1,long param_2,long param_3,undefined8 param_4,
                  undefined8 param_5)

{
  int iVar1;
  undefined7 uVar2;
  
  iVar1 = *(int *)(param_3 + 0x38);
  uVar2 = (undefined7)((ulong)param_5 >> 8);
  if (iVar1 == 1) {
    return CONCAT71(uVar2,*(ulong *)(param_2 + 0x10) < *(ulong *)(param_3 + 0x40)) & 0xffffffff;
  }
  if (iVar1 != 2) {
    if (iVar1 != 0) {
      return 0;
    }
    return (ulong)(*(ulong *)(param_3 + 0x40) < *(ulong *)(param_2 + 0x10));
  }
  return CONCAT71(uVar2,*(long *)(param_2 + 0x10) == *(long *)(param_3 + 0x40)) & 0xffffffff;
}




// Function: pred_lname @ 0x9260

void pred_lname(undefined8 param_1,long param_2,undefined8 param_3)

{
  FUN_00108ad0(param_1,*(undefined4 *)(param_2 + 0x18),param_3,0);
  return;
}




// Function: pred_mmin @ 0x9280

void pred_mmin(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined1 auVar1 [16];
  
  auVar1 = FUN_00125c60(param_2);
  FUN_001087d0(auVar1._0_8_,auVar1._8_8_,param_3,0x3c);
  return;
}




// Function: pred_mtime @ 0x92b0

void pred_mtime(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined1 auVar1 [16];
  
  auVar1 = FUN_00125c60(param_2);
  FUN_001087d0(auVar1._0_8_,auVar1._8_8_,param_3,0x15180);
  return;
}




// Function: pred_name @ 0x92e0

void pred_name(undefined8 param_1,undefined8 param_2,long param_3)

{
  FUN_00108940(param_1,*(undefined8 *)(param_3 + 0x38),0);
  return;
}




// Function: pred_negate @ 0x92f0

uint pred_negate(undefined8 param_1,undefined8 param_2,long param_3)

{
  uint uVar1;
  
  uVar1 = FUN_0010e2a0(param_1,param_2,*(undefined8 *)(param_3 + 0x118));
  return uVar1 ^ 1;
}




// Function: pred_newer @ 0x9310

bool pred_newer(undefined8 param_1,undefined8 param_2,long param_3)

{
  int iVar1;
  undefined1 auVar2 [16];
  
  if (*(int *)(param_3 + 0x3c) == 0) {
    auVar2 = FUN_00125c60(param_2);
    iVar1 = FUN_00108790(auVar2._0_8_,auVar2._8_8_,*(undefined8 *)(param_3 + 0x40),
                         *(undefined8 *)(param_3 + 0x48));
    return 0 < iVar1;
  }
                    /* WARNING: Subroutine does not return */
  __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0x26d,"pred_newer");
}




// Function: pred_newerXY @ 0x9370

bool pred_newerXY(undefined8 param_1,undefined8 param_2,long param_3)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined1 auVar4 [16];
  
  if (*(int *)(param_3 + 0x3c) != 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0x277,"pred_newerXY");
  }
  switch(*(undefined4 *)(param_3 + 0x38)) {
  case 0:
    auVar4 = FUN_00125c40();
    break;
  case 1:
    auVar4 = FUN_00125c70();
    if (auVar4._8_8_ < 0) {
      uVar2 = FUN_0010dfb0(0,param_1);
      uVar3 = dcgettext(0,"WARNING: cannot determine birth time of file %s",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar3,uVar2);
    }
    break;
  case 2:
    auVar4 = FUN_00125c50();
    break;
  case 3:
    auVar4 = FUN_00125c60(param_2);
    break;
  case 4:
                    /* WARNING: Subroutine does not return */
    __assert_fail("pred_ptr->args.reftime.xval != XVAL_TIME","pred.c",0x27c,"pred_newerXY");
  default:
                    /* WARNING: Subroutine does not return */
    __assert_fail("collected","pred.c",0x29b,"pred_newerXY");
  }
  iVar1 = FUN_00108790(auVar4._0_8_,auVar4._8_8_,*(undefined8 *)(param_3 + 0x40),
                       *(undefined8 *)(param_3 + 0x48));
  return 0 < iVar1;
}




// Function: pred_nogroup @ 0x94a0

bool pred_nogroup(undefined8 param_1,long param_2)

{
  group *pgVar1;
  
  pgVar1 = getgrgid(*(__gid_t *)(param_2 + 0x20));
  return pgVar1 == (group *)0x0;
}




// Function: pred_nouser @ 0x94c0

bool pred_nouser(undefined8 param_1,long param_2)

{
  passwd *ppVar1;
  
  ppVar1 = getpwuid(*(__uid_t *)(param_2 + 0x1c));
  return ppVar1 == (passwd *)0x0;
}




// Function: pred_ok @ 0x94e0

undefined8 pred_ok(undefined8 param_1,undefined8 param_2,long param_3)

{
  char cVar1;
  undefined8 uVar2;
  
  cVar1 = FUN_00108990(**(undefined8 **)(param_3 + 0xe8),param_1);
  if (cVar1 == '\0') {
    return 0;
  }
  uVar2 = FUN_0010a390(param_1,param_2,param_3);
  return uVar2;
}




// Function: pred_okdir @ 0x9530

undefined8 pred_okdir(undefined8 param_1,undefined8 param_2,long param_3)

{
  char cVar1;
  undefined8 uVar2;
  
  cVar1 = FUN_00108990(**(undefined8 **)(param_3 + 0xe8),param_1);
  if (cVar1 == '\0') {
    return 0;
  }
  uVar2 = FUN_0010a390(DAT_0013e5f0,param_2,param_3);
  return uVar2;
}




// Function: pred_path @ 0x95a0

bool pred_path(char *param_1,undefined8 param_2,long param_3)

{
  int iVar1;
  
  iVar1 = fnmatch(*(char **)(param_3 + 0x38),param_1,0);
  return iVar1 == 0;
}




// Function: pred_perm @ 0x95c0

ulong pred_perm(undefined8 param_1,long param_2,long param_3,undefined8 param_4,
                  undefined8 param_5)

{
  uint uVar1;
  uint uVar2;
  int iVar3;
  undefined7 uVar4;
  
  uVar1 = *(uint *)(param_2 + 0x18);
  uVar2 = *(uint *)(param_3 + 0x3c + (ulong)((uVar1 & 0xf000) == 0x4000) * 4);
  iVar3 = *(int *)(param_3 + 0x38);
  if (iVar3 == 1) {
    if (uVar2 == 0) {
      return 1;
    }
    return (ulong)((uVar1 & uVar2) != 0);
  }
  uVar4 = (undefined7)((ulong)param_5 >> 8);
  if (iVar3 != 2) {
    if (iVar3 != 0) {
                    /* WARNING: Subroutine does not return */
      abort();
    }
    return CONCAT71(uVar4,(uVar1 & uVar2) == uVar2) & 0xffffffff;
  }
  return CONCAT71(uVar4,(uVar1 & 0xfff) == uVar2) & 0xffffffff;
}




// Function: pred_executable @ 0x9640

bool pred_executable(void)

{
  int iVar1;
  
  iVar1 = faccessat(DAT_0013e5f8,DAT_0013e5f0,1,0);
  return iVar1 == 0;
}




// Function: pred_readable @ 0x9670

bool pred_readable(void)

{
  int iVar1;
  
  iVar1 = faccessat(DAT_0013e5f8,DAT_0013e5f0,4,0);
  return iVar1 == 0;
}




// Function: pred_writable @ 0x96a0

bool pred_writable(void)

{
  int iVar1;
  
  iVar1 = faccessat(DAT_0013e5f8,DAT_0013e5f0,2,0);
  return iVar1 == 0;
}




// Function: pred_regex @ 0x96f0

undefined8 pred_regex(char *param_1,undefined8 param_2,long param_3)

{
  int iVar1;
  size_t sVar2;
  undefined4 extraout_var;
  
  sVar2 = strlen(param_1);
  iVar1 = re_match(*(re_pattern_buffer **)(param_3 + 0x38),param_1,(int)sVar2,0,(re_registers *)0x0)
  ;
  return CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),iVar1 == (int)sVar2);
}




// Function: pred_size @ 0x9730

ulong pred_size(undefined8 param_1,long param_2,long param_3)

{
  int iVar1;
  ulong uVar2;
  undefined7 uVar3;
  
  iVar1 = *(int *)(param_3 + 0x38);
  uVar2 = (*(long *)(param_2 + 0x30) / (long)*(int *)(param_3 + 0x3c) + 1) -
          (ulong)(*(long *)(param_2 + 0x30) % (long)*(int *)(param_3 + 0x3c) == 0);
  uVar3 = (undefined7)((ulong)param_2 >> 8);
  if (iVar1 == 1) {
    return CONCAT71(uVar3,uVar2 < *(ulong *)(param_3 + 0x40)) & 0xffffffff;
  }
  if (iVar1 != 2) {
    if (iVar1 != 0) {
      return 0;
    }
    return (ulong)(*(ulong *)(param_3 + 0x40) < uVar2);
  }
  return CONCAT71(uVar3,*(ulong *)(param_3 + 0x40) == uVar2) & 0xffffffff;
}




// Function: pred_samefile @ 0x97a0

undefined8 pred_samefile(undefined8 param_1,long *param_2,long param_3)

{
  undefined8 uVar1;
  
  if (((param_2[1] == 0) || (param_2[1] == *(long *)(param_3 + 0x38))) &&
     (uVar1 = FUN_0010e080(param_1,DAT_0013e5f0,param_2), (int)uVar1 == 0)) {
    if (param_2[1] != *(long *)(param_3 + 0x38)) {
      return uVar1;
    }
    return CONCAT71((int7)((ulong)*(long *)(param_3 + 0x40) >> 8),
                    *param_2 == *(long *)(param_3 + 0x40));
  }
  return 0;
}




// Function: pred_type @ 0x9810

undefined1 pred_type(undefined8 param_1,long param_2,long param_3)

{
  uint uVar1;
  long lVar2;
  undefined1 uVar3;
  
  if (DAT_0013e5e5 == '\0') {
                    /* WARNING: Subroutine does not return */
    __assert_fail("state.have_type","pred.c",0x3d1,"pred_type");
  }
  uVar3 = 0;
  if (DAT_0013e5e8 != 0) {
    uVar1 = DAT_0013e5e8;
    if (DAT_0013e5e4 != '\0') {
      uVar1 = *(uint *)(param_2 + 0x18);
    }
    uVar1 = uVar1 & 0xf000;
    if (uVar1 == 0x6000) {
      lVar2 = 0;
    }
    else if (uVar1 < 0x6001) {
      if (uVar1 == 0x2000) {
        lVar2 = 1;
      }
      else {
        lVar2 = 2;
        if ((uVar1 != 0x4000) && (lVar2 = 5, uVar1 != 0x1000)) {
          return 0;
        }
      }
    }
    else if (uVar1 == 0xa000) {
      lVar2 = 4;
    }
    else {
      lVar2 = 6;
      if (uVar1 != 0xc000) {
        if (uVar1 != 0x8000) {
          return 0;
        }
        lVar2 = 3;
      }
    }
    uVar3 = *(undefined1 *)(param_3 + 0x38 + lVar2);
  }
  return uVar3;
}




// Function: pred_uid @ 0x98f0

ulong pred_uid(undefined8 param_1,long param_2,long param_3,undefined8 param_4,
                  undefined8 param_5)

{
  int iVar1;
  undefined7 uVar2;
  
  iVar1 = *(int *)(param_3 + 0x38);
  uVar2 = (undefined7)((ulong)param_5 >> 8);
  if (iVar1 == 1) {
    return CONCAT71(uVar2,(ulong)*(uint *)(param_2 + 0x1c) < *(ulong *)(param_3 + 0x40)) &
           0xffffffff;
  }
  if (iVar1 != 2) {
    if (iVar1 != 0) {
      return 0;
    }
    return (ulong)(*(ulong *)(param_3 + 0x40) < (ulong)*(uint *)(param_2 + 0x1c));
  }
  return CONCAT71(uVar2,(ulong)*(uint *)(param_2 + 0x1c) == *(ulong *)(param_3 + 0x40)) & 0xffffffff
  ;
}




// Function: pred_used @ 0x9940

undefined8 pred_used(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  int iVar1;
  undefined8 uVar2;
  long lVar3;
  long lVar4;
  undefined1 auVar5 [16];
  undefined1 auVar6 [16];
  
  auVar5 = FUN_00125c40(param_2);
  auVar6 = FUN_00125c50(param_2);
  iVar1 = FUN_00108790(auVar5._0_8_,auVar5._8_8_,auVar6._0_8_,auVar6._8_8_);
  if (-1 < iVar1) {
    lVar4 = auVar6._0_8_ - auVar5._0_8_;
    lVar3 = auVar6._8_8_ - auVar5._8_8_;
    if (lVar3 < 0) {
      lVar3 = lVar3 + 1000000000;
      lVar4 = lVar4 + -1;
    }
    uVar2 = FUN_001087d0(lVar4,lVar3,param_3,0x15180);
    return uVar2;
  }
  return 0;
}




// Function: pred_user @ 0x99d0

undefined4 pred_user(undefined8 param_1,long param_2,long param_3)

{
  return CONCAT31((int3)((uint)*(int *)(param_2 + 0x1c) >> 8),
                  *(int *)(param_3 + 0x38) == *(int *)(param_2 + 0x1c));
}




// Function: pred_xtype @ 0x99e0

void pred_xtype(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  undefined8 uVar4;
  code *pcVar5;
  long in_FS_OFFSET;
  undefined1 auStack_c8 [152];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0010dc30();
  if (cVar1 == '\0') {
    pcVar5 = FUN_0010daa0;
  }
  else {
    pcVar5 = FUN_0010db20;
  }
  FUN_0010d860(auStack_c8);
  iVar2 = (*pcVar5)(DAT_0013e5f0,auStack_c8);
  if (iVar2 == 0) {
    FUN_00109810(param_1,auStack_c8,param_3);
  }
  else {
    cVar1 = FUN_0010dc30();
    piVar3 = __errno_location();
    if ((cVar1 == '\0') || (*piVar3 != 2)) {
      uVar4 = FUN_0010dfb0(0,param_1);
                    /* WARNING: Subroutine does not return */
      error(0,*piVar3,&DAT_0013155a,uVar4);
    }
    FUN_00109810(param_1,param_2,param_3);
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: pred_context @ 0x9af0

ulong pred_context(undefined8 param_1,undefined8 param_2,long param_3)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  int *piVar4;
  undefined8 unaff_R12;
  long in_FS_OFFSET;
  char *local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = (*DAT_0013e680)(DAT_0013e5f8,DAT_0013e5f0,&local_38);
  if (iVar1 < 0) {
    uVar2 = FUN_0010dfb0(0,param_1);
    uVar3 = dcgettext(0,"getfilecon failed: %s",5);
    piVar4 = __errno_location();
                    /* WARNING: Subroutine does not return */
    error(0,*piVar4,uVar3,uVar2);
  }
  iVar1 = fnmatch(*(char **)(param_3 + 0x38),local_38,0);
  freecon(local_38);
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return CONCAT71((int7)((ulong)unaff_R12 >> 8),iVar1 == 0) & 0xffffffff;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_list @ 0x9bc0

void print_list(undefined8 param_1,long param_2)

{
  undefined8 uVar1;
  long in_FS_OFFSET;
  undefined1 auStack_138 [264];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  for (; param_2 != 0; param_2 = *(long *)(param_2 + 0x108)) {
    uVar1 = FUN_00108a40(*(undefined8 *)(param_2 + 8),auStack_138);
    __fprintf_chk(param_1,1,"[%s] ",uVar1);
  }
  __fprintf_chk(param_1,1,"\n");
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_optlist @ 0x9c70

void print_optlist(undefined8 param_1,long param_2)

{
  char *pcVar1;
  char *pcVar2;
  char *pcVar3;
  
  if (param_2 != 0) {
    FUN_00109e10(param_1,*(undefined8 *)(param_2 + 0x110));
    pcVar3 = "[need inum] ";
    if (*(char *)(param_2 + 0x1c) == '\0') {
      pcVar3 = "";
    }
    pcVar2 = "[need type] ";
    if (*(char *)(param_2 + 0x1b) == '\0') {
      pcVar2 = "";
    }
    pcVar1 = "[call stat] ";
    if (*(char *)(param_2 + 0x1a) == '\0') {
      pcVar1 = "";
    }
    __fprintf_chk(param_1,1,&DAT_0012b68e,pcVar1,pcVar2,pcVar3);
    FUN_0010b5a0(param_1,param_2);
    __fprintf_chk((double)*(float *)(param_2 + 0x24),param_1,1," [est success rate %.4g] ");
    if (((byte)DAT_0013e660 & 0x40) != 0) {
      __fprintf_chk(param_1,1,"[real success rate %lu/%lu",*(undefined8 *)(param_2 + 0x128),
                    *(undefined8 *)(param_2 + 0x120));
      if (*(ulong *)(param_2 + 0x120) == 0) {
        __fprintf_chk(param_1,1,&DAT_0012b6d2);
      }
      else {
        __fprintf_chk((double)*(ulong *)(param_2 + 0x128) / (double)*(ulong *)(param_2 + 0x120),
                      param_1,1,"=%.4g] ");
      }
    }
    FUN_00109e10(param_1,*(undefined8 *)(param_2 + 0x118));
    return;
  }
  return;
}




// Function: print_parenthesised @ 0x9e10

void print_parenthesised(undefined8 param_1,long *param_2)

{
  if (param_2 == (long *)0x0) {
    return;
  }
  while( true ) {
    if (((code *)*param_2 != FUN_001088c0) && ((code *)*param_2 != FUN_00108870)) break;
    if (param_2[0x22] != 0) goto LAB_00109e5e;
    param_2 = (long *)param_2[0x23];
    if (param_2 == (long *)0x0) {
      return;
    }
  }
  if ((param_2[0x22] == 0) && (param_2[0x23] == 0)) {
    FUN_00109c70(param_1,param_2);
    return;
  }
LAB_00109e5e:
  __fprintf_chk(param_1,1,&DAT_0013155a,&DAT_0012b6d7);
  FUN_00109c70(param_1,param_2);
  __fprintf_chk(param_1,1,&DAT_0013155a,&DAT_0012b6db);
  return;
}




// Function: show_success_rates @ 0x9ee0

void show_success_rates(undefined8 param_1)

{
  if (((byte)DAT_0013e660 & 0x40) == 0) {
    return;
  }
  __fprintf_chk(stderr,1,"Predicate success rates after completion:\n");
  FUN_00109c70(stderr,param_1);
  __fprintf_chk(stderr,1,"\n");
  return;
}




// Function: pred_sanity_check @ 0x9f40

void pred_sanity_check(long *param_1)

{
  code *pcVar1;
  int *piVar2;
  
  if (param_1 == (long *)0x0) {
    return;
  }
  do {
    pcVar1 = (code *)*param_1;
    if (pcVar1 == (code *)0x0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("p->pred_func != NULL","pred.c",0x51e,"pred_sanity_check");
    }
    piVar2 = (int *)param_1[0x26];
    if (piVar2 == (int *)0x0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("p->parser_entry != NULL","pred.c",0x521,"pred_sanity_check");
    }
    if ((pcVar1 != *(code **)(piVar2 + 6)) && (*(code **)(piVar2 + 6) != (code *)0x0)) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("p->parser_entry->pred_func == p->pred_func","pred.c",0x52a,"pred_sanity_check")
      ;
    }
    switch(*piVar2) {
    case 0:
    case 2:
      if (*piVar2 == 0) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("p->parser_entry->type != ARG_OPTION","pred.c",0x53a,"pred_sanity_check");
      }
                    /* WARNING: Subroutine does not return */
      __assert_fail("p->parser_entry->type != ARG_POSITIONAL_OPTION","pred.c",0x53b,
                    "pred_sanity_check");
    case 1:
    case 3:
    case 4:
    case 5:
      if (*(char *)((long)param_1 + 0x19) != '\0') {
                    /* WARNING: Subroutine does not return */
        __assert_fail("!p->no_default_print","pred.c",0x553,"pred_sanity_check");
      }
      if ((char)param_1[3] != '\0') {
                    /* WARNING: Subroutine does not return */
        __assert_fail("!p->side_effects","pred.c",0x554,"pred_sanity_check");
      }
      break;
    case 6:
      if ((char)param_1[3] == '\0') {
                    /* WARNING: Subroutine does not return */
        __assert_fail("p->side_effects","pred.c",0x53f,"pred_sanity_check");
      }
      if (((pcVar1 != FUN_001086f0) && (pcVar1 != FUN_00108a20)) &&
         (*(char *)((long)param_1 + 0x19) == '\0')) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("p->no_default_print","pred.c",0x545,"pred_sanity_check");
      }
    }
    param_1 = (long *)param_1[0x21];
    if (param_1 == (long *)0x0) {
      return;
    }
  } while( true );
}




// Function: initialize_wd_for_exec @ 0xa120

bool initialize_wd_for_exec(long param_1,undefined4 param_2,undefined8 param_3)

{
  int iVar1;
  undefined4 uVar2;
  undefined4 *puVar3;
  
  puVar3 = (undefined4 *)FUN_00126990(0x10);
  *(undefined4 **)(param_1 + 0xc0) = puVar3;
  *(undefined8 *)(puVar3 + 2) = 0;
  uVar2 = FUN_0011e690(param_2,param_3,0);
  *puVar3 = uVar2;
  iVar1 = **(int **)(param_1 + 0xc0);
  if (-1 < iVar1) {
    FUN_001174f0(iVar1,1);
  }
  return -1 < iVar1;
}




// Function: record_exec_dir @ 0xa190

ulong record_exec_dir(long param_1)

{
  char *pcVar1;
  uint uVar2;
  char *pcVar3;
  void *__ptr;
  ulong uVar4;
  
  pcVar1 = DAT_0013e5f0;
  if (*(int *)(param_1 + 0x98) != 0) {
    return 1;
  }
  pcVar3 = strchr(DAT_0013e5f0,0x2f);
  if (pcVar3 != (char *)0x0) {
    __ptr = (void *)FUN_001177a0(pcVar1);
    uVar2 = FUN_0010a120(param_1,DAT_0013e5f8,__ptr);
    free(__ptr);
    return (ulong)uVar2;
  }
  uVar4 = FUN_0010a120(param_1,DAT_0013e5f8,&DAT_0012f399);
  return uVar4;
}




// Function: prep_child_for_exec @ 0xa220

undefined4 prep_child_for_exec(undefined4 param_1,int *param_2)

{
  int iVar1;
  undefined8 uVar2;
  int *piVar3;
  undefined4 uVar4;
  char *pcVar5;
  char *pcVar6;
  long in_FS_OFFSET;
  undefined8 local_3a;
  undefined2 local_32;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar4 = 1;
  if ((char)param_1 != '\0') {
    local_3a = 0x6c756e2f7665642f;
    local_32 = 0x6c;
    iVar1 = close(0);
    if (iVar1 < 0) {
      uVar2 = dcgettext(0,"Cannot close standard input",5);
      piVar3 = __errno_location();
                    /* WARNING: Subroutine does not return */
      error(0,*piVar3,uVar2);
    }
    iVar1 = FUN_00117b20(&local_3a,0);
    uVar4 = param_1;
    if (iVar1 < 0) {
      uVar2 = FUN_0010dfb0(0,&local_3a);
      piVar3 = __errno_location();
                    /* WARNING: Subroutine does not return */
      error(0,*piVar3,&DAT_0013155a,uVar2);
    }
  }
  iVar1 = FUN_00125250(param_2);
  if (iVar1 != 0) {
    pcVar5 = "";
    pcVar6 = "";
    if (*param_2 < 0) {
      pcVar5 = *(char **)(param_2 + 2);
      pcVar6 = ": ";
      if (pcVar5 == (char *)0x0) {
        pcVar5 = "";
        pcVar6 = pcVar5;
      }
    }
    uVar2 = dcgettext(0,"Failed to change directory%s%s",5);
    piVar3 = __errno_location();
                    /* WARNING: Subroutine does not return */
    error(0,*piVar3,uVar2,pcVar6,pcVar5);
  }
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar4;
}




// Function: impl_pred_exec @ 0xa390

ulong impl_pred_exec(char *param_1,undefined8 param_2,undefined8 *param_3)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  char *__s;
  byte bVar3;
  char cVar4;
  size_t sVar5;
  size_t sVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  int *piVar9;
  long lVar10;
  ulong uVar11;
  undefined *local_50;
  byte local_41;
  char *local_40;
  
  bVar3 = FUN_0010df90(*param_3);
  if (bVar3 == 0) {
    if (param_3[0x1f] != DAT_0013e5c0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("execp->wd_for_exec == initial_wd","exec.c",0x94,"impl_pred_exec");
    }
    local_50 = (undefined *)0x0;
    cVar4 = '\0';
    local_40 = (char *)0x0;
  }
  else {
    cVar4 = FUN_0010a190(param_3 + 7);
    if (cVar4 == '\0') {
      uVar7 = FUN_0010dfb0(0,param_1);
      uVar8 = dcgettext(0,"Failed to save working directory in order to run a command on %s",5);
      piVar9 = __errno_location();
                    /* WARNING: Subroutine does not return */
      error(1,*piVar9,uVar8,uVar7);
    }
    param_1 = (char *)FUN_001176f0(DAT_0013e5f0);
    cVar4 = (*param_1 != '/') * '\x02';
    local_50 = (undefined *)0x0;
    local_40 = param_1;
    if (*param_1 != '/') {
      local_50 = &DAT_0012b98c;
    }
  }
  uVar11 = (ulong)*(byte *)(param_3 + 7);
  puVar2 = param_3 + 0x13;
  puVar1 = param_3 + 8;
  if (*(byte *)(param_3 + 7) == 0) {
    lVar10 = 0;
    if (0 < *(int *)(param_3 + 0x1e)) {
      do {
        sVar5 = strlen(param_1);
        __s = *(char **)(param_3[0x1d] + lVar10 * 8);
        lVar10 = lVar10 + 1;
        sVar6 = strlen(__s);
        FUN_00115290(puVar1,puVar2,__s,sVar6,local_50,cVar4,param_1,sVar5,0);
      } while ((int)lVar10 < *(int *)(param_3 + 0x1e));
    }
    FUN_00114e90(puVar1,puVar2);
    local_41 = (*(uint *)(param_3 + 0x20) >> 8 & 0xff) == 0 &&
               (*(uint *)(param_3 + 0x20) & 0x7f) == 0;
    uVar11 = CONCAT71((int7)((ulong)lVar10 >> 8),local_41);
    if (bVar3 == 0) {
      if (local_40 != (char *)0x0) {
LAB_0010a52b:
                    /* WARNING: Subroutine does not return */
        __assert_fail("local","exec.c",0xd0,"impl_pred_exec");
      }
      goto LAB_0010a49d;
    }
    FUN_00125280(param_3[0x1f]);
    if (local_40 == (char *)0x0) goto LAB_0010a49d;
  }
  else {
    sVar5 = strlen(param_1);
    FUN_00115000(puVar1,puVar2,param_1,sVar5 + 1,local_50,cVar4,0);
    if (*(int *)(param_3 + 0x1a) != 0) {
      DAT_0013e608 = 1;
    }
    if (local_40 == (char *)0x0) goto LAB_0010a49d;
    local_41 = bVar3;
    if (bVar3 == 0) goto LAB_0010a52b;
  }
  free(local_40);
  uVar11 = (ulong)local_41;
LAB_0010a49d:
  return uVar11 & 0xffffffff;
}




// Function: launch @ 0xa610

undefined8 launch(undefined8 param_1,char *param_2,undefined8 param_3,char **param_4)

{
  long lVar1;
  char cVar2;
  __pid_t __pid;
  __pid_t _Var3;
  int *piVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  uint uVar7;
  ulong uVar8;
  
  if (((byte)DAT_0013e660 & 0x20) != 0) {
    uVar8 = 0;
    __fprintf_chk(stderr,1,"DebugExec: launching process (argc=%lu):",*(long *)(param_2 + 0x60) + -1
                 );
    if (*(long *)(param_2 + 0x60) != 1) {
      do {
        lVar1 = uVar8 * 8;
        uVar8 = uVar8 + 1;
        uVar5 = FUN_0010dfb0(0,*(undefined8 *)(*(long *)(param_2 + 0x68) + lVar1));
        __fprintf_chk(stderr,1,&DAT_00131559,uVar5);
      } while (uVar8 < *(long *)(param_2 + 0x60) - 1U);
    }
    __fprintf_chk(stderr,1,"\n");
  }
  FUN_00117bc0(stdout);
  FUN_00117bc0(stderr);
  if (DAT_0013e01c != 0) {
    DAT_0013e01c = 0;
    signal(0x11,(__sighandler_t)0x0);
  }
  __pid = fork();
  if (__pid == -1) {
    uVar5 = dcgettext(0,"cannot fork",5);
    piVar4 = __errno_location();
                    /* WARNING: Subroutine does not return */
    error(1,*piVar4,uVar5);
  }
  if (__pid != 0) {
    do {
      _Var3 = waitpid(__pid,(int *)(param_2 + 200),0);
      if (_Var3 != -1) {
        uVar7 = *(uint *)(param_2 + 200);
        if ('\x01' < (char)(((byte)uVar7 & 0x7f) + 1)) {
          uVar5 = FUN_00124e50(0,DAT_0013e68c,*param_4);
          uVar6 = dcgettext(0,"%s terminated by signal %d",5);
                    /* WARNING: Subroutine does not return */
          error(0,0,uVar6,uVar5,uVar7 & 0x7f);
        }
        uVar7 = uVar7 >> 8 & 0xff;
        if (((byte)DAT_0013e660 & 0x20) != 0) {
          __fprintf_chk(stderr,1,"DebugExec: process (PID=%ld) terminated with exit status: %d\n",
                        (long)__pid,uVar7);
        }
        if ((uVar7 != 0) && (*param_2 != '\0')) {
          DAT_0013e604 = 1;
        }
        return 1;
      }
      piVar4 = __errno_location();
    } while (*piVar4 == 4);
    uVar5 = FUN_0010dfb0(0,*param_4);
    uVar6 = dcgettext(0,"error waiting for %s",5);
                    /* WARNING: Subroutine does not return */
    error(0,*piVar4,uVar6,uVar5);
  }
  if (*(long *)(param_2 + 0xc0) == 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("NULL != execp->wd_for_exec","exec.c",0x141,"launch");
  }
  cVar2 = FUN_0010a220(param_2[0xbc]);
  if (cVar2 == '\0') {
                    /* WARNING: Subroutine does not return */
    _exit(1);
  }
  cVar2 = FUN_00115dd0();
  if (cVar2 != '\0') {
    FUN_00115df0();
  }
  cVar2 = FUN_00115470(param_4);
  piVar4 = __errno_location();
  if (cVar2 == '\0') {
    execvp(*param_4,param_4);
  }
  else {
    *piVar4 = 7;
  }
  uVar5 = FUN_0010dfb0(0,*param_4);
                    /* WARNING: Subroutine does not return */
  error(0,*piVar4,&DAT_0013155a,uVar5);
}




// Function: predicate_is_cost_free @ 0xa950

undefined4 predicate_is_cost_free(long *param_1,undefined8 param_2)

{
  undefined *puVar1;
  byte bVar2;
  uint uVar3;
  undefined8 in_RAX;
  undefined7 uVar4;
  
  puVar1 = (undefined *)*param_1;
  uVar4 = (undefined7)((ulong)param_2 >> 8);
  uVar3 = (uint)CONCAT71((int7)((ulong)in_RAX >> 8),puVar1 == PTR_FUN_0013df58) |
          (uint)CONCAT71(uVar4,puVar1 == PTR_FUN_0013dfe0) |
          (uint)CONCAT71(uVar4,puVar1 == PTR_FUN_0013df30);
  bVar2 = (byte)uVar3 | puVar1 == PTR_FUN_0013df80;
  if (((bVar2 == 0) &&
      (((puVar1 != PTR_FUN_0013df50 && puVar1 != PTR_FUN_0013df48) && DAT_0013e688 != 0) &&
       puVar1 != PTR_FUN_0013df68)) && (puVar1 != PTR_FUN_0013dff8)) {
    return CONCAT31((int3)((uint)(int)param_1[4] >> 8),(int)param_1[4] == 0);
  }
  return CONCAT31((int3)(uVar3 >> 8),bVar2);
}




// Function: predlist_init @ 0xa9e0

void predlist_init(undefined8 *param_1)

{
  param_1[1] = 0;
  *param_1 = 0;
  return;
}




// Function: predlist_insert @ 0xa9f0

void predlist_insert(long *param_1,long param_2,undefined8 *param_3)

{
  long lVar1;
  
  *param_3 = *(undefined8 *)(param_2 + 0x110);
  lVar1 = param_1[1];
  *(long *)(param_2 + 0x110) = *param_1;
  *param_1 = param_2;
  if (lVar1 != 0) {
    return;
  }
  param_1[1] = param_2;
  return;
}




// Function: pred_cost_compare @ 0xaa20

uint pred_cost_compare(long param_1,long param_2,char param_3)

{
  float fVar1;
  float fVar2;
  uint uVar3;
  
  if (*(uint *)(param_1 + 0x20) == *(uint *)(param_2 + 0x20)) {
    fVar1 = *(float *)(param_1 + 0x24);
    fVar2 = *(float *)(param_2 + 0x24);
    uVar3 = 0;
    if (fVar1 != fVar2) {
      if (param_3 == '\0') {
        return ((fVar1 < fVar2) - 1) + (uint)(fVar1 < fVar2);
      }
      uVar3 = 0xffffffff;
      if (fVar2 <= fVar1) {
        uVar3 = 1;
      }
      return uVar3;
    }
  }
  else {
    uVar3 = -(uint)(*(uint *)(param_1 + 0x20) < *(uint *)(param_2 + 0x20)) | 1;
  }
  return uVar3;
}




// Function: subtree_has_side_effects @ 0xaa70

ulong subtree_has_side_effects(long param_1)

{
  ulong uVar1;
  
  while( true ) {
    if (param_1 == 0) {
      return 0;
    }
    if (*(byte *)(param_1 + 0x18) != 0) break;
    uVar1 = subtree_has_side_effects(*(undefined8 *)(param_1 + 0x110));
    if ((char)uVar1 != '\0') {
      return uVar1;
    }
    param_1 = *(long *)(param_1 + 0x118);
  }
  return (ulong)*(byte *)(param_1 + 0x18);
}




// Function: worst_cost @ 0xaab0

uint worst_cost(long param_1)

{
  uint uVar1;
  uint uVar2;
  
  if (param_1 != 0) {
    uVar1 = worst_cost(*(undefined8 *)(param_1 + 0x110));
    uVar2 = worst_cost(*(undefined8 *)(param_1 + 0x118));
    if (uVar2 <= uVar1) {
      uVar2 = uVar1;
    }
    if (uVar2 < *(uint *)(param_1 + 0x20)) {
      uVar2 = *(uint *)(param_1 + 0x20);
    }
    return uVar2;
  }
  return 0;
}




// Function: perform_arm_swap @ 0xab00

void perform_arm_swap(long param_1)

{
  undefined8 uVar1;
  
  uVar1 = *(undefined8 *)(*(long *)(param_1 + 0x110) + 0x118);
  *(undefined8 *)(*(long *)(param_1 + 0x110) + 0x118) = *(undefined8 *)(param_1 + 0x118);
  *(undefined8 *)(param_1 + 0x118) = uVar1;
  return;
}




// Function: constrain_rate @ 0xab30

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void constrain_rate(float param_1)

{
  if (param_1 <= _DAT_0012c588) {
    return;
  }
  return;
}




// Function: merge_pred @ 0xab60

void merge_pred(undefined8 param_1,long param_2,undefined8 *param_3)

{
  *(undefined8 *)(param_2 + 0x110) = *param_3;
  *param_3 = param_1;
  return;
}




// Function: getrate @ 0xab70

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 getrate(long param_1)

{
  undefined4 uVar1;
  
  uVar1 = _DAT_0012c588;
  if (param_1 != 0) {
    uVar1 = *(undefined4 *)(param_1 + 0x24);
  }
  return uVar1;
}




// Function: cost_name @ 0xab90

char * cost_name(int param_1)

{
  uint uVar1;
  ulong uVar2;
  int *piVar3;
  
  uVar2 = 0;
  piVar3 = &DAT_0013e020;
  do {
    if (*piVar3 == param_1) {
      return *(char **)(uVar2 * 0x10 + 0x13e028);
    }
    uVar1 = (int)uVar2 + 1;
    uVar2 = (ulong)uVar1;
    piVar3 = piVar3 + 4;
  } while (uVar1 != 0xb);
  return "unknown";
}




// Function: type_name @ 0xabd0

undefined * type_name(short param_1)

{
  undefined *puVar1;
  int iVar2;
  short sVar3;
  
  if (param_1 != 0) {
    sVar3 = 1;
    iVar2 = 0;
    puVar1 = &DAT_0013c040;
    while( true ) {
      iVar2 = iVar2 + 1;
      if ((param_1 == sVar3) || (sVar3 == -1)) break;
      sVar3 = *(short *)(puVar1 + 0x20);
      puVar1 = puVar1 + 0x10;
    }
    return (&PTR_DAT_0013c048)[(long)iVar2 * 2];
  }
  return &DAT_0012bb17;
}




// Function: prec_name @ 0xac30

undefined * prec_name(short param_1)

{
  undefined *puVar1;
  int iVar2;
  short sVar3;
  
  if (param_1 != 0) {
    sVar3 = 1;
    iVar2 = 0;
    puVar1 = &DAT_0013c0c0;
    while( true ) {
      iVar2 = iVar2 + 1;
      if ((param_1 == sVar3) || (sVar3 == -1)) break;
      sVar3 = *(short *)(puVar1 + 0x20);
      puVar1 = puVar1 + 0x10;
    }
    return (&PTR_DAT_0013c0c8)[(long)iVar2 * 2];
  }
  return &DAT_0012bb17;
}




// Function: check_normalization @ 0xac90

void check_normalization(long param_1,char param_2)

{
  while( true ) {
    if ((param_2 != '\0') && (*(int *)(param_1 + 0x10) != 3)) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("BI_OP == p->p_type","tree.c",0x4ce,"check_normalization");
    }
    if (*(long *)(param_1 + 0x110) != 0) {
      if (*(int *)(*(long *)(param_1 + 0x110) + 0x10) != 3) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("BI_OP == p->pred_left->p_type","tree.c",0x4d3,"check_normalization");
      }
      check_normalization();
    }
    param_1 = *(long *)(param_1 + 0x118);
    if (param_1 == 0) break;
    param_2 = '\0';
  }
  return;
}




// Function: scan_rest @ 0xad10

long scan_rest(long *param_1,long param_2,short param_3)

{
  short sVar1;
  uint uVar2;
  undefined8 uVar3;
  long lVar4;
  
  lVar4 = *param_1;
  if ((lVar4 == 0) || (*(int *)(lVar4 + 0x10) == 5)) {
    return 0;
  }
  while( true ) {
    if (*(int *)(lVar4 + 0x14) <= (int)param_3) {
      return param_2;
    }
    uVar2 = *(uint *)(lVar4 + 0x10);
    if (uVar2 == 4) break;
    if (4 < uVar2) {
      if (uVar2 == 5) {
        return param_2;
      }
      uVar3 = dcgettext(0,"oops -- invalid expression type (%d)!",5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar3,uVar2);
    }
    if (uVar2 != 3) break;
    *(long *)(lVar4 + 0x110) = param_2;
    param_2 = *param_1;
    sVar1 = *(short *)(param_2 + 0x14);
    *param_1 = *(long *)(param_2 + 0x108);
    uVar3 = FUN_0010ae10(param_1,(int)sVar1);
    *(undefined8 *)(param_2 + 0x118) = uVar3;
    lVar4 = *param_1;
    if (lVar4 == 0) {
      return param_2;
    }
  }
  uVar3 = dcgettext(0,"invalid expression",5);
                    /* WARNING: Subroutine does not return */
  error(1,0,uVar3);
}




// Function: get_expr @ 0xae10

long get_expr(long *param_1,short param_2,long param_3)

{
  int iVar1;
  long lVar2;
  long lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  
  lVar3 = *param_1;
  if (lVar3 == 0) {
    uVar5 = dcgettext(0,"invalid expression",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar5);
  }
  switch(*(undefined4 *)(lVar3 + 0x10)) {
  case 0:
    uVar5 = dcgettext(0,"invalid expression",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar5);
  case 1:
    lVar2 = *(long *)(lVar3 + 0x108);
    *param_1 = lVar2;
    break;
  case 2:
    *param_1 = *(long *)(lVar3 + 0x108);
    uVar5 = get_expr(param_1,4,lVar3);
    *(undefined8 *)(lVar3 + 0x118) = uVar5;
    lVar2 = *param_1;
    break;
  case 3:
    uVar5 = *(undefined8 *)(lVar3 + 8);
    uVar4 = dcgettext(0,
                      "invalid expression; you have used a binary operator \'%s\' with nothing before it."
                      ,5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar4,uVar5);
  case 4:
    lVar2 = *(long *)(lVar3 + 0x108);
    if ((lVar2 == 0) || (*(char *)(lVar2 + 0x29) != '\0')) {
      uVar5 = *(undefined8 *)(lVar3 + 8);
      uVar4 = dcgettext(0,
                        "invalid expression; expected to find a \')\' but didn\'t see one. Perhaps you need an extra predicate after \'%s\'"
                        ,5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar4,uVar5);
    }
    iVar1 = *(int *)(lVar2 + 0x10);
    *param_1 = lVar2;
    if (iVar1 == 5) {
      if (*(char *)(lVar3 + 0x29) == '\0') {
        uVar5 = dcgettext(0,"invalid expression; empty parentheses are not allowed.",5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar5);
      }
      uVar5 = *(undefined8 *)(lVar2 + 8);
      uVar4 = dcgettext(0,
                        "invalid expression: expected expression before closing parentheses \'%s\'."
                        ,5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar4,uVar5);
    }
    lVar3 = get_expr(param_1,0,lVar3);
    lVar2 = *param_1;
    if ((lVar2 == 0) || (*(int *)(lVar2 + 0x10) != 5)) {
      uVar5 = dcgettext(0,
                        "invalid expression; I was expecting to find a \')\' somewhere but did not see one."
                        ,5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar5);
    }
    lVar2 = *(long *)(lVar2 + 0x108);
    *param_1 = lVar2;
    break;
  case 5:
    if (param_3 == 0) {
      uVar5 = *(undefined8 *)(lVar3 + 8);
      uVar4 = dcgettext(0,
                        "invalid expression: expected expression before closing parentheses \'%s\'."
                        ,5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar4,uVar5);
    }
    if (*(int *)(param_3 + 0x10) - 2U < 2) {
      if (*(char *)(lVar3 + 0x29) == '\0') {
        uVar5 = *(undefined8 *)(param_3 + 8);
        uVar4 = dcgettext(0,"expected an expression between \'%s\' and \')\'",5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar4,uVar5);
      }
    }
    else if (*(char *)(lVar3 + 0x29) == '\0') {
      uVar5 = dcgettext(0,"invalid expression; you have too many \')\'",5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar5);
    }
    uVar5 = *(undefined8 *)(param_3 + 8);
    uVar4 = dcgettext(0,"expected an expression after \'%s\'",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar4,uVar5);
  default:
    uVar5 = dcgettext(0,"oops -- invalid expression type!",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar5);
  }
  if (((lVar2 != 0) && ((int)param_2 < *(int *)(lVar2 + 0x14))) &&
     (lVar3 = FUN_0010ad10(param_1,lVar3), lVar3 == 0)) {
    uVar5 = dcgettext(0,"invalid expression",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar5);
  }
  return lVar3;
}




// Function: cost_table_comparison @ 0xb160

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
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: set_new_parent @ 0xb1c0

void set_new_parent(long param_1,int param_2,long *param_3)

{
  undefined4 uVar1;
  undefined8 *puVar2;
  long lVar3;
  undefined4 extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  
  puVar2 = (undefined8 *)FUN_00126c50(0x138);
  *(undefined4 *)(puVar2 + 2) = 3;
  *(int *)((long)puVar2 + 0x14) = param_2;
  *(undefined4 *)(puVar2 + 4) = 0;
  if (param_2 == 2) {
    uVar1 = *(undefined4 *)(param_1 + 0x24);
    puVar2[1] = &DAT_0012bb67;
    *puVar2 = FUN_001088c0;
    lVar3 = FUN_0010ab30(uVar1);
    *(long *)(lVar3 + 0x118) = param_1;
    *(undefined4 *)(lVar3 + 0x24) = extraout_XMM0_Da;
    *param_3 = lVar3;
    return;
  }
  if (param_2 != 3) {
    if (param_2 != 1) {
      puVar2[0x23] = param_1;
      *param_3 = (long)puVar2;
      return;
    }
    *param_3 = (long)puVar2;
    puVar2[1] = &DAT_0012bb65;
    *puVar2 = FUN_00108cd0;
    *(undefined4 *)((long)puVar2 + 0x24) = 0x3f800000;
    puVar2[0x23] = param_1;
    return;
  }
  uVar1 = *(undefined4 *)(param_1 + 0x24);
  puVar2[1] = &DAT_0012bb6a;
  *puVar2 = FUN_00108870;
  lVar3 = FUN_0010ab30(uVar1);
  *(long *)(lVar3 + 0x118) = param_1;
  *(undefined4 *)(lVar3 + 0x24) = extraout_XMM0_Da_00;
  *param_3 = lVar3;
  return;
}




// Function: get_pred_cost @ 0xb350

uint get_pred_cost(long *param_1)

{
  long lVar1;
  char cVar2;
  uint uVar3;
  undefined *puVar4;
  void *pvVar5;
  undefined8 uVar6;
  uint uVar7;
  long in_FS_OFFSET;
  undefined *local_38 [3];
  long local_20;
  
  uVar7 = 3;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if ((*(char *)((long)param_1 + 0x1a) == '\0') &&
     (uVar7 = 1, *(char *)((long)param_1 + 0x1c) == '\0')) {
    uVar7 = (uint)*(byte *)((long)param_1 + 0x1b) * 2;
  }
  puVar4 = (undefined *)*param_1;
  if ((puVar4 == PTR_thunk_FUN_0010a390_0013dfd8) || (puVar4 == PTR_FUN_0013dfc8)) {
    uVar3 = 8 - *(byte *)(param_1 + 7);
  }
  else {
    if (puVar4 == PTR_FUN_0013dfb0) {
      uVar3 = *(uint *)(param_1 + 4);
    }
    else {
      if (DAT_0013e6b0 == 0) {
        qsort(&PTR_FUN_0013e0e0,0x3b,0x10,FUN_0010b160);
        cVar2 = FUN_0010b2d0();
        if (cVar2 == '\0') {
                    /* WARNING: Subroutine does not return */
          error(1,0,"failed to sort the costlookup array");
        }
        DAT_0013e6b0 = 1;
        puVar4 = (undefined *)*param_1;
      }
      local_38[0] = puVar4;
      pvVar5 = bsearch(local_38,&PTR_FUN_0013e0e0,0x3b,0x10,FUN_0010b160);
      if (pvVar5 == (void *)0x0) {
        lVar1 = param_1[1];
        uVar6 = dcgettext(0,
                          "warning: there is no entry in the predicate evaluation cost table for predicate %s; please report this as a bug"
                          ,5);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar6,lVar1);
      }
      uVar3 = *(uint *)((long)pvVar5 + 8);
    }
    if (uVar3 <= uVar7) {
      uVar3 = uVar7;
    }
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar3;
}




// Function: estimate_costs @ 0xb4d0

void estimate_costs(long param_1)

{
  undefined4 uVar1;
  
  if (param_1 != 0) {
    estimate_costs(*(undefined8 *)(param_1 + 0x118));
    estimate_costs(*(undefined8 *)(param_1 + 0x110));
    uVar1 = FUN_0010b350(param_1);
    *(undefined4 *)(param_1 + 0x20) = uVar1;
    return;
  }
  return;
}




// Function: matches_start_point @ 0xb510

undefined8 matches_start_point(char *param_1,byte param_2)

{
  int iVar1;
  undefined4 extraout_var;
  ulong uVar2;
  
  if (DAT_0013e6b8 == 0) {
    iVar1 = fnmatch(param_1,".",(uint)param_2 << 4);
    return CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),iVar1 == 0);
  }
  uVar2 = 0;
  do {
    iVar1 = fnmatch(param_1,*(char **)(DAT_0013e6c0 + uVar2 * 8),(uint)param_2 << 4);
    if (iVar1 == 0) {
      return 1;
    }
    uVar2 = uVar2 + 1;
  } while (uVar2 < DAT_0013e6b8);
  return 0;
}




// Function: print_predicate @ 0xb5a0

void print_predicate(undefined8 param_1,long param_2)

{
  if (*(long *)(param_2 + 0x30) != 0) {
    __fprintf_chk(param_1,1,"%s %s",*(undefined8 *)(param_2 + 8));
    return;
  }
  __fprintf_chk(param_1,1,&DAT_0013155a,*(undefined8 *)(param_2 + 8));
  return;
}




// Function: get_eval_tree @ 0xb5e0

undefined8 get_eval_tree(void)

{
  return DAT_0013e6d0;
}




// Function: calculate_derived_rates @ 0xb5f0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulong calculate_derived_rates(long *param_1)

{
  uint uVar1;
  long lVar2;
  long lVar3;
  undefined *puVar4;
  float fVar5;
  float fVar6;
  ulong uVar7;
  
  if (param_1 == (long *)0x0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("NULL != p","tree.c",0x484,"calculate_derived_rates");
  }
  lVar2 = param_1[0x23];
  if (lVar2 != 0) {
    calculate_derived_rates(lVar2);
  }
  lVar3 = param_1[0x22];
  if (lVar3 != 0) {
    calculate_derived_rates(lVar3);
  }
  uVar1 = *(uint *)(param_1 + 2);
  if (uVar1 == 5) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("p->p_type != CLOSE_PAREN","tree.c",0x48b,"calculate_derived_rates");
  }
  if (uVar1 == 4) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("p->p_type != OPEN_PAREN","tree.c",0x48c,"calculate_derived_rates");
  }
  if (uVar1 == 2) {
    if ((code *)*param_1 != FUN_001092f0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("pred_is (p, pred_negate)","tree.c",0x49c,"calculate_derived_rates");
    }
    if (lVar3 != 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("NULL == p->pred_left","tree.c",0x49d,"calculate_derived_rates");
    }
    fVar5 = _DAT_0012c588 - *(float *)(lVar2 + 0x24);
    *(float *)((long)param_1 + 0x24) = fVar5;
    return (ulong)(uint)fVar5;
  }
  if (2 < uVar1) {
    if (uVar1 != 3) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("0","tree.c",0x4c0,"calculate_derived_rates");
    }
    puVar4 = (undefined *)*param_1;
    if (puVar4 == PTR_FUN_0013df50) {
      fVar5 = (float)FUN_0010ab70(lVar2);
      fVar6 = (float)FUN_0010ab70(fVar5,fVar5,lVar3);
      fVar5 = fVar6 * fVar5;
    }
    else {
      fVar5 = _DAT_0012c588;
      if (puVar4 != PTR_FUN_0013df68) {
        if (puVar4 != PTR_FUN_0013dff8) {
                    /* WARNING: Subroutine does not return */
          __assert_fail("0","tree.c",0x4b4,"calculate_derived_rates");
        }
        fVar5 = (float)FUN_0010ab70(lVar2);
        fVar6 = (float)FUN_0010ab70(fVar5,fVar5,lVar3);
        fVar5 = fVar6 + fVar5;
      }
    }
    uVar7 = FUN_0010ab30(fVar5);
    *(int *)((long)param_1 + 0x24) = (int)uVar7;
    return uVar7;
  }
  if (uVar1 == 0) {
    if (lVar2 != 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("NULL == p->pred_right","tree.c",0x491,"calculate_derived_rates");
    }
    if (lVar3 != 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("NULL == p->pred_left","tree.c",0x492,"calculate_derived_rates");
    }
  }
  else {
    if (lVar2 != 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("NULL == p->pred_right","tree.c",0x496,"calculate_derived_rates");
    }
    if (lVar3 != 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("NULL == p->pred_left","tree.c",0x497,"calculate_derived_rates");
    }
  }
  return (ulong)*(uint *)((long)param_1 + 0x24);
}




// Function: get_new_pred @ 0xb8a0

void get_new_pred(int *param_1)

{
  undefined1 uVar1;
  long lVar2;
  long lVar3;
  
  if (*param_1 == 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("entry->type != ARG_OPTION","tree.c",0x5cd,"get_new_pred");
  }
  if (*param_1 == 2) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("entry->type != ARG_POSITIONAL_OPTION","tree.c",0x5ce,"get_new_pred");
  }
  lVar3 = FUN_00126c50(0x138);
  lVar2 = lVar3;
  if (DAT_0013e6d8 != 0) {
    *(long *)(DAT_0013e6c8 + 0x108) = lVar3;
    lVar2 = DAT_0013e6d8;
  }
  DAT_0013e6d8 = lVar2;
  *(undefined2 *)(lVar3 + 0x1a) = 0x101;
  uVar1 = DAT_0013e62f;
  *(int **)(lVar3 + 0x130) = param_1;
  *(undefined8 *)(lVar3 + 0x10) = 0;
  *(char **)(lVar3 + 0x30) = "ThisShouldBeSetToSomethingElse";
  *(undefined1 *)(lVar3 + 0x28) = uVar1;
  *(undefined8 *)(lVar3 + 0x20) = 0x3f8000000000000a;
  DAT_0013e6c8 = lVar3;
  FUN_0010b2b0(lVar3);
  return;
}




// Function: get_new_pred_noarg @ 0xb970

void get_new_pred_noarg(void)

{
  long lVar1;
  
  lVar1 = FUN_0010b8a0();
  if (lVar1 != 0) {
    *(undefined8 *)(lVar1 + 0x30) = 0;
  }
  return;
}




// Function: get_new_pred_chk_op @ 0xb990

void get_new_pred_chk_op(undefined8 param_1,undefined8 param_2)

{
  long lVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  
  if (DAT_0013e6a0 == 0) {
    DAT_0013e6a0 = FUN_001147f0(&DAT_0012bc18);
    if (DAT_0013e6a0 == 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("entry_and != NULL","tree.c",0x5f8,"get_new_pred_chk_op");
    }
  }
  if (DAT_0013e6c8 != 0) {
    if ((*(uint *)(DAT_0013e6c8 + 0x10) & 0xfffffffb) == 1) {
      puVar2 = (undefined8 *)FUN_0010b970();
      puVar2[1] = &DAT_0012bb6a;
      *puVar2 = FUN_00108870;
      puVar2[6] = 0;
      puVar2[7] = 0;
      puVar2[2] = 0x300000003;
      *(undefined4 *)(puVar2 + 3) = 0;
      *(undefined1 *)((long)puVar2 + 0x1c) = 0;
    }
    else if (*(uint *)(DAT_0013e6c8 + 0x10) == 0) {
      uVar3 = dcgettext(0,"oops -- invalid default insertion of and!",5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar3);
    }
  }
  lVar1 = FUN_0010b8a0(param_1);
  *(undefined8 *)(lVar1 + 0x30) = param_2;
  *(undefined8 *)(lVar1 + 0x130) = param_1;
  return;
}




// Function: print_tree @ 0xbaa0

void print_tree(undefined8 param_1,long param_2,int param_3)

{
  char cVar1;
  undefined8 uVar2;
  char *pcVar3;
  int iVar4;
  
  if (param_2 == 0) {
    return;
  }
  do {
    if (0 < param_3) {
      iVar4 = 0;
      do {
        __fprintf_chk(param_1,1,&DAT_0012bc32);
        iVar4 = iVar4 + 1;
      } while (param_3 != iVar4);
    }
    __fprintf_chk(param_1,1,"pred=[");
    FUN_0010b5a0(param_1,param_2);
    FUN_0010ac30((int)*(short *)(param_2 + 0x14));
    uVar2 = FUN_0010abd0((int)*(short *)(param_2 + 0x10));
    __fprintf_chk(param_1,1,"] type=%s prec=%s",uVar2);
    FUN_0010ab90(0,*(undefined4 *)(param_2 + 0x20),1);
    __fprintf_chk((double)*(float *)(param_2 + 0x24),param_1);
    if ((*(ulong *)(param_2 + 0x18) & 0xffffff0000) != 0) {
      __fprintf_chk(param_1,1,"Needs ");
      if (*(char *)(param_2 + 0x1a) == '\0') {
        if (*(char *)(param_2 + 0x1c) != '\0') {
          pcVar3 = "";
LAB_0010bcc1:
          __fprintf_chk(param_1,1,"%sinode",pcVar3);
          cVar1 = *(char *)(param_2 + 0x1b);
          goto joined_r0x0010bd80;
        }
        if (*(char *)(param_2 + 0x1b) == '\0') goto LAB_0010bba8;
        pcVar3 = "";
      }
      else {
        __fprintf_chk(param_1,1,"stat");
        if (*(char *)(param_2 + 0x1c) != '\0') {
          pcVar3 = ",";
          goto LAB_0010bcc1;
        }
        cVar1 = *(char *)(param_2 + 0x1b);
joined_r0x0010bd80:
        if (cVar1 == '\0') goto LAB_0010bba8;
        pcVar3 = ",";
      }
      __fprintf_chk(param_1,1,"%stype",pcVar3);
    }
LAB_0010bba8:
    __fprintf_chk(param_1,1,"\n");
    if (0 < param_3) {
      iVar4 = 0;
      do {
        __fprintf_chk(param_1,1,&DAT_0012bc32);
        iVar4 = iVar4 + 1;
      } while (param_3 != iVar4);
    }
    if (*(long *)(param_2 + 0x110) == 0) {
      if (*(long *)(param_2 + 0x118) == 0) {
        pcVar3 = "no children.\n";
        goto LAB_0010bd92;
      }
      __fprintf_chk(param_1,1,"no left.\n");
    }
    else {
      __fprintf_chk(param_1,1,"left:\n");
      print_tree(param_1,*(undefined8 *)(param_2 + 0x110),param_3 + 1);
    }
    iVar4 = 0;
    if (0 < param_3) {
      do {
        __fprintf_chk(param_1,1,&DAT_0012bc32);
        iVar4 = iVar4 + 1;
      } while (param_3 != iVar4);
    }
    if (*(long *)(param_2 + 0x118) == 0) {
      pcVar3 = "no right.\n";
LAB_0010bd92:
      __fprintf_chk(param_1,1,pcVar3);
      return;
    }
    param_3 = param_3 + 1;
    __fprintf_chk(param_1,1,"right:\n");
    param_2 = *(long *)(param_2 + 0x118);
    if (param_2 == 0) {
      return;
    }
  } while( true );
}




// Function: predlist_merge_sort @ 0xbdc0

void predlist_merge_sort(long *param_1,undefined8 param_2)

{
  long lVar1;
  int iVar2;
  long lVar3;
  long lVar4;
  long lVar5;
  long lVar6;
  long in_FS_OFFSET;
  long local_48;
  long local_40;
  long local_30;
  
  lVar4 = *param_1;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (lVar4 == 0) {
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
  }
  else {
    if (((byte)DAT_0013e660 & 8) != 0) {
      __fprintf_chk(stderr,1,&DAT_0012bcb3,"predlist before merge sort");
      FUN_0010baa0(stderr,*param_1,2);
      lVar4 = *param_1;
      FUN_0010b5f0(lVar4);
      FUN_0010a9e0(&local_48);
      lVar6 = local_48;
      goto joined_r0x0010bf60;
    }
    FUN_0010b5f0(lVar4);
    FUN_0010a9e0(&local_48);
    lVar5 = local_48;
    do {
      lVar1 = *(long *)(lVar4 + 0x110);
      *param_1 = lVar1;
      *(undefined8 *)(lVar4 + 0x110) = 0;
      lVar6 = lVar4;
      for (lVar3 = lVar5; lVar4 = lVar1, lVar3 != 0; lVar3 = *(long *)(lVar3 + 0x110)) {
        iVar2 = FUN_0010aa20(*(undefined8 *)(lVar3 + 0x118));
        if (-1 < iVar2) {
          lVar1 = *(long *)(lVar3 + 0x110);
          *(long *)(lVar6 + 0x110) = lVar1;
          if (lVar1 == 0) {
            local_40 = lVar6;
          }
          *(long *)(lVar3 + 0x110) = lVar6;
          lVar6 = lVar5;
          goto joined_r0x0010bf60;
        }
      }
      *(long *)(lVar6 + 0x110) = lVar5;
      local_48 = lVar6;
      if (local_40 == 0) {
        local_40 = lVar6;
      }
joined_r0x0010bf60:
      lVar5 = lVar6;
    } while (lVar4 != 0);
    if (((byte)DAT_0013e660 & 8) != 0) {
      __fprintf_chk(stderr,1,&DAT_0012bcb3,"predlist after merge sort");
      FUN_0010baa0(stderr,lVar6,2);
    }
    FUN_0010b5f0(lVar6);
    FUN_0010ab60(lVar6,local_40,param_2);
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      FUN_0010a9e0(param_1);
      return;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: opt_expr @ 0xc030

undefined8 opt_expr(long *param_1)

{
  int iVar1;
  uint uVar2;
  long lVar3;
  long lVar4;
  char cVar5;
  undefined1 uVar6;
  undefined1 *puVar7;
  long lVar8;
  undefined1 *puVar9;
  long lVar10;
  int iVar11;
  undefined4 uVar12;
  long *plVar13;
  undefined *puVar14;
  undefined8 uVar15;
  undefined8 *puVar16;
  long in_FS_OFFSET;
  undefined1 auVar17 [16];
  undefined8 local_118;
  undefined8 local_110;
  undefined8 local_108;
  undefined8 local_100;
  undefined1 local_f8 [176];
  undefined1 local_48 [8];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_118 = 0;
  local_110 = 0;
  local_108 = 0;
  local_100 = 0;
  if (*param_1 == 0) {
    uVar15 = 0;
    goto LAB_0010c1fc;
  }
  puVar9 = local_48;
  puVar7 = local_f8;
  do {
    auVar17 = FUN_0010a9e0(puVar7);
    lVar8 = auVar17._8_8_;
    puVar7 = auVar17._0_8_;
  } while (puVar9 != puVar7);
  lVar3 = lVar8;
  lVar4 = *(long *)(lVar8 + 0x110);
  if (*(long *)(lVar8 + 0x110) == 0) {
    uVar12 = 3;
    plVar13 = param_1;
  }
  else {
    do {
      lVar8 = lVar4;
      lVar10 = lVar3;
      lVar3 = lVar8;
      lVar4 = *(long *)(lVar8 + 0x110);
    } while (*(long *)(lVar8 + 0x110) != 0);
    uVar12 = *(undefined4 *)(lVar10 + 0x14);
    plVar13 = (long *)(lVar10 + 0x110);
  }
  if (*(int *)(lVar8 + 0x10) == 3) {
    if (((byte)DAT_0013e660 & 9) == 0) {
      lVar8 = *param_1;
      goto LAB_0010c0fe;
    }
LAB_0010c2e0:
    __fprintf_chk(stderr,1,"Normalized Eval Tree:\n");
    FUN_0010baa0(stderr,*param_1,0);
  }
  else {
    FUN_0010b1c0(lVar8,uVar12,plVar13);
    if (((byte)DAT_0013e660 & 9) != 0) goto LAB_0010c2e0;
  }
  lVar8 = *param_1;
  if (lVar8 != 0) {
LAB_0010c0fe:
    iVar1 = *(int *)(lVar8 + 0x10);
    iVar11 = 0;
    if (iVar1 == 3) {
      iVar11 = *(int *)(lVar8 + 0x14);
    }
    uVar15 = 0;
    do {
      if ((iVar1 == 3) && (*(int *)(lVar8 + 0x14) != iVar11)) {
        lVar8 = FUN_0010b1c0(lVar8,iVar11,param_1);
      }
      puVar16 = *(undefined8 **)(lVar8 + 0x118);
      uVar2 = *(uint *)(puVar16 + 2);
      puVar14 = (undefined *)*puVar16;
      if (uVar2 == 2) {
        uVar6 = opt_expr(puVar16 + 0x23);
        *(undefined1 *)(puVar16 + 3) = uVar6;
        puVar16 = *(undefined8 **)(lVar8 + 0x118);
LAB_0010c250:
        if (*(char *)(puVar16 + 3) != '\0') {
LAB_0010c2a0:
          uVar15 = 1;
          FUN_0010bfd0(local_f8,0xb,&local_108,&local_118,param_1);
        }
LAB_0010c257:
        param_1 = (long *)(lVar8 + 0x110);
      }
      else {
        if (2 < uVar2) {
          if (uVar2 != 3) {
            uVar15 = dcgettext(0,"oops -- invalid expression type!",5);
                    /* WARNING: Subroutine does not return */
            error(1,0,uVar15);
          }
          uVar6 = opt_expr();
          *(undefined1 *)(puVar16 + 3) = uVar6;
          puVar16 = *(undefined8 **)(lVar8 + 0x118);
          goto LAB_0010c250;
        }
        if (iVar11 == 1) goto LAB_0010c250;
        if (*(char *)(puVar16 + 3) != '\0') goto LAB_0010c2a0;
        cVar5 = FUN_0010a950();
        if (cVar5 == '\0') {
          if (puVar14 == PTR_FUN_0013df38) {
            FUN_0010a9f0(&local_118,lVar8,param_1);
            goto LAB_0010c25e;
          }
          if ((1 < DAT_0013e688) &&
             (((uVar2 = *(uint *)(puVar16 + 4), uVar2 - 1 < 2 &&
               (*(char *)((long)puVar16 + 0x1a) == '\0')) || (DAT_0013e688 != 2)))) {
            if (((byte)DAT_0013e660 & 8) != 0) {
              __fprintf_chk(stderr,1,"-O%d: categorising predicate ");
              FUN_0010b5a0(stderr,*(undefined8 *)(lVar8 + 0x118));
              FUN_0010ab90(*(undefined4 *)(*(long *)(lVar8 + 0x118) + 0x20),1);
              __fprintf_chk(stderr);
              uVar2 = *(uint *)(*(long *)(lVar8 + 0x118) + 0x20);
            }
            FUN_0010a9f0(local_f8 + (ulong)uVar2 * 0x10,lVar8,param_1);
            goto LAB_0010c25e;
          }
          goto LAB_0010c257;
        }
        if (((byte)DAT_0013e660 & 8) != 0) {
          __fprintf_chk(stderr,1,"-O%d: promoting cheap predicate ",DAT_0013e688);
          FUN_0010b5a0(stderr,*(undefined8 *)(lVar8 + 0x118));
          __fprintf_chk(stderr,1," into name_list\n");
        }
        FUN_0010a9f0(&local_108,lVar8,param_1);
      }
LAB_0010c25e:
      lVar8 = *param_1;
      if (lVar8 == 0) goto LAB_0010c1e4;
      iVar1 = *(int *)(lVar8 + 0x10);
    } while( true );
  }
  uVar15 = 0;
LAB_0010c1e4:
  FUN_0010bfd0(local_f8,0xb,&local_108,&local_118,param_1);
LAB_0010c1fc:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar15;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: consider_arm_swap @ 0xc450

undefined4 consider_arm_swap(long *param_1)

{
  float fVar1;
  float fVar2;
  char cVar3;
  undefined4 uVar4;
  int iVar5;
  int iVar6;
  ulong uVar7;
  long lVar8;
  char *pcVar9;
  long lVar10;
  
  pcVar9 = "Not a binary operation";
  uVar7 = (ulong)((uint)DAT_0013e660 & 8);
  if ((int)param_1[2] != 3) goto LAB_0010c471;
  if ((param_1[0x22] == 0) || (lVar10 = param_1[0x23], lVar10 == 0)) {
    pcVar9 = "Doesn\'t have two arms";
    goto LAB_0010c471;
  }
  lVar8 = *(long *)(param_1[0x22] + 0x118);
  if (lVar8 == 0) {
    pcVar9 = "Left arm has no child on RHS";
    goto LAB_0010c471;
  }
  cVar3 = FUN_0010aa70(lVar8);
  if (cVar3 != '\0') {
    pcVar9 = "Left subtree has side-effects";
    goto LAB_0010c471;
  }
  uVar4 = FUN_0010aa70(lVar10);
  if ((char)uVar4 != '\0') {
    pcVar9 = "Right subtree has side-effects";
    goto LAB_0010c471;
  }
  iVar5 = FUN_0010aab0(lVar8);
  iVar6 = FUN_0010aab0(lVar10);
  if (iVar5 < iVar6) {
    pcVar9 = "efficient as-is";
    goto LAB_0010c471;
  }
  if (iVar5 == iVar6) {
    fVar1 = *(float *)(lVar8 + 0x24);
    fVar2 = *(float *)(lVar10 + 0x24);
    if (uVar7 == 0) {
      if ((undefined *)*param_1 == PTR_FUN_0013dff8) {
        if (fVar1 <= fVar2) {
          return uVar4;
        }
        goto LAB_0010c5f0;
      }
      if ((undefined *)*param_1 != PTR_FUN_0013df50) {
        return uVar4;
      }
    }
    else {
      __fprintf_chk((double)fVar1,(double)fVar2,stderr,1,"Success rates: l=%f, r=%f\n");
      uVar7 = (ulong)((uint)DAT_0013e660 & 8);
      if ((undefined *)*param_1 == PTR_FUN_0013dff8) {
        pcVar9 = "Operation is OR; right success rate >= left";
        if (fVar1 <= fVar2) goto LAB_0010c471;
        goto LAB_0010c5c0;
      }
      pcVar9 = "Not \'AND\' or \'OR\'";
      if ((undefined *)*param_1 != PTR_FUN_0013df50) goto LAB_0010c471;
    }
    if (fVar2 <= fVar1) {
      pcVar9 = "Operation is AND; right success rate <= left";
LAB_0010c471:
      if (uVar7 == 0) {
        return 0;
      }
      __fprintf_chk(stderr,1,"Not an arm swap candidate (%s):\n",pcVar9);
      FUN_0010baa0(stderr,param_1,0);
      return 0;
    }
  }
LAB_0010c5c0:
  if (uVar7 != 0) {
    __fprintf_chk(stderr,1,"Performing arm swap on:\n");
    FUN_0010baa0(stderr,param_1,0);
  }
LAB_0010c5f0:
  FUN_0010ab00(param_1);
  return 1;
}




// Function: do_arm_swaps @ 0xc6b0

undefined8 do_arm_swaps(long param_1)

{
  char cVar1;
  
  if (param_1 != 0) {
    do {
      do {
        do {
          cVar1 = FUN_0010c450(param_1);
        } while (cVar1 != '\0');
        cVar1 = do_arm_swaps(*(undefined8 *)(param_1 + 0x110));
      } while (cVar1 != '\0');
      cVar1 = do_arm_swaps(*(undefined8 *)(param_1 + 0x118));
    } while (cVar1 != '\0');
  }
  return 0;
}




// Function: build_expression_tree @ 0xc6f0

undefined8 build_expression_tree(int param_1,long param_2,int param_3)

{
  long *plVar1;
  int iVar2;
  char cVar3;
  long lVar4;
  long lVar5;
  long lVar6;
  int *piVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  long in_FS_OFFSET;
  int local_5c [4];
  int local_4c;
  long *local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  lVar4 = (long)param_3;
  DAT_0013e6d8 = (long *)0x0;
  DAT_0013e6c0 = param_2 + lVar4 * 8;
  local_5c[0] = param_1;
  local_4c = param_3;
  if (param_3 < param_1) {
    while (cVar3 = FUN_0010dca0(*(undefined8 *)(param_2 + lVar4 * 8),1), cVar3 == '\0') {
      DAT_0013e6b8 = DAT_0013e6b8 + 1;
      local_4c = local_4c + 1;
      if (local_5c[0] <= local_4c) break;
      lVar4 = (long)local_4c;
    }
  }
  lVar4 = FUN_001147f0(&DAT_0012e7ce);
  lVar5 = FUN_001147f0(")");
  lVar6 = FUN_001147f0("print");
  if (lVar4 == 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("entry_open != NULL","tree.c",0x4f6,"build_expression_tree");
  }
  if (lVar5 == 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("entry_close != NULL","tree.c",0x4f7,"build_expression_tree");
  }
  if (lVar6 == 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("entry_print != NULL","tree.c",0x4f8,"build_expression_tree");
  }
  FUN_00113ec0(lVar4,param_2,local_5c);
  plVar1 = DAT_0013e6d8;
  *(undefined **)(DAT_0013e6c8 + 8) = &DAT_0012e7ce;
  *(undefined1 *)((long)plVar1 + 0x29) = 1;
  FUN_001147d0(param_2,local_5c[0]);
  FUN_00109f40(DAT_0013e6c8);
  if (local_4c < local_5c[0]) {
    do {
      DAT_0013e618 = 0;
      cVar3 = FUN_0010dca0(*(undefined8 *)(param_2 + (long)local_4c * 8),0);
      uVar9 = *(undefined8 *)(param_2 + (long)local_4c * 8);
      if (cVar3 == '\0') {
        uVar8 = dcgettext(0,"paths must precede expression: `%s\'",5);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar8,uVar9);
      }
      piVar7 = (int *)FUN_001147f0(uVar9);
      if (piVar7 == (int *)0x0) {
        uVar8 = dcgettext(0,"unknown predicate `%s\'",5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar8,uVar9);
      }
      if (*piVar7 != 4) {
        local_4c = local_4c + 1;
      }
      iVar2 = local_4c;
      cVar3 = (**(code **)(piVar7 + 4))(piVar7,param_2,&local_4c);
      lVar4 = DAT_0013e6c8;
      if (cVar3 == '\0') {
        lVar4 = *(long *)(param_2 + (long)local_4c * 8);
        if (lVar4 == 0) {
          uVar8 = dcgettext(0,"missing argument to `%s\'",5);
                    /* WARNING: Subroutine does not return */
          error(1,0,uVar8,uVar9);
        }
        if ((*piVar7 == 4) && (local_4c == iVar2)) {
          uVar8 = dcgettext(0,"invalid predicate `%s\'",5);
                    /* WARNING: Subroutine does not return */
          error(1,0,uVar8,uVar9);
        }
        uVar8 = dcgettext(0,"invalid argument `%s\' to `%s\'",5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar8,lVar4,uVar9);
      }
      uVar8 = 0;
      *(undefined8 *)(DAT_0013e6c8 + 8) = uVar9;
      if (local_4c != iVar2) {
        uVar8 = *(undefined8 *)(param_2 + (long)iVar2 * 8);
      }
      *(undefined8 *)(lVar4 + 0x30) = uVar8;
      FUN_00109f40();
      FUN_00109f40(DAT_0013e6d8);
    } while (local_4c < local_5c[0]);
  }
  FUN_001147e0(param_2,local_5c[0],DAT_0013e6c8,DAT_0013e6d8);
  plVar1 = DAT_0013e6d8;
  if (DAT_0013e6d8[0x21] == 0) {
    local_48 = DAT_0013e6d8;
    DAT_0013e6c8 = 0;
    DAT_0013e6d8 = (long *)0x0;
    free(plVar1);
    FUN_00112910(lVar6,param_2,local_5c);
    *(char **)(DAT_0013e6c8 + 8) = "-print";
    FUN_00109f40();
    FUN_00109f40(DAT_0013e6d8);
  }
  else {
    cVar3 = thunk_FUN_0010dc7d();
    if (cVar3 == '\0') {
      local_48 = DAT_0013e6d8;
      DAT_0013e6d8 = (long *)DAT_0013e6d8[0x21];
      FUN_00109f40();
      free(local_48);
    }
    else {
      FUN_00113690(lVar5,param_2,local_5c);
      lVar4 = DAT_0013e6c8;
      *(char **)(DAT_0013e6c8 + 8) = ")";
      *(undefined1 *)(lVar4 + 0x29) = 1;
      FUN_00109f40();
      FUN_00112910(lVar6,param_2,local_5c);
      lVar4 = DAT_0013e6c8;
      *(char **)(DAT_0013e6c8 + 8) = "-print";
      *(undefined1 *)(lVar4 + 0x29) = 1;
      FUN_00109f40();
      FUN_00109f40(DAT_0013e6d8);
    }
  }
  if (((byte)DAT_0013e660 & 9) != 0) {
    __fprintf_chk(stderr,1,"Predicate List:\n");
    FUN_00109bc0(stderr,DAT_0013e6d8);
  }
  FUN_00114670(DAT_0013e6d8);
  FUN_00109f40(DAT_0013e6d8);
  local_48 = DAT_0013e6d8;
  uVar9 = FUN_0010ae10(&local_48,0,0);
  DAT_0013e6d0 = uVar9;
  FUN_0010b5f0(uVar9);
  if (local_48 == (long *)0x0) {
    if (((byte)DAT_0013e660 & 9) != 0) {
      __fprintf_chk(stderr,1,"Eval Tree:\n");
      FUN_0010baa0(stderr,DAT_0013e6d0,0);
      uVar9 = DAT_0013e6d0;
    }
    FUN_0010b4d0(uVar9);
    FUN_0010c030(&DAT_0013e6d0);
    uVar9 = DAT_0013e6d0;
    FUN_0010ac90(DAT_0013e6d0,1);
    FUN_0010c6b0(uVar9);
    uVar9 = DAT_0013e6d0;
    FUN_0010ac90(DAT_0013e6d0,1);
    if (((byte)DAT_0013e660 & 9) != 0) {
      __fprintf_chk(stderr,1,"Optimized Eval Tree:\n");
      FUN_0010baa0(stderr,DAT_0013e6d0,0);
      __fprintf_chk(stderr,1,"Optimized command line:\n");
      FUN_00109c70(stderr,DAT_0013e6d0);
      __fprintf_chk(stderr,1,"\n");
      uVar9 = DAT_0013e6d0;
    }
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      return uVar9;
    }
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  if ((code *)*local_48 == FUN_00108c30) {
    uVar9 = dcgettext(0,"you have too many \')\'",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar9);
  }
  lVar4 = local_48[1];
  if (lVar4 != 0) {
    uVar9 = dcgettext(0,"unexpected extra predicate \'%s\'",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar9,lVar4);
  }
  uVar9 = dcgettext(0,"unexpected extra predicate",5);
                    /* WARNING: Subroutine does not return */
  error(1,0,uVar9);
}




// Function: traverse_tree @ 0xcdf0

void traverse_tree(long param_1,code *param_2)

{
  do {
    if (*(long *)(param_1 + 0x110) != 0) {
      traverse_tree(*(long *)(param_1 + 0x110),param_2);
    }
    (*param_2)(param_1);
    param_1 = *(long *)(param_1 + 0x118);
  } while (param_1 != 0);
  return;
}




// Function: undangle_file_pointers @ 0xce30

void undangle_file_pointers(long *param_1)

{
  undefined *puVar1;
  
  puVar1 = (undefined *)*param_1;
  if (((puVar1 == PTR_FUN_0013dfa0 || puVar1 == PTR_FUN_0013dfb0) || puVar1 == PTR_FUN_0013dfa8) ||
     (puVar1 == PTR_FUN_0013df60)) {
    param_1[8] = 0;
  }
  return;
}




// Function: show_valid_debug_options @ 0xce70

void show_valid_debug_options(int param_1)

{
  FILE *__stream;
  char *pcVar1;
  long lVar2;
  undefined **ppuVar3;
  undefined *puVar4;
  
  __stream = stdout;
  pcVar1 = (char *)dcgettext(0,"Valid arguments for -D:\n",5);
  fputs(pcVar1,__stream);
  ppuVar3 = &PTR_DAT_0013c158;
  if (param_1 != 0) {
    puVar4 = &DAT_0012e94b;
    pcVar1 = "Show diagnostic information relating to -exec, -execdir, -ok and -okdir";
    for (; __fprintf_chk(stdout,1,"%-10s %s\n",puVar4,pcVar1),
        ppuVar3 != (undefined **)&DAT_0013c218; ppuVar3 = ppuVar3 + 3) {
      pcVar1 = ppuVar3[2];
      puVar4 = *ppuVar3;
    }
    return;
  }
  puVar4 = &DAT_0012e94b;
  lVar2 = 0;
  do {
    if (lVar2 == 0) {
      __fprintf_chk(stdout,1,&DAT_0012b690,"",puVar4);
    }
    else {
      __fprintf_chk(stdout,1,&DAT_0012b690,&DAT_0012c5af,puVar4);
      if (lVar2 + 1 == 9) {
        return;
      }
    }
    puVar4 = *ppuVar3;
    ppuVar3 = ppuVar3 + 3;
    lVar2 = lVar2 + 1;
  } while( true );
}




// Function: do_exec @ 0xcf70

void do_exec(long param_1)

{
  FUN_001154f0(*(undefined8 *)(param_1 + 0xc0),FUN_0010d060,param_1);
  if (*(long *)(param_1 + 0xc0) != DAT_0013e5c0) {
    FUN_00125280();
    free(*(void **)(param_1 + 0xc0));
    *(undefined8 *)(param_1 + 0xc0) = 0;
  }
  return;
}




// Function: do_complete_pending_execdirs @ 0xcfc0

void do_complete_pending_execdirs(long *param_1)

{
  if (param_1 == (long *)0x0) {
    return;
  }
  do {
    while( true ) {
      if (DAT_0013e608 == '\0') {
                    /* WARNING: Subroutine does not return */
        __assert_fail("state.execdirs_outstanding","util.c",0x18c,"do_complete_pending_execdirs");
      }
      do_complete_pending_execdirs(param_1[0x22]);
      if (((((undefined *)*param_1 != PTR_FUN_0013df28) &&
           ((undefined *)*param_1 != PTR_FUN_0013dfc8)) || ((char)param_1[7] == '\0')) ||
         ((int)param_1[0x1a] == 0)) break;
      FUN_0010cf70(param_1 + 7);
      param_1 = (long *)param_1[0x23];
      if (param_1 == (long *)0x0) {
        return;
      }
    }
    param_1 = (long *)param_1[0x23];
  } while (param_1 != (long *)0x0);
  return;
}




// Function: exec_cb @ 0xd060

undefined8 exec_cb(long param_1)

{
  FUN_00114e90(param_1 + 8,param_1 + 0x60);
  return 0;
}




// Function: complete_pending_execs @ 0xd080

void complete_pending_execs(long *param_1)

{
  if (param_1 == (long *)0x0) {
    return;
  }
  do {
    complete_pending_execs(param_1[0x22]);
    if ((((code *)*param_1 == thunk_FUN_0010a390) && ((char)param_1[7] != '\0')) &&
       ((int)param_1[0x1a] != 0)) {
      FUN_00114e90(param_1 + 8,param_1 + 0x13);
    }
    param_1 = (long *)param_1[0x23];
  } while (param_1 != (long *)0x0);
  return;
}




// Function: cleanup_initial_cwd @ 0xd0f0

void cleanup_initial_cwd(void)

{
  int iVar1;
  undefined8 uVar2;
  int *piVar3;
  char *pcVar4;
  char *pcVar5;
  
  iVar1 = FUN_00125250(DAT_0013e5c0);
  if (iVar1 == 0) {
    FUN_00125280(DAT_0013e5c0);
    free(DAT_0013e5c0);
    DAT_0013e5c0 = (int *)0x0;
    return;
  }
  pcVar4 = "";
  pcVar5 = "";
  if (*DAT_0013e5c0 < 0) {
    pcVar4 = *(char **)(DAT_0013e5c0 + 2);
    pcVar5 = ": ";
    if (pcVar4 == (char *)0x0) {
      pcVar4 = "";
      pcVar5 = pcVar4;
    }
  }
  uVar2 = dcgettext(0,"Failed to restore initial working directory%s%s",5);
  piVar3 = __errno_location();
                    /* WARNING: Subroutine does not return */
  error(0,*piVar3,uVar2,pcVar5,pcVar4);
}




// Function: fallback_stat @ 0xd1a0

int fallback_stat(char *param_1,stat *param_2,int param_3)

{
  int iVar1;
  int *piVar2;
  
  piVar2 = __errno_location();
  if ((*piVar2 != 2) && (*piVar2 != 0x14)) {
    return param_3;
  }
  if (((byte)DAT_0013e660 & 2) != 0) {
    __fprintf_chk(stderr,1,"fallback_stat(): stat(%s) failed; falling back on lstat()\n",param_1);
  }
  iVar1 = fstatat(DAT_0013e5f8,param_1,param_2,0x100);
  return iVar1;
}




// Function: process_optimisation_option @ 0xd220

void process_optimisation_option(byte *param_1)

{
  byte bVar1;
  int iVar2;
  ushort **ppuVar3;
  int *piVar4;
  ulong uVar5;
  undefined8 uVar6;
  long in_FS_OFFSET;
  byte *local_38;
  long local_30;
  
  bVar1 = *param_1;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (bVar1 == 0) {
    uVar6 = dcgettext(0,"The -O option must be immediately followed by a decimal integer",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar6);
  }
  ppuVar3 = __ctype_b_loc();
  if ((*(byte *)((long)*ppuVar3 + (ulong)bVar1 * 2 + 1) & 8) == 0) {
    uVar6 = dcgettext(0,"Please specify a decimal number immediately after -O",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar6);
  }
  piVar4 = __errno_location();
  iVar2 = *piVar4;
  *piVar4 = 0;
  uVar5 = strtoul((char *)param_1,(char **)&local_38,10);
  if (uVar5 == 0) {
    if (local_38 == param_1) {
      uVar6 = dcgettext(0,"Please specify a decimal number immediately after -O",5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar6);
    }
    if (*local_38 == 0) {
LAB_0010d295:
      DAT_0013e688 = (undefined2)uVar5;
      *piVar4 = iVar2;
      if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return;
    }
  }
  else if (*local_38 == 0) {
    if (uVar5 == 0xffffffffffffffff) {
      if (*piVar4 != 0) {
        uVar6 = dcgettext(0,"Invalid optimisation level %s",5);
                    /* WARNING: Subroutine does not return */
        error(1,*piVar4,uVar6,param_1);
      }
    }
    else if (uVar5 < 0x10000) goto LAB_0010d295;
    uVar6 = dcgettext(0,
                      "Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate."
                      ,5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar6,uVar5);
  }
  uVar6 = dcgettext(0,"Invalid optimisation level %s",5);
                    /* WARNING: Subroutine does not return */
  error(1,0,uVar6,param_1);
}




// Function: now @ 0xd3d0

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
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_28.tv_sec;
}




// Function: insert_primary_withpred @ 0xd450

void insert_primary_withpred(long param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 uVar1;
  undefined8 *puVar2;
  
  puVar2 = (undefined8 *)FUN_0010b990(param_1,param_3);
  uVar1 = *(undefined8 *)(param_1 + 8);
  *puVar2 = param_2;
  puVar2[1] = uVar1;
  puVar2[7] = 0;
  puVar2[2] = 1;
  return;
}




// Function: insert_primary @ 0xd490

void insert_primary(long param_1,undefined8 param_2)

{
  if (*(long *)(param_1 + 0x18) != 0) {
    FUN_0010d450(param_1,*(long *)(param_1 + 0x18),param_2);
    return;
  }
                    /* WARNING: Subroutine does not return */
  __assert_fail("entry->pred_func != NULL","util.c",0x7b,"insert_primary");
}




// Function: insert_primary_noarg @ 0xd4d0

void insert_primary_noarg(undefined8 param_1)

{
  FUN_0010d490(param_1,0);
  return;
}




// Function: usage @ 0xd4e0

void usage(int param_1)

{
  FILE *pFVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  char *pcVar4;
  
  uVar2 = DAT_0013eed0;
  if (param_1 != 0) {
    uVar3 = dcgettext(0,"Try \'%s --help\' for more information.\n",5);
    __fprintf_chk(stderr,1,uVar3,uVar2);
                    /* WARNING: Subroutine does not return */
    exit(param_1);
  }
  uVar3 = dcgettext(0,"Usage: %s [-H] [-L] [-P] [-Olevel] [-D debugopts] [path...] [expression]\n",5
                   );
  __fprintf_chk(stdout,1,uVar3,uVar2);
  pFVar1 = stdout;
  pcVar4 = (char *)dcgettext(0,
                             "\nDefault path is the current directory; default expression is -print.\nExpression may consist of: operators, options, tests, and actions.\n"
                             ,5);
  fputs(pcVar4,pFVar1);
  pFVar1 = stdout;
  pcVar4 = (char *)dcgettext(0,
                             "\nOperators (decreasing precedence; -and is implicit where no others are given):\n      ( EXPR )   ! EXPR   -not EXPR   EXPR1 -a EXPR2   EXPR1 -and EXPR2\n      EXPR1 -o EXPR2   EXPR1 -or EXPR2   EXPR1 , EXPR2\n"
                             ,5);
  fputs(pcVar4,pFVar1);
  pFVar1 = stdout;
  pcVar4 = (char *)dcgettext(0,
                             "\nPositional options (always true):\n      -daystart -follow -nowarn -regextype -warn\n"
                             ,5);
  fputs(pcVar4,pFVar1);
  pFVar1 = stdout;
  pcVar4 = (char *)dcgettext(0,
                             "\nNormal options (always true, specified before other expressions):\n      -depth -files0-from FILE -maxdepth LEVELS -mindepth LEVELS\n       -mount -noleaf -xdev -ignore_readdir_race -noignore_readdir_race\n"
                             ,5);
  fputs(pcVar4,pFVar1);
  pFVar1 = stdout;
  pcVar4 = (char *)dcgettext(0,
                             "\nTests (N can be +N or -N or N):\n      -amin N -anewer FILE -atime N -cmin N -cnewer FILE -context CONTEXT\n      -ctime N -empty -false -fstype TYPE -gid N -group NAME -ilname PATTERN\n      -iname PATTERN -inum N -iwholename PATTERN -iregex PATTERN\n      -links N -lname PATTERN -mmin N -mtime N -name PATTERN -newer FILE\n      -nouser -nogroup -path PATTERN -perm [-/]MODE -regex PATTERN\n      -readable -writable -executable\n      -wholename PATTERN -size N[bcwkMG] -true -type [bcdpflsD] -uid N\n      -used N -user NAME -xtype [bcdpfls]\n"
                             ,5);
  fputs(pcVar4,pFVar1);
  pFVar1 = stdout;
  pcVar4 = (char *)dcgettext(0,
                             "\nActions:\n      -delete -print0 -printf FORMAT -fprintf FILE FORMAT -print \n      -fprint0 FILE -fprint FILE -ls -fls FILE -prune -quit\n      -exec COMMAND ; -exec COMMAND {} + -ok COMMAND ;\n      -execdir COMMAND ; -execdir COMMAND {} + -okdir COMMAND ;\n"
                             ,5);
  fputs(pcVar4,pFVar1);
  pFVar1 = stdout;
  pcVar4 = (char *)dcgettext(0,"\nOther common options:\n",5);
  fputs(pcVar4,pFVar1);
  pFVar1 = stdout;
  pcVar4 = (char *)dcgettext(0,"      --help                   display this help and exit\n",5);
  fputs(pcVar4,pFVar1);
  pFVar1 = stdout;
  pcVar4 = (char *)dcgettext(0,
                             "      --version                output version information and exit\n\n"
                             ,5);
  fputs(pcVar4,pFVar1);
  FUN_0010ce70(0);
  pFVar1 = stdout;
  pcVar4 = (char *)dcgettext(0,
                             "\nUse \'-D help\' for a description of the options, or see find(1)\n\n"
                             ,5);
  fputs(pcVar4,pFVar1);
  FUN_00117130(stdout,DAT_0013eed0);
                    /* WARNING: Subroutine does not return */
  exit(0);
}




// Function: process_debug_options @ 0xd6f0

void process_debug_options(char *param_1)

{
  int iVar1;
  char *__s2;
  undefined8 uVar2;
  undefined8 uVar3;
  char *__s1;
  undefined **ppuVar4;
  long lVar5;
  long in_FS_OFFSET;
  char *local_50;
  char local_42 [2];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_50 = (char *)0x0;
  local_42[0] = ',';
  local_42[1] = '\0';
  __s2 = strtok_r(param_1,local_42,&local_50);
  if (__s2 == (char *)0x0) {
    uVar2 = dcgettext(0,"Empty argument to the -D option.",5);
                    /* WARNING: Subroutine does not return */
    error(0,0,uVar2);
  }
  do {
    ppuVar4 = &PTR_DAT_0013c158;
    __s1 = "exec";
    lVar5 = 0;
    while (iVar1 = strcmp(__s1,__s2), iVar1 != 0) {
      lVar5 = lVar5 + 1;
      if (lVar5 == 9) {
        uVar2 = FUN_00124e50(0,DAT_0013e68c,param_1);
        uVar3 = dcgettext(0,"Ignoring unrecognised debug flag %s",5);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar3,uVar2);
      }
      __s1 = *ppuVar4;
      ppuVar4 = ppuVar4 + 3;
    }
    DAT_0013e660 = DAT_0013e660 | (long)(int)(&DAT_0013c148)[lVar5 * 6];
    __s2 = strtok_r((char *)0x0,local_42,&local_50);
  } while (__s2 != (char *)0x0);
  if ((DAT_0013e660 & 0x10) != 0) {
    FUN_0010ce70();
                    /* WARNING: Subroutine does not return */
    exit(0);
  }
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: set_stat_placeholders @ 0xd860

void set_stat_placeholders(void)

{
  return;
}




// Function: check_nofollow @ 0xd870

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

bool check_nofollow(void)

{
  int iVar1;
  bool bVar2;
  long in_FS_OFFSET;
  double dVar3;
  utsname uStack_1a8;
  long local_20;
  
  bVar2 = true;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = uname(&uStack_1a8);
  if (iVar1 == 0) {
    dVar3 = atof(uStack_1a8.release);
    iVar1 = strcmp("Linux",uStack_1a8.sysname);
    if (iVar1 == 0) {
      bVar2 = _DAT_0012d2e0 <= (float)dVar3;
    }
    else {
      iVar1 = strcmp("FreeBSD",uStack_1a8.sysname);
      if (iVar1 == 0) {
        bVar2 = (float)s__ffF_sharefile_c_0012d2e3._1_4_ <= (float)dVar3;
      }
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: complete_pending_execdirs @ 0xd940

void complete_pending_execdirs(void)

{
  undefined8 uVar1;
  
  if (DAT_0013e608 == '\0') {
    return;
  }
  uVar1 = FUN_0010b5e0();
  FUN_0010cfc0(uVar1);
  DAT_0013e608 = 0;
  return;
}




// Function: record_initial_cwd @ 0xd970

void record_initial_cwd(void)

{
  int iVar1;
  undefined8 uVar2;
  int *piVar3;
  char *pcVar4;
  char *pcVar5;
  
  DAT_0013e5c0 = (int *)FUN_00126990(0x10);
  iVar1 = FUN_001251f0(DAT_0013e5c0);
  if (iVar1 == 0) {
    return;
  }
  pcVar4 = "";
  pcVar5 = "";
  if (*DAT_0013e5c0 < 0) {
    pcVar4 = *(char **)(DAT_0013e5c0 + 2);
    pcVar5 = ": ";
    if (pcVar4 == (char *)0x0) {
      pcVar4 = "";
      pcVar5 = pcVar4;
    }
  }
  uVar2 = dcgettext(0,"Failed to save initial working directory%s%s",5);
  piVar3 = __errno_location();
                    /* WARNING: Subroutine does not return */
  error(1,*piVar3,uVar2,pcVar5,pcVar4);
}




// Function: optionh_stat @ 0xda00

int optionh_stat(char *param_1,stat *param_2)

{
  int iVar1;
  
  if ((DAT_0013e5f8 != -100) && (DAT_0013e5f8 < 0)) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("state.cwd_dir_fd >= 0","util.c",0x25d,"optionh_stat");
  }
  if (DAT_0013e5e0 != 0) {
    iVar1 = fstatat(DAT_0013e5f8,param_1,param_2,0x100);
    return iVar1;
  }
  iVar1 = fstatat(DAT_0013e5f8,param_1,param_2,0);
  if (iVar1 == 0) {
    return 0;
  }
  iVar1 = FUN_0010d1a0(param_1,param_2,iVar1);
  return iVar1;
}




// Function: optionl_stat @ 0xdaa0

undefined8 optionl_stat(char *param_1,stat *param_2)

{
  int iVar1;
  undefined8 uVar2;
  
  if ((DAT_0013e5f8 != -100) && (DAT_0013e5f8 < 0)) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("state.cwd_dir_fd >= 0","util.c",0x27c,"optionl_stat");
  }
  iVar1 = fstatat(DAT_0013e5f8,param_1,param_2,0);
  if (iVar1 == 0) {
    return 0;
  }
  uVar2 = FUN_0010d1a0(param_1,param_2,iVar1);
  return uVar2;
}




// Function: optionp_stat @ 0xdb20

void optionp_stat(char *param_1,stat *param_2)

{
  if ((DAT_0013e5f8 < 0) && (DAT_0013e5f8 != -100)) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("(state.cwd_dir_fd >= 0) || (state.cwd_dir_fd==AT_FDCWD)","util.c",0x28d,
                  "optionp_stat");
  }
  fstatat(DAT_0013e5f8,param_1,param_2,0x100);
  return;
}




// Function: debug_stat @ 0xdb70

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void debug_stat(undefined8 param_1,undefined8 param_2)

{
  _DAT_0013e6e0 = _DAT_0013e6e0 + 1;
  __fprintf_chk(stderr,1,"debug_stat (%s)\n",param_1);
  if (DAT_0013e668 == 1) {
    FUN_0010daa0(param_1,param_2);
    return;
  }
  if (DAT_0013e668 != 2) {
    if (DAT_0013e668 != 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("0","util.c",0x2a5,"debug_stat");
    }
    FUN_0010db20(param_1,param_2);
    return;
  }
  FUN_0010da00(param_1,param_2);
  return;
}




// Function: following_links @ 0xdc30

undefined4 following_links(void)

{
  undefined4 uVar1;
  
  uVar1 = 1;
  if ((DAT_0013e668 != 1) && (uVar1 = 0, DAT_0013e668 == 2)) {
    uVar1 = CONCAT31((int3)((uint)DAT_0013e5e0 >> 8),DAT_0013e5e0 == 0);
  }
  return uVar1;
}




// Function: looks_like_expression @ 0xdca0

ulong looks_like_expression(char *param_1,uint param_2)

{
  undefined7 uVar1;
  
  uVar1 = (undefined7)
          ((ulong)((long)&switchD_0010dcbf::switchdataD_0012d230 +
                  (long)(int)(&switchD_0010dcbf::switchdataD_0012d230)[(byte)(*param_1 - 0x21)]) >>
          8);
  switch(*param_1) {
  case '!':
  case '(':
    return CONCAT71(uVar1,param_1[1] == '\0');
  default:
    break;
  case ')':
  case ',':
    if (param_1[1] == '\0') {
      return (ulong)(param_2 ^ 1);
    }
    break;
  case '-':
    return CONCAT71(uVar1,param_1[1] != '\0');
  }
  return 0;
}




// Function: process_leading_options @ 0xdd00

int process_leading_options(int param_1,long param_2)

{
  char *__s2;
  int iVar1;
  long lVar2;
  undefined8 uVar3;
  int iVar4;
  
  if (param_1 < 2) {
    iVar4 = 1;
  }
  else {
    iVar4 = 1;
    do {
      while( true ) {
        lVar2 = (long)iVar4;
        __s2 = *(char **)(param_2 + lVar2 * 8);
        iVar1 = strcmp("-H",__s2);
        if (iVar1 != 0) break;
        iVar4 = iVar4 + 1;
        FUN_001146f0(2);
        if (param_1 <= iVar4) {
          return iVar4;
        }
      }
      iVar1 = strcmp("-L",__s2);
      if (iVar1 == 0) {
        FUN_001146f0(1);
      }
      else {
        iVar1 = strcmp("-P",__s2);
        if (iVar1 == 0) {
          FUN_001146f0(0);
        }
        else {
          iVar1 = strcmp("--",__s2);
          if (iVar1 == 0) {
            return iVar4 + 1;
          }
          iVar1 = strcmp("-D",__s2);
          if (iVar1 == 0) {
            iVar4 = iVar4 + 1;
            if (param_1 <= iVar4) {
              uVar3 = dcgettext(0,"Missing argument after the -D option.",5);
                    /* WARNING: Subroutine does not return */
              error(0,0,uVar3);
            }
            FUN_0010d6f0(*(undefined8 *)(param_2 + 8 + lVar2 * 8));
          }
          else {
            iVar1 = strncmp("-O",__s2,2);
            if (iVar1 != 0) {
              return iVar4;
            }
            FUN_0010d220(__s2 + 2);
          }
        }
      }
      iVar4 = iVar4 + 1;
    } while (iVar4 < param_1);
  }
  return iVar4;
}




// Function: set_option_defaults @ 0xde90

void set_option_defaults(undefined2 *param_1)

{
  undefined1 uVar1;
  int iVar2;
  char *pcVar3;
  undefined8 uVar4;
  undefined1 auVar5 [16];
  
  pcVar3 = getenv("POSIXLY_CORRECT");
  *(bool *)((long)param_1 + 0x11) = pcVar3 != (char *)0x0;
  uVar1 = FUN_0010d870();
  *(undefined4 *)(param_1 + 0x2e) = 0;
  *(undefined1 *)(param_1 + 0x2c) = uVar1;
  iVar2 = isatty(0);
  *(undefined1 *)((long)param_1 + 0xf) = 0;
  *(bool *)(param_1 + 8) = iVar2 != 0;
  if (*(char *)((long)param_1 + 0x11) != '\0') {
    *(undefined1 *)(param_1 + 8) = 0;
  }
  *(undefined8 *)(param_1 + 2) = 0xffffffffffffffff;
  *param_1 = 1;
  auVar5 = FUN_0010d3d0();
  *(undefined1 *)(param_1 + 0x1c) = 0;
  *(undefined1 (*) [16])(param_1 + 0xc) = auVar5;
  *(long *)(param_1 + 0x18) = auVar5._8_8_;
  *(long *)(param_1 + 0x14) = auVar5._0_8_ + -0x15180;
  *(undefined2 *)((long)param_1 + 0xd) = 0;
  *(undefined8 *)(param_1 + 0x20) = 0;
  *(uint *)(param_1 + 0x1e) = (-(uint)(*(char *)((long)param_1 + 0x11) == '\0') & 0x200) + 0x200;
  param_1[0x34] = 2;
  pcVar3 = getenv("FIND_BLOCK_SIZE");
  if (pcVar3 == (char *)0x0) {
    *(undefined1 *)(param_1 + 6) = 0;
    FUN_001146f0(0);
    *(undefined4 *)(param_1 + 0x36) = 8;
    *(undefined8 *)(param_1 + 0x38) = 0;
    *(undefined1 *)(param_1 + 0x3c) = 0;
    return;
  }
  uVar4 = dcgettext(0,
                    "The environment variable FIND_BLOCK_SIZE is not supported, the only thing that affects the block size is the POSIXLY_CORRECT environment variable"
                    ,5);
                    /* WARNING: Subroutine does not return */
  error(1,0,uVar4);
}




// Function: is_exec_in_local_dir @ 0xdf90

uint is_exec_in_local_dir(undefined *param_1,undefined8 param_2,undefined4 param_3)

{
  undefined8 in_RAX;
  
  return (uint)CONCAT71((int7)((ulong)in_RAX >> 8),param_1 == PTR_FUN_0013df28) |
         CONCAT31((int3)((uint)param_3 >> 8),param_1 == PTR_FUN_0013dfc8);
}




// Function: safely_quote_err_filename @ 0xdfb0

void safely_quote_err_filename(undefined8 param_1,undefined8 param_2)

{
  FUN_00124e50(param_1,DAT_0013e68c,param_2);
  return;
}




// Function: report_file_err @ 0xdfd0

void report_file_err(undefined4 param_1,undefined4 param_2,char param_3,undefined8 param_4)

{
  undefined8 uVar1;
  
  if (param_3 == '\0') {
    uVar1 = FUN_0010dfb0(0,param_4);
                    /* WARNING: Subroutine does not return */
    error(param_1,param_2,&DAT_0013155a,uVar1);
  }
  if (DAT_0013e618 != '\0') {
    DAT_0013e618 = 1;
    return;
  }
  uVar1 = FUN_0010dfb0(0,param_4);
                    /* WARNING: Subroutine does not return */
  error(param_1,param_2,&DAT_0013155a,uVar1);
}




// Function: nonfatal_target_file_error @ 0xe060

void nonfatal_target_file_error(undefined4 param_1,undefined8 param_2)

{
  FUN_0010dfd0(0,param_1,1,param_2);
  return;
}




// Function: get_statinfo @ 0xe080

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 get_statinfo(undefined8 param_1,undefined8 param_2,long param_3)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  int *piVar4;
  
  if (DAT_0013e5e4 == '\0') {
    iVar1 = (*DAT_0013e670)(param_2,param_3);
    if (iVar1 != 0) {
      piVar4 = __errno_location();
      if ((DAT_0013e62e != '\0') && (*piVar4 == 2)) {
        return 0xffffffff;
      }
      FUN_0010e060(*piVar4,param_1);
      return 0xffffffff;
    }
    iVar1 = *(int *)(param_3 + 0x18);
    if (iVar1 == 0) {
      uVar2 = FUN_00124e50(0,DAT_0013e68c,param_2);
      uVar3 = dcgettext(0,"WARNING: file %s appears to have mode 0000",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar3,uVar2);
    }
  }
  else {
    iVar1 = *(int *)(param_3 + 0x18);
  }
  DAT_0013e5e8 = iVar1;
  _DAT_0013e5e4 = 0x101;
  return 0;
}




// Function: digest_mode @ 0xe150

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4
digest_mode(uint *param_1,undefined8 param_2,undefined8 param_3,long param_4,undefined4 param_5)

{
  undefined4 uVar1;
  int iVar2;
  uint uVar3;
  
  uVar3 = *param_1;
  if (uVar3 == 0) {
    if ((char)param_5 != '\0') {
      _DAT_0013e5e4 = 0;
      DAT_0013e5e8 = 0;
      return param_5;
    }
    iVar2 = FUN_0010e080(param_2,param_3,param_4);
    if (iVar2 == 0) {
      DAT_0013e5e8 = *(uint *)(param_4 + 0x18);
      *param_1 = DAT_0013e5e8;
      _DAT_0013e5e4 = CONCAT11(1,DAT_0013e5e4);
      return 1;
    }
  }
  else {
    if (((uVar3 & 0xf000) != 0xa000) || (uVar1 = FUN_0010dc30(), (char)uVar1 == '\0')) {
      *(uint *)(param_4 + 0x18) = uVar3;
      _DAT_0013e5e4 = CONCAT11(1,DAT_0013e5e4);
      DAT_0013e5e8 = uVar3;
      return 1;
    }
    iVar2 = FUN_0010e080();
    if (iVar2 == 0) {
      DAT_0013e5e8 = *(uint *)(param_4 + 0x18);
      *param_1 = DAT_0013e5e8;
      _DAT_0013e5e4 = CONCAT11(1,DAT_0013e5e4);
      return uVar1;
    }
  }
  return 0;
}




// Function: get_info @ 0xe230

int get_info(undefined8 param_1,long param_2,long param_3)

{
  int iVar1;
  
  if ((((*(char *)(param_3 + 0x1a) == '\0') || (DAT_0013e5e4 != '\0')) &&
      ((*(char *)(param_3 + 0x1b) == '\0' || (DAT_0013e5e5 != '\0')))) &&
     ((*(char *)(param_3 + 0x1c) == '\0' ||
      (((*(long *)(param_2 + 8) != 0 && (DAT_0013e5e5 != '\0')) &&
       ((*(uint *)(param_2 + 0x18) & 0xf000) != 0x4000)))))) {
    return 0;
  }
  iVar1 = FUN_0010e080(param_1,DAT_0013e5f0,param_2);
  return -(uint)(iVar1 != 0);
}




// Function: apply_predicate @ 0xe2a0

undefined8 apply_predicate(undefined8 param_1,undefined8 param_2,undefined8 *param_3)

{
  int iVar1;
  undefined8 uVar2;
  
  param_3[0x24] = param_3[0x24] + 1;
  if ((((param_3[3] & 0xffffff0000) == 0) || (iVar1 = FUN_0010e230(), iVar1 == 0)) &&
     (uVar2 = (*(code *)*param_3)(param_1,param_2,param_3), (char)uVar2 != '\0')) {
    param_3[0x25] = param_3[0x25] + 1;
    return uVar2;
  }
  return 0;
}




// Function: fatal_target_file_error @ 0xe310

void fatal_target_file_error(undefined4 param_1,undefined8 param_2)

{
  FUN_0010dfd0(1,param_1,1,param_2);
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: nonfatal_nontarget_file_error @ 0xe340

void nonfatal_nontarget_file_error(undefined4 param_1,undefined8 param_2)

{
  FUN_0010dfd0(0,param_1,0,param_2);
  return;
}




// Function: cleanup @ 0xe360

void cleanup(void)

{
  char cVar1;
  int iVar2;
  long lVar3;
  int *piVar4;
  
  lVar3 = FUN_0010b5e0();
  if (lVar3 == 0) {
    FUN_0010e560(DAT_0013e610);
    FUN_0010d0f0();
    cVar1 = FUN_00115dd0();
  }
  else {
    FUN_0010cdf0(lVar3,FUN_0010d080);
    FUN_0010d940();
    FUN_0010e560(DAT_0013e610);
    FUN_0010cdf0(lVar3,FUN_0010ce30);
    FUN_0010d0f0();
    cVar1 = FUN_00115dd0();
  }
  if (cVar1 != '\0') {
    FUN_00115df0();
    FUN_00115da0();
  }
  iVar2 = FUN_00117bc0(stdout);
  if (iVar2 == -1) {
    piVar4 = __errno_location();
    FUN_0010e340(*piVar4,"standard output");
    return;
  }
  return;
}




// Function: fatal_nontarget_file_error @ 0xe410

void fatal_nontarget_file_error(undefined4 param_1,undefined8 param_2)

{
  DAT_0013e618 = 0;
  FUN_0010dfd0(1,param_1,0,param_2);
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: entry_comparator @ 0xe440

undefined8 entry_comparator(long *param_1,long *param_2)

{
  if (param_1[1] != param_2[1]) {
    return 0;
  }
  return CONCAT71((int7)((ulong)*param_2 >> 8),*param_1 == *param_2);
}




// Function: entry_hashfunc @ 0xe470

ulong entry_hashfunc(ulong *param_1,ulong param_2)

{
  return (*param_1 ^ param_1[1]) % param_2;
}




// Function: entry_free @ 0xe490

void entry_free(void *param_1)

{
  int iVar1;
  int *piVar2;
  
  if (*(long *)((long)param_1 + 0x18) != 0) {
    iVar1 = FUN_00117850();
    if (iVar1 != 0) {
      piVar2 = __errno_location();
                    /* WARNING: Subroutine does not return */
      FUN_0010e410(*piVar2,*(undefined8 *)((long)param_1 + 0x10));
    }
  }
  free(*(void **)((long)param_1 + 0x10));
  free(param_1);
  return;
}




// Function: sharefile_init @ 0xe4d0

undefined8 * sharefile_init(char *param_1)

{
  undefined8 *__ptr;
  char *pcVar1;
  long lVar2;
  undefined8 *puVar3;
  
  __ptr = malloc(0x10);
  puVar3 = __ptr;
  if (__ptr != (undefined8 *)0x0) {
    pcVar1 = strdup(param_1);
    *__ptr = pcVar1;
    if (pcVar1 == (char *)0x0) {
      puVar3 = (undefined8 *)0x0;
      free(__ptr);
    }
    else {
      lVar2 = FUN_0011ac70(0xb,0,FUN_0010e470,FUN_0010e440,FUN_0010e490);
      __ptr[1] = lVar2;
      if (lVar2 == 0) {
        free((void *)*__ptr);
        puVar3 = (undefined8 *)0x0;
        free(__ptr);
      }
    }
  }
  return puVar3;
}




// Function: sharefile_destroy @ 0xe560

void sharefile_destroy(undefined8 *param_1)

{
  free((void *)*param_1);
  FUN_0011ae30(param_1[1]);
  return;
}




// Function: sharefile_fopen @ 0xe580

__dev_t sharefile_fopen(undefined8 *param_1,char *param_2)

{
  undefined8 uVar1;
  int iVar2;
  __dev_t *__ptr;
  char *pcVar3;
  FILE *__stream;
  long lVar4;
  int *piVar5;
  __dev_t _Var6;
  long in_FS_OFFSET;
  stat local_c8;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  __ptr = malloc(0x20);
  if (__ptr == (__dev_t *)0x0) {
    _Var6 = 0;
  }
  else {
    pcVar3 = strdup(param_2);
    __ptr[2] = (__dev_t)pcVar3;
    if (pcVar3 == (char *)0x0) {
      free(__ptr);
      _Var6 = 0;
    }
    else {
      __stream = (FILE *)FUN_00117da0(param_2,*param_1);
      __ptr[3] = (__dev_t)__stream;
      if (__stream == (FILE *)0x0) {
        FUN_0010e490(__ptr);
        _Var6 = 0;
      }
      else {
        iVar2 = fileno(__stream);
        if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
          __assert_fail("fd >= 0","sharefile.c",0xa6,"sharefile_fopen");
        }
        FUN_001174f0(iVar2,1);
        iVar2 = fstat(iVar2,&local_c8);
        if (iVar2 < 0) {
          _Var6 = 0;
          FUN_0010e490(__ptr);
        }
        else {
          uVar1 = param_1[1];
          *__ptr = local_c8.st_dev;
          __ptr[1] = local_c8.st_ino;
          lVar4 = FUN_0011aa10(uVar1,__ptr);
          if (lVar4 == 0) {
            lVar4 = FUN_0011b290(param_1[1],__ptr);
            if (lVar4 == 0) {
              piVar5 = __errno_location();
              iVar2 = *piVar5;
              FUN_0010e490(__ptr);
              *piVar5 = iVar2;
              _Var6 = 0;
            }
            else {
              _Var6 = __ptr[3];
            }
          }
          else {
            FUN_0010e490(__ptr);
            _Var6 = *(__dev_t *)(lVar4 + 0x18);
          }
        }
      }
    }
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return _Var6;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: is_octal_char @ 0xe720

bool is_octal_char(char param_1)

{
  return (byte)(param_1 - 0x30U) < 8;
}




// Function: parse_escape_char @ 0xe730

int parse_escape_char(char param_1)

{
  int iVar1;
  
  iVar1 = 0;
  if ((byte)(param_1 + 0xa4U) < 0x1b) {
    iVar1 = (int)(char)(&DAT_0012da10)[(byte)(param_1 + 0xa4U)];
  }
  return iVar1;
}




// Function: mode_to_filetype @ 0xe750

char * mode_to_filetype(int param_1)

{
  char *pcVar1;
  
  pcVar1 = "f";
  if ((((param_1 != 0x8000) && (pcVar1 = "d", param_1 != 0x4000)) &&
      (pcVar1 = "l", param_1 != 0xa000)) &&
     (((pcVar1 = "s", param_1 != 0xc000 && (pcVar1 = "b", param_1 != 0x6000)) &&
      ((pcVar1 = "c", param_1 != 0x2000 && (pcVar1 = "U", param_1 == 0x1000)))))) {
    pcVar1 = "p";
  }
  return pcVar1;
}




// Function: get_format_specifer_length @ 0xe7d0

char get_format_specifer_length(char param_1)

{
  char *pcVar1;
  
  pcVar1 = strchr("abcdDfFgGhHiklmMnpPsStuUyYZ%",(int)param_1);
  if (pcVar1 != (char *)0x0) {
    return '\x01';
  }
  pcVar1 = strchr("ABCT",(int)param_1);
  return (pcVar1 != (char *)0x0) * '\x02';
}




// Function: get_format_flags_length @ 0xe820

long get_format_flags_length(long param_1)

{
  char *pcVar1;
  ushort **ppuVar2;
  byte bVar3;
  long lVar4;
  
  lVar4 = 0;
  do {
    lVar4 = lVar4 + 1;
    bVar3 = *(byte *)(param_1 + lVar4);
    if (bVar3 == 0) break;
    pcVar1 = strchr("-+ #",(int)(char)bVar3);
  } while (pcVar1 != (char *)0x0);
  ppuVar2 = __ctype_b_loc();
  while ((*(byte *)((long)*ppuVar2 + (ulong)bVar3 * 2 + 1) & 8) != 0) {
    lVar4 = lVar4 + 1;
    bVar3 = *(byte *)(param_1 + lVar4);
  }
  if (bVar3 == 0x2e) {
    do {
      lVar4 = lVar4 + 1;
    } while ((*(byte *)((long)*ppuVar2 + (ulong)*(byte *)(param_1 + lVar4) * 2 + 1) & 8) != 0);
  }
  return lVar4;
}




// Function: checked_fprintf @ 0xe8c0

void checked_fprintf(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8,
                 long param_9,undefined8 param_10,undefined8 param_11,undefined8 param_12,
                 undefined8 param_13,undefined8 param_14)

{
  char in_AL;
  int iVar1;
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
  local_d0 = &stack0x00000008;
  local_d8 = 0x10;
  local_c8 = local_b8;
  local_d4 = 0x30;
  local_a8 = param_11;
  local_a0 = param_12;
  local_98 = param_13;
  local_90 = param_14;
  iVar1 = __vfprintf_chk(*(undefined8 *)(param_9 + 8),1,param_10,&local_d8);
  if (iVar1 < 0) {
    piVar2 = __errno_location();
    FUN_0010e340(*piVar2,*(undefined8 *)(param_9 + 0x10));
  }
  if (local_c0 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: checked_fflush @ 0xe9a0

void checked_fflush(long param_1)

{
  int iVar1;
  int *piVar2;
  
  iVar1 = FUN_00117bc0(*(undefined8 *)(param_1 + 8));
  if (iVar1 == 0) {
    return;
  }
  piVar2 = __errno_location();
  FUN_0010e340(*piVar2,*(undefined8 *)(param_1 + 0x10));
  return;
}




// Function: checked_print_quoted @ 0xe9d0

void checked_print_quoted(long param_1,undefined8 param_2,undefined8 param_3)

{
  int iVar1;
  int *piVar2;
  
  iVar1 = FUN_00116960(*(undefined8 *)(param_1 + 8),*(undefined8 *)(param_1 + 0x20),
                       *(undefined1 *)(param_1 + 0x18),param_2,param_3);
  if (-1 < iVar1) {
    return;
  }
  piVar2 = __errno_location();
  FUN_0010e340(*piVar2,*(undefined8 *)(param_1 + 0x10));
  return;
}




// Function: do_time_format @ 0xeb50

long do_time_format(char *param_1,tm *param_2,void *param_3,size_t param_4)

{
  byte *__src;
  size_t sVar1;
  int iVar2;
  undefined4 uVar3;
  long lVar4;
  char cVar5;
  size_t sVar6;
  char *__format;
  char *pcVar7;
  ushort **ppuVar8;
  long lVar9;
  long in_FS_OFFSET;
  long local_88;
  long local_80;
  tm local_78;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  sVar6 = strlen(param_1);
  __format = (char *)FUN_00126990(sVar6 + 2);
  *__format = '_';
  sVar6 = strlen(param_1);
  memcpy(__format + 1,param_1,sVar6 + 1);
  iVar2 = param_2->tm_sec;
  local_78.tm_zone = param_2->tm_zone;
  uVar3 = param_2->tm_min;
  local_78.tm_hour = param_2->tm_hour;
  local_78.tm_mday = param_2->tm_mday;
  local_78.tm_mon = param_2->tm_mon;
  local_78.tm_year = param_2->tm_year;
  local_78.tm_wday = param_2->tm_wday;
  local_78.tm_yday = param_2->tm_yday;
  local_78.tm_isdst = param_2->tm_isdst;
  local_78._36_4_ = *(undefined4 *)&param_2->field_0x24;
  local_78.tm_gmtoff = param_2->tm_gmtoff;
  local_78.tm_sec = iVar2 + 0xb;
  if (10 < iVar2) {
    local_78.tm_sec = iVar2 + -0xb;
  }
  local_78.tm_min = uVar3;
  if (DAT_0013eb08 == (char *)0x0) {
    DAT_0013eb00 = 1;
    DAT_0013eb08 = (char *)FUN_00126990(1);
  }
  while ((pcVar7 = DAT_0013eb08, sVar6 = strftime(DAT_0013eb08,DAT_0013eb00,__format,param_2),
         sVar6 == 0 || (DAT_0013eb00 <= sVar6))) {
    DAT_0013eb08 = (char *)FUN_00126ad0(pcVar7,&DAT_0013eb00,1);
  }
  sVar1 = sVar6 + 1 + param_4;
  local_88 = 0;
  local_80 = 0;
  DAT_0013eb08 = (char *)FUN_001269e0(pcVar7,sVar1);
  DAT_0013eb00 = sVar1;
  pcVar7 = (char *)FUN_00126990(sVar1);
  strftime(pcVar7,DAT_0013eb00,__format,&local_78);
  lVar4 = (long)DAT_0013eb08;
  cVar5 = FUN_0010ea60(DAT_0013eb08,pcVar7,&local_88,&local_80);
  if ((cVar5 != '\0') && (local_80 == 2)) {
    ppuVar8 = __ctype_b_loc();
    lVar9 = local_88 + 2;
    __src = (byte *)(lVar4 + lVar9);
    if ((*(byte *)((long)*ppuVar8 + (ulong)*__src * 2 + 1) & 8) == 0) {
      sVar6 = (sVar6 - lVar9) + 1;
      if (lVar9 + param_4 + sVar6 != sVar1) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("end_of_seconds + ns_size + suffix_len == final_len","print.c",0x225,
                      "do_time_format");
      }
      memmove((void *)(lVar4 + lVar9 + param_4),__src,sVar6);
      memcpy(__src,param_3,param_4);
    }
  }
  free(__format);
  free(pcVar7);
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar4 + 1;
}




// Function: format_date @ 0xeda0

char * format_date(long param_1,undefined8 param_2,int param_3)

{
  bool bVar1;
  undefined8 uVar2;
  bool bVar3;
  int iVar4;
  char *__s;
  size_t sVar5;
  tm *ptVar6;
  long lVar7;
  ulong uVar8;
  long in_FS_OFFSET;
  long local_78;
  undefined8 local_70;
  undefined8 local_64;
  undefined4 local_5c;
  char local_58 [40];
  long local_30;
  
  uVar2 = local_64;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_78 = param_1;
  local_70 = param_2;
  if (param_3 == 0x2b) {
    local_64 = 0x64252d6d252d5925;
    local_5c = 0x54252b;
    iVar4 = __snprintf_chk(local_58,0x20,1,0x20,".%09ld0",param_2);
    bVar3 = true;
LAB_0010ef10:
    bVar1 = bVar3;
    ptVar6 = localtime(&local_78);
    if ((ptVar6 != (tm *)0x0) &&
       (__s = (char *)FUN_0010eb50(&local_64,ptVar6,local_58,(long)iVar4), __s != (char *)0x0))
    goto LAB_0010eeb3;
  }
  else {
    local_64._3_5_ = SUB85(uVar2,3);
    local_64._0_3_ = (uint3)CONCAT11((char)param_3,0x25);
    if ((param_3 - 0x40U < 0x19) && ((0x1180001UL >> ((ulong)(param_3 - 0x40U) & 0x3f) & 1) != 0)) {
      bVar3 = true;
      bVar1 = true;
      iVar4 = __snprintf_chk(local_58,0x20,1,0x20,".%09ld0",param_2);
    }
    else {
      local_58[0] = '\0';
      bVar3 = false;
      bVar1 = false;
      iVar4 = 0;
    }
    if (param_3 != 0x40) goto LAB_0010ef10;
  }
  lVar7 = -local_78;
  if (-local_78 < 0) {
    lVar7 = local_78;
  }
  __s = (char *)FUN_0011b800(lVar7,&DAT_0013eb21,0,1,1);
  if (__s < &DAT_0013eb21) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("p > buf","print.c",0x2b9,"format_date");
  }
  if ((char *)0x13ee36 < __s) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("p < (buf + (sizeof buf))","print.c",0x2ba,"format_date");
  }
  if (local_78 < 0) {
    __s[-1] = '-';
    __s = __s + -1;
  }
  if (bVar1) {
    sVar5 = strlen(__s);
    if ((char *)0x316 < __s + (sVar5 - 0x13eb20)) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("sizeof buf > used","print.c",0x2c7,"format_date");
    }
    uVar8 = 0x316 - (long)(__s + (sVar5 - 0x13eb20));
    sVar5 = strlen(local_58);
    if (uVar8 <= sVar5) {
                    /* WARNING: Subroutine does not return */
      error(0,0,"charsprinted=%ld but remaining=%lu: ns_buf=%s",(long)iVar4,uVar8,local_58);
    }
    strcat(__s,local_58);
  }
LAB_0010eeb3:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return __s;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: ctime_format @ 0xf050

undefined * ctime_format(time_t param_1,undefined8 param_2)

{
  int iVar1;
  int iVar2;
  int iVar3;
  tm *ptVar4;
  undefined *puVar5;
  time_t local_18;
  undefined8 local_10;
  
  local_18 = param_1;
  local_10 = param_2;
  ptVar4 = localtime(&local_18);
  if (ptVar4 == (tm *)0x0) {
    puVar5 = (undefined *)FUN_0010eda0(local_18,local_10,0x40);
  }
  else {
    iVar3 = ptVar4->tm_wday;
    if (iVar3 < 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("ptm->tm_wday >= 0","print.c",0x2ed,"ctime_format");
    }
    if (6 < iVar3) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("ptm->tm_wday < 7","print.c",0x2ee,"ctime_format");
    }
    iVar1 = ptVar4->tm_mon;
    if (iVar1 < 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("ptm->tm_mon >= 0","print.c",0x2ef,"ctime_format");
    }
    if (0xb < iVar1) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("ptm->tm_mon < 12","print.c",0x2f0,"ctime_format");
    }
    iVar2 = ptVar4->tm_hour;
    if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("ptm->tm_hour >= 0","print.c",0x2f1,"ctime_format");
    }
    if (0x17 < iVar2) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("ptm->tm_hour < 24","print.c",0x2f2,"ctime_format");
    }
    if (0x3b < ptVar4->tm_min) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("ptm->tm_min < 60","print.c",0x2f3,"ctime_format");
    }
    if (0x3d < ptVar4->tm_sec) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("ptm->tm_sec <= 61","print.c",0x2f4,"ctime_format");
    }
    iVar3 = __snprintf_chk(&DAT_0013e700,0x400,1,0x400,"%3s %3s %2d %02d:%02d:%02d.%09ld0 %04d",
                           (&PTR_DAT_0013c280)[iVar3],(&PTR_DAT_0013c220)[iVar1],ptVar4->tm_mday,
                           iVar2,ptVar4->tm_min,ptVar4->tm_sec,local_10,ptVar4->tm_year + 0x76c);
    if (0x3ff < iVar3) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("nout < TIME_BUF_LEN","print.c",0x302,"ctime_format");
    }
    puVar5 = &DAT_0013e700;
  }
  return puVar5;
}




// Function: do_fprintf @ 0xf310

void do_fprintf(undefined8 param_1,int *param_2,char *param_3,undefined8 *param_4)

{
  int iVar1;
  undefined8 uVar2;
  char *pcVar3;
  passwd *ppVar4;
  size_t sVar5;
  char *pcVar6;
  undefined8 uVar7;
  int *piVar8;
  uint uVar9;
  long in_FS_OFFSET;
  undefined1 auVar10 [16];
  undefined8 local_360;
  stat local_358;
  undefined1 local_2c8 [10];
  undefined1 local_2be;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = *param_2;
  if (iVar1 == 1) {
    FUN_0010ea10(*(undefined8 *)(param_2 + 2),(long)param_2[4],param_1);
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      FUN_0010e9a0(param_1);
      return;
    }
    goto LAB_0010f55e;
  }
  if (iVar1 != 2) {
    if (iVar1 == 0) goto switchD_0010f412_caseD_25;
    goto switchD_0010f412_caseD_1;
  }
  switch((char)param_2[1]) {
  case '\0':
    uVar2 = dcgettext(0,"error: %s at end of format string",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar2,&DAT_0012d32d);
  default:
    goto switchD_0010f412_caseD_1;
  case '%':
switchD_0010f412_caseD_25:
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      FUN_0010ea10(*(undefined8 *)(param_2 + 2),(long)param_2[4],param_1);
      return;
    }
    goto LAB_0010f55e;
  case 'D':
    uVar2 = FUN_0011b800(*param_4,local_2c8,0,1,1);
    break;
  case 'F':
    param_3 = (char *)FUN_001104c0(param_4,param_3);
    if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) goto LAB_0010f55e;
    goto LAB_0010f4a3;
  case 'G':
    goto switchD_0010f412_caseD_47;
  case 'H':
    pcVar3 = (char *)FUN_00126990((long)(DAT_0013e5fc + 1));
    sVar5 = (size_t)DAT_0013e5fc;
    memcpy(pcVar3,param_3,sVar5);
    pcVar3[sVar5] = '\0';
    FUN_0010e8c0(param_1,*(undefined8 *)(param_2 + 2),pcVar3);
    if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) goto LAB_0010f55e;
    goto LAB_0010f631;
  case 'M':
    FUN_00117d90(param_4,local_2c8);
    local_2be = 0;
    FUN_0010e8c0(param_1,*(undefined8 *)(param_2 + 2),local_2c8);
    goto switchD_0010f412_caseD_1;
  case 'P':
    pcVar3 = "";
    if (0 < (int)DAT_0013e5e0) {
      pcVar3 = param_3 + DAT_0013e5fc + (param_3[DAT_0013e5fc] == '/');
    }
    param_3 = pcVar3;
    if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) goto LAB_0010f55e;
LAB_0010f4a3:
    uVar2 = *(undefined8 *)(param_2 + 2);
LAB_0010f4a7:
    FUN_0010e9d0(param_1,uVar2,param_3);
    return;
  case 'S':
    FUN_0010f2c0(param_4[6],param_4[8]);
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      FUN_0010e8c0(param_1,*(undefined8 *)(param_2 + 2));
      return;
    }
    goto LAB_0010f55e;
  case 'U':
    goto switchD_0010f412_caseD_55;
  case 'Y':
    uVar9 = *(uint *)(param_4 + 3) & 0xf000;
    if (uVar9 != 0xa000) goto LAB_0010f571;
    iVar1 = fstatat(DAT_0013e5f8,DAT_0013e5f0,&local_358,0);
    if (iVar1 == 0) {
      uVar2 = FUN_0010e750(local_358.st_mode & 0xf000);
      break;
    }
    piVar8 = __errno_location();
    uVar2 = *(undefined8 *)(param_2 + 2);
    iVar1 = *piVar8;
    if ((iVar1 == 2) || (iVar1 == 0x14)) {
      FUN_0010e8c0(param_1,uVar2,"N");
    }
    else {
      if (iVar1 != 0x28) {
        FUN_0010e8c0(param_1,uVar2,&DAT_0012e86f);
        uVar2 = FUN_0010dfb0(0,param_3);
                    /* WARNING: Subroutine does not return */
        error(0,*piVar8,&DAT_0013155a,uVar2);
      }
      FUN_0010e8c0(param_1,uVar2,&DAT_0012c66c);
    }
    goto switchD_0010f412_caseD_1;
  case 'Z':
    iVar1 = (*DAT_0013e680)(DAT_0013e5f8,DAT_0013e5f0,&local_360);
    if (iVar1 < 0) {
      FUN_0010e8c0(param_1,*(undefined8 *)(param_2 + 2),"");
      uVar2 = FUN_0010dfb0(0,param_3);
      uVar7 = dcgettext(0,"getfilecon failed: %s",5);
      piVar8 = __errno_location();
                    /* WARNING: Subroutine does not return */
      error(0,*piVar8,uVar7,uVar2);
    }
    FUN_0010e8c0(param_1,*(undefined8 *)(param_2 + 2),local_360);
    freecon(local_360);
    goto switchD_0010f412_caseD_1;
  case 'a':
    auVar10 = FUN_00125c40(param_4);
    goto LAB_0010f690;
  case 'b':
    uVar2 = FUN_0011b800(param_4[8],local_2c8,0,0x200,0x200);
    break;
  case 'c':
    auVar10 = FUN_00125c50(param_4);
    goto LAB_0010f690;
  case 'd':
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      uVar2 = *(undefined8 *)(param_2 + 2);
      uVar9 = DAT_0013e5e0;
LAB_0010f74d:
      FUN_0010e8c0(param_1,uVar2,uVar9);
      return;
    }
    goto LAB_0010f55e;
  case 'f':
    pcVar3 = (char *)FUN_001176f0(param_3);
LAB_0010f84e:
    FUN_0010e9d0(param_1,*(undefined8 *)(param_2 + 2),pcVar3);
    goto LAB_0010f78e;
  case 'g':
    ppVar4 = (passwd *)getgrgid(*(__gid_t *)(param_4 + 4));
    if (ppVar4 != (passwd *)0x0) {
LAB_0010f659:
      *(undefined1 *)(*(long *)(param_2 + 2) + (long)param_2[4]) = 0x73;
      if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) goto LAB_0010f55e;
      uVar2 = *(undefined8 *)(param_2 + 2);
      pcVar3 = ppVar4->pw_name;
      goto LAB_0010f590;
    }
    goto switchD_0010f412_caseD_47;
  case 'h':
    pcVar3 = (char *)FUN_00126d30(param_3);
    sVar5 = strlen(pcVar3);
    for (pcVar6 = pcVar3 + (sVar5 - 1); pcVar3 <= pcVar6; pcVar6 = pcVar6 + -1) {
      if (*pcVar6 != '/') {
        if ((pcVar3 < pcVar6) && (pcVar6[1] == '/')) {
          pcVar6[1] = '\0';
        }
        break;
      }
    }
    pcVar6 = strrchr(pcVar3,0x2f);
    if (pcVar6 != (char *)0x0) {
      *pcVar6 = '\0';
      goto LAB_0010f84e;
    }
    FUN_0010e9d0(param_1,*(undefined8 *)(param_2 + 2),&DAT_0012f399);
    goto LAB_0010f78e;
  case 'i':
    uVar2 = FUN_0011b800(param_4[1],local_2c8,0,1,1);
    break;
  case 'k':
    uVar2 = FUN_0011b800(param_4[8],local_2c8,0,0x200,0x400);
    break;
  case 'l':
    if ((*(uint *)(param_4 + 3) & 0xf000) == 0xa000) {
      pcVar3 = (char *)FUN_00117190(DAT_0013e5f8,DAT_0013e5f0);
      if (pcVar3 != (char *)0x0) goto LAB_0010f84e;
      piVar8 = __errno_location();
      FUN_0010e060(*piVar8,param_3);
      DAT_0013e604 = 1;
    }
    pcVar3 = (char *)0x0;
    FUN_0010e9d0(param_1,*(undefined8 *)(param_2 + 2),"");
LAB_0010f78e:
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
LAB_0010f631:
      free(pcVar3);
      return;
    }
    goto LAB_0010f55e;
  case 'm':
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      uVar2 = *(undefined8 *)(param_2 + 2);
      uVar9 = *(uint *)(param_4 + 3) & 0xfff;
      goto LAB_0010f74d;
    }
    goto LAB_0010f55e;
  case 'n':
    uVar2 = FUN_0011b800(param_4[2],local_2c8,0,1,1);
    break;
  case 'p':
    if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) goto LAB_0010f55e;
    uVar2 = *(undefined8 *)(param_2 + 2);
    goto LAB_0010f4a7;
  case 's':
    uVar2 = FUN_0011b800(param_4[6],local_2c8,0,1,1);
    break;
  case 't':
    auVar10 = FUN_00125c60(param_4);
LAB_0010f690:
    pcVar3 = (char *)FUN_0010f050(auVar10._0_8_,auVar10._8_8_);
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
LAB_0010f58c:
      uVar2 = *(undefined8 *)(param_2 + 2);
LAB_0010f590:
      FUN_0010e8c0(param_1,uVar2,pcVar3);
      return;
    }
    goto LAB_0010f55e;
  case 'u':
    ppVar4 = getpwuid(*(__uid_t *)((long)param_4 + 0x1c));
    if (ppVar4 != (passwd *)0x0) goto LAB_0010f659;
    goto switchD_0010f412_caseD_55;
  case 'y':
    uVar9 = *(uint *)(param_4 + 3) & 0xf000;
LAB_0010f571:
    pcVar3 = (char *)FUN_0010e750(uVar9);
    if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) goto LAB_0010f55e;
    goto LAB_0010f58c;
  }
LAB_0010f438:
  FUN_0010e8c0(param_1,*(undefined8 *)(param_2 + 2),uVar2);
switchD_0010f412_caseD_1:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
LAB_0010f55e:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
switchD_0010f412_caseD_55:
  uVar2 = FUN_0011b800(*(undefined4 *)((long)param_4 + 0x1c),local_2c8,0,1,1);
  goto LAB_0010f438;
switchD_0010f412_caseD_47:
  uVar2 = FUN_0011b800(*(undefined4 *)(param_4 + 4),local_2c8,0,1,1);
  goto LAB_0010f438;
}




// Function: pred_fprintf @ 0xfad0

undefined8 pred_fprintf(undefined8 param_1,undefined8 param_2,long param_3)

{
  char cVar1;
  char cVar2;
  undefined8 uVar3;
  int *piVar4;
  undefined1 auVar5 [16];
  
  piVar4 = *(int **)(param_3 + 0x38);
  if (piVar4 != (int *)0x0) {
    param_3 = param_3 + 0x38;
    do {
      while ((*piVar4 != 2 || (cVar1 = *(char *)((long)piVar4 + 5), cVar1 == '\0'))) {
        FUN_0010f310(param_3,piVar4,param_1,param_2);
        piVar4 = *(int **)(piVar4 + 6);
        if (piVar4 == (int *)0x0) {
          return 1;
        }
      }
      cVar2 = (char)piVar4[1];
      if (cVar2 == 'C') {
        auVar5 = FUN_00125c50(param_2);
LAB_0010fb35:
        uVar3 = FUN_0010eda0(auVar5._0_8_,auVar5._8_8_,(int)cVar1);
        FUN_0010e8c0(param_3,*(undefined8 *)(piVar4 + 2),uVar3);
      }
      else {
        if ('C' < cVar2) {
          if (cVar2 != 'T') {
LAB_0010fbf3:
                    /* WARNING: Subroutine does not return */
            __assert_fail("0","print.c",0x518,"pred_fprintf");
          }
          auVar5 = FUN_00125c60(param_2);
          goto LAB_0010fb35;
        }
        if (cVar2 == 'A') {
          auVar5 = FUN_00125c40(param_2);
          goto LAB_0010fb35;
        }
        if (cVar2 != 'B') goto LAB_0010fbf3;
        auVar5 = FUN_00125c70(param_2);
        if ((cVar1 == '@') || (-1 < auVar5._8_8_)) goto LAB_0010fb35;
        FUN_0010e8c0(param_3,*(undefined8 *)(piVar4 + 2),"");
      }
      piVar4 = *(int **)(piVar4 + 6);
    } while (piVar4 != (int *)0x0);
  }
  return 1;
}




// Function: make_segment @ 0xfc20

long make_segment(long *param_1,char *param_2,uint param_3,uint param_4,char param_5,char param_6,
                 long param_7)

{
  long lVar1;
  uint uVar2;
  uint *puVar3;
  char *pcVar4;
  size_t __n;
  char local_39;
  
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
  __n = (size_t)(int)param_3;
  puVar3 = (uint *)FUN_00126990(0x20);
  *param_1 = (long)puVar3;
  *puVar3 = param_4;
  *(char *)(puVar3 + 1) = param_5;
  *(char *)((long)puVar3 + 5) = param_6;
  puVar3[6] = 0;
  puVar3[7] = 0;
  puVar3[4] = param_3;
  pcVar4 = (char *)FUN_00126990(__n + 2);
  *(char **)(puVar3 + 2) = pcVar4;
  pcVar4 = strncpy(pcVar4,param_2,__n);
  pcVar4 = pcVar4 + __n;
  if (param_4 < 2) {
    if (param_5 != '\0') {
                    /* WARNING: Subroutine does not return */
      __assert_fail("0 == format_char","print.c",0x61,"make_segment");
    }
    if (param_6 == '\0') {
      lVar1 = *param_1;
      *pcVar4 = '\0';
      return lVar1 + 0x18;
    }
                    /* WARNING: Subroutine does not return */
    __assert_fail("0 == aux_format_char","print.c",0x62,"make_segment");
  }
  if (param_4 != 2) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("kind == KIND_FORMAT","print.c",0x69,"make_segment");
  }
  switch(param_5) {
  case '%':
  case 'd':
    local_39 = param_5;
    goto LAB_0010fd69;
  default:
    uVar2 = 0;
    break;
  case 'A':
  case 'B':
  case 'C':
  case 'D':
  case 'F':
  case 'G':
  case 'M':
  case 'T':
  case 'U':
  case 'a':
  case 'b':
  case 'c':
  case 'g':
  case 'k':
  case 'n':
  case 's':
  case 't':
  case 'u':
    *pcVar4 = 's';
    pcVar4 = pcVar4 + 1;
    *(undefined1 *)(param_7 + 0x1a) = 1;
    uVar2 = 3;
    break;
  case 'H':
  case 'P':
  case 'f':
  case 'h':
  case 'p':
    local_39 = 's';
LAB_0010fd69:
    *pcVar4 = local_39;
    pcVar4[1] = '\0';
    goto LAB_0010fcf2;
  case 'S':
    *pcVar4 = 'g';
    pcVar4 = pcVar4 + 1;
    *(undefined1 *)(param_7 + 0x1a) = 1;
    uVar2 = 3;
    break;
  case 'Y':
    *pcVar4 = 's';
    pcVar4 = pcVar4 + 1;
    *(undefined1 *)(param_7 + 0x1a) = 1;
    uVar2 = 2;
    break;
  case 'Z':
    *pcVar4 = 's';
    uVar2 = 5;
    pcVar4 = pcVar4 + 1;
    break;
  case 'i':
    *pcVar4 = 's';
    pcVar4 = pcVar4 + 1;
    *(undefined1 *)(param_7 + 0x1c) = 1;
    uVar2 = 1;
    break;
  case 'l':
    *pcVar4 = 's';
    pcVar4 = pcVar4 + 1;
    *(undefined1 *)(param_7 + 0x1a) = 1;
    uVar2 = 4;
    break;
  case 'm':
    *pcVar4 = 'o';
    pcVar4 = pcVar4 + 1;
    *(undefined1 *)(param_7 + 0x1a) = 1;
    uVar2 = 3;
    break;
  case 'y':
    *pcVar4 = 's';
    pcVar4 = pcVar4 + 1;
    *(undefined1 *)(param_7 + 0x1b) = 1;
    uVar2 = 2;
  }
  *pcVar4 = '\0';
  if (*(uint *)(param_7 + 0x20) < uVar2) {
    *(uint *)(param_7 + 0x20) = uVar2;
  }
LAB_0010fcf2:
  return *param_1 + 0x18;
}




// Function: insert_fprintf @ 0xfef0

undefined8 insert_fprintf(undefined8 *param_1,undefined8 param_2,char *param_3)

{
  char cVar1;
  long lVar2;
  long lVar3;
  char *pcVar4;
  undefined8 uVar5;
  char cVar6;
  char *pcVar7;
  int iVar8;
  int iVar9;
  char *pcVar10;
  int iVar11;
  long lVar12;
  long in_FS_OFFSET;
  long lStack_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  lVar2 = FUN_0010d450(param_2,FUN_0010fad0);
  uVar5 = param_1[1];
  *(undefined8 *)(lVar2 + 0x38) = *param_1;
  *(undefined8 *)(lVar2 + 0x40) = uVar5;
  uVar5 = param_1[3];
  *(undefined8 *)(lVar2 + 0x48) = param_1[2];
  *(undefined8 *)(lVar2 + 0x50) = uVar5;
  uVar5 = param_1[4];
  *(undefined4 *)(lVar2 + 0x18) = 0x101;
  *(undefined8 *)(lVar2 + 0x58) = uVar5;
  *(undefined4 *)(lVar2 + 0x20) = 0;
  *(undefined8 *)(lVar2 + 0x38) = 0;
  cVar1 = *param_3;
  if (cVar1 != '\0') {
    lVar3 = lVar2 + 0x38;
    pcVar4 = param_3;
    do {
      while( true ) {
        cVar6 = pcVar4[1];
        iVar9 = (int)param_3;
        if (cVar1 != '\\') break;
        if (cVar6 == 'c') {
          FUN_0010fc20(lVar3,param_3,(int)pcVar4 - iVar9,1,0,0,lVar2);
          if ((*(char *)(lVar2 + 0x1a) != '\0') && (*(uint *)(lVar2 + 0x20) < 3)) {
            *(undefined4 *)(lVar2 + 0x20) = 3;
          }
          goto LAB_0011001a;
        }
        if (cVar6 == '\0') {
          uVar5 = dcgettext(0,"warning: escape `\\\' followed by nothing at all",5);
                    /* WARNING: Subroutine does not return */
          error(0,0,uVar5);
        }
        iVar11 = (int)cVar6;
        pcVar7 = pcVar4 + 1;
        cVar1 = FUN_0010e720(iVar11);
        if (cVar1 == '\0') {
          cVar1 = FUN_0010e730(iVar11);
          if (cVar1 == '\0') {
            uVar5 = dcgettext(0,"warning: unrecognized escape `\\%c\'",5);
                    /* WARNING: Subroutine does not return */
            error(0,0,uVar5,iVar11);
          }
          *pcVar4 = cVar1;
          lVar12 = 2;
        }
        else {
          lStack_48 = 0;
          cVar1 = FUN_0010f270(pcVar7,&lStack_48);
          *pcVar4 = cVar1;
          lVar12 = lStack_48 + 2;
          pcVar7 = pcVar4 + lStack_48 + 1;
        }
        pcVar10 = pcVar4 + lVar12;
        lVar3 = FUN_0010fc20(lVar3,param_3,((int)pcVar4 - iVar9) + 1,0,0,0,lVar2);
        cVar1 = pcVar7[1];
        param_3 = pcVar10;
        pcVar4 = pcVar10;
        pcVar7 = pcVar10;
        if (cVar1 == '\0') goto LAB_00110011;
      }
      if (cVar1 == '%') {
        if (cVar6 == '\0') {
          uVar5 = dcgettext(0,"error: %s at end of format string",5);
                    /* WARNING: Subroutine does not return */
          error(1,0,uVar5,pcVar4);
        }
        pcVar7 = pcVar4 + 1;
        if (cVar6 != '%') {
          lVar12 = FUN_0010e820(pcVar4);
          pcVar7 = pcVar4 + lVar12;
          cVar6 = *pcVar7;
        }
        iVar11 = (int)cVar6;
        lVar12 = FUN_0010e7d0(iVar11);
        if (lVar12 == 0) {
          pcVar4 = strchr("{[(",iVar11);
          if (pcVar4 != (char *)0x0) {
LAB_00110280:
            uVar5 = dcgettext(0,"error: the format directive `%%%c\' is reserved for future use",5);
                    /* WARNING: Subroutine does not return */
            error(1,0,uVar5,iVar11);
          }
        }
        else {
          pcVar4 = pcVar7 + lVar12 + -1;
          if (*pcVar4 != '\0') {
            iVar8 = 0;
            if (lVar12 == 2) {
              iVar8 = (int)pcVar7[1];
            }
            pcVar10 = pcVar7 + lVar12;
            lVar3 = FUN_0010fc20(lVar3,param_3,(int)pcVar7 - iVar9,2,iVar11,iVar8,lVar2);
            pcVar7 = pcVar10;
            goto LAB_0010ff7f;
          }
          pcVar4 = strchr("{[(",iVar11);
          if (pcVar4 != (char *)0x0) goto LAB_00110280;
          if ((lVar12 == 2) &&
             (pcVar4 = "warning: format directive `%%%c\' should be followed by another character",
             pcVar7[1] == '\0')) goto LAB_001100fc;
        }
        pcVar4 = "warning: unrecognized format directive `%%%c\'";
LAB_001100fc:
        uVar5 = dcgettext(0,pcVar4,5);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar5,iVar11);
      }
      pcVar10 = param_3;
      pcVar7 = pcVar4 + 1;
LAB_0010ff7f:
      cVar1 = pcVar4[1];
      param_3 = pcVar10;
      pcVar4 = pcVar7;
    } while (cVar1 != '\0');
LAB_00110011:
    if (pcVar10 < pcVar7) {
      FUN_0010fc20(lVar3,pcVar10,(int)pcVar7 - (int)pcVar10,0,0,0,lVar2);
    }
  }
LAB_0011001a:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return 1;
}




// Function: free_file_system_list @ 0x102e0

void free_file_system_list(long param_1)

{
  long lVar1;
  
  if (param_1 == 0) {
    return;
  }
  do {
    lVar1 = *(long *)(param_1 + 0x30);
    FUN_0011dc40(param_1);
    param_1 = lVar1;
  } while (lVar1 != 0);
  return;
}




// Function: get_file_system_list @ 0x10310

void get_file_system_list(char param_1)

{
  if (DAT_0013ee40 != 0) {
    if ((DAT_0013ee38 == '\x01') || (param_1 == '\0')) {
      return;
    }
    FUN_001102e0(DAT_0013ee40);
    DAT_0013ee40 = 0;
  }
  DAT_0013ee40 = FUN_0011dc90(param_1);
  DAT_0013ee38 = param_1;
  return;
}




// Function: set_fstype_devno @ 0x10360

undefined8 set_fstype_devno(long param_1)

{
  undefined8 uVar1;
  long in_FS_OFFSET;
  undefined8 local_b8 [19];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = 0;
  if (*(long *)(param_1 + 0x20) == -1) {
    FUN_0010d860(local_b8);
    uVar1 = (*DAT_0013e670)(*(undefined8 *)(param_1 + 8),local_b8);
    if ((int)uVar1 == 0) {
      *(undefined8 *)(param_1 + 0x20) = local_b8[0];
    }
    else {
      uVar1 = 0xffffffff;
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: filesystem_type @ 0x104c0

void filesystem_type(long *param_1)

{
  if (DAT_0013ee58 != (void *)0x0) {
    if ((DAT_0013ee50 != '\0') && (*param_1 == DAT_0013ee48)) {
      return;
    }
    free(DAT_0013ee58);
  }
  DAT_0013ee48 = *param_1;
  DAT_0013ee58 = (void *)FUN_001103e0(param_1);
  return;
}




// Function: is_used_fs_type @ 0x10510

undefined8 is_used_fs_type(char *param_1)

{
  int iVar1;
  long lVar2;
  
  iVar1 = strcmp("afs",param_1);
  if (iVar1 != 0) {
    lVar2 = FUN_00110310(0);
    if (lVar2 != 0) {
      while( true ) {
        iVar1 = strcmp(param_1,*(char **)(lVar2 + 0x18));
        if (iVar1 == 0) break;
        lVar2 = *(long *)(lVar2 + 0x30);
        if (lVar2 == 0) {
          return 0;
        }
      }
    }
  }
  return 1;
}




// Function: get_mounted_devices @ 0x10580

void * get_mounted_devices(long *param_1)

{
  long lVar1;
  int iVar2;
  long lVar3;
  void *pvVar4;
  long lVar5;
  long lVar6;
  void *__ptr;
  long in_FS_OFFSET;
  undefined8 local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_48 = 0;
  lVar3 = FUN_0011dc90(0);
  if (lVar3 == 0) {
    pvVar4 = (void *)0x0;
    FUN_001102e0(0);
  }
  else {
    lVar5 = 0;
    lVar6 = lVar3;
    __ptr = (void *)0x0;
    do {
      while( true ) {
        lVar1 = (lVar5 + 1) * 8;
        pvVar4 = (void *)FUN_00115710(__ptr,lVar1,&local_48);
        if (pvVar4 != (void *)0x0) break;
        pvVar4 = (void *)0x0;
        free(__ptr);
        lVar6 = *(long *)(lVar6 + 0x30);
        __ptr = pvVar4;
        if (lVar6 == 0) goto LAB_0011063a;
      }
      iVar2 = FUN_00110360(lVar6);
      if (iVar2 == 0) {
        *(undefined8 *)((long)pvVar4 + lVar1 + -8) = *(undefined8 *)(lVar6 + 0x20);
        lVar5 = lVar5 + 1;
      }
      lVar6 = *(long *)(lVar6 + 0x30);
      __ptr = pvVar4;
    } while (lVar6 != 0);
LAB_0011063a:
    FUN_001102e0(lVar3);
    if (pvVar4 != (void *)0x0) {
      *param_1 = lVar5;
    }
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return pvVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: get_noop @ 0x10690

undefined * get_noop(void)

{
  int iVar1;
  int *piVar2;
  int iVar3;
  undefined *puVar4;
  
  iVar3 = 0;
  piVar2 = &DAT_0013cd40;
  puVar4 = DAT_0013ee68;
  if (DAT_0013ee68 == (undefined *)0x0) {
    while( true ) {
      iVar3 = iVar3 + 1;
      puVar4 = (undefined *)0x0;
      if (*(long *)(piVar2 + 2) == 0) break;
      iVar1 = *piVar2;
      piVar2 = piVar2 + 8;
      if (iVar1 == 1) {
        DAT_0013ee68 = &DAT_0013cd20 + (long)iVar3 * 0x20;
        return &DAT_0013cd20 + (long)iVar3 * 0x20;
      }
    }
  }
  return puVar4;
}




// Function: should_issue_warnings @ 0x106f0

undefined1 should_issue_warnings(void)

{
  undefined1 uVar1;
  
  uVar1 = 0;
  if (DAT_0013e631 == '\0') {
    uVar1 = DAT_0013e630;
  }
  return uVar1;
}




// Function: estimate_file_age_success_rate @ 0x10710

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 estimate_file_age_success_rate(float param_1)

{
  undefined4 uVar1;
  
  if (DAT_0012ec74 <= param_1) {
    uVar1 = DAT_0012ec6c;
    if ((_DAT_0012c588 <= param_1) && (param_1 <= _DAT_0012ec78)) {
      uVar1 = DAT_0012ec70;
    }
    return uVar1;
  }
  return DAT_0012ec68;
}




// Function: estimate_timestamp_success_rate @ 0x10760

void estimate_timestamp_success_rate(long param_1)

{
  undefined1 auVar1 [16];
  
  auVar1._4_12_ = SUB1612((undefined1  [16])0x0,4);
  auVar1._0_4_ = (float)(int)((DAT_0013e648 - param_1) / 0x15180);
  FUN_00110710(auVar1._0_8_);
  return;
}




// Function: collect_arg_nonconst @ 0x10790

undefined8 collect_arg_nonconst(long param_1,int *param_2,long *param_3)

{
  int iVar1;
  long lVar2;
  
  if (param_1 != 0) {
    iVar1 = *param_2;
    lVar2 = *(long *)(param_1 + (long)iVar1 * 8);
    if (lVar2 != 0) {
      *param_3 = lVar2;
      *param_2 = iVar1 + 1;
      return 1;
    }
  }
  *param_3 = 0;
  return 0;
}




// Function: collect_arg @ 0x107d0

void collect_arg(undefined8 param_1,undefined8 param_2,undefined8 *param_3)

{
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00110790(param_1,param_2,&local_18);
  *param_3 = local_18;
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_files0_from @ 0x10810

void parse_files0_from(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_001107d0(param_2,param_3,&local_18);
  if (cVar1 != '\0') {
    DAT_0013e690 = local_18;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: get_comp_type @ 0x10860

undefined8 get_comp_type(undefined8 *param_1,undefined4 *param_2)

{
  char *pcVar1;
  
  pcVar1 = (char *)*param_1;
  if (*pcVar1 == '+') {
    *param_2 = 0;
    *param_1 = pcVar1 + 1;
    return 1;
  }
  if (*pcVar1 != '-') {
    *param_2 = 2;
    return 1;
  }
  *param_2 = 1;
  *param_1 = pcVar1 + 1;
  return 1;
}




// Function: found_parser @ 0x108b0

int * found_parser(long param_1,int *param_2)

{
  int iVar1;
  long lVar2;
  char cVar3;
  undefined8 uVar4;
  
  lVar2 = DAT_0013ee70;
  iVar1 = *param_2;
  if (iVar1 != 2) {
    if (iVar1 == 1) {
      param_2 = (int *)0x0;
    }
    else if (iVar1 == 0) {
      if (DAT_0013ee70 != 0) {
        cVar3 = FUN_001106f0();
        if (cVar3 != '\0') {
          uVar4 = dcgettext(0,
                            "warning: you have specified the global option %s after the argument %s, but global options are not positional, i.e., %s affects tests specified before it as well as those specified after it.  Please specify global options before other arguments."
                            ,5);
                    /* WARNING: Subroutine does not return */
          error(0,0,uVar4,param_1,lVar2,param_1);
        }
      }
    }
    else if (DAT_0013ee70 == 0) {
      DAT_0013ee70 = param_1;
    }
  }
  return param_2;
}




// Function: fallback_getfilecon @ 0x10950

ulong fallback_getfilecon(undefined4 param_1,undefined8 param_2,undefined8 param_3,uint param_4)

{
  int *piVar1;
  ulong uVar2;
  
  piVar1 = __errno_location();
  if ((*piVar1 != 2) && (*piVar1 != 0x14)) {
    return (ulong)param_4;
  }
  if (((byte)DAT_0013e660 & 2) != 0) {
    __fprintf_chk(stderr,1,
                  "fallback_getfilecon(): getfilecon(%s) failed; falling back on lgetfilecon()\n",
                  param_2);
  }
  uVar2 = FUN_001254c0(param_1,param_2,param_3);
  return uVar2;
}




// Function: optionh_getfilecon @ 0x109e0

undefined8 optionh_getfilecon(undefined4 param_1,undefined8 param_2,undefined8 param_3)

{
  int iVar1;
  undefined8 uVar2;
  
  if (DAT_0013e5e0 != 0) {
    uVar2 = FUN_001254c0();
    return uVar2;
  }
  iVar1 = FUN_001252a0();
  if (iVar1 == 0) {
    return 0;
  }
  uVar2 = FUN_00110950(param_1,param_2,param_3,iVar1);
  return uVar2;
}




// Function: optionl_getfilecon @ 0x10a40

undefined8 optionl_getfilecon(undefined4 param_1,undefined8 param_2,undefined8 param_3)

{
  int iVar1;
  undefined8 uVar2;
  
  iVar1 = FUN_001252a0();
  if (iVar1 == 0) {
    return 0;
  }
  uVar2 = FUN_00110950(param_1,param_2,param_3,iVar1);
  return uVar2;
}




// Function: parse_version @ 0x10a80

void parse_version(void)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  char *pcVar4;
  long in_FS_OFFSET;
  uint local_14;
  undefined8 local_10;
  
  local_10 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  FUN_00115e60(&DAT_0012b132);
  uVar3 = dcgettext(0,"Features enabled: ",5);
  __printf_chk(1,uVar3);
  __printf_chk(1,"D_TYPE ");
  pcVar4 = "enabled";
  if (DAT_0013e678 == '\0') {
    pcVar4 = "disabled";
  }
  __printf_chk(1,"O_NOFOLLOW(%s) ",pcVar4);
  __printf_chk(1,"LEAF_OPTIMISATION ");
  iVar2 = is_selinux_enabled();
  if (0 < iVar2) {
    __printf_chk(1,"SELINUX ");
  }
  local_14 = 0;
  cVar1 = FUN_001086d0(&local_14);
  if (cVar1 != '\0') {
    __printf_chk(1,&DAT_0012e7cb);
    if ((local_14 & 0x200) != 0) {
      __printf_chk(1,"FTS_CWDFD");
    }
    __printf_chk(1,&DAT_0012b6dc);
  }
  __printf_chk(1,"CBO(level=%d) ",DAT_0013e688);
  __printf_chk(1,"\n");
                    /* WARNING: Subroutine does not return */
  exit(0);
}




// Function: parse_true @ 0x10bf0

undefined8 parse_true(void)

{
  long lVar1;
  
  lVar1 = FUN_0010d4d0();
  *(undefined2 *)(lVar1 + 0x1a) = 0;
  *(undefined4 *)(lVar1 + 0x24) = 0x3f800000;
  return 1;
}




// Function: parse_quit @ 0x10c20

undefined8 parse_quit(void)

{
  long lVar1;
  
  lVar1 = FUN_0010d4d0();
  *(undefined4 *)(lVar1 + 0x18) = 1;
  *(undefined4 *)(lVar1 + 0x24) = 0x3f800000;
  return 1;
}




// Function: parse_prune @ 0x10c50

undefined8 parse_prune(void)

{
  long lVar1;
  
  lVar1 = FUN_0010d4d0();
  if (DAT_0013e620 == '\0') {
    *(undefined2 *)(lVar1 + 0x1a) = 0;
  }
  *(undefined2 *)(lVar1 + 0x18) = 1;
  return 1;
}




// Function: parse_nouser @ 0x10c80

undefined8 parse_nouser(void)

{
  long lVar1;
  
  lVar1 = FUN_0010d4d0();
  *(undefined4 *)(lVar1 + 0x24) = 0x3a83126f;
  return 1;
}




// Function: parse_accesscheck @ 0x10ca0

undefined8 parse_accesscheck(void)

{
  long *plVar1;
  undefined4 uVar2;
  
  plVar1 = (long *)FUN_0010d4d0();
  uVar2 = DAT_0012ec7c;
  *(undefined4 *)(plVar1 + 3) = 0;
  if ((code *)*plVar1 != FUN_00109640) {
    uVar2 = DAT_0012ec80;
  }
  *(undefined4 *)((long)plVar1 + 0x24) = uVar2;
  return 1;
}




// Function: parse_empty @ 0x10ce0

undefined8 parse_empty(void)

{
  long lVar1;
  
  lVar1 = FUN_0010d4d0();
  *(undefined4 *)(lVar1 + 0x24) = 0x3c23d70a;
  return 1;
}




// Function: parse_delete @ 0x10d00

undefined8 parse_delete(void)

{
  long lVar1;
  
  lVar1 = FUN_0010d4d0();
  DAT_0013e620 = 0;
  *(undefined4 *)(lVar1 + 0x18) = 0x101;
  *(undefined4 *)(lVar1 + 0x24) = 0x3f800000;
  return 1;
}




// Function: insert_type @ 0x10d30

undefined4 insert_type(undefined8 param_1,undefined8 param_2,undefined8 param_3,code *param_4)

{
  char cVar1;
  undefined4 uVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  long lVar10;
  long lVar11;
  undefined8 uVar12;
  char *pcVar13;
  long in_FS_OFFSET;
  float fVar14;
  char *local_48;
  long local_40;
  
  pcVar13 = "-xtype";
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_4 != FUN_001099e0) {
    pcVar13 = "-type";
  }
  uVar8 = FUN_001107d0(param_1,param_2,&local_48);
  if ((char)uVar8 != '\0') {
    if (*local_48 == '\0') {
      uVar12 = dcgettext(0,"Arguments to %s should contain at least one letter",5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar12,pcVar13);
    }
    lVar10 = FUN_0010d450(param_3,param_4,local_48);
    fVar7 = DAT_0012ec9c;
    fVar6 = DAT_0012ec98;
    fVar3 = DAT_0012ec84;
    uVar9 = 0;
    if (param_4 == FUN_001099e0) {
      uVar9 = uVar8;
    }
    *(undefined4 *)(lVar10 + 0x24) = 0;
    fVar5 = DAT_0012ec94;
    fVar4 = DAT_0012ec90;
    uVar2 = 0;
    if (param_4 != FUN_001099e0) {
      uVar2 = uVar8;
    }
    *(char *)(lVar10 + 0x1a) = (char)uVar9;
    *(char *)(lVar10 + 0x1b) = (char)uVar2;
    cVar1 = *local_48;
    while (cVar1 != '\0') {
      switch(cVar1) {
      case 'D':
        uVar12 = dcgettext(0,
                           "%s %c is not supported because Solaris doors are not supported on the platform find was compiled on."
                           ,5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar12,pcVar13,0x44);
      default:
        uVar12 = dcgettext(0,"Unknown argument to %s: %c",5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar12,pcVar13,(int)cVar1);
      case 'b':
        lVar11 = 0;
        fVar14 = fVar3;
        break;
      case 'c':
        lVar11 = 1;
        fVar14 = DAT_0012ec88;
        break;
      case 'd':
        lVar11 = 2;
        fVar14 = DAT_0012ec8c;
        break;
      case 'f':
        lVar11 = 3;
        fVar14 = fVar4;
        break;
      case 'l':
        lVar11 = 4;
        fVar14 = fVar5;
        break;
      case 'p':
        lVar11 = 5;
        fVar14 = fVar6;
        break;
      case 's':
        lVar11 = 6;
        fVar14 = fVar7;
      }
      if (*(char *)(lVar10 + 0x38 + lVar11) != '\0') {
        uVar12 = dcgettext(0,"Duplicate file type \'%c\' in the argument list to %s.",5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar12,(int)cVar1,pcVar13);
      }
      *(float *)(lVar10 + 0x24) = fVar14 + *(float *)(lVar10 + 0x24);
      *(undefined1 *)(lVar10 + 0x38 + lVar11) = 1;
      if (local_48[1] == '\0') break;
      if (local_48[1] != ',') {
        uVar12 = dcgettext(0,"Must separate multiple arguments to %s using: \',\'",5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar12,pcVar13);
      }
      cVar1 = local_48[2];
      local_48 = local_48 + 2;
      if (cVar1 == '\0') {
        uVar12 = dcgettext(0,
                           "Last file type in list argument to %s is missing, i.e., list is ending on: \',\'"
                           ,5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar12,pcVar13);
      }
    }
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar8;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_xtype @ 0x10fe0

void parse_xtype(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_00110d30(param_2,param_3,param_1,FUN_001099e0);
  return;
}




// Function: fnmatch_sanitycheck @ 0x11020

undefined8 fnmatch_sanitycheck(void)

{
  int iVar1;
  undefined8 uVar2;
  
  if (DAT_0013ee60 != '\0') {
    return 1;
  }
  iVar1 = fnmatch("foo","foo",0);
  if (iVar1 == 0) {
    iVar1 = fnmatch("Foo","foo",0);
    if (iVar1 != 0) {
      iVar1 = fnmatch("Foo","foo",0x10);
      if (iVar1 == 0) {
        DAT_0013ee60 = 1;
        return 1;
      }
    }
  }
  uVar2 = dcgettext(0,"sanity check of the fnmatch() library function failed.",5);
                    /* WARNING: Subroutine does not return */
  error(1,0,uVar2);
}




// Function: estimate_pattern_match_rate @ 0x110c0

undefined4 estimate_pattern_match_rate(char *param_1,int param_2)

{
  undefined4 uVar1;
  char *pcVar2;
  undefined4 uVar3;
  
  pcVar2 = strpbrk(param_1,"*?[");
  uVar1 = DAT_0012ec74;
  uVar3 = DAT_0012eca0;
  if ((pcVar2 == (char *)0x0) && (uVar3 = uVar1, param_2 != 0)) {
    pcVar2 = strpbrk(param_1,".");
    if (pcVar2 != (char *)0x0) {
      return DAT_0012eca0;
    }
  }
  return uVar3;
}




// Function: is_feasible_path_argument @ 0x11130

undefined8 is_feasible_path_argument(char *param_1,undefined1 param_2)

{
  char *pcVar1;
  undefined8 uVar2;
  
  pcVar1 = strrchr(param_1,0x2f);
  if ((pcVar1 != (char *)0x0) && (pcVar1[1] == '\0')) {
    uVar2 = FUN_0010b510(param_1,param_2);
    return uVar2;
  }
  return 1;
}




// Function: insert_path_check @ 0x11180

undefined4
insert_path_check(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
            undefined *param_5)

{
  char cVar1;
  undefined4 uVar2;
  long lVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  bool bVar5;
  undefined4 uVar6;
  undefined8 local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00111020();
  uVar2 = FUN_001107d0(param_2,param_3,&local_48);
  if ((char)uVar2 != '\0') {
    lVar3 = FUN_0010d450(param_1,param_5,local_48);
    *(undefined2 *)(lVar3 + 0x1a) = 0;
    *(undefined8 *)(lVar3 + 0x38) = local_48;
    uVar6 = FUN_001110c0(local_48,0);
    bVar5 = DAT_0013e631 == '\0';
    *(undefined4 *)(lVar3 + 0x24) = uVar6;
    if (bVar5) {
      cVar1 = FUN_00111130(local_48,param_5 == PTR_FUN_0013df80);
      if (cVar1 == '\0') {
        uVar4 = dcgettext(0,"warning: -%s %s will not match anything because it ends with /.",5);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar4,param_4,local_48);
      }
    }
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_wholename @ 0x11280

void parse_wholename(void)

{
  FUN_00111180();
  return;
}




// Function: parse_user @ 0x11300

undefined4 parse_user(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined4 uVar1;
  __uid_t _Var2;
  passwd *ppVar3;
  long lVar4;
  size_t sVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  long in_FS_OFFSET;
  undefined4 uVar8;
  char *local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_001107d0(param_2,param_3,&local_38);
  if ((char)uVar1 != '\0') {
    ppVar3 = getpwnam(local_38);
    endpwent();
    if (ppVar3 == (passwd *)0x0) {
      sVar5 = strspn(local_38,"0123456789");
      if ((sVar5 == 0) || (local_38[sVar5] != '\0')) {
        if (*local_38 != '\0') {
          uVar6 = FUN_00124e50(0,DAT_0013e68c,local_38);
          uVar7 = dcgettext(0,"%s is not the name of a known user",5);
                    /* WARNING: Subroutine does not return */
          error(1,0,uVar7,uVar6);
        }
        uVar6 = dcgettext(0,"The argument to -user should not be empty",5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar6);
      }
      _Var2 = FUN_00116f40(local_38,DAT_0013e68c);
    }
    else {
      _Var2 = ppVar3->pw_uid;
    }
    lVar4 = FUN_0010d490(param_1,local_38);
    uVar8 = DAT_0012eca4;
    *(__uid_t *)(lVar4 + 0x38) = _Var2;
    if (99 < _Var2) {
      uVar8 = DAT_0012ec7c;
    }
    *(undefined4 *)(lVar4 + 0x24) = uVar8;
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_nogroup @ 0x11460

undefined8 parse_nogroup(undefined8 param_1)

{
  long lVar1;
  
  lVar1 = FUN_0010d490(param_1,0);
  *(undefined4 *)(lVar1 + 0x24) = 0x38d1b717;
  return 1;
}




// Function: parse_lname @ 0x11480

undefined4 parse_lname(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined4 uVar1;
  long lVar2;
  long in_FS_OFFSET;
  float fVar3;
  undefined8 local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00111020();
  uVar1 = FUN_001107d0(param_2,param_3,&local_38);
  if ((char)uVar1 != '\0') {
    lVar2 = FUN_0010d490(param_1,local_38);
    *(undefined8 *)(lVar2 + 0x38) = local_38;
    fVar3 = (float)FUN_001110c0(local_38,0);
    *(float *)(lVar2 + 0x24) = fVar3 * DAT_0012ec74;
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_ilname @ 0x11520

undefined4 parse_ilname(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined4 uVar1;
  long lVar2;
  long in_FS_OFFSET;
  float fVar3;
  undefined8 local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_001107d0(param_2,param_3,&local_28);
  if ((char)uVar1 != '\0') {
    lVar2 = FUN_0010d490(param_1,local_28);
    *(undefined8 *)(lVar2 + 0x38) = local_28;
    fVar3 = (float)FUN_001110c0(local_28,0);
    *(float *)(lVar2 + 0x24) = fVar3 * DAT_0012ec74;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_context @ 0x115b0

undefined8 parse_context(undefined8 param_1,long param_2,int *param_3)

{
  int iVar1;
  long lVar2;
  undefined8 uVar3;
  
  if (param_2 == 0) {
    return 0;
  }
  if (*(long *)(param_2 + (long)*param_3 * 8) != 0) {
    iVar1 = is_selinux_enabled();
    if (0 < iVar1) {
      lVar2 = FUN_0010d490(param_1,0);
      iVar1 = *param_3;
      *(undefined4 *)(lVar2 + 0x24) = 0x3c23d70a;
      uVar3 = *(undefined8 *)(param_2 + (long)iVar1 * 8);
      *(undefined1 *)(lVar2 + 0x1a) = 0;
      *(undefined8 *)(lVar2 + 0x38) = uVar3;
      *param_3 = *param_3 + 1;
      return 1;
    }
    uVar3 = dcgettext(0,"invalid predicate -context: SELinux is not enabled.",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar3);
  }
  return 0;
}




// Function: get_relative_timestamp @ 0x11650

undefined4
get_relative_timestamp(double param_1,undefined8 param_2,long param_3,long param_4,long param_5,
            undefined8 param_6)

{
  undefined8 uVar1;
  char cVar2;
  undefined4 uVar3;
  long lVar4;
  long lVar5;
  long in_FS_OFFSET;
  double dVar6;
  undefined8 local_60 [2];
  double local_50;
  double local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_60[0] = param_2;
  cVar2 = FUN_00110860(local_60,param_3 + 4);
  uVar1 = local_60[0];
  if (cVar2 != '\0') {
    if (*(int *)(param_3 + 4) == 0) {
      *(undefined4 *)(param_3 + 4) = 1;
    }
    else if (*(int *)(param_3 + 4) == 1) {
      *(undefined4 *)(param_3 + 4) = 0;
    }
    uVar3 = FUN_00126d90(local_60[0],0,&local_50,PTR_strtod_0013dff0);
    if ((char)uVar3 != '\0') {
      dVar6 = modf(param_1 * local_50,&local_48);
      dVar6 = dVar6 * DAT_0012ecb0;
      if (DAT_0012ecb0 <= dVar6) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("nanosec < nanosec_per_sec","parser.c",0xc4d,"get_relative_timestamp");
      }
      lVar4 = (long)((double)param_4 - local_48);
      *(long *)(param_3 + 8) = lVar4;
      if (param_4 < lVar4 != local_48 < 0.0) {
                    /* WARNING: Subroutine does not return */
        error(1,0,param_6,uVar1);
      }
      lVar5 = (long)((double)param_5 - dVar6);
      if (dVar6 <= (double)param_5) {
        *(long *)(param_3 + 0x10) = lVar5;
      }
      else {
        *(long *)(param_3 + 0x10) = lVar5 + 1000000000;
        *(long *)(param_3 + 8) = lVar4 + -1;
      }
      goto LAB_001116cb;
    }
  }
  uVar3 = 0;
LAB_001116cb:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_used @ 0x11800

undefined4 parse_used(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined4 uVar1;
  long lVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined4 uVar4;
  undefined1 auVar5 [16];
  undefined8 local_40;
  undefined8 local_38;
  long lStack_30;
  undefined8 local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_001107d0(param_2,param_3,&local_40);
  if ((char)uVar1 != '\0') {
    uVar1 = FUN_00111650(DAT_0012ecb8,local_40,&local_38,0,0,
                         "arithmetic overflow while converting %s days to a number of seconds");
    if ((char)uVar1 == '\0') {
      uVar3 = dcgettext(0,"Invalid argument %s to -used",5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar3,local_40);
    }
    lVar2 = FUN_0010d490(param_1,local_40);
    *(undefined8 *)(lVar2 + 0x38) = local_38;
    *(long *)(lVar2 + 0x40) = lStack_30;
    *(undefined8 *)(lVar2 + 0x48) = local_28;
    auVar5._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar5._0_4_ = (float)(lStack_30 / 0x15180);
    uVar4 = FUN_00110710(auVar5._0_8_);
    *(undefined4 *)(lVar2 + 0x24) = uVar4;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: do_parse_xmin @ 0x11920

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4
do_parse_xmin(undefined8 param_1,undefined8 param_2,undefined4 *param_3,undefined4 param_4)

{
  undefined4 uVar1;
  long lVar2;
  long in_FS_OFFSET;
  undefined4 uVar3;
  undefined8 local_50;
  undefined4 local_48;
  undefined4 uStack_44;
  undefined8 uStack_40;
  undefined8 local_38;
  long local_30;
  
  uVar3 = *param_3;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_001107d0(param_2,param_3,&local_50);
  if ((char)uVar1 != '\0') {
    local_48 = param_4;
    uVar1 = FUN_00111650(_DAT_0012ecc0,local_50,&local_48,DAT_0013e648 + 0x15180,DAT_0013e650,
                         "arithmetic overflow while converting %s minutes to a number of seconds");
    if ((char)uVar1 == '\0') {
      *param_3 = uVar3;
    }
    else {
      lVar2 = FUN_0010d490(param_1,local_50);
      *(ulong *)(lVar2 + 0x38) = CONCAT44(uStack_44,local_48);
      *(undefined8 *)(lVar2 + 0x40) = uStack_40;
      *(undefined8 *)(lVar2 + 0x48) = local_38;
      uVar3 = FUN_00110760(uStack_40);
      *(undefined4 *)(lVar2 + 0x24) = uVar3;
    }
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_mmin @ 0x11a10

void parse_mmin(void)

{
  FUN_00111920();
  return;
}




// Function: get_stat_Ytime @ 0x11a40

ulong get_stat_Ytime(undefined8 param_1,char param_2,undefined1 (*param_3) [16])

{
  undefined1 auVar1 [16];
  
  if (param_2 == 'c') {
    auVar1 = FUN_00125c50();
    *param_3 = auVar1;
    return 1;
  }
  if (param_2 < 'd') {
    if (param_2 == 'B') {
      auVar1 = FUN_00125c70();
      *param_3 = auVar1;
      return ~*(ulong *)(*param_3 + 8) >> 0x3f;
    }
    if (param_2 == 'a') {
      auVar1 = FUN_00125c40();
      *param_3 = auVar1;
      return 1;
    }
  }
  else if (param_2 == 'm') {
    auVar1 = FUN_00125c60();
    *param_3 = auVar1;
    return 1;
  }
                    /* WARNING: Subroutine does not return */
  __assert_fail("0","parser.c",0x1f2,"get_stat_Ytime");
}




// Function: get_num @ 0x11ae0

bool get_num(long param_1,undefined8 param_2,undefined8 param_3)

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
  if (param_1 != 0) {
    cVar1 = FUN_00110860(&local_20,param_3,0,param_2);
    if (cVar1 != '\0') {
      iVar2 = FUN_00127300(local_20,local_18,10);
      bVar3 = iVar2 == 0;
      goto LAB_00111b32;
    }
  }
  bVar3 = false;
LAB_00111b32:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: insert_num @ 0x11b60

long insert_num(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long lVar2;
  char *pcVar3;
  undefined *puVar4;
  long in_FS_OFFSET;
  int local_34;
  undefined8 local_30;
  undefined8 local_28;
  long local_20;
  
  lVar2 = 0;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_001107d0(param_1,param_2,&local_30);
  if (cVar1 != '\0') {
    cVar1 = FUN_00111ae0(local_30,&local_28,&local_34);
    if (cVar1 != '\0') {
      lVar2 = FUN_0010d490(param_3,local_30);
      *(int *)(lVar2 + 0x38) = local_34;
      *(undefined8 *)(lVar2 + 0x40) = local_28;
      if (((byte)DAT_0013e660 & 1) != 0) {
        __fprintf_chk(stderr,1,"inserting %s\n",*(undefined8 *)(lVar2 + 8));
        puVar4 = &DAT_0012e871;
        pcVar3 = "gt";
        if (local_34 != 0) {
          puVar4 = &DAT_0012e87d;
          pcVar3 = "lt";
          if (local_34 != 1) {
            puVar4 = &DAT_0012e86e;
            if (local_34 == 2) {
              puVar4 = &DAT_0012e877;
            }
            pcVar3 = "?";
            if (local_34 == 2) {
              pcVar3 = "eq";
            }
          }
        }
        __fprintf_chk(stderr,1,"    type: %s    %s  ",pcVar3,puVar4);
        __fprintf_chk(stderr,1,"%lu\n",*(undefined8 *)(lVar2 + 0x40));
      }
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_links @ 0x11cb0

undefined8 parse_links(undefined8 param_1,undefined8 param_2,int *param_3)

{
  long lVar1;
  
  lVar1 = FUN_00111b60(param_2,param_3,param_1);
  if (lVar1 == 0) {
    *param_3 = *param_3 + -1;
    return 0;
  }
  if (*(long *)(lVar1 + 0x40) != 1) {
    if (*(long *)(lVar1 + 0x40) != 2) {
      *(undefined4 *)(lVar1 + 0x24) = 0x3a83126f;
      return 1;
    }
    *(undefined4 *)(lVar1 + 0x24) = 0x3c23d70a;
    return 1;
  }
  *(undefined4 *)(lVar1 + 0x24) = 0x3f7d70a4;
  return 1;
}




// Function: parse_inum @ 0x11d20

undefined8 parse_inum(undefined8 param_1,undefined8 param_2,int *param_3)

{
  long lVar1;
  
  lVar1 = FUN_00111b60(param_2,param_3,param_1);
  if (lVar1 != 0) {
    *(undefined4 *)(lVar1 + 0x24) = 0x358637bd;
    *(undefined2 *)(lVar1 + 0x1a) = 0;
    *(undefined1 *)(lVar1 + 0x1c) = 1;
    return 1;
  }
  *param_3 = *param_3 + -1;
  return 0;
}




// Function: parse_size @ 0x11de0

undefined4 parse_size(undefined8 param_1,long param_2,int *param_3)

{
  char *pcVar1;
  char *__s;
  undefined4 uVar2;
  size_t sVar3;
  long lVar4;
  undefined8 uVar5;
  char cVar6;
  long lVar7;
  long in_FS_OFFSET;
  undefined4 uVar8;
  int local_54;
  long local_50;
  char local_42 [2];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if ((param_2 == 0) || (__s = *(char **)(param_2 + (long)*param_3 * 8), __s == (char *)0x0)) {
    uVar2 = 0;
  }
  else {
    sVar3 = strlen(__s);
    if ((int)sVar3 == 0) {
      uVar5 = dcgettext(0,"invalid null argument to -size",5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar5);
    }
    pcVar1 = __s + (long)(int)sVar3 + -1;
    cVar6 = *pcVar1;
    if (cVar6 == 'b') {
      *pcVar1 = '\0';
      lVar7 = 0x200;
    }
    else if (cVar6 < 'c') {
      if (cVar6 == 'G') {
        *pcVar1 = '\0';
        lVar7 = 0x40000000;
      }
      else if (cVar6 < 'H') {
        if (9 < (byte)(cVar6 - 0x30U)) goto LAB_0011204c;
        lVar7 = 0x200;
        cVar6 = '\0';
      }
      else {
        if (cVar6 != 'M') {
LAB_0011204c:
          uVar5 = dcgettext(0,"invalid -size type `%c\'",5);
                    /* WARNING: Subroutine does not return */
          error(1,0,uVar5,(int)cVar6);
        }
        *pcVar1 = '\0';
        lVar7 = 0x100000;
      }
    }
    else if (cVar6 == 'k') {
      *pcVar1 = '\0';
      lVar7 = 0x400;
    }
    else if (cVar6 == 'w') {
      *pcVar1 = '\0';
      lVar7 = 2;
    }
    else {
      if (cVar6 != 'c') goto LAB_0011204c;
      *pcVar1 = '\0';
      lVar7 = 1;
    }
    uVar2 = FUN_00111ae0(__s,&local_50,&local_54);
    if ((char)uVar2 == '\0') {
      local_42[1] = 0;
      local_42[0] = cVar6;
      uVar5 = dcgettext(0,"Invalid argument `%s%s\' to -size",5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar5,__s,local_42);
    }
    lVar4 = FUN_0010d490(param_1,__s);
    *(int *)(lVar4 + 0x3c) = (int)lVar7;
    *(int *)(lVar4 + 0x38) = local_54;
    if (local_54 == 0) {
      uVar8 = DAT_0012ec74;
      if ((ulong)(lVar7 * local_50) < 0x5001) {
        uVar8 = DAT_0012ec80;
      }
    }
    else {
      uVar8 = DAT_0012ec68;
      if ((local_54 == 1) && (uVar8 = DAT_0012ec80, (ulong)(lVar7 * local_50) < 0x5001)) {
        uVar8 = DAT_0012ec74;
      }
    }
    *(long *)(lVar4 + 0x40) = local_50;
    *(undefined2 *)(lVar4 + 0x1a) = 1;
    *(undefined4 *)(lVar4 + 0x24) = uVar8;
    *param_3 = *param_3 + 1;
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: collect_arg_stat_info @ 0x12080

undefined4
collect_arg_stat_info(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 *param_4)

{
  undefined4 uVar1;
  int iVar2;
  int *piVar3;
  long in_FS_OFFSET;
  undefined8 local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_001107d0(param_1,param_2,&local_38);
  if ((char)uVar1 == '\0') {
    *param_4 = 0;
  }
  else {
    *param_4 = local_38;
    iVar2 = (*DAT_0013e670)(local_38,param_3);
    if (iVar2 != 0) {
      piVar3 = __errno_location();
                    /* WARNING: Subroutine does not return */
      FUN_0010e310(*piVar3,local_38);
    }
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_newer @ 0x12110

undefined4 parse_newer(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined4 uVar1;
  long lVar2;
  long in_FS_OFFSET;
  undefined4 uVar3;
  undefined1 auVar4 [16];
  undefined8 local_d0;
  undefined1 local_c8 [88];
  undefined8 local_70;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010d860(local_c8);
  uVar1 = FUN_00112080(param_2,param_3,local_c8,&local_d0);
  if ((char)uVar1 != '\0') {
    lVar2 = FUN_0010d490(param_1,local_d0);
    auVar4 = FUN_00125c60(local_c8);
    *(undefined8 *)(lVar2 + 0x38) = 3;
    *(undefined1 (*) [16])(lVar2 + 0x40) = auVar4;
    uVar3 = FUN_00110760(local_70);
    *(undefined4 *)(lVar2 + 0x24) = uVar3;
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_cnewer @ 0x121d0

undefined4 parse_cnewer(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined4 uVar1;
  long lVar2;
  long in_FS_OFFSET;
  undefined4 uVar3;
  undefined1 auVar4 [16];
  undefined8 local_d0;
  undefined1 local_c8 [88];
  undefined8 local_70;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010d860(local_c8);
  uVar1 = FUN_00112080(param_2,param_3,local_c8,&local_d0);
  if ((char)uVar1 != '\0') {
    lVar2 = FUN_0010d490(param_1,local_d0);
    *(undefined4 *)(lVar2 + 0x38) = 2;
    auVar4 = FUN_00125c60(local_c8);
    *(undefined4 *)(lVar2 + 0x3c) = 0;
    *(undefined1 (*) [16])(lVar2 + 0x40) = auVar4;
    uVar3 = FUN_00110760(local_70);
    *(undefined4 *)(lVar2 + 0x24) = uVar3;
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_anewer @ 0x122a0

undefined4 parse_anewer(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined4 uVar1;
  long lVar2;
  long in_FS_OFFSET;
  undefined4 uVar3;
  undefined1 auVar4 [16];
  undefined8 local_d0;
  undefined1 local_c8 [88];
  undefined8 local_70;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010d860(local_c8);
  uVar1 = FUN_00112080(param_2,param_3,local_c8,&local_d0);
  if ((char)uVar1 != '\0') {
    lVar2 = FUN_0010d490(param_1,local_d0);
    *(undefined4 *)(lVar2 + 0x38) = 0;
    auVar4 = FUN_00125c60(local_c8);
    *(undefined4 *)(lVar2 + 0x3c) = 0;
    *(undefined1 (*) [16])(lVar2 + 0x40) = auVar4;
    uVar3 = FUN_00110760(local_70);
    *(undefined4 *)(lVar2 + 0x24) = uVar3;
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_samefile @ 0x12370

undefined4 parse_samefile(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined4 uVar1;
  int __fd;
  int iVar2;
  long lVar3;
  int *piVar4;
  long in_FS_OFFSET;
  undefined8 local_170;
  __dev_t local_168;
  __ino_t _Stack_160;
  uint uStack_150;
  stat local_d8;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010d860(&local_168);
  uVar1 = FUN_00112080(param_2,param_3,&local_168,&local_170);
  if ((char)uVar1 == '\0') goto LAB_001123c8;
  FUN_0010d860(&local_d8);
  if (DAT_0013e668 == 0) {
    if (DAT_0013e678 != '\0') {
      __fd = FUN_00115cc0(local_170,0x20000);
      goto joined_r0x001124a3;
    }
    if ((uStack_150 & 0xf000) != 0xa000) goto LAB_00112440;
    __fd = -2;
  }
  else {
LAB_00112440:
    __fd = FUN_00115cc0(local_170,0);
joined_r0x001124a3:
    if (-1 < __fd) {
      iVar2 = fstat(__fd,&local_d8);
      if ((iVar2 != 0) || (iVar2 = (*DAT_0013e670)(local_170,&local_168), iVar2 != 0)) {
        piVar4 = __errno_location();
                    /* WARNING: Subroutine does not return */
        FUN_0010e310(*piVar4,local_170);
      }
      if ((DAT_0013e668 == 0) && (DAT_0013e678 == '\0')) {
        if ((((uStack_150 & 0xf000) == 0xa000) || (local_168 != local_d8.st_dev)) ||
           (_Stack_160 != local_d8.st_ino)) {
          close(__fd);
          __fd = -1;
        }
      }
      else {
        local_168 = local_d8.st_dev;
        _Stack_160 = local_d8.st_ino;
      }
    }
  }
  lVar3 = FUN_0010d490(param_1,local_170);
  *(int *)(lVar3 + 0x48) = __fd;
  *(__ino_t *)(lVar3 + 0x38) = _Stack_160;
  *(undefined4 *)(lVar3 + 0x24) = 0x3c23d70a;
  *(__dev_t *)(lVar3 + 0x40) = local_168;
  *(undefined2 *)(lVar3 + 0x1a) = 1;
LAB_001123c8:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar1;
}




// Function: insert_regex @ 0x125d0

undefined4 insert_regex(undefined8 param_1,undefined8 param_2,undefined8 param_3,int param_4)

{
  undefined4 uVar1;
  long lVar2;
  re_pattern_buffer *__buffer;
  uchar *puVar3;
  size_t __length;
  char *pcVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  undefined4 uVar6;
  char *local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_001107d0(param_1,param_2,&local_38);
  if ((char)uVar1 != '\0') {
    lVar2 = FUN_0010d450(param_3,FUN_001096f0,local_38);
    *(undefined2 *)(lVar2 + 0x1a) = 0;
    __buffer = (re_pattern_buffer *)FUN_00126990(0x40);
    *(re_pattern_buffer **)(lVar2 + 0x38) = __buffer;
    __buffer->allocated = 100;
    puVar3 = (uchar *)FUN_00126990(100);
    __buffer->fastmap = (char *)0x0;
    __buffer->buffer = puVar3;
    re_set_syntax((long)param_4);
    __buffer->syntax = (long)param_4;
    __buffer->translate = (uchar *)0x0;
    __length = strlen(local_38);
    pcVar4 = re_compile_pattern(local_38,__length,__buffer);
    if (pcVar4 != (char *)0x0) {
      uVar5 = dcgettext(0,"failed to compile regular expression \'%s\': %s",5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar5,local_38,pcVar4);
    }
    uVar6 = FUN_001110c0(local_38,1);
    *(undefined4 *)(lVar2 + 0x24) = uVar6;
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_iregex @ 0x12710

void parse_iregex(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_001125d0(param_2,param_3,param_1,DAT_0013e67c | 0x400000);
  return;
}




// Function: stream_is_tty @ 0x12740

undefined8 stream_is_tty(FILE *param_1)

{
  int iVar1;
  undefined4 extraout_var;
  undefined8 uVar2;
  
  iVar1 = fileno(param_1);
  uVar2 = 0;
  if (iVar1 != -1) {
    iVar1 = isatty(iVar1);
    uVar2 = CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),iVar1 != 0);
  }
  return uVar2;
}




// Function: open_output_file @ 0x12770

void open_output_file(char *param_1,undefined8 *param_2)

{
  undefined1 uVar1;
  int iVar2;
  undefined8 uVar3;
  long lVar4;
  int *piVar5;
  
  *param_2 = 0;
  uVar3 = FUN_00124ac0(0);
  param_2[4] = uVar3;
  iVar2 = strcmp(param_1,"/dev/stderr");
  if (iVar2 == 0) {
    param_2[1] = stderr;
    uVar3 = dcgettext(0,"standard error",5);
    param_2[2] = uVar3;
    uVar1 = FUN_00112740(param_2[1]);
    *(undefined1 *)(param_2 + 3) = uVar1;
    return;
  }
  iVar2 = strcmp(param_1,"/dev/stdout");
  if (iVar2 != 0) {
    lVar4 = FUN_0010e580(DAT_0013e610,param_1);
    param_2[2] = param_1;
    param_2[1] = lVar4;
    if (lVar4 != 0) {
      uVar1 = FUN_00112740(lVar4);
      *(undefined1 *)(param_2 + 3) = uVar1;
      return;
    }
    piVar5 = __errno_location();
                    /* WARNING: Subroutine does not return */
    FUN_0010e410(*piVar5,param_1);
  }
  param_2[1] = stdout;
  uVar3 = dcgettext(0,"standard output",5);
  param_2[2] = uVar3;
  uVar1 = FUN_00112740(param_2[1]);
  *(undefined1 *)(param_2 + 3) = uVar1;
  return;
}




// Function: open_stdout @ 0x12870

void open_stdout(undefined8 param_1)

{
  FUN_00112770("/dev/stdout",param_1);
  return;
}




// Function: parse_printf @ 0x12880

void parse_printf(undefined8 param_1,undefined8 param_2,undefined4 *param_3)

{
  undefined4 uVar1;
  char cVar2;
  long in_FS_OFFSET;
  undefined8 local_60;
  undefined1 local_58 [40];
  long local_30;
  
  uVar1 = *param_3;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  cVar2 = FUN_00110790(param_2,param_3,&local_60);
  if (cVar2 != '\0') {
    FUN_00112870(local_58);
    cVar2 = FUN_0010fef0(local_58,param_1,local_60);
    if (cVar2 == '\0') {
      *param_3 = uVar1;
    }
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_print @ 0x12910

undefined8 parse_print(void)

{
  long lVar1;
  
  lVar1 = FUN_0010d4d0();
  *(undefined4 *)(lVar1 + 0x18) = 0x101;
  FUN_00112870(lVar1 + 0x38);
  return 1;
}




// Function: insert_fprint @ 0x12940

undefined8 insert_fprint(undefined8 param_1,long param_2)

{
  long lVar1;
  
  lVar1 = FUN_0010d490();
  if (param_2 == 0) {
    FUN_00112870(lVar1 + 0x38);
  }
  else {
    FUN_00112770(param_2,lVar1 + 0x38);
  }
  *(undefined4 *)(lVar1 + 0x18) = 0x101;
  *(undefined4 *)(lVar1 + 0x24) = 0x3f800000;
  return 1;
}




// Function: parse_print0 @ 0x12990

void parse_print0(undefined8 param_1)

{
  FUN_00112940(param_1,0);
  return;
}




// Function: parse_fprint0 @ 0x129a0

void parse_fprint0(undefined8 param_1,undefined8 param_2,int *param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_001107d0(param_2,param_3,&local_28);
  if (cVar1 != '\0') {
    cVar1 = FUN_00112940(param_1,local_28);
    if (cVar1 == '\0') {
      *param_3 = *param_3 + -1;
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: insert_fls @ 0x12a10

undefined8 insert_fls(undefined8 param_1,long param_2)

{
  long lVar1;
  
  lVar1 = FUN_0010d4d0();
  if (param_2 == 0) {
    FUN_00112870(lVar1 + 0x38);
  }
  else {
    FUN_00112770(param_2,lVar1 + 0x38);
  }
  *(undefined4 *)(lVar1 + 0x24) = 0x3f800000;
  *(undefined2 *)(lVar1 + 0x18) = 0x101;
  return 1;
}




// Function: parse_ls @ 0x12a60

void parse_ls(undefined8 param_1)

{
  FUN_00112a10(param_1,0);
  return;
}




// Function: parse_fls @ 0x12a70

void parse_fls(undefined8 param_1,undefined8 param_2,int *param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_001107d0(param_2,param_3,&local_28);
  if (cVar1 != '\0') {
    cVar1 = FUN_00112a10(param_1,local_28);
    if (cVar1 == '\0') {
      *param_3 = *param_3 + -1;
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_fprintf @ 0x12ae0

undefined8 parse_fprintf(undefined8 param_1,undefined8 param_2,undefined4 *param_3)

{
  char cVar1;
  undefined8 uVar2;
  undefined4 uVar3;
  long in_FS_OFFSET;
  undefined8 local_68;
  undefined8 local_60;
  undefined1 local_58 [40];
  long local_30;
  
  uVar3 = *param_3;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_001107d0(param_2,param_3,&local_68);
  if ((cVar1 != '\0') && (cVar1 = FUN_00110790(param_2,param_3,&local_60), cVar1 != '\0')) {
    FUN_00112770(local_68,local_58);
    uVar3 = *param_3;
    uVar2 = FUN_0010fef0(local_58,param_1,local_60);
    if ((char)uVar2 != '\0') goto LAB_00112b21;
  }
  *param_3 = uVar3;
  uVar2 = 0;
LAB_00112b21:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: parse_fprint @ 0x12b90

undefined4 parse_fprint(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined4 uVar1;
  long lVar2;
  long in_FS_OFFSET;
  undefined8 local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_001107d0(param_2,param_3,&local_28);
  if ((char)uVar1 != '\0') {
    lVar2 = FUN_0010d490(param_1,local_28);
    FUN_00112770(local_28,lVar2 + 0x38);
    *(undefined4 *)(lVar2 + 0x18) = 0x101;
    *(undefined4 *)(lVar2 + 0x24) = 0x3f800000;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_perm @ 0x12c20

undefined4 parse_perm(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined4 uVar1;
  int iVar2;
  int iVar3;
  void *__ptr;
  long lVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  char *pcVar7;
  undefined4 uVar8;
  long in_FS_OFFSET;
  undefined4 local_54;
  char *local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_001107d0(param_2,param_3,&local_48);
  if ((char)uVar1 != '\0') {
    if (*local_48 == '-') {
      pcVar7 = local_48 + 1;
      uVar8 = 0;
      local_54 = DAT_0012ec7c;
    }
    else if (*local_48 == '/') {
      local_54 = DAT_0012ec6c;
      pcVar7 = local_48 + 1;
      uVar8 = 1;
    }
    else {
      uVar8 = 2;
      local_54 = DAT_0012ec68;
      pcVar7 = local_48;
    }
    __ptr = (void *)FUN_0011d600(pcVar7);
    if ((__ptr == (void *)0x0) || ((*local_48 == '+' && ((byte)(local_48[1] - 0x30U) < 8)))) {
      uVar5 = FUN_00124e50(0,DAT_0013e68c,local_48);
      uVar6 = dcgettext(0,"invalid mode %s",5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar6,uVar5);
    }
    iVar2 = FUN_0011d9d0(0,0,0,__ptr,0);
    iVar3 = FUN_0011d9d0(0,1,0,__ptr,0);
    free(__ptr);
    if ((*local_48 == '/') && (iVar3 == 0 && iVar2 == 0)) {
      uVar5 = dcgettext(0,
                        "warning: you have specified a mode pattern %s (which is equivalent to /000). The meaning of -perm /000 has now been changed to be consistent with -perm -000; that is, while it used to match no files, it now matches all files."
                        ,5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar5,local_48);
    }
    lVar4 = FUN_0010d490(param_1,local_48);
    *(undefined4 *)(lVar4 + 0x38) = uVar8;
    *(ulong *)(lVar4 + 0x3c) = CONCAT44(iVar3,iVar2);
    *(undefined4 *)(lVar4 + 0x24) = local_54;
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: check_path_safety @ 0x12e20

void check_path_safety(undefined8 param_1)

{
  char cVar1;
  char *pcVar2;
  undefined8 uVar3;
  char *pcVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  long local_40;
  size_t local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  pcVar2 = getenv("PATH");
  if (pcVar2 != (char *)0x0) {
    FUN_001170d0(pcVar2,":",1,&local_40,&local_38);
    do {
      if (local_38 == 0) {
LAB_00112ebe:
        uVar3 = dcgettext(0,
                          "The current directory is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove the current directory from your $PATH (that is, remove \".\", doubled colons, or leading or trailing colons)"
                          ,5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar3,param_1);
      }
      cVar1 = pcVar2[local_40];
      if ((local_38 == 1) && (cVar1 == '.')) goto LAB_00112ebe;
      if (cVar1 != '/') {
        pcVar4 = strndup(pcVar2 + local_40,local_38);
        if (pcVar4 == (char *)0x0) {
          pcVar4 = pcVar2 + local_40;
        }
        uVar3 = FUN_0010dfb0(0,pcVar4);
        uVar5 = dcgettext(0,
                          "The relative path %s is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove that entry from $PATH"
                          ,5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar5,uVar3,param_1);
      }
      cVar1 = FUN_001170d0(pcVar2,":",0,&local_40,&local_38);
    } while (cVar1 != '\0');
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: insert_exec_ok @ 0x12f60

undefined8 insert_exec_ok(undefined8 param_1,long param_2,long param_3,int *param_4)

{
  int iVar1;
  code *pcVar2;
  bool bVar3;
  int iVar4;
  void *__ptr;
  long lVar5;
  size_t sVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  long lVar11;
  long *plVar12;
  char *pcVar13;
  int iVar14;
  undefined8 *puVar15;
  long *plVar16;
  long in_FS_OFFSET;
  bool bVar17;
  bool bVar18;
  char *local_a8;
  undefined1 local_58 [24];
  long local_40;
  
  pcVar2 = *(code **)(param_2 + 0x18);
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if ((param_3 == 0) || (*(long *)(param_3 + (long)*param_4 * 8) == 0)) {
    uVar7 = 0;
    goto LAB_0011328a;
  }
  __ptr = (void *)FUN_0010d450(param_2,pcVar2,"(some -exec* arguments)");
  bVar17 = pcVar2 != (code *)PTR_FUN_0013dfd0;
  bVar18 = pcVar2 != (code *)PTR_FUN_0013df28;
  *(undefined4 *)((long)__ptr + 0x18) = 0x101;
  *(undefined8 *)((long)__ptr + 0xf8) = 0;
  if (bVar17 && bVar18) {
    *(undefined1 *)((long)__ptr + 0xf4) = 0;
    if (pcVar2 == FUN_00108ff0) goto LAB_00113028;
LAB_00113058:
    if (DAT_0013e5c0 == 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("NULL != initial_wd","parser.c",0xb79,"insert_exec_ok");
    }
    *(long *)((long)__ptr + 0xf8) = DAT_0013e5c0;
  }
  else {
    DAT_0013e698 = 1;
    *(undefined1 *)((long)__ptr + 0xf4) = 1;
    if ((pcVar2 != FUN_00108ff0) && (pcVar2 != (code *)PTR_FUN_0013df28)) goto LAB_00113058;
LAB_00113028:
    DAT_0013e62e = 0;
    FUN_00112e20(param_1);
  }
  *(undefined1 *)((long)__ptr + 0x38) = 0;
  iVar1 = *param_4;
  lVar5 = (long)iVar1 * 8;
  plVar16 = (long *)(param_3 + lVar5);
  pcVar13 = (char *)*plVar16;
  iVar14 = iVar1;
  if (pcVar13 != (char *)0x0) {
    puVar15 = (undefined8 *)(param_3 + 8 + lVar5);
    local_a8 = (char *)0x0;
    iVar4 = 0;
    bVar3 = false;
    plVar12 = plVar16;
    do {
      if (*pcVar13 == ';') {
        if (pcVar13[1] == '\0') {
LAB_0011315e:
          if ((iVar1 != iVar14) && (*plVar12 != 0)) {
            if (*(char *)((long)__ptr + 0x38) != '\0') {
              pcVar13 = "dir";
              if (pcVar2 != FUN_00108ff0) {
                pcVar13 = "";
              }
              if (1 < iVar4) {
                uVar7 = dcgettext(0,"Only one instance of {} is supported with -exec%s ... +",5);
                    /* WARNING: Subroutine does not return */
                error(1,0,uVar7,pcVar13);
              }
              sVar6 = strlen(local_a8);
              if (sVar6 != 2) {
                __snprintf_chk(local_58,0x13,1,0x13,"-exec%s ... {} +",pcVar13);
                uVar7 = FUN_00124e50(2,DAT_0013e68c,local_a8);
                uVar8 = FUN_00124e50(1,DAT_0013e68c,&DAT_0012e922);
                uVar9 = FUN_00124e50(0,DAT_0013e68c,local_58);
                uVar10 = dcgettext(0,"In %s the %s must appear by itself, but you specified %s",5);
                    /* WARNING: Subroutine does not return */
                error(1,0,uVar10,uVar9,uVar8,uVar7);
              }
            }
            lVar5 = (long)__ptr + 0x40;
            iVar4 = FUN_00114ca0(lVar5,0x800);
            if (iVar4 - 1U < 2) {
              uVar7 = dcgettext(0,"The environment is too large for exec().",5);
                    /* WARNING: Subroutine does not return */
              error(1,0,uVar7);
            }
            FUN_00114d80(lVar5);
            iVar4 = iVar14 - iVar1;
            lVar11 = (long)__ptr + 0x98;
            *(code **)((long)__ptr + 0x80) = FUN_0010a610;
            if (*(char *)((long)__ptr + 0x38) == '\0') {
              *(int *)((long)__ptr + 0xf0) = iVar4;
              *(undefined **)((long)__ptr + 0x70) = &DAT_0012e922;
              sVar6 = strlen("{}");
              *(size_t *)((long)__ptr + 0x68) = sVar6;
              *(undefined8 *)((long)__ptr + 0x88) = 0;
              *(undefined8 *)((long)__ptr + 0x90) = 0;
              uVar7 = FUN_00126990((long)iVar4 << 3);
              *(undefined8 *)((long)__ptr + 0xe8) = uVar7;
              FUN_00114db0(lVar5,lVar11,(long)__ptr + 0x38);
              iVar1 = *(int *)((long)__ptr + 0xf0);
              if (0 < iVar1) {
                lVar5 = *(long *)((long)__ptr + 0xe8);
                lVar11 = 0;
                do {
                  *(long *)(lVar5 + lVar11 * 8) = plVar16[lVar11];
                  lVar11 = lVar11 + 1;
                } while (lVar11 != iVar1);
              }
            }
            else {
              *(undefined8 *)((long)__ptr + 0xe8) = 0;
              *(undefined8 *)((long)__ptr + 0x70) = 0;
              *(long *)((long)__ptr + 0x78) = (long)(iVar4 + -1);
              *(undefined8 *)((long)__ptr + 0x68) = 0;
              *(undefined8 *)((long)__ptr + 0x88) = 0;
              *(undefined8 *)((long)__ptr + 0x90) = 0;
              FUN_00114db0(lVar5,lVar11,(long)__ptr + 0x38);
              if (iVar1 < iVar14 + -1) {
                do {
                  pcVar13 = (char *)*plVar16;
                  plVar16 = plVar16 + 1;
                  sVar6 = strlen(pcVar13);
                  FUN_00115000(lVar5,lVar11,pcVar13,sVar6 + 1,0,0,1);
                } while (plVar16 !=
                         (long *)(param_3 + 8 + ((ulong)((iVar14 - iVar1) - 2) + (long)iVar1) * 8));
              }
            }
            if (*plVar12 == 0) {
              *param_4 = iVar14;
              uVar7 = 1;
            }
            else {
              *param_4 = iVar14 + 1;
              uVar7 = 1;
            }
            goto LAB_0011328a;
          }
          break;
        }
      }
      else if ((((*pcVar13 == '+') && (bVar17 && bVar18)) && (pcVar13[1] == '\0')) && (bVar3)) {
        *(undefined1 *)((long)__ptr + 0x38) = 1;
        goto LAB_0011315e;
      }
      lVar5 = FUN_0011caf0(pcVar13);
      if (lVar5 == 0) {
        bVar3 = false;
      }
      else {
        iVar4 = iVar4 + 1;
        if ((iVar14 == 0) && ((pcVar2 == FUN_00108ff0 || (pcVar2 == (code *)PTR_FUN_0013df28)))) {
          uVar7 = dcgettext(0,
                            "You may not use {} within the utility name for -execdir and -okdir, because this is a potential security problem."
                            ,5);
                    /* WARNING: Subroutine does not return */
          error(1,0,uVar7);
        }
        bVar3 = true;
        local_a8 = pcVar13;
      }
      pcVar13 = (char *)*puVar15;
      iVar14 = iVar14 + 1;
      puVar15 = puVar15 + 1;
      plVar12 = plVar12 + 1;
    } while (pcVar13 != (char *)0x0);
  }
  *param_4 = iVar14;
  free(__ptr);
  uVar7 = 0;
LAB_0011328a:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar7;
}




// Function: parse_okdir @ 0x13550

void parse_okdir(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_00112f60("-okdir",param_1,param_2,param_3);
  return;
}




// Function: parse_or @ 0x135d0

undefined8 parse_or(void)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0010b970();
  puVar1[2] = 0x200000003;
  *puVar1 = FUN_001088c0;
  *(undefined2 *)((long)puVar1 + 0x1a) = 0;
  return 1;
}




// Function: parse_and @ 0x13610

undefined8 parse_and(void)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0010b970();
  puVar1[2] = 0x300000003;
  *puVar1 = FUN_00108870;
  *(undefined2 *)((long)puVar1 + 0x1a) = 0;
  return 1;
}




// Function: parse_comma @ 0x13650

undefined8 parse_comma(void)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0010b970();
  puVar1[2] = 0x100000003;
  *puVar1 = FUN_00108cd0;
  *(undefined2 *)((long)puVar1 + 0x1a) = 0;
  *(undefined4 *)((long)puVar1 + 0x24) = 0x3f800000;
  return 1;
}




// Function: parse_closeparen @ 0x13690

undefined8 parse_closeparen(void)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0010b970();
  puVar1[2] = 5;
  *puVar1 = FUN_00108c30;
  *(undefined2 *)((long)puVar1 + 0x1a) = 0;
  return 1;
}




// Function: check_name_arg @ 0x136c0

undefined8 check_name_arg(undefined8 param_1,undefined8 param_2)

{
  char cVar1;
  char *pcVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  char *__s;
  
  cVar1 = FUN_001106f0();
  if (cVar1 != '\0') {
    pcVar2 = strchr(__s,0x2f);
    if (pcVar2 != (char *)0x0) {
      uVar3 = FUN_0010dfb0(2,param_2);
      uVar4 = FUN_0010dfb0(1,&DAT_0012b98d);
      uVar5 = FUN_0010dfb0(0,param_1);
      uVar6 = dcgettext(0,
                        "warning: %s matches against basenames only, but the given pattern contains a directory separator (%s), thus the expression will evaluate to false all the time.  Did you mean %s?"
                        ,5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar6,uVar5,uVar4,uVar3);
    }
  }
  return 1;
}




// Function: parse_name @ 0x13750

undefined4 parse_name(undefined8 param_1,undefined8 param_2,undefined4 *param_3)

{
  undefined4 uVar1;
  long lVar2;
  long in_FS_OFFSET;
  undefined4 uVar3;
  undefined8 local_38;
  long local_30;
  
  uVar3 = *param_3;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_001107d0(param_2,param_3,&local_38);
  if ((char)uVar1 != '\0') {
    FUN_00111020();
    uVar1 = FUN_001136c0("-name","-wholename",local_38);
    if ((char)uVar1 == '\0') {
      *param_3 = uVar3;
    }
    else {
      lVar2 = FUN_0010d490(param_1,local_38);
      *(undefined2 *)(lVar2 + 0x1a) = 0;
      *(undefined8 *)(lVar2 + 0x38) = local_38;
      uVar3 = FUN_001110c0(local_38,0);
      *(undefined4 *)(lVar2 + 0x24) = uVar3;
    }
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_iname @ 0x13810

undefined4 parse_iname(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  undefined4 uVar2;
  long lVar3;
  long in_FS_OFFSET;
  undefined4 uVar4;
  undefined8 local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00111020();
  cVar1 = FUN_001107d0(param_2,param_3,&local_28);
  if (cVar1 != '\0') {
    uVar2 = FUN_001136c0("-iname","-iwholename",local_28);
    if ((char)uVar2 != '\0') {
      lVar3 = FUN_0010d490(param_1,local_28);
      *(undefined2 *)(lVar3 + 0x1a) = 0;
      *(undefined8 *)(lVar3 + 0x38) = local_28;
      uVar4 = FUN_001110c0(local_28,0);
      *(undefined4 *)(lVar3 + 0x24) = uVar4;
      goto LAB_00113850;
    }
  }
  uVar2 = 0;
LAB_00113850:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_group @ 0x138d0

undefined4 parse_group(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined4 uVar1;
  __gid_t _Var2;
  group *pgVar3;
  long lVar4;
  size_t sVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  long in_FS_OFFSET;
  undefined4 uVar9;
  char *local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_001107d0(param_2,param_3,&local_38);
  if ((char)uVar1 != '\0') {
    pgVar3 = getgrnam(local_38);
    endgrent();
    if (pgVar3 == (group *)0x0) {
      sVar5 = strspn(local_38,"0123456789");
      if ((int)sVar5 == 0) {
        if (*local_38 != '\0') {
          uVar6 = FUN_00124e50(0,DAT_0013e68c,local_38);
          uVar7 = dcgettext(0,"%s is not the name of an existing group",5);
                    /* WARNING: Subroutine does not return */
          error(1,0,uVar7,uVar6);
        }
        uVar6 = dcgettext(0,"argument to -group is empty, but should be a group name",5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar6);
      }
      if (local_38[(int)sVar5] != '\0') {
        uVar6 = FUN_00124e50(1,DAT_0013e68c,local_38 + (int)sVar5);
        uVar7 = FUN_00124e50(0,DAT_0013e68c,local_38);
        uVar8 = dcgettext(0,
                          "%s is not the name of an existing group and it does not look like a numeric group ID because it has the unexpected suffix %s"
                          ,5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar8,uVar7,uVar6);
      }
      _Var2 = FUN_00116f40(local_38);
    }
    else {
      _Var2 = pgVar3->gr_gid;
    }
    lVar4 = FUN_0010d490(param_1,local_38);
    uVar9 = DAT_0012eca4;
    *(__gid_t *)(lVar4 + 0x38) = _Var2;
    if (99 < *(ulong *)(lVar4 + 0x40)) {
      uVar9 = DAT_0012ec7c;
    }
    *(undefined4 *)(lVar4 + 0x24) = uVar9;
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: estimate_fstype_success_rate @ 0x13a70

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void estimate_fstype_success_rate(char *param_1)

{
  int iVar1;
  char *__s2;
  long in_FS_OFFSET;
  undefined4 uVar2;
  stat sStack_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = stat("/",&sStack_b8);
  uVar2 = _DAT_0012c588;
  if (iVar1 == 0) {
    __s2 = (char *)FUN_001104c0(_DAT_0012c588,&sStack_b8,&DAT_0012b98d);
    iVar1 = strcmp(param_1,__s2);
    uVar2 = DAT_0012ecac;
    if (iVar1 != 0) {
      uVar2 = DAT_0012ec6c;
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail(uVar2);
}




// Function: parse_daystart @ 0x13b10

undefined8 parse_daystart(void)

{
  tm *ptVar1;
  long lVar2;
  
  if (DAT_0013e658 != '\0') {
    return 1;
  }
  DAT_0013e648 = DAT_0013e648 + 0x15180;
  DAT_0013e650 = 0;
  ptVar1 = localtime(&DAT_0013e648);
  if (ptVar1 == (tm *)0x0) {
    lVar2 = DAT_0013e648 % 0x15180;
  }
  else {
    lVar2 = (long)(ptVar1->tm_min * 0x3c + ptVar1->tm_sec + ptVar1->tm_hour * 0xe10);
  }
  DAT_0013e658 = 1;
  DAT_0013e648 = DAT_0013e648 - lVar2;
  return 1;
}




// Function: parse_time @ 0x13bc0

undefined4 parse_time(undefined8 param_1,undefined8 param_2,undefined4 *param_3)

{
  char cVar1;
  undefined4 uVar2;
  long lVar3;
  char *pcVar4;
  undefined8 extraout_RDX;
  undefined *puVar5;
  long in_FS_OFFSET;
  undefined4 uVar6;
  int local_6c;
  undefined8 local_68;
  long local_60;
  undefined4 local_58;
  int iStack_54;
  undefined8 uStack_50;
  undefined8 local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  dcgettext(0,"arithmetic overflow while converting %s days to a number of seconds",5);
  uVar6 = *param_3;
  uVar2 = FUN_001107d0(param_2,param_3,&local_68);
  if ((char)uVar2 != '\0') {
    lVar3 = DAT_0013e648;
    cVar1 = FUN_00110860(&local_68,&local_6c,extraout_RDX,DAT_0013e650);
    if ((cVar1 != '\0') && (local_6c == 1)) {
      lVar3 = lVar3 + 0x1517f;
    }
    uVar2 = FUN_00111650(DAT_0012ecb8,local_68,&local_58,lVar3);
    if ((char)uVar2 == '\0') {
      *param_3 = uVar6;
    }
    else {
      lVar3 = FUN_0010d490(param_1,local_68);
      *(ulong *)(lVar3 + 0x38) = CONCAT44(iStack_54,local_58);
      *(undefined8 *)(lVar3 + 0x40) = uStack_50;
      *(undefined8 *)(lVar3 + 0x48) = local_48;
      uVar6 = FUN_00110760(uStack_50);
      *(undefined4 *)(lVar3 + 0x24) = uVar6;
      if (((byte)DAT_0013e660 & 1) != 0) {
        __fprintf_chk(stderr,1,"inserting %s\n",*(undefined8 *)(lVar3 + 8));
        if (iStack_54 == 0) {
          puVar5 = &DAT_0012e871;
          pcVar4 = "gt";
        }
        else if (iStack_54 == 1) {
          puVar5 = &DAT_0012e87d;
          pcVar4 = "lt";
        }
        else {
          puVar5 = &DAT_0012e86e;
          pcVar4 = "?";
          if (iStack_54 == 2) {
            __fprintf_chk(stderr,1,"    type: %s    %s  ",&DAT_0012e87a,&DAT_0012e974);
            local_60 = *(time_t *)(lVar3 + 0x40);
            pcVar4 = ctime(&local_60);
            __fprintf_chk(stderr,1,"%lu %s",*(undefined8 *)(lVar3 + 0x40),pcVar4);
            local_60 = *(long *)(lVar3 + 0x40) + 0x15180;
            pcVar4 = ctime(&local_60);
            __fprintf_chk(stderr,1,"                 <  %lu %s",local_60,pcVar4);
            goto LAB_00113c21;
          }
        }
        __fprintf_chk(stderr,1,"    type: %s    %s  ",pcVar4,puVar5);
        local_60 = *(long *)(lVar3 + 0x40);
        pcVar4 = ctime(&local_60);
        __fprintf_chk(stderr,1,"%lu %s",*(undefined8 *)(lVar3 + 0x40),pcVar4);
      }
    }
  }
LAB_00113c21:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: parse_negate @ 0x13e80

undefined8 parse_negate(undefined8 param_1)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0010b990(param_1,0);
  puVar1[2] = 0x400000002;
  *puVar1 = FUN_001092f0;
  *(undefined2 *)((long)puVar1 + 0x1a) = 0;
  return 1;
}




// Function: parse_openparen @ 0x13ec0

undefined8 parse_openparen(undefined8 param_1)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0010b990(param_1,0);
  puVar1[2] = 4;
  *puVar1 = thunk_FUN_00108ac0;
  *(undefined2 *)((long)puVar1 + 0x1a) = 0;
  return 1;
}




// Function: parse_newerXY @ 0x13f00

ulong parse_newerXY(undefined8 param_1,long param_2,int *param_3,undefined8 param_4,
                  undefined8 param_5,undefined8 param_6)

{
  char cVar1;
  undefined *puVar2;
  char cVar3;
  int iVar4;
  int iVar5;
  size_t sVar6;
  char *pcVar7;
  long *plVar8;
  char *pcVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  int *piVar12;
  bool bVar13;
  ulong uVar14;
  long in_FS_OFFSET;
  undefined4 uVar15;
  undefined1 local_d8 [146];
  char local_46 [6];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 != 0) {
    iVar5 = *param_3;
    pcVar9 = *(char **)(param_2 + (long)iVar5 * 8);
    if (pcVar9 != (char *)0x0) {
      sVar6 = strlen(pcVar9);
      if (sVar6 == 8) {
        builtin_strncpy(local_46,"aBcmt",6);
        iVar4 = strncmp("-newer",pcVar9,6);
        if (iVar4 != 0) {
                    /* WARNING: Subroutine does not return */
          __assert_fail("0 == strncmp (\"-newer\", argv[*arg_ptr], 6)","parser.c",0x613,
                        "parse_newerXY");
        }
        cVar3 = pcVar9[6];
        cVar1 = pcVar9[7];
        bVar13 = cVar3 == 'B' || cVar1 == 'B';
        uVar14 = CONCAT71((int7)((ulong)param_6 >> 8),bVar13);
        if (bVar13) {
          uVar10 = dcgettext(0,
                             "This system does not provide a way to find the birth time of a file.",
                             5);
                    /* WARNING: Subroutine does not return */
          error(0,0,uVar10);
        }
        if (cVar3 != 't') {
          pcVar7 = strchr(local_46,(int)cVar3);
          uVar14 = 0;
          if (pcVar7 != (char *)0x0) {
            pcVar7 = strchr(local_46,(int)cVar1);
            uVar14 = 0;
            if (pcVar7 != (char *)0x0) {
              if (*(long *)(param_2 + 8 + (long)iVar5 * 8) == 0) {
                uVar10 = FUN_00124e50(0,DAT_0013e68c,pcVar9);
                uVar11 = dcgettext(0,"The %s test needs an argument",5);
                    /* WARNING: Subroutine does not return */
                error(1,0,uVar11,uVar10);
              }
              *param_3 = iVar5 + 1;
              plVar8 = (long *)FUN_0010d490(param_1);
              if (cVar3 == 'c') {
                uVar15 = 2;
              }
              else if (cVar3 == 'm') {
                uVar15 = 3;
              }
              else {
                if (cVar3 != 'a') {
                  pcVar9 = strchr(local_46,(int)cVar3);
                  if (pcVar9 != (char *)0x0) {
                    /* WARNING: Subroutine does not return */
                    __assert_fail("0","parser.c",0x64c,"parse_newerXY");
                  }
                    /* WARNING: Subroutine does not return */
                  __assert_fail("strchr (validchars, x)","parser.c",0x64b,"parse_newerXY");
                }
                uVar15 = 0;
              }
              *(undefined4 *)(plVar8 + 7) = uVar15;
              if (cVar1 == 't') {
                cVar3 = FUN_00122c40(plVar8 + 8,*(undefined8 *)(param_2 + (long)*param_3 * 8),
                                     &DAT_0013e638);
                if (cVar3 == '\0') {
                  uVar10 = FUN_00124e50(0,DAT_0013e68c,*(undefined8 *)(param_2 + (long)*param_3 * 8)
                                       );
                  uVar11 = dcgettext(0,"I cannot figure out how to interpret %s as a date or time",5
                                    );
                    /* WARNING: Subroutine does not return */
                  error(1,0,uVar11,uVar10);
                }
              }
              else {
                FUN_0010d860(local_d8);
                iVar5 = (*DAT_0013e670)(*(undefined8 *)(param_2 + (long)*param_3 * 8),local_d8);
                if (iVar5 != 0) {
                  piVar12 = __errno_location();
                    /* WARNING: Subroutine does not return */
                  FUN_0010e310(*piVar12,*(undefined8 *)(param_2 + (long)*param_3 * 8));
                }
                iVar5 = FUN_00111a40(local_d8,(int)cVar1,plVar8 + 8);
                if (iVar5 == 0) {
                  uVar10 = FUN_0010dfb0(0,*(undefined8 *)(param_2 + (long)*param_3 * 8));
                  uVar11 = dcgettext(0,"Cannot obtain birth time of file %s",5);
                    /* WARNING: Subroutine does not return */
                  error(1,0,uVar11,uVar10);
                }
              }
              *(undefined4 *)((long)plVar8 + 0x3c) = 0;
              uVar15 = FUN_00110760(plVar8[8]);
              puVar2 = (undefined *)*plVar8;
              *(undefined4 *)((long)plVar8 + 0x24) = uVar15;
              *param_3 = *param_3 + 1;
              if (puVar2 == (undefined *)0x0) {
                    /* WARNING: Subroutine does not return */
                __assert_fail("our_pred->pred_func != NULL","parser.c",0x66f,"parse_newerXY");
              }
              if (puVar2 != PTR_FUN_0013dfb8) {
                    /* WARNING: Subroutine does not return */
                __assert_fail("our_pred->pred_func == pred_newerXY","parser.c",0x670,"parse_newerXY"
                             );
              }
              uVar14 = (ulong)*(byte *)((long)plVar8 + 0x1a);
              if (*(byte *)((long)plVar8 + 0x1a) == 0) {
                    /* WARNING: Subroutine does not return */
                __assert_fail("our_pred->need_stat","parser.c",0x671,"parse_newerXY");
              }
            }
          }
        }
        goto LAB_001140a3;
      }
    }
  }
  uVar14 = 0;
LAB_001140a3:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar14 & 0xffffffff;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_regextype @ 0x14370

undefined8 parse_regextype(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined8 local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_001107d0(param_2,param_3,&local_28);
  if (cVar1 == '\0') {
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      return 0;
    }
  }
  else {
    DAT_0013e67c = FUN_00116d30(local_28);
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      uVar2 = FUN_00114350(param_2,param_3);
      return uVar2;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: insert_depthspec @ 0x14400

undefined8 insert_depthspec(undefined8 param_1,long param_2,int *param_3,int *param_4)

{
  char cVar1;
  int iVar2;
  size_t sVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long in_FS_OFFSET;
  char *local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar4 = *(undefined8 *)(param_2 + -8 + (long)*param_3 * 8);
  cVar1 = FUN_001107d0(param_2,param_3,&local_38);
  if (cVar1 == '\0') {
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      return 0;
    }
LAB_001144fc:
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  sVar3 = strspn(local_38,"0123456789");
  if ((0 < (int)sVar3) && (local_38[(int)sVar3] == '\0')) {
    iVar2 = FUN_00116f40(local_38,DAT_0013e68c);
    *param_4 = iVar2;
    if (-1 < iVar2) {
      if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
        uVar4 = FUN_00114350(param_2,param_3);
        return uVar4;
      }
      goto LAB_001144fc;
    }
  }
  uVar5 = FUN_00124e50(0,DAT_0013e68c,local_38);
  uVar6 = dcgettext(0,"Expected a positive decimal integer argument to %s, but got %s",5);
                    /* WARNING: Subroutine does not return */
  error(1,0,uVar6,uVar4,uVar5);
}




// Function: parse_mindepth @ 0x14510

void parse_mindepth(void)

{
  FUN_00114400();
  return;
}




// Function: parse_depth @ 0x14530

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void parse_depth(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  _DAT_0013e620 = 0x100;
  FUN_00114350(param_2,param_3);
  return;
}




// Function: parse_d @ 0x14550

void parse_d(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  undefined8 uVar2;
  
  cVar1 = FUN_001106f0();
  if (cVar1 == '\0') {
    FUN_00114530(param_1,param_2,param_3);
    return;
  }
  uVar2 = dcgettext(0,
                    "warning: the -d option is deprecated; please use -depth instead, because the latter is a POSIX-compliant feature."
                    ,5);
                    /* WARNING: Subroutine does not return */
  error(0,0,uVar2);
}




// Function: parse_ignore_race @ 0x145b0

void parse_ignore_race(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  DAT_0013e62e = 1;
  FUN_00114350(param_2,param_3);
  return;
}




// Function: parse_xdev @ 0x145d0

void parse_xdev(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  DAT_0013e62d = 1;
  FUN_00114350(param_2,param_3);
  return;
}




// Function: parse_noleaf @ 0x145f0

void parse_noleaf(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  DAT_0013e62c = 1;
  FUN_00114350(param_2,param_3);
  return;
}




// Function: parse_noignore_race @ 0x14610

void parse_noignore_race(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  DAT_0013e62e = 0;
  FUN_00114350(param_2,param_3);
  return;
}




// Function: parse_nowarn @ 0x14630

void parse_nowarn(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  DAT_0013e630 = 0;
  FUN_00114350(param_2,param_3);
  return;
}




// Function: parse_warn @ 0x14650

void parse_warn(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  DAT_0013e630 = 1;
  FUN_00114350(param_2,param_3);
  return;
}




// Function: check_option_combinations @ 0x14670

void check_option_combinations(long *param_1)

{
  code *pcVar1;
  uint uVar2;
  undefined8 uVar3;
  
  if (param_1 != (long *)0x0) {
    uVar2 = 0;
    do {
      while( true ) {
        pcVar1 = (code *)*param_1;
        if (pcVar1 != FUN_00108d40) break;
        param_1 = (long *)param_1[0x21];
        uVar2 = uVar2 | 1;
        if (param_1 == (long *)0x0) goto LAB_001146ba;
      }
      param_1 = (long *)param_1[0x21];
      if (pcVar1 == (code *)PTR_FUN_0013dfc0) {
        uVar2 = uVar2 | 2;
      }
    } while (param_1 != (long *)0x0);
LAB_001146ba:
    if ((uVar2 == 3) && (DAT_0013e621 == '\0')) {
      uVar3 = dcgettext(0,
                        "The -delete action automatically turns on -depth, but -prune does nothing when -depth is in effect.  If you want to carry on anyway, just explicitly use the -depth option."
                        ,5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar3);
    }
  }
  return;
}




// Function: set_follow_state @ 0x146f0

void set_follow_state(int param_1)

{
  if (param_1 == 1) {
    DAT_0013e62c = 1;
    DAT_0013e670 = FUN_0010daa0;
    DAT_0013e680 = FUN_00110a40;
  }
  else if (param_1 == 2) {
    DAT_0013e62c = 1;
    DAT_0013e670 = FUN_0010da00;
    DAT_0013e680 = FUN_001109e0;
  }
  else if (param_1 == 0) {
    DAT_0013e670 = FUN_0010db20;
    DAT_0013e680 = thunk_FUN_001254c0;
  }
  DAT_0013e668 = param_1;
  if (((byte)DAT_0013e660 & 2) != 0) {
    DAT_0013e670 = FUN_0010db70;
  }
  return;
}




// Function: parse_follow @ 0x147a0

void parse_follow(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  FUN_001146f0(1,param_3,param_3,param_4,param_2);
  FUN_00114350(param_2);
  return;
}




// Function: parse_begin_user_args @ 0x147d0

void parse_begin_user_args(void)

{
  DAT_0013ee70 = 0;
  return;
}




// Function: parse_end_user_args @ 0x147e0

void parse_end_user_args(void)

{
  return;
}




// Function: find_parser @ 0x147f0

undefined8 find_parser(char *param_1)

{
  char cVar1;
  int iVar2;
  int iVar3;
  size_t sVar4;
  undefined8 uVar5;
  undefined **ppuVar6;
  undefined *puVar7;
  char *__s1;
  
  iVar2 = strncmp("-newer",param_1,6);
  if (iVar2 == 0) {
    sVar4 = strlen(param_1);
    puVar7 = &DAT_0013d800;
    if (sVar4 == 8) goto LAB_0011487d;
  }
  cVar1 = *param_1;
  ppuVar6 = &PTR_DAT_0013cd48;
  __s1 = "!";
  iVar2 = 0;
  while( true ) {
    iVar3 = strcmp(__s1,param_1 + (cVar1 == '-'));
    if (iVar3 == 0) break;
    __s1 = *ppuVar6;
    ppuVar6 = ppuVar6 + 4;
    iVar2 = iVar2 + 1;
    if (__s1 == (char *)0x0) {
      return 0;
    }
  }
  puVar7 = &DAT_0013cd20 + (long)iVar2 * 0x20;
LAB_0011487d:
  uVar5 = FUN_001108b0(param_1,puVar7);
  return uVar5;
}




// Function: insert_false @ 0x148a0

undefined8 insert_false(void)

{
  undefined8 uVar1;
  long lVar2;
  
  uVar1 = FUN_001147f0("false");
  lVar2 = FUN_0010d4d0(uVar1);
  *(undefined4 *)(lVar2 + 0x18) = 0;
  *(undefined4 *)(lVar2 + 0x24) = 0;
  return 1;
}




// Function: parse_fstype @ 0x148e0

ulong parse_fstype(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  uint uVar2;
  ulong uVar3;
  long lVar4;
  long in_FS_OFFSET;
  undefined4 uVar5;
  undefined8 local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar2 = FUN_001107d0(param_2,param_3,&local_28);
  if ((char)uVar2 != '\0') {
    if (1 < DAT_0013e688) {
      cVar1 = FUN_00110510(local_28);
      if (cVar1 == '\0') {
        if (((byte)DAT_0013e660 & 8) != 0) {
          __fprintf_chk(stderr,1,"-fstype %s can never succeed, substituting -false\n",local_28);
        }
        if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
          uVar3 = FUN_001148a0();
          return uVar3;
        }
        goto LAB_001149c0;
      }
    }
    lVar4 = FUN_0010d490(param_1,local_28);
    *(undefined8 *)(lVar4 + 0x38) = local_28;
    uVar5 = FUN_00113a70(local_28);
    *(undefined4 *)(lVar4 + 0x24) = uVar5;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return (ulong)uVar2;
  }
LAB_001149c0:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: main @ 0x5090

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

byte main(int param_1,long param_2)

{
  undefined1 auVar1 [16];
  bool bVar2;
  char *pcVar3;
  byte bVar4;
  byte bVar5;
  char cVar6;
  int iVar7;
  uint uVar8;
  undefined4 uVar9;
  int iVar10;
  FILE *__stream;
  undefined1 *puVar11;
  long lVar12;
  size_t sVar13;
  undefined **ppuVar14;
  char *pcVar15;
  char *pcVar16;
  undefined8 uVar17;
  int *piVar18;
  long lVar19;
  long *plVar20;
  stat *psVar21;
  undefined4 *puVar22;
  long lVar23;
  int iVar24;
  ulong uVar25;
  char *pcVar26;
  long in_FS_OFFSET;
  char *local_1c8;
  int local_1c0;
  int local_1bc;
  uint local_1a8;
  int local_1a4;
  long local_188;
  long local_180;
  long local_178;
  undefined1 local_170 [8];
  stat local_168;
  stat local_d8;
  undefined1 local_43;
  undefined1 local_42;
  undefined1 local_41;
  long local_40;
  
  bVar5 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_0012a684 = 10;
  local_188 = 0;
  local_180 = 0;
  DAT_0012a080 = 2;
  DAT_0012a804 = 0xffffffff;
  DAT_0012a7e0 = 0x7fffffffffffffff;
  DAT_0012a7f8 = -1;
  DAT_0012a7f0 = -1;
  local_178 = -1;
  DAT_0012a8fd = '\0';
  setlocale(6,"");
  bindtextdomain("grep","/usr/local/share/locale");
  textdomain("grep");
  FUN_001145d0(&DAT_0012a180);
  FUN_0011e1d0(FUN_00107a80);
  FUN_0010dd40(0);
  DAT_0012a8c8 = FUN_00118320(0,0,FUN_001073d0,FUN_00107420,0);
  if (DAT_0012a8c8 == 0) {
                    /* WARNING: Subroutine does not return */
    FUN_0011d4b0();
  }
  local_1a8 = 0;
  local_1bc = 0;
  local_1c0 = -1;
  local_1c8 = (char *)0x0;
  iVar10 = local_1bc;
switchD_00105201_caseD_0:
  local_1bc = iVar10;
  iVar24 = optind;
  iVar7 = FUN_00107aa0(param_1,param_2,&local_178);
  pcVar15 = optarg;
  if (iVar7 != -1) goto code_r0x001051f8;
  if (DAT_0012a905 != '\0') {
    uVar17 = FUN_001179e0();
    FUN_0011cf80(stdout,uVar17,"GNU grep",&DAT_0011f1dd,0);
    pcVar15 = (char *)dcgettext(0,
                                "Written by Mike Haertel and others; see\n<https://git.sv.gnu.org/cgit/grep.git/tree/AUTHORS>."
                                ,5);
    puts(pcVar15);
    bVar5 = 0;
    goto LAB_001059b1;
  }
  if (DAT_0012a908 != 0) {
    FUN_0010a440(0);
LAB_0010615d:
    piVar18 = __errno_location();
                    /* WARNING: Subroutine does not return */
    error(2,*piVar18,&DAT_001209f1,optarg);
  }
  if (local_1c8 == (char *)0x0) {
    lVar12 = (long)optind;
    if (param_1 <= optind) goto switchD_00105201_caseD_1;
    optind = optind + 1;
    pcVar15 = *(char **)(param_2 + lVar12 * 8);
    uVar25 = 0;
    if ((local_1c0 != 2) && (*pcVar15 == '\\')) {
      uVar25 = (ulong)(pcVar15[1] == '-');
    }
    local_1c8 = (char *)FUN_0011d490(pcVar15 + uVar25);
    DAT_0012a8d8 = local_1c8;
    sVar13 = strlen(local_1c8);
    local_1c8[sVar13] = '\n';
    local_188 = FUN_00107bd0(local_1c8,0,sVar13 + 1,"");
  }
  else if (local_188 == 0) {
    DAT_0012a686 = 0;
    DAT_0012a685 = '\0';
    DAT_0012a802 = DAT_0012a802 ^ 1;
    local_188 = 1;
    *local_1c8 = '\n';
  }
  lVar12 = local_188 + -1;
  local_188 = lVar12;
  FUN_001184e0(DAT_0012a8c8);
  if (DAT_0012a7a1 != '\0') {
    bVar2 = false;
    goto LAB_00105952;
  }
  iVar10 = fstat(1,&local_168);
  bVar2 = false;
  if (iVar10 == 0) {
    if ((local_168.st_mode & 0xf000) == 0x8000) {
      psVar21 = &local_168;
      puVar22 = &DAT_0012a920;
      for (lVar19 = 0x24; lVar19 != 0; lVar19 = lVar19 + -1) {
        *puVar22 = (int)psVar21->st_dev;
        psVar21 = (stat *)((long)psVar21 + (ulong)bVar5 * -8 + 4);
        puVar22 = puVar22 + (ulong)bVar5 * -2 + 1;
      }
    }
    else if ((local_168.st_mode & 0xf000) == 0x2000) {
      iVar10 = stat("/dev/null",&local_d8);
      if (((iVar10 == 0) && (local_168.st_ino == local_d8.st_ino)) &&
         (local_168.st_dev == local_d8.st_dev)) {
        DAT_0012a7a0 = '\x01';
      }
      else {
        bVar2 = true;
      }
    }
  }
  if (DAT_0012a7a0 == '\0') {
    if (DAT_0012a80c != 0) goto LAB_0010595c;
  }
  else {
LAB_00105952:
    DAT_0012a80c = 0;
LAB_0010595c:
    DAT_0012a7e8 = 0;
    DAT_0012a7a2 = 1;
  }
  DAT_0012a803 = DAT_0012a7e8 | DAT_0012a7a2;
  if (DAT_0012a7f0 < 0) {
    DAT_0012a7f0 = local_178;
  }
  if (DAT_0012a7f8 < 0) {
    DAT_0012a7f8 = local_178;
  }
  if (((DAT_0012a7e0 == 0) ||
      (((lVar12 == 0 && (DAT_0012a802 != 0)) && ((DAT_0012a685 == '\0' && (DAT_0012a686 == 0))))))
     && (DAT_0012a80c != 2)) {
    bVar5 = 1;
    goto LAB_001059b1;
  }
  if (DAT_0012a900 == 2) {
    if ((bVar2) && (iVar10 = FUN_0011d920(), iVar10 != 0)) {
      iVar10 = isatty(1);
      DAT_0012a900 = (uint)(iVar10 != 0);
    }
    else {
      DAT_0012a900 = 0;
    }
  }
  FUN_0011d960();
  if (DAT_0012a900 != 0) {
    pcVar16 = getenv("GREP_COLOR");
    pcVar15 = PTR_s_01_31_0012a060;
    pcVar3 = PTR_s_01_31_0012a068;
    if ((pcVar16 != (char *)0x0) && (cVar6 = *pcVar16, pcVar26 = pcVar16, cVar6 != '\0')) {
      for (; ((cVar6 == ';' ||
              (cVar6 = FUN_0010da90((int)cVar6), pcVar15 = PTR_s_01_31_0012a060,
              pcVar3 = PTR_s_01_31_0012a068, cVar6 != '\0')) &&
             (cVar6 = pcVar26[1], pcVar15 = pcVar16, pcVar3 = pcVar16, cVar6 != '\0'));
          pcVar26 = pcVar26 + 1) {
      }
    }
    PTR_s_01_31_0012a068 = pcVar3;
    PTR_s_01_31_0012a060 = pcVar15;
    FUN_00107970();
    if ((PTR_s_01_31_0012a068 == pcVar16) || (PTR_s_01_31_0012a060 == pcVar16)) {
      uVar17 = dcgettext(0,"warning: GREP_COLOR=\'%s\' is deprecated; use GREP_COLORS=\'mt=%s\'",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar17,pcVar16,pcVar16);
    }
  }
  FUN_00107520();
  uVar25 = 0;
  if (-1 < local_1c0) {
    uVar25 = (long)local_1c0;
  }
  if (local_1c0 < 3) {
    if (DAT_0012a687 != '\0') {
      FUN_00107d30();
    }
    if (local_1c0 != 2) {
      if (1 < DAT_0012a8d0) {
        iVar10 = FUN_00107fb0(uVar25 & 0xffffffff,local_1c8,&local_188);
        uVar25 = (ulong)iVar10;
      }
      goto LAB_00105a76;
    }
    if (DAT_0012a180 == '\0') {
      bVar5 = 0;
      if (DAT_0012a8d0 != 1) goto LAB_00105ed6;
      goto LAB_00106064;
    }
    cVar6 = FUN_00107730(local_1c8,lVar12);
    bVar5 = 1;
    if ((cVar6 == '\0') && (bVar5 = 0, DAT_0012a687 != '\0')) {
      bVar5 = FUN_00107f20(local_1c8,lVar12);
      bVar5 = bVar5 ^ 1;
    }
    goto LAB_00105ed6;
  }
  goto LAB_00105a76;
code_r0x001051f8:
  iVar10 = local_1bc;
  switch(iVar7) {
  case 0:
  case 0x55:
    goto switchD_00105201_caseD_0;
  default:
    goto switchD_00105201_caseD_1;
  case 0x41:
    FUN_00107900(optarg,&DAT_0012a7f0);
    goto switchD_00105201_caseD_0;
  case 0x42:
    FUN_00107900(optarg,&DAT_0012a7f8);
    goto switchD_00105201_caseD_0;
  case 0x43:
    FUN_00107900(optarg,&local_178);
    goto switchD_00105201_caseD_0;
  case 0x44:
    iVar24 = strcmp(optarg,"read");
    if (iVar24 == 0) {
      DAT_0012a884 = 1;
    }
    else {
      iVar24 = strcmp(pcVar15,"skip");
      if (iVar24 != 0) {
        uVar17 = dcgettext(0,"unknown devices method",5);
                    /* WARNING: Subroutine does not return */
        error(2,0,uVar17);
      }
      DAT_0012a884 = 2;
    }
    goto switchD_00105201_caseD_0;
  case 0x45:
    local_1c0 = FUN_001077b0("egrep",local_1c0);
    goto switchD_00105201_caseD_0;
  case 0x46:
    local_1c0 = FUN_001077b0("fgrep",local_1c0);
    goto switchD_00105201_caseD_0;
  case 0x47:
    local_1c0 = FUN_001077b0("grep",local_1c0);
    goto switchD_00105201_caseD_0;
  case 0x48:
    local_1a8 = 1;
    goto switchD_00105201_caseD_0;
  case 0x4c:
    DAT_0012a80c = 2;
    goto switchD_00105201_caseD_0;
  case 0x50:
    local_1c0 = FUN_001077b0(&DAT_0011f173,local_1c0);
    goto switchD_00105201_caseD_0;
  case 0x52:
    _DAT_0012a014 = 0x30a;
  case 0x72:
    DAT_0012a018 = 3;
    iVar10 = iVar24;
    goto switchD_00105201_caseD_0;
  case 0x54:
    DAT_0012a8fc = 1;
    goto switchD_00105201_caseD_0;
  case 0x56:
    DAT_0012a905 = '\x01';
    goto switchD_00105201_caseD_0;
  case 0x58:
    local_1c0 = FUN_001077b0(optarg,local_1c0);
    goto switchD_00105201_caseD_0;
  case 0x5a:
    DAT_0012a804 = 0;
    goto switchD_00105201_caseD_0;
  case 0x61:
switchD_00105201_caseD_61:
    DAT_0012a810 = 1;
    goto switchD_00105201_caseD_0;
  case 0x62:
    DAT_0012a800 = 1;
    goto switchD_00105201_caseD_0;
  case 99:
    DAT_0012a7e8 = 1;
    goto switchD_00105201_caseD_0;
  case 100:
    lVar12 = FUN_0010d900("--directories",optarg,&PTR_DAT_00128f80,&DAT_00120788,4,PTR_FUN_0012a078,
                          1);
    DAT_0012a018 = *(int *)(&DAT_00120788 + lVar12 * 4);
    iVar10 = iVar24;
    if (DAT_0012a018 != 3) {
      iVar10 = local_1bc;
    }
    goto switchD_00105201_caseD_0;
  case 0x65:
    sVar13 = strlen(optarg);
    lVar19 = local_188;
    lVar12 = sVar13 + 1 + (local_188 - local_180);
    if (0 < lVar12) {
      DAT_0012a8d8 = (char *)FUN_0011d2b0(local_1c8,&local_180,lVar12,0xffffffffffffffff);
      pcVar15 = optarg;
      local_1c8 = DAT_0012a8d8;
    }
    puVar11 = mempcpy(local_1c8 + lVar19,pcVar15,sVar13);
    *puVar11 = 10;
    local_188 = FUN_00107bd0(local_1c8,lVar19,sVar13 + 1 + lVar19);
    goto switchD_00105201_caseD_0;
  case 0x66:
    break;
  case 0x68:
    local_1a8 = 0xffffffff;
    goto switchD_00105201_caseD_0;
  case 0x69:
  case 0x79:
    DAT_0012a687 = '\x01';
    goto switchD_00105201_caseD_0;
  case 0x6c:
    DAT_0012a80c = 1;
    goto switchD_00105201_caseD_0;
  case 0x6d:
    uVar8 = FUN_0011d560(optarg,0,10,&DAT_0012a7e0);
    if (1 < uVar8) {
      uVar17 = dcgettext(0,"invalid max count",5);
                    /* WARNING: Subroutine does not return */
      error(2,0,uVar17);
    }
    goto switchD_00105201_caseD_0;
  case 0x6e:
    DAT_0012a801 = 1;
    goto switchD_00105201_caseD_0;
  case 0x6f:
    DAT_0012a8fd = '\x01';
    goto switchD_00105201_caseD_0;
  case 0x71:
    DAT_0012a7a1 = '\x01';
    DAT_0012a080 = 0;
    goto switchD_00105201_caseD_0;
  case 0x73:
    DAT_0012a904 = 1;
    goto switchD_00105201_caseD_0;
  case 0x75:
    uVar17 = dcgettext(0,"warning: --unix-byte-offsets (-u) is obsolete",5);
                    /* WARNING: Subroutine does not return */
    error(0,0,uVar17);
  case 0x76:
    DAT_0012a802 = 1;
    goto switchD_00105201_caseD_0;
  case 0x77:
    FUN_0010ca30();
    DAT_0012a686 = 1;
    goto switchD_00105201_caseD_0;
  case 0x78:
    DAT_0012a685 = '\x01';
    goto switchD_00105201_caseD_0;
  case 0x7a:
    DAT_0012a684 = 0;
    goto switchD_00105201_caseD_0;
  case 0x80:
    iVar24 = strcmp(optarg,"binary");
    if (iVar24 == 0) {
      DAT_0012a810 = 0;
      goto switchD_00105201_caseD_0;
    }
    iVar24 = strcmp(pcVar15,"text");
    if (iVar24 == 0) goto switchD_00105201_caseD_61;
    iVar24 = strcmp(pcVar15,"without-match");
    if (iVar24 != 0) {
      uVar17 = dcgettext(0,"unknown binary-files type",5);
                    /* WARNING: Subroutine does not return */
      error(2,0,uVar17);
    }
  case 0x49:
    DAT_0012a810 = 2;
    goto switchD_00105201_caseD_0;
  case 0x81:
    if (optarg != (char *)0x0) {
      iVar24 = FUN_0010ddf0(optarg,"always");
      if (((iVar24 == 0) || (iVar24 = FUN_0010ddf0(pcVar15,&DAT_0011f1b9), iVar24 == 0)) ||
         (iVar24 = FUN_0010ddf0(pcVar15,"force"), iVar24 == 0)) {
        DAT_0012a900 = 1;
        goto switchD_00105201_caseD_0;
      }
      iVar24 = FUN_0010ddf0(pcVar15,"never");
      if (((iVar24 == 0) || (iVar24 = FUN_0010ddf0(pcVar15,&DAT_0011f1c9), iVar24 == 0)) ||
         (iVar24 = FUN_0010ddf0(pcVar15,&DAT_0011f1cc), iVar24 == 0)) {
        DAT_0012a900 = 0;
        goto switchD_00105201_caseD_0;
      }
      iVar24 = FUN_0010ddf0(pcVar15,&DAT_0011f1d1);
      if (((iVar24 != 0) && (iVar24 = FUN_0010ddf0(pcVar15,&DAT_0011f1d9), iVar24 != 0)) &&
         (iVar24 = FUN_0010ddf0(pcVar15,&DAT_0011f1d6), iVar24 != 0)) {
        DAT_0012a908 = 1;
        goto switchD_00105201_caseD_0;
      }
    }
    DAT_0012a900 = 2;
    goto switchD_00105201_caseD_0;
  case 0x82:
    FUN_001147b0(optarg);
    if (DAT_0012a8a0 == 0) {
      DAT_0012a8a0 = FUN_00114d20();
    }
    lVar12 = DAT_0012a8a0;
    uVar9 = FUN_001076d0(0);
    FUN_00114fb0(lVar12,optarg,uVar9);
    if (DAT_0012a8a8 == 0) {
      DAT_0012a8a8 = FUN_00114d20();
    }
    lVar12 = DAT_0012a8a8;
    uVar9 = FUN_001076d0(1);
    FUN_00114fb0(lVar12,optarg,uVar9);
    goto switchD_00105201_caseD_0;
  case 0x83:
  case 0x86:
    lVar12 = 0;
    while( true ) {
      lVar19 = (&DAT_0012a8b0)[lVar12];
      if (lVar19 == 0) {
        lVar19 = FUN_00114d20(lVar12);
        (&DAT_0012a8b0)[lVar12] = lVar19;
      }
      uVar8 = FUN_001076d0(lVar12);
      FUN_00114fb0(lVar19,optarg,uVar8 | (uint)(iVar7 == 0x86) << 0x1d);
      if (lVar12 == 1) break;
      lVar12 = 1;
    }
    goto switchD_00105201_caseD_0;
  case 0x84:
    lVar12 = 0;
    while( true ) {
      if ((&DAT_0012a8b0)[lVar12] == 0) {
        uVar17 = FUN_00114d20(lVar12);
        (&DAT_0012a8b0)[lVar12] = uVar17;
      }
      FUN_001076d0(lVar12);
      iVar24 = FUN_00115440(FUN_00114fb0);
      if (iVar24 != 0) goto LAB_0010615d;
      if (lVar12 == 1) break;
      lVar12 = 1;
    }
    goto switchD_00105201_caseD_0;
  case 0x85:
    PTR_DAT_0012a070 = optarg;
    goto switchD_00105201_caseD_0;
  case 0x87:
    DAT_0012a7d8 = 1;
    goto switchD_00105201_caseD_0;
  case 0x88:
    DAT_0012a7d0 = optarg;
    goto switchD_00105201_caseD_0;
  case 0x89:
    DAT_0012a687 = '\0';
    goto switchD_00105201_caseD_0;
  }
  iVar24 = strcmp(optarg,"-");
  __stream = stdin;
  if ((iVar24 != 0) && (__stream = fopen(pcVar15,"r"), __stream == (FILE *)0x0)) {
    piVar18 = __errno_location();
                    /* WARNING: Subroutine does not return */
    error(2,*piVar18,&DAT_001209f1,optarg);
  }
  lVar12 = local_188;
  lVar19 = local_188;
  while( true ) {
    lVar23 = (lVar19 - local_180) + 2;
    if (0 < lVar23) {
      local_1c8 = (char *)FUN_0011d2b0(local_1c8,&local_180,lVar23,0xffffffffffffffff,1);
      DAT_0012a8d8 = local_1c8;
    }
    sVar13 = fread_unlocked(local_1c8 + lVar19,1,local_180 - (lVar19 + 1),__stream);
    if (sVar13 == 0) break;
    lVar19 = lVar19 + sVar13;
  }
  piVar18 = __errno_location();
  local_1a4 = *piVar18;
  iVar24 = ferror_unlocked(__stream);
  if (iVar24 == 0) {
    if (stdin == __stream) {
      clearerr_unlocked(__stream);
      goto LAB_0010575d;
    }
    iVar24 = fclose(__stream);
    if (iVar24 == 0) goto LAB_0010575d;
    local_1a4 = *piVar18;
  }
  if (local_1a4 != 0) {
                    /* WARNING: Subroutine does not return */
    error(2,local_1a4,&DAT_001209f1,optarg);
  }
LAB_0010575d:
  if ((lVar19 != lVar12) && (local_1c8[lVar19 + -1] != '\n')) {
    local_1c8[lVar19] = '\n';
    lVar19 = lVar19 + 1;
  }
  local_188 = FUN_00107bd0(local_1c8,lVar12,lVar19,optarg);
  goto switchD_00105201_caseD_0;
switchD_00105201_caseD_1:
  FUN_0010a440(2);
LAB_00106064:
  bVar5 = DAT_0012a686;
LAB_00105ed6:
  uVar25 = 2;
  if ((bVar5 & 1) != 0) {
    uVar25 = 0;
    FUN_0010a6b0(&DAT_0012a8d8,&local_188);
    local_1c8 = DAT_0012a8d8;
  }
LAB_00105a76:
  DAT_0012a878 = (code *)(&PTR_FUN_00129798)[uVar25 * 4];
  DAT_0012a870 = (*(code *)(&PTR_FUN_00129790)[uVar25 * 4])
                           (local_1c8,local_188,(long)(int)(&DAT_0012978c)[uVar25 * 8],
                            DAT_0012a8fd != '\0' || DAT_0012a900 != 0);
  local_42 = DAT_0012a684;
  local_43 = 0;
  local_41 = 0;
  lVar12 = (*DAT_0012a878)(DAT_0012a870,&local_42,1,local_170,0);
  DAT_0012a820 = (lVar12 == 0) == (bool)DAT_0012a802;
  param_1 = param_1 - optind;
  if ((local_1a8 == 0) && (param_1 < 2)) {
    DAT_0012a808 = -(uint)(DAT_0012a018 == 3);
  }
  else {
    DAT_0012a808 = ~local_1a8 >> 0x1f;
  }
  uVar25 = sysconf(0x1e);
  if (0x3ffffffffffffffa < uVar25 - 1) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  auVar1._8_8_ = 0;
  auVar1._0_8_ = uVar25;
  lVar19 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x18000)) % auVar1,0);
  lVar12 = 0x18000;
  if (lVar19 != 0) {
    lVar12 = (uVar25 + 0x18000) - lVar19;
  }
  DAT_0012a858 = uVar25 + 8 + lVar12;
  DAT_0012a838 = uVar25;
  DAT_0012a860 = FUN_0011d110();
  if (((_DAT_0012a014 & 2) != 0) && (DAT_0012a884 == 0)) {
    DAT_0012a884 = 1;
  }
  if (param_1 < 1) {
    if ((DAT_0012a018 == 3) && (0 < local_1bc)) {
      DAT_0012a88a = 1;
      ppuVar14 = &PTR_DAT_00128f70;
    }
    else {
      ppuVar14 = &PTR_DAT_00128f60;
    }
    bVar5 = FUN_0010a3f0(*ppuVar14);
  }
  else {
    bVar5 = 1;
    plVar20 = (long *)(param_2 + (long)optind * 8);
    lVar12 = *plVar20;
    do {
      bVar4 = FUN_0010a3f0(lVar12);
      lVar12 = plVar20[1];
      plVar20 = plVar20 + 1;
      bVar5 = bVar5 & bVar4;
    } while (lVar12 != 0);
  }
  if (DAT_0012a889 != '\0') {
    bVar5 = 2;
  }
LAB_001059b1:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: possible_backrefs_in_pattern @ 0x62e0

undefined8 possible_backrefs_in_pattern(void *param_1,long param_2,char param_3)

{
  void *pvVar1;
  
  pvVar1 = (void *)((long)param_1 + param_2 + -1);
  if (-1 < param_2 + -1) {
    while (param_1 = memchr(param_1,0x5c,(long)pvVar1 - (long)param_1), param_1 != (void *)0x0) {
      if ((byte)(*(char *)((long)param_1 + 1) - 0x31U) < 9) {
        return 1;
      }
      if (((int)*(char *)((long)param_1 + 1) == (-(uint)(param_3 == '\0') & 0x24) + 0x5c) &&
         (param_1 = (void *)((long)param_1 + 1), pvVar1 == param_1)) {
        return 0;
      }
      param_1 = (void *)((long)param_1 + 1);
    }
  }
  return 0;
}




// Function: kwsmusts @ 0x6350

void kwsmusts(undefined8 *param_1)

{
  char *__s;
  char cVar1;
  byte bVar2;
  undefined1 uVar3;
  char *__ptr;
  undefined8 uVar4;
  size_t sVar5;
  undefined1 *__ptr_00;
  void *pvVar6;
  long lVar7;
  
  __ptr = (char *)FUN_00113730(param_1[1]);
  if (__ptr != (char *)0x0) {
    uVar4 = FUN_0010ca70(0);
    cVar1 = *__ptr;
    __s = __ptr + 3;
    *param_1 = uVar4;
    if (cVar1 == '\0') {
      sVar5 = strlen(__s);
      FUN_0010bab0(uVar4,__s,sVar5);
    }
    else {
      param_1[7] = param_1[7] + 1;
      sVar5 = strlen(__s);
      lVar7 = (byte)__ptr[1] + sVar5 + (ulong)(byte)__ptr[2];
      __ptr_00 = (undefined1 *)FUN_0011d110(lVar7);
      uVar3 = DAT_0012a684;
      *__ptr_00 = DAT_0012a684;
      bVar2 = __ptr[1];
      *(byte *)(param_1 + 8) = *(byte *)(param_1 + 8) | bVar2;
      pvVar6 = memcpy(__ptr_00 + bVar2,__s,sVar5);
      if (__ptr[2] != '\0') {
        *(undefined1 *)((long)pvVar6 + sVar5) = uVar3;
      }
      FUN_0010bab0(*param_1,__ptr_00,lVar7);
      free(__ptr_00);
    }
    FUN_0010bfa0(*param_1);
    free(__ptr);
    return;
  }
  return;
}




// Function: regex_compile @ 0x6460

undefined4
regex_compile(long param_1,char *param_2,size_t param_3,long param_4,long param_5,ulong param_6,
            undefined4 param_7)

{
  char *pcVar1;
  char *pcVar2;
  undefined8 *puVar3;
  long in_FS_OFFSET;
  undefined8 local_90;
  regex_t local_88;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_88.buffer = (uchar *)0x0;
  local_88.allocated = 0;
  if ((DAT_0012a687 == '\0') && ((char)param_7 == '\0')) {
    local_88.fastmap = (char *)FUN_0011d110(0x100);
  }
  else {
    local_88.fastmap = (char *)0x0;
    local_88.translate = (uchar *)0x0;
    if ((char)param_7 != '\0') {
      re_set_syntax(param_6 | 0x2000000);
      pcVar1 = re_compile_pattern(param_2,param_3,&local_88);
      if (pcVar1 == (char *)0x0) {
        regfree(&local_88);
        goto LAB_00106532;
      }
      goto LAB_00106507;
    }
  }
  local_88.translate = (uchar *)0x0;
  re_set_syntax(param_6);
  pcVar1 = re_compile_pattern(param_2,param_3,&local_88);
  if (pcVar1 == (char *)0x0) {
    puVar3 = (undefined8 *)(param_4 * 0x40 + *(long *)(param_1 + 0x10));
    param_7 = 1;
    *puVar3 = local_88.buffer;
    puVar3[1] = local_88.allocated;
    puVar3[2] = local_88.used;
    puVar3[3] = local_88.syntax;
    puVar3[4] = local_88.fastmap;
    puVar3[5] = local_88.translate;
    puVar3[6] = local_88.re_nsub;
    puVar3[7] = local_88._56_8_;
LAB_00106532:
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      return param_7;
    }
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
LAB_00106507:
  free(local_88.fastmap);
  if (param_5 != -1) {
    pcVar2 = (char *)FUN_001089e0(param_5,&local_90);
    if (*pcVar2 != '\0') {
                    /* WARNING: Subroutine does not return */
      error(0,0,"%s:%td: %s",pcVar2,local_90,pcVar1);
    }
  }
                    /* WARNING: Subroutine does not return */
  error(0,0,&DAT_001209f1,pcVar1);
}




// Function: dfaerror @ 0x6630

void dfaerror(undefined8 param_1)

{
                    /* WARNING: Subroutine does not return */
  error(2,0,&DAT_001209f1,param_1);
}




// Function: dfawarn @ 0x6660

void dfawarn(undefined8 param_1)

{
  undefined8 uVar1;
  
  uVar1 = dcgettext(0,"warning: %s",5);
                    /* WARNING: Subroutine does not return */
  error(0,0,uVar1,param_1);
}




// Function: GEAcompile @ 0x6690

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

long GEAcompile(char *param_1,size_t param_2,ulong param_3,char param_4)

{
  undefined8 uVar1;
  byte bVar2;
  byte bVar3;
  char cVar4;
  long lVar5;
  undefined8 uVar6;
  char *pcVar7;
  long lVar8;
  long lVar9;
  byte bVar10;
  size_t sVar11;
  char *pcVar12;
  long lVar13;
  long lVar14;
  long in_FS_OFFSET;
  bool bVar15;
  void *local_a8;
  size_t local_a0;
  char *local_98;
  char *local_90;
  char *local_78;
  size_t local_70;
  long local_50;
  long local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  lVar5 = FUN_0011d390(1,0x48);
  uVar6 = FUN_00113f00();
  bVar15 = DAT_0012a687 != '\0';
  *(undefined8 *)(lVar5 + 8) = uVar6;
  if (bVar15) {
    param_3 = param_3 | 0x400000;
  }
  FUN_001142e0(uVar6,&DAT_0012a180,param_3,
               (uint)(DAT_0012a684 == '\0') * 2 | (uint)param_3 & 0x10 | 0x2c);
  bVar2 = DAT_0012a180 ^ 1 | DAT_0012a182;
  pcVar7 = param_1 + param_2;
  lVar8 = FUN_0011d0f0(0x40);
  *(undefined8 *)(lVar5 + 0x18) = 0;
  local_50 = 1;
  *(long *)(lVar5 + 0x10) = lVar8 + 0x40;
  local_48 = 0;
  local_a8 = rawmemchr(param_1,10);
  lVar14 = 0;
  lVar13 = (long)local_a8 - (long)param_1;
  bVar3 = FUN_001062e0(param_1,lVar13,bVar2);
  lVar8 = 0;
  bVar15 = false;
  local_a0 = 0;
  local_90 = (char *)0x0;
  lVar9 = 1;
  pcVar12 = param_1;
  local_98 = param_1;
  do {
    lVar8 = (lVar8 - lVar9) + 2;
    if (0 < lVar8) {
      lVar8 = FUN_0011d2b0(*(long *)(lVar5 + 0x10) + -0x40,&local_50,lVar8,0xffffffffffffffff,0x40);
      *(long *)(lVar5 + 0x10) = lVar8 + 0x40;
    }
    re_set_syntax(param_3);
    cVar4 = FUN_00106460(lVar5,pcVar12,lVar13,*(undefined8 *)(lVar5 + 0x18),lVar14,param_3,bVar3 ^ 1
                        );
    if (cVar4 == '\0') {
      bVar15 = true;
    }
    pcVar12 = (char *)((long)local_a8 + 1);
    lVar14 = lVar14 + 1;
    if (bVar3 == 0) {
      if (pcVar7 < pcVar12) {
LAB_00106976:
        if (bVar15) {
                    /* WARNING: Subroutine does not return */
          exit(2);
        }
        sVar11 = local_a0;
        pcVar12 = local_90;
        if ((local_98 <= pcVar7) && (sVar11 = param_2, pcVar12 = param_1, param_1 < local_98)) {
          sVar11 = local_a0 + ((long)pcVar7 - (long)local_98);
          pcVar12 = (char *)FUN_0011d180(local_90,sVar11);
          memcpy(pcVar12 + local_a0,local_98,(long)pcVar7 - (long)local_98);
        }
        local_90 = pcVar12;
        local_a0 = sVar11;
        if ((DAT_0012a686 != '\0') ||
           (pcVar12 = (char *)0x0, local_78 = param_1, local_70 = param_2, DAT_0012a685 != '\0')) {
          pcVar12 = (char *)FUN_0011d110(param_2 + 0x2d);
          uVar1 = _UNK_0011f088;
          uVar6 = _UNK_0011f068;
          if (DAT_0012a685 == '\0') {
            if ((param_3 & 0x2000) == 0) {
              *(undefined8 *)pcVar12 = _DAT_0011f080;
              *(undefined8 *)(pcVar12 + 8) = uVar1;
              *(undefined4 *)(pcVar12 + 0x10) = DAT_0011f090;
              *(undefined2 *)(pcVar12 + 0x14) = DAT_0011f094;
              pcVar12[0x16] = DAT_0011f096;
              sVar11 = strlen(pcVar12);
              lVar8 = sVar11 + param_2;
              memcpy(pcVar12 + sVar11,param_1,param_2);
              pcVar7 = "\\)\\([^[:alnum:]_]\\|$\\)";
            }
            else {
              *(undefined8 *)pcVar12 = _DAT_0011f060;
              *(undefined8 *)(pcVar12 + 8) = uVar6;
              *(undefined2 *)(pcVar12 + 0x10) = DAT_0011f070;
              pcVar12[0x12] = DAT_0011f072;
              sVar11 = strlen(pcVar12);
              lVar8 = sVar11 + param_2;
              memcpy(pcVar12 + sVar11,param_1,param_2);
              pcVar7 = ")([^[:alnum:]_]|$)";
            }
          }
          else if ((param_3 & 0x2000) == 0) {
            *(undefined4 *)pcVar12 = DAT_0011f09a;
            sVar11 = strlen(pcVar12);
            lVar8 = sVar11 + param_2;
            memcpy(pcVar12 + sVar11,param_1,param_2);
            pcVar7 = "\\)$";
          }
          else {
            *(undefined2 *)pcVar12 = DAT_0011f097;
            pcVar12[2] = DAT_0011f099;
            sVar11 = strlen(pcVar12);
            lVar8 = sVar11 + param_2;
            memcpy(pcVar12 + sVar11,param_1,param_2);
            pcVar7 = ")$";
          }
          pcVar7 = strcpy(pcVar12 + lVar8,pcVar7);
          local_70 = strlen(pcVar7);
          local_70 = local_70 + lVar8;
          local_78 = pcVar12;
        }
        FUN_00113340(local_78,local_70,*(undefined8 *)(lVar5 + 8));
        FUN_00106350(lVar5);
        FUN_00114250(0,0,*(undefined8 *)(lVar5 + 8),1);
        if (local_90 != (char *)0x0) {
          if ((param_4 != '\0') || (cVar4 = FUN_00113480(*(undefined8 *)(lVar5 + 8)), cVar4 == '\0')
             ) {
            *(long *)(lVar5 + 0x10) = *(long *)(lVar5 + 0x10) + -0x40;
            *(long *)(lVar5 + 0x18) = *(long *)(lVar5 + 0x18) + 1;
            cVar4 = FUN_00106460(lVar5,local_90,local_a0,0,0xffffffffffffffff,param_3,0);
            if (cVar4 == '\0') {
                    /* WARNING: Subroutine does not return */
              abort();
            }
          }
          if (local_78 != local_90) {
            free(local_90);
          }
        }
        free(pcVar12);
        if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
          __stack_chk_fail();
        }
        return lVar5;
      }
      local_a8 = rawmemchr(pcVar12,10);
      lVar13 = (long)local_a8 - (long)pcVar12;
      bVar3 = FUN_001062e0(pcVar12,lVar13,bVar2);
      bVar10 = local_98 < pcVar12 & bVar3;
      if (bVar10 != 0) {
        sVar11 = (long)pcVar12 - (long)local_98;
        lVar8 = (local_a0 - local_48) + sVar11;
        if (0 < lVar8) {
          local_90 = (char *)FUN_0011d2b0(local_90,&local_48,lVar8,0xffffffffffffffff,1);
        }
        memcpy(local_90 + local_a0,local_98,sVar11);
        local_a0 = local_a0 + sVar11;
        bVar3 = bVar10;
      }
    }
    else {
      *(long *)(lVar5 + 0x18) = *(long *)(lVar5 + 0x18) + 1;
      local_98 = pcVar12;
      if (pcVar7 < pcVar12) goto LAB_00106976;
      local_a8 = rawmemchr(pcVar12,10);
      lVar13 = (long)local_a8 - (long)pcVar12;
      bVar3 = FUN_001062e0(pcVar12,lVar13,bVar2);
    }
    lVar8 = *(long *)(lVar5 + 0x18);
    lVar9 = local_50;
  } while( true );
}




// Function: EGexecute @ 0x6c90

long EGexecute(long *param_1,char *param_2,long param_3,long *param_4,char *param_5)

{
  re_registers *__regs;
  undefined1 *puVar1;
  char cVar2;
  long lVar3;
  char cVar4;
  int iVar5;
  long lVar6;
  long lVar7;
  void *pvVar8;
  long lVar9;
  char *pcVar10;
  char *pcVar11;
  char *pcVar12;
  long lVar13;
  int iVar14;
  long lVar15;
  char *pcVar16;
  long lVar17;
  int iVar18;
  re_pattern_buffer *__buffer;
  char *pcVar19;
  long in_FS_OFFSET;
  bool bVar20;
  char *local_f8;
  char *local_e0;
  long local_d8;
  char *local_b0;
  long local_a0;
  char local_69;
  char *local_68;
  long local_60;
  long local_58 [3];
  long local_40;
  
  cVar2 = DAT_0012a684;
  pcVar16 = param_2 + param_3;
  lVar7 = param_1[1];
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  lVar6 = FUN_00113460(lVar7);
  cVar4 = FUN_00113470(lVar7);
  pcVar12 = param_2;
  local_68 = param_2;
joined_r0x00106d0b:
  while( true ) {
    if (pcVar16 <= pcVar12) {
      lVar7 = -1;
      goto LAB_00107225;
    }
    pcVar10 = pcVar12;
    local_e0 = pcVar16;
    local_b0 = param_5;
    if (param_5 == (char *)0x0) break;
LAB_00106f59:
    local_a0 = (long)local_e0 - (long)pcVar10;
    if (0x80000000 < local_a0) {
LAB_001073a6:
                    /* WARNING: Subroutine does not return */
      FUN_0011d4b0();
    }
    pcVar12 = local_e0;
    if (0 < param_1[3]) {
      __regs = (re_registers *)(param_1 + 4);
      local_d8 = 0;
      local_f8 = local_e0;
      pcVar12 = local_e0 + -1;
      lVar9 = (long)local_e0 - (long)local_b0;
      lVar7 = 0;
      iVar18 = (int)pcVar10;
      do {
        while( true ) {
          lVar17 = lVar7 * 0x40;
          __buffer = (re_pattern_buffer *)(param_1[2] + lVar17);
          bVar20 = DAT_0012a684 == '\n';
          __buffer->not_eol = 0;
          __buffer->newline_anchor = bVar20;
          iVar5 = re_search(__buffer,pcVar10,(int)local_a0 + -1,(int)local_b0 - iVar18,
                            (int)lVar9 + -1,__regs);
          if (iVar5 < -1) goto LAB_001073a6;
          pcVar11 = local_f8;
          lVar3 = local_d8;
          if (iVar5 != -1) break;
LAB_001070a8:
          local_d8 = lVar3;
          local_f8 = pcVar11;
          lVar7 = lVar7 + 1;
          if (param_1[3] <= lVar7) goto LAB_00107150;
        }
        pcVar19 = pcVar10 + iVar5;
        if (local_f8 < pcVar19) goto LAB_001070a8;
        lVar15 = (long)(*(int *)param_1[6] - iVar5);
        if (param_5 != (char *)0x0) {
          if (DAT_0012a686 != '\0') {
            if (DAT_0012a685 == '\0') goto LAB_0010700d;
            bVar20 = lVar9 + -1 != lVar15;
            lVar15 = lVar9;
            pcVar19 = local_b0;
            if (bVar20) goto LAB_001070a8;
          }
joined_r0x001071e0:
          pcVar11 = pcVar19;
          lVar3 = lVar15;
          if (((local_f8 <= pcVar19) && (pcVar11 = local_f8, lVar3 = local_d8, local_f8 == pcVar19))
             && (lVar3 = lVar15, lVar15 <= local_d8)) {
            lVar3 = local_d8;
          }
          goto LAB_001070a8;
        }
        if (DAT_0012a685 == '\0') {
          if (DAT_0012a686 != '\0') {
LAB_0010700d:
            do {
              lVar13 = FUN_0010cc90(pcVar19 + lVar15,pcVar12);
              if ((lVar13 == 0) && (lVar13 = FUN_0010cca0(pcVar10,pcVar19,pcVar12), lVar13 == 0)) {
                if (param_5 != (char *)0x0) goto joined_r0x001071e0;
                goto LAB_00107210;
              }
              iVar5 = (int)pcVar19;
              if (lVar15 < 1) {
LAB_00107042:
                if (pcVar12 == pcVar19) break;
                lVar15 = param_1[2];
                ((re_pattern_buffer *)(lVar17 + lVar15))->not_eol = 0;
                iVar5 = re_search((re_pattern_buffer *)(lVar17 + lVar15),pcVar10,(int)local_a0 + -1,
                                  (iVar5 + 1) - iVar18,((int)local_e0 - (iVar5 + 1)) + -1,__regs);
                if (iVar5 < 0) {
                  if (iVar5 != -1) goto LAB_001073a6;
                  break;
                }
                iVar14 = *(int *)param_1[6] - iVar5;
                pcVar19 = pcVar10 + iVar5;
              }
              else {
                lVar13 = param_1[2];
                ((re_pattern_buffer *)(lVar17 + lVar13))->not_eol = 1;
                iVar14 = re_match((re_pattern_buffer *)(lVar17 + lVar13),pcVar10,
                                  (iVar5 + -1 + (int)lVar15) - (int)local_b0,iVar5 - iVar18,__regs);
                if (iVar14 < -1) goto LAB_001073a6;
                if (iVar14 < 1) goto LAB_00107042;
              }
              lVar15 = (long)iVar14;
            } while (pcVar19 <= local_f8);
            goto LAB_001070a8;
          }
          goto LAB_00107210;
        }
        if (lVar15 == lVar9 + -1) goto LAB_00107210;
        lVar7 = lVar7 + 1;
      } while (lVar7 < param_1[3]);
LAB_00107150:
      pcVar12 = local_e0;
      if (local_f8 < local_e0) {
        local_a0 = local_d8;
        pcVar10 = local_f8;
LAB_00107210:
        lVar7 = (long)pcVar10 - (long)param_2;
        *param_4 = local_a0;
LAB_00107225:
        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
          return lVar7;
        }
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
    }
  }
  local_69 = '\0';
  local_60 = 0;
  pcVar11 = pcVar16;
  local_e0 = pcVar12;
  if (*param_1 == 0) {
LAB_00106e20:
    pcVar12 = local_e0;
    pcVar10 = local_e0;
    if (lVar6 != 0) {
      pcVar10 = (char *)FUN_00113450(lVar6,local_e0,pcVar11,0,&local_60,0);
      pcVar12 = pcVar11;
      if ((pcVar10 == (char *)0x0) || (pcVar11 == pcVar10)) goto joined_r0x00106d0b;
      if (local_60 != 0) {
        pvVar8 = memrchr(param_2,(int)cVar2,(long)pcVar10 - (long)param_2);
        local_e0 = (char *)((long)pvVar8 + 1);
      }
      pvVar8 = rawmemchr(pcVar10,(int)cVar2);
      local_60 = 0;
      pcVar12 = local_e0;
      pcVar10 = local_e0;
      pcVar11 = (char *)((long)pvVar8 + 1);
    }
LAB_00106eb5:
    local_e0 = pcVar11;
    pcVar11 = (char *)FUN_00113450(param_1[1],pcVar12,local_e0,0,&local_60,&local_69);
    pcVar12 = local_e0;
    if ((pcVar11 == (char *)0x0) || (pcVar12 = local_e0, local_e0 == pcVar11))
    goto joined_r0x00106d0b;
    if (local_60 != 0) {
      pvVar8 = memrchr(param_2,(int)cVar2,(long)pcVar11 - (long)param_2);
      pcVar10 = (char *)((long)pvVar8 + 1);
    }
    pvVar8 = rawmemchr(pcVar11,(int)cVar2);
    local_e0 = (char *)((long)pvVar8 + 1);
    local_b0 = pcVar10;
    if (local_69 != '\0') goto LAB_00106f59;
  }
  else {
    lVar7 = FUN_0010c870(*param_1,(long)pcVar12 - (ulong)*(byte *)(param_1 + 8),
                         (ulong)*(byte *)(param_1 + 8) + ((long)pcVar16 - (long)pcVar12),local_58,1)
    ;
    if (lVar7 < 0) goto LAB_00107225;
    pcVar19 = pcVar12 + lVar7;
    iVar18 = (int)cVar2;
    pvVar8 = memrchr(param_2,iVar18,(long)pcVar19 - (long)param_2);
    lVar9 = local_58[0];
    pcVar10 = (char *)((long)pvVar8 + 1);
    if (pvVar8 == (void *)0x0) {
      pcVar10 = param_2;
    }
    lVar17 = param_1[7];
    local_e0 = pcVar10;
    if ((cVar4 == '\x01') && (lVar17 <= local_58[0])) {
      lVar9 = (long)pcVar19 - (long)pcVar10;
      if (lVar9 < 0x10) {
        lVar9 = 0x10;
      }
      if (lVar9 < lVar7 >> 2) {
        pvVar8 = rawmemchr(pcVar19,iVar18);
        pcVar11 = (char *)((long)pvVar8 + 1);
      }
      else if (lVar9 < (long)pcVar16 - (long)pcVar12 >> 2) {
        pvVar8 = rawmemchr(pcVar12 + lVar9 * 4,iVar18);
        pcVar11 = (char *)((long)pvVar8 + 1);
      }
      goto LAB_00106e20;
    }
    pvVar8 = rawmemchr(pcVar19,iVar18);
    pcVar11 = (char *)((long)pvVar8 + 1);
    if (lVar17 <= lVar9) goto LAB_00106e20;
    local_e0 = pcVar11;
    if ((DAT_0012a180 == '\x01') && (DAT_0012a182 == '\0')) {
      if (local_68 < pcVar10) {
        local_68 = pcVar10;
      }
      lVar7 = FUN_0010caf0(&local_68,0,pcVar19,pcVar16);
      pcVar12 = local_68;
      if (lVar7 == 0) goto LAB_001073ab;
      goto LAB_00106eb5;
    }
  }
LAB_001073ab:
  local_a0 = (long)local_e0 - (long)pcVar10;
  goto LAB_00107210;
}




// Function: hash_pattern @ 0x73d0

ulong hash_pattern(long param_1,ulong param_2)

{
  byte bVar1;
  ulong uVar2;
  byte *pbVar3;
  
  pbVar3 = (byte *)(DAT_0012a8d8 + -1 + param_1);
  uVar2 = 0x3938a0565d85cb03;
  bVar1 = *pbVar3;
  while (bVar1 != 10) {
    pbVar3 = pbVar3 + 1;
    uVar2 = uVar2 * 0x21 ^ (ulong)bVar1;
    bVar1 = *pbVar3;
  }
  return uVar2 % param_2;
}




// Function: compare_patterns @ 0x7420

undefined8 compare_patterns(long param_1,long param_2)

{
  char *pcVar1;
  char *pcVar2;
  char *pcVar3;
  long lVar4;
  char cVar5;
  
  pcVar1 = (char *)(DAT_0012a8d8 + -1 + param_1);
  pcVar2 = (char *)(DAT_0012a8d8 + -1 + param_2);
  cVar5 = *pcVar1;
  if (cVar5 == *pcVar2) {
    lVar4 = 1;
    do {
      if (cVar5 == '\n') {
        return 1;
      }
      cVar5 = pcVar1[lVar4];
      pcVar3 = pcVar2 + lVar4;
      lVar4 = lVar4 + 1;
    } while (cVar5 == *pcVar3);
  }
  return 0;
}




// Function: color_cap_mt_fct @ 0x7470

void color_cap_mt_fct(void)

{
  PTR_s_01_31_0012a060 = PTR_s_01_31_0012a068;
  return;
}




// Function: color_cap_rv_fct @ 0x7490

void color_cap_rv_fct(void)

{
  DAT_0012a900 = 0xffffffff;
  return;
}




// Function: color_cap_ne_fct @ 0x74a0

void color_cap_ne_fct(void)

{
  PTR_DAT_0012a028 = &DAT_0011f09e;
  PTR_DAT_0012a020 = &DAT_0011f0a4;
  return;
}




// Function: is_device_mode @ 0x74d0

uint is_device_mode(uint param_1,undefined8 param_2,undefined8 param_3,undefined4 param_4)

{
  undefined8 in_RAX;
  uint uVar1;
  
  uVar1 = param_1 & 0xf000;
  return (uint)CONCAT71((int7)((ulong)in_RAX >> 8),(param_1 & 0xb000) == 0x2000) |
         CONCAT31((int3)((uint)param_4 >> 8),uVar1 == 0xc000) |
         CONCAT31((int3)(uVar1 >> 8),uVar1 == 0x1000);
}




// Function: skip_devices @ 0x7500

byte skip_devices(byte param_1)

{
  byte bVar1;
  
  bVar1 = 1;
  if (DAT_0012a884 != 2) {
    bVar1 = DAT_0012a884 == 0 & (param_1 ^ 1);
  }
  return bVar1;
}




// Function: initialize_unibyte_mask @ 0x7520

void initialize_unibyte_mask(void)

{
  uint uVar1;
  uint uVar2;
  long lVar3;
  uint uVar4;
  
  lVar3 = 1;
  uVar2 = 1;
  uVar4 = 0;
  do {
    while (((&DAT_0012a183)[lVar3] == '\x01' || (uVar1 = uVar2, (uVar4 & 0xff & (uint)lVar3) != 0)))
    {
      lVar3 = lVar3 + 1;
      if (lVar3 == 0x100) goto LAB_00107571;
    }
    do {
      uVar2 = uVar1;
      uVar1 = uVar2 * 2;
    } while ((int)(uVar2 * 2) <= (int)(uint)lVar3);
    lVar3 = lVar3 + 1;
    uVar4 = uVar4 | uVar2;
  } while (lVar3 != 0x100);
LAB_00107571:
  DAT_0012a868 = (ulong)(uVar4 & 0xff) * 0x101010101010101;
  return;
}




// Function: skip_easy_bytes @ 0x7590

ulong * skip_easy_bytes(ulong *param_1)

{
  char *pcVar1;
  ulong uVar2;
  byte bVar3;
  
  uVar2 = DAT_0012a868;
  while( true ) {
    if (((ulong)param_1 & 7) == 0) {
      if ((uVar2 & *param_1) == 0) {
        do {
          param_1 = param_1 + 1;
        } while ((uVar2 & *param_1) == 0);
        bVar3 = FUN_0010c9d0((int)(char)*param_1);
      }
      else {
        bVar3 = FUN_0010c9d0((int)(char)*param_1);
      }
      while ((bVar3 & uVar2) == 0) {
        pcVar1 = (char *)((long)param_1 + 1);
        param_1 = (ulong *)((long)param_1 + 1);
        bVar3 = FUN_0010c9d0((int)*pcVar1);
      }
      return param_1;
    }
    bVar3 = FUN_0010c9d0((int)(char)*param_1);
    if ((bVar3 & uVar2) != 0) break;
    param_1 = (ulong *)((long)param_1 + 1);
  }
  return param_1;
}




// Function: buf_has_encoding_errors @ 0x7630

undefined8 buf_has_encoding_errors(undefined1 *param_1,long param_2)

{
  undefined1 *puVar1;
  long lVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined8 local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0012a868 != 0) {
    puVar1 = param_1 + param_2;
    local_28 = 0;
    *puVar1 = 0xff;
    while( true ) {
      param_1 = (undefined1 *)FUN_00107590(param_1);
      if (puVar1 <= param_1) break;
      lVar2 = thunk_FUN_00118c10(param_1,(long)puVar1 - (long)param_1,&local_28);
      if (lVar2 < 0) {
        uVar3 = 1;
        goto LAB_00107699;
      }
      param_1 = param_1 + lVar2;
    }
  }
  uVar3 = 0;
LAB_00107699:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: exclude_options @ 0x76d0

int exclude_options(char param_1)

{
  return (-(uint)(param_1 == '\0') & 0x40000000) + 0x10000000;
}




// Function: all_zeros @ 0x76f0

undefined8 all_zeros(char *param_1,long param_2)

{
  char *pcVar1;
  
  pcVar1 = param_1 + param_2;
  if (param_1 < pcVar1) {
    do {
      if (*param_1 != '\0') {
        return 0;
      }
      param_1 = param_1 + 1;
    } while (param_1 != pcVar1);
  }
  return 1;
}




// Function: open_symlink_nofollow_error @ 0x7720

uint open_symlink_nofollow_error(int param_1,undefined8 param_2,undefined4 param_3)

{
  undefined8 in_RAX;
  
  return (uint)CONCAT71((int7)((ulong)in_RAX >> 8),param_1 == 0x28) |
         CONCAT31((int3)((uint)param_3 >> 8),param_1 == 0x1f);
}




// Function: contains_encoding_error @ 0x7730

undefined8 contains_encoding_error(long param_1,long param_2)

{
  long lVar1;
  undefined8 uVar2;
  long lVar3;
  long in_FS_OFFSET;
  undefined8 local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_38 = 0;
  if (0 < param_2) {
    lVar3 = 0;
    do {
      lVar1 = FUN_0010c9f0(param_1 + lVar3,param_2 - lVar3,&local_38);
      if (lVar1 < 0) {
        uVar2 = 1;
        goto LAB_00107790;
      }
      lVar3 = lVar3 + lVar1;
    } while (lVar3 < param_2);
  }
  uVar2 = 0;
LAB_00107790:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: setmatcher @ 0x77b0

ulong setmatcher(char *param_1,int param_2)

{
  int iVar1;
  undefined8 uVar2;
  ulong uVar3;
  
  uVar3 = 0;
  while( true ) {
    iVar1 = strcmp(param_1,&DAT_00129780 + uVar3 * 0x20);
    if (iVar1 == 0) break;
    uVar3 = uVar3 + 1;
    if (uVar3 == 7) {
      uVar2 = dcgettext(0,"invalid matcher %s",5);
                    /* WARNING: Subroutine does not return */
      error(2,0,uVar2,param_1);
    }
  }
  if ((-1 < param_2) && ((int)uVar3 != param_2)) {
    uVar2 = dcgettext(0,"conflicting matchers specified",5);
                    /* WARNING: Subroutine does not return */
    error(2,0,uVar2);
  }
  return uVar3 & 0xffffffff;
}




// Function: add_count @ 0x7860

void add_count(long param_1,long param_2)

{
  undefined8 uVar1;
  
  if (!SCARRY8(param_1,param_2)) {
    return;
  }
  uVar1 = dcgettext(0,"input is too large to count",5);
                    /* WARNING: Subroutine does not return */
  error(2,0,uVar1);
}




// Function: buf_has_nulls @ 0x7890

bool buf_has_nulls(char *param_1,size_t param_2)

{
  size_t sVar1;
  
  param_1[param_2] = '\0';
  sVar1 = strlen(param_1);
  return param_2 != sVar1;
}




// Function: zap_nuls @ 0x78b0

void zap_nuls(char *param_1,char *param_2,char param_3)

{
  size_t sVar1;
  char *pcVar2;
  
  if (param_3 == '\0') {
    return;
  }
  while( true ) {
    *param_2 = '\0';
    sVar1 = strlen(param_1);
    *param_2 = param_3;
    pcVar2 = param_1 + sVar1;
    if (param_2 == param_1 + sVar1) break;
    do {
      param_1 = pcVar2 + 1;
      *pcVar2 = param_3;
      pcVar2 = param_1;
    } while (*param_1 == '\0');
  }
  return;
}




// Function: context_length_arg @ 0x7900

void context_length_arg(undefined8 param_1,long *param_2)

{
  uint uVar1;
  undefined8 uVar2;
  
  uVar1 = FUN_0011d560(param_1,0,10,param_2,"");
  if ((uVar1 < 2) && (-1 < *param_2)) {
    return;
  }
  uVar2 = dcgettext(0,"invalid context length argument",5);
                    /* WARNING: Subroutine does not return */
  error(2,0,"%s: %s",param_1,uVar2);
}




// Function: parse_grep_colors @ 0x7970

void parse_grep_colors(void)

{
  char cVar1;
  int iVar2;
  char *pcVar3;
  char *__s2;
  undefined **ppuVar4;
  char *pcVar5;
  char *pcVar6;
  
  pcVar3 = getenv("GREP_COLORS");
  if ((pcVar3 != (char *)0x0) && (*pcVar3 != '\0')) {
    __s2 = (char *)FUN_0011d490(pcVar3);
    pcVar6 = (char *)0x0;
    pcVar3 = __s2;
LAB_001079dc:
    do {
      while( true ) {
        pcVar5 = pcVar3 + 1;
        cVar1 = *pcVar3;
        if ((cVar1 != ':') && (cVar1 != '\0')) break;
        *pcVar3 = '\0';
        ppuVar4 = &PTR_DAT_00129600;
        pcVar3 = "mt";
        do {
          iVar2 = strcmp(pcVar3,__s2);
          if (iVar2 == 0) break;
          pcVar3 = ppuVar4[3];
          ppuVar4 = ppuVar4 + 3;
        } while (pcVar3 != (char *)0x0);
        if ((pcVar6 != (char *)0x0) && ((undefined8 *)ppuVar4[1] != (undefined8 *)0x0)) {
          *(char **)ppuVar4[1] = pcVar6;
        }
        if ((code *)ppuVar4[2] != (code *)0x0) {
          (*(code *)ppuVar4[2])();
        }
        if (cVar1 == '\0') {
          return;
        }
        pcVar6 = (char *)0x0;
        __s2 = pcVar5;
        pcVar3 = pcVar5;
      }
      if (cVar1 == '=') {
        if (__s2 == pcVar3) {
          return;
        }
        if (pcVar6 != (char *)0x0) {
          return;
        }
        *pcVar3 = '\0';
        pcVar6 = pcVar5;
        pcVar3 = pcVar5;
        goto LAB_001079dc;
      }
      pcVar3 = pcVar5;
    } while (((pcVar6 == (char *)0x0) || (cVar1 == ';')) ||
            (cVar1 = FUN_0010da90((int)cVar1), cVar1 != '\0'));
  }
  return;
}




// Function: clean_up_stdout @ 0x7a80

void clean_up_stdout(void)

{
  if (DAT_0012a8c0 != 0) {
    return;
  }
  FUN_0010de70();
  return;
}




// Function: get_nondigit_option @ 0x7aa0

undefined4 get_nondigit_option(undefined4 param_1,undefined8 param_2,undefined8 param_3)

{
  int iVar1;
  char cVar2;
  undefined4 uVar3;
  char *pcVar4;
  long in_FS_OFFSET;
  char local_81;
  char *local_80;
  char local_68 [21];
  char local_53 [3];
  char local_50 [16];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_81 = '\0';
  local_80 = local_68;
  do {
    iVar1 = optind;
    uVar3 = getopt_long(param_1,param_2,"0123456789A:B:C:D:EFGHIPTUVX:abcd:e:f:hiLlm:noqRrsuvwxyZz",
                        &PTR_s_basic_regexp_00128fa0,0);
    cVar2 = FUN_0010da90(uVar3);
    if (cVar2 == '\0') {
      if (local_80 != local_68) {
LAB_00107b75:
        *local_80 = '\0';
        FUN_00107900(local_68,param_3);
      }
      if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return uVar3;
    }
    pcVar4 = local_68;
    if (((DAT_0012a010 == iVar1) && (local_81 != '\0')) &&
       (pcVar4 = local_80 + -(ulong)(local_68[0] == '0'),
       local_80 + -(ulong)(local_68[0] == '0') == local_53)) {
      builtin_strncpy(local_80 + -(ulong)(local_68[0] == '0'),"...",4);
      local_80 = local_53 + 3;
      goto LAB_00107b75;
    }
    local_80 = pcVar4 + 1;
    *pcVar4 = (char)uVar3;
    DAT_0012a010 = iVar1;
    local_81 = cVar2;
  } while( true );
}




// Function: update_patterns @ 0x7bd0

long update_patterns(long param_1,long param_2,long param_3,long param_4)

{
  long *plVar1;
  int iVar2;
  void *pvVar3;
  void *__dest;
  long lVar4;
  void *__s;
  int iVar5;
  
  __dest = (void *)(param_1 + param_2);
  if (__dest < (void *)(param_3 + param_1)) {
    lVar4 = 1;
    __s = __dest;
    iVar5 = 0;
    do {
      pvVar3 = rawmemchr(__s,10);
      pvVar3 = (void *)((long)pvVar3 + 1);
      memmove(__dest,__s,(long)pvVar3 - (long)__s);
      param_2 = (long)__dest - param_1;
      iVar2 = FUN_00118750(DAT_0012a8c8,param_2 + 1,0);
      if (iVar2 != 0) {
        if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_0011d4b0();
        }
        __dest = (void *)((long)__dest + ((long)pvVar3 - (long)__s));
        if (iVar5 == 0) {
          if (DAT_0012a8e0 == DAT_0012a8e8) {
            DAT_0012a8f0 = FUN_0011d2b0(DAT_0012a8f0,&DAT_0012a8e8,1,0xffffffffffffffff,0x18);
          }
          plVar1 = (long *)(DAT_0012a8f0 + DAT_0012a8e0 * 0x18);
          DAT_0012a8e0 = DAT_0012a8e0 + 1;
          plVar1[1] = param_4;
          *plVar1 = DAT_0012a8d0;
          plVar1[2] = lVar4;
        }
        DAT_0012a8d0 = DAT_0012a8d0 + 1;
        param_2 = (long)__dest - param_1;
      }
      lVar4 = lVar4 + 1;
      __s = pvVar3;
      iVar5 = iVar2;
    } while (pvVar3 < (void *)(param_3 + param_1));
  }
  return param_2;
}




// Function: setup_ok_fold @ 0x7d30

void setup_ok_fold(void)

{
  undefined1 uVar1;
  int iVar2;
  size_t sVar3;
  int *piVar4;
  undefined1 *puVar5;
  undefined1 *puVar6;
  long in_FS_OFFSET;
  mbstate_t local_e0;
  undefined1 local_d8 [128];
  char local_58 [24];
  long local_40;
  
  puVar5 = &DAT_0012a6a0;
  piVar4 = &DAT_0012a284;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  do {
    if (*piVar4 != -1) {
      iVar2 = FUN_001146b0(*piVar4,local_d8);
      if (0 < iVar2) {
        puVar6 = local_d8 + (long)iVar2 * 4;
        do {
          local_e0.__count = 0;
          local_e0.__value = (_union_27)0x0;
          sVar3 = wcrtomb(local_58,*(wchar_t *)(puVar6 + -4),&local_e0);
          if (sVar3 != 1) {
            uVar1 = 0xff;
            goto LAB_00107dfc;
          }
          puVar6 = puVar6 + -4;
        } while (puVar6 != local_d8 + (ulong)(iVar2 - 1) * -4 + (long)iVar2 * 4 + -4);
      }
      uVar1 = 1;
LAB_00107dfc:
      *puVar5 = uVar1;
    }
    piVar4 = piVar4 + 1;
    puVar5 = puVar5 + 1;
    if (piVar4 == (int *)&DAT_0012a684) {
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
  } while( true );
}




// Function: fgrep_icase_charlen @ 0x7e40

ulong fgrep_icase_charlen(byte *param_1,undefined8 param_2,undefined8 param_3)

{
  int iVar1;
  __int32_t **pp_Var2;
  ulong uVar3;
  ulong uVar4;
  long in_FS_OFFSET;
  undefined4 local_bc;
  undefined1 local_b8 [136];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if ((&DAT_0012a284)[*param_1] == -1) {
    uVar4 = FUN_00118c40(&local_bc,param_1,param_2,param_3);
    if ((uVar4 < 0x11) && (iVar1 = FUN_001146b0(local_bc,local_b8), uVar3 = uVar4, iVar1 == 0)) {
      do {
        uVar3 = uVar3 - 1;
        if ((long)uVar3 < 1) goto LAB_00107e80;
        pp_Var2 = __ctype_toupper_loc();
      } while ((uint)param_1[uVar3] == (*pp_Var2)[param_1[uVar3]]);
    }
    uVar4 = 0xffffffffffffffff;
  }
  else {
    uVar4 = (ulong)(char)(&DAT_0012a6a0)[*param_1];
  }
LAB_00107e80:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar4;
}




// Function: fgrep_icase_available @ 0x7f20

undefined8 fgrep_icase_available(long param_1,long param_2)

{
  int iVar1;
  undefined8 uVar2;
  long lVar3;
  long in_FS_OFFSET;
  undefined8 local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_38 = 0;
  if (0 < param_2) {
    lVar3 = 0;
    do {
      iVar1 = FUN_00107e40(param_1 + lVar3,param_2 - lVar3,&local_38);
      if (iVar1 < 0) {
        uVar2 = 0;
        goto LAB_00107f82;
      }
      lVar3 = lVar3 + iVar1;
    } while (lVar3 < param_2);
  }
  uVar2 = 1;
LAB_00107f82:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: try_fgrep_pattern @ 0x7fb0

int try_fgrep_pattern(int param_1,byte *param_2,size_t *param_3)

{
  byte bVar1;
  void *__src;
  ulong uVar2;
  size_t __n;
  void *__dest;
  undefined1 *puVar3;
  size_t sVar4;
  byte *__src_00;
  long in_FS_OFFSET;
  undefined8 local_48;
  long local_40;
  
  sVar4 = *param_3;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  __src = (void *)FUN_0011d110(sVar4 + 1);
  local_48 = 0;
  __dest = __src;
  __src_00 = param_2;
  for (; sVar4 != 0; sVar4 = sVar4 - __n) {
    bVar1 = *__src_00;
    if (bVar1 == 0x5b) goto LAB_001080b0;
    if ((char)bVar1 < '\\') {
      if ((char)bVar1 < '/') {
        if ('#' < (char)bVar1) {
          uVar2 = 1L << (bVar1 & 0x3f);
          if ((uVar2 & 0x441000000000) != 0) goto LAB_001080b0;
          if ((uVar2 & 0x90000000000) != 0) goto LAB_001080e9;
        }
      }
      else if (bVar1 == 0x3f) {
LAB_001080e9:
        if (param_1 != 0) goto LAB_001080b0;
      }
    }
    else if (bVar1 == 0x5c) {
      if (1 < (long)sVar4) {
        bVar1 = __src_00[1];
        if ((char)bVar1 < '=') {
          if ('\t' < (char)bVar1) {
            uVar2 = 1L << (bVar1 & 0x3f);
            if ((uVar2 & 0x13fe008000000400) == 0) {
              uVar2 = uVar2 & 0xb0000000000;
              goto joined_r0x001081a9;
            }
            goto LAB_001080b0;
          }
        }
        else if ((byte)(bVar1 - 0x3e) < 0x3f) {
          uVar2 = 1L << (bVar1 - 0x3e & 0x3f);
          if ((uVar2 & 0x220001402200011) != 0) goto LAB_001080b0;
          uVar2 = uVar2 & 0x6000000000000002;
joined_r0x001081a9:
          if ((uVar2 != 0) && (param_1 == 0)) goto LAB_001080b0;
        }
        __src_00 = __src_00 + 1;
        sVar4 = sVar4 - 1;
      }
    }
    else {
      if (bVar1 == 0x5e) goto LAB_001080b0;
      if ((byte)(bVar1 + 0x85) < 2) goto LAB_001080e9;
    }
    if (DAT_0012a687 == '\0') {
      __n = FUN_0010c9f0(__src_00,sVar4,&local_48);
    }
    else {
      __n = FUN_00107e40();
    }
    if ((long)__n < 0) goto LAB_001080b0;
    __dest = mempcpy(__dest,__src_00,__n);
    __src_00 = __src_00 + __n;
  }
  param_1 = 2;
  sVar4 = (long)__dest - (long)__src;
  if (*param_3 != sVar4) {
    *param_3 = sVar4;
    puVar3 = mempcpy(param_2,__src,sVar4);
    *puVar3 = 10;
  }
LAB_001080b0:
  free(__src);
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return param_1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: skipped_file @ 0x81e0

ulong skipped_file(undefined8 param_1,byte param_2,uint param_3)

{
  long lVar1;
  ulong uVar2;
  
  if ((char)param_3 == '\0') {
    lVar1 = (&DAT_0012a8b0)[param_2];
  }
  else {
    if (DAT_0012a018 == 4) {
      return (ulong)param_3;
    }
    if ((param_2 != 0) && (DAT_0012a88a != '\0')) {
      return 0;
    }
    lVar1 = (&DAT_0012a8a0)[param_2];
  }
  if (lVar1 == 0) {
    return 0;
  }
  uVar2 = FUN_00114ef0(lVar1,param_1);
  return uVar2;
}




// Function: nlscan @ 0x8250

void nlscan(void *param_1)

{
  void *pvVar1;
  long lVar2;
  int __c;
  
  if (DAT_0012a7c0 < param_1) {
    __c = (int)DAT_0012a684;
    lVar2 = 0;
    pvVar1 = DAT_0012a7c0;
    do {
      pvVar1 = memchr(pvVar1,__c,(long)param_1 - (long)pvVar1);
      if (pvVar1 == (void *)0x0) break;
      pvVar1 = (void *)((long)pvVar1 + 1);
      lVar2 = lVar2 + 1;
    } while (pvVar1 < param_1);
  }
  else {
    lVar2 = 0;
  }
  DAT_0012a818 = FUN_00107860(DAT_0012a818,lVar2);
  DAT_0012a7c0 = param_1;
  return;
}




// Function: pr_sgr_start @ 0x82c0

void pr_sgr_start(char *param_1)

{
  if (*param_1 == '\0') {
    return;
  }
  FUN_0011d970(PTR_DAT_0012a028,param_1);
  return;
}




// Function: pr_sgr_start_if @ 0x82e0

void pr_sgr_start_if(void)

{
  if (DAT_0012a900 == 0) {
    return;
  }
  FUN_001082c0();
  return;
}




// Function: fputs_errno @ 0x8300

void fputs_errno(char *param_1)

{
  int iVar1;
  int *piVar2;
  
  iVar1 = fputs_unlocked(param_1,stdout);
  if (iVar1 < 0) {
    piVar2 = __errno_location();
    DAT_0012a8c0 = *piVar2;
  }
  return;
}




// Function: printf_errno @ 0x8330

void printf_errno(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8,
                 undefined8 param_9,undefined8 param_10,undefined8 param_11,undefined8 param_12,
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
  undefined1 local_b8 [8];
  undefined8 local_b0;
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
  local_d8 = 8;
  local_c8 = local_b8;
  local_d4 = 0x30;
  local_b0 = param_10;
  local_a8 = param_11;
  local_a0 = param_12;
  local_98 = param_13;
  local_90 = param_14;
  iVar1 = __vfprintf_chk(stdout,1,param_9,&local_d8);
  if (iVar1 < 0) {
    piVar2 = __errno_location();
    DAT_0012a8c0 = *piVar2;
  }
  if (local_c0 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: putchar_errno @ 0x8410

void putchar_errno(int param_1)

{
  int iVar1;
  int *piVar2;
  
  iVar1 = putchar_unlocked(param_1);
  if (iVar1 < 0) {
    piVar2 = __errno_location();
    DAT_0012a8c0 = *piVar2;
  }
  return;
}




// Function: fflush_errno @ 0x8430

void fflush_errno(void)

{
  int iVar1;
  int *piVar2;
  
  iVar1 = fflush_unlocked(stdout);
  if (iVar1 != 0) {
    piVar2 = __errno_location();
    DAT_0012a8c0 = *piVar2;
  }
  return;
}




// Function: fillbuf @ 0x8560

byte fillbuf(ulong param_1,long param_2)

{
  size_t __n;
  char cVar1;
  byte bVar2;
  ulong uVar3;
  long lVar4;
  long lVar5;
  void *__dest;
  undefined8 uVar6;
  __off_t _Var7;
  int *piVar8;
  long extraout_RDX;
  undefined8 *puVar9;
  long lVar10;
  long lVar11;
  long lVar12;
  ulong uVar13;
  int __fd;
  
  __dest = DAT_0012a860;
  uVar13 = DAT_0012a838;
  lVar10 = DAT_0012a838 + 8;
  puVar9 = DAT_0012a840;
  if ((long)DAT_0012a860 + (DAT_0012a858 - (long)DAT_0012a840) < lVar10) {
    __n = param_1 + 1;
    uVar3 = ~param_1;
    lVar11 = ((DAT_0012a838 + param_1) - DAT_0012a858) + lVar10;
    if (lVar11 < 1) {
      puVar9 = (undefined8 *)((long)DAT_0012a860 + __n);
      if ((ulong)puVar9 % DAT_0012a838 == 0) {
        memmove(DAT_0012a860,(void *)((long)DAT_0012a840 + uVar3),__n);
      }
      else {
        puVar9 = (undefined8 *)
                 (((DAT_0012a838 + 1 + param_1) - (ulong)puVar9 % DAT_0012a838) + (long)DAT_0012a860
                 );
        memmove((void *)(uVar3 + (long)puVar9),(void *)((long)DAT_0012a840 + uVar3),__n);
      }
    }
    else {
      lVar12 = -1;
      lVar4 = param_2;
      cVar1 = FUN_00108550(*(undefined4 *)(param_2 + 0x18));
      if ((((cVar1 != '\0') && (lVar4 = *(long *)(lVar4 + 0x30) - DAT_0012a830, -1 < lVar4)) &&
          (lVar5 = lVar10 + param_1 + lVar4, !SCARRY8(lVar10 + param_1,lVar4))) &&
         (lVar12 = extraout_RDX + lVar11, extraout_RDX + lVar11 < lVar5)) {
        lVar12 = lVar5;
      }
      __dest = (void *)FUN_0011d2b0(0,&DAT_0012a858,lVar11,lVar12,1);
      uVar13 = DAT_0012a838;
      puVar9 = (undefined8 *)((long)__dest + __n);
      if ((ulong)puVar9 % DAT_0012a838 == 0) {
        memmove(__dest,(void *)(uVar3 + (long)DAT_0012a840),__n);
      }
      else {
        puVar9 = (undefined8 *)
                 (((DAT_0012a838 + 1 + param_1) - (ulong)puVar9 % DAT_0012a838) + (long)__dest);
        memmove((void *)(uVar3 + (long)puVar9),(void *)((long)DAT_0012a840 + uVar3),__n);
      }
      free(DAT_0012a860);
      DAT_0012a860 = __dest;
    }
  }
  DAT_0012a848 = (long)puVar9 - param_1;
  lVar10 = (long)__dest + ((DAT_0012a858 + -8) - (long)puVar9);
  __fd = DAT_0012a850;
  do {
    lVar11 = FUN_0011baf0(__fd,puVar9,lVar10 - lVar10 % (long)uVar13);
    if (lVar11 == -1) {
      bVar2 = 0;
LAB_00108704:
      *puVar9 = 0;
      DAT_0012a840 = puVar9;
      return bVar2;
    }
    DAT_0012a830 = DAT_0012a830 + lVar11;
    bVar2 = DAT_0012a821 ^ 1 | lVar11 == 0;
    if (bVar2 != 0) {
      puVar9 = (undefined8 *)((long)puVar9 + lVar11);
      goto LAB_00108704;
    }
    cVar1 = FUN_001076f0(puVar9,lVar11);
    if (cVar1 == '\0') {
      puVar9 = (undefined8 *)((long)puVar9 + lVar11);
      bVar2 = 1;
      goto LAB_00108704;
    }
    uVar6 = FUN_00107860(DAT_0012a818,lVar11);
    __fd = DAT_0012a850;
    lVar11 = DAT_0012a830;
    DAT_0012a818 = uVar6;
    if (DAT_0012a880 == '\0') {
      _Var7 = lseek(DAT_0012a850,DAT_0012a830,3);
      if ((_Var7 < 0) &&
         (((piVar8 = __errno_location(), *piVar8 != 6 ||
           (cVar1 = FUN_00108550(*(undefined4 *)(param_2 + 0x18)), cVar1 == '\0')) ||
          ((*(long *)(param_2 + 0x30) <= lVar11 || (_Var7 = lseek(__fd,0,2), _Var7 < 0)))))) {
        DAT_0012a880 = '\x01';
      }
      else {
        DAT_0012a818 = FUN_00107860(uVar6,_Var7 - lVar11);
        __fd = DAT_0012a850;
        DAT_0012a830 = _Var7;
      }
    }
  } while( true );
}




// Function: print_line_tail @ 0x88b0

ulong print_line_tail(ulong param_1,ulong param_2,undefined1 *param_3)

{
  char cVar1;
  long lVar2;
  ulong uVar3;
  
  uVar3 = param_2;
  if (param_1 < param_2) {
    lVar2 = 0;
    if (*(char *)(param_2 - 1) == DAT_0012a684) {
      uVar3 = param_2 - 1;
      if (uVar3 <= param_1) goto LAB_001088e8;
      lVar2 = 1;
      cVar1 = *(char *)(param_2 - 2);
    }
    else {
      cVar1 = *(char *)(param_2 - 1);
    }
    if (cVar1 == '\r') {
      uVar3 = param_2 - (lVar2 + 1);
    }
  }
LAB_001088e8:
  if (0 < (long)(uVar3 - param_1)) {
    FUN_001082c0(param_3);
    FUN_00108460(param_1,uVar3 - param_1);
    FUN_00108890(*param_3);
    param_1 = uVar3;
  }
  return param_1;
}




// Function: pr_sgr_end_if @ 0x8950

void pr_sgr_end_if(undefined1 *param_1)

{
  if (DAT_0012a900 == 0) {
    return;
  }
  FUN_00108890(*param_1);
  return;
}




// Function: print_offset @ 0x8970

void print_offset(undefined8 param_1,undefined8 param_2)

{
  FUN_001082e0(param_2);
  FUN_00108330(&DAT_0011f0ed,DAT_0012a8f8,param_1);
  FUN_00108950(param_2);
  return;
}




// Function: print_sep @ 0x89b0

void print_sep(char param_1)

{
  FUN_001082e0(PTR_DAT_0012a040);
  FUN_00108410((int)param_1);
  FUN_00108950(PTR_DAT_0012a040);
  return;
}




// Function: pattern_file_name @ 0x89e0

long pattern_file_name(long param_1,long *param_2)

{
  long lVar1;
  long *plVar2;
  long lVar3;
  
  plVar2 = DAT_0012a8f0;
  if (1 < DAT_0012a8e0) {
    lVar1 = 0x18;
    lVar3 = 1;
    while (*(long *)((long)DAT_0012a8f0 + lVar1) <= param_1) {
      lVar3 = lVar3 + 1;
      lVar1 = lVar1 + 0x18;
      if (lVar3 == DAT_0012a8e0) {
        plVar2 = DAT_0012a8f0 + lVar3 * 3 + -3;
        *param_2 = (param_1 - *plVar2) + plVar2[2];
        return plVar2[1];
      }
    }
    plVar2 = (long *)((long)DAT_0012a8f0 + lVar1 + -0x18);
  }
  *param_2 = (param_1 - *plVar2) + plVar2[2];
  return plVar2[1];
}




// Function: input_filename @ 0x8a50

void input_filename(void)

{
  if (DAT_0012a890 != 0) {
    return;
  }
  DAT_0012a890 = dcgettext(0,"(standard input)",5);
  return;
}




// Function: print_filename @ 0x8a90

void print_filename(void)

{
  undefined8 uVar1;
  
  FUN_001082e0(PTR_DAT_0012a058);
  uVar1 = FUN_00108a50();
  FUN_00108300(uVar1);
  FUN_00108950(PTR_DAT_0012a058);
  return;
}




// Function: print_line_head @ 0x8ac0

char print_line_head(long param_1,long param_2,ulong param_3,char param_4)

{
  undefined1 uVar1;
  bool bVar2;
  bool bVar3;
  char cVar4;
  undefined8 uVar5;
  
  if (DAT_0012a810 != 1) {
    uVar1 = *(undefined1 *)(param_1 + param_2);
    cVar4 = FUN_00107630();
    *(undefined1 *)(param_1 + param_2) = uVar1;
    if (cVar4 != '\0') {
      DAT_0012a888 = 1;
      return '\0';
    }
  }
  if (DAT_0012a808 != 0) {
    FUN_00108a90();
    if (DAT_0012a804 == 0) {
      FUN_00108410(0);
    }
    else {
      FUN_001089b0((int)param_4);
    }
  }
  if (DAT_0012a801 != '\0') {
    if (DAT_0012a7c0 < param_3) {
      FUN_00108250(param_1);
      DAT_0012a818 = FUN_00107860(DAT_0012a818,1);
      DAT_0012a7c0 = param_3;
    }
    FUN_00108970(DAT_0012a818,PTR_DAT_0012a050);
    FUN_001089b0((int)param_4);
  }
  if (DAT_0012a800 != '\0') {
    uVar5 = FUN_00107860(DAT_0012a7c8,param_1 - DAT_0012a848);
    FUN_00108970(uVar5,PTR_DAT_0012a048);
    FUN_001089b0((int)param_4);
  }
  if (DAT_0012a8fc == '\0') {
    cVar4 = '\x01';
  }
  else {
    bVar2 = DAT_0012a801 != '\0' || DAT_0012a800 != '\0';
    bVar3 = DAT_0012a808 != 0;
    cVar4 = DAT_0012a8fc;
    if ((bVar2 || bVar3) && param_2 != 0) {
      FUN_00108410(9);
      cVar4 = (bVar2 || bVar3) && param_2 != 0;
    }
  }
  return cVar4;
}




// Function: print_line_middle @ 0x8c60

ulong print_line_middle(ulong param_1,ulong param_2,undefined8 param_3,undefined8 param_4)

{
  ulong uVar1;
  char cVar2;
  long lVar3;
  ulong uVar4;
  ulong uVar5;
  ulong uVar6;
  long in_FS_OFFSET;
  long local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 < param_2) {
    uVar4 = 0;
    uVar6 = param_1;
    do {
      while( true ) {
        lVar3 = (*DAT_0012a878)(DAT_0012a870,param_1,param_2 - param_1,&local_48,uVar6);
        if ((lVar3 < 0) || (uVar1 = param_1 + lVar3, param_2 == uVar1)) goto LAB_00108d78;
        if (local_48 == 0) break;
        if (DAT_0012a8fd == '\0') {
          FUN_001082c0(param_3);
          uVar5 = uVar4;
          if (uVar4 == 0) {
            uVar5 = uVar6;
          }
          uVar4 = 0;
          FUN_00108460(uVar5,uVar1 - uVar5);
        }
        else {
          cVar2 = FUN_00108ac0(uVar1,local_48,param_2,(-(DAT_0012a802 == '\0') & 0xdU) + 0x2d);
          if (cVar2 == '\0') {
            param_2 = 0;
            goto LAB_00108d84;
          }
        }
        FUN_001082e0(param_4);
        FUN_00108460(uVar1,local_48);
        FUN_00108950(param_4);
        if (DAT_0012a8fd != '\0') {
          FUN_00108410((int)DAT_0012a684);
        }
        uVar6 = uVar1 + local_48;
        if (param_2 <= uVar6) goto LAB_00108d78;
      }
      local_48 = 1;
      if (uVar4 == 0) {
        uVar4 = uVar6;
      }
      uVar6 = uVar1 + 1;
    } while (uVar6 < param_2);
LAB_00108d78:
    if ((DAT_0012a8fd != '\0') || (param_2 = uVar4, param_1 = uVar6, uVar4 != 0)) goto LAB_00108d84;
  }
  else if (DAT_0012a8fd != '\0') goto LAB_00108d84;
  param_2 = param_1;
LAB_00108d84:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return param_2;
}




// Function: prline @ 0x8e20

void prline(ulong param_1,ulong param_2,char param_3)

{
  int iVar1;
  char cVar2;
  undefined8 uVar3;
  byte bVar4;
  char *pcVar5;
  byte bVar6;
  char *pcVar7;
  bool bVar8;
  
  if ((DAT_0012a8fd == 0) &&
     (cVar2 = FUN_00108ac0(param_1,(param_2 - param_1) + -1,param_2,(int)param_3), cVar2 == '\0')) {
    return;
  }
  bVar8 = param_3 == ':';
  bVar4 = (bVar8 ^ DAT_0012a802) & DAT_0012a8fd;
  bVar6 = DAT_0012a8fd;
  if (DAT_0012a900 == 0) {
    if (bVar4 != 0) {
      pcVar5 = (char *)0x0;
      pcVar7 = (char *)0x0;
LAB_00108f00:
      param_1 = FUN_00108c60(param_1,param_2,pcVar7,pcVar5);
      if (param_1 == 0) {
        return;
      }
      goto LAB_00108f20;
    }
  }
  else {
    pcVar7 = PTR_s__00121687_0x1b_0012a038;
    if (bVar8 == (bool)((byte)((uint)DAT_0012a900 >> 0x1f) & DAT_0012a802)) {
      pcVar7 = PTR_s__00121687_0x1b_0012a030;
    }
    pcVar5 = PTR_s_01_31_0012a068;
    if (param_3 != ':') {
      pcVar5 = PTR_s_01_31_0012a060;
    }
    if (bVar4 != 0) goto LAB_00108f00;
    if (*pcVar7 == '\0') {
      if (*pcVar5 == '\0') goto LAB_00108e6c;
      if (bVar8 != (bool)DAT_0012a802) goto LAB_00108f00;
LAB_00108f20:
      if (DAT_0012a8fd != 0) goto LAB_00108e88;
      if (*pcVar7 == '\0') {
        bVar6 = 0;
        goto LAB_00108e6c;
      }
    }
    else {
      if (bVar8 == (bool)DAT_0012a802) goto LAB_00108f20;
      if (*pcVar5 != '\0') goto LAB_00108f00;
    }
    param_1 = FUN_001088b0(param_1,param_2,pcVar7);
    bVar6 = DAT_0012a8fd;
  }
LAB_00108e6c:
  if ((param_1 < param_2) && (bVar6 != 1)) {
    FUN_00108460(param_1,param_2 - param_1);
  }
LAB_00108e88:
  if (DAT_0012a7d8 != '\0') {
    FUN_00108430();
  }
  iVar1 = DAT_0012a8c0;
  if (DAT_0012a8c0 == 0) {
    DAT_0012a7b8 = param_2;
    return;
  }
  uVar3 = dcgettext(0,"write error",5);
                    /* WARNING: Subroutine does not return */
  error(2,iVar1,uVar3);
}




// Function: prpending @ 0x8ff0

void prpending(void *param_1)

{
  void *__s;
  void *pvVar1;
  
  __s = DAT_0012a7b8;
  if (DAT_0012a7b8 == (void *)0x0) {
    __s = DAT_0012a848;
  }
  while ((0 < DAT_0012a7a8 && (__s < param_1))) {
    DAT_0012a7b8 = __s;
    pvVar1 = rawmemchr(__s,(int)DAT_0012a684);
    FUN_00108e20(__s,(long)pvVar1 + 1,0x2d);
    DAT_0012a7a8 = DAT_0012a7a8 + -1;
    __s = DAT_0012a7b8;
  }
  DAT_0012a7b8 = __s;
  return;
}




// Function: prtext @ 0x9070

void prtext(void *param_1,void *param_2)

{
  char *pcVar1;
  void *pvVar2;
  ulong uVar3;
  void *pvVar4;
  int __c;
  long lVar6;
  long lVar7;
  char cVar8;
  void *pvVar5;
  
  cVar8 = DAT_0012a684;
  __c = (int)DAT_0012a684;
  pvVar4 = param_1;
  if ((DAT_0012a803 == '\0') && (((long)DAT_0012a7a8 < 1 || (FUN_00108ff0(), DAT_0012a803 == '\0')))
     ) {
    pvVar2 = DAT_0012a7b8;
    if (DAT_0012a7b8 == (void *)0x0) {
      pvVar2 = DAT_0012a848;
    }
    uVar3 = 0;
    if ((long)DAT_0012a7f8 < 1) {
      if (((-1 < (long)(DAT_0012a7f8 & DAT_0012a7f0)) && (DAT_0012a7b8 != param_1)) &&
         ((DAT_0012a688 != '\0' && (PTR_DAT_0012a070 != (undefined *)0x0)))) {
LAB_00109107:
        FUN_001082e0(PTR_DAT_0012a040);
        FUN_00108300(PTR_DAT_0012a070);
        FUN_00108950(PTR_DAT_0012a040);
        FUN_00108410(10);
        goto LAB_00109135;
      }
    }
    else {
      do {
        while (pvVar5 = pvVar4, pvVar4 <= pvVar2) {
          uVar3 = uVar3 + 1;
          if (uVar3 == DAT_0012a7f8) goto LAB_00109219;
        }
        do {
          pvVar4 = (void *)((long)pvVar5 - 1);
          pcVar1 = (char *)((long)pvVar5 - 2);
          pvVar5 = pvVar4;
        } while (*pcVar1 != cVar8);
        uVar3 = uVar3 + 1;
      } while (uVar3 != DAT_0012a7f8);
LAB_00109219:
      if (((DAT_0012a688 != '\0') && (DAT_0012a7b8 != pvVar4)) &&
         (PTR_DAT_0012a070 != (undefined *)0x0)) goto LAB_00109107;
LAB_00109135:
      if (pvVar4 < param_1) {
        do {
          pvVar2 = rawmemchr(pvVar4,(int)cVar8);
          pvVar2 = (void *)((long)pvVar2 + 1);
          FUN_00108e20(pvVar4,pvVar2,0x2d);
          pvVar4 = pvVar2;
        } while (pvVar2 < param_1);
      }
    }
    cVar8 = DAT_0012a803;
    if (DAT_0012a802 == '\0') {
      if (DAT_0012a803 != '\0') {
LAB_00109278:
        DAT_0012a688 = 1;
        DAT_0012a7a8 = 0;
        DAT_0012a7b0 = DAT_0012a7b0 + -1;
        DAT_0012a828 = (long)param_2 + (DAT_0012a830 - DAT_0012a840);
        return;
      }
      FUN_00108e20(param_1,param_2,0x3a);
      DAT_0012a7b0 = DAT_0012a7b0 + -1;
      pvVar4 = param_2;
      cVar8 = DAT_0012a803;
      goto LAB_001091bc;
    }
  }
  else {
    cVar8 = DAT_0012a802;
    if (DAT_0012a802 == '\0') goto LAB_00109278;
  }
  lVar7 = 0;
  lVar6 = DAT_0012a7b0;
  if (pvVar4 < param_2) {
    for (; lVar7 < lVar6; lVar7 = lVar7 + 1) {
      pvVar2 = rawmemchr(pvVar4,__c);
      pvVar2 = (void *)((long)pvVar2 + 1);
      if (cVar8 == '\0') {
        FUN_00108e20(pvVar4,pvVar2,0x3a);
        lVar6 = DAT_0012a7b0;
        cVar8 = DAT_0012a803;
      }
      pvVar4 = pvVar2;
      if (param_2 <= pvVar2) {
        DAT_0012a7b0 = lVar6 - (lVar7 + 1);
        goto LAB_001091bc;
      }
    }
    DAT_0012a7b0 = lVar6 - lVar7;
  }
LAB_001091bc:
  DAT_0012a7a8 = 0;
  if ((cVar8 == '\0') && (DAT_0012a7a8 = DAT_0012a7f0, (long)DAT_0012a7f0 < 0)) {
    DAT_0012a7a8 = 0;
  }
  DAT_0012a688 = 1;
  DAT_0012a828 = (long)pvVar4 + (DAT_0012a830 - DAT_0012a840);
  return;
}




// Function: grepbuf @ 0x9350

long grepbuf(ulong param_1,ulong param_2)

{
  long lVar1;
  long lVar2;
  ulong uVar3;
  ulong uVar4;
  int __status;
  ulong uVar5;
  long in_FS_OFFSET;
  bool bVar6;
  long local_38;
  long local_30;
  
  lVar2 = DAT_0012a7b0;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 < param_2) {
    do {
      lVar1 = (*DAT_0012a878)(DAT_0012a870,param_1,param_2 - param_1,&local_38,0);
      if (lVar1 < 0) {
        if (DAT_0012a802 == '\0') {
LAB_00109429:
          lVar2 = lVar2 - DAT_0012a7b0;
          goto LAB_0010942f;
        }
        local_38 = 0;
        uVar4 = param_2;
        uVar5 = param_1;
        uVar3 = param_2;
        if (param_1 < param_2) goto LAB_001093a9;
        break;
      }
      uVar5 = param_1 + lVar1;
      uVar3 = local_38 + uVar5;
      if (DAT_0012a802 == '\0') {
        uVar4 = uVar3;
        if (param_2 == uVar5) goto LAB_00109429;
LAB_001093a9:
        FUN_00109070(uVar5,uVar4);
        if ((DAT_0012a7b0 == 0) || (DAT_0012a7a2 != '\0')) {
          if (DAT_0012a7a1 != '\0') {
            __status = 0;
            if (DAT_0012a889 != '\0') {
              __status = DAT_0012a080;
            }
                    /* WARNING: Subroutine does not return */
            exit(__status);
          }
          goto LAB_00109429;
        }
      }
      else {
        bVar6 = param_1 < uVar5;
        uVar4 = uVar5;
        uVar5 = param_1;
        if (bVar6) goto LAB_001093a9;
      }
      param_1 = uVar3;
    } while (uVar3 < param_2);
    lVar2 = lVar2 - DAT_0012a7b0;
  }
  else {
    lVar2 = 0;
  }
LAB_0010942f:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar2;
}




// Function: suppressible_error @ 0x94a0

void suppressible_error(undefined4 param_1)

{
  undefined8 uVar1;
  
  if (DAT_0012a904 != '\0') {
    DAT_0012a889 = 1;
    return;
  }
  uVar1 = FUN_00108a50();
                    /* WARNING: Subroutine does not return */
  error(0,param_1,&DAT_001209f1,uVar1);
}




// Function: file_must_have_nulls @ 0x94e0

bool file_must_have_nulls(__off_t param_1,int param_2,long param_3)

{
  char cVar1;
  __off_t _Var2;
  __off_t _Var3;
  int *piVar4;
  long extraout_RDX;
  
  if (DAT_0012a881 != '\0') {
    return false;
  }
  cVar1 = FUN_00108550();
  if ((((cVar1 != '\0') && (param_1 < *(long *)(extraout_RDX + 0x30))) &&
      ((param_2 != 0 || (param_1 = lseek(0,0,1), -1 < param_1)))) &&
     (_Var2 = lseek(param_2,param_1,4), -1 < _Var2)) {
    _Var3 = lseek(param_2,param_1,0);
    if (_Var3 < 0) {
      piVar4 = __errno_location();
      FUN_001094a0(*piVar4);
    }
    return _Var2 < *(long *)(param_3 + 0x30);
  }
  return false;
}




// Function: finalize_input @ 0x95a0

void finalize_input(int param_1,long param_2,char param_3)

{
  char cVar1;
  __off_t _Var2;
  int *piVar3;
  
  if (param_1 != 0) {
    return;
  }
  if (DAT_0012a7b0 == 0) {
    if (((DAT_0012a830 != DAT_0012a828) && (DAT_0012a881 == '\0')) &&
       (_Var2 = lseek(0,DAT_0012a828,0), _Var2 < 0)) goto LAB_00109607;
  }
  else if ((param_3 == '\0') &&
          (((DAT_0012a881 != '\0' ||
            ((_Var2 = lseek(0,0,2), _Var2 < 0 && (piVar3 = __errno_location(), *piVar3 != 0x16))))
           && (cVar1 = FUN_00108490(*(undefined4 *)(param_2 + 0x18)), cVar1 == '\0')))) {
LAB_00109607:
    piVar3 = __errno_location();
    FUN_001094a0(*piVar3);
    return;
  }
  return;
}




// Function: grep @ 0x9710

long grep(undefined4 param_1,long param_2,undefined1 *param_3)

{
  char cVar1;
  char cVar2;
  char cVar3;
  char cVar4;
  uint uVar5;
  void *pvVar6;
  long lVar7;
  int *piVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  ulong uVar11;
  int iVar12;
  long lVar13;
  char *pcVar14;
  char *pcVar16;
  char *pcVar17;
  bool bVar18;
  long local_70;
  long local_68;
  char local_59;
  long local_58;
  char *pcVar15;
  
  cVar3 = DAT_0012a803;
  cVar2 = DAT_0012a7a2;
  cVar1 = DAT_0012a684;
  cVar4 = FUN_00109630();
  if (cVar4 != '\0') {
    DAT_0012a7c8 = 0;
    DAT_0012a7b8 = (char *)0x0;
    DAT_0012a7b0 = DAT_0012a7e0;
    DAT_0012a821 = cVar1 == '\0' & DAT_0012a820;
    DAT_0012a818 = 0;
    DAT_0012a828 = 0;
    DAT_0012a7a8 = 0;
    DAT_0012a888 = '\0';
    uVar5 = FUN_00108560(0,param_2);
    uVar11 = (ulong)uVar5;
    if ((char)uVar5 != '\0') {
      DAT_0012a8f8 = 0;
      if (DAT_0012a8fc != '\0') {
        lVar7 = 0x7fffffffffffffff;
        cVar4 = FUN_00108550();
        if ((cVar4 != '\0') && (lVar7 = *(long *)(param_2 + 0x30), DAT_0012a801 != '\0')) {
          lVar7 = lVar7 + (ulong)(lVar7 != 0x7fffffffffffffff);
        }
        DAT_0012a8f8 = 0;
        do {
          DAT_0012a8f8 = DAT_0012a8f8 + 1;
          lVar7 = lVar7 / 10;
        } while (lVar7 != 0);
      }
      local_59 = '\0';
      uVar11 = uVar11 & 0xffffffff;
      lVar7 = 0;
      local_68 = -1;
      local_58 = 0;
      local_70 = 0;
      do {
        pcVar14 = DAT_0012a848;
        pcVar17 = DAT_0012a840;
        iVar12 = DAT_0012a810;
        if (((local_68 < 0) && (cVar1 != '\0')) && (DAT_0012a810 != 1)) {
          lVar13 = (long)DAT_0012a840 - (long)DAT_0012a848;
          cVar4 = FUN_00107890(DAT_0012a848,lVar13);
          if ((cVar4 != '\0') ||
             (((char)uVar11 != '\0' &&
              (cVar4 = FUN_001094e0(lVar13,param_1,param_2), pcVar14 = DAT_0012a848,
              pcVar17 = DAT_0012a840, iVar12 = DAT_0012a810, cVar4 != '\0')))) {
            if (iVar12 == 2) {
              return 0;
            }
            if (DAT_0012a7e8 == '\0') {
              DAT_0012a803 = '\x01';
              DAT_0012a7a2 = '\x01';
            }
            DAT_0012a821 = DAT_0012a820;
            local_68 = local_58;
            pcVar14 = DAT_0012a848;
            pcVar17 = DAT_0012a840;
            local_59 = cVar1;
          }
        }
        if (DAT_0012a7b8 != (char *)0x0) {
          DAT_0012a7b8 = pcVar14;
        }
        pcVar15 = pcVar14 + lVar7;
        DAT_0012a7c0 = pcVar14;
        if (pcVar15 == pcVar17) {
          *param_3 = 1;
          if (local_70 != 0) {
            DAT_0012a840 = pcVar17 + 1;
            bVar18 = DAT_0012a7b0 != 0;
            *pcVar17 = cVar1;
            if (bVar18) {
              lVar7 = FUN_00109350(pcVar14 + (lVar7 - local_70));
              local_58 = local_58 + lVar7;
            }
            if (DAT_0012a7a8 != 0) {
              FUN_00108ff0(DAT_0012a840);
            }
          }
          goto LAB_001099f0;
        }
        FUN_001078b0(pcVar15,pcVar17,(int)local_59);
        cVar4 = pcVar15[-1];
        pcVar15[-1] = cVar1;
        pvVar6 = memrchr(pcVar15 + -1,(int)cVar1,(size_t)(pcVar17 + (1 - (long)pcVar15)));
        pcVar15[-1] = cVar4;
        pcVar16 = pcVar15 + -local_70;
        pcVar14 = (char *)((long)pvVar6 + 1);
        if (pcVar15 == pcVar14) {
          local_70 = (long)pcVar17 - (long)pcVar16;
          pcVar14 = pcVar16;
        }
        else {
          local_70 = (long)pcVar17 - (long)pcVar14;
          if (pcVar16 < pcVar14) {
            if (DAT_0012a7b0 == 0) {
              if (DAT_0012a7a8 == 0) goto LAB_001099f0;
LAB_0010991a:
              FUN_00108ff0(pcVar14);
              if (DAT_0012a7b0 == 0 && DAT_0012a7a8 == 0) goto LAB_001099f0;
            }
            else {
              lVar7 = FUN_00109350(pcVar16,pcVar14);
              local_58 = local_58 + lVar7;
              if (DAT_0012a7a8 != 0) goto LAB_0010991a;
              if (DAT_0012a7b0 == 0) goto LAB_001099f0;
            }
            if (DAT_0012a7a2 != '\0') {
              lVar7 = 0;
              if (-1 < local_68) {
                lVar7 = local_68;
              }
              if (lVar7 < local_58) goto LAB_001099f0;
            }
          }
        }
        if (0 < DAT_0012a7f8) {
          lVar7 = 0;
          do {
            if (pcVar14 <= DAT_0012a848) break;
            if (pcVar14 == DAT_0012a7b8) goto LAB_001099af;
            lVar7 = lVar7 + 1;
            pcVar15 = pcVar14;
            do {
              pcVar14 = pcVar15 + -1;
              pcVar16 = pcVar15 + -2;
              pcVar15 = pcVar14;
            } while (*pcVar16 != cVar1);
          } while (lVar7 != DAT_0012a7f8);
        }
        if (pcVar14 != DAT_0012a7b8) {
          DAT_0012a7b8 = (char *)0x0;
        }
LAB_001099af:
        lVar7 = (long)pcVar17 - (long)pcVar14;
        if (DAT_0012a800 != '\0') {
          DAT_0012a7c8 = FUN_00107860(DAT_0012a7c8,DAT_0012a840 + (-lVar7 - (long)DAT_0012a848));
        }
        if (DAT_0012a801 != '\0') {
          FUN_00108250(pcVar14);
        }
        cVar4 = FUN_00108560(lVar7,param_2);
        uVar11 = 0;
      } while (cVar4 != '\0');
      piVar8 = __errno_location();
      FUN_001094a0(*piVar8);
LAB_001099f0:
      if (DAT_0012a810 != 0) {
        DAT_0012a7a2 = cVar2;
        DAT_0012a803 = cVar3;
        return local_58;
      }
      if (cVar3 != '\0') {
        DAT_0012a7a2 = cVar2;
        DAT_0012a803 = cVar3;
        return local_58;
      }
      if (DAT_0012a888 == '\0') {
        if (local_68 < 0) {
          DAT_0012a7a2 = cVar2;
          DAT_0012a803 = cVar3;
          return local_58;
        }
        if (local_58 <= local_68) {
          DAT_0012a7a2 = cVar2;
          DAT_0012a803 = cVar3;
          return local_58;
        }
      }
      DAT_0012a7a2 = cVar2;
      DAT_0012a803 = cVar3;
      uVar9 = FUN_00108a50();
      uVar10 = dcgettext(0,"%s: binary file matches",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar10,uVar9);
    }
    piVar8 = __errno_location();
    FUN_001094a0(*piVar8);
  }
  return 0;
}




// Function: grepdesc @ 0x9c70

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulong grepdesc(int param_1,byte param_2)

{
  char cVar1;
  int iVar2;
  long lVar3;
  int *piVar4;
  long lVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  uint uVar8;
  undefined8 unaff_R13;
  ulong uVar9;
  long in_FS_OFFSET;
  undefined1 local_d9;
  undefined8 local_d8;
  undefined8 local_d0;
  stat local_c8;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_d9 = 0;
  iVar2 = fstat(param_1,&local_c8);
  if (iVar2 == 0) {
    if (param_1 == 0) {
      if ((int)DAT_0012a808 < 0) {
        DAT_0012a808 = (uint)((local_c8.st_mode & 0xf000) == 0x4000);
      }
LAB_00109cca:
      if ((((DAT_0012a803 == '\0') && (DAT_0012a80c == 0)) && (1 < DAT_0012a7e0)) &&
         ((((local_c8.st_mode & 0xf000) == 0x8000 && (local_c8.st_ino == DAT_0012a928)) &&
          (local_c8.st_dev == _DAT_0012a920)))) {
        if (DAT_0012a904 == '\0') {
          uVar6 = FUN_00108a50();
          uVar7 = dcgettext(0,"%s: input file is also the output",5);
                    /* WARNING: Subroutine does not return */
          error(0,0,uVar7,uVar6);
        }
        DAT_0012a889 = 1;
        uVar9 = 1;
      }
      else {
        lVar3 = FUN_00109710(param_1,&local_c8,&local_d9);
        if (DAT_0012a7e8 != '\0') {
          if (DAT_0012a808 != 0) {
            FUN_00108a90();
            if (DAT_0012a804 == 0) {
              FUN_00108410(0);
            }
            else {
              FUN_001089b0(0x3a);
            }
          }
          FUN_00108330(&DAT_0011f127,lVar3);
          if (DAT_0012a7d8 != '\0') {
            FUN_00108430();
          }
        }
        uVar9 = CONCAT71((int7)((ulong)unaff_R13 >> 8),lVar3 == 0);
        if (DAT_0012a80c == 0) {
          FUN_001095a0(param_1,&local_c8,local_d9);
        }
        else if (DAT_0012a80c == (lVar3 == 0) + 1) {
          FUN_00108a90();
          FUN_00108410(DAT_0012a804 & 10);
          if (DAT_0012a7d8 != '\0') {
            FUN_00108430();
          }
        }
      }
      goto LAB_00109d70;
    }
    cVar1 = FUN_00107500(param_2);
    if (((cVar1 == '\0') || (cVar1 = FUN_001074d0(local_c8.st_mode), cVar1 == '\0')) &&
       ((param_2 == 0 ||
        (cVar1 = FUN_001081e0(DAT_0012a890,1,(local_c8.st_mode & 0xf000) == 0x4000), cVar1 == '\0'))
       )) {
      if ((int)DAT_0012a808 < 0) {
        DAT_0012a808 = (uint)((local_c8.st_mode & 0xf000) == 0x4000);
        if (DAT_0012a018 != 3) goto LAB_00109e01;
LAB_00109ed9:
        if ((local_c8.st_mode & 0xf000) == 0x4000) {
          uVar8 = ~(uint)(param_2 ^ 1) & _DAT_0012a014;
          iVar2 = close(param_1);
          if (iVar2 != 0) {
            piVar4 = __errno_location();
            FUN_001094a0(*piVar4);
          }
          local_d0 = 0;
          local_d8 = DAT_0012a890;
          lVar3 = FUN_00116c90(&local_d8,uVar8,0);
          if (lVar3 == 0) {
                    /* WARNING: Subroutine does not return */
            FUN_0011d4b0();
          }
          uVar9 = 1;
          while (lVar5 = FUN_001171f0(lVar3), lVar5 != 0) {
            uVar8 = FUN_0010a180(lVar3,lVar5,param_2);
            uVar9 = (ulong)((uint)uVar9 & uVar8);
          }
          piVar4 = __errno_location();
          if (*piVar4 != 0) {
            FUN_001094a0();
          }
          iVar2 = FUN_00117000(lVar3);
          if (iVar2 != 0) {
            FUN_001094a0(*piVar4);
          }
          goto LAB_00109d78;
        }
      }
      else {
        if (DAT_0012a018 == 3) goto LAB_00109ed9;
LAB_00109e01:
        if ((DAT_0012a018 == 4) && ((local_c8.st_mode & 0xf000) == 0x4000)) goto LAB_00109e36;
      }
      if (((DAT_0012a884 != 2) && ((DAT_0012a884 != 0 || (param_2 != 0)))) ||
         (cVar1 = FUN_001074d0(local_c8.st_mode), cVar1 == '\0')) goto LAB_00109cca;
    }
LAB_00109e36:
    uVar9 = 1;
  }
  else {
    piVar4 = __errno_location();
    uVar9 = 1;
    FUN_001094a0(*piVar4);
LAB_00109d70:
    if (param_1 == 0) goto LAB_00109d78;
  }
  iVar2 = close(param_1);
  if (iVar2 != 0) {
    piVar4 = __errno_location();
    FUN_001094a0(*piVar4);
  }
LAB_00109d78:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar9 & 0xffffffff;
}




// Function: grepfile @ 0xa0f0

undefined8 grepfile(uint param_1,undefined8 param_2,char param_3,undefined1 param_4)

{
  int iVar1;
  undefined8 uVar2;
  int *piVar3;
  ulong uVar4;
  
  uVar4 = (ulong)param_1;
  FUN_00107500(param_4);
  if (param_3 == '\0') {
    iVar1 = FUN_00119570(uVar4 & 0xffffffff);
    if (iVar1 < 0) {
      piVar3 = __errno_location();
      uVar2 = FUN_00107720(*piVar3);
      if ((char)uVar2 != '\0') {
        return uVar2;
      }
      FUN_001094a0();
      return 1;
    }
  }
  else {
    iVar1 = FUN_00119570(uVar4 & 0xffffffff);
    if (iVar1 < 0) {
      piVar3 = __errno_location();
      FUN_001094a0(*piVar3);
      return 1;
    }
  }
  uVar2 = FUN_00109c70(iVar1,param_4);
  return uVar2;
}




// Function: grepdirent @ 0xa180

ulong grepdirent(long param_1,long param_2,uint param_3)

{
  short sVar1;
  __mode_t _Var2;
  long lVar3;
  char cVar4;
  uint uVar5;
  int iVar6;
  undefined8 uVar7;
  int *piVar8;
  uint uVar9;
  ulong uVar10;
  long in_FS_OFFSET;
  stat sStack_d8;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  sVar1 = *(short *)(param_2 + 0x68);
  param_3 = *(long *)(param_2 + 0x58) == 0 & param_3;
  if (sVar1 == 6) goto switchD_0010a21b_caseD_c;
  if ((char)param_3 == '\0') {
    uVar9 = FUN_001081e0(param_2 + 0x100,0,(ushort)(sVar1 - 1U) < 2 || sVar1 == 4);
    uVar10 = (ulong)uVar9;
    if ((char)uVar9 == '\0') {
      sVar1 = *(short *)(param_2 + 0x68);
      goto LAB_0010a1ca;
    }
    FUN_00117830(param_1,param_2,4);
    goto LAB_0010a246;
  }
LAB_0010a1ca:
  DAT_0012a890 = *(long *)(param_2 + 0x38);
  if ((DAT_0012a88a != '\0') && (*(char *)(DAT_0012a890 + 1) != '\0')) {
    DAT_0012a890 = DAT_0012a890 + 2;
  }
  lVar3 = DAT_0012a890;
  uVar9 = 1;
  if ((*(uint *)(param_1 + 0x48) & 2) == 0) {
    uVar9 = *(uint *)(param_1 + 0x48) & param_3;
  }
  switch(sVar1) {
  default:
                    /* WARNING: Subroutine does not return */
    abort();
  case 1:
    if (DAT_0012a018 == 3) goto switchD_0010a21b_caseD_c;
    FUN_00117830(param_1,param_2,4);
    break;
  case 2:
    if (DAT_0012a904 == '\0') {
      uVar7 = dcgettext(0,"%s: warning: recursive directory loop",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar7,lVar3);
    }
  case 0xc:
  case 0xe:
switchD_0010a21b_caseD_c:
    uVar10 = 1;
LAB_0010a246:
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      return uVar10;
    }
    goto LAB_0010a3df;
  case 3:
  case 0xb:
    uVar5 = FUN_00107500(param_3);
    uVar10 = (ulong)uVar5;
    if ((char)uVar5 != '\0') {
      _Var2 = *(__mode_t *)(param_2 + 0x88);
      if ((*(__mode_t *)(param_2 + 0x88) == 0) &&
         (iVar6 = fstatat(*(int *)(param_1 + 0x2c),*(char **)(param_2 + 0x30),&sStack_d8,
                          uVar9 << 8 ^ 0x100), _Var2 = sStack_d8.st_mode, iVar6 != 0)) {
        piVar8 = __errno_location();
        FUN_001094a0(*piVar8);
      }
      else {
        cVar4 = FUN_001074d0(_Var2);
        if (cVar4 == '\0') break;
      }
      goto LAB_0010a246;
    }
    break;
  case 4:
  case 7:
  case 10:
    uVar10 = 1;
    FUN_001094a0(*(undefined4 *)(param_2 + 0x40));
    goto LAB_0010a246;
  case 8:
  case 0xd:
    break;
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    uVar10 = FUN_0010a0f0(*(undefined4 *)(param_1 + 0x2c),*(undefined8 *)(param_2 + 0x30),uVar9,
                          param_3);
    return uVar10;
  }
LAB_0010a3df:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: grep_command_line_arg @ 0xa3f0

void grep_command_line_arg(char *param_1)

{
  int iVar1;
  
  iVar1 = strcmp(param_1,"-");
  if (iVar1 == 0) {
    DAT_0012a890 = (char *)DAT_0012a7d0;
    FUN_00109c70(0,1);
    return;
  }
  DAT_0012a890 = param_1;
  FUN_0010a0f0(0xffffff9c,param_1,1,1);
  return;
}




// Function: usage @ 0xa440

void usage(int param_1)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  uVar1 = FUN_001179e0();
  if (param_1 == 0) {
    uVar2 = dcgettext(0,"Usage: %s [OPTION]... PATTERNS [FILE]...\n",5);
    __printf_chk(1,uVar2,uVar1);
    uVar1 = dcgettext(0,"Search for PATTERNS in each FILE.\n",5);
    __printf_chk(1,uVar1);
    uVar1 = FUN_001179e0();
    uVar2 = dcgettext(0,
                      "Example: %s -i \'hello world\' menu.h main.c\nPATTERNS can contain multiple patterns separated by newlines.\n\nPattern selection and interpretation:\n"
                      ,5);
    __printf_chk(1,uVar2,uVar1);
    uVar1 = dcgettext(0,
                      "  -E, --extended-regexp     PATTERNS are extended regular expressions\n  -F, --fixed-strings       PATTERNS are strings\n  -G, --basic-regexp        PATTERNS are basic regular expressions\n  -P, --perl-regexp         PATTERNS are Perl regular expressions\n"
                      ,5);
    __printf_chk(1,uVar1);
    uVar1 = dcgettext(0,
                      "  -e, --regexp=PATTERNS     use PATTERNS for matching\n  -f, --file=FILE           take PATTERNS from FILE\n  -i, --ignore-case         ignore case distinctions in patterns and data\n      --no-ignore-case      do not ignore case distinctions (default)\n  -w, --word-regexp         match only whole words\n  -x, --line-regexp         match only whole lines\n  -z, --null-data           a data line ends in 0 byte, not newline\n"
                      ,5);
    __printf_chk(1,uVar1);
    uVar1 = dcgettext(0,
                      "\nMiscellaneous:\n  -s, --no-messages         suppress error messages\n  -v, --invert-match        select non-matching lines\n  -V, --version             display version information and exit\n      --help                display this help text and exit\n"
                      ,5);
    __printf_chk(1,uVar1);
    uVar1 = dcgettext(0,
                      "\nOutput control:\n  -m, --max-count=NUM       stop after NUM selected lines\n  -b, --byte-offset         print the byte offset with output lines\n  -n, --line-number         print line number with output lines\n      --line-buffered       flush output on every line\n  -H, --with-filename       print file name with output lines\n  -h, --no-filename         suppress the file name prefix on output\n      --label=LABEL         use LABEL as the standard input file name prefix\n"
                      ,5);
    __printf_chk(1,uVar1);
    uVar1 = dcgettext(0,
                      "  -o, --only-matching       show only nonempty parts of lines that match\n  -q, --quiet, --silent     suppress all normal output\n      --binary-files=TYPE   assume that binary files are TYPE;\n                            TYPE is \'binary\', \'text\', or \'without-match\'\n  -a, --text                equivalent to --binary-files=text\n"
                      ,5);
    __printf_chk(1,uVar1);
    uVar1 = dcgettext(0,
                      "  -I                        equivalent to --binary-files=without-match\n  -d, --directories=ACTION  how to handle directories;\n                            ACTION is \'read\', \'recurse\', or \'skip\'\n  -D, --devices=ACTION      how to handle devices, FIFOs and sockets;\n                            ACTION is \'read\' or \'skip\'\n  -r, --recursive           like --directories=recurse\n  -R, --dereference-recursive  likewise, but follow all symlinks\n"
                      ,5);
    __printf_chk(1,uVar1);
    uVar1 = dcgettext(0,
                      "      --include=GLOB        search only files that match GLOB (a file pattern)\n      --exclude=GLOB        skip files that match GLOB\n      --exclude-from=FILE   skip files that match any file pattern from FILE\n      --exclude-dir=GLOB    skip directories that match GLOB\n"
                      ,5);
    __printf_chk(1,uVar1);
    uVar1 = dcgettext(0,
                      "  -L, --files-without-match  print only names of FILEs with no selected lines\n  -l, --files-with-matches  print only names of FILEs with selected lines\n  -c, --count               print only a count of selected lines per FILE\n  -T, --initial-tab         make tabs line up (if needed)\n  -Z, --null                print 0 byte after FILE name\n"
                      ,5);
    __printf_chk(1,uVar1);
    uVar1 = dcgettext(0,
                      "\nContext control:\n  -B, --before-context=NUM  print NUM lines of leading context\n  -A, --after-context=NUM   print NUM lines of trailing context\n  -C, --context=NUM         print NUM lines of output context\n"
                      ,5);
    __printf_chk(1,uVar1);
    uVar1 = dcgettext(0,
                      "  -NUM                      same as --context=NUM\n      --group-separator=SEP  print SEP on line between matches with context\n      --no-group-separator  do not print separator for matches with context\n      --color[=WHEN],\n      --colour[=WHEN]       use markers to highlight the matching strings;\n                            WHEN is \'always\', \'never\', or \'auto\'\n  -U, --binary              do not strip CR characters at EOL (MSDOS/Windows)\n\n"
                      ,5);
    __printf_chk(1,uVar1);
    uVar1 = dcgettext(0,
                      "When FILE is \'-\', read standard input.  With no FILE, read \'.\' if\nrecursive, \'-\' otherwise.  With fewer than two FILEs, assume -h.\nExit status is 0 if any line is selected, 1 otherwise;\nif any error occurs and -q is not given, the exit status is 2.\n"
                      ,5);
    __printf_chk(1,uVar1);
    FUN_0011d030();
  }
  else {
    uVar2 = dcgettext(0,"Usage: %s [OPTION]... PATTERNS [FILE]...\n",5);
    __fprintf_chk(stderr,1,uVar2,uVar1);
    uVar1 = FUN_001179e0();
    uVar2 = dcgettext(0,"Try \'%s --help\' for more information.\n",5);
    __fprintf_chk(stderr,1,uVar2,uVar1);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: fgrep_to_grep_pattern @ 0xa6b0

void fgrep_to_grep_pattern(undefined8 *param_1,size_t *param_2)

{
  byte bVar1;
  byte *pbVar2;
  size_t sVar3;
  byte *__src;
  byte *pbVar4;
  size_t sVar5;
  size_t sVar6;
  long in_FS_OFFSET;
  undefined8 local_48;
  long local_40;
  
  sVar6 = *param_2;
  __src = (byte *)*param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_48 = 0;
  pbVar2 = (byte *)FUN_0011d210(sVar6 + 1,2);
  pbVar4 = pbVar2;
  if (sVar6 == 0) {
    sVar6 = 0;
  }
  else {
    do {
      while( true ) {
        sVar3 = FUN_0010c9f0(__src,sVar6,&local_48);
        if (sVar3 != 0xffffffffffffffff) break;
        local_48 = 0;
LAB_0010a767:
        bVar1 = *__src;
        if (((byte)(bVar1 - 0x24) < 0x3b) &&
           ((0x580000000000441U >> ((ulong)(bVar1 - 0x24) & 0x3f) & 1) != 0)) {
          *pbVar4 = 0x5c;
          bVar1 = *__src;
          pbVar4 = pbVar4 + 1;
        }
        sVar6 = sVar6 - 1;
        *pbVar4 = bVar1;
        pbVar4 = pbVar4 + 1;
        __src = __src + 1;
        if (sVar6 == 0) goto LAB_0010a7ab;
      }
      if (sVar3 == 1) goto LAB_0010a767;
      sVar5 = 0;
      if (sVar3 != 0xfffffffffffffffe) {
        sVar5 = sVar6 - sVar3;
        sVar6 = sVar3;
      }
      pbVar4 = mempcpy(pbVar4,__src,sVar6);
      __src = __src + sVar6;
      sVar6 = sVar5;
    } while (sVar5 != 0);
LAB_0010a7ab:
    sVar6 = (long)pbVar4 - (long)pbVar2;
  }
  *pbVar4 = 10;
  free((void *)*param_1);
  *param_1 = pbVar2;
  *param_2 = sVar6;
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: Fcompile @ 0xa820

void Fcompile(char *param_1,long param_2)

{
  undefined8 uVar1;
  void *pvVar2;
  char *__ptr;
  undefined8 uVar3;
  undefined8 *puVar4;
  char cVar5;
  size_t __n;
  char *__s;
  long in_FS_OFFSET;
  size_t local_48;
  long local_40;
  
  __ptr = (char *)0x0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_48 = 0;
  uVar1 = FUN_0010ca70(1);
  __s = param_1;
  do {
    pvVar2 = rawmemchr(__s,10);
    __n = (long)pvVar2 - (long)__s;
    if (DAT_0012a685 != '\0') {
      if ((param_1 < __s) && (DAT_0012a684 == '\n')) {
        __s = __s + -1;
      }
      else {
        cVar5 = DAT_0012a684;
        if ((long)local_48 <= (long)(__n + 1)) {
          free(__ptr);
          local_48 = __n;
          __ptr = (char *)FUN_0011d2b0(0,&local_48,2,0xffffffffffffffff,1);
          cVar5 = DAT_0012a684;
          *__ptr = DAT_0012a684;
        }
        memcpy(__ptr + 1,__s,__n);
        __ptr[__n + 1] = cVar5;
        __s = __ptr;
      }
      __n = __n + 2;
    }
    FUN_0010bab0(uVar1,__s,__n);
    __s = (char *)((long)pvVar2 + 1U);
  } while ((char *)((long)pvVar2 + 1U) <= param_1 + param_2);
  free(__ptr);
  uVar3 = FUN_0010bf90(uVar1);
  FUN_0010bfa0(uVar1);
  puVar4 = (undefined8 *)FUN_0011d0f0(0x28);
  *puVar4 = uVar1;
  puVar4[2] = param_1;
  puVar4[1] = uVar3;
  puVar4[3] = param_2;
  puVar4[4] = 0;
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: Fexecute @ 0xa9c0

long Fexecute(undefined8 *param_1,void *param_2,long param_3,long *param_4,void *param_5)

{
  undefined8 uVar1;
  char cVar2;
  uint uVar3;
  long lVar4;
  void *pvVar5;
  long lVar6;
  void *pvVar7;
  void *pvVar8;
  undefined8 uVar9;
  byte bVar10;
  byte bVar11;
  uint uVar12;
  void *pvVar13;
  long lVar14;
  long in_FS_OFFSET;
  void *local_88;
  long local_80;
  undefined1 local_78 [16];
  long local_68;
  undefined1 local_58 [16];
  long local_48;
  long local_40;
  
  cVar2 = DAT_0012a684;
  pvVar8 = (void *)((long)param_2 + param_3);
  uVar1 = *param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar3 = (uint)DAT_0012a685;
  bVar10 = DAT_0012a180 & ((DAT_0012a182 | DAT_0012a685) ^ 1);
  bVar11 = bVar10 | DAT_0012a686;
  uVar12 = uVar3 ^ 1;
  pvVar13 = param_2;
  if (param_5 != (void *)0x0) {
    pvVar13 = param_5;
  }
  local_88 = pvVar13;
  if (pvVar13 <= pvVar8) {
    while (lVar4 = FUN_0010c870(uVar1,(long)pvVar13 - (ulong)uVar3,
                                (long)pvVar8 + ((ulong)uVar3 - (long)pvVar13),local_78,
                                (bVar11 | param_5 != (void *)0x0) & uVar12), -1 < lVar4) {
      lVar4 = (long)pvVar13 + lVar4;
      local_80 = 0;
      lVar14 = local_68 - (int)((uint)DAT_0012a685 * 2);
      if ((bVar10 == 0) || (lVar6 = FUN_0010caf0(&local_88,&local_80,lVar4,pvVar8), lVar6 == 0)) {
        pvVar13 = local_88;
        if ((DAT_0012a686 == 1) || (param_5 == (void *)0x0)) {
          if (DAT_0012a685 != 0) {
            lVar14 = lVar14 + (ulong)(param_5 == (void *)0x0);
            goto LAB_0010acc0;
          }
          pvVar7 = pvVar8;
          if (DAT_0012a686 != 0) {
            if (local_80 == 0) {
              pvVar5 = memrchr(local_88,(int)cVar2,lVar4 - (long)local_88);
              if (pvVar5 != (void *)0x0) {
                pvVar13 = (void *)((long)pvVar5 + 1);
                local_88 = pvVar13;
              }
LAB_0010ab44:
              lVar6 = FUN_0010cca0(pvVar13,lVar4,pvVar8);
            }
            else {
              if (local_80 < 1) goto LAB_0010ab44;
              lVar6 = FUN_0010cc90(lVar4 - local_80,pvVar8);
            }
            if (lVar6 == 0) {
              while( true ) {
                pvVar13 = (void *)(lVar4 + lVar14);
                lVar6 = FUN_0010cc90(pvVar13,pvVar8);
                if (lVar6 == 0) {
                  if (param_5 != (void *)0x0) goto LAB_0010acc0;
                  goto LAB_0010acfd;
                }
                if ((param_5 == (void *)0x0) && (DAT_0012a180 == 0)) break;
                if ((lVar14 == 0) ||
                   (lVar6 = FUN_0010c870(uVar1,lVar4,lVar14 + -1,local_58,1), lVar14 = local_48,
                   lVar6 != 0)) goto LAB_0010ac20;
              }
              if (param_1[4] == 0) {
                FUN_0010a6b0(param_1 + 2,param_1 + 3);
                uVar9 = FUN_00106690(param_1[2],param_1[3],0x10a46,0);
                param_1[4] = uVar9;
              }
              if (pvVar13 < pvVar8) {
                pvVar7 = rawmemchr(pvVar13,(int)cVar2);
                pvVar7 = (void *)((long)pvVar7 + 1);
              }
              lVar14 = FUN_00106c90(param_1[4],lVar4,(long)pvVar7 - lVar4,param_4,0);
              if (-1 < lVar14) goto LAB_0010ad17;
              lVar4 = (long)pvVar7 - 1;
            }
LAB_0010ac20:
            lVar14 = FUN_0010cc80(lVar4,pvVar8);
            local_88 = (void *)(lVar4 + lVar14);
            pvVar13 = (void *)((long)local_88 + 1);
            if ((void *)((long)local_88 + 1) <= pvVar8) goto LAB_0010ac3c;
            break;
          }
          pvVar13 = (void *)(lVar4 + lVar14);
LAB_0010acfd:
          if (pvVar13 < pvVar8) {
            pvVar7 = rawmemchr(pvVar13,(int)cVar2);
            pvVar7 = (void *)((long)pvVar7 + 1);
          }
LAB_0010ad17:
          pvVar8 = memrchr(param_2,(int)cVar2,lVar4 - (long)param_2);
          if (pvVar8 == (void *)0x0) {
            lVar4 = 0;
          }
          else {
            lVar4 = ((long)pvVar8 + 1) - (long)param_2;
            param_2 = (void *)((long)pvVar8 + 1);
          }
          lVar14 = (long)pvVar7 - (long)param_2;
        }
        else {
LAB_0010acc0:
          lVar4 = lVar4 - (long)param_2;
        }
        *param_4 = lVar14;
        goto LAB_0010acd0;
      }
      pvVar13 = local_88;
      if (pvVar8 < local_88) break;
LAB_0010ac3c:
      uVar3 = (uint)DAT_0012a685;
    }
  }
  lVar4 = -1;
LAB_0010acd0:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: U @ 0xadc0

void U(char param_1)

{
  FUN_0010c9d0((int)param_1);
  return;
}




// Function: tr @ 0xadd0

uint tr(long param_1,uint param_2)

{
  byte bVar1;
  
  if (param_1 != 0) {
    bVar1 = FUN_0010adc0((int)(char)param_2);
    return (uint)*(byte *)(param_1 + (ulong)bVar1);
  }
  return param_2;
}




// Function: enqueue @ 0xae00

void enqueue(undefined8 *param_1,long *param_2)

{
  long lVar1;
  
  if (param_1 != (undefined8 *)0x0) {
    enqueue(*param_1);
    enqueue(param_1[1],param_2);
    lVar1 = param_1[2];
    *(long *)(*param_2 + 0x18) = lVar1;
    *param_2 = lVar1;
    return;
  }
  return;
}




// Function: treefails @ 0xae50

void treefails(undefined8 *param_1,long param_2,undefined8 param_3,char param_4)

{
  undefined8 *puVar1;
  byte *pbVar2;
  long *plVar3;
  long *plVar4;
  undefined8 *puVar5;
  
  if (param_1 == (undefined8 *)0x0) {
    return;
  }
  treefails(*param_1,param_2,param_3,param_4);
  treefails(param_1[1],param_2,param_3,param_4);
  do {
    if (param_2 == 0) {
      *(undefined8 *)(param_1[2] + 0x20) = param_3;
      return;
    }
    puVar5 = *(undefined8 **)(param_2 + 8);
    if (puVar5 != (undefined8 *)0x0) {
      do {
        pbVar2 = (byte *)(puVar5 + 3);
        if (*(byte *)(param_1 + 3) == *pbVar2) {
          plVar3 = (long *)param_1[2];
          plVar4 = (long *)puVar5[2];
          plVar3[4] = (long)plVar4;
          if (param_4 != '\0') {
            return;
          }
          if (*plVar4 == 0) {
            return;
          }
          if (*plVar3 != 0) {
            return;
          }
          *plVar3 = -1;
          return;
        }
        puVar1 = puVar5 + 1;
        puVar5 = (undefined8 *)*puVar5;
        if (*pbVar2 <= *(byte *)(param_1 + 3)) {
          puVar5 = (undefined8 *)*puVar1;
        }
      } while (puVar5 != (undefined8 *)0x0);
    }
    param_2 = *(long *)(param_2 + 0x20);
  } while( true );
}




// Function: treedelta @ 0xaf10

void treedelta(undefined8 *param_1,long param_2,long param_3)

{
  if (param_1 != (undefined8 *)0x0) {
    treedelta(*param_1);
    treedelta(param_1[1],param_2,param_3);
    if (param_2 < (long)(ulong)*(byte *)(param_3 + (ulong)*(byte *)(param_1 + 3))) {
      *(byte *)(param_3 + (ulong)*(byte *)(param_1 + 3)) = (byte)param_2;
    }
    return;
  }
  return;
}




// Function: hasevery @ 0xaf60

undefined8 hasevery(undefined8 *param_1,undefined8 *param_2)

{
  undefined8 *puVar1;
  byte *pbVar2;
  undefined8 uVar3;
  
  if (param_2 == (undefined8 *)0x0) {
    return 1;
  }
  uVar3 = hasevery(param_1,*param_2);
  if (((char)uVar3 != '\0') && (uVar3 = hasevery(param_1,param_2[1]), (char)uVar3 != '\0')) {
    if (param_1 != (undefined8 *)0x0) {
      do {
        pbVar2 = (byte *)(param_1 + 3);
        if (*(byte *)(param_2 + 3) == *pbVar2) {
          return uVar3;
        }
        puVar1 = param_1 + 1;
        param_1 = (undefined8 *)*param_1;
        if (*pbVar2 <= *(byte *)(param_2 + 3)) {
          param_1 = (undefined8 *)*puVar1;
        }
      } while (param_1 != (undefined8 *)0x0);
    }
    return 0;
  }
  return uVar3;
}




// Function: treenext @ 0xafe0

void treenext(undefined8 *param_1,long param_2)

{
  if (param_1 != (undefined8 *)0x0) {
    treenext(*param_1);
    treenext(param_1[1],param_2);
    *(undefined8 *)(param_2 + (ulong)*(byte *)(param_1 + 3) * 8) = param_1[2];
    return;
  }
  return;
}




// Function: bm_delta2_search @ 0xb030

undefined8
bm_delta2_search(ulong *param_1,ulong param_2,long param_3,long param_4,undefined8 param_5,char param_6,
            char param_7,long param_8,long param_9)

{
  char cVar1;
  char cVar2;
  byte bVar3;
  undefined8 uVar4;
  long lVar5;
  ulong uVar6;
  long lVar7;
  long local_58;
  
  uVar6 = *param_1;
  local_58 = 0;
  lVar5 = param_4;
  do {
    cVar1 = FUN_0010add0(param_5,(int)*(char *)(uVar6 - 2));
    if (cVar1 == param_7) {
      lVar7 = 2;
      do {
        lVar7 = lVar7 + 1;
        if (lVar5 < lVar7) {
          lVar7 = lVar5 + 1 + local_58;
          goto joined_r0x0010b124;
        }
        cVar1 = FUN_0010add0(param_5,(int)*(char *)(uVar6 + -lVar7));
        cVar2 = FUN_0010add0(param_5,(int)*(char *)(param_3 + -lVar7));
      } while (cVar1 == cVar2);
LAB_0010b167:
      lVar5 = lVar7 * 8 + -0x10;
    }
    else {
      lVar5 = 0;
      lVar7 = 2;
    }
    lVar5 = *(long *)(*(long *)(param_9 + 0x978) + lVar5);
    uVar6 = uVar6 + lVar5;
    if (param_2 < uVar6) goto LAB_0010b197;
    cVar1 = *(char *)(uVar6 - 1);
    cVar2 = FUN_0010add0(param_5,(int)cVar1);
    if (cVar2 != param_6) {
      if (param_8 != 0) {
        bVar3 = FUN_0010adc0((int)cVar1);
        uVar6 = uVar6 + *(byte *)(param_8 + (ulong)bVar3);
      }
LAB_0010b197:
      uVar4 = 0;
LAB_0010b199:
      *param_1 = uVar6;
      return uVar4;
    }
    local_58 = lVar7 + -1;
  } while( true );
joined_r0x0010b124:
  if (param_4 < lVar7) {
    uVar6 = uVar6 - param_4;
    uVar4 = 1;
    goto LAB_0010b199;
  }
  cVar1 = FUN_0010add0(param_5,(int)*(char *)(uVar6 + -lVar7));
  cVar2 = FUN_0010add0(param_5,(int)*(char *)(param_3 + -lVar7));
  if (cVar1 != cVar2) goto LAB_0010b167;
  lVar7 = lVar7 + 1;
  goto joined_r0x0010b124;
}




// Function: memchr_kwset @ 0xb1c0

char * memchr_kwset(char *param_1,long param_2,long param_3)

{
  int iVar1;
  byte bVar2;
  char *pcVar3;
  
  pcVar3 = param_1 + param_2;
  iVar1 = *(int *)(param_3 + 0x98c);
  if (iVar1 < 0) {
    if (param_1 < pcVar3) {
      do {
        bVar2 = FUN_0010adc0((int)*param_1);
        if (*(long *)(param_3 + 0x170 + (ulong)bVar2 * 8) != 0) {
          return param_1;
        }
        param_1 = param_1 + 1;
      } while (pcVar3 != param_1);
    }
  }
  else {
    while (param_1 < pcVar3) {
      bVar2 = FUN_0010adc0((int)*param_1);
      if (*(long *)(param_3 + 0x170 + (ulong)bVar2 * 8) != 0) {
        return param_1;
      }
      param_1 = param_1 + 1;
      if (((ulong)param_1 & 0xf) == 0) {
        pcVar3 = (char *)FUN_001190f0(param_1,*(undefined4 *)(param_3 + 0x988),iVar1,
                                      (long)pcVar3 - (long)param_1);
        return pcVar3;
      }
    }
  }
  return (char *)0x0;
}




// Function: acexec_trans @ 0xb290

long acexec_trans(long param_1,char *param_2,long param_3,long *param_4,char param_5)

{
  undefined8 *puVar1;
  byte *pbVar2;
  char cVar3;
  int iVar4;
  undefined8 uVar5;
  long lVar6;
  long *plVar7;
  byte bVar8;
  char *pcVar9;
  undefined8 *puVar10;
  long lVar11;
  char *pcVar12;
  char *pcVar13;
  long *plVar14;
  ulong uVar15;
  long *plVar16;
  
  if (param_3 < *(long *)(param_1 + 0x68)) {
    return -1;
  }
  plVar14 = *(long **)(param_1 + 0x60);
  uVar5 = *(undefined8 *)(param_1 + 0x980);
  lVar11 = *plVar14;
  pcVar13 = param_2 + param_3;
  plVar16 = plVar14;
  pcVar12 = param_2;
  if (lVar11 == 0) {
    iVar4 = *(int *)(param_1 + 0x988);
    pcVar9 = param_2;
LAB_0010b2e6:
    pcVar12 = pcVar9;
    if (iVar4 < 0) {
      while( true ) {
        cVar3 = *pcVar12;
        pcVar12 = pcVar12 + 1;
        bVar8 = FUN_0010add0(uVar5,(int)cVar3);
        plVar14 = *(long **)(param_1 + 0x170 + (ulong)bVar8 * 8);
        if (plVar14 != (long *)0x0) break;
        if (pcVar13 <= pcVar12) {
          return -1;
        }
      }
    }
    else {
      pcVar9 = (char *)FUN_0010b1c0(pcVar9,(long)pcVar13 - (long)pcVar9,param_1);
      if (pcVar9 == (char *)0x0) {
        return -1;
      }
      pcVar12 = pcVar9 + 1;
      bVar8 = FUN_0010add0(uVar5,(int)*pcVar9);
      plVar14 = *(long **)(param_1 + 0x170 + (ulong)bVar8 * 8);
    }
    lVar11 = *plVar14;
    plVar16 = plVar14;
    if (lVar11 == 0) {
      if (pcVar13 <= pcVar12) {
        return -1;
      }
      do {
        pcVar9 = pcVar12 + 1;
        bVar8 = FUN_0010add0(uVar5,(int)*pcVar12);
        puVar10 = (undefined8 *)plVar14[1];
        while (pbVar2 = (byte *)(puVar10 + 3), *pbVar2 != bVar8) {
          puVar1 = puVar10 + 1;
          puVar10 = (undefined8 *)*puVar10;
          if (*pbVar2 <= bVar8) {
            puVar10 = (undefined8 *)*puVar1;
          }
          if (puVar10 == (undefined8 *)0x0) {
            plVar14 = (long *)plVar14[4];
            if (plVar14 == (long *)0x0) {
              plVar14 = *(long **)(param_1 + 0x170 + (ulong)bVar8 * 8);
              if (plVar14 != (long *)0x0) goto LAB_0010b375;
              if (pcVar13 <= pcVar9) {
                return -1;
              }
              goto LAB_0010b2e6;
            }
            lVar11 = *plVar14;
            plVar16 = plVar14;
            if (lVar11 != 0) goto joined_r0x0010b3a7;
            puVar10 = (undefined8 *)plVar14[1];
          }
        }
        plVar14 = (long *)puVar10[2];
LAB_0010b375:
        lVar11 = *plVar14;
        plVar16 = plVar14;
        pcVar12 = pcVar9;
        if (lVar11 != 0) break;
        if (pcVar13 == pcVar9) {
          return -1;
        }
      } while( true );
    }
  }
joined_r0x0010b3a7:
  while (lVar11 < 0) {
    plVar14 = (long *)plVar14[4];
    lVar11 = *plVar14;
  }
  lVar11 = plVar14[5];
  uVar15 = (long)pcVar12 - lVar11;
  if ((param_5 == '\0') || (pcVar13 <= pcVar12)) {
LAB_0010b434:
    lVar6 = *plVar14;
    param_4[2] = lVar11;
    *param_4 = lVar6 >> 1;
    param_4[1] = uVar15 - (long)param_2;
    return uVar15 - (long)param_2;
  }
  do {
    bVar8 = FUN_0010add0(uVar5,(int)*pcVar12);
LAB_0010b3e8:
    puVar10 = (undefined8 *)plVar16[1];
joined_r0x0010b3ef:
    if (puVar10 == (undefined8 *)0x0) break;
    pbVar2 = (byte *)(puVar10 + 3);
    if (*pbVar2 != bVar8) {
      puVar1 = puVar10 + 1;
      puVar10 = (undefined8 *)*puVar10;
      if (*pbVar2 <= bVar8) {
        puVar10 = (undefined8 *)*puVar1;
      }
      goto joined_r0x0010b3ef;
    }
    plVar16 = (long *)puVar10[2];
    pcVar12 = pcVar12 + 1;
    lVar11 = *plVar16;
    plVar7 = plVar16;
    if (lVar11 != 0) {
      while (lVar11 < 0) {
        lVar11 = *(long *)plVar7[4];
        plVar7 = (long *)plVar7[4];
      }
      if ((ulong)((long)pcVar12 - plVar7[5]) <= uVar15) {
        plVar14 = plVar7;
        uVar15 = (long)pcVar12 - plVar7[5];
      }
    }
    if (pcVar13 == pcVar12) {
      lVar11 = plVar14[5];
      goto LAB_0010b434;
    }
  } while( true );
  plVar16 = (long *)plVar16[4];
  lVar11 = plVar14[5];
  if ((plVar16 == (long *)0x0) || (plVar16[5] < lVar11)) goto LAB_0010b434;
  goto LAB_0010b3e8;
}




// Function: acexec @ 0xb530

void acexec(void)

{
  FUN_0010b290();
  return;
}




// Function: bmexec_trans @ 0xb540

long bmexec_trans(long param_1,long param_2,long param_3)

{
  char cVar1;
  char cVar2;
  long lVar3;
  undefined8 uVar4;
  byte bVar5;
  char cVar6;
  long lVar7;
  long lVar8;
  ulong uVar9;
  long lVar10;
  ulong uVar11;
  ulong uVar12;
  long in_FS_OFFSET;
  ulong local_48;
  long local_40;
  
  lVar3 = *(long *)(param_1 + 0x68);
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  lVar10 = 0;
  if (lVar3 == 0) goto LAB_0010b686;
  uVar4 = *(undefined8 *)(param_1 + 0x980);
  if (lVar3 <= param_3) {
    if (lVar3 == 1) {
      lVar10 = FUN_0010b1c0(param_2,param_3,param_1);
      if (lVar10 != 0) {
        lVar10 = lVar10 - param_2;
        goto LAB_0010b686;
      }
    }
    else {
      lVar10 = param_1 + 0x70;
      local_48 = param_2 + lVar3;
      lVar7 = *(long *)(param_1 + 0x970) + lVar3;
      cVar1 = *(char *)(param_1 + 0x988);
      cVar2 = *(char *)(param_1 + 0x990);
      if ((lVar3 * 0xc < param_3) && (SEXT816(lVar3 * 0xc) == SEXT816(lVar3) * SEXT816(0xc))) {
        uVar11 = param_3 + lVar3 * -0xb + param_2;
        uVar12 = local_48;
joined_r0x0010b6df:
        do {
          local_48 = uVar12;
          if (uVar11 < uVar12) goto LAB_0010b5f3;
          bVar5 = FUN_0010adc0((int)*(char *)(uVar12 - 1));
          lVar8 = uVar12 + *(byte *)(lVar10 + (ulong)bVar5);
          bVar5 = FUN_0010adc0((int)*(char *)(lVar8 + -1));
          uVar9 = lVar8 + (ulong)*(byte *)(lVar10 + (ulong)bVar5);
          local_48 = uVar9;
          if (*(byte *)(lVar10 + (ulong)bVar5) != 0) {
            bVar5 = FUN_0010adc0((int)*(char *)(uVar9 - 1));
            lVar8 = uVar9 + *(byte *)(lVar10 + (ulong)bVar5);
            bVar5 = FUN_0010adc0((int)*(char *)(lVar8 + -1));
            lVar8 = lVar8 + (ulong)*(byte *)(lVar10 + (ulong)bVar5);
            bVar5 = FUN_0010adc0((int)*(char *)(lVar8 + -1));
            uVar9 = lVar8 + (ulong)*(byte *)(lVar10 + (ulong)bVar5);
            local_48 = uVar9;
            if (*(byte *)(lVar10 + (ulong)bVar5) != 0) {
              bVar5 = FUN_0010adc0((int)*(char *)(uVar9 - 1));
              lVar8 = uVar9 + *(byte *)(lVar10 + (ulong)bVar5);
              bVar5 = FUN_0010adc0((int)*(char *)(lVar8 + -1));
              lVar8 = lVar8 + (ulong)*(byte *)(lVar10 + (ulong)bVar5);
              bVar5 = FUN_0010adc0((int)*(char *)(lVar8 + -1));
              uVar9 = lVar8 + (ulong)*(byte *)(lVar10 + (ulong)bVar5);
              local_48 = uVar9;
              if (*(byte *)(lVar10 + (ulong)bVar5) != 0) {
                bVar5 = FUN_0010adc0((int)*(char *)(uVar9 - 1));
                lVar8 = uVar9 + *(byte *)(lVar10 + (ulong)bVar5);
                bVar5 = FUN_0010adc0((int)*(char *)(lVar8 + -1));
                local_48 = lVar8 + (ulong)*(byte *)(lVar10 + (ulong)bVar5);
                lVar8 = local_48 - uVar12;
                uVar12 = local_48;
                if (0x7f < lVar8) goto joined_r0x0010b6df;
                lVar8 = FUN_0010b1c0(local_48 - 1,(param_2 + param_3) - (local_48 - 1),param_1);
                if (lVar8 == 0) goto LAB_0010b67f;
                local_48 = lVar8 + 1;
                if (uVar11 <= local_48) goto LAB_0010b5f3;
              }
            }
          }
          cVar6 = FUN_0010b030(&local_48,uVar11,lVar7,lVar3,uVar4,(int)cVar1,(int)cVar2,lVar10,
                               param_1);
          uVar12 = local_48;
        } while (cVar6 == '\0');
LAB_0010b6b0:
        lVar10 = local_48 - param_2;
        goto LAB_0010b686;
      }
LAB_0010b5f3:
      uVar12 = local_48;
      param_3 = param_3 + param_2;
      bVar5 = FUN_0010adc0((int)*(char *)(local_48 - 1));
      uVar11 = (ulong)*(byte *)(param_1 + 0x70 + (ulong)bVar5);
      while ((long)uVar11 <= (long)(param_3 - uVar12)) {
        while( true ) {
          uVar12 = uVar12 + uVar11;
          local_48 = uVar12;
          bVar5 = FUN_0010adc0((int)*(char *)(uVar12 - 1));
          uVar11 = (ulong)*(byte *)(lVar10 + (ulong)bVar5);
          if (*(byte *)(lVar10 + (ulong)bVar5) != 0) break;
          cVar6 = FUN_0010b030(&local_48,param_3,lVar7,lVar3,uVar4,(int)cVar1,(int)cVar2,0,param_1);
          if (cVar6 != '\0') goto LAB_0010b6b0;
          uVar12 = local_48;
          if ((long)(param_3 - local_48) < 0) goto LAB_0010b67f;
        }
      }
    }
  }
LAB_0010b67f:
  lVar10 = -1;
LAB_0010b686:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar10;
}




// Function: bmexec @ 0xb930

void bmexec(long param_1,undefined8 param_2,undefined8 param_3,undefined8 *param_4)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  uVar2 = FUN_0010b540();
  uVar1 = *(undefined8 *)(param_1 + 0x68);
  *param_4 = 0;
  param_4[1] = uVar2;
  param_4[2] = uVar1;
  return;
}




// Function: kwsalloc @ 0xb960

long kwsalloc(undefined8 param_1)

{
  long lVar1;
  undefined8 *puVar2;
  ulong uVar3;
  ulong uVar4;
  undefined8 *puVar5;
  
  lVar1 = FUN_0011d0f0(0x9a0);
  _obstack_begin(lVar1,0,0,FUN_0011d0f0,PTR_free_00129fb8);
  uVar4 = *(ulong *)(lVar1 + 0x20);
  *(undefined8 *)(lVar1 + 0x58) = 0;
  if (uVar4 - *(long *)(lVar1 + 0x18) < 0x40) {
    _obstack_newchunk(lVar1,0x40);
    puVar5 = *(undefined8 **)(lVar1 + 0x10);
    uVar4 = *(ulong *)(lVar1 + 0x20);
    puVar2 = (undefined8 *)(*(long *)(lVar1 + 0x18) + 0x40);
    if (puVar2 != puVar5) goto LAB_0010b9ca;
  }
  else {
    puVar5 = *(undefined8 **)(lVar1 + 0x10);
    puVar2 = (undefined8 *)(*(long *)(lVar1 + 0x18) + 0x40);
    if (puVar2 != puVar5) goto LAB_0010b9ca;
  }
  *(byte *)(lVar1 + 0x50) = *(byte *)(lVar1 + 0x50) | 2;
LAB_0010b9ca:
  *(undefined8 **)(lVar1 + 0x60) = puVar5;
  uVar3 = (long)puVar2 + *(ulong *)(lVar1 + 0x30) & ~*(ulong *)(lVar1 + 0x30);
  if (uVar3 - *(long *)(lVar1 + 8) <= uVar4 - *(long *)(lVar1 + 8)) {
    uVar4 = uVar3;
  }
  *(ulong *)(lVar1 + 0x18) = uVar4;
  *(ulong *)(lVar1 + 0x10) = uVar4;
  *puVar5 = 0;
  puVar5[1] = 0;
  puVar5[2] = 0;
  puVar5[3] = 0;
  puVar5[4] = 0;
  puVar5[5] = 0;
  puVar5[6] = 0;
  *(undefined8 *)(lVar1 + 0x68) = 0x7fffffffffffffff;
  *(undefined8 *)(lVar1 + 0x980) = param_1;
  *(code **)(lVar1 + 0x998) = FUN_0010b530;
  *(undefined8 *)(lVar1 + 0x970) = 0;
  return lVar1;
}




// Function: kwsincr @ 0xbab0

void kwsincr(long param_1,byte *param_2,long param_3)

{
  byte bVar1;
  long lVar2;
  long *plVar3;
  long lVar4;
  long lVar5;
  long *plVar6;
  char cVar7;
  undefined8 *puVar8;
  long *plVar9;
  long *plVar10;
  long lVar11;
  byte bVar12;
  ulong uVar13;
  int iVar14;
  long lVar15;
  undefined8 *puVar16;
  ulong uVar17;
  code *pcVar18;
  long in_FS_OFFSET;
  ulong local_f8;
  ulong local_f0;
  undefined8 local_e0;
  int local_d8 [12];
  long *local_a8 [13];
  long local_40;
  
  pcVar18 = *(code **)(param_1 + 0x998);
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  lVar2 = *(long *)(param_1 + 0x980);
  plVar6 = *(long **)(param_1 + 0x60);
  if (pcVar18 == FUN_0010b930) {
    param_2 = param_2 + param_3;
  }
joined_r0x0010bb05:
  do {
    if (param_3 == 0) {
      lVar2 = *(long *)(param_1 + 0x58);
      if (*plVar6 == 0) {
        *plVar6 = lVar2 * 2 + 1;
      }
      *(long *)(param_1 + 0x58) = lVar2 + 1;
      if (plVar6[5] < *(long *)(param_1 + 0x68)) {
        *(long *)(param_1 + 0x68) = plVar6[5];
      }
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    param_3 = param_3 + -1;
    if (pcVar18 == FUN_0010b930) {
      bVar12 = param_2[-1];
      param_2 = param_2 + -1;
    }
    else {
      bVar12 = *param_2;
      param_2 = param_2 + 1;
    }
    if (lVar2 != 0) {
      bVar12 = *(byte *)(lVar2 + (ulong)bVar12);
    }
    plVar9 = (long *)plVar6[1];
    local_d8[0] = 0;
    local_a8[0] = plVar6 + 1;
    lVar15 = 1;
    if (plVar9 == (long *)0x0) {
      lVar11 = 0;
    }
    else {
      do {
        lVar11 = lVar15;
        bVar1 = *(byte *)(plVar9 + 3);
        if (bVar1 == bVar12) {
          plVar6 = (long *)plVar9[2];
          goto joined_r0x0010bb05;
        }
        local_a8[lVar11] = plVar9;
        lVar15 = lVar11 + 1;
        if (bVar12 < bVar1) {
          plVar9 = (long *)*plVar9;
          *(undefined4 *)((long)&local_e0 + lVar15 * 4 + 4) = 0;
        }
        else {
          plVar9 = (long *)plVar9[1];
          *(undefined4 *)((long)&local_e0 + lVar15 * 4 + 4) = 1;
        }
      } while (plVar9 != (long *)0x0);
    }
    uVar13 = *(ulong *)(param_1 + 0x20);
    if (uVar13 - *(long *)(param_1 + 0x18) < 0x20) {
      _obstack_newchunk(param_1,0x20);
      puVar16 = *(undefined8 **)(param_1 + 0x10);
      uVar13 = *(ulong *)(param_1 + 0x20);
      puVar8 = (undefined8 *)(*(long *)(param_1 + 0x18) + 0x20);
      if (puVar8 == puVar16) goto LAB_0010bea7;
    }
    else {
      puVar16 = *(undefined8 **)(param_1 + 0x10);
      puVar8 = (undefined8 *)(*(long *)(param_1 + 0x18) + 0x20);
      if (puVar8 == puVar16) {
LAB_0010bea7:
        *(byte *)(param_1 + 0x50) = *(byte *)(param_1 + 0x50) | 2;
      }
    }
    uVar17 = *(ulong *)(param_1 + 0x30);
    local_f0 = ~uVar17;
    plVar9 = (long *)((long)puVar8 + uVar17 & local_f0);
    lVar15 = *(long *)(param_1 + 8);
    local_f8 = uVar13 - lVar15;
    if (local_f8 < (ulong)((long)plVar9 - lVar15)) {
      *(ulong *)(param_1 + 0x18) = uVar13;
      *(ulong *)(param_1 + 0x10) = uVar13;
      *puVar16 = 0;
      puVar16[1] = 0;
LAB_0010bc0f:
      local_e0 = pcVar18;
      _obstack_newchunk(param_1,0x40);
      uVar17 = *(ulong *)(param_1 + 0x30);
      uVar13 = *(ulong *)(param_1 + 0x20);
      local_f0 = ~uVar17;
      plVar10 = (long *)(*(long *)(param_1 + 0x18) + 0x40);
      lVar15 = *(long *)(param_1 + 8);
      local_f8 = uVar13 - lVar15;
      plVar9 = *(long **)(param_1 + 0x10);
      pcVar18 = local_e0;
      if (plVar10 == plVar9) {
LAB_0010be4f:
        *(byte *)(param_1 + 0x50) = *(byte *)(param_1 + 0x50) | 2;
      }
    }
    else {
      *(long **)(param_1 + 0x18) = plVar9;
      *(long **)(param_1 + 0x10) = plVar9;
      *puVar16 = 0;
      puVar16[1] = 0;
      if (uVar13 - (long)plVar9 < 0x40) goto LAB_0010bc0f;
      plVar10 = plVar9 + 8;
      if (plVar10 == plVar9) goto LAB_0010be4f;
    }
    local_f0 = (long)plVar10 + uVar17 & local_f0;
    iVar14 = local_d8[lVar11];
    if (local_f0 - lVar15 <= local_f8) {
      uVar13 = local_f0;
    }
    *(ulong *)(param_1 + 0x18) = uVar13;
    lVar15 = plVar6[5];
    *(ulong *)(param_1 + 0x10) = uVar13;
    puVar16[2] = plVar9;
    plVar9[5] = lVar15 + 1;
    plVar10 = local_a8[lVar11];
    *plVar9 = 0;
    plVar9[1] = 0;
    plVar9[2] = (long)plVar6;
    plVar9[3] = 0;
    plVar9[4] = 0;
    plVar9[6] = 0;
    *(byte *)(puVar16 + 3) = bVar12;
    *(undefined1 *)((long)puVar16 + 0x19) = 0;
    if (iVar14 == 0) {
      *plVar10 = (long)puVar16;
    }
    else {
      plVar10[1] = (long)puVar16;
    }
    plVar6 = plVar9;
  } while (lVar11 == 0);
  while( true ) {
    plVar9 = local_a8[lVar11];
    cVar7 = *(char *)((long)plVar9 + 0x19);
    if (cVar7 != '\0') break;
    *(byte *)((long)plVar9 + 0x19) = -(iVar14 == 0) | 1;
    lVar11 = lVar11 + -1;
    if (lVar11 == 0) goto joined_r0x0010bb05;
    iVar14 = local_d8[lVar11];
  }
  if (iVar14 == 0) {
    cVar7 = cVar7 + -1;
    *(char *)((long)plVar9 + 0x19) = cVar7;
  }
  else {
    if (iVar14 != 1) goto joined_r0x0010bb05;
    cVar7 = cVar7 + '\x01';
    *(char *)((long)plVar9 + 0x19) = cVar7;
  }
  if (cVar7 != '\0') {
    if (cVar7 == -2) {
      if (local_d8[lVar11 + 1] == 0) {
        plVar10 = (long *)*plVar9;
        lVar15 = plVar10[1];
        plVar10[1] = (long)plVar9;
        *(undefined1 *)((long)plVar9 + 0x19) = 0;
        *plVar9 = lVar15;
        *(undefined1 *)((long)plVar10 + 0x19) = 0;
      }
      else {
        if (local_d8[lVar11 + 1] != 1) goto LAB_0010bf79;
        lVar15 = *plVar9;
        plVar10 = *(long **)(lVar15 + 8);
        lVar4 = plVar10[1];
        lVar5 = *plVar10;
        *plVar10 = lVar15;
        *(long *)(lVar15 + 8) = lVar5;
        plVar10[1] = (long)plVar9;
        *plVar9 = lVar4;
        *(char *)(lVar15 + 0x19) = -(*(char *)((long)plVar10 + 0x19) == '\x01');
        *(bool *)((long)plVar9 + 0x19) = *(char *)((long)plVar10 + 0x19) == -1;
        *(undefined1 *)((long)plVar10 + 0x19) = 0;
      }
    }
    else {
      if (cVar7 != '\x02') {
LAB_0010bf79:
                    /* WARNING: Subroutine does not return */
        abort();
      }
      if (local_d8[lVar11 + 1] == 0) {
        plVar3 = (long *)plVar9[1];
        plVar10 = (long *)*plVar3;
        lVar15 = plVar10[1];
        lVar4 = *plVar10;
        *plVar10 = (long)plVar9;
        plVar9[1] = lVar4;
        plVar10[1] = (long)plVar3;
        *plVar3 = lVar15;
        *(char *)((long)plVar9 + 0x19) = -(*(char *)((long)plVar10 + 0x19) == '\x01');
        *(bool *)((long)plVar3 + 0x19) = *(char *)((long)plVar10 + 0x19) == -1;
        *(undefined1 *)((long)plVar10 + 0x19) = 0;
      }
      else {
        if (local_d8[lVar11 + 1] != 1) goto LAB_0010bf79;
        plVar10 = (long *)plVar9[1];
        lVar15 = *plVar10;
        *plVar10 = (long)plVar9;
        *(undefined1 *)((long)plVar9 + 0x19) = 0;
        plVar9[1] = lVar15;
        *(undefined1 *)((long)plVar10 + 0x19) = 0;
      }
    }
    if (local_d8[lVar11 + -1] == 0) {
      *local_a8[lVar11 + -1] = (long)plVar10;
    }
    else {
      local_a8[lVar11 + -1][1] = (long)plVar10;
    }
  }
  goto joined_r0x0010bb05;
}




// Function: kwswords @ 0xbf90

undefined8 kwswords(long param_1)

{
  return *(undefined8 *)(param_1 + 0x58);
}




// Function: kwsprep @ 0xbfa0

void kwsprep(undefined8 *param_1)

{
  long lVar1;
  undefined8 uVar2;
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  undefined1 auVar6 [16];
  uint uVar7;
  char cVar8;
  byte bVar9;
  undefined1 uVar10;
  ulong uVar11;
  long lVar12;
  undefined8 *puVar13;
  ulong uVar14;
  ulong uVar15;
  long lVar16;
  long lVar17;
  undefined8 *__ptr;
  long *plVar18;
  undefined8 uVar19;
  undefined8 extraout_RDX;
  undefined8 extraout_RDX_00;
  long lVar20;
  undefined8 *puVar21;
  long *plVar22;
  long **pplVar23;
  undefined8 *puVar24;
  long lVar25;
  uint uVar26;
  uint uVar27;
  uint uVar28;
  long in_FS_OFFSET;
  uint local_96c;
  long *local_950;
  undefined8 local_948 [256];
  undefined8 local_148 [33];
  long local_40;
  
  bVar9 = 0;
  lVar1 = param_1[0x130];
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  puVar24 = local_148;
  if (lVar1 == 0) {
    puVar24 = param_1 + 0xe;
  }
  lVar25 = param_1[0xb];
  if (lVar25 == 1) {
    pplVar23 = &local_950;
    plVar18 = (long *)param_1[0xc];
    plVar22 = plVar18;
    local_950 = plVar18;
    for (; plVar18 != (long *)0x0; plVar18 = (long *)plVar18[3]) {
      FUN_0010ae00(plVar18[1],pplVar23);
    }
    uVar11 = param_1[4];
    uVar14 = param_1[0xd];
    if (uVar11 - param_1[3] < uVar14) {
      _obstack_newchunk(param_1,uVar14);
      lVar20 = param_1[2];
      uVar11 = param_1[4];
      plVar22 = (long *)param_1[0xc];
      lVar17 = param_1[3] + uVar14;
      uVar14 = param_1[0xd];
      if (lVar17 == lVar20) goto LAB_0010c855;
    }
    else {
      lVar20 = param_1[2];
      lVar17 = param_1[3] + uVar14;
      if (lVar17 == lVar20) {
LAB_0010c855:
        *(byte *)(param_1 + 10) = *(byte *)(param_1 + 10) | 2;
      }
    }
    param_1[0x12e] = lVar20;
    uVar15 = lVar17 + param_1[6] & ~param_1[6];
    if (uVar15 - param_1[1] <= uVar11 - param_1[1]) {
      uVar11 = uVar15;
    }
    param_1[3] = uVar11;
    param_1[2] = uVar11;
    if (0 < (long)uVar14) {
      lVar17 = 0;
      while( true ) {
        *(undefined1 *)(lVar20 + lVar17) = *(undefined1 *)(plVar22[1] + 0x18);
        lVar17 = lVar17 + 1;
        plVar22 = (long *)plVar22[3];
        if ((long)param_1[0xd] <= lVar17) break;
        lVar20 = param_1[0x12e];
      }
    }
    __ptr = (undefined8 *)FUN_0010b960(param_1[0x130]);
    uVar19 = param_1[0xd];
    uVar2 = param_1[0x12e];
    __ptr[0x133] = FUN_0010b930;
    FUN_0010bab0(__ptr,uVar2,uVar19);
    _obstack_free(param_1,0);
    puVar13 = __ptr;
    puVar21 = param_1;
    for (lVar20 = 0x134; lVar20 != 0; lVar20 = lVar20 + -1) {
      *puVar21 = *puVar13;
      puVar13 = puVar13 + (ulong)bVar9 * -2 + 1;
      puVar21 = puVar21 + (ulong)bVar9 * -2 + 1;
    }
    free(__ptr);
    uVar11 = param_1[0xd];
    if (0xff < (long)uVar11) {
      uVar11 = 0xff;
    }
    uVar11 = uVar11 & 0xff;
    auVar5._8_8_ = 0;
    auVar5._0_8_ = uVar11;
    *puVar24 = SUB168(auVar5 * ZEXT816(0x101010101010101),0);
    lVar20 = uVar11 * 0x101010101010101 + SUB168(auVar5 * ZEXT816(0x101010101010101),8);
    puVar24[1] = lVar20;
    puVar24[3] = lVar20;
    puVar24[2] = uVar11 * 0x101010101010101;
    puVar24[5] = lVar20;
    puVar24[7] = lVar20;
    puVar24[9] = lVar20;
    puVar24[4] = uVar11 * 0x101010101010101;
    puVar24[0xb] = lVar20;
    puVar24[0xd] = lVar20;
    puVar24[0xf] = lVar20;
    puVar24[6] = uVar11 * 0x101010101010101;
    puVar24[0x11] = lVar20;
    puVar24[0x13] = lVar20;
    puVar24[0x15] = lVar20;
    puVar24[8] = uVar11 * 0x101010101010101;
    puVar24[0x17] = lVar20;
    puVar24[0x19] = lVar20;
    puVar24[0x1b] = lVar20;
    puVar24[10] = uVar11 * 0x101010101010101;
    puVar24[0x1d] = lVar20;
    puVar24[0xc] = uVar11 * 0x101010101010101;
    puVar24[0xe] = uVar11 * 0x101010101010101;
    puVar24[0x10] = uVar11 * 0x101010101010101;
    puVar24[0x12] = uVar11 * 0x101010101010101;
    puVar24[0x14] = uVar11 * 0x101010101010101;
    puVar24[0x16] = uVar11 * 0x101010101010101;
    puVar24[0x18] = uVar11 * 0x101010101010101;
    puVar24[0x1a] = uVar11 * 0x101010101010101;
    puVar24[0x1c] = uVar11 * 0x101010101010101;
    auVar6._8_8_ = 0;
    auVar6._0_8_ = uVar11;
    uVar19 = SUB168(auVar6 * ZEXT816(0x101010101010101),8);
    puVar24[0x1e] = SUB168(auVar6 * ZEXT816(0x101010101010101),0);
    puVar24[0x1f] = lVar20;
    plVar18 = (long *)param_1[0xc];
    if (plVar18 != (long *)0x0) goto LAB_0010c164;
    lVar20 = 0;
    local_950 = plVar18;
LAB_0010c3d0:
    while (lVar20 = *(long *)(lVar20 + 0x18), lVar20 != 0) {
      while( true ) {
        lVar17 = *(long *)(*(long *)(lVar20 + 0x10) + 0x38);
        lVar16 = *(long *)(lVar20 + 0x38);
        if (lVar17 < *(long *)(lVar20 + 0x38)) {
          *(long *)(lVar20 + 0x38) = lVar17;
          lVar16 = lVar17;
        }
        if (*(long *)(lVar20 + 0x30) <= lVar16) break;
        *(long *)(lVar20 + 0x30) = lVar16;
        lVar20 = *(long *)(lVar20 + 0x18);
        if (lVar20 == 0) goto LAB_0010c233;
      }
    }
  }
  else {
    uVar11 = param_1[0xd];
    if (0xff < (long)uVar11) {
      uVar11 = 0xff;
    }
    uVar11 = uVar11 & 0xff;
    auVar3._8_8_ = 0;
    auVar3._0_8_ = uVar11;
    *puVar24 = SUB168(auVar3 * ZEXT816(0x101010101010101),0);
    lVar20 = uVar11 * 0x101010101010101 + SUB168(auVar3 * ZEXT816(0x101010101010101),8);
    puVar24[1] = lVar20;
    puVar24[3] = lVar20;
    puVar24[2] = uVar11 * 0x101010101010101;
    puVar24[5] = lVar20;
    puVar24[7] = lVar20;
    puVar24[9] = lVar20;
    puVar24[4] = uVar11 * 0x101010101010101;
    puVar24[0xb] = lVar20;
    puVar24[0xd] = lVar20;
    puVar24[0xf] = lVar20;
    puVar24[6] = uVar11 * 0x101010101010101;
    puVar24[0x11] = lVar20;
    puVar24[0x13] = lVar20;
    puVar24[0x15] = lVar20;
    puVar24[8] = uVar11 * 0x101010101010101;
    puVar24[0x17] = lVar20;
    puVar24[0x19] = lVar20;
    puVar24[0x1b] = lVar20;
    puVar24[10] = uVar11 * 0x101010101010101;
    puVar24[0x1d] = lVar20;
    puVar24[0xc] = uVar11 * 0x101010101010101;
    puVar24[0xe] = uVar11 * 0x101010101010101;
    puVar24[0x10] = uVar11 * 0x101010101010101;
    puVar24[0x12] = uVar11 * 0x101010101010101;
    puVar24[0x14] = uVar11 * 0x101010101010101;
    puVar24[0x16] = uVar11 * 0x101010101010101;
    puVar24[0x18] = uVar11 * 0x101010101010101;
    puVar24[0x1a] = uVar11 * 0x101010101010101;
    puVar24[0x1c] = uVar11 * 0x101010101010101;
    auVar4._8_8_ = 0;
    auVar4._0_8_ = uVar11;
    uVar19 = SUB168(auVar4 * ZEXT816(0x101010101010101),8);
    puVar24[0x1e] = SUB168(auVar4 * ZEXT816(0x101010101010101),0);
    puVar24[0x1f] = lVar20;
    plVar18 = (long *)param_1[0xc];
    local_950 = plVar18;
    if (plVar18 != (long *)0x0) {
LAB_0010c164:
      pplVar23 = &local_950;
      local_950 = plVar18;
      do {
        while( true ) {
          lVar20 = plVar18[1];
          FUN_0010ae00(lVar20,pplVar23,uVar19,lVar25 == 1);
          FUN_0010af10(lVar20,plVar18[5],puVar24);
          lVar20 = plVar18[1];
          FUN_0010ae50(lVar20,plVar18[4],param_1[0xc]);
          if (lVar25 == 1) break;
LAB_0010c170:
          plVar18 = (long *)plVar18[3];
          uVar19 = extraout_RDX;
          if (plVar18 == (long *)0x0) goto LAB_0010c227;
        }
        lVar17 = param_1[0xd];
        lVar16 = plVar18[4];
        plVar18[6] = lVar17;
        plVar18[7] = lVar17;
        if (lVar16 == 0) goto LAB_0010c170;
        lVar17 = *plVar18;
        do {
          cVar8 = FUN_0010af60(*(undefined8 *)(lVar16 + 8),lVar20);
          if ((cVar8 == '\0') &&
             (lVar12 = plVar18[5] - *(long *)(lVar16 + 0x28), lVar12 < *(long *)(lVar16 + 0x30))) {
            *(long *)(lVar16 + 0x30) = lVar12;
          }
          if ((lVar17 != 0) &&
             (lVar12 = plVar18[5] - *(long *)(lVar16 + 0x28), lVar12 < *(long *)(lVar16 + 0x38))) {
            *(long *)(lVar16 + 0x38) = lVar12;
          }
          lVar16 = *(long *)(lVar16 + 0x20);
        } while (lVar16 != 0);
        plVar18 = (long *)plVar18[3];
        uVar19 = extraout_RDX_00;
      } while (plVar18 != (long *)0x0);
LAB_0010c227:
      if (lVar25 == 1) {
        lVar20 = param_1[0xc];
        goto LAB_0010c3d0;
      }
    }
  }
LAB_0010c233:
  puVar13 = param_1 + 0x2e;
  if (lVar1 != 0) {
    puVar13 = local_948;
  }
  lVar20 = 0;
  *puVar13 = 0;
  puVar13[0xff] = 0;
  puVar21 = (undefined8 *)((ulong)(puVar13 + 1) & 0xfffffffffffffff8);
  for (uVar11 = (ulong)(((int)puVar13 -
                        (int)(undefined8 *)((ulong)(puVar13 + 1) & 0xfffffffffffffff8)) + 0x800U >>
                       3); uVar11 != 0; uVar11 = uVar11 - 1) {
    *puVar21 = 0;
    puVar21 = puVar21 + (ulong)bVar9 * -2 + 1;
  }
  FUN_0010afe0(*(undefined8 *)(param_1[0xc] + 8),puVar13);
  local_96c = 0xffffffff;
  uVar27 = 0xfffffffe;
  do {
    while( true ) {
      uVar26 = (uint)lVar20;
      if (lVar1 == 0) {
        lVar17 = param_1[lVar20 + 0x2e];
        uVar28 = uVar26;
      }
      else {
        bVar9 = FUN_0010adc0((int)*(char *)(lVar1 + lVar20));
        uVar28 = (uint)bVar9;
        lVar17 = puVar13[bVar9];
        param_1[lVar20 + 0x2e] = lVar17;
      }
      uVar7 = local_96c;
      if (lVar17 != 0) break;
LAB_0010c2f0:
      local_96c = uVar7;
      uVar28 = uVar27;
      lVar20 = lVar20 + 1;
      uVar27 = uVar28;
      if (lVar20 == 0x100) goto LAB_0010c32d;
    }
    if (uVar27 != 0xfffffffe) {
      uVar7 = uVar26;
      if (uVar28 == uVar27) {
        if (uVar27 != local_96c) {
          uVar7 = 0xffffffff;
        }
      }
      else if ((uVar28 != uVar26) || (uVar27 != local_96c)) {
        uVar27 = 0xffffffff;
        uVar7 = local_96c;
      }
      goto LAB_0010c2f0;
    }
    lVar20 = lVar20 + 1;
    uVar27 = uVar28;
    local_96c = uVar26;
  } while (lVar20 != 0x100);
LAB_0010c32d:
  *(uint *)(param_1 + 0x131) = uVar28;
  *(uint *)((long)param_1 + 0x98c) = local_96c;
  if (lVar25 != 1) goto LAB_0010c34a;
  uVar11 = param_1[4];
  uVar14 = param_1[0xd];
  if (uVar11 - param_1[3] < uVar14) {
    _obstack_newchunk(param_1);
    lVar25 = param_1[2];
    uVar11 = param_1[4];
    uVar15 = param_1[0xd];
    lVar20 = param_1[3] + uVar14;
    if (lVar20 == lVar25) goto LAB_0010c821;
  }
  else {
    lVar25 = param_1[2];
    lVar20 = param_1[3] + uVar14;
    uVar15 = uVar14;
    if (lVar20 == lVar25) {
LAB_0010c821:
      *(byte *)(param_1 + 10) = *(byte *)(param_1 + 10) | 2;
    }
  }
  param_1[0x12e] = lVar25;
  uVar14 = lVar20 + param_1[6] & ~param_1[6];
  if (uVar14 - param_1[1] <= uVar11 - param_1[1]) {
    uVar11 = uVar14;
  }
  param_1[3] = uVar11;
  param_1[2] = uVar11;
  lVar20 = param_1[0xc];
  if (0 < (long)uVar15) {
    while( true ) {
      uVar15 = uVar15 - 1;
      *(undefined1 *)(lVar25 + uVar15) = *(undefined1 *)(*(long *)(lVar20 + 8) + 0x18);
      lVar20 = *(long *)(lVar20 + 0x18);
      if (uVar15 == 0) break;
      lVar25 = param_1[0x12e];
    }
    lVar25 = param_1[0xd];
    if (1 < lVar25) {
      uVar14 = param_1[4];
      lVar20 = param_1[3];
      uVar11 = lVar25 * 8 - 8;
      if (uVar14 - lVar20 < uVar11) {
        _obstack_newchunk(param_1,uVar11);
        lVar20 = param_1[3];
        uVar14 = param_1[4];
        lVar25 = param_1[0xd];
      }
      lVar17 = param_1[2];
      if (lVar20 + uVar11 == lVar17) {
        *(byte *)(param_1 + 10) = *(byte *)(param_1 + 10) | 2;
      }
      param_1[0x12f] = lVar17;
      uVar11 = lVar20 + uVar11 + param_1[6] & ~param_1[6];
      if (uVar11 - param_1[1] <= uVar14 - param_1[1]) {
        uVar14 = uVar11;
      }
      param_1[3] = uVar14;
      param_1[2] = uVar14;
      lVar20 = *(long *)(param_1[0xc] + 0x18);
      if (1 < lVar25) {
        lVar16 = 0;
        do {
          puVar13 = (undefined8 *)(lVar20 + 0x30);
          lVar20 = *(long *)(lVar20 + 0x18);
          *(undefined8 *)(lVar17 + lVar16 * 8) = *puVar13;
          lVar25 = param_1[0xd];
          lVar16 = lVar16 + 1;
        } while (lVar16 < lVar25 + -1);
      }
      uVar10 = FUN_0010add0(lVar1,(int)*(char *)(param_1[0x12e] + -2 + lVar25));
      *(undefined1 *)(param_1 + 0x132) = uVar10;
    }
  }
LAB_0010c34a:
  lVar25 = 0;
  if (lVar1 != 0) {
    do {
      bVar9 = FUN_0010adc0((int)*(char *)(lVar1 + lVar25));
      *(undefined1 *)((long)param_1 + lVar25 + 0x70) = *(undefined1 *)((long)puVar24 + (ulong)bVar9)
      ;
      lVar25 = lVar25 + 1;
    } while (lVar25 != 0x100);
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: kwsexec @ 0xc870

void kwsexec(long param_1)

{
                    /* WARNING: Could not recover jumptable at 0x0010c87f. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (**(code **)(param_1 + 0x998))();
  return;
}




// Function: wordchar @ 0xc8b0

undefined8 wordchar(wint_t param_1)

{
  int iVar1;
  undefined4 extraout_var;
  
  if (param_1 == 0x5f) {
    return 1;
  }
  iVar1 = iswalnum(param_1);
  return CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),iVar1 != 0);
}




// Function: wordchars_count @ 0xc8e0

long wordchars_count(long param_1,long param_2,char param_3)

{
  byte bVar1;
  char cVar2;
  long lVar3;
  long lVar4;
  long in_FS_OFFSET;
  undefined4 local_4c;
  undefined8 local_48;
  long local_40;
  
  lVar4 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_48 = 0;
  do {
    while( true ) {
      if (param_2 - param_1 <= lVar4) goto LAB_0010c9a0;
      bVar1 = *(byte *)(param_1 + lVar4);
      if ((&DAT_0012a9c0)[bVar1] != '\0') break;
      if ((&DAT_0012a183)[bVar1] != -2) goto LAB_0010c9a0;
      local_4c = 0;
      lVar3 = FUN_00118c40(&local_4c,(byte *)(param_1 + lVar4),(param_2 - param_1) - lVar4,&local_48
                          );
      cVar2 = FUN_0010c8b0(local_4c);
      if ((cVar2 == '\0') || (lVar4 = lVar4 + lVar3 + (ulong)(lVar3 == 0), param_3 == '\0'))
      goto LAB_0010c9a0;
    }
    lVar4 = lVar4 + 1;
  } while (param_3 != '\0');
LAB_0010c9a0:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: wordinit @ 0xca30

void wordinit(void)

{
  undefined4 uVar1;
  undefined1 uVar2;
  undefined4 *puVar3;
  undefined1 *puVar4;
  
  puVar3 = &DAT_0012a284;
  puVar4 = &DAT_0012a9c0;
  do {
    uVar1 = *puVar3;
    puVar3 = puVar3 + 1;
    uVar2 = FUN_0010c8b0(uVar1);
    *puVar4 = uVar2;
    puVar4 = puVar4 + 1;
  } while (puVar3 != (undefined4 *)&DAT_0012a684);
  return;
}




// Function: kwsinit @ 0xca70

void kwsinit(char param_1)

{
  int iVar1;
  size_t sVar2;
  long lVar3;
  long lVar4;
  
  if (DAT_0012a687 != '\0') {
    sVar2 = __ctype_get_mb_cur_max();
    if ((sVar2 == 1) || (param_1 != '\0')) {
      lVar4 = 0;
      lVar3 = FUN_0011d110(0x100);
      do {
        iVar1 = toupper((int)lVar4);
        *(char *)(lVar3 + lVar4) = (char)iVar1;
        lVar4 = lVar4 + 1;
      } while (lVar4 != 0x100);
      FUN_0010b960(lVar3);
      return;
    }
  }
  FUN_0010b960(0);
  return;
}




// Function: mb_goback @ 0xcaf0

long mb_goback(undefined8 *param_1,long *param_2,char *param_3,long param_4)

{
  uint uVar1;
  long lVar2;
  char *pcVar3;
  char *pcVar4;
  char *pcVar5;
  long in_FS_OFFSET;
  undefined8 local_48;
  long local_40;
  
  pcVar5 = (char *)*param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (pcVar5 < param_3) {
    if (DAT_0012a182 == '\0') {
      local_48 = 0;
      do {
        pcVar4 = pcVar5;
        lVar2 = FUN_0010c9f0(pcVar4,param_4 - (long)pcVar4,&local_48);
        if (lVar2 < 0) {
          local_48 = 0;
          lVar2 = 1;
          pcVar3 = pcVar4 + 1;
        }
        else {
          pcVar3 = pcVar4 + lVar2;
        }
        pcVar5 = pcVar3;
      } while (pcVar3 < param_3);
      pcVar5 = pcVar4;
      if (param_2 != (long *)0x0) {
        *param_2 = lVar2;
      }
LAB_0010cb86:
      *param_1 = pcVar3;
      if (pcVar3 != param_3) goto LAB_0010cb8f;
    }
    else {
      if (((int)*param_3 & 0xc0U) == 0x80) {
        uVar1 = (uint)param_3[-1];
        if ((uVar1 & 0xc0) == 0x80) {
          uVar1 = (uint)param_3[-2];
          if ((uVar1 & 0xc0) == 0x80) {
            uVar1 = (uint)param_3[-3];
            if ((uVar1 & 0xc0) == 0x80) goto LAB_0010cbcf;
            lVar2 = 3;
          }
          else {
            lVar2 = 2;
          }
        }
        else {
          lVar2 = 1;
        }
        if ((int)(~uVar1 & 0xff) >> (7U - (char)lVar2 & 0x1f) == 0) {
          local_48 = 0;
          pcVar4 = param_3 + -lVar2;
          lVar2 = thunk_FUN_00118c10(pcVar4,param_4 - (long)pcVar4,&local_48);
          pcVar3 = param_3;
          if (-1 < lVar2) {
            pcVar3 = pcVar4 + lVar2;
            pcVar5 = pcVar4;
          }
          goto LAB_0010cb86;
        }
      }
LAB_0010cbcf:
      *param_1 = param_3;
    }
    lVar2 = 0;
  }
  else {
LAB_0010cb8f:
    lVar2 = (long)param_3 - (long)pcVar5;
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: wordchars_size @ 0xcc80

void wordchars_size(undefined8 param_1,undefined8 param_2)

{
  FUN_0010c8e0(param_1,param_2,1);
  return;
}




// Function: wordchar_next @ 0xcc90

void wordchar_next(undefined8 param_1,undefined8 param_2)

{
  FUN_0010c8e0(param_1,param_2,0);
  return;
}




// Function: wordchar_prev @ 0xcca0

ulong wordchar_prev(long param_1,long param_2,undefined8 param_3)

{
  long lVar1;
  ulong uVar2;
  long in_FS_OFFSET;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 == param_2) {
    uVar2 = 0;
  }
  else {
    if ((DAT_0012a180 != '\0') && ((~(*(byte *)(param_2 + -1) >> 7) & DAT_0012a182) == 0)) {
      local_28 = param_1;
      lVar1 = FUN_0010caf0(&local_28,0,param_2 + -1,param_3);
      if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
        uVar2 = FUN_0010cc90((param_2 + -1) - lVar1,param_3);
        return uVar2;
      }
      goto LAB_0010cd54;
    }
    uVar2 = (ulong)(byte)(&DAT_0012a9c0)[*(byte *)(param_2 + -1)];
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
LAB_0010cd54:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: private_malloc @ 0xcd70

void private_malloc(long param_1)

{
  if (-1 < param_1) {
    FUN_0011d110();
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_0011d4b0();
}




// Function: jit_exec @ 0xcd90

int jit_exec(undefined8 *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                undefined4 param_5)

{
  int iVar1;
  long lVar2;
  long lVar3;
  long in_FS_OFFSET;
  bool bVar4;
  undefined8 uStack_60;
  int local_44;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  while( true ) {
    while( true ) {
      iVar1 = pcre2_match_8(param_1[1],param_2,param_3,param_4,param_5,param_1[3],param_1[2]);
      if (iVar1 == -0x2e) break;
      if (iVar1 != -0x35) goto LAB_0010ce36;
      pcre2_config_8(7,&local_44,uStack_60);
      bVar4 = local_44 < 0;
      local_44 = local_44 * 2;
      if (bVar4) goto LAB_0010ce36;
      lVar2 = param_1[2];
      if (lVar2 == 0) {
        lVar2 = pcre2_match_context_create_8(*param_1);
        param_1[2] = lVar2;
      }
      uStack_60 = 0x10cdf8;
      pcre2_set_depth_limit_8(lVar2,local_44);
    }
    lVar2 = param_1[5];
    if (0x3fffffffffffffff < lVar2) break;
    param_1[5] = lVar2 * 2;
    pcre2_jit_stack_free_8(param_1[4]);
    lVar2 = pcre2_jit_stack_create_8(lVar2,lVar2 * 2,*param_1);
    param_1[4] = lVar2;
    if (lVar2 == 0) {
                    /* WARNING: Subroutine does not return */
      FUN_0011d4b0();
    }
    lVar3 = param_1[2];
    if (lVar3 == 0) {
      lVar3 = pcre2_match_context_create_8(*param_1);
      lVar2 = param_1[4];
      param_1[2] = lVar3;
    }
    uStack_60 = 0x10cec2;
    pcre2_jit_stack_assign_8(lVar3,0,lVar2);
  }
LAB_0010ce36:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return iVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: Pcompile @ 0xcf00

undefined8 * Pcompile(char *param_1,size_t param_2)

{
  undefined4 uVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  char *pcVar5;
  undefined8 uVar6;
  long lVar7;
  undefined4 *puVar8;
  uint uVar9;
  long in_FS_OFFSET;
  bool bVar10;
  size_t local_168;
  uint local_154;
  undefined1 local_150 [8];
  undefined1 local_148 [264];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  bVar10 = DAT_0012a687 == '\0';
  puVar2 = (undefined8 *)FUN_0011d110(0x38);
  uVar9 = (-(uint)bVar10 & 0xfffffff8) + 0x18;
  uVar3 = pcre2_general_context_create_8(FUN_0010cd70,free,0);
  *puVar2 = uVar3;
  uVar4 = pcre2_compile_context_create_8(uVar3);
  if (DAT_0012a180 != '\0') {
    if (DAT_0012a182 == '\0') {
      uVar3 = dcgettext(0,"-P supports only unibyte and UTF-8 locales",5);
                    /* WARNING: Subroutine does not return */
      error(2,0,uVar3);
    }
    uVar9 = uVar9 | 0x4080000;
  }
  pcVar5 = rawmemchr(param_1,10);
  if (param_1 + param_2 == pcVar5) {
    local_168 = param_2;
    if (DAT_0012a685 == '\0') {
      pcVar5 = (char *)0x0;
      if (DAT_0012a686 != '\0') {
        local_168 = param_2 + 0x11;
        pcVar5 = (char *)FUN_0011d110(local_168);
        *(undefined8 *)pcVar5 = s___<__w)(_:invalid_argument__s_fo_00120990._0_8_;
        *(undefined2 *)(pcVar5 + 8) = s___<__w)(_:invalid_argument__s_fo_00120990._8_2_;
        puVar8 = mempcpy(pcVar5 + 10,param_1,param_2);
        *puVar8 = DAT_00120988;
        *(undefined2 *)(puVar8 + 1) = DAT_0012098c;
        *(undefined1 *)((long)puVar8 + 6) = DAT_0012098e;
        param_1 = pcVar5;
      }
    }
    else {
      pcre2_set_compile_extra_options_8(uVar4,8);
      pcVar5 = (char *)0x0;
    }
    uVar6 = pcre2_maketables_8(uVar3);
    pcre2_set_character_tables_8(uVar4,uVar6);
    lVar7 = pcre2_compile_8(param_1,local_168,uVar9,&local_154,local_150,uVar4);
    puVar2[1] = lVar7;
    if (lVar7 != 0) {
      free(pcVar5);
      pcre2_compile_context_free_8(uVar4);
      puVar2[2] = 0;
      uVar3 = pcre2_match_data_create_from_pattern_8(puVar2[1],uVar3);
      puVar2[3] = uVar3;
      uVar9 = pcre2_jit_compile_8(puVar2[1],1);
      local_154 = uVar9;
      if ((uVar9 + 0x30 < 0x31) &&
         ((0xfffefffffffffff6U >> ((ulong)(uVar9 + 0x30) & 0x3f) & 1) == 0)) {
        puVar2[4] = 0;
        puVar2[5] = 0x8000;
        uVar1 = FUN_0010cd90(puVar2,"",0,0,1);
        *(undefined4 *)(puVar2 + 6) = uVar1;
        uVar1 = FUN_0010cd90(puVar2,"",0,0,0);
        *(undefined4 *)((long)puVar2 + 0x34) = uVar1;
        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
          return puVar2;
        }
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      uVar3 = dcgettext(0,"JIT internal error: %d",5);
                    /* WARNING: Subroutine does not return */
      error(2,0,uVar3,uVar9);
    }
    pcre2_get_error_message_8(local_154,local_148,0x100);
                    /* WARNING: Subroutine does not return */
    error(2,0,&DAT_001209f1,local_148);
  }
  uVar3 = dcgettext(0,"the -P option only supports a single pattern",5);
                    /* WARNING: Subroutine does not return */
  error(2,0,uVar3);
}




// Function: Pexecute @ 0xd220

long Pexecute(long param_1,char *param_2,long param_3,long *param_4,char *param_5)

{
  byte bVar1;
  char cVar2;
  int iVar3;
  long *plVar4;
  char *pcVar5;
  long lVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  char *__s;
  long lVar9;
  char *pcVar10;
  uint uVar11;
  ulong uVar12;
  char *local_60;
  
  __s = param_2;
  if (param_5 != (char *)0x0) {
    __s = param_5;
  }
  uVar12 = (ulong)(__s[-1] == DAT_0012a684);
  plVar4 = (long *)pcre2_get_ovector_pointer_8(*(undefined8 *)(param_1 + 0x18));
  local_60 = param_2;
  do {
    pcVar5 = rawmemchr(__s,(int)DAT_0012a684);
    pcVar10 = local_60;
    while( true ) {
      while (bVar1 = FUN_0010c9d0((int)*__s), (&DAT_0012a183)[bVar1] == -1) {
        __s = __s + 1;
        uVar12 = 0;
        pcVar10 = __s;
      }
      lVar9 = (long)__s - (long)pcVar10;
      if (__s == pcVar5) break;
      uVar11 = (uint)uVar12 ^ 1;
      iVar3 = FUN_0010cd90(param_1,pcVar10,(long)pcVar5 - (long)pcVar10,lVar9,uVar11);
      cVar2 = FUN_0010cef0();
      if (cVar2 == '\0') goto LAB_0010d41b;
      lVar6 = pcre2_get_startchar_8(*(undefined8 *)(param_1 + 0x18));
      if (lVar6 < lVar9) {
        pcVar10 = pcVar10 + lVar6 + 1;
      }
      else {
        if (lVar6 == 0) {
          iVar3 = *(int *)(param_1 + 0x30 + uVar12 * 4);
          *plVar4 = 0;
          plVar4[1] = 0;
        }
        else {
          iVar3 = FUN_0010cd90(param_1,pcVar10,lVar6,lVar9,uVar11 | 0x40000002);
        }
        if (iVar3 != -1) goto LAB_0010d39e;
        __s = pcVar10 + lVar6 + 1;
        uVar12 = 0;
        pcVar10 = __s;
      }
    }
    iVar3 = *(int *)(param_1 + 0x30 + uVar12 * 4);
    plVar4[1] = lVar9;
    *plVar4 = lVar9;
LAB_0010d41b:
    if (iVar3 != -1) {
LAB_0010d39e:
      if (iVar3 < 1) {
        if ((iVar3 < -0x2d) && (-0x40 < iVar3)) {
          switch(iVar3) {
          case -0x3f:
            uVar7 = FUN_00108a50();
            uVar8 = dcgettext(0,"%s: exceeded PCRE\'s heap limit",5);
                    /* WARNING: Subroutine does not return */
            error(2,0,uVar8,uVar7);
          case -0x35:
            uVar7 = FUN_00108a50();
            uVar8 = dcgettext(0,"%s: exceeded PCRE\'s nested backtracking limit",5);
                    /* WARNING: Subroutine does not return */
            error(2,0,uVar8,uVar7);
          case -0x34:
            uVar7 = FUN_00108a50();
            uVar8 = dcgettext(0,"%s: PCRE detected recurse loop",5);
                    /* WARNING: Subroutine does not return */
            error(2,0,uVar8,uVar7);
          case -0x30:
            uVar7 = FUN_00108a50();
            uVar8 = dcgettext(0,"%s: memory exhausted",5);
                    /* WARNING: Subroutine does not return */
            error(2,0,uVar8,uVar7);
          case -0x2f:
            uVar7 = FUN_00108a50();
            uVar8 = dcgettext(0,"%s: exceeded PCRE\'s backtracking limit",5);
                    /* WARNING: Subroutine does not return */
            error(2,0,uVar8,uVar7);
          case -0x2e:
            uVar7 = FUN_00108a50();
            uVar8 = dcgettext(0,"%s: exhausted PCRE JIT stack",5);
                    /* WARNING: Subroutine does not return */
            error(2,0,uVar8,uVar7);
          }
        }
        uVar7 = FUN_00108a50();
        uVar8 = dcgettext(0,"%s: internal PCRE error: %d",5);
                    /* WARNING: Subroutine does not return */
        error(2,0,uVar8,uVar7,iVar3);
      }
      pcVar5 = pcVar5 + 1;
      if (param_5 != (char *)0x0) {
        local_60 = pcVar10 + *plVar4;
        pcVar5 = pcVar10 + plVar4[1];
      }
      *param_4 = (long)pcVar5 - (long)local_60;
      return (long)local_60 - (long)param_2;
    }
    uVar12 = 1;
    __s = pcVar5 + 1;
    local_60 = __s;
    if (param_2 + param_3 <= __s) {
      return -1;
    }
  } while( true );
}




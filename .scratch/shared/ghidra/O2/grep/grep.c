// Function: main @ 0x4ff0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

uint main(int param_1,long param_2)

{
  char cVar1;
  undefined1 auVar2 [16];
  bool bVar3;
  uint uVar4;
  undefined1 uVar5;
  int iVar6;
  int iVar7;
  uint uVar8;
  int iVar9;
  char *pcVar10;
  size_t sVar11;
  FILE *__stream;
  void *__src;
  void *__dest;
  undefined1 *puVar12;
  long lVar13;
  ulong uVar14;
  char *pcVar15;
  char *pcVar16;
  char *pcVar17;
  undefined8 uVar18;
  long lVar19;
  size_t sVar20;
  undefined **ppuVar21;
  stat *psVar22;
  undefined4 *puVar23;
  uint uVar24;
  char *pcVar25;
  size_t sVar26;
  int *piVar27;
  byte *__src_00;
  long in_FS_OFFSET;
  byte bVar28;
  int local_1e4;
  byte *local_1e0;
  int local_1d0;
  uint local_1cc;
  size_t local_1b0;
  long local_1a8;
  long local_1a0;
  undefined1 local_198 [8];
  mbstate_t local_190;
  stat local_188;
  stat local_f8;
  char local_68;
  undefined1 local_67;
  undefined1 local_66;
  char local_53 [3];
  char local_50 [16];
  long local_40;
  
  bVar28 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_0012b684 = 10;
  local_1b0 = 0;
  local_1a8 = 0;
  DAT_0012b080 = 2;
  DAT_0012b804 = 0xffffffff;
  DAT_0012b7e0 = 0x7fffffffffffffff;
  DAT_0012b7f8 = -1;
  DAT_0012b7f0 = -1;
  local_1a0 = -1;
  DAT_0012b8fd = '\0';
  setlocale(6,"");
  bindtextdomain("grep","/usr/local/share/locale");
  textdomain("grep");
  FUN_00116210(&DAT_0012b180);
  FUN_00120120(FUN_00107a00);
  FUN_0010f6a0(0);
  DAT_0012b8c8 = FUN_0011a2d0(0,0,FUN_00107850,FUN_001078a0,0);
  if (DAT_0012b8c8 == 0) {
                    /* WARNING: Subroutine does not return */
    FUN_0011f410();
  }
  local_1cc = 0;
  local_1d0 = 0;
  local_1e4 = -1;
  local_1e0 = (byte *)0x0;
  iVar7 = local_1d0;
switchD_001051f3_caseD_0:
  local_1d0 = iVar7;
  iVar9 = optind;
  bVar3 = false;
  pcVar16 = &local_68;
  while( true ) {
    iVar7 = optind;
    iVar6 = getopt_long(param_1,param_2,"0123456789A:B:C:D:EFGHIPTUVX:abcd:e:f:hiLlm:noqRrsuvwxyZz",
                        &PTR_s_basic_regexp_00129fc0,0);
    if (9 < iVar6 - 0x30U) break;
    pcVar10 = &local_68;
    if (((DAT_0012b010 == iVar7) && (bVar3)) &&
       (pcVar10 = pcVar16 + -(ulong)(local_68 == '0'),
       pcVar16 + -(ulong)(local_68 == '0') == local_53)) {
      builtin_strncpy(pcVar16 + -(ulong)(local_68 == '0'),"...",4);
      pcVar16 = local_50;
      goto LAB_001051b5;
    }
    pcVar16 = pcVar10 + 1;
    bVar3 = true;
    *pcVar10 = (char)iVar6;
    DAT_0012b010 = iVar7;
  }
  if (pcVar16 != &local_68) {
LAB_001051b5:
    *pcVar16 = '\0';
    FUN_00107d10(&local_68);
  }
  pcVar16 = optarg;
  if (iVar6 != -1) goto code_r0x001051e3;
  if (DAT_0012b905 != '\0') {
    uVar18 = FUN_00119950();
    FUN_0011ed40(stdout,uVar18,"GNU grep",&DAT_001211d1,0);
    pcVar16 = (char *)dcgettext(0,
                                "Written by Mike Haertel and others; see\n<https://git.sv.gnu.org/cgit/grep.git/tree/AUTHORS>."
                                ,5);
    puts(pcVar16);
    uVar24 = 0;
    goto LAB_001059cc;
  }
  if (DAT_0012b908 != 0) {
    FUN_0010a170(0);
    goto LAB_00106713;
  }
  if (local_1e0 == (byte *)0x0) {
    lVar13 = (long)optind;
    if (param_1 <= optind) goto switchD_001051f3_caseD_1;
    optind = optind + 1;
    uVar14 = 0;
    pcVar16 = *(char **)(param_2 + lVar13 * 8);
    if ((local_1e4 != 2) && (*pcVar16 == '\\')) {
      uVar14 = (ulong)(pcVar16[1] == '-');
    }
    local_1e0 = (byte *)FUN_0011f3f0(pcVar16 + uVar14);
    DAT_0012b8d8 = local_1e0;
    sVar20 = strlen((char *)local_1e0);
    local_1e0[sVar20] = 10;
    local_1b0 = FUN_00107a20(local_1e0,0,sVar20 + 1,"");
  }
  else if (local_1b0 == 0) {
    DAT_0012b686 = 0;
    DAT_0012b685 = '\0';
    DAT_0012b802 = DAT_0012b802 ^ 1;
    local_1b0 = 1;
    *local_1e0 = 10;
  }
  sVar20 = local_1b0;
  sVar11 = local_1b0 - 1;
  local_1b0 = sVar11;
  FUN_0011a490(DAT_0012b8c8);
  if (DAT_0012b7a1 != '\0') {
    bVar3 = false;
    goto LAB_00105965;
  }
  iVar7 = fstat(1,&local_188);
  bVar3 = false;
  if (iVar7 == 0) {
    if ((local_188.st_mode & 0xf000) == 0x8000) {
      psVar22 = &local_188;
      puVar23 = &DAT_0012b920;
      for (lVar13 = 0x24; lVar13 != 0; lVar13 = lVar13 + -1) {
        *puVar23 = (int)psVar22->st_dev;
        psVar22 = (stat *)((long)psVar22 + (ulong)bVar28 * -8 + 4);
        puVar23 = puVar23 + (ulong)bVar28 * -2 + 1;
      }
    }
    else if ((local_188.st_mode & 0xf000) == 0x2000) {
      iVar7 = stat("/dev/null",&local_f8);
      if (((iVar7 == 0) && (local_188.st_ino == local_f8.st_ino)) &&
         (local_188.st_dev == local_f8.st_dev)) {
        DAT_0012b7a0 = '\x01';
      }
      else {
        bVar3 = true;
      }
    }
  }
  if (DAT_0012b7a0 == '\0') {
    if (DAT_0012b80c != 0) goto LAB_0010596f;
  }
  else {
LAB_00105965:
    DAT_0012b80c = 0;
LAB_0010596f:
    DAT_0012b7e8 = 0;
    DAT_0012b7a2 = 1;
  }
  DAT_0012b803 = DAT_0012b7e8 | DAT_0012b7a2;
  if (DAT_0012b7f0 < 0) {
    DAT_0012b7f0 = local_1a0;
  }
  if (DAT_0012b7f8 < 0) {
    DAT_0012b7f8 = local_1a0;
  }
  if (((DAT_0012b7e0 == 0) ||
      (((sVar11 == 0 && (DAT_0012b802 != 0)) && ((DAT_0012b685 == '\0' && (DAT_0012b686 == 0))))))
     && (DAT_0012b80c != 2)) {
    uVar24 = 1;
    goto LAB_001059cc;
  }
  if (DAT_0012b900 == 2) {
    if ((bVar3) && (iVar7 = FUN_0011fa50(), iVar7 != 0)) {
      iVar7 = isatty(1);
      DAT_0012b900 = (uint)(iVar7 != 0);
    }
    else {
      DAT_0012b900 = 0;
    }
  }
  FUN_0011fa90();
  if (DAT_0012b900 != 0) {
    pcVar15 = getenv("GREP_COLOR");
    pcVar16 = PTR_s_01_31_0012b060;
    pcVar10 = PTR_s_01_31_0012b068;
    if ((pcVar15 != (char *)0x0) && (pcVar17 = pcVar15, *pcVar15 != '\0')) {
      for (; ((*pcVar17 == ';' ||
              (pcVar16 = PTR_s_01_31_0012b060, pcVar10 = PTR_s_01_31_0012b068,
              (int)*pcVar17 - 0x30U < 10)) &&
             (pcVar16 = pcVar15, pcVar10 = pcVar15, pcVar17[1] != '\0')); pcVar17 = pcVar17 + 1) {
      }
    }
    PTR_s_01_31_0012b068 = pcVar10;
    PTR_s_01_31_0012b060 = pcVar16;
    pcVar16 = getenv("GREP_COLORS");
    if ((pcVar16 != (char *)0x0) && (*pcVar16 != '\0')) {
      pcVar17 = (char *)FUN_0011f3f0();
      pcVar10 = (char *)0x0;
      pcVar16 = pcVar17;
LAB_001062d6:
      do {
        while( true ) {
          pcVar25 = pcVar16 + 1;
          cVar1 = *pcVar16;
          if ((cVar1 != ':') && (cVar1 != '\0')) break;
          *pcVar16 = '\0';
          ppuVar21 = &PTR_DAT_0012a620;
          pcVar16 = "mt";
          do {
            iVar7 = strcmp(pcVar16,pcVar17);
            if (iVar7 == 0) break;
            pcVar16 = ppuVar21[3];
            ppuVar21 = ppuVar21 + 3;
          } while (pcVar16 != (char *)0x0);
          if (((undefined8 *)ppuVar21[1] != (undefined8 *)0x0) && (pcVar10 != (char *)0x0)) {
            *(char **)ppuVar21[1] = pcVar10;
          }
          if ((code *)ppuVar21[2] != (code *)0x0) {
            (*(code *)ppuVar21[2])();
          }
          if (cVar1 == '\0') goto LAB_0010631c;
          pcVar10 = (char *)0x0;
          pcVar17 = pcVar25;
          pcVar16 = pcVar25;
        }
        if (cVar1 == '=') {
          if ((pcVar17 == pcVar16) || (pcVar10 != (char *)0x0)) break;
          *pcVar16 = '\0';
          pcVar10 = pcVar25;
          pcVar16 = pcVar25;
          goto LAB_001062d6;
        }
        pcVar16 = pcVar25;
      } while (((pcVar10 == (char *)0x0) || (cVar1 == ';')) || ((int)cVar1 - 0x30U < 10));
    }
LAB_0010631c:
    if ((PTR_s_01_31_0012b068 == pcVar15) || (PTR_s_01_31_0012b060 == pcVar15)) {
      uVar18 = dcgettext(0,"warning: GREP_COLOR=\'%s\' is deprecated; use GREP_COLORS=\'mt=%s\'",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar18,pcVar15,pcVar15);
    }
  }
  lVar13 = 1;
  uVar24 = 1;
  uVar8 = 0;
  do {
    while (((&DAT_0012b183)[lVar13] == '\x01' ||
           (uVar4 = uVar24, (uVar8 & 0xff & (uint)lVar13) != 0))) {
      lVar13 = lVar13 + 1;
      if (lVar13 == 0x100) goto LAB_00105ad9;
    }
    do {
      uVar24 = uVar4;
      uVar4 = uVar24 * 2;
    } while ((int)(uVar24 * 2) <= (int)(uint)lVar13);
    lVar13 = lVar13 + 1;
    uVar8 = uVar8 | uVar24;
  } while (lVar13 != 0x100);
LAB_00105ad9:
  DAT_0012b868 = (ulong)(uVar8 & 0xff) * 0x101010101010101;
  iVar7 = 0;
  if (-1 < local_1e4) {
    iVar7 = local_1e4;
  }
  if (2 < local_1e4) goto LAB_00105d5b;
  if (DAT_0012b687 != '\0') {
    piVar27 = &DAT_0012b284;
    puVar12 = &DAT_0012b6a0;
    do {
      if (*piVar27 != -1) {
        iVar9 = FUN_00116380(*piVar27,&local_f8);
        if (0 < iVar9) {
          lVar13 = (long)local_f8.__unused + (long)iVar9 * 4 + -0x78;
          do {
            local_190.__count = 0;
            local_190.__value = (_union_27)0x0;
            sVar26 = wcrtomb(&local_68,*(wchar_t *)(lVar13 + -4),&local_190);
            if (sVar26 != 1) {
              uVar5 = 0xff;
              goto LAB_0010619c;
            }
            lVar13 = lVar13 + -4;
          } while ((long)&local_188 + (ulong)(iVar9 - 1) * -4 + (long)iVar9 * 4 + 0x8c != lVar13);
        }
        uVar5 = 1;
LAB_0010619c:
        *puVar12 = uVar5;
      }
      piVar27 = piVar27 + 1;
      puVar12 = puVar12 + 1;
    } while (piVar27 != (int *)&DAT_0012b684);
  }
  if (local_1e4 != 2) {
    if (DAT_0012b8d0 < 2) goto LAB_00105d5b;
    __src = (void *)FUN_0011ef30(sVar20);
    local_f8.st_dev = 0;
    __dest = __src;
    __src_00 = local_1e0;
    for (sVar20 = sVar11; sVar20 != 0; sVar20 = sVar20 - sVar26) {
      bVar28 = *__src_00;
      if (bVar28 == 0x5b) goto LAB_00105d53;
      if ((char)bVar28 < '\\') {
        if ((char)bVar28 < '/') {
          if ('#' < (char)bVar28) {
            uVar14 = 1L << (bVar28 & 0x3f);
            if ((uVar14 & 0x441000000000) != 0) goto LAB_00105d53;
            if ((uVar14 & 0x90000000000) != 0) goto LAB_001061c8;
          }
        }
        else if (bVar28 == 0x3f) {
LAB_001061c8:
          if (local_1e4 == 1) goto LAB_00105d53;
        }
      }
      else if (bVar28 == 0x5c) {
        if (1 < (long)sVar20) {
          bVar28 = __src_00[1];
          if ((char)bVar28 < '=') {
            if ('\t' < (char)bVar28) {
              uVar14 = 1L << (bVar28 & 0x3f);
              if ((uVar14 & 0x13fe008000000400) == 0) {
                uVar14 = uVar14 & 0xb0000000000;
                goto joined_r0x00106596;
              }
              goto LAB_00105d53;
            }
          }
          else if ((byte)(bVar28 - 0x3e) < 0x3f) {
            uVar14 = 1L << (bVar28 - 0x3e & 0x3f);
            if ((uVar14 & 0x220001402200011) == 0) {
              uVar14 = uVar14 & 0x6000000000000002;
joined_r0x00106596:
              if ((uVar14 == 0) || (local_1e4 == 1)) goto LAB_0010622b;
              iVar7 = 0;
            }
            goto LAB_00105d53;
          }
LAB_0010622b:
          __src_00 = __src_00 + 1;
          sVar20 = sVar20 - 1;
        }
      }
      else {
        if (bVar28 == 0x5e) goto LAB_00105d53;
        if ((byte)(bVar28 + 0x85) < 2) goto LAB_001061c8;
      }
      if (DAT_0012b687 == '\0') {
        sVar26 = FUN_0010e540(__src_00,sVar20,&local_f8);
      }
      else if ((&DAT_0012b284)[*__src_00] == -1) {
        sVar26 = FUN_00107d80(__src_00,sVar20,&local_f8);
      }
      else {
        sVar26 = (size_t)(char)(&DAT_0012b6a0)[*__src_00];
      }
      if ((long)sVar26 < 0) goto LAB_00105d53;
      __dest = mempcpy(__dest,__src_00,sVar26);
      __src_00 = __src_00 + sVar26;
    }
    iVar7 = 2;
    sVar20 = (long)__dest - (long)__src;
    if (sVar11 != sVar20) {
      local_1b0 = sVar20;
      puVar12 = mempcpy(local_1e0,__src,sVar20);
      *puVar12 = 10;
    }
LAB_00105d53:
    free(__src);
    goto LAB_00105d5b;
  }
  if (DAT_0012b180 == '\0') {
    bVar28 = 0;
    if (DAT_0012b8d0 != 1) goto LAB_001060b9;
    goto LAB_00106502;
  }
  local_f8.st_dev = 0;
  for (lVar13 = 0; lVar13 < (long)sVar11; lVar13 = lVar13 + uVar14) {
    uVar14 = (ulong)(char)(&DAT_0012b183)[local_1e0[lVar13]];
    if ((((&DAT_0012b183)[local_1e0[lVar13]] != -2) ||
        (uVar14 = FUN_0011ac90(local_1e0 + lVar13,sVar11 - lVar13,&local_f8), 0x10 < uVar14)) &&
       ((long)uVar14 < 0)) {
LAB_0010663d:
      bVar28 = 1;
      goto LAB_001060b9;
    }
  }
  if (DAT_0012b687 != '\0') {
    local_f8.st_dev = 0;
    for (lVar13 = 0; lVar13 < (long)sVar11; lVar13 = lVar13 + lVar19) {
      bVar28 = local_1e0[lVar13];
      if ((&DAT_0012b284)[bVar28] == -1) {
        lVar19 = FUN_00107d80(local_1e0 + lVar13,sVar11 - lVar13,&local_f8);
      }
      else {
        lVar19 = (long)(char)(&DAT_0012b6a0)[bVar28];
      }
      if (lVar19 < 0) goto LAB_0010663d;
    }
  }
  bVar28 = 0;
LAB_001060b9:
  iVar7 = 2;
  if ((bVar28 & 1) != 0) {
    FUN_0010a3e0(&DAT_0012b8d8,&local_1b0);
    local_1e0 = DAT_0012b8d8;
    iVar7 = 0;
  }
LAB_00105d5b:
  lVar13 = (long)iVar7 * 0x20;
  DAT_0012b878 = (code *)(&PTR_FUN_0012a7b8)[(long)iVar7 * 4];
  DAT_0012b870 = (**(code **)(&UNK_0012a7b0 + lVar13))
                           (local_1e0,local_1b0,(long)*(int *)(&DAT_0012a7ac + lVar13),
                            DAT_0012b8fd != '\0' || DAT_0012b900 != 0);
  local_67 = DAT_0012b684;
  local_68 = '\0';
  local_66 = 0;
  lVar13 = (*DAT_0012b878)(DAT_0012b870,&local_67,1,local_198,0);
  DAT_0012b820 = (lVar13 == 0) == (bool)DAT_0012b802;
  param_1 = param_1 - optind;
  if ((local_1cc == 0) && (param_1 < 2)) {
    DAT_0012b808 = -(uint)(DAT_0012b018 == 3);
  }
  else {
    DAT_0012b808 = ~local_1cc >> 0x1f;
  }
  uVar14 = sysconf(0x1e);
  if (0x3ffffffffffffffa < uVar14 - 1) {
LAB_00106713:
                    /* WARNING: Subroutine does not return */
    abort();
  }
  auVar2._8_8_ = 0;
  auVar2._0_8_ = uVar14;
  lVar19 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x18000)) % auVar2,0);
  lVar13 = 0x18000;
  if (lVar19 != 0) {
    lVar13 = (uVar14 + 0x18000) - lVar19;
  }
  DAT_0012b858 = uVar14 + 8 + lVar13;
  DAT_0012b838 = uVar14;
  DAT_0012b860 = FUN_0011ef30();
  if (((_DAT_0012b014 & 2) != 0) && (DAT_0012b884 == 0)) {
    DAT_0012b884 = 1;
  }
  if (param_1 < 1) {
    if ((DAT_0012b018 == 3) && (0 < local_1d0)) {
      DAT_0012b88a = 1;
      ppuVar21 = &PTR_DAT_00129f90;
    }
    else {
      ppuVar21 = &PTR_DAT_00129f80;
    }
  }
  else {
    ppuVar21 = (undefined **)(param_2 + (long)optind * 8);
  }
  pcVar16 = *ppuVar21;
  uVar24 = 1;
  do {
    while( true ) {
      ppuVar21 = ppuVar21 + 1;
      iVar7 = strcmp(pcVar16,"-");
      if (iVar7 == 0) break;
      DAT_0012b890 = pcVar16;
      uVar8 = FUN_0010a0f0(0xffffff9c,pcVar16,1,1);
      pcVar16 = *ppuVar21;
      uVar24 = uVar24 & uVar8;
      if (pcVar16 == (char *)0x0) goto LAB_00105f29;
    }
    DAT_0012b890 = DAT_0012b7d0;
    uVar8 = FUN_00109130(0,1);
    pcVar16 = *ppuVar21;
    uVar24 = uVar24 & uVar8;
  } while (pcVar16 != (char *)0x0);
LAB_00105f29:
  if (DAT_0012b889 != '\0') {
    uVar24 = 2;
  }
LAB_001059cc:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar24;
code_r0x001051e3:
  iVar7 = local_1d0;
  switch(iVar6) {
  case 0:
  case 0x55:
    goto switchD_001051f3_caseD_0;
  default:
    goto switchD_001051f3_caseD_1;
  case 0x41:
    FUN_00107d10(optarg,&DAT_0012b7f0);
    goto switchD_001051f3_caseD_0;
  case 0x42:
    FUN_00107d10(optarg,&DAT_0012b7f8);
    goto switchD_001051f3_caseD_0;
  case 0x43:
    FUN_00107d10(optarg,&local_1a0);
    goto switchD_001051f3_caseD_0;
  case 0x44:
    iVar9 = strcmp(optarg,"read");
    if (iVar9 == 0) {
      DAT_0012b884 = 1;
    }
    else {
      iVar9 = strcmp(pcVar16,"skip");
      if (iVar9 != 0) {
        uVar18 = dcgettext(0,"unknown devices method",5);
                    /* WARNING: Subroutine does not return */
        error(2,0,uVar18);
      }
      DAT_0012b884 = 2;
    }
    goto switchD_001051f3_caseD_0;
  case 0x45:
    local_1e4 = FUN_00107950("egrep",local_1e4);
    goto switchD_001051f3_caseD_0;
  case 0x46:
    local_1e4 = FUN_00107950("fgrep",local_1e4);
    goto switchD_001051f3_caseD_0;
  case 0x47:
    local_1e4 = FUN_00107950("grep",local_1e4);
    goto switchD_001051f3_caseD_0;
  case 0x48:
    local_1cc = 1;
    goto switchD_001051f3_caseD_0;
  case 0x4c:
    DAT_0012b80c = 2;
    goto switchD_001051f3_caseD_0;
  case 0x50:
    local_1e4 = FUN_00107950(&DAT_00121165,local_1e4);
    goto switchD_001051f3_caseD_0;
  case 0x52:
    _DAT_0012b014 = 0x30a;
  case 0x72:
    DAT_0012b018 = 3;
    iVar7 = iVar9;
    goto switchD_001051f3_caseD_0;
  case 0x54:
    DAT_0012b8fc = 1;
    goto switchD_001051f3_caseD_0;
  case 0x56:
    DAT_0012b905 = '\x01';
    goto switchD_001051f3_caseD_0;
  case 0x58:
    local_1e4 = FUN_00107950(optarg,local_1e4);
    goto switchD_001051f3_caseD_0;
  case 0x5a:
    DAT_0012b804 = 0;
    goto switchD_001051f3_caseD_0;
  case 0x61:
switchD_001051f3_caseD_61:
    DAT_0012b810 = 1;
    goto switchD_001051f3_caseD_0;
  case 0x62:
    DAT_0012b800 = 1;
    goto switchD_001051f3_caseD_0;
  case 99:
    DAT_0012b7e8 = 1;
    goto switchD_001051f3_caseD_0;
  case 100:
    lVar13 = FUN_0010f3f0("--directories",optarg,&PTR_DAT_00129fa0,&DAT_00122788,4,PTR_FUN_0012b078,
                          1);
    DAT_0012b018 = *(int *)(&DAT_00122788 + lVar13 * 4);
    iVar7 = iVar9;
    if (DAT_0012b018 != 3) {
      iVar7 = local_1d0;
    }
    goto switchD_001051f3_caseD_0;
  case 0x65:
    sVar11 = strlen(optarg);
    sVar20 = local_1b0;
    lVar13 = sVar11 + 1 + (local_1b0 - local_1a8);
    if (0 < lVar13) {
      DAT_0012b8d8 = (byte *)FUN_0011f1f0(local_1e0,&local_1a8,lVar13,0xffffffffffffffff);
      pcVar16 = optarg;
      local_1e0 = DAT_0012b8d8;
    }
    puVar12 = mempcpy(local_1e0 + sVar20,pcVar16,sVar11);
    *puVar12 = 10;
    local_1b0 = FUN_00107a20(local_1e0,sVar20,sVar11 + 1 + sVar20,"");
    goto switchD_001051f3_caseD_0;
  case 0x66:
    break;
  case 0x68:
    local_1cc = 0xffffffff;
    goto switchD_001051f3_caseD_0;
  case 0x69:
  case 0x79:
    DAT_0012b687 = '\x01';
    goto switchD_001051f3_caseD_0;
  case 0x6c:
    DAT_0012b80c = 1;
    goto switchD_001051f3_caseD_0;
  case 0x6d:
    uVar24 = FUN_0011f450(optarg,0,10,&DAT_0012b7e0);
    if (1 < uVar24) {
      uVar18 = dcgettext(0,"invalid max count",5);
                    /* WARNING: Subroutine does not return */
      error(2,0,uVar18);
    }
    goto switchD_001051f3_caseD_0;
  case 0x6e:
    DAT_0012b801 = 1;
    goto switchD_001051f3_caseD_0;
  case 0x6f:
    DAT_0012b8fd = '\x01';
    goto switchD_001051f3_caseD_0;
  case 0x71:
    DAT_0012b7a1 = '\x01';
    DAT_0012b080 = 0;
    goto switchD_001051f3_caseD_0;
  case 0x73:
    DAT_0012b904 = 1;
    goto switchD_001051f3_caseD_0;
  case 0x75:
    uVar18 = dcgettext(0,"warning: --unix-byte-offsets (-u) is obsolete",5);
                    /* WARNING: Subroutine does not return */
    error(0,0,uVar18);
  case 0x76:
    DAT_0012b802 = 1;
    goto switchD_001051f3_caseD_0;
  case 0x77:
    FUN_0010e570();
    DAT_0012b686 = 1;
    goto switchD_001051f3_caseD_0;
  case 0x78:
    DAT_0012b685 = '\x01';
    goto switchD_001051f3_caseD_0;
  case 0x7a:
    DAT_0012b684 = 0;
    goto switchD_001051f3_caseD_0;
  case 0x80:
    iVar9 = strcmp(optarg,"binary");
    if (iVar9 == 0) {
      DAT_0012b810 = 0;
      goto switchD_001051f3_caseD_0;
    }
    iVar9 = strcmp(pcVar16,"text");
    if (iVar9 == 0) goto switchD_001051f3_caseD_61;
    iVar9 = strcmp(pcVar16,"without-match");
    if (iVar9 != 0) {
      uVar18 = dcgettext(0,"unknown binary-files type",5);
                    /* WARNING: Subroutine does not return */
      error(2,0,uVar18);
    }
  case 0x49:
    DAT_0012b810 = 2;
    goto switchD_001051f3_caseD_0;
  case 0x81:
    if (optarg != (char *)0x0) {
      iVar9 = FUN_0010f750(optarg,"always");
      if (((iVar9 == 0) || (iVar9 = FUN_0010f750(pcVar16,&DAT_001211ad), iVar9 == 0)) ||
         (iVar9 = FUN_0010f750(pcVar16,"force"), iVar9 == 0)) {
        DAT_0012b900 = 1;
        goto switchD_001051f3_caseD_0;
      }
      iVar9 = FUN_0010f750(pcVar16,"never");
      if (((iVar9 == 0) || (iVar9 = FUN_0010f750(pcVar16,&DAT_001211bd), iVar9 == 0)) ||
         (iVar9 = FUN_0010f750(pcVar16,&DAT_001211c0), iVar9 == 0)) {
        DAT_0012b900 = 0;
        goto switchD_001051f3_caseD_0;
      }
      iVar9 = FUN_0010f750(pcVar16,&DAT_001211c5);
      if (((iVar9 != 0) && (iVar9 = FUN_0010f750(pcVar16,&DAT_001211cd), iVar9 != 0)) &&
         (iVar9 = FUN_0010f750(pcVar16,&DAT_001211ca), iVar9 != 0)) {
        DAT_0012b908 = 1;
        goto switchD_001051f3_caseD_0;
      }
    }
    DAT_0012b900 = 2;
    goto switchD_001051f3_caseD_0;
  case 0x82:
    FUN_00116480(optarg);
    if (DAT_0012b8a0 == 0) {
      DAT_0012b8a0 = FUN_001169d0();
    }
    FUN_00116d40(DAT_0012b8a0,optarg,0x50000000);
    if (DAT_0012b8a8 == 0) {
      DAT_0012b8a8 = FUN_001169d0();
    }
    FUN_00116d40(DAT_0012b8a8,optarg,0x10000000);
    goto switchD_001051f3_caseD_0;
  case 0x83:
  case 0x86:
    if (DAT_0012b8b0 == 0) {
      DAT_0012b8b0 = FUN_001169d0();
    }
    uVar24 = (uint)(iVar6 == 0x86) << 0x1d;
    FUN_00116d40(DAT_0012b8b0,optarg,uVar24 | 0x50000000);
    if (DAT_0012b8b8 == 0) {
      DAT_0012b8b8 = FUN_001169d0();
    }
    FUN_00116d40(DAT_0012b8b8,optarg,uVar24 | 0x10000000);
    goto switchD_001051f3_caseD_0;
  case 0x84:
    if (DAT_0012b8b0 == 0) {
      DAT_0012b8b0 = FUN_001169d0();
    }
    iVar9 = FUN_00117290(FUN_00116d40,DAT_0012b8b0,optarg,0x50000000,10);
    if (iVar9 == 0) {
      if (DAT_0012b8b8 == 0) {
        DAT_0012b8b8 = FUN_001169d0();
      }
      iVar9 = FUN_00117290(FUN_00116d40,DAT_0012b8b8,optarg,0x10000000);
      if (iVar9 == 0) goto switchD_001051f3_caseD_0;
    }
    piVar27 = __errno_location();
                    /* WARNING: Subroutine does not return */
    error(2,*piVar27,&DAT_001229f1,optarg);
  case 0x85:
    PTR_DAT_0012b070 = optarg;
    goto switchD_001051f3_caseD_0;
  case 0x87:
    DAT_0012b7d8 = 1;
    goto switchD_001051f3_caseD_0;
  case 0x88:
    DAT_0012b7d0 = optarg;
    goto switchD_001051f3_caseD_0;
  case 0x89:
    DAT_0012b687 = '\0';
    goto switchD_001051f3_caseD_0;
  }
  iVar9 = strcmp(optarg,"-");
  sVar11 = local_1b0;
  sVar20 = local_1b0;
  __stream = stdin;
  if ((iVar9 != 0) &&
     (__stream = fopen(pcVar16,"r"), sVar11 = local_1b0, sVar20 = local_1b0, __stream == (FILE *)0x0
     )) {
    piVar27 = __errno_location();
                    /* WARNING: Subroutine does not return */
    error(2,*piVar27,&DAT_001229f1,optarg);
  }
  while( true ) {
    lVar13 = (sVar11 - local_1a8) + 2;
    if (0 < lVar13) {
      local_1e0 = (byte *)FUN_0011f1f0(local_1e0,&local_1a8,lVar13,0xffffffffffffffff);
      DAT_0012b8d8 = local_1e0;
    }
    sVar26 = fread_unlocked(local_1e0 + sVar11,1,local_1a8 - (sVar11 + 1),__stream);
    if (sVar26 == 0) break;
    sVar11 = sVar11 + sVar26;
  }
  piVar27 = __errno_location();
  iVar9 = *piVar27;
  if ((__stream->_flags & 0x20) == 0) {
    if (stdin == __stream) {
      clearerr_unlocked(__stream);
      goto LAB_00105751;
    }
    iVar9 = fclose(__stream);
    if (iVar9 == 0) goto LAB_00105751;
    iVar9 = *piVar27;
  }
  if (iVar9 != 0) {
                    /* WARNING: Subroutine does not return */
    error(2,iVar9,&DAT_001229f1,optarg);
  }
LAB_00105751:
  if ((sVar11 != sVar20) && (local_1e0[sVar11 - 1] != 10)) {
    local_1e0[sVar11] = 10;
    sVar11 = sVar11 + 1;
  }
  local_1b0 = FUN_00107a20(local_1e0,sVar20,sVar11,optarg);
  goto switchD_001051f3_caseD_0;
switchD_001051f3_caseD_1:
  FUN_0010a170(2);
LAB_00106502:
  bVar28 = DAT_0012b686;
  goto LAB_001060b9;
}




// Function: regex_compile @ 0x6840

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
  if ((DAT_0012b687 == '\0') && ((char)param_7 == '\0')) {
    local_88.fastmap = (char *)FUN_0011ef30(0x100);
  }
  else {
    local_88.fastmap = (char *)0x0;
    local_88.translate = (uchar *)0x0;
    if ((char)param_7 != '\0') {
      re_set_syntax(param_6 | 0x2000000);
      pcVar1 = re_compile_pattern(param_2,param_3,&local_88);
      if (pcVar1 == (char *)0x0) {
        regfree(&local_88);
        goto LAB_00106912;
      }
      goto LAB_001068e7;
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
LAB_00106912:
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      return param_7;
    }
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
LAB_001068e7:
  free(local_88.fastmap);
  if (param_5 != -1) {
    pcVar2 = (char *)FUN_00109080(param_5,&local_90);
    if (*pcVar2 != '\0') {
                    /* WARNING: Subroutine does not return */
      error(0,0,"%s:%td: %s",pcVar2,local_90,pcVar1);
    }
  }
                    /* WARNING: Subroutine does not return */
  error(0,0,&DAT_001229f1,pcVar1);
}




// Function: dfaerror @ 0x6a10

void dfaerror(undefined8 param_1)

{
                    /* WARNING: Subroutine does not return */
  error(2,0,&DAT_001229f1,param_1);
}




// Function: dfawarn @ 0x6a40

void dfawarn(undefined8 param_1)

{
  undefined8 uVar1;
  
  uVar1 = dcgettext(0,"warning: %s",5);
                    /* WARNING: Subroutine does not return */
  error(0,0,uVar1,param_1);
}




// Function: GEAcompile @ 0x6a70

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * GEAcompile(char *param_1,size_t param_2,ulong param_3,char param_4)

{
  byte bVar1;
  bool bVar2;
  undefined8 uVar3;
  char cVar4;
  char cVar5;
  char cVar6;
  undefined8 *puVar7;
  undefined8 uVar8;
  char *pcVar9;
  long lVar10;
  void *pvVar11;
  char *pcVar12;
  size_t sVar13;
  char *__ptr;
  long lVar14;
  char *pcVar15;
  long in_FS_OFFSET;
  bool bVar16;
  undefined8 local_b0;
  char *local_90;
  size_t local_88;
  char *local_80;
  char *local_70;
  size_t local_68;
  long local_50;
  long local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  puVar7 = (undefined8 *)FUN_0011f2f0(1,0x48);
  uVar8 = FUN_00115a90();
  bVar16 = DAT_0012b687 != '\0';
  puVar7[1] = uVar8;
  if (bVar16) {
    param_3 = param_3 | 0x400000;
  }
  FUN_00115fa0(uVar8,&DAT_0012b180,param_3,
               (uint)(DAT_0012b684 == '\0') * 2 | (uint)param_3 & 0x10 | 0x2c);
  cVar4 = DAT_0012b182;
  cVar6 = DAT_0012b180;
  pcVar9 = param_1 + param_2;
  lVar10 = FUN_0011ef10(0x40);
  bVar16 = false;
  puVar7[3] = 0;
  puVar7[2] = lVar10 + 0x40;
  lVar10 = 0;
  local_50 = 1;
  local_48 = 0;
  local_88 = 0;
  local_80 = (char *)0x0;
  pcVar15 = param_1;
  local_90 = param_1;
  do {
    pvVar11 = rawmemchr(pcVar15,10);
    lVar14 = ((long)pvVar11 - (long)pcVar15) + -1;
    if (-1 < lVar14) {
      pcVar12 = pcVar15;
      while (pcVar12 = memchr(pcVar12,0x5c,(long)(pcVar15 + lVar14) - (long)pcVar12),
            pcVar12 != (char *)0x0) {
        if ((byte)(pcVar12[1] - 0x31U) < 9) {
          if (local_90 < pcVar15) {
            sVar13 = (long)pcVar15 - (long)local_90;
            lVar14 = (local_88 - local_48) + sVar13;
            if (0 < lVar14) {
              local_80 = (char *)FUN_0011f1f0(local_80,&local_48,lVar14,0xffffffffffffffff,1);
            }
            memcpy(local_80 + local_88,local_90,sVar13);
            local_88 = local_88 + sVar13;
            bVar2 = true;
            local_b0 = 0;
          }
          else {
            local_b0 = 0;
            bVar2 = true;
          }
          goto LAB_00106bfa;
        }
        if (((int)pcVar12[1] == (-(uint)(cVar6 == '\x01' && cVar4 == '\0') & 0x24) + 0x5c) &&
           (pcVar12 = pcVar12 + 1, pcVar12 == pcVar15 + lVar14)) break;
        pcVar12 = pcVar12 + 1;
      }
    }
    local_b0 = 1;
    bVar2 = false;
LAB_00106bfa:
    lVar14 = (puVar7[3] - local_50) + 2;
    if (0 < lVar14) {
      lVar14 = FUN_0011f1f0(puVar7[2] + -0x40,&local_50,lVar14,0xffffffffffffffff,0x40);
      puVar7[2] = lVar14 + 0x40;
    }
    re_set_syntax(param_3);
    cVar5 = FUN_00106840(puVar7,pcVar15,(long)pvVar11 - (long)pcVar15,puVar7[3],lVar10,param_3,
                         local_b0);
    if (cVar5 == '\0') {
      bVar16 = true;
    }
    pcVar15 = (char *)((long)pvVar11 + 1);
    lVar10 = lVar10 + 1;
    if (bVar2) {
      puVar7[3] = puVar7[3] + 1;
      local_90 = pcVar15;
    }
    if (pcVar9 < pcVar15) {
      if (bVar16) {
                    /* WARNING: Subroutine does not return */
        exit(2);
      }
      sVar13 = local_88;
      pcVar15 = local_80;
      if ((local_90 <= pcVar9) && (sVar13 = param_2, pcVar15 = param_1, param_1 < local_90)) {
        sVar13 = local_88 + ((long)pcVar9 - (long)local_90);
        pcVar15 = (char *)FUN_0011efa0(local_80,sVar13);
        memcpy(pcVar15 + local_88,local_90,(long)pcVar9 - (long)local_90);
      }
      local_80 = pcVar15;
      local_88 = sVar13;
      if ((DAT_0012b686 != '\0') ||
         (pcVar15 = (char *)0x0, local_70 = param_1, local_68 = param_2, DAT_0012b685 != '\0')) {
        pcVar15 = (char *)FUN_0011ef30(param_2 + 0x2d);
        uVar3 = _UNK_00121088;
        uVar8 = _UNK_00121068;
        if (DAT_0012b685 == '\0') {
          if ((param_3 & 0x2000) == 0) {
            *(undefined8 *)pcVar15 = _DAT_00121080;
            *(undefined8 *)(pcVar15 + 8) = uVar3;
            *(undefined4 *)(pcVar15 + 0x10) = DAT_00121090;
            *(undefined2 *)(pcVar15 + 0x14) = DAT_00121094;
            pcVar15[0x16] = DAT_00121096;
            sVar13 = strlen(pcVar15);
            lVar10 = sVar13 + param_2;
            memcpy(pcVar15 + sVar13,param_1,param_2);
            pcVar9 = "\\)\\([^[:alnum:]_]\\|$\\)";
          }
          else {
            *(undefined8 *)pcVar15 = _DAT_00121060;
            *(undefined8 *)(pcVar15 + 8) = uVar8;
            *(undefined2 *)(pcVar15 + 0x10) = DAT_00121070;
            pcVar15[0x12] = DAT_00121072;
            sVar13 = strlen(pcVar15);
            lVar10 = sVar13 + param_2;
            memcpy(pcVar15 + sVar13,param_1,param_2);
            pcVar9 = ")([^[:alnum:]_]|$)";
          }
        }
        else if ((param_3 & 0x2000) == 0) {
          *(undefined4 *)pcVar15 = DAT_0012109a;
          sVar13 = strlen(pcVar15);
          lVar10 = sVar13 + param_2;
          memcpy(pcVar15 + sVar13,param_1,param_2);
          pcVar9 = "\\)$";
        }
        else {
          *(undefined2 *)pcVar15 = DAT_00121097;
          pcVar15[2] = DAT_00121099;
          sVar13 = strlen(pcVar15);
          lVar10 = sVar13 + param_2;
          memcpy(pcVar15 + sVar13,param_1,param_2);
          pcVar9 = ")$";
        }
        pcVar9 = strcpy(pcVar15 + lVar10,pcVar9);
        local_68 = strlen(pcVar9);
        local_68 = local_68 + lVar10;
        local_70 = pcVar15;
      }
      FUN_00114cf0(local_70,local_68,puVar7[1]);
      pcVar9 = (char *)FUN_00115020(puVar7[1]);
      if (pcVar9 != (char *)0x0) {
        pcVar12 = pcVar9 + 3;
        uVar8 = FUN_0010e5c0(0);
        cVar6 = *pcVar9;
        *puVar7 = uVar8;
        if (cVar6 == '\0') {
          sVar13 = strlen(pcVar12);
          FUN_0010c9d0(uVar8,pcVar12,sVar13);
        }
        else {
          puVar7[7] = puVar7[7] + 1;
          sVar13 = strlen(pcVar12);
          lVar10 = (byte)pcVar9[1] + sVar13 + (ulong)(byte)pcVar9[2];
          __ptr = (char *)FUN_0011ef30(lVar10);
          cVar6 = DAT_0012b684;
          *__ptr = DAT_0012b684;
          bVar1 = pcVar9[1];
          *(byte *)(puVar7 + 8) = *(byte *)(puVar7 + 8) | bVar1;
          pvVar11 = memcpy(__ptr + bVar1,pcVar12,sVar13);
          if (pcVar9[2] != '\0') {
            *(char *)((long)pvVar11 + sVar13) = cVar6;
          }
          FUN_0010c9d0(*puVar7,__ptr,lVar10);
          free(__ptr);
        }
        FUN_0010cec0(*puVar7);
        free(pcVar9);
      }
      FUN_00115aa0(0,0,puVar7[1],1);
      if (local_80 != (char *)0x0) {
        if ((param_4 != '\0') || (cVar6 = FUN_00114e30(puVar7[1]), cVar6 == '\0')) {
          puVar7[2] = puVar7[2] + -0x40;
          puVar7[3] = puVar7[3] + 1;
          cVar6 = FUN_00106840(puVar7,local_80,local_88,0,0xffffffffffffffff,param_3,0);
          if (cVar6 == '\0') {
                    /* WARNING: Subroutine does not return */
            abort();
          }
        }
        if (local_70 != local_80) {
          free(local_80);
        }
      }
      free(pcVar15);
      if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return puVar7;
    }
  } while( true );
}




// Function: EGexecute @ 0x7110

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
  
  cVar2 = DAT_0012b684;
  pcVar16 = param_2 + param_3;
  lVar7 = param_1[1];
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  lVar6 = FUN_00114e10(lVar7);
  cVar4 = FUN_00114e20(lVar7);
  pcVar12 = param_2;
  local_68 = param_2;
joined_r0x0010718b:
  while( true ) {
    if (pcVar16 <= pcVar12) {
      lVar7 = -1;
      goto LAB_001076a5;
    }
    pcVar10 = pcVar12;
    local_e0 = pcVar16;
    local_b0 = param_5;
    if (param_5 == (char *)0x0) break;
LAB_001073d9:
    local_a0 = (long)local_e0 - (long)pcVar10;
    if (0x80000000 < local_a0) {
LAB_00107826:
                    /* WARNING: Subroutine does not return */
      FUN_0011f410();
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
          bVar20 = DAT_0012b684 == '\n';
          __buffer->not_eol = 0;
          __buffer->newline_anchor = bVar20;
          iVar5 = re_search(__buffer,pcVar10,(int)local_a0 + -1,(int)local_b0 - iVar18,
                            (int)lVar9 + -1,__regs);
          if (iVar5 < -1) goto LAB_00107826;
          pcVar11 = local_f8;
          lVar3 = local_d8;
          if (iVar5 != -1) break;
LAB_00107528:
          local_d8 = lVar3;
          local_f8 = pcVar11;
          lVar7 = lVar7 + 1;
          if (param_1[3] <= lVar7) goto LAB_001075d0;
        }
        pcVar19 = pcVar10 + iVar5;
        if (local_f8 < pcVar19) goto LAB_00107528;
        lVar15 = (long)(*(int *)param_1[6] - iVar5);
        if (param_5 != (char *)0x0) {
          if (DAT_0012b686 != '\0') {
            if (DAT_0012b685 == '\0') goto LAB_0010748d;
            bVar20 = lVar9 + -1 != lVar15;
            lVar15 = lVar9;
            pcVar19 = local_b0;
            if (bVar20) goto LAB_00107528;
          }
joined_r0x00107660:
          pcVar11 = pcVar19;
          lVar3 = lVar15;
          if (((local_f8 <= pcVar19) && (pcVar11 = local_f8, lVar3 = local_d8, local_f8 == pcVar19))
             && (lVar3 = lVar15, lVar15 <= local_d8)) {
            lVar3 = local_d8;
          }
          goto LAB_00107528;
        }
        if (DAT_0012b685 == '\0') {
          if (DAT_0012b686 != '\0') {
LAB_0010748d:
            do {
              lVar13 = FUN_0010e820(pcVar19 + lVar15,pcVar12);
              if ((lVar13 == 0) && (lVar13 = FUN_0010e830(pcVar10,pcVar19,pcVar12), lVar13 == 0)) {
                if (param_5 != (char *)0x0) goto joined_r0x00107660;
                goto LAB_00107690;
              }
              iVar5 = (int)pcVar19;
              if (lVar15 < 1) {
LAB_001074c2:
                if (pcVar12 == pcVar19) break;
                lVar15 = param_1[2];
                ((re_pattern_buffer *)(lVar17 + lVar15))->not_eol = 0;
                iVar5 = re_search((re_pattern_buffer *)(lVar17 + lVar15),pcVar10,(int)local_a0 + -1,
                                  (iVar5 + 1) - iVar18,((int)local_e0 - (iVar5 + 1)) + -1,__regs);
                if (iVar5 < 0) {
                  if (iVar5 != -1) goto LAB_00107826;
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
                if (iVar14 < -1) goto LAB_00107826;
                if (iVar14 < 1) goto LAB_001074c2;
              }
              lVar15 = (long)iVar14;
            } while (pcVar19 <= local_f8);
            goto LAB_00107528;
          }
          goto LAB_00107690;
        }
        if (lVar15 == lVar9 + -1) goto LAB_00107690;
        lVar7 = lVar7 + 1;
      } while (lVar7 < param_1[3]);
LAB_001075d0:
      pcVar12 = local_e0;
      if (local_f8 < local_e0) {
        local_a0 = local_d8;
        pcVar10 = local_f8;
LAB_00107690:
        lVar7 = (long)pcVar10 - (long)param_2;
        *param_4 = local_a0;
LAB_001076a5:
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
LAB_001072a0:
    pcVar12 = local_e0;
    pcVar10 = local_e0;
    if (lVar6 != 0) {
      pcVar10 = (char *)FUN_00114e00(lVar6,local_e0,pcVar11,0,&local_60,0);
      pcVar12 = pcVar11;
      if ((pcVar10 == (char *)0x0) || (pcVar11 == pcVar10)) goto joined_r0x0010718b;
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
LAB_00107335:
    local_e0 = pcVar11;
    pcVar11 = (char *)FUN_00114e00(param_1[1],pcVar12,local_e0,0,&local_60,&local_69);
    pcVar12 = local_e0;
    if ((pcVar11 == (char *)0x0) || (pcVar12 = local_e0, local_e0 == pcVar11))
    goto joined_r0x0010718b;
    if (local_60 != 0) {
      pvVar8 = memrchr(param_2,(int)cVar2,(long)pcVar11 - (long)param_2);
      pcVar10 = (char *)((long)pvVar8 + 1);
    }
    pvVar8 = rawmemchr(pcVar11,(int)cVar2);
    local_e0 = (char *)((long)pvVar8 + 1);
    local_b0 = pcVar10;
    if (local_69 != '\0') goto LAB_001073d9;
  }
  else {
    lVar7 = FUN_0010e3e0(*param_1,(long)pcVar12 - (ulong)*(byte *)(param_1 + 8),
                         (ulong)*(byte *)(param_1 + 8) + ((long)pcVar16 - (long)pcVar12),local_58,1)
    ;
    if (lVar7 < 0) goto LAB_001076a5;
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
      goto LAB_001072a0;
    }
    pvVar8 = rawmemchr(pcVar19,iVar18);
    pcVar11 = (char *)((long)pvVar8 + 1);
    if (lVar17 <= lVar9) goto LAB_001072a0;
    local_e0 = pcVar11;
    if ((DAT_0012b180 == '\x01') && (DAT_0012b182 == '\0')) {
      if (local_68 < pcVar10) {
        local_68 = pcVar10;
      }
      lVar7 = FUN_0010e640(&local_68,0,pcVar19,pcVar16);
      pcVar12 = local_68;
      if (lVar7 == 0) goto LAB_0010782b;
      goto LAB_00107335;
    }
  }
LAB_0010782b:
  local_a0 = (long)local_e0 - (long)pcVar10;
  goto LAB_00107690;
}




// Function: hash_pattern @ 0x7850

ulong hash_pattern(long param_1,ulong param_2)

{
  byte bVar1;
  ulong uVar2;
  byte *pbVar3;
  
  pbVar3 = (byte *)(DAT_0012b8d8 + -1 + param_1);
  uVar2 = 0x3938a0565d85cb03;
  bVar1 = *pbVar3;
  while (bVar1 != 10) {
    pbVar3 = pbVar3 + 1;
    uVar2 = uVar2 * 0x21 ^ (ulong)bVar1;
    bVar1 = *pbVar3;
  }
  return uVar2 % param_2;
}




// Function: compare_patterns @ 0x78a0

undefined8 compare_patterns(long param_1,long param_2)

{
  char *pcVar1;
  char *pcVar2;
  char *pcVar3;
  long lVar4;
  char cVar5;
  
  pcVar1 = (char *)(DAT_0012b8d8 + -1 + param_1);
  pcVar2 = (char *)(DAT_0012b8d8 + -1 + param_2);
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




// Function: color_cap_mt_fct @ 0x78f0

void color_cap_mt_fct(void)

{
  PTR_s_01_31_0012b060 = PTR_s_01_31_0012b068;
  return;
}




// Function: color_cap_rv_fct @ 0x7910

void color_cap_rv_fct(void)

{
  DAT_0012b900 = 0xffffffff;
  return;
}




// Function: color_cap_ne_fct @ 0x7920

void color_cap_ne_fct(void)

{
  PTR_DAT_0012b028 = &DAT_0012109e;
  PTR_DAT_0012b020 = &DAT_001210a4;
  return;
}




// Function: setmatcher @ 0x7950

ulong setmatcher(char *param_1,int param_2)

{
  int iVar1;
  undefined8 uVar2;
  ulong uVar3;
  
  uVar3 = 0;
  while( true ) {
    iVar1 = strcmp(param_1,&DAT_0012a7a0 + uVar3 * 0x20);
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




// Function: update_patterns @ 0x7a20

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
      iVar2 = FUN_0011a700(DAT_0012b8c8,param_2 + 1,0);
      if (iVar2 != 0) {
        if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_0011f410();
        }
        __dest = (void *)((long)__dest + ((long)pvVar3 - (long)__s));
        if (iVar5 == 0) {
          if (DAT_0012b8e0 == DAT_0012b8e8) {
            DAT_0012b8f0 = FUN_0011f1f0(DAT_0012b8f0,&DAT_0012b8e8,1,0xffffffffffffffff,0x18);
          }
          plVar1 = (long *)(DAT_0012b8f0 + DAT_0012b8e0 * 0x18);
          DAT_0012b8e0 = DAT_0012b8e0 + 1;
          plVar1[1] = param_4;
          *plVar1 = DAT_0012b8d0;
          plVar1[2] = lVar4;
        }
        DAT_0012b8d0 = DAT_0012b8d0 + 1;
        param_2 = (long)__dest - param_1;
      }
      lVar4 = lVar4 + 1;
      __s = pvVar3;
      iVar5 = iVar2;
    } while (pvVar3 < (void *)(param_3 + param_1));
  }
  return param_2;
}




// Function: printf_errno @ 0x7b80

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
    DAT_0012b8c0 = *piVar2;
  }
  if (local_c0 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: nlscan @ 0x7c90

void nlscan(void *param_1)

{
  long lVar1;
  void *pvVar2;
  undefined8 uVar3;
  long lVar4;
  long lVar5;
  long *plVar6;
  void *__s;
  uint uVar7;
  
  lVar1 = DAT_0012b818;
  lVar5 = DAT_0012b818;
  if (DAT_0012b7c0 < param_1) {
    uVar7 = (uint)DAT_0012b684;
    lVar4 = 0;
    __s = DAT_0012b7c0;
    do {
      plVar6 = (long *)(ulong)uVar7;
      pvVar2 = memchr(__s,uVar7,(long)param_1 - (long)__s);
      if (pvVar2 == (void *)0x0) break;
      __s = (void *)((long)pvVar2 + 1);
      lVar4 = lVar4 + 1;
    } while (__s < param_1);
    lVar5 = lVar4 + lVar1;
    if (SCARRY8(lVar4,lVar1)) {
      FUN_00107c60();
      uVar7 = FUN_0011f450();
      if ((uVar7 < 2) && (-1 < *plVar6)) {
        return;
      }
      uVar3 = dcgettext(0,"invalid context length argument",5);
                    /* WARNING: Subroutine does not return */
      error(2,0,"%s: %s",__s,uVar3);
    }
  }
  DAT_0012b818 = lVar5;
  DAT_0012b7c0 = param_1;
  return;
}




// Function: print_sep @ 0x7ec0

void print_sep(char param_1)

{
  if ((DAT_0012b900 != 0) && (*PTR_DAT_0012b040 != '\0')) {
    FUN_0011faa0(PTR_DAT_0012b028);
  }
  FUN_00107e70((int)param_1);
  if ((DAT_0012b900 != 0) && (*PTR_DAT_0012b040 != '\0')) {
    FUN_0011fac0(PTR_DAT_0012b020);
    return;
  }
  return;
}




// Function: print_offset @ 0x7f20

void print_offset(undefined8 param_1,char *param_2)

{
  if ((DAT_0012b900 != 0) && (*param_2 != '\0')) {
    FUN_0011faa0(PTR_DAT_0012b028,param_2,param_1);
  }
  FUN_00107b80(&DAT_001210de,DAT_0012b8f8,param_1);
  if ((DAT_0012b900 != 0) && (*param_2 != '\0')) {
    FUN_0011fac0(PTR_DAT_0012b020);
    return;
  }
  return;
}




// Function: print_filename @ 0x7fa0

void print_filename(void)

{
  int iVar1;
  int *piVar2;
  
  if ((DAT_0012b900 != 0) && (*PTR_DAT_0012b058 != '\0')) {
    FUN_0011faa0(PTR_DAT_0012b028);
  }
  if (DAT_0012b890 == (char *)0x0) {
    DAT_0012b890 = (char *)dcgettext(0,"(standard input)",5);
  }
  iVar1 = fputs_unlocked(DAT_0012b890,stdout);
  if (iVar1 < 0) {
    piVar2 = __errno_location();
    DAT_0012b8c0 = *piVar2;
  }
  if ((DAT_0012b900 != 0) && (*PTR_DAT_0012b058 != '\0')) {
    FUN_0011fac0(PTR_DAT_0012b020);
    return;
  }
  return;
}




// Function: print_line_head @ 0x8050

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulong print_line_head(ulong *param_1,ulong *param_2,ulong param_3,ulong param_4)

{
  ulong *puVar1;
  char cVar2;
  int iVar3;
  ulong uVar4;
  int *piVar5;
  undefined8 uVar6;
  byte bVar7;
  char extraout_DL;
  long lVar8;
  ulong *puVar9;
  ulong *puVar10;
  ulong *puVar11;
  ulong *puVar12;
  ulong *puVar13;
  ulong *unaff_R13;
  undefined8 *unaff_R14;
  ulong *unaff_R15;
  long in_FS_OFFSET;
  bool bVar14;
  char *pcStack_b0;
  char *pcStack_a8;
  long lStack_a0;
  long lStack_98;
  ulong *puStack_88;
  ulong uStack_80;
  ulong *puStack_78;
  ulong *puStack_70;
  undefined8 *puStack_68;
  ulong *puStack_60;
  ulong local_58;
  byte local_49;
  undefined8 local_48;
  long local_40;
  
  uVar4 = DAT_0012b868;
  param_4 = param_4 & 0xffffffff;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  puVar11 = param_2;
  local_58 = param_3;
  if (DAT_0012b810 != 1) {
    unaff_R15 = (ulong *)((long)param_1 + (long)param_2);
    local_49 = (byte)*unaff_R15;
    if (DAT_0012b868 != 0) {
      *(byte *)unaff_R15 = 0xff;
      unaff_R14 = &local_48;
      local_48 = 0;
      unaff_R13 = param_1;
LAB_001081e6:
      for (; ((ulong)unaff_R13 & 7) != 0; unaff_R13 = (ulong *)((long)unaff_R13 + 1)) {
        if (((byte)*unaff_R13 & uVar4) != 0) goto LAB_001081ff;
      }
      for (; (uVar4 & *unaff_R13) == 0; unaff_R13 = unaff_R13 + 1) {
      }
      while (((byte)*unaff_R13 & uVar4) == 0) {
        unaff_R13 = (ulong *)((long)unaff_R13 + 1);
      }
LAB_001081ff:
      if (unaff_R15 <= unaff_R13) {
        *(byte *)unaff_R15 = local_49;
        goto LAB_00108087;
      }
      puVar11 = (ulong *)((long)unaff_R15 - (long)unaff_R13);
      puStack_60 = (ulong *)0x108219;
      uVar4 = FUN_0011ac90(unaff_R13,puVar11,unaff_R14);
      if ((uVar4 < 0x11) || (-1 < (long)uVar4)) {
        unaff_R13 = (ulong *)((long)unaff_R13 + uVar4);
        uVar4 = DAT_0012b868;
        goto LAB_001081e6;
      }
      DAT_0012b888 = 1;
      *(byte *)unaff_R15 = local_49;
      uVar4 = 0;
      goto LAB_00108137;
    }
  }
LAB_00108087:
  cVar2 = (char)param_4;
  if (DAT_0012b808 != 0) {
    puStack_60 = (ulong *)0x10824b;
    FUN_00107fa0();
    if (DAT_0012b804 == 0) {
      puStack_60 = (ulong *)0x10826a;
      FUN_00107e70(0);
    }
    else {
      puStack_60 = (ulong *)0x10825e;
      FUN_00107ec0((int)cVar2);
    }
  }
  if (DAT_0012b801 == '\0') {
LAB_001080f2:
    if (DAT_0012b800 != '\0') {
      lVar8 = (long)param_1 - DAT_0012b848;
      param_1 = (ulong *)(lVar8 + _DAT_0012b7c8);
      puVar10 = param_1;
      if (SCARRY8(lVar8,_DAT_0012b7c8)) goto LAB_00108297;
      puStack_60 = (ulong *)0x10811e;
      FUN_00107f20(param_1,PTR_DAT_0012b048);
      puStack_60 = (ulong *)0x108127;
      FUN_00107ec0((int)cVar2);
    }
    uVar4 = (ulong)DAT_0012b8fc;
    if (DAT_0012b8fc == 0) {
      uVar4 = 1;
    }
    else {
      bVar14 = DAT_0012b801 != '\0' || DAT_0012b800 != '\0';
      if ((bVar14 || DAT_0012b808 != 0) && param_2 != (ulong *)0x0) {
        local_58 = CONCAT71(local_58._1_7_,(bVar14 || DAT_0012b808 != 0) && param_2 != (ulong *)0x0)
        ;
        puStack_60 = (ulong *)0x108190;
        FUN_00107e70(9);
        uVar4 = local_58 & 0xff;
      }
    }
LAB_00108137:
    if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
      puStack_60 = (ulong *)0x108297;
      __stack_chk_fail();
    }
    return uVar4;
  }
  if (local_58 <= DAT_0012b7c0) {
LAB_001080dd:
    puStack_60 = (ulong *)0x1080e9;
    puVar11 = (ulong *)PTR_DAT_0012b050;
    FUN_00107f20(DAT_0012b818);
    puStack_60 = (ulong *)0x1080f2;
    FUN_00107ec0((int)cVar2);
    goto LAB_001080f2;
  }
  puStack_60 = (ulong *)0x1080ba;
  FUN_00107c90(param_1);
  puVar10 = (ulong *)((long)DAT_0012b818 + 1);
  if (!SCARRY8((long)DAT_0012b818,1)) {
    DAT_0012b7c0 = local_58;
    DAT_0012b818 = (ulong *)((long)DAT_0012b818 + 1);
    goto LAB_001080dd;
  }
LAB_00108297:
  puStack_60 = (ulong *)0x10829c;
  FUN_00107c60();
  lStack_98 = *(long *)(in_FS_OFFSET + 0x28);
  puStack_88 = param_1;
  uStack_80 = param_4;
  puStack_78 = param_2;
  puStack_70 = unaff_R13;
  puStack_68 = unaff_R14;
  puStack_60 = unaff_R15;
  if ((DAT_0012b8fd == 0) && (cVar2 = print_line_head(), puVar12 = DAT_0012b7b8, cVar2 == '\0'))
  goto LAB_0010833a;
  bVar14 = extraout_DL == ':';
  bVar7 = (bVar14 ^ DAT_0012b802) & DAT_0012b8fd;
  if (DAT_0012b900 == 0) {
    if (bVar7 != 0) {
      pcStack_b0 = (char *)0x0;
      pcStack_a8 = (char *)0x0;
LAB_001084c2:
      puVar9 = puVar11;
      if (puVar11 <= puVar10) {
LAB_001083c0:
        puVar10 = puVar9;
        puVar12 = DAT_0012b7b8;
        if (puVar9 == (ulong *)0x0) goto LAB_0010833a;
        goto LAB_001083d0;
      }
LAB_001084cb:
      puVar9 = (ulong *)0x0;
      puVar12 = puVar10;
      do {
        lVar8 = (*DAT_0012b878)(DAT_0012b870,puVar10,(long)puVar11 - (long)puVar10,&lStack_a0,
                                puVar12);
        if ((lVar8 < 0) || (puVar1 = (ulong *)((long)puVar10 + lVar8), puVar11 == puVar1)) break;
        if (lStack_a0 == 0) {
          lStack_a0 = 1;
          if (puVar9 == (ulong *)0x0) {
            puVar9 = puVar12;
          }
        }
        else {
          if (DAT_0012b8fd == 0) {
            if (*pcStack_a8 != '\0') {
              FUN_0011faa0(PTR_DAT_0012b028,pcStack_a8);
            }
            puVar13 = puVar9;
            if (puVar9 == (ulong *)0x0) {
              puVar13 = puVar12;
            }
            puVar9 = (ulong *)0x0;
            FUN_00107e40(puVar13,(long)puVar1 - (long)puVar13);
          }
          else {
            cVar2 = print_line_head(puVar1,lStack_a0,puVar11,(-(DAT_0012b802 == 0) & 0xdU) + 0x2d);
            puVar12 = DAT_0012b7b8;
            if (cVar2 == '\0') goto LAB_0010833a;
          }
          if ((DAT_0012b900 != 0) && (*pcStack_b0 != '\0')) {
            FUN_0011faa0(PTR_DAT_0012b028,pcStack_b0);
          }
          FUN_00107e40(puVar1,lStack_a0);
          if ((DAT_0012b900 != 0) && (*pcStack_b0 != '\0')) {
            FUN_0011fac0(PTR_DAT_0012b020);
          }
          if (DAT_0012b8fd != 0) {
            FUN_00107e70((int)(char)DAT_0012b684);
          }
        }
        puVar12 = (ulong *)((long)puVar1 + lStack_a0);
      } while (puVar12 < puVar11);
      puVar10 = puVar9;
      puVar9 = puVar11;
      if ((DAT_0012b8fd != 0) || (puVar9 = puVar12, puVar10 == (ulong *)0x0)) goto LAB_001083c0;
      cVar2 = *pcStack_a8;
LAB_00108650:
      if (cVar2 != '\0') {
LAB_001083f7:
        puVar12 = puVar11;
        if (puVar10 < puVar11) {
          uVar4 = 0;
          if (*(byte *)((long)puVar11 + -1) == DAT_0012b684) {
            puVar12 = (ulong *)((long)puVar11 + -1);
            if (puVar12 <= puVar10) goto LAB_00108430;
            uVar4 = 1;
          }
          if (*(byte *)((long)puVar11 + ~uVar4) == 0xd) {
            puVar12 = (ulong *)((long)puVar11 - (uVar4 + 1));
          }
        }
LAB_00108430:
        if (0 < (long)puVar12 - (long)puVar10) {
          FUN_0011faa0(PTR_DAT_0012b028,pcStack_a8);
          FUN_00107e40(puVar10,(long)puVar12 - (long)puVar10);
          puVar10 = puVar12;
          if (*pcStack_a8 != '\0') {
            FUN_0011fac0(PTR_DAT_0012b020);
          }
        }
      }
    }
LAB_001082fd:
    if ((puVar10 < puVar11) && (DAT_0012b8fd != 1)) {
      FUN_00107e40(puVar10,(long)puVar11 - (long)puVar10);
    }
  }
  else {
    pcStack_a8 = PTR_s__00123687_0x1b_0012b038;
    if (bVar14 == (bool)((byte)((uint)DAT_0012b900 >> 0x1f) & DAT_0012b802)) {
      pcStack_a8 = PTR_s__00123687_0x1b_0012b030;
    }
    pcStack_b0 = PTR_s_01_31_0012b068;
    if (extraout_DL != ':') {
      pcStack_b0 = PTR_s_01_31_0012b060;
    }
    if (bVar7 != 0) goto LAB_001084c2;
    if (*pcStack_a8 != '\0') {
      if (bVar14 == (bool)DAT_0012b802) goto LAB_001083d0;
      if (*pcStack_b0 != '\0') {
LAB_001083b0:
        puVar9 = puVar10;
        if (puVar10 < puVar11) goto LAB_001084cb;
        goto LAB_001083c0;
      }
      goto LAB_001083f7;
    }
    if (*pcStack_b0 == '\0') goto LAB_001082fd;
    if (bVar14 != (bool)DAT_0012b802) goto LAB_001083b0;
LAB_001083d0:
    if (DAT_0012b8fd == 0) {
      cVar2 = *pcStack_a8;
      goto LAB_00108650;
    }
  }
  if ((DAT_0012b7d8 != '\0') && (iVar3 = fflush_unlocked(stdout), iVar3 != 0)) {
    piVar5 = __errno_location();
    DAT_0012b8c0 = *piVar5;
  }
  iVar3 = DAT_0012b8c0;
  puVar12 = puVar11;
  if (DAT_0012b8c0 != 0) {
    uVar6 = dcgettext(0,"write error",5);
                    /* WARNING: Subroutine does not return */
    error(2,iVar3,uVar6);
  }
LAB_0010833a:
  DAT_0012b7b8 = puVar12;
  if (lStack_98 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return 0;
}




// Function: prline @ 0x82a0

void prline(ulong param_1,ulong param_2,char param_3)

{
  ulong uVar1;
  char cVar2;
  int iVar3;
  long lVar4;
  int *piVar5;
  ulong uVar6;
  undefined8 uVar7;
  byte bVar8;
  ulong uVar9;
  ulong uVar10;
  long in_FS_OFFSET;
  bool bVar11;
  char *local_58;
  char *local_50;
  long local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if ((DAT_0012b8fd == 0) &&
     (cVar2 = FUN_00108050(param_1,(param_2 - param_1) + -1,param_2,(int)param_3),
     uVar9 = DAT_0012b7b8, cVar2 == '\0')) goto LAB_0010833a;
  bVar11 = param_3 == ':';
  bVar8 = (bVar11 ^ DAT_0012b802) & DAT_0012b8fd;
  if (DAT_0012b900 == 0) {
    if (bVar8 != 0) {
      local_58 = (char *)0x0;
      local_50 = (char *)0x0;
LAB_001084c2:
      uVar6 = param_2;
      if (param_2 <= param_1) {
LAB_001083c0:
        param_1 = uVar6;
        uVar9 = DAT_0012b7b8;
        if (uVar6 == 0) goto LAB_0010833a;
        goto LAB_001083d0;
      }
LAB_001084cb:
      uVar6 = 0;
      uVar9 = param_1;
      do {
        lVar4 = (*DAT_0012b878)(DAT_0012b870,param_1,param_2 - param_1,&local_48,uVar9);
        if ((lVar4 < 0) || (uVar1 = param_1 + lVar4, param_2 == uVar1)) break;
        if (local_48 == 0) {
          local_48 = 1;
          if (uVar6 == 0) {
            uVar6 = uVar9;
          }
        }
        else {
          if (DAT_0012b8fd == 0) {
            if (*local_50 != '\0') {
              FUN_0011faa0(PTR_DAT_0012b028,local_50);
            }
            uVar10 = uVar6;
            if (uVar6 == 0) {
              uVar10 = uVar9;
            }
            uVar6 = 0;
            FUN_00107e40(uVar10,uVar1 - uVar10);
          }
          else {
            cVar2 = FUN_00108050(uVar1,local_48,param_2,(-(DAT_0012b802 == 0) & 0xdU) + 0x2d);
            uVar9 = DAT_0012b7b8;
            if (cVar2 == '\0') goto LAB_0010833a;
          }
          if ((DAT_0012b900 != 0) && (*local_58 != '\0')) {
            FUN_0011faa0(PTR_DAT_0012b028,local_58);
          }
          FUN_00107e40(uVar1,local_48);
          if ((DAT_0012b900 != 0) && (*local_58 != '\0')) {
            FUN_0011fac0(PTR_DAT_0012b020);
          }
          if (DAT_0012b8fd != 0) {
            FUN_00107e70((int)DAT_0012b684);
          }
        }
        uVar9 = uVar1 + local_48;
      } while (uVar9 < param_2);
      param_1 = uVar6;
      uVar6 = param_2;
      if ((DAT_0012b8fd != 0) || (uVar6 = uVar9, param_1 == 0)) goto LAB_001083c0;
      cVar2 = *local_50;
LAB_00108650:
      if (cVar2 != '\0') {
LAB_001083f7:
        uVar9 = param_2;
        if (param_1 < param_2) {
          uVar6 = 0;
          if (*(char *)(param_2 - 1) == DAT_0012b684) {
            uVar9 = param_2 - 1;
            if (uVar9 <= param_1) goto LAB_00108430;
            uVar6 = 1;
          }
          if (*(char *)(param_2 + ~uVar6) == '\r') {
            uVar9 = param_2 - (uVar6 + 1);
          }
        }
LAB_00108430:
        if (0 < (long)(uVar9 - param_1)) {
          FUN_0011faa0(PTR_DAT_0012b028,local_50);
          FUN_00107e40(param_1,uVar9 - param_1);
          param_1 = uVar9;
          if (*local_50 != '\0') {
            FUN_0011fac0(PTR_DAT_0012b020);
          }
        }
      }
    }
LAB_001082fd:
    if ((param_1 < param_2) && (DAT_0012b8fd != 1)) {
      FUN_00107e40(param_1,param_2 - param_1);
    }
  }
  else {
    local_50 = PTR_s__00123687_0x1b_0012b038;
    if (bVar11 == (bool)((byte)((uint)DAT_0012b900 >> 0x1f) & DAT_0012b802)) {
      local_50 = PTR_s__00123687_0x1b_0012b030;
    }
    local_58 = PTR_s_01_31_0012b068;
    if (param_3 != ':') {
      local_58 = PTR_s_01_31_0012b060;
    }
    if (bVar8 != 0) goto LAB_001084c2;
    if (*local_50 != '\0') {
      if (bVar11 == (bool)DAT_0012b802) goto LAB_001083d0;
      if (*local_58 != '\0') {
LAB_001083b0:
        uVar6 = param_1;
        if (param_1 < param_2) goto LAB_001084cb;
        goto LAB_001083c0;
      }
      goto LAB_001083f7;
    }
    if (*local_58 == '\0') goto LAB_001082fd;
    if (bVar11 != (bool)DAT_0012b802) goto LAB_001083b0;
LAB_001083d0:
    if (DAT_0012b8fd == 0) {
      cVar2 = *local_50;
      goto LAB_00108650;
    }
  }
  if ((DAT_0012b7d8 != '\0') && (iVar3 = fflush_unlocked(stdout), iVar3 != 0)) {
    piVar5 = __errno_location();
    DAT_0012b8c0 = *piVar5;
  }
  iVar3 = DAT_0012b8c0;
  uVar9 = param_2;
  if (DAT_0012b8c0 != 0) {
    uVar7 = dcgettext(0,"write error",5);
                    /* WARNING: Subroutine does not return */
    error(2,iVar3,uVar7);
  }
LAB_0010833a:
  DAT_0012b7b8 = uVar9;
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: prpending @ 0x8710

void prpending(void *param_1)

{
  void *__s;
  void *pvVar1;
  
  __s = DAT_0012b7b8;
  if (DAT_0012b7b8 == (void *)0x0) {
    __s = DAT_0012b848;
  }
  while ((0 < DAT_0012b7a8 && (__s < param_1))) {
    DAT_0012b7b8 = __s;
    pvVar1 = rawmemchr(__s,(int)DAT_0012b684);
    FUN_001082a0(__s,(long)pvVar1 + 1,0x2d);
    DAT_0012b7a8 = DAT_0012b7a8 + -1;
    __s = DAT_0012b7b8;
  }
  DAT_0012b7b8 = __s;
  return;
}




// Function: grepbuf @ 0x8790

long grepbuf(void *param_1,void *param_2)

{
  char *pcVar1;
  char cVar2;
  int iVar3;
  long lVar4;
  void *pvVar5;
  ulong uVar6;
  void *pvVar7;
  long lVar8;
  void *pvVar9;
  int *piVar10;
  long lVar11;
  char cVar12;
  void *pvVar13;
  long in_FS_OFFSET;
  void *local_70;
  long local_48;
  long local_40;
  
  lVar8 = DAT_0012b7b0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 < param_2) {
    do {
      lVar4 = (*DAT_0012b878)(DAT_0012b870,param_1,(long)param_2 - (long)param_1,&local_48,0);
      cVar2 = DAT_0012b684;
      if (lVar4 < 0) {
        if (DAT_0012b802 == '\0') {
LAB_00108bf3:
          lVar8 = lVar8 - DAT_0012b7b0;
          goto LAB_00108bfb;
        }
        local_48 = 0;
        pvVar5 = param_2;
        local_70 = param_2;
        if (param_1 < param_2) {
LAB_001088dc:
          pvVar7 = param_1;
          if (DAT_0012b803 == '\0') {
LAB_00108847:
            param_1 = pvVar7;
            if ((0 < (long)DAT_0012b7a8) && (FUN_00108710(), DAT_0012b803 != '\0'))
            goto LAB_001088f3;
            pvVar9 = DAT_0012b7b8;
            if (DAT_0012b7b8 == (void *)0x0) {
              pvVar9 = DAT_0012b848;
            }
            uVar6 = 0;
            pvVar13 = pvVar7;
            if ((long)DAT_0012b7f8 < 1) {
              if (((-1 < (long)(DAT_0012b7f8 & DAT_0012b7f0)) && (DAT_0012b7b8 != pvVar7)) &&
                 ((DAT_0012b688 != '\0' && (PTR_DAT_0012b070 != (undefined *)0x0)))) {
LAB_00108b46:
                if ((DAT_0012b900 != 0) && (*PTR_DAT_0012b040 != '\0')) {
                  FUN_0011faa0(PTR_DAT_0012b028);
                }
                iVar3 = fputs_unlocked(PTR_DAT_0012b070,stdout);
                if (iVar3 < 0) {
                  piVar10 = __errno_location();
                  DAT_0012b8c0 = *piVar10;
                }
                if ((DAT_0012b900 == 0) || (*PTR_DAT_0012b040 == '\0')) {
                  FUN_00107e70();
                }
                else {
                  FUN_0011fac0(PTR_DAT_0012b020);
                  FUN_00107e70();
                }
                goto LAB_00108a12;
              }
            }
            else {
              do {
                param_1 = pvVar13;
                if (pvVar9 < pvVar13) {
                  do {
                    param_1 = (void *)((long)pvVar13 - 1);
                    if (cVar2 == *(char *)((long)pvVar13 - 2)) break;
                    param_1 = (void *)((long)pvVar13 - 2);
                    pcVar1 = (char *)((long)pvVar13 - 3);
                    pvVar13 = param_1;
                  } while (cVar2 != *pcVar1);
                }
                uVar6 = uVar6 + 1;
                pvVar13 = param_1;
              } while (uVar6 != DAT_0012b7f8);
              if (((DAT_0012b688 != '\0') && (DAT_0012b7b8 != param_1)) &&
                 (PTR_DAT_0012b070 != (undefined *)0x0)) goto LAB_00108b46;
LAB_00108a12:
              if (param_1 < pvVar7) {
                do {
                  pvVar9 = rawmemchr(param_1,(int)cVar2);
                  pvVar9 = (void *)((long)pvVar9 + 1);
                  FUN_001082a0(param_1,pvVar9,0x2d);
                  param_1 = pvVar9;
                } while (pvVar9 < pvVar7);
              }
            }
            cVar12 = DAT_0012b803;
            if (DAT_0012b802 != '\0') goto LAB_00108911;
            if (DAT_0012b803 != '\0') goto LAB_00108a78;
            FUN_001082a0(pvVar7,local_70,0x3a);
            DAT_0012b7b0 = DAT_0012b7b0 + -1;
            param_1 = local_70;
            cVar12 = DAT_0012b803;
          }
          else {
LAB_001088f3:
            cVar12 = DAT_0012b802;
            if (DAT_0012b802 == '\0') goto LAB_00108a78;
LAB_00108911:
            lVar11 = 0;
            lVar4 = DAT_0012b7b0;
            if (param_1 < local_70) {
              for (; lVar11 < lVar4; lVar11 = lVar11 + 1) {
                pvVar7 = rawmemchr(param_1,(int)cVar2);
                pvVar7 = (void *)((long)pvVar7 + 1);
                if (cVar12 == '\0') {
                  FUN_001082a0(param_1,pvVar7,0x3a);
                  lVar4 = DAT_0012b7b0;
                  cVar12 = DAT_0012b803;
                }
                param_1 = pvVar7;
                if (local_70 <= pvVar7) {
                  DAT_0012b7b0 = lVar4 - (lVar11 + 1);
                  goto LAB_00108984;
                }
              }
              DAT_0012b7b0 = lVar4 - lVar11;
            }
          }
LAB_00108984:
          DAT_0012b828 = (long)param_1 + (DAT_0012b830 - DAT_0012b840);
          DAT_0012b7a8 = 0;
          if ((cVar12 == '\0') && (DAT_0012b7a8 = DAT_0012b7f0, (long)DAT_0012b7f0 < 0)) {
            DAT_0012b7a8 = 0;
          }
          goto LAB_001089b3;
        }
        break;
      }
      pvVar7 = (void *)((long)param_1 + lVar4);
      pvVar5 = (void *)(local_48 + (long)pvVar7);
      if (DAT_0012b802 == '\0') {
        if (param_2 == pvVar7) goto LAB_00108bf3;
        local_70 = pvVar5;
        if (DAT_0012b803 == '\0') goto LAB_00108847;
LAB_00108a78:
        DAT_0012b7b0 = DAT_0012b7b0 + -1;
        DAT_0012b7a8 = 0;
        DAT_0012b828 = (long)local_70 + (DAT_0012b830 - DAT_0012b840);
LAB_001089b3:
        DAT_0012b688 = '\x01';
        if ((DAT_0012b7b0 == 0) || (DAT_0012b7a2 != '\0')) {
          if (DAT_0012b7a1 != '\0') {
            iVar3 = 0;
            if (DAT_0012b889 != '\0') {
              iVar3 = DAT_0012b080;
            }
                    /* WARNING: Subroutine does not return */
            exit(iVar3);
          }
          goto LAB_00108bf3;
        }
      }
      else {
        local_70 = pvVar7;
        if (param_1 < pvVar7) goto LAB_001088dc;
      }
      param_1 = pvVar5;
    } while (pvVar5 < param_2);
    lVar8 = lVar8 - DAT_0012b7b0;
  }
  else {
    lVar8 = 0;
  }
LAB_00108bfb:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar8;
}




// Function: suppressible_error @ 0x8cb0

void suppressible_error(undefined4 param_1)

{
  if (DAT_0012b904 != '\0') {
    DAT_0012b889 = 1;
    return;
  }
  if (DAT_0012b890 == 0) {
    DAT_0012b890 = dcgettext(0,"(standard input)",5);
  }
                    /* WARNING: Subroutine does not return */
  error(0,param_1,&DAT_001229f1,DAT_0012b890);
}




// Function: fillbuf @ 0x8d20

byte fillbuf(ulong param_1,long param_2)

{
  size_t __n;
  byte bVar1;
  void *__dest;
  long lVar2;
  int *piVar3;
  undefined8 uVar4;
  long lVar5;
  char *pcVar6;
  long lVar7;
  char *pcVar8;
  long lVar9;
  char *pcVar10;
  ulong uVar11;
  long lVar12;
  int __fd;
  ulong uVar13;
  
  __dest = DAT_0012b860;
  uVar11 = DAT_0012b838;
  lVar2 = DAT_0012b838 + 8;
  pcVar10 = DAT_0012b840;
  if ((long)DAT_0012b860 + (DAT_0012b858 - (long)DAT_0012b840) < lVar2) {
    __n = param_1 + 1;
    uVar13 = ~param_1;
    lVar5 = ((DAT_0012b838 + param_1) - DAT_0012b858) + lVar2;
    if (lVar5 < 1) {
      pcVar10 = (char *)((long)DAT_0012b860 + __n);
      if ((ulong)pcVar10 % DAT_0012b838 == 0) {
        memmove(DAT_0012b860,DAT_0012b840 + uVar13,__n);
      }
      else {
        pcVar10 = (char *)(((DAT_0012b838 + 1 + param_1) - (ulong)pcVar10 % DAT_0012b838) +
                          (long)DAT_0012b860);
        memmove(pcVar10 + uVar13,DAT_0012b840 + uVar13,__n);
      }
    }
    else {
      lVar9 = -1;
      if (((((*(uint *)(param_2 + 0x18) & 0xf000) == 0x8000) &&
           (lVar12 = *(long *)(param_2 + 0x30) - DAT_0012b830, -1 < lVar12)) &&
          (lVar7 = lVar12 + lVar2 + param_1, !SCARRY8(lVar12,lVar2 + param_1))) &&
         (lVar9 = DAT_0012b858 + lVar5, DAT_0012b858 + lVar5 < lVar7)) {
        lVar9 = lVar7;
      }
      __dest = (void *)FUN_0011f1f0(0,&DAT_0012b858,lVar5,lVar9,1);
      uVar11 = DAT_0012b838;
      pcVar10 = (char *)((long)__dest + __n);
      if ((ulong)pcVar10 % DAT_0012b838 == 0) {
        memmove(__dest,DAT_0012b840 + uVar13,__n);
      }
      else {
        pcVar10 = (char *)(((DAT_0012b838 + 1 + param_1) - (ulong)pcVar10 % DAT_0012b838) +
                          (long)__dest);
        memmove(pcVar10 + uVar13,DAT_0012b840 + uVar13,__n);
      }
      free(DAT_0012b860);
      DAT_0012b860 = __dest;
    }
  }
  lVar9 = (long)__dest + ((DAT_0012b858 + -8) - (long)pcVar10);
  DAT_0012b848 = (long)pcVar10 - param_1;
  __fd = DAT_0012b850;
  lVar2 = DAT_0012b818;
  lVar5 = DAT_0012b830;
  while (DAT_0012b830 = lVar5, DAT_0012b818 = lVar2,
        lVar2 = FUN_0011da00(__fd,pcVar10,lVar9 - lVar9 % (long)uVar11), __fd = DAT_0012b850,
        lVar2 != -1) {
    pcVar8 = pcVar10 + lVar2;
    lVar12 = DAT_0012b830 + lVar2;
    bVar1 = DAT_0012b821 ^ 1 | lVar2 == 0;
    DAT_0012b830 = lVar12;
    if (bVar1 != 0) goto LAB_00108ea3;
    pcVar6 = pcVar10;
    if (pcVar10 < pcVar8) {
      do {
        if (*pcVar6 != '\0') {
          bVar1 = 1;
          goto LAB_00108ea3;
        }
        pcVar6 = pcVar6 + 1;
      } while (pcVar8 != pcVar6);
    }
    lVar7 = lVar2 + DAT_0012b818;
    if (SCARRY8(lVar2,DAT_0012b818)) {
      uVar4 = dcgettext(0,"input is too large to count",5);
                    /* WARNING: Subroutine does not return */
      error(2,0,uVar4);
    }
    lVar2 = lVar7;
    lVar5 = lVar12;
    if (DAT_0012b880 == '\0') {
      DAT_0012b818 = lVar7;
      lVar5 = lseek(DAT_0012b850,lVar12,3);
      if ((lVar5 < 0) &&
         (((piVar3 = __errno_location(), *piVar3 != 6 ||
           ((*(uint *)(param_2 + 0x18) & 0xf000) != 0x8000)) ||
          ((*(long *)(param_2 + 0x30) <= lVar12 || (lVar5 = lseek(__fd,0,2), lVar5 < 0)))))) {
        DAT_0012b880 = '\x01';
        lVar2 = DAT_0012b818;
        lVar5 = DAT_0012b830;
      }
      else {
        lVar2 = (lVar5 - lVar12) + lVar7;
        if (SCARRY8(lVar5 - lVar12,lVar7)) {
          uVar4 = dcgettext(0,"input is too large to count",5);
                    /* WARNING: Subroutine does not return */
          error(2,0,uVar4);
        }
      }
    }
  }
  bVar1 = 0;
  pcVar8 = pcVar10;
LAB_00108ea3:
  pcVar8[0] = '\0';
  pcVar8[1] = '\0';
  pcVar8[2] = '\0';
  pcVar8[3] = '\0';
  pcVar8[4] = '\0';
  pcVar8[5] = '\0';
  pcVar8[6] = '\0';
  pcVar8[7] = '\0';
  DAT_0012b840 = pcVar8;
  return bVar1;
}




// Function: pattern_file_name @ 0x9080

long pattern_file_name(long param_1,long *param_2)

{
  long lVar1;
  long *plVar2;
  long lVar3;
  
  plVar2 = DAT_0012b8f0;
  if (1 < DAT_0012b8e0) {
    lVar1 = 0x18;
    lVar3 = 1;
    while (*(long *)((long)DAT_0012b8f0 + lVar1) <= param_1) {
      lVar3 = lVar3 + 1;
      lVar1 = lVar1 + 0x18;
      if (lVar3 == DAT_0012b8e0) {
        plVar2 = DAT_0012b8f0 + lVar3 * 3 + -3;
        *param_2 = (param_1 - *plVar2) + plVar2[2];
        return plVar2[1];
      }
    }
    plVar2 = (long *)((long)DAT_0012b8f0 + lVar1 + -0x18);
  }
  *param_2 = (param_1 - *plVar2) + plVar2[2];
  return plVar2[1];
}




// Function: grepdesc @ 0x9130

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulong grepdesc(int param_1,byte param_2)

{
  char *pcVar1;
  char *pcVar2;
  char cVar3;
  char cVar4;
  char cVar5;
  char cVar6;
  short sVar7;
  int iVar8;
  uint uVar9;
  size_t sVar10;
  undefined8 *puVar11;
  void *pvVar12;
  long lVar13;
  int *piVar14;
  __off_t _Var15;
  long lVar16;
  __off_t _Var17;
  undefined8 uVar18;
  undefined8 uVar19;
  ssize_t sVar20;
  char cVar21;
  uint uVar22;
  size_t __offset;
  char *pcVar23;
  long lVar24;
  char *pcVar25;
  byte bVar26;
  ulong uVar27;
  char cVar28;
  char *pcVar29;
  long in_FS_OFFSET;
  bool bVar30;
  long local_1b8;
  long local_1b0;
  long local_1a8;
  char local_17b;
  long local_178 [2];
  stat local_168;
  stat local_d8;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  iVar8 = fstat(param_1,&local_168);
  if (iVar8 == 0) {
    if (param_1 != 0) {
      if (DAT_0012b884 == 2) {
        if (((local_168.st_mode & 0xb000) != 0x2000 && (local_168.st_mode & 0xf000) != 0xc000) &&
           ((local_168.st_mode & 0xf000) != 0x1000)) {
LAB_001094a4:
          if (param_2 != 0) {
            if ((local_168.st_mode & 0xf000) == 0x4000) {
              if (DAT_0012b018 == 4) goto LAB_0010944e;
              puVar11 = &DAT_0012b8a0;
              if (DAT_0012b88a != '\0') goto LAB_001094e0;
            }
            else {
              puVar11 = &DAT_0012b8b0;
            }
            if ((puVar11[1] != 0) && (cVar6 = FUN_00116b90(puVar11[1],DAT_0012b890), cVar6 != '\0'))
            goto LAB_0010944e;
          }
          goto LAB_001094e0;
        }
      }
      else {
        if ((DAT_0012b884 != 0) || (param_2 == 1)) goto LAB_001094a4;
        if (((local_168.st_mode & 0xb000) == 0x2000 || (local_168.st_mode & 0xf000) == 0xc000) ||
           ((local_168.st_mode & 0xf000) == 0x1000)) goto LAB_0010944e;
LAB_001094e0:
        if ((int)DAT_0012b808 < 0) {
          DAT_0012b808 = (uint)((local_168.st_mode & 0xf000) == 0x4000);
          if (DAT_0012b018 == 3) goto LAB_00109a0e;
LAB_001094fd:
          if ((DAT_0012b018 == 4) && ((local_168.st_mode & 0xf000) == 0x4000)) goto LAB_0010944e;
        }
        else {
          if (DAT_0012b018 != 3) goto LAB_001094fd;
LAB_00109a0e:
          if ((local_168.st_mode & 0xf000) == 0x4000) {
            uVar22 = ~(uint)(param_2 ^ 1) & _DAT_0012b014;
            iVar8 = close(param_1);
            if (iVar8 != 0) {
              piVar14 = __errno_location();
              FUN_00108cb0(*piVar14);
            }
            local_178[1] = 0;
            local_178[0] = DAT_0012b890;
            lVar13 = FUN_001189c0(local_178,uVar22,0);
            if (lVar13 == 0) {
                    /* WARNING: Subroutine does not return */
              FUN_0011f410();
            }
            uVar27 = 1;
switchD_00109b0d_caseD_c:
            do {
              lVar16 = FUN_00118fd0(lVar13);
              if (lVar16 == 0) {
                piVar14 = __errno_location();
                if (*piVar14 != 0) {
                  FUN_00108cb0();
                }
                iVar8 = FUN_00118de0(lVar13);
                if (iVar8 != 0) {
                  FUN_00108cb0(*piVar14);
                }
                goto LAB_001098de;
              }
              sVar7 = *(short *)(lVar16 + 0x68);
              bVar26 = *(long *)(lVar16 + 0x58) == 0 & param_2;
            } while (sVar7 == 6);
            if (bVar26 == 0) {
              if ((((ushort)(sVar7 - 1U) < 2) || (lVar24 = DAT_0012b8b0, sVar7 == 4)) &&
                 (lVar24 = DAT_0012b8a0, DAT_0012b018 == 4)) {
LAB_00109f64:
                FUN_00119720(lVar13,lVar16,4);
                goto switchD_00109b0d_caseD_c;
              }
              if (lVar24 != 0) {
                cVar6 = FUN_00116b90(lVar24,lVar16 + 0x100);
                if (cVar6 != '\0') goto LAB_00109f64;
                sVar7 = *(short *)(lVar16 + 0x68);
              }
            }
            DAT_0012b890 = *(long *)(lVar16 + 0x38);
            if ((DAT_0012b88a != '\0') && (*(char *)(DAT_0012b890 + 1) != '\0')) {
              DAT_0012b890 = DAT_0012b890 + 2;
            }
            lVar24 = DAT_0012b890;
            uVar22 = 1;
            if ((*(uint *)(lVar13 + 0x48) & 2) == 0) {
              uVar22 = *(uint *)(lVar13 + 0x48) & (uint)bVar26;
            }
            switch(sVar7) {
            default:
              goto switchD_00109b0d_caseD_0;
            case 1:
              if (DAT_0012b018 == 3) goto switchD_00109b0d_caseD_c;
              FUN_00119720(lVar13,lVar16,4);
              break;
            case 2:
              if (DAT_0012b904 == '\0') {
                uVar18 = dcgettext(0,"%s: warning: recursive directory loop",5);
                    /* WARNING: Subroutine does not return */
                error(0,0,uVar18,lVar24);
              }
              goto switchD_00109b0d_caseD_c;
            case 3:
            case 0xb:
              if ((DAT_0012b884 == 2) || ((bVar26 == 0 && (DAT_0012b884 == 0)))) {
                uVar9 = *(uint *)(lVar16 + 0x88);
                if ((*(uint *)(lVar16 + 0x88) == 0) &&
                   (iVar8 = fstatat(*(int *)(lVar13 + 0x2c),*(char **)(lVar16 + 0x30),&local_d8,
                                    uVar22 << 8 ^ 0x100), uVar9 = local_d8.st_mode, iVar8 != 0)) {
                  piVar14 = __errno_location();
                  FUN_00108cb0(*piVar14);
                  goto switchD_00109b0d_caseD_c;
                }
                if (((uVar9 & 0xb000) == 0x2000 || (uVar9 & 0xf000) == 0xc000) ||
                   ((uVar9 & 0xf000) == 0x1000)) goto switchD_00109b0d_caseD_c;
              }
              break;
            case 4:
            case 7:
            case 10:
              FUN_00108cb0(*(undefined4 *)(lVar16 + 0x40));
              goto switchD_00109b0d_caseD_c;
            case 8:
            case 0xd:
              break;
            case 0xc:
            case 0xe:
              goto switchD_00109b0d_caseD_c;
            }
            uVar22 = FUN_0010a0f0(*(undefined4 *)(lVar13 + 0x2c),*(undefined8 *)(lVar16 + 0x30),
                                  uVar22,bVar26);
            uVar27 = (ulong)((uint)uVar27 & uVar22);
            goto switchD_00109b0d_caseD_c;
          }
        }
        if (((DAT_0012b884 != 2) && ((DAT_0012b884 != 0 || (param_2 != 0)))) ||
           (((local_168.st_mode & 0xb000) != 0x2000 && (local_168.st_mode & 0xf000) != 0xc000 &&
            ((local_168.st_mode & 0xf000) != 0x1000)))) goto LAB_00109191;
      }
LAB_0010944e:
      uVar27 = 1;
      goto LAB_00109454;
    }
    if ((int)DAT_0012b808 < 0) {
      DAT_0012b808 = (uint)((local_168.st_mode & 0xf000) == 0x4000);
    }
LAB_00109191:
    cVar4 = DAT_0012b803;
    cVar3 = DAT_0012b7a2;
    cVar6 = DAT_0012b684;
    if (((((DAT_0012b803 == '\0') && (DAT_0012b80c == 0)) && (1 < DAT_0012b7e0)) &&
        (((local_168.st_mode & 0xf000) == 0x8000 && (local_168.st_ino == DAT_0012b928)))) &&
       (local_168.st_dev == _DAT_0012b920)) {
      if (DAT_0012b904 == '\0') {
        uVar18 = FUN_001090f0();
        uVar19 = dcgettext(0,"%s: input file is also the output",5);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar19,uVar18);
      }
      DAT_0012b889 = 1;
      uVar27 = 1;
      goto LAB_001098d2;
    }
    DAT_0012b840 = (char *)(DAT_0012b860 + 1);
    if ((ulong)DAT_0012b840 % DAT_0012b838 != 0) {
      DAT_0012b840 = (char *)(DAT_0012b860 +
                             ((DAT_0012b838 + 1) - (ulong)DAT_0012b840 % DAT_0012b838));
    }
    DAT_0012b848 = DAT_0012b840;
    DAT_0012b840[-1] = DAT_0012b684;
    DAT_0012b850 = param_1;
    if (param_1 == 0) {
      DAT_0012b830 = lseek(0,0,1);
      DAT_0012b880 = (byte)((ulong)DAT_0012b830 >> 0x3f);
      DAT_0012b881 = DAT_0012b880;
      if (-1 < DAT_0012b830) goto LAB_00109228;
      piVar14 = __errno_location();
      if (*piVar14 == 0x1d) {
        DAT_0012b830 = 0;
        goto LAB_00109228;
      }
      FUN_00108cb0();
      uVar27 = 1;
      local_17b = '\0';
      local_1a8 = 0;
    }
    else {
      DAT_0012b830 = 0;
      DAT_0012b881 = 0;
      DAT_0012b880 = 0;
LAB_00109228:
      _DAT_0012b7c8 = (char *)0x0;
      DAT_0012b7b8 = (char *)0x0;
      DAT_0012b7b0 = DAT_0012b7e0;
      DAT_0012b821 = cVar6 == '\0' & DAT_0012b820;
      DAT_0012b818 = 0;
      DAT_0012b828 = 0;
      DAT_0012b7a8 = 0;
      DAT_0012b888 = '\0';
      local_17b = FUN_00108d20(0,&local_168);
      if (local_17b == '\0') {
        __errno_location();
        uVar27 = 1;
        FUN_00108cb0();
        local_1a8 = 0;
      }
      else {
        DAT_0012b8f8 = 0;
        if (DAT_0012b8fc != '\0') {
          lVar13 = 0x7fffffffffffffff;
          if (((local_168.st_mode & 0xf000) == 0x8000) &&
             (lVar13 = local_168.st_size, DAT_0012b801 != '\0')) {
            lVar13 = local_168.st_size + (ulong)(local_168.st_size != 0x7fffffffffffffff);
          }
          DAT_0012b8f8 = 0;
          do {
            DAT_0012b8f8 = DAT_0012b8f8 + 1;
            lVar13 = lVar13 / 10;
          } while (lVar13 != 0);
        }
        local_1b0 = -1;
        local_1b8 = 0;
        local_1a8 = 0;
        lVar13 = 0;
        cVar21 = '\0';
        cVar28 = local_17b;
        do {
          pcVar29 = DAT_0012b848;
          pcVar25 = DAT_0012b840;
          iVar8 = DAT_0012b810;
          bVar26 = (byte)((ulong)local_1b0 >> 0x3f) & cVar6 != '\0';
          uVar27 = (ulong)bVar26;
          if ((bVar26 != 0) && (DAT_0012b810 != 1)) {
            *DAT_0012b840 = '\0';
            __offset = (long)pcVar25 - (long)pcVar29;
            sVar10 = strlen(pcVar29);
            if (__offset == sVar10) {
              if (((((cVar28 != '\0') && (DAT_0012b881 == 0)) &&
                   ((local_168.st_mode & 0xf000) == 0x8000)) &&
                  (((long)__offset < local_168.st_size &&
                   ((param_1 != 0 || (__offset = lseek(0,0,1), -1 < (long)__offset)))))) &&
                 (_Var17 = lseek(param_1,__offset,4), -1 < _Var17)) {
                _Var15 = lseek(param_1,__offset,0);
                if (_Var15 < 0) {
                  __errno_location();
                  FUN_00108cb0();
                }
                pcVar29 = DAT_0012b848;
                pcVar25 = DAT_0012b840;
                iVar8 = DAT_0012b810;
                if (_Var17 < local_168.st_size) goto LAB_00109355;
              }
            }
            else {
LAB_00109355:
              if (iVar8 == 2) {
                local_17b = '\0';
                local_1a8 = 0;
                goto LAB_001096ef;
              }
              if (DAT_0012b7e8 == '\0') {
                DAT_0012b803 = '\x01';
                DAT_0012b7a2 = '\x01';
              }
              DAT_0012b821 = DAT_0012b820;
              local_1b0 = local_1a8;
              pcVar29 = DAT_0012b848;
              pcVar25 = DAT_0012b840;
              cVar21 = cVar6;
            }
          }
          if (DAT_0012b7b8 != (char *)0x0) {
            DAT_0012b7b8 = pcVar29;
          }
          pcVar2 = pcVar29 + lVar13;
          DAT_0012b7c0 = pcVar29;
          if (pcVar2 == pcVar25) {
            if (local_1b8 != 0) {
              DAT_0012b840 = pcVar25 + 1;
              bVar30 = DAT_0012b7b0 != 0;
              *pcVar25 = cVar6;
              if (bVar30) {
                lVar13 = FUN_00108790(pcVar29 + (lVar13 - local_1b8));
                local_1a8 = local_1a8 + lVar13;
              }
              if (DAT_0012b7a8 != 0) {
                FUN_00108710(DAT_0012b840);
              }
            }
            goto LAB_001096b9;
          }
          pcVar29 = pcVar2;
          if (cVar21 != '\0') {
            while( true ) {
              *pcVar25 = '\0';
              sVar10 = strlen(pcVar29);
              *pcVar25 = cVar21;
              pcVar23 = pcVar29 + sVar10;
              if (pcVar29 + sVar10 == pcVar25) break;
              do {
                pcVar29 = pcVar23 + 1;
                *pcVar23 = cVar21;
                pcVar23 = pcVar29;
              } while (*pcVar29 == '\0');
            }
          }
          cVar28 = pcVar2[-1];
          pcVar2[-1] = cVar6;
          pvVar12 = memrchr(pcVar2 + -1,(int)cVar6,(size_t)(pcVar25 + (1 - (long)pcVar2)));
          pcVar2[-1] = cVar28;
          pcVar23 = pcVar2 + -local_1b8;
          pcVar29 = (char *)((long)pvVar12 + 1);
          if (pcVar2 == pcVar29) {
            local_1b8 = (long)pcVar25 - (long)pcVar23;
            pcVar29 = pcVar23;
          }
          else {
            local_1b8 = (long)pcVar25 - (long)pcVar29;
            if (pcVar23 < pcVar29) {
              if (DAT_0012b7b0 == 0) {
                if (DAT_0012b7a8 != 0) goto LAB_001095c2;
LAB_001097dd:
                local_17b = '\0';
                goto LAB_001096b9;
              }
              lVar13 = FUN_00108790(pcVar23,pcVar29);
              local_1a8 = local_1a8 + lVar13;
              if (DAT_0012b7a8 == 0) {
                if (DAT_0012b7b0 == 0) goto LAB_001097dd;
              }
              else {
LAB_001095c2:
                FUN_00108710(pcVar29);
                if (DAT_0012b7b0 == 0 && DAT_0012b7a8 == 0) goto LAB_001097dd;
              }
              if (DAT_0012b7a2 != '\0') {
                lVar13 = 0;
                if (-1 < local_1b0) {
                  lVar13 = local_1b0;
                }
                if (lVar13 < local_1a8) goto LAB_001097dd;
              }
            }
          }
          if (0 < DAT_0012b7f8) {
            lVar13 = 0;
            do {
              if (pcVar29 <= DAT_0012b848) break;
              if (pcVar29 == DAT_0012b7b8) goto LAB_0010964f;
              lVar13 = lVar13 + 1;
              pcVar23 = pcVar29;
              do {
                pcVar29 = pcVar23 + -1;
                pcVar1 = pcVar23 + -2;
                pcVar23 = pcVar29;
              } while (cVar6 != *pcVar1);
            } while (lVar13 != DAT_0012b7f8);
          }
          if (pcVar29 != DAT_0012b7b8) {
            DAT_0012b7b8 = (char *)0x0;
          }
LAB_0010964f:
          lVar13 = (long)pcVar25 - (long)pcVar29;
          pcVar25 = _DAT_0012b7c8;
          if ((DAT_0012b800 != '\0') &&
             (pcVar25 = DAT_0012b840 + (-lVar13 - (long)DAT_0012b848) + (long)_DAT_0012b7c8,
             SCARRY8((long)(DAT_0012b840 + (-lVar13 - (long)DAT_0012b848)),(long)_DAT_0012b7c8))) {
            uVar18 = dcgettext(0,"input is too large to count",5);
                    /* WARNING: Subroutine does not return */
            error(2,0,uVar18);
          }
          _DAT_0012b7c8 = pcVar25;
          if (DAT_0012b801 != '\0') {
            FUN_00107c90(pcVar29);
          }
          cVar28 = '\0';
          cVar5 = FUN_00108d20(lVar13,&local_168);
        } while (cVar5 != '\0');
        piVar14 = __errno_location();
        FUN_00108cb0(*piVar14);
        local_17b = '\0';
LAB_001096b9:
        uVar27 = CONCAT71((int7)((ulong)pcVar2 >> 8),local_1a8 == 0);
        DAT_0012b7a2 = cVar3;
        DAT_0012b803 = cVar4;
        if (((DAT_0012b810 == 0) && (cVar4 == '\0')) &&
           ((DAT_0012b888 != '\0' || ((-1 < local_1b0 && (local_1b0 < local_1a8)))))) {
          if (DAT_0012b890 == 0) {
            DAT_0012b890 = dcgettext(0,"(standard input)",5);
          }
          lVar13 = DAT_0012b890;
          uVar18 = dcgettext(0,"%s: binary file matches",5);
                    /* WARNING: Subroutine does not return */
          error(0,0,uVar18,lVar13);
        }
      }
    }
LAB_001096ef:
    if (DAT_0012b7e8 != '\0') {
      if (DAT_0012b808 != 0) {
        FUN_00107fa0();
        if (DAT_0012b804 == 0) {
          FUN_00107e70(0);
        }
        else {
          FUN_00107ec0(0x3a);
        }
      }
      FUN_00107b80(&DAT_00121118,local_1a8);
      if ((DAT_0012b7d8 != '\0') && (iVar8 = fflush_unlocked(stdout), iVar8 != 0)) {
        piVar14 = __errno_location();
        DAT_0012b8c0 = *piVar14;
      }
    }
    if (DAT_0012b80c != 0) {
      if (DAT_0012b80c == (local_1a8 == 0) + 1) {
        FUN_00107fa0();
        FUN_00107e70(DAT_0012b804 & 10);
        if ((DAT_0012b7d8 != '\0') && (iVar8 = fflush_unlocked(stdout), iVar8 != 0)) {
          piVar14 = __errno_location();
          DAT_0012b8c0 = *piVar14;
        }
      }
      goto LAB_001098d2;
    }
    if (param_1 != 0) goto LAB_00109454;
    if (DAT_0012b7b0 == 0) {
      if (((DAT_0012b830 == DAT_0012b828) || (DAT_0012b881 != 0)) ||
         (_Var17 = lseek(0,DAT_0012b828,0), -1 < _Var17)) goto LAB_001098de;
      goto LAB_00109465;
    }
    if ((local_17b != '\0') ||
       ((DAT_0012b881 == 0 &&
        ((_Var17 = lseek(0,0,2), -1 < _Var17 || (piVar14 = __errno_location(), *piVar14 == 0x16)))))
       ) goto LAB_001098de;
    if (((local_168.st_mode & 0xf000) == 0x1000) && (DAT_0012b7a0 != '\0')) {
      sVar20 = splice(0,(__off64_t *)0x0,1,(__off64_t *)0x0,0x18000,1);
      if (sVar20 < 0) {
        piVar14 = __errno_location();
        if (*piVar14 == 0x16) goto LAB_00109b90;
      }
      else {
        if (sVar20 == 0) goto LAB_001098de;
        do {
          sVar20 = splice(0,(__off64_t *)0x0,1,(__off64_t *)0x0,0x18000,1);
        } while (0 < sVar20);
        if (sVar20 == 0) goto LAB_001098de;
      }
    }
    else {
LAB_00109b90:
      do {
        lVar13 = FUN_0011da00(0,DAT_0012b860,DAT_0012b858);
        if (lVar13 == 0) goto LAB_001098de;
      } while (lVar13 != -1);
    }
  }
  else {
    piVar14 = __errno_location();
    uVar27 = 1;
    FUN_00108cb0(*piVar14);
LAB_001098d2:
    if (param_1 == 0) goto LAB_001098de;
LAB_00109454:
    iVar8 = close(param_1);
    if (iVar8 == 0) goto LAB_001098de;
  }
LAB_00109465:
  piVar14 = __errno_location();
  FUN_00108cb0(*piVar14);
LAB_001098de:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar27 & 0xffffffff;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
switchD_00109b0d_caseD_0:
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: grepfile @ 0xa0f0

undefined1 grepfile(undefined8 param_1,undefined8 param_2,char param_3,char param_4)

{
  undefined1 uVar1;
  bool bVar2;
  int iVar3;
  int *piVar4;
  uint uVar5;
  
  uVar5 = (-(uint)(param_3 == '\0') & 0x20000) + 0x100;
  if ((DAT_0012b884 == 2) || ((DAT_0012b884 == 0 && (param_4 != '\x01')))) {
    uVar5 = uVar5 | 0x800;
  }
  iVar3 = FUN_0011b5d0(param_1,param_2,uVar5);
  if (-1 < iVar3) {
    uVar1 = FUN_00109130(iVar3,param_4);
    return uVar1;
  }
  piVar4 = __errno_location();
  iVar3 = *piVar4;
  if ((param_3 != '\0') || (bVar2 = iVar3 == 0x28 || iVar3 == 0x1f, iVar3 != 0x28 && iVar3 != 0x1f))
  {
    FUN_00108cb0();
    bVar2 = true;
  }
  return bVar2;
}




// Function: usage @ 0xa170

void usage(int param_1)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  uVar1 = FUN_00119950();
  if (param_1 == 0) {
    uVar2 = dcgettext(0,"Usage: %s [OPTION]... PATTERNS [FILE]...\n",5);
    __printf_chk(1,uVar2,uVar1);
    uVar1 = dcgettext(0,"Search for PATTERNS in each FILE.\n",5);
    __printf_chk(1,uVar1);
    uVar1 = FUN_00119950();
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
    FUN_0011ee20();
  }
  else {
    uVar2 = dcgettext(0,"Usage: %s [OPTION]... PATTERNS [FILE]...\n",5);
    __fprintf_chk(stderr,1,uVar2,uVar1);
    uVar1 = FUN_00119950();
    uVar2 = dcgettext(0,"Try \'%s --help\' for more information.\n",5);
    __fprintf_chk(stderr,1,uVar2,uVar1);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: fgrep_to_grep_pattern @ 0xa3e0

void fgrep_to_grep_pattern(undefined8 *param_1,size_t *param_2)

{
  size_t sVar1;
  byte bVar2;
  byte *pbVar3;
  size_t sVar4;
  byte *__src;
  size_t __n;
  size_t sVar5;
  byte *__dest;
  long in_FS_OFFSET;
  undefined8 local_48;
  long local_40;
  
  sVar5 = *param_2;
  __src = (byte *)*param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_48 = 0;
  pbVar3 = (byte *)FUN_0011f050(sVar5 + 1,2);
  __dest = pbVar3;
  if (sVar5 == 0) {
    sVar5 = 0;
  }
  else {
    do {
      while( true ) {
        sVar4 = (size_t)(char)(&DAT_0012b183)[*__src];
        if ((&DAT_0012b183)[*__src] == -2) {
          sVar4 = FUN_0011ac90(__src,sVar5,&local_48);
        }
        if (sVar4 != 0xffffffffffffffff) break;
        local_48 = 0;
LAB_0010a49c:
        bVar2 = *__src;
        if (((byte)(bVar2 - 0x24) < 0x3b) &&
           ((0x580000000000441U >> ((ulong)(bVar2 - 0x24) & 0x3f) & 1) != 0)) {
          *__dest = 0x5c;
          bVar2 = *__src;
          __dest = __dest + 1;
        }
        sVar5 = sVar5 - 1;
        *__dest = bVar2;
        __dest = __dest + 1;
        __src = __src + 1;
        if (sVar5 == 0) goto LAB_0010a4de;
      }
      if (sVar4 == 1) goto LAB_0010a49c;
      __n = sVar5;
      sVar1 = 0;
      if (sVar4 != 0xfffffffffffffffe) {
        __n = sVar4;
        sVar1 = sVar5 - sVar4;
      }
      sVar5 = sVar1;
      __dest = mempcpy(__dest,__src,__n);
      __src = __src + __n;
    } while (sVar5 != 0);
LAB_0010a4de:
    sVar5 = (long)__dest - (long)pbVar3;
  }
  *__dest = 10;
  free((void *)*param_1);
  *param_1 = pbVar3;
  *param_2 = sVar5;
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: Fcompile @ 0xa570

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
  uVar1 = FUN_0010e5c0(1);
  __s = param_1;
  do {
    pvVar2 = rawmemchr(__s,10);
    __n = (long)pvVar2 - (long)__s;
    if (DAT_0012b685 != '\0') {
      if ((param_1 < __s) && (DAT_0012b684 == '\n')) {
        __s = __s + -1;
      }
      else {
        cVar5 = DAT_0012b684;
        if ((long)local_48 <= (long)(__n + 1)) {
          free(__ptr);
          local_48 = __n;
          __ptr = (char *)FUN_0011f1f0(0,&local_48,2,0xffffffffffffffff,1);
          cVar5 = DAT_0012b684;
          *__ptr = DAT_0012b684;
        }
        memcpy(__ptr + 1,__s,__n);
        __ptr[__n + 1] = cVar5;
        __s = __ptr;
      }
      __n = __n + 2;
    }
    FUN_0010c9d0(uVar1,__s,__n);
    __s = (char *)((long)pvVar2 + 1U);
  } while ((char *)((long)pvVar2 + 1U) <= param_1 + param_2);
  free(__ptr);
  uVar3 = FUN_0010ceb0(uVar1);
  FUN_0010cec0(uVar1);
  puVar4 = (undefined8 *)FUN_0011ef10(0x28);
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




// Function: Fexecute @ 0xa710

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
  
  cVar2 = DAT_0012b684;
  pvVar8 = (void *)((long)param_2 + param_3);
  uVar1 = *param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar3 = (uint)DAT_0012b685;
  bVar10 = DAT_0012b180 & ((DAT_0012b182 | DAT_0012b685) ^ 1);
  bVar11 = bVar10 | DAT_0012b686;
  uVar12 = uVar3 ^ 1;
  pvVar13 = param_2;
  if (param_5 != (void *)0x0) {
    pvVar13 = param_5;
  }
  local_88 = pvVar13;
  if (pvVar13 <= pvVar8) {
    while (lVar4 = FUN_0010e3e0(uVar1,(long)pvVar13 - (ulong)uVar3,
                                (long)pvVar8 + ((ulong)uVar3 - (long)pvVar13),local_78,
                                (bVar11 | param_5 != (void *)0x0) & uVar12), -1 < lVar4) {
      lVar4 = (long)pvVar13 + lVar4;
      local_80 = 0;
      lVar14 = local_68 - (int)((uint)DAT_0012b685 * 2);
      if ((bVar10 == 0) || (lVar6 = FUN_0010e640(&local_88,&local_80,lVar4,pvVar8), lVar6 == 0)) {
        pvVar13 = local_88;
        if ((DAT_0012b686 == 1) || (param_5 == (void *)0x0)) {
          if (DAT_0012b685 != 0) {
            lVar14 = lVar14 + (ulong)(param_5 == (void *)0x0);
            goto LAB_0010aa10;
          }
          pvVar7 = pvVar8;
          if (DAT_0012b686 != 0) {
            if (local_80 == 0) {
              pvVar5 = memrchr(local_88,(int)cVar2,lVar4 - (long)local_88);
              if (pvVar5 != (void *)0x0) {
                pvVar13 = (void *)((long)pvVar5 + 1);
                local_88 = pvVar13;
              }
LAB_0010a894:
              lVar6 = FUN_0010e830(pvVar13,lVar4,pvVar8);
            }
            else {
              if (local_80 < 1) goto LAB_0010a894;
              lVar6 = FUN_0010e820(lVar4 - local_80,pvVar8);
            }
            if (lVar6 == 0) {
              while( true ) {
                pvVar13 = (void *)(lVar4 + lVar14);
                lVar6 = FUN_0010e820(pvVar13,pvVar8);
                if (lVar6 == 0) {
                  if (param_5 != (void *)0x0) goto LAB_0010aa10;
                  goto LAB_0010aa4d;
                }
                if ((param_5 == (void *)0x0) && (DAT_0012b180 == 0)) break;
                if ((lVar14 == 0) ||
                   (lVar6 = FUN_0010e3e0(uVar1,lVar4,lVar14 + -1,local_58,1), lVar14 = local_48,
                   lVar6 != 0)) goto LAB_0010a970;
              }
              if (param_1[4] == 0) {
                FUN_0010a3e0(param_1 + 2,param_1 + 3);
                uVar9 = FUN_00106a70(param_1[2],param_1[3],0x10a46,0);
                param_1[4] = uVar9;
              }
              if (pvVar13 < pvVar8) {
                pvVar7 = rawmemchr(pvVar13,(int)cVar2);
                pvVar7 = (void *)((long)pvVar7 + 1);
              }
              lVar14 = FUN_00107110(param_1[4],lVar4,(long)pvVar7 - lVar4,param_4,0);
              if (-1 < lVar14) goto LAB_0010aa67;
              lVar4 = (long)pvVar7 - 1;
            }
LAB_0010a970:
            lVar14 = FUN_0010e810(lVar4,pvVar8);
            local_88 = (void *)(lVar4 + lVar14);
            pvVar13 = (void *)((long)local_88 + 1);
            if ((void *)((long)local_88 + 1) <= pvVar8) goto LAB_0010a98c;
            break;
          }
          pvVar13 = (void *)(lVar4 + lVar14);
LAB_0010aa4d:
          if (pvVar13 < pvVar8) {
            pvVar7 = rawmemchr(pvVar13,(int)cVar2);
            pvVar7 = (void *)((long)pvVar7 + 1);
          }
LAB_0010aa67:
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
LAB_0010aa10:
          lVar4 = lVar4 - (long)param_2;
        }
        *param_4 = lVar14;
        goto LAB_0010aa20;
      }
      pvVar13 = local_88;
      if (pvVar8 < local_88) break;
LAB_0010a98c:
      uVar3 = (uint)DAT_0012b685;
    }
  }
  lVar4 = -1;
LAB_0010aa20:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: treefails @ 0xab10

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




// Function: hasevery @ 0xabd0

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




// Function: memchr_kwset @ 0xac50

byte * memchr_kwset(byte *param_1,long param_2,long param_3)

{
  byte *pbVar1;
  
  pbVar1 = param_1 + param_2;
  if (*(int *)(param_3 + 0x98c) < 0) {
    if (param_1 < pbVar1) {
      do {
        if (*(long *)(param_3 + 0x170 + (ulong)*param_1 * 8) != 0) {
          return param_1;
        }
        param_1 = param_1 + 1;
      } while (pbVar1 != param_1);
      return (byte *)0x0;
    }
  }
  else {
    while (param_1 < pbVar1) {
      if (*(long *)(param_3 + 0x170 + (ulong)*param_1 * 8) != 0) {
        return param_1;
      }
      param_1 = param_1 + 1;
      if (((ulong)param_1 & 0xf) == 0) {
        pbVar1 = (byte *)FUN_0011b140(param_1,*(undefined4 *)(param_3 + 0x988),
                                      *(int *)(param_3 + 0x98c),(long)pbVar1 - (long)param_1);
        return pbVar1;
      }
    }
  }
  return (byte *)0x0;
}




// Function: acexec @ 0xaf60

void acexec(void)

{
  FUN_0010ace0();
  return;
}




// Function: bmexec_trans @ 0xaf70

long bmexec_trans(long param_1,long param_2,long param_3)

{
  long lVar1;
  byte bVar2;
  byte bVar3;
  long lVar4;
  long lVar5;
  long lVar6;
  byte bVar7;
  ulong uVar8;
  long lVar9;
  byte bVar10;
  long lVar11;
  long lVar12;
  long lVar13;
  ulong uVar14;
  ulong uVar15;
  
  lVar6 = *(long *)(param_1 + 0x68);
  lVar12 = 0;
  if (lVar6 != 0) {
    lVar12 = *(long *)(param_1 + 0x980);
    if (lVar6 <= param_3) {
      if (lVar6 != 1) {
        lVar1 = param_1 + 0x70;
        bVar2 = *(byte *)(param_1 + 0x988);
        bVar3 = *(byte *)(param_1 + 0x990);
        lVar9 = *(long *)(param_1 + 0x970) + lVar6;
        uVar8 = param_2 + lVar6;
        if ((SEXT816(lVar6 * 0xc) == SEXT816(lVar6) * SEXT816(0xc)) && (lVar6 * 0xc < param_3)) {
          uVar14 = param_3 + lVar6 * -0xb + param_2;
          uVar15 = uVar8;
          if (uVar8 <= uVar14) {
LAB_0010b168:
            lVar5 = *(byte *)(lVar1 + (ulong)*(byte *)(uVar15 - 1)) + uVar15;
            bVar7 = *(byte *)(lVar1 + (ulong)*(byte *)(lVar5 + -1));
            uVar8 = lVar5 + (ulong)bVar7;
            if (((bVar7 == 0) ||
                (lVar5 = *(byte *)(lVar1 + (ulong)*(byte *)(uVar8 - 1)) + uVar8,
                lVar5 = lVar5 + (ulong)*(byte *)(lVar1 + (ulong)*(byte *)(lVar5 + -1)),
                bVar7 = *(byte *)(lVar1 + (ulong)*(byte *)(lVar5 + -1)),
                uVar8 = (ulong)bVar7 + lVar5, bVar7 == 0)) ||
               (lVar5 = *(byte *)(lVar1 + (ulong)*(byte *)(uVar8 - 1)) + uVar8,
               lVar5 = lVar5 + (ulong)*(byte *)(lVar1 + (ulong)*(byte *)(lVar5 + -1)),
               bVar7 = *(byte *)(lVar1 + (ulong)*(byte *)(lVar5 + -1)), uVar8 = (ulong)bVar7 + lVar5
               , bVar7 == 0)) {
LAB_0010b260:
              lVar5 = 0;
              lVar13 = lVar6;
LAB_0010b2ae:
              bVar7 = *(byte *)(uVar8 - 2);
              if (lVar12 != 0) {
                bVar7 = *(byte *)(lVar12 + (ulong)bVar7);
              }
              if (bVar3 == bVar7) {
                lVar11 = -3;
                lVar4 = 2;
                do {
                  lVar4 = lVar4 + 1;
                  if (lVar13 < lVar4) {
                    lVar4 = lVar13 + lVar5 + 1;
                    if (lVar6 < lVar4) goto LAB_0010b3d0;
                    uVar15 = ~(lVar13 + lVar5);
                    goto LAB_0010b321;
                  }
                  bVar7 = *(byte *)(uVar8 + lVar11);
                  bVar10 = *(byte *)(lVar9 + lVar11);
                  if (lVar12 != 0) {
                    bVar7 = *(byte *)(lVar12 + (ulong)bVar7);
                    bVar10 = *(byte *)(lVar12 + (ulong)bVar10);
                  }
                  lVar11 = lVar11 + -1;
                } while (bVar10 == bVar7);
                goto LAB_0010b347;
              }
              lVar5 = 0;
              lVar4 = 2;
              goto LAB_0010b277;
            }
            lVar5 = uVar8 + *(byte *)(lVar1 + (ulong)*(byte *)(uVar8 - 1));
            uVar8 = lVar5 + (ulong)*(byte *)(lVar1 + (ulong)*(byte *)(lVar5 + -1));
            if (0x7f < (long)(uVar8 - uVar15)) goto LAB_0010b378;
            lVar5 = FUN_0010ac50(uVar8 - 1,(param_2 + param_3) - (uVar8 - 1),param_1);
            if (lVar5 == 0) goto LAB_0010b11c;
            uVar8 = lVar5 + 1;
            if (uVar8 < uVar14) goto LAB_0010b260;
          }
        }
LAB_0010affe:
        uVar15 = (ulong)*(byte *)(param_1 + 0x70 + (ulong)*(byte *)(uVar8 - 1));
LAB_0010b010:
        do {
          if ((long)((param_3 + param_2) - uVar8) < (long)uVar15) goto LAB_0010b11c;
          uVar8 = uVar8 + uVar15;
          bVar7 = *(byte *)(lVar1 + (ulong)*(byte *)(uVar8 - 1));
          uVar15 = (ulong)bVar7;
          if (bVar7 == 0) {
            lVar5 = 0;
            lVar13 = lVar6;
LAB_0010b071:
            bVar7 = *(byte *)(uVar8 - 2);
            if (lVar12 != 0) {
              bVar7 = *(byte *)(lVar12 + (ulong)bVar7);
            }
            if (bVar3 == bVar7) {
              lVar11 = -3;
              lVar4 = 2;
              do {
                lVar4 = lVar4 + 1;
                if (lVar13 < lVar4) {
                  lVar4 = lVar13 + lVar5 + 1;
                  if (lVar6 < lVar4) goto LAB_0010b390;
                  uVar14 = ~(lVar13 + lVar5);
                  goto LAB_0010b0e9;
                }
                bVar7 = *(byte *)(uVar8 + lVar11);
                bVar10 = *(byte *)(lVar9 + lVar11);
                if (lVar12 != 0) {
                  bVar7 = *(byte *)(lVar12 + (ulong)bVar7);
                  bVar10 = *(byte *)(lVar12 + (ulong)bVar10);
                }
                lVar11 = lVar11 + -1;
              } while (bVar10 == bVar7);
              goto LAB_0010b10f;
            }
            lVar5 = 0;
            lVar4 = 2;
            goto LAB_0010b047;
          }
        } while( true );
      }
      lVar6 = FUN_0010ac50(param_2,param_3,param_1);
      if (lVar6 != 0) {
        return lVar6 - param_2;
      }
    }
LAB_0010b11c:
    lVar12 = -1;
  }
  return lVar12;
LAB_0010b321:
  bVar7 = *(byte *)(uVar8 + uVar15);
  bVar10 = *(byte *)(lVar9 + uVar15);
  if (lVar12 != 0) {
    bVar7 = *(byte *)(lVar12 + (ulong)bVar7);
    bVar10 = *(byte *)(lVar12 + (ulong)bVar10);
  }
  if (bVar10 != bVar7) goto code_r0x0010b33e;
  lVar4 = lVar4 + 1;
  uVar15 = uVar15 - 1;
  if (lVar6 < lVar4) goto LAB_0010b3d0;
  goto LAB_0010b321;
code_r0x0010b33e:
  if (lVar6 < lVar4) {
LAB_0010b3d0:
    return (uVar8 - lVar6) - param_2;
  }
LAB_0010b347:
  lVar5 = lVar4 * 8 + -0x10;
LAB_0010b277:
  lVar13 = *(long *)(*(long *)(param_1 + 0x978) + lVar5);
  uVar8 = uVar8 + lVar13;
  bVar7 = *(byte *)(uVar8 - 1);
  uVar15 = (ulong)bVar7;
  if (uVar14 < uVar8) goto LAB_0010affe;
  if (lVar12 != 0) {
    bVar7 = *(byte *)(lVar12 + (ulong)bVar7);
  }
  if (bVar2 != bVar7) goto LAB_0010b370;
  lVar5 = lVar4 + -1;
  goto LAB_0010b2ae;
LAB_0010b370:
  uVar8 = *(byte *)(lVar1 + uVar15) + uVar8;
LAB_0010b378:
  uVar15 = uVar8;
  if (uVar14 < uVar8) goto LAB_0010affe;
  goto LAB_0010b168;
LAB_0010b0e9:
  bVar7 = *(byte *)(uVar8 + uVar14);
  bVar10 = *(byte *)(lVar9 + uVar14);
  if (lVar12 != 0) {
    bVar7 = *(byte *)(lVar12 + (ulong)bVar7);
    bVar10 = *(byte *)(lVar12 + (ulong)bVar10);
  }
  if (bVar10 != bVar7) goto code_r0x0010b106;
  lVar4 = lVar4 + 1;
  uVar14 = uVar14 - 1;
  if (lVar6 < lVar4) goto LAB_0010b390;
  goto LAB_0010b0e9;
code_r0x0010b106:
  if (lVar6 < lVar4) {
LAB_0010b390:
    return (uVar8 - lVar6) - param_2;
  }
LAB_0010b10f:
  lVar5 = lVar4 * 8 + -0x10;
LAB_0010b047:
  lVar13 = *(long *)(*(long *)(param_1 + 0x978) + lVar5);
  uVar8 = uVar8 + lVar13;
  if ((ulong)(param_3 + param_2) < uVar8) goto LAB_0010b010;
  bVar7 = *(byte *)(uVar8 - 1);
  if (lVar12 != 0) {
    bVar7 = *(byte *)(lVar12 + (ulong)bVar7);
  }
  if (bVar2 != bVar7) goto LAB_0010b010;
  lVar5 = lVar4 + -1;
  goto LAB_0010b071;
}




// Function: bmexec @ 0xb3f0

void bmexec(long param_1,undefined8 param_2,undefined8 param_3,undefined8 *param_4)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  uVar2 = FUN_0010af70();
  uVar1 = *(undefined8 *)(param_1 + 0x68);
  *param_4 = 0;
  param_4[1] = uVar2;
  param_4[2] = uVar1;
  return;
}




// Function: kwsalloc @ 0xc880

long kwsalloc(undefined8 param_1)

{
  long lVar1;
  undefined8 *puVar2;
  ulong uVar3;
  ulong uVar4;
  undefined8 *puVar5;
  
  lVar1 = FUN_0011ef10(0x9a0);
  _obstack_begin(lVar1,0,0,FUN_0011ef10,PTR_free_0012afb8);
  uVar4 = *(ulong *)(lVar1 + 0x20);
  *(undefined8 *)(lVar1 + 0x58) = 0;
  if (uVar4 - *(long *)(lVar1 + 0x18) < 0x40) {
    _obstack_newchunk(lVar1,0x40);
    puVar5 = *(undefined8 **)(lVar1 + 0x10);
    uVar4 = *(ulong *)(lVar1 + 0x20);
    puVar2 = (undefined8 *)(*(long *)(lVar1 + 0x18) + 0x40);
    if (puVar2 != puVar5) goto LAB_0010c8ea;
  }
  else {
    puVar5 = *(undefined8 **)(lVar1 + 0x10);
    puVar2 = (undefined8 *)(*(long *)(lVar1 + 0x18) + 0x40);
    if (puVar2 != puVar5) goto LAB_0010c8ea;
  }
  *(byte *)(lVar1 + 0x50) = *(byte *)(lVar1 + 0x50) | 2;
LAB_0010c8ea:
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
  *(code **)(lVar1 + 0x998) = FUN_0010af60;
  *(undefined8 *)(lVar1 + 0x970) = 0;
  return lVar1;
}




// Function: kwsincr @ 0xc9d0

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
  if (pcVar18 == FUN_0010b3f0) {
    param_2 = param_2 + param_3;
  }
joined_r0x0010ca25:
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
    if (pcVar18 == FUN_0010b3f0) {
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
          goto joined_r0x0010ca25;
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
      if (puVar8 == puVar16) goto LAB_0010cdc7;
    }
    else {
      puVar16 = *(undefined8 **)(param_1 + 0x10);
      puVar8 = (undefined8 *)(*(long *)(param_1 + 0x18) + 0x20);
      if (puVar8 == puVar16) {
LAB_0010cdc7:
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
LAB_0010cb2f:
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
LAB_0010cd6f:
        *(byte *)(param_1 + 0x50) = *(byte *)(param_1 + 0x50) | 2;
      }
    }
    else {
      *(long **)(param_1 + 0x18) = plVar9;
      *(long **)(param_1 + 0x10) = plVar9;
      *puVar16 = 0;
      puVar16[1] = 0;
      if (uVar13 - (long)plVar9 < 0x40) goto LAB_0010cb2f;
      plVar10 = plVar9 + 8;
      if (plVar10 == plVar9) goto LAB_0010cd6f;
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
    if (lVar11 == 0) goto joined_r0x0010ca25;
    iVar14 = local_d8[lVar11];
  }
  if (iVar14 == 0) {
    cVar7 = cVar7 + -1;
    *(char *)((long)plVar9 + 0x19) = cVar7;
  }
  else {
    if (iVar14 != 1) goto joined_r0x0010ca25;
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
        if (local_d8[lVar11 + 1] != 1) goto LAB_0010ce99;
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
LAB_0010ce99:
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
        if (local_d8[lVar11 + 1] != 1) goto LAB_0010ce99;
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
  goto joined_r0x0010ca25;
}




// Function: kwswords @ 0xceb0

undefined8 kwswords(long param_1)

{
  return *(undefined8 *)(param_1 + 0x58);
}




// Function: kwsprep @ 0xcec0

void kwsprep(undefined8 *param_1)

{
  long lVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  undefined1 auVar6 [16];
  undefined1 auVar7 [16];
  char cVar8;
  int iVar9;
  ulong uVar10;
  long *plVar11;
  long *plVar12;
  long lVar13;
  ulong uVar14;
  long lVar15;
  long lVar16;
  undefined8 *__ptr;
  long lVar17;
  long *plVar18;
  ulong uVar19;
  ulong uVar20;
  long *plVar21;
  long *extraout_RDX;
  long *extraout_RDX_00;
  long *extraout_RDX_01;
  long *extraout_RDX_02;
  long *plVar22;
  long lVar23;
  int iVar24;
  int iVar25;
  long lVar26;
  long *plVar27;
  undefined8 *puVar28;
  ulong uVar29;
  undefined8 *puVar30;
  long *plVar31;
  long *plVar32;
  ulong uVar33;
  uint uVar34;
  long **pplVar35;
  undefined8 *puVar36;
  byte bVar37;
  long in_FS_OFFSET;
  byte bVar38;
  long *local_950;
  undefined8 local_948 [256];
  undefined8 local_148 [33];
  long local_40;
  
  bVar38 = 0;
  lVar1 = param_1[0x130];
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  puVar36 = local_148;
  if (lVar1 == 0) {
    puVar36 = param_1 + 0xe;
  }
  lVar15 = param_1[0xb];
  if (lVar15 == 1) {
    pplVar35 = &local_950;
    plVar21 = (long *)param_1[0xc];
    plVar31 = plVar21;
    local_950 = plVar21;
    for (; plVar21 != (long *)0x0; plVar21 = (long *)plVar21[3]) {
      plVar32 = (long *)plVar21[1];
      if (plVar32 != (long *)0x0) {
        plVar18 = (long *)*plVar32;
        if (plVar18 != (long *)0x0) {
          if (*plVar18 != 0) {
            FUN_0010b420(*plVar18,pplVar35);
          }
          if (plVar18[1] != 0) {
            FUN_0010b420(plVar18[1],pplVar35);
          }
          plVar18 = (long *)plVar18[2];
          local_950[3] = (long)plVar18;
          local_950 = plVar18;
        }
        plVar18 = (long *)plVar32[1];
        plVar22 = local_950;
        if (plVar18 != (long *)0x0) {
          if (*plVar18 != 0) {
            FUN_0010b420(*plVar18,pplVar35);
          }
          if (plVar18[1] != 0) {
            FUN_0010b420(plVar18[1],pplVar35);
          }
          plVar22 = (long *)plVar18[2];
          local_950[3] = (long)plVar22;
        }
        local_950 = (long *)plVar32[2];
        plVar22[3] = (long)local_950;
      }
    }
    uVar10 = param_1[4];
    uVar19 = param_1[0xd];
    if (uVar10 - param_1[3] < uVar19) {
      _obstack_newchunk(param_1,uVar19);
      lVar26 = param_1[2];
      uVar10 = param_1[4];
      plVar31 = (long *)param_1[0xc];
      lVar16 = param_1[3] + uVar19;
      uVar19 = param_1[0xd];
      if (lVar16 == lVar26) goto LAB_0010e3d2;
    }
    else {
      lVar26 = param_1[2];
      lVar16 = param_1[3] + uVar19;
      if (lVar16 == lVar26) {
LAB_0010e3d2:
        *(byte *)(param_1 + 10) = *(byte *)(param_1 + 10) | 2;
      }
    }
    param_1[0x12e] = lVar26;
    uVar14 = lVar16 + param_1[6] & ~param_1[6];
    if (uVar14 - param_1[1] <= uVar10 - param_1[1]) {
      uVar10 = uVar14;
    }
    param_1[3] = uVar10;
    param_1[2] = uVar10;
    if (0 < (long)uVar19) {
      lVar16 = 0;
      while( true ) {
        *(undefined1 *)(lVar26 + lVar16) = *(undefined1 *)(plVar31[1] + 0x18);
        lVar16 = lVar16 + 1;
        plVar31 = (long *)plVar31[3];
        if ((long)param_1[0xd] <= lVar16) break;
        lVar26 = param_1[0x12e];
      }
    }
    __ptr = (undefined8 *)FUN_0010c880(param_1[0x130]);
    uVar2 = param_1[0xd];
    uVar3 = param_1[0x12e];
    __ptr[0x133] = FUN_0010b3f0;
    FUN_0010c9d0(__ptr,uVar3,uVar2);
    _obstack_free(param_1,0);
    puVar28 = __ptr;
    puVar30 = param_1;
    for (lVar26 = 0x134; lVar26 != 0; lVar26 = lVar26 + -1) {
      *puVar30 = *puVar28;
      puVar28 = puVar28 + (ulong)bVar38 * -2 + 1;
      puVar30 = puVar30 + (ulong)bVar38 * -2 + 1;
    }
    free(__ptr);
    uVar10 = param_1[0xd];
    if (0xff < (long)uVar10) {
      uVar10 = 0xff;
    }
    uVar10 = uVar10 & 0xff;
    auVar6._8_8_ = 0;
    auVar6._0_8_ = uVar10;
    *puVar36 = SUB168(auVar6 * ZEXT816(0x101010101010101),0);
    lVar26 = uVar10 * 0x101010101010101 + SUB168(auVar6 * ZEXT816(0x101010101010101),8);
    puVar36[1] = lVar26;
    puVar36[3] = lVar26;
    puVar36[2] = uVar10 * 0x101010101010101;
    puVar36[5] = lVar26;
    puVar36[7] = lVar26;
    puVar36[9] = lVar26;
    puVar36[4] = uVar10 * 0x101010101010101;
    puVar36[0xb] = lVar26;
    puVar36[0xd] = lVar26;
    puVar36[0xf] = lVar26;
    puVar36[6] = uVar10 * 0x101010101010101;
    puVar36[0x11] = lVar26;
    puVar36[0x13] = lVar26;
    puVar36[0x15] = lVar26;
    puVar36[8] = uVar10 * 0x101010101010101;
    puVar36[0x17] = lVar26;
    puVar36[0x19] = lVar26;
    puVar36[0x1b] = lVar26;
    puVar36[10] = uVar10 * 0x101010101010101;
    puVar36[0x1d] = lVar26;
    puVar36[0xc] = uVar10 * 0x101010101010101;
    puVar36[0xe] = uVar10 * 0x101010101010101;
    puVar36[0x10] = uVar10 * 0x101010101010101;
    puVar36[0x12] = uVar10 * 0x101010101010101;
    puVar36[0x14] = uVar10 * 0x101010101010101;
    puVar36[0x16] = uVar10 * 0x101010101010101;
    puVar36[0x18] = uVar10 * 0x101010101010101;
    puVar36[0x1a] = uVar10 * 0x101010101010101;
    puVar36[0x1c] = uVar10 * 0x101010101010101;
    auVar7._8_8_ = 0;
    auVar7._0_8_ = uVar10;
    plVar21 = SUB168(auVar7 * ZEXT816(0x101010101010101),8);
    puVar36[0x1e] = SUB168(auVar7 * ZEXT816(0x101010101010101),0);
    puVar36[0x1f] = lVar26;
    plVar31 = (long *)param_1[0xc];
    if (plVar31 != (long *)0x0) goto LAB_0010d084;
    lVar26 = 0;
    local_950 = plVar31;
LAB_0010e1bb:
    for (lVar26 = *(long *)(lVar26 + 0x18); lVar26 != 0; lVar26 = *(long *)(lVar26 + 0x18)) {
      lVar16 = *(long *)(*(long *)(lVar26 + 0x10) + 0x38);
      lVar17 = *(long *)(lVar26 + 0x38);
      if (lVar16 < *(long *)(lVar26 + 0x38)) {
        *(long *)(lVar26 + 0x38) = lVar16;
        lVar17 = lVar16;
      }
      if (lVar17 < *(long *)(lVar26 + 0x30)) {
        *(long *)(lVar26 + 0x30) = lVar17;
      }
    }
  }
  else {
    uVar10 = param_1[0xd];
    if (0xff < (long)uVar10) {
      uVar10 = 0xff;
    }
    uVar10 = uVar10 & 0xff;
    auVar4._8_8_ = 0;
    auVar4._0_8_ = uVar10;
    *puVar36 = SUB168(auVar4 * ZEXT816(0x101010101010101),0);
    lVar26 = uVar10 * 0x101010101010101 + SUB168(auVar4 * ZEXT816(0x101010101010101),8);
    puVar36[1] = lVar26;
    puVar36[3] = lVar26;
    puVar36[2] = uVar10 * 0x101010101010101;
    puVar36[5] = lVar26;
    puVar36[7] = lVar26;
    puVar36[9] = lVar26;
    puVar36[4] = uVar10 * 0x101010101010101;
    puVar36[0xb] = lVar26;
    puVar36[0xd] = lVar26;
    puVar36[0xf] = lVar26;
    puVar36[6] = uVar10 * 0x101010101010101;
    puVar36[0x11] = lVar26;
    puVar36[0x13] = lVar26;
    puVar36[0x15] = lVar26;
    puVar36[8] = uVar10 * 0x101010101010101;
    puVar36[0x17] = lVar26;
    puVar36[0x19] = lVar26;
    puVar36[0x1b] = lVar26;
    puVar36[10] = uVar10 * 0x101010101010101;
    puVar36[0x1d] = lVar26;
    puVar36[0xc] = uVar10 * 0x101010101010101;
    puVar36[0xe] = uVar10 * 0x101010101010101;
    puVar36[0x10] = uVar10 * 0x101010101010101;
    puVar36[0x12] = uVar10 * 0x101010101010101;
    puVar36[0x14] = uVar10 * 0x101010101010101;
    puVar36[0x16] = uVar10 * 0x101010101010101;
    puVar36[0x18] = uVar10 * 0x101010101010101;
    puVar36[0x1a] = uVar10 * 0x101010101010101;
    puVar36[0x1c] = uVar10 * 0x101010101010101;
    auVar5._8_8_ = 0;
    auVar5._0_8_ = uVar10;
    plVar21 = SUB168(auVar5 * ZEXT816(0x101010101010101),8);
    puVar36[0x1e] = SUB168(auVar5 * ZEXT816(0x101010101010101),0);
    puVar36[0x1f] = lVar26;
    plVar31 = (long *)param_1[0xc];
    local_950 = plVar31;
    if (plVar31 != (long *)0x0) {
LAB_0010d084:
      local_950 = plVar31;
LAB_0010dc63:
      do {
        puVar28 = (undefined8 *)plVar31[1];
        lVar16 = 0;
        if (puVar28 != (undefined8 *)0x0) {
          plVar32 = (long *)*puVar28;
          if (plVar32 == (long *)0x0) {
            plVar22 = (long *)puVar28[1];
            if (plVar22 != (long *)0x0) {
LAB_0010d327:
              plVar18 = (long *)*plVar22;
              if (plVar18 != (long *)0x0) {
                plVar11 = (long *)*plVar18;
                if (plVar11 != (long *)0x0) {
                  plVar12 = (long *)*plVar11;
                  if (plVar12 != (long *)0x0) {
                    if (*plVar12 != 0) {
                      FUN_0010b420(*plVar12,&local_950,plVar21);
                      plVar21 = extraout_RDX_00;
                    }
                    if (plVar12[1] != 0) {
                      FUN_0010b420(plVar12[1],&local_950,plVar21);
                    }
                    plVar21 = (long *)plVar12[2];
                    local_950[3] = (long)plVar21;
                    local_950 = plVar21;
                  }
                  plVar21 = (long *)plVar11[1];
                  plVar12 = local_950;
                  if (plVar21 != (long *)0x0) {
                    if (*plVar21 != 0) {
                      FUN_0010b420(*plVar21,&local_950);
                    }
                    if (plVar21[1] != 0) {
                      FUN_0010b420(plVar21[1],&local_950);
                    }
                    plVar12 = (long *)plVar21[2];
                    local_950[3] = (long)plVar12;
                  }
                  local_950 = (long *)plVar11[2];
                  plVar12[3] = (long)local_950;
                }
                plVar21 = (long *)plVar18[1];
                if (plVar21 != (long *)0x0) {
                  plVar11 = (long *)*plVar21;
                  if (plVar11 != (long *)0x0) {
                    if (*plVar11 != 0) {
                      FUN_0010b420(*plVar11,&local_950);
                    }
                    if (plVar11[1] != 0) {
                      FUN_0010b420(plVar11[1],&local_950);
                    }
                    plVar11 = (long *)plVar11[2];
                    local_950[3] = (long)plVar11;
                    local_950 = plVar11;
                  }
                  plVar11 = (long *)plVar21[1];
                  plVar12 = local_950;
                  if (plVar11 != (long *)0x0) {
                    if (*plVar11 != 0) {
                      FUN_0010b420(*plVar11,&local_950);
                    }
                    if (plVar11[1] != 0) {
                      FUN_0010b420(plVar11[1],&local_950);
                    }
                    plVar12 = (long *)plVar11[2];
                    local_950[3] = (long)plVar12;
                  }
                  local_950 = (long *)plVar21[2];
                  plVar12[3] = (long)local_950;
                }
                plVar21 = (long *)plVar18[2];
                local_950[3] = (long)plVar21;
                local_950 = plVar21;
              }
              plVar21 = (long *)plVar22[1];
              if (plVar21 != (long *)0x0) {
                plVar11 = (long *)*plVar21;
                if (plVar11 != (long *)0x0) {
                  plVar12 = (long *)*plVar11;
                  if (plVar12 != (long *)0x0) {
                    if (*plVar12 != 0) {
                      FUN_0010b420(*plVar12,&local_950);
                    }
                    if (plVar12[1] != 0) {
                      FUN_0010b420(plVar12[1],&local_950);
                    }
                    plVar12 = (long *)plVar12[2];
                    local_950[3] = (long)plVar12;
                    local_950 = plVar12;
                  }
                  plVar12 = (long *)plVar11[1];
                  if (plVar12 != (long *)0x0) {
                    if (*plVar12 != 0) {
                      FUN_0010b420(*plVar12,&local_950);
                    }
                    if (plVar12[1] != 0) {
                      FUN_0010b420(plVar12[1],&local_950);
                    }
                    plVar12 = (long *)plVar12[2];
                    local_950[3] = (long)plVar12;
                    local_950 = plVar12;
                  }
                  plVar11 = (long *)plVar11[2];
                  local_950[3] = (long)plVar11;
                  local_950 = plVar11;
                }
                plVar11 = (long *)plVar21[1];
                plVar12 = local_950;
                if (plVar11 != (long *)0x0) {
                  plVar12 = (long *)*plVar11;
                  if (plVar12 != (long *)0x0) {
                    if (*plVar12 != 0) {
                      FUN_0010b420(*plVar12,&local_950);
                    }
                    if (plVar12[1] != 0) {
                      FUN_0010b420(plVar12[1],&local_950);
                    }
                    plVar12 = (long *)plVar12[2];
                    local_950[3] = (long)plVar12;
                    local_950 = plVar12;
                  }
                  plVar12 = (long *)plVar11[1];
                  plVar27 = local_950;
                  if (plVar12 != (long *)0x0) {
                    if (*plVar12 != 0) {
                      FUN_0010b420(*plVar12,&local_950);
                    }
                    if (plVar12[1] != 0) {
                      FUN_0010b420(plVar12[1],&local_950);
                    }
                    plVar27 = (long *)plVar12[2];
                    local_950[3] = (long)plVar27;
                  }
                  plVar12 = (long *)plVar11[2];
                  plVar27[3] = (long)plVar12;
                }
                local_950 = (long *)plVar21[2];
                plVar12[3] = (long)local_950;
              }
              lVar16 = plVar22[2];
              lVar26 = plVar31[5];
              local_950[3] = lVar16;
              local_950 = (long *)puVar28[2];
              *(long **)(lVar16 + 0x18) = local_950;
              if (plVar32 != (long *)0x0) goto LAB_0010d613;
              goto LAB_0010d923;
            }
            plVar21 = (long *)puVar28[2];
            lVar26 = plVar31[5];
            local_950[3] = (long)plVar21;
            local_950 = plVar21;
          }
          else {
            plVar18 = (long *)*plVar32;
            if (plVar18 != (long *)0x0) {
              plVar22 = (long *)*plVar18;
              if (plVar22 != (long *)0x0) {
                plVar11 = (long *)*plVar22;
                if (plVar11 != (long *)0x0) {
                  if (*plVar11 != 0) {
                    FUN_0010b420(*plVar11,&local_950,plVar21);
                    plVar21 = extraout_RDX;
                  }
                  if (plVar11[1] != 0) {
                    FUN_0010b420(plVar11[1],&local_950,plVar21);
                  }
                  plVar21 = (long *)plVar11[2];
                  local_950[3] = (long)plVar21;
                  local_950 = plVar21;
                }
                plVar21 = (long *)plVar22[1];
                plVar11 = local_950;
                if (plVar21 != (long *)0x0) {
                  if (*plVar21 != 0) {
                    FUN_0010b420(*plVar21,&local_950);
                  }
                  if (plVar21[1] != 0) {
                    FUN_0010b420(plVar21[1],&local_950);
                  }
                  plVar11 = (long *)plVar21[2];
                  local_950[3] = (long)plVar11;
                }
                local_950 = (long *)plVar22[2];
                plVar11[3] = (long)local_950;
              }
              plVar21 = (long *)plVar18[1];
              if (plVar21 != (long *)0x0) {
                plVar22 = (long *)*plVar21;
                if (plVar22 != (long *)0x0) {
                  if (*plVar22 != 0) {
                    FUN_0010b420(*plVar22,&local_950);
                  }
                  if (plVar22[1] != 0) {
                    FUN_0010b420(plVar22[1],&local_950);
                  }
                  plVar22 = (long *)plVar22[2];
                  local_950[3] = (long)plVar22;
                  local_950 = plVar22;
                }
                plVar22 = (long *)plVar21[1];
                plVar11 = local_950;
                if (plVar22 != (long *)0x0) {
                  if (*plVar22 != 0) {
                    FUN_0010b420(*plVar22,&local_950);
                  }
                  if (plVar22[1] != 0) {
                    FUN_0010b420(plVar22[1],&local_950);
                  }
                  plVar11 = (long *)plVar22[2];
                  local_950[3] = (long)plVar11;
                }
                local_950 = (long *)plVar21[2];
                plVar11[3] = (long)local_950;
              }
              plVar21 = (long *)plVar18[2];
              local_950[3] = (long)plVar21;
              local_950 = plVar21;
            }
            plVar18 = (long *)plVar32[1];
            plVar21 = local_950;
            if (plVar18 != (long *)0x0) {
              plVar21 = (long *)*plVar18;
              if (plVar21 != (long *)0x0) {
                plVar22 = (long *)*plVar21;
                if (plVar22 != (long *)0x0) {
                  if (*plVar22 != 0) {
                    FUN_0010b420(*plVar22,&local_950);
                  }
                  if (plVar22[1] != 0) {
                    FUN_0010b420(plVar22[1],&local_950);
                  }
                  plVar22 = (long *)plVar22[2];
                  local_950[3] = (long)plVar22;
                  local_950 = plVar22;
                }
                plVar22 = (long *)plVar21[1];
                plVar11 = local_950;
                if (plVar22 != (long *)0x0) {
                  if (*plVar22 != 0) {
                    FUN_0010b420(*plVar22,&local_950);
                  }
                  if (plVar22[1] != 0) {
                    FUN_0010b420(plVar22[1],&local_950);
                  }
                  plVar11 = (long *)plVar22[2];
                  local_950[3] = (long)plVar11;
                }
                local_950 = (long *)plVar21[2];
                plVar11[3] = (long)local_950;
              }
              plVar21 = (long *)plVar18[1];
              if (plVar21 != (long *)0x0) {
                plVar22 = (long *)*plVar21;
                if (plVar22 != (long *)0x0) {
                  if (*plVar22 != 0) {
                    FUN_0010b420(*plVar22,&local_950);
                  }
                  if (plVar22[1] != 0) {
                    FUN_0010b420(plVar22[1],&local_950);
                  }
                  plVar22 = (long *)plVar22[2];
                  local_950[3] = (long)plVar22;
                  local_950 = plVar22;
                }
                plVar22 = (long *)plVar21[1];
                plVar11 = local_950;
                if (plVar22 != (long *)0x0) {
                  if (*plVar22 != 0) {
                    FUN_0010b420(*plVar22,&local_950);
                  }
                  if (plVar22[1] != 0) {
                    FUN_0010b420(plVar22[1],&local_950);
                  }
                  plVar11 = (long *)plVar22[2];
                  local_950[3] = (long)plVar11;
                }
                local_950 = (long *)plVar21[2];
                plVar11[3] = (long)local_950;
              }
              plVar21 = (long *)plVar18[2];
              local_950[3] = (long)plVar21;
            }
            plVar18 = (long *)plVar32[2];
            plVar22 = (long *)puVar28[1];
            plVar21[3] = (long)plVar18;
            local_950 = plVar18;
            if (plVar22 != (long *)0x0) goto LAB_0010d327;
            local_950 = (long *)puVar28[2];
            lVar26 = plVar31[5];
            plVar18[3] = (long)local_950;
LAB_0010d613:
            plVar21 = (long *)*plVar32;
            bVar37 = (byte)lVar26;
            if (plVar21 != (long *)0x0) {
              plVar18 = (long *)*plVar21;
              if (plVar18 != (long *)0x0) {
                puVar30 = (undefined8 *)*plVar18;
                if (puVar30 != (undefined8 *)0x0) {
                  FUN_0010c100(*puVar30,lVar26,puVar36);
                  FUN_0010c100(puVar30[1],lVar26,puVar36);
                  if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36)
                     ) {
                    *(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36) = bVar37;
                  }
                }
                puVar30 = (undefined8 *)plVar18[1];
                if (puVar30 != (undefined8 *)0x0) {
                  FUN_0010c100(*puVar30,lVar26,puVar36);
                  FUN_0010c100(puVar30[1],lVar26,puVar36);
                  if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36)
                     ) {
                    *(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36) = bVar37;
                  }
                }
                if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(plVar18 + 3) + (long)puVar36))
                {
                  *(byte *)((ulong)*(byte *)(plVar18 + 3) + (long)puVar36) = bVar37;
                }
              }
              plVar18 = (long *)plVar21[1];
              if (plVar18 != (long *)0x0) {
                puVar30 = (undefined8 *)*plVar18;
                if (puVar30 != (undefined8 *)0x0) {
                  FUN_0010c100(*puVar30,lVar26,puVar36);
                  FUN_0010c100(puVar30[1],lVar26,puVar36);
                  if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36)
                     ) {
                    *(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36) = bVar37;
                  }
                }
                puVar30 = (undefined8 *)plVar18[1];
                if (puVar30 != (undefined8 *)0x0) {
                  FUN_0010c100(*puVar30,lVar26,puVar36);
                  FUN_0010c100(puVar30[1],lVar26,puVar36);
                  if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36)
                     ) {
                    *(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36) = bVar37;
                  }
                }
                if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(plVar18 + 3) + (long)puVar36))
                {
                  *(byte *)((ulong)*(byte *)(plVar18 + 3) + (long)puVar36) = bVar37;
                }
              }
              if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(plVar21 + 3) + (long)puVar36)) {
                *(byte *)((ulong)*(byte *)(plVar21 + 3) + (long)puVar36) = bVar37;
              }
            }
            plVar21 = (long *)plVar32[1];
            if (plVar21 != (long *)0x0) {
              plVar18 = (long *)*plVar21;
              if (plVar18 != (long *)0x0) {
                puVar30 = (undefined8 *)*plVar18;
                if (puVar30 != (undefined8 *)0x0) {
                  FUN_0010c100(*puVar30,lVar26,puVar36);
                  FUN_0010c100(puVar30[1],lVar26,puVar36);
                  if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36)
                     ) {
                    *(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36) = bVar37;
                  }
                }
                puVar30 = (undefined8 *)plVar18[1];
                if (puVar30 != (undefined8 *)0x0) {
                  FUN_0010c100(*puVar30,lVar26,puVar36);
                  FUN_0010c100(puVar30[1],lVar26,puVar36);
                  if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36)
                     ) {
                    *(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36) = bVar37;
                  }
                }
                if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(plVar18 + 3) + (long)puVar36))
                {
                  *(byte *)((ulong)*(byte *)(plVar18 + 3) + (long)puVar36) = bVar37;
                }
              }
              plVar18 = (long *)plVar21[1];
              if (plVar18 != (long *)0x0) {
                puVar30 = (undefined8 *)*plVar18;
                if (puVar30 != (undefined8 *)0x0) {
                  FUN_0010c100(*puVar30,lVar26,puVar36);
                  FUN_0010c100(puVar30[1],lVar26,puVar36);
                  if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36)
                     ) {
                    *(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36) = bVar37;
                  }
                }
                puVar30 = (undefined8 *)plVar18[1];
                if (puVar30 != (undefined8 *)0x0) {
                  FUN_0010c100(*puVar30,lVar26,puVar36);
                  FUN_0010c100(puVar30[1],lVar26,puVar36);
                  if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36)
                     ) {
                    *(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36) = bVar37;
                  }
                }
                if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(plVar18 + 3) + (long)puVar36))
                {
                  *(byte *)((ulong)*(byte *)(plVar18 + 3) + (long)puVar36) = bVar37;
                }
              }
              if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(plVar21 + 3) + (long)puVar36)) {
                *(byte *)((ulong)*(byte *)(plVar21 + 3) + (long)puVar36) = bVar37;
              }
            }
            if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(plVar32 + 3) + (long)puVar36)) {
              *(byte *)((ulong)*(byte *)(plVar32 + 3) + (long)puVar36) = bVar37;
            }
            plVar22 = (long *)puVar28[1];
            if (plVar22 != (long *)0x0) {
              plVar18 = (long *)*plVar22;
LAB_0010d923:
              bVar37 = (byte)lVar26;
              if (plVar18 != (long *)0x0) {
                plVar21 = (long *)*plVar18;
                if (plVar21 != (long *)0x0) {
                  puVar30 = (undefined8 *)*plVar21;
                  if (puVar30 != (undefined8 *)0x0) {
                    FUN_0010c100(*puVar30,lVar26,puVar36);
                    FUN_0010c100(puVar30[1],lVar26,puVar36);
                    if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(puVar30 + 3) +
                                                       (long)puVar36)) {
                      *(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36) = bVar37;
                    }
                  }
                  puVar30 = (undefined8 *)plVar21[1];
                  if (puVar30 != (undefined8 *)0x0) {
                    FUN_0010c100(*puVar30,lVar26,puVar36);
                    FUN_0010c100(puVar30[1],lVar26,puVar36);
                    if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(puVar30 + 3) +
                                                       (long)puVar36)) {
                      *(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36) = bVar37;
                    }
                  }
                  if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(plVar21 + 3) + (long)puVar36)
                     ) {
                    *(byte *)((ulong)*(byte *)(plVar21 + 3) + (long)puVar36) = bVar37;
                  }
                }
                plVar21 = (long *)plVar18[1];
                if (plVar21 != (long *)0x0) {
                  puVar30 = (undefined8 *)*plVar21;
                  if (puVar30 != (undefined8 *)0x0) {
                    FUN_0010c100(*puVar30,lVar26,puVar36);
                    FUN_0010c100(puVar30[1],lVar26,puVar36);
                    if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(puVar30 + 3) +
                                                       (long)puVar36)) {
                      *(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36) = bVar37;
                    }
                  }
                  puVar30 = (undefined8 *)plVar21[1];
                  if (puVar30 != (undefined8 *)0x0) {
                    FUN_0010c100(*puVar30,lVar26,puVar36);
                    FUN_0010c100(puVar30[1],lVar26,puVar36);
                    if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(puVar30 + 3) +
                                                       (long)puVar36)) {
                      *(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36) = bVar37;
                    }
                  }
                  if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(plVar21 + 3) + (long)puVar36)
                     ) {
                    *(byte *)((ulong)*(byte *)(plVar21 + 3) + (long)puVar36) = bVar37;
                  }
                }
                if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(plVar18 + 3) + (long)puVar36))
                {
                  *(byte *)((ulong)*(byte *)(plVar18 + 3) + (long)puVar36) = bVar37;
                }
              }
              plVar21 = (long *)plVar22[1];
              if (plVar21 != (long *)0x0) {
                plVar32 = (long *)*plVar21;
                if (plVar32 != (long *)0x0) {
                  puVar30 = (undefined8 *)*plVar32;
                  if (puVar30 != (undefined8 *)0x0) {
                    FUN_0010c100(*puVar30,lVar26,puVar36);
                    FUN_0010c100(puVar30[1],lVar26,puVar36);
                    if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(puVar30 + 3) +
                                                       (long)puVar36)) {
                      *(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36) = bVar37;
                    }
                  }
                  puVar30 = (undefined8 *)plVar32[1];
                  if (puVar30 != (undefined8 *)0x0) {
                    FUN_0010c100(*puVar30,lVar26,puVar36);
                    FUN_0010c100(puVar30[1],lVar26,puVar36);
                    if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(puVar30 + 3) +
                                                       (long)puVar36)) {
                      *(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36) = bVar37;
                    }
                  }
                  if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(plVar32 + 3) + (long)puVar36)
                     ) {
                    *(byte *)((ulong)*(byte *)(plVar32 + 3) + (long)puVar36) = bVar37;
                  }
                }
                plVar32 = (long *)plVar21[1];
                if (plVar32 != (long *)0x0) {
                  puVar30 = (undefined8 *)*plVar32;
                  if (puVar30 != (undefined8 *)0x0) {
                    FUN_0010c100(*puVar30,lVar26,puVar36);
                    FUN_0010c100(puVar30[1],lVar26,puVar36);
                    if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(puVar30 + 3) +
                                                       (long)puVar36)) {
                      *(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36) = bVar37;
                    }
                  }
                  puVar30 = (undefined8 *)plVar32[1];
                  if (puVar30 != (undefined8 *)0x0) {
                    FUN_0010c100(*puVar30,lVar26,puVar36);
                    FUN_0010c100(puVar30[1],lVar26,puVar36);
                    if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(puVar30 + 3) +
                                                       (long)puVar36)) {
                      *(byte *)((ulong)*(byte *)(puVar30 + 3) + (long)puVar36) = bVar37;
                    }
                  }
                  if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(plVar32 + 3) + (long)puVar36)
                     ) {
                    *(byte *)((ulong)*(byte *)(plVar32 + 3) + (long)puVar36) = bVar37;
                  }
                }
                if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(plVar21 + 3) + (long)puVar36))
                {
                  *(byte *)((ulong)*(byte *)(plVar21 + 3) + (long)puVar36) = bVar37;
                }
              }
              if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(plVar22 + 3) + (long)puVar36)) {
                *(byte *)((ulong)*(byte *)(plVar22 + 3) + (long)puVar36) = bVar37;
              }
            }
          }
          if (lVar26 < (long)(ulong)*(byte *)((ulong)*(byte *)(puVar28 + 3) + (long)puVar36)) {
            *(byte *)((ulong)*(byte *)(puVar28 + 3) + (long)puVar36) = (byte)lVar26;
          }
          lVar16 = plVar31[1];
        }
        lVar26 = param_1[0xc];
        FUN_0010ab10(lVar16,plVar31[4],lVar26,lVar15 == 1);
        if (lVar15 == 1) {
          lVar17 = param_1[0xd];
          lVar23 = plVar31[4];
          plVar31[6] = lVar17;
          plVar31[7] = lVar17;
          if (lVar23 != 0) {
            lVar17 = *plVar31;
            do {
              cVar8 = FUN_0010abd0(*(undefined8 *)(lVar23 + 8),lVar16);
              if ((cVar8 == '\0') &&
                 (lVar13 = plVar31[5] - *(long *)(lVar23 + 0x28), lVar13 < *(long *)(lVar23 + 0x30))
                 ) {
                *(long *)(lVar23 + 0x30) = lVar13;
              }
              if ((lVar17 != 0) &&
                 (lVar13 = plVar31[5] - *(long *)(lVar23 + 0x28), lVar13 < *(long *)(lVar23 + 0x38))
                 ) {
                *(long *)(lVar23 + 0x38) = lVar13;
              }
              lVar23 = *(long *)(lVar23 + 0x20);
            } while (lVar23 != 0);
            plVar31 = (long *)plVar31[3];
            plVar21 = extraout_RDX_02;
            if (plVar31 == (long *)0x0) break;
            goto LAB_0010dc63;
          }
        }
        plVar31 = (long *)plVar31[3];
        plVar21 = extraout_RDX_01;
      } while (plVar31 != (long *)0x0);
      if (lVar15 == 1) goto LAB_0010e1bb;
    }
  }
  uVar10 = 0xffffffff;
  puVar28 = local_948;
  if (lVar1 == 0) {
    puVar28 = param_1 + 0x2e;
  }
  uVar34 = 0xffffffff;
  *puVar28 = 0;
  puVar28[0xff] = 0;
  puVar30 = (undefined8 *)((ulong)(puVar28 + 1) & 0xfffffffffffffff8);
  for (uVar19 = (ulong)(((int)puVar28 -
                        (int)(undefined8 *)((ulong)(puVar28 + 1) & 0xfffffffffffffff8)) + 0x800U >>
                       3); uVar19 != 0; uVar19 = uVar19 - 1) {
    *puVar30 = 0;
    puVar30 = puVar30 + (ulong)bVar38 * -2 + 1;
  }
  FUN_0010ba10(*(undefined8 *)(param_1[0xc] + 8),puVar28);
  uVar14 = 0;
  uVar19 = 0xfffffffe;
  do {
    while( true ) {
      iVar9 = (int)uVar14;
      uVar29 = uVar14 & 0xffffffff;
      if (lVar1 == 0) {
        lVar26 = param_1[uVar14 + 0x2e];
        uVar20 = uVar14 & 0xffffffff;
      }
      else {
        uVar20 = (ulong)*(byte *)(lVar1 + uVar14);
        lVar26 = puVar28[uVar20];
        param_1[uVar14 + 0x2e] = lVar26;
      }
      iVar25 = (int)uVar20;
      uVar33 = uVar10;
      if (lVar26 != 0) break;
LAB_0010ddd0:
      iVar25 = (int)uVar19;
      iVar9 = (int)uVar33;
      uVar14 = uVar14 + 1;
      uVar10 = uVar33;
      if (uVar14 == 0x100) goto LAB_0010de21;
    }
    iVar24 = (int)uVar19;
    if (iVar24 != -2) {
      uVar33 = uVar29;
      if (iVar25 == iVar24) {
        if (iVar24 != (int)uVar10) {
          uVar33 = (ulong)uVar34;
        }
      }
      else if ((iVar25 != iVar9) || (iVar24 != (int)uVar10)) {
        uVar19 = 0xffffffff;
        uVar33 = uVar10;
      }
      goto LAB_0010ddd0;
    }
    uVar14 = uVar14 + 1;
    uVar19 = uVar20;
    uVar10 = uVar29;
  } while (uVar14 != 0x100);
LAB_0010de21:
  *(int *)(param_1 + 0x131) = iVar25;
  *(int *)((long)param_1 + 0x98c) = iVar9;
  if (lVar15 != 1) goto LAB_0010de3a;
  uVar10 = param_1[4];
  uVar19 = param_1[0xd];
  if (uVar10 - param_1[3] < uVar19) {
    _obstack_newchunk(param_1);
    lVar15 = param_1[2];
    uVar10 = param_1[4];
    uVar14 = param_1[0xd];
    lVar26 = param_1[3] + uVar19;
    if (lVar26 == lVar15) goto LAB_0010e39e;
  }
  else {
    lVar15 = param_1[2];
    lVar26 = param_1[3] + uVar19;
    uVar14 = uVar19;
    if (lVar26 == lVar15) {
LAB_0010e39e:
      *(byte *)(param_1 + 10) = *(byte *)(param_1 + 10) | 2;
    }
  }
  param_1[0x12e] = lVar15;
  uVar19 = lVar26 + param_1[6] & ~param_1[6];
  if (uVar19 - param_1[1] <= uVar10 - param_1[1]) {
    uVar10 = uVar19;
  }
  param_1[3] = uVar10;
  param_1[2] = uVar10;
  lVar26 = param_1[0xc];
  if (0 < (long)uVar14) {
    while( true ) {
      uVar14 = uVar14 - 1;
      *(undefined1 *)(lVar15 + uVar14) = *(undefined1 *)(*(long *)(lVar26 + 8) + 0x18);
      lVar26 = *(long *)(lVar26 + 0x18);
      if (uVar14 == 0) break;
      lVar15 = param_1[0x12e];
    }
    lVar15 = param_1[0xd];
    if (1 < lVar15) {
      uVar19 = param_1[4];
      lVar26 = param_1[3];
      uVar10 = lVar15 * 8 - 8;
      if (uVar19 - lVar26 < uVar10) {
        _obstack_newchunk(param_1,uVar10);
        lVar26 = param_1[3];
        uVar19 = param_1[4];
        lVar15 = param_1[0xd];
      }
      lVar16 = param_1[2];
      if (lVar26 + uVar10 == lVar16) {
        *(byte *)(param_1 + 10) = *(byte *)(param_1 + 10) | 2;
      }
      param_1[0x12f] = lVar16;
      uVar10 = lVar26 + uVar10 + param_1[6] & ~param_1[6];
      if (uVar10 - param_1[1] <= uVar19 - param_1[1]) {
        uVar19 = uVar10;
      }
      param_1[3] = uVar19;
      param_1[2] = uVar19;
      lVar26 = *(long *)(param_1[0xc] + 0x18);
      if (1 < lVar15) {
        lVar17 = 0;
        do {
          puVar28 = (undefined8 *)(lVar26 + 0x30);
          lVar26 = *(long *)(lVar26 + 0x18);
          *(undefined8 *)(lVar16 + lVar17 * 8) = *puVar28;
          lVar15 = param_1[0xd];
          lVar17 = lVar17 + 1;
        } while (lVar17 < lVar15 + -1);
      }
      bVar38 = *(byte *)(param_1[0x12e] + -2 + lVar15);
      if (lVar1 != 0) {
        bVar38 = *(byte *)(lVar1 + (ulong)bVar38);
      }
      *(byte *)(param_1 + 0x132) = bVar38;
    }
  }
LAB_0010de3a:
  lVar15 = 0;
  if (lVar1 != 0) {
    do {
      *(undefined1 *)((long)param_1 + lVar15 + 0x70) =
           *(undefined1 *)((long)puVar36 + (ulong)*(byte *)(lVar1 + lVar15));
      lVar15 = lVar15 + 1;
    } while (lVar15 != 0x100);
  }
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: kwsexec @ 0xe3e0

void kwsexec(long param_1)

{
                    /* WARNING: Could not recover jumptable at 0x0010e3ef. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (**(code **)(param_1 + 0x998))();
  return;
}




// Function: wordchars_count @ 0xe420

long wordchars_count(long param_1,long param_2,char param_3)

{
  byte bVar1;
  int iVar2;
  long lVar3;
  long lVar4;
  long in_FS_OFFSET;
  wint_t local_4c;
  undefined8 local_48;
  long local_40;
  
  lVar4 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_48 = 0;
  do {
    if (param_2 - param_1 <= lVar4) break;
    bVar1 = *(byte *)(param_1 + lVar4);
    if ((&DAT_0012b9c0)[bVar1] == '\0') {
      if ((&DAT_0012b183)[bVar1] != -2) break;
      local_4c = 0;
      lVar3 = FUN_0011acc0(&local_4c,(byte *)(param_1 + lVar4),(param_2 - param_1) - lVar4,&local_48
                          );
      if (local_4c != 0x5f) {
        iVar2 = iswalnum(local_4c);
        if (iVar2 == 0) break;
      }
      lVar4 = lVar4 + lVar3 + (ulong)(lVar3 == 0);
    }
    else {
      lVar4 = lVar4 + 1;
    }
  } while (param_3 != '\0');
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: wordinit @ 0xe570

void wordinit(void)

{
  int iVar1;
  wint_t *pwVar2;
  undefined1 *puVar3;
  bool bVar4;
  
  puVar3 = &DAT_0012b9c0;
  pwVar2 = &DAT_0012b284;
  do {
    bVar4 = true;
    if (*pwVar2 != 0x5f) {
      iVar1 = iswalnum(*pwVar2);
      bVar4 = iVar1 != 0;
    }
    pwVar2 = pwVar2 + 1;
    *puVar3 = bVar4;
    puVar3 = puVar3 + 1;
  } while (pwVar2 != (wint_t *)&DAT_0012b684);
  return;
}




// Function: kwsinit @ 0xe5c0

void kwsinit(char param_1)

{
  __int32_t *p_Var1;
  size_t sVar2;
  long lVar3;
  __int32_t **pp_Var4;
  long lVar5;
  
  if (DAT_0012b687 == '\0') {
    FUN_0010c880(0);
    return;
  }
  sVar2 = __ctype_get_mb_cur_max();
  if ((sVar2 != 1) && (param_1 == '\0')) {
    FUN_0010c880(0);
    return;
  }
  lVar3 = FUN_0011ef30(0x100);
  pp_Var4 = __ctype_toupper_loc();
  p_Var1 = *pp_Var4;
  lVar5 = 0;
  do {
    *(char *)(lVar3 + lVar5) = (char)p_Var1[lVar5];
    lVar5 = lVar5 + 1;
  } while (lVar5 != 0x100);
  FUN_0010c880();
  return;
}




// Function: mb_goback @ 0xe640

long mb_goback(undefined8 *param_1,ulong *param_2,byte *param_3,long param_4)

{
  uint uVar1;
  byte *pbVar2;
  long lVar3;
  ulong uVar4;
  byte *pbVar5;
  byte *pbVar6;
  long in_FS_OFFSET;
  undefined8 local_48;
  long local_40;
  
  pbVar5 = (byte *)*param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (pbVar5 < param_3) {
    if (DAT_0012b182 == '\0') {
      local_48 = 0;
      do {
        pbVar6 = pbVar5;
        uVar4 = (ulong)(char)(&DAT_0012b183)[*pbVar6];
        if ((((&DAT_0012b183)[*pbVar6] == -2) &&
            (uVar4 = FUN_0011ac90(pbVar6,param_4 - (long)pbVar6,&local_48), uVar4 < 0x11)) ||
           (-1 < (long)uVar4)) {
          pbVar2 = pbVar6 + uVar4;
        }
        else {
          local_48 = 0;
          uVar4 = 1;
          pbVar2 = pbVar6 + 1;
        }
        pbVar5 = pbVar2;
      } while (pbVar2 < param_3);
      pbVar5 = pbVar6;
      if (param_2 != (ulong *)0x0) {
        *param_2 = uVar4;
      }
LAB_0010e6db:
      *param_1 = pbVar2;
      if (pbVar2 != param_3) goto LAB_0010e6e4;
    }
    else {
      if (((int)(char)*param_3 & 0xc0U) == 0x80) {
        uVar1 = (uint)(char)param_3[-1];
        if ((uVar1 & 0xc0) == 0x80) {
          uVar1 = (uint)(char)param_3[-2];
          if ((uVar1 & 0xc0) == 0x80) {
            uVar1 = (uint)(char)param_3[-3];
            if ((uVar1 & 0xc0) == 0x80) goto LAB_0010e71f;
            lVar3 = 3;
          }
          else {
            lVar3 = 2;
          }
        }
        else {
          lVar3 = 1;
        }
        if ((int)(~uVar1 & 0xff) >> (7U - (char)lVar3 & 0x1f) == 0) {
          local_48 = 0;
          pbVar6 = param_3 + -lVar3;
          uVar4 = FUN_0011ac90(pbVar6,param_4 - (long)pbVar6,&local_48);
          if ((uVar4 < 0x11) || (pbVar2 = param_3, -1 < (long)uVar4)) {
            pbVar2 = pbVar6 + uVar4;
            pbVar5 = pbVar6;
          }
          goto LAB_0010e6db;
        }
      }
LAB_0010e71f:
      *param_1 = param_3;
    }
    lVar3 = 0;
  }
  else {
LAB_0010e6e4:
    lVar3 = (long)param_3 - (long)pbVar5;
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: wordchars_size @ 0xe810

void wordchars_size(undefined8 param_1,undefined8 param_2)

{
  FUN_0010e420(param_1,param_2,1);
  return;
}




// Function: wordchar_prev @ 0xe830

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
    if ((DAT_0012b180 != '\0') && ((~(*(byte *)(param_2 + -1) >> 7) & DAT_0012b182) == 0)) {
      local_28 = param_1;
      lVar1 = FUN_0010e640(&local_28,0,param_2 + -1,param_3);
      if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
        uVar2 = FUN_0010e420((param_2 + -1) - lVar1,param_3,0);
        return uVar2;
      }
      goto LAB_0010e8e4;
    }
    uVar2 = (ulong)(byte)(&DAT_0012b9c0)[*(byte *)(param_2 + -1)];
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
LAB_0010e8e4:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: private_malloc @ 0xe900

void private_malloc(long param_1)

{
  if (-1 < param_1) {
    FUN_0011ef30();
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_0011f410();
}




// Function: jit_exec @ 0xe920

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
      if (iVar1 != -0x35) goto LAB_0010e9c6;
      pcre2_config_8(7,&local_44,uStack_60);
      bVar4 = local_44 < 0;
      local_44 = local_44 * 2;
      if (bVar4) goto LAB_0010e9c6;
      lVar2 = param_1[2];
      if (lVar2 == 0) {
        lVar2 = pcre2_match_context_create_8(*param_1);
        param_1[2] = lVar2;
      }
      uStack_60 = 0x10e988;
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
      FUN_0011f410();
    }
    lVar3 = param_1[2];
    if (lVar3 == 0) {
      lVar3 = pcre2_match_context_create_8(*param_1);
      lVar2 = param_1[4];
      param_1[2] = lVar3;
    }
    uStack_60 = 0x10ea52;
    pcre2_jit_stack_assign_8(lVar3,0,lVar2);
  }
LAB_0010e9c6:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return iVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: Pcompile @ 0xea80

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
  bVar10 = DAT_0012b687 == '\0';
  puVar2 = (undefined8 *)FUN_0011ef30(0x38);
  uVar9 = (-(uint)bVar10 & 0xfffffff8) + 0x18;
  uVar3 = pcre2_general_context_create_8(FUN_0010e900,free,0);
  *puVar2 = uVar3;
  uVar4 = pcre2_compile_context_create_8(uVar3);
  if (DAT_0012b180 != '\0') {
    if (DAT_0012b182 == '\0') {
      uVar3 = dcgettext(0,"-P supports only unibyte and UTF-8 locales",5);
                    /* WARNING: Subroutine does not return */
      error(2,0,uVar3);
    }
    uVar9 = uVar9 | 0x4080000;
  }
  pcVar5 = rawmemchr(param_1,10);
  if (param_1 + param_2 == pcVar5) {
    local_168 = param_2;
    if (DAT_0012b685 == '\0') {
      pcVar5 = (char *)0x0;
      if (DAT_0012b686 != '\0') {
        local_168 = param_2 + 0x11;
        pcVar5 = (char *)FUN_0011ef30(local_168);
        *(undefined8 *)pcVar5 = s___<__w)(_:invalid_argument__s_fo_00122990._0_8_;
        *(undefined2 *)(pcVar5 + 8) = s___<__w)(_:invalid_argument__s_fo_00122990._8_2_;
        puVar8 = mempcpy(pcVar5 + 10,param_1,param_2);
        *puVar8 = DAT_00122988;
        *(undefined2 *)(puVar8 + 1) = DAT_0012298c;
        *(undefined1 *)((long)puVar8 + 6) = DAT_0012298e;
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
        uVar1 = FUN_0010e920(puVar2,"",0,0,1);
        *(undefined4 *)(puVar2 + 6) = uVar1;
        uVar1 = FUN_0010e920(puVar2,"",0,0,0);
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
    error(2,0,&DAT_001229f1,local_148);
  }
  uVar3 = dcgettext(0,"the -P option only supports a single pattern",5);
                    /* WARNING: Subroutine does not return */
  error(2,0,uVar3);
}




// Function: Pexecute @ 0xeda0

long Pexecute(long param_1,byte *param_2,long param_3,long *param_4,byte *param_5)

{
  int iVar1;
  long *plVar2;
  byte *pbVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  long lVar6;
  byte *__s;
  uint uVar7;
  byte *pbVar8;
  byte *local_68;
  
  __s = param_2;
  if (param_5 != (byte *)0x0) {
    __s = param_5;
  }
  uVar7 = (uint)(__s[-1] == DAT_0012b684);
  plVar2 = (long *)pcre2_get_ovector_pointer_8(*(undefined8 *)(param_1 + 0x18));
  local_68 = param_2;
  while( true ) {
    pbVar3 = rawmemchr(__s,(int)(char)DAT_0012b684);
    if ((&DAT_0012b183)[*__s] == -1) {
      do {
        pbVar8 = __s + 1;
        __s = __s + 1;
      } while ((&DAT_0012b183)[*pbVar8] == -1);
      lVar6 = 0;
      uVar7 = 0;
      pbVar8 = __s;
    }
    else {
      lVar6 = (long)__s - (long)local_68;
      pbVar8 = local_68;
    }
    if (pbVar3 == __s) {
      iVar1 = *(int *)(param_1 + 0x30 + (ulong)uVar7 * 4);
      plVar2[1] = lVar6;
      *plVar2 = lVar6;
    }
    else {
      iVar1 = FUN_0010e920(param_1,pbVar8,(long)pbVar3 - (long)pbVar8,lVar6,uVar7 ^ 1);
    }
    if (iVar1 != -1) break;
    __s = pbVar3 + 1;
    uVar7 = 1;
    local_68 = __s;
    if (param_2 + param_3 <= __s) {
      return -1;
    }
  }
  if (iVar1 < 1) {
    if ((iVar1 < -0x2d) && (-0x40 < iVar1)) {
      switch(iVar1) {
      case -0x3f:
        uVar4 = FUN_001090f0();
        uVar5 = dcgettext(0,"%s: exceeded PCRE\'s heap limit",5);
                    /* WARNING: Subroutine does not return */
        error(2,0,uVar5,uVar4);
      case -0x35:
        uVar4 = FUN_001090f0();
        uVar5 = dcgettext(0,"%s: exceeded PCRE\'s nested backtracking limit",5);
                    /* WARNING: Subroutine does not return */
        error(2,0,uVar5,uVar4);
      case -0x34:
        uVar4 = FUN_001090f0();
        uVar5 = dcgettext(0,"%s: PCRE detected recurse loop",5);
                    /* WARNING: Subroutine does not return */
        error(2,0,uVar5,uVar4);
      case -0x30:
        uVar4 = FUN_001090f0();
        uVar5 = dcgettext(0,"%s: memory exhausted",5);
                    /* WARNING: Subroutine does not return */
        error(2,0,uVar5,uVar4);
      case -0x2f:
        uVar4 = FUN_001090f0();
        uVar5 = dcgettext(0,"%s: exceeded PCRE\'s backtracking limit",5);
                    /* WARNING: Subroutine does not return */
        error(2,0,uVar5,uVar4);
      case -0x2e:
        uVar4 = FUN_001090f0();
        uVar5 = dcgettext(0,"%s: exhausted PCRE JIT stack",5);
                    /* WARNING: Subroutine does not return */
        error(2,0,uVar5,uVar4);
      }
    }
    uVar4 = FUN_001090f0();
    uVar5 = dcgettext(0,"%s: internal PCRE error: %d",5);
                    /* WARNING: Subroutine does not return */
    error(2,0,uVar5,uVar4,iVar1);
  }
  pbVar3 = pbVar3 + 1;
  if (param_5 != (byte *)0x0) {
    local_68 = pbVar8 + *plVar2;
    pbVar3 = pbVar8 + plVar2[1];
  }
  *param_4 = (long)pbVar3 - (long)local_68;
  return (long)local_68 - (long)param_2;
}




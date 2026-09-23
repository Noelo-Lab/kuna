// Function: main @ 0x7240

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
    puVar6 = &DAT_00128132;
  }
  FUN_00121e40(puVar6);
  FUN_0010da20();
  DAT_00138618 = 0;
  DAT_00138604 = 0;
  DAT_00138608 = 0;
  DAT_001385f8 = 0xffffff9c;
  cVar1 = FUN_00116220();
  if (cVar1 != '\0') {
    FUN_00115fd0();
  }
  DAT_00138610 = FUN_0010ea90("w");
  if (DAT_00138610 != 0) {
    FUN_0010e630(&DAT_00138620);
    setlocale(6,"");
    bindtextdomain("findutils","/usr/local/share/locale");
    textdomain("findutils");
    iVar2 = FUN_00127a00(FUN_00117a40);
    if (iVar2 == 0) {
      iVar2 = FUN_0010e180(param_1,param_2);
      if ((DAT_00138660 & 2) != 0) {
        DAT_00138670 = FUN_0010dec0;
      }
      if ((DAT_00138660 & 0x80) != 0) {
        pcVar4 = ctime(&DAT_00138648);
        __fprintf_chk(stderr,1,"cur_day_start = %s",pcVar4);
      }
      uVar3 = FUN_0010c9f0(param_1,param_2,iVar2);
      cVar1 = FUN_00108070(param_1 - iVar2,param_2 + iVar2);
      if (cVar1 != '\0') {
        FUN_00109e00(uVar3);
        FUN_0010dab0();
      }
      return DAT_00138604;
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




// Function: get_fts_info_name @ 0x7510

char * get_fts_info_name(undefined4 param_1)

{
  switch(param_1) {
  default:
    __sprintf_chk(&DAT_00138598,1,0xe,&DAT_00128073,param_1);
    return &DAT_00138598;
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




// Function: consider_visiting @ 0x7670

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void consider_visiting(long param_1,long *param_2)

{
  undefined1 *puVar1;
  undefined1 uVar2;
  undefined1 uVar3;
  undefined4 uVar4;
  char cVar5;
  short sVar6;
  int iVar7;
  undefined8 uVar8;
  int *piVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  uint uVar12;
  long lVar13;
  bool bVar14;
  undefined8 uVar15;
  long in_FS_OFFSET;
  uint local_16c;
  long local_168;
  long lStack_160;
  long local_158;
  long lStack_150;
  long local_148;
  long lStack_140;
  long local_138;
  long lStack_130;
  long local_128;
  long lStack_120;
  long local_118;
  long lStack_110;
  long local_108;
  long lStack_100;
  long local_f8;
  long lStack_f0;
  long local_e8;
  long lStack_e0;
  undefined1 local_d8 [152];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (((byte)DAT_00138660 & 4) != 0) {
    uVar15 = FUN_001239f0(2,DAT_0013868c,param_2[6]);
    uVar10 = FUN_001239f0(1,DAT_0013868c,param_2[7]);
    iVar7 = DAT_00138014;
    lVar13 = param_2[0xb];
    uVar11 = FUN_00107510((short)param_2[0xd]);
    uVar8 = FUN_001239f0(0,DAT_0013868c,param_2[7]);
    __fprintf_chk(stderr,1,
                  "consider_visiting (early): %s: fts_info=%-6s, fts_level=%2d, prev_depth=%d fts_path=%s, fts_accpath=%s\n"
                  ,uVar8,uVar11,lVar13,iVar7,uVar10,uVar15);
  }
  sVar6 = (short)param_2[0xd];
  if (sVar6 == 6) {
    if ((DAT_00138018 & 0x200) != 0) {
LAB_00107908:
      if (-1 < DAT_00138010) {
        close(DAT_00138010);
        DAT_00138010 = -1;
        goto LAB_001076e0;
      }
      iVar7 = *(int *)(param_1 + 0x2c);
LAB_001076ea:
      if ((iVar7 != -100) && (iVar7 < 0)) {
        FUN_00107640();
        goto LAB_00107df1;
      }
      DAT_001385f8 = iVar7;
      if (DAT_00138010 < 0) {
        if (iVar7 == -100) {
          DAT_00138010 = -100;
          lVar13 = param_2[0xb];
          sVar6 = (short)param_2[0xd];
          goto LAB_00107713;
        }
        DAT_00138010 = FUN_00117a00();
      }
      goto LAB_0010770b;
    }
    DAT_00138014 = (int)param_2[0xb];
LAB_00107761:
    local_168 = param_2[0xe];
    lStack_160 = param_2[0xf];
    local_158 = param_2[0x10];
    lStack_150 = param_2[0x11];
    _DAT_001385e4 = 0x101;
    local_148 = param_2[0x12];
    lStack_140 = param_2[0x13];
    local_138 = param_2[0x14];
    lStack_130 = param_2[0x15];
    local_128 = param_2[0x16];
    lStack_120 = param_2[0x17];
    local_118 = param_2[0x18];
    lStack_110 = param_2[0x19];
    local_108 = param_2[0x1a];
    lStack_100 = param_2[0x1b];
    local_f8 = param_2[0x1c];
    lStack_f0 = param_2[0x1d];
    local_e8 = param_2[0x1e];
    lStack_e0 = param_2[0x1f];
    local_16c = (uint)lStack_150;
    DAT_001385e8 = (uint)lStack_150;
    if ((uint)lStack_150 == 0) {
      uVar15 = FUN_001239f0(0,DAT_0013868c,param_2[7]);
      uVar10 = dcgettext(0,"WARNING: file %s appears to have mode 0000",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar10,uVar15);
    }
    DAT_001385e0 = (undefined4)param_2[0xb];
LAB_00107808:
    DAT_001385e8 = local_16c;
    cVar5 = FUN_0010dfe0(&local_16c,param_2[7],param_2 + 0x20,&local_168,0);
    if (cVar5 == '\0') goto LAB_00107948;
    sVar6 = (short)param_2[0xd];
    if ((local_16c & 0xf000) != 0x4000) goto LAB_00107a4b;
LAB_00107847:
    if (sVar6 == 0xb) {
      FUN_0011a750(param_1,param_2,1);
      goto LAB_00107948;
    }
    uVar15 = 1;
    if (-1 < DAT_00138624) goto LAB_00107862;
LAB_0010786c:
    bVar14 = false;
LAB_0010786e:
    if (sVar6 == 1) {
      if (DAT_00138620 == '\0') goto LAB_0010792d;
      goto LAB_00107882;
    }
    if ((sVar6 != 6) || (DAT_00138620 == '\0')) goto LAB_00107882;
LAB_0010792d:
    uVar12 = (uint)CONCAT71(DAT_00138660._1_7_,(byte)DAT_00138660);
LAB_00107934:
    if ((uVar12 & 4) != 0) {
      bVar14 = true;
LAB_00107bf5:
      uVar3 = DAT_001385e5;
      uVar2 = DAT_001385e4;
      uVar10 = FUN_00107510(sVar6);
      uVar11 = FUN_001239f0(0,DAT_0013868c,param_2[7]);
      __fprintf_chk(stderr,1,
                    "consider_visiting (late): %s: fts_info=%-6s, isdir=%d ignore=%d have_stat=%d have_type=%d \n"
                    ,uVar11,uVar10,uVar15,bVar14,uVar2,uVar3);
      sVar6 = (short)param_2[0xd];
      goto LAB_001078a3;
    }
  }
  else {
    lVar13 = param_2[0xb];
    if ((DAT_00138014 < lVar13) || (lVar13 == 0)) {
      if ((DAT_00138018 & 0x200) != 0) goto LAB_00107908;
    }
    else {
LAB_001076e0:
      iVar7 = *(int *)(param_1 + 0x2c);
      if ((DAT_00138018 & 0x200) != 0) goto LAB_001076ea;
LAB_0010770b:
      lVar13 = param_2[0xb];
      sVar6 = (short)param_2[0xd];
    }
LAB_00107713:
    uVar4 = DAT_0013868c;
    lStack_160 = param_2[0xf];
    DAT_00138014 = (int)lVar13;
    if (sVar6 == 7) {
LAB_00107ba8:
      FUN_0010e8c0((int)param_2[8],param_2[7]);
      goto LAB_00107948;
    }
    if (sVar6 == 4) {
      FUN_0010e8c0((int)param_2[8],param_2[7]);
      if (DAT_00138620 != '\0') goto LAB_00107948;
      sVar6 = (short)param_2[0xd];
    }
    else {
      if (sVar6 == 2) {
        if ((*(uint *)(param_2 + 0x11) & 0xf000) == 0xa000) {
          uVar15 = FUN_0010e8a0(0,param_2[7]);
          uVar10 = dcgettext(0,
                             "Symbolic link %s is part of a loop in the directory hierarchy; we have already visited the directory to which it points."
                             ,5);
                    /* WARNING: Subroutine does not return */
          error(0,0,uVar10,uVar15);
        }
        lVar13 = *(long *)(*param_2 + 0x48);
        if (lVar13 == 0) {
          uVar15 = FUN_001239f0(1,DAT_0013868c,"");
        }
        else {
          puVar1 = (undefined1 *)(*(long *)(*param_2 + 0x38) + lVar13);
          uVar2 = *puVar1;
          *puVar1 = 0;
          uVar15 = FUN_001239f0(1,uVar4);
          *puVar1 = uVar2;
        }
        uVar10 = FUN_0010e8a0(0,param_2[7]);
        uVar11 = dcgettext(0,
                           "File system loop detected; %s is part of the same file system loop as %s."
                           ,5);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar11,uVar10,uVar15);
      }
      if (sVar6 == 0xd) {
        iVar7 = (*DAT_00138670)(param_2[6],local_d8);
        if ((iVar7 != 0) && (piVar9 = __errno_location(), *piVar9 == 0x28)) {
LAB_00107b65:
          FUN_0010e8c0(0x28,param_2[7]);
          goto LAB_00107948;
        }
        sVar6 = (short)param_2[0xd];
      }
      else if (sVar6 == 10) {
        if (lVar13 == 0) goto LAB_00107ba8;
        iVar7 = (*DAT_00138670)(param_2[6],local_d8);
        if ((iVar7 != 0) && (piVar9 = __errno_location(), *piVar9 == 0x28)) goto LAB_00107b65;
        FUN_0010e8c0((int)param_2[8],param_2[7]);
        sVar6 = (short)param_2[0xd];
      }
    }
    if (1 < (ushort)(sVar6 - 10U)) goto LAB_00107761;
    if (DAT_001385e4 != '\0') {
LAB_00107df1:
                    /* WARNING: Subroutine does not return */
      __assert_fail("!state.have_stat","ftsfind.c",0x17c,"consider_visiting");
    }
    local_16c = DAT_001385e8;
    if ((sVar6 != 0xb) && (DAT_001385e8 != 0)) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("ent->fts_info == FTS_NSOK || state.type == 0","ftsfind.c",0x17d,
                    "consider_visiting");
    }
    DAT_001385e0 = (undefined4)param_2[0xb];
    if (DAT_001385e8 != 0) goto LAB_00107808;
    sVar6 = (short)param_2[0xd];
LAB_00107a4b:
    if (((ushort)(sVar6 - 1U) < 2) || (sVar6 == 6)) goto LAB_00107847;
    uVar15 = 0;
    bVar14 = false;
    if (-1 < DAT_00138624) {
LAB_00107862:
      if (param_2[0xb] < (long)DAT_00138624) goto LAB_0010786c;
      FUN_0011a750(param_1,param_2,4);
      bVar14 = (long)DAT_00138624 < param_2[0xb];
      sVar6 = (short)param_2[0xd];
      goto LAB_0010786e;
    }
LAB_00107882:
    uVar12 = (uint)CONCAT71(DAT_00138660._1_7_,(byte)DAT_00138660);
    if (param_2[0xb] < (long)DAT_00138628) goto LAB_00107934;
    if (((byte)DAT_00138660 & 4) != 0) goto LAB_00107bf5;
LAB_001078a3:
    if (bVar14 == false) {
      DAT_001385f0 = param_2[6];
      _DAT_001385e4 = CONCAT11(DAT_001385e5,1 < (ushort)(sVar6 - 10U));
      DAT_001385f8 = *(int *)(param_1 + 0x2c);
      uVar15 = FUN_0010b580();
      FUN_0010e7b0(param_2[7],&local_168,uVar15);
      if (DAT_00138600 != '\0') {
        FUN_0011a750(param_1,param_2,4);
      }
    }
  }
  if ((short)param_2[0xd] == 6) {
    DAT_00138600 = '\0';
  }
LAB_00107948:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: find @ 0x7e10

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
  DAT_001385fc = (undefined4)sVar3;
  if (((DAT_00138018 & 0x200) != 0) && (DAT_001385f8 = 0xffffff9c, DAT_00138010 < 0)) {
    DAT_00138010 = -100;
  }
  local_40 = 0;
  if (DAT_00138668 == 1) {
    DAT_00138018 = DAT_00138018 | 3;
  }
  else if (DAT_00138668 == 2) {
    DAT_00138018 = DAT_00138018 | 0x11;
  }
  else if (DAT_00138668 == 0) {
    DAT_00138018 = DAT_00138018 | 0x10;
  }
  if (DAT_0013862d != '\0') {
    DAT_00138018 = DAT_00138018 | 0x40;
  }
  iVar2 = -0x80000000;
  local_48 = param_1;
  lVar4 = FUN_001199d0(&local_48,DAT_00138018,0);
  piVar5 = __errno_location();
  if (lVar4 == 0) {
    uVar8 = FUN_0010e8a0(0,param_1);
    uVar9 = dcgettext(0,"cannot search %s",5);
                    /* WARNING: Subroutine does not return */
    error(0,*piVar5,uVar9,uVar8);
  }
  while( true ) {
    *piVar5 = 0;
    lVar6 = FUN_00119ff0(lVar4);
    if (lVar6 == 0) break;
    iVar1 = (int)*(undefined8 *)(lVar6 + 0x58);
    if ((DAT_00138608 != '\0') && (iVar2 != iVar1)) {
      FUN_0010d9f0();
      iVar1 = *(int *)(lVar6 + 0x58);
    }
    iVar2 = iVar1;
    DAT_001385e8 = *(int *)(lVar6 + 0x88);
    DAT_00138618 = 0;
    DAT_001385e4 = 0;
    DAT_001385e5 = DAT_001385e8 != 0;
    FUN_00107670(lVar4);
  }
  if (*piVar5 == 0) {
    iVar2 = FUN_00119e00(lVar4);
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
    param_1 = (char *)FUN_0010e8a0(0,param_1);
    pcVar7 = "failed to read file names from file system at or below %s";
  }
                    /* WARNING: Subroutine does not return */
  error(0,*piVar5,pcVar7,param_1);
}




// Function: process_all_startpoints @ 0x8070

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
  if ((param_1 < 1) || (cVar2 = FUN_0010e120(*param_2,1), cVar2 != '\0')) {
    pFVar1 = DAT_00138690;
    if (DAT_00138690 == (FILE *)0x0) {
      local_42 = 0x2e;
      uVar5 = FUN_00107e10(&local_42);
      goto LAB_001081b1;
    }
    iVar3 = strcmp((char *)DAT_00138690,"-");
    if (iVar3 == 0) {
      if (DAT_00138698 != '\0') {
        uVar6 = dcgettext(0,
                          "option -files0-from reading from standard input cannot be combined with -ok, -okdir"
                          ,5);
                    /* WARNING: Subroutine does not return */
        error(1,0,"%s\n",uVar6);
      }
      uVar6 = dcgettext(0,"(standard input)",5);
      uVar6 = FUN_0010e8a0(0,uVar6);
      local_180 = stdin;
    }
    else {
      uVar6 = FUN_0010e8a0(0,pFVar1);
      local_180 = fopen((char *)DAT_00138690,"r");
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
      if ((((DAT_00138698 != '\0') && (iVar4 = fstat(iVar3,&local_168), iVar4 == 0)) &&
          (iVar4 = fstat(0,&local_d8), iVar4 == 0)) &&
         ((local_168.st_ino == local_d8.st_ino && (local_168.st_dev == local_d8.st_dev)))) {
        uVar11 = dcgettext(0,
                           "option -files0-from: standard input must not refer to the same file when combined with -ok, -okdir"
                           ,5);
                    /* WARNING: Subroutine does not return */
        error(1,0,"%s: %s\n",uVar11,uVar6);
      }
      FUN_00117990(iVar3,1);
    }
    lVar7 = FUN_001176a0(local_180);
  }
  else {
    local_180 = DAT_00138690;
    if (DAT_00138690 != (FILE *)0x0) {
      uVar6 = FUN_0010e8a0(0,*param_2);
      uVar11 = dcgettext(0,"extra operand %s",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar11,uVar6);
    }
    uVar6 = 0;
    lVar7 = FUN_00117670(param_2);
  }
  if (lVar7 == 0) {
LAB_001084cd:
                    /* WARNING: Subroutine does not return */
    FUN_00125f50();
  }
  uVar5 = 1;
  do {
    __s = (char *)FUN_001176e0(lVar7,&local_16c);
    if (__s == (char *)0x0) {
      if (local_16c == 3) goto LAB_001084cd;
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
      goto LAB_00108176;
    }
    if (*__s == '\0') {
      if (DAT_00138690 == (FILE *)0x0) {
        uVar6 = FUN_0010e8a0(0,__s);
                    /* WARNING: Subroutine does not return */
        error(0,2,&DAT_0012d8da,uVar6);
      }
      uVar11 = FUN_00117780(lVar7);
      uVar9 = dcgettext(0,"invalid zero-length file name",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,"%s:%lu: %s",uVar6,uVar11,uVar9);
    }
    if ((DAT_00138690 == (FILE *)0x0) && (cVar2 = FUN_0010e120(__s,1), cVar2 != '\0'))
    goto LAB_00108176;
    sVar8 = strlen(__s);
    DAT_001385fc = (undefined4)sVar8;
    cVar2 = FUN_00107e10(__s);
  } while (cVar2 != '\0');
  uVar5 = 0;
LAB_00108176:
  FUN_001177a0(lVar7);
  if ((((char)uVar5 != '\0') && (DAT_00138690 != (FILE *)0x0)) &&
     ((iVar3 = ferror(local_180), iVar3 != 0 || (iVar3 = FUN_00117d40(local_180), iVar3 != 0)))) {
    uVar11 = dcgettext(0,"error reading %s",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar11,uVar6);
  }
LAB_001081b1:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar5;
}




// Function: is_fts_enabled @ 0x8530

undefined8 is_fts_enabled(undefined4 *param_1)

{
  *param_1 = DAT_00138018;
  return 1;
}




// Function: is_ok @ 0x8550

void is_ok(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  undefined8 uVar2;
  int *piVar3;
  
  FUN_001180c0(stdout);
  uVar2 = dcgettext(0,"< %s ... %s > ? ",5);
  iVar1 = __fprintf_chk(stderr,1,uVar2,param_1,param_2);
  if (-1 < iVar1) {
    FUN_001180c0(stderr);
    FUN_001268c0();
    return;
  }
  uVar2 = dcgettext(0,"Failed to write prompt for -ok",5);
  piVar3 = __errno_location();
                    /* WARNING: Subroutine does not return */
  error(1,*piVar3,uVar2);
}




// Function: print_parenthesised @ 0x8870

void print_parenthesised(undefined8 param_1,long *param_2)

{
  if (param_2 == (long *)0x0) {
    return;
  }
  while( true ) {
    if (((code *)*param_2 != FUN_00108940) && ((code *)*param_2 != FUN_00108990)) break;
    if (param_2[0x22] != 0) goto LAB_001088be;
    param_2 = (long *)param_2[0x23];
    if (param_2 == (long *)0x0) {
      return;
    }
  }
  if ((param_2[0x22] == 0) && (param_2[0x23] == 0)) {
    FUN_001086e0(param_1,param_2);
    return;
  }
LAB_001088be:
  __fprintf_chk(param_1,1,&DAT_0012d8da,&DAT_0012868b);
  FUN_001086e0(param_1,param_2);
  __fprintf_chk(param_1,1,&DAT_0012d8da,&DAT_0012868f);
  return;
}




// Function: pred_timewindow @ 0x89e0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

bool pred_timewindow(time_t param_1,long param_2,long param_3,int param_4)

{
  int iVar1;
  long lVar2;
  bool bVar3;
  double dVar4;
  
  iVar1 = *(int *)(param_3 + 0x3c);
  if (iVar1 == 1) {
    lVar2 = *(long *)(param_3 + 0x48);
    if ((param_1 != *(long *)(param_3 + 0x40)) || (bVar3 = false, lVar2 != param_2)) {
      dVar4 = difftime(param_1,*(long *)(param_3 + 0x40));
      bVar3 = (double)(param_2 - lVar2) * _DAT_00128950 + dVar4 < 0.0;
    }
  }
  else if (iVar1 == 2) {
    dVar4 = difftime(param_1,*(time_t *)(param_3 + 0x40));
    bVar3 = false;
    dVar4 = (double)(param_2 - *(long *)(param_3 + 0x48)) * _DAT_00128950 + dVar4;
    if (_DAT_00128958 < dVar4) {
      return dVar4 <= (double)param_4;
    }
  }
  else {
    if (iVar1 != 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("0","pred.c",0x8a,"pred_timewindow");
    }
    lVar2 = *(long *)(param_3 + 0x48);
    if ((param_1 != *(long *)(param_3 + 0x40)) || (bVar3 = false, param_2 != lVar2)) {
      dVar4 = difftime(param_1,*(long *)(param_3 + 0x40));
      return 0.0 <= (double)(param_2 - lVar2) * _DAT_00128950 + dVar4;
    }
  }
  return bVar3;
}




// Function: pred_amin @ 0x8b10

void pred_amin(undefined8 param_1,long param_2,undefined8 param_3)

{
  FUN_001089e0(*(undefined8 *)(param_2 + 0x48),*(undefined8 *)(param_2 + 0x50),param_3,0x3c);
  return;
}




// Function: pred_anewer @ 0x8b30

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

bool pred_anewer(undefined8 param_1,long param_2,long param_3)

{
  long lVar1;
  long lVar2;
  double dVar3;
  
  if (*(int *)(param_3 + 0x3c) != 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0xa6,"pred_anewer");
  }
  lVar1 = *(long *)(param_2 + 0x50);
  lVar2 = *(long *)(param_3 + 0x48);
  if ((*(long *)(param_2 + 0x48) == *(long *)(param_3 + 0x40)) && (lVar1 == lVar2)) {
    return false;
  }
  dVar3 = difftime(*(long *)(param_2 + 0x48),*(long *)(param_3 + 0x40));
  return 0.0 <= (double)(lVar1 - lVar2) * _DAT_00128950 + dVar3;
}




// Function: pred_atime @ 0x8bc0

void pred_atime(undefined8 param_1,long param_2,undefined8 param_3)

{
  FUN_001089e0(*(undefined8 *)(param_2 + 0x48),*(undefined8 *)(param_2 + 0x50),param_3,0x15180);
  return;
}




// Function: pred_cmin @ 0x8bf0

void pred_cmin(undefined8 param_1,long param_2,undefined8 param_3)

{
  FUN_001089e0(*(undefined8 *)(param_2 + 0x68),*(undefined8 *)(param_2 + 0x70),param_3,0x3c);
  return;
}




// Function: pred_cnewer @ 0x8c10

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

bool pred_cnewer(undefined8 param_1,long param_2,long param_3)

{
  long lVar1;
  long lVar2;
  double dVar3;
  
  if (*(int *)(param_3 + 0x3c) != 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",199,"pred_cnewer");
  }
  lVar1 = *(long *)(param_2 + 0x70);
  lVar2 = *(long *)(param_3 + 0x48);
  if ((*(long *)(param_2 + 0x68) == *(long *)(param_3 + 0x40)) && (lVar1 == lVar2)) {
    return false;
  }
  dVar3 = difftime(*(long *)(param_2 + 0x68),*(long *)(param_3 + 0x40));
  return 0.0 <= (double)(lVar1 - lVar2) * _DAT_00128950 + dVar3;
}




// Function: pred_comma @ 0x8ca0

void pred_comma(undefined8 param_1,undefined8 param_2,long param_3)

{
  if (*(long *)(param_3 + 0x110) != 0) {
    FUN_0010e7b0();
  }
  FUN_0010e7b0(param_1,param_2,*(undefined8 *)(param_3 + 0x118));
  return;
}




// Function: pred_ctime @ 0x8ce0

void pred_ctime(undefined8 param_1,long param_2,undefined8 param_3)

{
  FUN_001089e0(*(undefined8 *)(param_2 + 0x68),*(undefined8 *)(param_2 + 0x70),param_3,0x15180);
  return;
}




// Function: pred_delete @ 0x8d00

char pred_delete(undefined8 param_1,long param_2)

{
  char *__name;
  char cVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  
  __name = DAT_001385f0;
  iVar2 = strcmp(DAT_001385f0,".");
  if (iVar2 != 0) {
    iVar2 = 0;
    if (DAT_001385e4 != '\0') {
      iVar2 = (uint)((*(uint *)(param_2 + 0x18) & 0xf000) == 0x4000) << 9;
    }
    iVar3 = unlinkat(DAT_001385f8,__name,iVar2);
    if (iVar3 != 0) {
      piVar4 = __errno_location();
      cVar1 = DAT_0013862e;
      if (*piVar4 == 2) {
        if (DAT_0013862e != '\0') {
          *piVar4 = 0;
          return cVar1;
        }
      }
      else if (((*piVar4 == 0x15) && (iVar2 == 0)) &&
              (iVar2 = unlinkat(DAT_001385f8,DAT_001385f0,0x200), iVar2 == 0)) {
        return '\x01';
      }
      uVar5 = FUN_0010e8a0(0,param_1);
      uVar6 = dcgettext(0,"cannot delete %s",5);
                    /* WARNING: Subroutine does not return */
      error(0,*piVar4,uVar6,uVar5);
    }
  }
  return '\x01';
}




// Function: pred_empty @ 0x8e20

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
  uVar5 = DAT_001385f0;
  *piVar3 = 0;
  iVar2 = FUN_0011f080(DAT_001385f8,uVar5,0x90900);
  if (-1 < iVar2) {
    __dirp = fdopendir(iVar2);
    if (__dirp == (DIR *)0x0) {
      uVar5 = FUN_0010e8a0(0,param_1);
                    /* WARNING: Subroutine does not return */
      error(0,*piVar3,&DAT_0012d8da,uVar5);
    }
    *piVar3 = 0;
    do {
      pdVar4 = readdir(__dirp);
      if (pdVar4 == (dirent *)0x0) {
        bVar6 = true;
        goto LAB_00108ed3;
      }
    } while ((pdVar4->d_name[0] == '.') &&
            ((pdVar4->d_name[1] == '\0' || (*(short *)(pdVar4->d_name + 1) == 0x2e))));
    bVar6 = false;
LAB_00108ed3:
    if (*piVar3 != 0) {
      uVar5 = FUN_0010e8a0(0,param_1);
                    /* WARNING: Subroutine does not return */
      error(0,*piVar3,&DAT_0012d8da,uVar5);
    }
    iVar2 = closedir(__dirp);
    if (iVar2 == 0) {
      return bVar6;
    }
  }
  uVar5 = FUN_0010e8a0(0,param_1);
                    /* WARNING: Subroutine does not return */
  error(0,*piVar3,&DAT_0012d8da,uVar5);
}




// Function: pred_execdir @ 0x8fc0

void pred_execdir(void)

{
  FUN_0010a040(DAT_001385f0);
  return;
}




// Function: pred_false @ 0x8fd0

undefined8 pred_false(void)

{
  return 0;
}




// Function: pred_fstype @ 0x9090

bool pred_fstype(undefined8 param_1,undefined8 param_2,long param_3)

{
  char *__s2;
  char *__s1;
  int iVar1;
  
  __s2 = *(char **)(param_3 + 0x38);
  __s1 = (char *)FUN_001108e0(param_2,param_1);
  iVar1 = strcmp(__s1,__s2);
  return iVar1 == 0;
}




// Function: pred_gid @ 0x90c0

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




// Function: pred_group @ 0x9110

undefined4 pred_group(undefined8 param_1,long param_2,long param_3)

{
  return CONCAT31((int3)((uint)*(int *)(param_2 + 0x20) >> 8),
                  *(int *)(param_3 + 0x38) == *(int *)(param_2 + 0x20));
}




// Function: pred_ilname @ 0x9120

undefined8 pred_ilname(undefined8 param_1,long param_2,undefined8 param_3)

{
  undefined8 uVar1;
  
  if ((*(uint *)(param_2 + 0x18) & 0xf000) != 0xa000) {
    return 0;
  }
  uVar1 = FUN_00108660(param_1,param_3,1);
  return uVar1;
}




// Function: pred_iname @ 0x9150

bool pred_iname(undefined8 param_1,undefined8 param_2,long param_3)

{
  char *__pattern;
  int iVar1;
  char *__name;
  
  __pattern = *(char **)(param_3 + 0x38);
  __name = (char *)FUN_00117b90();
  FUN_00117d00(__name);
  iVar1 = fnmatch(__pattern,__name,0x10);
  free(__name);
  return iVar1 == 0;
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

undefined8 pred_lname(undefined8 param_1,long param_2,undefined8 param_3)

{
  undefined8 uVar1;
  
  if ((*(uint *)(param_2 + 0x18) & 0xf000) != 0xa000) {
    return 0;
  }
  uVar1 = FUN_00108660(param_1,param_3,0);
  return uVar1;
}




// Function: pred_ls @ 0x9290

undefined8 pred_ls(undefined8 param_1,undefined8 param_2,long param_3)

{
  FUN_001165e0(param_1,DAT_001385f8,DAT_001385f0,param_2,DAT_00138638,DAT_0013865c,
               *(undefined1 *)(param_3 + 0x28),*(undefined8 *)(param_3 + 0x40));
  return 1;
}




// Function: pred_mmin @ 0x92d0

void pred_mmin(undefined8 param_1,long param_2,undefined8 param_3)

{
  FUN_001089e0(*(undefined8 *)(param_2 + 0x58),*(undefined8 *)(param_2 + 0x60),param_3,0x3c);
  return;
}




// Function: pred_mtime @ 0x92f0

void pred_mtime(undefined8 param_1,long param_2,undefined8 param_3)

{
  FUN_001089e0(*(undefined8 *)(param_2 + 0x58),*(undefined8 *)(param_2 + 0x60),param_3,0x15180);
  return;
}




// Function: pred_name @ 0x9310

bool pred_name(undefined8 param_1,undefined8 param_2,long param_3)

{
  char *__pattern;
  int iVar1;
  char *__name;
  
  __pattern = *(char **)(param_3 + 0x38);
  __name = (char *)FUN_00117b90();
  FUN_00117d00(__name);
  iVar1 = fnmatch(__pattern,__name,0);
  free(__name);
  return iVar1 == 0;
}




// Function: pred_negate @ 0x9350

uint pred_negate(undefined8 param_1,undefined8 param_2,long param_3)

{
  uint uVar1;
  
  uVar1 = FUN_0010e7b0(param_1,param_2,*(undefined8 *)(param_3 + 0x118));
  return uVar1 ^ 1;
}




// Function: pred_newer @ 0x9370

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

bool pred_newer(undefined8 param_1,long param_2,long param_3)

{
  long lVar1;
  long lVar2;
  double dVar3;
  
  if (*(int *)(param_3 + 0x3c) != 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0x26d,"pred_newer");
  }
  lVar1 = *(long *)(param_2 + 0x60);
  lVar2 = *(long *)(param_3 + 0x48);
  if ((*(long *)(param_2 + 0x58) == *(long *)(param_3 + 0x40)) && (lVar1 == lVar2)) {
    return false;
  }
  dVar3 = difftime(*(long *)(param_2 + 0x58),*(long *)(param_3 + 0x40));
  return 0.0 <= (double)(lVar1 - lVar2) * _DAT_00128950 + dVar3;
}




// Function: pred_newerXY @ 0x9400

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

bool pred_newerXY(undefined8 param_1,long param_2,long param_3)

{
  long lVar1;
  bool bVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long lVar5;
  time_t __time1;
  double dVar6;
  
  if (*(int *)(param_3 + 0x3c) == 0) {
    switch(*(undefined4 *)(param_3 + 0x38)) {
    case 0:
      __time1 = *(time_t *)(param_2 + 0x48);
      lVar5 = *(long *)(param_2 + 0x50);
      break;
    case 1:
      uVar3 = FUN_0010e8a0(0,param_1);
      uVar4 = dcgettext(0,"WARNING: cannot determine birth time of file %s",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar4,uVar3);
    case 2:
      __time1 = *(time_t *)(param_2 + 0x68);
      lVar5 = *(long *)(param_2 + 0x70);
      break;
    case 3:
      __time1 = *(time_t *)(param_2 + 0x58);
      lVar5 = *(long *)(param_2 + 0x60);
      break;
    case 4:
                    /* WARNING: Subroutine does not return */
      __assert_fail("pred_ptr->args.reftime.xval != XVAL_TIME","pred.c",0x27c,"pred_newerXY");
    default:
                    /* WARNING: Subroutine does not return */
      __assert_fail("collected","pred.c",0x29b,"pred_newerXY");
    }
    lVar1 = *(long *)(param_3 + 0x48);
    if ((*(long *)(param_3 + 0x40) != __time1) || (bVar2 = false, lVar5 != lVar1)) {
      dVar6 = difftime(__time1,*(long *)(param_3 + 0x40));
      bVar2 = 0.0 <= (double)(lVar5 - lVar1) * _DAT_00128950 + dVar6;
    }
    return bVar2;
  }
                    /* WARNING: Subroutine does not return */
  __assert_fail("COMP_GT == pred_ptr->args.reftime.kind","pred.c",0x277,"pred_newerXY");
}




// Function: pred_nogroup @ 0x9540

bool pred_nogroup(undefined8 param_1,long param_2)

{
  group *pgVar1;
  
  pgVar1 = getgrgid(*(__gid_t *)(param_2 + 0x20));
  return pgVar1 == (group *)0x0;
}




// Function: pred_nouser @ 0x9560

bool pred_nouser(undefined8 param_1,long param_2)

{
  passwd *ppVar1;
  
  ppVar1 = getpwuid(*(__uid_t *)(param_2 + 0x1c));
  return ppVar1 == (passwd *)0x0;
}




// Function: pred_ok @ 0x9580

undefined8 pred_ok(undefined8 param_1,undefined8 param_2,long param_3)

{
  char cVar1;
  undefined8 uVar2;
  
  cVar1 = FUN_00108550(**(undefined8 **)(param_3 + 0xe8),param_1);
  if (cVar1 == '\0') {
    return 0;
  }
  uVar2 = FUN_0010a040(param_1,param_2,param_3);
  return uVar2;
}




// Function: pred_okdir @ 0x95d0

undefined8 pred_okdir(undefined8 param_1,undefined8 param_2,long param_3)

{
  char cVar1;
  undefined8 uVar2;
  
  cVar1 = FUN_00108550(**(undefined8 **)(param_3 + 0xe8),param_1);
  if (cVar1 == '\0') {
    return 0;
  }
  uVar2 = FUN_0010a040(DAT_001385f0,param_2,param_3);
  return uVar2;
}




// Function: pred_path @ 0x9640

bool pred_path(char *param_1,undefined8 param_2,long param_3)

{
  int iVar1;
  
  iVar1 = fnmatch(*(char **)(param_3 + 0x38),param_1,0);
  return iVar1 == 0;
}




// Function: pred_perm @ 0x9660

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




// Function: pred_executable @ 0x96e0

bool pred_executable(void)

{
  int iVar1;
  
  iVar1 = faccessat(DAT_001385f8,DAT_001385f0,1,0);
  return iVar1 == 0;
}




// Function: pred_readable @ 0x9710

bool pred_readable(void)

{
  int iVar1;
  
  iVar1 = faccessat(DAT_001385f8,DAT_001385f0,4,0);
  return iVar1 == 0;
}




// Function: pred_writable @ 0x9740

bool pred_writable(void)

{
  int iVar1;
  
  iVar1 = faccessat(DAT_001385f8,DAT_001385f0,2,0);
  return iVar1 == 0;
}




// Function: pred_regex @ 0x97e0

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




// Function: pred_size @ 0x9820

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




// Function: pred_samefile @ 0x9890

undefined8 pred_samefile(undefined8 param_1,long *param_2,long param_3)

{
  undefined8 uVar1;
  
  if (((param_2[1] == 0) || (param_2[1] == *(long *)(param_3 + 0x38))) &&
     (uVar1 = FUN_0010d820(param_1,DAT_001385f0,param_2), (int)uVar1 == 0)) {
    if (param_2[1] != *(long *)(param_3 + 0x38)) {
      return uVar1;
    }
    return CONCAT71((int7)((ulong)*(long *)(param_3 + 0x40) >> 8),
                    *param_2 == *(long *)(param_3 + 0x40));
  }
  return 0;
}




// Function: pred_type @ 0x9900

undefined1 pred_type(undefined8 param_1,long param_2,long param_3)

{
  uint uVar1;
  long lVar2;
  undefined1 uVar3;
  
  if (DAT_001385e5 == '\0') {
                    /* WARNING: Subroutine does not return */
    __assert_fail("state.have_type","pred.c",0x3d1,"pred_type");
  }
  uVar3 = 0;
  if (DAT_001385e8 != 0) {
    uVar1 = DAT_001385e8;
    if (DAT_001385e4 != '\0') {
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




// Function: pred_uid @ 0x99e0

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




// Function: pred_used @ 0x9a30

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 pred_used(undefined8 param_1,long param_2,undefined8 param_3)

{
  long lVar1;
  long __time1;
  undefined8 uVar2;
  long lVar3;
  long lVar4;
  double dVar5;
  
  lVar1 = *(long *)(param_2 + 0x50);
  lVar3 = *(long *)(param_2 + 0x70);
  __time1 = *(long *)(param_2 + 0x48);
  lVar4 = *(long *)(param_2 + 0x68);
  if (lVar1 == lVar3 && __time1 == lVar4) {
    lVar3 = 0;
    lVar4 = 0;
  }
  else {
    dVar5 = difftime(__time1,lVar4);
    if ((double)(lVar1 - lVar3) * _DAT_00128950 + dVar5 < 0.0) {
      return 0;
    }
    lVar4 = lVar4 - __time1;
    lVar3 = lVar3 - lVar1;
    if (lVar3 < 0) {
      lVar3 = lVar3 + 1000000000;
      lVar4 = lVar4 + -1;
    }
  }
  uVar2 = FUN_001089e0(lVar4,lVar3,param_3,0x15180);
  return uVar2;
}




// Function: pred_user @ 0x9ae0

undefined4 pred_user(undefined8 param_1,long param_2,long param_3)

{
  return CONCAT31((int3)((uint)*(int *)(param_2 + 0x1c) >> 8),
                  *(int *)(param_3 + 0x38) == *(int *)(param_2 + 0x1c));
}




// Function: pred_xtype @ 0x9af0

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
  cVar1 = FUN_0010dfb0();
  if (cVar1 == '\0') {
    pcVar5 = FUN_0010ddb0;
  }
  else {
    pcVar5 = FUN_0010de70;
  }
  FUN_0010d810(auStack_c8);
  iVar2 = (*pcVar5)(DAT_001385f0,auStack_c8);
  if (iVar2 == 0) {
    FUN_00109900(param_1,auStack_c8,param_3);
  }
  else {
    cVar1 = FUN_0010dfb0();
    piVar3 = __errno_location();
    if ((cVar1 == '\0') || (*piVar3 != 2)) {
      uVar4 = FUN_0010e8a0(0,param_1);
                    /* WARNING: Subroutine does not return */
      error(0,*piVar3,&DAT_0012d8da,uVar4);
    }
    FUN_00109900(param_1,param_2,param_3);
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: pred_context @ 0x9c00

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
  iVar1 = (*DAT_00138680)(DAT_001385f8,DAT_001385f0,&local_38);
  if (iVar1 < 0) {
    uVar2 = FUN_0010e8a0(0,param_1);
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




// Function: print_list @ 0x9cd0

void print_list(undefined8 param_1,long param_2)

{
  char *pcVar1;
  size_t sVar2;
  char *pcVar3;
  int iVar4;
  long in_FS_OFFSET;
  char acStack_138 [264];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  do {
    if (param_2 == 0) {
      __fprintf_chk(param_1,1,"\n");
      if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return;
    }
    pcVar3 = (undefined1 *)0x0;
    if (*(long *)(param_2 + 8) != 0) {
      __strcpy_chk(acStack_138,*(long *)(param_2 + 8),0x100);
      sVar2 = strlen(acStack_138);
      iVar4 = (int)sVar2 + -1;
      if (iVar4 < 0) {
        pcVar3 = acStack_138 + (int)sVar2;
      }
      else {
        pcVar1 = acStack_138 + iVar4;
        do {
          pcVar3 = pcVar1;
          if ((*pcVar3 != ' ') && (*pcVar3 != '\t')) {
            pcVar3 = acStack_138 + (iVar4 + 1);
            break;
          }
          iVar4 = iVar4 + -1;
          pcVar1 = pcVar3 + -1;
        } while (iVar4 != -1);
      }
      *pcVar3 = '\0';
      pcVar3 = acStack_138;
    }
    __fprintf_chk(param_1,1,"[%s] ",pcVar3);
    param_2 = *(long *)(param_2 + 0x108);
  } while( true );
}




// Function: show_success_rates @ 0x9e00

void show_success_rates(long param_1)

{
  if (((byte)DAT_00138660 & 0x40) == 0) {
    return;
  }
  __fprintf_chk(stderr,1,"Predicate success rates after completion:\n");
  if (param_1 != 0) {
    FUN_001086e0(stderr,param_1);
  }
  __fprintf_chk(stderr,1,"\n");
  return;
}




// Function: pred_sanity_check @ 0x9e60

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
      if (((pcVar1 != FUN_00108600) && (pcVar1 != FUN_001085e0)) &&
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




// Function: impl_pred_exec @ 0xa040

ulong impl_pred_exec(char *param_1,undefined8 param_2,undefined8 *param_3)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  char *pcVar3;
  byte bVar4;
  undefined4 uVar5;
  size_t sVar6;
  size_t sVar7;
  char *pcVar8;
  void *__ptr;
  undefined4 *puVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  int *piVar12;
  long lVar13;
  ulong uVar14;
  char cVar15;
  undefined *local_50;
  byte local_41;
  char *local_40;
  
  bVar4 = FUN_0010e880(*param_3);
  pcVar3 = DAT_001385f0;
  if (bVar4 == 0) {
    if (param_3[0x1f] != DAT_001385c0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("execp->wd_for_exec == initial_wd","exec.c",0x94,"impl_pred_exec");
    }
    local_50 = (undefined *)0x0;
    cVar15 = '\0';
    local_40 = (char *)0x0;
  }
  else {
    if (*(int *)(param_3 + 0x1a) == 0) {
      pcVar8 = strchr(DAT_001385f0,0x2f);
      uVar5 = DAT_001385f8;
      if (pcVar8 == (char *)0x0) {
        puVar9 = (undefined4 *)FUN_00125a50(0x10);
        param_3[0x1f] = puVar9;
        *(undefined8 *)(puVar9 + 2) = 0;
        uVar5 = FUN_0011f080(uVar5,&DAT_0012c379,0);
        *puVar9 = uVar5;
        if (*(int *)param_3[0x1f] < 0) goto LAB_0010a33f;
        FUN_00117990(*(int *)param_3[0x1f],1);
      }
      else {
        __ptr = (void *)FUN_00117c40(pcVar3);
        uVar5 = DAT_001385f8;
        puVar9 = (undefined4 *)FUN_00125a50(0x10);
        param_3[0x1f] = puVar9;
        *(undefined8 *)(puVar9 + 2) = 0;
        uVar5 = FUN_0011f080(uVar5,__ptr,0);
        *puVar9 = uVar5;
        if (*(int *)param_3[0x1f] < 0) {
          free(__ptr);
LAB_0010a33f:
          uVar10 = FUN_0010e8a0(0,param_1);
          uVar11 = dcgettext(0,"Failed to save working directory in order to run a command on %s",5)
          ;
          piVar12 = __errno_location();
                    /* WARNING: Subroutine does not return */
          error(1,*piVar12,uVar11,uVar10);
        }
        FUN_00117990(*(int *)param_3[0x1f],1);
        free(__ptr);
      }
    }
    param_1 = (char *)FUN_00117b90(DAT_001385f0);
    cVar15 = (*param_1 != '/') * '\x02';
    local_50 = (undefined *)0x0;
    local_40 = param_1;
    if (*param_1 != '/') {
      local_50 = &DAT_00128960;
    }
  }
  uVar14 = (ulong)*(byte *)(param_3 + 7);
  puVar2 = param_3 + 0x13;
  puVar1 = param_3 + 8;
  if (*(byte *)(param_3 + 7) == 0) {
    lVar13 = 0;
    if (0 < *(int *)(param_3 + 0x1e)) {
      do {
        sVar6 = strlen(param_1);
        pcVar3 = *(char **)(param_3[0x1d] + lVar13 * 8);
        lVar13 = lVar13 + 1;
        sVar7 = strlen(pcVar3);
        FUN_00115270(puVar1,puVar2,pcVar3,sVar7,local_50,cVar15,param_1,sVar6,0);
      } while ((int)lVar13 < *(int *)(param_3 + 0x1e));
    }
    FUN_00115430(puVar1,puVar2);
    local_41 = (*(uint *)(param_3 + 0x20) >> 8 & 0xff) == 0 &&
               (*(uint *)(param_3 + 0x20) & 0x7f) == 0;
    uVar14 = CONCAT71((int7)((ulong)lVar13 >> 8),local_41);
    if (bVar4 == 0) {
      if (local_40 != (char *)0x0) {
LAB_0010a1db:
                    /* WARNING: Subroutine does not return */
        __assert_fail("local","exec.c",0xd0,"impl_pred_exec");
      }
      goto LAB_0010a14d;
    }
    FUN_001242c0(param_3[0x1f]);
    if (local_40 == (char *)0x0) goto LAB_0010a14d;
  }
  else {
    sVar6 = strlen(param_1);
    FUN_00114fb0(puVar1,puVar2,param_1,sVar6 + 1,local_50,cVar15,0);
    if (*(int *)(param_3 + 0x1a) != 0) {
      DAT_00138608 = 1;
    }
    if (local_40 == (char *)0x0) goto LAB_0010a14d;
    local_41 = bVar4;
    if (bVar4 == 0) goto LAB_0010a1db;
  }
  free(local_40);
  uVar14 = (ulong)local_41;
LAB_0010a14d:
  return uVar14 & 0xffffffff;
}




// Function: launch @ 0xa3a0

undefined8 launch(undefined8 param_1,char *param_2,undefined8 param_3,char **param_4)

{
  long lVar1;
  char *pcVar2;
  char cVar3;
  __pid_t __pid;
  int iVar4;
  __pid_t _Var5;
  int *piVar6;
  undefined8 uVar7;
  int *piVar8;
  undefined8 uVar9;
  uint uVar10;
  ulong uVar11;
  char *pcVar12;
  long in_FS_OFFSET;
  undefined8 local_3a;
  undefined2 local_32;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (((byte)DAT_00138660 & 0x20) != 0) {
    uVar11 = 0;
    __fprintf_chk(stderr,1,"DebugExec: launching process (argc=%lu):",*(long *)(param_2 + 0x60) + -1
                 );
    if (*(long *)(param_2 + 0x60) != 1) {
      do {
        lVar1 = uVar11 * 8;
        uVar11 = uVar11 + 1;
        uVar7 = FUN_0010e8a0(0,*(undefined8 *)(*(long *)(param_2 + 0x68) + lVar1));
        __fprintf_chk(stderr,1,&DAT_0012d8d9,uVar7);
      } while (uVar11 < *(long *)(param_2 + 0x60) - 1U);
    }
    __fprintf_chk(stderr,1,"\n");
  }
  FUN_001180c0(stdout);
  FUN_001180c0();
  if (DAT_0013801c != 0) {
    DAT_0013801c = 0;
    signal(0x11,(__sighandler_t)0x0);
  }
  __pid = fork();
  if (__pid == -1) {
    uVar7 = dcgettext(0,"cannot fork",5);
    piVar8 = __errno_location();
                    /* WARNING: Subroutine does not return */
    error(1,*piVar8,uVar7);
  }
  if (__pid != 0) {
    while (_Var5 = waitpid(__pid,(int *)(param_2 + 200),0), _Var5 == -1) {
      piVar8 = __errno_location();
      if (*piVar8 != 4) {
        uVar7 = FUN_0010e8a0(0,*param_4);
        uVar9 = dcgettext(0,"error waiting for %s",5);
                    /* WARNING: Subroutine does not return */
        error(0,*piVar8,uVar9,uVar7);
      }
    }
    uVar10 = *(uint *)(param_2 + 200);
    if ('\x01' < (char)(((byte)uVar10 & 0x7f) + 1)) {
      uVar7 = FUN_001239f0(0,DAT_0013868c,*param_4);
      uVar9 = dcgettext(0,"%s terminated by signal %d",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar9,uVar7,uVar10 & 0x7f);
    }
    uVar10 = uVar10 >> 8 & 0xff;
    if (((byte)DAT_00138660 & 0x20) != 0) {
      __fprintf_chk(stderr,1,"DebugExec: process (PID=%ld) terminated with exit status: %d\n",
                    (long)__pid,uVar10);
    }
    if ((uVar10 != 0) && (*param_2 != '\0')) {
      DAT_00138604 = 1;
    }
    if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    return 1;
  }
  piVar8 = *(int **)(param_2 + 0xc0);
  if (piVar8 == (int *)0x0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("NULL != execp->wd_for_exec","exec.c",0x141,"launch");
  }
  piVar6 = __errno_location();
  if (param_2[0xbc] != '\0') {
    local_3a = 0x6c756e2f7665642f;
    local_32 = 0x6c;
    iVar4 = close(0);
    if (iVar4 < 0) {
      uVar7 = dcgettext(0,"Cannot close standard input",5);
                    /* WARNING: Subroutine does not return */
      error(0,*piVar6,uVar7);
    }
    iVar4 = FUN_00118040(&local_3a,0);
    if (iVar4 < 0) {
      uVar7 = FUN_0010e8a0(0,&local_3a);
                    /* WARNING: Subroutine does not return */
      error(0,*piVar6,&DAT_0012d8da,uVar7);
    }
  }
  iVar4 = FUN_00124290(piVar8);
  if (iVar4 == 0) {
    cVar3 = FUN_00116220();
    if (cVar3 != '\0') {
      FUN_00116240();
    }
    cVar3 = FUN_00115970(param_4);
    if (cVar3 == '\0') {
      execvp(*param_4,param_4);
    }
    else {
      *piVar6 = 7;
    }
    uVar7 = FUN_0010e8a0(0,*param_4);
                    /* WARNING: Subroutine does not return */
    error(0,*piVar6,&DAT_0012d8da,uVar7);
  }
  if (*piVar8 < 0) {
    pcVar12 = *(char **)(piVar8 + 2);
    pcVar2 = "Features enabled: ";
    if (pcVar12 != (char *)0x0) goto LAB_0010a61e;
  }
  pcVar12 = "";
  pcVar2 = "debug_stat (%s)\n";
LAB_0010a61e:
  uVar7 = dcgettext(0,"Failed to change directory%s%s",5);
                    /* WARNING: Subroutine does not return */
  error(0,*piVar6,uVar7,pcVar2 + 0x10,pcVar12);
}




// Function: check_normalization @ 0xa840

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




// Function: get_expr @ 0xa8c0

long get_expr(long *param_1,short param_2,long param_3)

{
  short sVar1;
  int iVar2;
  uint uVar3;
  long lVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long lVar7;
  
  lVar4 = *param_1;
  if (lVar4 == 0) {
    uVar6 = dcgettext(0,"invalid expression",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar6);
  }
  switch(*(undefined4 *)(lVar4 + 0x10)) {
  case 0:
    uVar6 = dcgettext(0,"invalid expression",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar6);
  case 1:
    lVar7 = *(long *)(lVar4 + 0x108);
    *param_1 = lVar7;
    break;
  case 2:
    *param_1 = *(long *)(lVar4 + 0x108);
    uVar6 = get_expr(param_1,4,lVar4);
    *(undefined8 *)(lVar4 + 0x118) = uVar6;
    lVar7 = *param_1;
    break;
  case 3:
    uVar6 = *(undefined8 *)(lVar4 + 8);
    uVar5 = dcgettext(0,
                      "invalid expression; you have used a binary operator \'%s\' with nothing before it."
                      ,5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar5,uVar6);
  case 4:
    lVar7 = *(long *)(lVar4 + 0x108);
    if ((lVar7 == 0) || (*(char *)(lVar7 + 0x29) != '\0')) {
      uVar6 = *(undefined8 *)(lVar4 + 8);
      uVar5 = dcgettext(0,
                        "invalid expression; expected to find a \')\' but didn\'t see one. Perhaps you need an extra predicate after \'%s\'"
                        ,5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar5,uVar6);
    }
    iVar2 = *(int *)(lVar7 + 0x10);
    *param_1 = lVar7;
    if (iVar2 == 5) {
      if (*(char *)(lVar4 + 0x29) == '\0') {
        uVar6 = dcgettext(0,"invalid expression; empty parentheses are not allowed.",5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar6);
      }
      uVar6 = *(undefined8 *)(lVar7 + 8);
      uVar5 = dcgettext(0,
                        "invalid expression: expected expression before closing parentheses \'%s\'."
                        ,5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar5,uVar6);
    }
    lVar4 = get_expr(param_1,0,lVar4);
    lVar7 = *param_1;
    if ((lVar7 == 0) || (*(int *)(lVar7 + 0x10) != 5)) {
      uVar6 = dcgettext(0,
                        "invalid expression; I was expecting to find a \')\' somewhere but did not see one."
                        ,5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar6);
    }
    lVar7 = *(long *)(lVar7 + 0x108);
    *param_1 = lVar7;
    break;
  case 5:
    if (param_3 == 0) {
      uVar6 = *(undefined8 *)(lVar4 + 8);
      uVar5 = dcgettext(0,
                        "invalid expression: expected expression before closing parentheses \'%s\'."
                        ,5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar5,uVar6);
    }
    if (*(int *)(param_3 + 0x10) - 2U < 2) {
      if (*(char *)(lVar4 + 0x29) == '\0') {
        uVar6 = *(undefined8 *)(param_3 + 8);
        uVar5 = dcgettext(0,"expected an expression between \'%s\' and \')\'",5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar5,uVar6);
      }
    }
    else if (*(char *)(lVar4 + 0x29) == '\0') {
      uVar6 = dcgettext(0,"invalid expression; you have too many \')\'",5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar6);
    }
    uVar6 = *(undefined8 *)(param_3 + 8);
    uVar5 = dcgettext(0,"expected an expression after \'%s\'",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar5,uVar6);
  default:
    uVar6 = dcgettext(0,"oops -- invalid expression type!",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar6);
  }
  if ((lVar7 == 0) || (*(int *)(lVar7 + 0x14) <= (int)param_2)) {
    return lVar4;
  }
  uVar3 = *(uint *)(lVar7 + 0x10);
  if (uVar3 == 5) {
LAB_0010ab18:
    uVar6 = dcgettext(0,"invalid expression",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar6);
  }
  while (uVar3 != 4) {
    if (4 < uVar3) {
      if (uVar3 != 5) {
        uVar6 = dcgettext(0,"oops -- invalid expression type (%d)!",5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar6,uVar3);
      }
      if (lVar4 != 0) {
        return lVar4;
      }
      goto LAB_0010ab18;
    }
    if (uVar3 != 3) break;
    *(long *)(lVar7 + 0x110) = lVar4;
    lVar4 = *param_1;
    sVar1 = *(short *)(lVar4 + 0x14);
    *param_1 = *(long *)(lVar4 + 0x108);
    uVar6 = get_expr(param_1,(int)sVar1);
    *(undefined8 *)(lVar4 + 0x118) = uVar6;
    lVar7 = *param_1;
    if (lVar7 == 0) {
      return lVar4;
    }
    if (*(int *)(lVar7 + 0x14) <= (int)param_2) {
      return lVar4;
    }
    uVar3 = *(uint *)(lVar7 + 0x10);
  }
  uVar6 = dcgettext(0,"invalid expression",5);
                    /* WARNING: Subroutine does not return */
  error(1,0,uVar6);
}




// Function: set_new_parent @ 0xad30

void set_new_parent(long param_1,int param_2,undefined8 *param_3)

{
  float fVar1;
  float fVar2;
  undefined8 *puVar3;
  float fVar4;
  
  puVar3 = (undefined8 *)FUN_00125e50(0x138);
  *(undefined4 *)(puVar3 + 2) = 3;
  *(int *)((long)puVar3 + 0x14) = param_2;
  *(undefined4 *)(puVar3 + 4) = 0;
  fVar2 = DAT_00129578;
  if (param_2 == 2) {
    fVar1 = *(float *)(param_1 + 0x24);
    puVar3[1] = &DAT_00128b4c;
    *puVar3 = FUN_00108940;
  }
  else {
    if (param_2 != 3) {
      if (param_2 != 1) {
        puVar3[0x23] = param_1;
        *param_3 = puVar3;
        return;
      }
      *param_3 = puVar3;
      puVar3[1] = &DAT_00128b4a;
      *puVar3 = FUN_00108ca0;
      *(undefined4 *)((long)puVar3 + 0x24) = 0x3f800000;
      puVar3[0x23] = param_1;
      return;
    }
    fVar1 = *(float *)(param_1 + 0x24);
    puVar3[1] = &DAT_00128b4f;
    *puVar3 = FUN_00108990;
  }
  fVar4 = DAT_00129578;
  if ((fVar1 <= fVar2) && (fVar4 = 0.0, 0.0 <= fVar1)) {
    fVar4 = fVar1;
  }
  puVar3[0x23] = param_1;
  *(float *)((long)puVar3 + 0x24) = fVar4;
  *param_3 = puVar3;
  return;
}




// Function: estimate_costs @ 0xae30

void estimate_costs(long *param_1)

{
  long lVar1;
  undefined *puVar2;
  undefined *puVar3;
  int iVar4;
  uint uVar5;
  undefined8 uVar6;
  ulong uVar7;
  ulong uVar8;
  ulong uVar9;
  undefined **ppuVar10;
  long in_FS_OFFSET;
  undefined *local_68;
  uint local_5c;
  undefined *local_50;
  undefined *local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 != (long *)0x0) {
    estimate_costs(param_1[0x23]);
    estimate_costs(param_1[0x22]);
    local_5c = 3;
    if ((*(char *)((long)param_1 + 0x1a) == '\0') &&
       (local_5c = 1, *(char *)((long)param_1 + 0x1c) == '\0')) {
      local_5c = (uint)*(byte *)((long)param_1 + 0x1b) + (uint)*(byte *)((long)param_1 + 0x1b);
    }
    local_68 = (undefined *)*param_1;
    if ((local_68 == PTR_thunk_FUN_0010a040_00137fd0) || (local_68 == PTR_FUN_00137fc0)) {
      uVar5 = 8 - *(byte *)(param_1 + 7);
    }
    else {
      if (local_68 != PTR_FUN_00137fa8) {
        if (DAT_001386b0 == 0) {
          ppuVar10 = &PTR_FUN_001380e0;
          qsort(&PTR_FUN_001380e0,0x3b,0x10,FUN_0010acd0);
          do {
            puVar2 = ppuVar10[2];
            puVar3 = *ppuVar10;
            local_50 = puVar2;
            local_48 = puVar3;
            iVar4 = memcmp(&local_50,&local_48,8);
            if (iVar4 < 0) {
                    /* WARNING: Subroutine does not return */
              error(1,0,"failed to sort the costlookup array");
            }
            local_50 = puVar3;
            local_48 = puVar2;
            iVar4 = memcmp(&local_50,&local_48,8);
            if (0 < iVar4) {
                    /* WARNING: Subroutine does not return */
              __assert_fail("result <= 0","tree.c",0x3f5,"check_sorted");
            }
            ppuVar10 = ppuVar10 + 2;
          } while (ppuVar10 != &PTR_FUN_00138480);
          DAT_001386b0 = 1;
          local_68 = (undefined *)*param_1;
        }
        uVar9 = 0;
        uVar8 = 0x3b;
        do {
          while( true ) {
            uVar7 = uVar9 + uVar8 >> 1;
            local_50 = local_68;
            local_48 = (&PTR_FUN_001380e0)[uVar7 * 2];
            iVar4 = memcmp(&local_50,&local_48,8);
            if (-1 < iVar4) break;
            uVar8 = uVar7;
            if (uVar7 <= uVar9) goto LAB_0010af40;
          }
          if (iVar4 == 0) {
            uVar5 = *(uint *)(uVar7 * 0x10 + 0x1380e8);
            goto LAB_0010afd3;
          }
          uVar9 = uVar7 + 1;
        } while (uVar9 < uVar8);
LAB_0010af40:
        lVar1 = param_1[1];
        uVar6 = dcgettext(0,
                          "warning: there is no entry in the predicate evaluation cost table for predicate %s; please report this as a bug"
                          ,5);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar6,lVar1);
      }
      uVar5 = *(uint *)(param_1 + 4);
LAB_0010afd3:
      if (uVar5 <= local_5c) {
        uVar5 = local_5c;
      }
    }
    *(uint *)(param_1 + 4) = uVar5;
  }
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: matches_start_point @ 0xb4b0

undefined8 matches_start_point(char *param_1,byte param_2)

{
  int iVar1;
  undefined4 extraout_var;
  ulong uVar2;
  
  if (DAT_001386b8 == 0) {
    iVar1 = fnmatch(param_1,".",(uint)param_2 << 4);
    return CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),iVar1 == 0);
  }
  uVar2 = 0;
  do {
    iVar1 = fnmatch(param_1,*(char **)(DAT_001386c0 + uVar2 * 8),(uint)param_2 << 4);
    if (iVar1 == 0) {
      return 1;
    }
    uVar2 = uVar2 + 1;
  } while (uVar2 < DAT_001386b8);
  return 0;
}




// Function: get_eval_tree @ 0xb580

undefined8 get_eval_tree(void)

{
  return DAT_001386d0;
}




// Function: calculate_derived_rates @ 0xb590

float calculate_derived_rates(long *param_1)

{
  uint uVar1;
  long lVar2;
  long lVar3;
  undefined *puVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  
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
    if ((code *)*param_1 != FUN_00109350) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("pred_is (p, pred_negate)","tree.c",0x49c,"calculate_derived_rates");
    }
    if (lVar3 != 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("NULL == p->pred_left","tree.c",0x49d,"calculate_derived_rates");
    }
    fVar6 = DAT_00129578 - *(float *)(lVar2 + 0x24);
    *(float *)((long)param_1 + 0x24) = fVar6;
    return fVar6;
  }
  if (uVar1 < 3) {
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
    return *(float *)((long)param_1 + 0x24);
  }
  if (uVar1 != 3) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("0","tree.c",0x4c0,"calculate_derived_rates");
  }
  puVar4 = (undefined *)*param_1;
  fVar6 = DAT_00129578;
  if (puVar4 == PTR_FUN_00137f48) {
    fVar5 = DAT_00129578;
    if (lVar2 != 0) {
      fVar5 = *(float *)(lVar2 + 0x24);
    }
    if (lVar3 != 0) {
      fVar5 = fVar5 * *(float *)(lVar3 + 0x24);
    }
  }
  else {
    if (puVar4 == PTR_FUN_00137f60) goto LAB_0010b678;
    if (puVar4 != PTR_FUN_00137ff0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("0","tree.c",0x4b4,"calculate_derived_rates");
    }
    fVar7 = DAT_00129578;
    if (lVar2 != 0) {
      fVar7 = *(float *)(lVar2 + 0x24);
    }
    fVar5 = DAT_00129578;
    if (lVar3 != 0) {
      fVar5 = *(float *)(lVar3 + 0x24);
    }
    fVar5 = fVar5 + fVar7;
  }
  if ((fVar5 <= DAT_00129578) && (fVar6 = 0.0, 0.0 <= fVar5)) {
    fVar6 = fVar5;
  }
LAB_0010b678:
  *(float *)((long)param_1 + 0x24) = fVar6;
  return fVar6;
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
  lVar3 = FUN_00125e50(0x138);
  lVar2 = lVar3;
  if (DAT_001386d8 != 0) {
    *(long *)(DAT_001386c8 + 0x108) = lVar3;
    lVar2 = DAT_001386d8;
  }
  DAT_001386d8 = lVar2;
  *(int **)(lVar3 + 0x130) = param_1;
  *(undefined2 *)(lVar3 + 0x1a) = 0x101;
  uVar1 = DAT_0013862f;
  *(undefined8 *)(lVar3 + 0x10) = 0;
  *(char **)(lVar3 + 0x30) = "ThisShouldBeSetToSomethingElse";
  *(undefined1 *)(lVar3 + 0x28) = uVar1;
  *(undefined8 *)(lVar3 + 0x20) = 0x3f8000000000000a;
  *(undefined8 *)(lVar3 + 0x128) = 0;
  *(undefined8 *)(lVar3 + 0x120) = 0;
  DAT_001386c8 = lVar3;
  return;
}




// Function: get_new_pred_chk_op @ 0xb9b0

void get_new_pred_chk_op(undefined8 param_1,undefined8 param_2)

{
  long lVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  
  if (DAT_001386a0 == 0) {
    DAT_001386a0 = FUN_00114bc0(&DAT_00128bfd);
    if (DAT_001386a0 == 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("entry_and != NULL","tree.c",0x5f8,"get_new_pred_chk_op");
    }
  }
  if (DAT_001386c8 != 0) {
    if ((*(uint *)(DAT_001386c8 + 0x10) & 0xfffffffb) == 1) {
      puVar2 = (undefined8 *)FUN_0010b8a0();
      puVar2[1] = &DAT_00128b4f;
      *puVar2 = FUN_00108990;
      puVar2[6] = 0;
      puVar2[7] = 0;
      puVar2[2] = 0x300000003;
      *(undefined4 *)(puVar2 + 3) = 0;
      *(undefined1 *)((long)puVar2 + 0x1c) = 0;
    }
    else if (*(uint *)(DAT_001386c8 + 0x10) == 0) {
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




// Function: print_tree @ 0xbac0

void print_tree(undefined8 param_1,long param_2,int param_3)

{
  char cVar1;
  short sVar2;
  uint uVar3;
  ulong uVar4;
  undefined *puVar5;
  char *pcVar6;
  short sVar7;
  int *piVar8;
  undefined *puVar9;
  char *pcVar10;
  int iVar11;
  
  if (param_2 == 0) {
    return;
  }
  do {
    iVar11 = 0;
    if (0 < param_3) {
      do {
        __fprintf_chk(param_1,1,&DAT_00128c29);
        iVar11 = iVar11 + 1;
      } while (iVar11 != param_3);
    }
    __fprintf_chk(param_1,1,"pred=[");
    if (*(long *)(param_2 + 0x30) == 0) {
      __fprintf_chk(param_1,1,&DAT_0012d8da);
    }
    else {
      __fprintf_chk(param_1,1,"%s %s");
    }
    puVar9 = &DAT_00128c13;
    sVar2 = (short)*(undefined4 *)(param_2 + 0x14);
    if (sVar2 != 0) {
      sVar7 = 1;
      iVar11 = 0;
      puVar9 = &DAT_00136100;
      while( true ) {
        iVar11 = iVar11 + 1;
        if ((sVar2 == sVar7) || (sVar7 == -1)) break;
        sVar7 = *(short *)(puVar9 + 0x20);
        puVar9 = puVar9 + 0x10;
      }
      puVar9 = (&PTR_DAT_00136108)[(long)iVar11 * 2];
    }
    puVar5 = &DAT_00128c13;
    sVar2 = (short)*(undefined4 *)(param_2 + 0x10);
    if (sVar2 != 0) {
      sVar7 = 1;
      iVar11 = 0;
      puVar5 = &DAT_00136080;
      while( true ) {
        iVar11 = iVar11 + 1;
        if ((sVar2 == sVar7) || (sVar7 == -1)) break;
        sVar7 = *(short *)(puVar5 + 0x20);
        puVar5 = puVar5 + 0x10;
      }
      puVar5 = (&PTR_DAT_00136088)[(long)iVar11 * 2];
    }
    __fprintf_chk(param_1,1,"] type=%s prec=%s",puVar5,puVar9);
    pcVar10 = "";
    piVar8 = &DAT_00138020;
    if (*(char *)(param_2 + 0x18) == '\0') {
      pcVar10 = "no ";
    }
    uVar4 = 0;
    do {
      if (*(int *)(param_2 + 0x20) == *piVar8) {
        pcVar6 = *(char **)(uVar4 * 0x10 + 0x138028);
        goto LAB_0010bc5b;
      }
      uVar3 = (int)uVar4 + 1;
      uVar4 = (ulong)uVar3;
      piVar8 = piVar8 + 4;
    } while (uVar3 != 0xb);
    pcVar6 = "unknown";
LAB_0010bc5b:
    __fprintf_chk((double)*(float *)(param_2 + 0x24),param_1,1,
                  " cost=%s est_success_rate=%#.4g %sside effects ",pcVar6,pcVar10);
    if ((*(ulong *)(param_2 + 0x18) & 0xffffff0000) != 0) {
      __fprintf_chk(param_1,1,"Needs ");
      if (*(char *)(param_2 + 0x1a) == '\0') {
        if (*(char *)(param_2 + 0x1c) != '\0') {
          pcVar10 = "";
LAB_0010bdc5:
          __fprintf_chk(param_1,1,"%sinode",pcVar10);
          cVar1 = *(char *)(param_2 + 0x1b);
          goto joined_r0x0010beb0;
        }
        if (*(char *)(param_2 + 0x1b) == '\0') goto LAB_0010bc89;
        pcVar10 = "";
      }
      else {
        __fprintf_chk(param_1,1,"stat");
        if (*(char *)(param_2 + 0x1c) != '\0') {
          pcVar10 = ",";
          goto LAB_0010bdc5;
        }
        cVar1 = *(char *)(param_2 + 0x1b);
joined_r0x0010beb0:
        if (cVar1 == '\0') goto LAB_0010bc89;
        pcVar10 = ",";
      }
      __fprintf_chk(param_1,1,"%stype",pcVar10);
    }
LAB_0010bc89:
    iVar11 = 0;
    __fprintf_chk(param_1,1,"\n");
    if (0 < param_3) {
      do {
        __fprintf_chk(param_1,1,&DAT_00128c29);
        iVar11 = iVar11 + 1;
      } while (iVar11 != param_3);
    }
    if (*(long *)(param_2 + 0x110) == 0) {
      if (*(long *)(param_2 + 0x118) == 0) {
        pcVar10 = "no children.\n";
        goto LAB_0010bec2;
      }
      __fprintf_chk(param_1,1,"no left.\n");
    }
    else {
      __fprintf_chk(param_1,1,"left:\n");
      print_tree(param_1,*(undefined8 *)(param_2 + 0x110),param_3 + 1);
    }
    iVar11 = 0;
    if (0 < param_3) {
      do {
        __fprintf_chk(param_1,1,&DAT_00128c29);
        iVar11 = iVar11 + 1;
      } while (iVar11 != param_3);
    }
    if (*(long *)(param_2 + 0x118) == 0) {
      pcVar10 = "no right.\n";
LAB_0010bec2:
      __fprintf_chk(param_1,1,pcVar10);
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




// Function: predlist_merge_sort @ 0xbef0

void predlist_merge_sort(long *param_1,long *param_2)

{
  float fVar1;
  float fVar2;
  uint uVar3;
  uint uVar4;
  long lVar5;
  long lVar6;
  long lVar7;
  long lVar8;
  long lVar9;
  long lVar10;
  
  lVar10 = *param_1;
  if (lVar10 == 0) {
    return;
  }
  if (((byte)DAT_00138660 & 8) == 0) {
    FUN_0010b590(lVar10);
  }
  else {
    __fprintf_chk(stderr,1,&DAT_00128ca3,"predlist before merge sort");
    FUN_0010bac0(stderr,*param_1,2);
    lVar10 = *param_1;
    FUN_0010b590(lVar10);
    if (lVar10 == 0) {
      lVar8 = 0;
      lVar9 = 0;
      goto LAB_0010bfd3;
    }
  }
  lVar8 = 0;
  lVar9 = 0;
  do {
    lVar6 = *(long *)(lVar10 + 0x110);
    *param_1 = lVar6;
    *(undefined8 *)(lVar10 + 0x110) = 0;
    if (lVar8 != 0) {
      uVar3 = *(uint *)(*(long *)(lVar10 + 0x118) + 0x20);
      lVar7 = lVar8;
      do {
        uVar4 = *(uint *)(*(long *)(lVar7 + 0x118) + 0x20);
        if (uVar4 != uVar3) {
          if (uVar4 < uVar3) goto LAB_0010bfb1;
LAB_0010bf50:
          lVar5 = *(long *)(lVar7 + 0x110);
          *(long *)(lVar10 + 0x110) = lVar5;
          if (lVar5 == 0) {
            lVar9 = lVar10;
          }
          *(long *)(lVar7 + 0x110) = lVar10;
          goto joined_r0x0010bf6f;
        }
        fVar1 = *(float *)(*(long *)(lVar7 + 0x118) + 0x24);
        fVar2 = *(float *)(*(long *)(lVar10 + 0x118) + 0x24);
        if (fVar1 == fVar2) goto LAB_0010bf50;
        if (*(int *)(lVar7 + 0x14) == 2) {
          if (fVar1 < fVar2) goto LAB_0010bf50;
        }
        else if (fVar2 <= fVar1) goto LAB_0010bf50;
LAB_0010bfb1:
        lVar7 = *(long *)(lVar7 + 0x110);
      } while (lVar7 != 0);
    }
    *(long *)(lVar10 + 0x110) = lVar8;
    lVar8 = lVar10;
    if (lVar9 == 0) {
      lVar9 = lVar10;
    }
joined_r0x0010bf6f:
    lVar10 = lVar6;
  } while (lVar6 != 0);
LAB_0010bfd3:
  if (((byte)DAT_00138660 & 8) != 0) {
    __fprintf_chk(stderr,1,&DAT_00128ca3,"predlist after merge sort");
    FUN_0010bac0(stderr,lVar8,2);
  }
  FUN_0010b590(lVar8);
  *(long *)(lVar9 + 0x110) = *param_2;
  *param_2 = lVar8;
  param_1[1] = 0;
  *param_1 = 0;
  return;
}




// Function: opt_expr @ 0xc110

undefined8 opt_expr(long *param_1)

{
  int iVar1;
  undefined *puVar2;
  undefined8 uVar3;
  long lVar4;
  long lVar5;
  undefined1 uVar6;
  uint uVar7;
  long *plVar8;
  long *plVar9;
  ulong uVar10;
  long lVar11;
  long lVar12;
  char *pcVar13;
  int *piVar14;
  int iVar15;
  undefined4 uVar16;
  long lVar17;
  undefined8 uVar18;
  long in_FS_OFFSET;
  long local_118;
  long local_110;
  long local_108;
  long local_100;
  long local_f8 [23];
  long local_40;
  
  lVar11 = *param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_118 = 0;
  local_110 = 0;
  local_108 = 0;
  local_100 = 0;
  if (lVar11 == 0) {
    uVar18 = 0;
    goto LAB_0010c371;
  }
  plVar8 = local_f8;
  do {
    plVar8[1] = 0;
    plVar9 = plVar8 + 2;
    *plVar8 = 0;
    plVar8 = plVar9;
  } while (plVar9 != local_f8 + 0x16);
  lVar4 = lVar11;
  lVar5 = *(long *)(lVar11 + 0x110);
  if (*(long *)(lVar11 + 0x110) == 0) {
    uVar16 = 3;
    plVar8 = param_1;
    lVar17 = lVar11;
  }
  else {
    do {
      lVar17 = lVar5;
      lVar12 = lVar4;
      lVar4 = lVar17;
      lVar5 = *(long *)(lVar17 + 0x110);
    } while (*(long *)(lVar17 + 0x110) != 0);
    uVar16 = *(undefined4 *)(lVar12 + 0x14);
    plVar8 = (long *)(lVar12 + 0x110);
  }
  if (*(int *)(lVar17 + 0x10) == 3) {
    if (((byte)DAT_00138660 & 9) == 0) goto LAB_0010c1e6;
LAB_0010c448:
    __fprintf_chk(stderr,1,"Normalized Eval Tree:\n");
    FUN_0010bac0(stderr,*param_1,0);
  }
  else {
    FUN_0010ad30(lVar17,uVar16,plVar8);
    if (((byte)DAT_00138660 & 9) != 0) goto LAB_0010c448;
  }
  lVar11 = *param_1;
  if (lVar11 != 0) {
LAB_0010c1e6:
    iVar1 = *(int *)(lVar11 + 0x10);
    iVar15 = 0;
    if (iVar1 == 3) {
      iVar15 = *(int *)(lVar11 + 0x14);
    }
    uVar18 = 0;
    do {
      if ((iVar1 == 3) && (*(int *)(lVar11 + 0x14) != iVar15)) {
        lVar11 = FUN_0010ad30(lVar11,iVar15,param_1);
      }
      plVar8 = *(long **)(lVar11 + 0x118);
      uVar7 = *(uint *)(plVar8 + 2);
      puVar2 = (undefined *)*plVar8;
      if (uVar7 == 2) {
        uVar6 = opt_expr(plVar8 + 0x23);
        *(undefined1 *)(plVar8 + 3) = uVar6;
        plVar8 = *(long **)(lVar11 + 0x118);
LAB_0010c3c0:
        if ((char)plVar8[3] != '\0') {
LAB_0010c3c7:
          uVar18 = 1;
          FUN_0010c0b0(local_f8,0xb,&local_108,&local_118,param_1);
        }
LAB_0010c3e5:
        param_1 = (long *)(lVar11 + 0x110);
      }
      else {
        if (2 < uVar7) {
          if (uVar7 != 3) {
            uVar18 = dcgettext(0,"oops -- invalid expression type!",5);
                    /* WARNING: Subroutine does not return */
            error(1,0,uVar18);
          }
          uVar6 = opt_expr();
          *(undefined1 *)(plVar8 + 3) = uVar6;
          plVar8 = *(long **)(lVar11 + 0x118);
          goto LAB_0010c3c0;
        }
        if (iVar15 == 1) goto LAB_0010c3c0;
        if ((char)plVar8[3] != '\0') goto LAB_0010c3c7;
        if ((((puVar2 != PTR_FUN_00137f50 && puVar2 != PTR_FUN_00137fd8) &&
              puVar2 != PTR_FUN_00137f28) && (puVar2 != PTR_FUN_00137f78)) &&
           ((((puVar2 == PTR_FUN_00137f40 || puVar2 == PTR_FUN_00137f48) || DAT_00138688 == 0) ||
             puVar2 == PTR_FUN_00137f60 || ((puVar2 == PTR_FUN_00137ff0 || ((int)plVar8[4] != 0)))))
           ) {
          if (puVar2 == PTR_FUN_00137f30) {
            *param_1 = *(long *)(lVar11 + 0x110);
            *(long *)(lVar11 + 0x110) = local_118;
            local_118 = lVar11;
            if (local_110 == 0) {
              local_110 = lVar11;
            }
            goto LAB_0010c3ec;
          }
          if ((1 < DAT_00138688) &&
             (((uVar7 = *(uint *)(plVar8 + 4), uVar7 - 1 < 2 &&
               (*(char *)((long)plVar8 + 0x1a) == '\0')) || (DAT_00138688 != 2)))) {
            if (((byte)DAT_00138660 & 8) != 0) {
              __fprintf_chk(stderr,1,"-O%d: categorising predicate ",DAT_00138688);
              uVar3 = *(undefined8 *)(*(long *)(lVar11 + 0x118) + 8);
              if (*(long *)(*(long *)(lVar11 + 0x118) + 0x30) == 0) {
                __fprintf_chk(stderr,1,&DAT_0012d8da,uVar3);
              }
              else {
                __fprintf_chk(stderr,1,"%s %s",uVar3);
              }
              piVar14 = &DAT_00138020;
              uVar10 = 0;
              do {
                if (*(int *)(*(long *)(lVar11 + 0x118) + 0x20) == *piVar14) {
                  pcVar13 = *(char **)(uVar10 * 0x10 + 0x138028);
                  goto LAB_0010c617;
                }
                uVar7 = (int)uVar10 + 1;
                uVar10 = (ulong)uVar7;
                piVar14 = piVar14 + 4;
              } while (uVar7 != 0xb);
              pcVar13 = "unknown";
LAB_0010c617:
              __fprintf_chk(stderr,1," by cost (%s)\n",pcVar13);
              uVar7 = *(uint *)(*(long *)(lVar11 + 0x118) + 0x20);
            }
            uVar10 = (ulong)uVar7;
            plVar8 = local_f8 + uVar10 * 2;
            *param_1 = *(long *)(lVar11 + 0x110);
            *(long *)(lVar11 + 0x110) = *plVar8;
            *plVar8 = lVar11;
            if (local_f8[uVar10 * 2 + 1] == 0) {
              local_f8[uVar10 * 2 + 1] = lVar11;
            }
            goto LAB_0010c3ec;
          }
          goto LAB_0010c3e5;
        }
        if (((byte)DAT_00138660 & 8) != 0) {
          __fprintf_chk(stderr,1,"-O%d: promoting cheap predicate ",DAT_00138688);
          uVar3 = *(undefined8 *)(*(long *)(lVar11 + 0x118) + 8);
          if (*(long *)(*(long *)(lVar11 + 0x118) + 0x30) == 0) {
            __fprintf_chk(stderr,1,&DAT_0012d8da,uVar3);
          }
          else {
            __fprintf_chk(stderr,1,"%s %s",uVar3);
          }
          __fprintf_chk(stderr,1," into name_list\n");
        }
        *param_1 = *(long *)(lVar11 + 0x110);
        *(long *)(lVar11 + 0x110) = local_108;
        local_108 = lVar11;
        if (local_100 == 0) {
          local_100 = lVar11;
        }
      }
LAB_0010c3ec:
      lVar11 = *param_1;
      if (lVar11 == 0) goto LAB_0010c359;
      iVar1 = *(int *)(lVar11 + 0x10);
    } while( true );
  }
  uVar18 = 0;
LAB_0010c359:
  FUN_0010c0b0(local_f8,0xb,&local_108,&local_118,param_1);
LAB_0010c371:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar18;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: do_arm_swaps @ 0xc690

undefined8 do_arm_swaps(long *param_1)

{
  float fVar1;
  float fVar2;
  long lVar3;
  long lVar4;
  char cVar5;
  uint uVar6;
  uint uVar7;
  long lVar8;
  undefined *puVar9;
  char *pcVar10;
  uint uVar11;
  long extraout_RDX;
  long lVar12;
  ulong uVar13;
  long lVar14;
  bool bVar15;
  
  if (param_1 == (long *)0x0) {
    return 0;
  }
  do {
    uVar11 = (uint)DAT_00138660 & 8;
    pcVar10 = "Not a binary operation";
    if ((int)param_1[2] == 3) {
      lVar8 = param_1[0x22];
      uVar13 = DAT_00138660;
      do {
        uVar11 = (uint)uVar13;
        if ((lVar8 == 0) || (lVar14 = param_1[0x23], lVar14 == 0)) {
          uVar11 = uVar11 & 8;
          pcVar10 = "Doesn\'t have two arms";
          break;
        }
        lVar8 = *(long *)(lVar8 + 0x118);
        if (lVar8 == 0) {
          uVar11 = uVar11 & 8;
          pcVar10 = "Left arm has no child on RHS";
          break;
        }
        if (*(char *)(lVar8 + 0x18) != '\0') {
LAB_0010c774:
          uVar11 = uVar11 & 8;
          pcVar10 = "Left subtree has side-effects";
          break;
        }
        cVar5 = FUN_0010b430(*(undefined8 *)(lVar8 + 0x110));
        uVar11 = (uint)uVar13;
        if (cVar5 != '\0') goto LAB_0010c774;
        lVar12 = *(long *)(lVar8 + 0x118);
        cVar5 = FUN_0010b430(lVar12);
        uVar11 = (uint)uVar13;
        if (cVar5 != '\0') goto LAB_0010c774;
        if (*(char *)(lVar14 + 0x18) != '\0') {
LAB_0010c7ed:
          uVar11 = uVar11 & 8;
          pcVar10 = "Right subtree has side-effects";
          break;
        }
        lVar3 = *(long *)(lVar14 + 0x110);
        cVar5 = FUN_0010b430(lVar3);
        uVar11 = (uint)uVar13;
        if (cVar5 != '\0') goto LAB_0010c7ed;
        lVar4 = *(long *)(lVar14 + 0x118);
        cVar5 = FUN_0010b430(lVar4);
        uVar11 = (uint)uVar13;
        if (cVar5 != '\0') goto LAB_0010c7ed;
        uVar6 = 0;
        if (extraout_RDX != 0) {
          uVar6 = FUN_0010b0c0(extraout_RDX);
        }
        uVar11 = 0;
        if (lVar12 != 0) {
          uVar11 = FUN_0010b0c0(lVar12);
        }
        if (uVar6 < *(uint *)(lVar8 + 0x20)) {
          uVar6 = *(uint *)(lVar8 + 0x20);
        }
        if (uVar6 < uVar11) {
          uVar6 = uVar11;
        }
        uVar7 = 0;
        if (lVar3 != 0) {
          uVar7 = FUN_0010b0c0(lVar3);
        }
        uVar11 = 0;
        if (lVar4 != 0) {
          uVar11 = FUN_0010b0c0(lVar4);
        }
        if (uVar7 < *(uint *)(lVar14 + 0x20)) {
          uVar7 = *(uint *)(lVar14 + 0x20);
        }
        if (uVar7 < uVar11) {
          uVar7 = uVar11;
        }
        uVar11 = (uint)uVar13 & 8;
        if ((int)uVar6 < (int)uVar7) {
          pcVar10 = "efficient as-is";
          break;
        }
        if (uVar6 == uVar7) {
          fVar1 = *(float *)(lVar8 + 0x24);
          fVar2 = *(float *)(lVar14 + 0x24);
          if ((uVar13 & 8) == 0) {
            puVar9 = (undefined *)*param_1;
            if (puVar9 == PTR_FUN_00137ff0) {
LAB_0010c9be:
              if (fVar2 < fVar1) goto LAB_0010c8a2;
              pcVar10 = "Operation is OR; right success rate >= left";
              break;
            }
          }
          else {
            __fprintf_chk((double)fVar1,(double)fVar2,stderr,1,"Success rates: l=%f, r=%f\n");
            puVar9 = (undefined *)*param_1;
            uVar11 = (uint)DAT_00138660 & 8;
            uVar13 = DAT_00138660;
            if (puVar9 == PTR_FUN_00137ff0) goto LAB_0010c9be;
          }
          if (puVar9 != PTR_FUN_00137f48) {
            pcVar10 = "Not \'AND\' or \'OR\'";
            break;
          }
          if (fVar2 <= fVar1) {
            pcVar10 = "Operation is AND; right success rate <= left";
            break;
          }
        }
LAB_0010c8a2:
        bVar15 = uVar11 != 0;
        uVar11 = 0;
        if (bVar15) {
          __fprintf_chk(stderr,1,"Performing arm swap on:\n");
          FUN_0010bac0(stderr,param_1,0);
          uVar11 = (uint)DAT_00138660 & 8;
          uVar13 = DAT_00138660;
        }
        lVar8 = param_1[0x22];
        lVar12 = param_1[2];
        lVar14 = *(long *)(lVar8 + 0x118);
        *(long *)(lVar8 + 0x118) = param_1[0x23];
        param_1[0x23] = lVar14;
        pcVar10 = "Not a binary operation";
      } while ((int)lVar12 == 3);
    }
    if (uVar11 != 0) {
      __fprintf_chk(stderr,1,"Not an arm swap candidate (%s):\n",pcVar10);
      FUN_0010bac0(stderr,param_1,0);
    }
    cVar5 = do_arm_swaps(param_1[0x22]);
    if ((cVar5 == '\0') && (cVar5 = do_arm_swaps(param_1[0x23]), cVar5 == '\0')) {
      return 0;
    }
  } while( true );
}




// Function: build_expression_tree @ 0xc9f0

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
  DAT_001386d8 = (long *)0x0;
  DAT_001386c0 = param_2 + lVar4 * 8;
  local_5c[0] = param_1;
  local_4c = param_3;
  if (param_3 < param_1) {
    while (cVar3 = FUN_0010e120(*(undefined8 *)(param_2 + lVar4 * 8),1), cVar3 == '\0') {
      DAT_001386b8 = DAT_001386b8 + 1;
      local_4c = local_4c + 1;
      if (local_5c[0] <= local_4c) break;
      lVar4 = (long)local_4c;
    }
  }
  lVar4 = FUN_00114bc0(&DAT_0012ab1b);
  lVar5 = FUN_00114bc0(")");
  lVar6 = FUN_00114bc0("print");
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
  FUN_00112c60(lVar4,param_2,local_5c);
  plVar1 = DAT_001386d8;
  *(undefined **)(DAT_001386c8 + 8) = &DAT_0012ab1b;
  *(undefined1 *)((long)plVar1 + 0x29) = 1;
  FUN_00114ba0(param_2,local_5c[0]);
  FUN_00109e60(DAT_001386c8);
  if (local_4c < local_5c[0]) {
    do {
      DAT_00138618 = 0;
      cVar3 = FUN_0010e120(*(undefined8 *)(param_2 + (long)local_4c * 8),0);
      uVar9 = *(undefined8 *)(param_2 + (long)local_4c * 8);
      if (cVar3 == '\0') {
        uVar8 = dcgettext(0,"paths must precede expression: `%s\'",5);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar8,uVar9);
      }
      piVar7 = (int *)FUN_00114bc0(uVar9);
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
      lVar4 = DAT_001386c8;
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
      *(undefined8 *)(DAT_001386c8 + 8) = uVar9;
      if (local_4c != iVar2) {
        uVar8 = *(undefined8 *)(param_2 + (long)iVar2 * 8);
      }
      *(undefined8 *)(lVar4 + 0x30) = uVar8;
      FUN_00109e60();
      FUN_00109e60(DAT_001386d8);
    } while (local_4c < local_5c[0]);
  }
  FUN_00114bb0(param_2,local_5c[0],DAT_001386c8,DAT_001386d8);
  plVar1 = DAT_001386d8;
  if (DAT_001386d8[0x21] == 0) {
    local_48 = DAT_001386d8;
    DAT_001386c8 = 0;
    DAT_001386d8 = (long *)0x0;
    free(plVar1);
    FUN_00111e20(lVar6,param_2,local_5c);
    *(char **)(DAT_001386c8 + 8) = "-print";
    FUN_00109e60();
    FUN_00109e60(DAT_001386d8);
  }
  else {
    cVar3 = thunk_FUN_0010e0fd();
    if (cVar3 == '\0') {
      local_48 = DAT_001386d8;
      DAT_001386d8 = (long *)DAT_001386d8[0x21];
      FUN_00109e60();
      free(local_48);
    }
    else {
      FUN_00112a60(lVar5,param_2,local_5c);
      lVar4 = DAT_001386c8;
      *(char **)(DAT_001386c8 + 8) = ")";
      *(undefined1 *)(lVar4 + 0x29) = 1;
      FUN_00109e60();
      FUN_00111e20(lVar6,param_2,local_5c);
      lVar4 = DAT_001386c8;
      *(char **)(DAT_001386c8 + 8) = "-print";
      *(undefined1 *)(lVar4 + 0x29) = 1;
      FUN_00109e60();
      FUN_00109e60(DAT_001386d8);
    }
  }
  if (((byte)DAT_00138660 & 9) != 0) {
    __fprintf_chk(stderr,1,"Predicate List:\n");
    FUN_00109cd0(stderr,DAT_001386d8);
  }
  FUN_00114a70(DAT_001386d8);
  FUN_00109e60(DAT_001386d8);
  local_48 = DAT_001386d8;
  uVar9 = FUN_0010a8c0(&local_48,0,0);
  DAT_001386d0 = uVar9;
  FUN_0010b590(uVar9);
  if (local_48 == (long *)0x0) {
    if (((byte)DAT_00138660 & 9) != 0) {
      __fprintf_chk(stderr,1,"Eval Tree:\n");
      FUN_0010bac0(stderr,DAT_001386d0,0);
      uVar9 = DAT_001386d0;
    }
    FUN_0010ae30(uVar9);
    FUN_0010c110(&DAT_001386d0);
    uVar9 = DAT_001386d0;
    FUN_0010a840(DAT_001386d0,1);
    FUN_0010c690(uVar9);
    uVar9 = DAT_001386d0;
    FUN_0010a840(DAT_001386d0,1);
    if (((byte)DAT_00138660 & 9) != 0) {
      __fprintf_chk(stderr,1,"Optimized Eval Tree:\n");
      FUN_0010bac0(stderr,DAT_001386d0,0);
      __fprintf_chk(stderr,1,"Optimized command line:\n");
      FUN_00109de0(stderr,DAT_001386d0);
      __fprintf_chk(stderr,1,"\n");
      uVar9 = DAT_001386d0;
    }
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      return uVar9;
    }
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  if ((code *)*local_48 == FUN_00108be0) {
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




// Function: do_complete_pending_execdirs @ 0xd130

void do_complete_pending_execdirs(long *param_1)

{
  if (param_1 == (long *)0x0) {
    return;
  }
  do {
    while( true ) {
      if (DAT_00138608 == '\0') {
                    /* WARNING: Subroutine does not return */
        __assert_fail("state.execdirs_outstanding","util.c",0x18c,"do_complete_pending_execdirs");
      }
      do_complete_pending_execdirs(param_1[0x22]);
      if (((((undefined *)*param_1 != PTR_FUN_00137fc0) &&
           ((undefined *)*param_1 != PTR_FUN_00137f20)) || ((char)param_1[7] == '\0')) ||
         (((int)param_1[0x1a] == 0 ||
          (FUN_001159f0(param_1[0x1f],FUN_0010d220,param_1 + 7), param_1[0x1f] == DAT_001385c0))))
      break;
      FUN_001242c0();
      free((void *)param_1[0x1f]);
      param_1[0x1f] = 0;
      param_1 = (long *)param_1[0x23];
      if (param_1 == (long *)0x0) {
        return;
      }
    }
    param_1 = (long *)param_1[0x23];
  } while (param_1 != (long *)0x0);
  return;
}




// Function: exec_cb @ 0xd220

undefined8 exec_cb(long param_1)

{
  FUN_00115430(param_1 + 8,param_1 + 0x60);
  return 0;
}




// Function: complete_pending_execs @ 0xd240

void complete_pending_execs(long *param_1)

{
  if (param_1 == (long *)0x0) {
    return;
  }
  do {
    complete_pending_execs(param_1[0x22]);
    if ((((code *)*param_1 == thunk_FUN_0010a040) && ((char)param_1[7] != '\0')) &&
       ((int)param_1[0x1a] != 0)) {
      FUN_00115430(param_1 + 8,param_1 + 0x13);
    }
    param_1 = (long *)param_1[0x23];
  } while (param_1 != (long *)0x0);
  return;
}




// Function: insert_primary_noarg @ 0xd520

void insert_primary_noarg(long param_1)

{
  long lVar1;
  long lVar2;
  long *plVar3;
  
  lVar1 = *(long *)(param_1 + 0x18);
  if (lVar1 != 0) {
    plVar3 = (long *)FUN_0010b9b0(param_1,0);
    lVar2 = *(long *)(param_1 + 8);
    *plVar3 = lVar1;
    plVar3[1] = lVar2;
    plVar3[7] = 0;
    plVar3[2] = 1;
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_0010d2b0();
}




// Function: usage @ 0xd570

void usage(int param_1)

{
  FILE *pFVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  char *pcVar4;
  long lVar5;
  undefined **ppuVar6;
  
  uVar2 = DAT_00138ed0;
  if (param_1 != 0) {
    uVar3 = dcgettext(0,"Try \'%s --help\' for more information.\n",5);
    __fprintf_chk(stderr,1,uVar3,uVar2);
                    /* WARNING: Subroutine does not return */
    exit(param_1);
  }
  uVar3 = dcgettext(0,"Usage: %s [-H] [-L] [-P] [-Olevel] [-D debugopts] [path...] [expression]\n",5
                   );
  ppuVar6 = &PTR_DAT_00136180;
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
  pFVar1 = stdout;
  pcVar4 = (char *)dcgettext(0,"Valid arguments for -D:\n",5);
  fputs(pcVar4,pFVar1);
  lVar5 = 0;
  do {
    if (lVar5 == 0) {
      __fprintf_chk(stdout,1,&DAT_00128644,"",*ppuVar6);
    }
    else {
      __fprintf_chk(stdout,1,&DAT_00128644,&DAT_001295e8,*ppuVar6);
      pFVar1 = stdout;
      if (lVar5 + 1 == 9) {
        pcVar4 = (char *)dcgettext(0,
                                   "\nUse \'-D help\' for a description of the options, or see find(1)\n\n"
                                   ,5);
        fputs(pcVar4,pFVar1);
        FUN_001175f0(stdout,DAT_00138ed0);
                    /* WARNING: Subroutine does not return */
        exit(0);
      }
    }
    ppuVar6 = ppuVar6 + 3;
    lVar5 = lVar5 + 1;
  } while( true );
}




// Function: get_statinfo @ 0xd820

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 get_statinfo(undefined8 param_1,undefined8 param_2,long param_3)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  int *piVar4;
  
  if (DAT_001385e4 == '\0') {
    iVar1 = (*DAT_00138670)(param_2,param_3);
    if (iVar1 != 0) {
      piVar4 = __errno_location();
      iVar1 = *piVar4;
      if ((DAT_0013862e != '\0') && (iVar1 == 2)) {
        return 0xffffffff;
      }
      if (DAT_00138618 != '\0') {
        DAT_00138618 = 1;
        return 0xffffffff;
      }
      uVar2 = FUN_001239f0(0,DAT_0013868c,param_1);
                    /* WARNING: Subroutine does not return */
      error(0,iVar1,&DAT_0012d8da,uVar2);
    }
    iVar1 = *(int *)(param_3 + 0x18);
    if (iVar1 == 0) {
      uVar2 = FUN_001239f0(0,DAT_0013868c,param_2);
      uVar3 = dcgettext(0,"WARNING: file %s appears to have mode 0000",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar3,uVar2);
    }
  }
  else {
    iVar1 = *(int *)(param_3 + 0x18);
  }
  DAT_001385e8 = iVar1;
  _DAT_001385e4 = 0x101;
  return 0;
}




// Function: check_nofollow @ 0xd920

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 check_nofollow(void)

{
  int iVar1;
  undefined4 extraout_var;
  undefined4 extraout_var_00;
  undefined8 uVar2;
  long in_FS_OFFSET;
  double dVar3;
  utsname uStack_1a8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = uname(&uStack_1a8);
  if (iVar1 == 0) {
    dVar3 = strtod(uStack_1a8.release,(char **)0x0);
    iVar1 = strcmp("Linux",uStack_1a8.sysname);
    if (iVar1 == 0) {
      uVar2 = CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),_DAT_0012a2d0 <= (float)dVar3);
      goto LAB_0010d995;
    }
    iVar1 = strcmp("FreeBSD",uStack_1a8.sysname);
    if (iVar1 == 0) {
      uVar2 = CONCAT71((int7)(CONCAT44(extraout_var_00,iVar1) >> 8),
                       (float)s__ffF_sharefile_c_0012a2d3._1_4_ <= (float)dVar3);
      goto LAB_0010d995;
    }
  }
  uVar2 = 1;
LAB_0010d995:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: record_initial_cwd @ 0xda20

void record_initial_cwd(void)

{
  int iVar1;
  undefined8 uVar2;
  int *piVar3;
  char *pcVar4;
  char *pcVar5;
  
  DAT_001385c0 = (int *)FUN_00125a50(0x10);
  iVar1 = FUN_00124230(DAT_001385c0);
  if (iVar1 == 0) {
    return;
  }
  pcVar4 = "";
  pcVar5 = "";
  if (*DAT_001385c0 < 0) {
    pcVar4 = *(char **)(DAT_001385c0 + 2);
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




// Function: cleanup @ 0xdab0

void cleanup(void)

{
  long *plVar1;
  code *pcVar2;
  char cVar3;
  int iVar4;
  long *plVar5;
  int *piVar6;
  undefined8 uVar7;
  char *pcVar8;
  long *plVar9;
  char *pcVar10;
  
  plVar5 = (long *)FUN_0010b580();
  plVar9 = plVar5;
  if (plVar5 == (long *)0x0) {
    FUN_0010eb20(DAT_00138610);
  }
  else {
    do {
      if (plVar9[0x22] != 0) {
        FUN_0010d2e0(plVar9[0x22],FUN_0010d240);
      }
      FUN_0010d240(plVar9);
      plVar1 = plVar9 + 0x23;
      plVar9 = (long *)*plVar1;
    } while ((long *)*plVar1 != (long *)0x0);
    if (DAT_00138608 != '\0') {
      uVar7 = FUN_0010b580();
      FUN_0010d130(uVar7);
      DAT_00138608 = '\0';
    }
    FUN_0010eb20(DAT_00138610);
    do {
      if (plVar5[0x22] != 0) {
        FUN_0010d2e0(plVar5[0x22],FUN_0010d0f0);
      }
      pcVar2 = (code *)*plVar5;
      if (((pcVar2 == FUN_0010ff90 || pcVar2 == FUN_00109020) || pcVar2 == FUN_00108fe0) ||
         (pcVar2 == (code *)PTR_FUN_00137f58)) {
        plVar5[8] = 0;
      }
      plVar5 = (long *)plVar5[0x23];
    } while (plVar5 != (long *)0x0);
  }
  iVar4 = FUN_00124290(DAT_001385c0);
  if (iVar4 != 0) {
    pcVar8 = "";
    pcVar10 = "";
    if (*DAT_001385c0 < 0) {
      pcVar8 = *(char **)(DAT_001385c0 + 2);
      pcVar10 = ": ";
      if (pcVar8 == (char *)0x0) {
        pcVar8 = "";
        pcVar10 = pcVar8;
      }
    }
    uVar7 = dcgettext(0,"Failed to restore initial working directory%s%s",5);
    piVar6 = __errno_location();
                    /* WARNING: Subroutine does not return */
    error(0,*piVar6,uVar7,pcVar10,pcVar8);
  }
  FUN_001242c0(DAT_001385c0);
  free(DAT_001385c0);
  DAT_001385c0 = (int *)0x0;
  cVar3 = FUN_00116220();
  if (cVar3 != '\0') {
    FUN_00116240();
    FUN_001161f0();
  }
  iVar4 = FUN_001180c0(stdout);
  if (iVar4 == -1) {
    piVar6 = __errno_location();
    iVar4 = *piVar6;
    uVar7 = FUN_001239f0(0,DAT_0013868c,"standard output");
                    /* WARNING: Subroutine does not return */
    error(0,iVar4,&DAT_0012d8da,uVar7);
  }
  return;
}




// Function: optionh_stat @ 0xdcd0

int optionh_stat(char *param_1,stat *param_2)

{
  int iVar1;
  int *piVar2;
  
  if ((DAT_001385f8 < 0) && (DAT_001385f8 != -100)) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("state.cwd_dir_fd >= 0","util.c",0x25d,"optionh_stat");
  }
  if (DAT_001385e0 != 0) {
    iVar1 = fstatat(DAT_001385f8,param_1,param_2,0x100);
    return iVar1;
  }
  iVar1 = fstatat(DAT_001385f8,param_1,param_2,0);
  if (iVar1 != 0) {
    piVar2 = __errno_location();
    if ((*piVar2 == 2) || (*piVar2 == 0x14)) {
      if (((byte)DAT_00138660 & 2) != 0) {
        __fprintf_chk(stderr,1,"fallback_stat(): stat(%s) failed; falling back on lstat()\n",param_1
                     );
      }
      iVar1 = fstatat(DAT_001385f8,param_1,param_2,0x100);
      return iVar1;
    }
  }
  return iVar1;
}




// Function: debug_stat @ 0xdec0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void debug_stat(char *param_1,stat *param_2)

{
  _DAT_001386e0 = _DAT_001386e0 + 1;
  __fprintf_chk(stderr,1,"debug_stat (%s)\n",param_1);
  if (DAT_00138668 == 1) {
    FUN_0010ddb0(param_1,param_2);
    return;
  }
  if (DAT_00138668 == 2) {
    FUN_0010dcd0(param_1,param_2);
    return;
  }
  if (DAT_00138668 != 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("0","util.c",0x2a5,"debug_stat");
  }
  if ((DAT_001385f8 < 0) && (DAT_001385f8 != -100)) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("(state.cwd_dir_fd >= 0) || (state.cwd_dir_fd==AT_FDCWD)","util.c",0x28d,
                  "optionp_stat");
  }
  fstatat(DAT_001385f8,param_1,param_2,0x100);
  return;
}




// Function: digest_mode @ 0xdfe0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4
digest_mode(uint *param_1,undefined8 param_2,undefined8 param_3,long param_4,undefined4 param_5)

{
  uint uVar1;
  int iVar2;
  
  uVar1 = *param_1;
  if (uVar1 == 0) {
    if ((char)param_5 != '\0') {
      DAT_001385e8 = 0;
      _DAT_001385e4 = 0;
      return param_5;
    }
    iVar2 = FUN_0010d820(param_2,param_3,param_4);
    if (iVar2 == 0) {
      DAT_001385e8 = *(uint *)(param_4 + 0x18);
      *param_1 = DAT_001385e8;
      _DAT_001385e4 = CONCAT11(1,DAT_001385e4);
      return 1;
    }
  }
  else {
    if (((uVar1 & 0xf000) != 0xa000) ||
       ((DAT_00138668 != 1 && ((DAT_00138668 != 2 || (DAT_001385e0 != 0)))))) {
      _DAT_001385e4 = CONCAT11(1,DAT_001385e4);
      DAT_001385e8 = uVar1;
      *(uint *)(param_4 + 0x18) = uVar1;
      return 1;
    }
    iVar2 = FUN_0010d820(param_2,param_3,param_4);
    if (iVar2 == 0) {
      DAT_001385e8 = *(uint *)(param_4 + 0x18);
      *param_1 = DAT_001385e8;
      _DAT_001385e4 = CONCAT11(1,DAT_001385e4);
      return 1;
    }
  }
  return 0;
}




// Function: looks_like_expression @ 0xe120

ulong looks_like_expression(char *param_1,uint param_2)

{
  undefined7 uVar1;
  
  uVar1 = (undefined7)
          ((ulong)((long)&switchD_0010e13f::switchdataD_0012a220 +
                  (long)(int)(&switchD_0010e13f::switchdataD_0012a220)[(byte)(*param_1 - 0x21)]) >>
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




// Function: process_leading_options @ 0xe180

int process_leading_options(int param_1,long param_2)

{
  byte bVar1;
  FILE *__stream;
  int iVar2;
  long lVar3;
  char *pcVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  ushort **ppuVar7;
  int *piVar8;
  ulong uVar9;
  char *pcVar10;
  undefined *puVar11;
  char *__s1;
  int iVar12;
  undefined **ppuVar13;
  long in_FS_OFFSET;
  char *local_50;
  char local_42 [2];
  long local_40;
  
  iVar12 = 1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (1 < param_1) {
    do {
      lVar3 = (long)iVar12;
      pcVar10 = *(char **)(param_2 + lVar3 * 8);
      iVar2 = strcmp("-H",pcVar10);
      if (iVar2 == 0) {
        FUN_00114af0(2);
      }
      else {
        iVar2 = strcmp("-L",pcVar10);
        if (iVar2 == 0) {
          FUN_00114af0(1);
        }
        else {
          iVar2 = strcmp("-P",pcVar10);
          if (iVar2 == 0) {
            FUN_00114af0(0);
          }
          else {
            iVar2 = strcmp("--",pcVar10);
            if (iVar2 == 0) {
              iVar12 = iVar12 + 1;
              break;
            }
            iVar2 = strcmp("-D",pcVar10);
            if (iVar2 == 0) {
              iVar12 = iVar12 + 1;
              if (param_1 <= iVar12) {
                pcVar10 = "Missing argument after the -D option.";
LAB_0010e4bc:
                uVar5 = dcgettext(0,pcVar10,5);
                    /* WARNING: Subroutine does not return */
                error(0,0,uVar5);
              }
              local_50 = (char *)0x0;
              local_42[0] = ',';
              local_42[1] = '\0';
              pcVar10 = *(char **)(param_2 + 8 + lVar3 * 8);
              pcVar4 = strtok_r(pcVar10,local_42,&local_50);
              if (pcVar4 == (char *)0x0) {
                pcVar10 = "Empty argument to the -D option.";
                goto LAB_0010e4bc;
              }
              do {
                __s1 = "exec";
                lVar3 = 0;
                ppuVar13 = &PTR_DAT_00136198;
                while (iVar2 = strcmp(__s1,pcVar4), iVar2 != 0) {
                  lVar3 = lVar3 + 1;
                  if (lVar3 == 9) {
                    uVar5 = FUN_001239f0(0,DAT_0013868c,pcVar10);
                    uVar6 = dcgettext(0,"Ignoring unrecognised debug flag %s",5);
                    /* WARNING: Subroutine does not return */
                    error(0,0,uVar6,uVar5);
                  }
                  __s1 = *ppuVar13;
                  ppuVar13 = ppuVar13 + 3;
                }
                DAT_00138660 = DAT_00138660 | (long)(int)(&DAT_00136188)[lVar3 * 6];
                pcVar4 = strtok_r((char *)0x0,local_42,&local_50);
                __stream = stdout;
              } while (pcVar4 != (char *)0x0);
              if ((DAT_00138660 & 0x10) != 0) {
                pcVar10 = (char *)dcgettext(0,"Valid arguments for -D:\n",5);
                fputs(pcVar10,__stream);
                puVar11 = &DAT_0012ac2c;
                pcVar10 = "Show diagnostic information relating to -exec, -execdir, -ok and -okdir";
                for (ppuVar13 = &PTR_DAT_00136198;
                    __fprintf_chk(stdout,1,"%-10s %s\n",puVar11,pcVar10),
                    ppuVar13 != (undefined **)&DAT_00136258; ppuVar13 = ppuVar13 + 3) {
                  pcVar10 = ppuVar13[2];
                  puVar11 = *ppuVar13;
                }
                    /* WARNING: Subroutine does not return */
                exit(0);
              }
            }
            else {
              iVar2 = strncmp("-O",pcVar10,2);
              if (iVar2 != 0) break;
              pcVar4 = pcVar10 + 2;
              bVar1 = pcVar10[2];
              if (bVar1 == 0) {
                uVar5 = dcgettext(0,
                                  "The -O option must be immediately followed by a decimal integer",
                                  5);
                    /* WARNING: Subroutine does not return */
                error(1,0,uVar5);
              }
              ppuVar7 = __ctype_b_loc();
              if ((*(byte *)((long)*ppuVar7 + (ulong)bVar1 * 2 + 1) & 8) == 0) {
                uVar5 = dcgettext(0,"Please specify a decimal number immediately after -O",5);
                    /* WARNING: Subroutine does not return */
                error(1,0,uVar5);
              }
              piVar8 = __errno_location();
              iVar2 = *piVar8;
              *piVar8 = 0;
              uVar9 = strtoul(pcVar4,&local_50,10);
              if (uVar9 == 0) {
                if (pcVar4 == local_50) {
                  uVar5 = dcgettext(0,"Please specify a decimal number immediately after -O",5);
                    /* WARNING: Subroutine does not return */
                  error(1,0,uVar5);
                }
                if (*local_50 != '\0') {
LAB_0010e51a:
                  uVar5 = dcgettext(0,"Invalid optimisation level %s",5);
                    /* WARNING: Subroutine does not return */
                  error(1,0,uVar5,pcVar4);
                }
              }
              else {
                if (*local_50 != '\0') goto LAB_0010e51a;
                if (uVar9 == 0xffffffffffffffff) {
                  if (*piVar8 != 0) {
                    uVar5 = dcgettext(0,"Invalid optimisation level %s",5);
                    /* WARNING: Subroutine does not return */
                    error(1,*piVar8,uVar5,pcVar4);
                  }
LAB_0010e475:
                  uVar5 = dcgettext(0,
                                    "Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate."
                                    ,5);
                    /* WARNING: Subroutine does not return */
                  error(1,0,uVar5,uVar9);
                }
                if (0xffff < uVar9) goto LAB_0010e475;
              }
              DAT_00138688 = (undefined2)uVar9;
              *piVar8 = iVar2;
            }
          }
        }
      }
      iVar12 = iVar12 + 1;
    } while (iVar12 < param_1);
  }
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return iVar12;
}




// Function: set_option_defaults @ 0xe630

void set_option_defaults(undefined2 *param_1)

{
  undefined1 uVar1;
  int iVar2;
  char *pcVar3;
  undefined8 uVar4;
  long lVar5;
  long in_FS_OFFSET;
  timeval local_28;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  pcVar3 = getenv("POSIXLY_CORRECT");
  *(bool *)((long)param_1 + 0x11) = pcVar3 != (char *)0x0;
  uVar1 = FUN_0010d920();
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
  iVar2 = gettimeofday(&local_28,(__timezone_ptr_t)0x0);
  if (iVar2 == 0) {
    lVar5 = local_28.tv_usec * 1000;
  }
  else {
    local_28.tv_sec = time((time_t *)0x0);
    if (local_28.tv_sec == -1) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("t != (time_t)-1","util.c",0x3d2,"now");
    }
    lVar5 = 0;
  }
  *(__time_t *)(param_1 + 0xc) = local_28.tv_sec;
  *(__time_t *)(param_1 + 0x14) = local_28.tv_sec + -0x15180;
  *(undefined2 *)((long)param_1 + 0xd) = 0;
  *(long *)(param_1 + 0x10) = lVar5;
  *(long *)(param_1 + 0x18) = lVar5;
  *(undefined1 *)(param_1 + 0x1c) = 0;
  *(uint *)(param_1 + 0x1e) = (-(uint)(*(char *)((long)param_1 + 0x11) == '\0') & 0x200) + 0x200;
  *(undefined8 *)(param_1 + 0x20) = 0;
  param_1[0x34] = 2;
  pcVar3 = getenv("FIND_BLOCK_SIZE");
  if (pcVar3 == (char *)0x0) {
    *(undefined1 *)(param_1 + 6) = 0;
    FUN_00114af0(0);
    *(undefined4 *)(param_1 + 0x36) = 8;
    *(undefined8 *)(param_1 + 0x38) = 0;
    *(undefined1 *)(param_1 + 0x3c) = 0;
    if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  uVar4 = dcgettext(0,
                    "The environment variable FIND_BLOCK_SIZE is not supported, the only thing that affects the block size is the POSIXLY_CORRECT environment variable"
                    ,5);
                    /* WARNING: Subroutine does not return */
  error(1,0,uVar4);
}




// Function: apply_predicate @ 0xe7b0

undefined8 apply_predicate(undefined8 param_1,long param_2,undefined8 *param_3)

{
  int iVar1;
  undefined8 uVar2;
  
  param_3[0x24] = param_3[0x24] + 1;
  if (((((param_3[3] & 0xffffff0000) == 0) ||
       ((((*(char *)((long)param_3 + 0x1a) == '\0' || (DAT_001385e4 != '\0')) &&
         ((*(char *)((long)param_3 + 0x1b) == '\0' || (DAT_001385e5 != '\0')))) &&
        ((*(char *)((long)param_3 + 0x1c) == '\0' ||
         (((*(long *)(param_2 + 8) != 0 && (DAT_001385e5 != '\0')) &&
          ((*(uint *)(param_2 + 0x18) & 0xf000) != 0x4000)))))))) ||
      (iVar1 = FUN_0010d820(param_1,DAT_001385f0,param_2), iVar1 == 0)) &&
     (uVar2 = (*(code *)*param_3)(param_1,param_2,param_3), (char)uVar2 != '\0')) {
    param_3[0x25] = param_3[0x25] + 1;
    return uVar2;
  }
  return 0;
}




// Function: is_exec_in_local_dir @ 0xe880

uint is_exec_in_local_dir(undefined *param_1,undefined8 param_2,undefined4 param_3)

{
  undefined8 in_RAX;
  
  return (uint)CONCAT71((int7)((ulong)in_RAX >> 8),param_1 == PTR_FUN_00137f20) |
         CONCAT31((int3)((uint)param_3 >> 8),param_1 == PTR_FUN_00137fc0);
}




// Function: fatal_target_file_error @ 0xe920

void fatal_target_file_error(undefined4 param_1,undefined8 param_2)

{
  undefined8 uVar1;
  
  if (DAT_00138618 == '\0') {
    uVar1 = FUN_001239f0(0,DAT_0013868c,param_2);
                    /* WARNING: Subroutine does not return */
    error(1,param_1,&DAT_0012d8da,uVar1);
  }
  DAT_00138618 = 1;
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: fatal_nontarget_file_error @ 0xe9b0

void fatal_nontarget_file_error(undefined4 param_1,undefined8 param_2)

{
  undefined8 uVar1;
  
  DAT_00138618 = 0;
  uVar1 = FUN_001239f0(0,DAT_0013868c,param_2);
                    /* WARNING: Subroutine does not return */
  error(1,param_1,&DAT_0012d8da,uVar1);
}




// Function: entry_comparator @ 0xea00

undefined8 entry_comparator(long *param_1,long *param_2)

{
  if (param_1[1] != param_2[1]) {
    return 0;
  }
  return CONCAT71((int7)((ulong)*param_2 >> 8),*param_1 == *param_2);
}




// Function: entry_hashfunc @ 0xea30

ulong entry_hashfunc(ulong *param_1,ulong param_2)

{
  return (*param_1 ^ param_1[1]) % param_2;
}




// Function: sharefile_init @ 0xea90

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
      lVar2 = FUN_0011b2f0(0xb,0,FUN_0010ea30,FUN_0010ea00,FUN_0010ea50);
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




// Function: sharefile_destroy @ 0xeb20

void sharefile_destroy(undefined8 *param_1)

{
  free((void *)*param_1);
  FUN_0011b4b0(param_1[1]);
  return;
}




// Function: sharefile_fopen @ 0xeb40

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
      __stream = (FILE *)FUN_001182a0(param_2,*param_1);
      __ptr[3] = (__dev_t)__stream;
      if (__stream == (FILE *)0x0) {
        FUN_0010ea50(__ptr);
        _Var6 = 0;
      }
      else {
        iVar2 = fileno(__stream);
        if (iVar2 < 0) {
                    /* WARNING: Subroutine does not return */
          __assert_fail("fd >= 0","sharefile.c",0xa6,"sharefile_fopen");
        }
        FUN_00117990(iVar2,1);
        iVar2 = fstat(iVar2,&local_c8);
        if (iVar2 < 0) {
          _Var6 = 0;
          FUN_0010ea50(__ptr);
        }
        else {
          uVar1 = param_1[1];
          *__ptr = local_c8.st_dev;
          __ptr[1] = local_c8.st_ino;
          lVar4 = FUN_0011b060(uVar1,__ptr);
          if (lVar4 == 0) {
            lVar4 = FUN_0011b950(param_1[1],__ptr);
            if (lVar4 == 0) {
              piVar5 = __errno_location();
              iVar2 = *piVar5;
              FUN_0010ea50(__ptr);
              *piVar5 = iVar2;
              _Var6 = 0;
            }
            else {
              _Var6 = __ptr[3];
            }
          }
          else {
            FUN_0010ea50(__ptr);
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




// Function: checked_fprintf @ 0xece0

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
    FUN_0010e970(*piVar2,*(undefined8 *)(param_9 + 0x10));
  }
  if (local_c0 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: format_date @ 0xedc0

byte * format_date(long param_1,undefined8 param_2,int param_3)

{
  uint *__src;
  size_t sVar1;
  byte bVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  bool bVar5;
  undefined1 auVar6 [8];
  bool bVar7;
  uint uVar8;
  byte *pbVar9;
  size_t sVar10;
  tm *__tp;
  char *__format;
  char *__s;
  ushort **ppuVar11;
  uint uVar12;
  ulong uVar13;
  byte bVar14;
  long lVar15;
  long in_FS_OFFSET;
  long local_f0;
  long local_e0;
  long local_c8;
  undefined8 local_c0;
  tm local_b8;
  undefined1 local_74 [8];
  undefined4 local_6c;
  uint local_68;
  undefined4 uStack_64;
  long local_40;
  
  auVar6 = local_74;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_c8 = param_1;
  local_c0 = param_2;
  if (param_3 == 0x2b) {
    local_74 = (undefined1  [8])0x64252d6d252d5925;
    local_6c = 0x54252b;
    uVar8 = __snprintf_chk(&local_68,0x20,1,0x20,".%09ld0",param_2);
    bVar7 = true;
LAB_0010ef60:
    bVar5 = bVar7;
    __tp = localtime(&local_c8);
    if (__tp != (tm *)0x0) {
      uVar13 = (ulong)(int)uVar8;
      sVar10 = strlen(local_74);
      __format = (char *)FUN_00125a50(sVar10 + 2);
      *__format = '_';
      sVar10 = strlen(local_74);
      memcpy(__format + 1,local_74,sVar10 + 1);
      uVar3 = __tp->tm_sec;
      uVar4 = __tp->tm_min;
      local_b8.tm_hour = __tp->tm_hour;
      local_b8.tm_mday = __tp->tm_mday;
      local_b8.tm_mon = __tp->tm_mon;
      local_b8.tm_year = __tp->tm_year;
      local_b8.tm_wday = __tp->tm_wday;
      local_b8.tm_yday = __tp->tm_yday;
      local_b8.tm_isdst = __tp->tm_isdst;
      local_b8._36_4_ = *(undefined4 *)&__tp->field_0x24;
      local_b8.tm_gmtoff = __tp->tm_gmtoff;
      local_b8.tm_zone = __tp->tm_zone;
      local_b8.tm_sec = uVar3 + 0xb;
      if (10 < (int)uVar3) {
        local_b8.tm_sec = uVar3 + -0xb;
      }
      local_b8.tm_min = uVar4;
      if (DAT_00138b08 == (byte *)0x0) {
        DAT_00138b00 = 1;
        DAT_00138b08 = (byte *)FUN_00125a50(1);
      }
      while ((pbVar9 = DAT_00138b08,
             sVar10 = strftime((char *)DAT_00138b08,DAT_00138b00,__format,__tp), sVar10 == 0 ||
             (DAT_00138b00 <= sVar10))) {
        DAT_00138b08 = (byte *)FUN_00125ca0(pbVar9,&DAT_00138b00,1);
      }
      sVar1 = uVar13 + 1 + sVar10;
      DAT_00138b08 = (byte *)FUN_00125aa0(pbVar9,sVar1);
      DAT_00138b00 = sVar1;
      __s = (char *)FUN_00125a50(sVar1);
      strftime(__s,DAT_00138b00,__format,&local_b8);
      pbVar9 = DAT_00138b08;
      bVar14 = *DAT_00138b08;
      if (bVar14 != 0) {
        local_f0 = 0;
        lVar15 = 0;
        local_e0 = 0;
        bVar7 = false;
        do {
          bVar2 = __s[lVar15];
          if (bVar2 == 0) goto LAB_0010f210;
          if (bVar2 != bVar14) {
            ppuVar11 = __ctype_b_loc();
            if (((*(byte *)((long)*ppuVar11 + (ulong)bVar14 * 2 + 1) & 8) == 0) ||
               ((*(byte *)((long)*ppuVar11 + (ulong)bVar2 * 2 + 1) & 8) == 0)) goto LAB_0010f210;
            if (bVar7) {
              if (local_f0 != lVar15 - local_e0) goto LAB_0010f210;
              local_f0 = local_f0 + 1;
            }
            else {
              local_f0 = 1;
              bVar7 = true;
              local_e0 = lVar15;
            }
          }
          lVar15 = lVar15 + 1;
          bVar14 = pbVar9[lVar15];
        } while (bVar14 != 0);
        if ((__s[lVar15] == '\0') && (local_f0 == 2)) {
          ppuVar11 = __ctype_b_loc();
          local_e0 = local_e0 + 2;
          __src = (uint *)(pbVar9 + local_e0);
          if ((*(byte *)((long)*ppuVar11 + (ulong)(byte)*__src * 2 + 1) & 8) == 0) {
            if (sVar1 != sVar10 + 1 + uVar13) {
                    /* WARNING: Subroutine does not return */
              __assert_fail("end_of_seconds + ns_size + suffix_len == final_len","print.c",0x225,
                            "do_time_format");
            }
            memmove(pbVar9 + uVar13 + local_e0,__src,(sVar10 + 1) - local_e0);
            if (uVar8 < 8) {
              if ((uVar13 & 4) == 0) {
                if (uVar8 != 0) {
                  *(byte *)__src = (byte)local_68;
                  if ((uVar13 & 2) != 0) {
                    *(undefined2 *)((long)__src + ((ulong)uVar8 - 2)) =
                         *(undefined2 *)((long)&local_6c + (ulong)uVar8 + 2);
                  }
                }
              }
              else {
                *__src = local_68;
                *(undefined4 *)((long)__src + ((ulong)uVar8 - 4)) =
                     *(undefined4 *)((long)&local_6c + (ulong)uVar8);
              }
            }
            else {
              *(ulong *)__src = CONCAT44(uStack_64,local_68);
              *(undefined8 *)((long)__src + ((ulong)uVar8 - 8)) =
                   *(undefined8 *)(local_74 + (ulong)uVar8 + 4);
              lVar15 = (long)__src - ((ulong)(__src + 2) & 0xfffffffffffffff8);
              uVar8 = uVar8 + (int)lVar15 & 0xfffffff8;
              if (7 < uVar8) {
                uVar12 = 0;
                do {
                  uVar13 = (ulong)uVar12;
                  uVar12 = uVar12 + 8;
                  *(undefined8 *)(((ulong)(__src + 2) & 0xfffffffffffffff8) + uVar13) =
                       *(undefined8 *)((long)&local_68 + (uVar13 - lVar15));
                } while (uVar12 < uVar8);
              }
            }
          }
        }
      }
LAB_0010f210:
      free(__format);
      free(__s);
      pbVar9 = pbVar9 + 1;
      goto LAB_0010eeee;
    }
  }
  else {
    local_74[1] = (char)param_3;
    local_74[0] = 0x25;
    local_74._3_5_ = auVar6._3_5_;
    local_74[2] = 0;
    if ((param_3 - 0x40U < 0x19) && ((0x1180001UL >> ((ulong)(param_3 - 0x40U) & 0x3f) & 1) != 0)) {
      bVar7 = true;
      bVar5 = true;
      uVar8 = __snprintf_chk(&local_68,0x20,1,0x20,".%09ld0",param_2);
    }
    else {
      local_68 = local_68 & 0xffffff00;
      bVar7 = false;
      bVar5 = false;
      uVar8 = 0;
    }
    if (param_3 != 0x40) goto LAB_0010ef60;
  }
  lVar15 = -local_c8;
  if (-local_c8 < 0) {
    lVar15 = local_c8;
  }
  pbVar9 = (byte *)FUN_0011bb50(lVar15,&DAT_00138b21,0,1,1);
  if (pbVar9 < &DAT_00138b21) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("p > buf","print.c",0x2b9,"format_date");
  }
  if ((byte *)0x138e36 < pbVar9) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("p < (buf + (sizeof buf))","print.c",0x2ba,"format_date");
  }
  if (local_c8 < 0) {
    pbVar9[-1] = 0x2d;
    pbVar9 = pbVar9 + -1;
  }
  if (bVar5) {
    sVar10 = strlen((char *)pbVar9);
    if ((byte *)0x316 < pbVar9 + (sVar10 - 0x138b20)) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("sizeof buf > used","print.c",0x2c7,"format_date");
    }
    uVar13 = 0x316 - (long)(pbVar9 + (sVar10 - 0x138b20));
    sVar10 = strlen((char *)&local_68);
    if (uVar13 <= sVar10) {
                    /* WARNING: Subroutine does not return */
      error(0,0,"charsprinted=%ld but remaining=%lu: ns_buf=%s",(long)(int)uVar8,uVar13,&local_68);
    }
    strcat((char *)pbVar9,(char *)&local_68);
  }
LAB_0010eeee:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return pbVar9;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: ctime_format @ 0xf400

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
    puVar5 = (undefined *)FUN_0010edc0(local_18,local_10,0x40);
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
    iVar3 = __snprintf_chk(&DAT_00138700,0x400,1,0x400,"%3s %3s %2d %02d:%02d:%02d.%09ld0 %04d",
                           (&PTR_DAT_001362c0)[iVar3],(&PTR_DAT_00136260)[iVar1],ptVar4->tm_mday,
                           iVar2,ptVar4->tm_min,ptVar4->tm_sec,local_10,ptVar4->tm_year + 0x76c);
    if (0x3ff < iVar3) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("nout < TIME_BUF_LEN","print.c",0x302,"ctime_format");
    }
    puVar5 = &DAT_00138700;
  }
  return puVar5;
}




// Function: do_fprintf @ 0xf620

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void do_fprintf(long param_1,int *param_2,void *param_3,ulong *param_4)

{
  int iVar1;
  uint uVar2;
  size_t sVar3;
  int *piVar4;
  passwd *ppVar5;
  char *pcVar6;
  undefined8 uVar7;
  char *pcVar8;
  ulong uVar9;
  undefined8 uVar10;
  long in_FS_OFFSET;
  undefined8 local_360;
  stat local_358;
  undefined1 local_2c8 [10];
  undefined1 local_2be;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = *param_2;
  if (iVar1 == 1) {
    iVar1 = param_2[4];
    sVar3 = fwrite(*(void **)(param_2 + 2),1,(long)iVar1,*(FILE **)(param_1 + 8));
    if (sVar3 < (ulong)(long)iVar1) {
      piVar4 = __errno_location();
      FUN_0010e970(*piVar4,*(undefined8 *)(param_1 + 0x10));
    }
    iVar1 = FUN_001180c0(*(undefined8 *)(param_1 + 8));
    if (iVar1 == 0) goto switchD_0010f702_caseD_1;
    goto LAB_0010f6a6;
  }
  if (iVar1 != 2) {
    if (iVar1 == 0) goto switchD_0010f702_caseD_25;
    goto switchD_0010f702_caseD_1;
  }
  switch((char)param_2[1]) {
  case '\0':
    uVar7 = dcgettext(0,"error: %s at end of format string",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar7,&DAT_0012a494);
  default:
    goto switchD_0010f702_caseD_1;
  case '%':
switchD_0010f702_caseD_25:
    iVar1 = param_2[4];
    sVar3 = fwrite(*(void **)(param_2 + 2),1,(long)iVar1,*(FILE **)(param_1 + 8));
    if (sVar3 < (ulong)(long)iVar1) goto LAB_0010f6a6;
    goto switchD_0010f702_caseD_1;
  case 'D':
    uVar9 = *param_4;
    uVar10 = 1;
    uVar7 = 1;
    break;
  case 'F':
    pcVar8 = (char *)FUN_001108e0(param_4,param_3);
    goto LAB_0010f7c4;
  case 'G':
    goto switchD_0010f702_caseD_47;
  case 'H':
    pcVar8 = (char *)FUN_00125a50((long)(DAT_001385fc + 1));
    sVar3 = (size_t)DAT_001385fc;
    memcpy(pcVar8,param_3,sVar3);
    pcVar8[sVar3] = '\0';
    FUN_0010ece0(param_1,*(undefined8 *)(param_2 + 2),pcVar8);
    if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) goto LAB_0010fa3a;
    goto LAB_0010f924;
  case 'M':
    FUN_00118290(param_4,local_2c8);
    local_2be = 0;
    FUN_0010ece0(param_1,*(undefined8 *)(param_2 + 2),local_2c8);
    goto switchD_0010f702_caseD_1;
  case 'P':
    pcVar8 = "";
    if (0 < (int)DAT_001385e0) {
      pcVar8 = (char *)((long)DAT_001385fc + (long)param_3) +
               (*(char *)((long)DAT_001385fc + (long)param_3) == '/');
    }
LAB_0010f7c4:
    iVar1 = FUN_00116e80(*(undefined8 *)(param_1 + 8),*(undefined8 *)(param_1 + 0x20),
                         *(undefined1 *)(param_1 + 0x18),*(undefined8 *)(param_2 + 2),pcVar8);
joined_r0x0010fa78:
    if (iVar1 < 0) {
LAB_0010f6a6:
      piVar4 = __errno_location();
      if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
        FUN_0010e970(*piVar4,*(undefined8 *)(param_1 + 0x10));
        return;
      }
      goto LAB_0010fa3a;
    }
    goto switchD_0010f702_caseD_1;
  case 'S':
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      FUN_0010ece0(param_1,*(undefined8 *)(param_2 + 2));
      return;
    }
    goto LAB_0010fa3a;
  case 'U':
    goto switchD_0010f702_caseD_55;
  case 'Y':
    uVar2 = (uint)param_4[3] & 0xf000;
    if (uVar2 != 0xa000) {
      pcVar8 = "f";
      if ((uVar2 != 0x8000) && (pcVar8 = "d", uVar2 != 0x4000)) goto LAB_0010f96d;
      goto LAB_0010f9ae;
    }
    iVar1 = fstatat(DAT_001385f8,DAT_001385f0,&local_358,0);
    if (iVar1 != 0) {
      piVar4 = __errno_location();
      uVar7 = *(undefined8 *)(param_2 + 2);
      iVar1 = *piVar4;
      if ((iVar1 == 2) || (iVar1 == 0x14)) {
        FUN_0010ece0(param_1,uVar7,"N");
      }
      else {
        if (iVar1 != 0x28) {
          FUN_0010ece0(param_1,uVar7,&DAT_0012ad2b);
          uVar7 = FUN_0010e8a0(0,param_3);
                    /* WARNING: Subroutine does not return */
          error(0,*piVar4,&DAT_0012d8da,uVar7);
        }
        FUN_0010ece0(param_1,uVar7,&DAT_00129634);
      }
      goto switchD_0010f702_caseD_1;
    }
    pcVar8 = "f";
    uVar2 = local_358.st_mode & 0xf000;
    if (((uVar2 != 0x8000) && (pcVar8 = "d", uVar2 != 0x4000)) &&
       ((pcVar8 = "l", uVar2 != 0xa000 &&
        ((((pcVar8 = "s", uVar2 != 0xc000 && (pcVar8 = "b", uVar2 != 0x6000)) &&
          (pcVar8 = "c", uVar2 != 0x2000)) && (pcVar8 = "U", uVar2 == 0x1000)))))) {
      pcVar8 = "p";
    }
    goto LAB_0010f778;
  case 'Z':
    iVar1 = (*DAT_00138680)(DAT_001385f8,DAT_001385f0,&local_360);
    if (iVar1 < 0) {
      FUN_0010ece0(param_1,*(undefined8 *)(param_2 + 2),"");
      uVar7 = FUN_0010e8a0(0,param_3);
      uVar10 = dcgettext(0,"getfilecon failed: %s",5);
      piVar4 = __errno_location();
                    /* WARNING: Subroutine does not return */
      error(0,*piVar4,uVar10,uVar7);
    }
    FUN_0010ece0(param_1,*(undefined8 *)(param_2 + 2),local_360);
    freecon(local_360);
    goto switchD_0010f702_caseD_1;
  case 'a':
    pcVar8 = (char *)FUN_0010f400(param_4[9],param_4[10]);
    if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) goto LAB_0010fa3a;
    goto LAB_0010f9c1;
  case 'b':
    uVar9 = param_4[8];
    uVar10 = 0x200;
    uVar7 = 0x200;
    break;
  case 'c':
    pcVar8 = (char *)FUN_0010f400(param_4[0xd],param_4[0xe]);
    if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) goto LAB_0010fa3a;
    goto LAB_0010f9c1;
  case 'd':
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      uVar7 = *(undefined8 *)(param_2 + 2);
      uVar2 = DAT_001385e0;
LAB_0010fac5:
      FUN_0010ece0(param_1,uVar7,uVar2);
      return;
    }
    goto LAB_0010fa3a;
  case 'f':
    pcVar8 = (char *)FUN_00117b90(param_3);
LAB_0010fbce:
    iVar1 = FUN_00116e80(*(undefined8 *)(param_1 + 8),*(undefined8 *)(param_1 + 0x20),
                         *(undefined1 *)(param_1 + 0x18),*(undefined8 *)(param_2 + 2),pcVar8);
    goto joined_r0x0010fb12;
  case 'g':
    ppVar5 = (passwd *)getgrgid((__gid_t)param_4[4]);
    if (ppVar5 != (passwd *)0x0) {
LAB_0010f9ef:
      *(undefined1 *)(*(long *)(param_2 + 2) + (long)param_2[4]) = 0x73;
      if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) goto LAB_0010fa3a;
      uVar7 = *(undefined8 *)(param_2 + 2);
      pcVar8 = ppVar5->pw_name;
      goto LAB_0010f9c5;
    }
    goto switchD_0010f702_caseD_47;
  case 'h':
    pcVar8 = (char *)FUN_00125f30(param_3);
    sVar3 = strlen(pcVar8);
    for (pcVar6 = pcVar8 + (sVar3 - 1); pcVar8 <= pcVar6; pcVar6 = pcVar6 + -1) {
      if (*pcVar6 != '/') {
        if ((pcVar8 < pcVar6) && (pcVar6[1] == '/')) {
          pcVar6[1] = '\0';
        }
        break;
      }
    }
    pcVar6 = strrchr(pcVar8,0x2f);
    if (pcVar6 != (char *)0x0) {
      *pcVar6 = '\0';
      goto LAB_0010fbce;
    }
    iVar1 = FUN_00116e80(*(undefined8 *)(param_1 + 8),*(undefined8 *)(param_1 + 0x20),
                         *(undefined1 *)(param_1 + 0x18),*(undefined8 *)(param_2 + 2),&DAT_0012c379)
    ;
    goto joined_r0x0010fb12;
  case 'i':
    uVar9 = param_4[1];
    uVar10 = 1;
    uVar7 = 1;
    break;
  case 'k':
    uVar9 = param_4[8];
    uVar10 = 0x400;
    uVar7 = 0x200;
    break;
  case 'l':
    if ((param_4[3] & 0xf000) == 0xa000) {
      pcVar8 = (char *)FUN_00117650(DAT_001385f8,DAT_001385f0);
      if (pcVar8 != (char *)0x0) goto LAB_0010fbce;
      piVar4 = __errno_location();
      FUN_0010e8c0(*piVar4,param_3);
      DAT_00138604 = 1;
    }
    pcVar8 = (char *)0x0;
    iVar1 = FUN_00116e80(*(undefined8 *)(param_1 + 8),*(undefined8 *)(param_1 + 0x20),
                         *(undefined1 *)(param_1 + 0x18),*(undefined8 *)(param_2 + 2),"");
joined_r0x0010fb12:
    if (iVar1 < 0) {
      piVar4 = __errno_location();
      FUN_0010e970(*piVar4,*(undefined8 *)(param_1 + 0x10));
    }
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
LAB_0010f924:
      free(pcVar8);
      return;
    }
    goto LAB_0010fa3a;
  case 'm':
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      uVar7 = *(undefined8 *)(param_2 + 2);
      uVar2 = (uint)param_4[3] & 0xfff;
      goto LAB_0010fac5;
    }
    goto LAB_0010fa3a;
  case 'n':
    uVar9 = param_4[2];
    uVar10 = 1;
    uVar7 = 1;
    break;
  case 'p':
    iVar1 = FUN_00116e80(*(undefined8 *)(param_1 + 8),*(undefined8 *)(param_1 + 0x20),
                         *(undefined1 *)(param_1 + 0x18),*(undefined8 *)(param_2 + 2),param_3);
    goto joined_r0x0010fa78;
  case 's':
    uVar9 = param_4[6];
    uVar10 = 1;
    uVar7 = 1;
    break;
  case 't':
    pcVar8 = (char *)FUN_0010f400(param_4[0xb],param_4[0xc]);
    if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) goto LAB_0010fa3a;
    goto LAB_0010f9c1;
  case 'u':
    ppVar5 = getpwuid(*(__uid_t *)((long)param_4 + 0x1c));
    if (ppVar5 != (passwd *)0x0) goto LAB_0010f9ef;
    goto switchD_0010f702_caseD_55;
  case 'y':
    pcVar8 = "f";
    uVar2 = (uint)param_4[3] & 0xf000;
    if (((uVar2 != 0x8000) && (pcVar8 = "d", uVar2 != 0x4000)) && (pcVar8 = "l", uVar2 != 0xa000)) {
LAB_0010f96d:
      pcVar8 = "s";
      if ((uVar2 != 0xc000) &&
         (((pcVar8 = "b", uVar2 != 0x6000 && (pcVar8 = "c", uVar2 != 0x2000)) &&
          (pcVar8 = "U", uVar2 == 0x1000)))) {
        pcVar8 = "p";
      }
    }
LAB_0010f9ae:
    if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) goto LAB_0010fa3a;
LAB_0010f9c1:
    uVar7 = *(undefined8 *)(param_2 + 2);
LAB_0010f9c5:
    FUN_0010ece0(param_1,uVar7,pcVar8);
    return;
  }
LAB_0010f770:
  pcVar8 = (char *)FUN_0011bb50(uVar9,local_2c8,0,uVar7,uVar10);
LAB_0010f778:
  FUN_0010ece0(param_1,*(undefined8 *)(param_2 + 2),pcVar8);
switchD_0010f702_caseD_1:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
LAB_0010fa3a:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
switchD_0010f702_caseD_55:
  uVar9 = (ulong)*(uint *)((long)param_4 + 0x1c);
  uVar10 = 1;
  uVar7 = 1;
  goto LAB_0010f770;
switchD_0010f702_caseD_47:
  uVar9 = (ulong)(uint)param_4[4];
  uVar10 = 1;
  uVar7 = 1;
  goto LAB_0010f770;
}




// Function: pred_fprintf @ 0xff90

undefined8 pred_fprintf(undefined8 param_1,long param_2,long param_3)

{
  char cVar1;
  undefined8 uVar2;
  int *piVar3;
  undefined8 uVar4;
  
  piVar3 = *(int **)(param_3 + 0x38);
  if (piVar3 != (int *)0x0) {
    param_3 = param_3 + 0x38;
    do {
      while ((*piVar3 != 2 || (*(char *)((long)piVar3 + 5) == '\0'))) {
        FUN_0010f620(param_3,piVar3,param_1,param_2);
        piVar3 = *(int **)(piVar3 + 6);
        if (piVar3 == (int *)0x0) {
          return 1;
        }
      }
      cVar1 = (char)piVar3[1];
      if (cVar1 == 'C') {
        uVar2 = *(undefined8 *)(param_2 + 0x70);
        uVar4 = *(undefined8 *)(param_2 + 0x68);
LAB_0010ffe6:
        uVar2 = FUN_0010edc0(uVar4,uVar2);
        FUN_0010ece0(param_3,*(undefined8 *)(piVar3 + 2),uVar2);
      }
      else {
        if ('C' < cVar1) {
          if (cVar1 != 'T') {
LAB_0011008d:
                    /* WARNING: Subroutine does not return */
            __assert_fail("0","print.c",0x518,"pred_fprintf");
          }
          uVar2 = *(undefined8 *)(param_2 + 0x60);
          uVar4 = *(undefined8 *)(param_2 + 0x58);
          goto LAB_0010ffe6;
        }
        if (cVar1 == 'A') {
          uVar2 = *(undefined8 *)(param_2 + 0x50);
          uVar4 = *(undefined8 *)(param_2 + 0x48);
          goto LAB_0010ffe6;
        }
        if (cVar1 != 'B') goto LAB_0011008d;
        uVar4 = 0xffffffffffffffff;
        uVar2 = 0xffffffffffffffff;
        if (*(char *)((long)piVar3 + 5) == '@') goto LAB_0010ffe6;
        FUN_0010ece0(param_3,*(undefined8 *)(piVar3 + 2),"");
      }
      piVar3 = *(int **)(piVar3 + 6);
    } while (piVar3 != (int *)0x0);
  }
  return 1;
}




// Function: make_segment @ 0x100b0

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
  puVar3 = (uint *)FUN_00125a50(0x20);
  *param_1 = (long)puVar3;
  *puVar3 = param_4;
  *(char *)(puVar3 + 1) = param_5;
  *(char *)((long)puVar3 + 5) = param_6;
  puVar3[6] = 0;
  puVar3[7] = 0;
  puVar3[4] = param_3;
  pcVar4 = (char *)FUN_00125a50(__n + 2);
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
    goto LAB_001101f9;
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
LAB_001101f9:
    *pcVar4 = local_39;
    pcVar4[1] = '\0';
    goto LAB_00110182;
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
LAB_00110182:
  return *param_1 + 0x18;
}




// Function: insert_fprintf @ 0x10380

undefined8 insert_fprintf(undefined8 *param_1,undefined8 param_2,byte *param_3)

{
  byte bVar1;
  ushort *puVar2;
  long lVar3;
  ushort **ppuVar4;
  char *pcVar5;
  undefined8 uVar6;
  byte *pbVar7;
  byte bVar8;
  ulong uVar9;
  long lVar10;
  int iVar11;
  byte *pbVar12;
  byte *pbVar13;
  int iVar14;
  byte *pbVar15;
  int __c;
  long lVar16;
  long local_48;
  
  lVar3 = FUN_0010d490(param_2);
  uVar6 = param_1[1];
  *(undefined8 *)(lVar3 + 0x38) = *param_1;
  *(undefined8 *)(lVar3 + 0x40) = uVar6;
  uVar6 = param_1[3];
  *(undefined8 *)(lVar3 + 0x48) = param_1[2];
  *(undefined8 *)(lVar3 + 0x50) = uVar6;
  uVar6 = param_1[4];
  local_48 = lVar3 + 0x38;
  *(undefined4 *)(lVar3 + 0x18) = 0x101;
  *(undefined8 *)(lVar3 + 0x58) = uVar6;
  *(undefined4 *)(lVar3 + 0x20) = 0;
  *(undefined8 *)(lVar3 + 0x38) = 0;
  bVar8 = *param_3;
  pbVar15 = param_3;
  if (bVar8 != 0) {
    do {
      while( true ) {
        bVar1 = param_3[1];
        uVar9 = (ulong)bVar1;
        iVar14 = (int)pbVar15;
        if (bVar8 != 0x5c) break;
        if (bVar1 == 99) {
          FUN_001100b0(local_48,pbVar15,(int)param_3 - iVar14,1,0,0,lVar3);
          if (*(char *)(lVar3 + 0x1a) == '\0') {
            return 1;
          }
          if (2 < *(uint *)(lVar3 + 0x20)) {
            return 1;
          }
          *(undefined4 *)(lVar3 + 0x20) = 3;
          return 1;
        }
        pbVar13 = param_3 + 1;
        if (bVar1 == 0) {
          uVar6 = dcgettext(0,"warning: escape `\\\' followed by nothing at all",5);
                    /* WARNING: Subroutine does not return */
          error(0,0,uVar6);
        }
        if ((byte)(bVar1 - 0x30) < 8) {
          bVar8 = 0;
          pbVar12 = pbVar13;
          lVar10 = 0;
          while( true ) {
            if (7 < (byte)((char)uVar9 - 0x30U)) break;
            pbVar7 = pbVar12 + 1;
            bVar8 = (char)uVar9 + -0x30 + bVar8 * '\b';
            pbVar13 = pbVar12;
            pbVar12 = pbVar7;
            if (lVar10 + 1 == 3) goto LAB_00110740;
            uVar9 = (ulong)*pbVar7;
            lVar10 = lVar10 + 1;
          }
          pbVar13 = param_3 + lVar10;
LAB_00110740:
          *param_3 = bVar8;
        }
        else {
          if ((0x1a < (byte)(bVar1 + 0xa4)) || ((&DAT_0012aa00)[(byte)(bVar1 + 0xa4)] == 0)) {
            uVar6 = dcgettext(0,"warning: unrecognized escape `\\%c\'",5);
                    /* WARNING: Subroutine does not return */
            error(0,0,uVar6,(int)(char)bVar1);
          }
          *param_3 = (&DAT_0012aa00)[(byte)(bVar1 + 0xa4)];
          pbVar12 = param_3 + 2;
        }
        local_48 = FUN_001100b0(local_48,pbVar15,((int)param_3 - iVar14) + 1,0,0,0,lVar3);
        bVar8 = pbVar13[1];
        param_3 = pbVar12;
        pbVar15 = pbVar12;
        if (bVar8 == 0) goto LAB_001104b7;
      }
      if (bVar8 == 0x25) {
        if (bVar1 == 0) {
          uVar6 = dcgettext(0,"error: %s at end of format string",5);
                    /* WARNING: Subroutine does not return */
          error(1,0,uVar6,param_3);
        }
        if (bVar1 != 0x25) {
          lVar10 = 0;
          while( true ) {
            lVar16 = lVar10 + 1;
            if (((char)uVar9 == '\0') ||
               (pcVar5 = strchr("-+ #",(int)(char)uVar9), pcVar5 == (char *)0x0)) break;
            uVar9 = (ulong)param_3[lVar10 + 2];
            lVar10 = lVar16;
          }
          ppuVar4 = __ctype_b_loc();
          puVar2 = *ppuVar4;
          while (bVar8 = (byte)uVar9, (*(byte *)((long)puVar2 + uVar9 * 2 + 1) & 8) != 0) {
            lVar16 = lVar16 + 1;
            uVar9 = (ulong)param_3[lVar16];
          }
          pbVar13 = param_3 + lVar16;
          if (bVar8 == 0x2e) {
            pbVar13 = param_3 + lVar16 + 1;
            bVar8 = *pbVar13;
            if ((*(byte *)((long)puVar2 + (ulong)bVar8 * 2 + 1) & 8) != 0) {
              pbVar7 = param_3 + lVar16 + 2;
              do {
                pbVar13 = pbVar7;
                bVar8 = *pbVar13;
                pbVar7 = pbVar13 + 1;
              } while ((*(byte *)((long)puVar2 + (ulong)bVar8 * 2 + 1) & 8) != 0);
            }
          }
          __c = (int)(char)bVar8;
          pcVar5 = strchr("abcdDfFgGhHiklmMnpPsStuUyYZ%",__c);
          pbVar7 = pbVar13;
          if (pcVar5 == (char *)0x0) goto LAB_00110610;
          if (bVar8 != 0) {
            iVar11 = 0;
            goto LAB_0011068d;
          }
          __c = 0;
LAB_0011056a:
          pcVar5 = strchr("{[(",__c);
          if (pcVar5 != (char *)0x0) {
LAB_00110840:
            uVar6 = dcgettext(0,"error: the format directive `%%%c\' is reserved for future use",5);
                    /* WARNING: Subroutine does not return */
            error(1,0,uVar6,__c);
          }
          pcVar5 = "warning: unrecognized format directive `%%%c\'";
LAB_00110592:
          uVar6 = dcgettext(0,pcVar5,5);
                    /* WARNING: Subroutine does not return */
          error(0,0,uVar6,__c);
        }
        pbVar13 = param_3 + 1;
        __c = 0x25;
        pcVar5 = strchr("abcdDfFgGhHiklmMnpPsStuUyYZ%",0x25);
        pbVar7 = pbVar13;
        if (pcVar5 == (char *)0x0) {
LAB_00110610:
          pcVar5 = strchr("ABCT",__c);
          if (pcVar5 == (char *)0x0) goto LAB_0011056a;
          iVar11 = (int)(char)pbVar7[1];
          pbVar13 = pbVar7 + 1;
          if (pbVar7[1] == 0) {
            pcVar5 = strchr("{[(",__c);
            if (pcVar5 != (char *)0x0) goto LAB_00110840;
            pcVar5 = "warning: format directive `%%%c\' should be followed by another character";
            goto LAB_00110592;
          }
        }
        else {
          __c = 0x25;
          iVar11 = 0;
        }
LAB_0011068d:
        pbVar12 = pbVar13 + 1;
        local_48 = FUN_001100b0(local_48,pbVar15,(int)pbVar7 - iVar14,2,__c,iVar11,lVar3);
        pbVar15 = pbVar12;
      }
      else {
        pbVar12 = param_3 + 1;
        pbVar13 = param_3;
      }
      bVar8 = pbVar13[1];
      param_3 = pbVar12;
    } while (bVar8 != 0);
LAB_001104b7:
    if (pbVar15 < pbVar12) {
      FUN_001100b0(local_48,pbVar15,(int)pbVar12 - (int)pbVar15,0,0,0,lVar3);
    }
  }
  return 1;
}




// Function: get_file_system_list @ 0x10870

long get_file_system_list(char param_1)

{
  long lVar1;
  long lVar2;
  
  if (DAT_00138e40 != 0) {
    if ((DAT_00138e38 == '\x01') || (lVar2 = DAT_00138e40, param_1 == '\0')) {
      return DAT_00138e40;
    }
    do {
      lVar1 = *(long *)(lVar2 + 0x30);
      FUN_0011efb0(lVar2);
      lVar2 = lVar1;
    } while (lVar1 != 0);
    DAT_00138e40 = 0;
  }
  DAT_00138e40 = FUN_0011e550(param_1);
  DAT_00138e38 = param_1;
  return DAT_00138e40;
}




// Function: filesystem_type @ 0x108e0

void filesystem_type(long *param_1)

{
  int iVar1;
  long lVar2;
  long lVar3;
  undefined8 uVar4;
  long lVar5;
  long in_FS_OFFSET;
  long local_c8 [19];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00138e58 != (void *)0x0) {
    if ((DAT_00138e50 != '\0') && (*param_1 == DAT_00138e48)) goto LAB_001109dc;
    free(DAT_00138e58);
  }
  DAT_00138e48 = *param_1;
  lVar2 = FUN_00110870(1);
  if (lVar2 == 0) {
    uVar4 = dcgettext(0,"Cannot read mounted file system list",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar4);
  }
  lVar5 = 0;
  do {
    iVar1 = strcmp(*(char **)(lVar2 + 0x18),"ignore");
    if (iVar1 != 0) {
      lVar3 = *(long *)(lVar2 + 0x20);
      if (*(long *)(lVar2 + 0x20) == -1) {
        FUN_0010d810(local_c8);
        iVar1 = (*DAT_00138670)(*(undefined8 *)(lVar2 + 8),local_c8);
        if (iVar1 != 0) goto LAB_00110970;
        *(long *)(lVar2 + 0x20) = local_c8[0];
        lVar3 = local_c8[0];
      }
      if (*param_1 == lVar3) {
        lVar5 = lVar2;
      }
    }
LAB_00110970:
    lVar2 = *(long *)(lVar2 + 0x30);
  } while (lVar2 != 0);
  if (lVar5 == 0) {
    DAT_00138e50 = '\0';
    uVar4 = dcgettext(0,"unknown",5);
    DAT_00138e58 = (void *)FUN_00125f30(uVar4);
  }
  else {
    DAT_00138e58 = (void *)FUN_00125f30(*(undefined8 *)(lVar5 + 0x18));
    DAT_00138e50 = '\x01';
  }
LAB_001109dc:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: is_used_fs_type @ 0x10a50

undefined8 is_used_fs_type(char *param_1)

{
  int iVar1;
  long lVar2;
  
  iVar1 = strcmp("afs",param_1);
  if (iVar1 != 0) {
    lVar2 = FUN_00110870(0);
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




// Function: get_mounted_devices @ 0x10ac0

void * get_mounted_devices(long *param_1)

{
  long lVar1;
  int iVar2;
  long lVar3;
  long lVar4;
  void *pvVar5;
  long lVar6;
  void *__ptr;
  long in_FS_OFFSET;
  long local_100;
  undefined8 local_e0;
  long local_d8 [19];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_e0 = 0;
  lVar3 = FUN_0011e550(0);
  if (lVar3 == 0) {
    pvVar5 = (void *)0x0;
  }
  else {
    local_100 = 0;
    lVar6 = lVar3;
    __ptr = (void *)0x0;
    do {
      while( true ) {
        lVar1 = (local_100 + 1) * 8;
        pvVar5 = (void *)FUN_00115be0(__ptr,lVar1,&local_e0);
        if (pvVar5 != (void *)0x0) break;
        pvVar5 = (void *)0x0;
        free(__ptr);
        lVar6 = *(long *)(lVar6 + 0x30);
        __ptr = pvVar5;
        if (lVar6 == 0) goto LAB_00110b90;
      }
      lVar4 = *(long *)(lVar6 + 0x20);
      if (*(long *)(lVar6 + 0x20) == -1) {
        FUN_0010d810(local_d8);
        iVar2 = (*DAT_00138670)(*(undefined8 *)(lVar6 + 8),local_d8);
        if (iVar2 == 0) {
          *(long *)(lVar6 + 0x20) = local_d8[0];
          lVar4 = local_d8[0];
          goto LAB_00110b36;
        }
      }
      else {
LAB_00110b36:
        *(long *)((long)pvVar5 + lVar1 + -8) = lVar4;
        local_100 = local_100 + 1;
      }
      lVar6 = *(long *)(lVar6 + 0x30);
      __ptr = pvVar5;
    } while (lVar6 != 0);
LAB_00110b90:
    do {
      lVar6 = *(long *)(lVar3 + 0x30);
      FUN_0011efb0(lVar3);
      lVar3 = lVar6;
    } while (lVar6 != 0);
    if (pvVar5 != (void *)0x0) {
      *param_1 = local_100;
    }
  }
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return pvVar5;
}




// Function: parse_files0_from @ 0x10c30

bool parse_files0_from(undefined8 param_1,long param_2,int *param_3)

{
  long lVar1;
  
  if (param_2 != 0) {
    lVar1 = *(long *)(param_2 + (long)*param_3 * 8);
    if (lVar1 != 0) {
      *param_3 = *param_3 + 1;
      DAT_00138690 = lVar1;
    }
    return lVar1 != 0;
  }
  return false;
}




// Function: parse_version @ 0x10c80

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
  FUN_00116370(&DAT_00128132);
  uVar3 = dcgettext(0,"Features enabled: ",5);
  __printf_chk(1,uVar3);
  __printf_chk(1,"D_TYPE ");
  pcVar4 = "enabled";
  if (DAT_00138678 == '\0') {
    pcVar4 = "disabled";
  }
  __printf_chk(1,"O_NOFOLLOW(%s) ",pcVar4);
  __printf_chk(1,"LEAF_OPTIMISATION ");
  iVar2 = is_selinux_enabled();
  if (0 < iVar2) {
    __printf_chk(1,"SELINUX ");
  }
  local_14 = 0;
  cVar1 = FUN_00108530(&local_14);
  if (cVar1 != '\0') {
    __printf_chk(1,&DAT_0012ab18);
    if ((local_14 & 0x200) != 0) {
      __printf_chk(1,"FTS_CWDFD");
    }
    __printf_chk(1,&DAT_00128690);
  }
  __printf_chk(1,"CBO(level=%d) ",DAT_00138688);
  __printf_chk(1,"\n");
                    /* WARNING: Subroutine does not return */
  exit(0);
}




// Function: parse_quit @ 0x10e20

undefined8 parse_quit(void)

{
  long lVar1;
  
  lVar1 = FUN_0010d520();
  *(undefined4 *)(lVar1 + 0x18) = 1;
  *(undefined4 *)(lVar1 + 0x24) = 0x3f800000;
  return 1;
}




// Function: parse_prune @ 0x10e50

undefined8 parse_prune(void)

{
  long lVar1;
  
  lVar1 = FUN_0010d520();
  if (DAT_00138620 == '\0') {
    *(undefined2 *)(lVar1 + 0x1a) = 0;
  }
  *(undefined2 *)(lVar1 + 0x18) = 1;
  return 1;
}




// Function: parse_nouser @ 0x10e80

undefined8 parse_nouser(void)

{
  long lVar1;
  
  lVar1 = FUN_0010d520();
  *(undefined4 *)(lVar1 + 0x24) = 0x3a83126f;
  return 1;
}




// Function: parse_accesscheck @ 0x10ea0

undefined8 parse_accesscheck(void)

{
  long *plVar1;
  undefined4 uVar2;
  
  plVar1 = (long *)FUN_0010d520();
  uVar2 = DAT_0012bc38;
  *(undefined4 *)(plVar1 + 3) = 0;
  if ((code *)*plVar1 != FUN_001096e0) {
    uVar2 = DAT_0012bc3c;
  }
  *(undefined4 *)((long)plVar1 + 0x24) = uVar2;
  return 1;
}




// Function: parse_empty @ 0x10ee0

undefined8 parse_empty(void)

{
  long lVar1;
  
  lVar1 = FUN_0010d520();
  *(undefined4 *)(lVar1 + 0x24) = 0x3c23d70a;
  return 1;
}




// Function: parse_delete @ 0x10f00

undefined8 parse_delete(void)

{
  long lVar1;
  
  lVar1 = FUN_0010d520();
  DAT_00138620 = 0;
  *(undefined4 *)(lVar1 + 0x18) = 0x101;
  *(undefined4 *)(lVar1 + 0x24) = 0x3f800000;
  return 1;
}




// Function: insert_type @ 0x10f30

undefined8 insert_type(long param_1,int *param_2,undefined8 param_3,code *param_4)

{
  char cVar1;
  char *pcVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  long lVar8;
  long lVar9;
  undefined8 uVar10;
  char *pcVar11;
  float fVar12;
  
  pcVar11 = "-xtype";
  if (param_4 != FUN_00109af0) {
    pcVar11 = "-type";
  }
  if (param_1 != 0) {
    pcVar2 = *(char **)(param_1 + (long)*param_2 * 8);
    if (pcVar2 != (char *)0x0) {
      *param_2 = *param_2 + 1;
      if (*pcVar2 == '\0') {
        uVar10 = dcgettext(0,"Arguments to %s should contain at least one letter",5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar10,pcVar11);
      }
      lVar8 = FUN_0010d490(param_3,param_4,pcVar2);
      *(undefined4 *)(lVar8 + 0x24) = 0;
      *(bool *)(lVar8 + 0x1a) = param_4 == FUN_00109af0;
      *(bool *)(lVar8 + 0x1b) = param_4 != FUN_00109af0;
      fVar7 = DAT_0012bc5c;
      fVar6 = DAT_0012bc58;
      fVar5 = DAT_0012bc54;
      fVar4 = DAT_0012bc50;
      fVar3 = DAT_0012bc44;
      cVar1 = *pcVar2;
      if (cVar1 == '\0') {
        return 1;
      }
      while( true ) {
        if (0x2f < (byte)(cVar1 + 0xbcU)) break;
        switch(cVar1 + 0xbcU) {
        case 0:
          uVar10 = dcgettext(0,
                             "%s %c is not supported because Solaris doors are not supported on the platform find was compiled on."
                             ,5);
                    /* WARNING: Subroutine does not return */
          error(1,0,uVar10,pcVar11,0x44);
        default:
          goto switchD_00111002_caseD_1;
        case 0x1e:
          lVar9 = 0;
          fVar12 = fVar3;
          break;
        case 0x1f:
          lVar9 = 1;
          fVar12 = DAT_0012bc48;
          break;
        case 0x20:
          lVar9 = 2;
          fVar12 = DAT_0012bc4c;
          break;
        case 0x22:
          lVar9 = 3;
          fVar12 = fVar4;
          break;
        case 0x28:
          lVar9 = 4;
          fVar12 = fVar5;
          break;
        case 0x2c:
          lVar9 = 5;
          fVar12 = fVar6;
          break;
        case 0x2f:
          lVar9 = 6;
          fVar12 = fVar7;
        }
        if (*(char *)(lVar8 + 0x38 + lVar9) != '\0') {
          uVar10 = dcgettext(0,"Duplicate file type \'%c\' in the argument list to %s.",5);
                    /* WARNING: Subroutine does not return */
          error(1,0,uVar10,(int)cVar1,pcVar11);
        }
        *(float *)(lVar8 + 0x24) = fVar12 + *(float *)(lVar8 + 0x24);
        *(undefined1 *)(lVar8 + 0x38 + lVar9) = 1;
        if (pcVar2[1] == '\0') {
          return 1;
        }
        if (pcVar2[1] != ',') {
          uVar10 = dcgettext(0,"Must separate multiple arguments to %s using: \',\'",5);
                    /* WARNING: Subroutine does not return */
          error(1,0,uVar10,pcVar11);
        }
        cVar1 = pcVar2[2];
        pcVar2 = pcVar2 + 2;
        if (cVar1 == '\0') {
          uVar10 = dcgettext(0,
                             "Last file type in list argument to %s is missing, i.e., list is ending on: \',\'"
                             ,5);
                    /* WARNING: Subroutine does not return */
          error(1,0,uVar10,pcVar11);
        }
      }
switchD_00111002_caseD_1:
      uVar10 = dcgettext(0,"Unknown argument to %s: %c",5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar10,pcVar11,(int)cVar1);
    }
  }
  return 0;
}




// Function: parse_type @ 0x11200

void parse_type(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_00110f30(param_2,param_3,param_1,FUN_00109900);
  return;
}




// Function: fnmatch_sanitycheck @ 0x11220

undefined8 fnmatch_sanitycheck(void)

{
  int iVar1;
  undefined8 uVar2;
  
  if (DAT_00138e60 != '\0') {
    return 1;
  }
  iVar1 = fnmatch("foo","foo",0);
  if (iVar1 == 0) {
    iVar1 = fnmatch("Foo","foo",0);
    if (iVar1 != 0) {
      iVar1 = fnmatch("Foo","foo",0x10);
      if (iVar1 == 0) {
        DAT_00138e60 = 1;
        return 1;
      }
    }
  }
  uVar2 = dcgettext(0,"sanity check of the fnmatch() library function failed.",5);
                    /* WARNING: Subroutine does not return */
  error(1,0,uVar2);
}




// Function: insert_path_check @ 0x112c0

undefined8
insert_path_check(undefined8 param_1,long param_2,int *param_3,undefined8 param_4,undefined *param_5)

{
  char *__s;
  char cVar1;
  long lVar2;
  char *pcVar3;
  undefined8 uVar4;
  
  FUN_00111220();
  if (param_2 == 0) {
    return 0;
  }
  uVar4 = 0;
  __s = *(char **)(param_2 + (long)*param_3 * 8);
  if (__s != (char *)0x0) {
    *param_3 = *param_3 + 1;
    lVar2 = FUN_0010d490(param_1,param_5,__s);
    *(undefined2 *)(lVar2 + 0x1a) = 0;
    *(char **)(lVar2 + 0x38) = __s;
    pcVar3 = strpbrk(__s,"*?[");
    cVar1 = DAT_00138631;
    if (pcVar3 == (char *)0x0) {
      *(undefined4 *)(lVar2 + 0x24) = DAT_0012bc64;
    }
    else {
      *(undefined4 *)(lVar2 + 0x24) = DAT_0012bc60;
    }
    if ((((cVar1 == '\0') && (pcVar3 = strrchr(__s,0x2f), pcVar3 != (char *)0x0)) &&
        (pcVar3[1] == '\0')) &&
       (cVar1 = FUN_0010b4b0(__s,param_5 == PTR_FUN_00137f78), cVar1 == '\0')) {
      uVar4 = dcgettext(0,"warning: -%s %s will not match anything because it ends with /.",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar4,param_4,__s);
    }
    uVar4 = 1;
  }
  return uVar4;
}




// Function: parse_wholename @ 0x113f0

void parse_wholename(void)

{
  FUN_001112c0();
  return;
}




// Function: parse_nogroup @ 0x115c0

undefined8 parse_nogroup(undefined8 param_1)

{
  long lVar1;
  
  lVar1 = FUN_0010d4d0(param_1,0);
  *(undefined4 *)(lVar1 + 0x24) = 0x38d1b717;
  return 1;
}




// Function: parse_lname @ 0x115e0

undefined8 parse_lname(undefined8 param_1,long param_2,int *param_3)

{
  long lVar1;
  char *pcVar2;
  undefined8 uVar3;
  undefined4 uVar4;
  
  FUN_00111220();
  if (param_2 != 0) {
    uVar3 = 0;
    pcVar2 = *(char **)(param_2 + (long)*param_3 * 8);
    if (pcVar2 != (char *)0x0) {
      *param_3 = *param_3 + 1;
      lVar1 = FUN_0010d4d0(param_1,pcVar2);
      *(char **)(lVar1 + 0x38) = pcVar2;
      pcVar2 = strpbrk(pcVar2,"*?[");
      uVar4 = DAT_0012bc6c;
      if (pcVar2 == (char *)0x0) {
        uVar4 = DAT_0012bc70;
      }
      *(undefined4 *)(lVar1 + 0x24) = uVar4;
      uVar3 = 1;
    }
    return uVar3;
  }
  return 0;
}




// Function: parse_ilname @ 0x11690

undefined8 parse_ilname(undefined8 param_1,long param_2,int *param_3)

{
  long lVar1;
  char *pcVar2;
  undefined8 uVar3;
  undefined4 uVar4;
  
  if (param_2 != 0) {
    uVar3 = 0;
    pcVar2 = *(char **)(param_2 + (long)*param_3 * 8);
    if (pcVar2 != (char *)0x0) {
      *param_3 = *param_3 + 1;
      lVar1 = FUN_0010d4d0(param_1,pcVar2);
      *(char **)(lVar1 + 0x38) = pcVar2;
      pcVar2 = strpbrk(pcVar2,"*?[");
      uVar4 = DAT_0012bc6c;
      if (pcVar2 == (char *)0x0) {
        uVar4 = DAT_0012bc70;
      }
      *(undefined4 *)(lVar1 + 0x24) = uVar4;
      uVar3 = 1;
    }
    return uVar3;
  }
  return 0;
}




// Function: parse_size @ 0x11720

undefined8 parse_size(undefined8 param_1,long param_2,int *param_3)

{
  char *pcVar1;
  char *__s;
  char cVar2;
  int iVar3;
  size_t sVar4;
  long lVar5;
  undefined8 uVar6;
  char cVar7;
  long lVar8;
  long in_FS_OFFSET;
  undefined4 uVar9;
  long local_48;
  undefined1 local_40 [14];
  char local_32 [2];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if ((param_2 == 0) || (__s = *(char **)(param_2 + (long)*param_3 * 8), __s == (char *)0x0)) {
    uVar6 = 0;
    goto LAB_00111829;
  }
  sVar4 = strlen(__s);
  if ((int)sVar4 == 0) {
    uVar6 = dcgettext(0,"invalid null argument to -size",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar6);
  }
  pcVar1 = __s + (long)(int)sVar4 + -1;
  cVar7 = *pcVar1;
  if (cVar7 == 'b') {
    *pcVar1 = '\0';
    lVar8 = 0x200;
LAB_001117a9:
    cVar2 = *__s;
    if (cVar2 != '+') goto LAB_001117c0;
LAB_00111918:
    iVar3 = FUN_00126490(__s + 1,local_40,10,&local_48,"");
    if (iVar3 != 0) goto LAB_001119fe;
    lVar5 = FUN_0010d4d0(param_1,__s);
    uVar9 = DAT_0012bc64;
    *(int *)(lVar5 + 0x3c) = (int)lVar8;
    *(undefined4 *)(lVar5 + 0x38) = 0;
    if ((ulong)(lVar8 * local_48) < 0x5001) {
      uVar9 = DAT_0012bc3c;
    }
  }
  else {
    if ('b' < cVar7) {
      if (cVar7 == 'k') {
        *pcVar1 = '\0';
        lVar8 = 0x400;
      }
      else if (cVar7 == 'w') {
        *pcVar1 = '\0';
        lVar8 = 2;
      }
      else {
        if (cVar7 != 'c') goto LAB_001119b3;
        *pcVar1 = '\0';
        lVar8 = 1;
      }
      goto LAB_001117a9;
    }
    if (cVar7 == 'G') {
      *pcVar1 = '\0';
      lVar8 = 0x40000000;
      goto LAB_001117a9;
    }
    if ('G' < cVar7) {
      if (cVar7 != 'M') goto LAB_001119b3;
      *pcVar1 = '\0';
      lVar8 = 0x100000;
      goto LAB_001117a9;
    }
    if (9 < (byte)(cVar7 - 0x30U)) {
LAB_001119b3:
      uVar6 = dcgettext(0,"invalid -size type `%c\'",5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar6,(int)cVar7);
    }
    cVar2 = *__s;
    lVar8 = 0x200;
    cVar7 = '\0';
    if (cVar2 == '+') goto LAB_00111918;
LAB_001117c0:
    if (cVar2 == '-') {
      iVar3 = FUN_00126490(__s + 1,local_40,10,&local_48,"");
      if (iVar3 != 0) goto LAB_001119fe;
      lVar5 = FUN_0010d4d0(param_1,__s);
      uVar9 = DAT_0012bc3c;
      *(int *)(lVar5 + 0x3c) = (int)lVar8;
      *(undefined4 *)(lVar5 + 0x38) = 1;
      if ((ulong)(lVar8 * local_48) < 0x5001) {
        uVar9 = DAT_0012bc64;
      }
    }
    else {
      iVar3 = FUN_00126490(__s,local_40,10,&local_48,"");
      if (iVar3 != 0) {
LAB_001119fe:
        local_32[1] = 0;
        local_32[0] = cVar7;
        uVar6 = dcgettext(0,"Invalid argument `%s%s\' to -size",5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar6,__s,local_32);
      }
      lVar5 = FUN_0010d4d0(param_1,__s);
      uVar9 = DAT_0012bc40;
      *(undefined4 *)(lVar5 + 0x38) = 2;
      *(int *)(lVar5 + 0x3c) = (int)lVar8;
    }
  }
  *(long *)(lVar5 + 0x40) = local_48;
  *(undefined2 *)(lVar5 + 0x1a) = 1;
  *(undefined4 *)(lVar5 + 0x24) = uVar9;
  uVar6 = 1;
  *param_3 = *param_3 + 1;
LAB_00111829:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar6;
}




// Function: parse_samefile @ 0x11a40

undefined8 parse_samefile(undefined8 param_1,long param_2,int *param_3)

{
  int iVar1;
  int iVar2;
  long lVar3;
  undefined8 uVar4;
  int *piVar5;
  undefined4 uVar6;
  long in_FS_OFFSET;
  __dev_t local_168;
  __ino_t _Stack_160;
  uint uStack_150;
  stat local_d8;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010d810(&local_168);
  if (param_2 != 0) {
    lVar3 = *(long *)(param_2 + (long)*param_3 * 8);
    if (lVar3 != 0) {
      *param_3 = *param_3 + 1;
      iVar1 = (*DAT_00138670)(lVar3,&local_168);
      if (iVar1 != 0) goto LAB_00111c7a;
      FUN_0010d810(&local_d8);
      uVar6 = 0;
      if (DAT_00138668 == 0) {
        if (DAT_00138678 != '\0') {
          uVar6 = 0x20000;
          goto LAB_00111af5;
        }
        if ((uStack_150 & 0xf000) != 0xa000) goto LAB_00111af5;
        iVar1 = -2;
      }
      else {
LAB_00111af5:
        iVar1 = FUN_001160b0(lVar3,uVar6);
        if (-1 < iVar1) {
          iVar2 = fstat(iVar1,&local_d8);
          if (iVar2 != 0) {
LAB_00111c7a:
            piVar5 = __errno_location();
                    /* WARNING: Subroutine does not return */
            FUN_0010e920(*piVar5,lVar3);
          }
          iVar2 = (*DAT_00138670)(lVar3,&local_168);
          if (iVar2 != 0) goto LAB_00111c7a;
          if ((DAT_00138668 == 0) && (DAT_00138678 == '\0')) {
            if ((((uStack_150 & 0xf000) == 0xa000) || (local_168 != local_d8.st_dev)) ||
               (_Stack_160 != local_d8.st_ino)) {
              close(iVar1);
              iVar1 = -1;
            }
          }
          else {
            local_168 = local_d8.st_dev;
            _Stack_160 = local_d8.st_ino;
          }
        }
      }
      lVar3 = FUN_0010d4d0(param_1,lVar3);
      *(int *)(lVar3 + 0x48) = iVar1;
      *(__ino_t *)(lVar3 + 0x38) = _Stack_160;
      *(undefined4 *)(lVar3 + 0x24) = 0x3c23d70a;
      *(__dev_t *)(lVar3 + 0x40) = local_168;
      *(undefined2 *)(lVar3 + 0x1a) = 1;
      uVar4 = 1;
      goto LAB_00111b3b;
    }
  }
  uVar4 = 0;
LAB_00111b3b:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: open_output_file @ 0x11c90

void open_output_file(char *param_1,undefined8 *param_2)

{
  int iVar1;
  undefined8 uVar2;
  FILE *__stream;
  int *piVar3;
  bool bVar4;
  
  *param_2 = 0;
  uVar2 = FUN_00123640(0);
  param_2[4] = uVar2;
  iVar1 = strcmp(param_1,"/dev/stderr");
  if (iVar1 == 0) {
    param_2[1] = stderr;
    uVar2 = dcgettext(0,"standard error",5);
    __stream = (FILE *)param_2[1];
    param_2[2] = uVar2;
  }
  else {
    iVar1 = strcmp(param_1,"/dev/stdout");
    if (iVar1 == 0) {
      param_2[1] = stdout;
      uVar2 = dcgettext(0,"standard output",5);
      __stream = (FILE *)param_2[1];
      param_2[2] = uVar2;
    }
    else {
      __stream = (FILE *)FUN_0010eb40(DAT_00138610,param_1);
      param_2[2] = param_1;
      param_2[1] = __stream;
      if (__stream == (FILE *)0x0) {
        piVar3 = __errno_location();
                    /* WARNING: Subroutine does not return */
        FUN_0010e9b0(*piVar3,param_1);
      }
    }
  }
  iVar1 = fileno(__stream);
  bVar4 = false;
  if (iVar1 != -1) {
    iVar1 = isatty(iVar1);
    bVar4 = iVar1 != 0;
  }
  *(bool *)(param_2 + 3) = bVar4;
  return;
}




// Function: parse_printf @ 0x11d80

undefined8 parse_printf(undefined8 param_1,long param_2,int *param_3)

{
  int iVar1;
  long lVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined1 auStack_58 [40];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 == 0) {
    uVar3 = 0;
  }
  else {
    iVar1 = *param_3;
    lVar2 = *(long *)(param_2 + (long)iVar1 * 8);
    uVar3 = 0;
    if (lVar2 != 0) {
      *param_3 = iVar1 + 1;
      FUN_00111c90("/dev/stdout",auStack_58);
      uVar3 = FUN_00110380(auStack_58,param_1,lVar2);
      if ((char)uVar3 == '\0') {
        *param_3 = iVar1;
      }
    }
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_print @ 0x11e20

undefined8 parse_print(void)

{
  long lVar1;
  
  lVar1 = FUN_0010d520();
  *(undefined4 *)(lVar1 + 0x18) = 0x101;
  FUN_00111c90("/dev/stdout",lVar1 + 0x38);
  return 1;
}




// Function: parse_fprint0 @ 0x11e50

bool parse_fprint0(undefined8 param_1,long param_2,int *param_3)

{
  long lVar1;
  long lVar2;
  
  if (param_2 != 0) {
    lVar1 = *(long *)(param_2 + (long)*param_3 * 8);
    if (lVar1 != 0) {
      *param_3 = *param_3 + 1;
      lVar2 = FUN_0010d4d0(param_1,lVar1);
      FUN_00111c90(lVar1,lVar2 + 0x38);
      *(undefined4 *)(lVar2 + 0x18) = 0x101;
      *(undefined4 *)(lVar2 + 0x24) = 0x3f800000;
    }
    return lVar1 != 0;
  }
  return false;
}




// Function: parse_fls @ 0x11ec0

bool parse_fls(undefined8 param_1,long param_2,int *param_3)

{
  long lVar1;
  long lVar2;
  
  if (param_2 != 0) {
    lVar1 = *(long *)(param_2 + (long)*param_3 * 8);
    if (lVar1 != 0) {
      *param_3 = *param_3 + 1;
      lVar2 = FUN_0010d520();
      FUN_00111c90(lVar1,lVar2 + 0x38);
      *(undefined4 *)(lVar2 + 0x24) = 0x3f800000;
      *(undefined2 *)(lVar2 + 0x18) = 0x101;
    }
    return lVar1 != 0;
  }
  return false;
}




// Function: parse_fprintf @ 0x11f30

undefined8 parse_fprintf(undefined8 param_1,long param_2,int *param_3)

{
  long lVar1;
  long lVar2;
  undefined8 uVar3;
  int iVar4;
  long in_FS_OFFSET;
  undefined1 auStack_58 [40];
  long local_30;
  
  iVar4 = *param_3;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 != 0) {
    lVar1 = *(long *)(param_2 + (long)iVar4 * 8);
    if ((lVar1 != 0) && (lVar2 = *(long *)(param_2 + 8 + (long)iVar4 * 8), lVar2 != 0)) {
      *param_3 = iVar4 + 2;
      FUN_00111c90(lVar1,auStack_58);
      iVar4 = *param_3;
      uVar3 = FUN_00110380(auStack_58,param_1,lVar2);
      if ((char)uVar3 != '\0') goto LAB_00111fa3;
    }
  }
  *param_3 = iVar4;
  uVar3 = 0;
LAB_00111fa3:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_fprint @ 0x11fd0

bool parse_fprint(undefined8 param_1,long param_2,int *param_3)

{
  long lVar1;
  long lVar2;
  
  if (param_2 != 0) {
    lVar1 = *(long *)(param_2 + (long)*param_3 * 8);
    if (lVar1 != 0) {
      *param_3 = *param_3 + 1;
      lVar2 = FUN_0010d4d0(param_1,lVar1);
      FUN_00111c90(lVar1,lVar2 + 0x38);
      *(undefined4 *)(lVar2 + 0x18) = 0x101;
      *(undefined4 *)(lVar2 + 0x24) = 0x3f800000;
    }
    return lVar1 != 0;
  }
  return false;
}




// Function: parse_perm @ 0x12040

undefined8 parse_perm(undefined8 param_1,long param_2,int *param_3)

{
  char *pcVar1;
  int iVar2;
  int iVar3;
  void *__ptr;
  long lVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  char *pcVar7;
  undefined4 uVar8;
  undefined4 local_40;
  
  if (param_2 == 0) {
    return 0;
  }
  pcVar1 = *(char **)(param_2 + (long)*param_3 * 8);
  if (pcVar1 != (char *)0x0) {
    *param_3 = *param_3 + 1;
    if (*pcVar1 == '-') {
      pcVar7 = pcVar1 + 1;
      uVar8 = 0;
      local_40 = DAT_0012bc38;
    }
    else if (*pcVar1 == '/') {
      local_40 = DAT_0012bc74;
      pcVar7 = pcVar1 + 1;
      uVar8 = 1;
    }
    else {
      uVar8 = 2;
      local_40 = DAT_0012bc40;
      pcVar7 = pcVar1;
    }
    __ptr = (void *)FUN_0011def0(pcVar7);
    if ((__ptr != (void *)0x0) && ((*pcVar1 != '+' || (7 < (byte)(pcVar1[1] - 0x30U))))) {
      iVar2 = FUN_0011e310(0,0,0,__ptr,0);
      iVar3 = FUN_0011e310(0,1,0,__ptr,0);
      free(__ptr);
      if ((*pcVar1 == '/') && (iVar3 == 0 && iVar2 == 0)) {
        uVar5 = dcgettext(0,
                          "warning: you have specified a mode pattern %s (which is equivalent to /000). The meaning of -perm /000 has now been changed to be consistent with -perm -000; that is, while it used to match no files, it now matches all files."
                          ,5);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar5,pcVar1);
      }
      lVar4 = FUN_0010d4d0(param_1,pcVar1);
      *(undefined4 *)(lVar4 + 0x38) = uVar8;
      *(ulong *)(lVar4 + 0x3c) = CONCAT44(iVar3,iVar2);
      *(undefined4 *)(lVar4 + 0x24) = local_40;
      return 1;
    }
    uVar5 = FUN_001239f0(0,DAT_0013868c,pcVar1);
    uVar6 = dcgettext(0,"invalid mode %s",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar6,uVar5);
  }
  return 0;
}




// Function: insert_exec_ok @ 0x12230

undefined8 insert_exec_ok(undefined8 param_1,long param_2,long param_3,int *param_4)

{
  int iVar1;
  code *pcVar2;
  bool bVar3;
  char cVar4;
  int iVar5;
  void *__ptr;
  char *pcVar6;
  undefined8 uVar7;
  long lVar8;
  size_t sVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  long lVar13;
  char *pcVar14;
  long *plVar15;
  int iVar16;
  undefined8 *puVar17;
  long *plVar18;
  long in_FS_OFFSET;
  bool bVar19;
  bool bVar20;
  char *local_b8;
  long local_68;
  size_t local_60;
  undefined1 local_58 [24];
  long local_40;
  
  pcVar2 = *(code **)(param_2 + 0x18);
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if ((param_3 == 0) || (*(long *)(param_3 + (long)*param_4 * 8) == 0)) {
    uVar7 = 0;
    goto LAB_0011260a;
  }
  __ptr = (void *)FUN_0010d490(param_2,pcVar2,"(some -exec* arguments)");
  bVar19 = pcVar2 != (code *)PTR_FUN_00137fc8;
  bVar20 = pcVar2 != (code *)PTR_FUN_00137f20;
  *(undefined4 *)((long)__ptr + 0x18) = 0x101;
  *(undefined8 *)((long)__ptr + 0xf8) = 0;
  if (bVar19 && bVar20) {
    *(undefined1 *)((long)__ptr + 0xf4) = 0;
    if (pcVar2 == FUN_00108fc0) goto LAB_00112303;
LAB_001123dc:
    if (DAT_001385c0 == 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("NULL != initial_wd","parser.c",0xb79,"insert_exec_ok");
    }
    *(long *)((long)__ptr + 0xf8) = DAT_001385c0;
  }
  else {
    DAT_00138698 = 1;
    *(undefined1 *)((long)__ptr + 0xf4) = 1;
    if ((pcVar2 != FUN_00108fc0) && (pcVar2 != (code *)PTR_FUN_00137f20)) goto LAB_001123dc;
LAB_00112303:
    DAT_0013862e = 0;
    pcVar6 = getenv("PATH");
    if (pcVar6 != (char *)0x0) {
      FUN_00117570(pcVar6,":",1,&local_68,&local_60);
      do {
        if (local_60 == 0) {
LAB_00112392:
          uVar7 = dcgettext(0,
                            "The current directory is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove the current directory from your $PATH (that is, remove \".\", doubled colons, or leading or trailing colons)"
                            ,5);
                    /* WARNING: Subroutine does not return */
          error(1,0,uVar7,param_1);
        }
        cVar4 = pcVar6[local_68];
        if ((local_60 == 1) && (cVar4 == '.')) goto LAB_00112392;
        if (cVar4 != '/') {
          pcVar14 = strndup(pcVar6 + local_68,local_60);
          if (pcVar14 == (char *)0x0) {
            pcVar14 = pcVar6 + local_68;
          }
          uVar7 = FUN_0010e8a0(0,pcVar14);
          uVar10 = dcgettext(0,
                             "The relative path %s is included in the PATH environment variable, which is insecure in combination with the %s action of find.  Please remove that entry from $PATH"
                             ,5);
                    /* WARNING: Subroutine does not return */
          error(1,0,uVar10,uVar7,param_1);
        }
        cVar4 = FUN_00117570(pcVar6,":",0,&local_68,&local_60);
      } while (cVar4 != '\0');
    }
  }
  *(undefined1 *)((long)__ptr + 0x38) = 0;
  iVar1 = *param_4;
  lVar8 = (long)iVar1 * 8;
  plVar18 = (long *)(param_3 + lVar8);
  pcVar6 = (char *)*plVar18;
  iVar16 = iVar1;
  if (pcVar6 != (char *)0x0) {
    puVar17 = (undefined8 *)(param_3 + 8 + lVar8);
    local_b8 = (char *)0x0;
    iVar5 = 0;
    bVar3 = false;
    plVar15 = plVar18;
    do {
      if (*pcVar6 == ';') {
        if (pcVar6[1] == '\0') {
LAB_001124de:
          if ((iVar1 != iVar16) && (*plVar15 != 0)) {
            if (*(char *)((long)__ptr + 0x38) != '\0') {
              pcVar6 = "dir";
              if (pcVar2 != FUN_00108fc0) {
                pcVar6 = "";
              }
              if (1 < iVar5) {
                uVar7 = dcgettext(0,"Only one instance of {} is supported with -exec%s ... +",5);
                    /* WARNING: Subroutine does not return */
                error(1,0,uVar7,pcVar6);
              }
              sVar9 = strlen(local_b8);
              if (sVar9 != 2) {
                __snprintf_chk(local_58,0x13,1,0x13,"-exec%s ... {} +",pcVar6);
                uVar7 = FUN_001239f0(2,DAT_0013868c,local_b8);
                uVar10 = FUN_001239f0(1,DAT_0013868c,&DAT_0012ac03);
                uVar11 = FUN_001239f0(0,DAT_0013868c,local_58);
                uVar12 = dcgettext(0,"In %s the %s must appear by itself, but you specified %s",5);
                    /* WARNING: Subroutine does not return */
                error(1,0,uVar12,uVar11,uVar10,uVar7);
              }
            }
            lVar8 = (long)__ptr + 0x40;
            iVar5 = FUN_001156e0(lVar8,0x800);
            if (iVar5 - 1U < 2) {
              uVar7 = dcgettext(0,"The environment is too large for exec().",5);
                    /* WARNING: Subroutine does not return */
              error(1,0,uVar7);
            }
            FUN_00115860(lVar8);
            iVar5 = iVar16 - iVar1;
            lVar13 = (long)__ptr + 0x98;
            *(code **)((long)__ptr + 0x80) = FUN_0010a3a0;
            if (*(char *)((long)__ptr + 0x38) == '\0') {
              *(int *)((long)__ptr + 0xf0) = iVar5;
              *(undefined **)((long)__ptr + 0x70) = &DAT_0012ac03;
              sVar9 = strlen("{}");
              *(size_t *)((long)__ptr + 0x68) = sVar9;
              *(undefined8 *)((long)__ptr + 0x88) = 0;
              *(undefined8 *)((long)__ptr + 0x90) = 0;
              uVar7 = FUN_00125a50((long)iVar5 << 3);
              *(undefined8 *)((long)__ptr + 0xe8) = uVar7;
              FUN_00115890(lVar8,lVar13,(long)__ptr + 0x38);
              iVar1 = *(int *)((long)__ptr + 0xf0);
              if (0 < iVar1) {
                lVar8 = *(long *)((long)__ptr + 0xe8);
                lVar13 = 0;
                do {
                  *(long *)(lVar8 + lVar13 * 8) = plVar18[lVar13];
                  lVar13 = lVar13 + 1;
                } while (lVar13 != iVar1);
              }
            }
            else {
              *(undefined8 *)((long)__ptr + 0xe8) = 0;
              *(undefined8 *)((long)__ptr + 0x70) = 0;
              *(long *)((long)__ptr + 0x78) = (long)(iVar5 + -1);
              *(undefined8 *)((long)__ptr + 0x68) = 0;
              *(undefined8 *)((long)__ptr + 0x88) = 0;
              *(undefined8 *)((long)__ptr + 0x90) = 0;
              FUN_00115890(lVar8,lVar13,(long)__ptr + 0x38);
              if (iVar1 < iVar16 + -1) {
                do {
                  pcVar6 = (char *)*plVar18;
                  plVar18 = plVar18 + 1;
                  sVar9 = strlen(pcVar6);
                  FUN_00114fb0(lVar8,lVar13,pcVar6,sVar9 + 1,0,0,1);
                } while (plVar18 !=
                         (long *)(param_3 + 8 + ((ulong)((iVar16 - iVar1) - 2) + (long)iVar1) * 8));
              }
            }
            if (*plVar15 == 0) {
              *param_4 = iVar16;
              uVar7 = 1;
            }
            else {
              *param_4 = iVar16 + 1;
              uVar7 = 1;
            }
            goto LAB_0011260a;
          }
          break;
        }
      }
      else if ((((*pcVar6 == '+') && (bVar19 && bVar20)) && (pcVar6[1] == '\0')) && (bVar3)) {
        *(undefined1 *)((long)__ptr + 0x38) = 1;
        goto LAB_001124de;
      }
      lVar8 = FUN_0011d5d0(pcVar6);
      if (lVar8 == 0) {
        bVar3 = false;
      }
      else {
        iVar5 = iVar5 + 1;
        if ((iVar16 == 0) && ((pcVar2 == FUN_00108fc0 || (pcVar2 == (code *)PTR_FUN_00137f20)))) {
          uVar7 = dcgettext(0,
                            "You may not use {} within the utility name for -execdir and -okdir, because this is a potential security problem."
                            ,5);
                    /* WARNING: Subroutine does not return */
          error(1,0,uVar7);
        }
        bVar3 = true;
        local_b8 = pcVar6;
      }
      pcVar6 = (char *)*puVar17;
      iVar16 = iVar16 + 1;
      puVar17 = puVar17 + 1;
      plVar15 = plVar15 + 1;
    } while (pcVar6 != (char *)0x0);
  }
  *param_4 = iVar16;
  free(__ptr);
  uVar7 = 0;
LAB_0011260a:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar7;
}




// Function: parse_okdir @ 0x12920

void parse_okdir(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_00112230("-okdir",param_1,param_2,param_3);
  return;
}




// Function: parse_or @ 0x129a0

undefined8 parse_or(void)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0010b990();
  puVar1[2] = 0x200000003;
  *puVar1 = FUN_00108940;
  *(undefined2 *)((long)puVar1 + 0x1a) = 0;
  return 1;
}




// Function: parse_and @ 0x129e0

undefined8 parse_and(void)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0010b990();
  puVar1[2] = 0x300000003;
  *puVar1 = FUN_00108990;
  *(undefined2 *)((long)puVar1 + 0x1a) = 0;
  return 1;
}




// Function: parse_comma @ 0x12a20

undefined8 parse_comma(void)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0010b990();
  puVar1[2] = 0x100000003;
  *puVar1 = FUN_00108ca0;
  *(undefined2 *)((long)puVar1 + 0x1a) = 0;
  *(undefined4 *)((long)puVar1 + 0x24) = 0x3f800000;
  return 1;
}




// Function: parse_closeparen @ 0x12a60

undefined8 parse_closeparen(void)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0010b990();
  puVar1[2] = 5;
  *puVar1 = FUN_00108be0;
  *(undefined2 *)((long)puVar1 + 0x1a) = 0;
  return 1;
}




// Function: parse_group @ 0x12a90

undefined8 parse_group(undefined8 param_1,long param_2,int *param_3)

{
  char *__name;
  __gid_t _Var1;
  group *pgVar2;
  long lVar3;
  size_t sVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined4 uVar8;
  
  if (param_2 == 0) {
    return 0;
  }
  __name = *(char **)(param_2 + (long)*param_3 * 8);
  if (__name == (char *)0x0) {
    return 0;
  }
  *param_3 = *param_3 + 1;
  pgVar2 = getgrnam(__name);
  endgrent();
  if (pgVar2 == (group *)0x0) {
    sVar4 = strspn(__name,"0123456789");
    if ((int)sVar4 == 0) {
      if (*__name != '\0') {
        uVar5 = FUN_001239f0(0,DAT_0013868c,__name);
        uVar6 = dcgettext(0,"%s is not the name of an existing group",5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar6,uVar5);
      }
      uVar5 = dcgettext(0,"argument to -group is empty, but should be a group name",5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar5);
    }
    if (__name[(int)sVar4] != '\0') {
      uVar5 = FUN_001239f0(1,DAT_0013868c,__name + (int)sVar4);
      uVar6 = FUN_001239f0(0,DAT_0013868c,__name);
      uVar7 = dcgettext(0,
                        "%s is not the name of an existing group and it does not look like a numeric group ID because it has the unexpected suffix %s"
                        ,5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar7,uVar6,uVar5);
    }
    _Var1 = FUN_00117410(__name);
  }
  else {
    _Var1 = pgVar2->gr_gid;
  }
  lVar3 = FUN_0010d4d0(param_1,__name);
  uVar8 = DAT_0012bc68;
  *(__gid_t *)(lVar3 + 0x38) = _Var1;
  if (99 < *(ulong *)(lVar3 + 0x40)) {
    uVar8 = DAT_0012bc38;
  }
  *(undefined4 *)(lVar3 + 0x24) = uVar8;
  return 1;
}




// Function: parse_negate @ 0x12c20

undefined8 parse_negate(undefined8 param_1)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0010b9b0(param_1,0);
  puVar1[2] = 0x400000002;
  *puVar1 = FUN_00109350;
  *(undefined2 *)((long)puVar1 + 0x1a) = 0;
  return 1;
}




// Function: parse_openparen @ 0x12c60

undefined8 parse_openparen(undefined8 param_1)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0010b9b0(param_1,0);
  puVar1[2] = 4;
  *puVar1 = FUN_00109630;
  *(undefined2 *)((long)puVar1 + 0x1a) = 0;
  return 1;
}




// Function: optionl_getfilecon @ 0x12dc0

undefined8 optionl_getfilecon(undefined4 param_1,undefined8 param_2,undefined8 param_3)

{
  int iVar1;
  undefined8 uVar2;
  
  iVar1 = FUN_001242e0();
  if (iVar1 == 0) {
    return 0;
  }
  uVar2 = FUN_00112d30(param_1,param_2,param_3,iVar1);
  return uVar2;
}




// Function: parse_name @ 0x12fc0

undefined4 parse_name(undefined8 param_1,long param_2,int *param_3)

{
  int iVar1;
  undefined4 uVar2;
  long lVar3;
  char *pcVar4;
  undefined4 uVar5;
  
  if (param_2 == 0) {
    return 0;
  }
  iVar1 = *param_3;
  uVar2 = 0;
  pcVar4 = *(char **)(param_2 + (long)iVar1 * 8);
  if (pcVar4 != (char *)0x0) {
    *param_3 = iVar1 + 1;
    FUN_00111220();
    uVar2 = FUN_00112f00("-name","-wholename",pcVar4);
    if ((char)uVar2 != '\0') {
      lVar3 = FUN_0010d4d0(param_1,pcVar4);
      *(undefined2 *)(lVar3 + 0x1a) = 0;
      *(char **)(lVar3 + 0x38) = pcVar4;
      pcVar4 = strpbrk(pcVar4,"*?[");
      uVar5 = DAT_0012bc60;
      if (pcVar4 == (char *)0x0) {
        uVar5 = DAT_0012bc64;
      }
      *(undefined4 *)(lVar3 + 0x24) = uVar5;
      return uVar2;
    }
    *param_3 = iVar1;
  }
  return uVar2;
}




// Function: parse_iname @ 0x13090

undefined4 parse_iname(undefined8 param_1,long param_2,int *param_3)

{
  undefined4 uVar1;
  long lVar2;
  char *pcVar3;
  undefined4 uVar4;
  
  FUN_00111220();
  if (param_2 != 0) {
    pcVar3 = *(char **)(param_2 + (long)*param_3 * 8);
    if (pcVar3 != (char *)0x0) {
      *param_3 = *param_3 + 1;
      uVar1 = FUN_00112f00("-iname","-iwholename",pcVar3);
      if ((char)uVar1 != '\0') {
        lVar2 = FUN_0010d4d0(param_1,pcVar3);
        *(undefined2 *)(lVar2 + 0x1a) = 0;
        *(char **)(lVar2 + 0x38) = pcVar3;
        pcVar3 = strpbrk(pcVar3,"*?[");
        uVar4 = DAT_0012bc60;
        if (pcVar3 == (char *)0x0) {
          uVar4 = DAT_0012bc64;
        }
        *(undefined4 *)(lVar2 + 0x24) = uVar4;
        return uVar1;
      }
    }
  }
  return 0;
}




// Function: parse_regextype @ 0x13420

undefined8 parse_regextype(undefined8 param_1,long param_2,int *param_3)

{
  undefined8 uVar1;
  
  if (param_2 != 0) {
    if (*(long *)(param_2 + (long)*param_3 * 8) != 0) {
      *param_3 = *param_3 + 1;
      DAT_0013867c = FUN_00117200();
      uVar1 = FUN_001133b0();
      return uVar1;
    }
  }
  return 0;
}




// Function: insert_depthspec @ 0x13460

undefined8 insert_depthspec(undefined8 param_1,long param_2,int *param_3,int *param_4)

{
  char *__s;
  int iVar1;
  size_t sVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  
  iVar1 = *param_3;
  uVar3 = *(undefined8 *)(param_2 + -8 + (long)iVar1 * 8);
  if ((param_2 != 0) && (__s = *(char **)(param_2 + (long)iVar1 * 8), __s != (char *)0x0)) {
    *param_3 = iVar1 + 1;
    sVar2 = strspn(__s,"0123456789");
    if ((0 < (int)sVar2) && (__s[(int)sVar2] == '\0')) {
      iVar1 = FUN_00117410(__s,DAT_0013868c);
      *param_4 = iVar1;
      if (-1 < iVar1) {
        uVar3 = FUN_001133b0();
        return uVar3;
      }
    }
    uVar4 = FUN_001239f0(0,DAT_0013868c,__s);
    uVar5 = dcgettext(0,"Expected a positive decimal integer argument to %s, but got %s",5);
                    /* WARNING: Subroutine does not return */
    error(1,0,uVar5,uVar3,uVar4);
  }
  return 0;
}




// Function: parse_maxdepth @ 0x13520

void parse_maxdepth(void)

{
  FUN_00113460();
  return;
}




// Function: parse_newerXY @ 0x13620

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulong parse_newerXY(undefined8 param_1,long param_2,int *param_3,undefined8 param_4,
                  undefined8 param_5,undefined8 param_6)

{
  char cVar1;
  undefined *puVar2;
  float fVar3;
  long lVar4;
  char cVar5;
  int iVar6;
  int iVar7;
  size_t sVar8;
  char *pcVar9;
  long *plVar10;
  char *pcVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  int *piVar14;
  bool bVar15;
  ulong uVar16;
  long in_FS_OFFSET;
  float fVar17;
  undefined4 uVar18;
  undefined1 local_d8 [72];
  long local_90;
  long local_88;
  long local_80;
  long local_78;
  long local_70;
  long local_68;
  char local_46 [6];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 != 0) {
    iVar7 = *param_3;
    pcVar11 = *(char **)(param_2 + (long)iVar7 * 8);
    if ((pcVar11 != (char *)0x0) && (sVar8 = strlen(pcVar11), sVar8 == 8)) {
      builtin_strncpy(local_46,"aBcmt",6);
      iVar6 = strncmp("-newer",pcVar11,6);
      if (iVar6 != 0) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("0 == strncmp (\"-newer\", argv[*arg_ptr], 6)","parser.c",0x613,
                      "parse_newerXY");
      }
      cVar5 = pcVar11[6];
      cVar1 = pcVar11[7];
      bVar15 = cVar5 == 'B' || cVar1 == 'B';
      uVar16 = CONCAT71((int7)((ulong)param_6 >> 8),bVar15);
      if (bVar15) {
        uVar12 = dcgettext(0,"This system does not provide a way to find the birth time of a file.",
                           5);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar12);
      }
      if (cVar5 != 't') {
        pcVar9 = strchr(local_46,(int)cVar5);
        uVar16 = 0;
        if (pcVar9 != (char *)0x0) {
          pcVar9 = strchr(local_46,(int)cVar1);
          uVar16 = 0;
          if (pcVar9 != (char *)0x0) {
            if (*(long *)(param_2 + 8 + (long)iVar7 * 8) == 0) {
              uVar12 = FUN_001239f0(0,DAT_0013868c,pcVar11);
              uVar13 = dcgettext(0,"The %s test needs an argument",5);
                    /* WARNING: Subroutine does not return */
              error(1,0,uVar13,uVar12);
            }
            *param_3 = iVar7 + 1;
            plVar10 = (long *)FUN_0010d4d0(param_1);
            if (cVar5 == 'c') {
              uVar18 = 2;
            }
            else if (cVar5 == 'm') {
              uVar18 = 3;
            }
            else {
              if (cVar5 != 'a') {
                pcVar11 = strchr(local_46,(int)cVar5);
                if (pcVar11 == (char *)0x0) {
                    /* WARNING: Subroutine does not return */
                  __assert_fail("strchr (validchars, x)","parser.c",0x64b,"parse_newerXY");
                }
                    /* WARNING: Subroutine does not return */
                __assert_fail("0","parser.c",0x64c,"parse_newerXY");
              }
              uVar18 = 0;
            }
            *(undefined4 *)(plVar10 + 7) = uVar18;
            if (cVar1 == 't') {
              cVar5 = FUN_00121de0(plVar10 + 8,*(undefined8 *)(param_2 + (long)*param_3 * 8),
                                   &DAT_00138638);
              if (cVar5 == '\0') {
                uVar12 = FUN_001239f0(0,DAT_0013868c,*(undefined8 *)(param_2 + (long)*param_3 * 8));
                uVar13 = dcgettext(0,"I cannot figure out how to interpret %s as a date or time",5);
                    /* WARNING: Subroutine does not return */
                error(1,0,uVar13,uVar12);
              }
              local_70 = plVar10[8];
            }
            else {
              FUN_0010d810(local_d8);
              iVar7 = (*DAT_00138670)(*(undefined8 *)(param_2 + (long)*param_3 * 8),local_d8);
              if (iVar7 != 0) {
                piVar14 = __errno_location();
                    /* WARNING: Subroutine does not return */
                FUN_0010e920(*piVar14,*(undefined8 *)(param_2 + (long)*param_3 * 8));
              }
              if (cVar1 != 'c') {
                if (cVar1 < 'd') {
                  local_70 = local_90;
                  local_68 = local_88;
                  if (cVar1 != 'a') {
LAB_00113a43:
                    /* WARNING: Subroutine does not return */
                    __assert_fail("0","parser.c",0x1f2,"get_stat_Ytime");
                  }
                }
                else {
                  local_70 = local_80;
                  local_68 = local_78;
                  if (cVar1 != 'm') goto LAB_00113a43;
                }
              }
              plVar10[8] = local_70;
              plVar10[9] = local_68;
            }
            lVar4 = DAT_00138648;
            fVar3 = DAT_0012bc64;
            *(undefined4 *)((long)plVar10 + 0x3c) = 0;
            fVar17 = (float)(int)((lVar4 - local_70) / 0x15180);
            uVar18 = DAT_0012bc40;
            if (((fVar3 <= fVar17) && (uVar18 = DAT_0012bc74, DAT_00129578 <= fVar17)) &&
               (fVar17 <= _DAT_0012bc80)) {
              uVar18 = DAT_0012bc7c;
            }
            puVar2 = (undefined *)*plVar10;
            *(undefined4 *)((long)plVar10 + 0x24) = uVar18;
            *param_3 = *param_3 + 1;
            if (puVar2 == (undefined *)0x0) {
                    /* WARNING: Subroutine does not return */
              __assert_fail("our_pred->pred_func != NULL","parser.c",0x66f,"parse_newerXY");
            }
            if (puVar2 != PTR_FUN_00137fb0) {
                    /* WARNING: Subroutine does not return */
              __assert_fail("our_pred->pred_func == pred_newerXY","parser.c",0x670,"parse_newerXY");
            }
            uVar16 = (ulong)*(byte *)((long)plVar10 + 0x1a);
            if (*(byte *)((long)plVar10 + 0x1a) == 0) {
                    /* WARNING: Subroutine does not return */
              __assert_fail("our_pred->need_stat","parser.c",0x671,"parse_newerXY");
            }
          }
        }
      }
      goto LAB_001137b3;
    }
  }
  uVar16 = 0;
LAB_001137b3:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar16 & 0xffffffff;
}




// Function: parse_ls @ 0x13b00

undefined8 parse_ls(void)

{
  long lVar1;
  
  lVar1 = FUN_0010d520();
  FUN_00111c90("/dev/stdout",lVar1 + 0x38);
  *(undefined4 *)(lVar1 + 0x24) = 0x3f800000;
  *(undefined2 *)(lVar1 + 0x18) = 0x101;
  return 1;
}




// Function: parse_print0 @ 0x13b40

undefined8 parse_print0(undefined8 param_1)

{
  long lVar1;
  
  lVar1 = FUN_0010d4d0(param_1,0);
  FUN_00111c90("/dev/stdout",lVar1 + 0x38);
  *(undefined4 *)(lVar1 + 0x18) = 0x101;
  *(undefined4 *)(lVar1 + 0x24) = 0x3f800000;
  return 1;
}




// Function: parse_follow @ 0x13b80

void parse_follow(void)

{
  DAT_0013862c = 1;
  DAT_00138680 = FUN_00112dc0;
  DAT_00138668 = 1;
  if (((byte)DAT_00138660 & 2) == 0) {
    DAT_00138670 = FUN_0010ddb0;
    FUN_001133b0();
    return;
  }
  DAT_00138670 = FUN_0010dec0;
  FUN_001133b0();
  return;
}




// Function: get_relative_timestamp @ 0x13be0

undefined4
get_relative_timestamp(double param_1,char *param_2,long param_3,long param_4,long param_5,undefined8 param_6)

{
  undefined4 uVar1;
  long lVar2;
  long lVar3;
  long in_FS_OFFSET;
  double dVar4;
  double local_50;
  double local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (*param_2 == '+') {
    *(undefined4 *)(param_3 + 4) = 1;
    param_2 = param_2 + 1;
  }
  else if (*param_2 == '-') {
    *(undefined4 *)(param_3 + 4) = 0;
    param_2 = param_2 + 1;
  }
  else {
    *(undefined4 *)(param_3 + 4) = 2;
  }
  uVar1 = FUN_00125f90(param_2,0,&local_50,PTR_strtod_00137fe8);
  if ((char)uVar1 != '\0') {
    dVar4 = modf(param_1 * local_50,&local_48);
    dVar4 = dVar4 * DAT_0012bc88;
    if (DAT_0012bc88 <= dVar4) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("nanosec < nanosec_per_sec","parser.c",0xc4d,"get_relative_timestamp");
    }
    lVar2 = (long)((double)param_4 - local_48);
    *(long *)(param_3 + 8) = lVar2;
    if (param_4 < lVar2 != local_48 < 0.0) {
                    /* WARNING: Subroutine does not return */
      error(1,0,param_6,param_2);
    }
    lVar3 = (long)((double)param_5 - dVar4);
    if (dVar4 <= (double)param_5) {
      *(long *)(param_3 + 0x10) = lVar3;
    }
    else {
      *(long *)(param_3 + 0x10) = lVar3 + 1000000000;
      *(long *)(param_3 + 8) = lVar2 + -1;
    }
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_used @ 0x13d80

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 parse_used(undefined8 param_1,long param_2,int *param_3)

{
  float fVar1;
  undefined4 uVar2;
  long lVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  float fVar5;
  undefined4 uVar6;
  undefined8 local_38;
  long lStack_30;
  undefined8 local_28;
  long local_20;
  
  uVar4 = DAT_0012bc90;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 != 0) {
    lVar3 = *(long *)(param_2 + (long)*param_3 * 8);
    if (lVar3 != 0) {
      *param_3 = *param_3 + 1;
      uVar2 = FUN_00113be0(uVar4,lVar3,&local_38,0,0,
                           "arithmetic overflow while converting %s days to a number of seconds");
      if ((char)uVar2 == '\0') {
        uVar4 = dcgettext(0,"Invalid argument %s to -used",5);
                    /* WARNING: Subroutine does not return */
        error(1,0,uVar4,lVar3);
      }
      lVar3 = FUN_0010d4d0(param_1,lVar3);
      fVar1 = DAT_0012bc64;
      uVar6 = DAT_0012bc40;
      *(undefined8 *)(lVar3 + 0x38) = local_38;
      *(long *)(lVar3 + 0x40) = lStack_30;
      *(undefined8 *)(lVar3 + 0x48) = local_28;
      fVar5 = (float)(lStack_30 / 0x15180);
      if (((fVar1 <= fVar5) && (uVar6 = DAT_0012bc74, DAT_00129578 <= fVar5)) &&
         (fVar5 <= _DAT_0012bc80)) {
        uVar6 = DAT_0012bc7c;
      }
      *(undefined4 *)(lVar3 + 0x24) = uVar6;
      goto LAB_00113e6b;
    }
  }
  uVar2 = 0;
LAB_00113e6b:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: insert_num @ 0x13ed0

long insert_num(long param_1,int *param_2,undefined8 param_3)

{
  int iVar1;
  long lVar2;
  char *pcVar3;
  int iVar4;
  char *pcVar5;
  undefined *puVar6;
  long in_FS_OFFSET;
  undefined8 local_30;
  undefined1 local_28 [8];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 != 0) {
    pcVar3 = *(char **)(param_1 + (long)*param_2 * 8);
    lVar2 = 0;
    if (pcVar3 == (char *)0x0) goto LAB_00113f68;
    *param_2 = *param_2 + 1;
    if (*pcVar3 == '+') {
      pcVar5 = pcVar3 + 1;
      iVar4 = 0;
    }
    else if (*pcVar3 == '-') {
      pcVar5 = pcVar3 + 1;
      iVar4 = 1;
    }
    else {
      iVar4 = 2;
      pcVar5 = pcVar3;
    }
    iVar1 = FUN_00126490(pcVar5,local_28,10,&local_30,"");
    if (iVar1 == 0) {
      lVar2 = FUN_0010d4d0(param_3,pcVar3);
      *(int *)(lVar2 + 0x38) = iVar4;
      *(undefined8 *)(lVar2 + 0x40) = local_30;
      if (((byte)DAT_00138660 & 1) != 0) {
        __fprintf_chk(stderr,1,"inserting %s\n",*(undefined8 *)(lVar2 + 8));
        puVar6 = &DAT_0012acfe;
        pcVar3 = "gt";
        if (iVar4 != 0) {
          puVar6 = &DAT_0012acf8;
          if (iVar4 == 1) {
            puVar6 = &DAT_0012ad04;
          }
          pcVar3 = "eq";
          if (iVar4 == 1) {
            pcVar3 = "lt";
          }
        }
        __fprintf_chk(stderr,1,"    type: %s    %s  ",pcVar3,puVar6);
        __fprintf_chk(stderr,1,"%lu\n",*(undefined8 *)(lVar2 + 0x40));
      }
      goto LAB_00113f68;
    }
  }
  lVar2 = 0;
LAB_00113f68:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_links @ 0x14060

undefined8 parse_links(undefined8 param_1,undefined8 param_2,int *param_3)

{
  long lVar1;
  
  lVar1 = FUN_00113ed0(param_2,param_3,param_1);
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




// Function: parse_inum @ 0x140d0

undefined8 parse_inum(undefined8 param_1,undefined8 param_2,int *param_3)

{
  long lVar1;
  
  lVar1 = FUN_00113ed0(param_2,param_3,param_1);
  if (lVar1 != 0) {
    *(undefined4 *)(lVar1 + 0x24) = 0x358637bd;
    *(undefined2 *)(lVar1 + 0x1a) = 0;
    *(undefined1 *)(lVar1 + 0x1c) = 1;
    return 1;
  }
  *param_3 = *param_3 + -1;
  return 0;
}




// Function: parse_newer @ 0x141e0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 parse_newer(undefined8 param_1,long param_2,int *param_3)

{
  long lVar1;
  float fVar2;
  int iVar3;
  long lVar4;
  undefined8 uVar5;
  int *piVar6;
  long in_FS_OFFSET;
  float fVar7;
  undefined4 uVar8;
  undefined1 auStack_c8 [88];
  long local_70;
  undefined8 local_68;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010d810(auStack_c8);
  if (param_2 != 0) {
    lVar1 = *(long *)(param_2 + (long)*param_3 * 8);
    if (lVar1 != 0) {
      *param_3 = *param_3 + 1;
      iVar3 = (*DAT_00138670)(lVar1,auStack_c8);
      if (iVar3 != 0) {
        piVar6 = __errno_location();
                    /* WARNING: Subroutine does not return */
        FUN_0010e920(*piVar6,lVar1);
      }
      lVar4 = FUN_0010d4d0(param_1,lVar1);
      lVar1 = DAT_00138648;
      fVar2 = DAT_0012bc64;
      *(undefined8 *)(lVar4 + 0x48) = local_68;
      uVar8 = DAT_0012bc40;
      *(long *)(lVar4 + 0x40) = local_70;
      *(undefined8 *)(lVar4 + 0x38) = 3;
      fVar7 = (float)(int)((lVar1 - local_70) / 0x15180);
      if (((fVar2 <= fVar7) && (uVar8 = DAT_0012bc74, DAT_00129578 <= fVar7)) &&
         (fVar7 <= _DAT_0012bc80)) {
        uVar8 = DAT_0012bc7c;
      }
      *(undefined4 *)(lVar4 + 0x24) = uVar8;
      uVar5 = 1;
      goto LAB_001142ea;
    }
  }
  uVar5 = 0;
LAB_001142ea:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_anewer @ 0x14340

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 parse_anewer(undefined8 param_1,long param_2,int *param_3)

{
  long lVar1;
  float fVar2;
  int iVar3;
  long lVar4;
  undefined8 uVar5;
  int *piVar6;
  long in_FS_OFFSET;
  float fVar7;
  undefined4 uVar8;
  undefined1 auStack_c8 [88];
  long local_70;
  undefined8 local_68;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010d810(auStack_c8);
  if (param_2 != 0) {
    lVar1 = *(long *)(param_2 + (long)*param_3 * 8);
    if (lVar1 != 0) {
      *param_3 = *param_3 + 1;
      iVar3 = (*DAT_00138670)(lVar1,auStack_c8);
      if (iVar3 != 0) {
        piVar6 = __errno_location();
                    /* WARNING: Subroutine does not return */
        FUN_0010e920(*piVar6,lVar1);
      }
      lVar4 = FUN_0010d4d0(param_1,lVar1);
      lVar1 = DAT_00138648;
      fVar2 = DAT_0012bc64;
      *(undefined8 *)(lVar4 + 0x48) = local_68;
      uVar8 = DAT_0012bc40;
      *(long *)(lVar4 + 0x40) = local_70;
      *(undefined8 *)(lVar4 + 0x38) = 0;
      fVar7 = (float)(int)((lVar1 - local_70) / 0x15180);
      if (((fVar2 <= fVar7) && (uVar8 = DAT_0012bc74, DAT_00129578 <= fVar7)) &&
         (fVar7 <= _DAT_0012bc80)) {
        uVar8 = DAT_0012bc7c;
      }
      *(undefined4 *)(lVar4 + 0x24) = uVar8;
      uVar5 = 1;
      goto LAB_0011444a;
    }
  }
  uVar5 = 0;
LAB_0011444a:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_cnewer @ 0x144a0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 parse_cnewer(undefined8 param_1,long param_2,int *param_3)

{
  long lVar1;
  float fVar2;
  int iVar3;
  long lVar4;
  undefined8 uVar5;
  int *piVar6;
  long in_FS_OFFSET;
  float fVar7;
  undefined4 uVar8;
  undefined1 auStack_c8 [88];
  long local_70;
  undefined8 local_68;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010d810(auStack_c8);
  if (param_2 != 0) {
    lVar1 = *(long *)(param_2 + (long)*param_3 * 8);
    if (lVar1 != 0) {
      *param_3 = *param_3 + 1;
      iVar3 = (*DAT_00138670)(lVar1,auStack_c8);
      if (iVar3 != 0) {
        piVar6 = __errno_location();
                    /* WARNING: Subroutine does not return */
        FUN_0010e920(*piVar6,lVar1);
      }
      lVar4 = FUN_0010d4d0(param_1,lVar1);
      lVar1 = DAT_00138648;
      fVar2 = DAT_0012bc64;
      *(undefined8 *)(lVar4 + 0x48) = local_68;
      uVar8 = DAT_0012bc40;
      *(long *)(lVar4 + 0x40) = local_70;
      *(undefined8 *)(lVar4 + 0x38) = 2;
      fVar7 = (float)(int)((lVar1 - local_70) / 0x15180);
      if (((fVar2 <= fVar7) && (uVar8 = DAT_0012bc74, DAT_00129578 <= fVar7)) &&
         (fVar7 <= _DAT_0012bc80)) {
        uVar8 = DAT_0012bc7c;
      }
      *(undefined4 *)(lVar4 + 0x24) = uVar8;
      uVar5 = 1;
      goto LAB_001145aa;
    }
  }
  uVar5 = 0;
LAB_001145aa:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: do_parse_xmin @ 0x14600

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 do_parse_xmin(undefined8 param_1,long param_2,int *param_3,undefined4 param_4)

{
  int iVar1;
  float fVar2;
  undefined8 uVar3;
  undefined4 uVar4;
  long lVar5;
  long lVar6;
  long in_FS_OFFSET;
  float fVar7;
  undefined4 uVar8;
  undefined4 local_48;
  undefined4 uStack_44;
  long lStack_40;
  undefined8 local_38;
  long local_30;
  
  uVar3 = DAT_00138650;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 == 0) {
    uVar4 = 0;
  }
  else {
    iVar1 = *param_3;
    uVar4 = 0;
    lVar5 = *(long *)(param_2 + (long)iVar1 * 8);
    if (lVar5 != 0) {
      *param_3 = iVar1 + 1;
      local_48 = param_4;
      uVar4 = FUN_00113be0(_DAT_0012bc98,lVar5,&local_48,DAT_00138648 + 0x15180,uVar3,
                           "arithmetic overflow while converting %s minutes to a number of seconds")
      ;
      if ((char)uVar4 == '\0') {
        *param_3 = iVar1;
      }
      else {
        lVar5 = FUN_0010d4d0(param_1,lVar5);
        fVar2 = DAT_0012bc64;
        lVar6 = DAT_00138648 - lStack_40;
        *(ulong *)(lVar5 + 0x38) = CONCAT44(uStack_44,local_48);
        *(long *)(lVar5 + 0x40) = lStack_40;
        uVar8 = DAT_0012bc40;
        *(undefined8 *)(lVar5 + 0x48) = local_38;
        fVar7 = (float)(int)(lVar6 / 0x15180);
        if (((fVar2 <= fVar7) && (uVar8 = DAT_0012bc74, DAT_00129578 <= fVar7)) &&
           (fVar7 <= _DAT_0012bc80)) {
          uVar8 = DAT_0012bc7c;
        }
        *(undefined4 *)(lVar5 + 0x24) = uVar8;
      }
    }
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_mmin @ 0x14750

void parse_mmin(void)

{
  FUN_00114600();
  return;
}




// Function: parse_time @ 0x14780

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 parse_time(undefined8 param_1,long param_2,int *param_3)

{
  int iVar1;
  float fVar2;
  undefined8 uVar3;
  undefined4 uVar4;
  undefined8 uVar5;
  long lVar6;
  long lVar7;
  char *pcVar8;
  undefined *puVar9;
  long in_FS_OFFSET;
  float fVar10;
  undefined4 uVar11;
  long local_60;
  undefined4 local_58;
  int iStack_54;
  long lStack_50;
  undefined8 local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar5 = dcgettext(0,"arithmetic overflow while converting %s days to a number of seconds",5);
  lVar6 = DAT_00138648;
  uVar3 = DAT_0012bc90;
  if (param_2 == 0) {
    uVar4 = 0;
  }
  else {
    iVar1 = *param_3;
    uVar4 = 0;
    pcVar8 = *(char **)(param_2 + (long)iVar1 * 8);
    if (pcVar8 != (char *)0x0) {
      *param_3 = iVar1 + 1;
      if (*pcVar8 == '-') {
        lVar6 = lVar6 + 0x1517f;
      }
      uVar4 = FUN_00113be0(uVar3,pcVar8,&local_58,lVar6,DAT_00138650,uVar5);
      if ((char)uVar4 == '\0') {
        *param_3 = iVar1;
      }
      else {
        lVar6 = FUN_0010d4d0(param_1,pcVar8);
        fVar2 = DAT_0012bc64;
        lVar7 = DAT_00138648 - lStack_50;
        *(ulong *)(lVar6 + 0x38) = CONCAT44(iStack_54,local_58);
        *(long *)(lVar6 + 0x40) = lStack_50;
        uVar11 = DAT_0012bc40;
        *(undefined8 *)(lVar6 + 0x48) = local_48;
        fVar10 = (float)(int)(lVar7 / 0x15180);
        if (((fVar2 <= fVar10) && (uVar11 = DAT_0012bc74, DAT_00129578 <= fVar10)) &&
           (fVar10 <= _DAT_0012bc80)) {
          uVar11 = DAT_0012bc7c;
        }
        *(undefined4 *)(lVar6 + 0x24) = uVar11;
        if (((byte)DAT_00138660 & 1) != 0) {
          __fprintf_chk(stderr,1,"inserting %s\n",*(undefined8 *)(lVar6 + 8));
          if (iStack_54 == 0) {
            puVar9 = &DAT_0012acfe;
            pcVar8 = "gt";
          }
          else if (iStack_54 == 1) {
            puVar9 = &DAT_0012ad04;
            pcVar8 = "lt";
          }
          else {
            puVar9 = &DAT_0012ad2a;
            pcVar8 = "?";
            if (iStack_54 == 2) {
              __fprintf_chk(stderr,1,"    type: %s    %s  ",&DAT_0012acfb,&DAT_0012ad2d);
              local_60 = *(time_t *)(lVar6 + 0x40);
              pcVar8 = ctime(&local_60);
              __fprintf_chk(stderr,1,"%lu %s",*(undefined8 *)(lVar6 + 0x40),pcVar8);
              local_60 = *(long *)(lVar6 + 0x40) + 0x15180;
              pcVar8 = ctime(&local_60);
              __fprintf_chk(stderr,1,"                 <  %lu %s",local_60,pcVar8);
              goto LAB_00114820;
            }
          }
          __fprintf_chk(stderr,1,"    type: %s    %s  ",pcVar8,puVar9);
          local_60 = *(long *)(lVar6 + 0x40);
          pcVar8 = ctime(&local_60);
          __fprintf_chk(stderr,1,"%lu %s",*(undefined8 *)(lVar6 + 0x40),pcVar8);
        }
      }
    }
  }
LAB_00114820:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar4;
}




// Function: check_option_combinations @ 0x14a70

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
        if (pcVar1 != FUN_00108d00) break;
        param_1 = (long *)param_1[0x21];
        uVar2 = uVar2 | 1;
        if (param_1 == (long *)0x0) goto LAB_00114aba;
      }
      param_1 = (long *)param_1[0x21];
      if (pcVar1 == (code *)PTR_FUN_00137fb8) {
        uVar2 = uVar2 | 2;
      }
    } while (param_1 != (long *)0x0);
LAB_00114aba:
    if ((uVar2 == 3) && (DAT_00138621 == '\0')) {
      uVar3 = dcgettext(0,
                        "The -delete action automatically turns on -depth, but -prune does nothing when -depth is in effect.  If you want to carry on anyway, just explicitly use the -depth option."
                        ,5);
                    /* WARNING: Subroutine does not return */
      error(1,0,uVar3);
    }
  }
  return;
}




// Function: parse_begin_user_args @ 0x14ba0

void parse_begin_user_args(void)

{
  DAT_00138e70 = 0;
  return;
}




// Function: parse_end_user_args @ 0x14bb0

void parse_end_user_args(void)

{
  return;
}




// Function: find_parser @ 0x14bc0

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
    puVar7 = &DAT_00137840;
    if (sVar4 == 8) goto LAB_00114c4d;
  }
  cVar1 = *param_1;
  ppuVar6 = &PTR_DAT_00136d88;
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
  puVar7 = &DAT_00136d60 + (long)iVar2 * 0x20;
LAB_00114c4d:
  uVar5 = FUN_00112ca0(param_1,puVar7);
  return uVar5;
}




// Function: parse_fstype @ 0x14c70

undefined8 parse_fstype(undefined8 param_1,long param_2,int *param_3)

{
  char *__s1;
  ushort uVar1;
  char cVar2;
  int iVar3;
  undefined8 uVar4;
  long lVar5;
  char *__s2;
  long in_FS_OFFSET;
  undefined4 uVar6;
  stat sStack_c8;
  long local_30;
  
  uVar1 = DAT_00138688;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 == 0) {
    uVar4 = 0;
  }
  else {
    uVar4 = 0;
    __s1 = *(char **)(param_2 + (long)*param_3 * 8);
    if (__s1 != (char *)0x0) {
      *param_3 = *param_3 + 1;
      if (1 < uVar1) {
        cVar2 = FUN_00110a50(__s1);
        if (cVar2 == '\0') {
          if (((byte)DAT_00138660 & 8) != 0) {
            __fprintf_chk(stderr,1,"-fstype %s can never succeed, substituting -false\n",__s1);
          }
          uVar4 = FUN_00114bc0("false");
          lVar5 = FUN_0010d520(uVar4);
          uVar4 = 1;
          *(undefined4 *)(lVar5 + 0x18) = 0;
          *(undefined4 *)(lVar5 + 0x24) = 0;
          goto LAB_00114d03;
        }
      }
      lVar5 = FUN_0010d4d0(param_1,__s1);
      *(char **)(lVar5 + 0x38) = __s1;
      iVar3 = stat("/",&sStack_c8);
      uVar6 = DAT_00129578;
      if (iVar3 == 0) {
        __s2 = (char *)FUN_001108e0(&sStack_c8,&DAT_00128961);
        iVar3 = strcmp(__s1,__s2);
        uVar6 = DAT_0012bc84;
        if (iVar3 != 0) {
          uVar6 = DAT_0012bc74;
        }
      }
      *(undefined4 *)(lVar5 + 0x24) = uVar6;
      uVar4 = 1;
    }
  }
LAB_00114d03:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_false @ 0x14df0

undefined8 parse_false(void)

{
  undefined8 uVar1;
  long lVar2;
  
  uVar1 = FUN_00114bc0("false");
  lVar2 = FUN_0010d520(uVar1);
  *(undefined4 *)(lVar2 + 0x18) = 0;
  *(undefined4 *)(lVar2 + 0x24) = 0;
  return 1;
}




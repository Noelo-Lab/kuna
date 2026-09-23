// Function: main @ 0x3f80

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8
main(ulong param_1,undefined8 *param_2,undefined8 param_3,undefined8 param_4,long *param_5)

{
  undefined8 *puVar1;
  bool bVar2;
  bool bVar3;
  bool bVar4;
  char cVar5;
  char cVar6;
  int iVar7;
  int iVar8;
  char *pcVar9;
  char *pcVar10;
  lconv *plVar11;
  undefined8 *__ptr;
  long lVar12;
  ulong uVar13;
  ulong uVar14;
  undefined8 *puVar15;
  undefined8 *puVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  uint uVar19;
  long extraout_RDX;
  long *extraout_RDX_00;
  char *pcVar20;
  long *plVar21;
  uint uVar22;
  char *pcVar23;
  int *piVar24;
  ulong uVar25;
  long in_FS_OFFSET;
  undefined1 auVar26 [16];
  long *local_2f8;
  long *local_2f0;
  long *local_2e0;
  ulong local_2d8;
  long *local_2d0;
  uint local_2ac;
  undefined1 local_2a8 [80];
  undefined8 local_258 [4];
  undefined8 local_238;
  undefined8 local_230;
  undefined1 local_228;
  undefined1 local_227;
  undefined1 local_226;
  byte local_225;
  undefined1 local_224;
  undefined1 local_223;
  undefined1 local_222;
  byte local_221;
  undefined1 local_220;
  undefined8 local_208;
  undefined8 *local_200;
  _union_1457 local_e8;
  undefined8 local_e0;
  undefined8 uStack_d8;
  undefined8 local_d0;
  undefined8 uStack_c8;
  undefined8 local_c0;
  undefined8 uStack_b8;
  undefined8 local_b0;
  undefined8 uStack_a8;
  undefined8 local_a0;
  undefined8 uStack_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined8 local_70;
  undefined8 uStack_68;
  undefined4 local_60;
  undefined1 local_42;
  undefined1 local_41;
  long local_40;
  
  param_1 = param_1 & 0xffffffff;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  pcVar9 = getenv("POSIXLY_CORRECT");
  iVar7 = FUN_0010ee70();
  bVar3 = 0x2b8 < iVar7 - 0x30db0U;
  FUN_0010ef00(*param_2);
  pcVar10 = setlocale(6,"");
  bindtextdomain("coreutils");
  textdomain("coreutils");
  FUN_00108f60(3);
  DAT_0011e861 = FUN_0010cfb0();
  DAT_0011e860 = FUN_0010cfb0(2);
  plVar11 = localeconv();
  DAT_0011e868 = *plVar11->decimal_point;
  if ((DAT_0011e868 == '\0') || (plVar11->decimal_point[1] != '\0')) {
    DAT_0011e868 = '.';
  }
  cVar6 = *plVar11->thousands_sep;
  DAT_0011e864 = (int)cVar6;
  if (cVar6 == '\0') goto LAB_001045bc;
  if (plVar11->thousands_sep[1] != '\0') goto LAB_001045b5;
LAB_0010407e:
  DAT_0011e418 = '\0';
  FUN_00105b80();
  piVar24 = &DAT_00115324;
  sigemptyset((sigset_t *)&DAT_0011e380);
  local_2f0 = (long *)&DAT_00115324;
  iVar7 = 0xe;
  while( true ) {
    sigaction(iVar7,(sigaction *)0x0,(sigaction *)&local_e8);
    if (local_e8.sa_handler != (__sighandler_t)0x1) {
      sigaddset((sigset_t *)&DAT_0011e380,iVar7);
    }
    if (piVar24 == (int *)&DAT_0011534c) break;
    iVar7 = *piVar24;
    piVar24 = piVar24 + 1;
  }
  local_60 = 0;
  local_e8.sa_handler = FUN_00105b40;
  iVar7 = 0xe;
  local_e0 = _DAT_0011e380;
  uStack_d8 = uRam000000000011e388;
  local_d0 = _DAT_0011e390;
  uStack_c8 = uRam000000000011e398;
  local_c0 = _DAT_0011e3a0;
  uStack_b8 = uRam000000000011e3a8;
  local_b0 = _DAT_0011e3b0;
  uStack_a8 = uRam000000000011e3b8;
  local_a0 = _DAT_0011e3c0;
  uStack_98 = uRam000000000011e3c8;
  local_90 = _DAT_0011e3d0;
  uStack_88 = uRam000000000011e3d8;
  local_80 = _DAT_0011e3e0;
  uStack_78 = uRam000000000011e3e8;
  local_70 = _DAT_0011e3f0;
  uStack_68 = uRam000000000011e3f8;
  while( true ) {
    iVar8 = sigismember((sigset_t *)&DAT_0011e380,iVar7);
    if (iVar8 != 0) {
      sigaction(iVar7,(sigaction *)&local_e8,(sigaction *)0x0);
    }
    if (local_2f0 == (long *)&DAT_0011534c) break;
    iVar7 = *(int *)local_2f0;
    local_2f0 = (long *)((long)local_2f0 + 4);
  }
  uVar25 = 0;
  uVar22 = 0;
  signal(0x11,(__sighandler_t)0x0);
  pcVar23 = "-bcCdfghik:mMno:rRsS:t:T:uVy:z";
  FUN_00114980(FUN_00105ad0);
  FUN_00105cf0(local_258);
  local_258[0] = 0xffffffffffffffff;
  __ptr = (undefined8 *)FUN_00112fd0((long)(int)param_1);
  local_2f8 = (long *)0x0;
  uVar14 = 0;
  local_2f0 = (long *)0x0;
  local_2d8 = 0;
  local_2e0 = (long *)0x0;
  bVar4 = false;
  iVar7 = DAT_0011e028;
LAB_001042ce:
  DAT_0011e028 = iVar7;
  local_208._0_4_ = 0xffffffff;
  iVar8 = (int)param_1;
  if ((int)uVar14 == -1) {
LAB_001042de:
    lVar12 = (long)optind;
    goto LAB_001042b0;
  }
  if ((uVar25 == 0) || (pcVar9 == (char *)0x0)) {
LAB_00104310:
    param_5 = &local_208;
    auVar26 = getopt_long(param_1 & 0xffffffff,param_2,pcVar23,&PTR_s_ignore_leading_blanks_0011d5a0
                         );
    plVar21 = optarg;
    uVar13 = auVar26._0_8_;
    uVar19 = auVar26._0_4_;
    uVar14 = uVar13 & 0xffffffff;
    if (uVar19 == 0xffffffff) goto LAB_001042de;
    if ((int)uVar19 < 0x88) {
      if ((int)uVar19 < 1) {
        if (uVar19 == 0xffffff7d) {
          FUN_00112d40(stdout,"sort","GNU coreutils",PTR_DAT_0011e120,"Mike Haertel","Paul Eggert",0
                       ,uVar13);
                    /* WARNING: Subroutine does not return */
          exit(0);
        }
        if (uVar19 == 0xffffff7e) {
          FUN_0010be50(0);
          goto LAB_00104458;
        }
        goto switchD_00104361_caseD_2;
      }
      iVar7 = DAT_0011e028;
      switch(uVar13 & 0xffffffff) {
      case 1:
        if ((char)*optarg == '+') {
          if (optind == iVar8) goto LAB_00104dba;
          bVar2 = false;
          if (*(char *)param_2[optind] == '-') {
            bVar2 = (int)((char *)param_2[optind])[1] - 0x30U < 10;
          }
          goto LAB_00104b66;
        }
        goto LAB_001049b7;
      default:
        goto switchD_00104361_caseD_2;
      case 0x53:
        local_2d0 = (long *)(uVar13 & 0xffffffff);
        FUN_001089b0((undefined4)local_208);
        uVar14 = uVar13 & 0xffffffff;
        iVar7 = DAT_0011e028;
        break;
      case 0x54:
        local_2d0 = (long *)(uVar13 & 0xffffffff);
        FUN_001061e0(optarg);
        uVar14 = uVar13 & 0xffffffff;
        iVar7 = DAT_0011e028;
        break;
      case 0x6b:
        param_5 = (long *)FUN_00105cf0(local_2a8);
        pcVar20 = (char *)FUN_00105d20(optarg,param_5);
        lVar12 = *param_5;
        *param_5 = lVar12 + -1;
        if (lVar12 == 0) {
LAB_001052ce:
                    /* WARNING: Subroutine does not return */
          FUN_00105a10(optarg,"field number is zero");
        }
        if (*pcVar20 == '.') {
          pcVar20 = (char *)FUN_00105d20(pcVar20 + 1,param_5 + 1,"invalid number after \'.\'");
          lVar12 = param_5[1];
          param_5[1] = lVar12 + -1;
          if (lVar12 == 0) {
                    /* WARNING: Subroutine does not return */
            FUN_00105a10(optarg,"character offset is zero");
          }
        }
        if ((*param_5 == 0) && (param_5[1] == 0)) {
          *param_5 = -1;
        }
        pcVar20 = (char *)FUN_001057c0(pcVar20,param_5,0);
        if (*pcVar20 == ',') {
          pcVar20 = (char *)FUN_00105d20(pcVar20 + 1,param_5 + 2,"invalid number after \',\'");
          lVar12 = param_5[2];
          param_5[2] = lVar12 + -1;
          if (lVar12 == 0) goto LAB_001052ce;
          if (*pcVar20 == '.') {
            pcVar20 = (char *)FUN_00105d20(pcVar20 + 1,param_5 + 3,"invalid number after \'.\'");
          }
          pcVar20 = (char *)FUN_001057c0(pcVar20,param_5,1);
        }
        else {
          param_5[2] = -1;
          param_5[3] = 0;
        }
        if (*pcVar20 != '\0') {
                    /* WARNING: Subroutine does not return */
          FUN_00105a10(optarg,"stray character in field spec");
        }
        local_2d0 = (long *)(uVar13 & 0xffffffff);
        FUN_001060c0(param_5);
        uVar14 = uVar13 & 0xffffffff;
        iVar7 = DAT_0011e028;
        break;
      case 0x6d:
        bVar4 = true;
        break;
      case 0x6f:
        if (local_2f0 != (long *)0x0) {
          local_2d0 = (long *)(uVar13 & 0xffffffff);
          iVar7 = strcmp((char *)local_2f0,(char *)optarg);
          if (iVar7 != 0) {
            uVar17 = dcgettext(0,"multiple output files specified",5);
                    /* WARNING: Subroutine does not return */
            error(2,0,uVar17);
          }
        }
        uVar14 = uVar13 & 0xffffffff;
        local_2f0 = plVar21;
        iVar7 = DAT_0011e028;
        break;
      case 0x73:
        DAT_0011e41a = 1;
        break;
      case 0x74:
        iVar7 = (int)(char)*optarg;
        if ((char)*optarg == '\0') {
          uVar17 = dcgettext(0,"empty tab",5);
                    /* WARNING: Subroutine does not return */
          error(2,0,uVar17);
        }
        if (*(char *)((long)optarg + 1) != '\0') {
          local_2d0 = optarg;
          iVar7 = strcmp((char *)optarg,"\\0");
          if (iVar7 != 0) {
            uVar17 = FUN_00111430();
            uVar18 = dcgettext(0,"multi-character tab %s",5);
                    /* WARNING: Subroutine does not return */
            error(2,0,uVar18,uVar17);
          }
          iVar7 = 0;
        }
        uVar14 = uVar13 & 0xffffffff;
        if ((DAT_0011e028 != 0x80) && (DAT_0011e028 != iVar7)) {
          uVar17 = dcgettext(0,"incompatible tabs",5);
                    /* WARNING: Subroutine does not return */
          error(2,0,uVar17);
        }
        break;
      case 0x75:
        DAT_0011e419 = 1;
        break;
      case 0x79:
        if ((long *)param_2[(long)optind + -1] == optarg) {
          while ((int)(char)*plVar21 - 0x30U < 10) {
            plVar21 = (long *)((long)plVar21 + 1);
          }
          optind = optind + -1 + (uint)((char)*plVar21 == '\0');
        }
        break;
      case 0x7a:
        DAT_0011e038 = 0;
        break;
      case 0x80:
        uVar14 = 99;
        if (optarg != (long *)0x0) {
          param_5 = (long *)0x1;
          lVar12 = FUN_0010c4b0("--check",optarg,&PTR_s_quiet_0011d580,&DAT_00115466,1,
                                PTR_FUN_0011e128,1,auVar26._8_8_);
          uVar14 = (ulong)(uint)(int)(char)(&DAT_00115466)[lVar12];
        }
      case 0x43:
      case 99:
        local_2ac = (uint)uVar14;
        if (((char)uVar22 == '\0') || (uVar22 == local_2ac)) {
          uVar22 = (uint)(char)uVar14;
          iVar7 = DAT_0011e028;
        }
        else {
          FUN_00105940(&DAT_00116e40);
LAB_00105180:
          if ((char)uVar22 == '\0') {
            uVar22 = 0x6f;
          }
LAB_0010518b:
          pcVar20 = s_X___debug_0011e018;
          s_X___debug_0011e018[0] = (char)uVar22;
          FUN_00105940(s_X___debug_0011e018);
LAB_0010519e:
          pcVar20 = (char *)FUN_00105d20(pcVar20 + 1,param_5 + 3,"invalid number after \'.\'");
LAB_00104e57:
          if ((param_5[3] == 0) && (param_5[2] != 0)) {
            param_5[2] = param_5[2] + -1;
          }
          pcVar20 = (char *)FUN_001057c0(pcVar20,param_5,1);
          if (*pcVar20 != '\0') {
                    /* WARNING: Subroutine does not return */
            FUN_00105a10(local_2d0,"stray character in field spec");
          }
LAB_00104e97:
          *(undefined1 *)((long)param_5 + 0x39) = 1;
          local_2d0 = (long *)(ulong)local_2ac;
          FUN_001060c0(param_5);
          uVar14 = (ulong)local_2ac;
          iVar7 = DAT_0011e028;
        }
        break;
      case 0x81:
        if (DAT_0011e408 != (long *)0x0) {
          local_2d0 = optarg;
          iVar7 = strcmp((char *)DAT_0011e408,(char *)optarg);
          if (iVar7 != 0) {
            uVar17 = dcgettext(0,"multiple compress programs specified",5);
                    /* WARNING: Subroutine does not return */
            error(2,0,uVar17);
          }
        }
        uVar14 = uVar13 & 0xffffffff;
        DAT_0011e408 = plVar21;
        iVar7 = DAT_0011e028;
        break;
      case 0x82:
        DAT_0011e400 = '\x01';
        break;
      case 0x83:
        local_2f8 = optarg;
        break;
      case 0x84:
        local_2d0 = (long *)(uVar13 & 0xffffffff);
        FUN_00108b60((undefined4)local_208);
        uVar14 = uVar13 & 0xffffffff;
        iVar7 = DAT_0011e028;
        break;
      case 0x85:
        if (local_2e0 != (long *)0x0) {
          local_2d0 = (long *)(uVar13 & 0xffffffff);
          iVar7 = strcmp((char *)local_2e0,(char *)optarg);
          if (iVar7 != 0) {
            uVar17 = dcgettext(0,"multiple random sources specified",5);
                    /* WARNING: Subroutine does not return */
            error(2,0,uVar17);
          }
        }
        uVar14 = uVar13 & 0xffffffff;
        local_2e0 = plVar21;
        iVar7 = DAT_0011e028;
        break;
      case 0x86:
        plVar21 = (long *)0x1;
        lVar12 = FUN_0010c4b0("--sort",optarg,&PTR_s_general_numeric_0011d540,&DAT_00115460,1,
                              PTR_FUN_0011e128,1,param_5);
        uVar19 = (uint)(char)(&DAT_00115460)[lVar12];
        param_5 = plVar21;
      case 0x4d:
      case 0x52:
      case 0x56:
      case 0x62:
      case 100:
      case 0x66:
      case 0x67:
      case 0x68:
      case 0x69:
      case 0x6e:
      case 0x72:
        local_2d0 = (long *)(ulong)uVar19;
        local_42 = (undefined1)uVar19;
        local_41 = 0;
        FUN_001057c0(&local_42,local_258,2);
        uVar14 = (ulong)uVar19;
        iVar7 = DAT_0011e028;
        break;
      case 0x87:
        goto switchD_00104361_caseD_87;
      }
    }
    else {
switchD_00104361_caseD_2:
      FUN_0010be50(2);
      plVar21 = extraout_RDX_00;
LAB_00104dba:
      bVar2 = false;
LAB_00104b66:
      bVar3 = (bool)(bVar3 | pcVar9 == (char *)0x0 & bVar2);
      if (bVar3) {
        local_2ac = (uint)uVar14;
        auVar26 = FUN_00105cf0(local_2a8);
        param_5 = auVar26._0_8_;
        pcVar20 = (char *)FUN_00105d20(auVar26._8_8_ + 1,param_5,0);
        if (pcVar20 == (char *)0x0) {
          if ((*param_5 == 0) && (param_5[1] == 0)) {
            *param_5 = -1;
          }
LAB_00104bea:
          uVar14 = uVar14 & 0xffffffff;
          plVar21 = optarg;
          local_2d0 = param_5;
          goto LAB_001049b7;
        }
        if (*pcVar20 == '.') {
          pcVar20 = (char *)FUN_00105d20(pcVar20 + 1,param_5 + 1,0);
          if ((*param_5 == 0) && (param_5[1] == 0)) goto LAB_00104bda;
LAB_00104be1:
          if (pcVar20 == (char *)0x0) goto LAB_00104bea;
        }
        else if ((*param_5 == 0) && (param_5[1] == 0)) {
LAB_00104bda:
          *param_5 = -1;
          goto LAB_00104be1;
        }
        pcVar20 = (char *)FUN_001057c0(pcVar20,param_5,0);
        if (*pcVar20 != '\0') goto LAB_00104bea;
        if (bVar2) {
          lVar12 = (long)optind;
          optind = optind + 1;
          local_2d0 = (long *)param_2[lVar12];
          pcVar20 = (char *)FUN_00105d20((long)local_2d0 + 1,param_5 + 2,
                                         "invalid number after \'-\'");
          if (pcVar20 == (char *)0x0) {
                    /* WARNING: Subroutine does not return */
            __assert_fail("s","src/sort.c",0x1161,"main");
          }
          if (*pcVar20 == '.') goto LAB_0010519e;
          goto LAB_00104e57;
        }
        goto LAB_00104e97;
      }
LAB_001049b7:
      __ptr[uVar25] = plVar21;
      uVar25 = uVar25 + 1;
      iVar7 = DAT_0011e028;
    }
    goto LAB_001042ce;
  }
  lVar12 = (long)optind;
  if ((bVar3) && ((char)uVar22 == '\0')) {
    if (optind != iVar8) {
      pcVar20 = (char *)param_2[optind];
      if (((*pcVar20 == '-') && (pcVar20[1] == 'o')) &&
         ((pcVar20[2] != '\0' || (optind + 1 != iVar8)))) goto LAB_00104310;
      goto LAB_001042b0;
    }
LAB_00104458:
    if (local_2f8 != (long *)0x0) goto LAB_00104d4b;
    uVar22 = 0;
  }
  else {
LAB_001042b0:
    if ((int)lVar12 < iVar8) {
      optind = (int)lVar12 + 1;
      __ptr[uVar25] = param_2[lVar12];
      uVar25 = uVar25 + 1;
      iVar7 = DAT_0011e028;
      goto LAB_001042ce;
    }
    if (local_2f8 != (long *)0x0) {
      if (uVar25 != 0) {
LAB_00104d4b:
        uVar17 = FUN_00111170(4,*__ptr);
        uVar18 = dcgettext(0,"extra operand %s",5);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar18,uVar17);
      }
      pcVar23 = (char *)&local_208;
      param_2 = (undefined8 *)FUN_00106010(local_2f8,"r");
      FUN_00111ee0(pcVar23);
      cVar6 = FUN_00111f90(param_2,pcVar23);
      if (cVar6 == '\0') {
        uVar17 = FUN_00111170(4);
        uVar18 = dcgettext(0,"cannot read file names from %s",5);
                    /* WARNING: Subroutine does not return */
        error(2,0,uVar18,uVar17);
      }
      FUN_00106040(param_2);
      if (CONCAT44(local_208._4_4_,(undefined4)local_208) == 0) {
        uVar17 = FUN_00111170(4,local_2f8);
        uVar18 = dcgettext(0,"no input from %s",5);
                    /* WARNING: Subroutine does not return */
        error(2,0,uVar18,uVar17);
      }
      free(__ptr);
      __ptr = local_200;
      param_1 = CONCAT44(local_208._4_4_,(undefined4)local_208);
      if (param_1 != 0) {
        param_2 = (undefined8 *)&DAT_00117ace;
        uVar25 = 0;
        do {
          pcVar23 = (char *)__ptr[uVar25];
          iVar7 = strcmp(pcVar23,"-");
          if (iVar7 == 0) {
            uVar17 = FUN_00111170(4,pcVar23);
            uVar18 = dcgettext(0,"when reading file names from stdin, no file name of %s allowed",5)
            ;
                    /* WARNING: Subroutine does not return */
            error(2,0,uVar18,uVar17);
          }
          uVar25 = uVar25 + 1;
          if (*pcVar23 == '\0') {
            uVar17 = FUN_00111270(0,3,local_2f8);
            uVar18 = dcgettext(0,"%s:%lu: invalid zero-length file name",5);
                    /* WARNING: Subroutine does not return */
            error(2,0,uVar18,uVar17,uVar25);
          }
        } while (param_1 != uVar25);
      }
    }
  }
  local_2ac = (uint)uVar14;
  cVar6 = (char)uVar22;
  if (DAT_0011e410 == 0) {
    cVar5 = FUN_00105680(local_258);
    if (cVar5 == '\0') {
      param_2 = (undefined8 *)0x1;
      FUN_001060c0(local_258);
      param_1 = (ulong)local_225;
      goto LAB_00104532;
    }
    FUN_00105970();
    if (DAT_0011e400 == '\0') {
      DAT_0011e41b = local_221;
      goto LAB_00104560;
    }
    param_1 = 0;
    param_2 = (undefined8 *)0x0;
  }
  else {
    param_1 = 0;
    lVar12 = DAT_0011e410;
    do {
      while( true ) {
        cVar5 = FUN_00105680(lVar12);
        local_2ac = (uint)uVar14;
        if ((cVar5 != '\0') && (*(char *)(extraout_RDX + 0x37) == '\0')) break;
        lVar12 = *(long *)(extraout_RDX + 0x40);
        param_1 = (ulong)((uint)param_1 | (uint)*(byte *)(extraout_RDX + 0x33));
        if (lVar12 == 0) goto LAB_0010452f;
      }
      uVar14 = (ulong)local_221;
      local_2ac = (uint)local_221;
      *(undefined8 *)(extraout_RDX + 0x20) = local_238;
      *(undefined8 *)(extraout_RDX + 0x28) = local_230;
      *(undefined1 *)(extraout_RDX + 0x30) = local_228;
      *(undefined1 *)(extraout_RDX + 0x31) = local_227;
      *(undefined1 *)(extraout_RDX + 0x36) = local_222;
      *(undefined1 *)(extraout_RDX + 0x32) = local_226;
      *(undefined1 *)(extraout_RDX + 0x34) = local_224;
      *(undefined1 *)(extraout_RDX + 0x35) = local_223;
      *(byte *)(extraout_RDX + 0x37) = local_221;
      *(undefined1 *)(extraout_RDX + 0x38) = local_220;
      *(byte *)(extraout_RDX + 0x33) = local_225;
      lVar12 = *(long *)(extraout_RDX + 0x40);
      param_1 = (ulong)((uint)param_1 | (uint)local_225);
    } while (lVar12 != 0);
LAB_0010452f:
    param_2 = (undefined8 *)0x0;
LAB_00104532:
    FUN_00105970();
    if (DAT_0011e400 == '\0') goto code_r0x00104544;
  }
  if (cVar6 == '\0') {
    if (local_2f0 == (long *)0x0) {
      if ((pcVar10 != (char *)0x0) && (pcVar9 = setlocale(3,""), pcVar9 != (char *)0x0)) {
        if (DAT_0011e861 == '\0') {
          uVar17 = dcgettext(0,"text ordering performed using simple byte comparison",5);
                    /* WARNING: Subroutine does not return */
          error(0,0,&DAT_0011717b,uVar17);
        }
        pcVar9 = setlocale(3,(char *)0x0);
        uVar17 = FUN_00111430(pcVar9);
        uVar18 = dcgettext(0,"text ordering performed using %s sorting rules",5);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar18,uVar17);
      }
      uVar17 = dcgettext(0,"failed to set locale",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,&DAT_0011717b,uVar17);
    }
    goto LAB_00105180;
  }
  goto LAB_0010518b;
switchD_00104361_caseD_87:
  local_2d0 = (long *)(uVar13 & 0xffffffff);
  local_2d8 = FUN_00108900((undefined4)local_208);
  uVar14 = uVar13 & 0xffffffff;
  iVar7 = DAT_0011e028;
  goto LAB_001042ce;
code_r0x00104544:
  DAT_0011e41b = local_221;
  if ((char)param_1 != '\0') {
    FUN_00106110(local_2e0);
  }
LAB_00104560:
  if (DAT_0011e428 == 0) {
    pcVar23 = getenv("TMPDIR");
    if (pcVar23 == (char *)0x0) {
      pcVar23 = "/tmp";
    }
    FUN_001061e0(pcVar23);
  }
  if (uVar25 == 0) {
    uVar25 = 1;
    free(__ptr);
    __ptr = (undefined8 *)FUN_00112eb0(8);
    *__ptr = &DAT_00117ace;
  }
  uVar14 = DAT_0011e438;
  if ((DAT_0011e438 != 0) &&
     (uVar14 = (ulong)DAT_0011e024 * 0x22, (ulong)DAT_0011e024 * 0x22 < DAT_0011e438)) {
    uVar14 = DAT_0011e438;
  }
  DAT_0011e438 = uVar14;
  if (cVar6 == '\0') {
    FUN_00106c00(__ptr,uVar25);
    FUN_00106ca0(local_2f0);
    if (bVar4) {
      puVar15 = (undefined8 *)FUN_00113150(uVar25,0x10);
      puVar1 = __ptr + uVar25;
      puVar16 = puVar15;
      do {
        uVar17 = *__ptr;
        __ptr = __ptr + 1;
        *puVar16 = uVar17;
        puVar16 = puVar16 + 2;
      } while (__ptr != puVar1);
      FUN_0010b720(puVar15,0,uVar25,local_2f0);
    }
    else {
      if (local_2d8 == 0) {
        uVar14 = FUN_0010e9d0(2);
        local_2d8 = 8;
        if (uVar14 < 9) {
          local_2d8 = uVar14;
        }
      }
      if (0xffffffffffffff < local_2d8) {
        local_2d8 = 0xffffffffffffff;
      }
      FUN_0010bac0(__ptr,uVar25,local_2f0,local_2d8);
    }
    if ((DAT_0011e418 != '\0') && (iVar7 = FUN_0010c900(stdin), iVar7 == -1)) {
      uVar17 = dcgettext(0,"close failed",5);
                    /* WARNING: Subroutine does not return */
      FUN_00105e10(uVar17,&DAT_00117ace);
    }
    if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    return 0;
  }
  if (uVar25 != 1) {
    uVar17 = FUN_00111170(4,__ptr[1]);
    uVar18 = dcgettext(0,"extra operand %s not allowed with -%c",5);
                    /* WARNING: Subroutine does not return */
    error(2,0,uVar18,uVar17,(int)cVar6);
  }
  if (local_2f0 == (long *)0x0) {
    uVar22 = FUN_0010aac0(*__ptr,(int)cVar6);
                    /* WARNING: Subroutine does not return */
    exit((uVar22 ^ 1) & 0xff);
  }
  DAT_0011e010 = cVar6;
  FUN_00105940(&DAT_0011e010);
LAB_001045b5:
  DAT_0011e862 = 1;
LAB_001045bc:
  DAT_0011e864 = 0x80;
  goto LAB_0010407e;
}




// Function: proctab_hasher @ 0x5400

ulong proctab_hasher(long param_1,ulong param_2)

{
  return (ulong)(long)*(int *)(param_1 + 8) % param_2;
}




// Function: proctab_comparator @ 0x5420

undefined4 proctab_comparator(long param_1,long param_2)

{
  return CONCAT31((int3)((uint)*(int *)(param_2 + 8) >> 8),
                  *(int *)(param_1 + 8) == *(int *)(param_2 + 8));
}




// Function: traverse_raw_number @ 0x5430

int traverse_raw_number(long *param_1)

{
  char *pcVar1;
  char *pcVar2;
  int iVar3;
  char cVar4;
  char *pcVar5;
  int iVar6;
  
  pcVar2 = (char *)*param_1;
  cVar4 = *pcVar2;
  iVar3 = (int)cVar4;
  pcVar5 = pcVar2 + 1;
  if (iVar3 - 0x30U < 10) {
    iVar6 = 0;
    do {
      if ((char)iVar6 < (char)iVar3) {
        iVar6 = iVar3;
      }
      pcVar1 = pcVar2 + 1;
      pcVar2 = pcVar2 + 2;
      if (*pcVar1 != DAT_0011e864) {
        pcVar2 = pcVar5;
      }
      cVar4 = *pcVar2;
      iVar3 = (int)cVar4;
      pcVar5 = pcVar2 + 1;
    } while (iVar3 - 0x30U < 10);
    if (*pcVar1 == DAT_0011e864) {
      *param_1 = (long)(pcVar2 + -1);
      return iVar6;
    }
  }
  else {
    iVar6 = 0;
  }
  if (DAT_0011e868 == cVar4) {
    cVar4 = *pcVar5;
    while ((int)cVar4 - 0x30U < 10) {
      pcVar5 = pcVar5 + 1;
      if ((char)iVar6 < cVar4) {
        iVar6 = (int)cVar4;
      }
      cVar4 = *pcVar5;
    }
  }
  else {
    pcVar5 = pcVar5 + -1;
  }
  *param_1 = (long)pcVar5;
  return iVar6;
}




// Function: find_unit_order @ 0x54f0

int find_unit_order(char *param_1)

{
  char cVar1;
  int iVar2;
  char cVar3;
  long in_FS_OFFSET;
  byte *local_18;
  long local_10;
  
  cVar3 = *param_1;
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_18 = (byte *)(param_1 + (cVar3 == '-'));
  cVar1 = FUN_00105430(&local_18);
  iVar2 = 0;
  if ('0' < cVar1) {
    iVar2 = (int)(char)(&DAT_00115360)[*local_18];
    if (cVar3 == '-') {
      iVar2 = -(int)(char)(&DAT_00115360)[*local_18];
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return iVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: getmonth @ 0x5570

undefined4 getmonth(char *param_1,undefined8 *param_2)

{
  char *pcVar1;
  byte bVar2;
  byte bVar3;
  uint uVar4;
  ulong uVar5;
  char *pcVar6;
  char extraout_DL;
  long extraout_RDX;
  ulong uVar7;
  ulong uVar8;
  ulong uVar9;
  char *pcVar10;
  undefined1 *puVar11;
  ulong uVar12;
  
  while( true ) {
    uVar4 = FUN_001053e0((int)*param_1);
    uVar9 = (ulong)uVar4;
    if (*(char *)(extraout_RDX + (ulong)(uVar4 & 0xff)) == '\0') break;
    param_1 = param_1 + 1;
  }
  uVar8 = 0;
  puVar11 = &DAT_0011e440;
  uVar7 = 0xc;
  do {
    uVar12 = uVar8 + uVar7 >> 1;
    pcVar6 = (&PTR_DAT_0011e060)[uVar12 * 2];
    if (*pcVar6 == '\0') {
LAB_00105630:
      if (param_2 != (undefined8 *)0x0) {
        *param_2 = param_1;
      }
      return (&DAT_0011e068)[uVar12 * 4];
    }
    uVar5 = uVar9 & 0xffffffff;
    pcVar10 = param_1;
    while( true ) {
      bVar2 = FUN_001053e0((int)(char)puVar11[uVar5 & 0xff]);
      bVar3 = FUN_001053e0((int)extraout_DL);
      if (bVar2 < bVar3) break;
      if (bVar3 < bVar2) {
        uVar8 = uVar12 + 1;
        uVar12 = uVar7;
        break;
      }
      pcVar1 = pcVar6 + 1;
      pcVar6 = pcVar6 + 1;
      param_1 = param_1 + 1;
      if (*pcVar1 == '\0') goto LAB_00105630;
      uVar5 = FUN_001053e0((int)*param_1);
    }
    uVar7 = uVar12;
    param_1 = pcVar10;
    if (uVar12 <= uVar8) {
      return 0;
    }
  } while( true );
}




// Function: key_numeric @ 0x5660

undefined8 key_numeric(long param_1)

{
  ulong uVar1;
  
  uVar1 = *(ulong *)(param_1 + 0x30) & 0xffff00ff0000;
  return CONCAT71((int7)(uVar1 >> 8),uVar1 != 0);
}




// Function: default_key_compare @ 0x5680

ulong default_key_compare(long param_1)

{
  ulong uVar1;
  
  uVar1 = 0;
  if ((*(long *)(param_1 + 0x20) == 0) && (*(long *)(param_1 + 0x28) == 0)) {
    if (*(char *)(param_1 + 0x30) != '\0') {
      return 0;
    }
    uVar1 = 0;
    if (*(char *)(param_1 + 0x31) == '\0') {
      uVar1 = FUN_00105660();
      if ((char)uVar1 != '\0') {
        return 0;
      }
      if ((*(char *)(param_1 + 0x36) == '\0') && (*(char *)(param_1 + 0x38) == '\0')) {
        return (ulong)(*(byte *)(param_1 + 0x33) ^ 1);
      }
    }
  }
  return uVar1;
}




// Function: key_to_opts @ 0x56d0

void key_to_opts(long param_1,undefined1 *param_2)

{
  if (*(short *)(param_1 + 0x30) != 0) {
    *param_2 = 0x62;
    param_2 = param_2 + 1;
  }
  if (*(undefined1 **)(param_1 + 0x20) == &DAT_0011e540) {
    *param_2 = 100;
    param_2 = param_2 + 1;
  }
  if (*(long *)(param_1 + 0x28) != 0) {
    *param_2 = 0x66;
    param_2 = param_2 + 1;
  }
  if (*(char *)(param_1 + 0x34) != '\0') {
    *param_2 = 0x67;
    param_2 = param_2 + 1;
  }
  if (*(char *)(param_1 + 0x35) != '\0') {
    *param_2 = 0x68;
    param_2 = param_2 + 1;
  }
  if (*(undefined1 **)(param_1 + 0x20) == &DAT_0011e640) {
    *param_2 = 0x69;
    param_2 = param_2 + 1;
  }
  if (*(char *)(param_1 + 0x36) != '\0') {
    *param_2 = 0x4d;
    param_2 = param_2 + 1;
  }
  if (*(char *)(param_1 + 0x32) != '\0') {
    *param_2 = 0x6e;
    param_2 = param_2 + 1;
  }
  if (*(char *)(param_1 + 0x33) != '\0') {
    *param_2 = 0x52;
    param_2 = param_2 + 1;
  }
  if (*(char *)(param_1 + 0x37) != '\0') {
    *param_2 = 0x72;
    param_2 = param_2 + 1;
  }
  if (*(char *)(param_1 + 0x38) != '\0') {
    *param_2 = 0x56;
    param_2 = param_2 + 1;
  }
  *param_2 = 0;
  return;
}




// Function: compare_nodes @ 0x5780

bool compare_nodes(long param_1,long param_2)

{
  if (*(uint *)(param_1 + 0x50) != *(uint *)(param_2 + 0x50)) {
    return *(uint *)(param_1 + 0x50) < *(uint *)(param_2 + 0x50);
  }
  return (ulong)(*(long *)(param_1 + 0x30) + *(long *)(param_1 + 0x28)) <
         (ulong)(*(long *)(param_2 + 0x30) + *(long *)(param_2 + 0x28));
}




// Function: set_ordering @ 0x57c0

char * set_ordering(char *param_1,long param_2,uint param_3)

{
  char cVar1;
  
  cVar1 = *param_1;
  if (cVar1 != '\0') {
    do {
      switch(cVar1) {
      case 'M':
        *(undefined1 *)(param_2 + 0x36) = 1;
        break;
      default:
        goto switchD_00105801_caseD_4e;
      case 'R':
        *(undefined1 *)(param_2 + 0x33) = 1;
        break;
      case 'V':
        *(undefined1 *)(param_2 + 0x38) = 1;
        break;
      case 'b':
        if ((param_3 & 0xfffffffd) == 0) {
          *(undefined1 *)(param_2 + 0x30) = 1;
        }
        if (param_3 - 1 < 2) {
          *(undefined1 *)(param_2 + 0x31) = 1;
        }
        break;
      case 'd':
        *(undefined1 **)(param_2 + 0x20) = &DAT_0011e540;
        break;
      case 'f':
        *(undefined1 **)(param_2 + 0x28) = &DAT_0011e440;
        break;
      case 'g':
        *(undefined1 *)(param_2 + 0x34) = 1;
        break;
      case 'h':
        *(undefined1 *)(param_2 + 0x35) = 1;
        break;
      case 'i':
        if (*(long *)(param_2 + 0x20) == 0) {
          *(undefined1 **)(param_2 + 0x20) = &DAT_0011e640;
        }
        break;
      case 'n':
        *(undefined1 *)(param_2 + 0x32) = 1;
        break;
      case 'r':
        *(undefined1 *)(param_2 + 0x37) = 1;
      }
      cVar1 = param_1[1];
      param_1 = param_1 + 1;
    } while (cVar1 != '\0');
  }
switchD_00105801_caseD_4e:
  return param_1;
}




// Function: struct_month_cmp @ 0x5930

void struct_month_cmp(undefined8 *param_1,undefined8 *param_2)

{
  strcmp((char *)*param_1,(char *)*param_2);
  return;
}




// Function: incompatible_options @ 0x5940

void incompatible_options(undefined8 param_1)

{
  undefined8 uVar1;
  
  uVar1 = dcgettext(0,"options \'-%s\' are incompatible",5);
                    /* WARNING: Subroutine does not return */
  error(2,0,uVar1,param_1);
}




// Function: check_ordering_compatibility @ 0x5970

void check_ordering_compatibility(void)

{
  long lVar1;
  undefined1 *puVar2;
  long in_FS_OFFSET;
  undefined1 auStack_38 [40];
  long local_10;
  
  puVar2 = auStack_38;
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  lVar1 = DAT_0011e410;
  do {
    if (lVar1 == 0) {
      if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
LAB_001059fc:
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    if (1 < (uint)*(byte *)(lVar1 + 0x32) + (uint)*(byte *)(lVar1 + 0x34) +
            (uint)*(byte *)(lVar1 + 0x35) + (uint)*(byte *)(lVar1 + 0x36) +
            (uint)(byte)(*(byte *)(lVar1 + 0x38) | *(byte *)(lVar1 + 0x33) |
                        *(long *)(lVar1 + 0x20) != 0)) {
      *(undefined1 *)(lVar1 + 0x37) = 0;
      *(undefined2 *)(lVar1 + 0x30) = 0;
      FUN_001056d0(lVar1,auStack_38);
      FUN_00105940(puVar2);
      goto LAB_001059fc;
    }
    lVar1 = *(long *)(lVar1 + 0x40);
  } while( true );
}




// Function: badfieldspec @ 0x5a10

void badfieldspec(undefined8 param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  
  uVar1 = FUN_00111430();
  uVar2 = dcgettext(0,param_2,5);
  uVar3 = dcgettext(0,"%s: invalid field specification %s",5);
                    /* WARNING: Subroutine does not return */
  error(2,0,uVar3,uVar2,uVar1);
}




// Function: cs_enter @ 0x5a60

void cs_enter(long param_1)

{
  int iVar1;
  
  iVar1 = pthread_sigmask(0,(__sigset_t *)&DAT_0011e380,(__sigset_t *)(param_1 + 8));
  *(bool *)param_1 = iVar1 == 0;
  return;
}




// Function: cs_leave @ 0x5a80

void cs_leave(char *param_1)

{
  if (*param_1 == '\0') {
    return;
  }
  pthread_sigmask(2,(__sigset_t *)(param_1 + 8),(__sigset_t *)0x0);
  return;
}




// Function: cleanup @ 0x5aa0

void cleanup(void)

{
  undefined8 *puVar1;
  
  for (puVar1 = DAT_0011e368; puVar1 != (undefined8 *)0x0; puVar1 = (undefined8 *)*puVar1) {
    unlink((char *)((long)puVar1 + 0xd));
  }
  DAT_0011e368 = (undefined8 *)0x0;
  return;
}




// Function: exit_cleanup @ 0x5ad0

void exit_cleanup(void)

{
  long in_FS_OFFSET;
  undefined1 auStack_98 [136];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0011e368 != 0) {
    FUN_00105a60(auStack_98);
    FUN_00105aa0();
    FUN_00105a80(auStack_98);
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    FUN_0010c5c0();
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sighandler @ 0x5b40

void sighandler(int param_1)

{
  FUN_00105aa0();
  signal(param_1,(__sighandler_t)0x0);
  raise(param_1);
  return;
}




// Function: inittables @ 0x5b80

undefined8 inittables(void)

{
  ushort uVar1;
  undefined1 uVar2;
  byte bVar3;
  int iVar4;
  ushort **ppuVar5;
  char *__s;
  size_t sVar6;
  undefined *puVar7;
  undefined8 extraout_RAX;
  long extraout_RDX;
  ulong uVar8;
  long lVar9;
  char *pcVar10;
  ushort *puVar11;
  long lVar12;
  undefined **ppuVar13;
  undefined8 uStack_38;
  
  uVar8 = 0;
  ppuVar5 = __ctype_b_loc();
  do {
    uVar2 = FUN_00105b60(uVar8 & 0xffffffff);
    (&DAT_0011e740)[uVar8] = uVar2;
    uVar1 = (*ppuVar5)[uVar8];
    (&DAT_0011e640)[uVar8] = ((byte)(uVar1 >> 0xe) ^ 1) & 1;
    bVar3 = 0;
    if ((uVar1 & 8) == 0) {
      bVar3 = FUN_00105b60(uVar8 & 0xffffffff);
      bVar3 = bVar3 ^ 1;
    }
    (&DAT_0011e540)[uVar8] = bVar3;
    iVar4 = toupper((int)uVar8);
    (&DAT_0011e440)[uVar8] = (char)iVar4;
    uVar8 = uVar8 + 1;
  } while (uVar8 != 0x100);
  if (DAT_0011e860 != '\0') {
    ppuVar13 = &PTR_DAT_0011e060;
    lVar12 = 1;
    do {
      __s = nl_langinfo((int)lVar12 + 0x2000d);
      sVar6 = strlen(__s);
      puVar7 = (undefined *)FUN_00112eb0(sVar6 + 1);
      *(int *)(ppuVar13 + 1) = (int)lVar12;
      *ppuVar13 = puVar7;
      lVar9 = 0;
      if (sVar6 != 0) {
        puVar11 = *ppuVar5;
        pcVar10 = __s + sVar6;
        lVar9 = 0;
        do {
          bVar3 = FUN_001053e0((int)*__s);
          if ((puVar11[bVar3] & 1) == 0) {
            puVar7[lVar9] = (&DAT_0011e440)[bVar3];
            lVar9 = lVar9 + 1;
          }
          __s = (char *)(extraout_RDX + 1);
        } while (pcVar10 != __s);
      }
      lVar12 = lVar12 + 1;
      puVar7[lVar9] = 0;
      ppuVar13 = ppuVar13 + 2;
    } while (lVar12 != 0xd);
    qsort(&PTR_DAT_0011e060,0xc,0x10,FUN_00105930);
    return extraout_RAX;
  }
  return uStack_38;
}




// Function: key_init @ 0x5cf0

undefined1 (*) [16] key_init(undefined1 (*param_1) [16])

{
  *(undefined8 *)param_1[4] = 0;
  param_1[1] = (undefined1  [16])0x0;
  *param_1 = (undefined1  [16])0x0;
  *(undefined8 *)param_1[1] = 0xffffffffffffffff;
  param_1[2] = (undefined1  [16])0x0;
  param_1[3] = (undefined1  [16])0x0;
  return param_1;
}




// Function: parse_field_count @ 0x5d20

undefined8 parse_field_count(undefined8 param_1,undefined8 *param_2,long param_3)

{
  undefined4 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  undefined8 local_30;
  undefined8 local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_00113650(param_1,&local_30,10,&local_28,"");
  switch(uVar1) {
  case 0:
  case 2:
    *param_2 = local_28;
    break;
  case 1:
  case 3:
    *param_2 = 0xffffffffffffffff;
    break;
  case 4:
    if (param_3 != 0) {
      uVar2 = FUN_00111430(param_1);
      uVar3 = dcgettext(0,param_3,5);
      uVar4 = dcgettext(0,"%s: invalid count at start of %s",5);
                    /* WARNING: Subroutine does not return */
      error(2,0,uVar4,uVar3,uVar2);
    }
    local_30 = 0;
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_30;
}




// Function: sort_die @ 0x5e10

void sort_die(undefined8 param_1,long param_2)

{
  undefined8 uVar1;
  int *piVar2;
  
  if (param_2 == 0) {
    param_2 = dcgettext(0,"standard output",5);
  }
  uVar1 = FUN_00111270(0,3,param_2);
  piVar2 = __errno_location();
                    /* WARNING: Subroutine does not return */
  error(2,*piVar2,"%s: %s",param_1,uVar1);
}




// Function: get_outstatus @ 0x5e70

undefined * get_outstatus(void)

{
  int iVar1;
  int *piVar2;
  undefined *puVar3;
  
  if (DAT_0011e2f0 != 0) {
    puVar3 = (undefined *)0x0;
    if (DAT_0011e2f0 < 0) {
      puVar3 = &DAT_0011e260;
    }
    return puVar3;
  }
  iVar1 = fstat(1,(stat *)&DAT_0011e260);
  if (iVar1 == 0) {
    DAT_0011e2f0 = -1;
  }
  else {
    piVar2 = __errno_location();
    DAT_0011e2f0 = *piVar2;
    if (-1 < DAT_0011e2f0) {
      return (undefined *)0x0;
    }
  }
  return &DAT_0011e260;
}




// Function: stream_open @ 0x5ee0

FILE * stream_open(char *param_1,char *param_2)

{
  int iVar1;
  int *piVar2;
  long lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  FILE *pFVar6;
  
  if (*param_2 == 'r') {
    iVar1 = strcmp(param_1,"-");
    if (iVar1 == 0) {
      DAT_0011e418 = 1;
      pFVar6 = stdin;
    }
    else {
      pFVar6 = (FILE *)0x0;
      iVar1 = open(param_1,0x80000);
      if (-1 < iVar1) {
        pFVar6 = fdopen(iVar1,param_2);
      }
    }
    FUN_0010c8d0(pFVar6,2);
    return pFVar6;
  }
  if (*param_2 == 'w') {
    if (param_1 != (char *)0x0) {
      iVar1 = ftruncate(1,0);
      if (iVar1 != 0) {
        piVar2 = __errno_location();
        iVar1 = *piVar2;
        lVar3 = FUN_00105e70();
        if ((lVar3 == 0) || ((*(uint *)(lVar3 + 0x18) & 0xf000) == 0x8000)) {
          uVar4 = FUN_00111270(0,3,param_1);
          uVar5 = dcgettext(0,"%s: error truncating",5);
                    /* WARNING: Subroutine does not return */
          error(2,iVar1,uVar5,uVar4);
        }
      }
    }
    return stdout;
  }
                    /* WARNING: Subroutine does not return */
  __assert_fail("!\"unexpected mode passed to stream_open\"","src/sort.c",0x3d5,"stream_open");
}




// Function: xfopen @ 0x6010

void xfopen(undefined8 param_1)

{
  long lVar1;
  undefined8 uVar2;
  
  lVar1 = FUN_00105ee0();
  if (lVar1 != 0) {
    return;
  }
  uVar2 = dcgettext(0,"open failed",5);
                    /* WARNING: Subroutine does not return */
  FUN_00105e10(uVar2,param_1);
}




// Function: xfclose @ 0x6040

void xfclose(FILE *param_1,undefined8 param_2)

{
  undefined8 uVar1;
  int iVar2;
  char *pcVar3;
  
  iVar2 = fileno(param_1);
  if (iVar2 == 0) {
    clearerr_unlocked(param_1);
    return;
  }
  if (iVar2 == 1) {
    iVar2 = fflush_unlocked(param_1);
    if (iVar2 != 0) {
      pcVar3 = "fflush failed";
      goto LAB_00106098;
    }
  }
  else {
    iVar2 = FUN_0010c900(param_1);
    if (iVar2 != 0) {
      pcVar3 = "close failed";
LAB_00106098:
      uVar1 = dcgettext(0,pcVar3,5);
                    /* WARNING: Subroutine does not return */
      FUN_00105e10(uVar1,param_2);
    }
  }
  return;
}




// Function: insertkey @ 0x60c0

void insertkey(undefined8 param_1)

{
  long lVar1;
  long lVar2;
  long *plVar3;
  long lVar4;
  
  lVar2 = FUN_001131b0(param_1,0x48);
  plVar3 = &DAT_0011e410;
  lVar1 = DAT_0011e410;
  if (DAT_0011e410 != 0) {
    do {
      lVar4 = lVar1;
      lVar1 = *(long *)(lVar4 + 0x40);
    } while (lVar1 != 0);
    plVar3 = (long *)(lVar4 + 0x40);
  }
  *plVar3 = lVar2;
  *(undefined8 *)(lVar2 + 0x40) = 0;
  return;
}




// Function: random_md5_state_init @ 0x6110

void random_md5_state_init(char *param_1)

{
  int iVar1;
  long lVar2;
  undefined8 uVar3;
  char *pcVar4;
  long in_FS_OFFSET;
  undefined1 auStack_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  lVar2 = FUN_00111710(param_1,0x10);
  if (lVar2 == 0) {
    if (param_1 == (char *)0x0) {
      param_1 = "getrandom";
    }
    pcVar4 = "open failed";
  }
  else {
    FUN_00111830(lVar2,auStack_38,0x10);
    iVar1 = thunk_FUN_00111540(lVar2);
    if (iVar1 == 0) {
      MD5_Init((MD5_CTX *)&DAT_0011e300);
      FUN_0010c690(auStack_38,0x10,&DAT_0011e300);
      if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    pcVar4 = "close failed";
  }
  uVar3 = dcgettext(0,pcVar4,5);
                    /* WARNING: Subroutine does not return */
  FUN_00105e10(uVar3,param_1);
}




// Function: add_temp_dir @ 0x61e0

void add_temp_dir(undefined8 param_1)

{
  if (DAT_0011e428 == DAT_0011e420) {
    DAT_0011e430 = FUN_00112ff0(DAT_0011e430,&DAT_0011e420,8);
  }
  *(undefined8 *)(DAT_0011e430 + DAT_0011e428 * 8) = param_1;
  DAT_0011e428 = DAT_0011e428 + 1;
  return;
}




// Function: initbuf @ 0x6240

void initbuf(undefined8 *param_1,long param_2,ulong param_3)

{
  void *pvVar1;
  
  do {
    param_3 = (param_3 & 0xffffffffffffffe0) + 0x20;
    pvVar1 = malloc(param_3);
    *param_1 = pvVar1;
    if (pvVar1 != (void *)0x0) {
      param_1[5] = param_2;
      param_1[3] = param_3;
      param_1[2] = 0;
      param_1[4] = 0;
      param_1[1] = 0;
      *(undefined1 *)(param_1 + 6) = 0;
      return;
    }
    param_3 = param_3 >> 1;
  } while (param_2 + 1U < param_3);
                    /* WARNING: Subroutine does not return */
  FUN_00113270();
}




// Function: debug_width @ 0x62b0

long debug_width(char *param_1,char *param_2)

{
  int iVar1;
  long lVar2;
  char *pcVar3;
  
  iVar1 = FUN_0010e580(param_1,(long)param_2 - (long)param_1,0);
  lVar2 = (long)iVar1;
  if (param_1 < param_2) {
    do {
      pcVar3 = param_1 + 1;
      lVar2 = lVar2 + (ulong)(*param_1 == '\t');
      param_1 = pcVar3;
    } while (param_2 != pcVar3);
  }
  return lVar2;
}




// Function: mark_key @ 0x62f0

void mark_key(long param_1,long param_2)

{
  undefined8 uVar1;
  
  for (; param_1 != 0; param_1 = param_1 + -1) {
    putchar_unlocked(0x20);
  }
  if (param_2 != 0) {
    do {
      putchar_unlocked(0x5f);
      param_2 = param_2 + -1;
    } while (param_2 != 0);
    putchar_unlocked(10);
    return;
  }
  uVar1 = dcgettext(0,"^ no match for key\n",5);
  __printf_chk(1,uVar1);
  return;
}




// Function: nan_compare @ 0x6370

void nan_compare(char *param_1,char *param_2)

{
  long in_FS_OFFSET;
  undefined1 local_38 [16];
  undefined1 local_28 [24];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  strtold(param_1,(char **)0x0);
  strtold(param_2,(char **)0x0);
  memcmp(local_38,local_28,0x10);
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: general_numcompare @ 0x63d0

ulong general_numcompare(char *param_1,char *param_2)

{
  ulong uVar1;
  long in_FS_OFFSET;
  longdouble in_ST0;
  longdouble in_ST1;
  char *local_30;
  char *local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  strtold(param_1,&local_30);
  strtold(param_2,&local_28);
  if (local_30 == param_1) {
    uVar1 = (ulong)-(uint)(local_28 != param_2);
    goto LAB_00106448;
  }
  if (local_28 == param_2) {
LAB_00106494:
    uVar1 = 1;
  }
  else {
    if (in_ST1 <= in_ST0) {
      uVar1 = 1;
      if ((in_ST1 < in_ST0) || (uVar1 = 0, in_ST0 == in_ST1)) goto LAB_00106448;
      if (NAN(in_ST1)) {
        if (NAN(in_ST0)) {
          uVar1 = FUN_00106370(param_1,param_2);
          goto LAB_00106448;
        }
        goto LAB_00106494;
      }
    }
    uVar1 = 0xffffffff;
  }
LAB_00106448:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: numcompare @ 0x64d0

void numcompare(char *param_1,char *param_2)

{
  byte bVar1;
  long extraout_RDX;
  long extraout_RDX_00;
  
  while (bVar1 = FUN_001053e0((int)*param_1), *(char *)(extraout_RDX + (ulong)bVar1) != '\0') {
    param_1 = param_1 + 1;
  }
  while (bVar1 = FUN_001053e0((int)*param_2), *(char *)(extraout_RDX_00 + (ulong)bVar1) != '\0') {
    param_2 = param_2 + 1;
  }
  thunk_FUN_00112370(param_1);
  return;
}




// Function: human_numcompare @ 0x6530

void human_numcompare(char *param_1,char *param_2)

{
  byte bVar1;
  int iVar2;
  int iVar3;
  long extraout_RDX;
  long extraout_RDX_00;
  
  while (bVar1 = FUN_001053e0((int)*param_1), *(char *)(extraout_RDX + (ulong)bVar1) != '\0') {
    param_1 = param_1 + 1;
  }
  while (bVar1 = FUN_001053e0((int)*param_2), *(char *)(extraout_RDX_00 + (ulong)bVar1) != '\0') {
    param_2 = param_2 + 1;
  }
  iVar2 = FUN_001054f0(param_1);
  iVar3 = FUN_001054f0(param_2);
  if (iVar2 != iVar3) {
    return;
  }
  thunk_FUN_00112370(param_1,param_2,(int)DAT_0011e868,DAT_0011e864);
  return;
}




// Function: xstrxfrm @ 0x65c0

void xstrxfrm(char *param_1,char *param_2,size_t param_3)

{
  int *piVar1;
  undefined8 uVar2;
  
  piVar1 = __errno_location();
  *piVar1 = 0;
  strxfrm(param_1,param_2,param_3);
  if (*piVar1 == 0) {
    return;
  }
  uVar2 = dcgettext(0,"string transformation failed",5);
                    /* WARNING: Subroutine does not return */
  error(0,*piVar1,uVar2);
}




// Function: compare_random @ 0x6680

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int compare_random(char *param_1,ulong param_2,char *param_3,ulong param_4)

{
  char *pcVar1;
  char *pcVar2;
  int iVar3;
  size_t sVar4;
  char *__dest;
  long lVar5;
  ulong uVar6;
  ulong uVar7;
  char *pcVar8;
  ulong __size;
  ulong uVar9;
  long in_FS_OFFSET;
  char *local_1118;
  char *local_1108;
  ulong local_10f8;
  int local_10ec;
  undefined1 local_10c8 [16];
  undefined1 local_10b8 [16];
  undefined8 local_10a8;
  undefined8 uStack_10a0;
  undefined8 local_1098;
  undefined8 uStack_1090;
  undefined8 local_1088;
  undefined8 uStack_1080;
  undefined8 local_1078;
  undefined8 uStack_1070;
  undefined8 local_1068;
  undefined8 uStack_1060;
  undefined8 local_1058;
  undefined4 local_1050;
  undefined8 local_104c;
  undefined8 uStack_1044;
  undefined8 local_103c;
  undefined4 uStack_1034;
  undefined4 uStack_1030;
  undefined4 uStack_102c;
  undefined4 uStack_1028;
  undefined8 uStack_1024;
  undefined8 local_101c;
  undefined8 uStack_1014;
  undefined8 local_100c;
  undefined8 uStack_1004;
  undefined8 local_ffc;
  undefined4 local_ff4;
  char local_fe8 [4008];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_ff4 = DAT_0011e358;
  local_ffc = DAT_0011e350;
  local_1058 = DAT_0011e350;
  local_1050 = DAT_0011e358;
  local_104c = _DAT_0011e300;
  uStack_1044 = uRam000000000011e308;
  local_103c = _DAT_0011e310;
  uStack_1034 = (undefined4)uRam000000000011e318;
  uStack_1030 = (undefined4)((ulong)uRam000000000011e318 >> 0x20);
  uStack_102c = (undefined4)_DAT_0011e320;
  uStack_1028 = (undefined4)((ulong)_DAT_0011e320 >> 0x20);
  uStack_1024 = uRam000000000011e328;
  local_101c = _DAT_0011e330;
  uStack_1014 = uRam000000000011e338;
  local_100c = _DAT_0011e340;
  uStack_1004 = uRam000000000011e348;
  local_10a8 = _DAT_0011e300;
  uStack_10a0 = uRam000000000011e308;
  local_1098 = _DAT_0011e310;
  uStack_1090 = uRam000000000011e318;
  local_1088 = _DAT_0011e320;
  uStack_1080 = uRam000000000011e328;
  local_1078 = _DAT_0011e330;
  uStack_1070 = uRam000000000011e338;
  local_1068 = _DAT_0011e340;
  uStack_1060 = uRam000000000011e348;
  if (DAT_0011e861 == '\0') {
    FUN_0010c690(param_1,param_2,&local_10a8);
    FUN_0010c6c0(&local_10a8,local_10c8);
    FUN_0010c690(param_3,param_4,&local_104c);
    FUN_0010c6c0(&local_104c,local_10b8);
    local_10ec = memcmp(local_10c8,local_10b8,0x10);
    __dest = (char *)0x0;
    local_1118 = param_1;
  }
  else {
    pcVar1 = param_1 + param_2;
    __dest = (char *)0x0;
    __size = 4000;
    pcVar2 = param_3 + param_4;
    uVar7 = (param_2 + param_4) * 3 + 2;
    local_10ec = 0;
    local_1118 = local_fe8;
LAB_001067c8:
    if (__size < uVar7) goto LAB_00106899;
LAB_001067d3:
    uVar6 = __size;
    if (param_1 < pcVar1) goto LAB_001068d7;
LAB_001067de:
    if (param_3 < pcVar2) {
      param_2 = 0;
      pcVar8 = local_1118;
      uVar6 = __size;
      goto LAB_00106902;
    }
    local_10f8 = 0;
    param_2 = 0;
    local_1108 = local_1118;
LAB_001067fd:
    if (param_3 < pcVar2) {
LAB_00106808:
      do {
        sVar4 = strlen(param_3);
        param_3 = param_3 + sVar4 + 1;
        do {
          while( true ) {
            if ((pcVar1 <= param_1) && (pcVar2 <= param_3)) goto LAB_0010697c;
            FUN_0010c690(local_1118,param_2,&local_10a8);
            FUN_0010c690(local_1108,local_10f8,&local_104c);
            if (local_10ec != 0) goto LAB_001067c8;
            uVar6 = local_10f8;
            if (param_2 <= local_10f8) {
              uVar6 = param_2;
            }
            local_10ec = memcmp(local_1118,local_1108,uVar6);
            if (local_10ec != 0) goto LAB_001067c8;
            local_10ec = (uint)(param_2 >= local_10f8 && param_2 != local_10f8) -
                         (uint)(param_2 < local_10f8);
            if (uVar7 <= __size) goto LAB_001067d3;
LAB_00106899:
            __size = __size * 3 >> 1;
            if (__size < uVar7) {
              __size = uVar7;
            }
            free(__dest);
            __dest = malloc(__size);
            if (__dest == (char *)0x0) {
              __size = 4000;
              local_1118 = local_fe8;
              goto LAB_001067d3;
            }
            uVar6 = __size;
            local_1118 = __dest;
            if (pcVar1 <= param_1) goto LAB_001067de;
LAB_001068d7:
            lVar5 = FUN_001065c0(local_1118,param_1,uVar6);
            param_2 = lVar5 + 1;
            if (pcVar2 <= param_3) break;
            if (uVar6 < param_2) {
              __size = 0;
              pcVar8 = (char *)0x0;
            }
            else {
              __size = uVar6 - param_2;
              pcVar8 = local_1118 + param_2;
            }
LAB_00106902:
            lVar5 = FUN_001065c0(pcVar8,param_3,__size);
            local_10f8 = lVar5 + 1;
            __size = uVar6;
            uVar9 = local_10f8 + param_2;
            if (param_2 <= uVar6) goto LAB_00106b78;
LAB_00106920:
            __size = uVar9;
            if (uVar9 < 0x5555555555555555) {
              __size = uVar9 * 3 >> 1;
            }
            free(__dest);
            __dest = (char *)FUN_00112eb0(__size);
            local_1118 = __dest;
            if (param_1 < pcVar1) {
              strxfrm(__dest,param_1,param_2);
              local_1108 = __dest + param_2;
              if (param_3 < pcVar2) {
                strxfrm(local_1108,param_3,local_10f8);
                sVar4 = strlen(param_1);
                param_1 = param_1 + sVar4 + 1;
                goto LAB_00106808;
              }
            }
            else {
              local_1108 = __dest + param_2;
              if (param_3 < pcVar2) {
                strxfrm(local_1108,param_3,local_10f8);
                goto LAB_00106808;
              }
            }
            local_1108 = __dest + param_2;
            if (pcVar1 <= param_1) goto LAB_0010697c;
            sVar4 = strlen(param_1);
            param_1 = param_1 + sVar4 + 1;
          }
          local_10f8 = 0;
          __size = uVar6;
          uVar9 = param_2;
          if (uVar6 < param_2) goto LAB_00106920;
LAB_00106b78:
          if (__size < uVar9) goto LAB_00106920;
          local_1108 = local_1118 + param_2;
          if (pcVar1 <= param_1) goto LAB_001067fd;
          sVar4 = strlen(param_1);
          param_1 = param_1 + sVar4 + 1;
        } while (pcVar2 <= param_3);
      } while( true );
    }
LAB_0010697c:
    FUN_0010c690(local_1118,param_2,&local_10a8);
    FUN_0010c6c0(&local_10a8,local_10c8);
    FUN_0010c690(local_1108,local_10f8,&local_104c);
    FUN_0010c6c0(&local_104c,local_10b8);
    iVar3 = memcmp(local_10c8,local_10b8,0x10);
    param_3 = local_1108;
    param_4 = local_10f8;
    if (iVar3 != 0) goto LAB_001069e7;
  }
  iVar3 = local_10ec;
  if (local_10ec == 0) {
    uVar7 = param_4;
    if (param_2 <= param_4) {
      uVar7 = param_2;
    }
    iVar3 = memcmp(local_1118,param_3,uVar7);
    if (iVar3 == 0) {
      iVar3 = (uint)(param_4 < param_2) - (uint)(param_2 < param_4);
    }
  }
LAB_001069e7:
  local_10ec = iVar3;
  free(__dest);
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_10ec;
}




// Function: check_inputs @ 0x6c00

void check_inputs(undefined8 *param_1,long param_2)

{
  char *__s1;
  undefined8 uVar1;
  undefined8 uVar2;
  int iVar3;
  long lVar4;
  
  if (param_2 == 0) {
    return;
  }
  lVar4 = 0;
  do {
    __s1 = (char *)*param_1;
    iVar3 = strcmp(__s1,"-");
    if (iVar3 != 0) {
      iVar3 = euidaccess(__s1,4);
      if (iVar3 != 0) {
        uVar1 = *param_1;
        uVar2 = dcgettext(0,"cannot read",5);
                    /* WARNING: Subroutine does not return */
        FUN_00105e10(uVar2,uVar1);
      }
    }
    lVar4 = lVar4 + 1;
    param_1 = param_1 + 1;
  } while (param_2 != lVar4);
  return;
}




// Function: move_fd @ 0x6c80

void move_fd(int param_1,int param_2)

{
  if (param_1 == param_2) {
    return;
  }
  dup2(param_1,param_2);
  close(param_1);
  return;
}




// Function: check_output @ 0x6ca0

void check_output(char *param_1)

{
  int iVar1;
  undefined8 uVar2;
  
  if (param_1 == (char *)0x0) {
    return;
  }
  iVar1 = open(param_1,0x80041,0x1b6);
  if (-1 < iVar1) {
    FUN_00106c80(iVar1,1);
    return;
  }
  uVar2 = dcgettext(0,"open failed",5);
                    /* WARNING: Subroutine does not return */
  FUN_00105e10(uVar2,param_1);
}




// Function: delete_proc @ 0x6cf0

bool delete_proc(undefined4 param_1)

{
  long lVar1;
  long in_FS_OFFSET;
  undefined1 auStack_28 [8];
  undefined4 local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_20 = param_1;
  lVar1 = FUN_0010dff0(DAT_0011e360,auStack_28);
  if (lVar1 != 0) {
    *(undefined1 *)(lVar1 + 0xc) = 2;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar1 != 0;
}




// Function: reap @ 0x6d50

__pid_t reap(int param_1)

{
  char cVar1;
  __pid_t _Var2;
  int *piVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  uint local_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  _Var2 = -1;
  if (param_1 != 0) {
    _Var2 = param_1;
  }
  _Var2 = waitpid(_Var2,(int *)&local_24,(uint)(param_1 == 0));
  if (_Var2 < 0) {
    uVar4 = FUN_00111170(4,DAT_0011e408);
    uVar5 = dcgettext(0,"waiting for %s [-d]",5);
    piVar3 = __errno_location();
                    /* WARNING: Subroutine does not return */
    error(2,*piVar3,uVar5,uVar4);
  }
  if ((_Var2 != 0) && ((0 < param_1 || (cVar1 = FUN_00106cf0(_Var2), cVar1 != '\0')))) {
    if ((local_24 >> 8 & 0xff) != 0 || (local_24 & 0x7f) != 0) {
      uVar4 = FUN_00111170(4,DAT_0011e408);
      uVar5 = dcgettext(0,"%s [-d] terminated abnormally",5);
                    /* WARNING: Subroutine does not return */
      error(2,0,uVar5,uVar4);
    }
    DAT_0011e35c = DAT_0011e35c + -1;
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return _Var2;
}




// Function: wait_proc @ 0x6e60

void wait_proc(undefined4 param_1)

{
  char cVar1;
  
  cVar1 = FUN_00106cf0();
  if (cVar1 == '\0') {
    return;
  }
  FUN_00106d50(param_1);
  return;
}




// Function: zaptemp @ 0x6e80

void zaptemp(char *param_1)

{
  int iVar1;
  long lVar2;
  long *plVar3;
  long *__ptr;
  int iVar4;
  int *piVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  long in_FS_OFFSET;
  undefined1 local_c8 [136];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  plVar3 = (long *)&DAT_0011e368;
  for (__ptr = DAT_0011e368; param_1 != (char *)((long)__ptr + 0xd); __ptr = (long *)*__ptr) {
    plVar3 = __ptr;
  }
  if (*(char *)((long)__ptr + 0xc) == '\x01') {
    FUN_00106e60((int)__ptr[1]);
  }
  lVar2 = *__ptr;
  FUN_00105a60(local_c8);
  iVar4 = unlink(param_1);
  piVar5 = __errno_location();
  iVar1 = *piVar5;
  *plVar3 = lVar2;
  FUN_00105a80(local_c8);
  if (iVar4 != 0) {
    uVar6 = FUN_00111270(0,3,param_1);
    uVar7 = dcgettext(0,"warning: cannot remove: %s",5);
                    /* WARNING: Subroutine does not return */
    error(0,iVar1,uVar7,uVar6);
  }
  if (lVar2 == 0) {
    PTR_DAT_0011e040 = (undefined *)plVar3;
  }
  free(__ptr);
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: reap_exited @ 0x6fb0

void reap_exited(void)

{
  int iVar1;
  
  if (DAT_0011e35c < 1) {
    return;
  }
  do {
    iVar1 = FUN_00106d50(0);
    if (iVar1 == 0) {
      return;
    }
  } while (0 < DAT_0011e35c);
  return;
}




// Function: reap_some @ 0x6ff0

void reap_some(void)

{
  FUN_00106d50(0xffffffff);
  FUN_00106fb0();
  return;
}




// Function: reap_all @ 0x7010

void reap_all(void)

{
  if (DAT_0011e35c < 1) {
    return;
  }
  do {
    FUN_00106d50(0xffffffff);
  } while (0 < DAT_0011e35c);
  return;
}




// Function: pipe_fork @ 0x7040

__pid_t pipe_fork(int *param_1,long param_2)

{
  undefined8 uVar1;
  int iVar2;
  __pid_t _Var3;
  int *piVar4;
  long in_FS_OFFSET;
  double local_d8;
  undefined1 local_c8 [136];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = FUN_0010ec90(param_1,0x80000);
  if (iVar2 < 0) {
    _Var3 = -1;
  }
  else {
    if (DAT_0011e024 + 1U < DAT_0011e35c) {
      FUN_00106ff0();
    }
    piVar4 = __errno_location();
    local_d8 = DAT_001170f0;
    do {
      param_2 = param_2 + -1;
      FUN_00105a60(local_c8);
      uVar1 = DAT_0011e368;
      DAT_0011e368 = 0;
      _Var3 = fork();
      iVar2 = *piVar4;
      if (_Var3 == 0) {
        FUN_00105a80();
        *piVar4 = iVar2;
        close(0);
        close(1);
        goto LAB_001070ff;
      }
      DAT_0011e368 = uVar1;
      FUN_00105a80(local_c8);
      *piVar4 = iVar2;
      if (-1 < _Var3) {
        DAT_0011e35c = DAT_0011e35c + 1;
        goto LAB_001070ff;
      }
      if (iVar2 != 0xb) goto LAB_0010717d;
      FUN_00113440(local_d8);
      local_d8 = local_d8 + local_d8;
      FUN_00106fb0();
    } while (param_2 != 0);
    iVar2 = *piVar4;
LAB_0010717d:
    close(*param_1);
    close(param_1[1]);
    *piVar4 = iVar2;
  }
LAB_001070ff:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return _Var3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: async_safe_die @ 0x71d0

void async_safe_die(int param_1,char *param_2)

{
  size_t sVar1;
  char *__s;
  long in_FS_OFFSET;
  undefined1 local_2c [12];
  undefined8 local_20;
  
  local_20 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  sVar1 = strlen(param_2);
  write(2,param_2,sVar1);
  if (param_1 != 0) {
    __s = (char *)FUN_0010e440(param_1,local_2c);
    write(2,": errno ",8);
    sVar1 = strlen(__s);
    write(2,__s,sVar1);
  }
  write(2,"\n",1);
                    /* WARNING: Subroutine does not return */
  _exit(2);
}




// Function: register_proc @ 0x7270

void register_proc(long param_1)

{
  long lVar1;
  
  if (DAT_0011e360 == 0) {
    DAT_0011e360 = FUN_0010d970(0x2f,0,FUN_00105400,FUN_00105420,0);
    if (DAT_0011e360 == 0) goto LAB_001072c4;
  }
  lVar1 = DAT_0011e360;
  *(undefined1 *)(param_1 + 0xc) = 1;
  lVar1 = FUN_0010df90(lVar1,param_1);
  if (lVar1 != 0) {
    return;
  }
LAB_001072c4:
                    /* WARNING: Subroutine does not return */
  FUN_00113270();
}




// Function: open_temp @ 0x72d0

FILE * open_temp(long param_1)

{
  int iVar1;
  int iVar2;
  FILE *pFVar3;
  int *piVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long in_FS_OFFSET;
  int local_28;
  int local_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (*(char *)(param_1 + 0xc) == '\x01') {
    FUN_00106e60(*(undefined4 *)(param_1 + 8));
  }
  iVar1 = open((char *)(param_1 + 0xd),0);
  if (iVar1 < 0) {
    pFVar3 = (FILE *)0x0;
  }
  else {
    iVar2 = FUN_00107040(&local_28,9);
    if (iVar2 == -1) {
      piVar4 = __errno_location();
      if (*piVar4 != 0x18) {
        uVar5 = FUN_00111170(4,DAT_0011e408);
        uVar6 = dcgettext(0,"couldn\'t create process for %s -d",5);
                    /* WARNING: Subroutine does not return */
        error(2,*piVar4,uVar6,uVar5);
      }
      pFVar3 = (FILE *)0x0;
      close(iVar1);
      *piVar4 = 0x18;
    }
    else {
      if (iVar2 == 0) {
        close(local_28);
        FUN_00106c80(iVar1,0);
        FUN_00106c80(local_24,1);
        execlp(DAT_0011e408,DAT_0011e408,&DAT_00116c01,0);
        piVar4 = __errno_location();
        FUN_001071d0(*piVar4,"couldn\'t execute compress program (with -d)");
        goto LAB_00107421;
      }
      *(int *)(param_1 + 8) = iVar2;
      FUN_00107270(param_1);
      close(iVar1);
      close(local_24);
      pFVar3 = fdopen(local_28,"r");
      if (pFVar3 == (FILE *)0x0) {
        piVar4 = __errno_location();
        iVar1 = *piVar4;
        close(local_28);
        *piVar4 = iVar1;
      }
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return pFVar3;
  }
LAB_00107421:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: open_input_files @ 0x7470

long open_input_files(undefined8 *param_1,long param_2,long *param_3)

{
  long lVar1;
  long lVar2;
  long lVar3;
  
  lVar1 = FUN_00112fd0(param_2,8);
  *param_3 = lVar1;
  if (param_2 == 0) {
    lVar3 = 0;
  }
  else {
    lVar3 = 0;
    do {
      if ((param_1[1] == 0) || (*(char *)(param_1[1] + 0xc) == '\0')) {
        lVar2 = FUN_00105ee0(*param_1,"r");
        *(long *)(lVar1 + lVar3 * 8) = lVar2;
      }
      else {
        lVar2 = FUN_001072d0();
        *(long *)(lVar1 + lVar3 * 8) = lVar2;
      }
      if (lVar2 == 0) {
        return lVar3;
      }
      lVar3 = lVar3 + 1;
      param_1 = param_1 + 2;
    } while (param_2 != lVar3);
  }
  return lVar3;
}




// Function: create_temp_file @ 0x7510

undefined8 * create_temp_file(int *param_1,char param_2)

{
  int iVar1;
  char *__s;
  int iVar2;
  size_t __n;
  undefined8 *__ptr;
  int *piVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 *puVar6;
  long in_FS_OFFSET;
  undefined1 local_c8 [136];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  __s = *(char **)(DAT_0011e430 + DAT_0011e248 * 8);
  __n = strlen(__s);
  __ptr = (undefined8 *)FUN_00112eb0(__n + 0x20 & 0xfffffffffffffff8);
  memcpy((void *)((long)__ptr + 0xd),__s,__n);
  *(undefined8 *)((long)__ptr + __n + 0xd) = s__sortXXXXXX_001152f0._0_8_;
  *(undefined4 *)((long)__ptr + __n + 0x15) = s__sortXXXXXX_001152f0._8_4_;
  *__ptr = 0;
  DAT_0011e248 = DAT_0011e248 + 1;
  if (DAT_0011e248 == DAT_0011e428) {
    DAT_0011e248 = 0;
  }
  FUN_00105a60(local_c8);
  iVar2 = FUN_00112280((void *)((long)__ptr + 0xd),0x80000);
  piVar3 = __errno_location();
  if (iVar2 < 0) {
    iVar1 = *piVar3;
    FUN_00105a80(local_c8);
    *piVar3 = iVar1;
    if ((param_2 != '\x01') || (iVar1 != 0x18)) {
      uVar4 = FUN_00111170(4,__s);
      uVar5 = dcgettext(0,"cannot create temporary file in %s",5);
                    /* WARNING: Subroutine does not return */
      error(2,*piVar3,uVar5,uVar4);
    }
    puVar6 = (undefined8 *)0x0;
    free(__ptr);
  }
  else {
    iVar1 = *piVar3;
    puVar6 = __ptr;
    *(undefined8 **)PTR_DAT_0011e040 = __ptr;
    PTR_DAT_0011e040 = (undefined *)puVar6;
    FUN_00105a80(local_c8);
    *piVar3 = iVar1;
    puVar6 = __ptr;
  }
  *param_1 = iVar2;
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return puVar6;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: maybe_create_temp @ 0x76b0

long maybe_create_temp(undefined8 *param_1,undefined1 param_2)

{
  int iVar1;
  long lVar2;
  FILE *pFVar3;
  undefined8 uVar4;
  int *piVar5;
  long in_FS_OFFSET;
  bool bVar6;
  int local_2c;
  int local_28;
  int local_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  lVar2 = FUN_00107510(&local_2c,param_2);
  if (lVar2 != 0) {
    bVar6 = DAT_0011e408 != (char *)0x0;
    *(undefined1 *)(lVar2 + 0xc) = 0;
    if (bVar6) {
      iVar1 = FUN_00107040(&local_28,4);
      *(int *)(lVar2 + 8) = iVar1;
      if (iVar1 < 1) {
        if (iVar1 == 0) {
          close(local_24);
          FUN_00106c80(local_2c,1);
          FUN_00106c80(local_28,0);
          execlp(DAT_0011e408,DAT_0011e408,0);
          piVar5 = __errno_location();
          FUN_001071d0(*piVar5,"couldn\'t execute compress program");
          goto LAB_001077cc;
        }
      }
      else {
        close(local_2c);
        close(local_28);
        local_2c = local_24;
        FUN_00107270(lVar2);
      }
    }
    pFVar3 = fdopen(local_2c,"w");
    *param_1 = pFVar3;
    if (pFVar3 == (FILE *)0x0) {
      uVar4 = dcgettext(0,"couldn\'t create temporary file",5);
                    /* WARNING: Subroutine does not return */
      FUN_00105e10(uVar4,lVar2 + 0xd);
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar2;
  }
LAB_001077cc:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: create_temp @ 0x77e0

void create_temp(undefined8 param_1)

{
  FUN_001076b0(param_1,0);
  return;
}




// Function: default_sort_size @ 0x77f0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* WARNING: Enum "__rlimit_resource": Some values do not have unique names */

ulong default_sort_size(void)

{
  int iVar1;
  ulong uVar2;
  ulong uVar3;
  long in_FS_OFFSET;
  double dVar4;
  double dVar5;
  double dVar6;
  rlimit local_38;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = getrlimit(RLIMIT_DATA,&local_38);
  uVar3 = 0xffffffffffffffff;
  if (iVar1 == 0) {
    uVar3 = local_38.rlim_cur;
  }
  iVar1 = getrlimit(RLIMIT_AS,&local_38);
  if ((iVar1 == 0) && (local_38.rlim_cur < uVar3)) {
    uVar3 = local_38.rlim_cur;
  }
  uVar3 = uVar3 >> 1;
  iVar1 = getrlimit(__RLIMIT_RSS,&local_38);
  if ((iVar1 == 0) &&
     (uVar2 = (local_38.rlim_cur & 0xfffffffffffffff0) - (local_38.rlim_cur >> 4), uVar2 < uVar3)) {
    uVar3 = uVar2;
  }
  dVar4 = (double)FUN_0010eb50();
  dVar5 = (double)FUN_0010ea60();
  dVar6 = dVar5 * _DAT_00117100;
  if (dVar4 <= DAT_001170f8 * dVar5) {
    dVar4 = DAT_001170f8 * dVar5;
  }
  if (dVar6 < (double)(long)uVar3) {
    if (dVar6 < DAT_00117108) {
      uVar3 = (ulong)dVar6;
    }
    else {
      uVar3 = (long)(dVar6 - DAT_00117108) ^ 0x8000000000000000;
    }
  }
  if (dVar4 < (double)uVar3) {
    if (dVar4 < DAT_00117108) {
      uVar3 = (ulong)dVar4;
    }
    else {
      uVar3 = (long)(dVar4 - DAT_00117108) ^ 0x8000000000000000;
    }
  }
  uVar2 = (ulong)DAT_0011e024 * 0x22;
  if ((ulong)DAT_0011e024 * 0x22 < uVar3) {
    uVar2 = uVar3;
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: init_node @ 0x7970

long * init_node(long *param_1,long *param_2,long param_3,ulong param_4,long param_5,char param_6
                   )

{
  long *plVar1;
  long lVar2;
  ulong uVar3;
  long lVar4;
  long *plVar5;
  long *plVar6;
  long lVar7;
  
  while( true ) {
    plVar5 = param_2;
    lVar7 = param_3 + param_5 * -0x20;
    if (param_6 == '\0') {
      uVar3 = (ulong)param_1[6] >> 1;
      lVar4 = param_1[6] - uVar3;
      plVar6 = param_1 + 3;
    }
    else {
      uVar3 = (ulong)param_1[5] >> 1;
      lVar4 = param_1[5] - uVar3;
      plVar6 = param_1 + 2;
    }
    param_3 = lVar7 + uVar3 * -0x20;
    plVar5[7] = (long)param_1;
    lVar2 = param_1[10];
    plVar1 = plVar5 + 0x10;
    plVar5[4] = (long)plVar6;
    plVar5[2] = lVar7;
    *plVar5 = lVar7;
    plVar5[3] = param_3;
    plVar5[1] = param_3;
    plVar5[5] = uVar3;
    plVar5[6] = lVar4;
    *(int *)(plVar5 + 10) = (int)lVar2 + 1;
    *(undefined1 *)((long)plVar5 + 0x54) = 0;
    pthread_mutex_init((pthread_mutex_t *)(plVar5 + 0xb),(pthread_mutexattr_t *)0x0);
    if (param_4 < 2) break;
    plVar5[8] = (long)plVar1;
    uVar3 = param_4 >> 1;
    param_4 = param_4 - uVar3;
    param_2 = (long *)init_node(plVar5,plVar1,lVar7,uVar3,param_5);
    plVar5[9] = (long)param_2;
    param_6 = '\0';
    param_1 = plVar5;
  }
  plVar5[8] = 0;
  plVar5[9] = 0;
  return plVar1;
}




// Function: merge_tree_init @ 0x7aa0

undefined8 * merge_tree_init(long param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_00112eb0(param_1 << 8);
  puVar1[6] = param_2;
  puVar1[5] = param_2;
  puVar1[3] = 0;
  puVar1[2] = 0;
  puVar1[1] = 0;
  *puVar1 = 0;
  puVar1[4] = 0;
  puVar1[7] = 0;
  *(undefined4 *)(puVar1 + 10) = 0;
  *(undefined1 *)((long)puVar1 + 0x54) = 0;
  pthread_mutex_init((pthread_mutex_t *)(puVar1 + 0xb),(pthread_mutexattr_t *)0x0);
  FUN_00107970(puVar1,puVar1 + 0x10,param_3,param_1,param_2,0);
  return puVar1;
}




// Function: queue_init @ 0x7b40

void queue_init(undefined8 *param_1)

{
  undefined8 uVar1;
  
  uVar1 = FUN_0010e300(FUN_00105780);
  *param_1 = uVar1;
  pthread_mutex_init((pthread_mutex_t *)(param_1 + 1),(pthread_mutexattr_t *)0x0);
  pthread_cond_init((pthread_cond_t *)(param_1 + 6),(pthread_condattr_t *)0x0);
  return;
}




// Function: lock_node @ 0x7b70

void lock_node(long param_1)

{
  pthread_mutex_lock((pthread_mutex_t *)(param_1 + 0x58));
  return;
}




// Function: queue_insert @ 0x7b80

void queue_insert(undefined8 *param_1,long param_2)

{
  pthread_mutex_lock((pthread_mutex_t *)(param_1 + 1));
  FUN_0010e390(*param_1,param_2);
  *(undefined1 *)(param_2 + 0x54) = 1;
  pthread_cond_signal((pthread_cond_t *)(param_1 + 6));
  pthread_mutex_unlock((pthread_mutex_t *)(param_1 + 1));
  return;
}




// Function: queue_check_insert @ 0x7bc0

void queue_check_insert(undefined8 param_1,long *param_2)

{
  long lVar1;
  
  if (*(char *)((long)param_2 + 0x54) == '\0') {
    if (*param_2 == param_2[2]) {
      if (param_2[1] == param_2[3]) {
        return;
      }
      lVar1 = param_2[5];
    }
    else {
      if (param_2[1] != param_2[3]) goto LAB_00107bdc;
      lVar1 = param_2[6];
    }
    if (lVar1 == 0) {
LAB_00107bdc:
      FUN_00107b80();
      return;
    }
  }
  return;
}




// Function: unlock_node @ 0x7c00

void unlock_node(long param_1)

{
  pthread_mutex_unlock((pthread_mutex_t *)(param_1 + 0x58));
  return;
}




// Function: queue_check_insert_parent @ 0x7c10

void queue_check_insert_parent(undefined8 param_1,long param_2)

{
  if (1 < *(uint *)(param_2 + 0x50)) {
    FUN_00107b70(*(undefined8 *)(param_2 + 0x38));
    FUN_00107bc0(param_1,*(undefined8 *)(param_2 + 0x38));
    FUN_00107c00(*(undefined8 *)(param_2 + 0x38));
    return;
  }
  if (*(long *)(param_2 + 0x30) + *(long *)(param_2 + 0x28) != 0) {
    return;
  }
  FUN_00107b80(param_1,*(undefined8 *)(param_2 + 0x38));
  return;
}




// Function: queue_pop @ 0x7c70

long queue_pop(undefined8 *param_1)

{
  pthread_mutex_t *__mutex;
  long lVar1;
  
  __mutex = (pthread_mutex_t *)(param_1 + 1);
  pthread_mutex_lock(__mutex);
  while( true ) {
    lVar1 = FUN_0010e400(*param_1);
    if (lVar1 != 0) break;
    pthread_cond_wait((pthread_cond_t *)(param_1 + 6),__mutex);
  }
  pthread_mutex_unlock(__mutex);
  FUN_00107b70(lVar1);
  *(undefined1 *)(lVar1 + 0x54) = 0;
  return lVar1;
}




// Function: merge_tree_destroy @ 0x7cd0

void merge_tree_destroy(ulong param_1,void *param_2)

{
  pthread_mutex_t *__mutex;
  pthread_mutex_t *ppVar1;
  
  if ((param_1 & 0x7fffffffffffffff) != 0) {
    __mutex = (pthread_mutex_t *)((long)param_2 + 0x58);
    do {
      ppVar1 = (pthread_mutex_t *)((long)__mutex + 0x80);
      pthread_mutex_destroy(__mutex);
      __mutex = ppVar1;
    } while (ppVar1 != (pthread_mutex_t *)(param_1 * 0x100 + (long)param_2 + 0x58));
  }
  free(param_2);
  return;
}




// Function: queue_destroy @ 0x7d10

void queue_destroy(undefined8 *param_1)

{
  FUN_0010e370(*param_1);
  pthread_cond_destroy((pthread_cond_t *)(param_1 + 6));
  pthread_mutex_destroy((pthread_mutex_t *)(param_1 + 1));
  return;
}




// Function: key_warnings @ 0x7d30

/* WARNING: Removing unreachable block (ram,0x00108261) */
/* WARNING: Removing unreachable block (ram,0x00108268) */
/* WARNING: Removing unreachable block (ram,0x00108271) */
/* WARNING: Removing unreachable block (ram,0x00108580) */
/* WARNING: Removing unreachable block (ram,0x00108591) */
/* WARNING: Removing unreachable block (ram,0x001081ab) */
/* WARNING: Removing unreachable block (ram,0x00108431) */
/* WARNING: Removing unreachable block (ram,0x00108439) */
/* WARNING: Removing unreachable block (ram,0x0010843e) */
/* WARNING: Removing unreachable block (ram,0x0010844d) */
/* WARNING: Removing unreachable block (ram,0x00108609) */
/* WARNING: Removing unreachable block (ram,0x00108456) */
/* WARNING: Removing unreachable block (ram,0x0010845b) */
/* WARNING: Removing unreachable block (ram,0x001081c2) */
/* WARNING: Removing unreachable block (ram,0x001086af) */
/* WARNING: Removing unreachable block (ram,0x001086b4) */
/* WARNING: Removing unreachable block (ram,0x001086c4) */
/* WARNING: Removing unreachable block (ram,0x001081ce) */
/* WARNING: Removing unreachable block (ram,0x00108667) */
/* WARNING: Removing unreachable block (ram,0x001081ee) */
/* WARNING: Removing unreachable block (ram,0x001081f3) */
/* WARNING: Removing unreachable block (ram,0x00108217) */
/* WARNING: Removing unreachable block (ram,0x00108466) */
/* WARNING: Removing unreachable block (ram,0x0010846d) */
/* WARNING: Removing unreachable block (ram,0x00108478) */
/* WARNING: Removing unreachable block (ram,0x001086c9) */
/* WARNING: Removing unreachable block (ram,0x0010862f) */

void key_warnings(undefined8 *param_1,char param_2)

{
  ulong uVar1;
  undefined7 uVar2;
  uint7 uVar3;
  undefined2 uVar4;
  undefined3 uVar5;
  char cVar6;
  char cVar7;
  undefined8 uVar8;
  undefined2 *puVar9;
  undefined1 *puVar10;
  char *pcVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  ulong uVar14;
  size_t sVar15;
  char extraout_DL;
  long lVar16;
  byte bVar17;
  byte bVar18;
  ulong *puVar19;
  byte bVar20;
  byte bVar21;
  long in_FS_OFFSET;
  byte local_14a;
  byte local_149;
  long local_148;
  byte local_120;
  byte local_11f;
  byte local_11e;
  char local_11c;
  char local_11b;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  ulong local_f8;
  ulong uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined1 local_c8 [32];
  undefined1 local_a8;
  undefined1 local_a7 [47];
  undefined4 local_78;
  long local_40;
  
  local_118 = *param_1;
  uStack_110 = param_1[1];
  local_108 = param_1[2];
  uStack_100 = param_1[3];
  local_f8 = param_1[4];
  uStack_f0 = param_1[5];
  uVar14 = param_1[6];
  uStack_e0 = param_1[7];
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_d8 = param_1[8];
  local_e8 = uVar14;
  if (DAT_0011e410 != (ulong *)0x0) {
    local_e8._1_1_ = (byte)(uVar14 >> 8);
    local_e8._0_1_ = (byte)uVar14;
    local_14a = local_e8._1_1_;
    local_e8._6_1_ = (byte)(uVar14 >> 0x30);
    local_e8._4_1_ = (byte)(uVar14 >> 0x20);
    local_11b = '\0';
    local_149 = local_e8._6_1_;
    local_e8._2_1_ = (byte)(uVar14 >> 0x10);
    local_11c = '\0';
    local_e8._3_1_ = (byte)(uVar14 >> 0x18);
    local_120 = local_e8._2_1_;
    local_e8._5_1_ = (byte)(uVar14 >> 0x28);
    local_148 = 1;
    local_11f = local_e8._5_1_;
    local_e8._7_1_ = (byte)(uVar14 >> 0x38);
    local_11e = local_e8._7_1_;
    puVar19 = DAT_0011e410;
    bVar20 = (byte)local_e8;
    bVar17 = (byte)uStack_e0;
    bVar21 = local_e8._3_1_;
    bVar18 = local_e8._4_1_;
    do {
      local_e8 = uVar14;
      cVar6 = FUN_00105660(puVar19);
      cVar7 = local_11c;
      if ((cVar6 != '\0') && (cVar7 = cVar6, *(char *)((long)puVar19 + 0x34) != '\0')) {
        cVar7 = local_11c;
        local_11b = *(char *)((long)puVar19 + 0x34);
      }
      local_11c = cVar7;
      if (*(char *)((long)puVar19 + 0x39) != '\0') {
        uVar14 = *puVar19;
        uVar1 = puVar19[2];
        lVar16 = uVar14 + 1;
        if (uVar14 == 0xffffffffffffffff) {
          lVar16 = 1;
          uVar14 = 0;
        }
        uVar8 = FUN_0010e520(uVar14,local_c8);
        local_a8 = 0x2b;
        puVar9 = (undefined2 *)__stpcpy_chk(local_a7,uVar8,0x2d);
        uVar8 = FUN_0010e520(lVar16,local_c8);
        local_78 = 0x206b2d;
        puVar10 = (undefined1 *)__stpcpy_chk((long)&local_78 + 3,uVar8,0x2c);
        if (puVar19[2] != 0xffffffffffffffff) {
          pcVar11 = (char *)FUN_0010e520(uVar1 + 1,local_c8);
          *puVar9 = 0x2d20;
          strcpy((char *)(puVar9 + 1),pcVar11);
          pcVar11 = (char *)FUN_0010e520((ulong)(puVar19[3] == 0xffffffffffffffff) + uVar1 + 1,
                                         local_c8);
          *puVar10 = 0x2c;
          strcpy(puVar10 + 1,pcVar11);
        }
        uVar8 = FUN_00111420(1,&local_78);
        uVar12 = FUN_00111420(0,&local_a8);
        uVar13 = dcgettext(0,"obsolescent key %s used; consider %s instead",5);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar13,uVar12,uVar8);
      }
      if ((*puVar19 != 0xffffffffffffffff) && (puVar19[2] < *puVar19)) {
        uVar8 = dcgettext(0,"key %lu has zero width and will be ignored",5);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar8,local_148);
      }
      cVar6 = FUN_00105660(puVar19);
      cVar7 = '\x01';
      if (cVar6 == '\0') {
        cVar7 = *(char *)((long)puVar19 + 0x36);
      }
      if ((puVar19[2] == 0) && (puVar19[3] != 0)) {
        if (extraout_DL != '\0') {
LAB_00107e17:
          if (param_2 == '\0') goto LAB_00108108;
        }
      }
      else {
        if (extraout_DL != '\0') goto LAB_00107e17;
        if ((DAT_0011e028 == 0x80) &&
           ((((char)puVar19[6] == '\0' && ((cVar7 == '\0' || (puVar19[1] != 0)))) ||
            ((*(char *)((long)puVar19 + 0x31) == '\0' && (puVar19[3] != 0)))))) {
          uVar8 = dcgettext(0,
                            "leading blanks are significant in key %lu; consider also specifying \'b\'"
                            ,5);
                    /* WARNING: Subroutine does not return */
          error(0,0,uVar8,local_148);
        }
LAB_00108108:
        if (cVar6 != '\0') {
          uVar14 = *puVar19 + 1;
          if (uVar14 == 0) {
            uVar14 = 1;
          }
          if (uVar14 <= puVar19[2]) {
            uVar8 = dcgettext(0,"key %lu is numeric and spans multiple fields",5);
                    /* WARNING: Subroutine does not return */
            error(0,0,uVar8,local_148);
          }
        }
      }
      if ((local_f8 != 0) && (local_f8 == puVar19[4])) {
        local_f8 = 0;
      }
      if ((uStack_f0 != 0) && (uStack_f0 == puVar19[5])) {
        uStack_f0 = 0;
      }
      bVar18 = bVar18 & (*(byte *)((long)puVar19 + 0x34) ^ 1);
      local_e8 = (ulong)bVar18 << 0x20;
      bVar20 = bVar20 & ((byte)puVar19[6] ^ 1);
      local_14a = local_14a & (*(byte *)((long)puVar19 + 0x31) ^ 1);
      local_149 = local_149 & (*(byte *)((long)puVar19 + 0x36) ^ 1);
      local_e8._0_2_ = CONCAT11(local_14a,bVar20);
      uVar4 = (undefined2)local_e8;
      local_120 = local_120 & (*(byte *)((long)puVar19 + 0x32) ^ 1);
      local_e8._0_7_ = CONCAT16(local_149,(undefined6)local_e8);
      local_11f = local_11f & (*(byte *)((long)puVar19 + 0x35) ^ 1);
      uVar3 = (uint7)(undefined7)local_e8 >> 0x18;
      local_e8._0_3_ = CONCAT12(local_120,uVar4);
      uVar5 = (undefined3)local_e8;
      uVar2 = CONCAT43((int)uVar3,(undefined3)local_e8);
      bVar21 = bVar21 & (*(byte *)((long)puVar19 + 0x33) ^ 1);
      local_e8._0_6_ = CONCAT15(local_11f,(int5)uVar2);
      uVar2 = CONCAT16(local_149,(undefined6)local_e8);
      local_e8._0_4_ = CONCAT13(bVar21,uVar5);
      bVar17 = bVar17 & ((byte)puVar19[7] ^ 1);
      uStack_e0 = CONCAT71(uStack_e0._1_7_,bVar17);
      local_11e = local_11e & (*(byte *)((long)puVar19 + 0x37) ^ 1);
      local_e8 = CONCAT17(local_11e,CONCAT34((int3)((uint7)uVar2 >> 0x20),(undefined4)local_e8));
      puVar19 = (ulong *)puVar19[8];
      local_148 = local_148 + 1;
      uVar14 = local_e8;
    } while (puVar19 != (ulong *)0x0);
    if ((local_11c != '\0') || (local_11b != '\0')) {
      pcVar11 = "";
      local_78 = CONCAT31((int3)(local_78 >> 8),DAT_0011e868) & 0xffff00ff;
      uVar8 = FUN_00111430(&local_78);
      if (DAT_0011e868 != DAT_0011e028) {
        pcVar11 = (char *)dcgettext(0,"note ",5);
      }
      uVar12 = dcgettext(0,"%snumbers use %s as a decimal point in this locale",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar12,pcVar11,uVar8);
    }
  }
  cVar7 = FUN_00105680(&local_118);
  if (cVar7 == '\0') {
    if ((DAT_0011e41a == '\0') && (DAT_0011e419 == '\0')) {
      local_e8 = local_e8 & 0xffffffffffffff;
    }
    FUN_001056d0();
    sVar15 = strlen((char *)&local_78);
    FUN_001053f0(sVar15,"option \'-%s\' is ignored","options \'-%s\' are ignored");
    uVar8 = dcngettext(0);
                    /* WARNING: Subroutine does not return */
    error(0,0,uVar8,&local_78);
  }
  if (local_e8._7_1_ != '\0') {
    if (DAT_0011e41a == '\0') {
      if ((DAT_0011e419 != '\0') && (DAT_0011e410 != (ulong *)0x0)) {
        FUN_001056d0();
        sVar15 = strlen((char *)&local_78);
        FUN_001053f0(sVar15,"option \'-%s\' is ignored","options \'-%s\' are ignored");
        uVar8 = dcngettext(0);
                    /* WARNING: Subroutine does not return */
        error(0,0,uVar8,&local_78);
      }
      if ((DAT_0011e419 == '\0') && (DAT_0011e410 != (ulong *)0x0)) {
        uVar8 = dcgettext(0,"option \'-r\' only applies to last-resort comparison",5);
        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
          error(0,0,uVar8);
        }
        goto LAB_001086f4;
      }
    }
    else if (DAT_0011e410 != (ulong *)0x0) {
      FUN_001056d0();
      sVar15 = strlen((char *)&local_78);
      FUN_001053f0(sVar15,"option \'-%s\' is ignored","options \'-%s\' are ignored");
      uVar8 = dcngettext(0);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar8,&local_78);
    }
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
LAB_001086f4:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: debug_line @ 0x9400

undefined8 debug_line(undefined8 *param_1)

{
  undefined8 uVar1;
  long lVar2;
  undefined8 uStack_18;
  
  lVar2 = DAT_0011e410;
  do {
    FUN_00109230(*param_1,param_1[1],lVar2);
    if (lVar2 == 0) {
      return uStack_18;
    }
    lVar2 = *(long *)(lVar2 + 0x40);
  } while (lVar2 != 0);
  if ((DAT_0011e419 == '\0') && (DAT_0011e41a == '\0')) {
    uVar1 = FUN_00109230(*param_1,param_1[1],0);
    return uVar1;
  }
  return uStack_18;
}




// Function: write_line @ 0x9460

void write_line(undefined8 *param_1,FILE *param_2,long param_3)

{
  char *pcVar1;
  char cVar2;
  size_t __n;
  int iVar3;
  size_t sVar4;
  undefined8 uVar5;
  char *__ptr;
  
  __ptr = (char *)*param_1;
  __n = param_1[1];
  pcVar1 = __ptr + __n;
  if ((param_3 == 0) && (DAT_0011e400 != '\0')) {
    do {
      if (pcVar1 <= __ptr) {
        FUN_00109400(param_1);
        return;
      }
      cVar2 = *__ptr;
      __ptr = __ptr + 1;
      iVar3 = 0x3e;
      if ((cVar2 != '\t') && (iVar3 = 10, pcVar1 != __ptr)) {
        iVar3 = (int)cVar2;
      }
      iVar3 = fputc_unlocked(iVar3,param_2);
    } while (iVar3 != -1);
    uVar5 = dcgettext(0,"write failed",5);
                    /* WARNING: Subroutine does not return */
    FUN_00105e10(uVar5,0);
  }
  pcVar1[-1] = DAT_0011e038;
  sVar4 = fwrite_unlocked(__ptr,1,__n,param_2);
  if (sVar4 != __n) {
    uVar5 = dcgettext(0,"write failed",5);
                    /* WARNING: Subroutine does not return */
    FUN_00105e10(uVar5,param_3);
  }
  pcVar1[-1] = '\0';
  return;
}




// Function: keycompare @ 0x9550

int keycompare(undefined8 *param_1,long *param_2)

{
  long lVar1;
  char cVar2;
  byte bVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  char *pcVar7;
  char *pcVar8;
  char *__s1;
  char *__s2;
  char extraout_DL;
  char extraout_DL_00;
  int extraout_EDX;
  uint extraout_EDX_00;
  long extraout_RDX;
  ulong __n;
  long *plVar9;
  undefined1 *puVar10;
  long lVar11;
  long lVar12;
  char *pcVar13;
  ulong uVar14;
  ulong uVar15;
  long in_FS_OFFSET;
  bool bVar16;
  char local_1002;
  char local_1001;
  char *local_ff8;
  char local_fe8;
  char local_fe7 [4007];
  long local_40;
  
  __s1 = (char *)param_1[2];
  __s2 = (char *)param_2[2];
  pcVar7 = (char *)param_1[3];
  pcVar8 = (char *)param_2[3];
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  plVar9 = DAT_0011e410;
LAB_001095a0:
  lVar11 = plVar9[5];
  lVar1 = plVar9[4];
  if (pcVar7 <= __s1) {
    pcVar7 = __s1;
  }
  if (pcVar8 <= __s2) {
    pcVar8 = __s2;
  }
  uVar15 = (long)pcVar7 - (long)__s1;
  uVar14 = (long)pcVar8 - (long)__s2;
  if ((((DAT_0011e861 != '\0') || (cVar2 = FUN_00105660(plVar9), cVar2 != '\0')) ||
      ((plVar9[6] & 0xff0000ff000000U) != 0)) || ((char)plVar9[7] != '\0')) {
    if (lVar11 == 0 && lVar1 == 0) {
      local_1002 = *pcVar7;
      *pcVar7 = '\0';
      local_1001 = *pcVar8;
      *pcVar8 = '\0';
      pcVar8 = __s2;
    }
    else {
      local_ff8 = (char *)0x0;
      if (uVar15 + 2 + uVar14 < 0xfa1) {
        pcVar7 = local_fe7 + uVar15;
        pcVar8 = &local_fe8;
        if (uVar15 != 0) goto LAB_00109986;
LAB_00109abb:
        *pcVar8 = '\0';
        __s1 = pcVar8;
      }
      else {
        pcVar8 = (char *)FUN_00112eb0();
        pcVar7 = pcVar8 + uVar15 + 1;
        local_ff8 = pcVar8;
        if (uVar15 == 0) goto LAB_00109abb;
LAB_00109986:
        pcVar13 = __s1 + uVar15;
        uVar15 = 0;
        lVar12 = lVar11;
        do {
          if (lVar1 == 0) {
LAB_001099c8:
            bVar3 = FUN_001053e0();
            cVar2 = *(char *)(lVar12 + (ulong)bVar3);
LAB_001099ab:
            pcVar8[uVar15] = cVar2;
            uVar15 = uVar15 + 1;
          }
          else {
            bVar3 = FUN_001053e0((int)*__s1);
            if (*(char *)(lVar1 + (ulong)bVar3) == '\0') {
              cVar2 = extraout_DL;
              if (lVar12 != 0) goto LAB_001099c8;
              goto LAB_001099ab;
            }
          }
          __s1 = __s1 + 1;
        } while (pcVar13 != __s1);
        pcVar8[uVar15] = '\0';
        __s1 = pcVar8;
      }
      pcVar8 = pcVar7;
      if (uVar14 != 0) {
        pcVar7 = __s2 + uVar14;
        uVar14 = 0;
        lVar12 = lVar11;
        do {
          if (lVar1 == 0) {
LAB_00109a30:
            bVar3 = FUN_001053e0();
            cVar2 = *(char *)(lVar12 + (ulong)bVar3);
LAB_00109a13:
            pcVar8[uVar14] = cVar2;
            uVar14 = uVar14 + 1;
          }
          else {
            bVar3 = FUN_001053e0((int)*__s2);
            if (*(char *)(lVar1 + (ulong)bVar3) == '\0') {
              cVar2 = extraout_DL_00;
              if (lVar12 != 0) goto LAB_00109a30;
              goto LAB_00109a13;
            }
          }
          __s2 = __s2 + 1;
        } while (pcVar7 != __s2);
        pcVar7 = pcVar8 + uVar14;
      }
      *pcVar7 = '\0';
    }
    if (*(char *)((long)plVar9 + 0x32) == '\0') {
      if (*(char *)((long)plVar9 + 0x34) != '\0') {
        iVar6 = FUN_001063d0(__s1,pcVar8);
        goto LAB_0010977e;
      }
      if (*(char *)((long)plVar9 + 0x35) == '\0') {
        if (*(char *)((long)plVar9 + 0x36) == '\0') {
          if (*(char *)((long)plVar9 + 0x33) == '\0') {
            if ((char)plVar9[7] == '\0') {
              if (uVar15 == 0) {
                iVar6 = -(uint)(uVar14 != 0);
              }
              else {
                iVar6 = 1;
                if (uVar14 != 0) {
                  iVar6 = FUN_001133e0(__s1,uVar15 + 1,pcVar8,uVar14 + 1);
                }
              }
            }
            else {
              iVar6 = FUN_0010cd70(__s1,uVar15,pcVar8,uVar14);
            }
          }
          else {
            iVar6 = FUN_00106680(__s1,uVar15,pcVar8,uVar14);
          }
        }
        else {
          iVar6 = FUN_00105570(__s1,0);
          iVar5 = FUN_00105570(pcVar8);
          iVar6 = iVar6 - iVar5;
        }
        goto LAB_0010977e;
      }
      iVar6 = FUN_00106530(__s1);
      if (lVar11 == 0 && lVar1 == 0) goto LAB_0010978a;
    }
    else {
      iVar6 = FUN_001064d0(__s1);
LAB_0010977e:
      if (lVar11 == 0 && lVar1 == 0) {
LAB_0010978a:
        __s1[uVar15] = local_1002;
        pcVar8[uVar14] = local_1001;
        goto LAB_0010979c;
      }
    }
    free(local_ff8);
    if (iVar6 == 0) goto LAB_001097a4;
    goto LAB_00109880;
  }
  if (lVar1 != 0) {
    if (lVar11 == 0) {
      do {
        if ((pcVar7 <= __s1) ||
           (bVar3 = FUN_001053e0((int)*__s1), *(char *)(lVar1 + (ulong)bVar3) == '\0')) {
          if (pcVar8 <= __s2) goto LAB_00109c50;
          while (uVar15 = FUN_001053e0((int)*__s2), *(char *)(lVar1 + (uVar15 & 0xff)) != '\0') {
            __s2 = __s2 + 1;
            if (pcVar8 == __s2) goto LAB_00109c50;
          }
          if ((pcVar7 <= __s1) || (pcVar8 <= __s2)) goto LAB_00109c50;
          bVar3 = FUN_001053e0((int)*__s1);
          iVar6 = (uint)bVar3 - extraout_EDX;
          if (iVar6 != 0) goto LAB_00109880;
          __s2 = __s2 + 1;
        }
        __s1 = __s1 + 1;
      } while( true );
    }
    do {
      if ((pcVar7 <= __s1) ||
         (bVar3 = FUN_001053e0((int)*__s1), *(char *)(lVar1 + (ulong)bVar3) == '\0')) {
        if (pcVar8 <= __s2) goto LAB_00109bc0;
        while (bVar3 = FUN_001053e0((int)*__s2), *(char *)(lVar1 + (ulong)bVar3) != '\0') {
          __s2 = __s2 + 1;
          if (pcVar8 == __s2) goto LAB_00109bc0;
        }
        if ((pcVar7 <= __s1) || (pcVar8 <= __s2)) goto LAB_00109bc0;
        bVar3 = FUN_001053e0((int)*__s1);
        uVar4 = FUN_001053e0((int)*(char *)(lVar11 + (ulong)bVar3));
        bVar3 = FUN_001053e0((int)*(char *)(lVar11 + extraout_RDX));
        iVar6 = (uVar4 & 0xff) - (uint)bVar3;
        if (iVar6 != 0) {
          cVar2 = *(char *)((long)plVar9 + 0x37);
          goto joined_r0x00109be0;
        }
        __s2 = __s2 + 1;
      }
      __s1 = __s1 + 1;
    } while( true );
  }
  if (uVar15 == 0) {
    iVar6 = -(uint)(uVar14 != 0);
    goto LAB_0010979c;
  }
  if (uVar14 == 0) {
LAB_00109baf:
    iVar6 = 1;
    goto LAB_00109880;
  }
  if (lVar11 == 0) {
    __n = uVar14;
    if (uVar15 <= uVar14) {
      __n = uVar15;
    }
    iVar6 = memcmp(__s1,__s2,__n);
    if (iVar6 == 0) goto LAB_00109ba0;
    goto LAB_00109880;
  }
  while ((__s1 < pcVar7 && (__s2 < pcVar8))) {
    cVar2 = *__s1;
    __s1 = __s1 + 1;
    bVar3 = FUN_001053e0((int)cVar2);
    FUN_001053e0((int)*(char *)(lVar11 + (ulong)bVar3));
    bVar3 = FUN_001053e0((int)*__s2);
    uVar4 = FUN_001053e0((int)*(char *)(lVar11 + (ulong)bVar3));
    iVar6 = (extraout_EDX_00 & 0xff) - (uVar4 & 0xff);
    __s2 = __s2 + 1;
    if (iVar6 != 0) goto LAB_00109880;
  }
LAB_00109ba0:
  if (uVar15 < uVar14) {
    iVar6 = -1;
    goto LAB_00109880;
  }
  if (uVar15 != uVar14) goto LAB_00109baf;
  goto LAB_001097a4;
LAB_00109c50:
  bVar16 = __s1 < pcVar7;
  goto LAB_00109bc3;
LAB_00109bc0:
  bVar16 = __s1 < pcVar7;
LAB_00109bc3:
  iVar6 = (uint)bVar16 - (uint)(__s2 < pcVar8);
LAB_0010979c:
  if (iVar6 != 0) {
LAB_00109880:
    cVar2 = *(char *)((long)plVar9 + 0x37);
joined_r0x00109be0:
    if (cVar2 != '\0') {
      iVar6 = -iVar6;
    }
    goto LAB_0010988a;
  }
LAB_001097a4:
  plVar9 = (long *)plVar9[8];
  if (plVar9 == (long *)0x0) {
    iVar6 = 0;
LAB_0010988a:
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      return iVar6;
    }
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  __s1 = (char *)*param_1;
  lVar11 = param_1[1];
  if (plVar9[2] == -1) {
    pcVar7 = __s1 + lVar11 + -1;
    pcVar8 = (char *)(*param_2 + -1 + param_2[1]);
  }
  else {
    pcVar7 = (char *)FUN_00108f70(__s1,lVar11,plVar9);
    pcVar8 = (char *)FUN_00108f70(*param_2,param_2[1],plVar9);
  }
  if (*plVar9 == -1) {
    __s2 = (char *)*param_2;
    if ((char)plVar9[6] != '\0') {
      if (__s1 < pcVar7) {
        do {
          puVar10 = &DAT_0011e740;
          bVar3 = FUN_001053e0((int)*__s1);
          if (puVar10[bVar3] == '\0') break;
          __s1 = __s1 + 1;
        } while (pcVar7 != __s1);
      }
      if (__s2 < pcVar8) {
        do {
          puVar10 = &DAT_0011e740;
          bVar3 = FUN_001053e0((int)*__s2);
          if (puVar10[bVar3] == '\0') break;
          __s2 = __s2 + 1;
        } while (pcVar8 != __s2);
      }
    }
  }
  else {
    __s1 = (char *)FUN_001090d0(__s1,lVar11,plVar9);
    __s2 = (char *)FUN_001090d0(*param_2,param_2[1],plVar9);
  }
  goto LAB_001095a0;
}




// Function: compare @ 0x9c70

uint compare(undefined8 *param_1,undefined8 *param_2)

{
  ulong uVar1;
  long lVar2;
  long lVar3;
  uint uVar4;
  ulong __n;
  ulong uVar5;
  
  if (DAT_0011e410 != 0) {
    uVar4 = FUN_00109550();
    if (uVar4 != 0) {
      return uVar4;
    }
    if (DAT_0011e419 != '\0') {
      return 0;
    }
    if (DAT_0011e41a != '\0') {
      return 0;
    }
  }
  lVar2 = param_1[1];
  lVar3 = param_2[1];
  uVar1 = lVar3 - 1;
  uVar5 = lVar2 - 1;
  if (uVar5 == 0) {
    uVar4 = -(uint)(uVar1 != 0);
  }
  else {
    uVar4 = 1;
    if (uVar1 != 0) {
      if (DAT_0011e861 == '\0') {
        __n = uVar1;
        if (uVar5 <= uVar1) {
          __n = uVar5;
        }
        uVar4 = memcmp((void *)*param_1,(void *)*param_2,__n);
        if ((uVar4 == 0) && (uVar4 = 0xffffffff, uVar1 <= uVar5)) {
          uVar4 = (uint)(lVar2 != lVar3);
        }
      }
      else {
        uVar4 = FUN_001133e0((void *)*param_1,lVar2,(void *)*param_2,lVar3);
      }
    }
  }
  if (DAT_0011e41b != '\0') {
    uVar4 = -uVar4;
  }
  return uVar4;
}




// Function: mergelines @ 0x9d40

void mergelines(undefined8 *param_1,ulong param_2,long param_3)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  int iVar4;
  long lVar5;
  undefined8 *puVar6;
  undefined8 *puVar7;
  ulong uVar8;
  long lVar9;
  undefined8 *puVar10;
  long local_40;
  
  uVar8 = param_2 >> 1;
  local_40 = param_2 - uVar8;
  puVar6 = param_1 + uVar8 * -4 + -4;
  lVar5 = param_3 + -0x20;
  puVar10 = param_1 + uVar8 * -4;
  while( true ) {
    while( true ) {
      lVar9 = lVar5;
      puVar7 = puVar6;
      puVar6 = param_1 + -4;
      iVar4 = FUN_00109c70(lVar9,puVar7);
      if (iVar4 < 1) break;
      uVar1 = puVar10[-3];
      local_40 = local_40 + -1;
      *puVar6 = puVar10[-4];
      param_1[-3] = uVar1;
      uVar1 = puVar10[-1];
      param_1[-2] = puVar10[-2];
      param_1[-1] = uVar1;
      if (local_40 == 0) {
        lVar5 = -0x20;
        do {
          uVar1 = ((undefined8 *)(param_3 + lVar5))[1];
          puVar10 = (undefined8 *)(param_3 + 0x10 + lVar5);
          uVar2 = *puVar10;
          uVar3 = puVar10[1];
          *(undefined8 *)((long)puVar6 + lVar5) = *(undefined8 *)(param_3 + lVar5);
          ((undefined8 *)((long)puVar6 + lVar5))[1] = uVar1;
          puVar10 = (undefined8 *)((long)param_1 + lVar5 + -0x10);
          *puVar10 = uVar2;
          puVar10[1] = uVar3;
          lVar5 = lVar5 + -0x20;
          uVar8 = uVar8 - 1;
        } while (uVar8 != 0);
        return;
      }
      param_1 = puVar6;
      puVar6 = puVar7 + -4;
      lVar5 = lVar9;
      puVar10 = puVar7;
    }
    uVar1 = *(undefined8 *)(param_3 + -0x18);
    uVar2 = *(undefined8 *)(param_3 + -0x10);
    uVar3 = *(undefined8 *)(param_3 + -8);
    *puVar6 = *(undefined8 *)(param_3 + -0x20);
    param_1[-3] = uVar1;
    param_1[-2] = uVar2;
    param_1[-1] = uVar3;
    uVar8 = uVar8 - 1;
    if (uVar8 == 0) break;
    param_1 = puVar6;
    puVar6 = puVar7;
    lVar5 = lVar9 + -0x20;
    param_3 = lVar9;
  }
  return;
}




// Function: sequential_sort @ 0x9e20

void sequential_sort(long param_1,ulong param_2,long param_3,char param_4)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  int iVar9;
  undefined8 *puVar10;
  undefined8 *puVar11;
  long lVar12;
  ulong uVar13;
  
  if (param_2 != 2) {
    uVar13 = param_2 >> 1;
    lVar12 = param_1 + uVar13 * -0x20;
    if (param_4 == '\0') {
      sequential_sort(lVar12,param_2 - uVar13,param_3,0);
      lVar12 = param_3;
      if (param_2 < 4) {
        uVar1 = *(undefined8 *)(param_1 + -0x18);
        uVar2 = *(undefined8 *)(param_1 + -0x10);
        uVar3 = *(undefined8 *)(param_1 + -8);
        *(undefined8 *)(param_3 + -0x20) = *(undefined8 *)(param_1 + -0x20);
        *(undefined8 *)(param_3 + -0x18) = uVar1;
        *(undefined8 *)(param_3 + -0x10) = uVar2;
        *(undefined8 *)(param_3 + -8) = uVar3;
        param_3 = param_1;
      }
      else {
        sequential_sort(param_1,uVar13,param_3,1);
        param_3 = param_1;
      }
    }
    else {
      sequential_sort(lVar12,param_2 - uVar13,uVar13 * -0x20 + param_3,1);
      lVar12 = param_1;
      if (3 < param_2) {
        sequential_sort(param_1,uVar13,param_3,0);
      }
    }
    FUN_00109d40(param_3,param_2,lVar12);
    return;
  }
  iVar9 = FUN_00109c70(param_1 + -0x20,param_1 + -0x40);
  if (param_4 == '\0') {
    if (0 < iVar9) {
      uVar1 = *(undefined8 *)(param_1 + -0x20);
      uVar2 = *(undefined8 *)(param_1 + -0x18);
      uVar3 = *(undefined8 *)(param_1 + -0x10);
      uVar4 = *(undefined8 *)(param_1 + -8);
      uVar5 = *(undefined8 *)(param_1 + -0x40);
      uVar6 = *(undefined8 *)(param_1 + -0x38);
      uVar7 = *(undefined8 *)(param_1 + -0x30);
      uVar8 = *(undefined8 *)(param_1 + -0x28);
      *(undefined8 *)(param_3 + -0x20) = uVar1;
      *(undefined8 *)(param_3 + -0x18) = uVar2;
      *(undefined8 *)(param_3 + -0x10) = uVar3;
      *(undefined8 *)(param_3 + -8) = uVar4;
      *(undefined8 *)(param_1 + -0x20) = uVar5;
      *(undefined8 *)(param_1 + -0x18) = uVar6;
      *(undefined8 *)(param_1 + -0x10) = uVar7;
      *(undefined8 *)(param_1 + -8) = uVar8;
      *(undefined8 *)(param_1 + -0x40) = uVar1;
      *(undefined8 *)(param_1 + -0x38) = uVar2;
      *(undefined8 *)(param_1 + -0x30) = uVar3;
      *(undefined8 *)(param_1 + -0x28) = uVar4;
    }
    return;
  }
  puVar10 = (undefined8 *)((long)(int)((0 < iVar9) - 2) * 0x20 + param_1);
  puVar11 = (undefined8 *)((long)(int)~(uint)(0 < iVar9) * 0x20 + param_1);
  uVar1 = puVar10[1];
  uVar2 = puVar10[2];
  uVar3 = puVar10[3];
  uVar4 = *puVar11;
  uVar5 = puVar11[1];
  uVar6 = puVar11[2];
  uVar7 = puVar11[3];
  *(undefined8 *)(param_3 + -0x40) = *puVar10;
  *(undefined8 *)(param_3 + -0x38) = uVar1;
  *(undefined8 *)(param_3 + -0x20) = uVar4;
  *(undefined8 *)(param_3 + -0x18) = uVar5;
  *(undefined8 *)(param_3 + -0x10) = uVar6;
  *(undefined8 *)(param_3 + -8) = uVar7;
  *(undefined8 *)(param_3 + -0x30) = uVar2;
  *(undefined8 *)(param_3 + -0x28) = uVar3;
  return;
}




// Function: write_unique @ 0x9fb0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void write_unique(long *param_1,undefined8 param_2,undefined8 param_3)

{
  int iVar1;
  
  if (DAT_0011e419 != '\0') {
    if (DAT_0011e840 != 0) {
      iVar1 = FUN_00109c70(param_1,&DAT_0011e840);
      if (iVar1 == 0) {
        return;
      }
    }
    DAT_0011e840 = *param_1;
    lRam000000000011e848 = param_1[1];
    _DAT_0011e850 = param_1[2];
    lRam000000000011e858 = param_1[3];
  }
  FUN_00109460(param_1,param_2,param_3);
  return;
}




// Function: mergelines_node @ 0xa020

void mergelines_node(long *param_1,ulong param_2,undefined8 param_3,undefined8 param_4)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  int iVar5;
  undefined8 *puVar6;
  undefined8 *puVar7;
  undefined8 *puVar8;
  long lVar9;
  undefined8 *puVar10;
  long lVar11;
  long lVar12;
  long lVar13;
  long lVar14;
  undefined8 *puVar15;
  long lVar16;
  undefined8 *puVar17;
  undefined8 *puVar18;
  
  puVar1 = (undefined8 *)*param_1;
  puVar2 = (undefined8 *)param_1[1];
  puVar8 = (undefined8 *)param_1[2];
  lVar11 = (param_2 >> ((char)*(uint *)(param_1 + 10) * '\x02' + 2U & 0x3f)) + 1;
  puVar7 = puVar1;
  if (*(uint *)(param_1 + 10) < 2) {
    lVar13 = 0;
    lVar14 = 0;
    puVar10 = puVar2;
    lVar12 = lVar11;
    if (puVar1 != puVar8) {
LAB_0010a197:
      do {
        if ((undefined8 *)param_1[3] == puVar10) {
LAB_0010a217:
          lVar13 = (long)puVar2 - (long)puVar10 >> 5;
          if (param_1[6] != lVar13) {
            lVar9 = param_1[5];
            lVar14 = (long)puVar1 - (long)puVar7 >> 5;
            goto LAB_0010a237;
          }
          if (((undefined8 *)param_1[2] != puVar7) && (lVar12 != 0)) goto LAB_0010a286;
          lVar9 = param_1[5];
          lVar16 = lVar13;
          goto LAB_0010a13a;
        }
        lVar11 = lVar12 + -1;
        if (lVar12 == 0) {
          lVar12 = -1;
          goto LAB_0010a217;
        }
        iVar5 = FUN_00109c70(puVar7 + -4,puVar10 + -4);
        lVar12 = lVar11;
        if (iVar5 < 1) {
          lVar13 = *param_1;
          *param_1 = lVar13 + -0x20;
          FUN_00109fb0(lVar13 + -0x20,param_3,param_4);
          puVar7 = (undefined8 *)*param_1;
          puVar10 = (undefined8 *)param_1[1];
          if (puVar7 == (undefined8 *)param_1[2]) break;
          goto LAB_0010a197;
        }
        lVar13 = param_1[1];
        param_1[1] = lVar13 + -0x20;
        FUN_00109fb0(lVar13 + -0x20,param_3,param_4);
        puVar7 = (undefined8 *)*param_1;
        puVar10 = (undefined8 *)param_1[1];
      } while (puVar7 != (undefined8 *)param_1[2]);
      lVar14 = (long)puVar1 - (long)puVar7 >> 5;
      lVar13 = (long)puVar2 - (long)puVar10 >> 5;
    }
    lVar9 = param_1[5];
    lVar12 = lVar11;
    lVar16 = lVar13;
    if (param_1[6] != lVar13) {
LAB_0010a237:
      if (((lVar9 == lVar14) && (puVar10 != (undefined8 *)param_1[3])) && (lVar12 != 0)) {
        do {
          lVar12 = lVar12 + -1;
          param_1[1] = (long)(puVar10 + -4);
          FUN_00109fb0(puVar10 + -4,param_3,param_4);
          puVar10 = (undefined8 *)param_1[1];
          if (puVar10 == (undefined8 *)param_1[3]) break;
        } while (lVar12 != 0);
        puVar7 = (undefined8 *)*param_1;
        lVar9 = param_1[5];
        lVar13 = param_1[6];
        lVar16 = (long)puVar2 - (long)puVar10 >> 5;
      }
      else {
        lVar13 = param_1[6];
        lVar16 = (long)puVar2 - param_1[1] >> 5;
      }
    }
    goto LAB_0010a13a;
  }
  puVar15 = (undefined8 *)param_1[4];
  lVar16 = 0;
  lVar14 = 0;
  puVar18 = (undefined8 *)*puVar15;
  puVar6 = puVar1;
  puVar10 = puVar2;
  lVar12 = lVar11;
  if (puVar1 != puVar8) {
LAB_0010a0ae:
    do {
      if ((undefined8 *)param_1[3] == puVar10) {
        puVar15 = (undefined8 *)param_1[4];
        puVar7 = (undefined8 *)*param_1;
LAB_0010a2dd:
        lVar16 = (long)puVar2 - (long)puVar10 >> 5;
        lVar14 = (long)puVar1 - (long)puVar6 >> 5;
        lVar13 = param_1[6];
        lVar9 = param_1[5];
        if (lVar13 != lVar16) goto LAB_0010a12e;
        if ((puVar8 == puVar7) ||
           (lVar11 = lVar12 + -1, puVar10 = puVar18, puVar2 = puVar7, lVar12 == 0))
        goto LAB_0010a137;
        goto LAB_0010a322;
      }
      lVar11 = lVar12 + -1;
      if (lVar12 == 0) {
        puVar15 = (undefined8 *)param_1[4];
        lVar12 = -1;
        puVar7 = (undefined8 *)*param_1;
        goto LAB_0010a2dd;
      }
      puVar17 = puVar18 + -4;
      iVar5 = FUN_00109c70(puVar6 + -4,puVar10 + -4);
      lVar12 = lVar11;
      if (iVar5 < 1) {
        lVar13 = *param_1;
        puVar8 = (undefined8 *)param_1[2];
        uVar3 = *(undefined8 *)(lVar13 + -0x20);
        uVar4 = *(undefined8 *)(lVar13 + -0x18);
        puVar6 = (undefined8 *)(lVar13 + -0x20);
        *param_1 = (long)puVar6;
        *puVar17 = uVar3;
        puVar18[-3] = uVar4;
        uVar3 = *(undefined8 *)(lVar13 + -8);
        puVar10 = (undefined8 *)param_1[1];
        puVar18[-2] = *(undefined8 *)(lVar13 + -0x10);
        puVar18[-1] = uVar3;
        puVar18 = puVar17;
        if (puVar8 == puVar6) break;
        goto LAB_0010a0ae;
      }
      lVar13 = param_1[1];
      puVar8 = (undefined8 *)param_1[2];
      uVar3 = *(undefined8 *)(lVar13 + -0x20);
      uVar4 = *(undefined8 *)(lVar13 + -0x18);
      puVar10 = (undefined8 *)(lVar13 + -0x20);
      param_1[1] = (long)puVar10;
      *puVar17 = uVar3;
      puVar18[-3] = uVar4;
      uVar3 = *(undefined8 *)(lVar13 + -8);
      puVar6 = (undefined8 *)*param_1;
      puVar18[-2] = *(undefined8 *)(lVar13 + -0x10);
      puVar18[-1] = uVar3;
      puVar18 = puVar17;
    } while (puVar8 != puVar6);
    puVar15 = (undefined8 *)param_1[4];
    lVar14 = (long)puVar1 - (long)puVar6 >> 5;
    lVar16 = (long)puVar2 - (long)puVar10 >> 5;
    puVar7 = puVar6;
    puVar18 = puVar17;
  }
  lVar13 = param_1[6];
  lVar9 = param_1[5];
  lVar12 = lVar11;
  if (lVar16 != lVar13) {
LAB_0010a12e:
    if (((lVar14 == lVar9) && (puVar8 = (undefined8 *)param_1[3], puVar8 != puVar10)) &&
       (lVar11 = lVar12 + -1, puVar6 = puVar18, lVar12 != 0)) {
      do {
        puVar17 = puVar10 + -4;
        uVar3 = *puVar17;
        uVar4 = puVar10[-3];
        puVar18 = puVar6 + -4;
        param_1[1] = (long)puVar17;
        *puVar18 = uVar3;
        puVar6[-3] = uVar4;
        uVar3 = puVar10[-1];
        puVar6[-2] = puVar10[-2];
        puVar6[-1] = uVar3;
        if (puVar17 == puVar8) {
          lVar16 = (long)puVar2 - (long)puVar17 >> 5;
          goto LAB_0010a137;
        }
        lVar11 = lVar11 + -1;
        puVar6 = puVar18;
        puVar10 = puVar17;
      } while (lVar11 != -1);
      lVar16 = (long)puVar2 - (long)puVar17 >> 5;
    }
  }
  goto LAB_0010a137;
  while (lVar12 != 0) {
LAB_0010a286:
    lVar12 = lVar12 + -1;
    *param_1 = (long)(puVar7 + -4);
    FUN_00109fb0(puVar7 + -4,param_3,param_4);
    puVar7 = (undefined8 *)*param_1;
    if (puVar7 == (undefined8 *)param_1[2]) break;
  }
  lVar9 = param_1[5];
  lVar13 = param_1[6];
  lVar16 = (long)puVar2 - param_1[1] >> 5;
  goto LAB_0010a13a;
  while (lVar11 = lVar11 + -1, puVar10 = puVar18, puVar2 = puVar7, lVar11 != -1) {
LAB_0010a322:
    puVar7 = puVar2 + -4;
    uVar3 = *puVar7;
    uVar4 = puVar2[-3];
    puVar18 = puVar10 + -4;
    *param_1 = (long)puVar7;
    *puVar18 = uVar3;
    puVar10[-3] = uVar4;
    uVar3 = puVar2[-1];
    puVar10[-2] = puVar2[-2];
    puVar10[-1] = uVar3;
    if (puVar7 == puVar8) break;
  }
LAB_0010a137:
  *puVar15 = puVar18;
LAB_0010a13a:
  param_1[6] = lVar13 - lVar16;
  param_1[5] = lVar9 - ((long)puVar1 - (long)puVar7 >> 5);
  return;
}




// Function: merge_loop @ 0xa460

void merge_loop(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  long lVar1;
  
  while( true ) {
    lVar1 = FUN_00107c70(param_1);
    if (*(int *)(lVar1 + 0x50) == 0) break;
    FUN_0010a020(lVar1,param_2,param_3,param_4);
    FUN_00107bc0(param_1,lVar1);
    FUN_00107c10(param_1,lVar1);
    FUN_00107c00(lVar1);
  }
  FUN_00107c00(lVar1);
  FUN_00107b80(param_1,lVar1);
  return;
}




// Function: sortlines @ 0xa4e0

void sortlines(long param_1,ulong param_2,long param_3,long *param_4,undefined8 param_5,
                 undefined8 param_6,undefined8 param_7)

{
  int iVar1;
  ulong uVar2;
  long lVar3;
  ulong uVar4;
  long lVar5;
  ulong uVar6;
  long in_FS_OFFSET;
  pthread_t local_80;
  long local_78;
  ulong local_70;
  long local_68;
  long local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_48;
  long local_40;
  
  uVar4 = param_2 >> 1;
  uVar2 = param_4[5];
  uVar6 = param_4[6];
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_60 = param_4[8];
  local_48 = param_7;
  local_78 = param_1;
  local_70 = uVar4;
  local_68 = param_3;
  local_58 = param_5;
  local_50 = param_6;
  if ((0x1ffff < uVar2 + uVar6) && (1 < param_2)) {
    iVar1 = pthread_create(&local_80,(pthread_attr_t *)0x0,FUN_0010a710,&local_78);
    if (iVar1 == 0) {
      sortlines(param_1 + param_4[5] * -0x20,param_2 - uVar4,param_3,param_4[9],param_5,param_6,
                   param_7);
      pthread_join(local_80,(void **)0x0);
      goto LAB_0010a653;
    }
    uVar2 = param_4[5];
    uVar6 = param_4[6];
  }
  lVar5 = param_1 + param_3 * -0x20;
  lVar3 = param_1 + uVar2 * -0x20;
  if (1 < uVar6) {
    FUN_00109e20(lVar3,uVar6,lVar5 + (uVar2 >> 1) * -0x20,0);
  }
  if (1 < uVar2) {
    FUN_00109e20(param_1,uVar2,lVar5,0);
  }
  *param_4 = param_1;
  param_4[1] = lVar3;
  param_4[2] = lVar3;
  param_4[3] = param_1 + uVar2 * -0x20 + uVar6 * -0x20;
  FUN_00107b80(param_5,param_4);
  FUN_0010a460(param_5,param_3,param_6,param_7);
LAB_0010a653:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sortlines_thread @ 0xa710

undefined8 sortlines_thread(undefined8 *param_1)

{
  FUN_0010a4e0(*param_1,param_1[1],param_1[2],param_1[3],param_1[4],param_1[5],param_1[6]);
  return 0;
}




// Function: fillbuf @ 0xa750

undefined8 fillbuf(long *param_1,FILE *param_2,undefined8 param_3)

{
  long lVar1;
  long lVar2;
  char cVar3;
  long *plVar4;
  char *pcVar5;
  byte bVar6;
  int iVar7;
  ulong uVar8;
  size_t sVar9;
  long lVar10;
  undefined1 *puVar11;
  undefined8 uVar12;
  ulong uVar13;
  long *plVar14;
  long lVar15;
  long *plVar16;
  char *pcVar17;
  size_t sVar18;
  char *__ptr;
  ulong uVar19;
  long in_FS_OFFSET;
  undefined1 auVar20 [16];
  char *local_88;
  
  plVar4 = DAT_0011e410;
  cVar3 = DAT_0011e038;
  lVar10 = DAT_0011e030;
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  lVar2 = param_1[5];
  if ((char)param_1[6] == '\0') {
    sVar18 = param_1[1];
    sVar9 = param_1[4];
    if (sVar18 == sVar9) {
      lVar15 = param_1[2];
    }
    else {
      memmove((void *)*param_1,(void *)(*param_1 + (sVar18 - sVar9)),sVar9);
      sVar18 = param_1[4];
      param_1[2] = 0;
      lVar15 = 0;
      param_1[1] = sVar18;
    }
    uVar19 = lVar10 - 0x22;
    pcVar17 = (char *)*param_1;
    uVar13 = param_1[3];
    do {
      __ptr = pcVar17 + sVar18;
      auVar20 = FUN_0010a740(pcVar17,uVar13,lVar15);
      lVar10 = auVar20._8_8_;
      plVar16 = (long *)(auVar20._0_8_ + lVar10 * -0x20);
      uVar13 = (auVar20._0_8_ - lVar2 * lVar10) - (long)__ptr;
      if (lVar10 != 0) {
        pcVar17 = (char *)(plVar16[1] + *plVar16);
      }
      do {
        if (uVar13 <= lVar2 + 1U) break;
        uVar8 = (uVar13 - 1) / (lVar2 + 1U);
        sVar9 = fread_unlocked(__ptr,1,uVar8,param_2);
        uVar13 = uVar13 - sVar9;
        local_88 = __ptr + sVar9;
        if (uVar8 != sVar9) {
          iVar7 = ferror_unlocked(param_2);
          if (iVar7 != 0) {
            uVar12 = dcgettext(0,"read failed",5);
                    /* WARNING: Subroutine does not return */
            FUN_00105e10(uVar12,param_3);
          }
          iVar7 = feof_unlocked(param_2);
          if (iVar7 != 0) {
            *(undefined1 *)(param_1 + 6) = 1;
            if ((char *)*param_1 == local_88) goto LAB_0010a968;
            if ((pcVar17 != local_88) && (local_88[-1] != cVar3)) {
              *local_88 = cVar3;
              local_88 = local_88 + 1;
            }
          }
        }
        pcVar5 = pcVar17;
        plVar14 = plVar16;
        while (plVar16 = plVar14, pcVar17 = pcVar5,
              puVar11 = memchr(__ptr,(int)cVar3,(long)local_88 - (long)__ptr),
              puVar11 != (undefined1 *)0x0) {
          __ptr = puVar11 + 1;
          plVar14 = plVar16 + -4;
          *puVar11 = 0;
          *plVar14 = (long)pcVar17;
          uVar8 = (long)__ptr - (long)pcVar17;
          plVar16[-3] = uVar8;
          if (uVar19 < uVar8) {
            uVar19 = uVar8;
          }
          uVar13 = uVar13 - lVar2;
          pcVar5 = __ptr;
          if (plVar4 != (long *)0x0) {
            if (plVar4[2] != -1) {
              puVar11 = (undefined1 *)FUN_00108f70(pcVar17,uVar8,plVar4);
            }
            lVar10 = *plVar4;
            plVar16[-1] = (long)puVar11;
            if (lVar10 == -1) {
              if ((char)plVar4[6] != '\0') {
                while( true ) {
                  puVar11 = &DAT_0011e740;
                  bVar6 = FUN_001053e0((int)*pcVar17);
                  if (puVar11[bVar6] == '\0') break;
                  pcVar17 = pcVar17 + 1;
                }
              }
              plVar16[-2] = (long)pcVar17;
              pcVar5 = __ptr;
            }
            else {
              lVar10 = FUN_001090d0(pcVar17,uVar8,plVar4);
              plVar16[-2] = lVar10;
              pcVar5 = __ptr;
            }
          }
        }
        __ptr = local_88;
      } while ((char)param_1[6] == '\0');
      param_1[1] = (long)__ptr - *param_1;
      plVar14 = param_1;
      auVar20 = FUN_0010a740(*param_1,param_1[3]);
      lVar10 = auVar20._0_8_ - (long)plVar16 >> 5;
      plVar14[2] = lVar10;
      if (lVar10 != 0) goto LAB_0010aa7b;
      pcVar17 = (char *)FUN_00112ff0();
      uVar13 = auVar20._8_8_ & 0xffffffffffffffe0;
      sVar18 = param_1[1];
      lVar15 = param_1[2];
      *param_1 = (long)pcVar17;
      param_1[3] = uVar13;
    } while( true );
  }
LAB_0010a968:
  uVar12 = 0;
LAB_0010a96a:
  if (lVar1 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar12;
LAB_0010aa7b:
  DAT_0011e030 = uVar19 + 0x22;
  uVar12 = 1;
  plVar14[4] = (long)__ptr - (long)pcVar17;
  goto LAB_0010a96a;
}




// Function: check @ 0xaac0

undefined8 check(undefined8 param_1,char param_2)

{
  long lVar1;
  long lVar2;
  long lVar3;
  long *plVar4;
  char cVar5;
  int iVar6;
  undefined8 uVar7;
  long *plVar8;
  void *pvVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  ulong uVar13;
  uint uVar14;
  ulong uVar15;
  ulong uVar16;
  undefined8 uVar17;
  long *plVar18;
  long in_FS_OFFSET;
  long local_d8;
  void *local_b8;
  long local_b0;
  long local_a8;
  long local_a0;
  void *local_98 [2];
  long local_88;
  undefined8 local_80;
  undefined1 local_58 [24];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar7 = FUN_00106010(param_1,"r");
  lVar2 = DAT_0011e410;
  uVar14 = (uint)DAT_0011e419;
  uVar13 = DAT_0011e438;
  if (DAT_0011e438 < DAT_0011e030) {
    uVar13 = DAT_0011e030;
  }
  uVar15 = 0;
  FUN_00106240(local_98,0x20,uVar13);
  local_b8 = (void *)0x0;
  local_d8 = 0;
  do {
    cVar5 = FUN_0010a750(local_98,uVar7,param_1);
    pvVar9 = local_98[0];
    if (cVar5 == '\0') {
      uVar17 = 1;
LAB_0010acaa:
      FUN_00106040(uVar7,param_1);
      free(pvVar9);
      free(local_b8);
      if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return uVar17;
    }
    plVar8 = (long *)FUN_0010a740(local_98[0],local_80);
    lVar3 = local_88;
    lVar1 = local_88 * -4;
    if ((uVar15 != 0) &&
       (iVar6 = FUN_00109c70(&local_b8,plVar8 + -4), plVar18 = plVar8, (int)(uVar14 ^ 1) <= iVar6))
    {
LAB_0010aca0:
      uVar17 = 0;
      if (param_2 == 'c') {
        uVar10 = FUN_0010e520(((long)plVar8 - (long)(plVar18 + -4) >> 5) + local_d8,local_58);
        uVar12 = DAT_0011e888;
        uVar11 = dcgettext(0,"%s: %s:%s: disorder: ",5);
        __fprintf_chk(stderr,1,uVar11,uVar12,param_1,uVar10);
        uVar12 = dcgettext(0,"standard error",5);
        FUN_00109460(plVar18 + -4,stderr,uVar12);
      }
      goto LAB_0010acaa;
    }
    plVar4 = plVar8 + -4;
    while (plVar18 = plVar4, plVar8 + lVar1 < plVar18) {
      iVar6 = FUN_00109c70(plVar18,plVar18 + -4);
      plVar4 = plVar18 + -4;
      if ((int)(uVar14 ^ 1) <= iVar6) goto LAB_0010aca0;
    }
    uVar13 = plVar18[1];
    local_d8 = local_d8 + lVar3;
    uVar16 = uVar15;
    if (uVar15 < uVar13) {
      do {
        uVar16 = uVar16 * 2;
        uVar15 = uVar13;
        if (uVar16 == 0) break;
        uVar15 = uVar16;
      } while (uVar16 < uVar13);
      free(local_b8);
      local_b8 = (void *)FUN_00112eb0(uVar15);
      uVar13 = plVar18[1];
    }
    pvVar9 = memcpy(local_b8,(void *)*plVar18,uVar13);
    local_b0 = plVar18[1];
    if (lVar2 != 0) {
      local_a8 = (long)pvVar9 + (plVar18[2] - *plVar18);
      local_a0 = (long)pvVar9 + (plVar18[3] - *plVar18);
    }
  } while( true );
}




// Function: mergefps @ 0xad90

void mergefps(long param_1,ulong param_2,ulong param_3,undefined8 param_4,undefined8 param_5,
                 void *param_6)

{
  long lVar1;
  long *plVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  long lVar12;
  char cVar13;
  int iVar14;
  void *__ptr;
  void *__ptr_00;
  void *__ptr_01;
  ulong *__ptr_02;
  long lVar15;
  ulong *puVar16;
  void *pvVar17;
  ulong *puVar18;
  ulong uVar19;
  undefined8 *puVar20;
  ulong uVar21;
  undefined8 *puVar22;
  ulong uVar23;
  ulong uVar24;
  long in_FS_OFFSET;
  undefined1 auVar25 [16];
  ulong local_d0;
  ulong local_c8;
  void **local_b0;
  ulong local_88;
  ulong local_80;
  void *local_68;
  size_t local_60;
  long local_58;
  long local_50;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  __ptr = (void *)FUN_00112fd0(param_3,0x38);
  __ptr_00 = (void *)FUN_00112fd0(param_3,8);
  __ptr_01 = (void *)FUN_00112fd0(param_3,8);
  __ptr_02 = (ulong *)FUN_00112fd0(param_3,8);
  lVar12 = DAT_0011e410;
  local_68 = (void *)0x0;
  if (param_3 != 0) {
    uVar21 = 0;
    local_c8 = param_3;
    local_88 = param_2;
    do {
      while( true ) {
        lVar15 = uVar21 * 8;
        puVar20 = (undefined8 *)((long)__ptr + uVar21 * 0x38);
        uVar19 = DAT_0011e030;
        if (DAT_0011e030 <= DAT_0011e438 / local_c8) {
          uVar19 = DAT_0011e438 / local_c8;
        }
        puVar22 = (undefined8 *)(uVar21 * 0x10 + param_1);
        FUN_00106240(puVar20,0x20,uVar19);
        cVar13 = FUN_0010a750(puVar20,*(undefined8 *)((long)param_6 + lVar15),*puVar22);
        if (cVar13 != '\0') break;
        FUN_00106040(*(undefined8 *)((long)param_6 + lVar15),*puVar22);
        if (uVar21 < local_88) {
          local_88 = local_88 - 1;
          FUN_00106e80(*puVar22);
        }
        free((void *)*puVar20);
        uVar19 = local_c8 - 1;
        if (uVar19 <= uVar21) {
          local_c8 = uVar19;
          if (uVar19 == 0) goto LAB_0010b3cd;
          goto LAB_0010af06;
        }
        lVar1 = local_c8 * 8;
        do {
          puVar22 = (undefined8 *)(param_1 + 0x10 + lVar15 * 2);
          uVar4 = puVar22[1];
          uVar3 = *(undefined8 *)((long)param_6 + lVar15 + 8);
          puVar20 = (undefined8 *)(param_1 + lVar15 * 2);
          *puVar20 = *puVar22;
          puVar20[1] = uVar4;
          *(undefined8 *)((long)param_6 + lVar15) = uVar3;
          lVar15 = lVar15 + 8;
          local_c8 = uVar19;
        } while (lVar1 + -8 != lVar15);
      }
      lVar15 = FUN_0010a740(*puVar20,puVar20[3]);
      *(long *)((long)__ptr_00 + uVar21 * 8) = lVar15 + -0x20;
      *(long *)((long)__ptr_01 + uVar21 * 8) = lVar15 + puVar20[2] * -0x20;
      uVar21 = uVar21 + 1;
    } while (uVar21 < local_c8);
LAB_0010af06:
    uVar21 = 0;
    do {
      __ptr_02[uVar21] = uVar21;
      uVar21 = uVar21 + 1;
    } while (uVar21 != local_c8);
    if (local_c8 != 1) {
      uVar21 = 1;
      do {
        while( true ) {
          puVar16 = __ptr_02 + uVar21;
          puVar18 = __ptr_02 + (uVar21 - 1);
          iVar14 = FUN_00109c70(*(undefined8 *)((long)__ptr_00 + *puVar18 * 8),
                                *(undefined8 *)((long)__ptr_00 + *puVar16 * 8));
          if (iVar14 < 1) break;
          uVar19 = *puVar18;
          uVar21 = 1;
          *puVar18 = *puVar16;
          *puVar16 = uVar19;
        }
        uVar21 = uVar21 + 1;
      } while (uVar21 < local_c8);
    }
    local_80 = 0;
    local_b0 = (void **)0x0;
LAB_0010afa0:
    plVar2 = *(long **)((long)__ptr_00 + *__ptr_02 * 8);
    if (DAT_0011e419 == '\0') {
      FUN_00109460(plVar2,param_4,param_5);
    }
    else {
      if (local_b0 != (void **)0x0) {
        iVar14 = FUN_00109c70(local_b0,plVar2);
        if (iVar14 == 0) goto LAB_0010afd6;
        FUN_00109460(&local_68,param_4,param_5);
      }
      uVar21 = plVar2[1];
      if (local_80 < uVar21) {
        do {
          uVar19 = uVar21;
          if (local_80 == 0) break;
          local_80 = local_80 * 2;
          uVar19 = local_80;
        } while (local_80 < uVar21);
        free(local_68);
        local_68 = (void *)FUN_00112eb0(uVar19);
        uVar21 = plVar2[1];
        local_80 = uVar19;
      }
      local_60 = uVar21;
      pvVar17 = memcpy(local_68,(void *)*plVar2,uVar21);
      local_b0 = &local_68;
      if (lVar12 != 0) {
        local_58 = (plVar2[2] - *plVar2) + (long)pvVar17;
        local_50 = (long)pvVar17 + (plVar2[3] - *plVar2);
      }
    }
LAB_0010afd6:
    local_d0 = *__ptr_02;
    if (*(long **)((long)__ptr_01 + local_d0 * 8) < plVar2) {
      *(long **)((long)__ptr_00 + local_d0 * 8) = plVar2 + -4;
      if (local_c8 == 1) goto LAB_0010b14c;
LAB_0010b086:
      uVar21 = 1;
      uVar19 = 1;
      uVar24 = local_c8;
      do {
        while( true ) {
          uVar23 = uVar19;
          iVar14 = FUN_00109c70(*(undefined8 *)((long)__ptr_00 + local_d0 * 8),
                                *(undefined8 *)((long)__ptr_00 + __ptr_02[uVar23] * 8));
          if ((-1 < iVar14) && ((iVar14 != 0 || (__ptr_02[uVar23] <= local_d0)))) break;
          uVar19 = uVar21 + uVar23 >> 1;
          uVar24 = uVar23;
          if (uVar23 <= uVar21) goto LAB_0010b0f1;
        }
        uVar21 = uVar23 + 1;
        uVar19 = uVar21 + uVar24 >> 1;
      } while (uVar21 < uVar24);
LAB_0010b0f1:
      if (uVar21 - 1 != 0) {
        puVar18 = __ptr_02;
        do {
          puVar16 = puVar18 + 1;
          *puVar18 = puVar18[1];
          puVar18 = puVar16;
        } while (puVar16 != __ptr_02 + (uVar21 - 1));
      }
      __ptr_02[uVar21 - 1] = local_d0;
      goto LAB_0010afa0;
    }
    cVar13 = FUN_0010a750((void *)((long)__ptr + local_d0 * 0x38),
                          *(undefined8 *)((long)param_6 + local_d0 * 8),
                          *(undefined8 *)(param_1 + local_d0 * 0x10));
    if (cVar13 != '\0') {
      local_d0 = *__ptr_02;
      puVar20 = (undefined8 *)((long)__ptr + local_d0 * 0x38);
      auVar25 = FUN_0010a740(*puVar20,puVar20[3]);
      lVar15 = *(long *)(auVar25._8_8_ + 0x10);
      *(long *)((long)__ptr_00 + local_d0 * 8) = auVar25._0_8_ + -0x20;
      *(long *)((long)__ptr_01 + local_d0 * 8) = auVar25._0_8_ + lVar15 * -0x20;
      if (local_c8 != 1) goto LAB_0010b086;
LAB_0010b14c:
      *__ptr_02 = local_d0;
      goto LAB_0010afa0;
    }
    uVar21 = *__ptr_02;
    puVar18 = __ptr_02 + 1;
    if (local_c8 != 1) {
      do {
        if (uVar21 < *puVar18) {
          *puVar18 = *puVar18 - 1;
          uVar21 = *__ptr_02;
        }
        puVar18 = puVar18 + 1;
      } while (__ptr_02 + local_c8 != puVar18);
    }
    uVar19 = local_c8 - 1;
    FUN_00106040(*(undefined8 *)((long)param_6 + uVar21 * 8),
                 *(undefined8 *)(param_1 + uVar21 * 0x10));
    uVar21 = *__ptr_02;
    if (uVar21 < local_88) {
      local_88 = local_88 - 1;
      FUN_00106e80(*(undefined8 *)(param_1 + uVar21 * 0x10));
      uVar21 = *__ptr_02;
    }
    free(*(void **)((long)__ptr + uVar21 * 0x38));
    uVar21 = *__ptr_02;
    if (uVar21 < uVar19) {
      lVar15 = uVar21 * 8;
      puVar20 = (undefined8 *)((long)__ptr + uVar21 * 0x38);
      do {
        puVar22 = (undefined8 *)(param_1 + 0x10 + lVar15 * 2);
        uVar4 = *puVar22;
        uVar5 = puVar22[1];
        uVar6 = puVar20[7];
        uVar7 = puVar20[8];
        uVar8 = puVar20[9];
        uVar9 = puVar20[10];
        *(undefined8 *)((long)param_6 + lVar15) = *(undefined8 *)((long)param_6 + lVar15 + 8);
        uVar3 = puVar20[0xd];
        uVar10 = puVar20[0xb];
        uVar11 = puVar20[0xc];
        puVar22 = (undefined8 *)(param_1 + lVar15 * 2);
        *puVar22 = uVar4;
        puVar22[1] = uVar5;
        puVar20[6] = uVar3;
        uVar3 = *(undefined8 *)((long)__ptr_00 + lVar15 + 8);
        *puVar20 = uVar6;
        puVar20[1] = uVar7;
        *(undefined8 *)((long)__ptr_00 + lVar15) = uVar3;
        uVar3 = *(undefined8 *)((long)__ptr_01 + lVar15 + 8);
        puVar20[2] = uVar8;
        puVar20[3] = uVar9;
        *(undefined8 *)((long)__ptr_01 + lVar15) = uVar3;
        lVar15 = lVar15 + 8;
        puVar20[4] = uVar10;
        puVar20[5] = uVar11;
        puVar20 = puVar20 + 7;
      } while (local_c8 * 8 + -8 != lVar15);
    }
    if (uVar19 != 0) {
      lVar15 = local_c8 - 1;
      puVar18 = __ptr_02;
      do {
        puVar16 = puVar18 + 1;
        *puVar18 = puVar18[1];
        puVar18 = puVar16;
        local_c8 = uVar19;
      } while (__ptr_02 + lVar15 != puVar16);
      goto LAB_0010afa0;
    }
    if ((local_b0 != (void **)0x0) && (DAT_0011e419 != '\0')) {
      FUN_00109460(&local_68,param_4,param_5);
      free(local_68);
    }
  }
LAB_0010b3cd:
  FUN_00106040(param_4,param_5);
  free(param_6);
  free(__ptr);
  free(__ptr_02);
  free(__ptr_01);
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  free(__ptr_00);
  return;
}




// Function: mergefiles @ 0xb500

ulong mergefiles(long param_1,undefined8 param_2,ulong param_3,undefined8 param_4,
                  undefined8 param_5)

{
  undefined8 uVar1;
  ulong uVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined8 local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar2 = FUN_00107470(param_1,param_3,&local_48);
  if ((uVar2 < param_3) && (uVar2 < 2)) {
    uVar1 = *(undefined8 *)(param_1 + uVar2 * 0x10);
    uVar3 = dcgettext(0,"open failed",5);
                    /* WARNING: Subroutine does not return */
    FUN_00105e10(uVar3,uVar1);
  }
  FUN_0010ad90(param_1,param_2,uVar2,param_4,param_5,local_48);
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: avoid_trashing_input @ 0xb5b0

void avoid_trashing_input(long param_1,ulong param_2,ulong param_3,char *param_4)

{
  long *plVar1;
  char *__s1;
  int iVar2;
  int iVar3;
  __dev_t *p_Var4;
  long lVar5;
  long *plVar6;
  long *plVar7;
  long lVar8;
  long in_FS_OFFSET;
  undefined8 local_e0;
  stat local_d8;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 < param_3) {
    lVar5 = 0;
    plVar1 = (long *)(param_1 + param_3 * 0x10);
    plVar6 = (long *)(param_1 + param_2 * 0x10);
    do {
      while( true ) {
        __s1 = (char *)*plVar6;
        iVar2 = strcmp(__s1,"-");
        if (param_4 == (char *)0x0) break;
        iVar3 = strcmp(param_4,__s1);
        if (iVar3 != 0) break;
        if (iVar2 == 0) {
          p_Var4 = (__dev_t *)FUN_00105e70();
          if (p_Var4 != (__dev_t *)0x0) goto LAB_0010b629;
          goto LAB_0010b6cb;
        }
LAB_0010b6b0:
        lVar8 = lVar5 + 0xd;
        if (lVar5 == 0) {
          lVar5 = FUN_001077e0(&local_e0);
          lVar8 = lVar5 + 0xd;
          FUN_0010b500(plVar6,0,1,local_e0,lVar8);
        }
        *plVar6 = lVar8;
        plVar7 = plVar6 + 2;
        plVar6[1] = lVar5;
        plVar6 = plVar7;
        if (plVar7 == plVar1) goto LAB_0010b6cb;
      }
      p_Var4 = (__dev_t *)FUN_00105e70();
      if (p_Var4 == (__dev_t *)0x0) break;
      if (iVar2 == 0) {
LAB_0010b629:
        iVar2 = fstat(0,&local_d8);
        if ((iVar2 == 0) && (local_d8.st_ino == p_Var4[1])) goto LAB_0010b6a3;
      }
      else {
        iVar2 = stat((char *)*plVar6,&local_d8);
        if ((iVar2 == 0) && (local_d8.st_ino == p_Var4[1])) {
LAB_0010b6a3:
          if (local_d8.st_dev == *p_Var4) goto LAB_0010b6b0;
        }
      }
      plVar6 = plVar6 + 2;
    } while (plVar6 != plVar1);
  }
LAB_0010b6cb:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: merge @ 0xb720

void merge(long *param_1,ulong param_2,ulong param_3,undefined8 param_4)

{
  long lVar1;
  ulong uVar2;
  ulong uVar3;
  int *piVar4;
  undefined8 uVar5;
  ulong uVar6;
  ulong uVar7;
  long lVar8;
  long *plVar9;
  ulong uVar10;
  ulong uVar11;
  long *__src;
  long in_FS_OFFSET;
  ulong local_80;
  ulong local_78;
  long local_50;
  undefined8 local_48;
  long local_40;
  
  uVar3 = (ulong)DAT_0011e024;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_78 = param_3;
  if (uVar3 < param_3) {
    do {
      uVar2 = 0;
      lVar8 = 0;
      if (local_78 < uVar3) {
        uVar6 = 0;
        uVar7 = uVar3;
        uVar10 = local_78;
        plVar9 = param_1;
        __src = param_1;
      }
      else {
        do {
          uVar11 = uVar2;
          lVar1 = FUN_001077e0(&local_48);
          uVar2 = (ulong)DAT_0011e024;
          uVar3 = param_2;
          if (uVar2 <= param_2) {
            uVar3 = uVar2;
          }
          uVar2 = FUN_0010b500(param_1 + lVar8 * 2,uVar3,uVar2,local_48);
          uVar7 = (ulong)DAT_0011e024;
          uVar3 = param_2;
          if (uVar2 <= param_2) {
            uVar3 = uVar2;
          }
          lVar8 = lVar8 + uVar2;
          uVar2 = uVar11 + 1;
          param_1[uVar11 * 2] = lVar1 + 0xd;
          param_1[uVar11 * 2 + 1] = lVar1;
          param_2 = param_2 - uVar3;
        } while (uVar7 <= local_78 - lVar8);
        uVar6 = uVar2 % uVar7;
        uVar3 = uVar7 - uVar6;
        uVar10 = local_78 - lVar8;
        plVar9 = param_1 + uVar11 * 2 + 2;
        __src = param_1 + lVar8 * 2;
      }
      if (uVar3 < uVar10) {
        uVar7 = (uVar10 + 1 + uVar6) - uVar7;
        lVar1 = FUN_001077e0(&local_48);
        uVar3 = param_2;
        if (uVar7 <= param_2) {
          uVar3 = uVar7;
        }
        uVar7 = FUN_0010b500(__src,uVar3,uVar7,local_48);
        uVar3 = param_2;
        if (uVar7 <= param_2) {
          uVar3 = uVar7;
        }
        uVar2 = uVar2 + 1;
        lVar8 = lVar8 + uVar7;
        *plVar9 = lVar1 + 0xd;
        plVar9[1] = lVar1;
        param_2 = param_2 - uVar3;
        __src = param_1 + lVar8 * 2;
        plVar9 = param_1 + uVar2 * 2;
      }
      param_2 = param_2 + uVar2;
      memmove(plVar9,__src,(local_78 - lVar8) * 0x10);
      uVar3 = (ulong)DAT_0011e024;
      local_78 = local_78 + (uVar2 - lVar8);
    } while (uVar3 < local_78);
  }
  FUN_0010b5b0(param_1,param_2,local_78,param_4);
  local_80 = param_2;
  do {
    uVar3 = FUN_00107470(param_1,local_78,&local_50);
    if (local_78 == uVar3) {
      lVar8 = FUN_00105ee0(param_4,&DAT_00116c04);
      if (lVar8 != 0) {
        FUN_0010ad90(param_1,local_80,local_78,lVar8,param_4,local_50);
        if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
          __stack_chk_fail();
        }
        return;
      }
      piVar4 = __errno_location();
      if ((*piVar4 != 0x18) || (local_78 < 3)) {
        uVar5 = dcgettext(0,"open failed",5);
                    /* WARNING: Subroutine does not return */
        FUN_00105e10(uVar5,param_4);
      }
    }
    else if (uVar3 < 3) {
      lVar8 = param_1[uVar3 * 2];
      uVar5 = dcgettext(0,"open failed",5);
                    /* WARNING: Subroutine does not return */
      FUN_00105e10(uVar5,lVar8);
    }
    uVar3 = uVar3 - 1;
    plVar9 = param_1 + uVar3 * 2;
    while( true ) {
      FUN_00106040(*(undefined8 *)(local_50 + uVar3 * 8),*plVar9);
      lVar8 = FUN_001076b0(&local_48,2 < uVar3);
      if (lVar8 != 0) break;
      uVar3 = uVar3 - 1;
      plVar9 = plVar9 + -2;
    }
    uVar2 = local_80;
    if (uVar3 < local_80) {
      uVar2 = uVar3;
    }
    lVar1 = local_78 - uVar3;
    FUN_0010ad90(param_1,uVar2,uVar3,local_48,lVar8 + 0xd,local_50);
    local_78 = lVar1 + 1;
    *param_1 = lVar8 + 0xd;
    param_1[1] = lVar8;
    memmove(param_1 + 2,plVar9,lVar1 * 0x10);
    local_80 = (local_80 - uVar2) + 1;
  } while( true );
}




// Function: sort @ 0xbac0

void sort(undefined8 *param_1,long param_2,long param_3,ulong param_4)

{
  undefined8 uVar1;
  undefined8 *puVar2;
  long *plVar3;
  char cVar4;
  long lVar5;
  long lVar6;
  long lVar7;
  ulong uVar8;
  undefined8 uVar9;
  long *__ptr;
  undefined8 extraout_RDX;
  char cVar10;
  long in_FS_OFFSET;
  long local_138;
  long local_118;
  long local_110;
  undefined8 *local_108;
  undefined8 local_f8;
  undefined8 local_f0;
  void *local_e8;
  long local_e0;
  ulong local_d8;
  long local_d0;
  long local_c8;
  char local_b8;
  undefined1 local_a8 [104];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_d0 = 0;
  local_118 = param_2;
  if (param_2 != 0) {
    local_138 = 0;
    local_108 = param_1;
    do {
      lVar6 = local_d0;
      uVar1 = *local_108;
      local_f8 = FUN_00106010(uVar1,"r");
      local_110 = 0x30;
      if (1 < param_4) {
        local_110 = 1;
        uVar8 = 1;
        do {
          uVar8 = uVar8 * 2;
          local_110 = local_110 + 1;
        } while (uVar8 < param_4);
        local_110 = local_110 * 0x20;
      }
      if (lVar6 == 0) {
        uVar9 = FUN_00108700(&local_f8,local_108,local_118,local_110);
        FUN_00106240(&local_e8,local_110,uVar9);
      }
      local_118 = local_118 + -1;
      local_b8 = '\0';
      while (cVar4 = FUN_0010a750(&local_e8,local_f8,uVar1), cVar10 = local_b8, uVar8 = local_d8,
            cVar4 != '\0') {
        if (local_b8 == '\0') {
LAB_0010bc43:
          DAT_0011e840 = 0;
          lVar6 = FUN_0010a740(local_e8);
LAB_0010bc5b:
          local_138 = local_138 + 1;
          cVar10 = '\0';
          lVar7 = FUN_001077e0(&local_f0);
          lVar7 = lVar7 + 0xd;
        }
        else {
          if (local_118 != 0) {
            if ((local_d0 - local_e0) - local_110 * local_d8 <= local_110 + 1U) goto LAB_0010bc43;
            local_c8 = local_e0;
            FUN_00106040(local_f8,uVar1);
            goto LAB_0010bd03;
          }
          DAT_0011e840 = 0;
          lVar6 = FUN_0010a740(local_e8);
          if (local_138 != 0 || local_c8 != 0) goto LAB_0010bc5b;
          FUN_00106040(local_f8,uVar1);
          local_f0 = FUN_00106010(param_3,&DAT_00116c04);
          local_138 = 0;
          lVar7 = param_3;
        }
        if (uVar8 < 2) {
          FUN_00109fb0(lVar6 + -0x20,local_f0,lVar7);
          uVar9 = extraout_RDX;
        }
        else {
          FUN_00107b40(local_a8,param_4);
          lVar5 = FUN_00107aa0(param_4,uVar8,lVar6);
          FUN_0010a4e0(lVar6,param_4,uVar8,lVar5 + 0x80,local_a8,local_f0,lVar7);
          FUN_00107cd0(param_4,lVar5);
          FUN_00107d10(local_a8);
          uVar9 = 0x10bb91;
        }
        FUN_00106040(local_f0,lVar7,uVar9);
        if (cVar10 != '\0') {
          free(local_e8);
          goto LAB_0010bd22;
        }
      }
      FUN_00106040(local_f8,uVar1);
      if (local_118 == 0) goto LAB_0010bdb4;
LAB_0010bd03:
      local_108 = local_108 + 1;
    } while( true );
  }
LAB_0010bdbd:
  free(local_e8);
  puVar2 = DAT_0011e368;
  __ptr = (long *)FUN_00112fd0(local_118,0x10);
  plVar3 = __ptr;
  while (puVar2 != (undefined8 *)0x0) {
    plVar3[1] = (long)puVar2;
    lVar6 = (long)puVar2 + 0xd;
    puVar2 = (undefined8 *)*puVar2;
    *plVar3 = lVar6;
    plVar3 = plVar3 + 2;
  }
  FUN_0010b720(__ptr,local_118,local_118,param_3);
  free(__ptr);
LAB_0010bd22:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  FUN_00107010();
  return;
LAB_0010bdb4:
  local_118 = local_138;
  goto LAB_0010bdbd;
}




// Function: usage @ 0xbe50

void usage(int param_1)

{
  FILE *pFVar1;
  undefined8 uVar2;
  char *pcVar3;
  undefined8 uVar4;
  
  uVar4 = DAT_0011e888;
  if (param_1 == 0) {
    uVar2 = dcgettext(0,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5
                     );
    __printf_chk(1,uVar2,uVar4,uVar4);
    pFVar1 = stdout;
    pcVar3 = (char *)dcgettext(0,"Write sorted concatenation of all FILE(s) to standard output.\n",5
                              );
    fputs_unlocked(pcVar3,pFVar1);
    FUN_001058d0();
    FUN_00105900();
    pFVar1 = stdout;
    pcVar3 = (char *)dcgettext(0,"Ordering options:\n\n",5);
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)dcgettext(0,
                               "  -b, --ignore-leading-blanks  ignore leading blanks\n  -d, --dictionary-order      consider only blanks and alphanumeric characters\n  -f, --ignore-case           fold lower case to upper case characters\n"
                               ,5);
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)dcgettext(0,
                               "  -g, --general-numeric-sort  compare according to general numerical value\n  -i, --ignore-nonprinting    consider only printable characters\n  -M, --month-sort            compare (unknown) < \'JAN\' < ... < \'DEC\'\n"
                               ,5);
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)dcgettext(0,
                               "  -h, --human-numeric-sort    compare human readable numbers (e.g., 2K 1G)\n"
                               ,5);
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)dcgettext(0,
                               "  -n, --numeric-sort          compare according to string numerical value\n  -R, --random-sort           shuffle, but group identical keys.  See shuf(1)\n      --random-source=FILE    get random bytes from FILE\n  -r, --reverse               reverse the result of comparisons\n"
                               ,5);
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)dcgettext(0,
                               "      --sort=WORD             sort according to WORD:\n                                general-numeric -g, human-numeric -h, month -M,\n                                numeric -n, random -R, version -V\n  -V, --version-sort          natural sort of (version) numbers within text\n\n"
                               ,5);
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)dcgettext(0,"Other options:\n\n",5);
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)dcgettext(0,
                               "      --batch-size=NMERGE   merge at most NMERGE inputs at once;\n                            for more use temp files\n"
                               ,5);
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)dcgettext(0,
                               "  -c, --check, --check=diagnose-first  check for sorted input; do not sort\n  -C, --check=quiet, --check=silent  like -c, but do not report first bad line\n      --compress-program=PROG  compress temporaries with PROG;\n                              decompress them with PROG -d\n"
                               ,5);
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)dcgettext(0,
                               "      --debug               annotate the part of the line used to sort,\n                              and warn about questionable usage to stderr\n      --files0-from=F       read input from the files specified by\n                            NUL-terminated names in file F;\n                            If F is - then read names from standard input\n"
                               ,5);
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)dcgettext(0,
                               "  -k, --key=KEYDEF          sort via a key; KEYDEF gives location and type\n  -m, --merge               merge already sorted files; do not sort\n"
                               ,5);
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)dcgettext(0,
                               "  -o, --output=FILE         write result to FILE instead of standard output\n  -s, --stable              stabilize sort by disabling last-resort comparison\n  -S, --buffer-size=SIZE    use SIZE for main memory buffer\n"
                               ,5);
    fputs_unlocked(pcVar3,pFVar1);
    uVar4 = dcgettext(0,
                      "  -t, --field-separator=SEP  use SEP instead of non-blank to blank transition\n  -T, --temporary-directory=DIR  use DIR for temporaries, not $TMPDIR or %s;\n                              multiple options specify multiple directories\n      --parallel=N          change the number of sorts run concurrently to N\n  -u, --unique              with -c, check for strict ordering;\n                              without -c, output only the first of an equal run\n"
                      ,5);
    __printf_chk(1,uVar4,&DAT_00116db4);
    pFVar1 = stdout;
    pcVar3 = (char *)dcgettext(0,"  -z, --zero-terminated     line delimiter is NUL, not newline\n",
                               5);
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)dcgettext(0,"      --help        display this help and exit\n",5);
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)dcgettext(0,"      --version     output version information and exit\n",5);
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)dcgettext(0,
                               "\nKEYDEF is F[.C][OPTS][,F[.C][OPTS]] for start and stop position, where F is a\nfield number and C a character position in the field; both are origin 1, and\nthe stop position defaults to the line\'s end.  If neither -t nor -b is in\neffect, characters in a field are counted from the beginning of the preceding\nwhitespace.  OPTS is one or more single-letter ordering options [bdfgiMhnRrV],\nwhich override global ordering options for that key.  If no key is given, use\nthe entire line as the key.  Use --debug to diagnose incorrect key usage.\n\nSIZE may be followed by the following multiplicative suffixes:\n"
                               ,5);
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)dcgettext(0,
                               "% 1% of memory, b 1, K 1024 (default), and so on for M, G, T, P, E, Z, Y.\n\n*** WARNING ***\nThe locale specified by the environment affects sort order.\nSet LC_ALL=C to get the traditional sort order that uses\nnative byte values.\n"
                               ,5);
    fputs_unlocked(pcVar3,pFVar1);
    FUN_00108d30();
  }
  else {
    uVar2 = dcgettext(0,"Try \'%s --help\' for more information.\n",5);
    __fprintf_chk(stderr,1,uVar2,uVar4);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: is_prime @ 0xd040

undefined8 is_prime(ulong param_1)

{
  undefined1 auVar1 [16];
  ulong uVar2;
  ulong uVar3;
  ulong uVar4;
  ulong uVar5;
  long lVar6;
  
  auVar1._8_8_ = 0;
  auVar1._0_8_ = param_1;
  uVar2 = SUB168(auVar1 * ZEXT816(0xaaaaaaaaaaaaaaab),0);
  uVar4 = param_1 - ((SUB168(auVar1 * ZEXT816(0xaaaaaaaaaaaaaaab),8) & 0xfffffffffffffffe) +
                    param_1 / 3);
  if ((9 < param_1) && (uVar4 != 0)) {
    lVar6 = 0x10;
    uVar5 = 9;
    uVar3 = 3;
    do {
      uVar3 = uVar3 + 2;
      uVar5 = uVar5 + lVar6;
      uVar2 = param_1 / uVar3;
      uVar4 = param_1 % uVar3;
      if (param_1 <= uVar5) break;
      lVar6 = lVar6 + 8;
    } while (uVar4 != 0);
  }
  return CONCAT71((int7)(uVar2 >> 8),uVar4 != 0);
}




// Function: next_prime @ 0xd0b0

ulong next_prime(ulong param_1)

{
  char cVar1;
  ulong uVar2;
  
  uVar2 = 10;
  if (9 < param_1) {
    uVar2 = param_1;
  }
  uVar2 = uVar2 | 1;
  while ((uVar2 != 0xffffffffffffffff && (cVar1 = FUN_0010d040(uVar2), cVar1 == '\0'))) {
    uVar2 = uVar2 + 2;
  }
  return uVar2;
}




// Function: raw_hasher @ 0xd0f0

ulong raw_hasher(undefined8 param_1,ulong param_2)

{
  ulong uVar1;
  
  uVar1 = thunk_FUN_00113a40(param_1,3);
  return uVar1 % param_2;
}




// Function: raw_comparator @ 0xd110

bool raw_comparator(long param_1,long param_2)

{
  return param_2 == param_1;
}




// Function: check_tuning @ 0xd120

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 check_tuning(long param_1)

{
  float fVar1;
  float *pfVar2;
  float fVar3;
  
  pfVar2 = *(float **)(param_1 + 0x28);
  if (pfVar2 == (float *)&DAT_00117200) {
    return 1;
  }
  fVar1 = pfVar2[2];
  if ((((_DAT_00117214 < fVar1) && (fVar1 < DAT_00117218)) && (_DAT_0011721c < pfVar2[3])) &&
     (_DAT_00117110 <= *pfVar2)) {
    fVar3 = *pfVar2 + _DAT_00117214;
    if (((fVar3 < pfVar2[1]) && (pfVar2[1] <= DAT_00117220)) && (fVar3 < fVar1)) {
      return 1;
    }
  }
  *(undefined **)(param_1 + 0x28) = &DAT_00117200;
  return 0;
}




// Function: free_entry @ 0xd1b0

void free_entry(long param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  
  uVar1 = *(undefined8 *)(param_1 + 0x48);
  *param_2 = 0;
  param_2[1] = uVar1;
  *(undefined8 **)(param_1 + 0x48) = param_2;
  return;
}




// Function: safe_hasher @ 0xd1d0

long safe_hasher(long *param_1,undefined8 param_2)

{
  ulong uVar1;
  
  uVar1 = (*(code *)param_1[6])(param_2,param_1[2]);
  if (uVar1 < (ulong)param_1[2]) {
    return uVar1 * 0x10 + *param_1;
  }
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: hash_find_entry @ 0xd200

long hash_find_entry(long param_1,long param_2,undefined8 *param_3,char param_4)

{
  long *plVar1;
  long lVar2;
  char cVar3;
  long *plVar4;
  long lVar5;
  
  plVar4 = (long *)FUN_0010d1d0();
  *param_3 = plVar4;
  lVar5 = *plVar4;
  if (lVar5 == 0) {
LAB_0010d2a1:
    lVar5 = 0;
  }
  else {
    if (lVar5 != param_2) {
      cVar3 = (**(code **)(param_1 + 0x38))(param_2);
      if (cVar3 == '\0') {
        for (; (long *)plVar4[1] != (long *)0x0; plVar4 = (long *)plVar4[1]) {
          if ((*(long *)plVar4[1] == param_2) ||
             (cVar3 = (**(code **)(param_1 + 0x38))(param_2), cVar3 != '\0')) {
            lVar5 = *(long *)plVar4[1];
            if (param_4 == '\0') {
              return lVar5;
            }
            plVar4[1] = ((long *)plVar4[1])[1];
            FUN_0010d1b0(param_1);
            return lVar5;
          }
        }
        goto LAB_0010d2a1;
      }
      lVar5 = *plVar4;
    }
    if (param_4 != '\0') {
      plVar1 = (long *)plVar4[1];
      if (plVar1 != (long *)0x0) {
        lVar2 = plVar1[1];
        *plVar4 = *plVar1;
        plVar4[1] = lVar2;
        FUN_0010d1b0(param_1);
        return lVar5;
      }
      *plVar4 = 0;
    }
  }
  return lVar5;
}




// Function: allocate_entry @ 0xd300

void allocate_entry(long param_1)

{
  if (*(long *)(param_1 + 0x48) != 0) {
    *(undefined8 *)(param_1 + 0x48) = *(undefined8 *)(*(long *)(param_1 + 0x48) + 8);
    return;
  }
  malloc(0x10);
  return;
}




// Function: transfer_entries @ 0xd330

undefined8 transfer_entries(long param_1,undefined8 *param_2,char param_3)

{
  long lVar1;
  long *plVar2;
  long *plVar3;
  long *plVar4;
  long *plVar5;
  long lVar6;
  
  plVar5 = (long *)*param_2;
  if ((long *)param_2[1] <= plVar5) {
    return 1;
  }
  do {
    while (lVar6 = *plVar5, lVar6 == 0) {
LAB_0010d358:
      plVar5 = plVar5 + 2;
      if ((long *)param_2[1] <= plVar5) {
        return 1;
      }
    }
    plVar3 = (long *)plVar5[1];
    if ((long *)plVar5[1] != (long *)0x0) {
      do {
        while( true ) {
          lVar6 = *plVar3;
          plVar2 = (long *)FUN_0010d1d0(param_1,lVar6);
          plVar4 = (long *)plVar3[1];
          if (*plVar2 != 0) break;
          *plVar2 = lVar6;
          *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
          FUN_0010d1b0(param_1);
          plVar3 = plVar4;
          if (plVar4 == (long *)0x0) goto LAB_0010d3c8;
        }
        plVar3[1] = plVar2[1];
        plVar2[1] = (long)plVar3;
        plVar3 = plVar4;
      } while (plVar4 != (long *)0x0);
LAB_0010d3c8:
      lVar6 = *plVar5;
    }
    plVar5[1] = 0;
    if (param_3 != '\0') goto LAB_0010d358;
    plVar3 = (long *)FUN_0010d1d0(param_1,lVar6);
    if (*plVar3 == 0) {
      *plVar3 = lVar6;
      *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
    }
    else {
      plVar4 = (long *)FUN_0010d300(param_1);
      if (plVar4 == (long *)0x0) {
        return 0;
      }
      lVar1 = plVar3[1];
      *plVar4 = lVar6;
      plVar4[1] = lVar1;
      plVar3[1] = (long)plVar4;
    }
    *plVar5 = 0;
    plVar5 = plVar5 + 2;
    param_2[3] = param_2[3] + -1;
    if ((long *)param_2[1] <= plVar5) {
      return 1;
    }
  } while( true );
}




// Function: hash_get_n_buckets @ 0xd4f0

undefined8 hash_get_n_buckets(long param_1)

{
  return *(undefined8 *)(param_1 + 0x10);
}




// Function: hash_get_n_buckets_used @ 0xd500

undefined8 hash_get_n_buckets_used(long param_1)

{
  return *(undefined8 *)(param_1 + 0x18);
}




// Function: hash_get_n_entries @ 0xd510

undefined8 hash_get_n_entries(long param_1)

{
  return *(undefined8 *)(param_1 + 0x20);
}




// Function: hash_get_max_bucket_length @ 0xd520

ulong hash_get_max_bucket_length(undefined8 *param_1)

{
  long lVar1;
  long *plVar2;
  ulong uVar3;
  ulong uVar4;
  
  plVar2 = (long *)*param_1;
  uVar4 = 0;
  do {
    if ((long *)param_1[1] <= plVar2) {
      return uVar4;
    }
    while (*plVar2 != 0) {
      uVar3 = 1;
      for (lVar1 = plVar2[1]; lVar1 != 0; lVar1 = *(long *)(lVar1 + 8)) {
        uVar3 = uVar3 + 1;
      }
      if (uVar4 < uVar3) {
        uVar4 = uVar3;
      }
      plVar2 = plVar2 + 2;
      if ((long *)param_1[1] <= plVar2) {
        return uVar4;
      }
    }
    plVar2 = plVar2 + 2;
  } while( true );
}




// Function: hash_print_statistics @ 0xd5f0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void hash_print_statistics(undefined8 param_1,undefined8 param_2)

{
  undefined8 uVar1;
  ulong uVar2;
  ulong uVar3;
  undefined8 uVar4;
  
  uVar1 = FUN_0010d510();
  uVar2 = FUN_0010d4f0();
  uVar3 = FUN_0010d500();
  uVar4 = FUN_0010d520();
  __fprintf_chk(param_2,1,"# entries:         %lu\n",uVar1);
  __fprintf_chk(param_2,1,"# buckets:         %lu\n",uVar2);
  __fprintf_chk(((double)uVar3 * _DAT_00117110) / (double)uVar2,param_2,1,
                "# buckets used:    %lu (%.2f%%)\n",uVar3);
  __fprintf_chk(param_2,1,"max bucket length: %lu\n",uVar4);
  return;
}




// Function: hash_lookup @ 0xd710

long hash_lookup(long param_1,long param_2)

{
  char cVar1;
  long *plVar2;
  long lVar3;
  
  plVar2 = (long *)FUN_0010d1d0();
  lVar3 = *plVar2;
  if (lVar3 == 0) {
    return 0;
  }
  while( true ) {
    if (lVar3 == param_2) {
      return param_2;
    }
    cVar1 = (**(code **)(param_1 + 0x38))(param_2);
    if (cVar1 != '\0') break;
    plVar2 = (long *)plVar2[1];
    if (plVar2 == (long *)0x0) {
      return 0;
    }
    lVar3 = *plVar2;
  }
  return *plVar2;
}




// Function: hash_get_next @ 0xd7c0

long hash_get_next(long param_1,long param_2)

{
  long lVar1;
  long *plVar2;
  long *plVar3;
  
  plVar2 = (long *)FUN_0010d1d0();
  plVar3 = plVar2;
  do {
    lVar1 = *plVar3;
    plVar3 = (long *)plVar3[1];
    if (lVar1 == param_2) {
      if (plVar3 != (long *)0x0) {
        return *plVar3;
      }
      break;
    }
  } while (plVar3 != (long *)0x0);
  do {
    plVar2 = plVar2 + 2;
    if (*(long **)(param_1 + 8) <= plVar2) {
      return 0;
    }
  } while (*plVar2 == 0);
  return *plVar2;
}




// Function: hash_do_for_each @ 0xd880

long hash_do_for_each(ulong *param_1,code *param_2,undefined8 param_3)

{
  char cVar1;
  long *plVar2;
  long lVar3;
  long lVar4;
  long *plVar5;
  
  plVar5 = (long *)*param_1;
  if (plVar5 < (long *)param_1[1]) {
    lVar4 = 0;
    do {
      lVar3 = *plVar5;
      plVar2 = plVar5;
      if (lVar3 != 0) {
        while( true ) {
          cVar1 = (*param_2)(lVar3,param_3);
          if (cVar1 == '\0') {
            return lVar4;
          }
          plVar2 = (long *)plVar2[1];
          lVar4 = lVar4 + 1;
          if (plVar2 == (long *)0x0) break;
          lVar3 = *plVar2;
        }
      }
      plVar5 = plVar5 + 2;
    } while (plVar5 < (long *)param_1[1]);
  }
  else {
    lVar4 = 0;
  }
  return lVar4;
}




// Function: hash_initialize @ 0xd970

undefined8 *
hash_initialize(undefined8 param_1,undefined *param_2,code *param_3,code *param_4,undefined8 param_5)

{
  char cVar1;
  undefined8 *__ptr;
  size_t __nmemb;
  void *pvVar2;
  
  if (param_3 == (code *)0x0) {
    param_3 = FUN_0010d0f0;
  }
  if (param_4 == (code *)0x0) {
    param_4 = FUN_0010d110;
  }
  __ptr = malloc(0x50);
  if (__ptr != (undefined8 *)0x0) {
    if (param_2 == (undefined *)0x0) {
      param_2 = &DAT_00117200;
    }
    __ptr[5] = param_2;
    cVar1 = FUN_0010d120(__ptr);
    if (cVar1 != '\0') {
      __nmemb = FUN_0010d460(*(undefined4 *)(param_2 + 8),param_1,param_2[0x10]);
      __ptr[2] = __nmemb;
      if (__nmemb != 0) {
        pvVar2 = calloc(__nmemb,0x10);
        *__ptr = pvVar2;
        if (pvVar2 != (void *)0x0) {
          __ptr[6] = param_3;
          __ptr[3] = 0;
          __ptr[7] = param_4;
          __ptr[8] = param_5;
          __ptr[1] = (void *)((long)pvVar2 + __ptr[2] * 0x10);
          __ptr[4] = 0;
          __ptr[9] = 0;
          return __ptr;
        }
      }
    }
    free(__ptr);
  }
  return (undefined8 *)0x0;
}




// Function: hash_clear @ 0xda80

void hash_clear(undefined8 *param_1)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  code *pcVar4;
  code *pcVar5;
  long *plVar6;
  long *plVar7;
  
  plVar6 = (long *)*param_1;
  if ((long *)*param_1 < (long *)param_1[1]) {
    do {
      while (*plVar6 != 0) {
        puVar3 = (undefined8 *)plVar6[1];
        pcVar4 = (code *)param_1[8];
        while (puVar3 != (undefined8 *)0x0) {
          pcVar5 = (code *)0x0;
          if (pcVar4 != (code *)0x0) {
            (*pcVar4)(*puVar3);
            pcVar5 = (code *)param_1[8];
          }
          puVar1 = (undefined8 *)puVar3[1];
          uVar2 = param_1[9];
          *puVar3 = 0;
          puVar3[1] = uVar2;
          param_1[9] = puVar3;
          puVar3 = puVar1;
          pcVar4 = pcVar5;
        }
        if (pcVar4 != (code *)0x0) {
          (*pcVar4)(*plVar6);
        }
        *plVar6 = 0;
        plVar7 = plVar6 + 2;
        plVar6[1] = 0;
        plVar6 = plVar7;
        if ((long *)param_1[1] <= plVar7) goto LAB_0010db1b;
      }
      plVar6 = plVar6 + 2;
    } while (plVar6 < (long *)param_1[1]);
  }
LAB_0010db1b:
  param_1[3] = 0;
  param_1[4] = 0;
  return;
}




// Function: hash_free @ 0xdb30

void hash_free(ulong *param_1)

{
  void *pvVar1;
  void *pvVar2;
  long *plVar3;
  long *plVar4;
  long *plVar5;
  long lVar6;
  
  plVar5 = (long *)*param_1;
  plVar3 = (long *)param_1[1];
  if ((param_1[8] != 0) && (param_1[4] != 0)) {
    if (plVar3 <= plVar5) goto LAB_0010dbdc;
    do {
      while (lVar6 = *plVar5, plVar4 = plVar5, lVar6 == 0) {
        plVar5 = plVar5 + 2;
        if (plVar3 <= plVar5) goto LAB_0010db9f;
      }
      while( true ) {
        (*(code *)param_1[8])(lVar6);
        plVar4 = (long *)plVar4[1];
        if (plVar4 == (long *)0x0) break;
        lVar6 = *plVar4;
      }
      plVar3 = (long *)param_1[1];
      plVar5 = plVar5 + 2;
    } while (plVar5 < plVar3);
LAB_0010db9f:
    plVar5 = (long *)*param_1;
  }
  if (plVar5 < plVar3) {
    do {
      pvVar2 = (void *)plVar5[1];
      while (pvVar2 != (void *)0x0) {
        pvVar1 = *(void **)((long)pvVar2 + 8);
        free(pvVar2);
        pvVar2 = pvVar1;
      }
      plVar5 = plVar5 + 2;
    } while (plVar5 < (long *)param_1[1]);
  }
LAB_0010dbdc:
  pvVar2 = (void *)param_1[9];
  while (pvVar2 != (void *)0x0) {
    pvVar1 = *(void **)((long)pvVar2 + 8);
    free(pvVar2);
    pvVar2 = pvVar1;
  }
  free((void *)*param_1);
  free(param_1);
  return;
}




// Function: hash_rehash @ 0xdc20

undefined4 hash_rehash(undefined8 *param_1,undefined8 param_2)

{
  char cVar1;
  undefined4 uVar2;
  size_t __nmemb;
  long in_FS_OFFSET;
  void *local_88;
  void *local_80;
  size_t local_78;
  undefined8 local_70;
  undefined8 local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined8 local_40;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  __nmemb = FUN_0010d460(*(undefined4 *)(param_1[5] + 8),param_2,*(undefined1 *)(param_1[5] + 0x10))
  ;
  if (__nmemb != 0) {
    if (param_1[2] == __nmemb) {
      uVar2 = 1;
      goto LAB_0010dd26;
    }
    local_88 = calloc(__nmemb,0x10);
    if (local_88 != (void *)0x0) {
      local_80 = (void *)((long)local_88 + __nmemb * 0x10);
      local_70 = 0;
      local_60 = param_1[5];
      local_68 = 0;
      local_58 = param_1[6];
      local_50 = param_1[7];
      local_48 = param_1[8];
      local_40 = param_1[9];
      local_78 = __nmemb;
      uVar2 = FUN_0010d330(&local_88,param_1,0);
      if ((char)uVar2 == '\0') {
        param_1[9] = local_40;
        cVar1 = FUN_0010d330(param_1,&local_88,1);
        if (cVar1 != '\0') {
          cVar1 = FUN_0010d330(param_1,&local_88,0);
          if (cVar1 != '\0') {
            free(local_88);
            goto LAB_0010dd26;
          }
        }
                    /* WARNING: Subroutine does not return */
        abort();
      }
      free((void *)*param_1);
      *param_1 = local_88;
      param_1[1] = local_80;
      param_1[2] = local_78;
      param_1[3] = local_70;
      param_1[9] = local_40;
      goto LAB_0010dd26;
    }
  }
  uVar2 = 0;
LAB_0010dd26:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_insert_if_absent @ 0xdda0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 hash_insert_if_absent(long param_1,long param_2,long *param_3)

{
  char cVar1;
  long lVar2;
  long *plVar3;
  ulong uVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  float fVar6;
  undefined1 auVar7 [16];
  undefined1 auVar8 [16];
  long *local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 == 0) {
LAB_0010df67:
                    /* WARNING: Subroutine does not return */
    abort();
  }
  lVar2 = FUN_0010d200(param_1,param_2,&local_38,0);
  if (lVar2 != 0) {
    uVar5 = 0;
    if (param_3 != (long *)0x0) {
      *param_3 = lVar2;
    }
    goto LAB_0010ddee;
  }
  uVar4 = *(ulong *)(param_1 + 0x18);
  if ((long)uVar4 < 0) {
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)uVar4;
  }
  else {
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)(long)uVar4;
  }
  uVar4 = *(ulong *)(param_1 + 0x10);
  if ((long)uVar4 < 0) {
    auVar7._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar7._0_4_ = (float)uVar4;
  }
  else {
    auVar7._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar7._0_4_ = (float)(long)uVar4;
  }
  if (*(float *)(*(long *)(param_1 + 0x28) + 8) * auVar7._0_4_ < auVar8._0_4_) {
    FUN_0010d120(param_1);
    lVar2 = *(long *)(param_1 + 0x28);
    if (auVar8._0_4_ <= auVar7._0_4_ * *(float *)(lVar2 + 8)) goto LAB_0010de4e;
    fVar6 = auVar7._0_4_ * *(float *)(lVar2 + 0xc);
    if (*(char *)(lVar2 + 0x10) == '\0') {
      fVar6 = fVar6 * *(float *)(lVar2 + 8);
    }
    if (fVar6 < _DAT_00117224) {
      if (_DAT_00117228 <= fVar6) {
        uVar4 = (long)(fVar6 - _DAT_00117228) ^ 0x8000000000000000;
      }
      else {
        uVar4 = (ulong)fVar6;
      }
      cVar1 = FUN_0010dc20(param_1,uVar4);
      if (cVar1 != '\0') {
        lVar2 = FUN_0010d200(param_1,param_2,&local_38,0);
        if (lVar2 != 0) goto LAB_0010df67;
        goto LAB_0010de4e;
      }
    }
  }
  else {
LAB_0010de4e:
    if (*local_38 == 0) {
      *local_38 = param_2;
      uVar5 = 1;
      *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
      *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
      goto LAB_0010ddee;
    }
    plVar3 = (long *)FUN_0010d300(param_1);
    if (plVar3 != (long *)0x0) {
      *plVar3 = param_2;
      uVar5 = 1;
      plVar3[1] = local_38[1];
      local_38[1] = (long)plVar3;
      *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
      goto LAB_0010ddee;
    }
  }
  uVar5 = 0xffffffff;
LAB_0010ddee:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_insert @ 0xdf90

undefined8 hash_insert(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_0010dda0(param_1,param_2,&local_18);
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




// Function: hash_remove @ 0xdff0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

long hash_remove(long param_1,undefined8 param_2)

{
  float *pfVar1;
  void *pvVar2;
  void *__ptr;
  char cVar3;
  long lVar4;
  ulong uVar5;
  ulong uVar6;
  long in_FS_OFFSET;
  float fVar7;
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  long *local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  lVar4 = FUN_0010d200(param_1,param_2,&local_28,1);
  if ((lVar4 == 0) || (*(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + -1, *local_28 != 0))
  goto LAB_0010e033;
  uVar5 = *(long *)(param_1 + 0x18) - 1;
  *(ulong *)(param_1 + 0x18) = uVar5;
  if ((long)uVar5 < 0) {
    uVar6 = *(ulong *)(param_1 + 0x10);
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)uVar5;
    if (-1 < (long)uVar6) goto LAB_0010e080;
LAB_0010e136:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)uVar6;
  }
  else {
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)(long)uVar5;
    uVar6 = *(ulong *)(param_1 + 0x10);
    if ((long)uVar6 < 0) goto LAB_0010e136;
LAB_0010e080:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)(long)uVar6;
  }
  if (auVar9._0_4_ < **(float **)(param_1 + 0x28) * auVar8._0_4_) {
    FUN_0010d120(param_1);
    pfVar1 = *(float **)(param_1 + 0x28);
    if (auVar9._0_4_ < *pfVar1 * auVar8._0_4_) {
      fVar7 = auVar8._0_4_ * pfVar1[1];
      if (*(char *)(pfVar1 + 4) == '\0') {
        fVar7 = fVar7 * pfVar1[2];
      }
      if (_DAT_00117228 <= fVar7) {
        uVar5 = (long)(fVar7 - _DAT_00117228) ^ 0x8000000000000000;
      }
      else {
        uVar5 = (ulong)fVar7;
      }
      cVar3 = FUN_0010dc20(param_1,uVar5);
      if (cVar3 == '\0') {
        __ptr = *(void **)(param_1 + 0x48);
        while (__ptr != (void *)0x0) {
          pvVar2 = *(void **)((long)__ptr + 8);
          free(__ptr);
          __ptr = pvVar2;
        }
        *(undefined8 *)(param_1 + 0x48) = 0;
      }
    }
  }
LAB_0010e033:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar4;
}




// Function: parse_omp_threads @ 0xe7d0

ulong parse_omp_threads(char *param_1)

{
  char cVar1;
  char cVar2;
  ulong uVar3;
  char *pcVar4;
  long in_FS_OFFSET;
  char *local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 != (char *)0x0) {
    cVar1 = *param_1;
    while (cVar1 != '\0') {
      cVar2 = FUN_00113c00((int)cVar1);
      if (cVar2 == '\0') {
        cVar1 = FUN_00113b70((int)cVar1);
        goto joined_r0x0010e82a;
      }
      pcVar4 = param_1 + 1;
      param_1 = param_1 + 1;
      cVar1 = *pcVar4;
    }
    cVar1 = FUN_00113b70(0);
joined_r0x0010e82a:
    uVar3 = 0;
    if (cVar1 == '\0') goto LAB_0010e82c;
    local_28 = (char *)0x0;
    uVar3 = strtoul(param_1,&local_28,10);
    if (local_28 != (char *)0x0) {
      cVar1 = *local_28;
      while (cVar1 != '\0') {
        pcVar4 = local_28 + 1;
        cVar2 = FUN_00113c00((int)cVar1);
        if (cVar2 == '\0') {
          if (cVar1 != ',') goto LAB_0010e8c1;
          break;
        }
        local_28 = pcVar4;
        cVar1 = *pcVar4;
      }
      goto LAB_0010e82c;
    }
  }
LAB_0010e8c1:
  uVar3 = 0;
LAB_0010e82c:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar3;
}




// Function: num_processors_via_affinity_mask @ 0xe8d0

long num_processors_via_affinity_mask(void)

{
  int iVar1;
  long lVar2;
  long in_FS_OFFSET;
  cpu_set_t cStack_98;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = sched_getaffinity(0,0x80,&cStack_98);
  if (iVar1 == 0) {
    iVar1 = __sched_cpucount(0x80,&cStack_98);
    lVar2 = (long)iVar1;
    if (lVar2 != 0) goto LAB_0010e903;
  }
  lVar2 = 0;
LAB_0010e903:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar2;
}




// Function: num_processors_ignoring_omp @ 0xe940

ulong num_processors_ignoring_omp(int param_1)

{
  ulong uVar1;
  ulong uVar2;
  
  if (param_1 == 1) {
    uVar2 = FUN_0010e8d0();
    if (uVar2 != 0) {
      return uVar2;
    }
    uVar2 = sysconf(0x54);
    if (0 < (long)uVar2) {
      return uVar2;
    }
  }
  else {
    uVar1 = sysconf(0x53);
    uVar2 = uVar1;
    if ((uVar1 - 1 < 2) && (uVar2 = FUN_0010e8d0(), uVar2 <= uVar1)) {
      return uVar1;
    }
    if (0 < (long)uVar2) {
      return uVar2;
    }
  }
  return 1;
}




// Function: num_processors @ 0xe9d0

ulong num_processors(undefined8 param_1)

{
  ulong uVar1;
  char *pcVar2;
  ulong uVar3;
  
  uVar3 = 0xffffffffffffffff;
  if ((int)param_1 == 2) {
    pcVar2 = getenv("OMP_NUM_THREADS");
    uVar1 = FUN_0010e7d0(pcVar2);
    pcVar2 = getenv("OMP_THREAD_LIMIT");
    uVar3 = FUN_0010e7d0(pcVar2);
    param_1 = 1;
    if (uVar3 == 0) {
      uVar3 = 0xffffffffffffffff;
    }
    if (uVar1 != 0) {
      if (uVar3 <= uVar1) {
        uVar1 = uVar3;
      }
      return uVar1;
    }
  }
  uVar1 = FUN_0010e940(param_1);
  if (uVar3 <= uVar1) {
    uVar1 = uVar3;
  }
  return uVar1;
}




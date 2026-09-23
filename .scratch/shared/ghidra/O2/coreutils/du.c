// Function: main @ 0x3c50

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

byte main(uint param_1,undefined8 *param_2)

{
  ulong uVar1;
  ulong uVar2;
  char cVar3;
  int iVar4;
  char *pcVar5;
  long lVar6;
  int *piVar7;
  undefined8 *puVar8;
  ulong *puVar9;
  undefined8 uVar10;
  int *piVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  long lVar14;
  undefined8 extraout_RDX;
  ulong uVar15;
  ulong uVar16;
  undefined8 extraout_RDX_00;
  undefined8 extraout_RDX_01;
  undefined8 extraout_RDX_02;
  undefined8 extraout_RDX_03;
  undefined8 extraout_RDX_04;
  undefined8 extraout_RDX_05;
  byte bVar17;
  ulong uVar18;
  long lVar19;
  ulong uVar20;
  ulong uVar21;
  undefined **ppuVar22;
  byte bVar23;
  long in_FS_OFFSET;
  bool bVar24;
  bool local_160;
  undefined **local_158;
  undefined8 local_150;
  ulong local_148;
  int *local_140;
  long local_138;
  bool local_125;
  uint local_124;
  stat *local_120;
  int local_10c;
  ulong local_108;
  ulong local_100;
  ulong local_f8;
  ulong uStack_f0;
  undefined *local_e8;
  undefined8 local_e0;
  stat local_d8;
  long local_40;
  
  bVar23 = 8;
  bVar17 = 0x60;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_e0 = 0;
  local_e8 = &DAT_001159ef;
  FUN_0010eb20(*param_2);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  ppuVar22 = &PTR_DAT_0011a700;
  textdomain("coreutils");
  FUN_00113be0(FUN_00105f80);
  DAT_0011b140 = FUN_00106800();
  pcVar5 = getenv("DU_BLOCK_SIZE");
  FUN_0010d520(pcVar5,&DAT_0011b170,&DAT_0011b148);
  local_140 = (int *)((ulong)local_140 & 0xffffffffffffff00);
  local_150 = 0x10;
  local_148 = CONCAT44(local_148._4_4_,8);
  local_158 = (undefined **)0x0;
  local_160 = true;
  local_125 = false;
LAB_00103d50:
  do {
    uVar16 = (ulong)param_1;
    local_10c = -1;
    iVar4 = getopt_long(uVar16,param_2,"0abd:chHklmst:xB:DLPSX:",&PTR_DAT_0011a700,&local_10c);
    if (iVar4 == -1) {
      if (local_160 != false) {
        uVar12 = extraout_RDX;
        if (DAT_0011b186 == '\0') {
          if (local_125 == false) {
            if ((char)local_140 != '\0') goto LAB_00104bdf;
          }
          else if ((char)local_140 != '\0') {
            if (DAT_0011b010 != 0) {
LAB_0010445d:
              uVar16 = DAT_0011b010;
              uVar12 = dcgettext(0,"warning: summarizing conflicts with --max-depth=%lu",5);
              error(0,0,uVar12,uVar16);
              FUN_00105540(1);
              piVar7 = &local_10c;
              goto LAB_00104490;
            }
            uVar12 = dcgettext(0,"warning: summarizing is the same as using --max-depth=0",5);
            error(0,0,uVar12);
            uVar12 = extraout_RDX_00;
            if (DAT_0011b010 != 0) goto LAB_0010445d;
LAB_00104bdf:
            DAT_0011b010 = 0;
          }
LAB_00104163:
          if (DAT_0011b16d != '\0') {
            if (DAT_0011b185 != '\0') {
              uVar12 = dcgettext(0,
                                 "warning: options --apparent-size and -b are ineffective with --inodes"
                                 ,5);
              error(0,0,uVar12);
              uVar12 = extraout_RDX_03;
            }
            DAT_0011b148 = 1;
          }
          if (DAT_0011b16c == '\0') goto LAB_001041b8;
          pcVar5 = DAT_0011b160;
          if ((undefined **)DAT_0011b160 != (undefined **)0x0) goto LAB_001041a2;
          ppuVar22 = (undefined **)getenv("TIME_STYLE");
          uVar12 = extraout_RDX_01;
          if ((ppuVar22 == (undefined **)0x0) ||
             (DAT_0011b160 = (char *)ppuVar22, iVar4 = strcmp((char *)ppuVar22,"locale"),
             uVar12 = extraout_RDX_02, iVar4 == 0)) {
            pcVar5 = "long-iso";
            DAT_0011b160 = "long-iso";
            goto LAB_00104d36;
          }
LAB_00104faa:
          if (*(char *)ppuVar22 != '+') goto LAB_00104fc8;
          pcVar5 = strchr((char *)ppuVar22,10);
          if (pcVar5 == (char *)0x0) goto LAB_001041ad;
          *pcVar5 = '\0';
          uVar12 = extraout_RDX_05;
          pcVar5 = (char *)ppuVar22;
          goto LAB_001041a2;
        }
        if ((char)local_140 == '\0') goto LAB_00104163;
        uVar12 = dcgettext(0,"cannot both summarize and show all entries",5);
        error(0,0,uVar12);
      }
      lVar6 = FUN_00105540(1);
      goto LAB_00105141;
    }
    if (0x87 < iVar4) {
switchD_00103d8f_caseD_31:
      local_160 = false;
      goto LAB_00103d50;
    }
    if (iVar4 < 0x30) {
      if (iVar4 == -0x83) {
        FUN_00111ab0(stdout,&DAT_0011402d,"GNU coreutils",PTR_DAT_0011b018,"Torbjorn Granlund",
                     "David MacKenzie","Paul Eggert","Jim Meyering",0,uVar16);
                    /* WARNING: Subroutine does not return */
        exit(0);
      }
      if (iVar4 == -0x82) {
        FUN_00105540(0);
        goto LAB_00104faa;
      }
      goto switchD_00103d8f_caseD_31;
    }
    switch(iVar4) {
    case 0x30:
      DAT_0011b182 = 1;
      break;
    default:
      goto switchD_00103d8f_caseD_31;
    case 0x44:
    case 0x48:
      local_150 = 0x11;
      break;
    case 0x4c:
      local_150 = 2;
      break;
    case 0x50:
      local_150 = 0x10;
      break;
    case 0x53:
      DAT_0011b180 = '\x01';
      break;
    case 0x58:
      iVar4 = FUN_001070c0(FUN_00106b70,DAT_0011b140,optarg,0x10000000,10);
      if (iVar4 != 0) {
        uVar12 = FUN_00110b80(0,3,optarg);
        piVar7 = __errno_location();
        error(0,*piVar7,&DAT_0011575f,uVar12);
        local_160 = false;
      }
      break;
    case 0x61:
      DAT_0011b186 = '\x01';
      break;
    case 0x62:
      DAT_0011b185 = '\x01';
      DAT_0011b170 = 0;
      DAT_0011b148 = 1;
      break;
    case 99:
      DAT_0011b181 = '\x01';
      break;
    case 100:
      iVar4 = FUN_00112910(optarg,0,0,&local_108,"");
      if (iVar4 == 0) {
        DAT_0011b010 = local_108;
        local_125 = true;
      }
      else {
        uVar12 = FUN_00110ef0(optarg);
        uVar13 = dcgettext(0,"invalid maximum depth %s",5);
        error(0,0,uVar13,uVar12);
        local_160 = false;
      }
      break;
    case 0x68:
      DAT_0011b170 = 0xb0;
      DAT_0011b148 = 1;
      break;
    case 0x6b:
      DAT_0011b170 = 0;
      DAT_0011b148 = 0x400;
      break;
    case 0x6c:
      DAT_0011b184 = '\x01';
      break;
    case 0x6d:
      DAT_0011b170 = 0;
      DAT_0011b148 = 0x100000;
      break;
    case 0x73:
      local_140 = (int *)CONCAT71(local_140._1_7_,1);
      break;
    case 0x74:
      iVar4 = FUN_00112250(optarg,0,0,&DAT_0011b178,"kKmMGTPEZY0");
      if (iVar4 != 0) {
                    /* WARNING: Subroutine does not return */
        FUN_00112850(iVar4,local_10c,0x74,&PTR_DAT_0011a700,optarg);
      }
      if ((DAT_0011b178 != 0) || (*(char *)optarg != '-')) break;
      uVar12 = dcgettext(0,"invalid --threshold argument \'-0\'",5);
      error(1,0,uVar12);
    case 0x42:
      iVar4 = FUN_0010d520(optarg,&DAT_0011b170,&DAT_0011b148);
      if (iVar4 != 0) {
                    /* WARNING: Subroutine does not return */
        FUN_00112850(iVar4,local_10c,0x42,&PTR_DAT_0011a700,optarg);
      }
      break;
    case 0x78:
      local_148 = CONCAT44(local_148._4_4_,0x48);
      break;
    case 0x80:
      DAT_0011b185 = '\x01';
      break;
    case 0x81:
      FUN_00106b70(DAT_0011b140,optarg,0x10000000);
      break;
    case 0x82:
      local_158 = optarg;
      break;
    case 0x83:
      DAT_0011b170 = 0x90;
      DAT_0011b148 = 1;
      break;
    case 0x85:
      DAT_0011b16c = '\x01';
      iVar4 = 0;
      if (optarg != (undefined **)0x0) {
        lVar6 = FUN_00105ce0("--time",optarg,&PTR_s_atime_0011a6c0,&DAT_001156f0,4,PTR_FUN_0011b020,
                             1);
        iVar4 = *(int *)(&DAT_001156f0 + lVar6 * 4);
      }
      DAT_0011b168 = iVar4;
      pcVar5 = getenv("TZ");
      DAT_0011b150 = FUN_00110fe0(pcVar5);
      break;
    case 0x86:
      DAT_0011b160 = (char *)optarg;
      break;
    case 0x87:
      DAT_0011b16d = '\x01';
    }
  } while( true );
LAB_00104b60:
  piVar11 = __errno_location();
  if (*piVar11 != 0) {
    bVar23 = 0;
    uVar12 = FUN_00110b80(0,3,*(undefined8 *)(local_148 + 0x20));
    uVar13 = dcgettext(0,"fts_read failed: %s",5);
    error(0,*piVar11,uVar13,uVar12);
  }
  DAT_0011b188 = 0;
  iVar4 = FUN_0010abe0(local_148);
  if (iVar4 != 0) {
    bVar23 = 0;
    uVar12 = dcgettext(0,"fts_close failed",5);
    error(0,*piVar11,uVar12);
  }
  local_125 = (bool)(local_125 & bVar23);
  goto LAB_00104290;
LAB_00104fc8:
  while (pcVar5 = DAT_0011b160, iVar4 = strncmp(DAT_0011b160,"posix-",6), uVar12 = extraout_RDX_04,
        iVar4 == 0) {
    DAT_0011b160 = (char *)((long)pcVar5 + 6);
  }
LAB_001041a2:
  ppuVar22 = (undefined **)pcVar5;
  if (*pcVar5 == '+') {
LAB_001041ad:
    DAT_0011b158 = (char *)((long)ppuVar22 + 1);
  }
  else {
LAB_00104d36:
    lVar6 = FUN_00105ce0("time style",pcVar5,&PTR_s_full_iso_0011a6a0,&DAT_001156e0,4,
                         PTR_FUN_0011b020,1,uVar12);
    iVar4 = *(int *)(&DAT_001156e0 + lVar6 * 4);
    if (iVar4 == 1) {
      DAT_0011b158 = "%Y-%m-%d %H:%M";
    }
    else if (iVar4 == 2) {
      DAT_0011b158 = "%Y-%m-%d";
    }
    else if (iVar4 == 0) {
      DAT_0011b158 = "%Y-%m-%d %H:%M:%S.%N %z";
    }
  }
LAB_001041b8:
  lVar6 = (long)optind;
  if (local_158 != (undefined **)0x0) {
    if (optind < (int)param_1) {
LAB_00105141:
      uVar12 = FUN_00110ef0(param_2[lVar6]);
      uVar13 = dcgettext(0,"extra operand %s",5);
      error(0,0,uVar13,uVar12);
      uVar12 = dcgettext(0,"file operands cannot be combined with --files0-from",5);
      __fprintf_chk(stderr,1,"%s\n",uVar12);
      FUN_00105540(1);
    }
    else {
      iVar4 = strcmp((char *)local_158,"-");
      if ((iVar4 == 0) || (lVar6 = FUN_00108e20(local_158,"r",stdin), lVar6 != 0)) {
        piVar7 = (int *)FUN_00105e40(stdin);
        DAT_0011b183 = local_160;
        goto LAB_00104221;
      }
    }
    uVar12 = FUN_00110800(4,local_158);
    uVar13 = dcgettext(0,"cannot open %s for reading",5);
    piVar7 = __errno_location();
    error(1,*piVar7,uVar13,uVar12);
LAB_001051ec:
                    /* WARNING: Subroutine does not return */
    __assert_fail("e == ent","src/du.c",0x20f,"process_file");
  }
  ppuVar22 = &local_e8;
  if (optind < (int)param_1) {
    ppuVar22 = (undefined **)(param_2 + lVar6);
  }
  piVar7 = (int *)FUN_00105e10(ppuVar22);
  DAT_0011b183 = optind + 1 < (int)param_1 || (uint)local_150 == 2;
LAB_00104221:
  if ((piVar7 != (int *)0x0) && (DAT_0011b198 = FUN_00106130(), DAT_0011b198 != 0)) {
    if ((DAT_0011b184 != '\0') || (DAT_0011b183 == 0)) {
      local_148 = local_148 | 0x100;
    }
    local_124 = (uint)local_148 | (uint)local_150;
    local_125 = local_160;
    local_120 = &local_d8;
    local_140 = &local_10c;
LAB_00104290:
    while (pcVar5 = (char *)FUN_00105e80(piVar7,local_140), pcVar5 != (char *)0x0) {
      if (local_158 == (undefined **)0x0) {
        if (*pcVar5 != '\0') {
LAB_001042e9:
          DAT_0011b110 = pcVar5;
          local_148 = FUN_001121c0(&DAT_0011b110,local_124,0);
          bVar23 = local_160;
LAB_0010430c:
          param_2 = (undefined8 *)FUN_0010add0(local_148);
          if (param_2 == (undefined8 *)0x0) goto LAB_00104b60;
          local_150 = param_2[7];
          param_1 = (uint)*(ushort *)(param_2 + 0xd);
          if (param_1 == 4) {
            uVar12 = FUN_00110800(4,local_150);
            uVar13 = dcgettext(0,"cannot read directory %s",5);
            error(0,*(undefined4 *)(param_2 + 8),uVar13,uVar12);
            bVar17 = 0;
          }
          else {
            bVar17 = local_160;
            if (param_1 == 6) goto LAB_00104510;
            cVar3 = FUN_001069c0(DAT_0011b140,local_150);
            if (cVar3 != '\0') {
LAB_001043a8:
              if (param_1 == 1) {
                FUN_0010b520(local_148,param_2,4);
                puVar8 = (undefined8 *)FUN_0010add0(local_148);
                if (param_2 != puVar8) {
                    /* WARNING: Subroutine does not return */
                  __assert_fail("e == ent","src/du.c",0x230,"process_file");
                }
              }
              goto LAB_0010430c;
            }
            if (param_1 == 0xb) {
              FUN_0010b520(local_148,param_2,1);
              puVar8 = (undefined8 *)FUN_0010add0(local_148);
              if (param_2 != puVar8) goto LAB_001051ec;
              param_1 = (uint)*(ushort *)(param_2 + 0xd);
            }
            if (param_1 == 10 || param_1 == 0xd) {
              bVar23 = 0;
              uVar12 = FUN_00110800(4,local_150);
              uVar13 = dcgettext(0,"cannot access %s",5);
              error(0,*(undefined4 *)(param_2 + 8),uVar13,uVar12);
              goto LAB_0010430c;
            }
            bVar17 = 0;
            if ((((*(byte *)(local_148 + 0x48) & 0x40) != 0) && (0 < (long)param_2[0xb])) &&
               (*(long *)(local_148 + 0x18) != param_2[0xe])) goto LAB_001043a8;
LAB_00104490:
            if ((DAT_0011b184 == '\0') &&
               ((DAT_0011b183 != 0 ||
                (((*(uint *)(param_2 + 0x11) & 0xf000) != 0x4000 && (1 < (ulong)param_2[0x10]))))))
            {
              iVar4 = FUN_001061d0(DAT_0011b198,param_2[0xe],param_2[0xf]);
              if (iVar4 < 0) goto LAB_001050a5;
              if (iVar4 == 0) goto LAB_001043a8;
            }
            if (param_1 == 2) {
              cVar3 = FUN_00112210(local_148,param_2);
              if (cVar3 != '\0') {
                puVar8 = (undefined8 *)*param_2;
                if (DAT_0011b190 == 0) {
                  DAT_0011b190 = FUN_00106130();
                  if (DAT_0011b190 == 0) goto LAB_001050a5;
                  lVar6 = FUN_0010df90(0);
                  while (lVar6 != 0) {
                    if ((((*(byte *)(lVar6 + 0x28) & 3) == 0) &&
                        (iVar4 = stat(*(char **)(lVar6 + 8),local_120), iVar4 == 0)) &&
                       (iVar4 = FUN_001061d0(DAT_0011b190,local_d8.st_dev,local_d8.st_ino),
                       iVar4 < 0)) goto LAB_001050a5;
                    lVar19 = *(long *)(lVar6 + 0x30);
                    FUN_0010e9f0(lVar6);
                    lVar6 = lVar19;
                  }
                }
                do {
                  if (param_2 == puVar8) break;
                  iVar4 = FUN_00106260(DAT_0011b190,param_2[0xe],param_2[0xf]);
                  if (0 < iVar4) goto LAB_0010430c;
                  param_2 = (undefined8 *)param_2[1];
                } while (param_2 != (undefined8 *)0x0);
                bVar23 = 0;
                uVar12 = FUN_00110b80(0,3,local_150);
                uVar13 = dcgettext(0,
                                   "WARNING: Circular directory structure.\nThis almost certainly means that you have a corrupted file system.\nNOTIFY YOUR SYSTEM MANAGER.\nThe following directory is part of the cycle:\n  %s\n"
                                   ,5);
                error(0,0,uVar13,uVar12);
              }
              goto LAB_0010430c;
            }
            if (param_1 == 7) {
              uVar12 = FUN_00110b80(0,3,local_150);
              error(0,*(undefined4 *)(param_2 + 8),&DAT_0011575f,uVar12);
              goto LAB_00104510;
            }
            bVar17 = local_160;
            if (param_1 == 1) goto LAB_0010430c;
          }
LAB_00104510:
          if (DAT_0011b168 == 0) {
            uVar16 = param_2[0x19];
            uVar21 = param_2[0x1a];
          }
          else if (DAT_0011b168 == 2) {
            uVar16 = param_2[0x17];
            uVar21 = param_2[0x18];
          }
          else {
            uVar16 = param_2[0x1b];
            uVar21 = param_2[0x1c];
          }
          if (DAT_0011b185 == '\0') {
            uVar20 = param_2[0x16] << 9;
          }
          else {
            uVar20 = 0;
            if (-1 < (long)param_2[0x14]) {
              uVar20 = param_2[0x14];
            }
          }
          uVar1 = param_2[0xb];
          local_100 = 1;
          local_108 = uVar20;
          local_f8 = uVar16;
          uStack_f0 = uVar21;
          if (DAT_0011b108 == 0) {
            DAT_0011b108 = uVar1 + 10;
            DAT_0011b100 = FUN_00112060(DAT_0011b108,0x40);
          }
          else if (uVar1 != DAT_0011b188) {
            if (DAT_0011b188 < uVar1) {
              if (DAT_0011b108 <= uVar1) {
                DAT_0011b100 = FUN_00111d40(DAT_0011b100,uVar1,0x80);
                DAT_0011b108 = uVar1 * 2;
              }
              uVar15 = DAT_0011b188 + 1;
              if (uVar15 <= uVar1) {
                lVar6 = DAT_0011b188 * 0x40 + DAT_0011b100;
                do {
                  uVar15 = uVar15 + 1;
                  *(undefined8 *)(lVar6 + 0x40) = 0;
                  *(undefined8 *)(lVar6 + 0x48) = 0;
                  *(undefined8 *)(lVar6 + 0x50) = 0x8000000000000000;
                  *(undefined8 *)(lVar6 + 0x58) = 0xffffffffffffffff;
                  *(undefined8 *)(lVar6 + 0x60) = 0;
                  *(undefined8 *)(lVar6 + 0x68) = 0;
                  *(undefined8 *)(lVar6 + 0x70) = 0x8000000000000000;
                  *(undefined8 *)(lVar6 + 0x78) = 0xffffffffffffffff;
                  lVar6 = lVar6 + 0x40;
                } while (uVar15 <= uVar1);
              }
            }
            else {
              if (uVar1 != DAT_0011b188 - 1) {
                    /* WARNING: Subroutine does not return */
                __assert_fail("level == prev_level - 1","src/du.c",0x27e,"process_file");
              }
              puVar9 = (ulong *)(DAT_0011b188 * 0x40 + DAT_0011b100);
              local_108 = uVar20 + *puVar9;
              if (CARRY8(uVar20,*puVar9)) {
                local_108 = 0xffffffffffffffff;
              }
              local_100 = puVar9[1] + 1;
              if ((int)(((uint)((long)puVar9[3] < (long)uVar21) -
                        (uint)((long)uVar21 < (long)puVar9[3])) +
                       ((uint)((long)puVar9[2] < (long)uVar16) -
                       (uint)((long)uVar16 < (long)puVar9[2])) * 2) < 0) {
                local_f8 = puVar9[2];
                uStack_f0 = puVar9[3];
              }
              if (DAT_0011b180 == '\0') {
                bVar24 = CARRY8(local_108,puVar9[4]);
                local_108 = local_108 + puVar9[4];
                if (bVar24) {
                  local_108 = 0xffffffffffffffff;
                }
                local_100 = local_100 + puVar9[5];
                if ((int)(((uint)((long)puVar9[7] < (long)uStack_f0) -
                          (uint)((long)uStack_f0 < (long)puVar9[7])) +
                         ((uint)((long)puVar9[6] < (long)local_f8) -
                         (uint)((long)local_f8 < (long)puVar9[6])) * 2) < 0) {
                  local_f8 = puVar9[6];
                  uStack_f0 = puVar9[7];
                }
              }
              lVar6 = uVar1 * 0x40 + DAT_0011b100;
              uVar15 = *(ulong *)(lVar6 + 0x30);
              uVar18 = 0xffffffffffffffff;
              if (!CARRY8(*puVar9,*(ulong *)(lVar6 + 0x20))) {
                uVar18 = *puVar9 + *(ulong *)(lVar6 + 0x20);
              }
              local_138 = *(long *)(lVar6 + 0x38);
              *(ulong *)(lVar6 + 0x20) = uVar18;
              lVar19 = puVar9[1] + *(long *)(lVar6 + 0x28);
              uVar2 = puVar9[2];
              *(long *)(lVar6 + 0x28) = lVar19;
              if ((int)(((uint)((long)puVar9[3] < local_138) - (uint)(local_138 < (long)puVar9[3]))
                       + ((uint)((long)uVar2 < (long)uVar15) - (uint)((long)uVar15 < (long)uVar2)) *
                         2) < 0) {
                uVar2 = puVar9[3];
                uVar15 = puVar9[2];
                *(ulong *)(lVar6 + 0x30) = puVar9[2];
                *(ulong *)(lVar6 + 0x38) = uVar2;
                local_138 = *(long *)(lVar6 + 0x38);
              }
              lVar14 = uVar18 + puVar9[4];
              if (CARRY8(uVar18,puVar9[4])) {
                lVar14 = -1;
              }
              *(ulong *)(lVar6 + 0x28) = lVar19 + puVar9[5];
              uVar18 = puVar9[6];
              *(long *)(lVar6 + 0x20) = lVar14;
              if ((int)(((uint)((long)puVar9[7] < local_138) - (uint)(local_138 < (long)puVar9[7]))
                       + ((uint)((long)uVar18 < (long)uVar15) - (uint)((long)uVar15 < (long)uVar18))
                         * 2) < 0) {
                uVar15 = puVar9[7];
                *(ulong *)(lVar6 + 0x30) = puVar9[6];
                *(ulong *)(lVar6 + 0x38) = uVar15;
              }
            }
          }
          DAT_0011b188 = uVar1;
          if ((DAT_0011b180 == '\0') || ((param_1 & 0xfffffffd) != 4)) {
            puVar9 = (ulong *)(uVar1 * 0x40 + DAT_0011b100);
            uVar15 = uVar20 + *puVar9;
            if (CARRY8(uVar20,*puVar9)) {
              uVar15 = 0xffffffffffffffff;
            }
            puVar9[1] = puVar9[1] + 1;
            *puVar9 = uVar15;
            if ((int)(((uint)((long)uVar21 < (long)puVar9[3]) -
                      (uint)((long)puVar9[3] < (long)uVar21)) +
                     ((uint)((long)uVar16 < (long)puVar9[2]) -
                     (uint)((long)puVar9[2] < (long)uVar16)) * 2) < 0) {
              puVar9[2] = uVar16;
              puVar9[3] = uVar21;
            }
          }
          bVar24 = CARRY8(uVar20,_DAT_0011b120);
          _DAT_0011b120 = uVar20 + _DAT_0011b120;
          if (bVar24) {
            _DAT_0011b120 = 0xffffffffffffffff;
          }
          _DAT_0011b128 = _DAT_0011b128 + 1;
          if ((int)(((uint)((long)uVar21 < (long)DAT_0011b138) -
                    (uint)((long)DAT_0011b138 < (long)uVar21)) +
                   ((uint)((long)uVar16 < (long)DAT_0011b130) -
                   (uint)((long)DAT_0011b130 < (long)uVar16)) * 2) < 0) {
            DAT_0011b130 = uVar16;
            DAT_0011b138 = uVar21;
          }
          if (((((param_1 & 0xfffffffd) == 4) || (DAT_0011b186 != '\0')) && (uVar1 <= DAT_0011b010))
             || (uVar1 == 0)) {
            uVar16 = local_100;
            if (DAT_0011b16d == '\0') {
              uVar16 = local_108;
            }
            bVar24 = DAT_0011b178 <= uVar16;
            if ((long)DAT_0011b178 < 0) {
              bVar24 = uVar16 <= -DAT_0011b178;
            }
            bVar23 = bVar23 & bVar17;
            if (bVar24) {
              FUN_001053d0(&local_108,local_150);
            }
          }
          else {
            bVar23 = bVar23 & bVar17;
          }
          goto LAB_0010430c;
        }
        uVar12 = dcgettext(0,"invalid zero-length file name",5);
        error(0,0,&DAT_0011575f,uVar12);
        local_125 = false;
      }
      else {
        iVar4 = strcmp((char *)local_158,"-");
        if ((iVar4 == 0) && (iVar4 = strcmp(pcVar5,"-"), iVar4 == 0)) {
          uVar12 = FUN_00110800(4,pcVar5);
          uVar13 = dcgettext(0,"when reading file names from stdin, no file name of %s allowed",5);
          error(0,0,uVar13,uVar12);
          if (*pcVar5 == '\0') goto LAB_00104788;
        }
        else {
          if (*pcVar5 != '\0') goto LAB_001042e9;
LAB_00104788:
          uVar12 = FUN_00105f20(piVar7);
          uVar13 = dcgettext(0,"invalid zero-length file name",5);
          uVar10 = FUN_00110b80(0,3,local_158);
          error(0,0,"%s:%lu: %s",uVar10,uVar12,uVar13);
        }
        local_125 = false;
      }
    }
    if (local_10c != 3) {
      if (local_10c == 4) {
        uVar12 = FUN_00110b80(0,3,local_158);
        uVar13 = dcgettext(0,"%s: read error",5);
        piVar11 = __errno_location();
        error(0,*piVar11,uVar13,uVar12);
        local_125 = false;
      }
      else if (local_10c != 2) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("!\"unexpected error code from argv_iter\"","src/du.c",0x439,"main");
      }
      FUN_00105f40(piVar7);
      FUN_001061a0(DAT_0011b198);
      if (DAT_0011b190 != 0) {
        FUN_001061a0();
      }
      if (((local_158 == (undefined **)0x0) ||
          (((*stdin & 0x20) == 0 && (iVar4 = FUN_001071a0(), iVar4 == 0)))) || (local_125 == false))
      {
        if (DAT_0011b181 != '\0') {
          uVar12 = dcgettext(0,"total",5);
          FUN_001053d0(&DAT_0011b120,uVar12);
        }
        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
          return local_125 ^ 1;
        }
      }
      else {
        uVar12 = FUN_00110800(4,local_158);
        uVar13 = dcgettext(0,"error reading %s",5);
        error(1,0,uVar13,uVar12);
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
  }
LAB_001050a5:
                    /* WARNING: Subroutine does not return */
  FUN_00112180();
}




// Function: print_only_size @ 0x5340

void print_only_size(long param_1)

{
  FILE *__stream;
  char *__s;
  long in_FS_OFFSET;
  undefined1 auStack_2a8 [664];
  long local_10;
  
  __stream = stdout;
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 == -1) {
    __s = (char *)dcgettext(0,"Infinity",5);
  }
  else {
    __s = (char *)FUN_0010c920(param_1,auStack_2a8,DAT_0011b170,1,DAT_0011b148);
  }
  fputs_unlocked(__s,__stream);
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_size @ 0x53d0

void print_size(undefined8 *param_1,undefined8 param_2)

{
  long lVar1;
  char *pcVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  undefined8 local_98;
  ulong uStack_90;
  undefined1 local_88 [64];
  undefined1 local_48 [24];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar3 = param_1[1];
  if (DAT_0011b16d == '\0') {
    uVar3 = *param_1;
  }
  FUN_00105340(uVar3);
  if (DAT_0011b16c != '\0') {
    pcVar2 = stdout->_IO_write_ptr;
    if (pcVar2 < stdout->_IO_write_end) {
      stdout->_IO_write_ptr = pcVar2 + 1;
      *pcVar2 = '\t';
    }
    else {
      __overflow(stdout,9);
    }
    uVar4 = DAT_0011b158;
    uVar3 = DAT_0011b150;
    local_98 = param_1[2];
    uStack_90 = param_1[3];
    lVar1 = FUN_001112f0(DAT_0011b150,&local_98,local_88);
    if (lVar1 == 0) {
      pcVar2 = (char *)FUN_0010d920(local_98,local_48);
      uVar3 = FUN_00110ef0(pcVar2);
      uVar4 = dcgettext(0,"time %s is out of range",5);
      error(0,0,uVar4,uVar3);
      fputs_unlocked(pcVar2,stdout);
    }
    else {
      FUN_00108e00(stdout,uVar4,local_88,uVar3,uStack_90 & 0xffffffff);
    }
  }
  __printf_chk(1,"\t%s%c",param_2,-(DAT_0011b182 == '\0') & 10);
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    fflush_unlocked(stdout);
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: usage @ 0x5540

void usage(int param_1)

{
  FILE *pFVar1;
  int iVar2;
  undefined8 uVar3;
  char *pcVar4;
  undefined8 uVar5;
  undefined **ppuVar6;
  undefined *puVar7;
  undefined *puVar8;
  long in_FS_OFFSET;
  undefined *local_b8;
  char *local_b0;
  char *local_a8 [4];
  char *local_88;
  char *local_80;
  char *local_78;
  char *local_70;
  char *local_68;
  char *local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_40;
  
  uVar5 = DAT_0011b1b0;
  ppuVar6 = &local_b8;
  local_40 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  if (param_1 == 0) {
    uVar3 = dcgettext(0,"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n",5
                     );
    __printf_chk(1,uVar3,uVar5,uVar5);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "Summarize device usage of the set of FILEs, recursively for directories.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "\nMandatory arguments to long options are mandatory for short options too.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -0, --null            end each output line with NUL, not newline\n  -a, --all             write counts for all files, not just directories\n      --apparent-size   print apparent sizes rather than device usage; although\n                          the apparent size is usually smaller, it may be\n                          larger due to holes in (\'sparse\') files, internal\n                          fragmentation, indirect blocks, and the like\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -B, --block-size=SIZE  scale sizes by SIZE before printing them; e.g.,\n                           \'-BM\' prints sizes in units of 1,048,576 bytes;\n                           see SIZE format below\n  -b, --bytes           equivalent to \'--apparent-size --block-size=1\'\n  -c, --total           produce a grand total\n  -D, --dereference-args  dereference only symlinks that are listed on the\n                          command line\n  -d, --max-depth=N     print the total for a directory (or file, with --all)\n                          only if it is N or fewer levels below the command\n                          line argument;  --max-depth=0 is the same as\n                          --summarize\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "      --files0-from=F   summarize device usage of the\n                          NUL-terminated file names specified in file F;\n                          if F is -, then read names from standard input\n  -H                    equivalent to --dereference-args (-D)\n  -h, --human-readable  print sizes in human readable format (e.g., 1K 234M 2G)\n      --inodes          list inode usage information instead of block usage\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -k                    like --block-size=1K\n  -L, --dereference     dereference all symbolic links\n  -l, --count-links     count sizes many times if hard linked\n  -m                    like --block-size=1M\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -P, --no-dereference  don\'t follow any symbolic links (this is the default)\n  -S, --separate-dirs   for directories do not include size of subdirectories\n      --si              like -h, but use powers of 1000 not 1024\n  -s, --summarize       display only a total for each argument\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -t, --threshold=SIZE  exclude entries smaller than SIZE if positive,\n                          or entries greater than SIZE if negative\n      --time            show time of the last modification of any file in the\n                          directory, or any of its subdirectories\n      --time=WORD       show time as WORD instead of modification time:\n                          atime, access, use, ctime or status\n      --time-style=STYLE  show times using STYLE, which can be:\n                            full-iso, long-iso, iso, or +FORMAT;\n                            FORMAT is interpreted like in \'date\'\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -X, --exclude-from=FILE  exclude files that match any pattern in FILE\n      --exclude=PATTERN    exclude files that match PATTERN\n  -x, --one-file-system    skip directories on different file systems\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"      --help        display this help and exit\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"      --version     output version information and exit\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    uVar5 = dcgettext(0,
                      "\nDisplay values are in units of the first available SIZE from --block-size,\nand the %s_BLOCK_SIZE, BLOCK_SIZE and BLOCKSIZE environment variables.\nOtherwise, units default to 1024 bytes (or 512 if POSIXLY_CORRECT is set).\n"
                      ,5);
    __printf_chk(1,uVar5,&DAT_00114035);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "\nThe SIZE argument is an integer and optional unit (example: 10K is 10*1024).\nUnits are K,M,G,T,P,E,Z,Y (powers of 1024) or KB,MB,... (powers of 1000).\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    local_b8 = &DAT_0011402b;
    pcVar4 = "[";
    local_b0 = "test invocation";
    local_a8[0] = "coreutils";
    local_a8[1] = "Multi-call invocation";
    local_88 = "sha256sum";
    local_a8[2] = "sha224sum";
    local_78 = "sha384sum";
    local_a8[3] = "sha2 utilities";
    local_80 = "sha2 utilities";
    local_70 = "sha2 utilities";
    local_68 = "sha512sum";
    local_60 = "sha2 utilities";
    local_58 = 0;
    local_50 = 0;
    do {
      iVar2 = strcmp("du",pcVar4);
      if (iVar2 == 0) break;
      pcVar4 = *(char **)((long)ppuVar6 + 0x10);
      ppuVar6 = (undefined **)((long)ppuVar6 + 0x10);
    } while (pcVar4 != (char *)0x0);
    puVar8 = *(undefined **)((long)ppuVar6 + 8);
    if (puVar8 == (undefined *)0x0) {
      puVar8 = &DAT_0011402d;
    }
    uVar5 = dcgettext(0,"\n%s online help: <%s>\n",5);
    __printf_chk(1,uVar5,"GNU coreutils","https://www.gnu.org/software/coreutils/");
    pcVar4 = setlocale(5,(char *)0x0);
    if (pcVar4 != (char *)0x0) {
      iVar2 = strncmp(pcVar4,"en_",3);
      pFVar1 = stdout;
      if (iVar2 != 0) {
        pcVar4 = (char *)dcgettext(0,
                                   "Report any translation bugs to <https://translationproject.org/team/>\n"
                                   ,5);
        fputs_unlocked(pcVar4,pFVar1);
      }
    }
    puVar7 = &DAT_0011402d;
    iVar2 = strcmp("du","[");
    if (iVar2 == 0) {
      puVar7 = &DAT_00114030;
    }
    uVar5 = dcgettext(0,"Full documentation <%s%s>\n",5);
    pcVar4 = " invocation";
    __printf_chk(1,uVar5,"https://www.gnu.org/software/coreutils/",puVar7);
    if (puVar8 != &DAT_0011402d) {
      pcVar4 = "";
    }
    uVar5 = dcgettext(0,"or available locally via: info \'(coreutils) %s%s\'\n",5);
    __printf_chk(1,uVar5,puVar8,pcVar4);
  }
  else {
    uVar3 = dcgettext(0,"Try \'%s --help\' for more information.\n",5);
    __fprintf_chk(stderr,1,uVar3,uVar5);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: raw_hasher @ 0xb750

ulong raw_hasher(ulong param_1,ulong param_2)

{
  return (param_1 >> 3 | param_1 << 0x3d) % param_2;
}




// Function: raw_comparator @ 0xb770

bool raw_comparator(long param_1,long param_2)

{
  return param_2 == param_1;
}




// Function: check_tuning @ 0xb780

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 check_tuning(long param_1)

{
  float fVar1;
  float *pfVar2;
  float fVar3;
  
  pfVar2 = *(float **)(param_1 + 0x28);
  if (pfVar2 == (float *)&DAT_00115aa0) {
    return 1;
  }
  fVar1 = pfVar2[2];
  if ((((_DAT_00115ab4 < fVar1) && (fVar1 < DAT_00115ab8)) && (_DAT_00115abc < pfVar2[3])) &&
     (_DAT_00115ad0 <= *pfVar2)) {
    fVar3 = *pfVar2 + _DAT_00115ab4;
    if (((fVar3 < pfVar2[1]) && (pfVar2[1] <= DAT_00115ac0)) && (fVar3 < fVar1)) {
      return 1;
    }
  }
  *(undefined **)(param_1 + 0x28) = &DAT_00115aa0;
  return 0;
}




// Function: hash_find_entry @ 0xb810

long hash_find_entry(long *param_1,long param_2,undefined8 *param_3,char param_4)

{
  long *plVar1;
  long lVar2;
  char cVar3;
  ulong uVar4;
  long lVar5;
  long *plVar6;
  
  uVar4 = (*(code *)param_1[6])(param_2,param_1[2]);
  if ((ulong)param_1[2] <= uVar4) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  plVar6 = (long *)(uVar4 * 0x10 + *param_1);
  *param_3 = plVar6;
  lVar5 = *plVar6;
  if (lVar5 == 0) {
LAB_0010b8bf:
    lVar5 = 0;
  }
  else {
    if (lVar5 != param_2) {
      cVar3 = (*(code *)param_1[7])(param_2);
      if (cVar3 == '\0') {
        for (; (long *)plVar6[1] != (long *)0x0; plVar6 = (long *)plVar6[1]) {
          if ((*(long *)plVar6[1] == param_2) ||
             (cVar3 = (*(code *)param_1[7])(param_2), cVar3 != '\0')) {
            plVar1 = (long *)plVar6[1];
            lVar5 = *plVar1;
            if (param_4 == '\0') {
              return lVar5;
            }
            plVar6[1] = plVar1[1];
            *plVar1 = 0;
            plVar1[1] = param_1[9];
            param_1[9] = (long)plVar1;
            return lVar5;
          }
        }
        goto LAB_0010b8bf;
      }
      lVar5 = *plVar6;
    }
    if (param_4 != '\0') {
      plVar1 = (long *)plVar6[1];
      if (plVar1 != (long *)0x0) {
        lVar2 = plVar1[1];
        *plVar6 = *plVar1;
        plVar6[1] = lVar2;
        *plVar1 = 0;
        plVar1[1] = param_1[9];
        param_1[9] = (long)plVar1;
        return lVar5;
      }
      *plVar6 = 0;
    }
  }
  return lVar5;
}




// Function: transfer_entries @ 0xba60

undefined8 transfer_entries(long *param_1,undefined8 *param_2,char param_3)

{
  long lVar1;
  ulong uVar2;
  long *plVar3;
  ulong uVar4;
  long *plVar5;
  long *plVar6;
  long *plVar7;
  long lVar8;
  
  plVar6 = (long *)*param_2;
  if (plVar6 < (long *)param_2[1]) {
    do {
      while (lVar8 = *plVar6, lVar8 != 0) {
        if ((long *)plVar6[1] != (long *)0x0) {
          uVar4 = param_1[2];
          plVar7 = (long *)plVar6[1];
          do {
            lVar8 = *plVar7;
            uVar2 = (*(code *)param_1[6])(lVar8,uVar4);
            uVar4 = param_1[2];
            if (uVar4 <= uVar2) goto LAB_0010bbcb;
            plVar3 = (long *)(uVar2 * 0x10 + *param_1);
            plVar5 = (long *)plVar7[1];
            if (*plVar3 == 0) {
              *plVar3 = lVar8;
              lVar8 = param_1[9];
              param_1[3] = param_1[3] + 1;
              *plVar7 = 0;
              plVar7[1] = lVar8;
              param_1[9] = (long)plVar7;
            }
            else {
              plVar7[1] = plVar3[1];
              plVar3[1] = (long)plVar7;
            }
            plVar7 = plVar5;
          } while (plVar5 != (long *)0x0);
          lVar8 = *plVar6;
        }
        plVar6[1] = 0;
        if (param_3 != '\0') break;
        uVar4 = (*(code *)param_1[6])(lVar8,param_1[2]);
        if ((ulong)param_1[2] <= uVar4) {
LAB_0010bbcb:
                    /* WARNING: Subroutine does not return */
          abort();
        }
        plVar7 = (long *)(uVar4 * 0x10 + *param_1);
        if (*plVar7 == 0) {
          *plVar7 = lVar8;
          param_1[3] = param_1[3] + 1;
        }
        else {
          plVar5 = (long *)param_1[9];
          if (plVar5 == (long *)0x0) {
            plVar5 = malloc(0x10);
            if (plVar5 == (long *)0x0) {
              return 0;
            }
          }
          else {
            param_1[9] = plVar5[1];
          }
          lVar1 = plVar7[1];
          *plVar5 = lVar8;
          plVar5[1] = lVar1;
          plVar7[1] = (long)plVar5;
        }
        *plVar6 = 0;
        plVar6 = plVar6 + 2;
        param_2[3] = param_2[3] + -1;
        if ((long *)param_2[1] <= plVar6) {
          return 1;
        }
      }
      plVar6 = plVar6 + 2;
    } while (plVar6 < (long *)param_2[1]);
  }
  return 1;
}




// Function: hash_print_statistics @ 0xbcd0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void hash_print_statistics(undefined8 *param_1,undefined8 param_2)

{
  ulong uVar1;
  ulong uVar2;
  long lVar3;
  long *plVar4;
  ulong uVar5;
  ulong uVar6;
  
  uVar6 = 0;
  uVar1 = param_1[2];
  uVar2 = param_1[3];
  for (plVar4 = (long *)*param_1; plVar4 < (long *)param_1[1]; plVar4 = plVar4 + 2) {
    while (*plVar4 == 0) {
      plVar4 = plVar4 + 2;
      if ((long *)param_1[1] <= plVar4) goto LAB_0010bd3d;
    }
    uVar5 = 1;
    for (lVar3 = plVar4[1]; lVar3 != 0; lVar3 = *(long *)(lVar3 + 8)) {
      uVar5 = uVar5 + 1;
    }
    if (uVar6 < uVar5) {
      uVar6 = uVar5;
    }
  }
LAB_0010bd3d:
  __fprintf_chk(param_2,1,"# entries:         %lu\n",param_1[4]);
  __fprintf_chk(param_2,1,"# buckets:         %lu\n",uVar1);
  __fprintf_chk(((double)uVar2 * _DAT_00115ad0) / (double)uVar1,param_2,1,
                "# buckets used:    %lu (%.2f%%)\n",uVar2);
  __fprintf_chk(param_2,1,"max bucket length: %lu\n",uVar6);
  return;
}




// Function: hash_lookup @ 0xbe30

long hash_lookup(long *param_1,long param_2)

{
  char cVar1;
  ulong uVar2;
  long *plVar3;
  long lVar4;
  
  uVar2 = (*(code *)param_1[6])(param_2,param_1[2]);
  if ((ulong)param_1[2] <= uVar2) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  plVar3 = (long *)(uVar2 * 0x10 + *param_1);
  lVar4 = *plVar3;
  if (lVar4 != 0) {
    while( true ) {
      if (lVar4 == param_2) {
        return param_2;
      }
      cVar1 = (*(code *)param_1[7])(param_2);
      if (cVar1 != '\0') break;
      plVar3 = (long *)plVar3[1];
      if (plVar3 == (long *)0x0) {
        return 0;
      }
      lVar4 = *plVar3;
    }
    return *plVar3;
  }
  return 0;
}




// Function: hash_get_next @ 0xbef0

long hash_get_next(long *param_1,long param_2)

{
  long lVar1;
  ulong uVar2;
  long *plVar3;
  long *plVar4;
  
  uVar2 = (*(code *)param_1[6])(param_2,param_1[2]);
  if ((ulong)param_1[2] <= uVar2) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  plVar3 = (long *)(uVar2 * 0x10 + *param_1);
  plVar4 = plVar3;
  do {
    lVar1 = *plVar4;
    plVar4 = (long *)plVar4[1];
    if (lVar1 == param_2) {
      if (plVar4 != (long *)0x0) {
        return *plVar4;
      }
      break;
    }
  } while (plVar4 != (long *)0x0);
  do {
    plVar3 = plVar3 + 2;
    if ((long *)param_1[1] <= plVar3) {
      return 0;
    }
  } while (*plVar3 == 0);
  return *plVar3;
}




// Function: hash_do_for_each @ 0xbfd0

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




// Function: hash_string @ 0xc050

ulong hash_string(byte *param_1,ulong param_2)

{
  byte bVar1;
  ulong uVar2;
  
  uVar2 = 0;
  bVar1 = *param_1;
  while (bVar1 != 0) {
    param_1 = param_1 + 1;
    uVar2 = (uVar2 * 0x1f + (ulong)bVar1) % param_2;
    bVar1 = *param_1;
  }
  return uVar2;
}




// Function: hash_initialize @ 0xc0c0

undefined8 *
hash_initialize(undefined8 param_1,undefined *param_2,code *param_3,code *param_4,undefined8 param_5)

{
  char cVar1;
  undefined8 *__ptr;
  size_t __nmemb;
  void *pvVar2;
  
  if (param_3 == (code *)0x0) {
    param_3 = FUN_0010b750;
  }
  if (param_4 == (code *)0x0) {
    param_4 = FUN_0010b770;
  }
  __ptr = malloc(0x50);
  if (__ptr != (undefined8 *)0x0) {
    if (param_2 == (undefined *)0x0) {
      param_2 = &DAT_00115aa0;
    }
    __ptr[5] = param_2;
    cVar1 = FUN_0010b780(__ptr);
    if (cVar1 != '\0') {
      __nmemb = FUN_0010b920(*(undefined4 *)(param_2 + 8),param_1,param_2[0x10]);
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




// Function: hash_clear @ 0xc1d0

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
        if ((long *)param_1[1] <= plVar7) goto LAB_0010c26b;
      }
      plVar6 = plVar6 + 2;
    } while (plVar6 < (long *)param_1[1]);
  }
LAB_0010c26b:
  param_1[3] = 0;
  param_1[4] = 0;
  return;
}




// Function: hash_free @ 0xc280

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
    if (plVar3 <= plVar5) goto LAB_0010c32c;
    do {
      while (lVar6 = *plVar5, plVar4 = plVar5, lVar6 == 0) {
        plVar5 = plVar5 + 2;
        if (plVar3 <= plVar5) goto LAB_0010c2ef;
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
LAB_0010c2ef:
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
LAB_0010c32c:
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




// Function: hash_rehash @ 0xc370

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
  __nmemb = FUN_0010b920(*(undefined4 *)(param_1[5] + 8),param_2,*(undefined1 *)(param_1[5] + 0x10))
  ;
  if (__nmemb != 0) {
    if (param_1[2] == __nmemb) {
      uVar2 = 1;
      goto LAB_0010c476;
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
      uVar2 = FUN_0010ba60(&local_88,param_1,0);
      if ((char)uVar2 == '\0') {
        param_1[9] = local_40;
        cVar1 = FUN_0010ba60(param_1,&local_88,1);
        if (cVar1 != '\0') {
          cVar1 = FUN_0010ba60(param_1,&local_88,0);
          if (cVar1 != '\0') {
            free(local_88);
            goto LAB_0010c476;
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
      goto LAB_0010c476;
    }
  }
  uVar2 = 0;
LAB_0010c476:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_insert_if_absent @ 0xc4f0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 hash_insert_if_absent(long param_1,long param_2,long *param_3)

{
  char cVar1;
  long lVar2;
  ulong uVar3;
  long *plVar4;
  ulong uVar5;
  undefined8 uVar6;
  long in_FS_OFFSET;
  float fVar7;
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  long *local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 == 0) goto LAB_0010c6f3;
  lVar2 = FUN_0010b810(param_1,param_2,&local_38,0);
  if (lVar2 != 0) {
    uVar6 = 0;
    if (param_3 != (long *)0x0) {
      *param_3 = lVar2;
    }
    goto LAB_0010c53e;
  }
  uVar5 = *(ulong *)(param_1 + 0x18);
  if ((long)uVar5 < 0) {
    uVar3 = *(ulong *)(param_1 + 0x10);
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)uVar5;
    if (-1 < (long)uVar3) goto LAB_0010c57f;
LAB_0010c606:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)uVar3;
  }
  else {
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)(long)uVar5;
    uVar3 = *(ulong *)(param_1 + 0x10);
    if ((long)uVar3 < 0) goto LAB_0010c606;
LAB_0010c57f:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)(long)uVar3;
  }
  if (*(float *)(*(long *)(param_1 + 0x28) + 8) * auVar8._0_4_ < auVar9._0_4_) {
    FUN_0010b780(param_1);
    lVar2 = *(long *)(param_1 + 0x28);
    if (auVar9._0_4_ <= auVar8._0_4_ * *(float *)(lVar2 + 8)) goto LAB_0010c59e;
    fVar7 = auVar8._0_4_ * *(float *)(lVar2 + 0xc);
    if (*(char *)(lVar2 + 0x10) == '\0') {
      fVar7 = fVar7 * *(float *)(lVar2 + 8);
    }
    if (fVar7 < _DAT_00115ac4) {
      if (_DAT_00115ac8 <= fVar7) {
        uVar5 = (long)(fVar7 - _DAT_00115ac8) ^ 0x8000000000000000;
      }
      else {
        uVar5 = (ulong)fVar7;
      }
      cVar1 = FUN_0010c370(param_1,uVar5);
      if (cVar1 != '\0') {
        lVar2 = FUN_0010b810(param_1,param_2,&local_38,0);
        if (lVar2 != 0) {
LAB_0010c6f3:
                    /* WARNING: Subroutine does not return */
          abort();
        }
        goto LAB_0010c59e;
      }
    }
LAB_0010c65d:
    uVar6 = 0xffffffff;
  }
  else {
LAB_0010c59e:
    if (*local_38 == 0) {
      *local_38 = param_2;
      uVar6 = 1;
      *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
      *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
      goto LAB_0010c53e;
    }
    plVar4 = *(long **)(param_1 + 0x48);
    if (plVar4 == (long *)0x0) {
      plVar4 = malloc(0x10);
      if (plVar4 == (long *)0x0) goto LAB_0010c65d;
    }
    else {
      *(long *)(param_1 + 0x48) = plVar4[1];
    }
    lVar2 = local_38[1];
    *plVar4 = param_2;
    uVar6 = 1;
    plVar4[1] = lVar2;
    local_38[1] = (long)plVar4;
    *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
  }
LAB_0010c53e:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar6;
}




// Function: hash_insert @ 0xc720

undefined8 hash_insert(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_0010c4f0(param_1,param_2,&local_18);
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




// Function: hash_remove @ 0xc780

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
  lVar4 = FUN_0010b810(param_1,param_2,&local_28,1);
  if ((lVar4 == 0) || (*(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + -1, *local_28 != 0))
  goto LAB_0010c7c3;
  uVar5 = *(long *)(param_1 + 0x18) - 1;
  *(ulong *)(param_1 + 0x18) = uVar5;
  if ((long)uVar5 < 0) {
    uVar6 = *(ulong *)(param_1 + 0x10);
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)uVar5;
    if (-1 < (long)uVar6) goto LAB_0010c810;
LAB_0010c8c6:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)uVar6;
  }
  else {
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)(long)uVar5;
    uVar6 = *(ulong *)(param_1 + 0x10);
    if ((long)uVar6 < 0) goto LAB_0010c8c6;
LAB_0010c810:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)(long)uVar6;
  }
  if (auVar9._0_4_ < **(float **)(param_1 + 0x28) * auVar8._0_4_) {
    FUN_0010b780(param_1);
    pfVar1 = *(float **)(param_1 + 0x28);
    if (auVar9._0_4_ < *pfVar1 * auVar8._0_4_) {
      fVar7 = auVar8._0_4_ * pfVar1[1];
      if (*(char *)(pfVar1 + 4) == '\0') {
        fVar7 = fVar7 * pfVar1[2];
      }
      if (_DAT_00115ac8 <= fVar7) {
        uVar5 = (long)(fVar7 - _DAT_00115ac8) ^ 0x8000000000000000;
      }
      else {
        uVar5 = (ulong)fVar7;
      }
      cVar3 = FUN_0010c370(param_1,uVar5);
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
LAB_0010c7c3:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar4;
}




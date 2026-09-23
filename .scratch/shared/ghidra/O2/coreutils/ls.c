// Function: main @ 0x4d10

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 main(uint param_1,undefined8 *param_2)

{
  undefined8 *puVar1;
  undefined1 auVar2 [16];
  undefined *puVar3;
  FILE *pFVar4;
  undefined *puVar5;
  char cVar6;
  byte bVar7;
  int iVar8;
  int iVar9;
  uint uVar10;
  long *plVar11;
  void *pvVar12;
  long lVar13;
  size_t sVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  void *pvVar17;
  long lVar18;
  undefined **ppuVar19;
  char *pcVar20;
  char *pcVar21;
  ulong uVar22;
  long in_FS_OFFSET;
  bool bVar23;
  uint local_98;
  int local_90;
  int local_8c;
  ulong local_88;
  char *local_80;
  undefined *local_78;
  int local_70;
  char *local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined2 local_43;
  undefined1 local_41;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_001150f0(*param_2);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  DAT_001251f8 = 2;
  FUN_00119d40(FUN_0010f770);
  DAT_001262b8 = 1;
  DAT_00126210 = 0;
  DAT_00126380 = (long *)0x0;
  DAT_00126370 = 0x8000000000000000;
  DAT_00126378 = 0xffffffffffffffff;
  local_88 = 0xffffffffffffffff;
  local_78 = (undefined *)0xffffffffffffffff;
  local_8c = -1;
  local_90 = -1;
  local_70 = -1;
  local_98 = 0xffffffff;
  bVar23 = false;
  local_80 = (char *)0x0;
LAB_00104e20:
  local_58 = (undefined *)CONCAT44(local_58._4_4_,0xffffffff);
  uVar22 = (ulong)param_1;
  iVar8 = getopt_long((ulong)param_1,param_2,"abcdfghiklmnopqrstuvw:xABCDFGHI:LNQRST:UXZ1",
                      &PTR_s_all_0011bf2e_7_00124300,&local_58);
  if (iVar8 != -1) {
    switch(iVar8) {
    case 0x31:
      local_98 = (uint)(local_98 != 0);
      break;
    case 0x41:
      DAT_001262f0 = 1;
      break;
    case 0x42:
      FUN_00106be0(&DAT_0011bc67);
      FUN_00106be0(&DAT_0011bc66);
      break;
    case 0x43:
      local_98 = 2;
      break;
    case 0x44:
      DAT_00126318 = 1;
      break;
    case 0x46:
      if (optarg != (char *)0x0) {
        lVar18 = FUN_0010ecf0("--classify",optarg,&PTR_s_always_0011e647_0xd_001241e0,&DAT_0011a6e0,
                              4,PTR_FUN_001251f0,1);
        if ((*(int *)(&DAT_0011a6e0 + lVar18 * 4) != 1) &&
           ((*(int *)(&DAT_0011a6e0 + lVar18 * 4) != 2 || (cVar6 = FUN_00106d10(), cVar6 == '\0'))))
        break;
      }
      DAT_00126314 = 3;
      break;
    case 0x47:
      DAT_00125028 = 0;
      break;
    case 0x48:
      DAT_001262f8 = 2;
      break;
    case 0x49:
      FUN_00106be0(optarg);
      break;
    case 0x4c:
      DAT_001262f8 = 4;
      break;
    case 0x4e:
      local_90 = 0;
      break;
    case 0x51:
      local_90 = 5;
      break;
    case 0x52:
      DAT_001262f6 = '\x01';
      break;
    case 0x53:
      local_8c = 3;
      break;
    case 0x54:
      uVar15 = dcgettext(0,"invalid tab size",5);
      local_78 = (undefined *)FUN_00118ad0(optarg,0,0,0x7fffffffffffffff,&DAT_0011bb19,uVar15,2);
      break;
    case 0x55:
      local_8c = 6;
      break;
    case 0x58:
      local_8c = 1;
      break;
    case 0x5a:
      DAT_00126365 = '\x01';
      break;
    case 0x61:
      DAT_001262f0 = 2;
      break;
    case 0x62:
      local_90 = 7;
      break;
    case 99:
      DAT_00126334 = 1;
      break;
    case 100:
      DAT_001262f5 = '\x01';
      break;
    case 0x66:
      DAT_001262f0 = 2;
      DAT_00126312 = 0;
      DAT_00126311 = 0;
      uVar10 = 0xffffffff;
      if (local_98 != 0) {
        uVar10 = local_98;
      }
      DAT_0012632c = '\0';
      local_8c = 6;
      local_98 = uVar10;
      break;
    case 0x67:
      DAT_00125029 = 0;
      local_98 = 0;
      break;
    case 0x68:
      DAT_00126328 = 0xb0;
      DAT_0012631c = 0xb0;
      DAT_00126320 = 1;
      DAT_00125020 = 1;
      break;
    case 0x69:
      DAT_001262fc = 1;
      break;
    case 0x6b:
      bVar23 = true;
      break;
    case 0x6c:
      local_98 = 0;
      break;
    case 0x6d:
      local_98 = 4;
      break;
    case 0x6e:
      DAT_0012632d = 1;
      local_98 = 0;
      break;
    case 0x6f:
      DAT_00125028 = 0;
      local_98 = 0;
      break;
    case 0x70:
      DAT_00126314 = 1;
      break;
    case 0x71:
      local_70 = 1;
      break;
    case 0x72:
      DAT_0012632f = 1;
      break;
    case 0x73:
      DAT_0012632c = '\x01';
      break;
    case 0x74:
      local_8c = 5;
      break;
    case 0x75:
      DAT_00126334 = 2;
      break;
    case 0x77:
      local_88 = FUN_00106c10(optarg);
      if (-1 < (long)local_88) break;
      param_2 = (undefined8 *)FUN_001174c0(optarg);
      uVar15 = dcgettext(0,"invalid line width",5);
      error(2,0,"%s: %s",uVar15,param_2);
    case 0x76:
      local_8c = 4;
      break;
    case 0x78:
      local_98 = 3;
      break;
    case 0x80:
      DAT_0012632e = 1;
      break;
    case 0x81:
      iVar8 = FUN_00112150(optarg,&DAT_00126328,&DAT_00126320);
      if (iVar8 != 0) {
                    /* WARNING: Subroutine does not return */
        FUN_00118d20(iVar8,(ulong)local_58 & 0xffffffff,0,&PTR_s_all_0011bf2e_7_00124300,optarg);
      }
      DAT_0012631c = DAT_00126328;
      DAT_00125020 = DAT_00126320;
      break;
    case 0x82:
      if (optarg == (char *)0x0) {
LAB_001050a7:
        bVar7 = 1;
      }
      else {
        lVar18 = FUN_0010ecf0("--color",optarg,&PTR_s_always_0011e647_0xd_001241e0,&DAT_0011a6e0,4,
                              PTR_FUN_001251f0);
        if (*(int *)(&DAT_0011a6e0 + lVar18 * 4) == 1) goto LAB_001050a7;
        bVar7 = 0;
        if (*(int *)(&DAT_0011a6e0 + lVar18 * 4) == 2) {
          bVar7 = FUN_00106d10();
        }
      }
      DAT_00126312 = bVar7 & 1;
      break;
    case 0x83:
      DAT_001262f8 = 3;
      break;
    case 0x84:
      DAT_00126314 = 2;
      break;
    case 0x85:
      lVar18 = FUN_0010ecf0("--format",optarg,&PTR_s_verbose_001242c0,&DAT_0011a750,4,
                            PTR_FUN_001251f0);
      local_98 = *(uint *)(&DAT_0011a750 + lVar18 * 4);
      break;
    case 0x86:
      local_98 = 0;
      local_80 = "full-iso";
      break;
    case 0x87:
      DAT_001262f4 = 1;
      break;
    case 0x88:
      plVar11 = (long *)FUN_00118590(0x10);
      *plVar11 = (long)optarg;
      plVar11[1] = (long)DAT_001262e0;
      DAT_001262e0 = plVar11;
      break;
    case 0x89:
      if (optarg == (char *)0x0) {
LAB_00104f7e:
        bVar7 = 1;
      }
      else {
        lVar18 = FUN_0010ecf0("--hyperlink",optarg,&PTR_s_always_0011e647_0xd_001241e0,&DAT_0011a6e0
                              ,4,PTR_FUN_001251f0,1);
        if (*(int *)(&DAT_0011a6e0 + lVar18 * 4) == 1) goto LAB_00104f7e;
        bVar7 = 0;
        if (*(int *)(&DAT_0011a6e0 + lVar18 * 4) == 2) {
          bVar7 = FUN_00106d10();
        }
      }
      DAT_00126311 = bVar7 & 1;
      break;
    case 0x8a:
      lVar18 = FUN_0010ecf0("--indicator-style",optarg,&PTR_DAT_00124980,&DAT_0011a770,4,
                            PTR_FUN_001251f0,1);
      DAT_00126314 = *(uint *)(&DAT_0011a770 + lVar18 * 4);
      break;
    case 0x8b:
      lVar18 = FUN_0010ecf0("--quoting-style",optarg,&PTR_s_literal_00124a20,&DAT_0011e9c0,4,
                            PTR_FUN_001251f0,1);
      local_90 = *(int *)(&DAT_0011e9c0 + lVar18 * 4);
      break;
    case 0x8c:
      goto switchD_00104e5d_caseD_8c;
    case 0x8d:
      DAT_00126328 = 0x90;
      DAT_0012631c = 0x90;
      DAT_00126320 = 1;
      DAT_00125020 = 1;
      break;
    case 0x8e:
      lVar18 = FUN_0010ecf0("--sort",optarg,&DAT_00124280,&DAT_0011a730,4,PTR_FUN_001251f0,1);
      local_8c = *(int *)(&DAT_0011a730 + lVar18 * 4);
      break;
    case 0x8f:
      lVar18 = FUN_0010ecf0("--time",optarg,&PTR_s_atime_00124240,&DAT_0011a710,4,PTR_FUN_001251f0,1
                           );
      DAT_00126334 = *(int *)(&DAT_0011a710 + lVar18 * 4);
      break;
    case 0x90:
      goto switchD_00104e5d_caseD_90;
    case 0x91:
      DAT_00125019 = '\0';
      DAT_00126312 = 0;
      local_90 = 0;
      local_98 = (uint)(local_98 != 0);
switchD_00104e5d_caseD_8c:
      local_70 = 0;
      break;
    case -0x83:
      goto switchD_00104e5d_caseD_ffffff7d;
    case -0x82:
      FUN_0010df00(0);
      goto LAB_0010661f;
    default:
      goto switchD_00104e5d_caseD_ffffff7f;
    }
    goto LAB_00104e20;
  }
  if (DAT_00126320 == 0) {
    pcVar21 = getenv("LS_BLOCK_SIZE");
    FUN_00112150(pcVar21,&DAT_00126328,&DAT_00126320);
    if ((pcVar21 != (char *)0x0) || (pcVar21 = getenv("BLOCK_SIZE"), pcVar21 != (char *)0x0)) {
      DAT_0012631c = DAT_00126328;
      DAT_00125020 = DAT_00126320;
    }
    if (bVar23) {
      DAT_00126328 = 0;
      DAT_00126320 = 0x400;
    }
  }
  if ((int)local_98 < 0) {
    if (DAT_001251e0 == 1) {
      cVar6 = FUN_00106d10();
      if (cVar6 != '\0') goto LAB_00106386;
LAB_0010661f:
      DAT_00126338 = 1;
    }
    else {
      if (DAT_001251e0 == 2) {
LAB_00106386:
        DAT_00126338 = 2;
        goto LAB_0010556e;
      }
      DAT_00126338 = 0;
    }
LAB_00105af0:
    if (DAT_00126312 != 0) goto LAB_0010556e;
    if (local_88 != 0xffffffffffffffff) goto LAB_0010557a;
LAB_00105b09:
    local_88 = 0x50;
  }
  else {
    DAT_00126338 = local_98;
    if (2 < local_98 - 2) goto LAB_00105af0;
LAB_0010556e:
    if (local_88 == 0xffffffffffffffff) {
      cVar6 = FUN_00106d10();
      if ((cVar6 != '\0') && (iVar8 = ioctl(1,0x5413,&local_58), -1 < iVar8)) {
        local_88 = (ulong)local_58._2_2_;
        if (local_58._2_2_ != 0) goto LAB_0010557a;
      }
      pcVar21 = getenv("COLUMNS");
      if ((pcVar21 != (char *)0x0) && (*pcVar21 != '\0')) {
        local_88 = FUN_00106c10(pcVar21);
        if (-1 < (long)local_88) goto LAB_0010557a;
        uVar15 = FUN_001174c0(pcVar21);
        uVar16 = dcgettext(0,"ignoring invalid width in environment variable COLUMNS: %s",5);
        error(0,0,uVar16,uVar15);
      }
      goto LAB_00105b09;
    }
  }
LAB_0010557a:
  DAT_001262b0 = local_88;
  auVar2._8_8_ = 0;
  auVar2._0_8_ = local_88;
  DAT_00126200 = (ulong)(local_88 !=
                        (SUB168(auVar2 * ZEXT816(0xaaaaaaaaaaaaaaab),8) & 0xfffffffffffffffe) +
                        local_88 / 3) + local_88 / 3;
  puVar5 = DAT_001262c0;
  if ((DAT_00126338 - 2 < 3) && (puVar5 = local_78, (long)local_78 < 0)) {
    DAT_001262c0 = (undefined *)0x8;
    pcVar21 = getenv("TABSIZE");
    puVar5 = DAT_001262c0;
    if ((pcVar21 != (char *)0x0) &&
       (iVar8 = FUN_00118de0(pcVar21,0,0,&local_58,&DAT_0011bb19), puVar5 = local_58, iVar8 != 0)) {
      uVar15 = FUN_001174c0(pcVar21);
      uVar16 = dcgettext(0,"ignoring invalid tab size in environment variable TABSIZE: %s",5);
      error(0,0,uVar16,uVar15);
      puVar5 = DAT_001262c0;
    }
  }
  DAT_001262c0 = puVar5;
  bVar7 = (byte)local_70;
  if ((local_70 == -1) && (bVar7 = 0, DAT_001251e0 == 1)) {
    bVar7 = FUN_00106d10();
  }
  DAT_001262d8 = bVar7 & 1;
  if (local_90 < 0) {
    pcVar21 = getenv("QUOTING_STYLE");
    if (pcVar21 != (char *)0x0) {
      iVar8 = FUN_0010e9c0(pcVar21,&PTR_s_literal_00124a20,&DAT_0011e9c0,4);
      if (iVar8 < 0) {
        uVar15 = FUN_001174c0(pcVar21);
        uVar16 = dcgettext(0,"ignoring invalid value of environment variable QUOTING_STYLE: %s",5);
        error(0,0,uVar16,uVar15);
      }
      else {
        local_90 = *(int *)(&DAT_0011e9c0 + (long)iVar8 * 4);
        if (-1 < local_90) goto LAB_001055eb;
      }
    }
    local_90 = 7;
    if (DAT_001251e0 != 1) goto LAB_001055eb;
    cVar6 = FUN_00106d10();
    if (cVar6 != '\0') {
      local_90 = 3;
      goto LAB_001055eb;
    }
  }
  else {
LAB_001055eb:
    FUN_00116950(0,local_90);
  }
  iVar8 = FUN_00116930(0);
  if (((DAT_00126338 == 0) || ((DAT_00126338 - 2 < 2 && (DAT_001262b0 != 0)))) &&
     ((iVar8 == 3 || ((iVar8 == 6 || (iVar8 == 1)))))) {
    DAT_001263a8 = 1;
    DAT_001262d0 = FUN_001168f0(0);
  }
  else {
    DAT_001263a8 = 0;
    DAT_001262d0 = FUN_001168f0(0);
    if (iVar8 == 7) {
      FUN_00116970(DAT_001262d0,0x20,1);
    }
  }
  if (1 < DAT_00126314) {
    pcVar21 = &DAT_0011bd23 + (DAT_00126314 - 2);
    cVar6 = (&DAT_0011bd23)[DAT_00126314 - 2];
    while (cVar6 != '\0') {
      pcVar21 = pcVar21 + 1;
      FUN_00116970(DAT_001262d0,(int)cVar6,1);
      cVar6 = *pcVar21;
    }
  }
  DAT_001262c8 = FUN_001168f0(0);
  FUN_00116970(DAT_001262c8,0x3a,1);
  DAT_00126318 = (DAT_00126311 ^ 1) & DAT_00126338 == 0 & DAT_00126318;
  if ((int)DAT_00125019 < (int)(uint)DAT_00126318) goto LAB_0010675f;
  if (local_8c < 0) {
    if (DAT_00126338 == 0) {
      DAT_00126330 = 0;
      goto LAB_00105a12;
    }
    if (2 < DAT_00126334 - 1U) {
      local_8c = 0;
      goto LAB_001056f1;
    }
    DAT_00126330 = 5;
  }
  else {
LAB_001056f1:
    DAT_00126330 = local_8c;
    if (DAT_00126338 == 0) {
LAB_00105a12:
      if ((local_80 == (char *)0x0) && (local_80 = getenv("TIME_STYLE"), local_80 == (char *)0x0)) {
        local_80 = "locale";
      }
      while (iVar8 = strncmp(local_80,"posix-",6), iVar8 == 0) {
        cVar6 = FUN_00110260(2);
        if (cVar6 == '\0') goto LAB_001056ff;
        local_80 = local_80 + 6;
      }
      if (*local_80 == '+') {
        local_80 = local_80 + 1;
        pcVar20 = strchr(local_80,10);
        pcVar21 = local_80;
        if (pcVar20 != (char *)0x0) {
          pcVar21 = strchr(pcVar20 + 1,10);
          if (pcVar21 != (char *)0x0) {
            uVar15 = FUN_001174c0(local_80);
            uVar16 = dcgettext(0,"invalid time style format %s",5);
            error(2,0,uVar16,uVar15);
LAB_0010675f:
            uVar15 = dcgettext(0,"--dired and --zero are incompatible",5);
            error(2,0,uVar15);
LAB_00106783:
                    /* WARNING: Subroutine does not return */
            __assert_fail("dev_ino_size <= obstack_object_size (&dev_ino_obstack)","src/ls.c",0x41d,
                          "dev_ino_pop");
          }
          *pcVar20 = '\0';
          pcVar21 = pcVar20 + 1;
        }
      }
      else {
        ppuVar19 = &PTR_s_full_iso_001249c0;
        lVar18 = FUN_0010e9c0(local_80,&PTR_s_full_iso_001249c0,&DAT_0011a780,4);
        if (lVar18 < 0) {
          FUN_0010eb30("time style",local_80,lVar18);
          pFVar4 = stderr;
          pcVar21 = (char *)dcgettext(0,"Valid arguments are:\n",5);
          fputs_unlocked(pcVar21,pFVar4);
          pcVar21 = "full-iso";
          do {
            ppuVar19 = ppuVar19 + 1;
            __fprintf_chk(stderr,1,"  - [posix-]%s\n",pcVar21);
            pFVar4 = stderr;
            pcVar21 = *ppuVar19;
          } while (pcVar21 != (char *)0x0);
          pcVar21 = (char *)dcgettext(0,"  - +FORMAT (e.g., +%H:%M) for a \'date\'-style format\n",5
                                     );
          fputs_unlocked(pcVar21,pFVar4);
switchD_00104e5d_caseD_ffffff7f:
          uVar22 = 2;
          FUN_0010df00();
switchD_00104e5d_caseD_ffffff7d:
          pcVar21 = "ls";
          if ((DAT_001251e0 != 1) && (pcVar21 = "dir", DAT_001251e0 != 2)) {
            pcVar21 = "vdir";
          }
          FUN_001183c0(stdout,pcVar21,"GNU coreutils",PTR_DAT_001251e8,"Richard M. Stallman",
                       "David MacKenzie",0,uVar22);
                    /* WARNING: Subroutine does not return */
          exit(0);
        }
        if (lVar18 == 2) {
          PTR_DAT_00125040 = s__Y__m__d_0011bda9;
          PTR_DAT_00125048 = &DAT_0011bd9d;
          local_80 = PTR_DAT_00125040;
          pcVar21 = PTR_DAT_00125048;
        }
        else {
          local_80 = PTR_DAT_00125040;
          pcVar21 = PTR_DAT_00125048;
          if (lVar18 < 3) {
            if (lVar18 == 0) {
              PTR_DAT_00125048 = s__Y__m__d__H__M__S__N__z_0011bd82;
              PTR_DAT_00125040 = s__Y__m__d__H__M__S__N__z_0011bd82;
              local_80 = PTR_DAT_00125040;
              pcVar21 = PTR_DAT_00125048;
            }
            else if (lVar18 == 1) {
              PTR_DAT_00125048 = &DAT_0011bd9a;
              PTR_DAT_00125040 = &DAT_0011bd9a;
              local_80 = PTR_DAT_00125040;
              pcVar21 = PTR_DAT_00125048;
            }
          }
          else if ((lVar18 == 3) &&
                  (cVar6 = FUN_00110260(2), local_80 = PTR_DAT_00125040, pcVar21 = PTR_DAT_00125048,
                  cVar6 != '\0')) {
            PTR_DAT_00125040 = (undefined *)dcgettext(0,PTR_DAT_00125040,2);
            pcVar21 = (char *)dcgettext(0,PTR_DAT_00125048,2);
            local_80 = PTR_DAT_00125040;
          }
        }
      }
      PTR_DAT_00125048 = pcVar21;
      PTR_DAT_00125040 = local_80;
      FUN_00106d40();
    }
  }
LAB_001056ff:
  iVar8 = optind;
  if (DAT_00126312 == 0) {
LAB_00105713:
    if (DAT_001262f4 != 0) {
LAB_0010571c:
      DAT_001262fd = 1;
    }
  }
  else {
    local_60 = getenv("LS_COLORS");
    if ((local_60 == (char *)0x0) || (*local_60 == '\0')) {
      pcVar21 = getenv("COLORTERM");
      if ((pcVar21 == (char *)0x0) || (*pcVar21 == '\0')) {
        pcVar21 = getenv("TERM");
        if ((pcVar21 != (char *)0x0) && (*pcVar21 != '\0')) {
          pcVar20 = "# Configuration file for dircolors, a utility to help you set the";
          do {
            iVar9 = strncmp(pcVar20,"TERM ",5);
            if ((iVar9 == 0) && (iVar9 = fnmatch(pcVar20 + 5,pcVar21,0), iVar9 == 0))
            goto LAB_001061d7;
            sVar14 = strlen(pcVar20);
            pcVar20 = pcVar20 + sVar14 + 1;
          } while (pcVar20 + -0x11a7a0 < (char *)0x12c8);
        }
        DAT_00126312 = 0;
      }
    }
    else {
      local_43 = 0x3f3f;
      local_41 = 0;
      DAT_00126300 = (undefined *)FUN_00118a70(local_60);
      local_58 = DAT_00126300;
      do {
        while( true ) {
          while (cVar6 = *local_60, cVar6 == '*') {
            pcVar21 = local_60 + 1;
            pvVar17 = (void *)FUN_00118590(0x28);
            pvVar12 = pvVar17;
            *(void **)((long)pvVar17 + 0x20) = DAT_00126308;
            DAT_00126308 = pvVar12;
            *(undefined **)((long)pvVar17 + 8) = local_58;
            local_60 = pcVar21;
            cVar6 = FUN_00106960(&local_58,&local_60,1,pvVar17);
            pcVar21 = local_60;
            if ((cVar6 == '\0') || (pcVar21 = local_60 + 1, *local_60 != '=')) goto LAB_00106174;
            *(undefined **)((long)pvVar17 + 0x18) = local_58;
            local_60 = local_60 + 1;
            cVar6 = FUN_00106960(&local_58,&local_60,0,(long)pvVar17 + 0x10);
            pcVar21 = local_60;
            if (cVar6 == '\0') goto LAB_00106174;
          }
          if (cVar6 != ':') break;
          local_60 = local_60 + 1;
        }
        if (cVar6 == '\0') goto LAB_001061c9;
        local_43 = CONCAT11(local_43._1_1_,cVar6);
        pcVar21 = local_60 + 1;
        if (local_60[1] == '\0') goto LAB_00106174;
        local_43 = CONCAT11(local_60[1],cVar6);
        pcVar20 = local_60 + 2;
        local_60 = local_60 + 3;
        pcVar21 = local_60;
        if (*pcVar20 != '=') goto LAB_00106174;
        lVar18 = 0;
        pcVar21 = "lc";
        while (iVar9 = strcmp((char *)&local_43,pcVar21), iVar9 != 0) {
          lVar18 = lVar18 + 1;
          pcVar21 = (&PTR_DAT_001248a0)[lVar18];
          if (pcVar21 == (char *)0x0) goto LAB_00106141;
        }
        (&PTR_DAT_00125068)[(long)(int)lVar18 * 2] = local_58;
        cVar6 = FUN_00106960(&local_58,&local_60,0);
      } while (cVar6 != '\0');
LAB_00106141:
      uVar15 = FUN_001174c0(&local_43);
      uVar16 = dcgettext(0,"unrecognized prefix: %s",5);
      error(0,0,uVar16,uVar15);
      pcVar21 = local_60;
LAB_00106174:
      local_60 = pcVar21;
      uVar15 = dcgettext(0,"unparsable value for LS_COLORS environment variable",5);
      error(0,0,uVar15);
      free(DAT_00126300);
      pvVar12 = DAT_00126308;
      while (pvVar12 != (void *)0x0) {
        pvVar17 = *(void **)((long)pvVar12 + 0x20);
        free(pvVar12);
        pvVar12 = pvVar17;
      }
      DAT_00126312 = 0;
LAB_001061c9:
      if ((DAT_001250d0 == 6) && (iVar9 = strncmp(PTR_s_01_36_001250d8,"target",6), iVar9 == 0)) {
        DAT_00126390 = '\x01';
      }
    }
LAB_001061d7:
    if (DAT_00126312 == 0) goto LAB_00105713;
    DAT_001262c0 = (undefined *)0x0;
    if ((((DAT_001262f4 != 0) || (cVar6 = FUN_00106c80(0xd), cVar6 != '\0')) ||
        ((cVar6 = FUN_00106c80(0xe), cVar6 != '\0' && (DAT_00126390 != '\0')))) ||
       ((cVar6 = FUN_00106c80(0xc), cVar6 != '\0' && (DAT_00126338 == 0)))) goto LAB_0010571c;
  }
  lVar18 = (long)iVar8;
  if (((DAT_001262f8 == 0) && (DAT_001262f8 = 1, DAT_001262f5 == '\0')) && (DAT_00126314 != 3)) {
    DAT_001262f8 = (-(uint)(DAT_00126338 == 0) & 0xfffffffe) + 3;
  }
  if (DAT_001262f6 != '\0') {
    DAT_001263c8 = FUN_00110c60(0x1e,0,FUN_00106910,FUN_00106920,free);
    if (DAT_001263c8 == 0) {
                    /* WARNING: Subroutine does not return */
      FUN_00118a90();
    }
    _obstack_begin(&DAT_001260e0,0,0,PTR_malloc_00124fe8,PTR_free_00124fb0);
  }
  pcVar21 = getenv("TZ");
  DAT_001262a8 = FUN_00117950(pcVar21);
  puVar3 = PTR_malloc_00124fe8;
  puVar5 = PTR_free_00124fb0;
  if ((((DAT_00126330 - 3U & 0xfffffffd) == 0) || (DAT_00126338 == 0)) ||
     ((DAT_00126365 != '\0' || (DAT_0012632c != '\0')))) {
    DAT_001262a1 = 1;
    bVar7 = 0;
  }
  else {
    DAT_001262a1 = 0;
    bVar7 = 1;
    if (((DAT_001262f6 == '\0') && (DAT_00126312 == 0)) && (DAT_00126314 == 0)) {
      bVar7 = DAT_001262f4;
    }
  }
  DAT_001262a0 = bVar7 & 1;
  if (DAT_00126318 != 0) {
    _obstack_begin(&DAT_001261a0,0,0,PTR_malloc_00124fe8,PTR_free_00124fb0);
    _obstack_begin(&DAT_00126140,0,0,puVar3,puVar5);
  }
  if (DAT_00126311 != 0) {
    uVar22 = 0;
LAB_00105d7f:
    do {
      iVar9 = (int)uVar22;
      if (uVar22 < 0x5b) {
        bVar23 = true;
        if ((iVar9 < 0x41) && (9 < iVar9 - 0x30U)) goto joined_r0x00105d58;
      }
      else {
        bVar23 = true;
        if (0x19 < iVar9 - 0x61U) {
joined_r0x00105d58:
          if ((iVar9 - 0x2dU < 2) || (iVar9 == 0x7e)) {
            (&DAT_00125fe0)[uVar22] = (&DAT_00125fe0)[uVar22] | 1;
            uVar22 = uVar22 + 1;
            if (uVar22 == 0x100) break;
            goto LAB_00105d7f;
          }
          bVar23 = iVar9 == 0x5f;
        }
      }
      (&DAT_00125fe0)[uVar22] = (&DAT_00125fe0)[uVar22] | bVar23;
      uVar22 = uVar22 + 1;
    } while (uVar22 != 0x100);
    DAT_00126388 = (undefined1 *)FUN_00118c10();
    if (DAT_00126388 == (undefined1 *)0x0) {
      DAT_00126388 = &DAT_0011bb19;
    }
  }
  DAT_001263b8 = 100;
  DAT_001263c0 = FUN_001186d0(100,0xd0);
  iVar8 = param_1 - iVar8;
  DAT_001263b0 = 0;
  FUN_00108730();
  if (iVar8 < 1) {
    if (DAT_001262f5 == '\0') {
      FUN_00106fb0(&DAT_0011bdef,0,1);
    }
    else {
      FUN_001096f0(&DAT_0011bdef,3,1,&DAT_0011bb19);
    }
    if (DAT_001263b0 != 0) goto LAB_00105b72;
LAB_00105f36:
    if (DAT_00126380 == (long *)0x0) goto LAB_00105ba8;
    plVar11 = DAT_00126380;
    if (DAT_00126380[3] == 0) {
      DAT_001262b8 = 0;
    }
  }
  else {
    do {
      puVar1 = param_2 + lVar18;
      lVar18 = lVar18 + 1;
      FUN_001096f0(*puVar1,0,1,&DAT_0011bb19);
    } while ((int)lVar18 < (int)param_1);
    if (DAT_001263b0 == 0) {
LAB_0010587e:
      if (1 < iVar8) goto LAB_001058c5;
      goto LAB_00105f36;
    }
LAB_00105b72:
    FUN_00108e30();
    if (DAT_001262f5 == '\0') {
      FUN_00109510(0,1);
    }
    if (DAT_001263b0 == 0) goto LAB_0010587e;
    FUN_0010d420();
    if (DAT_00126380 == (long *)0x0) goto LAB_00105ba8;
    DAT_001261f8 = DAT_001261f8 + 1;
    pcVar21 = stdout->_IO_write_ptr;
    if (stdout->_IO_write_end <= pcVar21) {
      __overflow(stdout,10);
      goto LAB_001058c5;
    }
    stdout->_IO_write_ptr = pcVar21 + 1;
    *pcVar21 = '\n';
    plVar11 = DAT_00126380;
  }
  do {
    DAT_00126380 = (long *)plVar11[3];
    if ((DAT_001263c8 == 0) || (*plVar11 != 0)) {
      FUN_0010d820(*plVar11,plVar11[1],(char)plVar11[2]);
      free((void *)*plVar11);
      free((void *)plVar11[1]);
      free(plVar11);
      DAT_001262b8 = 1;
    }
    else {
      if ((ulong)(DAT_001260f8 - _DAT_001260f0) < 0x10) goto LAB_00106783;
      local_58 = *(undefined **)(DAT_001260f8 + -0x10);
      local_50 = *(undefined8 *)(DAT_001260f8 + -8);
      DAT_001260f8 = DAT_001260f8 + -0x10;
      pvVar12 = (void *)FUN_00111320(DAT_001263c8,&local_58);
      if (pvVar12 == (void *)0x0) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("found","src/ls.c",0x70d,"main");
      }
      free(pvVar12);
      free((void *)*plVar11);
      free((void *)plVar11[1]);
      free(plVar11);
    }
LAB_001058c5:
    plVar11 = DAT_00126380;
  } while (DAT_00126380 != (long *)0x0);
LAB_00105ba8:
  if ((DAT_00126312 != 0) && (DAT_00126310 != '\0')) {
    if ((DAT_00125060 != 2) ||
       (((iVar8 = memcmp(PTR_DAT_00125068,&DAT_0011bdf7,2), iVar8 != 0 || (DAT_00125070 != 1)) ||
        (*PTR_DAT_00125078 != 'm')))) {
      FUN_00107880(&DAT_00125060);
      FUN_00107880(&DAT_00125070);
    }
    fflush_unlocked(stdout);
    FUN_001076b0(0);
    for (iVar8 = DAT_00126214; iVar8 != 0; iVar8 = iVar8 + -1) {
      raise(0x13);
    }
    if (DAT_00126218 != 0) {
      raise(DAT_00126218);
    }
  }
  if (DAT_00126318 != 0) {
    FUN_00107450("//DIRED//",&DAT_001261a0);
    FUN_00107450("//SUBDIRED//",&DAT_00126140);
    uVar10 = FUN_00116930(DAT_001262d0);
    __printf_chk(1,"//DIRED-OPTIONS// --quoting-style=%s\n",(&PTR_s_literal_00124a20)[uVar10]);
  }
  lVar18 = DAT_001263c8;
  if (DAT_001263c8 != 0) {
    lVar13 = FUN_00110790(DAT_001263c8);
    if (lVar13 != 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("hash_get_n_entries (active_dir_set) == 0","src/ls.c",0x741,"main");
    }
    FUN_00110e20(lVar18);
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return DAT_00126210;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
switchD_00104e5d_caseD_90:
  local_80 = optarg;
  goto LAB_00104e20;
}




// Function: dev_ino_hash @ 0x6910

ulong dev_ino_hash(ulong *param_1,ulong param_2)

{
  return *param_1 % param_2;
}




// Function: dev_ino_compare @ 0x6920

undefined8 dev_ino_compare(long *param_1,long *param_2)

{
  if (*param_1 != *param_2) {
    return 0;
  }
  return CONCAT71((int7)((ulong)param_2[1] >> 8),param_1[1] == param_2[1]);
}




// Function: sighandler @ 0x6940

void sighandler(int param_1)

{
  if (DAT_00126218 == 0) {
    DAT_00126218 = param_1;
  }
  return;
}




// Function: get_funky_string @ 0x6960

undefined4 get_funky_string(long *param_1,long *param_2,undefined4 param_3,long *param_4)

{
  byte bVar1;
  char *pcVar2;
  char *pcVar3;
  char cVar4;
  char cVar5;
  long lVar6;
  long lVar7;
  long lVar8;
  long lVar9;
  long lVar10;
  char *pcVar11;
  bool bVar12;
  
  pcVar2 = (char *)*param_2;
  lVar10 = *param_1;
  lVar8 = 1;
  cVar4 = *pcVar2;
  lVar9 = 0;
  lVar7 = lVar10 + 1;
  cVar5 = cVar4 + -0x5c;
  bVar12 = false;
  pcVar3 = pcVar2;
  lVar6 = lVar7;
  if (cVar4 == '\\') goto LAB_001069c6;
LAB_0010699a:
  if (bVar12 || SBORROW1(cVar4,'\\') != cVar5 < '\0') {
    if (cVar4 == '=') {
      if ((char)param_3 != '\0') goto LAB_001069fe;
    }
    else if ((cVar4 < '>') && ((cVar4 == '\0' || (cVar4 == ':')))) {
      param_3 = 1;
LAB_001069fe:
      *param_1 = lVar10;
      *param_2 = (long)pcVar2;
      *param_4 = lVar9;
      return param_3;
    }
  }
  else if (cVar4 == '^') {
    bVar1 = pcVar2[1];
    if ((byte)(bVar1 - 0x40) < 0x3f) {
      pcVar2 = pcVar2 + 2;
      lVar8 = lVar8 + 1;
      lVar7 = lVar6 + 1;
      *(byte *)(lVar6 + -1) = bVar1 & 0x1f;
      goto LAB_001069b6;
    }
    pcVar2 = pcVar2 + 1;
    if (bVar1 == 0x3f) {
      lVar8 = lVar8 + 1;
      lVar7 = lVar6 + 1;
      *(undefined1 *)(lVar6 + -1) = 0x7f;
      goto LAB_001069b6;
    }
    param_3 = 0;
    goto LAB_001069fe;
  }
  pcVar2 = pcVar2 + 1;
  lVar7 = lVar6;
LAB_001069ab:
  *(char *)(lVar7 + -1) = cVar4;
  lVar8 = lVar8 + 1;
  lVar7 = lVar7 + 1;
LAB_001069b6:
  do {
    cVar4 = *pcVar2;
    lVar10 = lVar7 + -1;
    lVar9 = lVar8 + -1;
    cVar5 = cVar4 + -0x5c;
    bVar12 = cVar5 == '\0';
    pcVar3 = pcVar2;
    lVar6 = lVar7;
    if (!bVar12) goto LAB_0010699a;
LAB_001069c6:
    cVar5 = pcVar3[1];
    pcVar2 = pcVar3 + 2;
    if (cVar5 == '\0') {
      param_3 = 0;
      goto LAB_001069fe;
    }
    cVar4 = cVar5 + -0x30;
    switch(cVar4) {
    case '\0':
    case '\x01':
    case '\x02':
    case '\x03':
    case '\x04':
    case '\x05':
    case '\x06':
    case '\a':
      cVar5 = *pcVar2;
      if (7 < (byte)(cVar5 - 0x30U)) goto LAB_001069ab;
      do {
        pcVar2 = pcVar2 + 1;
        cVar4 = cVar5 + -0x30 + cVar4 * '\b';
        cVar5 = *pcVar2;
      } while ((byte)(cVar5 - 0x30U) < 8);
      *(char *)(lVar7 + -1) = cVar4;
      lVar8 = lVar8 + 1;
      lVar7 = lVar7 + 1;
      goto LAB_001069b6;
    default:
      goto switchD_001069eb_caseD_8;
    case '\x0f':
      cVar5 = '\x7f';
      break;
    case '(':
    case 'H':
      goto switchD_001069eb_caseD_28;
    case '/':
      cVar5 = ' ';
      break;
    case '1':
      cVar5 = '\a';
      goto switchD_001069eb_caseD_8;
    case '2':
      cVar5 = '\b';
      goto switchD_001069eb_caseD_8;
    case '5':
      cVar5 = '\x1b';
      break;
    case '6':
      cVar5 = '\f';
      break;
    case '>':
      cVar5 = '\n';
      break;
    case 'B':
      cVar5 = '\r';
      break;
    case 'D':
      cVar5 = '\t';
      break;
    case 'F':
      cVar5 = '\v';
switchD_001069eb_caseD_8:
    }
    *(char *)(lVar7 + -1) = cVar5;
    lVar8 = lVar8 + 1;
    lVar7 = lVar7 + 1;
  } while( true );
switchD_001069eb_caseD_28:
  cVar5 = pcVar3[2];
  pcVar11 = pcVar3 + 3;
  cVar4 = '\0';
  pcVar2 = pcVar3 + 2;
  if ('F' < cVar5) goto LAB_00106b22;
  do {
    if (cVar5 < 'A') {
      if (9 < (byte)(cVar5 - 0x30U)) goto LAB_001069ab;
      cVar4 = cVar5 + -0x30 + cVar4 * '\x10';
      pcVar2 = pcVar11;
    }
    else {
      cVar4 = cVar5 + -0x37 + cVar4 * '\x10';
      pcVar2 = pcVar11;
    }
    while( true ) {
      cVar5 = *pcVar2;
      pcVar11 = pcVar2 + 1;
      if (cVar5 < 'G') break;
LAB_00106b22:
      if (5 < (byte)(cVar5 + 0x9fU)) goto LAB_001069ab;
      cVar4 = cVar5 + -0x57 + cVar4 * '\x10';
      pcVar2 = pcVar11;
    }
  } while( true );
}




// Function: add_ignore_pattern @ 0x6be0

void add_ignore_pattern(undefined8 param_1)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_00118590(0x10);
  *puVar1 = param_1;
  puVar1[1] = DAT_001262e8;
  DAT_001262e8 = puVar1;
  return;
}




// Function: decode_line_length @ 0x6c10

long decode_line_length(undefined8 param_1)

{
  int iVar1;
  long in_FS_OFFSET;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_00118de0(param_1,0,0,&local_18,&DAT_0011bb19);
  if (iVar1 == 0) {
    if (local_18 < 0) {
      local_18 = 0;
    }
  }
  else {
    local_18 = -(ulong)(iVar1 != 1);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_18;
}




// Function: is_colored @ 0x6c80

ulong is_colored(uint param_1)

{
  long lVar1;
  int iVar2;
  undefined8 uVar3;
  
  uVar3 = 0;
  lVar1 = (&DAT_00125060)[(ulong)param_1 * 2];
  if (lVar1 == 0) {
    return 0;
  }
  if (lVar1 != 1) {
    uVar3 = 0;
    if (lVar1 != 2) {
      return 1;
    }
    iVar2 = strncmp((&PTR_DAT_00125068)[(ulong)param_1 * 2],"00",2);
    return CONCAT71((int7)((ulong)uVar3 >> 8),iVar2 != 0) & 0xffffffff;
  }
  iVar2 = strncmp((&PTR_DAT_00125068)[(ulong)param_1 * 2],"0",1);
  return CONCAT71((int7)((ulong)uVar3 >> 8),iVar2 != 0) & 0xffffffff;
}




// Function: stdout_isatty @ 0x6d10

uint stdout_isatty(void)

{
  uint uVar1;
  
  if (-1 < (char)DAT_00125018) {
    return DAT_00125018 & 1;
  }
  uVar1 = isatty(1);
  DAT_00125018 = (char)uVar1;
  return uVar1 & 1;
}




// Function: abformat_init @ 0x6d40

void abformat_init(void)

{
  undefined *puVar1;
  bool bVar2;
  char cVar3;
  uint uVar4;
  char *pcVar5;
  byte *__s;
  ushort **ppuVar6;
  ulong uVar7;
  char cVar8;
  undefined1 *puVar9;
  ulong uVar10;
  long lVar11;
  int __item;
  ulong uVar12;
  long in_FS_OFFSET;
  long local_678;
  ulong local_660;
  long local_658 [2];
  undefined1 local_648 [1536];
  undefined1 local_48 [8];
  long local_40;
  
  lVar11 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  do {
    pcVar5 = *(char **)((long)&PTR_DAT_00125040 + lVar11);
    cVar3 = *pcVar5;
    while (cVar3 != '\0') {
      cVar8 = pcVar5[1];
      if (cVar3 == '%') {
        if (cVar8 == '%') {
          cVar8 = pcVar5[2];
          pcVar5 = pcVar5 + 1;
        }
        else if (cVar8 == 'b') goto LAB_00106dae;
      }
      pcVar5 = pcVar5 + 1;
      cVar3 = cVar8;
    }
    pcVar5 = (char *)0x0;
LAB_00106dae:
    *(char **)((long)local_658 + lVar11) = pcVar5;
    lVar11 = lVar11 + 8;
  } while (lVar11 != 0x10);
  if ((local_658[0] != 0) || (local_658[1] != 0)) {
    uVar12 = 0xc;
    do {
      __item = 0x2000e;
      uVar10 = 0;
      puVar9 = local_648;
      do {
        local_660 = uVar12;
        __s = (byte *)nl_langinfo(__item);
        pcVar5 = strchr((char *)__s,0x25);
        if (pcVar5 != (char *)0x0) goto LAB_00106e94;
        ppuVar6 = __ctype_b_loc();
        uVar7 = FUN_00112810(__s,puVar9,0x80,&local_660,(*ppuVar6)[*__s] >> 0xb & 1,0);
        if (0x7f < uVar7) goto LAB_00106e94;
        if (uVar10 < local_660) {
          uVar10 = local_660;
        }
        __item = __item + 1;
        puVar9 = puVar9 + 0x80;
      } while (puVar9 != local_48);
      bVar2 = uVar10 < uVar12;
      uVar12 = uVar10;
    } while (bVar2);
    local_678 = 0;
    while( true ) {
      puVar1 = (&PTR_DAT_00125040)[local_678];
      lVar11 = local_658[local_678];
      pcVar5 = &DAT_001253e0 + local_678 * 0x600;
      puVar9 = local_648;
      do {
        if (lVar11 == 0) {
          uVar4 = snprintf(pcVar5,0x80,"%s",puVar1);
        }
        else {
          if (0x80 < lVar11 - (long)puVar1) goto LAB_00106e94;
          uVar4 = __snprintf_chk(pcVar5,0x80,1,0xffffffffffffffff,"%.*s%s%s",lVar11 - (long)puVar1,
                                 puVar1,puVar9,lVar11 + 2);
        }
        if (0x7f < uVar4) goto LAB_00106e94;
        puVar9 = puVar9 + 0x80;
        pcVar5 = pcVar5 + 0x80;
      } while (puVar9 != local_48);
      if (local_678 == 1) break;
      local_678 = 1;
    }
    DAT_001253c8 = 1;
  }
LAB_00106e94:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: do_statx @ 0x7050

void do_statx(undefined8 param_1,undefined8 param_2,ulong *param_3,uint param_4,uint param_5)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined1 uStack_128;
  byte local_127;
  uint local_124;
  uint local_118;
  undefined8 local_114;
  ushort local_10c;
  ulong local_108;
  ulong local_100;
  ulong local_f8;
  ulong local_e8;
  uint local_e0;
  ulong local_d8;
  uint local_d0;
  ulong local_c8;
  uint local_c0;
  ulong local_b8;
  uint local_b0;
  uint local_a8;
  uint local_a4;
  uint local_a0;
  uint local_9c;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = statx(param_1,param_2,param_4 | 0x800,param_5,&uStack_128);
  if (-1 < iVar1) {
    *param_3 = ((ulong)local_9c & 0xffffff00) << 0xc |
               ((ulong)local_a0 & 0xfffff000) << 0x20 | (ulong)((local_a0 & 0xfff) << 8) |
               (ulong)(byte)local_9c;
    param_3[2] = (ulong)local_118;
    param_3[1] = local_108;
    *(uint *)(param_3 + 3) = (uint)local_10c;
    *(undefined8 *)((long)param_3 + 0x1c) = local_114;
    param_3[5] = ((ulong)local_a4 & 0xffffff00) << 0xc |
                 ((ulong)local_a8 & 0xfffff000) << 0x20 | (ulong)((local_a8 & 0xfff) << 8) |
                 (ulong)(byte)local_a4;
    param_3[7] = (ulong)local_124;
    param_3[6] = local_100;
    param_3[10] = (ulong)local_e0;
    param_3[8] = local_f8;
    param_3[0xc] = (ulong)local_b0;
    param_3[9] = local_e8;
    param_3[0xe] = (ulong)local_c0;
    param_3[0xb] = local_b8;
    param_3[0xd] = local_c8;
    if ((param_5 & 0x800) != 0) {
      if ((local_127 & 8) == 0) {
        param_3[0xc] = 0xffffffffffffffff;
        param_3[0xb] = 0xffffffffffffffff;
      }
      else {
        param_3[0xb] = local_d8;
        param_3[0xc] = (ulong)local_d0;
      }
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: file_escape @ 0x72a0

byte * file_escape(byte *param_1,char param_2)

{
  byte bVar1;
  byte *pbVar2;
  size_t sVar3;
  byte *pbVar4;
  
  sVar3 = strlen((char *)param_1);
  pbVar4 = (byte *)FUN_001186d0(3,sVar3 + 1);
  bVar1 = *param_1;
  pbVar2 = pbVar4;
  do {
    while( true ) {
      if (bVar1 == 0) {
        *pbVar2 = 0;
        return pbVar4;
      }
      param_1 = param_1 + 1;
      if ((bVar1 != 0x2f) || (param_2 == '\0')) break;
      *pbVar2 = 0x2f;
LAB_001072ef:
      bVar1 = *param_1;
      pbVar2 = pbVar2 + 1;
    }
    if ((&DAT_00125fe0)[bVar1] != '\0') {
      *pbVar2 = bVar1;
      goto LAB_001072ef;
    }
    __sprintf_chk(pbVar2,1,0xffffffffffffffff,"%%%02x",bVar1);
    bVar1 = *param_1;
    pbVar2 = pbVar2 + 3;
  } while( true );
}




// Function: indent @ 0x7360

char * indent(ulong param_1,ulong param_2)

{
  ulong uVar1;
  uint uVar2;
  char *in_RAX;
  char *pcVar3;
  ulong uVar5;
  char *pcVar4;
  
  if (param_2 <= param_1) {
    return in_RAX;
  }
  do {
    while( true ) {
      uVar5 = DAT_001262c0;
      uVar1 = param_1 + 1;
      pcVar3 = stdout->_IO_write_ptr;
      if ((DAT_001262c0 == 0) || (param_2 / DAT_001262c0 <= uVar1 / DAT_001262c0)) break;
      if (pcVar3 < stdout->_IO_write_end) {
        stdout->_IO_write_ptr = pcVar3 + 1;
        *pcVar3 = '\t';
      }
      else {
        __overflow(stdout,9);
        uVar5 = DAT_001262c0;
      }
      pcVar3 = (char *)(param_1 / uVar5);
      param_1 = (uVar5 + param_1) - param_1 % uVar5;
      if (param_2 <= param_1) {
        return pcVar3;
      }
    }
    if (pcVar3 < stdout->_IO_write_end) {
      pcVar4 = pcVar3 + 1;
      stdout->_IO_write_ptr = pcVar4;
      *pcVar3 = ' ';
    }
    else {
      uVar2 = __overflow(stdout,0x20);
      pcVar4 = (char *)(ulong)uVar2;
    }
    param_1 = uVar1;
  } while (uVar1 < param_2);
  return pcVar4;
}




// Function: dired_outbuf @ 0x7430

void dired_outbuf(void *param_1,size_t param_2)

{
  DAT_001261f8 = DAT_001261f8 + param_2;
  fwrite_unlocked(param_1,1,param_2,stdout);
  return;
}




// Function: dired_dump_obstack @ 0x7450

void dired_dump_obstack(char *param_1,long param_2)

{
  undefined8 uVar1;
  char *pcVar2;
  ulong uVar3;
  ulong uVar4;
  undefined8 *puVar5;
  undefined8 *puVar6;
  
  puVar6 = *(undefined8 **)(param_2 + 0x18);
  puVar5 = *(undefined8 **)(param_2 + 0x10);
  if (7 < (ulong)((long)puVar6 - (long)puVar5)) {
    if (puVar6 == puVar5) {
      *(byte *)(param_2 + 0x50) = *(byte *)(param_2 + 0x50) | 2;
    }
    uVar3 = (long)puVar6 + *(ulong *)(param_2 + 0x30) & ~*(ulong *)(param_2 + 0x30);
    uVar4 = *(ulong *)(param_2 + 0x20);
    if (uVar3 - *(long *)(param_2 + 8) <= *(ulong *)(param_2 + 0x20) - *(long *)(param_2 + 8)) {
      uVar4 = uVar3;
    }
    puVar6 = (undefined8 *)(((long)puVar6 - (long)puVar5 & 0xfffffffffffffff8U) + (long)puVar5);
    *(ulong *)(param_2 + 0x18) = uVar4;
    *(ulong *)(param_2 + 0x10) = uVar4;
    fputs_unlocked(param_1,stdout);
    do {
      uVar1 = *puVar5;
      puVar5 = puVar5 + 1;
      __printf_chk(1,&DAT_0011ba7b,uVar1);
    } while (puVar5 != puVar6);
    pcVar2 = stdout->_IO_write_ptr;
    if (stdout->_IO_write_end <= pcVar2) {
      __overflow(stdout,10);
      return;
    }
    stdout->_IO_write_ptr = pcVar2 + 1;
    *pcVar2 = '\n';
  }
  return;
}




// Function: file_failure @ 0x7540

void file_failure(char param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 uVar1;
  int *piVar2;
  
  uVar1 = FUN_00116dd0(4,param_3);
  piVar2 = __errno_location();
  error(0,*piVar2,param_2,uVar1);
  if (param_1 == '\0') {
    if (DAT_00126210 == 0) {
      DAT_00126210 = 1;
      return;
    }
  }
  else {
    DAT_00126210 = 2;
  }
  return;
}




// Function: put_indicator @ 0x7880

void put_indicator(size_t *param_1)

{
  __pid_t _Var1;
  
  if (DAT_00126310 == '\0') {
    DAT_00126310 = '\x01';
    _Var1 = tcgetpgrp(1);
    if (-1 < _Var1) {
      FUN_001076b0(1);
    }
    if (DAT_00125088 == 0) {
      put_indicator(&DAT_00125060);
      put_indicator(&DAT_00125090);
      put_indicator(&DAT_00125070);
    }
    else {
      put_indicator(&DAT_00125080);
    }
  }
  fwrite_unlocked((void *)param_1[1],*param_1,1,stdout);
  return;
}




// Function: quote_name_width @ 0x7d70

long quote_name_width(undefined1 *param_1,undefined8 param_2,undefined4 param_3)

{
  long in_FS_OFFSET;
  byte local_2039;
  undefined1 *local_2038;
  long local_2030;
  undefined1 local_2028 [8200];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_2038 = local_2028;
  FUN_00107920(&local_2038,param_1,param_2,param_3,&local_2030,&local_2039);
  if ((local_2038 != local_2028) && (local_2038 != param_1)) {
    free(local_2038);
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return (ulong)local_2039 + local_2030;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: length_of_file_name_and_frills @ 0x7e10

long length_of_file_name_and_frills(undefined8 *param_1)

{
  char cVar1;
  size_t sVar2;
  long lVar3;
  char *pcVar4;
  long lVar5;
  long in_FS_OFFSET;
  undefined1 auStack_2b8 [664];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_001262fc == '\0') {
    lVar5 = 0;
LAB_00107ee2:
    if (DAT_0012632c != '\0') {
      if (DAT_00126338 != 4) goto LAB_00107e61;
      pcVar4 = "?";
      if (*(char *)(param_1 + 0x17) != '\0') {
        pcVar4 = (char *)FUN_00111550(param_1[0xb],auStack_2b8,DAT_00126328,0x200,DAT_00126320);
      }
      sVar2 = strlen(pcVar4);
LAB_00107e6c:
      lVar5 = lVar5 + sVar2 + 1;
    }
    if (DAT_00126365 != '\0') {
      if (DAT_00126338 == 4) {
        sVar2 = strlen((char *)param_1[0x16]);
      }
      else {
LAB_00107f29:
        sVar2 = (size_t)DAT_00126354;
      }
      lVar5 = lVar5 + sVar2 + 1;
      lVar3 = param_1[0x19];
      goto joined_r0x00107f41;
    }
  }
  else {
    if (DAT_00126338 == 4) {
      pcVar4 = (char *)FUN_00112720(param_1[4],auStack_2b8);
      sVar2 = strlen(pcVar4);
      lVar5 = sVar2 + 1;
      goto LAB_00107ee2;
    }
    lVar5 = (long)DAT_00126360 + 1;
    if (DAT_0012632c != '\0') {
LAB_00107e61:
      sVar2 = (size_t)DAT_0012635c;
      goto LAB_00107e6c;
    }
    if (DAT_00126365 != '\0') goto LAB_00107f29;
  }
  lVar3 = param_1[0x19];
joined_r0x00107f41:
  if (lVar3 == 0) {
    lVar3 = FUN_00107d70(*param_1,DAT_001262d0,*(undefined4 *)((long)param_1 + 0xc4));
  }
  lVar5 = lVar5 + lVar3;
  if (DAT_00126314 != 0) {
    cVar1 = FUN_001075b0(*(undefined1 *)(param_1 + 0x17),*(undefined4 *)(param_1 + 6),
                         *(undefined4 *)(param_1 + 0x15));
    lVar5 = (lVar5 + 1) - (ulong)(cVar1 == '\0');
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: calculate_columns @ 0x7ff0

ulong calculate_columns(char param_1)

{
  ulong *puVar1;
  undefined8 *puVar2;
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  ulong uVar5;
  long lVar6;
  undefined8 *puVar7;
  ulong uVar8;
  ulong uVar9;
  long lVar10;
  ulong uVar11;
  ulong uVar12;
  char *pcVar13;
  ulong uVar14;
  ulong uVar15;
  bool bVar16;
  
  uVar15 = DAT_001263b0;
  if (DAT_00126200 == 0) {
    if (DAT_001252e0 < DAT_001263b0) {
LAB_00108035:
      DAT_00126208 = (char *)FUN_00118650(DAT_00126208,uVar15,0x30);
      uVar12 = uVar15 * 2;
      goto LAB_00108054;
    }
LAB_0010827c:
    if (DAT_001263b0 == 0) goto LAB_00108288;
LAB_001080ec:
    uVar12 = DAT_001263b0;
    pcVar13 = DAT_00126208;
    lVar6 = 3;
    uVar14 = 0;
    do {
      puVar2 = *(undefined8 **)(pcVar13 + lVar6 * 8 + -8);
      pcVar13[lVar6 * 8 + -0x18] = '\x01';
      *(long *)(pcVar13 + lVar6 * 8 + -0x10) = lVar6;
      puVar7 = puVar2;
      do {
        *puVar7 = 3;
        bVar16 = puVar2 + uVar14 != puVar7;
        puVar7 = puVar7 + 1;
      } while (bVar16);
      uVar14 = uVar14 + 1;
      lVar6 = lVar6 + 3;
    } while (uVar14 < uVar15);
    if (uVar12 == 0) goto LAB_0010823d;
  }
  else {
    if (DAT_00126200 < DAT_001263b0) {
      uVar15 = DAT_00126200;
      if (DAT_00126200 <= DAT_001252e0) goto LAB_001080ec;
      if (DAT_00126200 < DAT_00126200 >> 1) goto LAB_00108035;
    }
    else {
      if (DAT_001263b0 <= DAT_001252e0) goto LAB_0010827c;
      if (DAT_001263b0 < DAT_00126200 >> 1) goto LAB_00108035;
    }
    DAT_00126208 = (char *)FUN_00118650(DAT_00126208,DAT_00126200,0x18);
    uVar12 = DAT_00126200;
LAB_00108054:
    auVar3._8_8_ = 0;
    auVar3._0_8_ = uVar12 - DAT_001252e0;
    auVar4._8_8_ = 0;
    auVar4._0_8_ = DAT_001252e0 + 1 + uVar12;
    if ((CARRY8(DAT_001252e0 + 1,uVar12)) || (SUB168(auVar3 * auVar4,8) != 0)) {
                    /* WARNING: Subroutine does not return */
      FUN_00118a90();
    }
    lVar6 = FUN_001186d0(SUB168(auVar3 * auVar4,0) >> 1,8);
    pcVar13 = DAT_00126208;
    if (DAT_001252e0 < uVar12) {
      lVar10 = DAT_001252e0 * 8 + 8;
      do {
        *(long *)(pcVar13 + lVar10 * 3 + -8) = lVar6;
        lVar6 = lVar6 + lVar10;
        lVar10 = lVar10 + 8;
      } while (uVar12 * 8 + 8 != lVar10);
    }
    DAT_001252e0 = uVar12;
    if (uVar15 != 0) goto LAB_001080ec;
LAB_00108288:
    if (DAT_001263b0 == 0) {
      return uVar15;
    }
  }
  uVar12 = 0;
  do {
    lVar6 = FUN_00107e10(*(undefined8 *)(DAT_001263a0 + uVar12 * 8));
    uVar5 = DAT_001263b0;
    uVar14 = DAT_001262b0;
    if (uVar15 != 0) {
      uVar8 = 0;
      pcVar13 = DAT_00126208;
      do {
        uVar9 = uVar8 + 1;
        if (*pcVar13 != '\0') {
          if (param_1 == '\0') {
            uVar11 = uVar12 % uVar9;
          }
          else {
            uVar11 = uVar12 / (((uVar5 - 1) + uVar9) / uVar9);
          }
          puVar1 = (ulong *)(*(long *)(pcVar13 + 0x10) + uVar11 * 8);
          uVar8 = lVar6 + (ulong)(uVar11 != uVar8) * 2;
          uVar11 = *puVar1;
          if (uVar11 < uVar8) {
            *(ulong *)(pcVar13 + 8) = *(long *)(pcVar13 + 8) + (uVar8 - uVar11);
            *puVar1 = uVar8;
            *pcVar13 = *(ulong *)(pcVar13 + 8) < uVar14;
          }
        }
        pcVar13 = pcVar13 + 0x18;
        uVar8 = uVar9;
      } while (uVar15 != uVar9);
    }
    uVar12 = uVar12 + 1;
  } while (uVar12 < uVar5);
LAB_0010823d:
  if (1 < uVar15) {
    pcVar13 = DAT_00126208 + uVar15 * 0x18 + -0x18;
    uVar12 = uVar15;
    do {
      if (*pcVar13 != '\0') {
        return uVar12;
      }
      uVar12 = uVar12 - 1;
      pcVar13 = pcVar13 + -0x18;
      uVar15 = 1;
    } while (uVar12 != 1);
  }
  return uVar15;
}




// Function: process_signals @ 0x84f0

void process_signals(void)

{
  int __sig;
  long in_FS_OFFSET;
  sigset_t sStack_b8;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  while ((DAT_00126218 != 0 || (DAT_00126214 != 0))) {
    if (DAT_00126310 != '\0') {
      FUN_00107880(&DAT_00125060);
      FUN_00107880(&DAT_00125070);
    }
    fflush_unlocked(stdout);
    sigprocmask(0,(sigset_t *)&DAT_00126220,&sStack_b8);
    __sig = DAT_00126218;
    if (DAT_00126214 == 0) {
      signal(DAT_00126218,(__sighandler_t)0x0);
    }
    else {
      DAT_00126214 = DAT_00126214 + -1;
      __sig = 0x13;
    }
    raise(__sig);
    sigprocmask(2,&sStack_b8,(sigset_t *)0x0);
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: format_user_width @ 0x85f0

ulong format_user_width(undefined4 param_1)

{
  uint uVar1;
  ulong uVar2;
  long lVar3;
  
  if (DAT_0012632d == '\0') {
    lVar3 = FUN_00112340();
    if (lVar3 != 0) {
      uVar1 = FUN_00112ec0(lVar3,0);
      uVar2 = (ulong)uVar1;
      if ((int)uVar1 < 0) {
        uVar2 = 0;
      }
      return uVar2;
    }
  }
  uVar2 = __snprintf_chk(0,0,1,0xffffffffffffffff,&DAT_0011ba82,param_1);
  return uVar2;
}




// Function: calc_req_mask @ 0x8640

uint calc_req_mask(void)

{
  uint uVar1;
  uint uVar2;
  
  uVar1 = (-(uint)(DAT_001262fc == '\0') & 0xffffff00) + 0x102;
  if (DAT_0012632c != '\0') {
    uVar1 = uVar1 | 0x400;
  }
  if (DAT_00126338 == 0) {
    if (DAT_00126334 < 4) {
      uVar2 = *(uint *)(&DAT_0011a640 + (ulong)DAT_00126334 * 4);
      if ((DAT_00125029 == '\0') && (DAT_0012632e == '\0')) {
        uVar1 = uVar1 | uVar2 | 0x204;
      }
      else {
        uVar1 = uVar1 | uVar2 | 0x20c;
      }
      if (DAT_00125028 != '\0') {
        uVar1 = uVar1 | 0x10;
      }
      switch(DAT_00126330) {
      case 0:
      case 1:
      case 2:
      case 4:
      case 6:
        goto switchD_001086c5_caseD_0;
      case 3:
        goto switchD_001086c5_caseD_3;
      case 5:
switchD_001086c5_caseD_5:
        return uVar1 | uVar2;
      }
    }
  }
  else {
    switch(DAT_00126330) {
    case 0:
    case 1:
    case 2:
    case 4:
    case 6:
switchD_001086c5_caseD_0:
      return uVar1;
    case 3:
switchD_001086c5_caseD_3:
      return uVar1 | 0x200;
    case 5:
      if (DAT_00126334 < 4) {
        uVar2 = *(uint *)(&DAT_0011a640 + (ulong)DAT_00126334 * 4);
        goto switchD_001086c5_caseD_5;
      }
    }
  }
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: clear_files @ 0x8730

void clear_files(void)

{
  undefined8 *puVar1;
  ulong uVar2;
  
  if (DAT_001263b0 != 0) {
    uVar2 = 0;
    do {
      puVar1 = *(undefined8 **)(DAT_001263a0 + uVar2 * 8);
      free((void *)*puVar1);
      free((void *)puVar1[1]);
      free((void *)puVar1[2]);
      if ((undefined *)puVar1[0x16] != &DAT_0012502a) {
        freecon();
      }
      uVar2 = uVar2 + 1;
    } while (uVar2 < DAT_001263b0);
  }
  DAT_001263a9 = 0;
  DAT_001263b0 = 0;
  DAT_00126364 = 0;
  DAT_00126360 = 0;
  DAT_0012635c = 0;
  DAT_00126358 = 0;
  DAT_00126350 = 0;
  DAT_0012634c = 0;
  DAT_00126348 = 0;
  DAT_00126354 = 0;
  DAT_00126344 = 0;
  DAT_00126340 = 0;
  DAT_0012633c = 0;
  return;
}




// Function: strcmp_width @ 0x8980

void strcmp_width(undefined8 *param_1,undefined8 *param_2)

{
  long lVar1;
  long lVar2;
  
  lVar2 = param_1[0x19];
  if (lVar2 == 0) {
    lVar2 = FUN_00107d70(*param_1,DAT_001262d0,*(undefined4 *)((long)param_1 + 0xc4));
  }
  lVar1 = param_2[0x19];
  if (lVar1 == 0) {
    lVar1 = FUN_00107d70(*param_2,DAT_001262d0,*(undefined4 *)((long)param_2 + 0xc4));
  }
  if ((int)lVar2 == (int)lVar1) {
    strcmp((char *)*param_1,(char *)*param_2);
    return;
  }
  return;
}




// Function: xstrcoll_atime @ 0x8a00

void xstrcoll_atime(undefined8 *param_1,undefined8 *param_2)

{
  char *__s1;
  char *__s2;
  int *piVar1;
  
  if (((uint)((long)param_1[0xd] < (long)param_2[0xd]) -
      (uint)((long)param_2[0xd] < (long)param_1[0xd])) +
      ((uint)((long)param_1[0xc] < (long)param_2[0xc]) -
      (uint)((long)param_2[0xc] < (long)param_1[0xc])) * 2 == 0) {
    __s1 = (char *)*param_1;
    __s2 = (char *)*param_2;
    piVar1 = __errno_location();
    *piVar1 = 0;
    strcoll(__s1,__s2);
    return;
  }
  return;
}




// Function: rev_xstrcoll_ctime @ 0x8ae0

void rev_xstrcoll_ctime(undefined8 *param_1,undefined8 *param_2)

{
  char *__s1;
  char *__s2;
  int *piVar1;
  
  if (((uint)((long)param_2[0x11] < (long)param_1[0x11]) -
      (uint)((long)param_1[0x11] < (long)param_2[0x11])) +
      ((uint)((long)param_2[0x10] < (long)param_1[0x10]) -
      (uint)((long)param_1[0x10] < (long)param_2[0x10])) * 2 == 0) {
    __s1 = (char *)*param_2;
    __s2 = (char *)*param_1;
    piVar1 = __errno_location();
    *piVar1 = 0;
    strcoll(__s1,__s2);
    return;
  }
  return;
}




// Function: rev_xstrcoll_df_ctime @ 0x8b60

ulong rev_xstrcoll_df_ctime(long param_1,long param_2)

{
  uint uVar1;
  int iVar2;
  ulong uVar3;
  
  iVar2 = *(int *)(param_1 + 0xa8);
  if (((*(int *)(param_2 + 0xa8) == 3) || (*(int *)(param_2 + 0xa8) == 9)) ||
     ((*(uint *)(param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar2 == 9 || iVar2 == 3) goto LAB_00108be0;
    iVar2 = 1;
  }
  else {
    if (iVar2 == 9 || iVar2 == 3) {
      return 0xffffffff;
    }
    iVar2 = 0;
  }
  uVar1 = iVar2 - (uint)((*(uint *)(param_1 + 0xac) & 0xf000) == 0x4000);
  if (uVar1 != 0) {
    return (ulong)uVar1;
  }
LAB_00108be0:
  uVar3 = FUN_00108ae0();
  return uVar3;
}




// Function: rev_xstrcoll_btime @ 0x8bf0

void rev_xstrcoll_btime(undefined8 *param_1,undefined8 *param_2)

{
  char *__s1;
  char *__s2;
  int *piVar1;
  
  if (((uint)((long)param_2[0xf] < (long)param_1[0xf]) -
      (uint)((long)param_1[0xf] < (long)param_2[0xf])) +
      ((uint)((long)param_2[0xe] < (long)param_1[0xe]) -
      (uint)((long)param_1[0xe] < (long)param_2[0xe])) * 2 == 0) {
    __s1 = (char *)*param_2;
    __s2 = (char *)*param_1;
    piVar1 = __errno_location();
    *piVar1 = 0;
    strcoll(__s1,__s2);
    return;
  }
  return;
}




// Function: xstrcoll_btime @ 0x8cd0

void xstrcoll_btime(undefined8 *param_1,undefined8 *param_2)

{
  char *__s1;
  char *__s2;
  int *piVar1;
  
  if (((uint)((long)param_1[0xf] < (long)param_2[0xf]) -
      (uint)((long)param_2[0xf] < (long)param_1[0xf])) +
      ((uint)((long)param_1[0xe] < (long)param_2[0xe]) -
      (uint)((long)param_2[0xe] < (long)param_1[0xe])) * 2 == 0) {
    __s1 = (char *)*param_1;
    __s2 = (char *)*param_2;
    piVar1 = __errno_location();
    *piVar1 = 0;
    strcoll(__s1,__s2);
    return;
  }
  return;
}




// Function: xstrcoll_ctime @ 0x8d40

void xstrcoll_ctime(undefined8 *param_1,undefined8 *param_2)

{
  char *__s1;
  char *__s2;
  int *piVar1;
  
  if (((uint)((long)param_1[0x11] < (long)param_2[0x11]) -
      (uint)((long)param_2[0x11] < (long)param_1[0x11])) +
      ((uint)((long)param_1[0x10] < (long)param_2[0x10]) -
      (uint)((long)param_2[0x10] < (long)param_1[0x10])) * 2 == 0) {
    __s1 = (char *)*param_1;
    __s2 = (char *)*param_2;
    piVar1 = __errno_location();
    *piVar1 = 0;
    strcoll(__s1,__s2);
    return;
  }
  return;
}




// Function: rev_xstrcoll_atime @ 0x8dc0

void rev_xstrcoll_atime(undefined8 *param_1,undefined8 *param_2)

{
  char *__s1;
  char *__s2;
  int *piVar1;
  
  if (((uint)((long)param_2[0xd] < (long)param_1[0xd]) -
      (uint)((long)param_1[0xd] < (long)param_2[0xd])) +
      ((uint)((long)param_2[0xc] < (long)param_1[0xc]) -
      (uint)((long)param_1[0xc] < (long)param_2[0xc])) * 2 == 0) {
    __s1 = (char *)*param_2;
    __s2 = (char *)*param_1;
    piVar1 = __errno_location();
    *piVar1 = 0;
    strcoll(__s1,__s2);
    return;
  }
  return;
}




// Function: sort_files @ 0x8e30

void sort_files(void)

{
  uint uVar1;
  long *plVar2;
  undefined8 *puVar3;
  int iVar4;
  long *plVar5;
  long *plVar6;
  long lVar7;
  ulong uVar8;
  bool bVar9;
  
  uVar8 = DAT_001263b0;
  if (DAT_00126398 < (DAT_001263b0 >> 1) + DAT_001263b0) {
    free(DAT_001263a0);
    DAT_001263a0 = (long *)FUN_001186d0(uVar8,0x18);
    DAT_00126398 = DAT_001263b0 * 3;
  }
  uVar8 = DAT_001263b0;
  if (DAT_001263b0 == 0) {
    if (DAT_00126330 == 2) goto LAB_00108ed9;
LAB_00108ec2:
    if (((DAT_001262b0 != 0) && (DAT_00126338 - 2U < 2)) && (uVar8 != 0)) goto LAB_00108fbb;
  }
  else {
    plVar2 = DAT_001263a0 + DAT_001263b0;
    plVar5 = DAT_001263a0;
    lVar7 = DAT_001263c0;
    do {
      *plVar5 = lVar7;
      plVar5 = plVar5 + 1;
      lVar7 = lVar7 + 0xd0;
    } while (plVar5 != plVar2);
    if (DAT_00126330 != 2) goto LAB_00108ec2;
LAB_00108fbb:
    uVar8 = 0;
    do {
      puVar3 = (undefined8 *)DAT_001263a0[uVar8];
      lVar7 = puVar3[0x19];
      if (lVar7 == 0) {
        lVar7 = FUN_00107d70(*puVar3,DAT_001262d0,*(undefined4 *)((long)puVar3 + 0xc4));
      }
      uVar8 = uVar8 + 1;
      bVar9 = uVar8 < DAT_001263b0;
      puVar3[0x19] = lVar7;
    } while (bVar9);
  }
  if (DAT_00126330 == 6) {
    return;
  }
LAB_00108ed9:
  iVar4 = _setjmp((__jmp_buf_tag *)&DAT_00125300);
  uVar8 = DAT_001263b0;
  plVar2 = DAT_001263a0;
  uVar1 = DAT_00126330;
  if (iVar4 == 0) {
    iVar4 = 0;
  }
  else {
    if (DAT_00126330 == 4) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("sort_type != sort_version","src/ls.c",0x1008,"sort_files");
    }
    if (DAT_001263b0 != 0) {
      plVar5 = DAT_001263a0 + DAT_001263b0;
      plVar6 = DAT_001263a0;
      lVar7 = DAT_001263c0;
      do {
        *plVar6 = lVar7;
        plVar6 = plVar6 + 1;
        lVar7 = lVar7 + 0xd0;
      } while (plVar5 != plVar6);
    }
    iVar4 = 1;
  }
  if (uVar1 == 5) {
    uVar1 = DAT_00126334 + 5;
  }
  FUN_001131b0(plVar2,uVar8,
               (&PTR_FUN_00123fa0)
               [(ulong)DAT_001262f4 +
                ((ulong)DAT_0012632f + ((long)iVar4 + (ulong)uVar1 * 2) * 2) * 2]);
  return;
}




// Function: rev_xstrcoll_width @ 0x91a0

void rev_xstrcoll_width(undefined8 *param_1,undefined8 *param_2)

{
  char *__s1;
  char *__s2;
  int *piVar1;
  long lVar2;
  long lVar3;
  
  lVar3 = param_2[0x19];
  if (lVar3 == 0) {
    lVar3 = FUN_00107d70(*param_2,DAT_001262d0,*(undefined4 *)((long)param_2 + 0xc4));
  }
  lVar2 = param_1[0x19];
  if (lVar2 == 0) {
    lVar2 = FUN_00107d70(*param_1,DAT_001262d0,*(undefined4 *)((long)param_1 + 0xc4));
  }
  if ((int)lVar3 == (int)lVar2) {
    piVar1 = __errno_location();
    __s1 = (char *)*param_2;
    __s2 = (char *)*param_1;
    *piVar1 = 0;
    strcoll(__s1,__s2);
    return;
  }
  return;
}




// Function: xstrcoll_width @ 0x9230

void xstrcoll_width(undefined8 *param_1,undefined8 *param_2)

{
  char *__s1;
  char *__s2;
  int *piVar1;
  long lVar2;
  long lVar3;
  
  lVar3 = param_1[0x19];
  if (lVar3 == 0) {
    lVar3 = FUN_00107d70(*param_1,DAT_001262d0,*(undefined4 *)((long)param_1 + 0xc4));
  }
  lVar2 = param_2[0x19];
  if (lVar2 == 0) {
    lVar2 = FUN_00107d70(*param_2,DAT_001262d0,*(undefined4 *)((long)param_2 + 0xc4));
  }
  if ((int)lVar3 == (int)lVar2) {
    piVar1 = __errno_location();
    __s1 = (char *)*param_1;
    __s2 = (char *)*param_2;
    *piVar1 = 0;
    strcoll(__s1,__s2);
    return;
  }
  return;
}




// Function: xstrcoll_extension @ 0x9370

void xstrcoll_extension(undefined8 *param_1,undefined8 *param_2)

{
  char *__s;
  char *__s_00;
  int iVar1;
  char *__s1;
  char *__s2;
  int *piVar2;
  
  __s = (char *)*param_1;
  __s1 = strrchr(__s,0x2e);
  __s_00 = (char *)*param_2;
  __s2 = strrchr(__s_00,0x2e);
  if (__s2 == (char *)0x0) {
    __s2 = "";
  }
  if (__s1 == (char *)0x0) {
    __s1 = "";
  }
  piVar2 = __errno_location();
  *piVar2 = 0;
  iVar1 = strcoll(__s1,__s2);
  if (iVar1 == 0) {
    strcoll(__s,__s_00);
    return;
  }
  return;
}




// Function: rev_xstrcoll_extension @ 0x9400

void rev_xstrcoll_extension(undefined8 *param_1,undefined8 *param_2)

{
  char *__s;
  char *__s_00;
  int iVar1;
  char *__s1;
  char *__s2;
  int *piVar2;
  
  __s = (char *)*param_2;
  __s1 = strrchr(__s,0x2e);
  __s_00 = (char *)*param_1;
  __s2 = strrchr(__s_00,0x2e);
  if (__s2 == (char *)0x0) {
    __s2 = "";
  }
  if (__s1 == (char *)0x0) {
    __s1 = "";
  }
  piVar2 = __errno_location();
  *piVar2 = 0;
  iVar1 = strcoll(__s1,__s2);
  if (iVar1 == 0) {
    strcoll(__s,__s_00);
    return;
  }
  return;
}




// Function: rev_strcmp_width @ 0x9490

void rev_strcmp_width(undefined8 *param_1,undefined8 *param_2)

{
  long lVar1;
  long lVar2;
  
  lVar2 = param_2[0x19];
  if (lVar2 == 0) {
    lVar2 = FUN_00107d70(*param_2,DAT_001262d0,*(undefined4 *)((long)param_2 + 0xc4));
  }
  lVar1 = param_1[0x19];
  if (lVar1 == 0) {
    lVar1 = FUN_00107d70(*param_1,DAT_001262d0,*(undefined4 *)((long)param_1 + 0xc4));
  }
  if ((int)lVar2 == (int)lVar1) {
    strcmp((char *)*param_2,(char *)*param_1);
    return;
  }
  return;
}




// Function: extract_dirs_from_files @ 0x9510

void extract_dirs_from_files(long param_1,undefined1 param_2)

{
  long *plVar1;
  int iVar2;
  char *pcVar3;
  long *plVar4;
  undefined8 *puVar5;
  undefined8 uVar6;
  char *pcVar7;
  void *__ptr;
  long *plVar8;
  long lVar9;
  bool bVar10;
  
  if ((param_1 != 0) && (DAT_001263c8 != 0)) {
    puVar5 = (undefined8 *)FUN_00118590(0x20);
    uVar6 = FUN_00118a70(param_1);
    *puVar5 = 0;
    puVar5[1] = uVar6;
    *(undefined1 *)(puVar5 + 2) = 0;
    puVar5[3] = DAT_00126380;
    DAT_00126380 = puVar5;
  }
  lVar9 = DAT_001263b0 + -1;
  if (DAT_001263b0 != 0) {
    do {
      while( true ) {
        puVar5 = (undefined8 *)DAT_001263a0[lVar9];
        if ((*(int *)(puVar5 + 0x15) == 3) || (*(int *)(puVar5 + 0x15) == 9)) break;
LAB_00109588:
        bVar10 = lVar9 == 0;
        lVar9 = lVar9 + -1;
        if (bVar10) goto LAB_00109640;
      }
      pcVar3 = (char *)*puVar5;
      if (param_1 == 0) {
LAB_001096a0:
        FUN_00106fb0(pcVar3,puVar5[1],param_2);
      }
      else {
        pcVar7 = (char *)FUN_0010ee20(pcVar3);
        if ((*pcVar7 == '.') &&
           ((pcVar7[(ulong)(pcVar7[1] == '.') + 1] == '\0' ||
            (pcVar7[(ulong)(pcVar7[1] == '.') + 1] == '/')))) goto LAB_00109588;
        if (*pcVar3 == '/') goto LAB_001096a0;
        __ptr = (void *)FUN_0010fbb0(param_1,pcVar3,0);
        FUN_00106fb0(__ptr,puVar5[1],param_2);
        free(__ptr);
      }
      if (*(int *)(puVar5 + 0x15) != 9) goto LAB_00109588;
      free((void *)*puVar5);
      free((void *)puVar5[1]);
      free((void *)puVar5[2]);
      if ((undefined *)puVar5[0x16] == &DAT_0012502a) goto LAB_00109588;
      freecon();
      bVar10 = lVar9 != 0;
      lVar9 = lVar9 + -1;
    } while (bVar10);
LAB_00109640:
    plVar4 = DAT_001263a0;
    if (DAT_001263b0 != 0) {
      plVar1 = DAT_001263a0 + DAT_001263b0;
      DAT_001263b0 = 0;
      plVar8 = DAT_001263a0;
      do {
        iVar2 = *(int *)(*plVar8 + 0xa8);
        plVar4[DAT_001263b0] = *plVar8;
        plVar8 = plVar8 + 1;
        DAT_001263b0 = DAT_001263b0 + (ulong)(iVar2 != 9);
      } while (plVar1 != plVar8);
      return;
    }
  }
  DAT_001263b0 = 0;
  return;
}




// Function: rev_strcmp_df_size @ 0xa660

int rev_strcmp_df_size(undefined8 *param_1,undefined8 *param_2)

{
  int iVar1;
  
  iVar1 = *(int *)(param_1 + 0x15);
  if (((*(int *)(param_2 + 0x15) == 3) || (*(int *)(param_2 + 0x15) == 9)) ||
     ((*(uint *)((long)param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar1 == 9 || iVar1 == 3) goto LAB_0010a6e0;
    iVar1 = 1;
  }
  else {
    if (iVar1 == 9 || iVar1 == 3) {
      return -1;
    }
    iVar1 = 0;
  }
  iVar1 = iVar1 - (uint)((*(uint *)((long)param_1 + 0xac) & 0xf000) == 0x4000);
  if (iVar1 != 0) {
    return iVar1;
  }
LAB_0010a6e0:
  if ((long)param_1[9] < (long)param_2[9]) {
    return -1;
  }
  if (param_2[9] != param_1[9]) {
    return 1;
  }
  iVar1 = strcmp((char *)*param_2,(char *)*param_1);
  return iVar1;
}




// Function: rev_strcmp_df_ctime @ 0xa710

int rev_strcmp_df_ctime(undefined8 *param_1,undefined8 *param_2)

{
  int iVar1;
  
  iVar1 = *(int *)(param_1 + 0x15);
  if (((*(int *)(param_2 + 0x15) == 3) || (*(int *)(param_2 + 0x15) == 9)) ||
     ((*(uint *)((long)param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar1 == 9 || iVar1 == 3) goto LAB_0010a790;
    iVar1 = 1;
  }
  else {
    if (iVar1 == 9 || iVar1 == 3) {
      return -1;
    }
    iVar1 = 0;
  }
  iVar1 = iVar1 - (uint)((*(uint *)((long)param_1 + 0xac) & 0xf000) == 0x4000);
  if (iVar1 != 0) {
    return iVar1;
  }
LAB_0010a790:
  iVar1 = ((uint)((long)param_2[0x11] < (long)param_1[0x11]) -
          (uint)((long)param_1[0x11] < (long)param_2[0x11])) +
          ((uint)((long)param_2[0x10] < (long)param_1[0x10]) -
          (uint)((long)param_1[0x10] < (long)param_2[0x10])) * 2;
  if (iVar1 != 0) {
    return iVar1;
  }
  iVar1 = strcmp((char *)*param_2,(char *)*param_1);
  return iVar1;
}




// Function: rev_strcmp_df_atime @ 0xa7f0

int rev_strcmp_df_atime(undefined8 *param_1,undefined8 *param_2)

{
  int iVar1;
  
  iVar1 = *(int *)(param_1 + 0x15);
  if (((*(int *)(param_2 + 0x15) == 3) || (*(int *)(param_2 + 0x15) == 9)) ||
     ((*(uint *)((long)param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar1 == 9 || iVar1 == 3) goto LAB_0010a870;
    iVar1 = 1;
  }
  else {
    if (iVar1 == 9 || iVar1 == 3) {
      return -1;
    }
    iVar1 = 0;
  }
  iVar1 = iVar1 - (uint)((*(uint *)((long)param_1 + 0xac) & 0xf000) == 0x4000);
  if (iVar1 != 0) {
    return iVar1;
  }
LAB_0010a870:
  iVar1 = ((uint)((long)param_2[0xd] < (long)param_1[0xd]) -
          (uint)((long)param_1[0xd] < (long)param_2[0xd])) +
          ((uint)((long)param_2[0xc] < (long)param_1[0xc]) -
          (uint)((long)param_1[0xc] < (long)param_2[0xc])) * 2;
  if (iVar1 != 0) {
    return iVar1;
  }
  iVar1 = strcmp((char *)*param_2,(char *)*param_1);
  return iVar1;
}




// Function: strcmp_df_name @ 0xa8c0

int strcmp_df_name(undefined8 *param_1,undefined8 *param_2)

{
  int iVar1;
  
  iVar1 = *(int *)(param_1 + 0x15);
  if (((*(int *)(param_2 + 0x15) == 3) || (*(int *)(param_2 + 0x15) == 9)) ||
     ((*(uint *)((long)param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar1 == 9 || iVar1 == 3) goto LAB_0010a940;
    iVar1 = 1;
  }
  else {
    if (iVar1 == 9 || iVar1 == 3) {
      return -1;
    }
    iVar1 = 0;
  }
  iVar1 = iVar1 - (uint)((*(uint *)((long)param_1 + 0xac) & 0xf000) == 0x4000);
  if (iVar1 != 0) {
    return iVar1;
  }
LAB_0010a940:
  iVar1 = strcmp((char *)*param_1,(char *)*param_2);
  return iVar1;
}




// Function: rev_xstrcoll_df_name @ 0xa960

int rev_xstrcoll_df_name(undefined8 *param_1,undefined8 *param_2)

{
  char *__s2;
  char *__s1;
  int iVar1;
  int *piVar2;
  
  iVar1 = *(int *)(param_1 + 0x15);
  if (((*(int *)(param_2 + 0x15) == 3) || (*(int *)(param_2 + 0x15) == 9)) ||
     ((*(uint *)((long)param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar1 == 3 || iVar1 == 9) goto LAB_0010a9e0;
    iVar1 = 1;
  }
  else {
    if (iVar1 == 3 || iVar1 == 9) {
      return -1;
    }
    iVar1 = 0;
  }
  iVar1 = iVar1 - (uint)((*(uint *)((long)param_1 + 0xac) & 0xf000) == 0x4000);
  if (iVar1 != 0) {
    return iVar1;
  }
LAB_0010a9e0:
  __s2 = (char *)*param_1;
  __s1 = (char *)*param_2;
  piVar2 = __errno_location();
  *piVar2 = 0;
  iVar1 = strcoll(__s1,__s2);
  return iVar1;
}




// Function: rev_xstrcoll_df_version @ 0xaa20

int rev_xstrcoll_df_version(undefined8 *param_1,undefined8 *param_2)

{
  char *__s1;
  char *__s2;
  int iVar1;
  
  iVar1 = *(int *)(param_1 + 0x15);
  if (((*(int *)(param_2 + 0x15) == 3) || (*(int *)(param_2 + 0x15) == 9)) ||
     ((*(uint *)((long)param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar1 == 9 || iVar1 == 3) goto LAB_0010aaa0;
    iVar1 = 1;
  }
  else {
    if (iVar1 == 9 || iVar1 == 3) {
      return -1;
    }
    iVar1 = 0;
  }
  iVar1 = iVar1 - (uint)((*(uint *)((long)param_1 + 0xac) & 0xf000) == 0x4000);
  if (iVar1 != 0) {
    return iVar1;
  }
LAB_0010aaa0:
  __s1 = (char *)*param_2;
  __s2 = (char *)*param_1;
  iVar1 = FUN_001101e0(__s1,__s2);
  if (iVar1 == 0) {
    iVar1 = strcmp(__s1,__s2);
    return iVar1;
  }
  return iVar1;
}




// Function: xstrcoll_df_version @ 0xaaf0

int xstrcoll_df_version(undefined8 *param_1,undefined8 *param_2)

{
  char *__s1;
  char *__s2;
  int iVar1;
  
  iVar1 = *(int *)(param_1 + 0x15);
  if (((*(int *)(param_2 + 0x15) == 3) || (*(int *)(param_2 + 0x15) == 9)) ||
     ((*(uint *)((long)param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar1 == 9 || iVar1 == 3) goto LAB_0010ab70;
    iVar1 = 1;
  }
  else {
    if (iVar1 == 9 || iVar1 == 3) {
      return -1;
    }
    iVar1 = 0;
  }
  iVar1 = iVar1 - (uint)((*(uint *)((long)param_1 + 0xac) & 0xf000) == 0x4000);
  if (iVar1 != 0) {
    return iVar1;
  }
LAB_0010ab70:
  __s1 = (char *)*param_1;
  __s2 = (char *)*param_2;
  iVar1 = FUN_001101e0(__s1,__s2);
  if (iVar1 == 0) {
    iVar1 = strcmp(__s1,__s2);
    return iVar1;
  }
  return iVar1;
}




// Function: xstrcoll_df_name @ 0xabc0

int xstrcoll_df_name(undefined8 *param_1,undefined8 *param_2)

{
  char *__s2;
  char *__s1;
  int iVar1;
  int *piVar2;
  
  iVar1 = *(int *)(param_1 + 0x15);
  if (((*(int *)(param_2 + 0x15) == 3) || (*(int *)(param_2 + 0x15) == 9)) ||
     ((*(uint *)((long)param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar1 == 3 || iVar1 == 9) goto LAB_0010ac40;
    iVar1 = 1;
  }
  else {
    if (iVar1 == 3 || iVar1 == 9) {
      return -1;
    }
    iVar1 = 0;
  }
  iVar1 = iVar1 - (uint)((*(uint *)((long)param_1 + 0xac) & 0xf000) == 0x4000);
  if (iVar1 != 0) {
    return iVar1;
  }
LAB_0010ac40:
  __s2 = (char *)*param_2;
  __s1 = (char *)*param_1;
  piVar2 = __errno_location();
  *piVar2 = 0;
  iVar1 = strcoll(__s1,__s2);
  return iVar1;
}




// Function: xstrcoll_df_btime @ 0xac80

ulong xstrcoll_df_btime(long param_1,long param_2)

{
  uint uVar1;
  int iVar2;
  ulong uVar3;
  
  iVar2 = *(int *)(param_1 + 0xa8);
  if (((*(int *)(param_2 + 0xa8) == 3) || (*(int *)(param_2 + 0xa8) == 9)) ||
     ((*(uint *)(param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar2 == 9 || iVar2 == 3) goto LAB_0010ad00;
    iVar2 = 1;
  }
  else {
    if (iVar2 == 9 || iVar2 == 3) {
      return 0xffffffff;
    }
    iVar2 = 0;
  }
  uVar1 = iVar2 - (uint)((*(uint *)(param_1 + 0xac) & 0xf000) == 0x4000);
  if (uVar1 != 0) {
    return (ulong)uVar1;
  }
LAB_0010ad00:
  uVar3 = FUN_00108cd0();
  return uVar3;
}




// Function: xstrcoll_df_ctime @ 0xad10

ulong xstrcoll_df_ctime(long param_1,long param_2)

{
  uint uVar1;
  int iVar2;
  ulong uVar3;
  
  iVar2 = *(int *)(param_1 + 0xa8);
  if (((*(int *)(param_2 + 0xa8) == 3) || (*(int *)(param_2 + 0xa8) == 9)) ||
     ((*(uint *)(param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar2 == 9 || iVar2 == 3) goto LAB_0010ad90;
    iVar2 = 1;
  }
  else {
    if (iVar2 == 9 || iVar2 == 3) {
      return 0xffffffff;
    }
    iVar2 = 0;
  }
  uVar1 = iVar2 - (uint)((*(uint *)(param_1 + 0xac) & 0xf000) == 0x4000);
  if (uVar1 != 0) {
    return (ulong)uVar1;
  }
LAB_0010ad90:
  uVar3 = FUN_00108d40();
  return uVar3;
}




// Function: rev_strcmp_df_width @ 0xada0

ulong rev_strcmp_df_width(long param_1,long param_2)

{
  uint uVar1;
  int iVar2;
  ulong uVar3;
  
  iVar2 = *(int *)(param_1 + 0xa8);
  if (((*(int *)(param_2 + 0xa8) == 3) || (*(int *)(param_2 + 0xa8) == 9)) ||
     ((*(uint *)(param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar2 == 9 || iVar2 == 3) goto LAB_0010ae20;
    iVar2 = 1;
  }
  else {
    if (iVar2 == 9 || iVar2 == 3) {
      return 0xffffffff;
    }
    iVar2 = 0;
  }
  uVar1 = iVar2 - (uint)((*(uint *)(param_1 + 0xac) & 0xf000) == 0x4000);
  if (uVar1 != 0) {
    return (ulong)uVar1;
  }
LAB_0010ae20:
  uVar3 = FUN_00109490();
  return uVar3;
}




// Function: rev_strcmp_df_btime @ 0xae30

int rev_strcmp_df_btime(undefined8 *param_1,undefined8 *param_2)

{
  int iVar1;
  
  iVar1 = *(int *)(param_1 + 0x15);
  if (((*(int *)(param_2 + 0x15) == 3) || (*(int *)(param_2 + 0x15) == 9)) ||
     ((*(uint *)((long)param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar1 == 9 || iVar1 == 3) goto LAB_0010aeb0;
    iVar1 = 1;
  }
  else {
    if (iVar1 == 9 || iVar1 == 3) {
      return -1;
    }
    iVar1 = 0;
  }
  iVar1 = iVar1 - (uint)((*(uint *)((long)param_1 + 0xac) & 0xf000) == 0x4000);
  if (iVar1 != 0) {
    return iVar1;
  }
LAB_0010aeb0:
  iVar1 = ((uint)((long)param_2[0xf] < (long)param_1[0xf]) -
          (uint)((long)param_1[0xf] < (long)param_2[0xf])) +
          ((uint)((long)param_2[0xe] < (long)param_1[0xe]) -
          (uint)((long)param_1[0xe] < (long)param_2[0xe])) * 2;
  if (iVar1 != 0) {
    return iVar1;
  }
  iVar1 = strcmp((char *)*param_2,(char *)*param_1);
  return iVar1;
}




// Function: strcmp_df_btime @ 0xaf00

int strcmp_df_btime(undefined8 *param_1,undefined8 *param_2)

{
  int iVar1;
  
  iVar1 = *(int *)(param_1 + 0x15);
  if (((*(int *)(param_2 + 0x15) == 3) || (*(int *)(param_2 + 0x15) == 9)) ||
     ((*(uint *)((long)param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar1 == 9 || iVar1 == 3) goto LAB_0010af80;
    iVar1 = 1;
  }
  else {
    if (iVar1 == 9 || iVar1 == 3) {
      return -1;
    }
    iVar1 = 0;
  }
  iVar1 = iVar1 - (uint)((*(uint *)((long)param_1 + 0xac) & 0xf000) == 0x4000);
  if (iVar1 != 0) {
    return iVar1;
  }
LAB_0010af80:
  iVar1 = ((uint)((long)param_1[0xf] < (long)param_2[0xf]) -
          (uint)((long)param_2[0xf] < (long)param_1[0xf])) +
          ((uint)((long)param_1[0xe] < (long)param_2[0xe]) -
          (uint)((long)param_2[0xe] < (long)param_1[0xe])) * 2;
  if (iVar1 != 0) {
    return iVar1;
  }
  iVar1 = strcmp((char *)*param_1,(char *)*param_2);
  return iVar1;
}




// Function: rev_xstrcoll_df_btime @ 0xafd0

ulong rev_xstrcoll_df_btime(long param_1,long param_2)

{
  uint uVar1;
  int iVar2;
  ulong uVar3;
  
  iVar2 = *(int *)(param_1 + 0xa8);
  if (((*(int *)(param_2 + 0xa8) == 3) || (*(int *)(param_2 + 0xa8) == 9)) ||
     ((*(uint *)(param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar2 == 9 || iVar2 == 3) goto LAB_0010b050;
    iVar2 = 1;
  }
  else {
    if (iVar2 == 9 || iVar2 == 3) {
      return 0xffffffff;
    }
    iVar2 = 0;
  }
  uVar1 = iVar2 - (uint)((*(uint *)(param_1 + 0xac) & 0xf000) == 0x4000);
  if (uVar1 != 0) {
    return (ulong)uVar1;
  }
LAB_0010b050:
  uVar3 = FUN_00108bf0();
  return uVar3;
}




// Function: xstrcoll_df_width @ 0xb060

ulong xstrcoll_df_width(long param_1,long param_2)

{
  uint uVar1;
  int iVar2;
  ulong uVar3;
  
  iVar2 = *(int *)(param_1 + 0xa8);
  if (((*(int *)(param_2 + 0xa8) == 3) || (*(int *)(param_2 + 0xa8) == 9)) ||
     ((*(uint *)(param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar2 == 9 || iVar2 == 3) goto LAB_0010b0e0;
    iVar2 = 1;
  }
  else {
    if (iVar2 == 9 || iVar2 == 3) {
      return 0xffffffff;
    }
    iVar2 = 0;
  }
  uVar1 = iVar2 - (uint)((*(uint *)(param_1 + 0xac) & 0xf000) == 0x4000);
  if (uVar1 != 0) {
    return (ulong)uVar1;
  }
LAB_0010b0e0:
  uVar3 = FUN_00109230();
  return uVar3;
}




// Function: strcmp_df_extension @ 0xb0f0

ulong strcmp_df_extension(long param_1,long param_2)

{
  uint uVar1;
  int iVar2;
  ulong uVar3;
  
  iVar2 = *(int *)(param_1 + 0xa8);
  if (((*(int *)(param_2 + 0xa8) == 3) || (*(int *)(param_2 + 0xa8) == 9)) ||
     ((*(uint *)(param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar2 == 9 || iVar2 == 3) goto LAB_0010b170;
    iVar2 = 1;
  }
  else {
    if (iVar2 == 9 || iVar2 == 3) {
      return 0xffffffff;
    }
    iVar2 = 0;
  }
  uVar1 = iVar2 - (uint)((*(uint *)(param_1 + 0xac) & 0xf000) == 0x4000);
  if (uVar1 != 0) {
    return (ulong)uVar1;
  }
LAB_0010b170:
  uVar3 = FUN_001071f0(param_1,param_2,PTR_strcmp_00124fd8);
  return uVar3;
}




// Function: rev_xstrcoll_df_extension @ 0xb190

ulong rev_xstrcoll_df_extension(long param_1,long param_2)

{
  uint uVar1;
  int iVar2;
  ulong uVar3;
  
  iVar2 = *(int *)(param_1 + 0xa8);
  if (((*(int *)(param_2 + 0xa8) == 3) || (*(int *)(param_2 + 0xa8) == 9)) ||
     ((*(uint *)(param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar2 == 9 || iVar2 == 3) goto LAB_0010b210;
    iVar2 = 1;
  }
  else {
    if (iVar2 == 9 || iVar2 == 3) {
      return 0xffffffff;
    }
    iVar2 = 0;
  }
  uVar1 = iVar2 - (uint)((*(uint *)(param_1 + 0xac) & 0xf000) == 0x4000);
  if (uVar1 != 0) {
    return (ulong)uVar1;
  }
LAB_0010b210:
  uVar3 = FUN_00109400();
  return uVar3;
}




// Function: strcmp_df_size @ 0xb220

int strcmp_df_size(undefined8 *param_1,undefined8 *param_2)

{
  int iVar1;
  
  iVar1 = *(int *)(param_1 + 0x15);
  if (((*(int *)(param_2 + 0x15) == 3) || (*(int *)(param_2 + 0x15) == 9)) ||
     ((*(uint *)((long)param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar1 == 9 || iVar1 == 3) goto LAB_0010b2a0;
    iVar1 = 1;
  }
  else {
    if (iVar1 == 9 || iVar1 == 3) {
      return -1;
    }
    iVar1 = 0;
  }
  iVar1 = iVar1 - (uint)((*(uint *)((long)param_1 + 0xac) & 0xf000) == 0x4000);
  if (iVar1 != 0) {
    return iVar1;
  }
LAB_0010b2a0:
  if ((long)param_2[9] < (long)param_1[9]) {
    return -1;
  }
  if (param_1[9] != param_2[9]) {
    return 1;
  }
  iVar1 = strcmp((char *)*param_1,(char *)*param_2);
  return iVar1;
}




// Function: rev_xstrcoll_df_size @ 0xb2d0

int rev_xstrcoll_df_size(undefined8 *param_1,undefined8 *param_2)

{
  char *__s1;
  char *__s2;
  int iVar1;
  int *piVar2;
  
  iVar1 = *(int *)(param_1 + 0x15);
  if (((*(int *)(param_2 + 0x15) == 3) || (*(int *)(param_2 + 0x15) == 9)) ||
     ((*(uint *)((long)param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar1 == 3 || iVar1 == 9) goto LAB_0010b350;
    iVar1 = 1;
  }
  else {
    if (iVar1 == 3 || iVar1 == 9) {
      return -1;
    }
    iVar1 = 0;
  }
  iVar1 = iVar1 - (uint)((*(uint *)((long)param_1 + 0xac) & 0xf000) == 0x4000);
  if (iVar1 != 0) {
    return iVar1;
  }
LAB_0010b350:
  if ((long)param_1[9] < (long)param_2[9]) {
    return -1;
  }
  if (param_2[9] != param_1[9]) {
    return 1;
  }
  __s1 = (char *)*param_2;
  __s2 = (char *)*param_1;
  piVar2 = __errno_location();
  *piVar2 = 0;
  iVar1 = strcoll(__s1,__s2);
  return iVar1;
}




// Function: strcmp_df_width @ 0xb3a0

ulong strcmp_df_width(long param_1,long param_2)

{
  uint uVar1;
  int iVar2;
  ulong uVar3;
  
  iVar2 = *(int *)(param_1 + 0xa8);
  if (((*(int *)(param_2 + 0xa8) == 3) || (*(int *)(param_2 + 0xa8) == 9)) ||
     ((*(uint *)(param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar2 == 9 || iVar2 == 3) goto LAB_0010b420;
    iVar2 = 1;
  }
  else {
    if (iVar2 == 9 || iVar2 == 3) {
      return 0xffffffff;
    }
    iVar2 = 0;
  }
  uVar1 = iVar2 - (uint)((*(uint *)(param_1 + 0xac) & 0xf000) == 0x4000);
  if (uVar1 != 0) {
    return (ulong)uVar1;
  }
LAB_0010b420:
  uVar3 = FUN_00108980();
  return uVar3;
}




// Function: rev_xstrcoll_df_width @ 0xb430

ulong rev_xstrcoll_df_width(long param_1,long param_2)

{
  uint uVar1;
  int iVar2;
  ulong uVar3;
  
  iVar2 = *(int *)(param_1 + 0xa8);
  if (((*(int *)(param_2 + 0xa8) == 3) || (*(int *)(param_2 + 0xa8) == 9)) ||
     ((*(uint *)(param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar2 == 9 || iVar2 == 3) goto LAB_0010b4b0;
    iVar2 = 1;
  }
  else {
    if (iVar2 == 9 || iVar2 == 3) {
      return 0xffffffff;
    }
    iVar2 = 0;
  }
  uVar1 = iVar2 - (uint)((*(uint *)(param_1 + 0xac) & 0xf000) == 0x4000);
  if (uVar1 != 0) {
    return (ulong)uVar1;
  }
LAB_0010b4b0:
  uVar3 = FUN_001091a0();
  return uVar3;
}




// Function: xstrcoll_df_extension @ 0xb4c0

ulong xstrcoll_df_extension(long param_1,long param_2)

{
  uint uVar1;
  int iVar2;
  ulong uVar3;
  
  iVar2 = *(int *)(param_1 + 0xa8);
  if (((*(int *)(param_2 + 0xa8) == 3) || (*(int *)(param_2 + 0xa8) == 9)) ||
     ((*(uint *)(param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar2 == 9 || iVar2 == 3) goto LAB_0010b540;
    iVar2 = 1;
  }
  else {
    if (iVar2 == 9 || iVar2 == 3) {
      return 0xffffffff;
    }
    iVar2 = 0;
  }
  uVar1 = iVar2 - (uint)((*(uint *)(param_1 + 0xac) & 0xf000) == 0x4000);
  if (uVar1 != 0) {
    return (ulong)uVar1;
  }
LAB_0010b540:
  uVar3 = FUN_00109370();
  return uVar3;
}




// Function: rev_strcmp_df_name @ 0xb550

int rev_strcmp_df_name(undefined8 *param_1,undefined8 *param_2)

{
  int iVar1;
  
  iVar1 = *(int *)(param_1 + 0x15);
  if (((*(int *)(param_2 + 0x15) == 3) || (*(int *)(param_2 + 0x15) == 9)) ||
     ((*(uint *)((long)param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar1 == 9 || iVar1 == 3) goto LAB_0010b5d0;
    iVar1 = 1;
  }
  else {
    if (iVar1 == 9 || iVar1 == 3) {
      return -1;
    }
    iVar1 = 0;
  }
  iVar1 = iVar1 - (uint)((*(uint *)((long)param_1 + 0xac) & 0xf000) == 0x4000);
  if (iVar1 != 0) {
    return iVar1;
  }
LAB_0010b5d0:
  iVar1 = strcmp((char *)*param_2,(char *)*param_1);
  return iVar1;
}




// Function: xstrcoll_df_size @ 0xb5f0

int xstrcoll_df_size(undefined8 *param_1,undefined8 *param_2)

{
  char *__s1;
  char *__s2;
  int iVar1;
  int *piVar2;
  
  iVar1 = *(int *)(param_1 + 0x15);
  if (((*(int *)(param_2 + 0x15) == 3) || (*(int *)(param_2 + 0x15) == 9)) ||
     ((*(uint *)((long)param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar1 == 3 || iVar1 == 9) goto LAB_0010b670;
    iVar1 = 1;
  }
  else {
    if (iVar1 == 3 || iVar1 == 9) {
      return -1;
    }
    iVar1 = 0;
  }
  iVar1 = iVar1 - (uint)((*(uint *)((long)param_1 + 0xac) & 0xf000) == 0x4000);
  if (iVar1 != 0) {
    return iVar1;
  }
LAB_0010b670:
  if ((long)param_2[9] < (long)param_1[9]) {
    return -1;
  }
  if (param_1[9] != param_2[9]) {
    return 1;
  }
  __s1 = (char *)*param_1;
  __s2 = (char *)*param_2;
  piVar2 = __errno_location();
  *piVar2 = 0;
  iVar1 = strcoll(__s1,__s2);
  return iVar1;
}




// Function: strcmp_df_atime @ 0xb6c0

int strcmp_df_atime(undefined8 *param_1,undefined8 *param_2)

{
  int iVar1;
  
  iVar1 = *(int *)(param_1 + 0x15);
  if (((*(int *)(param_2 + 0x15) == 3) || (*(int *)(param_2 + 0x15) == 9)) ||
     ((*(uint *)((long)param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar1 == 9 || iVar1 == 3) goto LAB_0010b740;
    iVar1 = 1;
  }
  else {
    if (iVar1 == 9 || iVar1 == 3) {
      return -1;
    }
    iVar1 = 0;
  }
  iVar1 = iVar1 - (uint)((*(uint *)((long)param_1 + 0xac) & 0xf000) == 0x4000);
  if (iVar1 != 0) {
    return iVar1;
  }
LAB_0010b740:
  iVar1 = ((uint)((long)param_1[0xd] < (long)param_2[0xd]) -
          (uint)((long)param_2[0xd] < (long)param_1[0xd])) +
          ((uint)((long)param_1[0xc] < (long)param_2[0xc]) -
          (uint)((long)param_2[0xc] < (long)param_1[0xc])) * 2;
  if (iVar1 != 0) {
    return iVar1;
  }
  iVar1 = strcmp((char *)*param_1,(char *)*param_2);
  return iVar1;
}




// Function: rev_xstrcoll_df_atime @ 0xb790

ulong rev_xstrcoll_df_atime(long param_1,long param_2)

{
  uint uVar1;
  int iVar2;
  ulong uVar3;
  
  iVar2 = *(int *)(param_1 + 0xa8);
  if (((*(int *)(param_2 + 0xa8) == 3) || (*(int *)(param_2 + 0xa8) == 9)) ||
     ((*(uint *)(param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar2 == 9 || iVar2 == 3) goto LAB_0010b810;
    iVar2 = 1;
  }
  else {
    if (iVar2 == 9 || iVar2 == 3) {
      return 0xffffffff;
    }
    iVar2 = 0;
  }
  uVar1 = iVar2 - (uint)((*(uint *)(param_1 + 0xac) & 0xf000) == 0x4000);
  if (uVar1 != 0) {
    return (ulong)uVar1;
  }
LAB_0010b810:
  uVar3 = FUN_00108dc0();
  return uVar3;
}




// Function: rev_strcmp_df_extension @ 0xb820

ulong rev_strcmp_df_extension(long param_1,long param_2)

{
  uint uVar1;
  int iVar2;
  ulong uVar3;
  
  iVar2 = *(int *)(param_1 + 0xa8);
  if (((*(int *)(param_2 + 0xa8) == 3) || (*(int *)(param_2 + 0xa8) == 9)) ||
     ((*(uint *)(param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar2 == 9 || iVar2 == 3) goto LAB_0010b8a0;
    iVar2 = 1;
  }
  else {
    if (iVar2 == 9 || iVar2 == 3) {
      return 0xffffffff;
    }
    iVar2 = 0;
  }
  uVar1 = iVar2 - (uint)((*(uint *)(param_1 + 0xac) & 0xf000) == 0x4000);
  if (uVar1 != 0) {
    return (ulong)uVar1;
  }
LAB_0010b8a0:
  uVar3 = FUN_001071f0(param_2,param_1,PTR_strcmp_00124fd8);
  return uVar3;
}




// Function: rev_strcmp_df_mtime @ 0xb8c0

int rev_strcmp_df_mtime(undefined8 *param_1,undefined8 *param_2)

{
  int iVar1;
  
  iVar1 = *(int *)(param_1 + 0x15);
  if (((*(int *)(param_2 + 0x15) == 3) || (*(int *)(param_2 + 0x15) == 9)) ||
     ((*(uint *)((long)param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar1 == 9 || iVar1 == 3) goto LAB_0010b940;
    iVar1 = 1;
  }
  else {
    if (iVar1 == 9 || iVar1 == 3) {
      return -1;
    }
    iVar1 = 0;
  }
  iVar1 = iVar1 - (uint)((*(uint *)((long)param_1 + 0xac) & 0xf000) == 0x4000);
  if (iVar1 != 0) {
    return iVar1;
  }
LAB_0010b940:
  iVar1 = ((uint)((long)param_2[0xf] < (long)param_1[0xf]) -
          (uint)((long)param_1[0xf] < (long)param_2[0xf])) +
          ((uint)((long)param_2[0xe] < (long)param_1[0xe]) -
          (uint)((long)param_1[0xe] < (long)param_2[0xe])) * 2;
  if (iVar1 != 0) {
    return iVar1;
  }
  iVar1 = strcmp((char *)*param_2,(char *)*param_1);
  return iVar1;
}




// Function: strcmp_df_mtime @ 0xb990

int strcmp_df_mtime(undefined8 *param_1,undefined8 *param_2)

{
  int iVar1;
  
  iVar1 = *(int *)(param_1 + 0x15);
  if (((*(int *)(param_2 + 0x15) == 3) || (*(int *)(param_2 + 0x15) == 9)) ||
     ((*(uint *)((long)param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar1 == 9 || iVar1 == 3) goto LAB_0010ba10;
    iVar1 = 1;
  }
  else {
    if (iVar1 == 9 || iVar1 == 3) {
      return -1;
    }
    iVar1 = 0;
  }
  iVar1 = iVar1 - (uint)((*(uint *)((long)param_1 + 0xac) & 0xf000) == 0x4000);
  if (iVar1 != 0) {
    return iVar1;
  }
LAB_0010ba10:
  iVar1 = ((uint)((long)param_1[0xf] < (long)param_2[0xf]) -
          (uint)((long)param_2[0xf] < (long)param_1[0xf])) +
          ((uint)((long)param_1[0xe] < (long)param_2[0xe]) -
          (uint)((long)param_2[0xe] < (long)param_1[0xe])) * 2;
  if (iVar1 != 0) {
    return iVar1;
  }
  iVar1 = strcmp((char *)*param_1,(char *)*param_2);
  return iVar1;
}




// Function: xstrcoll_df_atime @ 0xbaf0

ulong xstrcoll_df_atime(long param_1,long param_2)

{
  uint uVar1;
  int iVar2;
  ulong uVar3;
  
  iVar2 = *(int *)(param_1 + 0xa8);
  if (((*(int *)(param_2 + 0xa8) == 3) || (*(int *)(param_2 + 0xa8) == 9)) ||
     ((*(uint *)(param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar2 == 9 || iVar2 == 3) goto LAB_0010bb70;
    iVar2 = 1;
  }
  else {
    if (iVar2 == 9 || iVar2 == 3) {
      return 0xffffffff;
    }
    iVar2 = 0;
  }
  uVar1 = iVar2 - (uint)((*(uint *)(param_1 + 0xac) & 0xf000) == 0x4000);
  if (uVar1 != 0) {
    return (ulong)uVar1;
  }
LAB_0010bb70:
  uVar3 = FUN_00108a00();
  return uVar3;
}




// Function: strcmp_df_ctime @ 0xbc10

int strcmp_df_ctime(undefined8 *param_1,undefined8 *param_2)

{
  int iVar1;
  
  iVar1 = *(int *)(param_1 + 0x15);
  if (((*(int *)(param_2 + 0x15) == 3) || (*(int *)(param_2 + 0x15) == 9)) ||
     ((*(uint *)((long)param_2 + 0xac) & 0xf000) == 0x4000)) {
    if (iVar1 == 9 || iVar1 == 3) goto LAB_0010bc90;
    iVar1 = 1;
  }
  else {
    if (iVar1 == 9 || iVar1 == 3) {
      return -1;
    }
    iVar1 = 0;
  }
  iVar1 = iVar1 - (uint)((*(uint *)((long)param_1 + 0xac) & 0xf000) == 0x4000);
  if (iVar1 != 0) {
    return iVar1;
  }
LAB_0010bc90:
  iVar1 = ((uint)((long)param_1[0x11] < (long)param_2[0x11]) -
          (uint)((long)param_2[0x11] < (long)param_1[0x11])) +
          ((uint)((long)param_1[0x10] < (long)param_2[0x10]) -
          (uint)((long)param_2[0x10] < (long)param_1[0x10])) * 2;
  if (iVar1 != 0) {
    return iVar1;
  }
  iVar1 = strcmp((char *)*param_1,(char *)*param_2);
  return iVar1;
}




// Function: quote_name @ 0xbcf0

long quote_name(byte *param_1,undefined8 param_2,undefined4 param_3,long param_4,char param_5,
                 long param_6,long param_7)

{
  byte bVar1;
  byte *pbVar2;
  char cVar3;
  size_t sVar4;
  void *__ptr;
  char *pcVar5;
  char *pcVar6;
  long *plVar7;
  long lVar8;
  size_t sVar9;
  char cVar10;
  long in_FS_OFFSET;
  byte local_2051;
  byte *local_2050;
  byte local_2048 [8200];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_2050 = local_2048;
  sVar4 = FUN_00107920(&local_2050,param_1,param_2,param_3,0);
  if ((local_2051 != 0) && (param_5 != '\0')) {
    DAT_001261f8 = DAT_001261f8 + 1;
    pcVar5 = stdout->_IO_write_ptr;
    if (pcVar5 < stdout->_IO_write_end) {
      stdout->_IO_write_ptr = pcVar5 + 1;
      *pcVar5 = ' ';
    }
    else {
      __overflow(stdout,0x20);
    }
  }
  if (param_4 != 0) {
    cVar3 = FUN_00106c80(4);
    if (cVar3 != '\0') {
      FUN_00107880(&DAT_00125060);
      FUN_00107880(&DAT_00125070);
    }
    FUN_00107880(&DAT_00125060);
    FUN_00107880(param_4);
    FUN_00107880(&DAT_00125070);
  }
  cVar3 = DAT_001263a9;
  sVar9 = sVar4;
  if (param_7 == 0) {
    lVar8 = 0;
    cVar3 = '\0';
  }
  else {
    cVar10 = '\0';
    if ((DAT_001263a8 == '\0') || (cVar10 = DAT_001263a9, DAT_001263a9 == '\0')) {
      lVar8 = 0;
      cVar3 = cVar10;
    }
    else if (local_2051 == 0) {
      bVar1 = *local_2050;
      pbVar2 = (byte *)stdout->_IO_write_ptr;
      sVar9 = sVar4 - 2;
      if (pbVar2 < stdout->_IO_write_end) {
        lVar8 = 1;
        stdout->_IO_write_ptr = (char *)(pbVar2 + 1);
        *pbVar2 = bVar1;
      }
      else {
        __overflow(stdout,(uint)bVar1);
        lVar8 = 1;
      }
    }
    else {
      lVar8 = 0;
      cVar3 = '\0';
    }
    __ptr = (void *)FUN_001072a0(DAT_00126388,0);
    pcVar5 = (char *)FUN_001072a0(param_7,1);
    pcVar6 = "";
    if (*pcVar5 != '/') {
      pcVar6 = "/";
    }
    __printf_chk(1,&DAT_0011baff,__ptr,pcVar6);
    free(__ptr);
    free(pcVar5);
  }
  if (param_6 == 0) {
    fwrite_unlocked(local_2050 + lVar8,1,sVar9,stdout);
    DAT_001261f8 = DAT_001261f8 + sVar4;
  }
  else {
    if (DAT_00126318 != '\0') {
      plVar7 = *(long **)(param_6 + 0x18);
      if ((ulong)(*(long *)(param_6 + 0x20) - (long)plVar7) < 8) {
        _obstack_newchunk(param_6,8);
        plVar7 = *(long **)(param_6 + 0x18);
      }
      *plVar7 = DAT_001261f8;
      *(long *)(param_6 + 0x18) = *(long *)(param_6 + 0x18) + 8;
    }
    fwrite_unlocked(local_2050 + lVar8,1,sVar9,stdout);
    DAT_001261f8 = DAT_001261f8 + sVar4;
    if (DAT_00126318 != '\0') {
      plVar7 = *(long **)(param_6 + 0x18);
      if ((ulong)(*(long *)(param_6 + 0x20) - (long)plVar7) < 8) {
        _obstack_newchunk(param_6,8);
        plVar7 = *(long **)(param_6 + 0x18);
      }
      *plVar7 = DAT_001261f8;
      *(long *)(param_6 + 0x18) = *(long *)(param_6 + 0x18) + 8;
    }
  }
  if ((param_7 != 0) && (fputs_unlocked("\x1b]8;;\a",stdout), cVar3 != '\0')) {
    bVar1 = local_2050[sVar4 - 1];
    pbVar2 = (byte *)stdout->_IO_write_ptr;
    if (pbVar2 < stdout->_IO_write_end) {
      stdout->_IO_write_ptr = (char *)(pbVar2 + 1);
      *pbVar2 = bVar1;
    }
    else {
      __overflow(stdout,(uint)bVar1);
    }
  }
  if ((local_2050 != param_1) && (local_2050 != local_2048)) {
    free(local_2050);
  }
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_2051 + sVar4;
}




// Function: print_name_with_quoting @ 0xc0e0

long print_name_with_quoting(undefined8 *param_1,byte param_2,undefined8 param_3,ulong param_4)

{
  char cVar1;
  uint uVar2;
  int iVar3;
  long lVar4;
  size_t sVar5;
  ulong *puVar6;
  uint uVar7;
  ulong uVar8;
  uint uVar9;
  char *__s;
  
  if (param_2 == 0) {
    __s = (char *)*param_1;
    if (DAT_00126312 == '\0') goto LAB_0010c112;
    uVar2 = (uint)*(byte *)((long)param_1 + 0xb9);
    if ((DAT_00126390 == '\0') || (*(byte *)((long)param_1 + 0xb9) == 0)) {
      uVar9 = *(uint *)(param_1 + 6);
    }
    else {
      uVar9 = *(uint *)((long)param_1 + 0xac);
    }
LAB_0010c1b4:
    cVar1 = *(char *)(param_1 + 0x17);
    if (cVar1 == '\0') goto LAB_0010c1c5;
LAB_0010c303:
    uVar7 = uVar9 & 0xf000;
    if (uVar7 == 0x8000) {
      if (((uVar9 & 0x800) != 0) && (cVar1 = FUN_00106c80(0x10), cVar1 != '\0')) {
        lVar4 = 0x100;
        goto LAB_0010c21e;
      }
      if (((uVar9 & 0x400) != 0) && (cVar1 = FUN_00106c80(0x11), cVar1 != '\0')) {
        lVar4 = 0x110;
        goto LAB_0010c21e;
      }
      cVar1 = FUN_00106c80(0x15);
      if ((cVar1 != '\0') && (*(char *)(param_1 + 0x18) != '\0')) {
        lVar4 = 0x150;
        goto LAB_0010c21e;
      }
      if (((uVar9 & 0x49) != 0) && (cVar1 = FUN_00106c80(0xe), cVar1 != '\0')) {
        lVar4 = 0xe0;
        goto LAB_0010c21e;
      }
      if ((1 < (ulong)param_1[5]) && (cVar1 = FUN_00106c80(0x16), cVar1 != '\0')) {
        lVar4 = 0x160;
        goto LAB_0010c21e;
      }
      goto LAB_0010c3b0;
    }
    if (uVar7 != 0x4000) {
      if (uVar7 != 0xa000) {
        lVar4 = 0x80;
        if ((((uVar7 != 0x1000) && (lVar4 = 0x90, uVar7 != 0xc000)) &&
            (lVar4 = 0xa0, uVar7 != 0x6000)) && (lVar4 = 0xb0, uVar7 != 0x2000)) {
          lVar4 = 0xd0;
        }
        goto LAB_0010c21e;
      }
      uVar8 = 7;
      goto LAB_0010c1e6;
    }
    if ((uVar9 & 0x202) == 0x202) {
      cVar1 = FUN_00106c80(0x14);
      lVar4 = 0x140;
      if (cVar1 != '\0') goto LAB_0010c21e;
    }
    if ((uVar9 & 2) != 0) {
      cVar1 = FUN_00106c80(0x13);
      lVar4 = 0x130;
      if (cVar1 != '\0') goto LAB_0010c21e;
    }
    lVar4 = 0x60;
    if ((uVar9 & 0x200) != 0) {
      cVar1 = FUN_00106c80(0x12);
      lVar4 = (-(ulong)(cVar1 == '\0') & 0xffffffffffffff40) + 0x120;
    }
  }
  else {
    __s = (char *)param_1[1];
    if (DAT_00126312 == '\0') goto LAB_0010c112;
    uVar9 = *(uint *)((long)param_1 + 0xac);
    if (*(char *)((long)param_1 + 0xb9) == '\0') {
      cVar1 = FUN_00106c80(0xc);
      if (cVar1 == '\0') {
        uVar2 = 0xffffffff;
        goto LAB_0010c1b4;
      }
      lVar4 = 0xc0;
      goto LAB_0010c21e;
    }
    cVar1 = *(char *)(param_1 + 0x17);
    uVar2 = 0;
    if (cVar1 != '\0') goto LAB_0010c303;
LAB_0010c1c5:
    uVar9 = *(uint *)(&DAT_0011a6a0 + (ulong)*(uint *)(param_1 + 0x15) * 4);
    uVar8 = (ulong)uVar9;
    cVar1 = uVar9 == 7;
    if (uVar9 == 5) {
LAB_0010c3b0:
      sVar5 = strlen(__s);
      for (puVar6 = DAT_00126308; puVar6 != (ulong *)0x0; puVar6 = (ulong *)puVar6[4]) {
        if ((*puVar6 <= sVar5) &&
           (iVar3 = FUN_0010eeb0(__s + (sVar5 - *puVar6),puVar6[1]), iVar3 == 0)) {
          puVar6 = puVar6 + 2;
          goto LAB_0010c228;
        }
      }
      lVar4 = 0x50;
    }
    else {
LAB_0010c1e6:
      if ((uVar2 == 0) && (cVar1 != '\0')) {
        lVar4 = 0xd0;
        if (DAT_00126390 == '\0') {
          cVar1 = FUN_00106c80(0xd);
          lVar4 = (-(ulong)(cVar1 == '\0') & 0xffffffffffffffa0) + 0xd0;
        }
      }
      else {
        lVar4 = uVar8 << 4;
      }
    }
  }
LAB_0010c21e:
  puVar6 = (ulong *)((long)&DAT_00125060 + lVar4);
LAB_0010c228:
  if (puVar6[1] == 0) {
    cVar1 = FUN_00106c80(4);
    if (cVar1 == '\0') {
LAB_0010c112:
      lVar4 = FUN_0010bcf0(__s,DAT_001262d0,*(undefined4 *)((long)param_1 + 0xc4),0,param_2 ^ 1,
                           param_3,param_1[2]);
      FUN_001084f0();
      return lVar4;
    }
    puVar6 = (ulong *)0x0;
  }
  lVar4 = FUN_0010bcf0(__s,DAT_001262d0,*(undefined4 *)((long)param_1 + 0xc4),puVar6,param_2 ^ 1,
                       param_3,param_1[2]);
  FUN_001084f0();
  if (DAT_00125088 == 0) {
    FUN_00107880(&DAT_00125060);
    FUN_00107880(&DAT_00125090);
    FUN_00107880(&DAT_00125070);
  }
  else {
    FUN_00107880(&DAT_00125080);
  }
  if (DAT_001262b0 == 0) {
    return lVar4;
  }
  uVar8 = lVar4 + -1 + param_4;
  if (param_4 / DAT_001262b0 == uVar8 / DAT_001262b0) {
    return lVar4;
  }
  FUN_00107880(&DAT_001251d0,param_4 / DAT_001262b0,uVar8 % DAT_001262b0);
  return lVar4;
}




// Function: print_with_separator @ 0xc7c0

void print_with_separator(char param_1)

{
  ulong uVar1;
  undefined8 uVar2;
  char *pcVar3;
  byte *pbVar4;
  ulong uVar5;
  byte bVar6;
  ulong uVar7;
  ulong uVar8;
  
  if (DAT_001263b0 != 0) {
    uVar7 = 0;
    uVar8 = 0;
    do {
      uVar2 = *(undefined8 *)(DAT_001263a0 + uVar7 * 8);
      if (DAT_001262b0 == 0) {
        uVar5 = uVar8;
        if (uVar7 != 0) {
          uVar1 = uVar8 + 2;
LAB_0010c870:
          uVar5 = uVar1;
          uVar8 = uVar8 + 2;
          bVar6 = 0x20;
          goto LAB_0010c87a;
        }
      }
      else {
        uVar5 = FUN_00107e10(uVar2);
        if (uVar7 == 0) {
          uVar5 = uVar8 + uVar5;
        }
        else {
          uVar1 = uVar5 + uVar8 + 2;
          if ((DAT_001262b0 == 0) || ((uVar1 < DAT_001262b0 && (uVar8 <= -uVar5 - 3))))
          goto LAB_0010c870;
          uVar8 = 0;
          bVar6 = DAT_00125019;
LAB_0010c87a:
          pcVar3 = stdout->_IO_write_ptr;
          if (pcVar3 < stdout->_IO_write_end) {
            stdout->_IO_write_ptr = pcVar3 + 1;
            *pcVar3 = param_1;
          }
          else {
            __overflow(stdout,(int)param_1);
          }
          pbVar4 = (byte *)stdout->_IO_write_ptr;
          if (pbVar4 < stdout->_IO_write_end) {
            stdout->_IO_write_ptr = (char *)(pbVar4 + 1);
            *pbVar4 = bVar6;
          }
          else {
            __overflow(stdout,(uint)bVar6);
          }
        }
      }
      uVar7 = uVar7 + 1;
      FUN_0010c590(uVar2,uVar8);
      uVar8 = uVar5;
    } while (uVar7 < DAT_001263b0);
  }
  bVar6 = DAT_00125019;
  pbVar4 = (byte *)stdout->_IO_write_ptr;
  if (stdout->_IO_write_end <= pbVar4) {
    __overflow(stdout,(uint)DAT_00125019);
    return;
  }
  stdout->_IO_write_ptr = (char *)(pbVar4 + 1);
  *pbVar4 = bVar6;
  return;
}




// Function: print_long_format @ 0xc980

void print_long_format(long param_1)

{
  char cVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  byte *pbVar4;
  byte bVar5;
  int iVar6;
  int iVar7;
  uint uVar8;
  undefined1 *puVar9;
  long lVar10;
  size_t sVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined *puVar14;
  char *pcVar15;
  char *pcVar16;
  char *pcVar17;
  long lVar18;
  int iVar19;
  char *pcVar20;
  char *pcVar21;
  long in_FS_OFFSET;
  bool bVar22;
  undefined8 local_1340;
  ulong local_1338;
  ulong local_1330;
  undefined1 local_1328 [16];
  int local_1318;
  undefined1 local_12e8 [16];
  int local_12d8;
  char local_12a4;
  undefined8 local_12a3;
  undefined2 local_129b;
  undefined1 local_1299;
  undefined1 local_1298 [32];
  undefined1 local_1278 [1008];
  char local_e88 [3656];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (*(char *)(param_1 + 0xb8) == '\0') {
    local_1299 = 0;
    local_129b = 0x3f3f;
    local_12a4 = "?pcdb-lswd"[*(uint *)(param_1 + 0xa8)];
    local_12a3 = 0x3f3f3f3f3f3f3f3f;
  }
  else {
    FUN_0010fba0(param_1 + 0x18,&local_12a4);
  }
  if (DAT_00126364 == '\0') {
    local_129b = local_129b & 0xff;
LAB_0010c9e6:
    if (DAT_00126334 == 2) goto LAB_0010ccf8;
LAB_0010c9f5:
    if (DAT_00126334 < 3) {
      if (DAT_00126334 == 0) {
        local_1330 = *(ulong *)(param_1 + 0x78);
        local_1338 = *(ulong *)(param_1 + 0x70);
        bVar22 = true;
      }
      else {
        local_1330 = *(ulong *)(param_1 + 0x88);
        local_1338 = *(ulong *)(param_1 + 0x80);
        bVar22 = true;
      }
    }
    else {
      if (DAT_00126334 != 3) {
                    /* WARNING: Subroutine does not return */
        abort();
      }
      local_1338 = *(ulong *)(param_1 + 0x70);
      local_1330 = *(ulong *)(param_1 + 0x78);
      bVar22 = (local_1338 & local_1330) != 0xffffffffffffffff;
    }
  }
  else {
    if (*(int *)(param_1 + 0xbc) == 1) {
      local_129b = CONCAT11(0x2e,(undefined1)local_129b);
      goto LAB_0010c9e6;
    }
    if (*(int *)(param_1 + 0xbc) != 2) goto LAB_0010c9e6;
    local_129b = CONCAT11(0x2b,(undefined1)local_129b);
    if (DAT_00126334 != 2) goto LAB_0010c9f5;
LAB_0010ccf8:
    local_1330 = *(ulong *)(param_1 + 0x68);
    local_1338 = *(ulong *)(param_1 + 0x60);
    bVar22 = true;
  }
  pcVar16 = local_e88;
  if (DAT_001262fc != '\0') {
    puVar9 = &DAT_0011ba80;
    if ((*(char *)(param_1 + 0xb8) != '\0') && (*(long *)(param_1 + 0x20) != 0)) {
      puVar9 = (undefined1 *)FUN_00112720(*(long *)(param_1 + 0x20),local_1278);
    }
    iVar7 = __sprintf_chk(local_e88,1,0xe3b,&DAT_0011bb2a,DAT_00126360,puVar9);
    pcVar16 = local_e88 + iVar7;
  }
  if (DAT_0012632c != '\0') {
    pcVar21 = "?";
    if (*(char *)(param_1 + 0xb8) != '\0') {
      pcVar21 = (char *)FUN_00111550(*(undefined8 *)(param_1 + 0x58),local_1278,DAT_00126328,0x200,
                                     DAT_00126320);
    }
    iVar7 = DAT_0012635c;
    iVar6 = FUN_00112ec0(pcVar21,0);
    pcVar20 = pcVar16;
    if (0 < iVar7 - iVar6) {
      pcVar17 = pcVar16;
      do {
        pcVar15 = pcVar17 + 1;
        *pcVar17 = ' ';
        pcVar20 = pcVar16 + (iVar7 - iVar6);
        pcVar17 = pcVar15;
      } while (pcVar15 != pcVar16 + (iVar7 - iVar6));
    }
    do {
      pcVar17 = pcVar20;
      pcVar16 = pcVar17 + 1;
      cVar1 = *pcVar21;
      *pcVar17 = cVar1;
      pcVar20 = pcVar16;
      pcVar21 = pcVar21 + 1;
    } while (cVar1 != '\0');
    *pcVar17 = ' ';
  }
  puVar9 = &DAT_0011ba80;
  if (*(char *)(param_1 + 0xb8) != '\0') {
    puVar9 = (undefined1 *)FUN_00112720(*(undefined8 *)(param_1 + 0x28),local_1278);
  }
  iVar7 = __sprintf_chk(pcVar16,1,0xffffffffffffffff,"%s %*s ",&local_12a4,DAT_00126358,puVar9);
  pcVar16 = pcVar16 + iVar7;
  if (DAT_00126318 != '\0') {
    sVar11 = strlen("  ");
    FUN_00107430(&DAT_0011bb22,sVar11);
  }
  if ((((DAT_00125029 != '\0') || (DAT_00125028 != '\0')) || (DAT_0012632e != '\0')) ||
     (DAT_00126365 != '\0')) {
    FUN_00107430(local_e88,(long)pcVar16 - (long)local_e88);
    uVar2 = DAT_00126350;
    if (DAT_00125029 != '\0') {
      puVar9 = &DAT_0011ba80;
      uVar3 = *(undefined4 *)(param_1 + 0x34);
      if ((*(char *)(param_1 + 0xb8) != '\0') && (puVar9 = (undefined1 *)0x0, DAT_0012632d == '\0'))
      {
        puVar9 = (undefined1 *)FUN_00112340(uVar3);
      }
      FUN_001092c0(puVar9,uVar3,uVar2);
    }
    uVar3 = DAT_0012634c;
    uVar2 = DAT_00126348;
    if (DAT_00125028 != '\0') {
      puVar9 = &DAT_0011ba80;
      uVar2 = *(undefined4 *)(param_1 + 0x38);
      if ((*(char *)(param_1 + 0xb8) != '\0') && (puVar9 = (undefined1 *)0x0, DAT_0012632d == '\0'))
      {
        puVar9 = (undefined1 *)FUN_001124e0(uVar2);
      }
      FUN_001092c0(puVar9,uVar2,uVar3);
      uVar2 = DAT_00126348;
    }
    DAT_00126348 = uVar2;
    if (DAT_0012632e != '\0') {
      puVar9 = &DAT_0011ba80;
      uVar3 = *(undefined4 *)(param_1 + 0x34);
      if ((*(char *)(param_1 + 0xb8) != '\0') && (puVar9 = (undefined1 *)0x0, DAT_0012632d == '\0'))
      {
        puVar9 = (undefined1 *)FUN_00112340(uVar3);
      }
      FUN_001092c0(puVar9,uVar3,uVar2);
    }
    pcVar16 = local_e88;
    if (DAT_00126365 != '\0') {
      FUN_001092c0(*(undefined8 *)(param_1 + 0xb0),0,DAT_00126354);
    }
  }
  if (*(char *)(param_1 + 0xb8) == '\0') {
    pcVar21 = "?";
LAB_0010cb76:
    iVar7 = DAT_0012633c;
    iVar6 = FUN_00112ec0(pcVar21,0);
    pcVar20 = pcVar16;
    if (0 < iVar7 - iVar6) {
      pcVar17 = pcVar16;
      do {
        pcVar15 = pcVar17 + 1;
        *pcVar17 = ' ';
        pcVar17 = pcVar15;
        pcVar20 = pcVar16 + (iVar7 - iVar6);
      } while (pcVar15 != pcVar16 + (iVar7 - iVar6));
    }
    do {
      pcVar16 = pcVar20;
      cVar1 = *pcVar21;
      pcVar21 = pcVar21 + 1;
      pcVar20 = pcVar16 + 1;
      *pcVar16 = cVar1;
    } while (cVar1 != '\0');
    *pcVar16 = ' ';
  }
  else {
    if ((*(uint *)(param_1 + 0x30) & 0xb000) != 0x2000) {
      pcVar21 = (char *)FUN_00111550(*(undefined8 *)(param_1 + 0x48),local_1278,DAT_0012631c,1,
                                     DAT_00125020);
      goto LAB_0010cb76;
    }
    iVar19 = DAT_0012633c - (DAT_00126344 + 2 + DAT_00126340);
    uVar12 = FUN_00112720((uint)((*(ulong *)(param_1 + 0x40) >> 0x14) << 8) |
                          (uint)*(ulong *)(param_1 + 0x40) & 0xff,local_1278);
    iVar7 = DAT_00126340;
    uVar13 = FUN_00112720((uint)((ulong)*(undefined8 *)(param_1 + 0x40) >> 0x20) & 0xfffff000 |
                          (uint)((ulong)*(undefined8 *)(param_1 + 0x40) >> 8) & 0xfff,local_1298);
    iVar6 = 0;
    if (-1 < iVar19) {
      iVar6 = iVar19;
    }
    iVar7 = __sprintf_chk(pcVar16,1,0xffffffffffffffff,&DAT_0011bb25,iVar6 + DAT_00126344,uVar13,
                          iVar7,uVar12);
    pcVar20 = pcVar16 + iVar7;
  }
  *pcVar20 = '\x01';
  if ((*(char *)(param_1 + 0xb8) == '\0') || (!bVar22)) {
LAB_0010cbdc:
    puVar9 = &DAT_0011ba80;
LAB_0010cbe3:
    if (DAT_00125014 < 0) {
      local_1340 = 0;
      lVar10 = FUN_00117c60(DAT_001262a8,&local_1340,local_12e8);
      if (lVar10 != 0) {
        puVar14 = PTR_DAT_00125040;
        if (DAT_001253c8 != '\0') {
          puVar14 = &DAT_001253e0 + (long)local_12d8 * 0x80;
        }
        lVar10 = FUN_00114d80(local_1278,0x3e9,puVar14,local_12e8,DAT_001262a8,0);
        if (lVar10 != 0) {
          DAT_00125014 = FUN_00112ca0(local_1278,lVar10,0);
        }
      }
      if (DAT_00125014 < 0) {
        DAT_00125014 = 0;
      }
    }
    iVar7 = __sprintf_chk(pcVar20,1,0xffffffffffffffff,&DAT_0011bb2a,DAT_00125014,puVar9);
    pcVar20 = pcVar20 + iVar7;
  }
  else {
    lVar10 = FUN_00117c60(DAT_001262a8,&local_1338,local_1328);
    if (lVar10 == 0) {
LAB_0010d1c8:
      if (*pcVar20 != '\0') {
        if (*(char *)(param_1 + 0xb8) == '\0') goto LAB_0010cbdc;
        puVar9 = (undefined1 *)FUN_00112680(local_1338,local_1298);
        goto LAB_0010cbe3;
      }
    }
    else {
      bVar22 = (long)local_1330 < DAT_00126378;
      uVar8 = (uint)(DAT_00126378 < (long)local_1330);
      iVar7 = (uint)bVar22 - (uint)(DAT_00126378 < (long)local_1330);
      if ((int)(iVar7 + ((uint)((long)local_1338 < DAT_00126370) -
                        (uint)(DAT_00126370 < (long)local_1338)) * 2) < 0) {
        FUN_00110200(&DAT_00126370);
        bVar22 = (long)local_1330 < DAT_00126378;
        uVar8 = (uint)(DAT_00126378 < (long)local_1330);
        iVar7 = bVar22 - uVar8;
      }
      uVar8 = (uVar8 - bVar22) +
              ((uint)(DAT_00126370 < (long)local_1338) - (uint)((long)local_1338 < DAT_00126370)) *
              2 & iVar7 + ((uint)((long)local_1338 < DAT_00126370 + -0xf0c2ac) -
                          (uint)(DAT_00126370 + -0xf0c2ac < (long)local_1338)) * 2;
      if (DAT_001253c8 == '\0') {
        puVar14 = (&PTR_DAT_00125040)[uVar8 >> 0x1f];
      }
      else {
        puVar14 = &DAT_001253e0 + ((ulong)((int)uVar8 >> 0x1f & 0xc) + (long)local_1318) * 0x80;
      }
      lVar10 = FUN_00114d80(pcVar20,0x3e9,puVar14,local_1328,DAT_001262a8);
      if (lVar10 == 0) goto LAB_0010d1c8;
      pcVar20 = pcVar20 + lVar10;
    }
    *pcVar20 = ' ';
    pcVar20 = pcVar20 + 1;
  }
  lVar18 = (long)pcVar20 - (long)local_e88;
  FUN_00107430(local_e88,lVar18);
  lVar10 = FUN_0010c0e0(param_1,0,&DAT_001261a0,lVar18);
  if (*(int *)(param_1 + 0xa8) == 6) {
    if (*(long *)(param_1 + 8) == 0) goto LAB_0010cc5d;
    sVar11 = strlen(" -> ");
    FUN_00107430(&DAT_0011bb2f,sVar11);
    FUN_0010c0e0(param_1,1,0,lVar18 + 4 + lVar10);
    if (DAT_00126314 == 0) goto LAB_0010cc5d;
    bVar5 = FUN_001075b0(1,*(undefined4 *)(param_1 + 0xac),0);
  }
  else {
    if (DAT_00126314 == 0) goto LAB_0010cc5d;
    bVar5 = FUN_001075b0(*(undefined1 *)(param_1 + 0xb8),*(undefined4 *)(param_1 + 0x30));
  }
  if (bVar5 != 0) {
    DAT_001261f8 = DAT_001261f8 + 1;
    pbVar4 = (byte *)stdout->_IO_write_ptr;
    if (pbVar4 < stdout->_IO_write_end) {
      stdout->_IO_write_ptr = (char *)(pbVar4 + 1);
      *pbVar4 = bVar5;
    }
    else {
      __overflow(stdout,(uint)bVar5);
    }
  }
LAB_0010cc5d:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: print_current_files @ 0xd420

void print_current_files(void)

{
  long lVar1;
  byte *pbVar2;
  byte bVar3;
  char cVar4;
  ulong uVar5;
  long lVar6;
  ulong uVar7;
  ulong uVar8;
  long lVar9;
  undefined8 uVar10;
  long lVar11;
  long lVar12;
  ulong local_40;
  
  switch(DAT_00126338) {
  case 0:
    uVar5 = 0;
    if (DAT_001263b0 == 0) {
      return;
    }
    do {
      if ((DAT_00126312 != '\0') && (cVar4 = FUN_00106c80(4), cVar4 != '\0')) {
        FUN_00107880(&DAT_00125060);
        FUN_00107880(&DAT_001250a0);
        FUN_00107880(&DAT_00125070);
      }
      FUN_0010c980(DAT_001263a0[uVar5]);
      bVar3 = DAT_00125019;
      DAT_001261f8 = DAT_001261f8 + 1;
      pbVar2 = (byte *)stdout->_IO_write_ptr;
      if (pbVar2 < stdout->_IO_write_end) {
        stdout->_IO_write_ptr = (char *)(pbVar2 + 1);
        *pbVar2 = bVar3;
      }
      else {
        __overflow(stdout,(uint)DAT_00125019);
      }
      uVar5 = uVar5 + 1;
    } while (uVar5 < DAT_001263b0);
    return;
  case 1:
    uVar5 = 0;
    if (DAT_001263b0 != 0) {
      do {
        FUN_0010c590(DAT_001263a0[uVar5],0);
        bVar3 = DAT_00125019;
        pbVar2 = (byte *)stdout->_IO_write_ptr;
        if (pbVar2 < stdout->_IO_write_end) {
          stdout->_IO_write_ptr = (char *)(pbVar2 + 1);
          *pbVar2 = bVar3;
        }
        else {
          __overflow(stdout,(uint)DAT_00125019);
        }
        uVar5 = uVar5 + 1;
      } while (uVar5 < DAT_001263b0);
    }
    return;
  case 2:
    if (DAT_001262b0 != 0) {
      uVar5 = FUN_00107ff0(1);
      local_40 = 0;
      lVar1 = DAT_00126208 + uVar5 * 0x18;
      uVar5 = (DAT_001263b0 / uVar5 + 1) - (ulong)(DAT_001263b0 % uVar5 == 0);
      if (uVar5 == 0) {
        return;
      }
      do {
        lVar6 = 0;
        uVar8 = local_40;
        lVar11 = 0;
        while( true ) {
          uVar10 = DAT_001263a0[uVar8];
          lVar12 = FUN_00107e10();
          lVar9 = *(long *)(*(long *)(lVar1 + -8) + lVar6);
          lVar6 = lVar6 + 8;
          FUN_0010c590(uVar10,lVar11);
          bVar3 = DAT_00125019;
          uVar8 = uVar8 + uVar5;
          if (DAT_001263b0 <= uVar8) break;
          lVar9 = lVar9 + lVar11;
          FUN_00107360(lVar12 + lVar11,lVar9);
          lVar11 = lVar9;
        }
        pbVar2 = (byte *)stdout->_IO_write_ptr;
        if (pbVar2 < stdout->_IO_write_end) {
          stdout->_IO_write_ptr = (char *)(pbVar2 + 1);
          *pbVar2 = bVar3;
        }
        else {
          __overflow(stdout,(uint)DAT_00125019);
        }
        local_40 = local_40 + 1;
      } while (uVar5 != local_40);
      return;
    }
    break;
  case 3:
    if (DAT_001262b0 != 0) {
      uVar5 = FUN_00107ff0(0);
      lVar1 = DAT_00126208 + -0x18 + uVar5 * 0x18;
      uVar10 = *DAT_001263a0;
      lVar6 = FUN_00107e10(uVar10);
      lVar11 = **(long **)(lVar1 + 0x10);
      FUN_0010c590(uVar10,0);
      if (1 < DAT_001263b0) {
        uVar8 = 1;
        lVar9 = 0;
        do {
          bVar3 = DAT_00125019;
          uVar7 = uVar8 % uVar5;
          if (uVar7 == 0) {
            pbVar2 = (byte *)stdout->_IO_write_ptr;
            if (pbVar2 < stdout->_IO_write_end) {
              lVar12 = 0;
              stdout->_IO_write_ptr = (char *)(pbVar2 + 1);
              *pbVar2 = bVar3;
            }
            else {
              lVar12 = 0;
              __overflow(stdout,(uint)DAT_00125019);
            }
          }
          else {
            lVar12 = lVar11 + lVar9;
            FUN_00107360(lVar9 + lVar6,lVar12);
          }
          uVar10 = DAT_001263a0[uVar8];
          uVar8 = uVar8 + 1;
          FUN_0010c590(uVar10,lVar12);
          lVar6 = FUN_00107e10(uVar10);
          lVar11 = *(long *)(*(long *)(lVar1 + 0x10) + uVar7 * 8);
          lVar9 = lVar12;
        } while (uVar8 < DAT_001263b0);
      }
      bVar3 = DAT_00125019;
      pbVar2 = (byte *)stdout->_IO_write_ptr;
      if (pbVar2 < stdout->_IO_write_end) {
        stdout->_IO_write_ptr = (char *)(pbVar2 + 1);
        *pbVar2 = bVar3;
        return;
      }
      __overflow(stdout,(uint)DAT_00125019);
      return;
    }
    break;
  case 4:
    uVar10 = 0x2c;
    goto LAB_0010d555;
  default:
    return;
  }
  uVar10 = 0x20;
LAB_0010d555:
  FUN_0010c7c0(uVar10);
  return;
}




// Function: print_dir @ 0xd820

void print_dir(char *param_1,char *param_2,undefined1 param_3)

{
  char cVar1;
  byte bVar2;
  int iVar3;
  int *piVar4;
  DIR *__dirp;
  undefined8 *puVar5;
  undefined8 *puVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  size_t sVar9;
  dirent *pdVar10;
  void *__ptr;
  char *pcVar11;
  char *__s;
  size_t sVar12;
  long lVar13;
  undefined4 uVar14;
  long in_FS_OFFSET;
  bool bVar15;
  long local_378;
  undefined8 local_368;
  undefined8 local_360;
  undefined1 local_2d7 [663];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  piVar4 = __errno_location();
  *piVar4 = 0;
  __dirp = opendir(param_1);
  if (__dirp == (DIR *)0x0) {
    uVar7 = dcgettext(0,"cannot open directory %s",5);
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      FUN_00107540(param_3,uVar7,param_1);
      return;
    }
    goto LAB_0010defa;
  }
  if (DAT_001263c8 != 0) {
    iVar3 = dirfd(__dirp);
    if (iVar3 < 0) {
      iVar3 = FUN_00107050(0xffffff9c,param_1,&local_368,0,0x100);
      uVar7 = local_368;
      uVar8 = local_360;
    }
    else {
      iVar3 = FUN_00107050(iVar3,&DAT_0011bb19,&local_368,0x1000,0x100);
      uVar7 = local_368;
      uVar8 = local_360;
    }
    if (iVar3 < 0) {
      uVar7 = dcgettext(0,"cannot determine device and inode of %s",5);
      FUN_00107540(param_3,uVar7,param_1);
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        closedir(__dirp);
        return;
      }
      goto LAB_0010defa;
    }
    puVar5 = (undefined8 *)FUN_00118590(0x10);
    lVar13 = DAT_001263c8;
    puVar5[1] = uVar7;
    *puVar5 = uVar8;
    puVar6 = (undefined8 *)FUN_001112c0(lVar13,puVar5);
    if (puVar6 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
      FUN_00118a90();
    }
    if (puVar5 != puVar6) {
      free(puVar5);
      uVar7 = FUN_00117150(0,3,param_1);
      uVar8 = dcgettext(0,"%s: not listing already-listed directory",5);
      error(0,0,uVar8,uVar7);
      closedir(__dirp);
      DAT_00126210 = 2;
      goto LAB_0010d955;
    }
    puVar5 = DAT_001260f8;
    if ((ulong)(DAT_00126100 - (long)DAT_001260f8) < 0x10) {
      _obstack_newchunk(&DAT_001260e0,0x10);
      puVar5 = DAT_001260f8;
    }
    DAT_001260f8 = puVar5 + 2;
    puVar5[1] = uVar7;
    *puVar5 = uVar8;
  }
  FUN_00108730();
  cVar1 = DAT_00126318;
  if (DAT_001262f6 == '\0') {
    if (DAT_001262b8 == '\0') goto LAB_0010da47;
    if (DAT_00125010 == '\0') goto LAB_0010db04;
LAB_0010d9cf:
    DAT_00125010 = '\0';
    cVar1 = DAT_00126318;
  }
  else {
    if (DAT_00125010 != '\0') goto LAB_0010d9cf;
LAB_0010db04:
    DAT_001261f8 = DAT_001261f8 + 1;
    pcVar11 = stdout->_IO_write_ptr;
    if (stdout->_IO_write_end <= pcVar11) {
      __overflow(stdout,10);
      goto LAB_0010d9cf;
    }
    DAT_00125010 = '\0';
    stdout->_IO_write_ptr = pcVar11 + 1;
    *pcVar11 = '\n';
  }
  if (cVar1 != '\0') {
    sVar9 = strlen("  ");
    FUN_00107430(&DAT_0011bb22,sVar9);
  }
  __ptr = (void *)0x0;
  if ((DAT_00126311 != '\0') && (__ptr = (void *)FUN_0010ef30(param_1,2), __ptr == (void *)0x0)) {
    uVar7 = dcgettext(0,"error canonicalizing %s",5);
    FUN_00107540(param_3,uVar7,param_1);
  }
  if (param_2 == (char *)0x0) {
    param_2 = param_1;
  }
  FUN_0010bcf0(param_2,DAT_001262c8,0xffffffff,0,1,&DAT_00126140,__ptr);
  free(__ptr);
  sVar9 = strlen(":\n");
  FUN_00107430(":\n",sVar9);
LAB_0010da47:
  local_378 = 0;
  do {
    *piVar4 = 0;
    pdVar10 = readdir(__dirp);
    if (pdVar10 == (dirent *)0x0) {
      if (*piVar4 == 0) break;
      uVar7 = dcgettext(0,"reading directory %s",5);
      FUN_00107540(param_3,uVar7,param_1);
      if (*piVar4 != 0x4b) break;
    }
    else {
      pcVar11 = pdVar10->d_name;
      puVar5 = DAT_001262e8;
      if (DAT_001262f0 != 2) {
        if (pdVar10->d_name[0] == '.') {
          if ((DAT_001262f0 == 0) ||
             (pdVar10->d_name[(ulong)(pdVar10->d_name[1] == '.') + 1] == '\0')) goto LAB_0010dae0;
        }
        else {
          puVar6 = DAT_001262e0;
          if (DAT_001262f0 == 0) {
            for (; puVar5 = DAT_001262e8, puVar6 != (undefined8 *)0x0;
                puVar6 = (undefined8 *)puVar6[1]) {
              iVar3 = fnmatch((char *)*puVar6,pcVar11,4);
              if (iVar3 == 0) goto LAB_0010dae0;
            }
          }
        }
      }
      for (; puVar5 != (undefined8 *)0x0; puVar5 = (undefined8 *)puVar5[1]) {
        iVar3 = fnmatch((char *)*puVar5,pcVar11,4);
        if (iVar3 == 0) goto LAB_0010dae0;
      }
      uVar14 = 0;
      bVar2 = pdVar10->d_type - 1;
      if (bVar2 < 0xe) {
        uVar14 = *(undefined4 *)(&DAT_0011a600 + (ulong)bVar2 * 4);
      }
      lVar13 = FUN_001096f0(pcVar11,uVar14,0,param_1);
      local_378 = local_378 + lVar13;
      if ((((DAT_00126338 == 1) && (DAT_00126330 == 6)) && (DAT_0012632c == '\0')) &&
         (DAT_001262f6 == '\0')) {
        FUN_00108e30();
        FUN_0010d420();
        FUN_00108730();
      }
    }
LAB_0010dae0:
    FUN_001084f0();
  } while( true );
  iVar3 = closedir(__dirp);
  if (iVar3 != 0) {
    uVar7 = dcgettext(0,"closing directory %s",5);
    FUN_00107540(param_3,uVar7,param_1);
  }
  FUN_00108e30();
  if (DAT_001262f6 != '\0') {
    FUN_00109510(param_1,0);
  }
  if ((DAT_00126338 == 0) || (DAT_0012632c != '\0')) {
    pcVar11 = (char *)FUN_00111550(local_378,local_2d7,DAT_00126328,0x200,DAT_00126320);
    sVar9 = strlen(pcVar11);
    cVar1 = DAT_00125019;
    pcVar11[-1] = ' ';
    bVar15 = DAT_00126318 != '\0';
    pcVar11[sVar9] = cVar1;
    if (bVar15) {
      sVar12 = strlen("  ");
      FUN_00107430(&DAT_0011bb22,sVar12);
    }
    __s = (char *)dcgettext(0,"total",5);
    sVar12 = strlen(__s);
    FUN_00107430(__s,sVar12);
    FUN_00107430(pcVar11 + -1,pcVar11 + sVar9 + (1 - (long)(pcVar11 + -1)));
  }
  if (DAT_001263b0 != 0) {
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      FUN_0010d420();
      return;
    }
    goto LAB_0010defa;
  }
LAB_0010d955:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
LAB_0010defa:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: usage @ 0xdf00

void usage(int param_1)

{
  FILE *pFVar1;
  int iVar2;
  undefined8 uVar3;
  char *pcVar4;
  undefined8 uVar5;
  char *pcVar6;
  undefined **ppuVar7;
  char *__s1;
  char *pcVar8;
  long in_FS_OFFSET;
  undefined *local_b8;
  char *local_b0;
  char *local_a8 [4];
  char *local_88;
  char *local_80;
  char *local_78;
  char *local_70;
  undefined *local_68;
  char *local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_40;
  
  uVar5 = DAT_00126400;
  ppuVar7 = &local_b8;
  local_40 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  if (param_1 == 0) {
    uVar3 = dcgettext(0,"Usage: %s [OPTION]... [FILE]...\n",5);
    __printf_chk(1,uVar3,uVar5);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "List information about the FILEs (the current directory by default).\nSort entries alphabetically if none of -cftuvSUX nor --sort is specified.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "\nMandatory arguments to long options are mandatory for short options too.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -a, --all                  do not ignore entries starting with .\n  -A, --almost-all           do not list implied . and ..\n      --author               with -l, print the author of each file\n  -b, --escape               print C-style escapes for nongraphic characters\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "      --block-size=SIZE      with -l, scale sizes by SIZE when printing them;\n                             e.g., \'--block-size=M\'; see SIZE format below\n\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -B, --ignore-backups       do not list implied entries ending with ~\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -c                         with -lt: sort by, and show, ctime (time of last\n                             modification of file status information);\n                             with -l: show ctime and sort by name;\n                             otherwise: sort by ctime, newest first\n\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -C                         list entries by columns\n      --color[=WHEN]         color the output WHEN; more info below\n  -d, --directory            list directories themselves, not their contents\n  -D, --dired                generate output designed for Emacs\' dired mode\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -f                         list all entries in directory order\n  -F, --classify[=WHEN]      append indicator (one of */=>@|) to entries WHEN\n      --file-type            likewise, except do not append \'*\'\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "      --format=WORD          across -x, commas -m, horizontal -x, long -l,\n                             single-column -1, verbose -l, vertical -C\n\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"      --full-time            like -l --time-style=full-iso\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"  -g                         like -l, but do not list owner\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "      --group-directories-first\n                             group directories before files;\n                             can be augmented with a --sort option, but any\n                             use of --sort=none (-U) disables grouping\n\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -G, --no-group             in a long listing, don\'t print group names\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -h, --human-readable       with -l and -s, print sizes like 1K 234M 2G etc.\n      --si                   likewise, but use powers of 1000 not 1024\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -H, --dereference-command-line\n                             follow symbolic links listed on the command line\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "      --dereference-command-line-symlink-to-dir\n                             follow each command line symbolic link\n                             that points to a directory\n\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "      --hide=PATTERN         do not list implied entries matching shell PATTERN\n                             (overridden by -a or -A)\n\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"      --hyperlink[=WHEN]     hyperlink file names WHEN\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "      --indicator-style=WORD\n                             append indicator with style WORD to entry names:\n                             none (default), slash (-p),\n                             file-type (--file-type), classify (-F)\n\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -i, --inode                print the index number of each file\n  -I, --ignore=PATTERN       do not list implied entries matching shell PATTERN\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -k, --kibibytes            default to 1024-byte blocks for file system usage;\n                             used only with -s and per directory totals\n\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"  -l                         use a long listing format\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -L, --dereference          when showing file information for a symbolic\n                             link, show information for the file the link\n                             references rather than for the link itself\n\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -m                         fill width with a comma separated list of entries\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -n, --numeric-uid-gid      like -l, but list numeric user and group IDs\n  -N, --literal              print entry names without quoting\n  -o                         like -l, but do not list group information\n  -p, --indicator-style=slash\n                             append / indicator to directories\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -q, --hide-control-chars   print ? instead of nongraphic characters\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "      --show-control-chars   show nongraphic characters as-is (the default,\n                             unless program is \'ls\' and output is a terminal)\n\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -Q, --quote-name           enclose entry names in double quotes\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "      --quoting-style=WORD   use quoting style WORD for entry names:\n                             literal, locale, shell, shell-always,\n                             shell-escape, shell-escape-always, c, escape\n                             (overrides QUOTING_STYLE environment variable)\n\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -r, --reverse              reverse order while sorting\n  -R, --recursive            list subdirectories recursively\n  -s, --size                 print the allocated size of each file, in blocks\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"  -S                         sort by file size, largest first\n",5
                              );
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "      --sort=WORD            sort by WORD instead of name: none (-U), size (-S),\n                             time (-t), version (-v), extension (-X), width\n\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "      --time=WORD            change the default of using modification times;\n                               access time (-u): atime, access, use;\n                               change time (-c): ctime, status;\n                               birth time: birth, creation;\n                             with -l, WORD determines which time to show;\n                             with --sort=time, sort by WORD (newest first)\n\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "      --time-style=TIME_STYLE\n                             time/date format with -l; see TIME_STYLE below\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -t                         sort by time, newest first; see --time\n  -T, --tabsize=COLS         assume tab stops at each COLS instead of 8\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -u                         with -lt: sort by, and show, access time;\n                             with -l: show access time and sort by name;\n                             otherwise: sort by access time, newest first\n\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -U                         do not sort; list entries in directory order\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -v                         natural sort of (version) numbers within text\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "  -w, --width=COLS           set output width to COLS.  0 means no limit\n  -x                         list entries by lines instead of by columns\n  -X                         sort alphabetically by entry extension\n  -Z, --context              print any security context of each file\n      --zero                 end each output line with NUL, not newline\n  -1                         list one file per line\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"      --help        display this help and exit\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,"      --version     output version information and exit\n",5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "\nThe SIZE argument is an integer and optional unit (example: 10K is 10*1024).\nUnits are K,M,G,T,P,E,Z,Y (powers of 1024) or KB,MB,... (powers of 1000).\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "\nThe TIME_STYLE argument can be full-iso, long-iso, iso, locale, or +FORMAT.\nFORMAT is interpreted like in date(1).  If FORMAT is FORMAT1<newline>FORMAT2,\nthen FORMAT1 applies to non-recent files and FORMAT2 to recent files.\nTIME_STYLE prefixed with \'posix-\' takes effect only outside the POSIX locale.\nAlso the TIME_STYLE environment variable sets the default style to use.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "\nThe WHEN argument defaults to \'always\' and can also be \'auto\' or \'never\'.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "\nUsing color to distinguish file types is disabled both by default and\nwith --color=never.  With --color=auto, ls emits color codes only when\nstandard output is connected to a terminal.  The LS_COLORS environment\nvariable can change the settings.  Use the dircolors(1) command to set it.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "\nExit status:\n 0  if OK,\n 1  if minor problems (e.g., cannot access subdirectory),\n 2  if serious trouble (e.g., cannot access command-line argument).\n"
                               ,5);
    __s1 = "ls";
    fputs_unlocked(pcVar4,pFVar1);
    if ((DAT_001251e0 != 1) && (__s1 = "dir", DAT_001251e0 != 2)) {
      __s1 = "vdir";
    }
    local_58 = 0;
    local_b0 = "test invocation";
    local_a8[0] = "coreutils";
    pcVar4 = "[";
    local_a8[1] = "Multi-call invocation";
    local_88 = "sha256sum";
    local_a8[2] = "sha224sum";
    local_78 = "sha384sum";
    local_b8 = &DAT_0011bdf8;
    local_a8[3] = "sha2 utilities";
    local_80 = "sha2 utilities";
    local_70 = "sha2 utilities";
    local_68 = &DAT_0011bbda;
    local_60 = "sha2 utilities";
    local_50 = 0;
    do {
      iVar2 = strcmp(__s1,pcVar4);
      if (iVar2 == 0) break;
      pcVar4 = *(char **)((long)ppuVar7 + 0x10);
      ppuVar7 = (undefined **)((long)ppuVar7 + 0x10);
    } while (pcVar4 != (char *)0x0);
    pcVar4 = *(char **)((long)ppuVar7 + 8);
    if (*(char **)((long)ppuVar7 + 8) == (char *)0x0) {
      pcVar4 = __s1;
    }
    uVar5 = dcgettext(0,"\n%s online help: <%s>\n",5);
    __printf_chk(1,uVar5,"GNU coreutils","https://www.gnu.org/software/coreutils/");
    pcVar6 = setlocale(5,(char *)0x0);
    if (pcVar6 != (char *)0x0) {
      iVar2 = strncmp(pcVar6,"en_",3);
      pFVar1 = stdout;
      if (iVar2 != 0) {
        pcVar6 = (char *)dcgettext(0,
                                   "Report any translation bugs to <https://translationproject.org/team/>\n"
                                   ,5);
        fputs_unlocked(pcVar6,pFVar1);
      }
    }
    iVar2 = strcmp(__s1,"[");
    pcVar6 = "test";
    if (iVar2 != 0) {
      pcVar6 = __s1;
    }
    uVar5 = dcgettext(0,"Full documentation <%s%s>\n",5);
    pcVar8 = " invocation";
    __printf_chk(1,uVar5,"https://www.gnu.org/software/coreutils/",pcVar6);
    if (__s1 != pcVar4) {
      pcVar8 = "";
    }
    uVar5 = dcgettext(0,"or available locally via: info \'(coreutils) %s%s\'\n",5);
    __printf_chk(1,uVar5,pcVar4,pcVar8);
  }
  else {
    uVar3 = dcgettext(0,"Try \'%s --help\' for more information.\n",5);
    __fprintf_chk(stderr,1,uVar3,uVar5);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: raw_hasher @ 0x102f0

ulong raw_hasher(ulong param_1,ulong param_2)

{
  return (param_1 >> 3 | param_1 << 0x3d) % param_2;
}




// Function: raw_comparator @ 0x10310

bool raw_comparator(long param_1,long param_2)

{
  return param_2 == param_1;
}




// Function: check_tuning @ 0x10320

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 check_tuning(long param_1)

{
  float fVar1;
  float *pfVar2;
  float fVar3;
  
  pfVar2 = *(float **)(param_1 + 0x28);
  if (pfVar2 == (float *)&DAT_0011e310) {
    return 1;
  }
  fVar1 = pfVar2[2];
  if ((((_DAT_0011e324 < fVar1) && (fVar1 < DAT_0011e328)) && (_DAT_0011e32c < pfVar2[3])) &&
     (_DAT_0011e340 <= *pfVar2)) {
    fVar3 = *pfVar2 + _DAT_0011e324;
    if (((fVar3 < pfVar2[1]) && (pfVar2[1] <= DAT_0011e330)) && (fVar3 < fVar1)) {
      return 1;
    }
  }
  *(undefined **)(param_1 + 0x28) = &DAT_0011e310;
  return 0;
}




// Function: hash_find_entry @ 0x103b0

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
LAB_0011045f:
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
        goto LAB_0011045f;
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




// Function: transfer_entries @ 0x10600

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
            if (uVar4 <= uVar2) goto LAB_0011076b;
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
LAB_0011076b:
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




// Function: hash_print_statistics @ 0x10870

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
      if ((long *)param_1[1] <= plVar4) goto LAB_001108dd;
    }
    uVar5 = 1;
    for (lVar3 = plVar4[1]; lVar3 != 0; lVar3 = *(long *)(lVar3 + 8)) {
      uVar5 = uVar5 + 1;
    }
    if (uVar6 < uVar5) {
      uVar6 = uVar5;
    }
  }
LAB_001108dd:
  __fprintf_chk(param_2,1,"# entries:         %lu\n",param_1[4]);
  __fprintf_chk(param_2,1,"# buckets:         %lu\n",uVar1);
  __fprintf_chk(((double)uVar2 * _DAT_0011e340) / (double)uVar1,param_2,1,
                "# buckets used:    %lu (%.2f%%)\n",uVar2);
  __fprintf_chk(param_2,1,"max bucket length: %lu\n",uVar6);
  return;
}




// Function: hash_lookup @ 0x109d0

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




// Function: hash_get_next @ 0x10a90

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




// Function: hash_do_for_each @ 0x10b70

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




// Function: hash_initialize @ 0x10c60

undefined8 *
hash_initialize(undefined8 param_1,undefined *param_2,code *param_3,code *param_4,undefined8 param_5)

{
  char cVar1;
  undefined8 *__ptr;
  size_t __nmemb;
  void *pvVar2;
  
  if (param_3 == (code *)0x0) {
    param_3 = FUN_001102f0;
  }
  if (param_4 == (code *)0x0) {
    param_4 = FUN_00110310;
  }
  __ptr = malloc(0x50);
  if (__ptr != (undefined8 *)0x0) {
    if (param_2 == (undefined *)0x0) {
      param_2 = &DAT_0011e310;
    }
    __ptr[5] = param_2;
    cVar1 = FUN_00110320(__ptr);
    if (cVar1 != '\0') {
      __nmemb = FUN_001104c0(*(undefined4 *)(param_2 + 8),param_1,param_2[0x10]);
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




// Function: hash_clear @ 0x10d70

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
        if ((long *)param_1[1] <= plVar7) goto LAB_00110e0b;
      }
      plVar6 = plVar6 + 2;
    } while (plVar6 < (long *)param_1[1]);
  }
LAB_00110e0b:
  param_1[3] = 0;
  param_1[4] = 0;
  return;
}




// Function: hash_free @ 0x10e20

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
    if (plVar3 <= plVar5) goto LAB_00110ecc;
    do {
      while (lVar6 = *plVar5, plVar4 = plVar5, lVar6 == 0) {
        plVar5 = plVar5 + 2;
        if (plVar3 <= plVar5) goto LAB_00110e8f;
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
LAB_00110e8f:
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
LAB_00110ecc:
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




// Function: hash_rehash @ 0x10f10

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
  __nmemb = FUN_001104c0(*(undefined4 *)(param_1[5] + 8),param_2,*(undefined1 *)(param_1[5] + 0x10))
  ;
  if (__nmemb != 0) {
    if (param_1[2] == __nmemb) {
      uVar2 = 1;
      goto LAB_00111016;
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
      uVar2 = FUN_00110600(&local_88,param_1,0);
      if ((char)uVar2 == '\0') {
        param_1[9] = local_40;
        cVar1 = FUN_00110600(param_1,&local_88,1);
        if (cVar1 != '\0') {
          cVar1 = FUN_00110600(param_1,&local_88,0);
          if (cVar1 != '\0') {
            free(local_88);
            goto LAB_00111016;
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
      goto LAB_00111016;
    }
  }
  uVar2 = 0;
LAB_00111016:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_insert_if_absent @ 0x11090

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
  if (param_2 == 0) goto LAB_00111293;
  lVar2 = FUN_001103b0(param_1,param_2,&local_38,0);
  if (lVar2 != 0) {
    uVar6 = 0;
    if (param_3 != (long *)0x0) {
      *param_3 = lVar2;
    }
    goto LAB_001110de;
  }
  uVar5 = *(ulong *)(param_1 + 0x18);
  if ((long)uVar5 < 0) {
    uVar3 = *(ulong *)(param_1 + 0x10);
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)uVar5;
    if (-1 < (long)uVar3) goto LAB_0011111f;
LAB_001111a6:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)uVar3;
  }
  else {
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)(long)uVar5;
    uVar3 = *(ulong *)(param_1 + 0x10);
    if ((long)uVar3 < 0) goto LAB_001111a6;
LAB_0011111f:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)(long)uVar3;
  }
  if (*(float *)(*(long *)(param_1 + 0x28) + 8) * auVar8._0_4_ < auVar9._0_4_) {
    FUN_00110320(param_1);
    lVar2 = *(long *)(param_1 + 0x28);
    if (auVar9._0_4_ <= auVar8._0_4_ * *(float *)(lVar2 + 8)) goto LAB_0011113e;
    fVar7 = auVar8._0_4_ * *(float *)(lVar2 + 0xc);
    if (*(char *)(lVar2 + 0x10) == '\0') {
      fVar7 = fVar7 * *(float *)(lVar2 + 8);
    }
    if (fVar7 < _DAT_0011e334) {
      if (_DAT_0011e338 <= fVar7) {
        uVar5 = (long)(fVar7 - _DAT_0011e338) ^ 0x8000000000000000;
      }
      else {
        uVar5 = (ulong)fVar7;
      }
      cVar1 = FUN_00110f10(param_1,uVar5);
      if (cVar1 != '\0') {
        lVar2 = FUN_001103b0(param_1,param_2,&local_38,0);
        if (lVar2 != 0) {
LAB_00111293:
                    /* WARNING: Subroutine does not return */
          abort();
        }
        goto LAB_0011113e;
      }
    }
LAB_001111fd:
    uVar6 = 0xffffffff;
  }
  else {
LAB_0011113e:
    if (*local_38 == 0) {
      *local_38 = param_2;
      uVar6 = 1;
      *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
      *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
      goto LAB_001110de;
    }
    plVar4 = *(long **)(param_1 + 0x48);
    if (plVar4 == (long *)0x0) {
      plVar4 = malloc(0x10);
      if (plVar4 == (long *)0x0) goto LAB_001111fd;
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
LAB_001110de:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar6;
}




// Function: hash_insert @ 0x112c0

undefined8 hash_insert(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_00111090(param_1,param_2,&local_18);
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




// Function: hash_remove @ 0x11320

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
  lVar4 = FUN_001103b0(param_1,param_2,&local_28,1);
  if ((lVar4 == 0) || (*(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + -1, *local_28 != 0))
  goto LAB_00111363;
  uVar5 = *(long *)(param_1 + 0x18) - 1;
  *(ulong *)(param_1 + 0x18) = uVar5;
  if ((long)uVar5 < 0) {
    uVar6 = *(ulong *)(param_1 + 0x10);
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)uVar5;
    if (-1 < (long)uVar6) goto LAB_001113b0;
LAB_00111466:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)uVar6;
  }
  else {
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)(long)uVar5;
    uVar6 = *(ulong *)(param_1 + 0x10);
    if ((long)uVar6 < 0) goto LAB_00111466;
LAB_001113b0:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)(long)uVar6;
  }
  if (auVar9._0_4_ < **(float **)(param_1 + 0x28) * auVar8._0_4_) {
    FUN_00110320(param_1);
    pfVar1 = *(float **)(param_1 + 0x28);
    if (auVar9._0_4_ < *pfVar1 * auVar8._0_4_) {
      fVar7 = auVar8._0_4_ * pfVar1[1];
      if (*(char *)(pfVar1 + 4) == '\0') {
        fVar7 = fVar7 * pfVar1[2];
      }
      if (_DAT_0011e338 <= fVar7) {
        uVar5 = (long)(fVar7 - _DAT_0011e338) ^ 0x8000000000000000;
      }
      else {
        uVar5 = (ulong)fVar7;
      }
      cVar3 = FUN_00110f10(param_1,uVar5);
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
LAB_00111363:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar4;
}




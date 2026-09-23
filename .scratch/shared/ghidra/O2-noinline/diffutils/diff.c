// Function: main @ 0x4fa0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 main(int param_1,undefined8 *param_2)

{
  bool bVar1;
  bool bVar2;
  long lVar3;
  byte bVar4;
  char cVar5;
  int iVar6;
  undefined4 uVar7;
  int iVar8;
  intmax_t iVar9;
  size_t sVar10;
  ulong uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  long lVar17;
  char *pcVar18;
  ulong uVar19;
  undefined8 *puVar20;
  undefined8 *puVar21;
  int iVar22;
  ulong uVar23;
  ulong uVar24;
  long in_FS_OFFSET;
  bool bVar25;
  ulong local_88;
  long local_80;
  long local_58;
  long local_50;
  char *local_48;
  long local_40;
  
  uVar23 = (ulong)param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_58 = 0;
  local_50 = 0;
  DAT_001260b0 = 2;
  FUN_00113d30(*param_2);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  FUN_0010fa70(0);
  _DAT_00126600 = &DAT_00126500;
  _DAT_001265c0 = &DAT_001264c0;
  re_set_syntax(0x50a46);
  DAT_00126410 = FUN_00110050();
  DAT_00126180 = 0;
  FUN_00116390();
  bVar1 = false;
  local_88 = 0;
  bVar2 = false;
  local_80 = -1;
  uVar24 = DAT_00126498;
  iVar8 = -1;
LAB_001050a0:
  DAT_00126498 = uVar24;
  iVar22 = (int)uVar23;
  iVar6 = getopt_long(uVar23 & 0xffffffff,param_2,
                      "0123456789abBcC:dD:eEfF:hHiI:lL:nNpPqrsS:tTuU:vwW:x:X:yZ",
                      &PTR_s_binary_00125160,0);
  if (iVar6 == -1) {
    if (((DAT_0012657c == 1) && (pcVar18 = getenv("TERM"), pcVar18 != (char *)0x0)) &&
       (iVar8 = strcmp(pcVar18,"dumb"), iVar8 == 0)) {
      DAT_0012657c = 0;
    }
    if (DAT_00126580 == 0) {
      if (!bVar1) {
        FUN_00108d50(1);
        goto LAB_00105b6d;
      }
      FUN_00108d50(2);
      if (-1 < local_80) {
        if ((DAT_00126580 == 2) && (cVar5 = FUN_00110ab0(2), cVar5 == '\0')) {
          DAT_00126400 = "%a %b %e %T %Y";
        }
        else {
          DAT_00126400 = "%Y-%m-%d %H:%M:%S.%N %z";
        }
        goto LAB_001059d6;
      }
      DAT_00126570 = 3;
      if (DAT_00126580 == 2) {
        cVar5 = FUN_00110ab0(2);
        if (cVar5 == '\0') {
          DAT_00126400 = "%a %b %e %T %Y";
        }
        else {
          DAT_00126400 = "%Y-%m-%d %H:%M:%S.%N %z";
        }
      }
      else {
        DAT_00126400 = "%Y-%m-%d %H:%M:%S.%N %z";
      }
    }
    else {
LAB_00105b6d:
      if ((DAT_00126580 == 2) && (cVar5 = FUN_00110ab0(2), cVar5 == '\0')) {
        DAT_00126400 = "%a %b %e %T %Y";
      }
      else {
        DAT_00126400 = "%Y-%m-%d %H:%M:%S.%N %z";
      }
      if (-1 < local_80) {
LAB_001059d6:
        if ((DAT_00126580 - 2U < 2) &&
           ((DAT_00126570 < local_80 || ((local_80 != DAT_00126570 && (!bVar2)))))) {
          DAT_00126570 = local_80;
        }
      }
    }
    if (DAT_00126498 == 0) {
      DAT_00126498 = 8;
    }
    if (local_88 == 0) {
      if (DAT_001264a0 == '\0') {
        local_88 = 0x82;
        goto LAB_00105a1f;
      }
      local_88 = 0x43;
      DAT_00126430 = 0x3f;
      uVar24 = local_88;
    }
    else {
      uVar11 = 1;
      if (DAT_001264a0 == '\0') {
LAB_00105a1f:
        uVar11 = DAT_00126498;
      }
      uVar19 = (local_88 >> 1) + (uVar11 + 3 >> 1) +
               (ulong)((uint)(uVar11 + 3) & (uint)local_88 & 1);
      uVar24 = uVar19 - uVar19 % uVar11;
      if ((uVar24 < 4) || (local_88 <= uVar24)) {
        DAT_00126430 = 0;
        uVar24 = local_88;
      }
      else {
        DAT_00126430 = (uVar19 % uVar11 - uVar19) + local_88;
        if (uVar24 - 3 < DAT_00126430) {
          DAT_00126430 = uVar24 - 3;
        }
        if (DAT_00126430 == 0) {
          uVar24 = local_88;
        }
      }
    }
    local_88 = uVar24;
    DAT_00126428 = local_88;
    if (DAT_00126560 < DAT_00126570) {
      DAT_00126560 = DAT_00126570;
    }
    FUN_00108ba0(&DAT_001265e0);
    FUN_00108ba0(&DAT_001265a0);
    if (DAT_00126580 == 7) {
      if (DAT_00126440 == (char *)0x0) {
        DAT_00126440 = "%l\n";
      }
      if (DAT_00126448 == (undefined *)0x0) {
        DAT_00126448 = &DAT_0011b332;
      }
      if (DAT_00126450 == (undefined *)0x0) {
        DAT_00126450 = &DAT_0011b332;
      }
      if ((DAT_00126468 == (undefined *)0x0) &&
         (DAT_00126468 = &DAT_0011b1cc, DAT_00126478 != (undefined *)0x0)) {
        DAT_00126468 = DAT_00126478;
      }
      if ((DAT_00126470 == (undefined *)0x0) &&
         (DAT_00126470 = &DAT_0011b1cf, DAT_00126478 != (undefined *)0x0)) {
        DAT_00126470 = DAT_00126478;
      }
      if (DAT_00126460 == (char *)0x0) {
        DAT_00126460 = "%=";
        if (DAT_00126478 == (undefined *)0x0) goto LAB_00105e60;
      }
      else {
        if (DAT_00126478 == (undefined *)0x0) {
LAB_00105e60:
          DAT_00126478 = (undefined *)FUN_0010f560(DAT_00126468,DAT_00126470,&DAT_0011d5b0);
          if (DAT_00126580 != 7) goto LAB_00105acd;
        }
        bVar4 = 1;
        if (*DAT_00126460 == '\0') goto LAB_00105ad9;
      }
      iVar8 = strcmp(DAT_00126460,"%=");
      bVar4 = 0;
      if (iVar8 == 0) {
        bVar4 = *DAT_00126440 == '\0';
      }
    }
    else {
LAB_00105acd:
      bVar4 = DAT_00126580 != 8 | DAT_00126438;
    }
LAB_00105ad9:
    DAT_00126553 = ~(DAT_00126554 | DAT_00126552 | DAT_00126490 |
                    (DAT_001265a0 != 0 || DAT_00126558 != 0)) & DAT_001264a1 & 1;
    DAT_00126578 = bVar4;
    DAT_00126420 = FUN_00108d80(param_2 + 1,optind + -1);
    lVar3 = local_50;
    lVar17 = local_58;
    if (local_58 == 0) {
      lVar17 = (long)optind;
      if (local_50 == 0) {
        if (iVar22 - optind != 2) {
          if (1 < iVar22 - optind) {
                    /* WARNING: Subroutine does not return */
            FUN_00108c00("extra operand \'%s\'",param_2[lVar17 + 2]);
          }
                    /* WARNING: Subroutine does not return */
          FUN_00108c00("missing operand after \'%s\'",param_2[uVar23 - 1]);
        }
        iVar8 = FUN_00108e70(0,param_2[lVar17],param_2[lVar17 + 1]);
      }
      else {
        iVar8 = 0;
        while ((int)lVar17 < iVar22) {
          iVar6 = FUN_00108e70(0,param_2[lVar17],lVar3);
          if (iVar8 < iVar6) {
            iVar8 = iVar6;
          }
          optind = optind + 1;
          lVar17 = (long)optind;
        }
      }
    }
    else {
      if (local_50 != 0) {
                    /* WARNING: Subroutine does not return */
        FUN_0010e230("--from-file and --to-file both specified");
      }
      iVar8 = 0;
      for (; optind < iVar22; optind = optind + 1) {
        iVar6 = FUN_00108e70(0,lVar17,param_2[optind]);
        if (iVar8 < iVar6) {
          iVar8 = iVar6;
        }
      }
    }
    FUN_0010e0f0();
    FUN_00108e20();
                    /* WARNING: Subroutine does not return */
    exit(iVar8);
  }
  uVar24 = DAT_00126498;
  switch(iVar6) {
  case 0:
  case 0x68:
  case 0x80:
  case 0x85:
    break;
  default:
                    /* WARNING: Subroutine does not return */
    FUN_00108c00(0,0);
  case 0x30:
  case 0x31:
  case 0x32:
  case 0x33:
  case 0x34:
  case 0x35:
  case 0x36:
  case 0x37:
  case 0x38:
  case 0x39:
    if (iVar8 - 0x30U < 10) {
      if ((long)(local_80 - (ulong)(iVar6 < 0x34)) < 0x666666666666666) {
        local_80 = (long)(iVar6 + -0x30) + local_80 * 10;
        iVar8 = iVar6;
      }
      else {
        local_80 = 0x3fffffffffffffff;
        iVar8 = iVar6;
      }
    }
    else {
      local_80 = (long)(iVar6 + -0x30);
      iVar8 = iVar6;
    }
    goto LAB_001050a0;
  case 0x42:
    DAT_00126554 = 1;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x43:
  case 0x55:
    if (optarg == (char *)0x0) {
      lVar17 = 3;
    }
    else {
      iVar9 = strtoimax(optarg,&local_48,10);
      if ((*local_48 != '\0') || (iVar9 < 0)) {
                    /* WARNING: Subroutine does not return */
        FUN_00108c00("invalid context length \'%s\'",optarg);
      }
      lVar17 = 0x3fffffffffffffff;
      if (iVar9 < 0x4000000000000000) {
        lVar17 = iVar9;
      }
    }
    FUN_00108d50((iVar6 == 0x55) + '\x02');
    uVar24 = DAT_00126498;
    if (DAT_00126570 < lVar17) {
      bVar2 = true;
      DAT_00126570 = lVar17;
      iVar8 = iVar6;
    }
    else {
      bVar2 = true;
      iVar8 = iVar6;
    }
    goto LAB_001050a0;
  case 0x44:
    FUN_00108d50(7);
    sVar10 = strlen(optarg);
    pcVar18 = (char *)FUN_00115f50(sVar10 * 7 + 0x61);
    __sprintf_chk(pcVar18,1,0xffffffffffffffff,
                  "%%=%c#ifndef %s\n%%<#endif /* ! %s */\n%c#ifdef %s\n%%>#endif /* %s */\n%c#ifndef %s\n%%<#else /* %s */\n%%>#endif /* %s */\n"
                  ,0,optarg,optarg,0,optarg,optarg,0,optarg,optarg,optarg);
    puVar20 = &DAT_00126460;
    do {
      puVar21 = puVar20 + 1;
      FUN_00108c60(puVar20,pcVar18,&DAT_0011b206);
      sVar10 = strlen(pcVar18);
      pcVar18 = pcVar18 + sVar10 + 1;
      puVar20 = puVar21;
    } while (puVar21 != (undefined8 *)&DAT_00126480);
    uVar23 = (ulong)iVar22;
    uVar24 = DAT_00126498;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x45:
    iVar8 = iVar6;
    if (DAT_00126558 < 4) {
      DAT_00126558 = DAT_00126558 | 1;
    }
    goto LAB_001050a0;
  case 0x46:
    FUN_00108930(&DAT_001265e0,optarg);
    uVar24 = DAT_00126498;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x48:
    DAT_00126418 = 1;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x49:
    FUN_00108930(&DAT_001265a0,optarg);
    uVar24 = DAT_00126498;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x4c:
    if (DAT_00126540 == (char *)0x0) {
      DAT_00126540 = optarg;
      iVar8 = iVar6;
    }
    else {
      if (DAT_00126548 != (char *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_0010e230("too many file label options");
      }
      DAT_00126548 = optarg;
      iVar8 = iVar6;
    }
    goto LAB_001050a0;
  case 0x4e:
    DAT_00126586 = 1;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x50:
    DAT_00126585 = 1;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x53:
    FUN_00108c60(&DAT_00126488,optarg,&DAT_0011b234);
    uVar24 = DAT_00126498;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x54:
    DAT_00126492 = 1;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x57:
    uVar11 = strtoimax(optarg,&local_48,10);
    if (((long)uVar11 < 1) || (*local_48 != '\0')) {
                    /* WARNING: Subroutine does not return */
      FUN_00108c00("invalid width \'%s\'",optarg);
    }
    if (uVar11 != local_88) {
      bVar25 = local_88 != 0;
      uVar24 = DAT_00126498;
      local_88 = uVar11;
      iVar8 = iVar6;
      if (bVar25) {
                    /* WARNING: Subroutine does not return */
        FUN_0010e230("conflicting width options");
      }
      goto LAB_001050a0;
    }
    break;
  case 0x58:
    uVar7 = FUN_00108910();
    iVar8 = FUN_00110770(FUN_001102e0,DAT_00126410,optarg,uVar7);
    if (iVar8 != 0) {
                    /* WARNING: Subroutine does not return */
      FUN_0010e1f0(optarg);
    }
    break;
  case 0x5a:
    iVar8 = iVar6;
    if (DAT_00126558 < 4) {
      DAT_00126558 = DAT_00126558 | 2;
    }
    goto LAB_001050a0;
  case 0x61:
    DAT_00126568 = 1;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x62:
    iVar8 = iVar6;
    if (DAT_00126558 < 4) {
      DAT_00126558 = 4;
    }
    goto LAB_001050a0;
  case 99:
    uVar12 = 2;
    goto LAB_00105820;
  case 100:
    DAT_00126408 = 1;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x65:
    FUN_00108d50(4);
    uVar24 = DAT_00126498;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x66:
    FUN_00108d50(5);
    uVar24 = DAT_00126498;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x69:
    DAT_00126552 = 1;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x6c:
    if (s__usr_bin_pr_0011d558[0] == '\0') {
                    /* WARNING: Subroutine does not return */
      FUN_00108c00("pagination not supported on this host",0);
    }
    DAT_00126480 = 1;
    signal(0x11,(__sighandler_t)0x0);
    uVar24 = DAT_00126498;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x6e:
    FUN_00108d50(6);
    uVar24 = DAT_00126498;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x70:
    FUN_00108930(&DAT_001265e0,"^[[:alpha:]$_]");
    bVar1 = true;
    uVar24 = DAT_00126498;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x71:
    DAT_001264a1 = 1;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x72:
    DAT_00126608 = 1;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x73:
    DAT_00126584 = 1;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x74:
    DAT_001264a0 = '\x01';
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x75:
    uVar12 = 3;
LAB_00105820:
    FUN_00108d50(uVar12);
    uVar24 = DAT_00126498;
    iVar8 = iVar6;
    if (DAT_00126570 < 3) {
      DAT_00126570 = 3;
    }
    goto LAB_001050a0;
  case 0x76:
    uVar12 = FUN_00114040("Len Tower");
    uVar13 = FUN_00114040("Richard Stallman");
    uVar14 = FUN_00114040("David Hayes");
    uVar15 = FUN_00114040("Mike Haertel");
    uVar16 = FUN_00114040("Paul Eggert");
    FUN_00115de0(stdout,&DAT_0011b285,"GNU diffutils",PTR_DAT_001260a8,uVar16,uVar15,uVar14,uVar13,
                 uVar12,0);
    FUN_00108e20();
    goto LAB_0010544f;
  case 0x77:
    DAT_00126558 = 5;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x78:
    uVar7 = FUN_00108910();
    FUN_001102e0(DAT_00126410,optarg,uVar7);
    uVar24 = DAT_00126498;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x79:
    FUN_00108d50(8);
    uVar24 = DAT_00126498;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x81:
    FUN_00108c60(&local_58,optarg,"--from-file");
    uVar24 = DAT_00126498;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x82:
    FUN_00108a10();
    FUN_00108e20();
LAB_0010544f:
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      return 0;
    }
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  case 0x83:
    iVar9 = strtoimax(optarg,&local_48,10);
    if ((*local_48 != '\0') || (iVar9 < 0)) {
                    /* WARNING: Subroutine does not return */
      FUN_00108c00("invalid horizon length \'%s\'",optarg);
    }
    uVar24 = DAT_00126498;
    iVar8 = iVar6;
    if (DAT_00126560 < iVar9) {
      DAT_00126560 = iVar9;
    }
    goto LAB_001050a0;
  case 0x84:
    DAT_00126551 = 1;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x86:
    DAT_00126439 = 1;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x87:
    FUN_00108d50(7);
    puVar20 = &DAT_00126440;
    do {
      puVar21 = puVar20 + 1;
      FUN_00108c60(puVar20,optarg,"--line-format");
      puVar20 = puVar21;
      uVar24 = DAT_00126498;
      iVar8 = iVar6;
    } while (puVar21 != (undefined8 *)&DAT_00126458);
    goto LAB_001050a0;
  case 0x88:
    DAT_00126550 = 1;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x89:
    DAT_00126551 = 0;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x8a:
    FUN_00108d50(1);
    uVar24 = DAT_00126498;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x8b:
    FUN_00108d50(8);
    DAT_0012643a = 1;
    uVar24 = DAT_00126498;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x8c:
    DAT_00126490 = 1;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x8d:
    DAT_00126491 = 1;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x8e:
    DAT_00126438 = 1;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x8f:
    uVar24 = strtoimax(optarg,&local_48,10);
    if ((((long)uVar24 < 1) || (0xfffffffffffffffc < uVar24)) || (*local_48 != '\0')) {
                    /* WARNING: Subroutine does not return */
      FUN_00108c00("invalid tabsize \'%s\'",optarg);
    }
    if (uVar24 == DAT_00126498) break;
    iVar8 = iVar6;
    if (DAT_00126498 != 0) {
                    /* WARNING: Subroutine does not return */
      FUN_0010e230("conflicting tabsize options");
    }
    goto LAB_001050a0;
  case 0x90:
    FUN_00108c60(&local_50,optarg,"--to-file");
    uVar24 = DAT_00126498;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x91:
  case 0x92:
  case 0x93:
    FUN_00108d50(7);
    iVar8 = iVar6 + -0x91;
    FUN_00108c60(&DAT_00126440 + iVar8,optarg,"--unchanged-line-format" + (long)iVar8 * 0x18);
    uVar24 = DAT_00126498;
    goto LAB_001050a0;
  case 0x94:
  case 0x95:
  case 0x96:
  case 0x97:
    FUN_00108d50(7);
    iVar8 = iVar6 + -0x94;
    FUN_00108c60(&DAT_00126460 + iVar8,optarg,"--unchanged-group-format" + (long)iVar8 * 0x19);
    uVar24 = DAT_00126498;
    goto LAB_001050a0;
  case 0x98:
    FUN_00108cc0(optarg);
    uVar24 = DAT_00126498;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x99:
    FUN_0010e270(optarg);
    uVar24 = DAT_00126498;
    iVar8 = iVar6;
    goto LAB_001050a0;
  case 0x9a:
    DAT_00126180 = 1;
    iVar8 = iVar6;
    goto LAB_001050a0;
  }
  uVar24 = DAT_00126498;
  iVar8 = iVar6;
  goto LAB_001050a0;
}




// Function: shift_boundaries @ 0x6a70

void shift_boundaries(long param_1)

{
  char cVar1;
  long lVar2;
  long lVar3;
  long lVar4;
  long lVar5;
  long lVar6;
  long lVar7;
  long lVar8;
  long lVar9;
  long lVar10;
  long lVar11;
  long lVar12;
  undefined1 *puVar13;
  
  lVar12 = 0;
LAB_00106a7d:
  lVar2 = *(long *)(param_1 + 0x118 + lVar12);
  lVar9 = 0;
  lVar7 = 0;
  lVar3 = *(long *)(param_1 + 0xf8 + lVar12);
  lVar4 = *(long *)(param_1 + 200 + lVar12);
  lVar5 = *(long *)((param_1 + 0x248) - lVar12);
LAB_00106aac:
  do {
    if (lVar4 <= lVar7) goto LAB_00106bea;
    while (lVar10 = lVar7, *(char *)(lVar2 + lVar7) != '\0') {
      while( true ) {
        do {
          lVar7 = lVar7 + 1;
        } while (*(char *)(lVar2 + lVar7) != '\0');
        for (; *(char *)(lVar5 + lVar9) != '\0'; lVar9 = lVar9 + 1) {
        }
        do {
          lVar11 = lVar7 - lVar10;
          while ((lVar10 != 0 &&
                 (*(long *)(lVar3 + -8 + lVar10 * 8) == *(long *)(lVar3 + -8 + lVar7 * 8)))) {
            lVar7 = lVar7 + -1;
            *(undefined1 *)(lVar2 + lVar10 + -1) = 1;
            *(undefined1 *)(lVar2 + lVar7) = 0;
            cVar1 = *(char *)(lVar2 + -2 + lVar10);
            lVar10 = lVar10 + -1;
            while (cVar1 != '\0') {
              lVar10 = lVar10 + -1;
              cVar1 = *(char *)(lVar2 + -1 + lVar10);
            }
            do {
              lVar9 = lVar9 + -1;
            } while (*(char *)(lVar5 + lVar9) != '\0');
          }
          lVar6 = lVar7;
          if (*(char *)(lVar5 + -1 + lVar9) == '\0') {
            lVar6 = lVar4;
          }
          while ((lVar4 != lVar7 && (*(long *)(lVar3 + lVar10 * 8) == *(long *)(lVar3 + lVar7 * 8)))
                ) {
            lVar10 = lVar10 + 1;
            lVar8 = lVar7 + 1;
            *(undefined1 *)(lVar2 + -1 + lVar10) = 0;
            *(undefined1 *)(lVar2 + lVar7) = 1;
            lVar7 = lVar6;
            if (*(char *)(lVar2 + lVar8) == '\0') goto LAB_00106b93;
            do {
              lVar8 = lVar8 + 1;
            } while (*(char *)(lVar2 + lVar8) != '\0');
            lVar9 = lVar9 + 1;
            cVar1 = *(char *)(lVar5 + lVar9);
            while (lVar7 = lVar8, cVar1 != '\0') {
LAB_00106b93:
              lVar9 = lVar9 + 1;
              lVar6 = lVar7;
              cVar1 = *(char *)(lVar5 + lVar9);
            }
          }
        } while (lVar7 - lVar10 != lVar11);
        if (lVar7 <= lVar6) goto LAB_00106aac;
        puVar13 = (undefined1 *)(lVar2 + -1 + lVar7);
        do {
          puVar13[lVar10 - lVar7] = 1;
          *puVar13 = 0;
          do {
            lVar9 = lVar9 + -1;
          } while (*(char *)(lVar5 + lVar9) != '\0');
          puVar13 = puVar13 + -1;
        } while ((undefined1 *)(lVar2 + -1 + lVar6) != puVar13);
        lVar7 = lVar6;
        if (lVar6 < lVar4) break;
LAB_00106bea:
        lVar10 = lVar7;
        if (lVar7 == lVar4) {
          lVar12 = lVar12 + 0x130;
          if (lVar12 == 0x260) {
            return;
          }
          goto LAB_00106a7d;
        }
      }
    }
    do {
      lVar9 = lVar9 + 1;
    } while (*(char *)(lVar5 + -1 + lVar9) != '\0');
    lVar7 = lVar7 + 1;
  } while( true );
}




// Function: add_change @ 0x6c30

void add_change(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_00115f50(0x30);
  puVar1[3] = param_1;
  puVar1[4] = param_2;
  puVar1[1] = param_4;
  puVar1[2] = param_3;
  *puVar1 = param_5;
  return;
}




// Function: build_reverse_script @ 0x6c70

undefined8 build_reverse_script(long param_1)

{
  char cVar1;
  char cVar2;
  long lVar3;
  long lVar4;
  long lVar5;
  long lVar6;
  undefined8 uVar7;
  long lVar8;
  long lVar9;
  long lVar10;
  long lVar11;
  long lVar12;
  long lVar13;
  
  lVar3 = *(long *)(param_1 + 200);
  lVar4 = *(long *)(param_1 + 0x118);
  lVar5 = *(long *)(param_1 + 0x248);
  lVar6 = *(long *)(param_1 + 0x1f8);
  if ((lVar3 < 1) && (lVar6 < 1)) {
    uVar7 = 0;
  }
  else {
    lVar11 = 0;
    lVar12 = 0;
    uVar7 = 0;
    do {
      cVar1 = *(char *)(lVar4 + lVar12);
      cVar2 = *(char *)(lVar5 + lVar11);
      lVar10 = lVar11;
      lVar9 = lVar12;
      if (cVar1 != '\0' || cVar2 != '\0') {
        if (cVar1 == '\0') {
          lVar13 = 0;
        }
        else {
          do {
            lVar9 = lVar9 + 1;
          } while (*(char *)(lVar4 + lVar9) != '\0');
          lVar13 = lVar9 - lVar12;
        }
        if (cVar2 == '\0') {
          lVar8 = 0;
        }
        else {
          do {
            lVar10 = lVar10 + 1;
          } while (*(char *)(lVar5 + lVar10) != '\0');
          lVar8 = lVar10 - lVar11;
        }
        uVar7 = FUN_00106c30(lVar12,lVar11,lVar13,lVar8);
      }
      lVar12 = lVar9 + 1;
      lVar11 = lVar10 + 1;
    } while ((lVar12 < lVar3) || (lVar11 < lVar6));
  }
  return uVar7;
}




// Function: build_script @ 0x6d50

undefined8 build_script(long param_1)

{
  char cVar1;
  char cVar2;
  long lVar3;
  long lVar4;
  undefined8 uVar5;
  long lVar6;
  ulong uVar7;
  ulong uVar8;
  ulong uVar9;
  long lVar10;
  ulong uVar11;
  
  uVar5 = 0;
  uVar9 = *(ulong *)(param_1 + 200);
  uVar11 = *(ulong *)(param_1 + 0x1f8);
  lVar3 = *(long *)(param_1 + 0x118);
  lVar4 = *(long *)(param_1 + 0x248);
  uVar8 = uVar9 & uVar11;
  do {
    if ((long)uVar8 < 0) {
      return uVar5;
    }
    do {
      cVar1 = *(char *)(lVar3 + -1 + uVar9);
      cVar2 = *(char *)(lVar4 + -1 + uVar11);
      if (cVar1 != '\0' || cVar2 != '\0') {
        uVar7 = uVar9;
        uVar8 = uVar11;
        if (cVar1 == '\0') {
          lVar10 = 0;
          if (cVar2 == '\0') goto LAB_00106e20;
LAB_00106de0:
          do {
            uVar8 = uVar8 - 1;
          } while (*(char *)(lVar4 + -1 + uVar8) != '\0');
          lVar6 = uVar11 - uVar8;
          uVar11 = uVar8;
        }
        else {
          do {
            uVar7 = uVar7 - 1;
          } while (*(char *)(lVar3 + -1 + uVar7) != '\0');
          lVar10 = uVar9 - uVar7;
          uVar9 = uVar7;
          if (cVar2 != '\0') goto LAB_00106de0;
LAB_00106e20:
          lVar6 = 0;
        }
        uVar5 = FUN_00106c30(uVar9,uVar11,lVar10,lVar6);
      }
      uVar11 = uVar11 - 1;
      uVar9 = uVar9 - 1;
      uVar8 = uVar11;
    } while (-1 < (long)uVar9);
  } while( true );
}




// Function: discard_confusing_lines @ 0x6e30

void discard_confusing_lines(long param_1)

{
  char cVar1;
  ulong uVar2;
  long lVar3;
  void *__ptr;
  void *__ptr_00;
  ulong uVar4;
  long lVar5;
  char *pcVar6;
  ulong uVar7;
  long *plVar8;
  long lVar9;
  long lVar10;
  long lVar11;
  ulong uVar12;
  void *pvVar13;
  undefined8 *puVar14;
  long lVar15;
  void *pvVar16;
  long lVar17;
  long *plVar18;
  void **ppvVar19;
  char *pcVar20;
  long lVar21;
  long lVar22;
  undefined8 *puVar23;
  long in_FS_OFFSET;
  undefined8 local_68;
  long local_60;
  void *local_58 [3];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  lVar3 = FUN_00115f50((*(long *)(param_1 + 0x1f8) + *(long *)(param_1 + 200)) * 0x10);
  *(long *)(param_1 + 0x100) = lVar3;
  lVar22 = *(long *)(param_1 + 200) * 8;
  lVar3 = lVar3 + lVar22;
  *(long *)(param_1 + 0x108) = lVar3;
  lVar3 = lVar3 + lVar22;
  *(long *)(param_1 + 0x230) = lVar3;
  *(long *)(param_1 + 0x238) = lVar3 + *(long *)(param_1 + 0x1f8) * 8;
  __ptr = (void *)FUN_0010f5e0(*(long *)(param_1 + 0x128) << 4);
  pvVar13 = (void *)((long)__ptr + *(long *)(param_1 + 0x128) * 8);
  if (0 < *(long *)(param_1 + 200)) {
    lVar22 = *(long *)(param_1 + 0xf8);
    lVar3 = 0;
    do {
      lVar5 = lVar3 * 8;
      lVar3 = lVar3 + 1;
      plVar8 = (long *)((long)__ptr + *(long *)(lVar22 + lVar5) * 8);
      *plVar8 = *plVar8 + 1;
    } while (lVar3 < *(long *)(param_1 + 200));
  }
  if (0 < *(long *)(param_1 + 0x1f8)) {
    lVar22 = *(long *)(param_1 + 0x228);
    lVar3 = 0;
    do {
      lVar5 = lVar3 * 8;
      lVar3 = lVar3 + 1;
      plVar8 = (long *)((long)pvVar13 + *(long *)(lVar22 + lVar5) * 8);
      *plVar8 = *plVar8 + 1;
    } while (lVar3 < *(long *)(param_1 + 0x1f8));
  }
  puVar23 = &local_68;
  local_58[0] = __ptr;
  local_58[1] = pvVar13;
  __ptr_00 = (void *)FUN_0010f5e0();
  uVar7 = *(ulong *)(param_1 + 200);
  plVar8 = (long *)(param_1 + 0xf8);
  ppvVar19 = local_58;
  lVar22 = *plVar8;
  local_60 = (long)__ptr_00 + uVar7;
  uVar4 = uVar7 >> 8;
  pvVar16 = __ptr_00;
  puVar14 = puVar23;
  if (uVar4 == 0) goto LAB_0010737e;
LAB_00106f76:
  uVar12 = 5;
  do {
    uVar12 = uVar12 * 2;
    uVar4 = uVar4 >> 2;
    plVar18 = plVar8;
  } while (uVar4 != 0);
  do {
    uVar4 = 0;
    do {
      while (lVar3 = *(long *)(lVar22 + uVar4 * 8), lVar3 == 0) {
LAB_00106f94:
        uVar4 = uVar4 + 1;
        if (uVar4 == uVar7) goto LAB_00106fc8;
      }
      uVar2 = *(ulong *)((long)pvVar13 + lVar3 * 8);
      if (uVar2 == 0) {
        *(undefined1 *)((long)pvVar16 + uVar4) = 1;
        goto LAB_00106f94;
      }
      if (uVar2 <= uVar12) goto LAB_00106f94;
      *(undefined1 *)((long)pvVar16 + uVar4) = 2;
      uVar4 = uVar4 + 1;
    } while (uVar4 != uVar7);
LAB_00106fc8:
    do {
      plVar8 = plVar18 + 0x26;
      puVar14 = puVar14 + 1;
      if ((long *)(param_1 + 0x358) == plVar8) {
        lVar22 = 0;
        pvVar13 = __ptr_00;
        puVar14 = puVar23;
        goto LAB_00106ff4;
      }
      uVar7 = plVar18[0x20];
      pvVar13 = *ppvVar19;
      ppvVar19 = ppvVar19 + -1;
      pvVar16 = (void *)*puVar14;
      lVar22 = *plVar8;
      uVar4 = uVar7 >> 8;
      if (uVar4 != 0) goto LAB_00106f76;
LAB_0010737e:
      plVar18 = plVar8;
    } while (uVar7 == 0);
    uVar12 = 5;
  } while( true );
LAB_00106ff4:
  lVar3 = *(long *)(param_1 + 200 + lVar22);
  lVar5 = 0;
  if (0 < lVar3) {
    do {
      while( true ) {
        pcVar6 = (char *)((long)pvVar13 + lVar5);
        if (*pcVar6 != '\x02') break;
        lVar5 = lVar5 + 1;
        *pcVar6 = '\0';
        if (lVar3 <= lVar5) goto LAB_0010703b;
      }
      lVar10 = lVar5;
      if (*pcVar6 != '\0') {
        lVar15 = 0;
        lVar9 = lVar5;
        while ((lVar9 = lVar9 + 1, lVar9 < lVar3 && (*(char *)((long)pvVar13 + lVar9) != '\0'))) {
          lVar15 = lVar15 + (ulong)(*(char *)((long)pvVar13 + lVar9) == '\x02');
        }
        if (lVar5 < lVar9) {
          do {
            if (*(char *)((long)pvVar13 + lVar9 + -1) != '\x02') {
              lVar17 = lVar9 - lVar5;
              if (lVar15 * 4 <= lVar17) goto LAB_001071cb;
              if (lVar5 < lVar9) {
                pcVar6 = (char *)((long)pvVar13 + lVar9 + -1);
                do {
                  if (*pcVar6 == '\x02') {
                    *pcVar6 = '\0';
                  }
                  pcVar6 = pcVar6 + -1;
                } while ((char *)((long)pvVar13 + lVar5 + -1) != pcVar6);
              }
              goto LAB_00107018;
            }
            lVar9 = lVar9 + -1;
            lVar15 = lVar15 + -1;
            *(undefined1 *)((long)pvVar13 + lVar9) = 0;
          } while (lVar9 != lVar5);
          lVar17 = 0;
          if (lVar15 < 1) {
LAB_0010719b:
            lVar10 = lVar17 + -1 + lVar5;
            if (0 < lVar17) {
              lVar9 = 1;
              goto LAB_001071ee;
            }
          }
        }
        else {
          lVar17 = lVar9 - lVar5;
          if (lVar15 * 4 <= lVar17) {
LAB_001071cb:
            lVar10 = lVar17 >> 4;
            if (lVar10 < 1) goto LAB_0010719b;
            lVar9 = 1;
            do {
              lVar9 = lVar9 * 2;
              lVar10 = lVar10 >> 2;
            } while (lVar10 != 0);
            lVar10 = lVar17 + -1 + lVar5;
LAB_001071ee:
            lVar11 = 0;
            lVar15 = 0;
            do {
              while (pcVar20 = (char *)(lVar11 + lVar5 + (long)pvVar13), *pcVar20 != '\x02') {
                lVar21 = 0;
LAB_001071fb:
                lVar11 = lVar11 + 1;
                lVar15 = lVar21;
                if (lVar17 <= lVar11) goto LAB_00107238;
              }
              lVar21 = lVar15 + 1;
              if (lVar15 == lVar9) {
                lVar11 = lVar11 - lVar21;
                goto LAB_001071fb;
              }
              if (lVar15 <= lVar9) goto LAB_001071fb;
              lVar11 = lVar11 + 1;
              *pcVar20 = '\0';
              lVar15 = lVar21;
            } while (lVar11 < lVar17);
LAB_00107238:
            lVar5 = 0;
            pcVar20 = pcVar6;
            do {
              while( true ) {
                cVar1 = *pcVar20;
                if ((7 < (long)pcVar20 - (long)pcVar6) && (cVar1 == '\x01')) goto LAB_00107290;
                if (cVar1 != '\x02') break;
                *pcVar20 = '\0';
                pcVar20 = pcVar20 + 1;
                lVar5 = 0;
                if (pcVar6 + lVar17 == pcVar20) goto LAB_00107290;
              }
              if (cVar1 == '\0') {
                lVar5 = 0;
              }
              else {
                lVar5 = lVar5 + 1;
                if (lVar5 == 3) break;
              }
              pcVar20 = pcVar20 + 1;
            } while (pcVar6 + lVar17 != pcVar20);
LAB_00107290:
            lVar5 = 0;
            pcVar6 = (char *)((long)pvVar13 + lVar10);
            do {
              if ((7 < ((long)pvVar13 + lVar10) - (long)pcVar6) && (*pcVar6 == '\x01')) break;
              if (*pcVar6 == '\x02') {
                *pcVar6 = '\0';
                lVar5 = 0;
              }
              else if (*pcVar6 == '\0') {
                lVar5 = 0;
              }
              else {
                lVar5 = lVar5 + 1;
                if (lVar5 == 3) break;
              }
              pcVar6 = pcVar6 + -1;
            } while ((char *)((long)pvVar13 + (lVar10 - lVar17)) != pcVar6);
          }
        }
      }
LAB_00107018:
      lVar5 = lVar10 + 1;
    } while (lVar5 < lVar3);
  }
LAB_0010703b:
  lVar22 = lVar22 + 0x130;
  puVar14 = puVar14 + 1;
  if (lVar22 == 0x260) {
    plVar8 = (long *)(param_1 + 200);
    pvVar13 = __ptr_00;
    do {
      lVar22 = *plVar8;
      lVar3 = 0;
      if (0 < lVar22) {
        lVar5 = 0;
        do {
          while ((DAT_00126408 == '\0' && (*(char *)((long)pvVar13 + lVar5) != '\0'))) {
            *(undefined1 *)(plVar8[10] + lVar5) = 1;
            lVar5 = lVar5 + 1;
            if (lVar22 == lVar5) goto LAB_001070d6;
          }
          lVar10 = lVar3 * 8;
          *(undefined8 *)(plVar8[7] + lVar3 * 8) = *(undefined8 *)(plVar8[6] + lVar5 * 8);
          lVar3 = lVar3 + 1;
          *(long *)(plVar8[8] + lVar10) = lVar5;
          lVar5 = lVar5 + 1;
        } while (lVar22 != lVar5);
      }
LAB_001070d6:
      plVar8[9] = lVar3;
      plVar8 = plVar8 + 0x26;
      puVar23 = puVar23 + 1;
      if ((long *)(param_1 + 0x328) == plVar8) {
        free(__ptr_00);
        if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
          __stack_chk_fail();
        }
        free(__ptr);
        return;
      }
      pvVar13 = (void *)*puVar23;
    } while( true );
  }
  pvVar13 = (void *)*puVar14;
  goto LAB_00106ff4;
}




// Function: briefly_report @ 0x73b0

void briefly_report(int param_1,long param_2)

{
  long lVar1;
  long lVar2;
  undefined8 uVar3;
  
  if (param_1 == 0) {
    return;
  }
  lVar2 = DAT_00126548;
  if (DAT_00126548 == 0) {
    lVar2 = *(long *)(param_2 + 0x138);
  }
  lVar1 = DAT_00126540;
  if (DAT_00126540 == 0) {
    lVar1 = *(long *)(param_2 + 8);
  }
  if (DAT_001264a1 == '\0') {
    uVar3 = dcgettext(0,"Binary files %s and %s differ\n",5);
  }
  else {
    uVar3 = dcgettext(0,"Files %s and %s differ\n",5);
  }
  FUN_0010e0e0(uVar3,lVar1,lVar2);
  return;
}




// Function: diff_2_files @ 0x7460

bool diff_2_files(int *param_1)

{
  size_t __n;
  undefined8 *puVar1;
  undefined1 uVar2;
  char cVar3;
  int iVar4;
  undefined8 uVar5;
  size_t sVar6;
  void *pvVar7;
  undefined8 *puVar8;
  long *plVar9;
  long lVar10;
  long lVar11;
  undefined8 *puVar12;
  int *piVar13;
  long lVar14;
  long in_FS_OFFSET;
  bool bVar15;
  byte bVar16;
  undefined1 local_98 [8];
  undefined1 local_90 [8];
  undefined1 local_88 [8];
  undefined1 local_80 [8];
  undefined8 local_78;
  undefined8 local_70;
  long local_68;
  long local_60;
  undefined1 local_58;
  long local_50;
  long local_40;
  
  bVar16 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  cVar3 = FUN_0010c8f0(param_1,DAT_00126553);
  if (cVar3 != '\0') {
    if ((((*(long *)(param_1 + 0x10) < 1 || *(long *)(param_1 + 0x10) == *(long *)(param_1 + 0x5c))
         || (*(long *)(param_1 + 0x5c) < 1)) ||
        ((-1 < *param_1 && ((param_1[10] & 0xf000U) != 0x8000)))) ||
       ((-1 < param_1[0x4c] && ((param_1[0x56] & 0xf000U) != 0x8000)))) {
      if (*param_1 != param_1[0x4c]) {
        uVar5 = FUN_001165d0(*(undefined8 *)(param_1 + 0x12),*(undefined8 *)(param_1 + 0x5e),
                             0x7ffffffffffffffe);
        sVar6 = FUN_001165d0(8,uVar5,0x7ffffffffffffffe);
        uVar5 = FUN_00115fa0(*(undefined8 *)(param_1 + 0x28),sVar6);
        *(undefined8 *)(param_1 + 0x28) = uVar5;
        uVar5 = FUN_00115fa0(*(undefined8 *)(param_1 + 0x74),sVar6);
        *(undefined8 *)(param_1 + 0x74) = uVar5;
        while( true ) {
          if (-1 < *param_1) {
            FUN_0010bf50(param_1,sVar6 - *(long *)(param_1 + 0x2c));
          }
          if (-1 < param_1[0x4c]) {
            FUN_0010bf50(param_1 + 0x4c,sVar6 - *(long *)(param_1 + 0x78));
          }
          __n = *(size_t *)(param_1 + 0x2c);
          if ((__n != *(size_t *)(param_1 + 0x78)) ||
             (iVar4 = memcmp(*(void **)(param_1 + 0x28),*(void **)(param_1 + 0x74),__n), iVar4 != 0)
             ) goto LAB_001075bb;
          if (__n != sVar6) break;
          param_1[0x78] = 0;
          param_1[0x79] = 0;
          param_1[0x2c] = 0;
          param_1[0x2d] = 0;
        }
      }
      bVar15 = false;
    }
    else {
LAB_001075bb:
      bVar15 = true;
    }
    FUN_001073b0(bVar15,param_1);
    goto LAB_001075d6;
  }
  pvVar7 = (void *)FUN_0010f5e0(*(long *)(param_1 + 0x7e) + *(long *)(param_1 + 0x32) + 4);
  *(long *)(param_1 + 0x46) = (long)pvVar7 + 1;
  *(long *)(param_1 + 0x92) = (long)pvVar7 + *(long *)(param_1 + 0x32) + 3;
  FUN_00106e30(param_1);
  local_78 = *(undefined8 *)(param_1 + 0x40);
  lVar11 = *(long *)(param_1 + 0x90) + *(long *)(param_1 + 0x44) + 3;
  local_70 = *(undefined8 *)(param_1 + 0x8c);
  local_60 = FUN_00115f50(lVar11 * 0x10);
  uVar2 = DAT_00126408;
  lVar14 = *(long *)(param_1 + 0x90);
  lVar10 = lVar14 * 8 + 8;
  local_68 = local_60 + lVar10;
  local_60 = local_60 + lVar10 + lVar11 * 8;
  local_58 = DAT_00126418;
  if (lVar11 == 0) {
    local_50 = 0x1000;
  }
  else {
    local_50 = 1;
    do {
      local_50 = local_50 * 2;
      lVar11 = lVar11 >> 2;
    } while (lVar11 != 0);
    if (local_50 < 0x1000) {
      local_50 = 0x1000;
    }
  }
  piVar13 = param_1;
  puVar8 = &DAT_001261a0;
  for (lVar10 = 0x26; lVar10 != 0; lVar10 = lVar10 + -1) {
    *puVar8 = *(undefined8 *)piVar13;
    piVar13 = piVar13 + (ulong)bVar16 * -4 + 2;
    puVar8 = puVar8 + (ulong)bVar16 * -2 + 1;
  }
  piVar13 = param_1 + 0x4c;
  for (lVar10 = 0x26; lVar10 != 0; lVar10 = lVar10 + -1) {
    *puVar8 = *(undefined8 *)piVar13;
    piVar13 = piVar13 + (ulong)bVar16 * -4 + 2;
    puVar8 = puVar8 + (ulong)bVar16 * -2 + 1;
  }
  FUN_00106890(0,*(undefined8 *)(param_1 + 0x44),0,lVar14,uVar2,&local_78);
  free((void *)(*(long *)(param_1 + 0x90) * -8 + -8 + local_68));
  FUN_00106a70(param_1);
  if (DAT_00126580 == 4) {
    puVar8 = (undefined8 *)FUN_00106c70(param_1);
  }
  else {
    puVar8 = (undefined8 *)FUN_00106d50();
  }
  if ((DAT_00126554 == '\0') && (DAT_001264e0 == 0)) {
    bVar15 = puVar8 != (undefined8 *)0x0;
LAB_0010780b:
    if (DAT_001264a1 != '\0') goto LAB_001079cb;
    if (bVar15 == false) {
LAB_001078dc:
      bVar15 = false;
      if (DAT_00126578 == '\0') goto LAB_00107821;
    }
    else {
LAB_00107821:
      lVar10 = DAT_00126548;
      if (DAT_00126548 == 0) {
        lVar10 = *(long *)(param_1 + 0x4e);
      }
      lVar14 = DAT_00126540;
      if (DAT_00126540 == 0) {
        lVar14 = *(long *)(param_1 + 2);
      }
      FUN_0010e280(lVar14,lVar10,*(long *)(param_1 + 0x98) != 0);
      switch(DAT_00126580) {
      default:
                    /* WARNING: Subroutine does not return */
        abort();
      case 1:
        FUN_0010ccf0(puVar8);
        break;
      case 2:
        FUN_00108870(puVar8,0);
        break;
      case 3:
        FUN_00108870(puVar8,1);
        break;
      case 4:
        FUN_0010aa10(puVar8);
        break;
      case 5:
        FUN_0010aa30(puVar8);
        break;
      case 6:
        FUN_0010aa50(puVar8);
        break;
      case 7:
        FUN_0010b430(puVar8);
        break;
      case 8:
        FUN_0010d5b0(puVar8);
      }
      FUN_0010e550();
    }
  }
  else {
    if (puVar8 != (undefined8 *)0x0) {
      puVar12 = puVar8;
      do {
        plVar9 = (long *)FUN_0010eaf0(puVar12);
        puVar1 = (undefined8 *)*plVar9;
        *plVar9 = 0;
        iVar4 = FUN_0010f1a0(puVar12,local_98,local_90,local_88,local_80);
        *plVar9 = (long)puVar1;
        if (puVar1 == (undefined8 *)0x0) break;
        puVar12 = puVar1;
      } while (iVar4 == 0);
      bVar15 = iVar4 != 0;
      goto LAB_0010780b;
    }
    if (DAT_001264a1 == '\0') goto LAB_001078dc;
    bVar15 = false;
LAB_001079cb:
    FUN_001073b0(bVar15,param_1);
  }
  free(*(void **)(param_1 + 0x40));
  free(pvVar7);
  free(*(void **)(param_1 + 0x3e));
  free((void *)(*(long *)(param_1 + 0x2e) + *(long *)(param_1 + 0x30) * 8));
  free(*(void **)(param_1 + 0x8a));
  free((void *)(*(long *)(param_1 + 0x7a) + *(long *)(param_1 + 0x7c) * 8));
  while (puVar8 != (undefined8 *)0x0) {
    puVar12 = (undefined8 *)*puVar8;
    free(puVar8);
    puVar8 = puVar12;
  }
  if (DAT_00126580 - 4U < 2) {
    plVar9 = &DAT_00126540;
    piVar13 = param_1 + 0x48;
    do {
      if ((char)*piVar13 != '\0') {
        uVar5 = dcgettext(0,"No newline at end of file",5);
        lVar10 = *plVar9;
        if (lVar10 == 0) {
          lVar10 = *(long *)(piVar13 + -0x46);
        }
                    /* WARNING: Subroutine does not return */
        error(0,0,"%s: %s\n",lVar10,uVar5);
      }
      plVar9 = plVar9 + 1;
      piVar13 = piVar13 + 0x4c;
    } while (plVar9 != (long *)&DAT_00126550);
  }
LAB_001075d6:
  pvVar7 = *(void **)(param_1 + 0x74);
  if (*(void **)(param_1 + 0x28) != pvVar7) {
    free(*(void **)(param_1 + 0x28));
    pvVar7 = *(void **)(param_1 + 0x74);
  }
  free(pvVar7);
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return bVar15;
}




// Function: print_context_label @ 0x7ab0

void print_context_label(undefined8 param_1,long param_2,undefined8 param_3,long param_4)

{
  undefined4 uVar1;
  tm *ptVar2;
  long lVar3;
  long in_FS_OFFSET;
  undefined1 auStack_78 [56];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010eb70(0);
  if (param_4 != 0) {
    __fprintf_chk(DAT_00126188,1,"%s %s",param_1,param_4);
    goto LAB_00107b06;
  }
  ptVar2 = localtime((time_t *)(param_2 + 0x68));
  uVar1 = FUN_001154d0(param_2 + 0x10);
  if (ptVar2 == (tm *)0x0) {
LAB_00107b8a:
    __sprintf_chk(auStack_78,1,0x2b,"%ld.%.9d",*(undefined8 *)(param_2 + 0x68),uVar1);
  }
  else {
    lVar3 = FUN_00113d10(auStack_78,0x2b,DAT_00126400,ptVar2,0,uVar1);
    if (lVar3 == 0) goto LAB_00107b8a;
  }
  __fprintf_chk(DAT_00126188,1,"%s %s\t%s",param_1,param_3,auStack_78);
LAB_00107b06:
  FUN_0010eb70(3);
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    putc_unlocked(10,DAT_00126188);
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: mark_ignorable @ 0x7be0

void mark_ignorable(undefined8 *param_1)

{
  undefined8 *puVar1;
  int iVar2;
  long in_FS_OFFSET;
  undefined1 local_60 [8];
  undefined1 local_58 [8];
  undefined1 local_50 [8];
  undefined1 local_48 [8];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 != (undefined8 *)0x0) {
    do {
      puVar1 = (undefined8 *)*param_1;
      *param_1 = 0;
      iVar2 = FUN_0010f1a0(param_1,local_60,local_58,local_50,local_48);
      *param_1 = puVar1;
      *(bool *)(param_1 + 5) = iVar2 == 0;
      param_1 = puVar1;
    } while (puVar1 != (undefined8 *)0x0);
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: find_function @ 0x7c80

char * find_function(long param_1,long param_2)

{
  long lVar1;
  int iVar2;
  ulong uVar3;
  char *pcVar4;
  
  lVar1 = DAT_00126178;
  DAT_00126178 = param_2;
  do {
    param_2 = param_2 + -1;
    if (param_2 < lVar1) {
      pcVar4 = (char *)0x0;
      if (DAT_00126170 != 0x7fffffffffffffff) {
        pcVar4 = *(char **)(param_1 + DAT_00126170 * 8);
      }
      return pcVar4;
    }
    pcVar4 = *(char **)(param_1 + param_2 * 8);
    uVar3 = ~(ulong)pcVar4 + *(long *)(param_1 + 8 + param_2 * 8);
    if (0x7fffffff < uVar3) {
      uVar3 = 0x7fffffff;
    }
    iVar2 = re_search((re_pattern_buffer *)&DAT_00126500,pcVar4,(int)uVar3,0,(int)uVar3,
                      (re_registers *)0x0);
  } while (iVar2 < 0);
  DAT_00126170 = param_2;
  return pcVar4;
}




// Function: print_context_function @ 0x7d30

void print_context_function(FILE *param_1,char *param_2)

{
  char cVar1;
  char cVar2;
  char *pcVar3;
  int iVar4;
  char *__ptr;
  int iVar5;
  
  iVar5 = 0;
  putc_unlocked(0x20,param_1);
  __ptr = param_2;
  while( true ) {
    cVar2 = *__ptr;
    cVar1 = FUN_0010f850();
    if ((cVar2 == '\n') || (cVar1 == '\0')) break;
    iVar5 = iVar5 + 1;
    __ptr = __ptr + 1;
  }
  pcVar3 = param_2 + (long)iVar5 + 1;
  iVar4 = iVar5;
  while ((cVar2 != '\n' && (iVar4 = iVar4 + 1, iVar4 <= iVar5 + 0x27))) {
    cVar2 = *pcVar3;
    pcVar3 = pcVar3 + 1;
  }
  if (iVar5 < iVar4) {
    param_2 = param_2 + (long)iVar4 + -1;
    do {
      cVar2 = FUN_0010f850(*param_2);
      if (cVar2 == '\0') break;
      iVar4 = iVar4 + -1;
      param_2 = param_2 + -1;
    } while (iVar5 != iVar4);
  }
  fwrite_unlocked(__ptr,1,(long)(iVar4 - iVar5),param_1);
  return;
}




// Function: print_context_number_range @ 0x7e00

void print_context_number_range(void)

{
  long lVar1;
  long in_FS_OFFSET;
  long local_20;
  long local_18;
  
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010f0c0();
  if (local_20 < local_18) {
    __fprintf_chk(DAT_00126188,1,&DAT_0011b08e);
  }
  else {
    __fprintf_chk(DAT_00126188,1,&DAT_0011b092,local_18);
  }
  if (lVar1 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: pr_context_hunk @ 0x7e90

void pr_context_hunk(undefined8 *param_1)

{
  bool bVar1;
  uint uVar2;
  long lVar3;
  FILE *__stream;
  undefined *puVar4;
  undefined8 *puVar5;
  long in_FS_OFFSET;
  long local_60;
  long local_58;
  long local_50;
  long local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar2 = FUN_0010f1a0(param_1,&local_60,&local_58,&local_50,&local_48);
  if (uVar2 != 0) {
    lVar3 = -DAT_00126288;
    local_60 = local_60 - DAT_00126570;
    if (local_60 < lVar3) {
      local_60 = lVar3;
    }
    local_50 = local_50 - DAT_00126570;
    if (local_50 < lVar3) {
      local_50 = lVar3;
    }
    lVar3 = DAT_00126570 + local_58;
    bVar1 = local_58 < DAT_00126270 - DAT_00126570;
    local_58 = DAT_00126270 + -1;
    if (bVar1) {
      local_58 = lVar3;
    }
    lVar3 = DAT_00126570 + local_48;
    bVar1 = local_48 < DAT_001263a0 - DAT_00126570;
    local_48 = DAT_001263a0 + -1;
    if (bVar1) {
      local_48 = lVar3;
    }
    if (DAT_00126520 == 0) {
      FUN_0010e2b0();
      __stream = DAT_00126188;
      fputs_unlocked("***************",DAT_00126188);
    }
    else {
      lVar3 = FUN_00107c80(DAT_00126258);
      FUN_0010e2b0();
      __stream = DAT_00126188;
      fputs_unlocked("***************",DAT_00126188);
      if (lVar3 != 0) {
        FUN_00107d30(__stream,lVar3);
      }
    }
    putc_unlocked(10,__stream);
    FUN_0010eb70(4);
    fputs_unlocked("*** ",__stream);
    FUN_00107e00(&DAT_001261a0,local_60,local_58);
    fputs_unlocked(" ****",__stream);
    FUN_0010eb70(3);
    putc_unlocked(10,__stream);
    if (((uVar2 & 1) != 0) && (puVar5 = param_1, lVar3 = local_60, local_60 <= local_58)) {
      do {
        FUN_0010eb70(2);
        for (; puVar5 != (undefined8 *)0x0; puVar5 = (undefined8 *)*puVar5) {
          if (lVar3 < (long)(puVar5[2] + puVar5[3])) {
            if ((long)puVar5[3] <= lVar3) {
              puVar4 = &DAT_0011b0c7;
              if (0 < (long)puVar5[1]) {
                puVar4 = &DAT_0011b096;
              }
              goto LAB_00108084;
            }
            break;
          }
        }
        puVar4 = &DAT_0011b0ab;
LAB_00108084:
        FUN_0010ef20(puVar4,DAT_00126258 + lVar3 * 8,1);
        FUN_0010eb70(3);
        if (*(char *)(*(long *)(DAT_00126258 + 8 + lVar3 * 8) + -1) == '\n') {
          putc_unlocked(10,__stream);
        }
        lVar3 = lVar3 + 1;
      } while (lVar3 <= local_58);
    }
    FUN_0010eb70(4);
    fputs_unlocked("--- ",__stream);
    FUN_00107e00(&DAT_001262d0,local_50,local_48);
    fputs_unlocked(" ----",__stream);
    FUN_0010eb70(3);
    putc_unlocked(10,__stream);
    if (((uVar2 & 2) != 0) && (local_50 <= local_48)) {
      lVar3 = local_50;
      do {
        FUN_0010eb70(1);
        for (; puVar4 = &DAT_0011b0ab, param_1 != (undefined8 *)0x0;
            param_1 = (undefined8 *)*param_1) {
          if (lVar3 < (long)(param_1[1] + param_1[4])) {
            if (((long)param_1[4] <= lVar3) && (puVar4 = &DAT_0011b0ca, 0 < (long)param_1[2])) {
              puVar4 = &DAT_0011b096;
            }
            break;
          }
        }
        FUN_0010ef20(puVar4,DAT_00126388 + lVar3 * 8,1);
        FUN_0010eb70(3);
        if (*(char *)(*(long *)(DAT_00126388 + 8 + lVar3 * 8) + -1) == '\n') {
          putc_unlocked(10,__stream);
        }
        lVar3 = lVar3 + 1;
      } while (lVar3 <= local_48);
    }
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_unidiff_number_range @ 0x8280

void print_unidiff_number_range(void)

{
  long lVar1;
  char *pcVar2;
  long in_FS_OFFSET;
  long local_20;
  long local_18;
  
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010f0c0();
  if (local_20 < local_18) {
    __fprintf_chk(DAT_00126188,1,&DAT_0011b08e,local_20,(local_18 - local_20) + 1);
  }
  else {
    pcVar2 = "%ld,0";
    if (local_20 <= local_18) {
      pcVar2 = "%ld";
    }
    __fprintf_chk(DAT_00126188,1,pcVar2);
  }
  if (lVar1 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: pr_unidiff_hunk @ 0x8320

void pr_unidiff_hunk(undefined8 *param_1)

{
  long lVar1;
  bool bVar2;
  long lVar3;
  int iVar4;
  long lVar5;
  FILE *__stream;
  long lVar6;
  undefined8 *puVar7;
  long lVar8;
  long in_FS_OFFSET;
  long local_78;
  long local_60;
  long local_58;
  long local_50;
  long local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  iVar4 = FUN_0010f1a0(param_1,&local_60,&local_58,&local_50,&local_48);
  if (iVar4 != 0) {
    local_60 = local_60 - DAT_00126570;
    lVar5 = -DAT_00126288;
    if (local_60 < lVar5) {
      local_60 = lVar5;
    }
    local_50 = local_50 - DAT_00126570;
    if (local_50 < lVar5) {
      local_50 = lVar5;
    }
    lVar5 = DAT_00126570 + local_58;
    bVar2 = local_58 < DAT_00126270 - DAT_00126570;
    local_58 = DAT_00126270 + -1;
    if (bVar2) {
      local_58 = lVar5;
    }
    lVar5 = DAT_00126570 + local_48;
    bVar2 = local_48 < DAT_001263a0 - DAT_00126570;
    local_48 = DAT_001263a0 + -1;
    if (bVar2) {
      local_48 = lVar5;
    }
    if (DAT_00126520 == 0) {
      FUN_0010e2b0(DAT_001263a0 - DAT_00126570,local_60);
      __stream = DAT_00126188;
      FUN_0010eb70(4);
      fputs_unlocked("@@ -",__stream);
      FUN_00108280(&DAT_001261a0,local_60,local_58);
      fputs_unlocked(" +",__stream);
      FUN_00108280(&DAT_001262d0,local_50,local_48);
      fputs_unlocked(" @@",__stream);
      FUN_0010eb70(3);
    }
    else {
      lVar5 = FUN_00107c80(DAT_00126258);
      FUN_0010e2b0();
      __stream = DAT_00126188;
      FUN_0010eb70(4);
      fputs_unlocked("@@ -",__stream);
      FUN_00108280(&DAT_001261a0,local_60,local_58);
      fputs_unlocked(" +",__stream);
      FUN_00108280(&DAT_001262d0,local_50,local_48);
      fputs_unlocked(" @@",__stream);
      FUN_0010eb70(3);
      if (lVar5 != 0) {
        FUN_00107d30(__stream,lVar5);
      }
    }
    putc_unlocked(10,__stream);
    local_78 = local_50;
    lVar5 = local_60;
    while ((lVar5 <= local_58 || (local_78 <= local_48))) {
      while ((param_1 == (undefined8 *)0x0 || (lVar5 < (long)param_1[3]))) {
        lVar1 = lVar5 + 1;
        puVar7 = (undefined8 *)(DAT_00126258 + lVar5 * 8);
        if ((DAT_00126491 == '\0') || (*(char *)*puVar7 != '\n')) {
          putc_unlocked((-(uint)(DAT_00126492 == '\0') & 0x17) + 9,__stream);
        }
        FUN_0010f0a0(0,puVar7);
        local_78 = local_78 + 1;
        lVar5 = lVar1;
        if ((local_58 < lVar1) && (local_48 < local_78)) goto LAB_0010852b;
      }
      lVar1 = param_1[2];
      if (lVar1 != 0) {
        lVar8 = lVar5 * 8;
        lVar6 = lVar1;
        do {
          while( true ) {
            lVar3 = DAT_00126258;
            FUN_0010eb70(2);
            putc_unlocked(0x2d,__stream);
            puVar7 = (undefined8 *)(lVar3 + lVar8);
            if ((DAT_00126492 != '\0') && ((DAT_00126491 == '\0' || (*(char *)*puVar7 != '\n')))) {
              putc_unlocked(9,__stream);
            }
            FUN_0010ef20(0,puVar7,1);
            FUN_0010eb70();
            if (*(char *)(puVar7[1] + -1) != '\n') break;
            lVar8 = lVar8 + 8;
            putc_unlocked(10,__stream);
            lVar6 = lVar6 + -1;
            if (lVar6 == 0) goto LAB_00108686;
          }
          lVar8 = lVar8 + 8;
          lVar6 = lVar6 + -1;
        } while (lVar6 != 0);
LAB_00108686:
        lVar5 = lVar5 + lVar1;
      }
      lVar1 = param_1[1];
      if (lVar1 != 0) {
        lVar8 = local_78 * 8;
        lVar6 = lVar1;
        do {
          while( true ) {
            lVar3 = DAT_00126388;
            FUN_0010eb70(1);
            putc_unlocked(0x2b,__stream);
            puVar7 = (undefined8 *)(lVar3 + lVar8);
            if ((DAT_00126492 != '\0') && ((DAT_00126491 == '\0' || (*(char *)*puVar7 != '\n')))) {
              putc_unlocked(9,__stream);
            }
            FUN_0010ef20(0,puVar7,1);
            FUN_0010eb70();
            if (*(char *)(puVar7[1] + -1) != '\n') break;
            lVar8 = lVar8 + 8;
            putc_unlocked(10,__stream);
            lVar6 = lVar6 + -1;
            if (lVar6 == 0) goto LAB_0010874e;
          }
          lVar8 = lVar8 + 8;
          lVar6 = lVar6 + -1;
        } while (lVar6 != 0);
LAB_0010874e:
        local_78 = local_78 + lVar1;
      }
      param_1 = (undefined8 *)*param_1;
    }
  }
LAB_0010852b:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: find_hunk @ 0x8770

long * find_hunk(long *param_1)

{
  long lVar1;
  long lVar2;
  long lVar3;
  long lVar4;
  long *plVar5;
  long lVar6;
  
  lVar2 = param_1[4];
  lVar4 = param_1[3];
  while( true ) {
    plVar5 = param_1;
    param_1 = (long *)*plVar5;
    if (param_1 == (long *)0x0) {
      return plVar5;
    }
    lVar6 = DAT_00126570 * 2 + 1;
    if (*(char *)(param_1 + 5) != '\0') {
      lVar6 = DAT_00126570;
    }
    lVar1 = lVar2 + plVar5[1];
    lVar3 = param_1[3] - (lVar4 + plVar5[2]);
    lVar2 = param_1[4];
    if (lVar3 != lVar2 - lVar1) break;
    lVar4 = param_1[3];
    if (lVar6 <= lVar3) {
      return plVar5;
    }
  }
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: print_context_header @ 0x87e0

void print_context_header(long param_1,undefined8 *param_2,char param_3)

{
  if (param_3 != '\0') {
    FUN_00107ab0(&DAT_0011b0ba,param_1,*param_2,DAT_00126540);
    FUN_00107ab0(&DAT_0011b0d0,param_1 + 0x130,param_2[1],DAT_00126548);
    return;
  }
  FUN_00107ab0(&DAT_0011b0af,param_1,*param_2,DAT_00126540);
  FUN_00107ab0(&DAT_0011b0ba,param_1 + 0x130,param_2[1],DAT_00126548);
  return;
}




// Function: print_context_script @ 0x8870

void print_context_script(long *param_1,char param_2)

{
  undefined8 *puVar1;
  long *plVar2;
  code *pcVar3;
  
  if ((DAT_00126554 == '\0') && (plVar2 = param_1, DAT_001264e0 == 0)) {
    while (plVar2 != (long *)0x0) {
      *(undefined1 *)(plVar2 + 5) = 0;
      puVar1 = (undefined8 *)*plVar2;
      if (puVar1 == (undefined8 *)0x0) break;
      *(undefined1 *)(puVar1 + 5) = 0;
      plVar2 = (long *)*puVar1;
    }
  }
  else {
    FUN_00107be0(param_1);
  }
  pcVar3 = FUN_00108320;
  DAT_00126178 = -DAT_00126288;
  DAT_00126170 = 0x7fffffffffffffff;
  if (param_2 == '\0') {
    pcVar3 = FUN_00107e90;
  }
  FUN_0010eb10(param_1,FUN_00108770,pcVar3);
  return;
}




// Function: exclude_options @ 0x8910

int exclude_options(void)

{
  return (-(uint)(DAT_00126551 == '\0') & 0xfffffff0) + 0x10000010;
}




// Function: add_regexp @ 0x8930

void add_regexp(long *param_1,char *param_2)

{
  long lVar1;
  size_t __length;
  char *pcVar2;
  long lVar3;
  ulong uVar4;
  ulong uVar5;
  long lVar6;
  
  __length = strlen(param_2);
  pcVar2 = re_compile_pattern(param_2,__length,(re_pattern_buffer *)param_1[4]);
  if (pcVar2 != (char *)0x0) {
                    /* WARNING: Subroutine does not return */
    error(2,0,"%s: %s",param_2,pcVar2);
  }
  lVar1 = *param_1;
  lVar6 = param_1[1];
  uVar5 = param_1[2];
  *(bool *)(param_1 + 3) = lVar1 != 0;
  uVar4 = (ulong)((uint)(lVar1 != 0) * 2) + lVar6 + __length;
  param_1[1] = uVar4;
  lVar3 = lVar1;
  if (uVar5 <= uVar4) {
    if (uVar5 == 0) {
      uVar5 = 1;
    }
    do {
      uVar5 = uVar5 * 2;
    } while (uVar5 <= uVar4);
    param_1[2] = uVar5;
    lVar3 = FUN_00115fa0(lVar1);
    *param_1 = lVar3;
  }
  if (lVar1 != 0) {
    lVar1 = lVar6 + 1;
    *(undefined1 *)(lVar3 + lVar6) = 0x5c;
    lVar6 = lVar6 + 2;
    *(undefined1 *)(lVar3 + lVar1) = 0x7c;
  }
  memcpy((void *)(lVar3 + lVar6),param_2,__length + 1);
  return;
}




// Function: usage @ 0x8a10

void usage(void)

{
  FILE *__stream;
  char cVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  char *pcVar4;
  ulong uVar5;
  char *pcVar6;
  undefined8 extraout_RDX;
  undefined **ppuVar7;
  
  uVar3 = DAT_00136860;
  uVar2 = dcgettext(0,"Usage: %s [OPTION]... FILES\n",5);
  ppuVar7 = &PTR_s___normal_output_a_normal_diff__t_00124f20;
  __printf_chk(1,uVar2,uVar3);
  uVar3 = dcgettext(0,"Compare FILES line by line.",5);
  __printf_chk(1,&DAT_0011b114,uVar3);
  __stream = stdout;
  pcVar4 = (char *)dcgettext(0,
                             "Mandatory arguments to long options are mandatory for short options too.\n"
                             ,5);
  fputs_unlocked(pcVar4,__stream);
  cVar1 = ' ';
  pcVar4 = "    --normal                  output a normal diff (the default)";
  do {
    if (cVar1 == '\0') {
      putchar_unlocked(10);
      pcVar4 = ppuVar7[1];
    }
    else {
      pcVar4 = (char *)dcgettext(0,pcVar4,5);
      while (pcVar6 = strchr(pcVar4,10), pcVar6 != (char *)0x0) {
        uVar5 = (long)(pcVar6 + 1) - (long)pcVar4;
        if (0xfff < (int)uVar5) {
                    /* WARNING: Subroutine does not return */
          __assert_fail("msglen < 4096","diff.c",0x3f8,"usage");
        }
        __printf_chk(1,"  %.*s",uVar5 & 0xffffffff,pcVar4);
        pcVar4 = pcVar6 + 1;
      }
      __printf_chk(1,&DAT_0011b135 +
                     (int)(((uint)(*pcVar4 != '-') &
                           (uint)CONCAT71((int7)((ulong)extraout_RDX >> 8),*pcVar4 != ' ')) * 2),
                   pcVar4);
      pcVar4 = ppuVar7[1];
    }
    if (pcVar4 == (char *)0x0) {
      FUN_00115e90();
      return;
    }
    ppuVar7 = ppuVar7 + 1;
    cVar1 = *pcVar4;
  } while( true );
}




// Function: summarize_regexp_list @ 0x8ba0

undefined8 summarize_regexp_list(long *param_1,undefined8 param_2,undefined8 param_3)

{
  long lVar1;
  long lVar2;
  undefined8 in_RAX;
  undefined8 uVar3;
  char *pcVar4;
  
  if (*param_1 == 0) {
    return in_RAX;
  }
  lVar1 = param_1[4];
  uVar3 = FUN_00115f50(0x100);
  lVar2 = param_1[3];
  *(undefined8 *)(lVar1 + 0x20) = uVar3;
  if ((char)lVar2 != '\0') {
    pcVar4 = re_compile_pattern((char *)*param_1,param_1[1],(re_pattern_buffer *)param_1[4]);
    if (pcVar4 != (char *)0x0) {
                    /* WARNING: Subroutine does not return */
      error(2,0,"%s: %s",*param_1,pcVar4);
    }
  }
  return param_3;
}




// Function: try_help @ 0x8c00

void try_help(long param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  uVar1 = DAT_00136860;
  if (param_1 != 0) {
    uVar1 = dcgettext(0,param_1,5);
                    /* WARNING: Subroutine does not return */
    error(0,0,uVar1,param_2);
  }
  uVar2 = dcgettext(0,"Try \'%s --help\' for more information.",5);
                    /* WARNING: Subroutine does not return */
  error(2,0,uVar2,uVar1);
}




// Function: specify_value @ 0x8c60

void specify_value(undefined8 *param_1,char *param_2,undefined8 param_3)

{
  int iVar1;
  undefined8 uVar2;
  
  if ((char *)*param_1 != (char *)0x0) {
    iVar1 = strcmp((char *)*param_1,param_2);
    if (iVar1 != 0) {
      uVar2 = dcgettext(0,"conflicting %s option value \'%s\'",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar2,param_3,param_2);
    }
  }
  *param_1 = param_2;
  return;
}




// Function: specify_colors_style @ 0x8cc0

void specify_colors_style(char *param_1)

{
  int iVar1;
  
  if (param_1 == (char *)0x0) {
    DAT_0012657c = 1;
    return;
  }
  iVar1 = strcmp(param_1,"auto");
  if (iVar1 == 0) {
    DAT_0012657c = 1;
    return;
  }
  iVar1 = strcmp(param_1,"always");
  if (iVar1 == 0) {
    DAT_0012657c = 2;
    return;
  }
  iVar1 = strcmp(param_1,"never");
  if (iVar1 == 0) {
    DAT_0012657c = 0;
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_00108c00("invalid color \'%s\'",param_1);
}




// Function: specify_style @ 0x8d50

void specify_style(int param_1)

{
  int iVar1;
  
  iVar1 = DAT_00126580;
  if ((DAT_00126580 != param_1) && (iVar1 = param_1, DAT_00126580 != 0)) {
                    /* WARNING: Subroutine does not return */
    FUN_00108c00("conflicting output style options",0);
  }
  DAT_00126580 = iVar1;
  return;
}




// Function: option_list @ 0x8d80

undefined1 * option_list(undefined8 *param_1,int param_2)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  long lVar3;
  undefined1 *puVar4;
  undefined1 *puVar5;
  undefined8 *puVar6;
  long lVar7;
  
  if (param_2 < 1) {
    puVar5 = (undefined1 *)FUN_00115f50(1);
    puVar4 = puVar5;
  }
  else {
    lVar7 = 1;
    puVar1 = param_1 + (ulong)(param_2 - 1) + 1;
    puVar6 = param_1;
    do {
      uVar2 = *puVar6;
      puVar6 = puVar6 + 1;
      lVar3 = FUN_001144e0(uVar2);
      lVar7 = lVar7 + 1 + lVar3;
    } while (puVar6 != puVar1);
    puVar4 = (undefined1 *)FUN_00115f50(lVar7);
    puVar5 = puVar4;
    do {
      *puVar5 = 0x20;
      uVar2 = *param_1;
      param_1 = param_1 + 1;
      puVar5 = (undefined1 *)FUN_00114530(puVar5 + 1,uVar2);
    } while (param_1 != puVar1);
  }
  *puVar5 = 0;
  return puVar4;
}




// Function: set_mtime_to_now @ 0x8e10

void set_mtime_to_now(long param_1)

{
  FUN_00110a50(param_1 + 0x58);
  return;
}




// Function: check_stdout @ 0x8e20

void check_stdout(void)

{
  undefined8 uVar1;
  int iVar2;
  
  iVar2 = ferror_unlocked(stdout);
  if (iVar2 != 0) {
                    /* WARNING: Subroutine does not return */
    FUN_0010e230("write failed");
  }
  iVar2 = fclose(stdout);
  if (iVar2 == 0) {
    return;
  }
  uVar1 = dcgettext(0,"standard output",5);
                    /* WARNING: Subroutine does not return */
  FUN_0010e1f0(uVar1);
}




// Function: compare_files @ 0x8e70

int compare_files(long param_1,char *param_2,char *param_3)

{
  int iVar1;
  uint uVar2;
  int iVar3;
  int *piVar4;
  char *pcVar5;
  undefined8 uVar6;
  __off_t _Var7;
  char *pcVar8;
  char *pcVar9;
  undefined8 uVar10;
  long lVar11;
  int iVar12;
  int *piVar13;
  stat *psVar14;
  __dev_t *p_Var15;
  uint uVar16;
  long in_FS_OFFSET;
  bool bVar17;
  bool bVar18;
  byte bVar19;
  char *local_2c8;
  int local_2a8 [2];
  char *local_2a0;
  stat local_298 [2];
  int local_178;
  char *local_170;
  __dev_t local_168;
  __ino_t _Stack_160;
  __nlink_t local_158;
  undefined8 uStack_150;
  ulong local_148;
  __dev_t _Stack_140;
  long local_138;
  __blksize_t _Stack_130;
  __blkcnt_t local_128;
  __time_t _Stack_120;
  long local_118;
  long lStack_110;
  long local_108;
  long lStack_100;
  long local_f8;
  long lStack_f0;
  long local_e8;
  long lStack_e0;
  long local_48;
  long local_40;
  
  bVar19 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if ((param_2 == (char *)0x0) || (param_3 == (char *)0x0)) {
    if ((DAT_00126585 == '\x01') && (param_3 != (char *)0x0)) {
      local_48 = param_1;
      piVar4 = local_2a8;
      for (lVar11 = 0x4c; lVar11 != 0; lVar11 = lVar11 + -1) {
        piVar4[0] = 0;
        piVar4[1] = 0;
        piVar4 = piVar4 + 2;
      }
      if (param_2 == (char *)0x0) {
        local_2a8[0] = -1;
LAB_00109578:
        local_2a8[0] = -1;
        local_178 = -2;
        if (param_3 == (char *)0x0) {
LAB_00109560:
          local_2a8[0] = -1;
          param_2 = (char *)0x0;
          goto joined_r0x00108f50;
        }
      }
      else {
        local_2a8[0] = -2;
        local_178 = 0xfffffffe;
      }
      local_178 = -2;
      param_2 = param_3;
      goto joined_r0x00108f50;
    }
    if (DAT_00126586 == '\0') {
      iVar1 = 1;
      pcVar5 = param_2;
      if (param_2 == (char *)0x0) {
        pcVar5 = param_3;
      }
      FUN_0010e0e0("Only in %s: %s\n",
                   *(undefined8 *)(param_1 + 8 + (ulong)(param_2 == (char *)0x0) * 0x130),pcVar5);
      goto LAB_00109131;
    }
    local_48 = param_1;
    piVar4 = local_2a8;
    for (lVar11 = 0x4c; lVar11 != 0; lVar11 = lVar11 + -1) {
      piVar4[0] = 0;
      piVar4[1] = 0;
      piVar4 = piVar4 + 2;
    }
    if (param_2 == (char *)0x0) {
      local_2a8[0] = -1;
      if (param_3 != (char *)0x0) goto LAB_00109578;
      local_178 = -1;
      goto LAB_00109560;
    }
    local_2a8[0] = -2;
    if (param_3 == (char *)0x0) {
      local_178 = -1;
      param_3 = param_2;
      goto joined_r0x00108f50;
    }
  }
  else {
    local_48 = param_1;
    piVar4 = local_2a8;
    for (lVar11 = 0x4c; lVar11 != 0; lVar11 = lVar11 + -1) {
      piVar4[0] = 0;
      piVar4[1] = 0;
      piVar4 = piVar4 + 2;
    }
    local_2a8[0] = -2;
  }
  local_2a8[0] = -2;
  local_178 = -2;
joined_r0x00108f50:
  if (param_1 == 0) {
    local_2a0 = param_2;
    pcVar5 = (char *)0x0;
    local_2c8 = (char *)0x0;
  }
  else {
    pcVar5 = (char *)FUN_00110920(*(undefined8 *)(param_1 + 8),param_2,0);
    local_2a0 = pcVar5;
    param_3 = (char *)FUN_00110920(*(undefined8 *)(param_1 + 0x138),param_3,0);
    local_2c8 = param_3;
  }
  local_170 = param_3;
  bVar17 = false;
  psVar14 = local_298;
  do {
    if ((int)psVar14[-1].__unused[1] != -1) {
      if ((bVar17) && (iVar1 = strcmp(local_170,local_2a0), iVar1 == 0)) {
        uStack_150 = CONCAT44(local_298[0].st_uid,local_298[0].st_mode);
        local_148 = (ulong)local_298[0].st_gid;
        local_168 = local_298[0].st_dev;
        _Stack_160 = local_298[0].st_ino;
        local_178 = local_2a8[0];
        local_158 = local_298[0].st_nlink;
        _Stack_140 = local_298[0].st_rdev;
        local_138 = local_298[0].st_size;
        _Stack_130 = local_298[0].st_blksize;
        local_128 = local_298[0].st_blocks;
        _Stack_120 = local_298[0].st_atim.tv_sec;
        local_118 = local_298[0].st_atim.tv_nsec;
        lStack_110 = local_298[0].st_mtim.tv_sec;
        local_108 = local_298[0].st_mtim.tv_nsec;
        lStack_100 = local_298[0].st_ctim.tv_sec;
        local_f8 = local_298[0].st_ctim.tv_nsec;
        lStack_f0 = local_298[0].__unused[0];
        local_e8 = local_298[0].__unused[1];
        lStack_e0 = local_298[0].__unused[2];
      }
      else {
        pcVar8 = (char *)psVar14[-1].__unused[2];
        iVar1 = strcmp(pcVar8,"-");
        if (iVar1 == 0) {
          *(undefined4 *)(psVar14[-1].__unused + 1) = 0;
          isatty(0);
          iVar1 = fstat(0,psVar14);
          if (iVar1 != 0) goto LAB_0010926f;
          if ((psVar14->st_mode & 0xf000) == 0x8000) {
            _Var7 = lseek(0,0,1);
            if (_Var7 < 0) {
              piVar4 = __errno_location();
              *(int *)(psVar14[-1].__unused + 1) = -3 - *piVar4;
            }
            else {
              psVar14->st_size = psVar14->st_size - _Var7;
              lVar11 = psVar14->st_size;
              if (psVar14->st_size < 0) {
                lVar11 = 0;
              }
              psVar14->st_size = lVar11;
            }
          }
          FUN_00108e10(psVar14);
        }
        else {
          if (DAT_00126550 == '\0') {
            iVar1 = stat(pcVar8,psVar14);
          }
          else {
            iVar1 = lstat(pcVar8,psVar14);
          }
          if (iVar1 != 0) {
LAB_0010926f:
            piVar4 = __errno_location();
            *(int *)(psVar14[-1].__unused + 1) = -3 - *piVar4;
          }
        }
      }
    }
    psVar14 = (stat *)&psVar14[2].st_nlink;
    if (bVar17) break;
    bVar17 = true;
  } while( true );
  iVar1 = 0;
  piVar4 = local_2a8;
  piVar13 = local_2a8;
LAB_00109084:
  iVar1 = iVar1 + 1;
  if (DAT_00126586 == '\0') {
    if (iVar1 != 1) goto LAB_0010909b;
    if (DAT_00126585 != '\0') goto LAB_001091c0;
    goto LAB_0010920a;
  }
LAB_001091c0:
  iVar12 = *piVar4;
  if (iVar12 == -2) {
    if ((((piVar4[10] & 0xf000U) == 0x8000) && ((piVar4[10] & 0x1ffU) == 0)) &&
       (*(long *)(piVar4 + 0x10) == 0)) goto LAB_001091fa;
  }
  else if (((iVar12 == -5 || iVar12 == -0xc) && (param_1 == 0)) &&
          ((piVar13[0x4c] + 2U & 0xfffffffd) == 0)) {
LAB_001091fa:
    *piVar4 = -1;
    if (iVar1 != 2) goto LAB_0010920a;
LAB_001090a5:
    if (local_2a8[0] != -1) {
      iVar12 = -3 - local_2a8[0];
      if (local_178 == -1) {
        p_Var15 = &local_168;
        for (lVar11 = 0x12; lVar11 != 0; lVar11 = lVar11 + -1) {
          *p_Var15 = 0;
          p_Var15 = p_Var15 + (ulong)bVar19 * -2 + 1;
        }
        uStack_150 = CONCAT44(uStack_150._4_4_,local_298[0].st_mode);
        if (iVar12 < 0) goto LAB_001092c2;
      }
      else if (iVar12 < 0) goto LAB_001092b4;
      piVar4 = __errno_location();
      *piVar4 = iVar12;
      FUN_0010df30(local_2a0);
      iVar12 = -3 - local_178;
      if (-1 < iVar12) {
LAB_001090fd:
        *piVar4 = iVar12;
        FUN_0010df30(local_170);
      }
      goto LAB_0010910c;
    }
    uVar2 = (uint)uStack_150;
    psVar14 = local_298;
    for (lVar11 = 0x12; lVar11 != 0; lVar11 = lVar11 + -1) {
      psVar14->st_dev = 0;
      psVar14 = (stat *)((long)psVar14 + (ulong)bVar19 * -0x10 + 8);
    }
    local_298[0].st_mode = (uint)uStack_150;
    if (local_178 == -1) {
      p_Var15 = &local_168;
      for (lVar11 = 0x12; lVar11 != 0; lVar11 = lVar11 + -1) {
        *p_Var15 = 0;
        p_Var15 = p_Var15 + (ulong)bVar19 * -2 + 1;
      }
      uStack_150 = CONCAT44(uStack_150._4_4_,uVar2);
    }
    else {
LAB_001092b4:
      iVar12 = -3 - local_178;
      if (-1 < iVar12) {
        piVar4 = __errno_location();
        goto LAB_001090fd;
      }
    }
LAB_001092c2:
    if (param_1 == 0) {
      bVar17 = (local_298[0].st_mode & 0xf000) == 0x4000;
      if (bVar17 != (((uint)uStack_150 & 0xf000) == 0x4000)) {
        lVar11 = (long)(int)(uint)!bVar17;
        pcVar8 = (&local_2a0)[(ulong)bVar17 * 0x26];
        uVar10 = FUN_0010f670(pcVar8);
        pcVar5 = (char *)FUN_0010a4f0((&local_2a0)[lVar11 * 0x26],uVar10);
        (&local_2a0)[lVar11 * 0x26] = pcVar5;
        iVar12 = strcmp(pcVar8,"-");
        if (iVar12 == 0) {
                    /* WARNING: Subroutine does not return */
          FUN_0010e230("cannot compare \'-\' to a directory");
        }
        psVar14 = (stat *)((long)local_298 + lVar11 * 0x130);
        if (DAT_00126550 == '\0') {
          iVar12 = stat(pcVar5,psVar14);
        }
        else {
          iVar12 = lstat(pcVar5,psVar14);
        }
        if (iVar12 != 0) {
          FUN_0010df30(pcVar5);
          goto LAB_0010910c;
        }
      }
    }
    if (local_2a8[0] == -1) {
      if (local_178 != -1) {
        bVar17 = false;
        uVar2 = local_298[0].st_mode & 0xf000;
        goto LAB_001095aa;
      }
LAB_00109610:
      iVar1 = 0;
      if ((DAT_00126584 != '\0') && ((local_298[0].st_mode & 0xf000) != 0x4000)) {
        pcVar8 = DAT_00126548;
        if (DAT_00126548 == (char *)0x0) {
          pcVar8 = local_170;
        }
        pcVar9 = DAT_00126540;
        if (DAT_00126540 == (char *)0x0) {
          pcVar9 = local_2a0;
        }
        FUN_0010e0e0("Files %s and %s are identical\n",pcVar9,pcVar8);
      }
    }
    else {
      if (local_178 == -1) {
LAB_001095a0:
        bVar17 = false;
        uVar2 = local_298[0].st_mode & 0xf000;
LAB_001095aa:
        bVar18 = ((uint)uStack_150 & 0xf000) != 0x4000;
        if (uVar2 == 0x4000) {
          if (bVar18) goto LAB_00109811;
          if (DAT_00126580 == 7) {
                    /* WARNING: Subroutine does not return */
            FUN_0010e230("-D option not supported with directories");
          }
          if ((param_1 != 0) && (DAT_00126608 == '\0')) {
            FUN_0010e0e0("Common subdirectories: %s and %s\n",local_2a0,local_170);
            goto LAB_00109610;
          }
LAB_001095f7:
          iVar12 = FUN_0010a0c0(local_2a8,compare_files);
          goto LAB_00109608;
        }
        if (bVar18) goto LAB_00109437;
LAB_00109811:
        if (local_2a8[0] != -1) goto LAB_00109820;
LAB_00109829:
        if ((DAT_00126608 != '\0') &&
           ((DAT_00126586 != '\0' || ((DAT_00126585 != '\0' && (local_2a8[0] == -1))))))
        goto LAB_001095f7;
        if (param_1 == 0) {
                    /* WARNING: Subroutine does not return */
          __assert_fail("parent","diff.c",0x52d,"compare_files");
        }
LAB_00109473:
        iVar1 = 1;
        FUN_0010e0e0("Only in %s: %s\n",
                     *(undefined8 *)(param_1 + 8 + (ulong)(local_2a8[0] == -1) * 0x130),param_2);
      }
      else {
        if ((local_298[0].st_ino == _Stack_160) && (local_298[0].st_dev == local_168)) {
LAB_0010972b:
          if (local_298[0].st_mode == (uint)uStack_150) {
            if (((((local_298[0].st_nlink == local_158) && (local_298[0].st_uid == uStack_150._4_4_)
                  ) && (local_298[0].st_gid == (uint)local_148)) &&
                ((local_298[0].st_size == local_138 && (local_298[0].st_mtim.tv_sec == lStack_110)))
                ) && (local_298[0].st_ctim.tv_sec == lStack_100)) {
              if (DAT_00126578 != '\0') goto LAB_00109610;
              bVar17 = true;
              uVar2 = (uint)uStack_150 & 0xf000;
            }
            else {
              bVar17 = false;
              uVar2 = (uint)uStack_150 & 0xf000;
            }
            goto LAB_001095aa;
          }
          goto LAB_001095a0;
        }
        uVar2 = local_298[0].st_mode & 0xf000;
        uVar16 = (uint)uStack_150 & 0xf000;
        if (uVar2 == 0x6000) {
          if (uVar16 == 0x6000) {
LAB_001097ed:
            if (_Stack_140 == local_298[0].st_rdev) goto LAB_0010972b;
          }
        }
        else {
          bVar17 = false;
          if (uVar2 != 0x2000) goto LAB_001095aa;
          if (uVar16 == 0x2000) goto LAB_001097ed;
        }
        if ((uVar2 == 0x4000) || (bVar17 = false, uVar16 == 0x4000)) {
LAB_00109820:
          if (local_178 == -1) goto LAB_00109829;
LAB_00109780:
          iVar1 = 1;
          uVar10 = FUN_00110830(&local_168);
          pcVar8 = DAT_00126548;
          if (DAT_00126548 == (char *)0x0) {
            pcVar8 = local_170;
          }
          uVar6 = FUN_00110830(local_298);
          pcVar9 = DAT_00126540;
          if (DAT_00126540 == (char *)0x0) {
            pcVar9 = local_2a0;
          }
          FUN_0010df60("File %s is a %s while file %s is a %s\n",pcVar9,uVar6,pcVar8,uVar10);
          goto LAB_0010910c;
        }
LAB_00109437:
        uVar16 = (uint)uStack_150 & 0xf000;
        if ((param_1 != 0) &&
           (((uVar2 & 0xffffdfff) != 0x8000 || (((uint)uStack_150 & 0xd000) != 0x8000)))) {
          if ((local_2a8[0] == -1) || (local_178 == -1)) goto LAB_00109473;
          goto LAB_00109780;
        }
        if ((uVar2 == 0xa000) || (uVar16 == 0xa000)) {
          if (DAT_00126550 == '\0') {
                    /* WARNING: Subroutine does not return */
            __assert_fail("no_dereference_symlinks","diff.c",0x549,"compare_files");
          }
          if ((uVar2 != 0xa000) || (uVar16 != 0xa000)) goto LAB_00109780;
          pcVar8 = (char *)FUN_00116360(local_2a0);
          if (pcVar8 == (char *)0x0) {
            iVar12 = 0;
          }
          else {
            pcVar9 = (char *)FUN_00116360(local_170);
            iVar12 = 1;
            if (pcVar9 != (char *)0x0) {
              iVar1 = strcmp(pcVar8,pcVar9);
              if (iVar1 == 0) {
                free(pcVar8);
                free(pcVar9);
                goto LAB_00109610;
              }
              iVar1 = 1;
              FUN_0010e0e0("Symbolic links %s and %s differ\n",local_2a0,local_170);
              free(pcVar8);
              free(pcVar9);
              goto LAB_0010910c;
            }
          }
          FUN_0010df30((&local_2a0)[(long)iVar12 * 0x26]);
          free(pcVar8);
          free((void *)0x0);
          goto LAB_0010910c;
        }
        if (((((uVar2 == 0x8000 & DAT_00126553) != 0) && (uVar16 == 0x8000)) &&
            (local_298[0].st_size != local_138 && 0 < local_298[0].st_size)) && (0 < local_138)) {
          pcVar8 = DAT_00126548;
          if (DAT_00126548 == (char *)0x0) {
            pcVar8 = local_170;
          }
          pcVar9 = DAT_00126540;
          if (DAT_00126540 == (char *)0x0) {
            pcVar9 = local_2a0;
          }
          iVar1 = 1;
          FUN_0010e0e0("Files %s and %s differ\n",pcVar9,pcVar8);
          goto LAB_0010910c;
        }
        if ((local_2a8[0] == -2) && (local_2a8[0] = open(local_2a0,0,0), local_2a8[0] < 0)) {
          FUN_0010df30(local_2a0);
          if (local_178 == -2) {
            bVar18 = true;
            goto LAB_001098f7;
          }
LAB_00109912:
          if (-1 < local_2a8[0]) {
            iVar12 = 2;
            goto LAB_00109a55;
          }
          if (local_178 < 0) goto LAB_0010910c;
LAB_0010992d:
          iVar12 = close(local_178);
          if (iVar12 != 0) goto LAB_0010993a;
          goto LAB_0010910c;
        }
        bVar18 = false;
        if (local_178 == -2) {
LAB_001098f7:
          if (bVar17) {
            local_178 = local_2a8[0];
          }
          else {
            local_178 = open(local_170,0,0);
            if (local_178 < 0) {
              FUN_0010df30(local_170);
              goto LAB_00109912;
            }
          }
          if (bVar18) goto LAB_00109912;
        }
        iVar12 = FUN_00107460(local_2a8);
        if (-1 < local_2a8[0]) {
LAB_00109a55:
          iVar3 = close(local_2a8[0]);
          if (iVar3 != 0) {
            FUN_0010df30(local_2a0);
            if ((local_178 < 0) || (local_178 == local_2a8[0])) goto LAB_0010910c;
            goto LAB_0010992d;
          }
        }
        if (((-1 < local_178) && (local_178 != local_2a8[0])) &&
           (iVar3 = close(local_178), iVar3 != 0)) {
LAB_0010993a:
          FUN_0010df30(local_170);
          goto LAB_0010910c;
        }
LAB_00109608:
        iVar1 = iVar12;
        if (iVar1 == 0) goto LAB_00109610;
      }
LAB_0010910c:
      iVar12 = fflush_unlocked(stdout);
      if (iVar12 != 0) {
        uVar10 = dcgettext(0,"standard output",5);
                    /* WARNING: Subroutine does not return */
        FUN_0010e1f0(uVar10);
      }
    }
    free(pcVar5);
    free(local_2c8);
LAB_00109131:
    if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    return iVar1;
  }
LAB_0010909b:
  if (iVar1 == 2) goto LAB_001090a5;
LAB_0010920a:
  piVar4 = piVar4 + 0x4c;
  piVar13 = piVar13 + -0x4c;
  goto LAB_00109084;
}




// Function: dir_loop @ 0x9cd0

undefined8 dir_loop(long param_1,int param_2)

{
  long lVar1;
  long lVar2;
  uint uVar3;
  
  lVar1 = *(long *)(param_1 + 0x260);
  if (lVar1 == 0) {
    return 0;
  }
  lVar2 = (long)param_2 * 0x130;
  param_1 = param_1 + lVar2;
  do {
    if ((*(long *)(lVar1 + lVar2 + 0x18) == *(long *)(param_1 + 0x18)) &&
       (*(long *)(lVar1 + lVar2 + 0x10) == *(long *)(param_1 + 0x10))) {
      return 1;
    }
    uVar3 = *(uint *)(lVar1 + 0x28 + lVar2) & 0xf000;
    if (uVar3 == 0x6000) {
      if ((*(uint *)(param_1 + 0x28) & 0xf000) != 0x6000) goto LAB_00109d1a;
      if (*(long *)(lVar1 + 0x38 + lVar2) == *(long *)(param_1 + 0x38)) {
        return 1;
      }
LAB_00109d5c:
      lVar1 = *(long *)(lVar1 + 0x260);
    }
    else {
      if ((uVar3 == 0x2000) && ((*(uint *)(param_1 + 0x28) & 0xf000) == 0x2000)) {
        if (*(long *)(lVar1 + 0x38 + lVar2) == *(long *)(param_1 + 0x38)) {
          return 1;
        }
        goto LAB_00109d5c;
      }
LAB_00109d1a:
      lVar1 = *(long *)(lVar1 + 0x260);
    }
    if (lVar1 == 0) {
      return 0;
    }
  } while( true );
}




// Function: compare_collated @ 0x9da0

void compare_collated(char *param_1,char *param_2)

{
  int *piVar1;
  bool bVar2;
  
  piVar1 = __errno_location();
  bVar2 = DAT_00126551 != '\0';
  *piVar1 = 0;
  if (bVar2) {
    strcasecmp(param_1,param_2);
    return;
  }
  strcoll(param_1,param_2);
  return;
}




// Function: compare_names_for_qsort @ 0x9df0

void compare_names_for_qsort(undefined8 *param_1,undefined8 *param_2)

{
  char *__s1;
  char *__s2;
  int iVar1;
  
  __s1 = (char *)*param_1;
  __s2 = (char *)*param_2;
  if (DAT_001266e8 != '\0') {
    iVar1 = FUN_00109da0(__s1,__s2);
    if (iVar1 != 0) {
      return;
    }
  }
  strcmp(__s1,__s2);
  return;
}




// Function: compare_names @ 0x9e40

void compare_names(char *param_1,char *param_2)

{
  int iVar1;
  
  if (DAT_001266e8 == '\0') {
    strcmp(param_1,param_2);
    return;
  }
  iVar1 = FUN_00109da0();
  if (iVar1 == 0) {
    if (DAT_00126551 == '\0') {
      strcmp(param_1,param_2);
      return;
    }
  }
  return;
}




// Function: dir_read @ 0x9e90

undefined8 dir_read(int *param_1,ulong *param_2)

{
  char *__s;
  char cVar1;
  int iVar2;
  DIR *__dirp;
  char *__s_00;
  int *piVar3;
  dirent *pdVar4;
  size_t sVar5;
  undefined8 *puVar6;
  ulong uVar7;
  undefined8 *puVar8;
  ulong uVar9;
  ulong local_48;
  ulong local_40;
  
  iVar2 = *param_1;
  param_2[1] = 0;
  param_2[2] = 0;
  if (iVar2 == -1) {
    puVar6 = (undefined8 *)FUN_00115f50(8);
    *param_2 = 0;
    param_2[1] = (ulong)puVar6;
LAB_0010a079:
    *puVar6 = 0;
    return 1;
  }
  __dirp = opendir(*(char **)(param_1 + 2));
  if (__dirp != (DIR *)0x0) {
    uVar9 = 0x200;
    __s_00 = (char *)FUN_00115f50(0x200);
    param_2[2] = (ulong)__s_00;
    piVar3 = __errno_location();
    local_48 = 0;
    local_40 = 0;
    while( true ) {
      *piVar3 = 0;
      pdVar4 = readdir(__dirp);
      if (pdVar4 == (dirent *)0x0) break;
      __s = pdVar4->d_name;
      sVar5 = strlen(__s);
      if (((pdVar4->d_name[0] != '.') ||
          ((pdVar4->d_name[1] != '\0' && ((pdVar4->d_name[1] != '.' || (pdVar4->d_name[2] != '\0')))
           ))) && (cVar1 = FUN_00110220(DAT_00126410,__s), cVar1 == '\0')) {
        uVar7 = local_48 + sVar5 + 1;
        while (uVar9 < uVar7) {
          if (0x3ffffffffffffffe < uVar9) goto LAB_0010a0af;
          uVar9 = uVar9 * 2;
          __s_00 = (char *)FUN_00115fa0(__s_00,uVar9);
          param_2[2] = (ulong)__s_00;
        }
        memcpy(__s_00 + local_48,__s,sVar5 + 1);
        local_40 = local_40 + 1;
        local_48 = uVar7;
      }
    }
    iVar2 = *piVar3;
    if (iVar2 == 0) {
      iVar2 = closedir(__dirp);
      if (iVar2 == 0) {
        if (0xffffffffffffffd < local_40) {
LAB_0010a0af:
                    /* WARNING: Subroutine does not return */
          FUN_00116300();
        }
        puVar6 = (undefined8 *)FUN_00115f50(local_40 * 8 + 8);
        *param_2 = local_40;
        param_2[1] = (ulong)puVar6;
        if (local_40 != 0) {
          puVar8 = puVar6 + local_40;
          do {
            *puVar6 = __s_00;
            puVar6 = puVar6 + 1;
            sVar5 = strlen(__s_00);
            __s_00 = __s_00 + sVar5 + 1;
          } while (puVar6 != puVar8);
        }
        goto LAB_0010a079;
      }
    }
    else {
      closedir(__dirp);
      *piVar3 = iVar2;
    }
  }
  return 0;
}




// Function: diff_dirs @ 0xa0c0

int diff_dirs(int *param_1,code *param_2)

{
  undefined8 uVar1;
  long *__src;
  char *__s2;
  char cVar2;
  int iVar3;
  undefined8 uVar4;
  long lVar5;
  long lVar6;
  char *__s1;
  long lVar7;
  long *plVar8;
  long in_FS_OFFSET;
  int local_8c;
  long *local_88 [2];
  size_t local_78;
  long *local_70;
  void *local_68;
  size_t local_60;
  long *local_58;
  void *local_50;
  long local_40;
  
  iVar3 = *param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_8c = 0;
  if (((iVar3 == -1) || (cVar2 = FUN_00109cd0(param_1,0), cVar2 != '\0')) &&
     ((param_1[0x4c] == -1 || (cVar2 = FUN_00109cd0(param_1,1), cVar2 != '\0')))) {
    uVar1 = *(undefined8 *)(param_1 + (ulong)(iVar3 == -1) * 0x4c + 2);
    uVar4 = dcgettext(0,"%s: recursive directory loop",5);
                    /* WARNING: Subroutine does not return */
    error(0,0,uVar4,uVar1);
  }
  cVar2 = FUN_00109e90(param_1,&local_78);
  if (cVar2 == '\0') {
    FUN_0010df30(*(undefined8 *)(param_1 + 2));
    local_8c = 2;
  }
  cVar2 = FUN_00109e90(param_1 + 0x4c,&local_60);
  if (cVar2 == '\0') {
    FUN_0010df30(*(undefined8 *)(param_1 + 0x4e));
    local_8c = 2;
  }
  else if (local_8c == 0) {
    DAT_001266e8 = 1;
    local_88[0] = local_70;
    local_88[1] = local_58;
    iVar3 = _setjmp((__jmp_buf_tag *)&DAT_00126620);
    if (iVar3 != 0) {
      DAT_001266e8 = 0;
    }
    qsort(local_88[0],local_78,8,FUN_00109df0);
    qsort(local_88[1],local_60,8,FUN_00109df0);
    lVar6 = DAT_00126488;
    if ((DAT_00126488 != 0) && (*(long *)(param_1 + 0x98) == 0)) {
      while ((*local_88[0] != 0 && (iVar3 = FUN_00109e40(*local_88[0],lVar6), iVar3 < 0))) {
        local_88[0] = local_88[0] + 1;
      }
      while ((*local_88[1] != 0 && (iVar3 = FUN_00109e40(*local_88[1],lVar6), iVar3 < 0))) {
        local_88[1] = local_88[1] + 1;
      }
    }
    lVar6 = 0;
    while ((*local_88[0] != 0 || (*local_88[1] != 0))) {
      if (*local_88[0] == 0) {
        lVar5 = *local_88[1];
        lVar7 = 0;
        local_88[1] = local_88[1] + 1;
      }
      else {
        lVar5 = lVar6;
        if (*local_88[1] != 0) {
          iVar3 = FUN_00109e40(*local_88[0],*local_88[1]);
          if (iVar3 == 0) {
            if ((DAT_00126551 != '\0') &&
               (iVar3 = strcmp((char *)*local_88[0],(char *)*local_88[1]), iVar3 != 0)) {
              __src = local_88[(iVar3 >> 0x1f) + 1];
              __s1 = (char *)__src[1];
              __s2 = (char *)*local_88[(uint)-(iVar3 >> 0x1f)];
              plVar8 = __src + 1;
              if (__s1 != (char *)0x0) {
                while (iVar3 = FUN_00109e40(__s1,__s2), iVar3 == 0) {
                  iVar3 = strcmp(__s1,__s2);
                  if (-1 < iVar3) {
                    if (iVar3 == 0) {
                      memmove(__src + 1,__src,(long)plVar8 - (long)__src);
                      *__src = (long)__s2;
                    }
                    break;
                  }
                  __s1 = (char *)plVar8[1];
                  plVar8 = plVar8 + 1;
                  if (__s1 == (char *)0x0) break;
                }
              }
            }
            lVar5 = *local_88[1];
            local_88[1] = local_88[1] + 1;
          }
          else if (-1 < iVar3) {
            lVar5 = *local_88[1];
            lVar7 = lVar6;
            local_88[1] = local_88[1] + 1;
            goto LAB_0010a2f5;
          }
        }
        lVar7 = *local_88[0];
        local_88[0] = local_88[0] + 1;
      }
LAB_0010a2f5:
      iVar3 = (*param_2)(param_1,lVar7,lVar5);
      if (local_8c < iVar3) {
        local_8c = iVar3;
      }
    }
  }
  free(local_70);
  free(local_68);
  free(local_58);
  free(local_50);
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_8c;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: find_dir_file_pathname @ 0xa4f0

undefined8 find_dir_file_pathname(undefined8 param_1,char *param_2)

{
  undefined8 *puVar1;
  char *__s1;
  char *pcVar2;
  char cVar3;
  int iVar4;
  undefined8 uVar5;
  undefined8 *puVar6;
  long in_FS_OFFSET;
  char *local_190;
  undefined1 local_178 [8];
  undefined8 *local_170;
  void *local_168;
  undefined4 local_158 [2];
  undefined8 local_150;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_170 = (undefined8 *)0x0;
  local_168 = (void *)0x0;
  local_190 = param_2;
  if (DAT_00126551 != '\0') {
    local_158[0] = 0;
    local_150 = param_1;
    cVar3 = FUN_00109e90(local_158,local_178);
    local_190 = param_2;
    if (cVar3 != '\0') {
      DAT_001266e8 = 1;
      iVar4 = _setjmp((__jmp_buf_tag *)&DAT_00126620);
      local_190 = param_2;
      if (iVar4 == 0) {
        __s1 = (char *)*local_170;
        pcVar2 = param_2;
        puVar6 = local_170;
        while (local_190 = pcVar2, __s1 != (char *)0x0) {
          iVar4 = FUN_00109e40(__s1,param_2);
          if (iVar4 == 0) {
            iVar4 = strcmp(__s1,param_2);
            local_190 = __s1;
            if (iVar4 == 0) break;
            if (param_2 != pcVar2) {
              local_190 = pcVar2;
            }
          }
          puVar1 = puVar6 + 1;
          puVar6 = puVar6 + 1;
          pcVar2 = local_190;
          __s1 = (char *)*puVar1;
        }
      }
    }
  }
  uVar5 = FUN_00110920(param_1,local_190,0);
  free(local_170);
  free(local_168);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_ed_hunk @ 0xa650

void print_ed_hunk(undefined8 param_1)

{
  char *pcVar1;
  uint uVar2;
  long lVar3;
  long in_FS_OFFSET;
  undefined8 local_50;
  undefined8 local_48;
  long local_40;
  long local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar2 = FUN_0010f1a0(param_1,&local_50,&local_48,&local_40,&local_38);
  if (uVar2 != 0) {
    FUN_0010e2b0();
    FUN_0010f0f0(0x2c,&DAT_001261a0,local_50,local_48);
    fputc_unlocked((int)(char)(&DAT_0011d550)[uVar2],DAT_00126188);
    fputc_unlocked(10,DAT_00126188);
    if (uVar2 != 1) {
      if (local_40 <= local_38) {
        do {
          while( true ) {
            lVar3 = local_40 + 1;
            pcVar1 = *(char **)(DAT_00126388 + local_40 * 8);
            local_40 = lVar3;
            if ((*pcVar1 != '.') || (pcVar1[1] != '\n')) break;
            fputs_unlocked("..\n.\ns/.//\n",DAT_00126188);
            if (local_38 < lVar3) goto LAB_0010a68b;
            fputs_unlocked("a\n",DAT_00126188);
          }
          FUN_0010f0a0(&DAT_0011d5b0);
        } while (lVar3 <= local_38);
      }
      fputs_unlocked(".\n",DAT_00126188);
    }
  }
LAB_0010a68b:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: pr_forward_ed_hunk @ 0xa7a0

void pr_forward_ed_hunk(undefined8 param_1)

{
  uint uVar1;
  long lVar2;
  long in_FS_OFFSET;
  undefined8 local_40;
  undefined8 local_38;
  long local_30;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_0010f1a0(param_1,&local_40,&local_38,&local_30,&local_28);
  if (uVar1 != 0) {
    FUN_0010e2b0();
    fputc_unlocked((int)(char)(&DAT_0011d550)[uVar1],DAT_00126188);
    FUN_0010f0f0(0x20,&DAT_001261a0,local_40,local_38);
    fputc_unlocked(10,DAT_00126188);
    if (uVar1 != 1) {
      if (local_30 <= local_28) {
        do {
          lVar2 = local_30 + 1;
          FUN_0010f0a0(&DAT_0011d5b0,DAT_00126388 + local_30 * 8);
          local_30 = lVar2;
        } while (lVar2 <= local_28);
      }
      fputs_unlocked(".\n",DAT_00126188);
    }
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: print_rcs_hunk @ 0xa8a0

void print_rcs_hunk(undefined8 param_1)

{
  uint uVar1;
  long lVar2;
  long in_FS_OFFSET;
  undefined8 local_60;
  undefined8 local_58;
  long local_50;
  long local_48;
  long local_40;
  long local_38;
  long local_30;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_0010f1a0(param_1,&local_60,&local_58,&local_50,&local_48);
  if (uVar1 != 0) {
    FUN_0010e2b0();
    FUN_0010f0c0(&DAT_001261a0,local_60,local_58,&local_40,&local_38);
    if ((uVar1 & 1) != 0) {
      lVar2 = 1;
      if (local_40 <= local_38) {
        lVar2 = (local_38 - local_40) + 1;
      }
      __fprintf_chk(DAT_00126188,1,"d%ld %ld\n",local_40,lVar2);
    }
    if ((uVar1 & 2) != 0) {
      FUN_0010f0c0(&DAT_001262d0,local_50,local_48,&local_30,&local_28);
      lVar2 = 1;
      if (local_30 <= local_28) {
        lVar2 = (local_28 - local_30) + 1;
      }
      __fprintf_chk(DAT_00126188,1,"a%ld %ld\n",local_38,lVar2);
      if (local_50 <= local_48) {
        do {
          lVar2 = local_50 + 1;
          FUN_0010f0a0(&DAT_0011d5b0,DAT_00126388 + local_50 * 8);
          local_50 = lVar2;
        } while (lVar2 <= local_48);
      }
    }
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: print_ed_script @ 0xaa10

void print_ed_script(undefined8 param_1)

{
  FUN_0010eb10(param_1,thunk_FUN_0010eaf0,FUN_0010a650);
  return;
}




// Function: pr_forward_ed_script @ 0xaa30

void pr_forward_ed_script(undefined8 param_1)

{
  FUN_0010eb10(param_1,FUN_0010eaf0,FUN_0010a7a0);
  return;
}




// Function: print_rcs_script @ 0xaa50

void print_rcs_script(undefined8 param_1)

{
  FUN_0010eb10(param_1,FUN_0010eaf0,FUN_0010a8a0);
  return;
}




// Function: scan_char_literal @ 0xaa70

char * scan_char_literal(char *param_1,char *param_2)

{
  char cVar1;
  char cVar2;
  int iVar3;
  char *pcVar4;
  
  cVar1 = *param_1;
  if (cVar1 != '\'') {
    if (cVar1 == '\\') {
      iVar3 = 0;
      cVar2 = '\0';
      cVar1 = param_1[1];
      pcVar4 = param_1 + 2;
      while (cVar1 != '\'') {
        if (7 < (int)cVar1 - 0x30U) {
          return (char *)0x0;
        }
        iVar3 = ((int)cVar1 - 0x30U) + iVar3 * 8;
        cVar2 = (char)iVar3;
        cVar1 = *pcVar4;
        pcVar4 = pcVar4 + 1;
      }
      if (pcVar4 + (-3 - (long)param_1) < (char *)0x3) {
        *param_2 = cVar2;
        return pcVar4;
      }
    }
    else if (cVar1 != '\0') {
      pcVar4 = (char *)0x0;
      if (param_1[1] == '\'') {
        *param_2 = cVar1;
        pcVar4 = param_1 + 2;
      }
      return pcVar4;
    }
  }
  return (char *)0x0;
}




// Function: groups_letter_value @ 0xaaf0

long groups_letter_value(undefined8 *param_1,undefined1 param_2)

{
  long lVar1;
  
  switch(param_2) {
  case 0x45:
    param_1 = param_1 + 3;
switchD_0010ab37_caseD_65:
    lVar1 = FUN_0010f0b0(*param_1,param_1[1]);
    return lVar1 + -1;
  case 0x46:
    param_1 = param_1 + 3;
switchD_0010ab37_caseD_66:
    lVar1 = FUN_0010f0b0(*param_1,param_1[1]);
    return lVar1;
  default:
    switch(param_2) {
    case 0x65:
      goto switchD_0010ab37_caseD_65;
    case 0x66:
      goto switchD_0010ab37_caseD_66;
    default:
      return -1;
    case 0x6c:
      goto switchD_0010ab37_caseD_6c;
    case 0x6d:
      goto switchD_0010ab37_caseD_6d;
    case 0x6e:
      goto switchD_0010ab37_caseD_6e;
    }
  case 0x4c:
    param_1 = param_1 + 3;
switchD_0010ab37_caseD_6c:
    lVar1 = FUN_0010f0b0(*param_1,param_1[2]);
    return lVar1 + -1;
  case 0x4d:
    param_1 = param_1 + 3;
switchD_0010ab37_caseD_6d:
    lVar1 = FUN_0010f0b0(*param_1,param_1[2]);
    return lVar1;
  case 0x4e:
    param_1 = param_1 + 3;
switchD_0010ab37_caseD_6e:
    return param_1[2] - param_1[1];
  }
}




// Function: do_printf_spec @ 0xabe0

byte * do_printf_spec(FILE *param_1,void *param_2,long param_3,undefined8 param_4,undefined8 param_5)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte *pbVar4;
  long lVar5;
  uint uVar6;
  uint uVar7;
  undefined1 *puVar8;
  undefined1 *puVar9;
  undefined1 *puVar11;
  ulong uVar12;
  void *__dest;
  long in_FS_OFFSET;
  undefined1 auStack_68 [8];
  byte *local_60;
  byte *local_58;
  byte *local_50;
  char local_41;
  long local_40;
  undefined1 *puVar10;
  
  puVar9 = auStack_68;
  puVar10 = auStack_68;
  puVar11 = auStack_68;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  pbVar4 = (byte *)((long)param_2 + 1);
  do {
    bVar2 = *pbVar4;
    uVar7 = (uint)bVar2;
    pbVar4 = pbVar4 + 1;
    if (0x30 < bVar2) {
      if ((int)(char)bVar2 - 0x30U < 10) goto LAB_0010aca0;
      goto LAB_0010ac40;
    }
  } while ((0xfffedf7fffffffffU >> ((ulong)bVar2 & 0x3f) & 1) == 0);
  uVar7 = (uint)bVar2;
  uVar6 = (int)(char)bVar2;
  while (uVar6 - 0x30 < 10) {
LAB_0010aca0:
    bVar2 = *pbVar4;
    pbVar4 = pbVar4 + 1;
    uVar7 = (uint)(char)bVar2;
    uVar6 = uVar7;
  }
  if (bVar2 == 0x2e) {
    do {
      uVar7 = (uint)(char)*pbVar4;
      pbVar4 = pbVar4 + 1;
    } while (uVar7 - 0x30 < 10);
  }
LAB_0010ac40:
  bVar2 = *pbVar4;
  pbVar4 = pbVar4 + 1;
  bVar3 = (byte)(uVar7 - 0x58);
  if (bVar3 < 0x21) {
    if ((0x100801001U >> ((ulong)(uVar7 - 0x58) & 0x3f) & 1) == 0) {
      if ((bVar3 == 0xb) && (bVar2 == 0x27)) {
        pbVar4 = (byte *)FUN_0010aa70(pbVar4,&local_41);
        puVar8 = auStack_68;
        if ((pbVar4 != (byte *)0x0) && (puVar8 = auStack_68, param_1 != (FILE *)0x0)) {
          putc_unlocked((int)local_41,param_1);
          puVar8 = auStack_68;
        }
        goto LAB_0010ac69;
      }
    }
    else if (param_3 == 0) {
      lVar5 = FUN_0010aaf0(param_5,(int)(char)bVar2);
      if (-1 < lVar5) goto LAB_0010ad1a;
    }
    else if (bVar2 == 0x6e) {
      lVar5 = FUN_0010f0b0(param_3,param_4);
LAB_0010ad1a:
      puVar8 = auStack_68;
      if (param_1 != (FILE *)0x0) {
        local_60 = pbVar4 + (-2 - (long)param_2);
        local_50 = pbVar4 + (-1 - (long)param_2);
        local_58 = (byte *)lVar5;
        if (pbVar4 + (1 - (long)param_2) < (byte *)0xfa1) {
          puVar8 = auStack_68;
          while (puVar10 !=
                 auStack_68 + -((ulong)(pbVar4 + (0x37 - (long)param_2)) & 0xfffffffffffff000)) {
            puVar9 = puVar8 + -0x1000;
            *(undefined8 *)(puVar8 + -8) = *(undefined8 *)(puVar8 + -8);
            puVar10 = puVar8 + -0x1000;
            puVar8 = puVar8 + -0x1000;
          }
          uVar12 = (ulong)((uint)(pbVar4 + (0x37 - (long)param_2)) & 0xff0);
          lVar5 = -uVar12;
          puVar11 = puVar9 + lVar5;
          if (uVar12 != 0) {
            *(undefined8 *)(puVar9 + -8) = *(undefined8 *)(puVar9 + -8);
          }
          __dest = (void *)(((ulong)(puVar9 + lVar5 + 0xf) & 0xfffffffffffffff0) + 0x1f &
                           0xffffffffffffffe0);
        }
        else {
          __dest = (void *)FUN_00116340();
        }
        pbVar1 = (byte *)((long)__dest + (long)local_50);
        *(undefined8 *)(puVar11 + -8) = 0x10adb3;
        local_50 = local_60;
        local_60 = local_58;
        local_58 = pbVar1;
        memcpy(__dest,param_2,(size_t)local_50);
        *(byte *)((long)__dest + (long)local_50) = 0x6c;
        *local_58 = (byte)uVar7;
        local_58[1] = 0;
        *(undefined8 *)(puVar11 + -8) = 0x10adde;
        __fprintf_chk(param_1,1,__dest,local_60);
        *(undefined8 *)(puVar11 + -8) = 0x10ade6;
        FUN_00111ce0(__dest);
        puVar8 = puVar11;
      }
      goto LAB_0010ac69;
    }
  }
  pbVar4 = (byte *)0x0;
  puVar8 = auStack_68;
LAB_0010ac69:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    *(undefined **)(puVar8 + -8) = &UNK_0010ae31;
    __stack_chk_fail();
  }
  return pbVar4;
}




// Function: print_ifdef_lines @ 0xae40

void print_ifdef_lines(FILE *param_1,char *param_2,long *param_3)

{
  char *pcVar1;
  char cVar2;
  long lVar3;
  long lVar4;
  long lVar5;
  char *pcVar6;
  size_t __n;
  long lVar7;
  long lVar8;
  int __c;
  void *__ptr;
  char *pcVar9;
  
  lVar3 = *param_3;
  lVar7 = param_3[1];
  lVar4 = *(long *)(lVar3 + 0xb8);
  lVar5 = param_3[2];
  if (param_1 != (FILE *)0x0) {
    if ((DAT_001264a0 == '\0') && (*param_2 == '%')) {
      if (param_2[1] == 'l') {
        if ((param_2[2] == '\n') && (param_2[3] == '\0')) {
          if (lVar5 <= lVar7) {
            return;
          }
          __ptr = *(void **)(lVar4 + lVar7 * 8);
          lVar3 = *(long *)(lVar4 + lVar5 * 8);
          __n = (lVar3 + (ulong)(*(char *)(lVar3 + -1) != '\n')) - (long)__ptr;
          goto LAB_0010afd4;
        }
      }
      else if ((param_2[1] == 'L') && (param_2[2] == '\0')) {
        __ptr = *(void **)(lVar4 + lVar7 * 8);
        __n = *(long *)(lVar4 + lVar5 * 8) - (long)__ptr;
LAB_0010afd4:
        fwrite_unlocked(__ptr,1,__n,param_1);
        return;
      }
    }
    if (lVar7 < lVar5) {
      do {
        cVar2 = *param_2;
        pcVar6 = param_2;
        while (pcVar9 = pcVar6 + 1, cVar2 != '\0') {
          while (__c = (int)cVar2, cVar2 == '%') {
            cVar2 = pcVar6[1];
            pcVar1 = pcVar6 + 2;
            if (cVar2 == 'L') {
              lVar8 = *(long *)(lVar4 + 8 + lVar7 * 8);
            }
            else {
              if (cVar2 != 'l') {
                if (cVar2 == '%') {
                  __c = 0x25;
                  pcVar9 = pcVar1;
                }
                else {
                  pcVar6 = (char *)FUN_0010abe0(param_1,pcVar6,lVar3,lVar7,0);
                  if (pcVar6 != (char *)0x0) goto LAB_0010aecb;
                  __c = 0x25;
                }
                break;
              }
              lVar8 = *(long *)(lVar4 + 8 + lVar7 * 8);
              lVar8 = lVar8 - (ulong)(*(char *)(lVar8 + -1) == '\n');
            }
            pcVar9 = pcVar6 + 3;
            FUN_0010ed50(*(undefined8 *)(lVar4 + lVar7 * 8),lVar8,0,0);
            cVar2 = *pcVar1;
            pcVar6 = pcVar1;
            if (cVar2 == '\0') goto LAB_0010af6b;
          }
          putc_unlocked(__c,param_1);
          pcVar6 = pcVar9;
LAB_0010aecb:
          cVar2 = *pcVar6;
        }
LAB_0010af6b:
        lVar7 = lVar7 + 1;
      } while (lVar5 != lVar7);
    }
  }
  return;
}




// Function: format_group @ 0xb040

void format_group(FILE *param_1,char *param_2,char param_3,long param_4)

{
  FILE *pFVar1;
  char *pcVar2;
  int *piVar3;
  intmax_t iVar4;
  char *pcVar5;
  char *pcVar6;
  long lVar7;
  long lVar8;
  char cVar9;
  char cVar10;
  undefined8 uVar11;
  FILE *pFVar12;
  long in_FS_OFFSET;
  char *local_60;
  long local_58 [5];
  
  cVar9 = *param_2;
  local_58[3] = *(long *)(in_FS_OFFSET + 0x28);
  if ((cVar9 != param_3) && (cVar9 != '\0')) {
    do {
      pcVar6 = param_2 + 1;
      cVar10 = cVar9;
      if (cVar9 != '%') {
LAB_0010b08e:
        pcVar2 = pcVar6;
        if (param_1 != (FILE *)0x0) {
          putc_unlocked((int)cVar10,param_1);
        }
        cVar9 = *pcVar2;
        goto LAB_0010b0a5;
      }
      cVar10 = param_2[1];
      pcVar2 = param_2 + 2;
      lVar8 = param_4;
      switch(cVar10) {
      case '%':
        pcVar6 = pcVar2;
        goto LAB_0010b08e;
      default:
        pcVar2 = (char *)FUN_0010abe0(param_1,param_2,0,0,param_4);
        cVar10 = cVar9;
        if (pcVar2 == (char *)0x0) goto LAB_0010b08e;
        cVar9 = *pcVar2;
        goto LAB_0010b0a5;
      case '(':
        lVar8 = 0;
        while( true ) {
          if ((int)*pcVar2 - 0x30U < 10) {
            piVar3 = __errno_location();
            *piVar3 = 0;
            iVar4 = strtoimax(pcVar2,&local_60,10);
            local_58[lVar8] = iVar4;
            pcVar5 = local_60;
            cVar10 = cVar9;
            if (*piVar3 != 0) goto LAB_0010b08e;
          }
          else {
            lVar7 = FUN_0010aaf0(param_4);
            local_58[lVar8] = lVar7;
            cVar10 = cVar9;
            if (lVar7 < 0) goto LAB_0010b08e;
            pcVar5 = pcVar2 + 1;
          }
          pcVar2 = pcVar5 + 1;
          cVar10 = cVar9;
          if (*pcVar5 != (&DAT_0011d194)[lVar8]) goto LAB_0010b08e;
          if (lVar8 == 1) break;
          lVar8 = 1;
        }
        pFVar12 = (FILE *)0x0;
        pFVar1 = param_1;
        if (local_58[0] != local_58[1]) {
          pFVar12 = param_1;
          pFVar1 = (FILE *)0x0;
        }
        pcVar6 = (char *)format_group(pFVar1,pcVar2,0x3a,param_4);
        if ((*pcVar6 != '\0') &&
           (pcVar2 = (char *)format_group(pFVar12,pcVar6 + 1,0x29,param_4), *pcVar2 != '\0')) {
          cVar9 = pcVar2[1];
          pcVar2 = pcVar2 + 1;
          goto LAB_0010b0a5;
        }
        goto LAB_0010b0ae;
      case '<':
        uVar11 = DAT_00126448;
        break;
      case '=':
        uVar11 = DAT_00126440;
        break;
      case '>':
        lVar8 = param_4 + 0x18;
        uVar11 = DAT_00126450;
      }
      FUN_0010ae40(param_1,uVar11,lVar8);
      cVar9 = param_2[2];
LAB_0010b0a5:
      if ((cVar9 == '\0') || (param_2 = pcVar2, param_3 == cVar9)) break;
    } while( true );
  }
LAB_0010b0ae:
  if (local_58[3] != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: format_ifdef @ 0xb2e0

void format_ifdef(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5)

{
  long in_FS_OFFSET;
  undefined8 *local_48;
  undefined8 local_40;
  undefined8 local_38;
  undefined *local_30;
  undefined8 local_28;
  undefined8 local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_48 = &DAT_001261a0;
  local_30 = &DAT_001262d0;
  local_40 = param_2;
  local_38 = param_3;
  local_28 = param_4;
  local_20 = param_5;
  FUN_0010b040(DAT_00126188,param_1,0,&local_48);
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_ifdef_hunk @ 0xb350

void print_ifdef_hunk(undefined8 param_1)

{
  uint uVar1;
  long in_FS_OFFSET;
  long local_30;
  long local_28;
  long local_20;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_0010f1a0(param_1,&local_30,&local_28,&local_20,&local_18);
  if (uVar1 != 0) {
    FUN_0010e2b0();
    if ((DAT_001266f8 < local_30) || (DAT_001266f0 < local_20)) {
      FUN_0010b2e0(DAT_00126460,DAT_001266f8,local_30,DAT_001266f0,local_20);
    }
    DAT_001266f8 = local_28 + 1;
    DAT_001266f0 = local_18 + 1;
    FUN_0010b2e0((&DAT_00126460)[uVar1],local_30,DAT_001266f8,local_20);
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_ifdef_script @ 0xb430

void print_ifdef_script(undefined8 param_1)

{
  DAT_001266f0 = -DAT_00126288;
  DAT_001266f8 = DAT_001266f0;
  FUN_0010eb10(param_1,FUN_0010eaf0,FUN_0010b350);
  if ((DAT_00126270 <= DAT_001266f8) && (DAT_001263a0 <= DAT_001266f0)) {
    return;
  }
  FUN_0010e2b0();
  FUN_0010b2e0(DAT_00126460,DAT_001266f8,DAT_00126270,DAT_001266f0,DAT_001263a0);
  return;
}




// Function: guess_lines @ 0xb4c0

long guess_lines(long param_1,ulong param_2,ulong param_3)

{
  ulong uVar1;
  
  uVar1 = 0x20;
  if (9 < param_1) {
    uVar1 = param_2 / (param_1 - 1U);
  }
  param_3 = param_3 / uVar1;
  if (param_3 == 0) {
    param_3 = 1;
  }
  if (0x787878787878782 < param_3) {
    param_3 = 0x787878787878782;
  }
  return param_3 + 5;
}




// Function: prepare_text @ 0xb510

void prepare_text(long param_1)

{
  char *pcVar1;
  char cVar2;
  undefined8 *__s;
  char *pcVar3;
  char *pcVar4;
  char *pcVar5;
  long lVar6;
  undefined8 *puVar7;
  
  __s = *(undefined8 **)(param_1 + 0xa0);
  if (__s != (undefined8 *)0x0) {
    lVar6 = *(long *)(param_1 + 0xb0);
    if (DAT_00126490 != '\0') {
      pcVar1 = (char *)((long)__s + lVar6);
      *pcVar1 = '\r';
      pcVar3 = rawmemchr(__s,0xd);
      pcVar5 = pcVar1;
      pcVar4 = pcVar3;
      for (; pcVar1 != pcVar3; pcVar3 = pcVar3 + 2) {
        while ((cVar2 = *pcVar3, cVar2 != '\r' || (pcVar3[1] != '\n'))) {
          pcVar3 = pcVar3 + 1;
          pcVar5 = pcVar4 + 1;
          *pcVar4 = cVar2;
          pcVar4 = pcVar5;
          if (pcVar1 == pcVar3) goto LAB_0010b5a0;
        }
        pcVar5 = pcVar4 + 1;
        *pcVar4 = '\n';
        pcVar4 = pcVar5;
      }
LAB_0010b5a0:
      lVar6 = lVar6 - ((long)pcVar1 - (long)pcVar5);
    }
    puVar7 = __s;
    if ((lVar6 != 0) &&
       (puVar7 = (undefined8 *)((long)__s + lVar6), *(char *)((long)__s + lVar6 + -1) != '\n')) {
      *(undefined1 *)puVar7 = 10;
      lVar6 = lVar6 + 1;
      *(undefined1 *)(param_1 + 0x120) = 1;
      puVar7 = (undefined8 *)((long)__s + lVar6);
    }
    *puVar7 = 0;
    *(long *)(param_1 + 0xb0) = lVar6;
  }
  return;
}




// Function: find_and_hash_each_line @ 0xb600

void find_and_hash_each_line(long param_1)

{
  int *piVar1;
  long *plVar2;
  byte bVar3;
  byte bVar4;
  long lVar5;
  void *__s1;
  ushort *puVar6;
  char cVar7;
  uint uVar8;
  char cVar9;
  int iVar10;
  byte *pbVar11;
  ushort **ppuVar12;
  __int32_t **pp_Var13;
  long lVar14;
  uint uVar15;
  byte *__s2;
  ulong uVar16;
  ulong uVar17;
  ulong uVar18;
  ulong uVar19;
  ulong *puVar20;
  ulong uVar21;
  long lVar22;
  byte *pbVar23;
  byte *pbVar24;
  byte *pbVar25;
  bool bVar26;
  bool bVar27;
  ulong local_c0;
  ulong *local_b8;
  byte *local_b0;
  ulong local_a8;
  ulong local_a0;
  ulong local_90;
  long local_88;
  long local_80;
  ulong local_78;
  
  local_88 = *(long *)(param_1 + 0xb8);
  lVar5 = *(long *)(param_1 + 0xc0);
  pbVar25 = *(byte **)(param_1 + 0xe0);
  local_78 = *(ulong *)(param_1 + 0xd8);
  local_80 = FUN_00115f50(local_78 * 8);
  uVar8 = DAT_00126558;
  cVar7 = DAT_00126552;
  pbVar24 = *(byte **)(param_1 + 0xf0);
  local_a8 = DAT_00126708;
  local_90 = DAT_00126700;
  pbVar11 = (byte *)(*(long *)(param_1 + 0xb0) + *(long *)(param_1 + 0xa0));
  bVar27 = DAT_00126558 == 0;
  lVar14 = DAT_00126710;
  if (pbVar25 < pbVar24) {
    local_c0 = 0;
    uVar15 = DAT_00126558 & 1;
    __s2 = pbVar25;
    do {
      uVar18 = DAT_00126498;
      bVar3 = *__s2;
      pbVar25 = __s2 + 1;
      uVar19 = (ulong)bVar3;
      if (uVar8 == 4) {
        if (bVar3 != 10) {
          ppuVar12 = __ctype_b_loc();
          uVar17 = 0;
          puVar6 = *ppuVar12;
          do {
            pbVar23 = pbVar25;
            if ((*(byte *)((long)puVar6 + uVar19 * 2 + 1) & 0x20) != 0) {
              bVar3 = *pbVar25;
              while( true ) {
                pbVar25 = pbVar25 + 1;
                if (bVar3 == 10) goto LAB_0010bcb3;
                uVar19 = (ulong)bVar3;
                if ((*(byte *)((long)puVar6 + uVar19 * 2 + 1) & 0x20) == 0) break;
                bVar3 = *pbVar25;
              }
              uVar17 = (uVar17 << 7 | uVar17 >> 0x39) + 0x20;
              pbVar23 = pbVar25;
            }
            if (cVar7 != '\0') {
              pp_Var13 = __ctype_tolower_loc();
              uVar19 = (ulong)(*pp_Var13)[uVar19];
            }
            pbVar25 = pbVar23 + 1;
            uVar17 = (uVar17 << 7 | uVar17 >> 0x39) + uVar19;
            uVar19 = (ulong)*pbVar23;
          } while (*pbVar23 != 10);
LAB_0010bcb3:
          local_b0 = pbVar25 + (-1 - (long)__s2);
          goto LAB_0010b820;
        }
LAB_0010bd57:
        local_b0 = (byte *)0x0;
        uVar17 = 0;
      }
      else if (uVar8 < 5) {
        if (uVar8 - 1 < 3) {
          if (bVar3 == 10) goto LAB_0010bd57;
          uVar19 = 0;
          uVar17 = 0;
          pbVar23 = pbVar25;
          do {
            uVar21 = (ulong)bVar3;
            if ((uVar8 & 2) != 0) {
              ppuVar12 = __ctype_b_loc();
              pbVar25 = pbVar23;
              for (bVar4 = *(byte *)((long)*ppuVar12 + uVar21 * 2 + 1); (bVar4 & 0x20) != 0;
                  bVar4 = *(byte *)((long)*ppuVar12 + (ulong)bVar4 * 2 + 1)) {
                bVar4 = *pbVar25;
                pbVar25 = pbVar25 + 1;
                if (bVar4 == 10) {
                  local_b0 = pbVar25 + (-1 - (long)__s2);
                  goto LAB_0010b820;
                }
              }
            }
            if (uVar15 == 0) {
LAB_0010ba60:
              uVar16 = 1;
            }
            else if (bVar3 == 9) {
              uVar21 = 0x20;
              uVar16 = uVar18 - uVar19 % uVar18;
              bVar26 = CARRY8(uVar19,uVar16);
              uVar19 = uVar19 + uVar16;
              if (bVar26) {
                uVar19 = 0;
              }
            }
            else if (bVar3 == 0xd) {
              uVar21 = 0xd;
              uVar16 = 1;
              uVar19 = 0;
            }
            else {
              if (bVar3 == 8) {
                uVar21 = 8;
                uVar19 = (uVar19 - 1) + (ulong)(uVar19 == 0);
                goto LAB_0010ba60;
              }
              uVar19 = uVar19 + 1;
              uVar16 = 1;
            }
            if (cVar7 != '\0') {
              pp_Var13 = __ctype_tolower_loc();
              uVar21 = (ulong)*(byte *)(*pp_Var13 + uVar21);
            }
            do {
              uVar17 = (uVar17 << 7 | uVar17 >> 0x39) + uVar21;
              uVar16 = uVar16 - 1;
            } while (uVar16 != 0);
            bVar3 = *pbVar23;
            pbVar25 = pbVar23 + 1;
            pbVar23 = pbVar25;
          } while (bVar3 != 10);
          local_b0 = pbVar25 + (-1 - (long)__s2);
        }
        else {
LAB_0010bbe8:
          if (cVar7 == '\0') {
            uVar17 = 0;
            pbVar23 = pbVar25;
            if (bVar3 == 10) {
              local_b0 = (byte *)0x0;
            }
            else {
              do {
                pbVar25 = pbVar23 + 1;
                uVar17 = (uVar17 << 7 | uVar17 >> 0x39) + uVar19;
                bVar3 = *pbVar23;
                uVar19 = (ulong)bVar3;
                pbVar23 = pbVar25;
              } while (bVar3 != 10);
              local_b0 = pbVar25 + (-1 - (long)__s2);
            }
          }
          else {
            if (bVar3 == 10) goto LAB_0010bd57;
            pp_Var13 = __ctype_tolower_loc();
            uVar17 = 0;
            pbVar23 = pbVar25;
            do {
              pbVar25 = pbVar23 + 1;
              piVar1 = *pp_Var13 + uVar19;
              bVar3 = *pbVar23;
              uVar19 = (ulong)bVar3;
              uVar17 = (uVar17 << 7 | uVar17 >> 0x39) + (long)*piVar1;
              pbVar23 = pbVar25;
            } while (bVar3 != 10);
            local_b0 = pbVar25 + (-1 - (long)__s2);
          }
        }
      }
      else {
        if (uVar8 != 5) goto LAB_0010bbe8;
        if (bVar3 == 10) goto LAB_0010bd57;
        ppuVar12 = __ctype_b_loc();
        uVar17 = 0;
        puVar6 = *ppuVar12;
        do {
          if ((*(byte *)((long)puVar6 + uVar19 * 2 + 1) & 0x20) == 0) {
            if (cVar7 != '\0') {
              pp_Var13 = __ctype_tolower_loc();
              uVar19 = (ulong)(*pp_Var13)[uVar19];
            }
            uVar17 = (uVar17 << 7 | uVar17 >> 0x39) + uVar19;
          }
          bVar3 = *pbVar25;
          uVar19 = (ulong)bVar3;
          pbVar25 = pbVar25 + 1;
        } while (bVar3 != 10);
        local_b0 = pbVar25 + (-1 - (long)__s2);
      }
LAB_0010b820:
      puVar20 = (ulong *)(DAT_00126720 + (uVar17 % DAT_00126718) * 8);
      local_b8 = puVar20;
      if ((((pbVar25 == pbVar11) && (*(char *)(param_1 + 0x120) != '\0')) && (1 < DAT_00126580 - 4U)
          ) && (local_b8 = (ulong *)(DAT_00126720 + -8), 1 < uVar8)) {
        local_b8 = puVar20;
      }
      local_a0 = *local_b8;
      uVar18 = local_a0;
joined_r0x0010b857:
      if (uVar18 != 0) {
        do {
          puVar20 = (ulong *)(uVar18 * 0x20 + lVar14);
          if (puVar20[1] == uVar17) {
            __s1 = (void *)puVar20[2];
            if ((byte *)puVar20[3] == local_b0) {
              iVar10 = memcmp(__s1,__s2,(size_t)local_b0);
              if (iVar10 == 0) goto joined_r0x0010b8cb;
              if (bVar27 && cVar7 == '\0') goto code_r0x0010b934;
            }
            else if (uVar8 == 0) goto LAB_0010b878;
            cVar9 = FUN_0010e690(__s1);
            if (cVar9 == '\0') goto joined_r0x0010b8cb;
          }
LAB_0010b878:
          uVar18 = *puVar20;
          if (uVar18 == 0) break;
        } while( true );
      }
      if (local_90 == local_a8) {
        if (0x1fffffffffffffe < local_90) goto LAB_0010bf49;
        local_90 = local_90 << 1;
        lVar14 = FUN_00115fa0(lVar14);
        local_a0 = *local_b8;
      }
      puVar20 = (ulong *)(local_a8 * 0x20 + lVar14);
      *puVar20 = local_a0;
      puVar20[1] = uVar17;
      puVar20[2] = (ulong)__s2;
      puVar20[3] = (ulong)local_b0;
      *local_b8 = local_a8;
      uVar18 = local_a8;
      local_a8 = local_a8 + 1;
joined_r0x0010b8cb:
      if (local_c0 == local_78) {
        if (((0x2aaaaaaaaaaaaaa9 < (long)local_c0) ||
            (local_78 = local_c0 * 2 - lVar5, 0xffffffffffffffe < local_78)) ||
           (0xffffffffffffffe < local_c0 - lVar5)) goto LAB_0010bf49;
        local_80 = FUN_00115fa0(local_80,local_78 * 8);
        local_88 = FUN_00115fa0(local_88 + lVar5 * 8);
        local_88 = local_88 + lVar5 * -8;
      }
      *(byte **)(local_88 + local_c0 * 8) = __s2;
      *(ulong *)(local_80 + local_c0 * 8) = uVar18;
      local_c0 = local_c0 + 1;
      __s2 = pbVar25;
      if (pbVar24 <= pbVar25) goto LAB_0010bd6b;
    } while( true );
  }
  local_c0 = 0;
LAB_0010bd6b:
  lVar22 = 0;
  *(ulong *)(param_1 + 200) = local_c0;
  do {
    if (local_78 == local_c0) {
      if (((0x2aaaaaaaaaaaaaa9 < (long)local_78) ||
          (uVar18 = local_78 * 2 - lVar5, 0xffffffffffffffe < uVar18)) ||
         (0xffffffffffffffe < local_78 - lVar5)) {
LAB_0010bf49:
                    /* WARNING: Subroutine does not return */
        FUN_00116300();
      }
      local_88 = FUN_00115fa0(local_88 + lVar5 * 8,(uVar18 - lVar5) * 8);
      local_88 = local_88 + lVar5 * -8;
      plVar2 = (long *)(local_88 + local_c0 * 8);
      *plVar2 = (long)pbVar25;
      local_78 = uVar18;
    }
    else {
      plVar2 = (long *)(local_88 + local_c0 * 8);
      *plVar2 = (long)pbVar25;
    }
    if (pbVar25 == pbVar11) {
      if ((*(char *)(param_1 + 0x120) != '\0') && (1 < DAT_00126580 - 4U)) {
        *plVar2 = (long)(pbVar25 + -1);
      }
LAB_0010bea8:
      DAT_00126710 = lVar14;
      *(long *)(param_1 + 0xb8) = local_88;
      *(ulong *)(param_1 + 0xd0) = local_c0;
      *(ulong *)(param_1 + 0xd8) = local_78;
      *(long *)(param_1 + 0xf8) = local_80;
      DAT_00126700 = local_90;
      DAT_00126708 = local_a8;
      return;
    }
    if ((DAT_00126570 <= lVar22) && (DAT_00126578 != '\0')) goto LAB_0010bea8;
    local_c0 = local_c0 + 1;
    pbVar24 = pbVar25;
    do {
      pbVar25 = pbVar24 + 1;
      bVar3 = *pbVar24;
      pbVar24 = pbVar25;
    } while (bVar3 != 10);
    lVar22 = lVar22 + 1;
  } while( true );
code_r0x0010b934:
  uVar18 = *puVar20;
  goto joined_r0x0010b857;
}




// Function: file_block_read @ 0xbf50

void file_block_read(undefined4 *param_1,ulong param_2)

{
  ulong uVar1;
  
  if (param_2 == 0) {
    return;
  }
  if (*(char *)((long)param_1 + 0x121) != '\0') {
    return;
  }
  uVar1 = FUN_00116520(*param_1,*(long *)(param_1 + 0x2c) + *(long *)(param_1 + 0x28),param_2);
  if (uVar1 != 0xffffffffffffffff) {
    *(ulong *)(param_1 + 0x2c) = *(long *)(param_1 + 0x2c) + uVar1;
    *(bool *)((long)param_1 + 0x121) = uVar1 < param_2;
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_0010e1f0(*(undefined8 *)(param_1 + 2));
}




// Function: sip @ 0xbfd0

bool sip(int *param_1,char param_2)

{
  size_t __n;
  int iVar1;
  undefined8 uVar2;
  __off_t _Var3;
  void *pvVar4;
  
  if (*param_1 < 0) {
    param_1[0x2a] = 8;
    param_1[0x2b] = 0;
    uVar2 = FUN_00115f50(8);
    *(undefined8 *)(param_1 + 0x28) = uVar2;
  }
  else {
    uVar2 = FUN_001165d0(8,*(undefined8 *)(param_1 + 0x12),0x7fffffffffffffef);
    *(undefined8 *)(param_1 + 0x2a) = uVar2;
    uVar2 = FUN_00115f50(uVar2);
    *(undefined8 *)(param_1 + 0x28) = uVar2;
    if (param_2 == '\0') {
      iVar1 = thunk_FUN_0010f700(*param_1,0);
      FUN_0010bf50(param_1);
      __n = *(size_t *)(param_1 + 0x2c);
      if (iVar1 != 0) {
        _Var3 = lseek(*param_1,-__n,1);
        if (_Var3 < 0) {
                    /* WARNING: Subroutine does not return */
          FUN_0010e1f0(*(undefined8 *)(param_1 + 2));
        }
        param_1[0x2c] = 0;
        param_1[0x2d] = 0;
        *(undefined1 *)((long)param_1 + 0x121) = 0;
      }
      pvVar4 = memchr(*(void **)(param_1 + 0x28),0,__n);
      return pvVar4 != (void *)0x0;
    }
  }
  param_1[0x2c] = 0;
  param_1[0x2d] = 0;
  *(undefined1 *)((long)param_1 + 0x121) = 0;
  return false;
}




// Function: slurp @ 0xc0d0

void slurp(int *param_1)

{
  undefined8 uVar1;
  ulong uVar2;
  ulong uVar3;
  
  if (*param_1 < 0) {
    return;
  }
  if ((param_1[10] & 0xf000U) == 0x8000) {
    uVar3 = *(ulong *)(param_1 + 0x10);
    uVar2 = (uVar3 & 0xfffffffffffffff8) + 0x10;
    if ((uVar2 < uVar3) || (0x7ffffffffffffffe < uVar2)) {
LAB_0010c266:
                    /* WARNING: Subroutine does not return */
      FUN_00116300();
    }
    if (*(ulong *)(param_1 + 0x2a) < uVar2) {
      *(ulong *)(param_1 + 0x2a) = uVar2;
      uVar1 = FUN_00115fa0(*(undefined8 *)(param_1 + 0x28));
      *(undefined8 *)(param_1 + 0x28) = uVar1;
      uVar2 = *(ulong *)(param_1 + 0x2c);
    }
    else {
      uVar2 = *(ulong *)(param_1 + 0x2c);
    }
    if (uVar2 <= uVar3) {
      FUN_0010bf50(param_1,(uVar3 - uVar2) + 1);
      uVar2 = *(ulong *)(param_1 + 0x2c);
      if (uVar2 <= uVar3) {
        return;
      }
    }
  }
  else {
    uVar2 = *(ulong *)(param_1 + 0x2c);
  }
  FUN_0010bf50(param_1,*(long *)(param_1 + 0x2a) - uVar2);
  uVar2 = *(ulong *)(param_1 + 0x2c);
  if (uVar2 != 0) {
    uVar3 = *(ulong *)(param_1 + 0x2a);
    if (uVar2 == uVar3) {
      do {
        if (0x3ffffffffffffff7 < uVar3) goto LAB_0010c266;
        *(ulong *)(param_1 + 0x2a) = uVar3 * 2;
        uVar1 = FUN_00115fa0(*(undefined8 *)(param_1 + 0x28));
        *(undefined8 *)(param_1 + 0x28) = uVar1;
        FUN_0010bf50(param_1,*(long *)(param_1 + 0x2a) - *(long *)(param_1 + 0x2c));
        uVar2 = *(ulong *)(param_1 + 0x2c);
        uVar3 = *(ulong *)(param_1 + 0x2a);
      } while (uVar2 == uVar3);
    }
    *(ulong *)(param_1 + 0x2a) = uVar2 + 0x10 & 0xfffffffffffffff8;
    uVar1 = FUN_00115fa0(*(undefined8 *)(param_1 + 0x28));
    *(undefined8 *)(param_1 + 0x28) = uVar1;
  }
  return;
}




// Function: find_identical_ends @ 0xc270

void find_identical_ends(int *param_1)

{
  byte bVar1;
  byte *pbVar2;
  byte *pbVar3;
  byte *pbVar4;
  ulong uVar5;
  long lVar6;
  long lVar7;
  ulong uVar8;
  long lVar9;
  ulong uVar10;
  ulong uVar11;
  ulong uVar12;
  uint uVar13;
  byte *pbVar14;
  byte *pbVar15;
  ulong uVar16;
  byte *pbVar17;
  ulong uVar18;
  byte *pbVar19;
  byte *pbVar20;
  bool bVar21;
  byte *local_70;
  byte *local_50;
  byte *local_48;
  long local_40;
  
  FUN_0010c0d0();
  FUN_0010b510(param_1);
  if (*param_1 == param_1[0x4c]) {
    uVar5 = *(ulong *)(param_1 + 0x2c);
    pbVar20 = *(byte **)(param_1 + 0x28);
    *(undefined8 *)(param_1 + 0x76) = *(undefined8 *)(param_1 + 0x2a);
    *(byte **)(param_1 + 0x74) = pbVar20;
    *(ulong *)(param_1 + 0x78) = uVar5;
    *(char *)(param_1 + 0x94) = (char)param_1[0x48];
    uVar11 = uVar5;
LAB_0010c2da:
    pbVar4 = pbVar20 + uVar5;
    pbVar19 = pbVar20 + uVar11;
    uVar13 = DAT_00126580 - 4;
    pbVar14 = pbVar4;
    lVar6 = DAT_00126560;
    local_50 = pbVar4;
    local_70 = pbVar20;
  }
  else {
    FUN_0010c0d0(param_1 + 0x4c);
    FUN_0010b510(param_1 + 0x4c);
    local_70 = *(byte **)(param_1 + 0x74);
    pbVar20 = *(byte **)(param_1 + 0x28);
    uVar11 = *(ulong *)(param_1 + 0x2c);
    uVar5 = *(ulong *)(param_1 + 0x78);
    if (pbVar20 == local_70) goto LAB_0010c2da;
    pbVar19 = pbVar20 + uVar11;
    local_50 = local_70 + uVar5;
    if (uVar11 < uVar5) {
      *pbVar19 = ~local_70[uVar11];
    }
    else {
      *local_50 = ~pbVar20[uVar5];
    }
    pbVar4 = pbVar20;
    pbVar14 = local_70;
    if (*(long *)pbVar20 == *(long *)local_70) {
      do {
        pbVar4 = pbVar4 + 8;
        pbVar14 = pbVar14 + 8;
      } while (*(long *)pbVar4 == *(long *)pbVar14);
    }
    if (*pbVar14 == *pbVar4) {
      do {
        pbVar4 = pbVar4 + 1;
        pbVar14 = pbVar14 + 1;
      } while (*pbVar4 == *pbVar14);
    }
    uVar13 = DAT_00126580 - 4;
    lVar6 = DAT_00126560;
    if ((1 < uVar13) &&
       (pbVar20 + (uVar11 - *(byte *)(param_1 + 0x48)) < pbVar4 !=
        local_70 + (uVar5 - *(byte *)(param_1 + 0x94)) < pbVar14)) {
      pbVar4 = pbVar4 + -1;
      pbVar14 = pbVar14 + -1;
    }
  }
  for (; lVar9 = DAT_00126560, pbVar20 != pbVar4; pbVar4 = pbVar4 + -1) {
    while (pbVar4[-1] != 10) {
      pbVar4 = pbVar4 + -1;
      pbVar14 = pbVar14 + -1;
      if (pbVar20 == pbVar4) goto LAB_0010c33c;
    }
    if (lVar6 == 0) break;
    pbVar14 = pbVar14 + -1;
    lVar6 = lVar6 + -1;
  }
LAB_0010c33c:
  *(byte **)(param_1 + 0x38) = pbVar4;
  *(byte **)(param_1 + 0x84) = pbVar14;
  if ((uVar13 < 2) ||
     (local_48 = local_50, pbVar14 = pbVar19, (char)param_1[0x48] == (char)param_1[0x94])) {
    pbVar14 = pbVar19;
    pbVar3 = local_50;
    pbVar17 = pbVar4 + (uVar11 - uVar5);
    if (uVar11 < uVar5) {
      pbVar17 = pbVar4;
    }
    do {
      local_48 = pbVar3;
      pbVar15 = pbVar14;
      pbVar2 = pbVar17;
      if (pbVar15 == pbVar17) break;
      pbVar14 = pbVar15 + -1;
      pbVar3 = local_48 + -1;
      pbVar2 = pbVar15;
    } while (pbVar15[-1] == local_48[-1]);
    if ((pbVar20 == pbVar15) || (pbVar15[-1] == 10)) {
      if (local_70 != local_48) {
        lVar9 = lVar9 + (ulong)(local_48[-1] != 10);
      }
    }
    else {
      lVar9 = lVar9 + 1;
    }
    lVar6 = lVar9 + -1;
    pbVar14 = pbVar15;
    if ((lVar9 != 0) && (pbVar15 != pbVar19)) {
      do {
        do {
          pbVar14 = pbVar15 + 1;
          bVar1 = *pbVar15;
          pbVar15 = pbVar14;
        } while (bVar1 != 10);
        bVar21 = lVar6 != 0;
        lVar6 = lVar6 + -1;
      } while (pbVar14 != pbVar19 && bVar21);
    }
    local_48 = local_48 + ((long)pbVar14 - (long)pbVar2);
  }
  bVar21 = DAT_00126578 == '\0';
  *(byte **)(param_1 + 0x3c) = pbVar14;
  *(byte **)(param_1 + 0x88) = local_48;
  uVar5 = DAT_00126570;
  if ((((bVar21) || (DAT_00126520 != 0)) || (0x1ffffffffffffffe < (long)DAT_00126570)) ||
     (uVar11 <= DAT_00126570)) {
    uVar18 = 0xffffffffffffffff;
    uVar5 = FUN_0010b4c0(0,0,uVar11);
    local_40 = 0;
  }
  else {
    lVar6 = FUN_0010b4c0(0,0,(long)pbVar14 - (long)pbVar4);
    uVar18 = 0;
    uVar11 = FUN_0010b4c0(0,0,(long)pbVar19 - (long)pbVar14);
    local_40 = 1;
    if (0 < (long)uVar5) {
      do {
        local_40 = local_40 * 2;
      } while (local_40 <= (long)uVar5);
      uVar18 = local_40 - 1;
    }
    if ((long)uVar11 < (long)uVar5) {
      uVar5 = uVar11;
    }
    uVar5 = lVar6 + local_40 + uVar5;
  }
  lVar6 = FUN_00115f50(uVar5 * 8);
  pbVar19 = *(byte **)(param_1 + 0x38);
  pbVar4 = pbVar20;
  if ((DAT_00126578 == '\0') || (pbVar19 != pbVar14)) {
    uVar11 = 0;
    if (pbVar20 == pbVar19) goto LAB_0010c3fb;
LAB_0010c46f:
    do {
      uVar12 = uVar11 + 1;
      uVar16 = uVar5;
      if ((uVar11 & uVar18) == uVar5) {
        if (0x7fffffffffffffe < (long)uVar5) goto LAB_0010c8ea;
        uVar16 = uVar5 * 2;
        lVar6 = FUN_00115fa0(lVar6,uVar5 << 4);
      }
      *(byte **)(lVar6 + (uVar11 & uVar18) * 8) = pbVar4;
      do {
        pbVar14 = pbVar4 + 1;
        bVar1 = *pbVar4;
        pbVar4 = pbVar14;
      } while (bVar1 != 10);
      uVar11 = uVar12;
      uVar5 = uVar16;
    } while (pbVar14 != pbVar19);
    lVar9 = (long)pbVar14 - (long)pbVar20;
    pbVar14 = *(byte **)(param_1 + 0x84);
  }
  else {
    lVar9 = 0;
    uVar12 = 0;
    pbVar14 = local_48;
    if (*(byte **)(param_1 + 0x84) != local_48) {
      uVar11 = 0;
      if (pbVar20 != pbVar19) goto LAB_0010c46f;
LAB_0010c3fb:
      uVar12 = 0;
      lVar9 = 0;
      pbVar14 = *(byte **)(param_1 + 0x84);
    }
  }
  uVar11 = DAT_00126570;
  uVar16 = DAT_00126570;
  if ((long)uVar12 <= (long)DAT_00126570) {
    uVar16 = uVar12;
  }
  if (local_40 == 0) {
    uVar16 = uVar12;
  }
  lVar7 = FUN_0010b4c0(uVar12,lVar9,(long)local_48 - (long)pbVar14);
  uVar8 = FUN_0010b4c0(uVar12,lVar9,(long)local_50 - (long)local_48);
  if ((long)uVar11 < (long)uVar8) {
    uVar8 = uVar11;
  }
  uVar8 = uVar16 + lVar7 + uVar8;
  if (((long)uVar8 < (long)uVar16) || (0xffffffffffffffe < uVar8)) {
LAB_0010c8ea:
                    /* WARNING: Subroutine does not return */
    FUN_00116300();
  }
  lVar9 = FUN_00115f50(uVar8 * 8);
  if (uVar12 == uVar16) {
    if ((long)uVar12 < 1) goto LAB_0010c7a2;
  }
  else {
    if ((long)uVar16 < 1) goto LAB_0010c7a2;
    uVar10 = uVar12 - DAT_00126570;
    uVar11 = uVar16 + uVar10;
    lVar7 = uVar10 * -8;
    do {
      *(undefined8 *)(lVar9 + lVar7 + uVar10 * 8) = *(undefined8 *)(lVar6 + (uVar18 & uVar10) * 8);
      uVar10 = uVar10 + 1;
    } while (uVar11 != uVar10);
    uVar11 = 0;
    do {
      *(undefined8 *)(lVar6 + uVar11 * 8) = *(undefined8 *)(lVar9 + uVar11 * 8);
      uVar11 = uVar11 + 1;
    } while (uVar16 != uVar11);
  }
  lVar7 = 0;
  do {
    *(byte **)(lVar9 + lVar7 * 8) = local_70 + (*(long *)(lVar6 + lVar7 * 8) - (long)pbVar20);
    lVar7 = lVar7 + 1;
  } while (lVar7 < (long)uVar16);
LAB_0010c7a2:
  *(ulong *)(param_1 + 0x86) = uVar12;
  *(ulong *)(param_1 + 0x82) = uVar8 - uVar16;
  *(ulong *)(param_1 + 0x7a) = uVar16 * 8 + lVar9;
  *(ulong *)(param_1 + 0x2e) = lVar6 + uVar16 * 8;
  *(ulong *)(param_1 + 0x7c) = -uVar16;
  *(ulong *)(param_1 + 0x30) = -uVar16;
  *(ulong *)(param_1 + 0x36) = uVar5 - uVar16;
  *(ulong *)(param_1 + 0x3a) = uVar12;
  return;
}




// Function: read_files @ 0xc8f0

uint read_files(int *param_1,uint param_2)

{
  undefined8 uVar1;
  void *__ptr;
  uint uVar2;
  ulong uVar3;
  int iVar4;
  long lVar5;
  char cVar6;
  uint uVar7;
  
  uVar7 = (uint)DAT_00126568;
  uVar2 = FUN_0010bfd0(param_1,uVar7 | param_2 & 0xff);
  uVar2 = param_2 | uVar2;
  if (*param_1 == param_1[0x4c]) {
    *(undefined8 *)(param_1 + 0x74) = *(undefined8 *)(param_1 + 0x28);
    *(undefined8 *)(param_1 + 0x76) = *(undefined8 *)(param_1 + 0x2a);
    *(undefined8 *)(param_1 + 0x78) = *(undefined8 *)(param_1 + 0x2c);
    cVar6 = (char)uVar2;
  }
  else {
    uVar7 = FUN_0010bfd0(param_1 + 0x4c,(uVar7 | param_2 | uVar2) & 0xff);
    uVar2 = uVar2 | uVar7;
    cVar6 = (char)uVar2;
  }
  if (cVar6 != '\0') {
    return uVar2;
  }
  FUN_0010c270(param_1);
  DAT_00126700 = *(long *)(param_1 + 0x82) + *(long *)(param_1 + 0x36) + 1;
  if (0x3fffffffffffffe < DAT_00126700) {
LAB_0010caac:
                    /* WARNING: Subroutine does not return */
    FUN_00116300();
  }
  DAT_00126710 = (void *)FUN_00115f50(DAT_00126700 * 0x20);
  DAT_00126708 = 1;
  if ((ulong)((long)DAT_00126700 / 3) < 0x201) {
    DAT_00126718 = 0x1fd;
    lVar5 = 0xff0;
  }
  else {
    iVar4 = 9;
    do {
      iVar4 = iVar4 + 1;
      uVar3 = 1L << ((byte)iVar4 & 0x3f);
    } while (uVar3 < (ulong)((long)DAT_00126700 / 3));
    DAT_00126718 = uVar3 - (byte)(&DAT_0011d1a0)[iVar4];
    if (0xffffffffffffffe < DAT_00126718) goto LAB_0010caac;
    lVar5 = DAT_00126718 * 8 + 8;
  }
  lVar5 = FUN_0010f5e0(lVar5);
  DAT_00126720 = lVar5 + 8;
  FUN_0010b600(param_1);
  FUN_0010b600(param_1 + 0x4c);
  __ptr = DAT_00126710;
  uVar1 = DAT_00126708;
  *(undefined8 *)(param_1 + 0x96) = DAT_00126708;
  *(undefined8 *)(param_1 + 0x4a) = uVar1;
  free(__ptr);
  free((void *)(DAT_00126720 + -8));
  return uVar2;
}




// Function: print_normal_hunk @ 0xcac0

void print_normal_hunk(undefined8 param_1)

{
  uint uVar1;
  long lVar2;
  long lVar3;
  long lVar4;
  long in_FS_OFFSET;
  long local_50;
  long local_48;
  long local_40;
  long local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_0010f1a0(param_1,&local_50,&local_48,&local_40,&local_38);
  if (uVar1 != 0) {
    FUN_0010e2b0();
    FUN_0010eb70(4);
    FUN_0010f0f0(0x2c,&DAT_001261a0,local_50,local_48);
    fputc_unlocked((int)(char)(&DAT_0011d550)[uVar1],DAT_00126188);
    FUN_0010f0f0(0x2c,&DAT_001262d0,local_40,local_38);
    FUN_0010eb70(3);
    fputc_unlocked(10,DAT_00126188);
    if (((uVar1 & 1) != 0) && (local_50 <= local_48)) {
      lVar2 = local_50 * 8;
      lVar3 = local_50;
      do {
        while( true ) {
          FUN_0010eb70(2);
          lVar4 = DAT_00126258 + lVar2;
          lVar2 = lVar2 + 8;
          FUN_0010ef20(&DAT_0011b1cd,lVar4,1);
          FUN_0010eb70(3);
          if (*(char *)(*(long *)(DAT_00126258 + lVar2) + -1) != '\n') break;
          lVar3 = lVar3 + 1;
          putc_unlocked(10,DAT_00126188);
          if (local_48 < lVar3) goto LAB_0010cc30;
        }
        lVar3 = lVar3 + 1;
      } while (lVar3 <= local_48);
    }
LAB_0010cc30:
    if (uVar1 == 3) {
      fputs_unlocked("---\n",DAT_00126188);
    }
    else if ((uVar1 & 2) == 0) goto LAB_0010cafb;
    if (local_40 <= local_38) {
      lVar2 = local_40 * 8;
      lVar3 = local_40;
      do {
        FUN_0010eb70(1);
        lVar4 = DAT_00126388 + lVar2;
        lVar2 = lVar2 + 8;
        FUN_0010ef20(&DAT_0011b1d0,lVar4,1);
        FUN_0010eb70(3);
        if (*(char *)(*(long *)(DAT_00126388 + lVar2) + -1) == '\n') {
          putc_unlocked(10,DAT_00126188);
        }
        lVar3 = lVar3 + 1;
      } while (lVar3 <= local_38);
    }
  }
LAB_0010cafb:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_normal_script @ 0xccf0

void print_normal_script(undefined8 param_1)

{
  FUN_0010eb10(param_1,FUN_0010eaf0,FUN_0010cac0);
  return;
}




// Function: tab_from_to @ 0xcd10

ulong tab_from_to(ulong param_1,ulong param_2)

{
  ulong uVar1;
  FILE *__stream;
  ulong uVar2;
  ulong uVar3;
  
  uVar2 = DAT_00126498;
  __stream = DAT_00126188;
  if (DAT_001264a0 == '\0') {
    uVar3 = (DAT_00126498 + param_1) - param_1 % DAT_00126498;
    while (uVar1 = uVar3, uVar1 <= param_2) {
      putc_unlocked(9,__stream);
      param_1 = uVar1;
      uVar3 = uVar1 + uVar2;
    }
  }
  if (param_1 < param_2) {
    do {
      param_1 = param_1 + 1;
      putc_unlocked(0x20,__stream);
    } while (param_2 != param_1);
  }
  return param_2;
}




// Function: print_half_line @ 0xcda0

ulong print_half_line(undefined8 *param_1,undefined8 param_2,ulong param_3)

{
  char cVar1;
  char *pcVar2;
  FILE *__stream;
  size_t __n;
  long lVar3;
  char *__ptr;
  char *pcVar4;
  ulong uVar5;
  ulong uVar6;
  int iVar7;
  ulong uVar8;
  long in_FS_OFFSET;
  wchar_t local_4c;
  undefined8 local_48;
  long local_40;
  
  __stream = DAT_00126188;
  uVar8 = 0;
  pcVar2 = (char *)param_1[1];
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_48 = 0;
  if ((char *)*param_1 < pcVar2) {
    __ptr = (char *)*param_1;
    uVar6 = 0;
    do {
      while( true ) {
        cVar1 = *__ptr;
        iVar7 = (int)cVar1;
        pcVar4 = __ptr + 1;
        if ('_' < cVar1) break;
        if (cVar1 < '\b') {
switchD_0010ce21_caseD_6:
          __n = FUN_00116640(&local_4c,__ptr,(long)pcVar2 - (long)__ptr,&local_48);
          if (__n - 1 < 0xfffffffffffffffd) {
            iVar7 = wcwidth(local_4c);
            if (0 < iVar7) {
              uVar6 = (long)iVar7 + uVar6;
            }
            if (uVar6 <= param_3) {
              fwrite_unlocked(__ptr,1,__n,stdout);
              uVar8 = uVar6;
            }
            pcVar4 = __ptr + __n;
          }
          else {
switchD_0010ce21_caseD_3:
            if (uVar6 < param_3) {
              putc_unlocked(iVar7,__stream);
            }
          }
        }
        else {
          switch(iVar7 - 8U & 0xff) {
          case 0:
            if ((uVar6 != 0) && (uVar6 = uVar6 - 1, uVar6 < param_3)) {
              if (uVar6 < uVar8) {
                putc_unlocked(8,__stream);
                uVar8 = uVar6;
              }
              else if (uVar8 < uVar6) {
                do {
                  uVar8 = uVar8 + 1;
                  putc_unlocked(0x20,__stream);
                } while (uVar6 != uVar8);
              }
            }
            break;
          case 1:
            lVar3 = DAT_00126498 - uVar6 % DAT_00126498;
            if (uVar8 == uVar6) {
              uVar5 = lVar3 + uVar8;
              if (DAT_001264a0 == '\0') {
                if (uVar5 < param_3) {
                  putc_unlocked(9,__stream);
                  uVar8 = uVar5;
                }
              }
              else {
                if (param_3 <= uVar5) {
                  uVar5 = param_3;
                }
                if (uVar8 < uVar5) {
                  do {
                    uVar8 = uVar8 + 1;
                    putc_unlocked(0x20,__stream);
                  } while (uVar8 != uVar5);
                }
              }
            }
            uVar6 = uVar6 + lVar3;
            break;
          case 2:
            goto switchD_0010ce21_caseD_2;
          case 3:
          case 4:
            goto switchD_0010ce21_caseD_3;
          case 5:
            uVar6 = 0;
            putc_unlocked(0xd,__stream);
            FUN_0010cd10(0,param_2);
            uVar8 = 0;
            break;
          default:
            goto switchD_0010ce21_caseD_6;
          case 0x18:
          case 0x19:
          case 0x1a:
          case 0x1b:
          case 0x1d:
          case 0x1e:
          case 0x1f:
          case 0x20:
          case 0x21:
          case 0x22:
          case 0x23:
          case 0x24:
          case 0x25:
          case 0x26:
          case 0x27:
          case 0x28:
          case 0x29:
          case 0x2a:
          case 0x2b:
          case 0x2c:
          case 0x2d:
          case 0x2e:
          case 0x2f:
          case 0x30:
          case 0x31:
          case 0x32:
          case 0x33:
          case 0x34:
          case 0x35:
          case 0x36:
          case 0x37:
          case 0x39:
          case 0x3a:
          case 0x3b:
          case 0x3c:
          case 0x3d:
          case 0x3e:
          case 0x3f:
          case 0x40:
          case 0x41:
          case 0x42:
          case 0x43:
          case 0x44:
          case 0x45:
          case 0x46:
          case 0x47:
          case 0x48:
          case 0x49:
          case 0x4a:
          case 0x4b:
          case 0x4c:
          case 0x4d:
          case 0x4e:
          case 0x4f:
          case 0x50:
          case 0x51:
          case 0x52:
          case 0x53:
          case 0x54:
          case 0x55:
          case 0x56:
          case 0x57:
            goto switchD_0010ce21_caseD_18;
          }
        }
LAB_0010cec0:
        __ptr = pcVar4;
        if (pcVar2 <= pcVar4) goto switchD_0010ce21_caseD_2;
      }
      if (0x1d < (byte)(cVar1 + 0x9fU)) goto switchD_0010ce21_caseD_6;
switchD_0010ce21_caseD_18:
      uVar5 = uVar6 + 1;
      if (uVar6 < param_3) {
        putc_unlocked(iVar7,__stream);
        uVar6 = uVar5;
        uVar8 = uVar5;
        goto LAB_0010cec0;
      }
      __ptr = pcVar4;
      uVar6 = uVar5;
    } while (pcVar4 < pcVar2);
  }
switchD_0010ce21_caseD_2:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar8;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_1sdiff_line @ 0xd0a0

void print_1sdiff_line(long param_1,char param_2,undefined8 *param_3)

{
  bool bVar1;
  FILE *__stream;
  long lVar2;
  long lVar3;
  long lVar4;
  undefined8 uVar5;
  int __c;
  bool bVar6;
  
  lVar3 = DAT_00126430;
  lVar2 = DAT_00126428;
  __stream = DAT_00126188;
  if (param_2 == '<') {
    FUN_0010eb70(2);
LAB_0010d202:
    bVar1 = true;
    if (param_1 != 0) {
LAB_0010d0ec:
      bVar6 = *(char *)(*(long *)(param_1 + 8) + -1) == '\n';
      lVar4 = FUN_0010cda0(param_1,0,lVar3);
      goto joined_r0x0010d10b;
    }
    bVar6 = false;
    lVar4 = FUN_0010cd10(0,(ulong)(lVar3 + -1 + lVar2) >> 1);
    lVar4 = lVar4 + 1;
LAB_0010d224:
    __c = (int)param_2;
  }
  else {
    if (param_2 == '>') {
      FUN_0010eb70(1);
      goto LAB_0010d202;
    }
    bVar1 = false;
    if (param_1 != 0) goto LAB_0010d0ec;
    bVar6 = false;
    lVar4 = 0;
    bVar1 = false;
joined_r0x0010d10b:
    if (param_2 == ' ') goto LAB_0010d111;
    lVar4 = FUN_0010cd10(lVar4,(ulong)(lVar3 + -1 + lVar2) >> 1);
    lVar4 = lVar4 + 1;
    if (param_2 != '|') goto LAB_0010d224;
    __c = 0x7c;
    if ((*(char *)(param_3[1] + -1) == '\n') != bVar6) {
      __c = (-(uint)(bVar6 == false) & 0x2d) + 0x2f;
    }
  }
  putc_unlocked(__c,__stream);
LAB_0010d111:
  if ((param_3 != (undefined8 *)0x0) &&
     (bVar6 = (bool)(bVar6 | *(char *)(param_3[1] + -1) == '\n'), *(char *)*param_3 != '\n')) {
    uVar5 = FUN_0010cd10(lVar4,lVar2);
    FUN_0010cda0(param_3,uVar5,lVar3);
  }
  if (bVar6 != false) {
    putc_unlocked(10,__stream);
  }
  if (!bVar1) {
    return;
  }
  FUN_0010eb70(3);
  return;
}




// Function: print_sdiff_common_lines @ 0xd260

void print_sdiff_common_lines(long param_1,long param_2)

{
  long lVar1;
  long lVar2;
  bool bVar3;
  bool bVar4;
  
  lVar2 = DAT_00126730;
  lVar1 = DAT_00126728;
  if (DAT_00126438 == '\0') {
    bVar3 = DAT_00126730 != param_1;
    bVar4 = DAT_00126728 != param_2;
    if ((bVar3) || (bVar4)) {
      if (DAT_0012643a != '\0') {
        __fprintf_chk(DAT_00126188,1,"i%ld,%ld\n",param_1 - DAT_00126730,param_2 - DAT_00126728);
      }
      if (DAT_00126439 == '\0') {
        if ((bVar3) && (bVar4)) {
          do {
            lVar1 = lVar1 + 1;
            lVar2 = lVar2 + 1;
            FUN_0010d0a0(DAT_00126258 + -8 + lVar2 * 8,0x20,DAT_00126388 + -8 + lVar1 * 8);
            if (param_1 == lVar2) goto joined_r0x0010d32b;
          } while (param_2 != lVar1);
        }
        else {
joined_r0x0010d32b:
          for (; param_2 != lVar1; lVar1 = lVar1 + 1) {
            FUN_0010d0a0(0,0x29,DAT_00126388 + lVar1 * 8);
          }
        }
      }
      for (; param_1 != lVar2; lVar2 = lVar2 + 1) {
        FUN_0010d0a0(DAT_00126258 + lVar2 * 8,0x28,0);
      }
    }
  }
  DAT_00126730 = param_1;
  DAT_00126728 = param_2;
  return;
}




// Function: print_sdiff_hunk @ 0xd3c0

void print_sdiff_hunk(undefined8 param_1)

{
  long lVar1;
  long lVar2;
  uint uVar3;
  long lVar4;
  long lVar5;
  long in_FS_OFFSET;
  long local_40;
  long local_38;
  long local_30;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar3 = FUN_0010f1a0(param_1,&local_40,&local_38,&local_30,&local_28);
  lVar4 = DAT_00126730;
  if (uVar3 != 0) {
    FUN_0010d260(local_40,local_30);
    if (DAT_0012643a != '\0') {
      __fprintf_chk(DAT_00126188,1,"c%ld,%ld\n",(local_38 - local_40) + 1,(local_28 - local_30) + 1)
      ;
    }
    if (uVar3 == 3) {
      lVar4 = local_40;
      lVar5 = local_30;
      if (local_40 <= local_38) {
        do {
          local_40 = lVar4;
          local_30 = lVar5;
          if (local_28 < lVar5) break;
          lVar1 = lVar5 * 8;
          lVar5 = lVar5 + 1;
          lVar2 = lVar4 * 8;
          lVar4 = lVar4 + 1;
          FUN_0010d0a0(DAT_00126258 + lVar2,0x7c,DAT_00126388 + lVar1);
          local_40 = lVar4;
          local_30 = lVar5;
        } while (lVar4 <= local_38);
      }
      uVar3 = (uint)(local_40 <= local_38) + (uint)(local_30 <= local_28) * 2;
      DAT_00126728 = local_30;
      DAT_00126730 = local_40;
    }
    lVar4 = DAT_00126728;
    if (((uVar3 & 2) != 0) && (lVar5 = local_30, lVar4 = local_30, local_30 <= local_28)) {
      do {
        lVar4 = lVar5 + 1;
        FUN_0010d0a0(0,0x3e,DAT_00126388 + lVar5 * 8);
        lVar5 = lVar4;
      } while (lVar4 <= local_28);
    }
    DAT_00126728 = lVar4;
    lVar4 = DAT_00126730;
    if (((uVar3 & 1) != 0) && (lVar4 = local_40, local_40 <= local_38)) {
      do {
        lVar5 = lVar4 * 8;
        lVar4 = lVar4 + 1;
        FUN_0010d0a0(DAT_00126258 + lVar5,0x3c,0);
      } while (lVar4 <= local_38);
    }
  }
  DAT_00126730 = lVar4;
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: print_sdiff_script @ 0xd5b0

void print_sdiff_script(undefined8 param_1)

{
  FUN_0010e2b0();
  DAT_00126728 = -DAT_00126288;
  DAT_00126730 = DAT_00126728;
  FUN_0010eb10(param_1,FUN_0010eaf0,FUN_0010d3c0);
  FUN_0010d260(DAT_00126270,DAT_001263a0);
  return;
}




// Function: sighandler @ 0xd600

void sighandler(int param_1)

{
  if (DAT_00126774 == 0) {
    DAT_00126774 = param_1;
  }
  return;
}




// Function: stophandler @ 0xd620

void stophandler(void)

{
  if (DAT_00126774 == 0) {
    DAT_00126770 = DAT_00126770 + 1;
  }
  return;
}




// Function: get_funky_string @ 0xd640

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
  if (cVar4 == '\\') goto LAB_0010d6a6;
LAB_0010d67a:
  if (bVar12 || SBORROW1(cVar4,'\\') != cVar5 < '\0') {
    if (cVar4 == '=') {
      if ((char)param_3 != '\0') goto LAB_0010d6de;
    }
    else if ((cVar4 < '>') && ((cVar4 == '\0' || (cVar4 == ':')))) {
      param_3 = 1;
LAB_0010d6de:
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
      goto LAB_0010d696;
    }
    pcVar2 = pcVar2 + 1;
    if (bVar1 == 0x3f) {
      lVar8 = lVar8 + 1;
      lVar7 = lVar6 + 1;
      *(undefined1 *)(lVar6 + -1) = 0x7f;
      goto LAB_0010d696;
    }
    param_3 = 0;
    goto LAB_0010d6de;
  }
  pcVar2 = pcVar2 + 1;
  lVar7 = lVar6;
LAB_0010d68b:
  *(char *)(lVar7 + -1) = cVar4;
  lVar8 = lVar8 + 1;
  lVar7 = lVar7 + 1;
LAB_0010d696:
  do {
    cVar4 = *pcVar2;
    lVar10 = lVar7 + -1;
    lVar9 = lVar8 + -1;
    cVar5 = cVar4 + -0x5c;
    bVar12 = cVar5 == '\0';
    pcVar3 = pcVar2;
    lVar6 = lVar7;
    if (!bVar12) goto LAB_0010d67a;
LAB_0010d6a6:
    cVar5 = pcVar3[1];
    pcVar2 = pcVar3 + 2;
    if (cVar5 == '\0') {
      param_3 = 0;
      goto LAB_0010d6de;
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
      if (7 < (byte)(cVar5 - 0x30U)) goto LAB_0010d68b;
      do {
        pcVar2 = pcVar2 + 1;
        cVar4 = cVar5 + -0x30 + cVar4 * '\b';
        cVar5 = *pcVar2;
      } while ((byte)(cVar5 - 0x30U) < 8);
      *(char *)(lVar7 + -1) = cVar4;
      lVar8 = lVar8 + 1;
      lVar7 = lVar7 + 1;
      goto LAB_0010d696;
    default:
      goto switchD_0010d6cb_caseD_8;
    case '\x0f':
      cVar5 = '\x7f';
      break;
    case '(':
    case 'H':
      goto switchD_0010d6cb_caseD_28;
    case '/':
      cVar5 = ' ';
      break;
    case '1':
      cVar5 = '\a';
      goto switchD_0010d6cb_caseD_8;
    case '2':
      cVar5 = '\b';
      goto switchD_0010d6cb_caseD_8;
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
switchD_0010d6cb_caseD_8:
    }
    *(char *)(lVar7 + -1) = cVar5;
    lVar8 = lVar8 + 1;
    lVar7 = lVar7 + 1;
  } while( true );
switchD_0010d6cb_caseD_28:
  cVar5 = pcVar3[2];
  pcVar11 = pcVar3 + 3;
  cVar4 = '\0';
  pcVar2 = pcVar3 + 2;
  if ('F' < cVar5) goto LAB_0010d802;
  do {
    if (cVar5 < 'A') {
      if (9 < (byte)(cVar5 - 0x30U)) goto LAB_0010d68b;
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
LAB_0010d802:
      if (5 < (byte)(cVar5 + 0x9fU)) goto LAB_0010d68b;
      cVar4 = cVar5 + -0x57 + cVar4 * '\x10';
      pcVar2 = pcVar11;
    }
  } while( true );
}




// Function: c_escape_char @ 0xd8b0

bool c_escape_char(char param_1)

{
  if (param_1 < '#') {
    if ('\x06' < param_1) {
      switch(param_1) {
      case '\a':
        param_1 = 'a';
        break;
      case '\b':
        param_1 = 'b';
        break;
      case '\t':
        return (bool)0x74;
      case '\n':
        return (bool)0x6e;
      case '\v':
        return (bool)0x76;
      case '\f':
        return (bool)0x66;
      case '\r':
        return (bool)0x72;
      default:
        goto switchD_0010d8d7_caseD_e;
      case '\"':
        break;
      }
      return (bool)param_1;
    }
  }
  else if (param_1 == '\\') {
    return (bool)0x5c;
  }
switchD_0010d8d7_caseD_e:
  return param_1 < ' ';
}




// Function: c_escape @ 0xd970

byte * c_escape(byte *param_1)

{
  long lVar1;
  char cVar2;
  byte bVar3;
  byte *pbVar4;
  byte bVar5;
  long lVar6;
  ulong uVar7;
  byte *pbVar8;
  byte *pbVar9;
  char cVar10;
  undefined8 uVar11;
  
  bVar3 = *param_1;
  if (bVar3 != 0) {
    uVar11 = 0;
    lVar6 = 0;
    pbVar8 = param_1;
    do {
      while (bVar3 != 0x20) {
        cVar2 = FUN_0010d8b0();
        cVar10 = (char)uVar11;
        if (cVar2 != '\0') {
          lVar1 = lVar6 + 3;
          lVar6 = lVar6 + 1;
          if (cVar2 == '\x01') {
            lVar6 = lVar1;
          }
        }
        bVar3 = pbVar8[1];
        pbVar8 = pbVar8 + 1;
        if (bVar3 == 0) goto LAB_0010d9cd;
      }
      bVar3 = pbVar8[1];
      pbVar8 = pbVar8 + 1;
      uVar11 = 1;
      cVar10 = '\x01';
    } while (bVar3 != 0);
LAB_0010d9cd:
    if ((lVar6 != 0) || (cVar10 != '\0')) {
      pbVar4 = (byte *)FUN_00115f50(pbVar8 + ((lVar6 + 3) - (long)param_1));
      *pbVar4 = 0x22;
      bVar3 = *param_1;
      pbVar8 = pbVar4 + 1;
      while (bVar3 != 0) {
        uVar7 = (ulong)bVar3;
        bVar3 = FUN_0010d8b0((int)(char)bVar3);
        bVar5 = (byte)uVar7;
        if (bVar3 == 0) {
          *pbVar8 = bVar5;
          pbVar9 = pbVar8 + 1;
        }
        else if (bVar3 == 1) {
          *pbVar8 = 0x5c;
          pbVar9 = pbVar8 + 4;
          pbVar8[1] = ((byte)((uVar7 & 0xffffffff) >> 6) & 3) + 0x30;
          pbVar8[3] = (bVar5 & 7) + 0x30;
          pbVar8[2] = ((char)bVar5 >> 3 & 7U) + 0x30;
        }
        else {
          *pbVar8 = 0x5c;
          pbVar9 = pbVar8 + 2;
          pbVar8[1] = bVar3;
        }
        pbVar8 = param_1 + 1;
        param_1 = param_1 + 1;
        bVar3 = *pbVar8;
        pbVar8 = pbVar9;
      }
      pbVar8[0] = 0x22;
      pbVar8[1] = 0;
      return pbVar4;
    }
  }
  return param_1;
}




// Function: parse_diff_color @ 0xda80

void parse_diff_color(void)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  char *pcVar4;
  undefined8 in_R8;
  undefined8 in_R9;
  long lVar5;
  long in_FS_OFFSET;
  undefined **ppuVar6;
  char *local_58;
  undefined *local_50;
  char local_43 [3];
  long local_40;
  
  pcVar4 = DAT_00126748;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_58 = DAT_00126748;
  if ((DAT_00126748 == (char *)0x0) || (*DAT_00126748 == '\0')) {
LAB_0010dab4:
    if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    return;
  }
  local_43[2] = 0;
  local_43[0] = '?';
  local_43[1] = '?';
  local_50 = (undefined *)FUN_001162e0(DAT_00126748);
  while( true ) {
    while( true ) {
      while (cVar1 = *pcVar4, cVar1 == '*') {
        lVar5 = FUN_00115f50(0x28);
        ppuVar6 = &local_50;
        *(long *)(lVar5 + 0x20) = DAT_00126750;
        DAT_00126750 = lVar5;
        *(undefined **)(lVar5 + 8) = local_50;
        local_58 = pcVar4 + 1;
        cVar1 = FUN_0010d640(ppuVar6,&local_58,1,lVar5,in_R8,in_R9,ppuVar6);
        if ((cVar1 == '\0') ||
           (pcVar4 = local_58 + 1, cVar1 = *local_58, local_58 = pcVar4, cVar1 != '='))
        goto LAB_0010dbda;
        *(undefined **)(lVar5 + 0x18) = local_50;
        cVar1 = FUN_0010d640(ppuVar6,&local_58,0,lVar5 + 0x10,in_R8,in_R9,ppuVar6);
        pcVar4 = local_58;
        if (cVar1 == '\0') goto LAB_0010dbda;
      }
      if (cVar1 != ':') break;
      local_58 = pcVar4 + 1;
      pcVar4 = local_58;
    }
    if (cVar1 == '\0') goto LAB_0010dab4;
    local_43[0] = cVar1;
    local_58 = pcVar4 + 1;
    if (pcVar4[1] == '\0') break;
    local_43[1] = pcVar4[1];
    local_58 = pcVar4 + 3;
    if (pcVar4[2] != '=') break;
    lVar5 = 0;
    pcVar4 = "lc";
    while (iVar2 = strcmp(local_43,pcVar4), iVar2 != 0) {
      lVar5 = lVar5 + 1;
      pcVar4 = (&PTR_DAT_00125900)[lVar5];
      if (pcVar4 == (char *)0x0) goto LAB_0010dbb6;
    }
    (&PTR_DAT_00126028)[(long)(int)lVar5 * 2] = local_50;
    cVar1 = FUN_0010d640(&local_50,&local_58,0);
    pcVar4 = local_58;
    if (cVar1 == '\0') {
LAB_0010dbb6:
      uVar3 = dcgettext(0,"unrecognized prefix: %s",5);
                    /* WARNING: Subroutine does not return */
      error(0,0,uVar3,local_43);
    }
  }
LAB_0010dbda:
  uVar3 = dcgettext(0,"unparsable value for --palette",5);
                    /* WARNING: Subroutine does not return */
  error(0,0,uVar3);
}




// Function: install_signal_handlers @ 0xdcf0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void install_signal_handlers(void)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  int iVar4;
  long in_FS_OFFSET;
  _union_1457 local_d8;
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
  undefined8 local_60;
  undefined8 uStack_58;
  undefined4 local_50;
  long local_40;
  
  iVar4 = 0x14;
  piVar2 = &DAT_0011d524;
  piVar3 = &DAT_0011d524;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  sigemptyset((sigset_t *)&DAT_00126780);
  while( true ) {
    sigaction(iVar4,(sigaction *)0x0,(sigaction *)&local_d8);
    if (local_d8.sa_handler != (__sighandler_t)0x1) {
      sigaddset((sigset_t *)&DAT_00126780,iVar4);
    }
    if (piVar3 == (int *)&DAT_0011d550) break;
    iVar4 = *piVar3;
    piVar3 = piVar3 + 1;
  }
  local_50 = 0x10000000;
  iVar4 = 0x14;
  local_d0 = _DAT_00126780;
  uStack_c8 = uRam0000000000126788;
  local_c0 = _DAT_00126790;
  uStack_b8 = uRam0000000000126798;
  local_b0 = _DAT_001267a0;
  uStack_a8 = uRam00000000001267a8;
  local_a0 = _DAT_001267b0;
  uStack_98 = uRam00000000001267b8;
  local_90 = _DAT_001267c0;
  uStack_88 = uRam00000000001267c8;
  local_80 = _DAT_001267d0;
  uStack_78 = uRam00000000001267d8;
  local_70 = _DAT_001267e0;
  uStack_68 = uRam00000000001267e8;
  local_60 = _DAT_001267f0;
  uStack_58 = uRam00000000001267f8;
  while( true ) {
    iVar1 = sigismember((sigset_t *)&DAT_00126780,iVar4);
    if (iVar1 != 0) {
      local_d8.sa_handler = FUN_0010d600;
      if (iVar4 == 0x14) {
        local_d8.sa_handler = FUN_0010d620;
      }
      sigaction(iVar4,(sigaction *)&local_d8,(sigaction *)0x0);
    }
    if (piVar2 == (int *)&DAT_0011d550) break;
    iVar4 = *piVar2;
    piVar2 = piVar2 + 1;
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: check_color_output @ 0xde60

void check_color_output(char param_1)

{
  int iVar1;
  bool bVar2;
  
  if ((DAT_00126188 == (FILE *)0x0) || (DAT_0012657c == 0)) {
    return;
  }
  if (DAT_00126180 == '\0') {
    if (param_1 != '\0') {
      if (DAT_0012657c != 2) {
        DAT_00126758 = 0;
        return;
      }
      DAT_00126758 = 1;
      FUN_0010da80();
      return;
    }
    iVar1 = fileno(DAT_00126188);
    iVar1 = isatty(iVar1);
    bVar2 = iVar1 != 0;
  }
  else {
    bVar2 = true;
  }
  if ((DAT_0012657c == 2) || ((DAT_0012657c == 1 && (bVar2)))) {
    DAT_00126758 = 1;
    FUN_0010da80();
  }
  else {
    DAT_00126758 = 0;
  }
  if (!bVar2) {
    return;
  }
  FUN_0010dcf0();
  return;
}




// Function: perror_with_name @ 0xdf30

void perror_with_name(undefined8 param_1)

{
  int *piVar1;
  
  piVar1 = __errno_location();
                    /* WARNING: Subroutine does not return */
  error(0,*piVar1,"%s",param_1);
}




// Function: message5 @ 0xdf60

void message5(char *param_1,long param_2,undefined8 param_3,undefined *param_4,
                 undefined *param_5)

{
  size_t sVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  void *pvVar4;
  undefined8 *puVar5;
  long lVar6;
  char *__s;
  long lVar7;
  long in_FS_OFFSET;
  long local_80 [2];
  undefined *local_70;
  undefined *local_68;
  size_t local_58 [5];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00126480 == '\0') {
    if (DAT_0012643a != '\0') {
      putchar_unlocked(0x20);
    }
    uVar2 = dcgettext(0,param_1,5);
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      __printf_chk(1,uVar2,param_2,param_3,param_4,param_5);
      return;
    }
  }
  else {
    lVar7 = 8;
    if (param_4 == (undefined *)0x0) {
      param_4 = &DAT_0011d5b0;
    }
    if (param_5 == (undefined *)0x0) {
      param_5 = &DAT_0011d5b0;
    }
    lVar6 = 0;
    __s = param_1;
    local_80[0] = param_2;
    local_80[1] = param_3;
    local_70 = param_4;
    local_68 = param_5;
    while( true ) {
      sVar1 = strlen(__s);
      *(size_t *)((long)local_58 + lVar6) = sVar1 + 1;
      lVar7 = lVar7 + sVar1 + 1;
      if (lVar6 + 8 == 0x28) break;
      __s = *(char **)((long)local_80 + lVar6);
      lVar6 = lVar6 + 8;
    }
    puVar3 = (undefined8 *)FUN_00115f50(lVar7);
    puVar5 = puVar3 + 1;
    lVar7 = 0;
    while( true ) {
      sVar1 = *(size_t *)((long)local_58 + lVar7);
      pvVar4 = memcpy(puVar5,param_1,sVar1);
      puVar5 = (undefined8 *)((long)pvVar4 + sVar1);
      if (lVar7 + 8 == 0x28) break;
      param_1 = *(char **)((long)local_80 + lVar7);
      lVar7 = lVar7 + 8;
    }
    puVar5 = puVar3;
    *(undefined8 **)PTR_DAT_001260a0 = puVar3;
    PTR_DAT_001260a0 = (undefined *)puVar5;
    *puVar3 = 0;
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: message @ 0xe0e0

void message(void)

{
  FUN_0010df60();
  return;
}




// Function: print_message_queue @ 0xe0f0

void print_message_queue(void)

{
  long *plVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long lVar5;
  size_t sVar6;
  undefined8 uVar7;
  long *__s;
  long *__ptr;
  long *plVar8;
  long in_FS_OFFSET;
  long local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 local_50;
  long local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00126800 != (long *)0x0) {
    __ptr = DAT_00126800;
    do {
      plVar1 = (long *)*__ptr;
      __s = __ptr + 1;
      plVar8 = &local_68;
      do {
        plVar8 = plVar8 + 1;
        sVar6 = strlen((char *)__s);
        lVar5 = local_48;
        uVar4 = local_50;
        uVar3 = local_58;
        uVar2 = local_60;
        __s = (long *)((long)__s + sVar6 + 1);
        *plVar8 = (long)__s;
      } while (&local_48 != plVar8);
      uVar7 = dcgettext(0,__ptr + 1,5);
      __printf_chk(1,uVar7,uVar2,uVar3,uVar4,lVar5);
      free(__ptr);
      __ptr = plVar1;
    } while (plVar1 != (long *)0x0);
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: pfatal_with_name @ 0xe1f0

void pfatal_with_name(undefined8 param_1)

{
  int iVar1;
  int *piVar2;
  
  piVar2 = __errno_location();
  iVar1 = *piVar2;
  FUN_0010e0f0();
                    /* WARNING: Subroutine does not return */
  error(2,iVar1,"%s",param_1);
}




// Function: fatal @ 0xe230

void fatal(undefined8 param_1)

{
  undefined8 uVar1;
  
  FUN_0010e0f0();
  uVar1 = dcgettext(0,param_1,5);
                    /* WARNING: Subroutine does not return */
  error(2,0,"%s",uVar1);
}




// Function: set_color_palette @ 0xe270

void set_color_palette(undefined8 param_1)

{
  DAT_00126748 = param_1;
  return;
}




// Function: setup_output @ 0xe280

void setup_output(undefined8 param_1,undefined8 param_2,undefined1 param_3)

{
  DAT_00126768 = param_1;
  DAT_00126760 = param_2;
  DAT_00126759 = param_3;
  DAT_00126188 = 0;
  return;
}




// Function: begin_output @ 0xe2b0

void begin_output(void)

{
  int iVar1;
  void *__ptr;
  int *piVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  int local_60;
  int local_5c;
  void *local_58;
  void *local_50;
  char *local_48;
  undefined *local_40;
  void *local_38;
  undefined8 local_30;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00126188 == (FILE *)0x0) {
    local_58 = (void *)FUN_0010d970(DAT_00126768);
    local_50 = (void *)FUN_0010d970(DAT_00126760);
    __ptr = (void *)FUN_00116460("diff%s %s %s",DAT_00126420,local_58,local_50);
    if (DAT_00126480 == '\0') {
      DAT_00126188 = stdout;
      FUN_0010de60(0);
      if (DAT_00126759 != '\0') {
        __printf_chk(1,&DAT_0011b137,__ptr);
      }
    }
    else {
      iVar1 = fflush_unlocked(stdout);
      if (iVar1 != 0) {
        uVar3 = dcgettext(0,"write failed",5);
                    /* WARNING: Subroutine does not return */
        FUN_0010e1f0(uVar3);
      }
      local_48 = "/usr/bin/pr";
      local_40 = &DAT_0011d58c;
      local_30 = 0;
      local_38 = __ptr;
      iVar1 = pipe(&local_60);
      if (iVar1 != 0) {
                    /* WARNING: Subroutine does not return */
        FUN_0010e1f0(&DAT_0011d58f);
      }
      DAT_00126740 = fork();
      if (DAT_00126740 < 0) {
                    /* WARNING: Subroutine does not return */
        FUN_0010e1f0(&DAT_0011d594);
      }
      if (DAT_00126740 == 0) {
        close(local_5c);
        if (local_60 != 0) {
          iVar1 = dup2(local_60,0);
          if (iVar1 < 0) {
                    /* WARNING: Subroutine does not return */
            FUN_0010e1f0(&DAT_0011d599);
          }
          close(local_60);
        }
        execv("/usr/bin/pr",&local_48);
        piVar2 = __errno_location();
                    /* WARNING: Subroutine does not return */
        _exit((*piVar2 == 2) + 0x7e);
      }
      close(local_60);
      DAT_00126188 = fdopen(local_5c,"w");
      if (DAT_00126188 == (FILE *)0x0) {
                    /* WARNING: Subroutine does not return */
        FUN_0010e1f0("fdopen");
      }
      FUN_0010de60(1);
    }
    free(__ptr);
    if (DAT_00126580 == 2) {
      FUN_001087e0(&DAT_001261a0,&local_58,0);
    }
    else if (DAT_00126580 == 3) {
      FUN_001087e0(&DAT_001261a0,&local_58,1);
    }
    if (local_58 != DAT_00126768) {
      free(local_58);
    }
    if (local_50 != DAT_00126760) {
      free(local_50);
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: finish_output @ 0xe550

void finish_output(void)

{
  byte bVar4;
  int iVar1;
  __pid_t _Var2;
  undefined8 uVar3;
  uint uVar5;
  char *pcVar6;
  long in_FS_OFFSET;
  uint local_14;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if ((DAT_00126188 != (FILE *)0x0) && (DAT_00126188 != stdout)) {
    iVar1 = ferror_unlocked(DAT_00126188);
    if (iVar1 != 0) {
                    /* WARNING: Subroutine does not return */
      FUN_0010e230("write failed");
    }
    iVar1 = fclose(DAT_00126188);
    if (iVar1 != 0) {
      uVar3 = dcgettext(0,"write failed",5);
                    /* WARNING: Subroutine does not return */
      FUN_0010e1f0(uVar3);
    }
    _Var2 = waitpid(DAT_00126740,(int *)&local_14,0);
    if (_Var2 < 0) {
                    /* WARNING: Subroutine does not return */
      FUN_0010e1f0("waitpid");
    }
    if ((local_14 & 0x7f) == 0) {
      bVar4 = (byte)(local_14 >> 8);
      if (bVar4 == 0) goto LAB_0010e5cd;
      pcVar6 = "subsidiary program \'%s\' could not be invoked";
      uVar5 = (uint)bVar4;
      if ((bVar4 != 0x7e) &&
         (pcVar6 = "subsidiary program \'%s\' failed (exit status %d)", uVar5 = (uint)bVar4,
         bVar4 == 0x7f)) {
        pcVar6 = "subsidiary program \'%s\' not found";
      }
    }
    else {
      pcVar6 = "subsidiary program \'%s\' failed";
      uVar5 = 0x7fffffff;
    }
    uVar3 = dcgettext(0,pcVar6,5);
                    /* WARNING: Subroutine does not return */
    error(2,0,uVar3,"/usr/bin/pr",uVar5);
  }
LAB_0010e5cd:
  DAT_00126188 = (FILE *)0x0;
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: lines_differ @ 0xe690

undefined8 lines_differ(byte *param_1,byte *param_2)

{
  byte *pbVar1;
  byte bVar2;
  ushort *puVar3;
  ulong uVar4;
  char cVar5;
  uint uVar6;
  uint uVar7;
  ushort **ppuVar8;
  __int32_t **pp_Var9;
  long lVar10;
  byte bVar11;
  bool bVar12;
  byte bVar13;
  char cVar14;
  ulong uVar15;
  byte *pbVar16;
  byte *pbVar17;
  ulong uVar18;
  byte *pbVar19;
  byte *pbVar20;
  ulong uVar21;
  
  uVar6 = DAT_00126558;
  cVar5 = DAT_00126552;
  uVar4 = DAT_00126498;
  uVar21 = 0;
  uVar7 = DAT_00126558 - 2;
  pbVar19 = param_2;
  pbVar20 = param_1;
LAB_0010e6e0:
  do {
    bVar11 = *pbVar20;
    bVar13 = *pbVar19;
    uVar15 = (ulong)bVar13;
    pbVar17 = pbVar20 + 1;
    pbVar16 = pbVar19 + 1;
    if (bVar11 == bVar13) goto LAB_0010e7ca;
    if (uVar6 != 4) {
      if (uVar6 < 5) {
        if (uVar6 == 1) {
LAB_0010e880:
          uVar18 = uVar21;
          if ((bVar11 == 0x20) && (bVar13 == 9)) {
LAB_0010e8ca:
            do {
              if (bVar11 == 0x20) {
                uVar18 = uVar18 + 1;
              }
              else {
                if (bVar11 != 9) goto LAB_0010e8fc;
                uVar18 = (uVar18 + uVar4) - uVar18 % uVar4;
              }
              bVar11 = *pbVar17;
              pbVar17 = pbVar17 + 1;
            } while( true );
          }
          if ((bVar11 == 9) && (bVar13 == 0x20)) {
            bVar11 = 9;
            goto LAB_0010e8ca;
          }
        }
        else if (uVar7 < 2) {
          ppuVar8 = __ctype_b_loc();
          puVar3 = *ppuVar8;
          if (((*(byte *)((long)puVar3 + (ulong)bVar11 * 2 + 1) & 0x20) == 0) ||
             ((*(byte *)((long)puVar3 + (ulong)bVar13 * 2 + 1) & 0x20) == 0)) {
            if (uVar6 != 2) goto LAB_0010e880;
          }
          else {
            if (bVar11 != 10) {
              bVar2 = pbVar20[1];
              pbVar20 = pbVar17;
              while (bVar2 != 10) {
                if ((*(byte *)((long)puVar3 + (ulong)bVar2 * 2 + 1) & 0x20) == 0) goto LAB_0010e798;
                pbVar1 = pbVar20 + 1;
                pbVar20 = pbVar20 + 1;
                bVar2 = *pbVar1;
              }
            }
            if ((bVar13 == 10) || (bVar13 = pbVar19[1], pbVar19 = pbVar16, bVar13 == 10)) {
              return 0;
            }
            while ((*(byte *)((long)puVar3 + (ulong)bVar13 * 2 + 1) & 0x20) != 0) {
              bVar13 = pbVar19[1];
              pbVar19 = pbVar19 + 1;
              if (bVar13 == 10) {
                return 0;
              }
            }
          }
        }
      }
      else if (uVar6 == 5) {
        ppuVar8 = __ctype_b_loc();
        puVar3 = *ppuVar8;
        while (((*(byte *)((long)puVar3 + (ulong)bVar11 * 2 + 1) & 0x20) != 0 && (bVar11 != 10))) {
          bVar11 = *pbVar17;
          pbVar17 = pbVar17 + 1;
        }
        bVar13 = *(byte *)((long)puVar3 + (ulong)bVar13 * 2 + 1);
        while (((bVar13 & 0x20) != 0 && ((char)uVar15 != '\n'))) {
          uVar15 = (ulong)*pbVar16;
          pbVar16 = pbVar16 + 1;
          bVar13 = *(byte *)((long)puVar3 + uVar15 * 2 + 1);
        }
      }
      goto LAB_0010e798;
    }
    ppuVar8 = __ctype_b_loc();
    puVar3 = *ppuVar8;
    uVar18 = (ulong)bVar11;
    pbVar20 = pbVar17;
    pbVar19 = pbVar16;
    if ((*(byte *)((long)puVar3 + uVar18 * 2 + 1) & 0x20) == 0) {
      if ((puVar3[bVar13] & 0x2000) != 0) {
LAB_0010ea1f:
        do {
          pbVar16 = pbVar19;
          bVar11 = (byte)uVar18;
          if ((char)uVar15 == '\n') goto LAB_0010e798;
          uVar15 = (ulong)*pbVar16;
          pbVar19 = pbVar16 + 1;
        } while ((*(byte *)((long)puVar3 + uVar15 * 2 + 1) & 0x20) != 0);
        uVar15 = 0x20;
        goto LAB_0010e9f5;
      }
    }
    else {
      do {
        pbVar17 = pbVar20;
        if ((char)uVar18 == '\n') goto LAB_0010e9ef;
        uVar18 = (ulong)*pbVar17;
        pbVar20 = pbVar17 + 1;
      } while ((*(byte *)((long)puVar3 + uVar18 * 2 + 1) & 0x20) != 0);
      uVar18 = 0x20;
LAB_0010e9ef:
      bVar11 = (byte)uVar18;
      if ((puVar3[bVar13] & 0x2000) != 0) goto LAB_0010ea1f;
LAB_0010e9f5:
      if ((byte)uVar15 == bVar11) goto LAB_0010e798;
    }
    cVar14 = (char)uVar15;
    bVar12 = cVar14 == ' ' && bVar11 != 10;
    if (cVar14 != ' ' || bVar11 == 10) {
      bVar12 = cVar14 != '\n';
      break;
    }
    if (pbVar17 <= param_1 + 1) {
      uVar15 = 0x20;
      break;
    }
    uVar15 = 0x20;
    pbVar19 = pbVar16;
    pbVar20 = pbVar17 + -1;
  } while ((*(byte *)((long)puVar3 + (ulong)pbVar17[-2] * 2 + 1) & 0x20) != 0);
  if ((bVar11 == 0x20) && (bVar12)) {
    if (param_2 + 1 < pbVar16) {
      pbVar19 = pbVar16 + -1;
      pbVar20 = pbVar17;
      if ((*(byte *)((long)puVar3 + (ulong)pbVar16[-2] * 2 + 1) & 0x20) != 0) goto LAB_0010e6e0;
    }
    else {
      bVar11 = 0x20;
    }
  }
LAB_0010e798:
  if (cVar5 != '\0') {
    pp_Var9 = __ctype_tolower_loc();
    bVar11 = *(byte *)(*pp_Var9 + bVar11);
    uVar15 = (ulong)*(byte *)(*pp_Var9 + uVar15);
  }
  bVar13 = (byte)uVar15;
  if (bVar11 != bVar13) {
    return 1;
  }
LAB_0010e7ca:
  if (bVar13 == 10) {
    return 0;
  }
  lVar10 = 1;
  if (bVar13 == 9) {
    lVar10 = uVar4 - uVar21 % uVar4;
  }
  uVar21 = uVar21 + lVar10;
  pbVar19 = pbVar16;
  pbVar20 = pbVar17;
  goto LAB_0010e6e0;
LAB_0010e8fc:
  if ((char)uVar15 == ' ') {
    uVar21 = uVar21 + 1;
  }
  else {
    if ((char)uVar15 != '\t') goto LAB_0010ea30;
    uVar21 = (uVar4 + uVar21) - uVar21 % uVar4;
  }
  uVar15 = (ulong)*pbVar16;
  pbVar16 = pbVar16 + 1;
  goto LAB_0010e8fc;
LAB_0010ea30:
  if (uVar18 != uVar21) {
    return 1;
  }
  goto LAB_0010e798;
}




// Function: find_change @ 0xeaf0

undefined8 find_change(undefined8 param_1)

{
  return param_1;
}




// Function: print_script @ 0xeb10

void print_script(long param_1,code *param_2,code *param_3)

{
  long lVar1;
  long *plVar2;
  
  if (param_1 == 0) {
    return;
  }
  do {
    plVar2 = (long *)(*param_2)(param_1);
    lVar1 = *plVar2;
    *plVar2 = 0;
    (*param_3)(param_1);
    *plVar2 = lVar1;
    param_1 = lVar1;
  } while (lVar1 != 0);
  return;
}




// Function: set_color_context @ 0xeb70

void set_color_context(int param_1)

{
  if (param_1 != 3) {
    FUN_0010ec70();
  }
  if ((DAT_00126758 != '\0') && (DAT_00126010 != param_1)) {
    FUN_0010df10(DAT_00126020,PTR_DAT_00126028);
    switch(param_1) {
    case 0:
      FUN_0010df10(DAT_00126060,PTR_DAT_00126068);
      break;
    case 1:
      FUN_0010df10(DAT_00126070,PTR_DAT_00126078);
      break;
    case 2:
      FUN_0010df10(DAT_00126080,PTR_DAT_00126088);
      break;
    case 3:
      FUN_0010df10(DAT_00126050,PTR_s_0_0011d860_0x10_00126058);
      break;
    case 4:
      FUN_0010df10(DAT_00126090,PTR_DAT_00126098);
      break;
    default:
                    /* WARNING: Subroutine does not return */
      abort();
    }
    FUN_0010df10(DAT_00126030,PTR_s_m_0011b38a_0xb_00126038);
    DAT_00126010 = param_1;
    return;
  }
  return;
}




// Function: process_signals @ 0xec70

void process_signals(void)

{
  int __sig;
  long in_FS_OFFSET;
  sigset_t sStack_a8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  while ((DAT_00126774 != 0 || (DAT_00126770 != 0))) {
    FUN_0010eb70(3);
    fflush_unlocked(stdout);
    sigprocmask(0,(sigset_t *)&DAT_00126780,&sStack_a8);
    __sig = DAT_00126774;
    if (DAT_00126770 == 0) {
      signal(DAT_00126774,(__sighandler_t)0x0);
    }
    else {
      DAT_00126770 = DAT_00126770 + -1;
      __sig = 0x13;
    }
    raise(__sig);
    sigprocmask(2,&sStack_a8,(sigset_t *)0x0);
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: output_1_line @ 0xed50

void output_1_line(byte *param_1,byte *param_2,long param_3,undefined8 param_4)

{
  byte bVar1;
  FILE *__stream;
  size_t sVar2;
  ushort **ppuVar3;
  long lVar4;
  ulong uVar5;
  long lVar6;
  ulong uVar7;
  
  uVar5 = DAT_00126498;
  __stream = DAT_00126188;
  if (DAT_001264a0 == '\0') {
    for (uVar5 = (long)param_2 - (long)param_1; uVar5 != 0; uVar5 = uVar5 - sVar2) {
      uVar7 = 0x400;
      if (uVar5 < 0x401) {
        uVar7 = uVar5;
      }
      sVar2 = fwrite_unlocked(param_1,1,uVar7,DAT_00126188);
      if (sVar2 < uVar7) {
        return;
      }
      param_1 = param_1 + sVar2;
      FUN_0010ec70();
    }
  }
  else {
    uVar7 = 0;
    lVar6 = 1;
    if (param_1 < param_2) {
      do {
        bVar1 = *param_1;
        param_1 = param_1 + 1;
        if (bVar1 == 9) {
          lVar4 = uVar5 - uVar7 % uVar5;
          uVar7 = uVar7 + lVar4;
          do {
            putc_unlocked(0x20,__stream);
            lVar4 = lVar4 + -1;
          } while (lVar4 != 0);
LAB_0010ee4f:
          if (param_2 == param_1) {
            return;
          }
        }
        else {
          if (bVar1 != 0xd) {
            if (bVar1 == 8) {
              if (uVar7 == 0) goto LAB_0010ef10;
              uVar7 = uVar7 - 1;
              putc_unlocked(8,__stream);
            }
            else {
              ppuVar3 = __ctype_b_loc();
              uVar7 = uVar7 + ((*ppuVar3)[bVar1] >> 0xe & 1);
              putc_unlocked((uint)bVar1,__stream);
            }
            goto LAB_0010ee4f;
          }
          putc_unlocked(0xd,__stream);
          if ((param_3 == 0) || (param_2 <= param_1)) {
LAB_0010ef10:
            uVar7 = 0;
            goto LAB_0010ee4f;
          }
          uVar7 = 0;
          if (*param_1 != 10) {
            __fprintf_chk(__stream,1,param_3,param_4);
          }
        }
        lVar6 = lVar6 + 1;
        if (lVar6 == 0x400) {
          FUN_0010ec70();
          lVar6 = 0;
        }
      } while( true );
    }
  }
  return;
}




// Function: print_1_line_nl @ 0xef20

void print_1_line_nl(char *param_1,undefined8 *param_2,char param_3)

{
  char cVar1;
  char *pcVar2;
  long lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  char *pcVar6;
  char *pcVar7;
  char *pcVar8;
  
  uVar4 = DAT_00126188;
  pcVar2 = (char *)*param_2;
  lVar3 = param_2[1];
  if (param_1 == (char *)0x0) {
    if (param_3 == '\0') {
      FUN_0010ed50(pcVar2,lVar3,0,0);
    }
    else {
      FUN_0010ed50(pcVar2,lVar3 - (ulong)(*(char *)(lVar3 + -1) == '\n'),0,0);
    }
  }
  else {
    pcVar8 = (char *)0x0;
    if (*param_1 != '\0') {
      pcVar8 = "%s\t";
      if (DAT_00126492 == '\0') {
        pcVar8 = "%s ";
      }
      pcVar6 = param_1;
      pcVar7 = pcVar8;
      if ((DAT_00126491 != '\0') && (*pcVar2 == '\n')) {
        pcVar7 = "%s";
        pcVar6 = param_1 + (*param_1 == ' ');
      }
      __fprintf_chk(DAT_00126188,1,pcVar7,pcVar6);
    }
    if (param_3 == '\0') {
      FUN_0010ed50(pcVar2,lVar3,pcVar8,param_1);
      cVar1 = *param_1;
    }
    else {
      FUN_0010ed50(pcVar2,lVar3 - (ulong)(*(char *)(lVar3 + -1) == '\n'),pcVar8,param_1);
      cVar1 = *param_1;
    }
    if (cVar1 == '\0') {
      return;
    }
  }
  if (*(char *)(lVar3 + -1) == '\n') {
    return;
  }
  FUN_0010eb70(3);
  uVar5 = dcgettext(0,"No newline at end of file",5);
  __fprintf_chk(uVar4,1,"\n\\ %s\n",uVar5);
  return;
}




// Function: print_1_line @ 0xf0a0

void print_1_line(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ef20(param_1,param_2,0);
  return;
}




// Function: translate_line_number @ 0xf0b0

long translate_line_number(long param_1,long param_2)

{
  return param_2 + *(long *)(param_1 + 0xe8) + 1;
}




// Function: translate_range @ 0xf0c0

void translate_range(undefined8 param_1,long param_2,undefined8 param_3,long *param_4,long *param_5)

{
  long lVar1;
  undefined1 auVar2 [16];
  
  auVar2 = FUN_0010f0b0(param_1,param_2 + -1);
  *param_4 = auVar2._0_8_ + 1;
  lVar1 = FUN_0010f0b0(param_1,auVar2._8_8_ + 1);
  *param_5 = lVar1 + -1;
  return;
}




// Function: print_number_range @ 0xf0f0

void print_number_range(char param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  long in_FS_OFFSET;
  long local_20;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0010f0c0(param_2,param_3,param_4,&local_20,&local_18);
  if (local_20 < local_18) {
    if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
      __fprintf_chk(DAT_00126188,1,"%ld%c%ld",local_20,(int)param_1);
      return;
    }
  }
  else if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    __fprintf_chk(DAT_00126188,1,&DAT_0011b092,local_18);
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: analyze_hunk @ 0xf1a0

byte analyze_hunk(long *param_1,long *param_2,long *param_3,long *param_4,long *param_5)

{
  long lVar1;
  long lVar2;
  byte bVar3;
  byte *pbVar4;
  bool bVar5;
  bool bVar6;
  long lVar7;
  long lVar8;
  uint uVar9;
  ushort **ppuVar10;
  byte *pbVar11;
  byte bVar12;
  long lVar13;
  long lVar14;
  long lVar15;
  long lVar16;
  byte local_69;
  int local_64;
  long local_60;
  long local_58;
  
  lVar8 = DAT_00126388;
  lVar7 = DAT_00126258;
  uVar9 = (uint)DAT_00126554;
  if (DAT_00126554 == 0) {
    local_64 = -1;
    bVar5 = false;
    uVar9 = (uint)(DAT_001264e0 != 0);
    bVar6 = false;
  }
  else if (DAT_00126558 < 2) {
    local_64 = 0;
    bVar6 = false;
    bVar5 = false;
  }
  else {
    local_64 = 0;
    bVar6 = true;
    bVar5 = 3 < DAT_00126558;
  }
  local_58 = 0;
  local_60 = 0;
  *param_2 = param_1[3];
  *param_4 = param_1[4];
  do {
    lVar13 = param_1[3];
    lVar16 = param_1[4];
    lVar1 = param_1[2] + -1 + lVar13;
    local_58 = local_58 + param_1[1];
    local_60 = local_60 + param_1[2];
    lVar2 = lVar16 + -1 + param_1[1];
    bVar12 = lVar13 <= lVar1 & (byte)uVar9;
    if (bVar12 == 0) {
      local_69 = lVar16 <= lVar2 & (byte)uVar9;
    }
    else {
      do {
        lVar16 = *(long *)(lVar7 + 8 + lVar13 * 8);
        pbVar4 = *(byte **)(lVar7 + lVar13 * 8);
        lVar16 = lVar16 + -1 + (ulong)(*(char *)(lVar16 + -1) != '\n');
        lVar15 = lVar16 - (long)pbVar4;
        lVar14 = lVar15;
        if ((bVar6) && (bVar3 = *pbVar4, bVar3 != 10)) {
          ppuVar10 = __ctype_b_loc();
          pbVar11 = pbVar4;
          do {
            if ((*(byte *)((long)*ppuVar10 + (ulong)bVar3 * 2 + 1) & 0x20) == 0) {
              lVar14 = lVar16 - (long)pbVar11;
              if (!bVar5) {
                lVar14 = lVar15;
              }
              goto LAB_0010f2e2;
            }
            bVar3 = pbVar11[1];
            pbVar11 = pbVar11 + 1;
          } while (bVar3 != 10);
          lVar14 = lVar16 - (long)pbVar11;
        }
LAB_0010f2e2:
        if (local_64 == lVar14) {
          uVar9 = (uint)bVar12;
        }
        else {
          if (DAT_001264e0 == 0) {
            uVar9 = 0;
            goto LAB_0010f2fb;
          }
          uVar9 = re_search((re_pattern_buffer *)&DAT_001264c0,(char *)pbVar4,(int)lVar15,0,
                            (int)lVar15,(re_registers *)0x0);
          uVar9 = ~uVar9 >> 0x1f;
        }
        lVar13 = lVar13 + 1;
      } while ((lVar13 <= lVar1) && ((byte)uVar9 != 0));
      lVar16 = param_1[4];
      local_69 = lVar16 <= lVar2 & (byte)uVar9;
    }
    if (local_69 != 0) {
      do {
        lVar13 = *(long *)(lVar8 + 8 + lVar16 * 8);
        pbVar4 = *(byte **)(lVar8 + lVar16 * 8);
        lVar13 = lVar13 + -1 + (ulong)(*(char *)(lVar13 + -1) != '\n');
        lVar15 = lVar13 - (long)pbVar4;
        lVar14 = lVar15;
        if ((bVar6) && (bVar12 = *pbVar4, bVar12 != 10)) {
          ppuVar10 = __ctype_b_loc();
          pbVar11 = pbVar4;
          do {
            if ((*(byte *)((long)*ppuVar10 + (ulong)bVar12 * 2 + 1) & 0x20) == 0) {
              lVar14 = lVar13 - (long)pbVar11;
              if (!bVar5) {
                lVar14 = lVar15;
              }
              goto LAB_0010f465;
            }
            bVar12 = pbVar11[1];
            pbVar11 = pbVar11 + 1;
          } while (bVar12 != 10);
          lVar14 = lVar13 - (long)pbVar11;
        }
LAB_0010f465:
        if (local_64 == lVar14) {
          uVar9 = (uint)local_69;
        }
        else {
          if (DAT_001264e0 == 0) {
            uVar9 = 0;
            break;
          }
          uVar9 = re_search((re_pattern_buffer *)&DAT_001264c0,(char *)pbVar4,(int)lVar15,0,
                            (int)lVar15,(re_registers *)0x0);
          uVar9 = ~uVar9 >> 0x1f;
        }
        lVar16 = lVar16 + 1;
        if ((lVar2 < lVar16) || ((char)uVar9 == '\0')) break;
      } while( true );
    }
LAB_0010f2fb:
    param_1 = (long *)*param_1;
    if (param_1 == (long *)0x0) {
      bVar12 = 0;
      *param_3 = lVar1;
      *param_5 = lVar2;
      if ((char)uVar9 == '\0') {
        bVar12 = (local_58 != 0) * '\x02' | local_60 != 0;
      }
      return bVar12;
    }
  } while( true );
}




// Function: concat @ 0xf560

undefined1  [16] concat(char *param_1,char *param_2,char *param_3)

{
  size_t sVar1;
  size_t sVar2;
  size_t sVar3;
  undefined1 auVar4 [16];
  
  sVar1 = strlen(param_1);
  sVar2 = strlen(param_2);
  sVar3 = strlen(param_3);
  auVar4._0_8_ = FUN_00115f50(sVar1 + sVar2 + 1 + sVar3);
  __sprintf_chk(auVar4._0_8_,1,0xffffffffffffffff,"%s%s%s",param_1,param_2,param_3);
  auVar4._8_8_ = 0x10f59a;
  return auVar4;
}




// Function: zalloc @ 0xf5e0

void zalloc(size_t param_1)

{
  void *__s;
  
  __s = (void *)FUN_00115f50();
  memset(__s,0,param_1);
  return;
}




// Function: debug_script @ 0xf600

void debug_script(undefined8 *param_1)

{
  fflush_unlocked(stdout);
  for (; param_1 != (undefined8 *)0x0; param_1 = (undefined8 *)*param_1) {
    __fprintf_chk(stderr,1,"%3ld %3ld delete %ld insert %ld\n",param_1[3],param_1[4],param_1[2],
                  param_1[1]);
  }
  fflush_unlocked(stderr);
  return;
}




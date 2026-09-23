// Function: main @ 0x2900

/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 main(int param_1,undefined8 *param_2)

{
  long *__s2;
  char cVar1;
  undefined1 uVar2;
  undefined1 uVar3;
  FILE *__stream;
  char *pcVar4;
  char cVar5;
  int iVar6;
  long lVar7;
  long *__s1;
  undefined1 *puVar8;
  undefined1 *puVar9;
  undefined8 uVar10;
  undefined1 *puVar11;
  long lVar12;
  undefined1 *puVar13;
  char *pcVar14;
  long lVar15;
  long lVar16;
  long lVar17;
  int *piVar18;
  undefined8 uVar19;
  undefined8 uVar20;
  undefined8 uVar21;
  undefined **ppuVar22;
  ulong uVar23;
  long *plVar24;
  undefined1 *extraout_RDX;
  uint uVar25;
  undefined1 *puVar26;
  undefined1 *puVar27;
  long *plVar28;
  undefined1 *puVar29;
  long in_FS_OFFSET;
  bool bVar30;
  byte bVar31;
  undefined1 *local_210;
  undefined1 *local_208;
  int local_200;
  int local_1ec;
  undefined1 *local_1e8;
  bool local_1bd;
  int local_1bc;
  undefined1 *local_1b0;
  stat local_1a8;
  stat local_118;
  undefined1 local_82 [5];
  undefined1 local_7d [5];
  undefined1 local_78 [32];
  undefined1 local_58 [24];
  long local_40;
  
  bVar31 = 0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_0010d020 = 2;
  FUN_00104080(*param_2);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  FUN_00103e90(0);
  FUN_00106870();
LAB_001029a1:
  while( true ) {
    ppuVar22 = &PTR_s_print_bytes_0010ca60;
    iVar6 = getopt_long(param_1,param_2,"bci:ln:sv",&PTR_s_print_bytes_0010ca60,0);
    uVar10 = DAT_0011d280;
    if (iVar6 == -1) break;
    switch(iVar6) {
    case 0x62:
    case 99:
      DAT_0010d0b2 = '\x01';
      break;
    default:
      lVar7 = 0;
      optind = FUN_00103a80(0);
      goto LAB_001030fd;
    case 0x69:
      FUN_00103ae0(0,&optarg,0x3a);
      pcVar14 = optarg + 1;
      if (*optarg == ':') {
        optarg = pcVar14;
        FUN_00103ae0(1,&optarg,0);
      }
      else {
        optarg = pcVar14;
        if (DAT_0010d0c8 < DAT_0010d0c0) {
          DAT_0010d0c8 = DAT_0010d0c0;
        }
      }
      break;
    case 0x6c:
      FUN_00103c60(1);
      break;
    case 0x6e:
      iVar6 = FUN_00106940(optarg,0,0,&local_1b0);
      if ((iVar6 != 0) || ((long)local_1b0 < 0)) {
        FUN_00103a80("invalid --bytes value \'%s\'",optarg);
        goto LAB_001037c1;
      }
      if (((long)DAT_0010d010 < 0) || ((long)local_1b0 <= (long)DAT_0010d010)) {
        DAT_0010d010 = local_1b0;
      }
      break;
    case 0x73:
      goto switchD_001029d0_caseD_73;
    case 0x76:
      uVar10 = FUN_00104520("David MacKenzie");
      uVar19 = FUN_001045b0("Torbjorn Granlund",&DAT_0010962a);
      FUN_001061b0(stdout,&DAT_0010965d,"GNU diffutils",PTR_DAT_0010d018,uVar19,uVar10,0,ppuVar22);
      FUN_00103c90();
      goto LAB_00102bcf;
    case 0x80:
      uVar19 = dcgettext(0,"Usage: %s [OPTION]... FILE1 [FILE2 [SKIP1 [SKIP2]]]\n",5);
      ppuVar22 = &PTR_s__b____print_bytes_print_differin_0010ca00;
      __printf_chk(1,uVar19,uVar10);
      uVar10 = dcgettext(0,"Compare two files byte by byte.",5);
      __printf_chk(1,&DAT_00109675,uVar10);
      uVar10 = dcgettext(0,
                         "The optional SKIP1 and SKIP2 specify the number of bytes to skip\nat the beginning of each file (zero by default)."
                         ,5);
      __printf_chk(1,&DAT_00109661,uVar10);
      __stream = stdout;
      pcVar14 = (char *)dcgettext(0,
                                  "Mandatory arguments to long options are mandatory for short options too.\n"
                                  ,5);
      fputs_unlocked(pcVar14,__stream);
      pcVar14 = "-b, --print-bytes          print differing bytes";
      do {
        ppuVar22 = ppuVar22 + 1;
        uVar10 = dcgettext(0,pcVar14,5);
        __printf_chk(1,"  %s\n",uVar10);
        pcVar14 = *ppuVar22;
      } while (pcVar14 != (char *)0x0);
      uVar10 = dcgettext(0,"Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.",
                         5);
      uVar19 = dcgettext(0,"If a FILE is \'-\' or missing, read standard input.",5);
      uVar20 = dcgettext(0,
                         "SKIP values may be followed by the following multiplicative suffixes:\nkB 1000, K 1024, MB 1,000,000, M 1,048,576,\nGB 1,000,000,000, G 1,073,741,824, and so on for T, P, E, Z, Y."
                         ,5);
      __printf_chk(1,&DAT_0010966d,uVar20,uVar19,uVar10);
      FUN_00106290();
      FUN_00103c90();
      goto LAB_00102bcf;
    }
  }
  if (optind != param_1) {
    lVar7 = (long)optind * 8;
    DAT_0010d230 = (char *)param_2[optind];
    DAT_0010d238 = "-";
    iVar6 = optind + 1;
    if (optind + 1 < param_1) {
LAB_001030fd:
      DAT_0010d238 = *(char **)((long)param_2 + lVar7 + 8);
      iVar6 = optind + 2;
    }
    optind = iVar6;
    lVar7 = (long)optind;
    if (optind < param_1) {
      optind = optind + 1;
      local_1b0 = (undefined1 *)param_2[lVar7];
      FUN_00103ae0(0,&local_1b0,0);
      lVar7 = (long)optind;
      if (optind < param_1) {
        optind = optind + 1;
        local_1b0 = (undefined1 *)param_2[lVar7];
        FUN_00103ae0(1,&local_1b0,0);
        if (optind < param_1) {
          FUN_00103a80("extra operand \'%s\'",param_2[optind]);
LAB_00103944:
          piVar18 = __errno_location();
                    /* WARNING: Subroutine does not return */
          error(2,*piVar18,&DAT_001095d4,DAT_0010d230);
        }
      }
    }
    lVar7 = 0;
    do {
      if (((lVar7 != 0) && (DAT_0010d0c0 == DAT_0010d0c8)) &&
         (iVar6 = strcmp(DAT_0010d230,DAT_0010d238), iVar6 == 0)) goto LAB_00102bcf;
      pcVar14 = (&DAT_0010d230)[lVar7];
      iVar6 = strcmp(pcVar14,"-");
      if (iVar6 == 0) {
        (&DAT_0010d220)[lVar7] = 0;
        iVar6 = 0;
      }
      else {
        iVar6 = open(pcVar14,0,0);
        (&DAT_0010d220)[lVar7] = iVar6;
        if (iVar6 < 0) {
          if (DAT_0010d0b4 == 3) {
                    /* WARNING: Subroutine does not return */
            exit(2);
          }
          goto LAB_001036b5;
        }
      }
      iVar6 = fstat(iVar6,(stat *)(&DAT_0010d100 + lVar7 * 0x12));
      if (iVar6 != 0) {
LAB_001036b5:
        piVar18 = __errno_location();
                    /* WARNING: Subroutine does not return */
        error(2,*piVar18,&DAT_001095d4,(&DAT_0010d230)[(int)lVar7]);
      }
      if (lVar7 == 1) goto code_r0x00102da0;
      lVar7 = 1;
    } while( true );
  }
  FUN_00103a80("missing operand after \'%s\'",param_2[(long)param_1 + -1]);
  goto LAB_001037d7;
switchD_001029d0_caseD_73:
  FUN_00103c60(3);
  goto LAB_001029a1;
code_r0x00102da0:
  if ((DAT_0010d108 == DAT_0010d198) && (DAT_0010d100 == DAT_0010d190)) {
LAB_00103120:
    if (((DAT_0010d118 == DAT_0010d1a8) && (DAT_0010d110 == DAT_0010d1a0)) &&
       ((DAT_0010d11c == DAT_0010d1ac &&
        ((((DAT_0010d120 == DAT_0010d1b0 && (DAT_0010d130 == DAT_0010d1c0)) &&
          (DAT_0010d158 == DAT_0010d1e8)) && (DAT_0010d168 == DAT_0010d1f8)))))) {
      lVar7 = FUN_00103b80(0);
      lVar12 = FUN_00103b80(1);
      if (lVar7 == lVar12) {
LAB_00102bcf:
        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
          return 0;
        }
LAB_001037c1:
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
    }
LAB_00102ddb:
    if (DAT_0010d0b4 != 3) goto LAB_00103247;
  }
  else {
    if ((DAT_0010d118 & 0xf000) != 0x6000) {
      if ((DAT_0010d118 & 0xf000) == 0x2000) {
        if ((DAT_0010d1a8 & 0xf000) == 0x2000) goto LAB_00103531;
        goto LAB_0010323a;
      }
      goto LAB_00102ddb;
    }
    if ((DAT_0010d1a8 & 0xf000) == 0x6000) {
LAB_00103531:
      if (DAT_0010d128 == DAT_0010d1b8) goto LAB_00103120;
      goto LAB_00102ddb;
    }
LAB_0010323a:
    if (DAT_0010d0b4 == 3) goto LAB_00102e10;
LAB_00103247:
    iVar6 = fstat(1,&local_1a8);
    if ((iVar6 != 0) || (iVar6 = stat("/dev/null",&local_118), iVar6 != 0)) goto LAB_0010325e;
    if ((local_1a8.st_ino == local_118.st_ino) && (local_1a8.st_dev == local_118.st_dev)) {
LAB_0010365f:
      DAT_0010d0b4 = 2;
    }
    else if ((local_1a8.st_mode & 0xf000) == 0x6000) {
      if ((local_118.st_mode & 0xf000) == 0x6000) goto LAB_00103649;
    }
    else if (((local_1a8.st_mode & 0xf000) == 0x2000) && ((local_118.st_mode & 0xf000) == 0x2000)) {
LAB_00103649:
      if (local_1a8.st_rdev == local_118.st_rdev) goto LAB_0010365f;
    }
LAB_0010325e:
    if (DAT_0010d0b4 != 3) goto LAB_00102e10;
  }
  lVar7 = DAT_0010d130;
  if (((DAT_0010d118 & 0xf000) == 0x8000) && ((DAT_0010d1a8 & 0xf000) == 0x8000)) {
    lVar15 = FUN_00103b80(0);
    lVar12 = DAT_0010d1c0;
    lVar16 = FUN_00103b80(1);
    lVar17 = 0;
    if (-1 < lVar7 - lVar15) {
      lVar17 = lVar7 - lVar15;
    }
    lVar12 = lVar12 - lVar16;
    if (lVar12 < 0) {
      lVar12 = 0;
    }
    if (lVar12 != lVar17) {
      if (-1 < (long)DAT_0010d010) {
        if (lVar17 < lVar12) {
          lVar12 = lVar17;
        }
        if ((long)DAT_0010d010 <= lVar12) goto LAB_00102e10;
      }
                    /* WARNING: Subroutine does not return */
      exit(1);
    }
  }
LAB_00102e10:
  DAT_0010d0d0 = (undefined1 *)FUN_00107070(DAT_0010d138,DAT_0010d1c8,0x7ffffffffffffff7);
  puVar9 = DAT_0010d0d0 + 0xf;
  __s1 = (long *)FUN_001063b0(((ulong)puVar9 >> 3) << 4);
  lVar7 = DAT_0010d130;
  __s2 = (long *)((long)__s1 + ((ulong)puVar9 & 0xfffffffffffffff8));
  local_208 = DAT_0010d010;
  _DAT_0010d0e0 = __s1;
  _DAT_0010d0e8 = __s2;
  if (DAT_0010d0b4 == 1) {
    puVar9 = (undefined1 *)0x7fffffffffffffff;
    if (-1 < (long)DAT_0010d010) {
      puVar9 = DAT_0010d010;
    }
    if ((DAT_0010d118 & 0xf000) == 0x8000) {
      lVar12 = FUN_00103b80(0);
      if (lVar7 - lVar12 < (long)puVar9) {
        puVar9 = (undefined1 *)(lVar7 - lVar12);
      }
    }
    lVar7 = DAT_0010d1c0;
    if ((DAT_0010d1a8 & 0xf000) == 0x8000) {
      lVar12 = FUN_00103b80(1);
      if (lVar7 - lVar12 < (long)puVar9) {
        puVar9 = (undefined1 *)(lVar7 - lVar12);
      }
    }
    local_1ec = 1;
    while (puVar9 = (undefined1 *)((long)puVar9 / 10), puVar9 != (undefined1 *)0x0) {
      local_1ec = local_1ec + 1;
    }
  }
  lVar7 = 0;
  do {
    puVar9 = (undefined1 *)(&DAT_0010d0c0)[lVar7];
    if ((puVar9 != (undefined1 *)0x0) && (lVar12 = FUN_00103b80(lVar7), lVar12 == -1)) {
      do {
        puVar8 = DAT_0010d0d0;
        if (puVar9 <= DAT_0010d0d0) {
          puVar8 = puVar9;
        }
        puVar13 = (undefined1 *)FUN_00106fc0((&DAT_0010d220)[lVar7],__s1,puVar8);
        if (puVar8 != puVar13) {
          if (puVar13 == (undefined1 *)0xffffffffffffffff) {
            piVar18 = __errno_location();
                    /* WARNING: Subroutine does not return */
            error(2,*piVar18,&DAT_001095d4,(&DAT_0010d230)[(int)lVar7]);
          }
          break;
        }
        puVar9 = puVar9 + -(long)puVar8;
      } while (puVar9 != (undefined1 *)0x0);
    }
    if (lVar7 == 1) break;
    lVar7 = 1;
  } while( true );
  local_200 = 0;
  local_210 = (undefined1 *)0x1;
  local_1e8 = (undefined1 *)0x1;
  local_1bd = true;
LAB_00102ebc:
  puVar9 = DAT_0010d0d0;
  if (-1 < (long)local_208) {
    if (local_208 < DAT_0010d0d0) {
      puVar9 = local_208;
    }
    local_208 = local_208 + -(long)puVar9;
  }
  puVar8 = (undefined1 *)FUN_00106fc0(DAT_0010d220,__s1,puVar9);
  if (puVar8 == (undefined1 *)0xffffffffffffffff) goto LAB_00103944;
  puVar9 = (undefined1 *)FUN_00106fc0(DAT_0010d224,__s2,puVar9);
  if (puVar9 == (undefined1 *)0xffffffffffffffff) {
    piVar18 = __errno_location();
                    /* WARNING: Subroutine does not return */
    error(2,*piVar18,&DAT_001095d4,DAT_0010d238);
  }
  puVar13 = puVar9;
  if (puVar8 <= puVar9) {
    puVar13 = puVar8;
  }
  iVar6 = memcmp(__s1,__s2,(size_t)puVar13);
  uVar25 = DAT_0010d0b4;
  puVar26 = (undefined1 *)(ulong)DAT_0010d0b4;
  bVar30 = DAT_0010d0b4 == 0;
  if (iVar6 == 0) {
    puVar27 = local_210 + (long)puVar13;
    if ((puVar13 == (undefined1 *)0x0) || (puVar29 = puVar13, !bVar30)) goto LAB_0010328b;
LAB_001032fc:
    local_210 = puVar27;
    puVar27 = (undefined1 *)((long)__s1 + (long)puVar29);
    uVar2 = *puVar27;
    puVar26 = (undefined1 *)0x0;
    *puVar27 = 10;
    plVar24 = __s1;
    while (puVar11 = rawmemchr(plVar24,10), puVar27 != puVar11) {
      puVar26 = puVar26 + 1;
      plVar24 = (long *)(puVar11 + 1);
    }
    local_1e8 = local_1e8 + (long)puVar26;
    *puVar27 = uVar2;
    puVar27 = local_210;
    if (puVar29 < puVar13) {
LAB_00103496:
      local_210 = puVar27;
      uVar10 = FUN_00103fe0(local_210,local_78);
      uVar19 = FUN_00103fe0(local_1e8,local_58);
      if (DAT_0010d0b2 == '\0') {
        pcVar14 = (char *)dcgettext(0,"%s %s differ: byte %s, line %s\n",5);
        if ((pcVar14 == "%s %s differ: byte %s, line %s\n") &&
           (cVar5 = FUN_00103f50(5), cVar5 == '\0')) {
          pcVar14 = "%s %s differ: char %s, line %s\n";
        }
        __printf_chk(1,pcVar14,DAT_0010d230,DAT_0010d238,uVar10,uVar19);
      }
      else {
        uVar2 = *(undefined1 *)((long)__s1 + (long)puVar29);
        uVar3 = *(undefined1 *)((long)__s2 + (long)puVar29);
        FUN_00103bd0(local_82,uVar2);
        FUN_00103bd0(local_7d,uVar3);
        pcVar4 = DAT_0010d238;
        pcVar14 = DAT_0010d230;
        uVar20 = dcgettext(0,"%s %s differ: byte %s, line %s is %3o %s %3o %s\n",5);
        __printf_chk(1,uVar20,pcVar14,pcVar4,uVar10,uVar19,uVar2,local_82,uVar3,local_7d);
      }
      goto LAB_00103380;
    }
    local_1bd = *(char *)((long)__s1 + (long)(puVar29 + -1)) == '\n';
LAB_0010328b:
    local_210 = puVar27;
    if (puVar8 == puVar9) {
      if (puVar8 != DAT_0010d0d0) goto LAB_001032a1;
      goto LAB_00102ebc;
    }
    if (DAT_0010d0b4 != 3) {
      uVar10 = (&DAT_0010d230)[puVar9 < puVar8];
      if (local_210 != (undefined1 *)0x1) {
        uVar19 = FUN_00103fe0(local_210 + -1,local_78,puVar26);
        if (DAT_0010d0b4 == 0) {
          uVar20 = FUN_00103fe0((long)local_1e8 - (ulong)local_1bd,local_58);
          if (local_1bd == false) {
            uVar21 = dcgettext(0,"cmp: EOF on %s after byte %s, in line %s\n",5);
          }
          else {
            uVar21 = dcgettext(0,"cmp: EOF on %s after byte %s, line %s\n",5);
          }
          __fprintf_chk(stderr,1,uVar21,uVar10,uVar19,uVar20);
        }
        else {
          uVar20 = dcgettext(0,"cmp: EOF on %s after byte %s\n",5);
          __fprintf_chk(stderr,1,uVar20,uVar10,uVar19);
        }
        goto LAB_00103380;
      }
      uVar25 = 1;
      uVar19 = dcgettext(0,"cmp: EOF on %s which is empty\n",5);
      __fprintf_chk(stderr,1,uVar19,uVar10);
      goto LAB_001032ac;
    }
  }
  else {
    if ((puVar8 < puVar9) || (*(byte *)((long)__s2 + (long)puVar8) = 0x55, puVar8 == puVar9)) {
      *(undefined1 *)((long)__s1 + (long)puVar9) = 0x79;
    }
    *(byte *)((long)__s1 + (long)puVar8) = ~*(byte *)((long)__s2 + (long)puVar8);
    *(byte *)((long)__s2 + (long)puVar9) = ~*(byte *)((long)__s1 + (long)puVar9);
    puVar27 = (undefined1 *)((long)__s1 + (long)(puVar8 + 1));
    for (uVar23 = (ulong)(~(uint)puVar8 & 7); uVar23 != 0; uVar23 = uVar23 - 1) {
      *puVar27 = 0;
      puVar27 = puVar27 + (ulong)bVar31 * -2 + 1;
    }
    puVar27 = (undefined1 *)((long)__s2 + (long)(puVar9 + 1));
    for (uVar23 = (ulong)(~(uint)puVar9 & 7); plVar24 = __s1, plVar28 = __s2, uVar23 != 0;
        uVar23 = uVar23 - 1) {
      *puVar27 = 0;
      puVar27 = puVar27 + (ulong)bVar31 * -2 + 1;
    }
    for (; *plVar24 == *plVar28; plVar24 = plVar24 + 1) {
      plVar28 = plVar28 + 1;
    }
    lVar7 = 0;
    do {
      lVar12 = lVar7;
      lVar7 = lVar12 + 1;
    } while (*(char *)((long)plVar24 + lVar12) == *(char *)((long)plVar28 + lVar12));
    puVar29 = (undefined1 *)((long)plVar24 + (lVar12 - (long)__s1));
    puVar27 = local_210 + (long)puVar29;
    if ((puVar29 != (undefined1 *)0x0) && (bVar30)) goto LAB_001032fc;
    if (puVar13 <= puVar29) goto LAB_0010328b;
    if (uVar25 != 2) {
      if (2 < uVar25) {
        if (uVar25 != 3) goto LAB_0010328b;
        goto LAB_00103380;
      }
      if (uVar25 == 0) goto LAB_00103496;
      do {
        cVar5 = *(char *)((long)__s1 + (long)puVar29);
        cVar1 = *(char *)((long)__s2 + (long)puVar29);
        if (cVar5 != cVar1) {
          uVar10 = FUN_00103fe0(puVar29 + (long)local_210,local_58,puVar26);
          if (DAT_0010d0b2 == '\0') {
            __printf_chk(1,"%*s %3o %3o\n",local_1ec,uVar10,cVar5,cVar1);
            puVar26 = extraout_RDX;
          }
          else {
            FUN_00103bd0(local_7d,cVar5);
            puVar26 = local_78;
            FUN_00103bd0(puVar26,cVar1);
            __printf_chk(1,"%*s %3o %-4s %3o %s\n",local_1ec,uVar10,cVar5,local_7d,cVar1);
          }
        }
        puVar29 = puVar29 + 1;
      } while (puVar13 != puVar29);
      local_200 = -1;
      puVar27 = puVar13 + (long)local_210;
      goto LAB_0010328b;
    }
    if (puVar8 == puVar9) {
      local_200 = 1;
      goto LAB_001032a1;
    }
  }
LAB_00103380:
  uVar25 = 1;
  goto LAB_001032ac;
LAB_001032a1:
  uVar25 = (uint)(local_200 != 0);
LAB_001032ac:
  iVar6 = close(DAT_0010d220);
  local_1bc = 0;
  if (iVar6 != 0) goto LAB_001037df;
  iVar6 = close(DAT_0010d224);
  if (iVar6 == 0) {
    if ((uVar25 != 0) && (DAT_0010d0b4 < 2)) {
      FUN_00103c90();
    }
                    /* WARNING: Subroutine does not return */
    exit(uVar25);
  }
LAB_001037d7:
  local_1bc = 1;
LAB_001037df:
  piVar18 = __errno_location();
                    /* WARNING: Subroutine does not return */
  error(2,*piVar18,&DAT_001095d4,(&DAT_0010d230)[local_1bc]);
}




// Function: try_help @ 0x3a80

void try_help(long param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  uVar1 = DAT_0011d280;
  if (param_1 != 0) {
    uVar1 = dcgettext(0,param_1,5);
                    /* WARNING: Subroutine does not return */
    error(0,0,uVar1,param_2);
  }
  uVar2 = dcgettext(0,"Try \'%s --help\' for more information.",5);
                    /* WARNING: Subroutine does not return */
  error(2,0,uVar2,uVar1);
}




// Function: specify_ignore_initial @ 0x3ae0

void specify_ignore_initial(int param_1,undefined8 *param_2,char param_3)

{
  undefined8 uVar1;
  int iVar2;
  long in_FS_OFFSET;
  long local_38;
  long local_30;
  
  uVar1 = *param_2;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = FUN_00106940(uVar1,param_2,0,&local_38,"kKMGTPEZY0");
  if (((iVar2 == 0) || ((iVar2 == 2 && (*(char *)*param_2 == param_3)))) && (-1 < local_38)) {
    if ((long)(&DAT_0010d0c0)[param_1] < local_38) {
      (&DAT_0010d0c0)[param_1] = local_38;
    }
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
  }
  else {
    FUN_00103a80("invalid --ignore-initial value \'%s\'",uVar1);
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: file_position @ 0x3b80

__off_t file_position(int param_1)

{
  __off_t _Var1;
  long lVar2;
  
  lVar2 = (long)param_1;
  if ((&DAT_0010d0b0)[lVar2] != '\0') {
    return *(__off_t *)(&DAT_0010d0a0 + lVar2 * 8);
  }
  (&DAT_0010d0b0)[lVar2] = 1;
  _Var1 = lseek((&DAT_0010d220)[lVar2],(&DAT_0010d0c0)[lVar2],1);
  *(__off_t *)(&DAT_0010d0a0 + lVar2 * 8) = _Var1;
  return _Var1;
}




// Function: sprintc @ 0x3bd0

void sprintc(byte *param_1,byte param_2)

{
  ushort **ppuVar1;
  
  ppuVar1 = __ctype_b_loc();
  if ((*(byte *)((long)*ppuVar1 + (ulong)param_2 * 2 + 1) & 0x40) == 0) {
    if ((char)param_2 < '\0') {
      param_2 = param_2 + 0x80;
      param_1[0] = 0x4d;
      param_1[1] = 0x2d;
      param_1 = param_1 + 2;
    }
    if (param_2 < 0x20) {
      *param_1 = 0x5e;
      param_2 = param_2 + 0x40;
      param_1 = param_1 + 1;
    }
    else if (param_2 == 0x7f) {
      *param_1 = 0x5e;
      param_2 = 0x3f;
      param_1 = param_1 + 1;
    }
  }
  *param_1 = param_2;
  param_1[1] = 0;
  return;
}




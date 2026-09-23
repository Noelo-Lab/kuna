// Function: try_help @ 0x29e9

void try_help(long param_1,undefined8 param_2)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  char extraout_DL;
  undefined8 *puVar4;
  int iVar5;
  long in_FS_OFFSET;
  long lStack_48;
  undefined8 uStack_40;
  long lStack_38;
  code *pcStack_30;
  undefined8 local_28;
  long local_20;
  
  local_28 = param_2;
  local_20 = param_1;
  if (param_1 != 0) {
    pcStack_30 = (code *)0x102a11;
    uVar2 = gettext(param_1);
    pcStack_30 = (code *)0x102a2f;
    error(0,0,uVar2,local_28);
  }
  uVar2 = DAT_00120268;
  pcStack_30 = (code *)0x102a45;
  uVar3 = gettext("Try \'%s --help\' for more information.");
  puVar4 = (undefined8 *)0x0;
  iVar5 = 2;
  pcStack_30 = FUN_00102a5f;
  error(2,0,uVar3,uVar2);
  lStack_38 = *(long *)(in_FS_OFFSET + 0x28);
  uStack_40 = *puVar4;
  pcStack_30 = (code *)&stack0xfffffffffffffff8;
  iVar1 = FUN_00107e2e(uStack_40,puVar4,0,&lStack_48,"kKMGTPEZY0");
  if (((iVar1 != 0) && ((iVar1 != 2 || (extraout_DL != *(char *)*puVar4)))) || (lStack_48 < 0)) {
    try_help("invalid --ignore-initial value \'%s\'",uStack_40);
  }
  if ((long)(&DAT_00110200)[iVar5] < lStack_48) {
    (&DAT_00110200)[iVar5] = lStack_48;
  }
  if (lStack_38 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: specify_ignore_initial @ 0x2a5f

void specify_ignore_initial(int param_1,undefined8 *param_2,char param_3)

{
  int iVar1;
  long in_FS_OFFSET;
  long local_20;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_18 = *param_2;
  iVar1 = FUN_00107e2e(local_18,param_2,0,&local_20,"kKMGTPEZY0");
  if (((iVar1 != 0) && ((iVar1 != 2 || (param_3 != *(char *)*param_2)))) || (local_20 < 0)) {
    FUN_001029e9("invalid --ignore-initial value \'%s\'",local_18);
  }
  if ((long)(&DAT_00110200)[param_1] < local_20) {
    (&DAT_00110200)[param_1] = local_20;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: specify_comparison_type @ 0x2b46

void specify_comparison_type(int param_1)

{
  if ((DAT_00110210 != 0) && (param_1 != DAT_00110210)) {
    FUN_001029e9("options -l and -s are incompatible",0);
  }
  DAT_00110210 = param_1;
  return;
}




// Function: check_stdout @ 0x2b8a

void check_stdout(void)

{
  int iVar1;
  undefined8 uVar2;
  int *piVar3;
  
  iVar1 = ferror_unlocked(stdout);
  if (iVar1 != 0) {
    uVar2 = gettext("write failed");
    error(2,0,&DAT_0010c100,uVar2);
  }
  iVar1 = fclose(stdout);
  if (iVar1 != 0) {
    uVar2 = gettext("standard output");
    piVar3 = __errno_location();
    error(2,*piVar3,&DAT_0010c100,uVar2);
  }
  return;
}




// Function: usage @ 0x2c28

void usage(void)

{
  FILE *__stream;
  char *pcVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined **local_20;
  
  uVar2 = DAT_00120268;
  pcVar1 = (char *)gettext("Usage: %s [OPTION]... FILE1 [FILE2 [SKIP1 [SKIP2]]]\n");
  printf(pcVar1,uVar2);
  uVar2 = gettext("Compare two files byte by byte.");
  printf("%s\n",uVar2);
  uVar2 = gettext(
                 "The optional SKIP1 and SKIP2 specify the number of bytes to skip\nat the beginning of each file (zero by default)."
                 );
  printf("\n%s\n\n",uVar2);
  __stream = stdout;
  pcVar1 = (char *)gettext(
                          "Mandatory arguments to long options are mandatory for short options too.\n"
                          );
  fputs_unlocked(pcVar1,__stream);
  for (local_20 = &PTR_s__b____print_bytes_print_differin_0010fb40; *local_20 != (undefined *)0x0;
      local_20 = local_20 + 1) {
    uVar2 = gettext(*local_20);
    printf("  %s\n",uVar2);
  }
  uVar2 = gettext("Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.");
  uVar3 = gettext("If a FILE is \'-\' or missing, read standard input.");
  uVar4 = gettext(
                 "SKIP values may be followed by the following multiplicative suffixes:\nkB 1000, K 1024, MB 1,000,000, M 1,048,576,\nGB 1,000,000,000, G 1,073,741,824, and so on for T, P, E, Z, Y."
                 );
  printf("\n%s\n\n%s\n%s\n",uVar4,uVar3,uVar2);
  FUN_001074c0();
  return;
}




// Function: main @ 0x2d6d

undefined8 main(int param_1,undefined8 *param_2)

{
  char *pcVar1;
  bool bVar2;
  int iVar3;
  int iVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long lVar7;
  long lVar8;
  int *piVar9;
  long in_FS_OFFSET;
  int local_17c;
  int local_178;
  int local_174;
  long local_168;
  long local_160;
  long local_158;
  ulong local_150;
  stat local_148;
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_00110020 = 2;
  FUN_00104a4b(*param_2);
  setlocale(6,"");
  bindtextdomain("diffutils","/usr/local/share/locale");
  textdomain("diffutils");
  FUN_0010479f(0);
  FUN_00107c09();
  while (iVar3 = getopt_long(param_1,param_2,"bci:ln:sv",&PTR_s_print_bytes_0010fa00,0), iVar3 != -1
        ) {
    switch(iVar3) {
    case 0x62:
    case 99:
      DAT_00110214 = 1;
      break;
    default:
      FUN_001029e9(0,0);
      break;
    case 0x69:
      FUN_00102a5f(0,&optarg,0x3a);
      pcVar1 = optarg + 1;
      if (*optarg == ':') {
        optarg = pcVar1;
        FUN_00102a5f(1,&optarg,0);
      }
      else {
        optarg = pcVar1;
        if (DAT_00110208 < DAT_00110200) {
          DAT_00110208 = DAT_00110200;
        }
      }
      break;
    case 0x6c:
      FUN_00102b46(1);
      break;
    case 0x6e:
      iVar3 = FUN_00107e2e(optarg,0,0,&local_168,"kKMGTPEZY0");
      if ((iVar3 != 0) || (local_168 < 0)) {
        FUN_001029e9("invalid --bytes value \'%s\'",optarg);
      }
      if ((DAT_00110010 < 0) || (local_168 <= DAT_00110010)) {
        DAT_00110010 = local_168;
      }
      break;
    case 0x73:
      FUN_00102b46(3);
      break;
    case 0x76:
      uVar5 = FUN_0010500c("David MacKenzie");
      uVar6 = FUN_001050b2("Torbjorn Granlund",&DAT_0010c60c);
      FUN_001073dc(stdout,&DAT_0010c63f,"GNU diffutils",PTR_DAT_00110018,uVar6,uVar5,0);
      FUN_00102b8a();
      goto LAB_001037b8;
    case 0x80:
      FUN_00102c28();
      FUN_00102b8a();
      goto LAB_001037b8;
    }
  }
  if (param_1 == optind) {
    FUN_001029e9("missing operand after \'%s\'",param_2[(long)param_1 + -1]);
  }
  iVar3 = optind + 1;
  DAT_001100a0 = (char *)param_2[optind];
  if (iVar3 < param_1) {
    DAT_001100a8 = (char *)param_2[iVar3];
    optind = optind + 2;
  }
  else {
    DAT_001100a8 = "-";
    optind = iVar3;
  }
  local_17c = 0;
  while ((local_17c < 2 && (optind < param_1))) {
    local_168 = param_2[optind];
    optind = optind + 1;
    FUN_00102a5f(local_17c,&local_168,0);
    local_17c = local_17c + 1;
  }
  if (optind < param_1) {
    FUN_001029e9("extra operand \'%s\'",param_2[optind]);
  }
  for (local_178 = 0; local_178 < 2; local_178 = local_178 + 1) {
    if (((local_178 != 0) && (DAT_00110200 == DAT_00110208)) &&
       (iVar3 = strcmp(DAT_001100a0,DAT_001100a8), iVar3 == 0)) goto LAB_001037b8;
    iVar3 = strcmp((&DAT_001100a0)[local_178],"-");
    if (iVar3 == 0) {
      (&DAT_001100b0)[local_178] = 0;
    }
    else {
      iVar3 = open((&DAT_001100a0)[local_178],0,0);
      (&DAT_001100b0)[local_178] = iVar3;
    }
    if (((int)(&DAT_001100b0)[local_178] < 0) ||
       (iVar3 = fstat((&DAT_001100b0)[local_178],(stat *)(&DAT_001100c0 + (long)local_178 * 0x12)),
       iVar3 != 0)) {
      if (((int)(&DAT_001100b0)[local_178] < 0) && (DAT_00110210 == 3)) {
                    /* WARNING: Subroutine does not return */
        exit(2);
      }
      uVar5 = (&DAT_001100a0)[local_178];
      piVar9 = __errno_location();
      error(2,*piVar9,&DAT_0010c100,uVar5);
    }
  }
  if (((DAT_001100c8 == DAT_00110158) && (DAT_001100c0 == DAT_00110150)) ||
     (((((DAT_001100d8 & 0xf000) == 0x6000 && ((DAT_00110168 & 0xf000) == 0x6000)) ||
       (((DAT_001100d8 & 0xf000) == 0x2000 && ((DAT_00110168 & 0xf000) == 0x2000)))) &&
      (DAT_001100e8 == DAT_00110178)))) {
    bVar2 = true;
  }
  else {
    bVar2 = false;
  }
  if ((((((bVar2) && (DAT_001100d8 == DAT_00110168)) && (DAT_001100d0 == DAT_00110160)) &&
       ((DAT_001100dc == DAT_0011016c && (DAT_001100e0 == DAT_00110170)))) &&
      (DAT_001100f0 == DAT_00110180)) &&
     ((DAT_00110118 == DAT_001101a8 && (DAT_00110128 == DAT_001101b8)))) {
    lVar7 = FUN_0010447f(0);
    lVar8 = FUN_0010447f(1);
    if (lVar7 == lVar8) {
LAB_001037b8:
      if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return 0;
    }
  }
  if (((DAT_00110210 != 3) && (iVar3 = fstat(1,&local_148), iVar3 == 0)) &&
     (iVar3 = stat("/dev/null",&local_b8), iVar3 == 0)) {
    if (((local_148.st_ino == local_b8.st_ino) && (local_148.st_dev == local_b8.st_dev)) ||
       (((((local_148.st_mode & 0xf000) == 0x6000 && ((local_b8.st_mode & 0xf000) == 0x6000)) ||
         (((local_148.st_mode & 0xf000) == 0x2000 && ((local_b8.st_mode & 0xf000) == 0x2000)))) &&
        (local_148.st_rdev == local_b8.st_rdev)))) {
      bVar2 = true;
    }
    else {
      bVar2 = false;
    }
    if (bVar2) {
      DAT_00110210 = 2;
    }
  }
  lVar7 = DAT_001100f0;
  if (((DAT_00110210 == 3) && ((DAT_001100d8 & 0xf000) == 0x8000)) &&
     ((DAT_00110168 & 0xf000) == 0x8000)) {
    local_160 = FUN_0010447f(0);
    lVar8 = DAT_00110180;
    local_160 = lVar7 - local_160;
    local_158 = FUN_0010447f(1);
    local_158 = lVar8 - local_158;
    if (local_160 < 0) {
      local_160 = 0;
    }
    if (local_158 < 0) {
      local_158 = 0;
    }
    if (local_160 != local_158) {
      if (-1 < DAT_00110010) {
        lVar7 = local_160;
        if (local_158 < local_160) {
          lVar7 = local_158;
        }
        if (DAT_00110010 <= lVar7) goto LAB_0010368d;
      }
                    /* WARNING: Subroutine does not return */
      exit(1);
    }
  }
LAB_0010368d:
  DAT_001101f0 = FUN_00108307(DAT_001100f8,DAT_00110188,0x7ffffffffffffff7);
  local_150 = DAT_001101f0 + 0xfU >> 3;
  DAT_001101e0 = FUN_00107605(local_150 << 4);
  DAT_001101e8 = local_150 * 8 + DAT_001101e0;
  iVar3 = FUN_001037d2();
  for (local_174 = 0; local_174 < 2; local_174 = local_174 + 1) {
    iVar4 = close((&DAT_001100b0)[local_174]);
    if (iVar4 != 0) {
      uVar5 = (&DAT_001100a0)[local_174];
      piVar9 = __errno_location();
      error(2,*piVar9,&DAT_0010c100,uVar5);
    }
  }
  if ((iVar3 != 0) && (DAT_00110210 < 2)) {
    FUN_00102b8a();
  }
                    /* WARNING: Subroutine does not return */
  exit(iVar3);
}




// Function: cmp @ 0x37d2

bool cmp(void)

{
  byte bVar1;
  byte bVar2;
  undefined8 uVar3;
  void *__s1;
  void *__s2;
  char cVar4;
  int iVar5;
  long lVar6;
  ulong uVar7;
  int *piVar8;
  ulong uVar9;
  ulong __n;
  long lVar10;
  undefined8 uVar11;
  char *pcVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  long in_FS_OFFSET;
  bool bVar15;
  bool local_14a;
  int local_144;
  int local_140;
  uint local_13c;
  long local_138;
  long local_130;
  ulong local_128;
  ulong local_120;
  ulong local_118;
  ulong local_110;
  ulong local_108;
  undefined1 local_72 [5];
  undefined1 local_6d [5];
  undefined1 local_68 [32];
  undefined1 local_48 [24];
  long local_30;
  
  __s2 = DAT_001101e8;
  __s1 = DAT_001101e0;
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_14a = true;
  local_138 = 1;
  local_130 = 1;
  local_128 = DAT_00110010;
  local_144 = 0;
  if (DAT_00110210 == 1) {
    local_118 = DAT_00110010;
    if ((long)DAT_00110010 < 0) {
      local_118 = 0x7fffffffffffffff;
    }
    for (local_140 = 0; local_140 < 2; local_140 = local_140 + 1) {
      if (((&DAT_001100d8)[(long)local_140 * 0x24] & 0xf000) == 0x8000) {
        lVar10 = (&DAT_001100f0)[(long)local_140 * 0x12];
        lVar6 = FUN_0010447f(local_140);
        if (lVar10 - lVar6 < (long)local_118) {
          local_118 = lVar10 - lVar6;
        }
      }
    }
    local_13c = 1;
    while (local_118 = (long)local_118 / 10, local_118 != 0) {
      local_13c = local_13c + 1;
    }
  }
  for (local_140 = 0; local_140 < 2; local_140 = local_140 + 1) {
    local_110 = (&DAT_00110200)[local_140];
    if ((local_110 != 0) && (lVar10 = FUN_0010447f(local_140), lVar10 == -1)) {
      do {
        uVar7 = DAT_001101f0;
        if (local_110 <= DAT_001101f0) {
          uVar7 = local_110;
        }
        uVar9 = FUN_00108238((&DAT_001100b0)[local_140],__s1,uVar7);
        if (uVar9 != uVar7) {
          if (uVar9 != 0xffffffffffffffff) break;
          uVar11 = (&DAT_001100a0)[local_140];
          piVar8 = __errno_location();
          error(2,*piVar8,&DAT_0010c100,uVar11);
        }
        local_110 = local_110 - uVar9;
      } while (local_110 != 0);
    }
  }
  do {
    local_108 = DAT_001101f0;
    if (-1 < (long)local_128) {
      if (local_128 < DAT_001101f0) {
        local_108 = local_128;
      }
      local_128 = local_128 - local_108;
    }
    uVar7 = FUN_00108238(DAT_001100b0,__s1,local_108);
    uVar11 = DAT_001100a0;
    if (uVar7 == 0xffffffffffffffff) {
      piVar8 = __errno_location();
      error(2,*piVar8,&DAT_0010c100,uVar11);
    }
    uVar9 = FUN_00108238(DAT_001100b4,__s2,local_108);
    uVar11 = DAT_001100a8;
    if (uVar9 == 0xffffffffffffffff) {
      piVar8 = __errno_location();
      error(2,*piVar8,&DAT_0010c100,uVar11);
    }
    __n = uVar9;
    if (uVar7 <= uVar9) {
      __n = uVar7;
    }
    iVar5 = memcmp(__s1,__s2,__n);
    local_120 = __n;
    if (iVar5 != 0) {
      if (uVar9 <= uVar7) {
        *(undefined1 *)(uVar7 + (long)__s2) = 0x55;
      }
      if (uVar7 <= uVar9) {
        *(undefined1 *)(uVar9 + (long)__s1) = 0x79;
      }
      *(byte *)(uVar7 + (long)__s1) = ~*(byte *)(uVar7 + (long)__s2);
      *(byte *)(uVar9 + (long)__s2) = ~*(byte *)(uVar9 + (long)__s1);
      memset((void *)((long)__s1 + uVar7 + 1),0,(ulong)(~(uint)uVar7 & 7));
      memset((void *)((long)__s2 + uVar9 + 1),0,(ulong)(~(uint)uVar9 & 7));
      local_120 = FUN_001042d7(__s1,__s2);
    }
    local_130 = local_120 + local_130;
    if ((DAT_00110210 == 0) && (local_120 != 0)) {
      lVar10 = FUN_00104350(__s1,local_120);
      local_138 = lVar10 + local_138;
      local_14a = *(char *)((long)__s1 + (local_120 - 1)) == '\n';
    }
    if (local_120 < __n) {
      if (DAT_00110210 == 3) {
LAB_00103fa1:
        bVar15 = true;
        goto LAB_001042b6;
      }
      if (DAT_00110210 < 4) {
        if (DAT_00110210 == 2) {
          local_144 = 1;
        }
        else if (DAT_00110210 < 3) {
          if (DAT_00110210 == 0) {
            uVar11 = FUN_0010490d(local_130,local_68);
            uVar13 = FUN_0010490d(local_138,local_48);
            if (DAT_00110214 == '\x01') {
              bVar1 = *(byte *)(local_120 + (long)__s1);
              bVar2 = *(byte *)(local_120 + (long)__s2);
              FUN_001043cf(local_72,bVar1);
              FUN_001043cf(local_6d,bVar2);
              uVar3 = DAT_001100a8;
              uVar14 = DAT_001100a0;
              pcVar12 = (char *)gettext("%s %s differ: byte %s, line %s is %3o %s %3o %s\n");
              printf(pcVar12,uVar14,uVar3,uVar11,uVar13,(ulong)bVar1,local_72,(ulong)bVar2,local_6d)
              ;
            }
            else {
              pcVar12 = (char *)gettext("%s %s differ: byte %s, line %s\n");
              if ((pcVar12 == "%s %s differ: byte %s, line %s\n") &&
                 (cVar4 = FUN_00104863(5), cVar4 == '\0')) {
                bVar15 = false;
              }
              else {
                bVar15 = true;
              }
              if (!bVar15) {
                pcVar12 = "%s %s differ: char %s, line %s\n";
              }
              printf(pcVar12,DAT_001100a0,DAT_001100a8,uVar11,uVar13);
            }
            goto LAB_00103fa1;
          }
          if (DAT_00110210 == 1) {
            do {
              bVar1 = *(byte *)(local_120 + (long)__s1);
              bVar2 = *(byte *)(local_120 + (long)__s2);
              if (bVar1 != bVar2) {
                uVar11 = FUN_0010490d(local_130,local_48);
                if (DAT_00110214 == '\x01') {
                  FUN_001043cf(local_6d,bVar1);
                  FUN_001043cf(local_68,bVar2);
                  printf("%*s %3o %-4s %3o %s\n",(ulong)local_13c,uVar11,(ulong)bVar1,local_6d,
                         (ulong)bVar2,local_68);
                }
                else {
                  printf("%*s %3o %3o\n",(ulong)local_13c,uVar11,(ulong)bVar1,(ulong)bVar2);
                }
              }
              local_130 = local_130 + 1;
              local_120 = local_120 + 1;
            } while (local_120 < __n);
            local_144 = -1;
          }
        }
      }
    }
    if (uVar7 != uVar9) {
      if ((local_144 < 1) && (DAT_00110210 != 3)) {
        uVar11 = (&DAT_001100a0)[(int)(uint)(uVar9 < uVar7)];
        if (local_130 == 1) {
          pcVar12 = (char *)gettext("cmp: EOF on %s which is empty\n");
          fprintf(stderr,pcVar12,uVar11);
        }
        else {
          uVar13 = FUN_0010490d(local_130 + -1,local_68);
          if (DAT_00110210 == 0) {
            uVar14 = FUN_0010490d(local_138 - (ulong)local_14a,local_48);
            if (local_14a == false) {
              pcVar12 = (char *)gettext("cmp: EOF on %s after byte %s, in line %s\n");
            }
            else {
              pcVar12 = (char *)gettext("cmp: EOF on %s after byte %s, line %s\n");
            }
            fprintf(stderr,pcVar12,uVar11,uVar13,uVar14);
          }
          else {
            pcVar12 = (char *)gettext("cmp: EOF on %s after byte %s\n");
            fprintf(stderr,pcVar12,uVar11,uVar13);
          }
        }
      }
      bVar15 = true;
      goto LAB_001042b6;
    }
  } while ((local_144 < 1) && (uVar7 == DAT_001101f0));
  bVar15 = local_144 != 0;
LAB_001042b6:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar15;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: block_compare @ 0x42d7

long block_compare(long *param_1,long *param_2)

{
  long *local_28;
  long *local_20;
  long *local_18;
  long *local_10;
  
  local_20 = param_2;
  for (local_28 = param_1; *local_28 == *local_20; local_28 = local_28 + 1) {
    local_20 = local_20 + 1;
  }
  local_10 = local_20;
  for (local_18 = local_28; (char)*local_18 == (char)*local_10;
      local_18 = (long *)((long)local_18 + 1)) {
    local_10 = (long *)((long)local_10 + 1);
  }
  return (long)local_18 - (long)param_1;
}




// Function: count_newlines @ 0x4350

long count_newlines(undefined1 *param_1,long param_2)

{
  undefined1 uVar1;
  undefined1 *puVar2;
  long local_20;
  undefined1 *local_18;
  
  local_20 = 0;
  puVar2 = param_1 + param_2;
  uVar1 = *puVar2;
  *puVar2 = 10;
  local_18 = param_1;
  while( true ) {
    local_18 = rawmemchr(local_18,10);
    if (local_18 == puVar2) break;
    local_20 = local_20 + 1;
    local_18 = local_18 + 1;
  }
  *puVar2 = uVar1;
  return local_20;
}




// Function: sprintc @ 0x43cf

void sprintc(byte *param_1,byte param_2)

{
  ushort **ppuVar1;
  byte local_14;
  byte *local_10;
  
  ppuVar1 = __ctype_b_loc();
  local_14 = param_2;
  local_10 = param_1;
  if (((*ppuVar1)[param_2] & 0x4000) == 0) {
    local_10 = param_1;
    if ((char)param_2 < '\0') {
      *param_1 = 0x4d;
      local_10 = param_1 + 2;
      param_1[1] = 0x2d;
      local_14 = param_2 + 0x80;
    }
    if (local_14 < 0x20) {
      *local_10 = 0x5e;
      local_14 = local_14 + 0x40;
      local_10 = local_10 + 1;
    }
    else if (local_14 == 0x7f) {
      *local_10 = 0x5e;
      local_14 = 0x3f;
      local_10 = local_10 + 1;
    }
  }
  *local_10 = local_14;
  local_10[1] = 0;
  return;
}




// Function: file_position @ 0x447f

undefined8 file_position(int param_1)

{
  __off_t _Var1;
  
  if ((&DAT_00110215)[param_1] != '\x01') {
    (&DAT_00110215)[param_1] = 1;
    _Var1 = lseek((&DAT_001100b0)[param_1],(&DAT_00110200)[param_1],1);
    *(__off_t *)(&DAT_00110220 + (long)param_1 * 8) = _Var1;
  }
  return *(undefined8 *)(&DAT_00110220 + (long)param_1 * 8);
}




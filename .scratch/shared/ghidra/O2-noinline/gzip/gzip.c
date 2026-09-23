// Function: main @ 0x39c0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void main(undefined4 param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  int iVar2;
  int iVar3;
  uint uVar4;
  char *__s;
  size_t sVar5;
  int *piVar6;
  int iVar7;
  undefined8 *__ptr;
  long in_FS_OFFSET;
  undefined1 auVar8 [16];
  undefined8 uStack_68;
  undefined1 auStack_60 [16];
  int local_50;
  int local_4c;
  undefined8 *local_48;
  undefined8 local_40;
  
  uStack_68 = CONCAT44(param_1,(undefined4)uStack_68);
  local_40 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  __s = (char *)thunk_FUN_0010e060(*param_2);
  DAT_0011b850 = __s;
  sVar5 = strlen(__s);
  if (4 < sVar5) {
    iVar2 = strcmp(__s + (sVar5 - 4),".exe");
    if (iVar2 == 0) {
      __s[sVar5 - 4] = '\0';
    }
  }
  local_48 = param_2;
  DAT_001dd9d0 = FUN_0010d3d0(&local_50,&local_48,&DAT_00112e61);
  __ptr = (undefined8 *)0x0;
  if (DAT_001dd9d0 != 0) {
    __ptr = local_48;
  }
  DAT_001dd9c8 = &DAT_00112db3;
  DAT_001dd9c0 = strlen(".gz");
  local_4c = -1;
  if (__ptr != (undefined8 *)0x0) {
    if (((char *)__ptr[optind] == (char *)0x0) ||
       (iVar2 = strcmp((char *)__ptr[optind],"--"), iVar2 != 0)) {
      iVar3 = getopt_long(local_50,__ptr,"ab:cdfhH?klLmMnNqrS:tvVZ123456789",&PTR_s_ascii_001187c0,
                          &local_4c);
      iVar2 = iVar3 + 0x83;
      if (iVar3 < 0) goto LAB_00103bc0;
      uVar4 = iVar3 + 0x52;
      if (0xd3 < uVar4) goto LAB_00103ba1;
LAB_00103b5e:
                    /* WARNING: Could not recover jumptable at 0x00103b6c. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (*(code *)(&DAT_00112f80 + *(int *)(&DAT_00112f80 + (ulong)uVar4 * 4)))();
      return;
    }
    if (__ptr != (undefined8 *)0x0) {
      iVar2 = 0xb0;
      goto LAB_00103ac1;
    }
  }
  do {
    iVar2 = getopt_long(uStack_68._4_4_,param_2,"ab:cdfhH?klLmMnNqrS:tvVZ123456789",
                        &PTR_s_ascii_001187c0);
    iVar3 = optind;
    if (iVar2 < 0) {
      if (DAT_00119028 < 0) {
        DAT_00119028 = DAT_001dd9f8;
      }
      if (DAT_0011902c < 0) {
        DAT_0011902c = DAT_001dd9f8;
      }
      if (DAT_001dd9fc == 0) goto LAB_00104085;
      if (DAT_0011b85c == 0) goto LAB_00104069;
      if (0x1d < DAT_001dd9c0 - 1) {
LAB_001040cd:
        FUN_0010f630(stderr,"%s: invalid suffix \'%s\'\n",DAT_0011b850,DAT_001dd9c8);
                    /* WARNING: Subroutine does not return */
        FUN_00106fd0(1);
      }
      do {
        DAT_001dd920 = 0xd;
        do {
          iVar2 = uStack_68._4_4_;
          iVar7 = uStack_68._4_4_ - iVar3;
          FUN_00105570();
          if (iVar2 == iVar3) {
            FUN_00107070();
          }
          else {
            while (optind < uStack_68._4_4_) {
              optind = optind + 1;
              FUN_00107b60();
            }
          }
          if ((DAT_001dd510 != '\0') && (iVar2 = close(0), iVar2 != 0)) {
            _DAT_0011b420 = 0x69647473;
            _DAT_0011b424 = 0x6e;
                    /* WARNING: Subroutine does not return */
            FUN_0010d600();
          }
          if (DAT_001dd9e8 == 0) {
LAB_0010401c:
            if (((DAT_0011b860 == 0) ||
                ((((DAT_001dda00 == '\0' || (iVar2 = fdatasync(1), iVar2 == 0)) ||
                  (piVar6 = __errno_location(), *piVar6 == 0x16)) && (iVar2 = close(1), iVar2 == 0))
                )) || (piVar6 = __errno_location(), *piVar6 == 9)) {
                    /* WARNING: Subroutine does not return */
              FUN_00106fd0(DAT_0011b84c);
            }
          }
          else {
            if ((DAT_0011b85c == 0) && (1 < iVar7)) {
              FUN_00106c40(0xffffffff);
            }
            iVar2 = FUN_0010e460(stdout);
            if (iVar2 == 0) goto LAB_0010401c;
          }
          FUN_0010d670();
LAB_00104069:
          FUN_0010f630(stderr,"%s: option --ascii ignored on this system\n",DAT_0011b850);
LAB_00104085:
          if (0x1d < DAT_001dd9c0 - 1) goto LAB_001040cd;
          DAT_001dd920 = 0;
        } while (DAT_0011b85c == 0);
      } while( true );
    }
    uVar4 = iVar2 - 0x31;
    __ptr = (undefined8 *)0x0;
    if (uVar4 < 0xd4) goto LAB_00103b5e;
LAB_00103ba1:
    if ((0x82 < iVar2) && (iVar2 != 0xc2)) {
LAB_00103ac1:
      uStack_68 = CONCAT44(iVar2,(undefined4)uStack_68);
      FUN_0010f630(stderr,&DAT_001137cc,DAT_0011b850);
      if (local_4c < 0) {
        FUN_0010f630(stderr,"-%c: ",uStack_68._4_4_ + -0x83);
      }
      else {
        FUN_0010f630(stderr,"--%s: ",(&PTR_s_ascii_001187c0)[(long)local_4c * 4]);
      }
      FUN_0010f630(stderr,"option not valid in GZIP environment variable\n");
    }
    FUN_00107040();
LAB_00103bc0:
    if (optind != local_50) {
      FUN_0010f630(stderr,"%s: %s: non-option in GZIP environment variable\n",DAT_0011b850,
                   __ptr[optind]);
      auVar8 = FUN_00107040();
      uVar1 = uStack_68;
      uStack_68 = auVar8._0_8_;
      (*(code *)PTR___libc_start_main_00118fc0)
                (main,uVar1,auStack_60,0,0,auVar8._8_8_,&uStack_68);
      do {
                    /* WARNING: Do nothing block with infinite loop */
      } while( true );
    }
    if ((optind != 1) && (DAT_0011b85c == 0)) {
      FUN_0010f630(stderr,
                   "%s: warning: GZIP environment variable is deprecated; use an alias or script\n",
                   DAT_0011b850);
    }
    free(__ptr);
    local_4c = -1;
    optind = 1;
  } while( true );
}




// Function: bi_init @ 0x4260

void bi_init(int param_1)

{
  DAT_0011a058 = 0;
  DAT_0011a05c = 0;
  if (param_1 != -1) {
    DAT_0011a050 = FUN_0010e010;
  }
  return;
}




// Function: send_bits @ 0x4290

void send_bits(uint param_1,int param_2)

{
  int iVar1;
  ushort uVar2;
  ulong uVar3;
  uint uVar4;
  ulong uVar5;
  
  uVar2 = (ushort)(param_1 << ((byte)DAT_0011a058 & 0x1f)) | DAT_0011a05c;
  iVar1 = DAT_0011a058;
  if (0x10 - param_2 < DAT_0011a058) {
    uVar3 = (ulong)DAT_0011b004;
    uVar4 = DAT_0011b004 + 1;
    uVar5 = (ulong)uVar4;
    if (DAT_0011b004 < 0x3fffe) {
      uVar3 = (ulong)DAT_0011b004;
      DAT_0011b004 = DAT_0011b004 + 2;
      (&DAT_0015c000)[uVar3] = (char)uVar2;
      (&DAT_0015c000)[uVar5] = (char)(uVar2 >> 8);
    }
    else {
      DAT_0011a05c = uVar2;
      DAT_0011b004 = uVar4;
      (&DAT_0015c000)[uVar3] = (char)uVar2;
      if (uVar4 == 0x40000) {
        FUN_0010d7a0();
        uVar5 = (ulong)DAT_0011b004;
        uVar2 = DAT_0011a05c;
      }
      uVar4 = (int)uVar5 + 1;
      DAT_0011b004 = uVar4;
      (&DAT_0015c000)[uVar5] = (char)(uVar2 >> 8);
      if (uVar4 == 0x40000) {
        FUN_0010d7a0();
      }
    }
    iVar1 = DAT_0011a058 + -0x10;
    uVar2 = (ushort)((int)(param_1 & 0xffff) >> (0x10U - (char)DAT_0011a058 & 0x1f));
  }
  DAT_0011a05c = uVar2;
  DAT_0011a058 = param_2 + iVar1;
  return;
}




// Function: bi_reverse @ 0x4380

uint bi_reverse(uint param_1,int param_2)

{
  uint uVar1;
  uint uVar2;
  uint uVar3;
  
  uVar1 = 0;
  do {
    uVar3 = uVar1;
    param_2 = param_2 + -1;
    uVar2 = param_1 & 1;
    param_1 = param_1 >> 1;
    uVar1 = (uVar2 | uVar3) * 2;
  } while (0 < param_2);
  return uVar2 | uVar3 & 0x7fffffff;
}




// Function: bi_windup @ 0x43b0

void bi_windup(void)

{
  undefined2 uVar1;
  ulong uVar2;
  undefined1 uVar3;
  uint uVar4;
  int iVar5;
  ulong uVar6;
  
  uVar1 = DAT_0011a05c;
  uVar3 = (undefined1)DAT_0011a05c;
  if (DAT_0011a058 < 9) {
    if (DAT_0011a058 < 1) {
      DAT_0011a058 = 0;
      DAT_0011a05c = 0;
      return;
    }
    uVar2 = (ulong)DAT_0011b004;
    iVar5 = DAT_0011b004 + 1;
    DAT_0011b004 = iVar5;
    (&DAT_0015c000)[uVar2] = uVar3;
  }
  else {
    uVar2 = (ulong)DAT_0011b004;
    uVar4 = DAT_0011b004 + 1;
    uVar6 = (ulong)uVar4;
    if (DAT_0011b004 < 0x3fffe) {
      uVar2 = (ulong)DAT_0011b004;
      DAT_0011b004 = DAT_0011b004 + 2;
      (&DAT_0015c000)[uVar2] = uVar3;
      (&DAT_0015c000)[uVar6] = (char)((ushort)DAT_0011a05c >> 8);
      DAT_0011a058 = 0;
      DAT_0011a05c = 0;
      return;
    }
    DAT_0011b004 = uVar4;
    (&DAT_0015c000)[uVar2] = uVar3;
    if (uVar4 == 0x40000) {
      FUN_0010d7a0();
      uVar6 = (ulong)DAT_0011b004;
      uVar1 = DAT_0011a05c;
    }
    DAT_0011b004 = (int)uVar6 + 1;
    (&DAT_0015c000)[uVar6] = (char)((ushort)uVar1 >> 8);
    iVar5 = DAT_0011b004;
  }
  if (iVar5 == 0x40000) {
    FUN_0010d7a0();
  }
  DAT_0011a058 = 0;
  DAT_0011a05c = 0;
  return;
}




// Function: copy_block @ 0x44a0

void copy_block(undefined1 *param_1,uint param_2,int param_3)

{
  undefined1 uVar1;
  byte bVar2;
  ulong uVar3;
  uint uVar4;
  uint uVar5;
  undefined1 *puVar7;
  byte bVar8;
  ulong uVar6;
  
  FUN_001043b0();
  if (param_3 != 0) {
    uVar3 = (ulong)DAT_0011b004;
    bVar2 = (byte)(param_2 >> 8);
    uVar4 = DAT_0011b004 + 1;
    uVar6 = (ulong)uVar4;
    bVar8 = (byte)param_2;
    if (DAT_0011b004 < 0x3fffe) {
      uVar3 = (ulong)(DAT_0011b004 + 2);
      (&DAT_0015c000)[DAT_0011b004] = bVar8;
      (&DAT_0015c000)[uVar6] = bVar2;
    }
    else {
      DAT_0011b004 = uVar4;
      (&DAT_0015c000)[uVar3] = bVar8;
      if (uVar4 == 0x40000) {
        FUN_0010d7a0();
        uVar6 = (ulong)DAT_0011b004;
      }
      uVar4 = (int)uVar6 + 1;
      uVar3 = (ulong)uVar4;
      DAT_0011b004 = uVar4;
      (&DAT_0015c000)[uVar6] = bVar2;
      if (uVar4 == 0x40000) {
        FUN_0010d7a0();
        uVar3 = (ulong)DAT_0011b004;
      }
    }
    uVar4 = (uint)uVar3;
    uVar5 = uVar4 + 1;
    uVar6 = (ulong)uVar5;
    if (uVar4 < 0x3fffe) {
      DAT_0011b004 = uVar4 + 2;
      (&DAT_0015c000)[uVar3] = ~bVar8;
      (&DAT_0015c000)[uVar6] = ~bVar2;
    }
    else {
      DAT_0011b004 = uVar5;
      (&DAT_0015c000)[uVar3] = ~bVar8;
      if (uVar5 == 0x40000) {
        FUN_0010d7a0();
        uVar6 = (ulong)DAT_0011b004;
      }
      uVar4 = (int)uVar6 + 1;
      DAT_0011b004 = uVar4;
      (&DAT_0015c000)[uVar6] = ~bVar2;
      if (uVar4 == 0x40000) {
        FUN_0010d7a0();
      }
    }
  }
  puVar7 = param_1 + param_2;
  if (param_2 != 0) {
    do {
      while( true ) {
        uVar3 = (ulong)DAT_0011b004;
        uVar1 = *param_1;
        param_1 = param_1 + 1;
        uVar4 = DAT_0011b004 + 1;
        DAT_0011b004 = uVar4;
        (&DAT_0015c000)[uVar3] = uVar1;
        if (uVar4 != 0x40000) break;
        FUN_0010d7a0();
        if (param_1 == puVar7) {
          return;
        }
      }
    } while (param_1 != puVar7);
  }
  return;
}




// Function: longest_match @ 0x4620

uint longest_match(uint param_1)

{
  char *pcVar1;
  ushort uVar2;
  bool bVar3;
  ulong uVar4;
  ulong uVar5;
  char *pcVar6;
  uint uVar7;
  char cVar8;
  uint uVar9;
  long lVar10;
  char *pcVar11;
  uint uVar12;
  uint uVar13;
  char cVar14;
  uint local_34;
  
  uVar4 = (ulong)DAT_0011a06c;
  uVar12 = DAT_0011a06c;
  if (DAT_0011a06c < 0x7efa) {
    uVar12 = 0x7efa;
  }
  lVar10 = (long)(int)DAT_0011a070;
  pcVar1 = &DAT_0013c000 + uVar4;
  cVar8 = *(char *)(uVar4 + 0x13bfff + lVar10);
  cVar14 = pcVar1[lVar10];
  uVar7 = DAT_0011a064;
  if (DAT_0011a060 <= DAT_0011a070) {
    uVar7 = DAT_0011a064 >> 2;
  }
  bVar3 = false;
  uVar13 = DAT_0011a070;
  do {
    uVar5 = (ulong)param_1;
    if (((((&DAT_0013c000 + uVar5)[lVar10] == cVar14) &&
         (*(char *)(uVar5 + 0x13bfff + lVar10) == cVar8)) && ((&DAT_0013c000)[uVar5] == *pcVar1)) &&
       ((&DAT_0013c001)[uVar5] == (&DAT_0013c001)[uVar4])) {
      pcVar11 = (char *)(uVar4 + 0x13c002);
      pcVar6 = (char *)(uVar5 + 0x13c002);
      while( true ) {
        if (pcVar11[1] != pcVar6[1]) {
          pcVar11 = pcVar11 + 1;
          goto LAB_00104770;
        }
        if (pcVar11[2] != pcVar6[2]) break;
        if (pcVar11[3] != pcVar6[3]) {
          pcVar11 = pcVar11 + 3;
          goto LAB_00104770;
        }
        if (pcVar11[4] != pcVar6[4]) {
          pcVar11 = pcVar11 + 4;
          goto LAB_00104770;
        }
        if (pcVar11[5] != pcVar6[5]) {
          pcVar11 = pcVar11 + 5;
          goto LAB_00104770;
        }
        if (pcVar11[6] != pcVar6[6]) {
          pcVar11 = pcVar11 + 6;
          goto LAB_00104770;
        }
        if (pcVar11[7] != pcVar6[7]) {
          pcVar11 = pcVar11 + 7;
          goto LAB_00104770;
        }
        pcVar11 = pcVar11 + 8;
        pcVar6 = pcVar6 + 8;
        if ((*pcVar11 != *pcVar6) || ((char *)(uVar4 + 0x13c102) <= pcVar11)) goto LAB_00104770;
      }
      pcVar11 = pcVar11 + 2;
LAB_00104770:
      uVar9 = 0x102 - ((int)(char *)(uVar4 + 0x13c102) - (int)pcVar11);
      if ((int)uVar9 <= (int)uVar13) goto LAB_00104698;
      if (DAT_0011a080 <= (int)uVar9) {
        DAT_0011a068 = param_1;
        return uVar9;
      }
      uVar2 = (&DAT_0011b880)[param_1 & 0x7fff];
      cVar8 = *(char *)(uVar4 + 0x13bfff + (long)(int)uVar9);
      bVar3 = true;
      cVar14 = pcVar1[(int)uVar9];
      uVar13 = uVar9;
      local_34 = param_1;
    }
    else {
LAB_00104698:
      uVar2 = (&DAT_0011b880)[param_1 & 0x7fff];
    }
    param_1 = (uint)uVar2;
    if ((param_1 <= uVar12 - 0x7efa) || (uVar7 = uVar7 - 1, uVar7 == 0)) {
      if (!bVar3) {
        return uVar13;
      }
      DAT_0011a068 = local_34;
      return uVar13;
    }
    lVar10 = (long)(int)uVar13;
  } while( true );
}




// Function: rsync_roll @ 0x4840

void rsync_roll(uint param_1,uint param_2)

{
  byte *pbVar1;
  byte bVar2;
  bool bVar3;
  ulong uVar4;
  byte *pbVar5;
  ulong uVar6;
  
  uVar4 = (ulong)param_1;
  if (param_1 < 0x1000) {
    pbVar5 = &DAT_0013c000 + param_1;
    pbVar1 = pbVar5 + param_2;
    bVar3 = false;
    uVar6 = DAT_0011a090;
    do {
      if (pbVar1 == pbVar5) {
        if (!bVar3) {
          return;
        }
        DAT_0011a090 = uVar6;
        return;
      }
      bVar2 = *pbVar5;
      pbVar5 = pbVar5 + 1;
      uVar6 = uVar6 + bVar2;
      bVar3 = true;
    } while (&DAT_0013c001 + (ulong)(0xfff - param_1) + (ulong)param_1 != pbVar5);
    uVar4 = 0x1000;
    DAT_0011a090 = uVar6;
  }
  if (param_1 + param_2 <= (uint)uVar4) {
    return;
  }
  bVar3 = false;
  uVar6 = DAT_0011a088;
  do {
    DAT_0011a090 = DAT_0011a090 +
                   ((ulong)(byte)(&DAT_0013c000)[uVar4] -
                   (ulong)(byte)(&DAT_0013c000)[(int)uVar4 - 0x1000]);
    if ((uVar6 == 0xffffffff) && ((DAT_0011a090 & 0xfff) == 0)) {
      bVar3 = true;
      uVar6 = uVar4;
    }
    uVar4 = uVar4 + 1;
  } while ((uint)uVar4 < param_1 + param_2);
  if (bVar3) {
    DAT_0011a088 = uVar6;
  }
  return;
}




// Function: fill_window @ 0x4940

void fill_window(void)

{
  short *psVar1;
  short sVar2;
  short *psVar3;
  ulong uVar4;
  int iVar5;
  int iVar6;
  uint uVar7;
  
  uVar4 = (ulong)DAT_0011a09c;
  iVar5 = DAT_0011a09c + DAT_0011a06c;
  iVar6 = 0x10000 - iVar5;
  if (iVar6 == -1) {
    iVar6 = -2;
  }
  else if (0xfef9 < DAT_0011a06c) {
    uVar7 = DAT_0011a06c - 0x8000;
    memcpy(&DAT_0013c000,&DAT_00144000,0x8000);
    DAT_0011a068 = DAT_0011a068 + -0x8000;
    if (DAT_0011a088 != 0xffffffff) {
      DAT_0011a088 = DAT_0011a088 + -0x8000;
    }
    DAT_0011a078 = DAT_0011a078 + -0x8000;
    psVar1 = &DAT_0012b880;
    DAT_0011a06c = uVar7;
    do {
      sVar2 = *psVar1 + -0x8000;
      if (-1 < *psVar1) {
        sVar2 = 0;
      }
      psVar3 = psVar1 + 1;
      *psVar1 = sVar2;
      psVar1 = psVar3;
    } while (psVar3 != (short *)&DAT_0013b880);
    psVar1 = &DAT_0011b880;
    do {
      sVar2 = *psVar1 + -0x8000;
      if (-1 < *psVar1) {
        sVar2 = 0;
      }
      psVar3 = psVar1 + 1;
      *psVar1 = sVar2;
      psVar1 = psVar3;
    } while (psVar3 != &DAT_0012b880);
    iVar6 = 0x18000 - iVar5;
  }
  if (DAT_0011a0a0 == 0) {
    iVar5 = (*DAT_0011a050)(&DAT_0013c000 + DAT_0011a06c + uVar4,iVar6);
    if (0xfffffffd < iVar5 - 1U) {
      DAT_0011a0a0 = 1;
      *(undefined2 *)(&DAT_0013c000 + (ulong)DAT_0011a06c + (ulong)DAT_0011a09c) = 0;
      return;
    }
    DAT_0011a09c = DAT_0011a09c + iVar5;
  }
  return;
}




// Function: deflate_fast @ 0x4aa0

void deflate_fast(void)

{
  short *psVar1;
  ushort uVar2;
  short sVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  uint uVar7;
  uint uVar8;
  int iVar9;
  ulong uVar10;
  undefined1 *puVar11;
  
  DAT_0011a070 = 2;
  if (DAT_0011a09c == 0) {
    iVar6 = -1;
LAB_00104c1a:
    puVar11 = (undefined1 *)0x0;
    if (-1 < (long)DAT_0011a078) {
      puVar11 = &DAT_0013c000 + (DAT_0011a078 & 0xffffffff);
    }
    FUN_0010b070(puVar11,DAT_0011a06c - DAT_0011a078,iVar6,1);
    return;
  }
  do {
    uVar4 = DAT_0011a09c;
    uVar7 = DAT_0011a06c;
    uVar10 = (ulong)DAT_0011a06c;
    DAT_0011a0a4 = (DAT_0011a0a4 << 5 ^ (uint)(byte)(&DAT_0013c000)[DAT_0011a06c + 2]) & 0x7fff;
    uVar5 = DAT_0011a0a4 + 0x8000;
    uVar2 = (&DAT_0011b880)[uVar5];
    (&DAT_0011b880)[DAT_0011a06c & 0x7fff] = uVar2;
    (&DAT_0011b880)[uVar5] = (ushort)uVar7;
    if (((uVar2 == 0) || (0x7efa < uVar7 - uVar2)) || (0xfefa < uVar7)) {
LAB_00104b3a:
      iVar6 = FUN_0010b290(0,(&DAT_0013c000)[uVar10]);
      iVar9 = DAT_0011b000;
      uVar4 = DAT_0011a06c;
      if (DAT_0011b000 != 0) {
        FUN_00104840(DAT_0011a06c,1);
      }
      DAT_0011a06c = uVar4 + 1;
      DAT_0011a09c = DAT_0011a09c - 1;
    }
    else {
      uVar7 = FUN_00104620();
      if (uVar7 <= uVar4) {
        uVar4 = uVar7;
      }
      if (uVar4 < 3) {
        uVar10 = (ulong)DAT_0011a06c;
        goto LAB_00104b3a;
      }
      iVar6 = FUN_0010b290(DAT_0011a06c - DAT_0011a068,uVar4 - 3);
      iVar9 = DAT_0011b000;
      uVar7 = DAT_0011a06c;
      DAT_0011a09c = DAT_0011a09c - uVar4;
      if (DAT_0011b000 == 0) {
        if (uVar4 <= DAT_0011a098) goto LAB_00104d21;
LAB_00104cc0:
        DAT_0011a06c = uVar4 + uVar7;
        DAT_0011a0a4 = (uint)(byte)(&DAT_0013c000)[DAT_0011a06c] << 5 ^
                       (uint)(byte)(&DAT_0013c000)[DAT_0011a06c + 1];
      }
      else {
        FUN_00104840(DAT_0011a06c,uVar4);
        if (DAT_0011a098 < uVar4) goto LAB_00104cc0;
LAB_00104d21:
        uVar5 = uVar7;
        do {
          uVar8 = uVar5 + 1;
          DAT_0011a0a4 = (DAT_0011a0a4 << 5 ^ (uint)(byte)(&DAT_0013c000)[uVar5 + 3]) & 0x7fff;
          psVar1 = &DAT_0011b880 + (DAT_0011a0a4 + 0x8000);
          sVar3 = *psVar1;
          (&DAT_0011b880)[uVar8 & 0x7fff] = sVar3;
          *psVar1 = (short)uVar8;
          uVar5 = uVar8;
        } while (uVar8 != (uVar4 - 1) + uVar7);
        DAT_0011a06c = uVar7 + uVar4;
        (&DAT_0011b880)[uVar8 & 0x7fff] = sVar3;
        *psVar1 = (short)uVar7 + -1 + (short)uVar4;
      }
    }
    if ((iVar9 == 0) || (DAT_0011a06c <= DAT_0011a088)) {
      if (iVar6 != 0) {
        iVar9 = iVar6 + -1;
        goto LAB_00104ba5;
      }
    }
    else {
      iVar9 = 1;
      iVar6 = 2;
      DAT_0011a088 = 0xffffffff;
LAB_00104ba5:
      puVar11 = &DAT_0013c000 + (DAT_0011a078 & 0xffffffff);
      if ((long)DAT_0011a078 < 0) {
        puVar11 = (undefined1 *)0x0;
      }
      FUN_0010b070(puVar11,DAT_0011a06c - DAT_0011a078,iVar9,0);
      DAT_0011a078 = (ulong)DAT_0011a06c;
    }
    while (DAT_0011a09c < 0x106) {
      if (DAT_0011a0a0 != 0) {
        if (DAT_0011a09c == 0) {
          iVar6 = iVar6 + -1;
          DAT_0011a09c = 0;
          goto LAB_00104c1a;
        }
        break;
      }
      FUN_00104940();
    }
  } while( true );
}




// Function: lm_init @ 0x4db0

void lm_init(int param_1)

{
  long lVar1;
  
  if (8 < param_1 - 1U) {
                    /* WARNING: Subroutine does not return */
    FUN_0010d510("bad pack level");
  }
  memset(&DAT_0012b880,0,0x10000);
  lVar1 = (long)param_1;
  DAT_0011a090 = 0;
  DAT_0011a088 = 0xffffffff;
  DAT_0011a06c = 0;
  DAT_0011a078 = 0;
  DAT_0011a098 = (uint)*(ushort *)(&DAT_00112022 + lVar1 * 8);
  DAT_0011a060 = (uint)*(ushort *)(&DAT_00112020 + lVar1 * 8);
  DAT_0011a080 = (uint)*(ushort *)(&DAT_00112024 + lVar1 * 8);
  DAT_0011a064 = (uint)*(ushort *)(&DAT_00112026 + lVar1 * 8);
  DAT_0011a09c = (*DAT_0011a050)(&DAT_0013c000,0x10000);
  if (0xfffffffd < DAT_0011a09c - 1) {
    DAT_0011a0a0 = 1;
    DAT_0011a09c = 0;
    return;
  }
  DAT_0011a0a0 = 0;
  if (DAT_0011a09c < 0x106) {
    do {
      FUN_00104940();
      if (0x105 < DAT_0011a09c) break;
    } while (DAT_0011a0a0 == 0);
  }
  DAT_0011a0a4 = (uint)DAT_0013c000 << 5 ^ (uint)DAT_0013c001;
  return;
}




// Function: deflate @ 0x4ed0

void deflate(int param_1)

{
  int iVar1;
  short *psVar2;
  ushort uVar3;
  short sVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  ulong uVar8;
  int iVar9;
  uint uVar10;
  uint uVar11;
  undefined1 *puVar12;
  int iVar13;
  int iVar14;
  
  FUN_00104db0();
  if (param_1 < 4) {
    FUN_00104aa0();
    return;
  }
  iVar13 = -1;
  if (DAT_0011a09c == 0) {
LAB_0010509d:
    puVar12 = (undefined1 *)0x0;
    if (-1 < (long)DAT_0011a078) {
      puVar12 = &DAT_0013c000 + (DAT_0011a078 & 0xffffffff);
    }
    FUN_0010b070(puVar12,DAT_0011a06c - DAT_0011a078,iVar13,1);
    return;
  }
  iVar14 = 0;
  iVar13 = 0;
  uVar10 = DAT_0011a09c;
  uVar11 = 2;
  do {
    DAT_0011a070 = uVar11;
    uVar7 = DAT_0011a06c;
    uVar8 = (ulong)DAT_0011a06c;
    uVar11 = 2;
    DAT_0011a0a4 = (DAT_0011a0a4 << 5 ^ (uint)(byte)(&DAT_0013c000)[DAT_0011a06c + 2]) & 0x7fff;
    uVar5 = DAT_0011a0a4 + 0x8000;
    uVar3 = (&DAT_0011b880)[uVar5];
    (&DAT_0011b880)[DAT_0011a06c & 0x7fff] = uVar3;
    iVar9 = DAT_0011a068;
    (&DAT_0011b880)[uVar5] = (ushort)uVar7;
    if ((uVar3 != 0) && (DAT_0011a070 < DAT_0011a098)) {
      if ((uVar7 - uVar3 < 0x7efb) && (uVar7 < 0xfefb)) {
        uVar7 = FUN_00104620();
        uVar11 = uVar10;
        if (uVar7 <= uVar10) {
          uVar11 = uVar7;
        }
        if (uVar11 == 3) {
          uVar8 = (ulong)DAT_0011a06c;
          if (0x1000 < DAT_0011a06c - DAT_0011a068) {
            uVar11 = 2;
          }
        }
        else {
          uVar8 = (ulong)DAT_0011a06c;
        }
      }
      else {
        uVar11 = 2;
      }
    }
    uVar10 = (uint)uVar8;
    if ((DAT_0011a070 < 3) || (DAT_0011a070 < uVar11)) {
      if (iVar14 == 0) {
        if (DAT_0011b000 != 0) {
          if (DAT_0011a088 < uVar8) {
            iVar13 = 2;
            DAT_0011a088 = 0xffffffff;
            puVar12 = &DAT_0013c000 + (DAT_0011a078 & 0xffffffff);
            if ((long)DAT_0011a078 < 0) {
              puVar12 = (undefined1 *)0x0;
            }
            FUN_0010b070(puVar12,uVar8 - DAT_0011a078,1,0);
            uVar8 = (ulong)DAT_0011a06c;
            uVar10 = DAT_0011a06c;
            DAT_0011a078 = uVar8;
            if (DAT_0011b000 == 0) goto LAB_00105030;
          }
          uVar10 = (uint)uVar8;
          FUN_00104840(uVar8,1);
        }
LAB_00105030:
        DAT_0011a06c = uVar10 + 1;
        iVar14 = 1;
        DAT_0011a09c = DAT_0011a09c - 1;
        uVar10 = DAT_0011a09c;
      }
      else {
        iVar13 = FUN_0010b290(0,(&DAT_0013c000)[uVar10 - 1]);
        if (DAT_0011b000 == 0) {
          iVar9 = iVar13 + -1;
          if (iVar13 != 0) goto LAB_0010512c;
        }
        else {
          uVar8 = (ulong)DAT_0011a06c;
          if (DAT_0011a088 < uVar8) {
            iVar13 = 2;
            DAT_0011a088 = 0xffffffff;
            iVar9 = iVar14;
LAB_0010512c:
            puVar12 = &DAT_0013c000 + (DAT_0011a078 & 0xffffffff);
            if ((long)DAT_0011a078 < 0) {
              puVar12 = (undefined1 *)0x0;
            }
            FUN_0010b070(puVar12,DAT_0011a06c - DAT_0011a078,iVar9,0);
            uVar8 = (ulong)DAT_0011a06c;
            DAT_0011a078 = uVar8;
            if (DAT_0011b000 == 0) goto LAB_00105168;
          }
          else {
            iVar9 = iVar13 + -1;
            if (iVar13 != 0) goto LAB_0010512c;
          }
          FUN_00104840(uVar8,1);
          DAT_0011a06c = (uint)uVar8;
        }
LAB_00105168:
        DAT_0011a06c = DAT_0011a06c + 1;
        DAT_0011a09c = DAT_0011a09c - 1;
        uVar10 = DAT_0011a09c;
      }
    }
    else {
      iVar13 = FUN_0010b290((uVar10 - iVar9) + -1,DAT_0011a070 - 3);
      iVar9 = DAT_0011b000;
      uVar7 = DAT_0011a070;
      uVar11 = DAT_0011a06c;
      iVar14 = DAT_0011a070 - 2;
      uVar10 = (DAT_0011a09c + 1) - DAT_0011a070;
      DAT_0011a09c = uVar10;
      if (DAT_0011b000 != 0) {
        iVar1 = DAT_0011a070 - 1;
        DAT_0011a070 = iVar14;
        FUN_00104840(DAT_0011a06c,iVar1);
      }
      uVar5 = uVar11;
      do {
        uVar6 = uVar5 + 1;
        DAT_0011a0a4 = (DAT_0011a0a4 << 5 ^ (uint)(byte)(&DAT_0013c000)[uVar5 + 3]) & 0x7fff;
        psVar2 = &DAT_0011b880 + (DAT_0011a0a4 + 0x8000);
        sVar4 = *psVar2;
        (&DAT_0011b880)[uVar6 & 0x7fff] = sVar4;
        *psVar2 = (short)uVar6;
        uVar5 = uVar6;
      } while (uVar6 != iVar14 + uVar11);
      DAT_0011a06c = (uVar7 - 1) + uVar11;
      (&DAT_0011b880)[uVar6 & 0x7fff] = sVar4;
      *psVar2 = (short)uVar11 + -2 + (short)uVar7;
      DAT_0011a070 = 0;
      if ((iVar9 == 0) || (DAT_0011a06c <= DAT_0011a088)) {
        if (iVar13 == 0) {
          iVar14 = 0;
          uVar11 = 2;
          goto joined_r0x00105055;
        }
        iVar9 = iVar13 + -1;
      }
      else {
        iVar9 = 1;
        iVar13 = 2;
        DAT_0011a088 = 0xffffffff;
      }
      uVar11 = 2;
      puVar12 = &DAT_0013c000 + (DAT_0011a078 & 0xffffffff);
      if ((long)DAT_0011a078 < 0) {
        puVar12 = (undefined1 *)0x0;
      }
      iVar14 = 0;
      FUN_0010b070(puVar12,DAT_0011a06c - DAT_0011a078,iVar9,0);
      DAT_0011a078 = (ulong)DAT_0011a06c;
      uVar10 = DAT_0011a09c;
    }
joined_r0x00105055:
    while (uVar10 < 0x106) {
      if (DAT_0011a0a0 != 0) {
        uVar10 = DAT_0011a09c;
        if (DAT_0011a09c == 0) {
          iVar13 = iVar13 + -1;
          if (iVar14 != 0) {
            FUN_0010b290(0,(&DAT_0013c000)[DAT_0011a06c - 1]);
          }
          goto LAB_0010509d;
        }
        break;
      }
      FUN_00104940();
      uVar10 = DAT_0011a09c;
    }
  } while( true );
}




// Function: get_input_size_and_time @ 0x53f0

void get_input_size_and_time(void)

{
  undefined1 auVar1 [16];
  
  DAT_0011b840 = 0xffffffffffffffff;
  DAT_0011b838 = 0xffffffffffffffff;
  if ((DAT_001dd078 & 0xf000) == 0x8000) {
    DAT_0011b840 = DAT_001dd090;
    if ((DAT_00119028 == 0) || (DAT_001dd9e8 != 0)) {
      auVar1 = FUN_0010e940(&DAT_001dd060);
      DAT_0011b830 = auVar1._0_8_;
      DAT_0011b838 = auVar1._8_8_;
      return;
    }
  }
  return;
}




// Function: volatile_strcpy @ 0x5470

void volatile_strcpy(char *param_1,char *param_2)

{
  char cVar1;
  
  do {
    cVar1 = *param_2;
    param_2 = param_2 + 1;
    *param_1 = cVar1;
    param_1 = param_1 + 1;
  } while (cVar1 != '\0');
  return;
}




// Function: help @ 0x5490

void help(void)

{
  undefined **ppuVar1;
  char *pcVar2;
  
  ppuVar1 = &PTR_s_Compress_or_uncompress_FILEs__by_001186e0;
  FUN_0010f7b0("Usage: %s [OPTION]... [FILE]...\n",DAT_0011b850);
  pcVar2 = "Compress or uncompress FILEs (by default, compress FILES in-place).";
  do {
    ppuVar1 = ppuVar1 + 1;
    FUN_0010f7b0("%s\n",pcVar2);
    pcVar2 = *ppuVar1;
  } while (pcVar2 != (char *)0x0);
  return;
}




// Function: license @ 0x54e0

void license(void)

{
  undefined **ppuVar1;
  char *pcVar2;
  
  ppuVar1 = &PTR_s_Copyright__C__2018_Free_Software_00118b20;
  FUN_0010f7b0("%s %s\n",DAT_0011b850,PTR_DAT_001193f8);
  pcVar2 = "Copyright (C) 2018 Free Software Foundation, Inc.";
  do {
    ppuVar1 = ppuVar1 + 1;
    FUN_0010f7b0("%s\n",pcVar2);
    pcVar2 = *ppuVar1;
  } while (pcVar2 != (char *)0x0);
  return;
}




// Function: version @ 0x5540

void version(void)

{
  FUN_001054e0();
  FUN_0010f7b0("\n");
  FUN_0010f7b0("Written by Jean-loup Gailly.\n");
  return;
}




// Function: install_signal_handlers @ 0x5570

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void install_signal_handlers(void)

{
  int iVar1;
  int *piVar2;
  long lVar3;
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
  
  iVar4 = 2;
  piVar2 = (int *)&DAT_00113350;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  sigemptyset((sigset_t *)&DAT_001dd940);
  while( true ) {
    sigaction(iVar4,(sigaction *)0x0,(sigaction *)&local_d8);
    if (local_d8.sa_handler != (__sighandler_t)0x1) {
      sigaddset((sigset_t *)&DAT_001dd940,iVar4);
    }
    piVar2 = piVar2 + 1;
    if (piVar2 == (int *)&DAT_00113368) break;
    iVar4 = *piVar2;
  }
  local_50 = 0;
  local_d8.sa_handler = FUN_001057e0;
  lVar3 = 0;
  local_d0 = _DAT_001dd940;
  uStack_c8 = uRam00000000001dd948;
  iVar4 = 2;
  local_c0 = _DAT_001dd950;
  uStack_b8 = uRam00000000001dd958;
  local_b0 = _DAT_001dd960;
  uStack_a8 = uRam00000000001dd968;
  local_a0 = _DAT_001dd970;
  uStack_98 = uRam00000000001dd978;
  local_90 = _DAT_001dd980;
  uStack_88 = uRam00000000001dd988;
  local_80 = _DAT_001dd990;
  uStack_78 = uRam00000000001dd998;
  local_70 = _DAT_001dd9a0;
  uStack_68 = uRam00000000001dd9a8;
  local_60 = _DAT_001dd9b0;
  uStack_58 = uRam00000000001dd9b8;
  while( true ) {
    iVar1 = sigismember((sigset_t *)&DAT_001dd940,iVar4);
    if (iVar1 != 0) {
      if (lVar3 == 0) {
        DAT_001dd9e0 = 1;
      }
      sigaction(iVar4,(sigaction *)&local_d8,(sigaction *)0x0);
    }
    lVar3 = lVar3 + 1;
    if (lVar3 == 6) break;
    iVar4 = *(int *)(&DAT_00113350 + lVar3 * 4);
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: remove_output_file @ 0x56e0

void remove_output_file(char param_1)

{
  int iVar1;
  char *pcVar2;
  long in_FS_OFFSET;
  sigset_t sStack_498;
  char local_418 [1032];
  long local_10;
  
  iVar1 = DAT_00119018;
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 == '\0') {
    sigprocmask(0,(sigset_t *)&DAT_001dd940,&sStack_498);
    iVar1 = DAT_00119018;
    if (DAT_00119018 < 0) {
      sigprocmask(2,&sStack_498,(sigset_t *)0x0);
    }
    else {
      DAT_00119018 = -1;
      close(iVar1);
      pcVar2 = local_418;
      FUN_00105470(pcVar2,&DAT_001dd520);
      unlink(pcVar2);
      sigprocmask(2,&sStack_498,(sigset_t *)0x0);
    }
  }
  else if (-1 < DAT_00119018) {
    DAT_00119018 = -1;
    close(iVar1);
    pcVar2 = local_418;
    FUN_00105470(pcVar2,&DAT_001dd520);
    unlink(pcVar2);
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: abort_gzip_signal @ 0x57e0

void abort_gzip_signal(int param_1)

{
  FUN_001056e0(1);
  if (DAT_001dd920 != param_1) {
    signal(param_1,(__sighandler_t)0x0);
    raise(param_1);
    return;
  }
                    /* WARNING: Subroutine does not return */
  _exit(2);
}




// Function: progerror @ 0x5820

void progerror(char *param_1)

{
  int iVar1;
  int *piVar2;
  
  piVar2 = __errno_location();
  iVar1 = *piVar2;
  FUN_0010f630(stderr,&DAT_001137cc,DAT_0011b850);
  *piVar2 = iVar1;
  perror(param_1);
  DAT_0011b84c = 1;
  return;
}




// Function: check_ofname @ 0x5870

undefined4 check_ofname(void)

{
  char cVar1;
  int iVar2;
  
  if (DAT_001dd9f4 == 0) {
    FUN_0010f630(stderr,"%s: %s already exists;",DAT_0011b850,&DAT_0011b020);
    if (DAT_001dd9e0 != 0) {
      if (DAT_001dda01 == '\0') {
        iVar2 = isatty(0);
        if (iVar2 == 0) goto LAB_001058fb;
      }
      FUN_0010f630(stderr," do you wish to overwrite (y or n)? ");
      FUN_0010e460(stderr);
      cVar1 = FUN_0010f5f0();
      if (cVar1 != '\0') goto LAB_001058de;
    }
LAB_001058fb:
    FUN_0010f630(stderr,"\tnot overwritten\n");
    if (DAT_0011b84c == 0) {
      DAT_0011b84c = 2;
      return 1;
    }
  }
  else {
LAB_001058de:
    iVar2 = unlink(&DAT_0011b020);
    if (iVar2 == 0) {
      return 0;
    }
    FUN_00105820(&DAT_0011b020);
  }
  return 1;
}




// Function: input_eof @ 0x5940

undefined4 input_eof(void)

{
  int iVar1;
  
  if ((DAT_001dd9f8 != 0) && (DAT_001dd9dc == 0)) {
    if (DAT_0011b008 != DAT_0011b00c) {
      return 0;
    }
    if ((DAT_0011b00c == 0x40000) && (iVar1 = FUN_0010d830(1), iVar1 != -1)) {
      DAT_0011b008 = 0;
      return 0;
    }
  }
  return 1;
}




// Function: atdir_eq @ 0x59a0

bool atdir_eq(undefined *param_1,size_t param_2)

{
  int iVar1;
  bool bVar2;
  
  if (param_2 == 0) {
    param_2 = 1;
    param_1 = &DAT_00113310;
  }
  iVar1 = memcmp(&DAT_001dd100,param_1,param_2);
  bVar2 = false;
  if (iVar1 == 0) {
    bVar2 = (&DAT_001dd100)[param_2] == '\0';
  }
  return bVar2;
}




// Function: atdir_set @ 0x5a00

void atdir_set(undefined *param_1,long param_2)

{
  char cVar1;
  long lVar2;
  
  cVar1 = FUN_001059a0();
  if (cVar1 == '\0') {
    if (-1 < DAT_00119014) {
      close(DAT_00119014);
    }
    if (param_2 == 0) {
      param_2 = 1;
      param_1 = &DAT_00113310;
    }
    lVar2 = __memcpy_chk(&DAT_001dd100,param_1,param_2,0x400);
    *(undefined1 *)(lVar2 + param_2) = 0;
    DAT_00119014 = FUN_0010e3c0(lVar2,0x10000);
  }
  return;
}




// Function: discard_input_bytes @ 0x5a90

void discard_input_bytes(long param_1,uint param_2)

{
  ulong uVar1;
  long in_FS_OFFSET;
  char local_21;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
joined_r0x00105aba:
  if (param_1 == 0) {
LAB_00105af0:
    if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    return;
  }
  if (DAT_0011b00c <= DAT_0011b008) goto LAB_00105b25;
  do {
    uVar1 = (ulong)DAT_0011b008;
    DAT_0011b008 = DAT_0011b008 + 1;
    local_21 = (&DAT_0019d000)[uVar1];
    while( true ) {
      if ((param_2 & 2) != 0) {
        FUN_0010d1f0(&local_21,1);
      }
      if (param_1 != -1) {
        param_1 = param_1 + -1;
        goto joined_r0x00105aba;
      }
      if (local_21 == '\0') goto LAB_00105af0;
      if (DAT_0011b008 < DAT_0011b00c) break;
LAB_00105b25:
      local_21 = FUN_0010d830(0);
    }
  } while( true );
}




// Function: get_method @ 0x5b50

uint get_method(undefined4 param_1)

{
  undefined2 uVar1;
  char cVar2;
  byte bVar3;
  undefined1 uVar4;
  ushort uVar5;
  int iVar6;
  uint uVar7;
  uint uVar8;
  uint uVar9;
  int iVar10;
  char *__dest;
  ulong uVar11;
  size_t sVar12;
  char *pcVar13;
  ulong uVar14;
  long in_FS_OFFSET;
  undefined1 local_4c;
  undefined1 local_4b;
  char local_4a;
  undefined1 local_49;
  undefined1 local_48;
  byte local_47;
  undefined4 local_46;
  undefined1 local_42;
  undefined1 local_41;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar14 = (ulong)DAT_0011b008;
  if ((DAT_001dd9f4 == 0) || (DAT_0011b860 == 0)) {
    if (DAT_0011b008 < DAT_0011b00c) {
      DAT_0011b008 = DAT_0011b008 + 1;
      local_4a = (&DAT_0019d000)[uVar14];
    }
    else {
      local_4a = FUN_0010d830(0);
    }
    uVar14 = (ulong)DAT_0011b008;
    if (local_4a == '\0') {
      if (DAT_0011b008 < DAT_0011b00c) {
        DAT_0011b008 = DAT_0011b008 + 1;
        uVar8 = (uint)(byte)(&DAT_0019d000)[uVar14];
      }
      else {
        uVar8 = FUN_0010d830(1);
      }
      uVar9 = 0;
      uVar7 = uVar8;
    }
    else {
      if (DAT_0011b008 < DAT_0011b00c) {
        DAT_0011b008 = DAT_0011b008 + 1;
        uVar7 = (uint)(byte)(&DAT_0019d000)[uVar14];
      }
      else {
        uVar7 = FUN_0010d830(0);
      }
      uVar8 = 0;
      uVar9 = 0;
    }
  }
  else {
    if (DAT_0011b008 < DAT_0011b00c) {
      DAT_0011b008 = DAT_0011b008 + 1;
      uVar9 = (uint)(byte)(&DAT_0019d000)[uVar14];
    }
    else {
      uVar9 = FUN_0010d830(1);
    }
    uVar14 = (ulong)DAT_0011b008;
    local_4a = (char)uVar9;
    if (DAT_0011b008 < DAT_0011b00c) {
      DAT_0011b008 = DAT_0011b008 + 1;
      uVar8 = (uint)(byte)(&DAT_0019d000)[uVar14];
      uVar7 = (uint)(byte)(&DAT_0019d000)[uVar14];
    }
    else {
      uVar8 = FUN_0010d830(1);
      uVar7 = uVar8;
    }
  }
  local_49 = (undefined1)uVar7;
  DAT_00119020 = 0xffffffff;
  DAT_001e0ea8 = 0;
  iVar10 = DAT_001dd9d8 + 1;
  DAT_001dd9dc = 0;
  DAT_001dd9d8 = iVar10;
  iVar6 = memcmp(&local_4a,&DAT_00112cb4,2);
  if ((iVar6 == 0) || (iVar6 = memcmp(&local_4a,&DAT_00112cb7,2), iVar6 == 0)) {
    uVar14 = (ulong)DAT_0011b008;
    if (DAT_0011b008 < DAT_0011b00c) {
      DAT_0011b008 = DAT_0011b008 + 1;
      DAT_00119020 = (uint)(byte)(&DAT_0019d000)[uVar14];
    }
    else {
      DAT_00119020 = FUN_0010d830(0);
    }
    if (DAT_00119020 != 8) {
      FUN_0010f630(stderr,"%s: %s: unknown method %d -- not supported\n",DAT_0011b850,&DAT_0011b420,
                   DAT_00119020);
      uVar7 = 0xffffffff;
      DAT_0011b84c = 1;
      goto LAB_00105ce0;
    }
    PTR_FUN_001190d0 = FUN_0010cd20;
    uVar14 = (ulong)DAT_0011b008;
    if (DAT_0011b008 < DAT_0011b00c) {
      DAT_0011b008 = DAT_0011b008 + 1;
      bVar3 = (&DAT_0019d000)[uVar14];
    }
    else {
      bVar3 = FUN_0010d830(0);
    }
    if ((bVar3 & 0x20) == 0) {
      if ((bVar3 & 0xc0) == 0) {
        if (DAT_0011b00c <= DAT_0011b008) goto LAB_00106162;
LAB_00105e0f:
        uVar14 = (ulong)DAT_0011b008;
        DAT_0011b008 = DAT_0011b008 + 1;
        uVar14 = (ulong)(byte)(&DAT_0019d000)[uVar14];
        if (DAT_0011b00c <= DAT_0011b008) goto LAB_00106180;
LAB_00105e2c:
        uVar11 = (ulong)DAT_0011b008;
        DAT_0011b008 = DAT_0011b008 + 1;
        uVar14 = uVar14 | (ulong)(byte)(&DAT_0019d000)[uVar11] << 8;
        if (DAT_0011b00c <= DAT_0011b008) goto LAB_001061a4;
LAB_00105e4f:
        uVar11 = (ulong)DAT_0011b008;
        DAT_0011b008 = DAT_0011b008 + 1;
        uVar14 = uVar14 | (ulong)(byte)(&DAT_0019d000)[uVar11] << 0x10;
        if (DAT_0011b00c <= DAT_0011b008) goto LAB_001061c9;
LAB_00105e73:
        uVar11 = (ulong)DAT_0011b008;
        DAT_0011b008 = DAT_0011b008 + 1;
        uVar11 = (ulong)(byte)(&DAT_0019d000)[uVar11];
      }
      else {
        FUN_0010f630(stderr,"%s: %s has flags 0x%x -- not supported\n",DAT_0011b850,&DAT_0011b420,
                     bVar3);
        DAT_0011b84c = 1;
        if (DAT_001dd9f4 < 2) goto LAB_001066c4;
        if (DAT_0011b008 < DAT_0011b00c) goto LAB_00105e0f;
LAB_00106162:
        iVar10 = FUN_0010d830(0);
        uVar14 = (ulong)iVar10;
        if (DAT_0011b008 < DAT_0011b00c) goto LAB_00105e2c;
LAB_00106180:
        iVar10 = FUN_0010d830(0);
        uVar14 = uVar14 | (long)iVar10 << 8;
        if (DAT_0011b008 < DAT_0011b00c) goto LAB_00105e4f;
LAB_001061a4:
        iVar10 = FUN_0010d830(0);
        uVar14 = uVar14 | (long)iVar10 << 0x10;
        if (DAT_0011b008 < DAT_0011b00c) goto LAB_00105e73;
LAB_001061c9:
        iVar10 = FUN_0010d830(0);
        uVar11 = (ulong)iVar10;
      }
      uVar14 = uVar14 | uVar11 << 0x18;
      if ((uVar14 != 0) && (DAT_00119028 == 0)) {
        if ((long)uVar14 < 0) {
          if (DAT_0011b85c == 0) {
            FUN_0010f630(stderr,"%s: %s: MTIME %lu out of range for this platform\n",DAT_0011b850,
                         &DAT_0011b420,uVar14);
          }
          if (DAT_0011b84c == 0) {
            DAT_0011b84c = 2;
          }
          DAT_0011b838 = 999999999;
          DAT_0011b830 = 0x7fffffffffffffff;
        }
        else {
          DAT_0011b838 = 0;
          DAT_0011b830 = uVar14;
        }
      }
      uVar11 = (ulong)DAT_0011b008;
      if (DAT_0011b008 < DAT_0011b00c) {
        DAT_0011b008 = DAT_0011b008 + 1;
        local_42 = (&DAT_0019d000)[uVar11];
      }
      else {
        local_42 = FUN_0010d830(0);
      }
      uVar11 = (ulong)DAT_0011b008;
      if (DAT_0011b008 < DAT_0011b00c) {
        DAT_0011b008 = DAT_0011b008 + 1;
        local_41 = (&DAT_0019d000)[uVar11];
      }
      else {
        local_41 = FUN_0010d830(0);
      }
      if ((bVar3 & 2) != 0) {
        local_48 = 8;
        local_46 = (undefined4)uVar14;
        local_47 = bVar3;
        FUN_0010d1f0(0,0);
        FUN_0010d1f0(&local_4a,10);
      }
      if ((bVar3 & 4) != 0) {
        uVar14 = (ulong)DAT_0011b008;
        if (DAT_0011b008 < DAT_0011b00c) {
          DAT_0011b008 = DAT_0011b008 + 1;
          uVar4 = (&DAT_0019d000)[uVar14];
        }
        else {
          uVar4 = FUN_0010d830(0);
        }
        uVar14 = (ulong)DAT_0011b008;
        local_4c = uVar4;
        if (DAT_0011b008 < DAT_0011b00c) {
          DAT_0011b008 = DAT_0011b008 + 1;
          local_4b = (&DAT_0019d000)[uVar14];
        }
        else {
          local_4b = FUN_0010d830(0);
        }
        uVar1 = CONCAT11(local_4b,uVar4);
        if (DAT_001dd9e4 != 0) {
          FUN_0010f630(stderr,"%s: %s: extra field of %u bytes ignored\n",DAT_0011b850,&DAT_0011b420
                       ,uVar1);
        }
        if ((bVar3 & 2) != 0) {
          FUN_0010d1f0(&local_4c,2);
        }
        FUN_00105a90(uVar1,bVar3);
      }
      if ((bVar3 & 8) != 0) {
        if ((DAT_0011902c == 0) &&
           (((DAT_0011b860 == 0 || (DAT_001dd9e8 != 0)) && (DAT_001dd9d8 < 2)))) {
          __dest = (char *)thunk_FUN_0010e060(&DAT_0011b020);
          pcVar13 = __dest;
          while( true ) {
            uVar14 = (ulong)DAT_0011b008;
            if (DAT_0011b008 < DAT_0011b00c) {
              DAT_0011b008 = DAT_0011b008 + 1;
              cVar2 = (&DAT_0019d000)[uVar14];
            }
            else {
              cVar2 = FUN_0010d830(0);
            }
            *pcVar13 = cVar2;
            pcVar13 = pcVar13 + 1;
            if (cVar2 == '\0') break;
            if ((char *)0x11b41f < pcVar13) {
                    /* WARNING: Subroutine does not return */
              FUN_0010d510("corrupted input -- file name too large");
            }
          }
          if ((bVar3 & 2) != 0) {
            FUN_0010d1f0(__dest,(int)pcVar13 - (int)__dest);
          }
          pcVar13 = (char *)thunk_FUN_0010e060(__dest);
          sVar12 = strlen(pcVar13);
          memmove(__dest,pcVar13,sVar12 + 1);
        }
        else {
          FUN_00105a90(0xffffffffffffffff,bVar3);
        }
      }
      if ((bVar3 & 0x10) != 0) {
        FUN_00105a90(0xffffffffffffffff,bVar3);
      }
      if ((bVar3 & 2) != 0) {
        uVar5 = FUN_0010d1f0(&local_4a,0);
        uVar14 = (ulong)DAT_0011b008;
        if (DAT_0011b008 < DAT_0011b00c) {
          DAT_0011b008 = DAT_0011b008 + 1;
          uVar7 = (uint)(byte)(&DAT_0019d000)[uVar14];
        }
        else {
          uVar7 = FUN_0010d830(0);
        }
        uVar14 = (ulong)DAT_0011b008;
        if (DAT_0011b008 < DAT_0011b00c) {
          DAT_0011b008 = DAT_0011b008 + 1;
          uVar9 = (uint)(byte)(&DAT_0019d000)[uVar14];
        }
        else {
          uVar9 = FUN_0010d830(0);
        }
        uVar7 = uVar9 << 8 | uVar7;
        if (uVar5 != uVar7) {
          FUN_0010f630(stderr,"%s: %s: header checksum 0x%04x != computed checksum 0x%04x\n",
                       DAT_0011b850,&DAT_0011b420,uVar7,(uint)uVar5);
          DAT_0011b84c = 1;
          if (DAT_001dd9f4 < 2) goto LAB_001066c4;
        }
      }
      uVar7 = DAT_00119020;
      if (DAT_001dd9d8 != 1) {
        if (-1 < (int)DAT_00119020) goto LAB_00105ce0;
        goto LAB_00106060;
      }
      DAT_001e0ea8 = (ulong)(DAT_0011b008 + 8);
      if (-1 < (int)DAT_00119020) goto LAB_00105ce0;
      goto LAB_00106697;
    }
    pcVar13 = "%s: %s is encrypted -- not supported\n";
LAB_001066ac:
    FUN_0010f630(stderr,pcVar13,DAT_0011b850,&DAT_0011b420);
    DAT_0011b84c = 1;
  }
  else {
    iVar6 = memcmp(&local_4a,&DAT_00112cd7,2);
    if (((iVar6 != 0) || (DAT_0011b008 != 2)) ||
       (iVar6 = memcmp(&DAT_0019d000,&DAT_00112cd7,4), iVar6 != 0)) {
      iVar6 = memcmp(&local_4a,&DAT_00112cdc,2);
      if (iVar6 == 0) {
        DAT_00119020 = 2;
        PTR_FUN_001190d0 = FUN_0010c9f0;
        uVar7 = 2;
        goto LAB_00105ce0;
      }
      iVar6 = memcmp(&local_4a,&DAT_00112cdf,2);
      if (iVar6 == 0) {
        DAT_00119020 = 1;
        DAT_001dd9dc = 1;
        PTR_FUN_001190d0 = FUN_0010c040;
        uVar7 = 1;
        goto LAB_00105ce0;
      }
      iVar6 = memcmp(&local_4a,&DAT_00112ce2,2);
      if (iVar6 == 0) {
        DAT_00119020 = 3;
        DAT_001dd9dc = 1;
        PTR_FUN_001190d0 = FUN_0010bfd0;
        uVar7 = 3;
        goto LAB_00105ce0;
      }
      if (((DAT_001dd9f4 != 0) && (DAT_0011b860 != 0)) && (DAT_001dd9e8 == 0)) {
        DAT_00119020 = 0;
        PTR_FUN_001190d0 = FUN_0010d720;
        if (uVar8 != 0xffffffff) {
          DAT_0011b008 = DAT_0011b008 - 1;
        }
        DAT_001dd9dc = 1;
        uVar7 = 0;
        if (uVar9 == 0xffffffff) goto LAB_00105ce0;
        FUN_0010d6c0(1,&local_4a,1);
        goto LAB_0010667a;
      }
LAB_0010668e:
      if (iVar10 != 1) {
LAB_00106060:
        if (local_4a == '\0') {
          while (uVar8 == 0) {
            uVar14 = (ulong)DAT_0011b008;
            if (DAT_0011b008 < DAT_0011b00c) {
              DAT_0011b008 = DAT_0011b008 + 1;
              uVar8 = (uint)(byte)(&DAT_0019d000)[uVar14];
            }
            else {
              uVar8 = FUN_0010d830(1);
            }
          }
          if (uVar8 == 0xffffffff) {
            if (DAT_001dd9e4 != 0) {
              if (DAT_0011b85c == 0) {
                FUN_0010f630(stderr,"\n%s: %s: decompression OK, trailing zero bytes ignored\n",
                             DAT_0011b850,&DAT_0011b420);
              }
              if (DAT_0011b84c == 0) {
                DAT_0011b84c = 2;
              }
            }
            uVar7 = 0xfffffffd;
            goto LAB_00105ce0;
          }
        }
        if (DAT_0011b85c == 0) {
          FUN_0010f630(stderr,"\n%s: %s: decompression OK, trailing garbage ignored\n",DAT_0011b850,
                       &DAT_0011b420);
        }
        uVar7 = 0xfffffffe;
        if (DAT_0011b84c == 0) {
          DAT_0011b84c = 2;
        }
        goto LAB_00105ce0;
      }
LAB_00106697:
      pcVar13 = "\n%s: %s: not in gzip format\n";
      goto LAB_001066ac;
    }
    DAT_0011b008 = 0;
    PTR_FUN_001190d0 = FUN_0010cd20;
    iVar10 = FUN_0010cc20(param_1);
    if (iVar10 == 0) {
      DAT_001dd9dc = 1;
LAB_0010667a:
      iVar10 = DAT_001dd9d8;
      uVar7 = DAT_00119020;
      if (-1 < (int)DAT_00119020) goto LAB_00105ce0;
      goto LAB_0010668e;
    }
  }
LAB_001066c4:
  uVar7 = 0xffffffff;
LAB_00105ce0:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar7;
}




// Function: get_suffix @ 0x67e0

char * get_suffix(char *param_1)

{
  ulong uVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  size_t sVar6;
  void *__ptr;
  long lVar7;
  undefined **ppuVar8;
  char *pcVar9;
  char *pcVar10;
  long in_FS_OFFSET;
  char local_68 [40];
  long local_40;
  
  pcVar9 = DAT_001dd9c8;
  uVar1 = DAT_001dd9c0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (PTR_DAT_00119048 != (undefined *)0x0) {
    ppuVar8 = &PTR_DAT_00119048;
    pcVar10 = PTR_DAT_00119048;
    do {
      sVar6 = strlen(pcVar10);
      if (uVar1 < sVar6) {
        iVar2 = strcmp(pcVar9,pcVar10 + (sVar6 - uVar1));
        if (iVar2 == 0) {
          ppuVar8 = &PTR_DAT_00119048;
          __ptr = (void *)FUN_0010f5d0(pcVar9);
          FUN_0010d350(__ptr);
          lVar7 = 8;
          goto LAB_00106879;
        }
      }
      pcVar10 = ppuVar8[1];
      ppuVar8 = ppuVar8 + 1;
    } while (pcVar10 != (char *)0x0);
  }
  __ptr = (void *)FUN_0010f5d0(pcVar9);
  FUN_0010d350(__ptr);
  lVar7 = 0;
  ppuVar8 = (undefined **)&DAT_00119040;
LAB_00106879:
  (&DAT_00119040)[lVar7] = __ptr;
  sVar6 = strlen(param_1);
  iVar2 = (int)sVar6;
  if (iVar2 < 0x21) {
    __strcpy_chk(local_68,param_1,0x21);
  }
  else {
    __strcpy_chk(local_68,param_1 + (long)iVar2 + -0x20,0x21);
  }
  FUN_0010d350(local_68);
  sVar6 = strlen(local_68);
  pcVar9 = *ppuVar8;
  iVar3 = (int)sVar6;
  do {
    sVar6 = strlen(pcVar9);
    iVar4 = (int)sVar6;
    if ((iVar4 < iVar3) && (local_68[(iVar3 - iVar4) + -1] != '/')) {
      iVar5 = strcmp(local_68 + ((long)iVar3 - (long)iVar4),pcVar9);
      if (iVar5 == 0) {
        pcVar10 = param_1 + ((long)iVar2 - (long)iVar4);
        break;
      }
    }
    pcVar9 = ppuVar8[1];
    ppuVar8 = ppuVar8 + 1;
    pcVar10 = (char *)0x0;
  } while (pcVar9 != (char *)0x0);
  free(__ptr);
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return pcVar10;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: make_ofname @ 0x69d0

undefined4 make_ofname(void)

{
  int iVar1;
  char *__s1;
  size_t sVar2;
  
  __strcpy_chk(&DAT_0011b020,&DAT_0011b420,0x400);
  __s1 = (char *)FUN_001067e0(&DAT_0011b020);
  if (DAT_001dd9f8 == 0) {
    if ((__s1 != (char *)0x0) && (DAT_001dd9f4 == 0)) {
      if ((DAT_001dd9e4 == 0) && (DAT_001dd9ec != 0 || DAT_0011b85c != 0)) {
        return 2;
      }
      FUN_0010f630(stderr,"%s: %s already has %s suffix -- unchanged\n",DAT_0011b850,&DAT_0011b420,
                   __s1);
      return 2;
    }
    DAT_0011b848 = 0;
    sVar2 = strlen(&DAT_0011b020);
    if (sVar2 + DAT_001dd9c0 < 0x400) {
      __strcat_chk(&DAT_0011b020,DAT_001dd9c8,0x400);
      return 0;
    }
    if (DAT_0011b85c == 0) {
      FUN_0010f630(stderr,"%s: %s: file name too long\n",DAT_0011b850,&DAT_0011b420);
    }
    goto LAB_00106aa0;
  }
  if (__s1 != (char *)0x0) {
    FUN_0010d350(__s1);
    iVar1 = strcmp(__s1,".tgz");
    if ((iVar1 != 0) && (iVar1 = strcmp(__s1,".taz"), iVar1 != 0)) {
      *__s1 = '\0';
      return 0;
    }
    builtin_strncpy(__s1,".tar",5);
    return 0;
  }
  if (DAT_001dd9ec == 0) {
    if (DAT_0011b858 != 0) {
      return 0;
    }
    if (DAT_001dd9e4 != 0) goto LAB_00106a8a;
    if (DAT_0011b85c != 0) {
      return 2;
    }
  }
  else {
    if (DAT_001dd9e4 == 0) {
      return 2;
    }
LAB_00106a8a:
    if (DAT_0011b85c != 0) goto LAB_00106aa0;
  }
  FUN_0010f630(stderr,"%s: %s: unknown suffix -- ignored\n",DAT_0011b850,&DAT_0011b420);
LAB_00106aa0:
  if (DAT_0011b84c != 0) {
    return 2;
  }
  DAT_0011b84c = 2;
  return 2;
}




// Function: do_list @ 0x6c40

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void do_list(int param_1)

{
  tm *ptVar1;
  long lVar2;
  undefined8 uVar3;
  
  if (DAT_00119010 == 0) {
    if (param_1 < 0) goto LAB_00106d88;
  }
  else {
    if (param_1 < 0) {
LAB_00106d88:
      if ((0 < DAT_001dd508) && (0 < DAT_001dd500)) {
        if (((DAT_001dd9e4 != 0) && (FUN_0010f7b0(&DAT_00112d35), DAT_001dd9e4 != 0)) ||
           (DAT_0011b85c == 0)) {
          FUN_0010d930(stdout,DAT_001dd508,0x13);
          FUN_0010f7b0(&DAT_00112d50);
          FUN_0010d930(stdout,DAT_001dd500,0x13);
          FUN_0010f7b0(&DAT_00112d50);
        }
        FUN_0010d8e0(DAT_001dd500 - (DAT_001dd508 - DAT_001e0ea8),DAT_001dd500,stdout);
        FUN_0010f7b0(" (totals)\n");
        return;
      }
      return;
    }
    DAT_00119010 = 0;
    if (DAT_001dd9e4 != 0) {
      FUN_0010f7b0("method  crc     date  time  ");
    }
    if (DAT_0011b85c == 0) {
      FUN_0010f7b0("%*.*s %*.*s  ratio uncompressed_name\n",0x13,0x13,"compressed",0x13,0x13,
                   "uncompressed");
    }
  }
  uVar3 = 0xffffffffffffffff;
  if ((param_1 == 8) && (DAT_001dd9dc == 0)) {
    uVar3 = DAT_001e0e90;
  }
  if (DAT_001dd9e4 != 0) {
    ptVar1 = localtime(&DAT_0011b830);
    FUN_0010f7b0("%5s %08lx ",(&PTR_s_store_00118680)[param_1],uVar3);
    if (ptVar1 == (tm *)0x0) {
      FUN_0010f7b0("??? ?? ??:?? ");
    }
    else {
      FUN_0010f7b0("%s%3d %02d:%02d ",&DAT_001132e0 + (long)ptVar1->tm_mon * 4,ptVar1->tm_mday,
                   ptVar1->tm_hour,ptVar1->tm_min);
    }
  }
  FUN_0010d930(stdout,DAT_0011b828,0x13);
  FUN_0010f7b0(&DAT_00112d50);
  FUN_0010d930(stdout,_DAT_0011b820,0x13);
  FUN_0010f7b0(&DAT_00112d50);
  if (DAT_0011b828 == -1) {
    DAT_001dd508 = -1;
    DAT_001e0ea8 = 0;
    _DAT_0011b820 = 0;
    DAT_0011b828 = 0;
  }
  else {
    if (-1 < DAT_001dd508) {
      DAT_001dd508 = DAT_001dd508 + DAT_0011b828;
    }
    if (_DAT_0011b820 == -1) {
      DAT_001dd500 = -1;
      lVar2 = 0;
      DAT_001e0ea8 = 0;
      _DAT_0011b820 = 0;
      DAT_0011b828 = 0;
      goto LAB_00106d5b;
    }
  }
  if (-1 < DAT_001dd500) {
    DAT_001dd500 = DAT_001dd500 + _DAT_0011b820;
  }
  lVar2 = _DAT_0011b820 - (DAT_0011b828 - DAT_001e0ea8);
LAB_00106d5b:
  FUN_0010d8e0(lVar2,_DAT_0011b820,stdout);
  FUN_0010f7b0(" %s\n",&DAT_0011b020);
  return;
}




// Function: do_exit @ 0x6fd0

void do_exit(int param_1)

{
  if (DAT_001dd040 == 0) {
    DAT_001dd040 = 1;
    free(DAT_001dd9d0);
    DAT_001dd9d0 = (void *)0x0;
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: finish_out @ 0x7010

void finish_out(void)

{
  int iVar1;
  
  iVar1 = FUN_0010e0f0(stdout);
  if (iVar1 != 0) {
    FUN_0010d670();
  }
                    /* WARNING: Subroutine does not return */
  FUN_00106fd0(0);
}




// Function: try_help @ 0x7040

void try_help(void)

{
  FUN_0010f630(stderr,"Try `%s --help\' for more information.\n",DAT_0011b850);
                    /* WARNING: Subroutine does not return */
  FUN_00106fd0(1);
}




// Function: treat_stdin @ 0x7070

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void treat_stdin(void)

{
  int iVar1;
  char *pcVar2;
  undefined *puVar3;
  
  if ((DAT_001dd9f4 == 0 && DAT_001dd9e8 == 0) &&
     ((DAT_001dda01 != '\0' || (iVar1 = isatty((uint)(DAT_001dd9f8 == 0)), iVar1 != 0)))) {
    if (DAT_0011b85c == 0) {
      pcVar2 = "read from";
      if (DAT_001dd9f8 == 0) {
        pcVar2 = "written to";
      }
      puVar3 = &DAT_00112d87;
      if (DAT_001dd9f8 == 0) {
        puVar3 = &DAT_00112cdb;
      }
      FUN_0010f630(stderr,
                   "%s: compressed data not %s a terminal. Use -f to force %scompression.\nFor help, type: %s -h\n"
                   ,DAT_0011b850,pcVar2,puVar3,DAT_0011b850);
    }
  }
  else {
    _DAT_0011b420 = 0x69647473;
    _DAT_0011b424 = 0x6e;
    _DAT_0011b020 = 0x6f647473;
    _DAT_0011b024 = 0x7475;
    DAT_0011b026 = 0;
    iVar1 = fstat(0,(stat *)&DAT_001dd060);
    if (iVar1 == 0) {
      FUN_001053f0();
      FUN_0010d270();
      DAT_0011b860 = 1;
      DAT_001dd9d8 = 0;
      DAT_0011b014 = 0;
      DAT_001dd510 = 1;
      if ((DAT_001dd9f8 != 0) && (DAT_00119020 = FUN_00105b50(0), DAT_00119020 < 0)) {
                    /* WARNING: Subroutine does not return */
        FUN_00106fd0(DAT_0011b84c);
      }
      while( true ) {
        iVar1 = (*(code *)PTR_FUN_001190d0)(0,1);
        if (iVar1 != 0) {
          return;
        }
        iVar1 = FUN_00105940();
        if (iVar1 != 0) break;
        DAT_00119020 = FUN_00105b50();
        if (DAT_00119020 < 0) {
          return;
        }
        _DAT_0011b820 = 0;
      }
      if (DAT_001dd9e8 != 0) {
        FUN_00106c40(DAT_00119020);
        return;
      }
      if (DAT_001dd9e4 == 0) {
        return;
      }
      if (DAT_0011b858 != 0) {
        FUN_0010f630(stderr,&DAT_00112dae);
        return;
      }
      if (DAT_001dd9f8 != 0) {
        return;
      }
      FUN_0010d8e0((DAT_0011b828 - _DAT_0011b820) + DAT_001e0ea8,DAT_0011b828,stderr);
      FUN_0010f630(stderr,"\n");
      return;
    }
    FUN_00105820("standard input");
  }
                    /* WARNING: Subroutine does not return */
  FUN_00106fd0(1);
}




// Function: create_outfile @ 0x76e0

undefined4 create_outfile(void)

{
  bool bVar1;
  int iVar2;
  undefined4 uVar3;
  int iVar4;
  int *piVar5;
  undefined *puVar6;
  long in_FS_OFFSET;
  sigset_t local_c8;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_001dd9f0 == 0) {
    puVar6 = (undefined *)FUN_0010e060(&DAT_0011b020);
    iVar4 = FUN_00105a00(&DAT_0011b020,puVar6 + -0x11b020);
    if (-1 < iVar4) goto LAB_00107726;
  }
  iVar4 = -100;
  puVar6 = &DAT_0011b020;
LAB_00107726:
  piVar5 = __errno_location();
  bVar1 = false;
  do {
    while( true ) {
      FUN_00105470(&DAT_001dd520,&DAT_0011b020);
      sigprocmask(0,(sigset_t *)&DAT_001dd940,&local_c8);
      DAT_00119018 = FUN_0010e520(iVar4,puVar6,0xc1,0x180);
      iVar2 = *piVar5;
      DAT_0011b010 = DAT_00119018;
      sigprocmask(2,&local_c8,(sigset_t *)0x0);
      if (-1 < DAT_0011b010) {
        if ((bVar1) && (DAT_001dd9f8 != 0)) {
          if (DAT_0011b85c == 0) {
            FUN_0010f630(stderr,"%s: %s: warning, name truncated\n",DAT_0011b850,&DAT_0011b020);
          }
          if (DAT_0011b84c == 0) {
            DAT_0011b84c = 2;
            uVar3 = 0;
            goto LAB_001077f0;
          }
        }
        uVar3 = 0;
        goto LAB_001077f0;
      }
      if (iVar2 == 0x11) break;
      if (iVar2 != 0x24) {
        FUN_00105820(&DAT_0011b020);
        goto LAB_001077e0;
      }
      FUN_00107550();
      bVar1 = true;
    }
    iVar2 = FUN_00105870();
  } while (iVar2 == 0);
LAB_001077e0:
  close(DAT_0011b014);
  uVar3 = 1;
LAB_001077f0:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar3;
}




// Function: treat_file @ 0x7b60

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void treat_file(char *param_1)

{
  char cVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  char *pcVar5;
  long in_FS_OFFSET;
  sigset_t sStack_b8;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = strcmp(param_1,"-");
  iVar3 = DAT_0011b860;
  if (iVar2 == 0) {
    FUN_00107070();
    DAT_0011b860 = iVar3;
  }
  else {
    DAT_0011b014 = FUN_00107380(param_1);
    if (-1 < DAT_0011b014) {
      if ((DAT_001dd078 & 0xf000) != 0x4000) {
        if (DAT_0011b860 == 0) {
          if ((DAT_001dd078 & 0xf000) == 0x8000) {
            if ((DAT_001dd078 & 0x800) == 0) {
              if ((DAT_001dd078 & 0x400) == 0) {
                if (DAT_001dd9f4 != 0) {
LAB_00107ca8:
                  FUN_001053f0();
                  goto LAB_00107d03;
                }
                if ((DAT_001dd078 & 0x200) == 0) {
                  if (DAT_001dd070 < 2) goto LAB_00107ca8;
                  if (DAT_0011b85c == 0) {
                    pcVar5 = "";
                    if (DAT_001dd070 != 2) {
                      pcVar5 = "s";
                    }
                    FUN_0010f630(stderr,"%s: %s has %lu other link%s -- file ignored\n",DAT_0011b850
                                 ,&DAT_0011b420,DAT_001dd070 - 1,pcVar5);
                  }
                }
                else if (DAT_0011b85c == 0) {
                  FUN_0010f630(stderr,"%s: %s has the sticky bit set - file ignored\n",DAT_0011b850,
                               &DAT_0011b420);
                }
              }
              else if (DAT_0011b85c == 0) {
                FUN_0010f630(stderr,"%s: %s is set-group-ID on execution - ignored\n",DAT_0011b850,
                             &DAT_0011b420);
              }
            }
            else if (DAT_0011b85c == 0) {
              FUN_0010f630(stderr,"%s: %s is set-user-ID on execution - ignored\n",DAT_0011b850,
                           &DAT_0011b420);
            }
          }
          else if (DAT_0011b85c == 0) {
            FUN_0010f630(stderr,"%s: %s is not a directory or a regular file - ignored\n",
                         DAT_0011b850,&DAT_0011b420);
          }
          if (DAT_0011b84c == 0) {
            DAT_0011b84c = 2;
          }
          if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) goto LAB_00107c1c;
          goto LAB_001083e5;
        }
        FUN_001053f0();
        if (DAT_0011b858 == 0) {
          DAT_0011b026 = 0;
          _DAT_0011b020 = 0x6f647473;
          _DAT_0011b024 = 0x7475;
LAB_00107d10:
          FUN_0010d270();
          DAT_001dd9d8 = 0;
          if ((DAT_001dd9f8 == 0) || (DAT_00119020 = FUN_00105b50(DAT_0011b014), -1 < DAT_00119020))
          {
            if (DAT_0011b860 == 0) {
              iVar3 = FUN_001076e0();
              if (iVar3 != 0) goto LAB_00107c41;
              if (DAT_001dd9f8 != 0) goto LAB_00107d4d;
              if (DAT_0011b848 == 0) goto LAB_00107f20;
              if (DAT_001dd9e4 == 0 && DAT_0011b85c == 0) {
                FUN_0010f630(stderr,"%s: %s compressed to %s\n",DAT_0011b850,&DAT_0011b420,
                             &DAT_0011b020);
                goto LAB_00107d4d;
              }
            }
            else {
              DAT_0011b010 = 1;
LAB_00107d4d:
              if (DAT_0011b848 == 0) {
LAB_00107f20:
                DAT_0011b848 = (uint)(DAT_0011902c == 0);
              }
            }
            if ((DAT_001dd9e4 != 0) && (DAT_001dd9e8 == 0)) {
              FUN_0010f630(stderr,&DAT_00112e28,&DAT_0011b420);
            }
            while (iVar3 = (*(code *)PTR_FUN_001190d0)(DAT_0011b014,DAT_0011b010), iVar3 == 0) {
              iVar3 = FUN_00105940();
              if ((iVar3 != 0) || (DAT_00119020 = FUN_00105b50(DAT_0011b014), DAT_00119020 < 0))
              goto LAB_00107dd9;
              _DAT_0011b820 = 0;
            }
            DAT_00119020 = -1;
LAB_00107dd9:
            iVar3 = close(DAT_0011b014);
            if (iVar3 != 0) {
LAB_001083ef:
                    /* WARNING: Subroutine does not return */
              FUN_0010d600();
            }
            if (DAT_001dd9e8 != 0) {
              if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
                FUN_00106c40(DAT_00119020);
                return;
              }
              goto LAB_001083e5;
            }
            if (DAT_0011b860 == 0) {
              FUN_001078e0();
              if (((DAT_001dda00 != '\0') &&
                  ((((-1 < DAT_00119014 && (iVar3 = fdatasync(DAT_00119014), iVar3 != 0)) &&
                    (piVar4 = __errno_location(), *piVar4 != 0x16)) ||
                   ((iVar3 = fsync(DAT_0011b010), iVar3 != 0 &&
                    (piVar4 = __errno_location(), *piVar4 != 0x16)))))) ||
                 (iVar3 = close(DAT_0011b010), iVar3 != 0)) {
                FUN_0010d670();
                goto LAB_001083ef;
              }
              if (DAT_001dd9f0 == 0) {
                pcVar5 = (char *)FUN_0010e060(&DAT_0011b420);
                cVar1 = FUN_001059a0(&DAT_0011b420,pcVar5 + -0x11b420);
                iVar3 = DAT_00119014;
                if (cVar1 == '\0') {
                  sigprocmask(0,(sigset_t *)&DAT_001dd940,&sStack_b8);
LAB_001081b1:
                  DAT_00119018 = 0xffffffff;
                  iVar3 = unlink(&DAT_0011b420);
                }
                else {
                  sigprocmask(0,(sigset_t *)&DAT_001dd940,&sStack_b8);
                  DAT_00119018 = 0xffffffff;
                  if (iVar3 < 0) goto LAB_001081b1;
                  iVar3 = unlinkat(iVar3,pcVar5,0);
                }
                if (iVar3 == 0) {
                  sigprocmask(2,&sStack_b8,(sigset_t *)0x0);
                }
                else {
                  piVar4 = __errno_location();
                  iVar3 = *piVar4;
                  sigprocmask(2,&sStack_b8,(sigset_t *)0x0);
                  if (iVar3 != 0) {
                    if (DAT_0011b85c == 0) {
                      FUN_0010f630(stderr,&DAT_001137cc,DAT_0011b850);
                      if (DAT_0011b84c == 0) goto LAB_0010834d;
                    }
                    else {
                      if (DAT_0011b84c != 0) goto LAB_001081d0;
LAB_0010834d:
                      DAT_0011b84c = 2;
                    }
                    if (DAT_0011b85c == 0) {
                      *piVar4 = iVar3;
                      perror(&DAT_0011b420);
                    }
                  }
                }
              }
LAB_001081d0:
              if (DAT_00119020 == -1) {
                if (DAT_0011b860 == 0) {
                  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
                    FUN_001056e0(0);
                    return;
                  }
                  goto LAB_001083e5;
                }
                goto LAB_00107c41;
              }
            }
            else if (DAT_00119020 == -1) goto LAB_00107c41;
            if (DAT_001dd9e4 != 0) {
              if (DAT_0011b858 == 0) {
                if (DAT_001dd9f8 == 0) {
                  FUN_0010d8e0(DAT_0011b828 - (_DAT_0011b820 - DAT_001e0ea8));
                }
                else {
                  FUN_0010d8e0(_DAT_0011b820 - (DAT_0011b828 - DAT_001e0ea8),_DAT_0011b820);
                }
              }
              else {
                FUN_0010f630(stderr,&DAT_00112e2d);
              }
              if (DAT_0011b858 == 0) {
                pcVar5 = "created";
                if (DAT_001dd9f0 == 0) {
                  pcVar5 = "replaced with";
                }
                FUN_0010f630(stderr," -- %s %s",pcVar5,&DAT_0011b020);
              }
              if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
                FUN_0010f630(stderr,"\n");
                return;
              }
              goto LAB_001083e5;
            }
            goto LAB_00107c41;
          }
        }
        else {
LAB_00107d03:
          iVar3 = FUN_001069d0();
          if (iVar3 == 0) goto LAB_00107d10;
        }
        if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
LAB_00107c1c:
          close(DAT_0011b014);
          return;
        }
        goto LAB_001083e5;
      }
      if (DAT_001dd9ec != 0) {
        if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
          FUN_00108400(DAT_0011b014,param_1);
          return;
        }
        goto LAB_001083e5;
      }
      close(DAT_0011b014);
      if (DAT_0011b85c == 0) {
        FUN_0010f630(stderr,"%s: %s is a directory -- ignored\n",DAT_0011b850,&DAT_0011b420);
      }
      if (DAT_0011b84c == 0) {
        DAT_0011b84c = 2;
      }
    }
  }
LAB_00107c41:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
LAB_001083e5:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: treat_dir @ 0x8400

void treat_dir(int param_1,char *param_2)

{
  char cVar1;
  int iVar2;
  DIR *__dirp;
  char *pcVar3;
  size_t sVar4;
  size_t sVar5;
  char *__s;
  long in_FS_OFFSET;
  undefined8 local_450;
  char local_448 [1032];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  __dirp = fdopendir(param_1);
  if (__dirp == (DIR *)0x0) {
    FUN_00105820(param_2);
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      close(param_1);
      return;
    }
  }
  else {
    local_450 = (char *)FUN_0010e5b0(__dirp,0);
    if (local_450 != (char *)0x0) {
      iVar2 = closedir(__dirp);
      if (iVar2 != 0) {
        FUN_00105820(param_2);
      }
      cVar1 = *local_450;
      __s = local_450;
joined_r0x00108481:
      if (cVar1 != '\0') {
        do {
          sVar4 = strlen(__s);
          iVar2 = strcmp(__s,".");
          if ((iVar2 != 0) && (iVar2 = strcmp(__s,".."), iVar2 != 0)) {
            sVar5 = strlen(param_2);
            if (0x3fd < sVar5 + sVar4) goto code_r0x00108529;
            __strcpy_chk(local_448,param_2,0x400);
            pcVar3 = (char *)FUN_0010e060(local_448);
            if ((*pcVar3 != '\0') && (local_448[sVar5 - 1] != '/')) {
              local_448[sVar5] = '/';
              sVar5 = sVar5 + 1;
            }
            strcpy(local_448 + sVar5,__s);
            FUN_00107b60(local_448);
          }
          __s = __s + sVar4 + 1;
          if (*__s == '\0') break;
        } while( true );
      }
      free(local_450);
      goto LAB_0010856a;
    }
    FUN_00105820(param_2);
    iVar2 = closedir(__dirp);
    if (iVar2 == 0) {
LAB_0010856a:
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
    }
    else if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      FUN_00105820(param_2);
      return;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
code_r0x00108529:
  FUN_0010f630(stderr,"%s: %s/%s: pathname too long\n",DAT_0011b850,param_2,__s);
  cVar1 = __s[sVar4 + 1];
  DAT_0011b84c = 1;
  __s = __s + sVar4 + 1;
  goto joined_r0x00108481;
}




// Function: abort_gzip @ 0x8620

void abort_gzip(void)

{
  FUN_001056e0(0);
                    /* WARNING: Subroutine does not return */
  FUN_00106fd0(1);
}




// Function: inflate_stored @ 0x8640

undefined8 inflate_stored(void)

{
  byte bVar1;
  uint uVar2;
  ulong uVar3;
  uint uVar4;
  uint uVar5;
  undefined8 uVar6;
  uint uVar7;
  ulong uVar8;
  ulong uVar9;
  uint uVar10;
  uint uVar11;
  
  uVar11 = DAT_001dda0c;
  uVar7 = DAT_0011b004;
  uVar4 = DAT_001dda0c & 7;
  uVar2 = DAT_001dda0c - uVar4;
  uVar8 = DAT_001dda10 >> (sbyte)uVar4;
  uVar5 = uVar2;
  if (uVar2 < 0x10) {
    do {
      uVar9 = (ulong)DAT_0011b008;
      if (DAT_0011b008 < DAT_0011b00c) {
        DAT_0011b008 = DAT_0011b008 + 1;
        bVar1 = (&DAT_0019d000)[uVar9];
      }
      else {
        DAT_0011b004 = uVar7;
        bVar1 = FUN_0010d830(0);
      }
      uVar10 = uVar5 + 8;
      uVar8 = uVar8 | (ulong)bVar1 << ((byte)uVar5 & 0x3f);
      uVar5 = uVar10;
    } while (uVar10 < 0x10);
    uVar2 = ((uVar4 - uVar11) + 0xf & 0xfffffff8) + 8 + uVar2;
  }
  uVar5 = (uint)uVar8 & 0xffff;
  uVar11 = uVar2 - 0x10;
  uVar9 = uVar8 >> 0x10;
  if (uVar11 < 0x10) {
    do {
      uVar3 = (ulong)DAT_0011b008;
      if (DAT_0011b008 < DAT_0011b00c) {
        DAT_0011b008 = DAT_0011b008 + 1;
        bVar1 = (&DAT_0019d000)[uVar3];
      }
      else {
        DAT_0011b004 = uVar7;
        bVar1 = FUN_0010d830(0);
      }
      uVar4 = uVar11 + 8;
      uVar9 = uVar9 | (ulong)bVar1 << ((byte)uVar11 & 0x3f);
      uVar11 = uVar4;
    } while (uVar4 < 0x10);
    uVar11 = (uVar2 - 8) + (0x1f - uVar2 & 0xfffffff8);
  }
  uVar6 = 1;
  if ((~(uint)uVar9 & 0xffff) == uVar5) {
    uVar9 = uVar9 >> 0x10;
    uVar11 = uVar11 - 0x10;
    if ((uVar8 & 0xffff) != 0) {
      do {
        uVar5 = uVar5 - 1;
        if (uVar11 < 8) {
          uVar8 = (ulong)DAT_0011b008;
          if (DAT_0011b008 < DAT_0011b00c) {
            DAT_0011b008 = DAT_0011b008 + 1;
            bVar1 = (&DAT_0019d000)[uVar8];
          }
          else {
            DAT_0011b004 = uVar7;
            bVar1 = FUN_0010d830(0);
          }
          uVar9 = uVar9 | (ulong)bVar1 << ((byte)uVar11 & 0x3f);
        }
        else {
          uVar11 = uVar11 - 8;
        }
        uVar8 = (ulong)uVar7;
        uVar7 = uVar7 + 1;
        (&DAT_0013c000)[uVar8] = (char)uVar9;
        if (uVar7 == 0x8000) {
          DAT_0011b004 = 0x8000;
          uVar7 = 0;
          FUN_0010d7e0();
        }
        uVar9 = uVar9 >> 8;
      } while (uVar5 != 0);
    }
    uVar6 = 0;
    DAT_0011b004 = uVar7;
    DAT_001dda0c = uVar11;
    DAT_001dda10 = uVar9;
  }
  return uVar6;
}




// Function: inflate_codes @ 0x8840

undefined8 inflate_codes(long param_1,long param_2,uint param_3,uint param_4)

{
  uint uVar1;
  ushort uVar2;
  ushort uVar3;
  ushort uVar4;
  ushort uVar5;
  byte bVar6;
  byte bVar7;
  uint uVar8;
  ulong uVar9;
  ulong uVar10;
  uint uVar11;
  uint uVar12;
  uint uVar13;
  uint uVar14;
  byte *pbVar15;
  ulong uVar16;
  uint uVar17;
  
  uVar2 = *(ushort *)(&DAT_00113380 + (long)(int)param_3 * 2);
  uVar3 = *(ushort *)(&DAT_00113380 + (long)(int)param_4 * 2);
  uVar16 = DAT_001dda10;
  uVar13 = DAT_001dda0c;
  uVar12 = DAT_0011b004;
joined_r0x001088a5:
  do {
    while( true ) {
      for (; uVar13 < param_3; uVar13 = uVar13 + 8) {
        uVar10 = (ulong)DAT_0011b008;
        if (DAT_0011b008 < DAT_0011b00c) {
          DAT_0011b008 = DAT_0011b008 + 1;
          bVar7 = (&DAT_0019d000)[uVar10];
        }
        else {
          DAT_0011b004 = uVar12;
          bVar7 = FUN_0010d830(0);
        }
        uVar16 = uVar16 | (ulong)bVar7 << ((byte)uVar13 & 0x3f);
      }
      pbVar15 = (byte *)(param_1 + (ulong)((uint)uVar2 & (uint)uVar16) * 0x10);
      bVar7 = *pbVar15;
      while (0x10 < bVar7) {
        if (bVar7 == 99) {
          return 1;
        }
        uVar11 = bVar7 - 0x10;
        uVar16 = uVar16 >> (pbVar15[1] & 0x3f);
        for (uVar13 = uVar13 - pbVar15[1]; uVar13 < uVar11; uVar13 = uVar13 + 8) {
          uVar10 = (ulong)DAT_0011b008;
          if (DAT_0011b008 < DAT_0011b00c) {
            DAT_0011b008 = DAT_0011b008 + 1;
            bVar7 = (&DAT_0019d000)[uVar10];
          }
          else {
            DAT_0011b004 = uVar12;
            bVar7 = FUN_0010d830(0);
          }
          uVar16 = uVar16 | (ulong)bVar7 << ((byte)uVar13 & 0x3f);
        }
        pbVar15 = (byte *)((ulong)((uint)*(ushort *)(&DAT_00113380 + (ulong)uVar11 * 2) &
                                  (uint)uVar16) * 0x10 + *(long *)(pbVar15 + 8));
        bVar7 = *pbVar15;
      }
      uVar13 = uVar13 - pbVar15[1];
      uVar16 = uVar16 >> (pbVar15[1] & 0x3f);
      uVar11 = (uint)bVar7;
      if (uVar11 != 0x10) break;
      uVar10 = (ulong)uVar12;
      uVar12 = uVar12 + 1;
      (&DAT_0013c000)[uVar10] = (char)*(undefined2 *)(pbVar15 + 8);
      if (uVar12 == 0x8000) {
        DAT_0011b004 = 0x8000;
        FUN_0010d7e0();
        uVar12 = 0;
      }
    }
    if (uVar11 == 0xf) {
      DAT_0011b004 = uVar12;
      DAT_001dda0c = uVar13;
      DAT_001dda10 = uVar16;
      return 0;
    }
    for (; uVar13 < uVar11; uVar13 = uVar13 + 8) {
      uVar10 = (ulong)DAT_0011b008;
      if (DAT_0011b008 < DAT_0011b00c) {
        DAT_0011b008 = DAT_0011b008 + 1;
        bVar6 = (&DAT_0019d000)[uVar10];
      }
      else {
        DAT_0011b004 = uVar12;
        bVar6 = FUN_0010d830(0);
      }
      uVar16 = uVar16 | (ulong)bVar6 << ((byte)uVar13 & 0x3f);
    }
    uVar4 = *(ushort *)(pbVar15 + 8);
    uVar10 = uVar16 >> (bVar7 & 0x3f);
    uVar5 = *(ushort *)(&DAT_00113380 + (ulong)bVar7 * 2);
    for (uVar13 = uVar13 - uVar11; uVar13 < param_4; uVar13 = uVar13 + 8) {
      uVar9 = (ulong)DAT_0011b008;
      if (DAT_0011b008 < DAT_0011b00c) {
        DAT_0011b008 = DAT_0011b008 + 1;
        bVar7 = (&DAT_0019d000)[uVar9];
      }
      else {
        DAT_0011b004 = uVar12;
        bVar7 = FUN_0010d830(0);
      }
      uVar10 = uVar10 | (ulong)bVar7 << ((byte)uVar13 & 0x3f);
    }
    pbVar15 = (byte *)((ulong)((uint)uVar3 & (uint)uVar10) * 0x10 + param_2);
    uVar11 = (uint)*pbVar15;
    if (0x10 < *pbVar15) {
      do {
        if (uVar11 == 99) {
          return 1;
        }
        uVar10 = uVar10 >> (pbVar15[1] & 0x3f);
        for (uVar13 = uVar13 - pbVar15[1]; uVar13 < uVar11 - 0x10; uVar13 = uVar13 + 8) {
          uVar9 = (ulong)DAT_0011b008;
          if (DAT_0011b008 < DAT_0011b00c) {
            DAT_0011b008 = DAT_0011b008 + 1;
            bVar7 = (&DAT_0019d000)[uVar9];
          }
          else {
            DAT_0011b004 = uVar12;
            bVar7 = FUN_0010d830(0);
          }
          uVar10 = uVar10 | (ulong)bVar7 << ((byte)uVar13 & 0x3f);
        }
        pbVar15 = (byte *)((ulong)((uint)*(ushort *)(&DAT_00113380 + (ulong)(uVar11 - 0x10) * 2) &
                                  (uint)uVar10) * 0x10 + *(long *)(pbVar15 + 8));
        uVar11 = (uint)*pbVar15;
      } while (0x10 < uVar11);
    }
    uVar13 = uVar13 - pbVar15[1];
    uVar10 = uVar10 >> (pbVar15[1] & 0x3f);
    if (uVar13 < uVar11) {
      do {
        while( true ) {
          uVar8 = uVar13;
          uVar9 = (ulong)DAT_0011b008;
          if (DAT_0011b008 < DAT_0011b00c) break;
          DAT_0011b004 = uVar12;
          bVar7 = FUN_0010d830(0);
          uVar10 = uVar10 | (ulong)bVar7 << ((byte)uVar8 & 0x3f);
          uVar13 = uVar8 + 8;
          if (uVar11 <= uVar8 + 8) goto LAB_00108c1d;
        }
        DAT_0011b008 = DAT_0011b008 + 1;
        uVar10 = uVar10 | (ulong)(byte)(&DAT_0019d000)[uVar9] << ((byte)uVar8 & 0x3f);
        uVar13 = uVar8 + 8;
      } while (uVar8 + 8 < uVar11);
LAB_00108c1d:
      uVar13 = uVar8 + 8;
    }
    uVar13 = uVar13 - uVar11;
    uVar9 = (ulong)(((uint)uVar5 & (uint)uVar16) + (uint)uVar4);
    uVar16 = uVar10 >> ((byte)uVar11 & 0x3f);
    uVar11 = (uVar12 - *(ushort *)(pbVar15 + 8)) -
             ((uint)*(ushort *)(&DAT_00113380 + (ulong)uVar11 * 2) & (uint)uVar10);
    do {
      while( true ) {
        uVar11 = uVar11 & 0x7fff;
        uVar8 = uVar12;
        if (uVar12 < uVar11) {
          uVar8 = uVar11;
        }
        uVar10 = (ulong)(0x8000 - uVar8);
        if ((uint)uVar9 <= 0x8000 - uVar8) {
          uVar10 = uVar9;
        }
        uVar17 = (uint)uVar10;
        uVar14 = (uint)uVar9 - uVar17;
        uVar9 = (ulong)uVar14;
        uVar8 = uVar11 - uVar12;
        if (uVar11 < uVar12) {
          uVar8 = uVar12 - uVar11;
        }
        if (uVar17 <= uVar8) break;
        uVar8 = uVar11;
        do {
          uVar10 = (ulong)uVar8;
          uVar1 = uVar8 + (uVar12 - uVar11);
          uVar8 = uVar8 + 1;
          (&DAT_0013c000)[uVar1] = (&DAT_0013c000)[uVar10];
        } while (uVar8 != uVar11 + uVar17);
        if (uVar12 + uVar17 != 0x8000) goto LAB_00108ca2;
LAB_00108d17:
        uVar11 = uVar11 + uVar17;
        DAT_0011b004 = 0x8000;
        uVar12 = 0;
        FUN_0010d7e0();
        if (uVar14 == 0) goto joined_r0x001088a5;
      }
      memcpy(&DAT_0013c000 + uVar12,&DAT_0013c000 + uVar11,uVar10);
      if (uVar12 + uVar17 == 0x8000) goto LAB_00108d17;
LAB_00108ca2:
      uVar11 = uVar11 + uVar17;
      uVar12 = uVar12 + uVar17;
    } while (uVar14 != 0);
  } while( true );
}




// Function: huft_build @ 0x8dc0

undefined1
huft_build(uint *param_1,uint param_2,uint param_3,long param_4,long param_5,long *param_6,
            uint *param_7)

{
  uint *puVar1;
  byte bVar2;
  undefined1 uVar3;
  long lVar4;
  int *piVar5;
  ulong uVar7;
  void *pvVar8;
  uint *puVar9;
  uint uVar10;
  uint *puVar11;
  undefined6 uVar12;
  uint uVar13;
  uint uVar14;
  int iVar15;
  uint uVar16;
  undefined1 *puVar17;
  long lVar18;
  int iVar19;
  char cVar20;
  undefined8 *puVar21;
  uint uVar22;
  uint uVar23;
  undefined8 unaff_R13;
  long lVar24;
  uint uVar25;
  long in_FS_OFFSET;
  byte local_648;
  uint local_644;
  uint local_640;
  int local_63c;
  uint local_638;
  long local_630;
  uint *local_628;
  uint *local_620;
  uint local_614;
  int local_5f0;
  undefined1 local_5e8 [16];
  undefined1 local_5d8 [16];
  undefined1 local_5c8 [16];
  undefined1 local_5b8 [16];
  int local_5a8 [4];
  uint local_598 [18];
  long alStack_550 [17];
  uint local_4c8 [290];
  long local_40;
  int *piVar6;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_5e8 = (undefined1  [16])0x0;
  local_5a8[0] = 0;
  local_5d8 = (undefined1  [16])0x0;
  local_5c8 = (undefined1  [16])0x0;
  local_5b8 = (undefined1  [16])0x0;
  puVar11 = param_1;
  do {
    uVar25 = *puVar11;
    puVar11 = puVar11 + 1;
    *(int *)(local_5e8 + (ulong)uVar25 * 4) = *(int *)(local_5e8 + (ulong)uVar25 * 4) + 1;
  } while (param_1 + param_2 != puVar11);
  if (local_5e8._0_4_ == param_2) {
    pvVar8 = malloc(0x30);
    if (pvVar8 == (void *)0x0) {
LAB_001093fe:
      uVar3 = 3;
    }
    else {
      *(undefined8 *)((long)pvVar8 + 8) = 0;
      *(undefined2 *)((long)pvVar8 + 0x10) = 0x163;
      *(undefined2 *)((long)pvVar8 + 0x20) = 0x163;
      DAT_001dda08 = DAT_001dda08 + 3;
      *param_6 = (long)pvVar8 + 0x10;
      uVar3 = 0;
      *param_7 = 1;
    }
    goto LAB_00108f15;
  }
  lVar4 = 1;
  do {
    local_640 = (uint)lVar4;
    if (*(int *)(local_5e8 + lVar4 * 4) != 0) {
      iVar19 = 1 << ((byte)lVar4 & 0x1f);
      goto LAB_00108e9d;
    }
    lVar4 = lVar4 + 1;
  } while (lVar4 != 0x11);
  local_640 = 0x11;
  iVar19 = 0x20000;
LAB_00108e9d:
  uVar25 = *param_7;
  if (*param_7 < local_640) {
    uVar25 = local_640;
  }
  piVar5 = local_5a8;
  uVar22 = 0x10;
  do {
    if (*piVar5 != 0) {
      if (uVar22 < uVar25) {
        uVar25 = uVar22;
      }
      *param_7 = uVar25;
      local_614 = uVar22;
      if (local_640 < uVar22) {
        uVar7 = (ulong)local_640;
        iVar19 = iVar19 - *(int *)(local_5e8 + uVar7 * 4);
        if (iVar19 < 0) goto LAB_00108f10;
        puVar17 = local_5e8 + uVar7 * 4;
        goto LAB_00108f99;
      }
      local_5f0 = iVar19 - *(int *)(local_5e8 + (ulong)uVar22 * 4);
      if (local_5f0 < 0) goto LAB_00108f10;
      *(int *)(local_5e8 + (ulong)uVar22 * 4) = iVar19;
      local_598[1] = 0;
      if (uVar22 == 1) goto LAB_00108ffb;
      goto LAB_00108fcd;
    }
    piVar5 = piVar5 + -1;
    uVar22 = uVar22 - 1;
  } while (uVar22 != 0);
  if (uVar25 == 0) {
    local_5f0 = iVar19 - local_5e8._0_4_;
    *param_7 = 0;
    if (local_5f0 < 0) {
LAB_00108f10:
      uVar3 = 2;
      goto LAB_00108f15;
    }
    local_5e8._0_4_ = iVar19;
    local_598[1] = 0;
    local_614 = 0;
  }
  else {
    *param_7 = 0;
    uVar25 = 0;
    local_614 = 0;
    uVar13 = local_5e8._0_4_;
LAB_00108faa:
    local_5f0 = iVar19 - uVar13;
    if (local_5f0 < 0) goto LAB_00108f10;
    local_598[1] = 0;
    *(int *)(local_5e8 + (ulong)uVar22 * 4) = iVar19;
  }
LAB_00108fcd:
  uVar13 = 0;
  piVar5 = (int *)(local_5e8 + 4);
  puVar11 = local_598 + 2;
  do {
    piVar6 = piVar5 + 1;
    uVar13 = uVar13 + *piVar5;
    *puVar11 = uVar13;
    piVar5 = piVar6;
    puVar11 = puVar11 + 1;
  } while ((int *)(local_5e8 + (ulong)(uVar22 - 2) * 4 + 8) != piVar6);
LAB_00108ffb:
  local_598[1] = 0;
  uVar7 = 0;
  do {
    uVar22 = param_1[uVar7];
    if (uVar22 != 0) {
      uVar13 = local_598[uVar22];
      local_4c8[uVar13] = (uint)uVar7;
      local_598[uVar22] = uVar13 + 1;
    }
    uVar7 = uVar7 + 1;
  } while (param_2 != uVar7);
  alStack_550[1] = 0;
  iVar19 = -uVar25;
  local_598[0] = 0;
  if ((int)local_640 <= (int)local_614) {
    local_620 = local_4c8;
    uVar13 = 0;
    puVar11 = local_620 + local_598[(int)local_614];
    uVar22 = 0;
    local_63c = -1;
    local_628 = (uint *)(local_5e8 + (long)(int)local_640 * 4);
    lVar4 = 0;
    do {
      local_644 = *local_628;
      if (local_644 != 0) {
        uVar14 = 1 << ((char)local_640 - 1U & 0x1f);
        do {
          iVar15 = uVar25 + iVar19;
          if (iVar15 < (int)local_640) {
            uVar23 = local_614 - iVar15;
            lVar24 = (long)(local_63c + 1);
            uVar16 = local_640 - iVar15;
            do {
              local_638 = uVar16;
              iVar19 = iVar15;
              local_648 = (char)iVar19 - (char)uVar25;
              local_63c = (int)lVar24;
              uVar13 = 1 << ((byte)local_638 & 0x1f);
              uVar16 = local_638;
              if (local_644 < uVar13) {
                uVar10 = uVar25;
                if (uVar23 <= uVar25) {
                  uVar10 = uVar23;
                }
                if (local_638 < uVar10) {
                  iVar15 = uVar13 - local_644;
                  puVar9 = local_628;
                  while (uVar16 = uVar16 + 1, uVar16 < uVar10) {
                    puVar1 = puVar9 + 1;
                    puVar9 = puVar9 + 1;
                    if ((uint)(iVar15 * 2) <= *puVar1) break;
                    iVar15 = iVar15 * 2 - *puVar1;
                  }
                  uVar13 = 1 << ((byte)uVar16 & 0x1f);
                }
              }
              pvVar8 = malloc((ulong)(uVar13 + 1) << 4);
              if (pvVar8 == (void *)0x0) {
                if (local_63c != 0) {
                  FUN_00108d90(alStack_550[1]);
                }
                goto LAB_001093fe;
              }
              DAT_001dda08 = uVar13 + 1 + DAT_001dda08;
              lVar4 = (long)pvVar8 + 0x10;
              *param_6 = lVar4;
              param_6 = (long *)((long)pvVar8 + 8);
              *(undefined8 *)((long)pvVar8 + 8) = 0;
              alStack_550[lVar24 + 1] = lVar4;
              if (local_63c != 0) {
                local_598[lVar24] = uVar22;
                puVar21 = (undefined8 *)
                          (alStack_550[lVar24] + (ulong)(uVar22 >> (local_648 & 0x1f)) * 0x10);
                unaff_R13 = CONCAT62((int6)((ulong)unaff_R13 >> 0x10),
                                     CONCAT11((char)uVar25,(char)uVar16 + '\x10'));
                puVar21[1] = lVar4;
                *puVar21 = unaff_R13;
                local_630 = lVar4;
              }
              lVar24 = lVar24 + 1;
              uVar23 = uVar23 - uVar25;
              iVar15 = iVar19 + uVar25;
              uVar16 = local_638 - uVar25;
            } while ((int)(iVar19 + uVar25) < (int)local_640);
          }
          else {
            local_638 = local_640 - iVar19;
          }
          cVar20 = 'c';
          bVar2 = (byte)iVar19;
          if (local_620 < puVar11) {
            uVar16 = *local_620;
            local_620 = local_620 + 1;
            uVar12 = (undefined6)((ulong)local_630 >> 0x10);
            if (uVar16 < param_3) {
              local_630 = CONCAT62(uVar12,(short)uVar16);
              cVar20 = (uVar16 != 0x100) + '\x0f';
            }
            else {
              local_630 = CONCAT62(uVar12,*(undefined2 *)(param_4 + (ulong)(uVar16 - param_3) * 2));
              cVar20 = *(char *)(param_5 + (ulong)(uVar16 - param_3) * 2);
            }
          }
          for (uVar16 = uVar22 >> (bVar2 & 0x1f); uVar16 < uVar13;
              uVar16 = uVar16 + (1 << ((byte)local_638 & 0x1f))) {
            unaff_R13 = CONCAT62((int6)((ulong)unaff_R13 >> 0x10),
                                 CONCAT11((char)local_640 - bVar2,cVar20));
            puVar21 = (undefined8 *)((ulong)uVar16 * 0x10 + lVar4);
            *puVar21 = unaff_R13;
            puVar21[1] = local_630;
          }
          uVar16 = uVar14 & uVar22;
          uVar23 = uVar14;
          while (uVar16 != 0) {
            uVar22 = uVar22 ^ uVar23;
            uVar23 = uVar23 >> 1;
            uVar16 = uVar22 & uVar23;
          }
          uVar22 = uVar22 ^ uVar23;
          if (((1 << (bVar2 & 0x1f)) - 1U & uVar22) != local_598[local_63c]) {
            lVar24 = (long)(local_63c + -1);
            do {
              lVar18 = lVar24;
              iVar19 = iVar19 - uVar25;
              lVar24 = lVar18 + -1;
            } while (((1 << ((byte)iVar19 & 0x1f)) - 1U & uVar22) != local_598[lVar18]);
            local_63c = (int)lVar18;
          }
          local_644 = local_644 - 1;
        } while (local_644 != 0);
      }
      local_640 = local_640 + 1;
      local_628 = local_628 + 1;
    } while (local_640 != local_614 + 1);
  }
  uVar3 = local_614 != 1 && local_5f0 != 0;
LAB_00108f15:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
  while( true ) {
    iVar19 = iVar19 - *(int *)(puVar17 + 4);
    puVar17 = puVar17 + 4;
    if (iVar19 < 0) break;
LAB_00108f99:
    iVar19 = iVar19 * 2;
    if (local_5e8 + (((uVar22 - local_640) - 1) + uVar7) * 4 == puVar17) {
      uVar13 = *(uint *)(local_5e8 + (ulong)uVar22 * 4);
      goto LAB_00108faa;
    }
  }
  goto LAB_00108f10;
}




// Function: inflate_dynamic @ 0x94b0

uint inflate_dynamic(void)

{
  ushort uVar1;
  ushort uVar2;
  undefined4 uVar3;
  byte bVar4;
  byte bVar5;
  uint uVar6;
  int iVar7;
  uint uVar8;
  ulong uVar9;
  ulong uVar10;
  uint uVar11;
  uint *puVar12;
  uint uVar13;
  uint uVar14;
  uint uVar15;
  uint uVar16;
  long in_FS_OFFSET;
  uint local_588;
  uint local_57c;
  uint local_550;
  undefined4 local_54c;
  long local_548;
  char *local_540;
  uint local_538 [11];
  uint auStack_50c [307];
  long local_40;
  
  uVar9 = DAT_001dda10;
  uVar15 = DAT_001dda0c;
  uVar3 = DAT_0011b004;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar13 = DAT_001dda0c;
  if (DAT_001dda0c < 5) {
    uVar10 = (ulong)DAT_0011b008;
    if (DAT_0011b008 < DAT_0011b00c) {
      DAT_0011b008 = DAT_0011b008 + 1;
      bVar4 = (&DAT_0019d000)[uVar10];
    }
    else {
      bVar4 = FUN_0010d830(0);
    }
    uVar13 = uVar15 + 8;
    uVar9 = uVar9 | (ulong)bVar4 << ((byte)uVar15 & 0x3f);
  }
  uVar15 = uVar13 - 5;
  uVar10 = uVar9 >> 5;
  uVar6 = ((uint)uVar9 & 0x1f) + 0x101;
  uVar8 = uVar15;
  if (uVar15 < 5) {
    uVar9 = (ulong)DAT_0011b008;
    if (DAT_0011b008 < DAT_0011b00c) {
      DAT_0011b008 = DAT_0011b008 + 1;
      bVar4 = (&DAT_0019d000)[uVar9];
    }
    else {
      DAT_0011b004 = uVar3;
      bVar4 = FUN_0010d830(0);
    }
    uVar8 = uVar13 + 3;
    uVar10 = uVar10 | (ulong)bVar4 << ((byte)uVar15 & 0x3f);
  }
  uVar14 = uVar8 - 5;
  uVar9 = uVar10 >> 5;
  uVar13 = ((uint)uVar10 & 0x1f) + 1;
  uVar15 = uVar14;
  if (uVar14 < 4) {
    uVar10 = (ulong)DAT_0011b008;
    if (DAT_0011b008 < DAT_0011b00c) {
      DAT_0011b008 = DAT_0011b008 + 1;
      bVar4 = (&DAT_0019d000)[uVar10];
    }
    else {
      DAT_0011b004 = uVar3;
      bVar4 = FUN_0010d830(0);
    }
    uVar15 = uVar8 + 3;
    uVar9 = uVar9 | (ulong)bVar4 << ((byte)uVar14 & 0x3f);
  }
  if ((0x11e < uVar6) || (0x1e < uVar13)) {
LAB_00109b0a:
    uVar14 = 1;
    goto LAB_001097ba;
  }
  uVar15 = uVar15 - 4;
  puVar12 = &DAT_001133c0;
  uVar14 = (uint)uVar9 & 0xf;
  uVar8 = uVar14 + 4;
  uVar9 = uVar9 >> 4;
  do {
    uVar16 = uVar15;
    if (uVar15 < 3) {
      uVar10 = (ulong)DAT_0011b008;
      if (DAT_0011b008 < DAT_0011b00c) {
        DAT_0011b008 = DAT_0011b008 + 1;
        bVar4 = (&DAT_0019d000)[uVar10];
      }
      else {
        DAT_0011b004 = uVar3;
        bVar4 = FUN_0010d830(0);
      }
      uVar16 = uVar15 + 8;
      uVar9 = uVar9 | (ulong)bVar4 << ((byte)uVar15 & 0x3f);
    }
    uVar11 = *puVar12;
    puVar12 = puVar12 + 1;
    uVar10 = uVar9 >> 3;
    uVar15 = uVar16 - 3;
    local_538[uVar11] = (uint)uVar9 & 7;
    uVar9 = uVar10;
  } while ((uint *)(&DAT_001133d0 + (ulong)uVar14 * 4) != puVar12);
  for (; uVar8 != 0x13; uVar8 = uVar8 + 1) {
    local_538[(uint)(&DAT_001133c0)[uVar8]] = 0;
  }
  local_550 = 7;
  uVar14 = FUN_00108dc0(local_538,0x13,0x13,0,0,&local_548,&local_550);
  uVar8 = local_550;
  if (uVar14 == 0) {
    if (local_548 == 0) {
      uVar14 = 2;
      goto LAB_001097ba;
    }
    local_57c = 0;
    uVar16 = uVar6 + uVar13;
    local_588 = 0;
    uVar1 = *(ushort *)(&DAT_00113380 + (long)(int)local_550 * 2);
    uVar14 = 0;
    do {
      for (; uVar15 < uVar8; uVar15 = uVar15 + 8) {
        uVar9 = (ulong)DAT_0011b008;
        if (DAT_0011b008 < DAT_0011b00c) {
          DAT_0011b008 = DAT_0011b008 + 1;
          bVar4 = (&DAT_0019d000)[uVar9];
        }
        else {
          DAT_0011b004 = uVar3;
          bVar4 = FUN_0010d830(0);
        }
        uVar10 = uVar10 | (ulong)bVar4 << ((byte)uVar15 & 0x3f);
      }
      local_540 = (char *)((ulong)((uint)uVar1 & (uint)uVar10) * 0x10 + local_548);
      uVar9 = uVar10 >> (local_540[1] & 0x3fU);
      uVar15 = uVar15 - (byte)local_540[1];
      if (*local_540 == 'c') {
        FUN_00108d90();
        uVar14 = 2;
        goto LAB_001097ba;
      }
      uVar2 = *(ushort *)(local_540 + 8);
      uVar11 = (uint)uVar2;
      if (uVar2 < 0x10) {
        local_588 = uVar14 + 1;
        local_538[(int)uVar14] = uVar11;
        uVar10 = uVar9;
        local_57c = uVar11;
      }
      else {
        bVar4 = (byte)uVar15;
        if (uVar2 == 0x10) {
          if (uVar15 < 2) {
            uVar10 = (ulong)DAT_0011b008;
            if (DAT_0011b008 < DAT_0011b00c) {
              DAT_0011b008 = DAT_0011b008 + 1;
              bVar5 = (&DAT_0019d000)[uVar10];
            }
            else {
              DAT_0011b004 = uVar3;
              bVar5 = FUN_0010d830(0);
            }
            uVar15 = uVar15 + 8;
            uVar9 = uVar9 | (ulong)bVar5 << (bVar4 & 0x3f);
          }
          uVar10 = uVar9 >> 2;
          uVar15 = uVar15 - 2;
          uVar11 = (uint)uVar9 & 3;
          if (uVar16 < uVar11 + 3 + local_588) goto LAB_00109b0a;
          puVar12 = local_538 + (int)uVar14;
          do {
            *puVar12 = local_57c;
            puVar12 = puVar12 + 1;
          } while (local_538 + (ulong)uVar11 + (long)(int)uVar14 + 3 != puVar12);
          local_588 = uVar11 + 3 + uVar14;
        }
        else if (uVar11 == 0x11) {
          if (uVar15 < 3) {
            uVar10 = (ulong)DAT_0011b008;
            if (DAT_0011b008 < DAT_0011b00c) {
              DAT_0011b008 = DAT_0011b008 + 1;
              bVar5 = (&DAT_0019d000)[uVar10];
            }
            else {
              DAT_0011b004 = uVar3;
              bVar5 = FUN_0010d830(0);
            }
            uVar15 = uVar15 + 8;
            uVar9 = uVar9 | (ulong)bVar5 << (bVar4 & 0x3f);
          }
          uVar10 = uVar9 >> 3;
          uVar15 = uVar15 - 3;
          uVar11 = (uint)uVar9 & 7;
          if (uVar16 < uVar11 + 3 + local_588) goto LAB_00109b0a;
          puVar12 = local_538 + (int)uVar14;
          do {
            *puVar12 = 0;
            puVar12 = puVar12 + 1;
          } while (local_538 + (ulong)uVar11 + (long)(int)uVar14 + 3 != puVar12);
          local_57c = 0;
          local_588 = uVar11 + 3 + uVar14;
        }
        else {
          if (uVar15 < 7) {
            uVar10 = (ulong)DAT_0011b008;
            if (DAT_0011b008 < DAT_0011b00c) {
              DAT_0011b008 = DAT_0011b008 + 1;
              bVar5 = (&DAT_0019d000)[uVar10];
            }
            else {
              DAT_0011b004 = uVar3;
              bVar5 = FUN_0010d830(0);
            }
            uVar15 = uVar15 + 8;
            uVar9 = uVar9 | (ulong)bVar5 << (bVar4 & 0x3f);
          }
          uVar10 = uVar9 >> 7;
          uVar15 = uVar15 - 7;
          uVar11 = (uint)uVar9 & 0x7f;
          if (uVar16 < uVar11 + 0xb + local_588) goto LAB_00109b0a;
          puVar12 = local_538 + (int)uVar14;
          do {
            *puVar12 = 0;
            puVar12 = puVar12 + 1;
          } while (auStack_50c + (ulong)uVar11 + (long)(int)uVar14 != puVar12);
          local_57c = 0;
          local_588 = uVar11 + 0xb + uVar14;
        }
      }
      uVar14 = local_588;
    } while (local_588 < uVar16);
    FUN_00108d90(local_548);
    local_550 = 9;
    DAT_001dda0c = uVar15;
    DAT_001dda10 = uVar10;
    uVar14 = FUN_00108dc0(local_538,uVar6,0x101,&DAT_001191a0,&DAT_00119160,&local_548,&local_550);
    if (uVar14 != 0) goto LAB_001097b0;
    local_54c = 6;
    uVar14 = FUN_00108dc0(local_538 + uVar6,uVar13,0,&DAT_00119120,&DAT_001190e0,&local_540,
                          &local_54c);
    if (uVar14 == 0) {
      iVar7 = FUN_00108840(local_548,local_540,local_550,local_54c);
      uVar14 = (uint)(iVar7 != 0);
      FUN_00108d90(local_548);
      FUN_00108d90(local_540);
      goto LAB_001097ba;
    }
    if (uVar14 == 1) {
      FUN_00108d90(local_540);
    }
  }
  else {
LAB_001097b0:
    if (uVar14 != 1) goto LAB_001097ba;
  }
  FUN_00108d90(local_548);
LAB_001097ba:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar14;
}




// Function: inflate_fixed @ 0x9b70

ulong inflate_fixed(void)

{
  uint uVar1;
  int iVar2;
  undefined8 *puVar3;
  ulong uVar4;
  long in_FS_OFFSET;
  undefined4 local_4c0;
  undefined4 local_4bc;
  undefined8 local_4b8;
  undefined8 local_4b0;
  undefined8 local_4a8 [15];
  undefined8 local_430 [57];
  undefined8 local_268 [56];
  undefined8 local_a8 [12];
  undefined8 local_48;
  undefined8 local_40;
  undefined8 local_38;
  undefined8 local_30;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  puVar3 = local_4a8;
  do {
    *(undefined4 *)puVar3 = 8;
    puVar3 = (undefined8 *)((long)puVar3 + 4);
  } while (puVar3 != local_268);
  do {
    *(undefined4 *)puVar3 = 9;
    puVar3 = (undefined8 *)((long)puVar3 + 4);
  } while (local_a8 != puVar3);
  puVar3 = local_a8;
  do {
    *(undefined4 *)puVar3 = 7;
    puVar3 = (undefined8 *)((long)puVar3 + 4);
  } while (&local_48 != puVar3);
  local_4c0 = 7;
  local_48 = 0x800000008;
  local_40 = 0x800000008;
  local_38 = 0x800000008;
  local_30 = 0x800000008;
  uVar4 = FUN_00108dc0(local_4a8,0x120,0x101,&DAT_001191a0,&DAT_00119160,&local_4b8,&local_4c0);
  if ((int)uVar4 == 0) {
    puVar3 = local_4a8;
    do {
      *(undefined4 *)puVar3 = 5;
      puVar3 = (undefined8 *)((long)puVar3 + 4);
    } while (puVar3 != local_4a8 + 0xf);
    local_4bc = 5;
    uVar1 = FUN_00108dc0(local_4a8,0x1e,0,&DAT_00119120,&DAT_001190e0,&local_4b0);
    if ((int)uVar1 < 2) {
      iVar2 = FUN_00108840(local_4b8,local_4b0,local_4c0,local_4bc);
      if (iVar2 == 0) {
        FUN_00108d90(local_4b8);
        FUN_00108d90(local_4b0);
        uVar4 = 0;
      }
      else {
        uVar4 = 1;
      }
    }
    else {
      FUN_00108d90(local_4b8);
      uVar4 = (ulong)uVar1;
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: inflate_block @ 0x9d20

undefined8 inflate_block(uint *param_1)

{
  uint uVar1;
  undefined4 uVar2;
  int iVar3;
  byte bVar4;
  ulong uVar5;
  undefined8 uVar6;
  uint uVar7;
  ulong uVar8;
  
  uVar8 = DAT_001dda10;
  iVar3 = DAT_001dda0c;
  uVar2 = DAT_0011b004;
  uVar7 = (uint)DAT_001dda10;
  if (DAT_001dda0c == 0) {
    uVar5 = (ulong)DAT_0011b008;
    if (DAT_0011b008 < DAT_0011b00c) {
      DAT_0011b008 = DAT_0011b008 + 1;
      bVar4 = (&DAT_0019d000)[uVar5];
    }
    else {
      bVar4 = FUN_0010d830(0);
    }
    uVar8 = (bVar4 | uVar8) >> 1;
    *param_1 = (bVar4 | uVar7) & 1;
    DAT_001dda0c = 5;
  }
  else {
    uVar1 = DAT_001dda0c - 1;
    uVar8 = DAT_001dda10 >> 1;
    *param_1 = uVar7 & 1;
    if (uVar1 < 2) {
      uVar5 = (ulong)DAT_0011b008;
      if (DAT_0011b008 < DAT_0011b00c) {
        DAT_0011b008 = DAT_0011b008 + 1;
        bVar4 = (&DAT_0019d000)[uVar5];
      }
      else {
        DAT_0011b004 = uVar2;
        bVar4 = FUN_0010d830(0);
      }
      uVar8 = uVar8 | (ulong)bVar4 << ((byte)uVar1 & 0x3f);
      DAT_001dda0c = iVar3 + 5;
    }
    else {
      DAT_001dda0c = iVar3 + -3;
    }
  }
  DAT_001dda10 = uVar8 >> 2;
  uVar7 = (uint)uVar8 & 3;
  if (uVar7 == 2) {
    uVar6 = FUN_001094b0();
    return uVar6;
  }
  if ((uVar8 & 3) == 0) {
    uVar6 = FUN_00108640();
    return uVar6;
  }
  if (uVar7 != 1) {
    return 2;
  }
  uVar6 = FUN_00109b70();
  return uVar6;
}




// Function: inflate @ 0x9e80

ulong inflate(void)

{
  ulong uVar1;
  uint uVar2;
  long in_FS_OFFSET;
  int local_14;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_0011b004 = 0;
  DAT_001dda0c = 0;
  DAT_001dda10 = 0;
  do {
    DAT_001dda08 = 0;
    uVar1 = FUN_00109d20(&local_14);
    if ((int)uVar1 != 0) goto LAB_00109ede;
  } while (local_14 == 0);
  if (7 < DAT_001dda0c) {
    uVar2 = DAT_001dda0c - 8;
    DAT_001dda0c = DAT_001dda0c & 7;
    DAT_0011b008 = DAT_0011b008 + ~(uVar2 >> 3);
  }
  FUN_0010d7e0();
  uVar1 = uVar1 & 0xffffffff;
LAB_00109ede:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: init_block @ 0x9f30

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void init_block(void)

{
  undefined2 *puVar1;
  undefined2 *puVar2;
  
  puVar2 = &DAT_001e0080;
  do {
    puVar1 = puVar2 + 2;
    *puVar2 = 0;
    puVar2 = puVar1;
  } while (puVar1 != (undefined2 *)&DAT_001e04f8);
  puVar2 = &DAT_001dff80;
  do {
    puVar1 = puVar2 + 2;
    *puVar2 = 0;
    puVar2 = puVar1;
  } while (puVar1 != (undefined2 *)&DAT_001dfff8);
  puVar2 = &DAT_001df9e0;
  do {
    puVar1 = puVar2 + 2;
    *puVar2 = 0;
    puVar2 = puVar1;
  } while (puVar1 != (undefined2 *)&DAT_001dfa2c);
  DAT_001dda38 = 0;
  _DAT_001e0480 = 1;
  DAT_001dda40 = 0;
  DAT_001dda4c = 0;
  DAT_001dda50 = 0;
  DAT_001dda54 = 0;
  DAT_001dda49 = 0;
  DAT_001dda48 = 1;
  return;
}




// Function: pqdownheap @ 0x9fe0

void pqdownheap(long param_1,int param_2)

{
  ushort uVar1;
  ushort uVar2;
  ushort uVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  ushort uVar11;
  long lVar12;
  
  iVar7 = DAT_001df0a4;
  lVar12 = (long)param_2;
  param_2 = param_2 * 2;
  iVar4 = *(int *)(&DAT_001df0c0 + lVar12 * 4);
  if (param_2 <= DAT_001df0a4) {
    uVar1 = *(ushort *)(param_1 + (long)iVar4 * 4);
    do {
      iVar5 = *(int *)(&DAT_001df0c0 + (long)param_2 * 4);
      uVar2 = *(ushort *)(param_1 + (long)iVar5 * 4);
      iVar9 = param_2;
      uVar11 = uVar2;
      iVar10 = iVar5;
      if (param_2 < iVar7) {
        iVar8 = param_2 + 1;
        iVar6 = *(int *)(&DAT_001df0c0 + (long)iVar8 * 4);
        uVar3 = *(ushort *)(param_1 + (long)iVar6 * 4);
        iVar9 = iVar8;
        uVar11 = uVar3;
        iVar10 = iVar6;
        if (((uVar2 <= uVar3) && (iVar9 = param_2, uVar11 = uVar2, iVar10 = iVar5, uVar3 == uVar2))
           && (iVar9 = iVar8, iVar10 = iVar6,
              (byte)(&DAT_001dee60)[iVar5] < (byte)(&DAT_001dee60)[iVar6])) {
          iVar9 = param_2;
          iVar10 = iVar5;
        }
      }
      if ((uVar1 < uVar11) ||
         ((uVar1 == uVar11 && ((byte)(&DAT_001dee60)[iVar4] <= (byte)(&DAT_001dee60)[iVar10]))))
      break;
      param_2 = iVar9 * 2;
      *(int *)(&DAT_001df0c0 + lVar12 * 4) = iVar10;
      lVar12 = (long)iVar9;
    } while (param_2 <= iVar7);
  }
  *(int *)(&DAT_001df0c0 + lVar12 * 4) = iVar4;
  return;
}




// Function: gen_bitlen @ 0xa0c0

void gen_bitlen(long *param_1)

{
  ushort *puVar1;
  short sVar2;
  int iVar3;
  int iVar4;
  long lVar5;
  bool bVar6;
  bool bVar7;
  uint uVar8;
  ushort uVar9;
  uint uVar10;
  undefined2 *puVar11;
  undefined2 *puVar12;
  long lVar13;
  long lVar14;
  int iVar15;
  int iVar16;
  long lVar17;
  long lVar18;
  uint uVar19;
  undefined4 *puVar20;
  int iVar21;
  uint uVar22;
  long lVar23;
  long lVar24;
  long local_40;
  
  lVar23 = param_1[2];
  lVar5 = *param_1;
  iVar3 = *(int *)((long)param_1 + 0x24);
  uVar19 = *(uint *)(param_1 + 4);
  lVar18 = param_1[1];
  lVar13 = param_1[3];
  puVar11 = &DAT_001df9c0;
  do {
    puVar12 = puVar11 + 1;
    *puVar11 = 0;
    iVar16 = DAT_001df0a0;
    puVar11 = puVar12;
  } while (puVar12 != &DAT_001df9e0);
  lVar14 = (long)DAT_001df0a0;
  *(undefined2 *)(lVar5 + 2 + (long)*(int *)(&DAT_001df0c0 + lVar14 * 4) * 4) = 0;
  if (iVar16 < 0x23c) {
    bVar7 = false;
    iVar21 = 0;
    bVar6 = false;
    puVar20 = (undefined4 *)(&DAT_001df0c0 + lVar14 * 4);
    local_40 = DAT_001dda38;
    lVar24 = DAT_001dda40;
    do {
      iVar4 = puVar20[1];
      puVar1 = (ushort *)(lVar5 + (long)iVar4 * 4);
      uVar10 = *(ushort *)(lVar5 + 2 + (ulong)puVar1[1] * 4) + 1;
      if ((int)uVar19 < (int)uVar10) {
        iVar21 = iVar21 + 1;
        uVar10 = uVar19;
      }
      puVar1[1] = (ushort)uVar10;
      if (iVar4 <= iVar3) {
        lVar17 = (long)(int)uVar10;
        iVar15 = 0;
        (&DAT_001df9c0)[lVar17] = (&DAT_001df9c0)[lVar17] + 1;
        if ((int)lVar13 <= iVar4) {
          iVar15 = *(int *)(lVar23 + (long)(iVar4 - (int)lVar13) * 4);
          lVar17 = (long)(int)(uVar10 + iVar15);
        }
        lVar24 = lVar24 + lVar17 * (ulong)*puVar1;
        bVar7 = true;
        if (lVar18 != 0) {
          bVar6 = true;
          local_40 = local_40 +
                     (long)(int)((uint)*(ushort *)(lVar18 + 2 + (long)iVar4 * 4) + iVar15) *
                     (ulong)*puVar1;
        }
      }
      puVar20 = puVar20 + 1;
    } while (puVar20 != &DAT_001df0c4 + (ulong)(0x23b - iVar16) + lVar14);
    if (bVar6) {
      DAT_001dda38 = local_40;
    }
    if (bVar7) {
      DAT_001dda40 = lVar24;
    }
    if (iVar21 != 0) {
      iVar16 = iVar21 + -2;
      uVar10 = uVar19 - 1;
      iVar21 = iVar16 - (iVar21 - 1U & 0xfffffffe);
      sVar2 = (&DAT_001df9c0)[(int)uVar10];
      lVar23 = (long)(int)uVar19;
      uVar22 = uVar19;
      puVar11 = &DAT_001df9c0 + lVar23;
      uVar8 = uVar10;
      while( true ) {
        while (sVar2 == 0) {
          sVar2 = puVar11[-2];
          uVar22 = uVar8;
          puVar11 = puVar11 + -1;
          uVar8 = uVar8 - 1;
        }
        (&DAT_001df9c0)[(int)uVar8] = sVar2 + -1;
        (&DAT_001df9c0)[(int)uVar22] = (&DAT_001df9c0)[(int)uVar22] + 2;
        uVar9 = (&DAT_001df9c0)[lVar23] - 1;
        (&DAT_001df9c0)[lVar23] = uVar9;
        if (iVar16 == iVar21) break;
        sVar2 = (&DAT_001df9c0)[(int)uVar10];
        iVar16 = iVar16 + -2;
        uVar22 = uVar19;
        puVar11 = &DAT_001df9c0 + lVar23;
        uVar8 = uVar10;
      }
      if (uVar19 != 0) {
        bVar6 = false;
        iVar16 = 0x23d;
        lVar18 = DAT_001dda40;
        while( true ) {
          uVar22 = (uint)uVar9;
          if (uVar22 != 0) {
            lVar13 = (long)(iVar16 + -1);
            do {
              while( true ) {
                iVar16 = (int)lVar13;
                if (*(int *)(&DAT_001df0c0 + lVar13 * 4) <= iVar3) break;
                lVar13 = lVar13 + -1;
              }
              puVar1 = (ushort *)(lVar5 + (long)*(int *)(&DAT_001df0c0 + lVar13 * 4) * 4);
              uVar9 = puVar1[1];
              if (uVar9 != uVar19) {
                puVar1[1] = (ushort)uVar19;
                bVar6 = true;
                lVar18 = lVar18 + (ulong)*puVar1 * (lVar23 - (ulong)uVar9);
              }
              lVar13 = lVar13 + -1;
              uVar22 = uVar22 - 1;
            } while (uVar22 != 0);
          }
          lVar23 = lVar23 + -1;
          if (uVar10 == 0) break;
          uVar9 = (&DAT_001df9c0)[lVar23];
          uVar19 = uVar10;
          uVar10 = uVar10 - 1;
        }
        if (bVar6) {
          DAT_001dda40 = lVar18;
        }
      }
    }
  }
  return;
}




// Function: scan_tree @ 0xa370

void scan_tree(long param_1,int param_2)

{
  ushort *puVar1;
  ushort uVar2;
  int iVar3;
  ushort *puVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  int iVar8;
  int iVar9;
  
  uVar2 = *(ushort *)(param_1 + 2);
  *(undefined2 *)(param_1 + 6 + (long)param_2 * 4) = 0xffff;
  iVar9 = 4 - (uint)(uVar2 == 0);
  iVar8 = (-(uint)(uVar2 == 0) & 0x83) + 7;
  if (-1 < param_2) {
    puVar1 = (ushort *)(param_1 + 10 + (long)param_2 * 4);
    puVar4 = (ushort *)(param_1 + 6);
    iVar3 = 0;
    uVar7 = 0xffffffff;
    uVar5 = (uint)uVar2;
    do {
      while( true ) {
        iVar3 = iVar3 + 1;
        uVar2 = *puVar4;
        uVar6 = (uint)uVar2;
        if (iVar3 < iVar8) break;
LAB_0010a3c0:
        if (uVar5 == 0) {
          if (iVar3 < 0xb) {
            DAT_001dfa24 = DAT_001dfa24 + 1;
          }
          else {
            DAT_001dfa28 = DAT_001dfa28 + 1;
          }
        }
        else {
          if (uVar5 != uVar7) {
            (&DAT_001df9e0)[(long)(int)uVar5 * 2] = (&DAT_001df9e0)[(long)(int)uVar5 * 2] + 1;
          }
          DAT_001dfa20 = DAT_001dfa20 + 1;
        }
        if (uVar6 == 0) {
LAB_0010a440:
          iVar9 = 3;
          iVar8 = 0x8a;
          iVar3 = 0;
          uVar7 = uVar5;
          goto LAB_0010a3f5;
        }
        if (uVar6 == uVar5) {
          iVar9 = 3;
          iVar8 = 6;
          iVar3 = 0;
          uVar7 = uVar6;
          goto LAB_0010a3f5;
        }
LAB_0010a420:
        puVar4 = puVar4 + 2;
        iVar9 = 4;
        iVar3 = 0;
        iVar8 = 7;
        uVar7 = uVar5;
        uVar5 = uVar6;
        if (puVar1 == puVar4) {
          return;
        }
      }
      if (uVar6 != uVar5) {
        if (iVar9 <= iVar3) goto LAB_0010a3c0;
        (&DAT_001df9e0)[(long)(int)uVar5 * 2] = (&DAT_001df9e0)[(long)(int)uVar5 * 2] + (short)iVar3
        ;
        if (uVar2 != 0) goto LAB_0010a420;
        goto LAB_0010a440;
      }
LAB_0010a3f5:
      puVar4 = puVar4 + 2;
      uVar5 = uVar6;
    } while (puVar1 != puVar4);
  }
  return;
}




// Function: set_file_type @ 0xa480

void set_file_type(void)

{
  ushort uVar1;
  ushort *puVar2;
  uint uVar3;
  uint uVar4;
  
  uVar4 = 0;
  puVar2 = &DAT_001e0080;
  do {
    uVar1 = *puVar2;
    puVar2 = puVar2 + 2;
    uVar4 = uVar4 + uVar1;
  } while (puVar2 != &DAT_001e009c);
  puVar2 = &DAT_001e009c;
  uVar3 = 0;
  do {
    uVar1 = *puVar2;
    puVar2 = puVar2 + 2;
    uVar3 = uVar3 + uVar1;
  } while (puVar2 != &DAT_001e0280);
  puVar2 = &DAT_001e0280;
  do {
    uVar1 = *puVar2;
    puVar2 = puVar2 + 2;
    uVar4 = uVar4 + uVar1;
  } while (puVar2 != (ushort *)&DAT_001e0480);
  *DAT_001dda20 = (ushort)(uVar4 <= uVar3 >> 2);
  return;
}




// Function: gen_codes @ 0xa500

void gen_codes(undefined2 *param_1,int param_2)

{
  undefined2 *puVar1;
  ushort uVar2;
  short sVar3;
  undefined2 uVar4;
  long lVar5;
  long in_FS_OFFSET;
  short asStack_48 [20];
  long local_20;
  
  lVar5 = 0;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  sVar3 = 0;
  do {
    sVar3 = (sVar3 + *(short *)((long)&DAT_001df9c0 + lVar5)) * 2;
    *(short *)((long)asStack_48 + lVar5 + 2) = sVar3;
    lVar5 = lVar5 + 2;
  } while (lVar5 != 0x1e);
  if (-1 < param_2) {
    puVar1 = param_1 + (long)param_2 * 2 + 2;
    do {
      uVar2 = param_1[1];
      if (uVar2 != 0) {
        asStack_48[(int)(uint)uVar2] = asStack_48[(int)(uint)uVar2] + 1;
        uVar4 = FUN_00104380();
        *param_1 = uVar4;
      }
      param_1 = param_1 + 2;
    } while (param_1 != puVar1);
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: build_tree @ 0xa5a0

void build_tree(undefined8 *param_1)

{
  byte bVar1;
  short sVar2;
  undefined2 *puVar3;
  bool bVar4;
  short sVar5;
  int iVar6;
  ulong uVar7;
  long lVar8;
  ulong uVar9;
  long lVar10;
  uint uVar11;
  long lVar12;
  long lVar13;
  long lVar14;
  int iVar15;
  undefined2 *puVar16;
  short local_50;
  int local_48;
  int local_44;
  
  iVar6 = *(int *)((long)param_1 + 0x1c);
  puVar3 = (undefined2 *)*param_1;
  lVar13 = param_1[1];
  DAT_001df0a0 = 0x23d;
  if (iVar6 < 1) {
    local_48 = -1;
    local_44 = 0;
  }
  else {
    bVar4 = false;
    uVar7 = 0;
    local_44 = 0;
    uVar9 = 0xffffffff;
    do {
      while (local_48 = (int)uVar9, puVar3[uVar7 * 2] != 0) {
        local_44 = local_44 + 1;
        (&DAT_001dee60)[uVar7] = 0;
        local_48 = (int)uVar7;
        uVar9 = uVar7 & 0xffffffff;
        *(int *)(&DAT_001df0c0 + (long)local_44 * 4) = local_48;
        uVar7 = uVar7 + 1;
        bVar4 = true;
        if (uVar7 == (long)iVar6) goto LAB_0010a637;
      }
      puVar3[uVar7 * 2 + 1] = 0;
      uVar7 = uVar7 + 1;
    } while (uVar7 != (long)iVar6);
LAB_0010a637:
    if (bVar4) {
      if (1 < local_44) goto LAB_0010a715;
    }
    else {
      local_44 = 0;
    }
  }
  lVar12 = (long)local_44;
  bVar4 = false;
  lVar14 = DAT_001dda38;
  do {
    if (local_48 < 2) {
      local_48 = local_48 + 1;
      lVar8 = (long)local_48;
      lVar10 = (long)local_48 * 4;
      puVar16 = puVar3 + (long)local_48 * 2;
    }
    else {
      lVar10 = 0;
      lVar8 = 0;
      puVar16 = puVar3;
    }
    (&DAT_001df0c4)[lVar12] = (int)lVar8;
    *puVar16 = 1;
    (&DAT_001dee60)[lVar8] = 0;
    if (lVar13 != 0) {
      bVar4 = true;
      lVar14 = lVar14 - (ulong)*(ushort *)(lVar13 + 2 + lVar10);
    }
    lVar12 = lVar12 + 1;
  } while ((int)lVar12 < 2);
  uVar11 = 0;
  if (local_44 < 2) {
    uVar11 = 1U - local_44;
  }
  iVar15 = local_44 + 1 + uVar11;
  lVar13 = -(ulong)(1U - local_44);
  if (1 < local_44) {
    lVar13 = 0;
  }
  DAT_001dda40 = DAT_001dda40 + -1 + lVar13;
  local_44 = iVar15;
  if (bVar4) {
    DAT_001dda38 = lVar14;
  }
LAB_0010a715:
  lVar13 = (long)iVar6;
  DAT_001df0a4 = local_44;
  *(int *)((long)param_1 + 0x24) = local_48;
  iVar15 = local_44 >> 1;
  do {
    FUN_00109fe0(puVar3,iVar15);
    iVar15 = iVar15 + -1;
  } while (iVar15 != 0);
  lVar12 = (long)local_44;
  local_50 = (short)iVar6;
  lVar14 = 0x23b;
  do {
    iVar6 = DAT_001df0c4;
    DAT_001df0a4 = (int)lVar12 + -1;
    DAT_001df0c4 = *(int *)(&DAT_001df0c0 + lVar12 * 4);
    FUN_00109fe0(puVar3,1);
    DAT_001df0a0 = (int)lVar14;
    sVar5 = puVar3[(long)DAT_001df0c4 * 2];
    sVar2 = puVar3[(long)iVar6 * 2];
    (&DAT_001df0c4)[lVar14] = iVar6;
    *(int *)(&DAT_001df0c0 + lVar14 * 4) = DAT_001df0c4;
    puVar3[lVar13 * 2] = sVar5 + sVar2;
    bVar1 = (&DAT_001dee60)[DAT_001df0c4];
    if ((byte)(&DAT_001dee60)[DAT_001df0c4] <= (byte)(&DAT_001dee60)[iVar6]) {
      bVar1 = (&DAT_001dee60)[iVar6];
    }
    lVar14 = lVar14 + -2;
    (&DAT_001dee60)[lVar13] = bVar1 + 1;
    sVar5 = (local_50 + (short)local_44) - (short)lVar12;
    lVar12 = lVar12 + -1;
    (puVar3 + (long)DAT_001df0c4 * 2)[1] = sVar5;
    (puVar3 + (long)iVar6 * 2)[1] = sVar5;
    DAT_001df0c4 = (int)lVar13;
    lVar13 = lVar13 + 1;
    FUN_00109fe0(puVar3,1);
  } while (1 < (int)lVar12);
  iVar6 = local_44 * -2 + 0x241;
  if (local_44 < 2) {
    iVar6 = 0x23d;
  }
  DAT_001df0a0 = iVar6 + -3;
  *(int *)(&DAT_001df0c0 + (long)DAT_001df0a0 * 4) = DAT_001df0c4;
  FUN_0010a0c0(param_1);
  FUN_0010a500(puVar3,local_48);
  return;
}




// Function: build_bl_tree @ 0xa8c0

ulong build_bl_tree(void)

{
  ulong uVar1;
  ulong uVar2;
  
  FUN_0010a370(&DAT_001e0080,DAT_001193e4);
  FUN_0010a370(&DAT_001dff80,DAT_001193a4);
  FUN_0010a5a0(&PTR_DAT_00119340);
  uVar2 = 0xf;
  uVar1 = 0x12;
  while( true ) {
    if (*(short *)(&DAT_001df9e2 + uVar2 * 4) != 0) {
      DAT_001dda40 = DAT_001dda40 + ((int)uVar1 * 3 + 0x11);
      return uVar1 & 0xffffffff;
    }
    if (uVar1 - 1 == 2) break;
    uVar2 = (ulong)(byte)(&UNK_0011340f)[uVar1];
    uVar1 = uVar1 - 1;
  }
  DAT_001dda40 = DAT_001dda40 + 0x17;
  return 2;
}




// Function: send_tree @ 0xa960

void send_tree(long param_1,int param_2)

{
  ushort uVar1;
  int iVar2;
  int iVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  uint uVar7;
  long lVar8;
  int iVar9;
  
  uVar1 = *(ushort *)(param_1 + 2);
  iVar6 = 4 - (uint)(uVar1 == 0);
  iVar2 = (-(uint)(uVar1 == 0) & 0x83) + 7;
  if (-1 < param_2) {
    lVar8 = 0;
    iVar3 = 0;
    uVar7 = 0xffffffff;
    uVar4 = (uint)uVar1;
    do {
      iVar9 = iVar3 + 1;
      uVar1 = *(ushort *)(param_1 + 6 + lVar8 * 4);
      uVar5 = (uint)uVar1;
      if (iVar9 < iVar2) {
        if (uVar5 != uVar4) {
          if (iVar6 <= iVar9) goto LAB_0010aa30;
          do {
            FUN_00104290((&DAT_001df9e0)[(long)(int)uVar4 * 2],
                         *(undefined2 *)(&DAT_001df9e2 + (long)(int)uVar4 * 4));
            iVar9 = iVar9 + -1;
          } while (iVar9 != 0);
          if (uVar1 != 0) goto LAB_0010a9fe;
          goto LAB_0010aa98;
        }
      }
      else {
LAB_0010aa30:
        if (uVar4 == 0) {
          if (iVar9 < 0xb) {
            FUN_00104290(DAT_001dfa24,DAT_001dfa26);
            FUN_00104290(iVar3 + -2,3);
          }
          else {
            FUN_00104290(DAT_001dfa28,DAT_001dfa2a);
            FUN_00104290(iVar3 + -10,7);
          }
        }
        else {
          if (uVar7 != uVar4) {
            FUN_00104290((&DAT_001df9e0)[(long)(int)uVar4 * 2],
                         *(undefined2 *)(&DAT_001df9e2 + (long)(int)uVar4 * 4));
            iVar9 = iVar3;
          }
          FUN_00104290(DAT_001dfa20,DAT_001dfa22);
          FUN_00104290(iVar9 + -3,2);
        }
        if (uVar1 == 0) {
LAB_0010aa98:
          iVar6 = 3;
          iVar2 = 0x8a;
          iVar9 = 0;
          uVar7 = uVar4;
        }
        else if (uVar5 == uVar4) {
          iVar6 = 3;
          iVar2 = 6;
          iVar9 = 0;
          uVar7 = uVar5;
        }
        else {
LAB_0010a9fe:
          iVar6 = 4;
          iVar2 = 7;
          iVar9 = 0;
          uVar7 = uVar4;
        }
      }
      iVar3 = iVar9;
      lVar8 = lVar8 + 1;
      uVar4 = uVar5;
    } while ((int)lVar8 <= param_2);
  }
  return;
}




// Function: send_all_trees @ 0xab20

void send_all_trees(int param_1,int param_2,int param_3)

{
  ulong uVar1;
  byte *pbVar2;
  
  FUN_00104290(param_1 + -0x101,5);
  FUN_00104290(param_2 + -1,5);
  FUN_00104290(param_3 + -4,4);
  if (0 < param_3) {
    pbVar2 = &DAT_00113411;
    uVar1 = 0x10;
    while( true ) {
      FUN_00104290(*(undefined2 *)(&DAT_001df9e2 + uVar1 * 4),3);
      if (pbVar2 == &DAT_00113411 + (param_3 - 1)) break;
      uVar1 = (ulong)*pbVar2;
      pbVar2 = pbVar2 + 1;
    }
  }
  FUN_0010a960(&DAT_001e0080,param_1 + -1);
  FUN_0010a960(&DAT_001dff80,param_2 + -1);
  return;
}




// Function: compress_block @ 0xabd0

void compress_block(long param_1,long param_2)

{
  undefined2 *puVar1;
  byte bVar2;
  byte bVar3;
  ushort uVar4;
  ulong uVar5;
  byte bVar6;
  uint uVar7;
  ulong uVar8;
  ulong uVar9;
  ulong uVar10;
  
  if (DAT_001dda54 != 0) {
    bVar6 = 0;
    uVar8 = 0;
    uVar5 = 0;
    uVar9 = 0;
    while( true ) {
      uVar10 = uVar9;
      if ((uVar5 & 7) == 0) {
        uVar10 = (ulong)((int)uVar9 + 1);
        bVar6 = (&DAT_001dda60)[uVar9];
      }
      uVar7 = (int)uVar5 + 1;
      bVar2 = (&DAT_0019d000)[uVar5];
      if ((bVar6 & 1) == 0) {
        puVar1 = (undefined2 *)(param_1 + (ulong)bVar2 * 4);
        FUN_00104290(*puVar1,puVar1[1]);
      }
      else {
        bVar3 = (&DAT_001ded60)[bVar2];
        puVar1 = (undefined2 *)(param_1 + (ulong)(bVar3 + 0x101) * 4);
        FUN_00104290(*puVar1,puVar1[1]);
        if ((&DAT_001192c0)[bVar3] != 0) {
          FUN_00104290((uint)bVar2 - (&DAT_001deae0)[bVar3]);
        }
        uVar4 = (&DAT_0014c000)[uVar8];
        if (uVar4 < 0x100) {
          bVar2 = (&DAT_001deb60)[(uint)uVar4];
        }
        else {
          bVar2 = (&DAT_001deb60)[(uVar4 >> 7) + 0x100];
        }
        uVar5 = (ulong)bVar2;
        puVar1 = (undefined2 *)(param_2 + uVar5 * 4);
        FUN_00104290(*puVar1,puVar1[1]);
        uVar8 = (ulong)((int)uVar8 + 1);
        if ((&DAT_00119240)[uVar5] != 0) {
          FUN_00104290((uint)uVar4 - (&DAT_001dea60)[uVar5]);
        }
      }
      if (DAT_001dda54 <= uVar7) break;
      bVar6 = bVar6 >> 1;
      uVar5 = (ulong)uVar7;
      uVar9 = uVar10;
    }
  }
  FUN_00104290(*(undefined2 *)(param_1 + 0x400),*(undefined2 *)(param_1 + 0x402));
  return;
}




// Function: ct_init @ 0xad80

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void ct_init(undefined8 param_1)

{
  undefined2 uVar1;
  undefined1 *puVar2;
  undefined2 *puVar3;
  undefined2 *puVar4;
  ulong uVar5;
  long lVar6;
  int iVar7;
  int iVar8;
  
  DAT_001dda20 = param_1;
  _DAT_001dda28 = 0;
  DAT_001dda30 = 0;
  if (DAT_001dfa82 == 0) {
    lVar6 = 0;
    iVar7 = 0;
    do {
      (&DAT_001deae0)[lVar6] = iVar7;
      iVar8 = 1 << ((byte)(&DAT_001192c0)[lVar6] & 0x1f);
      if (0 < iVar8) {
        puVar2 = &DAT_001ded60 + iVar7;
        do {
          *puVar2 = (char)lVar6;
          puVar2 = puVar2 + 1;
        } while (puVar2 != &DAT_001ded61 + (ulong)(iVar8 - 1) + (long)iVar7);
        iVar7 = iVar7 + iVar8;
      }
      lVar6 = lVar6 + 1;
    } while (lVar6 != 0x1c);
    lVar6 = 0;
    iVar8 = 0;
    (&DAT_001ded60)[iVar7 + -1] = 0x1c;
    do {
      (&DAT_001dea60)[lVar6] = iVar8;
      iVar7 = 1 << ((byte)(&DAT_00119240)[lVar6] & 0x1f);
      if (0 < iVar7) {
        puVar2 = &DAT_001deb60 + iVar8;
        do {
          *puVar2 = (char)lVar6;
          puVar2 = puVar2 + 1;
        } while (puVar2 != &DAT_001deb61 + (ulong)(iVar7 - 1) + (long)iVar8);
        iVar8 = iVar8 + iVar7;
      }
      lVar6 = lVar6 + 1;
    } while (lVar6 != 0x10);
    iVar8 = iVar8 >> 7;
    lVar6 = 0x10;
    do {
      (&DAT_001dea60)[lVar6] = iVar8 << 7;
      iVar7 = 1 << ((char)(&DAT_00119240)[lVar6] - 7U & 0x1f);
      if (0 < iVar7) {
        puVar2 = &DAT_001deb60 + iVar8;
        do {
          puVar2[0x100] = (char)lVar6;
          puVar2 = puVar2 + 1;
        } while (puVar2 != &DAT_001deb61 + (ulong)(iVar7 - 1) + (long)iVar8);
        iVar8 = iVar8 + iVar7;
      }
      lVar6 = lVar6 + 1;
    } while (lVar6 != 0x1e);
    puVar4 = &DAT_001df9c0;
    do {
      puVar3 = puVar4 + 1;
      *puVar4 = 0;
      puVar4 = puVar3;
    } while (puVar3 != &DAT_001df9e0);
    puVar4 = &DAT_001dfb02;
    do {
      puVar3 = puVar4 + 2;
      *puVar4 = 8;
      puVar4 = puVar3;
    } while (puVar3 != &DAT_001dfd42);
    puVar4 = &DAT_001dfd42;
    do {
      puVar3 = puVar4 + 2;
      *puVar4 = 9;
      puVar4 = puVar3;
    } while (puVar3 != &DAT_001dff02);
    DAT_001df9d2 = DAT_001df9d2 + 0x70;
    puVar4 = &DAT_001dff02;
    do {
      puVar3 = puVar4 + 2;
      *puVar4 = 7;
      puVar4 = puVar3;
    } while (puVar3 != (undefined2 *)&DAT_001dff62);
    DAT_001df9ce = DAT_001df9ce + 0x18;
    _DAT_001dff66 = 8;
    DAT_001df9d0 = DAT_001df9d0 + 0x98;
    _DAT_001dff6a = 8;
    _DAT_001dff7e = 8;
    uVar5 = 0;
    _DAT_001dff62 = 8;
    _DAT_001dff6e = 8;
    _DAT_001dff72 = 8;
    _DAT_001dff76 = 8;
    _DAT_001dff7a = 8;
    FUN_0010a500(&DAT_001dfb00,0x11f);
    do {
      (&DAT_001dfa82)[uVar5 * 2] = 5;
      uVar1 = FUN_00104380(uVar5 & 0xffffffff,5);
      (&DAT_001dfa80)[uVar5 * 2] = uVar1;
      uVar5 = uVar5 + 1;
    } while (uVar5 != 0x1e);
    FUN_00109f30();
    return;
  }
  return;
}




// Function: flush_block @ 0xb070

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

long flush_block(long param_1,ulong param_2,int param_3,int param_4)

{
  int iVar1;
  ulong uVar2;
  ulong uVar3;
  ulong uVar4;
  
  (&DAT_001dda60)[DAT_001dda4c] = DAT_001dda49;
  if (*DAT_001dda20 == -1) {
    FUN_0010a480();
  }
  FUN_0010a5a0(&PTR_DAT_001193c0);
  FUN_0010a5a0(&PTR_DAT_00119380);
  iVar1 = FUN_0010a8c0();
  _DAT_001dda28 = _DAT_001dda28 + param_2;
  uVar4 = DAT_001dda40 + 10U >> 3;
  uVar3 = DAT_001dda38 + 10U >> 3;
  uVar2 = uVar3;
  if (uVar4 <= uVar3) {
    uVar2 = uVar4;
  }
  if ((uVar2 < param_2 + 4) || (param_1 == 0)) {
    if (uVar4 < uVar3) {
      FUN_00104290(param_4 + 4,3);
      FUN_0010ab20(DAT_001193e4 + 1,DAT_001193a4 + 1,iVar1 + 1);
      FUN_0010abd0(&DAT_001e0080,&DAT_001dff80);
      DAT_001dda30 = DAT_001dda30 + DAT_001dda40 + 3;
    }
    else {
      FUN_00104290(param_4 + 2,3);
      FUN_0010abd0(&DAT_001dfb00,&DAT_001dfa80);
      DAT_001dda30 = DAT_001dda30 + DAT_001dda38 + 3;
    }
    FUN_00109f30();
  }
  else {
    FUN_00104290(param_4,3);
    DAT_001dda30 = (DAT_001dda30 + 10 & 0xfffffffffffffff8) + (param_2 + 4) * 8;
    FUN_001044a0(param_1,param_2 & 0xffffffff,1);
    FUN_00109f30();
  }
  if (param_4 == 0) {
    if ((param_3 != 0) && ((DAT_001dda30 & 7) != 0)) {
      FUN_00104290(0,3);
      DAT_001dda30 = DAT_001dda30 + 10 & 0xfffffffffffffff8;
      FUN_001044a0(param_1,0,1);
    }
  }
  else {
    FUN_001043b0();
    DAT_001dda30 = DAT_001dda30 + 7;
  }
  return (long)DAT_001dda30 >> 3;
}




// Function: ct_tally @ 0xb290

bool ct_tally(int param_1,int param_2)

{
  int *piVar1;
  ushort *puVar2;
  byte bVar3;
  ulong uVar4;
  long lVar5;
  uint uVar6;
  
  uVar4 = (ulong)DAT_001dda54;
  uVar6 = DAT_001dda54 + 1;
  DAT_001dda54 = uVar6;
  (&DAT_0019d000)[uVar4] = (char)param_2;
  if (param_1 == 0) {
    (&DAT_001e0080)[(long)param_2 * 2] = (&DAT_001e0080)[(long)param_2 * 2] + 1;
  }
  else {
    param_1 = param_1 + -1;
    *(short *)(&DAT_001e0484 + (ulong)(byte)(&DAT_001ded60)[param_2] * 4) =
         *(short *)(&DAT_001e0484 + (ulong)(byte)(&DAT_001ded60)[param_2] * 4) + 1;
    if (param_1 < 0x100) {
      bVar3 = (&DAT_001deb60)[param_1];
    }
    else {
      bVar3 = (&DAT_001deb60)[(param_1 >> 7) + 0x100];
    }
    (&DAT_001dff80)[(ulong)bVar3 * 2] = (&DAT_001dff80)[(ulong)bVar3 * 2] + 1;
    uVar4 = (ulong)DAT_001dda50;
    DAT_001dda50 = DAT_001dda50 + 1;
    (&DAT_0014c000)[uVar4] = (short)param_1;
    DAT_001dda49 = DAT_001dda49 | DAT_001dda48;
  }
  bVar3 = DAT_001dda49;
  DAT_001dda48 = DAT_001dda48 * '\x02';
  if ((uVar6 & 7) == 0) {
    uVar4 = (ulong)DAT_001dda4c;
    DAT_001dda48 = '\x01';
    DAT_001dda49 = 0;
    DAT_001dda4c = DAT_001dda4c + 1;
    (&DAT_001dda60)[uVar4] = bVar3;
  }
  if ((2 < DAT_0011901c) && ((uVar6 & 0xfff) == 0)) {
    lVar5 = 0;
    uVar4 = (ulong)uVar6 << 3;
    do {
      piVar1 = (int *)((long)&DAT_00119240 + lVar5);
      puVar2 = (ushort *)((long)&DAT_001dff80 + lVar5);
      lVar5 = lVar5 + 4;
      uVar4 = uVar4 + ((long)*piVar1 + 5) * (ulong)*puVar2;
    } while (lVar5 != 0x78);
    if ((DAT_001dda50 < uVar6 >> 1) && (uVar4 >> 3 < (ulong)DAT_0011a06c - DAT_0011a078 >> 1)) {
      return true;
    }
  }
  return uVar6 == 0x7fff || DAT_001dda50 == 0x8000;
}




// Function: fillbuf @ 0xb450

void fillbuf(int param_1)

{
  ulong uVar1;
  int iVar2;
  ushort uVar3;
  uint uVar4;
  
  uVar4 = (uint)DAT_001e0994 << ((byte)param_1 & 0x1f);
  uVar3 = (ushort)uVar4;
  iVar2 = DAT_001e098c;
  if (DAT_001e098c < param_1) {
    do {
      param_1 = param_1 - DAT_001e098c;
      uVar4 = uVar4 | DAT_001e0990 << ((byte)param_1 & 0x1f);
      uVar1 = (ulong)DAT_0011b008;
      DAT_001e0994 = (ushort)uVar4;
      if (DAT_0011b008 < DAT_0011b00c) {
        DAT_0011b008 = DAT_0011b008 + 1;
        DAT_001e0990 = (uint)(byte)(&DAT_0019d000)[uVar1];
      }
      else {
        DAT_001e0990 = FUN_0010d830(1);
        if (DAT_001e0990 == 0xffffffff) {
          DAT_001e0990 = 0;
          uVar4 = (uint)DAT_001e0994;
        }
        else {
          uVar4 = (uint)DAT_001e0994;
        }
      }
      uVar3 = (ushort)uVar4;
      DAT_001e098c = 8;
      iVar2 = 8;
    } while (8 < param_1);
  }
  DAT_001e098c = iVar2 - param_1;
  DAT_001e0994 = uVar3 | (ushort)(DAT_001e0990 >> ((byte)(iVar2 - param_1) & 0x1f));
  return;
}




// Function: init_getbits @ 0xb520

void init_getbits(void)

{
  DAT_001e0990 = 0;
  DAT_001e0994 = 0;
  DAT_001e098c = 0;
  FUN_0010b450(0x10);
  return;
}




// Function: huf_decode_start @ 0xb550

void huf_decode_start(void)

{
  FUN_0010b520();
  DAT_001e0ba0 = 0;
  return;
}




// Function: decode_start @ 0xb570

void decode_start(void)

{
  FUN_0010b550();
  DAT_001e0988 = 0;
  DAT_001e0984 = 0;
  return;
}




// Function: getbits @ 0xb5a0

int getbits(char param_1)

{
  uint uVar1;
  
  uVar1 = (uint)DAT_001e0994;
  FUN_0010b450();
  return (int)uVar1 >> (0x10U - param_1 & 0x1f);
}




// Function: decode_p @ 0xb5c0

ulong decode_p(void)

{
  ushort *puVar1;
  uint uVar2;
  int iVar3;
  int iVar4;
  ulong uVar5;
  
  uVar5 = (ulong)(ushort)(&DAT_001e09a0)[(byte)(DAT_001e0994 >> 8)];
  if (0xd < (ushort)(&DAT_001e09a0)[(byte)(DAT_001e0994 >> 8)]) {
    uVar2 = 0x80;
    do {
      while ((uVar2 & DAT_001e0994) == 0) {
        puVar1 = &DAT_0011b880 + uVar5;
        uVar5 = (ulong)*puVar1;
        uVar2 = uVar2 >> 1;
        if (*puVar1 < 0xe) goto LAB_0010b61e;
      }
      iVar3 = (int)uVar5;
      uVar2 = uVar2 >> 1;
      uVar5 = (ulong)(ushort)(&DAT_0011b880)[iVar3 + 0x8000];
    } while (0xd < (ushort)(&DAT_0011b880)[iVar3 + 0x8000]);
  }
LAB_0010b61e:
  FUN_0010b450((&DAT_001e0bc0)[uVar5]);
  if ((int)uVar5 == 0) {
    return uVar5;
  }
  iVar4 = (int)uVar5 + -1;
  iVar3 = FUN_0010b5a0(iVar4);
  return (ulong)(uint)((1 << ((byte)iVar4 & 0x1f)) + iVar3);
}




// Function: make_table @ 0xb670

void make_table(uint param_1,byte *param_2,uint param_3,long param_4)

{
  byte bVar1;
  ushort uVar2;
  ushort uVar3;
  char cVar4;
  uint uVar5;
  ushort *puVar6;
  byte *pbVar7;
  long lVar8;
  ushort *puVar9;
  ulong uVar10;
  byte bVar11;
  ulong uVar12;
  uint uVar13;
  ulong uVar14;
  ulong uVar15;
  int iVar16;
  uint uVar17;
  long in_FS_OFFSET;
  ushort local_c8 [17];
  ushort local_a6 [7];
  ushort local_98 [24];
  ushort local_68 [20];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  puVar9 = local_c8 + 1;
  do {
    puVar6 = puVar9 + 1;
    *puVar9 = 0;
    puVar9 = puVar6;
  } while (puVar6 != local_a6);
  uVar12 = (ulong)param_1;
  pbVar7 = param_2;
  do {
    bVar1 = *pbVar7;
    pbVar7 = pbVar7 + 1;
    local_c8[bVar1] = local_c8[bVar1] + 1;
  } while (pbVar7 != param_2 + (ulong)(param_1 - 1) + 1);
  iVar16 = 0;
  local_68[1] = 0;
  lVar8 = 1;
  do {
    iVar16 = iVar16 + ((uint)local_c8[lVar8] << (0x10U - (char)lVar8 & 0x1f));
    local_68[lVar8 + 1] = (ushort)iVar16;
    lVar8 = lVar8 + 1;
  } while (lVar8 != 0x11);
  if (local_68[0x11] == 0) {
    lVar8 = 1;
    bVar1 = (byte)param_3;
    bVar11 = 0x10 - bVar1;
    uVar2 = 0;
    while( true ) {
      local_68[lVar8] = (ushort)((int)(uint)uVar2 >> (bVar11 & 0x1f));
      local_98[lVar8] = (ushort)(1 << (bVar1 - (char)lVar8 & 0x1f));
      lVar8 = lVar8 + 1;
      if (param_3 < (uint)lVar8) break;
      uVar2 = local_68[lVar8];
    }
    uVar5 = param_3 + 1;
    do {
      uVar15 = (ulong)uVar5;
      cVar4 = (char)uVar5;
      uVar5 = uVar5 + 1;
      local_98[uVar15] = (ushort)(1 << (0x10U - cVar4 & 0x1f));
    } while (uVar5 != 0x11);
    uVar5 = (int)(uint)local_68[(int)(param_3 + 1)] >> (bVar11 & 0x1f);
    if (uVar5 != 0) {
      for (; uVar5 != 1 << (bVar1 & 0x1f); uVar5 = uVar5 + 1) {
        *(undefined2 *)(param_4 + (ulong)uVar5 * 2) = 0;
      }
    }
    uVar15 = 0;
    do {
      if (param_2[uVar15] != 0) {
        uVar5 = (uint)param_2[uVar15];
        uVar2 = local_68[uVar5];
        uVar14 = (ulong)uVar2;
        uVar3 = local_98[uVar5];
        if (param_3 < uVar5) {
          puVar9 = (ushort *)(param_4 + (ulong)(uVar2 >> (bVar11 & 0x1f)) * 2);
          for (iVar16 = uVar5 - param_3; iVar16 != 0; iVar16 = iVar16 + -1) {
            while( true ) {
              uVar10 = (ulong)*puVar9;
              if (*puVar9 == 0) {
                (&DAT_0011b880)[uVar12] = 0;
                (&DAT_0012b880)[uVar12] = 0;
                uVar10 = uVar12 & 0xffff;
                *puVar9 = (ushort)uVar12;
                uVar12 = (ulong)((int)uVar12 + 1);
              }
              uVar13 = (uint)uVar14;
              if ((1 << (0xf - bVar1 & 0x1f) & uVar13) != 0) break;
              puVar9 = &DAT_0011b880 + uVar10;
              uVar14 = (ulong)(uVar13 * 2);
              iVar16 = iVar16 + -1;
              if (iVar16 == 0) goto LAB_0010b932;
            }
            puVar9 = &DAT_0012b880 + uVar10;
            uVar14 = (ulong)(uVar13 * 2);
          }
LAB_0010b932:
          *puVar9 = (ushort)uVar15;
        }
        else {
          uVar17 = (uint)uVar2;
          uVar13 = uVar3 + uVar17;
          if ((uint)(1 << (bVar1 & 0x1f)) < uVar13) break;
          if (uVar17 < uVar13) {
            puVar9 = (ushort *)(param_4 + uVar14 * 2);
            do {
              *puVar9 = (ushort)uVar15;
              puVar9 = puVar9 + 1;
            } while ((ushort *)(param_4 + 2 + (((uVar13 - uVar17) - 1) + uVar14) * 2) != puVar9);
          }
        }
        local_68[uVar5] = uVar2 + uVar3;
      }
      uVar15 = uVar15 + 1;
      if (param_1 == uVar15) {
        if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
          return;
        }
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
    } while( true );
  }
                    /* WARNING: Subroutine does not return */
  FUN_0010d510("Bad table\n");
}




// Function: read_c_len @ 0xb980

void read_c_len(void)

{
  ushort uVar1;
  int iVar2;
  undefined2 uVar3;
  int iVar4;
  uint uVar5;
  int iVar6;
  undefined1 *puVar7;
  undefined2 *puVar8;
  ulong uVar9;
  int iVar10;
  long lVar11;
  
  iVar4 = FUN_0010b5a0(9);
  if (iVar4 == 0) {
    uVar3 = FUN_0010b5a0(9);
    puVar7 = &DAT_0015c000;
    do {
      *puVar7 = 0;
      puVar7 = puVar7 + 1;
    } while (puVar7 != &DAT_0015c1fe);
    puVar8 = &DAT_0014c000;
    do {
      *puVar8 = uVar3;
      puVar8 = puVar8 + 1;
    } while (puVar8 != (undefined2 *)&DAT_0014e000);
    return;
  }
  iVar10 = 0;
  if (0 < iVar4) {
    do {
      while( true ) {
        uVar9 = (ulong)(ushort)(&DAT_001e09a0)[(byte)(DAT_001e0994 >> 8)];
        if (0x12 < (ushort)(&DAT_001e09a0)[(byte)(DAT_001e0994 >> 8)]) {
          uVar5 = 0x80;
          do {
            if ((uVar5 & DAT_001e0994) == 0) {
              uVar1 = (&DAT_0011b880)[uVar9];
            }
            else {
              uVar1 = (&DAT_0011b880)[(int)uVar9 + 0x8000];
            }
            uVar9 = (ulong)uVar1;
            uVar5 = uVar5 >> 1;
          } while (0x12 < uVar1);
        }
        uVar5 = (uint)uVar9;
        FUN_0010b450((&DAT_001e0bc0)[(int)uVar5]);
        if (uVar5 < 3) break;
        lVar11 = (long)iVar10;
        iVar10 = iVar10 + 1;
        (&DAT_0015c000)[lVar11] = (char)uVar9 + -2;
LAB_0010b9ee:
        if (iVar4 <= iVar10) goto LAB_0010ba89;
      }
      iVar6 = 0;
      if (uVar5 != 0) {
        if (uVar5 == 1) {
          iVar6 = FUN_0010b5a0(4);
          iVar6 = iVar6 + 3;
        }
        else {
          iVar6 = FUN_0010b5a0(9);
          iVar6 = iVar6 + 0x14;
        }
        iVar6 = iVar6 + -1;
        if (iVar6 < 0) goto LAB_0010b9ee;
      }
      puVar7 = &DAT_0015c000 + iVar10;
      iVar2 = (int)puVar7;
      do {
        *puVar7 = 0;
        puVar7 = puVar7 + 1;
      } while (-1 < (iVar6 + iVar2) - (int)puVar7);
      iVar10 = iVar10 + 1 + iVar6;
    } while (iVar10 < iVar4);
LAB_0010ba89:
    if (0x1fd < iVar10) goto LAB_0010baaa;
  }
  lVar11 = (long)iVar10;
  do {
    (&DAT_0015c000)[lVar11] = 0;
    lVar11 = lVar11 + 1;
  } while ((int)lVar11 < 0x1fe);
LAB_0010baaa:
  FUN_0010b670(0x1fe,&DAT_0015c000,0xc,&DAT_0014c000);
  return;
}




// Function: read_pt_len @ 0xbb40

void read_pt_len(int param_1,undefined4 param_2,int param_3)

{
  undefined2 uVar1;
  int iVar2;
  int iVar3;
  long lVar4;
  undefined4 *puVar5;
  ulong uVar6;
  undefined1 *puVar7;
  uint uVar8;
  int iVar9;
  byte bVar10;
  byte *pbVar11;
  byte *pbVar12;
  int iVar13;
  
  iVar2 = FUN_0010b5a0(param_2);
  if (iVar2 == 0) {
    uVar1 = FUN_0010b5a0(param_2);
    puVar7 = &DAT_001e0bc0;
    do {
      *puVar7 = 0;
      puVar7 = puVar7 + 1;
    } while (puVar7 != &DAT_001e0bc1 + (param_1 - 1));
    puVar5 = (undefined4 *)&DAT_001e09a0;
    do {
      *(undefined2 *)puVar5 = uVar1;
      puVar5 = (undefined4 *)((long)puVar5 + 2);
    } while (puVar5 != &DAT_001e0ba0);
    return;
  }
  iVar13 = 0;
  do {
    uVar8 = (param_3 + -1) - iVar13;
    lVar4 = (long)iVar13;
    pbVar12 = &DAT_001e0bc0 + iVar13;
    do {
      if (iVar2 <= iVar13) {
        if (iVar13 < param_1) {
          puVar7 = &DAT_001e0bc0 + iVar13;
          do {
            *puVar7 = 0;
            puVar7 = puVar7 + 1;
          } while (puVar7 != &DAT_001e0bc1 + (ulong)(uint)((param_1 + -1) - iVar13) + (long)iVar13);
        }
        FUN_0010b670(param_1,&DAT_001e0bc0,8,&DAT_001e09a0);
        return;
      }
      iVar9 = 3;
      bVar10 = (byte)(DAT_001e0994 >> 0xd);
      if (DAT_001e0994 >> 0xd == 7) {
        if ((DAT_001e0994 & 0x1000) == 0) {
          bVar10 = 7;
          iVar9 = 4;
        }
        else {
          uVar6 = 0x1000;
          iVar3 = 7;
          do {
            iVar9 = iVar3;
            uVar6 = uVar6 >> 1;
            iVar3 = iVar9 + 1;
          } while (((uint)DAT_001e0994 & (uint)uVar6) != 0);
          if (0x10 < iVar3) {
                    /* WARNING: Subroutine does not return */
            FUN_0010d510("Bad table\n");
          }
          iVar9 = iVar9 + -2;
          bVar10 = (byte)iVar3;
        }
      }
      FUN_0010b450(iVar9);
      pbVar11 = pbVar12 + 1;
      *pbVar12 = bVar10;
      iVar13 = iVar13 + 1;
      pbVar12 = pbVar11;
    } while (pbVar11 != &DAT_001e0bc1 + (ulong)uVar8 + lVar4);
    iVar9 = FUN_0010b5a0(2);
    if (0 < iVar9) {
      puVar7 = &DAT_001e0bc0 + iVar13;
      do {
        *puVar7 = 0;
        puVar7 = puVar7 + 1;
      } while (puVar7 != &DAT_001e0bc1 + (ulong)(iVar9 - 1) + (long)iVar13);
      iVar13 = iVar13 + iVar9;
    }
  } while( true );
}




// Function: decode_c @ 0xbd50

ulong decode_c(void)

{
  ushort *puVar1;
  uint uVar2;
  int iVar3;
  ulong uVar4;
  
  if (DAT_001e0ba0 == 0) {
    DAT_001e0ba0 = FUN_0010b5a0(0x10);
    if (DAT_001e0ba0 == 0) {
      return 0x1fe;
    }
    FUN_0010bb40(0x13,5,3);
    FUN_0010b980();
    FUN_0010bb40(0xe,4,0xffffffff);
  }
  DAT_001e0ba0 = DAT_001e0ba0 + -1;
  uVar4 = (ulong)(ushort)(&DAT_0014c000)[DAT_001e0994 >> 4];
  if (0x1fd < (ushort)(&DAT_0014c000)[DAT_001e0994 >> 4]) {
    uVar2 = 8;
    do {
      while ((uVar2 & DAT_001e0994) != 0) {
        iVar3 = (int)uVar4;
        uVar2 = uVar2 >> 1;
        uVar4 = (ulong)(ushort)(&DAT_0011b880)[iVar3 + 0x8000];
        if ((ushort)(&DAT_0011b880)[iVar3 + 0x8000] < 0x1fe) goto LAB_0010bdcb;
      }
      puVar1 = &DAT_0011b880 + uVar4;
      uVar4 = (ulong)*puVar1;
      uVar2 = uVar2 >> 1;
    } while (0x1fd < *puVar1);
  }
LAB_0010bdcb:
  FUN_0010b450((&DAT_0015c000)[uVar4]);
  return uVar4;
}




// Function: unlzh @ 0xbfd0

undefined8 unlzh(undefined4 param_1,undefined4 param_2)

{
  int iVar1;
  
  DAT_0011b010 = param_2;
  DAT_0011b014 = param_1;
  FUN_0010b570();
  while (DAT_001e0984 == 0) {
    iVar1 = FUN_0010be40(0x2000,&DAT_0013c000);
    if (iVar1 != 0) {
      FUN_0010d6c0(param_2,&DAT_0013c000,iVar1);
    }
  }
  return 0;
}




// Function: unlzw @ 0xc040

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 unlzw(undefined4 param_1,undefined4 param_2)

{
  byte bVar1;
  long lVar2;
  undefined1 *puVar3;
  ulong uVar4;
  int iVar5;
  long lVar6;
  long lVar7;
  uint uVar8;
  undefined1 *puVar9;
  undefined8 *puVar10;
  char *pcVar11;
  uint uVar12;
  ulong uVar13;
  int iVar14;
  ulong uVar15;
  ulong uVar16;
  bool bVar17;
  byte bVar18;
  long local_a0;
  undefined2 local_98;
  uint local_8c;
  long local_88;
  long local_70;
  uint local_48;
  uint local_44;
  uint local_40;
  
  bVar18 = 0;
  uVar4 = (ulong)DAT_0011b008;
  if (DAT_0011b008 < DAT_0011b00c) {
    DAT_0011b008 = DAT_0011b008 + 1;
    DAT_00119024 = (uint)(byte)(&DAT_0019d000)[uVar4];
  }
  else {
    DAT_00119024 = FUN_0010d830(0);
  }
  DAT_001193e8 = DAT_00119024 & 0x80;
  if ((DAT_00119024 & 0x60) != 0) {
    if (DAT_0011b85c == 0) {
      FUN_0010f630(stderr,"\n%s: %s: warning, unknown flags 0x%x\n",DAT_0011b850,&DAT_0011b420);
    }
    if (DAT_0011b84c == 0) {
      DAT_0011b84c = 2;
    }
  }
  DAT_00119024 = DAT_00119024 & 0x1f;
  lVar2 = 1L << (sbyte)DAT_00119024;
  if (0x10 < DAT_00119024) {
    FUN_0010f630(stderr,"\n%s: %s: compressed with %d bits, can only handle %d bits\n",DAT_0011b850,
                 &DAT_0011b420,DAT_00119024,0x10);
    DAT_0011b84c = 1;
    return 1;
  }
  _DAT_0011b880 = 0;
  uVar4 = (ulong)DAT_0011b00c;
  _DAT_0011b978 = 0;
  uVar13 = (ulong)(DAT_0011b008 * 8);
  local_a0 = 0x101 - (ulong)(DAT_001193e8 == 0);
  local_44 = DAT_0011b00c;
  puVar10 = &DAT_0011b888;
  for (lVar6 = 0x1f; lVar6 != 0; lVar6 = lVar6 + -1) {
    *puVar10 = 0;
    puVar10 = puVar10 + (ulong)bVar18 * -2 + 1;
  }
  lVar6 = 0xff;
  do {
    (&DAT_0013c000)[lVar6] = (char)lVar6;
    bVar17 = lVar6 != 0;
    lVar6 = lVar6 + -1;
  } while (bVar17);
  local_48 = 9;
  iVar14 = 0;
  local_88 = 0x1ff;
  local_40 = 0x1ff;
  local_8c = 0;
  uVar15 = 0xffffffffffffffff;
LAB_0010c19c:
  uVar12 = (uint)((long)uVar13 >> 3);
  uVar8 = (uint)uVar4;
  uVar16 = uVar15;
  if (uVar12 <= uVar8) goto LAB_0010c408;
LAB_0010c1a9:
  DAT_0011b00c = 0;
  puVar9 = &DAT_0019d000;
LAB_0010c1ba:
  local_44 = FUN_0010d2b0(param_1,puVar9,0x40000);
  if (local_44 == 0xffffffff) {
                    /* WARNING: Subroutine does not return */
    FUN_0010d600();
  }
  DAT_0011b828 = DAT_0011b828 + (int)local_44;
  DAT_0011b00c = DAT_0011b00c + local_44;
  uVar4 = (ulong)DAT_0011b00c;
LAB_0010c1f9:
  uVar12 = local_48;
  uVar8 = (uint)uVar4;
  uVar15 = uVar16;
  if (local_44 == 0) {
    local_70 = uVar4 * 8 - (long)(int)(local_48 - 1);
    if (local_70 < 1) {
LAB_0010c5a5:
      if (iVar14 < 1) {
        return 0;
      }
      FUN_0010d6c0(param_2,&DAT_0015c000,iVar14);
      return 0;
    }
  }
  else {
    uVar13 = 0;
    local_70 = (uVar4 - uVar4 % (ulong)local_48) * 8;
    if (local_70 < 1) goto LAB_0010c19c;
  }
  if (local_88 < local_a0) {
    lVar6 = -1;
    goto LAB_0010c39f;
  }
  uVar13 = 0;
LAB_0010c258:
  lVar6 = (long)uVar13 >> 3;
  bVar1 = (byte)uVar13;
  uVar13 = uVar13 + (long)(int)local_48;
  uVar16 = (long)((ulong)(byte)(&DAT_0019d001)[lVar6] << 8 |
                  (ulong)(byte)(&DAT_0019d002)[lVar6] << 0x10 | (ulong)(byte)(&DAT_0019d000)[lVar6])
           >> (bVar1 & 7) & (ulong)local_40;
  if (uVar15 == 0xffffffffffffffff) {
    if (0xff < uVar16) {
                    /* WARNING: Subroutine does not return */
      FUN_0010d510("corrupt input.");
    }
    lVar6 = (long)iVar14;
    local_8c = (uint)uVar16;
    iVar14 = iVar14 + 1;
    (&DAT_0015c000)[lVar6] = (char)uVar16;
    uVar8 = DAT_0011b00c;
  }
  else {
    if (uVar16 == 0x100) {
      if (DAT_001193e8 != 0) {
        _DAT_0011b880 = 0;
        _DAT_0011b978 = 0;
        puVar10 = &DAT_0011b888;
        for (lVar6 = 0x1f; lVar6 != 0; lVar6 = lVar6 + -1) {
          *puVar10 = 0;
          puVar10 = puVar10 + (ulong)bVar18 * -2 + 1;
        }
        local_40 = 0x1ff;
        local_a0 = 0x100;
        uVar4 = (ulong)DAT_0011b00c;
        local_48 = 9;
        local_88 = 0x1ff;
        lVar6 = (long)(int)(uVar12 << 3);
        uVar13 = (uVar13 - 1) + (lVar6 - (long)((uVar13 - 1) + lVar6) % lVar6);
        goto LAB_0010c19c;
      }
      uVar4 = 0x100;
      puVar9 = &DAT_0015bffe;
      if (local_a0 != 0x100) goto LAB_0010c2e8;
LAB_0010c2c8:
      puVar9 = &DAT_0015bffd;
      DAT_0015bffd = (undefined1)local_8c;
      uVar4 = uVar15;
    }
    else {
      if (local_a0 <= (long)uVar16) {
        if (local_a0 < (long)uVar16) {
          if (0 < iVar14) {
            FUN_0010d6c0(param_2,&DAT_0015c000,iVar14);
          }
          pcVar11 = "corrupt input.";
          if (DAT_0011b860 == 0) {
            pcVar11 = "corrupt input. Use zcat to recover some data.";
          }
                    /* WARNING: Subroutine does not return */
          FUN_0010d510(pcVar11);
        }
        goto LAB_0010c2c8;
      }
      puVar9 = &DAT_0015bffe;
      uVar4 = uVar16;
    }
    for (; 0xff < (long)uVar4; uVar4 = (ulong)(ushort)(&DAT_0011b880)[uVar4]) {
LAB_0010c2e8:
      puVar9 = puVar9 + -1;
      *puVar9 = (&DAT_0013c000)[uVar4];
    }
    bVar1 = (&DAT_0013c000)[uVar4];
    puVar3 = puVar9 + -1;
    puVar9[-1] = bVar1;
    lVar6 = (long)&DAT_0015bffe - (long)puVar3;
    iVar5 = iVar14 + (int)lVar6;
    local_8c = (uint)bVar1;
    if (iVar5 < 0x40000) {
      memcpy(&DAT_0015c000 + iVar14,puVar3,(long)(int)lVar6);
      iVar14 = iVar5;
    }
    else {
      do {
        iVar5 = 0x40000 - iVar14;
        if ((int)lVar6 < 0x40000 - iVar14) {
          iVar5 = (int)lVar6;
        }
        if (0 < iVar5) {
          lVar6 = (long)iVar14;
          iVar14 = iVar14 + iVar5;
          memcpy(&DAT_0015c000 + lVar6,puVar3,(long)iVar5);
        }
        if (0x3ffff < iVar14) {
          FUN_0010d6c0(param_2,&DAT_0015c000,iVar14);
          iVar14 = 0;
        }
        puVar3 = puVar3 + iVar5;
        lVar6 = (long)&DAT_0015bffe - (long)puVar3;
      } while (0 < (int)lVar6);
    }
    uVar8 = DAT_0011b00c;
    if (local_a0 < lVar2) {
      local_98 = (undefined2)uVar15;
      (&DAT_0011b880)[local_a0] = local_98;
      (&DAT_0013c000)[local_a0] = bVar1;
      local_a0 = local_a0 + 1;
      uVar8 = DAT_0011b00c;
    }
  }
  uVar15 = uVar16;
  DAT_0011b00c = uVar8;
  if (local_70 <= (long)uVar13) {
    if (local_44 == 0) goto LAB_0010c5a5;
    uVar4 = (ulong)uVar8;
    goto LAB_0010c19c;
  }
  if (local_88 < local_a0) goto code_r0x0010c395;
  goto LAB_0010c258;
code_r0x0010c395:
  lVar6 = uVar13 - 1;
LAB_0010c39f:
  iVar5 = local_48 * 8;
  local_48 = local_48 + 1;
  lVar7 = (long)iVar5;
  local_88 = lVar2;
  if (DAT_00119024 != local_48) {
    local_88 = (1L << ((byte)local_48 & 0x3f)) + -1;
  }
  local_40 = (1 << ((byte)local_48 & 0x1f)) - 1;
  uVar12 = (uint)(lVar6 + (lVar7 - (lVar7 + lVar6) % lVar7) >> 3);
  if (uVar8 < uVar12) goto LAB_0010c1a9;
LAB_0010c408:
  DAT_0011b00c = uVar8 - uVar12;
  uVar4 = (ulong)DAT_0011b00c;
  if (0 < (int)DAT_0011b00c) {
    puVar9 = &DAT_0019d000;
    do {
      puVar3 = puVar9 + 1;
      *puVar9 = puVar9[(int)uVar12];
      puVar9 = puVar3;
    } while (&DAT_0019d001 + (DAT_0011b00c - 1) != puVar3);
  }
  if (DAT_0011b00c < 0x40) goto code_r0x0010c447;
  goto LAB_0010c1f9;
code_r0x0010c447:
  puVar9 = &DAT_0019d000 + uVar4;
  goto LAB_0010c1ba;
}




// Function: read_byte @ 0xc820

ulong read_byte(void)

{
  ulong uVar1;
  
  uVar1 = (ulong)DAT_0011b008;
  if (DAT_0011b008 < DAT_0011b00c) {
    DAT_0011b008 = DAT_0011b008 + 1;
    return (ulong)(byte)(&DAT_0019d000)[uVar1];
  }
  uVar1 = FUN_0010d830(0);
  if (-1 < (int)uVar1) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  FUN_0010d510("invalid compressed data -- unexpected end of file");
}




// Function: read_tree @ 0xc870

void read_tree(void)

{
  ulong uVar1;
  byte bVar2;
  undefined1 uVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  long lVar7;
  undefined1 *puVar8;
  undefined1 *puVar9;
  bool bVar10;
  
  iVar5 = 4;
  DAT_001e0e88 = 0;
  do {
    uVar1 = DAT_001e0e88;
    bVar2 = FUN_0010c820();
    DAT_001e0e88 = uVar1 << 8 | (ulong)bVar2;
    iVar5 = iVar5 + -1;
  } while (iVar5 != 0);
  bVar2 = FUN_0010c820();
  DAT_001e0e80 = (uint)bVar2;
  if (0x18 < bVar2 - 1) {
                    /* WARNING: Subroutine does not return */
    FUN_0010d510("invalid compressed data -- Huffman code bit length out of range");
  }
  if (DAT_001e0e80 == 0) {
    DAT_001e0c80 = DAT_001e0c80 + 1;
    lVar7 = 0;
  }
  else {
    lVar7 = 1;
    iVar5 = 1;
    iVar6 = 0;
    do {
      bVar2 = FUN_0010c820();
      uVar4 = DAT_001e0e80;
      bVar10 = DAT_001e0e80 == (uint)lVar7;
      (&DAT_001e0c80)[lVar7] = (uint)bVar2;
      if ((int)(iVar5 - (uint)bVar10) < (int)(uint)bVar2) goto LAB_0010c9cf;
      lVar7 = lVar7 + 1;
      iVar6 = iVar6 + (uint)bVar2;
      iVar5 = (iVar5 - (uint)bVar2) * 2 + 1;
    } while ((int)lVar7 <= (int)uVar4);
    if (0xff < iVar6) {
LAB_0010c9cf:
                    /* WARNING: Subroutine does not return */
      FUN_0010d510("too many leaves in Huffman tree");
    }
    lVar7 = (long)(int)uVar4;
    (&DAT_001e0c80)[lVar7] = (&DAT_001e0c80)[lVar7] + 1;
    if (0 < (int)uVar4) {
      lVar7 = 1;
      iVar5 = 0;
      do {
        iVar6 = (&DAT_001e0c80)[lVar7];
        *(int *)(&DAT_001e0d00 + lVar7 * 4) = iVar5;
        if (0 < iVar6) {
          uVar4 = iVar6 - 1;
          puVar8 = &DAT_001e0d80 + iVar5;
          do {
            uVar3 = FUN_0010c820();
            puVar9 = puVar8 + 1;
            *puVar8 = uVar3;
            puVar8 = puVar9;
          } while (puVar9 != &DAT_001e0d81 + (ulong)uVar4 + (long)iVar5);
          iVar5 = iVar5 + 1 + uVar4;
          uVar4 = DAT_001e0e80;
        }
        lVar7 = lVar7 + 1;
      } while ((int)lVar7 <= (int)uVar4);
      lVar7 = (long)(int)uVar4;
    }
  }
  (&DAT_001e0c80)[lVar7] = (&DAT_001e0c80)[lVar7] + 1;
  return;
}




// Function: unpack @ 0xc9f0

undefined8 unpack(undefined4 param_1,undefined4 param_2)

{
  long lVar1;
  int iVar2;
  char cVar3;
  byte bVar4;
  uint uVar5;
  uint uVar6;
  ulong uVar7;
  uint uVar8;
  long lVar9;
  uint uVar10;
  
  DAT_0011b010 = param_2;
  DAT_0011b014 = param_1;
  FUN_0010c870();
  FUN_0010c740();
  DAT_001e0be0 = 0;
  DAT_001e0be8 = 0;
  uVar5 = (1 << ((byte)DAT_001e0bf0 & 0x1f)) - 1;
  iVar2 = (&DAT_001e0c80)[(int)DAT_001e0e80];
  while( true ) {
    for (; uVar7 = DAT_001e0be8, DAT_001e0be0 < (int)DAT_001e0bf0; DAT_001e0be0 = DAT_001e0be0 + 8)
    {
      bVar4 = FUN_0010c820();
      DAT_001e0be8 = uVar7 << 8 | (ulong)bVar4;
    }
    uVar6 = (uint)(DAT_001e0be8 >> ((char)DAT_001e0be0 - (char)DAT_001e0bf0 & 0x3fU)) & uVar5;
    bVar4 = (&DAT_0015c000)[uVar6];
    if (bVar4 == 0) {
      uVar8 = DAT_001e0bf0;
      if (uVar6 < *(uint *)(&DAT_001e0c00 + (long)(int)DAT_001e0bf0 * 4)) {
        lVar9 = (long)(int)(DAT_001e0bf0 + 1);
        uVar10 = uVar5;
        do {
          uVar8 = (uint)lVar9;
          uVar10 = uVar10 * 2 + 1;
          for (; DAT_001e0be0 < (int)uVar8; DAT_001e0be0 = DAT_001e0be0 + 8) {
            bVar4 = FUN_0010c820();
            uVar7 = uVar7 << 8 | (ulong)bVar4;
            DAT_001e0be8 = uVar7;
          }
          cVar3 = (char)lVar9;
          lVar1 = lVar9 * 4;
          lVar9 = lVar9 + 1;
          uVar6 = (uint)(uVar7 >> ((char)DAT_001e0be0 - cVar3 & 0x3fU)) & uVar10;
        } while (uVar6 < *(uint *)(&DAT_001e0c00 + lVar1));
      }
    }
    else {
      uVar6 = uVar6 >> ((char)DAT_001e0bf0 - bVar4 & 0x1f);
      uVar8 = (uint)bVar4;
    }
    if ((uVar6 == iVar2 - 1U) && (DAT_001e0e80 == uVar8)) break;
    uVar7 = (ulong)DAT_0011b004;
    uVar10 = DAT_0011b004 + 1;
    DAT_0011b004 = uVar10;
    (&DAT_0013c000)[uVar7] = (&DAT_001e0d80)[uVar6 + *(int *)(&DAT_001e0d00 + (long)(int)uVar8 * 4)]
    ;
    if (uVar10 == 0x8000) {
      FUN_0010d7e0();
    }
    DAT_001e0be0 = DAT_001e0be0 - uVar8;
  }
  FUN_0010d7e0();
  if (DAT_0011b820 == DAT_001e0e88) {
    return 0;
  }
                    /* WARNING: Subroutine does not return */
  FUN_0010d510("invalid compressed data--length error");
}




// Function: check_zipfile @ 0xcc20

undefined8 check_zipfile(undefined4 param_1)

{
  byte bVar1;
  ulong uVar2;
  char *pcVar3;
  
  uVar2 = (ulong)DAT_0011b008;
  DAT_0011b014 = param_1;
  DAT_0011b008 = *(ushort *)(&DAT_0019d01a + uVar2) + 0x1e +
                 (uint)*(ushort *)(&DAT_0019d01c + uVar2) + DAT_0011b008;
  if ((DAT_0011b00c < DAT_0011b008) || (*(int *)(&DAT_0019d000 + uVar2) != 0x4034b50)) {
    pcVar3 = "\n%s: %s: not a valid zip file\n";
  }
  else {
    pcVar3 = "\n%s: %s: first entry not deflated or stored -- use unzip\n";
    DAT_00119020 = (uint)(byte)(&DAT_0019d008)[uVar2];
    if (((&DAT_0019d008)[uVar2] & 0xf7) == 0) {
      bVar1 = (&DAT_0019d006)[uVar2];
      DAT_001e0ea0 = bVar1 & 1;
      if ((bVar1 & 1) == 0) {
        DAT_001e0e9c = 1;
        DAT_001e0e98 = bVar1 >> 3 & 1;
        return 0;
      }
      pcVar3 = "\n%s: %s: encrypted file -- use unzip\n";
    }
  }
  FUN_0010f630(stderr,pcVar3,DAT_0011b850,&DAT_0011b420);
  DAT_0011b84c = 1;
  return 1;
}




// Function: unzip @ 0xcd20

undefined4 unzip(undefined4 param_1,undefined4 param_2)

{
  undefined1 uVar1;
  int iVar2;
  ulong uVar3;
  ulong uVar4;
  uint *puVar5;
  undefined1 *puVar6;
  uint uVar7;
  uint uVar8;
  undefined4 uVar9;
  ulong uVar10;
  long in_FS_OFFSET;
  bool bVar11;
  uint local_48;
  uint local_44;
  undefined1 local_40 [4];
  uint local_3c;
  undefined1 local_38 [8];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_0011b010 = param_2;
  DAT_0011b014 = param_1;
  FUN_0010d1f0(0,0);
  if (DAT_001e0e9c == 0) {
    if (DAT_00119020 != 8) {
LAB_0010d17b:
                    /* WARNING: Subroutine does not return */
      FUN_0010d510("internal error, invalid method");
    }
    uVar8 = 0;
    uVar3 = 0;
LAB_0010ce42:
    iVar2 = FUN_00109e80();
    if (iVar2 == 3) {
                    /* WARNING: Subroutine does not return */
      FUN_0010d550();
    }
    if (iVar2 != 0) {
                    /* WARNING: Subroutine does not return */
      FUN_0010d510("invalid compressed data--format violated");
    }
    if (DAT_001e0e9c != 0) goto LAB_0010d045;
LAB_0010ce68:
    puVar5 = &local_48;
    do {
      while (uVar3 = (ulong)DAT_0011b008, DAT_0011b00c <= DAT_0011b008) {
        puVar6 = (undefined1 *)((long)puVar5 + 1);
        uVar1 = FUN_0010d830(0);
        *(undefined1 *)puVar5 = uVar1;
        puVar5 = (uint *)puVar6;
        if (puVar6 == local_40) goto LAB_0010cebb;
      }
      DAT_0011b008 = DAT_0011b008 + 1;
      puVar6 = (undefined1 *)((long)puVar5 + 1);
      *(undefined1 *)puVar5 = (&DAT_0019d000)[uVar3];
      puVar5 = (uint *)puVar6;
    } while (puVar6 != local_40);
LAB_0010cebb:
    uVar3 = (ulong)local_48;
    uVar8 = local_44;
  }
  else {
    uVar7 = DAT_0019d00e;
    uVar8 = DAT_0019d016;
    if (DAT_001e0e98 != 0) {
      uVar7 = 0;
      uVar8 = 0;
    }
    uVar3 = (ulong)uVar7;
    if (DAT_00119020 == 8) goto LAB_0010ce42;
    if (DAT_00119020 != 0) goto LAB_0010d17b;
    uVar4 = (ulong)DAT_0019d012;
    uVar10 = (ulong)DAT_0019d016;
    if (DAT_001e0ea0 != 0) {
      uVar4 = uVar4 - 0xc;
    }
    if (uVar10 != uVar4) {
      FUN_0010f630(stderr,"len %lu, siz %lu\n",uVar10);
                    /* WARNING: Subroutine does not return */
      FUN_0010d510("invalid compressed data--length mismatch");
    }
    for (; uVar10 != 0; uVar10 = uVar10 - 1) {
      while( true ) {
        uVar4 = (ulong)DAT_0011b008;
        if (DAT_0011b008 < DAT_0011b00c) {
          DAT_0011b008 = DAT_0011b008 + 1;
          uVar1 = (&DAT_0019d000)[uVar4];
        }
        else {
          uVar1 = FUN_0010d830(0);
        }
        uVar4 = (ulong)DAT_0011b004;
        DAT_0011b004 = DAT_0011b004 + 1;
        (&DAT_0013c000)[uVar4] = uVar1;
        if (DAT_0011b004 == 0x8000) break;
        uVar10 = uVar10 - 1;
        if (uVar10 == 0) goto LAB_0010d030;
      }
      FUN_0010d7e0();
    }
LAB_0010d030:
    FUN_0010d7e0();
    if (DAT_001e0e9c == 0) goto LAB_0010ce68;
LAB_0010d045:
    if (DAT_001e0e98 != 0) {
      puVar5 = &local_48;
      do {
        while (uVar3 = (ulong)DAT_0011b008, DAT_0011b00c <= DAT_0011b008) {
          puVar6 = (undefined1 *)((long)puVar5 + 1);
          uVar1 = FUN_0010d830(0);
          *(undefined1 *)puVar5 = uVar1;
          puVar5 = (uint *)puVar6;
          if (puVar6 == local_38) goto LAB_0010d0ab;
        }
        DAT_0011b008 = DAT_0011b008 + 1;
        puVar6 = (undefined1 *)((long)puVar5 + 1);
        *(undefined1 *)puVar5 = (&DAT_0019d000)[uVar3];
        puVar5 = (uint *)puVar6;
      } while (puVar6 != local_38);
LAB_0010d0ab:
      uVar3 = (ulong)local_44;
      uVar8 = local_3c;
    }
  }
  uVar4 = FUN_0010d1f0(&DAT_0015c000,0);
  bVar11 = uVar4 != uVar3;
  if (bVar11) {
    FUN_0010f630(stderr,"\n%s: %s: invalid compressed data--crc error\n",DAT_0011b850,&DAT_0011b420)
    ;
  }
  if (DAT_0011b820 == uVar8) {
    if (DAT_001e0e9c != 0) goto LAB_0010cf4a;
LAB_0010cfab:
    DAT_001e0e9c = 0;
    uVar9 = 0;
    if (!bVar11) goto LAB_0010cfe9;
  }
  else {
    FUN_0010f630(stderr,"\n%s: %s: invalid compressed data--length error\n",DAT_0011b850,
                 &DAT_0011b420);
    if (DAT_001e0e9c != 0) {
      bVar11 = true;
LAB_0010cf4a:
      if ((DAT_0011b008 + 4 < DAT_0011b00c) &&
         (((long)(int)(uint)*(ushort *)(&DAT_0019d002 + DAT_0011b008) << 0x10 |
          (long)(int)(uint)*(ushort *)(&DAT_0019d000 + DAT_0011b008)) == 0x4034b50)) {
        if (DAT_0011b860 == 0) {
          FUN_0010f630(stderr,"%s: %s has more than one entry -- unchanged\n",DAT_0011b850,
                       &DAT_0011b420);
          DAT_001e0e9c = 0;
          goto LAB_0010cfcb;
        }
        if (DAT_0011b85c == 0) {
          FUN_0010f630(stderr,"%s: %s has more than one entry--rest ignored\n",DAT_0011b850,
                       &DAT_0011b420);
        }
        if (DAT_0011b84c == 0) {
          DAT_0011b84c = 2;
        }
      }
      goto LAB_0010cfab;
    }
  }
LAB_0010cfcb:
  DAT_001e0e98 = 0;
  DAT_0011b84c = 1;
  if (DAT_0011b858 == 0) {
    DAT_001e0e90 = uVar3;
                    /* WARNING: Subroutine does not return */
    FUN_00108620();
  }
  uVar9 = 1;
LAB_0010cfe9:
  DAT_001e0e98 = 0;
  DAT_001e0e90 = uVar3;
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar9;
}




// Function: write_buffer @ 0xd1d0

void write_buffer(int param_1,void *param_2,uint param_3)

{
  if (0x7fffffff < param_3) {
    param_3 = 0x7fffffff;
  }
  write(param_1,param_2,(ulong)param_3);
  return;
}




// Function: updcrc @ 0xd1f0

ulong updcrc(byte *param_1,uint param_2)

{
  byte *pbVar1;
  byte bVar2;
  ulong uVar3;
  ulong uVar4;
  
  uVar3 = 0;
  uVar4 = 0xffffffff;
  if (param_1 != (byte *)0x0) {
    if (param_2 != 0) {
      pbVar1 = param_1 + param_2;
      do {
        bVar2 = *param_1;
        param_1 = param_1 + 1;
        DAT_001193f0 = DAT_001193f0 >> 8 ^
                       *(ulong *)(&DAT_00113800 +
                                 (ulong)(((uint)bVar2 ^ (uint)DAT_001193f0) & 0xff) * 8);
      } while (param_1 != pbVar1);
    }
    uVar3 = DAT_001193f0 ^ 0xffffffff;
    uVar4 = DAT_001193f0;
  }
  DAT_001193f0 = uVar4;
  return uVar3;
}




// Function: getcrc @ 0xd250

ulong getcrc(void)

{
  return DAT_001193f0 ^ 0xffffffff;
}




// Function: clear_bufs @ 0xd270

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void clear_bufs(void)

{
  DAT_0011b004 = 0;
  DAT_0011b008 = 0;
  DAT_0011b00c = 0;
  _DAT_0011b820 = 0;
  DAT_0011b828 = 0;
  return;
}




// Function: read_buffer @ 0xd2b0

ulong read_buffer(int param_1,void *param_2,uint param_3)

{
  uint uVar1;
  int iVar2;
  ulong uVar3;
  int *piVar4;
  ulong uVar5;
  
  if (0x7fffffff < param_3) {
    param_3 = 0x7fffffff;
  }
  uVar3 = read(param_1,param_2,(ulong)param_3);
  uVar5 = uVar3 & 0xffffffff;
  if ((int)uVar3 < 0) {
    piVar4 = __errno_location();
    if (*piVar4 == 0xb) {
      uVar1 = FUN_0010e270(param_1,3);
      if (-1 < (int)uVar1) {
        if ((uVar1 & 0x800) == 0) {
          *piVar4 = 0xb;
        }
        else {
          iVar2 = FUN_0010e270(param_1,4,uVar1 & 0xfffff7ff);
          if (iVar2 != -1) {
            uVar5 = read(param_1,param_2,(ulong)param_3);
            uVar5 = uVar5 & 0xffffffff;
          }
        }
      }
    }
  }
  return uVar5;
}




// Function: strlwr @ 0xd350

byte * strlwr(byte *param_1)

{
  ushort **ppuVar1;
  __int32_t **pp_Var2;
  byte bVar3;
  byte *pbVar4;
  
  bVar3 = *param_1;
  if (bVar3 != 0) {
    ppuVar1 = __ctype_b_loc();
    pbVar4 = param_1;
    do {
      if ((*(byte *)((long)*ppuVar1 + (ulong)bVar3 * 2 + 1) & 1) != 0) {
        pp_Var2 = __ctype_tolower_loc();
        bVar3 = *(byte *)(*pp_Var2 + bVar3);
      }
      *pbVar4 = bVar3;
      bVar3 = pbVar4[1];
      pbVar4 = pbVar4 + 1;
    } while (bVar3 != 0);
  }
  return param_1;
}




// Function: add_envopt @ 0xd3d0

char * add_envopt(uint *param_1,undefined8 *param_2,char *param_3)

{
  char cVar1;
  char *pcVar2;
  size_t sVar3;
  char *pcVar4;
  undefined8 *puVar5;
  char *pcVar6;
  int iVar7;
  undefined8 *puVar8;
  undefined8 *puVar9;
  
  pcVar2 = getenv(param_3);
  pcVar6 = (char *)0x0;
  if (pcVar2 != (char *)0x0) {
    iVar7 = 0;
    pcVar2 = (char *)FUN_0010f5d0(pcVar2);
    pcVar6 = pcVar2;
    if (*pcVar2 == '\0') {
LAB_0010d4f9:
      pcVar6 = (char *)0x0;
      free(pcVar2);
    }
    else {
      do {
        sVar3 = strspn(pcVar6," \t");
        pcVar6 = pcVar6 + sVar3;
        if (*pcVar6 == '\0') {
          if (iVar7 == 0) goto LAB_0010d4f9;
          break;
        }
        iVar7 = iVar7 + 1;
        sVar3 = strcspn(pcVar6," \t");
        pcVar4 = pcVar6 + sVar3;
        if (*pcVar4 == '\0') break;
        *pcVar4 = '\0';
        pcVar6 = pcVar4 + 1;
      } while (pcVar4[1] != '\0');
      *param_1 = iVar7 + 1U;
      puVar5 = (undefined8 *)FUN_0010f4d0((long)(iVar7 + 2),8);
      puVar8 = (undefined8 *)*param_2;
      *param_2 = puVar5;
      *puVar5 = *puVar8;
      pcVar6 = pcVar2;
      puVar8 = puVar5 + 1;
      do {
        puVar9 = puVar8 + 1;
        sVar3 = strspn(pcVar6," \t");
        *puVar8 = pcVar6 + sVar3;
        pcVar4 = pcVar6 + sVar3;
        do {
          pcVar6 = pcVar4 + 1;
          cVar1 = *pcVar4;
          pcVar4 = pcVar6;
        } while (cVar1 != '\0');
        puVar8 = puVar9;
      } while (puVar5 + (iVar7 + 1U) != puVar9);
      puVar5[iVar7 + 1U] = 0;
      pcVar6 = pcVar2;
    }
  }
  return pcVar6;
}




// Function: gzip_error @ 0xd510

void gzip_error(undefined8 param_1)

{
  FUN_0010f630(stderr,"\n%s: %s: %s\n",DAT_0011b850,&DAT_0011b420,param_1);
                    /* WARNING: Subroutine does not return */
  FUN_00108620();
}




// Function: xalloc_die @ 0xd550

void xalloc_die(void)

{
  FUN_0010f630(stderr,"\n%s: memory_exhausted\n",DAT_0011b850);
                    /* WARNING: Subroutine does not return */
  FUN_00108620();
}




// Function: warning @ 0xd580

void warning(undefined8 param_1)

{
  if (DAT_0011b85c == 0) {
    FUN_0010f630(stderr,"%s: %s: warning: %s\n",DAT_0011b850,&DAT_0011b420,param_1);
    if (DAT_0011b84c == 0) {
      DAT_0011b84c = 2;
    }
    return;
  }
  if (DAT_0011b84c == 0) {
    DAT_0011b84c = 2;
    return;
  }
  return;
}




// Function: read_error @ 0xd600

void read_error(void)

{
  int iVar1;
  int *piVar2;
  
  piVar2 = __errno_location();
  iVar1 = *piVar2;
  FUN_0010f630(stderr,&DAT_001137cb,DAT_0011b850);
  if (iVar1 == 0) {
    FUN_0010f630(stderr,"%s: unexpected end of file\n",&DAT_0011b420);
  }
  else {
    *piVar2 = iVar1;
    perror(&DAT_0011b420);
  }
                    /* WARNING: Subroutine does not return */
  FUN_00108620();
}




// Function: write_error @ 0xd670

void write_error(void)

{
  int iVar1;
  int *piVar2;
  
  piVar2 = __errno_location();
  iVar1 = *piVar2;
  FUN_0010f630(stderr,&DAT_001137cb,DAT_0011b850);
  *piVar2 = iVar1;
  perror(&DAT_0011b020);
                    /* WARNING: Subroutine does not return */
  FUN_00108620();
}




// Function: write_buf @ 0xd6c0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulong write_buf(ulong param_1,long param_2,ulong param_3)

{
  int iVar1;
  uint uVar2;
  ulong uVar3;
  int *piVar4;
  undefined4 uVar5;
  ulong uVar7;
  long lVar6;
  
  param_3 = param_3 & 0xffffffff;
  _DAT_0011b820 = _DAT_0011b820 + param_3;
  if (DAT_0011b858 != 0) {
    return param_3;
  }
  while( true ) {
    uVar7 = param_1 & 0xffffffff;
    lVar6 = param_2;
    uVar3 = FUN_0010d1d0(uVar7,param_2,param_3);
    uVar5 = (undefined4)lVar6;
    iVar1 = (int)uVar3;
    if ((int)param_3 == iVar1) {
      return uVar3;
    }
    if (iVar1 == -1) break;
    param_3 = (ulong)(uint)((int)param_3 - iVar1);
    param_2 = param_2 + (uVar3 & 0xffffffff);
  }
  FUN_0010d670();
  piVar4 = __errno_location();
  *piVar4 = 0;
  uVar3 = (ulong)DAT_0011b008;
  if (DAT_0011b008 < DAT_0011b00c) {
    do {
      write_buf(uVar5,&DAT_0019d000 + uVar3,DAT_0011b00c - (int)uVar3);
      uVar2 = FUN_0010d2b0(uVar7 & 0xffffffff,&DAT_0019d000,0x40000);
      if (uVar2 == 0xffffffff) {
                    /* WARNING: Subroutine does not return */
        FUN_0010d600();
      }
      uVar3 = 0;
      DAT_0011b828 = DAT_0011b828 + (int)uVar2;
      DAT_0011b008 = 0;
      DAT_0011b00c = uVar2;
    } while (uVar2 != 0);
  }
  return 0;
}




// Function: copy @ 0xd720

undefined8 copy(undefined4 param_1,undefined4 param_2)

{
  uint uVar1;
  int *piVar2;
  ulong uVar3;
  
  piVar2 = __errno_location();
  *piVar2 = 0;
  uVar3 = (ulong)DAT_0011b008;
  if (DAT_0011b008 < DAT_0011b00c) {
    do {
      FUN_0010d6c0(param_2,&DAT_0019d000 + uVar3,DAT_0011b00c - (int)uVar3);
      uVar1 = FUN_0010d2b0(param_1,&DAT_0019d000,0x40000);
      if (uVar1 == 0xffffffff) {
                    /* WARNING: Subroutine does not return */
        FUN_0010d600();
      }
      uVar3 = 0;
      DAT_0011b828 = DAT_0011b828 + (int)uVar1;
      DAT_0011b008 = 0;
      DAT_0011b00c = uVar1;
    } while (uVar1 != 0);
  }
  return 0;
}




// Function: flush_outbuf @ 0xd7a0

void flush_outbuf(void)

{
  if (DAT_0011b004 == 0) {
    return;
  }
  FUN_0010d6c0(DAT_0011b010,&DAT_0015c000);
  DAT_0011b004 = 0;
  return;
}




// Function: flush_window @ 0xd7e0

void flush_window(void)

{
  ulong uVar1;
  undefined1 *puVar2;
  
  uVar1 = (ulong)DAT_0011b004;
  if (DAT_0011b004 == 0) {
    return;
  }
  puVar2 = &DAT_0013c000;
  FUN_0010d1f0(&DAT_0013c000,uVar1);
  FUN_0010d6c0(DAT_0011b010,puVar2,uVar1 & 0xffffffff);
  DAT_0011b004 = 0;
  return;
}




// Function: fill_inbuf @ 0xd830

ulong fill_inbuf(int param_1)

{
  int iVar1;
  ulong uVar2;
  int *piVar3;
  
  uVar2 = 0;
  DAT_0011b00c = 0;
  do {
    iVar1 = FUN_0010d2b0(DAT_0011b014,&DAT_0019d000 + uVar2,0x40000 - (int)uVar2);
    if (iVar1 == 0) {
      uVar2 = (ulong)DAT_0011b00c;
      if (DAT_0011b00c == 0) {
        if (param_1 != 0) {
          return 0xffffffff;
        }
        FUN_0010d7e0();
        piVar3 = __errno_location();
        *piVar3 = 0;
LAB_0010d8d5:
                    /* WARNING: Subroutine does not return */
        FUN_0010d600();
      }
      break;
    }
    if (iVar1 == -1) goto LAB_0010d8d5;
    DAT_0011b00c = iVar1 + DAT_0011b00c;
    uVar2 = (ulong)DAT_0011b00c;
  } while (DAT_0011b00c < 0x40000);
  DAT_0011b008 = 1;
  DAT_0011b828 = DAT_0011b828 + uVar2;
  return (ulong)DAT_0019d000;
}




// Function: display_ratio @ 0xd8e0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void display_ratio(long param_1,long param_2,undefined8 param_3)

{
  double dVar1;
  
  dVar1 = 0.0;
  if (param_2 != 0) {
    dVar1 = ((double)param_1 * _DAT_00114000) / (double)param_2;
  }
  FUN_0010f630(dVar1,param_3,"%5.1f%%");
  return;
}




// Function: fprint_off @ 0xd930

void fprint_off(FILE *param_1,ulong param_2,int param_3)

{
  char *pcVar1;
  char cVar2;
  char *pcVar3;
  ulong uVar4;
  char *pcVar5;
  long in_FS_OFFSET;
  char local_3a [10];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  pcVar1 = local_3a + 2;
  pcVar5 = pcVar1;
  pcVar3 = pcVar1;
  if ((long)param_2 < 0) {
    do {
      pcVar5 = pcVar3;
      uVar4 = (long)param_2 / 10;
      pcVar5[-1] = ((char)uVar4 * '\n' + '0') - (char)param_2;
      param_2 = uVar4;
      pcVar3 = pcVar5 + -1;
    } while (uVar4 != 0);
    pcVar5[-2] = '-';
    pcVar5 = pcVar5 + -2;
  }
  else {
    do {
      pcVar5 = pcVar5 + -1;
      uVar4 = param_2 / 10;
      *pcVar5 = (char)param_2 + (char)uVar4 * -10 + '0';
      param_2 = uVar4;
    } while (uVar4 != 0);
  }
  param_3 = param_3 - ((int)pcVar1 - (int)pcVar5);
  if (0 < param_3) {
    do {
      param_3 = param_3 + -1;
      putc(0x20,param_1);
    } while (param_3 != 0);
  }
  if (pcVar5 < pcVar1) {
    do {
      cVar2 = *pcVar5;
      pcVar5 = pcVar5 + 1;
      putc((int)cVar2,param_1);
    } while (pcVar5 != pcVar1);
  }
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: zip @ 0xda60

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 zip(undefined4 param_1,undefined4 param_2)

{
  undefined1 uVar1;
  uint uVar2;
  undefined1 uVar6;
  undefined1 extraout_AH_00;
  undefined1 extraout_var;
  undefined1 extraout_var_01;
  ulong uVar3;
  char *pcVar4;
  undefined1 extraout_AH;
  undefined1 extraout_var_00;
  long lVar5;
  char cVar7;
  ulong uVar8;
  uint uVar9;
  ulong uVar10;
  long in_FS_OFFSET;
  undefined2 local_22;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_22 = 0;
  _DAT_0015c000 = 0x8b1f;
  DAT_0015c003 = (DAT_0011b848 != 0) << 3;
  DAT_00119020 = 8;
  DAT_0015c002 = 8;
  DAT_0011b004 = 4;
  DAT_0011b010 = param_2;
  DAT_0011b014 = param_1;
  if (DAT_0011b838 < 0) {
    uVar6 = 0;
    uVar1 = 0;
    DAT_0015c005 = 0;
    DAT_0015c004 = 0;
LAB_0010db09:
    uVar3 = 7;
    uVar2 = 6;
  }
  else {
    if (DAT_0011b830 - 1U < 0xffffffff) {
      DAT_0015c004 = (undefined1)DAT_0011b830;
      DAT_0015c005 = (undefined1)((ulong)DAT_0011b830 >> 8);
      uVar1 = (undefined1)((ulong)DAT_0011b830 >> 0x10);
      uVar6 = (undefined1)((ulong)DAT_0011b830 >> 0x18);
      goto LAB_0010db09;
    }
    FUN_0010d580("file timestamp out of range for gzip format");
    uVar3 = (ulong)DAT_0011b004;
    if (DAT_0011b004 < 0x3fffe) {
      uVar2 = DAT_0011b004 + 2;
      uVar9 = DAT_0011b004 + 3;
      (&DAT_0015c000)[DAT_0011b004] = 0;
      (&DAT_0015c000)[DAT_0011b004 + 1] = 0;
    }
    else {
      DAT_0011b004 = DAT_0011b004 + 1;
      (&DAT_0015c000)[uVar3] = 0;
      uVar9 = DAT_0011b004;
      if (DAT_0011b004 == 0x40000) {
        FUN_0010d7a0();
        uVar9 = DAT_0011b004;
      }
      uVar2 = uVar9 + 1;
      DAT_0011b004 = uVar2;
      (&DAT_0015c000)[uVar9] = 0;
      if (uVar2 == 0x40000) {
        FUN_0010d7a0();
        uVar9 = DAT_0011b004 + 1;
        uVar2 = DAT_0011b004;
      }
      else {
        uVar9 = uVar9 + 2;
      }
    }
    uVar3 = (ulong)uVar9;
    if (0x3fffd < uVar2) {
      DAT_0011b004 = uVar9;
      (&DAT_0015c000)[uVar2] = 0;
      if (uVar9 == 0x40000) {
        FUN_0010d7a0();
        uVar3 = (ulong)DAT_0011b004;
      }
      DAT_0011b004 = (int)uVar3 + 1;
      (&DAT_0015c000)[uVar3] = 0;
      if (DAT_0011b004 == 0x40000) {
        FUN_0010d7a0();
      }
      goto LAB_0010db34;
    }
    uVar6 = 0;
    uVar1 = 0;
  }
  DAT_0011b004 = uVar2 + 2;
  (&DAT_0015c000)[uVar2] = uVar1;
  (&DAT_0015c000)[uVar3] = uVar6;
LAB_0010db34:
  FUN_0010d1f0(0,0);
  FUN_00104260(param_2);
  FUN_0010ad80(&local_22,&DAT_00119020);
  cVar7 = '\x04';
  if (DAT_0011901c != 1) {
    cVar7 = (DAT_0011901c == 9) * '\x02';
  }
  uVar3 = (ulong)DAT_0011b004;
  DAT_0011b004 = DAT_0011b004 + 1;
  (&DAT_0015c000)[uVar3] = cVar7;
  if (DAT_0011b004 == 0x40000) {
    FUN_0010d7a0();
  }
  uVar3 = (ulong)DAT_0011b004;
  DAT_0011b004 = DAT_0011b004 + 1;
  (&DAT_0015c000)[uVar3] = 3;
  if (DAT_0011b004 == 0x40000) {
    FUN_0010d7a0();
  }
  uVar3 = (ulong)DAT_0011b004;
  if (DAT_0011b848 != 0) {
    pcVar4 = (char *)thunk_FUN_0010e060(&DAT_0011b420);
    do {
      while( true ) {
        uVar2 = (int)uVar3 + 1;
        DAT_0011b004 = uVar2;
        (&DAT_0015c000)[uVar3] = *pcVar4;
        uVar3 = (ulong)uVar2;
        if (uVar2 != 0x40000) break;
        FUN_0010d7a0();
        uVar3 = (ulong)DAT_0011b004;
        cVar7 = *pcVar4;
        pcVar4 = pcVar4 + 1;
        if (cVar7 == '\0') goto LAB_0010dc10;
      }
      cVar7 = *pcVar4;
      pcVar4 = pcVar4 + 1;
    } while (cVar7 != '\0');
  }
LAB_0010dc10:
  DAT_001e0ea8 = uVar3;
  FUN_00104ed0(DAT_0011901c);
  if ((DAT_0011b840 != -1) && (DAT_0011b840 != DAT_0011b828)) {
    FUN_0010f630(stderr,"%s: %s: file size changed while zipping\n",DAT_0011b850,&DAT_0011b420);
  }
  uVar1 = FUN_0010d250();
  uVar3 = (ulong)DAT_0011b004;
  uVar2 = DAT_0011b004 + 1;
  uVar10 = (ulong)uVar2;
  if (DAT_0011b004 < 0x3fffe) {
    uVar8 = (ulong)DAT_0011b004;
    uVar9 = DAT_0011b004 + 2;
    uVar3 = (ulong)uVar9;
    DAT_0011b004 = uVar2;
    (&DAT_0015c000)[uVar8] = uVar1;
    FUN_0010d250();
    DAT_0011b004 = uVar9;
    (&DAT_0015c000)[uVar10] = extraout_AH;
  }
  else {
    DAT_0011b004 = uVar2;
    (&DAT_0015c000)[uVar3] = uVar1;
    if (uVar2 == 0x40000) {
      FUN_0010d7a0();
      uVar10 = (ulong)DAT_0011b004;
    }
    FUN_0010d250();
    uVar2 = (int)uVar10 + 1;
    uVar3 = (ulong)uVar2;
    DAT_0011b004 = uVar2;
    (&DAT_0015c000)[uVar10] = extraout_AH_00;
    if (uVar2 == 0x40000) {
      FUN_0010d7a0();
      uVar3 = (ulong)DAT_0011b004;
    }
  }
  FUN_0010d250();
  uVar2 = (uint)uVar3;
  uVar9 = uVar2 + 1;
  uVar10 = (ulong)uVar9;
  DAT_0011b004 = uVar9;
  if (uVar2 < 0x3fffe) {
    uVar8 = (ulong)(uVar2 + 2);
    (&DAT_0015c000)[uVar3] = extraout_var;
    FUN_0010d250();
    (&DAT_0015c000)[uVar10] = extraout_var_00;
  }
  else {
    (&DAT_0015c000)[uVar3] = extraout_var;
    if (uVar9 == 0x40000) {
      FUN_0010d7a0();
      uVar10 = (ulong)DAT_0011b004;
    }
    FUN_0010d250();
    uVar2 = (int)uVar10 + 1;
    uVar8 = (ulong)uVar2;
    DAT_0011b004 = uVar2;
    (&DAT_0015c000)[uVar10] = extraout_var_01;
    if (uVar2 == 0x40000) {
      FUN_0010d7a0();
      uVar8 = (ulong)DAT_0011b004;
    }
  }
  lVar5 = DAT_0011b828;
  uVar2 = (uint)uVar8;
  uVar9 = uVar2 + 1;
  uVar3 = (ulong)uVar9;
  if (uVar2 < 0x3fffe) {
    uVar10 = (ulong)(uVar2 + 2);
    (&DAT_0015c000)[uVar8] = (char)DAT_0011b828;
    (&DAT_0015c000)[uVar3] = (char)((ulong)lVar5 >> 8);
  }
  else {
    DAT_0011b004 = uVar9;
    (&DAT_0015c000)[uVar8] = (char)DAT_0011b828;
    if (uVar9 == 0x40000) {
      FUN_0010d7a0();
      uVar3 = (ulong)DAT_0011b004;
      lVar5 = DAT_0011b828;
    }
    DAT_0011b004 = (int)uVar3 + 1;
    uVar10 = (ulong)DAT_0011b004;
    (&DAT_0015c000)[uVar3] = (char)((ulong)lVar5 >> 8);
    if (DAT_0011b004 == 0x40000) {
      FUN_0010d7a0();
      uVar10 = (ulong)DAT_0011b004;
      lVar5 = DAT_0011b828;
    }
  }
  uVar2 = (uint)uVar10;
  uVar9 = uVar2 + 1;
  uVar3 = (ulong)uVar9;
  uVar1 = (undefined1)((ulong)lVar5 >> 0x10);
  if (uVar2 < 0x3fffe) {
    DAT_0011b004 = uVar2 + 2;
    (&DAT_0015c000)[uVar10] = uVar1;
    (&DAT_0015c000)[uVar3] = (char)((ulong)lVar5 >> 0x18);
  }
  else {
    DAT_0011b004 = uVar9;
    (&DAT_0015c000)[uVar10] = uVar1;
    if (uVar9 == 0x40000) {
      FUN_0010d7a0();
      uVar3 = (ulong)DAT_0011b004;
      lVar5 = DAT_0011b828;
    }
    DAT_0011b004 = (int)uVar3 + 1;
    (&DAT_0015c000)[uVar3] = (char)((ulong)lVar5 >> 0x18);
    if (DAT_0011b004 == 0x40000) {
      FUN_0010d7a0();
    }
  }
  DAT_001e0ea8 = DAT_001e0ea8 + 8;
  FUN_0010d7a0();
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return 0;
}




// Function: file_read @ 0xe010

ulong file_read(undefined8 param_1,undefined4 param_2)

{
  uint uVar1;
  
  uVar1 = FUN_0010d2b0(DAT_0011b014,param_1,param_2);
  if (uVar1 != 0) {
    if (uVar1 == 0xffffffff) {
                    /* WARNING: Subroutine does not return */
      FUN_0010d600();
    }
    FUN_0010d1f0(param_1,uVar1);
    DAT_0011b828 = DAT_0011b828 + (ulong)uVar1;
  }
  return (ulong)uVar1;
}




// Function: last_component @ 0xe060

char * last_component(char *param_1)

{
  bool bVar1;
  char *pcVar2;
  char cVar3;
  
  cVar3 = *param_1;
  while (cVar3 == '/') {
    pcVar2 = param_1 + 1;
    param_1 = param_1 + 1;
    cVar3 = *pcVar2;
  }
  if (cVar3 != '\0') {
    bVar1 = false;
    pcVar2 = param_1;
    do {
      while (cVar3 != '/') {
        if (bVar1) {
          bVar1 = false;
          param_1 = pcVar2;
        }
        cVar3 = pcVar2[1];
        pcVar2 = pcVar2 + 1;
        if (cVar3 == '\0') {
          return param_1;
        }
      }
      cVar3 = pcVar2[1];
      pcVar2 = pcVar2 + 1;
      bVar1 = true;
    } while (cVar3 != '\0');
  }
  return param_1;
}




// Function: rpl_fcntl_DUPFD @ 0xe180

void rpl_fcntl_DUPFD(int param_1,ulong param_2)

{
  fcntl(param_1,0,param_2 & 0xffffffff);
  return;
}




// Function: rpl_fcntl_DUPFD_CLOEXEC @ 0xe190

int rpl_fcntl_DUPFD_CLOEXEC(int param_1,uint param_2)

{
  int iVar1;
  uint uVar2;
  int iVar3;
  int *piVar4;
  
  if (DAT_001e0eb0 < 0) {
    iVar1 = FUN_0010e180();
    if (iVar1 < 0) {
      return iVar1;
    }
    if (DAT_001e0eb0 != -1) {
      return iVar1;
    }
  }
  else {
    iVar1 = fcntl(param_1,0x406,(ulong)param_2);
    if ((-1 < iVar1) || (piVar4 = __errno_location(), *piVar4 != 0x16)) {
      DAT_001e0eb0 = 1;
      return iVar1;
    }
    iVar1 = FUN_0010e180(param_1,param_2);
    if (iVar1 < 0) {
      return iVar1;
    }
    DAT_001e0eb0 = -1;
  }
  uVar2 = fcntl(iVar1,1);
  if ((-1 < (int)uVar2) && (iVar3 = fcntl(iVar1,2,(ulong)(uVar2 | 1)), iVar3 != -1)) {
    return iVar1;
  }
  piVar4 = __errno_location();
  iVar3 = *piVar4;
  close(iVar1);
  *piVar4 = iVar3;
  return -1;
}




// Function: rpl_fcntl @ 0xe270

/* WARNING: Removing unreachable block (ram,0x0010e3a0) */
/* WARNING: Removing unreachable block (ram,0x0010e3b0) */
/* WARNING: Heritage AFTER dead removal. Example location: s0xffffffffffffffd8 : 0x0010e38d */
/* WARNING: Restarted to delay deadcode elimination for space: stack */

void rpl_fcntl(int param_1,int param_2,uint param_3)

{
  long lVar1;
  ulong uVar2;
  long in_FS_OFFSET;
  undefined8 local_28;
  
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  local_28 = CONCAT44(local_28._4_4_,param_3);
  if (param_2 == 0) {
    FUN_0010e180(param_1,param_3);
    goto LAB_0010e355;
  }
  if (param_2 == 0x406) {
    FUN_0010e190(param_1,param_3);
    goto LAB_0010e355;
  }
  if (param_2 < 0xc) {
    if (0 < param_2) {
      uVar2 = 1L << ((byte)param_2 & 0x3f);
      if ((uVar2 & 0x514) != 0) {
LAB_0010e38d:
        fcntl(param_1,param_2,(ulong)param_3);
        goto LAB_0010e355;
      }
      uVar2 = uVar2 & 0xa0a;
joined_r0x0010e317:
      if (uVar2 != 0) {
        fcntl(param_1,param_2);
        goto LAB_0010e355;
      }
    }
  }
  else if (param_2 - 0x400U < 0xb) {
    uVar2 = 1L << ((byte)(param_2 - 0x400U) & 0x3f);
    if ((uVar2 & 0x2c5) != 0) goto LAB_0010e38d;
    uVar2 = uVar2 & 0x502;
    goto joined_r0x0010e317;
  }
  fcntl(param_1,param_2,local_28);
LAB_0010e355:
  if (lVar1 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: open_safer @ 0xe3c0

void open_safer(char *param_1,uint param_2,ulong param_3)

{
  long lVar1;
  int iVar2;
  ulong uVar3;
  long in_FS_OFFSET;
  
  uVar3 = 0;
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  if ((param_2 & 0x40) != 0) {
    uVar3 = param_3 & 0xffffffff;
  }
  iVar2 = open(param_1,param_2,uVar3);
  FUN_0010e980(iVar2);
  if (lVar1 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: clear_ungetc_buffer_preserving_position @ 0xe440

void clear_ungetc_buffer_preserving_position(uint *param_1)

{
  if ((*param_1 & 0x100) == 0) {
    return;
  }
  FUN_0010e4a0(param_1,0,1);
  return;
}




// Function: rpl_fseeko @ 0xe4a0

int rpl_fseeko(FILE *param_1,__off_t param_2,int param_3)

{
  int iVar1;
  __off_t _Var2;
  
  if (((param_1->_IO_read_end == param_1->_IO_read_ptr) &&
      (param_1->_IO_write_ptr == param_1->_IO_write_base)) &&
     (param_1->_IO_save_base == (char *)0x0)) {
    iVar1 = fileno(param_1);
    _Var2 = lseek(iVar1,param_2,param_3);
    if (_Var2 == -1) {
      iVar1 = -1;
    }
    else {
      param_1->_flags = param_1->_flags & 0xffffffef;
      param_1->_offset = _Var2;
      iVar1 = 0;
    }
    return iVar1;
  }
  iVar1 = fseeko(param_1,param_2,param_3);
  return iVar1;
}




// Function: openat_safer @ 0xe520

void openat_safer(int param_1,char *param_2,uint param_3,ulong param_4)

{
  long lVar1;
  int iVar2;
  ulong uVar3;
  long in_FS_OFFSET;
  
  uVar3 = 0;
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  if ((param_3 & 0x40) != 0) {
    uVar3 = param_4 & 0xffffffff;
  }
  iVar2 = openat(param_1,param_2,param_3,uVar3);
  FUN_0010e980(iVar2);
  if (lVar1 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: direntry_cmp_name @ 0xe5a0

void direntry_cmp_name(undefined8 *param_1,undefined8 *param_2)

{
  strcmp((char *)*param_1,(char *)*param_2);
  return;
}




// Function: streamsavedir @ 0xe5b0

undefined1 * streamsavedir(DIR *param_1,uint param_2)

{
  long lVar1;
  char cVar2;
  __compar_fn_t __compar;
  int *piVar3;
  dirent *pdVar4;
  size_t sVar5;
  undefined8 uVar6;
  undefined1 *puVar7;
  char *pcVar8;
  void *__ptr;
  undefined1 *__ptr_00;
  long lVar9;
  char *pcVar10;
  size_t __nmemb;
  long in_FS_OFFSET;
  long local_50;
  size_t local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_50 = 0;
  local_48 = 0;
  __compar = *(__compar_fn_t *)(&DAT_00118b50 + (ulong)param_2 * 8);
  if (param_1 == (DIR *)0x0) {
    puVar7 = (undefined1 *)0x0;
  }
  else {
    lVar9 = 0;
    __nmemb = 0;
    __ptr = (void *)0x0;
    piVar3 = __errno_location();
    __ptr_00 = (undefined1 *)0x0;
    while( true ) {
      *piVar3 = 0;
      pdVar4 = readdir(param_1);
      if (pdVar4 == (dirent *)0x0) break;
      cVar2 = pdVar4->d_name[0];
      pcVar10 = pdVar4->d_name;
      if ((cVar2 == '.') && (cVar2 = pdVar4->d_name[1], cVar2 == '.')) {
        cVar2 = pdVar4->d_name[2];
      }
      if (cVar2 != '\0') {
        sVar5 = strlen(pcVar10);
        sVar5 = sVar5 + 1;
        if (__compar == (__compar_fn_t)0x0) {
          if (local_50 - lVar9 <= (long)sVar5) {
            __ptr_00 = (undefined1 *)
                       FUN_0010f3f0(__ptr_00,&local_50,sVar5 - (local_50 - lVar9),0x7ffffffffffffffe
                                    ,1);
          }
          memcpy(__ptr_00 + lVar9,pcVar10,sVar5);
        }
        else {
          if (local_48 == __nmemb) {
            __ptr = (void *)FUN_0010f3f0(__ptr,&local_48,1,0xffffffffffffffff,8);
          }
          lVar1 = __nmemb * 8;
          __nmemb = __nmemb + 1;
          uVar6 = FUN_0010f5d0(pcVar10);
          *(undefined8 *)((long)__ptr + lVar1) = uVar6;
        }
        lVar9 = lVar9 + sVar5;
      }
    }
    if (*piVar3 == 0) {
      if (__compar == (__compar_fn_t)0x0) {
        if (lVar9 == local_50) {
          __ptr_00 = (undefined1 *)FUN_0010f2c0(__ptr_00,lVar9 + 1);
        }
        puVar7 = __ptr_00 + lVar9;
      }
      else {
        if (__nmemb == 0) {
          puVar7 = (undefined1 *)FUN_0010f250(lVar9 + 1);
          __ptr_00 = puVar7;
        }
        else {
          pcVar10 = (char *)0x0;
          qsort(__ptr,__nmemb,8,__compar);
          sVar5 = 0;
          __ptr_00 = (undefined1 *)FUN_0010f250(lVar9 + 1);
          do {
            pcVar8 = stpcpy(__ptr_00 + (long)pcVar10,*(char **)((long)__ptr + sVar5 * 8));
            lVar9 = sVar5 * 8;
            sVar5 = sVar5 + 1;
            pcVar10 = pcVar8 + (long)(pcVar10 + (1 - (long)(__ptr_00 + (long)pcVar10)));
            free(*(void **)((long)__ptr + lVar9));
          } while (__nmemb != sVar5);
          puVar7 = __ptr_00 + (long)pcVar10;
        }
        free(__ptr);
      }
      *puVar7 = 0;
      puVar7 = __ptr_00;
    }
    else {
      free(__ptr);
      puVar7 = (undefined1 *)0x0;
      free(__ptr_00);
    }
  }
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return puVar7;
}




// Function: savedir @ 0xe870

void * savedir(undefined8 param_1,undefined4 param_2)

{
  int iVar1;
  DIR *__dirp;
  void *__ptr;
  
  __dirp = (DIR *)FUN_00110990();
  if (__dirp == (DIR *)0x0) {
    return (void *)0x0;
  }
  __ptr = (void *)FUN_0010e5b0(__dirp,param_2);
  iVar1 = closedir(__dirp);
  if (iVar1 == 0) {
    return __ptr;
  }
  free(__ptr);
  return (void *)0x0;
}




// Function: fd_safer @ 0xe980

uint fd_safer(uint param_1)

{
  int iVar1;
  uint uVar2;
  int *piVar3;
  
  if (2 < param_1) {
    return param_1;
  }
  uVar2 = FUN_00110b10();
  piVar3 = __errno_location();
  iVar1 = *piVar3;
  close(param_1);
  *piVar3 = iVar1;
  return uVar2;
}




// Function: validate_timespec @ 0xe9e0

int validate_timespec(undefined8 *param_1)

{
  ulong uVar1;
  ulong uVar2;
  bool bVar3;
  int *piVar4;
  char cVar5;
  uint uVar6;
  
  uVar1 = param_1[1];
  if ((uVar1 - 0x3ffffffe < 2) || (uVar1 < 1000000000)) {
    uVar2 = param_1[3];
    if ((uVar2 - 0x3ffffffe < 2) || (uVar2 < 1000000000)) {
      bVar3 = uVar1 - 0x3ffffffe < 2;
      if (bVar3) {
        *param_1 = 0;
      }
      cVar5 = bVar3 && uVar1 == 0x3ffffffe;
      uVar6 = (uint)bVar3;
      if (uVar2 - 0x3ffffffe < 2) {
        param_1[2] = 0;
        uVar6 = 1;
        cVar5 = cVar5 + (uVar2 == 0x3ffffffe);
      }
      return (cVar5 == '\x01') + uVar6;
    }
  }
  piVar4 = __errno_location();
  *piVar4 = 0x16;
  return -1;
}




// Function: update_timespec @ 0xea90

undefined8 update_timespec(undefined8 param_1,long *param_2)

{
  undefined1 (*pauVar1) [16];
  long lVar2;
  undefined1 auVar3 [16];
  
  pauVar1 = (undefined1 (*) [16])*param_2;
  lVar2 = *(long *)(pauVar1[1] + 8);
  if (*(long *)(*pauVar1 + 8) == 0x3ffffffe) {
    if (lVar2 == 0x3ffffffe) {
      return 1;
    }
    auVar3 = FUN_0010e920();
    *pauVar1 = auVar3;
  }
  else {
    if (*(long *)(*pauVar1 + 8) == 0x3fffffff) {
      if (lVar2 == 0x3fffffff) {
        *param_2 = 0;
        return 0;
      }
      FUN_00110a40(pauVar1);
      lVar2 = *(long *)(pauVar1[1] + 8);
    }
    if (lVar2 == 0x3ffffffe) {
      auVar3 = FUN_0010e940(param_1);
      pauVar1[1] = auVar3;
      return 0;
    }
  }
  if (lVar2 != 0x3fffffff) {
    return 0;
  }
  FUN_00110a40(pauVar1 + 1);
  return 0;
}




// Function: fdutimens @ 0xeb50

int fdutimens(int param_1,char *param_2,__time_t *param_3)

{
  bool bVar1;
  char cVar2;
  int iVar3;
  int iVar4;
  int *piVar5;
  long lVar6;
  long lVar7;
  timeval *__tvp;
  long in_FS_OFFSET;
  bool bVar8;
  timespec tVar9;
  timespec *local_140;
  timespec local_138;
  __time_t local_128;
  __time_t _Stack_120;
  timeval local_118;
  __time_t local_108;
  long lStack_100;
  timeval local_f8;
  __time_t local_e8;
  __suseconds_t _Stack_e0;
  stat local_d8;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_3 == (__time_t *)0x0) {
    local_140 = (timespec *)0x0;
    iVar3 = 0;
LAB_0010ebbc:
    if ((-1 < param_1) || (param_2 != (char *)0x0)) {
      if (-1 < DAT_001e0eb8) {
        if (iVar3 == 2) {
          if (param_1 < 0) {
            iVar3 = stat(param_2,&local_d8);
          }
          else {
            iVar3 = fstat(param_1,&local_d8);
          }
          if (iVar3 != 0) goto LAB_0010ed0b;
          if (local_140->tv_nsec == 0x3ffffffe) {
            iVar3 = 3;
            tVar9 = (timespec)FUN_0010e920(&local_d8);
            *local_140 = tVar9;
          }
          else {
            iVar3 = 3;
            if (local_140[1].tv_nsec == 0x3ffffffe) {
              tVar9 = (timespec)FUN_0010e940(&local_d8);
              local_140[1] = tVar9;
            }
          }
        }
        if (param_1 < 0) {
          iVar4 = utimensat(-100,param_2,local_140,0);
        }
        else {
          iVar4 = futimens(param_1,local_140);
        }
        if (iVar4 < 1) {
          if ((iVar4 == 0) || (piVar5 = __errno_location(), *piVar5 != 0x26)) {
            DAT_001e0eb8 = 1;
            goto LAB_0010ed11;
          }
        }
        else {
          piVar5 = __errno_location();
          *piVar5 = 0x26;
        }
      }
      DAT_001e0eb8 = -1;
      DAT_001e0eb4 = 0xffffffff;
      if (iVar3 != 0) {
        if (iVar3 != 3) {
          if (param_1 < 0) {
            iVar3 = stat(param_2,&local_d8);
          }
          else {
            iVar3 = fstat(param_1,&local_d8);
          }
          if (iVar3 != 0) goto LAB_0010ed0b;
        }
        if (local_140 != (timespec *)0x0) {
          iVar4 = 0;
          cVar2 = FUN_0010ea90(&local_d8);
          if (cVar2 != '\0') goto LAB_0010ed11;
        }
      }
      __tvp = (timeval *)0x0;
      if (local_140 != (timespec *)0x0) {
        local_118.tv_sec = local_140->tv_sec;
        local_108 = local_140[1].tv_sec;
        local_118.tv_usec = local_140->tv_nsec / 1000;
        __tvp = &local_118;
        lStack_100 = local_140[1].tv_nsec / 1000;
      }
      if (param_1 < 0) {
        iVar4 = futimesat(-100,param_2,__tvp);
        goto LAB_0010ed11;
      }
      iVar3 = futimesat(param_1,(char *)0x0,__tvp);
      if (iVar3 != 0) {
        iVar4 = -1;
        if (param_2 != (char *)0x0) {
          iVar4 = utimes(param_2,__tvp);
        }
        goto LAB_0010ed11;
      }
      iVar4 = 0;
      if (__tvp == (timeval *)0x0) goto LAB_0010ed11;
      bVar1 = __tvp->tv_usec < 500000;
      lVar7 = __tvp[1].tv_usec;
      if ((bVar1) && (lVar7 < 500000)) goto LAB_0010ed11;
      iVar3 = fstat(param_1,&local_d8);
      if (iVar3 != 0) goto LAB_0010ed11;
      local_f8.tv_sec = __tvp->tv_sec;
      local_f8.tv_usec = __tvp->tv_usec;
      local_e8 = __tvp[1].tv_sec;
      _Stack_e0 = __tvp[1].tv_usec;
      bVar8 = local_d8.st_mtim.tv_sec - __tvp[1].tv_sec == 1;
      if (((local_d8.st_atim.tv_sec - __tvp->tv_sec != 1) || (bVar1)) ||
         (lVar6 = FUN_0010e8e0(&local_d8), lVar6 != 0)) {
        if ((499999 < lVar7 && bVar8) && (lVar7 = FUN_0010e900(&local_d8), lVar7 == 0))
        goto LAB_0010ef5a;
      }
      else {
        local_f8.tv_usec = 0;
        if ((499999 < lVar7 && bVar8) && (lVar7 = FUN_0010e900(&local_d8), lVar7 == 0)) {
LAB_0010ef5a:
          _Stack_e0 = 0;
        }
        futimesat(param_1,(char *)0x0,&local_f8);
      }
      iVar4 = 0;
      goto LAB_0010ed11;
    }
    piVar5 = __errno_location();
    *piVar5 = 9;
  }
  else {
    local_138.tv_sec = *param_3;
    local_138.tv_nsec = param_3[1];
    local_128 = param_3[2];
    _Stack_120 = param_3[3];
    local_140 = &local_138;
    iVar3 = FUN_0010e9e0(local_140);
    if (-1 < iVar3) goto LAB_0010ebbc;
  }
LAB_0010ed0b:
  iVar4 = -1;
LAB_0010ed11:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return iVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: lutimens @ 0xef90

int lutimens(char *param_1,__time_t *param_2)

{
  char cVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  timespec *__times;
  long in_FS_OFFSET;
  timespec *local_f0;
  timespec local_e8;
  undefined1 local_d8 [16];
  stat local_c8;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 == (__time_t *)0x0) {
    iVar2 = 0;
    local_f0 = (timespec *)0x0;
    __times = (timespec *)0x0;
    if (-1 < DAT_001e0eb4) goto LAB_0010f050;
LAB_0010f134:
    DAT_001e0eb4 = -1;
    iVar2 = lstat(param_1,&local_c8);
    if (iVar2 == 0) {
LAB_0010f0bb:
      if ((local_c8.st_mode & 0xf000) != 0xa000) {
        iVar3 = FUN_0010eb50(0xffffffff,param_1,local_f0);
        goto LAB_0010f0e4;
      }
      piVar4 = __errno_location();
      *piVar4 = 0x26;
    }
  }
  else {
    local_e8.tv_sec = *param_2;
    local_e8.tv_nsec = param_2[1];
    local_d8._0_8_ = param_2[2];
    local_d8._8_8_ = param_2[3];
    __times = &local_e8;
    local_f0 = __times;
    iVar2 = FUN_0010e9e0(__times);
    if (-1 < iVar2) {
      if (-1 < DAT_001e0eb4) {
        if (iVar2 == 2) {
          iVar2 = lstat(param_1,&local_c8);
          if (iVar2 != 0) goto LAB_0010f150;
          if (local_e8.tv_nsec == 0x3ffffffe) {
            iVar2 = 3;
            local_e8 = (timespec)FUN_0010e920(&local_c8);
          }
          else {
            iVar2 = 3;
            if (local_d8._8_8_ == 0x3ffffffe) {
              local_d8 = FUN_0010e940(&local_c8);
            }
          }
        }
LAB_0010f050:
        iVar3 = utimensat(-100,param_1,__times,0x100);
        if (iVar3 < 1) {
          if (iVar3 != 0) {
            piVar4 = __errno_location();
            if (*piVar4 == 0x26) goto LAB_0010f083;
          }
          DAT_001e0eb8 = 1;
          DAT_001e0eb4 = 1;
          goto LAB_0010f0e4;
        }
        piVar4 = __errno_location();
        *piVar4 = 0x26;
      }
LAB_0010f083:
      DAT_001e0eb4 = -1;
      if (iVar2 == 0) goto LAB_0010f134;
      if (iVar2 != 3) {
        iVar2 = lstat(param_1,&local_c8);
        if (iVar2 != 0) goto LAB_0010f150;
      }
      if (__times != (timespec *)0x0) {
        iVar3 = 0;
        cVar1 = FUN_0010ea90(&local_c8,&local_f0);
        if (cVar1 != '\0') goto LAB_0010f0e4;
      }
      goto LAB_0010f0bb;
    }
  }
LAB_0010f150:
  iVar3 = -1;
LAB_0010f0e4:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return iVar3;
}




// Function: xmalloc @ 0xf230

void xmalloc(size_t param_1)

{
  void *pvVar1;
  
  pvVar1 = malloc(param_1);
  FUN_0010f220(pvVar1);
  return;
}




// Function: ximalloc @ 0xf250

void ximalloc(size_t param_1)

{
  void *pvVar1;
  
  pvVar1 = malloc(param_1);
  FUN_0010f220(pvVar1);
  return;
}




// Function: xrealloc @ 0xf280

void xrealloc(void *param_1,size_t param_2)

{
  void *pvVar1;
  
  pvVar1 = realloc(param_1,param_2);
  if ((pvVar1 == (void *)0x0) && ((param_1 == (void *)0x0 || (param_2 != 0)))) {
                    /* WARNING: Subroutine does not return */
    FUN_0010d550();
  }
  return;
}




// Function: xirealloc @ 0xf2c0

void xirealloc(void)

{
  undefined8 uVar1;
  
  uVar1 = FUN_00110ab0();
  FUN_0010f220(uVar1);
  return;
}




// Function: xreallocarray @ 0xf2e0

void xreallocarray(long param_1,long param_2,long param_3)

{
  long lVar1;
  
  lVar1 = reallocarray();
  if ((lVar1 == 0) && ((param_1 == 0 || ((param_2 != 0 && (param_3 != 0)))))) {
                    /* WARNING: Subroutine does not return */
    FUN_0010d550();
  }
  return;
}




// Function: xireallocarray @ 0xf330

void xireallocarray(void)

{
  undefined8 uVar1;
  
  uVar1 = FUN_00110ae0();
  FUN_0010f220(uVar1);
  return;
}




// Function: x2nrealloc @ 0xf370

void x2nrealloc(long param_1,ulong *param_2,ulong param_3)

{
  undefined1 auVar1 [16];
  ulong uVar2;
  ulong uVar3;
  bool bVar4;
  
  uVar3 = *param_2;
  if (param_1 == 0) {
    if (uVar3 == 0) {
      auVar1._8_8_ = 0;
      auVar1._0_8_ = param_3;
      uVar3 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / auVar1,0) + (ulong)(0x80 < param_3);
    }
  }
  else {
    uVar2 = (uVar3 >> 1) + 1;
    bVar4 = CARRY8(uVar3,uVar2);
    uVar3 = uVar3 + uVar2;
    if (bVar4) {
                    /* WARNING: Subroutine does not return */
      FUN_0010d550();
    }
  }
  FUN_0010f2e0(param_1,uVar3,param_3);
  *param_2 = uVar3;
  return;
}




// Function: xpalloc @ 0xf3f0

void xpalloc(long param_1,long *param_2,long param_3,long param_4,long param_5)

{
  long lVar1;
  long lVar2;
  long lVar3;
  
  lVar1 = *param_2;
  lVar2 = (lVar1 >> 1) + lVar1;
  if (SCARRY8(lVar1 >> 1,lVar1)) {
    lVar2 = 0x7fffffffffffffff;
  }
  lVar3 = param_4;
  if (lVar2 <= param_4) {
    lVar3 = lVar2;
  }
  if (-1 < param_4) {
    lVar2 = lVar3;
  }
  lVar3 = lVar2 * param_5;
  if (SEXT816(lVar3) == SEXT816(lVar2) * SEXT816(param_5)) {
    if (0x7f < lVar3) goto joined_r0x0010f496;
    lVar3 = 0x80;
  }
  else {
    lVar3 = 0x7fffffffffffffff;
  }
  lVar2 = lVar3 / param_5;
  lVar3 = lVar3 - lVar3 % param_5;
joined_r0x0010f496:
  if (param_1 == 0) {
    *param_2 = 0;
  }
  if ((lVar2 - lVar1 < param_3) &&
     ((lVar2 = lVar1 + param_3, SCARRY8(lVar1,param_3) ||
      (((param_4 < lVar2 && (-1 < param_4)) ||
       (lVar3 = lVar2 * param_5, SEXT816(lVar3) != SEXT816(lVar2) * SEXT816(param_5))))))) {
                    /* WARNING: Subroutine does not return */
    FUN_0010d550(param_1,lVar3);
  }
  FUN_0010f280();
  *param_2 = lVar2;
  return;
}




// Function: xcalloc @ 0xf4d0

void xcalloc(size_t param_1,size_t param_2)

{
  void *pvVar1;
  
  pvVar1 = calloc(param_1,param_2);
  FUN_0010f220(pvVar1);
  return;
}




// Function: xicalloc @ 0xf500

void xicalloc(size_t param_1,size_t param_2)

{
  void *pvVar1;
  
  pvVar1 = calloc(param_1,param_2);
  FUN_0010f220(pvVar1);
  return;
}




// Function: xmemdup @ 0xf530

void xmemdup(void *param_1,size_t param_2)

{
  void *__dest;
  
  __dest = (void *)FUN_0010f230(param_2);
  memcpy(__dest,param_1,param_2);
  return;
}




// Function: ximemdup @ 0xf560

void ximemdup(void *param_1,size_t param_2)

{
  void *__dest;
  
  __dest = (void *)FUN_0010f250(param_2);
  memcpy(__dest,param_1,param_2);
  return;
}




// Function: ximemdup0 @ 0xf590

void ximemdup0(void *param_1,size_t param_2)

{
  void *__dest;
  
  __dest = (void *)FUN_0010f250(param_2 + 1);
  *(undefined1 *)((long)__dest + param_2) = 0;
  memcpy(__dest,param_1,param_2);
  return;
}




// Function: xstrdup @ 0xf5d0

void xstrdup(char *param_1)

{
  size_t sVar1;
  
  sVar1 = strlen(param_1);
  FUN_0010f530(param_1,sVar1 + 1);
  return;
}




// Function: yesno @ 0xf5f0

ulong yesno(void)

{
  uint uVar1;
  uint uVar2;
  undefined8 unaff_R12;
  
  uVar1 = getchar();
  uVar2 = uVar1 & 0xffffffdf;
  while ((uVar1 != 10 && (uVar1 != 0xffffffff))) {
    uVar1 = getchar();
  }
  return CONCAT71((int7)((ulong)unaff_R12 >> 8),uVar2 == 0x59) & 0xffffffff;
}




// Function: rpl_fprintf @ 0xf630

ulong rpl_fprintf(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                  undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8,
                  FILE *param_9,undefined8 param_10,undefined8 param_11,undefined8 param_12,
                  undefined8 param_13,undefined8 param_14)

{
  size_t sVar1;
  char in_AL;
  undefined1 *__ptr;
  size_t sVar2;
  int *piVar3;
  ulong uVar4;
  long in_FS_OFFSET;
  size_t local_8d8;
  undefined4 local_8d0;
  undefined4 local_8cc;
  undefined1 *local_8c8;
  undefined1 *local_8c0;
  undefined1 local_8b8 [2008];
  long local_e0;
  undefined1 local_d8 [16];
  undefined8 local_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined8 local_b0;
  undefined8 local_a8;
  undefined8 local_98;
  undefined8 local_88;
  undefined8 local_78;
  undefined8 local_68;
  undefined8 local_58;
  undefined8 local_48;
  undefined8 local_38;
  
  if (in_AL != '\0') {
    local_a8 = param_1;
    local_98 = param_2;
    local_88 = param_3;
    local_78 = param_4;
    local_68 = param_5;
    local_58 = param_6;
    local_48 = param_7;
    local_38 = param_8;
  }
  local_e0 = *(long *)(in_FS_OFFSET + 0x28);
  local_8c8 = &stack0x00000008;
  local_8d8 = 2000;
  local_8c0 = local_d8;
  local_8d0 = 0x10;
  local_8cc = 0x30;
  local_c8 = param_11;
  local_c0 = param_12;
  local_b8 = param_13;
  local_b0 = param_14;
  __ptr = (undefined1 *)FUN_0010f880(local_8b8,&local_8d8,param_10,&local_8d0);
  sVar1 = local_8d8;
  if (__ptr != (undefined1 *)0x0) {
    sVar2 = fwrite(__ptr,1,local_8d8,param_9);
    if (sVar2 < sVar1) {
      uVar4 = 0xffffffff;
      if (__ptr != local_8b8) {
        free(__ptr);
        uVar4 = 0xffffffff;
      }
      goto LAB_0010f748;
    }
    if (__ptr != local_8b8) {
      free(__ptr);
    }
    if (sVar1 < 0x80000000) {
      uVar4 = sVar1 & 0xffffffff;
      goto LAB_0010f748;
    }
    piVar3 = __errno_location();
    *piVar3 = 0x4b;
  }
  FUN_00110a30(param_9);
  uVar4 = 0xffffffff;
LAB_0010f748:
  if (local_e0 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: __printf__ @ 0xf7b0

void __printf__(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8,
                 undefined8 param_9,undefined8 param_10,undefined8 param_11,undefined8 param_12,
                 undefined8 param_13,undefined8 param_14)

{
  char in_AL;
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
  FUN_001108a0(stdout,param_9,&local_d8);
  if (local_c0 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: vasnprintf @ 0xf880

void * vasnprintf(void *param_1,ulong *param_2,int *param_3,undefined8 param_4)

{
  unkbyte10 Var1;
  undefined1 uVar2;
  uint uVar3;
  long lVar4;
  size_t *psVar5;
  size_t __n;
  int iVar6;
  ulong uVar7;
  ulong uVar8;
  void *pvVar9;
  int *piVar10;
  int *__n_00;
  undefined8 uVar11;
  ulong uVar12;
  void *pvVar13;
  undefined1 *puVar14;
  undefined1 *puVar15;
  ulong __n_01;
  undefined1 *puVar16;
  ulong uVar17;
  undefined8 *puVar18;
  long in_FS_OFFSET;
  bool bVar19;
  undefined1 auStack_428 [8];
  ulong local_420;
  int local_414;
  undefined1 *local_410;
  ulong *local_408;
  undefined1 *local_400;
  long local_3f8;
  undefined1 *local_3f0;
  undefined8 local_3e8;
  int *local_3e0;
  size_t local_3d8;
  void *local_3d0;
  int local_3c4;
  uint local_3c0 [2];
  undefined1 local_3b8 [8];
  int *local_3b0;
  int local_3a8 [56];
  long local_2c8;
  undefined8 *local_2c0;
  undefined8 local_2b8;
  undefined8 local_2b0;
  undefined8 local_2a8 [77];
  long local_40;
  
  puVar14 = auStack_428;
  puVar15 = auStack_428;
  puVar16 = auStack_428;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_408 = param_2;
  local_3d0 = param_1;
  iVar6 = FUN_00110dd0(param_3,&local_2c8,local_3b8);
  if (-1 < iVar6) {
    iVar6 = FUN_00110ba0(param_4,local_3b8);
    if (iVar6 < 0) {
      if (local_2c0 != local_2a8) {
        free(local_2c0);
      }
      if (local_3b0 != local_3a8) {
        free(local_3b0);
      }
      piVar10 = __errno_location();
      pvVar9 = (void *)0x0;
      *piVar10 = 0x16;
      puVar14 = auStack_428;
      goto LAB_0010fc0a;
    }
    uVar7 = FUN_00110b70(7,local_2b8,local_2b0,6);
    if (uVar7 < 4000) {
      puVar14 = auStack_428;
      while (puVar15 != auStack_428 + -(uVar7 + 0x17 & 0xfffffffffffff000)) {
        puVar16 = puVar14 + -0x1000;
        *(undefined8 *)(puVar14 + -8) = *(undefined8 *)(puVar14 + -8);
        puVar15 = puVar14 + -0x1000;
        puVar14 = puVar14 + -0x1000;
      }
      uVar7 = (ulong)((uint)(uVar7 + 0x17) & 0xff0);
      lVar4 = -uVar7;
      if (uVar7 != 0) {
        *(undefined8 *)(puVar16 + -8) = *(undefined8 *)(puVar16 + -8);
      }
      local_410 = (undefined1 *)0x0;
      local_3f0 = (undefined1 *)((ulong)(puVar16 + lVar4 + 0xf) & 0xfffffffffffffff0);
      puVar14 = puVar16 + lVar4;
      puVar16 = local_410;
LAB_0010f941:
      local_410 = puVar16;
      uVar7 = 0;
      if (local_3d0 != (void *)0x0) {
        uVar7 = *local_408;
      }
      __n_01 = 0;
      local_3f8 = 0;
      piVar10 = (int *)*local_2c0;
      puVar18 = local_2c0;
      pvVar13 = local_3d0;
      if (piVar10 == param_3) goto LAB_0010fa3a;
      do {
        local_3e0 = param_3;
        local_3d8 = (long)piVar10 - (long)param_3;
        *(undefined8 *)(puVar14 + -8) = 0x10f9a4;
        uVar8 = FUN_00110b30(__n_01,(long)piVar10 - (long)param_3);
        piVar10 = local_3e0;
        pvVar9 = pvVar13;
        if (uVar8 <= uVar7) goto LAB_0010fa2b;
        if (uVar7 != 0) {
          if (-1 < (long)uVar7) {
            uVar7 = uVar7 * 2;
            goto LAB_0010f9cc;
          }
LAB_0010fb98:
          *(undefined8 *)(puVar14 + -8) = 0x10fb9d;
          local_3e0 = __errno_location();
LAB_0010fba4:
          if ((pvVar13 == local_3d0) || (pvVar13 == (void *)0x0)) goto LAB_0010fbb6;
          goto LAB_00110460;
        }
        uVar7 = 0xc;
LAB_0010f9cc:
        if (uVar7 < uVar8) {
          uVar7 = uVar8;
        }
        if (uVar7 == 0xffffffffffffffff) goto LAB_0010fb98;
        if ((pvVar13 == (void *)0x0) || (pvVar13 == local_3d0)) {
          local_3e8 = local_3e0;
          local_3e0 = (int *)CONCAT71(local_3e0._1_7_,pvVar13 == local_3d0);
          *(undefined8 *)(puVar14 + -8) = 0x11038c;
          pvVar9 = malloc(uVar7);
          if (pvVar9 == (void *)0x0) goto LAB_0010fb98;
          piVar10 = local_3e8;
          if ((__n_01 != 0) && (piVar10 = local_3e8, (char)local_3e0 != '\0')) {
            local_3e0 = local_3e8;
            *(undefined8 *)(puVar14 + -8) = 0x1103da;
            pvVar9 = memcpy(pvVar9,pvVar13,__n_01);
            piVar10 = local_3e0;
          }
        }
        else {
          *(undefined8 *)(puVar14 + -8) = 0x10fa11;
          pvVar9 = realloc(pvVar13,uVar7);
          piVar10 = local_3e0;
          if (pvVar9 == (void *)0x0) goto LAB_0011044e;
        }
LAB_0010fa2b:
        __n = local_3d8;
        *(undefined8 *)(puVar14 + -8) = 0x10fa3a;
        memcpy((void *)((long)pvVar9 + __n_01),piVar10,__n);
        __n_01 = uVar8;
        pvVar13 = pvVar9;
LAB_0010fa3a:
        do {
          if (local_2c8 == local_3f8) {
            *(undefined8 *)(puVar14 + -8) = 0x110765;
            uVar8 = FUN_00110b30(__n_01,1);
            pvVar9 = pvVar13;
            if (uVar7 < uVar8) {
              if (uVar7 == 0) {
                uVar17 = 0xc;
              }
              else {
                uVar17 = uVar7 * 2;
                if ((long)uVar7 < 0) goto LAB_0010fb98;
              }
              uVar7 = uVar17;
              if (uVar17 <= uVar8) {
                uVar7 = uVar8;
              }
              if (uVar7 == 0xffffffffffffffff) goto LAB_0010fb98;
              bVar19 = pvVar13 == local_3d0;
              if ((pvVar13 == (void *)0x0) || (bVar19)) {
                *(undefined8 *)(puVar14 + -8) = 0x110845;
                pvVar9 = malloc(uVar7);
                if (pvVar9 == (void *)0x0) goto LAB_0010fb98;
                if ((__n_01 != 0) && (bVar19)) {
                  *(undefined8 *)(puVar14 + -8) = 0x110869;
                  pvVar9 = memcpy(pvVar9,pvVar13,__n_01);
                }
              }
              else {
                *(undefined8 *)(puVar14 + -8) = 0x1107b9;
                pvVar9 = realloc(pvVar13,uVar7);
                if (pvVar9 == (void *)0x0) goto LAB_0011044e;
              }
            }
            *(undefined1 *)((long)pvVar9 + __n_01) = 0;
            if ((pvVar9 != local_3d0) && (__n_01 + 1 < uVar7)) {
              *(undefined8 *)(puVar14 + -8) = 0x110879;
              pvVar13 = realloc(pvVar9,__n_01 + 1);
              if (pvVar13 != (void *)0x0) {
                pvVar9 = pvVar13;
              }
            }
            puVar16 = local_410;
            if (local_410 != (undefined1 *)0x0) {
              *(undefined8 *)(puVar14 + -8) = 0x1107f4;
              free(puVar16);
            }
            if (local_2c0 != local_2a8) {
              *(undefined8 *)(puVar14 + -8) = 0x11080c;
              free(local_2c0);
            }
            if (local_3b0 != local_3a8) {
              *(undefined8 *)(puVar14 + -8) = 0x110824;
              free(local_3b0);
            }
            *local_408 = __n_01;
            goto LAB_0010fc0a;
          }
          lVar4 = puVar18[10];
          if (*(char *)(puVar18 + 9) == '%') {
            if (lVar4 != -1) goto switchD_0010fde6_caseD_0;
            *(undefined8 *)(puVar14 + -8) = 0x1102d7;
            uVar8 = FUN_00110b30(__n_01,1);
            pvVar9 = pvVar13;
            if (uVar7 < uVar8) {
              if (uVar7 == 0) {
                uVar7 = 0xc;
              }
              else {
                if ((long)uVar7 < 0) goto LAB_0010fb98;
                uVar7 = uVar7 * 2;
              }
              if (uVar7 < uVar8) {
                uVar7 = uVar8;
              }
              if (uVar7 == 0xffffffffffffffff) goto LAB_0010fb98;
              if ((pvVar13 == (void *)0x0) || (pvVar13 == local_3d0)) {
                local_3d8 = CONCAT71(local_3d8._1_7_,pvVar13 == local_3d0);
                *(undefined8 *)(puVar14 + -8) = 0x1105fe;
                pvVar9 = malloc(uVar7);
                if (pvVar9 == (void *)0x0) goto LAB_0010fb98;
                if ((__n_01 != 0) && ((char)local_3d8 != '\0')) {
                  *(undefined8 *)(puVar14 + -8) = 0x110630;
                  pvVar9 = memcpy(pvVar9,pvVar13,__n_01);
                }
              }
              else {
                *(undefined8 *)(puVar14 + -8) = 0x110324;
                pvVar9 = realloc(pvVar13,uVar7);
                if (pvVar9 == (void *)0x0) goto LAB_0011044e;
              }
            }
            pvVar13 = pvVar9;
            *(undefined1 *)((long)pvVar13 + __n_01) = 0x25;
            __n_01 = uVar8;
          }
          else {
            if (lVar4 == -1) {
switchD_0010fde6_caseD_0:
                    /* WARNING: Subroutine does not return */
              *(undefined8 *)(puVar14 + -8) = 0x11088a;
              abort();
            }
            iVar6 = local_3b0[lVar4 * 8];
            local_3d8 = CONCAT44(local_3d8._4_4_,iVar6);
            if (*(char *)(puVar18 + 9) == 'n') {
              if (4 < iVar6 - 0x12U) goto switchD_0010fde6_caseD_0;
              psVar5 = *(size_t **)(local_3b0 + lVar4 * 8 + 4);
              switch(iVar6) {
              case 0x12:
                *(char *)psVar5 = (char)__n_01;
                break;
              case 0x13:
                *(short *)psVar5 = (short)__n_01;
                break;
              case 0x14:
                *(int *)psVar5 = (int)__n_01;
                break;
              default:
                *psVar5 = __n_01;
              }
            }
            else {
              uVar3 = *(uint *)(puVar18 + 2);
              puVar16 = local_3f0 + 1;
              *local_3f0 = 0x25;
              if ((uVar3 & 1) != 0) {
                local_3f0[1] = 0x27;
                puVar16 = local_3f0 + 2;
              }
              if ((uVar3 & 2) != 0) {
                *puVar16 = 0x2d;
                puVar16 = puVar16 + 1;
              }
              if ((uVar3 & 4) != 0) {
                *puVar16 = 0x2b;
                puVar16 = puVar16 + 1;
              }
              if ((uVar3 & 8) != 0) {
                *puVar16 = 0x20;
                puVar16 = puVar16 + 1;
              }
              if ((uVar3 & 0x10) != 0) {
                *puVar16 = 0x23;
                puVar16 = puVar16 + 1;
              }
              if ((uVar3 & 0x40) != 0) {
                *puVar16 = 0x49;
                puVar16 = puVar16 + 1;
              }
              if ((uVar3 & 0x20) != 0) {
                *puVar16 = 0x30;
                puVar16 = puVar16 + 1;
              }
              pvVar9 = (void *)puVar18[3];
              piVar10 = local_3b0;
              if (pvVar9 != (void *)puVar18[4]) {
                piVar10 = (int *)((long)puVar18[4] - (long)pvVar9);
                local_3e8 = local_3b0;
                local_3e0 = piVar10;
                *(undefined8 *)(puVar14 + -8) = 0x10fb25;
                memcpy(puVar16,pvVar9,(size_t)piVar10);
                puVar16 = puVar16 + (long)local_3e0;
                piVar10 = local_3e8;
              }
              pvVar9 = (void *)puVar18[6];
              if (pvVar9 != (void *)puVar18[7]) {
                __n_00 = (int *)((long)puVar18[7] - (long)pvVar9);
                local_3e8 = piVar10;
                local_3e0 = __n_00;
                *(undefined8 *)(puVar14 + -8) = 0x10fb5f;
                memcpy(puVar16,pvVar9,(size_t)__n_00);
                puVar16 = puVar16 + (long)local_3e0;
                piVar10 = local_3e8;
              }
              switch((undefined4)local_3d8) {
              case 9:
              case 10:
                *puVar16 = 0x6c;
                puVar16 = puVar16 + 1;
              case 7:
              case 8:
              case 0xe:
              case 0x10:
                *puVar16 = 0x6c;
                puVar16 = puVar16 + 1;
                break;
              case 0xc:
                *puVar16 = 0x4c;
                puVar16 = puVar16 + 1;
              }
              uVar2 = *(undefined1 *)(puVar18 + 9);
              puVar16[1] = 0;
              *puVar16 = uVar2;
              if (puVar18[5] == -1) {
                local_3e8 = (int *)((ulong)local_3e8._4_4_ << 0x20);
              }
              else {
                if (piVar10[puVar18[5] * 8] != 5) goto switchD_0010fde6_caseD_0;
                local_3e8 = (int *)CONCAT44(local_3e8._4_4_,1);
                local_3c0[0] = (piVar10 + puVar18[5] * 8)[4];
              }
              if (puVar18[8] != -1) {
                if (piVar10[puVar18[8] * 8] != 5) goto switchD_0010fde6_caseD_0;
                local_3c0[(ulong)local_3e8 & 0xffffffff] = (piVar10 + puVar18[8] * 8)[4];
                local_3e8 = (int *)CONCAT44(local_3e8._4_4_,(int)local_3e8 + 1);
              }
              *(undefined8 *)(puVar14 + -8) = 0x10fd2a;
              uVar8 = FUN_00110b30(__n_01,2);
              pvVar9 = pvVar13;
              if (uVar7 < uVar8) {
                if (uVar7 == 0) {
                  uVar7 = 0xc;
                }
                else {
                  if ((long)uVar7 < 0) goto LAB_0010fb98;
                  uVar7 = uVar7 * 2;
                }
                if (uVar7 <= uVar8) {
                  uVar7 = uVar8;
                }
                if (uVar7 == 0xffffffffffffffff) goto LAB_0010fb98;
                if ((pvVar13 == (void *)0x0) || (pvVar13 == local_3d0)) {
                  local_3e0 = (int *)CONCAT71(local_3e0._1_7_,pvVar13 == local_3d0);
                  *(undefined8 *)(puVar14 + -8) = 0x1105ae;
                  pvVar9 = malloc(uVar7);
                  if (pvVar9 == (void *)0x0) goto LAB_0010fb98;
                  if ((__n_01 != 0) && ((char)local_3e0 != '\0')) {
                    *(undefined8 *)(puVar14 + -8) = 0x1105e0;
                    pvVar9 = memcpy(pvVar9,pvVar13,__n_01);
                  }
                }
                else {
                  *(undefined8 *)(puVar14 + -8) = 0x10fd74;
                  pvVar9 = realloc(pvVar13,uVar7);
                  if (pvVar9 == (void *)0x0) goto LAB_0010fb98;
                }
              }
              *(undefined1 *)((long)pvVar9 + __n_01) = 0;
              *(undefined8 *)(puVar14 + -8) = 0x10fd8a;
              local_3e0 = __errno_location();
              local_414 = *local_3e0;
              local_400 = puVar16;
LAB_0010fda0:
              pvVar13 = pvVar9;
              uVar3 = local_3c0[0];
              puVar16 = local_3f0;
              local_3c4 = -1;
              uVar17 = uVar7 - __n_01;
              *local_3e0 = 0;
              uVar8 = 0x7fffffff;
              if (uVar17 < 0x80000000) {
                uVar8 = uVar17;
              }
              pvVar9 = pvVar13;
              switch(local_3d8 & 0xffffffff) {
              default:
                goto switchD_0010fde6_caseD_0;
              case 1:
                uVar12 = (ulong)(uint)(int)(char)local_3b0[puVar18[10] * 8 + 4];
                break;
              case 2:
                uVar12 = (ulong)*(byte *)(local_3b0 + puVar18[10] * 8 + 4);
                break;
              case 3:
                uVar12 = (ulong)(uint)(int)(short)local_3b0[puVar18[10] * 8 + 4];
                break;
              case 4:
                uVar12 = (ulong)*(ushort *)(local_3b0 + puVar18[10] * 8 + 4);
                break;
              case 5:
              case 6:
              case 0xd:
              case 0xe:
                uVar12 = (ulong)(uint)local_3b0[puVar18[10] * 8 + 4];
                break;
              case 7:
              case 8:
              case 9:
              case 10:
              case 0xf:
              case 0x10:
              case 0x11:
                piVar10 = *(int **)(local_3b0 + puVar18[10] * 8 + 4);
                if ((int)local_3e8 == 1) {
                  *(int **)(puVar14 + -8) = &local_3c4;
                  *(int **)(puVar14 + -0x10) = piVar10;
                  goto LAB_0011014a;
                }
                if ((int)local_3e8 != 2) {
                  *(int **)(puVar14 + -0x10) = &local_3c4;
LAB_0010fe2b:
                  puVar16 = local_3f0;
                  local_420 = uVar8;
                  *(undefined8 *)(puVar14 + -0x18) = 0x10fe4c;
                  iVar6 = __snprintf_chk((long)pvVar13 + __n_01,uVar8,1,0xffffffffffffffff,puVar16,
                                         piVar10);
                  goto LAB_0010fe55;
                }
                *(int **)(puVar14 + -0x10) = &local_3c4;
                *(int **)(puVar14 + -0x18) = piVar10;
                goto LAB_0011019e;
              case 0xb:
                uVar11 = *(undefined8 *)(local_3b0 + puVar18[10] * 8 + 4);
                if ((int)local_3e8 == 1) {
                  *(int **)(puVar14 + -0x10) = &local_3c4;
                }
                else {
                  if ((int)local_3e8 != 2) {
                    local_420 = uVar8;
                    *(undefined8 *)(puVar14 + -8) = 0x1104ab;
                    iVar6 = __snprintf_chk(uVar11,(long)pvVar13 + __n_01,uVar8,1,0xffffffffffffffff,
                                           puVar16,&local_3c4);
                    goto LAB_0010fe55;
                  }
                  *(int **)(puVar14 + -8) = &local_3c4;
                  *(ulong *)(puVar14 + -0x10) = (ulong)local_3c0[1];
                }
                uVar3 = local_3c0[0];
                puVar16 = local_3f0;
                local_420 = uVar8;
                *(undefined8 *)(puVar14 + -0x18) = 0x110102;
                iVar6 = __snprintf_chk(uVar11,(long)pvVar13 + __n_01,uVar8,1,0xffffffffffffffff,
                                       puVar16,uVar3);
                goto LAB_0010fe55;
              case 0xc:
                Var1 = *(unkbyte10 *)(local_3b0 + puVar18[10] * 8 + 4);
                if ((int)local_3e8 != 1) {
                  if ((int)local_3e8 != 2) {
                    piVar10 = &local_3c4;
                    *(unkbyte10 *)(puVar14 + -0x10) = Var1;
                    goto LAB_0010fe2b;
                  }
                  *(int **)(puVar14 + -0x10) = &local_3c4;
                  uVar12 = (ulong)local_3c0[1];
                  local_420 = uVar8;
                  *(unkbyte10 *)(puVar14 + -0x20) = Var1;
                  *(ulong *)(puVar14 + -0x30) = uVar12;
                  *(undefined8 *)(puVar14 + -0x38) = 0x110503;
                  iVar6 = __snprintf_chk((long)pvVar13 + __n_01,uVar8,1,0xffffffffffffffff,puVar16,
                                         uVar3);
                  goto LAB_0010fe55;
                }
                *(int **)(puVar14 + -0x10) = &local_3c4;
                *(unkbyte10 *)(puVar14 + -0x20) = Var1;
                goto LAB_001101a5;
              }
              if ((int)local_3e8 == 1) {
                *(int **)(puVar14 + -8) = &local_3c4;
                *(ulong *)(puVar14 + -0x10) = uVar12;
LAB_0011014a:
                uVar12 = (ulong)local_3c0[0];
LAB_0010ff6b:
                puVar16 = local_3f0;
                local_420 = uVar8;
                *(undefined8 *)(puVar14 + -0x18) = 0x10ff8c;
                iVar6 = __snprintf_chk((long)pvVar13 + __n_01,uVar8,1,0xffffffffffffffff,puVar16,
                                       uVar12);
                if (-1 < local_3c4) goto LAB_0010fe63;
LAB_0010ffa3:
                if (local_400[1] == '\0') {
                  if (-1 < iVar6) goto LAB_0010fe7d;
                  if (*local_3e0 == 0) {
                    iVar6 = 0x16;
                    if ((*(byte *)(puVar18 + 9) & 0xef) == 99) {
                      iVar6 = 0x54;
                    }
                    *local_3e0 = iVar6;
                  }
                  if ((pvVar13 != local_3d0) && (pvVar13 != (void *)0x0)) {
                    *(undefined8 *)(puVar14 + -8) = 0x10ffff;
                    free(pvVar13);
                  }
                  puVar16 = local_410;
                  if (local_410 != (undefined1 *)0x0) {
                    *(undefined8 *)(puVar14 + -8) = 0x110013;
                    free(puVar16);
                  }
                  if (local_2c0 != local_2a8) {
                    *(undefined8 *)(puVar14 + -8) = 0x11002b;
                    free(local_2c0);
                  }
                  if (local_3b0 != local_3a8) {
                    *(undefined8 *)(puVar14 + -8) = 0x110047;
                    free(local_3b0);
                  }
                  goto LAB_0010fc07;
                }
                local_400[1] = 0;
                goto LAB_0010fda0;
              }
              if ((int)local_3e8 != 2) {
                *(int **)(puVar14 + -0x10) = &local_3c4;
                goto LAB_0010ff6b;
              }
              *(int **)(puVar14 + -0x10) = &local_3c4;
              *(ulong *)(puVar14 + -0x18) = uVar12;
LAB_0011019e:
              *(ulong *)(puVar14 + -0x20) = (ulong)local_3c0[1];
LAB_001101a5:
              uVar3 = local_3c0[0];
              puVar16 = local_3f0;
              local_420 = uVar8;
              *(undefined8 *)(puVar14 + -0x28) = 0x1101cd;
              iVar6 = __snprintf_chk((long)pvVar13 + __n_01,uVar8,1,0xffffffffffffffff,puVar16,uVar3
                                    );
LAB_0010fe55:
              if (local_3c4 < 0) goto LAB_0010ffa3;
LAB_0010fe63:
              if (((ulong)(long)local_3c4 < local_420) &&
                 (*(char *)((long)pvVar13 + __n_01 + (long)local_3c4) != '\0'))
              goto switchD_0010fde6_caseD_0;
              if (local_3c4 < iVar6) {
LAB_0010fe7d:
                local_3c4 = iVar6;
              }
              if (local_420 <= local_3c4 + 1) {
                if (0x7ffffffe < uVar17) {
                  if ((pvVar13 != local_3d0) && (pvVar13 != (void *)0x0)) {
                    *(undefined8 *)(puVar14 + -8) = 0x1106ce;
                    free(pvVar13);
                  }
                  puVar16 = local_410;
                  if (local_410 != (undefined1 *)0x0) {
                    *(undefined8 *)(puVar14 + -8) = 0x1106e2;
                    free(puVar16);
                  }
                  if (local_2c0 != local_2a8) {
                    *(undefined8 *)(puVar14 + -8) = 0x1106fa;
                    free(local_2c0);
                  }
                  if (local_3b0 != local_3a8) {
                    *(undefined8 *)(puVar14 + -8) = 0x110712;
                    free(local_3b0);
                  }
                  pvVar9 = (void *)0x0;
                  *local_3e0 = 0x4b;
                  goto LAB_0010fc0a;
                }
                iVar6 = local_3c4 + 2;
                uVar8 = uVar7 * 2;
                if ((long)uVar7 < 0) {
                  uVar8 = 0xffffffffffffffff;
                }
                *(undefined8 *)(puVar14 + -8) = 0x10febb;
                uVar11 = FUN_00110b30(__n_01,iVar6);
                *(undefined8 *)(puVar14 + -8) = 0x10fec6;
                uVar17 = FUN_00110b90(uVar11,uVar8);
                if (uVar7 < uVar17) {
                  if (uVar7 == 0) {
                    uVar8 = 0xc;
                  }
                  uVar7 = uVar17;
                  if (uVar17 <= uVar8) {
                    uVar7 = uVar8;
                  }
                  if (uVar7 == 0xffffffffffffffff) goto LAB_0010fba4;
                  bVar19 = pvVar13 == local_3d0;
                  if ((pvVar13 == (void *)0x0) || (bVar19)) {
                    *(undefined8 *)(puVar14 + -8) = 0x110248;
                    pvVar9 = malloc(uVar7);
                    if (pvVar9 == (void *)0x0) goto LAB_0010fba4;
                    if ((__n_01 != 0) && (bVar19)) {
                      *(undefined8 *)(puVar14 + -8) = 0x110274;
                      pvVar9 = memcpy(pvVar9,pvVar13,__n_01);
                    }
                  }
                  else {
                    *(undefined8 *)(puVar14 + -8) = 0x10ff17;
                    pvVar9 = realloc(pvVar13,uVar7);
                    if (pvVar9 == (void *)0x0) goto LAB_0010fba4;
                  }
                }
                goto LAB_0010fda0;
              }
              __n_01 = (long)local_3c4 + __n_01;
              *local_3e0 = local_414;
            }
          }
          param_3 = (int *)puVar18[1];
          piVar10 = (int *)puVar18[0xb];
          puVar18 = puVar18 + 0xb;
          local_3f8 = local_3f8 + 1;
        } while (piVar10 == param_3);
      } while( true );
    }
    if ((uVar7 != 0xffffffffffffffff) &&
       (local_3f0 = malloc(uVar7), puVar14 = auStack_428, puVar16 = local_3f0,
       local_3f0 != (undefined1 *)0x0)) goto LAB_0010f941;
    local_3e0 = __errno_location();
    puVar14 = auStack_428;
    goto LAB_0010fbca;
  }
  goto LAB_0010fc07;
LAB_0011044e:
  *(undefined8 *)(puVar14 + -8) = 0x110453;
  local_3e0 = __errno_location();
LAB_00110460:
  *(undefined8 *)(puVar14 + -8) = 0x110468;
  free(pvVar13);
LAB_0010fbb6:
  puVar16 = local_410;
  if (local_410 != (undefined1 *)0x0) {
    *(undefined8 *)(puVar14 + -8) = 0x10fbca;
    free(puVar16);
  }
LAB_0010fbca:
  if (local_2c0 != local_2a8) {
    *(undefined8 *)(puVar14 + -8) = 0x10fbe2;
    free(local_2c0);
  }
  if (local_3b0 != local_3a8) {
    *(undefined8 *)(puVar14 + -8) = 0x10fbfa;
    free(local_3b0);
  }
  *local_3e0 = 0xc;
LAB_0010fc07:
  pvVar9 = (void *)0x0;
LAB_0010fc0a:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return pvVar9;
  }
                    /* WARNING: Subroutine does not return */
  *(undefined **)(puVar14 + -8) = &UNK_00110897;
  __stack_chk_fail();
}




// Function: rpl_vfprintf @ 0x108a0

ulong rpl_vfprintf(FILE *param_1,undefined8 param_2,undefined8 param_3)

{
  size_t sVar1;
  undefined1 *__ptr;
  size_t sVar2;
  int *piVar3;
  ulong uVar4;
  long in_FS_OFFSET;
  size_t local_810;
  undefined1 local_808 [2008];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_810 = 2000;
  __ptr = (undefined1 *)FUN_0010f880(local_808,&local_810,param_2,param_3);
  sVar1 = local_810;
  if (__ptr != (undefined1 *)0x0) {
    sVar2 = fwrite(__ptr,1,local_810,param_1);
    if (sVar2 < sVar1) {
      uVar4 = 0xffffffff;
      if (__ptr != local_808) {
        free(__ptr);
        uVar4 = 0xffffffff;
      }
      goto LAB_0011092c;
    }
    if (__ptr != local_808) {
      free(__ptr);
    }
    if (sVar1 < 0x80000000) {
      uVar4 = sVar1 & 0xffffffff;
      goto LAB_0011092c;
    }
    piVar3 = __errno_location();
    *piVar3 = 0x4b;
  }
  FUN_00110a30(param_1);
  uVar4 = 0xffffffff;
LAB_0011092c:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: opendir_safer @ 0x10990

DIR * opendir_safer(char *param_1)

{
  uint uVar1;
  int __fd;
  DIR *__dirp;
  int *piVar2;
  DIR *pDVar3;
  int iVar4;
  
  __dirp = opendir(param_1);
  if (__dirp != (DIR *)0x0) {
    uVar1 = dirfd(__dirp);
    if (uVar1 < 3) {
      __fd = FUN_0010e270(uVar1,0x406,3);
      piVar2 = __errno_location();
      if (__fd < 0) {
        iVar4 = *piVar2;
        pDVar3 = (DIR *)0x0;
      }
      else {
        pDVar3 = fdopendir(__fd);
        iVar4 = *piVar2;
        if (pDVar3 == (DIR *)0x0) {
          close(__fd);
        }
      }
      closedir(__dirp);
      *piVar2 = iVar4;
      return pDVar3;
    }
  }
  return __dirp;
}




// Function: fseterr @ 0x10a30

void fseterr(uint *param_1)

{
  *param_1 = *param_1 | 0x20;
  return;
}




// Function: gettime @ 0x10a40

void gettime(timespec *param_1)

{
  clock_gettime(0,param_1);
  return;
}




// Function: current_timespec @ 0x10a50

undefined8 current_timespec(void)

{
  long in_FS_OFFSET;
  undefined8 local_28 [3];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00110a40(local_28);
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_28[0];
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: dup_safer @ 0x10b10

void dup_safer(undefined8 param_1)

{
  FUN_0010e270(param_1,0,3);
  return;
}




// Function: printf_fetchargs @ 0x10ba0

undefined8 printf_fetchargs(uint *param_1,long *param_2)

{
  uint uVar1;
  long lVar2;
  undefined4 *puVar3;
  unkbyte10 *pVar4;
  undefined *puVar5;
  char *pcVar6;
  long lVar7;
  undefined8 *puVar8;
  undefined4 *puVar9;
  long *plVar10;
  
  lVar2 = *param_2;
  puVar3 = (undefined4 *)param_2[1];
  if (lVar2 != 0) {
    lVar7 = 0;
    do {
      switch(*puVar3) {
      default:
        return 0xffffffff;
      case 1:
      case 2:
        uVar1 = *param_1;
        if (uVar1 < 0x30) {
          puVar9 = (undefined4 *)((ulong)uVar1 + *(long *)(param_1 + 4));
          *param_1 = uVar1 + 8;
        }
        else {
          puVar9 = *(undefined4 **)(param_1 + 2);
          *(undefined4 **)(param_1 + 2) = puVar9 + 2;
        }
        *(char *)(puVar3 + 4) = (char)*puVar9;
        break;
      case 3:
      case 4:
        uVar1 = *param_1;
        if (uVar1 < 0x30) {
          puVar9 = (undefined4 *)((ulong)uVar1 + *(long *)(param_1 + 4));
          *param_1 = uVar1 + 8;
        }
        else {
          puVar9 = *(undefined4 **)(param_1 + 2);
          *(undefined4 **)(param_1 + 2) = puVar9 + 2;
        }
        *(short *)(puVar3 + 4) = (short)*puVar9;
        break;
      case 5:
      case 6:
      case 0xd:
      case 0xe:
        uVar1 = *param_1;
        if (uVar1 < 0x30) {
          puVar9 = (undefined4 *)((ulong)uVar1 + *(long *)(param_1 + 4));
          *param_1 = uVar1 + 8;
        }
        else {
          puVar9 = *(undefined4 **)(param_1 + 2);
          *(undefined4 **)(param_1 + 2) = puVar9 + 2;
        }
        puVar3[4] = *puVar9;
        break;
      case 7:
      case 8:
      case 9:
      case 10:
      case 0x11:
      case 0x12:
      case 0x13:
      case 0x14:
      case 0x15:
      case 0x16:
        uVar1 = *param_1;
        if (uVar1 < 0x30) {
          puVar8 = (undefined8 *)((ulong)uVar1 + *(long *)(param_1 + 4));
          *param_1 = uVar1 + 8;
        }
        else {
          puVar8 = *(undefined8 **)(param_1 + 2);
          *(undefined8 **)(param_1 + 2) = puVar8 + 1;
        }
        *(undefined8 *)(puVar3 + 4) = *puVar8;
        break;
      case 0xb:
        uVar1 = param_1[1];
        if (uVar1 < 0xb0) {
          puVar8 = (undefined8 *)((ulong)uVar1 + *(long *)(param_1 + 4));
          param_1[1] = uVar1 + 0x10;
        }
        else {
          puVar8 = *(undefined8 **)(param_1 + 2);
          *(undefined8 **)(param_1 + 2) = puVar8 + 1;
        }
        *(undefined8 *)(puVar3 + 4) = *puVar8;
        break;
      case 0xc:
        pVar4 = (unkbyte10 *)(*(long *)(param_1 + 2) + 0xfU & 0xfffffffffffffff0);
        *(unkbyte10 **)(param_1 + 2) = pVar4 + 1;
        *(unkbyte10 *)(puVar3 + 4) = *pVar4;
        break;
      case 0xf:
        uVar1 = *param_1;
        if (uVar1 < 0x30) {
          plVar10 = (long *)((ulong)uVar1 + *(long *)(param_1 + 4));
          *param_1 = uVar1 + 8;
        }
        else {
          plVar10 = *(long **)(param_1 + 2);
          *(long **)(param_1 + 2) = plVar10 + 1;
        }
        pcVar6 = (char *)*plVar10;
        if (pcVar6 == (char *)0x0) {
          pcVar6 = "(NULL)";
        }
        *(char **)(puVar3 + 4) = pcVar6;
        break;
      case 0x10:
        uVar1 = *param_1;
        if (uVar1 < 0x30) {
          puVar8 = (undefined8 *)((ulong)uVar1 + *(long *)(param_1 + 4));
          *param_1 = uVar1 + 8;
        }
        else {
          puVar8 = *(undefined8 **)(param_1 + 2);
          *(undefined8 **)(param_1 + 2) = puVar8 + 1;
        }
        puVar5 = (undefined *)*puVar8;
        if (puVar5 == (undefined *)0x0) {
          puVar5 = &DAT_00114160;
        }
        *(undefined **)(puVar3 + 4) = puVar5;
      }
      lVar7 = lVar7 + 1;
      puVar3 = puVar3 + 8;
    } while (lVar7 != lVar2);
  }
  return 0;
}




// Function: printf_parse @ 0x10dd0

undefined8 printf_parse(byte *param_1,ulong *param_2,ulong *param_3)

{
  ulong *__src;
  ulong *__src_00;
  byte *pbVar1;
  int iVar2;
  ulong *puVar3;
  ulong *puVar4;
  int *piVar5;
  ulong uVar6;
  ulong *puVar7;
  byte *pbVar8;
  ulong *__dest;
  ulong uVar9;
  int iVar10;
  uint uVar11;
  uint uVar12;
  ulong uVar13;
  byte *pbVar14;
  byte bVar15;
  undefined1 uVar16;
  uint uVar17;
  long lVar18;
  ulong uVar19;
  ulong uVar20;
  ulong uVar21;
  bool bVar22;
  ulong local_78;
  ulong local_70;
  ulong local_60;
  
  __src = param_2 + 4;
  __src_00 = param_3 + 2;
  uVar19 = 7;
  *param_2 = 0;
  param_2[1] = (ulong)__src;
  uVar13 = 0;
  *param_3 = 0;
  param_3[1] = (ulong)__src_00;
  local_70 = 0;
  local_78 = 0;
  local_60 = 0;
  puVar3 = __src;
  uVar20 = 7;
LAB_00110e54:
  do {
    pbVar14 = param_1;
    if (*pbVar14 == 0) {
      puVar3[uVar13 * 0xb] = (ulong)pbVar14;
      param_2[2] = local_78;
      param_2[3] = local_70;
      return 0;
    }
    param_1 = pbVar14 + 1;
  } while (*pbVar14 != 0x25);
  uVar21 = 0xffffffffffffffff;
  puVar3 = puVar3 + uVar13 * 0xb;
  *puVar3 = (ulong)pbVar14;
  *(undefined4 *)(puVar3 + 2) = 0;
  puVar3[3] = 0;
  puVar3[4] = 0;
  puVar3[5] = 0xffffffffffffffff;
  puVar3[6] = 0;
  puVar3[7] = 0;
  puVar3[8] = 0xffffffffffffffff;
  puVar3[10] = 0xffffffffffffffff;
  uVar17 = (uint)pbVar14[1];
  pbVar8 = param_1;
  if (9 < (byte)(pbVar14[1] - 0x30)) {
LAB_00110efe:
    pbVar14 = param_1 + 1;
    if ((char)uVar17 == '\'') goto LAB_00110fe1;
    do {
      switch(uVar17 - 0x20 & 0xff) {
      case 0:
        uVar17 = (uint)puVar3[2] | 8;
        param_1 = pbVar14;
        break;
      default:
        goto switchD_00110f21_caseD_1;
      case 3:
        uVar17 = (uint)puVar3[2] | 0x10;
        param_1 = pbVar14;
        break;
      case 0xb:
        uVar17 = (uint)puVar3[2] | 4;
        param_1 = pbVar14;
        break;
      case 0xd:
        uVar17 = (uint)puVar3[2] | 2;
        param_1 = pbVar14;
        break;
      case 0x10:
        uVar17 = (uint)puVar3[2] | 0x20;
        param_1 = pbVar14;
        break;
      case 0x29:
        uVar17 = (uint)puVar3[2] | 0x40;
        param_1 = pbVar14;
      }
      while( true ) {
        *(uint *)(puVar3 + 2) = uVar17;
        uVar17 = (uint)*param_1;
        pbVar14 = param_1 + 1;
        if (*param_1 != 0x27) break;
LAB_00110fe1:
        uVar17 = (uint)puVar3[2] | 1;
        param_1 = pbVar14;
      }
    } while( true );
  }
  do {
    pbVar14 = pbVar8 + 1;
    pbVar8 = pbVar8 + 1;
  } while ((byte)(*pbVar14 - 0x30) < 10);
  uVar21 = 0xffffffffffffffff;
  if (*pbVar14 != 0x24) goto LAB_00110efe;
  uVar21 = 0;
  do {
    pbVar14 = param_1;
    lVar18 = -1;
    if (uVar21 < 0x199999999999999a) {
      lVar18 = uVar21 * 10;
    }
    uVar21 = FUN_00110b30(lVar18,(long)(char)(uVar17 - 0x30),uVar17 - 0x30);
    uVar17 = (uint)pbVar14[1];
    param_1 = pbVar14 + 1;
  } while ((byte)(pbVar14[1] - 0x30) < 10);
  uVar21 = uVar21 - 1;
  if (uVar21 < 0xfffffffffffffffe) {
    uVar17 = (uint)pbVar14[2];
    param_1 = pbVar14 + 2;
    goto LAB_00110efe;
  }
  goto switchD_00110f94_caseD_26;
switchD_00110f21_caseD_1:
  if ((char)uVar17 == '*') {
    puVar3[3] = (ulong)param_1;
    puVar3[4] = (ulong)pbVar14;
    bVar15 = param_1[1];
    uVar13 = 1;
    if (local_78 != 0) {
      uVar13 = local_78;
    }
    pbVar8 = pbVar14;
    if ((byte)(bVar15 - 0x30) < 10) {
      do {
        pbVar1 = pbVar8 + 1;
        pbVar8 = pbVar8 + 1;
      } while ((byte)(*pbVar1 - 0x30) < 10);
      if (*pbVar1 != 0x24) goto LAB_0011116d;
      uVar6 = 0;
      do {
        pbVar8 = pbVar14;
        lVar18 = -1;
        if (uVar6 < 0x199999999999999a) {
          lVar18 = uVar6 * 10;
        }
        uVar6 = FUN_00110b30(lVar18,(long)(char)(bVar15 - 0x30));
        bVar15 = pbVar8[1];
        pbVar14 = pbVar8 + 1;
      } while ((byte)(bVar15 - 0x30) < 10);
      uVar6 = uVar6 - 1;
      if (0xfffffffffffffffd < uVar6) goto switchD_00110f94_caseD_26;
      puVar3[5] = uVar6;
      pbVar14 = pbVar8 + 2;
    }
    else {
LAB_0011116d:
      puVar3[5] = local_60;
      bVar22 = local_60 == 0xffffffffffffffff;
      uVar6 = local_60;
      local_60 = local_60 + 1;
      if (bVar22) goto switchD_00110f94_caseD_26;
    }
    if (uVar6 < uVar19) {
      __dest = (ulong *)param_3[1];
    }
    else {
      uVar19 = uVar19 * 2;
      if (uVar19 <= uVar6) {
        uVar19 = FUN_00110b30(uVar6,1);
      }
      puVar7 = (ulong *)param_3[1];
      if (0x7ffffffffffffff < uVar19) goto LAB_001116ed;
      if (__src_00 == puVar7) {
        __dest = malloc(uVar19 << 5);
      }
      else {
        __dest = realloc(puVar7,uVar19 << 5);
      }
      puVar7 = (ulong *)param_3[1];
      if (__dest == (ulong *)0x0) goto LAB_001116ed;
      if (__src_00 == puVar7) {
        __dest = memcpy(__dest,__src_00,*param_3 << 5);
      }
      param_3[1] = (ulong)__dest;
    }
    uVar9 = *param_3;
    puVar7 = __dest + uVar9 * 4;
    if (uVar9 <= uVar6) {
      do {
        puVar4 = puVar7;
        uVar9 = uVar9 + 1;
        *(undefined4 *)puVar4 = 0;
        puVar7 = puVar4 + 4;
      } while (uVar9 <= uVar6);
      *param_3 = uVar9;
      *(undefined4 *)puVar4 = 0;
    }
    iVar10 = (int)__dest[uVar6 * 4];
    local_78 = uVar13;
    if (iVar10 != 0) {
      if (iVar10 != 5) goto LAB_001113e4;
      goto LAB_0011145c;
    }
    *(int *)(__dest + uVar6 * 4) = 5;
    bVar15 = *pbVar14;
    uVar17 = (uint)bVar15;
    uVar13 = local_60;
  }
  else {
    pbVar14 = param_1;
    if ((byte)((char)uVar17 - 0x30U) < 10) {
      puVar3[3] = (ulong)param_1;
      uVar13 = local_78;
      if ((byte)(*param_1 - 0x30) < 10) {
        do {
          pbVar8 = pbVar14 + 1;
          pbVar14 = pbVar14 + 1;
        } while ((byte)(*pbVar8 - 0x30) < 10);
        uVar13 = (long)pbVar14 - (long)param_1;
        if ((ulong)((long)pbVar14 - (long)param_1) <= local_78) {
          uVar13 = local_78;
        }
      }
      local_78 = uVar13;
      puVar3[4] = (ulong)pbVar14;
LAB_0011145c:
      uVar17 = (uint)*pbVar14;
    }
    bVar15 = (byte)uVar17;
    uVar13 = local_60;
  }
  param_1 = pbVar14;
  uVar6 = local_70;
  local_60 = uVar13;
  if (bVar15 != 0x2e) goto LAB_00110f47;
  if (pbVar14[1] != 0x2a) {
    puVar3[6] = (ulong)pbVar14;
    param_1 = pbVar14 + 1;
    if ((byte)(pbVar14[1] - 0x30) < 10) {
      do {
        pbVar8 = param_1 + 1;
        param_1 = param_1 + 1;
      } while ((byte)(*pbVar8 - 0x30) < 10);
      uVar6 = (long)param_1 - (long)pbVar14;
    }
    else {
      uVar6 = 1;
    }
    puVar3[7] = (ulong)param_1;
    uVar17 = (uint)*param_1;
    if (uVar6 <= local_70) {
      uVar6 = local_70;
    }
    goto LAB_00110f47;
  }
  param_1 = pbVar14 + 2;
  puVar3[6] = (ulong)pbVar14;
  puVar3[7] = (ulong)param_1;
  uVar6 = 2;
  if (1 < local_70) {
    uVar6 = local_70;
  }
  bVar15 = pbVar14[2];
  pbVar14 = param_1;
  if ((byte)(bVar15 - 0x30) < 10) {
    do {
      pbVar8 = pbVar14 + 1;
      pbVar14 = pbVar14 + 1;
    } while ((byte)(*pbVar8 - 0x30) < 10);
    if (*pbVar8 != 0x24) goto LAB_001112b5;
    uVar9 = 0;
    do {
      pbVar14 = param_1;
      lVar18 = -1;
      if (uVar9 < 0x199999999999999a) {
        lVar18 = uVar9 * 10;
      }
      uVar9 = FUN_00110b30(lVar18,(long)(char)(bVar15 - 0x30));
      bVar15 = pbVar14[1];
      param_1 = pbVar14 + 1;
    } while ((byte)(bVar15 - 0x30) < 10);
    uVar9 = uVar9 - 1;
    if (0xfffffffffffffffd < uVar9) goto switchD_00110f94_caseD_26;
    puVar3[8] = uVar9;
    param_1 = pbVar14 + 2;
  }
  else {
LAB_001112b5:
    uVar9 = puVar3[8];
    if (puVar3[8] == 0xffffffffffffffff) {
      puVar3[8] = uVar13;
      local_60 = uVar13 + 1;
      uVar9 = uVar13;
      if (uVar13 == 0xffffffffffffffff) goto switchD_00110f94_caseD_26;
    }
  }
  if (uVar9 < uVar19) {
    __dest = (ulong *)param_3[1];
  }
  else {
    uVar19 = uVar19 * 2;
    if (uVar19 <= uVar9) {
      uVar19 = FUN_00110b30(uVar9,1);
    }
    puVar7 = (ulong *)param_3[1];
    if (0x7ffffffffffffff < uVar19) goto LAB_001116ed;
    if (__src_00 == puVar7) {
      __dest = malloc(uVar19 << 5);
    }
    else {
      __dest = realloc(puVar7,uVar19 << 5);
    }
    puVar7 = (ulong *)param_3[1];
    if (__dest == (ulong *)0x0) goto LAB_001116ed;
    if (__src_00 == puVar7) {
      __dest = memcpy(__dest,__src_00,*param_3 << 5);
    }
    param_3[1] = (ulong)__dest;
  }
  uVar13 = *param_3;
  puVar7 = __dest + uVar13 * 4;
  if (uVar13 <= uVar9) {
    do {
      puVar4 = puVar7;
      uVar13 = uVar13 + 1;
      *(undefined4 *)puVar4 = 0;
      puVar7 = puVar4 + 4;
    } while (uVar13 <= uVar9);
    *param_3 = uVar13;
    *(undefined4 *)puVar4 = 0;
  }
  iVar10 = (int)__dest[uVar9 * 4];
  if (iVar10 == 0) {
    *(int *)(__dest + uVar9 * 4) = 5;
    uVar17 = (uint)*param_1;
LAB_00110f47:
    local_70 = uVar6;
    uVar12 = 0;
    param_1 = param_1 + 1;
    uVar11 = 0;
    if ((char)uVar17 == 'h') goto LAB_00110fb0;
    do {
      uVar16 = (undefined1)uVar17;
      switch(uVar16) {
      case 0x4c:
        uVar12 = uVar11 | 4;
        break;
      default:
        goto code_r0x00110f83;
      case 0x5a:
      case 0x6a:
      case 0x6c:
      case 0x74:
      case 0x7a:
        uVar12 = uVar11 + 8;
      }
      while( true ) {
        bVar15 = *param_1;
        uVar17 = (uint)bVar15;
        param_1 = param_1 + 1;
        uVar11 = uVar12;
        if (bVar15 != 0x68) break;
LAB_00110fb0:
        uVar12 = uVar12 | 1 << ((byte)uVar12 & 1);
      }
    } while( true );
  }
  if (iVar10 == 5) {
    uVar17 = (uint)*param_1;
    goto LAB_00110f47;
  }
  goto LAB_001113e4;
code_r0x00110f83:
  switch(uVar16) {
  case 0x25:
    goto switchD_00110f94_caseD_25;
  default:
    goto switchD_00110f94_caseD_26;
  case 0x41:
  case 0x45:
  case 0x46:
  case 0x47:
  case 0x61:
  case 0x65:
  case 0x66:
  case 0x67:
    iVar10 = 0xc;
    if ((int)uVar11 < 0x10) {
      iVar10 = ((uVar11 & 4) != 0) + 0xb;
    }
    break;
  case 0x43:
    iVar10 = 0xe;
    uVar16 = 99;
    break;
  case 0x53:
    iVar10 = 0x10;
    uVar16 = 0x73;
    break;
  case 0x58:
  case 0x6f:
  case 0x75:
  case 0x78:
    iVar10 = 10;
    if ((((int)uVar11 < 0x10) && ((uVar11 & 4) == 0)) &&
       ((iVar10 = 8, (int)uVar11 < 8 && (iVar10 = 2, (uVar11 & 2) == 0)))) {
      iVar10 = (-(uint)((uVar11 & 1) == 0) & 2) + 4;
    }
    break;
  case 99:
    iVar10 = (7 < (int)uVar11) + 0xd;
    break;
  case 100:
  case 0x69:
    iVar10 = 9;
    if ((((int)uVar11 < 0x10) && ((uVar11 & 4) == 0)) &&
       ((iVar10 = 7, (int)uVar11 < 8 && (iVar10 = 1, (uVar11 & 2) == 0)))) {
      iVar10 = (-(uint)((uVar11 & 1) == 0) & 2) + 3;
    }
    break;
  case 0x6e:
    iVar10 = 0x16;
    if (((((int)uVar11 < 0x10) && ((uVar11 & 4) == 0)) && (iVar10 = 0x15, (int)uVar11 < 8)) &&
       (iVar10 = 0x12, (uVar11 & 2) == 0)) {
      iVar10 = 0x14 - (uVar11 & 1);
    }
    break;
  case 0x70:
    iVar10 = 0x11;
    break;
  case 0x73:
    iVar10 = (7 < (int)uVar11) + 0xf;
  }
  if (uVar21 == 0xffffffffffffffff) {
    puVar3[10] = local_60;
    uVar13 = local_60 + 1;
    uVar21 = local_60;
    if (local_60 == 0xffffffffffffffff) {
switchD_00110f94_caseD_26:
      __dest = (ulong *)param_3[1];
      goto LAB_001113e4;
    }
  }
  else {
    puVar3[10] = uVar21;
    uVar13 = local_60;
  }
  local_60 = uVar13;
  if (uVar21 < uVar19) {
    __dest = (ulong *)param_3[1];
  }
  else {
    uVar19 = uVar19 * 2;
    if (uVar19 <= uVar21) {
      uVar19 = FUN_00110b30(uVar21,1);
    }
    puVar7 = (ulong *)param_3[1];
    if (0x7ffffffffffffff < uVar19) goto LAB_001116ed;
    if (__src_00 == puVar7) {
      __dest = malloc(uVar19 << 5);
    }
    else {
      __dest = realloc(puVar7,uVar19 << 5);
    }
    puVar7 = (ulong *)param_3[1];
    if (__dest == (ulong *)0x0) goto LAB_001116ed;
    if (__src_00 == puVar7) {
      __dest = memcpy(__dest,__src_00,*param_3 << 5);
    }
    param_3[1] = (ulong)__dest;
  }
  uVar13 = *param_3;
  puVar7 = __dest + uVar13 * 4;
  if (uVar13 <= uVar21) {
    do {
      puVar4 = puVar7;
      uVar13 = uVar13 + 1;
      *(undefined4 *)puVar4 = 0;
      puVar7 = puVar4 + 4;
    } while (uVar13 <= uVar21);
    *param_3 = uVar13;
    *(undefined4 *)puVar4 = 0;
  }
  iVar2 = (int)__dest[uVar21 * 4];
  if (iVar2 == 0) {
    *(int *)(__dest + uVar21 * 4) = iVar10;
  }
  else if (iVar2 != iVar10) {
LAB_001113e4:
    if (__src_00 != __dest) {
      free(__dest);
    }
    if (__src != (ulong *)param_2[1]) {
      free((ulong *)param_2[1]);
    }
    piVar5 = __errno_location();
    *piVar5 = 0x16;
    return 0xffffffff;
  }
switchD_00110f94_caseD_25:
  *(undefined1 *)(puVar3 + 9) = uVar16;
  uVar13 = *param_2;
  puVar3[1] = (ulong)param_1;
  uVar13 = uVar13 + 1;
  *param_2 = uVar13;
  if (uVar13 < uVar20) {
    puVar3 = (ulong *)param_2[1];
    goto LAB_00110e54;
  }
  if ((-1 < (long)uVar20) && (uVar20 * 2 < 0x2e8ba2e8ba2e8bb)) {
    if (__src == (ulong *)param_2[1]) {
      puVar3 = malloc(uVar20 * 0xb0);
    }
    else {
      puVar3 = realloc((ulong *)param_2[1],uVar20 * 0xb0);
    }
    if (puVar3 != (ulong *)0x0) {
      if (__src == (ulong *)param_2[1]) {
        puVar3 = memcpy(puVar3,__src,*param_2 * 0x58);
      }
      param_2[1] = (ulong)puVar3;
      uVar13 = *param_2;
      uVar20 = uVar20 * 2;
      goto LAB_00110e54;
    }
  }
  puVar7 = (ulong *)param_3[1];
LAB_001116ed:
  if (__src_00 != puVar7) {
    free(puVar7);
  }
  if (__src != (ulong *)param_2[1]) {
    free((ulong *)param_2[1]);
  }
  piVar5 = __errno_location();
  *piVar5 = 0xc;
  return 0xffffffff;
}




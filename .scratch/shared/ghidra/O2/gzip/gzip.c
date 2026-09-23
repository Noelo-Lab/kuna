// Function: main @ 0x39a0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void main(undefined4 param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  int iVar2;
  uint uVar3;
  int iVar4;
  int iVar5;
  char *__s;
  size_t sVar6;
  long lVar7;
  undefined4 *puVar8;
  undefined4 *puVar9;
  undefined8 *__ptr;
  int *piVar10;
  long lVar11;
  long in_FS_OFFSET;
  byte bVar12;
  undefined1 auVar13 [16];
  undefined8 uStack_f8;
  int local_f0 [2];
  int local_e8;
  int local_e4;
  undefined8 *local_e0;
  _union_1457 local_d8;
  undefined4 local_d0 [32];
  undefined4 local_50;
  undefined8 local_40;
  
  bVar12 = 0;
  uStack_f8 = CONCAT44(param_1,(undefined4)uStack_f8);
  local_40 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  __s = (char *)thunk_FUN_0010dfd0(*param_2);
  DAT_0011a850 = __s;
  sVar6 = strlen(__s);
  if (4 < sVar6) {
    iVar2 = strcmp(__s + (sVar6 - 4),".exe");
    if (iVar2 == 0) {
      __s[sVar6 - 4] = '\0';
    }
  }
  local_e0 = param_2;
  DAT_001dc9d0 = FUN_0010d2b0(&local_e8,&local_e0,&DAT_00112e43);
  __ptr = (undefined8 *)0x0;
  if (DAT_001dc9d0 != 0) {
    __ptr = local_e0;
  }
  DAT_001dc9c8 = &DAT_00112d5c;
  DAT_001dc9c0 = strlen(".gz");
  local_e4 = -1;
  if (__ptr != (undefined8 *)0x0) {
    if (((char *)__ptr[optind] == (char *)0x0) ||
       (iVar2 = strcmp((char *)__ptr[optind],"--"), iVar2 != 0)) {
      iVar4 = getopt_long(local_e8,__ptr,"ab:cdfhH?klLmMnNqrS:tvVZ123456789",&PTR_s_ascii_001177c0,
                          &local_e4);
      iVar2 = iVar4 + 0x83;
      if (iVar4 < 0) goto LAB_00103bb0;
      uVar3 = iVar4 + 0x52;
      if (0xd3 < uVar3) goto LAB_00103b91;
LAB_00103b46:
                    /* WARNING: Could not recover jumptable at 0x00103b54. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (*(code *)(&DAT_00112f80 + *(int *)(&DAT_00112f80 + (ulong)uVar3 * 4)))();
      return;
    }
    if (__ptr != (undefined8 *)0x0) {
      iVar2 = 0xb0;
      goto LAB_00103aa7;
    }
  }
  while( true ) {
    iVar2 = getopt_long(uStack_f8._4_4_,param_2,"ab:cdfhH?klLmMnNqrS:tvVZ123456789",
                        &PTR_s_ascii_001177c0);
    if (iVar2 < 0) break;
    uVar3 = iVar2 - 0x31;
    __ptr = (undefined8 *)0x0;
    if (uVar3 < 0xd4) goto LAB_00103b46;
LAB_00103b91:
    if ((0x82 < iVar2) && (iVar2 != 0xc2)) {
LAB_00103aa7:
      uStack_f8 = CONCAT44(iVar2,(undefined4)uStack_f8);
      FUN_0010f6b0(stderr,&DAT_001137cc,DAT_0011a850);
      if (local_e4 < 0) {
        FUN_0010f6b0(stderr,"-%c: ",uStack_f8._4_4_ + -0x83);
      }
      else {
        FUN_0010f6b0(stderr,"--%s: ",(&PTR_s_ascii_001177c0)[(long)local_e4 * 4]);
      }
      FUN_0010f6b0(stderr,"option not valid in GZIP environment variable\n");
    }
    FUN_001069d0();
LAB_00103bb0:
    if (optind != local_e8) {
      FUN_0010f6b0(stderr,"%s: %s: non-option in GZIP environment variable\n",DAT_0011a850,
                   __ptr[optind]);
      auVar13 = FUN_001069d0();
      uVar1 = uStack_f8;
      uStack_f8 = auVar13._0_8_;
      (*(code *)PTR___libc_start_main_00117fc0)
                (main,uVar1,local_f0,0,0,auVar13._8_8_,&uStack_f8);
      do {
                    /* WARNING: Do nothing block with infinite loop */
      } while( true );
    }
    if ((optind != 1) && (DAT_0011a85c == 0)) {
      FUN_0010f6b0(stderr,
                   "%s: warning: GZIP environment variable is deprecated; use an alias or script\n",
                   DAT_0011a850);
    }
    free(__ptr);
    local_e4 = -1;
    optind = 1;
  }
  if (DAT_00118028 < 0) {
    DAT_00118028 = DAT_001dc9f8;
  }
  if (DAT_0011802c < 0) {
    DAT_0011802c = DAT_001dc9f8;
  }
  local_f0[0] = optind;
  if (DAT_001dc9fc == 0) {
LAB_0010411e:
    if (0x1d < DAT_001dc9c0 - 1) {
LAB_00104166:
      FUN_0010f6b0(stderr,"%s: invalid suffix \'%s\'\n",DAT_0011a850,DAT_001dc9c8);
                    /* WARNING: Subroutine does not return */
      FUN_00106960(1);
    }
    DAT_001dc920 = 0;
    if (DAT_0011a85c == 0) goto LAB_00103ff6;
  }
  else {
    if (DAT_0011a85c == 0) {
      FUN_0010f6b0(stderr,"%s: option --ascii ignored on this system\n",DAT_0011a850);
      goto LAB_0010411e;
    }
    if (0x1d < DAT_001dc9c0 - 1) goto LAB_00104166;
  }
  DAT_001dc920 = 0xd;
LAB_00103ff6:
  piVar10 = &DAT_00113350;
  sigemptyset((sigset_t *)&DAT_001dc940);
  do {
    iVar2 = *piVar10;
    sigaction(iVar2,(sigaction *)0x0,(sigaction *)&local_d8);
    if (local_d8.sa_handler != (__sighandler_t)0x1) {
      sigaddset((sigset_t *)&DAT_001dc940,iVar2);
    }
    piVar10 = piVar10 + 1;
  } while (piVar10 != (int *)&DAT_00113368);
  lVar11 = 0;
  puVar8 = &DAT_001dc940;
  puVar9 = local_d0;
  for (lVar7 = 0x20; lVar7 != 0; lVar7 = lVar7 + -1) {
    *puVar9 = *puVar8;
    puVar8 = puVar8 + (ulong)bVar12 * -2 + 1;
    puVar9 = puVar9 + (ulong)bVar12 * -2 + 1;
  }
  local_50 = 0;
  local_d8.sa_handler = FUN_00105600;
  do {
    iVar2 = (&DAT_00113350)[lVar11];
    iVar4 = sigismember((sigset_t *)&DAT_001dc940,iVar2);
    if (iVar4 != 0) {
      if (lVar11 == 0) {
        DAT_001dc9e0 = 1;
      }
      sigaction(iVar2,(sigaction *)&local_d8,(sigaction *)0x0);
    }
    lVar11 = lVar11 + 1;
  } while (lVar11 != 6);
  iVar2 = uStack_f8._4_4_ - local_f0[0];
  if (uStack_f8._4_4_ == local_f0[0]) {
    FUN_00106a80();
  }
  else {
    while( true ) {
      lVar7 = (long)optind;
      if (uStack_f8._4_4_ <= optind) break;
      optind = optind + 1;
      iVar5 = strcmp((char *)param_2[lVar7],"-");
      iVar4 = DAT_0011a860;
      if (iVar5 == 0) {
        FUN_00106a80();
        DAT_0011a860 = iVar4;
      }
      else {
        FUN_00106e90();
      }
    }
  }
  if ((DAT_001dc510 != '\0') && (iVar4 = close(0), iVar4 != 0)) {
    _DAT_0011a420 = 0x69647473;
    _DAT_0011a424 = 0x6e;
                    /* WARNING: Subroutine does not return */
    FUN_0010d4e0();
  }
  if (DAT_001dc9e8 != 0) {
    if ((DAT_0011a85c == 0) && (1 < iVar2)) {
      FUN_001065d0(0xffffffff);
    }
    iVar2 = FUN_0010e3e0(stdout);
    if (iVar2 != 0) goto LAB_00104221;
  }
  if (DAT_0011a860 != 0) {
    if (DAT_001dca00 == '\0') goto LAB_00104226;
    iVar2 = fdatasync(1);
    if (iVar2 == 0) goto LAB_00104226;
    piVar10 = __errno_location();
    if (*piVar10 == 0x16) goto LAB_00104226;
    do {
      piVar10 = __errno_location();
      if (*piVar10 == 9) break;
LAB_00104221:
      FUN_0010d550();
LAB_00104226:
      iVar2 = close(1);
    } while (iVar2 != 0);
  }
                    /* WARNING: Subroutine does not return */
  FUN_00106960(DAT_0011a84c);
}




// Function: bi_init @ 0x4380

void bi_init(int param_1)

{
  DAT_00119058 = 0;
  DAT_0011905c = 0;
  if (param_1 != -1) {
    DAT_00119050 = FUN_0010df80;
  }
  return;
}




// Function: send_bits @ 0x43b0

void send_bits(uint param_1,int param_2)

{
  int iVar1;
  ushort uVar2;
  ulong uVar3;
  uint uVar4;
  ulong uVar5;
  
  uVar2 = (ushort)(param_1 << ((byte)DAT_00119058 & 0x1f)) | DAT_0011905c;
  iVar1 = DAT_00119058;
  if (0x10 - param_2 < DAT_00119058) {
    uVar3 = (ulong)DAT_0011a004;
    uVar4 = DAT_0011a004 + 1;
    uVar5 = (ulong)uVar4;
    if (DAT_0011a004 < 0x3fffe) {
      uVar3 = (ulong)DAT_0011a004;
      DAT_0011a004 = DAT_0011a004 + 2;
      (&DAT_0015b000)[uVar3] = (char)uVar2;
      (&DAT_0015b000)[uVar5] = (char)(uVar2 >> 8);
    }
    else {
      DAT_0011905c = uVar2;
      DAT_0011a004 = uVar4;
      (&DAT_0015b000)[uVar3] = (char)uVar2;
      if (uVar4 == 0x40000) {
        FUN_0010d600();
        uVar5 = (ulong)DAT_0011a004;
        uVar2 = DAT_0011905c;
      }
      uVar4 = (int)uVar5 + 1;
      DAT_0011a004 = uVar4;
      (&DAT_0015b000)[uVar5] = (char)(uVar2 >> 8);
      if (uVar4 == 0x40000) {
        FUN_0010d600();
      }
    }
    iVar1 = DAT_00119058 + -0x10;
    uVar2 = (ushort)((int)(param_1 & 0xffff) >> (0x10U - (char)DAT_00119058 & 0x1f));
  }
  DAT_0011905c = uVar2;
  DAT_00119058 = param_2 + iVar1;
  return;
}




// Function: bi_reverse @ 0x44a0

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




// Function: bi_windup @ 0x44d0

void bi_windup(void)

{
  undefined2 uVar1;
  ulong uVar2;
  undefined1 uVar3;
  uint uVar4;
  int iVar5;
  ulong uVar6;
  
  uVar1 = DAT_0011905c;
  uVar3 = (undefined1)DAT_0011905c;
  if (DAT_00119058 < 9) {
    if (DAT_00119058 < 1) {
      DAT_00119058 = 0;
      DAT_0011905c = 0;
      return;
    }
    uVar2 = (ulong)DAT_0011a004;
    iVar5 = DAT_0011a004 + 1;
    DAT_0011a004 = iVar5;
    (&DAT_0015b000)[uVar2] = uVar3;
  }
  else {
    uVar2 = (ulong)DAT_0011a004;
    uVar4 = DAT_0011a004 + 1;
    uVar6 = (ulong)uVar4;
    if (DAT_0011a004 < 0x3fffe) {
      uVar2 = (ulong)DAT_0011a004;
      DAT_0011a004 = DAT_0011a004 + 2;
      (&DAT_0015b000)[uVar2] = uVar3;
      (&DAT_0015b000)[uVar6] = (char)((ushort)DAT_0011905c >> 8);
      DAT_00119058 = 0;
      DAT_0011905c = 0;
      return;
    }
    DAT_0011a004 = uVar4;
    (&DAT_0015b000)[uVar2] = uVar3;
    if (uVar4 == 0x40000) {
      FUN_0010d600();
      uVar6 = (ulong)DAT_0011a004;
      uVar1 = DAT_0011905c;
    }
    DAT_0011a004 = (int)uVar6 + 1;
    (&DAT_0015b000)[uVar6] = (char)((ushort)uVar1 >> 8);
    iVar5 = DAT_0011a004;
  }
  if (iVar5 == 0x40000) {
    FUN_0010d600();
  }
  DAT_00119058 = 0;
  DAT_0011905c = 0;
  return;
}




// Function: copy_block @ 0x45c0

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
  
  FUN_001044d0();
  if (param_3 != 0) {
    uVar3 = (ulong)DAT_0011a004;
    bVar2 = (byte)(param_2 >> 8);
    uVar4 = DAT_0011a004 + 1;
    uVar6 = (ulong)uVar4;
    bVar8 = (byte)param_2;
    if (DAT_0011a004 < 0x3fffe) {
      uVar3 = (ulong)(DAT_0011a004 + 2);
      (&DAT_0015b000)[DAT_0011a004] = bVar8;
      (&DAT_0015b000)[uVar6] = bVar2;
    }
    else {
      DAT_0011a004 = uVar4;
      (&DAT_0015b000)[uVar3] = bVar8;
      if (uVar4 == 0x40000) {
        FUN_0010d600();
        uVar6 = (ulong)DAT_0011a004;
      }
      uVar4 = (int)uVar6 + 1;
      uVar3 = (ulong)uVar4;
      DAT_0011a004 = uVar4;
      (&DAT_0015b000)[uVar6] = bVar2;
      if (uVar4 == 0x40000) {
        FUN_0010d600();
        uVar3 = (ulong)DAT_0011a004;
      }
    }
    uVar4 = (uint)uVar3;
    uVar5 = uVar4 + 1;
    uVar6 = (ulong)uVar5;
    if (uVar4 < 0x3fffe) {
      DAT_0011a004 = uVar4 + 2;
      (&DAT_0015b000)[uVar3] = ~bVar8;
      (&DAT_0015b000)[uVar6] = ~bVar2;
    }
    else {
      DAT_0011a004 = uVar5;
      (&DAT_0015b000)[uVar3] = ~bVar8;
      if (uVar5 == 0x40000) {
        FUN_0010d600();
        uVar6 = (ulong)DAT_0011a004;
      }
      uVar4 = (int)uVar6 + 1;
      DAT_0011a004 = uVar4;
      (&DAT_0015b000)[uVar6] = ~bVar2;
      if (uVar4 == 0x40000) {
        FUN_0010d600();
      }
    }
  }
  puVar7 = param_1 + param_2;
  if (param_2 != 0) {
    do {
      while( true ) {
        uVar3 = (ulong)DAT_0011a004;
        uVar1 = *param_1;
        param_1 = param_1 + 1;
        uVar4 = DAT_0011a004 + 1;
        DAT_0011a004 = uVar4;
        (&DAT_0015b000)[uVar3] = uVar1;
        if (uVar4 != 0x40000) break;
        FUN_0010d600();
        if (param_1 == puVar7) {
          return;
        }
      }
    } while (param_1 != puVar7);
  }
  return;
}




// Function: longest_match @ 0x4740

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
  
  uVar4 = (ulong)DAT_0011906c;
  uVar12 = DAT_0011906c;
  if (DAT_0011906c < 0x7efa) {
    uVar12 = 0x7efa;
  }
  lVar10 = (long)(int)DAT_00119070;
  pcVar1 = &DAT_0013b000 + uVar4;
  cVar8 = *(char *)(uVar4 + 0x13afff + lVar10);
  cVar14 = pcVar1[lVar10];
  uVar7 = DAT_00119064;
  if (DAT_00119060 <= DAT_00119070) {
    uVar7 = DAT_00119064 >> 2;
  }
  bVar3 = false;
  uVar13 = DAT_00119070;
  do {
    uVar5 = (ulong)param_1;
    if (((((&DAT_0013b000 + uVar5)[lVar10] == cVar14) &&
         (*(char *)(uVar5 + 0x13afff + lVar10) == cVar8)) && ((&DAT_0013b000)[uVar5] == *pcVar1)) &&
       ((&DAT_0013b001)[uVar5] == (&DAT_0013b001)[uVar4])) {
      pcVar11 = &DAT_0013b002 + uVar4;
      pcVar6 = &DAT_0013b002 + uVar5;
      while( true ) {
        if (pcVar11[1] != pcVar6[1]) {
          pcVar11 = pcVar11 + 1;
          goto LAB_00104890;
        }
        if (pcVar11[2] != pcVar6[2]) break;
        if (pcVar11[3] != pcVar6[3]) {
          pcVar11 = pcVar11 + 3;
          goto LAB_00104890;
        }
        if (pcVar11[4] != pcVar6[4]) {
          pcVar11 = pcVar11 + 4;
          goto LAB_00104890;
        }
        if (pcVar11[5] != pcVar6[5]) {
          pcVar11 = pcVar11 + 5;
          goto LAB_00104890;
        }
        if (pcVar11[6] != pcVar6[6]) {
          pcVar11 = pcVar11 + 6;
          goto LAB_00104890;
        }
        if (pcVar11[7] != pcVar6[7]) {
          pcVar11 = pcVar11 + 7;
          goto LAB_00104890;
        }
        pcVar11 = pcVar11 + 8;
        pcVar6 = pcVar6 + 8;
        if ((*pcVar11 != *pcVar6) || ((char *)(uVar4 + 0x13b102) <= pcVar11)) goto LAB_00104890;
      }
      pcVar11 = pcVar11 + 2;
LAB_00104890:
      uVar9 = 0x102 - ((int)(char *)(uVar4 + 0x13b102) - (int)pcVar11);
      if ((int)uVar9 <= (int)uVar13) goto LAB_001047b8;
      if (DAT_00119080 <= (int)uVar9) {
        DAT_00119068 = param_1;
        return uVar9;
      }
      uVar2 = (&DAT_0011a880)[param_1 & 0x7fff];
      cVar8 = *(char *)(uVar4 + 0x13afff + (long)(int)uVar9);
      bVar3 = true;
      cVar14 = pcVar1[(int)uVar9];
      uVar13 = uVar9;
      local_34 = param_1;
    }
    else {
LAB_001047b8:
      uVar2 = (&DAT_0011a880)[param_1 & 0x7fff];
    }
    param_1 = (uint)uVar2;
    if ((param_1 <= uVar12 - 0x7efa) || (uVar7 = uVar7 - 1, uVar7 == 0)) {
      if (!bVar3) {
        return uVar13;
      }
      DAT_00119068 = local_34;
      return uVar13;
    }
    lVar10 = (long)(int)uVar13;
  } while( true );
}




// Function: rsync_roll @ 0x4960

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
    pbVar5 = &DAT_0013b000 + param_1;
    pbVar1 = pbVar5 + param_2;
    bVar3 = false;
    uVar6 = DAT_00119090;
    do {
      if (pbVar1 == pbVar5) {
        if (!bVar3) {
          return;
        }
        DAT_00119090 = uVar6;
        return;
      }
      bVar2 = *pbVar5;
      pbVar5 = pbVar5 + 1;
      uVar6 = uVar6 + bVar2;
      bVar3 = true;
    } while (&DAT_0013b001 + (ulong)(0xfff - param_1) + (ulong)param_1 != pbVar5);
    uVar4 = 0x1000;
    DAT_00119090 = uVar6;
  }
  if (param_1 + param_2 <= (uint)uVar4) {
    return;
  }
  bVar3 = false;
  uVar6 = DAT_00119088;
  do {
    DAT_00119090 = DAT_00119090 +
                   ((ulong)(byte)(&DAT_0013b000)[uVar4] -
                   (ulong)(byte)(&DAT_0013b000)[(int)uVar4 - 0x1000]);
    if ((uVar6 == 0xffffffff) && ((DAT_00119090 & 0xfff) == 0)) {
      bVar3 = true;
      uVar6 = uVar4;
    }
    uVar4 = uVar4 + 1;
  } while ((uint)uVar4 < param_1 + param_2);
  if (bVar3) {
    DAT_00119088 = uVar6;
  }
  return;
}




// Function: fill_window @ 0x4a60

void fill_window(void)

{
  short *psVar1;
  short sVar2;
  short *psVar3;
  ulong uVar4;
  int iVar5;
  int iVar6;
  uint uVar7;
  
  uVar4 = (ulong)DAT_0011909c;
  iVar5 = DAT_0011909c + DAT_0011906c;
  iVar6 = 0x10000 - iVar5;
  if (iVar6 == -1) {
    iVar6 = -2;
  }
  else if (0xfef9 < DAT_0011906c) {
    uVar7 = DAT_0011906c - 0x8000;
    memcpy(&DAT_0013b000,&DAT_00143000,0x8000);
    DAT_00119068 = DAT_00119068 + -0x8000;
    if (DAT_00119088 != 0xffffffff) {
      DAT_00119088 = DAT_00119088 + -0x8000;
    }
    DAT_00119078 = DAT_00119078 + -0x8000;
    psVar1 = &DAT_0012a880;
    DAT_0011906c = uVar7;
    do {
      sVar2 = *psVar1 + -0x8000;
      if (-1 < *psVar1) {
        sVar2 = 0;
      }
      psVar3 = psVar1 + 1;
      *psVar1 = sVar2;
      psVar1 = psVar3;
    } while (psVar3 != (short *)&DAT_0013a880);
    psVar1 = &DAT_0011a880;
    do {
      sVar2 = *psVar1 + -0x8000;
      if (-1 < *psVar1) {
        sVar2 = 0;
      }
      psVar3 = psVar1 + 1;
      *psVar1 = sVar2;
      psVar1 = psVar3;
    } while (psVar3 != &DAT_0012a880);
    iVar6 = 0x18000 - iVar5;
  }
  if (DAT_001190a0 == 0) {
    iVar5 = (*DAT_00119050)(&DAT_0013b000 + DAT_0011906c + uVar4,iVar6);
    if (0xfffffffd < iVar5 - 1U) {
      DAT_001190a0 = 1;
      *(undefined2 *)(&DAT_0013b000 + (ulong)DAT_0011906c + (ulong)DAT_0011909c) = 0;
      return;
    }
    DAT_0011909c = DAT_0011909c + iVar5;
  }
  return;
}




// Function: deflate @ 0x4bc0

void deflate(int param_1)

{
  short *psVar1;
  int iVar2;
  ushort uVar3;
  short sVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  ulong uVar8;
  int iVar9;
  uint uVar10;
  long lVar11;
  undefined1 *puVar12;
  uint uVar13;
  int iVar14;
  int iVar15;
  
  if (8 < param_1 - 1U) {
                    /* WARNING: Subroutine does not return */
    FUN_0010d3f0("bad pack level");
  }
  memset(&DAT_0012a880,0,0x10000);
  lVar11 = (long)param_1;
  DAT_00119088 = 0xffffffff;
  DAT_00119090 = 0;
  DAT_0011906c = 0;
  DAT_00119098 = (uint)*(ushort *)(&DAT_00112022 + lVar11 * 8);
  DAT_00119078 = 0;
  DAT_00119060 = (uint)*(ushort *)(&DAT_00112020 + lVar11 * 8);
  DAT_00119080 = (uint)*(ushort *)(&DAT_00112024 + lVar11 * 8);
  DAT_00119064 = (uint)*(ushort *)(&DAT_00112026 + lVar11 * 8);
  DAT_0011909c = (*DAT_00119050)(&DAT_0013b000,0x10000);
  if (DAT_0011909c - 1 < 0xfffffffe) {
    DAT_001190a0 = 0;
    if (DAT_0011909c < 0x106) {
      do {
        FUN_00104a60();
        if (0x105 < DAT_0011909c) break;
      } while (DAT_001190a0 == 0);
    }
    DAT_001190a4 = (uint)DAT_0013b000 << 5 ^ (uint)DAT_0013b001;
    if (param_1 < 4) {
LAB_00105136:
      DAT_00119070 = 2;
      if (DAT_0011909c != 0) {
        do {
          uVar13 = DAT_0011909c;
          uVar10 = DAT_0011906c;
          uVar8 = (ulong)DAT_0011906c;
          DAT_001190a4 = (DAT_001190a4 << 5 ^ (uint)(&DAT_0013b000)[DAT_0011906c + 2]) & 0x7fff;
          uVar7 = DAT_001190a4 + 0x8000;
          uVar3 = (&DAT_0011a880)[uVar7];
          (&DAT_0011a880)[DAT_0011906c & 0x7fff] = uVar3;
          (&DAT_0011a880)[uVar7] = (ushort)uVar10;
          if (((uVar3 == 0) || (0x7efa < uVar10 - uVar3)) || (0xfefa < uVar10)) {
LAB_001051ba:
            iVar14 = FUN_0010b160(0,(&DAT_0013b000)[uVar8]);
            iVar15 = DAT_0011a000;
            uVar13 = DAT_0011906c;
            if (DAT_0011a000 != 0) {
              FUN_00104960(DAT_0011906c,1);
            }
            DAT_0011906c = uVar13 + 1;
            DAT_0011909c = DAT_0011909c - 1;
          }
          else {
            uVar10 = FUN_00104740();
            if (uVar10 <= uVar13) {
              uVar13 = uVar10;
            }
            if (uVar13 < 3) {
              uVar8 = (ulong)DAT_0011906c;
              goto LAB_001051ba;
            }
            iVar14 = FUN_0010b160(DAT_0011906c - DAT_00119068,uVar13 - 3);
            iVar15 = DAT_0011a000;
            uVar10 = DAT_0011906c;
            DAT_0011909c = DAT_0011909c - uVar13;
            if (DAT_0011a000 == 0) {
              if (uVar13 <= DAT_00119098) goto LAB_0010538f;
LAB_00105338:
              DAT_0011906c = uVar13 + uVar10;
              DAT_001190a4 = (uint)(&DAT_0013b000)[DAT_0011906c] << 5 ^
                             (uint)(&DAT_0013b000)[DAT_0011906c + 1];
            }
            else {
              FUN_00104960(DAT_0011906c,uVar13);
              if (DAT_00119098 < uVar13) goto LAB_00105338;
LAB_0010538f:
              uVar7 = uVar10;
              do {
                uVar5 = uVar7 + 1;
                DAT_001190a4 = (DAT_001190a4 << 5 ^ (uint)(&DAT_0013b000)[uVar7 + 3]) & 0x7fff;
                psVar1 = &DAT_0011a880 + (DAT_001190a4 + 0x8000);
                sVar4 = *psVar1;
                (&DAT_0011a880)[uVar5 & 0x7fff] = sVar4;
                *psVar1 = (short)uVar5;
                uVar7 = uVar5;
              } while (uVar5 != (uVar13 - 1) + uVar10);
              DAT_0011906c = uVar10 + uVar13;
              (&DAT_0011a880)[uVar5 & 0x7fff] = sVar4;
              *psVar1 = (short)uVar10 + -1 + (short)uVar13;
            }
          }
          if ((iVar15 == 0) || (DAT_0011906c <= DAT_00119088)) {
            if (iVar14 != 0) {
              iVar15 = iVar14 + -1;
              goto LAB_00105225;
            }
          }
          else {
            iVar15 = 1;
            iVar14 = 2;
            DAT_00119088 = 0xffffffff;
LAB_00105225:
            puVar12 = &DAT_0013b000 + (DAT_00119078 & 0xffffffff);
            if ((long)DAT_00119078 < 0) {
              puVar12 = (undefined1 *)0x0;
            }
            FUN_0010add0(puVar12,DAT_0011906c - DAT_00119078,iVar15,0);
            DAT_00119078 = (ulong)DAT_0011906c;
          }
          while (DAT_0011909c < 0x106) {
            if (DAT_001190a0 != 0) {
              if (DAT_0011909c == 0) {
                iVar14 = iVar14 + -1;
                DAT_0011909c = 0;
                goto LAB_0010529c;
              }
              break;
            }
            FUN_00104a60();
          }
        } while( true );
      }
      iVar14 = -1;
LAB_0010529c:
      lVar11 = DAT_0011906c - DAT_00119078;
      puVar12 = &DAT_0013b000 + (DAT_00119078 & 0xffffffff);
      if ((long)DAT_00119078 < 0) {
        puVar12 = (undefined1 *)0x0;
      }
      goto LAB_00104fa4;
    }
    if (DAT_0011909c != 0) {
      iVar15 = 0;
      iVar14 = 0;
      uVar10 = DAT_0011909c;
      uVar13 = 2;
      do {
        DAT_00119070 = uVar13;
        uVar7 = DAT_0011906c;
        iVar9 = DAT_00119068;
        uVar8 = (ulong)DAT_0011906c;
        uVar13 = 2;
        DAT_001190a4 = ((uint)(&DAT_0013b000)[DAT_0011906c + 2] ^ DAT_001190a4 << 5) & 0x7fff;
        uVar5 = DAT_001190a4 + 0x8000;
        uVar3 = (&DAT_0011a880)[uVar5];
        (&DAT_0011a880)[DAT_0011906c & 0x7fff] = uVar3;
        (&DAT_0011a880)[uVar5] = (ushort)uVar7;
        uVar5 = (uint)uVar3;
        if ((uVar5 != 0) && (DAT_00119070 < DAT_00119098)) {
          if ((uVar7 - uVar5 < 0x7efb) && (uVar7 < 0xfefb)) {
            uVar7 = FUN_00104740(uVar5);
            uVar13 = uVar10;
            if (uVar7 <= uVar10) {
              uVar13 = uVar7;
            }
            if (uVar13 == 3) {
              uVar8 = (ulong)DAT_0011906c;
              if (0x1000 < DAT_0011906c - DAT_00119068) {
                uVar13 = 2;
              }
            }
            else {
              uVar8 = (ulong)DAT_0011906c;
            }
          }
          else {
            uVar13 = 2;
          }
        }
        uVar10 = (uint)uVar8;
        if ((DAT_00119070 < 3) || (DAT_00119070 < uVar13)) {
          if (iVar15 == 0) {
            if (DAT_0011a000 != 0) {
              if (DAT_00119088 < uVar8) {
                iVar14 = 2;
                DAT_00119088 = 0xffffffff;
                puVar12 = &DAT_0013b000 + (DAT_00119078 & 0xffffffff);
                if ((long)DAT_00119078 < 0) {
                  puVar12 = (undefined1 *)0x0;
                }
                FUN_0010add0(puVar12,uVar8 - DAT_00119078,1,0);
                uVar8 = (ulong)DAT_0011906c;
                uVar10 = DAT_0011906c;
                DAT_00119078 = uVar8;
                if (DAT_0011a000 == 0) goto LAB_00104e00;
              }
              FUN_00104960(uVar8,1);
              uVar10 = (uint)uVar8;
            }
LAB_00104e00:
            DAT_0011906c = uVar10 + 1;
            iVar15 = 1;
            DAT_0011909c = DAT_0011909c - 1;
            uVar10 = DAT_0011909c;
          }
          else {
            iVar14 = FUN_0010b160(0,(&DAT_0013b000)[uVar10 - 1]);
            if (DAT_0011a000 == 0) {
              iVar9 = iVar14 + -1;
              if (iVar14 != 0) goto LAB_00104ebc;
            }
            else {
              uVar8 = (ulong)DAT_0011906c;
              if (DAT_00119088 < uVar8) {
                iVar14 = 2;
                DAT_00119088 = 0xffffffff;
                iVar9 = iVar15;
LAB_00104ebc:
                puVar12 = &DAT_0013b000 + (DAT_00119078 & 0xffffffff);
                if ((long)DAT_00119078 < 0) {
                  puVar12 = (undefined1 *)0x0;
                }
                FUN_0010add0(puVar12,DAT_0011906c - DAT_00119078,iVar9,0);
                uVar8 = (ulong)DAT_0011906c;
                DAT_00119078 = uVar8;
                if (DAT_0011a000 == 0) goto LAB_00104ef8;
              }
              else {
                iVar9 = iVar14 + -1;
                if (iVar14 != 0) goto LAB_00104ebc;
              }
              FUN_00104960(uVar8,1);
              DAT_0011906c = (uint)uVar8;
            }
LAB_00104ef8:
            DAT_0011906c = DAT_0011906c + 1;
            DAT_0011909c = DAT_0011909c - 1;
            uVar10 = DAT_0011909c;
          }
        }
        else {
          iVar14 = FUN_0010b160((uVar10 - iVar9) + -1,DAT_00119070 - 3);
          iVar9 = DAT_0011a000;
          uVar7 = DAT_00119070;
          uVar13 = DAT_0011906c;
          iVar15 = DAT_00119070 - 2;
          uVar10 = (DAT_0011909c + 1) - DAT_00119070;
          DAT_0011909c = uVar10;
          if (DAT_0011a000 != 0) {
            iVar2 = DAT_00119070 - 1;
            DAT_00119070 = iVar15;
            FUN_00104960(DAT_0011906c,iVar2);
          }
          uVar5 = uVar13;
          do {
            uVar6 = uVar5 + 1;
            DAT_001190a4 = (DAT_001190a4 << 5 ^ (uint)(&DAT_0013b000)[uVar5 + 3]) & 0x7fff;
            psVar1 = &DAT_0011a880 + (DAT_001190a4 + 0x8000);
            sVar4 = *psVar1;
            (&DAT_0011a880)[uVar6 & 0x7fff] = sVar4;
            *psVar1 = (short)uVar6;
            uVar5 = uVar6;
          } while (uVar6 != iVar15 + uVar13);
          DAT_0011906c = (uVar7 - 1) + uVar13;
          (&DAT_0011a880)[uVar6 & 0x7fff] = sVar4;
          *psVar1 = (short)uVar13 + -2 + (short)uVar7;
          DAT_00119070 = 0;
          if ((iVar9 == 0) || (DAT_0011906c <= DAT_00119088)) {
            if (iVar14 == 0) {
              iVar15 = 0;
              uVar13 = 2;
              goto joined_r0x00104e25;
            }
            iVar9 = iVar14 + -1;
          }
          else {
            iVar9 = 1;
            iVar14 = 2;
            DAT_00119088 = 0xffffffff;
          }
          uVar13 = 2;
          puVar12 = &DAT_0013b000 + (DAT_00119078 & 0xffffffff);
          if ((long)DAT_00119078 < 0) {
            puVar12 = (undefined1 *)0x0;
          }
          iVar15 = 0;
          FUN_0010add0(puVar12,DAT_0011906c - DAT_00119078,iVar9,0);
          DAT_00119078 = (ulong)DAT_0011906c;
          uVar10 = DAT_0011909c;
        }
joined_r0x00104e25:
        while (uVar10 < 0x106) {
          if (DAT_001190a0 != 0) {
            uVar10 = DAT_0011909c;
            if (DAT_0011909c == 0) {
              iVar14 = iVar14 + -1;
              if (iVar15 != 0) {
                FUN_0010b160(0,(&DAT_0013b000)[DAT_0011906c - 1]);
              }
              goto LAB_00104f79;
            }
            break;
          }
          FUN_00104a60();
          uVar10 = DAT_0011909c;
        }
      } while( true );
    }
  }
  else {
    DAT_001190a0 = 1;
    DAT_0011909c = 0;
    if (param_1 < 4) goto LAB_00105136;
  }
  iVar14 = -1;
LAB_00104f79:
  lVar11 = DAT_0011906c - DAT_00119078;
  puVar12 = &DAT_0013b000 + (DAT_00119078 & 0xffffffff);
  if ((long)DAT_00119078 < 0) {
    puVar12 = (undefined1 *)0x0;
  }
LAB_00104fa4:
  FUN_0010add0(puVar12,lVar11,iVar14,1);
  return;
}




// Function: license @ 0x54d0

void license(void)

{
  undefined **ppuVar1;
  char *pcVar2;
  
  ppuVar1 = &PTR_s_Copyright__C__2018_Free_Software_00117b20;
  FUN_0010f830("%s %s\n",DAT_0011a850,PTR_DAT_001183f8);
  pcVar2 = "Copyright (C) 2018 Free Software Foundation, Inc.";
  do {
    ppuVar1 = ppuVar1 + 1;
    FUN_0010f830("%s\n",pcVar2);
    pcVar2 = *ppuVar1;
  } while (pcVar2 != (char *)0x0);
  return;
}




// Function: remove_output_file @ 0x5530

void remove_output_file(char param_1)

{
  char cVar1;
  char *pcVar2;
  char *pcVar3;
  int __fd;
  long in_FS_OFFSET;
  sigset_t sStack_4a8;
  char local_428 [1032];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 == '\0') {
    sigprocmask(0,(sigset_t *)&DAT_001dc940,&sStack_4a8);
    __fd = DAT_00118018;
    if (-1 < DAT_00118018) goto LAB_0010559e;
  }
  else {
    __fd = DAT_00118018;
    if (DAT_00118018 < 0) goto LAB_0010555d;
LAB_0010559e:
    DAT_00118018 = -1;
    close(__fd);
    pcVar2 = &DAT_001dc520;
    pcVar3 = local_428;
    do {
      cVar1 = *pcVar2;
      *pcVar3 = cVar1;
      pcVar2 = pcVar2 + 1;
      pcVar3 = pcVar3 + 1;
    } while (cVar1 != '\0');
    unlink(local_428);
    if (param_1 != '\0') goto LAB_0010555d;
  }
  sigprocmask(2,&sStack_4a8,(sigset_t *)0x0);
LAB_0010555d:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: abort_gzip_signal @ 0x5600

void abort_gzip_signal(int param_1)

{
  FUN_00105530(1);
  if (DAT_001dc920 != param_1) {
    signal(param_1,(__sighandler_t)0x0);
    raise(param_1);
    return;
  }
                    /* WARNING: Subroutine does not return */
  _exit(2);
}




// Function: discard_input_bytes @ 0x5690

void discard_input_bytes(long param_1,uint param_2)

{
  ulong uVar1;
  long in_FS_OFFSET;
  char local_21;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
joined_r0x001056ba:
  if (param_1 == 0) {
LAB_001056f0:
    if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    return;
  }
  if (DAT_0011a00c <= DAT_0011a008) goto LAB_00105725;
  do {
    uVar1 = (ulong)DAT_0011a008;
    DAT_0011a008 = DAT_0011a008 + 1;
    local_21 = (&DAT_0019c000)[uVar1];
    while( true ) {
      if ((param_2 & 2) != 0) {
        FUN_0010d0d0(&local_21,1);
      }
      if (param_1 != -1) {
        param_1 = param_1 + -1;
        goto joined_r0x001056ba;
      }
      if (local_21 == '\0') goto LAB_001056f0;
      if (DAT_0011a008 < DAT_0011a00c) break;
LAB_00105725:
      local_21 = FUN_0010d6c0(0);
    }
  } while( true );
}




// Function: get_method @ 0x5750

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
  uVar14 = (ulong)DAT_0011a008;
  if ((DAT_001dc9f4 == 0) || (DAT_0011a860 == 0)) {
    if (DAT_0011a008 < DAT_0011a00c) {
      DAT_0011a008 = DAT_0011a008 + 1;
      local_4a = (&DAT_0019c000)[uVar14];
    }
    else {
      local_4a = FUN_0010d6c0(0);
    }
    uVar14 = (ulong)DAT_0011a008;
    if (local_4a == '\0') {
      if (DAT_0011a008 < DAT_0011a00c) {
        DAT_0011a008 = DAT_0011a008 + 1;
        uVar8 = (uint)(byte)(&DAT_0019c000)[uVar14];
      }
      else {
        uVar8 = FUN_0010d6c0(1);
      }
      uVar9 = 0;
      uVar7 = uVar8;
    }
    else {
      if (DAT_0011a008 < DAT_0011a00c) {
        DAT_0011a008 = DAT_0011a008 + 1;
        uVar7 = (uint)(byte)(&DAT_0019c000)[uVar14];
      }
      else {
        uVar7 = FUN_0010d6c0(0);
      }
      uVar8 = 0;
      uVar9 = 0;
    }
  }
  else {
    if (DAT_0011a008 < DAT_0011a00c) {
      DAT_0011a008 = DAT_0011a008 + 1;
      uVar9 = (uint)(byte)(&DAT_0019c000)[uVar14];
    }
    else {
      uVar9 = FUN_0010d6c0(1);
    }
    uVar14 = (ulong)DAT_0011a008;
    local_4a = (char)uVar9;
    if (DAT_0011a008 < DAT_0011a00c) {
      DAT_0011a008 = DAT_0011a008 + 1;
      uVar8 = (uint)(byte)(&DAT_0019c000)[uVar14];
      uVar7 = (uint)(byte)(&DAT_0019c000)[uVar14];
    }
    else {
      uVar8 = FUN_0010d6c0(1);
      uVar7 = uVar8;
    }
  }
  local_49 = (undefined1)uVar7;
  DAT_00118020 = 0xffffffff;
  DAT_001dfea8 = 0;
  iVar10 = DAT_001dc9d8 + 1;
  DAT_001dc9dc = 0;
  DAT_001dc9d8 = iVar10;
  iVar6 = memcmp(&local_4a,&DAT_00112c6d,2);
  if ((iVar6 == 0) || (iVar6 = memcmp(&local_4a,&DAT_00112c70,2), iVar6 == 0)) {
    uVar14 = (ulong)DAT_0011a008;
    if (DAT_0011a008 < DAT_0011a00c) {
      DAT_0011a008 = DAT_0011a008 + 1;
      DAT_00118020 = (uint)(byte)(&DAT_0019c000)[uVar14];
    }
    else {
      DAT_00118020 = FUN_0010d6c0(0);
    }
    if (DAT_00118020 != 8) {
      FUN_0010f6b0(stderr,"%s: %s: unknown method %d -- not supported\n",DAT_0011a850,&DAT_0011a420,
                   DAT_00118020);
      uVar7 = 0xffffffff;
      DAT_0011a84c = 1;
      goto LAB_001058e0;
    }
    PTR_FUN_001180d0 = FUN_0010cc20;
    uVar14 = (ulong)DAT_0011a008;
    if (DAT_0011a008 < DAT_0011a00c) {
      DAT_0011a008 = DAT_0011a008 + 1;
      bVar3 = (&DAT_0019c000)[uVar14];
    }
    else {
      bVar3 = FUN_0010d6c0(0);
    }
    if ((bVar3 & 0x20) == 0) {
      if ((bVar3 & 0xc0) == 0) {
        if (DAT_0011a00c <= DAT_0011a008) goto LAB_00105d62;
LAB_00105a0f:
        uVar14 = (ulong)DAT_0011a008;
        DAT_0011a008 = DAT_0011a008 + 1;
        uVar14 = (ulong)(byte)(&DAT_0019c000)[uVar14];
        if (DAT_0011a00c <= DAT_0011a008) goto LAB_00105d80;
LAB_00105a2c:
        uVar11 = (ulong)DAT_0011a008;
        DAT_0011a008 = DAT_0011a008 + 1;
        uVar14 = uVar14 | (ulong)(byte)(&DAT_0019c000)[uVar11] << 8;
        if (DAT_0011a00c <= DAT_0011a008) goto LAB_00105da4;
LAB_00105a4f:
        uVar11 = (ulong)DAT_0011a008;
        DAT_0011a008 = DAT_0011a008 + 1;
        uVar14 = uVar14 | (ulong)(byte)(&DAT_0019c000)[uVar11] << 0x10;
        if (DAT_0011a00c <= DAT_0011a008) goto LAB_00105dc9;
LAB_00105a73:
        uVar11 = (ulong)DAT_0011a008;
        DAT_0011a008 = DAT_0011a008 + 1;
        uVar11 = (ulong)(byte)(&DAT_0019c000)[uVar11];
      }
      else {
        FUN_0010f6b0(stderr,"%s: %s has flags 0x%x -- not supported\n",DAT_0011a850,&DAT_0011a420,
                     bVar3);
        DAT_0011a84c = 1;
        if (DAT_001dc9f4 < 2) goto LAB_001062c4;
        if (DAT_0011a008 < DAT_0011a00c) goto LAB_00105a0f;
LAB_00105d62:
        iVar10 = FUN_0010d6c0(0);
        uVar14 = (ulong)iVar10;
        if (DAT_0011a008 < DAT_0011a00c) goto LAB_00105a2c;
LAB_00105d80:
        iVar10 = FUN_0010d6c0(0);
        uVar14 = uVar14 | (long)iVar10 << 8;
        if (DAT_0011a008 < DAT_0011a00c) goto LAB_00105a4f;
LAB_00105da4:
        iVar10 = FUN_0010d6c0(0);
        uVar14 = uVar14 | (long)iVar10 << 0x10;
        if (DAT_0011a008 < DAT_0011a00c) goto LAB_00105a73;
LAB_00105dc9:
        iVar10 = FUN_0010d6c0(0);
        uVar11 = (ulong)iVar10;
      }
      uVar14 = uVar14 | uVar11 << 0x18;
      if ((uVar14 != 0) && (DAT_00118028 == 0)) {
        if ((long)uVar14 < 0) {
          if (DAT_0011a85c == 0) {
            FUN_0010f6b0(stderr,"%s: %s: MTIME %lu out of range for this platform\n",DAT_0011a850,
                         &DAT_0011a420,uVar14);
          }
          if (DAT_0011a84c == 0) {
            DAT_0011a84c = 2;
          }
          DAT_0011a838 = 999999999;
          DAT_0011a830 = 0x7fffffffffffffff;
        }
        else {
          DAT_0011a838 = 0;
          DAT_0011a830 = uVar14;
        }
      }
      uVar11 = (ulong)DAT_0011a008;
      if (DAT_0011a008 < DAT_0011a00c) {
        DAT_0011a008 = DAT_0011a008 + 1;
        local_42 = (&DAT_0019c000)[uVar11];
      }
      else {
        local_42 = FUN_0010d6c0(0);
      }
      uVar11 = (ulong)DAT_0011a008;
      if (DAT_0011a008 < DAT_0011a00c) {
        DAT_0011a008 = DAT_0011a008 + 1;
        local_41 = (&DAT_0019c000)[uVar11];
      }
      else {
        local_41 = FUN_0010d6c0(0);
      }
      if ((bVar3 & 2) != 0) {
        local_48 = 8;
        local_46 = (undefined4)uVar14;
        local_47 = bVar3;
        FUN_0010d0d0(0,0);
        FUN_0010d0d0(&local_4a,10);
      }
      if ((bVar3 & 4) != 0) {
        uVar14 = (ulong)DAT_0011a008;
        if (DAT_0011a008 < DAT_0011a00c) {
          DAT_0011a008 = DAT_0011a008 + 1;
          uVar4 = (&DAT_0019c000)[uVar14];
        }
        else {
          uVar4 = FUN_0010d6c0(0);
        }
        uVar14 = (ulong)DAT_0011a008;
        local_4c = uVar4;
        if (DAT_0011a008 < DAT_0011a00c) {
          DAT_0011a008 = DAT_0011a008 + 1;
          local_4b = (&DAT_0019c000)[uVar14];
        }
        else {
          local_4b = FUN_0010d6c0(0);
        }
        uVar1 = CONCAT11(local_4b,uVar4);
        if (DAT_001dc9e4 != 0) {
          FUN_0010f6b0(stderr,"%s: %s: extra field of %u bytes ignored\n",DAT_0011a850,&DAT_0011a420
                       ,uVar1);
        }
        if ((bVar3 & 2) != 0) {
          FUN_0010d0d0(&local_4c,2);
        }
        FUN_00105690(uVar1,bVar3);
      }
      if ((bVar3 & 8) != 0) {
        if ((DAT_0011802c == 0) &&
           (((DAT_0011a860 == 0 || (DAT_001dc9e8 != 0)) && (DAT_001dc9d8 < 2)))) {
          __dest = (char *)thunk_FUN_0010dfd0(&DAT_0011a020);
          pcVar13 = __dest;
          while( true ) {
            uVar14 = (ulong)DAT_0011a008;
            if (DAT_0011a008 < DAT_0011a00c) {
              DAT_0011a008 = DAT_0011a008 + 1;
              cVar2 = (&DAT_0019c000)[uVar14];
            }
            else {
              cVar2 = FUN_0010d6c0(0);
            }
            *pcVar13 = cVar2;
            pcVar13 = pcVar13 + 1;
            if (cVar2 == '\0') break;
            if ((char *)0x11a41f < pcVar13) {
                    /* WARNING: Subroutine does not return */
              FUN_0010d3f0("corrupted input -- file name too large");
            }
          }
          if ((bVar3 & 2) != 0) {
            FUN_0010d0d0(__dest,(int)pcVar13 - (int)__dest);
          }
          pcVar13 = (char *)thunk_FUN_0010dfd0(__dest);
          sVar12 = strlen(pcVar13);
          memmove(__dest,pcVar13,sVar12 + 1);
        }
        else {
          FUN_00105690(0xffffffffffffffff,bVar3);
        }
      }
      if ((bVar3 & 0x10) != 0) {
        FUN_00105690(0xffffffffffffffff,bVar3);
      }
      if ((bVar3 & 2) != 0) {
        uVar5 = FUN_0010d0d0(&local_4a,0);
        uVar14 = (ulong)DAT_0011a008;
        if (DAT_0011a008 < DAT_0011a00c) {
          DAT_0011a008 = DAT_0011a008 + 1;
          uVar7 = (uint)(byte)(&DAT_0019c000)[uVar14];
        }
        else {
          uVar7 = FUN_0010d6c0(0);
        }
        uVar14 = (ulong)DAT_0011a008;
        if (DAT_0011a008 < DAT_0011a00c) {
          DAT_0011a008 = DAT_0011a008 + 1;
          uVar9 = (uint)(byte)(&DAT_0019c000)[uVar14];
        }
        else {
          uVar9 = FUN_0010d6c0(0);
        }
        uVar7 = uVar9 << 8 | uVar7;
        if (uVar5 != uVar7) {
          FUN_0010f6b0(stderr,"%s: %s: header checksum 0x%04x != computed checksum 0x%04x\n",
                       DAT_0011a850,&DAT_0011a420,uVar7,(uint)uVar5);
          DAT_0011a84c = 1;
          if (DAT_001dc9f4 < 2) goto LAB_001062c4;
        }
      }
      uVar7 = DAT_00118020;
      if (DAT_001dc9d8 != 1) {
        if (-1 < (int)DAT_00118020) goto LAB_001058e0;
        goto LAB_00105c60;
      }
      DAT_001dfea8 = (ulong)(DAT_0011a008 + 8);
      if (-1 < (int)DAT_00118020) goto LAB_001058e0;
      goto LAB_00106297;
    }
    pcVar13 = "%s: %s is encrypted -- not supported\n";
LAB_001062ac:
    FUN_0010f6b0(stderr,pcVar13,DAT_0011a850,&DAT_0011a420);
    DAT_0011a84c = 1;
  }
  else {
    iVar6 = memcmp(&local_4a,&DAT_00112c90,2);
    if (((iVar6 != 0) || (DAT_0011a008 != 2)) ||
       (iVar6 = memcmp(&DAT_0019c000,&DAT_00112c90,4), iVar6 != 0)) {
      iVar6 = memcmp(&local_4a,&DAT_00112c95,2);
      if (iVar6 == 0) {
        DAT_00118020 = 2;
        PTR_FUN_001180d0 = FUN_0010c590;
        uVar7 = 2;
        goto LAB_001058e0;
      }
      iVar6 = memcmp(&local_4a,&DAT_00112c98,2);
      if (iVar6 == 0) {
        DAT_00118020 = 1;
        DAT_001dc9dc = 1;
        PTR_FUN_001180d0 = FUN_0010be90;
        uVar7 = 1;
        goto LAB_001058e0;
      }
      iVar6 = memcmp(&local_4a,&DAT_00112c9b,2);
      if (iVar6 == 0) {
        DAT_00118020 = 3;
        DAT_001dc9dc = 1;
        PTR_FUN_001180d0 = FUN_0010b920;
        uVar7 = 3;
        goto LAB_001058e0;
      }
      if (((DAT_001dc9f4 != 0) && (DAT_0011a860 != 0)) && (DAT_001dc9e8 == 0)) {
        DAT_00118020 = 0;
        PTR_FUN_001180d0 = FUN_0010d770;
        if (uVar8 != 0xffffffff) {
          DAT_0011a008 = DAT_0011a008 - 1;
        }
        DAT_001dc9dc = 1;
        uVar7 = 0;
        if (uVar9 == 0xffffffff) goto LAB_001058e0;
        FUN_0010d5a0(1,&local_4a,1);
        goto LAB_0010627a;
      }
LAB_0010628e:
      if (iVar10 != 1) {
LAB_00105c60:
        if (local_4a == '\0') {
          while (uVar8 == 0) {
            uVar14 = (ulong)DAT_0011a008;
            if (DAT_0011a008 < DAT_0011a00c) {
              DAT_0011a008 = DAT_0011a008 + 1;
              uVar8 = (uint)(byte)(&DAT_0019c000)[uVar14];
            }
            else {
              uVar8 = FUN_0010d6c0(1);
            }
          }
          if (uVar8 == 0xffffffff) {
            if (DAT_001dc9e4 != 0) {
              if (DAT_0011a85c == 0) {
                FUN_0010f6b0(stderr,"\n%s: %s: decompression OK, trailing zero bytes ignored\n",
                             DAT_0011a850,&DAT_0011a420);
              }
              if (DAT_0011a84c == 0) {
                DAT_0011a84c = 2;
              }
            }
            uVar7 = 0xfffffffd;
            goto LAB_001058e0;
          }
        }
        if (DAT_0011a85c == 0) {
          FUN_0010f6b0(stderr,"\n%s: %s: decompression OK, trailing garbage ignored\n",DAT_0011a850,
                       &DAT_0011a420);
        }
        uVar7 = 0xfffffffe;
        if (DAT_0011a84c == 0) {
          DAT_0011a84c = 2;
        }
        goto LAB_001058e0;
      }
LAB_00106297:
      pcVar13 = "\n%s: %s: not in gzip format\n";
      goto LAB_001062ac;
    }
    DAT_0011a008 = 0;
    PTR_FUN_001180d0 = FUN_0010cc20;
    iVar10 = FUN_0010cb20(param_1);
    if (iVar10 == 0) {
      DAT_001dc9dc = 1;
LAB_0010627a:
      iVar10 = DAT_001dc9d8;
      uVar7 = DAT_00118020;
      if (-1 < (int)DAT_00118020) goto LAB_001058e0;
      goto LAB_0010628e;
    }
  }
LAB_001062c4:
  uVar7 = 0xffffffff;
LAB_001058e0:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar7;
}




// Function: get_suffix @ 0x63e0

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
  
  pcVar9 = DAT_001dc9c8;
  uVar1 = DAT_001dc9c0;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (PTR_DAT_00118048 != (undefined *)0x0) {
    ppuVar8 = &PTR_DAT_00118048;
    pcVar10 = PTR_DAT_00118048;
    do {
      sVar6 = strlen(pcVar10);
      if (uVar1 < sVar6) {
        iVar2 = strcmp(pcVar9,pcVar10 + (sVar6 - uVar1));
        if (iVar2 == 0) {
          ppuVar8 = &PTR_DAT_00118048;
          __ptr = (void *)FUN_0010f640(pcVar9);
          FUN_0010d230(__ptr);
          lVar7 = 8;
          goto LAB_00106479;
        }
      }
      pcVar10 = ppuVar8[1];
      ppuVar8 = ppuVar8 + 1;
    } while (pcVar10 != (char *)0x0);
  }
  __ptr = (void *)FUN_0010f640(pcVar9);
  FUN_0010d230(__ptr);
  lVar7 = 0;
  ppuVar8 = (undefined **)&DAT_00118040;
LAB_00106479:
  (&DAT_00118040)[lVar7] = __ptr;
  sVar6 = strlen(param_1);
  iVar2 = (int)sVar6;
  if (iVar2 < 0x21) {
    __strcpy_chk(local_68,param_1,0x21);
  }
  else {
    __strcpy_chk(local_68,param_1 + (long)iVar2 + -0x20,0x21);
  }
  FUN_0010d230(local_68);
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




// Function: do_list @ 0x65d0

void do_list(int param_1)

{
  tm *ptVar1;
  long lVar2;
  undefined8 uVar3;
  
  if (DAT_00118010 == 0) {
    if (param_1 < 0) goto LAB_00106718;
  }
  else {
    if (param_1 < 0) {
LAB_00106718:
      if ((0 < DAT_001dc508) && (0 < DAT_001dc500)) {
        if (((DAT_001dc9e4 != 0) && (FUN_0010f830(&DAT_00112cc8), DAT_001dc9e4 != 0)) ||
           (DAT_0011a85c == 0)) {
          FUN_0010d8a0(stdout,DAT_001dc508,0x13);
          FUN_0010f830(&DAT_00112ce3);
          FUN_0010d8a0(stdout,DAT_001dc500,0x13);
          FUN_0010f830(&DAT_00112ce3);
        }
        FUN_0010d850(DAT_001dc500 - (DAT_001dc508 - DAT_001dfea8),DAT_001dc500,stdout);
        FUN_0010f830(" (totals)\n");
        return;
      }
      return;
    }
    DAT_00118010 = 0;
    if (DAT_001dc9e4 != 0) {
      FUN_0010f830("method  crc     date  time  ");
    }
    if (DAT_0011a85c == 0) {
      FUN_0010f830("%*.*s %*.*s  ratio uncompressed_name\n",0x13,0x13,"compressed",0x13,0x13,
                   "uncompressed");
    }
  }
  uVar3 = 0xffffffffffffffff;
  if ((param_1 == 8) && (DAT_001dc9dc == 0)) {
    uVar3 = DAT_001dfe90;
  }
  if (DAT_001dc9e4 != 0) {
    ptVar1 = localtime(&DAT_0011a830);
    FUN_0010f830("%5s %08lx ",(&PTR_s_store_00117680)[param_1],uVar3);
    if (ptVar1 == (tm *)0x0) {
      FUN_0010f830("??? ?? ??:?? ");
    }
    else {
      FUN_0010f830("%s%3d %02d:%02d ",&DAT_001132e0 + (long)ptVar1->tm_mon * 4,ptVar1->tm_mday,
                   ptVar1->tm_hour,ptVar1->tm_min);
    }
  }
  FUN_0010d8a0(stdout,DAT_0011a828,0x13);
  FUN_0010f830(&DAT_00112ce3);
  FUN_0010d8a0(stdout,DAT_0011a820,0x13);
  FUN_0010f830(&DAT_00112ce3);
  if (DAT_0011a828 == -1) {
    DAT_001dc508 = -1;
    DAT_001dfea8 = 0;
    DAT_0011a820 = 0;
    DAT_0011a828 = 0;
  }
  else {
    if (-1 < DAT_001dc508) {
      DAT_001dc508 = DAT_001dc508 + DAT_0011a828;
    }
    if (DAT_0011a820 == -1) {
      DAT_001dc500 = -1;
      lVar2 = 0;
      DAT_001dfea8 = 0;
      DAT_0011a820 = 0;
      DAT_0011a828 = 0;
      goto LAB_001066eb;
    }
  }
  if (-1 < DAT_001dc500) {
    DAT_001dc500 = DAT_001dc500 + DAT_0011a820;
  }
  lVar2 = DAT_0011a820 - (DAT_0011a828 - DAT_001dfea8);
LAB_001066eb:
  FUN_0010d850(lVar2,DAT_0011a820,stdout);
  FUN_0010f830(" %s\n",&DAT_0011a020);
  return;
}




// Function: do_exit @ 0x6960

void do_exit(int param_1)

{
  if (DAT_001dc040 == 0) {
    DAT_001dc040 = 1;
    free(DAT_001dc9d0);
    DAT_001dc9d0 = (void *)0x0;
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: finish_out @ 0x69a0

void finish_out(void)

{
  int iVar1;
  
  iVar1 = FUN_0010e060(stdout);
  if (iVar1 != 0) {
    FUN_0010d550();
  }
                    /* WARNING: Subroutine does not return */
  FUN_00106960(0);
}




// Function: try_help @ 0x69d0

void try_help(void)

{
  FUN_0010f6b0(stderr,"Try `%s --help\' for more information.\n",DAT_0011a850);
                    /* WARNING: Subroutine does not return */
  FUN_00106960(1);
}




// Function: treat_stdin @ 0x6a80

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void treat_stdin(void)

{
  int iVar1;
  char *pcVar2;
  undefined *puVar3;
  
  if ((DAT_001dc9f4 == 0 && DAT_001dc9e8 == 0) &&
     ((DAT_001dca01 != '\0' || (iVar1 = isatty((uint)(DAT_001dc9f8 == 0)), iVar1 != 0)))) {
    if (DAT_0011a85c == 0) {
      pcVar2 = "read from";
      if (DAT_001dc9f8 == 0) {
        pcVar2 = "written to";
      }
      puVar3 = &DAT_00112d1a;
      if (DAT_001dc9f8 == 0) {
        puVar3 = &DAT_00112c94;
      }
      FUN_0010f6b0(stderr,
                   "%s: compressed data not %s a terminal. Use -f to force %scompression.\nFor help, type: %s -h\n"
                   ,DAT_0011a850,pcVar2,puVar3,DAT_0011a850);
    }
  }
  else {
    _DAT_0011a420 = 0x69647473;
    _DAT_0011a424 = 0x6e;
    _DAT_0011a020 = 0x6f647473;
    _DAT_0011a024 = 0x7475;
    DAT_0011a026 = 0;
    iVar1 = fstat(0,(stat *)&DAT_001dc060);
    if (iVar1 == 0) {
      FUN_00106a00();
      FUN_0010d150();
      DAT_0011a860 = 1;
      DAT_001dc9d8 = 0;
      DAT_0011a014 = 0;
      DAT_001dc510 = 1;
      if ((DAT_001dc9f8 != 0) && (DAT_00118020 = FUN_00105750(0), DAT_00118020 < 0)) {
                    /* WARNING: Subroutine does not return */
        FUN_00106960(DAT_0011a84c);
      }
      while( true ) {
        iVar1 = (*(code *)PTR_FUN_001180d0)(0,1);
        if (iVar1 != 0) {
          return;
        }
        if ((DAT_001dc9f8 == 0) || (DAT_001dc9dc != 0)) break;
        if (DAT_0011a008 == DAT_0011a00c) {
          if ((DAT_0011a00c != 0x40000) || (iVar1 = FUN_0010d6c0(1), iVar1 == -1)) break;
          DAT_0011a008 = 0;
        }
        DAT_00118020 = FUN_00105750();
        if (DAT_00118020 < 0) {
          return;
        }
        DAT_0011a820 = 0;
      }
      if (DAT_001dc9e8 != 0) {
        FUN_001065d0(DAT_00118020);
        return;
      }
      if (DAT_001dc9e4 == 0) {
        return;
      }
      if (DAT_0011a858 != 0) {
        FUN_0010f6b0(stderr,&DAT_00112d41);
        return;
      }
      if (DAT_001dc9f8 != 0) {
        return;
      }
      FUN_0010d850((DAT_0011a828 - DAT_0011a820) + DAT_001dfea8,DAT_0011a828,stderr);
      FUN_0010f6b0(stderr,"\n");
      return;
    }
    FUN_00105640("standard input");
  }
                    /* WARNING: Subroutine does not return */
  FUN_00106960(1);
}




// Function: atdir_set @ 0x6d00

void atdir_set(undefined *param_1,size_t param_2)

{
  int iVar1;
  
  if (param_2 == 0) {
    param_2 = 1;
    param_1 = &DAT_00113310;
  }
  iVar1 = memcmp(&DAT_001dc100,param_1,param_2);
  if ((iVar1 == 0) && ((&DAT_001dc100)[param_2] == '\0')) {
    return;
  }
  if (-1 < DAT_00118014) {
    close(DAT_00118014);
  }
  __memcpy_chk(&DAT_001dc100,param_1,param_2,0x400);
  (&DAT_001dc100)[param_2] = 0;
  DAT_00118014 = FUN_0010e360(&DAT_001dc100,0x10000);
  return;
}




// Function: abort_gzip @ 0x8440

void abort_gzip(void)

{
  FUN_00105530(0);
                    /* WARNING: Subroutine does not return */
  FUN_00106960(1);
}




// Function: inflate_codes @ 0x8460

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
  uVar16 = DAT_001dca10;
  uVar13 = DAT_001dca0c;
  uVar12 = DAT_0011a004;
joined_r0x001084c5:
  do {
    while( true ) {
      for (; uVar13 < param_3; uVar13 = uVar13 + 8) {
        uVar10 = (ulong)DAT_0011a008;
        if (DAT_0011a008 < DAT_0011a00c) {
          DAT_0011a008 = DAT_0011a008 + 1;
          bVar7 = (&DAT_0019c000)[uVar10];
        }
        else {
          DAT_0011a004 = uVar12;
          bVar7 = FUN_0010d6c0(0);
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
          uVar10 = (ulong)DAT_0011a008;
          if (DAT_0011a008 < DAT_0011a00c) {
            DAT_0011a008 = DAT_0011a008 + 1;
            bVar7 = (&DAT_0019c000)[uVar10];
          }
          else {
            DAT_0011a004 = uVar12;
            bVar7 = FUN_0010d6c0(0);
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
      (&DAT_0013b000)[uVar10] = (char)*(undefined2 *)(pbVar15 + 8);
      if (uVar12 == 0x8000) {
        DAT_0011a004 = 0x8000;
        FUN_0010d640();
        uVar12 = 0;
      }
    }
    if (uVar11 == 0xf) {
      DAT_0011a004 = uVar12;
      DAT_001dca0c = uVar13;
      DAT_001dca10 = uVar16;
      return 0;
    }
    for (; uVar13 < uVar11; uVar13 = uVar13 + 8) {
      uVar10 = (ulong)DAT_0011a008;
      if (DAT_0011a008 < DAT_0011a00c) {
        DAT_0011a008 = DAT_0011a008 + 1;
        bVar6 = (&DAT_0019c000)[uVar10];
      }
      else {
        DAT_0011a004 = uVar12;
        bVar6 = FUN_0010d6c0(0);
      }
      uVar16 = uVar16 | (ulong)bVar6 << ((byte)uVar13 & 0x3f);
    }
    uVar4 = *(ushort *)(pbVar15 + 8);
    uVar10 = uVar16 >> (bVar7 & 0x3f);
    uVar5 = *(ushort *)(&DAT_00113380 + (ulong)bVar7 * 2);
    for (uVar13 = uVar13 - uVar11; uVar13 < param_4; uVar13 = uVar13 + 8) {
      uVar9 = (ulong)DAT_0011a008;
      if (DAT_0011a008 < DAT_0011a00c) {
        DAT_0011a008 = DAT_0011a008 + 1;
        bVar7 = (&DAT_0019c000)[uVar9];
      }
      else {
        DAT_0011a004 = uVar12;
        bVar7 = FUN_0010d6c0(0);
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
          uVar9 = (ulong)DAT_0011a008;
          if (DAT_0011a008 < DAT_0011a00c) {
            DAT_0011a008 = DAT_0011a008 + 1;
            bVar7 = (&DAT_0019c000)[uVar9];
          }
          else {
            DAT_0011a004 = uVar12;
            bVar7 = FUN_0010d6c0(0);
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
          uVar9 = (ulong)DAT_0011a008;
          if (DAT_0011a008 < DAT_0011a00c) break;
          DAT_0011a004 = uVar12;
          bVar7 = FUN_0010d6c0(0);
          uVar10 = uVar10 | (ulong)bVar7 << ((byte)uVar8 & 0x3f);
          uVar13 = uVar8 + 8;
          if (uVar11 <= uVar8 + 8) goto LAB_0010883d;
        }
        DAT_0011a008 = DAT_0011a008 + 1;
        uVar10 = uVar10 | (ulong)(byte)(&DAT_0019c000)[uVar9] << ((byte)uVar8 & 0x3f);
        uVar13 = uVar8 + 8;
      } while (uVar8 + 8 < uVar11);
LAB_0010883d:
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
          (&DAT_0013b000)[uVar1] = (&DAT_0013b000)[uVar10];
        } while (uVar8 != uVar11 + uVar17);
        if (uVar12 + uVar17 != 0x8000) goto LAB_001088c2;
LAB_00108937:
        uVar11 = uVar11 + uVar17;
        DAT_0011a004 = 0x8000;
        uVar12 = 0;
        FUN_0010d640();
        if (uVar14 == 0) goto joined_r0x001084c5;
      }
      memcpy(&DAT_0013b000 + uVar12,&DAT_0013b000 + uVar11,uVar10);
      if (uVar12 + uVar17 == 0x8000) goto LAB_00108937;
LAB_001088c2:
      uVar11 = uVar11 + uVar17;
      uVar12 = uVar12 + uVar17;
    } while (uVar14 != 0);
  } while( true );
}




// Function: huft_build @ 0x89b0

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
LAB_00108ff6:
      uVar3 = 3;
    }
    else {
      *(undefined8 *)((long)pvVar8 + 8) = 0;
      *(undefined2 *)((long)pvVar8 + 0x10) = 0x163;
      *(undefined2 *)((long)pvVar8 + 0x20) = 0x163;
      DAT_001dca08 = DAT_001dca08 + 3;
      *param_6 = (long)pvVar8 + 0x10;
      uVar3 = 0;
      *param_7 = 1;
    }
    goto LAB_00108b05;
  }
  lVar4 = 1;
  do {
    local_640 = (uint)lVar4;
    if (*(int *)(local_5e8 + lVar4 * 4) != 0) {
      iVar19 = 1 << ((byte)lVar4 & 0x1f);
      goto LAB_00108a8d;
    }
    lVar4 = lVar4 + 1;
  } while (lVar4 != 0x11);
  local_640 = 0x11;
  iVar19 = 0x20000;
LAB_00108a8d:
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
      if (uVar22 <= local_640) {
        local_5f0 = iVar19 - *(int *)(local_5e8 + (ulong)uVar22 * 4);
        if (local_5f0 < 0) goto LAB_00108b00;
        *(int *)(local_5e8 + (ulong)uVar22 * 4) = iVar19;
        local_598[1] = 0;
        if (uVar22 != 1) goto LAB_00108bbd;
        goto LAB_00108beb;
      }
      uVar7 = (ulong)local_640;
      iVar19 = iVar19 - *(int *)(local_5e8 + uVar7 * 4);
      if (iVar19 < 0) goto LAB_00108b00;
      puVar17 = local_5e8 + uVar7 * 4;
      goto LAB_00108b89;
    }
    piVar5 = piVar5 + -1;
    uVar22 = uVar22 - 1;
  } while (uVar22 != 0);
  if (uVar25 == 0) {
    local_5f0 = iVar19 - local_5e8._0_4_;
    *param_7 = 0;
    if (local_5f0 < 0) {
LAB_00108b00:
      uVar3 = 2;
      goto LAB_00108b05;
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
LAB_00108b9a:
    local_5f0 = iVar19 - uVar13;
    if (local_5f0 < 0) goto LAB_00108b00;
    local_598[1] = 0;
    *(int *)(local_5e8 + (ulong)uVar22 * 4) = iVar19;
  }
LAB_00108bbd:
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
LAB_00108beb:
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
                lVar4 = alStack_550[1];
                if (local_63c != 0) {
                  while (lVar4 != 0) {
                    lVar24 = *(long *)(lVar4 + -8);
                    free((void *)(lVar4 + -0x10));
                    lVar4 = lVar24;
                  }
                }
                goto LAB_00108ff6;
              }
              DAT_001dca08 = uVar13 + 1 + DAT_001dca08;
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
          if (local_598[local_63c] != ((1 << (bVar2 & 0x1f)) - 1U & uVar22)) {
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
LAB_00108b05:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
  while( true ) {
    iVar19 = iVar19 - *(int *)(puVar17 + 4);
    puVar17 = puVar17 + 4;
    if (iVar19 < 0) break;
LAB_00108b89:
    iVar19 = iVar19 * 2;
    if (local_5e8 + (((uVar22 - local_640) - 1) + uVar7) * 4 == puVar17) {
      uVar13 = *(uint *)(local_5e8 + (ulong)uVar22 * 4);
      goto LAB_00108b9a;
    }
  }
  goto LAB_00108b00;
}




// Function: inflate_fixed @ 0x90c0

int inflate_fixed(void)

{
  void *__ptr;
  long lVar1;
  int iVar2;
  undefined8 *puVar3;
  long in_FS_OFFSET;
  undefined4 local_4c0;
  undefined4 local_4bc;
  long local_4b8;
  long local_4b0;
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
  } while (puVar3 != local_a8);
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
  iVar2 = FUN_001089b0(local_4a8,0x120,0x101,&DAT_001181a0,&DAT_00118160,&local_4b8,&local_4c0);
  if (iVar2 == 0) {
    puVar3 = local_4a8;
    do {
      *(undefined4 *)puVar3 = 5;
      puVar3 = (undefined8 *)((long)puVar3 + 4);
    } while (puVar3 != local_4a8 + 0xf);
    local_4bc = 5;
    iVar2 = FUN_001089b0(local_4a8,0x1e,0,&DAT_00118120,&DAT_001180e0,&local_4b0,&local_4bc);
    if (iVar2 < 2) {
      iVar2 = FUN_00108460(local_4b8,local_4b0,local_4c0,local_4bc);
      if (iVar2 == 0) {
        while (local_4b8 != 0) {
          lVar1 = *(long *)(local_4b8 + -8);
          free((void *)(local_4b8 + -0x10));
          local_4b8 = lVar1;
        }
        iVar2 = 0;
        lVar1 = local_4b0;
        while (lVar1 != 0) {
          __ptr = (void *)(lVar1 + -0x10);
          lVar1 = *(long *)(lVar1 + -8);
          free(__ptr);
        }
      }
      else {
        iVar2 = 1;
      }
    }
    else {
      while (local_4b8 != 0) {
        lVar1 = *(long *)(local_4b8 + -8);
        free((void *)(local_4b8 + -0x10));
        local_4b8 = lVar1;
      }
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return iVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: inflate_dynamic @ 0x92b0

uint inflate_dynamic(void)

{
  void *__ptr;
  ushort uVar1;
  ushort uVar2;
  long lVar3;
  long lVar4;
  char *pcVar5;
  char *pcVar6;
  undefined4 uVar7;
  byte bVar8;
  byte bVar9;
  uint uVar10;
  int iVar11;
  uint uVar12;
  ulong uVar13;
  ulong uVar14;
  uint uVar15;
  uint *puVar16;
  uint uVar17;
  uint uVar18;
  uint uVar19;
  uint uVar20;
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
  
  uVar13 = DAT_001dca10;
  uVar19 = DAT_001dca0c;
  uVar7 = DAT_0011a004;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar17 = DAT_001dca0c;
  if (DAT_001dca0c < 5) {
    uVar14 = (ulong)DAT_0011a008;
    if (DAT_0011a008 < DAT_0011a00c) {
      DAT_0011a008 = DAT_0011a008 + 1;
      bVar8 = (&DAT_0019c000)[uVar14];
    }
    else {
      bVar8 = FUN_0010d6c0(0);
    }
    uVar17 = uVar19 + 8;
    uVar13 = uVar13 | (ulong)bVar8 << ((byte)uVar19 & 0x3f);
  }
  uVar19 = uVar17 - 5;
  uVar14 = uVar13 >> 5;
  uVar10 = ((uint)uVar13 & 0x1f) + 0x101;
  uVar12 = uVar19;
  if (uVar19 < 5) {
    uVar13 = (ulong)DAT_0011a008;
    if (DAT_0011a008 < DAT_0011a00c) {
      DAT_0011a008 = DAT_0011a008 + 1;
      bVar8 = (&DAT_0019c000)[uVar13];
    }
    else {
      DAT_0011a004 = uVar7;
      bVar8 = FUN_0010d6c0(0);
    }
    uVar12 = uVar17 + 3;
    uVar14 = uVar14 | (ulong)bVar8 << ((byte)uVar19 & 0x3f);
  }
  uVar18 = uVar12 - 5;
  uVar13 = uVar14 >> 5;
  uVar17 = ((uint)uVar14 & 0x1f) + 1;
  uVar19 = uVar18;
  if (uVar18 < 4) {
    uVar14 = (ulong)DAT_0011a008;
    if (DAT_0011a008 < DAT_0011a00c) {
      DAT_0011a008 = DAT_0011a008 + 1;
      bVar8 = (&DAT_0019c000)[uVar14];
    }
    else {
      DAT_0011a004 = uVar7;
      bVar8 = FUN_0010d6c0(0);
    }
    uVar19 = uVar12 + 3;
    uVar13 = uVar13 | (ulong)bVar8 << ((byte)uVar18 & 0x3f);
  }
  if ((uVar10 < 0x11f) && (uVar17 < 0x1f)) {
    uVar19 = uVar19 - 4;
    puVar16 = &DAT_001133c0;
    uVar18 = (uint)uVar13 & 0xf;
    uVar12 = uVar18 + 4;
    uVar13 = uVar13 >> 4;
    do {
      uVar20 = uVar19;
      if (uVar19 < 3) {
        uVar14 = (ulong)DAT_0011a008;
        if (DAT_0011a008 < DAT_0011a00c) {
          DAT_0011a008 = DAT_0011a008 + 1;
          bVar8 = (&DAT_0019c000)[uVar14];
        }
        else {
          DAT_0011a004 = uVar7;
          bVar8 = FUN_0010d6c0(0);
        }
        uVar20 = uVar19 + 8;
        uVar13 = uVar13 | (ulong)bVar8 << ((byte)uVar19 & 0x3f);
      }
      uVar15 = *puVar16;
      puVar16 = puVar16 + 1;
      uVar14 = uVar13 >> 3;
      uVar19 = uVar20 - 3;
      local_538[uVar15] = (uint)uVar13 & 7;
      uVar13 = uVar14;
    } while ((uint *)(&DAT_001133d0 + (ulong)uVar18 * 4) != puVar16);
    for (; uVar12 != 0x13; uVar12 = uVar12 + 1) {
      local_538[(uint)(&DAT_001133c0)[uVar12]] = 0;
    }
    local_550 = 7;
    uVar18 = FUN_001089b0(local_538,0x13,0x13,0,0,&local_548,&local_550);
    uVar12 = local_550;
    if (uVar18 == 0) {
      if (local_548 == 0) goto LAB_0010991a;
      local_57c = 0;
      uVar20 = uVar10 + uVar17;
      local_588 = 0;
      uVar1 = *(ushort *)(&DAT_00113380 + (long)(int)local_550 * 2);
      uVar18 = 0;
      do {
        for (; uVar19 < uVar12; uVar19 = uVar19 + 8) {
          uVar13 = (ulong)DAT_0011a008;
          if (DAT_0011a008 < DAT_0011a00c) {
            DAT_0011a008 = DAT_0011a008 + 1;
            bVar8 = (&DAT_0019c000)[uVar13];
          }
          else {
            DAT_0011a004 = uVar7;
            bVar8 = FUN_0010d6c0(0);
          }
          uVar14 = uVar14 | (ulong)bVar8 << ((byte)uVar19 & 0x3f);
        }
        local_540 = (char *)((ulong)((uint)uVar1 & (uint)uVar14) * 0x10 + local_548);
        uVar13 = uVar14 >> (local_540[1] & 0x3fU);
        uVar19 = uVar19 - (byte)local_540[1];
        if (*local_540 == 'c') goto joined_r0x001098fd;
        uVar2 = *(ushort *)(local_540 + 8);
        uVar15 = (uint)uVar2;
        if (uVar2 < 0x10) {
          local_588 = uVar18 + 1;
          local_538[(int)uVar18] = uVar15;
          uVar14 = uVar13;
          local_57c = uVar15;
        }
        else {
          bVar8 = (byte)uVar19;
          if (uVar2 == 0x10) {
            if (uVar19 < 2) {
              uVar14 = (ulong)DAT_0011a008;
              if (DAT_0011a008 < DAT_0011a00c) {
                DAT_0011a008 = DAT_0011a008 + 1;
                bVar9 = (&DAT_0019c000)[uVar14];
              }
              else {
                DAT_0011a004 = uVar7;
                bVar9 = FUN_0010d6c0(0);
              }
              uVar19 = uVar19 + 8;
              uVar13 = uVar13 | (ulong)bVar9 << (bVar8 & 0x3f);
            }
            uVar14 = uVar13 >> 2;
            uVar19 = uVar19 - 2;
            uVar15 = (uint)uVar13 & 3;
            if (uVar20 < uVar15 + 3 + local_588) goto LAB_001096f8;
            puVar16 = local_538 + (int)uVar18;
            do {
              *puVar16 = local_57c;
              puVar16 = puVar16 + 1;
            } while (local_538 + (ulong)uVar15 + (long)(int)uVar18 + 3 != puVar16);
            local_588 = uVar15 + 3 + uVar18;
          }
          else if (uVar15 == 0x11) {
            if (uVar19 < 3) {
              uVar14 = (ulong)DAT_0011a008;
              if (DAT_0011a008 < DAT_0011a00c) {
                DAT_0011a008 = DAT_0011a008 + 1;
                bVar9 = (&DAT_0019c000)[uVar14];
              }
              else {
                DAT_0011a004 = uVar7;
                bVar9 = FUN_0010d6c0(0);
              }
              uVar19 = uVar19 + 8;
              uVar13 = uVar13 | (ulong)bVar9 << (bVar8 & 0x3f);
            }
            uVar14 = uVar13 >> 3;
            uVar19 = uVar19 - 3;
            uVar15 = (uint)uVar13 & 7;
            if (uVar20 < uVar15 + 3 + local_588) goto LAB_001096f8;
            puVar16 = local_538 + (int)uVar18;
            do {
              *puVar16 = 0;
              puVar16 = puVar16 + 1;
            } while (local_538 + (ulong)uVar15 + (long)(int)uVar18 + 3 != puVar16);
            local_57c = 0;
            local_588 = uVar15 + 3 + uVar18;
          }
          else {
            if (uVar19 < 7) {
              uVar14 = (ulong)DAT_0011a008;
              if (DAT_0011a008 < DAT_0011a00c) {
                DAT_0011a008 = DAT_0011a008 + 1;
                bVar9 = (&DAT_0019c000)[uVar14];
              }
              else {
                DAT_0011a004 = uVar7;
                bVar9 = FUN_0010d6c0(0);
              }
              uVar19 = uVar19 + 8;
              uVar13 = uVar13 | (ulong)bVar9 << (bVar8 & 0x3f);
            }
            uVar14 = uVar13 >> 7;
            uVar19 = uVar19 - 7;
            uVar15 = (uint)uVar13 & 0x7f;
            if (uVar20 < uVar15 + 0xb + local_588) goto LAB_001096f8;
            puVar16 = local_538 + (int)uVar18;
            do {
              *puVar16 = 0;
              puVar16 = puVar16 + 1;
            } while (auStack_50c + (ulong)uVar15 + (long)(int)uVar18 != puVar16);
            local_57c = 0;
            local_588 = uVar15 + 0xb + uVar18;
          }
        }
        uVar18 = local_588;
        lVar4 = local_548;
      } while (local_588 < uVar20);
      while (lVar4 != 0) {
        lVar3 = *(long *)(lVar4 + -8);
        free((void *)(lVar4 + -0x10));
        lVar4 = lVar3;
      }
      local_550 = 9;
      DAT_001dca0c = uVar19;
      DAT_001dca10 = uVar14;
      uVar18 = FUN_001089b0(local_538,uVar10,0x101,&DAT_001181a0,&DAT_00118160,&local_548,&local_550
                           );
      if (uVar18 == 0) {
        local_54c = 6;
        uVar18 = FUN_001089b0(local_538 + uVar10,uVar17,0,&DAT_00118120,&DAT_001180e0,&local_540,
                              &local_54c);
        if (uVar18 == 0) {
          iVar11 = FUN_00108460(local_548,local_540,local_550,local_54c);
          uVar18 = (uint)(iVar11 != 0);
          while (pcVar6 = local_540, local_548 != 0) {
            __ptr = (void *)(local_548 + -0x10);
            local_548 = *(long *)(local_548 + -8);
            free(__ptr);
          }
          while (pcVar6 != (char *)0x0) {
            pcVar5 = *(char **)(pcVar6 + -8);
            free(pcVar6 + -0x10);
            pcVar6 = pcVar5;
          }
        }
        else {
          pcVar6 = local_540;
          if (uVar18 == 1) {
            while (pcVar6 != (char *)0x0) {
              pcVar5 = *(char **)(pcVar6 + -8);
              free(pcVar6 + -0x10);
              pcVar6 = pcVar5;
            }
          }
          while (local_548 != 0) {
            lVar4 = *(long *)(local_548 + -8);
            free((void *)(local_548 + -0x10));
            local_548 = lVar4;
          }
        }
        goto LAB_001095d0;
      }
      if (uVar18 != 1) goto LAB_001095d0;
      while (local_548 != 0) {
        lVar4 = *(long *)(local_548 + -8);
        free((void *)(local_548 + -0x10));
        local_548 = lVar4;
      }
    }
    else {
      if (uVar18 != 1) goto LAB_001095d0;
      while (local_548 != 0) {
        lVar4 = *(long *)(local_548 + -8);
        free((void *)(local_548 + -0x10));
        local_548 = lVar4;
      }
    }
  }
LAB_001096f8:
  uVar18 = 1;
LAB_001095d0:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar18;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
joined_r0x001098fd:
  while (local_548 != 0) {
    lVar4 = *(long *)(local_548 + -8);
    free((void *)(local_548 + -0x10));
    local_548 = lVar4;
  }
LAB_0010991a:
  uVar18 = 2;
  goto LAB_001095d0;
}




// Function: inflate @ 0x9a50

undefined8 inflate(void)

{
  byte bVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  ulong uVar5;
  ulong uVar6;
  undefined8 uVar7;
  uint uVar8;
  int iVar9;
  ulong uVar10;
  uint uVar11;
  uint uVar12;
  uint uVar13;
  bool bVar14;
  
  DAT_0011a004 = 0;
  DAT_001dca0c = 0;
  DAT_001dca10 = 0;
  do {
    uVar5 = DAT_001dca10;
    uVar13 = DAT_001dca0c;
    DAT_001dca08 = 0;
    uVar11 = (uint)DAT_001dca10;
    if (DAT_001dca0c == 0) {
      uVar10 = (ulong)DAT_0011a008;
      if (DAT_0011a008 < DAT_0011a00c) {
        DAT_0011a008 = DAT_0011a008 + 1;
        bVar1 = (&DAT_0019c000)[uVar10];
      }
      else {
        bVar1 = FUN_0010d6c0(0);
      }
      uVar13 = 5;
      uVar5 = (bVar1 | uVar5) >> 1;
      uVar11 = (bVar1 | uVar11) & 1;
    }
    else {
      uVar12 = DAT_001dca0c - 1;
      uVar5 = DAT_001dca10 >> 1;
      uVar11 = uVar11 & 1;
      if (uVar12 < 2) {
        uVar10 = (ulong)DAT_0011a008;
        if (DAT_0011a008 < DAT_0011a00c) {
          DAT_0011a008 = DAT_0011a008 + 1;
          bVar1 = (&DAT_0019c000)[uVar10];
        }
        else {
          bVar1 = FUN_0010d6c0(0);
        }
        uVar13 = uVar13 + 5;
        uVar5 = uVar5 | (ulong)bVar1 << ((byte)uVar12 & 0x3f);
      }
      else {
        uVar13 = DAT_001dca0c - 3;
      }
    }
    uVar12 = DAT_0011a004;
    DAT_001dca10 = uVar5 >> 2;
    uVar2 = (uint)uVar5 & 3;
    DAT_001dca0c = uVar13;
    if (uVar2 == 2) {
      uVar7 = FUN_001092b0();
LAB_00109d95:
      uVar5 = DAT_001dca10;
      uVar13 = DAT_001dca0c;
      if ((int)uVar7 != 0) {
        return uVar7;
      }
    }
    else {
      if ((uVar5 & 3) != 0) {
        if (uVar2 != 1) {
          return 2;
        }
        uVar7 = FUN_001090c0();
        goto LAB_00109d95;
      }
      uVar3 = uVar13 & 7;
      uVar10 = DAT_001dca10 >> (sbyte)uVar3;
      uVar2 = uVar13 - uVar3;
      uVar4 = uVar2;
      if (uVar2 < 0x10) {
        do {
          while( true ) {
            uVar5 = (ulong)DAT_0011a008;
            if (DAT_0011a008 < DAT_0011a00c) break;
            DAT_0011a004 = uVar12;
            bVar1 = FUN_0010d6c0(0);
            uVar8 = uVar4 + 8;
            uVar10 = uVar10 | (ulong)bVar1 << ((byte)uVar4 & 0x3f);
            uVar4 = uVar8;
            if (0xf < uVar8) goto LAB_00109c01;
          }
          DAT_0011a008 = DAT_0011a008 + 1;
          uVar8 = uVar4 + 8;
          uVar10 = uVar10 | (ulong)(byte)(&DAT_0019c000)[uVar5] << ((byte)uVar4 & 0x3f);
          uVar4 = uVar8;
        } while (uVar8 < 0x10);
LAB_00109c01:
        uVar2 = uVar2 + 8 + ((uVar3 - uVar13) + 0xf & 0xfffffff8);
      }
      uVar4 = (uint)uVar10 & 0xffff;
      uVar5 = uVar10 >> 0x10;
      uVar13 = uVar2 - 0x10;
      if (uVar13 < 0x10) {
        do {
          uVar6 = (ulong)DAT_0011a008;
          if (DAT_0011a008 < DAT_0011a00c) {
            DAT_0011a008 = DAT_0011a008 + 1;
            bVar1 = (&DAT_0019c000)[uVar6];
          }
          else {
            DAT_0011a004 = uVar12;
            bVar1 = FUN_0010d6c0(0);
          }
          uVar3 = uVar13 + 8;
          uVar5 = uVar5 | (ulong)bVar1 << ((byte)uVar13 & 0x3f);
          uVar13 = uVar3;
        } while (uVar3 < 0x10);
        uVar13 = (uVar2 - 8) + (0x1f - uVar2 & 0xfffffff8);
      }
      if (uVar4 != (~(uint)uVar5 & 0xffff)) {
        return 1;
      }
      uVar5 = uVar5 >> 0x10;
      uVar13 = uVar13 - 0x10;
      iVar9 = uVar4 - 1;
      DAT_0011a004 = uVar12;
      if ((uVar10 & 0xffff) != 0) {
        do {
          while( true ) {
            if (uVar13 < 8) {
              uVar10 = (ulong)DAT_0011a008;
              if (DAT_0011a008 < DAT_0011a00c) {
                DAT_0011a008 = DAT_0011a008 + 1;
                bVar1 = (&DAT_0019c000)[uVar10];
              }
              else {
                DAT_0011a004 = uVar12;
                bVar1 = FUN_0010d6c0(0);
              }
              uVar5 = uVar5 | (ulong)bVar1 << ((byte)uVar13 & 0x3f);
            }
            else {
              uVar13 = uVar13 - 8;
            }
            uVar10 = (ulong)uVar12;
            uVar12 = uVar12 + 1;
            (&DAT_0013b000)[uVar10] = (char)uVar5;
            if (uVar12 == 0x8000) break;
            uVar5 = uVar5 >> 8;
            bVar14 = iVar9 == 0;
            iVar9 = iVar9 + -1;
            DAT_0011a004 = uVar12;
            if (bVar14) goto joined_r0x00109d53;
          }
          DAT_0011a004 = 0x8000;
          uVar12 = 0;
          uVar5 = uVar5 >> 8;
          FUN_0010d640();
          bVar14 = iVar9 != 0;
          iVar9 = iVar9 + -1;
          DAT_0011a004 = uVar12;
        } while (bVar14);
      }
    }
joined_r0x00109d53:
    DAT_001dca0c = uVar13;
    DAT_001dca10 = uVar5;
    if (uVar11 != 0) {
      if (7 < DAT_001dca0c) {
        uVar13 = DAT_001dca0c - 8;
        DAT_001dca0c = DAT_001dca0c & 7;
        DAT_0011a008 = DAT_0011a008 + ~(uVar13 >> 3);
      }
      FUN_0010d640();
      return 0;
    }
  } while( true );
}




// Function: init_block @ 0x9df0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void init_block(void)

{
  undefined2 *puVar1;
  undefined2 *puVar2;
  
  puVar2 = &DAT_001df080;
  do {
    puVar1 = puVar2 + 2;
    *puVar2 = 0;
    puVar2 = puVar1;
  } while (puVar1 != (undefined2 *)&DAT_001df4f8);
  puVar2 = &DAT_001def80;
  do {
    puVar1 = puVar2 + 2;
    *puVar2 = 0;
    puVar2 = puVar1;
  } while (puVar1 != (undefined2 *)&DAT_001deff8);
  puVar2 = &DAT_001de9e0;
  do {
    puVar1 = puVar2 + 2;
    *puVar2 = 0;
    puVar2 = puVar1;
  } while (puVar1 != (undefined2 *)&DAT_001dea2c);
  DAT_001dca38 = 0;
  _DAT_001df480 = 1;
  DAT_001dca40 = 0;
  DAT_001dca4c = 0;
  DAT_001dca50 = 0;
  DAT_001dca54 = 0;
  DAT_001dca49 = 0;
  DAT_001dca48 = 1;
  return;
}




// Function: pqdownheap @ 0x9ea0

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
  
  iVar7 = DAT_001de0a4;
  lVar12 = (long)param_2;
  param_2 = param_2 * 2;
  iVar4 = *(int *)(&DAT_001de0c0 + lVar12 * 4);
  if (param_2 <= DAT_001de0a4) {
    uVar1 = *(ushort *)(param_1 + (long)iVar4 * 4);
    do {
      iVar5 = *(int *)(&DAT_001de0c0 + (long)param_2 * 4);
      uVar2 = *(ushort *)(param_1 + (long)iVar5 * 4);
      iVar9 = param_2;
      uVar11 = uVar2;
      iVar10 = iVar5;
      if (param_2 < iVar7) {
        iVar8 = param_2 + 1;
        iVar6 = *(int *)(&DAT_001de0c0 + (long)iVar8 * 4);
        uVar3 = *(ushort *)(param_1 + (long)iVar6 * 4);
        iVar9 = iVar8;
        uVar11 = uVar3;
        iVar10 = iVar6;
        if (((uVar2 <= uVar3) && (iVar9 = param_2, uVar11 = uVar2, iVar10 = iVar5, uVar3 == uVar2))
           && (iVar9 = iVar8, iVar10 = iVar6,
              (byte)(&DAT_001dde60)[iVar5] < (byte)(&DAT_001dde60)[iVar6])) {
          iVar9 = param_2;
          iVar10 = iVar5;
        }
      }
      if ((uVar1 < uVar11) ||
         ((uVar1 == uVar11 && ((byte)(&DAT_001dde60)[iVar4] <= (byte)(&DAT_001dde60)[iVar10]))))
      break;
      param_2 = iVar9 * 2;
      *(int *)(&DAT_001de0c0 + lVar12 * 4) = iVar10;
      lVar12 = (long)iVar9;
    } while (param_2 <= iVar7);
  }
  *(int *)(&DAT_001de0c0 + lVar12 * 4) = iVar4;
  return;
}




// Function: scan_tree @ 0x9f80

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
LAB_00109fd0:
        if (uVar5 == 0) {
          if (iVar3 < 0xb) {
            DAT_001dea24 = DAT_001dea24 + 1;
          }
          else {
            DAT_001dea28 = DAT_001dea28 + 1;
          }
        }
        else {
          if (uVar5 != uVar7) {
            (&DAT_001de9e0)[(long)(int)uVar5 * 2] = (&DAT_001de9e0)[(long)(int)uVar5 * 2] + 1;
          }
          DAT_001dea20 = DAT_001dea20 + 1;
        }
        if (uVar6 == 0) {
LAB_0010a050:
          iVar9 = 3;
          iVar8 = 0x8a;
          iVar3 = 0;
          uVar7 = uVar5;
          goto LAB_0010a005;
        }
        if (uVar6 == uVar5) {
          iVar9 = 3;
          iVar8 = 6;
          iVar3 = 0;
          uVar7 = uVar6;
          goto LAB_0010a005;
        }
LAB_0010a030:
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
        if (iVar9 <= iVar3) goto LAB_00109fd0;
        (&DAT_001de9e0)[(long)(int)uVar5 * 2] = (&DAT_001de9e0)[(long)(int)uVar5 * 2] + (short)iVar3
        ;
        if (uVar2 != 0) goto LAB_0010a030;
        goto LAB_0010a050;
      }
LAB_0010a005:
      puVar4 = puVar4 + 2;
      uVar5 = uVar6;
    } while (puVar1 != puVar4);
  }
  return;
}




// Function: build_tree @ 0xa090

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void build_tree(undefined8 *param_1)

{
  ushort *puVar1;
  byte bVar2;
  short sVar3;
  int iVar4;
  int iVar5;
  bool bVar6;
  bool bVar7;
  uint uVar8;
  short sVar9;
  undefined2 uVar10;
  ulong uVar11;
  long lVar12;
  ulong uVar13;
  long lVar14;
  uint uVar15;
  long lVar16;
  undefined2 *puVar17;
  undefined2 *puVar18;
  undefined2 *puVar19;
  int iVar20;
  ushort uVar21;
  uint uVar22;
  long lVar23;
  int *piVar24;
  uint uVar25;
  int iVar26;
  int iVar27;
  long lVar28;
  int iVar29;
  long in_FS_OFFSET;
  short local_98;
  int local_84;
  short asStack_68 [20];
  long local_40;
  
  puVar19 = (undefined2 *)*param_1;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  lVar28 = param_1[1];
  _DAT_001de0a0 = 0x23d;
  iVar26 = *(int *)((long)param_1 + 0x1c);
  if (iVar26 < 1) {
    local_84 = -1;
    DAT_001de0a4 = 0;
  }
  else {
    bVar6 = false;
    uVar11 = 0;
    DAT_001de0a4 = 0;
    uVar13 = 0xffffffff;
    do {
      while (local_84 = (int)uVar13, puVar19[uVar11 * 2] == 0) {
        puVar19[uVar11 * 2 + 1] = 0;
        uVar11 = uVar11 + 1;
        if (uVar11 == (long)iVar26) goto LAB_0010a136;
      }
      DAT_001de0a4 = DAT_001de0a4 + 1;
      (&DAT_001dde60)[uVar11] = 0;
      local_84 = (int)uVar11;
      uVar13 = uVar11 & 0xffffffff;
      *(int *)(&DAT_001de0c0 + (long)DAT_001de0a4 * 4) = local_84;
      uVar11 = uVar11 + 1;
      bVar6 = true;
    } while (uVar11 != (long)iVar26);
LAB_0010a136:
    if (bVar6) {
      if (1 < DAT_001de0a4) goto LAB_0010a215;
    }
    else {
      DAT_001de0a4 = 0;
    }
  }
  lVar16 = (long)DAT_001de0a4;
  bVar6 = false;
  lVar23 = DAT_001dca38;
  do {
    if (local_84 < 2) {
      local_84 = local_84 + 1;
      lVar12 = (long)local_84;
      lVar14 = (long)local_84 * 4;
      puVar17 = puVar19 + (long)local_84 * 2;
    }
    else {
      lVar14 = 0;
      lVar12 = 0;
      puVar17 = puVar19;
    }
    (&DAT_001de0c4)[lVar16] = (int)lVar12;
    *puVar17 = 1;
    (&DAT_001dde60)[lVar12] = 0;
    if (lVar28 != 0) {
      bVar6 = true;
      lVar23 = lVar23 - (ulong)*(ushort *)(lVar28 + 2 + lVar14);
    }
    lVar16 = lVar16 + 1;
  } while ((int)lVar16 < 2);
  uVar15 = 0;
  if (DAT_001de0a4 < 2) {
    uVar15 = 1U - DAT_001de0a4;
  }
  iVar4 = DAT_001de0a4 + 1 + uVar15;
  lVar16 = -(ulong)(1U - DAT_001de0a4);
  if (1 < DAT_001de0a4) {
    lVar16 = 0;
  }
  DAT_001dca40 = DAT_001dca40 + -1 + lVar16;
  DAT_001de0a4 = iVar4;
  if (bVar6) {
    DAT_001dca38 = lVar23;
  }
LAB_0010a215:
  iVar4 = DAT_001de0a4;
  lVar23 = (long)iVar26;
  *(int *)((long)param_1 + 0x24) = local_84;
  iVar27 = iVar4 >> 1;
  do {
    FUN_00109ea0(puVar19,iVar27);
    iVar27 = iVar27 + -1;
  } while (iVar27 != 0);
  lVar16 = (long)iVar4;
  local_98 = (short)iVar26;
  lVar12 = 0x23b;
  do {
    iVar26 = DAT_001de0c4;
    DAT_001de0a4 = (int)lVar16 + -1;
    DAT_001de0c4 = *(int *)(&DAT_001de0c0 + lVar16 * 4);
    FUN_00109ea0(puVar19,1);
    _DAT_001de0a0 = (int)lVar12;
    (&DAT_001de0c4)[lVar12] = iVar26;
    sVar9 = puVar19[(long)DAT_001de0c4 * 2];
    sVar3 = puVar19[(long)iVar26 * 2];
    *(int *)(&DAT_001de0c0 + lVar12 * 4) = DAT_001de0c4;
    puVar19[lVar23 * 2] = sVar9 + sVar3;
    bVar2 = (&DAT_001dde60)[DAT_001de0c4];
    if ((byte)(&DAT_001dde60)[DAT_001de0c4] <= (byte)(&DAT_001dde60)[iVar26]) {
      bVar2 = (&DAT_001dde60)[iVar26];
    }
    lVar12 = lVar12 + -2;
    (&DAT_001dde60)[lVar23] = bVar2 + 1;
    sVar9 = (local_98 + (short)iVar4) - (short)lVar16;
    lVar16 = lVar16 + -1;
    (puVar19 + (long)DAT_001de0c4 * 2)[1] = sVar9;
    (puVar19 + (long)iVar26 * 2)[1] = sVar9;
    DAT_001de0c4 = (int)lVar23;
    lVar23 = lVar23 + 1;
    FUN_00109ea0(puVar19,1);
  } while (1 < (int)lVar16);
  lVar23 = (long)DAT_001de0c4;
  iVar26 = iVar4 * -2 + 0x23f;
  if (iVar4 < 2) {
    iVar26 = 0x23b;
  }
  _DAT_001de0a0 = iVar4 * -2 + 0x241;
  if (iVar4 < 2) {
    _DAT_001de0a0 = 0x23d;
  }
  _DAT_001de0a0 = _DAT_001de0a0 + -3;
  *(int *)(&DAT_001de0c0 + (long)_DAT_001de0a0 * 4) = DAT_001de0c4;
  lVar16 = param_1[2];
  iVar4 = *(int *)((long)param_1 + 0x24);
  uVar15 = *(uint *)(param_1 + 4);
  iVar27 = *(int *)(param_1 + 3);
  puVar17 = &DAT_001de9c0;
  do {
    puVar18 = puVar17 + 1;
    *puVar17 = 0;
    puVar17 = puVar18;
  } while (puVar18 != &DAT_001de9e0);
  puVar19[lVar23 * 2 + 1] = 0;
  if (iVar26 < 0x23d) {
    bVar7 = false;
    iVar29 = 0;
    bVar6 = false;
    piVar24 = (int *)(&DAT_001de0c0 + (long)iVar26 * 4);
    lVar23 = DAT_001dca40;
    lVar12 = DAT_001dca38;
    do {
      iVar5 = *piVar24;
      puVar1 = puVar19 + (long)iVar5 * 2;
      uVar25 = (ushort)puVar19[(ulong)puVar1[1] * 2 + 1] + 1;
      if ((int)uVar15 < (int)((ushort)puVar19[(ulong)puVar1[1] * 2 + 1] + 1)) {
        iVar29 = iVar29 + 1;
        uVar25 = uVar15;
      }
      puVar1[1] = (ushort)uVar25;
      if (iVar5 <= iVar4) {
        lVar14 = (long)(int)uVar25;
        iVar20 = 0;
        (&DAT_001de9c0)[lVar14] = (&DAT_001de9c0)[lVar14] + 1;
        if (iVar27 <= iVar5) {
          iVar20 = *(int *)(lVar16 + (long)(iVar5 - iVar27) * 4);
          lVar14 = (long)(int)(uVar25 + iVar20);
        }
        lVar23 = lVar23 + lVar14 * (ulong)*puVar1;
        bVar6 = true;
        if (lVar28 != 0) {
          bVar7 = true;
          lVar12 = lVar12 + (long)(int)((uint)*(ushort *)(lVar28 + 2 + (long)iVar5 * 4) + iVar20) *
                            (ulong)*puVar1;
        }
      }
      piVar24 = piVar24 + 1;
    } while (&DAT_001de0c4 + (ulong)(0x23c - iVar26) + (long)iVar26 != piVar24);
    if (bVar7) {
      DAT_001dca38 = lVar12;
    }
    if (bVar6) {
      DAT_001dca40 = lVar23;
    }
    if (iVar29 != 0) {
      iVar26 = iVar29 + -2;
      uVar25 = uVar15 - 1;
      lVar28 = (long)(int)uVar15;
      iVar27 = iVar26 - (iVar29 - 1U & 0xfffffffe);
      sVar9 = (&DAT_001de9c0)[(int)uVar25];
      uVar22 = uVar15;
      puVar17 = &DAT_001de9c0 + lVar28;
      uVar8 = uVar25;
      while( true ) {
        while (sVar9 == 0) {
          sVar9 = puVar17[-2];
          uVar22 = uVar8;
          puVar17 = puVar17 + -1;
          uVar8 = uVar8 - 1;
        }
        (&DAT_001de9c0)[(int)uVar8] = sVar9 + -1;
        (&DAT_001de9c0)[(int)uVar22] = (&DAT_001de9c0)[(int)uVar22] + 2;
        uVar21 = (&DAT_001de9c0)[lVar28] - 1;
        (&DAT_001de9c0)[lVar28] = uVar21;
        if (iVar26 == iVar27) break;
        sVar9 = (&DAT_001de9c0)[(int)uVar25];
        iVar26 = iVar26 + -2;
        uVar22 = uVar15;
        puVar17 = &DAT_001de9c0 + lVar28;
        uVar8 = uVar25;
      }
      if (uVar15 != 0) {
        bVar6 = false;
        iVar26 = 0x23d;
        lVar23 = DAT_001dca40;
        while( true ) {
          uVar22 = (uint)uVar21;
          if (uVar22 != 0) {
            lVar16 = (long)(iVar26 + -1);
            do {
              while( true ) {
                iVar26 = (int)lVar16;
                if (*(int *)(&DAT_001de0c0 + lVar16 * 4) <= iVar4) break;
                lVar16 = lVar16 + -1;
              }
              puVar1 = puVar19 + (long)*(int *)(&DAT_001de0c0 + lVar16 * 4) * 2;
              uVar21 = puVar1[1];
              if (uVar21 != uVar15) {
                puVar1[1] = (ushort)uVar15;
                bVar6 = true;
                lVar23 = lVar23 + (ulong)*puVar1 * (lVar28 - (ulong)uVar21);
              }
              lVar16 = lVar16 + -1;
              uVar22 = uVar22 - 1;
            } while (uVar22 != 0);
          }
          lVar28 = lVar28 + -1;
          if (uVar25 == 0) break;
          uVar21 = (&DAT_001de9c0)[lVar28];
          uVar15 = uVar25;
          uVar25 = uVar25 - 1;
        }
        if (bVar6) {
          DAT_001dca40 = lVar23;
        }
      }
    }
  }
  lVar28 = 0;
  sVar9 = 0;
  do {
    sVar9 = (sVar9 + *(short *)((long)&DAT_001de9c0 + lVar28)) * 2;
    *(short *)((long)asStack_68 + lVar28 + 2) = sVar9;
    lVar28 = lVar28 + 2;
  } while (lVar28 != 0x1e);
  if (local_84 != -1) {
    puVar17 = puVar19 + (long)local_84 * 2 + 2;
    do {
      uVar21 = puVar19[1];
      if (uVar21 != 0) {
        asStack_68[(int)(uint)uVar21] = asStack_68[(int)(uint)uVar21] + 1;
        uVar10 = FUN_001044a0();
        *puVar19 = uVar10;
      }
      puVar19 = puVar19 + 2;
    } while (puVar17 != puVar19);
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: send_tree @ 0xa6b0

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
          if (iVar6 <= iVar9) goto LAB_0010a780;
          do {
            FUN_001043b0((&DAT_001de9e0)[(long)(int)uVar4 * 2],
                         *(undefined2 *)(&DAT_001de9e2 + (long)(int)uVar4 * 4));
            iVar9 = iVar9 + -1;
          } while (iVar9 != 0);
          if (uVar1 != 0) goto LAB_0010a74e;
          goto LAB_0010a7e8;
        }
      }
      else {
LAB_0010a780:
        if (uVar4 == 0) {
          if (iVar9 < 0xb) {
            FUN_001043b0(DAT_001dea24,DAT_001dea26);
            FUN_001043b0(iVar3 + -2,3);
          }
          else {
            FUN_001043b0(DAT_001dea28,DAT_001dea2a);
            FUN_001043b0(iVar3 + -10,7);
          }
        }
        else {
          if (uVar7 != uVar4) {
            FUN_001043b0((&DAT_001de9e0)[(long)(int)uVar4 * 2],
                         *(undefined2 *)(&DAT_001de9e2 + (long)(int)uVar4 * 4));
            iVar9 = iVar3;
          }
          FUN_001043b0(DAT_001dea20,DAT_001dea22);
          FUN_001043b0(iVar9 + -3,2);
        }
        if (uVar1 == 0) {
LAB_0010a7e8:
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
LAB_0010a74e:
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




// Function: compress_block @ 0xa870

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
  
  if (DAT_001dca54 != 0) {
    bVar6 = 0;
    uVar8 = 0;
    uVar5 = 0;
    uVar9 = 0;
    while( true ) {
      uVar10 = uVar9;
      if ((uVar5 & 7) == 0) {
        uVar10 = (ulong)((int)uVar9 + 1);
        bVar6 = (&DAT_001dca60)[uVar9];
      }
      uVar7 = (int)uVar5 + 1;
      bVar2 = (&DAT_0019c000)[uVar5];
      if ((bVar6 & 1) == 0) {
        puVar1 = (undefined2 *)(param_1 + (ulong)bVar2 * 4);
        FUN_001043b0(*puVar1,puVar1[1]);
      }
      else {
        bVar3 = (&DAT_001ddd60)[bVar2];
        puVar1 = (undefined2 *)(param_1 + (ulong)(bVar3 + 0x101) * 4);
        FUN_001043b0(*puVar1,puVar1[1]);
        if ((&DAT_001182c0)[bVar3] != 0) {
          FUN_001043b0((uint)bVar2 - (&DAT_001ddae0)[bVar3]);
        }
        uVar4 = (&DAT_0014b000)[uVar8];
        if (uVar4 < 0x100) {
          bVar2 = (&DAT_001ddb60)[(uint)uVar4];
        }
        else {
          bVar2 = (&DAT_001ddb60)[(uVar4 >> 7) + 0x100];
        }
        uVar5 = (ulong)bVar2;
        puVar1 = (undefined2 *)(param_2 + uVar5 * 4);
        FUN_001043b0(*puVar1,puVar1[1]);
        uVar8 = (ulong)((int)uVar8 + 1);
        if ((&DAT_00118240)[uVar5] != 0) {
          FUN_001043b0((uint)uVar4 - (&DAT_001dda60)[uVar5]);
        }
      }
      if (DAT_001dca54 <= uVar7) break;
      bVar6 = bVar6 >> 1;
      uVar5 = (ulong)uVar7;
      uVar9 = uVar10;
    }
  }
  FUN_001043b0(*(undefined2 *)(param_1 + 0x400),*(undefined2 *)(param_1 + 0x402));
  return;
}




// Function: ct_init @ 0xaa20

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void ct_init(undefined8 param_1)

{
  ushort uVar1;
  undefined2 uVar2;
  undefined1 *puVar3;
  undefined2 *puVar4;
  undefined2 *puVar5;
  undefined2 *puVar6;
  ulong uVar7;
  short sVar8;
  long lVar9;
  int iVar10;
  int iVar11;
  long in_FS_OFFSET;
  short asStack_58 [20];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_001dca20 = param_1;
  _DAT_001dca28 = 0;
  DAT_001dca30 = 0;
  if (DAT_001dea82 == 0) {
    lVar9 = 0;
    iVar10 = 0;
    do {
      (&DAT_001ddae0)[lVar9] = iVar10;
      iVar11 = 1 << ((byte)(&DAT_001182c0)[lVar9] & 0x1f);
      if (0 < iVar11) {
        puVar3 = &DAT_001ddd60 + iVar10;
        do {
          *puVar3 = (char)lVar9;
          puVar3 = puVar3 + 1;
        } while (puVar3 != &DAT_001ddd61 + (ulong)(iVar11 - 1) + (long)iVar10);
        iVar10 = iVar10 + iVar11;
      }
      lVar9 = lVar9 + 1;
    } while (lVar9 != 0x1c);
    lVar9 = 0;
    iVar11 = 0;
    (&DAT_001ddd60)[iVar10 + -1] = 0x1c;
    do {
      (&DAT_001dda60)[lVar9] = iVar11;
      iVar10 = 1 << ((byte)(&DAT_00118240)[lVar9] & 0x1f);
      if (0 < iVar10) {
        puVar3 = &DAT_001ddb60 + iVar11;
        do {
          *puVar3 = (char)lVar9;
          puVar3 = puVar3 + 1;
        } while (puVar3 != &DAT_001ddb61 + (ulong)(iVar10 - 1) + (long)iVar11);
        iVar11 = iVar11 + iVar10;
      }
      lVar9 = lVar9 + 1;
    } while (lVar9 != 0x10);
    iVar11 = iVar11 >> 7;
    lVar9 = 0x10;
    do {
      (&DAT_001dda60)[lVar9] = iVar11 << 7;
      iVar10 = 1 << ((char)(&DAT_00118240)[lVar9] - 7U & 0x1f);
      if (0 < iVar10) {
        puVar3 = &DAT_001ddb60 + iVar11;
        do {
          puVar3[0x100] = (char)lVar9;
          puVar3 = puVar3 + 1;
        } while (puVar3 != &DAT_001ddb61 + (ulong)(iVar10 - 1) + (long)iVar11);
        iVar11 = iVar11 + iVar10;
      }
      lVar9 = lVar9 + 1;
    } while (lVar9 != 0x1e);
    puVar4 = &DAT_001de9c0;
    do {
      puVar6 = puVar4 + 1;
      *puVar4 = 0;
      puVar4 = puVar6;
    } while (puVar6 != &DAT_001de9e0);
    puVar4 = &DAT_001deb00;
    puVar6 = &DAT_001deb02;
    do {
      puVar5 = puVar6 + 2;
      *puVar6 = 8;
      puVar6 = puVar5;
    } while (puVar5 != &DAT_001ded42);
    puVar6 = &DAT_001ded42;
    do {
      puVar5 = puVar6 + 2;
      *puVar6 = 9;
      puVar6 = puVar5;
    } while (puVar5 != &DAT_001def02);
    DAT_001de9d2 = DAT_001de9d2 + 0x70;
    puVar6 = &DAT_001def02;
    do {
      puVar5 = puVar6 + 2;
      *puVar6 = 7;
      puVar6 = puVar5;
    } while (puVar5 != (undefined2 *)&DAT_001def62);
    DAT_001de9ce = DAT_001de9ce + 0x18;
    _DAT_001def66 = 8;
    _DAT_001def6a = 8;
    _DAT_001def6e = 8;
    _DAT_001def72 = 8;
    _DAT_001def76 = 8;
    _DAT_001def7a = 8;
    _DAT_001def7e = 8;
    DAT_001de9d0 = DAT_001de9d0 + 0x98;
    _DAT_001def62 = 8;
    lVar9 = 0;
    sVar8 = 0;
    do {
      sVar8 = (sVar8 + *(short *)((long)&DAT_001de9c0 + lVar9)) * 2;
      *(short *)((long)asStack_58 + lVar9 + 2) = sVar8;
      lVar9 = lVar9 + 2;
    } while (lVar9 != 0x1e);
    do {
      uVar1 = puVar4[1];
      if (uVar1 != 0) {
        asStack_58[(int)(uint)uVar1] = asStack_58[(int)(uint)uVar1] + 1;
        uVar2 = FUN_001044a0();
        *puVar4 = uVar2;
      }
      puVar4 = puVar4 + 2;
    } while (puVar4 != &DAT_001def80);
    uVar7 = 0;
    do {
      (&DAT_001dea82)[uVar7 * 2] = 5;
      uVar2 = FUN_001044a0(uVar7 & 0xffffffff,5);
      (&DAT_001dea80)[uVar7 * 2] = uVar2;
      uVar7 = uVar7 + 1;
    } while (uVar7 != 0x1e);
    if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
      FUN_00109df0();
      return;
    }
  }
  else if (*(long *)(in_FS_OFFSET + 0x28) == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: flush_block @ 0xadd0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

long flush_block(long param_1,ulong param_2,int param_3,int param_4)

{
  ushort uVar1;
  undefined4 uVar2;
  int iVar3;
  ushort *puVar4;
  long lVar5;
  ulong uVar6;
  ushort *puVar7;
  uint uVar8;
  uint uVar9;
  ulong uVar10;
  ulong uVar11;
  long lVar12;
  undefined *puVar13;
  
  puVar4 = DAT_001dca20;
  uVar1 = *DAT_001dca20;
  (&DAT_001dca60)[DAT_001dca4c] = DAT_001dca49;
  if (uVar1 == 0xffff) {
    puVar7 = &DAT_001df080;
    uVar9 = 0;
    do {
      uVar1 = *puVar7;
      puVar7 = puVar7 + 2;
      uVar9 = uVar9 + uVar1;
    } while (puVar7 != &DAT_001df09c);
    puVar7 = &DAT_001df09c;
    uVar8 = 0;
    do {
      uVar1 = *puVar7;
      puVar7 = puVar7 + 2;
      uVar8 = uVar8 + uVar1;
    } while (puVar7 != &DAT_001df280);
    puVar7 = &DAT_001df280;
    do {
      uVar1 = *puVar7;
      puVar7 = puVar7 + 2;
      uVar9 = uVar9 + uVar1;
    } while (puVar7 != (ushort *)&DAT_001df480);
    *puVar4 = (ushort)(uVar9 <= uVar8 >> 2);
  }
  FUN_0010a090(&PTR_DAT_001183c0);
  FUN_0010a090(&PTR_DAT_00118380);
  FUN_00109f80(&DAT_001df080,DAT_001183e4);
  FUN_00109f80(&DAT_001def80,DAT_001183a4);
  FUN_0010a090(&PTR_DAT_00118340);
  uVar10 = 0xf;
  lVar5 = 0x12;
  do {
    lVar12 = (long)(int)lVar5;
    if (*(short *)(&DAT_001de9e2 + uVar10 * 4) != 0) {
      lVar5 = (long)((int)lVar5 * 3 + 0x11);
LAB_0010aeb0:
      DAT_001dca40 = lVar5 + DAT_001dca40;
      uVar6 = DAT_001dca40 + 10U >> 3;
      uVar11 = DAT_001dca38 + 10U >> 3;
      _DAT_001dca28 = _DAT_001dca28 + param_2;
      uVar10 = uVar11;
      if (uVar6 <= uVar11) {
        uVar10 = uVar6;
      }
      if ((uVar10 < param_2 + 4) || (param_1 == 0)) {
        if (uVar6 < uVar11) {
          FUN_001043b0(param_4 + 4,3);
          iVar3 = DAT_001183e4;
          uVar2 = DAT_001183a4;
          FUN_001043b0(DAT_001183e4 + -0x100,5);
          FUN_001043b0(uVar2,5);
          FUN_001043b0((int)lVar12 + -3,4);
          uVar10 = 0x10;
          for (puVar13 = &DAT_00113410; FUN_001043b0(*(undefined2 *)(&DAT_001de9e2 + uVar10 * 4),3),
              &DAT_00113410 + lVar12 != puVar13; puVar13 = puVar13 + 1) {
            uVar10 = (ulong)(byte)puVar13[1];
          }
          FUN_0010a6b0(&DAT_001df080,iVar3);
          FUN_0010a6b0(&DAT_001def80,uVar2);
          FUN_0010a870(&DAT_001df080,&DAT_001def80);
          DAT_001dca30 = DAT_001dca30 + DAT_001dca40 + 3;
        }
        else {
          FUN_001043b0(param_4 + 2,3);
          FUN_0010a870(&DAT_001deb00,&DAT_001dea80);
          DAT_001dca30 = DAT_001dca30 + DAT_001dca38 + 3;
        }
        FUN_00109df0();
      }
      else {
        FUN_001043b0(param_4,3);
        DAT_001dca30 = (DAT_001dca30 + 10 & 0xfffffffffffffff8) + (param_2 + 4) * 8;
        FUN_001045c0(param_1,param_2 & 0xffffffff,1);
        FUN_00109df0();
      }
      if (param_4 == 0) {
        if ((param_3 != 0) && ((DAT_001dca30 & 7) != 0)) {
          FUN_001043b0(0,3);
          DAT_001dca30 = DAT_001dca30 + 10 & 0xfffffffffffffff8;
          FUN_001045c0(param_1,0,1);
        }
      }
      else {
        FUN_001044d0();
        DAT_001dca30 = DAT_001dca30 + 7;
      }
      return (long)DAT_001dca30 >> 3;
    }
    if (lVar5 + -1 == 2) {
      lVar5 = 0x17;
      lVar12 = 2;
      goto LAB_0010aeb0;
    }
    uVar10 = (ulong)(byte)(&UNK_0011340f)[lVar5];
    lVar5 = lVar5 + -1;
  } while( true );
}




// Function: ct_tally @ 0xb160

bool ct_tally(int param_1,int param_2)

{
  int *piVar1;
  ushort *puVar2;
  byte bVar3;
  ulong uVar4;
  long lVar5;
  uint uVar6;
  
  uVar4 = (ulong)DAT_001dca54;
  uVar6 = DAT_001dca54 + 1;
  DAT_001dca54 = uVar6;
  (&DAT_0019c000)[uVar4] = (char)param_2;
  if (param_1 == 0) {
    (&DAT_001df080)[(long)param_2 * 2] = (&DAT_001df080)[(long)param_2 * 2] + 1;
  }
  else {
    param_1 = param_1 + -1;
    *(short *)(&DAT_001df484 + (ulong)(byte)(&DAT_001ddd60)[param_2] * 4) =
         *(short *)(&DAT_001df484 + (ulong)(byte)(&DAT_001ddd60)[param_2] * 4) + 1;
    if (param_1 < 0x100) {
      bVar3 = (&DAT_001ddb60)[param_1];
    }
    else {
      bVar3 = (&DAT_001ddb60)[(param_1 >> 7) + 0x100];
    }
    (&DAT_001def80)[(ulong)bVar3 * 2] = (&DAT_001def80)[(ulong)bVar3 * 2] + 1;
    uVar4 = (ulong)DAT_001dca50;
    DAT_001dca50 = DAT_001dca50 + 1;
    (&DAT_0014b000)[uVar4] = (short)param_1;
    DAT_001dca49 = DAT_001dca49 | DAT_001dca48;
  }
  bVar3 = DAT_001dca49;
  DAT_001dca48 = DAT_001dca48 * '\x02';
  if ((uVar6 & 7) == 0) {
    uVar4 = (ulong)DAT_001dca4c;
    DAT_001dca48 = '\x01';
    DAT_001dca49 = 0;
    DAT_001dca4c = DAT_001dca4c + 1;
    (&DAT_001dca60)[uVar4] = bVar3;
  }
  if ((2 < DAT_0011801c) && ((uVar6 & 0xfff) == 0)) {
    lVar5 = 0;
    uVar4 = (ulong)uVar6 << 3;
    do {
      piVar1 = (int *)((long)&DAT_00118240 + lVar5);
      puVar2 = (ushort *)((long)&DAT_001def80 + lVar5);
      lVar5 = lVar5 + 4;
      uVar4 = uVar4 + ((long)*piVar1 + 5) * (ulong)*puVar2;
    } while (lVar5 != 0x78);
    if ((DAT_001dca50 < uVar6 >> 1) && (uVar4 >> 3 < (ulong)DAT_0011906c - DAT_00119078 >> 1)) {
      return true;
    }
  }
  return uVar6 == 0x7fff || DAT_001dca50 == 0x8000;
}




// Function: fillbuf @ 0xb320

void fillbuf(int param_1)

{
  ulong uVar1;
  int iVar2;
  ushort uVar3;
  uint uVar4;
  
  uVar4 = (uint)DAT_001df994 << ((byte)param_1 & 0x1f);
  uVar3 = (ushort)uVar4;
  iVar2 = DAT_001df98c;
  if (DAT_001df98c < param_1) {
    do {
      param_1 = param_1 - DAT_001df98c;
      uVar4 = uVar4 | DAT_001df990 << ((byte)param_1 & 0x1f);
      uVar1 = (ulong)DAT_0011a008;
      DAT_001df994 = (ushort)uVar4;
      if (DAT_0011a008 < DAT_0011a00c) {
        DAT_0011a008 = DAT_0011a008 + 1;
        DAT_001df990 = (uint)(byte)(&DAT_0019c000)[uVar1];
      }
      else {
        DAT_001df990 = FUN_0010d6c0(1);
        if (DAT_001df990 == 0xffffffff) {
          DAT_001df990 = 0;
          uVar4 = (uint)DAT_001df994;
        }
        else {
          uVar4 = (uint)DAT_001df994;
        }
      }
      uVar3 = (ushort)uVar4;
      DAT_001df98c = 8;
      iVar2 = 8;
    } while (8 < param_1);
  }
  DAT_001df98c = iVar2 - param_1;
  DAT_001df994 = uVar3 | (ushort)(DAT_001df990 >> ((byte)(iVar2 - param_1) & 0x1f));
  return;
}




// Function: make_table @ 0xb3f0

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
                (&DAT_0011a880)[uVar12] = 0;
                (&DAT_0012a880)[uVar12] = 0;
                uVar10 = uVar12 & 0xffff;
                *puVar9 = (ushort)uVar12;
                uVar12 = (ulong)((int)uVar12 + 1);
              }
              uVar13 = (uint)uVar14;
              if ((1 << (0xf - bVar1 & 0x1f) & uVar13) != 0) break;
              puVar9 = &DAT_0011a880 + uVar10;
              uVar14 = (ulong)(uVar13 * 2);
              iVar16 = iVar16 + -1;
              if (iVar16 == 0) goto LAB_0010b6b2;
            }
            puVar9 = &DAT_0012a880 + uVar10;
            uVar14 = (ulong)(uVar13 * 2);
          }
LAB_0010b6b2:
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
  FUN_0010d3f0("Bad table\n");
}




// Function: read_pt_len @ 0xb700

void read_pt_len(int param_1,undefined4 param_2,int param_3)

{
  int iVar1;
  undefined1 *puVar2;
  long lVar3;
  undefined4 *puVar4;
  byte bVar5;
  ulong uVar6;
  long lVar7;
  int iVar8;
  uint uVar9;
  int iVar10;
  byte *pbVar11;
  byte *pbVar12;
  int iVar13;
  
  bVar5 = 0x10 - (char)param_2;
  iVar8 = (int)(uint)DAT_001df994 >> (bVar5 & 0x1f);
  FUN_0010b320(param_2);
  if (iVar8 == 0) {
    uVar9 = (uint)DAT_001df994;
    FUN_0010b320(param_2);
    puVar2 = &DAT_001dfbc0;
    do {
      *puVar2 = 0;
      puVar2 = puVar2 + 1;
    } while (puVar2 != &DAT_001dfbc1 + (param_1 - 1));
    puVar4 = (undefined4 *)&DAT_001df9a0;
    do {
      *(short *)puVar4 = (short)((int)uVar9 >> (bVar5 & 0x1f));
      puVar4 = (undefined4 *)((long)puVar4 + 2);
    } while (puVar4 != &DAT_001dfba0);
    return;
  }
  iVar13 = 0;
  do {
    uVar9 = (param_3 + -1) - iVar13;
    lVar3 = (long)iVar13;
    pbVar12 = &DAT_001dfbc0 + iVar13;
    do {
      if (iVar8 <= iVar13) {
        if (iVar13 < param_1) {
          puVar2 = &DAT_001dfbc0 + iVar13;
          do {
            *puVar2 = 0;
            puVar2 = puVar2 + 1;
          } while (puVar2 != &DAT_001dfbc1 + (ulong)(uint)((param_1 + -1) - iVar13) + (long)iVar13);
        }
        FUN_0010b3f0(param_1,&DAT_001dfbc0,8,&DAT_001df9a0);
        return;
      }
      iVar10 = 3;
      bVar5 = (byte)(DAT_001df994 >> 0xd);
      if (DAT_001df994 >> 0xd == 7) {
        if ((DAT_001df994 & 0x1000) == 0) {
          bVar5 = 7;
          iVar10 = 4;
        }
        else {
          uVar6 = 0x1000;
          iVar1 = 7;
          do {
            iVar10 = iVar1;
            uVar6 = uVar6 >> 1;
            iVar1 = iVar10 + 1;
          } while (((uint)DAT_001df994 & (uint)uVar6) != 0);
          if (0x10 < iVar1) {
                    /* WARNING: Subroutine does not return */
            FUN_0010d3f0("Bad table\n");
          }
          iVar10 = iVar10 + -2;
          bVar5 = (byte)iVar1;
        }
      }
      FUN_0010b320(iVar10);
      pbVar11 = pbVar12 + 1;
      *pbVar12 = bVar5;
      iVar13 = iVar13 + 1;
      pbVar12 = pbVar11;
    } while (pbVar11 != &DAT_001dfbc1 + (ulong)uVar9 + lVar3);
    iVar10 = (int)(uint)DAT_001df994 >> 0xe;
    FUN_0010b320(2);
    if (iVar10 != 0) {
      lVar3 = (long)iVar13;
      lVar7 = iVar10 + lVar3;
      do {
        (&DAT_001dfbc0)[lVar3] = 0;
        lVar3 = lVar3 + 1;
      } while (lVar3 != lVar7);
      iVar13 = iVar13 + iVar10;
    }
  } while( true );
}




// Function: unlzh @ 0xb920

undefined8 unlzh(undefined4 param_1,undefined4 param_2)

{
  ushort *puVar1;
  ushort uVar2;
  bool bVar3;
  byte bVar4;
  uint uVar5;
  ulong uVar6;
  uint uVar7;
  int iVar8;
  undefined1 *puVar9;
  undefined2 *puVar10;
  ulong uVar11;
  int iVar12;
  int iVar13;
  ulong uVar14;
  int iVar15;
  long lVar16;
  
  DAT_001df994 = 0;
  DAT_001df990 = 0;
  DAT_001df98c = 0;
  DAT_0011a010 = param_2;
  DAT_0011a014 = param_1;
  FUN_0010b320();
  DAT_001dfba0 = 0;
  DAT_001df988 = 0;
  DAT_001df984 = 0;
LAB_0010b98e:
  uVar6 = 0;
  bVar3 = false;
  uVar5 = DAT_001df980;
  do {
    uVar14 = uVar6 & 0xffffffff;
    DAT_001df988 = DAT_001df988 + -1;
    if (DAT_001df988 < 0) {
      if (bVar3) {
        DAT_001df980 = uVar5;
      }
      goto LAB_0010b9f0;
    }
    uVar14 = (ulong)uVar5;
    uVar5 = uVar5 + 1 & 0x1fff;
    (&DAT_0013b000)[uVar6] = (&DAT_0013b000)[uVar14];
    uVar6 = uVar6 + 1;
    bVar3 = true;
  } while (uVar6 != 0x2000);
  uVar14 = 0x2000;
  goto LAB_0010ba98;
  while( true ) {
    uVar7 = iVar13 + 1;
    (&DAT_0013b000)[uVar14] = (char)uVar6;
    uVar14 = (ulong)uVar7;
    uVar5 = DAT_001df980;
    if (uVar7 == 0x2000) break;
LAB_0010b9f0:
    uVar5 = (uint)DAT_001df994;
    iVar13 = (int)uVar14;
    if (DAT_001dfba0 == 0) {
      FUN_0010b320(0x10);
      DAT_001dfba0 = uVar5;
      if (uVar5 == 0) {
        DAT_001df984 = 1;
        uVar5 = DAT_001df980;
        if (iVar13 == 0) {
          DAT_001df984 = 1;
          DAT_001dfba0 = 0;
          return 0;
        }
        break;
      }
      FUN_0010b700(0x13,5,3);
      iVar12 = 0;
      iVar15 = (int)(uint)DAT_001df994 >> 7;
      FUN_0010b320(9);
      if (iVar15 == 0) {
        uVar5 = (uint)DAT_001df994;
        FUN_0010b320(9);
        puVar9 = &DAT_0015b000;
        do {
          *puVar9 = 0;
          puVar9 = puVar9 + 1;
        } while (puVar9 != &DAT_0015b1fe);
        puVar10 = &DAT_0014b000;
        do {
          *puVar10 = (short)((int)uVar5 >> 7);
          puVar10 = puVar10 + 1;
        } while (puVar10 != (undefined2 *)&DAT_0014d000);
      }
      else {
        do {
          while( true ) {
            uVar6 = (ulong)(ushort)(&DAT_001df9a0)[(byte)(DAT_001df994 >> 8)];
            if (0x12 < (ushort)(&DAT_001df9a0)[(byte)(DAT_001df994 >> 8)]) {
              uVar5 = 0x80;
              do {
                if ((uVar5 & DAT_001df994) == 0) {
                  uVar2 = (&DAT_0011a880)[uVar6];
                }
                else {
                  uVar2 = (&DAT_0011a880)[(int)uVar6 + 0x8000];
                }
                uVar6 = (ulong)uVar2;
                uVar5 = uVar5 >> 1;
              } while (0x12 < uVar2);
            }
            uVar5 = (uint)uVar6;
            lVar16 = (long)(int)uVar5;
            FUN_0010b320((&DAT_001dfbc0)[lVar16]);
            if (uVar5 < 3) break;
            lVar16 = (long)iVar12;
            iVar12 = iVar12 + 1;
            (&DAT_0015b000)[lVar16] = (char)uVar6 + -2;
            if (iVar15 <= iVar12) goto LAB_0010bc2a;
          }
          iVar8 = 0;
          if (uVar5 != 0) {
            uVar7 = (uint)DAT_001df994;
            if (uVar5 == 1) {
              FUN_0010b320(4);
              iVar8 = ((int)uVar7 >> 0xc) + 2;
              lVar16 = (long)iVar8;
            }
            else {
              FUN_0010b320(9);
              iVar8 = ((int)uVar7 >> 7) + 0x13;
              lVar16 = (long)iVar8;
            }
          }
          puVar9 = &DAT_0015b000 + iVar12;
          do {
            *puVar9 = 0;
            puVar9 = puVar9 + 1;
          } while (&DAT_0015b001 + lVar16 + iVar12 != puVar9);
          iVar12 = iVar8 + 1 + iVar12;
        } while (iVar12 < iVar15);
LAB_0010bc2a:
        if (iVar12 < 0x1fe) {
          puVar9 = &DAT_0015b000 + iVar12;
          do {
            *puVar9 = 0;
            puVar9 = puVar9 + 1;
          } while (puVar9 != &DAT_0015b001 + (ulong)(0x1fd - iVar12) + (long)iVar12);
        }
        FUN_0010b3f0(0x1fe,&DAT_0015b000,0xc,&DAT_0014b000);
      }
      FUN_0010b700(0xe,4,0xffffffff);
      uVar5 = (uint)DAT_001df994;
    }
    DAT_001dfba0 = DAT_001dfba0 - 1;
    uVar6 = (ulong)(ushort)(&DAT_0014b000)[uVar5 >> 4];
    if (0x1fd < (ushort)(&DAT_0014b000)[uVar5 >> 4]) {
      uVar7 = 8;
      do {
        while ((uVar7 & uVar5) == 0) {
          puVar1 = &DAT_0011a880 + uVar6;
          uVar6 = (ulong)*puVar1;
          uVar7 = uVar7 >> 1;
          if (*puVar1 < 0x1fe) goto LAB_0010ba66;
        }
        iVar12 = (int)uVar6;
        uVar7 = uVar7 >> 1;
        uVar6 = (ulong)(ushort)(&DAT_0011a880)[iVar12 + 0x8000];
      } while (0x1fd < (ushort)(&DAT_0011a880)[iVar12 + 0x8000]);
    }
LAB_0010ba66:
    FUN_0010b320((&DAT_0015b000)[uVar6]);
    if (0xff < (uint)uVar6) {
      DAT_001df988 = (uint)uVar6 - 0xfd;
      uVar6 = (ulong)(ushort)(&DAT_001df9a0)[(byte)(DAT_001df994 >> 8)];
      if (0xd < (ushort)(&DAT_001df9a0)[(byte)(DAT_001df994 >> 8)]) {
        uVar5 = 0x80;
        do {
          while ((uVar5 & DAT_001df994) == 0) {
            puVar1 = &DAT_0011a880 + uVar6;
            uVar6 = (ulong)*puVar1;
            uVar5 = uVar5 >> 1;
            if (*puVar1 < 0xe) goto LAB_0010bd20;
          }
          iVar12 = (int)uVar6;
          uVar5 = uVar5 >> 1;
          uVar6 = (ulong)(ushort)(&DAT_0011a880)[iVar12 + 0x8000];
        } while (0xd < (ushort)(&DAT_0011a880)[iVar12 + 0x8000]);
      }
LAB_0010bd20:
      FUN_0010b320((&DAT_001dfbc0)[uVar6]);
      iVar12 = 0;
      if ((int)uVar6 != 0) {
        iVar12 = (int)uVar6 + -1;
        uVar5 = (uint)DAT_001df994;
        bVar4 = (byte)iVar12;
        FUN_0010b320(iVar12);
        iVar12 = ((int)uVar5 >> (0x10 - bVar4 & 0x1f)) + (1 << (bVar4 & 0x1f));
      }
      bVar3 = false;
      uVar5 = (iVar13 + -1) - iVar12 & 0x1fff;
      uVar6 = uVar14;
      DAT_001df980 = uVar5;
      while (DAT_001df988 = DAT_001df988 + -1, -1 < DAT_001df988) {
        uVar11 = (ulong)uVar5;
        uVar7 = (int)uVar6 + 1;
        uVar14 = (ulong)uVar7;
        uVar5 = uVar5 + 1 & 0x1fff;
        (&DAT_0013b000)[uVar6] = (&DAT_0013b000)[uVar11];
        bVar3 = true;
        uVar6 = uVar14;
        if (uVar7 == 0x2000) goto LAB_0010ba98;
      }
      uVar14 = uVar6;
      if (bVar3) {
        DAT_001df980 = uVar5;
      }
      goto LAB_0010b9f0;
    }
  }
LAB_0010ba98:
  DAT_001df980 = uVar5;
  FUN_0010d5a0(param_2,&DAT_0013b000,uVar14);
  if (DAT_001df984 != 0) {
    return 0;
  }
  goto LAB_0010b98e;
}




// Function: unlzw @ 0xbe90

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
  uVar4 = (ulong)DAT_0011a008;
  if (DAT_0011a008 < DAT_0011a00c) {
    DAT_0011a008 = DAT_0011a008 + 1;
    DAT_00118024 = (uint)(byte)(&DAT_0019c000)[uVar4];
  }
  else {
    DAT_00118024 = FUN_0010d6c0(0);
  }
  DAT_001183e8 = DAT_00118024 & 0x80;
  if ((DAT_00118024 & 0x60) != 0) {
    if (DAT_0011a85c == 0) {
      FUN_0010f6b0(stderr,"\n%s: %s: warning, unknown flags 0x%x\n",DAT_0011a850,&DAT_0011a420);
    }
    if (DAT_0011a84c == 0) {
      DAT_0011a84c = 2;
    }
  }
  DAT_00118024 = DAT_00118024 & 0x1f;
  lVar2 = 1L << (sbyte)DAT_00118024;
  if (0x10 < DAT_00118024) {
    FUN_0010f6b0(stderr,"\n%s: %s: compressed with %d bits, can only handle %d bits\n",DAT_0011a850,
                 &DAT_0011a420,DAT_00118024,0x10);
    DAT_0011a84c = 1;
    return 1;
  }
  _DAT_0011a880 = 0;
  uVar4 = (ulong)DAT_0011a00c;
  _DAT_0011a978 = 0;
  uVar13 = (ulong)(DAT_0011a008 * 8);
  local_a0 = 0x101 - (ulong)(DAT_001183e8 == 0);
  local_44 = DAT_0011a00c;
  puVar10 = &DAT_0011a888;
  for (lVar6 = 0x1f; lVar6 != 0; lVar6 = lVar6 + -1) {
    *puVar10 = 0;
    puVar10 = puVar10 + (ulong)bVar18 * -2 + 1;
  }
  lVar6 = 0xff;
  do {
    (&DAT_0013b000)[lVar6] = (char)lVar6;
    bVar17 = lVar6 != 0;
    lVar6 = lVar6 + -1;
  } while (bVar17);
  local_48 = 9;
  iVar14 = 0;
  local_88 = 0x1ff;
  local_40 = 0x1ff;
  local_8c = 0;
  uVar15 = 0xffffffffffffffff;
LAB_0010bfec:
  uVar12 = (uint)((long)uVar13 >> 3);
  uVar8 = (uint)uVar4;
  uVar16 = uVar15;
  if (uVar12 <= uVar8) goto LAB_0010c258;
LAB_0010bff9:
  DAT_0011a00c = 0;
  puVar9 = &DAT_0019c000;
LAB_0010c00a:
  local_44 = FUN_0010d190(param_1,puVar9,0x40000);
  if (local_44 == 0xffffffff) {
                    /* WARNING: Subroutine does not return */
    FUN_0010d4e0();
  }
  DAT_0011a828 = DAT_0011a828 + (int)local_44;
  DAT_0011a00c = DAT_0011a00c + local_44;
  uVar4 = (ulong)DAT_0011a00c;
LAB_0010c049:
  uVar12 = local_48;
  uVar8 = (uint)uVar4;
  uVar15 = uVar16;
  if (local_44 == 0) {
    local_70 = uVar4 * 8 - (long)(int)(local_48 - 1);
    if (local_70 < 1) {
LAB_0010c3f5:
      if (iVar14 < 1) {
        return 0;
      }
      FUN_0010d5a0(param_2,&DAT_0015b000,iVar14);
      return 0;
    }
  }
  else {
    uVar13 = 0;
    local_70 = (uVar4 - uVar4 % (ulong)local_48) * 8;
    if (local_70 < 1) goto LAB_0010bfec;
  }
  if (local_88 < local_a0) {
    lVar6 = -1;
    goto LAB_0010c1ef;
  }
  uVar13 = 0;
LAB_0010c0a8:
  lVar6 = (long)uVar13 >> 3;
  bVar1 = (byte)uVar13;
  uVar13 = uVar13 + (long)(int)local_48;
  uVar16 = (long)((ulong)(byte)(&DAT_0019c001)[lVar6] << 8 |
                  (ulong)(byte)(&DAT_0019c002)[lVar6] << 0x10 | (ulong)(byte)(&DAT_0019c000)[lVar6])
           >> (bVar1 & 7) & (ulong)local_40;
  if (uVar15 == 0xffffffffffffffff) {
    if (0xff < uVar16) {
                    /* WARNING: Subroutine does not return */
      FUN_0010d3f0("corrupt input.");
    }
    lVar6 = (long)iVar14;
    local_8c = (uint)uVar16;
    iVar14 = iVar14 + 1;
    (&DAT_0015b000)[lVar6] = (char)uVar16;
    uVar8 = DAT_0011a00c;
  }
  else {
    if (uVar16 == 0x100) {
      if (DAT_001183e8 != 0) {
        _DAT_0011a880 = 0;
        _DAT_0011a978 = 0;
        puVar10 = &DAT_0011a888;
        for (lVar6 = 0x1f; lVar6 != 0; lVar6 = lVar6 + -1) {
          *puVar10 = 0;
          puVar10 = puVar10 + (ulong)bVar18 * -2 + 1;
        }
        local_40 = 0x1ff;
        local_a0 = 0x100;
        uVar4 = (ulong)DAT_0011a00c;
        local_48 = 9;
        local_88 = 0x1ff;
        lVar6 = (long)(int)(uVar12 << 3);
        uVar13 = (uVar13 - 1) + (lVar6 - (long)((uVar13 - 1) + lVar6) % lVar6);
        goto LAB_0010bfec;
      }
      uVar4 = 0x100;
      puVar9 = &DAT_0015affe;
      if (local_a0 != 0x100) goto LAB_0010c138;
LAB_0010c118:
      puVar9 = &DAT_0015affd;
      DAT_0015affd = (undefined1)local_8c;
      uVar4 = uVar15;
    }
    else {
      if (local_a0 <= (long)uVar16) {
        if (local_a0 < (long)uVar16) {
          if (0 < iVar14) {
            FUN_0010d5a0(param_2,&DAT_0015b000,iVar14);
          }
          pcVar11 = "corrupt input.";
          if (DAT_0011a860 == 0) {
            pcVar11 = "corrupt input. Use zcat to recover some data.";
          }
                    /* WARNING: Subroutine does not return */
          FUN_0010d3f0(pcVar11);
        }
        goto LAB_0010c118;
      }
      puVar9 = &DAT_0015affe;
      uVar4 = uVar16;
    }
    for (; 0xff < (long)uVar4; uVar4 = (ulong)(ushort)(&DAT_0011a880)[uVar4]) {
LAB_0010c138:
      puVar9 = puVar9 + -1;
      *puVar9 = (&DAT_0013b000)[uVar4];
    }
    bVar1 = (&DAT_0013b000)[uVar4];
    puVar3 = puVar9 + -1;
    puVar9[-1] = bVar1;
    lVar6 = (long)&DAT_0015affe - (long)puVar3;
    iVar5 = iVar14 + (int)lVar6;
    local_8c = (uint)bVar1;
    if (iVar5 < 0x40000) {
      memcpy(&DAT_0015b000 + iVar14,puVar3,(long)(int)lVar6);
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
          memcpy(&DAT_0015b000 + lVar6,puVar3,(long)iVar5);
        }
        if (0x3ffff < iVar14) {
          FUN_0010d5a0(param_2,&DAT_0015b000,iVar14);
          iVar14 = 0;
        }
        puVar3 = puVar3 + iVar5;
        lVar6 = (long)&DAT_0015affe - (long)puVar3;
      } while (0 < (int)lVar6);
    }
    uVar8 = DAT_0011a00c;
    if (local_a0 < lVar2) {
      local_98 = (undefined2)uVar15;
      (&DAT_0011a880)[local_a0] = local_98;
      (&DAT_0013b000)[local_a0] = bVar1;
      local_a0 = local_a0 + 1;
      uVar8 = DAT_0011a00c;
    }
  }
  uVar15 = uVar16;
  DAT_0011a00c = uVar8;
  if (local_70 <= (long)uVar13) {
    if (local_44 == 0) goto LAB_0010c3f5;
    uVar4 = (ulong)uVar8;
    goto LAB_0010bfec;
  }
  if (local_88 < local_a0) goto code_r0x0010c1e5;
  goto LAB_0010c0a8;
code_r0x0010c1e5:
  lVar6 = uVar13 - 1;
LAB_0010c1ef:
  iVar5 = local_48 * 8;
  local_48 = local_48 + 1;
  lVar7 = (long)iVar5;
  local_88 = lVar2;
  if (DAT_00118024 != local_48) {
    local_88 = (1L << ((byte)local_48 & 0x3f)) + -1;
  }
  local_40 = (1 << ((byte)local_48 & 0x1f)) - 1;
  uVar12 = (uint)(lVar6 + (lVar7 - (lVar7 + lVar6) % lVar7) >> 3);
  if (uVar8 < uVar12) goto LAB_0010bff9;
LAB_0010c258:
  DAT_0011a00c = uVar8 - uVar12;
  uVar4 = (ulong)DAT_0011a00c;
  if (0 < (int)DAT_0011a00c) {
    puVar9 = &DAT_0019c000;
    do {
      puVar3 = puVar9 + 1;
      *puVar9 = puVar9[(int)uVar12];
      puVar9 = puVar3;
    } while (&DAT_0019c001 + (DAT_0011a00c - 1) != puVar3);
  }
  if (DAT_0011a00c < 0x40) goto code_r0x0010c297;
  goto LAB_0010c049;
code_r0x0010c297:
  puVar9 = &DAT_0019c000 + uVar4;
  goto LAB_0010c00a;
}




// Function: unpack @ 0xc590

undefined8 unpack(undefined4 param_1,undefined4 param_2)

{
  long lVar1;
  byte bVar2;
  char cVar3;
  uint uVar4;
  uint uVar5;
  ulong uVar6;
  ulong uVar7;
  char *pcVar8;
  ulong uVar9;
  uint uVar10;
  char *pcVar11;
  uint uVar12;
  int iVar13;
  uint uVar14;
  long lVar15;
  long lVar16;
  long lVar17;
  int iVar18;
  long lVar19;
  
  iVar13 = 4;
  uVar9 = (ulong)DAT_0011a008;
  DAT_001dfe88 = 0;
  DAT_0011a010 = param_2;
  DAT_0011a014 = param_1;
  uVar4 = DAT_0011a00c;
  do {
    uVar6 = DAT_001dfe88 << 8;
    if ((uint)uVar9 < uVar4) {
      DAT_0011a008 = (uint)uVar9 + 1;
      uVar7 = (ulong)(byte)(&DAT_0019c000)[uVar9];
    }
    else {
      uVar7 = FUN_0010d6c0(0);
      uVar4 = DAT_0011a00c;
      if ((int)uVar7 < 0) goto LAB_0010ca59;
    }
    uVar9 = (ulong)DAT_0011a008;
    DAT_001dfe88 = uVar7 & 0xff | uVar6;
    iVar13 = iVar13 + -1;
  } while (iVar13 != 0);
  if (DAT_0011a008 < uVar4) {
    DAT_0011a008 = DAT_0011a008 + 1;
    DAT_001dfe80 = (uint)(byte)(&DAT_0019c000)[uVar9];
  }
  else {
    uVar4 = FUN_0010d6c0(0);
    if ((int)uVar4 < 0) {
LAB_0010ca59:
                    /* WARNING: Subroutine does not return */
      FUN_0010d3f0("invalid compressed data -- unexpected end of file");
    }
    DAT_001dfe80 = uVar4 & 0xff;
  }
  if (0x18 < DAT_001dfe80 - 1) {
                    /* WARNING: Subroutine does not return */
    FUN_0010d3f0("invalid compressed data -- Huffman code bit length out of range");
  }
  lVar15 = 1;
  iVar18 = 0;
  iVar13 = 1;
  do {
    uVar9 = (ulong)DAT_0011a008;
    if (DAT_0011a008 < DAT_0011a00c) {
      DAT_0011a008 = DAT_0011a008 + 1;
      uVar4 = (uint)(byte)(&DAT_0019c000)[uVar9];
    }
    else {
      uVar4 = FUN_0010d6c0(0);
      if ((int)uVar4 < 0) goto LAB_0010ca59;
      uVar4 = uVar4 & 0xff;
    }
    uVar12 = DAT_001dfe80;
    *(uint *)(&DAT_001dfc80 + lVar15 * 4) = uVar4;
    if ((int)(iVar13 - (uint)(uVar12 == (uint)lVar15)) < (int)uVar4) goto LAB_0010cb10;
    lVar15 = lVar15 + 1;
    iVar18 = iVar18 + uVar4;
    iVar13 = (iVar13 - uVar4) * 2 + 1;
  } while ((int)lVar15 <= (int)uVar12);
  if (0xff < iVar18) {
LAB_0010cb10:
                    /* WARNING: Subroutine does not return */
    FUN_0010d3f0("too many leaves in Huffman tree");
  }
  lVar15 = (long)(int)uVar12;
  iVar13 = *(int *)(&DAT_001dfc80 + lVar15 * 4);
  *(int *)(&DAT_001dfc80 + lVar15 * 4) = iVar13 + 1;
  if ((int)uVar12 < 1) {
    *(int *)(&DAT_001dfc80 + lVar15 * 4) = iVar13 + 2;
  }
  else {
    lVar19 = 1;
    lVar15 = 0;
    do {
      *(int *)(&DAT_001dfd00 + lVar19 * 4) = (int)lVar15;
      if (0 < *(int *)(&DAT_001dfc80 + lVar19 * 4)) {
        lVar1 = lVar15 + 2 + (ulong)(*(int *)(&DAT_001dfc80 + lVar19 * 4) - 1);
        lVar17 = (long)((int)lVar15 + 1);
        do {
          while( true ) {
            uVar9 = (ulong)DAT_0011a008;
            lVar15 = (long)(int)lVar17;
            if (DAT_0011a008 < DAT_0011a00c) break;
            iVar13 = FUN_0010d6c0(0);
            if (iVar13 < 0) goto LAB_0010ca59;
            (&DAT_001dfd7f)[lVar17] = (char)iVar13;
            lVar17 = lVar17 + 1;
            uVar12 = DAT_001dfe80;
            if (lVar1 == lVar17) goto LAB_0010c78d;
          }
          DAT_0011a008 = DAT_0011a008 + 1;
          lVar16 = lVar17 + 1;
          (&DAT_001dfd7f)[lVar17] = (&DAT_0019c000)[uVar9];
          lVar17 = lVar16;
          uVar12 = DAT_001dfe80;
        } while (lVar1 != lVar16);
      }
LAB_0010c78d:
      lVar19 = lVar19 + 1;
    } while ((int)lVar19 <= (int)uVar12);
    lVar15 = (long)(int)uVar12;
    uVar4 = *(uint *)(&DAT_001dfc80 + lVar15 * 4);
    iVar13 = uVar4 + 1;
    *(int *)(&DAT_001dfc80 + lVar15 * 4) = iVar13;
    if (0 < (int)uVar12) {
      iVar18 = 0;
      while( true ) {
        iVar18 = iVar18 >> 1;
        *(int *)(&DAT_001dfc00 + lVar15 * 4) = iVar18;
        *(int *)(&DAT_001dfd00 + lVar15 * 4) = *(int *)(&DAT_001dfd00 + lVar15 * 4) - iVar18;
        lVar15 = lVar15 + -1;
        iVar18 = iVar18 + iVar13;
        if ((int)lVar15 < 1) break;
        iVar13 = *(int *)(&DAT_001dfc80 + lVar15 * 4);
      }
      if (iVar18 >> 1 == 1) {
        lVar15 = 1;
        if (0xc < (int)uVar12) {
          uVar12 = 0xc;
        }
        iVar13 = 1 << ((byte)uVar12 & 0x1f);
        pcVar8 = &DAT_0015b000 + iVar13;
        DAT_001dfbf0 = uVar12;
        do {
          iVar18 = *(int *)(&DAT_001dfc80 + lVar15 * 4) << ((byte)uVar12 - (char)lVar15 & 0x1f);
          if (iVar18 != 0) {
            pcVar11 = pcVar8 + ~(ulong)(iVar18 - 1);
            do {
              pcVar8 = pcVar8 + -1;
              *pcVar8 = (char)lVar15;
            } while (pcVar8 != pcVar11);
          }
          lVar15 = lVar15 + 1;
        } while ((int)lVar15 <= (int)uVar12);
        if (&DAT_0015b000 < pcVar8) {
          do {
            pcVar8 = pcVar8 + -1;
            *pcVar8 = '\0';
          } while (pcVar8 != &DAT_0015b000);
        }
        DAT_001dfbe0 = 0;
        uVar5 = iVar13 - 1;
        DAT_001dfbe8 = 0;
        do {
          if ((int)uVar12 <= DAT_001dfbe0) {
            do {
              uVar14 = uVar5 & (uint)(DAT_001dfbe8 >> ((char)DAT_001dfbe0 - (char)uVar12 & 0x3fU));
              bVar2 = (&DAT_0015b000)[uVar14];
              if (bVar2 == 0) {
                if (uVar14 < *(uint *)(&DAT_001dfc00 + (long)(int)uVar12 * 4)) {
                  lVar15 = (long)(int)(uVar12 + 1);
                  uVar9 = DAT_001dfbe8;
                  uVar10 = uVar5;
                  do {
                    uVar12 = (uint)lVar15;
                    uVar10 = uVar10 * 2 + 1;
                    for (; DAT_001dfbe0 < (int)uVar12; DAT_001dfbe0 = DAT_001dfbe0 + 8) {
                      uVar6 = (ulong)DAT_0011a008;
                      if (DAT_0011a008 < DAT_0011a00c) {
                        DAT_0011a008 = DAT_0011a008 + 1;
                        uVar6 = (ulong)(byte)(&DAT_0019c000)[uVar6];
                      }
                      else {
                        uVar6 = FUN_0010d6c0(0);
                        if ((int)uVar6 < 0) goto LAB_0010ca59;
                      }
                      uVar9 = uVar6 & 0xff | uVar9 << 8;
                      DAT_001dfbe8 = uVar9;
                    }
                    cVar3 = (char)lVar15;
                    lVar19 = lVar15 * 4;
                    lVar15 = lVar15 + 1;
                    uVar14 = (uint)(uVar9 >> ((char)DAT_001dfbe0 - cVar3 & 0x3fU)) & uVar10;
                  } while (uVar14 < *(uint *)(&DAT_001dfc00 + lVar19));
                }
              }
              else {
                uVar14 = uVar14 >> ((char)uVar12 - bVar2 & 0x1f);
                uVar12 = (uint)bVar2;
              }
              if ((uVar14 == uVar4) && (DAT_001dfe80 == uVar12)) {
                FUN_0010d640();
                if ((uint)DAT_0011a820 == DAT_001dfe88) {
                  return 0;
                }
                    /* WARNING: Subroutine does not return */
                FUN_0010d3f0("invalid compressed data--length error");
              }
              uVar9 = (ulong)DAT_0011a004;
              uVar10 = DAT_0011a004 + 1;
              DAT_0011a004 = uVar10;
              (&DAT_0013b000)[uVar9] =
                   (&DAT_001dfd80)[uVar14 + *(int *)(&DAT_001dfd00 + (long)(int)uVar12 * 4)];
              if (uVar10 == 0x8000) {
                FUN_0010d640();
              }
              DAT_001dfbe0 = DAT_001dfbe0 - uVar12;
              uVar12 = DAT_001dfbf0;
            } while ((int)DAT_001dfbf0 <= DAT_001dfbe0);
          }
          uVar9 = DAT_001dfbe8 << 8;
          uVar6 = (ulong)DAT_0011a008;
          if (DAT_0011a008 < DAT_0011a00c) {
            DAT_0011a008 = DAT_0011a008 + 1;
            uVar6 = (ulong)(byte)(&DAT_0019c000)[uVar6];
          }
          else {
            uVar6 = FUN_0010d6c0(0);
            uVar12 = DAT_001dfbf0;
            if ((int)uVar6 < 0) goto LAB_0010ca59;
          }
          DAT_001dfbe0 = DAT_001dfbe0 + 8;
          DAT_001dfbe8 = uVar6 & 0xff | uVar9;
        } while( true );
      }
    }
  }
                    /* WARNING: Subroutine does not return */
  FUN_0010d3f0("too few leaves in Huffman tree");
}




// Function: check_zipfile @ 0xcb20

undefined8 check_zipfile(undefined4 param_1)

{
  byte bVar1;
  ulong uVar2;
  char *pcVar3;
  
  uVar2 = (ulong)DAT_0011a008;
  DAT_0011a014 = param_1;
  DAT_0011a008 = *(ushort *)(&DAT_0019c01a + uVar2) + 0x1e +
                 (uint)*(ushort *)(&DAT_0019c01c + uVar2) + DAT_0011a008;
  if ((DAT_0011a00c < DAT_0011a008) || (*(int *)(&DAT_0019c000 + uVar2) != 0x4034b50)) {
    pcVar3 = "\n%s: %s: not a valid zip file\n";
  }
  else {
    pcVar3 = "\n%s: %s: first entry not deflated or stored -- use unzip\n";
    DAT_00118020 = (uint)(byte)(&DAT_0019c008)[uVar2];
    if (((&DAT_0019c008)[uVar2] & 0xf7) == 0) {
      bVar1 = (&DAT_0019c006)[uVar2];
      DAT_001dfea0 = bVar1 & 1;
      if ((bVar1 & 1) == 0) {
        DAT_001dfe9c = 1;
        DAT_001dfe98 = bVar1 >> 3 & 1;
        return 0;
      }
      pcVar3 = "\n%s: %s: encrypted file -- use unzip\n";
    }
  }
  FUN_0010f6b0(stderr,pcVar3,DAT_0011a850,&DAT_0011a420);
  DAT_0011a84c = 1;
  return 1;
}




// Function: unzip @ 0xcc20

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
  DAT_0011a010 = param_2;
  DAT_0011a014 = param_1;
  FUN_0010d0d0(0,0);
  if (DAT_001dfe9c == 0) {
    if (DAT_00118020 != 8) {
LAB_0010d07b:
                    /* WARNING: Subroutine does not return */
      FUN_0010d3f0("internal error, invalid method");
    }
    uVar8 = 0;
    uVar3 = 0;
LAB_0010cd42:
    iVar2 = FUN_00109a50();
    if (iVar2 == 3) {
                    /* WARNING: Subroutine does not return */
      FUN_0010d430();
    }
    if (iVar2 != 0) {
                    /* WARNING: Subroutine does not return */
      FUN_0010d3f0("invalid compressed data--format violated");
    }
    if (DAT_001dfe9c != 0) goto LAB_0010cf45;
LAB_0010cd68:
    puVar5 = &local_48;
    do {
      while (uVar3 = (ulong)DAT_0011a008, DAT_0011a00c <= DAT_0011a008) {
        puVar6 = (undefined1 *)((long)puVar5 + 1);
        uVar1 = FUN_0010d6c0(0);
        *(undefined1 *)puVar5 = uVar1;
        puVar5 = (uint *)puVar6;
        if (puVar6 == local_40) goto LAB_0010cdbb;
      }
      DAT_0011a008 = DAT_0011a008 + 1;
      puVar6 = (undefined1 *)((long)puVar5 + 1);
      *(undefined1 *)puVar5 = (&DAT_0019c000)[uVar3];
      puVar5 = (uint *)puVar6;
    } while (puVar6 != local_40);
LAB_0010cdbb:
    uVar3 = (ulong)local_48;
    uVar8 = local_44;
  }
  else {
    uVar7 = DAT_0019c00e;
    uVar8 = DAT_0019c016;
    if (DAT_001dfe98 != 0) {
      uVar7 = 0;
      uVar8 = 0;
    }
    uVar3 = (ulong)uVar7;
    if (DAT_00118020 == 8) goto LAB_0010cd42;
    if (DAT_00118020 != 0) goto LAB_0010d07b;
    uVar4 = (ulong)DAT_0019c012;
    uVar10 = (ulong)DAT_0019c016;
    if (DAT_001dfea0 != 0) {
      uVar4 = uVar4 - 0xc;
    }
    if (uVar10 != uVar4) {
      FUN_0010f6b0(stderr,"len %lu, siz %lu\n",uVar10);
                    /* WARNING: Subroutine does not return */
      FUN_0010d3f0("invalid compressed data--length mismatch");
    }
    for (; uVar10 != 0; uVar10 = uVar10 - 1) {
      while( true ) {
        uVar4 = (ulong)DAT_0011a008;
        if (DAT_0011a008 < DAT_0011a00c) {
          DAT_0011a008 = DAT_0011a008 + 1;
          uVar1 = (&DAT_0019c000)[uVar4];
        }
        else {
          uVar1 = FUN_0010d6c0(0);
        }
        uVar4 = (ulong)DAT_0011a004;
        DAT_0011a004 = DAT_0011a004 + 1;
        (&DAT_0013b000)[uVar4] = uVar1;
        if (DAT_0011a004 == 0x8000) break;
        uVar10 = uVar10 - 1;
        if (uVar10 == 0) goto LAB_0010cf30;
      }
      FUN_0010d640();
    }
LAB_0010cf30:
    FUN_0010d640();
    if (DAT_001dfe9c == 0) goto LAB_0010cd68;
LAB_0010cf45:
    if (DAT_001dfe98 != 0) {
      puVar5 = &local_48;
      do {
        while (uVar3 = (ulong)DAT_0011a008, DAT_0011a00c <= DAT_0011a008) {
          puVar6 = (undefined1 *)((long)puVar5 + 1);
          uVar1 = FUN_0010d6c0(0);
          *(undefined1 *)puVar5 = uVar1;
          puVar5 = (uint *)puVar6;
          if (puVar6 == local_38) goto LAB_0010cfab;
        }
        DAT_0011a008 = DAT_0011a008 + 1;
        puVar6 = (undefined1 *)((long)puVar5 + 1);
        *(undefined1 *)puVar5 = (&DAT_0019c000)[uVar3];
        puVar5 = (uint *)puVar6;
      } while (puVar6 != local_38);
LAB_0010cfab:
      uVar3 = (ulong)local_44;
      uVar8 = local_3c;
    }
  }
  uVar4 = FUN_0010d0d0(&DAT_0015b000,0);
  bVar11 = uVar4 != uVar3;
  if (bVar11) {
    FUN_0010f6b0(stderr,"\n%s: %s: invalid compressed data--crc error\n",DAT_0011a850,&DAT_0011a420)
    ;
  }
  if ((uint)DAT_0011a820 == uVar8) {
    if (DAT_001dfe9c != 0) goto LAB_0010ce4a;
LAB_0010ceab:
    DAT_001dfe9c = 0;
    uVar9 = 0;
    if (!bVar11) goto LAB_0010cee9;
  }
  else {
    FUN_0010f6b0(stderr,"\n%s: %s: invalid compressed data--length error\n",DAT_0011a850,
                 &DAT_0011a420);
    if (DAT_001dfe9c != 0) {
      bVar11 = true;
LAB_0010ce4a:
      if ((DAT_0011a008 + 4 < DAT_0011a00c) &&
         (((long)(int)(uint)*(ushort *)(&DAT_0019c002 + DAT_0011a008) << 0x10 |
          (long)(int)(uint)*(ushort *)(&DAT_0019c000 + DAT_0011a008)) == 0x4034b50)) {
        if (DAT_0011a860 == 0) {
          FUN_0010f6b0(stderr,"%s: %s has more than one entry -- unchanged\n",DAT_0011a850,
                       &DAT_0011a420);
          DAT_001dfe9c = 0;
          goto LAB_0010cecb;
        }
        if (DAT_0011a85c == 0) {
          FUN_0010f6b0(stderr,"%s: %s has more than one entry--rest ignored\n",DAT_0011a850,
                       &DAT_0011a420);
        }
        if (DAT_0011a84c == 0) {
          DAT_0011a84c = 2;
        }
      }
      goto LAB_0010ceab;
    }
  }
LAB_0010cecb:
  DAT_001dfe98 = 0;
  DAT_0011a84c = 1;
  if (DAT_0011a858 == 0) {
    DAT_001dfe90 = uVar3;
                    /* WARNING: Subroutine does not return */
    FUN_00108440();
  }
  uVar9 = 1;
LAB_0010cee9:
  DAT_001dfe98 = 0;
  DAT_001dfe90 = uVar3;
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar9;
}




// Function: getcrc @ 0xd130

ulong getcrc(void)

{
  return DAT_001183f0 ^ 0xffffffff;
}




// Function: clear_bufs @ 0xd150

void clear_bufs(void)

{
  DAT_0011a004 = 0;
  DAT_0011a008 = 0;
  DAT_0011a00c = 0;
  DAT_0011a820 = 0;
  DAT_0011a828 = 0;
  return;
}




// Function: read_buffer @ 0xd190

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
      uVar1 = FUN_0010e0f0(param_1,3);
      if (-1 < (int)uVar1) {
        if ((uVar1 & 0x800) == 0) {
          *piVar4 = 0xb;
        }
        else {
          iVar2 = FUN_0010e0f0(param_1,4,uVar1 & 0xfffff7ff);
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




// Function: strlwr @ 0xd230

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




// Function: add_envopt @ 0xd2b0

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
    pcVar2 = (char *)FUN_0010f640(pcVar2);
    pcVar6 = pcVar2;
    if (*pcVar2 == '\0') {
LAB_0010d3d9:
      pcVar6 = (char *)0x0;
      free(pcVar2);
    }
    else {
      do {
        sVar3 = strspn(pcVar6," \t");
        pcVar6 = pcVar6 + sVar3;
        if (*pcVar6 == '\0') {
          if (iVar7 == 0) goto LAB_0010d3d9;
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
      puVar5 = (undefined8 *)FUN_0010f540((long)(iVar7 + 2),8);
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




// Function: gzip_error @ 0xd3f0

void gzip_error(undefined8 param_1)

{
  FUN_0010f6b0(stderr,"\n%s: %s: %s\n",DAT_0011a850,&DAT_0011a420,param_1);
                    /* WARNING: Subroutine does not return */
  FUN_00108440();
}




// Function: xalloc_die @ 0xd430

void xalloc_die(void)

{
  FUN_0010f6b0(stderr,"\n%s: memory_exhausted\n",DAT_0011a850);
                    /* WARNING: Subroutine does not return */
  FUN_00108440();
}




// Function: warning @ 0xd460

void warning(undefined8 param_1)

{
  if (DAT_0011a85c == 0) {
    FUN_0010f6b0(stderr,"%s: %s: warning: %s\n",DAT_0011a850,&DAT_0011a420,param_1);
    if (DAT_0011a84c == 0) {
      DAT_0011a84c = 2;
    }
    return;
  }
  if (DAT_0011a84c == 0) {
    DAT_0011a84c = 2;
    return;
  }
  return;
}




// Function: read_error @ 0xd4e0

void read_error(void)

{
  int iVar1;
  int *piVar2;
  
  piVar2 = __errno_location();
  iVar1 = *piVar2;
  FUN_0010f6b0(stderr,&DAT_001137cb,DAT_0011a850);
  if (iVar1 == 0) {
    FUN_0010f6b0(stderr,"%s: unexpected end of file\n",&DAT_0011a420);
  }
  else {
    *piVar2 = iVar1;
    perror(&DAT_0011a420);
  }
                    /* WARNING: Subroutine does not return */
  FUN_00108440();
}




// Function: write_error @ 0xd550

void write_error(void)

{
  int iVar1;
  int *piVar2;
  
  piVar2 = __errno_location();
  iVar1 = *piVar2;
  FUN_0010f6b0(stderr,&DAT_001137cb,DAT_0011a850);
  *piVar2 = iVar1;
  perror(&DAT_0011a020);
                    /* WARNING: Subroutine does not return */
  FUN_00108440();
}




// Function: flush_outbuf @ 0xd600

void flush_outbuf(void)

{
  if (DAT_0011a004 == 0) {
    return;
  }
  FUN_0010d5a0(DAT_0011a010,&DAT_0015b000);
  DAT_0011a004 = 0;
  return;
}




// Function: flush_window @ 0xd640

void flush_window(void)

{
  byte bVar1;
  byte *pbVar2;
  
  if (DAT_0011a004 == 0) {
    return;
  }
  pbVar2 = &DAT_0013b000;
  do {
    bVar1 = *pbVar2;
    pbVar2 = pbVar2 + 1;
    DAT_001183f0 = DAT_001183f0 >> 8 ^
                   *(ulong *)(&DAT_00113800 + (ulong)(((uint)bVar1 ^ (uint)DAT_001183f0) & 0xff) * 8
                             );
  } while (pbVar2 != &DAT_0013b000 + DAT_0011a004);
  FUN_0010d5a0(DAT_0011a010,&DAT_0013b000,DAT_0011a004);
  DAT_0011a004 = 0;
  return;
}




// Function: fill_inbuf @ 0xd6c0

ulong fill_inbuf(int param_1)

{
  int iVar1;
  ulong uVar2;
  int *piVar3;
  
  uVar2 = 0;
  DAT_0011a00c = 0;
  do {
    iVar1 = FUN_0010d190(DAT_0011a014,&DAT_0019c000 + uVar2,0x40000 - (int)uVar2);
    if (iVar1 == 0) {
      uVar2 = (ulong)DAT_0011a00c;
      if (DAT_0011a00c == 0) {
        if (param_1 != 0) {
          return 0xffffffff;
        }
        FUN_0010d640();
        piVar3 = __errno_location();
        *piVar3 = 0;
LAB_0010d765:
                    /* WARNING: Subroutine does not return */
        FUN_0010d4e0();
      }
      break;
    }
    if (iVar1 == -1) goto LAB_0010d765;
    DAT_0011a00c = iVar1 + DAT_0011a00c;
    uVar2 = (ulong)DAT_0011a00c;
  } while (DAT_0011a00c < 0x40000);
  DAT_0011a008 = 1;
  DAT_0011a828 = DAT_0011a828 + uVar2;
  return (ulong)DAT_0019c000;
}




// Function: copy @ 0xd770

undefined8 copy(undefined4 param_1,int param_2)

{
  uint uVar1;
  uint uVar2;
  int *piVar3;
  ulong uVar4;
  size_t __n;
  size_t sVar5;
  undefined1 *__buf;
  
  piVar3 = __errno_location();
  *piVar3 = 0;
  uVar4 = (ulong)DAT_0011a008;
  if (DAT_0011a008 < DAT_0011a00c) {
    do {
      sVar5 = (size_t)(DAT_0011a00c - (int)uVar4);
      __buf = &DAT_0019c000 + uVar4;
      DAT_0011a820 = DAT_0011a820 + sVar5;
      if (DAT_0011a858 == 0) {
        while( true ) {
          uVar2 = (uint)sVar5;
          __n = 0x7fffffff;
          if (uVar2 < 0x80000000) {
            __n = sVar5;
          }
          uVar4 = write(param_2,__buf,__n);
          uVar1 = (uint)uVar4;
          if (uVar1 == uVar2) break;
          if (uVar1 == 0xffffffff) {
            FUN_0010d550();
            goto LAB_0010d846;
          }
          sVar5 = (size_t)(uVar2 - uVar1);
          __buf = __buf + (uVar4 & 0xffffffff);
        }
      }
      uVar2 = FUN_0010d190(param_1,&DAT_0019c000,0x40000);
      if (uVar2 == 0xffffffff) {
LAB_0010d846:
                    /* WARNING: Subroutine does not return */
        FUN_0010d4e0();
      }
      uVar4 = 0;
      DAT_0011a828 = DAT_0011a828 + (int)uVar2;
      DAT_0011a008 = 0;
      DAT_0011a00c = uVar2;
    } while (uVar2 != 0);
  }
  return 0;
}




// Function: display_ratio @ 0xd850

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void display_ratio(long param_1,long param_2,undefined8 param_3)

{
  double dVar1;
  
  dVar1 = 0.0;
  if (param_2 != 0) {
    dVar1 = ((double)param_1 * _DAT_00114000) / (double)param_2;
  }
  FUN_0010f6b0(dVar1,param_3,"%5.1f%%");
  return;
}




// Function: fprint_off @ 0xd8a0

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




// Function: zip @ 0xd9d0

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
  _DAT_0015b000 = 0x8b1f;
  DAT_0015b003 = (DAT_0011a848 != 0) << 3;
  DAT_00118020 = 8;
  DAT_0015b002 = 8;
  DAT_0011a004 = 4;
  DAT_0011a010 = param_2;
  DAT_0011a014 = param_1;
  if (DAT_0011a838 < 0) {
    uVar6 = 0;
    uVar1 = 0;
    DAT_0015b005 = 0;
    DAT_0015b004 = 0;
LAB_0010da79:
    uVar3 = 7;
    uVar2 = 6;
  }
  else {
    if (DAT_0011a830 - 1U < 0xffffffff) {
      DAT_0015b004 = (undefined1)DAT_0011a830;
      DAT_0015b005 = (undefined1)((ulong)DAT_0011a830 >> 8);
      uVar1 = (undefined1)((ulong)DAT_0011a830 >> 0x10);
      uVar6 = (undefined1)((ulong)DAT_0011a830 >> 0x18);
      goto LAB_0010da79;
    }
    FUN_0010d460("file timestamp out of range for gzip format");
    uVar3 = (ulong)DAT_0011a004;
    if (DAT_0011a004 < 0x3fffe) {
      uVar2 = DAT_0011a004 + 2;
      uVar9 = DAT_0011a004 + 3;
      (&DAT_0015b000)[DAT_0011a004] = 0;
      (&DAT_0015b000)[DAT_0011a004 + 1] = 0;
    }
    else {
      DAT_0011a004 = DAT_0011a004 + 1;
      (&DAT_0015b000)[uVar3] = 0;
      uVar9 = DAT_0011a004;
      if (DAT_0011a004 == 0x40000) {
        FUN_0010d600();
        uVar9 = DAT_0011a004;
      }
      uVar2 = uVar9 + 1;
      DAT_0011a004 = uVar2;
      (&DAT_0015b000)[uVar9] = 0;
      if (uVar2 == 0x40000) {
        FUN_0010d600();
        uVar9 = DAT_0011a004 + 1;
        uVar2 = DAT_0011a004;
      }
      else {
        uVar9 = uVar9 + 2;
      }
    }
    uVar3 = (ulong)uVar9;
    if (0x3fffd < uVar2) {
      DAT_0011a004 = uVar9;
      (&DAT_0015b000)[uVar2] = 0;
      if (uVar9 == 0x40000) {
        FUN_0010d600();
        uVar3 = (ulong)DAT_0011a004;
      }
      DAT_0011a004 = (int)uVar3 + 1;
      (&DAT_0015b000)[uVar3] = 0;
      if (DAT_0011a004 == 0x40000) {
        FUN_0010d600();
      }
      goto LAB_0010daa4;
    }
    uVar6 = 0;
    uVar1 = 0;
  }
  DAT_0011a004 = uVar2 + 2;
  (&DAT_0015b000)[uVar2] = uVar1;
  (&DAT_0015b000)[uVar3] = uVar6;
LAB_0010daa4:
  FUN_0010d0d0(0,0);
  FUN_00104380(param_2);
  FUN_0010aa20(&local_22,&DAT_00118020);
  cVar7 = '\x04';
  if (DAT_0011801c != 1) {
    cVar7 = (DAT_0011801c == 9) * '\x02';
  }
  uVar3 = (ulong)DAT_0011a004;
  DAT_0011a004 = DAT_0011a004 + 1;
  (&DAT_0015b000)[uVar3] = cVar7;
  if (DAT_0011a004 == 0x40000) {
    FUN_0010d600();
  }
  uVar3 = (ulong)DAT_0011a004;
  DAT_0011a004 = DAT_0011a004 + 1;
  (&DAT_0015b000)[uVar3] = 3;
  if (DAT_0011a004 == 0x40000) {
    FUN_0010d600();
  }
  uVar3 = (ulong)DAT_0011a004;
  if (DAT_0011a848 != 0) {
    pcVar4 = (char *)thunk_FUN_0010dfd0(&DAT_0011a420);
    do {
      while( true ) {
        uVar2 = (int)uVar3 + 1;
        DAT_0011a004 = uVar2;
        (&DAT_0015b000)[uVar3] = *pcVar4;
        uVar3 = (ulong)uVar2;
        if (uVar2 != 0x40000) break;
        FUN_0010d600();
        uVar3 = (ulong)DAT_0011a004;
        cVar7 = *pcVar4;
        pcVar4 = pcVar4 + 1;
        if (cVar7 == '\0') goto LAB_0010db80;
      }
      cVar7 = *pcVar4;
      pcVar4 = pcVar4 + 1;
    } while (cVar7 != '\0');
  }
LAB_0010db80:
  DAT_001dfea8 = uVar3;
  FUN_00104bc0(DAT_0011801c);
  if ((DAT_0011a840 != -1) && (DAT_0011a840 != DAT_0011a828)) {
    FUN_0010f6b0(stderr,"%s: %s: file size changed while zipping\n",DAT_0011a850,&DAT_0011a420);
  }
  uVar1 = FUN_0010d130();
  uVar3 = (ulong)DAT_0011a004;
  uVar2 = DAT_0011a004 + 1;
  uVar10 = (ulong)uVar2;
  if (DAT_0011a004 < 0x3fffe) {
    uVar8 = (ulong)DAT_0011a004;
    uVar9 = DAT_0011a004 + 2;
    uVar3 = (ulong)uVar9;
    DAT_0011a004 = uVar2;
    (&DAT_0015b000)[uVar8] = uVar1;
    FUN_0010d130();
    DAT_0011a004 = uVar9;
    (&DAT_0015b000)[uVar10] = extraout_AH;
  }
  else {
    DAT_0011a004 = uVar2;
    (&DAT_0015b000)[uVar3] = uVar1;
    if (uVar2 == 0x40000) {
      FUN_0010d600();
      uVar10 = (ulong)DAT_0011a004;
    }
    FUN_0010d130();
    uVar2 = (int)uVar10 + 1;
    uVar3 = (ulong)uVar2;
    DAT_0011a004 = uVar2;
    (&DAT_0015b000)[uVar10] = extraout_AH_00;
    if (uVar2 == 0x40000) {
      FUN_0010d600();
      uVar3 = (ulong)DAT_0011a004;
    }
  }
  FUN_0010d130();
  uVar2 = (uint)uVar3;
  uVar9 = uVar2 + 1;
  uVar10 = (ulong)uVar9;
  DAT_0011a004 = uVar9;
  if (uVar2 < 0x3fffe) {
    uVar8 = (ulong)(uVar2 + 2);
    (&DAT_0015b000)[uVar3] = extraout_var;
    FUN_0010d130();
    (&DAT_0015b000)[uVar10] = extraout_var_00;
  }
  else {
    (&DAT_0015b000)[uVar3] = extraout_var;
    if (uVar9 == 0x40000) {
      FUN_0010d600();
      uVar10 = (ulong)DAT_0011a004;
    }
    FUN_0010d130();
    uVar2 = (int)uVar10 + 1;
    uVar8 = (ulong)uVar2;
    DAT_0011a004 = uVar2;
    (&DAT_0015b000)[uVar10] = extraout_var_01;
    if (uVar2 == 0x40000) {
      FUN_0010d600();
      uVar8 = (ulong)DAT_0011a004;
    }
  }
  lVar5 = DAT_0011a828;
  uVar2 = (uint)uVar8;
  uVar9 = uVar2 + 1;
  uVar3 = (ulong)uVar9;
  if (uVar2 < 0x3fffe) {
    uVar10 = (ulong)(uVar2 + 2);
    (&DAT_0015b000)[uVar8] = (char)DAT_0011a828;
    (&DAT_0015b000)[uVar3] = (char)((ulong)lVar5 >> 8);
  }
  else {
    DAT_0011a004 = uVar9;
    (&DAT_0015b000)[uVar8] = (char)DAT_0011a828;
    if (uVar9 == 0x40000) {
      FUN_0010d600();
      uVar3 = (ulong)DAT_0011a004;
      lVar5 = DAT_0011a828;
    }
    DAT_0011a004 = (int)uVar3 + 1;
    uVar10 = (ulong)DAT_0011a004;
    (&DAT_0015b000)[uVar3] = (char)((ulong)lVar5 >> 8);
    if (DAT_0011a004 == 0x40000) {
      FUN_0010d600();
      uVar10 = (ulong)DAT_0011a004;
      lVar5 = DAT_0011a828;
    }
  }
  uVar2 = (uint)uVar10;
  uVar9 = uVar2 + 1;
  uVar3 = (ulong)uVar9;
  uVar1 = (undefined1)((ulong)lVar5 >> 0x10);
  if (uVar2 < 0x3fffe) {
    DAT_0011a004 = uVar2 + 2;
    (&DAT_0015b000)[uVar10] = uVar1;
    (&DAT_0015b000)[uVar3] = (char)((ulong)lVar5 >> 0x18);
  }
  else {
    DAT_0011a004 = uVar9;
    (&DAT_0015b000)[uVar10] = uVar1;
    if (uVar9 == 0x40000) {
      FUN_0010d600();
      uVar3 = (ulong)DAT_0011a004;
      lVar5 = DAT_0011a828;
    }
    DAT_0011a004 = (int)uVar3 + 1;
    (&DAT_0015b000)[uVar3] = (char)((ulong)lVar5 >> 0x18);
    if (DAT_0011a004 == 0x40000) {
      FUN_0010d600();
    }
  }
  DAT_001dfea8 = DAT_001dfea8 + 8;
  FUN_0010d600();
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return 0;
}




// Function: file_read @ 0xdf80

ulong file_read(undefined8 param_1,undefined4 param_2)

{
  uint uVar1;
  
  uVar1 = FUN_0010d190(DAT_0011a014,param_1,param_2);
  if (uVar1 != 0) {
    if (uVar1 == 0xffffffff) {
                    /* WARNING: Subroutine does not return */
      FUN_0010d4e0();
    }
    FUN_0010d0d0(param_1,uVar1);
    DAT_0011a828 = DAT_0011a828 + (ulong)uVar1;
  }
  return (ulong)uVar1;
}




// Function: last_component @ 0xdfd0

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




// Function: open_safer @ 0xe360

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
  FUN_0010e870(iVar2);
  if (lVar1 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: rpl_fseeko @ 0xe430

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




// Function: openat_safer @ 0xe4b0

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
  FUN_0010e870(iVar2);
  if (lVar1 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: direntry_cmp_name @ 0xe530

void direntry_cmp_name(undefined8 *param_1,undefined8 *param_2)

{
  strcmp((char *)*param_1,(char *)*param_2);
  return;
}




// Function: streamsavedir @ 0xe540

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
  __compar = *(__compar_fn_t *)(&DAT_00117b50 + (ulong)param_2 * 8);
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
                       FUN_0010f440(__ptr_00,&local_50,sVar5 - (local_50 - lVar9),0x7ffffffffffffffe
                                    ,1);
          }
          memcpy(__ptr_00 + lVar9,pcVar10,sVar5);
        }
        else {
          if (local_48 == __nmemb) {
            __ptr = (void *)FUN_0010f440(__ptr,&local_48,1,0xffffffffffffffff,8);
          }
          lVar1 = __nmemb * 8;
          __nmemb = __nmemb + 1;
          uVar6 = FUN_0010f640(pcVar10);
          *(undefined8 *)((long)__ptr + lVar1) = uVar6;
        }
        lVar9 = lVar9 + sVar5;
      }
    }
    if (*piVar3 == 0) {
      if (__compar == (__compar_fn_t)0x0) {
        if (lVar9 == local_50) {
          __ptr_00 = (undefined1 *)FUN_0010f1f0(__ptr_00,lVar9 + 1);
        }
        puVar7 = __ptr_00 + lVar9;
      }
      else {
        if (__nmemb == 0) {
          puVar7 = (undefined1 *)FUN_0010f180(lVar9 + 1);
          __ptr_00 = puVar7;
        }
        else {
          pcVar10 = (char *)0x0;
          qsort(__ptr,__nmemb,8,__compar);
          sVar5 = 0;
          __ptr_00 = (undefined1 *)FUN_0010f180(lVar9 + 1);
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




// Function: savedir @ 0xe800

void * savedir(undefined8 param_1,undefined4 param_2)

{
  int iVar1;
  DIR *__dirp;
  void *__ptr;
  
  __dirp = (DIR *)FUN_00110af0();
  if (__dirp == (DIR *)0x0) {
    return (void *)0x0;
  }
  __ptr = (void *)FUN_0010e540(__dirp,param_2);
  iVar1 = closedir(__dirp);
  if (iVar1 == 0) {
    return __ptr;
  }
  free(__ptr);
  return (void *)0x0;
}




// Function: fd_safer @ 0xe870

uint fd_safer(uint param_1)

{
  int iVar1;
  uint uVar2;
  int *piVar3;
  
  if (2 < param_1) {
    return param_1;
  }
  uVar2 = FUN_00110c00();
  piVar3 = __errno_location();
  iVar1 = *piVar3;
  close(param_1);
  *piVar3 = iVar1;
  return uVar2;
}




// Function: validate_timespec @ 0xe8d0

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




// Function: update_timespec @ 0xe980

undefined8 update_timespec(long param_1,long *param_2)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  long lVar3;
  
  puVar1 = (undefined8 *)*param_2;
  lVar3 = puVar1[3];
  if (puVar1[1] == 0x3ffffffe) {
    if (lVar3 == 0x3ffffffe) {
      return 1;
    }
    uVar2 = *(undefined8 *)(param_1 + 0x50);
    *puVar1 = *(undefined8 *)(param_1 + 0x48);
    puVar1[1] = uVar2;
  }
  else {
    if (puVar1[1] == 0x3fffffff) {
      if (lVar3 == 0x3fffffff) {
        *param_2 = 0;
        return 0;
      }
      FUN_00110ba0(puVar1);
      lVar3 = puVar1[3];
    }
    if (lVar3 == 0x3ffffffe) {
      uVar2 = *(undefined8 *)(param_1 + 0x60);
      puVar1[2] = *(undefined8 *)(param_1 + 0x58);
      puVar1[3] = uVar2;
      return 0;
    }
  }
  if (lVar3 != 0x3fffffff) {
    return 0;
  }
  FUN_00110ba0(puVar1 + 2);
  return 0;
}




// Function: fdutimens @ 0xea40

int fdutimens(int param_1,char *param_2,__time_t *param_3)

{
  bool bVar1;
  bool bVar2;
  char cVar3;
  int iVar4;
  int iVar5;
  int *piVar6;
  timeval *__tvp;
  long in_FS_OFFSET;
  bool bVar7;
  timespec *local_130;
  timespec local_128;
  __time_t local_118;
  __time_t _Stack_110;
  timeval local_108;
  __time_t local_f8;
  long lStack_f0;
  timeval local_e8;
  __time_t local_d8;
  __suseconds_t _Stack_d0;
  stat local_c8;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_3 == (__time_t *)0x0) {
    local_130 = (timespec *)0x0;
    iVar4 = 0;
LAB_0010eaaa:
    if ((-1 < param_1) || (param_2 != (char *)0x0)) {
      if (-1 < DAT_001dfeb8) {
        if (iVar4 == 2) {
          if (param_1 < 0) {
            iVar4 = stat(param_2,&local_c8);
          }
          else {
            iVar4 = fstat(param_1,&local_c8);
          }
          if (iVar4 != 0) goto LAB_0010ebfb;
          if (local_130->tv_nsec == 0x3ffffffe) {
            iVar4 = 3;
            local_130->tv_sec = local_c8.st_atim.tv_sec;
            local_130->tv_nsec = local_c8.st_atim.tv_nsec;
          }
          else {
            iVar4 = 3;
            if (local_130[1].tv_nsec == 0x3ffffffe) {
              local_130[1].tv_sec = local_c8.st_mtim.tv_sec;
              local_130[1].tv_nsec = local_c8.st_mtim.tv_nsec;
            }
          }
        }
        if (param_1 < 0) {
          iVar5 = utimensat(-100,param_2,local_130,0);
        }
        else {
          iVar5 = futimens(param_1,local_130);
        }
        if (iVar5 < 1) {
          if ((iVar5 == 0) || (piVar6 = __errno_location(), *piVar6 != 0x26)) {
            DAT_001dfeb8 = 1;
            goto LAB_0010ec01;
          }
        }
        else {
          piVar6 = __errno_location();
          *piVar6 = 0x26;
        }
      }
      DAT_001dfeb8 = -1;
      DAT_001dfeb4 = 0xffffffff;
      if (iVar4 != 0) {
        if (iVar4 != 3) {
          if (param_1 < 0) {
            iVar4 = stat(param_2,&local_c8);
          }
          else {
            iVar4 = fstat(param_1,&local_c8);
          }
          if (iVar4 != 0) goto LAB_0010ebfb;
        }
        if (local_130 != (timespec *)0x0) {
          iVar5 = 0;
          cVar3 = FUN_0010e980(&local_c8);
          if (cVar3 != '\0') goto LAB_0010ec01;
        }
      }
      __tvp = (timeval *)0x0;
      if (local_130 != (timespec *)0x0) {
        local_108.tv_sec = local_130->tv_sec;
        local_f8 = local_130[1].tv_sec;
        local_108.tv_usec = local_130->tv_nsec / 1000;
        __tvp = &local_108;
        lStack_f0 = local_130[1].tv_nsec / 1000;
      }
      if (param_1 < 0) {
        iVar5 = futimesat(-100,param_2,__tvp);
        goto LAB_0010ec01;
      }
      iVar4 = futimesat(param_1,(char *)0x0,__tvp);
      if (iVar4 != 0) {
        iVar5 = -1;
        if (param_2 != (char *)0x0) {
          iVar5 = utimes(param_2,__tvp);
        }
        goto LAB_0010ec01;
      }
      iVar5 = 0;
      if (__tvp == (timeval *)0x0) goto LAB_0010ec01;
      bVar1 = __tvp->tv_usec < 500000;
      bVar2 = __tvp[1].tv_usec < 500000;
      if (((bVar1) && (bVar2)) || (iVar4 = fstat(param_1,&local_c8), iVar4 != 0)) goto LAB_0010ec01;
      local_e8.tv_sec = __tvp->tv_sec;
      local_e8.tv_usec = __tvp->tv_usec;
      local_d8 = __tvp[1].tv_sec;
      _Stack_d0 = __tvp[1].tv_usec;
      bVar7 = local_c8.st_mtim.tv_sec - __tvp[1].tv_sec == 1;
      if (((local_c8.st_atim.tv_sec - __tvp->tv_sec != 1) || (bVar1)) ||
         (local_c8.st_atim.tv_nsec != 0)) {
        if ((bVar7 && !bVar2) && (local_c8.st_mtim.tv_nsec == 0)) goto LAB_0010ee4f;
      }
      else {
        local_e8.tv_usec = 0;
        if ((bVar7 && !bVar2) && (local_c8.st_mtim.tv_nsec == 0)) {
LAB_0010ee4f:
          _Stack_d0 = 0;
        }
        futimesat(param_1,(char *)0x0,&local_e8);
      }
      iVar5 = 0;
      goto LAB_0010ec01;
    }
    piVar6 = __errno_location();
    *piVar6 = 9;
  }
  else {
    local_128.tv_sec = *param_3;
    local_128.tv_nsec = param_3[1];
    local_118 = param_3[2];
    _Stack_110 = param_3[3];
    local_130 = &local_128;
    iVar4 = FUN_0010e8d0(local_130);
    if (-1 < iVar4) goto LAB_0010eaaa;
  }
LAB_0010ebfb:
  iVar5 = -1;
LAB_0010ec01:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return iVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: lutimens @ 0xee90

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
  __time_t local_d8;
  long lStack_d0;
  stat local_c8;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 == (__time_t *)0x0) {
    iVar2 = 0;
    local_f0 = (timespec *)0x0;
    __times = (timespec *)0x0;
    if (-1 < DAT_001dfeb4) goto LAB_0010ef48;
LAB_0010f02c:
    DAT_001dfeb4 = -1;
    iVar2 = lstat(param_1,&local_c8);
    if (iVar2 == 0) {
LAB_0010efb3:
      if ((local_c8.st_mode & 0xf000) != 0xa000) {
        iVar3 = FUN_0010ea40(0xffffffff,param_1,local_f0);
        goto LAB_0010efdc;
      }
      piVar4 = __errno_location();
      *piVar4 = 0x26;
    }
  }
  else {
    local_e8.tv_sec = *param_2;
    local_e8.tv_nsec = param_2[1];
    local_d8 = param_2[2];
    lStack_d0 = param_2[3];
    __times = &local_e8;
    local_f0 = __times;
    iVar2 = FUN_0010e8d0(__times);
    if (-1 < iVar2) {
      if (-1 < DAT_001dfeb4) {
        if (iVar2 == 2) {
          iVar2 = lstat(param_1,&local_c8);
          if (iVar2 != 0) goto LAB_0010f048;
          if (local_e8.tv_nsec == 0x3ffffffe) {
            local_e8.tv_sec = local_c8.st_atim.tv_sec;
            iVar2 = 3;
            local_e8.tv_nsec = local_c8.st_atim.tv_nsec;
          }
          else {
            iVar2 = 3;
            if (lStack_d0 == 0x3ffffffe) {
              local_d8 = local_c8.st_mtim.tv_sec;
              lStack_d0 = local_c8.st_mtim.tv_nsec;
            }
          }
        }
LAB_0010ef48:
        iVar3 = utimensat(-100,param_1,__times,0x100);
        if (iVar3 < 1) {
          if ((iVar3 == 0) || (piVar4 = __errno_location(), *piVar4 != 0x26)) {
            DAT_001dfeb8 = 1;
            DAT_001dfeb4 = 1;
            goto LAB_0010efdc;
          }
        }
        else {
          piVar4 = __errno_location();
          *piVar4 = 0x26;
        }
      }
      DAT_001dfeb4 = -1;
      if (iVar2 == 0) goto LAB_0010f02c;
      if ((iVar2 != 3) && (iVar2 = lstat(param_1,&local_c8), iVar2 != 0)) goto LAB_0010f048;
      if (__times != (timespec *)0x0) {
        iVar3 = 0;
        cVar1 = FUN_0010e980(&local_c8,&local_f0);
        if (cVar1 != '\0') goto LAB_0010efdc;
      }
      goto LAB_0010efb3;
    }
  }
LAB_0010f048:
  iVar3 = -1;
LAB_0010efdc:
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return iVar3;
}




// Function: xmalloc @ 0xf160

void xmalloc(size_t param_1)

{
  void *pvVar1;
  
  pvVar1 = malloc(param_1);
  if (pvVar1 != (void *)0x0) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_0010d430();
}




// Function: ximalloc @ 0xf180

void ximalloc(size_t param_1)

{
  void *pvVar1;
  
  pvVar1 = malloc(param_1);
  if (pvVar1 != (void *)0x0) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_0010d430();
}




// Function: xirealloc @ 0xf1f0

void xirealloc(void *param_1,ulong param_2)

{
  void *pvVar1;
  
  pvVar1 = realloc(param_1,param_2 | param_2 == 0);
  if (pvVar1 != (void *)0x0) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_0010d430();
}




// Function: xnmalloc @ 0xf2a0

void xnmalloc(undefined8 param_1,undefined8 param_2)

{
  long lVar1;
  
  lVar1 = reallocarray(0,param_1,param_2);
  if (lVar1 != 0) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_0010d430();
}




// Function: xinmalloc @ 0xf2d0

void xinmalloc(long param_1,long param_2)

{
  long lVar1;
  
  if ((param_1 == 0) || (param_2 == 0)) {
    param_1 = 1;
    param_2 = 1;
  }
  lVar1 = reallocarray(0,param_1,param_2);
  if (lVar1 == 0) {
                    /* WARNING: Subroutine does not return */
    FUN_0010d430();
  }
  return;
}




// Function: x2realloc @ 0xf320

void x2realloc(long param_1,ulong *param_2)

{
  ulong uVar1;
  long lVar2;
  ulong uVar3;
  ulong uVar4;
  
  uVar4 = *param_2;
  if (param_1 == 0) {
    if (uVar4 == 0) {
      uVar4 = 0x80;
    }
    lVar2 = reallocarray(0,uVar4,1);
    if (lVar2 != 0) {
      *param_2 = uVar4;
      return;
    }
  }
  else {
    uVar1 = (uVar4 >> 1) + 1;
    uVar3 = uVar4 + uVar1;
    if (!CARRY8(uVar4,uVar1)) {
      lVar2 = reallocarray(param_1,uVar3,1);
      if ((lVar2 != 0) || (uVar3 == 0)) {
        *param_2 = uVar3;
        return;
      }
    }
  }
                    /* WARNING: Subroutine does not return */
  FUN_0010d430();
}




// Function: xpalloc @ 0xf440

void xpalloc(void *param_1,long *param_2,long param_3,long param_4,long param_5)

{
  long lVar1;
  void *pvVar2;
  long lVar3;
  size_t __size;
  long lVar4;
  
  lVar1 = *param_2;
  lVar3 = (lVar1 >> 1) + lVar1;
  if (SCARRY8(lVar1 >> 1,lVar1)) {
    lVar3 = 0x7fffffffffffffff;
  }
  lVar4 = param_4;
  if (lVar3 <= param_4) {
    lVar4 = lVar3;
  }
  if (-1 < param_4) {
    lVar3 = lVar4;
  }
  __size = lVar3 * param_5;
  if (SEXT816((long)__size) == SEXT816(lVar3) * SEXT816(param_5)) {
    if (0x7f < (long)__size) goto joined_r0x0010f4fe;
    lVar4 = 0x80;
  }
  else {
    lVar4 = 0x7fffffffffffffff;
  }
  lVar3 = lVar4 / param_5;
  __size = lVar4 - lVar4 % param_5;
joined_r0x0010f4fe:
  if (param_1 == (void *)0x0) {
    *param_2 = 0;
  }
  if (((param_3 <= lVar3 - lVar1) ||
      ((lVar3 = lVar1 + param_3, !SCARRY8(lVar1,param_3) &&
       (((lVar3 <= param_4 || (param_4 < 0)) &&
        (__size = lVar3 * param_5, SEXT816((long)__size) == SEXT816(lVar3) * SEXT816(param_5)))))))
     && ((pvVar2 = realloc(param_1,__size), pvVar2 != (void *)0x0 ||
         ((param_1 != (void *)0x0 && (__size == 0)))))) {
    *param_2 = lVar3;
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_0010d430();
}




// Function: xcalloc @ 0xf540

void xcalloc(size_t param_1,size_t param_2)

{
  void *pvVar1;
  
  pvVar1 = calloc(param_1,param_2);
  if (pvVar1 != (void *)0x0) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_0010d430();
}




// Function: xicalloc @ 0xf570

void xicalloc(size_t param_1,size_t param_2)

{
  void *pvVar1;
  
  pvVar1 = calloc(param_1,param_2);
  if (pvVar1 != (void *)0x0) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  FUN_0010d430();
}




// Function: xmemdup @ 0xf5a0

void xmemdup(void *param_1,size_t param_2)

{
  void *__dest;
  
  __dest = (void *)FUN_0010f160(param_2);
  memcpy(__dest,param_1,param_2);
  return;
}




// Function: ximemdup @ 0xf5d0

void ximemdup(void *param_1,size_t param_2)

{
  void *__dest;
  
  __dest = (void *)FUN_0010f180(param_2);
  memcpy(__dest,param_1,param_2);
  return;
}




// Function: ximemdup0 @ 0xf600

void ximemdup0(void *param_1,size_t param_2)

{
  void *__dest;
  
  __dest = (void *)FUN_0010f180(param_2 + 1);
  *(undefined1 *)((long)__dest + param_2) = 0;
  memcpy(__dest,param_1,param_2);
  return;
}




// Function: xstrdup @ 0xf640

void xstrdup(char *param_1)

{
  size_t sVar1;
  
  sVar1 = strlen(param_1);
  FUN_0010f5a0(param_1,sVar1 + 1);
  return;
}




// Function: yesno @ 0xf660

ulong yesno(void)

{
  uint uVar1;
  uint uVar2;
  undefined8 unaff_R12;
  
  uVar1 = getc(stdin);
  uVar2 = uVar1 & 0xffffffdf;
  while ((uVar1 != 10 && (uVar1 != 0xffffffff))) {
    uVar1 = getc(stdin);
  }
  return CONCAT71((int7)((ulong)unaff_R12 >> 8),uVar2 == 0x59) & 0xffffffff;
}




// Function: rpl_fprintf @ 0xf6b0

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
  __ptr = (undefined1 *)FUN_0010f900(local_8b8,&local_8d8,param_10,&local_8d0);
  sVar1 = local_8d8;
  if (__ptr != (undefined1 *)0x0) {
    sVar2 = fwrite(__ptr,1,local_8d8,param_9);
    if (sVar2 < sVar1) {
      uVar4 = 0xffffffff;
      if (__ptr != local_8b8) {
        free(__ptr);
        uVar4 = 0xffffffff;
      }
      goto LAB_0010f7c8;
    }
    if (__ptr != local_8b8) {
      free(__ptr);
    }
    if (sVar1 < 0x80000000) {
      uVar4 = sVar1 & 0xffffffff;
      goto LAB_0010f7c8;
    }
    piVar3 = __errno_location();
    *piVar3 = 0x4b;
  }
  FUN_00110b90(param_9);
  uVar4 = 0xffffffff;
LAB_0010f7c8:
  if (local_e0 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: __printf__ @ 0xf830

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
  FUN_00110a00(stdout,param_9,&local_d8);
  if (local_c0 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: vasnprintf @ 0xf900

int * vasnprintf(int *param_1,ulong *param_2,int *param_3,undefined8 param_4)

{
  undefined8 uVar1;
  unkbyte10 Var2;
  undefined1 uVar3;
  uint uVar4;
  long lVar5;
  void *pvVar6;
  ulong *puVar7;
  undefined1 *puVar8;
  int iVar9;
  ulong uVar10;
  ulong uVar11;
  int *__dest;
  int *piVar12;
  ulong uVar13;
  ulong uVar14;
  undefined1 *puVar15;
  undefined1 *puVar16;
  size_t __size;
  int *piVar17;
  ulong uVar18;
  undefined8 *puVar19;
  long in_FS_OFFSET;
  bool bVar20;
  undefined1 auStack_428 [8];
  ulong local_420;
  int local_414;
  undefined1 *local_410;
  ulong *local_408;
  int *local_400;
  long local_3f8;
  undefined1 *local_3f0;
  undefined8 local_3e8;
  int *local_3e0;
  int *local_3d8;
  int *local_3d0;
  int local_3c4;
  uint local_3c0 [2];
  undefined1 local_3b8 [8];
  undefined1 *local_3b0;
  undefined1 local_3a8 [224];
  long local_2c8;
  undefined8 *local_2c0;
  ulong local_2b8;
  ulong local_2b0;
  undefined8 local_2a8 [77];
  long local_40;
  
  puVar15 = auStack_428;
  puVar16 = auStack_428;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_408 = param_2;
  local_3d8 = param_3;
  local_3d0 = param_1;
  iVar9 = FUN_00110e50(param_3,&local_2c8,local_3b8);
  if (-1 < iVar9) {
    iVar9 = FUN_00110c20(param_4,local_3b8);
    piVar17 = local_3d8;
    if (iVar9 < 0) {
      if (local_2c0 != local_2a8) {
        free(local_2c0);
      }
      if (local_3b0 != local_3a8) {
        free(local_3b0);
      }
      piVar17 = __errno_location();
      __dest = (int *)0x0;
      *piVar17 = 0x16;
      puVar15 = auStack_428;
      goto LAB_0010faf0;
    }
    uVar10 = local_2b8 + 7;
    if (0xfffffffffffffff8 < local_2b8) {
      uVar10 = 0xffffffffffffffff;
    }
    uVar11 = uVar10 + local_2b0;
    if ((!CARRY8(uVar10,local_2b0)) && (__size = uVar11 + 6, uVar11 < 0xfffffffffffffffa)) {
      if (__size < 4000) {
        for (; puVar16 != auStack_428 + -(uVar11 + 0x1d & 0xfffffffffffff000);
            puVar16 = puVar16 + -0x1000) {
          *(undefined8 *)(puVar16 + -8) = *(undefined8 *)(puVar16 + -8);
        }
        uVar10 = (ulong)((uint)(uVar11 + 0x1d) & 0xff0);
        lVar5 = -uVar10;
        if (uVar10 != 0) {
          *(undefined8 *)(puVar16 + -8) = *(undefined8 *)(puVar16 + -8);
        }
        local_410 = (undefined1 *)0x0;
        local_3f0 = (undefined1 *)((ulong)(puVar16 + lVar5 + 0xf) & 0xfffffffffffffff0);
        puVar15 = puVar16 + lVar5;
        puVar8 = local_410;
LAB_0010f9f9:
        local_410 = puVar8;
        uVar10 = 0;
        if (local_3d0 != (int *)0x0) {
          uVar10 = *local_408;
        }
        local_3f8 = 0;
        piVar12 = (int *)*local_2c0;
        uVar11 = 0;
        __dest = local_3d0;
        uVar13 = uVar11;
        puVar19 = local_2c0;
        if (piVar12 == piVar17) goto LAB_0010fbd2;
        do {
          uVar18 = (long)piVar12 - (long)piVar17;
          uVar11 = uVar13 + uVar18;
          if (CARRY8(uVar13,uVar18)) {
            uVar11 = 0xffffffffffffffff;
          }
          if (uVar11 <= uVar10) goto LAB_0010fbb5;
          piVar12 = __dest;
          if (uVar10 != 0) {
            if (-1 < (long)uVar10) {
              uVar10 = uVar10 * 2;
              goto LAB_0010fb56;
            }
LAB_0010fa77:
            local_3e0 = piVar12;
            *(undefined8 *)(puVar15 + -8) = 0x10fa7c;
            local_3d8 = __errno_location();
            __dest = local_3e0;
LAB_0010fa8a:
            if ((__dest == local_3d0) || (__dest == (int *)0x0)) goto LAB_0010fa9c;
            goto LAB_00110020;
          }
          uVar10 = 0xc;
LAB_0010fb56:
          if (uVar10 < uVar11) {
            uVar10 = uVar11;
          }
          if (uVar10 == 0xffffffffffffffff) goto LAB_0010fa77;
          if ((__dest == (int *)0x0) || (__dest == local_3d0)) {
            local_3d8 = (int *)CONCAT71(local_3d8._1_7_,__dest == local_3d0);
            local_3e8 = piVar17;
            local_3e0 = __dest;
            *(undefined8 *)(puVar15 + -8) = 0x10ff6c;
            __dest = malloc(uVar10);
            piVar12 = local_3e0;
            if (__dest == (int *)0x0) goto LAB_0010fa77;
            piVar17 = local_3e8;
            if ((uVar13 != 0) && ((char)local_3d8 != '\0')) {
              local_3d8 = local_3e8;
              *(undefined8 *)(puVar15 + -8) = 0x10ffb3;
              __dest = memcpy(__dest,piVar12,uVar13);
              piVar17 = local_3d8;
            }
          }
          else {
            local_3e0 = piVar17;
            local_3d8 = __dest;
            *(undefined8 *)(puVar15 + -8) = 0x10fb9b;
            __dest = realloc(__dest,uVar10);
            piVar17 = local_3e0;
            if (__dest == (int *)0x0) goto LAB_0010ffff;
          }
LAB_0010fbb5:
          local_3d8 = __dest;
          *(undefined8 *)(puVar15 + -8) = 0x10fbcb;
          memcpy((void *)((long)__dest + uVar13),piVar17,uVar18);
          __dest = local_3d8;
LAB_0010fbd2:
          do {
            puVar8 = local_3b0;
            piVar12 = __dest;
            if (local_2c8 == local_3f8) {
              uVar13 = uVar11 + 1;
              if (uVar11 < 0xffffffffffffffff) {
                if (uVar10 < uVar13) {
                  if (uVar10 == 0) {
                    if (uVar13 < 0xd) {
                      uVar10 = 0xc;
                    }
                    else {
LAB_001109be:
                      uVar10 = uVar13;
                      if (uVar13 == 0xffffffffffffffff) goto LAB_0010fa77;
                    }
                  }
                  else {
                    if ((long)uVar10 < 0) goto LAB_0010fa77;
                    uVar10 = uVar10 * 2;
                    if (uVar10 < uVar13) goto LAB_001109be;
                  }
                  bVar20 = __dest == local_3d0;
                  if ((__dest == (int *)0x0) || (bVar20)) {
                    local_3d8 = __dest;
                    *(undefined8 *)(puVar15 + -8) = 0x110979;
                    __dest = malloc(uVar10);
                    piVar17 = local_3d8;
                    piVar12 = local_3d8;
                    if (__dest == (int *)0x0) goto LAB_0010fa77;
                    if ((uVar11 != 0) && (bVar20)) {
                      *(undefined8 *)(puVar15 + -8) = 0x1109a4;
                      __dest = memcpy(__dest,piVar17,uVar11);
                    }
                  }
                  else {
                    local_3d8 = __dest;
                    *(undefined8 *)(puVar15 + -8) = 0x1108aa;
                    __dest = realloc(__dest,uVar10);
                    if (__dest == (int *)0x0) goto LAB_0010ffff;
                  }
                }
              }
              else if (uVar10 != 0xffffffffffffffff) goto LAB_0010fa77;
              *(undefined1 *)((long)__dest + uVar11) = 0;
              if ((uVar13 < uVar10) && (__dest != local_3d0)) {
                local_3d0 = __dest;
                *(undefined8 *)(puVar15 + -8) = 0x1108e2;
                piVar17 = realloc(__dest,uVar13);
                __dest = local_3d0;
                if (piVar17 != (int *)0x0) {
                  __dest = piVar17;
                }
              }
              puVar8 = local_410;
              if (local_410 != (undefined1 *)0x0) {
                local_3d0 = __dest;
                *(undefined8 *)(puVar15 + -8) = 0x110908;
                free(puVar8);
                __dest = local_3d0;
              }
              if (local_2c0 != local_2a8) {
                local_3d0 = __dest;
                *(undefined8 *)(puVar15 + -8) = 0x11092e;
                free(local_2c0);
                __dest = local_3d0;
              }
              if (local_3b0 != local_3a8) {
                local_3d0 = __dest;
                *(undefined8 *)(puVar15 + -8) = 0x110954;
                free(local_3b0);
                __dest = local_3d0;
              }
              *local_408 = uVar11;
              goto LAB_0010faf0;
            }
            lVar5 = puVar19[10];
            if (*(char *)(puVar19 + 9) == '%') {
              if (lVar5 != -1) goto switchD_0010fe99_caseD_0;
              uVar13 = uVar11 + 1;
              if (uVar11 == 0xffffffffffffffff) {
                uVar13 = 0xffffffffffffffff;
              }
              if (uVar10 < uVar13) {
                if (uVar10 == 0) {
                  uVar10 = 0xc;
                }
                else {
                  if ((long)uVar10 < 0) goto LAB_0010fa77;
                  uVar10 = uVar10 * 2;
                }
                if (uVar10 < uVar13) {
                  uVar10 = uVar13;
                }
                if (uVar10 == 0xffffffffffffffff) goto LAB_0010fa77;
                bVar20 = __dest == local_3d0;
                if ((__dest == (int *)0x0) || (bVar20)) {
                  local_3d8 = __dest;
                  *(undefined8 *)(puVar15 + -8) = 0x11063f;
                  __dest = malloc(uVar10);
                  piVar17 = local_3d8;
                  piVar12 = local_3d8;
                  if (__dest == (int *)0x0) goto LAB_0010fa77;
                  if ((uVar11 != 0) && (bVar20)) {
                    *(undefined8 *)(puVar15 + -8) = 0x110672;
                    __dest = memcpy(__dest,piVar17,uVar11);
                  }
                }
                else {
                  local_3d8 = __dest;
                  *(undefined8 *)(puVar15 + -8) = 0x10ff11;
                  __dest = realloc(__dest,uVar10);
                  if (__dest == (int *)0x0) goto LAB_0010ffff;
                }
              }
              *(undefined1 *)((long)__dest + uVar11) = 0x25;
              uVar11 = uVar13;
            }
            else {
              if (lVar5 == -1) {
switchD_0010fe99_caseD_0:
                    /* WARNING: Subroutine does not return */
                *(undefined8 *)(puVar15 + -8) = 0x1109d5;
                abort();
              }
              iVar9 = *(int *)(local_3b0 + lVar5 * 0x20);
              local_3e0 = (int *)CONCAT44(local_3e0._4_4_,iVar9);
              if (*(char *)(puVar19 + 9) == 'n') {
                if (4 < iVar9 - 0x12U) goto switchD_0010fe99_caseD_0;
                puVar7 = *(ulong **)((long)(local_3b0 + lVar5 * 0x20) + 0x10);
                switch(iVar9) {
                case 0x12:
                  *(char *)puVar7 = (char)uVar11;
                  break;
                case 0x13:
                  *(short *)puVar7 = (short)uVar11;
                  break;
                case 0x14:
                  *(int *)puVar7 = (int)uVar11;
                  break;
                default:
                  *puVar7 = uVar11;
                }
              }
              else {
                uVar4 = *(uint *)(puVar19 + 2);
                piVar17 = (int *)(local_3f0 + 1);
                *local_3f0 = 0x25;
                if ((uVar4 & 1) != 0) {
                  local_3f0[1] = 0x27;
                  piVar17 = (int *)(local_3f0 + 2);
                }
                if ((uVar4 & 2) != 0) {
                  *(undefined1 *)piVar17 = 0x2d;
                  piVar17 = (int *)((long)piVar17 + 1);
                }
                if ((uVar4 & 4) != 0) {
                  *(undefined1 *)piVar17 = 0x2b;
                  piVar17 = (int *)((long)piVar17 + 1);
                }
                if ((uVar4 & 8) != 0) {
                  *(undefined1 *)piVar17 = 0x20;
                  piVar17 = (int *)((long)piVar17 + 1);
                }
                if ((uVar4 & 0x10) != 0) {
                  *(undefined1 *)piVar17 = 0x23;
                  piVar17 = (int *)((long)piVar17 + 1);
                }
                if ((uVar4 & 0x40) != 0) {
                  *(undefined1 *)piVar17 = 0x49;
                  piVar17 = (int *)((long)piVar17 + 1);
                }
                if ((uVar4 & 0x20) != 0) {
                  *(undefined1 *)piVar17 = 0x30;
                  piVar17 = (int *)((long)piVar17 + 1);
                }
                pvVar6 = (void *)puVar19[3];
                if (pvVar6 != (void *)puVar19[4]) {
                  piVar12 = (int *)((long)puVar19[4] - (long)pvVar6);
                  local_3e8 = __dest;
                  local_3d8 = piVar12;
                  *(undefined8 *)(puVar15 + -8) = 0x10fcbd;
                  memcpy(piVar17,pvVar6,(size_t)piVar12);
                  piVar17 = (int *)((long)piVar17 + (long)local_3d8);
                  __dest = local_3e8;
                }
                pvVar6 = (void *)puVar19[6];
                if (pvVar6 != (void *)puVar19[7]) {
                  piVar12 = (int *)((long)puVar19[7] - (long)pvVar6);
                  local_3e8 = __dest;
                  local_3d8 = piVar12;
                  *(undefined8 *)(puVar15 + -8) = 0x10fcf7;
                  memcpy(piVar17,pvVar6,(size_t)piVar12);
                  piVar17 = (int *)((long)piVar17 + (long)local_3d8);
                  __dest = local_3e8;
                }
                switch(local_3e0._0_4_) {
                case 9:
                case 10:
                  *(undefined1 *)piVar17 = 0x6c;
                  piVar17 = (int *)((long)piVar17 + 1);
                case 7:
                case 8:
                case 0xe:
                case 0x10:
                  *(undefined1 *)piVar17 = 0x6c;
                  piVar17 = (int *)((long)piVar17 + 1);
                  break;
                case 0xc:
                  *(undefined1 *)piVar17 = 0x4c;
                  piVar17 = (int *)((long)piVar17 + 1);
                }
                uVar3 = *(undefined1 *)(puVar19 + 9);
                *(undefined1 *)((long)piVar17 + 1) = 0;
                *(undefined1 *)piVar17 = uVar3;
                if (puVar19[5] == -1) {
                  local_3e8 = (int *)((ulong)local_3e8._4_4_ << 0x20);
                }
                else {
                  piVar12 = (int *)(puVar8 + puVar19[5] * 0x20);
                  if (*piVar12 != 5) goto switchD_0010fe99_caseD_0;
                  local_3e8 = (int *)CONCAT44(local_3e8._4_4_,1);
                  local_3c0[0] = piVar12[4];
                }
                if (puVar19[8] != -1) {
                  piVar12 = (int *)(puVar8 + puVar19[8] * 0x20);
                  if (*piVar12 != 5) goto switchD_0010fe99_caseD_0;
                  local_3c0[(ulong)local_3e8 & 0xffffffff] = piVar12[4];
                  local_3e8 = (int *)CONCAT44(local_3e8._4_4_,(int)local_3e8 + 1);
                }
                uVar13 = uVar11 + 2;
                piVar12 = __dest;
                if (uVar11 < 0xfffffffffffffffe) {
                  if (uVar10 < uVar13) {
                    if (uVar10 == 0) {
                      if (uVar13 < 0xd) {
                        uVar10 = 0xc;
                      }
                      else {
LAB_001103e2:
                        uVar10 = uVar13;
                        if (uVar13 == 0xffffffffffffffff) goto LAB_0010fa77;
                      }
                    }
                    else {
                      if ((long)uVar10 < 0) goto LAB_0010fa77;
                      uVar10 = uVar10 * 2;
                      if (uVar10 < uVar13) goto LAB_001103e2;
                    }
                    if ((__dest == (int *)0x0) || (__dest == local_3d0)) {
                      local_3d8 = (int *)CONCAT71(local_3d8._1_7_,__dest == local_3d0);
                      local_400 = __dest;
                      *(undefined8 *)(puVar15 + -8) = 0x1105e5;
                      __dest = malloc(uVar10);
                      piVar12 = local_400;
                      if (__dest == (int *)0x0) goto LAB_0010fa77;
                      if ((uVar11 != 0) && ((char)local_3d8 != '\0')) {
                        *(undefined8 *)(puVar15 + -8) = 0x11061e;
                        memcpy(__dest,piVar12,uVar11);
                      }
                    }
                    else {
                      local_3d8 = __dest;
                      *(undefined8 *)(puVar15 + -8) = 0x10fe12;
                      __dest = realloc(__dest,uVar10);
                      piVar12 = local_3d8;
                      if (__dest == (int *)0x0) goto LAB_0010fa77;
                    }
                  }
                }
                else if (uVar10 != 0xffffffffffffffff) goto LAB_0010fa77;
                *(undefined1 *)((long)__dest + uVar11) = 0;
                *(undefined8 *)(puVar15 + -8) = 0x10fe2f;
                local_3d8 = __errno_location();
                local_414 = *local_3d8;
                piVar12 = __dest;
                local_400 = piVar17;
LAB_0010fe50:
                __dest = piVar12;
                uVar4 = local_3c0[0];
                puVar8 = local_3f0;
                local_3c4 = -1;
                uVar18 = uVar10 - uVar11;
                *local_3d8 = 0;
                uVar13 = 0x7fffffff;
                if (uVar18 < 0x80000000) {
                  uVar13 = uVar18;
                }
                piVar12 = __dest;
                switch((ulong)local_3e0 & 0xffffffff) {
                default:
                  goto switchD_0010fe99_caseD_0;
                case 1:
                  uVar14 = (ulong)(uint)(int)(char)local_3b0[puVar19[10] * 0x20 + 0x10];
                  break;
                case 2:
                  uVar14 = (ulong)(byte)local_3b0[puVar19[10] * 0x20 + 0x10];
                  break;
                case 3:
                  uVar14 = (ulong)(uint)(int)*(short *)(local_3b0 + puVar19[10] * 0x20 + 0x10);
                  break;
                case 4:
                  uVar14 = (ulong)*(ushort *)(local_3b0 + puVar19[10] * 0x20 + 0x10);
                  break;
                case 5:
                case 6:
                case 0xd:
                case 0xe:
                  uVar14 = (ulong)*(uint *)(local_3b0 + puVar19[10] * 0x20 + 0x10);
                  break;
                case 7:
                case 8:
                case 9:
                case 10:
                case 0xf:
                case 0x10:
                case 0x11:
                  piVar17 = *(int **)(local_3b0 + puVar19[10] * 0x20 + 0x10);
                  if ((int)local_3e8 == 1) {
                    local_420 = uVar18;
                    *(int **)(puVar15 + -8) = &local_3c4;
                    *(int **)(puVar15 + -0x10) = piVar17;
                    goto LAB_0011056a;
                  }
                  if ((int)local_3e8 != 2) {
                    local_420 = uVar18;
                    *(int **)(puVar15 + -0x10) = &local_3c4;
LAB_00110072:
                    puVar8 = local_3f0;
                    *(undefined8 *)(puVar15 + -0x18) = 0x11008f;
                    iVar9 = __snprintf_chk((long)__dest + uVar11,uVar13,1,0xffffffffffffffff,puVar8,
                                           piVar17);
                    goto LAB_00110098;
                  }
                  local_420 = uVar18;
                  *(int **)(puVar15 + -0x10) = &local_3c4;
                  *(int **)(puVar15 + -0x18) = piVar17;
                  goto LAB_0011051e;
                case 0xb:
                  uVar1 = *(undefined8 *)(local_3b0 + puVar19[10] * 0x20 + 0x10);
                  if ((int)local_3e8 == 1) {
                    local_420 = uVar18;
                    *(int **)(puVar15 + -0x10) = &local_3c4;
                  }
                  else {
                    if ((int)local_3e8 != 2) {
                      local_420 = uVar18;
                      *(undefined8 *)(puVar15 + -8) = 0x11033b;
                      iVar9 = __snprintf_chk(uVar1,(long)__dest + uVar11,uVar13,1,0xffffffffffffffff
                                             ,puVar8,&local_3c4);
                      goto LAB_00110098;
                    }
                    local_420 = uVar18;
                    *(int **)(puVar15 + -8) = &local_3c4;
                    *(ulong *)(puVar15 + -0x10) = (ulong)local_3c0[1];
                  }
                  uVar4 = local_3c0[0];
                  puVar8 = local_3f0;
                  *(undefined8 *)(puVar15 + -0x18) = 0x110776;
                  iVar9 = __snprintf_chk(uVar1,(long)__dest + uVar11,uVar13,1,0xffffffffffffffff,
                                         puVar8,uVar4);
                  goto LAB_00110098;
                case 0xc:
                  Var2 = *(unkbyte10 *)(local_3b0 + puVar19[10] * 0x20 + 0x10);
                  if ((int)local_3e8 != 1) {
                    if ((int)local_3e8 != 2) {
                      piVar17 = &local_3c4;
                      local_420 = uVar18;
                      *(unkbyte10 *)(puVar15 + -0x10) = Var2;
                      goto LAB_00110072;
                    }
                    local_420 = uVar18;
                    *(int **)(puVar15 + -0x10) = &local_3c4;
                    uVar18 = (ulong)local_3c0[1];
                    *(unkbyte10 *)(puVar15 + -0x20) = Var2;
                    *(ulong *)(puVar15 + -0x30) = uVar18;
                    *(undefined8 *)(puVar15 + -0x38) = 0x110714;
                    iVar9 = __snprintf_chk((long)__dest + uVar11,uVar13,1,0xffffffffffffffff,puVar8,
                                           uVar4);
                    goto LAB_00110098;
                  }
                  local_420 = uVar18;
                  *(int **)(puVar15 + -0x10) = &local_3c4;
                  *(unkbyte10 *)(puVar15 + -0x20) = Var2;
                  goto LAB_00110525;
                }
                if ((int)local_3e8 == 1) {
                  local_420 = uVar18;
                  *(int **)(puVar15 + -8) = &local_3c4;
                  *(ulong *)(puVar15 + -0x10) = uVar14;
LAB_0011056a:
                  uVar14 = (ulong)local_3c0[0];
LAB_001101b2:
                  puVar8 = local_3f0;
                  *(undefined8 *)(puVar15 + -0x18) = 0x1101cf;
                  iVar9 = __snprintf_chk((long)__dest + uVar11,uVar13,1,0xffffffffffffffff,puVar8,
                                         uVar14);
                  if (-1 < local_3c4) goto LAB_001100a6;
LAB_001101e6:
                  if (*(char *)((long)local_400 + 1) == '\0') {
                    if (-1 < iVar9) goto LAB_001100bf;
                    if (*local_3d8 == 0) {
                      iVar9 = 0x16;
                      if ((*(byte *)(puVar19 + 9) & 0xef) == 99) {
                        iVar9 = 0x54;
                      }
                      *local_3d8 = iVar9;
                    }
                    if ((__dest != local_3d0) && (__dest != (int *)0x0)) {
                      *(undefined8 *)(puVar15 + -8) = 0x110245;
                      free(__dest);
                    }
                    puVar8 = local_410;
                    if (local_410 != (undefined1 *)0x0) {
                      *(undefined8 *)(puVar15 + -8) = 0x110259;
                      free(puVar8);
                    }
                    if (local_2c0 != local_2a8) {
                      *(undefined8 *)(puVar15 + -8) = 0x110271;
                      free(local_2c0);
                    }
                    if (local_3b0 != local_3a8) {
                      *(undefined8 *)(puVar15 + -8) = 0x11028d;
                      free(local_3b0);
                    }
                    goto LAB_0010faed;
                  }
                  *(undefined1 *)((long)local_400 + 1) = 0;
                  goto LAB_0010fe50;
                }
                if ((int)local_3e8 != 2) {
                  local_420 = uVar18;
                  *(int **)(puVar15 + -0x10) = &local_3c4;
                  goto LAB_001101b2;
                }
                local_420 = uVar18;
                *(int **)(puVar15 + -0x10) = &local_3c4;
                *(ulong *)(puVar15 + -0x18) = uVar14;
LAB_0011051e:
                *(ulong *)(puVar15 + -0x20) = (ulong)local_3c0[1];
LAB_00110525:
                uVar4 = local_3c0[0];
                puVar8 = local_3f0;
                *(undefined8 *)(puVar15 + -0x28) = 0x110549;
                iVar9 = __snprintf_chk((long)__dest + uVar11,uVar13,1,0xffffffffffffffff,puVar8,
                                       uVar4);
LAB_00110098:
                if (local_3c4 < 0) goto LAB_001101e6;
LAB_001100a6:
                if (((ulong)(long)local_3c4 < uVar13) &&
                   (*(char *)((long)__dest + uVar11 + (long)local_3c4) != '\0'))
                goto switchD_0010fe99_caseD_0;
                if (local_3c4 < iVar9) {
LAB_001100bf:
                  local_3c4 = iVar9;
                }
                if (uVar13 <= local_3c4 + 1) {
                  if (0x7ffffffe < local_420) {
                    if ((__dest != local_3d0) && (__dest != (int *)0x0)) {
                      *(undefined8 *)(puVar15 + -8) = 0x1107de;
                      free(__dest);
                    }
                    puVar8 = local_410;
                    if (local_410 != (undefined1 *)0x0) {
                      *(undefined8 *)(puVar15 + -8) = 0x1107f2;
                      free(puVar8);
                    }
                    if (local_2c0 != local_2a8) {
                      *(undefined8 *)(puVar15 + -8) = 0x11080a;
                      free(local_2c0);
                    }
                    if (local_3b0 != local_3a8) {
                      *(undefined8 *)(puVar15 + -8) = 0x110822;
                      free(local_3b0);
                    }
                    __dest = (int *)0x0;
                    *local_3d8 = 0x4b;
                    goto LAB_0010faf0;
                  }
                  uVar13 = (ulong)(local_3c4 + 2);
                  if ((long)uVar10 < 0) {
                    if (CARRY8(uVar13,uVar11)) {
                      if (uVar10 != 0xffffffffffffffff) goto LAB_0010fa8a;
                      goto LAB_0010fe50;
                    }
                    uVar14 = 0xffffffffffffffff;
                    uVar18 = 0xffffffffffffffff;
                  }
                  else {
                    uVar18 = uVar10 * 2;
                    if (CARRY8(uVar13,uVar11)) goto LAB_0010fa8a;
                    uVar14 = uVar13 + uVar11;
                    if (uVar13 + uVar11 <= uVar18) {
                      uVar14 = uVar18;
                    }
                  }
                  if (uVar10 < uVar14) {
                    if (uVar10 == 0) {
                      uVar18 = 0xc;
                    }
                    uVar10 = uVar14;
                    if (uVar14 <= uVar18) {
                      uVar10 = uVar18;
                    }
                    if (uVar10 == 0xffffffffffffffff) goto LAB_0010fa8a;
                    bVar20 = __dest == local_3d0;
                    if ((__dest == (int *)0x0) || (bVar20)) {
                      *(undefined8 *)(puVar15 + -8) = 0x1104e0;
                      piVar12 = malloc(uVar10);
                      if (piVar12 == (int *)0x0) goto LAB_0010fa8a;
                      if ((uVar11 != 0) && (bVar20)) {
                        *(undefined8 *)(puVar15 + -8) = 0x110508;
                        piVar12 = memcpy(piVar12,__dest,uVar11);
                      }
                    }
                    else {
                      *(undefined8 *)(puVar15 + -8) = 0x11015a;
                      piVar12 = realloc(__dest,uVar10);
                      if (piVar12 == (int *)0x0) goto LAB_0010fa8a;
                    }
                  }
                  goto LAB_0010fe50;
                }
                uVar11 = (long)local_3c4 + uVar11;
                *local_3d8 = local_414;
              }
            }
            piVar17 = (int *)puVar19[1];
            piVar12 = (int *)puVar19[0xb];
            puVar19 = puVar19 + 0xb;
            local_3f8 = local_3f8 + 1;
            uVar13 = uVar11;
          } while (piVar12 == piVar17);
        } while( true );
      }
      if ((__size != 0xffffffffffffffff) &&
         (local_3f0 = malloc(__size), puVar15 = auStack_428, piVar17 = local_3d8, puVar8 = local_3f0
         , local_3f0 != (undefined1 *)0x0)) goto LAB_0010f9f9;
    }
    local_3d8 = __errno_location();
    puVar15 = auStack_428;
    goto LAB_0010fab0;
  }
  goto LAB_0010faed;
LAB_0010ffff:
  local_3d0 = local_3d8;
  *(undefined8 *)(puVar15 + -8) = 0x11000b;
  local_3d8 = __errno_location();
  __dest = local_3d0;
LAB_00110020:
  *(undefined8 *)(puVar15 + -8) = 0x110028;
  free(__dest);
LAB_0010fa9c:
  puVar8 = local_410;
  if (local_410 != (undefined1 *)0x0) {
    *(undefined8 *)(puVar15 + -8) = 0x10fab0;
    free(puVar8);
  }
LAB_0010fab0:
  if (local_2c0 != local_2a8) {
    *(undefined8 *)(puVar15 + -8) = 0x10fac8;
    free(local_2c0);
  }
  if (local_3b0 != local_3a8) {
    *(undefined8 *)(puVar15 + -8) = 0x10fae0;
    free(local_3b0);
  }
  *local_3d8 = 0xc;
LAB_0010faed:
  __dest = (int *)0x0;
LAB_0010faf0:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return __dest;
  }
                    /* WARNING: Subroutine does not return */
  *(undefined8 *)(puVar15 + -8) = 0x1109e2;
  __stack_chk_fail();
}




// Function: rpl_vfprintf @ 0x10a00

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
  __ptr = (undefined1 *)FUN_0010f900(local_808,&local_810,param_2,param_3);
  sVar1 = local_810;
  if (__ptr != (undefined1 *)0x0) {
    sVar2 = fwrite(__ptr,1,local_810,param_1);
    if (sVar2 < sVar1) {
      uVar4 = 0xffffffff;
      if (__ptr != local_808) {
        free(__ptr);
        uVar4 = 0xffffffff;
      }
      goto LAB_00110a8c;
    }
    if (__ptr != local_808) {
      free(__ptr);
    }
    if (sVar1 < 0x80000000) {
      uVar4 = sVar1 & 0xffffffff;
      goto LAB_00110a8c;
    }
    piVar3 = __errno_location();
    *piVar3 = 0x4b;
  }
  FUN_00110b90(param_1);
  uVar4 = 0xffffffff;
LAB_00110a8c:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: opendir_safer @ 0x10af0

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
      __fd = FUN_0010e0f0(uVar1,0x406,3);
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




// Function: fseterr @ 0x10b90

void fseterr(uint *param_1)

{
  *param_1 = *param_1 | 0x20;
  return;
}




// Function: current_timespec @ 0x10bb0

__time_t current_timespec(void)

{
  long in_FS_OFFSET;
  timespec local_28;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  clock_gettime(0,&local_28);
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_28.tv_sec;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: dup_safer @ 0x10c00

void dup_safer(undefined8 param_1)

{
  FUN_0010e0f0(param_1,0,3);
  return;
}




// Function: printf_fetchargs @ 0x10c20

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




// Function: printf_parse @ 0x10e50

undefined8 printf_parse(char *param_1,ulong *param_2,ulong *param_3)

{
  byte bVar1;
  ulong *__src;
  ulong *__src_00;
  char *pcVar2;
  int iVar3;
  ulong *puVar4;
  ulong *puVar5;
  int *piVar6;
  ulong uVar7;
  ulong *puVar8;
  ulong *__dest;
  char *pcVar9;
  int iVar10;
  uint uVar11;
  ulong uVar12;
  ulong uVar13;
  ulong uVar14;
  char *pcVar15;
  char cVar16;
  ulong uVar17;
  ulong uVar18;
  ulong uVar19;
  bool bVar20;
  ulong local_78;
  ulong local_70;
  ulong local_68;
  
  __src = param_2 + 4;
  __src_00 = param_3 + 2;
  uVar17 = 7;
  *param_2 = 0;
  param_2[1] = (ulong)__src;
  uVar12 = 0;
  *param_3 = 0;
  param_3[1] = (ulong)__src_00;
  local_70 = 0;
  local_78 = 0;
  local_68 = 0;
  puVar5 = __src;
  uVar18 = 7;
LAB_00110ecc:
  do {
    pcVar15 = param_1;
    if (*pcVar15 == '\0') {
      puVar5[uVar12 * 0xb] = (ulong)pcVar15;
      param_2[2] = local_78;
      param_2[3] = local_70;
      return 0;
    }
    param_1 = pcVar15 + 1;
  } while (*pcVar15 != '%');
  uVar19 = 0xffffffffffffffff;
  puVar5 = puVar5 + uVar12 * 0xb;
  *puVar5 = (ulong)pcVar15;
  *(undefined4 *)(puVar5 + 2) = 0;
  puVar5[3] = 0;
  puVar5[4] = 0;
  puVar5[5] = 0xffffffffffffffff;
  puVar5[6] = 0;
  puVar5[7] = 0;
  puVar5[8] = 0xffffffffffffffff;
  puVar5[10] = 0xffffffffffffffff;
  cVar16 = pcVar15[1];
  pcVar9 = param_1;
  if (9 < (byte)(cVar16 - 0x30U)) {
LAB_00110f77:
    pcVar15 = param_1 + 1;
    if (cVar16 == '\'') goto LAB_00111021;
    do {
      switch(cVar16) {
      case ' ':
        uVar11 = (uint)puVar5[2] | 8;
        param_1 = pcVar15;
        break;
      default:
        goto switchD_00110fa1_caseD_21;
      case '#':
        uVar11 = (uint)puVar5[2] | 0x10;
        param_1 = pcVar15;
        break;
      case '+':
        uVar11 = (uint)puVar5[2] | 4;
        param_1 = pcVar15;
        break;
      case '-':
        uVar11 = (uint)puVar5[2] | 2;
        param_1 = pcVar15;
        break;
      case '0':
        uVar11 = (uint)puVar5[2] | 0x20;
        param_1 = pcVar15;
        break;
      case 'I':
        uVar11 = (uint)puVar5[2] | 0x40;
        param_1 = pcVar15;
      }
      while( true ) {
        *(uint *)(puVar5 + 2) = uVar11;
        cVar16 = *param_1;
        pcVar15 = param_1 + 1;
        if (cVar16 != '\'') break;
LAB_00111021:
        uVar11 = (uint)puVar5[2] | 1;
        param_1 = pcVar15;
      }
    } while( true );
  }
  do {
    pcVar2 = pcVar9 + 1;
    pcVar9 = pcVar9 + 1;
  } while ((byte)(*pcVar2 - 0x30U) < 10);
  uVar19 = 0xffffffffffffffff;
  if (*pcVar2 != '$') goto LAB_00110f77;
  pcVar15 = pcVar15 + 2;
  uVar19 = 0;
  while( true ) {
    param_1 = pcVar15 + -1;
    uVar12 = (ulong)(char)(cVar16 + -0x30);
    if (uVar19 < 0x199999999999999a) {
      uVar7 = uVar19 * 10;
    }
    else {
      uVar7 = 0xffffffffffffffff;
    }
    while( true ) {
      pcVar9 = pcVar15;
      cVar16 = *pcVar9;
      uVar19 = uVar7 + uVar12;
      bVar1 = cVar16 - 0x30;
      if (!CARRY8(uVar7,uVar12)) break;
      if (9 < bVar1) goto switchD_0011107c_caseD_26;
      uVar12 = (ulong)(char)bVar1;
      uVar7 = 0xffffffffffffffff;
      pcVar15 = pcVar9 + 1;
      param_1 = pcVar9;
    }
    if (9 < bVar1) break;
    pcVar15 = pcVar9 + 1;
  }
  uVar19 = uVar19 - 1;
  if (uVar19 < 0xfffffffffffffffe) {
    cVar16 = param_1[2];
    param_1 = param_1 + 2;
    goto LAB_00110f77;
  }
  goto switchD_0011107c_caseD_26;
switchD_00110fa1_caseD_21:
  if (cVar16 == '*') {
    puVar5[3] = (ulong)param_1;
    puVar5[4] = (ulong)pcVar15;
    cVar16 = param_1[1];
    uVar12 = 1;
    if (local_78 != 0) {
      uVar12 = local_78;
    }
    pcVar9 = pcVar15;
    if ((byte)(cVar16 - 0x30U) < 10) {
      do {
        pcVar2 = pcVar9 + 1;
        pcVar9 = pcVar9 + 1;
      } while ((byte)(*pcVar2 - 0x30U) < 10);
      if (*pcVar2 != '$') goto LAB_001110d5;
      param_1 = param_1 + 2;
      uVar7 = 0;
      while( true ) {
        pcVar15 = param_1 + -1;
        uVar13 = (ulong)(char)(cVar16 + -0x30);
        if (uVar7 < 0x199999999999999a) {
          uVar14 = uVar7 * 10;
        }
        else {
          uVar14 = 0xffffffffffffffff;
        }
        while( true ) {
          uVar7 = uVar14 + uVar13;
          cVar16 = *param_1;
          bVar1 = cVar16 - 0x30;
          if (!CARRY8(uVar14,uVar13)) break;
          if (9 < bVar1) goto switchD_0011107c_caseD_26;
          uVar13 = (ulong)(char)bVar1;
          uVar14 = 0xffffffffffffffff;
          pcVar15 = param_1;
          param_1 = param_1 + 1;
        }
        if (9 < bVar1) break;
        param_1 = param_1 + 1;
      }
      uVar7 = uVar7 - 1;
      if (0xfffffffffffffffd < uVar7) goto switchD_0011107c_caseD_26;
      puVar5[5] = uVar7;
      pcVar15 = pcVar15 + 2;
    }
    else {
LAB_001110d5:
      puVar5[5] = local_68;
      bVar20 = local_68 == 0xffffffffffffffff;
      uVar7 = local_68;
      local_68 = local_68 + 1;
      if (bVar20) goto switchD_0011107c_caseD_26;
    }
    puVar8 = (ulong *)param_3[1];
    __dest = puVar8;
    if (uVar17 <= uVar7) {
      uVar17 = uVar17 * 2;
      if (uVar17 <= uVar7) {
        uVar17 = uVar7 + 1;
      }
      if (uVar17 >> 0x3b != 0) goto LAB_001116ab;
      if (__src_00 == puVar8) {
        __dest = malloc(uVar17 << 5);
      }
      else {
        __dest = realloc(puVar8,uVar17 << 5);
      }
      puVar8 = (ulong *)param_3[1];
      if (__dest == (ulong *)0x0) goto LAB_001116ab;
      if (__src_00 == puVar8) {
        __dest = memcpy(__dest,__src_00,*param_3 << 5);
      }
      param_3[1] = (ulong)__dest;
    }
    uVar13 = *param_3;
    puVar8 = __dest + uVar13 * 4;
    if (uVar13 <= uVar7) {
      do {
        puVar4 = puVar8;
        uVar13 = uVar13 + 1;
        *(undefined4 *)puVar4 = 0;
        puVar8 = puVar4 + 4;
      } while (uVar13 <= uVar7);
      *param_3 = uVar13;
      *(undefined4 *)puVar4 = 0;
    }
    iVar10 = (int)__dest[uVar7 * 4];
    local_78 = uVar12;
    if (iVar10 != 0) {
      if (iVar10 != 5) goto LAB_00111464;
      goto LAB_001117ec;
    }
    *(int *)(__dest + uVar7 * 4) = 5;
    cVar16 = *pcVar15;
    uVar12 = local_68;
  }
  else {
    pcVar15 = param_1;
    uVar12 = local_68;
    if ((byte)(cVar16 - 0x30U) < 10) {
      puVar5[3] = (ulong)param_1;
      uVar12 = local_78;
      if ((byte)(*param_1 - 0x30U) < 10) {
        do {
          pcVar9 = pcVar15 + 1;
          pcVar15 = pcVar15 + 1;
        } while ((byte)(*pcVar9 - 0x30U) < 10);
        uVar12 = (long)pcVar15 - (long)param_1;
        if ((ulong)((long)pcVar15 - (long)param_1) <= local_78) {
          uVar12 = local_78;
        }
      }
      local_78 = uVar12;
      puVar5[4] = (ulong)pcVar15;
LAB_001117ec:
      cVar16 = *pcVar15;
      uVar12 = local_68;
    }
  }
  param_1 = pcVar15;
  uVar7 = local_70;
  local_68 = uVar12;
  if (cVar16 != '.') goto LAB_00110fc7;
  if (pcVar15[1] != '*') {
    puVar5[6] = (ulong)pcVar15;
    param_1 = pcVar15 + 1;
    if ((byte)(pcVar15[1] - 0x30U) < 10) {
      do {
        pcVar9 = param_1 + 1;
        param_1 = param_1 + 1;
      } while ((byte)(*pcVar9 - 0x30U) < 10);
      uVar7 = (long)param_1 - (long)pcVar15;
    }
    else {
      uVar7 = 1;
    }
    puVar5[7] = (ulong)param_1;
    cVar16 = *param_1;
    if (uVar7 <= local_70) {
      uVar7 = local_70;
    }
    goto LAB_00110fc7;
  }
  param_1 = pcVar15 + 2;
  puVar5[6] = (ulong)pcVar15;
  puVar5[7] = (ulong)param_1;
  cVar16 = pcVar15[2];
  uVar7 = 2;
  if (1 < local_70) {
    uVar7 = local_70;
  }
  pcVar9 = param_1;
  if ((byte)(cVar16 - 0x30U) < 10) {
    do {
      pcVar2 = pcVar9 + 1;
      pcVar9 = pcVar9 + 1;
    } while ((byte)(*pcVar2 - 0x30U) < 10);
    if (*pcVar2 != '$') goto LAB_00111215;
    pcVar15 = pcVar15 + 3;
    uVar13 = 0;
    while( true ) {
      param_1 = pcVar15 + -1;
      uVar12 = (ulong)(char)(cVar16 + -0x30);
      if (uVar13 < 0x199999999999999a) {
        uVar14 = uVar13 * 10;
      }
      else {
        uVar14 = 0xffffffffffffffff;
      }
      while( true ) {
        cVar16 = *pcVar15;
        uVar13 = uVar14 + uVar12;
        bVar1 = cVar16 - 0x30;
        if (!CARRY8(uVar14,uVar12)) break;
        if (9 < bVar1) goto switchD_0011107c_caseD_26;
        uVar12 = (ulong)(char)bVar1;
        uVar14 = 0xffffffffffffffff;
        param_1 = pcVar15;
        pcVar15 = pcVar15 + 1;
      }
      if (9 < bVar1) break;
      pcVar15 = pcVar15 + 1;
    }
    uVar13 = uVar13 - 1;
    if (0xfffffffffffffffd < uVar13) goto switchD_0011107c_caseD_26;
    puVar5[8] = uVar13;
    param_1 = param_1 + 2;
  }
  else {
LAB_00111215:
    uVar13 = puVar5[8];
    if (puVar5[8] == 0xffffffffffffffff) {
      puVar5[8] = uVar12;
      local_68 = uVar12 + 1;
      uVar13 = uVar12;
      if (uVar12 == 0xffffffffffffffff) goto switchD_0011107c_caseD_26;
    }
  }
  puVar8 = (ulong *)param_3[1];
  __dest = puVar8;
  if (uVar17 <= uVar13) {
    uVar17 = uVar17 * 2;
    if (uVar17 <= uVar13) {
      uVar17 = uVar13 + 1;
    }
    if (uVar17 >> 0x3b != 0) goto LAB_001116ab;
    if (__src_00 == puVar8) {
      __dest = malloc(uVar17 << 5);
    }
    else {
      __dest = realloc(puVar8,uVar17 << 5);
    }
    puVar8 = (ulong *)param_3[1];
    if (__dest == (ulong *)0x0) goto LAB_001116ab;
    if (__src_00 == puVar8) {
      __dest = memcpy(__dest,__src_00,*param_3 << 5);
    }
    param_3[1] = (ulong)__dest;
  }
  uVar12 = *param_3;
  puVar8 = __dest + uVar12 * 4;
  if (uVar12 <= uVar13) {
    do {
      puVar4 = puVar8;
      uVar12 = uVar12 + 1;
      *(undefined4 *)puVar4 = 0;
      puVar8 = puVar4 + 4;
    } while (uVar12 <= uVar13);
    *param_3 = uVar12;
    *(undefined4 *)puVar4 = 0;
  }
  iVar10 = (int)__dest[uVar13 * 4];
  if (iVar10 == 0) {
    *(int *)(__dest + uVar13 * 4) = 5;
    cVar16 = *param_1;
LAB_00110fc7:
    local_70 = uVar7;
    uVar11 = 0;
    param_1 = param_1 + 1;
    if (cVar16 == 'h') goto LAB_00111094;
    do {
      switch(cVar16) {
      case 'L':
        uVar11 = uVar11 | 4;
        break;
      default:
        goto code_r0x0011106b;
      case 'Z':
      case 'j':
      case 'l':
      case 't':
      case 'z':
        uVar11 = uVar11 + 8;
      }
      while( true ) {
        cVar16 = *param_1;
        param_1 = param_1 + 1;
        if (cVar16 != 'h') break;
LAB_00111094:
        uVar11 = uVar11 | 1 << ((byte)uVar11 & 1);
      }
    } while( true );
  }
  if (iVar10 == 5) {
    cVar16 = *param_1;
    goto LAB_00110fc7;
  }
  goto LAB_00111464;
code_r0x0011106b:
  switch(cVar16) {
  case '%':
    goto switchD_0011107c_caseD_25;
  default:
    goto switchD_0011107c_caseD_26;
  case 'A':
  case 'E':
  case 'F':
  case 'G':
  case 'a':
  case 'e':
  case 'f':
  case 'g':
    iVar10 = 0xc;
    if ((int)uVar11 < 0x10) {
      iVar10 = ((uVar11 & 4) != 0) + 0xb;
    }
    break;
  case 'C':
    iVar10 = 0xe;
    cVar16 = 'c';
    break;
  case 'S':
    iVar10 = 0x10;
    cVar16 = 's';
    break;
  case 'X':
  case 'o':
  case 'u':
  case 'x':
    iVar10 = 10;
    if ((((int)uVar11 < 0x10) && ((uVar11 & 4) == 0)) &&
       ((iVar10 = 8, (int)uVar11 < 8 && (iVar10 = 2, (uVar11 & 2) == 0)))) {
      iVar10 = (-(uint)((uVar11 & 1) == 0) & 2) + 4;
    }
    break;
  case 'c':
    iVar10 = (7 < (int)uVar11) + 0xd;
    break;
  case 'd':
  case 'i':
    iVar10 = 9;
    if ((((int)uVar11 < 0x10) && ((uVar11 & 4) == 0)) &&
       ((iVar10 = 7, (int)uVar11 < 8 && (iVar10 = 1, (uVar11 & 2) == 0)))) {
      iVar10 = (-(uint)((uVar11 & 1) == 0) & 2) + 3;
    }
    break;
  case 'n':
    iVar10 = 0x16;
    if (((((int)uVar11 < 0x10) && ((uVar11 & 4) == 0)) && (iVar10 = 0x15, (int)uVar11 < 8)) &&
       (iVar10 = 0x12, (uVar11 & 2) == 0)) {
      iVar10 = 0x14 - (uVar11 & 1);
    }
    break;
  case 'p':
    iVar10 = 0x11;
    break;
  case 's':
    iVar10 = (7 < (int)uVar11) + 0xf;
  }
  if (uVar19 == 0xffffffffffffffff) {
    puVar5[10] = local_68;
    bVar20 = local_68 == 0xffffffffffffffff;
    uVar19 = local_68;
    local_68 = local_68 + 1;
    if (bVar20) {
switchD_0011107c_caseD_26:
      __dest = (ulong *)param_3[1];
      goto LAB_00111464;
    }
  }
  else {
    puVar5[10] = uVar19;
  }
  puVar8 = (ulong *)param_3[1];
  __dest = puVar8;
  if (uVar17 <= uVar19) {
    uVar17 = uVar17 * 2;
    if (uVar17 <= uVar19) {
      uVar17 = uVar19 + 1;
    }
    if (uVar17 >> 0x3b != 0) goto LAB_001116ab;
    if (__src_00 == puVar8) {
      __dest = malloc(uVar17 << 5);
    }
    else {
      __dest = realloc(puVar8,uVar17 << 5);
    }
    puVar8 = (ulong *)param_3[1];
    if (__dest == (ulong *)0x0) goto LAB_001116ab;
    if (__src_00 == puVar8) {
      __dest = memcpy(__dest,__src_00,*param_3 << 5);
    }
    param_3[1] = (ulong)__dest;
  }
  uVar12 = *param_3;
  puVar8 = __dest + uVar12 * 4;
  if (uVar12 <= uVar19) {
    do {
      puVar4 = puVar8;
      uVar12 = uVar12 + 1;
      *(undefined4 *)puVar4 = 0;
      puVar8 = puVar4 + 4;
    } while (uVar12 <= uVar19);
    *param_3 = uVar12;
    *(undefined4 *)puVar4 = 0;
  }
  iVar3 = (int)__dest[uVar19 * 4];
  if (iVar3 == 0) {
    *(int *)(__dest + uVar19 * 4) = iVar10;
  }
  else if (iVar3 != iVar10) {
LAB_00111464:
    if (__src_00 != __dest) {
      free(__dest);
    }
    if (__src != (ulong *)param_2[1]) {
      free((ulong *)param_2[1]);
    }
    piVar6 = __errno_location();
    *piVar6 = 0x16;
    return 0xffffffff;
  }
switchD_0011107c_caseD_25:
  *(char *)(puVar5 + 9) = cVar16;
  uVar12 = *param_2;
  puVar5[1] = (ulong)param_1;
  uVar12 = uVar12 + 1;
  *param_2 = uVar12;
  if (uVar12 < uVar18) {
    puVar5 = (ulong *)param_2[1];
    goto LAB_00110ecc;
  }
  if ((-1 < (long)uVar18) && (uVar18 * 2 < 0x2e8ba2e8ba2e8bb)) {
    if (__src == (ulong *)param_2[1]) {
      puVar5 = malloc(uVar18 * 0xb0);
    }
    else {
      puVar5 = realloc((ulong *)param_2[1],uVar18 * 0xb0);
    }
    if (puVar5 != (ulong *)0x0) {
      if (__src == (ulong *)param_2[1]) {
        puVar5 = memcpy(puVar5,__src,*param_2 * 0x58);
      }
      param_2[1] = (ulong)puVar5;
      uVar12 = *param_2;
      uVar18 = uVar18 * 2;
      goto LAB_00110ecc;
    }
  }
  puVar8 = (ulong *)param_3[1];
LAB_001116ab:
  if (__src_00 != puVar8) {
    free(puVar8);
  }
  if (__src != (ulong *)param_2[1]) {
    free((ulong *)param_2[1]);
  }
  piVar6 = __errno_location();
  *piVar6 = 0xc;
  return 0xffffffff;
}




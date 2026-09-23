// Function: bi_init @ 0x3a29

void bi_init(int param_1)

{
  DAT_0011a058 = param_1;
  DAT_0011a05c = 0;
  DAT_0011a060 = 0;
  if (param_1 != -1) {
    DAT_0011a050 = FUN_001101d7;
  }
  return;
}




// Function: send_bits @ 0x3a6c

void send_bits(uint param_1,int param_2)

{
  ulong uVar1;
  
  if (0x10 - param_2 < DAT_0011a060) {
    DAT_0011a05c = DAT_0011a05c | (ushort)(param_1 << ((byte)DAT_0011a060 & 0x1f));
    if (DAT_001dd870 < 0x3fffe) {
      uVar1 = (ulong)DAT_001dd870;
      DAT_001dd870 = DAT_001dd870 + 1;
      (&DAT_0015c000)[uVar1] = (char)DAT_0011a05c;
      uVar1 = (ulong)DAT_001dd870;
      DAT_001dd870 = DAT_001dd870 + 1;
      (&DAT_0015c000)[uVar1] = (char)(DAT_0011a05c >> 8);
    }
    else {
      uVar1 = (ulong)DAT_001dd870;
      DAT_001dd870 = DAT_001dd870 + 1;
      (&DAT_0015c000)[uVar1] = (char)DAT_0011a05c;
      if (DAT_001dd870 == 0x40000) {
        FUN_0010f28e();
      }
      uVar1 = (ulong)DAT_001dd870;
      DAT_001dd870 = DAT_001dd870 + 1;
      (&DAT_0015c000)[uVar1] = (char)(DAT_0011a05c >> 8);
      if (DAT_001dd870 == 0x40000) {
        FUN_0010f28e();
      }
    }
    DAT_0011a05c = (ushort)((int)(param_1 & 0xffff) >> (0x10U - (char)DAT_0011a060 & 0x1f));
    DAT_0011a060 = param_2 + DAT_0011a060 + -0x10;
  }
  else {
    DAT_0011a05c = DAT_0011a05c | (ushort)(param_1 << ((byte)DAT_0011a060 & 0x1f));
    DAT_0011a060 = param_2 + DAT_0011a060;
  }
  return;
}




// Function: bi_reverse @ 0x3bed

uint bi_reverse(uint param_1,int param_2)

{
  uint uVar1;
  uint uVar2;
  uint uVar3;
  undefined4 local_18;
  undefined4 local_14;
  
  uVar1 = 0;
  local_18 = param_2;
  local_14 = param_1;
  do {
    uVar3 = uVar1;
    uVar2 = local_14 & 1;
    local_14 = local_14 >> 1;
    local_18 = local_18 + -1;
    uVar1 = (uVar3 | uVar2) * 2;
  } while (0 < local_18);
  return uVar3 & 0x7fffffff | uVar2;
}




// Function: bi_windup @ 0x3c22

void bi_windup(void)

{
  undefined1 uVar1;
  ulong uVar2;
  
  uVar1 = (undefined1)DAT_0011a05c;
  if (DAT_0011a060 < 9) {
    if (0 < DAT_0011a060) {
      uVar2 = (ulong)DAT_001dd870;
      DAT_001dd870 = DAT_001dd870 + 1;
      (&DAT_0015c000)[uVar2] = uVar1;
      if (DAT_001dd870 == 0x40000) {
        FUN_0010f28e();
      }
    }
  }
  else if (DAT_001dd870 < 0x3fffe) {
    uVar2 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar2] = uVar1;
    uVar2 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar2] = (char)((ushort)DAT_0011a05c >> 8);
  }
  else {
    uVar2 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar2] = uVar1;
    if (DAT_001dd870 == 0x40000) {
      FUN_0010f28e();
    }
    uVar2 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar2] = (char)((ushort)DAT_0011a05c >> 8);
    if (DAT_001dd870 == 0x40000) {
      FUN_0010f28e();
    }
  }
  DAT_0011a05c = 0;
  DAT_0011a060 = 0;
  return;
}




// Function: copy_block @ 0x3d59

void copy_block(undefined1 *param_1,int param_2,int param_3)

{
  int iVar1;
  undefined1 *puVar2;
  byte bVar3;
  ulong uVar5;
  int local_14;
  undefined1 *local_10;
  byte bVar4;
  
  FUN_00103c22();
  local_14 = param_2;
  local_10 = param_1;
  if (param_3 != 0) {
    bVar3 = (byte)param_2;
    bVar4 = (byte)((uint)param_2 >> 8);
    if (DAT_001dd870 < 0x3fffe) {
      uVar5 = (ulong)DAT_001dd870;
      DAT_001dd870 = DAT_001dd870 + 1;
      (&DAT_0015c000)[uVar5] = bVar3;
      uVar5 = (ulong)DAT_001dd870;
      DAT_001dd870 = DAT_001dd870 + 1;
      (&DAT_0015c000)[uVar5] = bVar4;
    }
    else {
      uVar5 = (ulong)DAT_001dd870;
      DAT_001dd870 = DAT_001dd870 + 1;
      (&DAT_0015c000)[uVar5] = bVar3;
      if (DAT_001dd870 == 0x40000) {
        FUN_0010f28e();
      }
      uVar5 = (ulong)DAT_001dd870;
      DAT_001dd870 = DAT_001dd870 + 1;
      (&DAT_0015c000)[uVar5] = bVar4;
      if (DAT_001dd870 == 0x40000) {
        FUN_0010f28e();
      }
    }
    if (DAT_001dd870 < 0x3fffe) {
      uVar5 = (ulong)DAT_001dd870;
      DAT_001dd870 = DAT_001dd870 + 1;
      (&DAT_0015c000)[uVar5] = ~bVar3;
      uVar5 = (ulong)DAT_001dd870;
      DAT_001dd870 = DAT_001dd870 + 1;
      (&DAT_0015c000)[uVar5] = ~bVar4;
      local_14 = param_2;
      local_10 = param_1;
    }
    else {
      uVar5 = (ulong)DAT_001dd870;
      DAT_001dd870 = DAT_001dd870 + 1;
      (&DAT_0015c000)[uVar5] = ~bVar3;
      if (DAT_001dd870 == 0x40000) {
        FUN_0010f28e();
      }
      uVar5 = (ulong)DAT_001dd870;
      DAT_001dd870 = DAT_001dd870 + 1;
      (&DAT_0015c000)[uVar5] = ~bVar4;
      local_14 = param_2;
      local_10 = param_1;
      if (DAT_001dd870 == 0x40000) {
        FUN_0010f28e();
      }
    }
  }
  while (iVar1 = local_14 + -1, local_14 != 0) {
    puVar2 = local_10 + 1;
    uVar5 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar5] = *local_10;
    local_14 = iVar1;
    local_10 = puVar2;
    if (DAT_001dd870 == 0x40000) {
      FUN_0010f28e();
    }
  }
  return;
}




// Function: lm_init @ 0x3f52

void lm_init(int param_1)

{
  ulong uVar1;
  
  if ((param_1 < 1) || (9 < param_1)) {
    FUN_0010f5ef("bad pack level");
  }
  memset(&DAT_001cd000,0,0x10000);
  DAT_0011a0a0 = 0xffffffff;
  DAT_0011a098 = 0;
  DAT_0011a090 = (uint)*(ushort *)(&DAT_00119042 + (long)param_1 * 8);
  DAT_0011a080 = (uint)*(ushort *)(&DAT_00119040 + (long)param_1 * 8);
  DAT_0011a0a8 = (uint)*(ushort *)(&DAT_00119044 + (long)param_1 * 8);
  DAT_0011a07c = (uint)*(ushort *)(&DAT_00119046 + (long)param_1 * 8);
  DAT_0011a074 = 0;
  DAT_0011a068 = 0;
  DAT_0011a08c = (*DAT_0011a050)(&DAT_001ad000,0x10000);
  if ((DAT_0011a08c == 0) || (DAT_0011a08c == 0xffffffff)) {
    DAT_0011a088 = 1;
    DAT_0011a08c = 0;
  }
  else {
    DAT_0011a088 = 0;
    while ((DAT_0011a08c < 0x106 && (DAT_0011a088 == 0))) {
      FUN_00104350();
    }
    DAT_0011a084 = 0;
    for (uVar1 = 0; (uint)uVar1 < 2; uVar1 = (ulong)((uint)uVar1 + 1)) {
      DAT_0011a084 = ((uint)(byte)(&DAT_001ad000)[uVar1] ^ DAT_0011a084 << 5) & 0x7fff;
    }
  }
  return;
}




// Function: longest_match @ 0x40fe

uint longest_match(uint param_1)

{
  char *pcVar1;
  uint uVar2;
  uint uVar3;
  ulong uVar4;
  char *pcVar5;
  char *pcVar7;
  char cVar8;
  char cVar9;
  uint local_44;
  uint local_3c;
  uint local_38;
  char *pcVar6;
  
  local_3c = DAT_0011a07c;
  pcVar6 = &DAT_001ad000 + DAT_0011a074;
  local_38 = DAT_0011a070;
  uVar2 = DAT_0011a074;
  if (DAT_0011a074 < 0x7efa) {
    uVar2 = 0x7efa;
  }
  pcVar1 = (char *)((ulong)DAT_0011a074 + 0x1ad102);
  cVar8 = pcVar6[(long)(int)DAT_0011a070 + -1];
  cVar9 = pcVar6[(int)DAT_0011a070];
  local_44 = param_1;
  if (DAT_0011a080 <= DAT_0011a070) {
    local_3c = DAT_0011a07c >> 2;
  }
  do {
    uVar4 = (ulong)local_44;
    pcVar7 = &DAT_001ad000 + uVar4;
    if ((((cVar9 == pcVar7[(int)local_38]) && (cVar8 == pcVar7[(long)(int)local_38 + -1])) &&
        (*pcVar7 == *pcVar6)) && ((&DAT_001ad001)[uVar4] == pcVar6[1])) {
      pcVar7 = (char *)(uVar4 + 0x1ad002);
      pcVar6 = pcVar6 + 2;
      while (((((pcVar5 = pcVar6 + 1, *pcVar5 == pcVar7[1] &&
                (pcVar5 = pcVar6 + 2, *pcVar5 == pcVar7[2])) &&
               ((pcVar5 = pcVar6 + 3, *pcVar5 == pcVar7[3] &&
                ((pcVar5 = pcVar6 + 4, *pcVar5 == pcVar7[4] &&
                 (pcVar5 = pcVar6 + 5, *pcVar5 == pcVar7[5])))))) &&
              (pcVar5 = pcVar6 + 6, *pcVar5 == pcVar7[6])) &&
             (pcVar5 = pcVar6 + 7, *pcVar5 == pcVar7[7]))) {
        pcVar5 = pcVar6 + 8;
        pcVar7 = pcVar7 + 8;
        if ((*pcVar5 != *pcVar7) || (pcVar6 = pcVar5, pcVar1 <= pcVar5)) break;
      }
      uVar3 = 0x102 - ((int)pcVar1 - (int)pcVar5);
      pcVar6 = &DAT_001ad000 + DAT_0011a074;
      if ((int)local_38 < (int)uVar3) {
        DAT_0011a078 = local_44;
        if (DAT_0011a0a8 <= (int)uVar3) {
          return uVar3;
        }
        cVar8 = pcVar6[(long)(int)uVar3 + -1];
        cVar9 = pcVar6[(int)uVar3];
        local_38 = uVar3;
      }
    }
    local_44 = (uint)(ushort)(&DAT_001bd000)[local_44 & 0x7fff];
    if (local_44 <= uVar2 - 0x7efa) {
      return local_38;
    }
    local_3c = local_3c - 1;
    if (local_3c == 0) {
      return local_38;
    }
  } while( true );
}




// Function: fill_window @ 0x4350

void fill_window(void)

{
  short sVar1;
  int iVar2;
  ulong uVar3;
  int local_1c;
  
  local_1c = ((int)DAT_00119020 - DAT_0011a08c) - DAT_0011a074;
  if (local_1c == -1) {
    local_1c = -2;
  }
  else if (0xfef9 < DAT_0011a074) {
    memcpy(&DAT_001ad000,&DAT_001b5000,0x8000);
    DAT_0011a078 = DAT_0011a078 + -0x8000;
    DAT_0011a074 = DAT_0011a074 - 0x8000;
    if (DAT_0011a0a0 != 0xffffffff) {
      DAT_0011a0a0 = DAT_0011a0a0 + -0x8000;
    }
    DAT_0011a068 = DAT_0011a068 + -0x8000;
    for (uVar3 = 0; (uint)uVar3 < 0x8000; uVar3 = (ulong)((uint)uVar3 + 1)) {
      if ((ushort)(&DAT_001bd000)[uVar3 + 0x8000] < 0x8000) {
        sVar1 = 0;
      }
      else {
        sVar1 = (&DAT_001bd000)[uVar3 + 0x8000] + 0x8000;
      }
      (&DAT_001bd000)[uVar3 + 0x8000] = sVar1;
    }
    for (uVar3 = 0; (uint)uVar3 < 0x8000; uVar3 = (ulong)((uint)uVar3 + 1)) {
      if ((ushort)(&DAT_001bd000)[uVar3] < 0x8000) {
        sVar1 = 0;
      }
      else {
        sVar1 = (&DAT_001bd000)[uVar3] + 0x8000;
      }
      (&DAT_001bd000)[uVar3] = sVar1;
    }
    local_1c = local_1c + 0x8000;
  }
  if (DAT_0011a088 == 0) {
    iVar2 = (*DAT_0011a050)(&DAT_001ad000 + (ulong)DAT_0011a074 + (ulong)DAT_0011a08c,local_1c);
    if ((iVar2 == 0) || (iVar2 == -1)) {
      DAT_0011a088 = 1;
      memset(&DAT_001ad000 + (ulong)DAT_0011a074 + (ulong)DAT_0011a08c,0,2);
    }
    else {
      DAT_0011a08c = DAT_0011a08c + iVar2;
    }
  }
  return;
}




// Function: rsync_roll @ 0x4567

void rsync_roll(uint param_1,int param_2)

{
  int local_20;
  uint local_1c;
  uint local_c;
  
  local_20 = param_2;
  local_1c = param_1;
  local_c = param_1;
  if (param_1 < 0x1000) {
    for (; local_c < 0x1000; local_c = local_c + 1) {
      if (local_c == param_2 + param_1) {
        return;
      }
      DAT_0011a098 = DAT_0011a098 + (byte)(&DAT_001ad000)[local_c];
    }
    local_20 = param_2 + param_1 + -0x1000;
    local_1c = 0x1000;
  }
  for (local_c = local_1c; local_c < local_20 + local_1c; local_c = local_c + 1) {
    DAT_0011a098 = (DAT_0011a098 + (byte)(&DAT_001ad000)[local_c]) -
                   (ulong)(byte)(&DAT_001ad000)[local_c - 0x1000];
    if ((DAT_0011a0a0 == 0xffffffff) && ((DAT_0011a098 & 0xfff) == 0)) {
      DAT_0011a0a0 = (ulong)local_c;
    }
  }
  return;
}




// Function: deflate_fast @ 0x4677

void deflate_fast(void)

{
  uint uVar1;
  undefined1 *puVar2;
  int local_14;
  uint local_10;
  
  local_14 = 0;
  local_10 = 0;
  DAT_0011a070 = 2;
  while (DAT_0011a08c != 0) {
    DAT_0011a084 = ((uint)(byte)(&DAT_001ad000)[DAT_0011a074 + 2] ^ DAT_0011a084 << 5) & 0x7fff;
    uVar1 = (uint)(ushort)(&DAT_001bd000)[(ulong)DAT_0011a084 + 0x8000];
    (&DAT_001bd000)[DAT_0011a074 & 0x7fff] = (&DAT_001bd000)[(ulong)DAT_0011a084 + 0x8000];
    (&DAT_001bd000)[(ulong)DAT_0011a084 + 0x8000] = (short)DAT_0011a074;
    if (((uVar1 != 0) && (DAT_0011a074 - uVar1 < 0x7efb)) &&
       ((ulong)DAT_0011a074 <= DAT_00119020 - 0x106U)) {
      local_10 = FUN_001040fe(uVar1);
      if (DAT_0011a08c < local_10) {
        local_10 = DAT_0011a08c;
      }
    }
    if (local_10 < 3) {
      local_14 = FUN_0010c45d(0,(&DAT_001ad000)[DAT_0011a074]);
      if (DAT_001dd874 != 0) {
        FUN_00104567(DAT_0011a074,1);
      }
      DAT_0011a08c = DAT_0011a08c - 1;
      DAT_0011a074 = DAT_0011a074 + 1;
    }
    else {
      local_14 = FUN_0010c45d(DAT_0011a074 - DAT_0011a078,local_10 - 3);
      DAT_0011a08c = DAT_0011a08c - local_10;
      if (DAT_001dd874 != 0) {
        FUN_00104567(DAT_0011a074,local_10);
      }
      if (DAT_0011a090 < local_10) {
        DAT_0011a074 = local_10 + DAT_0011a074;
        local_10 = 0;
        DAT_0011a084 = (uint)(byte)(&DAT_001ad000)[DAT_0011a074 + 1] ^
                       (uint)(byte)(&DAT_001ad000)[DAT_0011a074] << 5;
      }
      else {
        local_10 = local_10 - 1;
        do {
          uVar1 = DAT_0011a074 + 1;
          DAT_0011a084 = ((uint)(byte)(&DAT_001ad000)[DAT_0011a074 + 3] ^ DAT_0011a084 << 5) &
                         0x7fff;
          DAT_0011a074 = uVar1;
          (&DAT_001bd000)[uVar1 & 0x7fff] = (&DAT_001bd000)[(ulong)DAT_0011a084 + 0x8000];
          (&DAT_001bd000)[(ulong)DAT_0011a084 + 0x8000] = (short)DAT_0011a074;
          local_10 = local_10 + -1;
        } while (local_10 != 0);
        DAT_0011a074 = DAT_0011a074 + 1;
        local_10 = 0;
      }
    }
    if ((DAT_001dd874 != 0) && (DAT_0011a0a0 < DAT_0011a074)) {
      DAT_0011a0a0 = 0xffffffff;
      local_14 = 2;
    }
    if (local_14 != 0) {
      if ((long)DAT_0011a068 < 0) {
        puVar2 = (undefined1 *)0x0;
      }
      else {
        puVar2 = &DAT_001ad000 + (DAT_0011a068 & 0xffffffff);
      }
      FUN_0010c1ea(puVar2,DAT_0011a074 - DAT_0011a068,local_14 + -1,0);
      DAT_0011a068 = (ulong)DAT_0011a074;
    }
    while ((DAT_0011a08c < 0x106 && (DAT_0011a088 == 0))) {
      FUN_00104350();
    }
  }
  if ((long)DAT_0011a068 < 0) {
    puVar2 = (undefined1 *)0x0;
  }
  else {
    puVar2 = &DAT_001ad000 + (DAT_0011a068 & 0xffffffff);
  }
  FUN_0010c1ea(puVar2,DAT_0011a074 - DAT_0011a068,local_14 + -1,1);
  return;
}




// Function: deflate @ 0x4a9e

void deflate(int param_1)

{
  int iVar1;
  bool bVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  undefined1 *puVar6;
  int local_28;
  
  local_28 = 0;
  bVar2 = false;
  uVar5 = 2;
  FUN_00103f52(param_1);
  if (param_1 < 4) {
    FUN_00104677();
  }
  else {
    while (DAT_0011a08c != 0) {
      DAT_0011a084 = ((uint)(byte)(&DAT_001ad000)[DAT_0011a074 + 2] ^ DAT_0011a084 << 5) & 0x7fff;
      uVar3 = (uint)(ushort)(&DAT_001bd000)[(ulong)DAT_0011a084 + 0x8000];
      (&DAT_001bd000)[DAT_0011a074 & 0x7fff] = (&DAT_001bd000)[(ulong)DAT_0011a084 + 0x8000];
      (&DAT_001bd000)[(ulong)DAT_0011a084 + 0x8000] = (short)DAT_0011a074;
      iVar1 = DAT_0011a078;
      uVar4 = 2;
      DAT_0011a070 = uVar5;
      if ((((uVar3 != 0) && (uVar5 < DAT_0011a090)) && (DAT_0011a074 - uVar3 < 0x7efb)) &&
         ((ulong)DAT_0011a074 <= DAT_00119020 - 0x106U)) {
        uVar4 = FUN_001040fe(uVar3);
        if (DAT_0011a08c < uVar4) {
          uVar4 = DAT_0011a08c;
        }
        if ((uVar4 == 3) && (0x1000 < DAT_0011a074 - DAT_0011a078)) {
          uVar4 = 2;
        }
      }
      uVar5 = uVar4;
      if ((DAT_0011a070 < 3) || (DAT_0011a070 < uVar5)) {
        if (bVar2) {
          local_28 = FUN_0010c45d(0,(&DAT_001ad000)[DAT_0011a074 - 1]);
          if ((DAT_001dd874 != 0) && (DAT_0011a0a0 < DAT_0011a074)) {
            DAT_0011a0a0 = 0xffffffff;
            local_28 = 2;
          }
          if (local_28 != 0) {
            if ((long)DAT_0011a068 < 0) {
              puVar6 = (undefined1 *)0x0;
            }
            else {
              puVar6 = &DAT_001ad000 + (DAT_0011a068 & 0xffffffff);
            }
            FUN_0010c1ea(puVar6,DAT_0011a074 - DAT_0011a068,local_28 + -1,0);
            DAT_0011a068 = (ulong)DAT_0011a074;
          }
          if (DAT_001dd874 != 0) {
            FUN_00104567(DAT_0011a074,1);
          }
          DAT_0011a074 = DAT_0011a074 + 1;
          DAT_0011a08c = DAT_0011a08c - 1;
        }
        else {
          if ((DAT_001dd874 != 0) && (DAT_0011a0a0 < DAT_0011a074)) {
            DAT_0011a0a0 = 0xffffffff;
            local_28 = 2;
            if ((long)DAT_0011a068 < 0) {
              puVar6 = (undefined1 *)0x0;
            }
            else {
              puVar6 = &DAT_001ad000 + (DAT_0011a068 & 0xffffffff);
            }
            FUN_0010c1ea(puVar6,DAT_0011a074 - DAT_0011a068,1,0);
            DAT_0011a068 = (ulong)DAT_0011a074;
          }
          bVar2 = true;
          if (DAT_001dd874 != 0) {
            FUN_00104567(DAT_0011a074,1);
          }
          DAT_0011a074 = DAT_0011a074 + 1;
          DAT_0011a08c = DAT_0011a08c - 1;
        }
      }
      else {
        local_28 = FUN_0010c45d((DAT_0011a074 - iVar1) + -1,DAT_0011a070 - 3);
        DAT_0011a08c = (DAT_0011a08c - DAT_0011a070) + 1;
        uVar5 = DAT_0011a070 - 2;
        if (DAT_001dd874 != 0) {
          iVar1 = DAT_0011a070 - 1;
          DAT_0011a070 = DAT_0011a070 - 2;
          FUN_00104567(DAT_0011a074,iVar1);
          uVar5 = DAT_0011a070;
        }
        do {
          DAT_0011a070 = uVar5;
          uVar5 = DAT_0011a074 + 1;
          DAT_0011a084 = ((uint)(byte)(&DAT_001ad000)[DAT_0011a074 + 3] ^ DAT_0011a084 << 5) &
                         0x7fff;
          DAT_0011a074 = uVar5;
          (&DAT_001bd000)[uVar5 & 0x7fff] = (&DAT_001bd000)[(ulong)DAT_0011a084 + 0x8000];
          (&DAT_001bd000)[(ulong)DAT_0011a084 + 0x8000] = (short)DAT_0011a074;
          DAT_0011a070 = DAT_0011a070 - 1;
          uVar5 = DAT_0011a070;
        } while (DAT_0011a070 != 0);
        bVar2 = false;
        uVar5 = 2;
        DAT_0011a074 = DAT_0011a074 + 1;
        if ((DAT_001dd874 != 0) && (DAT_0011a0a0 < DAT_0011a074)) {
          DAT_0011a0a0 = 0xffffffff;
          local_28 = 2;
        }
        if (local_28 != 0) {
          if ((long)DAT_0011a068 < 0) {
            puVar6 = (undefined1 *)0x0;
          }
          else {
            puVar6 = &DAT_001ad000 + (DAT_0011a068 & 0xffffffff);
          }
          FUN_0010c1ea(puVar6,DAT_0011a074 - DAT_0011a068,local_28 + -1,0);
          DAT_0011a068 = (ulong)DAT_0011a074;
        }
      }
      while ((DAT_0011a08c < 0x106 && (DAT_0011a088 == 0))) {
        FUN_00104350();
      }
    }
    if (bVar2) {
      FUN_0010c45d(0,(&DAT_001ad000)[DAT_0011a074 - 1]);
    }
    if ((long)DAT_0011a068 < 0) {
      puVar6 = (undefined1 *)0x0;
    }
    else {
      puVar6 = &DAT_001ad000 + (DAT_0011a068 & 0xffffffff);
    }
    FUN_0010c1ea(puVar6,DAT_0011a074 - DAT_0011a068,local_28 + -1,1);
  }
  return;
}




// Function: try_help @ 0x5097

void try_help(void)

{
  fprintf(stderr,"Try `%s --help\' for more information.\n",DAT_001dd010);
                    /* WARNING: Subroutine does not return */
  FUN_00108e40(1);
}




// Function: help @ 0x50ce

void help(void)

{
  undefined **local_10;
  
  local_10 = &PTR_s_Compress_or_uncompress_FILEs__by_00118a40;
  printf("Usage: %s [OPTION]... [FILE]...\n",DAT_001dd010);
  while (*local_10 != (undefined *)0x0) {
    printf("%s\n",*local_10);
    local_10 = local_10 + 1;
  }
  return;
}




// Function: license @ 0x513b

void license(void)

{
  undefined **local_10;
  
  local_10 = &PTR_s_Copyright__C__2018_Free_Software_001186a0;
  printf("%s %s\n",DAT_001dd010,PTR_DAT_00119558);
  while (*local_10 != (undefined *)0x0) {
    printf("%s\n",*local_10);
    local_10 = local_10 + 1;
  }
  return;
}




// Function: version @ 0x51af

void version(void)

{
  FUN_0010513b();
  printf("\n");
  printf("Written by Jean-loup Gailly.\n");
  return;
}




// Function: progerror @ 0x51e7

void progerror(char *param_1)

{
  int iVar1;
  int *piVar2;
  
  piVar2 = __errno_location();
  iVar1 = *piVar2;
  fprintf(stderr,"%s: ",DAT_001dd010);
  piVar2 = __errno_location();
  *piVar2 = iVar1;
  perror(param_1);
  DAT_001dd018 = 1;
  return;
}




// Function: main @ 0x5249

void main(int param_1,undefined8 *param_2)

{
  int iVar1;
  long lVar2;
  int *piVar3;
  long in_FS_OFFSET;
  int local_38;
  int local_34;
  int local_30;
  int local_2c;
  undefined8 *local_28;
  undefined8 *local_20;
  size_t local_18;
  undefined8 local_10;
  
  local_10 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  DAT_001dd010 = (char *)FUN_0010f40a(*param_2);
  local_18 = strlen(DAT_001dd010);
  if ((4 < local_18) && (iVar1 = strcmp(DAT_001dd010 + (local_18 - 4),".exe"), iVar1 == 0)) {
    DAT_001dd010[local_18 - 4] = '\0';
  }
  local_28 = param_2;
  DAT_001dd8a8 = FUN_0010f454(&local_38,&local_28,&DAT_001132c6);
  local_20 = local_28;
  if (DAT_001dd8a8 == 0) {
    local_20 = (undefined8 *)0x0;
  }
  DAT_001dd8b0 = ".gz";
  DAT_001dd8b8 = strlen(".gz");
  while( true ) {
    local_34 = -1;
    if (local_20 != (undefined8 *)0x0) {
      if ((local_20[optind] == 0) || (iVar1 = strcmp((char *)local_20[optind],"--"), iVar1 != 0)) {
        local_30 = getopt_long(local_38,local_20,"ab:cdfhH?klLmMnNqrS:tvVZ123456789",
                               &PTR_s_ascii_001186e0,&local_34);
        if (local_30 < 0) {
          if (optind != local_38) {
            fprintf(stderr,"%s: %s: non-option in GZIP environment variable\n",DAT_001dd010,
                    local_20[optind]);
            FUN_00105097();
          }
          if ((local_38 != 1) && (DAT_001dd004 == 0)) {
            fprintf(stderr,
                    "%s: warning: GZIP environment variable is deprecated; use an alias or script\n"
                    ,DAT_001dd010);
          }
          free(local_20);
          local_20 = (undefined8 *)0x0;
          optind = 1;
          local_34 = -1;
        }
        else {
          local_30 = local_30 + 0x83;
        }
      }
      else {
        local_30 = 0xb0;
      }
    }
    if (local_20 == (undefined8 *)0x0) {
      local_30 = getopt_long(param_1,param_2,"ab:cdfhH?klLmMnNqrS:tvVZ123456789",
                             &PTR_s_ascii_001186e0,&local_34);
    }
    if (local_30 < 0) break;
    switch(local_30) {
    default:
      if ((0x82 < local_30) && (local_30 != 0xc2)) {
        fprintf(stderr,"%s: ",DAT_001dd010);
        if (local_34 < 0) {
          fprintf(stderr,"-%c: ",(ulong)(local_30 - 0x83));
        }
        else {
          fprintf(stderr,"--%s: ",(&PTR_s_ascii_001186e0)[(long)local_34 * 4]);
        }
        fprintf(stderr,"option not valid in GZIP environment variable\n");
      }
      FUN_00105097();
      break;
    case 0x48:
    case 0x68:
      FUN_001050ce();
      FUN_00108e91();
      break;
    case 0x4c:
      FUN_0010513b();
      FUN_00108e91();
      break;
    case 0x4d:
      DAT_00119094 = 0;
      break;
    case 0x4e:
    case 0xd1:
      DAT_00119094 = 0;
      DAT_00119090 = 0;
      break;
    case 0x53:
      DAT_001dd8b8 = strlen(optarg);
      DAT_001dd8b0 = optarg;
      break;
    case 0x56:
      FUN_001051af();
      FUN_00108e91();
      break;
    case 0x5a:
      fprintf(stderr,"%s: -Z not supported in this version\n",DAT_001dd010);
      FUN_00105097();
    case 0xb4:
    case 0xb5:
    case 0xb6:
    case 0xb7:
    case 0xb8:
    case 0xb9:
    case 0xba:
    case 0xbb:
    case 0xbc:
      local_30 = local_30 + -0x83;
    case 0x31:
    case 0x32:
    case 0x33:
    case 0x34:
    case 0x35:
    case 0x36:
    case 0x37:
    case 0x38:
    case 0x39:
      DAT_001190a0 = local_30 + -0x30;
      break;
    case 0x61:
      DAT_001dd87c = 1;
      break;
    case 0x62:
      DAT_00119098 = atoi(optarg);
      for (; *optarg != '\0'; optarg = optarg + 1) {
        if ((*optarg < '0') || ('9' < *optarg)) {
          fprintf(stderr,"%s: -b operand is not an integer\n",DAT_001dd010);
          FUN_00105097();
        }
      }
      break;
    case 99:
      DAT_001dd000 = 1;
      break;
    case 100:
      DAT_001dd880 = 1;
      break;
    case 0x66:
      DAT_001dd884 = DAT_001dd884 + 1;
      break;
    case 0x6b:
      DAT_001dd888 = 1;
      break;
    case 0x6c:
      DAT_001dd000 = 1;
      DAT_001dd008 = 1;
      DAT_001dd880 = 1;
      DAT_001dd890 = 1;
      break;
    case 0x6d:
      DAT_00119094 = 1;
      break;
    case 0x6e:
    case 0xf1:
      DAT_00119094 = 1;
      DAT_00119090 = 1;
      break;
    case 0x71:
    case 0xf4:
      DAT_001dd004 = 1;
      DAT_001dd894 = 0;
      break;
    case 0x72:
      DAT_001dd88c = 1;
      break;
    case 0x74:
      DAT_001dd000 = 1;
      DAT_001dd880 = 1;
      DAT_001dd008 = 1;
      break;
    case 0x76:
    case 0xf9:
      DAT_001dd894 = DAT_001dd894 + 1;
      DAT_001dd004 = 0;
      break;
    case 0x80:
      DAT_001dd878 = 1;
      break;
    case 0x81:
    case 0x104:
      DAT_001dd874 = 1;
      break;
    case 0x82:
      DAT_001dd879 = '\x01';
    }
  }
  if (DAT_00119094 < 0) {
    DAT_00119094 = DAT_001dd880;
  }
  if (DAT_00119090 < 0) {
    DAT_00119090 = DAT_001dd880;
  }
  local_2c = param_1 - optind;
  if ((DAT_001dd87c != 0) && (DAT_001dd004 == 0)) {
    fprintf(stderr,"%s: option --ascii ignored on this system\n",DAT_001dd010);
  }
  if ((DAT_001dd8b8 != 0) && (DAT_001dd8b8 < 0x1f)) {
    if (DAT_001dd004 == 0) {
      DAT_001dd940 = 0;
    }
    else {
      DAT_001dd940 = 0xd;
    }
    FUN_00108c07();
    if (local_2c == 0) {
      FUN_00105b3e();
    }
    else {
      while (optind < param_1) {
        lVar2 = (long)optind;
        optind = optind + 1;
        FUN_00105f11(param_2[lVar2]);
      }
    }
    if ((DAT_001ddd60 != '\0') && (iVar1 = close(0), iVar1 != 0)) {
      strcpy(&DAT_001dd060,"stdin");
      FUN_0010f6c7();
    }
    if (DAT_001dd890 != 0) {
      if ((DAT_001dd004 == 0) && (1 < local_2c)) {
        FUN_00107fdc(0xffffffff);
      }
      iVar1 = FUN_001109d7(stdout);
      if (iVar1 != 0) {
        FUN_0010f74d();
      }
    }
    if (((DAT_001dd000 != 0) &&
        ((((DAT_001dd879 != '\0' && (iVar1 = fdatasync(1), iVar1 != 0)) &&
          (piVar3 = __errno_location(), *piVar3 != 0x16)) || (iVar1 = close(1), iVar1 != 0)))) &&
       (piVar3 = __errno_location(), *piVar3 != 9)) {
      FUN_0010f74d();
    }
                    /* WARNING: Subroutine does not return */
    FUN_00108e40(DAT_001dd018);
  }
  fprintf(stderr,"%s: invalid suffix \'%s\'\n",DAT_001dd010,DAT_001dd8b0);
                    /* WARNING: Subroutine does not return */
  FUN_00108e40(1);
}




// Function: input_eof @ 0x5a65

undefined8 input_eof(void)

{
  int iVar1;
  
  if ((DAT_001dd880 == 0) || (DAT_001dd89c != 0)) {
    return 1;
  }
  if (DAT_001dd86c == DAT_001dd868) {
    if ((DAT_001dd868 != 0x40000) || (iVar1 = FUN_0010f0bc(1), iVar1 == -1)) {
      return 1;
    }
    DAT_001dd86c = 0;
  }
  return 0;
}




// Function: get_input_size_and_time @ 0x5acc

void get_input_size_and_time(void)

{
  undefined1 auVar1 [16];
  
  DAT_001dd020 = 0xffffffffffffffff;
  DAT_001dd038 = 0xffffffffffffffff;
  if (((DAT_001de198 & 0xf000) == 0x8000) &&
     ((DAT_001dd020 = DAT_001de1b0, DAT_00119094 == 0 || (DAT_001dd890 != 0)))) {
    auVar1 = FUN_0011108f(&DAT_001de180);
    DAT_001dd038 = auVar1._8_8_;
    DAT_001dd030 = auVar1._0_8_;
  }
  return;
}




// Function: treat_stdin @ 0x5b3e

void treat_stdin(void)

{
  int iVar1;
  char *pcVar2;
  undefined *puVar3;
  
  if (((DAT_001dd884 == 0) && (DAT_001dd890 == 0)) &&
     ((DAT_001dd878 != '\0' || (iVar1 = isatty((uint)(DAT_001dd880 == 0)), iVar1 != 0)))) {
    if (DAT_001dd004 == 0) {
      if (DAT_001dd880 == 0) {
        puVar3 = &DAT_0011378f;
        pcVar2 = "written to";
      }
      else {
        puVar3 = &DAT_0011378c;
        pcVar2 = "read from";
      }
      fprintf(stderr,
              "%s: compressed data not %s a terminal. Use -f to force %scompression.\nFor help, type: %s -h\n"
              ,DAT_001dd010,pcVar2,puVar3,DAT_001dd010);
    }
                    /* WARNING: Subroutine does not return */
    FUN_00108e40(1);
  }
  strcpy(&DAT_001dd060,"stdin");
  strcpy(&DAT_001dd460,"stdout");
  iVar1 = fstat(0,(stat *)&DAT_001de180);
  if (iVar1 != 0) {
    FUN_001051e7("standard input");
                    /* WARNING: Subroutine does not return */
    FUN_00108e40(1);
  }
  FUN_00105acc();
  FUN_0010f078();
  DAT_001dd000 = 1;
  DAT_001dd8a0 = 0;
  DAT_001dd860 = 0;
  DAT_001ddd60 = 1;
  if ((DAT_001dd880 != 0) && (DAT_0011909c = FUN_001072f1(0), DAT_0011909c < 0)) {
                    /* WARNING: Subroutine does not return */
    FUN_00108e40(DAT_001dd018);
  }
  while( true ) {
    iVar1 = (*(code *)PTR_FUN_001190d0)(0,1);
    if (iVar1 != 0) {
      return;
    }
    iVar1 = FUN_00105a65();
    if (iVar1 != 0) break;
    DAT_0011909c = FUN_001072f1(DAT_001dd860);
    if (DAT_0011909c < 0) {
      return;
    }
    DAT_001dd048 = 0;
  }
  if (DAT_001dd890 != 0) {
    FUN_00107fdc(DAT_0011909c);
    return;
  }
  if (DAT_001dd894 == 0) {
    return;
  }
  if (DAT_001dd008 != 0) {
    fprintf(stderr," OK\n");
    return;
  }
  if (DAT_001dd880 != 0) {
    return;
  }
  FUN_0010f7a6(DAT_001dd040 - (DAT_001dd048 - DAT_001e1698),DAT_001dd040,stderr);
  fprintf(stderr,"\n");
  return;
}




// Function: atdir_eq @ 0x5dee

undefined4 atdir_eq(undefined *param_1,size_t param_2)

{
  int iVar1;
  undefined4 uVar2;
  size_t local_18;
  undefined *local_10;
  
  local_18 = param_2;
  local_10 = param_1;
  if (param_2 == 0) {
    local_10 = &DAT_00113819;
    local_18 = 1;
  }
  iVar1 = memcmp(&DAT_001ddd80,local_10,local_18);
  if ((iVar1 == 0) && ((&DAT_001ddd80)[local_18] == '\0')) {
    uVar2 = 1;
  }
  else {
    uVar2 = 0;
  }
  return uVar2;
}




// Function: atdir_set @ 0x5e60

int atdir_set(undefined *param_1,size_t param_2)

{
  char cVar1;
  size_t local_18;
  undefined *local_10;
  
  cVar1 = FUN_00105dee(param_1,param_2);
  if (cVar1 != '\x01') {
    if (-1 < DAT_001190a8) {
      close(DAT_001190a8);
    }
    local_18 = param_2;
    local_10 = param_1;
    if (param_2 == 0) {
      local_10 = &DAT_00113819;
      local_18 = 1;
    }
    memcpy(&DAT_001ddd80,local_10,local_18);
    (&DAT_001ddd80)[local_18] = 0;
    DAT_001190a8 = FUN_00110863(&DAT_001ddd80,0x10000);
  }
  return DAT_001190a8;
}




// Function: treat_file @ 0x5f11

void treat_file(char *param_1)

{
  char cVar1;
  int iVar2;
  int iVar3;
  undefined *puVar4;
  int *piVar5;
  char *pcVar6;
  long in_FS_OFFSET;
  sigset_t local_98;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = strcmp(param_1,"-");
  iVar3 = DAT_001dd000;
  if (iVar2 == 0) {
    FUN_00105b3e();
    DAT_001dd000 = iVar3;
  }
  else {
    DAT_001dd860 = FUN_00106d5a(param_1,&DAT_001de180);
    if (-1 < DAT_001dd860) {
      if ((DAT_001de198 & 0xf000) == 0x4000) {
        if (DAT_001dd88c == 0) {
          close(DAT_001dd860);
          if (DAT_001dd004 == 0) {
            fprintf(stderr,"%s: %s is a directory -- ignored\n",DAT_001dd010,&DAT_001dd060);
          }
          if (DAT_001dd018 == 0) {
            DAT_001dd018 = 2;
          }
        }
        else {
          FUN_0010897b(DAT_001dd860,param_1);
        }
      }
      else {
        if (DAT_001dd000 == 0) {
          if ((DAT_001de198 & 0xf000) != 0x8000) {
            if (DAT_001dd004 == 0) {
              fprintf(stderr,"%s: %s is not a directory or a regular file - ignored\n",DAT_001dd010,
                      &DAT_001dd060);
            }
            if (DAT_001dd018 == 0) {
              DAT_001dd018 = 2;
            }
            close(DAT_001dd860);
            goto LAB_001067c6;
          }
          if ((DAT_001de198 & 0x800) != 0) {
            if (DAT_001dd004 == 0) {
              fprintf(stderr,"%s: %s is set-user-ID on execution - ignored\n",DAT_001dd010,
                      &DAT_001dd060);
            }
            if (DAT_001dd018 == 0) {
              DAT_001dd018 = 2;
            }
            close(DAT_001dd860);
            goto LAB_001067c6;
          }
          if ((DAT_001de198 & 0x400) != 0) {
            if (DAT_001dd004 == 0) {
              fprintf(stderr,"%s: %s is set-group-ID on execution - ignored\n",DAT_001dd010,
                      &DAT_001dd060);
            }
            if (DAT_001dd018 == 0) {
              DAT_001dd018 = 2;
            }
            close(DAT_001dd860);
            goto LAB_001067c6;
          }
          if (DAT_001dd884 == 0) {
            if ((DAT_001de198 & 0x200) != 0) {
              if (DAT_001dd004 == 0) {
                fprintf(stderr,"%s: %s has the sticky bit set - file ignored\n",DAT_001dd010,
                        &DAT_001dd060);
              }
              if (DAT_001dd018 == 0) {
                DAT_001dd018 = 2;
              }
              close(DAT_001dd860);
              goto LAB_001067c6;
            }
            if (1 < DAT_001de190) {
              if (DAT_001dd004 == 0) {
                if (DAT_001de190 == 2) {
                  puVar4 = &DAT_0011378f;
                }
                else {
                  puVar4 = &DAT_0011390e;
                }
                fprintf(stderr,"%s: %s has %lu other link%s -- file ignored\n",DAT_001dd010,
                        &DAT_001dd060,DAT_001de190 - 1,puVar4);
              }
              if (DAT_001dd018 == 0) {
                DAT_001dd018 = 2;
              }
              close(DAT_001dd860);
              goto LAB_001067c6;
            }
          }
        }
        FUN_00105acc();
        if ((DAT_001dd000 == 0) || (DAT_001dd008 != 0)) {
          iVar3 = FUN_00106ff8();
          if (iVar3 != 0) {
            close(DAT_001dd860);
            goto LAB_001067c6;
          }
        }
        else {
          strcpy(&DAT_001dd460,"stdout");
        }
        FUN_0010f078();
        DAT_001dd8a0 = 0;
        if ((DAT_001dd880 == 0) || (DAT_0011909c = FUN_001072f1(DAT_001dd860), -1 < DAT_0011909c)) {
          if (DAT_001dd000 == 0) {
            iVar3 = FUN_00106814();
            if (iVar3 != 0) goto LAB_001067c6;
            if ((((DAT_001dd880 == 0) && (DAT_001dd01c != 0)) && (DAT_001dd894 == 0)) &&
               (DAT_001dd004 == 0)) {
              fprintf(stderr,"%s: %s compressed to %s\n",DAT_001dd010,&DAT_001dd060,&DAT_001dd460);
            }
          }
          else {
            DAT_001dd864 = 1;
          }
          if (DAT_001dd01c == 0) {
            DAT_001dd01c = (uint)(DAT_00119090 == 0);
          }
          if ((DAT_001dd894 != 0) && (DAT_001dd890 == 0)) {
            fprintf(stderr,"%s:\t",&DAT_001dd060);
          }
          while (iVar3 = (*(code *)PTR_FUN_001190d0)(DAT_001dd860,DAT_001dd864), iVar3 == 0) {
            iVar3 = FUN_00105a65();
            if ((iVar3 != 0) || (DAT_0011909c = FUN_001072f1(DAT_001dd860), DAT_0011909c < 0))
            goto LAB_0010645e;
            DAT_001dd048 = 0;
          }
          DAT_0011909c = -1;
LAB_0010645e:
          iVar3 = close(DAT_001dd860);
          if (iVar3 != 0) {
            FUN_0010f6c7();
          }
          if (DAT_001dd890 == 0) {
            if (DAT_001dd000 == 0) {
              FUN_0010870d(&DAT_001de180);
              if (((DAT_001dd879 != '\0') &&
                  ((((-1 < DAT_001190a8 && (iVar3 = fdatasync(DAT_001190a8), iVar3 != 0)) &&
                    (piVar5 = __errno_location(), *piVar5 != 0x16)) ||
                   ((iVar3 = fsync(DAT_001dd864), iVar3 != 0 &&
                    (piVar5 = __errno_location(), *piVar5 != 0x16)))))) ||
                 (iVar3 = close(DAT_001dd864), iVar3 != 0)) {
                FUN_0010f74d();
              }
              if (DAT_001dd888 == 0) {
                pcVar6 = (char *)FUN_00110244(&DAT_001dd060);
                cVar1 = FUN_00105dee(&DAT_001dd060,pcVar6 + -0x1dd060);
                iVar3 = DAT_001190a8;
                if (cVar1 == '\0') {
                  iVar3 = -1;
                }
                sigprocmask(0,(sigset_t *)&DAT_001dd8c0,&local_98);
                DAT_001190a4 = 0xffffffff;
                if (iVar3 < 0) {
                  iVar3 = FUN_0010f430(&DAT_001dd060);
                }
                else {
                  iVar3 = unlinkat(iVar3,pcVar6,0);
                }
                if (iVar3 == 0) {
                  iVar3 = 0;
                }
                else {
                  piVar5 = __errno_location();
                  iVar3 = *piVar5;
                }
                sigprocmask(2,&local_98,(sigset_t *)0x0);
                if (iVar3 != 0) {
                  if (DAT_001dd004 == 0) {
                    fprintf(stderr,"%s: ",DAT_001dd010);
                  }
                  if (DAT_001dd018 == 0) {
                    DAT_001dd018 = 2;
                  }
                  if (DAT_001dd004 == 0) {
                    piVar5 = __errno_location();
                    *piVar5 = iVar3;
                    perror(&DAT_001dd060);
                  }
                }
              }
            }
            if (DAT_0011909c == -1) {
              if (DAT_001dd000 == 0) {
                FUN_00108ebb(0);
              }
            }
            else if (DAT_001dd894 != 0) {
              if (DAT_001dd008 == 0) {
                if (DAT_001dd880 == 0) {
                  FUN_0010f7a6(DAT_001dd040 - (DAT_001dd048 - DAT_001e1698),DAT_001dd040,stderr);
                }
                else {
                  FUN_0010f7a6(DAT_001dd048 - (DAT_001dd040 - DAT_001e1698),DAT_001dd048,stderr);
                }
              }
              else {
                fprintf(stderr," OK");
              }
              if (DAT_001dd008 == 0) {
                if (DAT_001dd888 == 0) {
                  pcVar6 = "replaced with";
                }
                else {
                  pcVar6 = "created";
                }
                fprintf(stderr," -- %s %s",pcVar6,&DAT_001dd460);
              }
              fprintf(stderr,"\n");
            }
          }
          else {
            FUN_00107fdc(DAT_0011909c);
          }
        }
        else {
          close(DAT_001dd860);
        }
      }
    }
  }
LAB_001067c6:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: volatile_strcpy @ 0x67dc

void volatile_strcpy(char *param_1,char *param_2)

{
  char cVar1;
  char *local_18;
  char *local_10;
  
  local_18 = param_2;
  local_10 = param_1;
  do {
    cVar1 = *local_18;
    *local_10 = cVar1;
    local_18 = local_18 + 1;
    local_10 = local_10 + 1;
  } while (cVar1 != '\0');
  return;
}




// Function: create_outfile @ 0x6814

undefined8 create_outfile(void)

{
  bool bVar1;
  int iVar2;
  undefined *puVar3;
  int *piVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  int local_b8;
  undefined *local_a8;
  sigset_t local_98;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  bVar1 = false;
  local_a8 = &DAT_001dd460;
  local_b8 = -100;
  if (DAT_001dd888 == 0) {
    puVar3 = (undefined *)FUN_00110244(&DAT_001dd460);
    iVar2 = FUN_00105e60(&DAT_001dd460,puVar3 + -0x1dd460);
    if (-1 < iVar2) {
      local_b8 = iVar2;
      local_a8 = puVar3;
    }
  }
  do {
    while( true ) {
      FUN_001067dc(&DAT_001dd960,&DAT_001dd460);
      sigprocmask(0,(sigset_t *)&DAT_001dd8c0,&local_98);
      DAT_001190a4 = FUN_00110ae4(local_b8,local_a8,0xc1,0x180);
      DAT_001dd864 = DAT_001190a4;
      piVar4 = __errno_location();
      iVar2 = *piVar4;
      sigprocmask(2,&local_98,(sigset_t *)0x0);
      if (-1 < DAT_001dd864) {
        if ((bVar1) && (DAT_001dd880 != 0)) {
          if (DAT_001dd004 == 0) {
            fprintf(stderr,"%s: %s: warning, name truncated\n",DAT_001dd010,&DAT_001dd460);
          }
          if (DAT_001dd018 == 0) {
            DAT_001dd018 = 2;
          }
        }
        uVar5 = 0;
        goto LAB_00106a3e;
      }
      if (iVar2 == 0x11) break;
      if (iVar2 != 0x24) {
        FUN_001051e7(&DAT_001dd460);
        close(DAT_001dd860);
        uVar5 = 1;
        goto LAB_00106a3e;
      }
      FUN_001083ea(&DAT_001dd460);
      bVar1 = true;
    }
    iVar2 = FUN_001085c8();
  } while (iVar2 == 0);
  close(DAT_001dd860);
  uVar5 = 1;
LAB_00106a3e:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: get_suffix @ 0x6a54

char * get_suffix(char *param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  void *__ptr;
  long lVar5;
  size_t sVar6;
  long in_FS_OFFSET;
  byte local_65;
  undefined **local_58;
  char *local_50;
  char local_38 [40];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_65 = 0;
  for (local_58 = &PTR_DAT_001190e8; *local_58 != (undefined *)0x0; local_58 = local_58 + 1) {
    sVar6 = strlen(*local_58);
    if (DAT_001dd8b8 < sVar6) {
      iVar1 = strcmp(DAT_001dd8b0,*local_58 + (sVar6 - DAT_001dd8b8));
      if (iVar1 == 0) {
        local_65 = 1;
        break;
      }
    }
  }
  __ptr = (void *)FUN_00112377(DAT_001dd8b0);
  FUN_0010f393(__ptr);
  if (local_65 == 0) {
    lVar5 = 0;
  }
  else {
    lVar5 = 8;
  }
  (&DAT_001190e0)[lVar5] = __ptr;
  local_58 = (undefined **)(&DAT_001190e0 + local_65);
  sVar6 = strlen(param_1);
  iVar1 = (int)sVar6;
  if (iVar1 < 0x21) {
    strcpy(local_38,param_1);
  }
  else {
    strcpy(local_38,param_1 + (long)iVar1 + -0x20);
  }
  FUN_0010f393(local_38);
  sVar6 = strlen(local_38);
  iVar2 = (int)sVar6;
  local_50 = (char *)0x0;
  do {
    sVar6 = strlen(*local_58);
    iVar3 = (int)sVar6;
    if ((iVar3 < iVar2) && (local_38[(iVar2 - iVar3) + -1] != '/')) {
      iVar4 = strcmp(local_38 + ((long)iVar2 - (long)iVar3),*local_58);
      if (iVar4 == 0) {
        local_50 = param_1 + ((long)iVar1 - (long)iVar3);
        break;
      }
    }
    local_58 = local_58 + 1;
  } while (*local_58 != (undefined *)0x0);
  free(__ptr);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_50;
}




// Function: open_and_stat @ 0x6c6d

int open_and_stat(long param_1,uint param_2,stat *param_3)

{
  int iVar1;
  int iVar2;
  long lVar3;
  int *piVar4;
  uint local_34;
  int local_28;
  long local_18;
  
  local_28 = -100;
  local_34 = param_2;
  if ((DAT_001dd000 == 0) && (DAT_001dd884 == 0)) {
    local_34 = param_2 | 0x20000;
  }
  local_18 = param_1;
  if (DAT_001dd888 == 0) {
    lVar3 = FUN_00110244(param_1);
    iVar1 = FUN_00105e60(param_1,lVar3 - param_1);
    local_18 = param_1;
    if (-1 < iVar1) {
      local_28 = iVar1;
      local_18 = lVar3;
    }
  }
  iVar1 = FUN_00110ae4(local_28,local_18,local_34);
  if ((-1 < iVar1) && (iVar2 = fstat(iVar1,param_3), iVar2 != 0)) {
    piVar4 = __errno_location();
    iVar2 = *piVar4;
    close(iVar1);
    piVar4 = __errno_location();
    *piVar4 = iVar2;
    iVar1 = -1;
  }
  return iVar1;
}




// Function: open_input_file @ 0x6d5a

int open_input_file(char *param_1,undefined8 param_2)

{
  char *__s;
  int iVar1;
  size_t sVar2;
  int *piVar3;
  long lVar4;
  size_t sVar5;
  int local_40;
  undefined **local_30;
  
  local_40 = 0;
  local_30 = &DAT_00119140;
  DAT_00119140 = DAT_001dd8b0;
  sVar2 = strlen(param_1);
  if (sVar2 < 0x3ff) {
    strcpy(&DAT_001dd060,param_1);
    iVar1 = FUN_00106c6d(&DAT_001dd060,0x900,param_2);
    if (iVar1 < 0) {
      if ((DAT_001dd880 == 0) || (piVar3 = __errno_location(), *piVar3 != 2)) {
        FUN_001051e7(&DAT_001dd060);
        iVar1 = -1;
      }
      else {
        lVar4 = FUN_00106a54(&DAT_001dd060);
        if (lVar4 == 0) {
          sVar2 = strlen(&DAT_001dd060);
          iVar1 = strcmp(DAT_001dd8b0,".gz");
          if (iVar1 == 0) {
            local_30 = &PTR_DAT_00119148;
          }
          do {
            __s = *local_30;
            strcpy(&DAT_001dd060,param_1);
            sVar5 = strlen(__s);
            if (0x3ff < sVar5 + (long)(int)sVar2) goto LAB_00106fbd;
            strcat(&DAT_001dd060,__s);
            iVar1 = FUN_00106c6d(&DAT_001dd060,0x900,param_2);
            if (-1 < iVar1) {
              return iVar1;
            }
            piVar3 = __errno_location();
            if (*piVar3 != 2) {
              FUN_001051e7(&DAT_001dd060);
              return -1;
            }
            iVar1 = strcmp(__s,DAT_001dd8b0);
            if (iVar1 == 0) {
              piVar3 = __errno_location();
              local_40 = *piVar3;
            }
            local_30 = local_30 + 1;
          } while (*local_30 != (undefined *)0x0);
          strcpy(&DAT_001dd060,param_1);
          strcat(&DAT_001dd060,DAT_001dd8b0);
          piVar3 = __errno_location();
          *piVar3 = local_40;
          FUN_001051e7(&DAT_001dd060);
          iVar1 = -1;
        }
        else {
          FUN_001051e7(&DAT_001dd060);
          iVar1 = -1;
        }
      }
    }
  }
  else {
LAB_00106fbd:
    fprintf(stderr,"%s: %s: file name too long\n",DAT_001dd010,param_1);
    DAT_001dd018 = 1;
    iVar1 = -1;
  }
  return iVar1;
}




// Function: make_ofname @ 0x6ff8

undefined8 make_ofname(void)

{
  int iVar1;
  char *__s1;
  size_t sVar2;
  
  strcpy(&DAT_001dd460,&DAT_001dd060);
  __s1 = (char *)FUN_00106a54(&DAT_001dd460);
  if (DAT_001dd880 == 0) {
    if ((__s1 != (char *)0x0) && (DAT_001dd884 == 0)) {
      if ((DAT_001dd894 != 0) || ((DAT_001dd88c == 0 && (DAT_001dd004 == 0)))) {
        fprintf(stderr,"%s: %s already has %s suffix -- unchanged\n",DAT_001dd010,&DAT_001dd060,__s1
               );
      }
      return 2;
    }
    DAT_001dd01c = 0;
    sVar2 = strlen(&DAT_001dd460);
    if (0x3ff < sVar2 + DAT_001dd8b8) {
      if (DAT_001dd004 == 0) {
        fprintf(stderr,"%s: %s: file name too long\n",DAT_001dd010,&DAT_001dd060);
      }
      if (DAT_001dd018 == 0) {
        DAT_001dd018 = 2;
      }
      return 2;
    }
    strcat(&DAT_001dd460,DAT_001dd8b0);
  }
  else {
    if (__s1 == (char *)0x0) {
      if ((DAT_001dd88c == 0) && (DAT_001dd008 != 0)) {
        return 0;
      }
      if ((DAT_001dd894 != 0) || ((DAT_001dd88c == 0 && (DAT_001dd004 == 0)))) {
        if (DAT_001dd004 == 0) {
          fprintf(stderr,"%s: %s: unknown suffix -- ignored\n",DAT_001dd010,&DAT_001dd060);
        }
        if (DAT_001dd018 == 0) {
          DAT_001dd018 = 2;
        }
      }
      return 2;
    }
    FUN_0010f393(__s1);
    iVar1 = strcmp(__s1,".tgz");
    if ((iVar1 == 0) || (iVar1 = strcmp(__s1,".taz"), iVar1 == 0)) {
      strcpy(__s1,".tar");
    }
    else {
      *__s1 = '\0';
    }
  }
  return 0;
}




// Function: discard_input_bytes @ 0x7240

void discard_input_bytes(long param_1,uint param_2)

{
  long in_FS_OFFSET;
  long local_20;
  char local_11;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_20 = param_1;
  do {
    while( true ) {
      if (local_20 == 0) goto code_r0x001072db;
      if (DAT_001dd86c < DAT_001dd868) {
        local_11 = (&DAT_0011b000)[DAT_001dd86c];
        DAT_001dd86c = DAT_001dd86c + 1;
      }
      else {
        local_11 = FUN_0010f0bc(0);
      }
      if ((param_2 & 2) != 0) {
        FUN_0010efda(&local_11,1);
      }
      if (local_20 == -1) break;
      local_20 = local_20 + -1;
    }
  } while (local_11 != '\0');
code_r0x001072db:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: get_method @ 0x72f1

uint get_method(undefined4 param_1)

{
  ushort uVar1;
  byte bVar2;
  undefined1 uVar3;
  char cVar4;
  int iVar5;
  uint uVar6;
  uint uVar7;
  uint uVar8;
  ulong uVar9;
  ulong uVar10;
  ulong uVar11;
  ulong uVar12;
  char *__dest;
  char *pcVar13;
  size_t sVar14;
  long in_FS_OFFSET;
  uint local_50;
  uint local_4c;
  uint local_48;
  char *local_38;
  undefined1 local_1c;
  undefined1 local_1b;
  char local_1a;
  undefined1 local_19;
  undefined1 local_18;
  byte local_17;
  undefined1 local_16;
  undefined1 local_15;
  undefined1 local_14;
  undefined1 local_13;
  undefined1 local_12;
  undefined1 local_11;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if ((DAT_001dd884 == 0) || (DAT_001dd000 == 0)) {
    if (DAT_001dd86c < DAT_001dd868) {
      local_1a = (&DAT_0011b000)[DAT_001dd86c];
      DAT_001dd86c = DAT_001dd86c + 1;
    }
    else {
      local_1a = FUN_0010f0bc(0);
    }
    local_50 = 0;
    if (local_1a == '\0') {
      if (DAT_001dd86c < DAT_001dd868) {
        local_4c = (uint)(byte)(&DAT_0011b000)[DAT_001dd86c];
        DAT_001dd86c = DAT_001dd86c + 1;
      }
      else {
        local_4c = FUN_0010f0bc(1);
      }
      local_19 = (undefined1)local_4c;
    }
    else {
      if (DAT_001dd86c < DAT_001dd868) {
        local_19 = (&DAT_0011b000)[DAT_001dd86c];
        DAT_001dd86c = DAT_001dd86c + 1;
      }
      else {
        local_19 = FUN_0010f0bc(0);
      }
      local_4c = 0;
    }
  }
  else {
    if (DAT_001dd86c < DAT_001dd868) {
      local_50 = (uint)(byte)(&DAT_0011b000)[DAT_001dd86c];
      DAT_001dd86c = DAT_001dd86c + 1;
    }
    else {
      local_50 = FUN_0010f0bc(1);
    }
    local_1a = (char)local_50;
    if (DAT_001dd86c < DAT_001dd868) {
      local_4c = (uint)(byte)(&DAT_0011b000)[DAT_001dd86c];
      DAT_001dd86c = DAT_001dd86c + 1;
    }
    else {
      local_4c = FUN_0010f0bc(1);
    }
    local_19 = (undefined1)local_4c;
  }
  DAT_0011909c = 0xffffffff;
  DAT_001dd8a0 = DAT_001dd8a0 + 1;
  DAT_001e1698 = 0;
  DAT_001dd89c = 0;
  iVar5 = memcmp(&local_1a,&DAT_00113a23,2);
  if ((iVar5 == 0) || (iVar5 = memcmp(&local_1a,&DAT_00113a26,2), iVar5 == 0)) {
    if (DAT_001dd86c < DAT_001dd868) {
      DAT_0011909c = (uint)(byte)(&DAT_0011b000)[DAT_001dd86c];
      DAT_001dd86c = DAT_001dd86c + 1;
    }
    else {
      DAT_0011909c = FUN_0010f0bc(0);
    }
    if (DAT_0011909c != 8) {
      fprintf(stderr,"%s: %s: unknown method %d -- not supported\n",DAT_001dd010,&DAT_001dd060,
              (ulong)DAT_0011909c);
      DAT_001dd018 = 1;
      uVar6 = 0xffffffff;
      goto LAB_00107fc6;
    }
    PTR_FUN_001190d0 = FUN_0010e86a;
    if (DAT_001dd86c < DAT_001dd868) {
      bVar2 = (&DAT_0011b000)[DAT_001dd86c];
      DAT_001dd86c = DAT_001dd86c + 1;
    }
    else {
      bVar2 = FUN_0010f0bc(0);
    }
    if ((bVar2 & 0x20) != 0) {
      fprintf(stderr,"%s: %s is encrypted -- not supported\n",DAT_001dd010,&DAT_001dd060);
      DAT_001dd018 = 1;
      uVar6 = 0xffffffff;
      goto LAB_00107fc6;
    }
    if ((bVar2 & 0xc0) != 0) {
      fprintf(stderr,"%s: %s has flags 0x%x -- not supported\n",DAT_001dd010,&DAT_001dd060,
              (ulong)bVar2);
      DAT_001dd018 = 1;
      if (DAT_001dd884 < 2) {
        uVar6 = 0xffffffff;
        goto LAB_00107fc6;
      }
    }
    if (DAT_001dd86c < DAT_001dd868) {
      uVar9 = (ulong)(byte)(&DAT_0011b000)[DAT_001dd86c];
      DAT_001dd86c = DAT_001dd86c + 1;
    }
    else {
      iVar5 = FUN_0010f0bc(0);
      uVar9 = (ulong)iVar5;
    }
    if (DAT_001dd86c < DAT_001dd868) {
      uVar10 = (ulong)(byte)(&DAT_0011b000)[DAT_001dd86c];
      DAT_001dd86c = DAT_001dd86c + 1;
    }
    else {
      iVar5 = FUN_0010f0bc(0);
      uVar10 = (ulong)iVar5;
    }
    uVar10 = uVar9 | uVar10 << 8;
    if (DAT_001dd86c < DAT_001dd868) {
      uVar11 = (ulong)(byte)(&DAT_0011b000)[DAT_001dd86c];
      DAT_001dd86c = DAT_001dd86c + 1;
    }
    else {
      iVar5 = FUN_0010f0bc(0);
      uVar11 = (ulong)iVar5;
    }
    uVar11 = uVar10 | uVar11 << 0x10;
    if (DAT_001dd86c < DAT_001dd868) {
      uVar12 = (ulong)(byte)(&DAT_0011b000)[DAT_001dd86c];
      DAT_001dd86c = DAT_001dd86c + 1;
    }
    else {
      iVar5 = FUN_0010f0bc(0);
      uVar12 = (ulong)iVar5;
    }
    uVar12 = uVar11 | uVar12 << 0x18;
    if ((uVar12 != 0) && (DAT_00119094 == 0)) {
      if ((long)uVar12 < 0) {
        if (DAT_001dd004 == 0) {
          fprintf(stderr,"%s: %s: MTIME %lu out of range for this platform\n",DAT_001dd010,
                  &DAT_001dd060,uVar12);
        }
        if (DAT_001dd018 == 0) {
          DAT_001dd018 = 2;
        }
        DAT_001dd030 = 0x7fffffffffffffff;
        DAT_001dd038 = 999999999;
      }
      else {
        DAT_001dd038 = 0;
        DAT_001dd030 = uVar12;
      }
    }
    if (DAT_001dd86c < DAT_001dd868) {
      local_12 = (&DAT_0011b000)[DAT_001dd86c];
      DAT_001dd86c = DAT_001dd86c + 1;
    }
    else {
      local_12 = FUN_0010f0bc(0);
    }
    if (DAT_001dd86c < DAT_001dd868) {
      local_11 = (&DAT_0011b000)[DAT_001dd86c];
      DAT_001dd86c = DAT_001dd86c + 1;
    }
    else {
      local_11 = FUN_0010f0bc(0);
    }
    if ((bVar2 & 2) != 0) {
      local_18 = 8;
      local_16 = (undefined1)uVar9;
      local_15 = (undefined1)(uVar10 >> 8);
      local_14 = (undefined1)(uVar11 >> 0x10);
      local_13 = (undefined1)(uVar12 >> 0x18);
      local_17 = bVar2;
      FUN_0010efda(0,0);
      FUN_0010efda(&local_1a,10);
    }
    if ((bVar2 & 4) != 0) {
      if (DAT_001dd86c < DAT_001dd868) {
        uVar3 = (&DAT_0011b000)[DAT_001dd86c];
        DAT_001dd86c = DAT_001dd86c + 1;
      }
      else {
        uVar3 = FUN_0010f0bc(0);
      }
      local_1c = uVar3;
      if (DAT_001dd86c < DAT_001dd868) {
        local_1b = (&DAT_0011b000)[DAT_001dd86c];
        DAT_001dd86c = DAT_001dd86c + 1;
      }
      else {
        local_1b = FUN_0010f0bc(0);
      }
      uVar1 = CONCAT11(local_1b,uVar3);
      if (DAT_001dd894 != 0) {
        fprintf(stderr,"%s: %s: extra field of %u bytes ignored\n",DAT_001dd010,&DAT_001dd060,
                (ulong)(uint)uVar1);
      }
      if ((bVar2 & 2) != 0) {
        FUN_0010efda(&local_1c,2);
      }
      FUN_00107240((uint)uVar1,bVar2);
    }
    if ((bVar2 & 8) != 0) {
      if ((DAT_00119090 == 0) &&
         (((DAT_001dd000 == 0 || (DAT_001dd890 != 0)) && (DAT_001dd8a0 < 2)))) {
        __dest = (char *)FUN_0010f40a(&DAT_001dd460);
        local_38 = __dest;
        do {
          if (DAT_001dd86c < DAT_001dd868) {
            cVar4 = (&DAT_0011b000)[DAT_001dd86c];
            DAT_001dd86c = DAT_001dd86c + 1;
          }
          else {
            cVar4 = FUN_0010f0bc(0);
          }
          *local_38 = cVar4;
          pcVar13 = local_38 + 1;
          if (*local_38 == '\0') goto code_r0x00107aeb;
          local_38 = pcVar13;
        } while (pcVar13 < &DAT_001dd860);
        FUN_0010f5ef("corrupted input -- file name too large");
code_r0x00107aeb:
        if ((bVar2 & 2) != 0) {
          FUN_0010efda(__dest,(int)pcVar13 - (int)__dest);
        }
        pcVar13 = (char *)FUN_0010f40a(__dest);
        sVar14 = strlen(pcVar13);
        memmove(__dest,pcVar13,sVar14 + 1);
        if ((DAT_001dd890 == 0) && (__dest != (char *)0x0)) {
          DAT_001dd890 = 0;
        }
      }
      else {
        FUN_00107240(0xffffffffffffffff,bVar2);
      }
    }
    if ((bVar2 & 0x10) != 0) {
      FUN_00107240(0xffffffffffffffff,bVar2);
    }
    if ((bVar2 & 2) != 0) {
      uVar6 = FUN_0010efda(&local_1a,0);
      if (DAT_001dd86c < DAT_001dd868) {
        uVar7 = (uint)(byte)(&DAT_0011b000)[DAT_001dd86c];
        DAT_001dd86c = DAT_001dd86c + 1;
      }
      else {
        uVar7 = FUN_0010f0bc(0);
      }
      if (DAT_001dd86c < DAT_001dd868) {
        uVar8 = (uint)(byte)(&DAT_0011b000)[DAT_001dd86c];
        DAT_001dd86c = DAT_001dd86c + 1;
      }
      else {
        uVar8 = FUN_0010f0bc(0);
      }
      uVar7 = uVar7 | uVar8 << 8;
      if (uVar7 != (uVar6 & 0xffff)) {
        fprintf(stderr,"%s: %s: header checksum 0x%04x != computed checksum 0x%04x\n",DAT_001dd010,
                &DAT_001dd060,(ulong)uVar7,(ulong)(uVar6 & 0xffff));
        DAT_001dd018 = 1;
        if (DAT_001dd884 < 2) {
          uVar6 = 0xffffffff;
          goto LAB_00107fc6;
        }
      }
    }
    if (DAT_001dd8a0 == 1) {
      DAT_001e1698 = (ulong)(DAT_001dd86c + 8);
    }
  }
  else {
    iVar5 = memcmp(&local_1a,&DAT_00113b7c,2);
    if (((iVar5 == 0) && (DAT_001dd86c == 2)) &&
       (iVar5 = memcmp(&DAT_0011b000,&DAT_00113b7c,4), iVar5 == 0)) {
      DAT_001dd86c = 0;
      PTR_FUN_001190d0 = FUN_0010e86a;
      iVar5 = FUN_0010e655(param_1);
      if (iVar5 != 0) {
        uVar6 = 0xffffffff;
        goto LAB_00107fc6;
      }
      DAT_001dd89c = 1;
    }
    else {
      iVar5 = memcmp(&local_1a,&DAT_00113b81,2);
      if (iVar5 == 0) {
        PTR_FUN_001190d0 = FUN_0010e3fd;
        DAT_0011909c = 2;
      }
      else {
        iVar5 = memcmp(&local_1a,&DAT_00113b84,2);
        if (iVar5 == 0) {
          PTR_FUN_001190d0 = FUN_0010d846;
          DAT_0011909c = 1;
          DAT_001dd89c = 1;
        }
        else {
          iVar5 = memcmp(&local_1a,&DAT_00113b87,2);
          if (iVar5 == 0) {
            PTR_FUN_001190d0 = FUN_0010d7d6;
            DAT_0011909c = 3;
            DAT_001dd89c = 1;
          }
          else if (((DAT_001dd884 != 0) && (DAT_001dd000 != 0)) && (DAT_001dd890 == 0)) {
            DAT_0011909c = 0;
            PTR_FUN_001190d0 = FUN_0010ef1d;
            if (local_4c != 0xffffffff) {
              DAT_001dd86c = DAT_001dd86c - 1;
            }
            DAT_001dd89c = 1;
            if (local_50 != 0xffffffff) {
              FUN_0010f321(1,&local_1a,1);
            }
          }
        }
      }
    }
  }
  uVar6 = DAT_0011909c;
  if ((int)DAT_0011909c < 0) {
    if (DAT_001dd8a0 == 1) {
      fprintf(stderr,"\n%s: %s: not in gzip format\n",DAT_001dd010,&DAT_001dd060);
      DAT_001dd018 = 1;
      uVar6 = 0xffffffff;
    }
    else {
      if (local_1a == '\0') {
        local_48 = local_4c;
        while (local_48 == 0) {
          if (DAT_001dd86c < DAT_001dd868) {
            local_48 = (uint)(byte)(&DAT_0011b000)[DAT_001dd86c];
            DAT_001dd86c = DAT_001dd86c + 1;
          }
          else {
            local_48 = FUN_0010f0bc(1);
          }
        }
        if (local_48 == 0xffffffff) {
          if (DAT_001dd894 != 0) {
            if (DAT_001dd004 == 0) {
              fprintf(stderr,"\n%s: %s: decompression OK, trailing zero bytes ignored\n",
                      DAT_001dd010,&DAT_001dd060);
            }
            if (DAT_001dd018 == 0) {
              DAT_001dd018 = 2;
            }
          }
          uVar6 = 0xfffffffd;
          goto LAB_00107fc6;
        }
      }
      if (DAT_001dd004 == 0) {
        fprintf(stderr,"\n%s: %s: decompression OK, trailing garbage ignored\n",DAT_001dd010,
                &DAT_001dd060);
      }
      if (DAT_001dd018 == 0) {
        DAT_001dd018 = 2;
      }
      uVar6 = 0xfffffffe;
    }
  }
LAB_00107fc6:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar6;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: do_list @ 0x7fdc

void do_list(int param_1)

{
  tm *ptVar1;
  undefined8 local_18;
  
  if ((DAT_001190c8 == 0) || (param_1 < 0)) {
    if (param_1 < 0) {
      if (DAT_001ddd68 < 1) {
        return;
      }
      if (DAT_001ddd70 < 1) {
        return;
      }
      if (DAT_001dd894 != 0) {
        printf("                            ");
      }
      if ((DAT_001dd894 != 0) || (DAT_001dd004 == 0)) {
        FUN_0010f81a(stdout,DAT_001ddd68,0x13);
        printf(" ");
        FUN_0010f81a(stdout,DAT_001ddd70,0x13);
        printf(" ");
      }
      FUN_0010f7a6(DAT_001ddd70 - (DAT_001ddd68 - DAT_001e1698),DAT_001ddd70,stdout);
      printf(" (totals)\n");
      return;
    }
  }
  else {
    DAT_001190c8 = 0;
    if (DAT_001dd894 != 0) {
      printf("method  crc     date  time  ");
    }
    if (DAT_001dd004 == 0) {
      printf("%*.*s %*.*s  ratio uncompressed_name\n",0x13,0x13,"compressed",0x13,0x13,
             "uncompressed");
    }
  }
  local_18 = 0xffffffffffffffff;
  if ((param_1 == 8) && (DAT_001dd89c == 0)) {
    local_18 = DAT_001e1680;
  }
  if (DAT_001dd894 != 0) {
    ptVar1 = localtime(&DAT_001dd030);
    printf("%5s %08lx ",(&PTR_s_store_00118b20)[param_1],local_18);
    if (ptVar1 == (tm *)0x0) {
      printf("??? ?? ??:?? ");
    }
    else {
      printf("%s%3d %02d:%02d ",&DAT_001142e0 + (long)ptVar1->tm_mon * 4,
             (ulong)(uint)ptVar1->tm_mday,(ulong)(uint)ptVar1->tm_hour,(ulong)(uint)ptVar1->tm_min);
    }
  }
  FUN_0010f81a(stdout,DAT_001dd040,0x13);
  printf(" ");
  FUN_0010f81a(stdout,DAT_001dd048,0x13);
  printf(" ");
  if (DAT_001dd040 == -1) {
    DAT_001ddd68 = -1;
    DAT_001e1698 = 0;
    DAT_001dd048 = 0;
    DAT_001dd040 = 0;
  }
  else if (-1 < DAT_001ddd68) {
    DAT_001ddd68 = DAT_001dd040 + DAT_001ddd68;
  }
  if (DAT_001dd048 == -1) {
    DAT_001ddd70 = -1;
    DAT_001e1698 = 0;
    DAT_001dd048 = 0;
    DAT_001dd040 = 0;
  }
  else if (-1 < DAT_001ddd70) {
    DAT_001ddd70 = DAT_001dd048 + DAT_001ddd70;
  }
  FUN_0010f7a6(DAT_001dd048 - (DAT_001dd040 - DAT_001e1698),DAT_001dd048,stdout);
  printf(" %s\n",&DAT_001dd460);
  return;
}




// Function: shorten_name @ 0x83ea

void shorten_name(char *param_1)

{
  char *pcVar1;
  int iVar2;
  size_t sVar3;
  undefined1 *puVar4;
  int local_24;
  char *local_18;
  char *local_10;
  
  local_18 = (char *)0x0;
  local_24 = 3;
  sVar3 = strlen(param_1);
  iVar2 = (int)sVar3;
  if (DAT_001dd880 == 0) {
    puVar4 = (undefined1 *)FUN_00106a54(param_1);
    if (puVar4 == (undefined1 *)0x0) {
      FUN_0010f5ef("can\'t recover suffix\n");
    }
    *puVar4 = 0;
    DAT_001dd01c = 1;
    if ((iVar2 < 5) || (iVar2 = strcmp(puVar4 + -4,".tar"), iVar2 != 0)) {
      do {
        local_10 = (char *)FUN_00110244(param_1);
        while (*local_10 != '\0') {
          sVar3 = strcspn(local_10,".");
          local_10 = local_10 + (int)sVar3;
          if (local_24 < (int)sVar3) {
            local_18 = local_10 + -1;
          }
          if (*local_10 != '\0') {
            local_10 = local_10 + 1;
          }
        }
      } while ((local_18 == (char *)0x0) && (local_24 = local_24 + -1, local_24 != 0));
      pcVar1 = local_18;
      if (local_18 == (char *)0x0) {
        local_18 = strrchr(param_1,0x2e);
        if (local_18 == (char *)0x0) {
          FUN_0010f5ef("internal error in shorten_name");
        }
        if (local_18[1] == '\0') {
          local_18 = local_18 + -1;
        }
      }
      else {
        do {
          local_18 = pcVar1;
          *local_18 = local_18[1];
          pcVar1 = local_18 + 1;
        } while (*local_18 != '\0');
      }
      strcpy(local_18,DAT_001dd8b0);
    }
    else {
      strcpy(puVar4 + -4,".tgz");
    }
  }
  else {
    if (iVar2 < 2) {
      FUN_0010f5ef("name too short");
    }
    param_1[(long)iVar2 + -1] = '\0';
  }
  return;
}




// Function: check_ofname @ 0x85c8

bool check_ofname(void)

{
  char cVar1;
  int iVar2;
  
  if (DAT_001dd884 == 0) {
    cVar1 = '\0';
    fprintf(stderr,"%s: %s already exists;",DAT_001dd010,&DAT_001dd460);
    if ((DAT_001dd898 != 0) && ((DAT_001dd878 != '\0' || (iVar2 = isatty(0), iVar2 != 0)))) {
      fprintf(stderr," do you wish to overwrite (y or n)? ");
      FUN_001109d7(stderr);
      cVar1 = FUN_001123a8();
    }
    if (cVar1 == '\0') {
      fprintf(stderr,"\tnot overwritten\n");
      if (DAT_001dd018 == 0) {
        DAT_001dd018 = 2;
      }
      return true;
    }
  }
  iVar2 = FUN_0010f430(&DAT_001dd460);
  if (iVar2 != 0) {
    FUN_001051e7(&DAT_001dd460);
  }
  return iVar2 != 0;
}




// Function: do_chown @ 0x86dc

void do_chown(int param_1,undefined8 param_2,__uid_t param_3,__gid_t param_4)

{
  fchown(param_1,param_3,param_4);
  return;
}




// Function: copy_stat @ 0x870d

void copy_stat(long param_1)

{
  uint uVar1;
  undefined1 auVar2 [16];
  bool bVar3;
  int iVar4;
  int *piVar5;
  long in_FS_OFFSET;
  undefined1 local_38 [16];
  undefined1 local_28 [16];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = *(uint *)(param_1 + 0x18);
  local_38 = FUN_0011105b(param_1);
  local_28 = FUN_0011108f(param_1);
  auVar2._8_8_ = DAT_001dd038;
  auVar2._0_8_ = DAT_001dd030;
  if (((DAT_001dd880 == 0) || (DAT_001dd038 < 0)) || (local_28 == auVar2)) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  if (bVar3) {
    local_28._8_8_ = DAT_001dd038;
    local_28._0_8_ = DAT_001dd030;
  }
  iVar4 = FUN_001113ca(DAT_001dd864,&DAT_001dd460,local_38);
  if (iVar4 == 0) {
    if ((bVar3) && (1 < DAT_001dd894)) {
      fprintf(stderr,"%s: timestamp restored\n",&DAT_001dd460);
    }
  }
  else {
    piVar5 = __errno_location();
    iVar4 = *piVar5;
    if (DAT_001dd004 == 0) {
      fprintf(stderr,"%s: ",DAT_001dd010);
    }
    if (DAT_001dd018 == 0) {
      DAT_001dd018 = 2;
    }
    if (DAT_001dd004 == 0) {
      piVar5 = __errno_location();
      *piVar5 = iVar4;
      perror(&DAT_001dd460);
    }
  }
  FUN_001086dc(DAT_001dd864,&DAT_001dd460,0xffffffff,*(undefined4 *)(param_1 + 0x20));
  iVar4 = fchmod(DAT_001dd864,uVar1 & 0x1ff);
  if (iVar4 != 0) {
    piVar5 = __errno_location();
    iVar4 = *piVar5;
    if (DAT_001dd004 == 0) {
      fprintf(stderr,"%s: ",DAT_001dd010);
    }
    if (DAT_001dd018 == 0) {
      DAT_001dd018 = 2;
    }
    if (DAT_001dd004 == 0) {
      piVar5 = __errno_location();
      *piVar5 = iVar4;
      perror(&DAT_001dd460);
    }
  }
  FUN_001086dc(DAT_001dd864,&DAT_001dd460,*(undefined4 *)(param_1 + 0x1c),0xffffffff);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: treat_dir @ 0x897b

void treat_dir(int param_1,char *param_2)

{
  int iVar1;
  DIR *__dirp;
  char *__ptr;
  size_t sVar2;
  char *pcVar3;
  long in_FS_OFFSET;
  char *local_440;
  size_t local_438;
  undefined8 local_420;
  char local_418 [1032];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  __dirp = fdopendir(param_1);
  if (__dirp == (DIR *)0x0) {
    FUN_001051e7(param_2);
    close(param_1);
  }
  else {
    __ptr = (char *)FUN_00110c61(__dirp,0);
    if (__ptr == (char *)0x0) {
      FUN_001051e7(param_2);
    }
    iVar1 = closedir(__dirp);
    if (iVar1 != 0) {
      FUN_001051e7(param_2);
    }
    local_440 = __ptr;
    if (__ptr != (char *)0x0) {
      for (; *local_440 != '\0'; local_440 = local_440 + local_420 + 1) {
        sVar2 = strlen(param_2);
        local_420 = strlen(local_440);
        iVar1 = strcmp(local_440,".");
        if ((iVar1 != 0) && (iVar1 = strcmp(local_440,".."), iVar1 != 0)) {
          if (local_420 + sVar2 < 0x3fe) {
            strcpy(local_418,param_2);
            pcVar3 = (char *)FUN_00110244(local_418);
            local_438 = sVar2;
            if ((*pcVar3 != '\0') && (local_418[sVar2 - 1] != '/')) {
              local_438 = sVar2 + 1;
              local_418[sVar2] = '/';
            }
            strcpy(local_418 + local_438,local_440);
            FUN_00105f11(local_418);
          }
          else {
            fprintf(stderr,"%s: %s/%s: pathname too long\n",DAT_001dd010,param_2,local_440);
            DAT_001dd018 = 1;
          }
        }
      }
      free(__ptr);
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: install_signal_handlers @ 0x8c07

void install_signal_handlers(void)

{
  int iVar1;
  long in_FS_OFFSET;
  int local_b0;
  _union_1457 local_a8;
  undefined8 local_a0;
  undefined8 local_98;
  undefined8 local_90;
  undefined8 local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 local_70;
  undefined8 local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined8 local_40;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 local_28;
  undefined4 local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  sigemptyset((sigset_t *)&DAT_001dd8c0);
  for (local_b0 = 0; local_b0 < 6; local_b0 = local_b0 + 1) {
    sigaction(*(int *)(&DAT_001190b0 + (long)local_b0 * 4),(sigaction *)0x0,(sigaction *)&local_a8);
    if (local_a8.sa_handler != (__sighandler_t)0x1) {
      sigaddset((sigset_t *)&DAT_001dd8c0,*(int *)(&DAT_001190b0 + (long)local_b0 * 4));
    }
  }
  local_a8.sa_handler = FUN_00108fbb;
  local_a0 = DAT_001dd8c0;
  local_98 = DAT_001dd8c8;
  local_90 = DAT_001dd8d0;
  local_88 = DAT_001dd8d8;
  local_80 = DAT_001dd8e0;
  local_78 = DAT_001dd8e8;
  local_70 = DAT_001dd8f0;
  local_68 = DAT_001dd8f8;
  local_60 = DAT_001dd900;
  local_58 = DAT_001dd908;
  local_50 = DAT_001dd910;
  local_48 = DAT_001dd918;
  local_40 = DAT_001dd920;
  local_38 = DAT_001dd928;
  local_30 = DAT_001dd930;
  local_28 = DAT_001dd938;
  local_20 = 0;
  for (local_b0 = 0; local_b0 < 6; local_b0 = local_b0 + 1) {
    iVar1 = sigismember((sigset_t *)&DAT_001dd8c0,*(int *)(&DAT_001190b0 + (long)local_b0 * 4));
    if (iVar1 != 0) {
      if (local_b0 == 0) {
        DAT_001dd898 = 1;
      }
      sigaction(*(int *)(&DAT_001190b0 + (long)local_b0 * 4),(sigaction *)&local_a8,(sigaction *)0x0
               );
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: do_exit @ 0x8e40

void do_exit(int param_1)

{
  if (DAT_001de210 != 0) {
                    /* WARNING: Subroutine does not return */
    exit(param_1);
  }
  DAT_001de210 = 1;
  free(DAT_001dd8a8);
  DAT_001dd8a8 = (void *)0x0;
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: finish_out @ 0x8e91

void finish_out(void)

{
  int iVar1;
  
  iVar1 = FUN_00110304(stdout);
  if (iVar1 != 0) {
    FUN_0010f74d();
  }
                    /* WARNING: Subroutine does not return */
  FUN_00108e40(0);
}




// Function: remove_output_file @ 0x8ebb

void remove_output_file(char param_1)

{
  int __fd;
  long in_FS_OFFSET;
  sigset_t local_498;
  undefined1 local_418 [1032];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 != '\x01') {
    sigprocmask(0,(sigset_t *)&DAT_001dd8c0,&local_498);
  }
  __fd = DAT_001190a4;
  if (-1 < DAT_001190a4) {
    DAT_001190a4 = -1;
    close(__fd);
    FUN_001067dc(local_418,&DAT_001dd960);
    FUN_0010f430(local_418);
  }
  if (param_1 != '\x01') {
    sigprocmask(2,&local_498,(sigset_t *)0x0);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: abort_gzip @ 0x8f9f

void abort_gzip(void)

{
  FUN_00108ebb(0);
                    /* WARNING: Subroutine does not return */
  FUN_00108e40(1);
}




// Function: abort_gzip_signal @ 0x8fbb

void abort_gzip_signal(int param_1)

{
  FUN_00108ebb(1);
  if (param_1 == DAT_001dd940) {
                    /* WARNING: Subroutine does not return */
    _exit(2);
  }
  signal(param_1,(__sighandler_t)0x0);
  raise(param_1);
  return;
}




// Function: huft_build @ 0x9005

undefined8
huft_build(uint *param_1,uint param_2,uint param_3,long param_4,long param_5,long *param_6,
            uint *param_7)

{
  undefined8 *puVar1;
  int iVar2;
  uint uVar3;
  char cVar4;
  byte bVar5;
  uint uVar6;
  uint uVar7;
  void *pvVar8;
  undefined8 uVar9;
  uint uVar10;
  ulong uVar11;
  ulong uVar12;
  uint uVar13;
  uint uVar14;
  uint uVar15;
  ulong uVar16;
  uint *puVar17;
  uint *puVar18;
  long lVar19;
  int iVar20;
  long in_FS_OFFSET;
  int local_65c;
  long *local_650;
  uint local_61c;
  int local_618;
  int local_614;
  uint local_610;
  int local_60c;
  uint local_608;
  uint *local_600;
  undefined1 local_5f8;
  undefined6 uStack_5f6;
  long local_5f0;
  uint local_5e8 [20];
  uint local_598 [14];
  uint auStack_560 [6];
  long local_548 [16];
  uint local_4c8 [290];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  memset(local_5e8,0,0x44);
  puVar18 = param_1;
  uVar13 = param_2;
  do {
    local_5e8[*puVar18] = local_5e8[*puVar18] + 1;
    puVar18 = puVar18 + 1;
    uVar13 = uVar13 - 1;
  } while (uVar13 != 0);
  if (param_2 == local_5e8[0]) {
    pvVar8 = malloc(0x30);
    if (pvVar8 == (void *)0x0) {
      uVar9 = 3;
    }
    else {
      DAT_001de224 = DAT_001de224 + 3;
      *(undefined8 *)((long)pvVar8 + 8) = 0;
      *(undefined1 *)((long)pvVar8 + 0x10) = 99;
      *(undefined1 *)((long)pvVar8 + 0x11) = 1;
      *(undefined1 *)((long)pvVar8 + 0x20) = 99;
      *(undefined1 *)((long)pvVar8 + 0x21) = 1;
      *param_6 = (long)pvVar8 + 0x10;
      *param_7 = 1;
      uVar9 = 0;
    }
  }
  else {
    uVar11 = 1;
    while ((uVar13 = (uint)uVar11, uVar13 < 0x11 && (local_5e8[uVar11] == 0))) {
      uVar11 = (ulong)(uVar13 + 1);
    }
    local_610 = *param_7;
    if (*param_7 < uVar13) {
      local_610 = uVar13;
    }
    uVar16 = 0x10;
    while ((uVar13 = (uint)uVar16, uVar13 != 0 && (local_5e8[uVar16] == 0))) {
      uVar16 = (ulong)(uVar13 - 1);
    }
    if (uVar13 < local_610) {
      local_610 = uVar13;
    }
    *param_7 = local_610;
    local_60c = 1 << ((byte)uVar11 & 0x1f);
    for (uVar12 = uVar11; (uint)uVar12 < uVar13; uVar12 = (ulong)((uint)uVar12 + 1)) {
      if ((int)(local_60c - local_5e8[uVar12]) < 0) {
        uVar9 = 2;
        goto LAB_00109735;
      }
      local_60c = (local_60c - local_5e8[uVar12]) * 2;
    }
    local_60c = local_60c - local_5e8[uVar16];
    if (local_60c < 0) {
      uVar9 = 2;
    }
    else {
      local_5e8[uVar16] = local_60c + local_5e8[uVar16];
      uVar15 = 0;
      local_598[1] = 0;
      puVar18 = local_5e8;
      local_600 = local_598 + 2;
      while (puVar18 = puVar18 + 1, uVar14 = (int)uVar16 - 1, uVar16 = (ulong)uVar14, uVar14 != 0) {
        uVar15 = uVar15 + *puVar18;
        *local_600 = uVar15;
        local_600 = local_600 + 1;
      }
      uVar15 = 0;
      do {
        uVar14 = *param_1;
        if (uVar14 != 0) {
          uVar3 = local_598[uVar14];
          local_598[uVar14] = uVar3 + 1;
          local_4c8[uVar3] = uVar15;
        }
        uVar15 = uVar15 + 1;
        param_1 = param_1 + 1;
      } while (uVar15 < param_2);
      uVar15 = local_598[(int)uVar13];
      uVar14 = 0;
      local_598[0] = 0;
      puVar18 = local_4c8;
      local_614 = -1;
      local_65c = -local_610;
      local_548[0] = 0;
      lVar19 = 0;
      local_608 = 0;
      local_650 = param_6;
      while (iVar20 = (int)uVar11, iVar20 <= (int)uVar13) {
        local_61c = local_5e8[iVar20];
        while (uVar3 = local_61c - 1, iVar2 = local_614, local_61c != 0) {
          while (local_614 = iVar2, (int)(local_610 + local_65c) < iVar20) {
            iVar2 = local_614 + 1;
            local_65c = local_610 + local_65c;
            uVar10 = uVar13 - local_65c;
            if (local_610 < uVar10) {
              uVar10 = local_610;
            }
            uVar6 = iVar20 - local_65c;
            uVar7 = 1 << ((byte)uVar6 & 0x1f);
            if (local_61c < uVar7) {
              local_618 = (uVar7 - uVar3) + -1;
              local_600 = local_5e8 + iVar20;
              if (uVar6 < uVar10) {
                while (uVar6 = uVar6 + 1, uVar6 < uVar10) {
                  local_600 = local_600 + 1;
                  if ((uint)(local_618 * 2) <= *local_600) break;
                  local_618 = local_618 * 2 - *local_600;
                }
              }
            }
            local_608 = 1 << ((byte)uVar6 & 0x1f);
            pvVar8 = malloc((ulong)(local_608 + 1) << 4);
            if (pvVar8 == (void *)0x0) {
              if (iVar2 != 0) {
                FUN_0010975b(local_548[0]);
              }
              uVar9 = 3;
              goto LAB_00109735;
            }
            DAT_001de224 = local_608 + DAT_001de224 + 1;
            *local_650 = (long)pvVar8 + 0x10;
            local_650 = (long *)((long)pvVar8 + 8);
            *local_650 = 0;
            lVar19 = (long)pvVar8 + 0x10;
            local_548[iVar2] = lVar19;
            if (iVar2 != 0) {
              local_598[iVar2] = uVar14;
              puVar1 = (undefined8 *)
                       (local_548[local_614] +
                       (ulong)(uVar14 >> ((char)local_65c - (char)local_610 & 0x1fU)) * 0x10);
              *puVar1 = CONCAT62(uStack_5f6,CONCAT11((char)local_610,(byte)uVar6 + 0x10));
              puVar1[1] = lVar19;
              local_5f0 = lVar19;
            }
          }
          bVar5 = (byte)local_65c;
          cVar4 = (char)uVar11;
          if (puVar18 < local_4c8 + uVar15) {
            if (*puVar18 < param_3) {
              if (*puVar18 < 0x100) {
                local_5f8 = 0x10;
              }
              else {
                local_5f8 = 0xf;
              }
              local_5f0 = CONCAT62(local_5f0._2_6_,(short)*puVar18);
              puVar17 = puVar18 + 1;
            }
            else {
              local_5f8 = (undefined1)*(undefined2 *)(param_5 + (ulong)(*puVar18 - param_3) * 2);
              puVar17 = puVar18 + 1;
              local_5f0 = CONCAT62(local_5f0._2_6_,
                                   *(undefined2 *)(param_4 + (ulong)(*puVar18 - param_3) * 2));
            }
          }
          else {
            local_5f8 = 99;
            puVar17 = puVar18;
          }
          for (uVar10 = uVar14 >> (bVar5 & 0x1f); uVar10 < local_608;
              uVar10 = (1 << (cVar4 - bVar5 & 0x1f)) + uVar10) {
            puVar1 = (undefined8 *)(lVar19 + (ulong)uVar10 * 0x10);
            *puVar1 = CONCAT62(uStack_5f6,CONCAT11(cVar4 - bVar5,local_5f8));
            puVar1[1] = local_5f0;
          }
          uVar10 = 1 << (cVar4 - 1U & 0x1f);
          while ((uVar14 & uVar10) != 0) {
            uVar14 = uVar14 ^ uVar10;
            uVar10 = uVar10 >> 1;
          }
          uVar14 = uVar14 ^ uVar10;
          for (; puVar18 = puVar17, local_61c = uVar3,
              ((1 << ((byte)local_65c & 0x1f)) - 1U & uVar14) != local_598[local_614];
              local_614 = local_614 + -1) {
            local_65c = local_65c - local_610;
          }
        }
        uVar11 = (ulong)(iVar20 + 1);
      }
      if ((local_60c == 0) || (uVar13 == 1)) {
        uVar9 = 0;
      }
      else {
        uVar9 = 1;
      }
    }
  }
LAB_00109735:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar9;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: huft_free @ 0x975b

undefined8 huft_free(long param_1)

{
  void *__ptr;
  
  while (param_1 != 0) {
    __ptr = (void *)(param_1 + -0x10);
    param_1 = *(long *)(param_1 + -8);
    free(__ptr);
  }
  return 0;
}




// Function: inflate_codes @ 0x979a

undefined8 inflate_codes(long param_1,long param_2,uint param_3,uint param_4)

{
  ushort uVar1;
  ushort uVar2;
  byte bVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  ulong __n;
  ulong uVar8;
  uint local_44;
  uint local_40;
  uint local_3c;
  byte *local_30;
  
  local_3c = DAT_001dd870;
  uVar1 = *(ushort *)(&DAT_001192e0 + (long)(int)param_3 * 2);
  uVar2 = *(ushort *)(&DAT_001192e0 + (long)(int)param_4 * 2);
  uVar8 = DAT_001de218;
  uVar5 = DAT_001de220;
  while( true ) {
    while( true ) {
      for (; uVar5 < param_3; uVar5 = uVar5 + 8) {
        if (DAT_001dd86c < DAT_001dd868) {
          bVar3 = (&DAT_0011b000)[DAT_001dd86c];
          DAT_001dd86c = DAT_001dd86c + 1;
        }
        else {
          DAT_001dd870 = local_3c;
          bVar3 = FUN_0010f0bc(0);
        }
        uVar8 = uVar8 | (ulong)bVar3 << ((byte)uVar5 & 0x3f);
      }
      local_30 = (byte *)(param_1 + (ulong)((uint)uVar8 & (uint)uVar1) * 0x10);
      uVar6 = (uint)*local_30;
      if (0x10 < *local_30) {
        do {
          if (uVar6 == 99) {
            return 1;
          }
          uVar8 = uVar8 >> (local_30[1] & 0x3f);
          for (uVar5 = uVar5 - local_30[1]; uVar5 < uVar6 - 0x10; uVar5 = uVar5 + 8) {
            if (DAT_001dd86c < DAT_001dd868) {
              bVar3 = (&DAT_0011b000)[DAT_001dd86c];
              DAT_001dd86c = DAT_001dd86c + 1;
            }
            else {
              DAT_001dd870 = local_3c;
              bVar3 = FUN_0010f0bc(0);
            }
            uVar8 = uVar8 | (ulong)bVar3 << ((byte)uVar5 & 0x3f);
          }
          local_30 = (byte *)((ulong)((uint)*(ushort *)(&DAT_001192e0 + (ulong)(uVar6 - 0x10) * 2) &
                                     (uint)uVar8) * 0x10 + *(long *)(local_30 + 8));
          uVar6 = (uint)*local_30;
        } while (0x10 < uVar6);
      }
      uVar8 = uVar8 >> (local_30[1] & 0x3f);
      uVar5 = uVar5 - local_30[1];
      if (uVar6 != 0x10) break;
      uVar6 = local_3c + 1;
      (&DAT_001ad000)[local_3c] = (char)*(undefined2 *)(local_30 + 8);
      local_3c = uVar6;
      if (uVar6 == 0x8000) {
        DAT_001dd870 = uVar6;
        FUN_0010f2cc();
        local_3c = 0;
      }
    }
    if (uVar6 == 0xf) break;
    for (; uVar5 < uVar6; uVar5 = uVar5 + 8) {
      if (DAT_001dd86c < DAT_001dd868) {
        bVar3 = (&DAT_0011b000)[DAT_001dd86c];
        DAT_001dd86c = DAT_001dd86c + 1;
      }
      else {
        DAT_001dd870 = local_3c;
        bVar3 = FUN_0010f0bc(0);
      }
      uVar8 = uVar8 | (ulong)bVar3 << ((byte)uVar5 & 0x3f);
    }
    local_44 = ((uint)*(ushort *)(&DAT_001192e0 + (ulong)uVar6 * 2) & (uint)uVar8) +
               (uint)*(ushort *)(local_30 + 8);
    uVar8 = uVar8 >> ((byte)uVar6 & 0x3f);
    for (uVar5 = uVar5 - uVar6; uVar5 < param_4; uVar5 = uVar5 + 8) {
      if (DAT_001dd86c < DAT_001dd868) {
        bVar3 = (&DAT_0011b000)[DAT_001dd86c];
        DAT_001dd86c = DAT_001dd86c + 1;
      }
      else {
        DAT_001dd870 = local_3c;
        bVar3 = FUN_0010f0bc(0);
      }
      uVar8 = uVar8 | (ulong)bVar3 << ((byte)uVar5 & 0x3f);
    }
    local_30 = (byte *)(param_2 + (ulong)((uint)uVar8 & (uint)uVar2) * 0x10);
    uVar6 = (uint)*local_30;
    if (0x10 < *local_30) {
      do {
        if (uVar6 == 99) {
          return 1;
        }
        uVar8 = uVar8 >> (local_30[1] & 0x3f);
        for (uVar5 = uVar5 - local_30[1]; uVar5 < uVar6 - 0x10; uVar5 = uVar5 + 8) {
          if (DAT_001dd86c < DAT_001dd868) {
            bVar3 = (&DAT_0011b000)[DAT_001dd86c];
            DAT_001dd86c = DAT_001dd86c + 1;
          }
          else {
            DAT_001dd870 = local_3c;
            bVar3 = FUN_0010f0bc(0);
          }
          uVar8 = uVar8 | (ulong)bVar3 << ((byte)uVar5 & 0x3f);
        }
        local_30 = (byte *)((ulong)((uint)*(ushort *)(&DAT_001192e0 + (ulong)(uVar6 - 0x10) * 2) &
                                   (uint)uVar8) * 0x10 + *(long *)(local_30 + 8));
        uVar6 = (uint)*local_30;
      } while (0x10 < uVar6);
    }
    uVar8 = uVar8 >> (local_30[1] & 0x3f);
    for (uVar5 = uVar5 - local_30[1]; uVar5 < uVar6; uVar5 = uVar5 + 8) {
      if (DAT_001dd86c < DAT_001dd868) {
        bVar3 = (&DAT_0011b000)[DAT_001dd86c];
        DAT_001dd86c = DAT_001dd86c + 1;
      }
      else {
        DAT_001dd870 = local_3c;
        bVar3 = FUN_0010f0bc(0);
      }
      uVar8 = uVar8 | (ulong)bVar3 << ((byte)uVar5 & 0x3f);
    }
    local_40 = (local_3c - *(ushort *)(local_30 + 8)) -
               ((uint)uVar8 & (uint)*(ushort *)(&DAT_001192e0 + (ulong)uVar6 * 2));
    uVar8 = uVar8 >> ((byte)uVar6 & 0x3f);
    uVar5 = uVar5 - uVar6;
    do {
      local_40 = local_40 & 0x7fff;
      uVar6 = local_3c;
      if (local_3c < local_40) {
        uVar6 = local_40;
      }
      uVar6 = 0x8000 - uVar6;
      if (local_44 < uVar6) {
        uVar6 = local_44;
      }
      __n = (ulong)uVar6;
      local_44 = local_44 - uVar6;
      if (local_40 < local_3c) {
        uVar4 = local_3c - local_40;
      }
      else {
        uVar4 = local_40 - local_3c;
      }
      if (uVar4 < uVar6) {
        do {
          uVar6 = local_40 + 1;
          uVar4 = local_3c + 1;
          (&DAT_001ad000)[local_3c] = (&DAT_001ad000)[local_40];
          uVar7 = (int)__n - 1;
          __n = (ulong)uVar7;
          local_40 = uVar6;
          local_3c = uVar4;
        } while (uVar7 != 0);
      }
      else {
        memcpy(&DAT_001ad000 + local_3c,&DAT_001ad000 + local_40,__n);
        local_40 = local_40 + uVar6;
        local_3c = local_3c + uVar6;
      }
      if (local_3c == 0x8000) {
        DAT_001dd870 = local_3c;
        FUN_0010f2cc();
        local_3c = 0;
      }
    } while (local_44 != 0);
  }
  DAT_001de220 = uVar5;
  DAT_001de218 = uVar8;
  DAT_001dd870 = local_3c;
  return 0;
}




// Function: inflate_stored @ 0x9d74

undefined8 inflate_stored(void)

{
  uint uVar1;
  byte bVar2;
  undefined8 uVar3;
  uint uVar4;
  ulong uVar5;
  uint local_20;
  uint local_1c;
  
  uVar1 = DAT_001dd870;
  local_1c = DAT_001dd870;
  uVar5 = DAT_001de218 >> (sbyte)(DAT_001de220 & 7);
  for (uVar4 = DAT_001de220 - (DAT_001de220 & 7); uVar4 < 0x10; uVar4 = uVar4 + 8) {
    if (DAT_001dd86c < DAT_001dd868) {
      bVar2 = (&DAT_0011b000)[DAT_001dd86c];
      DAT_001dd86c = DAT_001dd86c + 1;
    }
    else {
      DAT_001dd870 = uVar1;
      bVar2 = FUN_0010f0bc(0);
    }
    uVar5 = uVar5 | (ulong)bVar2 << ((byte)uVar4 & 0x3f);
  }
  local_20 = (uint)uVar5 & 0xffff;
  uVar5 = uVar5 >> 0x10;
  for (uVar4 = uVar4 - 0x10; uVar4 < 0x10; uVar4 = uVar4 + 8) {
    if (DAT_001dd86c < DAT_001dd868) {
      bVar2 = (&DAT_0011b000)[DAT_001dd86c];
      DAT_001dd86c = DAT_001dd86c + 1;
    }
    else {
      DAT_001dd870 = uVar1;
      bVar2 = FUN_0010f0bc(0);
    }
    uVar5 = uVar5 | (ulong)bVar2 << ((byte)uVar4 & 0x3f);
  }
  if (local_20 == (~(uint)uVar5 & 0xffff)) {
    uVar5 = uVar5 >> 0x10;
    uVar4 = uVar4 - 0x10;
    while (local_20 != 0) {
      for (; uVar4 < 8; uVar4 = uVar4 + 8) {
        if (DAT_001dd86c < DAT_001dd868) {
          bVar2 = (&DAT_0011b000)[DAT_001dd86c];
          DAT_001dd86c = DAT_001dd86c + 1;
        }
        else {
          DAT_001dd870 = local_1c;
          bVar2 = FUN_0010f0bc(0);
        }
        uVar5 = uVar5 | (ulong)bVar2 << ((byte)uVar4 & 0x3f);
      }
      DAT_001dd870 = local_1c + 1;
      (&DAT_001ad000)[local_1c] = (char)uVar5;
      local_1c = DAT_001dd870;
      if (DAT_001dd870 == 0x8000) {
        FUN_0010f2cc();
        local_1c = 0;
      }
      uVar5 = uVar5 >> 8;
      uVar4 = uVar4 - 8;
      local_20 = local_20 - 1;
    }
    DAT_001dd870 = local_1c;
    uVar3 = 0;
    DAT_001de218 = uVar5;
    DAT_001de220 = uVar4;
  }
  else {
    uVar3 = 1;
  }
  return uVar3;
}




// Function: inflate_fixed @ 0x9f58

int inflate_fixed(void)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined4 local_4b4;
  undefined4 local_4b0;
  int local_4ac;
  undefined8 local_4a8;
  undefined8 local_4a0;
  undefined4 local_498 [144];
  undefined4 auStack_258 [112];
  undefined4 auStack_98 [24];
  undefined4 auStack_38 [10];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  for (local_4ac = 0; local_4ac < 0x90; local_4ac = local_4ac + 1) {
    local_498[local_4ac] = 8;
  }
  for (; local_4ac < 0x100; local_4ac = local_4ac + 1) {
    local_498[local_4ac] = 9;
  }
  for (; local_4ac < 0x118; local_4ac = local_4ac + 1) {
    local_498[local_4ac] = 7;
  }
  for (; local_4ac < 0x120; local_4ac = local_4ac + 1) {
    local_498[local_4ac] = 8;
  }
  local_4b4 = 7;
  local_4ac = FUN_00109005(local_498,0x120,0x101,&DAT_001191e0,&DAT_00119220,&local_4a8,&local_4b4);
  iVar1 = local_4ac;
  if (local_4ac == 0) {
    for (local_4ac = 0; local_4ac < 0x1e; local_4ac = local_4ac + 1) {
      local_498[local_4ac] = 5;
    }
    local_4b0 = 5;
    local_4ac = FUN_00109005(local_498,0x1e,0,&DAT_00119260,&DAT_001192a0,&local_4a0,&local_4b0);
    if (local_4ac < 2) {
      iVar1 = FUN_0010979a(local_4a8,local_4a0,local_4b4,local_4b0);
      if (iVar1 == 0) {
        FUN_0010975b(local_4a8);
        FUN_0010975b(local_4a0);
        iVar1 = 0;
      }
      else {
        iVar1 = 1;
      }
    }
    else {
      FUN_0010975b(local_4a8);
      iVar1 = local_4ac;
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return iVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: inflate_dynamic @ 0xa191

uint inflate_dynamic(void)

{
  byte bVar1;
  int iVar2;
  uint uVar3;
  ulong uVar4;
  long in_FS_OFFSET;
  uint local_558;
  undefined4 local_554;
  uint local_550;
  uint local_54c;
  uint local_548;
  undefined4 local_544;
  uint local_540;
  uint local_53c;
  uint local_538;
  uint local_534;
  uint local_530;
  uint local_52c;
  long local_528;
  char *local_520;
  uint local_518 [318];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_544 = DAT_001dd870;
  uVar4 = DAT_001de218;
  for (uVar3 = DAT_001de220; uVar3 < 5; uVar3 = uVar3 + 8) {
    if (DAT_001dd86c < DAT_001dd868) {
      bVar1 = (&DAT_0011b000)[DAT_001dd86c];
      DAT_001dd86c = DAT_001dd86c + 1;
    }
    else {
      DAT_001dd870 = local_544;
      bVar1 = FUN_0010f0bc(0);
    }
    uVar4 = uVar4 | (ulong)bVar1 << ((byte)uVar3 & 0x3f);
  }
  local_540 = ((uint)uVar4 & 0x1f) + 0x101;
  uVar4 = uVar4 >> 5;
  for (uVar3 = uVar3 - 5; uVar3 < 5; uVar3 = uVar3 + 8) {
    if (DAT_001dd86c < DAT_001dd868) {
      bVar1 = (&DAT_0011b000)[DAT_001dd86c];
      DAT_001dd86c = DAT_001dd86c + 1;
    }
    else {
      DAT_001dd870 = local_544;
      bVar1 = FUN_0010f0bc(0);
    }
    uVar4 = uVar4 | (ulong)bVar1 << ((byte)uVar3 & 0x3f);
  }
  local_53c = ((uint)uVar4 & 0x1f) + 1;
  uVar4 = uVar4 >> 5;
  for (uVar3 = uVar3 - 5; uVar3 < 4; uVar3 = uVar3 + 8) {
    if (DAT_001dd86c < DAT_001dd868) {
      bVar1 = (&DAT_0011b000)[DAT_001dd86c];
      DAT_001dd86c = DAT_001dd86c + 1;
    }
    else {
      DAT_001dd870 = local_544;
      bVar1 = FUN_0010f0bc(0);
    }
    uVar4 = uVar4 | (ulong)bVar1 << ((byte)uVar3 & 0x3f);
  }
  local_538 = ((uint)uVar4 & 0xf) + 4;
  uVar4 = uVar4 >> 4;
  uVar3 = uVar3 - 4;
  if ((local_540 < 0x11f) && (local_53c < 0x1f)) {
    for (local_54c = 0; local_54c < local_538; local_54c = local_54c + 1) {
      for (; uVar3 < 3; uVar3 = uVar3 + 8) {
        if (DAT_001dd86c < DAT_001dd868) {
          bVar1 = (&DAT_0011b000)[DAT_001dd86c];
          DAT_001dd86c = DAT_001dd86c + 1;
        }
        else {
          DAT_001dd870 = local_544;
          bVar1 = FUN_0010f0bc(0);
        }
        uVar4 = uVar4 | (ulong)bVar1 << ((byte)uVar3 & 0x3f);
      }
      local_518[*(uint *)(&DAT_00119180 + (ulong)local_54c * 4)] = (uint)uVar4 & 7;
      uVar4 = uVar4 >> 3;
      uVar3 = uVar3 - 3;
    }
    for (; local_54c < 0x13; local_54c = local_54c + 1) {
      local_518[*(uint *)(&DAT_00119180 + (ulong)local_54c * 4)] = 0;
    }
    local_558 = 7;
    local_550 = FUN_00109005(local_518,0x13,0x13,0,0,&local_528,&local_558);
    if (local_550 == 0) {
      if (local_528 == 0) {
        uVar3 = 2;
      }
      else {
        local_534 = local_53c + local_540;
        local_530 = (uint)*(ushort *)(&DAT_001192e0 + (long)(int)local_558 * 2);
        local_548 = 0;
        local_550 = 0;
        while (local_550 < local_534) {
          for (; uVar3 < local_558; uVar3 = uVar3 + 8) {
            if (DAT_001dd86c < DAT_001dd868) {
              bVar1 = (&DAT_0011b000)[DAT_001dd86c];
              DAT_001dd86c = DAT_001dd86c + 1;
            }
            else {
              DAT_001dd870 = local_544;
              bVar1 = FUN_0010f0bc(0);
            }
            uVar4 = uVar4 | (ulong)bVar1 << ((byte)uVar3 & 0x3f);
          }
          local_520 = (char *)(local_528 + (ulong)((uint)uVar4 & local_530) * 0x10);
          local_54c = (uint)(byte)local_520[1];
          uVar4 = uVar4 >> (local_520[1] & 0x3fU);
          uVar3 = uVar3 - local_54c;
          if (*local_520 == 'c') {
            FUN_0010975b(local_528);
            uVar3 = 2;
            goto LAB_0010aa3b;
          }
          local_54c = (uint)*(ushort *)(local_520 + 8);
          if (local_54c < 0x10) {
            local_518[(int)local_550] = local_54c;
            local_550 = local_550 + 1;
            local_548 = local_54c;
          }
          else if (local_54c == 0x10) {
            local_54c = 0x10;
            for (; uVar3 < 2; uVar3 = uVar3 + 8) {
              if (DAT_001dd86c < DAT_001dd868) {
                bVar1 = (&DAT_0011b000)[DAT_001dd86c];
                DAT_001dd86c = DAT_001dd86c + 1;
              }
              else {
                DAT_001dd870 = local_544;
                bVar1 = FUN_0010f0bc(0);
              }
              uVar4 = uVar4 | (ulong)bVar1 << ((byte)uVar3 & 0x3f);
            }
            local_54c = ((uint)uVar4 & 3) + 3;
            uVar4 = uVar4 >> 2;
            uVar3 = uVar3 - 2;
            if (local_534 < local_54c + local_550) {
              uVar3 = 1;
              goto LAB_0010aa3b;
            }
            while (local_54c != 0) {
              local_518[(int)local_550] = local_548;
              local_550 = local_550 + 1;
              local_54c = local_54c - 1;
            }
            local_54c = 0xffffffff;
          }
          else if (local_54c == 0x11) {
            local_54c = 0x11;
            for (; uVar3 < 3; uVar3 = uVar3 + 8) {
              if (DAT_001dd86c < DAT_001dd868) {
                bVar1 = (&DAT_0011b000)[DAT_001dd86c];
                DAT_001dd86c = DAT_001dd86c + 1;
              }
              else {
                DAT_001dd870 = local_544;
                bVar1 = FUN_0010f0bc(0);
              }
              uVar4 = uVar4 | (ulong)bVar1 << ((byte)uVar3 & 0x3f);
            }
            local_54c = ((uint)uVar4 & 7) + 3;
            uVar4 = uVar4 >> 3;
            uVar3 = uVar3 - 3;
            if (local_534 < local_54c + local_550) {
              uVar3 = 1;
              goto LAB_0010aa3b;
            }
            while (local_54c != 0) {
              local_518[(int)local_550] = 0;
              local_550 = local_550 + 1;
              local_54c = local_54c - 1;
            }
            local_548 = 0;
            local_54c = 0xffffffff;
          }
          else {
            for (; uVar3 < 7; uVar3 = uVar3 + 8) {
              if (DAT_001dd86c < DAT_001dd868) {
                bVar1 = (&DAT_0011b000)[DAT_001dd86c];
                DAT_001dd86c = DAT_001dd86c + 1;
              }
              else {
                DAT_001dd870 = local_544;
                bVar1 = FUN_0010f0bc(0);
              }
              uVar4 = uVar4 | (ulong)bVar1 << ((byte)uVar3 & 0x3f);
            }
            local_54c = ((uint)uVar4 & 0x7f) + 0xb;
            uVar4 = uVar4 >> 7;
            uVar3 = uVar3 - 7;
            if (local_534 < local_54c + local_550) {
              uVar3 = 1;
              goto LAB_0010aa3b;
            }
            while (local_54c != 0) {
              local_518[(int)local_550] = 0;
              local_550 = local_550 + 1;
              local_54c = local_54c - 1;
            }
            local_548 = 0;
            local_54c = 0xffffffff;
          }
        }
        FUN_0010975b(local_528);
        local_558 = DAT_00119304;
        DAT_001de218 = uVar4;
        DAT_001de220 = uVar3;
        local_550 = FUN_00109005(local_518,local_540,0x101,&DAT_001191e0,&DAT_00119220,&local_528,
                                 &local_558);
        if (local_550 == 0) {
          local_554 = DAT_00119308;
          local_550 = FUN_00109005(local_518 + local_540,local_53c,0,&DAT_00119260,&DAT_001192a0,
                                   &local_520,&local_554);
          if (local_550 == 0) {
            iVar2 = FUN_0010979a(local_528,local_520,local_558,local_554);
            local_52c = (uint)(iVar2 != 0);
            FUN_0010975b(local_528);
            FUN_0010975b(local_520);
            uVar3 = local_52c;
          }
          else {
            if (local_550 == 1) {
              FUN_0010975b(local_520);
            }
            FUN_0010975b(local_528);
            uVar3 = local_550;
          }
        }
        else {
          uVar3 = local_550;
          if (local_550 == 1) {
            FUN_0010975b(local_528);
            uVar3 = local_550;
          }
        }
      }
    }
    else {
      uVar3 = local_550;
      if (local_550 == 1) {
        FUN_0010975b(local_528);
        uVar3 = local_550;
      }
    }
  }
  else {
    uVar3 = 1;
  }
LAB_0010aa3b:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: inflate_block @ 0xaa58

undefined8 inflate_block(uint *param_1)

{
  undefined4 uVar1;
  byte bVar2;
  uint uVar3;
  undefined8 uVar4;
  int iVar5;
  uint uVar6;
  ulong uVar7;
  
  uVar1 = DAT_001dd870;
  uVar7 = DAT_001de218;
  iVar5 = DAT_001de220;
  while (iVar5 == 0) {
    if (DAT_001dd86c < DAT_001dd868) {
      bVar2 = (&DAT_0011b000)[DAT_001dd86c];
      DAT_001dd86c = DAT_001dd86c + 1;
    }
    else {
      DAT_001dd870 = uVar1;
      bVar2 = FUN_0010f0bc(0);
    }
    uVar7 = uVar7 | bVar2;
    iVar5 = 8;
  }
  *param_1 = (uint)uVar7 & 1;
  uVar7 = uVar7 >> 1;
  for (uVar6 = iVar5 - 1; uVar6 < 2; uVar6 = uVar6 + 8) {
    if (DAT_001dd86c < DAT_001dd868) {
      bVar2 = (&DAT_0011b000)[DAT_001dd86c];
      DAT_001dd86c = DAT_001dd86c + 1;
    }
    else {
      DAT_001dd870 = uVar1;
      bVar2 = FUN_0010f0bc(0);
    }
    uVar7 = uVar7 | (ulong)bVar2 << ((byte)uVar6 & 0x3f);
  }
  uVar3 = (uint)uVar7 & 3;
  DAT_001de218 = uVar7 >> 2;
  DAT_001de220 = uVar6 - 2;
  if (uVar3 == 2) {
    uVar4 = FUN_0010a191();
  }
  else if ((uVar7 & 3) == 0) {
    uVar4 = FUN_00109d74();
  }
  else if (uVar3 == 1) {
    uVar4 = FUN_00109f58();
  }
  else {
    uVar4 = 2;
  }
  return uVar4;
}




// Function: inflate @ 0xab98

int inflate(void)

{
  int iVar1;
  long in_FS_OFFSET;
  int local_1c;
  uint local_18;
  int local_14;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_001dd870 = 0;
  DAT_001de220 = 0;
  DAT_001de218 = 0;
  local_18 = 0;
  iVar1 = local_14;
  do {
    local_14 = iVar1;
    DAT_001de224 = 0;
    local_14 = FUN_0010aa58(&local_1c);
    iVar1 = local_14;
    if (local_14 != 0) goto LAB_0010ac59;
    if (local_18 < DAT_001de224) {
      local_18 = DAT_001de224;
    }
    iVar1 = 0;
  } while (local_1c == 0);
  for (; 7 < DAT_001de220; DAT_001de220 = DAT_001de220 - 8) {
    DAT_001dd86c = DAT_001dd86c + -1;
  }
  FUN_0010f2cc();
  iVar1 = 0;
LAB_0010ac59:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return iVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: ct_init @ 0xac6f

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void ct_init(undefined8 param_1,undefined8 param_2)

{
  undefined2 uVar1;
  int local_1c;
  int local_18;
  int local_14;
  int local_10;
  int local_c;
  
  DAT_001e1168 = 0;
  DAT_001e1160 = 0;
  DAT_001e1170 = param_1;
  _DAT_001e1178 = param_2;
  if (DAT_001df0c2 == 0) {
    local_14 = 0;
    for (local_10 = 0; local_10 < 0x1c; local_10 = local_10 + 1) {
      *(int *)(&DAT_001e0040 + (long)local_10 * 4) = local_14;
      for (local_1c = 0;
          local_1c < 1 << ((byte)*(undefined4 *)(&DAT_00119320 + (long)local_10 * 4) & 0x1f);
          local_1c = local_1c + 1) {
        (&DAT_001dfd40)[local_14] = (char)local_10;
        local_14 = local_14 + 1;
      }
    }
    (&DAT_001dfd40)[local_14 + -1] = (char)local_10;
    local_c = 0;
    for (local_10 = 0; local_10 < 0x10; local_10 = local_10 + 1) {
      *(int *)(&DAT_001e00c0 + (long)local_10 * 4) = local_c;
      for (local_1c = 0;
          local_1c < 1 << ((byte)*(undefined4 *)(&DAT_001193a0 + (long)local_10 * 4) & 0x1f);
          local_1c = local_1c + 1) {
        (&DAT_001dfe40)[local_c] = (char)local_10;
        local_c = local_c + 1;
      }
    }
    local_c = local_c >> 7;
    for (; local_10 < 0x1e; local_10 = local_10 + 1) {
      *(int *)(&DAT_001e00c0 + (long)local_10 * 4) = local_c << 7;
      for (local_1c = 0;
          local_1c < 1 << ((char)*(undefined4 *)(&DAT_001193a0 + (long)local_10 * 4) - 7U & 0x1f);
          local_1c = local_1c + 1) {
        (&DAT_001dfe40)[local_c + 0x100] = (char)local_10;
        local_c = local_c + 1;
      }
    }
    for (local_18 = 0; local_18 < 0x10; local_18 = local_18 + 1) {
      (&DAT_001df1e0)[local_18] = 0;
    }
    local_1c = 0;
    while (local_1c < 0x90) {
      *(undefined2 *)(&DAT_001dec42 + (long)local_1c * 4) = 8;
      DAT_001df1f0 = DAT_001df1f0 + 1;
      local_1c = local_1c + 1;
    }
    while (local_1c < 0x100) {
      *(undefined2 *)(&DAT_001dec42 + (long)local_1c * 4) = 9;
      DAT_001df1f2 = DAT_001df1f2 + 1;
      local_1c = local_1c + 1;
    }
    while (local_1c < 0x118) {
      *(undefined2 *)(&DAT_001dec42 + (long)local_1c * 4) = 7;
      DAT_001df1ee = DAT_001df1ee + 1;
      local_1c = local_1c + 1;
    }
    while (local_1c < 0x120) {
      *(undefined2 *)(&DAT_001dec42 + (long)local_1c * 4) = 8;
      DAT_001df1f0 = DAT_001df1f0 + 1;
      local_1c = local_1c + 1;
    }
    FUN_0010b76a(&DAT_001dec40,0x11f);
    for (local_1c = 0; local_1c < 0x1e; local_1c = local_1c + 1) {
      (&DAT_001df0c2)[(long)local_1c * 2] = 5;
      uVar1 = FUN_00103bed(local_1c,5);
      *(undefined2 *)(&DAT_001df0c0 + (long)local_1c * 4) = uVar1;
    }
    FUN_0010aff6();
  }
  return;
}




// Function: init_block @ 0xaff6

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void init_block(void)

{
  int local_c;
  
  for (local_c = 0; local_c < 0x11e; local_c = local_c + 1) {
    *(undefined2 *)(&DAT_001de240 + (long)local_c * 4) = 0;
  }
  for (local_c = 0; local_c < 0x1e; local_c = local_c + 1) {
    *(undefined2 *)(&DAT_001deb40 + (long)local_c * 4) = 0;
  }
  for (local_c = 0; local_c < 0x13; local_c = local_c + 1) {
    *(undefined2 *)(&DAT_001df140 + (long)local_c * 4) = 0;
  }
  _DAT_001de640 = 1;
  DAT_001e1158 = 0;
  DAT_001e1150 = 0;
  DAT_001e1148 = 0;
  DAT_001e1144 = 0;
  DAT_001e1140 = 0;
  DAT_001e114c = 0;
  DAT_001e114d = 1;
  return;
}




// Function: pqdownheap @ 0xb0dd

void pqdownheap(long param_1,int param_2)

{
  int iVar1;
  int local_24;
  int local_10;
  
  iVar1 = *(int *)(&DAT_001df200 + (long)param_2 * 4);
  local_24 = param_2;
  for (local_10 = param_2 * 2; local_10 <= DAT_001dfaf4; local_10 = local_10 << 1) {
    if ((local_10 < DAT_001dfaf4) &&
       ((*(ushort *)(param_1 + (long)*(int *)(&DAT_001df200 + (long)(local_10 + 1) * 4) * 4) <
         *(ushort *)(param_1 + (long)*(int *)(&DAT_001df200 + (long)local_10 * 4) * 4) ||
        ((*(short *)(param_1 + (long)*(int *)(&DAT_001df200 + (long)(local_10 + 1) * 4) * 4) ==
          *(short *)(param_1 + (long)*(int *)(&DAT_001df200 + (long)local_10 * 4) * 4) &&
         ((byte)(&DAT_001dfb00)[*(int *)(&DAT_001df200 + (long)(local_10 + 1) * 4)] <=
          (byte)(&DAT_001dfb00)[*(int *)(&DAT_001df200 + (long)local_10 * 4)])))))) {
      local_10 = local_10 + 1;
    }
    if ((*(ushort *)(param_1 + (long)iVar1 * 4) <
         *(ushort *)(param_1 + (long)*(int *)(&DAT_001df200 + (long)local_10 * 4) * 4)) ||
       ((*(short *)(param_1 + (long)iVar1 * 4) ==
         *(short *)(param_1 + (long)*(int *)(&DAT_001df200 + (long)local_10 * 4) * 4) &&
        ((byte)(&DAT_001dfb00)[iVar1] <=
         (byte)(&DAT_001dfb00)[*(int *)(&DAT_001df200 + (long)local_10 * 4)])))) break;
    *(undefined4 *)(&DAT_001df200 + (long)local_24 * 4) =
         *(undefined4 *)(&DAT_001df200 + (long)local_10 * 4);
    local_24 = local_10;
  }
  *(int *)(&DAT_001df200 + (long)local_24 * 4) = iVar1;
  return;
}




// Function: gen_bitlen @ 0xb363

void gen_bitlen(long *param_1)

{
  ushort uVar1;
  int iVar2;
  uint uVar3;
  int iVar4;
  long lVar5;
  long lVar6;
  long lVar7;
  uint uVar8;
  long lVar9;
  int local_44;
  uint local_40;
  uint local_3c;
  int local_38;
  int local_34;
  
  lVar5 = *param_1;
  lVar6 = param_1[2];
  lVar9 = param_1[3];
  iVar2 = *(int *)((long)param_1 + 0x24);
  uVar3 = *(uint *)(param_1 + 4);
  lVar7 = param_1[1];
  local_34 = 0;
  for (local_3c = 0; (int)local_3c < 0x10; local_3c = local_3c + 1) {
    (&DAT_001df1e0)[(int)local_3c] = 0;
  }
  *(undefined2 *)(lVar5 + (long)*(int *)(&DAT_001df200 + (long)DAT_001dfaf8 * 4) * 4 + 2) = 0;
  local_44 = DAT_001dfaf8;
  while (local_44 = local_44 + 1, local_44 < 0x23d) {
    iVar4 = *(int *)(&DAT_001df200 + (long)local_44 * 4);
    local_3c = *(ushort *)(lVar5 + (ulong)*(ushort *)(lVar5 + (long)iVar4 * 4 + 2) * 4 + 2) + 1;
    if ((int)uVar3 < (int)local_3c) {
      local_34 = local_34 + 1;
      local_3c = uVar3;
    }
    *(short *)(lVar5 + (long)iVar4 * 4 + 2) = (short)local_3c;
    if (iVar4 <= iVar2) {
      (&DAT_001df1e0)[(int)local_3c] = (&DAT_001df1e0)[(int)local_3c] + 1;
      local_38 = 0;
      if ((int)lVar9 <= iVar4) {
        local_38 = *(int *)(lVar6 + (long)(iVar4 - (int)lVar9) * 4);
      }
      uVar1 = *(ushort *)(lVar5 + (long)iVar4 * 4);
      DAT_001e1150 = DAT_001e1150 + (ulong)uVar1 * (long)(int)(local_38 + local_3c);
      if (lVar7 != 0) {
        DAT_001e1158 = DAT_001e1158 +
                       (ulong)uVar1 *
                       (long)(int)(local_38 + (uint)*(ushort *)(lVar7 + (long)iVar4 * 4 + 2));
      }
    }
  }
  local_3c = uVar3;
  if (local_34 != 0) {
    do {
      do {
        uVar8 = local_3c;
        local_3c = uVar8 - 1;
      } while ((&DAT_001df1e0)[(int)local_3c] == 0);
      (&DAT_001df1e0)[(int)local_3c] = (&DAT_001df1e0)[(int)local_3c] + -1;
      (&DAT_001df1e0)[(int)uVar8] = (&DAT_001df1e0)[(int)uVar8] + 2;
      (&DAT_001df1e0)[(int)uVar3] = (&DAT_001df1e0)[(int)uVar3] + -1;
      local_34 = local_34 + -2;
      local_3c = uVar3;
    } while (0 < local_34);
    for (; local_3c != 0; local_3c = local_3c - 1) {
      local_40 = (uint)(ushort)(&DAT_001df1e0)[(int)local_3c];
      while (local_40 != 0) {
        local_44 = local_44 + -1;
        iVar4 = *(int *)(&DAT_001df200 + (long)local_44 * 4);
        if (iVar4 <= iVar2) {
          if (*(ushort *)(lVar5 + (long)iVar4 * 4 + 2) != local_3c) {
            DAT_001e1150 = DAT_001e1150 +
                           (ulong)*(ushort *)(lVar5 + (long)iVar4 * 4) *
                           ((long)(int)local_3c - (ulong)*(ushort *)(lVar5 + (long)iVar4 * 4 + 2));
            *(short *)(lVar5 + (long)iVar4 * 4 + 2) = (short)local_3c;
          }
          local_40 = local_40 - 1;
        }
      }
    }
  }
  return;
}




// Function: gen_codes @ 0xb76a

void gen_codes(long param_1,int param_2)

{
  short sVar1;
  undefined2 uVar2;
  uint uVar3;
  long in_FS_OFFSET;
  short local_46;
  int local_44;
  int local_40;
  short asStack_38 [20];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_46 = 0;
  for (local_44 = 1; local_44 < 0x10; local_44 = local_44 + 1) {
    local_46 = (local_46 + (&DAT_001df1e0)[local_44 + -1]) * 2;
    asStack_38[local_44] = local_46;
  }
  for (local_40 = 0; local_40 <= param_2; local_40 = local_40 + 1) {
    uVar3 = (uint)*(ushort *)(param_1 + (long)local_40 * 4 + 2);
    if (uVar3 != 0) {
      sVar1 = asStack_38[(int)uVar3];
      asStack_38[(int)uVar3] = sVar1 + 1;
      uVar2 = FUN_00103bed(sVar1,uVar3);
      *(undefined2 *)(param_1 + (long)local_40 * 4) = uVar2;
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: build_tree @ 0xb868

void build_tree(long *param_1)

{
  char cVar1;
  long lVar2;
  int iVar3;
  long lVar4;
  int iVar5;
  int local_30;
  int local_2c;
  int local_28;
  
  lVar2 = *param_1;
  lVar4 = param_1[1];
  local_28 = *(int *)((long)param_1 + 0x1c);
  local_2c = -1;
  DAT_001dfaf4 = 0;
  DAT_001dfaf8 = 0x23d;
  for (local_30 = 0; local_30 < local_28; local_30 = local_30 + 1) {
    if (*(short *)(lVar2 + (long)local_30 * 4) == 0) {
      *(undefined2 *)(lVar2 + (long)local_30 * 4 + 2) = 0;
    }
    else {
      local_2c = local_30;
      DAT_001dfaf4 = DAT_001dfaf4 + 1;
      *(int *)(&DAT_001df200 + (long)DAT_001dfaf4 * 4) = local_30;
      (&DAT_001dfb00)[local_30] = 0;
    }
  }
  while (DAT_001dfaf4 < 2) {
    if (local_2c < 2) {
      iVar5 = local_2c + 1;
      local_2c = iVar5;
    }
    else {
      iVar5 = 0;
    }
    iVar3 = DAT_001dfaf4 + 1;
    DAT_001dfaf4 = iVar3;
    *(int *)(&DAT_001df200 + (long)iVar3 * 4) = iVar5;
    iVar5 = *(int *)(&DAT_001df200 + (long)iVar3 * 4);
    *(undefined2 *)(lVar2 + (long)iVar5 * 4) = 1;
    (&DAT_001dfb00)[iVar5] = 0;
    DAT_001e1150 = DAT_001e1150 + -1;
    if (lVar4 != 0) {
      DAT_001e1158 = DAT_001e1158 - (ulong)*(ushort *)(lVar4 + (long)iVar5 * 4 + 2);
    }
  }
  *(int *)((long)param_1 + 0x24) = local_2c;
  for (local_30 = DAT_001dfaf4 / 2; 0 < local_30; local_30 = local_30 + -1) {
    FUN_0010b0dd(lVar2,local_30);
  }
  do {
    iVar5 = DAT_001df204;
    DAT_001df204 = *(int *)(&DAT_001df200 + (long)DAT_001dfaf4 * 4);
    DAT_001dfaf4 = DAT_001dfaf4 + -1;
    FUN_0010b0dd(lVar2,1);
    DAT_001dfaf8 = DAT_001dfaf8 + -1;
    *(int *)(&DAT_001df200 + (long)DAT_001dfaf8 * 4) = iVar5;
    DAT_001dfaf8 = DAT_001dfaf8 + -1;
    *(int *)(&DAT_001df200 + (long)DAT_001dfaf8 * 4) = DAT_001df204;
    *(short *)(lVar2 + (long)local_28 * 4) =
         *(short *)(lVar2 + (long)DAT_001df204 * 4) + *(short *)(lVar2 + (long)iVar5 * 4);
    if ((byte)(&DAT_001dfb00)[iVar5] < (byte)(&DAT_001dfb00)[DAT_001df204]) {
      cVar1 = (&DAT_001dfb00)[DAT_001df204];
    }
    else {
      cVar1 = (&DAT_001dfb00)[iVar5];
    }
    (&DAT_001dfb00)[local_28] = cVar1 + '\x01';
    lVar4 = lVar2 + (long)DAT_001df204 * 4;
    *(short *)(lVar4 + 2) = (short)local_28;
    *(undefined2 *)(lVar2 + (long)iVar5 * 4 + 2) = *(undefined2 *)(lVar4 + 2);
    DAT_001df204 = local_28;
    FUN_0010b0dd(lVar2,1);
    local_28 = local_28 + 1;
  } while (1 < DAT_001dfaf4);
  DAT_001dfaf8 = DAT_001dfaf8 + -1;
  *(int *)(&DAT_001df200 + (long)DAT_001dfaf8 * 4) = DAT_001df204;
  FUN_0010b363(param_1);
  FUN_0010b76a(lVar2,local_2c);
  return;
}




// Function: scan_tree @ 0xbc5c

void scan_tree(long param_1,int param_2)

{
  ushort uVar1;
  uint uVar2;
  int local_24;
  uint local_20;
  uint local_1c;
  int local_18;
  int local_14;
  int local_10;
  
  local_20 = 0xffffffff;
  uVar1 = *(ushort *)(param_1 + 2);
  local_18 = 0;
  local_14 = 7;
  local_10 = 4;
  if (uVar1 == 0) {
    local_14 = 0x8a;
    local_10 = 3;
  }
  *(undefined2 *)(param_1 + ((long)param_2 + 1) * 4 + 2) = 0xffff;
  local_1c = (uint)uVar1;
  for (local_24 = 0; local_24 <= param_2; local_24 = local_24 + 1) {
    uVar2 = (uint)*(ushort *)(param_1 + ((long)local_24 + 1) * 4 + 2);
    local_18 = local_18 + 1;
    if ((local_14 <= local_18) || (local_1c != uVar2)) {
      if (local_18 < local_10) {
        *(short *)(&DAT_001df140 + (long)(int)local_1c * 4) =
             *(short *)(&DAT_001df140 + (long)(int)local_1c * 4) + (short)local_18;
      }
      else if (local_1c == 0) {
        if (local_18 < 0xb) {
          DAT_001df184 = DAT_001df184 + 1;
        }
        else {
          DAT_001df188 = DAT_001df188 + 1;
        }
      }
      else {
        if (local_1c != local_20) {
          *(short *)(&DAT_001df140 + (long)(int)local_1c * 4) =
               *(short *)(&DAT_001df140 + (long)(int)local_1c * 4) + 1;
        }
        DAT_001df180 = DAT_001df180 + 1;
      }
      local_18 = 0;
      local_20 = local_1c;
      if (uVar2 == 0) {
        local_14 = 0x8a;
        local_10 = 3;
      }
      else if (local_1c == uVar2) {
        local_14 = 6;
        local_10 = 3;
      }
      else {
        local_14 = 7;
        local_10 = 4;
      }
    }
    local_1c = uVar2;
  }
  return;
}




// Function: send_tree @ 0xbe31

void send_tree(long param_1,int param_2)

{
  int iVar1;
  uint uVar2;
  int local_24;
  uint local_20;
  uint local_1c;
  int local_18;
  int local_14;
  int local_10;
  
  local_20 = 0xffffffff;
  local_18 = 0;
  local_14 = 7;
  local_10 = 4;
  if (*(ushort *)(param_1 + 2) == 0) {
    local_14 = 0x8a;
    local_10 = 3;
  }
  local_1c = (uint)*(ushort *)(param_1 + 2);
  for (local_24 = 0; local_24 <= param_2; local_24 = local_24 + 1) {
    uVar2 = (uint)*(ushort *)(param_1 + ((long)local_24 + 1) * 4 + 2);
    iVar1 = local_18 + 1;
    if ((local_14 <= iVar1) || (local_1c != uVar2)) {
      if (iVar1 < local_10) {
        do {
          local_18 = iVar1;
          FUN_00103a6c(*(undefined2 *)(&DAT_001df140 + (long)(int)local_1c * 4),
                       *(undefined2 *)(&DAT_001df142 + (long)(int)local_1c * 4));
          iVar1 = local_18 + -1;
        } while (local_18 + -1 != 0);
      }
      else if (local_1c == 0) {
        if (iVar1 < 0xb) {
          FUN_00103a6c(DAT_001df184,DAT_001df186);
          FUN_00103a6c(local_18 + -2,3);
        }
        else {
          FUN_00103a6c(DAT_001df188,DAT_001df18a);
          FUN_00103a6c(local_18 + -10,7);
        }
      }
      else {
        if (local_1c != local_20) {
          FUN_00103a6c(*(undefined2 *)(&DAT_001df140 + (long)(int)local_1c * 4),
                       *(undefined2 *)(&DAT_001df142 + (long)(int)local_1c * 4));
          iVar1 = local_18;
        }
        local_18 = iVar1;
        FUN_00103a6c(DAT_001df180,DAT_001df182);
        FUN_00103a6c(local_18 + -3,2);
      }
      local_18 = 0;
      local_20 = local_1c;
      iVar1 = local_18;
      if (uVar2 == 0) {
        local_14 = 0x8a;
        local_10 = 3;
      }
      else if (local_1c == uVar2) {
        local_14 = 6;
        local_10 = 3;
      }
      else {
        local_14 = 7;
        local_10 = 4;
      }
    }
    local_18 = iVar1;
    local_1c = uVar2;
  }
  return;
}




// Function: build_bl_tree @ 0xc06c

int build_bl_tree(void)

{
  int local_c;
  
  FUN_0010bc5c(&DAT_001de240,DAT_001194c4);
  FUN_0010bc5c(&DAT_001deb40,DAT_00119504);
  FUN_0010b868(&PTR_DAT_00119520);
  local_c = 0x12;
  while ((2 < local_c &&
         (*(short *)(&DAT_001df142 + (long)(int)(uint)(byte)(&DAT_00119470)[local_c] * 4) == 0))) {
    local_c = local_c + -1;
  }
  DAT_001e1150 = DAT_001e1150 + ((local_c + 1) * 3 + 0xe);
  return local_c;
}




// Function: send_all_trees @ 0xc120

void send_all_trees(int param_1,int param_2,int param_3)

{
  int local_c;
  
  FUN_00103a6c(param_1 + -0x101,5);
  FUN_00103a6c(param_2 + -1,5);
  FUN_00103a6c(param_3 + -4,4);
  for (local_c = 0; local_c < param_3; local_c = local_c + 1) {
    FUN_00103a6c(*(undefined2 *)
                  (&DAT_001df142 + (long)(int)(uint)(byte)(&DAT_00119470)[local_c] * 4),3);
  }
  FUN_0010be31(&DAT_001de240,param_1 + -1);
  FUN_0010be31(&DAT_001deb40,param_2 + -1);
  return;
}




// Function: flush_block @ 0xc1ea

long flush_block(long param_1,ulong param_2,int param_3,int param_4)

{
  int iVar1;
  ulong uVar2;
  ulong local_18;
  
  (&DAT_001e0140)[DAT_001e1148] = DAT_001e114c;
  if (*DAT_001e1170 == -1) {
    FUN_0010c97c();
  }
  FUN_0010b868(&PTR_DAT_001194a0);
  FUN_0010b868(&PTR_DAT_001194e0);
  iVar1 = FUN_0010c06c();
  local_18 = DAT_001e1150 + 10U >> 3;
  uVar2 = DAT_001e1158 + 10U >> 3;
  DAT_001e1168 = param_2 + DAT_001e1168;
  if (uVar2 <= local_18) {
    local_18 = uVar2;
  }
  if ((local_18 < param_2 + 4) || (param_1 == 0)) {
    if (uVar2 == local_18) {
      FUN_00103a6c(param_4 + 2,3);
      FUN_0010c6f9(&DAT_001dec40,&DAT_001df0c0);
      DAT_001e1160 = DAT_001e1160 + DAT_001e1158 + 3;
    }
    else {
      FUN_00103a6c(param_4 + 4,3);
      FUN_0010c120(DAT_001194c4 + 1,DAT_00119504 + 1,iVar1 + 1);
      FUN_0010c6f9(&DAT_001de240,&DAT_001deb40);
      DAT_001e1160 = DAT_001e1160 + DAT_001e1150 + 3;
    }
  }
  else {
    FUN_00103a6c(param_4,3);
    DAT_001e1160 = (DAT_001e1160 + 10 & 0xfffffffffffffff8) + (param_2 + 4) * 8;
    FUN_00103d59(param_1,param_2 & 0xffffffff,1);
  }
  FUN_0010aff6();
  if (param_4 == 0) {
    if ((param_3 != 0) && ((DAT_001e1160 & 7) != 0)) {
      FUN_00103a6c(0,3);
      DAT_001e1160 = DAT_001e1160 + 10 & 0xfffffffffffffff8;
      FUN_00103d59(param_1,0,1);
    }
  }
  else {
    FUN_00103c22();
    DAT_001e1160 = DAT_001e1160 + 7;
  }
  return (long)DAT_001e1160 >> 3;
}




// Function: ct_tally @ 0xc45d

undefined8 ct_tally(int param_1,int param_2)

{
  byte bVar1;
  ulong uVar2;
  undefined8 uVar3;
  int local_1c;
  ulong local_18;
  
  uVar2 = (ulong)DAT_001e1140;
  DAT_001e1140 = DAT_001e1140 + 1;
  (&DAT_0011b000)[uVar2] = (char)param_2;
  if (param_1 == 0) {
    *(short *)(&DAT_001de240 + (long)param_2 * 4) =
         *(short *)(&DAT_001de240 + (long)param_2 * 4) + 1;
  }
  else {
    param_1 = param_1 + -1;
    *(short *)(&DAT_001de240 + (long)(int)((byte)(&DAT_001dfd40)[param_2] + 0x101) * 4) =
         *(short *)(&DAT_001de240 + (long)(int)((byte)(&DAT_001dfd40)[param_2] + 0x101) * 4) + 1;
    if (param_1 < 0x100) {
      bVar1 = (&DAT_001dfe40)[param_1];
    }
    else {
      bVar1 = (&DAT_001dfe40)[(param_1 >> 7) + 0x100];
    }
    *(short *)(&DAT_001deb40 + (long)(int)(uint)bVar1 * 4) =
         *(short *)(&DAT_001deb40 + (long)(int)(uint)bVar1 * 4) + 1;
    uVar2 = (ulong)DAT_001e1144;
    DAT_001e1144 = DAT_001e1144 + 1;
    *(short *)(&DAT_0019c800 + uVar2 * 2) = (short)param_1;
    DAT_001e114c = DAT_001e114d | DAT_001e114c;
  }
  DAT_001e114d = DAT_001e114d * '\x02';
  if ((DAT_001e1140 & 7) == 0) {
    uVar2 = (ulong)DAT_001e1148;
    DAT_001e1148 = DAT_001e1148 + 1;
    (&DAT_001e0140)[uVar2] = DAT_001e114c;
    DAT_001e114c = 0;
    DAT_001e114d = '\x01';
  }
  if ((2 < DAT_001190a0) && ((DAT_001e1140 & 0xfff) == 0)) {
    local_18 = (ulong)DAT_001e1140 << 3;
    for (local_1c = 0; local_1c < 0x1e; local_1c = local_1c + 1) {
      local_18 = local_18 +
                 (ulong)*(ushort *)(&DAT_001deb40 + (long)local_1c * 4) *
                 ((long)*(int *)(&DAT_001193a0 + (long)local_1c * 4) + 5);
    }
    if ((DAT_001e1144 < DAT_001e1140 >> 1) &&
       (local_18 >> 3 < (ulong)DAT_0011a074 - DAT_0011a068 >> 1)) {
      return 1;
    }
  }
  if ((DAT_001e1140 == 0x7fff) || (DAT_001e1144 == 0x8000)) {
    uVar3 = 1;
  }
  else {
    uVar3 = 0;
  }
  return uVar3;
}




// Function: compress_block @ 0xc6f9

void compress_block(long param_1,long param_2)

{
  uint uVar1;
  byte bVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  byte local_25;
  uint local_24;
  uint local_20;
  uint local_1c;
  
  local_24 = 0;
  local_20 = 0;
  local_1c = 0;
  local_25 = 0;
  if (DAT_001e1140 != 0) {
    do {
      if ((local_24 & 7) == 0) {
        local_25 = (&DAT_001e0140)[local_1c];
        local_1c = local_1c + 1;
      }
      uVar1 = local_24 + 1;
      uVar3 = (uint)(byte)(&DAT_0011b000)[local_24];
      if ((local_25 & 1) == 0) {
        FUN_00103a6c(*(undefined2 *)(param_1 + (long)(int)uVar3 * 4),
                     *(undefined2 *)(param_1 + (long)(int)uVar3 * 4 + 2));
      }
      else {
        uVar4 = (uint)(byte)(&DAT_001dfd40)[(int)uVar3];
        FUN_00103a6c(*(undefined2 *)(param_1 + (ulong)(uVar4 + 0x101) * 4),
                     *(undefined2 *)(param_1 + (ulong)(uVar4 + 0x101) * 4 + 2));
        if (*(int *)(&DAT_00119320 + (ulong)uVar4 * 4) != 0) {
          FUN_00103a6c(uVar3 - *(int *)(&DAT_001e0040 + (ulong)uVar4 * 4),
                       *(int *)(&DAT_00119320 + (ulong)uVar4 * 4));
        }
        uVar3 = local_20 + 1;
        uVar4 = (uint)*(ushort *)(&DAT_0019c800 + (ulong)local_20 * 2);
        if (uVar4 < 0x100) {
          bVar2 = (&DAT_001dfe40)[uVar4];
        }
        else {
          bVar2 = (&DAT_001dfe40)[(*(ushort *)(&DAT_0019c800 + (ulong)local_20 * 2) >> 7) + 0x100];
        }
        uVar5 = (uint)bVar2;
        FUN_00103a6c(*(undefined2 *)(param_2 + (ulong)uVar5 * 4),
                     *(undefined2 *)(param_2 + (ulong)uVar5 * 4 + 2));
        local_20 = uVar3;
        if (*(int *)(&DAT_001193a0 + (ulong)uVar5 * 4) != 0) {
          FUN_00103a6c(uVar4 - *(int *)(&DAT_001e00c0 + (ulong)uVar5 * 4),
                       *(int *)(&DAT_001193a0 + (ulong)uVar5 * 4));
        }
      }
      local_25 = local_25 >> 1;
      local_24 = uVar1;
    } while (uVar1 < DAT_001e1140);
  }
  FUN_00103a6c(*(undefined2 *)(param_1 + 0x400),*(undefined2 *)(param_1 + 0x402));
  return;
}




// Function: set_file_type @ 0xc97c

void set_file_type(void)

{
  int local_14;
  uint local_10;
  uint local_c;
  
  local_10 = 0;
  local_c = 0;
  for (local_14 = 0; local_14 < 7; local_14 = local_14 + 1) {
    local_c = local_c + *(ushort *)(&DAT_001de240 + (long)local_14 * 4);
  }
  for (; local_14 < 0x80; local_14 = local_14 + 1) {
    local_10 = local_10 + *(ushort *)(&DAT_001de240 + (long)local_14 * 4);
  }
  for (; local_14 < 0x100; local_14 = local_14 + 1) {
    local_c = local_c + *(ushort *)(&DAT_001de240 + (long)local_14 * 4);
  }
  *DAT_001e1170 = (ushort)(local_c <= local_10 >> 2);
  return;
}




// Function: fillbuf @ 0xca3c

void fillbuf(int param_1)

{
  int local_c;
  
  DAT_001e13c0 = DAT_001e13c0 << ((byte)param_1 & 0x1f);
  local_c = param_1;
  while (DAT_001e13c8 < local_c) {
    local_c = local_c - DAT_001e13c8;
    DAT_001e13c0 = DAT_001e13c0 | (ushort)(DAT_001e13c4 << ((byte)local_c & 0x1f));
    if (DAT_001dd86c < DAT_001dd868) {
      DAT_001e13c4 = (uint)(byte)(&DAT_0011b000)[DAT_001dd86c];
      DAT_001dd86c = DAT_001dd86c + 1;
    }
    else {
      DAT_001e13c4 = FUN_0010f0bc(1);
    }
    if (DAT_001e13c4 == 0xffffffff) {
      DAT_001e13c4 = 0;
    }
    DAT_001e13c8 = 8;
  }
  DAT_001e13c8 = DAT_001e13c8 - local_c;
  DAT_001e13c0 = DAT_001e13c0 | (ushort)(DAT_001e13c4 >> ((byte)DAT_001e13c8 & 0x1f));
  return;
}




// Function: getbits @ 0xcb39

int getbits(undefined4 param_1)

{
  uint uVar1;
  
  uVar1 = (uint)DAT_001e13c0;
  FUN_0010ca3c(param_1);
  return (int)uVar1 >> (0x10U - (char)param_1 & 0x1f);
}




// Function: init_getbits @ 0xcb74

void init_getbits(void)

{
  DAT_001e13c0 = 0;
  DAT_001e13c4 = 0;
  DAT_001e13c8 = 0;
  FUN_0010ca3c(0x10);
  return;
}




// Function: make_table @ 0xcba6

void make_table(uint param_1,long param_2,uint param_3,long param_4)

{
  byte bVar1;
  uint uVar2;
  uint uVar3;
  long lVar4;
  byte bVar5;
  long in_FS_OFFSET;
  uint local_c0;
  uint local_bc;
  uint local_b8;
  uint local_b4;
  ushort *local_a0;
  short asStack_98 [24];
  ushort auStack_68 [24];
  ushort auStack_38 [20];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  for (local_c0 = 1; local_c0 < 0x11; local_c0 = local_c0 + 1) {
    asStack_98[local_c0] = 0;
  }
  for (local_c0 = 0; local_c0 < param_1; local_c0 = local_c0 + 1) {
    asStack_98[(int)(uint)*(byte *)(param_2 + (ulong)local_c0)] =
         asStack_98[(int)(uint)*(byte *)(param_2 + (ulong)local_c0)] + 1;
  }
  auStack_38[1] = 0;
  for (local_c0 = 1; local_c0 < 0x11; local_c0 = local_c0 + 1) {
    auStack_38[local_c0 + 1] =
         auStack_38[local_c0] + (asStack_98[local_c0] << (0x10U - (char)local_c0 & 0x1f));
  }
  if (auStack_38[0x11] != 0) {
    FUN_0010f5ef("Bad table\n");
  }
  bVar1 = (byte)param_3;
  bVar5 = 0x10 - bVar1;
  for (local_c0 = 1; local_c0 <= param_3; local_c0 = local_c0 + 1) {
    auStack_38[local_c0] = (ushort)((int)(uint)auStack_38[local_c0] >> (bVar5 & 0x1f));
    auStack_68[local_c0] = (ushort)(1 << (bVar1 - (char)local_c0 & 0x1f));
  }
  for (; local_c0 < 0x11; local_c0 = local_c0 + 1) {
    auStack_68[local_c0] = (ushort)(1 << (0x10U - (char)local_c0 & 0x1f));
  }
  local_c0 = (int)(uint)auStack_38[(int)(param_3 + 1)] >> (bVar5 & 0x1f);
  if (local_c0 != 0) {
    while (local_c0 != 1 << (bVar1 & 0x1f)) {
      *(undefined2 *)(param_4 + (ulong)local_c0 * 2) = 0;
      local_c0 = local_c0 + 1;
    }
  }
  local_b4 = param_1;
  for (local_b8 = 0; local_b8 < param_1; local_b8 = local_b8 + 1) {
    uVar2 = (uint)*(byte *)(param_2 + (ulong)local_b8);
    if (uVar2 != 0) {
      uVar3 = (uint)auStack_68[uVar2] + (uint)auStack_38[uVar2];
      if (param_3 < uVar2) {
        local_bc = (uint)auStack_38[uVar2];
        local_a0 = (ushort *)(param_4 + (ulong)(auStack_38[uVar2] >> (bVar5 & 0x1f)) * 2);
        for (local_c0 = uVar2 - param_3; local_c0 != 0; local_c0 = local_c0 + -1) {
          if (*local_a0 == 0) {
            (&DAT_001bd000)[local_b4] = 0;
            (&DAT_001bd000)[(ulong)local_b4 + 0x8000] = (&DAT_001bd000)[local_b4];
            *local_a0 = (ushort)local_b4;
            local_b4 = local_b4 + 1;
          }
          if ((local_bc & 1 << (0xf - bVar1 & 0x1f)) == 0) {
            lVar4 = (long)(int)(uint)*local_a0;
          }
          else {
            lVar4 = (ulong)*local_a0 + 0x8000;
          }
          local_a0 = &DAT_001bd000 + lVar4;
          local_bc = local_bc << 1;
        }
        *local_a0 = (ushort)local_b8;
      }
      else {
        if ((uint)(1 << (bVar1 & 0x1f)) < uVar3) {
          FUN_0010f5ef("Bad table\n");
        }
        for (local_c0 = (uint)auStack_38[uVar2]; local_c0 < uVar3; local_c0 = local_c0 + 1) {
          *(ushort *)(param_4 + (ulong)local_c0 * 2) = (ushort)local_b8;
        }
      }
      auStack_38[uVar2] = (ushort)uVar3;
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: read_pt_len @ 0xd0a0

void read_pt_len(int param_1,undefined4 param_2,int param_3)

{
  int iVar1;
  undefined2 uVar2;
  int iVar3;
  uint uVar4;
  int local_18;
  uint local_14;
  uint local_10;
  
  iVar3 = FUN_0010cb39(param_2);
  if (iVar3 == 0) {
    uVar2 = FUN_0010cb39(param_2);
    for (local_18 = 0; local_18 < param_1; local_18 = local_18 + 1) {
      (&DAT_001e1180)[local_18] = 0;
    }
    for (local_18 = 0; local_18 < 0x100; local_18 = local_18 + 1) {
      *(undefined2 *)(&DAT_001e11c0 + (long)local_18 * 2) = uVar2;
    }
  }
  else {
    local_18 = 0;
    while (local_18 < iVar3) {
      local_14 = (uint)(DAT_001e13c0 >> 0xd);
      if (local_14 == 7) {
        for (local_10 = 0x1000; (DAT_001e13c0 & local_10) != 0; local_10 = local_10 >> 1) {
          local_14 = local_14 + 1;
        }
        if (0x10 < (int)local_14) {
          FUN_0010f5ef("Bad table\n");
        }
      }
      uVar4 = local_14;
      if ((int)local_14 < 6) {
        uVar4 = 6;
      }
      FUN_0010ca3c(uVar4 - 3);
      iVar1 = local_18 + 1;
      (&DAT_001e1180)[local_18] = (char)local_14;
      local_18 = iVar1;
      if (iVar1 == param_3) {
        local_14 = FUN_0010cb39(2);
        while (local_14 = local_14 + -1, -1 < (int)local_14) {
          (&DAT_001e1180)[local_18] = 0;
          local_18 = local_18 + 1;
        }
      }
    }
    while (local_18 < param_1) {
      (&DAT_001e1180)[local_18] = 0;
      local_18 = local_18 + 1;
    }
    FUN_0010cba6(param_1,&DAT_001e1180,8,&DAT_001e11c0);
  }
  return;
}




// Function: read_c_len @ 0xd23e

void read_c_len(void)

{
  ushort uVar1;
  undefined2 uVar2;
  int iVar3;
  int iVar4;
  int local_18;
  uint local_14;
  uint local_10;
  
  iVar3 = FUN_0010cb39(9);
  if (iVar3 == 0) {
    uVar2 = FUN_0010cb39(9);
    for (local_18 = 0; local_18 < 0x1fe; local_18 = local_18 + 1) {
      (&DAT_0015c000)[local_18] = 0;
    }
    for (local_18 = 0; local_18 < 0x1000; local_18 = local_18 + 1) {
      *(undefined2 *)(&DAT_0019c800 + (long)local_18 * 2) = uVar2;
    }
  }
  else {
    local_18 = 0;
    while (local_18 < iVar3) {
      local_14 = (uint)*(ushort *)(&DAT_001e11c0 + (long)(int)(uint)(DAT_001e13c0 >> 8) * 2);
      if (0x12 < local_14) {
        local_10 = 0x80;
        do {
          if ((DAT_001e13c0 & local_10) == 0) {
            uVar1 = (&DAT_001bd000)[(int)local_14];
          }
          else {
            uVar1 = (&DAT_001bd000)[(long)(int)local_14 + 0x8000];
          }
          local_14 = (uint)uVar1;
          local_10 = local_10 >> 1;
        } while (0x12 < local_14);
      }
      FUN_0010ca3c((&DAT_001e1180)[(int)local_14]);
      if (local_14 < 3) {
        if (local_14 == 0) {
          local_14 = 1;
        }
        else if (local_14 == 1) {
          iVar4 = FUN_0010cb39(4);
          local_14 = iVar4 + 3;
        }
        else {
          iVar4 = FUN_0010cb39(9);
          local_14 = iVar4 + 0x14;
        }
        while (local_14 = local_14 + -1, -1 < (int)local_14) {
          (&DAT_0015c000)[local_18] = 0;
          local_18 = local_18 + 1;
        }
      }
      else {
        (&DAT_0015c000)[local_18] = (char)local_14 + -2;
        local_18 = local_18 + 1;
      }
    }
    while (local_18 < 0x1fe) {
      (&DAT_0015c000)[local_18] = 0;
      local_18 = local_18 + 1;
    }
    FUN_0010cba6(0x1fe,&DAT_0015c000,0xc,&DAT_0019c800);
  }
  return;
}




// Function: decode_c @ 0xd445

uint decode_c(void)

{
  ushort uVar1;
  uint local_10;
  uint local_c;
  
  if (DAT_001e11a0 == 0) {
    DAT_001e11a0 = FUN_0010cb39(0x10);
    if (DAT_001e11a0 == 0) {
      DAT_001e11a0 = 0;
      return 0x1fe;
    }
    FUN_0010d0a0(0x13,5,3);
    FUN_0010d23e();
    FUN_0010d0a0(0xe,4,0xffffffff);
  }
  DAT_001e11a0 = DAT_001e11a0 + -1;
  local_10 = (uint)*(ushort *)(&DAT_0019c800 + (long)(int)(uint)(DAT_001e13c0 >> 4) * 2);
  if (0x1fd < local_10) {
    local_c = 8;
    do {
      if ((DAT_001e13c0 & local_c) == 0) {
        uVar1 = (&DAT_001bd000)[local_10];
      }
      else {
        uVar1 = (&DAT_001bd000)[(ulong)local_10 + 0x8000];
      }
      local_10 = (uint)uVar1;
      local_c = local_c >> 1;
    } while (0x1fd < local_10);
  }
  FUN_0010ca3c((&DAT_0015c000)[local_10]);
  return local_10;
}




// Function: decode_p @ 0xd564

uint decode_p(void)

{
  ushort uVar1;
  int iVar2;
  uint local_20;
  uint local_1c;
  
  local_20 = (uint)*(ushort *)(&DAT_001e11c0 + (long)(int)(uint)(DAT_001e13c0 >> 8) * 2);
  if (0xd < local_20) {
    local_1c = 0x80;
    do {
      if ((DAT_001e13c0 & local_1c) == 0) {
        uVar1 = (&DAT_001bd000)[local_20];
      }
      else {
        uVar1 = (&DAT_001bd000)[(ulong)local_20 + 0x8000];
      }
      local_20 = (uint)uVar1;
      local_1c = local_1c >> 1;
    } while (0xd < local_20);
  }
  FUN_0010ca3c((&DAT_001e1180)[local_20]);
  if (local_20 != 0) {
    iVar2 = FUN_0010cb39(local_20 - 1);
    local_20 = iVar2 + (1 << ((char)local_20 - 1U & 0x1f));
  }
  return local_20;
}




// Function: huf_decode_start @ 0xd641

void huf_decode_start(void)

{
  FUN_0010cb74();
  DAT_001e11a0 = 0;
  return;
}




// Function: decode_start @ 0xd65b

void decode_start(void)

{
  FUN_0010d641();
  DAT_001e13cc = 0;
  DAT_001e13d0 = 0;
  return;
}




// Function: decode @ 0xd67f

uint decode(uint param_1,long param_2)

{
  uint uVar1;
  int iVar2;
  uint local_10;
  
  local_10 = 0;
  do {
    DAT_001e13cc = DAT_001e13cc + -1;
    if (DAT_001e13cc < 0) {
      do {
        while( true ) {
          uVar1 = FUN_0010d445();
          if (uVar1 == 0x1fe) {
            DAT_001e13d0 = 1;
            return local_10;
          }
          if (uVar1 < 0x100) break;
          DAT_001e13cc = uVar1 - 0xfd;
          iVar2 = FUN_0010d564();
          DAT_001e13d4 = (local_10 - iVar2) - 1 & 0x1fff;
          while (DAT_001e13cc = DAT_001e13cc + -1, -1 < DAT_001e13cc) {
            *(undefined1 *)(param_2 + (ulong)local_10) =
                 *(undefined1 *)(param_2 + (ulong)DAT_001e13d4);
            DAT_001e13d4 = DAT_001e13d4 + 1 & 0x1fff;
            local_10 = local_10 + 1;
            if (local_10 == param_1) {
              return local_10;
            }
          }
        }
        *(char *)(param_2 + (ulong)local_10) = (char)uVar1;
        local_10 = local_10 + 1;
      } while (local_10 != param_1);
      return local_10;
    }
    *(undefined1 *)(param_2 + (ulong)local_10) = *(undefined1 *)(param_2 + (ulong)DAT_001e13d4);
    DAT_001e13d4 = DAT_001e13d4 + 1 & 0x1fff;
    local_10 = local_10 + 1;
  } while (local_10 != param_1);
  return local_10;
}




// Function: unlzh @ 0xd7d6

undefined8 unlzh(undefined4 param_1,undefined4 param_2)

{
  int iVar1;
  
  DAT_001dd860 = param_1;
  DAT_001dd864 = param_2;
  FUN_0010d65b();
  while (DAT_001e13d0 == 0) {
    iVar1 = FUN_0010d67f(0x2000,&DAT_001ad000);
    if (iVar1 != 0) {
      FUN_0010f321(param_2,&DAT_001ad000,iVar1);
    }
  }
  return 0;
}




// Function: unlzw @ 0xd846

undefined8 unlzw(undefined4 param_1,undefined4 param_2)

{
  byte bVar1;
  uint uVar2;
  undefined8 uVar3;
  long lVar4;
  ulong uVar5;
  char *pcVar6;
  long lVar7;
  uint local_7c;
  int local_78;
  uint local_74;
  uint local_70;
  uint local_6c;
  int local_68;
  int local_64;
  byte *local_58;
  ulong local_50;
  ulong local_48;
  ulong local_40;
  long local_38;
  long local_30;
  
  if (DAT_001dd86c < DAT_001dd868) {
    DAT_00119098 = (uint)(byte)(&DAT_0011b000)[DAT_001dd86c];
    DAT_001dd86c = DAT_001dd86c + 1;
  }
  else {
    DAT_00119098 = FUN_0010f0bc(0);
  }
  DAT_00119548 = DAT_00119098 & 0x80;
  if ((DAT_00119098 & 0x60) != 0) {
    if (DAT_001dd004 == 0) {
      fprintf(stderr,"\n%s: %s: warning, unknown flags 0x%x\n",DAT_001dd010,&DAT_001dd060,
              (ulong)(DAT_00119098 & 0x60));
    }
    if (DAT_001dd018 == 0) {
      DAT_001dd018 = 2;
    }
  }
  DAT_00119098 = DAT_00119098 & 0x1f;
  lVar7 = 1L << (sbyte)DAT_00119098;
  if (DAT_00119098 < 0x11) {
    local_6c = DAT_001dd868;
    local_70 = 9;
    local_30 = 0x1ff;
    local_74 = 0x1ff;
    local_48 = 0xffffffffffffffff;
    local_7c = 0;
    local_78 = 0;
    local_40 = (ulong)(DAT_001dd86c << 3);
    if (DAT_00119548 == 0) {
      local_38 = 0x100;
    }
    else {
      local_38 = 0x101;
    }
    memset(&DAT_001bd000,0,0x100);
    for (local_50 = 0xff; -1 < (long)local_50; local_50 = local_50 + -1) {
      (&DAT_001ad000)[local_50] = (char)local_50;
    }
LAB_0010da53:
    do {
      uVar2 = (uint)((long)local_40 >> 3);
      if (DAT_001dd868 < uVar2) {
        DAT_001dd868 = 0;
      }
      else {
        DAT_001dd868 = DAT_001dd868 - uVar2;
      }
      for (local_68 = 0; local_68 < (int)DAT_001dd868; local_68 = local_68 + 1) {
        (&DAT_0011b000)[local_68] = (&DAT_0011b000)[(int)(uVar2 + local_68)];
      }
      local_40 = 0;
      if (DAT_001dd868 < 0x40) {
        local_6c = FUN_0010f194(param_1,&DAT_0011b000 + DAT_001dd868,0x40000);
        if (local_6c == 0xffffffff) {
          FUN_0010f6c7();
        }
        DAT_001dd868 = local_6c + DAT_001dd868;
        DAT_001dd040 = DAT_001dd040 + (int)local_6c;
      }
      uVar5 = local_48;
      if (local_6c == 0) {
        lVar4 = (ulong)DAT_001dd868 * 8 - (long)(int)(local_70 - 1);
      }
      else {
        lVar4 = ((ulong)DAT_001dd868 - (ulong)(DAT_001dd868 % local_70)) * 8;
      }
      while (local_48 = uVar5, (long)local_40 < lVar4) {
        if (local_30 < local_38) {
          local_40 = ((long)(int)(local_70 << 3) -
                     (long)((long)(int)(local_70 << 3) + (local_40 - 1)) %
                     (long)(int)(local_70 << 3)) + (local_40 - 1);
          local_70 = local_70 + 1;
          local_30 = lVar7;
          if (local_70 != DAT_00119098) {
            local_30 = (1L << ((byte)local_70 & 0x3f)) + -1;
          }
          local_74 = (1 << ((byte)local_70 & 0x1f)) - 1;
          goto LAB_0010da53;
        }
        uVar5 = (ulong)local_74 &
                (long)(ulong)*(uint3 *)(&DAT_0011b000 + ((long)local_40 >> 3)) >>
                ((byte)local_40 & 7);
        local_40 = local_40 + (long)(int)local_70;
        if (local_48 == 0xffffffffffffffff) {
          if (0xff < uVar5) {
            FUN_0010f5ef("corrupt input.");
          }
          local_7c = (uint)uVar5;
          (&DAT_0015c000)[local_78] = (char)uVar5;
          local_78 = local_78 + 1;
        }
        else {
          if ((uVar5 == 0x100) && (DAT_00119548 != 0)) {
            memset(&DAT_001bd000,0,0x100);
            local_38 = 0x100;
            local_40 = ((long)(int)(local_70 << 3) -
                       (long)((long)(int)(local_70 << 3) + (local_40 - 1)) %
                       (long)(int)(local_70 << 3)) + (local_40 - 1);
            local_70 = 9;
            local_30 = 0x1ff;
            local_74 = 0x1ff;
            goto LAB_0010da53;
          }
          local_58 = &DAT_001ac7fe;
          local_50 = uVar5;
          if (local_38 <= (long)uVar5) {
            if (local_38 < (long)uVar5) {
              if (0 < local_78) {
                FUN_0010f321(param_2,&DAT_0015c000,local_78);
              }
              if (DAT_001dd000 == 0) {
                pcVar6 = "corrupt input. Use zcat to recover some data.";
              }
              else {
                pcVar6 = "corrupt input.";
              }
              FUN_0010f5ef(pcVar6);
            }
            local_58 = &DAT_001ac7fd;
            DAT_001ac7fd = (undefined1)local_7c;
            local_50 = local_48;
          }
          for (; 0xff < local_50; local_50 = (ulong)(ushort)(&DAT_001bd000)[local_50]) {
            local_58 = local_58 + -1;
            *local_58 = (&DAT_001ad000)[local_50];
          }
          bVar1 = (&DAT_001ad000)[local_50];
          local_7c = (uint)bVar1;
          local_58 = local_58 + -1;
          *local_58 = bVar1;
          local_64 = 0x1ac7fe - (int)local_58;
          if (local_78 + local_64 < 0x40000) {
            memcpy(&DAT_0015c000 + local_78,local_58,(long)local_64);
            local_78 = local_78 + local_64;
          }
          else {
            do {
              if (0x40000 - local_78 < local_64) {
                local_64 = 0x40000 - local_78;
              }
              if (0 < local_64) {
                memcpy(&DAT_0015c000 + local_78,local_58,(long)local_64);
                local_78 = local_78 + local_64;
              }
              if (0x3ffff < local_78) {
                FUN_0010f321(param_2,&DAT_0015c000,local_78);
                local_78 = 0;
              }
              local_58 = local_58 + local_64;
              local_64 = 0x1ac7fe - (int)local_58;
            } while (0 < local_64);
          }
          if (local_38 < lVar7) {
            (&DAT_001bd000)[local_38] = (short)local_48;
            (&DAT_001ad000)[local_38] = bVar1;
            local_38 = local_38 + 1;
          }
        }
      }
    } while (local_6c != 0);
    if (0 < local_78) {
      FUN_0010f321(param_2,&DAT_0015c000,local_78);
    }
    uVar3 = 0;
  }
  else {
    fprintf(stderr,"\n%s: %s: compressed with %d bits, can only handle %d bits\n",DAT_001dd010,
            &DAT_001dd060,(ulong)DAT_00119098,0x10);
    DAT_001dd018 = 1;
    uVar3 = 1;
  }
  return uVar3;
}




// Function: read_byte @ 0xdff1

uint read_byte(void)

{
  uint uVar1;
  
  if (DAT_001dd86c < DAT_001dd868) {
    uVar1 = (uint)(byte)(&DAT_0011b000)[DAT_001dd86c];
    DAT_001dd86c = DAT_001dd86c + 1;
  }
  else {
    uVar1 = FUN_0010f0bc(0);
  }
  if ((int)uVar1 < 0) {
    FUN_0010f5ef("invalid compressed data -- unexpected end of file");
  }
  return uVar1;
}




// Function: read_tree @ 0xe055

void read_tree(void)

{
  byte bVar1;
  undefined1 uVar2;
  ulong uVar3;
  uint local_28;
  int local_24;
  int local_20;
  int local_1c;
  
  local_1c = 1;
  DAT_001e13e0 = 0;
  for (local_20 = 1; local_20 < 5; local_20 = local_20 + 1) {
    uVar3 = DAT_001e13e0 << 8;
    bVar1 = FUN_0010dff1();
    DAT_001e13e0 = bVar1 | uVar3;
  }
  bVar1 = FUN_0010dff1();
  DAT_001e13e8 = (uint)bVar1;
  if ((DAT_001e13e8 == 0) || (0x19 < DAT_001e13e8)) {
    FUN_0010f5ef("invalid compressed data -- Huffman code bit length out of range");
  }
  local_20 = 0;
  for (local_28 = 1; (int)local_28 <= (int)DAT_001e13e8; local_28 = local_28 + 1) {
    bVar1 = FUN_0010dff1();
    *(uint *)(&DAT_001e1580 + (long)(int)local_28 * 4) = (uint)bVar1;
    if ((int)(local_1c - (uint)(local_28 == DAT_001e13e8)) <
        *(int *)(&DAT_001e1580 + (long)(int)local_28 * 4)) {
      FUN_0010f5ef("too many leaves in Huffman tree");
    }
    local_1c = ((local_1c - *(int *)(&DAT_001e1580 + (long)(int)local_28 * 4)) + 1) * 2 + -1;
    local_20 = local_20 + *(int *)(&DAT_001e1580 + (long)(int)local_28 * 4);
  }
  if (0xff < local_20) {
    FUN_0010f5ef("too many leaves in Huffman tree");
  }
  *(int *)(&DAT_001e1580 + (long)(int)DAT_001e13e8 * 4) =
       *(int *)(&DAT_001e1580 + (long)(int)DAT_001e13e8 * 4) + 1;
  local_24 = 0;
  for (local_28 = 1; (int)local_28 <= (int)DAT_001e13e8; local_28 = local_28 + 1) {
    *(int *)(&DAT_001e1500 + (long)(int)local_28 * 4) = local_24;
    for (local_20 = *(int *)(&DAT_001e1580 + (long)(int)local_28 * 4); 0 < local_20;
        local_20 = local_20 + -1) {
      uVar2 = FUN_0010dff1();
      (&DAT_001e1400)[local_24] = uVar2;
      local_24 = local_24 + 1;
    }
  }
  *(int *)(&DAT_001e1580 + (long)(int)DAT_001e13e8 * 4) =
       *(int *)(&DAT_001e1580 + (long)(int)DAT_001e13e8 * 4) + 1;
  return;
}




// Function: unpack @ 0xe3fd

undefined8 unpack(undefined4 param_1,undefined4 param_2)

{
  int iVar1;
  byte bVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  ulong uVar6;
  uint local_2c;
  
  DAT_001dd860 = param_1;
  DAT_001dd864 = param_2;
  FUN_0010e055();
  FUN_0010e29d();
  DAT_001e1678 = 0;
  DAT_001e1670 = 0;
  uVar3 = (1 << ((byte)DAT_001e1668 & 0x1f)) - 1;
  iVar1 = *(int *)(&DAT_001e1580 + (long)(int)DAT_001e13e8 * 4);
  while( true ) {
    for (; DAT_001e1678 < (int)DAT_001e1668; DAT_001e1678 = DAT_001e1678 + 8) {
      uVar6 = DAT_001e1670 << 8;
      bVar2 = FUN_0010dff1();
      DAT_001e1670 = bVar2 | uVar6;
    }
    uVar4 = (uint)(DAT_001e1670 >> ((char)DAT_001e1678 - (char)DAT_001e1668 & 0x3fU)) & uVar3;
    local_2c = (uint)(byte)(&DAT_0015c000)[uVar4];
    if (local_2c == 0) {
      local_2c = DAT_001e1668;
      uVar5 = uVar3;
      while (uVar4 < *(uint *)(&DAT_001e1600 + (long)(int)local_2c * 4)) {
        local_2c = local_2c + 1;
        uVar5 = uVar5 * 2 + 1;
        for (; DAT_001e1678 < (int)local_2c; DAT_001e1678 = DAT_001e1678 + 8) {
          uVar6 = DAT_001e1670 << 8;
          bVar2 = FUN_0010dff1();
          DAT_001e1670 = bVar2 | uVar6;
        }
        uVar4 = (uint)(DAT_001e1670 >> ((char)DAT_001e1678 - (char)local_2c & 0x3fU)) & uVar5;
      }
    }
    else {
      uVar4 = uVar4 >> ((char)DAT_001e1668 - (&DAT_0015c000)[uVar4] & 0x1fU);
    }
    if ((uVar4 == iVar1 - 1U) && (local_2c == DAT_001e13e8)) break;
    uVar6 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_001ad000)[uVar6] =
         (&DAT_001e1400)[uVar4 + *(int *)(&DAT_001e1500 + (long)(int)local_2c * 4)];
    if (DAT_001dd870 == 0x8000) {
      FUN_0010f2cc();
    }
    DAT_001e1678 = DAT_001e1678 - local_2c;
  }
  FUN_0010f2cc();
  if ((DAT_001dd048 & 0xffffffff) != DAT_001e13e0) {
    FUN_0010f5ef("invalid compressed data--length error");
  }
  return 0;
}




// Function: check_zipfile @ 0xe655

undefined8 check_zipfile(undefined4 param_1)

{
  undefined8 uVar1;
  ulong uVar2;
  
  uVar2 = (ulong)DAT_001dd86c;
  DAT_001dd86c = DAT_001dd86c +
                 (uint)*(ushort *)(&DAT_0011b01c + uVar2) +
                 *(ushort *)(&DAT_0011b01a + uVar2) + 0x1e;
  DAT_001dd860 = param_1;
  if ((DAT_001dd868 < DAT_001dd86c) ||
     (((long)(int)(uint)*(ushort *)(&DAT_0011b002 + uVar2) << 0x10 |
      (long)(int)(uint)*(ushort *)(&DAT_0011b000 + uVar2)) != 0x4034b50)) {
    fprintf(stderr,"\n%s: %s: not a valid zip file\n",DAT_001dd010,&DAT_001dd060);
    DAT_001dd018 = 1;
    uVar1 = 1;
  }
  else {
    DAT_0011909c = (uint)(byte)(&DAT_0011b008)[uVar2];
    if ((DAT_0011909c == 0) || (DAT_0011909c == 8)) {
      DAT_001e1688 = (byte)(&DAT_0011b006)[uVar2] & 1;
      if (((&DAT_0011b006)[uVar2] & 1) == 0) {
        DAT_001e1690 = (uint)(((&DAT_0011b006)[uVar2] & 8) != 0);
        DAT_001e168c = 1;
        uVar1 = 0;
      }
      else {
        fprintf(stderr,"\n%s: %s: encrypted file -- use unzip\n",DAT_001dd010,&DAT_001dd060);
        DAT_001dd018 = 1;
        uVar1 = 1;
      }
    }
    else {
      fprintf(stderr,"\n%s: %s: first entry not deflated or stored -- use unzip\n",DAT_001dd010,
              &DAT_001dd060);
      DAT_001dd018 = 1;
      uVar1 = 1;
    }
  }
  return uVar1;
}




// Function: unzip @ 0xe86a

bool unzip(undefined4 param_1,undefined4 param_2)

{
  undefined1 uVar1;
  int iVar2;
  long lVar3;
  ulong uVar4;
  ulong uVar5;
  long in_FS_OFFSET;
  bool bVar6;
  int local_54;
  ulong local_48;
  ulong local_40;
  ushort local_38;
  ushort local_36;
  ushort local_34;
  ushort local_32;
  ushort local_2c;
  ushort local_2a;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_48 = 0;
  local_40 = 0;
  DAT_001dd860 = param_1;
  DAT_001dd864 = param_2;
  FUN_0010efda(0,0);
  if ((DAT_001e168c != 0) && (DAT_001e1690 == 0)) {
    local_48 = (long)(int)(uint)CONCAT11(DAT_0011b011,DAT_0011b010) << 0x10 |
               (long)(int)(uint)CONCAT11(DAT_0011b00f,DAT_0011b00e);
    local_40 = (long)(int)(uint)CONCAT11(DAT_0011b019,DAT_0011b018) << 0x10 |
               (long)(int)(uint)CONCAT11(DAT_0011b017,DAT_0011b016);
  }
  if (DAT_0011909c == 8) {
    iVar2 = FUN_0010ab98();
    if (iVar2 == 3) {
      FUN_0010f634();
    }
    if (iVar2 == 0) goto LAB_0010eb57;
    FUN_0010f5ef("invalid compressed data--format violated");
  }
  if ((DAT_001e168c == 0) || (DAT_0011909c != 0)) {
    FUN_0010f5ef("internal error, invalid method");
  }
  else {
    uVar5 = (long)(int)(uint)CONCAT11(DAT_0011b017,DAT_0011b016) |
            (long)(int)(uint)CONCAT11(DAT_0011b019,DAT_0011b018) << 0x10;
    if (DAT_001e1688 == 0) {
      lVar3 = 0;
    }
    else {
      lVar3 = 0xc;
    }
    uVar4 = uVar5;
    if (uVar5 == ((long)(int)(uint)CONCAT11(DAT_0011b013,DAT_0011b012) |
                 (long)(int)(uint)CONCAT11(DAT_0011b015,DAT_0011b014) << 0x10) - lVar3)
    goto LAB_0010eb35;
    fprintf(stderr,"len %lu, siz %lu\n",uVar5,
            (long)(int)(uint)CONCAT11(DAT_0011b013,DAT_0011b012) |
            (long)(int)(uint)CONCAT11(DAT_0011b015,DAT_0011b014) << 0x10);
    FUN_0010f5ef("invalid compressed data--length mismatch");
    do {
      if (DAT_001dd86c < DAT_001dd868) {
        uVar1 = (&DAT_0011b000)[DAT_001dd86c];
        DAT_001dd86c = DAT_001dd86c + 1;
      }
      else {
        uVar1 = FUN_0010f0bc(0);
      }
      uVar4 = (ulong)DAT_001dd870;
      DAT_001dd870 = DAT_001dd870 + 1;
      (&DAT_001ad000)[uVar4] = uVar1;
      uVar4 = uVar5;
      if (DAT_001dd870 == 0x8000) {
        FUN_0010f2cc();
      }
LAB_0010eb35:
      uVar5 = uVar4 - 1;
    } while (uVar4 != 0);
    FUN_0010f2cc();
  }
LAB_0010eb57:
  if (DAT_001e168c == 0) {
    for (local_54 = 0; local_54 < 8; local_54 = local_54 + 1) {
      if (DAT_001dd86c < DAT_001dd868) {
        uVar1 = (&DAT_0011b000)[DAT_001dd86c];
        DAT_001dd86c = DAT_001dd86c + 1;
      }
      else {
        uVar1 = FUN_0010f0bc(0);
      }
      *(undefined1 *)((long)&local_38 + (long)local_54) = uVar1;
    }
    local_48 = (long)(int)(uint)local_36 << 0x10 | (long)(int)(uint)local_38;
    local_40 = (long)(int)(uint)local_32 << 0x10 | (long)(int)(uint)local_34;
  }
  else if (DAT_001e1690 != 0) {
    for (local_54 = 0; local_54 < 0x10; local_54 = local_54 + 1) {
      if (DAT_001dd86c < DAT_001dd868) {
        uVar1 = (&DAT_0011b000)[DAT_001dd86c];
        DAT_001dd86c = DAT_001dd86c + 1;
      }
      else {
        uVar1 = FUN_0010f0bc(0);
      }
      *(undefined1 *)((long)&local_38 + (long)local_54) = uVar1;
    }
    local_48 = (long)(int)(uint)local_32 << 0x10 | (long)(int)(uint)local_34;
    local_40 = (long)(int)(uint)local_2a << 0x10 | (long)(int)(uint)local_2c;
  }
  uVar5 = FUN_0010efda(&DAT_0015c000,0);
  if (local_48 != uVar5) {
    fprintf(stderr,"\n%s: %s: invalid compressed data--crc error\n",DAT_001dd010,&DAT_001dd060);
  }
  bVar6 = local_40 != (DAT_001dd048 & 0xffffffff);
  if (bVar6) {
    fprintf(stderr,"\n%s: %s: invalid compressed data--length error\n",DAT_001dd010,&DAT_001dd060);
  }
  bVar6 = bVar6 || local_48 != uVar5;
  if (((DAT_001e168c != 0) && (DAT_001dd86c + 4 < DAT_001dd868)) &&
     (((long)(int)(uint)CONCAT11((&DAT_0011b003)[DAT_001dd86c],(&DAT_0011b002)[DAT_001dd86c]) <<
       0x10 | (long)(int)(uint)CONCAT11((&DAT_0011b001)[DAT_001dd86c],(&DAT_0011b000)[DAT_001dd86c])
      ) == 0x4034b50)) {
    if (DAT_001dd000 == 0) {
      fprintf(stderr,"%s: %s has more than one entry -- unchanged\n",DAT_001dd010,&DAT_001dd060);
      bVar6 = true;
    }
    else {
      if (DAT_001dd004 == 0) {
        fprintf(stderr,"%s: %s has more than one entry--rest ignored\n",DAT_001dd010,&DAT_001dd060);
      }
      if (DAT_001dd018 == 0) {
        DAT_001dd018 = 2;
      }
    }
  }
  DAT_001e168c = 0;
  DAT_001e1690 = 0;
  DAT_001e1680 = local_48;
  if (bVar6 == false) {
    bVar6 = false;
  }
  else {
    DAT_001dd018 = 1;
    if (DAT_001dd008 == 0) {
                    /* WARNING: Subroutine does not return */
      FUN_00108f9f();
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar6;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: copy @ 0xef1d

undefined8 copy(undefined4 param_1,undefined4 param_2)

{
  uint uVar1;
  int *piVar2;
  
  piVar2 = __errno_location();
  *piVar2 = 0;
  while (DAT_001dd86c < DAT_001dd868) {
    FUN_0010f321(param_2,&DAT_0011b000 + DAT_001dd86c,DAT_001dd868 - DAT_001dd86c);
    uVar1 = FUN_0010f194(param_1,&DAT_0011b000,0x40000);
    if (uVar1 == 0xffffffff) {
      FUN_0010f6c7();
    }
    DAT_001dd040 = DAT_001dd040 + (int)uVar1;
    DAT_001dd868 = uVar1;
    DAT_001dd86c = 0;
  }
  return 0;
}




// Function: updcrc @ 0xefda

ulong updcrc(byte *param_1,int param_2)

{
  if (param_1 == (byte *)0x0) {
    DAT_00119550 = 0xffffffff;
  }
  else {
    for (; param_2 != 0; param_2 = param_2 + -1) {
      DAT_00119550 = *(ulong *)(&DAT_00114680 +
                               (long)(int)(((uint)*param_1 ^ (uint)DAT_00119550) & 0xff) * 8) ^
                     DAT_00119550 >> 8;
      param_1 = param_1 + 1;
    }
  }
  return DAT_00119550 ^ 0xffffffff;
}




// Function: getcrc @ 0xf05f

ulong getcrc(void)

{
  return DAT_00119550 ^ 0xffffffff;
}




// Function: clear_bufs @ 0xf078

void clear_bufs(void)

{
  DAT_001dd870 = 0;
  DAT_001dd86c = 0;
  DAT_001dd868 = 0;
  DAT_001dd048 = 0;
  DAT_001dd040 = 0;
  return;
}




// Function: fill_inbuf @ 0xf0bc

ulong fill_inbuf(int param_1)

{
  int iVar1;
  int *piVar2;
  
  DAT_001dd868 = 0;
  do {
    iVar1 = FUN_0010f194(DAT_001dd860,&DAT_0011b000 + DAT_001dd868,0x40000 - DAT_001dd868);
    if (iVar1 == 0) break;
    if (iVar1 == -1) {
      FUN_0010f6c7();
    }
    DAT_001dd868 = iVar1 + DAT_001dd868;
  } while (DAT_001dd868 < 0x40000);
  if (DAT_001dd868 == 0) {
    if (param_1 != 0) {
      return 0xffffffff;
    }
    FUN_0010f2cc();
    piVar2 = __errno_location();
    *piVar2 = 0;
    FUN_0010f6c7();
  }
  DAT_001dd86c = 1;
  DAT_001dd040 = DAT_001dd040 + (ulong)DAT_001dd868;
  return (ulong)DAT_0011b000;
}




// Function: read_buffer @ 0xf194

int read_buffer(int param_1,void *param_2,uint param_3)

{
  uint uVar1;
  int iVar2;
  ssize_t sVar3;
  int *piVar4;
  undefined4 local_20;
  undefined4 local_10;
  
  local_20 = param_3;
  if ((int)param_3 < 0) {
    local_20 = 0x7fffffff;
  }
  sVar3 = read(param_1,param_2,(ulong)local_20);
  local_10 = (int)sVar3;
  if (local_10 < 0) {
    piVar4 = __errno_location();
    if (*piVar4 == 0xb) {
      uVar1 = FUN_001103bd(param_1,3);
      if (-1 < (int)uVar1) {
        if ((uVar1 & 0x800) == 0) {
          piVar4 = __errno_location();
          *piVar4 = 0xb;
        }
        else {
          iVar2 = FUN_001103bd(param_1,4,uVar1 & 0xfffff7ff);
          if (iVar2 != -1) {
            sVar3 = read(param_1,param_2,(ulong)local_20);
            local_10 = (int)sVar3;
          }
        }
      }
    }
  }
  return local_10;
}




// Function: write_buffer @ 0xf254

void write_buffer(int param_1,void *param_2,uint param_3)

{
  undefined4 local_10;
  
  local_10 = param_3;
  if ((int)param_3 < 0) {
    local_10 = 0x7fffffff;
  }
  write(param_1,param_2,(ulong)local_10);
  return;
}




// Function: flush_outbuf @ 0xf28e

void flush_outbuf(void)

{
  if (DAT_001dd870 != 0) {
    FUN_0010f321(DAT_001dd864,&DAT_0015c000,DAT_001dd870);
    DAT_001dd870 = 0;
  }
  return;
}




// Function: flush_window @ 0xf2cc

void flush_window(void)

{
  if (DAT_001dd870 != 0) {
    FUN_0010efda(&DAT_001ad000,DAT_001dd870);
    FUN_0010f321(DAT_001dd864,&DAT_001ad000,DAT_001dd870);
    DAT_001dd870 = 0;
  }
  return;
}




// Function: write_buf @ 0xf321

void write_buf(undefined4 param_1,long param_2,uint param_3)

{
  uint uVar1;
  long local_28;
  uint local_20;
  
  DAT_001dd048 = DAT_001dd048 + (ulong)param_3;
  local_28 = param_2;
  local_20 = param_3;
  if (DAT_001dd008 == 0) {
    for (; uVar1 = FUN_0010f254(param_1,local_28,local_20), uVar1 != local_20;
        local_20 = local_20 - uVar1) {
      if (uVar1 == 0xffffffff) {
        FUN_0010f74d();
      }
      local_28 = local_28 + (ulong)uVar1;
    }
  }
  return;
}




// Function: strlwr @ 0xf393

byte * strlwr(byte *param_1)

{
  byte bVar1;
  ushort **ppuVar3;
  byte *local_10;
  int iVar2;
  
  for (local_10 = param_1; *local_10 != 0; local_10 = local_10 + 1) {
    ppuVar3 = __ctype_b_loc();
    if (((*ppuVar3)[*local_10] & 0x100) == 0) {
      bVar1 = *local_10;
    }
    else {
      iVar2 = tolower((uint)*local_10);
      bVar1 = (byte)iVar2;
    }
    *local_10 = bVar1;
  }
  return param_1;
}




// Function: gzip_base_name @ 0xf40a

undefined8 gzip_base_name(undefined8 param_1)

{
  undefined8 uVar1;
  
  uVar1 = FUN_00110244(param_1);
  return uVar1;
}




// Function: xunlink @ 0xf430

int xunlink(char *param_1)

{
  int iVar1;
  
  iVar1 = unlink(param_1);
  return iVar1;
}




// Function: add_envopt @ 0xf454

char * add_envopt(int *param_1,undefined8 *param_2,char *param_3)

{
  char *pcVar1;
  char cVar2;
  long *plVar3;
  char *pcVar4;
  size_t sVar5;
  int local_2c;
  char *local_28;
  long *local_20;
  
  local_2c = 0;
  pcVar4 = getenv(param_3);
  if (pcVar4 == (char *)0x0) {
    pcVar4 = (char *)0x0;
  }
  else {
    pcVar4 = (char *)FUN_00112377(pcVar4);
    local_28 = pcVar4;
    while (*local_28 != '\0') {
      sVar5 = strspn(local_28," \t");
      local_28 = local_28 + sVar5;
      if (*local_28 == '\0') break;
      sVar5 = strcspn(local_28," \t");
      pcVar1 = local_28 + sVar5;
      local_28 = pcVar1;
      if (*pcVar1 != '\0') {
        local_28 = pcVar1 + 1;
        *pcVar1 = '\0';
      }
      local_2c = local_2c + 1;
    }
    if (local_2c == 0) {
      free(pcVar4);
      pcVar4 = (char *)0x0;
    }
    else {
      *param_1 = local_2c + 1;
      local_20 = (long *)FUN_0011224e((long)(*param_1 + 1),8);
      plVar3 = (long *)*param_2;
      *param_2 = local_20;
      *local_20 = *plVar3;
      local_28 = pcVar4;
      for (; local_20 = local_20 + 1, 0 < local_2c; local_2c = local_2c + -1) {
        sVar5 = strspn(local_28," \t");
        *local_20 = (long)(local_28 + sVar5);
        local_28 = local_28 + sVar5;
        do {
          pcVar1 = local_28 + 1;
          cVar2 = *local_28;
          local_28 = pcVar1;
        } while (cVar2 != '\0');
      }
      *local_20 = 0;
    }
  }
  return pcVar4;
}




// Function: gzip_error @ 0xf5ef

void gzip_error(undefined8 param_1)

{
  fprintf(stderr,"\n%s: %s: %s\n",DAT_001dd010,&DAT_001dd060,param_1);
                    /* WARNING: Subroutine does not return */
  FUN_00108f9f();
}




// Function: xalloc_die @ 0xf634

void xalloc_die(void)

{
  fprintf(stderr,"\n%s: memory_exhausted\n",DAT_001dd010);
                    /* WARNING: Subroutine does not return */
  FUN_00108f9f();
}




// Function: warning @ 0xf666

void warning(undefined8 param_1)

{
  if (DAT_001dd004 == 0) {
    fprintf(stderr,"%s: %s: warning: %s\n",DAT_001dd010,&DAT_001dd060,param_1);
  }
  if (DAT_001dd018 == 0) {
    DAT_001dd018 = 2;
  }
  return;
}




// Function: read_error @ 0xf6c7

void read_error(void)

{
  int iVar1;
  int *piVar2;
  
  piVar2 = __errno_location();
  iVar1 = *piVar2;
  fprintf(stderr,"\n%s: ",DAT_001dd010);
  if (iVar1 == 0) {
    fprintf(stderr,"%s: unexpected end of file\n",&DAT_001dd060);
  }
  else {
    piVar2 = __errno_location();
    *piVar2 = iVar1;
    perror(&DAT_001dd060);
  }
                    /* WARNING: Subroutine does not return */
  FUN_00108f9f();
}




// Function: write_error @ 0xf74d

void write_error(void)

{
  int iVar1;
  int *piVar2;
  
  piVar2 = __errno_location();
  iVar1 = *piVar2;
  fprintf(stderr,"\n%s: ",DAT_001dd010);
  piVar2 = __errno_location();
  *piVar2 = iVar1;
  perror(&DAT_001dd460);
                    /* WARNING: Subroutine does not return */
  FUN_00108f9f();
}




// Function: display_ratio @ 0xf7a6

void display_ratio(long param_1,long param_2,FILE *param_3)

{
  double dVar1;
  
  dVar1 = DAT_00114ef0;
  if (param_2 != 0) {
    dVar1 = (DAT_00114ee8 * (double)param_1) / (double)param_2;
  }
  fprintf(param_3,"%5.1f%%",dVar1);
  return;
}




// Function: fprint_off @ 0xf81a

void fprint_off(FILE *param_1,long param_2,int param_3)

{
  char *pcVar1;
  long in_FS_OFFSET;
  int local_7c;
  long local_78;
  char *local_60;
  undefined1 local_58 [62];
  char local_1a [10];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_60 = local_1a + 2;
  local_78 = param_2;
  pcVar1 = local_60;
  if (param_2 < 0) {
    do {
      local_60 = pcVar1;
      local_60[-1] = '0' - ((char)local_78 + (char)(local_78 / 10) * -10);
      local_78 = local_78 / 10;
      pcVar1 = local_60 + -1;
    } while (local_78 != 0);
    local_60 = local_60 + -2;
    *local_60 = '-';
  }
  else {
    do {
      local_60 = local_60 + -1;
      *local_60 = (char)local_78 + (char)(local_78 / 10) * -10 + '0';
      local_78 = local_78 / 10;
    } while (local_78 != 0);
  }
  local_7c = param_3 - (((int)local_58 + 0x40) - (int)local_60);
  while (0 < local_7c) {
    putc(0x20,param_1);
    local_7c = local_7c + -1;
  }
  for (; local_60 < local_1a + 2; local_60 = local_60 + 1) {
    putc((int)*local_60,param_1);
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: zip @ 0xf9dd

/* WARNING: Removing unreachable block (ram,0x0010fa61) */
/* WARNING: Removing unreachable block (ram,0x0010fa93) */
/* WARNING: Removing unreachable block (ram,0x0010fac1) */
/* WARNING: Removing unreachable block (ram,0x0010fb00) */

undefined8 zip(undefined4 param_1,undefined4 param_2)

{
  char *pcVar1;
  char cVar2;
  undefined1 uVar3;
  undefined1 extraout_AH;
  undefined1 extraout_AH_00;
  undefined1 extraout_var;
  undefined1 extraout_var_00;
  undefined1 extraout_var_01;
  undefined1 extraout_var_02;
  ulong uVar4;
  long in_FS_OFFSET;
  undefined1 local_25;
  undefined2 local_24;
  ushort local_22;
  long local_20;
  char *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_25 = 0;
  if (DAT_001dd01c != 0) {
    local_25 = 8;
  }
  DAT_0015c003 = local_25;
  DAT_0015c002 = 8;
  DAT_0015c001 = 0x8b;
  DAT_0015c000 = 0x1f;
  local_22 = 0;
  local_24 = 0;
  DAT_001dd870 = 4;
  DAT_0011909c = 8;
  DAT_001dd860 = param_1;
  DAT_001dd864 = param_2;
  if (DAT_001dd038 < 0) {
    local_20 = 0;
  }
  else if ((DAT_001dd030 < 1) || (0xffffffff < DAT_001dd030)) {
    FUN_0010f666("file timestamp out of range for gzip format");
    local_20 = 0;
  }
  else {
    local_20 = DAT_001dd030;
  }
  if (DAT_001dd870 < 0x3fffe) {
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = (char)local_20;
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = (char)((ulong)local_20 >> 8);
  }
  else {
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = (char)local_20;
    if (DAT_001dd870 == 0x40000) {
      FUN_0010f28e();
    }
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = (char)((ulong)local_20 >> 8);
    if (DAT_001dd870 == 0x40000) {
      FUN_0010f28e();
    }
  }
  uVar3 = (undefined1)((ulong)local_20 >> 0x10);
  if (DAT_001dd870 < 0x3fffe) {
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = uVar3;
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = (char)((ulong)local_20 >> 0x18);
  }
  else {
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = uVar3;
    if (DAT_001dd870 == 0x40000) {
      FUN_0010f28e();
    }
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = (char)((ulong)local_20 >> 0x18);
    if (DAT_001dd870 == 0x40000) {
      FUN_0010f28e();
    }
  }
  FUN_0010efda(0,0);
  FUN_00103a29(param_2);
  FUN_0010ac6f(&local_24,&DAT_0011909c);
  if (DAT_001190a0 == 1) {
    local_22 = local_22 | 4;
  }
  else if (DAT_001190a0 == 9) {
    local_22 = local_22 | 2;
  }
  uVar4 = (ulong)DAT_001dd870;
  DAT_001dd870 = DAT_001dd870 + 1;
  (&DAT_0015c000)[uVar4] = (char)local_22;
  if (DAT_001dd870 == 0x40000) {
    FUN_0010f28e();
  }
  uVar4 = (ulong)DAT_001dd870;
  DAT_001dd870 = DAT_001dd870 + 1;
  (&DAT_0015c000)[uVar4] = 3;
  if (DAT_001dd870 == 0x40000) {
    FUN_0010f28e();
  }
  if (DAT_001dd01c != 0) {
    local_18 = (char *)FUN_0010f40a(&DAT_001dd060);
    do {
      uVar4 = (ulong)DAT_001dd870;
      DAT_001dd870 = DAT_001dd870 + 1;
      (&DAT_0015c000)[uVar4] = *local_18;
      if (DAT_001dd870 == 0x40000) {
        FUN_0010f28e();
      }
      pcVar1 = local_18 + 1;
      cVar2 = *local_18;
      local_18 = pcVar1;
    } while (cVar2 != '\0');
  }
  DAT_001e1698 = (ulong)DAT_001dd870;
  FUN_00104a9e(DAT_001190a0);
  if ((DAT_001dd020 != -1) && (DAT_001dd040 != DAT_001dd020)) {
    fprintf(stderr,"%s: %s: file size changed while zipping\n",DAT_001dd010,&DAT_001dd060);
  }
  if (DAT_001dd870 < 0x3fffe) {
    uVar3 = FUN_0010f05f();
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = uVar3;
    FUN_0010f05f();
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = extraout_AH;
  }
  else {
    uVar3 = FUN_0010f05f();
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = uVar3;
    if (DAT_001dd870 == 0x40000) {
      FUN_0010f28e();
    }
    FUN_0010f05f();
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = extraout_AH_00;
    if (DAT_001dd870 == 0x40000) {
      FUN_0010f28e();
    }
  }
  if (DAT_001dd870 < 0x3fffe) {
    FUN_0010f05f();
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = extraout_var;
    FUN_0010f05f();
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = extraout_var_01;
  }
  else {
    FUN_0010f05f();
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = extraout_var_00;
    if (DAT_001dd870 == 0x40000) {
      FUN_0010f28e();
    }
    FUN_0010f05f();
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = extraout_var_02;
    if (DAT_001dd870 == 0x40000) {
      FUN_0010f28e();
    }
  }
  if (DAT_001dd870 < 0x3fffe) {
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = (char)DAT_001dd040;
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = (char)((ulong)DAT_001dd040 >> 8);
  }
  else {
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = (char)DAT_001dd040;
    if (DAT_001dd870 == 0x40000) {
      FUN_0010f28e();
    }
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = (char)((ulong)DAT_001dd040 >> 8);
    if (DAT_001dd870 == 0x40000) {
      FUN_0010f28e();
    }
  }
  uVar3 = (undefined1)((ulong)DAT_001dd040 >> 0x10);
  if (DAT_001dd870 < 0x3fffe) {
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = uVar3;
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = (char)((ulong)DAT_001dd040 >> 0x18);
  }
  else {
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = uVar3;
    if (DAT_001dd870 == 0x40000) {
      FUN_0010f28e();
    }
    uVar4 = (ulong)DAT_001dd870;
    DAT_001dd870 = DAT_001dd870 + 1;
    (&DAT_0015c000)[uVar4] = (char)((ulong)DAT_001dd040 >> 0x18);
    if (DAT_001dd870 == 0x40000) {
      FUN_0010f28e();
    }
  }
  DAT_001e1698 = DAT_001e1698 + 8;
  FUN_0010f28e();
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return 0;
}




// Function: file_read @ 0x101d7

uint file_read(undefined8 param_1,undefined4 param_2)

{
  uint uVar1;
  
  uVar1 = FUN_0010f194(DAT_001dd860,param_1,param_2);
  if (uVar1 != 0) {
    if (uVar1 == 0xffffffff) {
      FUN_0010f6c7();
    }
    FUN_0010efda(param_1,uVar1);
    DAT_001dd040 = DAT_001dd040 + (ulong)uVar1;
  }
  return uVar1;
}




// Function: last_component @ 0x10244

char * last_component(char *param_1)

{
  bool bVar1;
  char *local_18;
  char *local_10;
  
  bVar1 = false;
  for (local_18 = param_1; *local_18 == '/'; local_18 = local_18 + 1) {
  }
  for (local_10 = local_18; *local_10 != '\0'; local_10 = local_10 + 1) {
    if (*local_10 == '/') {
      bVar1 = true;
    }
    else if (bVar1) {
      local_18 = local_10;
      bVar1 = false;
    }
  }
  return local_18;
}




// Function: base_len @ 0x102b1

ulong base_len(char *param_1)

{
  ulong local_18;
  
  for (local_18 = strlen(param_1); (1 < local_18 && (param_1[local_18 - 1] == '/'));
      local_18 = local_18 - 1) {
  }
  return local_18;
}




// Function: rpl_fcntl @ 0x103bd

/* WARNING: Removing unreachable block (ram,0x0011065b) */
/* WARNING: Removing unreachable block (ram,0x00110511) */
/* WARNING: Removing unreachable block (ram,0x001106cb) */
/* WARNING: Removing unreachable block (ram,0x001104a9) */
/* WARNING: Heritage AFTER dead removal. Example location: s0xffffffffffffff58 : 0x001104bb */
/* WARNING: Restarted to delay deadcode elimination for space: stack */

int rpl_fcntl(int param_1,int param_2)

{
  long lVar1;
  ulong uVar2;
  long in_FS_OFFSET;
  int local_f0;
  ulong local_a8;
  
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 == 0) {
    local_f0 = FUN_0011072f(param_1,(undefined4)local_a8);
    goto LAB_00110710;
  }
  if (param_2 == 0x406) {
    local_f0 = FUN_00110760(param_1,(undefined4)local_a8);
    goto LAB_00110710;
  }
  if (param_2 < 0xc) {
    if (-1 < param_2) {
      uVar2 = 1L << ((byte)param_2 & 0x3f);
      if ((uVar2 & 0x515) != 0) {
LAB_0011066d:
        local_f0 = fcntl(param_1,param_2,local_a8 & 0xffffffff);
        goto LAB_00110710;
      }
      uVar2 = uVar2 & 0xa0a;
joined_r0x001105a2:
      if (uVar2 != 0) {
        local_f0 = fcntl(param_1,param_2);
        goto LAB_00110710;
      }
    }
  }
  else if ((param_2 < 0x40b) && (0x3ff < param_2)) {
    uVar2 = 1L << ((byte)param_2 & 0x3f);
    if ((uVar2 & 0x2c5) != 0) goto LAB_0011066d;
    uVar2 = uVar2 & 0x502;
    goto joined_r0x001105a2;
  }
  local_f0 = fcntl(param_1,param_2,local_a8);
LAB_00110710:
  if (lVar1 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_f0;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: rpl_fcntl_DUPFD @ 0x1072f

int rpl_fcntl_DUPFD(int param_1,uint param_2)

{
  int iVar1;
  
  iVar1 = fcntl(param_1,0,(ulong)param_2);
  return iVar1;
}




// Function: rpl_fcntl_DUPFD_CLOEXEC @ 0x10760

int rpl_fcntl_DUPFD_CLOEXEC(int param_1,uint param_2)

{
  uint uVar1;
  int iVar2;
  int *piVar3;
  int local_14;
  
  if (DAT_001e16a0 < 0) {
    local_14 = FUN_0011072f(param_1,param_2);
  }
  else {
    local_14 = fcntl(param_1,0x406,(ulong)param_2);
    if ((local_14 < 0) && (piVar3 = __errno_location(), *piVar3 == 0x16)) {
      local_14 = FUN_0011072f(param_1,param_2);
      if (-1 < local_14) {
        DAT_001e16a0 = -1;
      }
    }
    else {
      DAT_001e16a0 = 1;
    }
  }
  if ((-1 < local_14) && (DAT_001e16a0 == -1)) {
    uVar1 = fcntl(local_14,1);
    if ((-1 < (int)uVar1) && (iVar2 = fcntl(local_14,2,(ulong)(uVar1 | 1)), iVar2 != -1)) {
      return local_14;
    }
    piVar3 = __errno_location();
    iVar2 = *piVar3;
    close(local_14);
    piVar3 = __errno_location();
    *piVar3 = iVar2;
    local_14 = -1;
  }
  return local_14;
}




// Function: open_safer @ 0x10863

/* WARNING: Removing unreachable block (ram,0x00110943) */
/* WARNING: Heritage AFTER dead removal. Example location: s0xffffffffffffff58 : 0x00110955 */
/* WARNING: Restarted to delay deadcode elimination for space: stack */

void open_safer(char *param_1,uint param_2)

{
  long lVar1;
  int iVar2;
  long in_FS_OFFSET;
  uint local_dc;
  uint local_a8;
  
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  local_dc = 0;
  if ((param_2 & 0x40) != 0) {
    local_dc = local_a8;
  }
  iVar2 = open(param_1,param_2,(ulong)local_dc);
  FUN_001110e3(iVar2);
  if (lVar1 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: clear_ungetc_buffer_preserving_position @ 0x1099f

void clear_ungetc_buffer_preserving_position(uint *param_1)

{
  if ((*param_1 & 0x100) != 0) {
    FUN_00110a26(param_1,0,1);
  }
  return;
}




// Function: rpl_fseeko @ 0x10a26

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
  }
  else {
    iVar1 = fseeko(param_1,param_2,param_3);
  }
  return iVar1;
}




// Function: openat_safer @ 0x10ae4

/* WARNING: Removing unreachable block (ram,0x00110bc3) */
/* WARNING: Heritage AFTER dead removal. Example location: s0xffffffffffffff60 : 0x00110bd5 */
/* WARNING: Restarted to delay deadcode elimination for space: stack */

void openat_safer(int param_1,char *param_2,uint param_3)

{
  long lVar1;
  int iVar2;
  long in_FS_OFFSET;
  uint local_dc;
  uint local_a0;
  
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  local_dc = 0;
  if ((param_3 & 0x40) != 0) {
    local_dc = local_a0;
  }
  iVar2 = openat(param_1,param_2,param_3,(ulong)local_dc);
  FUN_001110e3(iVar2);
  if (lVar1 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: direntry_cmp_name @ 0x10c22

void direntry_cmp_name(undefined8 *param_1,undefined8 *param_2)

{
  strcmp((char *)*param_1,(char *)*param_2);
  return;
}




// Function: streamsavedir @ 0x10c61

void * streamsavedir(DIR *param_1,uint param_2)

{
  undefined8 *puVar1;
  int *piVar2;
  undefined8 uVar3;
  char *pcVar4;
  void *pvVar5;
  long lVar6;
  long in_FS_OFFSET;
  char *local_80;
  size_t local_78;
  void *local_70;
  void *local_68;
  size_t local_60;
  char *local_58;
  long local_50;
  __compar_fn_t local_48;
  dirent *local_40;
  char *local_38;
  size_t local_30;
  char *local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_70 = (void *)0x0;
  local_80 = (char *)0x0;
  local_68 = (void *)0x0;
  local_78 = 0;
  local_60 = 0;
  local_58 = (char *)0x0;
  local_48 = *(__compar_fn_t *)(&DAT_00118b70 + (ulong)param_2 * 8);
  if (param_1 == (DIR *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    while( true ) {
      piVar2 = __errno_location();
      *piVar2 = 0;
      local_40 = readdir(param_1);
      if (local_40 == (dirent *)0x0) break;
      local_38 = local_40->d_name;
      if (*local_38 == '.') {
        if (local_40->d_name[1] == '.') {
          lVar6 = 2;
        }
        else {
          lVar6 = 1;
        }
      }
      else {
        lVar6 = 0;
      }
      if (local_38[lVar6] != '\0') {
        local_30 = strlen(local_40->d_name);
        local_30 = local_30 + 1;
        if (local_48 == (__compar_fn_t)0x0) {
          if ((long)local_80 - (long)local_58 <= (long)local_30) {
            local_70 = (void *)FUN_0011204c(local_70,&local_80,
                                            local_30 - ((long)local_80 - (long)local_58),
                                            0x7ffffffffffffffe,1);
          }
          memcpy(local_58 + (long)local_70,local_38,local_30);
        }
        else {
          if (local_60 == local_78) {
            local_68 = (void *)FUN_0011204c(local_68,&local_78,1,0xffffffffffffffff,8);
          }
          puVar1 = (undefined8 *)(local_60 * 8 + (long)local_68);
          uVar3 = FUN_00112377(local_38);
          *puVar1 = uVar3;
          local_60 = local_60 + 1;
        }
        local_58 = local_58 + local_30;
      }
    }
    piVar2 = __errno_location();
    if (*piVar2 == 0) {
      if (local_48 == (__compar_fn_t)0x0) {
        if (local_58 == local_80) {
          local_70 = (void *)FUN_00111e25(local_70,local_58 + 1);
        }
      }
      else {
        if (local_60 != 0) {
          qsort(local_68,local_60,8,local_48);
        }
        local_70 = (void *)FUN_00111d96(local_58 + 1);
        local_58 = (char *)0x0;
        for (local_50 = 0; local_50 < (long)local_60; local_50 = local_50 + 1) {
          local_28 = (char *)((long)local_70 + (long)local_58);
          pcVar4 = stpcpy(local_28,*(char **)((long)local_68 + local_50 * 8));
          local_58 = pcVar4 + (long)(local_58 + (1 - (long)local_28));
          free(*(void **)((long)local_68 + local_50 * 8));
        }
        free(local_68);
      }
      *(char *)((long)local_70 + (long)local_58) = '\0';
      pvVar5 = local_70;
    }
    else {
      free(local_68);
      free(local_70);
      pvVar5 = (void *)0x0;
    }
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return pvVar5;
}




// Function: savedir @ 0x10f97

void * savedir(undefined8 param_1,undefined4 param_2)

{
  int iVar1;
  DIR *__dirp;
  void *__ptr;
  
  __dirp = (DIR *)FUN_001123f7(param_1);
  if (__dirp == (DIR *)0x0) {
    __ptr = (void *)0x0;
  }
  else {
    __ptr = (void *)FUN_00110c61(__dirp,param_2);
    iVar1 = closedir(__dirp);
    if (iVar1 != 0) {
      free(__ptr);
      __ptr = (void *)0x0;
    }
  }
  return __ptr;
}




// Function: fd_safer @ 0x110e3

int fd_safer(int param_1)

{
  int iVar1;
  int *piVar2;
  undefined4 local_1c;
  
  local_1c = param_1;
  if ((-1 < param_1) && (param_1 < 3)) {
    local_1c = FUN_00112614(param_1);
    piVar2 = __errno_location();
    iVar1 = *piVar2;
    close(param_1);
    piVar2 = __errno_location();
    *piVar2 = iVar1;
  }
  return local_1c;
}




// Function: validate_timespec @ 0x11134

int validate_timespec(undefined8 *param_1)

{
  int *piVar1;
  int local_10;
  int local_c;
  
  local_10 = 0;
  local_c = 0;
  if ((((param_1[1] == 0x3fffffff) || (param_1[1] == 0x3ffffffe)) ||
      ((-1 < (long)param_1[1] && ((long)param_1[1] < 1000000000)))) &&
     (((param_1[3] == 0x3fffffff || (param_1[3] == 0x3ffffffe)) ||
      ((-1 < (long)param_1[3] && ((long)param_1[3] < 1000000000)))))) {
    if ((param_1[1] == 0x3fffffff) || (param_1[1] == 0x3ffffffe)) {
      *param_1 = 0;
      local_10 = 1;
      if (param_1[1] == 0x3ffffffe) {
        local_c = 1;
      }
    }
    if ((param_1[3] == 0x3fffffff) || (param_1[3] == 0x3ffffffe)) {
      param_1[2] = 0;
      local_10 = 1;
      if (param_1[3] == 0x3ffffffe) {
        local_c = local_c + 1;
      }
    }
    local_10 = local_10 + (uint)(local_c == 1);
  }
  else {
    piVar1 = __errno_location();
    *piVar1 = 0x16;
    local_10 = -1;
  }
  return local_10;
}




// Function: update_timespec @ 0x1129e

undefined8 update_timespec(undefined8 param_1,long *param_2)

{
  undefined1 (*pauVar1) [16];
  undefined8 uVar2;
  undefined1 auVar3 [16];
  
  pauVar1 = (undefined1 (*) [16])*param_2;
  if ((*(long *)(*pauVar1 + 8) == 0x3ffffffe) && (*(long *)(pauVar1[1] + 8) == 0x3ffffffe)) {
    uVar2 = 1;
  }
  else if ((*(long *)(*pauVar1 + 8) == 0x3fffffff) && (*(long *)(pauVar1[1] + 8) == 0x3fffffff)) {
    *param_2 = 0;
    uVar2 = 0;
  }
  else {
    if (*(long *)(*pauVar1 + 8) == 0x3ffffffe) {
      auVar3 = FUN_0011105b(param_1);
      *pauVar1 = auVar3;
    }
    else if (*(long *)(*pauVar1 + 8) == 0x3fffffff) {
      FUN_001124c4(pauVar1);
    }
    if (*(long *)(pauVar1[1] + 8) == 0x3ffffffe) {
      auVar3 = FUN_0011108f(param_1);
      pauVar1[1] = auVar3;
    }
    else if (*(long *)(pauVar1[1] + 8) == 0x3fffffff) {
      FUN_001124c4(pauVar1 + 1);
    }
    uVar2 = 0;
  }
  return uVar2;
}




// Function: fdutimens @ 0x113ca

int fdutimens(int param_1,char *param_2,__time_t *param_3)

{
  timespec *ptVar1;
  bool bVar2;
  bool bVar3;
  char cVar4;
  int iVar5;
  int *piVar6;
  long lVar7;
  long in_FS_OFFSET;
  timespec tVar8;
  int local_148;
  timespec *local_140;
  timeval *local_138;
  timeval *local_130;
  long local_128;
  long local_120;
  timespec local_118;
  __time_t local_108;
  __time_t local_100;
  timeval local_f8;
  __time_t local_e8;
  long local_e0;
  timeval local_d8;
  __time_t local_c8;
  __suseconds_t local_c0;
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_3 == (__time_t *)0x0) {
    local_140 = (timespec *)0x0;
  }
  else {
    local_140 = &local_118;
  }
  local_148 = 0;
  if (local_140 != (timespec *)0x0) {
    local_118.tv_nsec = param_3[1];
    local_118.tv_sec = *param_3;
    local_100 = param_3[3];
    local_108 = param_3[2];
    local_148 = FUN_00111134(local_140);
  }
  if (local_148 < 0) {
    iVar5 = -1;
    goto LAB_001119f8;
  }
  if ((param_1 < 0) && (param_2 == (char *)0x0)) {
    piVar6 = __errno_location();
    *piVar6 = 9;
    iVar5 = -1;
    goto LAB_001119f8;
  }
  if (-1 < DAT_001e16a4) {
    if (local_148 == 2) {
      if (param_1 < 0) {
        iVar5 = stat(param_2,&local_b8);
      }
      else {
        iVar5 = fstat(param_1,&local_b8);
      }
      ptVar1 = local_140;
      if (iVar5 != 0) {
        iVar5 = -1;
        goto LAB_001119f8;
      }
      if (local_140->tv_nsec == 0x3ffffffe) {
        tVar8 = (timespec)FUN_0011105b(&local_b8);
        *ptVar1 = tVar8;
      }
      else if (local_140[1].tv_nsec == 0x3ffffffe) {
        ptVar1 = local_140 + 1;
        tVar8 = (timespec)FUN_0011108f(&local_b8);
        *ptVar1 = tVar8;
      }
      local_148 = 3;
    }
    if (param_1 < 0) {
      iVar5 = utimensat(-100,param_2,local_140,0);
      if (0 < iVar5) {
        piVar6 = __errno_location();
        *piVar6 = 0x26;
      }
      if (iVar5 != 0) {
        piVar6 = __errno_location();
        if (*piVar6 == 0x26) goto LAB_0011160a;
      }
      DAT_001e16a4 = 1;
      goto LAB_001119f8;
    }
LAB_0011160a:
    if (-1 < param_1) {
      iVar5 = futimens(param_1,local_140);
      if (0 < iVar5) {
        piVar6 = __errno_location();
        *piVar6 = 0x26;
      }
      if (iVar5 != 0) {
        piVar6 = __errno_location();
        if (*piVar6 == 0x26) goto LAB_0011166e;
      }
      DAT_001e16a4 = 1;
      goto LAB_001119f8;
    }
  }
LAB_0011166e:
  DAT_001e16a4 = -1;
  DAT_001e16a8 = 0xffffffff;
  if (local_148 != 0) {
    if (local_148 != 3) {
      if (param_1 < 0) {
        iVar5 = stat(param_2,&local_b8);
      }
      else {
        iVar5 = fstat(param_1,&local_b8);
      }
      if (iVar5 != 0) {
        iVar5 = -1;
        goto LAB_001119f8;
      }
    }
    if (local_140 != (timespec *)0x0) {
      cVar4 = FUN_0011129e(&local_b8,&local_140);
      if (cVar4 != '\0') {
        iVar5 = 0;
        goto LAB_001119f8;
      }
    }
  }
  if (local_140 == (timespec *)0x0) {
    local_138 = (timeval *)0x0;
  }
  else {
    local_f8.tv_sec = local_140->tv_sec;
    local_f8.tv_usec = local_140->tv_nsec / 1000;
    local_e8 = local_140[1].tv_sec;
    local_e0 = local_140[1].tv_nsec / 1000;
    local_138 = &local_f8;
  }
  if (param_1 < 0) {
    iVar5 = futimesat(-100,param_2,local_138);
  }
  else {
    iVar5 = futimesat(param_1,(char *)0x0,local_138);
    if (iVar5 == 0) {
      if (local_138 != (timeval *)0x0) {
        bVar2 = 499999 < local_138->tv_usec;
        bVar3 = 499999 < local_138[1].tv_usec;
        if (bVar2 || bVar3) {
          iVar5 = fstat(param_1,&local_b8);
          if (iVar5 == 0) {
            local_128 = local_b8.st_atim.tv_sec - local_138->tv_sec;
            local_120 = local_b8.st_mtim.tv_sec - local_138[1].tv_sec;
            local_130 = (timeval *)0x0;
            local_d8.tv_usec = local_138->tv_usec;
            local_d8.tv_sec = local_138->tv_sec;
            local_c0 = local_138[1].tv_usec;
            local_c8 = local_138[1].tv_sec;
            if ((bVar2) && (local_128 == 1)) {
              lVar7 = FUN_00111006(&local_b8);
              if (lVar7 == 0) {
                local_130 = &local_d8;
                local_d8.tv_usec = 0;
              }
            }
            if ((bVar3) && (local_120 == 1)) {
              lVar7 = FUN_00111032(&local_b8);
              if (lVar7 == 0) {
                local_130 = &local_d8;
                local_c0 = 0;
              }
            }
            if (local_130 != (timeval *)0x0) {
              futimesat(param_1,(char *)0x0,local_130);
            }
          }
        }
      }
      iVar5 = 0;
    }
    else if (param_2 == (char *)0x0) {
      iVar5 = -1;
    }
    else {
      iVar5 = utimes(param_2,local_138);
    }
  }
LAB_001119f8:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return iVar5;
}




// Function: utimens @ 0x11a12

void utimens(undefined8 param_1,undefined8 param_2)

{
  FUN_001113ca(0xffffffff,param_1,param_2);
  return;
}




// Function: lutimens @ 0x11a3d

ulong lutimens(char *param_1,__time_t *param_2)

{
  timespec *ptVar1;
  char cVar2;
  int iVar3;
  uint uVar4;
  ulong uVar5;
  int *piVar6;
  long in_FS_OFFSET;
  timespec tVar7;
  int local_e8;
  timespec *local_e0;
  timespec local_d8;
  __time_t local_c8;
  __time_t local_c0;
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 == (__time_t *)0x0) {
    local_e0 = (timespec *)0x0;
  }
  else {
    local_e0 = &local_d8;
  }
  local_e8 = 0;
  if (local_e0 != (timespec *)0x0) {
    local_d8.tv_nsec = param_2[1];
    local_d8.tv_sec = *param_2;
    local_c0 = param_2[3];
    local_c8 = param_2[2];
    local_e8 = FUN_00111134(local_e0);
  }
  if (local_e8 < 0) {
    uVar5 = 0xffffffff;
    goto LAB_00111d03;
  }
  if (DAT_001e16a8 < 0) {
LAB_00111c24:
    DAT_001e16a8 = -1;
    if (local_e8 != 0) {
      if (local_e8 != 3) {
        iVar3 = lstat(param_1,&local_b8);
        if (iVar3 != 0) {
          uVar5 = 0xffffffff;
          goto LAB_00111d03;
        }
      }
      if (local_e0 != (timespec *)0x0) {
        cVar2 = FUN_0011129e(&local_b8,&local_e0);
        if (cVar2 != '\0') {
          uVar5 = 0;
          goto LAB_00111d03;
        }
      }
    }
    if (local_e8 == 0) {
      iVar3 = lstat(param_1,&local_b8);
      if (iVar3 != 0) {
        uVar5 = 0xffffffff;
        goto LAB_00111d03;
      }
    }
    if ((local_b8.st_mode & 0xf000) == 0xa000) {
      piVar6 = __errno_location();
      *piVar6 = 0x26;
      uVar5 = 0xffffffff;
    }
    else {
      uVar5 = FUN_001113ca(0xffffffff,param_1,local_e0);
    }
  }
  else {
    if (local_e8 == 2) {
      iVar3 = lstat(param_1,&local_b8);
      ptVar1 = local_e0;
      if (iVar3 != 0) {
        uVar5 = 0xffffffff;
        goto LAB_00111d03;
      }
      if (local_e0->tv_nsec == 0x3ffffffe) {
        tVar7 = (timespec)FUN_0011105b(&local_b8);
        *ptVar1 = tVar7;
      }
      else if (local_e0[1].tv_nsec == 0x3ffffffe) {
        ptVar1 = local_e0 + 1;
        tVar7 = (timespec)FUN_0011108f(&local_b8);
        *ptVar1 = tVar7;
      }
      local_e8 = 3;
    }
    uVar4 = utimensat(-100,param_1,local_e0,0x100);
    if (0 < (int)uVar4) {
      piVar6 = __errno_location();
      *piVar6 = 0x26;
    }
    if (uVar4 != 0) {
      piVar6 = __errno_location();
      if (*piVar6 == 0x26) goto LAB_00111c24;
    }
    DAT_001e16a4 = 1;
    DAT_001e16a8 = 1;
    uVar5 = (ulong)uVar4;
  }
LAB_00111d03:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar5;
}




// Function: nonnull @ 0x11d4e

long nonnull(long param_1)

{
  if (param_1 == 0) {
    FUN_0010f634();
  }
  return param_1;
}




// Function: xmalloc @ 0x11d70

void xmalloc(size_t param_1)

{
  void *pvVar1;
  
  pvVar1 = malloc(param_1);
  FUN_00111d4e(pvVar1);
  return;
}




// Function: ximalloc @ 0x11d96

void ximalloc(undefined8 param_1)

{
  undefined8 uVar1;
  
  uVar1 = FUN_00112547(param_1);
  FUN_00111d4e(uVar1);
  return;
}




// Function: xcharalloc @ 0x11dbc

void xcharalloc(undefined8 param_1)

{
  FUN_00111d70(param_1);
  return;
}




// Function: xrealloc @ 0x11dda

void * xrealloc(void *param_1,size_t param_2)

{
  void *pvVar1;
  
  pvVar1 = realloc(param_1,param_2);
  if ((pvVar1 == (void *)0x0) && ((param_1 == (void *)0x0 || (param_2 != 0)))) {
    FUN_0010f634();
  }
  return pvVar1;
}




// Function: xirealloc @ 0x11e25

void xirealloc(undefined8 param_1,undefined8 param_2)

{
  undefined8 uVar1;
  
  uVar1 = FUN_00112565(param_1,param_2);
  FUN_00111d4e(uVar1);
  return;
}




// Function: xreallocarray @ 0x11e56

long xreallocarray(long param_1,long param_2,long param_3)

{
  long lVar1;
  
  lVar1 = reallocarray(param_1,param_2,param_3);
  if ((lVar1 == 0) && ((param_1 == 0 || ((param_2 != 0 && (param_3 != 0)))))) {
    FUN_0010f634();
  }
  return lVar1;
}




// Function: xireallocarray @ 0x11eb0

void xireallocarray(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 uVar1;
  
  uVar1 = FUN_001125c5(param_1,param_2,param_3);
  FUN_00111d4e(uVar1);
  return;
}




// Function: xnmalloc @ 0x11ee9

void xnmalloc(undefined8 param_1,undefined8 param_2)

{
  FUN_00111e56(0,param_1,param_2);
  return;
}




// Function: xinmalloc @ 0x11f14

void xinmalloc(undefined8 param_1,undefined8 param_2)

{
  FUN_00111eb0(0,param_1,param_2);
  return;
}




// Function: x2realloc @ 0x11f3f

void x2realloc(undefined8 param_1,undefined8 param_2)

{
  FUN_00111f6d(param_1,param_2,1);
  return;
}




// Function: x2nrealloc @ 0x11f6d

undefined8 x2nrealloc(long param_1,ulong *param_2,ulong param_3)

{
  long lVar1;
  undefined1 auVar2 [16];
  long lVar3;
  ulong uVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  bool bVar6;
  undefined8 local_18;
  
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  local_18 = *param_2;
  if (param_1 == 0) {
    if (local_18 == 0) {
      auVar2._8_8_ = 0;
      auVar2._0_8_ = param_3;
      lVar3 = SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x80)) / auVar2,0);
      local_18 = lVar3 + (ulong)(lVar3 == 0);
    }
  }
  else {
    uVar4 = (local_18 >> 1) + 1;
    bVar6 = CARRY8(uVar4,local_18);
    local_18 = uVar4 + local_18;
    if (bVar6) {
      FUN_0010f634();
    }
  }
  uVar5 = FUN_00111e56(param_1,local_18,param_3);
  *param_2 = local_18;
  if (lVar1 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: xpalloc @ 0x1204c

undefined8 xpalloc(long param_1,long *param_2,long param_3,long param_4,long param_5)

{
  long lVar1;
  long lVar2;
  long lVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  long local_30;
  long local_28;
  
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  lVar2 = *param_2;
  local_30 = lVar2 + (lVar2 >> 1);
  if (SCARRY8(lVar2,lVar2 >> 1)) {
    local_30 = 0x7fffffffffffffff;
  }
  if ((-1 < param_4) && (param_4 < local_30)) {
    local_30 = param_4;
  }
  local_28 = local_30 * param_5;
  if (SEXT816(local_28) == SEXT816(local_30) * SEXT816(param_5)) {
    if (local_28 < 0x80) {
      lVar3 = 0x80;
    }
    else {
      lVar3 = 0;
    }
  }
  else {
    lVar3 = 0x7fffffffffffffff;
  }
  if (lVar3 != 0) {
    local_30 = lVar3 / param_5;
    local_28 = lVar3 - lVar3 % param_5;
  }
  if (param_1 == 0) {
    *param_2 = 0;
  }
  if ((local_30 - lVar2 < param_3) &&
     ((local_30 = param_3 + lVar2, SCARRY8(param_3,lVar2) ||
      (((-1 < param_4 && (param_4 < local_30)) ||
       (local_28 = local_30 * param_5, SEXT816(local_28) != SEXT816(local_30) * SEXT816(param_5)))))
     )) {
    FUN_0010f634();
  }
  uVar4 = FUN_00111dda(param_1,local_28);
  *param_2 = local_30;
  if (lVar1 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar4;
}




// Function: xzalloc @ 0x12208

void xzalloc(undefined8 param_1)

{
  FUN_0011224e(param_1,1);
  return;
}




// Function: xizalloc @ 0x1222b

void xizalloc(undefined8 param_1)

{
  FUN_0011227f(param_1,1);
  return;
}




// Function: xcalloc @ 0x1224e

void xcalloc(size_t param_1,size_t param_2)

{
  void *pvVar1;
  
  pvVar1 = calloc(param_1,param_2);
  FUN_00111d4e(pvVar1);
  return;
}




// Function: xicalloc @ 0x1227f

void xicalloc(undefined8 param_1,undefined8 param_2)

{
  undefined8 uVar1;
  
  uVar1 = FUN_0011259c(param_1,param_2);
  FUN_00111d4e(uVar1);
  return;
}




// Function: xmemdup @ 0x122b0

void xmemdup(void *param_1,size_t param_2)

{
  void *__dest;
  
  __dest = (void *)FUN_00111d70(param_2);
  memcpy(__dest,param_1,param_2);
  return;
}




// Function: ximemdup @ 0x122e8

void ximemdup(void *param_1,size_t param_2)

{
  void *__dest;
  
  __dest = (void *)FUN_00111d96(param_2);
  memcpy(__dest,param_1,param_2);
  return;
}




// Function: ximemdup0 @ 0x12328

void ximemdup0(void *param_1,size_t param_2)

{
  void *__dest;
  
  __dest = (void *)FUN_00111d96(param_2 + 1);
  *(undefined1 *)((long)__dest + param_2) = 0;
  memcpy(__dest,param_1,param_2);
  return;
}




// Function: xstrdup @ 0x12377

void xstrdup(char *param_1)

{
  size_t sVar1;
  
  sVar1 = strlen(param_1);
  FUN_001122b0(param_1,sVar1 + 1);
  return;
}




// Function: yesno @ 0x123a8

undefined1 yesno(void)

{
  undefined1 uVar1;
  int local_c;
  
  local_c = getchar();
  if ((local_c == 0x79) || (local_c == 0x59)) {
    uVar1 = 1;
  }
  else {
    uVar1 = 0;
  }
  while ((local_c != 10 && (local_c != -1))) {
    local_c = getchar();
  }
  return uVar1;
}




// Function: opendir_safer @ 0x123f7

DIR * opendir_safer(char *param_1)

{
  int iVar1;
  int *piVar2;
  undefined4 local_24;
  undefined8 local_18;
  undefined8 local_10;
  
  local_18 = opendir(param_1);
  if (local_18 != (DIR *)0x0) {
    iVar1 = dirfd(local_18);
    if ((-1 < iVar1) && (iVar1 < 3)) {
      iVar1 = FUN_001103bd(iVar1,0x406,3);
      if (iVar1 < 0) {
        piVar2 = __errno_location();
        local_24 = *piVar2;
        local_10 = (DIR *)0x0;
      }
      else {
        local_10 = fdopendir(iVar1);
        piVar2 = __errno_location();
        local_24 = *piVar2;
        if (local_10 == (DIR *)0x0) {
          close(iVar1);
        }
      }
      closedir(local_18);
      piVar2 = __errno_location();
      *piVar2 = local_24;
      local_18 = local_10;
    }
  }
  return local_18;
}




// Function: gettime @ 0x124c4

void gettime(timespec *param_1)

{
  clock_gettime(0,param_1);
  return;
}




// Function: current_timespec @ 0x124e8

undefined8 current_timespec(void)

{
  long in_FS_OFFSET;
  undefined8 local_28 [3];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_001124c4(local_28);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_28[0];
}




// Function: dup_safer @ 0x12614

void dup_safer(undefined4 param_1)

{
  FUN_001103bd(param_1,0,3);
  return;
}




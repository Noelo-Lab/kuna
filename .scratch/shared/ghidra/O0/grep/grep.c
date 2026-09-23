// Function: dfaerror @ 0x513f

void dfaerror(undefined8 param_1)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  uVar2 = 2;
  error(2,0,&DAT_00129010,param_1);
  uVar1 = gettext("warning: %s");
  error(0,0,uVar1,uVar2);
  return;
}




// Function: dfawarn @ 0x5174

void dfawarn(undefined8 param_1)

{
  undefined8 uVar1;
  
  uVar1 = gettext("warning: %s");
  error(0,0,uVar1,param_1);
  return;
}




// Function: kwsmusts @ 0x51b4

void kwsmusts(undefined8 *param_1)

{
  byte bVar1;
  char *pcVar2;
  undefined8 uVar3;
  size_t sVar4;
  long lVar5;
  undefined1 *__ptr;
  
  pcVar2 = (char *)FUN_0011ae2f(param_1[1]);
  if (pcVar2 != (char *)0x0) {
    uVar3 = FUN_0010f7c8(0);
    *param_1 = uVar3;
    if (*pcVar2 == '\0') {
      sVar4 = strlen(pcVar2 + 3);
      FUN_0010cacb(*param_1,pcVar2 + 3,sVar4);
    }
    else {
      param_1[7] = param_1[7] + 1;
      sVar4 = strlen(pcVar2 + 3);
      lVar5 = (ulong)(byte)pcVar2[2] + (byte)pcVar2[1] + sVar4;
      __ptr = (undefined1 *)FUN_00126b10(lVar5);
      *__ptr = DAT_00133183;
      bVar1 = pcVar2[1];
      *(bool *)(param_1 + 8) = pcVar2[1] != '\0' || *(char *)(param_1 + 8) != '\0';
      memcpy(__ptr + bVar1,pcVar2 + 3,sVar4);
      if (pcVar2[2] != '\0') {
        (__ptr + bVar1)[sVar4] = DAT_00133183;
      }
      FUN_0010cacb(*param_1,__ptr,lVar5);
      free(__ptr);
    }
    FUN_0010d9b5(*param_1);
    FUN_0011bb28(pcVar2);
  }
  return;
}




// Function: possible_backrefs_in_pattern @ 0x5354

undefined8 possible_backrefs_in_pattern(void *param_1,long param_2,char param_3)

{
  int iVar1;
  void *pvVar2;
  void *local_18;
  
  if (param_3 == '\0') {
    iVar1 = 0x80;
  }
  else {
    iVar1 = 0x5c;
  }
  if (-1 < param_2 + -1) {
    pvVar2 = (void *)((long)param_1 + param_2 + -1);
    local_18 = param_1;
    while (local_18 = memchr(local_18,0x5c,(long)pvVar2 - (long)local_18), local_18 != (void *)0x0)
    {
      if (('0' < *(char *)((long)local_18 + 1)) && (*(char *)((long)local_18 + 1) < ':')) {
        return 1;
      }
      if ((iVar1 == *(char *)((long)local_18 + 1)) &&
         (local_18 = (void *)((long)local_18 + 1), local_18 == pvVar2)) {
        return 0;
      }
      local_18 = (void *)((long)local_18 + 1);
    }
  }
  return 0;
}




// Function: regex_compile @ 0x5428

undefined8
regex_compile(long param_1,char *param_2,size_t param_3,long param_4,long param_5,ulong param_6,
            char param_7)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined8 local_90;
  undefined8 local_88;
  char *local_80;
  char *local_78;
  undefined8 local_70;
  re_pattern_buffer local_68;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_68.buffer = (uchar *)0x0;
  local_68.allocated = 0;
  local_88 = 0xff;
  if (DAT_00133180 == '\0' && param_7 == '\0') {
    local_68.fastmap = (char *)FUN_00126b10(0x100);
  }
  else {
    local_68.fastmap = (char *)0x0;
  }
  local_68.translate = (uchar *)0x0;
  if (param_7 == '\0') {
    re_set_syntax(param_6);
  }
  else {
    re_set_syntax(param_6 | 0x2000000);
  }
  local_80 = re_compile_pattern(param_2,param_3,&local_68);
  if (local_80 == (char *)0x0) {
    if (param_7 == '\0') {
      puVar1 = (undefined8 *)(param_4 * 0x40 + *(long *)(param_1 + 0x10));
      *puVar1 = local_68.buffer;
      puVar1[1] = local_68.allocated;
      puVar1[2] = local_68.used;
      puVar1[3] = local_68.syntax;
      puVar1[4] = local_68.fastmap;
      puVar1[5] = local_68.translate;
      puVar1[6] = local_68.re_nsub;
      puVar1[7] = local_68._56_8_;
    }
    else {
      regfree(&local_68);
    }
    uVar2 = 1;
  }
  else {
    free(local_68.fastmap);
    if (param_5 < 0) {
      local_78 = "";
    }
    else {
      local_78 = (char *)FUN_00106a2e(param_5,&local_90);
    }
    if (*local_78 == '\0') {
      error(0,0,&DAT_00129010,local_80);
    }
    else {
      local_70 = local_90;
      error(0,0,"%s:%td: %s",local_78,local_90,local_80);
    }
    uVar2 = 0;
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: GEAcompile @ 0x5661

long GEAcompile(char *param_1,size_t param_2,ulong param_3,char param_4)

{
  bool bVar1;
  bool bVar2;
  size_t sVar3;
  char cVar4;
  char cVar5;
  uint uVar6;
  undefined8 uVar7;
  char *pcVar8;
  long in_FS_OFFSET;
  bool bVar9;
  reg_syntax_t local_d0;
  size_t local_c8;
  char *local_c0;
  long local_a8;
  long local_a0;
  char *local_98;
  char *local_90;
  char *local_88;
  char *local_80;
  size_t local_78;
  long local_70;
  long local_68;
  char *local_60;
  void *local_58;
  long local_50;
  size_t local_48;
  long local_40;
  long local_38;
  size_t local_30;
  undefined8 local_28;
  char *local_20;
  size_t local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_68 = FUN_00126fc8(1,0x48);
  uVar7 = FUN_0011bb47();
  *(undefined8 *)(local_68 + 8) = uVar7;
  local_d0 = param_3;
  if (DAT_00133180 != '\0') {
    local_d0 = param_3 | 0x400000;
  }
  if (DAT_00133183 == '\0') {
    uVar6 = 2;
  }
  else {
    uVar6 = 0;
  }
  FUN_0011bb5b(*(undefined8 *)(local_68 + 8),&DAT_001331a0,local_d0,
               uVar6 | (uint)local_d0 & 0x10 | 0x2c);
  bVar9 = DAT_001331a2 != '\0';
  bVar1 = DAT_001331a0 != '\x01';
  local_60 = param_1 + param_2;
  bVar2 = false;
  local_90 = param_1;
  uVar7 = FUN_00126aea(0x40);
  *(undefined8 *)(local_68 + 0x10) = uVar7;
  *(long *)(local_68 + 0x10) = *(long *)(local_68 + 0x10) + 0x40;
  *(undefined8 *)(local_68 + 0x18) = 0;
  local_a8 = 1;
  local_80 = (char *)0x0;
  local_78 = 0;
  local_a0 = 0;
  local_70 = 0;
  local_88 = param_1;
  do {
    local_58 = rawmemchr(local_90,10);
    local_50 = (long)local_58 - (long)local_90;
    cVar4 = FUN_00105354(local_90,local_50,bVar9 || bVar1);
    if ((cVar4 != '\0') && (local_88 < local_90)) {
      local_48 = (long)local_90 - (long)local_88;
      local_40 = local_48 + (local_78 - local_a0);
      if (0 < local_40) {
        local_80 = (char *)FUN_00126dc6(local_80,&local_a0,local_40,0xffffffffffffffff,1);
      }
      memcpy(local_80 + local_78,local_88,local_48);
      local_78 = local_78 + local_48;
    }
    local_38 = (*(long *)(local_68 + 0x18) - local_a8) + 2;
    if (0 < local_38) {
      uVar7 = FUN_00126dc6(*(long *)(local_68 + 0x10) + -0x40,&local_a8,local_38,0xffffffffffffffff,
                           0x40);
      *(undefined8 *)(local_68 + 0x10) = uVar7;
      *(long *)(local_68 + 0x10) = *(long *)(local_68 + 0x10) + 0x40;
    }
    re_set_syntax(local_d0);
    cVar5 = FUN_00105428(local_68,local_90,local_50,*(undefined8 *)(local_68 + 0x18),local_70,
                         local_d0,cVar4 == '\0');
    if (cVar5 != '\x01') {
      bVar2 = true;
    }
    local_90 = (char *)((long)local_58 + 1);
    local_70 = local_70 + 1;
    if (cVar4 != '\0') {
      *(long *)(local_68 + 0x18) = *(long *)(local_68 + 0x18) + 1;
      local_88 = local_90;
    }
  } while (local_90 <= local_60);
  if (bVar2) {
                    /* WARNING: Subroutine does not return */
    exit(2);
  }
  pcVar8 = local_80;
  sVar3 = local_78;
  if ((local_88 <= local_60) && (pcVar8 = param_1, sVar3 = param_2, param_1 < local_88)) {
    local_30 = (long)local_60 - (long)local_88;
    local_80 = (char *)FUN_00126b9f(local_80,local_78 + local_30);
    memcpy(local_80 + local_78,local_88,local_30);
    pcVar8 = local_80;
    sVar3 = local_78 + local_30;
  }
  local_78 = sVar3;
  local_80 = pcVar8;
  if ((DAT_00133181 == '\0') && (DAT_00133182 == '\0')) {
    local_98 = (char *)0x0;
    local_c8 = param_2;
    local_c0 = param_1;
  }
  else {
    bVar9 = (local_d0 & 0x2000) != 0;
    local_28 = 0x2d;
    local_20 = (char *)FUN_00126b10(param_2 + 0x2d);
    if (DAT_00133182 == '\0') {
      if (bVar9) {
        pcVar8 = "(^|[^[:alnum:]_])(";
      }
      else {
        pcVar8 = "\\(^\\|[^[:alnum:]_]\\)\\(";
      }
    }
    else if (bVar9) {
      pcVar8 = "^(";
    }
    else {
      pcVar8 = "^\\(";
    }
    strcpy(local_20,pcVar8);
    local_18 = strlen(local_20);
    memcpy(local_20 + local_18,param_1,param_2);
    local_18 = local_18 + param_2;
    if (DAT_00133182 == '\0') {
      if (bVar9) {
        pcVar8 = ")([^[:alnum:]_]|$)";
      }
      else {
        pcVar8 = "\\)\\([^[:alnum:]_]\\|$\\)";
      }
    }
    else if (bVar9) {
      pcVar8 = ")$";
    }
    else {
      pcVar8 = "\\)$";
    }
    strcpy(local_20 + local_18,pcVar8);
    local_c8 = strlen(local_20 + local_18);
    local_c8 = local_c8 + local_18;
    local_98 = local_20;
    local_c0 = local_20;
    local_18 = local_c8;
  }
  FUN_00114b84(local_c0,local_c8,*(undefined8 *)(local_68 + 8));
  FUN_001051b4(local_68);
  FUN_0011a490(0,0,*(undefined8 *)(local_68 + 8),1);
  if (local_80 != (char *)0x0) {
    if ((param_4 != '\0') || (cVar4 = FUN_00119e86(*(undefined8 *)(local_68 + 8)), cVar4 != '\x01'))
    {
      *(long *)(local_68 + 0x10) = *(long *)(local_68 + 0x10) + -0x40;
      *(long *)(local_68 + 0x18) = *(long *)(local_68 + 0x18) + 1;
      cVar4 = FUN_00105428(local_68,local_80,local_78,0,0xffffffffffffffff,local_d0,0);
      if (cVar4 != '\x01') {
                    /* WARNING: Subroutine does not return */
        abort();
      }
    }
    if (local_80 != local_c0) {
      free(local_80);
    }
  }
  free(local_98);
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_68;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: EGexecute @ 0x5d23

long EGexecute(long *param_1,char *param_2,long param_3,long *param_4,char *param_5)

{
  char *pcVar1;
  void *pvVar2;
  long lVar3;
  long in_FS_OFFSET;
  char local_bc;
  char local_bb;
  char local_ba;
  char local_b9;
  int local_b8;
  int local_b4;
  char *local_b0;
  long local_a8;
  char *local_a0;
  char *local_98;
  char *local_90;
  char *local_88;
  char *local_80;
  char *local_78;
  char *local_70;
  long local_68;
  char *local_60;
  long *local_58;
  long local_50;
  char *local_48;
  long local_40;
  char *local_38;
  char *local_30;
  long local_28 [3];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_ba = DAT_00133183;
  local_58 = param_1;
  local_50 = FUN_00119da3(param_1[1]);
  local_b9 = FUN_00119dbc(local_58[1]);
  local_48 = param_2 + param_3;
  local_b0 = param_2;
  local_98 = param_2;
LAB_001066d2:
  while( true ) {
    local_a0 = local_98;
    if (local_48 <= local_98) {
      lVar3 = -1;
      goto LAB_00106724;
    }
    local_98 = local_48;
    pcVar1 = param_5;
    if (param_5 == (char *)0x0) break;
LAB_00106255:
    local_90 = pcVar1;
    if (0x80000000 < (long)local_98 - (long)local_a0) {
      FUN_00127122();
    }
    local_80 = local_98;
    local_70 = (char *)0x0;
    for (local_68 = 0; local_68 < local_58[3]; local_68 = local_68 + 1) {
      lVar3 = local_68 * 0x40 + local_58[2];
      *(byte *)(lVar3 + 0x38) = *(byte *)(lVar3 + 0x38) & 0xbf;
      lVar3 = local_68 * 0x40 + local_58[2];
      *(byte *)(lVar3 + 0x38) = *(byte *)(lVar3 + 0x38) & 0x7f | (DAT_00133183 == '\n') << 7;
      local_b4 = re_search((re_pattern_buffer *)(local_58[2] + local_68 * 0x40),local_a0,
                           ((int)local_98 - (int)local_a0) + -1,(int)local_90 - (int)local_a0,
                           ((int)local_98 - (int)local_90) + -1,(re_registers *)(local_58 + 4));
      if (local_b4 < -1) {
        FUN_00127122();
      }
      if (-1 < local_b4) {
        local_78 = (char *)(long)(*(int *)local_58[6] - local_b4);
        local_88 = local_a0 + local_b4;
        if (local_88 <= local_80) {
          if ((param_5 == (char *)0x0) || (DAT_00133181 == '\x01')) {
            if (((DAT_00133182 == '\x01') || (DAT_00133181 == '\x01')) &&
               ((DAT_00133182 == '\0' || (local_78 != local_98 + (-1 - (long)local_90))))) {
              if ((DAT_00133182 != '\x01') && (DAT_00133181 != '\0')) {
                while (local_88 <= local_80) {
                  local_b8 = 0;
                  lVar3 = FUN_0010fbe0(local_88 + (long)local_78,local_98 + -1);
                  if ((lVar3 == 0) &&
                     (lVar3 = FUN_0010fc0e(local_a0,local_88,local_98 + -1), lVar3 == 0))
                  goto LAB_00106645;
                  if (0 < (long)local_78) {
                    local_78 = local_78 + -1;
                    lVar3 = local_68 * 0x40 + local_58[2];
                    *(byte *)(lVar3 + 0x38) = *(byte *)(lVar3 + 0x38) | 0x40;
                    local_b8 = re_match((re_pattern_buffer *)(local_58[2] + local_68 * 0x40),
                                        local_a0,((int)local_88 + (int)local_78) - (int)local_90,
                                        (int)local_88 - (int)local_a0,(re_registers *)(local_58 + 4)
                                       );
                    if (local_b8 < -1) {
                      FUN_00127122();
                    }
                  }
                  if (local_b8 < 1) {
                    if (local_88 == local_98 + -1) break;
                    local_88 = local_88 + 1;
                    lVar3 = local_68 * 0x40 + local_58[2];
                    *(byte *)(lVar3 + 0x38) = *(byte *)(lVar3 + 0x38) & 0xbf;
                    local_b4 = re_search((re_pattern_buffer *)(local_58[2] + local_68 * 0x40),
                                         local_a0,((int)local_98 - (int)local_a0) + -1,
                                         (int)local_88 - (int)local_a0,
                                         ((int)local_98 - (int)local_88) + -1,
                                         (re_registers *)(local_58 + 4));
                    if (local_b4 < 0) {
                      if (-2 < local_b4) break;
                      FUN_00127122();
                    }
                    local_78 = (char *)(long)(*(int *)local_58[6] - local_b4);
                    local_88 = local_a0 + local_b4;
                  }
                  else {
                    local_78 = (char *)(long)local_b8;
                  }
                }
              }
              goto LAB_00106687;
            }
            local_88 = local_90;
            local_78 = local_98 + -(long)local_90;
          }
LAB_00106645:
          if (param_5 == (char *)0x0) goto LAB_001066f6;
          if ((local_88 < local_80) || ((local_88 == local_80 && ((long)local_70 < (long)local_78)))
             ) {
            local_80 = local_88;
            local_70 = local_78;
          }
        }
      }
LAB_00106687:
    }
    if (local_80 < local_98) {
      local_a0 = local_80;
      local_78 = local_70;
LAB_00106708:
      *param_4 = (long)local_78;
      lVar3 = (long)local_a0 - (long)param_2;
LAB_00106724:
      if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
        return lVar3;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
  }
  local_60 = local_a0;
  local_a8 = 0;
  local_bb = '\0';
  local_bc = '\0';
  if (*local_58 == 0) {
LAB_00106088:
    if ((local_50 != 0) && (local_bb != '\x01')) {
      local_30 = (char *)FUN_00119d4d(local_50,local_60,local_98,0,&local_a8,0);
      if ((local_30 == (char *)0x0) || (local_30 == local_98)) goto LAB_001066d2;
      if (local_a8 != 0) {
        pvVar2 = memrchr(param_2,(int)local_ba,(long)local_30 - (long)param_2);
        local_a0 = (char *)((long)pvVar2 + 1);
        local_60 = local_a0;
      }
      pvVar2 = rawmemchr(local_30,(int)local_ba);
      local_98 = (char *)((long)pvVar2 + 1);
      local_a8 = 0;
    }
    local_30 = (char *)FUN_00119d4d(local_58[1],local_60,local_98,0,&local_a8,&local_bc);
    if ((local_30 == (char *)0x0) || (local_30 == local_98)) goto LAB_001066d2;
    if (local_a8 != 0) {
      pvVar2 = memrchr(param_2,(int)local_ba,(long)local_30 - (long)param_2);
      local_a0 = (char *)((long)pvVar2 + 1);
    }
    pvVar2 = rawmemchr(local_30,(int)local_ba);
    local_98 = (char *)((long)pvVar2 + 1);
    if (local_bc == '\x01') {
      local_90 = local_a0;
      pcVar1 = local_90;
      goto LAB_00106255;
    }
  }
  else {
    lVar3 = FUN_0010f585(*local_58,(long)local_a0 - (ulong)*(byte *)(local_58 + 8),
                         local_48 + ((ulong)*(byte *)(local_58 + 8) - (long)local_a0),local_28,1);
    local_40 = lVar3;
    if (lVar3 < 0) goto LAB_00106724;
    local_88 = local_a0 + lVar3;
    local_38 = local_a0;
    pvVar2 = memrchr(param_2,(int)local_ba,(long)local_88 - (long)param_2);
    local_a0 = param_2;
    if (pvVar2 != (void *)0x0) {
      local_a0 = (char *)((long)pvVar2 + 1);
    }
    local_bb = local_28[0] < local_58[7];
    local_60 = local_a0;
    if (((bool)local_bb) || (local_b9 != '\x01')) {
LAB_00105f74:
      pvVar2 = rawmemchr(local_88,(int)local_ba);
      local_98 = (char *)((long)pvVar2 + 1);
    }
    else {
      lVar3 = 0x10;
      if (0xf < (long)local_88 - (long)local_a0) {
        lVar3 = (long)local_88 - (long)local_a0;
      }
      if (lVar3 < (long)local_88 - (long)local_38 >> 2) goto LAB_00105f74;
      lVar3 = 0x10;
      if (0xf < (long)local_88 - (long)local_a0) {
        lVar3 = (long)local_88 - (long)local_a0;
      }
      if (lVar3 < (long)local_48 - (long)local_38 >> 2) {
        lVar3 = (long)local_88 - (long)local_a0;
        if (lVar3 < 0x10) {
          lVar3 = 0x10;
        }
        pvVar2 = rawmemchr(local_38 + lVar3 * 4,(int)local_ba);
        local_98 = (char *)((long)pvVar2 + 1);
      }
      else {
        local_98 = local_48;
      }
    }
    if (local_bb == '\0') goto LAB_00106088;
    if (DAT_001331a2 == '\0' && DAT_001331a0 == '\x01') {
      if (local_b0 < local_a0) {
        local_b0 = local_a0;
      }
      lVar3 = FUN_0010f84a(&local_b0,0,local_88,local_48);
      if (lVar3 != 0) {
        local_60 = local_b0;
        goto LAB_00106088;
      }
    }
  }
LAB_001066f6:
  local_78 = local_98 + -(long)local_a0;
  goto LAB_00106708;
}




// Function: hash_pattern @ 0x6760

ulong hash_pattern(long param_1,ulong param_2)

{
  ulong local_38;
  byte *local_30;
  
  local_38 = 0x3938a0565d85cb03;
  for (local_30 = (byte *)(param_1 + -1 + DAT_00133780); *local_30 != 10; local_30 = local_30 + 1) {
    local_38 = (ulong)*local_30 ^ local_38 * 0x21;
  }
  return local_38 % param_2;
}




// Function: compare_patterns @ 0x67f8

undefined8 compare_patterns(long param_1,long param_2)

{
  char *local_28;
  char *local_20;
  
  local_28 = (char *)(param_1 + -1 + DAT_00133780);
  local_20 = (char *)(param_2 + -1 + DAT_00133780);
  while( true ) {
    if (*local_28 != *local_20) {
      return 0;
    }
    if (*local_28 == '\n') break;
    local_28 = local_28 + 1;
    local_20 = local_20 + 1;
  }
  return 1;
}




// Function: update_patterns @ 0x687b

long update_patterns(long param_1,long param_2,long param_3,long param_4)

{
  int iVar1;
  void *pvVar2;
  size_t __n;
  long *plVar3;
  int local_68;
  void *local_60;
  long local_58;
  void *local_50;
  
  local_60 = (void *)(param_1 + param_2);
  local_58 = 1;
  local_68 = 0;
  for (local_50 = (void *)(param_1 + param_2); local_50 < (void *)(param_1 + param_3);
      local_50 = (void *)((long)local_50 + __n)) {
    pvVar2 = rawmemchr(local_50,10);
    __n = (long)pvVar2 + (1 - (long)local_50);
    memmove(local_60,local_50,__n);
    iVar1 = FUN_001212a6(DAT_00133790,(long)local_60 + (1 - param_1),0);
    if (iVar1 != 0) {
      if (iVar1 < 0) {
        FUN_00127122();
      }
      local_60 = (void *)((long)local_60 + __n);
      if (local_68 == 0) {
        if (DAT_00133778 == DAT_00133770) {
          DAT_00133768 = FUN_00126dc6(DAT_00133768,&DAT_00133770,1,0xffffffffffffffff,0x18);
        }
        plVar3 = (long *)(DAT_00133778 * 0x18 + DAT_00133768);
        DAT_00133778 = DAT_00133778 + 1;
        *plVar3 = DAT_00133788;
        plVar3[1] = param_4;
        plVar3[2] = local_58;
      }
      DAT_00133788 = DAT_00133788 + 1;
    }
    local_58 = local_58 + 1;
    local_68 = iVar1;
  }
  return (long)local_60 - param_1;
}




// Function: pattern_file_name @ 0x6a2e

undefined8 pattern_file_name(long param_1,long *param_2)

{
  long local_10;
  
  for (local_10 = 1;
      (local_10 < DAT_00133778 && (*(long *)(local_10 * 0x18 + DAT_00133768) <= param_1));
      local_10 = local_10 + 1) {
  }
  *param_2 = (param_1 - *(long *)(local_10 * 0x18 + -0x18 + DAT_00133768)) +
             *(long *)(local_10 * 0x18 + DAT_00133768 + -8);
  return *(undefined8 *)(local_10 * 0x18 + DAT_00133768 + -0x10);
}




// Function: clear_asan_poison @ 0x6b00

void clear_asan_poison(void)

{
  return;
}




// Function: asan_poison @ 0x6b0b

void asan_poison(void)

{
  return;
}




// Function: pr_sgr_start @ 0x6b1e

void pr_sgr_start(char *param_1)

{
  if (*param_1 != '\0') {
    FUN_001276e4(PTR_DAT_00133068,param_1);
  }
  return;
}




// Function: pr_sgr_end @ 0x6b52

void pr_sgr_end(char *param_1)

{
  if (*param_1 != '\0') {
    FUN_00127713(PTR_DAT_00133070);
  }
  return;
}




// Function: pr_sgr_start_if @ 0x6b7f

void pr_sgr_start_if(undefined8 param_1)

{
  if (DAT_00133758 != 0) {
    FUN_00106b1e(param_1);
  }
  return;
}




// Function: pr_sgr_end_if @ 0x6ba8

void pr_sgr_end_if(undefined8 param_1)

{
  if (DAT_00133758 != 0) {
    FUN_00106b52(param_1);
  }
  return;
}




// Function: color_cap_mt_fct @ 0x6bd1

void color_cap_mt_fct(void)

{
  PTR_s_01_31_00133030 = PTR_s_01_31_00133028;
  return;
}




// Function: color_cap_rv_fct @ 0x6bea

void color_cap_rv_fct(void)

{
  DAT_00133758 = 0xffffffff;
  return;
}




// Function: color_cap_ne_fct @ 0x6bff

void color_cap_ne_fct(void)

{
  PTR_DAT_00133068 = &DAT_001290e3;
  PTR_DAT_00133070 = &DAT_001290e9;
  return;
}




// Function: putchar_errno @ 0x6c26

void putchar_errno(int param_1)

{
  int iVar1;
  int *piVar2;
  
  iVar1 = putchar_unlocked(param_1);
  if (iVar1 < 0) {
    piVar2 = __errno_location();
    DAT_00133798 = *piVar2;
  }
  return;
}




// Function: fputs_errno @ 0x6c53

void fputs_errno(char *param_1)

{
  int iVar1;
  int *piVar2;
  
  iVar1 = fputs_unlocked(param_1,stdout);
  if (iVar1 < 0) {
    piVar2 = __errno_location();
    DAT_00133798 = *piVar2;
  }
  return;
}




// Function: printf_errno @ 0x6c8d

void printf_errno(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8,
                 char *param_9,undefined8 param_10,undefined8 param_11,undefined8 param_12,
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
  local_d8 = 8;
  local_d4 = 0x30;
  local_d0 = &stack0x00000008;
  local_c8 = local_b8;
  local_b0 = param_10;
  local_a8 = param_11;
  local_a0 = param_12;
  local_98 = param_13;
  local_90 = param_14;
  iVar1 = vfprintf(stdout,param_9,&local_d8);
  if (iVar1 < 0) {
    piVar2 = __errno_location();
    DAT_00133798 = *piVar2;
  }
  if (local_c0 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: fwrite_errno @ 0x6d74

void fwrite_errno(void *param_1,size_t param_2,size_t param_3)

{
  size_t sVar1;
  int *piVar2;
  
  sVar1 = fwrite_unlocked(param_1,param_2,param_3,stdout);
  if (sVar1 != param_3) {
    piVar2 = __errno_location();
    DAT_00133798 = *piVar2;
  }
  return;
}




// Function: fflush_errno @ 0x6dc0

void fflush_errno(void)

{
  int iVar1;
  int *piVar2;
  
  iVar1 = fflush_unlocked(stdout);
  if (iVar1 != 0) {
    piVar2 = __errno_location();
    DAT_00133798 = *piVar2;
  }
  return;
}




// Function: is_device_mode @ 0x6deb

undefined4 is_device_mode(uint param_1)

{
  undefined4 uVar1;
  
  if (((((param_1 & 0xf000) == 0x2000) || ((param_1 & 0xf000) == 0x6000)) ||
      ((param_1 & 0xf000) == 0xc000)) || ((param_1 & 0xf000) == 0x1000)) {
    uVar1 = 1;
  }
  else {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: skip_devices @ 0x6e43

undefined4 skip_devices(byte param_1)

{
  undefined4 uVar1;
  
  if ((DAT_001337cc == 2) || (((param_1 ^ 1) & DAT_001337cc == 0) != 0)) {
    uVar1 = 1;
  }
  else {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: usable_st_size @ 0x6e8a

undefined4 usable_st_size(long param_1)

{
  uint uVar1;
  
  uVar1 = *(uint *)(param_1 + 0x18) & 0xf000;
  return CONCAT31((int3)(uVar1 >> 8),uVar1 == 0x8000);
}




// Function: input_filename @ 0x6eac

long input_filename(void)

{
  if (DAT_001337c0 == 0) {
    DAT_001337c0 = gettext("(standard input)");
  }
  return DAT_001337c0;
}




// Function: suppressible_error @ 0x6edf

void suppressible_error(undefined4 param_1)

{
  undefined8 uVar1;
  
  if (DAT_00133755 != '\x01') {
    uVar1 = FUN_00106eac();
    error(0,param_1,&DAT_001293c5,uVar1);
  }
  DAT_001337c9 = 1;
  return;
}




// Function: clean_up_stdout @ 0x6f2c

void clean_up_stdout(void)

{
  if (DAT_00133798 == 0) {
    FUN_0011143e();
  }
  return;
}




// Function: initialize_unibyte_mask @ 0x6f46

void initialize_unibyte_mask(void)

{
  byte local_11;
  int local_10;
  uint local_c;
  
  local_11 = 0;
  local_10 = 1;
  for (local_c = 1; (int)local_c < 0x100; local_c = local_c + 1) {
    if ((local_11 & local_c) == 0 && (&DAT_001331a3)[(int)local_c] != '\x01') {
      for (; local_10 * 2 <= (int)local_c; local_10 = local_10 << 1) {
      }
      local_11 = local_11 | (byte)local_10;
    }
  }
  DAT_001337e8 = (ulong)local_11 *
                 (SUB168(ZEXT816(0xffffffffffffffff) * ZEXT816(0x8080808080808081),8) >> 7);
  return;
}




// Function: skip_easy_bytes @ 0x6fdd

ulong * skip_easy_bytes(ulong *param_1)

{
  byte bVar1;
  ulong *local_20;
  ulong *local_18;
  
  local_20 = param_1;
  while( true ) {
    if (((ulong)local_20 & 7) == 0) {
      for (local_18 = local_20; (DAT_001337e8 & *local_18) == 0; local_18 = local_18 + 1) {
      }
      local_20 = local_18;
      while (bVar1 = FUN_0010f64d((int)(char)*local_20), (DAT_001337e8 & bVar1) == 0) {
        local_20 = (ulong *)((long)local_20 + 1);
      }
      return local_20;
    }
    bVar1 = FUN_0010f64d((int)(char)*local_20);
    if ((DAT_001337e8 & bVar1) != 0) break;
    local_20 = (ulong *)((long)local_20 + 1);
  }
  return local_20;
}




// Function: buf_has_encoding_errors @ 0x7098

undefined8 buf_has_encoding_errors(long param_1,long param_2)

{
  undefined8 uVar1;
  ulong uVar2;
  long in_FS_OFFSET;
  long local_28;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_001337e8 == 0) {
    uVar1 = 0;
  }
  else {
    local_18 = 0;
    *(undefined1 *)(param_1 + param_2) = 0xff;
    local_28 = param_1;
    while( true ) {
      uVar2 = FUN_00106fdd(local_28);
      if ((ulong)(param_1 + param_2) <= uVar2) break;
      local_28 = FUN_0010f686(uVar2,(param_1 + param_2) - uVar2,&local_18);
      if (local_28 < 0) {
        uVar1 = 1;
        goto LAB_00107156;
      }
      local_28 = uVar2 + local_28;
    }
    uVar1 = 0;
  }
LAB_00107156:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar1;
}




// Function: buf_has_nulls @ 0x716c

bool buf_has_nulls(char *param_1,size_t param_2)

{
  size_t sVar1;
  
  param_1[param_2] = '\0';
  sVar1 = strlen(param_1);
  return sVar1 != param_2;
}




// Function: file_must_have_nulls @ 0x71a6

undefined8 file_must_have_nulls(long param_1,int param_2,long param_3)

{
  char cVar1;
  __off_t _Var2;
  __off_t _Var3;
  int *piVar4;
  __off_t local_18;
  
  if (((DAT_001337d0 != '\x01') && (cVar1 = FUN_00106e8a(param_3), cVar1 != '\0')) &&
     (param_1 < *(long *)(param_3 + 0x30))) {
    local_18 = param_1;
    if ((param_2 == 0) && (local_18 = lseek(0,0,1), local_18 < 0)) {
      return 0;
    }
    _Var2 = lseek(param_2,local_18,4);
    if (-1 < _Var2) {
      _Var3 = lseek(param_2,local_18,0);
      if (_Var3 < 0) {
        piVar4 = __errno_location();
        FUN_00106edf(*piVar4);
      }
      if (_Var2 < *(long *)(param_3 + 0x30)) {
        return 1;
      }
    }
  }
  return 0;
}




// Function: context_length_arg @ 0x728f

void context_length_arg(undefined8 param_1,long *param_2)

{
  uint uVar1;
  undefined8 uVar2;
  
  uVar1 = FUN_00127281(param_1,0,10,param_2,&DAT_001290d2);
  if ((1 < uVar1) || (*param_2 < 0)) {
    uVar2 = gettext("invalid context length argument");
    error(2,0,"%s: %s",param_1,uVar2);
  }
  return;
}




// Function: exclude_options @ 0x7316

undefined8 exclude_options(char param_1)

{
  undefined8 uVar1;
  
  if (param_1 == '\0') {
    uVar1 = 0x50000000;
  }
  else {
    uVar1 = 0x10000000;
  }
  return uVar1;
}




// Function: skipped_file @ 0x7337

undefined4 skipped_file(undefined8 param_1,byte param_2,char param_3)

{
  char cVar1;
  undefined4 uVar2;
  undefined *local_10;
  
  if (param_3 == '\x01') {
    if (DAT_00133010 == 4) {
      return 1;
    }
    if ((param_2 != 0) && (DAT_001337c8 != '\0')) {
      return 0;
    }
    local_10 = &DAT_001337b0;
  }
  else {
    local_10 = &DAT_001337a0;
  }
  if ((*(long *)(local_10 + (ulong)param_2 * 8) == 0) ||
     (cVar1 = FUN_0011cbb6(*(undefined8 *)(local_10 + (ulong)param_2 * 8),param_1), cVar1 == '\0'))
  {
    uVar2 = 0;
  }
  else {
    uVar2 = 1;
  }
  return uVar2;
}




// Function: add_count @ 0x73f5

long add_count(long param_1,long param_2)

{
  long lVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  if (SCARRY8(param_2,param_1)) {
    uVar2 = gettext("input is too large to count");
    error(2,0,uVar2);
  }
  if (lVar1 == *(long *)(in_FS_OFFSET + 0x28)) {
    return param_2 + param_1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: all_zeros @ 0x747d

undefined8 all_zeros(char *param_1,long param_2)

{
  char *local_10;
  
  local_10 = param_1;
  while( true ) {
    if (param_1 + param_2 <= local_10) {
      return 1;
    }
    if (*local_10 != '\0') break;
    local_10 = local_10 + 1;
  }
  return 0;
}




// Function: reset @ 0x74c6

undefined8 reset(int param_1)

{
  int *piVar1;
  
  if ((DAT_001337f0 + 1U) % DAT_00133818 == 0) {
    DAT_00133808 = DAT_001337f0 + 1;
  }
  else {
    DAT_00133808 = (DAT_00133818 - (DAT_001337f0 + 1U) % DAT_00133818) + 1 + DAT_001337f0;
  }
  DAT_00133810 = DAT_00133808;
  *(undefined1 *)(DAT_00133808 + -1) = DAT_00133183;
  DAT_00133800 = param_1;
  if (param_1 == 0) {
    DAT_00133820 = lseek(0,0,1);
  }
  else {
    DAT_00133820 = 0;
  }
  DAT_001337d0 = -(char)(DAT_00133820 >> 0x3f);
  DAT_001337d1 = DAT_001337d0;
  if (DAT_001337d0 != '\0') {
    piVar1 = __errno_location();
    if (*piVar1 != 0x1d) {
      piVar1 = __errno_location();
      FUN_00106edf(*piVar1);
      return 0;
    }
    DAT_00133820 = 0;
  }
  return 1;
}




// Function: fillbuf @ 0x7608

undefined1 fillbuf(long param_1,long param_2)

{
  long lVar1;
  char cVar2;
  long lVar3;
  long lVar4;
  long lVar5;
  size_t __n;
  int *piVar6;
  long lVar7;
  long in_FS_OFFSET;
  undefined1 local_71;
  long local_68;
  void *local_60;
  long local_58;
  long local_50;
  long local_48;
  
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  lVar3 = DAT_00133818 + 8;
  if ((long)DAT_001337f0 + (DAT_001337f8 - (long)DAT_00133810) < lVar3) {
    lVar7 = lVar3 + ((param_1 + DAT_00133818) - DAT_001337f8);
    if (lVar7 < 1) {
      local_60 = DAT_001337f0;
    }
    else {
      local_58 = -1;
      cVar2 = FUN_00106e8a(param_2);
      if ((((cVar2 != '\0') && (lVar4 = *(long *)(param_2 + 0x30) - DAT_00133820, -1 < lVar4)) &&
          (lVar5 = lVar4 + param_1 + lVar3, !SCARRY8(lVar4,param_1 + lVar3))) &&
         (local_58 = lVar5, lVar5 <= DAT_001337f8 + lVar7)) {
        local_58 = DAT_001337f8 + lVar7;
      }
      local_60 = (void *)FUN_00126dc6(0,&DAT_001337f8,lVar7,local_58,1);
    }
    lVar3 = param_1;
    if ((ulong)((long)local_60 + param_1 + 1) % DAT_00133818 != 0) {
      lVar3 = param_1 + (DAT_00133818 - (ulong)((long)local_60 + param_1 + 1) % DAT_00133818);
    }
    local_68 = (long)local_60 + lVar3 + 1;
    __n = param_1 + 1;
    memmove((void *)(local_68 - __n),(void *)((long)DAT_00133810 - __n),__n);
    if (0 < lVar7) {
      free(DAT_001337f0);
      DAT_001337f0 = local_60;
    }
  }
  else {
    local_68 = (long)DAT_00133810;
  }
  DAT_00133808 = local_68 - param_1;
  FUN_00106b00();
  lVar3 = (long)DAT_001337f0 + ((DAT_001337f8 + -8) - local_68);
  lVar7 = lVar3 % (long)DAT_00133818;
  local_71 = 1;
  while( true ) {
    local_50 = FUN_00124c3e(DAT_00133800,local_68,lVar3 - lVar7);
    if (local_50 == -1) {
      local_50 = 0;
      local_71 = 0;
    }
    DAT_00133820 = local_50 + DAT_00133820;
    if ((DAT_00133830 != '\x01' || local_50 == 0) ||
       (cVar2 = FUN_0010747d(local_68,local_50), cVar2 != '\x01')) break;
    DAT_00133838 = FUN_001073f5(DAT_00133838,local_50);
    if (DAT_001337d1 != '\x01') {
      local_48 = lseek(DAT_00133800,DAT_00133820,3);
      if (((local_48 < 0) && (piVar6 = __errno_location(), *piVar6 == 6)) &&
         ((cVar2 = FUN_00106e8a(param_2), cVar2 != '\0' &&
          (DAT_00133820 < *(long *)(param_2 + 0x30))))) {
        local_48 = lseek(DAT_00133800,0,2);
      }
      if (local_48 < 0) {
        DAT_001337d1 = '\x01';
      }
      else {
        DAT_00133838 = FUN_001073f5(DAT_00133838,local_48 - DAT_00133820);
        DAT_00133820 = local_48;
      }
    }
  }
  DAT_00133810 = (void *)(local_68 + local_50);
  memset(DAT_00133810,0,8);
  FUN_00106b0b((long)DAT_00133810 + 8,
               (DAT_001337f8 - ((long)DAT_00133810 - (long)DAT_001337f0)) + -8);
  if (lVar1 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_71;
}




// Function: nlscan @ 0x7a8d

void nlscan(void *param_1)

{
  long local_18;
  void *local_10;
  
  local_18 = 0;
  local_10 = DAT_00133890;
  while ((local_10 < param_1 &&
         (local_10 = memchr(local_10,(int)DAT_00133183,(long)param_1 - (long)local_10),
         local_10 != (void *)0x0))) {
    local_18 = local_18 + 1;
    local_10 = (void *)((long)local_10 + 1);
  }
  DAT_00133838 = FUN_001073f5(DAT_00133838,local_18);
  DAT_00133890 = param_1;
  return;
}




// Function: print_filename @ 0x7b22

void print_filename(void)

{
  undefined8 uVar1;
  
  FUN_00106b7f(PTR_DAT_00133038);
  uVar1 = FUN_00106eac();
  FUN_00106c53(uVar1);
  FUN_00106ba8(PTR_DAT_00133038);
  return;
}




// Function: print_sep @ 0x7b58

void print_sep(char param_1)

{
  FUN_00106b7f(PTR_DAT_00133050);
  FUN_00106c26((int)param_1);
  FUN_00106ba8(PTR_DAT_00133050);
  return;
}




// Function: print_offset @ 0x7b95

void print_offset(undefined8 param_1,undefined8 param_2)

{
  FUN_00106b7f(param_2);
  FUN_00106c8d(&DAT_00129413,DAT_00133760,param_1);
  FUN_00106ba8(param_2);
  return;
}




// Function: print_line_head @ 0x7be4

undefined8 print_line_head(long param_1,long param_2,ulong param_3,char param_4)

{
  undefined1 uVar1;
  char cVar2;
  undefined8 uVar3;
  
  if (DAT_00133840 != 1) {
    uVar1 = *(undefined1 *)(param_1 + param_2);
    cVar2 = FUN_00107098(param_1,param_2);
    *(undefined1 *)(param_2 + param_1) = uVar1;
    if (cVar2 != '\0') {
      DAT_001337ca = 1;
      return 0;
    }
  }
  if (DAT_00133848 != 0) {
    FUN_00107b22();
    if (DAT_0013384c == 0) {
      FUN_00106c26(0);
    }
    else {
      FUN_00107b58((int)param_4);
    }
  }
  if (DAT_00133852 != '\0') {
    if (DAT_00133890 < param_3) {
      FUN_00107a8d(param_1);
      DAT_00133838 = FUN_001073f5(DAT_00133838,1);
      DAT_00133890 = param_3;
    }
    FUN_00107b95(DAT_00133838,PTR_DAT_00133040);
    FUN_00107b58((int)param_4);
  }
  if (DAT_00133853 != '\0') {
    uVar3 = FUN_001073f5(DAT_00133888,param_1 - DAT_00133808);
    FUN_00107b95(uVar3,PTR_DAT_00133048);
    FUN_00107b58((int)param_4);
  }
  if (((DAT_0013375d != '\0') &&
      (DAT_00133853 != '\0' || (DAT_00133852 != '\0' || DAT_00133848 != 0))) && (param_2 != 0)) {
    FUN_00106c26(9);
  }
  return 1;
}




// Function: print_line_middle @ 0x7d91

ulong print_line_middle(ulong param_1,ulong param_2,undefined8 param_3,undefined8 param_4)

{
  ulong uVar1;
  undefined1 uVar2;
  char cVar3;
  long in_FS_OFFSET;
  long local_38;
  ulong local_30;
  ulong local_28;
  long local_20;
  ulong local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_28 = 0;
  local_30 = param_1;
  do {
    if (((param_2 <= local_30) ||
        (local_20 = (*DAT_001337d8)(DAT_001337e0,param_1,param_2 - param_1,&local_38,local_30),
        local_20 < 0)) || (local_18 = param_1 + local_20, local_18 == param_2)) {
      uVar1 = param_2;
      if ((DAT_0013375c == '\0') && (param_2 = local_30, uVar1 = local_30, local_28 != 0)) {
        local_30 = local_28;
        param_2 = local_28;
        uVar1 = local_30;
      }
LAB_00107f71:
      local_30 = uVar1;
      if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
        return param_2;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    if (local_38 == 0) {
      local_38 = 1;
      if (local_28 == 0) {
        local_28 = local_30;
      }
    }
    else {
      if (DAT_0013375c == '\0') {
        FUN_00106b1e(param_3);
        if (local_28 != 0) {
          local_30 = local_28;
          local_28 = 0;
        }
        FUN_00106d74(local_30,1,local_18 - local_30);
      }
      else {
        if (DAT_00133851 == '\0') {
          uVar2 = 0x3a;
        }
        else {
          uVar2 = 0x2d;
        }
        cVar3 = FUN_00107be4(local_18,local_38,param_2,uVar2);
        if (cVar3 != '\x01') {
          param_2 = 0;
          uVar1 = local_30;
          goto LAB_00107f71;
        }
      }
      FUN_00106b7f(param_4);
      FUN_00106d74(local_18,1,local_38);
      FUN_00106ba8(param_4);
      if (DAT_0013375c != '\0') {
        FUN_00106c26((int)DAT_00133183);
      }
    }
    local_30 = local_18 + local_38;
  } while( true );
}




// Function: print_line_tail @ 0x7f87

ulong print_line_tail(ulong param_1,ulong param_2,undefined8 param_3)

{
  int iVar1;
  ulong uVar2;
  long lVar3;
  undefined8 local_20;
  
  if ((param_1 < param_2) && (*(char *)(param_2 - 1) == DAT_00133183)) {
    iVar1 = 1;
  }
  else {
    iVar1 = 0;
  }
  uVar2 = (ulong)iVar1;
  if ((param_1 < param_2 - uVar2) && (*(char *)(param_2 + ~uVar2) == '\r')) {
    iVar1 = 1;
  }
  else {
    iVar1 = 0;
  }
  lVar3 = (param_2 - (uVar2 + (long)iVar1)) - param_1;
  local_20 = param_1;
  if (0 < lVar3) {
    FUN_00106b1e(param_3);
    FUN_00106d74(param_1,1,lVar3);
    local_20 = param_1 + lVar3;
    FUN_00106b52(param_3);
  }
  return local_20;
}




// Function: prline @ 0x806d

void prline(ulong param_1,ulong param_2,char param_3)

{
  byte bVar1;
  ulong uVar2;
  char cVar3;
  undefined8 uVar4;
  bool bVar5;
  ulong local_30;
  char *local_18;
  char *local_10;
  
  if ((DAT_0013375c == '\x01') ||
     (cVar3 = FUN_00107be4(param_1,(param_2 - param_1) + -1,param_2,(int)param_3),
     uVar2 = DAT_00133898, cVar3 == '\x01')) {
    bVar5 = (bool)DAT_00133851 != (param_3 == ':');
    if (DAT_00133758 == 0) {
      local_10 = (char *)0x0;
      local_18 = (char *)0x0;
    }
    else {
      if ((DAT_00133851 == '\0') || (-1 < DAT_00133758)) {
        bVar1 = 0;
      }
      else {
        bVar1 = 1;
      }
      local_18 = PTR_DAT_00133060;
      if ((bool)(bVar1 ^ param_3 == ':')) {
        local_18 = PTR_DAT_00133058;
      }
      local_10 = PTR_s_01_31_00133030;
      if (param_3 == ':') {
        local_10 = PTR_s_01_31_00133028;
      }
    }
    local_30 = param_1;
    if (((DAT_0013375c != '\0') && (bVar5)) ||
       ((DAT_00133758 != 0 && ((*local_18 != '\0' || (*local_10 != '\0')))))) {
      if ((bVar5) &&
         (((DAT_0013375c != '\0' || (*local_10 != '\0')) &&
          (local_30 = FUN_00107d91(param_1,param_2,local_18,local_10), local_30 == 0)))) {
        return;
      }
      if ((DAT_0013375c != '\x01') && (*local_18 != '\0')) {
        local_30 = FUN_00107f87(local_30,param_2,local_18);
      }
    }
    if ((DAT_0013375c != '\x01') && (local_30 < param_2)) {
      FUN_00106d74(local_30,1,param_2 - local_30);
    }
    if (DAT_00133878 != '\0') {
      FUN_00106dc0();
    }
    uVar2 = param_2;
    if (DAT_00133798 != 0) {
      uVar4 = gettext("write error");
      error(2,DAT_00133798,uVar4);
      uVar2 = param_2;
    }
  }
  DAT_00133898 = uVar2;
  return;
}




// Function: prpending @ 0x828e

void prpending(void *param_1)

{
  void *pvVar1;
  
  if (DAT_00133898 == (void *)0x0) {
    DAT_00133898 = DAT_00133808;
  }
  while ((0 < DAT_001338a8 && (DAT_00133898 < param_1))) {
    pvVar1 = rawmemchr(DAT_00133898,(int)DAT_00133183);
    FUN_0010806d(DAT_00133898,(long)pvVar1 + 1,0x2d);
    DAT_001338a8 = DAT_001338a8 + -1;
  }
  return;
}




// Function: prtext @ 0x8326

void prtext(void *param_1,void *param_2)

{
  void *pvVar1;
  char cVar2;
  char *pcVar3;
  void *pvVar4;
  void *local_38;
  long local_30;
  long local_28;
  
  cVar2 = DAT_00133183;
  if ((DAT_00133850 != '\x01') && (0 < DAT_001338a8)) {
    FUN_0010828e(param_1);
  }
  local_38 = param_1;
  if (DAT_00133850 != '\x01') {
    pvVar4 = DAT_00133808;
    if (DAT_00133898 != (void *)0x0) {
      pvVar4 = DAT_00133898;
    }
    for (local_30 = 0; local_30 < DAT_00133858; local_30 = local_30 + 1) {
      if (pvVar4 < local_38) {
        do {
          pvVar1 = (void *)((long)local_38 - 1);
          pcVar3 = (char *)((long)local_38 - 2);
          local_38 = pvVar1;
        } while (cVar2 != *pcVar3);
      }
    }
    if ((((-1 < DAT_00133858) || (-1 < DAT_00133860)) && (DAT_001339c0 != '\0')) &&
       ((local_38 != DAT_00133898 && (PTR_DAT_00133020 != (undefined *)0x0)))) {
      FUN_00106b7f(PTR_DAT_00133050);
      FUN_00106c53(PTR_DAT_00133020);
      FUN_00106ba8(PTR_DAT_00133050);
      FUN_00106c26(10);
    }
    while (local_38 < param_1) {
      pvVar4 = rawmemchr(local_38,(int)cVar2);
      FUN_0010806d(local_38,(void *)((long)pvVar4 + 1),0x2d);
      local_38 = (void *)((long)pvVar4 + 1);
    }
  }
  if (DAT_00133851 == '\0') {
    if (DAT_00133850 != '\x01') {
      FUN_0010806d(param_1,param_2,0x3a);
    }
    local_28 = 1;
    local_38 = param_2;
  }
  else {
    for (local_28 = 0; (local_38 < param_2 && (local_28 < DAT_001338a0)); local_28 = local_28 + 1) {
      pvVar4 = rawmemchr(local_38,(int)cVar2);
      if (DAT_00133850 != '\x01') {
        FUN_0010806d(local_38,(void *)((long)pvVar4 + 1),0x3a);
      }
      local_38 = (void *)((long)pvVar4 + 1);
    }
  }
  DAT_00133828 = DAT_00133820 - (DAT_00133810 - (long)local_38);
  if (DAT_00133850 == '\0') {
    DAT_001338a8 = DAT_00133860;
    if (DAT_00133860 < 0) {
      DAT_001338a8 = 0;
    }
  }
  else {
    DAT_001338a8 = 0;
  }
  DAT_001339c0 = 1;
  DAT_001338a0 = DAT_001338a0 - local_28;
  return;
}




// Function: zap_nuls @ 0x85c1

void zap_nuls(char *param_1,char *param_2,char param_3)

{
  char *pcVar1;
  size_t sVar2;
  char *local_10;
  
  local_10 = param_1;
  if (param_3 != '\0') {
    while( true ) {
      *param_2 = '\0';
      sVar2 = strlen(local_10);
      local_10 = local_10 + sVar2;
      *param_2 = param_3;
      if (local_10 == param_2) break;
      do {
        pcVar1 = local_10 + 1;
        *local_10 = param_3;
        local_10 = pcVar1;
      } while (*pcVar1 == '\0');
    }
  }
  return;
}




// Function: grepbuf @ 0x862e

long grepbuf(ulong param_1,ulong param_2)

{
  int __status;
  long in_FS_OFFSET;
  long local_50;
  ulong local_48;
  long local_40;
  long local_38;
  ulong local_30;
  ulong local_28;
  ulong local_20;
  ulong local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_38 = DAT_001338a0;
  local_48 = param_1;
  do {
    if (param_2 <= local_48) {
LAB_001087c5:
      if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return local_38 - DAT_001338a0;
    }
    local_40 = (*DAT_001337d8)(DAT_001337e0,local_48,param_2 - local_48,&local_50,0);
    if (local_40 < 0) {
      if (DAT_00133851 != '\x01') goto LAB_001087c5;
      local_40 = param_2 - local_48;
      local_50 = 0;
    }
    local_30 = local_48 + local_40;
    local_28 = local_30 + local_50;
    if ((DAT_00133851 != '\x01') && (local_30 == param_2)) goto LAB_001087c5;
    if ((DAT_00133851 != '\x01') || (local_48 < local_30)) {
      local_18 = local_28;
      local_20 = local_30;
      if (DAT_00133851 != '\0') {
        local_18 = local_30;
        local_20 = local_48;
      }
      FUN_00108326(local_20,local_18);
      if ((DAT_001338a0 == 0) || (DAT_001338b0 != '\0')) {
        if (DAT_001338b1 != '\0') {
          __status = DAT_00133080;
          if (DAT_001337c9 == '\0') {
            __status = 0;
          }
                    /* WARNING: Subroutine does not return */
          exit(__status);
        }
        goto LAB_001087c5;
      }
    }
    local_48 = local_28;
  } while( true );
}




// Function: grep @ 0x87e9

long grep(undefined4 param_1,long param_2,undefined1 *param_3)

{
  bool bVar1;
  char cVar2;
  char cVar3;
  char cVar4;
  char cVar5;
  int iVar6;
  int *piVar7;
  char *pcVar8;
  void *pvVar9;
  char *pcVar10;
  long lVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  char local_5e;
  long local_58;
  long local_50;
  long local_48;
  char *local_40;
  char *local_38;
  char *local_30;
  long local_28;
  long local_20;
  
  cVar4 = DAT_001338b0;
  cVar3 = DAT_00133850;
  cVar2 = DAT_00133183;
  local_5e = '\0';
  local_28 = -1;
  cVar5 = FUN_001074c6(param_1,param_2);
  if (cVar5 == '\x01') {
    DAT_00133888 = 0;
    DAT_00133898 = (char *)0x0;
    DAT_00133838 = 0;
    DAT_001338a0 = DAT_00133870;
    DAT_00133828 = 0;
    DAT_001338a8 = 0;
    if ((DAT_00133831 == '\0') || (cVar2 != '\0')) {
      DAT_00133830 = '\0';
    }
    else {
      DAT_00133830 = '\x01';
    }
    DAT_001337ca = '\0';
    local_58 = 0;
    local_48 = 0;
    local_40 = (char *)0x0;
    cVar5 = FUN_00107608(0,param_2);
    if (cVar5 == '\x01') {
      DAT_00133760 = 0;
      if (DAT_0013375d != '\0') {
        cVar5 = FUN_00106e8a(param_2);
        if (cVar5 == '\0') {
          local_20 = 0x7fffffffffffffff;
        }
        else {
          local_20 = *(long *)(param_2 + 0x30);
        }
        if ((DAT_00133852 == '\0') || (local_20 == 0x7fffffffffffffff)) {
          iVar6 = 0;
        }
        else {
          iVar6 = 1;
        }
        local_20 = local_20 + iVar6;
        do {
          DAT_00133760 = DAT_00133760 + 1;
          local_20 = local_20 / 10;
        } while (local_20 != 0);
      }
      bVar1 = true;
      while( true ) {
        if ((((local_28 < 0) && (cVar2 != '\0')) && (DAT_00133840 != 1)) &&
           ((cVar5 = FUN_0010716c(DAT_00133808,(long)DAT_00133810 - (long)DAT_00133808),
            cVar5 != '\0' ||
            ((bVar1 && (cVar5 = FUN_001071a6((long)DAT_00133810 - (long)DAT_00133808,param_1,param_2
                                            ), cVar5 != '\0')))))) {
          if (DAT_00133840 == 2) {
            return 0;
          }
          if (DAT_00133868 != '\x01') {
            DAT_00133850 = '\x01';
            DAT_001338b0 = '\x01';
          }
          local_28 = local_58;
          local_5e = cVar2;
          DAT_00133830 = DAT_00133831;
        }
        DAT_00133890 = DAT_00133808;
        if (DAT_00133898 != (char *)0x0) {
          DAT_00133898 = DAT_00133808;
        }
        pcVar8 = DAT_00133808 + (long)local_40;
        if (pcVar8 == DAT_00133810) {
          *param_3 = 1;
          if (local_48 != 0) {
            pcVar8 = DAT_00133810 + 1;
            *DAT_00133810 = cVar2;
            DAT_00133810 = pcVar8;
            if (DAT_001338a0 != 0) {
              lVar11 = FUN_0010862e(DAT_00133808 + ((long)local_40 - local_48),DAT_00133810);
              local_58 = local_58 + lVar11;
            }
            if (DAT_001338a8 != 0) {
              FUN_0010828e(DAT_00133810);
            }
          }
          goto LAB_00108d91;
        }
        FUN_001085c1(pcVar8,DAT_00133810,(int)local_5e);
        cVar5 = pcVar8[-1];
        pcVar8[-1] = cVar2;
        pvVar9 = memrchr(pcVar8 + -1,(int)cVar2,(size_t)(DAT_00133810 + (1 - (long)pcVar8)));
        local_30 = (char *)((long)pvVar9 + 1);
        pcVar8[-1] = cVar5;
        if (local_30 == pcVar8) {
          local_30 = pcVar8 + -local_48;
        }
        lVar11 = -local_48;
        local_48 = (long)DAT_00133810 - (long)local_30;
        if (pcVar8 + lVar11 < local_30) {
          if (DAT_001338a0 != 0) {
            lVar11 = FUN_0010862e(pcVar8 + lVar11,local_30);
            local_58 = local_58 + lVar11;
          }
          if (DAT_001338a8 != 0) {
            FUN_0010828e(local_30);
          }
          if ((DAT_001338a0 == 0) && (DAT_001338a8 == 0)) goto LAB_00108d91;
          if (DAT_001338b0 != '\0') {
            lVar11 = local_28;
            if (local_28 < 0) {
              lVar11 = 0;
            }
            if (lVar11 < local_58) goto LAB_00108d91;
          }
        }
        local_50 = 0;
        local_38 = local_30;
        while (((local_50 < DAT_00133858 && (DAT_00133808 < local_38)) && (local_38 != DAT_00133898)
               )) {
          local_50 = local_50 + 1;
          do {
            pcVar8 = local_38 + -1;
            pcVar10 = local_38 + -2;
            local_38 = pcVar8;
          } while (cVar2 != *pcVar10);
        }
        if (local_38 != DAT_00133898) {
          DAT_00133898 = (char *)0x0;
        }
        local_40 = local_30 + (local_48 - (long)local_38);
        if (DAT_00133853 != '\0') {
          DAT_00133888 = FUN_001073f5(DAT_00133888,
                                      DAT_00133810 + (-(long)local_40 - (long)DAT_00133808));
        }
        if (DAT_00133852 != '\0') {
          FUN_00107a8d(local_38);
        }
        cVar5 = FUN_00107608(local_40,param_2);
        if (cVar5 != '\x01') break;
        bVar1 = false;
      }
      piVar7 = __errno_location();
      FUN_00106edf(*piVar7);
LAB_00108d91:
      DAT_00133850 = cVar3;
      DAT_001338b0 = cVar4;
      if (((DAT_00133840 == 0) && (cVar3 != '\x01')) &&
         ((DAT_001337ca != '\0' || ((-1 < local_28 && (local_28 < local_58)))))) {
        uVar12 = FUN_00106eac();
        uVar13 = gettext("%s: binary file matches");
        error(0,0,uVar13,uVar12);
      }
    }
    else {
      piVar7 = __errno_location();
      FUN_00106edf(*piVar7);
      local_58 = 0;
    }
  }
  else {
    local_58 = 0;
  }
  return local_58;
}




// Function: grepdirent @ 0x8e14

undefined8 grepdirent(long param_1,long param_2,byte param_3)

{
  long lVar1;
  char cVar2;
  bool bVar3;
  undefined4 uVar4;
  int iVar5;
  undefined8 uVar6;
  int *piVar7;
  long in_FS_OFFSET;
  bool bVar8;
  stat *local_c0;
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  bVar8 = (*(long *)(param_2 + 0x58) == 0 & param_3) != 0;
  if (*(short *)(param_2 + 0x68) == 6) {
    uVar6 = 1;
    goto LAB_0010917d;
  }
  if (!bVar8) {
    if (((*(short *)(param_2 + 0x68) == 1) || (*(short *)(param_2 + 0x68) == 2)) ||
       (*(short *)(param_2 + 0x68) == 4)) {
      uVar4 = 1;
    }
    else {
      uVar4 = 0;
    }
    cVar2 = FUN_00107337(param_2 + 0x100,0,uVar4);
    if (cVar2 != '\0') {
      FUN_0011e92f(param_1,param_2,4);
      uVar6 = 1;
      goto LAB_0010917d;
    }
  }
  DAT_001337c0 = *(long *)(param_2 + 0x38);
  if ((DAT_001337c8 != '\0') && (*(char *)(DAT_001337c0 + 1) != '\0')) {
    DAT_001337c0 = DAT_001337c0 + 2;
  }
  lVar1 = DAT_001337c0;
  if (((*(uint *)(param_1 + 0x48) & 2) != 0) || (((*(uint *)(param_1 + 0x48) & 1) != 0 && (bVar8))))
  {
    bVar3 = true;
  }
  else {
    bVar3 = false;
  }
  switch(*(undefined2 *)(param_2 + 0x68)) {
  default:
                    /* WARNING: Subroutine does not return */
    abort();
  case 1:
    if (DAT_00133010 == 3) {
      uVar6 = 1;
      goto LAB_0010917d;
    }
    FUN_0011e92f(param_1,param_2,4);
    break;
  case 2:
    if (DAT_00133755 != '\x01') {
      uVar6 = gettext("%s: warning: recursive directory loop");
      error(0,0,uVar6,lVar1);
    }
    uVar6 = 1;
    goto LAB_0010917d;
  case 3:
  case 0xb:
    cVar2 = FUN_00106e43(bVar8);
    if (cVar2 != '\0') {
      local_c0 = (stat *)(param_2 + 0x70);
      if (*(int *)(param_2 + 0x88) == 0) {
        if (bVar3) {
          iVar5 = 0;
        }
        else {
          iVar5 = 0x100;
        }
        iVar5 = fstatat(*(int *)(param_1 + 0x2c),*(char **)(param_2 + 0x30),&local_b8,iVar5);
        if (iVar5 != 0) {
          piVar7 = __errno_location();
          FUN_00106edf(*piVar7);
          uVar6 = 1;
          goto LAB_0010917d;
        }
        local_c0 = &local_b8;
      }
      cVar2 = FUN_00106deb(local_c0->st_mode);
      if (cVar2 != '\0') {
        uVar6 = 1;
        goto LAB_0010917d;
      }
    }
    break;
  case 4:
  case 7:
  case 10:
    FUN_00106edf(*(undefined4 *)(param_2 + 0x40));
    uVar6 = 1;
    goto LAB_0010917d;
  case 8:
  case 0xd:
    break;
  case 0xc:
  case 0xe:
    uVar6 = 1;
    goto LAB_0010917d;
  }
  uVar6 = FUN_001091bc(*(undefined4 *)(param_1 + 0x2c),*(undefined8 *)(param_2 + 0x30),bVar3,bVar8);
LAB_0010917d:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar6;
}




// Function: open_symlink_nofollow_error @ 0x9197

undefined8 open_symlink_nofollow_error(int param_1)

{
  undefined8 uVar1;
  
  if ((param_1 == 0x28) || (param_1 == 0x1f)) {
    uVar1 = 1;
  }
  else {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: grepfile @ 0x91bc

undefined8 grepfile(undefined4 param_1,undefined8 param_2,char param_3,undefined1 param_4)

{
  char cVar1;
  uint uVar2;
  int iVar3;
  int *piVar4;
  undefined8 uVar5;
  uint uVar6;
  
  if (param_3 == '\0') {
    uVar6 = 0x20100;
  }
  else {
    uVar6 = 0x100;
  }
  cVar1 = FUN_00106e43(param_4);
  if (cVar1 == '\0') {
    uVar2 = 0;
  }
  else {
    uVar2 = 0x800;
  }
  iVar3 = FUN_00122af5(param_1,param_2,uVar2 | uVar6);
  if (iVar3 < 0) {
    if (param_3 == '\0') {
      piVar4 = __errno_location();
      cVar1 = FUN_00109197(*piVar4);
      if (cVar1 == '\x01') {
        return 1;
      }
    }
    piVar4 = __errno_location();
    FUN_00106edf(*piVar4);
    return 1;
  }
  uVar5 = FUN_00109456(iVar3,param_4);
  return uVar5;
}




// Function: drain_input @ 0x9272

bool drain_input(int param_1,long param_2)

{
  int *piVar1;
  long lVar2;
  bool bVar3;
  long local_10;
  
  if ((((*(uint *)(param_2 + 0x18) & 0xf000) == 0x1000) && (DAT_001338b2 != '\0')) &&
     ((local_10 = splice(param_1,(__off64_t *)0x0,1,(__off64_t *)0x0,0x18000,1), -1 < local_10 ||
      (piVar1 = __errno_location(), *piVar1 != 0x16)))) {
    while (0 < local_10) {
      local_10 = splice(param_1,(__off64_t *)0x0,1,(__off64_t *)0x0,0x18000,1);
    }
    bVar3 = local_10 == 0;
  }
  else {
    do {
      lVar2 = FUN_00124c3e(param_1,DAT_001337f0,DAT_001337f8);
      if (lVar2 == 0) {
        return true;
      }
    } while (lVar2 != -1);
    bVar3 = false;
  }
  return bVar3;
}




// Function: finalize_input @ 0x9361

void finalize_input(int param_1,undefined8 param_2,char param_3)

{
  bool bVar1;
  char cVar2;
  __off_t _Var3;
  int *piVar4;
  
  if (param_1 == 0) {
    if (DAT_001338a0 == 0) {
      if (((DAT_00133820 == DAT_00133828) || (DAT_001337d0 == '\x01')) ||
         (_Var3 = lseek(0,DAT_00133828,0), -1 < _Var3)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
    }
    else if ((param_3 == '\x01') ||
            (((DAT_001337d0 == '\0' &&
              ((_Var3 = lseek(0,0,2), -1 < _Var3 || (piVar4 = __errno_location(), *piVar4 == 0x16)))
              ) || (cVar2 = FUN_00109272(0,param_2), cVar2 == '\x01')))) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    if (bVar1) {
      piVar4 = __errno_location();
      FUN_00106edf(*piVar4);
    }
  }
  return;
}




// Function: grepdesc @ 0x9456

byte grepdesc(int param_1,byte param_2)

{
  char cVar1;
  byte bVar2;
  int iVar3;
  int *piVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  long in_FS_OFFSET;
  undefined1 local_e6;
  byte local_e5;
  uint local_e4;
  long local_e0;
  long local_d8;
  long local_d0;
  undefined8 local_c8;
  undefined8 local_c0;
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_e5 = 1;
  local_e6 = 0;
  iVar3 = fstat(param_1,&local_b8);
  if (iVar3 == 0) {
    if ((((param_1 == 0) || (cVar1 = FUN_00106e43(param_2), cVar1 == '\0')) ||
        (cVar1 = FUN_00106deb(local_b8.st_mode), cVar1 == '\0')) &&
       (((param_1 == 0 || (param_2 == 0)) ||
        (cVar1 = FUN_00107337(DAT_001337c0,1,(local_b8.st_mode & 0xf000) == 0x4000), cVar1 == '\0'))
       )) {
      if ((int)DAT_00133848 < 0) {
        DAT_00133848 = (uint)((local_b8.st_mode & 0xf000) == 0x4000);
      }
      if (((param_1 != 0) && (DAT_00133010 == 3)) && ((local_b8.st_mode & 0xf000) == 0x4000)) {
        local_e4 = DAT_00133014 & ~(uint)(param_2 ^ 1);
        iVar3 = close(param_1);
        if (iVar3 != 0) {
          piVar4 = __errno_location();
          FUN_00106edf(*piVar4);
        }
        local_c8 = DAT_001337c0;
        local_c0 = 0;
        local_e0 = FUN_0011d975(&local_c8,local_e4,0);
        if (local_e0 != 0) goto LAB_0010964b;
        FUN_00127122();
        do {
          bVar2 = FUN_00108e14(local_e0,local_d8,param_2);
          local_e5 = (bVar2 & local_e5) != 0;
LAB_0010964b:
          local_d8 = FUN_0011e2b9(local_e0);
        } while (local_d8 != 0);
        piVar4 = __errno_location();
        if (*piVar4 != 0) {
          piVar4 = __errno_location();
          FUN_00106edf(*piVar4);
        }
        iVar3 = FUN_0011ddff(local_e0);
        if (iVar3 != 0) {
          piVar4 = __errno_location();
          FUN_00106edf(*piVar4);
        }
        goto LAB_00109911;
      }
      if ((param_1 == 0) ||
         (((DAT_00133010 != 4 || ((local_b8.st_mode & 0xf000) != 0x4000)) &&
          (((DAT_001337cc != 2 && ((DAT_001337cc != 0 || (param_2 == 1)))) ||
           (cVar1 = FUN_00106deb(local_b8.st_mode), cVar1 == '\0')))))) {
        if (((((DAT_00133850 == '\x01') || (DAT_00133844 != 0)) || (DAT_00133870 < 2)) ||
            (((local_b8.st_mode & 0xf000) != 0x8000 || (local_b8.st_ino != DAT_001336c8)))) ||
           (local_b8.st_dev != DAT_001336c0)) {
          local_d0 = FUN_001087e9(param_1,&local_b8,&local_e6);
          if (DAT_00133868 != '\0') {
            if (DAT_00133848 != 0) {
              FUN_00107b22();
              if (DAT_0013384c == 0) {
                FUN_00106c26(0);
              }
              else {
                FUN_00107b58(0x3a);
              }
            }
            FUN_00106c8d(&DAT_001294ca,local_d0);
            if (DAT_00133878 != '\0') {
              FUN_00106dc0();
            }
          }
          local_e5 = local_d0 == 0;
          if (DAT_00133844 == 0) {
            FUN_00109361(param_1,&local_b8,local_e6);
          }
          else {
            if ((bool)local_e5) {
              iVar3 = 2;
            }
            else {
              iVar3 = 1;
            }
            if (iVar3 == DAT_00133844) {
              FUN_00107b22();
              FUN_00106c26(DAT_0013384c & 10);
              if (DAT_00133878 != '\0') {
                FUN_00106dc0();
              }
            }
          }
        }
        else {
          if (DAT_00133755 != '\x01') {
            uVar5 = FUN_00106eac();
            uVar6 = gettext("%s: input file is also the output");
            error(0,0,uVar6,uVar5);
          }
          DAT_001337c9 = 1;
        }
      }
    }
  }
  else {
    piVar4 = __errno_location();
    FUN_00106edf(*piVar4);
  }
  if ((param_1 != 0) && (iVar3 = close(param_1), iVar3 != 0)) {
    piVar4 = __errno_location();
    FUN_00106edf(*piVar4);
  }
LAB_00109911:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_e5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: grep_command_line_arg @ 0x992b

void grep_command_line_arg(char *param_1)

{
  int iVar1;
  
  iVar1 = strcmp(param_1,"-");
  if (iVar1 == 0) {
    DAT_001337c0 = DAT_00133880;
    if (DAT_001338b3 != '\0') {
      FUN_0012716d(0,0);
    }
    FUN_00109456(0,1);
  }
  else {
    DAT_001337c0 = param_1;
    FUN_001091bc(0xffffff9c,param_1,1,1);
  }
  return;
}




// Function: usage @ 0x99b6

void usage(int param_1)

{
  undefined8 uVar1;
  char *pcVar2;
  
  if (param_1 == 0) {
    uVar1 = FUN_0011ff0c();
    pcVar2 = (char *)gettext("Usage: %s [OPTION]... PATTERNS [FILE]...\n");
    printf(pcVar2,uVar1);
    pcVar2 = (char *)gettext("Search for PATTERNS in each FILE.\n");
    printf(pcVar2);
    uVar1 = FUN_0011ff0c();
    pcVar2 = (char *)gettext(
                            "Example: %s -i \'hello world\' menu.h main.c\nPATTERNS can contain multiple patterns separated by newlines.\n\nPattern selection and interpretation:\n"
                            );
    printf(pcVar2,uVar1);
    pcVar2 = (char *)gettext(
                            "  -E, --extended-regexp     PATTERNS are extended regular expressions\n  -F, --fixed-strings       PATTERNS are strings\n  -G, --basic-regexp        PATTERNS are basic regular expressions\n  -P, --perl-regexp         PATTERNS are Perl regular expressions\n"
                            );
    printf(pcVar2);
    pcVar2 = (char *)gettext(
                            "  -e, --regexp=PATTERNS     use PATTERNS for matching\n  -f, --file=FILE           take PATTERNS from FILE\n  -i, --ignore-case         ignore case distinctions in patterns and data\n      --no-ignore-case      do not ignore case distinctions (default)\n  -w, --word-regexp         match only whole words\n  -x, --line-regexp         match only whole lines\n  -z, --null-data           a data line ends in 0 byte, not newline\n"
                            );
    printf(pcVar2);
    pcVar2 = (char *)gettext(
                            "\nMiscellaneous:\n  -s, --no-messages         suppress error messages\n  -v, --invert-match        select non-matching lines\n  -V, --version             display version information and exit\n      --help                display this help text and exit\n"
                            );
    printf(pcVar2);
    pcVar2 = (char *)gettext(
                            "\nOutput control:\n  -m, --max-count=NUM       stop after NUM selected lines\n  -b, --byte-offset         print the byte offset with output lines\n  -n, --line-number         print line number with output lines\n      --line-buffered       flush output on every line\n  -H, --with-filename       print file name with output lines\n  -h, --no-filename         suppress the file name prefix on output\n      --label=LABEL         use LABEL as the standard input file name prefix\n"
                            );
    printf(pcVar2);
    pcVar2 = (char *)gettext(
                            "  -o, --only-matching       show only nonempty parts of lines that match\n  -q, --quiet, --silent     suppress all normal output\n      --binary-files=TYPE   assume that binary files are TYPE;\n                            TYPE is \'binary\', \'text\', or \'without-match\'\n  -a, --text                equivalent to --binary-files=text\n"
                            );
    printf(pcVar2);
    pcVar2 = (char *)gettext(
                            "  -I                        equivalent to --binary-files=without-match\n  -d, --directories=ACTION  how to handle directories;\n                            ACTION is \'read\', \'recurse\', or \'skip\'\n  -D, --devices=ACTION      how to handle devices, FIFOs and sockets;\n                            ACTION is \'read\' or \'skip\'\n  -r, --recursive           like --directories=recurse\n  -R, --dereference-recursive  likewise, but follow all symlinks\n"
                            );
    printf(pcVar2);
    pcVar2 = (char *)gettext(
                            "      --include=GLOB        search only files that match GLOB (a file pattern)\n      --exclude=GLOB        skip files that match GLOB\n      --exclude-from=FILE   skip files that match any file pattern from FILE\n      --exclude-dir=GLOB    skip directories that match GLOB\n"
                            );
    printf(pcVar2);
    pcVar2 = (char *)gettext(
                            "  -L, --files-without-match  print only names of FILEs with no selected lines\n  -l, --files-with-matches  print only names of FILEs with selected lines\n  -c, --count               print only a count of selected lines per FILE\n  -T, --initial-tab         make tabs line up (if needed)\n  -Z, --null                print 0 byte after FILE name\n"
                            );
    printf(pcVar2);
    pcVar2 = (char *)gettext(
                            "\nContext control:\n  -B, --before-context=NUM  print NUM lines of leading context\n  -A, --after-context=NUM   print NUM lines of trailing context\n  -C, --context=NUM         print NUM lines of output context\n"
                            );
    printf(pcVar2);
    pcVar2 = (char *)gettext(
                            "  -NUM                      same as --context=NUM\n      --group-separator=SEP  print SEP on line between matches with context\n      --no-group-separator  do not print separator for matches with context\n      --color[=WHEN],\n      --colour[=WHEN]       use markers to highlight the matching strings;\n                            WHEN is \'always\', \'never\', or \'auto\'\n  -U, --binary              do not strip CR characters at EOL (MSDOS/Windows)\n\n"
                            );
    printf(pcVar2);
    pcVar2 = (char *)gettext(
                            "When FILE is \'-\', read standard input.  With no FILE, read \'.\' if\nrecursive, \'-\' otherwise.  With fewer than two FILEs, assume -h.\nExit status is 0 if any line is selected, 1 otherwise;\nif any error occurs and -q is not given, the exit status is 2.\n"
                            );
    printf(pcVar2);
    FUN_001269ee();
  }
  else {
    uVar1 = FUN_0011ff0c();
    pcVar2 = (char *)gettext("Usage: %s [OPTION]... PATTERNS [FILE]...\n");
    fprintf(stderr,pcVar2,uVar1);
    uVar1 = FUN_0011ff0c();
    pcVar2 = (char *)gettext("Try \'%s --help\' for more information.\n");
    fprintf(stderr,pcVar2,uVar1);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: setmatcher @ 0x9be6

ulong setmatcher(char *param_1,uint param_2)

{
  int iVar1;
  undefined8 uVar2;
  ulong uVar3;
  uint local_c;
  
  local_c = 0;
  while( true ) {
    if (6 < local_c) {
      uVar2 = gettext("invalid matcher %s");
      uVar3 = error(2,0,uVar2,param_1);
      return uVar3;
    }
    iVar1 = strcmp(param_1,&DAT_001327a0 + (long)(int)local_c * 0x20);
    if (iVar1 == 0) break;
    local_c = local_c + 1;
  }
  if ((-1 < (int)param_2) && (param_2 != local_c)) {
    uVar2 = gettext("conflicting matchers specified");
    error(2,0,uVar2);
  }
  return (ulong)local_c;
}




// Function: get_nondigit_option @ 0x9c9f

undefined4 get_nondigit_option(undefined4 param_1,undefined8 param_2,undefined8 param_3)

{
  bool bVar1;
  int iVar2;
  char cVar3;
  undefined4 uVar4;
  long in_FS_OFFSET;
  char *local_40;
  char local_38 [21];
  char acStack_23 [19];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  bVar1 = false;
  local_40 = local_38;
  do {
    iVar2 = optind;
    uVar4 = getopt_long(param_1,param_2,"0123456789A:B:C:D:EFGHIPTUVX:abcd:e:f:hiLlm:noqRrsuvwxyZz",
                        &PTR_s_basic_regexp_001320a0,0);
    cVar3 = FUN_00110eab(uVar4);
    if (cVar3 != '\x01') {
LAB_00109da2:
      if (local_40 != local_38) {
        *local_40 = '\0';
        FUN_0010728f(local_38,param_3);
      }
      if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return uVar4;
    }
    if ((iVar2 == DAT_00133018) && (bVar1)) {
      local_40 = local_40 + -(ulong)(local_38[0] == '0');
    }
    else {
      local_40 = local_38;
    }
    if (local_40 == acStack_23) {
      strcpy(local_40,"...");
      local_40 = local_40 + 3;
      goto LAB_00109da2;
    }
    *local_40 = (char)uVar4;
    bVar1 = true;
    DAT_00133018 = iVar2;
    local_40 = local_40 + 1;
  } while( true );
}




// Function: parse_grep_colors @ 0x9ddf

void parse_grep_colors(void)

{
  char cVar1;
  int iVar2;
  char *pcVar3;
  char *local_30;
  char *local_28;
  char *local_20;
  undefined **local_18;
  
  pcVar3 = getenv("GREP_COLORS");
  if ((pcVar3 != (char *)0x0) && (*pcVar3 != '\0')) {
    local_28 = (char *)FUN_001270f1(pcVar3);
    local_20 = (char *)0x0;
    local_30 = local_28;
    while( true ) {
      while ((*local_30 != ':' && (*local_30 != '\0'))) {
        if (*local_30 == '=') {
          if (local_30 == local_28) {
            return;
          }
          if (local_20 != (char *)0x0) {
            return;
          }
          local_20 = local_30 + 1;
          *local_30 = '\0';
          local_30 = local_20;
        }
        else {
          if (local_20 != (char *)0x0) {
            if ((*local_30 != ';') && (cVar1 = FUN_00110eab((int)*local_30), cVar1 == '\0')) {
              return;
            }
            goto LAB_00109f6a;
          }
          local_30 = local_30 + 1;
        }
      }
      cVar1 = *local_30;
      *local_30 = '\0';
      local_18 = &PTR_DAT_00131f80;
      while ((*local_18 != (undefined *)0x0 && (iVar2 = strcmp(*local_18,local_28), iVar2 != 0))) {
        local_18 = local_18 + 3;
      }
      if ((local_18[1] != (undefined *)0x0) && (local_20 != (char *)0x0)) {
        *(char **)local_18[1] = local_20;
      }
      if (local_18[2] != (undefined *)0x0) {
        (*(code *)local_18[2])();
      }
      if (cVar1 == '\0') break;
      local_20 = (char *)0x0;
      local_28 = local_30 + 1;
LAB_00109f6a:
      local_30 = local_30 + 1;
    }
  }
  return;
}




// Function: contains_encoding_error @ 0x9f7b

undefined8 contains_encoding_error(long param_1,long param_2)

{
  long lVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  long local_28;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_18 = 0;
  local_28 = 0;
  do {
    if (param_2 <= local_28) {
      uVar2 = 0;
LAB_00109ffb:
      if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return uVar2;
    }
    lVar1 = FUN_0010f6da(local_28 + param_1,param_2 - local_28,&local_18);
    if (lVar1 < 0) {
      uVar2 = 1;
      goto LAB_00109ffb;
    }
    local_28 = local_28 + lVar1;
  } while( true );
}




// Function: setup_ok_fold @ 0xa011

void setup_ok_fold(void)

{
  size_t sVar1;
  long in_FS_OFFSET;
  int local_c0;
  undefined1 local_bc;
  int local_b8;
  mbstate_t local_b0;
  wchar_t local_a8 [32];
  char local_28 [24];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_c0 = 0;
  do {
    if (0xff < local_c0) {
      if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return;
    }
    if (*(int *)(&DAT_001331a4 + ((long)local_c0 + 0x40) * 4) != -1) {
      local_bc = 1;
      local_b8 = FUN_0011c01a(*(int *)(&DAT_001331a4 + ((long)local_c0 + 0x40) * 4),local_a8);
      do {
        local_b8 = local_b8 + -1;
        if (local_b8 < 0) goto LAB_0010a0e9;
        local_b0.__count = 0;
        local_b0.__value = (_union_27)0x0;
        sVar1 = wcrtomb(local_28,local_a8[local_b8],&local_b0);
      } while (sVar1 == 1);
      local_bc = 0xff;
LAB_0010a0e9:
      (&DAT_001338c0)[local_c0] = local_bc;
    }
    local_c0 = local_c0 + 1;
  } while( true );
}




// Function: fgrep_icase_charlen @ 0xa134

ulong fgrep_icase_charlen(byte *param_1,undefined8 param_2,undefined8 param_3)

{
  byte bVar1;
  int iVar2;
  uint uVar3;
  ulong uVar4;
  long in_FS_OFFSET;
  undefined4 local_ac;
  ulong local_a8;
  ulong local_a0;
  undefined1 local_98 [136];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (*(int *)(&DAT_001331a4 + ((long)(int)(uint)*param_1 + 0x40) * 4) == -1) {
    local_a0 = FUN_00121b80(&local_ac,param_1,param_2,param_3);
    if (local_a0 < 0x11) {
      iVar2 = FUN_0011c01a(local_ac,local_98);
      if (iVar2 == 0) {
        local_a8 = local_a0;
        do {
          local_a8 = local_a8 - 1;
          uVar4 = local_a0;
          if ((long)local_a8 < 1) goto LAB_0010a284;
          bVar1 = param_1[local_a8];
          uVar3 = toupper((uint)bVar1);
        } while (uVar3 == bVar1);
        uVar4 = 0xffffffffffffffff;
      }
      else {
        uVar4 = 0xffffffffffffffff;
      }
    }
    else {
      uVar4 = 0xffffffffffffffff;
    }
  }
  else {
    uVar4 = (long)(char)(&DAT_001338c0)[(int)(uint)*param_1];
  }
LAB_0010a284:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar4;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: fgrep_icase_available @ 0xa29a

undefined8 fgrep_icase_available(long param_1,long param_2)

{
  int iVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  long local_20;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_18 = 0;
  local_20 = 0;
  do {
    if (param_2 <= local_20) {
      uVar2 = 1;
LAB_0010a319:
      if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return uVar2;
    }
    iVar1 = FUN_0010a134(local_20 + param_1,param_2 - local_20,&local_18);
    if (iVar1 < 0) {
      uVar2 = 0;
      goto LAB_0010a319;
    }
    local_20 = local_20 + iVar1;
  } while( true );
}




// Function: fgrep_to_grep_pattern @ 0xa32f

void fgrep_to_grep_pattern(undefined8 *param_1,size_t *param_2)

{
  char *pcVar1;
  long in_FS_OFFSET;
  size_t local_40;
  char *local_38;
  char *local_30;
  size_t local_28;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_40 = *param_2;
  local_38 = (char *)*param_1;
  local_18 = 0;
  pcVar1 = (char *)FUN_00126c63(local_40 + 1,2);
  local_30 = pcVar1;
  do {
    if (local_40 == 0) {
      *local_30 = '\n';
      free((void *)*param_1);
      *param_1 = pcVar1;
      *param_2 = (long)local_30 - (long)pcVar1;
      if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    local_28 = FUN_0010f6da(local_38,local_40,&local_18);
    if (local_28 == 1) {
LAB_0010a412:
      if (((int)*local_38 - 0x24U < 0x3b) &&
         ((0x580000000000441U >> ((byte)((int)*local_38 - 0x24U) & 0x3f) & 1) != 0)) {
        *local_30 = '\\';
        local_30 = local_30 + 1;
      }
      *local_30 = *local_38;
      local_30 = local_30 + 1;
    }
    else {
      if ((long)local_28 < 2) {
        if (local_28 == 0xfffffffffffffffe) {
          local_28 = local_40;
        }
        else if (local_28 == 0xffffffffffffffff) {
          memset(&local_18,0,8);
          local_28 = 1;
          goto LAB_0010a412;
        }
      }
      local_30 = mempcpy(local_30,local_38,local_28);
    }
    local_38 = local_38 + local_28;
    local_40 = local_40 - local_28;
  } while( true );
}




// Function: try_fgrep_pattern @ 0xa4d3

int try_fgrep_pattern(int param_1,byte *param_2,long *param_3)

{
  byte bVar1;
  void *__src;
  size_t __n;
  undefined1 *puVar2;
  ulong uVar3;
  long in_FS_OFFSET;
  int local_4c;
  long local_48;
  void *local_40;
  byte *local_38;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_48 = *param_3;
  __src = (void *)FUN_00126b10(local_48 + 1);
  local_18 = 0;
  local_40 = __src;
  local_38 = param_2;
  for (; local_48 != 0; local_48 = local_48 - __n) {
    bVar1 = *local_38;
    local_4c = param_1;
    if ((char)bVar1 < '}') {
      if ('z' < (char)bVar1) goto LAB_0010a5e5;
      if (bVar1 == 0x5e) goto LAB_0010a7ac;
      if ((char)bVar1 < '_') {
        if (bVar1 == 0x5c) {
          if (1 < local_48) {
            bVar1 = local_38[1];
            if ((char)bVar1 < '=') {
              if ('\t' < (char)bVar1) {
                uVar3 = 1L << (bVar1 & 0x3f);
                if ((uVar3 & 0x13fe008000000400) != 0) goto LAB_0010a7ac;
                uVar3 = uVar3 & 0xb0000000000;
joined_r0x0010a65d:
                if ((uVar3 != 0) && (param_1 == 0)) goto LAB_0010a7ac;
              }
            }
            else if (((char)bVar1 < '}') && ('=' < (char)bVar1)) {
              uVar3 = 1L << (bVar1 - 0x3e & 0x3f);
              if ((uVar3 & 0x220001402200011) == 0) {
                uVar3 = uVar3 & 0x6000000000000002;
                goto joined_r0x0010a65d;
              }
              goto LAB_0010a7ac;
            }
            local_38 = local_38 + 1;
            local_48 = local_48 + -1;
          }
        }
        else if ((char)bVar1 < ']') {
          if (bVar1 == 0x5b) goto LAB_0010a7ac;
          if ((char)bVar1 < '\\') {
            if ((char)bVar1 < '/') {
              if ('#' < (char)bVar1) {
                uVar3 = 1L << (bVar1 & 0x3f);
                if ((uVar3 & 0x441000000000) != 0) goto LAB_0010a7ac;
                if ((uVar3 & 0x90000000000) != 0) goto LAB_0010a5e5;
              }
            }
            else if (bVar1 == 0x3f) {
LAB_0010a5e5:
              if (param_1 != 0) goto LAB_0010a7ac;
            }
          }
        }
      }
    }
    if (DAT_00133180 == '\0') {
      __n = FUN_0010f6da(local_38,local_48,&local_18);
    }
    else {
      __n = FUN_0010a134(local_38,local_48,&local_18);
    }
    if ((long)__n < 0) goto LAB_0010a7ac;
    local_40 = mempcpy(local_40,local_38,__n);
    local_38 = local_38 + __n;
  }
  if (*param_3 != (long)local_40 - (long)__src) {
    *param_3 = (long)local_40 - (long)__src;
    puVar2 = mempcpy(param_2,__src,(long)local_40 - (long)__src);
    *puVar2 = 10;
  }
  local_4c = 2;
LAB_0010a7ac:
  free(__src);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_4c;
}




// Function: main @ 0xa7d1

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined1 main(int param_1,long param_2)

{
  undefined *puVar1;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  bool bVar4;
  char cVar5;
  byte bVar6;
  uint uVar7;
  uint uVar8;
  undefined4 uVar9;
  int iVar10;
  int *piVar11;
  undefined8 uVar12;
  char *pcVar13;
  long lVar14;
  long in_FS_OFFSET;
  bool local_202;
  int local_200;
  int local_1fc;
  uint local_1f8;
  int local_1f4;
  int local_1f0;
  int local_1ec;
  int local_1e8;
  int local_1e4;
  int local_1e0;
  long local_1d8;
  long local_1d0;
  long local_1c8;
  undefined1 local_1c0 [8];
  char *local_1b8;
  FILE *local_1b0;
  long local_1a8;
  char *local_1a0;
  undefined **local_198;
  char *local_190;
  size_t local_188;
  char *local_180;
  ulong local_178;
  long local_170;
  size_t local_168;
  size_t local_160;
  long local_158;
  undefined1 *local_150;
  stat local_148;
  stat local_b8;
  undefined1 local_23;
  undefined1 local_22;
  undefined1 local_21;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_1b8 = (char *)0x0;
  local_1d8 = 0;
  local_1d0 = 0;
  local_200 = -1;
  DAT_00133080 = 2;
  local_1f8 = 0;
  DAT_00133183 = 10;
  DAT_0013384c = 0xffffffff;
  DAT_00133870 = 0x7fffffffffffffff;
  DAT_00133858 = -1;
  DAT_00133860 = -1;
  local_1c8 = -1;
  DAT_0013375c = '\0';
  setlocale(6,"");
  bindtextdomain(&DAT_0012a4c2,"/usr/local/share/locale");
  textdomain(&DAT_0012a4c2);
  FUN_0011bf18(&DAT_001331a0);
  FUN_001285f0(FUN_00106f2c);
  FUN_001112dd(0);
  local_1fc = 0;
  DAT_00133790 = FUN_001208d6(0,0,FUN_00106760,FUN_001067f8,0);
  if (DAT_00133790 != 0) goto switchD_0010a970_caseD_0;
  FUN_00127122();
  do {
    switch(local_1e0) {
    case 0:
    case 0x55:
      break;
    default:
      FUN_001099b6(2);
      break;
    case 0x41:
      FUN_0010728f(optarg,&DAT_00133860);
      break;
    case 0x42:
      FUN_0010728f(optarg,&DAT_00133858);
      break;
    case 0x43:
      FUN_0010728f(optarg,&local_1c8);
      break;
    case 0x44:
      iVar10 = strcmp(optarg,"read");
      if (iVar10 == 0) {
        DAT_001337cc = 1;
      }
      else {
        iVar10 = strcmp(optarg,"skip");
        if (iVar10 != 0) {
          uVar12 = gettext("unknown devices method");
          error(2,0,uVar12);
          goto switchD_0010a970_caseD_45;
        }
        DAT_001337cc = 2;
      }
      break;
    case 0x45:
switchD_0010a970_caseD_45:
      local_200 = FUN_00109be6("egrep",local_200);
      break;
    case 0x46:
      local_200 = FUN_00109be6("fgrep",local_200);
      break;
    case 0x47:
      local_200 = FUN_00109be6(&DAT_0012a4c2,local_200);
      break;
    case 0x48:
      local_1f8 = 1;
      break;
    case 0x49:
      DAT_00133840 = 2;
      break;
    case 0x4c:
      DAT_00133844 = 2;
      break;
    case 0x50:
      local_200 = FUN_00109be6(&DAT_0012a4ea,local_200);
      break;
    case 0x52:
      DAT_00133014 = 0x30a;
    case 0x72:
      DAT_00133010 = 3;
      local_1fc = local_1e4;
      break;
    case 0x54:
      DAT_0013375d = 1;
      break;
    case 0x56:
      DAT_00133754 = '\x01';
      break;
    case 0x58:
      local_200 = FUN_00109be6(optarg,local_200);
      break;
    case 0x5a:
      DAT_0013384c = 0;
      break;
    case 0x61:
      DAT_00133840 = 1;
      break;
    case 0x62:
      DAT_00133853 = 1;
      break;
    case 99:
      DAT_00133868 = '\x01';
      break;
    case 100:
      lVar14 = FUN_00110c86("--directories",optarg,&PTR_DAT_00132700,&DAT_001293a8,4,
                            PTR_FUN_00133078,1);
      DAT_00133010 = *(int *)(&DAT_001293a8 + lVar14 * 4);
      if (DAT_00133010 == 3) {
        local_1fc = local_1e4;
      }
      break;
    case 0x65:
      local_160 = strlen(optarg);
      local_158 = local_160 + (local_1d8 - local_1d0) + 1;
      if (0 < local_158) {
        DAT_00133780 = (char *)FUN_00126dc6(local_1b8,&local_1d0,local_158,0xffffffffffffffff,1);
        local_1b8 = DAT_00133780;
      }
      local_150 = mempcpy(local_1b8 + local_1d8,optarg,local_160);
      *local_150 = 10;
      local_1d8 = FUN_0010687b(local_1b8,local_1d8,local_160 + local_1d8 + 1,&DAT_001290d2);
      break;
    case 0x66:
      iVar10 = strcmp(optarg,"-");
      if (iVar10 == 0) {
        if (DAT_001338b3 != '\0') {
          FUN_0012716d(0,0);
        }
        local_1b0 = stdin;
      }
      else {
        if (DAT_001338b3 == '\0') {
          pcVar13 = "r";
        }
        else {
          pcVar13 = "rb";
        }
        local_1b0 = fopen(optarg,pcVar13);
        pcVar13 = optarg;
        if (local_1b0 == (FILE *)0x0) {
          piVar11 = __errno_location();
          error(2,*piVar11,&DAT_001293c5,pcVar13);
        }
      }
      local_1a8 = local_1d8;
      while( true ) {
        local_170 = (local_1a8 - local_1d0) + 2;
        if (0 < local_170) {
          DAT_00133780 = (char *)FUN_00126dc6(local_1b8,&local_1d0,local_170,0xffffffffffffffff,1);
          local_1b8 = DAT_00133780;
        }
        local_168 = fread_unlocked(local_1b8 + local_1a8,1,local_1d0 - (local_1a8 + 1),local_1b0);
        if (local_168 == 0) break;
        local_1a8 = local_1a8 + local_168;
      }
      piVar11 = __errno_location();
      local_1f4 = *piVar11;
      iVar10 = ferror_unlocked(local_1b0);
      if (iVar10 == 0) {
        local_1f4 = 0;
        if (local_1b0 == stdin) {
          clearerr_unlocked(local_1b0);
        }
        else {
          iVar10 = fclose(local_1b0);
          if (iVar10 != 0) {
            piVar11 = __errno_location();
            local_1f4 = *piVar11;
          }
        }
      }
      if (local_1f4 != 0) {
        error(2,local_1f4,&DAT_001293c5,optarg);
      }
      if ((local_1a8 != local_1d8) && (local_1b8[local_1a8 + -1] != '\n')) {
        local_1b8[local_1a8] = '\n';
        local_1a8 = local_1a8 + 1;
      }
      local_1d8 = FUN_0010687b(local_1b8,local_1d8,local_1a8,optarg);
      break;
    case 0x68:
      local_1f8 = 0xffffffff;
      break;
    case 0x69:
    case 0x79:
      DAT_00133180 = '\x01';
      break;
    case 0x6c:
      DAT_00133844 = 1;
      break;
    case 0x6d:
      uVar7 = FUN_00127281(optarg,0,10,&DAT_00133870,&DAT_001290d2);
      if (1 < uVar7) {
        uVar12 = gettext("invalid max count");
        error(2,0,uVar12);
      }
      break;
    case 0x6e:
      DAT_00133852 = 1;
      break;
    case 0x6f:
      DAT_0013375c = '\x01';
      break;
    case 0x71:
      DAT_001338b1 = '\x01';
      DAT_00133080 = 0;
      break;
    case 0x73:
      DAT_00133755 = 1;
      break;
    case 0x75:
      uVar12 = gettext("warning: --unix-byte-offsets (-u) is obsolete");
      error(0,0,uVar12);
      break;
    case 0x76:
      DAT_00133851 = '\x01';
      break;
    case 0x77:
      FUN_0010f770();
      DAT_00133181 = '\x01';
      break;
    case 0x78:
      DAT_00133182 = '\x01';
      break;
    case 0x7a:
      DAT_00133183 = 0;
      break;
    case 0x80:
      iVar10 = strcmp(optarg,"binary");
      if (iVar10 == 0) {
        DAT_00133840 = 0;
      }
      else {
        iVar10 = strcmp(optarg,"text");
        if (iVar10 == 0) {
          DAT_00133840 = 1;
        }
        else {
          iVar10 = strcmp(optarg,"without-match");
          if (iVar10 != 0) {
            uVar12 = gettext("unknown binary-files type");
            error(2,0,uVar12);
            goto switchD_0010a970_caseD_81;
          }
          DAT_00133840 = 2;
        }
      }
      break;
    case 0x81:
switchD_0010a970_caseD_81:
      if (optarg == (char *)0x0) {
        DAT_00133758 = 2;
      }
      else {
        iVar10 = FUN_00111390(optarg,"always");
        if (((iVar10 == 0) || (iVar10 = FUN_00111390(optarg,&DAT_0012a572), iVar10 == 0)) ||
           (iVar10 = FUN_00111390(optarg,"force"), iVar10 == 0)) {
          DAT_00133758 = 1;
        }
        else {
          iVar10 = FUN_00111390(optarg,"never");
          if (((iVar10 == 0) || (iVar10 = FUN_00111390(optarg,&DAT_0012a582), iVar10 == 0)) ||
             (iVar10 = FUN_00111390(optarg,&DAT_0012a585), iVar10 == 0)) {
            DAT_00133758 = 0;
          }
          else {
            iVar10 = FUN_00111390(optarg,&DAT_0012a58a);
            if (((iVar10 == 0) || (iVar10 = FUN_00111390(optarg,&DAT_0012a58f), iVar10 == 0)) ||
               (iVar10 = FUN_00111390(optarg,"if-tty"), iVar10 == 0)) {
              DAT_00133758 = 2;
            }
            else {
              DAT_00133750 = 1;
            }
          }
        }
      }
      break;
    case 0x82:
      FUN_0011c12b(optarg);
      for (local_1e8 = 0; local_1e8 < 2; local_1e8 = local_1e8 + 1) {
        if (*(long *)(&DAT_001337b0 + (long)local_1e8 * 8) == 0) {
          uVar12 = FUN_0011c384();
          *(undefined8 *)(&DAT_001337b0 + (long)local_1e8 * 8) = uVar12;
        }
        uVar9 = FUN_00107316(local_1e8 != 0);
        FUN_0011cca9(*(undefined8 *)(&DAT_001337b0 + (long)local_1e8 * 8),optarg,uVar9);
      }
      break;
    case 0x83:
    case 0x86:
      for (local_1f0 = 0; local_1f0 < 2; local_1f0 = local_1f0 + 1) {
        if (*(long *)(&DAT_001337a0 + (long)local_1f0 * 8) == 0) {
          uVar12 = FUN_0011c384();
          *(undefined8 *)(&DAT_001337a0 + (long)local_1f0 * 8) = uVar12;
        }
        if (local_1e0 == 0x86) {
          uVar7 = 0x20000000;
        }
        else {
          uVar7 = 0;
        }
        uVar8 = FUN_00107316(local_1f0 != 0);
        FUN_0011cca9(*(undefined8 *)(&DAT_001337a0 + (long)local_1f0 * 8),optarg,uVar7 | uVar8);
      }
      break;
    case 0x84:
      for (local_1ec = 0; local_1ec < 2; local_1ec = local_1ec + 1) {
        if (*(long *)(&DAT_001337a0 + (long)local_1ec * 8) == 0) {
          uVar12 = FUN_0011c384();
          *(undefined8 *)(&DAT_001337a0 + (long)local_1ec * 8) = uVar12;
        }
        uVar9 = FUN_00107316(local_1ec != 0);
        iVar10 = FUN_0011d272(FUN_0011cca9,*(undefined8 *)(&DAT_001337a0 + (long)local_1ec * 8),
                              optarg,uVar9,10);
        pcVar13 = optarg;
        if (iVar10 != 0) {
          piVar11 = __errno_location();
          error(2,*piVar11,&DAT_001293c5,pcVar13);
        }
      }
      break;
    case 0x85:
      PTR_DAT_00133020 = optarg;
      break;
    case 0x87:
      DAT_00133878 = 1;
      break;
    case 0x88:
      DAT_00133880 = optarg;
      break;
    case 0x89:
      DAT_00133180 = '\0';
    }
switchD_0010a970_caseD_0:
    local_1e4 = optind;
    local_1e0 = FUN_00109c9f(param_1,param_2,&local_1c8);
  } while (local_1e0 != -1);
  if (DAT_00133754 != '\0') {
    uVar12 = FUN_0011ff0c();
    FUN_0012690a(stdout,uVar12,"GNU grep",&DAT_0012a59a,0);
    pcVar13 = (char *)gettext(
                             "Written by Mike Haertel and others; see\n<https://git.sv.gnu.org/cgit/grep.git/tree/AUTHORS>."
                             );
    puts(pcVar13);
    local_202 = false;
    goto LAB_0010bfff;
  }
  if (DAT_00133750 != 0) {
    FUN_001099b6(0);
  }
  if (local_1b8 == (char *)0x0) {
    if (optind < param_1) {
      local_190 = *(char **)(param_2 + (long)optind * 8);
      if (((local_200 == 2) || (*local_190 != '\\')) || (local_190[1] != '-')) {
        bVar6 = 0;
      }
      else {
        bVar6 = 1;
      }
      optind = optind + 1;
      DAT_00133780 = (char *)FUN_001270f1(local_190 + bVar6);
      local_1b8 = DAT_00133780;
      local_188 = strlen(DAT_00133780);
      local_1b8[local_188] = '\n';
      local_1d8 = FUN_0010687b(local_1b8,0,local_188 + 1,&DAT_001290d2);
    }
    else {
      FUN_001099b6(2);
    }
  }
  else if (local_1d8 == 0) {
    DAT_00133851 = DAT_00133851 != '\x01';
    DAT_00133181 = '\0';
    DAT_00133182 = '\0';
    local_1d8 = 1;
    *local_1b8 = '\n';
  }
  local_1d8 = local_1d8 + -1;
  FUN_00120b56(DAT_00133790);
  bVar4 = false;
  if ((DAT_001338b1 != '\x01') && (iVar10 = fstat(1,&local_148), iVar10 == 0)) {
    if ((local_148.st_mode & 0xf000) == 0x8000) {
      DAT_001336c0 = local_148.st_dev;
      DAT_001336c8 = local_148.st_ino;
      _DAT_001336d8 = CONCAT44(local_148.st_uid,local_148.st_mode);
      _DAT_001336d0 = local_148.st_nlink;
      _DAT_001336e0 = local_148.st_gid;
      DAT_001336e0_4 = local_148.__pad0;
      _DAT_001336e8 = local_148.st_rdev;
      _DAT_001336f0 = local_148.st_size;
      _DAT_001336f8 = local_148.st_blksize;
      _DAT_00133700 = local_148.st_blocks;
      _DAT_00133708 = local_148.st_atim.tv_sec;
      _DAT_00133710 = local_148.st_atim.tv_nsec;
      _DAT_00133718 = local_148.st_mtim.tv_sec;
      _DAT_00133720 = local_148.st_mtim.tv_nsec;
      _DAT_00133728 = local_148.st_ctim.tv_sec;
      _DAT_00133730 = local_148.st_ctim.tv_nsec;
      _DAT_00133738 = local_148.__unused[0];
      _DAT_00133740 = local_148.__unused[1];
      _DAT_00133748 = local_148.__unused[2];
    }
    else if ((local_148.st_mode & 0xf000) == 0x2000) {
      iVar10 = stat("/dev/null",&local_b8);
      if (((iVar10 == 0) && (local_148.st_ino == local_b8.st_ino)) &&
         (local_148.st_dev == local_b8.st_dev)) {
        DAT_001338b2 = '\x01';
      }
      else {
        bVar4 = true;
      }
    }
  }
  if (DAT_001338b2 == '\0' && DAT_001338b1 == '\0') {
    if (DAT_00133844 != 0) goto LAB_0010b9b9;
  }
  else {
    DAT_00133844 = 0;
LAB_0010b9b9:
    DAT_00133868 = '\0';
    DAT_001338b0 = '\x01';
  }
  DAT_00133850 = DAT_001338b0 != '\0' || DAT_00133868 != '\0';
  if (DAT_00133860 < 0) {
    DAT_00133860 = local_1c8;
  }
  if (DAT_00133858 < 0) {
    DAT_00133858 = local_1c8;
  }
  if (((DAT_00133870 == 0) ||
      ((((local_1d8 == 0 && (DAT_00133851 != '\0')) && (DAT_00133182 != '\x01')) &&
       (DAT_00133181 != '\x01')))) && (DAT_00133844 != 2)) {
    local_202 = true;
  }
  else {
    if (DAT_00133758 == 2) {
      if (((bVar4) && (iVar10 = FUN_0012768b(), iVar10 != 0)) && (iVar10 = isatty(1), iVar10 != 0))
      {
        DAT_00133758 = 1;
      }
      else {
        DAT_00133758 = 0;
      }
    }
    FUN_001276d9();
    if (DAT_00133758 != 0) {
      local_180 = getenv("GREP_COLOR");
      if ((local_180 != (char *)0x0) && (pcVar13 = local_180, *local_180 != '\0')) {
        while( true ) {
          local_1a0 = pcVar13;
          if ((*local_1a0 != ';') && (cVar5 = FUN_00110eab((int)*local_1a0), cVar5 == '\0'))
          goto LAB_0010bb59;
          if (local_1a0[1] == '\0') break;
          pcVar13 = local_1a0 + 1;
        }
        PTR_s_01_31_00133030 = local_180;
        PTR_s_01_31_00133028 = local_180;
      }
LAB_0010bb59:
      FUN_00109ddf();
      if ((local_180 == PTR_s_01_31_00133028) || (local_180 == PTR_s_01_31_00133030)) {
        uVar12 = gettext("warning: GREP_COLOR=\'%s\' is deprecated; use GREP_COLORS=\'mt=%s\'");
        error(0,0,uVar12,local_180,local_180);
      }
    }
    FUN_00106f46();
    if (local_200 < 0) {
      local_200 = 0;
    }
    if (((local_200 == 2) || (local_200 == 1)) || (local_200 == 0)) {
      if (DAT_00133180 != '\0') {
        FUN_0010a011();
      }
      if (local_200 == 2) {
        if (DAT_001331a0 == '\x01') {
          cVar5 = FUN_00109f7b(local_1b8,local_1d8);
          if ((cVar5 == '\0') &&
             ((DAT_00133180 == '\0' || (cVar5 = FUN_0010a29a(local_1b8,local_1d8), cVar5 == '\x01'))
             )) {
            bVar4 = false;
          }
          else {
            bVar4 = true;
          }
        }
        else if ((DAT_00133788 == 1) && (DAT_00133181 != '\0')) {
          bVar4 = true;
        }
        else {
          bVar4 = false;
        }
        if (bVar4) {
          FUN_0010a32f(&DAT_00133780,&local_1d8);
          local_1b8 = DAT_00133780;
          local_200 = 0;
        }
      }
      else if (1 < DAT_00133788) {
        local_200 = FUN_0010a4d3(local_200,local_1b8,&local_1d8);
      }
    }
    DAT_001337d8 = (code *)(&PTR_FUN_001327b8)[(long)local_200 * 4];
    DAT_001337e0 = (*(code *)(&PTR_FUN_001327b0)[(long)local_200 * 4])
                             (local_1b8,local_1d8,
                              (long)*(int *)(&DAT_001327ac + (long)local_200 * 0x20),
                              DAT_00133758 != 0 || DAT_0013375c != '\0');
    local_23 = 0;
    local_22 = DAT_00133183;
    local_21 = 0;
    lVar14 = (*DAT_001337d8)(DAT_001337e0,&local_22,1,local_1c0,0);
    DAT_00133831 = (lVar14 == 0) == (bool)DAT_00133851;
    param_1 = param_1 - optind;
    if ((local_1f8 == 0) && (param_1 < 2)) {
      DAT_00133848 = -(uint)(DAT_00133010 == 3);
    }
    else {
      DAT_00133848 = ~local_1f8 >> 0x1f;
    }
    if (DAT_001338b3 != '\0') {
      FUN_0012716d(1,0);
    }
    local_178 = sysconf(0x1e);
    if (((long)local_178 < 1) || (0x3ffffffffffffffb < (long)local_178)) {
                    /* WARNING: Subroutine does not return */
      abort();
    }
    auVar2._8_8_ = 0;
    auVar2._0_8_ = local_178;
    if (SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x18000)) % auVar2,0) == 0) {
      lVar14 = 0x18000;
    }
    else {
      auVar3._8_8_ = 0;
      auVar3._0_8_ = local_178;
      lVar14 = (local_178 - SUB168((ZEXT816(0) << 0x40 | ZEXT816(0x18000)) % auVar3,0)) + 0x18000;
    }
    DAT_001337f8 = local_178 + lVar14 + 8;
    DAT_00133818 = local_178;
    DAT_001337f0 = FUN_00126b10(DAT_001337f8);
    if (((DAT_00133014 & 2) != 0) && (DAT_001337cc == 0)) {
      DAT_001337cc = 1;
    }
    if (param_1 < 1) {
      if ((DAT_00133010 == 3) && (0 < local_1fc)) {
        local_198 = &PTR_DAT_00132720;
        DAT_001337c8 = 1;
      }
      else {
        local_198 = &PTR_DAT_00132730;
      }
    }
    else {
      local_198 = (undefined **)(param_2 + (long)optind * 8);
    }
    local_202 = true;
    do {
      puVar1 = *local_198;
      local_198 = local_198 + 1;
      bVar6 = FUN_0010992b(puVar1);
      local_202 = (bVar6 & local_202) != 0;
    } while (*local_198 != (undefined *)0x0);
    if (DAT_001337c9 != '\0') {
      local_202 = true;
    }
  }
LAB_0010bfff:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_202;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: Fcompile @ 0xc03f

undefined8 * Fcompile(char *param_1,long param_2)

{
  long in_FS_OFFSET;
  size_t local_50;
  char *local_48;
  char *local_40;
  size_t local_38;
  undefined8 local_30;
  void *local_28;
  undefined8 local_20;
  undefined8 *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_48 = (char *)0x0;
  local_50 = 0;
  local_30 = FUN_0010f7c8(1);
  local_40 = param_1;
  do {
    local_28 = rawmemchr(local_40,10);
    local_38 = (long)local_28 - (long)local_40;
    if (DAT_00133182 != '\0') {
      if ((DAT_00133183 == '\n') && (param_1 < local_40)) {
        local_40 = local_40 + -1;
      }
      else {
        if ((long)local_50 <= (long)(local_38 + 1)) {
          free(local_48);
          local_50 = local_38;
          local_48 = (char *)FUN_00126dc6(0,&local_50,2,0xffffffffffffffff,1);
          *local_48 = DAT_00133183;
        }
        memcpy(local_48 + 1,local_40,local_38);
        local_48[local_38 + 1] = DAT_00133183;
        local_40 = local_48;
      }
      local_38 = local_38 + 2;
    }
    FUN_0010cacb(local_30,local_40,local_38);
    local_40 = (char *)((long)local_28 + 1);
  } while (local_40 <= param_1 + param_2);
  free(local_48);
  local_20 = FUN_0010d633(local_30);
  FUN_0010d9b5(local_30);
  local_18 = (undefined8 *)FUN_00126aea(0x28);
  *local_18 = local_30;
  local_18[1] = local_20;
  local_18[2] = param_1;
  local_18[3] = param_2;
  local_18[4] = 0;
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_18;
}




// Function: Fexecute @ 0xc23c

long Fexecute(undefined8 *param_1,void *param_2,long param_3,long *param_4,void *param_5)

{
  char cVar1;
  byte bVar2;
  undefined8 uVar3;
  long lVar4;
  void *pvVar5;
  byte bVar6;
  long in_FS_OFFSET;
  bool bVar7;
  void *local_90;
  long local_88;
  void *local_80;
  long local_78;
  long local_70;
  undefined8 *local_68;
  undefined8 local_60;
  long local_58;
  void *local_50;
  undefined1 local_48 [16];
  long local_38;
  undefined1 local_28 [16];
  long local_18;
  long local_10;
  
  cVar1 = DAT_00133183;
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_60 = *param_1;
  bVar7 = (byte)((DAT_00133182 ^ 1) & DAT_001331a0 & (DAT_001331a2 ^ 1)) != 0;
  bVar6 = (bVar7 || param_5 != (void *)0x0) | DAT_00133181;
  bVar2 = DAT_00133182 ^ 1;
  local_90 = param_2;
  local_80 = param_2;
  local_68 = param_1;
  if (param_5 != (void *)0x0) {
    local_90 = param_5;
    local_80 = param_5;
  }
  do {
    if (((void *)((long)param_2 + param_3) < local_80) ||
       (local_58 = FUN_0010f585(local_60,(long)local_80 - (ulong)DAT_00133182,
                                (long)param_2 + (ulong)DAT_00133182 + (param_3 - (long)local_80),
                                local_48,(bVar2 & bVar6) != 0), local_58 < 0)) {
      lVar4 = -1;
LAB_0010c809:
      if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
        return lVar4;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    local_70 = local_38 - (int)((uint)DAT_00133182 * 2);
    local_88 = 0;
    if ((!bVar7) ||
       (lVar4 = FUN_0010f84a(&local_90,&local_88,local_58 + (long)local_80,param_3 + (long)param_2),
       lVar4 == 0)) {
      local_80 = (void *)((long)local_80 + local_58);
      if (((DAT_00133181 ^ 1) & param_5 != (void *)0x0) == 0) {
        if (DAT_00133182 == 0) {
          if (DAT_00133181 == 1) {
            if ((local_88 == 0) &&
               (local_50 = memrchr(local_90,(int)cVar1,(long)local_80 - (long)local_90),
               local_50 != (void *)0x0)) {
              local_90 = (void *)((long)local_50 + 1);
            }
            if (local_88 < 1) {
              lVar4 = FUN_0010fc0e(local_90,local_80,param_3 + (long)param_2);
            }
            else {
              lVar4 = FUN_0010fbe0((long)local_80 - local_88,param_3 + (long)param_2);
            }
            if (lVar4 == 0) {
              while( true ) {
                lVar4 = FUN_0010fbe0((long)local_80 + local_70,param_3 + (long)param_2);
                if (lVar4 == 0) {
                  if (param_5 != (void *)0x0) goto LAB_0010c7f0;
                  goto LAB_0010c73e;
                }
                if ((param_5 == (void *)0x0) && (DAT_001331a0 != 1)) break;
                if (local_70 == 0) goto LAB_0010c6dc;
                local_70 = local_70 + -1;
                lVar4 = FUN_0010f585(local_60,local_80,local_70,local_28,1);
                if (lVar4 != 0) goto LAB_0010c6dc;
                local_70 = local_18;
              }
              if (local_68[4] == 0) {
                FUN_0010a32f(local_68 + 2,local_68 + 3);
                uVar3 = FUN_00105661(local_68[2],local_68[3],0x10a46,0);
                local_68[4] = uVar3;
              }
              if ((ulong)(local_70 + (long)local_80) < (ulong)((long)param_2 + param_3)) {
                pvVar5 = rawmemchr((void *)((long)local_80 + local_70),(int)cVar1);
                local_78 = (long)pvVar5 + 1;
              }
              else {
                local_78 = (long)param_2 + param_3;
              }
              lVar4 = FUN_00105d23(local_68[4],local_80,local_78 - (long)local_80,param_4,0);
              if (-1 < lVar4) goto LAB_0010c79b;
              local_80 = (void *)(local_78 + -1);
            }
LAB_0010c6dc:
            lVar4 = FUN_0010fbb2(local_80,param_3 + (long)param_2);
            local_90 = (void *)((long)local_80 + lVar4);
            local_80 = local_90;
            goto LAB_0010c70b;
          }
LAB_0010c73e:
          if ((ulong)(local_70 + (long)local_80) < (ulong)((long)param_2 + param_3)) {
            pvVar5 = rawmemchr((void *)((long)local_80 + local_70),(int)cVar1);
            local_78 = (long)pvVar5 + 1;
          }
          else {
            local_78 = (long)param_2 + param_3;
          }
LAB_0010c79b:
          pvVar5 = memrchr(param_2,(int)cVar1,(long)local_80 - (long)param_2);
          local_80 = param_2;
          if (pvVar5 != (void *)0x0) {
            local_80 = (void *)((long)pvVar5 + 1);
          }
          local_70 = local_78 - (long)local_80;
        }
        else {
          local_70 = local_70 + (ulong)(param_5 == (void *)0x0);
        }
      }
LAB_0010c7f0:
      *param_4 = local_70;
      lVar4 = (long)local_80 - (long)param_2;
      goto LAB_0010c809;
    }
    local_80 = (void *)((long)local_90 + -1);
LAB_0010c70b:
    local_80 = (void *)((long)local_80 + 1);
  } while( true );
}




// Function: U @ 0xc845

void U(char param_1)

{
  FUN_0010f64d((int)param_1);
  return;
}




// Function: tr @ 0xc863

char tr(long param_1,char param_2)

{
  byte bVar1;
  
  if (param_1 != 0) {
    bVar1 = FUN_0010c845((int)param_2);
    param_2 = *(char *)(param_1 + (ulong)bVar1);
  }
  return param_2;
}




// Function: kwsalloc @ 0xc89b

long kwsalloc(undefined8 param_1)

{
  long lVar1;
  long lVar2;
  
  lVar2 = FUN_00126aea(0x9a0);
  _obstack_begin(lVar2,0,0,FUN_00126aea,PTR_free_00132fc8);
  *(undefined8 *)(lVar2 + 0x58) = 0;
  if ((ulong)(*(long *)(lVar2 + 0x20) - *(long *)(lVar2 + 0x18)) < 0x40) {
    _obstack_newchunk(lVar2,0x40);
  }
  *(long *)(lVar2 + 0x18) = *(long *)(lVar2 + 0x18) + 0x40;
  lVar1 = *(long *)(lVar2 + 0x10);
  if (lVar1 == *(long *)(lVar2 + 0x18)) {
    *(byte *)(lVar2 + 0x50) = *(byte *)(lVar2 + 0x50) | 2;
  }
  *(ulong *)(lVar2 + 0x18) =
       ~*(ulong *)(lVar2 + 0x30) & *(long *)(lVar2 + 0x18) + *(long *)(lVar2 + 0x30);
  if ((ulong)(*(long *)(lVar2 + 0x20) - *(long *)(lVar2 + 8)) <
      (ulong)(*(long *)(lVar2 + 0x18) - *(long *)(lVar2 + 8))) {
    *(undefined8 *)(lVar2 + 0x18) = *(undefined8 *)(lVar2 + 0x20);
  }
  *(undefined8 *)(lVar2 + 0x10) = *(undefined8 *)(lVar2 + 0x18);
  *(long *)(lVar2 + 0x60) = lVar1;
  **(undefined8 **)(lVar2 + 0x60) = 0;
  *(undefined8 *)(*(long *)(lVar2 + 0x60) + 8) = 0;
  *(undefined8 *)(*(long *)(lVar2 + 0x60) + 0x10) = 0;
  *(undefined8 *)(*(long *)(lVar2 + 0x60) + 0x18) = 0;
  *(undefined8 *)(*(long *)(lVar2 + 0x60) + 0x20) = 0;
  *(undefined8 *)(*(long *)(lVar2 + 0x60) + 0x28) = 0;
  *(undefined8 *)(*(long *)(lVar2 + 0x60) + 0x30) = 0;
  *(undefined8 *)(lVar2 + 0x68) = 0x7fffffffffffffff;
  *(undefined8 *)(lVar2 + 0x970) = 0;
  *(undefined8 *)(lVar2 + 0x980) = param_1;
  *(code **)(lVar2 + 0x998) = FUN_0010f50c;
  return lVar2;
}




// Function: kwsincr @ 0xcacb

void kwsincr(long param_1,byte *param_2,long param_3)

{
  long lVar1;
  long lVar2;
  long *plVar3;
  long lVar4;
  long *plVar5;
  long lVar6;
  byte bVar7;
  undefined1 uVar8;
  long in_FS_OFFSET;
  bool bVar9;
  long local_170;
  byte *local_168;
  long *local_150;
  long *local_148;
  long local_140;
  long *local_138;
  int local_a8 [12];
  long *local_78 [13];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_150 = *(long **)(param_1 + 0x60);
  lVar1 = *(long *)(param_1 + 0x980);
  bVar9 = *(code **)(param_1 + 0x998) == FUN_0010f061;
  local_170 = param_3;
  local_168 = param_2;
  if (bVar9) {
    local_168 = param_2 + param_3;
    local_170 = param_3;
  }
  do {
    if (local_170 == 0) {
      if (*local_150 == 0) {
        *local_150 = *(long *)(param_1 + 0x58) * 2 + 1;
      }
      *(long *)(param_1 + 0x58) = *(long *)(param_1 + 0x58) + 1;
      if (local_150[5] < *(long *)(param_1 + 0x68)) {
        *(long *)(param_1 + 0x68) = local_150[5];
      }
      if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    if (bVar9) {
      bVar7 = local_168[-1];
      local_168 = local_168 + -1;
    }
    else {
      bVar7 = *local_168;
      local_168 = local_168 + 1;
    }
    if (lVar1 != 0) {
      bVar7 = *(byte *)(lVar1 + (ulong)bVar7);
    }
    local_148 = (long *)local_150[1];
    local_78[0] = local_150 + 1;
    local_a8[0] = 0;
    local_140 = 1;
    while ((local_148 != (long *)0x0 && (bVar7 != *(byte *)(local_148 + 3)))) {
      local_78[local_140] = local_148;
      if (bVar7 < *(byte *)(local_148 + 3)) {
        local_a8[local_140] = 0;
        local_148 = (long *)*local_148;
        local_140 = local_140 + 1;
      }
      else {
        local_a8[local_140] = 1;
        local_148 = (long *)local_148[1];
        local_140 = local_140 + 1;
      }
    }
    if (local_148 == (long *)0x0) {
      if ((ulong)(*(long *)(param_1 + 0x20) - *(long *)(param_1 + 0x18)) < 0x20) {
        _obstack_newchunk(param_1,0x20);
      }
      *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 0x20;
      local_148 = *(long **)(param_1 + 0x10);
      if (local_148 == *(long **)(param_1 + 0x18)) {
        *(byte *)(param_1 + 0x50) = *(byte *)(param_1 + 0x50) | 2;
      }
      *(ulong *)(param_1 + 0x18) =
           ~*(ulong *)(param_1 + 0x30) & *(long *)(param_1 + 0x18) + *(long *)(param_1 + 0x30);
      if ((ulong)(*(long *)(param_1 + 0x20) - *(long *)(param_1 + 8)) <
          (ulong)(*(long *)(param_1 + 0x18) - *(long *)(param_1 + 8))) {
        *(undefined8 *)(param_1 + 0x18) = *(undefined8 *)(param_1 + 0x20);
      }
      *(undefined8 *)(param_1 + 0x10) = *(undefined8 *)(param_1 + 0x18);
      *local_148 = 0;
      local_148[1] = 0;
      if ((ulong)(*(long *)(param_1 + 0x20) - *(long *)(param_1 + 0x18)) < 0x40) {
        _obstack_newchunk(param_1,0x40);
      }
      *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 0x40;
      lVar2 = *(long *)(param_1 + 0x10);
      if (lVar2 == *(long *)(param_1 + 0x18)) {
        *(byte *)(param_1 + 0x50) = *(byte *)(param_1 + 0x50) | 2;
      }
      *(ulong *)(param_1 + 0x18) =
           ~*(ulong *)(param_1 + 0x30) & *(long *)(param_1 + 0x18) + *(long *)(param_1 + 0x30);
      if ((ulong)(*(long *)(param_1 + 0x20) - *(long *)(param_1 + 8)) <
          (ulong)(*(long *)(param_1 + 0x18) - *(long *)(param_1 + 8))) {
        *(undefined8 *)(param_1 + 0x18) = *(undefined8 *)(param_1 + 0x20);
      }
      *(undefined8 *)(param_1 + 0x10) = *(undefined8 *)(param_1 + 0x18);
      local_148[2] = lVar2;
      *(undefined8 *)local_148[2] = 0;
      *(undefined8 *)(local_148[2] + 8) = 0;
      *(long **)(local_148[2] + 0x10) = local_150;
      *(undefined8 *)(local_148[2] + 0x18) = 0;
      *(undefined8 *)(local_148[2] + 0x20) = 0;
      *(long *)(local_148[2] + 0x28) = local_150[5] + 1;
      *(undefined8 *)(local_148[2] + 0x30) = 0;
      *(byte *)(local_148 + 3) = bVar7;
      *(undefined1 *)((long)local_148 + 0x19) = 0;
      local_140 = local_140 + -1;
      if (local_a8[local_140] == 0) {
        *local_78[local_140] = (long)local_148;
      }
      else {
        local_78[local_140][1] = (long)local_148;
      }
      while ((local_140 != 0 && (*(char *)((long)local_78[local_140] + 0x19) == '\0'))) {
        if (local_a8[local_140] == 0) {
          *(char *)((long)local_78[local_140] + 0x19) =
               *(char *)((long)local_78[local_140] + 0x19) + -1;
        }
        else {
          *(char *)((long)local_78[local_140] + 0x19) =
               *(char *)((long)local_78[local_140] + 0x19) + '\x01';
        }
        local_140 = local_140 + -1;
      }
      if (local_140 != 0) {
        if (local_a8[local_140] == 0) {
          plVar3 = local_78[local_140];
          *(char *)((long)plVar3 + 0x19) = *(char *)((long)plVar3 + 0x19) + -1;
          if (*(char *)((long)plVar3 + 0x19) == '\0') goto LAB_0010d19a;
LAB_0010d1d3:
          if (*(char *)((long)local_78[local_140] + 0x19) == -2) {
            if (local_a8[local_140 + 1] == 0) {
              plVar3 = local_78[local_140];
              local_138 = (long *)*plVar3;
              lVar2 = local_138[1];
              local_138[1] = (long)plVar3;
              *plVar3 = lVar2;
              *(undefined1 *)((long)plVar3 + 0x19) = 0;
              *(undefined1 *)((long)local_138 + 0x19) = *(undefined1 *)((long)plVar3 + 0x19);
            }
            else {
              if (local_a8[local_140 + 1] != 1) {
                    /* WARNING: Subroutine does not return */
                abort();
              }
              plVar3 = local_78[local_140];
              lVar2 = *plVar3;
              local_138 = *(long **)(lVar2 + 8);
              lVar6 = local_138[1];
              lVar4 = *local_138;
              *local_138 = lVar2;
              *(long *)(lVar2 + 8) = lVar4;
              local_138[1] = (long)plVar3;
              *plVar3 = lVar6;
              if (*(char *)((long)local_138 + 0x19) == '\x01') {
                uVar8 = 0xff;
              }
              else {
                uVar8 = 0;
              }
              *(undefined1 *)(lVar2 + 0x19) = uVar8;
              *(bool *)((long)plVar3 + 0x19) = *(char *)((long)local_138 + 0x19) == -1;
              *(undefined1 *)((long)local_138 + 0x19) = 0;
            }
          }
          else {
            if (*(char *)((long)local_78[local_140] + 0x19) != '\x02') {
                    /* WARNING: Subroutine does not return */
              abort();
            }
            if (local_a8[local_140 + 1] == 0) {
              plVar3 = local_78[local_140];
              plVar5 = (long *)plVar3[1];
              local_138 = (long *)*plVar5;
              lVar2 = *local_138;
              lVar6 = local_138[1];
              *local_138 = (long)plVar3;
              plVar3[1] = lVar2;
              local_138[1] = (long)plVar5;
              *plVar5 = lVar6;
              if (*(char *)((long)local_138 + 0x19) == '\x01') {
                uVar8 = 0xff;
              }
              else {
                uVar8 = 0;
              }
              *(undefined1 *)((long)plVar3 + 0x19) = uVar8;
              *(bool *)((long)plVar5 + 0x19) = *(char *)((long)local_138 + 0x19) == -1;
              *(undefined1 *)((long)local_138 + 0x19) = 0;
            }
            else {
              if (local_a8[local_140 + 1] != 1) {
                    /* WARNING: Subroutine does not return */
                abort();
              }
              plVar3 = local_78[local_140];
              local_138 = (long *)plVar3[1];
              lVar2 = *local_138;
              *local_138 = (long)plVar3;
              plVar3[1] = lVar2;
              *(undefined1 *)((long)plVar3 + 0x19) = 0;
              *(undefined1 *)((long)local_138 + 0x19) = *(undefined1 *)((long)plVar3 + 0x19);
            }
          }
          if (local_a8[local_140 + -1] == 0) {
            *local_78[local_140 + -1] = (long)local_138;
          }
          else {
            local_78[local_140 + -1][1] = (long)local_138;
          }
        }
        else {
LAB_0010d19a:
          if (local_a8[local_140] == 1) {
            plVar3 = local_78[local_140];
            *(char *)((long)plVar3 + 0x19) = *(char *)((long)plVar3 + 0x19) + '\x01';
            if (*(char *)((long)plVar3 + 0x19) != '\0') goto LAB_0010d1d3;
          }
        }
      }
    }
    local_150 = (long *)local_148[2];
    local_170 = local_170 + -1;
  } while( true );
}




// Function: kwswords @ 0xd633

undefined8 kwswords(long param_1)

{
  return *(undefined8 *)(param_1 + 0x58);
}




// Function: enqueue @ 0xd649

void enqueue(undefined8 *param_1,long *param_2)

{
  long lVar1;
  
  if (param_1 != (undefined8 *)0x0) {
    enqueue(*param_1,param_2);
    enqueue(param_1[1],param_2);
    lVar1 = *param_2;
    *(undefined8 *)(lVar1 + 0x18) = param_1[2];
    *param_2 = *(long *)(lVar1 + 0x18);
  }
  return;
}




// Function: treefails @ 0xd6b4

void treefails(undefined8 *param_1,long param_2,undefined8 param_3,char param_4)

{
  long local_28;
  undefined8 *local_10;
  
  if (param_1 != (undefined8 *)0x0) {
    treefails(*param_1,param_2,param_3,param_4);
    treefails(param_1[1],param_2,param_3,param_4);
    for (local_28 = param_2; local_28 != 0; local_28 = *(long *)(local_28 + 0x20)) {
      local_10 = *(undefined8 **)(local_28 + 8);
      while ((local_10 != (undefined8 *)0x0 && (*(char *)(param_1 + 3) != *(char *)(local_10 + 3))))
      {
        if (*(byte *)(param_1 + 3) < *(byte *)(local_10 + 3)) {
          local_10 = (undefined8 *)*local_10;
        }
        else {
          local_10 = (undefined8 *)local_10[1];
        }
      }
      if (local_10 != (undefined8 *)0x0) {
        *(undefined8 *)(param_1[2] + 0x20) = local_10[2];
        if (param_4 == '\x01') {
          return;
        }
        if (*(long *)local_10[2] == 0) {
          return;
        }
        if (*(long *)param_1[2] != 0) {
          return;
        }
        *(undefined8 *)param_1[2] = 0xffffffffffffffff;
        return;
      }
    }
    *(undefined8 *)(param_1[2] + 0x20) = param_3;
  }
  return;
}




// Function: treedelta @ 0xd7f4

void treedelta(undefined8 *param_1,long param_2,long param_3)

{
  if (param_1 != (undefined8 *)0x0) {
    treedelta(*param_1,param_2,param_3);
    treedelta(param_1[1],param_2,param_3);
    if (param_2 < (long)(ulong)*(byte *)(param_3 + (ulong)*(byte *)(param_1 + 3))) {
      *(char *)(param_3 + (ulong)*(byte *)(param_1 + 3)) = (char)param_2;
    }
  }
  return;
}




// Function: hasevery @ 0xd883

undefined8 hasevery(undefined8 *param_1,undefined8 *param_2)

{
  char cVar1;
  uint uVar2;
  undefined8 uVar3;
  undefined8 *puVar4;
  undefined8 *local_10;
  
  if (param_2 == (undefined8 *)0x0) {
    uVar3 = 1;
  }
  else {
    cVar1 = hasevery(param_1,*param_2);
    if (cVar1 == '\x01') {
      uVar2 = hasevery(param_1,param_2[1]);
      puVar4 = (undefined8 *)(ulong)(uVar2 ^ 1);
      local_10 = param_1;
      if ((char)(uVar2 ^ 1) == '\0') {
        while ((local_10 != (undefined8 *)0x0 &&
               (puVar4 = (undefined8 *)0x0, *(char *)(param_2 + 3) != *(char *)(local_10 + 3)))) {
          if (*(byte *)(param_2 + 3) < *(byte *)(local_10 + 3)) {
            puVar4 = (undefined8 *)*local_10;
            local_10 = puVar4;
          }
          else {
            puVar4 = (undefined8 *)local_10[1];
            local_10 = puVar4;
          }
        }
        uVar3 = CONCAT71((int7)((ulong)puVar4 >> 8),local_10 != (undefined8 *)0x0);
      }
      else {
        uVar3 = 0;
      }
    }
    else {
      uVar3 = 0;
    }
  }
  return uVar3;
}




// Function: treenext @ 0xd943

void treenext(undefined8 *param_1,long param_2)

{
  if (param_1 != (undefined8 *)0x0) {
    treenext(*param_1,param_2);
    treenext(param_1[1],param_2);
    *(undefined8 *)((ulong)*(byte *)(param_1 + 3) * 8 + param_2) = param_1[2];
  }
  return;
}




// Function: kwsprep @ 0xd9b5

/* WARNING: Removing unreachable block (ram,0x0010dd2f) */
/* WARNING: Removing unreachable block (ram,0x0010dd43) */

void kwsprep(undefined8 *param_1)

{
  char cVar1;
  undefined1 uVar2;
  byte bVar3;
  long lVar4;
  undefined8 *puVar5;
  undefined8 *puVar6;
  long in_FS_OFFSET;
  bool bVar7;
  bool bVar8;
  uint local_a1c;
  uint local_a18;
  uint local_a14;
  uint local_a10;
  int local_a0c;
  long *local_a08;
  long *local_a00;
  long local_9f8;
  long local_9f0;
  long local_9e8;
  long local_9e0;
  long local_9d8;
  undefined8 *local_9d0;
  undefined8 *local_9c8;
  undefined8 *local_9c0;
  ulong local_9b8;
  undefined8 *local_9b0;
  undefined8 *local_9a8;
  long local_9a0;
  undefined8 *local_998;
  undefined8 *local_990;
  undefined8 local_988;
  undefined8 *local_980;
  undefined8 *local_978;
  undefined8 *local_970;
  ulong local_968;
  undefined8 *local_960;
  undefined8 *local_958;
  long local_950;
  undefined8 *local_948;
  undefined8 *local_940;
  ulong local_938;
  undefined8 *local_930;
  undefined8 *local_928;
  long local_920;
  undefined8 local_918 [256];
  undefined8 local_118 [33];
  long local_10;
  
  bVar3 = 0;
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_9d8 = param_1[0x130];
  if (local_9d8 == 0) {
    local_9d0 = param_1 + 0xe;
  }
  else {
    local_9d0 = local_118;
  }
  bVar7 = param_1[0xb] == 1;
  if (bVar7) {
    local_a08 = (long *)param_1[0xc];
    for (local_a00 = local_a08; local_a00 != (long *)0x0; local_a00 = (long *)local_a00[3]) {
      FUN_0010d649(local_a00[1],&local_a08);
    }
    local_9b8 = param_1[0xd];
    local_9c8 = param_1;
    local_9c0 = param_1;
    local_9b0 = param_1;
    if ((ulong)(param_1[4] - param_1[3]) < local_9b8) {
      _obstack_newchunk(param_1,local_9b8);
    }
    local_9c0[3] = local_9c0[3] + local_9b8;
    local_9a8 = local_9c8;
    local_9a0 = local_9c8[2];
    if (local_9a0 == local_9c8[3]) {
      *(byte *)(local_9c8 + 10) = *(byte *)(local_9c8 + 10) | 2;
    }
    local_9c8[3] = ~local_9c8[6] & local_9c8[3] + local_9c8[6];
    if ((ulong)(local_9c8[4] - local_9c8[1]) < (ulong)(local_9c8[3] - local_9c8[1])) {
      local_9c8[3] = local_9c8[4];
    }
    local_9c8[2] = local_9c8[3];
    param_1[0x12e] = local_9a0;
    local_a00 = (long *)param_1[0xc];
    for (local_9f8 = 0; local_9f8 < (long)param_1[0xd]; local_9f8 = local_9f8 + 1) {
      *(undefined1 *)(local_9f8 + param_1[0x12e]) =
           *(undefined1 *)(*(long *)((long)local_a00 + 8) + 0x18);
      local_a00 = *(long **)((long)local_a00 + 0x18);
    }
    local_998 = (undefined8 *)FUN_0010c89b(param_1[0x130]);
    local_998[0x133] = FUN_0010f061;
    FUN_0010cacb(local_998,param_1[0x12e],param_1[0xd]);
    local_988 = 0;
    local_990 = param_1;
    _obstack_free(param_1,0);
    puVar5 = local_998;
    puVar6 = param_1;
    for (lVar4 = 0x134; lVar4 != 0; lVar4 = lVar4 + -1) {
      *puVar6 = *puVar5;
      puVar5 = puVar5 + (ulong)bVar3 * -2 + 1;
      puVar6 = puVar6 + (ulong)bVar3 * -2 + 1;
    }
    free(local_998);
  }
  lVar4 = param_1[0xd];
  if (0xff < lVar4) {
    lVar4 = 0xff;
  }
  memset(local_9d0,(int)lVar4,0x100);
  local_a08 = (long *)param_1[0xc];
  for (local_a00 = local_a08; local_a00 != (long *)0x0; local_a00 = (long *)local_a00[3]) {
    FUN_0010d649(local_a00[1],&local_a08);
    FUN_0010d7f4(local_a00[1],local_a00[5],local_9d0);
    FUN_0010d6b4(local_a00[1],local_a00[4],param_1[0xc],bVar7);
    if (bVar7) {
      local_a00[6] = param_1[0xd];
      local_a00[7] = param_1[0xd];
      for (local_9f0 = local_a00[4]; local_9f0 != 0; local_9f0 = *(long *)(local_9f0 + 0x20)) {
        cVar1 = FUN_0010d883(*(undefined8 *)(local_9f0 + 8),local_a00[1]);
        if ((cVar1 != '\x01') &&
           (local_a00[5] - *(long *)(local_9f0 + 0x28) < *(long *)(local_9f0 + 0x30))) {
          *(long *)(local_9f0 + 0x30) = local_a00[5] - *(long *)(local_9f0 + 0x28);
        }
        if ((*local_a00 != 0) &&
           (local_a00[5] - *(long *)(local_9f0 + 0x28) < *(long *)(local_9f0 + 0x38))) {
          *(long *)(local_9f0 + 0x38) = local_a00[5] - *(long *)(local_9f0 + 0x28);
        }
      }
    }
  }
  if (bVar7) {
    for (local_a00 = *(long **)(param_1[0xc] + 0x18); local_a00 != (long *)0x0;
        local_a00 = (long *)local_a00[3]) {
      if (*(long *)(local_a00[2] + 0x38) < local_a00[7]) {
        local_a00[7] = *(long *)(local_a00[2] + 0x38);
      }
      if (local_a00[7] < local_a00[6]) {
        local_a00[6] = local_a00[7];
      }
    }
  }
  if (local_9d8 == 0) {
    local_980 = param_1 + 0x2e;
  }
  else {
    local_980 = local_918;
  }
  memset(local_980,0,0x800);
  FUN_0010d943(*(undefined8 *)(param_1[0xc] + 8),local_980);
  local_a1c = 0xfffffffe;
  local_a18 = 0xffffffff;
  for (local_a14 = 0; (int)local_a14 < 0x100; local_a14 = local_a14 + 1) {
    local_a10 = local_a14;
    if (local_9d8 != 0) {
      bVar3 = FUN_0010c845((int)*(char *)(local_9d8 + (int)local_a14));
      local_a10 = (uint)bVar3;
      param_1[(long)(int)local_a14 + 0x2e] = local_980[(int)local_a10];
    }
    if (param_1[(long)(int)local_a14 + 0x2e] != 0) {
      if ((int)local_a1c < -1) {
        local_a1c = local_a10;
        local_a18 = local_a14;
      }
      else if (local_a1c == local_a10) {
        bVar8 = local_a18 != local_a10;
        local_a18 = local_a14;
        if (bVar8) {
          local_a18 = 0xffffffff;
        }
      }
      else if ((local_a14 == local_a10) && (local_a1c == local_a18)) {
        local_a18 = local_a14;
      }
      else {
        local_a1c = 0xffffffff;
      }
    }
  }
  *(uint *)(param_1 + 0x131) = local_a1c;
  *(uint *)((long)param_1 + 0x98c) = local_a18;
  if (bVar7) {
    local_968 = param_1[0xd];
    local_978 = param_1;
    local_970 = param_1;
    local_960 = param_1;
    if ((ulong)(param_1[4] - param_1[3]) < local_968) {
      _obstack_newchunk(param_1,local_968);
    }
    local_970[3] = local_970[3] + local_968;
    local_958 = local_978;
    local_950 = local_978[2];
    if (local_950 == local_978[3]) {
      *(byte *)(local_978 + 10) = *(byte *)(local_978 + 10) | 2;
    }
    local_978[3] = ~local_978[6] & local_978[3] + local_978[6];
    if ((ulong)(local_978[4] - local_978[1]) < (ulong)(local_978[3] - local_978[1])) {
      local_978[3] = local_978[4];
    }
    local_978[2] = local_978[3];
    param_1[0x12e] = local_950;
    local_a00 = (long *)param_1[0xc];
    for (local_9e8 = param_1[0xd]; 0 < local_9e8; local_9e8 = local_9e8 + -1) {
      *(undefined1 *)(local_9e8 + -1 + param_1[0x12e]) = *(undefined1 *)(local_a00[1] + 0x18);
      local_a00 = (long *)local_a00[3];
    }
    if (1 < (long)param_1[0xd]) {
      local_938 = (param_1[0xd] + -1) * 8;
      local_948 = param_1;
      local_940 = param_1;
      local_930 = param_1;
      if ((ulong)(param_1[4] - param_1[3]) < local_938) {
        _obstack_newchunk(param_1,local_938);
      }
      local_940[3] = local_940[3] + local_938;
      local_928 = local_948;
      local_920 = local_948[2];
      if (local_920 == local_948[3]) {
        *(byte *)(local_948 + 10) = *(byte *)(local_948 + 10) | 2;
      }
      local_948[3] = ~local_948[6] & local_948[3] + local_948[6];
      if ((ulong)(local_948[4] - local_948[1]) < (ulong)(local_948[3] - local_948[1])) {
        local_948[3] = local_948[4];
      }
      local_948[2] = local_948[3];
      param_1[0x12f] = local_920;
      local_a00 = *(long **)(param_1[0xc] + 0x18);
      for (local_9e0 = 0; local_9e0 < param_1[0xd] + -1; local_9e0 = local_9e0 + 1) {
        *(long *)(param_1[0x12f] + local_9e0 * 8) = local_a00[6];
        local_a00 = (long *)local_a00[3];
      }
      uVar2 = FUN_0010c863(local_9d8,(int)*(char *)(param_1[0xd] + -2 + param_1[0x12e]));
      *(undefined1 *)(param_1 + 0x132) = uVar2;
    }
  }
  if (local_9d8 != 0) {
    for (local_a0c = 0; local_a0c < 0x100; local_a0c = local_a0c + 1) {
      bVar3 = FUN_0010c845((int)*(char *)(local_9d8 + local_a0c));
      *(undefined1 *)((long)param_1 + (long)local_a0c + 0x70) =
           *(undefined1 *)((long)local_9d0 + (ulong)bVar3);
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: bm_delta2_search @ 0xe78b

undefined8
bm_delta2_search(ulong *param_1,ulong param_2,long param_3,long param_4,undefined8 param_5,char param_6,
            char param_7,long param_8,long param_9)

{
  char cVar1;
  char cVar2;
  byte bVar3;
  ulong local_38;
  long local_30;
  long local_28;
  long local_20;
  
  local_38 = *param_1;
  local_28 = 0;
  local_30 = param_4;
  do {
    local_20 = 2;
    cVar1 = FUN_0010c863(param_5,(int)*(char *)(local_38 - 2));
    if (param_7 == cVar1) {
      do {
        local_20 = local_20 + 1;
        if (local_30 < local_20) break;
        cVar1 = FUN_0010c863(param_5,(int)*(char *)(local_38 - local_20));
        cVar2 = FUN_0010c863(param_5,(int)*(char *)(param_3 - local_20));
      } while (cVar1 == cVar2);
      if (local_30 < local_20) {
        local_20 = local_28 + local_30;
        do {
          local_20 = local_20 + 1;
          if (param_4 < local_20) break;
          cVar1 = FUN_0010c863(param_5,(int)*(char *)(local_38 - local_20));
          cVar2 = FUN_0010c863(param_5,(int)*(char *)(param_3 - local_20));
        } while (cVar1 == cVar2);
        if (param_4 < local_20) {
          *param_1 = local_38 - param_4;
          return 1;
        }
      }
    }
    local_30 = *(long *)(local_20 * 8 + -0x10 + *(long *)(param_9 + 0x978));
    local_38 = local_38 + local_30;
    if (param_2 < local_38) goto LAB_0010e9ac;
    cVar1 = FUN_0010c863(param_5,(int)*(char *)(local_38 - 1));
    if (param_6 != cVar1) {
      if (param_8 != 0) {
        bVar3 = FUN_0010c845((int)*(char *)(local_38 - 1));
        local_38 = local_38 + *(byte *)(param_8 + (ulong)bVar3);
      }
LAB_0010e9ac:
      *param_1 = local_38;
      return 0;
    }
    local_28 = local_20 + -1;
  } while( true );
}




// Function: memchr_kwset @ 0xe9c2

char * memchr_kwset(char *param_1,long param_2,long param_3)

{
  byte bVar1;
  char *pcVar2;
  char *local_30;
  
  pcVar2 = param_1 + param_2;
  local_30 = param_1;
  if (*(int *)(param_3 + 0x98c) < 0) {
    for (; local_30 < pcVar2; local_30 = local_30 + 1) {
      bVar1 = FUN_0010c845((int)*local_30);
      if (*(long *)(param_3 + ((long)(int)(uint)bVar1 + 0x2e) * 8) != 0) {
        return local_30;
      }
    }
LAB_0010eada:
    pcVar2 = (char *)0x0;
  }
  else {
    do {
      if (pcVar2 <= local_30) goto LAB_0010eada;
      bVar1 = FUN_0010c845((int)*local_30);
      if (*(long *)(param_3 + ((long)(int)(uint)bVar1 + 0x2e) * 8) != 0) {
        return local_30;
      }
      local_30 = local_30 + 1;
    } while ((ulong)local_30 % 0x10 != 0);
    pcVar2 = (char *)FUN_001222e7(local_30,*(undefined4 *)(param_3 + 0x988),
                                  *(undefined4 *)(param_3 + 0x98c),(long)pcVar2 - (long)local_30);
  }
  return pcVar2;
}




// Function: bmexec_trans @ 0xeae1

long bmexec_trans(long param_1,long param_2,long param_3)

{
  char cVar1;
  char cVar2;
  byte bVar3;
  char cVar4;
  long lVar5;
  long in_FS_OFFSET;
  uint local_58;
  ulong local_50;
  long local_48;
  long local_40;
  undefined8 local_38;
  long local_30;
  long local_28;
  ulong local_20;
  ulong local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_40 = *(long *)(param_1 + 0x68);
  local_38 = *(undefined8 *)(param_1 + 0x980);
  if (local_40 == 0) {
    param_2 = 0;
  }
  else if (param_3 < local_40) {
    param_2 = -1;
  }
  else if (local_40 == 1) {
    local_50 = FUN_0010e9c2(param_2,param_3,param_1);
    if (local_50 == 0) {
      param_2 = -1;
    }
    else {
      param_2 = local_50 - param_2;
    }
  }
  else {
    local_30 = param_1 + 0x70;
    local_28 = local_40 + *(long *)(param_1 + 0x970);
    local_50 = param_2 + local_40;
    cVar2 = (char)*(undefined4 *)(param_1 + 0x988);
    cVar1 = *(char *)(param_1 + 0x990);
    local_48 = local_40 * 0xc;
    if ((SEXT816(local_48) == SEXT816(local_40) * SEXT816(0xc)) && (local_48 < param_3)) {
      local_20 = param_2 + param_3 + local_40 * -0xb;
LAB_0010ef5d:
      do {
        if (local_20 < local_50) goto LAB_0010ef6e;
        local_18 = local_50;
        bVar3 = FUN_0010c845((int)*(char *)(local_50 - 1));
        local_50 = (long)(int)(uint)*(byte *)(local_30 + (ulong)bVar3) + local_50;
        bVar3 = FUN_0010c845((int)*(char *)(local_50 - 1));
        local_50 = (long)(int)(uint)*(byte *)(local_30 + (ulong)bVar3) + local_50;
        if (*(byte *)(local_30 + (ulong)bVar3) != 0) {
          bVar3 = FUN_0010c845((int)*(char *)(local_50 - 1));
          local_50 = (long)(int)(uint)*(byte *)(local_30 + (ulong)bVar3) + local_50;
          bVar3 = FUN_0010c845((int)*(char *)(local_50 - 1));
          local_50 = (long)(int)(uint)*(byte *)(local_30 + (ulong)bVar3) + local_50;
          bVar3 = FUN_0010c845((int)*(char *)(local_50 - 1));
          local_50 = (long)(int)(uint)*(byte *)(local_30 + (ulong)bVar3) + local_50;
          if (*(byte *)(local_30 + (ulong)bVar3) != 0) {
            bVar3 = FUN_0010c845((int)*(char *)(local_50 - 1));
            local_50 = (long)(int)(uint)*(byte *)(local_30 + (ulong)bVar3) + local_50;
            bVar3 = FUN_0010c845((int)*(char *)(local_50 - 1));
            local_50 = (long)(int)(uint)*(byte *)(local_30 + (ulong)bVar3) + local_50;
            bVar3 = FUN_0010c845((int)*(char *)(local_50 - 1));
            local_50 = (long)(int)(uint)*(byte *)(local_30 + (ulong)bVar3) + local_50;
            if (*(byte *)(local_30 + (ulong)bVar3) != 0) {
              bVar3 = FUN_0010c845((int)*(char *)(local_50 - 1));
              local_50 = (long)(int)(uint)*(byte *)(local_30 + (ulong)bVar3) + local_50;
              bVar3 = FUN_0010c845((int)*(char *)(local_50 - 1));
              local_50 = (long)(int)(uint)*(byte *)(local_30 + (ulong)bVar3) + local_50;
              if (0x7f < (long)(local_50 - local_18)) goto LAB_0010ef5d;
              local_50 = local_50 - 1;
              lVar5 = FUN_0010e9c2(local_50,(param_2 + param_3) - local_50,param_1);
              if (lVar5 == 0) {
                param_2 = -1;
                local_50 = 0;
                goto LAB_0010f04b;
              }
              local_50 = lVar5 + 1;
              if (local_20 <= local_50) goto LAB_0010ef6e;
            }
          }
        }
        cVar4 = FUN_0010e78b(&local_50,local_20,local_28,local_40,local_38,(int)cVar2,(int)cVar1,
                             local_30,param_1);
      } while (cVar4 == '\0');
      param_2 = local_50 - param_2;
    }
    else {
LAB_0010ef6e:
      local_20 = param_2 + param_3;
      bVar3 = FUN_0010c845((int)*(char *)(local_50 - 1));
      local_58 = (uint)*(byte *)(local_30 + (ulong)bVar3);
      do {
        if ((long)(local_20 - local_50) < (long)(int)local_58) {
          param_2 = -1;
          goto LAB_0010f04b;
        }
        local_50 = (long)(int)local_58 + local_50;
        bVar3 = FUN_0010c845((int)*(char *)(local_50 - 1));
        local_58 = (uint)*(byte *)(local_30 + (ulong)bVar3);
      } while ((local_58 != 0) ||
              (cVar4 = FUN_0010e78b(&local_50,local_20,local_28,local_40,local_38,(int)cVar2,
                                    (int)cVar1,0,param_1), cVar4 == '\0'));
      param_2 = local_50 - param_2;
    }
  }
LAB_0010f04b:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return param_2;
}




// Function: bmexec @ 0xf061

undefined8 bmexec(long param_1,undefined8 param_2,undefined8 param_3,undefined8 *param_4)

{
  undefined8 uVar1;
  
  if (*(long *)(param_1 + 0x980) == 0) {
    uVar1 = FUN_0010eae1(param_1,param_2,param_3);
  }
  else {
    uVar1 = FUN_0010eae1(param_1,param_2,param_3);
  }
  *param_4 = 0;
  param_4[1] = uVar1;
  param_4[2] = *(undefined8 *)(param_1 + 0x68);
  return uVar1;
}




// Function: acexec_trans @ 0xf0f4

long acexec_trans(long param_1,char *param_2,long param_3,long *param_4,char param_5)

{
  int iVar1;
  undefined8 uVar2;
  byte bVar3;
  long lVar4;
  char *pcVar5;
  char *pcVar6;
  long *local_50;
  long *local_48;
  char *local_40;
  ulong local_38;
  undefined8 *local_30;
  long *local_28;
  
  if (param_3 < *(long *)(param_1 + 0x68)) {
    lVar4 = -1;
  }
  else {
    uVar2 = *(undefined8 *)(param_1 + 0x980);
    local_50 = *(long **)(param_1 + 0x60);
    pcVar5 = param_2 + param_3;
    local_40 = param_2;
    if (*local_50 == 0) {
      iVar1 = *(int *)(param_1 + 0x988);
      local_40 = param_2;
LAB_0010f17b:
      if (iVar1 < 0) {
        while( true ) {
          pcVar6 = local_40 + 1;
          bVar3 = FUN_0010c863(uVar2,(int)*local_40);
          local_50 = *(long **)(param_1 + ((long)(int)(uint)bVar3 + 0x2e) * 8);
          local_40 = pcVar6;
          if (local_50 != (long *)0x0) break;
          if (pcVar5 <= pcVar6) {
            return -1;
          }
        }
      }
      else {
        pcVar6 = (char *)FUN_0010e9c2(local_40,(long)pcVar5 - (long)local_40,param_1);
        if (pcVar6 == (char *)0x0) {
          return -1;
        }
        bVar3 = FUN_0010c863(uVar2,(int)*pcVar6);
        local_50 = *(long **)(param_1 + ((long)(int)(uint)bVar3 + 0x2e) * 8);
        local_40 = pcVar6 + 1;
      }
LAB_0010f24b:
      do {
        if (*local_50 != 0) break;
        if (pcVar5 <= local_40) {
          return -1;
        }
        pcVar6 = local_40 + 1;
        bVar3 = FUN_0010c863(uVar2,(int)*local_40);
        local_30 = (undefined8 *)local_50[1];
        while (bVar3 != *(byte *)(local_30 + 3)) {
          if (bVar3 < *(byte *)(local_30 + 3)) {
            local_30 = (undefined8 *)*local_30;
          }
          else {
            local_30 = (undefined8 *)local_30[1];
          }
          if (local_30 == (undefined8 *)0x0) {
            local_50 = (long *)local_50[4];
            if (local_50 == (long *)0x0) {
              local_50 = *(long **)(param_1 + ((long)(int)(uint)bVar3 + 0x2e) * 8);
              local_40 = pcVar6;
              if (local_50 != (long *)0x0) goto LAB_0010f24b;
              if (pcVar5 <= pcVar6) {
                return -1;
              }
              goto LAB_0010f17b;
            }
            if (*local_50 != 0) goto LAB_0010f366;
            local_30 = (undefined8 *)local_50[1];
          }
        }
        local_50 = (long *)local_30[2];
        local_40 = pcVar6;
      } while( true );
    }
LAB_0010f366:
    for (local_48 = local_50; *local_48 < 0; local_48 = (long *)local_48[4]) {
    }
    local_38 = (long)local_40 - local_48[5];
    if (param_5 != '\0') {
      while (local_40 < pcVar5) {
        pcVar6 = local_40 + 1;
        bVar3 = FUN_0010c863(uVar2,(int)*local_40);
        do {
          local_30 = (undefined8 *)local_50[1];
          while ((local_30 != (undefined8 *)0x0 && (bVar3 != *(byte *)(local_30 + 3)))) {
            if (bVar3 < *(byte *)(local_30 + 3)) {
              local_30 = (undefined8 *)*local_30;
            }
            else {
              local_30 = (undefined8 *)local_30[1];
            }
          }
        } while (((local_30 == (undefined8 *)0x0) &&
                 (local_50 = (long *)local_50[4], local_50 != (long *)0x0)) &&
                (local_48[5] <= local_50[5]));
        if (local_30 == (undefined8 *)0x0) break;
        local_50 = (long *)local_30[2];
        local_40 = pcVar6;
        local_28 = local_50;
        if (*local_50 != 0) {
          for (; *local_28 < 0; local_28 = (long *)local_28[4]) {
          }
          if ((ulong)((long)pcVar6 - local_28[5]) <= local_38) {
            local_48 = local_28;
            local_38 = (long)pcVar6 - local_28[5];
          }
        }
      }
    }
    *param_4 = *local_48 >> 1;
    param_4[1] = local_38 - (long)param_2;
    param_4[2] = local_48[5];
    lVar4 = local_38 - (long)param_2;
  }
  return lVar4;
}




// Function: acexec @ 0xf50c

void acexec(long param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined1 param_5)

{
  if (*(long *)(param_1 + 0x980) == 0) {
    FUN_0010f0f4(param_1,param_2,param_3,param_4,param_5);
  }
  else {
    FUN_0010f0f4(param_1,param_2,param_3,param_4,param_5);
  }
  return;
}




// Function: kwsexec @ 0xf585

void kwsexec(long param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined1 param_5)

{
  (**(code **)(param_1 + 0x998))(param_1,param_2,param_3,param_4,param_5);
  return;
}




// Function: kwsfree @ 0xf5d1

/* WARNING: Removing unreachable block (ram,0x0010f5ff) */
/* WARNING: Removing unreachable block (ram,0x0010f60d) */

void kwsfree(void *param_1)

{
  _obstack_free(param_1,0);
  free(param_1);
  return;
}




// Function: wordchar @ 0xf73c

undefined4 wordchar(wint_t param_1)

{
  int iVar1;
  
  if ((param_1 != 0x5f) && (iVar1 = iswalnum(param_1), iVar1 == 0)) {
    return 0;
  }
  return 1;
}




// Function: wordinit @ 0xf770

void wordinit(void)

{
  undefined1 uVar1;
  int local_c;
  
  for (local_c = 0; local_c < 0x100; local_c = local_c + 1) {
    uVar1 = FUN_0010f73c(*(undefined4 *)(&DAT_001331a4 + ((long)local_c + 0x40) * 4));
    (&DAT_001339e0)[local_c] = uVar1;
  }
  return;
}




// Function: kwsinit @ 0xf7c8

void kwsinit(char param_1)

{
  int iVar1;
  size_t sVar2;
  undefined4 local_14;
  undefined8 local_10;
  
  local_10 = 0;
  if (DAT_00133180 != '\0') {
    sVar2 = __ctype_get_mb_cur_max();
    if ((sVar2 == 1) || (param_1 != '\0')) {
      local_10 = FUN_00126b10(0x100);
      for (local_14 = 0; local_14 < 0x100; local_14 = local_14 + 1) {
        iVar1 = toupper(local_14);
        *(char *)(local_10 + local_14) = (char)iVar1;
      }
    }
  }
  FUN_0010c89b(local_10);
  return;
}




// Function: mb_goback @ 0xf84a

long mb_goback(ulong *param_1,long *param_2,char *param_3,long param_4)

{
  long lVar1;
  long in_FS_OFFSET;
  int local_3c;
  char *local_38;
  char *local_30;
  long local_28;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_30 = (char *)*param_1;
  if (local_30 < param_3) {
    if (DAT_001331a2 == '\0') {
      local_18 = 0;
      local_38 = local_30;
      do {
        local_28 = FUN_0010f6da(local_38,param_4 - (long)local_38,&local_18);
        if (local_28 < 0) {
          local_28 = 1;
          memset(&local_18,0,8);
        }
        local_30 = local_38;
        local_38 = local_38 + local_28;
      } while (local_38 < param_3);
      if (param_2 != (long *)0x0) {
        *param_2 = local_28;
      }
    }
    else {
      local_38 = param_3;
      if (((int)*param_3 & 0xc0U) == 0x80) {
        for (local_3c = 1; local_3c < 4; local_3c = local_3c + 1) {
          if (((int)param_3[-local_3c] & 0xc0U) != 0x80) {
            if ((int)(~(int)param_3[-local_3c] & 0xffU) >> (7U - (char)local_3c & 0x1f) == 0) {
              local_18 = 0;
              lVar1 = FUN_0010f686((long)param_3 - (long)local_3c,
                                   param_4 - ((long)param_3 - (long)local_3c),&local_18);
              if (-1 < lVar1) {
                local_30 = param_3 + -(long)local_3c;
                local_38 = local_30 + lVar1;
              }
            }
            break;
          }
        }
      }
    }
    *param_1 = (ulong)local_38;
    if (local_38 == param_3) {
      lVar1 = 0;
    }
    else {
      lVar1 = (long)param_3 - (long)local_30;
    }
  }
  else {
    lVar1 = (long)param_3 - (long)local_30;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: wordchars_count @ 0xfa82

long wordchars_count(long param_1,long param_2,char param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined4 local_2c;
  long local_28;
  long local_20;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_28 = 0;
  local_18 = 0;
  do {
    if (param_2 - param_1 <= local_28) break;
    if ((&DAT_001339e0)[(int)(uint)*(byte *)(param_1 + local_28)] == '\0') {
      if ((&DAT_001331a3)[(int)(uint)*(byte *)(param_1 + local_28)] != -2) break;
      local_2c = 0;
      local_20 = FUN_00121b80(&local_2c,local_28 + param_1,(param_2 - param_1) - local_28,&local_18)
      ;
      cVar1 = FUN_0010f73c(local_2c);
      if (cVar1 != '\x01') break;
      local_28 = local_28 + (ulong)(local_20 == 0) + local_20;
    }
    else {
      local_28 = local_28 + 1;
    }
  } while (param_3 == '\x01');
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_28;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: wordchars_size @ 0xfbb2

void wordchars_size(undefined8 param_1,undefined8 param_2)

{
  FUN_0010fa82(param_1,param_2,1);
  return;
}




// Function: wordchar_next @ 0xfbe0

void wordchar_next(undefined8 param_1,undefined8 param_2)

{
  FUN_0010fa82(param_1,param_2,0);
  return;
}




// Function: wordchar_prev @ 0xfc0e

ulong wordchar_prev(long param_1,long param_2,undefined8 param_3)

{
  byte *pbVar1;
  ulong uVar2;
  long lVar3;
  long in_FS_OFFSET;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 == param_2) {
    uVar2 = 0;
  }
  else {
    pbVar1 = (byte *)(param_2 + -1);
    if ((DAT_001331a0 == '\x01') && ((~(*pbVar1 >> 7) & DAT_001331a2) == 0)) {
      local_18 = param_1;
      lVar3 = FUN_0010f84a(&local_18,0,pbVar1,param_3);
      uVar2 = FUN_0010fbe0((long)pbVar1 - lVar3,param_3);
    }
    else {
      uVar2 = (ulong)(byte)(&DAT_001339e0)[(int)(uint)*pbVar1];
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: private_malloc @ 0xfd0f

void private_malloc(long param_1)

{
  if (param_1 < 0) {
    FUN_00127122();
  }
  FUN_00126b10(param_1);
  return;
}




// Function: private_free @ 0xfd3f

void private_free(void *param_1)

{
  free(param_1);
  return;
}




// Function: jit_exec @ 0xfd62

/* WARNING: Removing unreachable block (ram,0x0010fd9e) */

int jit_exec(undefined8 *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                undefined4 param_5)

{
  int iVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  bool bVar3;
  int local_34 [3];
  long local_28;
  long local_20;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  while( true ) {
    while( true ) {
      local_34[1] = 0x2000;
      local_28 = 0x7fffffffffffffff;
      local_34[2] = pcre2_match_8(param_1[1],param_2,param_3,param_4,param_5,param_1[3],param_1[2]);
      if ((local_34[2] != -0x2e) || (local_28 / 2 < (long)param_1[5])) break;
      local_20 = param_1[5];
      param_1[5] = local_20 * 2;
      local_18 = param_1[5];
      pcre2_jit_stack_free_8(param_1[4]);
      uVar2 = pcre2_jit_stack_create_8(local_20,local_18,*param_1);
      param_1[4] = uVar2;
      if (param_1[4] == 0) {
        FUN_00127122();
      }
      if (param_1[2] == 0) {
        uVar2 = pcre2_match_context_create_8(*param_1);
        param_1[2] = uVar2;
      }
      pcre2_jit_stack_assign_8(param_1[2],0,param_1[4]);
    }
    if (local_34[2] != -0x35) break;
    pcre2_config_8(7,local_34);
    iVar1 = local_34[0] * 2;
    bVar3 = local_34[0] < 0;
    local_34[0] = iVar1;
    if (bVar3) break;
    if (param_1[2] == 0) {
      uVar2 = pcre2_match_context_create_8(*param_1);
      param_1[2] = uVar2;
    }
    pcre2_set_depth_limit_8(param_1[2],local_34[0]);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_34[2];
}




// Function: bad_utf8_from_pcre2 @ 0xff6e

undefined8 bad_utf8_from_pcre2(void)

{
  return 0;
}




// Function: Pcompile @ 0xff80

undefined8 * Pcompile(void *param_1,size_t param_2)

{
  int iVar1;
  undefined4 uVar2;
  undefined8 uVar3;
  void *pvVar4;
  long in_FS_OFFSET;
  size_t local_188;
  void *local_180;
  int local_178;
  uint local_174;
  undefined1 local_170 [8];
  void *local_168;
  void *local_160;
  undefined8 *local_158;
  undefined8 local_150;
  undefined8 local_148;
  size_t local_140;
  void *local_138;
  void *local_130;
  undefined1 local_128 [264];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_00133180 == '\0') {
    local_174 = 0x10;
  }
  else {
    local_174 = 0x18;
  }
  local_160 = (void *)((long)param_1 + param_2);
  local_158 = (undefined8 *)FUN_00126b10(0x38);
  uVar3 = pcre2_general_context_create_8(FUN_0010fd0f,FUN_0010fd3f,0);
  *local_158 = uVar3;
  local_150 = *local_158;
  local_148 = pcre2_compile_context_create_8(local_150);
  if (DAT_001331a0 != '\0') {
    if (DAT_001331a2 != '\x01') {
      uVar3 = gettext("-P supports only unibyte and UTF-8 locales");
      error(2,0,uVar3);
    }
    local_174 = local_174 | 0x4080000;
  }
  pvVar4 = rawmemchr(param_1,10);
  if (local_160 != pvVar4) {
    uVar3 = gettext("the -P option only supports a single pattern");
    error(2,0,uVar3);
  }
  local_168 = (void *)0x0;
  local_188 = param_2;
  local_180 = param_1;
  if (DAT_00133182 == '\0') {
    if (DAT_00133181 != '\0') {
      local_140 = param_2 + 0x11;
      local_168 = (void *)FUN_00126b10(local_140);
      local_138 = local_168;
      local_130 = mempcpy(local_168,"(?<!\\w)(?:)(?!\\w)invalid argument %s for %s",10);
      local_130 = mempcpy(local_130,param_1,param_2);
      memcpy(local_130,")(?!\\w)invalid argument %s for %s",7);
      local_180 = local_138;
      local_188 = local_140;
    }
  }
  else {
    pcre2_set_compile_extra_options_8(local_148,8);
  }
  uVar3 = pcre2_maketables_8(local_150);
  pcre2_set_character_tables_8(local_148,uVar3);
  uVar3 = pcre2_compile_8(local_180,local_188,local_174,&local_178,local_170,local_148);
  local_158[1] = uVar3;
  if (local_158[1] == 0) {
    pcre2_get_error_message_8(local_178,local_128,0x100);
    error(2,0,&DAT_0012a8ed,local_128);
  }
  free(local_168);
  pcre2_compile_context_free_8(local_148);
  local_158[2] = 0;
  uVar3 = pcre2_match_data_create_from_pattern_8(local_158[1],local_150);
  local_158[3] = uVar3;
  iVar1 = pcre2_jit_compile_8(local_158[1],1);
  local_178 = iVar1;
  if (((iVar1 != 0) && (iVar1 != -0x2d)) && (iVar1 != -0x30)) {
    uVar3 = gettext("JIT internal error: %d");
    error(2,0,uVar3,iVar1);
  }
  local_158[4] = 0;
  local_158[5] = 0x8000;
  uVar2 = FUN_0010fd62(local_158,&DAT_0012a907,0,0,1);
  *(undefined4 *)(local_158 + 6) = uVar2;
  uVar2 = FUN_0010fd62(local_158,&DAT_0012a907,0,0,0);
  *(undefined4 *)((long)local_158 + 0x34) = uVar2;
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_158;
}




// Function: Pexecute @ 0x10409

long Pexecute(long param_1,char *param_2,long param_3,long *param_4,char *param_5)

{
  byte bVar1;
  char cVar2;
  long *plVar3;
  char *pcVar4;
  long lVar5;
  long lVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  bool local_81;
  int local_80;
  uint local_7c;
  char *local_78;
  char *local_70;
  char *local_68;
  char *local_60;
  char *local_58;
  
  local_78 = param_2;
  if (param_5 != (char *)0x0) {
    local_78 = param_5;
  }
  local_81 = local_78[-1] == DAT_00133183;
  plVar3 = (long *)pcre2_get_ovector_pointer_8(*(undefined8 *)(param_1 + 0x18));
  local_68 = param_2;
  do {
    local_70 = local_68;
    pcVar4 = rawmemchr(local_78,(int)DAT_00133183);
    while( true ) {
      while (bVar1 = FUN_0010f64d((int)*local_78), (&DAT_001331a3)[(int)(uint)bVar1] == -1) {
        local_78 = local_78 + 1;
        local_81 = false;
        local_68 = local_78;
      }
      lVar5 = (long)local_78 - (long)local_68;
      if (local_78 == pcVar4) break;
      local_7c = (uint)(local_81 != true);
      local_80 = FUN_0010fd62(param_1,local_68,(long)pcVar4 - (long)local_68,lVar5,local_7c);
      cVar2 = FUN_0010ff6e(local_80);
      if (cVar2 != '\x01') goto LAB_00110655;
      lVar6 = pcre2_get_startchar_8(*(undefined8 *)(param_1 + 0x18));
      if (lVar5 <= lVar6) {
        if (lVar6 == 0) {
          *plVar3 = 0;
          plVar3[1] = 0;
          local_80 = *(int *)(param_1 + ((long)(int)(uint)local_81 + 0xc) * 4);
        }
        else {
          local_80 = FUN_0010fd62(param_1,local_68,lVar6,lVar5,local_7c | 0x40000002);
        }
        if (local_80 != -1) goto LAB_00110655;
        local_78 = local_68 + lVar6 + 1;
        local_81 = false;
      }
      local_68 = local_68 + lVar6 + 1;
    }
    plVar3[1] = lVar5;
    *plVar3 = plVar3[1];
    local_80 = *(int *)(param_1 + ((long)(int)(uint)local_81 + 0xc) * 4);
LAB_00110655:
    if (local_80 != -1) break;
    local_81 = true;
    local_78 = pcVar4 + 1;
    local_70 = local_78;
    local_68 = local_78;
  } while (local_78 < param_2 + param_3);
  if (0 < local_80) {
    local_60 = local_68 + *plVar3;
    local_58 = local_68 + plVar3[1];
    if (param_5 == (char *)0x0) {
      local_60 = local_70;
      local_58 = pcVar4 + 1;
    }
    *param_4 = (long)local_58 - (long)local_60;
    return (long)local_60 - (long)param_2;
  }
  if (local_80 < -0x2d) {
    if (-0x40 < local_80) {
      switch(local_80) {
      case -0x30:
        uVar7 = FUN_00106eac();
        uVar8 = gettext("%s: memory exhausted");
        error(2,0,uVar8,uVar7);
      case -0x2e:
        uVar7 = FUN_00106eac();
        uVar8 = gettext("%s: exhausted PCRE JIT stack");
        error(2,0,uVar8,uVar7);
      case -0x2f:
        uVar7 = FUN_00106eac();
        uVar8 = gettext("%s: exceeded PCRE\'s backtracking limit");
        error(2,0,uVar8,uVar7);
      case -0x35:
        uVar7 = FUN_00106eac();
        uVar8 = gettext("%s: exceeded PCRE\'s nested backtracking limit");
        error(2,0,uVar8,uVar7);
      case -0x34:
        uVar7 = FUN_00106eac();
        uVar8 = gettext("%s: PCRE detected recurse loop");
        error(2,0,uVar8,uVar7);
      case -0x3f:
        uVar7 = FUN_00106eac();
        uVar8 = gettext("%s: exceeded PCRE\'s heap limit");
        error(2,0,uVar8,uVar7);
      }
    }
  }
  else if (local_80 == -1) {
    return -1;
  }
  uVar7 = FUN_00106eac();
  uVar8 = gettext("%s: internal PCRE error: %d");
  error(2,0,uVar8,uVar7,local_80);
  return -1;
}




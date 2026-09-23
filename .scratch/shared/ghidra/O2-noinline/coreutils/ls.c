// Function: main @ 0x4dd0

undefined4 main(int param_1,undefined8 *param_2)

{
  undefined8 *puVar1;
  undefined *puVar2;
  undefined *puVar3;
  char cVar4;
  byte bVar5;
  int iVar6;
  uint uVar7;
  char *pcVar8;
  void *__ptr;
  long lVar9;
  long lVar10;
  long *plVar11;
  long in_FS_OFFSET;
  undefined1 auVar12 [16];
  undefined1 local_58 [16];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00113390(*param_2);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  FUN_00108dd0();
  FUN_00118230(FUN_0010df00);
  DAT_00127210 = 0;
  DAT_001272b8 = 1;
  DAT_00127380 = (long *)0x0;
  DAT_00127370 = 0x8000000000000000;
  DAT_00127378 = 0xffffffffffffffff;
  iVar6 = FUN_0010bda0(param_1,param_2);
  lVar10 = (long)iVar6;
  if ((DAT_00127312 == '\0') || (FUN_00106af0(), DAT_00127312 == '\0')) {
    if (DAT_001272f4 != 0) goto LAB_001051c8;
  }
  else {
    DAT_001272c0 = 0;
    if ((((DAT_001272f4 != 0) || (cVar4 = FUN_00106420(0xd), cVar4 != '\0')) ||
        ((cVar4 = FUN_00106420(0xe), cVar4 != '\0' && (DAT_00127390 != '\0')))) ||
       ((cVar4 = FUN_00106420(0xc), cVar4 != '\0' && (DAT_00127338 == 0)))) {
LAB_001051c8:
      DAT_001272fd = 1;
    }
  }
  if (((DAT_001272f8 == 0) && (DAT_001272f8 = 1, DAT_001272f5 == '\0')) && (DAT_00127314 != 3)) {
    DAT_001272f8 = (-(uint)(DAT_00127338 == 0) & 0xfffffffe) + 3;
  }
  if (DAT_001272f6 != '\0') {
    DAT_001273c8 = FUN_0010f380(0x1e,0,FUN_001056d0,FUN_001056e0,free);
    if (DAT_001273c8 == 0) {
                    /* WARNING: Subroutine does not return */
      FUN_00116da0();
    }
    _obstack_begin(&DAT_001270e0,0,0,PTR_malloc_00125fd0,PTR_free_00125f98);
  }
  pcVar8 = getenv("TZ");
  DAT_001272a8 = FUN_00115d80(pcVar8);
  puVar3 = PTR_malloc_00125fd0;
  puVar2 = PTR_free_00125f98;
  if ((((DAT_00127330 - 3U & 0xfffffffd) == 0) || (DAT_00127338 == 0)) ||
     ((DAT_00127365 != '\0' || (DAT_0012732c != '\0')))) {
    DAT_001272a1 = 1;
    bVar5 = 0;
  }
  else {
    DAT_001272a1 = 0;
    bVar5 = 1;
    if (((DAT_001272f6 == '\0') && (DAT_00127312 == '\0')) && (DAT_00127314 == 0)) {
      bVar5 = DAT_001272f4;
    }
  }
  DAT_001272a0 = bVar5 & 1;
  if (DAT_00127318 != '\0') {
    _obstack_begin(&DAT_001271a0,0,0,PTR_malloc_00125fd0,PTR_free_00125f98);
    _obstack_begin(&DAT_00127140,0,0,puVar3,puVar2);
  }
  if (DAT_00127311 != '\0') {
    FUN_00105670();
    DAT_00127388 = (undefined1 *)FUN_00116f20();
    if (DAT_00127388 == (undefined1 *)0x0) {
      DAT_00127388 = &DAT_0011abb9;
    }
  }
  DAT_001273b8 = 100;
  DAT_001273c0 = FUN_00116b00(100,0xd0);
  DAT_001273b0 = 0;
  FUN_00106e20();
  if (param_1 - iVar6 < 1) {
    if (DAT_001272f5 == '\0') {
      FUN_00106840(&DAT_0011adcf,0,1);
    }
    else {
      FUN_0010a550(&DAT_0011adcf,3,1,&DAT_0011abb9);
    }
    if (DAT_001273b0 != 0) goto LAB_001052b2;
LAB_00105292:
    auVar12._8_8_ = local_58._8_8_;
    auVar12._0_8_ = local_58._0_8_;
    if (DAT_00127380 != (long *)0x0) {
      plVar11 = DAT_00127380;
      if (DAT_00127380[3] != 0) goto LAB_00105038;
      DAT_001272b8 = 0;
      local_58 = auVar12;
      goto LAB_00105038;
    }
  }
  else {
    do {
      puVar1 = param_2 + lVar10;
      lVar10 = lVar10 + 1;
      FUN_0010a550(*puVar1,0,1,&DAT_0011abb9);
    } while ((int)lVar10 < param_1);
    if (DAT_001273b0 == 0) {
LAB_00104fe9:
      if (param_1 - iVar6 < 2) goto LAB_00105292;
LAB_00104ffd:
      while (plVar11 = DAT_00127380, auVar12 = local_58, DAT_00127380 != (long *)0x0) {
LAB_00105038:
        while( true ) {
          lVar10 = DAT_001273c8;
          DAT_00127380 = (long *)plVar11[3];
          if ((DAT_001273c8 != 0) && (*plVar11 == 0)) break;
          FUN_0010b180(*plVar11,plVar11[1],(char)plVar11[2]);
          FUN_001061b0(plVar11);
          DAT_001272b8 = 1;
          plVar11 = DAT_00127380;
          auVar12 = local_58;
          if (DAT_00127380 == (long *)0x0) goto LAB_001050a0;
        }
        auVar12 = FUN_00106330();
        local_58 = auVar12;
        __ptr = (void *)FUN_0010fa00(lVar10,local_58);
        if (__ptr == (void *)0x0) {
                    /* WARNING: Subroutine does not return */
          __assert_fail("found","src/ls.c",0x70d,"main");
        }
        free(__ptr);
        FUN_001061b0(plVar11);
      }
    }
    else {
LAB_001052b2:
      FUN_001089f0();
      if (DAT_001272f5 == '\0') {
        FUN_0010a1e0(0,1);
      }
      if (DAT_001273b0 == 0) goto LAB_00104fe9;
      FUN_00109f20();
      auVar12._8_8_ = local_58._8_8_;
      auVar12._0_8_ = local_58._0_8_;
      if (DAT_00127380 != (long *)0x0) {
        FUN_00107460(10);
        goto LAB_00104ffd;
      }
    }
  }
LAB_001050a0:
  local_58 = auVar12;
  if ((DAT_00127312 == '\0') || (DAT_00127310 == '\0')) goto LAB_00105130;
  if (DAT_00126060 == 2) {
    iVar6 = memcmp(PTR_DAT_00126068,&DAT_0011add7,2);
    if (((iVar6 != 0) || (DAT_00126070 != 1)) || (*PTR_DAT_00126078 != 'm')) goto LAB_001050ea;
  }
  else {
LAB_001050ea:
    FUN_00107cc0();
  }
  fflush_unlocked(stdout);
  FUN_00107dd0();
  for (iVar6 = DAT_00127214; iVar6 != 0; iVar6 = iVar6 + -1) {
    raise(0x13);
  }
  if (DAT_00127218 != 0) {
    raise(DAT_00127218);
  }
LAB_00105130:
  if (DAT_00127318 != '\0') {
    FUN_00108060("//DIRED//",&DAT_001271a0);
    FUN_00108060("//SUBDIRED//",&DAT_00127140);
    uVar7 = FUN_001151e0(DAT_001272d0);
    __printf_chk(1,"//DIRED-OPTIONS// --quoting-style=%s\n",(&PTR_s_literal_001259e0)[uVar7]);
  }
  lVar10 = DAT_001273c8;
  if (DAT_001273c8 != 0) {
    lVar9 = FUN_0010ef20(DAT_001273c8);
    if (lVar9 != 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("hash_get_n_entries (active_dir_set) == 0","src/ls.c",0x741,"main");
    }
    FUN_0010f540(lVar10);
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return DAT_00127210;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: file_or_link_mode @ 0x55e0

undefined4 file_or_link_mode(long param_1)

{
  if ((DAT_00127390 != '\0') && (*(char *)(param_1 + 0xb9) != '\0')) {
    return *(undefined4 *)(param_1 + 0xac);
  }
  return *(undefined4 *)(param_1 + 0x30);
}




// Function: first_percent_b @ 0x5620

char * first_percent_b(char *param_1)

{
  char cVar1;
  char cVar2;
  
  cVar1 = *param_1;
  do {
    if (cVar1 == '\0') {
      return (char *)0x0;
    }
    cVar2 = param_1[1];
    if (cVar1 == '%') {
      if (cVar2 == '%') {
        cVar2 = param_1[2];
        param_1 = param_1 + 1;
      }
      else if (cVar2 == 'b') {
        return param_1;
      }
    }
    param_1 = param_1 + 1;
    cVar1 = cVar2;
  } while( true );
}




// Function: file_escape_init @ 0x5670

void file_escape_init(void)

{
  char cVar1;
  int iVar2;
  ulong uVar3;
  bool bVar4;
  
  uVar3 = 0;
  do {
    iVar2 = (int)uVar3;
    cVar1 = FUN_0010d3f0(uVar3 & 0xffffffff);
    bVar4 = true;
    if (cVar1 == '\0') {
      if ((iVar2 - 0x2dU < 2) || (iVar2 == 0x7e)) {
        bVar4 = true;
      }
      else {
        bVar4 = iVar2 == 0x5f;
      }
    }
    (&DAT_00126fe0)[uVar3] = (&DAT_00126fe0)[uVar3] | bVar4;
    uVar3 = uVar3 + 1;
  } while (uVar3 != 0x100);
  return;
}




// Function: dev_ino_hash @ 0x56d0

ulong dev_ino_hash(ulong *param_1,ulong param_2)

{
  return *param_1 % param_2;
}




// Function: dev_ino_compare @ 0x56e0

undefined8 dev_ino_compare(long *param_1,long *param_2)

{
  if (*param_1 != *param_2) {
    return 0;
  }
  return CONCAT71((int7)((ulong)param_2[1] >> 8),param_1[1] == param_2[1]);
}




// Function: sighandler @ 0x5700

void sighandler(int param_1)

{
  if (DAT_00127218 == 0) {
    DAT_00127218 = param_1;
  }
  return;
}




// Function: stophandler @ 0x5720

void stophandler(void)

{
  if (DAT_00127218 == 0) {
    DAT_00127214 = DAT_00127214 + 1;
  }
  return;
}




// Function: get_funky_string @ 0x5740

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
  if (cVar4 == '\\') goto LAB_001057a6;
LAB_0010577a:
  if (bVar12 || SBORROW1(cVar4,'\\') != cVar5 < '\0') {
    if (cVar4 == '=') {
      if ((char)param_3 != '\0') goto LAB_001057de;
    }
    else if ((cVar4 < '>') && ((cVar4 == '\0' || (cVar4 == ':')))) {
      param_3 = 1;
LAB_001057de:
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
      goto LAB_00105796;
    }
    pcVar2 = pcVar2 + 1;
    if (bVar1 == 0x3f) {
      lVar8 = lVar8 + 1;
      lVar7 = lVar6 + 1;
      *(undefined1 *)(lVar6 + -1) = 0x7f;
      goto LAB_00105796;
    }
    param_3 = 0;
    goto LAB_001057de;
  }
  pcVar2 = pcVar2 + 1;
  lVar7 = lVar6;
LAB_0010578b:
  *(char *)(lVar7 + -1) = cVar4;
  lVar8 = lVar8 + 1;
  lVar7 = lVar7 + 1;
LAB_00105796:
  do {
    cVar4 = *pcVar2;
    lVar10 = lVar7 + -1;
    lVar9 = lVar8 + -1;
    cVar5 = cVar4 + -0x5c;
    bVar12 = cVar5 == '\0';
    pcVar3 = pcVar2;
    lVar6 = lVar7;
    if (!bVar12) goto LAB_0010577a;
LAB_001057a6:
    cVar5 = pcVar3[1];
    pcVar2 = pcVar3 + 2;
    if (cVar5 == '\0') {
      param_3 = 0;
      goto LAB_001057de;
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
      if (7 < (byte)(cVar5 - 0x30U)) goto LAB_0010578b;
      do {
        pcVar2 = pcVar2 + 1;
        cVar4 = cVar5 + -0x30 + cVar4 * '\b';
        cVar5 = *pcVar2;
      } while ((byte)(cVar5 - 0x30U) < 8);
      *(char *)(lVar7 + -1) = cVar4;
      lVar8 = lVar8 + 1;
      lVar7 = lVar7 + 1;
      goto LAB_00105796;
    default:
      goto switchD_001057cb_caseD_8;
    case '\x0f':
      cVar5 = '\x7f';
      break;
    case '(':
    case 'H':
      goto switchD_001057cb_caseD_28;
    case '/':
      cVar5 = ' ';
      break;
    case '1':
      cVar5 = '\a';
      goto switchD_001057cb_caseD_8;
    case '2':
      cVar5 = '\b';
      goto switchD_001057cb_caseD_8;
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
switchD_001057cb_caseD_8:
    }
    *(char *)(lVar7 + -1) = cVar5;
    lVar8 = lVar8 + 1;
    lVar7 = lVar7 + 1;
  } while( true );
switchD_001057cb_caseD_28:
  cVar5 = pcVar3[2];
  pcVar11 = pcVar3 + 3;
  cVar4 = '\0';
  pcVar2 = pcVar3 + 2;
  if ('F' < cVar5) goto LAB_00105902;
  do {
    if (cVar5 < 'A') {
      if (9 < (byte)(cVar5 - 0x30U)) goto LAB_0010578b;
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
LAB_00105902:
      if (5 < (byte)(cVar5 + 0x9fU)) goto LAB_0010578b;
      cVar4 = cVar5 + -0x57 + cVar4 * '\x10';
      pcVar2 = pcVar11;
    }
  } while( true );
}




// Function: set_exit_status @ 0x59b0

void set_exit_status(char param_1)

{
  if (param_1 != '\0') {
    DAT_00127210 = 2;
    return;
  }
  if (DAT_00127210 == 0) {
    DAT_00127210 = 1;
  }
  return;
}




// Function: unsigned_file_size @ 0x59e0

undefined8 unsigned_file_size(undefined8 param_1)

{
  return param_1;
}




// Function: errno_unsupported @ 0x59f0

undefined8 errno_unsupported(int param_1)

{
  undefined8 uVar1;
  
  if ((param_1 - 0x16U & 0xffffffef) != 0) {
    uVar1 = FUN_001055c0();
    return uVar1;
  }
  return 1;
}




// Function: is_linked_directory @ 0x5a10

bool is_linked_directory(long param_1)

{
  int iVar1;
  bool bVar2;
  
  iVar1 = *(int *)(param_1 + 0xa8);
  bVar2 = iVar1 == 3 || iVar1 == 9;
  if (iVar1 != 3 && iVar1 != 9) {
    bVar2 = (*(uint *)(param_1 + 0xac) & 0xf000) == 0x4000;
  }
  return bVar2;
}




// Function: dirfirst_check @ 0x5a40

void dirfirst_check(undefined8 param_1,undefined8 param_2,code *UNRECOVERED_JUMPTABLE)

{
  uint uVar1;
  uint uVar2;
  
  uVar1 = FUN_00105a10(param_2);
  uVar2 = FUN_00105a10(param_1);
  if ((uVar1 & 0xff) == (uVar2 & 0xff)) {
                    /* WARNING: Could not recover jumptable at 0x00105a6e. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*UNRECOVERED_JUMPTABLE)();
    return;
  }
  return;
}




// Function: cmp_ctime @ 0x5a80

void cmp_ctime(undefined8 *param_1,undefined8 *param_2,code *UNRECOVERED_JUMPTABLE)

{
  int iVar1;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  
  auVar2 = FUN_00115ca0(param_1 + 3);
  auVar3 = FUN_00115ca0(param_2 + 3);
  iVar1 = FUN_001162a0(auVar3._0_8_,auVar3._8_8_,auVar2._0_8_,auVar2._8_8_);
  if (iVar1 == 0) {
                    /* WARNING: Could not recover jumptable at 0x00105ad0. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*UNRECOVERED_JUMPTABLE)(*param_1,*param_2);
    return;
  }
  return;
}




// Function: cmp_mtime @ 0x5af0

void cmp_mtime(undefined8 *param_1,undefined8 *param_2,code *UNRECOVERED_JUMPTABLE)

{
  int iVar1;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  
  auVar2 = FUN_00115cb0(param_1 + 3);
  auVar3 = FUN_00115cb0(param_2 + 3);
  iVar1 = FUN_001162a0(auVar3._0_8_,auVar3._8_8_,auVar2._0_8_,auVar2._8_8_);
  if (iVar1 == 0) {
                    /* WARNING: Could not recover jumptable at 0x00105b40. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*UNRECOVERED_JUMPTABLE)(*param_1,*param_2);
    return;
  }
  return;
}




// Function: cmp_atime @ 0x5b60

void cmp_atime(undefined8 *param_1,undefined8 *param_2,code *UNRECOVERED_JUMPTABLE)

{
  int iVar1;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  
  auVar2 = FUN_00115c90(param_1 + 3);
  auVar3 = FUN_00115c90(param_2 + 3);
  iVar1 = FUN_001162a0(auVar3._0_8_,auVar3._8_8_,auVar2._0_8_,auVar2._8_8_);
  if (iVar1 == 0) {
                    /* WARNING: Could not recover jumptable at 0x00105bb0. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*UNRECOVERED_JUMPTABLE)(*param_1,*param_2);
    return;
  }
  return;
}




// Function: cmp_btime @ 0x5bd0

void cmp_btime(undefined8 *param_1,undefined8 *param_2,code *UNRECOVERED_JUMPTABLE)

{
  int iVar1;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  
  auVar2 = thunk_FUN_00115cb0(param_1 + 3);
  auVar3 = thunk_FUN_00115cb0(param_2 + 3);
  iVar1 = FUN_001162a0(auVar3._0_8_,auVar3._8_8_,auVar2._0_8_,auVar2._8_8_);
  if (iVar1 == 0) {
                    /* WARNING: Could not recover jumptable at 0x00105c20. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*UNRECOVERED_JUMPTABLE)(*param_1,*param_2);
    return;
  }
  return;
}




// Function: off_cmp @ 0x5c40

ulong off_cmp(long param_1,long param_2)

{
  ulong uVar1;
  
  uVar1 = (ulong)(param_2 < param_1);
  if (param_1 < param_2) {
    uVar1 = 0xffffffff;
  }
  return uVar1;
}




// Function: xstrcoll_ctime @ 0x5c60

void xstrcoll_ctime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a80(param_1,param_2,FUN_001072a0);
  return;
}




// Function: strcmp_ctime @ 0x5c70

void strcmp_ctime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a80(param_1,param_2,PTR_strcmp_00125fc0);
  return;
}




// Function: rev_xstrcoll_ctime @ 0x5c80

void rev_xstrcoll_ctime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a80(param_2,param_1,FUN_001072a0);
  return;
}




// Function: rev_strcmp_ctime @ 0x5ca0

void rev_strcmp_ctime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a80(param_2,param_1,PTR_strcmp_00125fc0);
  return;
}




// Function: xstrcoll_df_ctime @ 0x5cc0

void xstrcoll_df_ctime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_00105c60);
  return;
}




// Function: strcmp_df_ctime @ 0x5cd0

void strcmp_df_ctime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_00105c70);
  return;
}




// Function: rev_xstrcoll_df_ctime @ 0x5ce0

void rev_xstrcoll_df_ctime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_00105c80);
  return;
}




// Function: rev_strcmp_df_ctime @ 0x5cf0

void rev_strcmp_df_ctime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_00105ca0);
  return;
}




// Function: xstrcoll_atime @ 0x5da0

void xstrcoll_atime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105b60(param_1,param_2,FUN_001072a0);
  return;
}




// Function: strcmp_atime @ 0x5db0

void strcmp_atime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105b60(param_1,param_2,PTR_strcmp_00125fc0);
  return;
}




// Function: rev_xstrcoll_atime @ 0x5dc0

void rev_xstrcoll_atime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105b60(param_2,param_1,FUN_001072a0);
  return;
}




// Function: rev_strcmp_atime @ 0x5de0

void rev_strcmp_atime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105b60(param_2,param_1,PTR_strcmp_00125fc0);
  return;
}




// Function: xstrcoll_df_atime @ 0x5e00

void xstrcoll_df_atime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_00105da0);
  return;
}




// Function: strcmp_df_atime @ 0x5e10

void strcmp_df_atime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_00105db0);
  return;
}




// Function: rev_xstrcoll_df_atime @ 0x5e20

void rev_xstrcoll_df_atime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_00105dc0);
  return;
}




// Function: rev_strcmp_df_atime @ 0x5e30

void rev_strcmp_df_atime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_00105de0);
  return;
}




// Function: xstrcoll_btime @ 0x5e40

void xstrcoll_btime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105bd0(param_1,param_2,FUN_001072a0);
  return;
}




// Function: strcmp_btime @ 0x5e50

void strcmp_btime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105bd0(param_1,param_2,PTR_strcmp_00125fc0);
  return;
}




// Function: rev_xstrcoll_btime @ 0x5e60

void rev_xstrcoll_btime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105bd0(param_2,param_1,FUN_001072a0);
  return;
}




// Function: rev_strcmp_btime @ 0x5e80

void rev_strcmp_btime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105bd0(param_2,param_1,PTR_strcmp_00125fc0);
  return;
}




// Function: xstrcoll_df_btime @ 0x5ea0

void xstrcoll_df_btime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_00105e40);
  return;
}




// Function: strcmp_df_btime @ 0x5eb0

void strcmp_df_btime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_00105e50);
  return;
}




// Function: rev_xstrcoll_df_btime @ 0x5ec0

void rev_xstrcoll_df_btime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_00105e60);
  return;
}




// Function: rev_strcmp_df_btime @ 0x5ed0

void rev_strcmp_df_btime(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_00105e80);
  return;
}




// Function: xstrcoll_df_size @ 0x5ee0

void xstrcoll_df_size(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_0010a1a0);
  return;
}




// Function: strcmp_df_size @ 0x5ef0

void strcmp_df_size(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_0010a170);
  return;
}




// Function: rev_xstrcoll_df_size @ 0x5f00

void rev_xstrcoll_df_size(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_0010a150);
  return;
}




// Function: rev_strcmp_df_size @ 0x5f10

void rev_strcmp_df_size(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_0010a130);
  return;
}




// Function: rev_strcmp_df_extension @ 0x5f90

void rev_strcmp_df_extension(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_00107350);
  return;
}




// Function: xstrcoll_df_width @ 0x5fa0

void xstrcoll_df_width(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_00108970);
  return;
}




// Function: strcmp_df_width @ 0x5fb0

void strcmp_df_width(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_00108940);
  return;
}




// Function: rev_xstrcoll_df_width @ 0x5fc0

void rev_xstrcoll_df_width(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_00108950);
  return;
}




// Function: rev_strcmp_df_width @ 0x5fd0

void rev_strcmp_df_width(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_00108920);
  return;
}




// Function: xstrcoll_df_version @ 0x5fe0

void xstrcoll_df_version(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_0010a050);
  return;
}




// Function: rev_xstrcoll_df_version @ 0x5ff0

void rev_xstrcoll_df_version(undefined8 param_1,undefined8 param_2)

{
  FUN_00105a40(param_1,param_2,FUN_0010a060);
  return;
}




// Function: initialize_ordering_vector @ 0x6000

void initialize_ordering_vector(void)

{
  long *plVar1;
  long *plVar2;
  long lVar3;
  
  if (DAT_001273b0 != 0) {
    plVar1 = DAT_001273a0 + DAT_001273b0;
    plVar2 = DAT_001273a0;
    lVar3 = DAT_001273c0;
    do {
      *plVar2 = lVar3;
      plVar2 = plVar2 + 1;
      lVar3 = lVar3 + 0xd0;
    } while (plVar2 != plVar1);
  }
  return;
}




// Function: get_type_indicator @ 0x6040

byte get_type_indicator(char param_1,uint param_2,int param_3)

{
  uint uVar1;
  bool bVar2;
  
  if (param_1 == '\0') {
    if (param_3 == 5) {
      return 0;
    }
    if ((param_3 == 3) || (param_3 == 9)) {
      return 0x2f;
    }
    bVar2 = param_3 == 6;
    if (DAT_00127314 == 1) {
      return 0;
    }
  }
  else {
    uVar1 = param_2 & 0xf000;
    if (uVar1 == 0x8000) {
      if (DAT_00127314 != 3) {
        return 0;
      }
      return -((param_2 & 0x49) != 0) & 0x2a;
    }
    if (uVar1 == 0x4000) {
      return 0x2f;
    }
    bVar2 = uVar1 == 0xa000;
    if (DAT_00127314 == 1) {
      return 0;
    }
  }
  if (bVar2) {
    return 0x40;
  }
  if (param_1 == '\0') {
    if (param_3 == 1) {
      return 0x7c;
    }
    bVar2 = param_3 == 7;
  }
  else {
    if ((param_2 & 0xf000) == 0x1000) {
      return 0x7c;
    }
    bVar2 = (param_2 & 0xf000) == 0xc000;
  }
  return -bVar2 & 0x3d;
}




// Function: attach @ 0x6140

void attach(char *param_1,char *param_2,char *param_3)

{
  char cVar1;
  char cVar2;
  char *pcVar3;
  char *pcVar4;
  char *pcVar5;
  
  cVar2 = *param_2;
  cVar1 = cVar2;
  if (cVar2 == '.') {
    cVar1 = param_2[1];
  }
  pcVar4 = param_2;
  if (cVar1 == '\0') goto LAB_0010618a;
  do {
    pcVar5 = param_1;
    pcVar3 = pcVar4;
    pcVar4 = pcVar3 + 1;
    param_1 = pcVar5 + 1;
    *pcVar5 = cVar2;
    cVar2 = *pcVar4;
  } while (cVar2 != '\0');
  if (pcVar4 <= param_2) goto LAB_0010618a;
  if (*pcVar3 == '/') goto LAB_0010618a;
  *param_1 = '/';
  param_1 = pcVar5 + 2;
  cVar2 = *param_3;
  while (cVar2 != '\0') {
    *param_1 = cVar2;
    param_1 = param_1 + 1;
    param_3 = param_3 + 1;
LAB_0010618a:
    cVar2 = *param_3;
  }
  *param_1 = '\0';
  return;
}




// Function: free_pending_ent @ 0x61b0

void free_pending_ent(undefined8 *param_1)

{
  free((void *)*param_1);
  free((void *)param_1[1]);
  free(param_1);
  return;
}




// Function: init_column_info @ 0x61d0

void init_column_info(ulong param_1)

{
  undefined8 *puVar1;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  long lVar4;
  long lVar5;
  undefined8 *puVar6;
  long lVar7;
  ulong uVar8;
  
  uVar8 = DAT_001262e0;
  if (DAT_001262e0 < param_1) {
    if ((DAT_00127200 == 0) || (param_1 < DAT_00127200 >> 1)) {
      DAT_00127208 = thunk_FUN_00116a90(DAT_00127208,param_1,0x30);
      uVar8 = param_1 * 2;
    }
    else {
      DAT_00127208 = thunk_FUN_00116a90(DAT_00127208,DAT_00127200,0x18);
      uVar8 = DAT_00127200;
    }
    auVar2._8_8_ = 0;
    auVar2._0_8_ = uVar8 - DAT_001262e0;
    auVar3._8_8_ = 0;
    auVar3._0_8_ = DAT_001262e0 + 1 + uVar8;
    if ((CARRY8(DAT_001262e0 + 1,uVar8)) || (SUB168(auVar2 * auVar3,8) != 0)) {
                    /* WARNING: Subroutine does not return */
      FUN_00116da0();
    }
    lVar5 = FUN_00116b00(SUB168(auVar2 * auVar3,0) >> 1);
    lVar4 = DAT_00127208;
    if (DAT_001262e0 < uVar8) {
      lVar7 = DAT_001262e0 * 8 + 8;
      do {
        *(long *)(lVar4 + -8 + lVar7 * 3) = lVar5;
        lVar5 = lVar5 + lVar7;
        lVar7 = lVar7 + 8;
      } while (lVar7 != uVar8 * 8 + 8);
    }
  }
  DAT_001262e0 = uVar8;
  lVar4 = DAT_00127208;
  if (param_1 != 0) {
    lVar7 = 0;
    lVar5 = 3;
    do {
      puVar6 = *(undefined8 **)(lVar4 + -8 + lVar5 * 8);
      *(undefined1 *)(lVar4 + -0x18 + lVar5 * 8) = 1;
      lVar7 = lVar7 + 8;
      *(long *)(lVar4 + -0x10 + lVar5 * 8) = lVar5;
      puVar1 = (undefined8 *)(lVar7 + (long)puVar6);
      do {
        *puVar6 = 3;
        puVar6 = puVar6 + 1;
      } while (puVar1 != puVar6);
      lVar5 = lVar5 + 3;
    } while (param_1 * 3 + 3 != lVar5);
  }
  return;
}




// Function: dev_ino_pop @ 0x6330

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined1  [16] dev_ino_pop(void)

{
  long lVar1;
  
  lVar1 = DAT_001270f8;
  if (0xf < (ulong)(DAT_001270f8 - _DAT_001270f0)) {
    DAT_001270f8 = DAT_001270f8 + -0x10;
    return *(undefined1 (*) [16])(lVar1 + -0x10);
  }
                    /* WARNING: Subroutine does not return */
  __assert_fail("dev_ino_size <= obstack_object_size (&dev_ino_obstack)","src/ls.c",0x41d,
                "dev_ino_pop");
}




// Function: add_ignore_pattern @ 0x6380

void add_ignore_pattern(undefined8 param_1)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_001169e0(0x10);
  *puVar1 = param_1;
  puVar1[1] = DAT_001272e8;
  DAT_001272e8 = puVar1;
  return;
}




// Function: decode_line_length @ 0x63b0

long decode_line_length(undefined8 param_1)

{
  int iVar1;
  long in_FS_OFFSET;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_00117190(param_1,0,0,&local_18,&DAT_0011abb9);
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




// Function: is_colored @ 0x6420

ulong is_colored(uint param_1)

{
  long lVar1;
  int iVar2;
  undefined8 uVar3;
  
  uVar3 = 0;
  lVar1 = (&DAT_00126060)[(ulong)param_1 * 2];
  if (lVar1 == 0) {
    return 0;
  }
  if (lVar1 != 1) {
    uVar3 = 0;
    if (lVar1 != 2) {
      return 1;
    }
    iVar2 = strncmp((&PTR_DAT_00126068)[(ulong)param_1 * 2],"00",2);
    return CONCAT71((int7)((ulong)uVar3 >> 8),iVar2 != 0) & 0xffffffff;
  }
  iVar2 = strncmp((&PTR_DAT_00126068)[(ulong)param_1 * 2],"0",1);
  return CONCAT71((int7)((ulong)uVar3 >> 8),iVar2 != 0) & 0xffffffff;
}




// Function: getenv_quoting_style @ 0x64b0

undefined4 getenv_quoting_style(void)

{
  int iVar1;
  undefined4 uVar2;
  char *pcVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  
  pcVar3 = getenv("QUOTING_STYLE");
  if (pcVar3 == (char *)0x0) {
    uVar2 = 0xffffffff;
  }
  else {
    iVar1 = FUN_0010cf70(pcVar3,&PTR_s_literal_001259e0,&DAT_0011d9c0,4);
    if (iVar1 < 0) {
      uVar4 = FUN_001158c0(pcVar3);
      uVar5 = dcgettext(0,"ignoring invalid value of environment variable QUOTING_STYLE: %s",5);
      error(0,0,uVar5,uVar4);
      uVar2 = 0xffffffff;
    }
    else {
      uVar2 = *(undefined4 *)(&DAT_0011d9c0 + (long)iVar1 * 4);
    }
  }
  return uVar2;
}




// Function: stdout_isatty @ 0x6590

uint stdout_isatty(void)

{
  uint uVar1;
  
  if (-1 < (char)DAT_00126018) {
    return DAT_00126018 & 1;
  }
  uVar1 = isatty(1);
  DAT_00126018 = (char)uVar1;
  return uVar1 & 1;
}




// Function: abmon_init @ 0x65c0

undefined8 abmon_init(long param_1)

{
  bool bVar1;
  char *__s;
  char *pcVar2;
  ushort **ppuVar3;
  ulong uVar4;
  undefined8 uVar5;
  ulong uVar6;
  long lVar7;
  ulong uVar8;
  int __item;
  long in_FS_OFFSET;
  ulong local_48;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  uVar8 = 0xc;
  do {
    __item = 0x2000e;
    uVar6 = 0;
    lVar7 = param_1;
    do {
      local_48 = uVar8;
      __s = nl_langinfo(__item);
      pcVar2 = strchr(__s,0x25);
      if (pcVar2 != (char *)0x0) {
LAB_00106690:
        uVar5 = 0;
        goto LAB_00106692;
      }
      ppuVar3 = __ctype_b_loc();
      FUN_00105570((int)*__s,lVar7,ppuVar3,&local_48);
      uVar4 = FUN_00110df0(__s);
      if (0x7f < uVar4) goto LAB_00106690;
      if (uVar6 < local_48) {
        uVar6 = local_48;
      }
      __item = __item + 1;
      lVar7 = lVar7 + 0x80;
    } while (__item != 0x2001a);
    bVar1 = uVar6 < uVar8;
    uVar8 = uVar6;
  } while (bVar1);
  uVar5 = 1;
LAB_00106692:
  if (local_40 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar5;
}




// Function: abformat_init @ 0x66c0

void abformat_init(void)

{
  undefined *puVar1;
  long lVar2;
  char cVar3;
  uint uVar4;
  long lVar5;
  undefined1 *puVar6;
  char *__s;
  long in_FS_OFFSET;
  long local_658 [2];
  undefined1 local_648 [1536];
  undefined1 local_48 [8];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  lVar5 = FUN_00105620(PTR_s__b__e__Y_00126040);
  local_658[0] = lVar5;
  local_658[1] = FUN_00105620(PTR_s__b__e__H__M_00126048);
  if (lVar5 != 0 || local_658[1] != 0) {
    cVar3 = FUN_001065c0(local_648);
    if (cVar3 != '\0') {
      lVar5 = 0;
      while( true ) {
        puVar1 = (&PTR_s__b__e__Y_00126040)[lVar5];
        lVar2 = local_658[lVar5];
        __s = &DAT_001263e0 + lVar5 * 0x600;
        puVar6 = local_648;
        do {
          if (lVar2 == 0) {
            uVar4 = snprintf(__s,0x80,"%s",puVar1);
          }
          else {
            if (0x80 < lVar2 - (long)puVar1) goto LAB_001067f0;
            uVar4 = __snprintf_chk(__s,0x80,1,0xffffffffffffffff,"%.*s%s%s",lVar2 - (long)puVar1,
                                   puVar1,puVar6,lVar2 + 2);
          }
          if (0x7f < uVar4) goto LAB_001067f0;
          puVar6 = puVar6 + 0x80;
          __s = __s + 0x80;
        } while (puVar6 != local_48);
        if (lVar5 == 1) break;
        lVar5 = 1;
      }
      DAT_001263c8 = 1;
    }
  }
LAB_001067f0:
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: queue_directory @ 0x6840

void queue_directory(long param_1,long param_2,undefined1 param_3)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  
  puVar1 = (undefined8 *)FUN_001169e0(0x20);
  uVar2 = 0;
  if (param_2 != 0) {
    uVar2 = FUN_00116d80(param_2);
  }
  puVar1[1] = uVar2;
  uVar2 = 0;
  if (param_1 != 0) {
    uVar2 = FUN_00116d80(param_1);
  }
  *puVar1 = uVar2;
  *(undefined1 *)(puVar1 + 2) = param_3;
  puVar1[3] = DAT_00127380;
  DAT_00127380 = puVar1;
  return;
}




// Function: time_type_to_statx @ 0x68b0

undefined4 time_type_to_statx(void)

{
  if (DAT_00127334 < 4) {
    return *(undefined4 *)(&DAT_00119620 + (ulong)DAT_00127334 * 4);
  }
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: calc_req_mask @ 0x68d0

uint calc_req_mask(void)

{
  uint uVar1;
  uint uVar2;
  
  uVar2 = (-(uint)(DAT_001272fc == '\0') & 0xffffff00) + 0x102;
  if (DAT_0012732c != '\0') {
    uVar2 = uVar2 | 0x400;
  }
  if (DAT_00127338 == 0) {
    uVar1 = FUN_001068b0();
    if ((DAT_00126029 == '\0') && (DAT_0012732e == '\0')) {
      uVar2 = uVar2 | uVar1 | 0x204;
    }
    else {
      uVar2 = uVar2 | uVar1 | 0x20c;
    }
    if (DAT_00126028 != '\0') {
      uVar2 = uVar2 | 0x10;
    }
  }
  switch(DAT_00127330) {
  case 0:
  case 1:
  case 2:
  case 4:
  case 6:
    goto switchD_00106942_caseD_0;
  case 3:
    return uVar2 | 0x200;
  case 5:
    uVar1 = FUN_001068b0();
    uVar2 = uVar2 | uVar1;
switchD_00106942_caseD_0:
    return uVar2;
  default:
                    /* WARNING: Subroutine does not return */
    abort();
  }
}




// Function: patterns_match @ 0x6990

undefined8 patterns_match(undefined8 *param_1,char *param_2)

{
  int iVar1;
  
  if (param_1 == (undefined8 *)0x0) {
    return 0;
  }
  do {
    iVar1 = fnmatch((char *)*param_1,param_2,4);
    if (iVar1 == 0) {
      return 1;
    }
    param_1 = (undefined8 *)param_1[1];
  } while (param_1 != (undefined8 *)0x0);
  return 0;
}




// Function: file_ignored @ 0x69f0

undefined8 file_ignored(char *param_1)

{
  char cVar1;
  undefined8 uVar2;
  
  if (DAT_001272f0 != 2) {
    if (*param_1 == '.') {
      if ((DAT_001272f0 == 0) || (param_1[(ulong)(param_1[1] == '.') + 1] == '\0')) {
        return 1;
      }
    }
    else if ((DAT_001272f0 == 0) && (cVar1 = FUN_00106990(DAT_001272e0,param_1), cVar1 != '\0')) {
      return 1;
    }
  }
  uVar2 = FUN_00106990(DAT_001272e8,param_1);
  return uVar2;
}




// Function: known_term_type @ 0x6a50

undefined8 known_term_type(void)

{
  int iVar1;
  char *__name;
  size_t sVar2;
  char *__s1;
  
  __name = getenv("TERM");
  if ((__name != (char *)0x0) && (*__name != '\0')) {
    __s1 = "# Configuration file for dircolors, a utility to help you set the";
    do {
      iVar1 = strncmp(__s1,"TERM ",5);
      if ((iVar1 == 0) && (iVar1 = fnmatch(__s1 + 5,__name,0), iVar1 == 0)) {
        return 1;
      }
      sVar2 = strlen(__s1);
      __s1 = __s1 + sVar2 + 1;
    } while (__s1 + -0x119780 < (char *)0x12c8);
  }
  return 0;
}




// Function: parse_ls_color @ 0x6af0

void parse_ls_color(void)

{
  char *pcVar1;
  void *pvVar2;
  char cVar3;
  int iVar4;
  char *pcVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  void *pvVar8;
  long lVar9;
  undefined8 in_R8;
  undefined8 in_R9;
  long in_FS_OFFSET;
  undefined **ppuVar10;
  char *local_58;
  undefined *local_50;
  undefined2 local_43;
  undefined1 local_41;
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_58 = getenv("LS_COLORS");
  if ((local_58 == (char *)0x0) || (*local_58 == '\0')) {
    pcVar5 = getenv("COLORTERM");
    if (((pcVar5 == (char *)0x0) || (*pcVar5 == '\0')) && (cVar3 = FUN_00106a50(), cVar3 == '\0')) {
      DAT_00127312 = 0;
    }
  }
  else {
    local_43 = 0x3f3f;
    local_41 = 0;
    DAT_00127300 = (undefined *)FUN_00116d80(local_58);
    local_50 = DAT_00127300;
    do {
      while( true ) {
        while (cVar3 = *local_58, cVar3 == '*') {
          pcVar5 = local_58 + 1;
          pvVar8 = (void *)FUN_001169e0(0x28);
          ppuVar10 = &local_50;
          *(void **)((long)pvVar8 + 0x20) = DAT_00127308;
          DAT_00127308 = pvVar8;
          *(undefined **)((long)pvVar8 + 8) = local_50;
          local_58 = pcVar5;
          cVar3 = FUN_00105740(ppuVar10,&local_58,1,pvVar8,in_R8,in_R9,ppuVar10);
          pcVar5 = local_58;
          if ((cVar3 == '\0') || (pcVar5 = local_58 + 1, *local_58 != '=')) goto LAB_00106c65;
          *(undefined **)((long)pvVar8 + 0x18) = local_50;
          local_58 = local_58 + 1;
          cVar3 = FUN_00105740(ppuVar10,&local_58,0,(long)pvVar8 + 0x10,in_R8,in_R9,ppuVar10);
          pcVar5 = local_58;
          if (cVar3 == '\0') goto LAB_00106c65;
        }
        if (cVar3 != ':') break;
        local_58 = local_58 + 1;
      }
      if (cVar3 == '\0') goto LAB_00106cb8;
      local_43 = CONCAT11(local_43._1_1_,cVar3);
      pcVar5 = local_58 + 1;
      if (local_58[1] == '\0') goto LAB_00106c65;
      local_43 = CONCAT11(local_58[1],cVar3);
      pcVar1 = local_58 + 2;
      local_58 = local_58 + 3;
      pcVar5 = local_58;
      if (*pcVar1 != '=') goto LAB_00106c65;
      lVar9 = 0;
      pcVar5 = "lc";
      while (iVar4 = strcmp((char *)&local_43,pcVar5), iVar4 != 0) {
        lVar9 = lVar9 + 1;
        pcVar5 = (&PTR_DAT_00125860)[lVar9];
        if (pcVar5 == (char *)0x0) goto LAB_00106c36;
      }
      (&PTR_DAT_00126068)[(long)(int)lVar9 * 2] = local_50;
      cVar3 = FUN_00105740(&local_50,&local_58,0);
    } while (cVar3 != '\0');
LAB_00106c36:
    uVar6 = FUN_001158c0(&local_43);
    uVar7 = dcgettext(0,"unrecognized prefix: %s",5);
    error(0,0,uVar7,uVar6);
    pcVar5 = local_58;
LAB_00106c65:
    local_58 = pcVar5;
    uVar6 = dcgettext(0,"unparsable value for LS_COLORS environment variable",5);
    error(0,0,uVar6);
    free(DAT_00127300);
    pvVar8 = DAT_00127308;
    while (pvVar8 != (void *)0x0) {
      pvVar2 = *(void **)((long)pvVar8 + 0x20);
      free(pvVar8);
      pvVar8 = pvVar2;
    }
    DAT_00127312 = 0;
LAB_00106cb8:
    if ((DAT_001260d0 == 6) && (iVar4 = strncmp(PTR_s_01_36_001260d8,"target",6), iVar4 == 0)) {
      DAT_00127390 = 1;
    }
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: free_ent @ 0x6dc0

void free_ent(undefined8 *param_1)

{
  char cVar1;
  undefined *__ptr;
  
  free((void *)*param_1);
  free((void *)param_1[1]);
  free((void *)param_1[2]);
  __ptr = (undefined *)param_1[0x16];
  if (__ptr == &DAT_0012602a) {
    return;
  }
  cVar1 = FUN_001055d0();
  if (cVar1 != '\0') {
    free(__ptr);
    return;
  }
  freecon();
  return;
}




// Function: clear_files @ 0x6e20

void clear_files(void)

{
  long lVar1;
  ulong uVar2;
  
  if (DAT_001273b0 != 0) {
    uVar2 = 0;
    do {
      lVar1 = uVar2 * 8;
      uVar2 = uVar2 + 1;
      FUN_00106dc0(*(undefined8 *)(DAT_001273a0 + lVar1));
    } while (uVar2 < DAT_001273b0);
  }
  DAT_001273b0 = 0;
  DAT_001273a9 = 0;
  DAT_00127364 = 0;
  DAT_00127360 = 0;
  DAT_0012735c = 0;
  DAT_00127358 = 0;
  DAT_00127350 = 0;
  DAT_0012734c = 0;
  DAT_00127348 = 0;
  DAT_00127354 = 0;
  DAT_00127344 = 0;
  DAT_00127340 = 0;
  DAT_0012733c = 0;
  return;
}




// Function: format_user_or_group_width @ 0x6ed0

ulong format_user_or_group_width(long param_1,undefined8 param_2)

{
  uint uVar1;
  ulong uVar2;
  
  if (param_1 != 0) {
    uVar1 = FUN_001113a0(param_1,0);
    uVar2 = (ulong)uVar1;
    if ((int)uVar1 < 0) {
      uVar2 = 0;
    }
    return uVar2;
  }
  uVar2 = __snprintf_chk(0,0,1,0xffffffffffffffff,&DAT_0011aaa7,param_2);
  return uVar2;
}




// Function: format_inode @ 0x6f10

undefined1 * format_inode(undefined8 param_1,undefined8 param_2,long param_3)

{
  undefined1 *puVar1;
  
  if (*(char *)(param_3 + 0xb8) == '\0') {
    return &DAT_0011aaab;
  }
  if (*(long *)(param_3 + 0x20) == 0) {
    return &DAT_0011aaab;
  }
  puVar1 = (undefined1 *)FUN_00110c20(*(long *)(param_3 + 0x20),param_1);
  return puVar1;
}




// Function: file_failure @ 0x6f50

void file_failure(undefined1 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 uVar1;
  int *piVar2;
  
  uVar1 = FUN_00115600(4,param_3);
  piVar2 = __errno_location();
  error(0,*piVar2,param_2,uVar1);
  FUN_001059b0(param_1);
  return;
}




// Function: getfilecon_cache @ 0x6f90

int getfilecon_cache(undefined8 param_1,long param_2,char param_3)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  
  if (*(long *)(param_2 + 0x18) == DAT_001262f0) {
    piVar3 = __errno_location();
    iVar2 = -1;
    *piVar3 = 0x5f;
  }
  else {
    if (param_3 == '\0') {
      iVar2 = FUN_00115b30();
    }
    else {
      iVar2 = FUN_00115b10(param_1,param_2 + 0xb0);
    }
    if (iVar2 < 0) {
      piVar3 = __errno_location();
      cVar1 = FUN_001059f0(*piVar3);
      if (cVar1 != '\0') {
        DAT_001262f0 = *(undefined8 *)(param_2 + 0x18);
        return iVar2;
      }
    }
  }
  return iVar2;
}




// Function: file_has_acl_cache @ 0x7020

ulong file_has_acl_cache(undefined8 param_1,long param_2)

{
  char cVar1;
  uint uVar2;
  int *piVar3;
  ulong uVar4;
  
  piVar3 = __errno_location();
  if (*(long *)(param_2 + 0x18) == DAT_001262e8) {
    *piVar3 = 0x5f;
    return 0;
  }
  *piVar3 = 0;
  uVar2 = FUN_0010e060(param_1,param_2 + 0x18);
  uVar4 = (ulong)uVar2;
  if ((int)uVar2 < 1) {
    cVar1 = FUN_001059f0(*piVar3);
    if (cVar1 != '\0') {
      DAT_001262e8 = *(undefined8 *)(param_2 + 0x18);
      return uVar4 & 0xffffffff;
    }
  }
  return uVar4 & 0xffffffff;
}




// Function: get_link_name @ 0x70b0

void get_link_name(undefined8 param_1,long param_2,undefined1 param_3)

{
  long lVar1;
  undefined8 uVar2;
  
  lVar1 = FUN_0010cdd0(param_1,*(undefined8 *)(param_2 + 0x48));
  *(long *)(param_2 + 8) = lVar1;
  if (lVar1 != 0) {
    return;
  }
  uVar2 = dcgettext(0,"cannot read symbolic link %s",5);
  FUN_00106f50(param_3,uVar2,param_1);
  return;
}




// Function: make_link_name @ 0x7100

char * make_link_name(char *param_1,char *param_2)

{
  size_t __n;
  size_t sVar1;
  char *pcVar2;
  char *__dest;
  
  if (param_2 == (char *)0x0) {
    pcVar2 = (char *)0x0;
  }
  else {
    if ((*param_2 == '/') || (__n = FUN_0010dfb0(), __n == 0)) {
      pcVar2 = (char *)FUN_00116d80(param_2);
      return pcVar2;
    }
    sVar1 = strlen(param_2);
    pcVar2 = (char *)FUN_001169e0(__n + 2 + sVar1);
    if (param_1[__n - 1] != '/') {
      __n = __n + 1;
    }
    __dest = stpncpy(pcVar2,param_1,__n);
    strcpy(__dest,param_2);
  }
  return pcVar2;
}




// Function: needs_quoting @ 0x71a0

bool needs_quoting(char *param_1)

{
  size_t sVar1;
  size_t sVar2;
  long in_FS_OFFSET;
  bool bVar3;
  char local_22 [2];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  sVar1 = FUN_001152c0(local_22,2,param_1,0xffffffffffffffff,DAT_001272d0);
  if (*param_1 == local_22[0]) {
    sVar2 = strlen(param_1);
    bVar3 = sVar2 != sVar1;
  }
  else {
    bVar3 = true;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: format_group_width @ 0x7220

void format_group_width(undefined4 param_1)

{
  undefined8 uVar1;
  
  if (DAT_0012732d != '\0') {
    FUN_00106ed0(0,param_1);
    return;
  }
  uVar1 = FUN_001109e0();
  FUN_00106ed0(uVar1,param_1);
  return;
}




// Function: format_user_width @ 0x7260

void format_user_width(undefined4 param_1)

{
  undefined8 uVar1;
  
  if (DAT_0012732d != '\0') {
    FUN_00106ed0(0,param_1);
    return;
  }
  uVar1 = FUN_00110840();
  FUN_00106ed0(uVar1,param_1);
  return;
}




// Function: xstrcoll @ 0x72a0

void xstrcoll(char *param_1,char *param_2)

{
  int *piVar1;
  
  piVar1 = __errno_location();
  *piVar1 = 0;
  strcoll(param_1,param_2);
  return;
}




// Function: cmp_extension @ 0x72d0

void cmp_extension(undefined8 *param_1,undefined8 *param_2,code *UNRECOVERED_JUMPTABLE)

{
  int iVar1;
  char *pcVar2;
  char *pcVar3;
  
  pcVar2 = strrchr((char *)*param_1,0x2e);
  pcVar3 = strrchr((char *)*param_2,0x2e);
  if (pcVar3 == (char *)0x0) {
    pcVar3 = "";
  }
  if (pcVar2 == (char *)0x0) {
    pcVar2 = "";
  }
  iVar1 = (*UNRECOVERED_JUMPTABLE)(pcVar2,pcVar3);
  if (iVar1 == 0) {
                    /* WARNING: Could not recover jumptable at 0x00107337. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*UNRECOVERED_JUMPTABLE)(*param_1,*param_2);
    return;
  }
  return;
}




// Function: rev_strcmp_extension @ 0x7350

void rev_strcmp_extension(undefined8 param_1,undefined8 param_2)

{
  FUN_001072d0(param_2,param_1,PTR_strcmp_00125fc0);
  return;
}




// Function: basename_is_dot_or_dotdot @ 0x73b0

void basename_is_dot_or_dotdot(void)

{
  undefined8 uVar1;
  
  uVar1 = FUN_0010d360();
  FUN_00105580(uVar1);
  return;
}




// Function: indent @ 0x73d0

ulong indent(ulong param_1,ulong param_2)

{
  uint uVar1;
  ulong in_RAX;
  ulong uVar2;
  
  if (param_2 <= param_1) {
    return in_RAX;
  }
  do {
    while ((uVar2 = param_1 + 1, DAT_001272c0 != 0 &&
           (uVar2 / DAT_001272c0 < param_2 / DAT_001272c0))) {
      putchar_unlocked(9);
      uVar2 = param_1 / DAT_001272c0;
      param_1 = (DAT_001272c0 + param_1) - param_1 % DAT_001272c0;
      if (param_2 <= param_1) {
        return uVar2;
      }
    }
    uVar1 = putchar_unlocked(0x20);
    param_1 = uVar2;
  } while (uVar2 < param_2);
  return (ulong)uVar1;
}




// Function: dired_outbyte @ 0x7460

void dired_outbyte(char param_1)

{
  DAT_001271f8 = DAT_001271f8 + 1;
  putchar_unlocked((int)param_1);
  return;
}




// Function: get_color_indicator @ 0x7480

ulong * get_color_indicator(undefined8 *param_1,char param_2)

{
  uint uVar1;
  int iVar2;
  long lVar3;
  ulong *puVar4;
  size_t sVar5;
  ulong *puVar6;
  ulong uVar7;
  uint uVar8;
  char *__s;
  uint uVar9;
  undefined8 *puVar10;
  char cVar11;
  
  if (param_2 == '\0') {
    puVar10 = param_1;
    uVar1 = FUN_001055e0();
    __s = (char *)*puVar10;
    uVar9 = (uint)*(byte *)((long)puVar10 + 0xb9);
LAB_00107542:
    cVar11 = *(char *)(param_1 + 0x17);
    uVar8 = uVar9;
    if (cVar11 == '\0') goto LAB_001074b9;
LAB_00107551:
    uVar8 = uVar1 & 0xf000;
    if (uVar8 == 0x8000) {
      if (((uVar1 & 0x800) != 0) && (cVar11 = FUN_00106420(0x10), cVar11 != '\0')) {
        lVar3 = 0x100;
        goto LAB_0010750f;
      }
      if (((uVar1 & 0x400) != 0) && (cVar11 = FUN_00106420(0x11), cVar11 != '\0')) {
        lVar3 = 0x110;
        goto LAB_0010750f;
      }
      cVar11 = FUN_00106420(0x15);
      if ((cVar11 != '\0') && (*(char *)(param_1 + 0x18) != '\0')) {
        lVar3 = 0x150;
        goto LAB_0010750f;
      }
      if (((uVar1 & 0x49) != 0) && (cVar11 = FUN_00106420(0xe), cVar11 != '\0')) {
        lVar3 = 0xe0;
        goto LAB_0010750f;
      }
      if ((1 < (ulong)param_1[5]) && (cVar11 = FUN_00106420(0x16), cVar11 != '\0')) {
        lVar3 = 0x160;
        goto LAB_0010750f;
      }
      goto LAB_00107660;
    }
    if (uVar8 == 0x4000) {
      if ((uVar1 & 0x202) == 0x202) {
        cVar11 = FUN_00106420(0x14);
        lVar3 = 0x140;
        if (cVar11 != '\0') goto LAB_0010750f;
      }
      if ((uVar1 & 2) != 0) {
        cVar11 = FUN_00106420(0x13);
        lVar3 = 0x130;
        if (cVar11 != '\0') goto LAB_0010750f;
      }
      lVar3 = 0x60;
      if ((uVar1 & 0x200) != 0) {
        cVar11 = FUN_00106420(0x12);
        lVar3 = (-(ulong)(cVar11 == '\0') & 0xffffffffffffff40) + 0x120;
      }
      goto LAB_0010750f;
    }
    if (uVar8 != 0xa000) {
      lVar3 = 0x80;
      if ((((uVar8 != 0x1000) && (lVar3 = 0x90, uVar8 != 0xc000)) && (lVar3 = 0xa0, uVar8 != 0x6000)
          ) && (lVar3 = 0xd0, uVar8 == 0x2000)) {
        lVar3 = 0xb0;
      }
      goto LAB_0010750f;
    }
    uVar7 = 7;
  }
  else {
    __s = (char *)param_1[1];
    uVar1 = *(uint *)((long)param_1 + 0xac);
    if (*(char *)((long)param_1 + 0xb9) == '\0') {
      cVar11 = FUN_00106420(0xc);
      uVar9 = 0xffffffff;
      if (cVar11 != '\0') {
        lVar3 = 0xc0;
        goto LAB_0010750f;
      }
      goto LAB_00107542;
    }
    cVar11 = *(char *)(param_1 + 0x17);
    uVar9 = 0;
    uVar8 = 0;
    if (cVar11 != '\0') goto LAB_00107551;
LAB_001074b9:
    uVar9 = uVar8;
    uVar1 = *(uint *)(&DAT_00119680 + (ulong)*(uint *)(param_1 + 0x15) * 4);
    uVar7 = (ulong)uVar1;
    cVar11 = uVar1 == 7;
    if (uVar1 == 5) {
LAB_00107660:
      sVar5 = strlen(__s);
      for (puVar4 = DAT_00127308; puVar4 != (ulong *)0x0; puVar4 = (ulong *)puVar4[4]) {
        if ((*puVar4 <= sVar5) &&
           (iVar2 = FUN_0010d5c0(__s + (sVar5 - *puVar4),puVar4[1]), iVar2 == 0)) {
          puVar6 = puVar4 + 2;
          if (puVar4[3] == 0) {
            puVar6 = (ulong *)0x0;
          }
          return puVar6;
        }
      }
      lVar3 = 0x50;
      goto LAB_0010750f;
    }
  }
  if ((uVar9 == 0) && (cVar11 != '\0')) {
    lVar3 = 0xd0;
    if (DAT_00127390 == '\0') {
      cVar11 = FUN_00106420(0xd);
      lVar3 = (-(ulong)(cVar11 == '\0') & 0xffffffffffffffa0) + 0xd0;
    }
  }
  else {
    lVar3 = uVar7 << 4;
  }
LAB_0010750f:
  puVar4 = (ulong *)((long)&DAT_00126060 + lVar3);
  if (*(long *)((long)&PTR_DAT_00126068 + lVar3) == 0) {
    puVar4 = (ulong *)0x0;
  }
  return puVar4;
}




// Function: dired_outbuf @ 0x77d0

void dired_outbuf(void *param_1,size_t param_2)

{
  DAT_001271f8 = DAT_001271f8 + param_2;
  fwrite_unlocked(param_1,1,param_2,stdout);
  return;
}




// Function: dired_outstring @ 0x77f0

void dired_outstring(char *param_1)

{
  size_t sVar1;
  
  sVar1 = strlen(param_1);
  FUN_001077d0(param_1,sVar1);
  return;
}




// Function: dired_indent @ 0x7810

void dired_indent(void)

{
  if (DAT_00127318 == '\0') {
    return;
  }
  FUN_001077f0(&DAT_0011aaca);
  return;
}




// Function: file_escape @ 0x7830

undefined1 * file_escape(char *param_1,char param_2)

{
  char cVar1;
  undefined1 *puVar2;
  byte bVar3;
  size_t sVar4;
  undefined1 *puVar5;
  undefined1 extraout_DL;
  
  sVar4 = strlen(param_1);
  puVar5 = (undefined1 *)FUN_00116b00(3,sVar4 + 1);
  cVar1 = *param_1;
  puVar2 = puVar5;
  do {
    while( true ) {
      if (cVar1 == '\0') {
        *puVar2 = 0;
        return puVar5;
      }
      param_1 = param_1 + 1;
      if ((cVar1 != '/') || (param_2 == '\0')) break;
      *puVar2 = 0x2f;
LAB_0010787f:
      cVar1 = *param_1;
      puVar2 = puVar2 + 1;
    }
    bVar3 = FUN_00105570((int)cVar1);
    if ((&DAT_00126fe0)[bVar3] != '\0') {
      *puVar2 = extraout_DL;
      goto LAB_0010787f;
    }
    __sprintf_chk(puVar2,1,0xffffffffffffffff,"%%%02x",bVar3);
    cVar1 = *param_1;
    puVar2 = puVar2 + 3;
  } while( true );
}




// Function: push_current_dired_pos @ 0x7900

void push_current_dired_pos(long param_1)

{
  if (DAT_00127318 == '\0') {
    return;
  }
  if (7 < (ulong)(*(long *)(param_1 + 0x20) - (long)*(undefined8 **)(param_1 + 0x18))) {
    **(undefined8 **)(param_1 + 0x18) = DAT_001271f8;
    *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 8;
    return;
  }
  _obstack_newchunk(param_1,8);
  **(undefined8 **)(param_1 + 0x18) = DAT_001271f8;
  *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 8;
  return;
}




// Function: dev_ino_push @ 0x7970

void dev_ino_push(undefined8 param_1,undefined8 param_2)

{
  undefined8 *puVar1;
  
  puVar1 = DAT_001270f8;
  if ((ulong)(DAT_00127100 - (long)DAT_001270f8) < 0x10) {
    _obstack_newchunk(&DAT_001270e0,0x10);
    puVar1 = DAT_001270f8;
  }
  DAT_001270f8 = puVar1 + 2;
  puVar1[1] = param_1;
  *puVar1 = param_2;
  return;
}




// Function: signal_setup @ 0x79d0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void signal_setup(char param_1)

{
  int iVar1;
  int *piVar2;
  int iVar3;
  int *piVar4;
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
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 == '\0') {
    piVar2 = &DAT_00119644;
    iVar3 = 0x14;
    while( true ) {
      iVar1 = sigismember((sigset_t *)&DAT_00127220,iVar3);
      if (iVar1 != 0) {
        signal(iVar3,(__sighandler_t)0x0);
      }
      if (piVar2 == (int *)&DAT_00119670) break;
      iVar3 = *piVar2;
      piVar2 = piVar2 + 1;
    }
  }
  else {
    piVar2 = &DAT_00119644;
    iVar3 = 0x14;
    piVar4 = &DAT_00119644;
    sigemptyset((sigset_t *)&DAT_00127220);
    while( true ) {
      sigaction(iVar3,(sigaction *)0x0,(sigaction *)&local_d8);
      if (local_d8.sa_handler != (__sighandler_t)0x1) {
        sigaddset((sigset_t *)&DAT_00127220,iVar3);
      }
      if (piVar4 == (int *)&DAT_00119670) break;
      iVar3 = *piVar4;
      piVar4 = piVar4 + 1;
    }
    local_50 = 0x10000000;
    iVar3 = 0x14;
    local_d0 = _DAT_00127220;
    uStack_c8 = uRam0000000000127228;
    local_c0 = _DAT_00127230;
    uStack_b8 = uRam0000000000127238;
    local_b0 = _DAT_00127240;
    uStack_a8 = uRam0000000000127248;
    local_a0 = _DAT_00127250;
    uStack_98 = uRam0000000000127258;
    local_90 = _DAT_00127260;
    uStack_88 = uRam0000000000127268;
    local_80 = _DAT_00127270;
    uStack_78 = uRam0000000000127278;
    local_70 = _DAT_00127280;
    uStack_68 = uRam0000000000127288;
    local_60 = _DAT_00127290;
    uStack_58 = uRam0000000000127298;
    while( true ) {
      iVar1 = sigismember((sigset_t *)&DAT_00127220,iVar3);
      if (iVar1 != 0) {
        local_d8.sa_handler = FUN_00105700;
        if (iVar3 == 0x14) {
          local_d8.sa_handler = FUN_00105720;
        }
        sigaction(iVar3,(sigaction *)&local_d8,(sigaction *)0x0);
      }
      if (piVar2 == (int *)&DAT_00119670) break;
      iVar3 = *piVar2;
      piVar2 = piVar2 + 1;
    }
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: signal_init @ 0x7ba0

void signal_init(void)

{
  FUN_001079d0(1);
  return;
}




// Function: put_indicator @ 0x7bb0

void put_indicator(size_t *param_1)

{
  __pid_t _Var1;
  
  if (DAT_00127310 == '\0') {
    DAT_00127310 = '\x01';
    _Var1 = tcgetpgrp(1);
    if (_Var1 < 0) {
      FUN_00107c10();
    }
    else {
      FUN_00107ba0();
      FUN_00107c10();
    }
  }
  fwrite_unlocked((void *)param_1[1],*param_1,1,stdout);
  return;
}




// Function: prep_non_filename_text @ 0x7c10

void prep_non_filename_text(void)

{
  if (DAT_00126088 != 0) {
    FUN_00107bb0(&DAT_00126080);
    return;
  }
  FUN_00107bb0(&DAT_00126060);
  FUN_00107bb0(&DAT_00126090);
  FUN_00107bb0(&DAT_00126070);
  return;
}




// Function: set_normal_color @ 0x7c60

void set_normal_color(void)

{
  char cVar1;
  
  if (DAT_00127312 == '\0') {
    return;
  }
  cVar1 = FUN_00106420(4);
  if (cVar1 == '\0') {
    return;
  }
  FUN_00107bb0(&DAT_00126060);
  FUN_00107bb0(&DAT_001260a0);
  FUN_00107bb0(&DAT_00126070);
  return;
}




// Function: restore_default_color @ 0x7cc0

void restore_default_color(void)

{
  FUN_00107bb0(&DAT_00126060);
  FUN_00107bb0(&DAT_00126070);
  return;
}




// Function: process_signals @ 0x7ce0

void process_signals(void)

{
  int __sig;
  long in_FS_OFFSET;
  sigset_t sStack_a8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  while ((DAT_00127218 != 0 || (DAT_00127214 != 0))) {
    if (DAT_00127310 != '\0') {
      FUN_00107cc0();
    }
    fflush_unlocked(stdout);
    sigprocmask(0,(sigset_t *)&DAT_00127220,&sStack_a8);
    __sig = DAT_00127218;
    if (DAT_00127214 == 0) {
      signal(DAT_00127218,(__sighandler_t)0x0);
    }
    else {
      DAT_00127214 = DAT_00127214 + -1;
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




// Function: signal_restore @ 0x7dd0

void signal_restore(void)

{
  FUN_001079d0(0);
  return;
}




// Function: visit_dir @ 0x7df0

ulong visit_dir(undefined8 param_1,undefined8 param_2)

{
  undefined8 uVar1;
  undefined8 *__ptr;
  undefined8 *puVar2;
  
  __ptr = (undefined8 *)FUN_001169e0(0x10);
  uVar1 = DAT_001273c8;
  *__ptr = param_2;
  __ptr[1] = param_1;
  puVar2 = (undefined8 *)FUN_0010f9a0(uVar1,__ptr);
  if (puVar2 != (undefined8 *)0x0) {
    if (__ptr != puVar2) {
      free(__ptr);
    }
    return CONCAT71((int7)((ulong)param_2 >> 8),__ptr != puVar2) & 0xffffffff;
  }
                    /* WARNING: Subroutine does not return */
  FUN_00116da0();
}




// Function: dired_dump_obstack @ 0x8060

void dired_dump_obstack(char *param_1,long param_2)

{
  undefined8 uVar1;
  ulong uVar2;
  ulong uVar3;
  undefined8 *puVar4;
  undefined8 *puVar5;
  
  puVar5 = *(undefined8 **)(param_2 + 0x18);
  puVar4 = *(undefined8 **)(param_2 + 0x10);
  if (7 < (ulong)((long)puVar5 - (long)puVar4)) {
    if (puVar5 == puVar4) {
      *(byte *)(param_2 + 0x50) = *(byte *)(param_2 + 0x50) | 2;
    }
    uVar2 = (long)puVar5 + *(ulong *)(param_2 + 0x30) & ~*(ulong *)(param_2 + 0x30);
    uVar3 = *(ulong *)(param_2 + 0x20);
    if (uVar2 - *(long *)(param_2 + 8) <= *(ulong *)(param_2 + 0x20) - *(long *)(param_2 + 8)) {
      uVar3 = uVar2;
    }
    puVar5 = (undefined8 *)(((long)puVar5 - (long)puVar4 & 0xfffffffffffffff8U) + (long)puVar4);
    *(ulong *)(param_2 + 0x18) = uVar3;
    *(ulong *)(param_2 + 0x10) = uVar3;
    fputs_unlocked(param_1,stdout);
    do {
      uVar1 = *puVar4;
      puVar4 = puVar4 + 1;
      __printf_chk(1,&DAT_0011ab7a,uVar1);
    } while (puVar5 != puVar4);
    putchar_unlocked(10);
    return;
  }
  return;
}




// Function: format_user_or_group @ 0x8100

void format_user_or_group(long param_1,undefined8 param_2,int param_3)

{
  int iVar1;
  int iVar2;
  bool bVar3;
  
  if (param_1 != 0) {
    iVar1 = FUN_001113a0(param_1,0);
    iVar2 = param_3 - iVar1;
    if (param_3 - iVar1 < 0) {
      iVar2 = 0;
    }
    FUN_001077f0(param_1);
    do {
      FUN_00107460(0x20);
      bVar3 = iVar2 != 0;
      iVar2 = iVar2 + -1;
    } while (bVar3);
    return;
  }
  iVar2 = __printf_chk(1,"%*lu ",param_3,param_2);
  DAT_001271f8 = DAT_001271f8 + iVar2;
  return;
}




// Function: format_group @ 0x8180

void format_group(undefined4 param_1,undefined4 param_2,char param_3)

{
  undefined1 *puVar1;
  
  puVar1 = &DAT_0011aaab;
  if ((param_3 != '\0') && (puVar1 = (undefined1 *)0x0, DAT_0012732d == '\0')) {
    puVar1 = (undefined1 *)FUN_001109e0();
  }
  FUN_00108100(puVar1,param_1,param_2);
  return;
}




// Function: format_user @ 0x81d0

void format_user(undefined4 param_1,undefined4 param_2,char param_3)

{
  undefined1 *puVar1;
  
  puVar1 = &DAT_0011aaab;
  if ((param_3 != '\0') && (puVar1 = (undefined1 *)0x0, DAT_0012732d == '\0')) {
    puVar1 = (undefined1 *)FUN_00110840();
  }
  FUN_00108100(puVar1,param_1,param_2);
  return;
}




// Function: long_time_expected_width @ 0x82b0

ulong long_time_expected_width(void)

{
  long lVar1;
  ulong uVar2;
  long in_FS_OFFSET;
  undefined8 local_460;
  undefined1 local_458 [64];
  undefined1 local_418 [1016];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar2 = (ulong)DAT_00126014;
  if (-1 < (int)DAT_00126014) goto LAB_001082d5;
  local_460 = 0;
  lVar1 = FUN_001160a0(DAT_001272a8,&local_460,local_458);
  if (lVar1 == 0) {
LAB_00108348:
    uVar2 = (ulong)DAT_00126014;
  }
  else {
    lVar1 = FUN_00108240(local_418,0,local_458,DAT_001272a8,0);
    if (lVar1 == 0) goto LAB_00108348;
    uVar2 = FUN_00111180(local_418,lVar1,0);
    DAT_00126014 = (uint)uVar2;
  }
  if ((int)uVar2 < 0) {
    DAT_00126014 = 0;
    uVar2 = 0;
  }
LAB_001082d5:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: quote_name_width @ 0x87f0

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
  FUN_00108390(&local_2038,param_1,param_2,param_3,&local_2030,&local_2039);
  if ((local_2038 != local_2028) && (local_2038 != param_1)) {
    free(local_2038);
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return (ulong)local_2039 + local_2030;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: fileinfo_name_width @ 0x8890

void fileinfo_name_width(undefined8 *param_1)

{
  if (param_1[0x19] != 0) {
    return;
  }
  FUN_001087f0(*param_1,DAT_001272d0,*(undefined4 *)((long)param_1 + 0xc4));
  return;
}




// Function: cmp_width @ 0x88c0

void cmp_width(undefined8 *param_1,undefined8 *param_2,code *UNRECOVERED_JUMPTABLE)

{
  int iVar1;
  int iVar2;
  
  iVar1 = FUN_00108890();
  iVar2 = FUN_00108890(param_2);
  if (iVar1 == iVar2) {
                    /* WARNING: Could not recover jumptable at 0x00108902. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*UNRECOVERED_JUMPTABLE)(*param_1,*param_2);
    return;
  }
  return;
}




// Function: rev_strcmp_width @ 0x8920

void rev_strcmp_width(undefined8 param_1,undefined8 param_2)

{
  FUN_001088c0(param_2,param_1,PTR_strcmp_00125fc0);
  return;
}




// Function: strcmp_width @ 0x8940

void strcmp_width(undefined8 param_1,undefined8 param_2)

{
  FUN_001088c0(param_1,param_2,PTR_strcmp_00125fc0);
  return;
}




// Function: rev_xstrcoll_width @ 0x8950

void rev_xstrcoll_width(undefined8 param_1,undefined8 param_2)

{
  FUN_001088c0(param_2,param_1,FUN_001072a0);
  return;
}




// Function: xstrcoll_width @ 0x8970

void xstrcoll_width(undefined8 param_1,undefined8 param_2)

{
  FUN_001088c0(param_1,param_2,FUN_001072a0);
  return;
}




// Function: update_current_files_info @ 0x8980

void update_current_files_info(void)

{
  long lVar1;
  ulong uVar2;
  undefined8 uVar3;
  ulong uVar4;
  
  if ((DAT_00127330 != 2) && ((DAT_001272b0 == 0 || (1 < DAT_00127338 - 2U)))) {
    return;
  }
  uVar4 = 0;
  if (DAT_001273b0 != 0) {
    do {
      lVar1 = *(long *)(DAT_001273a0 + uVar4 * 8);
      uVar4 = uVar4 + 1;
      uVar3 = FUN_00108890(lVar1);
      uVar2 = DAT_001273b0;
      *(undefined8 *)(lVar1 + 200) = uVar3;
    } while (uVar4 < uVar2);
  }
  return;
}




// Function: sort_files @ 0x89f0

void sort_files(void)

{
  int iVar1;
  int iVar2;
  ulong uVar3;
  
  uVar3 = DAT_001273b0;
  if (DAT_00127398 < (DAT_001273b0 >> 1) + DAT_001273b0) {
    free(DAT_001273a0);
    DAT_001273a0 = (void *)FUN_00116b00(uVar3,0x18);
    DAT_00127398 = DAT_001273b0 * 3;
  }
  FUN_00106000();
  FUN_00108980();
  if (DAT_00127330 != 6) {
    iVar1 = _setjmp((__jmp_buf_tag *)&DAT_00126300);
    uVar3 = (ulong)DAT_00127330;
    iVar2 = 0;
    if (iVar1 != 0) {
      if (DAT_00127330 == 4) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("sort_type != sort_version","src/ls.c",0x1008,"sort_files");
      }
      FUN_00106000();
      iVar2 = 1;
    }
    if ((int)uVar3 == 5) {
      uVar3 = (ulong)(DAT_00127334 + 5);
    }
    FUN_00111610(DAT_001273a0,DAT_001273b0,
                 (&PTR_FUN_00124f60)
                 [(ulong)DAT_001272f4 + ((ulong)DAT_0012732f + ((long)iVar2 + uVar3 * 2) * 2) * 2]);
  }
  return;
}




// Function: length_of_file_name_and_frills @ 0x8af0

long length_of_file_name_and_frills(long param_1)

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
  if (DAT_001272fc == '\0') {
    lVar5 = 0;
LAB_00108bba:
    if (DAT_0012732c != '\0') {
      if (DAT_00127338 != 4) goto LAB_00108b41;
      pcVar4 = "?";
      if (*(char *)(param_1 + 0xb8) != '\0') {
        pcVar4 = (char *)FUN_0010ffa0(*(undefined8 *)(param_1 + 0x58),auStack_2b8,DAT_00127328,0x200
                                      ,DAT_00127320);
      }
      sVar2 = strlen(pcVar4);
LAB_00108b4c:
      lVar5 = lVar5 + sVar2 + 1;
    }
    if (DAT_00127365 == '\0') goto LAB_00108b5c;
    if (DAT_00127338 != 4) goto LAB_00108bf9;
    sVar2 = strlen(*(char **)(param_1 + 0xb0));
  }
  else {
    if (DAT_00127338 == 4) {
      pcVar4 = (char *)FUN_00110c20(*(undefined8 *)(param_1 + 0x20),auStack_2b8);
      sVar2 = strlen(pcVar4);
      lVar5 = sVar2 + 1;
      goto LAB_00108bba;
    }
    lVar5 = (long)DAT_00127360 + 1;
    if (DAT_0012732c != '\0') {
LAB_00108b41:
      sVar2 = (size_t)DAT_0012735c;
      goto LAB_00108b4c;
    }
    if (DAT_00127365 == '\0') goto LAB_00108b5c;
LAB_00108bf9:
    sVar2 = (size_t)DAT_00127354;
  }
  lVar5 = lVar5 + sVar2 + 1;
LAB_00108b5c:
  lVar3 = FUN_00108890(param_1);
  lVar5 = lVar5 + lVar3;
  if (DAT_00127314 != 0) {
    cVar1 = FUN_00106040(*(undefined1 *)(param_1 + 0xb8),*(undefined4 *)(param_1 + 0x30),
                         *(undefined4 *)(param_1 + 0xa8));
    lVar5 = (lVar5 + 1) - (ulong)(cVar1 == '\0');
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar5;
}




// Function: calculate_columns @ 0x8c90

ulong calculate_columns(char param_1)

{
  ulong *puVar1;
  ulong uVar2;
  ulong uVar3;
  long lVar4;
  ulong uVar5;
  ulong uVar6;
  ulong uVar7;
  ulong uVar8;
  char *pcVar9;
  ulong uVar10;
  
  uVar8 = DAT_00127200;
  if (DAT_001273b0 <= DAT_00127200) {
    uVar8 = DAT_001273b0;
  }
  uVar10 = DAT_001273b0;
  if (DAT_00127200 != 0) {
    uVar10 = uVar8;
  }
  uVar8 = 0;
  FUN_001061d0(uVar10);
  if (DAT_001273b0 != 0) {
    do {
      lVar4 = FUN_00108af0(*(undefined8 *)(DAT_001273a0 + uVar8 * 8));
      uVar3 = DAT_001273b0;
      uVar2 = DAT_001272b0;
      if (uVar10 != 0) {
        uVar5 = 0;
        pcVar9 = DAT_00127208;
        do {
          uVar6 = uVar5 + 1;
          if (*pcVar9 != '\0') {
            if (param_1 == '\0') {
              uVar7 = uVar8 % uVar6;
            }
            else {
              uVar7 = uVar8 / (((uVar3 - 1) + uVar6) / uVar6);
            }
            puVar1 = (ulong *)(*(long *)(pcVar9 + 0x10) + uVar7 * 8);
            uVar5 = lVar4 + (ulong)(uVar7 != uVar5) * 2;
            uVar7 = *puVar1;
            if (uVar7 < uVar5) {
              *(ulong *)(pcVar9 + 8) = *(long *)(pcVar9 + 8) + (uVar5 - uVar7);
              *puVar1 = uVar5;
              *pcVar9 = *(ulong *)(pcVar9 + 8) < uVar2;
            }
          }
          pcVar9 = pcVar9 + 0x18;
          uVar5 = uVar6;
        } while (uVar10 != uVar6);
      }
      uVar8 = uVar8 + 1;
    } while (uVar8 < uVar3);
  }
  if (1 < uVar10) {
    pcVar9 = DAT_00127208 + uVar10 * 0x18 + -0x18;
    uVar8 = uVar10;
    do {
      if (*pcVar9 != '\0') {
        return uVar8;
      }
      uVar8 = uVar8 - 1;
      pcVar9 = pcVar9 + -0x18;
      uVar10 = 1;
    } while (uVar8 != 1);
  }
  return uVar10;
}




// Function: quote_name @ 0x8eb0

long quote_name(char *param_1,undefined8 param_2,undefined4 param_3,long param_4,char param_5,
                 long param_6,long param_7)

{
  size_t sVar1;
  void *__ptr;
  char *__ptr_00;
  char *pcVar2;
  size_t sVar3;
  char cVar4;
  char cVar5;
  long in_FS_OFFSET;
  long local_2078;
  byte local_2051;
  char *local_2050;
  char local_2048 [8200];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_2050 = local_2048;
  sVar1 = FUN_00108390(&local_2050,param_1,param_2,param_3,0,&local_2051);
  if ((local_2051 != 0) && (param_5 != '\0')) {
    FUN_00107460(0x20);
  }
  if (param_4 != 0) {
    FUN_00108e50(param_4);
  }
  cVar5 = DAT_001273a9;
  sVar3 = sVar1;
  if (param_7 == 0) {
    local_2078 = 0;
    cVar5 = '\0';
  }
  else {
    cVar4 = '\0';
    if ((DAT_001273a8 == '\0') || (cVar4 = DAT_001273a9, DAT_001273a9 == '\0')) {
      local_2078 = 0;
      cVar5 = cVar4;
    }
    else if (local_2051 == 0) {
      putchar_unlocked((int)*local_2050);
      local_2078 = 1;
      sVar3 = sVar1 - 2;
    }
    else {
      local_2078 = 0;
      cVar5 = '\0';
    }
    __ptr = (void *)FUN_00107830(DAT_00127388,0);
    __ptr_00 = (char *)FUN_00107830(param_7,1);
    pcVar2 = "";
    if (*__ptr_00 != '/') {
      pcVar2 = "/";
    }
    __printf_chk(1,&DAT_0011ab9f,__ptr,pcVar2);
    free(__ptr);
    free(__ptr_00);
  }
  if (param_6 == 0) {
    fwrite_unlocked(local_2050 + local_2078,1,sVar3,stdout);
    DAT_001271f8 = DAT_001271f8 + sVar1;
  }
  else {
    FUN_00107900(param_6);
    fwrite_unlocked(local_2050 + local_2078,1,sVar3,stdout);
    DAT_001271f8 = DAT_001271f8 + sVar1;
    FUN_00107900(param_6);
  }
  if (param_7 != 0) {
    fputs_unlocked("\x1b]8;;\a",stdout);
    if (cVar5 != '\0') {
      putchar_unlocked((int)local_2050[sVar1 - 1]);
    }
  }
  if ((local_2050 != local_2048) && (local_2050 != param_1)) {
    free(local_2050);
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_2051 + sVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_name_with_quoting @ 0x9160

long print_name_with_quoting(undefined8 *param_1,byte param_2,undefined8 param_3,ulong param_4)

{
  ulong uVar1;
  char cVar2;
  long lVar3;
  undefined8 uVar4;
  
  uVar4 = param_1[1];
  if (param_2 == 0) {
    uVar4 = *param_1;
  }
  if ((DAT_00127312 == '\0') ||
     ((lVar3 = FUN_00107480(param_1,param_2), lVar3 == 0 && (cVar2 = FUN_00106420(4), cVar2 == '\0')
      ))) {
    lVar3 = FUN_00108eb0(uVar4,DAT_001272d0,*(undefined4 *)((long)param_1 + 0xc4),0,param_2 ^ 1,
                         param_3,param_1[2]);
    FUN_00107ce0();
  }
  else {
    lVar3 = FUN_00108eb0(uVar4,DAT_001272d0,*(undefined4 *)((long)param_1 + 0xc4),lVar3,param_2 ^ 1,
                         param_3,param_1[2]);
    FUN_00107ce0();
    FUN_00107c10();
    if (DAT_001272b0 != 0) {
      uVar1 = lVar3 + -1 + param_4;
      if (param_4 / DAT_001272b0 != uVar1 / DAT_001272b0) {
        FUN_00107bb0(&DAT_001261d0,param_4 / DAT_001272b0,uVar1 % DAT_001272b0);
      }
    }
  }
  return lVar3;
}




// Function: print_long_format @ 0x92b0

void print_long_format(long param_1)

{
  char cVar1;
  undefined1 auVar2 [16];
  int iVar3;
  int iVar4;
  undefined4 uVar5;
  uint uVar6;
  undefined1 *puVar7;
  long lVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 extraout_RDX;
  undefined8 extraout_RDX_00;
  undefined8 extraout_RDX_01;
  undefined8 extraout_RDX_02;
  undefined8 extraout_RDX_03;
  undefined8 extraout_RDX_04;
  char *pcVar11;
  char *pcVar12;
  char *pcVar13;
  long lVar14;
  int iVar15;
  char *pcVar16;
  char *pcVar17;
  long in_FS_OFFSET;
  bool bVar18;
  undefined1 local_1198 [16];
  undefined1 local_1188 [68];
  char local_1144;
  undefined8 local_1143;
  undefined2 local_113b;
  undefined1 local_1139;
  undefined1 local_1138 [32];
  undefined1 local_1118 [656];
  char local_e88 [3656];
  long local_40;
  
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  if (*(char *)(param_1 + 0xb8) == '\0') {
    local_1139 = 0;
    local_113b = 0x3f3f;
    local_1144 = "?pcdb-lswd"[*(uint *)(param_1 + 0xa8)];
    local_1143 = 0x3f3f3f3f3f3f3f3f;
  }
  else {
    FUN_0010e2f0(param_1 + 0x18,&local_1144);
  }
  if (DAT_00127364 == '\0') {
    local_113b = local_113b & 0xff;
LAB_00109310:
    if (DAT_00127334 != 2) goto LAB_0010931f;
LAB_00109600:
    bVar18 = true;
    local_1198 = FUN_00115c90(param_1 + 0x18);
  }
  else {
    if (*(int *)(param_1 + 0xbc) == 1) {
      local_113b = CONCAT11(0x2e,(undefined1)local_113b);
      goto LAB_00109310;
    }
    if (*(int *)(param_1 + 0xbc) != 2) goto LAB_00109310;
    local_113b = CONCAT11(0x2b,(undefined1)local_113b);
    if (DAT_00127334 == 2) goto LAB_00109600;
LAB_0010931f:
    if (DAT_00127334 < 3) {
      if (DAT_00127334 == 0) {
        bVar18 = true;
        local_1198 = FUN_00115cb0(param_1 + 0x18);
      }
      else {
        bVar18 = true;
        local_1198 = FUN_00115ca0(param_1 + 0x18);
      }
    }
    else {
      if (DAT_00127334 != 3) {
                    /* WARNING: Subroutine does not return */
        abort();
      }
      local_1198 = thunk_FUN_00115cb0(param_1 + 0x18);
      bVar18 = (local_1198._0_8_ & local_1198._8_8_) != 0xffffffffffffffff;
    }
  }
  pcVar12 = local_e88;
  if (DAT_001272fc != '\0') {
    uVar9 = FUN_00106f10(local_1118,0x15,param_1);
    iVar4 = __sprintf_chk(local_e88,1,0xe3b,&DAT_0011abc7,DAT_00127360,uVar9);
    pcVar12 = local_e88 + iVar4;
  }
  if (DAT_0012732c != '\0') {
    pcVar17 = "?";
    if (*(char *)(param_1 + 0xb8) != '\0') {
      pcVar17 = (char *)FUN_0010ffa0(*(undefined8 *)(param_1 + 0x58),local_1118,DAT_00127328,0x200,
                                     DAT_00127320);
    }
    iVar4 = DAT_0012735c;
    iVar3 = FUN_001113a0(pcVar17,0);
    pcVar16 = pcVar12;
    if (0 < iVar4 - iVar3) {
      pcVar13 = pcVar12;
      do {
        pcVar11 = pcVar13 + 1;
        *pcVar13 = ' ';
        pcVar16 = pcVar12 + (iVar4 - iVar3);
        pcVar13 = pcVar11;
      } while (pcVar12 + (iVar4 - iVar3) != pcVar11);
    }
    do {
      pcVar13 = pcVar16;
      pcVar12 = pcVar13 + 1;
      cVar1 = *pcVar17;
      *pcVar13 = cVar1;
      pcVar16 = pcVar12;
      pcVar17 = pcVar17 + 1;
    } while (cVar1 != '\0');
    *pcVar13 = ' ';
  }
  puVar7 = &DAT_0011aaab;
  if (*(char *)(param_1 + 0xb8) != '\0') {
    puVar7 = (undefined1 *)FUN_00110c20(*(undefined8 *)(param_1 + 0x28),local_1118);
  }
  iVar4 = __sprintf_chk(pcVar12,1,0xffffffffffffffff,"%s %*s ",&local_1144,DAT_00127358,puVar7);
  pcVar12 = pcVar12 + iVar4;
  FUN_00107810();
  if ((((DAT_00126029 != '\0') || (DAT_00126028 != '\0')) || (DAT_0012732e != '\0')) ||
     (uVar9 = extraout_RDX, DAT_00127365 != '\0')) {
    FUN_001077d0(local_e88,(long)pcVar12 - (long)local_e88);
    uVar9 = extraout_RDX_00;
    if (DAT_00126029 != '\0') {
      FUN_001081d0(*(undefined4 *)(param_1 + 0x34),DAT_00127350,*(undefined1 *)(param_1 + 0xb8));
      uVar9 = extraout_RDX_02;
    }
    if (DAT_00126028 != '\0') {
      FUN_00108180(*(undefined4 *)(param_1 + 0x38),DAT_0012734c,*(undefined1 *)(param_1 + 0xb8));
      uVar9 = extraout_RDX_03;
    }
    if (DAT_0012732e != '\0') {
      FUN_001081d0(*(undefined4 *)(param_1 + 0x34),DAT_00127348,*(undefined1 *)(param_1 + 0xb8));
      uVar9 = extraout_RDX_04;
    }
    pcVar12 = local_e88;
    if (DAT_00127365 != '\0') {
      FUN_00108100(*(undefined8 *)(param_1 + 0xb0),0,DAT_00127354);
      uVar9 = extraout_RDX_01;
    }
  }
  if (*(char *)(param_1 + 0xb8) == '\0') {
    pcVar17 = "?";
LAB_00109493:
    iVar4 = DAT_0012733c;
    iVar3 = FUN_001113a0(pcVar17,0);
    pcVar16 = pcVar12;
    if (0 < iVar4 - iVar3) {
      pcVar13 = pcVar12;
      do {
        pcVar11 = pcVar13 + 1;
        *pcVar13 = ' ';
        pcVar13 = pcVar11;
        pcVar16 = pcVar12 + (iVar4 - iVar3);
      } while (pcVar12 + (iVar4 - iVar3) != pcVar11);
    }
    do {
      pcVar12 = pcVar16;
      cVar1 = *pcVar17;
      pcVar17 = pcVar17 + 1;
      pcVar16 = pcVar12 + 1;
      *pcVar12 = cVar1;
    } while (cVar1 != '\0');
    *pcVar12 = ' ';
  }
  else {
    if ((*(uint *)(param_1 + 0x30) & 0xb000) != 0x2000) {
      uVar9 = FUN_001059e0(*(undefined8 *)(param_1 + 0x48),local_1118,uVar9,1);
      pcVar17 = (char *)FUN_0010ffa0(uVar9);
      goto LAB_00109493;
    }
    iVar15 = DAT_0012733c - (DAT_00127344 + 2 + DAT_00127340);
    uVar6 = gnu_dev_minor(*(ulonglong *)(param_1 + 0x40));
    uVar9 = FUN_00110c20(uVar6,local_1118);
    iVar4 = DAT_00127340;
    uVar6 = gnu_dev_major(*(ulonglong *)(param_1 + 0x40));
    uVar10 = FUN_00110c20(uVar6,local_1138);
    iVar3 = 0;
    if (-1 < iVar15) {
      iVar3 = iVar15;
    }
    iVar4 = __sprintf_chk(pcVar12,1,0xffffffffffffffff,&DAT_0011abc2,iVar3 + DAT_00127344,uVar10,
                          iVar4,uVar9);
    pcVar16 = pcVar12 + iVar4;
  }
  *pcVar16 = '\x01';
  if ((*(char *)(param_1 + 0xb8) == '\0') || (!bVar18)) {
LAB_001094f4:
    puVar7 = &DAT_0011aaab;
LAB_001094fb:
    uVar5 = FUN_001082b0();
    iVar4 = __sprintf_chk(pcVar16,1,0xffffffffffffffff,&DAT_0011abc7,uVar5,puVar7);
    pcVar16 = pcVar16 + iVar4;
  }
  else {
    lVar8 = FUN_001160a0(DAT_001272a8,local_1198,local_1188);
    auVar2 = local_1198;
    if (lVar8 == 0) {
LAB_00109a20:
      if (*pcVar16 != '\0') {
        if (*(char *)(param_1 + 0xb8) == '\0') goto LAB_001094f4;
        puVar7 = (undefined1 *)thunk_FUN_00110b80(local_1198._0_8_,local_1118);
        goto LAB_001094fb;
      }
    }
    else {
      iVar4 = FUN_001162a0(DAT_00127370,DAT_00127378,local_1198._0_8_,local_1198._8_8_);
      if (iVar4 < 0) {
        FUN_0010e960(&DAT_00127370);
        auVar2 = local_1198;
      }
      iVar4 = FUN_001162a0(DAT_00127370 + -0xf0c2ac,DAT_00127378,auVar2._0_8_,auVar2._8_8_);
      uVar6 = 0;
      if (iVar4 < 0) {
        uVar6 = FUN_001162a0(local_1198._0_8_,local_1198._8_8_,DAT_00127370,DAT_00127378);
        uVar6 = uVar6 >> 0x1f;
      }
      lVar8 = FUN_00108240(pcVar16,uVar6,local_1188,DAT_001272a8,local_1198._8_8_ & 0xffffffff);
      if (lVar8 == 0) goto LAB_00109a20;
      pcVar16 = pcVar16 + lVar8;
    }
    *pcVar16 = ' ';
    pcVar16 = pcVar16 + 1;
  }
  lVar14 = (long)pcVar16 - (long)local_e88;
  FUN_001077d0(local_e88,lVar14);
  lVar8 = FUN_00109160(param_1,0,&DAT_001271a0,lVar14);
  if (*(int *)(param_1 + 0xa8) == 6) {
    if (*(long *)(param_1 + 8) != 0) {
      FUN_001077f0(&DAT_0011abcc);
      FUN_00109160(param_1,1,0,lVar14 + 4 + lVar8);
      if (DAT_00127314 != 0) {
        FUN_00109280(1,*(undefined4 *)(param_1 + 0xac),0);
      }
    }
  }
  else if (DAT_00127314 != 0) {
    FUN_00109280(*(undefined1 *)(param_1 + 0xb8),*(undefined4 *)(param_1 + 0x30));
  }
  if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_with_separator @ 0x9c00

void print_with_separator(char param_1)

{
  ulong uVar1;
  undefined8 uVar2;
  ulong uVar3;
  ulong uVar4;
  ulong uVar5;
  int __c;
  
  if (DAT_001273b0 == 0) {
    putchar_unlocked((int)DAT_00126019);
    return;
  }
  uVar4 = 0;
  uVar5 = 0;
  do {
    uVar2 = *(undefined8 *)(DAT_001273a0 + uVar4 * 8);
    if (DAT_001272b0 == 0) {
      uVar3 = uVar5;
      if (uVar4 != 0) {
        uVar1 = uVar5 + 2;
LAB_00109cb0:
        uVar3 = uVar1;
        uVar5 = uVar5 + 2;
        __c = 0x20;
        goto LAB_00109cb6;
      }
    }
    else {
      uVar3 = FUN_00108af0(uVar2);
      if (uVar4 == 0) {
        uVar3 = uVar5 + uVar3;
      }
      else {
        uVar1 = uVar3 + uVar5 + 2;
        if ((DAT_001272b0 == 0) || ((uVar1 < DAT_001272b0 && (uVar5 <= -uVar3 - 3))))
        goto LAB_00109cb0;
        __c = (int)DAT_00126019;
        uVar5 = 0;
LAB_00109cb6:
        putchar_unlocked((int)param_1);
        putchar_unlocked(__c);
      }
    }
    uVar4 = uVar4 + 1;
    FUN_00109a80(uVar2,uVar5);
    uVar5 = uVar3;
    if (DAT_001273b0 <= uVar4) {
      putchar_unlocked((int)DAT_00126019);
      return;
    }
  } while( true );
}




// Function: print_horizontal @ 0x9d30

void print_horizontal(void)

{
  long lVar1;
  undefined8 uVar2;
  ulong uVar3;
  long lVar4;
  ulong uVar5;
  ulong uVar6;
  long lVar7;
  long lVar8;
  long lVar9;
  
  uVar3 = FUN_00108c90(0);
  lVar1 = DAT_00127208 + -0x18 + uVar3 * 0x18;
  uVar2 = *DAT_001273a0;
  lVar4 = FUN_00108af0(uVar2);
  lVar7 = **(long **)(lVar1 + 0x10);
  FUN_00109a80(uVar2,0);
  if (1 < DAT_001273b0) {
    uVar6 = 1;
    lVar9 = 0;
    do {
      uVar5 = uVar6 % uVar3;
      if (uVar5 == 0) {
        lVar8 = 0;
        putchar_unlocked((int)DAT_00126019);
      }
      else {
        lVar8 = lVar7 + lVar9;
        FUN_001073d0(lVar9 + lVar4,lVar8);
      }
      uVar2 = DAT_001273a0[uVar6];
      uVar6 = uVar6 + 1;
      FUN_00109a80(uVar2,lVar8);
      lVar4 = FUN_00108af0(uVar2);
      lVar7 = *(long *)(*(long *)(lVar1 + 0x10) + uVar5 * 8);
      lVar9 = lVar8;
    } while (uVar6 < DAT_001273b0);
  }
  putchar_unlocked((int)DAT_00126019);
  return;
}




// Function: print_many_per_line @ 0x9e30

void print_many_per_line(void)

{
  long lVar1;
  undefined8 uVar2;
  ulong uVar3;
  long lVar4;
  long lVar5;
  ulong uVar6;
  long lVar7;
  long lVar8;
  undefined8 local_40;
  
  uVar3 = FUN_00108c90(1);
  local_40 = 0;
  lVar1 = DAT_00127208 + uVar3 * 0x18;
  uVar3 = (DAT_001273b0 / uVar3 + 1) - (ulong)(DAT_001273b0 % uVar3 == 0);
  if (uVar3 != 0) {
    do {
      lVar7 = 0;
      uVar6 = local_40;
      lVar8 = 0;
      while( true ) {
        uVar2 = *(undefined8 *)(DAT_001273a0 + uVar6 * 8);
        lVar4 = FUN_00108af0();
        lVar5 = *(long *)(*(long *)(lVar1 + -8) + lVar7);
        lVar7 = lVar7 + 8;
        FUN_00109a80(uVar2,lVar8);
        uVar6 = uVar6 + uVar3;
        if (DAT_001273b0 <= uVar6) break;
        lVar5 = lVar5 + lVar8;
        FUN_001073d0(lVar8 + lVar4,lVar5);
        lVar8 = lVar5;
      }
      putchar_unlocked((int)DAT_00126019);
      local_40 = local_40 + 1;
    } while (uVar3 != local_40);
  }
  return;
}




// Function: print_current_files @ 0x9f20

void print_current_files(void)

{
  long lVar1;
  ulong uVar2;
  
  switch(DAT_00127338) {
  case 0:
    uVar2 = 0;
    if (DAT_001273b0 != 0) {
      do {
        FUN_00107c60();
        lVar1 = uVar2 * 8;
        uVar2 = uVar2 + 1;
        FUN_001092b0(*(undefined8 *)(DAT_001273a0 + lVar1));
        FUN_00107460((int)DAT_00126019);
      } while (uVar2 < DAT_001273b0);
      return;
    }
    break;
  case 1:
    uVar2 = 0;
    if (DAT_001273b0 != 0) {
      do {
        lVar1 = uVar2 * 8;
        uVar2 = uVar2 + 1;
        FUN_00109a80(*(undefined8 *)(DAT_001273a0 + lVar1),0);
        putchar_unlocked((int)DAT_00126019);
      } while (uVar2 < DAT_001273b0);
    }
    break;
  case 2:
    if (DAT_001272b0 != 0) {
      FUN_00109e30();
      return;
    }
    goto LAB_0010a000;
  case 3:
    if (DAT_001272b0 != 0) {
      FUN_00109d30();
      return;
    }
LAB_0010a000:
    FUN_00109c00(0x20);
    return;
  case 4:
    FUN_00109c00(0x2c);
    return;
  default:
    return;
  }
  return;
}




// Function: xstrcoll_version @ 0xa050

void xstrcoll_version(undefined8 *param_1,undefined8 *param_2)

{
  FUN_0010a010(*param_1,*param_2);
  return;
}




// Function: rev_xstrcoll_version @ 0xa060

void rev_xstrcoll_version(undefined8 *param_1,undefined8 *param_2)

{
  FUN_0010a010(*param_2,*param_1);
  return;
}




// Function: rev_strcmp_size @ 0xa130

void rev_strcmp_size(undefined8 *param_1,undefined8 *param_2)

{
  FUN_0010a100(*param_2,param_2[9],*param_1,param_1[9],PTR_strcmp_00125fc0);
  return;
}




// Function: rev_xstrcoll_size @ 0xa150

void rev_xstrcoll_size(undefined8 *param_1,undefined8 *param_2)

{
  FUN_0010a100(*param_2,param_2[9],*param_1,param_1[9],FUN_001072a0);
  return;
}




// Function: strcmp_size @ 0xa170

void strcmp_size(undefined8 *param_1,undefined8 *param_2)

{
  FUN_0010a100(*param_1,param_1[9],*param_2,param_2[9],PTR_strcmp_00125fc0);
  return;
}




// Function: xstrcoll_size @ 0xa1a0

void xstrcoll_size(undefined8 *param_1,undefined8 *param_2)

{
  FUN_0010a100(*param_1,param_1[9],*param_2,param_2[9],FUN_001072a0);
  return;
}




// Function: extract_dirs_from_files @ 0xa1e0

void extract_dirs_from_files(long param_1,undefined1 param_2)

{
  long *plVar1;
  int iVar2;
  undefined8 *puVar3;
  char *pcVar4;
  long *plVar5;
  char cVar6;
  void *__ptr;
  long *plVar7;
  long lVar8;
  bool bVar9;
  
  if ((param_1 != 0) && (DAT_001273c8 != 0)) {
    FUN_00106840(0,param_1,0);
  }
  lVar8 = DAT_001273b0 + -1;
  if (DAT_001273b0 != 0) {
LAB_0010a26e:
    do {
      puVar3 = (undefined8 *)DAT_001273a0[lVar8];
      cVar6 = FUN_0010a1d0(*(undefined4 *)(puVar3 + 0x15));
      if (cVar6 != '\0') {
        pcVar4 = (char *)*puVar3;
        if (param_1 == 0) {
LAB_0010a291:
          FUN_00106840(pcVar4,puVar3[1],param_2);
          iVar2 = *(int *)(puVar3 + 0x15);
        }
        else {
          cVar6 = FUN_001073b0(pcVar4);
          if (cVar6 != '\0') goto LAB_0010a268;
          if (*pcVar4 == '/') goto LAB_0010a291;
          __ptr = (void *)FUN_0010e300(param_1,pcVar4,0);
          FUN_00106840(__ptr,puVar3[1],param_2);
          free(__ptr);
          iVar2 = *(int *)(puVar3 + 0x15);
        }
        if (iVar2 == 9) {
          FUN_00106dc0(puVar3);
          bVar9 = lVar8 == 0;
          lVar8 = lVar8 + -1;
          if (bVar9) break;
          goto LAB_0010a26e;
        }
      }
LAB_0010a268:
      bVar9 = lVar8 != 0;
      lVar8 = lVar8 + -1;
    } while (bVar9);
    plVar5 = DAT_001273a0;
    if (DAT_001273b0 != 0) {
      plVar1 = DAT_001273a0 + DAT_001273b0;
      DAT_001273b0 = 0;
      plVar7 = DAT_001273a0;
      do {
        iVar2 = *(int *)(*plVar7 + 0xa8);
        plVar5[DAT_001273b0] = *plVar7;
        plVar7 = plVar7 + 1;
        DAT_001273b0 = DAT_001273b0 + (ulong)(iVar2 != 9);
      } while (plVar1 != plVar7);
      return;
    }
  }
  DAT_001273b0 = 0;
  return;
}




// Function: do_statx @ 0xa400

int do_statx(undefined8 param_1,undefined8 param_2,long param_3,uint param_4,uint param_5)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined1 auVar2 [16];
  undefined1 uStack_138;
  byte local_137;
  undefined8 local_e8;
  undefined4 local_e0;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = statx(param_1,param_2,param_4 | 0x800,param_5,&uStack_138);
  if (-1 < iVar1) {
    FUN_0010a340(&uStack_138,param_3);
    if ((param_5 & 0x800) != 0) {
      if ((local_137 & 8) == 0) {
        *(undefined8 *)(param_3 + 0x60) = 0xffffffffffffffff;
        *(undefined8 *)(param_3 + 0x58) = 0xffffffffffffffff;
      }
      else {
        auVar2 = FUN_0010a330(local_e8,local_e0);
        *(undefined1 (*) [16])(param_3 + 0x58) = auVar2;
      }
    }
  }
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return iVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: do_stat @ 0xa4c0

void do_stat(undefined8 param_1,undefined8 param_2)

{
  undefined4 uVar1;
  
  uVar1 = FUN_001068d0();
  FUN_0010a400(0xffffff9c,param_1,param_2,0,uVar1);
  return;
}




// Function: do_lstat @ 0xa4f0

void do_lstat(undefined8 param_1,undefined8 param_2)

{
  undefined4 uVar1;
  
  uVar1 = FUN_001068d0();
  FUN_0010a400(0xffffff9c,param_1,param_2,0x100,uVar1);
  return;
}




// Function: stat_for_mode @ 0xa530

void stat_for_mode(undefined8 param_1,undefined8 param_2)

{
  FUN_0010a400(0xffffff9c,param_1,param_2,0,2);
  return;
}




// Function: fstat_for_ino @ 0xb140

void fstat_for_ino(undefined8 param_1,undefined8 param_2)

{
  FUN_0010a400(param_1,&DAT_0011abb9,param_2,0x1000,0x100);
  return;
}




// Function: stat_for_ino @ 0xb160

void stat_for_ino(undefined8 param_1,undefined8 param_2)

{
  FUN_0010a400(0xffffff9c,param_1,param_2,0,0x100);
  return;
}




// Function: print_dir @ 0xb180

void print_dir(char *param_1,char *param_2,undefined1 param_3)

{
  char cVar1;
  byte bVar2;
  int iVar3;
  int *piVar4;
  DIR *__dirp;
  long lVar5;
  dirent *pdVar6;
  undefined8 uVar7;
  char *__s;
  size_t sVar8;
  undefined8 uVar9;
  void *__ptr;
  undefined4 uVar10;
  long in_FS_OFFSET;
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
      FUN_00106f50(param_3,uVar7,param_1);
      return;
    }
  }
  else {
    if (DAT_001273c8 == 0) {
LAB_0010b22c:
      FUN_00106e20();
      if ((DAT_001272f6 != '\0') || (DAT_001272b8 != '\0')) {
        if (DAT_00126010 == '\0') {
          FUN_00107460(10);
        }
        DAT_00126010 = '\0';
        __ptr = (void *)0x0;
        FUN_00107810();
        if ((DAT_00127311 != '\0') &&
           (__ptr = (void *)FUN_0010de90(param_1,2), __ptr == (void *)0x0)) {
          uVar7 = dcgettext(0,"error canonicalizing %s",5);
          FUN_00106f50(param_3,uVar7,param_1);
        }
        if (param_2 == (char *)0x0) {
          param_2 = param_1;
        }
        FUN_00108eb0(param_2,DAT_001272c8,0xffffffff,0,1,&DAT_00127140,__ptr);
        free(__ptr);
        FUN_001077f0(":\n");
      }
      local_378 = 0;
      do {
        *piVar4 = 0;
        pdVar6 = readdir(__dirp);
        if (pdVar6 == (dirent *)0x0) {
          if (*piVar4 == 0) goto LAB_0010b357;
          uVar7 = dcgettext(0,"reading directory %s",5);
          FUN_00106f50(param_3,uVar7,param_1);
          if (*piVar4 != 0x4b) goto LAB_0010b357;
        }
        else {
          cVar1 = FUN_001069f0(pdVar6->d_name);
          if (cVar1 == '\0') {
            uVar10 = 0;
            bVar2 = pdVar6->d_type - 1;
            if (bVar2 < 0xe) {
              uVar10 = *(undefined4 *)(&DAT_001195e0 + (ulong)bVar2 * 4);
            }
            lVar5 = FUN_0010a550(pdVar6->d_name,uVar10,0,param_1);
            local_378 = local_378 + lVar5;
            if ((((DAT_00127338 == 1) && (DAT_00127330 == 6)) && (DAT_0012732c == '\0')) &&
               (DAT_001272f6 == '\0')) {
              FUN_001089f0();
              FUN_00109f20();
              FUN_00106e20();
            }
          }
        }
        FUN_00107ce0();
      } while( true );
    }
    iVar3 = dirfd(__dirp);
    if (iVar3 < 0) {
      iVar3 = FUN_0010b160(param_1,&local_368);
      uVar7 = local_368;
      uVar9 = local_360;
    }
    else {
      iVar3 = FUN_0010b140(iVar3);
      uVar7 = local_368;
      uVar9 = local_360;
    }
    if (iVar3 < 0) {
      uVar7 = dcgettext(0,"cannot determine device and inode of %s",5);
      FUN_00106f50(param_3,uVar7,param_1);
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        closedir(__dirp);
        return;
      }
    }
    else {
      cVar1 = FUN_00107df0(uVar7);
      if (cVar1 == '\0') {
        FUN_00107970(uVar7,uVar9);
        goto LAB_0010b22c;
      }
      uVar7 = FUN_00115700(0,3,param_1);
      uVar9 = dcgettext(0,"%s: not listing already-listed directory",5);
      error(0,0,uVar9,uVar7);
      closedir(__dirp);
      if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
        FUN_001059b0(1);
        return;
      }
    }
  }
  goto LAB_0010b68a;
LAB_0010b357:
  iVar3 = closedir(__dirp);
  if (iVar3 != 0) {
    uVar7 = dcgettext(0,"closing directory %s",5);
    FUN_00106f50(param_3,uVar7,param_1);
  }
  FUN_001089f0();
  if (DAT_001272f6 != '\0') {
    FUN_0010a1e0(param_1,0);
  }
  if ((DAT_00127338 == 0) || (DAT_0012732c != '\0')) {
    __s = (char *)FUN_0010ffa0(local_378,local_2d7,DAT_00127328,0x200,DAT_00127320);
    sVar8 = strlen(__s);
    __s[-1] = ' ';
    __s[sVar8] = DAT_00126019;
    FUN_00107810();
    uVar7 = dcgettext(0,"total",5);
    FUN_001077f0(uVar7);
    FUN_001077d0(__s + -1,__s + sVar8 + (1 - (long)(__s + -1)));
  }
  if (DAT_001273b0 == 0) {
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
  }
  else if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
    FUN_00109f20();
    return;
  }
LAB_0010b68a:
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: usage @ 0xb690

void usage(int param_1)

{
  FILE *pFVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  char *pcVar4;
  
  uVar2 = DAT_00127400;
  if (param_1 == 0) {
    uVar3 = dcgettext(0,"Usage: %s [OPTION]... [FILE]...\n",5);
    __printf_chk(1,uVar3,uVar2);
    pFVar1 = stdout;
    pcVar4 = (char *)dcgettext(0,
                               "List information about the FILEs (the current directory by default).\nSort entries alphabetically if none of -cftuvSUX nor --sort is specified.\n"
                               ,5);
    fputs_unlocked(pcVar4,pFVar1);
    FUN_00106530();
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
    FUN_00106560();
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
    fputs_unlocked(pcVar4,pFVar1);
    pcVar4 = "ls";
    if ((DAT_001261e0 != 1) && (pcVar4 = "dir", DAT_001261e0 != 2)) {
      pcVar4 = "vdir";
    }
    FUN_00107e40(pcVar4);
  }
  else {
    uVar3 = dcgettext(0,"Try \'%s --help\' for more information.\n",5);
    __fprintf_chk(stderr,1,uVar3,uVar2);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: decode_switches @ 0xbda0

undefined4 decode_switches(uint param_1,char *param_2)

{
  undefined1 auVar1 [16];
  bool bVar2;
  FILE *pFVar3;
  char cVar4;
  byte bVar5;
  int iVar6;
  uint uVar7;
  long *plVar8;
  long lVar9;
  char *pcVar10;
  char *pcVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  uint *puVar14;
  undefined **ppuVar15;
  char *pcVar16;
  ulong uVar17;
  long in_FS_OFFSET;
  uint local_74;
  int local_70;
  int local_6c;
  ulong local_68;
  char *local_60;
  long local_58;
  int local_50;
  undefined2 local_48;
  ushort uStack_46;
  undefined4 uStack_44;
  long local_40;
  
  puVar14 = &switchD_0010be5c::switchdataD_00119174;
  local_40 = *(long *)(in_FS_OFFSET + 0x28);
  local_6c = -1;
  local_68 = 0xffffffffffffffff;
  local_58 = -1;
  local_70 = -1;
  local_50 = -1;
  local_74 = 0xffffffff;
  bVar2 = false;
  local_60 = (char *)0x0;
LAB_0010be20:
  local_48 = 0xffff;
  uStack_46 = 0xffff;
  uVar17 = (ulong)param_1;
  iVar6 = getopt_long((ulong)param_1,param_2,"abcdfghiklmnopqrstuvw:xABCDFGHI:LNQRST:UXZ1",
                      &PTR_s_all_0011af0e_7_001252c0,&local_48);
  if (iVar6 != -1) {
    if (0x114 < iVar6 + 0x83U) goto switchD_0010be5c_caseD_10cc62;
    switch((long)(int)puVar14[iVar6 + 0x83U] + (long)puVar14) {
    case 0x10be5f:
      DAT_00126019 = '\0';
      DAT_00127312 = 0;
      local_70 = 0;
      local_74 = (uint)(local_74 != 0);
switchD_0010be5c_caseD_10be86:
      local_50 = 0;
      break;
    case 0x10be86:
      goto switchD_0010be5c_caseD_10be86;
    case 0x10be90:
      lVar9 = FUN_0010d270("--quoting-style",optarg,&PTR_s_literal_001259e0,&DAT_0011d9c0,4,
                           PTR_FUN_001261f0,1);
      local_70 = *(int *)(&DAT_0011d9c0 + lVar9 * 4);
      break;
    case 0x10bedb:
      lVar9 = FUN_0010d270("--indicator-style",optarg,&PTR_DAT_00125940,&DAT_00119750,4,
                           PTR_FUN_001261f0,1);
      DAT_00127314 = *(uint *)(&DAT_00119750 + lVar9 * 4);
      break;
    case 0x10bf27:
      if (optarg == (char *)0x0) {
LAB_0010bf7f:
        bVar5 = 1;
      }
      else {
        lVar9 = FUN_0010d270("--hyperlink",optarg,&PTR_s_always_0011d645_0xd_001251a0,&DAT_001196c0,
                             4,PTR_FUN_001261f0,1);
        if (*(int *)(&DAT_001196c0 + lVar9 * 4) == 1) goto LAB_0010bf7f;
        bVar5 = 0;
        if (*(int *)(&DAT_001196c0 + lVar9 * 4) == 2) {
          bVar5 = FUN_00106590();
        }
      }
      DAT_00127311 = bVar5 & 1;
      break;
    case 0x10bf96:
      plVar8 = (long *)FUN_001169e0(0x10);
      *plVar8 = (long)optarg;
      plVar8[1] = (long)DAT_001272e0;
      DAT_001272e0 = plVar8;
      break;
    case 0x10bfc1:
      DAT_001272f4 = 1;
      break;
    case 0x10bfcd:
      local_60 = (char *)FUN_001055b0("full-iso");
      local_74 = 0;
      break;
    case 0x10bfeb:
      lVar9 = FUN_0010d270("--format",optarg,&PTR_s_verbose_00125280,&DAT_00119730,4,
                           PTR_FUN_001261f0);
      local_74 = *(uint *)(&DAT_00119730 + lVar9 * 4);
      break;
    case 0x10c036:
      DAT_00127314 = 2;
      break;
    case 0x10c045:
      DAT_001272f8 = 3;
      break;
    case 0x10c054:
      if (optarg == (char *)0x0) {
LAB_0010c0ae:
        bVar5 = 1;
      }
      else {
        lVar9 = FUN_0010d270("--color",optarg,&PTR_s_always_0011d645_0xd_001251a0,&DAT_001196c0,4,
                             PTR_FUN_001261f0);
        if (*(int *)(&DAT_001196c0 + lVar9 * 4) == 1) goto LAB_0010c0ae;
        bVar5 = 0;
        if (*(int *)(&DAT_001196c0 + lVar9 * 4) == 2) {
          bVar5 = FUN_00106590();
        }
      }
      DAT_00127312 = bVar5 & 1;
      break;
    case 0x10c0c5:
      iVar6 = FUN_00110810(optarg,&DAT_00127328,&DAT_00127320);
      if (iVar6 != 0) {
        FUN_00117110(iVar6,CONCAT22(uStack_46,local_48),0,&PTR_s_all_0011af0e_7_001252c0,optarg);
        goto LAB_0010cd66;
      }
      DAT_0012731c = DAT_00127328;
      DAT_00126020 = DAT_00127320;
      break;
    case 0x10c106:
      DAT_0012732e = 1;
      break;
    case 0x10c112:
      local_74 = 3;
      break;
    case 0x10c11f:
      local_68 = FUN_001063b0(optarg);
      if (-1 < (long)local_68) break;
      puVar14 = (uint *)FUN_001158c0(optarg);
      uVar12 = dcgettext(0,"invalid line width",5);
      error(2,0,"%s: %s",uVar12,puVar14);
    case 0x10c180:
      local_6c = 4;
      break;
    case 0x10c18d:
      DAT_00127334 = 2;
      break;
    case 0x10c19c:
      local_6c = 5;
      break;
    case 0x10c1a9:
      DAT_0012732c = 1;
      break;
    case 0x10c1b5:
      DAT_0012732f = 1;
      break;
    case 0x10c1c1:
      local_50 = 1;
      break;
    case 0x10c1ce:
      DAT_00127314 = 1;
      break;
    case 0x10c1dd:
      DAT_00126028 = 0;
      local_74 = 0;
      break;
    case 0x10c1f1:
      DAT_0012732d = 1;
      local_74 = 0;
      break;
    case 0x10c205:
      local_74 = 4;
      break;
    case 0x10c212:
      DAT_001272fc = 1;
      break;
    case 0x10c21e:
      DAT_00127328 = 0xb0;
      DAT_0012731c = 0xb0;
      DAT_00127320 = 1;
      DAT_00126020 = 1;
      break;
    case 0x10c24d:
      DAT_00126029 = 0;
      local_74 = 0;
      break;
    case 0x10c261:
      DAT_001272f0 = 2;
      DAT_00127312 = 0;
      DAT_00127311 = 0;
      uVar7 = 0xffffffff;
      if (local_74 != 0) {
        uVar7 = local_74;
      }
      DAT_0012732c = 0;
      local_6c = 6;
      local_74 = uVar7;
      break;
    case 0x10c29f:
      DAT_001272f5 = 1;
      break;
    case 0x10c2ab:
      DAT_00127334 = 1;
      break;
    case 0x10c2ba:
      local_70 = 7;
      break;
    case 0x10c2c7:
      DAT_001272f0 = 2;
      break;
    case 0x10c2d6:
      DAT_00127365 = 1;
      break;
    case 0x10c2e2:
      local_6c = 1;
      break;
    case 0x10c2ef:
      local_6c = 6;
      break;
    case 0x10c2fc:
      uVar12 = dcgettext(0,"invalid tab size",5);
      local_58 = FUN_00116de0(optarg,0,0,0x7fffffffffffffff,&DAT_0011abb9,uVar12,2);
      break;
    case 0x10c347:
      local_6c = 3;
      break;
    case 0x10c354:
      DAT_001272f6 = 1;
      break;
    case 0x10c360:
      local_70 = 5;
      break;
    case 0x10c36d:
      local_70 = 0;
      break;
    case 0x10c37a:
      DAT_001272f8 = 4;
      break;
    case 0x10c389:
      FUN_00106380(optarg);
      break;
    case 0x10c39a:
      DAT_001272f8 = 2;
      break;
    case 0x10c3a9:
      DAT_00126028 = 0;
      break;
    case 0x10c3b5:
      if (optarg != (char *)0x0) {
        lVar9 = FUN_0010d270("--classify",optarg,&PTR_s_always_0011d645_0xd_001251a0,&DAT_001196c0,4
                             ,PTR_FUN_001261f0,1);
        if ((*(int *)(&DAT_001196c0 + lVar9 * 4) != 1) &&
           ((*(int *)(&DAT_001196c0 + lVar9 * 4) != 2 || (cVar4 = FUN_00106590(), cVar4 == '\0'))))
        break;
      }
      DAT_00127314 = 3;
      break;
    case 0x10c423:
      DAT_00127318 = 1;
      break;
    case 0x10c42f:
      local_74 = 2;
      break;
    case 0x10c43c:
      FUN_00106380(&DAT_0011ac76);
      FUN_00106380(&DAT_0011ac75);
      break;
    case 0x10c459:
      DAT_001272f0 = 1;
      break;
    case 0x10c468:
      local_74 = (uint)(local_74 != 0);
      break;
    case 0x10c47c:
      goto switchD_0010be5c_caseD_10c47c;
    case 0x10c48d:
      lVar9 = FUN_0010d270("--time",optarg,&PTR_s_atime_00125200,&DAT_001196f0,4,PTR_FUN_001261f0,1)
      ;
      DAT_00127334 = *(int *)(&DAT_001196f0 + lVar9 * 4);
      break;
    case 0x10c4d8:
      lVar9 = FUN_0010d270("--sort",optarg,&DAT_00125240,&DAT_00119710,4,PTR_FUN_001261f0,1);
      local_6c = *(int *)(&DAT_00119710 + lVar9 * 4);
      break;
    case 0x10c521:
      DAT_00127328 = 0x90;
      DAT_0012731c = 0x90;
      DAT_00127320 = 1;
      DAT_00126020 = 1;
      break;
    case 1100000:
      local_74 = 0;
      break;
    case 0x10c9a1:
      bVar2 = true;
      break;
    case 0x10cc62:
      goto switchD_0010be5c_caseD_10cc62;
    case 0x10cc6c:
      goto switchD_0010be5c_caseD_10cc6c;
    case 0x10cc73:
      goto switchD_0010be5c_caseD_10cc73;
    }
    goto LAB_0010be20;
  }
  if (DAT_00127320 == 0) {
    pcVar16 = getenv("LS_BLOCK_SIZE");
    FUN_00110810(pcVar16,&DAT_00127328,&DAT_00127320);
    if ((pcVar16 != (char *)0x0) || (pcVar16 = getenv("BLOCK_SIZE"), pcVar16 != (char *)0x0)) {
      DAT_0012731c = DAT_00127328;
      DAT_00126020 = DAT_00127320;
    }
    if (bVar2) {
      DAT_00127328 = 0;
      DAT_00127320 = 0x400;
    }
  }
  if ((int)local_74 < 0) {
    if (DAT_001261e0 == 1) {
      cVar4 = FUN_00106590();
      if (cVar4 != '\0') goto LAB_0010cae0;
      DAT_00127338 = 1;
    }
    else {
      if (DAT_001261e0 == 2) {
LAB_0010cae0:
        DAT_00127338 = 2;
        goto LAB_0010c580;
      }
      DAT_00127338 = 0;
    }
LAB_0010c89f:
    if (DAT_00127312 != 0) goto LAB_0010c580;
    if (local_68 == 0xffffffffffffffff) goto LAB_0010c8b8;
  }
  else {
    DAT_00127338 = local_74;
    if (2 < local_74 - 2) goto LAB_0010c89f;
LAB_0010c580:
    if ((local_68 == 0xffffffffffffffff) &&
       (((cVar4 = FUN_00106590(), cVar4 == '\0' || (iVar6 = ioctl(1,0x5413,&local_48), iVar6 < 0))
        || (local_68 = (ulong)uStack_46, uStack_46 == 0)))) {
      param_2 = getenv("COLUMNS");
      if ((param_2 == (char *)0x0) || (*param_2 == '\0')) {
LAB_0010c8b8:
        local_68 = 0x50;
      }
      else {
        local_68 = FUN_001063b0(param_2);
        if ((long)local_68 < 0) {
          uVar12 = FUN_001158c0(param_2);
          uVar13 = dcgettext(0,"ignoring invalid width in environment variable COLUMNS: %s",5);
          error(0,0,uVar13,uVar12);
          local_68 = 0x50;
        }
      }
    }
  }
  DAT_001272b0 = local_68;
  auVar1._8_8_ = 0;
  auVar1._0_8_ = local_68;
  DAT_00127200 = (ulong)(local_68 !=
                        (SUB168(auVar1 * ZEXT816(0xaaaaaaaaaaaaaaab),8) & 0xfffffffffffffffe) +
                        local_68 / 3) + local_68 / 3;
  if ((DAT_00127338 - 2 < 3) && (DAT_001272c0 = local_58, local_58 < 0)) {
    DAT_001272c0 = 8;
    param_2 = getenv("TABSIZE");
    if (param_2 != (char *)0x0) {
      iVar6 = FUN_00117190(param_2,0,0,&local_48,&DAT_0011abb9);
      if (iVar6 == 0) {
        DAT_001272c0 = CONCAT44(uStack_44,CONCAT22(uStack_46,local_48));
      }
      else {
        uVar12 = FUN_001158c0(param_2);
        uVar13 = dcgettext(0,"ignoring invalid tab size in environment variable TABSIZE: %s",5);
        error(0,0,uVar13,uVar12);
      }
    }
  }
  bVar5 = (byte)local_50;
  if ((local_50 == -1) && (bVar5 = 0, DAT_001261e0 == 1)) {
    bVar5 = FUN_00106590();
  }
  DAT_001272d8 = bVar5 & 1;
  if (((local_70 < 0) && (local_70 = FUN_001064b0(), local_70 < 0)) &&
     (local_70 = 7, DAT_001261e0 == 1)) {
    cVar4 = FUN_00106590();
    if (cVar4 != '\0') {
      local_70 = 3;
      goto LAB_0010c5fd;
    }
  }
  else {
LAB_0010c5fd:
    FUN_00115200(0,local_70);
  }
  iVar6 = FUN_001151e0(0);
  if (((DAT_00127338 == 0) || ((DAT_00127338 - 2 < 2 && (DAT_001272b0 != 0)))) &&
     ((iVar6 == 3 || ((iVar6 == 6 || (iVar6 == 1)))))) {
    DAT_001273a8 = 1;
    DAT_001272d0 = FUN_001151a0(0);
  }
  else {
    DAT_001273a8 = 0;
    DAT_001272d0 = FUN_001151a0(0);
    if (iVar6 == 7) {
      FUN_00115220(DAT_001272d0,0x20,1);
    }
  }
  if (1 < DAT_00127314) {
    pcVar16 = &DAT_0011ad24 + (DAT_00127314 - 2);
    cVar4 = (&DAT_0011ad24)[DAT_00127314 - 2];
    while (cVar4 != '\0') {
      pcVar16 = pcVar16 + 1;
      FUN_00115220(DAT_001272d0,(int)cVar4,1);
      cVar4 = *pcVar16;
    }
  }
  DAT_001272c8 = FUN_001151a0(0);
  FUN_00115220(DAT_001272c8,0x3a,1);
  DAT_00127318 = (DAT_00127311 ^ 1) & DAT_00127338 == 0 & DAT_00127318;
  if ((int)DAT_00126019 < (int)(uint)DAT_00127318) {
LAB_0010cd66:
    uVar12 = dcgettext(0,"--dired and --zero are incompatible",5);
    error(2,0,uVar12);
LAB_0010cd8a:
    uVar12 = FUN_001158c0(param_2);
    uVar13 = dcgettext(0,"invalid time style format %s",5);
    error(2,0,uVar13,uVar12);
  }
  else {
    if (local_6c < 0) {
      if (DAT_00127338 == 0) {
        DAT_00127330 = 0;
        goto LAB_0010c7c2;
      }
      if (2 < DAT_00127334 - 1U) {
        local_6c = 0;
        goto LAB_0010c700;
      }
      DAT_00127330 = 5;
    }
    else {
LAB_0010c700:
      DAT_00127330 = local_6c;
      if (DAT_00127338 == 0) {
LAB_0010c7c2:
        if ((local_60 == (char *)0x0) && (local_60 = getenv("TIME_STYLE"), local_60 == (char *)0x0))
        {
          local_60 = (char *)FUN_001055b0("locale");
        }
        while (iVar6 = strncmp(local_60,"posix-",6), iVar6 == 0) {
          cVar4 = FUN_0010e9c0(2);
          if (cVar4 == '\0') goto LAB_0010c70e;
          local_60 = local_60 + 6;
        }
        if (*local_60 == '+') {
          param_2 = local_60 + 1;
          pcVar10 = strchr(param_2,10);
          pcVar16 = param_2;
          pcVar11 = param_2;
          if (pcVar10 != (char *)0x0) {
            pcVar16 = strchr(pcVar10 + 1,10);
            if (pcVar16 != (char *)0x0) goto LAB_0010cd8a;
            *pcVar10 = '\0';
            pcVar16 = param_2;
            pcVar11 = pcVar10 + 1;
          }
        }
        else {
          ppuVar15 = &PTR_s_full_iso_00125980;
          lVar9 = FUN_0010cf70(local_60,&PTR_s_full_iso_00125980,&DAT_00119760,4);
          if (lVar9 < 0) {
            FUN_0010d0e0("time style",local_60,lVar9);
            pFVar3 = stderr;
            pcVar16 = (char *)dcgettext(0,"Valid arguments are:\n",5);
            fputs_unlocked(pcVar16,pFVar3);
            pcVar16 = "full-iso";
            do {
              ppuVar15 = ppuVar15 + 1;
              __fprintf_chk(stderr,1,"  - [posix-]%s\n",pcVar16);
              pFVar3 = stderr;
              pcVar16 = *ppuVar15;
            } while (pcVar16 != (char *)0x0);
            pcVar16 = (char *)dcgettext(0,"  - +FORMAT (e.g., +%H:%M) for a \'date\'-style format\n"
                                        ,5);
            fputs_unlocked(pcVar16,pFVar3);
switchD_0010be5c_caseD_10cc62:
            FUN_0010b690();
switchD_0010be5c_caseD_10cc6c:
            uVar17 = 0;
            FUN_0010b690();
switchD_0010be5c_caseD_10cc73:
            pcVar16 = "ls";
            if ((DAT_001261e0 != 1) && (pcVar16 = "dir", DAT_001261e0 != 2)) {
              pcVar16 = "vdir";
            }
            FUN_00116870(stdout,pcVar16,"GNU coreutils",PTR_DAT_001261e8,"Richard M. Stallman",
                         "David MacKenzie",0,uVar17);
                    /* WARNING: Subroutine does not return */
            exit(0);
          }
          if (lVar9 == 2) {
            PTR_s__b__e__Y_00126040 = s__Y__m__d_0011adaa;
            PTR_s__b__e__H__M_00126048 = &DAT_0011ad9e;
            pcVar16 = PTR_s__b__e__Y_00126040;
            pcVar11 = PTR_s__b__e__H__M_00126048;
          }
          else {
            pcVar16 = PTR_s__b__e__Y_00126040;
            pcVar11 = PTR_s__b__e__H__M_00126048;
            if (lVar9 < 3) {
              if (lVar9 == 0) {
                PTR_s__b__e__H__M_00126048 = s__Y__m__d__H__M__S__N__z_0011ad83;
                PTR_s__b__e__Y_00126040 = s__Y__m__d__H__M__S__N__z_0011ad83;
                pcVar16 = PTR_s__b__e__Y_00126040;
                pcVar11 = PTR_s__b__e__H__M_00126048;
              }
              else if (lVar9 == 1) {
                PTR_s__b__e__H__M_00126048 = &DAT_0011ad9b;
                PTR_s__b__e__Y_00126040 = &DAT_0011ad9b;
                pcVar16 = PTR_s__b__e__Y_00126040;
                pcVar11 = PTR_s__b__e__H__M_00126048;
              }
            }
            else if ((lVar9 == 3) &&
                    (cVar4 = FUN_0010e9c0(2), pcVar16 = PTR_s__b__e__Y_00126040,
                    pcVar11 = PTR_s__b__e__H__M_00126048, cVar4 != '\0')) {
              PTR_s__b__e__Y_00126040 = (undefined *)dcgettext(0,PTR_s__b__e__Y_00126040,2);
              pcVar11 = (char *)dcgettext(0,PTR_s__b__e__H__M_00126048,2);
              pcVar16 = PTR_s__b__e__Y_00126040;
            }
          }
        }
        PTR_s__b__e__H__M_00126048 = pcVar11;
        PTR_s__b__e__Y_00126040 = pcVar16;
        FUN_001066c0();
      }
    }
LAB_0010c70e:
    if (local_40 == *(long *)(in_FS_OFFSET + 0x28)) {
      return optind;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
switchD_0010be5c_caseD_10c47c:
  local_60 = optarg;
  goto LAB_0010be20;
}




// Function: is_prime @ 0xea50

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




// Function: next_prime @ 0xeac0

ulong next_prime(ulong param_1)

{
  char cVar1;
  ulong uVar2;
  
  uVar2 = 10;
  if (9 < param_1) {
    uVar2 = param_1;
  }
  uVar2 = uVar2 | 1;
  while ((uVar2 != 0xffffffffffffffff && (cVar1 = FUN_0010ea50(uVar2), cVar1 == '\0'))) {
    uVar2 = uVar2 + 2;
  }
  return uVar2;
}




// Function: raw_hasher @ 0xeb00

ulong raw_hasher(undefined8 param_1,ulong param_2)

{
  ulong uVar1;
  
  uVar1 = thunk_FUN_00117e70(param_1,3);
  return uVar1 % param_2;
}




// Function: raw_comparator @ 0xeb20

bool raw_comparator(long param_1,long param_2)

{
  return param_2 == param_1;
}




// Function: check_tuning @ 0xeb30

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 check_tuning(long param_1)

{
  float fVar1;
  float *pfVar2;
  float fVar3;
  
  pfVar2 = *(float **)(param_1 + 0x28);
  if (pfVar2 == (float *)&DAT_0011d2f0) {
    return 1;
  }
  fVar1 = pfVar2[2];
  if ((((_DAT_0011d304 < fVar1) && (fVar1 < DAT_0011d308)) && (_DAT_0011d30c < pfVar2[3])) &&
     (_DAT_0011d320 <= *pfVar2)) {
    fVar3 = *pfVar2 + _DAT_0011d304;
    if (((fVar3 < pfVar2[1]) && (pfVar2[1] <= DAT_0011d310)) && (fVar3 < fVar1)) {
      return 1;
    }
  }
  *(undefined **)(param_1 + 0x28) = &DAT_0011d2f0;
  return 0;
}




// Function: free_entry @ 0xebc0

void free_entry(long param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  
  uVar1 = *(undefined8 *)(param_1 + 0x48);
  *param_2 = 0;
  param_2[1] = uVar1;
  *(undefined8 **)(param_1 + 0x48) = param_2;
  return;
}




// Function: safe_hasher @ 0xebe0

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




// Function: hash_find_entry @ 0xec10

long hash_find_entry(long param_1,long param_2,undefined8 *param_3,char param_4)

{
  long *plVar1;
  long lVar2;
  char cVar3;
  long *plVar4;
  long lVar5;
  
  plVar4 = (long *)FUN_0010ebe0();
  *param_3 = plVar4;
  lVar5 = *plVar4;
  if (lVar5 == 0) {
LAB_0010ecb1:
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
            FUN_0010ebc0(param_1);
            return lVar5;
          }
        }
        goto LAB_0010ecb1;
      }
      lVar5 = *plVar4;
    }
    if (param_4 != '\0') {
      plVar1 = (long *)plVar4[1];
      if (plVar1 != (long *)0x0) {
        lVar2 = plVar1[1];
        *plVar4 = *plVar1;
        plVar4[1] = lVar2;
        FUN_0010ebc0(param_1);
        return lVar5;
      }
      *plVar4 = 0;
    }
  }
  return lVar5;
}




// Function: allocate_entry @ 0xed10

void allocate_entry(long param_1)

{
  if (*(long *)(param_1 + 0x48) != 0) {
    *(undefined8 *)(param_1 + 0x48) = *(undefined8 *)(*(long *)(param_1 + 0x48) + 8);
    return;
  }
  malloc(0x10);
  return;
}




// Function: transfer_entries @ 0xed40

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
LAB_0010ed68:
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
          plVar2 = (long *)FUN_0010ebe0(param_1,lVar6);
          plVar4 = (long *)plVar3[1];
          if (*plVar2 != 0) break;
          *plVar2 = lVar6;
          *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
          FUN_0010ebc0(param_1);
          plVar3 = plVar4;
          if (plVar4 == (long *)0x0) goto LAB_0010edd8;
        }
        plVar3[1] = plVar2[1];
        plVar2[1] = (long)plVar3;
        plVar3 = plVar4;
      } while (plVar4 != (long *)0x0);
LAB_0010edd8:
      lVar6 = *plVar5;
    }
    plVar5[1] = 0;
    if (param_3 != '\0') goto LAB_0010ed68;
    plVar3 = (long *)FUN_0010ebe0(param_1,lVar6);
    if (*plVar3 == 0) {
      *plVar3 = lVar6;
      *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
    }
    else {
      plVar4 = (long *)FUN_0010ed10(param_1);
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




// Function: hash_get_n_buckets @ 0xef00

undefined8 hash_get_n_buckets(long param_1)

{
  return *(undefined8 *)(param_1 + 0x10);
}




// Function: hash_get_n_buckets_used @ 0xef10

undefined8 hash_get_n_buckets_used(long param_1)

{
  return *(undefined8 *)(param_1 + 0x18);
}




// Function: hash_get_n_entries @ 0xef20

undefined8 hash_get_n_entries(long param_1)

{
  return *(undefined8 *)(param_1 + 0x20);
}




// Function: hash_get_max_bucket_length @ 0xef30

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




// Function: hash_print_statistics @ 0xf000

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void hash_print_statistics(undefined8 param_1,undefined8 param_2)

{
  undefined8 uVar1;
  ulong uVar2;
  ulong uVar3;
  undefined8 uVar4;
  
  uVar1 = FUN_0010ef20();
  uVar2 = FUN_0010ef00();
  uVar3 = FUN_0010ef10();
  uVar4 = FUN_0010ef30();
  __fprintf_chk(param_2,1,"# entries:         %lu\n",uVar1);
  __fprintf_chk(param_2,1,"# buckets:         %lu\n",uVar2);
  __fprintf_chk(((double)uVar3 * _DAT_0011d320) / (double)uVar2,param_2,1,
                "# buckets used:    %lu (%.2f%%)\n",uVar3);
  __fprintf_chk(param_2,1,"max bucket length: %lu\n",uVar4);
  return;
}




// Function: hash_lookup @ 0xf120

long hash_lookup(long param_1,long param_2)

{
  char cVar1;
  long *plVar2;
  long lVar3;
  
  plVar2 = (long *)FUN_0010ebe0();
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




// Function: hash_get_next @ 0xf1d0

long hash_get_next(long param_1,long param_2)

{
  long lVar1;
  long *plVar2;
  long *plVar3;
  
  plVar2 = (long *)FUN_0010ebe0();
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




// Function: hash_do_for_each @ 0xf290

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




// Function: hash_initialize @ 0xf380

undefined8 *
hash_initialize(undefined8 param_1,undefined *param_2,code *param_3,code *param_4,undefined8 param_5)

{
  char cVar1;
  undefined8 *__ptr;
  size_t __nmemb;
  void *pvVar2;
  
  if (param_3 == (code *)0x0) {
    param_3 = FUN_0010eb00;
  }
  if (param_4 == (code *)0x0) {
    param_4 = FUN_0010eb20;
  }
  __ptr = malloc(0x50);
  if (__ptr != (undefined8 *)0x0) {
    if (param_2 == (undefined *)0x0) {
      param_2 = &DAT_0011d2f0;
    }
    __ptr[5] = param_2;
    cVar1 = FUN_0010eb30(__ptr);
    if (cVar1 != '\0') {
      __nmemb = FUN_0010ee70(*(undefined4 *)(param_2 + 8),param_1,param_2[0x10]);
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




// Function: hash_clear @ 0xf490

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
        if ((long *)param_1[1] <= plVar7) goto LAB_0010f52b;
      }
      plVar6 = plVar6 + 2;
    } while (plVar6 < (long *)param_1[1]);
  }
LAB_0010f52b:
  param_1[3] = 0;
  param_1[4] = 0;
  return;
}




// Function: hash_free @ 0xf540

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
    if (plVar3 <= plVar5) goto LAB_0010f5ec;
    do {
      while (lVar6 = *plVar5, plVar4 = plVar5, lVar6 == 0) {
        plVar5 = plVar5 + 2;
        if (plVar3 <= plVar5) goto LAB_0010f5af;
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
LAB_0010f5af:
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
LAB_0010f5ec:
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




// Function: hash_rehash @ 0xf630

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
  __nmemb = FUN_0010ee70(*(undefined4 *)(param_1[5] + 8),param_2,*(undefined1 *)(param_1[5] + 0x10))
  ;
  if (__nmemb != 0) {
    if (param_1[2] == __nmemb) {
      uVar2 = 1;
      goto LAB_0010f736;
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
      uVar2 = FUN_0010ed40(&local_88,param_1,0);
      if ((char)uVar2 == '\0') {
        param_1[9] = local_40;
        cVar1 = FUN_0010ed40(param_1,&local_88,1);
        if (cVar1 != '\0') {
          cVar1 = FUN_0010ed40(param_1,&local_88,0);
          if (cVar1 != '\0') {
            free(local_88);
            goto LAB_0010f736;
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
      goto LAB_0010f736;
    }
  }
  uVar2 = 0;
LAB_0010f736:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_insert_if_absent @ 0xf7b0

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
LAB_0010f977:
                    /* WARNING: Subroutine does not return */
    abort();
  }
  lVar2 = FUN_0010ec10(param_1,param_2,&local_38,0);
  if (lVar2 != 0) {
    uVar5 = 0;
    if (param_3 != (long *)0x0) {
      *param_3 = lVar2;
    }
    goto LAB_0010f7fe;
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
    FUN_0010eb30(param_1);
    lVar2 = *(long *)(param_1 + 0x28);
    if (auVar8._0_4_ <= auVar7._0_4_ * *(float *)(lVar2 + 8)) goto LAB_0010f85e;
    fVar6 = auVar7._0_4_ * *(float *)(lVar2 + 0xc);
    if (*(char *)(lVar2 + 0x10) == '\0') {
      fVar6 = fVar6 * *(float *)(lVar2 + 8);
    }
    if (fVar6 < _DAT_0011d314) {
      if (_DAT_0011d318 <= fVar6) {
        uVar4 = (long)(fVar6 - _DAT_0011d318) ^ 0x8000000000000000;
      }
      else {
        uVar4 = (ulong)fVar6;
      }
      cVar1 = FUN_0010f630(param_1,uVar4);
      if (cVar1 != '\0') {
        lVar2 = FUN_0010ec10(param_1,param_2,&local_38,0);
        if (lVar2 != 0) goto LAB_0010f977;
        goto LAB_0010f85e;
      }
    }
  }
  else {
LAB_0010f85e:
    if (*local_38 == 0) {
      *local_38 = param_2;
      uVar5 = 1;
      *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
      *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
      goto LAB_0010f7fe;
    }
    plVar3 = (long *)FUN_0010ed10(param_1);
    if (plVar3 != (long *)0x0) {
      *plVar3 = param_2;
      uVar5 = 1;
      plVar3[1] = local_38[1];
      local_38[1] = (long)plVar3;
      *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
      goto LAB_0010f7fe;
    }
  }
  uVar5 = 0xffffffff;
LAB_0010f7fe:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_insert @ 0xf9a0

undefined8 hash_insert(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_0010f7b0(param_1,param_2,&local_18);
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




// Function: hash_remove @ 0xfa00

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
  lVar4 = FUN_0010ec10(param_1,param_2,&local_28,1);
  if ((lVar4 == 0) || (*(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + -1, *local_28 != 0))
  goto LAB_0010fa43;
  uVar5 = *(long *)(param_1 + 0x18) - 1;
  *(ulong *)(param_1 + 0x18) = uVar5;
  if ((long)uVar5 < 0) {
    uVar6 = *(ulong *)(param_1 + 0x10);
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)uVar5;
    if (-1 < (long)uVar6) goto LAB_0010fa90;
LAB_0010fb46:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)uVar6;
  }
  else {
    auVar9._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar9._0_4_ = (float)(long)uVar5;
    uVar6 = *(ulong *)(param_1 + 0x10);
    if ((long)uVar6 < 0) goto LAB_0010fb46;
LAB_0010fa90:
    auVar8._4_12_ = SUB1612((undefined1  [16])0x0,4);
    auVar8._0_4_ = (float)(long)uVar6;
  }
  if (auVar9._0_4_ < **(float **)(param_1 + 0x28) * auVar8._0_4_) {
    FUN_0010eb30(param_1);
    pfVar1 = *(float **)(param_1 + 0x28);
    if (auVar9._0_4_ < *pfVar1 * auVar8._0_4_) {
      fVar7 = auVar8._0_4_ * pfVar1[1];
      if (*(char *)(pfVar1 + 4) == '\0') {
        fVar7 = fVar7 * pfVar1[2];
      }
      if (_DAT_0011d318 <= fVar7) {
        uVar5 = (long)(fVar7 - _DAT_0011d318) ^ 0x8000000000000000;
      }
      else {
        uVar5 = (ulong)fVar7;
      }
      cVar3 = FUN_0010f630(param_1,uVar5);
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
LAB_0010fa43:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar4;
}




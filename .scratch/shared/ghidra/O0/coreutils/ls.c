// Function: file_or_link_mode @ 0x53e7

undefined4 file_or_link_mode(long param_1)

{
  undefined4 uVar1;
  
  if ((DAT_0012b358 == '\0') || (*(char *)(param_1 + 0xb9) == '\0')) {
    uVar1 = *(undefined4 *)(param_1 + 0x30);
  }
  else {
    uVar1 = *(undefined4 *)(param_1 + 0xac);
  }
  return uVar1;
}




// Function: dired_outbyte @ 0x5422

void dired_outbyte(char param_1)

{
  DAT_0012b500 = DAT_0012b500 + 1;
  putchar_unlocked((int)param_1);
  return;
}




// Function: dired_outbuf @ 0x5453

void dired_outbuf(void *param_1,size_t param_2)

{
  DAT_0012b500 = param_2 + DAT_0012b500;
  fwrite_unlocked(param_1,1,param_2,stdout);
  return;
}




// Function: dired_outstring @ 0x549e

void dired_outstring(char *param_1)

{
  size_t sVar1;
  
  sVar1 = strlen(param_1);
  FUN_00105453(param_1,sVar1);
  return;
}




// Function: dired_indent @ 0x54cf

void dired_indent(void)

{
  if (DAT_0012b3cc != '\0') {
    FUN_0010549e(&DAT_001209a4);
  }
  return;
}




// Function: push_current_dired_pos @ 0x54f4

void push_current_dired_pos(long param_1)

{
  if (DAT_0012b3cc != '\0') {
    if ((ulong)(*(long *)(param_1 + 0x20) - *(long *)(param_1 + 0x18)) < 8) {
      _obstack_newchunk(param_1,8);
    }
    memcpy(*(void **)(param_1 + 0x18),&DAT_0012b500,8);
    *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 8;
  }
  return;
}




// Function: dev_ino_push @ 0x558e

void dev_ino_push(undefined8 param_1,undefined8 param_2)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  
  if ((ulong)(DAT_0012b600 - (long)DAT_0012b5f8) < 0x10) {
    _obstack_newchunk(&DAT_0012b5e0,0x10);
  }
  puVar2 = DAT_0012b5f8;
  puVar1 = DAT_0012b5f8 + 1;
  DAT_0012b5f8 = DAT_0012b5f8 + 2;
  *puVar1 = param_1;
  *puVar2 = param_2;
  return;
}




// Function: dev_ino_pop @ 0x563d

undefined1  [16] dev_ino_pop(void)

{
  if ((ulong)((long)DAT_0012b5f8 - DAT_0012b5f0) < 0x10) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("dev_ino_size <= obstack_object_size (&dev_ino_obstack)","src/ls.c",0x41d,
                  "dev_ino_pop");
  }
  DAT_0012b5f8 = (undefined1 (*) [16])((long)DAT_0012b5f8 + -0x10);
  return *DAT_0012b5f8;
}




// Function: assert_matching_dev_ino @ 0x56dc

void assert_matching_dev_ino(char *param_1,__ino_t param_2,__dev_t param_3)

{
  int iVar1;
  long in_FS_OFFSET;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 == (char *)0x0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("name","src/ls.c",0x428,"assert_matching_dev_ino");
  }
  iVar1 = stat(param_1,&local_a8);
  if (iVar1 < 0) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("0 <= stat (name, &sb)","src/ls.c",0x429,"assert_matching_dev_ino");
  }
  if (local_a8.st_dev != param_3) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("sb.st_dev == di.st_dev","src/ls.c",0x42a,"assert_matching_dev_ino");
  }
  if (local_a8.st_ino != param_2) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("sb.st_ino == di.st_ino","src/ls.c",0x42b,"assert_matching_dev_ino");
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: dired_dump_obstack @ 0x581f

void dired_dump_obstack(char *param_1,long param_2)

{
  long lVar1;
  ulong uVar2;
  ulong local_40;
  
  uVar2 = (ulong)(*(long *)(param_2 + 0x18) - *(long *)(param_2 + 0x10)) >> 3;
  if (uVar2 != 0) {
    lVar1 = *(long *)(param_2 + 0x10);
    if (lVar1 == *(long *)(param_2 + 0x18)) {
      *(byte *)(param_2 + 0x50) = *(byte *)(param_2 + 0x50) | 2;
    }
    *(ulong *)(param_2 + 0x18) =
         ~*(ulong *)(param_2 + 0x30) & *(long *)(param_2 + 0x18) + *(long *)(param_2 + 0x30);
    if ((ulong)(*(long *)(param_2 + 0x20) - *(long *)(param_2 + 8)) <
        (ulong)(*(long *)(param_2 + 0x18) - *(long *)(param_2 + 8))) {
      *(undefined8 *)(param_2 + 0x18) = *(undefined8 *)(param_2 + 0x20);
    }
    *(undefined8 *)(param_2 + 0x10) = *(undefined8 *)(param_2 + 0x18);
    fputs_unlocked(param_1,stdout);
    for (local_40 = 0; local_40 < uVar2; local_40 = local_40 + 1) {
      printf(" %ld",*(undefined8 *)(lVar1 + local_40 * 8));
    }
    putchar_unlocked(10);
  }
  return;
}




// Function: get_stat_btime @ 0x5990

undefined1  [16] get_stat_btime(undefined8 param_1)

{
  undefined1 auVar1 [16];
  
  auVar1 = FUN_0011b491(param_1);
  return auVar1;
}




// Function: time_type_to_statx @ 0x59be

undefined8 time_type_to_statx(void)

{
  undefined8 uVar1;
  
  if (DAT_0012b3b0 == 3) {
    uVar1 = 0x800;
  }
  else {
    if (3 < DAT_0012b3b0) {
LAB_00105a05:
                    /* WARNING: Subroutine does not return */
      abort();
    }
    if (DAT_0012b3b0 == 2) {
      uVar1 = 0x20;
    }
    else {
      if (2 < DAT_0012b3b0) goto LAB_00105a05;
      if (DAT_0012b3b0 == 0) {
        uVar1 = 0x40;
      }
      else {
        if (DAT_0012b3b0 != 1) goto LAB_00105a05;
        uVar1 = 0x80;
      }
    }
  }
  return uVar1;
}




// Function: calc_req_mask @ 0x5a0c

uint calc_req_mask(void)

{
  uint uVar1;
  uint local_c;
  
  local_c = 2;
  if (DAT_0012b3e9 != '\0') {
    local_c = 0x102;
  }
  if (DAT_0012b3bb != '\0') {
    local_c = local_c | 0x400;
  }
  if (DAT_0012b3ac == 0) {
    uVar1 = FUN_001059be();
    uVar1 = uVar1 | local_c;
    local_c = uVar1 | 0x204;
    if ((DAT_0012b022 != '\0') || (DAT_0012b3b9 != '\0')) {
      local_c = uVar1 | 0x20c;
    }
    if (DAT_0012b023 != '\0') {
      local_c = local_c | 0x10;
    }
  }
  switch(DAT_0012b3b4) {
  case 0:
  case 1:
  case 2:
  case 4:
  case 6:
    break;
  case 3:
    local_c = local_c | 0x200;
    break;
  case 5:
    uVar1 = FUN_001059be();
    local_c = local_c | uVar1;
    break;
  default:
                    /* WARNING: Subroutine does not return */
    abort();
  }
  return local_c;
}




// Function: do_statx @ 0x5ad2

int do_statx(undefined4 param_1,undefined8 param_2,long param_3,uint param_4,uint param_5)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined1 auVar2 [16];
  uint local_128 [20];
  undefined8 local_d8;
  undefined8 local_d0;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = statx(param_1,param_2,param_4 | 0x800,param_5,local_128);
  if (-1 < iVar1) {
    FUN_00105297(local_128,param_3);
    if ((param_5 & 0x800) != 0) {
      if ((local_128[0] & 0x800) == 0) {
        *(undefined8 *)(param_3 + 0x60) = 0xffffffffffffffff;
        *(undefined8 *)(param_3 + 0x58) = *(undefined8 *)(param_3 + 0x60);
      }
      else {
        auVar2 = FUN_00105267(local_d8,local_d0);
        *(undefined1 (*) [16])(param_3 + 0x58) = auVar2;
      }
    }
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return iVar1;
}




// Function: do_stat @ 0x5c0e

void do_stat(undefined8 param_1,undefined8 param_2)

{
  undefined4 uVar1;
  
  uVar1 = FUN_00105a0c();
  FUN_00105ad2(0xffffff9c,param_1,param_2,0,uVar1);
  return;
}




// Function: do_lstat @ 0x5c48

void do_lstat(undefined8 param_1,undefined8 param_2)

{
  undefined4 uVar1;
  
  uVar1 = FUN_00105a0c();
  FUN_00105ad2(0xffffff9c,param_1,param_2,0x100,uVar1);
  return;
}




// Function: stat_for_mode @ 0x5c82

void stat_for_mode(undefined8 param_1,undefined8 param_2)

{
  FUN_00105ad2(0xffffff9c,param_1,param_2,0,2);
  return;
}




// Function: stat_for_ino @ 0x5cb8

void stat_for_ino(undefined8 param_1,undefined8 param_2)

{
  FUN_00105ad2(0xffffff9c,param_1,param_2,0,0x100);
  return;
}




// Function: fstat_for_ino @ 0x5cee

void fstat_for_ino(undefined4 param_1,undefined8 param_2)

{
  FUN_00105ad2(param_1,&DAT_0011f27b,param_2,0x1000,0x100);
  return;
}




// Function: first_percent_b @ 0x5d23

char * first_percent_b(char *param_1)

{
  char *local_10;
  
  local_10 = param_1;
  do {
    if (*local_10 == '\0') {
      return (char *)0x0;
    }
    if (*local_10 == '%') {
      if (local_10[1] == '%') {
        local_10 = local_10 + 1;
      }
      else if (local_10[1] == 'b') {
        return local_10;
      }
    }
    local_10 = local_10 + 1;
  } while( true );
}




// Function: file_escape_init @ 0x5d77

void file_escape_init(void)

{
  char cVar1;
  byte bVar2;
  int local_c;
  
  for (local_c = 0; local_c < 0x100; local_c = local_c + 1) {
    cVar1 = FUN_00110016(local_c);
    if ((((cVar1 == '\0') && (local_c != 0x7e)) && (local_c != 0x2d)) &&
       ((local_c != 0x2e && (local_c != 0x5f)))) {
      bVar2 = 0;
    }
    else {
      bVar2 = 1;
    }
    (&DAT_0012b640)[local_c] = (&DAT_0012b640)[local_c] | bVar2;
  }
  return;
}




// Function: abmon_init @ 0x5df4

undefined8 abmon_init(long param_1)

{
  ushort *puVar1;
  byte bVar2;
  char *pcVar3;
  ushort **ppuVar4;
  ulong uVar5;
  undefined8 uVar6;
  long in_FS_OFFSET;
  int local_50;
  ulong local_48;
  ulong local_40;
  ulong local_38;
  char *local_30;
  ulong local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_40 = 0xc;
  do {
    local_38 = local_40;
    local_40 = 0;
    for (local_50 = 0; local_50 < 0xc; local_50 = local_50 + 1) {
      local_48 = local_38;
      local_30 = nl_langinfo(local_50 + 0x2000e);
      pcVar3 = strchr(local_30,0x25);
      if (pcVar3 != (char *)0x0) {
        uVar6 = 0;
        goto LAB_00105f29;
      }
      ppuVar4 = __ctype_b_loc();
      puVar1 = *ppuVar4;
      bVar2 = FUN_00104eb2((int)*local_30);
      local_28 = FUN_00115291(local_30,(long)local_50 * 0x80 + param_1,0x80,&local_48,
                              (puVar1[bVar2] & 0x800) != 0,0);
      if (0x7f < local_28) {
        uVar6 = 0;
        goto LAB_00105f29;
      }
      uVar5 = local_48;
      if (local_48 <= local_40) {
        uVar5 = local_40;
      }
      local_40 = uVar5;
    }
  } while (local_40 < local_38);
  uVar6 = 1;
LAB_00105f29:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar6;
}




// Function: abformat_init @ 0x5f43

void abformat_init(void)

{
  undefined *puVar1;
  char cVar2;
  long lVar3;
  long in_FS_OFFSET;
  int local_64c;
  int local_648;
  int local_644;
  int local_640;
  long local_628 [2];
  undefined1 local_618 [1544];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  for (local_64c = 0; local_64c < 2; local_64c = local_64c + 1) {
    lVar3 = FUN_00105d23((&PTR_s__b__e__Y_0012b200)[local_64c]);
    local_628[local_64c] = lVar3;
  }
  if (((local_628[0] != 0) || (local_628[1] != 0)) &&
     (cVar2 = FUN_00105df4(local_618), cVar2 == '\x01')) {
    for (local_648 = 0; local_648 < 2; local_648 = local_648 + 1) {
      puVar1 = (&PTR_s__b__e__Y_0012b200)[local_648];
      for (local_644 = 0; local_644 < 0xc; local_644 = local_644 + 1) {
        if (local_628[local_648] == 0) {
          local_640 = snprintf(&DAT_0012b740 + ((long)local_648 * 0xc + (long)local_644) * 0x80,0x80
                               ,"%s",puVar1);
        }
        else {
          if (0x80 < local_628[local_648] - (long)puVar1) goto LAB_001061a7;
          local_640 = snprintf(&DAT_0012b740 + ((long)local_648 * 0xc + (long)local_644) * 0x80,0x80
                               ,"%.*s%s%s",(ulong)(uint)((int)local_628[local_648] - (int)puVar1),
                               puVar1,local_618 + (long)local_644 * 0x80,local_628[local_648] + 2);
        }
        if ((local_640 < 0) || (0x7f < local_640)) goto LAB_001061a7;
      }
    }
    DAT_0012c340 = 1;
  }
LAB_001061a7:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: dev_ino_hash @ 0x61bd

ulong dev_ino_hash(ulong *param_1,ulong param_2)

{
  return *param_1 % param_2;
}




// Function: dev_ino_compare @ 0x61ea

undefined4 dev_ino_compare(long *param_1,long *param_2)

{
  undefined4 uVar1;
  
  if ((*param_1 == *param_2) && (param_1[1] == param_2[1])) {
    uVar1 = 1;
  }
  else {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: dev_ino_free @ 0x6243

void dev_ino_free(void *param_1)

{
  free(param_1);
  return;
}




// Function: visit_dir @ 0x6262

bool visit_dir(undefined8 param_1,undefined8 param_2)

{
  undefined8 *__ptr;
  undefined8 *puVar1;
  
  __ptr = (undefined8 *)FUN_0011c68b(0x10);
  *__ptr = param_2;
  __ptr[1] = param_1;
  puVar1 = (undefined8 *)FUN_001136d0(DAT_0012b320,__ptr);
  if (puVar1 == (undefined8 *)0x0) {
    FUN_0011ccc3();
  }
  if (puVar1 != __ptr) {
    free(__ptr);
  }
  return puVar1 != __ptr;
}




// Function: free_pending_ent @ 0x62e7

void free_pending_ent(undefined8 *param_1)

{
  free((void *)*param_1);
  free((void *)param_1[1]);
  free(param_1);
  return;
}




// Function: is_colored @ 0x6325

undefined4 is_colored(uint param_1)

{
  long lVar1;
  char *__s1;
  int iVar2;
  undefined4 uVar3;
  
  lVar1 = (&DAT_0012b080)[(ulong)param_1 * 2];
  __s1 = (&PTR_DAT_0012b088)[(ulong)param_1 * 2];
  if (lVar1 == 0) {
LAB_001063c0:
    uVar3 = 0;
  }
  else {
    if (lVar1 == 1) {
      iVar2 = strncmp(__s1,"0",1);
      if (iVar2 == 0) goto LAB_001063c0;
    }
    if (lVar1 == 2) {
      iVar2 = strncmp(__s1,"00",2);
      if (iVar2 == 0) goto LAB_001063c0;
    }
    uVar3 = 1;
  }
  return uVar3;
}




// Function: restore_default_color @ 0x63ca

void restore_default_color(void)

{
  FUN_0010e4ca(&DAT_0012b080);
  FUN_0010e4ca(&DAT_0012b090);
  return;
}




// Function: set_normal_color @ 0x63f3

void set_normal_color(void)

{
  char cVar1;
  
  if (DAT_0012b3d4 != '\0') {
    cVar1 = FUN_00106325(4);
    if (cVar1 != '\0') {
      FUN_0010e4ca(&DAT_0012b080);
      FUN_0010e4ca(&DAT_0012b0c0);
      FUN_0010e4ca(&DAT_0012b090);
    }
  }
  return;
}




// Function: sighandler @ 0x6444

void sighandler(int param_1)

{
  if (DAT_0012b4e0 == 0) {
    DAT_0012b4e0 = param_1;
  }
  return;
}




// Function: stophandler @ 0x6465

void stophandler(void)

{
  if (DAT_0012b4e0 == 0) {
    DAT_0012b4e4 = DAT_0012b4e4 + 1;
  }
  return;
}




// Function: process_signals @ 0x648c

void process_signals(void)

{
  long in_FS_OFFSET;
  int local_a0;
  sigset_t local_98;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  while ((DAT_0012b4e0 != 0 || (DAT_0012b4e4 != 0))) {
    if (DAT_0012b3d6 != '\0') {
      FUN_001063ca();
    }
    fflush_unlocked(stdout);
    sigprocmask(0,(sigset_t *)&DAT_0012b460,&local_98);
    local_a0 = DAT_0012b4e0;
    if (DAT_0012b4e4 == 0) {
      signal(DAT_0012b4e0,(__sighandler_t)0x0);
    }
    else {
      DAT_0012b4e4 = DAT_0012b4e4 + -1;
      local_a0 = 0x13;
    }
    raise(local_a0);
    sigprocmask(2,&local_98,(sigset_t *)0x0);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: signal_setup @ 0x6593

void signal_setup(char param_1)

{
  int iVar1;
  long in_FS_OFFSET;
  int local_ac;
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
  if (param_1 == '\0') {
    for (local_ac = 0; local_ac < 0xc; local_ac = local_ac + 1) {
      iVar1 = sigismember((sigset_t *)&DAT_0012b460,*(int *)(&DAT_00123380 + (long)local_ac * 4));
      if (iVar1 != 0) {
        signal(*(int *)(&DAT_00123380 + (long)local_ac * 4),(__sighandler_t)0x0);
      }
    }
  }
  else {
    sigemptyset((sigset_t *)&DAT_0012b460);
    for (local_ac = 0; local_ac < 0xc; local_ac = local_ac + 1) {
      sigaction(*(int *)(&DAT_00123380 + (long)local_ac * 4),(sigaction *)0x0,(sigaction *)&local_a8
               );
      if (local_a8.sa_handler != (__sighandler_t)0x1) {
        sigaddset((sigset_t *)&DAT_0012b460,*(int *)(&DAT_00123380 + (long)local_ac * 4));
      }
    }
    local_a0 = DAT_0012b460;
    local_98 = DAT_0012b468;
    local_90 = DAT_0012b470;
    local_88 = DAT_0012b478;
    local_80 = DAT_0012b480;
    local_78 = DAT_0012b488;
    local_70 = DAT_0012b490;
    local_68 = DAT_0012b498;
    local_60 = DAT_0012b4a0;
    local_58 = DAT_0012b4a8;
    local_50 = DAT_0012b4b0;
    local_48 = DAT_0012b4b8;
    local_40 = DAT_0012b4c0;
    local_38 = DAT_0012b4c8;
    local_30 = DAT_0012b4d0;
    local_28 = DAT_0012b4d8;
    local_20 = 0x10000000;
    for (local_ac = 0; local_ac < 0xc; local_ac = local_ac + 1) {
      iVar1 = sigismember((sigset_t *)&DAT_0012b460,*(int *)(&DAT_00123380 + (long)local_ac * 4));
      if (iVar1 != 0) {
        if (*(int *)(&DAT_00123380 + (long)local_ac * 4) == 0x14) {
          local_a8.sa_handler = FUN_00106465;
        }
        else {
          local_a8.sa_handler = FUN_00106444;
        }
        sigaction(*(int *)(&DAT_00123380 + (long)local_ac * 4),(sigaction *)&local_a8,
                  (sigaction *)0x0);
      }
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: signal_init @ 0x6858

void signal_init(void)

{
  FUN_00106593(1);
  return;
}




// Function: signal_restore @ 0x686d

void signal_restore(void)

{
  FUN_00106593(0);
  return;
}




// Function: main @ 0x6882

undefined4 main(int param_1,undefined8 *param_2)

{
  int iVar1;
  long *plVar2;
  long *plVar3;
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  long *plVar6;
  char cVar7;
  int iVar8;
  uint uVar9;
  char *pcVar10;
  long lVar11;
  long in_FS_OFFSET;
  undefined1 auVar12 [16];
  int local_44;
  int local_40;
  undefined1 local_28 [16];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00118e89(*param_2);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  FUN_00104e99(2);
  FUN_0011ec10(FUN_001110ae);
  DAT_0012b4e8 = 0;
  DAT_0012b428 = 1;
  DAT_0012b368 = (long *)0x0;
  DAT_0012b370 = 0x8000000000000000;
  DAT_0012b378 = 0xffffffffffffffff;
  iVar8 = FUN_001070af(param_1,param_2);
  if (DAT_0012b3d4 != '\0') {
    FUN_001088ab();
  }
  if (DAT_0012b3d4 != '\0') {
    DAT_0012b420 = 0;
  }
  if (DAT_0012b3f2 == '\0') {
    if ((DAT_0012b3d4 != '\0') &&
       (((cVar7 = FUN_00106325(0xd), cVar7 != '\0' ||
         ((cVar7 = FUN_00106325(0xe), cVar7 != '\0' && (DAT_0012b358 != '\0')))) ||
        ((cVar7 = FUN_00106325(0xc), cVar7 != '\0' && (DAT_0012b3ac == 0)))))) {
      DAT_0012b3e8 = 1;
    }
  }
  else {
    DAT_0012b3e8 = 1;
  }
  if (DAT_0012b3ec == 0) {
    if (((DAT_0012b3f1 == '\0') && (DAT_0012b3d0 != 3)) && (DAT_0012b3ac != 0)) {
      DAT_0012b3ec = 3;
    }
    else {
      DAT_0012b3ec = 1;
    }
  }
  if (DAT_0012b3f0 != '\0') {
    DAT_0012b320 = FUN_0011298f(0x1e,0,FUN_001061bd,FUN_001061ea,FUN_00106243);
    if (DAT_0012b320 == 0) {
      FUN_0011ccc3();
    }
    _obstack_begin(&DAT_0012b5e0,0,0,PTR_malloc_0012afd0,PTR_free_0012af98);
  }
  pcVar10 = getenv("TZ");
  DAT_0012b438 = FUN_0011b525(pcVar10);
  if (((DAT_0012b3b4 == 5) || (DAT_0012b3b4 == 3)) ||
     ((DAT_0012b3ac == 0 || ((DAT_0012b380 != '\0' || (DAT_0012b3bb != '\0')))))) {
    DAT_0012b440 = '\x01';
  }
  else {
    DAT_0012b440 = '\0';
  }
  if ((DAT_0012b440 == '\x01') ||
     ((((DAT_0012b3f0 == '\0' && (DAT_0012b3d4 == '\0')) && (DAT_0012b3d0 == 0)) &&
      (DAT_0012b3f2 == '\0')))) {
    DAT_0012b441 = 0;
  }
  else {
    DAT_0012b441 = 1;
  }
  if (DAT_0012b3cc != '\0') {
    _obstack_begin(&DAT_0012b520,0,0,PTR_malloc_0012afd0,PTR_free_0012af98);
    _obstack_begin(&DAT_0012b580,0,0,PTR_malloc_0012afd0,PTR_free_0012af98);
  }
  if (DAT_0012b3d5 != '\0') {
    FUN_00105d77();
    DAT_0012b360 = (undefined *)FUN_0011ce96();
    if (DAT_0012b360 == (undefined *)0x0) {
      DAT_0012b360 = &DAT_0011f27b;
    }
  }
  DAT_0012b330 = 100;
  DAT_0012b328 = FUN_0011c804(100,0xd0);
  DAT_0012b338 = 0;
  FUN_001096f3();
  local_44 = iVar8;
  if (param_1 - iVar8 < 1) {
    if (DAT_0012b3f1 == '\0') {
      FUN_00108e46(&DAT_00120a7e,0,1);
    }
    else {
      FUN_00109a40(&DAT_00120a7e,3,0,1,&DAT_0011f27b);
    }
  }
  else {
    do {
      iVar1 = local_44 + 1;
      FUN_00109a40(param_2[local_44],0,0,1,&DAT_0011f27b);
      local_44 = iVar1;
    } while (iVar1 < param_1);
  }
  if ((DAT_0012b338 != 0) && (FUN_0010bfc5(), DAT_0012b3f1 != '\x01')) {
    FUN_0010aa65(0,1);
  }
  auVar5._8_8_ = local_28._8_8_;
  auVar5._0_8_ = local_28._0_8_;
  auVar4._8_8_ = local_28._8_8_;
  auVar4._0_8_ = local_28._0_8_;
  auVar12._8_8_ = local_28._8_8_;
  auVar12._0_8_ = local_28._0_8_;
  if (DAT_0012b338 == 0) {
    if (((param_1 - iVar8 < 2) && (local_28 = auVar12, DAT_0012b368 != (long *)0x0)) &&
       (local_28 = auVar4, DAT_0012b368[3] == 0)) {
      DAT_0012b428 = 0;
      local_28 = auVar5;
    }
  }
  else {
    FUN_0010c12d();
    if (DAT_0012b368 != (long *)0x0) {
      FUN_00105422(10);
    }
  }
  while (plVar6 = DAT_0012b368, DAT_0012b368 != (long *)0x0) {
    if ((DAT_0012b320 == 0) || (*DAT_0012b368 != 0)) {
      plVar2 = DAT_0012b368 + 2;
      plVar3 = DAT_0012b368 + 1;
      lVar11 = *DAT_0012b368;
      DAT_0012b368 = (long *)DAT_0012b368[3];
      FUN_00108ede(lVar11,*plVar3,(char)*plVar2);
      FUN_001062e7(plVar6);
      DAT_0012b428 = 1;
    }
    else {
      DAT_0012b368 = (long *)DAT_0012b368[3];
      auVar12 = FUN_0010563d();
      local_28 = auVar12;
      lVar11 = FUN_00113740(DAT_0012b320,local_28);
      if (lVar11 == 0) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("found","src/ls.c",0x70d,"main");
      }
      FUN_00106243(lVar11);
      FUN_001062e7(plVar6);
    }
  }
  if ((DAT_0012b3d4 == '\0') || (DAT_0012b3d6 == '\0')) goto LAB_00106efb;
  if (DAT_0012b080 == 2) {
    iVar8 = memcmp(PTR_DAT_0012b088,&DAT_00120638,2);
    if (((iVar8 != 0) || (DAT_0012b090 != 1)) || (*PTR_DAT_0012b098 != 'm')) goto LAB_00106eaa;
  }
  else {
LAB_00106eaa:
    FUN_001063ca();
  }
  fflush_unlocked(stdout);
  FUN_0010686d();
  for (local_40 = DAT_0012b4e4; local_40 != 0; local_40 = local_40 + -1) {
    raise(0x13);
  }
  if (DAT_0012b4e0 != 0) {
    raise(DAT_0012b4e0);
  }
LAB_00106efb:
  if (DAT_0012b3cc != '\0') {
    FUN_0010581f("//DIRED//",&DAT_0012b520);
    FUN_0010581f("//SUBDIRED//",&DAT_0012b580);
    uVar9 = FUN_00118fc3(DAT_0012b410);
    printf("//DIRED-OPTIONS// --quoting-style=%s\n",(&PTR_s_literal_0012a9e0)[uVar9]);
  }
  if (DAT_0012b320 != 0) {
    lVar11 = FUN_00112002(DAT_0012b320);
    if (lVar11 != 0) {
                    /* WARNING: Subroutine does not return */
      __assert_fail("hash_get_n_entries (active_dir_set) == 0","src/ls.c",0x741,"main");
    }
    FUN_00112c0f(DAT_0012b320);
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return DAT_0012b4e8;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: decode_line_length @ 0x6fe6

long decode_line_length(undefined8 param_1)

{
  int iVar1;
  long in_FS_OFFSET;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_0011d251(param_1,0,0,&local_18,&DAT_0011f27b);
  if (iVar1 == 0) {
    if (local_18 < 0) {
      local_18 = 0;
    }
  }
  else if (iVar1 == 1) {
    local_18 = 0;
  }
  else {
    local_18 = -1;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_18;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: stdout_isatty @ 0x706a

bool stdout_isatty(void)

{
  int iVar1;
  
  if (DAT_0012b031 < '\0') {
    iVar1 = isatty(1);
    DAT_0012b031 = (char)iVar1;
  }
  return DAT_0012b031 != '\0';
}




// Function: decode_switches @ 0x70af

undefined4 decode_switches(undefined4 param_1,undefined8 param_2)

{
  bool bVar1;
  FILE *pFVar2;
  char cVar3;
  int iVar4;
  long *plVar5;
  char *pcVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  char *pcVar9;
  long lVar10;
  undefined *puVar11;
  long in_FS_OFFSET;
  int local_c4;
  int local_c0;
  int local_bc;
  int local_b8;
  int local_b4;
  int local_b0;
  int local_ac;
  int local_a8;
  int local_a4;
  char *local_98;
  long local_90;
  ulong local_88;
  ulong local_80;
  char *local_78;
  char *local_70;
  char *local_68;
  undefined **local_60;
  undefined2 local_28;
  ushort uStack_26;
  undefined4 uStack_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_98 = (char *)0x0;
  bVar1 = false;
  local_c4 = -1;
  local_c0 = -1;
  local_bc = -1;
  local_b8 = -1;
  local_90 = -1;
  local_88 = 0xffffffffffffffff;
LAB_00107128:
  local_28 = 0xffff;
  uStack_26 = 0xffff;
  iVar4 = getopt_long(param_1,param_2,"abcdfghiklmnopqrstuvw:xABCDFGHI:LNQRST:UXZ1",
                      &PTR_DAT_0012a0c0,&local_28);
  if (iVar4 != -1) {
    switch(iVar4) {
    case 0x31:
      if (local_c4 != 0) {
        local_c4 = 1;
      }
      break;
    case 0x41:
      DAT_0012b3f4 = 1;
      break;
    case 0x42:
      FUN_001094e9(&DAT_00120b3e);
      FUN_001094e9(&DAT_00120b41);
      break;
    case 0x43:
      local_c4 = 2;
      break;
    case 0x44:
      DAT_0012b3cc = 1;
      break;
    case 0x46:
      if (optarg == (char *)0x0) {
        local_b4 = 1;
      }
      else {
        lVar10 = FUN_0010fe17("--classify",optarg,&PTR_s_always_0012a720,&DAT_00120980,4,
                              PTR_FUN_0012b220,1);
        local_b4 = *(int *)(&DAT_00120980 + lVar10 * 4);
      }
      if ((local_b4 == 1) || ((local_b4 == 2 && (cVar3 = FUN_0010706a(), cVar3 != '\0')))) {
        DAT_0012b3d0 = 3;
      }
      break;
    case 0x47:
      DAT_0012b023 = 0;
      break;
    case 0x48:
      DAT_0012b3ec = 2;
      break;
    case 0x49:
      FUN_001094e9(optarg);
      break;
    case 0x4c:
      DAT_0012b3ec = 4;
      break;
    case 0x4e:
      local_bc = 0;
      break;
    case 0x51:
      local_bc = 5;
      break;
    case 0x52:
      DAT_0012b3f0 = 1;
      break;
    case 0x53:
      local_b8 = 3;
      break;
    case 0x54:
      uVar7 = gettext("invalid tab size");
      local_90 = FUN_0011cd03(optarg,0,0,0x7fffffffffffffff,&DAT_0011f27b,uVar7,2);
      break;
    case 0x55:
      local_b8 = 6;
      break;
    case 0x58:
      local_b8 = 1;
      break;
    case 0x5a:
      DAT_0012b380 = 1;
      break;
    case 0x61:
      DAT_0012b3f4 = 2;
      break;
    case 0x62:
      local_bc = 7;
      break;
    case 99:
      DAT_0012b3b0 = 1;
      break;
    case 100:
      DAT_0012b3f1 = 1;
      break;
    case 0x66:
      DAT_0012b3f4 = 2;
      local_b8 = 6;
      if (local_c4 == 0) {
        local_c4 = -1;
      }
      DAT_0012b3d4 = '\0';
      DAT_0012b3d5 = 0;
      DAT_0012b3bb = 0;
      break;
    case 0x67:
      local_c4 = 0;
      DAT_0012b022 = 0;
      break;
    case 0x68:
      DAT_0012b3bc = 0xb0;
      DAT_0012b3c8 = 0xb0;
      DAT_0012b3c0 = 1;
      DAT_0012b028 = 1;
      break;
    case 0x69:
      DAT_0012b3e9 = 1;
      break;
    case 0x6b:
      bVar1 = true;
      break;
    case 0x6c:
      local_c4 = 0;
      break;
    case 0x6d:
      local_c4 = 4;
      break;
    case 0x6e:
      DAT_0012b3ba = 1;
      local_c4 = 0;
      break;
    case 0x6f:
      local_c4 = 0;
      DAT_0012b023 = 0;
      break;
    case 0x70:
      DAT_0012b3d0 = 1;
      break;
    case 0x71:
      local_c0 = 1;
      break;
    case 0x72:
      DAT_0012b3b8 = 1;
      break;
    case 0x73:
      DAT_0012b3bb = 1;
      break;
    case 0x74:
      local_b8 = 5;
      break;
    case 0x75:
      DAT_0012b3b0 = 2;
      break;
    case 0x76:
      local_b8 = 4;
      break;
    case 0x77:
      local_88 = FUN_00106fe6(optarg);
      if (-1 < (long)local_88) break;
      uVar7 = FUN_0011aeca(optarg);
      uVar8 = gettext("invalid line width");
      error(2,0,"%s: %s",uVar8,uVar7);
    case 0x78:
      local_c4 = 3;
      break;
    case 0x80:
      DAT_0012b3b9 = 1;
      break;
    case 0x81:
      iVar4 = FUN_001149fd(optarg,&DAT_0012b3bc,&DAT_0012b3c0);
      if (iVar4 != 0) {
        FUN_0011d130(iVar4,CONCAT22(uStack_26,local_28),0,&PTR_DAT_0012a0c0,optarg);
      }
      DAT_0012b3c8 = DAT_0012b3bc;
      DAT_0012b028 = DAT_0012b3c0;
      break;
    case 0x82:
      if (optarg == (char *)0x0) {
        local_b0 = 1;
      }
      else {
        lVar10 = FUN_0010fe17("--color",optarg,&PTR_s_always_0012a720,&DAT_00120980,4,
                              PTR_FUN_0012b220,1);
        local_b0 = *(int *)(&DAT_00120980 + lVar10 * 4);
      }
      if ((local_b0 == 1) || ((local_b0 == 2 && (cVar3 = FUN_0010706a(), cVar3 != '\0')))) {
        DAT_0012b3d4 = '\x01';
      }
      else {
        DAT_0012b3d4 = '\0';
      }
      break;
    case 0x83:
      DAT_0012b3ec = 3;
      break;
    case 0x84:
      DAT_0012b3d0 = 2;
      break;
    case 0x85:
      lVar10 = FUN_0010fe17("--format",optarg,&PTR_s_verbose_0012a660,&DAT_001208a0,4,
                            PTR_FUN_0012b220,1);
      local_c4 = *(int *)(&DAT_001208a0 + lVar10 * 4);
      break;
    case 0x86:
      local_c4 = 0;
      local_98 = (char *)FUN_0010523e("full-iso");
      break;
    case 0x87:
      DAT_0012b3f2 = 1;
      break;
    case 0x88:
      plVar5 = (long *)FUN_0011c68b(0x10);
      *plVar5 = (long)optarg;
      plVar5[1] = (long)DAT_0012b400;
      DAT_0012b400 = plVar5;
      break;
    case 0x89:
      if (optarg == (char *)0x0) {
        local_ac = 1;
      }
      else {
        lVar10 = FUN_0010fe17("--hyperlink",optarg,&PTR_s_always_0012a720,&DAT_00120980,4,
                              PTR_FUN_0012b220,1);
        local_ac = *(int *)(&DAT_00120980 + lVar10 * 4);
      }
      if ((local_ac == 1) || ((local_ac == 2 && (cVar3 = FUN_0010706a(), cVar3 != '\0')))) {
        DAT_0012b3d5 = 1;
      }
      else {
        DAT_0012b3d5 = 0;
      }
      break;
    case 0x8a:
      lVar10 = FUN_0010fe17("--indicator-style",optarg,&PTR_DAT_00129fa0,&DAT_001205e0,4,
                            PTR_FUN_0012b220,1);
      DAT_0012b3d0 = *(uint *)(&DAT_001205e0 + lVar10 * 4);
      break;
    case 0x8b:
      lVar10 = FUN_0010fe17("--quoting-style",optarg,&PTR_s_literal_0012a9e0,&DAT_001238c0,4,
                            PTR_FUN_0012b220,1);
      local_bc = *(int *)(&DAT_001238c0 + lVar10 * 4);
      break;
    case 0x8c:
      local_c0 = 0;
      break;
    case 0x8d:
      DAT_0012b3bc = 0x90;
      DAT_0012b3c8 = 0x90;
      DAT_0012b3c0 = 1;
      DAT_0012b028 = 1;
      break;
    case 0x8e:
      lVar10 = FUN_0010fe17("--sort",optarg,&PTR_DAT_0012a6a0,&DAT_001208d0,4,PTR_FUN_0012b220,1);
      local_b8 = *(int *)(&DAT_001208d0 + lVar10 * 4);
      break;
    case 0x8f:
      lVar10 = FUN_0010fe17("--time",optarg,&PTR_s_atime_0012a6e0,&DAT_00120920,4,PTR_FUN_0012b220,1
                           );
      DAT_0012b3b0 = *(int *)(&DAT_00120920 + lVar10 * 4);
      break;
    case 0x90:
      local_98 = optarg;
      break;
    case 0x91:
      goto switchD_001071b5_caseD_91;
    case -0x82:
      FUN_0010f135(0);
    case -0x83:
      if (DAT_0012b210 == 1) {
        puVar11 = &DAT_00120bb7;
      }
      else if (DAT_0012b210 == 2) {
        puVar11 = &DAT_00120bae;
      }
      else {
        puVar11 = &DAT_00120bb2;
      }
      FUN_0011c4ab(stdout,puVar11,"GNU coreutils",PTR_DAT_0012b218,"Richard M. Stallman",
                   "David MacKenzie",0);
                    /* WARNING: Subroutine does not return */
      exit(0);
    default:
      FUN_0010f135(2);
    }
    goto LAB_00107128;
  }
  if (DAT_0012b3c0 == 0) {
    pcVar6 = getenv("LS_BLOCK_SIZE");
    FUN_001149fd(pcVar6,&DAT_0012b3bc,&DAT_0012b3c0);
    if ((pcVar6 != (char *)0x0) || (pcVar6 = getenv("BLOCK_SIZE"), pcVar6 != (char *)0x0)) {
      DAT_0012b3c8 = DAT_0012b3bc;
      DAT_0012b028 = DAT_0012b3c0;
    }
    if (bVar1) {
      DAT_0012b3bc = 0;
      DAT_0012b3c0 = 0x400;
    }
  }
  if (local_c4 < 0) {
    if (DAT_0012b210 == 1) {
      cVar3 = FUN_0010706a();
      if (cVar3 == '\0') {
        local_c4 = 1;
      }
      else {
        local_c4 = 2;
      }
    }
    else if (DAT_0012b210 == 2) {
      local_c4 = 2;
    }
    else {
      local_c4 = 0;
    }
  }
  local_80 = local_88;
  DAT_0012b3ac = local_c4;
  if ((((local_c4 == 2) || (local_c4 == 3)) || (local_c4 == 4)) || (DAT_0012b3d4 != '\0')) {
    if ((((long)local_88 < 0) && (cVar3 = FUN_0010706a(), cVar3 != '\0')) &&
       ((iVar4 = ioctl(1,0x5413,&local_28), -1 < iVar4 && (uStack_26 != 0)))) {
      local_80 = (ulong)uStack_26;
    }
    if ((((long)local_80 < 0) && (pcVar6 = getenv("COLUMNS"), pcVar6 != (char *)0x0)) &&
       ((*pcVar6 != '\0' && (local_80 = FUN_00106fe6(pcVar6), (long)local_80 < 0)))) {
      uVar7 = FUN_0011aeca(pcVar6);
      uVar8 = gettext("ignoring invalid width in environment variable COLUMNS: %s");
      error(0,0,uVar8,uVar7);
    }
  }
  if ((long)local_80 < 0) {
    local_80 = 0x50;
  }
  DAT_0012b4f8 = local_80 / 3 + (ulong)(local_80 % 3 != 0);
  DAT_0012b430 = local_80;
  if (((DAT_0012b3ac == 2) || (DAT_0012b3ac == 3)) || (DAT_0012b3ac == 4)) {
    if (local_90 < 0) {
      DAT_0012b420 = 8;
      pcVar6 = getenv("TABSIZE");
      if (pcVar6 != (char *)0x0) {
        iVar4 = FUN_0011d251(pcVar6,0,0,&local_28,&DAT_0011f27b);
        if (iVar4 == 0) {
          DAT_0012b420 = CONCAT44(uStack_24,CONCAT22(uStack_26,local_28));
        }
        else {
          uVar7 = FUN_0011aeca(pcVar6);
          uVar8 = gettext("ignoring invalid tab size in environment variable TABSIZE: %s");
          error(0,0,uVar8,uVar7);
        }
      }
    }
    else {
      DAT_0012b420 = local_90;
    }
  }
  if (local_c0 < 0) {
    if ((DAT_0012b210 == 1) && (cVar3 = FUN_0010706a(), cVar3 != '\0')) {
      DAT_0012b408 = true;
    }
    else {
      DAT_0012b408 = false;
    }
  }
  else {
    DAT_0012b408 = local_c0 != 0;
  }
  local_a8 = local_bc;
  if (local_bc < 0) {
    local_a8 = FUN_00108cff();
  }
  if (local_a8 < 0) {
    if (DAT_0012b210 == 1) {
      cVar3 = FUN_0010706a();
      if (cVar3 == '\0') {
        local_a8 = -1;
      }
      else {
        local_a8 = 3;
      }
    }
    else {
      local_a8 = 7;
    }
  }
  if (-1 < local_a8) {
    FUN_00118fe7(0,local_a8);
  }
  iVar4 = FUN_00118fc3(0);
  if (((DAT_0012b3ac == 0) || (((DAT_0012b3ac == 2 || (DAT_0012b3ac == 3)) && (DAT_0012b430 != 0))))
     && (((iVar4 == 1 || (iVar4 == 3)) || (iVar4 == 6)))) {
    DAT_0012b341 = 1;
  }
  else {
    DAT_0012b341 = 0;
  }
  DAT_0012b410 = FUN_00118f74(0);
  if (iVar4 == 7) {
    FUN_00119012(DAT_0012b410,0x20,1);
  }
  if (1 < DAT_0012b3d0) {
    for (local_78 = &DAT_00120c86 + (DAT_0012b3d0 - 2); *local_78 != '\0'; local_78 = local_78 + 1)
    {
      FUN_00119012(DAT_0012b410,(int)*local_78,1);
    }
  }
  DAT_0012b418 = FUN_00118f74(0);
  FUN_00119012(DAT_0012b418,0x3a,1);
  DAT_0012b3cc = ((DAT_0012b3d5 ^ 1) & DAT_0012b3ac == 0 & DAT_0012b3cc) != 0;
  if ((int)DAT_0012b030 < (int)(uint)DAT_0012b3cc) {
    uVar7 = gettext("--dired and --zero are incompatible");
    error(2,0,uVar7);
  }
  if (local_b8 < 0) {
    if ((DAT_0012b3ac == 0) || (((DAT_0012b3b0 != 1 && (DAT_0012b3b0 != 2)) && (DAT_0012b3b0 != 3)))
       ) {
      local_b8 = 0;
    }
    else {
      local_b8 = 5;
    }
  }
  DAT_0012b3b4 = local_b8;
  if (DAT_0012b3ac == 0) {
    local_70 = local_98;
    if ((local_98 == (char *)0x0) && (local_70 = getenv("TIME_STYLE"), local_70 == (char *)0x0)) {
      local_70 = (char *)FUN_0010523e("locale");
    }
    while (iVar4 = strncmp(local_70,"posix-",6), iVar4 == 0) {
      cVar3 = FUN_00111f2c(2);
      if (cVar3 != '\x01') goto LAB_00108415;
      local_70 = local_70 + 6;
    }
    if (*local_70 == '+') {
      local_70 = local_70 + 1;
      pcVar6 = strchr(local_70,10);
      local_68 = local_70;
      if (pcVar6 != (char *)0x0) {
        pcVar9 = strchr(pcVar6 + 1,10);
        if (pcVar9 != (char *)0x0) {
          uVar7 = FUN_0011aeca(local_70);
          uVar8 = gettext("invalid time style format %s");
          error(2,0,uVar8,uVar7);
        }
        *pcVar6 = '\0';
        local_68 = pcVar6 + 1;
      }
      PTR_s__b__e__H__M_0012b208 = local_68;
      PTR_s__b__e__Y_0012b200 = local_70;
    }
    else {
      lVar10 = FUN_0010fa7e(local_70,&PTR_s_full_iso_00129f60,&DAT_001205b0,4);
      if (lVar10 < 0) {
        FUN_0010fc27("time style",local_70,lVar10);
        pFVar2 = stderr;
        pcVar6 = (char *)gettext("Valid arguments are:\n");
        fputs_unlocked(pcVar6,pFVar2);
        local_60 = &PTR_s_full_iso_00129f60;
        while (pFVar2 = stderr, *local_60 != (undefined *)0x0) {
          fprintf(stderr,"  - [posix-]%s\n",*local_60);
          local_60 = local_60 + 1;
        }
        pcVar6 = (char *)gettext("  - +FORMAT (e.g., +%H:%M) for a \'date\'-style format\n");
        fputs_unlocked(pcVar6,pFVar2);
        FUN_0010f135(2);
      }
      if (lVar10 == 3) {
        cVar3 = FUN_00111f2c(2);
        if (cVar3 != '\0') {
          for (local_a4 = 0; local_a4 < 2; local_a4 = local_a4 + 1) {
            puVar11 = (undefined *)dcgettext(0,(&PTR_s__b__e__Y_0012b200)[local_a4],2);
            (&PTR_s__b__e__Y_0012b200)[local_a4] = puVar11;
          }
        }
      }
      else if (lVar10 < 4) {
        if (lVar10 == 2) {
          PTR_s__b__e__Y_0012b200 = s__Y__m__d_00120d6d;
          PTR_s__b__e__H__M_0012b208 = s__m__d__H__M_00120d77;
        }
        else if (lVar10 < 3) {
          if (lVar10 == 0) {
            PTR_s__b__e__H__M_0012b208 = s__Y__m__d__H__M__S__N__z_00120d46;
            PTR_s__b__e__Y_0012b200 = s__Y__m__d__H__M__S__N__z_00120d46;
          }
          else if (lVar10 == 1) {
            PTR_s__b__e__H__M_0012b208 = s__Y__m__d__H__M_00120d5e;
            PTR_s__b__e__Y_0012b200 = s__Y__m__d__H__M_00120d5e;
          }
        }
      }
    }
    FUN_00105f43();
  }
LAB_00108415:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return optind;
switchD_001071b5_caseD_91:
  DAT_0012b030 = '\0';
  local_c0 = 0;
  if (local_c4 != 0) {
    local_c4 = 1;
  }
  DAT_0012b3d4 = '\0';
  local_bc = 0;
  goto LAB_00107128;
}




// Function: get_funky_string @ 0x842f

/* WARNING: Removing unreachable block (ram,0x001087b3) */
/* WARNING: Removing unreachable block (ram,0x001087c2) */

undefined8 get_funky_string(void)

{
  undefined8 uVar1;
  
                    /* WARNING: Could not recover jumptable at 0x001084a5. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  uVar1 = (*(code *)((long)&switchD_001084a5::switchdataD_001211d8 +
                    (long)(int)switchD_001084a5::switchdataD_001211d8))();
  return uVar1;
}




// Function: known_term_type @ 0x87ec

undefined8 known_term_type(void)

{
  int iVar1;
  char *__name;
  size_t sVar2;
  char *local_18;
  
  __name = getenv("TERM");
  if ((__name != (char *)0x0) && (*__name != '\0')) {
    for (local_18 = "# Configuration file for dircolors, a utility to help you set the";
        local_18 + -0x11f2c0 < (char *)0x12c8; local_18 = local_18 + sVar2 + 1) {
      iVar1 = strncmp(local_18,"TERM ",5);
      if ((iVar1 == 0) && (iVar1 = fnmatch(local_18 + 5,__name,0), iVar1 == 0)) {
        return 1;
      }
      sVar2 = strlen(local_18);
    }
  }
  return 0;
}




// Function: parse_ls_color @ 0x88ab

/* WARNING: Removing unreachable block (ram,0x00108c2a) */

void parse_ls_color(void)

{
  char cVar1;
  char *pcVar2;
  long in_FS_OFFSET;
  char local_23 [3];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  pcVar2 = getenv("LS_COLORS");
  if ((pcVar2 != (char *)0x0) && (*pcVar2 != '\0')) {
    strcpy(local_23,"??");
    DAT_0012b3e0 = FUN_0011cc92(pcVar2);
                    /* WARNING: Could not recover jumptable at 0x001089a1. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)((long)&switchD_001089a1::switchdataD_0012138c + (long)(int)UINT_00121390))();
    return;
  }
  pcVar2 = getenv("COLORTERM");
  if (((pcVar2 == (char *)0x0) || (*pcVar2 == '\0')) && (cVar1 = FUN_001087ec(), cVar1 != '\x01')) {
    DAT_0012b3d4 = 0;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: getenv_quoting_style @ 0x8cff

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
    iVar1 = FUN_0010fa7e(pcVar3,&PTR_s_literal_0012a9e0,&DAT_001238c0,4);
    if (iVar1 < 0) {
      uVar4 = FUN_0011aeca(pcVar3);
      uVar5 = gettext("ignoring invalid value of environment variable QUOTING_STYLE: %s");
      error(0,0,uVar5,uVar4);
      uVar2 = 0xffffffff;
    }
    else {
      uVar2 = *(undefined4 *)(&DAT_001238c0 + (long)iVar1 * 4);
    }
  }
  return uVar2;
}




// Function: set_exit_status @ 0x8db1

void set_exit_status(char param_1)

{
  if (param_1 == '\0') {
    if (DAT_0012b4e8 == 0) {
      DAT_0012b4e8 = 1;
    }
  }
  else {
    DAT_0012b4e8 = 2;
  }
  return;
}




// Function: file_failure @ 0x8de7

void file_failure(undefined1 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 uVar1;
  int *piVar2;
  
  uVar1 = FUN_0011aa83(4,param_3);
  piVar2 = __errno_location();
  error(0,*piVar2,param_2,uVar1);
  FUN_00108db1(param_1);
  return;
}




// Function: queue_directory @ 0x8e46

void queue_directory(long param_1,long param_2,undefined1 param_3)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  
  puVar1 = (undefined8 *)FUN_0011c68b(0x20);
  if (param_2 == 0) {
    uVar2 = 0;
  }
  else {
    uVar2 = FUN_0011cc92(param_2);
  }
  puVar1[1] = uVar2;
  if (param_1 == 0) {
    uVar2 = 0;
  }
  else {
    uVar2 = FUN_0011cc92(param_1);
  }
  *puVar1 = uVar2;
  *(undefined1 *)(puVar1 + 2) = param_3;
  puVar1[3] = DAT_0012b368;
  DAT_0012b368 = puVar1;
  return;
}




// Function: print_dir @ 0x8ede

void print_dir(char *param_1,char *param_2,undefined1 param_3)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  DIR *__dirp;
  undefined8 uVar4;
  char extraout_var;
  char extraout_var_00;
  undefined8 uVar5;
  char *pcVar6;
  dirent *pdVar7;
  long lVar8;
  char *__s;
  size_t sVar9;
  long in_FS_OFFSET;
  undefined4 local_380;
  long local_378;
  void *local_370;
  undefined8 local_348;
  undefined8 local_340;
  undefined1 auStack_2b7 [663];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_378 = 0;
  piVar3 = __errno_location();
  *piVar3 = 0;
  __dirp = opendir(param_1);
  if (__dirp == (DIR *)0x0) {
    uVar4 = gettext("cannot open directory %s");
    FUN_00108de7(param_3,uVar4,param_1);
  }
  else {
    if (DAT_0012b320 == 0) {
LAB_001090b9:
      FUN_001096f3();
      if ((DAT_0012b3f0 != '\0') || (DAT_0012b428 != '\0')) {
        if (DAT_0012b032 != '\x01') {
          FUN_00105422(10);
        }
        DAT_0012b032 = '\0';
        FUN_001054cf();
        local_370 = (void *)0x0;
        if ((DAT_0012b3d5 != '\0') &&
           (local_370 = (void *)FUN_0011101b(param_1,2), local_370 == (void *)0x0)) {
          uVar4 = gettext("error canonicalizing %s");
          FUN_00108de7(param_3,uVar4,param_1);
        }
        pcVar6 = param_1;
        if (param_2 != (char *)0x0) {
          pcVar6 = param_2;
        }
        FUN_0010d87f(pcVar6,DAT_0012b418,0xffffffff,0,1,&DAT_0012b580,local_370);
        free(local_370);
        FUN_0010549e(&DAT_00121481);
      }
      do {
        piVar3 = __errno_location();
        *piVar3 = 0;
        pdVar7 = readdir(__dirp);
        if (pdVar7 == (dirent *)0x0) {
          piVar3 = __errno_location();
          if (*piVar3 == 0) goto LAB_00109382;
          uVar4 = gettext("reading directory %s");
          FUN_00108de7(param_3,uVar4,param_1);
          piVar3 = __errno_location();
          if (*piVar3 != 0x4b) goto LAB_00109382;
        }
        else {
          cVar1 = FUN_0010958d(pdVar7->d_name);
          if (cVar1 != '\x01') {
            local_380 = 0;
            switch(pdVar7->d_type) {
            case '\x01':
              local_380 = 1;
              break;
            case '\x02':
              local_380 = 2;
              break;
            case '\x04':
              local_380 = 3;
              break;
            case '\x06':
              local_380 = 4;
              break;
            case '\b':
              local_380 = 5;
              break;
            case '\n':
              local_380 = 6;
              break;
            case '\f':
              local_380 = 7;
              break;
            case '\x0e':
              local_380 = 8;
            }
            lVar8 = FUN_00109a40(pdVar7->d_name,local_380,0,0,param_1);
            local_378 = local_378 + lVar8;
            if ((((DAT_0012b3ac == 1) && (DAT_0012b3b4 == 6)) && (DAT_0012b3bb != '\x01')) &&
               (DAT_0012b3f0 != '\x01')) {
              FUN_0010bfc5();
              FUN_0010c12d();
              FUN_001096f3();
            }
          }
        }
        FUN_0010648c();
      } while( true );
    }
    iVar2 = dirfd(__dirp);
    if (iVar2 < 0) {
      FUN_00105cb8(param_1,&local_348);
      cVar1 = extraout_var_00;
    }
    else {
      FUN_00105cee(iVar2,&local_348);
      cVar1 = extraout_var;
    }
    if (cVar1 < '\0') {
      uVar4 = gettext("cannot determine device and inode of %s");
      FUN_00108de7(param_3,uVar4,param_1);
      closedir(__dirp);
    }
    else {
      cVar1 = FUN_00106262(local_348,local_340);
      if (cVar1 == '\0') {
        FUN_0010558e(local_348,local_340);
        goto LAB_001090b9;
      }
      uVar4 = FUN_0011ac15(0,3,param_1);
      uVar5 = gettext("%s: not listing already-listed directory");
      error(0,0,uVar5,uVar4);
      closedir(__dirp);
      FUN_00108db1(1);
    }
  }
  goto LAB_001094cf;
LAB_00109382:
  iVar2 = closedir(__dirp);
  if (iVar2 != 0) {
    uVar4 = gettext("closing directory %s");
    FUN_00108de7(param_3,uVar4,param_1);
  }
  FUN_0010bfc5();
  if (DAT_0012b3f0 != '\0') {
    FUN_0010aa65(param_1,0);
  }
  if ((DAT_0012b3ac == 0) || (DAT_0012b3bb != '\0')) {
    __s = (char *)FUN_00113d91(local_378,auStack_2b7,DAT_0012b3bc,0x200,DAT_0012b3c0);
    sVar9 = strlen(__s);
    pcVar6 = __s + -1;
    *pcVar6 = ' ';
    __s[sVar9] = DAT_0012b030;
    FUN_001054cf();
    uVar4 = gettext("total");
    FUN_0010549e(uVar4);
    FUN_00105453(pcVar6,__s + sVar9 + (1 - (long)pcVar6));
  }
  if (DAT_0012b338 != 0) {
    FUN_0010c12d();
  }
LAB_001094cf:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: add_ignore_pattern @ 0x94e9

void add_ignore_pattern(undefined8 param_1)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_0011c68b(0x10);
  *puVar1 = param_1;
  puVar1[1] = DAT_0012b3f8;
  DAT_0012b3f8 = puVar1;
  return;
}




// Function: patterns_match @ 0x952f

undefined8 patterns_match(undefined8 *param_1,char *param_2)

{
  int iVar1;
  undefined8 *local_10;
  
  local_10 = param_1;
  while( true ) {
    if (local_10 == (undefined8 *)0x0) {
      return 0;
    }
    iVar1 = fnmatch((char *)*local_10,param_2,4);
    if (iVar1 == 0) break;
    local_10 = (undefined8 *)local_10[1];
  }
  return 1;
}




// Function: file_ignored @ 0x958d

undefined4 file_ignored(char *param_1)

{
  char cVar1;
  long lVar2;
  
  if ((DAT_0012b3f4 != 2) && (*param_1 == '.')) {
    if (DAT_0012b3f4 == 0) {
      return 1;
    }
    if (param_1[1] == '.') {
      lVar2 = 2;
    }
    else {
      lVar2 = 1;
    }
    if (param_1[lVar2] == '\0') {
      return 1;
    }
  }
  if (((DAT_0012b3f4 != 0) || (cVar1 = FUN_0010952f(DAT_0012b400,param_1), cVar1 == '\0')) &&
     (cVar1 = FUN_0010952f(DAT_0012b3f8,param_1), cVar1 == '\0')) {
    return 0;
  }
  return 1;
}




// Function: unsigned_file_size @ 0x9635

undefined8 unsigned_file_size(undefined8 param_1)

{
  return param_1;
}




// Function: has_capability @ 0x9647

undefined8 has_capability(void)

{
  int *piVar1;
  
  piVar1 = __errno_location();
  *piVar1 = 0x5f;
  return 0;
}




// Function: free_ent @ 0x9669

void free_ent(undefined8 *param_1)

{
  char cVar1;
  
  free((void *)*param_1);
  free((void *)param_1[1]);
  free((void *)param_1[2]);
  if ((undefined *)param_1[0x16] != &DAT_0012b020) {
    cVar1 = FUN_0010525c();
    if (cVar1 == '\0') {
      freecon(param_1[0x16]);
    }
    else {
      free((void *)param_1[0x16]);
    }
  }
  return;
}




// Function: clear_files @ 0x96f3

void clear_files(void)

{
  undefined8 local_18;
  
  for (local_18 = 0; local_18 < DAT_0012b338; local_18 = local_18 + 1) {
    FUN_00109669(*(undefined8 *)(local_18 * 8 + DAT_0012b348));
  }
  DAT_0012b338 = 0;
  DAT_0012b340 = 0;
  DAT_0012b381 = 0;
  DAT_0012b384 = 0;
  DAT_0012b388 = 0;
  DAT_0012b38c = 0;
  DAT_0012b394 = 0;
  DAT_0012b398 = 0;
  DAT_0012b39c = 0;
  DAT_0012b390 = 0;
  DAT_0012b3a0 = 0;
  DAT_0012b3a4 = 0;
  DAT_0012b3a8 = 0;
  return;
}




// Function: errno_unsupported @ 0x97c0

undefined4 errno_unsupported(int param_1)

{
  char cVar1;
  
  if (((param_1 != 0x16) && (param_1 != 0x26)) && (cVar1 = FUN_0010524c(param_1), cVar1 == '\0')) {
    return 0;
  }
  return 1;
}




// Function: getfilecon_cache @ 0x97fa

int getfilecon_cache(undefined8 param_1,long param_2,char param_3)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  
  if (*(long *)(param_2 + 0x18) == DAT_0012c428) {
    piVar3 = __errno_location();
    *piVar3 = 0x5f;
    iVar2 = -1;
  }
  else {
    if (param_3 == '\0') {
      iVar2 = FUN_0011b268(param_1,param_2 + 0xb0);
    }
    else {
      iVar2 = FUN_0011b22b(param_1,param_2 + 0xb0);
    }
    if (iVar2 < 0) {
      piVar3 = __errno_location();
      cVar1 = FUN_001097c0(*piVar3);
      if (cVar1 != '\0') {
        DAT_0012c428 = *(long *)(param_2 + 0x18);
      }
    }
  }
  return iVar2;
}




// Function: file_has_acl_cache @ 0x98ab

int file_has_acl_cache(undefined8 param_1,long param_2)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  
  if (*(long *)(param_2 + 0x18) == DAT_0012c430) {
    piVar3 = __errno_location();
    *piVar3 = 0x5f;
    iVar2 = 0;
  }
  else {
    piVar3 = __errno_location();
    *piVar3 = 0;
    iVar2 = FUN_001112c6(param_1,param_2 + 0x18);
    if (iVar2 < 1) {
      piVar3 = __errno_location();
      cVar1 = FUN_001097c0(*piVar3);
      if (cVar1 != '\0') {
        DAT_0012c430 = *(long *)(param_2 + 0x18);
      }
    }
  }
  return iVar2;
}




// Function: has_capability_cache @ 0x9936

char has_capability_cache(undefined8 param_1,long param_2)

{
  char cVar1;
  char cVar2;
  int *piVar3;
  
  if (*(long *)(param_2 + 0x18) == DAT_0012c438) {
    piVar3 = __errno_location();
    *piVar3 = 0x5f;
    cVar1 = '\0';
  }
  else {
    cVar1 = FUN_00109647(param_1);
    if (cVar1 != '\x01') {
      piVar3 = __errno_location();
      cVar2 = FUN_001097c0(*piVar3);
      if (cVar2 != '\0') {
        DAT_0012c438 = *(long *)(param_2 + 0x18);
      }
    }
  }
  return cVar1;
}




// Function: needs_quoting @ 0x99b1

undefined4 needs_quoting(char *param_1)

{
  undefined4 uVar1;
  size_t sVar2;
  size_t sVar3;
  long in_FS_OFFSET;
  char local_12 [2];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  sVar2 = FUN_0011a389(local_12,2,param_1,0xffffffffffffffff,DAT_0012b410);
  if (*param_1 == local_12[0]) {
    sVar3 = strlen(param_1);
    if (sVar2 == sVar3) {
      uVar1 = 0;
      goto LAB_00109a27;
    }
  }
  uVar1 = 1;
LAB_00109a27:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar1;
}




// Function: gobble_file @ 0x9a40

undefined8 gobble_file(char *param_1,int param_2,long param_3,char param_4,char *param_5)

{
  undefined8 *puVar1;
  ulonglong uVar2;
  long lVar3;
  char *pcVar4;
  void *pvVar5;
  byte bVar6;
  char cVar7;
  undefined1 uVar8;
  int iVar9;
  undefined4 uVar10;
  uint uVar11;
  size_t sVar12;
  ulong uVar13;
  undefined8 uVar14;
  int *piVar15;
  size_t sVar16;
  char *pcVar17;
  char **ppcVar18;
  char **ppcVar19;
  long in_FS_OFFSET;
  char *local_3e8;
  long local_3e0;
  char local_3d8;
  int local_3d4;
  char *local_3d0;
  undefined1 local_3c4;
  char local_3c3;
  char local_3c2;
  char local_3c1;
  uint local_3c0;
  int local_3bc;
  uint local_3b8;
  int local_3b4;
  int local_3b0;
  int local_3ac;
  int local_3a8;
  int local_3a4;
  int local_3a0;
  int local_39c;
  int local_398;
  int local_394;
  undefined8 local_390;
  char *local_388;
  undefined8 *local_380;
  void *local_378;
  undefined8 local_370;
  undefined1 local_368 [24];
  undefined4 local_350;
  undefined1 local_2d8 [32];
  undefined1 local_2b8 [664];
  long local_20;
  
  ppcVar18 = &local_3e8;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_390 = 0;
  local_3e8 = param_5;
  local_3e0 = param_3;
  local_3d8 = param_4;
  local_3d4 = param_2;
  local_3d0 = param_1;
  if ((param_4 == '\x01') && (param_3 != 0)) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("! command_line_arg || inode == NOT_AN_INODE_NUMBER","src/ls.c",0xd07,
                  "gobble_file");
  }
  if (DAT_0012b338 == DAT_0012b330) {
    DAT_0012b328 = FUN_0011c638(DAT_0012b328,DAT_0012b330,0x1a0);
    DAT_0012b330 = DAT_0012b330 * 2;
  }
  local_380 = (undefined8 *)(DAT_0012b338 * 0xd0 + DAT_0012b328);
  memset(local_380,0,0xd0);
  local_380[4] = local_3e0;
  *(int *)(local_380 + 0x15) = local_3d4;
  *(undefined4 *)((long)local_380 + 0xc4) = 0xffffffff;
  if ((DAT_0012b340 != '\x01') && (DAT_0012b341 != '\0')) {
    bVar6 = FUN_001099b1(local_3d0);
    *(uint *)((long)local_380 + 0xc4) = (uint)bVar6;
    if (*(int *)((long)local_380 + 0xc4) != 0) {
      DAT_0012b340 = '\x01';
    }
  }
  if (((((((local_3d8 != '\0') || (DAT_0012b3d5 != '\0')) || (DAT_0012b440 != '\0')) ||
        (((local_3d4 == 3 && (DAT_0012b3d4 != '\0')) &&
         ((cVar7 = FUN_00106325(0x13), cVar7 != '\0' ||
          ((cVar7 = FUN_00106325(0x12), cVar7 != '\0' || (cVar7 = FUN_00106325(0x14), cVar7 != '\0')
           ))))))) ||
       (((DAT_0012b3e9 != '\0' || (DAT_0012b441 != '\0')) &&
        (((local_3d4 == 6 || (local_3d4 == 0)) &&
         ((DAT_0012b3ec == 4 || ((DAT_0012b358 != '\0' || (DAT_0012b3e8 != '\0')))))))))) ||
      ((DAT_0012b3e9 != '\0' && (local_3e0 == 0)))) ||
     ((ppcVar19 = &local_3e8, DAT_0012b441 != '\0' &&
      (((local_3d4 == 0 || (local_3d8 != '\0')) ||
       ((ppcVar19 = &local_3e8, local_3d4 == 5 &&
        ((DAT_0012b3d0 == 3 ||
         ((ppcVar19 = &local_3e8, DAT_0012b3d4 != '\0' &&
          ((((cVar7 = FUN_00106325(0xe), cVar7 != '\0' ||
             (cVar7 = FUN_00106325(0x10), cVar7 != '\0')) ||
            (cVar7 = FUN_00106325(0x11), cVar7 != '\0')) ||
           (cVar7 = FUN_00106325(0x15), ppcVar19 = &local_3e8, cVar7 != '\0')))))))))))))) {
    if ((*local_3d0 == '/') || (*local_3e8 == '\0')) {
      local_388 = local_3d0;
      ppcVar19 = &local_3e8;
    }
    else {
      sVar16 = strlen(local_3d0);
      sVar12 = strlen(local_3e8);
      uVar13 = ((sVar12 + sVar16 + 0x19) / 0x10) * 0x10;
      for (; ppcVar18 != (char **)((long)&local_3e8 - (uVar13 & 0xfffffffffffff000));
          ppcVar18 = (char **)((long)ppcVar18 + -0x1000)) {
        *(undefined8 *)((long)ppcVar18 + -8) = *(undefined8 *)((long)ppcVar18 + -8);
      }
      lVar3 = -(ulong)((uint)uVar13 & 0xfff);
      ppcVar19 = (char **)((long)ppcVar18 + lVar3);
      if ((uVar13 & 0xfff) != 0) {
        *(undefined8 *)((long)ppcVar18 + ((ulong)((uint)uVar13 & 0xfff) - 8) + lVar3) =
             *(undefined8 *)((long)ppcVar18 + ((ulong)((uint)uVar13 & 0xfff) - 8) + lVar3);
      }
      pcVar4 = local_3d0;
      pcVar17 = local_3e8;
      local_388 = (char *)((ulong)((long)ppcVar18 + lVar3 + 0xf) & 0xfffffffffffffff0);
      *(undefined8 *)((long)ppcVar18 + lVar3 + -8) = 0x109e47;
      FUN_0010eb94((char *)((ulong)((long)ppcVar18 + lVar3 + 0xf) & 0xfffffffffffffff0),pcVar17,
                   pcVar4);
    }
    pcVar17 = local_388;
    if (DAT_0012b3d5 != '\0') {
      *(undefined8 *)((long)ppcVar19 + -8) = 0x109e66;
      uVar14 = FUN_0011101b(pcVar17,2);
      local_380[2] = uVar14;
      if (local_380[2] == 0) {
        *(undefined8 *)((long)ppcVar19 + -8) = 0x109e93;
        uVar14 = gettext("error canonicalizing %s");
        pcVar17 = local_388;
        cVar7 = local_3d8;
        *(undefined8 *)((long)ppcVar19 + -8) = 0x109eae;
        FUN_00108de7(cVar7,uVar14,pcVar17);
      }
    }
    pcVar17 = local_388;
    if (DAT_0012b3ec < 4) {
      if ((DAT_0012b3ec < 2) || (local_3d8 == '\0')) goto LAB_00109f7c;
      puVar1 = local_380 + 3;
      *(undefined8 *)((long)ppcVar19 + -8) = 0x109f21;
      local_3c0 = FUN_00105c0e(pcVar17,puVar1);
      local_3c4 = 1;
      if (DAT_0012b3ec != 2) {
        if ((int)local_3c0 < 0) {
          *(undefined8 *)((long)ppcVar19 + -8) = 0x109f47;
          piVar15 = __errno_location();
          local_3c1 = *piVar15 == 2;
        }
        else {
          local_3c1 = (*(uint *)(local_380 + 6) & 0xf000) != 0x4000;
        }
        if ((bool)local_3c1 == true) goto LAB_00109f7c;
      }
    }
    else {
      if (DAT_0012b3ec == 4) {
        puVar1 = local_380 + 3;
        *(undefined8 *)((long)ppcVar19 + -8) = 0x109ee9;
        local_3c0 = FUN_00105c0e(pcVar17,puVar1);
        local_3c4 = 1;
        goto LAB_00109fac;
      }
LAB_00109f7c:
      pcVar17 = local_388;
      puVar1 = local_380 + 3;
      *(undefined8 *)((long)ppcVar19 + -8) = 0x109f99;
      local_3c0 = FUN_00105c48(pcVar17,puVar1);
      local_3c4 = 0;
    }
LAB_00109fac:
    if (local_3c0 != 0) {
      *(undefined8 *)((long)ppcVar19 + -8) = 0x109fc8;
      uVar14 = gettext("cannot access %s");
      pcVar17 = local_388;
      cVar7 = local_3d8;
      *(undefined8 *)((long)ppcVar19 + -8) = 0x109fe3;
      FUN_00108de7(cVar7,uVar14,pcVar17);
      pcVar17 = local_3d0;
      local_380[0x16] = &DAT_0012b020;
      if (local_3d8 == '\0') {
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a01a;
        uVar14 = FUN_0011cc92(pcVar17);
        *local_380 = uVar14;
        DAT_0012b338 = DAT_0012b338 + 1;
        uVar14 = 0;
      }
      else {
        uVar14 = 0;
      }
      goto LAB_0010a84f;
    }
    *(undefined1 *)(local_380 + 0x17) = 1;
    if (((local_3d4 == 5) || ((*(uint *)(local_380 + 6) & 0xf000) == 0x8000)) &&
       (DAT_0012b3d4 != '\0')) {
      *(undefined8 *)((long)ppcVar19 + -8) = 0x10a085;
      cVar7 = FUN_00106325(0x15);
      puVar1 = local_380;
      pcVar17 = local_388;
      if (cVar7 != '\0') {
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a0a2;
        uVar8 = FUN_00109936(pcVar17,puVar1);
        *(undefined1 *)(local_380 + 0x18) = uVar8;
      }
    }
    puVar1 = local_380;
    pcVar17 = local_388;
    uVar8 = local_3c4;
    if ((DAT_0012b3ac == 0) || (DAT_0012b380 != '\0')) {
      local_3c3 = '\0';
      local_3c2 = '\0';
      *(undefined8 *)((long)ppcVar19 + -8) = 0x10a0f6;
      local_3bc = FUN_001097fa(pcVar17,puVar1,uVar8);
      local_3c0 = -(local_3bc >> 0x1f);
      if (local_3c0 == 0) {
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a11c;
        cVar7 = FUN_0010525c();
        if (cVar7 == '\0') {
          pcVar17 = (char *)local_380[0x16];
          *(undefined8 *)((long)ppcVar19 + -8) = 0x10a16d;
          iVar9 = strcmp("unlabeled",pcVar17);
          local_3c3 = iVar9 != 0;
        }
        else {
          pcVar17 = (char *)local_380[0x16];
          *(undefined8 *)((long)ppcVar19 + -8) = 0x10a140;
          iVar9 = strcmp("_",pcVar17);
          local_3c3 = iVar9 != 0;
        }
      }
      else {
        local_380[0x16] = &DAT_0012b020;
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a194;
        piVar15 = __errno_location();
        iVar9 = *piVar15;
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a19d;
        cVar7 = FUN_0010524c(iVar9);
        if (cVar7 == '\0') {
          *(undefined8 *)((long)ppcVar19 + -8) = 0x10a1a6;
          piVar15 = __errno_location();
          if (*piVar15 != 0x3d) goto LAB_0010a1b7;
        }
        local_3c0 = 0;
      }
LAB_0010a1b7:
      puVar1 = local_380;
      pcVar17 = local_388;
      if ((local_3c0 == 0) && (DAT_0012b3ac == 0)) {
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a1e3;
        local_3b8 = FUN_001098ab(pcVar17,puVar1);
        local_3c0 = local_3b8 >> 0x1f;
        local_3c2 = 0 < (int)local_3b8;
      }
      pcVar17 = local_388;
      if ((local_3c3 == '\0') && (local_3c2 == '\0')) {
        uVar10 = 0;
      }
      else if ((local_3c3 == '\0') || (local_3c2 == '\x01')) {
        uVar10 = 2;
      }
      else {
        uVar10 = 1;
      }
      *(undefined4 *)((long)local_380 + 0xbc) = uVar10;
      DAT_0012b381 = *(int *)((long)local_380 + 0xbc) != 0 || DAT_0012b381 != '\0';
      if (local_3c0 != 0) {
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a2a2;
        uVar14 = FUN_0011ac15(0,3,pcVar17);
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a2aa;
        piVar15 = __errno_location();
        iVar9 = *piVar15;
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a2c7;
        error(0,iVar9,&DAT_00120a54,uVar14);
      }
    }
    puVar1 = local_380;
    pcVar17 = local_388;
    cVar7 = local_3d8;
    if (((*(uint *)(local_380 + 6) & 0xf000) == 0xa000) &&
       ((DAT_0012b3ac == 0 || (DAT_0012b3e8 != '\0')))) {
      *(undefined8 *)((long)ppcVar19 + -8) = 0x10a31a;
      FUN_0010a8f5(pcVar17,puVar1,cVar7);
      pcVar17 = local_388;
      uVar14 = local_380[1];
      *(undefined8 *)((long)ppcVar19 + -8) = 0x10a337;
      local_378 = (void *)FUN_0010a964(pcVar17,uVar14);
      if ((local_378 != (void *)0x0) && (*(int *)((long)local_380 + 0xc4) == 0)) {
        uVar14 = local_380[1];
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a36c;
        cVar7 = FUN_001099b1(uVar14);
        if (cVar7 != '\0') {
          *(undefined4 *)((long)local_380 + 0xc4) = 0xffffffff;
        }
      }
      pvVar5 = local_378;
      if ((local_378 != (void *)0x0) && ((1 < DAT_0012b3d0 || (DAT_0012b3e8 != '\0')))) {
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a3ba;
        iVar9 = FUN_00105c82(pvVar5,local_368);
        if (iVar9 == 0) {
          *(undefined1 *)((long)local_380 + 0xb9) = 1;
          *(undefined4 *)((long)local_380 + 0xac) = local_350;
        }
      }
      pvVar5 = local_378;
      *(undefined8 *)((long)ppcVar19 + -8) = 0x10a3ee;
      free(pvVar5);
    }
    if ((*(uint *)(local_380 + 6) & 0xf000) == 0xa000) {
      *(undefined4 *)(local_380 + 0x15) = 6;
    }
    else if ((*(uint *)(local_380 + 6) & 0xf000) == 0x4000) {
      if ((local_3d8 == '\0') || (DAT_0012b3f1 == '\x01')) {
        *(undefined4 *)(local_380 + 0x15) = 3;
      }
      else {
        *(undefined4 *)(local_380 + 0x15) = 9;
      }
    }
    else {
      *(undefined4 *)(local_380 + 0x15) = 5;
    }
    uVar14 = local_380[0xb];
    local_390 = uVar14;
    if ((DAT_0012b3ac == 0) || (DAT_0012b3bb != '\0')) {
      *(undefined8 *)((long)ppcVar19 + -8) = 0x10a4cd;
      uVar14 = FUN_00113d91(uVar14,local_2b8,DAT_0012b3bc,0x200,DAT_0012b3c0);
      *(undefined8 *)((long)ppcVar19 + -8) = 0x10a4da;
      local_3b4 = FUN_0011574e(uVar14,0);
      if (DAT_0012b388 < local_3b4) {
        DAT_0012b388 = local_3b4;
      }
    }
    if (DAT_0012b3ac == 0) {
      if (DAT_0012b022 != '\0') {
        uVar10 = *(undefined4 *)((long)local_380 + 0x34);
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a524;
        local_3b0 = FUN_0010c5c3(uVar10);
        if (DAT_0012b394 < local_3b0) {
          DAT_0012b394 = local_3b0;
        }
      }
      if (DAT_0012b023 != '\0') {
        uVar10 = *(undefined4 *)(local_380 + 7);
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a560;
        local_3ac = FUN_0010c603(uVar10);
        if (DAT_0012b398 < local_3ac) {
          DAT_0012b398 = local_3ac;
        }
      }
      if (DAT_0012b3b9 != '\0') {
        uVar10 = *(undefined4 *)((long)local_380 + 0x34);
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a59c;
        local_3a8 = FUN_0010c5c3(uVar10);
        if (DAT_0012b39c < local_3a8) {
          DAT_0012b39c = local_3a8;
        }
      }
    }
    if (DAT_0012b380 != '\0') {
      pcVar17 = (char *)local_380[0x16];
      *(undefined8 *)((long)ppcVar19 + -8) = 0x10a5dd;
      sVar16 = strlen(pcVar17);
      local_3a4 = (int)sVar16;
      if (DAT_0012b390 < local_3a4) {
        DAT_0012b390 = local_3a4;
      }
    }
    if (DAT_0012b3ac == 0) {
      uVar14 = local_380[5];
      *(undefined8 *)((long)ppcVar19 + -8) = 0x10a628;
      pcVar17 = (char *)FUN_00115035(uVar14,local_2d8);
      *(undefined8 *)((long)ppcVar19 + -8) = 0x10a630;
      sVar16 = strlen(pcVar17);
      local_3a0 = (int)sVar16;
      if (DAT_0012b38c < local_3a0) {
        DAT_0012b38c = local_3a0;
      }
      if (((*(uint *)(local_380 + 6) & 0xf000) == 0x2000) ||
         ((*(uint *)(local_380 + 6) & 0xf000) == 0x6000)) {
        uVar2 = local_380[8];
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a693;
        uVar11 = gnu_dev_major(uVar2);
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a6a7;
        pcVar17 = (char *)FUN_00115035(uVar11,local_2b8);
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a6af;
        sVar16 = strlen(pcVar17);
        local_398 = (int)sVar16;
        if (DAT_0012b3a0 < local_398) {
          DAT_0012b3a0 = local_398;
        }
        uVar2 = local_380[8];
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a6e2;
        uVar11 = gnu_dev_minor(uVar2);
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a6f6;
        pcVar17 = (char *)FUN_00115035(uVar11,local_2b8);
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a6fe;
        sVar16 = strlen(pcVar17);
        if (DAT_0012b3a4 < (int)sVar16) {
          DAT_0012b3a4 = (int)sVar16;
        }
        local_398 = DAT_0012b3a4 + DAT_0012b3a0 + 2;
        if (DAT_0012b3a8 < local_398) {
          DAT_0012b3a8 = local_398;
        }
      }
      else {
        uVar14 = local_380[9];
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a768;
        uVar14 = FUN_00109635(uVar14);
        local_370 = uVar14;
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a79a;
        uVar14 = FUN_00113d91(uVar14,local_2b8,DAT_0012b3c8,1,DAT_0012b028);
        *(undefined8 *)((long)ppcVar19 + -8) = 0x10a7a7;
        local_39c = FUN_0011574e(uVar14,0);
        if (DAT_0012b3a8 < local_39c) {
          DAT_0012b3a8 = local_39c;
        }
      }
    }
  }
  if (DAT_0012b3e9 != '\0') {
    uVar14 = local_380[4];
    *(undefined8 *)((long)ppcVar19 + -8) = 0x10a7f2;
    pcVar17 = (char *)FUN_00115035(uVar14,local_2b8);
    *(undefined8 *)((long)ppcVar19 + -8) = 0x10a7fa;
    sVar16 = strlen(pcVar17);
    local_394 = (int)sVar16;
    if (DAT_0012b384 < local_394) {
      DAT_0012b384 = local_394;
    }
  }
  pcVar17 = local_3d0;
  *(undefined8 *)((long)ppcVar19 + -8) = 0x10a829;
  uVar14 = FUN_0011cc92(pcVar17);
  *local_380 = uVar14;
  DAT_0012b338 = DAT_0012b338 + 1;
  uVar14 = local_390;
LAB_0010a84f:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar14;
  }
                    /* WARNING: Subroutine does not return */
  *(undefined8 *)((long)ppcVar19 + -8) = 0x10a863;
  __stack_chk_fail();
}




// Function: is_directory @ 0xa869

undefined4 is_directory(long param_1)

{
  undefined4 uVar1;
  
  if ((*(int *)(param_1 + 0xa8) == 3) || (*(int *)(param_1 + 0xa8) == 9)) {
    uVar1 = 1;
  }
  else {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: is_linked_directory @ 0xa8a4

undefined4 is_linked_directory(long param_1)

{
  undefined4 uVar1;
  
  if (((*(int *)(param_1 + 0xa8) == 3) || (*(int *)(param_1 + 0xa8) == 9)) ||
     ((*(uint *)(param_1 + 0xac) & 0xf000) == 0x4000)) {
    uVar1 = 1;
  }
  else {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: get_link_name @ 0xa8f5

void get_link_name(undefined8 param_1,long param_2,undefined1 param_3)

{
  undefined8 uVar1;
  
  uVar1 = FUN_0010f7a0(param_1,*(undefined8 *)(param_2 + 0x48));
  *(undefined8 *)(param_2 + 8) = uVar1;
  if (*(long *)(param_2 + 8) == 0) {
    uVar1 = gettext("cannot read symbolic link %s");
    FUN_00108de7(param_3,uVar1,param_1);
  }
  return;
}




// Function: make_link_name @ 0xa964

char * make_link_name(char *param_1,char *param_2)

{
  char *__dest;
  size_t sVar1;
  char *__dest_00;
  size_t local_18;
  
  if (param_2 == (char *)0x0) {
    __dest = (char *)0x0;
  }
  else if (*param_2 == '/') {
    __dest = (char *)FUN_0011cc92(param_2);
  }
  else {
    local_18 = FUN_001111a6(param_1);
    if (local_18 == 0) {
      __dest = (char *)FUN_0011cc92(param_2);
    }
    else {
      sVar1 = strlen(param_2);
      __dest = (char *)FUN_0011c68b(sVar1 + local_18 + 2);
      if (param_1[local_18 - 1] != '/') {
        local_18 = local_18 + 1;
      }
      __dest_00 = stpncpy(__dest,param_1,local_18);
      stpcpy(__dest_00,param_2);
    }
  }
  return __dest;
}




// Function: basename_is_dot_or_dotdot @ 0xaa37

void basename_is_dot_or_dotdot(undefined8 param_1)

{
  undefined8 uVar1;
  
  uVar1 = FUN_0010ff56(param_1);
  FUN_00104ec1(uVar1);
  return;
}




// Function: extract_dirs_from_files @ 0xaa65

void extract_dirs_from_files(long param_1,undefined1 param_2)

{
  ulong uVar1;
  undefined8 *puVar2;
  long lVar3;
  char cVar4;
  void *__ptr;
  ulong local_30;
  long local_28;
  
  if ((param_1 != 0) && (DAT_0012b320 != 0)) {
    FUN_00108e46(0,param_1,0);
  }
  local_30 = DAT_0012b338;
LAB_0010aba6:
  do {
    uVar1 = local_30 - 1;
    if (local_30 == 0) {
      local_28 = 0;
      for (local_30 = 0; local_30 < DAT_0012b338; local_30 = local_30 + 1) {
        lVar3 = *(long *)(local_30 * 8 + DAT_0012b348);
        *(long *)(DAT_0012b348 + local_28 * 8) = lVar3;
        local_28 = local_28 + (ulong)(*(int *)(lVar3 + 0xa8) != 9);
      }
      DAT_0012b338 = local_28;
      return;
    }
    puVar2 = *(undefined8 **)(uVar1 * 8 + DAT_0012b348);
    cVar4 = FUN_0010a869(puVar2);
    local_30 = uVar1;
  } while (cVar4 == '\0');
  if (param_1 != 0) goto code_r0x0010aaf6;
  goto LAB_0010ab10;
code_r0x0010aaf6:
  cVar4 = FUN_0010aa37(*puVar2);
  if (cVar4 != '\x01') {
LAB_0010ab10:
    if ((param_1 == 0) || (*(char *)*puVar2 == '/')) {
      FUN_00108e46(*puVar2,puVar2[1],param_2);
    }
    else {
      __ptr = (void *)FUN_001116a7(param_1,*puVar2,0);
      FUN_00108e46(__ptr,puVar2[1],param_2);
      free(__ptr);
    }
    if (*(int *)(puVar2 + 0x15) == 9) {
      FUN_00109669(puVar2);
    }
  }
  goto LAB_0010aba6;
}




// Function: xstrcoll @ 0xac36

int xstrcoll(char *param_1,char *param_2)

{
  int iVar1;
  int *piVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  
  piVar2 = __errno_location();
  *piVar2 = 0;
  iVar1 = strcoll(param_1,param_2);
  piVar2 = __errno_location();
  if (*piVar2 != 0) {
    uVar3 = FUN_0011ae9d(1,param_2);
    uVar4 = FUN_0011ae9d(0,param_1);
    uVar5 = gettext("cannot compare file names %s and %s");
    piVar2 = __errno_location();
    error(0,*piVar2,uVar5,uVar4,uVar3);
    FUN_00108db1(0);
                    /* WARNING: Subroutine does not return */
    longjmp((__jmp_buf_tag *)&DAT_0012c360,1);
  }
  return iVar1;
}




// Function: dirfirst_check @ 0xad02

ulong dirfirst_check(undefined8 param_1,undefined8 param_2,code *param_3)

{
  byte bVar1;
  byte bVar2;
  ulong uVar3;
  
  bVar1 = FUN_0010a8a4(param_2);
  bVar2 = FUN_0010a8a4(param_1);
  if ((uint)bVar1 - (uint)bVar2 == 0) {
    uVar3 = (*param_3)(param_1,param_2);
  }
  else {
    uVar3 = (ulong)((uint)bVar1 - (uint)bVar2);
  }
  return uVar3;
}




// Function: cmp_ctime @ 0xad65

ulong cmp_ctime(undefined8 *param_1,undefined8 *param_2,code *param_3)

{
  uint uVar1;
  ulong uVar2;
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  
  auVar3 = FUN_0011b477(param_1 + 3);
  auVar4 = FUN_0011b477(param_2 + 3);
  uVar1 = FUN_0011bc1f(auVar4._0_8_,auVar4._8_8_,auVar3._0_8_,auVar3._8_8_);
  if (uVar1 == 0) {
    uVar2 = (*param_3)(*param_1,*param_2);
  }
  else {
    uVar2 = (ulong)uVar1;
  }
  return uVar2;
}




// Function: cmp_mtime @ 0xadf8

ulong cmp_mtime(undefined8 *param_1,undefined8 *param_2,code *param_3)

{
  uint uVar1;
  ulong uVar2;
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  
  auVar3 = FUN_0011b491(param_1 + 3);
  auVar4 = FUN_0011b491(param_2 + 3);
  uVar1 = FUN_0011bc1f(auVar4._0_8_,auVar4._8_8_,auVar3._0_8_,auVar3._8_8_);
  if (uVar1 == 0) {
    uVar2 = (*param_3)(*param_1,*param_2);
  }
  else {
    uVar2 = (ulong)uVar1;
  }
  return uVar2;
}




// Function: cmp_atime @ 0xae8b

ulong cmp_atime(undefined8 *param_1,undefined8 *param_2,code *param_3)

{
  uint uVar1;
  ulong uVar2;
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  
  auVar3 = FUN_0011b45d(param_1 + 3);
  auVar4 = FUN_0011b45d(param_2 + 3);
  uVar1 = FUN_0011bc1f(auVar4._0_8_,auVar4._8_8_,auVar3._0_8_,auVar3._8_8_);
  if (uVar1 == 0) {
    uVar2 = (*param_3)(*param_1,*param_2);
  }
  else {
    uVar2 = (ulong)uVar1;
  }
  return uVar2;
}




// Function: cmp_btime @ 0xaf1e

ulong cmp_btime(undefined8 *param_1,undefined8 *param_2,code *param_3)

{
  uint uVar1;
  ulong uVar2;
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  
  auVar3 = FUN_00105990(param_1 + 3);
  auVar4 = FUN_00105990(param_2 + 3);
  uVar1 = FUN_0011bc1f(auVar4._0_8_,auVar4._8_8_,auVar3._0_8_,auVar3._8_8_);
  if (uVar1 == 0) {
    uVar2 = (*param_3)(*param_1,*param_2);
  }
  else {
    uVar2 = (ulong)uVar1;
  }
  return uVar2;
}




// Function: off_cmp @ 0xafb1

ulong off_cmp(long param_1,long param_2)

{
  ulong uVar1;
  
  if (param_1 < param_2) {
    uVar1 = 0xffffffff;
  }
  else {
    uVar1 = (ulong)(param_2 < param_1);
  }
  return uVar1;
}




// Function: cmp_size @ 0xafe2

ulong cmp_size(undefined8 *param_1,undefined8 *param_2,code *param_3)

{
  uint uVar1;
  ulong uVar2;
  
  uVar1 = FUN_0010afb1(param_2[9],param_1[9]);
  if (uVar1 == 0) {
    uVar2 = (*param_3)(*param_1,*param_2);
  }
  else {
    uVar2 = (ulong)uVar1;
  }
  return uVar2;
}




// Function: cmp_name @ 0xb03f

void cmp_name(undefined8 *param_1,undefined8 *param_2,code *param_3)

{
  (*param_3)(*param_1,*param_2);
  return;
}




// Function: cmp_extension @ 0xb073

ulong cmp_extension(undefined8 *param_1,undefined8 *param_2,code *param_3)

{
  uint uVar1;
  char *pcVar2;
  char *pcVar3;
  ulong uVar4;
  
  pcVar2 = strrchr((char *)*param_1,0x2e);
  pcVar3 = strrchr((char *)*param_2,0x2e);
  if (pcVar3 == (char *)0x0) {
    pcVar3 = "";
  }
  if (pcVar2 == (char *)0x0) {
    pcVar2 = "";
  }
  uVar1 = (*param_3)(pcVar2,pcVar3);
  if (uVar1 == 0) {
    uVar4 = (*param_3)(*param_1,*param_2);
  }
  else {
    uVar4 = (ulong)uVar1;
  }
  return uVar4;
}




// Function: fileinfo_name_width @ 0xb119

undefined8 fileinfo_name_width(undefined8 *param_1)

{
  undefined8 uVar1;
  
  if (param_1[0x19] == 0) {
    uVar1 = FUN_0010d671(*param_1,DAT_0012b410,*(undefined4 *)((long)param_1 + 0xc4));
  }
  else {
    uVar1 = param_1[0x19];
  }
  return uVar1;
}




// Function: cmp_width @ 0xb16b

ulong cmp_width(undefined8 *param_1,undefined8 *param_2,code *param_3)

{
  int iVar1;
  int iVar2;
  ulong uVar3;
  
  iVar1 = FUN_0010b119(param_1);
  iVar2 = FUN_0010b119(param_2);
  if (iVar1 - iVar2 == 0) {
    uVar3 = (*param_3)(*param_1,*param_2);
  }
  else {
    uVar3 = (ulong)(uint)(iVar1 - iVar2);
  }
  return uVar3;
}




// Function: xstrcoll_ctime @ 0xb1d2

void xstrcoll_ctime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad65(param_1,param_2,FUN_0010ac36);
  return;
}




// Function: strcmp_ctime @ 0xb202

void strcmp_ctime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad65(param_1,param_2,PTR_strcmp_0012afc0);
  return;
}




// Function: rev_xstrcoll_ctime @ 0xb232

void rev_xstrcoll_ctime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad65(param_2,param_1,FUN_0010ac36);
  return;
}




// Function: rev_strcmp_ctime @ 0xb262

void rev_strcmp_ctime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad65(param_2,param_1,PTR_strcmp_0012afc0);
  return;
}




// Function: xstrcoll_df_ctime @ 0xb292

void xstrcoll_df_ctime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b1d2);
  return;
}




// Function: strcmp_df_ctime @ 0xb2c2

void strcmp_df_ctime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b202);
  return;
}




// Function: rev_xstrcoll_df_ctime @ 0xb2f2

void rev_xstrcoll_df_ctime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b232);
  return;
}




// Function: rev_strcmp_df_ctime @ 0xb322

void rev_strcmp_df_ctime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b262);
  return;
}




// Function: xstrcoll_mtime @ 0xb352

void xstrcoll_mtime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010adf8(param_1,param_2,FUN_0010ac36);
  return;
}




// Function: strcmp_mtime @ 0xb382

void strcmp_mtime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010adf8(param_1,param_2,PTR_strcmp_0012afc0);
  return;
}




// Function: rev_xstrcoll_mtime @ 0xb3b2

void rev_xstrcoll_mtime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010adf8(param_2,param_1,FUN_0010ac36);
  return;
}




// Function: rev_strcmp_mtime @ 0xb3e2

void rev_strcmp_mtime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010adf8(param_2,param_1,PTR_strcmp_0012afc0);
  return;
}




// Function: xstrcoll_df_mtime @ 0xb412

void xstrcoll_df_mtime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b352);
  return;
}




// Function: strcmp_df_mtime @ 0xb442

void strcmp_df_mtime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b382);
  return;
}




// Function: rev_xstrcoll_df_mtime @ 0xb472

void rev_xstrcoll_df_mtime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b3b2);
  return;
}




// Function: rev_strcmp_df_mtime @ 0xb4a2

void rev_strcmp_df_mtime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b3e2);
  return;
}




// Function: xstrcoll_atime @ 0xb4d2

void xstrcoll_atime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ae8b(param_1,param_2,FUN_0010ac36);
  return;
}




// Function: strcmp_atime @ 0xb502

void strcmp_atime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ae8b(param_1,param_2,PTR_strcmp_0012afc0);
  return;
}




// Function: rev_xstrcoll_atime @ 0xb532

void rev_xstrcoll_atime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ae8b(param_2,param_1,FUN_0010ac36);
  return;
}




// Function: rev_strcmp_atime @ 0xb562

void rev_strcmp_atime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ae8b(param_2,param_1,PTR_strcmp_0012afc0);
  return;
}




// Function: xstrcoll_df_atime @ 0xb592

void xstrcoll_df_atime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b4d2);
  return;
}




// Function: strcmp_df_atime @ 0xb5c2

void strcmp_df_atime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b502);
  return;
}




// Function: rev_xstrcoll_df_atime @ 0xb5f2

void rev_xstrcoll_df_atime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b532);
  return;
}




// Function: rev_strcmp_df_atime @ 0xb622

void rev_strcmp_df_atime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b562);
  return;
}




// Function: xstrcoll_btime @ 0xb652

void xstrcoll_btime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010af1e(param_1,param_2,FUN_0010ac36);
  return;
}




// Function: strcmp_btime @ 0xb682

void strcmp_btime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010af1e(param_1,param_2,PTR_strcmp_0012afc0);
  return;
}




// Function: rev_xstrcoll_btime @ 0xb6b2

void rev_xstrcoll_btime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010af1e(param_2,param_1,FUN_0010ac36);
  return;
}




// Function: rev_strcmp_btime @ 0xb6e2

void rev_strcmp_btime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010af1e(param_2,param_1,PTR_strcmp_0012afc0);
  return;
}




// Function: xstrcoll_df_btime @ 0xb712

void xstrcoll_df_btime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b652);
  return;
}




// Function: strcmp_df_btime @ 0xb742

void strcmp_df_btime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b682);
  return;
}




// Function: rev_xstrcoll_df_btime @ 0xb772

void rev_xstrcoll_df_btime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b6b2);
  return;
}




// Function: rev_strcmp_df_btime @ 0xb7a2

void rev_strcmp_df_btime(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b6e2);
  return;
}




// Function: xstrcoll_size @ 0xb7d2

void xstrcoll_size(undefined8 param_1,undefined8 param_2)

{
  FUN_0010afe2(param_1,param_2,FUN_0010ac36);
  return;
}




// Function: strcmp_size @ 0xb802

void strcmp_size(undefined8 param_1,undefined8 param_2)

{
  FUN_0010afe2(param_1,param_2,PTR_strcmp_0012afc0);
  return;
}




// Function: rev_xstrcoll_size @ 0xb832

void rev_xstrcoll_size(undefined8 param_1,undefined8 param_2)

{
  FUN_0010afe2(param_2,param_1,FUN_0010ac36);
  return;
}




// Function: rev_strcmp_size @ 0xb862

void rev_strcmp_size(undefined8 param_1,undefined8 param_2)

{
  FUN_0010afe2(param_2,param_1,PTR_strcmp_0012afc0);
  return;
}




// Function: xstrcoll_df_size @ 0xb892

void xstrcoll_df_size(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b7d2);
  return;
}




// Function: strcmp_df_size @ 0xb8c2

void strcmp_df_size(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b802);
  return;
}




// Function: rev_xstrcoll_df_size @ 0xb8f2

void rev_xstrcoll_df_size(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b832);
  return;
}




// Function: rev_strcmp_df_size @ 0xb922

void rev_strcmp_df_size(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b862);
  return;
}




// Function: xstrcoll_name @ 0xb952

void xstrcoll_name(undefined8 param_1,undefined8 param_2)

{
  FUN_0010b03f(param_1,param_2,FUN_0010ac36);
  return;
}




// Function: strcmp_name @ 0xb982

void strcmp_name(undefined8 param_1,undefined8 param_2)

{
  FUN_0010b03f(param_1,param_2,PTR_strcmp_0012afc0);
  return;
}




// Function: rev_xstrcoll_name @ 0xb9b2

void rev_xstrcoll_name(undefined8 param_1,undefined8 param_2)

{
  FUN_0010b03f(param_2,param_1,FUN_0010ac36);
  return;
}




// Function: rev_strcmp_name @ 0xb9e2

void rev_strcmp_name(undefined8 param_1,undefined8 param_2)

{
  FUN_0010b03f(param_2,param_1,PTR_strcmp_0012afc0);
  return;
}




// Function: xstrcoll_df_name @ 0xba12

void xstrcoll_df_name(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b952);
  return;
}




// Function: strcmp_df_name @ 0xba42

void strcmp_df_name(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b982);
  return;
}




// Function: rev_xstrcoll_df_name @ 0xba72

void rev_xstrcoll_df_name(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b9b2);
  return;
}




// Function: rev_strcmp_df_name @ 0xbaa2

void rev_strcmp_df_name(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010b9e2);
  return;
}




// Function: xstrcoll_extension @ 0xbad2

void xstrcoll_extension(undefined8 param_1,undefined8 param_2)

{
  FUN_0010b073(param_1,param_2,FUN_0010ac36);
  return;
}




// Function: strcmp_extension @ 0xbb02

void strcmp_extension(undefined8 param_1,undefined8 param_2)

{
  FUN_0010b073(param_1,param_2,PTR_strcmp_0012afc0);
  return;
}




// Function: rev_xstrcoll_extension @ 0xbb32

void rev_xstrcoll_extension(undefined8 param_1,undefined8 param_2)

{
  FUN_0010b073(param_2,param_1,FUN_0010ac36);
  return;
}




// Function: rev_strcmp_extension @ 0xbb62

void rev_strcmp_extension(undefined8 param_1,undefined8 param_2)

{
  FUN_0010b073(param_2,param_1,PTR_strcmp_0012afc0);
  return;
}




// Function: xstrcoll_df_extension @ 0xbb92

void xstrcoll_df_extension(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010bad2);
  return;
}




// Function: strcmp_df_extension @ 0xbbc2

void strcmp_df_extension(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010bb02);
  return;
}




// Function: rev_xstrcoll_df_extension @ 0xbbf2

void rev_xstrcoll_df_extension(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010bb32);
  return;
}




// Function: rev_strcmp_df_extension @ 0xbc22

void rev_strcmp_df_extension(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010bb62);
  return;
}




// Function: xstrcoll_width @ 0xbc52

void xstrcoll_width(undefined8 param_1,undefined8 param_2)

{
  FUN_0010b16b(param_1,param_2,FUN_0010ac36);
  return;
}




// Function: strcmp_width @ 0xbc82

void strcmp_width(undefined8 param_1,undefined8 param_2)

{
  FUN_0010b16b(param_1,param_2,PTR_strcmp_0012afc0);
  return;
}




// Function: rev_xstrcoll_width @ 0xbcb2

void rev_xstrcoll_width(undefined8 param_1,undefined8 param_2)

{
  FUN_0010b16b(param_2,param_1,FUN_0010ac36);
  return;
}




// Function: rev_strcmp_width @ 0xbce2

void rev_strcmp_width(undefined8 param_1,undefined8 param_2)

{
  FUN_0010b16b(param_2,param_1,PTR_strcmp_0012afc0);
  return;
}




// Function: xstrcoll_df_width @ 0xbd12

void xstrcoll_df_width(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010bc52);
  return;
}




// Function: strcmp_df_width @ 0xbd42

void strcmp_df_width(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010bc82);
  return;
}




// Function: rev_xstrcoll_df_width @ 0xbd72

void rev_xstrcoll_df_width(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010bcb2);
  return;
}




// Function: rev_strcmp_df_width @ 0xbda2

void rev_strcmp_df_width(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010bce2);
  return;
}




// Function: cmp_version @ 0xbdd2

int cmp_version(undefined8 *param_1,undefined8 *param_2)

{
  int iVar1;
  
  iVar1 = FUN_00111c04(*param_1,*param_2);
  if (iVar1 == 0) {
    iVar1 = strcmp((char *)*param_1,(char *)*param_2);
  }
  return iVar1;
}




// Function: xstrcoll_version @ 0xbe28

void xstrcoll_version(undefined8 param_1,undefined8 param_2)

{
  FUN_0010bdd2(param_1,param_2);
  return;
}




// Function: rev_xstrcoll_version @ 0xbe51

void rev_xstrcoll_version(undefined8 param_1,undefined8 param_2)

{
  FUN_0010bdd2(param_2,param_1);
  return;
}




// Function: xstrcoll_df_version @ 0xbe7a

void xstrcoll_df_version(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010be28);
  return;
}




// Function: rev_xstrcoll_df_version @ 0xbeaa

void rev_xstrcoll_df_version(undefined8 param_1,undefined8 param_2)

{
  FUN_0010ad02(param_1,param_2,FUN_0010be51);
  return;
}




// Function: initialize_ordering_vector @ 0xbeda

void initialize_ordering_vector(void)

{
  undefined8 local_10;
  
  for (local_10 = 0; local_10 < DAT_0012b338; local_10 = local_10 + 1) {
    *(ulong *)(local_10 * 8 + DAT_0012b348) = DAT_0012b328 + local_10 * 0xd0;
  }
  return;
}




// Function: update_current_files_info @ 0xbf3d

void update_current_files_info(void)

{
  long lVar1;
  undefined8 uVar2;
  undefined8 local_18;
  
  if ((DAT_0012b3b4 == 2) || ((DAT_0012b430 != 0 && ((DAT_0012b3ac == 2 || (DAT_0012b3ac == 3))))))
  {
    for (local_18 = 0; local_18 < DAT_0012b338; local_18 = local_18 + 1) {
      lVar1 = *(long *)(local_18 * 8 + DAT_0012b348);
      uVar2 = FUN_0010b119(lVar1);
      *(undefined8 *)(lVar1 + 200) = uVar2;
    }
  }
  return;
}




// Function: sort_files @ 0xbfc5

void sort_files(void)

{
  int iVar1;
  byte local_9;
  
  if (DAT_0012b350 < (DAT_0012b338 >> 1) + DAT_0012b338) {
    free(DAT_0012b348);
    DAT_0012b348 = (void *)FUN_0011c804(DAT_0012b338,0x18);
    DAT_0012b350 = DAT_0012b338 * 3;
  }
  FUN_0010beda();
  FUN_0010bf3d();
  if (DAT_0012b3b4 != 6) {
    iVar1 = _setjmp((__jmp_buf_tag *)&DAT_0012c360);
    if (iVar1 == 0) {
      local_9 = 0;
    }
    else {
      local_9 = 1;
      if (DAT_0012b3b4 == 4) {
                    /* WARNING: Subroutine does not return */
        __assert_fail("sort_type != sort_version","src/ls.c",0x1008,"sort_files");
      }
      FUN_0010beda();
    }
    iVar1 = DAT_0012b3b0;
    if (DAT_0012b3b4 != 5) {
      iVar1 = 0;
    }
    FUN_00115dfc(DAT_0012b348,DAT_0012b338,
                 (&PTR_FUN_0012a780)
                 [((long)(int)(uint)DAT_0012b3b8 +
                  ((long)(int)(uint)local_9 + (ulong)(uint)(iVar1 + DAT_0012b3b4) * 2) * 2) * 2 +
                  (long)(int)(uint)DAT_0012b3f2]);
  }
  return;
}




// Function: print_current_files @ 0xc12d

void print_current_files(void)

{
  ulong local_10;
  
  switch(DAT_0012b3ac) {
  case 0:
    for (local_10 = 0; local_10 < DAT_0012b338; local_10 = local_10 + 1) {
      FUN_001063f3();
      FUN_0010c6c8(*(undefined8 *)(local_10 * 8 + DAT_0012b348));
      FUN_00105422((int)DAT_0012b030);
    }
    break;
  case 1:
    for (local_10 = 0; local_10 < DAT_0012b338; local_10 = local_10 + 1) {
      FUN_0010dd38(*(undefined8 *)(local_10 * 8 + DAT_0012b348),0);
      putchar_unlocked((int)DAT_0012b030);
    }
    break;
  case 2:
    if (DAT_0012b430 == 0) {
      FUN_0010e9d6(0x20);
    }
    else {
      FUN_0010e708();
    }
    break;
  case 3:
    if (DAT_0012b430 == 0) {
      FUN_0010e9d6(0x20);
    }
    else {
      FUN_0010e86a();
    }
    break;
  case 4:
    FUN_0010e9d6(0x2c);
  }
  return;
}




// Function: align_nstrftime @ 0xc25f

void align_nstrftime(undefined8 param_1,undefined8 param_2,byte param_3,long param_4,undefined8 param_5
                 ,undefined4 param_6)

{
  undefined *puVar1;
  
  if (DAT_0012c340 == '\0') {
    puVar1 = (&PTR_s__b__e__Y_0012b200)[(int)(uint)param_3];
  }
  else {
    puVar1 = &DAT_0012b740 +
             ((long)(int)(uint)param_3 * 0xc + (long)*(int *)(param_4 + 0x10)) * 0x80;
  }
  FUN_00115f45(param_1,param_2,puVar1,param_4,param_5,param_6);
  return;
}




// Function: long_time_expected_width @ 0xc308

int long_time_expected_width(void)

{
  long lVar1;
  long in_FS_OFFSET;
  undefined8 local_458;
  long local_450;
  undefined1 local_448 [64];
  undefined1 local_408 [1016];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0012b034 < 0) {
    local_458 = 0;
    lVar1 = FUN_0011b9a5(DAT_0012b438,&local_458,local_448);
    if (lVar1 != 0) {
      local_450 = FUN_0010c25f(local_408,0x3e9,0,local_448,DAT_0012b438,0);
      if (local_450 != 0) {
        DAT_0012b034 = FUN_00115784(local_408,local_450,0);
      }
    }
    if (DAT_0012b034 < 0) {
      DAT_0012b034 = 0;
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return DAT_0012b034;
}




// Function: format_user_or_group @ 0xc3fc

void format_user_or_group(long param_1,undefined8 param_2,uint param_3)

{
  int iVar1;
  bool bVar2;
  int local_10;
  
  if (param_1 == 0) {
    iVar1 = printf("%*lu ",(ulong)param_3,param_2);
    DAT_0012b500 = DAT_0012b500 + iVar1;
  }
  else {
    local_10 = FUN_0011574e(param_1,0);
    local_10 = param_3 - local_10;
    if (local_10 < 0) {
      local_10 = 0;
    }
    FUN_0010549e(param_1);
    do {
      FUN_00105422(0x20);
      bVar2 = local_10 != 0;
      local_10 = local_10 + -1;
    } while (bVar2);
  }
  return;
}




// Function: format_user @ 0xc49e

void format_user(undefined4 param_1,undefined4 param_2,char param_3)

{
  undefined1 *puVar1;
  
  if (param_3 == '\x01') {
    if (DAT_0012b3ba == '\0') {
      puVar1 = (undefined1 *)FUN_00114a53(param_1);
    }
    else {
      puVar1 = (undefined1 *)0x0;
    }
  }
  else {
    puVar1 = &DAT_001215ba;
  }
  FUN_0010c3fc(puVar1,param_1,param_2);
  return;
}




// Function: format_group @ 0xc4fe

void format_group(undefined4 param_1,undefined4 param_2,char param_3)

{
  undefined1 *puVar1;
  
  if (param_3 == '\x01') {
    if (DAT_0012b3ba == '\0') {
      puVar1 = (undefined1 *)FUN_00114ca5(param_1);
    }
    else {
      puVar1 = (undefined1 *)0x0;
    }
  }
  else {
    puVar1 = &DAT_001215ba;
  }
  FUN_0010c3fc(puVar1,param_1,param_2);
  return;
}




// Function: format_user_or_group_width @ 0xc55e

int format_user_or_group_width(long param_1,undefined8 param_2)

{
  int iVar1;
  
  if (param_1 == 0) {
    iVar1 = snprintf((char *)0x0,0,"%lu",param_2);
  }
  else {
    iVar1 = FUN_0011574e(param_1,0);
    if (iVar1 < 0) {
      iVar1 = 0;
    }
  }
  return iVar1;
}




// Function: format_user_width @ 0xc5c3

void format_user_width(undefined4 param_1)

{
  undefined8 uVar1;
  
  if (DAT_0012b3ba == '\0') {
    uVar1 = FUN_00114a53(param_1);
  }
  else {
    uVar1 = 0;
  }
  FUN_0010c55e(uVar1,param_1);
  return;
}




// Function: format_group_width @ 0xc603

void format_group_width(undefined4 param_1)

{
  undefined8 uVar1;
  
  if (DAT_0012b3ba == '\0') {
    uVar1 = FUN_00114ca5(param_1);
  }
  else {
    uVar1 = 0;
  }
  FUN_0010c55e(uVar1,param_1);
  return;
}




// Function: format_inode @ 0xc643

undefined1 * format_inode(undefined8 param_1,ulong param_2,long param_3)

{
  undefined1 *puVar1;
  
  if (param_2 < 0x15) {
                    /* WARNING: Subroutine does not return */
    __assert_fail("INT_BUFSIZE_BOUND (uintmax_t) <= buflen","src/ls.c",0x10be,"format_inode");
  }
  if ((*(char *)(param_3 + 0xb8) == '\0') || (*(long *)(param_3 + 0x20) == 0)) {
    puVar1 = &DAT_001215ba;
  }
  else {
    puVar1 = (undefined1 *)FUN_00115035(*(undefined8 *)(param_3 + 0x20),param_1);
  }
  return puVar1;
}




// Function: print_long_format @ 0xc6c8

void print_long_format(long param_1)

{
  char *pcVar1;
  bool bVar2;
  undefined4 uVar3;
  undefined1 uVar4;
  int iVar5;
  int iVar6;
  uint uVar7;
  uint uVar8;
  undefined8 uVar9;
  undefined1 *puVar10;
  undefined8 uVar11;
  long lVar12;
  long in_FS_OFFSET;
  int local_11bc;
  int local_11b8;
  long local_11b0;
  char *local_11a8;
  char *local_11a0;
  char *local_1198;
  undefined1 local_1188 [16];
  long local_1178;
  undefined8 local_1170;
  undefined1 local_1168 [68];
  char local_1124;
  undefined1 auStack_1123 [9];
  undefined1 local_111a;
  undefined1 local_1119;
  undefined1 local_1118 [32];
  undefined1 local_10f8 [656];
  char local_e68 [3656];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  bVar2 = true;
  if (*(char *)(param_1 + 0xb8) == '\0') {
    local_1124 = "?pcdb-lswd"[*(uint *)(param_1 + 0xa8)];
    memset(auStack_1123,0x3f,10);
    local_1119 = 0;
  }
  else {
    FUN_0011167b(param_1 + 0x18,&local_1124);
  }
  if (DAT_0012b381 == '\x01') {
    if (*(int *)(param_1 + 0xbc) == 1) {
      local_111a = 0x2e;
    }
    else if (*(int *)(param_1 + 0xbc) == 2) {
      local_111a = 0x2b;
    }
  }
  else {
    local_111a = 0;
  }
  if (DAT_0012b3b0 == 3) {
    local_1188 = FUN_00105990(param_1 + 0x18);
    if ((local_1188._0_8_ == -1) && (local_1188._8_8_ == -1)) {
      bVar2 = false;
    }
  }
  else {
    if (3 < DAT_0012b3b0) {
LAB_0010c8a6:
                    /* WARNING: Subroutine does not return */
      abort();
    }
    if (DAT_0012b3b0 == 2) {
      local_1188 = FUN_0011b45d(param_1 + 0x18);
    }
    else {
      if (2 < DAT_0012b3b0) goto LAB_0010c8a6;
      if (DAT_0012b3b0 == 0) {
        local_1188 = FUN_0011b491(param_1 + 0x18);
      }
      else {
        if (DAT_0012b3b0 != 1) goto LAB_0010c8a6;
        local_1188 = FUN_0011b477(param_1 + 0x18);
      }
    }
  }
  local_11a8 = local_e68;
  if (DAT_0012b3e9 != '\0') {
    uVar9 = FUN_0010c643(local_10f8,0x15,param_1);
    iVar5 = sprintf(local_11a8,"%*s ",(ulong)DAT_0012b384,uVar9);
    local_11a8 = local_11a8 + iVar5;
  }
  if (DAT_0012b3bb != '\0') {
    if (*(char *)(param_1 + 0xb8) == '\x01') {
      local_11a0 = (char *)FUN_00113d91(*(undefined8 *)(param_1 + 0x58),local_10f8,DAT_0012b3bc,
                                        0x200,DAT_0012b3c0);
    }
    else {
      local_11a0 = "?";
    }
    local_11bc = DAT_0012b388;
    iVar5 = FUN_0011574e(local_11a0,0);
    for (local_11bc = local_11bc - iVar5; pcVar1 = local_11a8, 0 < local_11bc;
        local_11bc = local_11bc + -1) {
      *local_11a8 = ' ';
      local_11a8 = local_11a8 + 1;
    }
    do {
      local_11a8 = pcVar1;
      *local_11a8 = *local_11a0;
      pcVar1 = local_11a8 + 1;
      local_11a0 = local_11a0 + 1;
    } while (*local_11a8 != '\0');
    *local_11a8 = ' ';
    local_11a8 = local_11a8 + 1;
  }
  if (*(char *)(param_1 + 0xb8) == '\x01') {
    puVar10 = (undefined1 *)FUN_00115035(*(undefined8 *)(param_1 + 0x28),local_10f8);
  }
  else {
    puVar10 = &DAT_001215ba;
  }
  iVar5 = sprintf(local_11a8,"%s %*s ",&local_1124,(ulong)DAT_0012b38c,puVar10);
  local_11a8 = local_11a8 + iVar5;
  FUN_001054cf();
  if ((((DAT_0012b022 != '\0') || (DAT_0012b023 != '\0')) || (DAT_0012b3b9 != '\0')) ||
     (DAT_0012b380 != '\0')) {
    FUN_00105453(local_e68,(long)local_11a8 - (long)local_e68);
    if (DAT_0012b022 != '\0') {
      FUN_0010c49e(*(undefined4 *)(param_1 + 0x34),DAT_0012b394,*(undefined1 *)(param_1 + 0xb8));
    }
    if (DAT_0012b023 != '\0') {
      FUN_0010c4fe(*(undefined4 *)(param_1 + 0x38),DAT_0012b398,*(undefined1 *)(param_1 + 0xb8));
    }
    if (DAT_0012b3b9 != '\0') {
      FUN_0010c49e(*(undefined4 *)(param_1 + 0x34),DAT_0012b39c,*(undefined1 *)(param_1 + 0xb8));
    }
    if (DAT_0012b380 != '\0') {
      FUN_0010c3fc(*(undefined8 *)(param_1 + 0xb0),0,DAT_0012b390);
    }
    local_11a8 = local_e68;
  }
  uVar3 = DAT_0012b3c8;
  uVar9 = DAT_0012b028;
  if ((*(char *)(param_1 + 0xb8) == '\0') ||
     (((*(uint *)(param_1 + 0x30) & 0xf000) != 0x2000 &&
      ((*(uint *)(param_1 + 0x30) & 0xf000) != 0x6000)))) {
    if (*(char *)(param_1 + 0xb8) == '\x01') {
      uVar11 = FUN_00109635(*(undefined8 *)(param_1 + 0x48));
      local_1198 = (char *)FUN_00113d91(uVar11,local_10f8,uVar3,1,uVar9);
    }
    else {
      local_1198 = "?";
    }
    local_11b8 = DAT_0012b3a8;
    iVar5 = FUN_0011574e(local_1198,0);
    for (local_11b8 = local_11b8 - iVar5; pcVar1 = local_11a8, 0 < local_11b8;
        local_11b8 = local_11b8 + -1) {
      *local_11a8 = ' ';
      local_11a8 = local_11a8 + 1;
    }
    do {
      local_11a8 = pcVar1;
      *local_11a8 = *local_1198;
      pcVar1 = local_11a8 + 1;
      local_1198 = local_1198 + 1;
    } while (*local_11a8 != '\0');
    *local_11a8 = ' ';
    local_11a8 = local_11a8 + 1;
  }
  else {
    iVar6 = DAT_0012b3a8 - (DAT_0012b3a4 + DAT_0012b3a0 + 2);
    uVar7 = gnu_dev_minor(*(ulonglong *)(param_1 + 0x40));
    uVar9 = FUN_00115035(uVar7,local_10f8);
    uVar7 = DAT_0012b3a4;
    uVar8 = gnu_dev_major(*(ulonglong *)(param_1 + 0x40));
    uVar11 = FUN_00115035(uVar8,local_1118);
    iVar5 = 0;
    if (-1 < iVar6) {
      iVar5 = iVar6;
    }
    iVar5 = sprintf(local_11a8,"%*s, %*s ",(ulong)(uint)(iVar5 + DAT_0012b3a0),uVar11,(ulong)uVar7,
                    uVar9);
    local_11a8 = local_11a8 + iVar5;
  }
  local_11b0 = 0;
  *local_11a8 = '\x01';
  if ((*(char *)(param_1 + 0xb8) == '\0') || (!bVar2)) goto LAB_0010cf05;
  lVar12 = FUN_0011b9a5(DAT_0012b438,local_1188,local_1168);
  if (lVar12 == 0) goto LAB_0010cf05;
  iVar5 = FUN_0011bc1f(DAT_0012b370,DAT_0012b378,local_1188._0_8_,local_1188._8_8_);
  if (iVar5 < 0) {
    FUN_00111ec3(&DAT_0012b370);
  }
  local_1178 = DAT_0012b370 + -0xf0c2ac;
  local_1170 = DAT_0012b378;
  iVar5 = FUN_0011bc1f(local_1178,DAT_0012b378,local_1188._0_8_,local_1188._8_8_);
  if (iVar5 < 0) {
    iVar5 = FUN_0011bc1f(local_1188._0_8_,local_1188._8_8_,DAT_0012b370,DAT_0012b378);
    if (-1 < iVar5) goto LAB_0010ceb4;
    uVar4 = 1;
  }
  else {
LAB_0010ceb4:
    uVar4 = 0;
  }
  local_11b0 = FUN_0010c25f(local_11a8,0x3e9,uVar4,local_1168,DAT_0012b438,
                            local_1188._8_8_ & 0xffffffff);
LAB_0010cf05:
  if ((local_11b0 == 0) && (*local_11a8 != '\0')) {
    if ((*(char *)(param_1 + 0xb8) == '\0') || (!bVar2)) {
      puVar10 = &DAT_001215ba;
    }
    else {
      puVar10 = (undefined1 *)FUN_00105219(local_1188._0_8_,local_10f8);
    }
    uVar7 = FUN_0010c308();
    iVar5 = sprintf(local_11a8,"%*s ",(ulong)uVar7,puVar10);
    local_11a8 = local_11a8 + iVar5;
  }
  else {
    pcVar1 = local_11a8 + local_11b0;
    local_11a8 = pcVar1 + 1;
    *pcVar1 = ' ';
  }
  FUN_00105453(local_e68,(long)local_11a8 - (long)local_e68);
  lVar12 = FUN_0010db96(param_1,0,&DAT_0012b520,(long)local_11a8 - (long)local_e68);
  if (*(int *)(param_1 + 0xa8) == 6) {
    if (*(long *)(param_1 + 8) != 0) {
      FUN_0010549e(&DAT_001215ff);
      FUN_0010db96(param_1,1,0,local_11a8 + (lVar12 - (long)local_e68) + 4);
      if (DAT_0012b3d0 != 0) {
        FUN_0010e05c(1,*(undefined4 *)(param_1 + 0xac),0);
      }
    }
  }
  else if (DAT_0012b3d0 != 0) {
    FUN_0010e05c(*(undefined1 *)(param_1 + 0xb8),*(undefined4 *)(param_1 + 0x30),
                 *(undefined4 *)(param_1 + 0xa8));
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: quote_name_buf @ 0xd10c

ulong quote_name_buf(undefined8 *param_1,ulong param_2,char *param_3,undefined8 param_4,int param_5,
                  ulong *param_6,undefined1 *param_7)

{
  ushort *puVar1;
  bool bVar2;
  bool bVar3;
  byte bVar4;
  undefined1 uVar5;
  int iVar6;
  size_t sVar7;
  ushort **ppuVar8;
  long in_FS_OFFSET;
  wchar_t local_8c;
  int local_88;
  int local_84;
  char *local_80;
  ulong local_78;
  ulong local_70;
  char *local_68;
  char *local_60;
  long local_58;
  char *local_50;
  char *local_48;
  char *local_40;
  char *local_38;
  char *local_30;
  mbstate_t local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_80 = (char *)*param_1;
  local_70 = 0;
  local_88 = FUN_00118fc3(param_4);
  if ((DAT_0012b408 == '\0') || (((local_88 != 1 && (local_88 != 2)) && (local_88 != 0)))) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  if (param_5 == 0) {
    if (bVar2) {
      local_70 = strlen(param_3);
      if (param_2 <= local_70) {
        local_80 = (char *)FUN_0011c68b(local_70 + 1);
      }
      memcpy(local_80,param_3,local_70 + 1);
      bVar3 = false;
    }
    else {
      local_70 = strlen(param_3);
      bVar3 = false;
      local_80 = param_3;
    }
  }
  else {
    local_70 = FUN_0011a389(local_80,param_2,param_3,0xffffffffffffffff,param_4);
    if (param_2 <= local_70) {
      local_80 = (char *)FUN_0011c68b(local_70 + 1);
      FUN_0011a389(local_80,local_70 + 1,param_3,0xffffffffffffffff,param_4);
    }
    if ((*param_3 == *local_80) && (sVar7 = strlen(param_3), local_70 == sVar7)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
  }
  if (bVar2) {
    sVar7 = __ctype_get_mb_cur_max();
    if (sVar7 < 2) {
      local_38 = local_80 + local_70;
      for (local_50 = local_80; local_50 < local_38; local_50 = local_50 + 1) {
        ppuVar8 = __ctype_b_loc();
        puVar1 = *ppuVar8;
        bVar4 = FUN_00104eb2((int)*local_50);
        if ((puVar1[bVar4] & 0x4000) == 0) {
          *local_50 = '?';
        }
      }
      local_78 = local_70;
    }
    else {
      local_68 = local_80;
      local_30 = local_80 + local_70;
      local_60 = local_80;
      local_78 = 0;
LAB_0010d4c6:
      if (local_68 < local_30) {
        iVar6 = (int)*local_68;
        if (iVar6 < 0x7f) {
          if (iVar6 < 0x61) {
            if (0x5f < iVar6) goto LAB_0010d3b9;
            if (iVar6 < 0x41) {
              if (iVar6 < 0x24) {
                if (iVar6 < 0x20) goto LAB_0010d3b9;
              }
              else if (0x1a < iVar6 - 0x25U) goto LAB_0010d3b9;
            }
          }
          *local_60 = *local_68;
          local_78 = local_78 + 1;
          local_68 = local_68 + 1;
          local_60 = local_60 + 1;
        }
        else {
LAB_0010d3b9:
          local_28.__count = 0;
          local_28.__value = (_union_27)0x0;
          do {
            local_58 = FUN_001150be(&local_8c,local_68,(long)local_30 - (long)local_68,&local_28);
            if (local_58 == -1) {
              *local_60 = '?';
              local_78 = local_78 + 1;
              local_68 = local_68 + 1;
              local_60 = local_60 + 1;
              break;
            }
            if (local_58 == -2) {
              local_68 = local_30;
              *local_60 = '?';
              local_78 = local_78 + 1;
              local_60 = local_60 + 1;
              break;
            }
            if (local_58 == 0) {
              local_58 = 1;
            }
            local_84 = wcwidth(local_8c);
            if (local_84 < 0) {
              local_68 = local_68 + local_58;
              *local_60 = '?';
              local_78 = local_78 + 1;
              local_60 = local_60 + 1;
            }
            else {
              for (; local_58 != 0; local_58 = local_58 + -1) {
                *local_60 = *local_68;
                local_68 = local_68 + 1;
                local_60 = local_60 + 1;
              }
              local_78 = local_78 + (long)local_84;
            }
            iVar6 = mbsinit(&local_28);
          } while (iVar6 == 0);
        }
        goto LAB_0010d4c6;
      }
      local_70 = (long)local_60 - (long)local_80;
    }
  }
  else if (param_6 != (ulong *)0x0) {
    sVar7 = __ctype_get_mb_cur_max();
    if (sVar7 < 2) {
      local_40 = local_80 + local_70;
      local_78 = 0;
      for (local_48 = local_80; local_48 < local_40; local_48 = local_48 + 1) {
        ppuVar8 = __ctype_b_loc();
        puVar1 = *ppuVar8;
        bVar4 = FUN_00104eb2((int)*local_48);
        if ((puVar1[bVar4] & 0x4000) != 0) {
          local_78 = local_78 + 1;
        }
      }
    }
    else {
      iVar6 = FUN_00115784(local_80,local_70,0);
      local_78 = (ulong)iVar6;
    }
  }
  if (((DAT_0012b341 == '\0') || (DAT_0012b340 == '\0')) || (bVar3)) {
    uVar5 = 0;
  }
  else {
    uVar5 = 1;
  }
  *param_7 = uVar5;
  if (param_6 != (ulong *)0x0) {
    *param_6 = local_78;
  }
  *param_1 = local_80;
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_70;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: quote_name_width @ 0xd671

long quote_name_width(undefined1 *param_1,undefined8 param_2,undefined4 param_3)

{
  long in_FS_OFFSET;
  byte local_2029;
  undefined1 *local_2028;
  long local_2020;
  undefined1 local_2018 [8200];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_2028 = local_2018;
  FUN_0010d10c(&local_2028,0x2000,param_1,param_2,param_3,&local_2020,&local_2029);
  if ((local_2028 != local_2018) && (param_1 != local_2028)) {
    free(local_2028);
  }
  local_2020 = local_2020 + (ulong)local_2029;
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_2020;
}




// Function: file_escape @ 0xd775

char * file_escape(char *param_1,char param_2)

{
  byte bVar1;
  int iVar2;
  size_t sVar3;
  char *pcVar4;
  char *local_20;
  char *local_18;
  
  sVar3 = strlen(param_1);
  pcVar4 = (char *)FUN_0011c804(3,sVar3 + 1);
  local_20 = param_1;
  local_18 = pcVar4;
  while (*local_20 != '\0') {
    if ((param_2 == '\0') || (*local_20 != '/')) {
      bVar1 = FUN_00104eb2((int)*local_20);
      if ((&DAT_0012b640)[(int)(uint)bVar1] == '\0') {
        bVar1 = FUN_00104eb2((int)*local_20);
        iVar2 = sprintf(local_18,"%%%02x",(ulong)bVar1);
        local_18 = local_18 + iVar2;
        local_20 = local_20 + 1;
      }
      else {
        *local_18 = *local_20;
        local_20 = local_20 + 1;
        local_18 = local_18 + 1;
      }
    }
    else {
      *local_18 = '/';
      local_20 = local_20 + 1;
      local_18 = local_18 + 1;
    }
  }
  *local_18 = '\0';
  return pcVar4;
}




// Function: quote_name @ 0xd87f

long quote_name(char *param_1,undefined8 param_2,undefined4 param_3,long param_4,char param_5,
                 long param_6,long param_7)

{
  undefined *puVar1;
  long in_FS_OFFSET;
  byte local_203a;
  byte local_2039;
  char *local_2038;
  long local_2030;
  void *local_2028;
  char *local_2020;
  char local_2018 [8200];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_2038 = local_2018;
  local_2030 = FUN_0010d10c(&local_2038,0x2000,param_1,param_2,param_3,0,&local_203a);
  if ((local_203a != 0) && (param_5 != '\0')) {
    FUN_00105422(0x20);
  }
  if (param_4 != 0) {
    FUN_0010e0a3(param_4);
  }
  local_2039 = 0;
  if (param_7 != 0) {
    if (((DAT_0012b341 != '\0') && (DAT_0012b340 != '\0')) && (local_203a != 1)) {
      local_2039 = 1;
      putchar_unlocked((int)*local_2038);
    }
    local_2028 = (void *)FUN_0010d775(DAT_0012b360,0);
    local_2020 = (char *)FUN_0010d775(param_7,1);
    if (*local_2020 == '/') {
      puVar1 = &DAT_0011f27b;
    }
    else {
      puVar1 = &DAT_0012160b;
    }
    printf("\x1b]8;;file://%s%s%s\a",local_2028,puVar1,local_2020);
    free(local_2028);
    free(local_2020);
  }
  if (param_6 != 0) {
    FUN_001054f4(param_6);
  }
  fwrite_unlocked(local_2038 + local_2039,1,local_2030 - (int)((uint)local_2039 * 2),stdout);
  DAT_0012b500 = local_2030 + DAT_0012b500;
  if (param_6 != 0) {
    FUN_001054f4(param_6);
  }
  if (param_7 != 0) {
    fputs_unlocked("\x1b]8;;\a",stdout);
    if (local_2039 != 0) {
      putchar_unlocked((int)local_2038[local_2030 + -1]);
    }
  }
  if ((local_2038 != local_2018) && (param_1 != local_2038)) {
    free(local_2038);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_2030 + (ulong)local_203a;
}




// Function: print_name_with_quoting @ 0xdb96

long print_name_with_quoting(undefined8 *param_1,char param_2,undefined8 param_3,ulong param_4)

{
  char cVar1;
  bool bVar2;
  undefined8 uVar3;
  long lVar4;
  ulong uVar5;
  
  if (param_2 == '\0') {
    uVar3 = *param_1;
  }
  else {
    uVar3 = param_1[1];
  }
  if (DAT_0012b3d4 == '\0') {
    lVar4 = 0;
  }
  else {
    lVar4 = FUN_0010e101(param_1,param_2);
  }
  if (DAT_0012b3d4 == '\0') {
LAB_0010dc1d:
    bVar2 = false;
  }
  else {
    if (lVar4 == 0) {
      cVar1 = FUN_00106325(4);
      if (cVar1 == '\0') goto LAB_0010dc1d;
    }
    bVar2 = true;
  }
  lVar4 = FUN_0010d87f(uVar3,DAT_0012b410,*(undefined4 *)((long)param_1 + 0xc4),lVar4,
                       param_2 == '\0',param_3,param_1[2]);
  FUN_0010648c();
  if (bVar2) {
    FUN_0010dce3();
    if (DAT_0012b430 != 0) {
      uVar5 = (lVar4 + param_4) - 1;
      if (param_4 / DAT_0012b430 != uVar5 / DAT_0012b430) {
        FUN_0010e4ca(&DAT_0012b1f0,DAT_0012b430,uVar5 % DAT_0012b430);
      }
    }
  }
  return lVar4;
}




// Function: prep_non_filename_text @ 0xdce3

void prep_non_filename_text(void)

{
  if (DAT_0012b0a8 == 0) {
    FUN_0010e4ca(&DAT_0012b080);
    FUN_0010e4ca(&DAT_0012b0b0);
    FUN_0010e4ca(&DAT_0012b090);
  }
  else {
    FUN_0010e4ca(&DAT_0012b0a0);
  }
  return;
}




// Function: print_file_name_and_frills @ 0xdd38

long print_file_name_and_frills(long param_1,undefined8 param_2)

{
  byte bVar1;
  uint uVar2;
  undefined8 uVar3;
  undefined1 *puVar4;
  ulong uVar5;
  long in_FS_OFFSET;
  long local_2b0;
  undefined1 local_2a8 [664];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_001063f3();
  if (DAT_0012b3e9 != '\0') {
    uVar3 = FUN_0010c643(local_2a8,0x28c,param_1);
    uVar2 = DAT_0012b384;
    if (DAT_0012b3ac == 4) {
      uVar2 = 0;
    }
    printf("%*s ",(ulong)uVar2,uVar3);
  }
  if (DAT_0012b3bb != '\0') {
    if (*(char *)(param_1 + 0xb8) == '\x01') {
      puVar4 = (undefined1 *)
               FUN_00113d91(*(undefined8 *)(param_1 + 0x58),local_2a8,DAT_0012b3bc,0x200,
                            DAT_0012b3c0);
    }
    else {
      puVar4 = &DAT_001215ba;
    }
    if (DAT_0012b3ac == 4) {
      uVar5 = 0;
    }
    else {
      uVar5 = (ulong)DAT_0012b388;
    }
    printf("%*s ",uVar5,puVar4);
  }
  if (DAT_0012b380 != '\0') {
    uVar2 = DAT_0012b390;
    if (DAT_0012b3ac == 4) {
      uVar2 = 0;
    }
    printf("%*s ",(ulong)uVar2,*(undefined8 *)(param_1 + 0xb0));
  }
  local_2b0 = FUN_0010db96(param_1,0,0,param_2);
  if (DAT_0012b3d0 != 0) {
    bVar1 = FUN_0010e05c(*(undefined1 *)(param_1 + 0xb8),*(undefined4 *)(param_1 + 0x30),
                         *(undefined4 *)(param_1 + 0xa8));
    local_2b0 = local_2b0 + (ulong)bVar1;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_2b0;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: get_type_indicator @ 0xdf23

undefined1 get_type_indicator(char param_1,uint param_2,int param_3)

{
  bool bVar1;
  undefined1 local_9;
  
  if (param_1 == '\0') {
    bVar1 = param_3 == 5;
  }
  else {
    bVar1 = (param_2 & 0xf000) == 0x8000;
  }
  if (bVar1) {
    if (((param_1 == '\0') || (DAT_0012b3d0 != 3)) || ((param_2 & 0x49) == 0)) {
      local_9 = 0;
    }
    else {
      local_9 = 0x2a;
    }
  }
  else {
    if (param_1 == '\0') {
      if ((param_3 == 3) || (param_3 == 9)) {
        bVar1 = true;
      }
      else {
        bVar1 = false;
      }
    }
    else {
      bVar1 = (param_2 & 0xf000) == 0x4000;
    }
    if (bVar1) {
      local_9 = 0x2f;
    }
    else if (DAT_0012b3d0 == 1) {
      local_9 = 0;
    }
    else {
      if (param_1 == '\0') {
        bVar1 = param_3 == 6;
      }
      else {
        bVar1 = (param_2 & 0xf000) == 0xa000;
      }
      if (bVar1) {
        local_9 = 0x40;
      }
      else {
        if (param_1 == '\0') {
          bVar1 = param_3 == 1;
        }
        else {
          bVar1 = (param_2 & 0xf000) == 0x1000;
        }
        if (bVar1) {
          local_9 = 0x7c;
        }
        else {
          if (param_1 == '\0') {
            bVar1 = param_3 == 7;
          }
          else {
            bVar1 = (param_2 & 0xf000) == 0xc000;
          }
          if (bVar1) {
            local_9 = 0x3d;
          }
          else {
            local_9 = 0;
          }
        }
      }
    }
  }
  return local_9;
}




// Function: print_type_indicator @ 0xe05c

bool print_type_indicator(undefined1 param_1,undefined4 param_2,undefined4 param_3)

{
  char cVar1;
  
  cVar1 = FUN_0010df23(param_1,param_2,param_3);
  if (cVar1 != '\0') {
    FUN_00105422((int)cVar1);
  }
  return cVar1 != '\0';
}




// Function: print_color_indicator @ 0xe0a3

bool print_color_indicator(long param_1)

{
  char cVar1;
  
  if (param_1 != 0) {
    cVar1 = FUN_00106325(4);
    if (cVar1 != '\0') {
      FUN_001063ca();
    }
    FUN_0010e4ca(&DAT_0012b080);
    FUN_0010e4ca(param_1);
    FUN_0010e4ca(&DAT_0012b090);
  }
  return param_1 != 0;
}




// Function: get_color_indicator @ 0xe101

ulong * get_color_indicator(undefined8 *param_1,char param_2)

{
  char cVar1;
  int iVar2;
  size_t sVar3;
  uint local_34;
  uint local_30;
  uint local_2c;
  ulong *local_28;
  char *local_20;
  
  if (param_2 == '\0') {
    local_20 = (char *)*param_1;
    local_30 = FUN_001053e7(param_1);
    local_2c = (uint)*(byte *)((long)param_1 + 0xb9);
  }
  else {
    local_20 = (char *)param_1[1];
    local_30 = *(uint *)((long)param_1 + 0xac);
    if (*(char *)((long)param_1 + 0xb9) == '\0') {
      local_2c = 0xffffffff;
    }
    else {
      local_2c = 0;
    }
  }
  if ((local_2c == 0xffffffff) && (cVar1 = FUN_00106325(0xc), cVar1 != '\0')) {
    local_34 = 0xc;
  }
  else if (*(char *)(param_1 + 0x17) == '\x01') {
    if ((local_30 & 0xf000) == 0x8000) {
      local_34 = 5;
      if (((local_30 & 0x800) == 0) || (cVar1 = FUN_00106325(0x10), cVar1 == '\0')) {
        if (((local_30 & 0x400) == 0) || (cVar1 = FUN_00106325(0x11), cVar1 == '\0')) {
          cVar1 = FUN_00106325(0x15);
          if ((cVar1 == '\0') || (*(char *)(param_1 + 0x18) == '\0')) {
            if (((local_30 & 0x49) == 0) || (cVar1 = FUN_00106325(0xe), cVar1 == '\0')) {
              if ((1 < (ulong)param_1[5]) && (cVar1 = FUN_00106325(0x16), cVar1 != '\0')) {
                local_34 = 0x16;
              }
            }
            else {
              local_34 = 0xe;
            }
          }
          else {
            local_34 = 0x15;
          }
        }
        else {
          local_34 = 0x11;
        }
      }
      else {
        local_34 = 0x10;
      }
    }
    else if ((local_30 & 0xf000) == 0x4000) {
      local_34 = 6;
      if ((((local_30 & 0x200) == 0) || ((local_30 & 2) == 0)) ||
         (cVar1 = FUN_00106325(0x14), cVar1 == '\0')) {
        if (((local_30 & 2) == 0) || (cVar1 = FUN_00106325(0x13), cVar1 == '\0')) {
          if (((local_30 & 0x200) != 0) && (cVar1 = FUN_00106325(0x12), cVar1 != '\0')) {
            local_34 = 0x12;
          }
        }
        else {
          local_34 = 0x13;
        }
      }
      else {
        local_34 = 0x14;
      }
    }
    else if ((local_30 & 0xf000) == 0xa000) {
      local_34 = 7;
    }
    else if ((local_30 & 0xf000) == 0x1000) {
      local_34 = 8;
    }
    else if ((local_30 & 0xf000) == 0xc000) {
      local_34 = 9;
    }
    else if ((local_30 & 0xf000) == 0x6000) {
      local_34 = 10;
    }
    else if ((local_30 & 0xf000) == 0x2000) {
      local_34 = 0xb;
    }
    else {
      local_34 = 0xd;
    }
  }
  else {
    local_34 = *(uint *)(&DAT_0012b040 + (ulong)*(uint *)(param_1 + 0x15) * 4);
  }
  local_28 = (ulong *)0x0;
  if (local_34 == 5) {
    sVar3 = strlen(local_20);
    local_28 = DAT_0012b3d8;
    while ((local_28 != (ulong *)0x0 &&
           ((sVar3 < *local_28 ||
            (iVar2 = FUN_001102b5(local_20 + (sVar3 - *local_28),local_28[1],*local_28), iVar2 != 0)
            )))) {
      local_28 = (ulong *)local_28[4];
    }
  }
  if (((local_34 == 7) && (local_2c == 0)) &&
     ((DAT_0012b358 != '\0' || (cVar1 = FUN_00106325(0xd), cVar1 != '\0')))) {
    local_34 = 0xd;
  }
  if (local_28 == (ulong *)0x0) {
    local_28 = &DAT_0012b080 + (ulong)local_34 * 2;
  }
  else {
    local_28 = local_28 + 2;
  }
  if (local_28[1] == 0) {
    local_28 = (ulong *)0x0;
  }
  return local_28;
}




// Function: put_indicator @ 0xe4ca

void put_indicator(size_t *param_1)

{
  __pid_t _Var1;
  
  if (DAT_0012b3d6 != '\x01') {
    DAT_0012b3d6 = '\x01';
    _Var1 = tcgetpgrp(1);
    if (-1 < _Var1) {
      FUN_00106858();
    }
    FUN_0010dce3();
  }
  fwrite_unlocked((void *)param_1[1],*param_1,1,stdout);
  return;
}




// Function: length_of_file_name_and_frills @ 0xe530

long length_of_file_name_and_frills(long param_1)

{
  char cVar1;
  char *pcVar2;
  size_t sVar3;
  long lVar4;
  long in_FS_OFFSET;
  long local_2b0;
  undefined1 local_2a8 [664];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_2b0 = 0;
  if (DAT_0012b3e9 != '\0') {
    if (DAT_0012b3ac == 4) {
      pcVar2 = (char *)FUN_00115035(*(undefined8 *)(param_1 + 0x20),local_2a8);
      sVar3 = strlen(pcVar2);
    }
    else {
      sVar3 = (size_t)DAT_0012b384;
    }
    local_2b0 = sVar3 + 1;
  }
  if (DAT_0012b3bb != '\0') {
    if (DAT_0012b3ac == 4) {
      if (*(char *)(param_1 + 0xb8) == '\x01') {
        pcVar2 = (char *)FUN_00113d91(*(undefined8 *)(param_1 + 0x58),local_2a8,DAT_0012b3bc,0x200,
                                      DAT_0012b3c0);
      }
      else {
        pcVar2 = "?";
      }
      sVar3 = strlen(pcVar2);
    }
    else {
      sVar3 = (size_t)DAT_0012b388;
    }
    local_2b0 = local_2b0 + sVar3 + 1;
  }
  if (DAT_0012b380 != '\0') {
    if (DAT_0012b3ac == 4) {
      sVar3 = strlen(*(char **)(param_1 + 0xb0));
    }
    else {
      sVar3 = (size_t)DAT_0012b390;
    }
    local_2b0 = local_2b0 + sVar3 + 1;
  }
  lVar4 = FUN_0010b119(param_1);
  local_2b0 = local_2b0 + lVar4;
  if (DAT_0012b3d0 != 0) {
    cVar1 = FUN_0010df23(*(undefined1 *)(param_1 + 0xb8),*(undefined4 *)(param_1 + 0x30),
                         *(undefined4 *)(param_1 + 0xa8));
    local_2b0 = local_2b0 + (ulong)(cVar1 != '\0');
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_2b0;
}




// Function: print_many_per_line @ 0xe708

void print_many_per_line(void)

{
  undefined8 uVar1;
  long lVar2;
  ulong uVar3;
  long lVar4;
  long lVar5;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined8 local_40;
  
  uVar3 = FUN_0010ee73(1);
  lVar4 = uVar3 * 0x18 + DAT_0012b4f0;
  uVar3 = (ulong)(DAT_0012b338 % uVar3 != 0) + DAT_0012b338 / uVar3;
  for (local_58 = 0; local_58 < uVar3; local_58 = local_58 + 1) {
    local_50 = 0;
    local_48 = local_58;
    local_40 = 0;
    while( true ) {
      uVar1 = *(undefined8 *)(local_48 * 8 + DAT_0012b348);
      lVar5 = FUN_0010e530(uVar1);
      lVar2 = *(long *)(local_50 * 8 + *(long *)(lVar4 + -8));
      FUN_0010dd38(uVar1,local_40);
      local_48 = local_48 + uVar3;
      if (DAT_0012b338 <= local_48) break;
      FUN_0010eaef(lVar5 + local_40,local_40 + lVar2);
      local_40 = local_40 + lVar2;
      local_50 = local_50 + 1;
    }
    putchar_unlocked((int)DAT_0012b030);
  }
  return;
}




// Function: print_horizontal @ 0xe86a

void print_horizontal(void)

{
  undefined8 uVar1;
  ulong uVar2;
  long lVar3;
  ulong local_48;
  long local_40;
  long local_38;
  long local_30;
  
  local_40 = 0;
  uVar2 = FUN_0010ee73(0);
  lVar3 = uVar2 * 0x18 + -0x18 + DAT_0012b4f0;
  uVar1 = *DAT_0012b348;
  local_38 = FUN_0010e530(uVar1);
  local_30 = **(long **)(lVar3 + 0x10);
  FUN_0010dd38(uVar1,0);
  for (local_48 = 1; local_48 < DAT_0012b338; local_48 = local_48 + 1) {
    if (local_48 % uVar2 == 0) {
      putchar_unlocked((int)DAT_0012b030);
      local_40 = 0;
    }
    else {
      FUN_0010eaef(local_38 + local_40,local_40 + local_30);
      local_40 = local_40 + local_30;
    }
    uVar1 = DAT_0012b348[local_48];
    FUN_0010dd38(uVar1,local_40);
    local_38 = FUN_0010e530(uVar1);
    local_30 = *(long *)((local_48 % uVar2) * 8 + *(long *)(lVar3 + 0x10));
  }
  putchar_unlocked((int)DAT_0012b030);
  return;
}




// Function: print_with_separator @ 0xe9d6

void print_with_separator(char param_1)

{
  undefined8 uVar1;
  long lVar2;
  undefined1 local_29;
  undefined8 local_28;
  undefined8 local_20;
  
  local_20 = 0;
  for (local_28 = 0; local_28 < DAT_0012b338; local_28 = local_28 + 1) {
    uVar1 = *(undefined8 *)(local_28 * 8 + DAT_0012b348);
    if (DAT_0012b430 == 0) {
      lVar2 = 0;
    }
    else {
      lVar2 = FUN_0010e530(uVar1);
    }
    if (local_28 != 0) {
      if ((DAT_0012b430 == 0) ||
         ((lVar2 + local_20 + 2 < DAT_0012b430 && (local_20 <= -lVar2 - 3U)))) {
        local_20 = local_20 + 2;
        local_29 = ' ';
      }
      else {
        local_20 = 0;
        local_29 = DAT_0012b030;
      }
      putchar_unlocked((int)param_1);
      putchar_unlocked((int)local_29);
    }
    FUN_0010dd38(uVar1,local_20);
    local_20 = local_20 + lVar2;
  }
  putchar_unlocked((int)DAT_0012b030);
  return;
}




// Function: indent @ 0xeaef

void indent(ulong param_1,ulong param_2)

{
  undefined8 local_10;
  
  local_10 = param_1;
  while (local_10 < param_2) {
    if ((DAT_0012b420 == 0) || (param_2 / DAT_0012b420 <= (local_10 + 1) / DAT_0012b420)) {
      putchar_unlocked(0x20);
      local_10 = local_10 + 1;
    }
    else {
      putchar_unlocked(9);
      local_10 = local_10 + (DAT_0012b420 - local_10 % DAT_0012b420);
    }
  }
  return;
}




// Function: attach @ 0xeb94

void attach(char *param_1,char *param_2,char *param_3)

{
  char *local_30;
  char *local_20;
  char *local_10;
  
  local_20 = param_1;
  local_10 = param_2;
  if ((*param_2 != '.') ||
     (local_30 = param_3, local_20 = param_1, local_10 = param_2, param_2[1] != '\0')) {
    while (*local_10 != '\0') {
      *local_20 = *local_10;
      local_20 = local_20 + 1;
      local_10 = local_10 + 1;
    }
    local_30 = param_3;
    if ((param_2 < local_10) && (local_30 = param_3, local_10[-1] != '/')) {
      *local_20 = '/';
      local_30 = param_3;
      local_20 = local_20 + 1;
    }
  }
  while (*local_30 != '\0') {
    *local_20 = *local_30;
    local_30 = local_30 + 1;
    local_20 = local_20 + 1;
  }
  *local_20 = '\0';
  return;
}




// Function: init_column_info @ 0xec50

void init_column_info(ulong param_1)

{
  ulong uVar1;
  ulong uVar2;
  undefined8 local_40;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 local_28;
  
  if (DAT_0012c440 < param_1) {
    if ((DAT_0012b4f8 == 0) || (param_1 < DAT_0012b4f8 >> 1)) {
      DAT_0012b4f0 = FUN_0011c638(DAT_0012b4f0,param_1,0x30);
      local_38 = param_1 * 2;
    }
    else {
      DAT_0012b4f0 = FUN_0011c638(DAT_0012b4f0,DAT_0012b4f8,0x18);
      local_38 = DAT_0012b4f8;
    }
    uVar1 = local_38 + DAT_0012c440 + 1;
    uVar2 = uVar1 * (local_38 - DAT_0012c440);
    if ((uVar1 < local_38) || (uVar1 != uVar2 / (local_38 - DAT_0012c440))) {
      FUN_0011ccc3();
    }
    local_30 = FUN_0011c804(uVar2 >> 1,8);
    for (local_40 = DAT_0012c440; local_40 < local_38; local_40 = local_40 + 1) {
      *(long *)(DAT_0012b4f0 + local_40 * 0x18 + 0x10) = local_30;
      local_30 = local_30 + (local_40 + 1) * 8;
    }
    DAT_0012c440 = local_38;
  }
  for (local_40 = 0; local_40 < param_1; local_40 = local_40 + 1) {
    *(undefined1 *)(local_40 * 0x18 + DAT_0012b4f0) = 1;
    *(ulong *)(DAT_0012b4f0 + local_40 * 0x18 + 8) = (local_40 + 1) * 3;
    for (local_28 = 0; local_28 <= local_40; local_28 = local_28 + 1) {
      *(undefined8 *)(local_28 * 8 + *(long *)(local_40 * 0x18 + DAT_0012b4f0 + 0x10)) = 3;
    }
  }
  return;
}




// Function: calculate_columns @ 0xee73

ulong calculate_columns(char param_1)

{
  long lVar1;
  ulong uVar2;
  ulong uVar3;
  long lVar4;
  ulong local_48;
  ulong local_40;
  ulong local_38;
  
  local_40 = DAT_0012b338;
  if ((DAT_0012b4f8 != 0) && (DAT_0012b4f8 < DAT_0012b338)) {
    local_40 = DAT_0012b4f8;
  }
  FUN_0010ec50(local_40);
  for (local_48 = 0; local_48 < DAT_0012b338; local_48 = local_48 + 1) {
    lVar1 = FUN_0010e530(*(undefined8 *)(local_48 * 8 + DAT_0012b348));
    for (local_38 = 0; local_38 < local_40; local_38 = local_38 + 1) {
      if (*(char *)(local_38 * 0x18 + DAT_0012b4f0) != '\0') {
        if (param_1 == '\0') {
          uVar2 = local_48 % (local_38 + 1);
        }
        else {
          uVar2 = local_48 / ((local_38 + DAT_0012b338) / (local_38 + 1));
        }
        if (uVar2 == local_38) {
          lVar4 = 0;
        }
        else {
          lVar4 = 2;
        }
        uVar3 = lVar1 + lVar4;
        if (*(ulong *)(uVar2 * 8 + *(long *)(local_38 * 0x18 + DAT_0012b4f0 + 0x10)) < uVar3) {
          *(ulong *)(local_38 * 0x18 + DAT_0012b4f0 + 8) =
               *(long *)(local_38 * 0x18 + DAT_0012b4f0 + 8) +
               (uVar3 - *(long *)(uVar2 * 8 + *(long *)(local_38 * 0x18 + DAT_0012b4f0 + 0x10)));
          *(ulong *)(*(long *)(local_38 * 0x18 + DAT_0012b4f0 + 0x10) + uVar2 * 8) = uVar3;
          *(bool *)(local_38 * 0x18 + DAT_0012b4f0) =
               *(ulong *)(local_38 * 0x18 + DAT_0012b4f0 + 8) < DAT_0012b430;
        }
      }
    }
  }
  while ((1 < local_40 && (*(char *)(local_40 * 0x18 + -0x18 + DAT_0012b4f0) == '\0'))) {
    local_40 = local_40 - 1;
  }
  return local_40;
}




// Function: usage @ 0xf135

void usage(int param_1)

{
  FILE *pFVar1;
  undefined8 uVar2;
  char *pcVar3;
  undefined *puVar4;
  
  uVar2 = DAT_0012c478;
  if (param_1 == 0) {
    pcVar3 = (char *)gettext("Usage: %s [OPTION]... [FILE]...\n");
    printf(pcVar3,uVar2);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "List information about the FILEs (the current directory by default).\nSort entries alphabetically if none of -cftuvSUX nor --sort is specified.\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    FUN_00104f20();
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -a, --all                  do not ignore entries starting with .\n  -A, --almost-all           do not list implied . and ..\n      --author               with -l, print the author of each file\n  -b, --escape               print C-style escapes for nongraphic characters\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "      --block-size=SIZE      with -l, scale sizes by SIZE when printing them;\n                             e.g., \'--block-size=M\'; see SIZE format below\n\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -B, --ignore-backups       do not list implied entries ending with ~\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -c                         with -lt: sort by, and show, ctime (time of last\n                             modification of file status information);\n                             with -l: show ctime and sort by name;\n                             otherwise: sort by ctime, newest first\n\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -C                         list entries by columns\n      --color[=WHEN]         color the output WHEN; more info below\n  -d, --directory            list directories themselves, not their contents\n  -D, --dired                generate output designed for Emacs\' dired mode\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -f                         list all entries in directory order\n  -F, --classify[=WHEN]      append indicator (one of */=>@|) to entries WHEN\n      --file-type            likewise, except do not append \'*\'\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "      --format=WORD          across -x, commas -m, horizontal -x, long -l,\n                             single-column -1, verbose -l, vertical -C\n\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("      --full-time            like -l --time-style=full-iso\n");
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("  -g                         like -l, but do not list owner\n");
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "      --group-directories-first\n                             group directories before files;\n                             can be augmented with a --sort option, but any\n                             use of --sort=none (-U) disables grouping\n\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -G, --no-group             in a long listing, don\'t print group names\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -h, --human-readable       with -l and -s, print sizes like 1K 234M 2G etc.\n      --si                   likewise, but use powers of 1000 not 1024\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -H, --dereference-command-line\n                             follow symbolic links listed on the command line\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "      --dereference-command-line-symlink-to-dir\n                             follow each command line symbolic link\n                             that points to a directory\n\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "      --hide=PATTERN         do not list implied entries matching shell PATTERN\n                             (overridden by -a or -A)\n\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("      --hyperlink[=WHEN]     hyperlink file names WHEN\n");
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "      --indicator-style=WORD\n                             append indicator with style WORD to entry names:\n                             none (default), slash (-p),\n                             file-type (--file-type), classify (-F)\n\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -i, --inode                print the index number of each file\n  -I, --ignore=PATTERN       do not list implied entries matching shell PATTERN\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -k, --kibibytes            default to 1024-byte blocks for file system usage;\n                             used only with -s and per directory totals\n\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("  -l                         use a long listing format\n");
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -L, --dereference          when showing file information for a symbolic\n                             link, show information for the file the link\n                             references rather than for the link itself\n\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -m                         fill width with a comma separated list of entries\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -n, --numeric-uid-gid      like -l, but list numeric user and group IDs\n  -N, --literal              print entry names without quoting\n  -o                         like -l, but do not list group information\n  -p, --indicator-style=slash\n                             append / indicator to directories\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -q, --hide-control-chars   print ? instead of nongraphic characters\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "      --show-control-chars   show nongraphic characters as-is (the default,\n                             unless program is \'ls\' and output is a terminal)\n\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("  -Q, --quote-name           enclose entry names in double quotes\n");
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "      --quoting-style=WORD   use quoting style WORD for entry names:\n                             literal, locale, shell, shell-always,\n                             shell-escape, shell-escape-always, c, escape\n                             (overrides QUOTING_STYLE environment variable)\n\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -r, --reverse              reverse order while sorting\n  -R, --recursive            list subdirectories recursively\n  -s, --size                 print the allocated size of each file, in blocks\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("  -S                         sort by file size, largest first\n");
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "      --sort=WORD            sort by WORD instead of name: none (-U), size (-S),\n                             time (-t), version (-v), extension (-X), width\n\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "      --time=WORD            change the default of using modification times;\n                               access time (-u): atime, access, use;\n                               change time (-c): ctime, status;\n                               birth time: birth, creation;\n                             with -l, WORD determines which time to show;\n                             with --sort=time, sort by WORD (newest first)\n\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "      --time-style=TIME_STYLE\n                             time/date format with -l; see TIME_STYLE below\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -t                         sort by time, newest first; see --time\n  -T, --tabsize=COLS         assume tab stops at each COLS instead of 8\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -u                         with -lt: sort by, and show, access time;\n                             with -l: show access time and sort by name;\n                             otherwise: sort by access time, newest first\n\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -U                         do not sort; list entries in directory order\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -v                         natural sort of (version) numbers within text\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "  -w, --width=COLS           set output width to COLS.  0 means no limit\n  -x                         list entries by lines instead of by columns\n  -X                         sort alphabetically by entry extension\n  -Z, --context              print any security context of each file\n      --zero                 end each output line with NUL, not newline\n  -1                         list one file per line\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("      --help        display this help and exit\n");
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext("      --version     output version information and exit\n");
    fputs_unlocked(pcVar3,pFVar1);
    FUN_00104f51();
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "\nThe TIME_STYLE argument can be full-iso, long-iso, iso, locale, or +FORMAT.\nFORMAT is interpreted like in date(1).  If FORMAT is FORMAT1<newline>FORMAT2,\nthen FORMAT1 applies to non-recent files and FORMAT2 to recent files.\nTIME_STYLE prefixed with \'posix-\' takes effect only outside the POSIX locale.\nAlso the TIME_STYLE environment variable sets the default style to use.\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "\nThe WHEN argument defaults to \'always\' and can also be \'auto\' or \'never\'.\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "\nUsing color to distinguish file types is disabled both by default and\nwith --color=never.  With --color=auto, ls emits color codes only when\nstandard output is connected to a terminal.  The LS_COLORS environment\nvariable can change the settings.  Use the dircolors(1) command to set it.\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    pFVar1 = stdout;
    pcVar3 = (char *)gettext(
                            "\nExit status:\n 0  if OK,\n 1  if minor problems (e.g., cannot access subdirectory),\n 2  if serious trouble (e.g., cannot access command-line argument).\n"
                            );
    fputs_unlocked(pcVar3,pFVar1);
    if (DAT_0012b210 == 1) {
      puVar4 = &DAT_00120bb7;
    }
    else if (DAT_0012b210 == 2) {
      puVar4 = &DAT_00120bae;
    }
    else {
      puVar4 = &DAT_00120bb2;
    }
    FUN_00104f82(puVar4);
  }
  else {
    pcVar3 = (char *)gettext("Try \'%s --help\' for more information.\n");
    fprintf(stderr,pcVar3,uVar2);
  }
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: hash_get_n_buckets @ 0x11fd6

undefined8 hash_get_n_buckets(long param_1)

{
  return *(undefined8 *)(param_1 + 0x10);
}




// Function: hash_get_n_buckets_used @ 0x11fec

undefined8 hash_get_n_buckets_used(long param_1)

{
  return *(undefined8 *)(param_1 + 0x18);
}




// Function: hash_get_n_entries @ 0x12002

undefined8 hash_get_n_entries(long param_1)

{
  return *(undefined8 *)(param_1 + 0x20);
}




// Function: hash_get_max_bucket_length @ 0x12018

ulong hash_get_max_bucket_length(undefined8 *param_1)

{
  long *local_28;
  ulong local_20;
  long *local_18;
  ulong local_10;
  
  local_20 = 0;
  for (local_28 = (long *)*param_1; local_28 < (long *)param_1[1]; local_28 = local_28 + 2) {
    if (*local_28 != 0) {
      local_18 = local_28;
      local_10 = 1;
      while (local_18 = (long *)local_18[1], local_18 != (long *)0x0) {
        local_10 = local_10 + 1;
      }
      if (local_20 < local_10) {
        local_20 = local_10;
      }
    }
  }
  return local_20;
}




// Function: hash_table_ok @ 0x1209a

undefined8 hash_table_ok(undefined8 *param_1)

{
  undefined8 uVar1;
  long *local_28;
  long local_20;
  long local_18;
  long *local_10;
  
  local_20 = 0;
  local_18 = 0;
  for (local_28 = (long *)*param_1; local_28 < (long *)param_1[1]; local_28 = local_28 + 2) {
    if (*local_28 != 0) {
      local_10 = local_28;
      local_20 = local_20 + 1;
      do {
        local_18 = local_18 + 1;
        local_10 = (long *)local_10[1];
      } while (local_10 != (long *)0x0);
    }
  }
  if ((local_20 == param_1[3]) && (local_18 == param_1[4])) {
    uVar1 = 1;
  }
  else {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: hash_print_statistics @ 0x12138

void hash_print_statistics(undefined8 param_1,FILE *param_2)

{
  undefined8 uVar1;
  ulong uVar2;
  ulong uVar3;
  undefined8 uVar4;
  
  uVar1 = FUN_00112002(param_1);
  uVar2 = FUN_00111fd6(param_1);
  uVar3 = FUN_00111fec(param_1);
  uVar4 = FUN_00112018(param_1);
  fprintf(param_2,"# entries:         %lu\n",uVar1);
  fprintf(param_2,"# buckets:         %lu\n",uVar2);
  fprintf(param_2,"# buckets used:    %lu (%.2f%%)\n",(DAT_001234f8 * (double)uVar3) / (double)uVar2
          ,uVar3);
  fprintf(param_2,"max bucket length: %lu\n",uVar4);
  return;
}




// Function: safe_hasher @ 0x1227f

long safe_hasher(long *param_1,undefined8 param_2)

{
  ulong uVar1;
  
  uVar1 = (*(code *)param_1[6])(param_2,param_1[2]);
  if ((ulong)param_1[2] <= uVar1) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  return uVar1 * 0x10 + *param_1;
}




// Function: hash_lookup @ 0x122da

long hash_lookup(long param_1,long param_2)

{
  char cVar1;
  long *local_18;
  
  local_18 = (long *)FUN_0011227f(param_1,param_2);
  if (*local_18 != 0) {
    for (; local_18 != (long *)0x0; local_18 = (long *)local_18[1]) {
      if ((param_2 == *local_18) ||
         (cVar1 = (**(code **)(param_1 + 0x38))(param_2,*local_18), cVar1 != '\0')) {
        return *local_18;
      }
    }
  }
  return 0;
}




// Function: hash_get_first @ 0x12371

long hash_get_first(undefined8 *param_1)

{
  long lVar1;
  long *local_10;
  
  if (param_1[4] == 0) {
    lVar1 = 0;
  }
  else {
    local_10 = (long *)*param_1;
    while( true ) {
      if ((long *)param_1[1] <= local_10) {
                    /* WARNING: Subroutine does not return */
        abort();
      }
      if (*local_10 != 0) break;
      local_10 = local_10 + 2;
    }
    lVar1 = *local_10;
  }
  return lVar1;
}




// Function: hash_get_next @ 0x123d1

long hash_get_next(long param_1,long param_2)

{
  long *local_18;
  long *local_10;
  
  local_18 = (long *)FUN_0011227f(param_1,param_2);
  local_10 = local_18;
  while ((param_2 != *local_10 || (local_10[1] == 0))) {
    local_10 = (long *)local_10[1];
    if (local_10 == (long *)0x0) {
      do {
        local_18 = local_18 + 2;
        if (*(long **)(param_1 + 8) <= local_18) {
          return 0;
        }
      } while (*local_18 == 0);
      return *local_18;
    }
  }
  return *(long *)local_10[1];
}




// Function: hash_get_entries @ 0x1246f

ulong hash_get_entries(undefined8 *param_1,long param_2,ulong param_3)

{
  ulong local_20;
  long *local_18;
  long *local_10;
  
  local_20 = 0;
  local_18 = (long *)*param_1;
  do {
    if ((long *)param_1[1] <= local_18) {
      return local_20;
    }
    if (*local_18 != 0) {
      for (local_10 = local_18; local_10 != (long *)0x0; local_10 = (long *)local_10[1]) {
        if (param_3 <= local_20) {
          return local_20;
        }
        *(long *)(local_20 * 8 + param_2) = *local_10;
        local_20 = local_20 + 1;
      }
    }
    local_18 = local_18 + 2;
  } while( true );
}




// Function: hash_do_for_each @ 0x1250f

long hash_do_for_each(undefined8 *param_1,code *param_2,undefined8 param_3)

{
  char cVar1;
  long local_20;
  long *local_18;
  long *local_10;
  
  local_20 = 0;
  local_18 = (long *)*param_1;
  do {
    if ((long *)param_1[1] <= local_18) {
      return local_20;
    }
    if (*local_18 != 0) {
      for (local_10 = local_18; local_10 != (long *)0x0; local_10 = (long *)local_10[1]) {
        cVar1 = (*param_2)(*local_10,param_3);
        if (cVar1 != '\x01') {
          return local_20;
        }
        local_20 = local_20 + 1;
      }
    }
    local_18 = local_18 + 2;
  } while( true );
}




// Function: hash_string @ 0x125a7

ulong hash_string(byte *param_1,ulong param_2)

{
  byte *local_20;
  ulong local_10;
  
  local_10 = 0;
  for (local_20 = param_1; *local_20 != 0; local_20 = local_20 + 1) {
    local_10 = ((ulong)*local_20 + local_10 * 0x1f) % param_2;
  }
  return local_10;
}




// Function: is_prime @ 0x12601

undefined8 is_prime(ulong param_1)

{
  undefined8 local_18;
  undefined8 local_10;
  
  local_10 = 9;
  for (local_18 = 3; (local_10 < param_1 && (param_1 % local_18 != 0)); local_18 = local_18 + 2) {
    local_10 = local_10 + (local_18 + 1) * 4;
  }
  return CONCAT71((int7)(param_1 % local_18 >> 8),param_1 % local_18 != 0);
}




// Function: next_prime @ 0x12670

ulong next_prime(ulong param_1)

{
  char cVar1;
  ulong local_10;
  
  local_10 = param_1;
  if (param_1 < 10) {
    local_10 = 10;
  }
  local_10 = local_10 | 1;
  while( true ) {
    if (local_10 == 0xffffffffffffffff) {
      return 0xffffffffffffffff;
    }
    cVar1 = FUN_00112601(local_10);
    if (cVar1 == '\x01') break;
    local_10 = local_10 + 2;
  }
  return local_10;
}




// Function: hash_reset_tuning @ 0x126bb

void hash_reset_tuning(undefined4 *param_1)

{
  *param_1 = 0;
  param_1[1] = DAT_00123500;
  param_1[2] = DAT_00123504;
  param_1[3] = DAT_00123508;
  *(undefined1 *)(param_1 + 4) = 0;
  return;
}




// Function: raw_hasher @ 0x12711

ulong raw_hasher(undefined8 param_1,ulong param_2)

{
  ulong uVar1;
  
  uVar1 = FUN_0011e664(param_1,3);
  return uVar1 % param_2;
}




// Function: raw_comparator @ 0x1274c

bool raw_comparator(long param_1,long param_2)

{
  return param_1 == param_2;
}




// Function: check_tuning @ 0x12769

undefined8 check_tuning(long param_1)

{
  float *pfVar1;
  undefined8 uVar2;
  
  pfVar1 = *(float **)(param_1 + 0x28);
  if (pfVar1 == (float *)&DAT_00123470) {
    uVar2 = 1;
  }
  else if ((((pfVar1[2] <= DAT_0012350c) || (DAT_00123500 - DAT_0012350c <= pfVar1[2])) ||
           (pfVar1[3] <= DAT_0012350c + DAT_00123500)) ||
          (((*pfVar1 < 0.0 || (pfVar1[1] <= *pfVar1 + DAT_0012350c)) ||
           ((DAT_00123500 < pfVar1[1] || (pfVar1[2] <= *pfVar1 + DAT_0012350c)))))) {
    *(undefined **)(param_1 + 0x28) = &DAT_00123470;
    uVar2 = 0;
  }
  else {
    uVar2 = 1;
  }
  return uVar2;
}




// Function: compute_bucket_size @ 0x12876

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulong compute_bucket_size(ulong param_1,long param_2)

{
  ulong uVar1;
  float fVar2;
  ulong local_20;
  
  local_20 = param_1;
  if (*(char *)(param_2 + 0x10) != '\x01') {
    fVar2 = (float)param_1 / *(float *)(param_2 + 8);
    if (_DAT_00123510 <= fVar2) {
      return 0;
    }
    if (DAT_00123514 <= fVar2) {
      local_20 = (long)(fVar2 - DAT_00123514) ^ 0x8000000000000000;
    }
    else {
      local_20 = (ulong)fVar2;
    }
  }
  uVar1 = FUN_00112670(local_20);
  if ((long)(uVar1 << 3) < 0 || uVar1 >> 0x3d != 0) {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: hash_initialize @ 0x1298f

long * hash_initialize(undefined8 param_1,undefined *param_2,code *param_3,code *param_4,long param_5)

{
  char cVar1;
  long *__ptr;
  long lVar2;
  void *pvVar3;
  code *local_38;
  code *local_30;
  undefined *local_28;
  
  local_30 = param_3;
  if (param_3 == (code *)0x0) {
    local_30 = FUN_00112711;
  }
  local_38 = param_4;
  if (param_4 == (code *)0x0) {
    local_38 = FUN_0011274c;
  }
  __ptr = malloc(0x50);
  if (__ptr != (long *)0x0) {
    local_28 = param_2;
    if (param_2 == (undefined *)0x0) {
      local_28 = &DAT_00123470;
    }
    __ptr[5] = (long)local_28;
    cVar1 = FUN_00112769(__ptr);
    if (cVar1 == '\x01') {
      lVar2 = FUN_00112876(param_1,local_28);
      __ptr[2] = lVar2;
      if (__ptr[2] != 0) {
        pvVar3 = calloc(__ptr[2],0x10);
        *__ptr = (long)pvVar3;
        if (*__ptr != 0) {
          __ptr[1] = *__ptr + __ptr[2] * 0x10;
          __ptr[3] = 0;
          __ptr[4] = 0;
          __ptr[6] = (long)local_30;
          __ptr[7] = (long)local_38;
          __ptr[8] = param_5;
          __ptr[9] = 0;
          return __ptr;
        }
      }
    }
    free(__ptr);
  }
  return (long *)0x0;
}




// Function: hash_clear @ 0x12b04

void hash_clear(undefined8 *param_1)

{
  undefined8 *puVar1;
  long *local_20;
  undefined8 *local_18;
  
  for (local_20 = (long *)*param_1; local_20 < (long *)param_1[1]; local_20 = local_20 + 2) {
    if (*local_20 != 0) {
      local_18 = (undefined8 *)local_20[1];
      while (local_18 != (undefined8 *)0x0) {
        if (param_1[8] != 0) {
          (*(code *)param_1[8])(*local_18);
        }
        *local_18 = 0;
        puVar1 = (undefined8 *)local_18[1];
        local_18[1] = param_1[9];
        param_1[9] = local_18;
        local_18 = puVar1;
      }
      if (param_1[8] != 0) {
        (*(code *)param_1[8])(*local_20);
      }
      *local_20 = 0;
      local_20[1] = 0;
    }
  }
  param_1[3] = 0;
  param_1[4] = 0;
  return;
}




// Function: hash_free @ 0x12c0f

void hash_free(ulong *param_1)

{
  void *pvVar1;
  long *local_20;
  long *local_18;
  
  if ((param_1[8] != 0) && (param_1[4] != 0)) {
    for (local_20 = (long *)*param_1; local_20 < (long *)param_1[1]; local_20 = local_20 + 2) {
      if (*local_20 != 0) {
        for (local_18 = local_20; local_18 != (long *)0x0; local_18 = (long *)local_18[1]) {
          (*(code *)param_1[8])(*local_18);
        }
      }
    }
  }
  for (local_20 = (long *)*param_1; local_20 < param_1[1];
      local_20 = (long *)((long)local_20 + 0x10)) {
    local_18 = *(void **)((long)local_20 + 8);
    while (local_18 != (void *)0x0) {
      pvVar1 = *(void **)((long)local_18 + 8);
      free(local_18);
      local_18 = pvVar1;
    }
  }
  local_18 = (void *)param_1[9];
  while (local_18 != (void *)0x0) {
    pvVar1 = *(void **)((long)local_18 + 8);
    free(local_18);
    local_18 = pvVar1;
  }
  free((void *)*param_1);
  free(param_1);
  return;
}




// Function: allocate_entry @ 0x12d3e

void * allocate_entry(long param_1)

{
  undefined8 local_10;
  
  if (*(long *)(param_1 + 0x48) == 0) {
    local_10 = malloc(0x10);
  }
  else {
    local_10 = *(void **)(param_1 + 0x48);
    *(undefined8 *)(param_1 + 0x48) = *(undefined8 *)((long)local_10 + 8);
  }
  return local_10;
}




// Function: free_entry @ 0x12d8d

void free_entry(long param_1,undefined8 *param_2)

{
  *param_2 = 0;
  param_2[1] = *(undefined8 *)(param_1 + 0x48);
  *(undefined8 **)(param_1 + 0x48) = param_2;
  return;
}




// Function: hash_find_entry @ 0x12dc7

long hash_find_entry(long param_1,long param_2,undefined8 *param_3,char param_4)

{
  long *plVar1;
  long lVar2;
  char cVar3;
  long lVar4;
  long *local_38;
  
  local_38 = (long *)FUN_0011227f(param_1,param_2);
  *param_3 = local_38;
  if (*local_38 == 0) {
    lVar4 = 0;
  }
  else if ((param_2 == *local_38) ||
          (cVar3 = (**(code **)(param_1 + 0x38))(param_2,*local_38), cVar3 != '\0')) {
    lVar4 = *local_38;
    if (param_4 != '\0') {
      if (local_38[1] == 0) {
        *local_38 = 0;
      }
      else {
        plVar1 = (long *)local_38[1];
        lVar2 = plVar1[1];
        *local_38 = *plVar1;
        local_38[1] = lVar2;
        FUN_00112d8d(param_1,plVar1);
      }
    }
  }
  else {
    for (; local_38[1] != 0; local_38 = (long *)local_38[1]) {
      if ((param_2 == *(long *)local_38[1]) ||
         (cVar3 = (**(code **)(param_1 + 0x38))(param_2,*(undefined8 *)local_38[1]), cVar3 != '\0'))
      {
        lVar4 = *(long *)local_38[1];
        if (param_4 == '\0') {
          return lVar4;
        }
        lVar2 = local_38[1];
        local_38[1] = *(long *)(lVar2 + 8);
        FUN_00112d8d(param_1,lVar2);
        return lVar4;
      }
    }
    lVar4 = 0;
  }
  return lVar4;
}




// Function: transfer_entries @ 0x12f60

undefined8 transfer_entries(long param_1,undefined8 *param_2,char param_3)

{
  long lVar1;
  long *plVar2;
  long *plVar3;
  long *local_38;
  long *local_30;
  
  local_38 = (long *)*param_2;
  do {
    if ((long *)param_2[1] <= local_38) {
      return 1;
    }
    if (*local_38 != 0) {
      local_30 = (long *)local_38[1];
      while (local_30 != (long *)0x0) {
        lVar1 = *local_30;
        plVar3 = (long *)FUN_0011227f(param_1,lVar1);
        plVar2 = (long *)local_30[1];
        if (*plVar3 == 0) {
          *plVar3 = lVar1;
          *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
          FUN_00112d8d(param_1,local_30);
          local_30 = plVar2;
        }
        else {
          local_30[1] = plVar3[1];
          plVar3[1] = (long)local_30;
          local_30 = plVar2;
        }
      }
      lVar1 = *local_38;
      local_38[1] = 0;
      if (param_3 == '\0') {
        plVar2 = (long *)FUN_0011227f(param_1,lVar1);
        if (*plVar2 == 0) {
          *plVar2 = lVar1;
          *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
        }
        else {
          plVar3 = (long *)FUN_00112d3e(param_1);
          if (plVar3 == (long *)0x0) {
            return 0;
          }
          *plVar3 = lVar1;
          plVar3[1] = plVar2[1];
          plVar2[1] = (long)plVar3;
        }
        *local_38 = 0;
        param_2[3] = param_2[3] + -1;
      }
    }
    local_38 = local_38 + 2;
  } while( true );
}




// Function: hash_rehash @ 0x13134

undefined8 hash_rehash(undefined8 *param_1,undefined8 param_2)

{
  char cVar1;
  size_t __nmemb;
  undefined8 uVar2;
  long in_FS_OFFSET;
  void *local_68;
  void *local_60;
  size_t local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined8 local_40;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 local_28;
  undefined8 local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  __nmemb = FUN_00112876(param_2,param_1[5]);
  if (__nmemb == 0) {
    uVar2 = 0;
  }
  else if (__nmemb == param_1[2]) {
    uVar2 = 1;
  }
  else {
    local_68 = calloc(__nmemb,0x10);
    if (local_68 == (void *)0x0) {
      uVar2 = 0;
    }
    else {
      local_60 = (void *)((long)local_68 + __nmemb * 0x10);
      local_50 = 0;
      local_48 = 0;
      local_40 = param_1[5];
      local_38 = param_1[6];
      local_30 = param_1[7];
      local_28 = param_1[8];
      local_20 = param_1[9];
      local_58 = __nmemb;
      cVar1 = FUN_00112f60(&local_68,param_1,0);
      if (cVar1 == '\0') {
        param_1[9] = local_20;
        cVar1 = FUN_00112f60(param_1,&local_68,1);
        if (cVar1 == '\x01') {
          cVar1 = FUN_00112f60(param_1,&local_68,0);
          if (cVar1 == '\x01') {
            free(local_68);
            uVar2 = 0;
            goto LAB_00113349;
          }
        }
                    /* WARNING: Subroutine does not return */
        abort();
      }
      free((void *)*param_1);
      *param_1 = local_68;
      param_1[1] = local_60;
      param_1[2] = local_58;
      param_1[3] = local_50;
      param_1[9] = local_20;
      uVar2 = 1;
    }
  }
LAB_00113349:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: hash_insert_if_absent @ 0x1335f

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 hash_insert_if_absent(long param_1,long param_2,long *param_3)

{
  char cVar1;
  undefined8 uVar2;
  ulong uVar3;
  long lVar4;
  long in_FS_OFFSET;
  float fVar5;
  long *local_30;
  long local_28;
  long local_20;
  long *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 == 0) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  local_28 = FUN_00112dc7(param_1,param_2,&local_30,0);
  if (local_28 != 0) {
    if (param_3 != (long *)0x0) {
      *param_3 = local_28;
    }
    uVar2 = 0;
    goto LAB_001136ba;
  }
  if (((float)*(ulong *)(param_1 + 0x10) * *(float *)(*(long *)(param_1 + 0x28) + 8) <
       (float)*(ulong *)(param_1 + 0x18)) &&
     (FUN_00112769(param_1),
     (float)*(ulong *)(param_1 + 0x10) * *(float *)(*(long *)(param_1 + 0x28) + 8) <
     (float)*(ulong *)(param_1 + 0x18))) {
    local_20 = *(long *)(param_1 + 0x28);
    if (*(char *)(local_20 + 0x10) == '\0') {
      fVar5 = *(float *)(local_20 + 8) *
              *(float *)(local_20 + 0xc) * (float)*(ulong *)(param_1 + 0x10);
    }
    else {
      fVar5 = (float)*(ulong *)(param_1 + 0x10) * *(float *)(local_20 + 0xc);
    }
    if (_DAT_00123510 <= fVar5) {
      uVar2 = 0xffffffff;
      goto LAB_001136ba;
    }
    if (DAT_00123514 <= fVar5) {
      uVar3 = (long)(fVar5 - DAT_00123514) ^ 0x8000000000000000;
    }
    else {
      uVar3 = (ulong)fVar5;
    }
    cVar1 = FUN_00113134(param_1,uVar3);
    if (cVar1 != '\x01') {
      uVar2 = 0xffffffff;
      goto LAB_001136ba;
    }
    lVar4 = FUN_00112dc7(param_1,param_2,&local_30,0);
    if (lVar4 != 0) {
                    /* WARNING: Subroutine does not return */
      abort();
    }
  }
  if (*local_30 == 0) {
    *local_30 = param_2;
    *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
    *(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + 1;
    uVar2 = 1;
  }
  else {
    local_18 = (long *)FUN_00112d3e(param_1);
    if (local_18 == (long *)0x0) {
      uVar2 = 0xffffffff;
    }
    else {
      *local_18 = param_2;
      local_18[1] = local_30[1];
      local_30[1] = (long)local_18;
      *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
      uVar2 = 1;
    }
  }
LAB_001136ba:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: hash_insert @ 0x136d0

undefined8 hash_insert(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_0011335f(param_1,param_2,&local_18);
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




// Function: hash_remove @ 0x13740

long hash_remove(long param_1,undefined8 param_2)

{
  char cVar1;
  long lVar2;
  long in_FS_OFFSET;
  float fVar3;
  long *local_40;
  void *local_38;
  long local_30;
  long local_28;
  ulong local_20;
  void *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_30 = FUN_00112dc7(param_1,param_2,&local_40,1);
  if (local_30 == 0) {
    lVar2 = 0;
  }
  else {
    *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + -1;
    lVar2 = local_30;
    if (((*local_40 == 0) &&
        (*(long *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + -1,
        (float)*(ulong *)(param_1 + 0x18) <
        (float)*(ulong *)(param_1 + 0x10) * **(float **)(param_1 + 0x28))) &&
       (FUN_00112769(param_1), lVar2 = local_30,
       (float)*(ulong *)(param_1 + 0x18) <
       (float)*(ulong *)(param_1 + 0x10) * **(float **)(param_1 + 0x28))) {
      local_28 = *(long *)(param_1 + 0x28);
      if (*(char *)(local_28 + 0x10) == '\0') {
        fVar3 = *(float *)(local_28 + 8) *
                *(float *)(local_28 + 4) * (float)*(ulong *)(param_1 + 0x10);
        if (DAT_00123514 <= fVar3) {
          local_20 = (long)(fVar3 - DAT_00123514) ^ 0x8000000000000000;
        }
        else {
          local_20 = (ulong)fVar3;
        }
      }
      else {
        fVar3 = (float)*(ulong *)(param_1 + 0x10) * *(float *)(local_28 + 4);
        if (DAT_00123514 <= fVar3) {
          local_20 = (long)(fVar3 - DAT_00123514) ^ 0x8000000000000000;
        }
        else {
          local_20 = (ulong)fVar3;
        }
      }
      cVar1 = FUN_00113134(param_1,local_20);
      lVar2 = local_30;
      if (cVar1 != '\x01') {
        local_38 = *(void **)(param_1 + 0x48);
        while (local_38 != (void *)0x0) {
          local_18 = *(void **)((long)local_38 + 8);
          free(local_38);
          local_38 = local_18;
        }
        *(undefined8 *)(param_1 + 0x48) = 0;
        lVar2 = local_30;
      }
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return lVar2;
}




// Function: hash_delete @ 0x13a46

void hash_delete(undefined8 param_1,undefined8 param_2)

{
  FUN_00113740(param_1,param_2);
  return;
}




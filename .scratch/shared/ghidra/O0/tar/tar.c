// Function: mv_begin_write @ 0xab09

void mv_begin_write(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  
  if (DAT_0019f8fc != '\0') {
    puVar2 = (undefined8 *)FUN_00160b5b(0x30);
    puVar1 = puVar2;
    if (DAT_0019f068 != (undefined8 *)0x0) {
      *DAT_0019f068 = puVar2;
      puVar1 = DAT_0019f060;
    }
    DAT_0019f060 = puVar1;
    DAT_0019f068 = puVar2;
    *puVar2 = 0;
    puVar2[1] = DAT_0019efb0 - DAT_0019efa0 >> 9;
    uVar3 = FUN_00160cc6(param_1);
    puVar2[2] = uVar3;
    puVar2[3] = param_2;
    puVar2[4] = param_3;
    puVar2[5] = 0;
  }
  return;
}




// Function: bufmap_locate @ 0xabd9

long * bufmap_locate(ulong param_1)

{
  undefined8 local_10;
  
  local_10 = DAT_0019f060;
  while (((local_10 != (long *)0x0 && (*local_10 != 0)) &&
         ((ulong)(*(long *)(*local_10 + 8) << 9) <= param_1))) {
    local_10 = (long *)*local_10;
  }
  return local_10;
}




// Function: bufmap_free @ 0xac2b

void bufmap_free(undefined8 *param_1)

{
  undefined8 *puVar1;
  undefined8 *local_18;
  
  local_18 = DAT_0019f060;
  while ((local_18 != (undefined8 *)0x0 && (local_18 != param_1))) {
    puVar1 = (undefined8 *)*local_18;
    free((void *)local_18[2]);
    free(local_18);
    local_18 = puVar1;
  }
  DAT_0019f060 = local_18;
  if (local_18 == (undefined8 *)0x0) {
    DAT_0019f068 = local_18;
  }
  return;
}




// Function: bufmap_reset @ 0xacb0

void bufmap_reset(undefined8 *param_1,long param_2)

{
  undefined8 *local_10;
  
  FUN_0010ac2b(param_1);
  local_10 = param_1;
  if (param_1 != (undefined8 *)0x0) {
    for (; local_10 != (undefined8 *)0x0; local_10 = (undefined8 *)*local_10) {
      local_10[1] = local_10[1] + param_2;
      local_10[5] = 0;
    }
  }
  return;
}




// Function: buffer_write_global_xheader @ 0xad11

void buffer_write_global_xheader(void)

{
  FUN_0011d3e6(&DAT_0019f1e8);
  return;
}




// Function: mv_begin_read @ 0xad2b

void mv_begin_read(undefined8 *param_1)

{
  FUN_0010ab09(*param_1,param_1[0x11],param_1[0x11]);
  return;
}




// Function: mv_end @ 0xad66

void mv_end(void)

{
  if (DAT_0019f8fc != '\0') {
    FUN_0010ac2b(0);
  }
  return;
}




// Function: mv_size_left @ 0xad86

void mv_size_left(undefined8 param_1)

{
  if (DAT_0019f060 != 0) {
    *(undefined8 *)(DAT_0019f060 + 0x20) = param_1;
  }
  return;
}




// Function: clear_read_error_count @ 0xadb0

void clear_read_error_count(void)

{
  DAT_0019f044 = 0;
  return;
}




// Function: set_start_time @ 0xadc5

void set_start_time(void)

{
  FUN_001521fa(&DAT_0019f9e0);
  DAT_0019f9f0 = DAT_0019f9e0;
  DAT_0019f9f8 = DAT_0019f9e8;
  DAT_0019fa00 = DAT_0019f9e0;
  DAT_0019fa08 = DAT_0019f9e8;
  return;
}




// Function: set_volume_start_time @ 0xae17

void set_volume_start_time(void)

{
  FUN_001521fa(&DAT_0019f9f0);
  DAT_0019fa00 = DAT_0019f9f0;
  DAT_0019fa08 = DAT_0019f9f8;
  return;
}




// Function: compute_duration @ 0xae4d

double compute_duration(void)

{
  long in_FS_OFFSET;
  long local_28;
  long local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_001521fa(&local_28);
  DAT_0019f238 = DAT_0019f238 +
                 (double)(local_28 - DAT_0019fa00) +
                 (double)(local_20 - DAT_0019fa08) / DAT_00180790;
  FUN_001521fa(&DAT_0019fa00);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return DAT_0019f238;
}




// Function: find_zip_program @ 0xaefd

undefined * find_zip_program(int param_1,int *param_2)

{
  int local_c;
  
  local_c = *param_2;
  while( true ) {
    if (*(int *)(&DAT_0019c020 + (long)local_c * 0x18) == 0) {
      *param_2 = local_c;
      return (undefined *)0x0;
    }
    if (param_1 == *(int *)(&DAT_0019c020 + (long)local_c * 0x18)) break;
    local_c = local_c + 1;
  }
  *param_2 = local_c + 1;
  return &DAT_0019c020 + (long)local_c * 0x18;
}




// Function: first_decompress_program @ 0xafa2

long first_decompress_program(undefined4 *param_1)

{
  long lVar1;
  
  lVar1 = DAT_0019f898;
  if (DAT_0019f898 == 0) {
    if (DAT_0019f240 == 0) {
      lVar1 = 0;
    }
    else {
      *param_1 = 0;
      lVar1 = FUN_0010aefd(DAT_0019f240,param_1);
      if (lVar1 == 0) {
        lVar1 = 0;
      }
      else {
        lVar1 = *(long *)(lVar1 + 8);
      }
    }
  }
  return lVar1;
}




// Function: next_decompress_program @ 0xb012

undefined8 next_decompress_program(undefined8 param_1)

{
  undefined8 uVar1;
  long lVar2;
  
  if (DAT_0019f898 == 0) {
    lVar2 = FUN_0010aefd(DAT_0019f240,param_1);
    if (lVar2 == 0) {
      uVar1 = 0;
    }
    else {
      uVar1 = *(undefined8 *)(lVar2 + 8);
    }
  }
  else {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: compress_option @ 0xb065

undefined8 compress_option(undefined4 param_1)

{
  undefined8 uVar1;
  long in_FS_OFFSET;
  undefined4 local_1c;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_1c = 0;
  local_18 = FUN_0010aefd(param_1,&local_1c);
  if (local_18 == 0) {
    uVar1 = 0;
  }
  else {
    uVar1 = *(undefined8 *)(local_18 + 0x10);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar1;
}




// Function: check_compressed_archive @ 0xb0cb

undefined4 check_compressed_archive(undefined1 *param_1)

{
  int iVar1;
  undefined4 uVar2;
  long lVar3;
  long in_FS_OFFSET;
  undefined1 *local_30;
  undefined1 local_1a;
  undefined1 local_19;
  undefined **local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_30 = param_1;
  if (param_1 == (undefined1 *)0x0) {
    local_30 = &local_1a;
  }
  DAT_0019efa8 = DAT_0019efa0;
  local_19 = DAT_0019f049;
  DAT_0019f049 = 1;
  lVar3 = FUN_0010bb4b();
  *local_30 = lVar3 == 0;
  DAT_0019f049 = local_19;
  if (DAT_0019efa0 != DAT_0019efa8) {
    iVar1 = strcmp((char *)((long)DAT_0019efa0 + 0x101),"ustar");
    if (iVar1 != 0) {
      iVar1 = strcmp((char *)((long)DAT_0019efa0 + 0x101),"ustar  ");
      if (iVar1 != 0) goto LAB_0010b1ac;
    }
    iVar1 = FUN_00125d84(DAT_0019efa0,1);
    if (iVar1 == 1) {
      uVar2 = 1;
      goto LAB_0010b1fe;
    }
  }
LAB_0010b1ac:
  for (local_18 = (undefined **)&DAT_0019a170; local_18 < &PTR_s_Total_bytes_read_0019a230;
      local_18 = local_18 + 3) {
    iVar1 = memcmp(DAT_0019efa0,local_18[2],(size_t)local_18[1]);
    if (iVar1 == 0) {
      uVar2 = *(undefined4 *)local_18;
      goto LAB_0010b1fe;
    }
  }
  uVar2 = 0;
LAB_0010b1fe:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: guess_seekable_archive @ 0xb214

void guess_seekable_archive(void)

{
  int iVar1;
  long in_FS_OFFSET;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0019f860 == 4) {
    DAT_0019fc18 = 0;
  }
  if (DAT_0019fc14 == -1) {
    if ((DAT_0019f8fc != '\x01') && (DAT_0019f898 == 0)) {
      iVar1 = fstat(DAT_0019f9cc,&local_a8);
      if (iVar1 == 0) {
        DAT_0019fc18 = (local_a8.st_mode & 0xf000) == 0x8000;
        goto LAB_0010b2b9;
      }
    }
    DAT_0019fc18 = false;
  }
  else {
    DAT_0019fc18 = DAT_0019fc14 != 0;
  }
LAB_0010b2b9:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: open_compressed_archive @ 0xb2cf

int open_compressed_archive(void)

{
  int iVar1;
  void *pvVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  char local_15;
  int local_14;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if ((((DAT_0019fdf8 == '\x01') ||
       (DAT_0019fdf0 = strchr((char *)*DAT_0019fbd8,0x3a), DAT_0019fdf0 == (char *)0x0)) ||
      (DAT_0019fdf0 <= (char *)*DAT_0019fbd8)) ||
     (pvVar2 = memchr((void *)*DAT_0019fbd8,0x2f,(long)DAT_0019fdf0 - *DAT_0019fbd8),
     pvVar2 != (void *)0x0)) {
    DAT_0019f9cc = open((char *)*DAT_0019fbd8,0,0x1b6);
  }
  else {
    DAT_0019f9cc = FUN_0013f860(*DAT_0019fbd8,0,0x40000000,DAT_0019f950);
  }
  if (DAT_0019f9cc == -1) goto LAB_0010b55a;
  if (DAT_0019f8fc != '\x01') {
    iVar1 = DAT_0019f240;
    if (DAT_0019f898 == 0) {
      local_14 = FUN_0010b0cb(&local_15);
      if (local_14 == 0) {
        if (local_15 != '\0') {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar3 = gettext("This does not look like a tar archive");
          error(0,0,uVar3);
          DAT_0019fdd0 = 2;
        }
        FUN_00132970(*DAT_0019fbd8,0);
        iVar1 = DAT_0019f240;
        if (DAT_0019f898 == 0) goto LAB_0010b55a;
      }
      else {
        iVar1 = local_14;
        if (local_14 == 1) {
          if (local_15 != '\0') {
            if (DAT_0019fdc8 != (code *)0x0) {
              (*DAT_0019fdc8)();
            }
            uVar3 = gettext("This does not look like a tar archive");
            error(0,0,uVar3);
            DAT_0019fdd0 = 2;
          }
          goto LAB_0010b55a;
        }
      }
    }
    DAT_0019f240 = iVar1;
    if (DAT_0019f9cc < 0x40000000) {
      close(DAT_0019f9cc);
    }
    else {
      FUN_0013fe0c(DAT_0019f9cc + -0x40000000);
    }
    DAT_0019f048 = 0;
    DAT_0019f040 = FUN_0013399b();
    DAT_0019f049 = 1;
  }
  DAT_0019efc0 = 0;
  DAT_0019efa8 = DAT_0019efa0;
LAB_0010b55a:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return DAT_0019f9cc;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: print_stats @ 0xb570

int print_stats(double param_1,FILE *param_2,char *param_3)

{
  int iVar1;
  undefined8 uVar2;
  ulong uVar3;
  undefined *puVar4;
  long in_FS_OFFSET;
  double dVar5;
  int local_550;
  undefined1 local_548 [656];
  undefined1 local_2b8 [664];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_550 = 0;
  if ((param_3 != (char *)0x0) && (*param_3 != '\0')) {
    uVar2 = gettext(param_3);
    local_550 = fprintf(param_2,"%s: ",uVar2);
  }
  if (0.0 < DAT_0019f238) {
    if (param_1 / DAT_0019f238 < DAT_00180798) {
      dVar5 = param_1 / DAT_0019f238;
      if (DAT_001807a0 <= dVar5) {
        uVar3 = (long)(dVar5 - DAT_001807a0) ^ 0x8000000000000000;
      }
      else {
        uVar3 = (ulong)dVar5;
      }
      puVar4 = (undefined *)FUN_00153f1c(uVar3,local_2b8,0x1b0,1,1);
      goto LAB_0010b6b2;
    }
  }
  puVar4 = &DAT_001800c3;
LAB_0010b6b2:
  if (DAT_001807a0 <= param_1) {
    uVar3 = (long)(param_1 - DAT_001807a0) ^ 0x8000000000000000;
  }
  else {
    uVar3 = (ulong)param_1;
  }
  uVar2 = FUN_00153f1c(uVar3,local_548,0x1b0,1,1);
  iVar1 = fprintf(param_2,"%.0f (%s, %s/s)",param_1,uVar2,puVar4);
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return iVar1 + local_550;
}




// Function: format_total_stats @ 0xb768

int format_total_stats(FILE *param_1,undefined8 *param_2,int param_3,int param_4)

{
  int iVar1;
  int iVar2;
  undefined8 uVar3;
  ulong uVar4;
  long in_FS_OFFSET;
  double dVar5;
  int local_3c;
  undefined1 local_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0019f860 != 8) {
    if ((8 < DAT_0019f860) || (7 < DAT_0019f860)) goto LAB_0010ba65;
    if (4 < DAT_0019f860) {
      uVar4 = DAT_0019f870 * DAT_0019efc0;
      uVar3 = gettext(*param_2);
      local_3c = FUN_0010b570((double)uVar4,param_1,uVar3);
      goto LAB_0010ba6a;
    }
    if (3 < DAT_0019f860) {
      if (DAT_0019f860 != 4) goto LAB_0010ba65;
      iVar1 = FUN_0010b570((double)(ulong)(DAT_0019f870 * DAT_0019efc0),param_1,*param_2);
      fputc_unlocked(param_3,param_1);
      iVar2 = FUN_0010b570(DAT_0019f000 + DAT_0019f008,param_1,param_2[1]);
      fputc_unlocked(param_3,param_1);
      local_3c = iVar1 + iVar2 + 2;
      if ((param_2[2] != 0) && (*(char *)param_2[2] != '\0')) {
        uVar3 = gettext(param_2[2]);
        iVar1 = fprintf(param_1,"%s: ",uVar3);
        local_3c = local_3c + iVar1;
      }
      dVar5 = (double)(ulong)(DAT_0019f870 * (DAT_0019efc0 - DAT_0019f370)) -
              (DAT_0019f008 + DAT_0019f000);
      if (DAT_001807a0 <= dVar5) {
        uVar4 = (long)(dVar5 - DAT_001807a0) ^ 0x8000000000000000;
      }
      else {
        uVar4 = (ulong)dVar5;
      }
      uVar3 = FUN_00154e5a(uVar4,local_38);
      iVar1 = fprintf(param_1,"%s",uVar3);
      local_3c = local_3c + iVar1;
      goto LAB_0010ba6a;
    }
    if (DAT_0019f860 == 0) {
LAB_0010ba65:
                    /* WARNING: Subroutine does not return */
      abort();
    }
  }
  local_3c = FUN_0010b570(DAT_0019f000 + DAT_0019f008,param_1,param_2[1]);
LAB_0010ba6a:
  if (param_4 != 0) {
    fputc_unlocked(param_4,param_1);
    local_3c = local_3c + 1;
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_3c;
}




// Function: print_total_stats @ 0xbaa2

void print_total_stats(void)

{
  FUN_0010b768(stderr,&PTR_s_Total_bytes_read_0019a230,10,10);
  return;
}




// Function: current_block_ordinal @ 0xbacd

long current_block_ordinal(void)

{
  return DAT_0019f038 + (DAT_0019efb0 - DAT_0019efa0 >> 9);
}




// Function: reset_eof @ 0xbaf9

void reset_eof(void)

{
  if (DAT_0019f048 != '\0') {
    DAT_0019f048 = '\0';
    DAT_0019efb0 = DAT_0019efa0;
    DAT_0019efa8 = (long)DAT_0019f868 * 0x200 + DAT_0019efa0;
    DAT_0019efb8 = 1;
  }
  return;
}




// Function: find_next_block @ 0xbb4b

long find_next_block(void)

{
  long lVar1;
  
  lVar1 = DAT_0019efb0;
  if (DAT_0019efb0 == DAT_0019efa8) {
    if (DAT_0019f048 == '\0') {
      FUN_0010cac8();
      lVar1 = DAT_0019efb0;
      if (DAT_0019efb0 == DAT_0019efa8) {
        DAT_0019f048 = '\x01';
        lVar1 = 0;
      }
    }
    else {
      lVar1 = 0;
    }
  }
  return lVar1;
}




// Function: set_next_block_after @ 0xbba7

void set_next_block_after(ulong param_1)

{
  for (; DAT_0019efb0 <= param_1; DAT_0019efb0 = DAT_0019efb0 + 0x200) {
  }
  if (DAT_0019efa8 < DAT_0019efb0) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  return;
}




// Function: available_space_after @ 0xbbf5

long available_space_after(long param_1)

{
  return DAT_0019efa8 - param_1;
}




// Function: xclose @ 0xbc11

void xclose(int param_1)

{
  int iVar1;
  undefined8 uVar2;
  
  iVar1 = close(param_1);
  if (iVar1 != 0) {
    uVar2 = gettext("(pipe)");
    FUN_0013e1ff(uVar2);
  }
  return;
}




// Function: init_buffer @ 0xbc48

void init_buffer(void)

{
  int iVar1;
  undefined8 uVar2;
  
  iVar1 = DAT_0019f030;
  if (*(long *)(&DAT_0019f020 + (long)DAT_0019f030 * 8) == 0) {
    uVar2 = FUN_0012b601(&DAT_0019f010 + DAT_0019f030,DAT_0019f870);
    *(undefined8 *)(&DAT_0019f020 + (long)iVar1 * 8) = uVar2;
  }
  DAT_0019efa0 = *(long *)(&DAT_0019f020 + (long)DAT_0019f030 * 8);
  DAT_0019efb0 = DAT_0019efa0;
  DAT_0019efa8 = (long)DAT_0019f868 * 0x200 + DAT_0019efa0;
  return;
}




// Function: check_tty @ 0xbd12

void check_tty(int param_1)

{
  int iVar1;
  undefined8 uVar2;
  
  iVar1 = strcmp((char *)*DAT_0019fbd8,"-");
  if (iVar1 == 0) {
    iVar1 = isatty((uint)(param_1 != 0));
    if (iVar1 != 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      if (param_1 == 0) {
        uVar2 = gettext("Refusing to read archive contents from terminal (missing -f option?)");
      }
      else {
        uVar2 = gettext("Refusing to write archive contents to terminal (missing -f option?)");
      }
      error(0,0,uVar2);
                    /* WARNING: Subroutine does not return */
      FUN_001177f4();
    }
  }
  return;
}




// Function: _open_archive @ 0xbdb0

void _open_archive(uint param_1)

{
  char cVar1;
  int iVar2;
  uint uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  void *pvVar6;
  int *piVar7;
  long in_FS_OFFSET;
  char local_2d;
  int local_2c;
  int local_28;
  int local_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_2c = 0;
  if (DAT_0019f870 == 0) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar4 = gettext("Invalid value for record_size");
    error(0,0,uVar4);
                    /* WARNING: Subroutine does not return */
    FUN_001177f4();
  }
  if (DAT_0019fbe0 != 0) {
    FUN_00139d1c(&DAT_0019fa20);
    DAT_0019f030 = 0;
    FUN_0010bc48();
    DAT_0019efb8 = param_1;
    if (param_1 == 2) {
      DAT_0019efb8 = 0;
    }
    FUN_0010bd12(DAT_0019efb8);
    DAT_0019f049 = DAT_0019f94d;
    DAT_0019efc0 = 0;
    if (DAT_0019f898 == 0) {
      iVar2 = strcmp((char *)*DAT_0019fbd8,"-");
      if (iVar2 == 0) {
        DAT_0019f049 = 1;
        if (DAT_0019f9b0 != '\0') {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar4 = gettext("Cannot verify stdin/stdout archive");
          error(0,0,uVar4);
                    /* WARNING: Subroutine does not return */
          FUN_001177f4();
        }
        if (param_1 == 2) {
          DAT_0019f9cc = 0;
          DAT_0019efd8 = 1;
          DAT_0019efa8 = DAT_0019efa0;
          if (DAT_0019fbf0 == 0) {
            DAT_0019efd0 = stderr;
          }
        }
        else if (param_1 < 3) {
          if (param_1 == 0) {
            DAT_0019f9cc = 0;
            local_28 = FUN_0010b0cb(&local_2d);
            if ((local_28 != 1) && (local_28 != 0)) {
              if (DAT_0019fdc8 != (code *)0x0) {
                (*DAT_0019fdc8)();
              }
              uVar4 = FUN_0010b065(local_28);
              uVar5 = gettext("Archive is compressed. Use %s option");
              error(0,0,uVar5,uVar4);
                    /* WARNING: Subroutine does not return */
              FUN_001177f4();
            }
            if (local_2d != '\0') {
              if (DAT_0019fdc8 != (code *)0x0) {
                (*DAT_0019fdc8)();
              }
              uVar4 = gettext("This does not look like a tar archive");
              error(0,0,uVar4);
              DAT_0019fdd0 = 2;
            }
          }
          else if ((param_1 == 1) && (DAT_0019f9cc = 1, DAT_0019fbf0 == 0)) {
            DAT_0019efd0 = stderr;
          }
        }
      }
      else if (param_1 == 2) {
        if ((((DAT_0019fdf8 == '\x01') ||
             (DAT_0019fdf0 = strchr((char *)*DAT_0019fbd8,0x3a), DAT_0019fdf0 == (char *)0x0)) ||
            (DAT_0019fdf0 <= (char *)*DAT_0019fbd8)) ||
           (pvVar6 = memchr((void *)*DAT_0019fbd8,0x2f,(long)DAT_0019fdf0 - *DAT_0019fbd8),
           pvVar6 != (void *)0x0)) {
          DAT_0019f9cc = open((char *)*DAT_0019fbd8,0x42,0x1b6);
        }
        else {
          DAT_0019f9cc = FUN_0013f860(*DAT_0019fbd8,0x42,0x40000000,DAT_0019f950);
        }
        uVar3 = FUN_0010b0cb(0);
        if (1 < uVar3) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar4 = gettext("Cannot update compressed archives");
          error(0,0,uVar4);
                    /* WARNING: Subroutine does not return */
          FUN_001177f4();
        }
      }
      else if (param_1 < 3) {
        if (param_1 == 0) {
          DAT_0019f9cc = FUN_0010b2cf();
          if (-1 < DAT_0019f9cc) {
            FUN_0010b214();
          }
        }
        else if (param_1 == 1) {
          if (DAT_0019f884 != '\0') {
            FUN_0012a571(*DAT_0019fbd8,1);
            local_2c = 1;
          }
          if (DAT_0019f9b0 == '\0') {
            if (((DAT_0019fdf8 == '\x01') ||
                (DAT_0019fdf0 = strchr((char *)*DAT_0019fbd8,0x3a), DAT_0019fdf0 == (char *)0x0)) ||
               ((DAT_0019fdf0 <= (char *)*DAT_0019fbd8 ||
                (pvVar6 = memchr((void *)*DAT_0019fbd8,0x2f,(long)DAT_0019fdf0 - *DAT_0019fbd8),
                pvVar6 != (void *)0x0)))) {
              DAT_0019f9cc = creat((char *)*DAT_0019fbd8,0x1b6);
            }
            else {
              DAT_0019f9cc = FUN_0013f860(*DAT_0019fbd8,0x41,0x40000000,DAT_0019f950);
            }
          }
          else if (((DAT_0019fdf8 == '\x01') ||
                   (DAT_0019fdf0 = strchr((char *)*DAT_0019fbd8,0x3a), DAT_0019fdf0 == (char *)0x0))
                  || ((DAT_0019fdf0 <= (char *)*DAT_0019fbd8 ||
                      (pvVar6 = memchr((void *)*DAT_0019fbd8,0x2f,(long)DAT_0019fdf0 - *DAT_0019fbd8
                                      ), pvVar6 != (void *)0x0)))) {
            DAT_0019f9cc = open((char *)*DAT_0019fbd8,0x42,0x1b6);
          }
          else {
            DAT_0019f9cc = FUN_0013f860(*DAT_0019fbd8,0x42,0x40000000,DAT_0019f950);
          }
        }
      }
    }
    else {
      if (param_1 == 2) {
                    /* WARNING: Subroutine does not return */
        abort();
      }
      if (param_1 < 3) {
        if (param_1 == 0) {
          DAT_0019f040 = FUN_0013399b();
          DAT_0019f049 = 1;
          DAT_0019efa8 = DAT_0019efa0;
        }
        else if (param_1 == 1) {
          DAT_0019f040 = FUN_0013321b();
        }
      }
      if (((DAT_0019fbf0 == 0) && (param_1 == 1)) &&
         (iVar2 = strcmp((char *)*DAT_0019fbd8,"-"), iVar2 == 0)) {
        DAT_0019efd0 = stderr;
      }
    }
    if ((DAT_0019f9cc < 0) ||
       ((DAT_0019f9cc < 0x40000000 && (cVar1 = FUN_00132b11(), cVar1 != '\x01')))) {
      piVar7 = __errno_location();
      local_24 = *piVar7;
      if (local_2c != 0) {
        FUN_0012a874();
      }
      piVar7 = __errno_location();
      *piVar7 = local_24;
      FUN_0013e3b4(*DAT_0019fbd8);
    }
    FUN_00132bdc();
    FUN_00132b88();
    if (param_1 == 0) {
      FUN_0010bb4b();
    }
    else if (param_1 - 1 < 2) {
      DAT_0019efc8 = 0;
    }
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  if (DAT_0019fdc8 != (code *)0x0) {
    (*DAT_0019fdc8)();
  }
  uVar4 = gettext("No archive name given");
  error(0,0,uVar4);
                    /* WARNING: Subroutine does not return */
  FUN_001177f4();
}




// Function: _flush_write @ 0xc54f

long _flush_write(void)

{
  int *piVar1;
  long local_28;
  undefined8 *local_20;
  ulong local_18;
  long local_10;
  
  FUN_001106d8(1);
  if ((DAT_0019f990 == 0.0) || (DAT_0019f008 < DAT_0019f990)) {
    if (DAT_0019f9d0 == '\0') {
      local_28 = FUN_0013301a();
    }
    else {
      local_28 = DAT_0019f870;
    }
  }
  else {
    piVar1 = __errno_location();
    *piVar1 = 0x1c;
    local_28 = 0;
  }
  if ((((local_28 != 0) && (DAT_0019f8fc != '\0')) && (DAT_0019f070 == 0)) &&
     (local_20 = (undefined8 *)FUN_0010abd9(local_28), local_20 != (undefined8 *)0x0)) {
    local_18 = local_28 + local_20[1] * -0x200;
    local_20[5] = local_20[5] + (local_18 >> 9);
    if ((ulong)local_20[4] < local_18) {
      local_18 = local_20[4];
    }
    local_20[4] = local_20[4] - local_18;
    if (local_20[4] == 0) {
      local_10 = local_20[5] + local_20[1];
      local_20 = (undefined8 *)*local_20;
    }
    else {
      local_10 = local_20[1];
    }
    FUN_0010acb0(local_20,-local_10);
  }
  return local_28;
}




// Function: archive_write_error @ 0xc6d5

void archive_write_error(undefined8 param_1)

{
  int iVar1;
  int *piVar2;
  undefined8 uVar3;
  
  if (DAT_0019f999 != '\0') {
    piVar2 = __errno_location();
    iVar1 = *piVar2;
    FUN_0010baa2();
    piVar2 = __errno_location();
    *piVar2 = iVar1;
  }
  FUN_0010d132(*DAT_0019fbe8,param_1,DAT_0019f870);
  FUN_0013e403(*DAT_0019fbe8);
  iVar1 = DAT_0019f044;
  if (DAT_0019f038 == 0) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar3 = gettext("At beginning of tape, quitting now");
    error(0,0,uVar3);
                    /* WARNING: Subroutine does not return */
    FUN_001177f4();
  }
  DAT_0019f044 = DAT_0019f044 + 1;
  if (10 < iVar1) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar3 = gettext("Too many errors, quitting");
    error(0,0,uVar3);
                    /* WARNING: Subroutine does not return */
    FUN_001177f4();
  }
  return;
}




// Function: archive_read_error @ 0xc729

void archive_read_error(void)

{
  int iVar1;
  undefined8 uVar2;
  
  FUN_0013e403(*DAT_0019fbe8);
  iVar1 = DAT_0019f044;
  if (DAT_0019f038 == 0) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar2 = gettext("At beginning of tape, quitting now");
    error(0,0,uVar2);
                    /* WARNING: Subroutine does not return */
    FUN_001177f4();
  }
  DAT_0019f044 = DAT_0019f044 + 1;
  if (10 < iVar1) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar2 = gettext("Too many errors, quitting");
    error(0,0,uVar2);
                    /* WARNING: Subroutine does not return */
    FUN_001177f4();
  }
  return;
}




// Function: archive_is_dev @ 0xc7e6

undefined4 archive_is_dev(void)

{
  int iVar1;
  undefined4 uVar2;
  long in_FS_OFFSET;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = fstat(DAT_0019f9cc,&local_a8);
  if (iVar1 == 0) {
    if (((local_a8.st_mode & 0xf000) == 0x6000) || ((local_a8.st_mode & 0xf000) == 0x2000)) {
      uVar2 = 1;
    }
    else {
      uVar2 = 0;
    }
  }
  else {
    FUN_0012b444(*DAT_0019fbe8);
    uVar2 = 0;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: short_read @ 0xc881

void short_read(ulong param_1)

{
  char cVar1;
  undefined8 uVar2;
  long lVar3;
  ulong local_30;
  ulong local_28;
  long local_20;
  
  local_20 = param_1 + DAT_0019efa0;
  local_28 = DAT_0019f870 - param_1;
  local_30 = param_1;
  if (((((local_28 != 0) && ((local_28 & 0x1ff) == 0)) && ((DAT_0019ec84 & 0x400000) != 0)) &&
      ((DAT_0019f038 == 0 && (param_1 != 0)))) && (cVar1 = FUN_0010c7e6(), cVar1 != '\0')) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar2 = ngettext("Record size = %lu block","Record size = %lu blocks",param_1 >> 9);
    error(0,0,uVar2,param_1 >> 9);
  }
  while (((local_28 & 0x1ff) != 0 ||
         (((local_28 != 0 && (local_30 != 0)) && (DAT_0019f049 != '\0'))))) {
    if (local_30 != 0) {
      while( true ) {
        if (DAT_0019f9cc < 0x40000000) {
          local_30 = FUN_0015ec70(DAT_0019f9cc,local_20,local_28);
        }
        else {
          local_30 = FUN_0013fe62(DAT_0019f9cc + -0x40000000,local_20,local_28);
        }
        if (local_30 != 0xffffffffffffffff) break;
        FUN_0010c729();
      }
    }
    if (local_30 == 0) break;
    if (DAT_0019f049 != '\x01') {
      lVar3 = DAT_0019f870 - local_28;
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar2 = ngettext("Unaligned block (%lu byte) in archive",
                       "Unaligned block (%lu bytes) in archive",lVar3);
      error(0,0,uVar2,lVar3);
                    /* WARNING: Subroutine does not return */
      FUN_001177f4();
    }
    local_28 = local_28 - local_30;
    local_20 = local_20 + local_30;
  }
  DAT_0019efa8 = (DAT_0019f870 - local_28 & 0xfffffffffffffe00) + DAT_0019efa0;
  DAT_0019efc0 = DAT_0019efc0 + 1;
  return;
}




// Function: flush_archive @ 0xcac8

void flush_archive(void)

{
  long lVar1;
  
  if ((DAT_0019efb8 == 0) && (DAT_0019fd70 != '\0')) {
    DAT_0019efb8 = 1;
    DAT_0019fd70 = '\0';
    FUN_0010cc3a();
    if ((long)DAT_0019efa8 - DAT_0019efa0 >> 9 < (long)DAT_0019f868) {
      memset(DAT_0019efa8,0,((long)DAT_0019f868 - ((long)DAT_0019efa8 - DAT_0019efa0 >> 9)) * 0x200)
      ;
      DAT_0019efa8 = (void *)((long)DAT_0019f868 * 0x200 + DAT_0019efa0);
      return;
    }
  }
  lVar1 = DAT_0019efb0 - DAT_0019efa0;
  DAT_0019f038 = DAT_0019f038 + ((long)DAT_0019efa8 - DAT_0019efa0 >> 9);
  DAT_0019efb0 = DAT_0019efa0;
  DAT_0019efa8 = (void *)((long)DAT_0019f868 * 0x200 + DAT_0019efa0);
  if (DAT_0019efb8 != 2) {
    if (DAT_0019efb8 < 3) {
      if (DAT_0019efb8 == 0) {
        FUN_0010f662();
      }
      else if (DAT_0019efb8 == 1) {
        (*DAT_0019f050)(lVar1);
      }
    }
    return;
  }
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: backspace_output @ 0xcc3a

void backspace_output(void)

{
  char cVar1;
  char extraout_var_00;
  int iVar2;
  char extraout_var;
  int *piVar3;
  long lVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  long local_20;
  undefined2 local_18 [2];
  undefined4 local_14;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_18[0] = 4;
  local_14 = 1;
  if (DAT_0019f9cc < 0x40000000) {
    iVar2 = ioctl(DAT_0019f9cc,0x40086d01,local_18);
    cVar1 = (char)((uint)iVar2 >> 0x18);
  }
  else {
    FUN_00140254(DAT_0019f9cc + -0x40000000,0x40086d01,local_18);
    cVar1 = extraout_var;
  }
  if (cVar1 < '\0') {
    piVar3 = __errno_location();
    if (*piVar3 == 5) {
      if (DAT_0019f9cc < 0x40000000) {
        iVar2 = ioctl(DAT_0019f9cc,0x40086d01,local_18);
        cVar1 = (char)((uint)iVar2 >> 0x18);
      }
      else {
        FUN_00140254(DAT_0019f9cc + -0x40000000,0x40086d01,local_18);
        cVar1 = extraout_var_00;
      }
      if (-1 < cVar1) goto LAB_0010ce63;
    }
    if (DAT_0019f9cc < 0x40000000) {
      local_20 = lseek(DAT_0019f9cc,0,1);
    }
    else {
      local_20 = FUN_001400be(DAT_0019f9cc + -0x40000000,0,1);
    }
    local_20 = local_20 - (DAT_0019efa8 - (long)DAT_0019efa0);
    if (local_20 < 0) {
      local_20 = 0;
    }
    if (DAT_0019f9cc < 0x40000000) {
      lVar4 = lseek(DAT_0019f9cc,local_20,0);
    }
    else {
      lVar4 = FUN_001400be(DAT_0019f9cc + -0x40000000,local_20,0);
    }
    if (lVar4 != local_20) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar5 = gettext("Cannot backspace archive file; it may be unreadable without -i");
      error(0,0,uVar5);
      if (DAT_0019efa0 != DAT_0019fd78) {
        memset(DAT_0019efa0,0,(long)DAT_0019fd78 - (long)DAT_0019efa0);
      }
    }
  }
LAB_0010ce63:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: seek_archive @ 0xce79

ulong seek_archive(long param_1)

{
  long lVar1;
  long lVar2;
  ulong uVar3;
  undefined8 uVar4;
  ulong uVar5;
  
  lVar1 = FUN_0010bacd();
  lVar2 = (long)DAT_0019f868 - (DAT_0019efb0 - DAT_0019efa0 >> 9);
  if (lVar2 * 0x200 < param_1) {
    uVar3 = (ulong)(param_1 + lVar2 * -0x200) / DAT_0019f870;
    if (uVar3 == 0) {
      uVar3 = 0;
    }
    else {
      if (DAT_0019f9cc < 0x40000000) {
        uVar3 = lseek(DAT_0019f9cc,DAT_0019f870 * uVar3,1);
      }
      else {
        uVar3 = FUN_001400be(DAT_0019f9cc + -0x40000000,DAT_0019f870 * uVar3,1);
      }
      if (-1 < (long)uVar3) {
        if (uVar3 % DAT_0019f870 != 0) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar4 = gettext("rmtlseek not stopped at a record boundary");
          error(0,0,uVar4);
                    /* WARNING: Subroutine does not return */
          FUN_001177f4();
        }
        uVar5 = uVar3;
        if ((long)uVar3 < 0) {
          uVar5 = uVar3 + 0x1ff;
        }
        uVar3 = ((long)uVar5 >> 9) - lVar1;
        DAT_0019efc0 = DAT_0019efc0 + (long)uVar3 / (long)DAT_0019f868;
        DAT_0019f038 = ((long)uVar5 >> 9) - (long)DAT_0019f868;
        DAT_0019efb0 = DAT_0019efa8;
      }
    }
  }
  else {
    uVar3 = 0;
  }
  return uVar3;
}




// Function: close_archive @ 0xd046

void close_archive(void)

{
  int iVar1;
  
  if ((DAT_0019fd70 != '\0') || (DAT_0019efb8 == 1)) {
    do {
      FUN_0010cac8();
    } while (DAT_0019efa0 < DAT_0019efb0);
  }
  FUN_0010ae4d();
  if (DAT_0019f9b0 != '\0') {
    FUN_00111a1c();
  }
  if (DAT_0019f9cc < 0x40000000) {
    iVar1 = close(DAT_0019f9cc);
  }
  else {
    iVar1 = FUN_0013fe0c(DAT_0019f9cc + -0x40000000);
  }
  if (iVar1 != 0) {
    FUN_0013e1ff(*DAT_0019fbe8);
  }
  FUN_00132ca8(DAT_0019f040,DAT_0019f048);
  FUN_00139d1c(&DAT_0019fa20);
  free(DAT_0019f010);
  free(DAT_0019f018);
  FUN_0010ac2b(0);
  return;
}




// Function: write_fatal_details @ 0xd132

void write_fatal_details(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  int iVar1;
  
  FUN_0013eb4b(param_1,param_2,param_3);
  if (DAT_0019f9cc < 0x40000000) {
    iVar1 = close(DAT_0019f9cc);
  }
  else {
    iVar1 = FUN_0013fe0c(DAT_0019f9cc + -0x40000000);
  }
  if (iVar1 != 0) {
    FUN_0013e1ff(*DAT_0019fbe8);
  }
  FUN_00132ca8(DAT_0019f040,0);
                    /* WARNING: Subroutine does not return */
  FUN_001177f4();
}




// Function: init_volume_number @ 0xd1c6

void init_volume_number(void)

{
  int iVar1;
  FILE *__stream;
  undefined8 uVar2;
  undefined8 uVar3;
  int *piVar4;
  
  __stream = fopen(DAT_0019f9b8,"r");
  if (__stream == (FILE *)0x0) {
    piVar4 = __errno_location();
    if (*piVar4 != 2) {
      FUN_0013e38b(DAT_0019f9b8);
    }
  }
  else {
    iVar1 = __isoc99_fscanf(__stream,&DAT_00180384,&DAT_0019c014);
    if ((iVar1 != 1) || (DAT_0019c014 < 0)) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar2 = FUN_0015e548(DAT_0019f9b8);
      uVar3 = gettext("%s: contains invalid volume number");
      error(0,0,uVar3,uVar2);
                    /* WARNING: Subroutine does not return */
      FUN_001177f4();
    }
    iVar1 = ferror_unlocked(__stream);
    if (iVar1 != 0) {
      FUN_0013e403(DAT_0019f9b8);
    }
    iVar1 = fclose(__stream);
    if (iVar1 != 0) {
      FUN_0013e1ff(DAT_0019f9b8);
    }
  }
  return;
}




// Function: closeout_volume_number @ 0xd2e3

void closeout_volume_number(void)

{
  int iVar1;
  FILE *__stream;
  
  __stream = fopen(DAT_0019f9b8,"w");
  if (__stream == (FILE *)0x0) {
    FUN_0013e38b(DAT_0019f9b8);
  }
  else {
    fprintf(__stream,"%d\n",(ulong)DAT_0019c014);
    iVar1 = ferror_unlocked(__stream);
    if (iVar1 != 0) {
      FUN_0013eb22(DAT_0019f9b8);
    }
    iVar1 = fclose(__stream);
    if (iVar1 != 0) {
      FUN_0013e1ff(DAT_0019f9b8);
    }
  }
  return;
}




// Function: increase_volume_number @ 0xd386

void increase_volume_number(void)

{
  undefined8 uVar1;
  
  DAT_0019c014 = DAT_0019c014 + 1;
  if (DAT_0019c014 < 0) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar1 = gettext("Volume number overflow");
    error(0,0,uVar1);
                    /* WARNING: Subroutine does not return */
    FUN_001177f4();
  }
  DAT_0019c010 = DAT_0019c010 + 1;
  return;
}




// Function: change_tape_menu @ 0xd3f9

void change_tape_menu(FILE *param_1)

{
  uint uVar1;
  char cVar2;
  bool bVar3;
  undefined8 *puVar4;
  undefined8 uVar5;
  char *pcVar6;
  __ssize_t _Var7;
  long in_FS_OFFSET;
  char *local_40;
  size_t local_38;
  char *local_30;
  char *local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_40 = (char *)0x0;
  local_38 = 0;
  bVar3 = false;
LAB_0010d7df:
  if (bVar3) {
LAB_0010d7ee:
    free(local_40);
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      return;
    }
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  fputc_unlocked(7,stderr);
  uVar5 = FUN_0015e84e(*DAT_0019fbe8);
  uVar1 = DAT_0019c014 + 1;
  pcVar6 = (char *)gettext("Prepare volume #%d for %s and hit return: ");
  fprintf(stderr,pcVar6,(ulong)uVar1,uVar5);
  fflush_unlocked(stderr);
  _Var7 = getline(&local_40,&local_38,param_1);
  if (_Var7 < 1) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar5 = gettext("EOF where user reply was expected");
    error(0,0,uVar5);
    if (((DAT_0019f860 != 6) && (DAT_0019f860 != 7)) && (DAT_0019f860 != 5)) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar5 = gettext("WARNING: Archive is incomplete");
      error(0,0,uVar5);
    }
                    /* WARNING: Subroutine does not return */
    FUN_001177f4();
  }
  if (((*local_40 == '\n') || (*local_40 == 'y')) || (*local_40 == 'Y')) goto LAB_0010d7ee;
  cVar2 = *local_40;
  if (cVar2 == 'q') {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar5 = gettext("No new volume; exiting.\n");
    error(0,0,uVar5);
    if (((DAT_0019f860 != 6) && (DAT_0019f860 != 7)) && (DAT_0019f860 != 5)) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar5 = gettext("WARNING: Archive is incomplete");
      error(0,0,uVar5);
    }
                    /* WARNING: Subroutine does not return */
    FUN_001177f4();
  }
  if (cVar2 < 'r') {
    pcVar6 = local_40;
    if (cVar2 == 'n') {
      do {
        do {
          local_30 = pcVar6 + 1;
          pcVar6 = local_30;
        } while (*local_30 == ' ');
      } while (*local_30 == '\t');
      for (local_28 = local_30; (*local_28 != '\0' && (*local_28 != '\n')); local_28 = local_28 + 1)
      {
      }
      *local_28 = '\0';
      puVar4 = DAT_0019fbe8;
      if (*local_30 == '\0') {
        uVar5 = gettext("File name not specified. Try again.\n");
        fprintf(stderr,"%s",uVar5);
      }
      else {
        uVar5 = FUN_00160cc6(local_30);
        *puVar4 = uVar5;
        bVar3 = true;
      }
      goto LAB_0010d7df;
    }
    if ('n' < cVar2) goto LAB_0010d7b6;
    if (cVar2 == '!') {
      if (DAT_0019f9a9 != '\x01') {
        FUN_00132df0();
        goto LAB_0010d7df;
      }
    }
    else if (cVar2 == '?') {
      pcVar6 = (char *)gettext(
                              " n name        Give a new file name for the next (and subsequent) volume(s)\n q             Abort tar\n y or newline  Continue operation\n"
                              );
      fprintf(stderr,pcVar6);
      if (DAT_0019f9a9 != '\x01') {
        pcVar6 = (char *)gettext(" !             Spawn a subshell\n");
        fprintf(stderr,pcVar6);
      }
      pcVar6 = (char *)gettext(" ?             Print this list\n");
      fprintf(stderr,pcVar6);
      goto LAB_0010d7df;
    }
  }
LAB_0010d7b6:
  pcVar6 = (char *)gettext("Invalid input. Type ? for help.\n");
  fprintf(stderr,pcVar6);
  goto LAB_0010d7df;
}




// Function: new_volume @ 0xd818

undefined8 new_volume(uint param_1)

{
  int iVar1;
  FILE *pFVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  void *pvVar5;
  int local_1c;
  
  pFVar2 = DAT_0019f248;
  if (((DAT_0019f248 == (FILE *)0x0) && (DAT_0019f8c0 == 0)) && (pFVar2 = stdin, DAT_0019f9cc == 0))
  {
    pFVar2 = fopen("/dev/tty","r");
  }
  DAT_0019f248 = pFVar2;
  if (DAT_0019f320 == '\0') {
    if (DAT_0019f9b0 != '\0') {
      FUN_00111a1c();
    }
    FUN_00129547(&DAT_0019efe0,0);
    FUN_00129547(&DAT_0019efe8,0);
    DAT_0019eff8 = 0;
    DAT_0019eff0 = 0;
    DAT_0019efb0 = DAT_0019efa0;
    if (DAT_0019f9cc < 0x40000000) {
      iVar1 = close(DAT_0019f9cc);
    }
    else {
      iVar1 = FUN_0013fe0c(DAT_0019f9cc + -0x40000000);
    }
    if (iVar1 != 0) {
      FUN_0013e1ff(*DAT_0019fbe8);
    }
    DAT_0019fbe8 = DAT_0019fbe8 + 1;
    if (DAT_0019fbd8 + DAT_0019fbe0 == DAT_0019fbe8) {
      DAT_0019fbe8 = DAT_0019fbd8;
      DAT_0019f250 = 1;
    }
    local_1c = DAT_0019f250;
    while( true ) {
      if (local_1c != 0) {
        if (DAT_0019f8c0 == 0) {
          FUN_0010d3f9(DAT_0019f248);
        }
        else {
          if (DAT_0019f9b8 != 0) {
            FUN_0010d2e3();
          }
          iVar1 = FUN_001345e7(DAT_0019fbe8,DAT_0019c014 + 1);
          if (iVar1 != 0) {
            if (DAT_0019fdc8 != (code *)0x0) {
              (*DAT_0019fdc8)();
            }
            uVar3 = FUN_0015e84e(DAT_0019f8c0);
            uVar4 = gettext("%s command failed");
            error(0,0,uVar4,uVar3);
                    /* WARNING: Subroutine does not return */
            FUN_001177f4();
          }
        }
      }
      iVar1 = strcmp((char *)*DAT_0019fbe8,"-");
      if (iVar1 == 0) {
        DAT_0019f049 = 1;
        DAT_0019f9cc = 0;
      }
      else if (DAT_0019f9b0 == '\0') {
        if (param_1 == 2) {
          if (((DAT_0019fdf8 == '\x01') ||
              (DAT_0019fdf0 = strchr((char *)*DAT_0019fbe8,0x3a), DAT_0019fdf0 == (char *)0x0)) ||
             ((DAT_0019fdf0 <= (char *)*DAT_0019fbe8 ||
              (pvVar5 = memchr((void *)*DAT_0019fbe8,0x2f,(long)DAT_0019fdf0 - *DAT_0019fbe8),
              pvVar5 != (void *)0x0)))) {
            DAT_0019f9cc = open((char *)*DAT_0019fbe8,0x42,0x1b6);
          }
          else {
            DAT_0019f9cc = FUN_0013f860(*DAT_0019fbe8,0x42,0x40000000,DAT_0019f950);
          }
        }
        else if (param_1 < 3) {
          if (param_1 == 0) {
            if (((DAT_0019fdf8 == '\x01') ||
                (DAT_0019fdf0 = strchr((char *)*DAT_0019fbe8,0x3a), DAT_0019fdf0 == (char *)0x0)) ||
               ((DAT_0019fdf0 <= (char *)*DAT_0019fbe8 ||
                (pvVar5 = memchr((void *)*DAT_0019fbe8,0x2f,(long)DAT_0019fdf0 - *DAT_0019fbe8),
                pvVar5 != (void *)0x0)))) {
              DAT_0019f9cc = open((char *)*DAT_0019fbe8,0,0x1b6);
            }
            else {
              DAT_0019f9cc = FUN_0013f860(*DAT_0019fbe8,0,0x40000000,DAT_0019f950);
            }
            FUN_0010b214();
          }
          else if (param_1 == 1) {
            if (DAT_0019f884 != '\0') {
              FUN_0012a571(*DAT_0019fbe8,1);
            }
            if ((((DAT_0019fdf8 == '\x01') ||
                 (DAT_0019fdf0 = strchr((char *)*DAT_0019fbe8,0x3a), DAT_0019fdf0 == (char *)0x0))
                || (DAT_0019fdf0 <= (char *)*DAT_0019fbe8)) ||
               (pvVar5 = memchr((void *)*DAT_0019fbe8,0x2f,(long)DAT_0019fdf0 - *DAT_0019fbe8),
               pvVar5 != (void *)0x0)) {
              DAT_0019f9cc = creat((char *)*DAT_0019fbe8,0x1b6);
            }
            else {
              DAT_0019f9cc = FUN_0013f860(*DAT_0019fbe8,0x41,0x40000000,DAT_0019f950);
            }
          }
        }
      }
      else if (((DAT_0019fdf8 == '\x01') ||
               (DAT_0019fdf0 = strchr((char *)*DAT_0019fbe8,0x3a), DAT_0019fdf0 == (char *)0x0)) ||
              ((DAT_0019fdf0 <= (char *)*DAT_0019fbe8 ||
               (pvVar5 = memchr((void *)*DAT_0019fbe8,0x2f,(long)DAT_0019fdf0 - *DAT_0019fbe8),
               pvVar5 != (void *)0x0)))) {
        DAT_0019f9cc = open((char *)*DAT_0019fbe8,0x42,0x1b6);
      }
      else {
        DAT_0019f9cc = FUN_0013f860(*DAT_0019fbe8,0x42,0x40000000,DAT_0019f950);
      }
      if (-1 < DAT_0019f9cc) break;
      FUN_0013e3da(*DAT_0019fbe8);
      if (((DAT_0019f9b0 != '\x01') && (param_1 == 1)) && (DAT_0019f884 != '\0')) {
        FUN_0012a874();
      }
      local_1c = 1;
    }
    uVar3 = 1;
  }
  else {
    uVar3 = 0;
  }
  return uVar3;
}




// Function: read_header0 @ 0xde84

undefined8 read_header0(undefined8 param_1)

{
  int iVar1;
  undefined8 uVar2;
  
  FUN_00139c55(param_1);
  iVar1 = FUN_00125ecf(&DAT_0019f460,param_1,0);
  if (iVar1 == 1) {
    FUN_0010bba7(DAT_0019f460);
    uVar2 = 1;
  }
  else {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar2 = gettext("This does not look like a tar archive");
    error(0,0,uVar2);
    DAT_0019fdd0 = 2;
    uVar2 = 0;
  }
  return uVar2;
}




// Function: try_new_volume @ 0xdf26

undefined8 try_new_volume(void)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  size_t sVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  long in_FS_OFFSET;
  undefined4 local_a4;
  long local_a0;
  long local_98;
  ulong local_90;
  undefined1 local_88 [32];
  undefined1 local_68 [32];
  undefined1 local_48 [24];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0019f860 < 3) {
    if (DAT_0019f860 != 0) goto LAB_0010df5e;
LAB_0010df6a:
    local_a4 = 0;
  }
  else {
    if (DAT_0019f860 != 8) goto LAB_0010df6a;
LAB_0010df5e:
    local_a4 = 2;
  }
  cVar1 = FUN_0010d818(local_a4);
  if (cVar1 != '\x01') {
    uVar3 = 1;
    goto LAB_0010e65f;
  }
  while( true ) {
    if (DAT_0019f9cc < 0x40000000) {
      local_98 = FUN_0015ec70(DAT_0019f9cc,DAT_0019efa0,DAT_0019f870);
    }
    else {
      local_98 = FUN_0013fe62(DAT_0019f9cc + -0x40000000,DAT_0019efa0,DAT_0019f870);
    }
    if (local_98 != -1) break;
    FUN_0010c729();
  }
  if (local_98 != DAT_0019f870) {
    FUN_0010c881(local_98);
  }
  local_a0 = FUN_0010bb4b();
  if (local_a0 == 0) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar3 = gettext("This does not look like a tar archive");
    error(0,0,uVar3);
    uVar3 = 0;
    goto LAB_0010e65f;
  }
  cVar1 = *(char *)(local_a0 + 0x9c);
  if (cVar1 == 'g') {
    FUN_00139c55(&DAT_0019f080);
    iVar2 = FUN_00125ecf(&local_a0,&DAT_0019f080,2);
    if (iVar2 != 2) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar3 = gettext("This does not look like a tar archive");
      error(0,0,uVar3);
      uVar3 = 0;
      goto LAB_0010e65f;
    }
    FUN_0011df5f(&DAT_0019f080);
    FUN_00139d1c(&DAT_0019f080);
    iVar2 = FUN_00125ecf(&local_a0,&DAT_0019f080,0);
    if (iVar2 == 1) {
      FUN_0010bba7(local_a0);
    }
    else if (iVar2 != 5) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar3 = gettext("This does not look like a tar archive");
      error(0,0,uVar3);
      uVar3 = 0;
      goto LAB_0010e65f;
    }
  }
  else if (cVar1 < 'h') {
    if (cVar1 == 'M') {
LAB_0010e24b:
      cVar1 = FUN_0010de84(&DAT_0019f080);
      if (cVar1 != '\x01') {
        uVar3 = 0;
        goto LAB_0010e65f;
      }
      FUN_00139d1c(&DAT_0019f080);
      FUN_00129591(&DAT_0019efe8,DAT_0019f460,100);
      DAT_0019eff0 = FUN_00127a73(DAT_0019f460 + 0x7c,0xc);
      DAT_0019eff8 = FUN_00127a73(DAT_0019f460 + 0x171,0xc);
    }
    else if (cVar1 == 'V') {
      cVar1 = FUN_0010de84(&DAT_0019f080);
      if (cVar1 != '\x01') {
        uVar3 = 0;
        goto LAB_0010e65f;
      }
      FUN_00139d1c(&DAT_0019f080);
      FUN_00129591(&DAT_0019efe0,DAT_0019f460,100);
      FUN_0010bba7(local_a0);
      local_a0 = FUN_0010bb4b();
      if (*(char *)(local_a0 + 0x9c) == 'M') goto LAB_0010e24b;
    }
  }
  if (DAT_0019f060 != 0) {
    if (DAT_0019efe8 == (char *)0x0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar3 = FUN_0015e84e(*(undefined8 *)(DAT_0019f060 + 0x10));
      uVar4 = gettext("%s is not continued on this volume");
      error(0,0,uVar4,uVar3);
      uVar3 = 0;
      goto LAB_0010e65f;
    }
    iVar2 = strcmp(DAT_0019efe8,*(char **)(DAT_0019f060 + 0x10));
    if (iVar2 != 0) {
      if ((((DAT_0019f864 != 6) && (DAT_0019f864 != 2)) ||
          (sVar5 = strlen(*(char **)(DAT_0019f060 + 0x10)), sVar5 < 100)) ||
         (iVar2 = strncmp(DAT_0019efe8,*(char **)(DAT_0019f060 + 0x10),100), iVar2 != 0)) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar3 = FUN_0015e84e(*(undefined8 *)(DAT_0019f060 + 0x10));
        uVar4 = gettext("%s is not continued on this volume");
        error(0,0,uVar4,uVar3);
        uVar3 = 0;
        goto LAB_0010e65f;
      }
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar3 = FUN_0015e84e(*(undefined8 *)(DAT_0019f060 + 0x10));
      uVar4 = gettext("%s is possibly continued on this volume: header contains truncated name");
      error(0,0,uVar4,uVar3);
    }
    local_90 = DAT_0019eff8 + DAT_0019eff0;
    if ((*(ulong *)(DAT_0019f060 + 0x18) != local_90) || (local_90 < DAT_0019eff8)) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar3 = FUN_00154e5a(DAT_0019eff8,local_48);
      uVar4 = FUN_00154e5a(DAT_0019eff0,local_68);
      uVar6 = FUN_00154e5a(*(undefined8 *)(DAT_0019f060 + 0x18),local_88);
      uVar7 = FUN_0015e84e(DAT_0019efe8);
      uVar8 = gettext("%s is the wrong size (%s != %s + %s)");
      error(0,0,uVar8,uVar7,uVar6,uVar4,uVar3);
      uVar3 = 0;
      goto LAB_0010e65f;
    }
    if (*(long *)(DAT_0019f060 + 0x18) - *(long *)(DAT_0019f060 + 0x20) != DAT_0019eff8) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar3 = FUN_00154e5a(DAT_0019eff8,local_48);
      uVar4 = FUN_00154e5a(*(undefined8 *)(DAT_0019f060 + 0x20),local_68);
      uVar6 = FUN_00154e5a(*(undefined8 *)(DAT_0019f060 + 0x18),local_88);
      uVar7 = gettext("This volume is out of sequence (%s - %s != %s)");
      error(0,0,uVar7,uVar6,uVar4,uVar3);
      uVar3 = 0;
      goto LAB_0010e65f;
    }
  }
  FUN_0010d386();
  uVar3 = 1;
LAB_0010e65f:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: drop_volume_label_suffix @ 0xe680

void * drop_volume_label_suffix(byte *param_1)

{
  int iVar1;
  size_t sVar2;
  ushort **ppuVar3;
  void *__dest;
  byte *local_20;
  
  sVar2 = strlen((char *)param_1);
  if (sVar2 != 0) {
    local_20 = param_1 + (sVar2 - 1);
    while ((param_1 < local_20 && (ppuVar3 = __ctype_b_loc(), ((*ppuVar3)[*local_20] & 0x800) != 0))
          ) {
      local_20 = local_20 + -1;
    }
    if ((param_1 < local_20) && (param_1 < local_20 + -7)) {
      iVar1 = memcmp(local_20 + -7," Volume ",8);
      if (iVar1 == 0) {
        sVar2 = (long)(local_20 + -7) - (long)param_1;
        __dest = (void *)FUN_00160b5b(sVar2 + 1);
        memcpy(__dest,param_1,sVar2);
        *(undefined1 *)(sVar2 + (long)__dest) = 0;
        return __dest;
      }
    }
  }
  return (void *)0x0;
}




// Function: check_label_pattern @ 0xe78a

bool check_label_pattern(char *param_1)

{
  int iVar1;
  char *__pattern;
  undefined1 local_11;
  
  local_11 = false;
  iVar1 = fnmatch(DAT_0019f9c0,param_1,0);
  if (iVar1 == 0) {
    local_11 = true;
  }
  else if (DAT_0019f8fc == '\x01') {
    __pattern = (char *)FUN_0010e680(param_1);
    if (__pattern != (char *)0x0) {
      iVar1 = fnmatch(__pattern,DAT_0019f9c0,0);
      local_11 = iVar1 == 0;
      free(__pattern);
    }
  }
  else {
    local_11 = false;
  }
  return local_11;
}




// Function: match_volume_label @ 0xe825

void match_volume_label(void)

{
  char cVar1;
  long lVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  undefined1 local_1d8 [360];
  undefined1 auStack_70 [80];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0019efe0 == 0) {
    lVar2 = FUN_0010bb4b();
    if (lVar2 == 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar3 = FUN_0015e84e(DAT_0019f9c0);
      uVar4 = gettext("Archive not labeled to match %s");
      error(0,0,uVar4,uVar3);
                    /* WARNING: Subroutine does not return */
      FUN_001177f4();
    }
    if (*(char *)(lVar2 + 0x9c) == 'V') {
      FUN_00129591(&DAT_0019efe0,lVar2,100);
    }
    else if (*(char *)(lVar2 + 0x9c) == 'g') {
      FUN_00139c55(local_1d8);
      uVar3 = FUN_0012798b(lVar2 + 0x7c,0xc);
      FUN_0011e28e(auStack_70,lVar2,uVar3);
      FUN_0011df5f(local_1d8);
      FUN_00139d1c(local_1d8);
    }
  }
  if (DAT_0019efe0 == 0) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar3 = FUN_0015e84e(DAT_0019f9c0);
    uVar4 = gettext("Archive not labeled to match %s");
    error(0,0,uVar4,uVar3);
                    /* WARNING: Subroutine does not return */
    FUN_001177f4();
  }
  cVar1 = FUN_0010e78a(DAT_0019efe0);
  if (cVar1 != '\x01') {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar3 = FUN_0015e821(1,DAT_0019f9c0);
    uVar4 = FUN_0015e821(0,DAT_0019efe0);
    uVar5 = gettext("Volume %s does not match %s");
    error(0,0,uVar5,uVar4,uVar3);
                    /* WARNING: Subroutine does not return */
    FUN_001177f4();
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: _write_volume_label @ 0xea79

void _write_volume_label(char *param_1)

{
  char *__dest;
  
  if (DAT_0019f864 == 4) {
    FUN_0011e1d3("GNU.volume.label",&DAT_0019f080,param_1);
  }
  else {
    __dest = (char *)FUN_0010bb4b();
    memset(__dest,0,0x200);
    strcpy(__dest,param_1);
    FUN_00129547(&DAT_0019fa28,__dest);
    DAT_0019fa30 = FUN_0014e2e5(DAT_0019fa28);
    __dest[0x9c] = 'V';
    FUN_00112af2(DAT_0019f9e0,__dest + 0x88,0xc);
    FUN_0011422e(&DAT_0019fa20,__dest,0xffffffffffffffff);
    FUN_0010bba7(__dest);
  }
  return;
}




// Function: add_volume_label @ 0xeb6f

void add_volume_label(void)

{
  char *__s;
  size_t sVar1;
  size_t sVar2;
  char *__s_00;
  long in_FS_OFFSET;
  undefined1 local_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  __s = (char *)FUN_00154e5a((long)DAT_0019c010,local_38);
  sVar1 = strlen(DAT_0019f9c0);
  sVar2 = strlen(__s);
  __s_00 = (char *)FUN_00160b5b(sVar2 + sVar1 + 9);
  sprintf(__s_00,"%s %s %s",DAT_0019f9c0,"Volume",__s);
  FUN_0010ea79(__s_00);
  free(__s_00);
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: add_chunk_header @ 0xec37

void add_chunk_header(long param_1)

{
  long lVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  void *local_1c8;
  void *local_1c0;
  undefined4 local_158;
  __uid_t local_154;
  __gid_t local_150;
  undefined8 local_140;
  undefined8 local_b0;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0019f864 == 4) {
    memset(&local_1c8,0,0x1b8);
    local_1c8 = *(void **)(param_1 + 0x10);
    local_158 = 0x81a4;
    local_1c0 = local_1c8;
    local_154 = getuid();
    local_150 = getgid();
    local_1c8 = (void *)FUN_0011cd02(&local_1c8,"%d/GNUFileParts/%f.%n",(long)DAT_0019c010);
    local_140 = *(undefined8 *)(param_1 + 0x20);
    local_1c0 = local_1c8;
    local_b0 = local_140;
    lVar1 = FUN_0011381d(&local_1c8);
    if (lVar1 == 0) {
                    /* WARNING: Subroutine does not return */
      abort();
    }
    uVar2 = FUN_001135e3(0,&local_1c8,lVar1);
    FUN_0011418c(uVar2);
    free(local_1c8);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: write_volume_label @ 0xed89

void write_volume_label(void)

{
  if (DAT_0019f8fc == '\0') {
    FUN_0010ea79(DAT_0019f9c0);
  }
  else {
    FUN_0010eb6f();
  }
  return;
}




// Function: gnu_add_multi_volume_header @ 0xedb5

void gnu_add_multi_volume_header(long param_1)

{
  undefined4 uVar1;
  void *__s;
  undefined8 uVar2;
  undefined8 uVar3;
  size_t local_28;
  
  __s = (void *)FUN_0010bb4b();
  local_28 = strlen(*(char **)(param_1 + 0x10));
  if (100 < local_28) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar2 = FUN_0015e548(*(undefined8 *)(param_1 + 0x10));
    uVar3 = gettext("%s: file name too long to be stored in a GNU multivolume header, truncated");
    error(0,0,uVar3,uVar2);
    local_28 = 100;
  }
  memset(__s,0,0x200);
  memcpy(__s,*(void **)(param_1 + 0x10),local_28);
  *(undefined1 *)((long)__s + 0x9c) = 0x4d;
  FUN_00112a9a(*(undefined8 *)(param_1 + 0x20),(long)__s + 0x7c,0xc);
  FUN_00112a9a(*(long *)(param_1 + 0x18) - *(long *)(param_1 + 0x20),(long)__s + 0x171,0xc);
  uVar1 = DAT_0019f9ac;
  DAT_0019f9ac = 0;
  FUN_0011422e(&DAT_0019fa20,__s,0xffffffffffffffff);
  DAT_0019f9ac = uVar1;
  FUN_0010bba7(__s);
  return;
}




// Function: add_multi_volume_header @ 0xef1c

void add_multi_volume_header(long param_1)

{
  long in_FS_OFFSET;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0019f864 == 4) {
    local_18 = *(long *)(param_1 + 0x18) - *(long *)(param_1 + 0x20);
    FUN_0011e1d3("GNU.volume.filename",&DAT_0019f080,*(undefined8 *)(param_1 + 0x10));
    FUN_0011e1d3("GNU.volume.size",&DAT_0019f080,param_1 + 0x20);
    FUN_0011e1d3("GNU.volume.offset",&DAT_0019f080,&local_18);
  }
  else {
    FUN_0010edb5(param_1);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: simple_flush_read @ 0xeff1

void simple_flush_read(void)

{
  long lVar1;
  
  FUN_001106d8(0);
  DAT_0019f044 = 0;
  if ((DAT_0019efd8 != '\0') && (DAT_0019f038 != 0)) {
    DAT_0019f9cc = 1;
    lVar1 = FUN_0013301a();
    DAT_0019f9cc = 0;
    if (lVar1 != DAT_0019f870) {
      FUN_0010c6d5(lVar1);
    }
  }
  while( true ) {
    if (DAT_0019f9cc < 0x40000000) {
      lVar1 = FUN_0015ec70(DAT_0019f9cc,DAT_0019efa0,DAT_0019f870);
    }
    else {
      lVar1 = FUN_0013fe62(DAT_0019f9cc + -0x40000000,DAT_0019efa0,DAT_0019f870);
    }
    if (lVar1 == DAT_0019f870) break;
    if (lVar1 != -1) {
      FUN_0010c881(lVar1);
      return;
    }
    FUN_0010c729();
  }
  DAT_0019efc0 = DAT_0019efc0 + 1;
  return;
}




// Function: simple_flush_write @ 0xf0fb

void simple_flush_write(void)

{
  long lVar1;
  
  lVar1 = FUN_0010c54f();
  if (lVar1 != DAT_0019f870) {
    FUN_0010c6d5(lVar1);
  }
  DAT_0019efc8 = DAT_0019efc8 + 1;
  DAT_0019f008 = DAT_0019f008 + (double)lVar1;
  return;
}




// Function: _gnu_flush_read @ 0xf160

void _gnu_flush_read(void)

{
  char cVar1;
  long lVar2;
  int *piVar3;
  
  FUN_001106d8(0);
  DAT_0019f044 = 0;
  if ((DAT_0019efd8 != '\0') && (DAT_0019f038 != 0)) {
    DAT_0019f9cc = 1;
    lVar2 = FUN_0013301a();
    DAT_0019f9cc = 0;
    if (lVar2 != DAT_0019f870) {
      FUN_0010c6d5(lVar2);
    }
  }
  while( true ) {
    if (DAT_0019f9cc < 0x40000000) {
      lVar2 = FUN_0015ec70(DAT_0019f9cc,DAT_0019efa0,DAT_0019f870);
    }
    else {
      lVar2 = FUN_0013fe62(DAT_0019f9cc + -0x40000000,DAT_0019efa0,DAT_0019f870);
    }
    if (lVar2 == DAT_0019f870) break;
    if (((lVar2 == 0) || ((lVar2 == -1 && (piVar3 = __errno_location(), *piVar3 == 0x1c)))) &&
       (DAT_0019f8fc != '\0')) {
      do {
        cVar1 = FUN_0010df26();
      } while (cVar1 != '\x01');
      if (DAT_0019efb0 == DAT_0019efa8) {
        FUN_0010cac8();
      }
      return;
    }
    if (lVar2 != -1) {
      FUN_0010c881(lVar2);
      return;
    }
    FUN_0010c729();
  }
  DAT_0019efc0 = DAT_0019efc0 + 1;
  return;
}




// Function: gnu_flush_read @ 0xf2b9

void gnu_flush_read(void)

{
  DAT_0019f058 = FUN_0010eff1;
  FUN_0010f160();
  DAT_0019f058 = gnu_flush_read;
  return;
}




// Function: _gnu_flush_write @ 0xf2e5

void _gnu_flush_write(long param_1)

{
  char cVar1;
  long lVar2;
  undefined8 uVar3;
  int *piVar4;
  void *local_38;
  void *local_30;
  ulong local_28;
  ulong local_20;
  
  local_28 = FUN_0010c54f();
  if ((local_28 != DAT_0019f870) && (DAT_0019f8fc != '\x01')) {
    FUN_0010c6d5(local_28);
  }
  if (local_28 != 0) {
    DAT_0019efc8 = DAT_0019efc8 + 1;
  }
  DAT_0019f008 = DAT_0019f008 + (double)(long)local_28;
  if (local_28 != DAT_0019f870) {
    lVar2 = FUN_0010abd9(local_28);
    if ((local_28 & 0x1ff) != 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar3 = gettext("write did not end on a block boundary");
      error(0,0,uVar3);
      DAT_0019fdd0 = 2;
      FUN_0010c6d5(local_28);
    }
    if (((((long)local_28 < 0) && (piVar4 = __errno_location(), *piVar4 != 0x1c)) &&
        (piVar4 = __errno_location(), *piVar4 != 5)) && (piVar4 = __errno_location(), *piVar4 != 6))
    {
      FUN_0010c6d5(local_28);
    }
    cVar1 = FUN_0010d818(1);
    if (cVar1 == '\x01') {
      FUN_00139d1c(&DAT_0019f080);
      FUN_0010d386();
      DAT_0019f000 = DAT_0019f008 + DAT_0019f000;
      DAT_0019f008 = 0.0;
      local_30 = (void *)(local_28 + DAT_0019efa0);
      local_28 = param_1 - local_28;
      DAT_0019f030 = (uint)(DAT_0019f030 == 0);
      FUN_0010bc48();
      DAT_0019f070 = 1;
      if (DAT_0019f9c0 != 0) {
        FUN_0010eb6f();
      }
      if (lVar2 != 0) {
        FUN_0010ef1c(lVar2);
      }
      uVar3 = FUN_0010bb4b();
      FUN_001135e3(1,&DAT_0019f080,uVar3);
      FUN_00139d1c(&DAT_0019f080);
      if (lVar2 != 0) {
        FUN_0010ec37(lVar2);
      }
      local_38 = (void *)FUN_0010bb4b();
      FUN_0010acb0(lVar2,(long)local_38 - DAT_0019efa0 >> 9);
      local_20 = FUN_0010bbf5(local_38);
      DAT_0019f070 = 0;
      while (local_20 < local_28) {
        memcpy(local_38,local_30,local_20);
        local_30 = (void *)((long)local_30 + local_20);
        local_28 = local_28 - local_20;
        FUN_0010bba7((long)local_38 + (local_20 - 1 & 0xfffffffffffffe00));
        local_38 = (void *)FUN_0010bb4b();
        local_20 = FUN_0010bbf5(local_38);
      }
      memcpy(local_38,local_30,local_28);
      memset((void *)((long)local_38 + local_28),0,local_20 - local_28);
      FUN_0010bba7((long)local_38 + (local_28 - 1 & 0xfffffffffffffe00));
      FUN_0010bb4b();
    }
  }
  return;
}




// Function: gnu_flush_write @ 0xf627

void gnu_flush_write(undefined8 param_1)

{
  DAT_0019f050 = FUN_0010f0fb;
  FUN_0010f2e5(param_1);
  DAT_0019f050 = gnu_flush_write;
  return;
}




// Function: flush_read @ 0xf662

void flush_read(void)

{
  (*DAT_0019f058)();
  return;
}




// Function: flush_write @ 0xf676

void flush_write(void)

{
  (*DAT_0019f050)(DAT_0019f870);
  return;
}




// Function: open_archive @ 0xf694

void open_archive(uint param_1)

{
  DAT_0019f058 = FUN_0010f2b9;
  DAT_0019f050 = FUN_0010f627;
  FUN_0010bdb0(param_1);
  if (param_1 != 2) {
    if (2 < param_1) goto LAB_0010f718;
    if (param_1 != 0) {
      if ((param_1 == 1) && (DAT_0019efc8 = 0, DAT_0019f9c0 != 0)) {
        FUN_0010ed89();
      }
      goto LAB_0010f718;
    }
  }
  if (DAT_0019f9c0 != 0) {
    FUN_0010e825();
  }
LAB_0010f718:
  FUN_0010ae17();
  return;
}




// Function: alloc_action @ 0xf720

long alloc_action(undefined4 param_1)

{
  long lVar1;
  long lVar2;
  
  lVar2 = FUN_00160c06(0x18);
  lVar1 = lVar2;
  if (DAT_0019f270 != (long *)0x0) {
    *DAT_0019f270 = lVar2;
    lVar1 = DAT_0019f268;
  }
  DAT_0019f268 = lVar1;
  DAT_0019f270 = (long *)lVar2;
  *(undefined4 *)(lVar2 + 8) = param_1;
  return lVar2;
}




// Function: copy_string_unquote @ 0xf77f

char * copy_string_unquote(undefined8 param_1)

{
  char *__s;
  size_t sVar1;
  
  __s = (char *)FUN_00160cc6(param_1);
  sVar1 = strlen(__s);
  if (((*__s == '\"') || (*__s == '\'')) && (__s[sVar1 - 1] == *__s)) {
    memmove(__s,__s + 1,sVar1 - 2);
    __s[sVar1 - 2] = '\0';
  }
  FUN_0012961d(__s);
  return __s;
}




// Function: checkpoint_compile_action @ 0xf825

void checkpoint_compile_action(char *param_1)

{
  int iVar1;
  undefined4 uVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  char *local_28;
  long local_20;
  ulong local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0019f278 == 0) {
    sigemptyset((sigset_t *)&DAT_0019f280);
    DAT_0019f278 = 1;
  }
  iVar1 = strcmp(param_1,".");
  if (iVar1 != 0) {
    iVar1 = strcmp(param_1,"dot");
    if (iVar1 != 0) {
      iVar1 = strcmp(param_1,"bell");
      if (iVar1 == 0) {
        FUN_0010f720(1);
      }
      else {
        iVar1 = strcmp(param_1,"echo");
        if (iVar1 == 0) {
          FUN_0010f720(2);
        }
        else {
          iVar1 = strncmp(param_1,"echo=",5);
          if (iVar1 == 0) {
            local_20 = FUN_0010f720(2);
            uVar3 = FUN_0010f77f(param_1 + 5);
            *(undefined8 *)(local_20 + 0x10) = uVar3;
          }
          else {
            iVar1 = strncmp(param_1,"exec=",5);
            if (iVar1 == 0) {
              local_20 = FUN_0010f720(5);
              uVar3 = FUN_0010f77f(param_1 + 5);
              *(undefined8 *)(local_20 + 0x10) = uVar3;
            }
            else {
              iVar1 = strncmp(param_1,"ttyout=",7);
              if (iVar1 == 0) {
                local_20 = FUN_0010f720(3);
                uVar3 = FUN_0010f77f(param_1 + 7);
                *(undefined8 *)(local_20 + 0x10) = uVar3;
              }
              else {
                iVar1 = strncmp(param_1,"sleep=",6);
                if (iVar1 == 0) {
                  local_18 = strtoul(param_1 + 6,&local_28,10);
                  if (*local_28 != '\0') {
                    if (DAT_0019fdc8 != (code *)0x0) {
                      (*DAT_0019fdc8)();
                    }
                    uVar3 = gettext("%s: not a valid timeout");
                    error(0,0,uVar3,param_1);
                    /* WARNING: Subroutine does not return */
                    FUN_001177f4();
                  }
                  local_20 = FUN_0010f720(4);
                  *(ulong *)(local_20 + 0x10) = local_18;
                }
                else {
                  iVar1 = strcmp(param_1,"totals");
                  if (iVar1 == 0) {
                    FUN_0010f720(6);
                  }
                  else {
                    iVar1 = strncmp(param_1,"wait=",5);
                    if (iVar1 != 0) {
                      if (DAT_0019fdc8 != (code *)0x0) {
                        (*DAT_0019fdc8)();
                      }
                      uVar3 = gettext("%s: unknown checkpoint action");
                      error(0,0,uVar3,param_1);
                    /* WARNING: Subroutine does not return */
                      FUN_001177f4();
                    }
                    local_20 = FUN_0010f720(7);
                    uVar2 = FUN_00135562(param_1 + 5);
                    *(undefined4 *)(local_20 + 0x10) = uVar2;
                    sigaddset((sigset_t *)&DAT_0019f280,*(int *)(local_20 + 0x10));
                  }
                }
              }
            }
          }
        }
      }
      goto LAB_0010fb5c;
    }
  }
  FUN_0010f720(0);
LAB_0010fb5c:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: checkpoint_finish_compile @ 0xfb72

void checkpoint_finish_compile(void)

{
  if (((DAT_0019f278 == 0) && (DAT_0019f890 != 0)) && (DAT_0019f268 == 0)) {
    FUN_0010f825(&DAT_001807b3);
  }
  if (DAT_0019f278 == 1) {
    sigprocmask(0,(sigset_t *)&DAT_0019f280,(sigset_t *)0x0);
    if (DAT_0019f890 == 0) {
      DAT_0019f890 = 10;
    }
    DAT_0019f278 = 2;
  }
  return;
}




// Function: getwidth @ 0xfbee

ulong getwidth(FILE *param_1)

{
  int iVar1;
  char *__nptr;
  ulong uVar2;
  long in_FS_OFFSET;
  undefined1 local_18 [2];
  ushort local_16;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = fileno(param_1);
  iVar1 = ioctl(iVar1,0x5413,local_18);
  if ((iVar1 == 0) && (local_16 != 0)) {
    uVar2 = (ulong)local_16;
  }
  else {
    __nptr = getenv("COLUMNS");
    if ((__nptr == (char *)0x0) || (uVar2 = strtol(__nptr,(char **)0x0,10), (long)uVar2 < 1)) {
      uVar2 = 0x50;
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: getarg @ 0xfca5

long getarg(char *param_1,long *param_2,long *param_3,ulong *param_4)

{
  char *pcVar1;
  ulong uVar2;
  long lVar3;
  
  if ((*param_1 == '{') && (pcVar1 = strchr(param_1 + 1,0x7d), pcVar1 != (char *)0x0)) {
    uVar2 = (long)pcVar1 - (long)param_1;
    if (*param_4 < uVar2) {
      *param_4 = uVar2;
      lVar3 = FUN_00160b8d(*param_3,*param_4);
      *param_3 = lVar3;
    }
    memcpy((void *)*param_3,param_1 + 1,uVar2 - 1);
    *(undefined1 *)((uVar2 - 1) + *param_3) = 0;
    *param_2 = (long)(pcVar1 + 1);
    return *param_3;
  }
  *param_2 = (long)param_1;
  return 0;
}




// Function: format_checkpoint_string @ 0xfd96

ulong format_checkpoint_string(FILE *param_1,ulong param_2,char *param_3,char param_4,undefined4 param_5)

{
  int iVar1;
  size_t sVar2;
  undefined8 uVar3;
  long lVar4;
  long in_FS_OFFSET;
  undefined8 uVar5;
  char *local_1c0;
  ulong local_1b8;
  int local_19c;
  char *local_198;
  char *local_190;
  undefined **local_188;
  char *local_180;
  char *local_178;
  char *local_170;
  tm *local_168;
  ulong local_160;
  timeval local_158;
  undefined *local_148 [4];
  ulong local_128;
  long local_120;
  undefined *local_f0;
  undefined1 local_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_4 == '\0') {
    local_180 = (char *)gettext(&DAT_0018085c);
  }
  else {
    local_180 = (char *)gettext("write");
  }
  local_178 = (char *)FUN_00154e5a(param_5,local_38);
  local_190 = (char *)0x0;
  local_1c0 = param_3;
  if (param_3 == (char *)0x0) {
    if (param_4 == '\0') {
      local_1c0 = (char *)gettext("Read checkpoint %u");
    }
    else {
      local_1c0 = (char *)gettext("Write checkpoint %u");
    }
  }
  local_198 = local_1c0;
  local_1b8 = param_2;
  do {
    if (*local_198 == '\0') {
      fflush_unlocked(param_1);
      if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return local_1b8;
    }
    if (*local_198 == '%') {
      local_198 = local_198 + 1;
      if ((*local_198 == '{') &&
         (local_190 = (char *)FUN_0010fca5(local_198,&local_198,&DAT_0019f310,&DAT_0019f318),
         local_190 == (char *)0x0)) {
        fputc_unlocked(0x25,param_1);
        fputc_unlocked((int)*local_198,param_1);
        local_1b8 = local_1b8 + 2;
      }
      else {
        iVar1 = (int)*local_198;
        if (iVar1 == 0x2a) {
          if (local_190 == (char *)0x0) {
            local_160 = FUN_0010fbee(param_1);
          }
          else {
            local_160 = strtol(local_190,(char **)0x0,10);
          }
          for (; local_1b8 < local_160; local_1b8 = local_1b8 + 1) {
            fputc_unlocked(0x20,param_1);
          }
          goto LAB_00110328;
        }
        if (((iVar1 < 0x2a) || (0x75 < iVar1)) || (iVar1 < 0x54)) {
switchD_0010ff8a_caseD_55:
          fputc_unlocked(0x25,param_1);
          fputc_unlocked((int)*local_198,param_1);
          local_1b8 = local_1b8 + 2;
        }
        else {
          switch(iVar1) {
          case 0x54:
            local_188 = &PTR_DAT_0019c140;
            FUN_0010ae4d();
            if (local_190 != (char *)0x0) {
              local_f0 = &DAT_0018088d;
              iVar1 = FUN_001461a6(local_190,&local_128,0x4644);
              if (iVar1 == 0) {
                for (local_19c = 0; (ulong)(long)local_19c < local_128; local_19c = local_19c + 1) {
                  local_148[local_19c] = *(undefined **)((long)local_19c * 8 + local_120);
                }
                for (; local_19c < 3; local_19c = local_19c + 1) {
                  local_148[local_19c] = (undefined *)0x0;
                }
                local_188 = local_148;
              }
              else {
                if (DAT_0019fdc8 != (code *)0x0) {
                  (*DAT_0019fdc8)();
                }
                uVar5 = FUN_0014647f(&local_128);
                uVar3 = gettext("cannot split string \'%s\': %s");
                error(0,0,uVar3,local_190,uVar5);
                DAT_0019fdd0 = 2;
              }
            }
            iVar1 = FUN_0010b768(param_1,local_188,0x2c,0);
            local_1b8 = local_1b8 + (long)iVar1;
            if (local_190 != (char *)0x0) {
              FUN_00146399(&local_128);
            }
            break;
          default:
            goto switchD_0010ff8a_caseD_55;
          case 99:
            iVar1 = format_checkpoint_string(param_1,local_1b8,PTR_s____Y__m__d__H__M__S_t___ds____re_0019c158,
                                 param_4,param_5);
            local_1b8 = local_1b8 + (long)iVar1;
            break;
          case 100:
            uVar5 = FUN_0010ae4d();
            iVar1 = fprintf(param_1,"%.0f",uVar5);
            local_1b8 = local_1b8 + (long)iVar1;
            break;
          case 0x73:
            fputs_unlocked(local_180,param_1);
            sVar2 = strlen(local_180);
            local_1b8 = local_1b8 + sVar2;
            break;
          case 0x74:
            local_170 = local_190;
            if (local_190 == (char *)0x0) {
              local_170 = "%c";
            }
            gettimeofday(&local_158,(__timezone_ptr_t)0x0);
            local_168 = localtime(&local_158.tv_sec);
            lVar4 = FUN_0014f679(param_1,local_170,local_168,0,(int)local_158.tv_usec * 1000);
            local_1b8 = local_1b8 + lVar4;
            break;
          case 0x75:
            fputs_unlocked(local_178,param_1);
            sVar2 = strlen(local_178);
            local_1b8 = local_1b8 + sVar2;
          }
        }
LAB_00110328:
        local_190 = (char *)0x0;
      }
    }
    else {
      fputc_unlocked((int)*local_198,param_1);
      if (*local_198 == '\r') {
        local_1b8 = 0;
        DAT_0019f300 = 1;
      }
      else {
        local_1b8 = local_1b8 + 1;
      }
    }
    local_198 = local_198 + 1;
  } while( true );
}




// Function: run_checkpoint_actions @ 0x103d4

void run_checkpoint_actions(undefined1 param_1)

{
  long in_FS_OFFSET;
  int local_20;
  int local_1c;
  undefined8 *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  for (local_18 = DAT_0019f268; local_18 != (undefined8 *)0x0; local_18 = (undefined8 *)*local_18) {
    switch(*(undefined4 *)(local_18 + 1)) {
    case 0:
      fputc_unlocked(0x2e,DAT_0019efd0);
      fflush_unlocked(DAT_0019efd0);
      break;
    case 1:
      if (DAT_0019f308 == (FILE *)0x0) {
        DAT_0019f308 = fopen("/dev/tty","w");
      }
      if (DAT_0019f308 != (FILE *)0x0) {
        fputc_unlocked(7,DAT_0019f308);
        fflush_unlocked(DAT_0019f308);
      }
      break;
    case 2:
      local_1c = fprintf(stderr,"%s: ",DAT_0019fe48);
      FUN_0010fd96(stderr,(long)local_1c,local_18[2],param_1,DAT_0019f260);
      fputc_unlocked(10,stderr);
      break;
    case 3:
      if (DAT_0019f308 == (FILE *)0x0) {
        DAT_0019f308 = fopen("/dev/tty","w");
      }
      if (DAT_0019f308 != (FILE *)0x0) {
        FUN_0010fd96(DAT_0019f308,0,local_18[2],param_1,DAT_0019f260);
      }
      break;
    case 4:
      sleep((uint)local_18[2]);
      break;
    case 5:
      FUN_001348d7(local_18[2],*DAT_0019fbe8,DAT_0019f260);
      break;
    case 6:
      FUN_0010ae4d();
      FUN_0010baa2();
      break;
    case 7:
      sigwait((sigset_t *)&DAT_0019f280,&local_20);
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: checkpoint_flush_actions @ 0x10622

void checkpoint_flush_actions(void)

{
  undefined8 *local_18;
  long local_10;
  
  for (local_18 = DAT_0019f268; local_18 != (undefined8 *)0x0; local_18 = (undefined8 *)*local_18) {
    if (((*(int *)(local_18 + 1) == 3) && (DAT_0019f308 != (FILE *)0x0)) && (DAT_0019f300 != 0)) {
      local_10 = FUN_0010fbee(DAT_0019f308);
      while (local_10 != 0) {
        fputc_unlocked(0x20,DAT_0019f308);
        local_10 = local_10 + -1;
      }
      fputc_unlocked(0xd,DAT_0019f308);
      fflush_unlocked(DAT_0019f308);
    }
  }
  return;
}




// Function: checkpoint_run @ 0x106d8

void checkpoint_run(undefined1 param_1)

{
  if (DAT_0019f890 != 0) {
    DAT_0019f260 = DAT_0019f260 + 1;
    if (DAT_0019f260 % DAT_0019f890 == 0) {
      FUN_001103d4(param_1);
    }
  }
  return;
}




// Function: checkpoint_finish @ 0x10729

void checkpoint_finish(void)

{
  if (DAT_0019f890 != 0) {
    FUN_00110622();
    if (DAT_0019f308 != (FILE *)0x0) {
      fclose(DAT_0019f308);
    }
  }
  return;
}




// Function: diff_init @ 0x1075e

void diff_init(void)

{
  long in_FS_OFFSET;
  undefined1 local_18 [8];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_0019f328 = FUN_0012b601(local_18,DAT_0019f870);
  if (DAT_0019f8e0 != 0) {
    FUN_00124090();
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: report_difference @ 0x107be

void report_difference(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8,
                 long param_9,char *param_10,undefined8 param_11,undefined8 param_12,
                 undefined8 param_13,undefined8 param_14)

{
  char in_AL;
  undefined8 uVar1;
  long in_FS_OFFSET;
  undefined4 local_d8;
  undefined4 local_d4;
  undefined1 *local_d0;
  undefined1 *local_c8;
  long local_c0;
  undefined1 local_b8 [16];
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
  local_a8 = param_11;
  local_a0 = param_12;
  local_98 = param_13;
  local_90 = param_14;
  if (param_10 != (char *)0x0) {
    uVar1 = FUN_00129516(1,*(undefined8 *)(param_9 + 8));
    fprintf(DAT_0019efd0,"%s: ",uVar1);
    local_d8 = 0x10;
    local_d4 = 0x30;
    local_d0 = &stack0x00000008;
    local_c8 = local_b8;
    vfprintf(DAT_0019efd0,param_10,&local_d8);
    fprintf(DAT_0019efd0,"\n");
  }
  FUN_00139e98(1);
  if (local_c0 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: process_noop @ 0x10903

undefined8 process_noop(void)

{
  return 1;
}




// Function: process_rawdata @ 0x1091a

undefined8 process_rawdata(size_t param_1,void *param_2)

{
  int iVar1;
  size_t sVar2;
  undefined8 uVar3;
  
  sVar2 = FUN_0012a9e9(DAT_0019f324,DAT_0019f328,param_1);
  if (sVar2 == param_1) {
    iVar1 = memcmp(param_2,DAT_0019f328,param_1);
    if (iVar1 == 0) {
      uVar3 = 1;
    }
    else {
      uVar3 = gettext("Contents differ");
      FUN_001107be(&DAT_0019fa20,uVar3);
      uVar3 = 0;
    }
  }
  else {
    if (sVar2 == 0xffffffffffffffff) {
      FUN_0013e403(DAT_0019fa28);
      FUN_001107be(&DAT_0019fa20,0);
    }
    else {
      uVar3 = ngettext("Could only read %lu of %lu byte","Could only read %lu of %lu bytes",param_1)
      ;
      FUN_001107be(&DAT_0019fa20,uVar3,sVar2,param_1);
    }
    uVar3 = 0;
  }
  return uVar3;
}




// Function: read_and_process @ 0x10a26

void read_and_process(long param_1,code *param_2)

{
  int iVar1;
  long lVar2;
  undefined8 uVar3;
  code *local_38;
  ulong local_20;
  ulong local_18;
  
  local_18 = *(ulong *)(param_1 + 0x88);
  FUN_0010ad2b(param_1);
  local_38 = param_2;
  while( true ) {
    if (local_18 == 0) {
      FUN_0010ad66();
      return;
    }
    lVar2 = FUN_0010bb4b();
    if (lVar2 == 0) break;
    local_20 = FUN_0010bbf5(lVar2);
    if (local_18 < local_20) {
      local_20 = local_18;
    }
    iVar1 = (*local_38)(local_20,lVar2);
    if (iVar1 == 0) {
      local_38 = FUN_00110903;
    }
    FUN_0010bba7(lVar2 + (local_20 - 1));
    local_18 = local_18 - local_20;
    FUN_0010ad86(local_18);
  }
  if (DAT_0019fdc8 != (code *)0x0) {
    (*DAT_0019fdc8)();
  }
  uVar3 = gettext("Unexpected EOF in archive");
  error(0,0,uVar3);
  DAT_0019fdd0 = 2;
  return;
}




// Function: get_stat_data @ 0x10b3a

undefined8 get_stat_data(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  int *piVar2;
  undefined8 uVar3;
  
  iVar1 = FUN_0012a9b8(param_1,param_2);
  if (iVar1 == 0) {
    uVar3 = 1;
  }
  else {
    piVar2 = __errno_location();
    if (*piVar2 == 2) {
      FUN_0013ea2c(param_1);
    }
    else {
      FUN_0013ea03(param_1);
    }
    FUN_001107be(&DAT_0019fa20,0);
    uVar3 = 0;
  }
  return uVar3;
}




// Function: diff_dir @ 0x10bb7

void diff_dir(void)

{
  int iVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined1 local_a8 [24];
  uint local_90;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_00110b3a(DAT_0019fa28,local_a8);
  if (iVar1 != 0) {
    if ((local_90 & 0xf000) == 0x4000) {
      if (((local_90 ^ DAT_0019fa90) & 0xfff) != 0) {
        uVar2 = gettext("Mode differs");
        FUN_001107be(&DAT_0019fa20,uVar2);
      }
    }
    else {
      uVar2 = gettext("File type differs");
      FUN_001107be(&DAT_0019fa20,uVar2);
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: diff_file @ 0x10c82

void diff_file(void)

{
  char *__file;
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined1 auVar4 [16];
  undefined1 local_b8 [24];
  uint local_a0;
  long local_88;
  long local_20;
  
  __file = DAT_0019fa28;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = FUN_00110b3a(DAT_0019fa28,local_b8);
  if (iVar2 == 0) {
    FUN_00128891();
  }
  else if ((local_a0 & 0xf000) == 0x8000) {
    if (((local_a0 ^ DAT_0019fa90) & 0xfff) != 0) {
      uVar3 = gettext("Mode differs");
      FUN_001107be(&DAT_0019fa20,uVar3);
    }
    cVar1 = FUN_00132ebf(local_b8,&DAT_0019fa78);
    if (cVar1 != '\x01') {
      uVar3 = gettext("Uid differs");
      FUN_001107be(&DAT_0019fa20,uVar3);
    }
    cVar1 = FUN_00132ee4(local_b8,&DAT_0019fa78);
    if (cVar1 != '\x01') {
      uVar3 = gettext("Gid differs");
      FUN_001107be(&DAT_0019fa20,uVar3);
    }
    auVar4 = FUN_0015fd9e(local_b8);
    iVar2 = FUN_00139e35(auVar4._0_8_,auVar4._8_8_,DAT_0019fb18,DAT_0019fb20);
    if (iVar2 != 0) {
      uVar3 = gettext("Mod time differs");
      FUN_001107be(&DAT_0019fa20,uVar3);
    }
    if ((*(char *)(DAT_0019f460 + 0x9c) == 'S') || (local_88 == DAT_0019faa8)) {
      DAT_0019f324 = openat(DAT_0019c268,__file,DAT_0019fc08);
      if (DAT_0019f324 < 0) {
        FUN_0013e38b(__file);
        FUN_00128891();
        FUN_001107be(&DAT_0019fa20,0);
      }
      else {
        if (DAT_0019fb40 == '\0') {
          FUN_00110a26(&DAT_0019fa20,FUN_0011091a);
        }
        else {
          FUN_00130972(DAT_0019f324,&DAT_0019fa20);
        }
        if ((DAT_0019f880 == 1) && (local_88 != 0)) {
          auVar4 = FUN_0015fd6a(local_b8);
          iVar2 = FUN_0012ab5e(DAT_0019f324,DAT_0019c268,__file,auVar4._0_8_,auVar4._8_8_);
          if (iVar2 != 0) {
            FUN_0013ead0(__file);
          }
        }
        iVar2 = close(DAT_0019f324);
        if (iVar2 != 0) {
          FUN_0013e1ff(__file);
        }
      }
    }
    else {
      uVar3 = gettext("Size differs");
      FUN_001107be(&DAT_0019fa20,uVar3);
      FUN_00128891();
    }
  }
  else {
    uVar3 = gettext("File type differs");
    FUN_001107be(&DAT_0019fa20,uVar3);
    FUN_00128891();
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: diff_link @ 0x10fea

void diff_link(void)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  undefined1 local_148 [144];
  undefined1 local_b8 [152];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = FUN_00110b3a(DAT_0019fa28,local_148);
  if (iVar2 != 0) {
    iVar2 = FUN_00110b3a(DAT_0019fa38,local_b8);
    if (iVar2 != 0) {
      cVar1 = FUN_00132f09(local_148,local_b8);
      if (cVar1 != '\x01') {
        uVar3 = FUN_00129516(0,DAT_0019fa38);
        uVar4 = gettext("Not linked to %s");
        FUN_001107be(&DAT_0019fa20,uVar4,uVar3);
      }
    }
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: diff_symlink @ 0x110be

void diff_symlink(void)

{
  int iVar1;
  size_t __n;
  char *__buf;
  size_t sVar2;
  int *piVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  char local_418 [1032];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  __n = strlen(DAT_0019fa38);
  if (__n < 0x400) {
    __buf = local_418;
  }
  else {
    __buf = (char *)FUN_00160b5b(__n + 1);
  }
  sVar2 = readlinkat(DAT_0019c268,DAT_0019fa28,__buf,__n + 1);
  if ((long)sVar2 < 0) {
    piVar3 = __errno_location();
    if (*piVar3 == 2) {
      FUN_0013e6dc(DAT_0019fa28);
    }
    else {
      FUN_0013e6b3(DAT_0019fa28);
    }
    FUN_001107be(&DAT_0019fa20,0);
  }
  else {
    if (sVar2 == __n) {
      iVar1 = memcmp(DAT_0019fa38,__buf,__n);
      if (iVar1 == 0) goto LAB_001111fa;
    }
    uVar4 = gettext("Symlink differs");
    FUN_001107be(&DAT_0019fa20,uVar4);
  }
LAB_001111fa:
  if (__buf != local_418) {
    free(__buf);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: diff_special @ 0x11230

void diff_special(void)

{
  int iVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  bool bVar3;
  undefined1 local_a8 [24];
  uint local_90;
  long local_80;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_00110b3a(DAT_0019fa28,local_a8);
  if (iVar1 != 0) {
    if (*(char *)(DAT_0019f460 + 0x9c) == '3') {
      bVar3 = (local_90 & 0xf000) != 0x2000;
    }
    else if (*(char *)(DAT_0019f460 + 0x9c) == '4') {
      bVar3 = (local_90 & 0xf000) != 0x6000;
    }
    else {
      bVar3 = (local_90 & 0xf000) != 0x1000;
    }
    if (bVar3) {
      uVar2 = gettext("File type differs");
      FUN_001107be(&DAT_0019fa20,uVar2);
    }
    else if (((*(char *)(DAT_0019f460 + 0x9c) == '3') || (*(char *)(DAT_0019f460 + 0x9c) == '4')) &&
            (DAT_0019faa0 != local_80)) {
      uVar2 = gettext("Device number differs");
      FUN_001107be(&DAT_0019fa20,uVar2);
    }
    else if (((local_90 ^ DAT_0019fa90) & 0xfff) != 0) {
      uVar2 = gettext("Mode differs");
      FUN_001107be(&DAT_0019fa20,uVar2);
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: dumpdir_cmp @ 0x113b1

int dumpdir_cmp(char *param_1,char *param_2)

{
  int iVar1;
  char *pcVar2;
  size_t sVar3;
  ulong uVar4;
  char *local_28;
  char *local_20;
  
  local_28 = param_2;
  local_20 = param_1;
  while( true ) {
    while( true ) {
      do {
        if (*local_20 == '\0') {
          return (int)*local_28;
        }
      } while (0x15 < (int)*local_20 - 0x44U);
      uVar4 = 1L << ((byte)((int)*local_20 - 0x44U) & 0x3f);
      if ((uVar4 & 0x114000) != 0) {
        return (int)*local_28;
      }
      if ((uVar4 & 0x200400) != 0) break;
      if ((uVar4 & 1) != 0) {
        iVar1 = strcmp(local_20,local_28);
        if (iVar1 != 0) {
          return 1;
        }
        sVar3 = strlen(local_20);
        local_20 = local_20 + sVar3 + 1;
        local_28 = local_28 + sVar3 + 1;
      }
    }
    pcVar2 = strchr("YN",(int)*local_28);
    if (pcVar2 == (char *)0x0) {
      return 1;
    }
    iVar1 = strcmp(local_20 + 1,local_28 + 1);
    if (iVar1 != 0) break;
    sVar3 = strlen(local_20);
    local_20 = local_20 + sVar3 + 1;
    local_28 = local_28 + sVar3 + 1;
  }
  return 1;
}




// Function: diff_dumpdir @ 0x11512

void diff_dumpdir(undefined8 *param_1)

{
  int iVar1;
  int iVar2;
  undefined8 uVar3;
  long lVar4;
  code *local_18;
  
  if (*(int *)(param_1 + 0x35) == 0) {
    local_18 = (code *)0x0;
    iVar1 = FUN_001157f9(param_1[0x33],*param_1,DAT_0019fc08);
    if (iVar1 < 0) {
      local_18 = FUN_0012b2b1;
    }
    else {
      iVar2 = fstat(iVar1,(stat *)(param_1 + 0xb));
      if (iVar2 == 0) {
        *(int *)(param_1 + 0x35) = iVar1;
      }
      else {
        local_18 = FUN_0012b444;
        close(iVar1);
      }
    }
    if (local_18 != (code *)0x0) {
      FUN_0012b48b(*param_1,0,local_18);
      return;
    }
  }
  uVar3 = FUN_00121eb9(param_1);
  lVar4 = FUN_0012241b(uVar3);
  if (lVar4 == 0) {
    FUN_00110a26(param_1,FUN_00110903);
  }
  else {
    iVar1 = FUN_001113b1(param_1[0x32],lVar4);
    if (iVar1 != 0) {
      uVar3 = gettext("Contents differ");
      FUN_001107be(param_1,uVar3);
    }
  }
  return;
}




// Function: diff_multivol @ 0x11650

void diff_multivol(void)

{
  int iVar1;
  undefined8 uVar2;
  long __offset;
  __off_t _Var3;
  long in_FS_OFFSET;
  undefined1 local_a8 [24];
  uint local_90;
  long local_78;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0019fa30 == '\0') {
    iVar1 = FUN_00110b3a(DAT_0019fa28,local_a8);
    if (iVar1 != 0) {
      if ((local_90 & 0xf000) == 0x8000) {
        __offset = FUN_0012798b(DAT_0019f460 + 0x171,0xc);
        if (((__offset < 0) || (SCARRY8(__offset,DAT_0019faa8))) ||
           (local_78 != __offset + DAT_0019faa8)) {
          uVar2 = gettext("Size differs");
          FUN_001107be(&DAT_0019fa20,uVar2);
          FUN_00128891();
        }
        else {
          iVar1 = openat(DAT_0019c268,DAT_0019fa28,DAT_0019fc08);
          if (iVar1 < 0) {
            FUN_0013e38b(DAT_0019fa28);
            FUN_001107be(&DAT_0019fa20,0);
            FUN_00128891();
          }
          else {
            _Var3 = lseek(iVar1,__offset,0);
            if (_Var3 < 0) {
              FUN_0013e7a9(DAT_0019fa28,__offset);
              FUN_001107be(&DAT_0019fa20,0);
            }
            else {
              FUN_00110a26(&DAT_0019fa20,FUN_0011091a);
            }
            iVar1 = close(iVar1);
            if (iVar1 != 0) {
              FUN_0013e1ff(DAT_0019fa28);
            }
          }
        }
      }
      else {
        uVar2 = gettext("File type differs");
        FUN_001107be(&DAT_0019fa20,uVar2);
        FUN_00128891();
      }
    }
  }
  else {
    FUN_00110bb7();
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: diff_archive @ 0x11897

void diff_archive(void)

{
  char cVar1;
  char *__format;
  undefined8 uVar2;
  undefined8 uVar3;
  
  FUN_0010bba7(DAT_0019f460);
  if (DAT_0019f9ac != 0) {
    if (DAT_0019f320 != '\0') {
      __format = (char *)gettext("Verify ");
      fprintf(DAT_0019efd0,__format);
    }
    FUN_0012864c(&DAT_0019fa20,DAT_0019f460,0xffffffffffffffff);
  }
  switch(*(undefined1 *)(DAT_0019f460 + 0x9c)) {
  default:
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    cVar1 = *(char *)(DAT_0019f460 + 0x9c);
    uVar2 = FUN_0015e548(DAT_0019fa28);
    uVar3 = gettext("%s: Unknown file type \'%c\', diffed as normal file");
    error(0,0,uVar3,uVar2,(int)cVar1);
    DAT_0019fdd0 = 2;
  case 0:
  case 0x30:
  case 0x37:
  case 0x53:
    if (DAT_0019fa30 == '\0') {
      FUN_00110c82();
    }
    else {
      FUN_00110bb7();
    }
    break;
  case 0x31:
    FUN_00110fea();
    break;
  case 0x32:
    FUN_001110be();
    break;
  case 0x33:
  case 0x34:
  case 0x36:
    FUN_00111230();
    break;
  case 0x35:
  case 0x44:
    cVar1 = FUN_0012490b(&DAT_0019fa20);
    if (cVar1 != '\0') {
      FUN_00111512(&DAT_0019fa20);
    }
    FUN_00110bb7();
    break;
  case 0x4d:
    FUN_00111650();
    break;
  case 0x56:
    break;
  }
  return;
}




// Function: verify_volume @ 0x11a1c

void verify_volume(void)

{
  bool bVar1;
  char cVar2;
  int iVar3;
  undefined8 uVar4;
  int *piVar5;
  long lVar6;
  undefined8 uVar7;
  long in_FS_OFFSET;
  int local_4c;
  int local_48;
  undefined2 local_40 [2];
  undefined4 local_3c;
  undefined1 local_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  bVar1 = false;
  cVar2 = FUN_0013ed80();
  if (cVar2 != '\0') {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar4 = gettext("Archive contains file names with leading prefixes removed.");
    error(0,0,uVar4);
    bVar1 = true;
  }
  cVar2 = FUN_0013ba20();
  if (cVar2 != '\0') {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar4 = gettext("Archive contains transformed file names.");
    error(0,0,uVar4);
    bVar1 = true;
  }
  if (bVar1) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar4 = gettext("Verification may fail to locate original files.");
    error(0,0,uVar4);
  }
  FUN_00120f88();
  if (DAT_0019f328 == 0) {
    FUN_0011075e();
  }
  fsync(DAT_0019f9cc);
  ioctl(DAT_0019f9cc,0x24b);
  local_40[0] = 2;
  local_3c = 1;
  if (DAT_0019f9cc < 0x40000000) {
    iVar3 = ioctl(DAT_0019f9cc,0x40086d01,local_40);
  }
  else {
    iVar3 = FUN_00140254(DAT_0019f9cc + -0x40000000,0x40086d01,local_40);
  }
  if (iVar3 < 0) {
    piVar5 = __errno_location();
    if (*piVar5 == 5) {
      if (DAT_0019f9cc < 0x40000000) {
        iVar3 = ioctl(DAT_0019f9cc,0x40086d01,local_40);
      }
      else {
        iVar3 = FUN_00140254(DAT_0019f9cc + -0x40000000,0x40086d01,local_40);
      }
      if (-1 < iVar3) goto LAB_00111c88;
    }
    if (DAT_0019f9cc < 0x40000000) {
      lVar6 = lseek(DAT_0019f9cc,0,0);
    }
    else {
      lVar6 = FUN_001400be(DAT_0019f9cc + -0x40000000,0,0);
    }
    if (lVar6 != 0) {
      FUN_0013e867(*DAT_0019fbd8);
      goto LAB_00111e87;
    }
  }
LAB_00111c88:
  DAT_0019efb8 = 0;
  DAT_0019f320 = 1;
  FUN_0010f662();
  while( true ) {
    local_4c = FUN_00125ecf(&DAT_0019f460,&DAT_0019fa20,0);
    if (local_4c == 5) {
      local_48 = 0;
      do {
        local_48 = local_48 + 1;
        FUN_0010bba7(DAT_0019f460);
        local_4c = FUN_00125ecf(&DAT_0019f460,&DAT_0019fa20,0);
      } while (local_4c == 5);
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar4 = ngettext("VERIFY FAILURE: %d invalid header detected",
                       "VERIFY FAILURE: %d invalid headers detected",(long)local_48);
      error(0,0,uVar4,local_48);
      DAT_0019fdd0 = 2;
    }
    if (local_4c == 4) break;
    if (local_4c == 3) {
      FUN_0010bba7(DAT_0019f460);
      if (DAT_0019f8bd != '\x01') {
        iVar3 = FUN_00125ecf(&DAT_0019f460,&DAT_0019fa20,0);
        if (iVar3 == 3) break;
        if ((DAT_0019ec84 & 1) != 0) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar4 = FUN_0010bacd();
          uVar4 = FUN_00154e5a(uVar4,local_38);
          uVar7 = gettext("A lone zero block at %s");
          error(0,0,uVar7,uVar4);
        }
      }
    }
    else {
      FUN_00126747(DAT_0019f460,&DAT_0019fa20,&DAT_0019f468,1);
      FUN_00111897();
      FUN_00139d1c(&DAT_0019fa20);
    }
  }
  DAT_0019efb8 = 1;
  DAT_0019f320 = 0;
LAB_00111e87:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: add_exclusion_tag @ 0x11ea1

void add_exclusion_tag(char *param_1,undefined4 param_2,undefined8 param_3)

{
  undefined8 *puVar1;
  size_t sVar2;
  
  puVar1 = (undefined8 *)FUN_00160b5b(0x28);
  puVar1[4] = DAT_0019f330;
  *puVar1 = param_1;
  *(undefined4 *)(puVar1 + 2) = param_2;
  puVar1[3] = param_3;
  sVar2 = strlen(param_1);
  puVar1[1] = sVar2;
  DAT_0019f330 = puVar1;
  return;
}




// Function: exclusion_tag_warning @ 0x11f18

void exclusion_tag_warning(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  
  if ((DAT_0019f9ac != 0) && ((DAT_0019ec84 & 4) != 0)) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar1 = FUN_0015e285(1,param_2);
    uVar2 = FUN_0015e548(param_1);
    uVar3 = gettext("%s: contains a cache directory tag %s; %s");
    error(0,0,uVar3,uVar2,uVar1,param_3);
  }
  return;
}




// Function: check_exclusion_tags @ 0x11fbf

undefined4 check_exclusion_tags(undefined8 param_1,undefined8 *param_2)

{
  char cVar1;
  bool bVar2;
  int __fd;
  undefined8 *local_10;
  
  local_10 = DAT_0019f330;
  do {
    if (local_10 == (undefined8 *)0x0) {
      return 0;
    }
    __fd = FUN_001157f9(param_1,*local_10,DAT_0019fc08);
    if (-1 < __fd) {
      if ((local_10[3] == 0) || (cVar1 = (*(code *)local_10[3])(__fd), cVar1 != '\0')) {
        bVar2 = true;
      }
      else {
        bVar2 = false;
      }
      close(__fd);
      if (bVar2) {
        if (param_2 != (undefined8 *)0x0) {
          *param_2 = *local_10;
        }
        return *(undefined4 *)(local_10 + 2);
      }
    }
    local_10 = (undefined8 *)local_10[4];
  } while( true );
}




// Function: cachedir_file_p @ 0x12087

undefined4 cachedir_file_p(int param_1)

{
  int iVar1;
  undefined4 uVar2;
  ssize_t sVar3;
  long in_FS_OFFSET;
  undefined1 local_48 [56];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  sVar3 = read(param_1,local_48,0x2b);
  if (sVar3 == 0x2b) {
    iVar1 = memcmp(local_48,"Signature: 8a477f597d28d172789f06886806bc55",0x2b);
    if (iVar1 == 0) {
      uVar2 = 1;
      goto LAB_001120ec;
    }
  }
  uVar2 = 0;
LAB_001120ec:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: to_octal @ 0x12105

void to_octal(ulong param_1,long param_2,long param_3)

{
  undefined8 local_18;
  undefined8 local_10;
  
  local_18 = param_1;
  local_10 = param_3;
  do {
    local_10 = local_10 + -1;
    *(byte *)(local_10 + param_2) = ((byte)local_18 & 7) + 0x30;
    local_18 = local_18 >> 3;
  } while (local_10 != 0);
  return;
}




// Function: tar_copy_str @ 0x12157

void tar_copy_str(long param_1,long param_2,ulong param_3)

{
  ulong local_10;
  
  for (local_10 = 0;
      (local_10 < param_3 &&
      (*(char *)(local_10 + param_1) = *(char *)(param_2 + local_10),
      *(char *)(local_10 + param_1) != '\0')); local_10 = local_10 + 1) {
  }
  return;
}




// Function: tar_name_copy_str @ 0x121ac

void tar_name_copy_str(long param_1,undefined8 param_2,long param_3)

{
  FUN_00112157(param_1,param_2,param_3);
  if (DAT_0019f864 == 2) {
    *(undefined1 *)(param_1 + param_3 + -1) = 0;
  }
  return;
}




// Function: to_base256 @ 0x121fb

void to_base256(int param_1,ulong param_2,long param_3,long param_4)

{
  undefined8 local_20;
  undefined8 local_18;
  
  local_20 = param_2;
  local_18 = param_4;
  do {
    local_18 = local_18 + -1;
    *(char *)(local_18 + param_3) = (char)local_20;
    local_20 = local_20 >> 8 | (long)-param_1 << 0x38;
  } while (local_18 != 0);
  return;
}




// Function: to_chars_subst @ 0x12262

undefined8
to_chars_subst(int param_1,int param_2,long param_3,undefined8 param_4,code *param_5,
            undefined8 param_6,long param_7,undefined8 param_8)

{
  ulong uVar1;
  long lVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  uint local_dc;
  undefined *local_d8;
  undefined1 *local_d0;
  undefined1 *local_c8;
  ulong local_c0;
  undefined8 local_b8;
  long local_b0;
  undefined *local_a8;
  undefined1 *local_a0;
  ulong local_98;
  ulong local_90;
  undefined1 local_88 [32];
  undefined1 local_68;
  undefined1 auStack_67 [32];
  undefined1 auStack_47 [32];
  undefined1 auStack_27 [23];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 == 0) {
    if (param_7 * 3 - 3U < 0x40) {
      local_c0 = (1L << ((char)param_7 * '\x03' - 3U & 0x3f)) - 1;
    }
    else {
      local_c0 = 0xffffffffffffffff;
    }
  }
  else if ((ulong)((param_7 + -1) * 8) < 0x40) {
    local_c0 = (1L << (((char)param_7 + -1) * '\b' & 0x3fU)) - 1;
  }
  else {
    local_c0 = 0xffffffffffffffff;
  }
  local_b8 = FUN_00154e5a(local_c0,local_88);
  if (param_2 == 0) {
    local_d8 = &DAT_00180d84;
  }
  else {
    uVar1 = local_c0;
    if (local_c0 == 0xffffffffffffffff) {
      uVar1 = 0x7fffffffffffffff;
    }
    local_b0 = uVar1 + 1;
    lVar2 = FUN_00154e5a(local_b0,auStack_47);
    local_d8 = (undefined *)(lVar2 + -1);
    *local_d8 = 0x2d;
    local_a8 = local_d8;
  }
  if (param_1 == 0) {
    local_d0 = (undefined1 *)FUN_00154e5a(param_3,&local_68);
  }
  else {
    lVar2 = FUN_00154e5a(-param_3,auStack_67);
    local_d0 = (undefined1 *)(lVar2 + -1);
    *local_d0 = 0x2d;
    local_a0 = local_d0;
  }
  if (param_5 == (code *)0x0) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar3 = gettext("value %s out of %s range %s..%s");
    error(0,0,uVar3,local_d0,param_8,local_d8,local_b8);
    DAT_0019fdd0 = 2;
    uVar3 = 0;
  }
  else {
    local_98 = (*param_5)(&local_dc);
    local_98 = local_98 & local_c0;
    local_dc = local_dc & DAT_0019f864 == 6;
    local_90 = local_98;
    if (local_dc != 0) {
      local_90 = -local_98;
    }
    local_c8 = (undefined1 *)FUN_00154e5a(local_90,auStack_27);
    if (local_dc != 0) {
      local_c8 = local_c8 + -1;
      *local_c8 = 0x2d;
    }
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar3 = gettext("value %s out of %s range %s..%s; substituting %s");
    error(0,0,uVar3,local_d0,param_8,local_d8,local_b8,local_c8);
    uVar3 = FUN_0011262b(local_dc,local_90,param_4,0,param_6,param_7,param_8);
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: to_chars @ 0x1262b

undefined8
to_chars(int param_1,ulong param_2,long param_3,undefined8 param_4,undefined1 *param_5,
            long param_6,undefined8 param_7)

{
  int iVar1;
  ulong uVar2;
  undefined8 uVar3;
  undefined1 uVar4;
  ulong uVar5;
  undefined8 local_38;
  
  if ((DAT_0019f864 == 6) || (DAT_0019f864 == 2)) {
    iVar1 = 1;
  }
  else {
    iVar1 = 0;
  }
  if (param_1 == 0) {
    if (param_6 * 3 - 3U < 0x40) {
      uVar2 = (1L << ((char)param_6 * '\x03' - 3U & 0x3f)) - 1;
    }
    else {
      uVar2 = 0xffffffffffffffff;
    }
    if (param_2 <= uVar2) {
      param_5[param_6 + -1] = 0;
      FUN_00112105(param_2,param_5,param_6 + -1);
      return 1;
    }
  }
  if (iVar1 == 0) {
    local_38 = 0;
  }
  else {
    uVar2 = param_2;
    if (param_1 != 0) {
      uVar2 = ~param_2;
    }
    if ((ulong)((param_6 + -1) * 8) < 0x40) {
      uVar5 = (1L << (((char)param_6 + -1) * '\b' & 0x3fU)) - 1;
    }
    else {
      uVar5 = 0xffffffffffffffff;
    }
    if (uVar2 <= uVar5) {
      if (param_1 == 0) {
        uVar4 = 0x80;
      }
      else {
        uVar4 = 0xff;
      }
      *param_5 = uVar4;
      FUN_001121fb(param_1,param_2,param_5 + 1,param_6 + -1);
      return 1;
    }
    local_38 = param_4;
    if ((param_1 != 0) &&
       (uVar2 = param_6 * 3 - 3, (ulong)(param_3 * 8) < uVar2 || param_3 * 8 - uVar2 == 0)) {
      if (DAT_0019f348 == 0) {
        DAT_0019f348 = 1;
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar3 = gettext("Generating negative octal headers");
        error(0,0,uVar3);
      }
      param_5[param_6 + -1] = 0;
      if ((ulong)(param_3 << 3) < 0x40) {
        uVar2 = (1L << ((byte)((int)param_3 << 3) & 0x3f)) - 1;
      }
      else {
        uVar2 = 0xffffffffffffffff;
      }
      FUN_00112105(uVar2 & param_2,param_5,param_6 + -1);
      return 1;
    }
  }
  uVar3 = FUN_00112262(param_1,iVar1,param_2,param_3,local_38,param_5,param_6,param_7);
  return uVar3;
}




// Function: gid_substitute @ 0x128a8

int gid_substitute(undefined4 *param_1)

{
  int iVar1;
  
  if (DAT_0019f34c == 0) {
    iVar1 = FUN_0012c261("nobody",&DAT_0019f34c);
    if (iVar1 == 0) {
      DAT_0019f34c = -2;
    }
  }
  iVar1 = DAT_0019f34c;
  *param_1 = 0;
  return iVar1;
}




// Function: gid_to_chars @ 0x12901

void gid_to_chars(undefined4 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011262b(0,param_1,4,FUN_001128a8,param_2,param_3,"gid_t");
  return;
}




// Function: major_to_chars @ 0x12957

void major_to_chars(uint param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011262b(param_1 >> 0x1f,(long)(int)param_1,4,0,param_2,param_3,"major_t");
  return;
}




// Function: minor_to_chars @ 0x129ae

void minor_to_chars(uint param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011262b(param_1 >> 0x1f,(long)(int)param_1,4,0,param_2,param_3,"minor_t");
  return;
}




// Function: mode_to_chars @ 0x12a05

void mode_to_chars(uint param_1,undefined8 param_2,undefined8 param_3)

{
  if (((DAT_0019f864 == 4) || (DAT_0019f864 == 3)) || (DAT_0019f864 == 6)) {
    param_1 = param_1 & 0xfff;
  }
  FUN_0011262b(0,param_1,4,0,param_2,param_3,"mode_t");
  return;
}




// Function: off_to_chars @ 0x12a9a

void off_to_chars(ulong param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011262b(param_1 >> 0x3f,param_1,8,0,param_2,param_3,"off_t");
  return;
}




// Function: time_to_chars @ 0x12af2

void time_to_chars(ulong param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011262b(param_1 >> 0x3f,param_1,8,0,param_2,param_3,"time_t");
  return;
}




// Function: uid_substitute @ 0x12b4a

int uid_substitute(undefined4 *param_1)

{
  int iVar1;
  
  if (DAT_0019f350 == 0) {
    iVar1 = FUN_0012c173("nobody",&DAT_0019f350);
    if (iVar1 == 0) {
      DAT_0019f350 = -2;
    }
  }
  iVar1 = DAT_0019f350;
  *param_1 = 0;
  return iVar1;
}




// Function: uid_to_chars @ 0x12ba3

void uid_to_chars(undefined4 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011262b(0,param_1,4,FUN_00112b4a,param_2,param_3,"uid_t");
  return;
}




// Function: uintmax_to_chars @ 0x12bf9

void uintmax_to_chars(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011262b(0,param_1,8,0,param_2,param_3,"uintmax_t");
  return;
}




// Function: string_to_chars @ 0x12c4c

void string_to_chars(undefined8 param_1,long param_2,long param_3)

{
  FUN_00112157(param_2,param_1,param_3);
  *(undefined1 *)(param_2 + param_3 + -1) = 0;
  return;
}




// Function: file_dumpable_p @ 0x12c90

undefined4 file_dumpable_p(long param_1)

{
  int iVar1;
  undefined4 uVar2;
  long lVar3;
  
  if ((*(uint *)(param_1 + 0x18) & 0xf000) == 0x4000) {
    uVar2 = 1;
  }
  else if ((*(uint *)(param_1 + 0x18) & 0xf000) == 0x8000) {
    if (DAT_0019f9d0 == '\0') {
      if ((*(long *)(param_1 + 0x30) == 0) && ((*(uint *)(param_1 + 0x18) & 0x124) == 0x124)) {
        uVar2 = 0;
      }
      else {
        uVar2 = 1;
      }
    }
    else {
      if ((DAT_0019f999 != '\0') && (DAT_0019f979 != '\0')) {
        lVar3 = *(long *)(param_1 + 0x30);
        if (lVar3 < 0) {
          lVar3 = lVar3 + 0x1ff;
        }
        if (((*(ulong *)(param_1 + 0x30) & 0x1ff) == 0) ||
           (*(long *)(param_1 + 0x30) + 0x1ffU < 0x3ff)) {
          iVar1 = 0;
        }
        else {
          iVar1 = 1;
        }
        if (*(long *)(param_1 + 0x40) < (long)iVar1 + (lVar3 >> 9)) {
          return 1;
        }
      }
      uVar2 = 0;
    }
  }
  else {
    uVar2 = 0;
  }
  return uVar2;
}




// Function: write_eot @ 0x12da0

void write_eot(void)

{
  void *pvVar1;
  size_t __n;
  
  pvVar1 = (void *)FUN_0010bb4b();
  memset(pvVar1,0,0x200);
  FUN_0010bba7(pvVar1);
  pvVar1 = (void *)FUN_0010bb4b();
  __n = FUN_0010bbf5(pvVar1);
  memset(pvVar1,0,__n);
  FUN_0010bba7(pvVar1);
  return;
}




// Function: start_private_header @ 0x12e0f

void * start_private_header(undefined8 param_1,undefined8 param_2,ulong param_3)

{
  void *__s;
  
  __s = (void *)FUN_0010bb4b();
  memset(__s,0,0x200);
  FUN_001121ac(__s,param_1,100);
  FUN_00112a9a(param_2,(long)__s + 0x7c,0xc);
  if ((long)param_3 < 0) {
    param_3 = 0;
  }
  else if (0x1ffffffff < param_3) {
    param_3 = 0x1ffffffff;
  }
  FUN_00112af2(param_3,(long)__s + 0x88,0xc);
  FUN_00112a05(0x81a4,(long)__s + 100,8);
  FUN_00112ba3(0,(long)__s + 0x6c,8);
  FUN_00112901(0,(long)__s + 0x74,8);
  memcpy((void *)((long)__s + 0x101),"ustar",6);
  memcpy((void *)((long)__s + 0x107),&DAT_00180e49,2);
  return __s;
}




// Function: write_short_name @ 0x12f4e

void * write_short_name(long param_1)

{
  void *__s;
  
  __s = (void *)FUN_0010bb4b();
  memset(__s,0,0x200);
  FUN_001121ac(__s,*(undefined8 *)(param_1 + 8),100);
  return __s;
}




// Function: write_gnu_long_link @ 0x12f9f

void write_gnu_long_link(undefined8 param_1,char *param_2,undefined1 param_3)

{
  size_t sVar1;
  long lVar2;
  char *local_38;
  ulong local_20;
  ulong local_18;
  void *local_10;
  
  sVar1 = strlen(param_2);
  local_20 = sVar1 + 1;
  lVar2 = FUN_00112e0f("././@LongLink",local_20,0);
  if (DAT_0019f934 != '\x01') {
    if (DAT_0019f358 == 0) {
      FUN_0012bfd1(0,&DAT_0019f358);
      FUN_0012c0a2(0,&DAT_0019f360);
    }
    FUN_00112c4c(DAT_0019f358,lVar2 + 0x109,0x20);
    FUN_00112c4c(DAT_0019f360,lVar2 + 0x129,0x20);
  }
  strcpy((char *)(lVar2 + 0x101),"ustar  ");
  *(undefined1 *)(lVar2 + 0x9c) = param_3;
  FUN_0011422e(param_1,lVar2,0xffffffffffffffff);
  local_10 = (void *)FUN_0010bb4b();
  local_18 = FUN_0010bbf5(local_10);
  local_38 = param_2;
  while (local_18 < local_20) {
    memcpy(local_10,local_38,local_18);
    local_38 = local_38 + local_18;
    local_20 = local_20 - local_18;
    FUN_0010bba7((long)local_10 + (local_18 - 1 & 0xfffffffffffffe00));
    local_10 = (void *)FUN_0010bb4b();
    local_18 = FUN_0010bbf5(local_10);
  }
  memcpy(local_10,local_38,local_20);
  memset((void *)((long)local_10 + local_20),0,local_18 - local_20);
  FUN_0010bba7((long)local_10 + (local_20 - 1 & 0xfffffffffffffe00));
  return;
}




// Function: split_long_name @ 0x13197

ulong split_long_name(long param_1,ulong param_2)

{
  ulong local_28;
  ulong local_10;
  
  if (param_2 < 0x9d) {
    local_28 = param_2;
    if (*(char *)(param_1 + (param_2 - 1)) == '/') {
      local_28 = param_2 - 1;
    }
  }
  else {
    local_28 = 0x9c;
  }
  do {
    local_10 = local_28 - 1;
    if (local_10 == 0) {
      return 0;
    }
    local_28 = local_10;
  } while (*(char *)(local_10 + param_1) != '/');
  return local_10;
}




// Function: write_ustar_long_name @ 0x1320b

void * write_ustar_long_name(char *param_1)

{
  size_t sVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  void *__s;
  size_t __n;
  
  sVar1 = strlen(param_1);
  if (sVar1 < 0x101) {
    __n = FUN_00113197(param_1,sVar1);
    if (((__n == 0) || (100 < (sVar1 - __n) - 1)) || (sVar1 - __n == 1)) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar2 = FUN_0015e548(param_1);
      uVar3 = gettext("%s: file name is too long (cannot be split); not dumped");
      error(0,0,uVar3,uVar2);
      DAT_0019fdd0 = 2;
      __s = (void *)0x0;
    }
    else {
      __s = (void *)FUN_0010bb4b();
      memset(__s,0,0x200);
      memcpy((void *)((long)__s + 0x159),param_1,__n);
      memcpy(__s,param_1 + __n + 1,(sVar1 - __n) - 1);
    }
  }
  else {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar2 = FUN_0015e548(param_1);
    uVar3 = gettext("%s: file name is too long (max %d); not dumped");
    error(0,0,uVar3,uVar2,0x100);
    DAT_0019fdd0 = 2;
    __s = (void *)0x0;
  }
  return __s;
}




// Function: write_long_link @ 0x133a8

void write_long_link(long param_1)

{
  undefined8 uVar1;
  undefined8 uVar2;
  ulong uVar3;
  
  if (DAT_0019f864 < 7) {
    uVar3 = 1L << ((byte)DAT_0019f864 & 0x3f);
    if ((uVar3 & 0x2a) == 0) {
      if ((uVar3 & 0x44) == 0) {
        if ((uVar3 & 0x10) == 0) goto LAB_001134aa;
        FUN_0011e1d3("linkpath",param_1,0);
      }
      else {
        FUN_00112f9f(param_1,*(undefined8 *)(param_1 + 0x18),0x4b);
      }
    }
    else {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar1 = FUN_0015e548(*(undefined8 *)(param_1 + 0x18));
      uVar2 = gettext("%s: link name is too long; not dumped");
      error(0,0,uVar2,uVar1);
      DAT_0019fdd0 = 2;
    }
    return;
  }
LAB_001134aa:
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: write_long_name @ 0x134b6

undefined8 write_long_name(long param_1)

{
  size_t sVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  
  switch(DAT_0019f864) {
  default:
                    /* WARNING: Subroutine does not return */
    abort();
  case 1:
    sVar1 = strlen(*(char **)(param_1 + 8));
    if (99 < sVar1) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar3 = FUN_0015e548(*(undefined8 *)(param_1 + 8));
      uVar2 = gettext("%s: file name is too long (max %d); not dumped");
      error(0,0,uVar2,uVar3,99);
      DAT_0019fdd0 = 2;
      return 0;
    }
    break;
  case 2:
  case 6:
    FUN_00112f9f(param_1,*(undefined8 *)(param_1 + 8),0x4c);
    break;
  case 3:
  case 5:
    uVar3 = FUN_0011320b(*(undefined8 *)(param_1 + 8));
    return uVar3;
  case 4:
    FUN_0011e1d3(&DAT_00180f06,param_1,0);
  }
  uVar3 = FUN_00112f4e(param_1);
  return uVar3;
}




// Function: write_extended @ 0x135e3

void * write_extended(char param_1,long param_2,void *param_3)

{
  long in_FS_OFFSET;
  undefined1 local_234;
  void *local_230;
  undefined8 local_228;
  undefined1 local_218 [520];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if ((*(long *)(param_2 + 0x178) == 0) && (*(long *)(param_2 + 0x168) != 0)) {
    FUN_0011e672(param_2 + 0x168);
    memcpy(local_218,param_3,0x200);
    if (param_1 == '\0') {
      local_234 = 0x78;
      local_230 = (void *)FUN_0011d123(param_2);
      local_228 = DAT_0019f920;
      if (DAT_0019f910 == 0) {
        local_228 = *(undefined8 *)(param_2 + 0xb0);
      }
    }
    else {
      local_234 = 0x67;
      local_230 = (void *)FUN_0011d18d();
      local_228 = DAT_0019f9e0;
    }
    FUN_0011d280(local_234,local_230,local_228,param_2 + 0x168);
    free(local_230);
    param_3 = (void *)FUN_0010bb4b();
    memcpy(param_3,local_218,0x200);
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return param_3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: write_header_name @ 0x13775

void write_header_name(long param_1)

{
  char cVar1;
  size_t sVar2;
  ulong uVar3;
  
  if ((DAT_0019f864 == 4) && (cVar1 = FUN_0013c9b7(*(undefined8 *)(param_1 + 8)), cVar1 != '\x01'))
  {
    FUN_0011e1d3(&DAT_00180f06,param_1,0);
    FUN_00112f4e(param_1);
    return;
  }
  if (DAT_0019f864 == 2) {
    uVar3 = 99;
  }
  else {
    uVar3 = 100;
  }
  sVar2 = strlen(*(char **)(param_1 + 8));
  if (uVar3 < sVar2) {
    FUN_001134b6(param_1);
  }
  else {
    FUN_00112f4e(param_1);
  }
  return;
}




// Function: start_header @ 0x1381d

long start_header(long param_1)

{
  uint uVar1;
  char cVar2;
  uint uVar3;
  int iVar4;
  undefined8 uVar5;
  size_t sVar6;
  long lVar7;
  undefined1 uVar8;
  long in_FS_OFFSET;
  uint local_70;
  uint local_6c;
  uint local_68;
  uint local_64;
  long local_60;
  long local_58;
  ulong local_50;
  long local_48;
  long local_40;
  ulong local_38;
  long local_30;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_60 = 0;
  local_58 = 0;
  local_48 = FUN_00113775(param_1);
  if (local_48 == 0) {
    lVar7 = 0;
    goto LAB_00114172;
  }
  FUN_00129274(*(undefined4 *)(param_1 + 0x74),param_1 + 0x74,&local_60);
  FUN_001293ce(*(undefined4 *)(param_1 + 0x78),param_1 + 0x78,&local_58);
  if (DAT_0019f8f0 != 0) {
    uVar1 = *(uint *)(param_1 + 0x70);
    uVar3 = FUN_00155b51(*(undefined4 *)(param_1 + 0x70),
                         (*(uint *)(param_1 + 0x70) & 0xf000) == 0x4000,DAT_0019f8f8,DAT_0019f8f0,0)
    ;
    *(uint *)(param_1 + 0x70) = uVar1 & 0xfffff000 | uVar3;
  }
  if ((DAT_0019f864 == 1) || (DAT_0019f864 == 3)) {
    FUN_00112a05(*(uint *)(param_1 + 0x70) & 0xfff,local_48 + 100,8);
  }
  else {
    FUN_00112a05(*(undefined4 *)(param_1 + 0x70),local_48 + 100,8);
  }
  local_70 = *(uint *)(param_1 + 0x74);
  if ((DAT_0019f864 == 4) && (0x1fffff < local_70)) {
    FUN_0011e1d3(&DAT_00180f28,param_1,0);
    local_70 = 0;
  }
  cVar2 = FUN_00112ba3(local_70,local_48 + 0x6c,8);
  if (cVar2 != '\x01') {
    lVar7 = 0;
    goto LAB_00114172;
  }
  local_6c = *(uint *)(param_1 + 0x78);
  if ((DAT_0019f864 == 4) && (0x1fffff < local_6c)) {
    FUN_0011e1d3(&DAT_00180f2c,param_1,0);
    local_6c = 0;
  }
  cVar2 = FUN_00112901(local_6c,local_48 + 0x74,8);
  if (cVar2 != '\x01') {
    lVar7 = 0;
    goto LAB_00114172;
  }
  local_50 = *(ulong *)(param_1 + 0x88);
  if ((DAT_0019f864 == 4) && (0x1ffffffff < local_50)) {
    FUN_0011e1d3(&DAT_00180f30,param_1,0);
    local_50 = 0;
  }
  cVar2 = FUN_00112a9a(local_50,local_48 + 0x7c,0xc);
  if (cVar2 != '\x01') {
    lVar7 = 0;
    goto LAB_00114172;
  }
  if (DAT_0019f910 == 2) {
    iVar4 = FUN_0015ff90(*(undefined8 *)(param_1 + 0xf8),*(undefined8 *)(param_1 + 0x100),
                         DAT_0019f920,DAT_0019f928);
    if (iVar4 < 1) {
      local_30 = *(long *)(param_1 + 0x100);
      local_38 = *(ulong *)(param_1 + 0xf8);
    }
    else {
      local_38 = DAT_0019f920;
      local_30 = DAT_0019f928;
    }
  }
  else if (DAT_0019f910 < 3) {
    if (DAT_0019f910 == 0) {
      local_30 = *(long *)(param_1 + 0x100);
      local_38 = *(ulong *)(param_1 + 0xf8);
    }
    else if (DAT_0019f910 == 1) {
      local_38 = DAT_0019f920;
      local_30 = DAT_0019f928;
    }
  }
  if (DAT_0019f864 == 4) {
    if ((0x1ffffffff < local_38) || (local_30 != 0)) {
      FUN_0011e1d3("mtime",param_1,&local_38);
    }
    if (0x1ffffffff < local_38) {
      local_38 = 0;
    }
  }
  cVar2 = FUN_00112af2(local_38,local_48 + 0x88,0xc);
  if (cVar2 != '\x01') {
    lVar7 = 0;
    goto LAB_00114172;
  }
  if (((*(uint *)(param_1 + 0x70) & 0xf000) == 0x2000) ||
     ((*(uint *)(param_1 + 0x70) & 0xf000) == 0x6000)) {
    local_68 = gnu_dev_major(*(ulonglong *)(param_1 + 0x80));
    local_64 = gnu_dev_minor(*(ulonglong *)(param_1 + 0x80));
    if ((DAT_0019f864 == 4) && (0x1fffff < local_68)) {
      FUN_0011e1d3("devmajor",param_1,0);
      local_68 = 0;
    }
    cVar2 = FUN_00112957(local_68,local_48 + 0x149,8);
    if (cVar2 != '\x01') {
      lVar7 = 0;
      goto LAB_00114172;
    }
    if ((DAT_0019f864 == 4) && (0x1fffff < local_64)) {
      FUN_0011e1d3("devminor",param_1,0);
      local_64 = 0;
    }
    cVar2 = FUN_001129ae(local_64,local_48 + 0x151,8);
    if (cVar2 != '\x01') {
      lVar7 = 0;
      goto LAB_00114172;
    }
  }
  else if (((DAT_0019f864 != 6) && (DAT_0019f864 != 2)) &&
          ((cVar2 = FUN_00112957(0,local_48 + 0x149,8), cVar2 != '\x01' ||
           (cVar2 = FUN_001129ae(0,local_48 + 0x151,8), cVar2 != '\x01')))) {
    lVar7 = 0;
    goto LAB_00114172;
  }
  if (DAT_0019f864 == 4) {
    FUN_0011e1d3("atime",param_1,0);
    FUN_0011e1d3("ctime",param_1,0);
  }
  else if ((DAT_0019f8be != '\0') && ((DAT_0019f864 == 2 || (DAT_0019f864 == 6)))) {
    FUN_00112af2(*(undefined8 *)(param_1 + 0xe8),local_48 + 0x159,0xc);
    FUN_00112af2(*(undefined8 *)(param_1 + 0x108),local_48 + 0x165,0xc);
  }
  if (DAT_0019f864 == 1) {
    uVar8 = 0;
  }
  else {
    uVar8 = 0x30;
  }
  *(undefined1 *)(local_48 + 0x9c) = uVar8;
  if (DAT_0019f864 == 6) {
LAB_00113e77:
    strcpy((char *)(local_48 + 0x101),"ustar  ");
  }
  else {
    if ((6 < DAT_0019f864) || (4 < DAT_0019f864)) {
LAB_00113ed9:
                    /* WARNING: Subroutine does not return */
      abort();
    }
    if (DAT_0019f864 < 3) {
      if (DAT_0019f864 != 1) {
        if (DAT_0019f864 != 2) goto LAB_00113ed9;
        goto LAB_00113e77;
      }
    }
    else {
      memcpy((void *)(local_48 + 0x101),"ustar",6);
      memcpy((void *)(local_48 + 0x107),&DAT_00180e49,2);
    }
  }
  if ((DAT_0019f864 != 1) && (DAT_0019f934 == '\0')) {
    if (local_60 == 0) {
      FUN_0012bfd1(*(undefined4 *)(param_1 + 0x74),param_1 + 0x20);
    }
    else {
      uVar5 = FUN_00160cc6(local_60);
      *(undefined8 *)(param_1 + 0x20) = uVar5;
    }
    if (local_58 == 0) {
      FUN_0012c0a2(*(undefined4 *)(param_1 + 0x78),param_1 + 0x28);
    }
    else {
      uVar5 = FUN_00160cc6(local_58);
      *(undefined8 *)(param_1 + 0x28) = uVar5;
    }
    if ((DAT_0019f864 == 4) &&
       ((sVar6 = strlen(*(char **)(param_1 + 0x20)), 0x20 < sVar6 ||
        (cVar2 = FUN_0013c9b7(*(undefined8 *)(param_1 + 0x20)), cVar2 != '\x01')))) {
      FUN_0011e1d3("uname",param_1,0);
    }
    FUN_00112c4c(*(undefined8 *)(param_1 + 0x20),local_48 + 0x109,0x20);
    if ((DAT_0019f864 == 4) &&
       ((sVar6 = strlen(*(char **)(param_1 + 0x28)), 0x20 < sVar6 ||
        (cVar2 = FUN_0013c9b7(*(undefined8 *)(param_1 + 0x28)), cVar2 != '\x01')))) {
      FUN_0011e1d3("gname",param_1,0);
    }
    FUN_00112c4c(*(undefined8 *)(param_1 + 0x28),local_48 + 0x129,0x20);
  }
  lVar7 = local_48;
  if (DAT_0019f864 == 4) {
    if (0 < DAT_0019f968) {
      if (*(long *)(param_1 + 0x38) != 0) {
        FUN_0011e1d3("SCHILY.acl.access",param_1,0);
      }
      if (*(long *)(param_1 + 0x48) != 0) {
        FUN_0011e1d3("SCHILY.acl.default",param_1,0);
      }
    }
    if ((0 < DAT_0019f964) && (*(long *)(param_1 + 0x30) != 0)) {
      FUN_0011e1d3("RHT.security.selinux",param_1,0);
    }
    lVar7 = local_48;
    if (0 < DAT_0019f96c) {
      local_38 = 0;
      local_40 = *(long *)(param_1 + 0x160);
      for (; lVar7 = local_48, local_38 < *(ulong *)(param_1 + 0x158); local_38 = local_38 + 1) {
        FUN_0011e1d3(*(undefined8 *)(local_40 + local_38 * 0x18),param_1,&local_38);
      }
    }
  }
LAB_00114172:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return lVar7;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: simple_finish_header @ 0x1418c

void simple_finish_header(byte *param_1)

{
  int local_1c;
  long local_18;
  byte *local_10;
  
  memcpy(param_1 + 0x94,"        ",8);
  local_1c = 0;
  local_18 = 0x200;
  local_10 = param_1;
  while (local_18 != 0) {
    local_1c = local_1c + (uint)*local_10;
    local_18 = local_18 + -1;
    local_10 = local_10 + 1;
  }
  FUN_00112bf9((long)local_1c,param_1 + 0x94,7);
  FUN_0010bba7(param_1);
  return;
}




// Function: finish_header @ 0x1422e

void finish_header(undefined8 param_1,long param_2,undefined8 param_3)

{
  undefined8 uVar1;
  
  if ((((DAT_0019f9ac != 0) && (*(char *)(param_2 + 0x9c) != 'K')) &&
      (*(char *)(param_2 + 0x9c) != 'L')) &&
     ((*(char *)(param_2 + 0x9c) != 'x' && (*(char *)(param_2 + 0x9c) != 'g')))) {
    DAT_0019f468 = DAT_0019f864;
    FUN_0012864c(param_1,param_2,param_3);
  }
  uVar1 = FUN_001135e3(0,param_1,param_2);
  FUN_0011418c(uVar1);
  return;
}




// Function: pad_archive @ 0x142d7

void pad_archive(long param_1)

{
  void *__s;
  undefined8 local_20;
  
  for (local_20 = param_1; 0 < local_20; local_20 = local_20 + -0x200) {
    __s = (void *)FUN_0010bb4b();
    memset(__s,0,0x200);
    FUN_0010bba7(__s);
  }
  return;
}




// Function: dump_regular_file @ 0x14327

undefined8 dump_regular_file(int param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  long lVar2;
  ulong uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  ulong local_60;
  ulong local_58;
  undefined1 local_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_60 = param_2[0x11];
  uVar1 = FUN_0010bacd();
  lVar2 = FUN_0011381d(param_2);
  if (lVar2 == 0) {
    uVar1 = 2;
  }
  else {
    FUN_0011422e(param_2,lVar2,uVar1);
    FUN_0010ab09(param_2[1],param_2[0x11],param_2[0x11]);
    do {
      if ((long)local_60 < 1) {
        uVar1 = 0;
        goto LAB_001145c6;
      }
      lVar2 = FUN_0010bb4b();
      local_58 = FUN_0010bbf5(lVar2);
      if (local_60 < local_58) {
        local_58 = local_60;
        uVar3 = (ulong)((uint)local_60 & 0x1ff);
        if (uVar3 != 0) {
          memset((void *)(lVar2 + local_60),0,0x200 - uVar3);
        }
      }
      uVar3 = local_58;
      if (0 < param_1) {
        uVar3 = FUN_0012a9e9(param_1,lVar2,local_58);
      }
      if (uVar3 == 0xffffffffffffffff) {
        FUN_0012b2f8(*param_2,param_2[0x11] - local_60,local_58);
        FUN_001142d7(local_60);
        uVar1 = 1;
        goto LAB_001145c6;
      }
      local_60 = local_60 - uVar3;
      FUN_0010bba7(lVar2 + (local_58 - 1 & 0xfffffffffffffe00));
    } while (uVar3 == local_58);
    memset((void *)(lVar2 + uVar3),0,local_58 - uVar3);
    if ((DAT_0019ec84 & 0x80) != 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar1 = FUN_00154e5a(local_60,local_38);
      uVar4 = FUN_0015e548(*param_2);
      uVar5 = ngettext("%s: File shrank by %s byte; padding with zeros",
                       "%s: File shrank by %s bytes; padding with zeros",local_60);
      error(0,0,uVar5,uVar4,uVar1);
    }
    if (DAT_0019f8bc != '\x01') {
      FUN_00139e98(1);
    }
    FUN_001142d7(local_60 + (uVar3 - local_58));
    uVar1 = 1;
  }
LAB_001145c6:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: dump_dir0 @ 0x145e3

void dump_dir0(undefined8 *param_1,char *param_2)

{
  long lVar1;
  char cVar2;
  uint uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  size_t sVar6;
  size_t sVar7;
  long in_FS_OFFSET;
  char *local_90;
  ulong local_88;
  ulong local_80;
  void *local_78;
  char *local_70;
  size_t local_68;
  char *local_60;
  void *local_58;
  undefined8 local_50;
  void *local_48;
  ulong local_40;
  ulong local_38;
  size_t local_30;
  size_t local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  lVar1 = param_1[0x33];
  local_58 = (void *)0x0;
  local_50 = FUN_0010bacd();
  param_1[0x11] = 0;
  local_58 = (void *)FUN_0011381d(param_1);
  if (local_58 != (void *)0x0) {
    FUN_00117909(param_1);
    if ((DAT_0019f8be == '\0') || (DAT_0019f864 == 4)) {
      *(undefined1 *)((long)local_58 + 0x9c) = 0x35;
    }
    else {
      *(undefined1 *)((long)local_58 + 0x9c) = 0x44;
    }
    if (DAT_0019f8be == '\x01') {
      if (*(long *)(DAT_0019f6b0 + 0x38) != 0) {
        if (DAT_0019f864 == 4) {
          uVar4 = FUN_00122456(*(undefined8 *)(DAT_0019f6b0 + 0x38));
          FUN_0011e1d3("GNU.dumpdir",param_1,uVar4);
          FUN_0011422e(param_1,local_58,local_50);
        }
        else {
          local_50 = FUN_0010bacd();
          local_48 = (void *)FUN_00122456(*(undefined8 *)(DAT_0019f6b0 + 0x38));
          local_40 = FUN_00120bfe(local_48);
          FUN_00112a9a(local_40,(long)local_58 + 0x7c,0xc);
          FUN_0011422e(param_1,local_58,local_50);
          local_78 = local_48;
          local_88 = local_40;
          FUN_0010ab09(param_1[1],local_40,local_40);
          while (0 < (long)local_88) {
            local_58 = (void *)FUN_0010bb4b();
            local_80 = FUN_0010bbf5(local_58);
            if (local_88 < local_80) {
              local_80 = local_88;
              local_38 = (ulong)((uint)local_88 & 0x1ff);
              if (local_38 != 0) {
                memset((void *)(local_88 + (long)local_58),0,0x200 - local_38);
              }
            }
            memcpy(local_58,local_78,local_80);
            local_88 = local_88 - local_80;
            local_78 = (void *)((long)local_78 + local_80);
            FUN_0010bba7((long)local_58 + (local_80 - 1 & 0xfffffffffffffe00));
          }
        }
        goto LAB_00114bb5;
      }
    }
    else {
      FUN_0011422e(param_1,local_58,local_50);
    }
    if (DAT_0019f930 != 0) {
      if (((DAT_0019f935 == '\0') || (lVar1 == 0)) ||
         (*(long *)(param_1[0x33] + 0x58) == param_1[0xb])) {
        uVar3 = FUN_00111fbf(param_1,&local_90);
        if ((uVar3 != 3) && (uVar3 < 4)) {
          if (uVar3 == 2) {
            uVar4 = gettext("contents not dumped");
            FUN_00111f18(*param_1,local_90,uVar4);
          }
          else if (uVar3 < 3) {
            if (uVar3 == 0) {
              local_70 = (char *)FUN_00160cc6(*param_1);
              local_68 = strlen(local_70);
              local_60 = param_2;
              local_30 = local_68;
              while (local_28 = strlen(local_60), local_28 != 0) {
                if (local_68 < local_28 + local_30) {
                  local_68 = local_28 + local_30;
                  local_70 = (char *)FUN_00160b8d(local_70,local_68 + 1);
                }
                strcpy(local_70 + local_30,local_60);
                cVar2 = FUN_00117c43(local_70,param_1);
                if (cVar2 != '\x01') {
                  FUN_00116a25(param_1,local_60,local_70);
                }
                local_60 = local_60 + local_28 + 1;
              }
              free(local_70);
            }
            else if (uVar3 == 1) {
              uVar4 = gettext("contents not dumped");
              FUN_00111f18(*param_1,local_90,uVar4);
              sVar6 = strlen((char *)*param_1);
              sVar7 = strlen(local_90);
              local_68 = sVar7 + sVar6 + 1;
              local_70 = (char *)FUN_00160b5b(local_68);
              strcpy(local_70,(char *)*param_1);
              strcat(local_70,local_90);
              FUN_00116a25(param_1,local_90,local_70);
              free(local_70);
            }
          }
        }
      }
      else if ((DAT_0019f9ac != 0) && ((DAT_0019ec84 & 0x40000) != 0)) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar4 = FUN_0015e548(*param_1);
        uVar5 = gettext("%s: file is on a different filesystem; not dumped");
        error(0,0,uVar5,uVar4);
      }
    }
  }
LAB_00114bb5:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: ensure_slash @ 0x14bcf

void ensure_slash(long *param_1)

{
  long lVar1;
  undefined8 local_10;
  
  for (local_10 = strlen((char *)*param_1);
      (local_10 != 0 && (*(char *)((local_10 - 1) + *param_1) == '/')); local_10 = local_10 - 1) {
  }
  if (*(char *)(local_10 + *param_1) != '/') {
    lVar1 = FUN_00160b8d(*param_1,local_10 + 2);
    *param_1 = lVar1;
  }
  *(undefined1 *)(local_10 + *param_1) = 0x2f;
  *(undefined1 *)(local_10 + 1 + *param_1) = 0;
  return;
}




// Function: open_failure_recover @ 0x14c7c

undefined8 open_failure_recover(long param_1)

{
  int *piVar1;
  long local_10;
  
  piVar1 = __errno_location();
  if (((*piVar1 == 0x18) && (param_1 != 0)) && (*(long *)(param_1 + 0x198) != 0)) {
    for (local_10 = *(long *)(*(long *)(param_1 + 0x198) + 0x198); local_10 != 0;
        local_10 = *(long *)(local_10 + 0x198)) {
      if ((0 < *(int *)(local_10 + 0x1a8)) &&
         ((*(long *)(local_10 + 0x198) == 0 || (*(int *)(*(long *)(local_10 + 0x198) + 0x1a8) < 1)))
         ) {
        FUN_00139c7e(local_10);
        return 1;
      }
    }
    piVar1 = __errno_location();
    *piVar1 = 0x18;
  }
  return 0;
}




// Function: get_directory_entries @ 0x14d3d

undefined8 get_directory_entries(long param_1)

{
  char cVar1;
  DIR *pDVar2;
  undefined8 uVar3;
  
  do {
    pDVar2 = fdopendir(*(int *)(param_1 + 0x1a8));
    *(DIR **)(param_1 + 0x1a0) = pDVar2;
    if (*(long *)(param_1 + 0x1a0) != 0) {
      uVar3 = FUN_0015eee2(*(undefined8 *)(param_1 + 0x1a0),DAT_0019fc2c);
      return uVar3;
    }
    cVar1 = FUN_00114c7c(param_1);
  } while (cVar1 == '\x01');
  return 0;
}




// Function: dump_dir @ 0x14db4

bool dump_dir(undefined8 *param_1)

{
  void *__ptr;
  
  __ptr = (void *)FUN_00114d3d(param_1);
  if (__ptr != (void *)0x0) {
    FUN_001145e3(param_1,__ptr);
    FUN_0011588e(param_1);
    free(__ptr);
  }
  else {
    FUN_0012b3a4(*param_1);
  }
  return __ptr != (void *)0x0;
}




// Function: create_archive @ 0x14e23

void create_archive(void)

{
  char cVar1;
  int iVar2;
  long in_FS_OFFSET;
  ulong local_200;
  void *local_1f8;
  char *local_1f0;
  ulong local_1e8;
  long local_1e0;
  long local_1d8;
  size_t local_1d0;
  long local_1c8 [11];
  stat local_170 [2];
  int local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if ((DAT_0019f6a0 == 2) || (DAT_0019f8a0 == '\x01')) {
    iVar2 = 0;
  }
  else {
    iVar2 = 1;
  }
  DAT_0019f338 = (long)iVar2;
  FUN_0010f694(1);
  FUN_0010ad11();
  if (DAT_0019f8be == '\0') {
    while (local_1e0 = FUN_0012d37c(1), local_1e0 != 0) {
      cVar1 = FUN_00117c43(local_1e0,0);
      if (cVar1 != '\x01') {
        FUN_00116a25(0,local_1e0,local_1e0);
      }
    }
  }
  else {
    local_200 = 0;
    local_1f8 = (void *)0x0;
    FUN_0012e719();
    while (local_1d8 = FUN_0012eda0(), local_1d8 != 0) {
      cVar1 = FUN_00117c43(*(undefined8 *)(local_1d8 + 0x10),0);
      if (cVar1 != '\x01') {
        FUN_00116a25(0,*(undefined8 *)(local_1d8 + 0x10),*(undefined8 *)(local_1d8 + 0x10));
      }
    }
    FUN_0012ee43();
    while (local_1d8 = FUN_0012eda0(), local_1d8 != 0) {
      cVar1 = FUN_00117c43(*(undefined8 *)(local_1d8 + 0x10),0);
      if (cVar1 != '\x01') {
        local_1e8 = strlen(*(char **)(local_1d8 + 0x10));
        while (local_200 <= local_1e8) {
          local_1f8 = (void *)FUN_00160bd8(local_1f8,&local_200);
        }
        memcpy(local_1f8,*(void **)(local_1d8 + 0x10),local_1e8);
        if (*(char *)((long)local_1f8 + (local_1e8 - 1)) != '/') {
          *(undefined1 *)(local_1e8 + (long)local_1f8) = 0x2f;
          local_1e8 = local_1e8 + 1;
        }
        FUN_00139c55(local_1c8);
        local_1f0 = (char *)FUN_0012241b(*(undefined8 *)(local_1d8 + 0x38));
        if (local_1f0 != (char *)0x0) {
          for (; *local_1f0 != '\0'; local_1f0 = local_1f0 + local_1d0 + 1) {
            local_1d0 = strlen(local_1f0);
            if (*local_1f0 == 'Y') {
              if (local_1c8[0] == 0) {
                iVar2 = openat(DAT_0019c268,*(char **)(local_1d8 + 0x10),DAT_0019fc0c);
                if (iVar2 < 0) {
                  FUN_0012b48b(*(undefined8 *)(local_1d8 + 0x10),*(long *)(local_1d8 + 0x40) == 0,
                               FUN_0012b2b1);
                  break;
                }
                local_20 = iVar2;
                iVar2 = fstat(iVar2,local_170);
                if (iVar2 != 0) {
                  FUN_0012b48b(*(undefined8 *)(local_1d8 + 0x10),*(long *)(local_1d8 + 0x40) == 0,
                               FUN_0012b444);
                  break;
                }
                local_1c8[0] = FUN_00160cc6(*(undefined8 *)(local_1d8 + 0x10));
              }
              while (local_200 < local_1e8 + local_1d0) {
                local_1f8 = (void *)FUN_00160bd8(local_1f8,&local_200);
              }
              strcpy((char *)(local_1e8 + (long)local_1f8),local_1f0 + 1);
              FUN_00116a25(local_1c8,local_1f0 + 1,local_1f8);
            }
          }
        }
        FUN_00139d1c(local_1c8);
      }
    }
    free(local_1f8);
  }
  FUN_00112da0();
  FUN_0010d046();
  FUN_0013be6b();
  if (DAT_0019f8e0 != 0) {
    FUN_00124600();
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: hash_link @ 0x1529f

ulong hash_link(ulong *param_1,ulong param_2)

{
  return (param_1[1] ^ *param_1) % param_2;
}




// Function: compare_links @ 0x152df

undefined8 compare_links(ulong *param_1,ulong *param_2)

{
  ulong uVar1;
  
  uVar1 = param_2[1] ^ param_1[1] | *param_1 ^ *param_2;
  return CONCAT71((int7)(uVar1 >> 8),uVar1 == 0);
}




// Function: unknown_file_error @ 0x15331

void unknown_file_error(undefined8 param_1)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  if ((DAT_0019ec84 & 0x20) != 0) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar1 = FUN_0015e548(param_1);
    uVar2 = gettext("%s: Unknown file type; file ignored");
    error(0,0,uVar2,uVar1);
  }
  if (DAT_0019f8bc != '\x01') {
    FUN_00139e98(2);
  }
  return;
}




// Function: dump_hard_link @ 0x153bb

undefined8 dump_hard_link(undefined8 *param_1)

{
  long lVar1;
  char *__s;
  undefined8 uVar2;
  size_t sVar3;
  ulong uVar4;
  long in_FS_OFFSET;
  undefined8 local_48;
  undefined8 local_40;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if ((DAT_0019f340 != 0) && ((DAT_0019f338 < (ulong)param_1[0xd] || (DAT_0019f94e != '\0')))) {
    local_40 = param_1[0xc];
    local_48 = param_1[0xb];
    lVar1 = FUN_00152567(DAT_0019f340,&local_48);
    if (lVar1 != 0) {
      __s = (char *)FUN_0013edd9(lVar1 + 0x18,1,DAT_0019f878);
      if (*(long *)(lVar1 + 0x10) != 0) {
        *(long *)(lVar1 + 0x10) = *(long *)(lVar1 + 0x10) + -1;
      }
      uVar2 = FUN_0010bacd();
      FUN_00129547(param_1 + 3,__s);
      if (DAT_0019f864 == 2) {
        uVar4 = 99;
      }
      else {
        uVar4 = 100;
      }
      sVar3 = strlen(__s);
      if (uVar4 < sVar3) {
        FUN_001133a8(param_1);
      }
      param_1[0x11] = 0;
      lVar1 = FUN_0011381d(param_1);
      if (lVar1 == 0) {
        uVar2 = 0;
      }
      else {
        FUN_00112157(lVar1 + 0x9d,__s,100);
        *(undefined1 *)(lVar1 + 0x9c) = 0x31;
        FUN_0011422e(param_1,lVar1,uVar2);
        if (DAT_0019f94e != '\0') {
          FUN_0013bebb(*param_1,0);
        }
        uVar2 = 1;
      }
      goto LAB_0011557e;
    }
  }
  uVar2 = 0;
LAB_0011557e:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: file_count_links @ 0x15598

void file_count_links(undefined8 *param_1)

{
  undefined8 uVar1;
  size_t sVar2;
  long in_FS_OFFSET;
  char *local_28;
  undefined8 *local_20;
  undefined8 *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if ((DAT_0019f8a1 != '\0') || ((ulong)param_1[0xd] <= DAT_0019f338)) goto LAB_00115729;
  local_28 = (char *)0x0;
  uVar1 = FUN_0013edd9(*param_1,1,DAT_0019f878);
  FUN_00129547(&local_28,uVar1);
  FUN_0013b9f2(&local_28,2);
  sVar2 = strlen(local_28);
  local_20 = (undefined8 *)FUN_00160b5b(sVar2 + 0x19);
  local_20[1] = param_1[0xc];
  *local_20 = param_1[0xb];
  local_20[2] = param_1[0xd];
  strcpy((char *)(local_20 + 3),local_28);
  free(local_28);
  if (DAT_0019f340 == 0) {
    DAT_0019f340 = FUN_00152c1c(0,0,FUN_0011529f,FUN_001152df,0);
    if (DAT_0019f340 != 0) goto LAB_001156dd;
LAB_001156fe:
    FUN_0011783c();
  }
  else {
LAB_001156dd:
    local_18 = (undefined8 *)FUN_0015395d(DAT_0019f340,local_20);
    if (local_18 == (undefined8 *)0x0) goto LAB_001156fe;
  }
  if (local_18 != local_20) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  local_20[2] = local_20[2] + -1;
LAB_00115729:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: check_links @ 0x1573f

void check_links(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  long local_20;
  
  if (DAT_0019f340 != 0) {
    for (local_20 = FUN_001525fe(DAT_0019f340); local_20 != 0;
        local_20 = FUN_0015265e(DAT_0019f340,local_20)) {
      if (*(long *)(local_20 + 0x10) != 0) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar1 = FUN_0015e84e(local_20 + 0x18);
        uVar2 = gettext("Missing links to %s.");
        error(0,0,uVar2,uVar1);
      }
    }
  }
  return;
}




// Function: subfile_open @ 0x157f9

int subfile_open(long param_1,char *param_2,int param_3)

{
  char cVar1;
  int iVar2;
  
  if (DAT_0019f368 != '\x01') {
    DAT_0019f368 = '\x01';
    strerror(2);
    gettext(&DAT_00181099);
  }
  do {
    iVar2 = DAT_0019c268;
    if (param_1 != 0) {
      iVar2 = *(int *)(param_1 + 0x1a8);
    }
    iVar2 = openat(iVar2,param_2,param_3);
  } while ((iVar2 < 0) && (cVar1 = FUN_00114c7c(param_1), cVar1 != '\0'));
  return iVar2;
}




// Function: restore_parent_fd @ 0x1588e

void restore_parent_fd(long param_1)

{
  undefined8 *puVar1;
  int __fd;
  int iVar2;
  int *piVar3;
  long in_FS_OFFSET;
  int local_b8;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  puVar1 = *(undefined8 **)(param_1 + 0x198);
  if ((puVar1 != (undefined8 *)0x0) && (*(int *)(puVar1 + 0x35) == 0)) {
    local_b8 = openat(*(int *)(param_1 + 0x1a8),"..",DAT_0019fc0c);
    if (local_b8 < 0) {
      piVar3 = __errno_location();
      local_b8 = -*piVar3;
    }
    else {
      iVar2 = fstat(local_b8,&local_a8);
      if (((iVar2 != 0) || (puVar1[0xc] != local_a8.st_ino)) || (puVar1[0xb] != local_a8.st_dev)) {
        close(local_b8);
        local_b8 = 2;
      }
    }
    iVar2 = local_b8;
    if (((local_b8 < 0) && (__fd = openat(DAT_0019c268,(char *)*puVar1,DAT_0019fc0c), -1 < __fd)) &&
       ((iVar2 = fstat(local_b8,&local_a8), iVar2 != 0 ||
        ((puVar1[0xc] != local_a8.st_ino || (iVar2 = __fd, puVar1[0xb] != local_a8.st_dev)))))) {
      close(__fd);
      iVar2 = local_b8;
    }
    local_b8 = iVar2;
    *(int *)(puVar1 + 0x35) = local_b8;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: dump_file0 @ 0x15a63

void dump_file0(undefined8 *param_1,char *param_2,undefined8 param_3)

{
  char cVar1;
  byte bVar2;
  int iVar3;
  uint uVar4;
  undefined8 uVar5;
  int *piVar6;
  undefined8 uVar7;
  long lVar8;
  size_t sVar9;
  ulong uVar10;
  long in_FS_OFFSET;
  bool bVar11;
  bool bVar12;
  undefined1 auVar13 [16];
  bool local_108;
  char local_107;
  int local_104;
  int local_100;
  uint local_fc;
  undefined8 local_f8;
  code *local_f0;
  undefined8 local_e8;
  long local_e0;
  long local_d8;
  long local_d0;
  undefined1 local_c8 [16];
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_e8 = 0xffffffffffffffff;
  local_100 = 0;
  local_e0 = param_1[0x33];
  bVar11 = local_e0 == 0;
  local_104 = DAT_0019c268;
  if (!bVar11) {
    local_104 = *(int *)(local_e0 + 0x1a8);
  }
  local_f0 = (code *)0x0;
  if (DAT_0019f8c8 != '\0') {
    iVar3 = FUN_00134af4(&DAT_0018109d,param_3);
    if (iVar3 == 0) goto LAB_00116a0b;
  }
  FUN_00129547(param_1,param_3);
  uVar5 = FUN_0013edd9(param_3,0,DAT_0019f878);
  FUN_00129547(param_1 + 1,uVar5);
  FUN_0013b9f2(param_1 + 1,1);
  if ((-1 < local_104) || (bVar11)) {
    iVar3 = fstatat(local_104,param_2,(stat *)(param_1 + 0xb),DAT_0019fc10);
    if (iVar3 == 0) {
      cVar1 = FUN_00112c90(param_1 + 0xb);
      if (cVar1 != '\0') {
        local_100 = FUN_001157f9(local_e0,param_2,DAT_0019fc08);
        if (local_100 < 0) {
          local_f0 = FUN_0012b2b1;
        }
        else {
          *(int *)(param_1 + 0x35) = local_100;
          iVar3 = fstat(local_100,(stat *)(param_1 + 0xb));
          if (iVar3 != 0) {
            local_f0 = FUN_0012b444;
          }
        }
      }
    }
    else {
      local_f0 = FUN_0012b444;
    }
  }
  else {
    piVar6 = __errno_location();
    *piVar6 = -local_104;
    local_f0 = FUN_0012b2b1;
  }
  if (local_f0 != (code *)0x0) {
    FUN_0012b48b(param_3,bVar11,local_f0);
    goto LAB_00116a0b;
  }
  local_d8 = param_1[0x11];
  param_1[0x23] = local_d8;
  auVar13 = FUN_0015fd6a(param_1 + 0xb);
  *(undefined1 (*) [16])(param_1 + 0x1d) = auVar13;
  auVar13 = FUN_0015fd9e(param_1 + 0xb);
  *(undefined1 (*) [16])(param_1 + 0x1f) = auVar13;
  local_c8 = FUN_0015fd84(param_1 + 0xb);
  *(undefined1 (*) [16])(param_1 + 0x21) = local_c8;
  if (((DAT_0019f8be != '\x01') || (bVar11)) && ((*(uint *)(param_1 + 0xe) & 0xf000) != 0x4000)) {
    iVar3 = FUN_0015ff90(param_1[0x1f],param_1[0x20],DAT_0019f900,DAT_0019f908);
    if (iVar3 < 0) {
      if (DAT_0019f87c != 0) {
        iVar3 = FUN_0015ff90(param_1[0x21],param_1[0x22],DAT_0019f900,DAT_0019f908);
        if (-1 < iVar3) goto LAB_00115ec3;
      }
      if (((DAT_0019f8be != '\x01') && (DAT_0019f9ac != 0)) && ((DAT_0019ec84 & 0x100) != 0)) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar5 = FUN_0015e548(param_3);
        uVar7 = gettext("%s: file is unchanged; not dumped");
        error(0,0,uVar7,uVar5);
      }
      goto LAB_00116a0b;
    }
  }
LAB_00115ec3:
  cVar1 = FUN_00132b37(param_1);
  if (cVar1 != '\0') {
    if ((DAT_0019ec84 & 0x400) != 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar5 = FUN_0015e548(param_3);
      uVar7 = gettext("%s: file is the archive; not dumped");
      error(0,0,uVar7,uVar5);
    }
    goto LAB_00116a0b;
  }
  bVar12 = (*(uint *)(param_1 + 0xe) & 0xf000) == 0x4000;
  if (!bVar12) {
    cVar1 = FUN_001153bb(param_1);
    if (cVar1 != '\0') goto LAB_00116a0b;
  }
  if ((!bVar12) && ((*(uint *)(param_1 + 0xe) & 0xf000) != 0x8000)) {
    if ((*(uint *)(param_1 + 0xe) & 0xf000) == 0xa000) {
      uVar5 = FUN_00147649(local_104,param_2,param_1[0x11]);
      param_1[3] = uVar5;
      if (param_1[3] == 0) {
        piVar6 = __errno_location();
        if (*piVar6 == 0xc) {
          FUN_0011783c();
        }
        FUN_0012b48b(param_3,bVar11,FUN_0012b35d);
      }
      else {
        FUN_0013b9f2(param_1 + 3,4);
        if (DAT_0019f864 == 2) {
          uVar10 = 99;
        }
        else {
          uVar10 = 100;
        }
        sVar9 = strlen((char *)param_1[3]);
        if (uVar10 < sVar9) {
          FUN_001133a8(param_1);
        }
        FUN_0013d566(local_104,param_2,param_1,0);
        FUN_0013d1a5(local_104,param_2,param_1,0);
        local_e8 = FUN_0010bacd();
        param_1[0x11] = 0;
        local_d0 = FUN_0011381d(param_1);
        if (local_d0 != 0) {
          FUN_00112157(local_d0 + 0x9d,param_1[3],100);
          *(undefined1 *)(local_d0 + 0x9c) = 0x32;
          FUN_0011422e(param_1,local_d0,local_e8);
          if (DAT_0019f94e != '\0') {
            FUN_0013bebb(param_3,0);
          }
          FUN_00115598(param_1);
        }
      }
    }
    else {
      if ((*(uint *)(param_1 + 0xe) & 0xf000) == 0x2000) {
        local_107 = '3';
        FUN_0013cf89(local_104,param_2,param_1,0,1);
        FUN_0013d566(local_104,param_2,param_1,0);
        FUN_0013d1a5(local_104,param_2,param_1,0);
      }
      else if ((*(uint *)(param_1 + 0xe) & 0xf000) == 0x6000) {
        local_107 = '4';
        FUN_0013cf89(local_104,param_2,param_1,0,1);
        FUN_0013d566(local_104,param_2,param_1,0);
        FUN_0013d1a5(local_104,param_2,param_1,0);
      }
      else {
        if ((*(uint *)(param_1 + 0xe) & 0xf000) != 0x1000) {
          if ((*(uint *)(param_1 + 0xe) & 0xf000) == 0xc000) {
            if ((DAT_0019ec84 & 0x20) != 0) {
              if (DAT_0019fdc8 != (code *)0x0) {
                (*DAT_0019fdc8)();
              }
              uVar5 = FUN_0015e548(param_3);
              uVar7 = gettext("%s: socket ignored");
              error(0,0,uVar7,uVar5);
            }
          }
          else {
            FUN_00115331(param_3);
          }
          goto LAB_00116a0b;
        }
        local_107 = '6';
        FUN_0013cf89(local_104,param_2,param_1,0,1);
        FUN_0013d566(local_104,param_2,param_1,0);
        FUN_0013d1a5(local_104,param_2,param_1,0);
      }
      if (DAT_0019f864 == 1) {
        FUN_00115331(param_3);
      }
      else {
        local_e8 = FUN_0010bacd();
        param_1[0x11] = 0;
        local_d0 = FUN_0011381d(param_1);
        if (local_d0 != 0) {
          *(char *)(local_d0 + 0x9c) = local_107;
          if (local_107 != '6') {
            lVar8 = local_d0 + 0x149;
            uVar4 = gnu_dev_major(param_1[0x10]);
            FUN_00112957(uVar4,lVar8,8);
            lVar8 = local_d0 + 0x151;
            uVar4 = gnu_dev_minor(param_1[0x10]);
            FUN_001129ae(uVar4,lVar8,8);
          }
          FUN_0011422e(param_1,local_d0,local_e8);
          if (DAT_0019f94e != '\0') {
            FUN_0013bebb(param_3,0);
          }
        }
      }
    }
    goto LAB_00116a0b;
  }
  FUN_0013cf89(local_104,param_2,param_1,0,!bVar12);
  FUN_0013d566(local_104,param_2,param_1,local_100);
  FUN_0013d1a5(local_104,param_2,param_1,local_100);
  if (bVar12) {
    FUN_00114bcf(param_1);
    FUN_00114bcf(param_1 + 1);
    iVar3 = FUN_00111fbf(param_1,&local_f8);
    if (iVar3 == 3) {
      uVar5 = gettext("directory not dumped");
      FUN_00111f18(*param_1,local_f8,uVar5);
      goto LAB_00116a0b;
    }
    local_108 = (bool)FUN_00114db4(param_1);
    local_100 = *(int *)(param_1 + 0x35);
    local_104 = DAT_0019c268;
    if (!bVar11) {
      local_104 = *(int *)(local_e0 + 0x1a8);
    }
  }
  else {
    if ((local_100 == 0) || (DAT_0019f979 == '\0')) {
LAB_001161c2:
      local_fc = FUN_00114327(local_100,param_1);
    }
    else {
      lVar8 = param_1[0x11];
      if (lVar8 < 0) {
        lVar8 = lVar8 + 0x1ff;
      }
      if (((param_1[0x11] & 0x1ff) == 0) || (param_1[0x11] + 0x1ff < 0x3ff)) {
        iVar3 = 0;
      }
      else {
        iVar3 = 1;
      }
      if ((long)iVar3 + (lVar8 >> 9) <= (long)param_1[0x13]) goto LAB_001161c2;
      local_fc = FUN_00130090(local_100,param_1);
      if (local_fc == 3) {
        local_fc = FUN_00114327(local_100,param_1);
      }
    }
    if (local_fc == 3) {
                    /* WARNING: Subroutine does not return */
      abort();
    }
    if ((local_fc < 4) && (local_fc < 2)) {
      FUN_00115598(param_1);
    }
    local_108 = local_fc == 0;
  }
  if (local_108 != false) {
    if (local_100 < 0) {
      piVar6 = __errno_location();
      *piVar6 = -local_100;
      local_108 = false;
    }
    else if (local_100 == 0) {
      if ((-1 < local_104) || (bVar11)) {
        iVar3 = fstatat(local_104,param_2,&local_b8,DAT_0019fc10);
        local_108 = iVar3 == 0;
      }
      else {
        piVar6 = __errno_location();
        *piVar6 = -local_104;
        local_108 = false;
      }
    }
    else {
      iVar3 = fstat(local_100,&local_b8);
      local_108 = iVar3 == 0;
    }
    if (local_108 != true) {
      FUN_0012b48b(param_3,bVar11,FUN_0012b444);
    }
  }
  if (local_108 != false) {
    auVar13 = FUN_0015fd84(&local_b8);
    iVar3 = FUN_0015ff90(auVar13._0_8_,auVar13._8_8_,local_c8._0_8_,local_c8._8_8_);
    if (((iVar3 == 0) || ((DAT_0019f94e == '\x01' && (bVar12)))) && (local_b8.st_size <= local_d8))
    {
      if (((DAT_0019f880 == 1) && (local_100 != 0)) && ((bVar12 || (local_d8 != 0)))) {
        iVar3 = FUN_0012ab5e(local_100,local_104,param_2,param_1[0x1d],param_1[0x1e]);
        if (iVar3 != 0) {
          FUN_0013ead0(param_3);
        }
      }
    }
    else {
      if ((DAT_0019ec84 & 0x10) != 0) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar5 = FUN_0015e548(param_3);
        uVar7 = gettext("%s: file changed as we read it");
        error(0,0,uVar7,uVar5);
      }
      FUN_00139e98(1);
    }
  }
  bVar2 = FUN_00139c7e(param_1);
  if (((bVar2 & local_108) != 0) && (DAT_0019f94e != '\0')) {
    FUN_0013bebb(param_3,bVar12);
  }
LAB_00116a0b:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: dump_file @ 0x16a25

void dump_file(long param_1,undefined8 param_2,undefined8 param_3)

{
  long in_FS_OFFSET;
  undefined1 local_1c8 [408];
  long local_30;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00139c55(local_1c8);
  local_30 = param_1;
  FUN_00115a63(local_1c8,param_2,param_3);
  if ((param_1 != 0) && (DAT_0019f8e0 != 0)) {
    FUN_0012130d(param_1);
  }
  FUN_00139d1c(local_1c8);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: move_archive @ 0x16add

void move_archive(long param_1)

{
  char cVar1;
  char extraout_var_00;
  int iVar2;
  char extraout_var;
  int *piVar3;
  long lVar4;
  long lVar5;
  long in_FS_OFFSET;
  bool bVar6;
  long local_30;
  undefined2 local_18 [2];
  int local_14;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 != 0) {
    local_14 = (int)param_1;
    if (param_1 < 0) {
      local_18[0] = 4;
      local_14 = -local_14;
      bVar6 = (long)local_14 == -param_1;
    }
    else {
      local_18[0] = 3;
      bVar6 = param_1 == local_14;
    }
    if (bVar6) {
      if (DAT_0019f9cc < 0x40000000) {
        iVar2 = ioctl(DAT_0019f9cc,0x40086d01,local_18);
        cVar1 = (char)((uint)iVar2 >> 0x18);
      }
      else {
        FUN_00140254(DAT_0019f9cc + -0x40000000,0x40086d01,local_18);
        cVar1 = extraout_var;
      }
      if (-1 < cVar1) goto LAB_00116d34;
      piVar3 = __errno_location();
      if (*piVar3 == 5) {
        if (DAT_0019f9cc < 0x40000000) {
          iVar2 = ioctl(DAT_0019f9cc,0x40086d01,local_18);
          cVar1 = (char)((uint)iVar2 >> 0x18);
        }
        else {
          FUN_00140254(DAT_0019f9cc + -0x40000000,0x40086d01,local_18);
          cVar1 = extraout_var_00;
        }
        if (-1 < cVar1) goto LAB_00116d34;
      }
    }
    if (DAT_0019f9cc < 0x40000000) {
      lVar4 = lseek(DAT_0019f9cc,0,1);
    }
    else {
      lVar4 = FUN_001400be(DAT_0019f9cc + -0x40000000,0,1);
    }
    lVar5 = param_1 * DAT_0019f870;
    local_30 = lVar5 + lVar4;
    if ((lVar5 / param_1 == DAT_0019f870) && (SUB81((ulong)lVar5 >> 0x3f,0) == local_30 < lVar4)) {
      if (local_30 < 0) {
        local_30 = 0;
      }
      if (DAT_0019f9cc < 0x40000000) {
        lVar4 = lseek(DAT_0019f9cc,local_30,0);
      }
      else {
        lVar4 = FUN_001400be(DAT_0019f9cc + -0x40000000,local_30,0);
      }
      if (lVar4 == local_30) goto LAB_00116d34;
    }
    FUN_0013e7a9(*DAT_0019fbd8,local_30);
  }
LAB_00116d34:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: write_record @ 0x16d4a

void write_record(int param_1)

{
  undefined8 uVar1;
  
  uVar1 = DAT_0019efa0;
  DAT_0019efa0 = DAT_0019f378;
  if (DAT_0019f384 == '\0') {
    FUN_00116add((DAT_0019f370 + DAT_0019efc8) - DAT_0019efc0);
    FUN_0010f676();
  }
  else {
    DAT_0019f9cc = 1;
    FUN_0010f676();
    DAT_0019f9cc = 0;
  }
  DAT_0019efa0 = uVar1;
  if ((param_1 != 0) && (DAT_0019f384 != '\x01')) {
    FUN_00116add(DAT_0019efc0 - (DAT_0019f370 + DAT_0019efc8));
  }
  DAT_0019f380 = 0;
  return;
}




// Function: write_recent_blocks @ 0x16e0f

void write_recent_blocks(long param_1,ulong param_2)

{
  undefined8 *puVar1;
  ulong uVar2;
  long lVar3;
  undefined8 *puVar4;
  byte bVar5;
  ulong local_10;
  
  bVar5 = 0;
  for (local_10 = 0; local_10 < param_2; local_10 = local_10 + 1) {
    puVar4 = (undefined8 *)(local_10 * 0x200 + param_1);
    puVar1 = (undefined8 *)((long)DAT_0019f380 * 0x200 + DAT_0019f378);
    DAT_0019f380 = DAT_0019f380 + 1;
    *puVar1 = *puVar4;
    puVar1[0x3f] = puVar4[0x3f];
    lVar3 = (long)puVar1 - (long)((ulong)(puVar1 + 1) & 0xfffffffffffffff8);
    puVar4 = (undefined8 *)((long)puVar4 - lVar3);
    puVar1 = (undefined8 *)((ulong)(puVar1 + 1) & 0xfffffffffffffff8);
    for (uVar2 = (ulong)((int)lVar3 + 0x200U >> 3); uVar2 != 0; uVar2 = uVar2 - 1) {
      *puVar1 = *puVar4;
      puVar4 = puVar4 + (ulong)bVar5 * -2 + 1;
      puVar1 = puVar1 + (ulong)bVar5 * -2 + 1;
    }
    if (DAT_0019f380 == DAT_0019f868) {
      FUN_00116d4a(1);
    }
  }
  return;
}




// Function: write_recent_bytes @ 0x16ee1

void write_recent_bytes(long param_1,ulong param_2)

{
  ulong uVar1;
  size_t __n;
  
  uVar1 = param_2 >> 9;
  __n = param_2 + uVar1 * -0x200;
  FUN_00116e0f(param_1,uVar1);
  memcpy((void *)((long)DAT_0019f380 * 0x200 + DAT_0019f378),(void *)(uVar1 * 0x200 + param_1),__n);
  if (__n < 0x200) {
    memset((void *)((long)DAT_0019f380 * 0x200 + DAT_0019f378 + __n),0,0x200 - __n);
  }
  DAT_0019f380 = DAT_0019f380 + 1;
  if (DAT_0019f380 == DAT_0019f868) {
    FUN_00116d4a(1);
  }
  return;
}




// Function: flush_file @ 0x16fd5

void flush_file(void)

{
  undefined8 local_10;
  
  FUN_0010bba7(DAT_0019f460);
  local_10 = DAT_0019faa8 + 0x1ff;
  if (DAT_0019faa8 + 0x1ff < 0) {
    local_10 = DAT_0019faa8 + 0x3fe;
  }
  local_10 = local_10 >> 9;
  while (DAT_0019efa8 - DAT_0019efb0 >> 9 <= local_10) {
    local_10 = local_10 - (DAT_0019efa8 - DAT_0019efb0 >> 9);
    FUN_0010cac8();
  }
  DAT_0019efb0 = local_10 * 0x200 + DAT_0019efb0;
  return;
}




// Function: delete_archive_members @ 0x17069

void delete_archive_members(void)

{
  int iVar1;
  uint uVar2;
  undefined4 uVar3;
  long lVar4;
  undefined8 uVar5;
  ulong uVar6;
  undefined8 *puVar7;
  undefined8 *puVar8;
  bool bVar9;
  byte bVar10;
  uint local_38;
  uint local_34;
  int local_30;
  int local_2c;
  int local_28;
  long local_18;
  
  bVar10 = 0;
  local_38 = 0;
  local_34 = 0;
  FUN_0012d3b1();
  FUN_0010f694(2);
  iVar1 = strcmp((char *)*DAT_0019fbd8,"-");
  DAT_0019f384 = iVar1 == 0;
  do {
    uVar2 = FUN_00125ecf(&DAT_0019f460,&DAT_0019fa20,1);
    switch(uVar2) {
    case 0:
                    /* WARNING: Subroutine does not return */
      abort();
    case 1:
      lVar4 = FUN_0012ed0e(DAT_0019fa28);
      if (lVar4 == 0) {
        FUN_00128891();
      }
      else {
        *(long *)(lVar4 + 0x30) = *(long *)(lVar4 + 0x30) + 1;
        if (DAT_0019f8d0 == 0) {
          bVar9 = *(long *)(lVar4 + 0x30) == 0;
        }
        else {
          bVar9 = *(long *)(lVar4 + 0x30) != DAT_0019f8d0;
        }
        if (!bVar9) goto switchD_0011710d_caseD_2;
        FUN_00128891();
      }
      break;
    case 2:
switchD_0011710d_caseD_2:
      local_38 = uVar2;
      break;
    case 3:
      if (DAT_0019f8bd == '\0') goto switchD_0011710d_caseD_4;
      FUN_0010bba7(DAT_0019f460);
      break;
    case 4:
switchD_0011710d_caseD_4:
      local_38 = 4;
      break;
    case 5:
      FUN_0010bba7(DAT_0019f460);
      if ((local_34 != 5) && (local_34 < 6)) {
        if (local_34 == 4) {
                    /* WARNING: Subroutine does not return */
          abort();
        }
        if (local_34 < 5) {
          if (local_34 == 0) {
            if (DAT_0019fdc8 != (code *)0x0) {
              (*DAT_0019fdc8)();
            }
            uVar5 = gettext("This does not look like a tar archive");
            error(0,0,uVar5);
          }
          else if (2 < local_34 - 1) break;
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar5 = gettext("Skipping to next header");
          error(0,0,uVar5);
          DAT_0019fdd0 = 2;
        }
      }
    }
    local_34 = uVar2;
  } while (local_38 == 0);
  DAT_0019f370 = DAT_0019efc0 + -1;
  DAT_0019f378 = (void *)FUN_00160b5b(DAT_0019f870);
  if ((local_38 == 1) || (local_38 == 2)) {
    DAT_0019efd8 = 0;
    DAT_0019f380 = (int)((long)DAT_0019efb0 - (long)DAT_0019efa0 >> 9);
    if (DAT_0019f380 != 0) {
      memcpy(DAT_0019f378,DAT_0019efa0,(long)(DAT_0019f380 << 9));
    }
    if (local_38 == 1) {
      local_38 = 0;
      FUN_00116fd5();
    }
    while (local_38 != 4) {
      if (DAT_0019efb0 == DAT_0019efa8) {
        FUN_0010cac8();
      }
      uVar3 = FUN_00125ecf(&DAT_0019f460,&DAT_0019fa20,0);
      switch(uVar3) {
      case 0:
      case 2:
                    /* WARNING: Subroutine does not return */
        abort();
      case 1:
        FUN_0011df5f(&DAT_0019fa20);
        lVar4 = FUN_0012ed0e(DAT_0019fa28);
        if (lVar4 != 0) {
          *(long *)(lVar4 + 0x30) = *(long *)(lVar4 + 0x30) + 1;
          if (DAT_0019f8d0 == 0) {
            bVar9 = *(long *)(lVar4 + 0x30) != 0;
          }
          else {
            bVar9 = *(long *)(lVar4 + 0x30) == DAT_0019f8d0;
          }
          if (bVar9) {
            FUN_00116fd5();
            break;
          }
        }
        if (DAT_0019fb90 == 0) {
          FUN_00116e0f(DAT_0019f470,DAT_0019f480);
          FUN_00116e0f(DAT_0019f478,DAT_0019f488);
        }
        else {
          FUN_00116ee1(DAT_0019fb98,DAT_0019fb90);
        }
        puVar8 = DAT_0019f460;
        puVar7 = (undefined8 *)((long)DAT_0019f380 * 0x200 + (long)DAT_0019f378);
        *puVar7 = *DAT_0019f460;
        puVar7[0x3f] = puVar8[0x3f];
        lVar4 = (long)puVar7 - (long)((ulong)(puVar7 + 1) & 0xfffffffffffffff8);
        puVar8 = (undefined8 *)((long)puVar8 - lVar4);
        puVar7 = (undefined8 *)((ulong)(puVar7 + 1) & 0xfffffffffffffff8);
        for (uVar6 = (ulong)((int)lVar4 + 0x200U >> 3); uVar6 != 0; uVar6 = uVar6 - 1) {
          *puVar7 = *puVar8;
          puVar8 = puVar8 + (ulong)bVar10 * -2 + 1;
          puVar7 = puVar7 + (ulong)bVar10 * -2 + 1;
        }
        DAT_0019f380 = DAT_0019f380 + 1;
        local_18 = DAT_0019faa8 + 0x1ff;
        if (DAT_0019faa8 + 0x1ff < 0) {
          local_18 = DAT_0019faa8 + 0x3fe;
        }
        local_18 = local_18 >> 9;
        FUN_0010bba7(DAT_0019f460);
        if (DAT_0019f380 == DAT_0019f868) {
          FUN_00116d4a(1);
        }
        local_30 = (int)((long)DAT_0019efa8 - (long)DAT_0019efb0 >> 9);
        if (local_18 < local_30) {
          local_30 = (int)local_18;
        }
        while (local_18 != 0) {
          if (DAT_0019efb0 == DAT_0019efa8) {
            FUN_0010f662();
            DAT_0019efb0 = DAT_0019efa0;
            local_30 = DAT_0019f868;
            if (local_18 < DAT_0019f868) {
              local_30 = (int)local_18;
            }
          }
          local_2c = local_30;
          if (DAT_0019f868 - DAT_0019f380 < local_30) {
            local_2c = DAT_0019f868 - DAT_0019f380;
          }
          if (local_2c == 0) {
                    /* WARNING: Subroutine does not return */
            abort();
          }
          memcpy((void *)((long)DAT_0019f380 * 0x200 + (long)DAT_0019f378),DAT_0019efb0,
                 (long)(local_2c << 9));
          DAT_0019f380 = local_2c + DAT_0019f380;
          DAT_0019efb0 = (void *)((long)local_2c * 0x200 + (long)DAT_0019efb0);
          local_18 = local_18 - local_2c;
          local_30 = local_30 - local_2c;
          if (DAT_0019f380 == DAT_0019f868) {
            FUN_00116d4a(1);
          }
        }
        break;
      case 3:
        if (DAT_0019f8bd == '\0') {
          local_38 = 4;
        }
        else {
          FUN_0010bba7(DAT_0019f460);
        }
        break;
      case 4:
        local_38 = 4;
        break;
      case 5:
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar5 = gettext("Deleting non-header from archive");
        error(0,0,uVar5);
        DAT_0019fdd0 = 2;
        FUN_0010bba7(DAT_0019f460);
        break;
      default:
                    /* WARNING: Subroutine does not return */
        abort();
      }
      FUN_00139d1c(&DAT_0019fa20);
    }
    local_28 = 0;
    do {
      iVar1 = DAT_0019f868 - DAT_0019f380;
      memset((void *)((long)DAT_0019f378 + (long)DAT_0019f380 * 0x200),0,(long)(iVar1 * 0x200));
      local_28 = local_28 + iVar1;
      FUN_00116d4a(local_28 < 2);
    } while (local_28 < 2);
    if (((DAT_0019f384 != '\x01') && (DAT_0019f9cc < 0x40000000)) &&
       (iVar1 = FUN_00132f52(DAT_0019f9cc), iVar1 != 0)) {
      FUN_0013ea7e(*DAT_0019fbd8);
    }
  }
  free(DAT_0019f378);
  FUN_0010d046();
  FUN_0012dbba();
  return;
}




// Function: fatal_exit @ 0x177f4

void fatal_exit(void)

{
  undefined8 uVar1;
  
  if (DAT_0019f388 != (code *)0x0) {
    (*DAT_0019f388)();
  }
  uVar1 = gettext("Error is not recoverable: exiting now");
  error(2,0,uVar1);
                    /* WARNING: Subroutine does not return */
  abort();
}




// Function: xalloc_die @ 0x1783c

void xalloc_die(void)

{
  undefined8 uVar1;
  
  uVar1 = gettext("memory exhausted");
  error(0,0,&DAT_0018121f,uVar1);
                    /* WARNING: Subroutine does not return */
  FUN_001177f4();
}




// Function: excfile_add @ 0x17879

void excfile_add(char *param_1,undefined4 param_2)

{
  undefined8 *puVar1;
  size_t sVar2;
  undefined8 *puVar3;
  
  sVar2 = strlen(param_1);
  puVar3 = (undefined8 *)FUN_00160b5b(sVar2 + 0x10);
  *puVar3 = 0;
  *(undefined4 *)(puVar3 + 1) = param_2;
  strcpy((char *)((long)puVar3 + 0xc),param_1);
  puVar1 = puVar3;
  if (DAT_0019f398 != (undefined8 *)0x0) {
    *DAT_0019f398 = puVar3;
    puVar1 = DAT_0019f390;
  }
  DAT_0019f390 = puVar1;
  DAT_0019f398 = puVar3;
  return;
}




// Function: info_attach_exclist @ 0x17909

void info_attach_exclist(long param_1)

{
  undefined8 *puVar1;
  int iVar2;
  undefined4 uVar3;
  FILE *__stream;
  undefined8 uVar4;
  int *piVar5;
  long lVar6;
  undefined8 uVar7;
  undefined8 *puVar8;
  undefined8 *local_50;
  undefined8 *local_48;
  undefined8 *local_40;
  
  local_48 = (undefined8 *)0x0;
  local_40 = (undefined8 *)0x0;
  if (*(long *)(param_1 + 0x1b0) == 0) {
    for (local_50 = DAT_0019f390; local_50 != (undefined8 *)0x0; local_50 = (undefined8 *)*local_50)
    {
      iVar2 = DAT_0019c268;
      if (param_1 != 0) {
        iVar2 = *(int *)(param_1 + 0x1a8);
      }
      iVar2 = faccessat(iVar2,(char *)((long)local_50 + 0xc),0,0);
      puVar1 = local_48;
      puVar8 = local_40;
      if (iVar2 == 0) {
        iVar2 = FUN_001157f9(param_1,(long)local_50 + 0xc,0);
        if (iVar2 == -1) {
          FUN_0013e38b((long)local_50 + 0xc);
        }
        else {
          __stream = fdopen(iVar2,"r");
          if (__stream == (FILE *)0x0) {
            if (DAT_0019fdc8 != (code *)0x0) {
              (*DAT_0019fdc8)();
            }
            uVar4 = gettext("%s: fdopen failed");
            piVar5 = __errno_location();
            error(0,*piVar5,uVar4,(long)local_50 + 0xc);
            DAT_0019fdd0 = 2;
            close(iVar2);
          }
          else {
            uVar4 = FUN_0014e53e();
            lVar6 = FUN_00118242((long)local_50 + 0xc);
            if (*(long *)(lVar6 + 0x18) != 0) {
              uVar7 = (**(code **)(lVar6 + 0x18))(*(undefined8 *)(lVar6 + 0x20));
              *(undefined8 *)(lVar6 + 0x20) = uVar7;
            }
            iVar2 = FUN_0014f189(*(undefined8 *)(lVar6 + 0x10),uVar4,__stream,0x50000001,10,
                                 *(undefined8 *)(lVar6 + 0x20));
            if (iVar2 != 0) {
              piVar5 = __errno_location();
              iVar2 = *piVar5;
              if (DAT_0019fdc8 != (code *)0x0) {
                (*DAT_0019fdc8)();
              }
              uVar4 = FUN_0015e548((long)local_50 + 0xc);
              error(0,iVar2,&DAT_00181236,uVar4);
                    /* WARNING: Subroutine does not return */
              FUN_001177f4();
            }
            fclose(__stream);
            puVar8 = (undefined8 *)FUN_00160b5b(0x20);
            puVar8[3] = uVar4;
            if (*(int *)(local_50 + 1) == 0) {
              uVar3 = *(undefined4 *)(local_50 + 1);
            }
            else {
              uVar3 = *(undefined4 *)(lVar6 + 8);
            }
            *(undefined4 *)(puVar8 + 2) = uVar3;
            puVar8[1] = local_40;
            *puVar8 = 0;
            puVar1 = puVar8;
            if (local_40 != (undefined8 *)0x0) {
              *local_40 = puVar8;
              puVar1 = local_48;
            }
          }
        }
      }
      local_40 = puVar8;
      local_48 = puVar1;
    }
    *(undefined8 **)(param_1 + 0x1b0) = local_48;
  }
  return;
}




// Function: info_free_exclist @ 0x17bda

void info_free_exclist(long param_1)

{
  undefined8 *puVar1;
  undefined8 *local_18;
  
  local_18 = *(undefined8 **)(param_1 + 0x1b0);
  while (local_18 != (undefined8 *)0x0) {
    puVar1 = (undefined8 *)*local_18;
    FUN_0014e904(local_18[3]);
    free(local_18);
    local_18 = puVar1;
  }
  *(undefined8 *)(param_1 + 0x1b0) = 0;
  return;
}




// Function: excluded_name @ 0x17c43

char excluded_name(char *param_1,long param_2)

{
  char *pcVar1;
  char cVar2;
  long local_38;
  char local_25;
  uint local_24;
  undefined8 *local_20;
  char *local_18;
  void *local_10;
  
  local_18 = (char *)0x0;
  local_10 = (void *)0x0;
  local_24 = 0;
  cVar2 = FUN_0014ed6a(DAT_0019f8a8,param_1);
  if (cVar2 == '\0') {
    if (param_2 == 0) {
      local_25 = '\0';
    }
    else {
      local_25 = '\0';
      local_38 = param_2;
      while ((local_38 != 0 && (local_25 != '\x01'))) {
        for (local_20 = *(undefined8 **)(local_38 + 0x1b0); local_20 != (undefined8 *)0x0;
            local_20 = (undefined8 *)*local_20) {
          if ((*(uint *)(local_20 + 2) & local_24) == 0) {
            local_25 = FUN_0014ed6a(local_20[3],param_1);
            if (local_25 != '\0') break;
            pcVar1 = param_1;
            if (local_18 == (char *)0x0) {
              while ((local_18 = pcVar1, *local_18 == '.' && (local_18[1] == '/'))) {
                pcVar1 = local_18 + 2;
              }
            }
            local_25 = FUN_0014ed6a(local_20[3],local_18);
            if (local_25 != '\0') break;
            if (local_10 == (void *)0x0) {
              local_10 = (void *)FUN_0014e13e(param_1);
            }
            local_25 = FUN_0014ed6a(local_20[3],local_10);
            if (local_25 != '\0') break;
          }
        }
        local_38 = *(long *)(local_38 + 0x198);
        local_24 = 2;
      }
      free(local_10);
    }
  }
  else {
    local_25 = '\x01';
  }
  return local_25;
}




// Function: cvs_addfn @ 0x17ddd

void cvs_addfn(undefined8 param_1,undefined8 param_2,undefined4 param_3)

{
  int iVar1;
  long in_FS_OFFSET;
  ulong local_100;
  ulong local_f8;
  long local_f0;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = FUN_001461a6(param_2,&local_f8,0x844);
  if (iVar1 == 0) {
    for (local_100 = 0; local_100 < local_f8; local_100 = local_100 + 1) {
      FUN_0014ee5d(param_1,*(undefined8 *)(local_100 * 8 + local_f0),param_3);
    }
    FUN_00146399(&local_f8);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: git_addfn @ 0x17eb5

void git_addfn(undefined8 param_1,char *param_2,undefined4 param_3)

{
  ushort **ppuVar1;
  char *local_18;
  
  local_18 = param_2;
  while( true ) {
    ppuVar1 = __ctype_b_loc();
    if (((*ppuVar1)[*local_18] & 0x2000) == 0) break;
    local_18 = local_18 + 1;
  }
  if ((*local_18 != '\0') && (*local_18 != '#')) {
    if ((*local_18 == '\\') && (local_18[1] == '#')) {
      local_18 = local_18 + 1;
    }
    FUN_0014ee5d(param_1,local_18,param_3);
  }
  return;
}




// Function: bzr_addfn @ 0x17f4f

void bzr_addfn(undefined8 param_1,char *param_2,uint param_3)

{
  int iVar1;
  ushort **ppuVar2;
  uint local_1c;
  char *local_18;
  
  local_18 = param_2;
  while( true ) {
    ppuVar2 = __ctype_b_loc();
    if (((*ppuVar2)[*local_18] & 0x2000) == 0) break;
    local_18 = local_18 + 1;
  }
  if ((*local_18 != '\0') && (*local_18 != '#')) {
    local_1c = param_3;
    if (*local_18 == '!') {
      if (local_18[1] == '!') {
        local_18 = local_18 + 2;
      }
      else {
        local_1c = param_3 | 0x20000000;
        local_18 = local_18 + 1;
      }
    }
    iVar1 = strncmp(local_18,"RE:",3);
    if (iVar1 == 0) {
      local_18 = local_18 + 3;
      local_1c = local_1c & 0xefffffff | 0x8000000;
    }
    FUN_0014ee5d(param_1,local_18,local_1c);
  }
  return;
}




// Function: hg_initfn @ 0x18025

undefined4 * hg_initfn(undefined4 *param_1)

{
  if (param_1 == (undefined4 *)0x0) {
    param_1 = &DAT_0019f3a0;
  }
  *param_1 = 0x8000000;
  return param_1;
}




// Function: hg_addfn @ 0x18059

void hg_addfn(undefined8 param_1,char *param_2,uint param_3,uint *param_4)

{
  int iVar1;
  ushort **ppuVar2;
  size_t sVar3;
  char *__dest;
  uint local_3c;
  char *local_38;
  
  local_38 = param_2;
  while (ppuVar2 = __ctype_b_loc(), ((*ppuVar2)[*local_38] & 0x2000) != 0) {
    local_38 = local_38 + 1;
  }
  if ((*local_38 != '\0') && (*local_38 != '#')) {
    iVar1 = strncmp(local_38,"syntax:",7);
    if (iVar1 == 0) {
      local_38 = local_38 + 7;
      while (ppuVar2 = __ctype_b_loc(), ((*ppuVar2)[*local_38] & 0x2000) != 0) {
        local_38 = local_38 + 1;
      }
      iVar1 = strcmp(local_38,"regexp");
      if (iVar1 == 0) {
        *param_4 = 0x8000000;
      }
      else {
        iVar1 = strcmp(local_38,"glob");
        if (iVar1 == 0) {
          *param_4 = 0x10000000;
        }
      }
    }
    else {
      sVar3 = strlen(local_38);
      local_3c = param_3;
      if (local_38[sVar3 - 1] == '/') {
        __dest = (char *)FUN_00160b5b(sVar3);
        memcpy(__dest,local_38,sVar3 - 1);
        __dest[sVar3 - 1] = '\0';
        FUN_0014e34b(param_1,__dest);
        local_3c = param_3 | 0x4000008;
        local_38 = __dest;
      }
      if (*param_4 == 0x8000000) {
        local_3c = local_3c & 0xefffffff;
      }
      else {
        local_3c = local_3c & 0xf7ffffff;
      }
      FUN_0014ee5d(param_1,local_38,local_3c | *param_4);
    }
  }
  return;
}




// Function: get_vcs_ignore_file @ 0x18242

undefined ** get_vcs_ignore_file(char *param_1)

{
  int iVar1;
  undefined **local_10;
  
  local_10 = &PTR_s__cvsignore_0019c160;
  while ((*local_10 != (undefined *)0x0 && (iVar1 = strcmp(*local_10,param_1), iVar1 != 0))) {
    local_10 = local_10 + 5;
  }
  return local_10;
}




// Function: exclude_vcs_ignores @ 0x18293

void exclude_vcs_ignores(void)

{
  undefined **local_10;
  
  for (local_10 = &PTR_s__cvsignore_0019c160; *local_10 != (undefined *)0x0; local_10 = local_10 + 5
      ) {
    FUN_00117879(*local_10,0);
  }
  return;
}




// Function: implemented @ 0x182d5

undefined4 implemented(int param_1)

{
  undefined4 uVar1;
  
  if ((param_1 == 0x26) || (param_1 == 0x5f)) {
    uVar1 = 0;
  }
  else {
    uVar1 = 1;
  }
  return uVar1;
}




// Function: extr_init @ 0x182fd

void extr_init(void)

{
  __uid_t _Var1;
  
  _Var1 = geteuid();
  DAT_0019f3a8 = _Var1 == 0;
  DAT_0019f960 = DAT_0019f960 + (uint)(byte)DAT_0019f3a8;
  DAT_0019f95c = DAT_0019f95c + (uint)(byte)DAT_0019f3a8;
  DAT_0019f3ac = umask(0);
  if (DAT_0019f960 < 1) {
    umask(DAT_0019f3ac);
    DAT_0019f3b0 = DAT_0019f3ac;
  }
  else {
    DAT_0019f3b0 = 0;
  }
  return;
}




// Function: fd_i_chmod @ 0x18387

int fd_i_chmod(int param_1,char *param_2,__mode_t param_3,int param_4)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  
  if (-1 < param_1) {
    iVar2 = fchmod(param_1,param_3);
    if (iVar2 == 0) {
      return 0;
    }
    piVar3 = __errno_location();
    cVar1 = FUN_001182d5(*piVar3);
    if (cVar1 != '\0') {
      return iVar2;
    }
  }
  iVar2 = fchmodat(DAT_0019c268,param_2,param_3,param_4);
  return iVar2;
}




// Function: fd_chmod @ 0x183ee

undefined8 fd_chmod(undefined4 param_1,undefined8 param_2,uint param_3,int param_4,int param_5)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  undefined8 uVar4;
  int local_c;
  
  iVar2 = FUN_00118387(param_1,param_2,param_3,param_4);
  if (iVar2 == 0) {
    local_c = 0;
  }
  else {
    piVar3 = __errno_location();
    local_c = *piVar3;
  }
  if (((local_c == 1) && ((param_3 & 0x800) != 0)) && (iVar2 = FUN_0015c835(), iVar2 == 0)) {
    iVar2 = FUN_00118387(param_1,param_2,param_3,param_4);
    if (iVar2 == 0) {
      local_c = 0;
    }
    else {
      piVar3 = __errno_location();
      local_c = *piVar3;
    }
    FUN_0015c826();
  }
  if (((param_4 != 0) && (param_5 != 0x32)) && (cVar1 = FUN_001182d5(local_c), cVar1 != '\x01')) {
    iVar2 = FUN_00118387(param_1,param_2,param_3,0);
    if (iVar2 == 0) {
      local_c = 0;
    }
    else {
      piVar3 = __errno_location();
      local_c = *piVar3;
    }
  }
  if ((local_c == 0) || ((param_5 == 0x32 && (cVar1 = FUN_001182d5(local_c), cVar1 == '\0')))) {
    uVar4 = 0;
  }
  else {
    piVar3 = __errno_location();
    *piVar3 = local_c;
    uVar4 = 0xffffffff;
  }
  return uVar4;
}




// Function: fd_chown @ 0x184f7

int fd_chown(int param_1,char *param_2,__uid_t param_3,__gid_t param_4,int param_5)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  
  if (-1 < param_1) {
    iVar2 = fchown(param_1,param_3,param_4);
    if (iVar2 == 0) {
      return 0;
    }
    piVar3 = __errno_location();
    cVar1 = FUN_001182d5(*piVar3);
    if (cVar1 != '\0') {
      return iVar2;
    }
  }
  iVar2 = fchownat(DAT_0019c268,param_2,param_3,param_4,param_5);
  return iVar2;
}




// Function: fd_stat @ 0x1856b

void fd_stat(int param_1,char *param_2,stat *param_3,int param_4)

{
  if (param_1 < 0) {
    fstatat(DAT_0019c268,param_2,param_3,param_4);
  }
  else {
    fstat(param_1,param_3);
  }
  return;
}




// Function: set_mode @ 0x185b8

void set_mode(undefined8 param_1,uint param_2,uint param_3,undefined4 param_4,uint param_5,
                 uint param_6,char param_7,undefined4 param_8)

{
  int iVar1;
  uint uVar2;
  long in_FS_OFFSET;
  uint local_c0;
  undefined1 local_a8 [24];
  uint local_90;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (((~param_6 | param_5 ^ param_2) & param_3) != 0) {
    local_c0 = param_5;
    if ((~(param_3 | param_6) & 0xfff) != 0) {
      iVar1 = FUN_0011856b(param_4,param_1,local_a8,param_8);
      if (iVar1 != 0) {
        FUN_0013ea03(param_1);
        goto LAB_00118703;
      }
      local_c0 = local_90;
    }
    uVar2 = param_2 & param_3 | ~param_3 & local_c0 & 0xfff;
    if ((local_c0 & 0xfff) != uVar2) {
      iVar1 = FUN_001183ee(param_4,param_1,uVar2,param_8,(int)param_7);
      if (iVar1 != 0) {
        FUN_0013e0b2(param_1,uVar2);
      }
    }
  }
LAB_00118703:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: check_time @ 0x18719

void check_time(undefined8 param_1,long param_2,long param_3)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  long local_68;
  long local_60;
  long local_58;
  long local_50;
  undefined1 local_48 [40];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 < 0) {
    if ((DAT_0019ec84 & 0x8000) != 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar2 = FUN_00127abc(param_2,param_3,1);
      uVar3 = gettext("%s: implausibly old time stamp %s");
      error(0,0,uVar3,param_1,uVar2);
    }
  }
  else {
    iVar1 = FUN_0015ff90(DAT_0019f9f0,DAT_0019f9f8,param_2,param_3);
    if (iVar1 < 0) {
      FUN_001521fa(&local_68);
      iVar1 = FUN_0015ff90(local_68,local_60,param_2,param_3);
      if (iVar1 < 0) {
        local_58 = param_2 - local_68;
        local_50 = param_3 - local_60;
        if (local_50 < 0) {
          local_50 = local_50 + 1000000000;
          local_58 = local_58 + -1;
        }
        if ((DAT_0019ec84 & 0x8000) != 0) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar2 = FUN_00129f2e(local_58,local_50,local_48);
          uVar3 = FUN_00127abc(param_2,param_3,1);
          uVar4 = gettext("%s: time stamp %s is %s s in the future");
          error(0,0,uVar4,param_1,uVar3,uVar2);
        }
      }
    }
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: set_stat @ 0x18913

void set_stat(undefined8 param_1,long param_2,undefined4 param_3,uint param_4,uint param_5,
                 char param_6,char param_7,undefined4 param_8)

{
  undefined4 uVar1;
  undefined4 uVar2;
  char cVar3;
  int iVar4;
  int *piVar5;
  undefined8 uVar6;
  long in_FS_OFFSET;
  uint local_64;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 local_28;
  undefined8 local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if ((DAT_0019f99a != '\x01') && (param_7 != '\x01')) {
    if (DAT_0019f8be == '\0') {
      local_30 = 0x3ffffffe;
    }
    else {
      local_30 = *(undefined8 *)(param_2 + 0xf0);
      local_38 = *(undefined8 *)(param_2 + 0xe8);
    }
    local_20 = *(undefined8 *)(param_2 + 0x100);
    local_28 = *(undefined8 *)(param_2 + 0xf8);
    iVar4 = FUN_0014f4ec(param_3,DAT_0019c268,param_1,&local_38,param_8);
    if (iVar4 == 0) {
      if (DAT_0019f8be != '\0') {
        FUN_00118719(param_1,local_38,local_30);
      }
      FUN_00118719(param_1,local_28,local_20);
    }
    else {
      if (param_6 == '2') {
        piVar5 = __errno_location();
        cVar3 = FUN_001182d5(*piVar5);
        if (cVar3 == '\0') goto LAB_00118a37;
      }
      FUN_0013ead0(param_1);
    }
  }
LAB_00118a37:
  local_64 = param_5;
  if ((0 < DAT_0019f95c) && (param_7 != '\x01')) {
    uVar1 = *(undefined4 *)(param_2 + 0x74);
    uVar2 = *(undefined4 *)(param_2 + 0x78);
    iVar4 = FUN_001184f7(param_3,param_1,uVar1,uVar2,param_8);
    if (iVar4 == 0) {
      if (((~param_5 | param_4) & 0x49) != 0) {
        local_64 = param_5 & ~(param_4 & 0xc00);
      }
    }
    else {
      if (param_6 == '2') {
        piVar5 = __errno_location();
        cVar3 = FUN_001182d5(*piVar5);
        if (cVar3 == '\0') goto LAB_00118acc;
      }
      FUN_0013e169(param_1,uVar1,uVar2);
    }
  }
LAB_00118acc:
  if ((DAT_0019f960 < 1) || (param_7 == '\x01')) {
    uVar6 = 0x1ff;
  }
  else {
    uVar6 = 0xfff;
  }
  FUN_001185b8(param_1,*(uint *)(param_2 + 0x70) & ~DAT_0019f3b0,uVar6,param_3,param_4,local_64,
               (int)param_6,param_8);
  FUN_0013d8a1(param_2,param_1,(int)param_6,1);
  FUN_0013d003(param_2,param_1,(int)param_6);
  FUN_0013d609(param_2,param_1,(int)param_6);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: find_direct_ancestor @ 0x18b89

undefined8 * find_direct_ancestor(char *param_1)

{
  int iVar1;
  char *pcVar2;
  undefined8 *local_10;
  
  local_10 = DAT_0019f3b8;
  while ((local_10 != (undefined8 *)0x0 &&
         ((((*(char *)(local_10 + 0xb) == '\x01' ||
            (iVar1 = strncmp(param_1,(char *)local_10[0x14],local_10[0x13]), iVar1 != 0)) ||
           (param_1[local_10[0x13]] != '/')) ||
          (pcVar2 = (char *)FUN_0014df20(param_1), pcVar2 != param_1 + local_10[0x13] + 1))))) {
    local_10 = (undefined8 *)*local_10;
  }
  return local_10;
}




// Function: mark_after_links @ 0x18c40

void mark_after_links(long *param_1)

{
  int iVar1;
  long in_FS_OFFSET;
  long *local_b0;
  long local_a8;
  long local_a0;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_b0 = param_1;
  do {
    *(undefined1 *)(local_b0 + 0xb) = 1;
    iVar1 = FUN_0012a9b8(local_b0[0x14],&local_a8);
    if (iVar1 == 0) {
      local_b0[1] = local_a8;
      local_b0[2] = local_a0;
    }
    else {
      FUN_0013ea03(local_b0[0x14]);
    }
    local_b0 = (long *)*local_b0;
  } while ((local_b0 != (long *)0x0) && (*(char *)(local_b0 + 0xb) != '\x01'));
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: delay_set_stat @ 0x18d26

void delay_set_stat(char *param_1,long param_2,undefined4 param_3,undefined4 param_4,
                 undefined4 param_5,undefined4 param_6)

{
  char cVar1;
  int iVar2;
  size_t sVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  undefined8 *local_b8;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  sVar3 = strlen(param_1);
  local_b8 = DAT_0019f3b8;
  while ((local_b8 != (undefined8 *)0x0 &&
         (iVar2 = strcmp((char *)local_b8[0x14],param_1), iVar2 != 0))) {
    local_b8 = (undefined8 *)*local_b8;
  }
  if (local_b8 == (undefined8 *)0x0) {
    local_b8 = (undefined8 *)FUN_00160b5b(0xa8);
    *local_b8 = DAT_0019f3b8;
    DAT_0019f3b8 = local_b8;
    local_b8[0x13] = sVar3;
    uVar4 = FUN_00160cc6(param_1);
    local_b8[0x14] = uVar4;
    *(undefined1 *)(local_b8 + 0xb) = 0;
    if (param_2 != 0) {
      local_b8[1] = *(undefined8 *)(param_2 + 0x58);
      local_b8[2] = *(undefined8 *)(param_2 + 0x60);
    }
  }
  else if (*(char *)(local_b8 + 10) != '\0') {
    iVar2 = fstatat(DAT_0019c268,(char *)local_b8[0x14],&local_a8,*(int *)((long)local_b8 + 0x54));
    if (iVar2 == 0) {
      local_b8[1] = local_a8.st_dev;
      local_b8[2] = local_a8.st_ino;
    }
    else {
      FUN_0013ea03(local_b8[0x14]);
    }
  }
  *(undefined4 *)(local_b8 + 3) = param_5;
  if (param_2 != 0) {
    *(undefined4 *)((long)local_b8 + 0x1c) = *(undefined4 *)(param_2 + 0x74);
    *(undefined4 *)(local_b8 + 4) = *(undefined4 *)(param_2 + 0x78);
    uVar4 = *(undefined8 *)(param_2 + 0xf0);
    local_b8[5] = *(undefined8 *)(param_2 + 0xe8);
    local_b8[6] = uVar4;
    uVar4 = *(undefined8 *)(param_2 + 0x100);
    local_b8[7] = *(undefined8 *)(param_2 + 0xf8);
    local_b8[8] = uVar4;
  }
  *(undefined4 *)(local_b8 + 9) = param_3;
  *(undefined4 *)((long)local_b8 + 0x4c) = param_4;
  *(bool *)(local_b8 + 10) = param_2 == 0;
  *(undefined4 *)((long)local_b8 + 0x54) = param_6;
  *(undefined4 *)((long)local_b8 + 0x5c) = DAT_0019f600;
  local_b8[0xc] = 0;
  if ((param_2 == 0) ||
     (FUN_00129547(local_b8 + 0xc,*(undefined8 *)(param_2 + 0x30)), *(long *)(param_2 + 0x38) == 0))
  {
    local_b8[0xd] = 0;
    local_b8[0xe] = 0;
  }
  else {
    uVar4 = FUN_00160c8e(*(undefined8 *)(param_2 + 0x38),*(long *)(param_2 + 0x40) + 1);
    local_b8[0xd] = uVar4;
    local_b8[0xe] = *(undefined8 *)(param_2 + 0x40);
  }
  if ((param_2 == 0) || (*(long *)(param_2 + 0x48) == 0)) {
    local_b8[0xf] = 0;
    local_b8[0x10] = 0;
  }
  else {
    uVar4 = FUN_00160c8e(*(undefined8 *)(param_2 + 0x48),*(long *)(param_2 + 0x50) + 1);
    local_b8[0xf] = uVar4;
    local_b8[0x10] = *(undefined8 *)(param_2 + 0x50);
  }
  if (param_2 == 0) {
    local_b8[0x12] = 0;
    local_b8[0x11] = 0;
  }
  else {
    FUN_0011d854(param_2,local_b8 + 0x12,local_b8 + 0x11);
  }
  cVar1 = FUN_0012a1fe(param_1);
  if (cVar1 != '\0') {
    FUN_00118c40(local_b8);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: repair_delayed_set_stat @ 0x191c5

void repair_delayed_set_stat(undefined8 param_1,__dev_t *param_2)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined8 *local_c0;
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_c0 = DAT_0019f3b8;
  do {
    if (local_c0 == (undefined8 *)0x0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar2 = FUN_0015e548(param_1);
      uVar3 = gettext("%s: Unexpected inconsistency when making directory");
      error(0,0,uVar3,uVar2);
      DAT_0019fdd0 = 2;
LAB_001193b7:
      if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return;
    }
    iVar1 = fstatat(DAT_0019c268,(char *)local_c0[0x14],&local_b8,*(int *)((long)local_c0 + 0x54));
    if (iVar1 != 0) {
      FUN_0013ea03(local_c0[0x14]);
      goto LAB_001193b7;
    }
    if ((local_b8.st_dev == *param_2) && (local_b8.st_ino == param_2[1])) {
      local_c0[1] = DAT_0019fa78;
      local_c0[2] = DAT_0019fa80;
      *(undefined4 *)(local_c0 + 3) = DAT_0019fa90;
      *(undefined4 *)((long)local_c0 + 0x1c) = DAT_0019fa94;
      *(undefined4 *)(local_c0 + 4) = DAT_0019fa98;
      uVar2 = DAT_0019fb10;
      local_c0[5] = DAT_0019fb08;
      local_c0[6] = uVar2;
      uVar2 = DAT_0019fb20;
      local_c0[7] = DAT_0019fb18;
      local_c0[8] = uVar2;
      *(__mode_t *)(local_c0 + 9) = local_b8.st_mode;
      *(undefined4 *)((long)local_c0 + 0x4c) = 0xffffffff;
      *(undefined1 *)(local_c0 + 10) = 0;
      goto LAB_001193b7;
    }
    local_c0 = (undefined8 *)*local_c0;
  } while( true );
}




// Function: free_delayed_set_stat @ 0x193d1

void free_delayed_set_stat(void *param_1)

{
  free(*(void **)((long)param_1 + 0xa0));
  FUN_0011d55d(*(undefined8 *)((long)param_1 + 0x90),*(undefined8 *)((long)param_1 + 0x88));
  free(*(void **)((long)param_1 + 0x60));
  free(*(void **)((long)param_1 + 0x68));
  free(*(void **)((long)param_1 + 0x78));
  free(param_1);
  return;
}




// Function: remove_delayed_set_stat @ 0x19454

void remove_delayed_set_stat(char *param_1)

{
  undefined8 *puVar1;
  int iVar2;
  undefined8 *local_20;
  undefined8 *local_18;
  
  local_18 = (undefined8 *)0x0;
  local_20 = DAT_0019f3b8;
  while( true ) {
    if (local_20 == (undefined8 *)0x0) {
      return;
    }
    puVar1 = (undefined8 *)*local_20;
    if ((*(int *)((long)local_20 + 0x5c) == DAT_0019f600) &&
       (iVar2 = strcmp((char *)local_20[0x14],param_1), iVar2 == 0)) break;
    local_18 = local_20;
    local_20 = puVar1;
  }
  FUN_001193d1(local_20);
  if (local_18 == (undefined8 *)0x0) {
    DAT_0019f3b8 = puVar1;
    return;
  }
  *local_18 = puVar1;
  return;
}




// Function: fixup_delayed_set_stat @ 0x194f9

void fixup_delayed_set_stat(char *param_1,char *param_2)

{
  int iVar1;
  undefined8 uVar2;
  size_t sVar3;
  undefined8 *local_10;
  
  local_10 = DAT_0019f3b8;
  while( true ) {
    if (local_10 == (undefined8 *)0x0) {
      return;
    }
    if ((*(int *)((long)local_10 + 0x5c) == DAT_0019f600) &&
       (iVar1 = strcmp((char *)local_10[0x14],param_1), iVar1 == 0)) break;
    local_10 = (undefined8 *)*local_10;
  }
  free((void *)local_10[0x14]);
  uVar2 = FUN_00160cc6(param_2);
  local_10[0x14] = uVar2;
  sVar3 = strlen(param_2);
  local_10[0x13] = sVar3;
  return;
}




// Function: make_directories @ 0x195aa

int make_directories(char *param_1,undefined1 *param_2)

{
  uint uVar1;
  uint uVar2;
  int iVar3;
  int *piVar4;
  long in_FS_OFFSET;
  int local_c8;
  char *local_b8;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  for (local_b8 = param_1; *local_b8 != '\0'; local_b8 = local_b8 + 1) {
    if ((((*local_b8 == '/') && (local_b8 != param_1)) && (local_b8[-1] != '/')) &&
       ((local_b8[-1] != '.' ||
        (((local_b8 != param_1 + 1 && (local_b8[-2] != '/')) &&
         ((local_b8[-2] != '.' || ((local_b8 != param_1 + 2 && (local_b8[-3] != '/')))))))))) {
      *local_b8 = '\0';
      uVar1 = ~DAT_0019f3ac & 0x1ff;
      if (DAT_0019f3a8 == '\0') {
        uVar2 = 0xc0;
      }
      else {
        uVar2 = 0;
      }
      iVar3 = mkdirat(DAT_0019c268,param_1,uVar2 | uVar1);
      if (iVar3 == 0) {
        FUN_00118d26(param_1,0,~DAT_0019f3b0 & (uVar2 | uVar1),0x1ff,uVar1,0x100);
        FUN_001286af(param_1,(int)local_b8 - (int)param_1,uVar1);
        *param_2 = 1;
        local_c8 = 0;
      }
      else {
        piVar4 = __errno_location();
        if (*piVar4 == 0x11) {
          local_c8 = 0;
        }
        else {
          piVar4 = __errno_location();
          iVar3 = *piVar4;
          local_c8 = fstatat(DAT_0019c268,param_1,&local_a8,0);
          if (local_c8 != 0) {
            piVar4 = __errno_location();
            *piVar4 = iVar3;
            FUN_0013e310(param_1);
          }
        }
      }
      *local_b8 = '/';
      if (local_c8 != 0) goto LAB_0011983b;
    }
  }
  local_c8 = 0;
LAB_0011983b:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_c8;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: file_newer_p @ 0x19851

undefined4 file_newer_p(undefined8 param_1,undefined1 *param_2,long param_3)

{
  int iVar1;
  undefined4 uVar2;
  int *piVar3;
  long in_FS_OFFSET;
  undefined1 auVar4 [16];
  undefined1 *local_b8;
  undefined1 local_a8 [152];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_b8 = param_2;
  if (param_2 == (undefined1 *)0x0) {
    iVar1 = FUN_0012a9b8(param_1,local_a8);
    if (iVar1 != 0) {
      piVar3 = __errno_location();
      if (*piVar3 == 2) {
        uVar2 = 0;
      }
      else {
        FUN_0013ea2c(param_1);
        uVar2 = 1;
      }
      goto LAB_0011993a;
    }
    local_b8 = local_a8;
  }
  if ((*(uint *)(local_b8 + 0x18) & 0xf000) != 0x4000) {
    auVar4 = FUN_0015fd9e(local_b8);
    iVar1 = FUN_00139e35(*(undefined8 *)(param_3 + 0xf8),*(undefined8 *)(param_3 + 0x100),
                         auVar4._0_8_,auVar4._8_8_);
    if (iVar1 < 1) {
      uVar2 = 1;
      goto LAB_0011993a;
    }
  }
  uVar2 = 0;
LAB_0011993a:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: maybe_recoverable @ 0x19950

undefined8 maybe_recoverable(char *param_1,char param_2,char *param_3)

{
  int iVar1;
  char cVar2;
  int iVar3;
  int *piVar4;
  undefined8 uVar5;
  char *pcVar6;
  long in_FS_OFFSET;
  undefined1 *local_b0;
  undefined1 local_a8 [152];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  piVar4 = __errno_location();
  iVar1 = *piVar4;
  local_b0 = (undefined1 *)0x0;
  if (*param_3 != '\0') {
    uVar5 = 0;
    goto LAB_00119bb8;
  }
  if (iVar1 != 0x5f) {
    if (iVar1 < 0x60) {
      if (iVar1 == 0x28) goto LAB_00119a11;
      if (0x28 < iVar1) goto LAB_00119ba6;
      if (iVar1 == 0x1f) goto LAB_00119a11;
      if (0x1f < iVar1) goto LAB_00119ba6;
      if (iVar1 != 2) {
        if (iVar1 != 0x11) goto LAB_00119ba6;
        goto LAB_00119a89;
      }
      goto switchD_00119ab8_default;
    }
    goto LAB_00119ba6;
  }
LAB_00119a11:
  if (((param_2 != '\x01') || (DAT_0019f8d8 != 2)) || (DAT_0019f8a0 != '\0')) goto LAB_00119ba6;
  pcVar6 = strchr(param_1,0x2f);
  if (pcVar6 != (char *)0x0) {
    iVar3 = FUN_0012a9b8(param_1,local_a8);
    if (iVar3 != 0) goto LAB_00119ba6;
    local_b0 = local_a8;
  }
LAB_00119a89:
  switch(DAT_0019f8d8) {
  case 0:
  case 1:
  case 2:
switchD_00119ab8_caseD_0:
    iVar3 = FUN_0012a36f(param_1,0);
    if (0 < iVar3) {
      uVar5 = 1;
      goto LAB_00119bb8;
    }
    break;
  case 3:
    break;
  case 4:
    uVar5 = 0;
    goto LAB_00119bb8;
  case 5:
    if ((DAT_0019ec84 & 0x100000) != 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar5 = gettext("%s: skipping existing file");
      error(0,0,uVar5,param_1);
    }
    uVar5 = 2;
    goto LAB_00119bb8;
  case 6:
    cVar2 = FUN_00119851(param_1,local_b0,&DAT_0019fa20);
    if (cVar2 == '\0') goto switchD_00119ab8_caseD_0;
  }
switchD_00119ab8_default:
  iVar3 = FUN_001195aa(param_1,param_3);
  if ((iVar3 == 0) && (*param_3 != '\0')) {
    uVar5 = 1;
  }
  else {
LAB_00119ba6:
    piVar4 = __errno_location();
    *piVar4 = iVar1;
    uVar5 = 0;
  }
LAB_00119bb8:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: set_xattr @ 0x19bce

undefined8 set_xattr(char *param_1,long param_2,uint param_3,char param_4,undefined4 *param_5)

{
  int iVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined1 local_15;
  uint local_14;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_15 = 0;
  if ((DAT_0019f96c < 1) || (*(long *)(param_2 + 0x158) == 0)) {
    uVar2 = 0;
  }
  else {
    local_14 = ~DAT_0019f3b0 & DAT_0019fa90 & 0x1ff;
    do {
      iVar1 = mknodat(DAT_0019c268,param_1,local_14 ^ param_3,0);
      if (iVar1 == 0) {
        FUN_0013d8a1(param_2,param_1,(int)param_4,0);
        *param_5 = 1;
        uVar2 = 0;
        goto LAB_00119cd3;
      }
      iVar1 = FUN_00119950(param_1,0,&local_15);
      if (iVar1 == 2) {
        uVar2 = 0;
        goto LAB_00119cd3;
      }
    } while ((2 < iVar1) || (iVar1 != 0));
    FUN_00128891();
    FUN_0013e38b(param_1);
    uVar2 = 1;
  }
LAB_00119cd3:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: apply_nonancestor_delayed_set_stat @ 0x19ce9

void apply_nonancestor_delayed_set_stat(char *param_1,byte param_2)

{
  char cVar1;
  bool bVar2;
  bool bVar3;
  undefined8 *puVar4;
  int iVar5;
  size_t sVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  long in_FS_OFFSET;
  __mode_t local_280;
  undefined4 local_27c;
  stat local_268;
  undefined1 local_1d8 [48];
  undefined8 local_1a8;
  undefined8 local_1a0;
  undefined8 local_198;
  undefined8 local_190;
  undefined8 local_188;
  undefined4 local_168;
  undefined4 local_164;
  undefined4 local_160;
  undefined8 local_f0;
  undefined8 local_e8;
  undefined8 local_e0;
  undefined8 local_d8;
  undefined8 local_80;
  undefined8 local_78;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  sVar6 = strlen(param_1);
  bVar2 = false;
  while (puVar4 = DAT_0019f3b8, DAT_0019f3b8 != (undefined8 *)0x0) {
    bVar3 = false;
    local_280 = *(__mode_t *)(DAT_0019f3b8 + 9);
    local_27c = *(undefined4 *)((long)DAT_0019f3b8 + 0x4c);
    cVar1 = *(char *)(DAT_0019f3b8 + 0xb);
    if (param_2 < *(byte *)(DAT_0019f3b8 + 0xb)) break;
    if ((((ulong)DAT_0019f3b8[0x13] < sVar6) && (param_1[DAT_0019f3b8[0x13]] != '\0')) &&
       ((param_1[DAT_0019f3b8[0x13]] == '/' || (param_1[DAT_0019f3b8[0x13] + -1] == '/')))) {
      iVar5 = memcmp(param_1,(void *)DAT_0019f3b8[0x14],DAT_0019f3b8[0x13]);
      if (iVar5 == 0) break;
    }
    FUN_0012adf1(*(undefined4 *)((long)puVar4 + 0x5c));
    if (cVar1 != '\0' || bVar2) {
      iVar5 = fstatat(DAT_0019c268,(char *)puVar4[0x14],&local_268,*(int *)((long)puVar4 + 0x54));
      if (iVar5 == 0) {
        local_280 = local_268.st_mode;
        local_27c = 0xffffffff;
        if ((local_268.st_dev != puVar4[1]) || (local_268.st_ino != puVar4[2])) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar7 = FUN_0015e548(puVar4[0x14]);
          uVar8 = gettext("%s: Directory renamed before its status could be extracted");
          error(0,0,uVar8,uVar7);
          DAT_0019fdd0 = 2;
          bVar3 = true;
        }
      }
      else {
        FUN_0013ea03(puVar4[0x14]);
        bVar3 = true;
      }
    }
    if (!bVar3) {
      local_168 = *(undefined4 *)(puVar4 + 3);
      local_164 = *(undefined4 *)((long)puVar4 + 0x1c);
      local_160 = *(undefined4 *)(puVar4 + 4);
      local_e8 = puVar4[6];
      local_f0 = puVar4[5];
      local_d8 = puVar4[8];
      local_e0 = puVar4[7];
      local_1a8 = puVar4[0xc];
      local_1a0 = puVar4[0xd];
      local_198 = puVar4[0xe];
      local_190 = puVar4[0xf];
      local_188 = puVar4[0x10];
      local_78 = puVar4[0x12];
      local_80 = puVar4[0x11];
      FUN_00118913(puVar4[0x14],local_1d8,0xffffffff,local_280,local_27c,0x35,
                   *(undefined1 *)(puVar4 + 10),*(undefined4 *)((long)puVar4 + 0x54));
    }
    DAT_0019f3b8 = (undefined8 *)*puVar4;
    FUN_001193d1(puVar4);
    bVar2 = cVar1 != '\0' || bVar2;
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: is_directory_link @ 0x1a10f

undefined1 is_directory_link(char *param_1)

{
  int iVar1;
  undefined1 uVar2;
  int iVar3;
  int *piVar4;
  long in_FS_OFFSET;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  piVar4 = __errno_location();
  iVar1 = *piVar4;
  iVar3 = fstatat(DAT_0019c268,param_1,&local_a8,0x100);
  if ((iVar3 == 0) && ((local_a8.st_mode & 0xf000) == 0xa000)) {
    iVar3 = fstatat(DAT_0019c268,param_1,&local_a8,0);
    if ((iVar3 == 0) && ((local_a8.st_mode & 0xf000) == 0x4000)) {
      uVar2 = 1;
      goto LAB_0011a1b9;
    }
  }
  uVar2 = 0;
LAB_0011a1b9:
  piVar4 = __errno_location();
  *piVar4 = iVar1;
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: safe_dir_mode @ 0x1a1ec

uint safe_dir_mode(long param_1)

{
  uint uVar1;
  uint uVar2;
  
  if ((DAT_0019f95c < 1) && (DAT_0019f960 < 1)) {
    uVar1 = 0x1ff;
  }
  else {
    uVar1 = 0x1c0;
  }
  if (DAT_0019f3a8 == '\0') {
    uVar2 = 0xc0;
  }
  else {
    uVar2 = 0;
  }
  return uVar2 | *(uint *)(param_1 + 0x18) & uVar1;
}




// Function: extract_dir @ 0x1a238

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int extract_dir(char *param_1,int param_2)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  long in_FS_OFFSET;
  char local_bd;
  int local_bc;
  uint local_b8;
  undefined4 local_b4;
  undefined4 local_b0;
  __mode_t local_ac;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_b8 = 0;
  local_b4 = 0;
  local_b0 = 0;
  local_bd = '\0';
  if ((DAT_0019f935 != '\0') && (DAT_0019fc20 == 0)) {
    iVar2 = fstatat(DAT_0019c268,".",&local_a8,0);
    if (iVar2 == 0) {
      DAT_0019fc20 = local_a8.st_dev;
    }
    else {
      FUN_0012b444(&DAT_0018137b);
    }
  }
  if (DAT_0019f8be == '\0') {
    if (param_2 == 0x44) {
      FUN_00128891();
    }
  }
  else {
    FUN_00125377(param_1);
  }
  local_ac = FUN_0011a1ec(&DAT_0019fa78);
  do {
    local_bc = mkdirat(DAT_0019c268,param_1,local_ac);
    if (local_bc == 0) {
      local_b8 = ~DAT_0019f3b0 & local_ac;
      local_b4 = 0x1ff;
      local_b0 = 0x100;
      break;
    }
    piVar3 = __errno_location();
    if ((*piVar3 == 0x11) &&
       ((((local_bd != '\0' || (DAT_0019f8dc != '\0')) || (DAT_0019f8d8 == 1)) ||
        ((DAT_0019f8d8 == 0 || (DAT_0019f8d8 == 2)))))) {
      if ((DAT_0019f8dc != '\0') && (cVar1 = FUN_0011a10f(param_1), cVar1 != '\0')) {
        iVar2 = 0;
        goto LAB_0011a69f;
      }
      iVar2 = FUN_0012a9b8(param_1,&local_a8);
      if (iVar2 == 0) {
        local_b8 = local_a8.st_mode;
        local_b4 = 0xffffffff;
        if ((local_a8.st_mode & 0xf000) == 0x4000) {
          if (local_bd != '\0') {
            FUN_001191c5(param_1,&local_a8);
            iVar2 = 0;
            goto LAB_0011a69f;
          }
          if (DAT_0019f8d8 == 1) {
            local_ac = FUN_0011a1ec(&local_a8);
            local_bc = FUN_001183ee(0xffffffff,param_1,local_ac,0x100,0x35);
            if (local_bc == 0) {
              DAT_0019fa78 = local_a8.st_dev;
              DAT_0019fa80 = local_a8.st_ino;
              _DAT_0019fa90 = CONCAT44(local_a8.st_uid,local_a8.st_mode);
              _DAT_0019fa88 = local_a8.st_nlink;
              DAT_0019fa98 = local_a8.st_gid;
              DAT_0019fa98_4 = local_a8.__pad0;
              DAT_0019faa0 = local_a8.st_rdev;
              DAT_0019faa8 = local_a8.st_size;
              _DAT_0019fab0 = local_a8.st_blksize;
              _DAT_0019fab8 = local_a8.st_blocks;
              _DAT_0019fac0 = local_a8.st_atim.tv_sec;
              _DAT_0019fac8 = local_a8.st_atim.tv_nsec;
              _DAT_0019fad0 = local_a8.st_mtim.tv_sec;
              _DAT_0019fad8 = local_a8.st_mtim.tv_nsec;
              _DAT_0019fae0 = local_a8.st_ctim.tv_sec;
              _DAT_0019fae8 = local_a8.st_ctim.tv_nsec;
              _DAT_0019faf0 = local_a8.__unused[0];
              _DAT_0019faf8 = local_a8.__unused[1];
              _DAT_0019fb00 = local_a8.__unused[2];
              local_b8 = ~DAT_0019f3b0 & local_ac;
              local_b4 = 0x1ff;
              local_b0 = 0x100;
            }
            else {
              FUN_0013e0b2(param_1,local_ac);
            }
          }
          break;
        }
      }
      piVar3 = __errno_location();
      *piVar3 = 0x11;
    }
    iVar2 = FUN_00119950(param_1,0,&local_bd);
    if ((iVar2 == 2) || (2 < iVar2)) break;
    if (iVar2 == 0) {
      piVar3 = __errno_location();
      if (*piVar3 != 0x11) {
        FUN_0013e310(param_1);
        iVar2 = 1;
        goto LAB_0011a69f;
      }
      break;
    }
  } while (iVar2 == 1);
  if (((local_bc == 0) || (DAT_0019f8d8 == 0)) || (iVar2 = local_bc, DAT_0019f8d8 == 2)) {
    FUN_00118d26(param_1,&DAT_0019fa20,local_b8,local_b4,_DAT_0019fa90 & 0xffffffff,local_b0);
    iVar2 = local_bc;
  }
LAB_0011a69f:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return iVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: open_output_file @ 0x1a6b5

int open_output_file(char *param_1,int param_2,uint param_3,int param_4,uint *param_5,
                undefined4 *param_6)

{
  int __fd;
  int iVar1;
  undefined8 uVar2;
  int *piVar3;
  long in_FS_OFFSET;
  bool bVar4;
  uint local_b4;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  bVar4 = DAT_0019f8d8 != 2;
  if (bVar4) {
    local_b4 = 0x809c1;
  }
  else if (DAT_0019f8a0 == '\0') {
    local_b4 = 0xa0b41;
  }
  else {
    local_b4 = 0x80b41;
  }
  if (param_4 != 0) {
    local_b4 = local_b4 & 0xffffff7f;
  }
  if (((param_2 == 0x37) && (DAT_0019f3c8 == 0)) && (DAT_0019f3c8 = 1, (DAT_0019ec84 & 8) != 0)) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar2 = gettext("Extracting contiguous files as regular files");
    error(0,0,uVar2);
  }
  __fd = openat(DAT_0019c268,param_1,local_b4,(ulong)param_3);
  if (-1 < __fd) {
    if (bVar4) {
      *param_5 = ~DAT_0019f3b0 & param_3;
      *param_6 = 0x1ff;
    }
    else {
      iVar1 = fstat(__fd,&local_a8);
      if (iVar1 == 0) {
        if ((local_a8.st_mode & 0xf000) == 0x8000) {
          *param_5 = local_a8.st_mode;
          *param_6 = 0xffffffff;
        }
        else {
          close(__fd);
          piVar3 = __errno_location();
          *piVar3 = 0x11;
          __fd = -1;
        }
      }
      else {
        piVar3 = __errno_location();
        iVar1 = *piVar3;
        close(__fd);
        piVar3 = __errno_location();
        *piVar3 = iVar1;
        __fd = -1;
      }
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return __fd;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: extract_file @ 0x1a8d0

int extract_file(undefined8 param_1,undefined4 param_2)

{
  uint uVar1;
  int iVar2;
  int *piVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  undefined1 local_4d;
  undefined4 local_4c;
  undefined4 local_48;
  int local_44;
  uint local_40;
  uint local_3c;
  int local_38;
  int local_34;
  ulong local_30;
  ulong local_28;
  long local_20;
  ulong local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_4d = 0;
  if (DAT_0019f95c < 1) {
    uVar1 = 0xffffffff;
  }
  else {
    uVar1 = 0xffffffc0;
  }
  local_40 = uVar1 & DAT_0019fa90 & 0x1ff;
  if (DAT_0019f95c < 1) {
    local_3c = 0;
  }
  else {
    local_3c = uVar1 & DAT_0019fa90 & 0x3f;
  }
  local_4c = 0;
  local_48 = 0;
  if (DAT_0019f998 == '\0') {
    if (DAT_0019f9a0 == 0) {
      local_30 = local_30 & 0xffffffff00000000;
      iVar2 = FUN_00119bce(param_1,&DAT_0019fa20,local_3c,(int)(char)param_2,&local_30);
      if (iVar2 == 0) {
        do {
          local_44 = FUN_0011a6b5(param_1,param_2,local_40,local_30 & 0xffffffff,&local_4c,&local_48
                                 );
          if (-1 < local_44) goto LAB_0011aa48;
          local_38 = FUN_00119950(param_1,1,&local_4d);
        } while (local_38 == 1);
        FUN_00128891();
        if (local_38 == 2) {
          iVar2 = 0;
        }
        else {
          FUN_0013e38b(param_1);
          iVar2 = 1;
        }
      }
      else {
        iVar2 = 1;
      }
      goto LAB_0011ac4b;
    }
    local_44 = FUN_00134323(param_1,0x66,&DAT_0019fa20);
    if (local_44 < 0) {
      FUN_00128891();
      iVar2 = 0;
      goto LAB_0011ac4b;
    }
  }
  else {
    local_44 = 1;
  }
LAB_0011aa48:
  FUN_0010ad2b(&DAT_0019fa20);
  if (DAT_0019fb40 == '\0') {
    local_30 = DAT_0019faa8;
    do {
      if ((long)local_30 < 1) goto LAB_0011ab9d;
      FUN_0010ad86(local_30);
      local_20 = FUN_0010bb4b();
      if (local_20 == 0) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar4 = gettext("Unexpected EOF in archive");
        error(0,0,uVar4);
        DAT_0019fdd0 = 2;
        goto LAB_0011ab9d;
      }
      local_28 = FUN_0010bbf5(local_20);
      if (local_30 < local_28) {
        local_28 = local_30;
      }
      piVar3 = __errno_location();
      *piVar3 = 0;
      local_18 = FUN_0012aa95(local_44,local_20,local_28);
      local_30 = local_30 - local_28;
      FUN_0010bba7(local_20 + (local_28 - 1));
    } while (local_18 == local_28);
    if (DAT_0019f9a0 == 0) {
      FUN_0013eb4b(param_1,local_18,local_28);
    }
  }
  else {
    FUN_001302a4(local_44,&DAT_0019fa20,&local_30);
  }
LAB_0011ab9d:
  FUN_001287c1(local_30);
  FUN_0010ad66();
  if (DAT_0019f998 == '\0') {
    if (DAT_0019f9a0 == 0) {
      if (DAT_0019f8d8 == 2) {
        uVar4 = 0;
      }
      else {
        uVar4 = 0x100;
      }
      FUN_00118913(param_1,&DAT_0019fa20,local_44,local_4c,local_48,(int)(char)param_2,0,uVar4);
    }
    local_34 = close(local_44);
    if (local_34 < 0) {
      FUN_0013e1ff(param_1);
    }
    iVar2 = local_34;
    if (DAT_0019f9a0 != 0) {
      FUN_001343ea();
      iVar2 = local_34;
    }
  }
  else {
    iVar2 = 0;
  }
LAB_0011ac4b:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return iVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: find_delayed_link_source @ 0x1ac61

undefined8 * find_delayed_link_source(char *param_1)

{
  int iVar1;
  int *piVar2;
  long in_FS_OFFSET;
  undefined8 *local_b0;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0019f3c0 == (undefined8 *)0x0) {
    local_b0 = (undefined8 *)0x0;
  }
  else {
    iVar1 = fstatat(DAT_0019c268,param_1,&local_a8,0x100);
    if (iVar1 == 0) {
      for (local_b0 = DAT_0019f3c0;
          (local_b0 != (undefined8 *)0x0 &&
          ((local_b0[1] != local_a8.st_dev || (local_b0[2] != local_a8.st_ino))));
          local_b0 = (undefined8 *)*local_b0) {
      }
    }
    else {
      piVar2 = __errno_location();
      if (*piVar2 != 2) {
        FUN_0013ea03(param_1);
      }
      local_b0 = (undefined8 *)0x0;
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_b0;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: create_placeholder_file @ 0x1ad5b

undefined8 create_placeholder_file(char *param_1,char param_2,undefined8 param_3,undefined8 *param_4)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  size_t sVar4;
  undefined8 *puVar5;
  long lVar6;
  undefined8 uVar7;
  long in_FS_OFFSET;
  undefined1 auVar8 [16];
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  do {
    iVar1 = openat(DAT_0019c268,param_1,0xc1,0);
    if (-1 < iVar1) {
      iVar2 = fstat(iVar1,&local_b8);
      if (iVar2 == 0) {
        iVar1 = close(iVar1);
        if (iVar1 == 0) {
          sVar4 = strlen(DAT_0019fa38);
          puVar5 = (undefined8 *)FUN_00160b5b(sVar4 + 0xa1);
          if (param_4 == (undefined8 *)0x0) {
            *puVar5 = DAT_0019f3c0;
            DAT_0019f3c0 = puVar5;
          }
          else {
            *puVar5 = *param_4;
            *param_4 = puVar5;
          }
          puVar5[1] = local_b8.st_dev;
          puVar5[2] = local_b8.st_ino;
          auVar8 = FUN_0015fdb8(&local_b8);
          *(undefined1 (*) [16])(puVar5 + 3) = auVar8;
          *(char *)(puVar5 + 5) = param_2;
          if (param_2 != '\0') {
            *(undefined4 *)((long)puVar5 + 0x2c) = DAT_0019fa90;
            *(undefined4 *)(puVar5 + 6) = DAT_0019fa94;
            *(undefined4 *)((long)puVar5 + 0x34) = DAT_0019fa98;
            uVar7 = DAT_0019fb10;
            puVar5[7] = DAT_0019fb08;
            puVar5[8] = uVar7;
            uVar7 = DAT_0019fb20;
            puVar5[9] = DAT_0019fb18;
            puVar5[10] = uVar7;
          }
          *(undefined4 *)(puVar5 + 0xb) = DAT_0019f600;
          sVar4 = strlen(param_1);
          uVar7 = FUN_00160b5b(sVar4 + 9);
          puVar5[0xc] = uVar7;
          *(undefined8 *)puVar5[0xc] = 0;
          strcpy((char *)(puVar5[0xc] + 8),param_1);
          puVar5[0xd] = 0;
          FUN_00129547(puVar5 + 0xd,DAT_0019fa50);
          puVar5[0xe] = 0;
          puVar5[0xf] = 0;
          puVar5[0x10] = 0;
          puVar5[0x11] = 0;
          FUN_0011d854(&DAT_0019fa20,puVar5 + 0x13,puVar5 + 0x12);
          strcpy((char *)(puVar5 + 0x14),DAT_0019fa38);
          lVar6 = FUN_00118b89(param_1);
          if (lVar6 != 0) {
            FUN_00118c40(lVar6);
          }
          uVar7 = 0;
          goto LAB_0011b147;
        }
        FUN_0013e1ff(param_1);
      }
      else {
        FUN_0013ea03(param_1);
        close(iVar1);
      }
      uVar7 = 0xffffffff;
      goto LAB_0011b147;
    }
    piVar3 = __errno_location();
    if ((*piVar3 == 0x11) && (lVar6 = FUN_0011ac61(param_1), lVar6 != 0)) {
      uVar7 = 0;
      goto LAB_0011b147;
    }
    iVar1 = FUN_00119950(param_1,0,param_3);
    if (iVar1 == 2) {
      uVar7 = 0;
      goto LAB_0011b147;
    }
  } while ((2 < iVar1) || (iVar1 != 0));
  FUN_0013e38b(param_1);
  uVar7 = 0xffffffff;
LAB_0011b147:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar7;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: extract_link @ 0x1b161

undefined8 extract_link(char *param_1)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  int *piVar4;
  size_t sVar5;
  long in_FS_OFFSET;
  undefined1 auVar6 [16];
  undefined1 local_165;
  int local_164;
  int local_160;
  int local_15c;
  undefined8 *local_158;
  char *local_150;
  long local_148;
  undefined8 *local_140;
  stat local_138;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_165 = 0;
  local_150 = DAT_0019fa38;
  if ((DAT_0019f878 == '\x01') || (cVar1 = FUN_0012efdc(DAT_0019fa38), cVar1 == '\0')) {
    local_148 = FUN_0011ac61(local_150);
    if (local_148 == 0) {
      do {
        local_164 = linkat(DAT_0019c268,local_150,DAT_0019c268,param_1,0);
        piVar4 = __errno_location();
        local_160 = *piVar4;
        if (local_164 == 0) {
          local_158 = DAT_0019f3c0;
          if ((DAT_0019f3c0 == (undefined8 *)0x0) ||
             (iVar2 = fstatat(DAT_0019c268,local_150,&local_138,0x100), iVar2 != 0))
          goto LAB_0011b3c0;
          goto LAB_0011b3b2;
        }
        if (((local_160 == 0x11) && (iVar2 = strcmp(local_150,param_1), iVar2 == 0)) ||
           (((iVar2 = fstatat(DAT_0019c268,local_150,&local_138,0x100), iVar2 == 0 &&
             ((iVar2 = fstatat(DAT_0019c268,param_1,&local_a8,0x100), iVar2 == 0 &&
              (local_138.st_dev == local_a8.st_dev)))) && (local_138.st_ino == local_a8.st_ino)))) {
          uVar3 = 0;
          goto LAB_0011b4f2;
        }
        piVar4 = __errno_location();
        *piVar4 = local_160;
        local_15c = FUN_00119950(param_1,0,&local_165);
      } while (local_15c == 1);
      if (local_15c == 2) {
        uVar3 = 0;
      }
      else if ((DAT_0019f8be == '\x01') && (piVar4 = __errno_location(), *piVar4 == 0x11)) {
        uVar3 = 0;
      }
      else {
        FUN_0013e277(local_150,param_1);
        uVar3 = 1;
      }
    }
    else {
      uVar3 = FUN_0011ad5b(param_1,0,&local_165,local_148);
    }
  }
  else {
    uVar3 = FUN_0011ad5b(param_1,0,&local_165,0);
  }
LAB_0011b4f2:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
LAB_0011b3b2:
  if (local_158 == (undefined8 *)0x0) goto LAB_0011b3c0;
  if (((*(int *)(local_158 + 0xb) == DAT_0019f600) && (local_158[1] == local_138.st_dev)) &&
     (local_158[2] == local_138.st_ino)) {
    auVar6 = FUN_0015fdb8(&local_138);
    iVar2 = FUN_0015ff90(local_158[3],local_158[4],auVar6._0_8_,auVar6._8_8_);
    if (iVar2 == 0) {
      sVar5 = strlen(param_1);
      local_140 = (undefined8 *)FUN_00160b5b(sVar5 + 9);
      strcpy((char *)(local_140 + 1),param_1);
      *local_140 = local_158[0xc];
      local_158[0xc] = local_140;
      goto LAB_0011b3c0;
    }
  }
  local_158 = (undefined8 *)*local_158;
  goto LAB_0011b3b2;
LAB_0011b3c0:
  uVar3 = 0;
  goto LAB_0011b4f2;
}




// Function: extract_symlink @ 0x1b508

undefined8 extract_symlink(char *param_1)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined1 local_11;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_11 = 0;
  if ((DAT_0019f878 == '\x01') ||
     ((*DAT_0019fa38 != '/' && (cVar1 = FUN_0012efdc(DAT_0019fa38), cVar1 == '\0')))) {
    do {
      iVar2 = symlinkat(DAT_0019fa38,DAT_0019c268,param_1);
      if (iVar2 == 0) {
        FUN_00118913(param_1,&DAT_0019fa20,0xffffffff,0,0,0x32,0,0x100);
        uVar3 = 0;
        goto LAB_0011b624;
      }
      iVar2 = FUN_00119950(param_1,0,&local_11);
      if (iVar2 == 2) {
        uVar3 = 0;
        goto LAB_0011b624;
      }
    } while ((2 < iVar2) || (iVar2 != 0));
    FUN_0013e944(DAT_0019fa38,param_1);
    uVar3 = 0xffffffff;
  }
  else {
    uVar3 = FUN_0011ad5b(param_1,1,&local_11,0);
  }
LAB_0011b624:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: extract_node @ 0x1b63a

undefined8 extract_node(char *param_1,char param_2)

{
  uint uVar1;
  int iVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined1 local_15;
  __mode_t local_14;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_15 = 0;
  if (DAT_0019f95c < 1) {
    uVar1 = 0xffffffff;
  }
  else {
    uVar1 = 0xffffffc0;
  }
  local_14 = uVar1 & DAT_0019fa90 & 0x61ff;
  while( true ) {
    iVar2 = mknodat(DAT_0019c268,param_1,local_14,DAT_0019faa0);
    if (iVar2 == 0) {
      FUN_00118913(param_1,&DAT_0019fa20,0xffffffff,~DAT_0019f3b0 & local_14,0x1ff,(int)param_2,0,
                   0x100);
      uVar3 = 0;
      goto LAB_0011b72f;
    }
    iVar2 = FUN_00119950(param_1,0,&local_15);
    if (iVar2 == 2) break;
    if ((iVar2 < 3) && (iVar2 == 0)) {
      FUN_0013e362(param_1);
      uVar3 = 0xffffffff;
LAB_0011b72f:
      if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
        return uVar3;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
  }
  uVar3 = 0;
  goto LAB_0011b72f;
}




// Function: extract_fifo @ 0x1b745

undefined8 extract_fifo(char *param_1,char param_2)

{
  uint uVar1;
  int iVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined1 local_15;
  __mode_t local_14;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_15 = 0;
  if (DAT_0019f95c < 1) {
    uVar1 = 0xffffffff;
  }
  else {
    uVar1 = 0xffffffc0;
  }
  local_14 = uVar1 & DAT_0019fa90 & 0x1ff;
  while( true ) {
    iVar2 = mkfifoat(DAT_0019c268,param_1,local_14);
    if (iVar2 == 0) {
      FUN_00118913(param_1,&DAT_0019fa20,0xffffffff,~DAT_0019f3b0 & local_14,0x1ff,(int)param_2,0,
                   0x100);
      uVar3 = 0;
      goto LAB_0011b836;
    }
    iVar2 = FUN_00119950(param_1,0,&local_15);
    if (iVar2 == 2) break;
    if ((iVar2 < 3) && (iVar2 == 0)) {
      FUN_0013e339(param_1);
      uVar3 = 0xffffffff;
LAB_0011b836:
      if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
        return uVar3;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
  }
  uVar3 = 0;
  goto LAB_0011b836;
}




// Function: prepare_to_extract @ 0x1b84c

undefined8 prepare_to_extract(undefined8 param_1,undefined4 param_2,undefined8 *param_3)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  int *piVar5;
  code *local_20;
  
  switch(param_2) {
  case 0:
  case 0x30:
  case 0x37:
    if (DAT_0019fa30 == '\0') {
      local_20 = FUN_0011a8d0;
    }
    else {
      local_20 = FUN_0011a238;
    }
    break;
  default:
    if ((DAT_0019ec84 & 0x10000) != 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar3 = FUN_0015e548(param_1);
      uVar4 = gettext("%s: Unknown file type \'%c\', extracted as normal file");
      error(0,0,uVar4,uVar3,param_2);
    }
    local_20 = FUN_0011a8d0;
    break;
  case 0x31:
    local_20 = FUN_0011b161;
    break;
  case 0x32:
    local_20 = FUN_0011b508;
    break;
  case 0x33:
    DAT_0019fa90 = DAT_0019fa90 | 0x2000;
    local_20 = FUN_0011b63a;
    break;
  case 0x34:
    DAT_0019fa90 = DAT_0019fa90 | 0x6000;
    local_20 = FUN_0011b63a;
    break;
  case 0x35:
  case 0x44:
    local_20 = FUN_0011a238;
    if (DAT_0019fba8 != '\0') {
      DAT_0019fc31 = 1;
    }
    break;
  case 0x36:
    local_20 = FUN_0011b745;
    break;
  case 0x4b:
  case 0x4c:
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar3 = gettext("Unexpected long name header");
    error(0,0,uVar3);
    DAT_0019fdd0 = 2;
    return 0;
  case 0x4d:
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar3 = FUN_0015e548(DAT_0019fa28);
    uVar4 = gettext("%s: Cannot extract -- file is continued from another volume");
    error(0,0,uVar4,uVar3);
    DAT_0019fdd0 = 2;
    return 0;
  case 0x53:
    local_20 = FUN_0011a8d0;
    break;
  case 0x56:
    return 0;
  }
  if ((DAT_0019f998 == '\0') && (DAT_0019f9a0 == 0)) {
    if (DAT_0019f8d8 == 3) {
      iVar2 = FUN_0012a36f(param_1,DAT_0019f94c);
      if (((iVar2 == 0) && (piVar5 = __errno_location(), *piVar5 != 0)) &&
         (piVar5 = __errno_location(), *piVar5 != 2)) {
        FUN_0013eaa7(param_1);
        return 0;
      }
    }
    else if ((DAT_0019f8d8 == 6) && (cVar1 = FUN_00119851(param_1,0,&DAT_0019fa20), cVar1 != '\0'))
    {
      if ((DAT_0019ec84 & 0x800) != 0) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar3 = FUN_0015e84e(param_1);
        uVar4 = gettext("Current %s is newer or same age");
        error(0,0,uVar4,uVar3);
      }
      return 0;
    }
  }
  else if (local_20 != FUN_0011a8d0) {
    return 0;
  }
  *param_3 = local_20;
  return 1;
}




// Function: extract_archive @ 0x1bbcb

void extract_archive(void)

{
  undefined4 uVar1;
  char cVar2;
  bool bVar3;
  char cVar4;
  int iVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  int *piVar8;
  long in_FS_OFFSET;
  code *local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  DAT_0019f388 = FUN_0011c2ee;
  FUN_0010bba7(DAT_0019f460);
  if ((DAT_0019f878 == '\x01') || (cVar2 = FUN_0012efdc(DAT_0019fa20), cVar2 == '\0')) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  if (bVar3) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar6 = FUN_0015e548(DAT_0019fa20);
    uVar7 = gettext("%s: Member name contains \'..\'");
    error(0,0,uVar7,uVar6);
    DAT_0019fdd0 = 2;
  }
  if (((*DAT_0019fa28 == '\0') || (bVar3)) ||
     ((DAT_0019f8c8 != '\0' && (iVar5 = FUN_00134af4("extract",DAT_0019fa28), iVar5 == 0)))) {
    FUN_00128891();
  }
  else {
    if (DAT_0019f9ac != 0) {
      FUN_0012864c(&DAT_0019fa20,DAT_0019f460,0xffffffffffffffff);
    }
    uVar1 = DAT_0019f600;
    if (DAT_0019fc31 != '\x01') {
      FUN_00119ce9(DAT_0019fa28,0);
      FUN_0012adf1(uVar1);
    }
    if ((DAT_0019f884 == '\0') || (cVar2 = FUN_0012a571(DAT_0019fa28,0), cVar2 == '\x01')) {
      cVar2 = FUN_001301de(&DAT_0019fa20);
      if (cVar2 == '\0') {
        cVar2 = *(char *)(DAT_0019f460 + 0x9c);
      }
      else {
        cVar2 = 'S';
      }
      cVar4 = FUN_0011b84c(DAT_0019fa28,(int)cVar2,&local_28);
      if (cVar4 == '\0') {
        FUN_00128891();
      }
      else {
        iVar5 = (*local_28)(DAT_0019fa28,(int)cVar2);
        if (iVar5 == 0) goto LAB_0011be4b;
      }
      if (DAT_0019f884 != '\0') {
        FUN_0012a874();
      }
    }
    else {
      piVar8 = __errno_location();
      iVar5 = *piVar8;
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar6 = FUN_0015e548(DAT_0019fa28);
      uVar7 = gettext("%s: Was unable to backup this file");
      error(0,iVar5,uVar7,uVar6);
      DAT_0019fdd0 = 2;
      FUN_00128891();
    }
  }
LAB_0011be4b:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: apply_delayed_links @ 0x1be65

void apply_delayed_links(void)

{
  undefined8 *puVar1;
  int iVar2;
  char *__file;
  long in_FS_OFFSET;
  undefined1 auVar3 [16];
  undefined8 *local_298;
  undefined8 *local_290;
  char *local_288;
  stat local_268;
  undefined1 local_1d8 [48];
  undefined8 local_1a8;
  undefined8 local_1a0;
  undefined8 local_198;
  undefined8 local_190;
  undefined8 local_188;
  undefined4 local_168;
  undefined4 local_164;
  undefined4 local_160;
  undefined8 local_f0;
  undefined8 local_e8;
  undefined8 local_e0;
  undefined8 local_d8;
  undefined8 local_80;
  undefined8 local_78;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_298 = DAT_0019f3c0;
  do {
    if (local_298 == (undefined8 *)0x0) {
      DAT_0019f3c0 = (undefined8 *)0x0;
      if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return;
    }
    local_288 = (char *)0x0;
    FUN_0012adf1(*(undefined4 *)(local_298 + 0xb));
    for (local_290 = (undefined8 *)local_298[0xc]; local_290 != (undefined8 *)0x0;
        local_290 = (undefined8 *)*local_290) {
      __file = (char *)(local_290 + 1);
      iVar2 = fstatat(DAT_0019c268,__file,&local_268,0x100);
      if (((iVar2 == 0) && (local_268.st_dev == local_298[1])) && (local_268.st_ino == local_298[2])
         ) {
        auVar3 = FUN_0015fdb8(&local_268);
        iVar2 = FUN_0015ff90(auVar3._0_8_,auVar3._8_8_,local_298[3],local_298[4]);
        if (iVar2 == 0) {
          iVar2 = unlinkat(DAT_0019c268,__file,0);
          if (iVar2 == 0) {
            if (local_288 != (char *)0x0) {
              iVar2 = linkat(DAT_0019c268,local_288,DAT_0019c268,__file,0);
              if (iVar2 == 0) goto LAB_0011c1e7;
            }
            if (*(char *)(local_298 + 5) == '\x01') {
              iVar2 = symlinkat((char *)(local_298 + 0x14),DAT_0019c268,__file);
              if (iVar2 == 0) {
                local_168 = *(undefined4 *)((long)local_298 + 0x2c);
                local_164 = *(undefined4 *)(local_298 + 6);
                local_160 = *(undefined4 *)((long)local_298 + 0x34);
                local_e8 = local_298[8];
                local_f0 = local_298[7];
                local_d8 = local_298[10];
                local_e0 = local_298[9];
                local_1a8 = local_298[0xd];
                local_1a0 = local_298[0xe];
                local_198 = local_298[0xf];
                local_190 = local_298[0x10];
                local_188 = local_298[0x11];
                local_78 = local_298[0x13];
                local_80 = local_298[0x12];
                FUN_00118913(__file,local_1d8,0xffffffff,0,0,0x32,0,0x100);
                local_288 = __file;
              }
              else {
                FUN_0013e944(local_298 + 0x14,__file);
              }
            }
            else {
              iVar2 = linkat(DAT_0019c268,(char *)(local_298 + 0x14),DAT_0019c268,__file,0);
              if (iVar2 != 0) {
                FUN_0013e277(local_298 + 0x14,__file);
              }
            }
          }
          else {
            FUN_0013eaa7(__file);
          }
        }
      }
LAB_0011c1e7:
    }
    local_290 = (undefined8 *)local_298[0xc];
    while (local_290 != (undefined8 *)0x0) {
      puVar1 = (undefined8 *)*local_290;
      free(local_290);
      local_290 = puVar1;
    }
    FUN_0011d55d(local_298[0x13],local_298[0x12]);
    free((void *)local_298[0xd]);
    puVar1 = (undefined8 *)*local_298;
    free(local_298);
    local_298 = puVar1;
  } while( true );
}




// Function: extract_finish @ 0x1c2ee

void extract_finish(void)

{
  FUN_00119ce9(&DAT_00181623,0);
  FUN_0011be65();
  FUN_00119ce9(&DAT_00181623,1);
  return;
}




// Function: rename_directory @ 0x1c326

undefined8 rename_directory(char *param_1,char *param_2)

{
  int iVar1;
  undefined8 uVar2;
  int *piVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  undefined1 local_25;
  int local_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = renameat(DAT_0019c268,param_1,DAT_0019c268,param_2);
  if (iVar1 == 0) {
    FUN_001194f9(param_1,param_2);
    uVar2 = 1;
  }
  else {
    piVar3 = __errno_location();
    local_24 = *piVar3;
    if ((local_24 == 2) && (iVar1 = FUN_001195aa(param_2,&local_25), iVar1 == 0)) {
      iVar1 = renameat(DAT_0019c268,param_1,DAT_0019c268,param_2);
      if (iVar1 == 0) {
        uVar2 = 1;
        goto LAB_0011c460;
      }
      piVar3 = __errno_location();
      local_24 = *piVar3;
    }
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar2 = FUN_0015e821(1,param_2);
    uVar4 = FUN_0015e821(0,param_1);
    uVar5 = gettext("Cannot rename %s to %s");
    error(0,local_24,uVar5,uVar4,uVar2);
    DAT_0019fdd0 = 2;
    uVar2 = 0;
  }
LAB_0011c460:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: x_obstack_grow @ 0x1c47d

void x_obstack_grow(long *param_1,void *param_2,ulong param_3)

{
  long lVar1;
  
  lVar1 = *param_1;
  if ((ulong)(*(long *)(lVar1 + 0x20) - *(long *)(lVar1 + 0x18)) < param_3) {
    _obstack_newchunk(lVar1,param_3);
  }
  memcpy(*(void **)(lVar1 + 0x18),param_2,param_3);
  *(ulong *)(lVar1 + 0x18) = *(long *)(lVar1 + 0x18) + param_3;
  param_1[1] = param_1[1] + param_3;
  return;
}




// Function: x_obstack_1grow @ 0x1c52b

void x_obstack_1grow(long *param_1,undefined1 param_2)

{
  long lVar1;
  undefined1 *puVar2;
  
  lVar1 = *param_1;
  if (*(long *)(lVar1 + 0x20) == *(long *)(lVar1 + 0x18)) {
    _obstack_newchunk(lVar1,1);
  }
  puVar2 = *(undefined1 **)(lVar1 + 0x18);
  *(undefined1 **)(lVar1 + 0x18) = puVar2 + 1;
  *puVar2 = param_2;
  param_1[1] = param_1[1] + 1;
  return;
}




// Function: x_obstack_blank @ 0x1c5b0

void x_obstack_blank(long *param_1,ulong param_2)

{
  long lVar1;
  
  lVar1 = *param_1;
  if ((ulong)(*(long *)(lVar1 + 0x20) - *(long *)(lVar1 + 0x18)) < param_2) {
    _obstack_newchunk(lVar1,param_2);
  }
  *(ulong *)(lVar1 + 0x18) = *(long *)(lVar1 + 0x18) + param_2;
  param_1[1] = param_1[1] + param_2;
  return;
}




// Function: xheader_keyword_deleted_p @ 0x1c63f

undefined8 xheader_keyword_deleted_p(char *param_1)

{
  int iVar1;
  undefined8 *local_10;
  
  local_10 = DAT_0019f3d8;
  while( true ) {
    if (local_10 == (undefined8 *)0x0) {
      return 0;
    }
    iVar1 = fnmatch((char *)local_10[1],param_1,0);
    if (iVar1 == 0) break;
    local_10 = (undefined8 *)*local_10;
  }
  return 1;
}




// Function: xheader_keyword_override_p @ 0x1c69c

undefined8 xheader_keyword_override_p(char *param_1)

{
  int iVar1;
  undefined8 *local_10;
  
  local_10 = DAT_0019f3e8;
  while( true ) {
    if (local_10 == (undefined8 *)0x0) {
      return 0;
    }
    iVar1 = strcmp((char *)local_10[1],param_1);
    if (iVar1 == 0) break;
    local_10 = (undefined8 *)*local_10;
  }
  return 1;
}




// Function: xheader_list_append @ 0x1c6f4

void xheader_list_append(undefined8 *param_1,undefined8 param_2,long param_3)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  
  puVar1 = (undefined8 *)FUN_00160b5b(0x18);
  uVar2 = FUN_00160cc6(param_2);
  puVar1[1] = uVar2;
  if (param_3 == 0) {
    uVar2 = 0;
  }
  else {
    uVar2 = FUN_00160cc6(param_3);
  }
  puVar1[2] = uVar2;
  *puVar1 = *param_1;
  *param_1 = puVar1;
  return;
}




// Function: xheader_list_destroy @ 0x1c772

void xheader_list_destroy(undefined8 *param_1)

{
  undefined8 *puVar1;
  undefined8 *local_18;
  
  if (param_1 != (undefined8 *)0x0) {
    local_18 = (undefined8 *)*param_1;
    while (local_18 != (undefined8 *)0x0) {
      puVar1 = (undefined8 *)*local_18;
      free((void *)local_18[1]);
      free((void *)local_18[2]);
      free(local_18);
      local_18 = puVar1;
    }
    *param_1 = 0;
  }
  return;
}




// Function: xheader_set_single_keyword @ 0x1c7ea

void xheader_set_single_keyword(undefined8 param_1)

{
  undefined8 uVar1;
  
  if (DAT_0019fdc8 != (code *)0x0) {
    (*DAT_0019fdc8)();
  }
  uVar1 = gettext("Keyword %s is unknown or not yet implemented");
  error(0,0,uVar1,param_1);
                    /* WARNING: Subroutine does not return */
  FUN_0013814c(2);
}




// Function: assign_time_option @ 0x1c846

void assign_time_option(undefined8 param_1,undefined8 *param_2,undefined8 param_3)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  char *local_30;
  undefined1 local_28 [16];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_28 = FUN_00129ff7(param_3,&local_30,0);
  cVar1 = FUN_001294f0(local_28._0_8_,local_28._8_8_);
  if ((cVar1 == '\x01') && (*local_30 == '\0')) {
    *param_2 = local_28._0_8_;
    FUN_00129547(param_1,param_3);
  }
  else {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar2 = gettext("Time stamp is out of allowed range");
    error(0,0,uVar2);
    DAT_0019fdd0 = 2;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: xheader_set_keyword_equal @ 0x1c92e

void xheader_set_keyword_equal(byte *param_1,byte *param_2)

{
  bool bVar1;
  char cVar2;
  int iVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  ushort **ppuVar6;
  byte *local_20;
  
  bVar1 = true;
  if (param_2 == param_1) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar4 = FUN_0015e84e(param_1);
    uVar5 = gettext("Malformed pax option: %s");
    error(0,0,uVar5,uVar4);
                    /* WARNING: Subroutine does not return */
    FUN_0013814c(2);
  }
  local_20 = param_2;
  if (param_2[-1] == 0x3a) {
    local_20 = param_2 + -1;
    bVar1 = false;
  }
  while ((param_1 < local_20 && (ppuVar6 = __ctype_b_loc(), ((*ppuVar6)[*local_20] & 0x2000) != 0)))
  {
    local_20 = local_20 + -1;
  }
  *local_20 = 0;
  do {
    local_20 = param_2 + 1;
    if (*local_20 == 0) break;
    ppuVar6 = __ctype_b_loc();
    param_2 = local_20;
  } while (((*ppuVar6)[*local_20] & 0x2000) != 0);
  iVar3 = strcmp((char *)param_1,"delete");
  if (iVar3 == 0) {
    cVar2 = FUN_0011d9d3(local_20);
    if (cVar2 != '\0') {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar4 = FUN_0015e84e(local_20);
      uVar5 = gettext("Pattern %s cannot be used");
      error(0,0,uVar5,uVar4);
                    /* WARNING: Subroutine does not return */
      FUN_0013814c(2);
    }
    FUN_0011c6f4(&DAT_0019f3d8,local_20,0);
  }
  else {
    iVar3 = strcmp((char *)param_1,"exthdr.name");
    if (iVar3 == 0) {
      FUN_00129547(&DAT_0019f3f8,local_20);
    }
    else {
      iVar3 = strcmp((char *)param_1,"globexthdr.name");
      if (iVar3 == 0) {
        FUN_00129547(&DAT_0019f410,local_20);
      }
      else {
        iVar3 = strcmp((char *)param_1,"exthdr.mtime");
        if (iVar3 == 0) {
          FUN_0011c846(&DAT_0019f400,&DAT_0019f408,local_20);
        }
        else {
          iVar3 = strcmp((char *)param_1,"globexthdr.mtime");
          if (iVar3 == 0) {
            FUN_0011c846(&DAT_0019f418,&DAT_0019f420,local_20);
          }
          else {
            cVar2 = FUN_0011da4b(param_1);
            if (cVar2 != '\0') {
              if (DAT_0019fdc8 != (code *)0x0) {
                (*DAT_0019fdc8)();
              }
              uVar4 = gettext("Keyword %s cannot be overridden");
              error(0,0,uVar4,param_1);
                    /* WARNING: Subroutine does not return */
              FUN_0013814c(2);
            }
            if (bVar1) {
              FUN_0011c6f4(&DAT_0019f3e0,param_1,local_20);
            }
            else {
              FUN_0011c6f4(&DAT_0019f3e8,param_1,local_20);
            }
          }
        }
      }
    }
  }
  return;
}




// Function: xheader_set_option @ 0x1cc78

void xheader_set_option(char *param_1)

{
  char *pcVar1;
  char *local_18;
  
  local_18 = strtok(param_1,",");
  while (local_18 != (char *)0x0) {
    pcVar1 = strchr(local_18,0x3d);
    if (pcVar1 == (char *)0x0) {
      FUN_0011c7ea(local_18);
    }
    FUN_0011c92e(local_18,pcVar1);
    local_18 = strtok((char *)0x0,",");
  }
  return;
}




// Function: xheader_format_name @ 0x1cd02

char * xheader_format_name(undefined8 *param_1,char *param_2,undefined8 param_3)

{
  char *pcVar1;
  char cVar2;
  __pid_t _Var3;
  size_t sVar4;
  char *pcVar5;
  char *pcVar6;
  long in_FS_OFFSET;
  undefined1 *local_90;
  char *local_88;
  char *local_80;
  void *local_78;
  char *local_70;
  char *local_68;
  char *local_60;
  char *local_58;
  undefined1 local_48 [20];
  undefined1 auStack_34 [12];
  undefined1 local_28 [20];
  undefined1 auStack_14 [4];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_78 = (void *)0x0;
  local_70 = (char *)0x0;
  local_68 = (char *)0x0;
  local_60 = (char *)0x0;
  local_58 = (char *)0x0;
  local_90 = (undefined1 *)0x0;
  for (local_80 = param_2; *local_80 != '\0'; local_80 = local_80 + 1) {
    if ((*local_80 == '%') && (local_80[1] != '\0')) {
      local_80 = local_80 + 1;
      cVar2 = *local_80;
      if (cVar2 == 'p') {
        _Var3 = getpid();
        local_60 = (char *)FUN_00154e5a((long)_Var3,local_48);
        local_90 = auStack_34 + (long)(local_90 + -(long)local_60);
      }
      else if (cVar2 < 'q') {
        if (cVar2 == 'n') {
          local_58 = (char *)FUN_00154e5a(param_3,local_28);
          local_90 = auStack_14 + (long)(local_90 + -(long)local_58);
        }
        else {
          if ('n' < cVar2) goto LAB_0011cef4;
          if (cVar2 == 'f') {
            if (param_1 != (undefined8 *)0x0) {
              local_68 = (char *)FUN_0014df20(*param_1);
              sVar4 = strlen(local_68);
              local_90 = local_90 + sVar4;
            }
          }
          else {
            if ('f' < cVar2) goto LAB_0011cef4;
            if (cVar2 == '%') {
              local_90 = local_90 + 1;
            }
            else {
              if (cVar2 != 'd') goto LAB_0011cef4;
              if (param_1 != (undefined8 *)0x0) {
                if (local_78 == (void *)0x0) {
                  local_78 = (void *)FUN_0014e10c(*param_1);
                }
                local_70 = (char *)FUN_0013edd9(local_78,0,DAT_0019f878);
                sVar4 = strlen(local_70);
                local_90 = local_90 + sVar4;
              }
            }
          }
        }
      }
      else {
LAB_0011cef4:
        local_90 = local_90 + 2;
      }
    }
    else {
      local_90 = local_90 + 1;
    }
  }
  pcVar5 = (char *)FUN_00160b5b(local_90 + 1);
  pcVar6 = pcVar5;
  pcVar1 = param_2;
LAB_0011d0c7:
  while( true ) {
    while( true ) {
      local_80 = pcVar1;
      local_88 = pcVar6;
      if (*local_80 == '\0') {
        free(local_78);
        for (; (pcVar5 < local_88 && (local_88[-1] == '/')); local_88 = local_88 + -1) {
        }
        *local_88 = '\0';
        if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
          return pcVar5;
        }
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      if (*local_80 == '%') break;
      *local_88 = *local_80;
      pcVar6 = local_88 + 1;
      pcVar1 = local_80 + 1;
    }
    cVar2 = local_80[1];
    if (cVar2 != 'p') break;
    pcVar6 = stpcpy(local_88,local_60);
    pcVar1 = local_80 + 2;
  }
  if (cVar2 < 'q') {
    if (cVar2 == 'n') {
      pcVar6 = stpcpy(local_88,local_58);
      pcVar1 = local_80 + 2;
      goto LAB_0011d0c7;
    }
    if ('n' < cVar2) goto LAB_0011d063;
    if (cVar2 == 'f') {
      if (local_68 != (char *)0x0) {
        local_88 = stpcpy(local_88,local_68);
      }
      pcVar6 = local_88;
      pcVar1 = local_80 + 2;
      goto LAB_0011d0c7;
    }
    if (cVar2 < 'g') {
      if (cVar2 == '%') {
        *local_88 = *local_80;
        pcVar6 = local_88 + 1;
        pcVar1 = local_80 + 2;
        goto LAB_0011d0c7;
      }
      if (cVar2 == 'd') {
        if (local_70 != (char *)0x0) {
          local_88 = stpcpy(local_88,local_70);
        }
        pcVar6 = local_88;
        pcVar1 = local_80 + 2;
        goto LAB_0011d0c7;
      }
    }
  }
LAB_0011d063:
  pcVar1 = local_80 + 1;
  *local_88 = *local_80;
  pcVar6 = local_88 + 1;
  if (*pcVar1 != '\0') {
    local_88[1] = *pcVar1;
    pcVar6 = local_88 + 2;
    pcVar1 = local_80 + 2;
  }
  goto LAB_0011d0c7;
}




// Function: xheader_xhdr_name @ 0x1d123

void xheader_xhdr_name(undefined8 param_1)

{
  if (DAT_0019f3f8 == 0) {
    FUN_00129547(&DAT_0019f3f8,(&PTR_s__d_PaxHeaders__f_0019c240)[(int)(uint)DAT_0019f9c8]);
  }
  FUN_0011cd02(param_1,DAT_0019f3f8,0);
  return;
}




// Function: xheader_ghdr_name @ 0x1d18d

void xheader_ghdr_name(void)

{
  char *__s;
  size_t sVar1;
  size_t sVar2;
  char *local_30;
  
  if (DAT_0019f410 == (char *)0x0) {
    __s = (&PTR_s__d_PaxHeaders__f_0019c240)[(long)(int)(uint)DAT_0019f9c8 + 2];
    local_30 = getenv("TMPDIR");
    if (local_30 == (char *)0x0) {
      local_30 = "/tmp";
    }
    sVar1 = strlen(local_30);
    sVar2 = strlen(__s);
    DAT_0019f410 = (char *)FUN_00160b5b(sVar2 + sVar1 + 1);
    strcpy(DAT_0019f410,local_30);
    strcat(DAT_0019f410,__s);
  }
  FUN_0011cd02(0,DAT_0019f410,DAT_0019f3d0 + 1);
  return;
}




// Function: xheader_write @ 0x1d280

void xheader_write(char param_1,undefined8 param_2,undefined8 param_3,long param_4)

{
  long lVar1;
  void *__dest;
  undefined8 local_40;
  ulong local_28;
  void *local_20;
  ulong local_18;
  
  local_28 = *(ulong *)(param_4 + 8);
  local_40 = param_3;
  if (param_1 == 'g') {
    if (DAT_0019f418 != 0) {
      local_40 = DAT_0019f420;
    }
  }
  else if ((param_1 == 'x') && (DAT_0019f400 != 0)) {
    local_40 = DAT_0019f408;
  }
  lVar1 = FUN_00112e0f(param_2,local_28,local_40);
  *(char *)(lVar1 + 0x9c) = param_1;
  FUN_0011418c(lVar1);
  local_20 = *(void **)(param_4 + 0x10);
  do {
    __dest = (void *)FUN_0010bb4b();
    local_18 = 0x200;
    if (local_28 < 0x200) {
      local_18 = local_28;
    }
    memcpy(__dest,local_20,local_18);
    if (local_18 < 0x200) {
      memset((void *)((long)__dest + local_18),0,0x200 - local_18);
    }
    local_20 = (void *)((long)local_20 + local_18);
    local_28 = local_28 - local_18;
    FUN_0010bba7(__dest);
  } while (local_28 != 0);
  FUN_0011e785(param_4);
  if (param_1 == 'g') {
    DAT_0019f3d0 = DAT_0019f3d0 + 1;
  }
  return;
}




// Function: xheader_write_global @ 0x1d3e6

void xheader_write_global(long *param_1)

{
  void *__ptr;
  undefined8 *local_18;
  
  if (DAT_0019f3e0 != (undefined8 *)0x0) {
    FUN_0011e173(param_1);
    for (local_18 = DAT_0019f3e0; local_18 != (undefined8 *)0x0; local_18 = (undefined8 *)*local_18)
    {
      FUN_0011ecd7(local_18[2],local_18[1],param_1);
    }
  }
  if (*param_1 != 0) {
    FUN_0011e672(param_1);
    __ptr = (void *)FUN_0011d18d();
    FUN_0011d280(0x67,__ptr,DAT_0019f9e0,param_1);
    free(__ptr);
  }
  return;
}




// Function: xheader_forbid_global @ 0x1d498

void xheader_forbid_global(void)

{
  undefined8 uVar1;
  
  if (DAT_0019f3e0 != 0) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar1 = gettext("can\'t update global extended header record");
    error(0,0,uVar1);
                    /* WARNING: Subroutine does not return */
    FUN_0013814c(2);
  }
  return;
}




// Function: xheader_xattr_init @ 0x1d4f4

void xheader_xattr_init(long param_1)

{
  *(undefined8 *)(param_1 + 0x160) = 0;
  *(undefined8 *)(param_1 + 0x158) = 0;
  *(undefined8 *)(param_1 + 0x38) = 0;
  *(undefined8 *)(param_1 + 0x40) = 0;
  *(undefined8 *)(param_1 + 0x48) = 0;
  *(undefined8 *)(param_1 + 0x50) = 0;
  *(undefined8 *)(param_1 + 0x30) = 0;
  return;
}




// Function: xheader_xattr_free @ 0x1d55d

void xheader_xattr_free(void *param_1,ulong param_2)

{
  ulong local_10;
  
  for (local_10 = 0; local_10 < param_2; local_10 = local_10 + 1) {
    free(*(void **)((long)param_1 + local_10 * 0x18));
    free(*(void **)((long)param_1 + local_10 * 0x18 + 8));
  }
  free(param_1);
  return;
}




// Function: xheader_xattr__add @ 0x1d5e6

void xheader_xattr__add(long *param_1,long *param_2,undefined8 param_3,undefined8 param_4,long param_5)

{
  long lVar1;
  long lVar2;
  undefined8 uVar3;
  
  lVar1 = *param_2;
  *param_2 = lVar1 + 1;
  lVar2 = FUN_00160b8d(*param_1,*param_2 * 0x18);
  *param_1 = lVar2;
  lVar2 = *param_1;
  uVar3 = FUN_00160cc6(param_3);
  *(undefined8 *)(lVar2 + lVar1 * 0x18) = uVar3;
  lVar2 = *param_1;
  uVar3 = FUN_00160c8e(param_4,param_5 + 1);
  *(undefined8 *)(lVar2 + lVar1 * 0x18 + 8) = uVar3;
  *(long *)(*param_1 + lVar1 * 0x18 + 0x10) = param_5;
  return;
}




// Function: xattr_decode_keyword @ 0x1d6da

void xattr_decode_keyword(char *param_1)

{
  char *local_18;
  char *local_10;
  
  local_18 = param_1;
  local_10 = param_1;
  do {
    if (*local_18 == '%') {
      if ((local_18[1] == '3') && (local_18[2] == 'D')) {
        *local_10 = '=';
        local_18 = local_18 + 3;
      }
      else {
        if ((local_18[1] != '2') || (local_18[2] != '5')) goto LAB_0011d763;
        *local_10 = '%';
        local_18 = local_18 + 3;
      }
    }
    else {
LAB_0011d763:
      *local_10 = *local_18;
      if (*local_18 == '\0') {
        return;
      }
      local_18 = local_18 + 1;
    }
    local_10 = local_10 + 1;
  } while( true );
}




// Function: xheader_xattr_add @ 0x1d78e

void xheader_xattr_add(long param_1,char *param_2,undefined8 param_3,undefined8 param_4)

{
  size_t sVar1;
  size_t sVar2;
  char *__dest;
  char *__dest_00;
  
  sVar1 = strlen(param_2);
  sVar2 = strlen("SCHILY.xattr.");
  __dest = (char *)FUN_00160b5b(sVar2 + sVar1 + 1);
  __dest_00 = stpcpy(__dest,"SCHILY.xattr.");
  stpcpy(__dest_00,param_2);
  FUN_0011d5e6(param_1 + 0x160,param_1 + 0x158,__dest,param_3,param_4);
  free(__dest);
  return;
}




// Function: xheader_xattr_copy @ 0x1d854

void xheader_xattr_copy(long param_1,undefined8 *param_2,undefined8 *param_3)

{
  undefined8 local_28;
  
  local_28 = 0;
  *param_2 = 0;
  *param_3 = 0;
  for (; local_28 < *(ulong *)(param_1 + 0x158); local_28 = local_28 + 1) {
    FUN_0011d5e6(param_2,param_3,*(undefined8 *)(local_28 * 0x18 + *(long *)(param_1 + 0x160)),
                 *(undefined8 *)(local_28 * 0x18 + *(long *)(param_1 + 0x160) + 8),
                 *(undefined8 *)(local_28 * 0x18 + *(long *)(param_1 + 0x160) + 0x10));
  }
  return;
}




// Function: locate_handler @ 0x1d940

undefined ** locate_handler(char *param_1)

{
  int iVar1;
  size_t __n;
  undefined **local_10;
  
  local_10 = &PTR_s_atime_0019a260;
  while( true ) {
    if (*local_10 == (undefined *)0x0) {
      return (undefined **)0x0;
    }
    if (*(char *)((long)local_10 + 0x1c) == '\0') {
      iVar1 = strcmp(*local_10,param_1);
    }
    else {
      __n = strlen(*local_10);
      iVar1 = strncmp(*local_10,param_1,__n);
    }
    if (iVar1 == 0) break;
    local_10 = local_10 + 4;
  }
  return local_10;
}




// Function: xheader_protected_pattern_p @ 0x1d9d3

undefined8 xheader_protected_pattern_p(char *param_1)

{
  int iVar1;
  undefined **local_10;
  
  local_10 = &PTR_s_atime_0019a260;
  while( true ) {
    if (*local_10 == (undefined *)0x0) {
      return 0;
    }
    if (((*(char *)((long)local_10 + 0x1c) != '\x01') && (((ulong)local_10[3] & 1) != 0)) &&
       (iVar1 = fnmatch(param_1,*local_10,0), iVar1 == 0)) break;
    local_10 = local_10 + 4;
  }
  return 1;
}




// Function: xheader_protected_keyword_p @ 0x1da4b

undefined8 xheader_protected_keyword_p(char *param_1)

{
  int iVar1;
  undefined **local_10;
  
  local_10 = &PTR_s_atime_0019a260;
  while( true ) {
    if (*local_10 == (undefined *)0x0) {
      return 0;
    }
    if (((*(char *)((long)local_10 + 0x1c) != '\x01') && (((ulong)local_10[3] & 1) != 0)) &&
       (iVar1 = strcmp(*local_10,param_1), iVar1 == 0)) break;
    local_10 = local_10 + 4;
  }
  return 1;
}




// Function: decode_record @ 0x1dabe

undefined8 decode_record(long param_1,long *param_2,code *param_3,undefined8 param_4)

{
  int iVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  char *local_48;
  char *local_40;
  char *local_38;
  ulong local_30;
  uintmax_t local_28;
  char *local_20;
  char *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_38 = (char *)*param_2;
  local_30 = (*(long *)(param_1 + 8) + *(long *)(param_1 + 0x10)) - (long)local_38;
  for (local_40 = local_38; (*local_40 == ' ' || (*local_40 == '\t')); local_40 = local_40 + 1) {
  }
  if ((int)*local_40 - 0x30U < 10) {
    local_28 = strtoumax(local_40,&local_48,10);
    if (local_30 < local_28) {
      iVar1 = (int)local_40;
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar2 = gettext("Extended header length %*s is out of range");
      error(0,0,uVar2,(int)local_48 - iVar1,local_40);
      DAT_0019fdd0 = 2;
      uVar2 = 0;
    }
    else {
      local_20 = local_38 + local_28;
      for (local_40 = local_48; (*local_40 == ' ' || (*local_40 == '\t')); local_40 = local_40 + 1)
      {
      }
      if (local_40 == local_48) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar2 = gettext("Malformed extended header: missing blank after length");
        error(0,0,uVar2);
        DAT_0019fdd0 = 2;
        uVar2 = 0;
      }
      else {
        local_18 = local_40;
        local_40 = strchr(local_40,0x3d);
        if ((local_40 == (char *)0x0) || (local_20 <= local_40)) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar2 = gettext("Malformed extended header: missing equal sign");
          error(0,0,uVar2);
          DAT_0019fdd0 = 2;
          uVar2 = 0;
        }
        else if (local_20[-1] == '\n') {
          local_20[-1] = '\0';
          *local_40 = local_20[-1];
          (*param_3)(param_4,local_18,local_40 + 1,local_20 + (-2 - (long)local_40));
          *local_40 = '=';
          local_20[-1] = '\n';
          *param_2 = (long)local_20;
          uVar2 = 1;
        }
        else {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar2 = gettext("Malformed extended header: missing newline");
          error(0,0,uVar2);
          DAT_0019fdd0 = 2;
          uVar2 = 0;
        }
      }
    }
  }
  else {
    if (*local_40 != '\0') {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar2 = gettext("Malformed extended header: missing length");
      error(0,0,uVar2);
      DAT_0019fdd0 = 2;
    }
    uVar2 = 0;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: run_override_list @ 0x1de07

void run_override_list(undefined8 *param_1,undefined8 param_2)

{
  code *pcVar1;
  undefined8 *puVar2;
  size_t sVar3;
  undefined8 *local_30;
  
  for (local_30 = param_1; local_30 != (undefined8 *)0x0; local_30 = (undefined8 *)*local_30) {
    puVar2 = (undefined8 *)FUN_0011d940(local_30[1]);
    if (puVar2 != (undefined8 *)0x0) {
      pcVar1 = (code *)puVar2[2];
      sVar3 = strlen((char *)local_30[2]);
      (*pcVar1)(param_2,*puVar2,local_30[2],sVar3);
    }
  }
  return;
}




// Function: decx @ 0x1de86

void decx(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  char cVar1;
  long lVar2;
  undefined8 uVar3;
  
  cVar1 = FUN_0011c63f(param_2);
  if (cVar1 == '\0') {
    cVar1 = FUN_0011c69c(param_2);
    if (cVar1 == '\0') {
      lVar2 = FUN_0011d940(param_2);
      if (lVar2 == 0) {
        if ((DAT_0019ec84 & 0x20000) != 0) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar3 = gettext("Ignoring unknown extended header keyword \'%s\'");
          error(0,0,uVar3,param_2);
        }
      }
      else {
        (**(code **)(lVar2 + 0x10))(param_1,param_2,param_3,param_4);
      }
    }
  }
  return;
}




// Function: xheader_decode @ 0x1df5f

void xheader_decode(long param_1)

{
  char cVar1;
  long in_FS_OFFSET;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0011de07(DAT_0019f3e0,param_1);
  FUN_0011de07(DAT_0019f3f0,param_1);
  if (*(long *)(param_1 + 0x170) != 0) {
    local_18 = *(long *)(param_1 + 0x178) + 0x200;
    do {
      cVar1 = FUN_0011dabe(param_1 + 0x168,&local_18,FUN_0011de86,param_1);
    } while (cVar1 != '\0');
  }
  FUN_0011de07(DAT_0019f3e8,param_1);
  *(undefined8 *)(param_1 + 0x118) = *(undefined8 *)(param_1 + 0x88);
  if (*(char *)(param_1 + 0x150) != '\0') {
    *(undefined8 *)(param_1 + 0x88) = *(undefined8 *)(param_1 + 0x148);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: decg @ 0x1e063

void decg(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  long lVar1;
  
  lVar1 = FUN_0011d940(param_2);
  if ((lVar1 == 0) || ((*(uint *)(lVar1 + 0x18) & 2) == 0)) {
    FUN_0011c6f4(param_1,param_2,param_3);
  }
  else {
    (**(code **)(lVar1 + 0x10))(param_1,param_2,param_3,param_4);
  }
  return;
}




// Function: xheader_decode_global @ 0x1e0e7

void xheader_decode_global(long param_1)

{
  char cVar1;
  long in_FS_OFFSET;
  long local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (*(long *)(param_1 + 8) != 0) {
    local_18 = *(long *)(param_1 + 0x10) + 0x200;
    FUN_0011c772(&DAT_0019f3f0);
    do {
      cVar1 = FUN_0011dabe(param_1,&local_18,FUN_0011e063,&DAT_0019f3f0);
    } while (cVar1 != '\0');
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: xheader_init @ 0x1e173

void xheader_init(long *param_1)

{
  long lVar1;
  
  if (*param_1 == 0) {
    lVar1 = FUN_00160b5b(0x58);
    *param_1 = lVar1;
    _obstack_begin(*param_1,0,0,FUN_00160b5b,PTR_free_0019bfc0);
  }
  return;
}




// Function: xheader_store @ 0x1e1d3

void xheader_store(undefined8 param_1,long param_2,undefined8 param_3)

{
  char cVar1;
  long lVar2;
  
  if ((((*(long *)(param_2 + 0x178) == 0) && (lVar2 = FUN_0011d940(param_1), lVar2 != 0)) &&
      (*(long *)(lVar2 + 8) != 0)) && (cVar1 = FUN_0011c63f(param_1), cVar1 == '\0')) {
    FUN_0011e173(param_2 + 0x168);
    cVar1 = FUN_0011c69c(param_1);
    if (cVar1 != '\x01') {
      (**(code **)(lVar2 + 8))(param_2,param_1,param_2 + 0x168,param_3);
    }
  }
  return;
}




// Function: xheader_read @ 0x1e28e

void xheader_read(long param_1,void *param_2,ulong param_3)

{
  ulong uVar1;
  undefined8 uVar2;
  ulong local_30;
  void *local_28;
  long local_18;
  size_t local_10;
  
  local_18 = 0;
  local_30 = param_3;
  if ((long)param_3 < 0) {
    local_30 = 0;
  }
  if (0xfffffffffffffdfe < local_30) {
    FUN_0011783c();
  }
  uVar1 = local_30 + 0x200;
  *(ulong *)(param_1 + 8) = uVar1;
  uVar2 = FUN_00160b5b(local_30 + 0x201);
  *(undefined8 *)(param_1 + 0x10) = uVar2;
  *(undefined1 *)(uVar1 + *(long *)(param_1 + 0x10)) = 0;
  local_30 = uVar1;
  local_28 = param_2;
  while( true ) {
    local_10 = local_30;
    if (0x200 < local_30) {
      local_10 = 0x200;
    }
    if (local_28 == (void *)0x0) break;
    memcpy((void *)(*(long *)(param_1 + 0x10) + local_18),local_28,local_10);
    FUN_0010bba7(local_28);
    local_28 = (void *)FUN_0010bb4b();
    local_18 = local_18 + local_10;
    local_30 = local_30 - local_10;
    if ((long)local_30 < 1) {
      return;
    }
  }
  if (DAT_0019fdc8 != (code *)0x0) {
    (*DAT_0019fdc8)();
  }
  uVar2 = gettext("Unexpected EOF in archive");
  error(0,0,uVar2);
                    /* WARNING: Subroutine does not return */
  FUN_001177f4();
}




// Function: xattr_encode_keyword @ 0x1e3ca

undefined1 * xattr_encode_keyword(char *param_1)

{
  char cVar1;
  char *local_20;
  long local_10;
  
  if (DAT_0019f428 == (undefined1 *)0x0) {
    DAT_0019f430 = 0x100;
    DAT_0019f428 = (undefined1 *)FUN_00160b5b(0x100);
  }
  else {
    *DAT_0019f428 = 0;
  }
  local_10 = 0;
  for (local_20 = param_1; *local_20 != '\0'; local_20 = local_20 + 1) {
    cVar1 = *local_20;
    if (DAT_0019f430 <= local_10 + 2U) {
      DAT_0019f428 = (undefined1 *)FUN_00160bd8(DAT_0019f428,&DAT_0019f430);
    }
    if (cVar1 == '%') {
      strcpy(DAT_0019f428 + local_10,"%25");
      local_10 = local_10 + 2;
    }
    else if (cVar1 == '=') {
      strcpy(DAT_0019f428 + local_10,"%3D");
      local_10 = local_10 + 2;
    }
    else {
      DAT_0019f428[local_10] = cVar1;
    }
    local_10 = local_10 + 1;
  }
  DAT_0019f428[local_10] = 0;
  return DAT_0019f428;
}




// Function: xheader_print_n @ 0x1e4ff

void xheader_print_n(undefined8 param_1,undefined8 param_2,undefined8 param_3,long param_4)

{
  char *__s;
  size_t sVar1;
  long lVar2;
  undefined1 *puVar3;
  long in_FS_OFFSET;
  bool bVar4;
  undefined1 *local_50;
  undefined1 local_28 [20];
  undefined1 auStack_14 [4];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_50 = (undefined1 *)0x0;
  __s = (char *)FUN_0011e3ca(param_2);
  sVar1 = strlen(__s);
  do {
    lVar2 = FUN_00154e5a(local_50 + param_4 + sVar1 + 3,local_28);
    puVar3 = auStack_14 + -lVar2;
    bVar4 = puVar3 != local_50;
    local_50 = puVar3;
  } while (bVar4);
  FUN_0011c47d(param_1,lVar2,puVar3);
  FUN_0011c52b(param_1,0x20);
  FUN_0011c47d(param_1,__s,sVar1);
  FUN_0011c52b(param_1,0x3d);
  FUN_0011c47d(param_1,param_3,param_4);
  FUN_0011c52b(param_1,10);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: xheader_print @ 0x1e634

void xheader_print(undefined8 param_1,undefined8 param_2,char *param_3)

{
  size_t sVar1;
  
  sVar1 = strlen(param_3);
  FUN_0011e4ff(param_1,param_2,param_3,sVar1);
  return;
}




// Function: xheader_finish @ 0x1e672

void xheader_finish(long *param_1)

{
  long lVar1;
  long lVar2;
  undefined8 *local_20;
  
  for (local_20 = DAT_0019f3e8; local_20 != (undefined8 *)0x0; local_20 = (undefined8 *)*local_20) {
    FUN_0011ecd7(local_20[2],local_20[1],param_1);
  }
  lVar1 = *param_1;
  lVar2 = *(long *)(lVar1 + 0x10);
  if (lVar2 == *(long *)(lVar1 + 0x18)) {
    *(byte *)(lVar1 + 0x50) = *(byte *)(lVar1 + 0x50) | 2;
  }
  *(ulong *)(lVar1 + 0x18) =
       ~*(ulong *)(lVar1 + 0x30) & *(long *)(lVar1 + 0x18) + *(long *)(lVar1 + 0x30);
  if ((ulong)(*(long *)(lVar1 + 0x20) - *(long *)(lVar1 + 8)) <
      (ulong)(*(long *)(lVar1 + 0x18) - *(long *)(lVar1 + 8))) {
    *(undefined8 *)(lVar1 + 0x18) = *(undefined8 *)(lVar1 + 0x20);
  }
  *(undefined8 *)(lVar1 + 0x10) = *(undefined8 *)(lVar1 + 0x18);
  param_1[2] = lVar2;
  return;
}




// Function: xheader_destroy @ 0x1e785

/* WARNING: Removing unreachable block (ram,0x0011e7c2) */
/* WARNING: Removing unreachable block (ram,0x0011e7d0) */

void xheader_destroy(long *param_1)

{
  if (*param_1 == 0) {
    free((void *)param_1[2]);
  }
  else {
    _obstack_free(*param_1,0);
    free((void *)*param_1);
    *param_1 = 0;
  }
  param_1[2] = 0;
  param_1[1] = 0;
  return;
}




// Function: xheader_string_begin @ 0x1e848

void xheader_string_begin(long param_1)

{
  *(undefined8 *)(param_1 + 0x18) = 0;
  return;
}




// Function: xheader_string_add @ 0x1e863

void xheader_string_add(long param_1,char *param_2)

{
  long lVar1;
  size_t sVar2;
  
  if (*(long *)(param_1 + 0x10) == 0) {
    FUN_0011e173(param_1);
    lVar1 = *(long *)(param_1 + 0x18);
    sVar2 = strlen(param_2);
    *(size_t *)(param_1 + 0x18) = lVar1 + sVar2;
    sVar2 = strlen(param_2);
    FUN_0011c47d(param_1,param_2,sVar2);
  }
  return;
}




// Function: xheader_string_end @ 0x1e8dc

/* WARNING: Removing unreachable block (ram,0x0011e9bd) */
/* WARNING: Removing unreachable block (ram,0x0011e9c9) */
/* WARNING: Removing unreachable block (ram,0x0011e9d2) */
/* WARNING: Removing unreachable block (ram,0x0011ea40) */
/* WARNING: Removing unreachable block (ram,0x0011ea4e) */
/* WARNING: Removing unreachable block (ram,0x0011eab1) */
/* WARNING: Removing unreachable block (ram,0x0011eac1) */
/* WARNING: Removing unreachable block (ram,0x0011eae7) */
/* WARNING: Removing unreachable block (ram,0x0011eb13) */
/* WARNING: Removing unreachable block (ram,0x0011eaf5) */
/* WARNING: Removing unreachable block (ram,0x0011eb26) */

undefined8 xheader_string_end(long *param_1,char *param_2)

{
  long lVar1;
  undefined8 uVar2;
  size_t sVar3;
  char *pcVar4;
  char *pcVar5;
  char *__dest;
  long in_FS_OFFSET;
  bool bVar6;
  char *local_78;
  undefined1 local_28 [20];
  char acStack_14 [4];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_78 = (char *)0x0;
  if (param_1[2] == 0) {
    FUN_0011e173(param_1);
    sVar3 = strlen(param_2);
    lVar1 = param_1[3];
    do {
      pcVar4 = (char *)FUN_00154e5a(local_78 + lVar1 + sVar3 + 3,local_28);
      pcVar5 = acStack_14 + -(long)pcVar4;
      bVar6 = pcVar5 != local_78;
      local_78 = pcVar5;
    } while (bVar6);
    sVar3 = strlen(param_2);
    pcVar5 = pcVar5 + sVar3 + 2;
    FUN_0011c5b0(param_1,pcVar5);
    FUN_0011c52b(param_1,10);
    __dest = (char *)((-(long)pcVar5 - param_1[3]) + -1 + *(long *)(*param_1 + 0x18));
    memmove(__dest + (long)pcVar5,__dest,param_1[3]);
    pcVar4 = stpcpy(__dest,pcVar4);
    *pcVar4 = ' ';
    pcVar4 = stpcpy(pcVar4 + 1,param_2);
    *pcVar4 = '=';
    uVar2 = 1;
  }
  else {
    uVar2 = 0;
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: out_of_range_header @ 0x1ec02

void out_of_range_header(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined1 local_48 [32];
  undefined1 local_28 [24];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_00154bde(param_3,local_48);
  uVar2 = FUN_00154e5a(param_4,local_28);
  if (DAT_0019fdc8 != (code *)0x0) {
    (*DAT_0019fdc8)();
  }
  uVar3 = gettext("Extended header %s=%s is out of range %s..%s");
  error(0,0,uVar3,param_1,param_2,uVar1,uVar2);
  DAT_0019fdd0 = 2;
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: code_string @ 0x1ecd7

void code_string(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  void *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0013c8a0(1,param_1,&local_18);
  if (cVar1 != '\x01') {
    local_18 = (void *)FUN_00160cc6(param_1);
  }
  FUN_0011e634(param_3,param_2,local_18);
  free(local_18);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: decode_string @ 0x1ed64

void decode_string(long *param_1,undefined8 param_2)

{
  char cVar1;
  
  if (*param_1 != 0) {
    free((void *)*param_1);
    *param_1 = 0;
  }
  cVar1 = FUN_0013c8a0(0,param_2,param_1);
  if (cVar1 != '\x01') {
    FUN_00129547(param_1,param_2);
  }
  return;
}




// Function: code_time @ 0x1edd0

void code_time(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  undefined8 uVar1;
  long in_FS_OFFSET;
  undefined1 local_38 [40];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_00129f2e(param_1,param_2,local_38);
  FUN_0011e634(param_4,param_3,uVar1);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: decode_time @ 0x1ee4e

undefined8 decode_time(undefined1 (*param_1) [16],char *param_2,undefined8 param_3)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  char *local_30;
  undefined1 local_28 [16];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_28 = FUN_00129ff7(param_2,&local_30,1);
  cVar1 = FUN_001294f0(local_28._0_8_,local_28._8_8_);
  if (cVar1 == '\x01') {
    *param_1 = local_28;
    uVar2 = 1;
  }
  else {
    if ((param_2 < local_30) && (*local_30 == '\0')) {
      FUN_0011ec02(param_3,param_2,0x8000000000000000,0x7fffffffffffffff);
    }
    else {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar2 = gettext("Malformed extended header: invalid %s=%s");
      error(0,0,uVar2,param_3,param_2);
      DAT_0019fdd0 = 2;
    }
    uVar2 = 0;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: code_signed_num @ 0x1ef79

void code_signed_num(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5)

{
  undefined8 uVar1;
  long in_FS_OFFSET;
  undefined1 local_28 [24];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  uVar1 = FUN_00129c96(param_1,param_3,param_4,local_28);
  FUN_0011e634(param_5,param_2,uVar1);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: code_num @ 0x1efed

void code_num(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011ef79(param_1,param_2,0,0xffffffffffffffff,param_3);
  return;
}




// Function: decode_signed_num @ 0x1f02b

undefined8
decode_signed_num(undefined8 *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
            undefined8 param_5)

{
  int *piVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  char *local_20;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_18 = FUN_00129cfc(param_2,&local_20,param_3,param_4);
  piVar1 = __errno_location();
  if ((*piVar1 == 0x16) || (*local_20 != '\0')) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar2 = gettext("Malformed extended header: invalid %s=%s");
    error(0,0,uVar2,param_5,param_2);
    DAT_0019fdd0 = 2;
    uVar2 = 0;
  }
  else {
    piVar1 = __errno_location();
    if (*piVar1 == 0x22) {
      FUN_0011ec02(param_5,param_2,param_3,param_4);
      uVar2 = 0;
    }
    else {
      *param_1 = local_18;
      uVar2 = 1;
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: dummy_coder @ 0x1f1bd

void dummy_coder(void)

{
  return;
}




// Function: dummy_decoder @ 0x1f1d8

void dummy_decoder(void)

{
  return;
}




// Function: atime_coder @ 0x1f1f3

void atime_coder(long param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6)

{
  FUN_0011edd0(*(undefined8 *)(param_1 + 0xe8),*(undefined8 *)(param_1 + 0xf0),param_2,param_3,
               param_5,param_6,param_4);
  return;
}




// Function: atime_decoder @ 0x1f237

void atime_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_28;
  undefined8 local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011ee4e(&local_28,param_3,param_2);
  if (cVar1 != '\0') {
    *(undefined8 *)(param_1 + 0xe8) = local_28;
    *(undefined8 *)(param_1 + 0xf0) = local_20;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: gid_coder @ 0x1f2ae

void gid_coder(long param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6)

{
  FUN_0011ef79(*(undefined4 *)(param_1 + 0x78),param_2,0,0xffffffff,param_3,param_6,param_4);
  return;
}




// Function: gid_decoder @ 0x1f2f6

void gid_decoder(long param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined4 local_18 [2];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011f02b(local_18,param_3,0,0xffffffff,param_2,param_6,param_4);
  if (cVar1 != '\0') {
    *(undefined4 *)(param_1 + 0x78) = local_18[0];
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: gname_coder @ 0x1f36d

void gname_coder(long param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011ecd7(*(undefined8 *)(param_1 + 0x28),param_2,param_3);
  return;
}




// Function: gname_decoder @ 0x1f3a7

void gname_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011ed64(param_1 + 0x28,param_3);
  return;
}




// Function: linkpath_coder @ 0x1f3dd

void linkpath_coder(long param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011ecd7(*(undefined8 *)(param_1 + 0x18),param_2,param_3);
  return;
}




// Function: linkpath_decoder @ 0x1f417

void linkpath_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011ed64(param_1 + 0x18,param_3);
  return;
}




// Function: ctime_coder @ 0x1f44d

void ctime_coder(long param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6)

{
  FUN_0011edd0(*(undefined8 *)(param_1 + 0x108),*(undefined8 *)(param_1 + 0x110),param_2,param_3,
               param_5,param_6,param_4);
  return;
}




// Function: ctime_decoder @ 0x1f491

void ctime_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_28;
  undefined8 local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011ee4e(&local_28,param_3,param_2);
  if (cVar1 != '\0') {
    *(undefined8 *)(param_1 + 0x108) = local_28;
    *(undefined8 *)(param_1 + 0x110) = local_20;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: mtime_coder @ 0x1f508

void mtime_coder(long param_1,undefined8 param_2,undefined8 param_3,undefined8 *param_4)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  if (param_4 == (undefined8 *)0x0) {
    uVar2 = *(undefined8 *)(param_1 + 0x100);
    uVar1 = *(undefined8 *)(param_1 + 0xf8);
  }
  else {
    uVar2 = param_4[1];
    uVar1 = *param_4;
  }
  FUN_0011edd0(uVar1,uVar2,param_2,param_3,uVar1,uVar2);
  return;
}




// Function: mtime_decoder @ 0x1f574

void mtime_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_28;
  undefined8 local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011ee4e(&local_28,param_3,param_2);
  if (cVar1 != '\0') {
    *(undefined8 *)(param_1 + 0xf8) = local_28;
    *(undefined8 *)(param_1 + 0x100) = local_20;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: path_coder @ 0x1f5eb

void path_coder(long param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011ecd7(*(undefined8 *)(param_1 + 8),param_2,param_3);
  return;
}




// Function: raw_path_decoder @ 0x1f625

void raw_path_decoder(long param_1,undefined8 param_2)

{
  undefined1 uVar1;
  
  FUN_0011ed64(param_1,param_2);
  FUN_0011ed64(param_1 + 8,param_2);
  uVar1 = FUN_0014e2e5(*(undefined8 *)(param_1 + 8));
  *(undefined1 *)(param_1 + 0x10) = uVar1;
  return;
}




// Function: path_decoder @ 0x1f67d

void path_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  if (*(char *)(param_1 + 0x151) != '\x01') {
    FUN_0011f625(param_1,param_3);
  }
  return;
}




// Function: sparse_path_decoder @ 0x1f6c1

void sparse_path_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  *(undefined1 *)(param_1 + 0x151) = 1;
  FUN_0011f625(param_1,param_3);
  return;
}




// Function: size_coder @ 0x1f6fe

void size_coder(long param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011efed(*(undefined8 *)(param_1 + 0x88),param_2,param_3);
  return;
}




// Function: size_decoder @ 0x1f73e

void size_decoder(long param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011f138(&local_18,param_3,0x7fffffffffffffff,param_2,param_5,param_6,param_4);
  if (cVar1 != '\0') {
    *(undefined8 *)(param_1 + 0x88) = local_18;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: uid_coder @ 0x1f7b7

void uid_coder(long param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6)

{
  FUN_0011ef79(*(undefined4 *)(param_1 + 0x74),param_2,0,0xffffffff,param_3,param_6,param_4);
  return;
}




// Function: uid_decoder @ 0x1f7ff

void uid_decoder(long param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined4 local_18 [2];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011f02b(local_18,param_3,0,0xffffffff,param_2,param_6,param_4);
  if (cVar1 != '\0') {
    *(undefined4 *)(param_1 + 0x74) = local_18[0];
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: uname_coder @ 0x1f876

void uname_coder(long param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011ecd7(*(undefined8 *)(param_1 + 0x20),param_2,param_3);
  return;
}




// Function: uname_decoder @ 0x1f8b0

void uname_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011ed64(param_1 + 0x20,param_3);
  return;
}




// Function: sparse_size_coder @ 0x1f8e6

void sparse_size_coder(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  FUN_0011f6fe(param_1,param_2,param_3,param_4);
  return;
}




// Function: sparse_size_decoder @ 0x1f91d

void sparse_size_decoder(long param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011f138(&local_18,param_3,0x7fffffffffffffff,param_2,param_5,param_6,param_4);
  if (cVar1 != '\0') {
    *(undefined1 *)(param_1 + 0x150) = 1;
    *(undefined8 *)(param_1 + 0x148) = local_18;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: sparse_numblocks_coder @ 0x1f9a1

void sparse_numblocks_coder(long param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011efed(*(undefined8 *)(param_1 + 0x130),param_2,param_3);
  return;
}




// Function: sparse_numblocks_decoder @ 0x1f9de

void sparse_numblocks_decoder(long param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011f138(&local_18,param_3,0xffffffffffffffff,param_2,param_5,param_6,param_4);
  if (cVar1 != '\0') {
    *(undefined8 *)(param_1 + 0x138) = local_18;
    uVar2 = FUN_00160c29(local_18,0x10);
    *(undefined8 *)(param_1 + 0x140) = uVar2;
    *(undefined8 *)(param_1 + 0x130) = 0;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: sparse_offset_coder @ 0x1fa7f

void sparse_offset_coder(long param_1,undefined8 param_2,undefined8 param_3,long *param_4)

{
  FUN_0011efed(*(undefined8 *)(*param_4 * 0x10 + *(long *)(param_1 + 0x140)),param_2,param_3);
  return;
}




// Function: sparse_offset_decoder @ 0x1fad8

void sparse_offset_decoder(long param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011f138(&local_18,param_3,0x7fffffffffffffff,param_2,param_5,param_6,param_4);
  if (cVar1 != '\0') {
    if (*(ulong *)(param_1 + 0x130) < *(ulong *)(param_1 + 0x138)) {
      *(undefined8 *)(*(long *)(param_1 + 0x130) * 0x10 + *(long *)(param_1 + 0x140)) = local_18;
    }
    else {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar2 = gettext("Malformed extended header: excess %s=%s");
      error(0,0,uVar2,"GNU.sparse.offset",param_3);
      DAT_0019fdd0 = 2;
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: sparse_numbytes_coder @ 0x1fbda

void sparse_numbytes_coder(long param_1,undefined8 param_2,undefined8 param_3,long *param_4)

{
  FUN_0011efed(*(undefined8 *)(*param_4 * 0x10 + *(long *)(param_1 + 0x140) + 8),param_2,param_3);
  return;
}




// Function: sparse_numbytes_decoder @ 0x1fc34

void sparse_numbytes_decoder(long param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6)

{
  long lVar1;
  char cVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar2 = FUN_0011f138(&local_18,param_3,0x7fffffffffffffff,param_2,param_5,param_6,param_4);
  if (cVar2 != '\0') {
    if (*(ulong *)(param_1 + 0x130) < *(ulong *)(param_1 + 0x138)) {
      lVar1 = *(long *)(param_1 + 0x130);
      *(long *)(param_1 + 0x130) = lVar1 + 1;
      *(undefined8 *)(lVar1 * 0x10 + *(long *)(param_1 + 0x140) + 8) = local_18;
    }
    else {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar3 = gettext("Malformed extended header: excess %s=%s");
      error(0,0,uVar3,param_2,param_3);
      DAT_0019fdd0 = 2;
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: sparse_map_decoder @ 0x1fd46

void sparse_map_decoder(long param_1,undefined8 param_2,char *param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6)

{
  intmax_t *piVar1;
  char cVar2;
  long lVar3;
  bool bVar4;
  undefined8 uVar5;
  int *piVar6;
  long in_FS_OFFSET;
  char *local_70;
  char *local_48;
  intmax_t local_40;
  intmax_t local_38;
  intmax_t local_30;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  bVar4 = true;
  *(undefined8 *)(param_1 + 0x130) = 0;
  local_70 = param_3;
  do {
    if (9 < (int)*local_70 - 0x30U) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar5 = gettext("Malformed extended header: invalid %s=%s");
      error(0,0,uVar5,param_2,local_70,param_6,param_4);
      DAT_0019fdd0 = 2;
LAB_00120030:
      if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    piVar6 = __errno_location();
    *piVar6 = 0;
    local_40 = strtoimax(local_70,&local_48,10);
    if (bVar4) {
      local_38 = local_40;
      piVar6 = __errno_location();
      if (*piVar6 == 0x22) {
        FUN_0011ec02(param_2,local_70,0,0x7fffffffffffffff);
        goto LAB_00120030;
      }
    }
    else {
      local_30 = local_40;
      piVar6 = __errno_location();
      if (*piVar6 == 0x22) {
        FUN_0011ec02(param_2,local_70,0,0x7fffffffffffffff);
        goto LAB_00120030;
      }
      if (*(ulong *)(param_1 + 0x138) <= *(ulong *)(param_1 + 0x130)) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar5 = gettext("Malformed extended header: excess %s=%s");
        error(0,0,uVar5,param_2,local_70);
        DAT_0019fdd0 = 2;
        goto LAB_00120030;
      }
      lVar3 = *(long *)(param_1 + 0x130);
      *(long *)(param_1 + 0x130) = lVar3 + 1;
      piVar1 = (intmax_t *)(*(long *)(param_1 + 0x140) + lVar3 * 0x10);
      *piVar1 = local_38;
      piVar1[1] = local_30;
    }
    bVar4 = !bVar4;
    if (*local_48 == '\0') {
      if (!bVar4) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar5 = gettext("Malformed extended header: invalid %s: odd number of values");
        error(0,0,uVar5,param_2);
        DAT_0019fdd0 = 2;
      }
      goto LAB_00120030;
    }
    if (*local_48 != ',') {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      cVar2 = *local_48;
      uVar5 = gettext("Malformed extended header: invalid %s: unexpected delimiter %c");
      error(0,0,uVar5,param_2,(int)cVar2);
      DAT_0019fdd0 = 2;
      goto LAB_00120030;
    }
    local_70 = local_48 + 1;
  } while( true );
}




// Function: dumpdir_coder @ 0x2004a

void dumpdir_coder(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  undefined8 uVar1;
  
  uVar1 = FUN_00120bfe(param_4);
  FUN_0011e4ff(param_3,param_2,param_4,uVar1);
  return;
}




// Function: dumpdir_decoder @ 0x2008c

void dumpdir_decoder(long param_1,undefined8 param_2,void *param_3,size_t param_4)

{
  undefined8 uVar1;
  
  uVar1 = FUN_00160b5b(param_4);
  *(undefined8 *)(param_1 + 400) = uVar1;
  memcpy(*(void **)(param_1 + 400),param_3,param_4);
  return;
}




// Function: volume_label_coder @ 0x200e3

void volume_label_coder(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  FUN_0011ecd7(param_4,param_2,param_3);
  return;
}




// Function: volume_label_decoder @ 0x20119

void volume_label_decoder(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011ed64(&DAT_0019efe0,param_3);
  return;
}




// Function: volume_size_coder @ 0x2014e

void volume_size_coder(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 *param_4)

{
  FUN_0011efed(*param_4,param_2,param_3);
  return;
}




// Function: volume_size_decoder @ 0x20192

void volume_size_decoder(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011f138(&local_18,param_3,0xffffffffffffffff,param_2,param_5,param_6,param_4,param_3,
                       param_2,param_1);
  if (cVar1 != '\0') {
    DAT_0019eff0 = local_18;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: volume_offset_coder @ 0x20201

void volume_offset_coder(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 *param_4)

{
  FUN_0011efed(*param_4,param_2,param_3);
  return;
}




// Function: volume_offset_decoder @ 0x20245

void volume_offset_decoder(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011f138(&local_18,param_3,0xffffffffffffffff,param_2,param_5,param_6,param_4,param_3,
                       param_2,param_1);
  if (cVar1 != '\0') {
    DAT_0019eff8 = local_18;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: volume_filename_decoder @ 0x202b4

void volume_filename_decoder(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011ed64(&DAT_0019efe8,param_3);
  return;
}




// Function: xattr_selinux_coder @ 0x202e9

void xattr_selinux_coder(long param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011ecd7(*(undefined8 *)(param_1 + 0x30),param_2,param_3);
  return;
}




// Function: xattr_selinux_decoder @ 0x20323

void xattr_selinux_decoder(long param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011ed64(param_1 + 0x30,param_3);
  return;
}




// Function: xattr_acls_a_coder @ 0x20359

void xattr_acls_a_coder(long param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6)

{
  FUN_0011e4ff(param_3,param_2,*(undefined8 *)(param_1 + 0x38),*(undefined8 *)(param_1 + 0x40),
               param_5,param_6,param_4);
  return;
}




// Function: xattr_acls_a_decoder @ 0x20398

void xattr_acls_a_decoder(long param_1,undefined8 param_2,undefined8 param_3,long param_4)

{
  undefined8 uVar1;
  
  uVar1 = FUN_00160c8e(param_3,param_4 + 1);
  *(undefined8 *)(param_1 + 0x38) = uVar1;
  *(long *)(param_1 + 0x40) = param_4;
  return;
}




// Function: xattr_acls_d_coder @ 0x203e2

void xattr_acls_d_coder(long param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6)

{
  FUN_0011e4ff(param_3,param_2,*(undefined8 *)(param_1 + 0x48),*(undefined8 *)(param_1 + 0x50),
               param_5,param_6,param_4);
  return;
}




// Function: xattr_acls_d_decoder @ 0x20421

void xattr_acls_d_decoder(long param_1,undefined8 param_2,undefined8 param_3,long param_4)

{
  undefined8 uVar1;
  
  uVar1 = FUN_00160c8e(param_3,param_4 + 1);
  *(undefined8 *)(param_1 + 0x48) = uVar1;
  *(long *)(param_1 + 0x50) = param_4;
  return;
}




// Function: xattr_coder @ 0x2046b

void xattr_coder(long param_1,undefined8 param_2,undefined8 param_3,long *param_4)

{
  FUN_0011e4ff(param_3,param_2,*(undefined8 *)(*(long *)(param_1 + 0x160) + *param_4 * 0x18 + 8),
               *(undefined8 *)(*(long *)(param_1 + 0x160) + *param_4 * 0x18 + 0x10));
  return;
}




// Function: xattr_decoder @ 0x204f5

void xattr_decoder(undefined8 param_1,char *param_2,void *param_3,long param_4)

{
  long lVar1;
  long lVar2;
  long lVar3;
  void *pvVar4;
  char *__src;
  undefined8 uVar5;
  ulong uVar6;
  size_t sVar7;
  long *plVar8;
  undefined1 *puVar9;
  long in_FS_OFFSET;
  long local_48;
  void *local_40;
  char *local_38;
  undefined8 local_30;
  size_t local_28;
  void *local_20;
  void *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_48 = param_4;
  local_40 = param_3;
  local_38 = param_2;
  local_30 = param_1;
  local_28 = strlen(param_2);
  uVar6 = ((local_28 + 0x18) / 0x10) * 0x10;
  for (plVar8 = &local_48; plVar8 != (long *)((long)&local_48 - (uVar6 & 0xfffffffffffff000));
      plVar8 = (long *)((long)plVar8 + -0x1000)) {
    *(undefined8 *)((long)plVar8 + -8) = *(undefined8 *)((long)plVar8 + -8);
  }
  lVar2 = -(ulong)((uint)uVar6 & 0xfff);
  puVar9 = (undefined1 *)((long)plVar8 + lVar2);
  if ((uVar6 & 0xfff) != 0) {
    *(undefined8 *)((long)plVar8 + ((ulong)((uint)uVar6 & 0xfff) - 8) + lVar2) =
         *(undefined8 *)((long)plVar8 + ((ulong)((uint)uVar6 & 0xfff) - 8) + lVar2);
  }
  __src = local_38;
  sVar7 = local_28 + 1;
  local_20 = (void *)((ulong)((long)plVar8 + lVar2 + 0xf) & 0xfffffffffffffff0);
  *(undefined8 *)((long)plVar8 + lVar2 + -8) = 0x1205d8;
  memcpy((void *)((ulong)((long)plVar8 + lVar2 + 0xf) & 0xfffffffffffffff0),__src,sVar7);
  uVar6 = ((local_48 + 0x18U) / 0x10) * 0x10;
  for (; puVar9 != (undefined1 *)((long)plVar8 + (lVar2 - (uVar6 & 0xfffffffffffff000)));
      puVar9 = puVar9 + -0x1000) {
    *(undefined8 *)(puVar9 + -8) = *(undefined8 *)(puVar9 + -8);
  }
  lVar2 = -(ulong)((uint)uVar6 & 0xfff);
  if ((uVar6 & 0xfff) != 0) {
    *(undefined8 *)(puVar9 + ((ulong)((uint)uVar6 & 0xfff) - 8) + lVar2) =
         *(undefined8 *)(puVar9 + ((ulong)((uint)uVar6 & 0xfff) - 8) + lVar2);
  }
  pvVar4 = local_40;
  sVar7 = local_48 + 1;
  local_18 = (void *)((ulong)(puVar9 + lVar2 + 0xf) & 0xfffffffffffffff0);
  *(undefined8 *)(puVar9 + lVar2 + -8) = 0x120680;
  memcpy((void *)((ulong)(puVar9 + lVar2 + 0xf) & 0xfffffffffffffff0),pvVar4,sVar7);
  pvVar4 = local_20;
  *(undefined8 *)(puVar9 + lVar2 + -8) = 0x12068c;
  FUN_0011d6da(pvVar4);
  *(undefined8 *)(puVar9 + lVar2 + -8) = 0x12069b;
  sVar7 = strlen("SCHILY.xattr.");
  pvVar4 = local_18;
  uVar5 = local_30;
  lVar3 = local_48;
  lVar1 = sVar7 + (long)local_20;
  *(undefined8 *)(puVar9 + lVar2 + -8) = 0x1206b7;
  FUN_0011d78e(uVar5,lVar1,pvVar4,lVar3);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    *(undefined8 *)(puVar9 + lVar2 + -8) = 0x1206cc;
    __stack_chk_fail();
  }
  return;
}




// Function: sparse_major_coder @ 0x206ce

void sparse_major_coder(long param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011efed(*(undefined4 *)(param_1 + 0x124),param_2,param_3);
  return;
}




// Function: sparse_major_decoder @ 0x2070c

void sparse_major_decoder(long param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined4 local_18 [2];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011f138(local_18,param_3,0xffffffff,param_2,param_5,param_6,param_4);
  if (cVar1 != '\0') {
    *(undefined4 *)(param_1 + 0x124) = local_18[0];
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: sparse_minor_coder @ 0x2077e

void sparse_minor_coder(long param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_0011efed(*(undefined4 *)(param_1 + 0x128),param_2,param_3);
  return;
}




// Function: sparse_minor_decoder @ 0x207bc

void sparse_minor_decoder(long param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,undefined8 param_6)

{
  char cVar1;
  long in_FS_OFFSET;
  undefined4 local_18 [2];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0011f138(local_18,param_3,0xffffffff,param_2,param_5,param_6,param_4);
  if (cVar1 != '\0') {
    *(undefined4 *)(param_1 + 0x128) = local_18[0];
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: dumpdir_create0 @ 0x2082e

long * dumpdir_create0(char *param_1,char *param_2)

{
  long *plVar1;
  long lVar2;
  char *pcVar3;
  size_t sVar4;
  long local_40;
  long local_38;
  size_t local_30;
  char *local_28;
  char *local_20;
  
  local_40 = 0;
  local_38 = 0;
  local_30 = 1;
  for (local_20 = param_1; *local_20 != '\0'; local_20 = local_20 + sVar4 + 1) {
    sVar4 = strlen(local_20);
    local_30 = local_30 + sVar4 + 1;
    if (param_2 == (char *)0x0) {
LAB_001208a4:
      local_40 = local_40 + 1;
    }
    else {
      pcVar3 = strchr(param_2,(int)*local_20);
      if (pcVar3 != (char *)0x0) goto LAB_001208a4;
    }
    local_38 = local_38 + 1;
  }
  plVar1 = (long *)FUN_00160b5b(local_30 + 0x20);
  *plVar1 = (long)(plVar1 + 4);
  memcpy((void *)*plVar1,param_1,local_30);
  plVar1[1] = local_38;
  plVar1[2] = local_40;
  lVar2 = FUN_00160c29(local_40 + 1,8);
  plVar1[3] = lVar2;
  local_40 = 0;
  local_28 = (char *)*plVar1;
  do {
    if (*local_28 == '\0') {
      *(undefined8 *)(local_40 * 8 + plVar1[3]) = 0;
      return plVar1;
    }
    if (param_2 == (char *)0x0) {
LAB_0012096f:
      *(char **)(local_40 * 8 + plVar1[3]) = local_28 + 1;
      local_40 = local_40 + 1;
    }
    else {
      pcVar3 = strchr(param_2,(int)*local_28);
      if (pcVar3 != (char *)0x0) goto LAB_0012096f;
    }
    sVar4 = strlen(local_28);
    local_28 = local_28 + sVar4 + 1;
  } while( true );
}




// Function: dumpdir_create @ 0x209d4

void dumpdir_create(undefined8 param_1)

{
  FUN_0012082e(param_1,&DAT_00181be0);
  return;
}




// Function: dumpdir_free @ 0x209fc

void dumpdir_free(void *param_1)

{
  free(*(void **)((long)param_1 + 0x18));
  free(param_1);
  return;
}




// Function: compare_dirnames @ 0x20a2b

void compare_dirnames(undefined8 *param_1,undefined8 *param_2)

{
  strcmp((char *)*param_1,(char *)*param_2);
  return;
}




// Function: dumpdir_locate @ 0x20a6a

long dumpdir_locate(long param_1,undefined8 param_2)

{
  long lVar1;
  long *plVar2;
  undefined8 local_28;
  long local_20;
  
  if (param_1 == 0) {
    lVar1 = 0;
  }
  else {
    local_28 = param_2;
    local_20 = param_1;
    plVar2 = bsearch(&local_28,*(void **)(param_1 + 0x18),*(size_t *)(param_1 + 0x10),8,FUN_00120a2b
                    );
    if (plVar2 == (long *)0x0) {
      lVar1 = 0;
    }
    else {
      lVar1 = *plVar2 + -1;
    }
  }
  return lVar1;
}




// Function: dumpdir_next @ 0x20ad3

char * dumpdir_next(long *param_1)

{
  ulong uVar1;
  long lVar2;
  size_t sVar3;
  char *local_28;
  
  uVar1 = param_1[2];
  local_28 = (char *)0x0;
  if ((int)param_1[1] == 0) {
    if (uVar1 < *(ulong *)(*param_1 + 0x10)) {
      local_28 = (char *)(*(long *)(uVar1 * 8 + *(long *)(*param_1 + 0x18)) + -1);
      param_1[2] = param_1[2] + 1;
    }
  }
  else {
    local_28 = (char *)(uVar1 + *(long *)*param_1);
    if (*local_28 == '\0') {
      local_28 = (char *)0x0;
    }
    else {
      lVar2 = param_1[2];
      sVar3 = strlen(local_28);
      param_1[2] = sVar3 + lVar2 + 1;
    }
  }
  return local_28;
}




// Function: dumpdir_first @ 0x20b9f

void dumpdir_first(undefined8 param_1,undefined4 param_2,long *param_3)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_00160b5b(0x18);
  *puVar1 = param_1;
  *(undefined4 *)(puVar1 + 1) = param_2;
  puVar1[2] = 0;
  *param_3 = (long)puVar1;
  FUN_00120ad3(puVar1);
  return;
}




// Function: dumpdir_size @ 0x20bfe

long dumpdir_size(char *param_1)

{
  size_t sVar1;
  char *local_20;
  long local_18;
  
  local_18 = 0;
  for (local_20 = param_1; *local_20 != '\0'; local_20 = local_20 + sVar1 + 1) {
    sVar1 = strlen(local_20);
    local_18 = local_18 + sVar1 + 1;
  }
  return local_18 + 1;
}




// Function: hash_directory_canonical_name @ 0x20c51

void hash_directory_canonical_name(long param_1,undefined8 param_2)

{
  FUN_00152834(*(undefined8 *)(param_1 + 0x50),param_2);
  return;
}




// Function: compare_directory_canonical_names @ 0x20c86

undefined8 compare_directory_canonical_names(long param_1,long param_2)

{
  int iVar1;
  undefined4 extraout_var;
  
  iVar1 = strcmp(*(char **)(param_1 + 0x50),*(char **)(param_2 + 0x50));
  return CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),iVar1 == 0);
}




// Function: hash_directory_meta @ 0x20ccc

ulong hash_directory_meta(long param_1,ulong param_2)

{
  return (ulong)(*(long *)(param_1 + 0x20) + *(long *)(param_1 + 0x18)) % param_2;
}




// Function: compare_directory_meta @ 0x20d05

undefined4 compare_directory_meta(long param_1,long param_2)

{
  undefined4 uVar1;
  
  if ((*(long *)(param_1 + 0x18) == *(long *)(param_2 + 0x18)) &&
     (*(long *)(param_1 + 0x20) == *(long *)(param_2 + 0x20))) {
    uVar1 = 1;
  }
  else {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: make_directory @ 0x20d60

undefined8 * make_directory(char *param_1,undefined8 param_2)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  size_t local_18;
  
  local_18 = strlen(param_1);
  puVar1 = (undefined8 *)FUN_00160b5b(0x60);
  *puVar1 = 0;
  puVar1[6] = 0;
  puVar1[5] = puVar1[6];
  puVar1[8] = 0;
  *(undefined4 *)((long)puVar1 + 0x3c) = 0;
  if ((1 < local_18) && (param_1[local_18 - 1] == '/')) {
    local_18 = local_18 - 1;
  }
  uVar2 = FUN_00160b5b(local_18 + 1);
  puVar1[0xb] = uVar2;
  memcpy((void *)puVar1[0xb],param_1,local_18);
  *(undefined1 *)(local_18 + puVar1[0xb]) = 0;
  puVar1[10] = param_2;
  puVar1[9] = 0;
  return puVar1;
}




// Function: free_directory @ 0x20e58

void free_directory(void *param_1)

{
  free(*(void **)((long)param_1 + 0x50));
  free(*(void **)((long)param_1 + 0x58));
  free(param_1);
  return;
}




// Function: attach_directory @ 0x20e97

undefined8 attach_directory(undefined8 param_1)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  uVar1 = FUN_00129aac(DAT_0019f600,param_1);
  uVar2 = FUN_00120d60(param_1,uVar1);
  uVar1 = uVar2;
  if (DAT_0019f440 != (undefined8 *)0x0) {
    *DAT_0019f440 = uVar2;
    uVar1 = DAT_0019f438;
  }
  DAT_0019f438 = uVar1;
  DAT_0019f440 = (undefined8 *)uVar2;
  return uVar2;
}




// Function: dirlist_replace_prefix @ 0x20f0e

void dirlist_replace_prefix(char *param_1,char *param_2)

{
  size_t sVar1;
  size_t sVar2;
  undefined8 *local_20;
  
  sVar1 = strlen(param_1);
  sVar2 = strlen(param_2);
  for (local_20 = DAT_0019f438; local_20 != (undefined8 *)0x0; local_20 = (undefined8 *)*local_20) {
    FUN_00129b93(local_20 + 0xb,param_1,sVar1,param_2,sVar2);
  }
  return;
}




// Function: clear_directory_table @ 0x20f88

void clear_directory_table(void)

{
  undefined8 *puVar1;
  undefined8 local_18;
  
  if (DAT_0019f448 != 0) {
    FUN_00152d91(DAT_0019f448);
  }
  if (DAT_0019f450 != 0) {
    FUN_00152d91(DAT_0019f450);
  }
  local_18 = DAT_0019f438;
  while (local_18 != (undefined8 *)0x0) {
    puVar1 = (undefined8 *)*local_18;
    FUN_00120e58(local_18);
    local_18 = puVar1;
  }
  DAT_0019f440 = 0;
  DAT_0019f438 = (undefined8 *)0x0;
  return;
}




// Function: note_directory @ 0x21019

long note_directory(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5,char param_6,char param_7,long param_8)

{
  long lVar1;
  undefined8 uVar2;
  long lVar3;
  
  lVar1 = FUN_00120e97(param_1);
  *(undefined8 *)(lVar1 + 8) = param_2;
  *(undefined8 *)(lVar1 + 0x10) = param_3;
  *(undefined8 *)(lVar1 + 0x18) = param_4;
  *(undefined8 *)(lVar1 + 0x20) = param_5;
  *(undefined4 *)(lVar1 + 0x38) = 1;
  if (param_6 != '\0') {
    *(uint *)(lVar1 + 0x3c) = *(uint *)(lVar1 + 0x3c) | 2;
  }
  if (param_7 != '\0') {
    *(uint *)(lVar1 + 0x3c) = *(uint *)(lVar1 + 0x3c) | 4;
  }
  if (param_8 == 0) {
    *(undefined8 *)(lVar1 + 0x28) = 0;
  }
  else {
    uVar2 = FUN_001209d4(param_8);
    *(undefined8 *)(lVar1 + 0x28) = uVar2;
  }
  if (((DAT_0019f448 == 0) &&
      (DAT_0019f448 = FUN_00152c1c(0,0,FUN_00120c51,FUN_00120c86,0), DAT_0019f448 == 0)) ||
     (lVar3 = FUN_0015395d(DAT_0019f448,lVar1), lVar3 == 0)) {
    FUN_0011783c();
  }
  if (((DAT_0019f450 != 0) ||
      (DAT_0019f450 = FUN_00152c1c(0,0,FUN_00120ccc,FUN_00120d05,0), DAT_0019f450 != 0)) &&
     (lVar3 = FUN_0015395d(DAT_0019f450,lVar1), lVar3 != 0)) {
    return lVar1;
  }
  FUN_0011783c();
  return lVar1;
}




// Function: find_directory @ 0x211c6

undefined8 find_directory(undefined8 param_1)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  if (DAT_0019f448 == 0) {
    uVar1 = 0;
  }
  else {
    uVar2 = FUN_00129aac(DAT_0019f600,param_1);
    uVar2 = FUN_00120d60(param_1,uVar2);
    uVar1 = FUN_00152567(DAT_0019f448,uVar2);
    FUN_00120e58(uVar2);
  }
  return uVar1;
}




// Function: rebase_directory @ 0x21244

void rebase_directory(long param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5)

{
  FUN_00129b93(param_1 + 0x58,param_2,param_3,param_4,param_5);
  return;
}




// Function: find_directory_meta @ 0x2128a

undefined8 find_directory_meta(undefined8 param_1,undefined8 param_2)

{
  undefined8 uVar1;
  long lVar2;
  
  if (DAT_0019f450 == 0) {
    uVar1 = 0;
  }
  else {
    lVar2 = FUN_00120d60(&DAT_00181be4,0);
    *(undefined8 *)(lVar2 + 0x18) = param_1;
    *(undefined8 *)(lVar2 + 0x20) = param_2;
    uVar1 = FUN_00152567(DAT_0019f450,lVar2);
    FUN_00120e58(lVar2);
  }
  return uVar1;
}




// Function: update_parent_directory @ 0x2130d

void update_parent_directory(undefined8 *param_1)

{
  int iVar1;
  long lVar2;
  long in_FS_OFFSET;
  undefined1 auVar3 [16];
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  lVar2 = FUN_001211c6(*param_1);
  if (lVar2 != 0) {
    iVar1 = fstat(*(int *)(param_1 + 0x35),&local_b8);
    if (iVar1 == 0) {
      auVar3 = FUN_0015fd9e(&local_b8);
      *(undefined1 (*) [16])(lVar2 + 8) = auVar3;
    }
    else {
      FUN_0012b444(*(undefined8 *)(lVar2 + 0x58));
    }
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: procdir @ 0x213c6

long procdir(char *param_1,long param_2,uint param_3,undefined1 *param_4)

{
  long lVar1;
  long lVar2;
  bool bVar3;
  char cVar4;
  int iVar5;
  uint uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined4 uVar10;
  long in_FS_OFFSET;
  undefined1 auVar11 [16];
  undefined8 local_58;
  long local_50;
  long *local_48;
  long local_40;
  long local_38;
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_48 = (long *)(param_2 + 0x58);
  cVar4 = -(char)(*local_48 >> 0x3f);
  bVar3 = false;
  local_50 = FUN_001211c6(param_1);
  if (local_50 == 0) {
    local_40 = FUN_0012128a(*local_48,local_48[1]);
    lVar1 = local_48[1];
    lVar2 = *local_48;
    auVar11 = FUN_0015fd9e(local_48);
    local_50 = FUN_00121019(param_1,auVar11._0_8_,auVar11._8_8_,lVar2,lVar1,cVar4,1,0);
    if (local_40 == 0) {
      *(uint *)(local_50 + 0x3c) = *(uint *)(local_50 + 0x3c) | 8;
      if ((DAT_0019ec84 & 0x1000) != 0) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar7 = FUN_0015e548(param_1);
        uVar8 = gettext("%s: Directory is new");
        error(0,0,uVar8,uVar7);
      }
      if (DAT_0019f8e0 == 0) {
        auVar11 = FUN_0015fd9e(local_48);
        iVar5 = FUN_0015ff90(auVar11._0_8_,auVar11._8_8_,DAT_0019f900,DAT_0019f908);
        if (iVar5 < 0) goto LAB_001218c3;
        if (DAT_0019f87c != 0) {
          auVar11 = FUN_0015fd84(local_48);
          iVar5 = FUN_0015ff90(auVar11._0_8_,auVar11._8_8_,DAT_0019f900,DAT_0019f908);
          if (iVar5 < 0) goto LAB_001218c3;
        }
        uVar10 = 1;
      }
      else {
LAB_001218c3:
        uVar10 = 2;
      }
      *(undefined4 *)(local_50 + 0x38) = uVar10;
    }
    else {
      iVar5 = strcmp(*(char **)(local_40 + 0x58),param_1);
      if (iVar5 != 0) {
        if ((DAT_0019ec84 & 0x2000) != 0) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar7 = FUN_0015e821(1,*(undefined8 *)(local_40 + 0x58));
          uVar8 = FUN_0015e548(param_1);
          uVar9 = gettext("%s: Directory has been renamed from %s");
          error(0,0,uVar9,uVar8,uVar7);
        }
        *(long *)(local_50 + 0x40) = local_40;
        *(uint *)(local_50 + 0x3c) = *(uint *)(local_50 + 0x3c) | 0x10;
        *(uint *)(local_40 + 0x3c) = *(uint *)(local_40 + 0x3c) & 0xffffffef;
        FUN_00120f0e(*(undefined8 *)(local_40 + 0x58),param_1);
      }
      *(undefined4 *)(local_50 + 0x38) = 1;
    }
  }
  else {
    if ((*(uint *)(local_50 + 0x3c) & 1) != 0) {
      if ((param_3 & 0x20) == 0) {
        *param_4 = 0x4e;
        goto LAB_00121b55;
      }
      FUN_00129547(local_50 + 0x58,param_1);
    }
    iVar5 = strcmp(*(char **)(local_50 + 0x58),param_1);
    if (iVar5 != 0) {
      *param_4 = 0x4e;
      goto LAB_00121b55;
    }
    if (((DAT_0019f8ec == '\0') ||
        ((((*(uint *)(local_50 + 0x3c) & 2) != 0 && (cVar4 == '\x01')) ||
         (*(long *)(local_50 + 0x18) == *local_48)))) && (*(long *)(local_50 + 0x20) == local_48[1])
       ) {
      *(undefined4 *)(local_50 + 0x38) = 1;
    }
    else {
      local_38 = FUN_0012128a(*local_48,local_48[1]);
      if (local_38 == 0) {
        bVar3 = true;
        *(undefined4 *)(local_50 + 0x38) = 2;
        *(long *)(local_50 + 0x18) = *local_48;
        *(long *)(local_50 + 0x20) = local_48[1];
      }
      else {
        iVar5 = strcmp(*(char **)(local_38 + 0x58),param_1);
        if (iVar5 != 0) {
          if ((DAT_0019ec84 & 0x2000) != 0) {
            if (DAT_0019fdc8 != (code *)0x0) {
              (*DAT_0019fdc8)();
            }
            uVar7 = FUN_0015e821(1,*(undefined8 *)(local_38 + 0x58));
            uVar8 = FUN_0015e548(param_1);
            uVar9 = gettext("%s: Directory has been renamed from %s");
            error(0,0,uVar9,uVar8,uVar7);
          }
          *(long *)(local_50 + 0x40) = local_38;
          *(uint *)(local_50 + 0x3c) = *(uint *)(local_50 + 0x3c) | 0x10;
          *(uint *)(local_38 + 0x3c) = *(uint *)(local_38 + 0x3c) & 0xffffffef;
          FUN_00120f0e(*(undefined8 *)(local_38 + 0x58),param_1);
        }
        *(undefined4 *)(local_50 + 0x38) = 1;
      }
      if (cVar4 != '\0') {
        *(uint *)(local_50 + 0x3c) = *(uint *)(local_50 + 0x3c) | 2;
      }
    }
    *(uint *)(local_50 + 0x3c) = *(uint *)(local_50 + 0x3c) | 4;
  }
  if (((DAT_0019f935 == '\0') || (*(long *)(param_2 + 0x198) == 0)) ||
     (*local_48 == *(long *)(*(long *)(param_2 + 0x198) + 0x58))) {
    if (((param_3 & 0x10) != 0) &&
       (*(uint *)(local_50 + 0x38) = param_3 & 3, *(int *)(local_50 + 0x38) == 0)) {
      *param_4 = 0x4e;
    }
  }
  else {
    if ((DAT_0019ec84 & 0x40000) != 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar7 = FUN_0015e548(*(undefined8 *)(local_50 + 0x58));
      uVar8 = gettext("%s: directory is on a different filesystem; not dumped");
      error(0,0,uVar8,uVar7);
    }
    *(undefined4 *)(local_50 + 0x38) = 0;
    if (*(long *)(local_50 + 0x28) != 0) {
      FUN_001209fc(*(undefined8 *)(local_50 + 0x28));
      *(undefined8 *)(local_50 + 0x28) = 0;
    }
    bVar3 = false;
  }
  if ((bVar3) && ((DAT_0019ec84 & 0x2000) != 0)) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar7 = FUN_0015e548(param_1);
    uVar8 = gettext("%s: Directory has been renamed");
    error(0,0,uVar8,uVar7);
  }
  *(uint *)(local_50 + 0x3c) = *(uint *)(local_50 + 0x3c) | 1;
  if (*(int *)(local_50 + 0x38) != 0) {
    uVar6 = FUN_00111fbf(param_2,&local_58);
    if (uVar6 == 3) {
      uVar7 = gettext("directory not dumped");
      FUN_00111f18(param_1,local_58,uVar7);
      *param_4 = 0x4e;
      *(undefined4 *)(local_50 + 0x38) = 0;
    }
    else if (uVar6 < 4) {
      if (uVar6 == 2) {
        uVar7 = gettext("contents not dumped");
        FUN_00111f18(param_1,local_58,uVar7);
        *(undefined8 *)(local_50 + 0x48) = local_58;
      }
      else if (((uVar6 < 3) && (uVar6 != 0)) && (uVar6 == 1)) {
        uVar7 = gettext("contents not dumped");
        FUN_00111f18(param_1,local_58,uVar7);
        *(undefined4 *)(local_50 + 0x38) = 0;
        *(undefined8 *)(local_50 + 0x48) = local_58;
      }
    }
  }
LAB_00121b55:
  if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_50;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: makedumpdir @ 0x21b74

void makedumpdir(long param_1,char *param_2)

{
  char *pcVar1;
  char cVar2;
  void *__base;
  size_t sVar3;
  char *__ptr;
  long lVar4;
  undefined8 uVar5;
  ulong local_50;
  size_t local_48;
  long local_40;
  char *local_38;
  char *local_30;
  undefined8 local_28;
  
  if (*(int *)(param_1 + 0x38) == 2) {
    local_28 = 0;
  }
  else if (*(long *)(param_1 + 0x40) == 0) {
    local_28 = *(undefined8 *)(param_1 + 0x28);
  }
  else if (*(long *)(*(long *)(param_1 + 0x40) + 0x30) == 0) {
    local_28 = *(undefined8 *)(*(long *)(param_1 + 0x40) + 0x28);
  }
  else {
    local_28 = *(undefined8 *)(*(long *)(param_1 + 0x40) + 0x30);
  }
  local_48 = 0;
  local_40 = 0;
  for (local_38 = param_2; *local_38 != '\0'; local_38 = local_38 + sVar3 + 1) {
    sVar3 = strlen(local_38);
    local_40 = sVar3 + local_40 + 2;
    sVar3 = strlen(local_38);
    local_48 = local_48 + 1;
  }
  __base = (void *)FUN_00160c29(local_48,8);
  local_50 = 0;
  for (local_38 = param_2; *local_38 != '\0'; local_38 = local_38 + sVar3 + 1) {
    *(char **)(local_50 * 8 + (long)__base) = local_38;
    sVar3 = strlen(local_38);
    local_50 = local_50 + 1;
  }
  qsort(__base,local_48,8,FUN_00120a2b);
  __ptr = (char *)FUN_00160b5b(local_40 + 1);
  local_30 = __ptr;
  for (local_50 = 0; local_50 < local_48; local_50 = local_50 + 1) {
    lVar4 = FUN_00120a6a(local_28,*(undefined8 *)((long)__base + local_50 * 8));
    if (lVar4 == 0) {
      if (*(long *)(param_1 + 0x48) == 0) {
        *local_30 = 'Y';
      }
      else {
        *local_30 = 'I';
      }
    }
    else if (*(long *)(param_1 + 0x48) == 0) {
      *local_30 = ' ';
    }
    else {
      *local_30 = 'I';
    }
    local_30 = local_30 + 1;
    local_38 = *(char **)((long)__base + local_50 * 8);
    do {
      pcVar1 = local_30 + 1;
      *local_30 = *local_38;
      cVar2 = *local_30;
      local_38 = local_38 + 1;
      local_30 = pcVar1;
    } while (cVar2 != '\0');
  }
  *local_30 = '\0';
  *(undefined8 *)(param_1 + 0x30) = *(undefined8 *)(param_1 + 0x28);
  uVar5 = FUN_0012082e(__ptr,0);
  *(undefined8 *)(param_1 + 0x28) = uVar5;
  free(__ptr);
  free(__base);
  return;
}




// Function: maketagdumpdir @ 0x21e0d

void maketagdumpdir(long param_1)

{
  size_t sVar1;
  undefined1 *__ptr;
  undefined8 uVar2;
  
  sVar1 = strlen(*(char **)(param_1 + 0x48));
  __ptr = (undefined1 *)FUN_00160b5b(sVar1 + 3);
  *__ptr = 0x59;
  memcpy(__ptr + 1,*(void **)(param_1 + 0x48),sVar1 + 1);
  __ptr[sVar1 + 2] = 0;
  *(undefined8 *)(param_1 + 0x30) = *(undefined8 *)(param_1 + 0x28);
  uVar2 = FUN_0012082e(__ptr,0);
  *(undefined8 *)(param_1 + 0x28) = uVar2;
  free(__ptr);
  return;
}




// Function: scan_directory @ 0x21eb9

long scan_directory(undefined8 *param_1)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  undefined1 auVar5 [16];
  undefined1 local_236;
  char local_235;
  undefined4 local_234;
  int local_230;
  int local_22c;
  void *local_228;
  char *local_220;
  code *local_218;
  undefined8 local_210;
  void *local_208;
  __dev_t local_200;
  void *local_1f8;
  long local_1f0;
  undefined8 local_1e8;
  undefined8 local_1e0;
  undefined1 local_1d8 [88];
  stat local_180 [2];
  undefined8 *local_40;
  int local_30;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_210 = *param_1;
  local_208 = (void *)FUN_00114d3d(param_1);
  local_200 = param_1[0xb];
  local_235 = param_1[0x33] == 0;
  if (local_208 == (void *)0x0) {
    FUN_0013e72e(local_210);
  }
  FUN_00117909(param_1);
  local_1f8 = (void *)FUN_00160cc6(local_210);
  FUN_001298d7(local_1f8);
  if (local_235 == '\0') {
    uVar4 = 0;
  }
  else {
    uVar4 = 0x20;
  }
  local_1f0 = FUN_001213c6(local_1f8,param_1,uVar4,&local_236);
  free(local_1f8);
  local_1e8 = FUN_0012b66c(local_210);
  if (local_208 != (void *)0x0) {
    if (*(int *)(local_1f0 + 0x38) == 0) {
      if (*(long *)(local_1f0 + 0x48) != 0) {
        FUN_00121e0d(local_1f0);
      }
    }
    else {
      FUN_00121b74(local_1f0,local_208);
      local_220 = (char *)FUN_00120b9f(*(undefined8 *)(local_1f0 + 0x28),1,&local_228);
      while (local_220 != (char *)0x0) {
        local_1e0 = FUN_0012b75b(local_1e8,local_220 + 1);
        if (*local_220 == 'I') {
          *local_220 = 'N';
        }
        else {
          cVar1 = FUN_00117c43(local_1e0,param_1);
          if (cVar1 == '\0') {
            local_230 = *(int *)(param_1 + 0x35);
            local_218 = (code *)0x0;
            FUN_00139c55(local_1d8);
            if (local_230 < 0) {
              piVar3 = __errno_location();
              *piVar3 = -local_230;
              local_218 = FUN_0012b2b1;
            }
            else {
              iVar2 = fstatat(local_230,local_220 + 1,local_180,DAT_0019fc10);
              if (iVar2 == 0) {
                if ((local_180[0].st_mode & 0xf000) == 0x4000) {
                  local_22c = FUN_001157f9(param_1,local_220 + 1,DAT_0019fc08);
                  if (local_22c < 0) {
                    local_218 = FUN_0012b2b1;
                  }
                  else {
                    local_30 = local_22c;
                    iVar2 = fstat(local_22c,local_180);
                    if (iVar2 != 0) {
                      local_218 = FUN_0012b444;
                    }
                  }
                }
              }
              else {
                local_218 = FUN_0012b444;
              }
            }
            if (local_218 == (code *)0x0) {
              if ((local_180[0].st_mode & 0xf000) == 0x4000) {
                local_234 = 0;
                if (DAT_0019f930 == 0) {
                  local_234 = 0x10;
                }
                else if (*(int *)(local_1f0 + 0x38) == 2) {
                  local_234 = 0x12;
                }
                *local_220 = 'D';
                local_40 = param_1;
                FUN_001213c6(local_1e0,local_1d8,local_234,local_220);
                FUN_0011588e(local_1d8);
              }
              else if ((DAT_0019f935 == '\0') || (local_200 == local_180[0].st_dev)) {
                if (*local_220 != 'Y') {
                  auVar5 = FUN_0015fd9e(local_180);
                  iVar2 = FUN_0015ff90(auVar5._0_8_,auVar5._8_8_,DAT_0019f900,DAT_0019f908);
                  if (iVar2 < 0) {
                    if (DAT_0019f87c != 0) {
                      auVar5 = FUN_0015fd84(local_180);
                      iVar2 = FUN_0015ff90(auVar5._0_8_,auVar5._8_8_,DAT_0019f900,DAT_0019f908);
                      if (-1 < iVar2) goto LAB_0012236f;
                    }
                    *local_220 = 'N';
                  }
                  else {
LAB_0012236f:
                    *local_220 = 'Y';
                  }
                }
              }
              else {
                *local_220 = 'N';
              }
            }
            else {
              FUN_0012b48b(local_1e0,0,local_218);
              *local_220 = 'N';
            }
            FUN_00139d1c(local_1d8);
          }
          else {
            *local_220 = 'N';
          }
        }
        local_220 = (char *)FUN_00120ad3(local_228);
      }
      free(local_228);
    }
  }
  FUN_0012b72d(local_1e8);
  free(local_208);
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_1f0;
}




// Function: directory_contents @ 0x2241b

undefined8 directory_contents(long param_1)

{
  undefined8 uVar1;
  
  if (param_1 == 0) {
    uVar1 = 0;
  }
  else if (*(long *)(param_1 + 0x28) == 0) {
    uVar1 = 0;
  }
  else {
    uVar1 = **(undefined8 **)(param_1 + 0x28);
  }
  return uVar1;
}




// Function: safe_directory_contents @ 0x22456

undefined * safe_directory_contents(undefined8 param_1)

{
  undefined *puVar1;
  
  puVar1 = (undefined *)FUN_0012241b(param_1);
  if (puVar1 == (undefined *)0x0) {
    puVar1 = &DAT_00181ca8;
  }
  return puVar1;
}




// Function: obstack_code_rename @ 0x2248c

void obstack_code_rename(long param_1,char *param_2,char *param_3)

{
  undefined1 *puVar1;
  size_t sVar2;
  ulong uVar3;
  
  if (*param_2 != '\0') {
    param_2 = (char *)FUN_0013edd9(param_2,0,DAT_0019f878);
  }
  if (*(long *)(param_1 + 0x20) == *(long *)(param_1 + 0x18)) {
    _obstack_newchunk(param_1,1);
  }
  puVar1 = *(undefined1 **)(param_1 + 0x18);
  *(undefined1 **)(param_1 + 0x18) = puVar1 + 1;
  *puVar1 = 0x52;
  sVar2 = strlen(param_2);
  uVar3 = sVar2 + 1;
  if ((ulong)(*(long *)(param_1 + 0x20) - *(long *)(param_1 + 0x18)) < uVar3) {
    _obstack_newchunk(param_1,uVar3);
  }
  memcpy(*(void **)(param_1 + 0x18),param_2,uVar3);
  *(ulong *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + uVar3;
  if (*param_3 != '\0') {
    param_3 = (char *)FUN_0013edd9(param_3,0,DAT_0019f878);
  }
  if (*(long *)(param_1 + 0x20) == *(long *)(param_1 + 0x18)) {
    _obstack_newchunk(param_1,1);
  }
  puVar1 = *(undefined1 **)(param_1 + 0x18);
  *(undefined1 **)(param_1 + 0x18) = puVar1 + 1;
  *puVar1 = 0x54;
  sVar2 = strlen(param_3);
  uVar3 = sVar2 + 1;
  if ((ulong)(*(long *)(param_1 + 0x20) - *(long *)(param_1 + 0x18)) < uVar3) {
    _obstack_newchunk(param_1,uVar3);
  }
  memcpy(*(void **)(param_1 + 0x18),param_3,uVar3);
  *(ulong *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + uVar3;
  return;
}




// Function: store_rename @ 0x226b7

void store_rename(long param_1,long param_2)

{
  undefined1 *puVar1;
  char *__s;
  size_t sVar2;
  ulong __n;
  long local_48;
  long local_40;
  
  for (local_48 = param_1; (local_48 != 0 && (param_1 != *(long *)(local_48 + 0x40)));
      local_48 = *(long *)(local_48 + 0x40)) {
  }
  local_40 = param_1;
  if (local_48 == 0) {
    for (; (local_40 != 0 && (*(long *)(local_40 + 0x40) != 0));
        local_40 = *(long *)(local_40 + 0x40)) {
      FUN_0012248c(param_2,*(undefined8 *)(*(long *)(local_40 + 0x40) + 0x58),
                   *(undefined8 *)(local_40 + 0x58));
    }
  }
  else {
    __s = (char *)FUN_0014e10c(*(undefined8 *)(param_1 + 0x58));
    if (*(long *)(param_2 + 0x20) == *(long *)(param_2 + 0x18)) {
      _obstack_newchunk(param_2,1);
    }
    puVar1 = *(undefined1 **)(param_2 + 0x18);
    *(undefined1 **)(param_2 + 0x18) = puVar1 + 1;
    *puVar1 = 0x58;
    sVar2 = strlen(__s);
    __n = sVar2 + 1;
    if ((ulong)(*(long *)(param_2 + 0x20) - *(long *)(param_2 + 0x18)) < __n) {
      _obstack_newchunk(param_2,__n);
    }
    memcpy(*(void **)(param_2 + 0x18),__s,__n);
    *(ulong *)(param_2 + 0x18) = *(long *)(param_2 + 0x18) + __n;
    FUN_0012248c(param_2,*(undefined8 *)(param_1 + 0x58),&DAT_00181be4);
    for (; local_40 != local_48; local_40 = *(long *)(local_40 + 0x40)) {
      FUN_0012248c(param_2,*(undefined8 *)(*(long *)(local_40 + 0x40) + 0x58),
                   *(undefined8 *)(local_40 + 0x58));
    }
    FUN_0012248c(param_2,&DAT_00181be4,*(undefined8 *)(local_48 + 0x58));
    free(__s);
  }
  return;
}




// Function: append_incremental_renames @ 0x228cd

/* WARNING: Removing unreachable block (ram,0x00122c1a) */
/* WARNING: Removing unreachable block (ram,0x00122c28) */

void append_incremental_renames(long param_1)

{
  undefined1 *puVar1;
  void *__src;
  long lVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  ulong local_d0;
  undefined8 *local_c8;
  undefined1 local_68 [8];
  long local_60;
  undefined1 *local_58;
  undefined1 *local_50;
  undefined1 *local_48;
  ulong local_38;
  byte local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0019f438 != (undefined8 *)0x0) {
    _obstack_begin(local_68,0,0,FUN_00160b5b,PTR_free_0019bfc0);
    __src = (void *)FUN_0012241b(param_1);
    if (__src == (void *)0x0) {
      local_d0 = 0;
    }
    else {
      lVar2 = FUN_00120bfe(__src);
      local_d0 = lVar2 - 1;
      if ((ulong)((long)local_48 - (long)local_50) < local_d0) {
        _obstack_newchunk(local_68,local_d0);
      }
      memcpy(local_50,__src,local_d0);
      local_50 = local_50 + local_d0;
    }
    for (local_c8 = DAT_0019f438; local_c8 != (undefined8 *)0x0; local_c8 = (undefined8 *)*local_c8)
    {
      if ((*(uint *)((long)local_c8 + 0x3c) & 0x10) != 0) {
        FUN_001226b7(local_c8,local_68);
      }
    }
    if ((param_1 != 0) && ((long)local_50 - (long)local_58 != local_d0)) {
      if (local_48 == local_50) {
        _obstack_newchunk(local_68,1);
      }
      *local_50 = 0;
      local_50 = local_50 + 1;
      FUN_001209fc(*(undefined8 *)(param_1 + 0x28));
      puVar1 = local_58;
      if (local_58 == local_50) {
        local_18 = local_18 | 2;
      }
      local_50 = (undefined1 *)(~local_38 & (ulong)(local_50 + local_38));
      if ((ulong)((long)local_48 - local_60) < (ulong)((long)local_50 - local_60)) {
        local_50 = local_48;
      }
      local_58 = local_50;
      uVar3 = FUN_001209d4(puVar1);
      *(undefined8 *)(param_1 + 0x28) = uVar3;
    }
    _obstack_free(local_68,0);
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: read_incr_db_01 @ 0x22c72

void read_incr_db_01(int param_1,char *param_2)

{
  char cVar1;
  char cVar2;
  int iVar3;
  __ssize_t _Var4;
  undefined8 uVar5;
  undefined8 uVar6;
  int *piVar7;
  long in_FS_OFFSET;
  undefined1 auVar8 [16];
  char *local_80;
  size_t local_78;
  char *local_70;
  long local_68;
  char *local_60;
  char *local_58;
  ulong local_50;
  undefined8 local_48;
  undefined8 local_40;
  undefined1 local_38 [16];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_80 = (char *)0x0;
  local_78 = 0;
  local_68 = 1;
  if (param_1 == 1) {
    _Var4 = getline(&local_80,&local_78,DAT_0019f458);
    if (_Var4 < 1) {
      FUN_0013e403(DAT_0019f8e0);
      free(local_80);
      goto LAB_001232c7;
    }
    local_68 = local_68 + 1;
  }
  else {
    local_80 = strdup(param_2);
    local_78 = strlen(local_80);
    local_78 = local_78 + 1;
  }
  auVar8 = FUN_00129ff7(local_80,&local_70,0);
  DAT_0019f908 = auVar8._8_8_;
  DAT_0019f900 = auVar8._0_8_;
  cVar1 = FUN_001294f0(DAT_0019f900,DAT_0019f908);
  if (cVar1 != '\x01') {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar5 = gettext("Invalid time stamp");
    uVar6 = FUN_0015e548(DAT_0019f8e0);
    piVar7 = __errno_location();
    error(0,*piVar7,"%s:%ld: %s",uVar6,local_68,uVar5);
                    /* WARNING: Subroutine does not return */
    FUN_001177f4();
  }
  if ((param_1 == 1) && (*local_70 != '\0')) {
    local_58 = local_70 + 1;
    piVar7 = __errno_location();
    *piVar7 = 0;
    local_50 = strtoumax(local_58,&local_70,10);
    piVar7 = __errno_location();
    if ((*piVar7 == 0) && (999999999 < local_50)) {
      piVar7 = __errno_location();
      *piVar7 = 0x22;
    }
    piVar7 = __errno_location();
    if ((*piVar7 == 0) && (local_58 != local_70)) {
      DAT_0019f908 = local_50;
    }
    else {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar5 = gettext("Invalid time stamp");
      uVar6 = FUN_0015e548(DAT_0019f8e0);
      piVar7 = __errno_location();
      error(0,*piVar7,"%s:%ld: %s",uVar6,local_68,uVar5);
      DAT_0019fdd0 = 2;
      DAT_0019f900 = 0x8000000000000000;
      DAT_0019f908 = 0xffffffffffffffff;
    }
  }
  while( true ) {
    _Var4 = getline(&local_80,&local_78,DAT_0019f458);
    iVar3 = (int)_Var4;
    if (iVar3 < 1) break;
    cVar1 = *local_80;
    local_60 = local_80 + (cVar1 == '+');
    local_68 = local_68 + 1;
    if (local_80[(long)iVar3 + -1] == '\n') {
      local_80[(long)iVar3 + -1] = '\0';
    }
    if (param_1 == 1) {
      local_38 = FUN_00129ff7(local_60,&local_70,0);
      local_60 = local_70;
      cVar2 = FUN_001294f0(local_38._0_8_,local_38._8_8_);
      if ((cVar2 != '\x01') || (*local_60 != ' ')) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar5 = gettext("Invalid modification time");
        uVar6 = FUN_0015e548(DAT_0019f8e0);
        piVar7 = __errno_location();
        error(0,*piVar7,"%s:%ld: %s",uVar6,local_68,uVar5);
                    /* WARNING: Subroutine does not return */
        FUN_001177f4();
      }
      piVar7 = __errno_location();
      *piVar7 = 0;
      local_50 = strtoumax(local_60,&local_70,10);
      piVar7 = __errno_location();
      if ((*piVar7 == 0) && (999999999 < local_50)) {
        piVar7 = __errno_location();
        *piVar7 = 0x22;
      }
      piVar7 = __errno_location();
      if (((*piVar7 != 0) || (local_60 == local_70)) || (*local_70 != ' ')) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar5 = gettext("Invalid modification time (nanoseconds)");
        uVar6 = FUN_0015e548(DAT_0019f8e0);
        piVar7 = __errno_location();
        error(0,*piVar7,"%s:%ld: %s",uVar6,local_68,uVar5);
                    /* WARNING: Subroutine does not return */
        FUN_001177f4();
      }
      local_38._8_8_ = local_50;
      local_60 = local_70;
    }
    else {
      local_38._8_8_ = 0;
      local_38._0_8_ = 0;
    }
    local_48 = FUN_00129cfc(local_60,&local_70,0,0xffffffffffffffff);
    local_60 = local_70;
    piVar7 = __errno_location();
    if ((*piVar7 != 0) || (*local_60 != ' ')) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar5 = gettext("Invalid device number");
      uVar6 = FUN_0015e548(DAT_0019f8e0);
      piVar7 = __errno_location();
      error(0,*piVar7,"%s:%ld: %s",uVar6,local_68,uVar5);
                    /* WARNING: Subroutine does not return */
      FUN_001177f4();
    }
    local_40 = FUN_00129cfc(local_60,&local_70,0,0xffffffffffffffff);
    local_60 = local_70;
    piVar7 = __errno_location();
    if ((*piVar7 != 0) || (*local_60 != ' ')) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar5 = gettext("Invalid inode number");
      uVar6 = FUN_0015e548(DAT_0019f8e0);
      piVar7 = __errno_location();
      error(0,*piVar7,"%s:%ld: %s",uVar6,local_68,uVar5);
                    /* WARNING: Subroutine does not return */
      FUN_001177f4();
    }
    local_60 = local_60 + 1;
    FUN_0012961d(local_60);
    FUN_00121019(local_60,local_38._0_8_,local_38._8_8_,local_48,local_40,cVar1 == '+',0,0);
  }
  free(local_80);
LAB_001232c7:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: read_obstack @ 0x232e4

int read_obstack(FILE *param_1,long param_2,long *param_3)

{
  undefined1 *puVar1;
  int local_34;
  long local_30;
  
  local_30 = 0;
  local_34 = getc_unlocked(param_1);
  while ((local_34 != -1 && (local_34 != 0))) {
    if (*(long *)(param_2 + 0x20) == *(long *)(param_2 + 0x18)) {
      _obstack_newchunk(param_2,1);
    }
    puVar1 = *(undefined1 **)(param_2 + 0x18);
    *(undefined1 **)(param_2 + 0x18) = puVar1 + 1;
    *puVar1 = (char)local_34;
    local_34 = getc_unlocked(param_1);
    local_30 = local_30 + 1;
  }
  if (*(long *)(param_2 + 0x20) == *(long *)(param_2 + 0x18)) {
    _obstack_newchunk(param_2,1);
  }
  puVar1 = *(undefined1 **)(param_2 + 0x18);
  *(undefined1 **)(param_2 + 0x18) = puVar1 + 1;
  *puVar1 = 0;
  *param_3 = local_30;
  return local_34;
}




// Function: read_num @ 0x233ed

undefined8
read_num(FILE *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
            undefined8 *param_5)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  __off_t _Var4;
  undefined8 uVar5;
  int *piVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  long in_FS_OFFSET;
  bool bVar9;
  int local_b8;
  int local_b4;
  undefined1 local_a8 [32];
  undefined1 local_88 [32];
  undefined1 local_68 [32];
  undefined1 local_48 [24];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  local_b4 = getc_unlocked(param_1);
  bVar9 = local_b4 == 0x2d;
  local_b8 = 0;
  while (((local_b8 == 0 && (bVar9)) || (local_b4 - 0x30U < 10))) {
    local_a8[local_b8] = (char)local_b4;
    if (local_b8 == 0x14) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      _Var4 = ftello(param_1);
      uVar2 = FUN_00154d1c(_Var4,local_88);
      uVar3 = FUN_0015e548(DAT_0019f8e0);
      uVar5 = gettext("%s: byte %s: %s %.*s... too long");
      error(0,0,uVar5,uVar3,uVar2,param_2,0x15,local_a8);
                    /* WARNING: Subroutine does not return */
      FUN_001177f4();
    }
    local_b4 = getc_unlocked(param_1);
    local_b8 = local_b8 + 1;
  }
  local_a8[local_b8] = 0;
  if (local_b4 < 0) {
    iVar1 = ferror_unlocked(param_1);
    if (iVar1 != 0) {
      FUN_0013e5d6(DAT_0019f8e0);
    }
    if (local_b8 != 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar2 = gettext("Unexpected EOF in snapshot file");
      uVar3 = FUN_0015e548(DAT_0019f8e0);
      error(0,0,"%s: %s",uVar3,uVar2);
                    /* WARNING: Subroutine does not return */
      FUN_001177f4();
    }
    uVar2 = 0;
  }
  else {
    if (local_b4 != 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      _Var4 = ftello(param_1);
      uVar2 = FUN_00154d1c(_Var4,local_88);
      uVar3 = FUN_0015e548(DAT_0019f8e0);
      uVar5 = gettext("%s: byte %s: %s %s followed by invalid byte 0x%02x");
      error(0,0,uVar5,uVar3,uVar2,param_2,local_a8,local_b4);
                    /* WARNING: Subroutine does not return */
      FUN_001177f4();
    }
    uVar2 = FUN_00129cfc(local_a8,0,param_3,param_4);
    *param_5 = uVar2;
    piVar6 = __errno_location();
    iVar1 = *piVar6;
    if (iVar1 != 0) {
      if (iVar1 == 0x22) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar2 = FUN_00154e5a(param_4,local_48);
        uVar3 = FUN_00154bde(param_3,local_68);
        _Var4 = ftello(param_1);
        uVar5 = FUN_00154d1c(_Var4,local_88);
        uVar7 = FUN_0015e548(DAT_0019f8e0);
        uVar8 = gettext("%s: byte %s: (valid range %s..%s)\n\t%s %s");
        error(0,0x22,uVar8,uVar7,uVar5,uVar3,uVar2,param_2,local_a8);
                    /* WARNING: Subroutine does not return */
        FUN_001177f4();
      }
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      _Var4 = ftello(param_1);
      uVar2 = FUN_00154d1c(_Var4,local_88);
      uVar3 = FUN_0015e548(DAT_0019f8e0);
      uVar5 = gettext("%s: byte %s: %s %s");
      error(0,iVar1,uVar5,uVar3,uVar2,param_2,local_a8);
                    /* WARNING: Subroutine does not return */
      FUN_001177f4();
    }
    uVar2 = 1;
  }
  if (local_30 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: read_timespec @ 0x238b6

void read_timespec(undefined8 param_1,undefined8 *param_2)

{
  char cVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined8 local_30;
  undefined8 local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_001233ed(param_1,&DAT_00181e04,0x8000000000000000,0x7fffffffffffffff,&local_30);
  if (cVar1 != '\0') {
    cVar1 = FUN_001233ed(param_1,&DAT_00181e08,0,999999999,&local_28);
    if (cVar1 != '\0') {
      *param_2 = local_30;
      param_2[1] = local_28;
      if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return;
    }
  }
  if (DAT_0019fdc8 != (code *)0x0) {
    (*DAT_0019fdc8)();
  }
  uVar2 = gettext("Unexpected EOF in snapshot file");
  uVar3 = FUN_0015e548(DAT_0019f8e0);
  error(0,0,"%s: %s",uVar3,uVar2);
                    /* WARNING: Subroutine does not return */
  FUN_001177f4();
}




// Function: read_incr_db_2 @ 0x239ca

/* WARNING: Heritage AFTER dead removal. Example location: s0xffffffffffffff60 : 0x00123bd8 */
/* WARNING: Restarted to delay deadcode elimination for space: stack */

void read_incr_db_2(void)

{
  ulong uVar1;
  char cVar2;
  int iVar3;
  undefined8 uVar4;
  __off_t _Var5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  long in_FS_OFFSET;
  bool bVar9;
  long local_118;
  ulong local_110;
  long local_108;
  long local_100;
  undefined1 *local_f8;
  ulong local_f0;
  ulong local_e8;
  undefined1 *local_e0;
  ulong local_d8;
  ulong local_d0;
  undefined1 *local_c8;
  ulong local_c0;
  undefined8 local_b8;
  undefined8 local_b0;
  undefined1 local_a8 [8];
  ulong local_a0;
  ulong local_98;
  ulong local_90;
  ulong local_88;
  ulong local_78;
  byte local_58;
  undefined1 local_48 [24];
  long local_30;
  
  local_30 = *(long *)(in_FS_OFFSET + 0x28);
  _obstack_begin(local_a8,0,0,FUN_00160b5b,PTR_free_0019bfc0);
  FUN_001238b6(DAT_0019f458,&DAT_0019f900);
  while( true ) {
    cVar2 = FUN_001233ed(DAT_0019f458,&DAT_00181e0d,0,1,&local_118);
    if (cVar2 != '\x01') {
      if (local_30 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    bVar9 = local_118 != 0;
    FUN_001238b6(DAT_0019f458,&local_b8);
    cVar2 = FUN_001233ed(DAT_0019f458,&DAT_00181e11,0,0xffffffffffffffff,&local_118);
    if (cVar2 != '\x01') break;
    local_108 = local_118;
    cVar2 = FUN_001233ed(DAT_0019f458,&DAT_00181e15,0,0xffffffffffffffff,&local_118);
    if (cVar2 != '\x01') break;
    local_100 = local_118;
    iVar3 = FUN_001232e4(DAT_0019f458,local_a8,&local_110);
    uVar1 = local_98;
    if (iVar3 != 0) break;
    local_f8 = local_a8;
    local_f0 = local_98;
    if (local_98 == local_90) {
      local_58 = local_58 | 2;
    }
    local_90 = ~local_78 & local_90 + local_78;
    if (local_88 - local_a0 < local_90 - local_a0) {
      local_90 = local_88;
    }
    local_98 = local_90;
    local_e8 = uVar1;
    do {
      iVar3 = FUN_001232e4(DAT_0019f458,local_a8,&local_110);
      if (iVar3 != 0) break;
    } while (1 < local_110);
    iVar3 = getc_unlocked(DAT_0019f458);
    uVar1 = local_98;
    if (iVar3 != 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar4 = gettext("Missing record terminator");
      _Var5 = ftello(DAT_0019f458);
      uVar6 = FUN_00154d1c(_Var5,local_48);
      uVar7 = FUN_0015e548(DAT_0019f8e0);
      uVar8 = gettext("%s: byte %s: %s");
      error(0,0,uVar8,uVar7,uVar6,uVar4);
                    /* WARNING: Subroutine does not return */
      FUN_001177f4();
    }
    local_e0 = local_a8;
    local_d8 = local_98;
    if (local_98 == local_90) {
      local_58 = local_58 | 2;
    }
    local_90 = ~local_78 & local_90 + local_78;
    if (local_88 - local_a0 < local_90 - local_a0) {
      local_90 = local_88;
    }
    local_98 = local_90;
    local_d0 = uVar1;
    FUN_00121019(local_e8,local_b8,local_b0,local_108,local_100,bVar9,0,uVar1);
    local_c8 = local_a8;
    local_c0 = local_d0;
    if ((local_a0 < local_d0) && (local_d0 < local_88)) {
      local_98 = local_d0;
      local_90 = local_d0;
    }
    else {
      _obstack_free(local_c8,local_d0);
    }
  }
  if (DAT_0019fdc8 != (code *)0x0) {
    (*DAT_0019fdc8)();
  }
  uVar4 = gettext("Unexpected EOF in snapshot file");
  uVar6 = FUN_0015e548(DAT_0019f8e0);
  error(0,0,"%s: %s",uVar6,uVar4);
                    /* WARNING: Subroutine does not return */
  FUN_001177f4();
}




// Function: show_snapshot_field_ranges @ 0x23f6b

void show_snapshot_field_ranges(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined **local_60;
  undefined1 local_58 [32];
  undefined1 local_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  printf("This tar\'s snapshot file field ranges are\n");
  printf("   (%-15s => [ %s, %s ]):\n\n","field name",&DAT_00181e8f,&DAT_00181e8b);
  for (local_60 = &PTR_DAT_0019a640; *local_60 != (undefined *)0x0; local_60 = local_60 + 3) {
    uVar1 = FUN_00129c96(local_60[2],local_60[1],local_60[2],local_38);
    uVar2 = FUN_00129c96(local_60[1],local_60[1],local_60[2],local_58);
    printf("    %-15s => [ %s, %s ],\n",*local_60,uVar2,uVar1);
  }
  printf("\n");
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: read_directory_file @ 0x24090

void read_directory_file(void)

{
  int iVar1;
  __ssize_t _Var2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  int local_38;
  char *local_30;
  size_t local_28;
  char *local_20;
  ulong local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_30 = (char *)0x0;
  local_28 = 0;
  local_38 = 0x42;
  if (DAT_0019f8e8 == 0) {
    local_38 = 0x242;
  }
  iVar1 = open(DAT_0019f8e0,local_38,0x1b6);
  if (iVar1 < 0) {
    FUN_0013e38b(DAT_0019f8e0);
  }
  else {
    DAT_0019f458 = fdopen(iVar1,"r+");
    if (DAT_0019f458 == (FILE *)0x0) {
      FUN_0013e38b(DAT_0019f8e0);
      close(iVar1);
    }
    else {
      FUN_0012eda0();
      FUN_0012ee43();
      _Var2 = getline(&local_30,&local_28,DAT_0019f458);
      if (0 < _Var2) {
        iVar1 = strncmp(local_30,"GNU tar",7);
        if (iVar1 == 0) {
          local_20 = local_30 + 8;
          if (local_30[7] != '-') {
            if (DAT_0019fdc8 != (code *)0x0) {
              (*DAT_0019fdc8)();
            }
            uVar3 = gettext("Bad incremental file format");
            error(1,0,uVar3);
            DAT_0019fdd0 = 2;
          }
          for (; *local_20 != '-'; local_20 = local_20 + 1) {
            if (*local_20 == '\0') {
              if (DAT_0019fdc8 != (code *)0x0) {
                (*DAT_0019fdc8)();
              }
              uVar3 = gettext("Bad incremental file format");
              error(1,0,uVar3);
              DAT_0019fdd0 = 2;
            }
          }
          local_18 = strtoumax(local_20 + 1,(char **)0x0,10);
        }
        else {
          local_18 = 0;
        }
        if (local_18 < 2) {
          FUN_00122c72(local_18 & 0xffffffff,local_30);
        }
        else if (local_18 == 2) {
          FUN_001239ca();
        }
        else {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar3 = gettext("Unsupported incremental format version: %lu");
          error(1,0,uVar3,local_18);
          DAT_0019fdd0 = 2;
        }
      }
      iVar1 = ferror_unlocked(DAT_0019f458);
      if (iVar1 != 0) {
        FUN_0013e403(DAT_0019f8e0);
      }
      free(local_30);
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: write_directory_file_entry @ 0x24350

undefined8 write_directory_file_entry(long param_1,FILE *param_2)

{
  int iVar1;
  size_t sVar2;
  undefined4 extraout_var;
  long in_FS_OFFSET;
  void *local_50;
  char *local_48;
  long local_40;
  FILE *local_38;
  char *local_30;
  undefined1 local_28 [24];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_40 = param_1;
  local_38 = param_2;
  if ((*(uint *)(param_1 + 0x3c) & 4) != 0) {
    if ((*(uint *)(param_1 + 0x3c) & 2) == 0) {
      local_30 = &DAT_00181f2e;
    }
    else {
      local_30 = &DAT_00181f2c;
    }
    fwrite_unlocked(local_30,2,1,param_2);
    local_30 = (char *)FUN_00129c96(*(undefined8 *)(local_40 + 8),0x8000000000000000,
                                    0x7fffffffffffffff,local_28);
    sVar2 = strlen(local_30);
    fwrite_unlocked(local_30,sVar2 + 1,1,local_38);
    local_30 = (char *)FUN_00154bde(*(undefined8 *)(local_40 + 0x10),local_28);
    sVar2 = strlen(local_30);
    fwrite_unlocked(local_30,sVar2 + 1,1,local_38);
    local_30 = (char *)FUN_00129c96(*(undefined8 *)(local_40 + 0x18),0,0xffffffffffffffff,local_28);
    sVar2 = strlen(local_30);
    fwrite_unlocked(local_30,sVar2 + 1,1,local_38);
    local_30 = (char *)FUN_00129c96(*(undefined8 *)(local_40 + 0x20),0,0xffffffffffffffff,local_28);
    sVar2 = strlen(local_30);
    fwrite_unlocked(local_30,sVar2 + 1,1,local_38);
    sVar2 = strlen(*(char **)(local_40 + 0x58));
    fwrite_unlocked(*(void **)(local_40 + 0x58),sVar2 + 1,1,local_38);
    if (*(long *)(local_40 + 0x28) != 0) {
      local_48 = (char *)FUN_00120b9f(*(undefined8 *)(local_40 + 0x28),0,&local_50);
      while (local_48 != (char *)0x0) {
        sVar2 = strlen(local_48);
        fwrite_unlocked(local_48,sVar2 + 1,1,local_38);
        local_48 = (char *)FUN_00120ad3(local_50);
      }
      free(local_50);
    }
    fwrite_unlocked(&DAT_00181f30,2,1,local_38);
  }
  iVar1 = ferror_unlocked(local_38);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),iVar1 == 0);
}




// Function: write_directory_file @ 0x24600

void write_directory_file(void)

{
  FILE *__stream;
  int iVar1;
  char *pcVar2;
  size_t sVar3;
  long in_FS_OFFSET;
  undefined1 local_28 [24];
  long local_10;
  
  __stream = DAT_0019f458;
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0019f458 != (FILE *)0x0) {
    iVar1 = fseeko(DAT_0019f458,0,0);
    if (iVar1 != 0) {
      FUN_0013e780(DAT_0019f8e0);
    }
    iVar1 = fileno(__stream);
    iVar1 = FUN_00132f52(iVar1);
    if (iVar1 != 0) {
      FUN_0013ea55(DAT_0019f8e0);
    }
    fprintf(__stream,"%s-%s-%d\n","GNU tar",&DAT_00181f33,2);
    pcVar2 = (char *)FUN_00154bde(DAT_0019f9e0,local_28);
    sVar3 = strlen(pcVar2);
    fwrite_unlocked(pcVar2,sVar3 + 1,1,__stream);
    pcVar2 = (char *)FUN_00154e5a(DAT_0019f9e8,local_28);
    sVar3 = strlen(pcVar2);
    fwrite_unlocked(pcVar2,sVar3 + 1,1,__stream);
    iVar1 = ferror_unlocked(__stream);
    if ((iVar1 == 0) && (DAT_0019f448 != 0)) {
      FUN_0015279c(DAT_0019f448,FUN_00124350,__stream);
    }
    iVar1 = ferror_unlocked(__stream);
    if (iVar1 != 0) {
      FUN_0013eb22(DAT_0019f8e0);
    }
    iVar1 = fclose(__stream);
    if (iVar1 != 0) {
      FUN_0013e1ff(DAT_0019f8e0);
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: get_gnu_dumpdir @ 0x247c6

void get_gnu_dumpdir(long param_1)

{
  void *pvVar1;
  void *__src;
  undefined8 uVar2;
  ulong local_30;
  ulong local_28;
  void *local_20;
  
  local_30 = *(ulong *)(param_1 + 0x88);
  pvVar1 = (void *)FUN_00160b5b(local_30);
  FUN_0010bba7(DAT_0019f460);
  FUN_0010ad2b(param_1);
  local_20 = pvVar1;
  for (; local_30 != 0; local_30 = local_30 - local_28) {
    FUN_0010ad86(local_30);
    __src = (void *)FUN_0010bb4b();
    if (__src == (void *)0x0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar2 = gettext("Unexpected EOF in archive");
      error(1,0,uVar2);
      DAT_0019fdd0 = 2;
    }
    local_28 = FUN_0010bbf5(__src);
    if (local_30 < local_28) {
      local_28 = local_30;
    }
    memcpy(local_20,__src,local_28);
    local_20 = (void *)((long)local_20 + local_28);
    FUN_0010bba7((long)__src + (local_28 - 1));
  }
  FUN_0010ad66();
  *(void **)(param_1 + 400) = pvVar1;
  *(undefined1 *)(param_1 + 0x189) = 1;
  return;
}




// Function: is_dumpdir @ 0x2490b

undefined1 is_dumpdir(long param_1)

{
  if ((*(char *)(param_1 + 0x188) != '\0') && (*(long *)(param_1 + 400) == 0)) {
    FUN_001247c6(param_1);
  }
  return *(undefined1 *)(param_1 + 0x188);
}




// Function: dumpdir_ok @ 0x24953

undefined8 dumpdir_ok(char *param_1)

{
  char cVar1;
  bool bVar2;
  size_t sVar3;
  undefined8 uVar4;
  int local_24;
  char *local_20;
  
  bVar2 = false;
  local_24 = 0;
  local_20 = param_1;
  do {
    if (*local_20 == '\0') {
      if (local_24 == 0) {
        if ((bVar2) && ((DAT_0019ec84 & 2) != 0)) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar4 = gettext("Malformed dumpdir: \'X\' never used");
          error(0,0,uVar4);
        }
        uVar4 = 1;
      }
      else {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar4 = gettext("Malformed dumpdir: expected \'%c\' but found end of data");
        error(0,0,uVar4,local_24);
        DAT_0019fdd0 = 2;
        uVar4 = 0;
      }
      return uVar4;
    }
    if ((local_24 != 0) && (local_24 != *local_20)) {
      cVar1 = *local_20;
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar4 = gettext("Malformed dumpdir: expected \'%c\' but found %#3o");
      error(0,0,uVar4,local_24,cVar1);
      DAT_0019fdd0 = 2;
      return 0;
    }
    switch(*local_20) {
    case 'R':
      if (local_20[1] == '\0') {
        if (!bVar2) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar4 = gettext("Malformed dumpdir: empty name in \'R\'");
          error(0,0,uVar4);
          DAT_0019fdd0 = 2;
          return 0;
        }
        bVar2 = false;
      }
      local_24 = 0x54;
      break;
    case 'T':
      if (local_24 != 0x54) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar4 = gettext("Malformed dumpdir: \'T\' not preceded by \'R\'");
        error(0,0,uVar4);
        DAT_0019fdd0 = 2;
        return 0;
      }
      if ((local_20[1] == '\0') && (!bVar2)) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar4 = gettext("Malformed dumpdir: empty name in \'T\'");
        error(0,0,uVar4);
        DAT_0019fdd0 = 2;
        return 0;
      }
      local_24 = 0;
      break;
    case 'X':
      if (bVar2) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar4 = gettext("Malformed dumpdir: \'X\' duplicated");
        error(0,0,uVar4);
        DAT_0019fdd0 = 2;
        return 0;
      }
      bVar2 = true;
    }
    sVar3 = strlen(local_20);
    local_20 = local_20 + sVar3 + 1;
  } while( true );
}




// Function: try_purge_directory @ 0x24ca0

undefined8 try_purge_directory(undefined8 param_1)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  char *__ptr;
  int *piVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  char *pcVar7;
  size_t sVar8;
  long in_FS_OFFSET;
  char *local_108;
  char *local_100;
  void *local_f8;
  char *local_f0;
  char *local_e8;
  char *local_e0;
  long local_b8 [3];
  uint local_a0;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_f0 = (char *)0x0;
  cVar1 = FUN_0012490b(&DAT_0019fa20);
  if (cVar1 == '\x01') {
    __ptr = (char *)FUN_0012b8e1(param_1,0);
    if (__ptr == (char *)0x0) {
      uVar3 = 0;
    }
    else {
      cVar1 = FUN_00124953(DAT_0019fbb0);
      if (cVar1 == '\x01') {
        for (local_100 = DAT_0019fbb0; *local_100 != '\0'; local_100 = local_100 + sVar8 + 1) {
          if (*local_100 == 'X') {
            sVar8 = strlen(local_100 + 1);
            local_f0 = (char *)FUN_00160b8d(local_f0,sVar8 + 0xc);
            memcpy(local_f0,local_100 + 1,sVar8);
            local_f0[sVar8] = '/';
            memcpy(local_f0 + sVar8 + 1,"tar.XXXXXX",0xb);
            pcVar7 = mkdtemp(local_f0);
            if (pcVar7 == (char *)0x0) {
              if (DAT_0019fdc8 != (code *)0x0) {
                (*DAT_0019fdc8)();
              }
              uVar3 = FUN_0015e84e(local_f0);
              uVar5 = gettext("Cannot create temporary directory using template %s");
              piVar4 = __errno_location();
              error(0,*piVar4,uVar5,uVar3);
              DAT_0019fdd0 = 2;
              free(local_f0);
              free(__ptr);
              uVar3 = 0;
              goto LAB_00125357;
            }
          }
          else if (*local_100 == 'R') {
            local_e8 = local_100 + 1;
            sVar8 = strlen(local_100);
            local_100 = local_100 + sVar8 + 1;
            local_e0 = local_100 + 1;
            if (*local_e8 != '\0') {
              local_e8 = (char *)FUN_0013edd9(local_e8,0,DAT_0019f878);
            }
            if (*local_e0 != '\0') {
              local_e0 = (char *)FUN_0013edd9(local_e0,0,DAT_0019f878);
            }
            if (*local_e8 == '\0') {
              local_e8 = local_f0;
            }
            else if (*local_e0 == '\0') {
              local_e0 = local_f0;
            }
            cVar1 = FUN_0011c326(local_e8,local_e0);
            if (cVar1 != '\x01') {
              free(local_f0);
              free(__ptr);
              uVar3 = 0;
              goto LAB_00125357;
            }
          }
          sVar8 = strlen(local_100);
        }
        free(local_f0);
        uVar3 = FUN_001209d4(DAT_0019fbb0);
        local_f8 = (void *)0x0;
        for (local_108 = __ptr; *local_108 != '\0'; local_108 = local_108 + sVar8 + 1) {
          free(local_f8);
          local_f8 = (void *)FUN_0012ee85(param_1,local_108);
          iVar2 = FUN_0012a9b8(local_f8,local_b8);
          if (iVar2 == 0) {
            pcVar7 = (char *)FUN_00120a6a(uVar3,local_108);
            if (((pcVar7 == (char *)0x0) || ((*pcVar7 == 'D' && ((local_a0 & 0xf000) != 0x4000))))
               || ((*pcVar7 == 'Y' && ((local_a0 & 0xf000) == 0x4000)))) {
              if ((DAT_0019f935 == '\0') || (local_b8[0] == DAT_0019fc20)) {
                if (DAT_0019f8c8 == '\x01') {
                  iVar2 = FUN_00134af4("delete",local_f8);
                  if (iVar2 == 0) goto LAB_001252f9;
                }
                if (DAT_0019f9ac != 0) {
                  uVar6 = FUN_0015e84e(local_f8);
                  uVar5 = DAT_0019fe48;
                  pcVar7 = (char *)gettext("%s: Deleting %s\n");
                  fprintf(DAT_0019efd0,pcVar7,uVar5,uVar6);
                }
                iVar2 = FUN_0012a36f(local_f8,1);
                if (iVar2 == 0) {
                  piVar4 = __errno_location();
                  iVar2 = *piVar4;
                  if (DAT_0019fdc8 != (code *)0x0) {
                    (*DAT_0019fdc8)();
                  }
                  uVar5 = FUN_0015e548(local_f8);
                  uVar6 = gettext("%s: Cannot remove");
                  error(0,iVar2,uVar6,uVar5);
                  DAT_0019fdd0 = 2;
                }
              }
              else {
                if (DAT_0019fdc8 != (code *)0x0) {
                  (*DAT_0019fdc8)();
                }
                uVar5 = FUN_0015e548(local_f8);
                uVar6 = gettext("%s: directory is on a different device: not purging");
                error(0,0,uVar6,uVar5);
              }
            }
          }
          else {
            piVar4 = __errno_location();
            if (*piVar4 != 2) {
              FUN_0012b444(local_f8);
              if (DAT_0019fdc8 != (code *)0x0) {
                (*DAT_0019fdc8)();
              }
              uVar5 = FUN_0015e548(local_f8);
              uVar6 = gettext("%s: Not purging directory: unable to stat");
              error(0,0,uVar6,uVar5);
            }
          }
LAB_001252f9:
          sVar8 = strlen(local_108);
        }
        free(local_f8);
        FUN_001209fc(uVar3);
        free(__ptr);
        uVar3 = 1;
      }
      else {
        uVar3 = 0;
      }
    }
  }
  else {
    uVar3 = 0;
  }
LAB_00125357:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar3;
}




// Function: purge_directory @ 0x25377

void purge_directory(undefined8 param_1)

{
  char cVar1;
  
  cVar1 = FUN_00124ca0(param_1);
  if (cVar1 != '\x01') {
    FUN_00128891();
  }
  return;
}




// Function: list_dumpdir @ 0x253a2

void list_dumpdir(char *param_1,long param_2)

{
  char cVar1;
  bool bVar2;
  long local_28;
  char *local_20;
  
  bVar2 = false;
  local_28 = param_2;
  local_20 = param_1;
  while (local_28 != 0) {
    cVar1 = *local_20;
    if (cVar1 == '\0') {
      fputc_unlocked(10,DAT_0019efd0);
      local_20 = local_20 + 1;
      local_28 = local_28 + -1;
      bVar2 = false;
    }
    else if ((((cVar1 < '\0') || ('Y' < cVar1)) || (cVar1 < 'D')) ||
            ((0x314401UL >> (cVar1 + 0xbcU & 0x3f) & 1) == 0)) {
      fputc_unlocked((int)*local_20,DAT_0019efd0);
      local_20 = local_20 + 1;
      local_28 = local_28 + -1;
    }
    else {
      fprintf(DAT_0019efd0,"%c",(ulong)(uint)(int)*local_20);
      if (!bVar2) {
        fprintf(DAT_0019efd0," ");
        bVar2 = true;
      }
      local_20 = local_20 + 1;
      local_28 = local_28 + -1;
    }
  }
  return;
}




// Function: base64_init @ 0x254c9

void base64_init(void)

{
  int local_c;
  
  memset(&DAT_0019f4a0,0x40,0x100);
  for (local_c = 0; local_c < 0x40; local_c = local_c + 1) {
    (&DAT_0019f4a0)
    [(int)"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/%s: Omitting"[local_c]] =
         (char)local_c;
  }
  return;
}




// Function: decode_xform @ 0x25529

char * decode_xform(char *param_1,int *param_2)

{
  int iVar1;
  char *local_20;
  size_t local_10;
  
  iVar1 = *param_2;
  if (iVar1 != 4) {
    local_20 = param_1;
    if (iVar1 < 5) {
      if (iVar1 == 1) {
        local_20 = (char *)FUN_0013edd9(param_1,0,DAT_0019f878);
      }
      else if (iVar1 == 2) {
        local_20 = (char *)FUN_0013edd9(param_1,1,DAT_0019f878);
      }
    }
    param_1 = local_20;
    if (DAT_0019f970 != 0) {
      local_10 = FUN_0012ef5c(local_20,DAT_0019f970);
      if (local_10 == 0xffffffffffffffff) {
        local_10 = strlen(local_20);
      }
      param_1 = local_20 + local_10;
    }
  }
  return param_1;
}




// Function: transform_member_name @ 0x255f5

void transform_member_name(undefined8 param_1,undefined4 param_2)

{
  undefined4 local_14;
  undefined8 local_10;
  
  local_14 = param_2;
  local_10 = param_1;
  FUN_0013b8af(param_1,param_2,FUN_00125529,&local_14);
  return;
}




// Function: enforce_one_top_level @ 0x25627

void enforce_one_top_level(undefined8 *param_1)

{
  char *__ptr;
  int iVar1;
  int iVar2;
  size_t sVar3;
  undefined8 uVar4;
  char *local_18;
  
  __ptr = (char *)*param_1;
  for (local_18 = __ptr; (*local_18 != '\0' && ((*local_18 == '/' || (*local_18 == '.'))));
      local_18 = local_18 + 1) {
  }
  if (*local_18 == '\0') {
    uVar4 = FUN_00160cc6(DAT_0019f938);
    *param_1 = uVar4;
  }
  else {
    sVar3 = strlen(DAT_0019f938);
    iVar1 = (int)sVar3;
    iVar2 = strncmp(local_18,DAT_0019f938,(long)iVar1);
    if (iVar2 == 0) {
      if (local_18[iVar1] == '/') {
        return;
      }
      if (local_18[iVar1] == '\0') {
        return;
      }
    }
    uVar4 = FUN_0012ee85(DAT_0019f938,__ptr);
    *param_1 = uVar4;
    FUN_00129946(*param_1);
  }
  free(__ptr);
  return;
}




// Function: transform_stat_info @ 0x25733

void transform_stat_info(int param_1,long param_2)

{
  if (param_1 != 0x56) {
    FUN_001255f5(param_2 + 8,1);
    if (param_1 == 0x31) {
      FUN_001255f5(param_2 + 0x18,2);
    }
    else if (param_1 == 0x32) {
      FUN_001255f5(param_2 + 0x18,4);
    }
    if (DAT_0019f936 != '\0') {
      FUN_00125627(&DAT_0019fa28);
    }
  }
  return;
}




// Function: read_and @ 0x257b8

void read_and(code *param_1)

{
  char cVar1;
  undefined4 uVar2;
  char extraout_var;
  int iVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  char *pcVar6;
  long lVar7;
  long in_FS_OFFSET;
  undefined4 local_58;
  undefined1 local_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_58 = 0;
  FUN_001254c9();
  FUN_0012d3b1();
  FUN_0010f694(0);
  do {
    FUN_00139d1c(&DAT_0019fa20);
    uVar2 = FUN_00125ecf(&DAT_0019f460,&DAT_0019fa20,0);
    switch(uVar2) {
    case 0:
    case 2:
                    /* WARNING: Subroutine does not return */
      abort();
    case 1:
      FUN_00126747(DAT_0019f460,&DAT_0019fa20,&DAT_0019f468,1);
      cVar1 = FUN_0012d88a(DAT_0019fa28);
      if (cVar1 == '\x01') {
        if (-1 < DAT_0019f908) {
          DAT_0019fb18 = FUN_001279d7(DAT_0019f460 + 0x88,0xc);
          DAT_0019fb20 = 0;
          FUN_0015ff90(DAT_0019fb18,0,DAT_0019f900,DAT_0019f908);
          if (extraout_var < '\0') goto LAB_00125932;
        }
        cVar1 = FUN_00117c43(DAT_0019fa28,DAT_0019fbb8);
        if (cVar1 != '\0') goto LAB_00125932;
      }
      else {
LAB_00125932:
        cVar1 = *(char *)(DAT_0019f460 + 0x9c);
        if (cVar1 != 'V') {
          if (cVar1 < 'W') {
            if (cVar1 == '5') {
              if (DAT_0019f978 != '\0') {
                if (DAT_0019fdc8 != (code *)0x0) {
                  (*DAT_0019fdc8)();
                }
                uVar5 = FUN_0015e548(DAT_0019fa28);
                uVar4 = gettext("%s: Omitting");
                error(0,0,uVar4,uVar5);
              }
            }
            else if (cVar1 == 'M') goto LAB_001259bf;
          }
          FUN_00128891();
          break;
        }
      }
LAB_001259bf:
      FUN_00125733((int)*(char *)(DAT_0019f460 + 0x9c),&DAT_0019fa20);
      (*param_1)();
      break;
    case 3:
      if (DAT_0019f88c != '\0') {
        uVar5 = FUN_0010bacd();
        uVar5 = FUN_00154e5a(uVar5,local_38);
        pcVar6 = (char *)gettext("block %s: ** Block of NULs **\n");
        fprintf(DAT_0019efd0,pcVar6,uVar5);
      }
      FUN_0010bba7(DAT_0019f460);
      uVar2 = local_58;
      if (DAT_0019f8bd != '\x01') {
        iVar3 = FUN_00125ecf(&DAT_0019f460,&DAT_0019fa20,0);
        if ((iVar3 != 3) && ((DAT_0019ec84 & 1) != 0)) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar5 = FUN_0010bacd();
          uVar5 = FUN_00154e5a(uVar5,local_38);
          uVar4 = gettext("A lone zero block at %s");
          error(0,0,uVar4,uVar5);
        }
        goto switchD_00125854_default;
      }
      break;
    case 4:
      if (DAT_0019f88c != '\0') {
        uVar5 = FUN_0010bacd();
        uVar5 = FUN_00154e5a(uVar5,local_38);
        pcVar6 = (char *)gettext("block %s: ** End of File **\n");
        fprintf(DAT_0019efd0,pcVar6,uVar5);
      }
      goto switchD_00125854_default;
    case 5:
      FUN_0010bba7(DAT_0019f460);
      switch(local_58) {
      case 0:
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar5 = gettext("This does not look like a tar archive");
        error(0,0,uVar5);
        DAT_0019fdd0 = 2;
      case 1:
      case 3:
        if (DAT_0019f88c != '\0') {
          lVar7 = FUN_0010bacd();
          uVar5 = FUN_00154e5a((lVar7 - DAT_0019f480) - DAT_0019f488,local_38);
          pcVar6 = (char *)gettext("block %s: ");
          fprintf(DAT_0019efd0,pcVar6,uVar5);
        }
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar5 = gettext("Skipping to next header");
        error(0,0,uVar5);
        DAT_0019fdd0 = 2;
      case 4:
      case 5:
        break;
      case 2:
                    /* WARNING: Subroutine does not return */
        abort();
      }
      break;
    default:
      goto switchD_00125854_default;
    }
    local_58 = uVar2;
    cVar1 = FUN_0012da15(&DAT_0019fa20);
  } while (cVar1 != '\x01');
switchD_00125854_default:
  FUN_0010d046();
  FUN_0012dbba();
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: list_archive @ 0x25cf3

void list_archive(void)

{
  char cVar1;
  undefined8 uVar2;
  
  uVar2 = FUN_0010bacd();
  if (DAT_0019f9ac != 0) {
    FUN_0012864c(&DAT_0019fa20,DAT_0019f460,uVar2);
  }
  if ((DAT_0019f8be != '\0') && (2 < DAT_0019f9ac)) {
    cVar1 = FUN_0012490b(&DAT_0019fa20);
    if (cVar1 != '\0') {
      uVar2 = FUN_00120bfe(DAT_0019fbb0);
      FUN_001253a2(DAT_0019fbb0,uVar2);
    }
  }
  FUN_00128891();
  return;
}




// Function: tar_checksum @ 0x25d84

undefined8 tar_checksum(byte *param_1,undefined1 param_2)

{
  int iVar1;
  undefined8 uVar2;
  int local_28;
  int local_24;
  long local_18;
  byte *local_10;
  
  local_28 = 0;
  local_24 = 0;
  local_18 = 0x200;
  local_10 = param_1;
  while (local_18 != 0) {
    local_28 = local_28 + (uint)*local_10;
    local_24 = local_24 + (char)*local_10;
    local_18 = local_18 + -1;
    local_10 = local_10 + 1;
  }
  if (local_28 == 0) {
    uVar2 = 3;
  }
  else {
    local_18 = 8;
    while (local_18 != 0) {
      local_28 = local_28 - (uint)param_1[local_18 + 0x93];
      local_24 = local_24 - (char)param_1[local_18 + 0x93];
      local_18 = local_18 + -1;
    }
    iVar1 = FUN_00126c90(param_1 + 0x94,8,0,0,0x7fffffff,1,param_2);
    if (iVar1 < 0) {
      uVar2 = 5;
    }
    else if ((local_28 + 0x100 == iVar1) || (local_24 + 0x100 == iVar1)) {
      uVar2 = 1;
    }
    else {
      uVar2 = 5;
    }
  }
  return uVar2;
}




// Function: read_header @ 0x25ecf

int read_header(undefined8 *param_1,long param_2,int param_3)

{
  char *pcVar1;
  undefined1 uVar2;
  int iVar3;
  undefined8 *puVar4;
  ulong uVar5;
  void *__src;
  undefined8 uVar6;
  size_t sVar7;
  ulong uVar8;
  long lVar9;
  undefined8 *puVar10;
  long in_FS_OFFSET;
  byte bVar11;
  int local_1b4;
  undefined8 *local_1b0;
  ulong local_1a8;
  ulong local_1a0;
  undefined8 *local_198;
  undefined8 *local_190;
  ulong local_188;
  ulong local_180;
  char *local_178;
  char *local_170;
  undefined1 local_138 [32];
  char local_118 [100];
  undefined1 local_b4;
  undefined1 local_7d;
  long local_10;
  
  bVar11 = 0;
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_198 = (undefined8 *)0x0;
  local_190 = (undefined8 *)0x0;
  local_188 = 0;
  local_180 = 0;
  while( true ) {
    puVar4 = (undefined8 *)FUN_0010bb4b();
    *param_1 = puVar4;
    if (puVar4 == (undefined8 *)0x0) {
      local_1b4 = 4;
      goto LAB_0012670d;
    }
    local_1b4 = FUN_00125d84(puVar4,0);
    if (local_1b4 != 1) goto LAB_0012670d;
    if (*(char *)((long)puVar4 + 0x9c) == '1') {
      *(undefined8 *)(param_2 + 0x88) = 0;
    }
    else {
      uVar6 = FUN_0012798b((long)puVar4 + 0x7c,0xc);
      *(undefined8 *)(param_2 + 0x88) = uVar6;
      if (*(long *)(param_2 + 0x88) < 0) {
        local_1b4 = 5;
        goto LAB_0012670d;
      }
    }
    if ((((*(char *)((long)puVar4 + 0x9c) != 'L') && (*(char *)((long)puVar4 + 0x9c) != 'K')) &&
        (*(char *)((long)puVar4 + 0x9c) != 'x')) &&
       ((*(char *)((long)puVar4 + 0x9c) != 'g' && (*(char *)((long)puVar4 + 0x9c) != 'X')))) break;
    if (param_3 == 1) {
      local_1b4 = 2;
      goto LAB_0012670d;
    }
    if ((*(char *)((long)puVar4 + 0x9c) == 'L') || (*(char *)((long)puVar4 + 0x9c) == 'K')) {
      uVar8 = *(ulong *)(param_2 + 0x88);
      uVar5 = (ulong)((uint)uVar8 & 0x1ff);
      local_1a8 = uVar8 + 0x200;
      if (uVar5 != 0) {
        local_1a8 = (local_1a8 - uVar5) + 0x200;
      }
      if ((*(ulong *)(param_2 + 0x88) != uVar8) || (local_1a8 < uVar8)) {
        FUN_0011783c();
      }
      local_1b0 = (undefined8 *)FUN_00160b5b(local_1a8 + 1);
      if (*(char *)((long)puVar4 + 0x9c) == 'L') {
        free(local_198);
        local_188 = local_1a8 >> 9;
        local_198 = local_1b0;
      }
      else {
        free(local_190);
        local_180 = local_1a8 >> 9;
        local_190 = local_1b0;
      }
      FUN_0010bba7(puVar4);
      *local_1b0 = *puVar4;
      local_1b0[0x3f] = puVar4[0x3f];
      lVar9 = (long)local_1b0 - (long)((ulong)(local_1b0 + 1) & 0xfffffffffffffff8);
      puVar4 = (undefined8 *)((long)puVar4 - lVar9);
      puVar10 = (undefined8 *)((ulong)(local_1b0 + 1) & 0xfffffffffffffff8);
      for (uVar8 = (ulong)((int)lVar9 + 0x200U >> 3); uVar8 != 0; uVar8 = uVar8 - 1) {
        *puVar10 = *puVar4;
        puVar4 = puVar4 + (ulong)bVar11 * -2 + 1;
        puVar10 = puVar10 + (ulong)bVar11 * -2 + 1;
      }
      local_1b0 = local_1b0 + 0x40;
      for (local_1a8 = local_1a8 - 0x200; local_1a8 != 0; local_1a8 = local_1a8 - local_1a0) {
        __src = (void *)FUN_0010bb4b();
        if (__src == (void *)0x0) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar6 = gettext("Unexpected EOF in archive");
          error(0,0,uVar6);
          DAT_0019fdd0 = 2;
          break;
        }
        local_1a0 = FUN_0010bbf5(__src);
        if (local_1a8 < local_1a0) {
          local_1a0 = local_1a8;
        }
        memcpy(local_1b0,__src,local_1a0);
        local_1b0 = (undefined8 *)((long)local_1b0 + local_1a0);
        FUN_0010bba7((long)__src + (local_1a0 - 1));
      }
      *(undefined1 *)local_1b0 = 0;
    }
    else if ((*(char *)((long)puVar4 + 0x9c) == 'x') || (*(char *)((long)puVar4 + 0x9c) == 'X')) {
      uVar6 = FUN_0012798b((long)puVar4 + 0x7c,0xc);
      FUN_0011e28e(param_2 + 0x168,puVar4,uVar6);
    }
    else if (*(char *)((long)puVar4 + 0x9c) == 'g') {
      if (DAT_0019f490 == (void *)0x0) {
        DAT_0019f490 = (void *)FUN_00160b5b(0x200);
      }
      memcpy(DAT_0019f490,puVar4,0x200);
      memset(local_138,0,0x20);
      uVar6 = FUN_0012798b((long)puVar4 + 0x7c,0xc);
      FUN_0011e28e(local_138,puVar4,uVar6);
      FUN_0011e0e7(local_138);
      FUN_0011e785(local_138);
      if (param_3 == 2) {
        local_1b4 = 2;
LAB_0012670d:
        free(local_198);
        free(local_190);
        if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
          return local_1b4;
        }
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
    }
  }
  free(DAT_0019f470);
  if (local_198 == (undefined8 *)0x0) {
    local_170 = local_118;
    if ((*(char *)((long)puVar4 + 0x159) != '\0') &&
       (iVar3 = strcmp((char *)((long)puVar4 + 0x101),"ustar"), iVar3 == 0)) {
      memcpy(local_170,(void *)((long)puVar4 + 0x159),0x9b);
      local_7d = 0;
      sVar7 = strlen(local_170);
      pcVar1 = local_170 + sVar7;
      local_170 = local_118 + sVar7 + 1;
      *pcVar1 = '/';
    }
    memcpy(local_170,puVar4,100);
    local_170[100] = '\0';
    local_178 = local_118;
    DAT_0019f470 = (undefined8 *)0x0;
    DAT_0019f480 = 0;
  }
  else {
    local_178 = (char *)(local_198 + 0x40);
    DAT_0019f470 = local_198;
    DAT_0019f480 = local_188;
    local_198 = (undefined8 *)0x0;
  }
  FUN_00129547(param_2,local_178);
  FUN_00129547(param_2 + 8,local_178);
  uVar2 = FUN_0014e2e5(*(undefined8 *)(param_2 + 8));
  *(undefined1 *)(param_2 + 0x10) = uVar2;
  free(DAT_0019f478);
  if (local_190 == (undefined8 *)0x0) {
    memcpy(local_118,(void *)((long)puVar4 + 0x9d),100);
    local_b4 = 0;
    local_178 = local_118;
    DAT_0019f478 = (undefined8 *)0x0;
    DAT_0019f488 = 0;
  }
  else {
    local_178 = (char *)(local_190 + 0x40);
    DAT_0019f478 = local_190;
    DAT_0019f488 = local_180;
    local_190 = (undefined8 *)0x0;
  }
  FUN_00129547(param_2 + 0x18,local_178);
  goto LAB_0012670d;
}




// Function: decode_header @ 0x26747

void decode_header(long param_1,long param_2,int *param_3,int param_4)

{
  char cVar1;
  int iVar2;
  undefined4 uVar3;
  uint __minor;
  uint __major;
  undefined8 uVar4;
  long lVar5;
  ulonglong uVar6;
  long in_FS_OFFSET;
  char local_29;
  int local_28;
  undefined4 local_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_24 = FUN_0012790d(param_1 + 100,8,&local_29);
  iVar2 = strcmp((char *)(param_1 + 0x101),"ustar");
  if (iVar2 == 0) {
    if (((((*(char *)(param_1 + 0x1db) == '\0') && ('/' < *(char *)(param_1 + 0x1dc))) &&
         (*(char *)(param_1 + 0x1dc) < '8')) &&
        ((*(char *)(param_1 + 0x1e7) == ' ' && ('/' < *(char *)(param_1 + 0x1e8))))) &&
       ((*(char *)(param_1 + 0x1e8) < '8' && (*(char *)(param_1 + 499) == ' ')))) {
      local_28 = 5;
    }
    else if (*(long *)(param_2 + 0x170) == 0) {
      local_28 = 3;
    }
    else {
      local_28 = 4;
    }
  }
  else {
    iVar2 = strcmp((char *)(param_1 + 0x101),"ustar  ");
    if (iVar2 == 0) {
      if (local_29 == '\0') {
        local_28 = 6;
      }
      else {
        local_28 = 2;
      }
    }
    else {
      local_28 = 1;
    }
  }
  *param_3 = local_28;
  *(undefined4 *)(param_2 + 0x70) = local_24;
  uVar4 = FUN_001279d7(param_1 + 0x88,0xc);
  *(undefined8 *)(param_2 + 0xf8) = uVar4;
  *(undefined8 *)(param_2 + 0x100) = 0;
  if (*(char *)(param_1 + 0x109) == '\0') {
    lVar5 = 0;
  }
  else {
    lVar5 = param_1 + 0x109;
  }
  FUN_00129591(param_2 + 0x20,lVar5,0x20);
  if (*(char *)(param_1 + 0x129) == '\0') {
    lVar5 = 0;
  }
  else {
    lVar5 = param_1 + 0x129;
  }
  FUN_00129591(param_2 + 0x28,lVar5,0x20);
  FUN_0011d4f4(param_2);
  uVar4 = DAT_0019f9e8;
  if ((local_28 == 2) && (DAT_0019f8be != '\0')) {
    uVar4 = FUN_001279d7(param_1 + 0x159,0xc);
    *(undefined8 *)(param_2 + 0xe8) = uVar4;
    uVar4 = FUN_001279d7(param_1 + 0x165,0xc);
    *(undefined8 *)(param_2 + 0x108) = uVar4;
    *(undefined8 *)(param_2 + 0x110) = 0;
    *(undefined8 *)(param_2 + 0xf0) = *(undefined8 *)(param_2 + 0x110);
  }
  else if (local_28 == 5) {
    uVar4 = FUN_001279d7(param_1 + 0x1dc,0xc);
    *(undefined8 *)(param_2 + 0xe8) = uVar4;
    uVar4 = FUN_001279d7(param_1 + 0x1e8,0xc);
    *(undefined8 *)(param_2 + 0x108) = uVar4;
    *(undefined8 *)(param_2 + 0x110) = 0;
    *(undefined8 *)(param_2 + 0xf0) = *(undefined8 *)(param_2 + 0x110);
  }
  else {
    *(undefined8 *)(param_2 + 0x108) = DAT_0019f9e0;
    *(undefined8 *)(param_2 + 0x110) = uVar4;
    *(undefined8 *)(param_2 + 0xe8) = *(undefined8 *)(param_2 + 0x108);
    *(undefined8 *)(param_2 + 0xf0) = *(undefined8 *)(param_2 + 0x110);
  }
  if (local_28 == 1) {
    uVar3 = FUN_00127a2b(param_1 + 0x6c,8);
    *(undefined4 *)(param_2 + 0x74) = uVar3;
    uVar3 = FUN_00127831(param_1 + 0x74,8);
    *(undefined4 *)(param_2 + 0x78) = uVar3;
    *(undefined8 *)(param_2 + 0x80) = 0;
  }
  else {
    if (param_4 != 0) {
      if (((DAT_0019f934 != '\0') || (*(char *)(param_1 + 0x109) == '\0')) ||
         (iVar2 = FUN_0012c173(param_1 + 0x109,param_2 + 0x74), iVar2 == 0)) {
        uVar3 = FUN_00127a2b(param_1 + 0x6c,8);
        *(undefined4 *)(param_2 + 0x74) = uVar3;
      }
      if (((DAT_0019f934 != '\0') || (*(char *)(param_1 + 0x129) == '\0')) ||
         (iVar2 = FUN_0012c261(param_1 + 0x129,param_2 + 0x78), iVar2 == 0)) {
        uVar3 = FUN_00127831(param_1 + 0x74,8);
        *(undefined4 *)(param_2 + 0x78) = uVar3;
      }
    }
    if ((int)*(char *)(param_1 + 0x9c) - 0x33U < 2) {
      __minor = FUN_001278c3(param_1 + 0x151,8);
      __major = FUN_00127879(param_1 + 0x149,8);
      uVar6 = gnu_dev_makedev(__major,__minor);
      *(ulonglong *)(param_2 + 0x80) = uVar6;
    }
    else {
      *(undefined8 *)(param_2 + 0x80) = 0;
    }
  }
  FUN_0011df5f(param_2);
  cVar1 = FUN_001301de(param_2);
  if (cVar1 == '\0') {
    *(undefined1 *)(param_2 + 0x120) = 0;
    if ((((DAT_0019f468 == 6) || (DAT_0019f468 == 2)) && (*(char *)(DAT_0019f460 + 0x9c) == 'D')) ||
       (*(long *)(param_2 + 400) != 0)) {
      *(undefined1 *)(param_2 + 0x188) = 1;
    }
  }
  else {
    FUN_00130241(param_2);
    *(undefined1 *)(param_2 + 0x120) = 1;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: from_header @ 0x26c90

undefined8
from_header(byte *param_1,size_t param_2,long param_3,long param_4,ulong param_5,char param_6,
            char param_7)

{
  byte *pbVar1;
  byte *pbVar2;
  int iVar3;
  int iVar4;
  long lVar5;
  long lVar6;
  size_t __n;
  void *pvVar7;
  undefined8 uVar8;
  ushort **ppuVar9;
  ulong uVar10;
  undefined8 uVar11;
  char *pcVar12;
  char *pcVar13;
  char *pcVar14;
  char *pcVar15;
  char *pcVar16;
  char *pcVar17;
  char *pcVar18;
  long in_FS_OFFSET;
  bool bVar19;
  char local_4d8 [4];
  char local_4d4;
  ulong local_4d0;
  long local_4c8;
  long local_4c0;
  size_t local_4b8;
  byte *local_4b0;
  bool local_4a6;
  bool local_4a5;
  uint local_4a4;
  uint local_4a0;
  int local_49c;
  ulong local_498;
  byte *local_490;
  byte *local_488;
  undefined1 *local_480;
  undefined1 *local_478;
  long local_470;
  ulong local_468;
  ulong local_460;
  void *local_458;
  byte *local_450;
  undefined1 local_448 [33];
  undefined1 auStack_427 [31];
  undefined1 local_408;
  undefined1 auStack_407 [999];
  long local_20;
  
  pcVar12 = local_4d8;
  pcVar18 = local_4d8;
  local_4d8[0] = param_7;
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_468 = -param_4;
  local_488 = param_1 + param_2;
  local_4a6 = false;
  local_4d4 = param_6;
  local_4d0 = param_5;
  local_4c8 = param_4;
  local_4c0 = param_3;
  local_4b8 = param_2;
  local_4b0 = param_1;
  local_470 = param_4;
  for (local_490 = param_1 + (*param_1 == 0); local_490 != local_488; local_490 = local_490 + 1) {
    ppuVar9 = __ctype_b_loc();
    pbVar2 = local_490;
    if (((*ppuVar9)[*local_490] & 0x2000) == 0) {
      local_498 = 0;
      if ((int)(char)*local_490 - 0x30U < 8) {
        local_450 = local_490;
        local_4a5 = false;
        while( true ) {
          pbVar1 = local_490 + 1;
          local_498 = local_498 + (long)((char)*local_490 + -0x30);
          local_490 = pbVar1;
          if ((pbVar1 == local_488) || (7 < (int)(char)*pbVar1 - 0x30U)) break;
          local_4a5 = local_498 != (local_498 & 0x1fffffffffffffff) || local_4a5 != false;
          local_498 = local_498 * 8;
        }
        if (((local_4a5 != false) || (local_4d0 < local_498)) &&
           (('1' < (char)*pbVar2 && (local_4c0 != 0)))) {
          local_4a4 = (int)(char)*pbVar2 - 0x30U | 4;
          bVar19 = false;
          local_498 = 0;
          local_490 = pbVar2;
          while( true ) {
            local_498 = local_498 + (long)(int)(7 - local_4a4);
            local_490 = local_490 + 1;
            if ((local_490 == local_488) || (7 < (int)(char)*local_490 - 0x30U)) break;
            local_4a4 = (int)(char)*local_490 - 0x30;
            bVar19 = local_498 != (local_498 & 0x1fffffffffffffff) || bVar19;
            local_498 = local_498 * 8;
          }
          local_498 = local_498 + 1;
          local_4a5 = local_498 == 0 || bVar19;
          if ((local_498 != 0 && !bVar19) && (local_498 <= local_468)) {
            if (local_4d8[0] != '\x01') {
              if (DAT_0019fdc8 != (code *)0x0) {
                (*DAT_0019fdc8)();
              }
              iVar4 = (int)local_450;
              iVar3 = (int)local_490;
              uVar8 = gettext(
                             "Archive octal value %.*s is out of %s range; assuming two\'s complement"
                             );
              error(0,0,uVar8,iVar3 - iVar4,local_450,local_4c0);
            }
            local_4a6 = true;
          }
        }
        if (local_4a5 != false) {
          if ((local_4c0 != 0) && (local_4d8[0] != '\x01')) {
            if (DAT_0019fdc8 != (code *)0x0) {
              (*DAT_0019fdc8)();
            }
            iVar4 = (int)local_450;
            iVar3 = (int)local_490;
            uVar8 = gettext("Archive octal value %.*s is out of %s range");
            error(0,0,uVar8,iVar3 - iVar4,local_450,local_4c0);
            DAT_0019fdd0 = 2;
          }
          uVar8 = 0xffffffffffffffff;
          pcVar18 = local_4d8;
          goto LAB_00127817;
        }
        goto LAB_0012752e;
      }
      if (local_4d4 != '\0') goto LAB_0012752e;
      if ((*local_490 != 0x2d) && (*local_490 != 0x2b)) {
        if ((*local_490 != 0x80) && (*local_490 != 0xff)) goto LAB_0012752e;
        local_4a0 = (int)(char)*local_490 & 0x40;
        local_460 = (long)(int)-local_4a0 << 0x32;
        local_498 = (ulong)(int)(((int)(char)*local_490 & 0x3fU) - local_4a0);
        local_490 = local_490 + 1;
        goto LAB_00127432;
      }
      if ((local_4d8[0] != '\x01') && (DAT_0019f5a1 != '\x01')) {
        DAT_0019f5a1 = '\x01';
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar8 = gettext("Archive contains obsolescent base-64 headers");
        error(0,0,uVar8);
      }
      pbVar2 = local_490;
      local_490 = local_490 + 1;
      local_4a6 = *pbVar2 == 0x2d;
      goto LAB_00127374;
    }
  }
  if ((local_4c0 != 0) && (local_4d8[0] != '\x01')) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar8 = gettext("Blanks in header where numeric %s value expected");
    error(0,0,uVar8,local_4c0);
    DAT_0019fdd0 = 2;
  }
  uVar8 = 0xffffffffffffffff;
  pcVar18 = local_4d8;
  goto LAB_00127817;
LAB_00127374:
  if (local_490 == local_488) goto LAB_0012752e;
  local_49c = (int)(char)(&DAT_0019f4a0)[(int)(uint)*local_490];
  if (0x3f < local_49c) goto LAB_0012752e;
  if (local_498 != (local_498 & 0x3ffffffffffffff)) {
    uVar10 = ((local_4b8 + 0x18) / 0x10) * 0x10;
    for (; pcVar12 != local_4d8 + -(uVar10 & 0xfffffffffffff000); pcVar12 = pcVar12 + -0x1000) {
      *(undefined8 *)(pcVar12 + -8) = *(undefined8 *)(pcVar12 + -8);
    }
    lVar5 = -(ulong)((uint)uVar10 & 0xfff);
    pcVar18 = pcVar12 + lVar5;
    if ((uVar10 & 0xfff) != 0) {
      *(undefined8 *)(pcVar12 + ((ulong)((uint)uVar10 & 0xfff) - 8) + lVar5) =
           *(undefined8 *)(pcVar12 + ((ulong)((uint)uVar10 & 0xfff) - 8) + lVar5);
    }
    pbVar2 = local_4b0;
    __n = local_4b8;
    pcVar13 = pcVar12 + lVar5 + -8;
    local_458 = (void *)((ulong)(pcVar12 + lVar5 + 0xf) & 0xfffffffffffffff0);
    pcVar13[0] = -0x50;
    pcVar13[1] = 'r';
    pcVar13[2] = '\x12';
    pcVar13[3] = '\0';
    pcVar13[4] = '\0';
    pcVar13[5] = '\0';
    pcVar13[6] = '\0';
    pcVar13[7] = '\0';
    memcpy((void *)((ulong)(pcVar12 + lVar5 + 0xf) & 0xfffffffffffffff0),pbVar2,__n);
    *(undefined1 *)(local_4b8 + (long)local_458) = 0;
    if ((local_4c0 != 0) && (local_4d8[0] != '\x01')) {
      if (DAT_0019fdc8 != (code *)0x0) {
        pcVar14 = pcVar12 + lVar5 + -8;
        pcVar14[0] = -0xf;
        pcVar14[1] = 'r';
        pcVar14[2] = '\x12';
        pcVar14[3] = '\0';
        pcVar14[4] = '\0';
        pcVar14[5] = '\0';
        pcVar14[6] = '\0';
        pcVar14[7] = '\0';
        (*DAT_0019fdc8)();
      }
      pvVar7 = local_458;
      pcVar15 = pcVar12 + lVar5 + -8;
      pcVar15[0] = '\0';
      pcVar15[1] = 's';
      pcVar15[2] = '\x12';
      pcVar15[3] = '\0';
      pcVar15[4] = '\0';
      pcVar15[5] = '\0';
      pcVar15[6] = '\0';
      pcVar15[7] = '\0';
      uVar8 = FUN_0015e84e(pvVar7);
      pcVar16 = pcVar12 + lVar5 + -8;
      builtin_strncpy(pcVar12 + lVar5 + -8,"\x12s\x12",4);
      pcVar16[4] = '\0';
      pcVar16[5] = '\0';
      pcVar16[6] = '\0';
      pcVar16[7] = '\0';
      uVar11 = gettext("Archive signed base-64 string %s is out of %s range");
      lVar6 = local_4c0;
      pcVar17 = pcVar12 + lVar5 + -8;
      builtin_strncpy(pcVar12 + lVar5 + -8,"6s\x12",4);
      pcVar17[4] = '\0';
      pcVar17[5] = '\0';
      pcVar17[6] = '\0';
      pcVar17[7] = '\0';
      error(0,0,uVar11,uVar8,lVar6);
      DAT_0019fdd0 = 2;
    }
    uVar8 = 0xffffffffffffffff;
    goto LAB_00127817;
  }
  local_498 = (long)local_49c | local_498 << 6;
  local_490 = local_490 + 1;
  goto LAB_00127374;
LAB_0012752e:
  if (((local_490 == local_488) || (*local_490 == 0)) ||
     (ppuVar9 = __ctype_b_loc(), ((*ppuVar9)[*local_490] & 0x2000) != 0)) {
    uVar10 = local_4d0;
    if (local_4a6 != false) {
      uVar10 = local_468;
    }
    if (uVar10 < local_498) {
      if ((local_4c0 != 0) && (local_4d8[0] != '\x01')) {
        local_480 = (undefined1 *)FUN_00154e5a(local_468,auStack_427);
        local_478 = (undefined1 *)FUN_00154e5a(local_498,auStack_407);
        if (local_4a6 != false) {
          local_478 = local_478 + -1;
          *local_478 = 0x2d;
        }
        if (local_468 != 0) {
          local_480 = local_480 + -1;
          *local_480 = 0x2d;
        }
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar8 = FUN_00154e5a(local_4d0,local_448);
        uVar11 = gettext("Archive value %s is out of %s range %s..%s");
        error(0,0,uVar11,local_478,local_4c0,local_480,uVar8);
        DAT_0019fdd0 = 2;
      }
      uVar8 = 0xffffffffffffffff;
      pcVar18 = local_4d8;
    }
    else {
      uVar10 = local_498;
      if (local_4a6 != false) {
        uVar10 = -local_498;
      }
      uVar8 = FUN_001294ad(uVar10);
      pcVar18 = local_4d8;
    }
  }
  else {
    if (local_4c0 != 0) {
      if (DAT_0019f5a8 == 0) {
        DAT_0019f5a8 = FUN_0015c92f(0);
        FUN_0015c9a2(DAT_0019f5a8,8);
      }
      for (; (local_4b0 != local_488 && (local_488[-1] == 0)); local_488 = local_488 + -1) {
      }
      FUN_0015dd44(&local_408,1000,local_4b0,(long)local_488 - (long)local_4b0,DAT_0019f5a8);
      if (local_4d8[0] != '\x01') {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar8 = gettext("Archive contains %.*s where numeric %s value expected");
        error(0,0,uVar8,1000,&local_408,local_4c0);
        DAT_0019fdd0 = 2;
      }
    }
    uVar8 = 0xffffffffffffffff;
    pcVar18 = local_4d8;
  }
  goto LAB_00127817;
  while (local_498 == (local_498 & 0xffffffffffffff | local_460)) {
LAB_00127432:
    pbVar2 = local_490 + 1;
    local_498 = (ulong)*local_490 + local_498 * 0x100;
    local_490 = pbVar2;
    if (pbVar2 == local_488) {
      local_4a6 = local_4a0 != 0;
      if (local_4a6) {
        local_498 = -local_498;
      }
      goto LAB_0012752e;
    }
  }
  if ((local_4c0 != 0) && (local_4d8[0] != '\x01')) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar8 = gettext("Archive base-256 value is out of %s range");
    error(0,0,uVar8,local_4c0);
    DAT_0019fdd0 = 2;
  }
  uVar8 = 0xffffffffffffffff;
LAB_00127817:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar8;
  }
                    /* WARNING: Subroutine does not return */
  *(undefined8 *)(pcVar18 + -8) = 0x12782b;
  __stack_chk_fail();
}




// Function: gid_from_header @ 0x27831

void gid_from_header(undefined8 param_1,undefined8 param_2)

{
  FUN_00126c90(param_1,param_2,"gid_t",0,0xffffffff,0,0);
  return;
}




// Function: major_from_header @ 0x27879

void major_from_header(undefined8 param_1,undefined8 param_2)

{
  FUN_00126c90(param_1,param_2,"major_t",0xffffffff80000000,0x7fffffff,0,0);
  return;
}




// Function: minor_from_header @ 0x278c3

void minor_from_header(undefined8 param_1,undefined8 param_2)

{
  FUN_00126c90(param_1,param_2,"minor_t",0xffffffff80000000,0x7fffffff,0,0);
  return;
}




// Function: mode_from_header @ 0x2790d

uint mode_from_header(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  ulong uVar1;
  
  uVar1 = FUN_00126c90(param_1,param_2,"mode_t",0x8000000000000000,0xffffffffffffffff,0,0);
  *(bool *)param_3 = (uVar1 & 0xfffffffffffff000) != 0;
  return (uint)uVar1 & 0xfff;
}




// Function: off_from_header @ 0x2798b

void off_from_header(undefined8 param_1,undefined8 param_2)

{
  FUN_00126c90(param_1,param_2,"off_t",0,0x7fffffffffffffff,0,0);
  return;
}




// Function: time_from_header @ 0x279d7

void time_from_header(undefined8 param_1,undefined8 param_2)

{
  FUN_00126c90(param_1,param_2,"time_t",0x8000000000000000,0x7fffffffffffffff,0,0);
  return;
}




// Function: uid_from_header @ 0x27a2b

void uid_from_header(undefined8 param_1,undefined8 param_2)

{
  FUN_00126c90(param_1,param_2,"uid_t",0,0xffffffff,0,0);
  return;
}




// Function: uintmax_from_header @ 0x27a73

void uintmax_from_header(undefined8 param_1,undefined8 param_2)

{
  FUN_00126c90(param_1,param_2,"uintmax_t",0,0xffffffffffffffff,0,0);
  return;
}




// Function: tartime @ 0x27abc

undefined * tartime(long param_1,int param_2,char param_3)

{
  size_t sVar1;
  long lVar2;
  undefined *puVar3;
  long in_FS_OFFSET;
  int local_3c;
  long local_38;
  undefined *local_30;
  tm *local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_3c = param_2;
  local_38 = param_1;
  if ((param_1 < 0) && (param_2 != 0)) {
    local_38 = param_1 + 1;
    local_3c = 1000000000 - param_2;
  }
  if (DAT_0019f879 == '\0') {
    local_28 = localtime(&local_38);
  }
  else {
    local_28 = gmtime(&local_38);
  }
  if (local_28 == (tm *)0x0) {
    lVar2 = local_38;
    if (param_1 < 0) {
      lVar2 = -local_38;
    }
    local_30 = (undefined *)FUN_00154e5a(lVar2,&DAT_0019f5c6);
    if (param_1 < 0) {
      local_30 = local_30 + -1;
      *local_30 = 0x2d;
    }
    while( true ) {
      if (param_3 == '\0') {
        puVar3 = &DAT_0019f5d4;
      }
      else {
        puVar3 = &DAT_0019f5e1;
      }
      if (local_30 <= puVar3) break;
      local_30 = local_30 + -1;
      *local_30 = 0x20;
    }
    puVar3 = local_30;
    if (param_3 != '\0') {
      FUN_00129e23(local_3c,&DAT_0019f5da);
      puVar3 = local_30;
    }
  }
  else {
    if (param_3 == '\0') {
      strftime(&DAT_0019f5c0,0x25,"%Y-%m-%d %H:%M",local_28);
    }
    else {
      strftime(&DAT_0019f5c0,0x25,"%Y-%m-%d %H:%M:%S",local_28);
      sVar1 = strlen(&DAT_0019f5c0);
      FUN_00129e23(local_3c,&DAT_0019f5c0 + sVar1);
    }
    puVar3 = &DAT_0019f5c0;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return puVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: simple_print_header @ 0x27c80

void simple_print_header(long *param_1,long param_2,long param_3)

{
  FILE *__stream;
  uint uVar1;
  int iVar2;
  undefined8 uVar3;
  char *pcVar4;
  size_t sVar5;
  char *pcVar6;
  size_t sVar7;
  size_t sVar8;
  long in_FS_OFFSET;
  long local_110;
  long local_e8;
  char *local_e0;
  char *local_d8;
  undefined1 local_c6;
  undefined1 local_c5;
  undefined1 local_c4;
  undefined1 auStack_c3 [9];
  undefined1 auStack_ba [2];
  undefined1 local_b8 [32];
  undefined1 local_98 [32];
  undefined1 local_78 [32];
  char local_58 [56];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0019fc30 == '\0') {
    if (*param_1 == 0) {
      local_e8 = param_1[1];
    }
    else {
      local_e8 = *param_1;
    }
  }
  else if (param_1[1] == 0) {
    local_e8 = *param_1;
  }
  else {
    local_e8 = param_1[1];
  }
  if (DAT_0019f88c != '\0') {
    local_110 = param_3;
    if (param_3 < 0) {
      local_110 = FUN_0010bacd();
    }
    uVar3 = FUN_00154e5a((local_110 - DAT_0019f480) - DAT_0019f488,local_58);
    pcVar4 = (char *)gettext("block %s: ");
    fprintf(DAT_0019efd0,pcVar4,uVar3);
  }
  __stream = DAT_0019efd0;
  if (DAT_0019f9ac < 2) {
    pcVar4 = (char *)FUN_0015e2eb(local_e8);
    fputs_unlocked(pcVar4,__stream);
    if ((DAT_0019fc30 != '\0') && ((char)param_1[2] != '\0')) {
      fputc_unlocked(0x2f,DAT_0019efd0);
    }
    fputc_unlocked(10,DAT_0019efd0);
  }
  else {
    local_c4 = 0x3f;
    switch(*(undefined1 *)(param_2 + 0x9c)) {
    case 0:
    case 0x30:
    case 0x53:
      if ((char)param_1[2] == '\0') {
        local_c4 = 0x2d;
      }
      else {
        local_c4 = 100;
      }
      break;
    case 0x31:
      local_c4 = 0x68;
      break;
    case 0x32:
      local_c4 = 0x6c;
      break;
    case 0x33:
      local_c4 = 99;
      break;
    case 0x34:
      local_c4 = 0x62;
      break;
    case 0x35:
      local_c4 = 100;
      break;
    case 0x36:
      local_c4 = 0x70;
      break;
    case 0x37:
      local_c4 = 0x43;
      break;
    case 0x44:
      local_c4 = 100;
      break;
    case 0x4b:
    case 0x4c:
      local_c4 = 0x4c;
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar3 = gettext("Unexpected long name header");
      error(0,0,uVar3);
      DAT_0019fdd0 = 2;
      break;
    case 0x4d:
      local_c4 = 0x4d;
      break;
    case 0x56:
      DAT_0019f5a0 = 1;
      local_c4 = 0x56;
    }
    FUN_0013dd6f((int)param_1[0xe],auStack_c3);
    FUN_0013d9ec(param_1,auStack_ba);
    pcVar4 = (char *)FUN_00127abc(param_1[0x1f],param_1[0x20],DAT_0019f87a);
    sVar5 = strlen(pcVar4);
    if ((int)DAT_0019c264 < (int)(uint)sVar5) {
      DAT_0019c264 = (uint)sVar5;
    }
    if ((((param_1[4] == 0) || (*(char *)param_1[4] == '\0')) || (DAT_0019f468 == 1)) ||
       (DAT_0019f934 == '\x01')) {
      local_e0 = (char *)FUN_00154e5a(*(undefined4 *)((long)param_1 + 0x74),local_b8);
    }
    else {
      local_e0 = (char *)param_1[4];
    }
    if (((param_1[5] == 0) || (*(char *)param_1[5] == '\0')) ||
       ((DAT_0019f468 == 1 || (DAT_0019f934 == '\x01')))) {
      local_d8 = (char *)FUN_00154e5a((int)param_1[0xf],local_98);
    }
    else {
      local_d8 = (char *)param_1[5];
    }
    if ((int)*(char *)(param_2 + 0x9c) - 0x33U < 2) {
      uVar1 = gnu_dev_major(param_1[0x10]);
      pcVar6 = (char *)FUN_00154e5a(uVar1,local_78);
      strcpy(local_58,pcVar6);
      strcat(local_58,",");
      uVar1 = gnu_dev_minor(param_1[0x10]);
      pcVar6 = (char *)FUN_00154e5a(uVar1,local_78);
      strcat(local_58,pcVar6);
    }
    else {
      pcVar6 = (char *)FUN_00154e5a(param_1[0x11],local_78);
      strcpy(local_58,pcVar6);
    }
    sVar5 = strlen(local_58);
    sVar7 = strlen(local_e0);
    sVar8 = strlen(local_d8);
    iVar2 = (int)sVar5 + (int)sVar7 + (int)sVar8 + 2;
    if (DAT_0019c260 < iVar2) {
      DAT_0019c260 = iVar2;
    }
    fprintf(DAT_0019efd0,"%s %s/%s %*s %-*s",&local_c4,local_e0,local_d8,
            (ulong)(uint)((DAT_0019c260 - iVar2) + (int)sVar5),local_58,(ulong)DAT_0019c264,pcVar4);
    uVar3 = FUN_0015e2eb(local_e8);
    fprintf(DAT_0019efd0," %s",uVar3);
    if ((DAT_0019fc30 != '\0') && ((char)param_1[2] != '\0')) {
      fputc_unlocked(0x2f,DAT_0019efd0);
    }
    switch(*(undefined1 *)(param_2 + 0x9c)) {
    case 0:
    case 0x30:
    case 0x33:
    case 0x34:
    case 0x35:
    case 0x36:
    case 0x37:
    case 0x44:
    case 0x53:
      putc_unlocked(10,DAT_0019efd0);
      break;
    default:
      local_c6 = *(undefined1 *)(param_2 + 0x9c);
      local_c5 = 0;
      uVar3 = FUN_0015e84e(&local_c6);
      pcVar4 = (char *)gettext(" unknown file type %s\n");
      fprintf(DAT_0019efd0,pcVar4,uVar3);
      break;
    case 0x31:
      uVar3 = FUN_0015e2eb(param_1[3]);
      pcVar4 = (char *)gettext(" link to %s\n");
      fprintf(DAT_0019efd0,pcVar4,uVar3);
      break;
    case 0x32:
      uVar3 = FUN_0015e2eb(param_1[3]);
      fprintf(DAT_0019efd0," -> %s\n",uVar3);
      break;
    case 0x4b:
      pcVar4 = (char *)gettext("--Long Link--\n");
      fprintf(DAT_0019efd0,pcVar4);
      break;
    case 0x4c:
      pcVar4 = (char *)gettext("--Long Name--\n");
      fprintf(DAT_0019efd0,pcVar4);
      break;
    case 0x4d:
      uVar3 = FUN_00127a73(param_2 + 0x171,0xc);
      pcVar4 = (char *)FUN_00154e5a(uVar3,local_78);
      strcpy(local_58,pcVar4);
      pcVar4 = (char *)gettext("--Continued at byte %s--\n");
      fprintf(DAT_0019efd0,pcVar4,local_58);
      break;
    case 0x56:
      pcVar4 = (char *)gettext("--Volume Header--\n");
      fprintf(DAT_0019efd0,pcVar4);
    }
  }
  fflush_unlocked(DAT_0019efd0);
  FUN_0013db33(param_1);
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: print_volume_label @ 0x28528

void print_volume_label(void)

{
  long in_FS_OFFSET;
  undefined1 local_3dc [4];
  undefined1 local_3d8 [8];
  undefined1 auStack_3d0 [440];
  undefined1 local_218 [136];
  undefined1 auStack_190 [20];
  undefined1 local_17c;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  memset(local_218,0,0x200);
  local_17c = 0x56;
  if (DAT_0019f490 != 0) {
    memcpy(auStack_190,(void *)(DAT_0019f490 + 0x88),0xc);
  }
  FUN_00139c55(local_3d8);
  FUN_00129547(auStack_3d0,&DAT_0018289c);
  FUN_00126747(local_218,local_3d8,local_3dc,0);
  FUN_00129547(auStack_3d0,DAT_0019efe0);
  FUN_00127c80(local_3d8,local_218,0);
  FUN_00139d1c(local_3d8);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: print_header @ 0x2864c

void print_header(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  if (((DAT_0019f468 == 4) && (DAT_0019f5a0 != '\x01')) && (DAT_0019efe0 != 0)) {
    FUN_00128528();
    DAT_0019f5a0 = '\x01';
  }
  FUN_00127c80(param_1,param_2,param_3);
  return;
}




// Function: print_for_mkdir @ 0x286af

void print_for_mkdir(undefined8 param_1,undefined8 param_2,undefined4 param_3)

{
  undefined8 uVar1;
  char *__format;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined1 local_43;
  undefined1 auStack_42 [10];
  undefined1 local_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (1 < DAT_0019f9ac) {
    local_43 = 100;
    FUN_0013dd6f(param_3,auStack_42);
    if (DAT_0019f88c != '\0') {
      uVar1 = FUN_0010bacd();
      uVar1 = FUN_00154e5a(uVar1,local_38);
      __format = (char *)gettext("block %s: ");
      fprintf(DAT_0019efd0,__format,uVar1);
    }
    uVar1 = FUN_0015e2eb(param_1);
    uVar2 = gettext("Creating directory:");
    fprintf(DAT_0019efd0,"%s %*s %s\n",&local_43,(ulong)(uint)(DAT_0019c260 + 1 + DAT_0019c264),
            uVar2,uVar1);
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: skip_file @ 0x287c1

void skip_file(long param_1)

{
  long lVar1;
  undefined8 uVar2;
  long local_20;
  
  local_20 = param_1;
  if (DAT_0019fc18 != '\0') {
    lVar1 = FUN_0010ce79(param_1);
    if (lVar1 < 0) {
      DAT_0019fc18 = '\0';
    }
    else {
      local_20 = param_1 + lVar1 * -0x200;
    }
  }
  FUN_0010ad86(local_20);
  while( true ) {
    if (local_20 < 1) {
      return;
    }
    lVar1 = FUN_0010bb4b();
    if (lVar1 == 0) break;
    FUN_0010bba7(lVar1);
    local_20 = local_20 + -0x200;
    FUN_0010ad86(local_20);
  }
  if (DAT_0019fdc8 != (code *)0x0) {
    (*DAT_0019fdc8)();
  }
  uVar2 = gettext("Unexpected EOF in archive");
  error(0,0,uVar2);
                    /* WARNING: Subroutine does not return */
  FUN_001177f4();
}




// Function: skip_member @ 0x28891

void skip_member(void)

{
  char cVar1;
  
  if (DAT_0019fba9 != '\x01') {
    cVar1 = *(char *)(DAT_0019f460 + 0x9c);
    FUN_0010bba7(DAT_0019f460);
    FUN_0010ad2b(&DAT_0019fa20);
    if (DAT_0019fb40 == '\0') {
      if (cVar1 != '5') {
        FUN_001287c1(DAT_0019faa8);
      }
    }
    else {
      FUN_001303bd(&DAT_0019fa20);
    }
    FUN_0010ad66();
  }
  return;
}




// Function: test_archive_label @ 0x28913

void test_archive_label(void)

{
  char cVar1;
  int iVar2;
  void *__ptr;
  
  FUN_001254c9();
  FUN_0012d3b1();
  FUN_0010f694(0);
  iVar2 = FUN_00125ecf(&DAT_0019f460,&DAT_0019fa20,0);
  if (iVar2 == 1) {
    FUN_00126747(DAT_0019f460,&DAT_0019fa20,&DAT_0019f468,0);
    if (*(char *)(DAT_0019f460 + 0x9c) == 'V') {
      FUN_00129591(&DAT_0019efe0,DAT_0019f460,100);
    }
    if (DAT_0019efe0 != 0) {
      if (DAT_0019f9ac != 0) {
        FUN_00128528();
      }
      cVar1 = FUN_0012d88a(DAT_0019efe0);
      if ((cVar1 != '\x01') && (DAT_0019f8fc != '\0')) {
        __ptr = (void *)FUN_0010e680(DAT_0019efe0);
        FUN_0012d88a(__ptr);
        free(__ptr);
      }
    }
  }
  FUN_0010d046();
  FUN_0012dd64();
  return;
}




// Function: map_hash @ 0x28a20

ulong map_hash(ulong *param_1,ulong param_2)

{
  return *param_1 % param_2;
}




// Function: map_compare @ 0x28a4d

undefined8 map_compare(long *param_1,long *param_2)

{
  return CONCAT71((int7)((ulong)*param_2 >> 8),*param_1 == *param_2);
}




// Function: parse_id @ 0x28a83

undefined8
parse_id(ulong *param_1,char *param_2,undefined8 param_3,ulong param_4,undefined8 param_5,
            undefined4 param_6)

{
  int *piVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  char *local_20;
  ulong local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  piVar1 = __errno_location();
  *piVar1 = 0;
  local_18 = strtoumax(param_2,&local_20,10);
  if (*local_20 == '\0') {
    piVar1 = __errno_location();
    if (*piVar1 == 0) {
      if (param_4 < local_18) {
        uVar2 = gettext("%s:%u: %s out of range: %s");
        error(0,0,uVar2,param_5,param_6,param_3,param_2);
        uVar2 = 0xffffffff;
      }
      else {
        *param_1 = local_18;
        uVar2 = 0;
      }
      goto LAB_00128ba5;
    }
  }
  uVar2 = gettext("%s:%u: invalid %s: %s");
  error(0,0,uVar2,param_5,param_6,param_3,param_2);
  uVar2 = 0xffffffff;
LAB_00128ba5:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: map_read @ 0x28bbb

void map_read(long *param_1,char *param_2,code *param_3,undefined8 param_4,undefined8 param_5)

{
  char *pcVar1;
  bool bVar2;
  int iVar3;
  undefined8 uVar4;
  long lVar5;
  undefined8 uVar6;
  long in_FS_OFFSET;
  uint local_15c;
  int local_158;
  char *local_150;
  size_t local_148;
  long local_140;
  long local_138;
  long local_130;
  FILE *local_128;
  __ssize_t local_120;
  char *local_118;
  long *local_110;
  long local_108;
  long *local_100;
  undefined *local_c8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_150 = (char *)0x0;
  local_148 = 0;
  bVar2 = false;
  local_128 = fopen(param_2,"r");
  if (local_128 == (FILE *)0x0) {
    FUN_0013e3b4(param_2);
  }
  local_c8 = &DAT_001828f0;
  local_15c = 0x8e44;
  local_158 = 0;
LAB_00129135:
  while( true ) {
    while( true ) {
      while( true ) {
        do {
          local_120 = getline(&local_150,&local_148,local_128);
          if (local_120 < 1) {
            if ((local_15c & 8) != 0) {
              FUN_00146399(&local_108);
            }
            fclose(local_128);
            if (bVar2) {
              if (DAT_0019fdc8 != (code *)0x0) {
                (*DAT_0019fdc8)();
              }
              uVar6 = gettext("errors reading map file");
              error(0,0,uVar6);
                    /* WARNING: Subroutine does not return */
              FUN_001177f4();
            }
            if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
              __stack_chk_fail();
            }
            return;
          }
          local_130 = 0;
          local_158 = local_158 + 1;
          iVar3 = FUN_001461a6(local_150,&local_108,local_15c);
          if (iVar3 != 0) {
            if (DAT_0019fdc8 != (code *)0x0) {
              (*DAT_0019fdc8)();
            }
            uVar6 = FUN_0014647f(&local_108);
            uVar4 = gettext("%s:%u: cannot split line: %s");
            error(0,0,uVar4,param_2,local_158,uVar6);
                    /* WARNING: Subroutine does not return */
            FUN_001177f4();
          }
          local_15c = local_15c | 8;
        } while (local_108 == 0);
        if (local_108 == 2) break;
        uVar6 = gettext("%s:%u: malformed line");
        error(0,0,uVar6,param_2,local_158);
        bVar2 = true;
      }
      if (*(char *)*local_100 == '+') break;
      if (param_3 == (code *)0x0) goto LAB_00128e8a;
      local_140 = (*param_3)(*local_100);
      if (local_140 != -1) goto LAB_00128e8a;
      lVar5 = *local_100;
      uVar6 = gettext("%s:%u: can\'t obtain %s of %s");
      error(0,0,uVar6,param_2,local_158,param_4,lVar5);
      bVar2 = true;
    }
    iVar3 = FUN_00128a83(&local_140,*local_100 + 1,param_4,param_5,param_2,local_158);
    if (iVar3 == 0) break;
    bVar2 = true;
  }
LAB_00128e8a:
  local_118 = strchr((char *)local_100[1],0x3a);
  if (local_118 == (char *)0x0) {
    if (*(char *)local_100[1] == '+') {
      iVar3 = FUN_00128a83(&local_138,local_100[1],param_4,param_5,param_2,local_158);
      if (iVar3 != 0) {
        bVar2 = true;
        goto LAB_00129135;
      }
    }
    else {
      local_130 = local_100[1];
      local_138 = (*param_3)(local_100[1]);
      if (local_138 == -1) {
        lVar5 = local_100[1];
        uVar6 = gettext("%s:%u: can\'t obtain %s of %s");
        error(0,0,uVar6,param_2,local_158,param_4,lVar5);
        bVar2 = true;
        goto LAB_00129135;
      }
    }
  }
  else {
    if ((char *)local_100[1] < local_118) {
      local_130 = local_100[1];
    }
    pcVar1 = local_118 + 1;
    *local_118 = '\0';
    local_118 = pcVar1;
    iVar3 = FUN_00128a83(&local_138,pcVar1,param_4,param_5,param_2,local_158);
    if (iVar3 != 0) {
      bVar2 = true;
      goto LAB_00129135;
    }
  }
  local_110 = (long *)FUN_00160b5b(0x18);
  *local_110 = local_140;
  local_110[1] = local_138;
  if (local_130 == 0) {
    lVar5 = 0;
  }
  else {
    lVar5 = FUN_00160cc6(local_130);
  }
  local_110[2] = lVar5;
  if (*param_1 == 0) {
    lVar5 = FUN_00152c1c(0,0,FUN_00128a20,FUN_00128a4d,0);
    *param_1 = lVar5;
    if (*param_1 != 0) goto LAB_0012910e;
  }
  else {
LAB_0012910e:
    lVar5 = FUN_0015395d(*param_1,local_110);
    if (lVar5 != 0) goto LAB_00129135;
  }
  FUN_0011783c();
  goto LAB_00129135;
}




// Function: name_to_uid @ 0x291f9

ulong name_to_uid(char *param_1)

{
  passwd *ppVar1;
  ulong uVar2;
  
  ppVar1 = getpwnam(param_1);
  if (ppVar1 == (passwd *)0x0) {
    uVar2 = 0xffffffffffffffff;
  }
  else {
    uVar2 = (ulong)ppVar1->pw_uid;
  }
  return uVar2;
}




// Function: owner_map_read @ 0x29234

void owner_map_read(undefined8 param_1)

{
  FUN_00128bbb(&DAT_0019f5e8,param_1,FUN_001291f9,&DAT_0018295a,0xffffffff);
  return;
}




// Function: owner_map_translate @ 0x29274

bool owner_map_translate(uint param_1,int *param_2,long *param_3)

{
  long lVar1;
  long in_FS_OFFSET;
  bool bVar2;
  ulong local_28 [3];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0019f5e8 != 0) {
    local_28[0] = (ulong)param_1;
    lVar1 = FUN_00152567(DAT_0019f5e8,local_28);
    if (lVar1 != 0) {
      *param_2 = (int)*(undefined8 *)(lVar1 + 8);
      *param_3 = *(long *)(lVar1 + 0x10);
      bVar2 = false;
      goto LAB_0012933d;
    }
  }
  bVar2 = DAT_0019f948 != -1;
  if (bVar2) {
    *param_2 = DAT_0019f948;
  }
  bVar2 = !bVar2;
  if (DAT_0019f940 != 0) {
    *param_3 = DAT_0019f940;
    bVar2 = false;
  }
LAB_0012933d:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: name_to_gid @ 0x29353

ulong name_to_gid(char *param_1)

{
  group *pgVar1;
  ulong uVar2;
  
  pgVar1 = getgrnam(param_1);
  if (pgVar1 == (group *)0x0) {
    uVar2 = 0xffffffffffffffff;
  }
  else {
    uVar2 = (ulong)pgVar1->gr_gid;
  }
  return uVar2;
}




// Function: group_map_read @ 0x2938e

void group_map_read(undefined8 param_1)

{
  FUN_00128bbb(&DAT_0019f5f0,param_1,FUN_00129353,&DAT_0018295e,0xffffffff);
  return;
}




// Function: group_map_translate @ 0x293ce

bool group_map_translate(uint param_1,int *param_2,long *param_3)

{
  long lVar1;
  long in_FS_OFFSET;
  bool bVar2;
  ulong local_28 [3];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (DAT_0019f5f0 != 0) {
    local_28[0] = (ulong)param_1;
    lVar1 = FUN_00152567(DAT_0019f5f0,local_28);
    if (lVar1 != 0) {
      *param_2 = (int)*(undefined8 *)(lVar1 + 8);
      *param_3 = *(long *)(lVar1 + 0x10);
      bVar2 = false;
      goto LAB_00129497;
    }
  }
  bVar2 = DAT_0019f8b8 != -1;
  if (bVar2) {
    *param_2 = DAT_0019f8b8;
  }
  bVar2 = !bVar2;
  if (DAT_0019f8b0 != 0) {
    *param_3 = DAT_0019f8b0;
    bVar2 = false;
  }
LAB_00129497:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: quote_n_colon @ 0x29516

void quote_n_colon(undefined4 param_1,undefined8 param_2)

{
  undefined4 uVar1;
  
  uVar1 = FUN_0015c97e(0);
  FUN_0015e599(param_1,uVar1,param_2);
  return;
}




// Function: assign_string @ 0x29547

void assign_string(undefined8 *param_1,long param_2)

{
  undefined8 uVar1;
  
  free((void *)*param_1);
  if (param_2 == 0) {
    uVar1 = 0;
  }
  else {
    uVar1 = FUN_00160cc6(param_2);
  }
  *param_1 = uVar1;
  return;
}




// Function: assign_string_n @ 0x29591

void assign_string_n(undefined8 *param_1,char *param_2,size_t param_3)

{
  size_t __n;
  void *__dest;
  
  free((void *)*param_1);
  if (param_2 != (char *)0x0) {
    __n = strnlen(param_2,param_3);
    __dest = (void *)FUN_00160b5b(__n + 1);
    memcpy(__dest,param_2,__n);
    *(undefined1 *)(__n + (long)__dest) = 0;
    *param_1 = __dest;
  }
  return;
}




// Function: unquote_string @ 0x2961d

undefined4 unquote_string(char *param_1)

{
  char *pcVar1;
  char *pcVar2;
  char *pcVar3;
  char cVar4;
  undefined4 local_20;
  char *local_18;
  char *local_10;
  
  local_20 = 1;
  pcVar3 = param_1;
  pcVar1 = param_1;
  while (local_10 = pcVar1, local_18 = pcVar3, *local_18 != '\0') {
    if (*local_18 == '\\') {
      pcVar3 = local_18 + 1;
      switch(*pcVar3) {
      case '0':
      case '1':
      case '2':
      case '3':
      case '4':
      case '5':
      case '6':
      case '7':
        pcVar1 = local_18 + 2;
        if ((*pcVar1 < '0') || ('7' < *pcVar1)) {
          *local_10 = *pcVar3 + -0x30;
          pcVar3 = pcVar1;
          pcVar1 = local_10 + 1;
        }
        else {
          pcVar2 = local_18 + 3;
          cVar4 = *pcVar1 + (*pcVar3 + -0x30) * '\b' + -0x30;
          if ((*pcVar2 < '0') || ('7' < *pcVar2)) {
            *local_10 = cVar4;
            pcVar3 = pcVar2;
            pcVar1 = local_10 + 1;
          }
          else {
            *local_10 = *pcVar2 + cVar4 * '\b' + -0x30;
            pcVar3 = local_18 + 4;
            pcVar1 = local_10 + 1;
          }
        }
        break;
      default:
        local_20 = 0;
        *local_10 = '\\';
        pcVar1 = local_10 + 1;
        if (*pcVar3 != '\0') {
          local_10[1] = *pcVar3;
          pcVar3 = local_18 + 2;
          pcVar1 = local_10 + 2;
        }
        break;
      case '?':
        *local_10 = '\x7f';
        pcVar3 = local_18 + 2;
        pcVar1 = local_10 + 1;
        break;
      case '\\':
        *local_10 = '\\';
        pcVar3 = local_18 + 2;
        pcVar1 = local_10 + 1;
        break;
      case 'a':
        *local_10 = '\a';
        pcVar3 = local_18 + 2;
        pcVar1 = local_10 + 1;
        break;
      case 'b':
        *local_10 = '\b';
        pcVar3 = local_18 + 2;
        pcVar1 = local_10 + 1;
        break;
      case 'f':
        *local_10 = '\f';
        pcVar3 = local_18 + 2;
        pcVar1 = local_10 + 1;
        break;
      case 'n':
        *local_10 = '\n';
        pcVar3 = local_18 + 2;
        pcVar1 = local_10 + 1;
        break;
      case 'r':
        *local_10 = '\r';
        pcVar3 = local_18 + 2;
        pcVar1 = local_10 + 1;
        break;
      case 't':
        *local_10 = '\t';
        pcVar3 = local_18 + 2;
        pcVar1 = local_10 + 1;
        break;
      case 'v':
        *local_10 = '\v';
        pcVar3 = local_18 + 2;
        pcVar1 = local_10 + 1;
      }
    }
    else if (local_18 == local_10) {
      pcVar3 = local_18 + 1;
      pcVar1 = local_10 + 1;
    }
    else {
      *local_10 = *local_18;
      pcVar3 = local_18 + 1;
      pcVar1 = local_10 + 1;
    }
  }
  if (local_18 != local_10) {
    *local_10 = '\0';
  }
  return local_20;
}




// Function: zap_slashes @ 0x298d7

char * zap_slashes(char *param_1)

{
  size_t sVar1;
  char *local_10;
  
  if ((param_1 != (char *)0x0) && (*param_1 != '\0')) {
    sVar1 = strlen(param_1);
    local_10 = param_1 + (sVar1 - 1);
    while ((param_1 < local_10 && (*local_10 == '/'))) {
      *local_10 = '\0';
      local_10 = local_10 + -1;
    }
  }
  return param_1;
}




// Function: normalize_filename_x @ 0x29946

void normalize_filename_x(char *param_1)

{
  char *pcVar1;
  char *pcVar2;
  char cVar3;
  int iVar4;
  int iVar5;
  long lVar6;
  char *local_20;
  char *local_18;
  
  local_20 = param_1;
  local_18 = param_1;
  while ((*local_20 = *local_18, *local_20 == '.' && (local_18[1] == '/'))) {
    for (local_18 = local_18 + 2; *local_18 == '/'; local_18 = local_18 + 1) {
    }
    local_20 = local_20 + (*local_18 == '\0');
  }
  while( true ) {
    pcVar1 = local_18 + 1;
    cVar3 = *local_18;
    pcVar2 = local_20 + 1;
    *local_20 = cVar3;
    if (*local_20 == '\0') break;
    local_20 = pcVar2;
    local_18 = pcVar1;
    if (cVar3 == '/') {
      for (; local_18[*local_18 == '.'] == '/'; local_18 = local_18 + lVar6) {
        if (*local_18 == '.') {
          lVar6 = 2;
        }
        else {
          lVar6 = 1;
        }
      }
    }
  }
  if (2 < (long)pcVar2 - (long)param_1) {
    if ((local_20[-1] == '.') && (local_20[-2] == '/')) {
      iVar4 = 1;
    }
    else {
      iVar4 = 0;
    }
    if (((long)(pcVar2 + (-(long)param_1 - (long)iVar4)) < 3) || (pcVar2[-2 - (long)iVar4] != '/'))
    {
      iVar5 = 0;
    }
    else {
      iVar5 = 1;
    }
    pcVar2[(-(long)iVar5 - (long)iVar4) + -1] = '\0';
  }
  return;
}




// Function: normalize_filename @ 0x29aac

char * normalize_filename(undefined4 param_1,char *param_2)

{
  char *__s;
  size_t sVar1;
  size_t sVar2;
  char *local_30;
  
  local_30 = (char *)0x0;
  if (*param_2 != '/') {
    __s = (char *)FUN_0012afed(param_1);
    sVar1 = strlen(__s);
    sVar2 = strlen(param_2);
    local_30 = (char *)FUN_00160b5b(sVar2 + sVar1 + 2);
    strcpy(local_30,__s);
    local_30[sVar1] = '/';
    strcpy(local_30 + sVar1 + 1,param_2);
  }
  if (local_30 == (char *)0x0) {
    local_30 = (char *)FUN_00160cc6(param_2);
  }
  FUN_00129946(local_30);
  return local_30;
}




// Function: replace_prefix @ 0x29b93

void replace_prefix(undefined8 *param_1,void *param_2,ulong param_3,void *param_4,ulong param_5)

{
  int iVar1;
  size_t sVar2;
  char *local_18;
  
  local_18 = (char *)*param_1;
  sVar2 = strlen(local_18);
  if (param_3 < sVar2) {
    iVar1 = memcmp(local_18,param_2,param_3);
    if ((iVar1 == 0) && (local_18[param_3] == '/')) {
      if (param_3 < param_5) {
        local_18 = (char *)FUN_00160b8d(local_18,param_5 + (sVar2 - param_3) + 1);
        *param_1 = local_18;
      }
      memmove(local_18 + param_5,local_18 + param_3,(sVar2 - param_3) + 1);
      memcpy(local_18,param_4,param_5);
    }
  }
  return;
}




// Function: sysinttostr @ 0x29c96

void sysinttostr(ulong param_1,undefined8 param_2,ulong param_3,undefined8 param_4)

{
  if (param_3 < param_1) {
    FUN_00154bde(param_1,param_4);
  }
  else {
    FUN_00154e5a(param_1,param_4);
  }
  return;
}




// Function: strtosysint @ 0x29cfc

ulong strtosysint(char *param_1,char **param_2,ulong param_3,ulong param_4)

{
  int *piVar1;
  ulong uVar2;
  uintmax_t uVar3;
  
  piVar1 = __errno_location();
  *piVar1 = 0;
  if ((long)param_4 < 0) {
    if ((int)*param_1 - 0x30U < 10) {
      uVar3 = strtoumax(param_1,param_2,10);
      if (uVar3 <= param_4) {
        uVar2 = FUN_001294ad(uVar3);
        return uVar2;
      }
      piVar1 = __errno_location();
      *piVar1 = 0x22;
      return param_4;
    }
  }
  else if ((int)param_1[*param_1 == '-'] - 0x30U < 10) {
    uVar2 = strtoimax(param_1,param_2,10);
    if (((long)param_3 <= (long)uVar2) && ((long)uVar2 <= (long)param_4)) {
      return uVar2;
    }
    piVar1 = __errno_location();
    *piVar1 = 0x22;
    if ((long)param_3 <= (long)uVar2) {
      return param_4;
    }
    return param_3;
  }
  piVar1 = __errno_location();
  *piVar1 = 0x16;
  return 0;
}




// Function: code_ns_fraction @ 0x29e23

void code_ns_fraction(int param_1,undefined1 *param_2)

{
  int local_1c;
  int local_c;
  
  if (param_1 == 0) {
    *param_2 = 0;
  }
  else {
    local_c = 9;
    *param_2 = 0x2e;
    for (local_1c = param_1; local_1c % 10 == 0; local_1c = local_1c / 10) {
      local_c = local_c + -1;
    }
    param_2[(long)local_c + 1] = 0;
    while( true ) {
      local_c = local_c + -1;
      param_2[(long)local_c + 1] = (char)local_1c + (char)(local_1c / 10) * -10 + '0';
      if (local_c == 0) break;
      local_1c = local_1c / 10;
    }
  }
  return;
}




// Function: code_timespec @ 0x29f2e

undefined1 * code_timespec(long param_1,int param_2,long param_3)

{
  int local_2c;
  long local_28;
  undefined1 *local_20;
  
  if ((999999999 < param_2) || (local_2c = param_2, param_2 < 0)) {
    local_2c = 0;
  }
  local_28 = param_1;
  if ((param_1 < 0) && (local_2c != 0)) {
    local_28 = param_1 + 1;
    local_2c = 1000000000 - local_2c;
  }
  if (param_1 < 0) {
    local_28 = -local_28;
  }
  local_20 = (undefined1 *)FUN_00154e5a(local_28,param_3 + 1);
  if (param_1 < 0) {
    local_20 = local_20 + -1;
    *local_20 = 0x2d;
  }
  FUN_00129e23(local_2c,param_3 + 0x15);
  return local_20;
}




// Function: decode_timespec @ 0x29ff7

undefined1  [16] decode_timespec(char *param_1,char **param_2,char param_3)

{
  undefined1 auVar1 [16];
  int *piVar2;
  uintmax_t uVar3;
  bool bVar4;
  bool local_42;
  int local_40;
  int local_3c;
  uintmax_t local_38;
  char *local_30;
  
  local_38 = 0x8000000000000000;
  local_40 = -1;
  bVar4 = *param_1 == '-';
  if ((int)param_1[bVar4] - 0x30U < 10) {
    piVar2 = __errno_location();
    *piVar2 = 0;
    if (bVar4) {
      uVar3 = strtoimax(param_1,param_2,10);
    }
    else {
      uVar3 = strtoumax(param_1,param_2,10);
      if ((long)uVar3 < 0) {
        piVar2 = __errno_location();
        *piVar2 = 0x22;
        uVar3 = local_38;
      }
    }
    local_38 = uVar3;
    local_30 = *param_2;
    local_40 = 0;
    if ((param_3 != '\0') && (*local_30 == '.')) {
      local_3c = 0;
      local_42 = false;
      while (local_30 = local_30 + 1, (int)*local_30 - 0x30U < 10) {
        if (local_3c < 9) {
          local_3c = local_3c + 1;
          local_40 = *local_30 + -0x30 + local_40 * 10;
        }
        else {
          local_42 = *local_30 != '0' || local_42 != false;
        }
      }
      for (; local_3c < 9; local_3c = local_3c + 1) {
        local_40 = local_40 * 10;
      }
      if ((bVar4) && (local_40 = local_40 + (uint)local_42, local_40 != 0)) {
        if (local_38 == 0x8000000000000000) {
          local_40 = -1;
        }
        else {
          local_38 = local_38 - 1;
          local_40 = 1000000000 - local_40;
        }
      }
    }
    piVar2 = __errno_location();
    if (*piVar2 == 0x22) {
      local_40 = -1;
    }
  }
  else {
    piVar2 = __errno_location();
    *piVar2 = 0x16;
    local_30 = param_1;
  }
  *param_2 = local_30;
  auVar1._8_8_ = (long)local_40;
  auVar1._0_8_ = local_38;
  return auVar1;
}




// Function: must_be_dot_or_slash @ 0x2a1fe

bool must_be_dot_or_slash(char *param_1)

{
  long lVar1;
  bool bVar2;
  char *local_10;
  
  local_10 = param_1;
  if (*param_1 == '/') {
    while( true ) {
      for (; local_10[1] == '/'; local_10 = local_10 + 1) {
      }
      if (local_10[1] != '.') break;
      if (local_10[2] == '.') {
        lVar1 = 3;
      }
      else {
        lVar1 = 2;
      }
      if (local_10[lVar1] != '/') break;
      if (local_10[2] == '.') {
        lVar1 = 3;
      }
      else {
        lVar1 = 2;
      }
      local_10 = local_10 + lVar1;
    }
    bVar2 = local_10[1] == '\0';
  }
  else {
    while ((*local_10 == '.' && (local_10[1] == '/'))) {
      for (local_10 = local_10 + 2; *local_10 == '/'; local_10 = local_10 + 1) {
      }
    }
    if ((*local_10 == '\0') || ((*local_10 == '.' && (local_10[1] == '\0')))) {
      bVar2 = true;
    }
    else {
      bVar2 = false;
    }
  }
  return bVar2;
}




// Function: safer_rmdir @ 0x2a306

undefined8 safer_rmdir(char *param_1)

{
  char cVar1;
  int iVar2;
  int *piVar3;
  undefined8 uVar4;
  
  cVar1 = FUN_0012a1fe(param_1);
  if (cVar1 == '\0') {
    iVar2 = unlinkat(DAT_0019c268,param_1,0x200);
    if (iVar2 == 0) {
      FUN_00119454(param_1);
      uVar4 = 0;
    }
    else {
      uVar4 = 0xffffffff;
    }
  }
  else {
    piVar3 = __errno_location();
    *piVar3 = 0;
    uVar4 = 0xffffffff;
  }
  return uVar4;
}




// Function: remove_any_file @ 0x2a36f

/* WARNING: Removing unreachable block (ram,0x0012a42b) */
/* WARNING: Removing unreachable block (ram,0x0012a448) */

ulong remove_any_file(char *param_1,uint param_2)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  char *__ptr;
  void *__ptr_00;
  size_t sVar4;
  char *local_28;
  
  iVar1 = unlinkat(DAT_0019c268,param_1,0);
  if (iVar1 == 0) {
    return 1;
  }
  piVar3 = __errno_location();
  if ((*piVar3 != 1) && (piVar3 = __errno_location(), *piVar3 != 0x15)) {
    return 0;
  }
  iVar1 = FUN_0012a306(param_1);
  if (iVar1 == 0) {
    return 1;
  }
  piVar3 = __errno_location();
  iVar1 = *piVar3;
  if (iVar1 != 0x27) {
    if (0x27 < iVar1) {
      return 0;
    }
    if (iVar1 == 0x14) {
      return 0;
    }
    if (0x14 < iVar1) {
      return 0;
    }
    if ((iVar1 != 0) && (iVar1 != 0x11)) {
      return 0;
    }
  }
  if (param_2 == 2) {
    return 0xffffffff;
  }
  if (((param_2 < 3) && (param_2 != 0)) && (param_2 == 1)) {
    __ptr = (char *)FUN_0012b8e1(param_1,0);
    local_28 = __ptr;
    if (__ptr == (char *)0x0) {
      return 0;
    }
    while( true ) {
      sVar4 = strlen(local_28);
      if (sVar4 == 0) {
        free(__ptr);
        iVar1 = FUN_0012a306(param_1);
        return (ulong)(iVar1 == 0);
      }
      __ptr_00 = (void *)FUN_0012ee85(param_1,local_28);
      iVar2 = remove_any_file(__ptr_00,1);
      piVar3 = __errno_location();
      iVar1 = *piVar3;
      free(__ptr_00);
      if (iVar2 == 0) break;
      local_28 = local_28 + sVar4 + 1;
    }
    free(__ptr);
    piVar3 = __errno_location();
    *piVar3 = iVar1;
    return 0;
  }
  return 0;
}




// Function: maybe_backup_file @ 0x2a571

undefined8 maybe_backup_file(char *param_1,char param_2)

{
  int iVar1;
  void *pvVar2;
  undefined8 uVar3;
  int *piVar4;
  undefined8 uVar5;
  char *__format;
  undefined8 uVar6;
  long in_FS_OFFSET;
  undefined1 local_b8 [24];
  uint local_a0;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_00129547(&DAT_0019f608,param_1);
  FUN_00129547(&DAT_0019f610,0);
  if ((param_2 != '\0') && (DAT_0019fdf8 != '\x01')) {
    DAT_0019fdf0 = strchr(param_1,0x3a);
    if ((DAT_0019fdf0 != (char *)0x0) && (param_1 < DAT_0019fdf0)) {
      pvVar2 = memchr(param_1,0x2f,(long)DAT_0019fdf0 - (long)param_1);
      if (pvVar2 == (void *)0x0) {
        uVar3 = 1;
        goto LAB_0012a854;
      }
    }
  }
  iVar1 = FUN_0012a9b8(param_1,local_b8);
  if (iVar1 == 0) {
    if ((local_a0 & 0xf000) == 0x4000) {
      uVar3 = 1;
    }
    else if ((param_2 == '\0') ||
            (((local_a0 & 0xf000) != 0x6000 && ((local_a0 & 0xf000) != 0x2000)))) {
      DAT_0019f610 = (char *)FUN_0014de0e(DAT_0019c268,param_1,DAT_0019f888);
      if (DAT_0019f610 == (char *)0x0) {
        FUN_0011783c();
      }
      iVar1 = renameat(DAT_0019c268,DAT_0019f608,DAT_0019c268,DAT_0019f610);
      if (iVar1 == 0) {
        if (DAT_0019f9ac != 0) {
          uVar3 = FUN_0015e821(1,DAT_0019f610);
          uVar5 = FUN_0015e821(0,DAT_0019f608);
          __format = (char *)gettext("Renaming %s to %s\n");
          fprintf(DAT_0019efd0,__format,uVar5,uVar3);
        }
        uVar3 = 1;
      }
      else {
        piVar4 = __errno_location();
        iVar1 = *piVar4;
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar3 = FUN_0015e821(1,DAT_0019f610);
        uVar5 = FUN_0015e548(DAT_0019f608);
        uVar6 = gettext("%s: Cannot rename to %s");
        error(0,iVar1,uVar6,uVar5,uVar3);
        DAT_0019fdd0 = 2;
        FUN_00129547(&DAT_0019f610,0);
        uVar3 = 0;
      }
    }
    else {
      uVar3 = 1;
    }
  }
  else {
    piVar4 = __errno_location();
    if (*piVar4 == 2) {
      uVar3 = 1;
    }
    else {
      FUN_0013ea03(param_1);
      uVar3 = 0;
    }
  }
LAB_0012a854:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar3;
}




// Function: undo_last_backup @ 0x2a874

void undo_last_backup(void)

{
  int iVar1;
  int *piVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  char *__format;
  
  if (DAT_0019f610 != (char *)0x0) {
    iVar1 = renameat(DAT_0019c268,DAT_0019f610,DAT_0019c268,DAT_0019f608);
    if (iVar1 != 0) {
      piVar2 = __errno_location();
      iVar1 = *piVar2;
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar3 = FUN_0015e821(1,DAT_0019f608);
      uVar4 = FUN_0015e548(DAT_0019f610);
      uVar5 = gettext("%s: Cannot rename to %s");
      error(0,iVar1,uVar5,uVar4,uVar3);
      DAT_0019fdd0 = 2;
    }
    if (DAT_0019f9ac != 0) {
      uVar3 = FUN_0015e821(1,DAT_0019f608);
      uVar4 = FUN_0015e821(0,DAT_0019f610);
      __format = (char *)gettext("Renaming %s back to %s\n");
      fprintf(DAT_0019efd0,__format,uVar4,uVar3);
    }
    FUN_00129547(&DAT_0019f610,0);
  }
  return;
}




// Function: deref_stat @ 0x2a9b8

void deref_stat(char *param_1,stat *param_2)

{
  fstatat(DAT_0019c268,param_1,param_2,DAT_0019fc10);
  return;
}




// Function: blocking_read @ 0x2a9e9

long blocking_read(undefined4 param_1,undefined8 param_2,undefined8 param_3)

{
  uint uVar1;
  int iVar2;
  int *piVar3;
  undefined8 local_10;
  
  local_10 = FUN_0015ec70(param_1,param_2,param_3);
  if (local_10 == -1) {
    piVar3 = __errno_location();
    if (*piVar3 == 0xb) {
      uVar1 = FUN_001617eb(param_1,3);
      if ((-1 < (int)uVar1) && ((uVar1 & 0x800) != 0)) {
        iVar2 = FUN_001617eb(param_1,4,uVar1 & 0xfffff7ff);
        if (iVar2 != -1) {
          local_10 = FUN_0015ec70(param_1,param_2,param_3);
        }
      }
    }
  }
  return local_10;
}




// Function: blocking_write @ 0x2aa95

ulong blocking_write(undefined4 param_1,long param_2,ulong param_3)

{
  uint uVar1;
  int iVar2;
  int *piVar3;
  long lVar4;
  undefined8 local_18;
  
  local_18 = FUN_00152175(param_1,param_2,param_3);
  if (local_18 < param_3) {
    piVar3 = __errno_location();
    if (*piVar3 == 0xb) {
      uVar1 = FUN_001617eb(param_1,3);
      if ((-1 < (int)uVar1) && ((uVar1 & 0x800) != 0)) {
        iVar2 = FUN_001617eb(param_1,4,uVar1 & 0xfffff7ff);
        if (iVar2 != -1) {
          lVar4 = FUN_00152175(param_1,param_2 + local_18,param_3 - local_18);
          local_18 = local_18 + lVar4;
        }
      }
    }
  }
  return local_18;
}




// Function: set_file_atime @ 0x2ab5e

void set_file_atime(undefined4 param_1,undefined4 param_2,undefined8 param_3,undefined8 param_4,
                 undefined8 param_5)

{
  long in_FS_OFFSET;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_20 = 0x3ffffffe;
  local_38 = param_4;
  local_30 = param_5;
  FUN_0014f4ec(param_1,param_2,param_3,&local_38,DAT_0019fc10);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: chdir_count @ 0x2abe0

int chdir_count(void)

{
  int iVar1;
  
  if (DAT_0019f620 == 0) {
    iVar1 = 0;
  }
  else {
    iVar1 = (int)DAT_0019f620 + -1;
  }
  return iVar1;
}




// Function: chdir_arg @ 0x2ac09

ulong chdir_arg(char *param_1)

{
  ulong uVar1;
  char *local_10;
  
  if (DAT_0019f620 == DAT_0019f628) {
    if (DAT_0019f628 == 0) {
      DAT_0019f628 = 2;
    }
    DAT_0019f618 = (undefined8 *)FUN_00160a5f(DAT_0019f618,&DAT_0019f628,0x18);
    if (DAT_0019f620 == 0) {
      *DAT_0019f618 = &DAT_00182ac7;
      DAT_0019f618[DAT_0019f620 * 3 + 1] = 0;
      *(undefined4 *)(DAT_0019f618 + DAT_0019f620 * 3 + 2) = 0xffffff9c;
      DAT_0019f620 = DAT_0019f620 + 1;
    }
  }
  local_10 = param_1;
  if (*param_1 != '\0') {
    while ((*local_10 == '.' && (local_10[1] == '/'))) {
      for (local_10 = local_10 + 2; *local_10 == '/'; local_10 = local_10 + 1) {
      }
    }
    if (local_10[*local_10 == '.'] == '\0') {
      return (ulong)((int)DAT_0019f620 - 1);
    }
  }
  DAT_0019f618[DAT_0019f620 * 3] = local_10;
  DAT_0019f618[DAT_0019f620 * 3 + 1] = 0;
  *(undefined4 *)(DAT_0019f618 + DAT_0019f620 * 3 + 2) = 0;
  uVar1 = DAT_0019f620;
  DAT_0019f620 = DAT_0019f620 + 1;
  return uVar1;
}




// Function: chdir_do @ 0x2adf1

void chdir_do(int param_1)

{
  int *piVar1;
  int iVar2;
  undefined8 *puVar3;
  int local_2c;
  int local_28;
  long local_20;
  
  if (param_1 != DAT_0019f600) {
    puVar3 = (undefined8 *)((long)param_1 * 0x18 + DAT_0019f618);
    local_2c = *(int *)(puVar3 + 2);
    if (local_2c == 0) {
      if (*(char *)*puVar3 != '/') {
        chdir_do(param_1 + -1);
      }
      local_2c = openat(DAT_0019c268,(char *)*puVar3,DAT_0019fc0c & 0xfffdffff);
      if (local_2c < 0) {
        FUN_0013e3b4(*puVar3);
      }
      *(int *)(puVar3 + 2) = local_2c;
      if (DAT_0019f680 < 0x10) {
        piVar1 = &DAT_0019f640 + DAT_0019f680;
        DAT_0019f680 = DAT_0019f680 + 1;
        *piVar1 = param_1;
      }
      else {
        puVar3 = (undefined8 *)((long)DAT_0019f67c * 0x18 + DAT_0019f618);
        iVar2 = close(*(int *)(puVar3 + 2));
        if (iVar2 != 0) {
          FUN_0012b26a(*puVar3);
        }
        *(undefined4 *)(puVar3 + 2) = 0;
        DAT_0019f67c = param_1;
      }
    }
    if (0 < local_2c) {
      local_28 = DAT_0019f640;
      local_20 = 1;
      while (DAT_0019f640 = param_1, local_28 != param_1) {
        iVar2 = (&DAT_0019f640)[local_20];
        (&DAT_0019f640)[local_20] = local_28;
        if (iVar2 == param_1) break;
        local_20 = local_20 + 1;
        local_28 = iVar2;
      }
    }
    DAT_0019c268 = local_2c;
    DAT_0019f600 = param_1;
  }
  return;
}




// Function: tar_dirname @ 0x2afc0

undefined8 tar_dirname(void)

{
  return *(undefined8 *)((long)DAT_0019f600 * 0x18 + DAT_0019f618);
}




// Function: tar_getcdpath @ 0x2afed

long tar_getcdpath(int param_1)

{
  undefined4 uVar1;
  long lVar2;
  undefined8 uVar3;
  int local_28;
  
  uVar1 = DAT_0019f600;
  if (DAT_0019f618 == 0) {
    lVar2 = DAT_0019f688;
    if ((DAT_0019f688 == 0) && (lVar2 = FUN_00160cf7(), DAT_0019f688 = lVar2, lVar2 == 0)) {
                    /* WARNING: Subroutine does not return */
      FUN_0013dfc0("getcwd",&DAT_00182ac7);
    }
  }
  else {
    local_28 = param_1;
    if (*(long *)((long)param_1 * 0x18 + DAT_0019f618 + 8) == 0) {
      while ((-1 < local_28 && (*(long *)((long)local_28 * 0x18 + DAT_0019f618 + 8) == 0))) {
        local_28 = local_28 + -1;
      }
      while (local_28 = local_28 + 1, local_28 <= param_1) {
        FUN_0012adf1(local_28);
        lVar2 = DAT_0019f618;
        if (local_28 == 0) {
          uVar3 = FUN_00160cf7();
          *(undefined8 *)(lVar2 + 8) = uVar3;
          if (*(long *)(lVar2 + 8) == 0) {
                    /* WARNING: Subroutine does not return */
            FUN_0013dfc0("getcwd",&DAT_00182ac7);
          }
        }
        else if (**(char **)((long)local_28 * 0x18 + DAT_0019f618) == '/') {
          lVar2 = DAT_0019f618 + (long)local_28 * 0x18;
          uVar3 = FUN_00160cc6(*(undefined8 *)((long)local_28 * 0x18 + DAT_0019f618));
          *(undefined8 *)(lVar2 + 8) = uVar3;
        }
        else {
          uVar3 = FUN_0012b66c(*(undefined8 *)((long)local_28 * 0x18 + DAT_0019f618 + -0x10));
          FUN_0012b7ec(uVar3,*(undefined8 *)((long)local_28 * 0x18 + DAT_0019f618));
          lVar2 = DAT_0019f618 + (long)local_28 * 0x18;
          uVar3 = FUN_0012b882(uVar3);
          *(undefined8 *)(lVar2 + 8) = uVar3;
        }
      }
      FUN_0012adf1(uVar1);
    }
    lVar2 = *(long *)((long)param_1 * 0x18 + DAT_0019f618 + 8);
  }
  return lVar2;
}




// Function: close_diag @ 0x2b26a

void close_diag(undefined8 param_1)

{
  if (DAT_0019f8bc == '\0') {
    FUN_0013e1ff(param_1);
  }
  else if ((DAT_0019ec84 & 0x800000) != 0) {
    FUN_0013e228(param_1);
  }
  return;
}




// Function: open_diag @ 0x2b2b1

void open_diag(undefined8 param_1)

{
  if (DAT_0019f8bc == '\0') {
    FUN_0013e38b(param_1);
  }
  else if ((DAT_0019ec84 & 0x800000) != 0) {
    FUN_0013e3da(param_1);
  }
  return;
}




// Function: read_diag_details @ 0x2b2f8

void read_diag_details(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  if (DAT_0019f8bc == '\0') {
    FUN_0013e42c(param_1,param_2,param_3);
  }
  else if ((DAT_0019ec84 & 0x800000) != 0) {
    FUN_0013e506(param_1,param_2,param_3);
  }
  return;
}




// Function: readlink_diag @ 0x2b35d

void readlink_diag(undefined8 param_1)

{
  if (DAT_0019f8bc == '\0') {
    FUN_0013e6b3(param_1);
  }
  else if ((DAT_0019ec84 & 0x800000) != 0) {
    FUN_0013e6dc(param_1);
  }
  return;
}




// Function: savedir_diag @ 0x2b3a4

void savedir_diag(undefined8 param_1)

{
  if (DAT_0019f8bc == '\0') {
    FUN_0013e72e(param_1);
  }
  else if ((DAT_0019ec84 & 0x800000) != 0) {
    FUN_0013e757(param_1);
  }
  return;
}




// Function: seek_diag_details @ 0x2b3eb

void seek_diag_details(undefined8 param_1,undefined8 param_2)

{
  if (DAT_0019f8bc == '\0') {
    FUN_0013e7a9(param_1,param_2);
  }
  else if ((DAT_0019ec84 & 0x800000) != 0) {
    FUN_0013e890(param_1,param_2);
  }
  return;
}




// Function: stat_diag @ 0x2b444

void stat_diag(undefined8 param_1)

{
  if (DAT_0019f8bc == '\0') {
    FUN_0013ea03(param_1);
  }
  else if ((DAT_0019ec84 & 0x800000) != 0) {
    FUN_0013ea2c(param_1);
  }
  return;
}




// Function: file_removed_diag @ 0x2b48b

void file_removed_diag(undefined8 param_1,char param_2,code *param_3)

{
  int *piVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  
  if ((param_2 != '\x01') && (piVar1 = __errno_location(), *piVar1 == 2)) {
    if ((DAT_0019ec84 & 0x40) != 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar2 = FUN_0015e548(param_1);
      uVar3 = gettext("%s: File removed before we read it");
      error(0,0,uVar3,uVar2);
    }
    FUN_00139e98(1);
    return;
  }
  (*param_3)(param_1);
  return;
}




// Function: xfork @ 0x2b537

__pid_t xfork(void)

{
  __pid_t _Var1;
  undefined8 uVar2;
  
  _Var1 = fork();
  if (_Var1 == -1) {
    uVar2 = gettext("child process");
                    /* WARNING: Subroutine does not return */
    FUN_0013dfc0(&DAT_00182b01,uVar2);
  }
  return _Var1;
}




// Function: xpipe @ 0x2b577

void xpipe(int *param_1)

{
  int iVar1;
  undefined8 uVar2;
  
  iVar1 = pipe(param_1);
  if (iVar1 < 0) {
    uVar2 = gettext("interprocess channel");
                    /* WARNING: Subroutine does not return */
    FUN_0013dfc0(&DAT_00182b1b,uVar2);
  }
  return;
}




// Function: ptr_align @ 0x2b5bb

long ptr_align(long param_1,ulong param_2)

{
  ulong uVar1;
  
  uVar1 = param_1 + (param_2 - 1);
  return uVar1 - uVar1 % param_2;
}




// Function: page_aligned_alloc @ 0x2b601

void page_aligned_alloc(undefined8 *param_1,ulong param_2)

{
  int iVar1;
  ulong uVar2;
  undefined8 uVar3;
  
  iVar1 = getpagesize();
  uVar2 = (long)iVar1 + param_2;
  if (uVar2 < param_2) {
    FUN_0011783c();
  }
  uVar3 = FUN_00160b5b(uVar2);
  *param_1 = uVar3;
  FUN_0012b5bb(*param_1,(long)iVar1);
  return;
}




// Function: namebuf_create @ 0x2b66c

long * namebuf_create(char *param_1)

{
  long *plVar1;
  size_t sVar2;
  long lVar3;
  
  plVar1 = (long *)FUN_00160b5b(0x18);
  sVar2 = strlen(param_1);
  plVar1[1] = sVar2 + 2;
  lVar3 = FUN_00160b5b(plVar1[1]);
  *plVar1 = lVar3;
  strcpy((char *)*plVar1,param_1);
  sVar2 = strlen((char *)*plVar1);
  plVar1[2] = sVar2;
  if (*(char *)(plVar1[2] + -1 + *plVar1) != '/') {
    lVar3 = plVar1[2];
    plVar1[2] = lVar3 + 1;
    *(undefined1 *)(lVar3 + *plVar1) = 0x2f;
  }
  return plVar1;
}




// Function: namebuf_free @ 0x2b72d

void namebuf_free(undefined8 *param_1)

{
  free((void *)*param_1);
  free(param_1);
  return;
}




// Function: namebuf_name @ 0x2b75b

long namebuf_name(long *param_1,char *param_2)

{
  size_t sVar1;
  long lVar2;
  
  sVar1 = strlen(param_2);
  while ((ulong)param_1[1] <= sVar1 + param_1[2] + 1) {
    lVar2 = FUN_00160bd8(*param_1,param_1 + 1);
    *param_1 = lVar2;
  }
  strcpy((char *)(*param_1 + param_1[2]),param_2);
  return *param_1;
}




// Function: namebuf_add_dir @ 0x2b7ec

void namebuf_add_dir(long *param_1,char *param_2)

{
  long lVar1;
  size_t sVar2;
  
  if (*(char *)(param_1[2] + -1 + *param_1) != '/') {
    FUN_0012b75b(param_1,&DAT_0019c26c);
    param_1[2] = param_1[2] + 1;
  }
  FUN_0012b75b(param_1,param_2);
  lVar1 = param_1[2];
  sVar2 = strlen(param_2);
  param_1[2] = lVar1 + sVar2;
  return;
}




// Function: namebuf_finish @ 0x2b882

long namebuf_finish(long *param_1)

{
  long lVar1;
  
  lVar1 = *param_1;
  if (*(char *)(param_1[2] + -1 + *param_1) == '/') {
    *(undefined1 *)(param_1[2] + *param_1) = 0;
  }
  free(param_1);
  return lVar1;
}




// Function: tar_savedir @ 0x2b8e1

long tar_savedir(char *param_1,int param_2)

{
  int iVar1;
  int *piVar2;
  bool bVar3;
  long lVar4;
  DIR *local_10;
  
  lVar4 = 0;
  local_10 = (DIR *)0x0;
  iVar1 = openat(DAT_0019c268,param_1,DAT_0019fc08 | 0x10000);
  if (iVar1 < 0) {
    if ((param_2 == 0) && (piVar2 = __errno_location(), *piVar2 == 2)) {
      return 0;
    }
    FUN_0013e38b(param_1);
  }
  else {
    local_10 = fdopendir(iVar1);
    if ((local_10 == (DIR *)0x0) || (lVar4 = FUN_0015eee2(local_10,DAT_0019fc2c), lVar4 == 0)) {
      FUN_0013e72e(param_1);
    }
  }
  if (local_10 == (DIR *)0x0) {
    if ((iVar1 < 0) || (iVar1 = close(iVar1), iVar1 == 0)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
  }
  else {
    iVar1 = closedir(local_10);
    bVar3 = iVar1 != 0;
  }
  if (bVar3) {
    FUN_0013e72e(param_1);
  }
  return lVar4;
}




// Function: file_selection_option @ 0x2b9f2

long * file_selection_option(int param_1)

{
  long *local_10;
  
  for (local_10 = &DAT_0019c280; ((*local_10 != 0 || ((int)local_10[1] != 0)) || (local_10[4] != 0))
      ; local_10 = local_10 + 6) {
    if (param_1 == (int)local_10[1]) {
      return local_10;
    }
  }
  return (long *)0x0;
}




// Function: file_selection_option_name @ 0x2ba4c

undefined8 file_selection_option_name(undefined4 param_1)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  
  puVar1 = (undefined8 *)FUN_0012b9f2(param_1);
  if (puVar1 == (undefined8 *)0x0) {
    uVar2 = 0;
  }
  else {
    uVar2 = *puVar1;
  }
  return uVar2;
}




// Function: is_file_selection_option @ 0x2ba80

bool is_file_selection_option(undefined4 param_1)

{
  long lVar1;
  
  lVar1 = FUN_0012b9f2(param_1);
  return lVar1 != 0;
}




// Function: names_parse_opt @ 0x2baa1

undefined8 names_parse_opt(int param_1,undefined8 param_2,long param_3)

{
  long *plVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  char cVar4;
  undefined8 uVar5;
  
  if (param_1 == 0x1000005) {
    plVar1 = *(long **)(param_3 + 0x28);
    if (*(int *)*plVar1 == 2) {
      uVar2 = *(undefined8 *)(*plVar1 + 0x10);
      uVar3 = *(undefined8 *)(*plVar1 + 8);
      uVar5 = gettext("%s:%lu: unrecognized option");
      error(0,0,uVar5,uVar3,uVar2);
      FUN_00139e98(2);
    }
    return 7;
  }
  if (param_1 < 0x1000006) {
    if (param_1 == 0x100) {
      FUN_0012c829(param_2);
      return 0;
    }
    if (param_1 < 0x101) {
      if (param_1 == 0x43) {
        FUN_0012c8c4(param_2);
        return 0;
      }
      if (param_1 == 0x54) {
        FUN_0012c8f7(param_2);
        return 0;
      }
    }
  }
  cVar4 = FUN_0012ba80(param_1);
  if (cVar4 != '\0') {
    FUN_0012c884(param_1,param_2);
    return 0;
  }
  return 7;
}




// Function: add_exclude_array @ 0x2bbce

void add_exclude_array(long param_1,undefined4 param_2)

{
  undefined4 local_c;
  
  local_c = 0;
  while (*(long *)(param_1 + (long)local_c * 8) != 0) {
    FUN_0014ee5d(DAT_0019f8a8,*(undefined8 *)(param_1 + (long)local_c * 8),param_2);
    local_c = local_c + 1;
  }
  return;
}




// Function: handle_file_selection_option @ 0x2bc3a

void handle_file_selection_option(undefined4 param_1,undefined8 param_2)

{
  int iVar1;
  int *piVar2;
  undefined8 uVar3;
  uint uVar4;
  
  switch(param_1) {
  case 0x58:
    if (DAT_0019f6bc == 1) {
      uVar4 = 0;
    }
    else {
      uVar4 = 0x10000000;
    }
    iVar1 = FUN_0014f407(FUN_0014ee5d,DAT_0019f8a8,param_2,uVar4 | DAT_0019f6c0 | DAT_0019f930,10);
    if (iVar1 != 0) {
      piVar2 = __errno_location();
      iVar1 = *piVar2;
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar3 = FUN_0015e548(param_2);
      error(0,iVar1,&DAT_0018345c,uVar3);
                    /* WARNING: Subroutine does not return */
      FUN_001177f4();
    }
    break;
  default:
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    error(0,0,"unhandled positional option %d",param_1);
                    /* WARNING: Subroutine does not return */
    FUN_001177f4();
  case 0x101:
    FUN_0012bbce(&PTR_DAT_0019a7a0,0x10000000);
    break;
  case 0x102:
    FUN_00111ea1("CACHEDIR.TAG",1,FUN_00112087);
    break;
  case 0x103:
    FUN_00111ea1("CACHEDIR.TAG",2,FUN_00112087);
    break;
  case 0x104:
    FUN_00111ea1("CACHEDIR.TAG",3,FUN_00112087);
    break;
  case 0x105:
    if (DAT_0019f6bc == 1) {
      uVar4 = 0;
    }
    else {
      uVar4 = 0x10000000;
    }
    FUN_0014ee5d(DAT_0019f8a8,param_2,uVar4 | DAT_0019f6c0 | DAT_0019f930);
    break;
  case 0x106:
    FUN_00117879(param_2,2);
    break;
  case 0x107:
    FUN_00117879(param_2,1);
    break;
  case 0x108:
    FUN_00111ea1(param_2,1,0);
    break;
  case 0x109:
    FUN_00111ea1(param_2,2,0);
    break;
  case 0x10a:
    FUN_00111ea1(param_2,3,0);
    break;
  case 0x10b:
    FUN_0012bbce(&PTR_DAT_0019a6e0,0);
    break;
  case 0x10c:
    FUN_00118293();
    break;
  case 0x10d:
    DAT_0019f6c0 = DAT_0019f6c0 | 0x10;
    break;
  case 0x10e:
    DAT_0019f6c0 = DAT_0019f6c0 & 0xffffffef;
    break;
  case 0x10f:
    DAT_0019f6c0 = DAT_0019f6c0 | 0x40000000;
    break;
  case 0x110:
    DAT_0019c274 = 0;
    DAT_0019f6c0 = DAT_0019f6c0 & 0xbfffffff;
    break;
  case 0x111:
    DAT_0019f930 = 8;
    break;
  case 0x112:
    DAT_0019f930 = 0;
    break;
  case 0x113:
    DAT_0019fc28 = 1;
    break;
  case 0x114:
    DAT_0019fc28 = 0;
    break;
  case 0x115:
    DAT_0019f6b8 = 0;
    break;
  case 0x116:
    DAT_0019f6c0 = DAT_0019f6c0 | 1;
    break;
  case 0x117:
    DAT_0019f6bc = 1;
    break;
  case 0x118:
    DAT_0019c270 = 0;
    DAT_0019f6b8 = 1;
    break;
  case 0x119:
    DAT_0019c270 = 10;
    DAT_0019f6b8 = 0;
    break;
  case 0x11a:
    DAT_0019f6b8 = 1;
    break;
  case 0x11b:
    DAT_0019f6c0 = DAT_0019f6c0 & 0xfffffffe;
    break;
  case 0x11c:
    DAT_0019f6bc = 2;
  }
  return;
}




// Function: uid_to_uname @ 0x2bfd1

void uid_to_uname(__uid_t param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  passwd *ppVar2;
  
  if ((param_1 == 0) || (param_1 != DAT_0019f6f0)) {
    if ((DAT_0019f6c8 == 0) || (param_1 != DAT_0019f6d8)) {
      ppVar2 = getpwuid(param_1);
      if (ppVar2 == (passwd *)0x0) {
        DAT_0019f6f0 = param_1;
        uVar1 = FUN_00160cc6(&DAT_00183794);
        *param_2 = uVar1;
        return;
      }
      DAT_0019f6d8 = param_1;
      FUN_00129547(&DAT_0019f6c8,ppVar2->pw_name);
    }
    uVar1 = FUN_00160cc6(DAT_0019f6c8);
    *param_2 = uVar1;
  }
  else {
    uVar1 = FUN_00160cc6(&DAT_00183794);
    *param_2 = uVar1;
  }
  return;
}




// Function: gid_to_gname @ 0x2c0a2

void gid_to_gname(__gid_t param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  group *pgVar2;
  
  if ((param_1 == 0) || (param_1 != DAT_0019f6f4)) {
    if ((DAT_0019f6d0 == 0) || (param_1 != DAT_0019f6dc)) {
      pgVar2 = getgrgid(param_1);
      if (pgVar2 == (group *)0x0) {
        DAT_0019f6f4 = param_1;
        uVar1 = FUN_00160cc6(&DAT_00183794);
        *param_2 = uVar1;
        return;
      }
      DAT_0019f6dc = param_1;
      FUN_00129547(&DAT_0019f6d0,pgVar2->gr_name);
    }
    uVar1 = FUN_00160cc6(DAT_0019f6d0);
    *param_2 = uVar1;
  }
  else {
    uVar1 = FUN_00160cc6(&DAT_00183794);
    *param_2 = uVar1;
  }
  return;
}




// Function: uname_to_uid @ 0x2c173

undefined8 uname_to_uid(char *param_1,__uid_t *param_2)

{
  int iVar1;
  passwd *ppVar2;
  
  if ((DAT_0019f6e0 != (char *)0x0) && (iVar1 = strcmp(param_1,DAT_0019f6e0), iVar1 == 0)) {
    return 0;
  }
  if (((DAT_0019f6c8 == (char *)0x0) || (*param_1 != *DAT_0019f6c8)) ||
     (iVar1 = strcmp(param_1,DAT_0019f6c8), iVar1 != 0)) {
    ppVar2 = getpwnam(param_1);
    if (ppVar2 == (passwd *)0x0) {
      FUN_00129547(&DAT_0019f6e0,param_1);
      return 0;
    }
    DAT_0019f6d8 = ppVar2->pw_uid;
    FUN_00129547(&DAT_0019f6c8,ppVar2->pw_name);
  }
  *param_2 = DAT_0019f6d8;
  return 1;
}




// Function: gname_to_gid @ 0x2c261

undefined8 gname_to_gid(char *param_1,__gid_t *param_2)

{
  int iVar1;
  group *pgVar2;
  
  if ((DAT_0019f6e8 != (char *)0x0) && (iVar1 = strcmp(param_1,DAT_0019f6e8), iVar1 == 0)) {
    return 0;
  }
  if (((DAT_0019f6d0 == (char *)0x0) || (*param_1 != *DAT_0019f6d0)) ||
     (iVar1 = strcmp(param_1,DAT_0019f6d0), iVar1 != 0)) {
    pgVar2 = getgrnam(param_1);
    if (pgVar2 == (group *)0x0) {
      FUN_00129547(&DAT_0019f6e8,param_1);
      return 0;
    }
    DAT_0019f6dc = pgVar2->gr_gid;
    FUN_00129547(&DAT_0019f6d0,param_1);
  }
  *param_2 = DAT_0019f6dc;
  return 1;
}




// Function: make_name @ 0x2c34c

long make_name(undefined *param_1)

{
  long lVar1;
  undefined8 uVar2;
  size_t sVar3;
  undefined *local_20;
  
  lVar1 = FUN_00160c06(0x60);
  local_20 = param_1;
  if (param_1 == (undefined *)0x0) {
    local_20 = &DAT_00183794;
  }
  uVar2 = FUN_00160cc6(local_20);
  *(undefined8 *)(lVar1 + 0x10) = uVar2;
  sVar3 = strlen(*(char **)(lVar1 + 0x10));
  *(size_t *)(lVar1 + 0x18) = sVar3;
  return lVar1;
}




// Function: free_name @ 0x2c3b1

void free_name(void *param_1)

{
  if (param_1 != (void *)0x0) {
    free(*(void **)((long)param_1 + 0x10));
    free(*(void **)((long)param_1 + 0x58));
    free(param_1);
  }
  return;
}




// Function: name_elt_alloc @ 0x2c3f7

undefined8 * name_elt_alloc(void)

{
  undefined8 *local_10;
  
  local_10 = (undefined8 *)FUN_00160b5b(0x38);
  if (DAT_0019f708 == (undefined8 *)0x0) {
    DAT_0019f708 = local_10;
    *local_10 = 0;
    DAT_0019f708[1] = *local_10;
    *(undefined4 *)(DAT_0019f708 + 2) = 3;
    local_10 = (undefined8 *)FUN_00160b5b(0x38);
  }
  local_10[1] = DAT_0019f708[1];
  if (DAT_0019f708[1] != 0) {
    *(undefined8 **)DAT_0019f708[1] = local_10;
  }
  *local_10 = DAT_0019f708;
  DAT_0019f708[1] = local_10;
  return local_10;
}




// Function: name_list_adjust @ 0x2c4b8

void name_list_adjust(void)

{
  if (DAT_0019f708 != 0) {
    for (; *(long *)(DAT_0019f708 + 8) != 0; DAT_0019f708 = *(long *)(DAT_0019f708 + 8)) {
    }
  }
  return;
}




// Function: unconsumed_option_push @ 0x2c4f3

void unconsumed_option_push(undefined8 *param_1)

{
  *param_1 = 0;
  param_1[1] = DAT_0019f6a8;
  if (DAT_0019f6a8 != (undefined8 *)0x0) {
    *DAT_0019f6a8 = param_1;
  }
  DAT_0019f6a8 = param_1;
  return;
}




// Function: unconsumed_option_free @ 0x2c541

void unconsumed_option_free(void)

{
  void *__ptr;
  
  while (__ptr = DAT_0019f6a8, DAT_0019f6a8 != (void *)0x0) {
    DAT_0019f6a8 = *(void **)((long)DAT_0019f6a8 + 8);
    free(__ptr);
  }
  return;
}




// Function: unconsumed_option_report @ 0x2c588

void unconsumed_option_report(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 *local_20;
  
  if (DAT_0019f6a8 != (undefined8 *)0x0) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar1 = gettext(
                   "The following options were used after any non-optional arguments in archive create or update mode.  These options are positional and affect only arguments that follow them.  Please, rearrange them properly."
                   );
    error(0,0,uVar1);
    for (local_20 = DAT_0019f6a8; local_20[1] != 0; local_20 = (undefined8 *)local_20[1]) {
    }
    for (; DAT_0019fdd0 = 2, local_20 != (undefined8 *)0x0; local_20 = (undefined8 *)*local_20) {
      if (*(int *)(local_20 + 2) == 1) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar1 = FUN_0015e84e(local_20[3]);
        uVar2 = gettext("-C %s has no effect");
        error(0,0,uVar2,uVar1);
      }
      else if (*(int *)(local_20 + 2) == 4) {
        if (local_20[4] == 0) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar1 = FUN_0012ba4c(*(undefined4 *)(local_20 + 3));
          uVar2 = gettext("--%s has no effect");
          error(0,0,uVar2,uVar1);
        }
        else {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar1 = FUN_0015e84e(local_20[4]);
          uVar2 = FUN_0012ba4c(*(undefined4 *)(local_20 + 3));
          uVar3 = gettext("--%s %s has no effect");
          error(0,0,uVar3,uVar2,uVar1);
        }
      }
    }
    FUN_0012c541();
  }
  return;
}




// Function: name_list_advance @ 0x2c78a

void name_list_advance(void)

{
  long *__ptr;
  
  __ptr = DAT_0019f708;
  DAT_0019f708 = (long *)*DAT_0019f708;
  if (DAT_0019f708 != (long *)0x0) {
    *(undefined8 *)((long)DAT_0019f708 + 8) = 0;
  }
  if (((int)__ptr[2] == 4) || ((int)__ptr[2] == 1)) {
    if ((DAT_0019f860 == 3) || (DAT_0019f860 == 8)) {
      FUN_0012c4f3(__ptr);
    }
  }
  else {
    if ((int)__ptr[2] != 3) {
      FUN_0012c541();
    }
    free(__ptr);
  }
  return;
}




// Function: name_add_name @ 0x2c829

void name_add_name(undefined8 param_1)

{
  long lVar1;
  
  lVar1 = FUN_0012c3f7();
  *(undefined4 *)(lVar1 + 0x10) = 0;
  *(undefined8 *)(lVar1 + 0x18) = param_1;
  if (DAT_0019f6a0 == 0) {
    DAT_0019f6a0 = 1;
  }
  else if (DAT_0019f6a0 == 1) {
    DAT_0019f6a0 = 2;
  }
  return;
}




// Function: name_add_option @ 0x2c884

void name_add_option(undefined4 param_1,undefined8 param_2)

{
  long lVar1;
  
  lVar1 = FUN_0012c3f7();
  *(undefined4 *)(lVar1 + 0x10) = 4;
  *(undefined4 *)(lVar1 + 0x18) = param_1;
  *(undefined8 *)(lVar1 + 0x20) = param_2;
  return;
}




// Function: name_add_dir @ 0x2c8c4

void name_add_dir(undefined8 param_1)

{
  long lVar1;
  
  lVar1 = FUN_0012c3f7();
  *(undefined4 *)(lVar1 + 0x10) = 1;
  *(undefined8 *)(lVar1 + 0x18) = param_1;
  return;
}




// Function: name_add_file @ 0x2c8f7

void name_add_file(undefined8 param_1)

{
  long lVar1;
  
  lVar1 = FUN_0012c3f7();
  *(undefined4 *)(lVar1 + 0x10) = 2;
  *(undefined8 *)(lVar1 + 0x18) = param_1;
  *(undefined8 *)(lVar1 + 0x20) = 0;
  *(undefined8 *)(lVar1 + 0x30) = 0;
  DAT_0019f6a0 = 2;
  return;
}




// Function: name_init @ 0x2c94c

void name_init(void)

{
  DAT_0019f710 = FUN_00160b5b(0x66);
  DAT_0019f718 = 100;
  FUN_0012c4b8();
  return;
}




// Function: name_term @ 0x2c978

void name_term(void)

{
  free(DAT_0019f710);
  return;
}




// Function: file_list_name @ 0x2c992

undefined8 file_list_name(void)

{
  undefined8 uVar1;
  undefined8 *local_10;
  
  local_10 = DAT_0019f708;
  while( true ) {
    if (local_10 == (undefined8 *)0x0) {
      uVar1 = gettext("command line");
      return uVar1;
    }
    if ((*(int *)(local_10 + 2) == 2) && (local_10[6] != 0)) break;
    local_10 = (undefined8 *)*local_10;
  }
  return local_10[3];
}




// Function: add_file_id @ 0x2c9f1

undefined8 add_file_id(char *param_1)

{
  undefined8 uVar1;
  int iVar2;
  undefined4 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 *puVar7;
  long in_FS_OFFSET;
  undefined8 *local_c8;
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  iVar2 = stat(param_1,&local_b8);
  if (iVar2 != 0) {
    FUN_0013e9dd(param_1);
  }
  uVar4 = FUN_0012c992();
  local_c8 = DAT_0019f720;
  do {
    if (local_c8 == (undefined8 *)0x0) {
      puVar7 = (undefined8 *)FUN_00160b5b(0x20);
      *puVar7 = DAT_0019f720;
      puVar7[1] = local_b8.st_ino;
      puVar7[2] = local_b8.st_dev;
      puVar7[3] = uVar4;
      uVar4 = 0;
      DAT_0019f720 = puVar7;
LAB_0012cbd6:
      if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return uVar4;
    }
    if ((local_c8[1] == local_b8.st_ino) && (local_c8[2] == local_b8.st_dev)) {
      uVar3 = FUN_0015c9cd(0,0x3a,1);
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar1 = local_c8[3];
      uVar5 = FUN_0015e285(0,param_1);
      uVar6 = gettext("%s: file list requested from %s already read from %s");
      error(0,0,uVar6,uVar5,uVar4,uVar1);
      DAT_0019fdd0 = 2;
      FUN_0015c9cd(0,0x3a,uVar3);
      uVar4 = 1;
      goto LAB_0012cbd6;
    }
    local_c8 = (undefined8 *)*local_c8;
  } while( true );
}




// Function: chopslash @ 0x2cbf6

void chopslash(char *param_1)

{
  size_t sVar1;
  char *local_10;
  
  sVar1 = strlen(param_1);
  local_10 = param_1 + (sVar1 - 1);
  while ((param_1 < local_10 && (*local_10 == '/'))) {
    *local_10 = '\0';
    local_10 = local_10 + -1;
  }
  return;
}




// Function: read_name_from_file @ 0x2cc4a

undefined8 read_name_from_file(long param_1)

{
  int iVar1;
  FILE *__stream;
  undefined8 uVar2;
  int local_20;
  long local_18;
  
  local_18 = 0;
  __stream = *(FILE **)(param_1 + 0x30);
  iVar1 = *(int *)(param_1 + 0x28);
  *(long *)(param_1 + 0x20) = *(long *)(param_1 + 0x20) + 1;
  local_20 = getc_unlocked(__stream);
  while( true ) {
    if ((local_20 == -1) || (local_20 == iVar1)) {
      if ((local_18 == 0) && (local_20 != -1)) {
        uVar2 = 3;
      }
      else {
        if (local_18 == DAT_0019f718) {
          DAT_0019f710 = FUN_00160bd8(DAT_0019f710,&DAT_0019f718);
        }
        *(undefined1 *)(local_18 + DAT_0019f710) = 0;
        FUN_0012cbf6(DAT_0019f710);
        if ((local_18 == 0) && (local_20 == -1)) {
          uVar2 = 1;
        }
        else {
          uVar2 = 0;
        }
      }
      return uVar2;
    }
    if (local_18 == DAT_0019f718) {
      DAT_0019f710 = FUN_00160bd8(DAT_0019f710,&DAT_0019f718);
    }
    *(char *)(local_18 + DAT_0019f710) = (char)local_20;
    if (local_20 == 0) break;
    local_20 = getc_unlocked(__stream);
    local_18 = local_18 + 1;
  }
  return 2;
}




// Function: handle_option @ 0x2cd8e

undefined8 handle_option(char *param_1,long param_2)

{
  int iVar1;
  ushort **ppuVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  char *local_140;
  int local_12c;
  undefined4 local_128 [2];
  undefined8 local_120;
  undefined8 local_118;
  long local_108;
  undefined8 *local_100;
  long local_f8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  for (local_140 = param_1; *local_140 != '\0'; local_140 = local_140 + 1) {
    ppuVar2 = __ctype_b_loc();
    if (((*ppuVar2)[*local_140] & 0x2000) == 0) break;
  }
  if (*local_140 == '-') {
    local_f8 = 1;
    iVar1 = FUN_001461a6(local_140,&local_108,0x2000e46);
    if (iVar1 != 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar3 = FUN_0014647f(&local_108);
      uVar4 = gettext("cannot split string \'%s\': %s");
      error(0,0,uVar4,local_140,uVar3);
                    /* WARNING: Subroutine does not return */
      FUN_001177f4();
    }
    *local_100 = DAT_0019fe48;
    local_128[0] = 2;
    local_120 = *(undefined8 *)(param_2 + 0x18);
    local_118 = *(undefined8 *)(param_2 + 0x20);
    FUN_0013828e((int)local_f8 + (int)local_108,local_100,local_128);
    local_12c = 0;
    while( true ) {
      if ((ulong)(local_f8 + local_108) <= (ulong)(long)local_12c) break;
      local_100[local_12c] = 0;
      local_12c = local_12c + 1;
    }
    FUN_00146399(&local_108);
    uVar3 = 0;
  }
  else {
    uVar3 = 1;
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar3;
}




// Function: read_next_name @ 0x2cf89

undefined8 read_next_name(long param_1,long param_2)

{
  int iVar1;
  uint uVar2;
  FILE *pFVar3;
  undefined8 uVar4;
  
  if (*(long *)(param_1 + 0x30) == 0) {
    iVar1 = strcmp(*(char **)(param_1 + 0x18),"-");
    if (iVar1 == 0) {
      FUN_00134a6f(&DAT_0018390c);
      *(undefined8 *)(param_1 + 0x30) = stdin;
    }
    else {
      iVar1 = FUN_0012c9f1(*(undefined8 *)(param_1 + 0x18));
      if (iVar1 != 0) {
        FUN_0012c78a();
        return 1;
      }
      pFVar3 = fopen(*(char **)(param_1 + 0x18),"r");
      *(FILE **)(param_1 + 0x30) = pFVar3;
      if (*(long *)(param_1 + 0x30) == 0) {
        FUN_0013e3b4(*(undefined8 *)(param_1 + 0x18));
      }
    }
    *(int *)(param_1 + 0x28) = (int)DAT_0019c270;
    *(undefined1 *)(param_1 + 0x2c) = DAT_0019f6b8;
  }
  while( true ) {
    do {
      do {
        uVar2 = FUN_0012cc4a(param_1);
      } while (uVar2 == 3);
    } while (3 < uVar2);
    if (uVar2 == 2) break;
    if (uVar2 < 3) {
      if (uVar2 == 0) goto LAB_0012d103;
      if (uVar2 == 1) {
        iVar1 = strcmp(*(char **)(param_1 + 0x18),"-");
        if (iVar1 != 0) {
          fclose(*(FILE **)(param_1 + 0x30));
        }
        *(undefined8 *)(param_1 + 0x30) = 0;
        FUN_0012c78a();
        return 1;
      }
    }
  }
  if ((DAT_0019ec84 & 0x200) != 0) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar4 = FUN_0015e548(*(undefined8 *)(param_1 + 0x18));
    error(0,0,"%s: file name read contains nul character",uVar4);
  }
  *(undefined4 *)(param_1 + 0x28) = 0;
LAB_0012d103:
  if (*(char *)(param_1 + 0x2c) != '\x01') {
    if (DAT_0019fc28 != '\0') {
      FUN_0012961d(DAT_0019f710);
    }
    iVar1 = FUN_0012cd8e(DAT_0019f710,param_1);
    if (iVar1 == 0) {
      FUN_0012c4b8();
      return 1;
    }
  }
  *(undefined4 *)(param_2 + 0x10) = 0;
  *(undefined8 *)(param_2 + 0x18) = DAT_0019f710;
  return 0;
}




// Function: copy_name @ 0x2d1c1

void copy_name(long param_1)

{
  char *__s;
  size_t sVar1;
  
  __s = *(char **)(param_1 + 0x18);
  sVar1 = strlen(__s);
  while (DAT_0019f718 <= sVar1) {
    DAT_0019f710 = (char *)FUN_00160bd8(DAT_0019f710,&DAT_0019f718);
  }
  strcpy(DAT_0019f710,__s);
  FUN_0012cbf6(DAT_0019f710);
  return;
}




// Function: name_next_elt @ 0x2d244

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined * name_next_elt(int param_1)

{
  long lVar1;
  int iVar2;
  undefined4 uVar3;
  undefined8 uVar4;
  
  do {
    lVar1 = DAT_0019f708;
    if (DAT_0019f708 == 0) {
      FUN_0012c588();
      return (undefined *)0x0;
    }
    switch(*(undefined4 *)(DAT_0019f708 + 0x10)) {
    case 0:
switchD_0012d289_caseD_0:
      FUN_0012d1c1(DAT_0019f708);
      if (DAT_0019fc28 != '\0') {
        FUN_0012961d(DAT_0019f710);
      }
      _DAT_0019f750 = *(undefined4 *)(lVar1 + 0x10);
      _DAT_0019f758 = DAT_0019f710;
      FUN_0012c78a();
      return &DAT_0019f740;
    case 1:
      if (param_1 == 0) goto switchD_0012d289_caseD_0;
      uVar4 = FUN_00160cc6(*(undefined8 *)(DAT_0019f708 + 0x18));
      uVar3 = FUN_0012ac09(uVar4);
      FUN_0012adf1(uVar3);
      FUN_0012c78a();
      break;
    case 2:
      iVar2 = FUN_0012cf89(DAT_0019f708,&DAT_0019f740);
      if (iVar2 == 0) {
        return &DAT_0019f740;
      }
      break;
    case 3:
      FUN_0012c78a();
      break;
    case 4:
      FUN_0012bc3a(*(undefined4 *)(DAT_0019f708 + 0x18),*(undefined8 *)(DAT_0019f708 + 0x20));
      FUN_0012c78a();
    }
  } while( true );
}




// Function: name_next @ 0x2d37c

undefined8 name_next(undefined4 param_1)

{
  long lVar1;
  undefined8 uVar2;
  
  lVar1 = FUN_0012d244(param_1);
  if (lVar1 == 0) {
    uVar2 = 0;
  }
  else {
    uVar2 = *(undefined8 *)(lVar1 + 0x18);
  }
  return uVar2;
}




// Function: name_gather @ 0x2d3b1

void name_gather(void)

{
  int iVar1;
  undefined8 uVar2;
  long lVar3;
  uint uVar4;
  int local_18;
  
  if (DAT_0019f958 == '\0') {
    local_18 = 0;
    iVar1 = local_18;
    while( true ) {
      while ((lVar3 = FUN_0012d244(0), lVar3 != 0 && (*(int *)(lVar3 + 0x10) == 1))) {
        uVar2 = FUN_00160cc6(*(undefined8 *)(lVar3 + 0x18));
        local_18 = FUN_0012ac09(uVar2);
      }
      if (lVar3 == 0) break;
      FUN_0012d5b5(*(undefined8 *)(lVar3 + 0x18),local_18,1,0);
      iVar1 = local_18;
    }
    if (local_18 != iVar1) {
      FUN_0012d5b5(0,local_18,0,0);
    }
  }
  else {
    while ((lVar3 = FUN_0012d244(0), lVar3 != 0 && (*(int *)(lVar3 + 0x10) == 1))) {
      uVar2 = FUN_00160cc6(*(undefined8 *)(lVar3 + 0x18));
      DAT_0019f778 = FUN_0012ac09(uVar2);
    }
    if (lVar3 == 0) {
      if (DAT_0019f778 != 0) {
        FUN_0012d5b5(0,DAT_0019f778,0,0);
      }
    }
    else {
      FUN_0012c3b1(DAT_0019f780);
      DAT_0019f780 = (undefined8 *)FUN_0012c34c(*(undefined8 *)(lVar3 + 0x18));
      *(int *)(DAT_0019f780 + 5) = DAT_0019f778;
      *DAT_0019f780 = 0;
      DAT_0019f780[6] = 0;
      if (DAT_0019f6bc == 2) {
        uVar4 = 0x10000000;
      }
      else {
        uVar4 = 0;
      }
      *(uint *)(DAT_0019f780 + 4) = DAT_0019f930 | uVar4 | DAT_0019c274 | DAT_0019f6c0;
      DAT_0019f780[7] = 0;
      DAT_0019f780[8] = 0;
      *(undefined1 *)((long)DAT_0019f780 + 0x24) = 1;
      DAT_0019f700 = DAT_0019f780;
      DAT_0019f6f8 = DAT_0019f780;
    }
  }
  return;
}




// Function: addname @ 0x2d5b5

undefined8 *
addname(undefined8 param_1,undefined4 param_2,undefined1 param_3,undefined8 param_4)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  uint uVar3;
  
  puVar2 = (undefined8 *)FUN_0012c34c(param_1);
  puVar2[1] = DAT_0019f700;
  *puVar2 = 0;
  puVar2[6] = 0;
  if (DAT_0019f6bc == 2) {
    uVar3 = 0x10000000;
  }
  else {
    uVar3 = 0;
  }
  *(uint *)(puVar2 + 4) = uVar3 | DAT_0019c274 | DAT_0019f6c0 | DAT_0019f930;
  *(undefined4 *)(puVar2 + 5) = param_2;
  puVar2[7] = 0;
  puVar2[8] = param_4;
  *(undefined1 *)((long)puVar2 + 0x24) = param_3;
  puVar1 = puVar2;
  if (DAT_0019f700 != (undefined8 *)0x0) {
    *DAT_0019f700 = puVar2;
    puVar1 = DAT_0019f6f8;
  }
  DAT_0019f6f8 = puVar1;
  DAT_0019f700 = puVar2;
  return puVar2;
}




// Function: add_starting_file @ 0x2d6a2

void add_starting_file(undefined8 param_1)

{
  undefined8 uVar1;
  undefined8 *puVar2;
  uint uVar3;
  
  puVar2 = (undefined8 *)FUN_0012c34c(param_1);
  uVar1 = DAT_0019f6f8;
  if (DAT_0019f988 != '\0') {
    FUN_0012d817(DAT_0019f6f8);
    FUN_0012c3b1(uVar1);
  }
  puVar2[1] = 0;
  *puVar2 = DAT_0019f6f8;
  if (DAT_0019f700 == (undefined8 *)0x0) {
    DAT_0019f700 = puVar2;
  }
  DAT_0019f6f8 = puVar2;
  puVar2[6] = 0;
  if (DAT_0019f6bc == 2) {
    uVar3 = 0x10000000;
  }
  else {
    uVar3 = 0;
  }
  *(uint *)(puVar2 + 4) = uVar3 | DAT_0019c274 | DAT_0019f6c0 | DAT_0019f930;
  *(undefined4 *)(puVar2 + 5) = 0;
  puVar2[7] = 0;
  puVar2[8] = 0;
  *(undefined1 *)((long)puVar2 + 0x24) = 1;
  DAT_0019f988 = 1;
  return;
}




// Function: namelist_match @ 0x2d7a6

undefined8 * namelist_match(undefined8 param_1)

{
  char cVar1;
  undefined8 *local_10;
  
  local_10 = DAT_0019f6f8;
  while( true ) {
    if (local_10 == (undefined8 *)0x0) {
      return (undefined8 *)0x0;
    }
    if ((*(char *)local_10[2] != '\0') &&
       (cVar1 = FUN_0014ead5(local_10[2],param_1,*(undefined4 *)(local_10 + 4)), cVar1 != '\0'))
    break;
    local_10 = (undefined8 *)*local_10;
  }
  return local_10;
}




// Function: remname @ 0x2d817

void remname(long *param_1)

{
  if ((long *)param_1[1] == (long *)0x0) {
    DAT_0019f6f8 = *param_1;
  }
  else {
    *(long *)param_1[1] = *param_1;
  }
  if (*param_1 == 0) {
    DAT_0019f700 = param_1[1];
  }
  else {
    *(long *)(*param_1 + 8) = param_1[1];
  }
  return;
}




// Function: name_match @ 0x2d88a

bool name_match(char *param_1)

{
  size_t sVar1;
  long local_18;
  
  sVar1 = strlen(param_1);
  while( true ) {
    if (DAT_0019f6f8 == 0) {
      return true;
    }
    if (**(char **)(DAT_0019f6f8 + 0x10) == '\0') break;
    local_18 = FUN_0012d7a6(param_1,sVar1);
    if (DAT_0019f988 != '\0') {
      if (local_18 == DAT_0019f6f8) {
        DAT_0019f988 = '\0';
      }
      else {
        local_18 = 0;
      }
    }
    if (local_18 != 0) {
      if (((param_1[*(long *)(local_18 + 0x18)] != '/') || (DAT_0019f930 == 0)) ||
         (*(long *)(local_18 + 0x30) == 0)) {
        *(long *)(local_18 + 0x30) = *(long *)(local_18 + 0x30) + 1;
      }
      FUN_0012adf1(*(undefined4 *)(local_18 + 0x28));
      if (DAT_0019f8d0 == 0) {
        return *(long *)(local_18 + 0x30) != 0;
      }
      return *(long *)(local_18 + 0x30) == DAT_0019f8d0;
    }
    if ((DAT_0019f958 == '\0') || (*(long *)(DAT_0019f6f8 + 0x30) == 0)) {
      return false;
    }
    FUN_0012d3b1();
    if (*(long *)(DAT_0019f6f8 + 0x30) != 0) {
      return false;
    }
  }
  FUN_0012adf1(*(undefined4 *)(DAT_0019f6f8 + 0x28));
  DAT_0019f6f8 = 0;
  DAT_0019f700 = 0;
  return true;
}




// Function: all_names_found @ 0x2da15

undefined8 all_names_found(long param_1)

{
  undefined8 uVar1;
  size_t sVar2;
  undefined8 *local_18;
  
  if (((*(long *)(param_1 + 8) == 0) || (DAT_0019f8d0 == 0)) || (*(char *)(param_1 + 0x10) != '\0'))
  {
    uVar1 = 0;
  }
  else {
    sVar2 = strlen(*(char **)(param_1 + 8));
    for (local_18 = DAT_0019f6f8; local_18 != (undefined8 *)0x0; local_18 = (undefined8 *)*local_18)
    {
      if (*(char *)local_18[2] != '\0') {
        if (DAT_0019f8d0 == 0) {
          if (local_18[6] == 0) {
            return 0;
          }
        }
        else if ((ulong)local_18[6] < DAT_0019f8d0) {
          return 0;
        }
      }
      if (((ulong)local_18[3] <= sVar2) && (*(char *)(local_18[3] + *(long *)(param_1 + 8)) == '/'))
      {
        return 0;
      }
    }
    uVar1 = 1;
  }
  return uVar1;
}




// Function: regex_usage_warning @ 0x2dafb

undefined4 regex_usage_warning(undefined8 param_1)

{
  char cVar1;
  undefined8 uVar2;
  
  if (DAT_0019f6bc == 0) {
    cVar1 = FUN_0014e397(param_1,0);
    if (cVar1 != '\0') {
      DAT_0019f788 = 1;
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar2 = gettext("Pattern matching characters used in file names");
      error(0,0,uVar2);
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar2 = gettext(
                     "Use --wildcards to enable pattern matching, or --no-wildcards to suppress this warning"
                     );
      error(0,0,uVar2);
    }
  }
  return DAT_0019f788;
}




// Function: names_notfound @ 0x2dbba

void names_notfound(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  long lVar3;
  bool bVar4;
  undefined8 *local_28;
  
  for (local_28 = DAT_0019f6f8; local_28 != (undefined8 *)0x0; local_28 = (undefined8 *)*local_28) {
    if (DAT_0019f8d0 == 0) {
      bVar4 = local_28[6] == 0;
    }
    else {
      bVar4 = (ulong)local_28[6] < DAT_0019f8d0;
    }
    if ((bVar4) && (*(char *)local_28[2] != '\0')) {
      FUN_0012dafb(local_28[2]);
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar1 = FUN_0015e548(local_28[2]);
      if (local_28[6] == 0) {
        uVar2 = gettext("%s: Not found in archive");
      }
      else {
        uVar2 = gettext("%s: Required occurrence not found in archive");
      }
      error(0,0,uVar2,uVar1);
      DAT_0019fdd0 = 2;
    }
  }
  DAT_0019f6f8 = (undefined8 *)0x0;
  DAT_0019f700 = 0;
  if (DAT_0019f958 != '\0') {
    while (lVar3 = FUN_0012d37c(1), lVar3 != 0) {
      FUN_0012dafb(lVar3);
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar1 = FUN_0015e548(lVar3);
      uVar2 = gettext("%s: Not found in archive");
      error(0,0,uVar2,uVar1);
      DAT_0019fdd0 = 2;
    }
  }
  return;
}




// Function: label_notfound @ 0x2dd64

void label_notfound(void)

{
  int iVar1;
  undefined8 uVar2;
  long lVar3;
  bool bVar4;
  undefined8 *local_18;
  
  if (DAT_0019f6f8 != (undefined8 *)0x0) {
    for (local_18 = DAT_0019f6f8; local_18 != (undefined8 *)0x0; local_18 = (undefined8 *)*local_18)
    {
      if (DAT_0019f8d0 == 0) {
        bVar4 = local_18[6] != 0;
      }
      else {
        bVar4 = DAT_0019f8d0 <= (ulong)local_18[6];
      }
      if (bVar4) {
        return;
      }
    }
    if (DAT_0019f9ac != 0) {
      uVar2 = gettext("Archive label mismatch");
      error(0,0,uVar2);
    }
    FUN_00139e98(1);
    local_18 = DAT_0019f6f8;
    while ((local_18 != (undefined8 *)0x0 && (iVar1 = FUN_0012dafb(local_18[2]), iVar1 == 0))) {
      local_18 = (undefined8 *)*local_18;
    }
    DAT_0019f6f8 = (undefined8 *)0x0;
    DAT_0019f700 = 0;
    if (DAT_0019f958 != '\0') {
      do {
        lVar3 = FUN_0012d37c(1);
        if (lVar3 == 0) {
          return;
        }
        iVar1 = FUN_0012dafb(lVar3);
      } while (iVar1 == 0);
    }
  }
  return;
}




// Function: merge_sort_sll @ 0x2de97

undefined8 **** merge_sort_sll(undefined8 ****param_1,int param_2,code *param_3)

{
  int iVar1;
  long in_FS_OFFSET;
  int local_44;
  undefined8 ***local_38;
  undefined8 ***local_30;
  undefined8 ***local_28;
  undefined8 ***local_20;
  undefined8 ***local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_2 != 1) {
    if (param_2 == 2) {
      iVar1 = (*param_3)(param_1,*param_1);
      if (0 < iVar1) {
        local_38 = *param_1;
        *local_38 = param_1;
        *param_1 = (undefined8 ***)0x0;
        param_1 = (undefined8 ****)local_38;
      }
    }
    else {
      iVar1 = (param_2 + 1) / 2;
      local_18 = param_1;
      local_44 = iVar1;
      while (local_44 = local_44 + -1, local_44 != 0) {
        local_18 = (undefined8 ***)*local_18;
      }
      local_28 = (undefined8 ***)*local_18;
      *local_18 = (undefined8 ***)0x0;
      local_30 = param_1;
      local_30 = (undefined8 ***)merge_sort_sll(param_1,iVar1,param_3);
      local_28 = (undefined8 ***)merge_sort_sll(local_28,param_2 / 2,param_3);
      local_20 = &local_38;
      while (((undefined8 ****)local_30 != (undefined8 ****)0x0 &&
             ((undefined8 ****)local_28 != (undefined8 ****)0x0))) {
        iVar1 = (*param_3)(local_30,local_28);
        if (iVar1 < 0) {
          local_18 = (undefined8 ***)*local_30;
          *local_20 = local_30;
          local_20 = local_30;
          local_30 = local_18;
        }
        else {
          local_18 = (undefined8 ***)*local_28;
          *local_20 = local_28;
          local_20 = local_28;
          local_28 = local_18;
        }
      }
      if ((undefined8 ****)local_30 == (undefined8 ****)0x0) {
        *local_20 = local_28;
        param_1 = (undefined8 ****)local_38;
      }
      else {
        *local_20 = local_30;
        param_1 = (undefined8 ****)local_38;
      }
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return param_1;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: merge_sort @ 0x2e071

undefined8 * merge_sort(undefined8 param_1,undefined4 param_2,undefined8 param_3)

{
  undefined8 *puVar1;
  undefined8 *local_20;
  undefined8 *local_18;
  
  puVar1 = (undefined8 *)FUN_0012de97(param_1,param_2,param_3);
  local_18 = (undefined8 *)0x0;
  for (local_20 = puVar1; local_20 != (undefined8 *)0x0; local_20 = (undefined8 *)*local_20) {
    local_20[1] = local_18;
    local_18 = local_20;
  }
  return puVar1;
}




// Function: compare_names_found @ 0x2e0df

int compare_names_found(long param_1,long param_2)

{
  int iVar1;
  bool bVar2;
  bool bVar3;
  
  if (DAT_0019f8d0 == 0) {
    bVar2 = *(long *)(param_2 + 0x30) != 0;
  }
  else {
    bVar2 = DAT_0019f8d0 <= *(ulong *)(param_2 + 0x30);
  }
  if (DAT_0019f8d0 == 0) {
    bVar3 = *(long *)(param_1 + 0x30) != 0;
  }
  else {
    bVar3 = DAT_0019f8d0 <= *(ulong *)(param_1 + 0x30);
  }
  iVar1 = (uint)bVar2 - (uint)bVar3;
  if (iVar1 == 0) {
    iVar1 = strcmp(*(char **)(param_1 + 0x10),*(char **)(param_2 + 0x10));
  }
  return iVar1;
}




// Function: compare_names @ 0x2e18e

void compare_names(long param_1,long param_2)

{
  strcmp(*(char **)(param_1 + 0x10),*(char **)(param_2 + 0x10));
  return;
}




// Function: add_hierarchy_to_namelist @ 0x2e1bf

void add_hierarchy_to_namelist(long param_1,long param_2)

{
  undefined4 uVar1;
  long lVar2;
  int iVar3;
  undefined8 uVar4;
  int *piVar5;
  long in_FS_OFFSET;
  int local_228;
  ulong local_220;
  long local_218;
  long local_210;
  ulong local_208;
  char *local_200;
  char *local_1f8;
  char *local_1f0;
  size_t local_1e8;
  long local_1e0;
  undefined8 local_1d8 [11];
  stat local_180 [2];
  long local_40;
  int local_30;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar4 = FUN_00121eb9(param_1);
  *(undefined8 *)(param_2 + 0x38) = uVar4;
  local_1f0 = (char *)FUN_0012241b(*(undefined8 *)(param_2 + 0x38));
  if (local_1f0 != (char *)0x0) {
    local_218 = 0;
    local_210 = 0;
    local_208 = *(ulong *)(param_2 + 0x18);
    if (local_208 < 100) {
      local_220 = 0x66;
    }
    else {
      local_220 = local_208 + 0x66;
    }
    local_200 = (char *)FUN_00160b5b(local_220);
    uVar1 = *(undefined4 *)(param_2 + 0x28);
    strcpy(local_200,*(char **)(param_2 + 0x10));
    if (local_200[local_208 - 1] != '/') {
      local_200[local_208] = '/';
      local_200[local_208 + 1] = '\0';
      local_208 = local_208 + 1;
    }
    for (local_1f8 = local_1f0; *local_1f8 != '\0'; local_1f8 = local_1f8 + local_1e8 + 1) {
      local_1e8 = strlen(local_1f8);
      if (*local_1f8 == 'D') {
        while (local_220 < local_208 + local_1e8) {
          local_200 = (char *)FUN_00160bd8(local_200,&local_220);
        }
        strcpy(local_200 + local_208,local_1f8 + 1);
        local_1e0 = FUN_0012d5b5(local_200,uVar1,0,param_2);
        lVar2 = local_1e0;
        if (local_218 != 0) {
          *(long *)(local_210 + 0x50) = local_1e0;
          lVar2 = local_218;
        }
        local_218 = lVar2;
        local_210 = local_1e0;
        FUN_00139c55(local_1d8);
        local_40 = param_1;
        if (*(int *)(param_1 + 0x1a8) < 0) {
          local_228 = -1;
          iVar3 = *(int *)(param_1 + 0x1a8);
          piVar5 = __errno_location();
          *piVar5 = -iVar3;
        }
        else {
          local_228 = FUN_001157f9(param_1,local_1f8 + 1,DAT_0019fc08 | 0x10000);
        }
        if (local_228 < 0) {
          FUN_0012b2b1(local_200);
        }
        else {
          local_30 = local_228;
          iVar3 = fstat(local_228,local_180);
          if (iVar3 == 0) {
            local_1d8[0] = FUN_00160cc6(local_200);
            add_hierarchy_to_namelist(local_1d8,local_1e0);
            FUN_0011588e(local_1d8);
          }
          else {
            FUN_0012b444(local_200);
          }
        }
        FUN_00139d1c(local_1d8);
      }
    }
    free(local_200);
    *(long *)(param_2 + 0x48) = local_218;
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: name_hash @ 0x2e590

void name_hash(long param_1,undefined8 param_2)

{
  FUN_00152834(*(undefined8 *)(param_1 + 0x58),param_2);
  return;
}




// Function: name_compare @ 0x2e5c5

undefined8 name_compare(long param_1,long param_2)

{
  int iVar1;
  undefined4 extraout_var;
  
  iVar1 = strcmp(*(char **)(param_1 + 0x58),*(char **)(param_2 + 0x58));
  return CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),iVar1 == 0);
}




// Function: rebase_child_list @ 0x2e60b

void rebase_child_list(long param_1,long param_2)

{
  long lVar1;
  long lVar2;
  char *__src;
  long lVar3;
  char *__dest;
  undefined8 local_40;
  
  lVar1 = *(long *)(*(long *)(param_1 + 0x40) + 0x18);
  lVar2 = *(long *)(param_2 + 0x18);
  __src = *(char **)(param_2 + 0x10);
  for (local_40 = param_1; local_40 != 0; local_40 = *(long *)(local_40 + 0x50)) {
    lVar3 = lVar2 + (*(long *)(local_40 + 0x18) - lVar1);
    __dest = (char *)FUN_00160b5b(lVar3 + 1);
    strcpy(__dest,__src);
    strcat(__dest,(char *)(*(long *)(local_40 + 0x10) + lVar1));
    free(*(void **)(local_40 + 0x10));
    *(char **)(local_40 + 0x10) = __dest;
    *(long *)(local_40 + 0x18) = lVar3;
    FUN_00121244(*(undefined8 *)(local_40 + 0x38),*(undefined8 *)(*(long *)(local_40 + 0x40) + 0x10)
                 ,lVar1,__src,lVar2);
  }
  return;
}




// Function: collect_and_sort_names @ 0x2e719

void collect_and_sort_names(void)

{
  undefined8 *puVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long lVar5;
  long in_FS_OFFSET;
  int local_1f8;
  undefined8 *local_1f0;
  undefined8 *local_1e8;
  undefined8 local_1c8 [11];
  stat local_170 [2];
  int local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_1e8 = (undefined8 *)0x0;
  FUN_0012d3b1();
  if (DAT_0019f6f8 == (undefined8 *)0x0) {
    FUN_0012d5b5(&DAT_00183a3c,0,0,0);
  }
  if (DAT_0019f8e0 != 0) {
    iVar2 = FUN_0012abe0();
    if (iVar2 != 0) {
      if (iVar2 != 1) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar3 = gettext("Only one -C option is allowed with --listed-incremental");
        error(0,0,uVar3);
                    /* WARNING: Subroutine does not return */
        FUN_0013814c(2);
      }
      if (*(int *)(DAT_0019f6f8 + 5) == 0) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar3 = gettext("Using -C option inside file list is not allowed with --listed-incremental")
        ;
        error(0,0,uVar3);
                    /* WARNING: Subroutine does not return */
        FUN_0013814c(2);
      }
    }
    FUN_00124090();
  }
  local_1f8 = 0;
  for (local_1f0 = DAT_0019f6f8; local_1f0 != (undefined8 *)0x0;
      local_1f0 = (undefined8 *)*local_1f0) {
    if ((((local_1f0[6] == 0) && (local_1f0[7] == 0)) &&
        ((*(uint *)(local_1f0 + 4) & 0x10000000) == 0)) &&
       (FUN_0012adf1(*(undefined4 *)(local_1f0 + 5)), *(char *)local_1f0[2] != '\0')) {
      FUN_00139c55(local_1c8);
      iVar2 = FUN_0012a9b8(local_1f0[2],local_170);
      if (iVar2 == 0) {
        if ((local_170[0].st_mode & 0xf000) == 0x4000) {
          iVar2 = openat(DAT_0019c268,(char *)local_1f0[2],DAT_0019fc08 | 0x10000);
          if (iVar2 < 0) {
            FUN_0012b2b1(local_1f0[2]);
          }
          else {
            local_20 = iVar2;
            iVar2 = fstat(iVar2,local_170);
            if (iVar2 == 0) {
              local_1c8[0] = FUN_00160cc6(local_1f0[2]);
              local_1f0[6] = local_1f0[6] + 1;
              FUN_0012e1bf(local_1c8,local_1f0);
            }
            else {
              FUN_0012b444(local_1f0[2]);
            }
          }
        }
        FUN_00139d1c(local_1c8);
      }
      else {
        FUN_0012b444(local_1f0[2]);
      }
    }
    local_1f8 = local_1f8 + 1;
  }
  DAT_0019f6f8 = (undefined8 *)FUN_0012e071(DAT_0019f6f8,local_1f8,FUN_0012e18e);
  local_1f8 = 0;
  uVar3 = FUN_00152c1c(0,0,FUN_0012e590,FUN_0012e5c5,0);
  local_1f0 = DAT_0019f6f8;
LAB_0012ec41:
  do {
    if (local_1f0 == (undefined8 *)0x0) {
      DAT_0019f700 = local_1e8;
      FUN_00152e9c(uVar3);
      DAT_0019f6f8 = (undefined8 *)FUN_0012e071(DAT_0019f6f8,local_1f8,FUN_0012e0df);
      local_1f0 = DAT_0019f6f8;
      if (DAT_0019f8e0 != 0) {
        for (; (local_1f0 != (undefined8 *)0x0 && (*(char *)local_1f0[2] == '\0'));
            local_1f0 = (undefined8 *)*local_1f0) {
        }
        if (local_1f0 != (undefined8 *)0x0) {
          FUN_001228cd(local_1f0[7]);
        }
      }
      if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
        return;
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    puVar1 = (undefined8 *)*local_1f0;
    uVar4 = FUN_00129aac(*(undefined4 *)(local_1f0 + 5),local_1f0[2]);
    local_1f0[0xb] = uVar4;
    if ((local_1e8 != (undefined8 *)0x0) && (lVar5 = FUN_00152567(uVar3,local_1f0), lVar5 != 0)) {
      if (local_1f0[8] != 0) {
        if (local_1f0[9] != 0) {
          FUN_0012e60b(local_1f0[9],lVar5);
        }
        FUN_0012d817(local_1f0);
        FUN_0012c3b1(local_1f0);
        local_1f0 = puVar1;
        goto LAB_0012ec41;
      }
      if (*(long *)(lVar5 + 0x48) != 0) {
        FUN_0012e60b(*(undefined8 *)(lVar5 + 0x48),local_1f0);
      }
      FUN_001539cd(uVar3,local_1f0);
      FUN_0012d817(lVar5);
      FUN_0012c3b1(lVar5);
      local_1f8 = local_1f8 + -1;
    }
    local_1f0[6] = 0;
    lVar5 = FUN_0015395d(uVar3,local_1f0);
    if (lVar5 == 0) {
      FUN_0011783c();
    }
    local_1e8 = local_1f0;
    local_1f8 = local_1f8 + 1;
    local_1f0 = puVar1;
  } while( true );
}




// Function: name_scan @ 0x2ed0e

long name_scan(char *param_1)

{
  size_t sVar1;
  long lVar2;
  
  sVar1 = strlen(param_1);
  while( true ) {
    lVar2 = FUN_0012d7a6(param_1,sVar1);
    if (lVar2 != 0) {
      return lVar2;
    }
    if (((DAT_0019f958 == '\0') || (DAT_0019f6f8 == 0)) || (*(long *)(DAT_0019f6f8 + 0x30) == 0))
    break;
    FUN_0012d3b1();
    if (*(long *)(DAT_0019f6f8 + 0x30) != 0) {
      return 0;
    }
  }
  return 0;
}




// Function: name_from_list @ 0x2eda0

undefined8 * name_from_list(void)

{
  undefined8 *puVar1;
  
  if (DAT_0019f6b0 == (undefined8 *)0x0) {
    DAT_0019f6b0 = DAT_0019f6f8;
  }
  for (; (DAT_0019f6b0 != (undefined8 *)0x0 &&
         ((DAT_0019f6b0[6] != 0 || (*(char *)DAT_0019f6b0[2] == '\0'))));
      DAT_0019f6b0 = (undefined8 *)*DAT_0019f6b0) {
  }
  if (DAT_0019f6b0 == (undefined8 *)0x0) {
    puVar1 = (undefined8 *)0x0;
  }
  else {
    DAT_0019f6b0[6] = DAT_0019f6b0[6] + 1;
    FUN_0012adf1(*(undefined4 *)(DAT_0019f6b0 + 5));
    puVar1 = DAT_0019f6b0;
  }
  return puVar1;
}




// Function: blank_name_list @ 0x2ee43

void blank_name_list(void)

{
  undefined8 *local_10;
  
  DAT_0019f6b0 = 0;
  for (local_10 = DAT_0019f6f8; local_10 != (undefined8 *)0x0; local_10 = (undefined8 *)*local_10) {
    local_10[6] = 0;
  }
  return;
}




// Function: make_file_name @ 0x2ee85

void * make_file_name(char *param_1,char *param_2)

{
  int iVar1;
  size_t __n;
  size_t sVar2;
  void *__dest;
  
  __n = strlen(param_1);
  sVar2 = strlen(param_2);
  if ((__n == 0) || (param_1[__n - 1] == '/')) {
    iVar1 = 0;
  }
  else {
    iVar1 = 1;
  }
  __dest = (void *)FUN_00160b5b(sVar2 + 1 + (long)iVar1 + __n);
  memcpy(__dest,param_1,__n);
  *(undefined1 *)(__n + (long)__dest) = 0x2f;
  memcpy((void *)((long)iVar1 + __n + (long)__dest),param_2,sVar2 + 1);
  return __dest;
}




// Function: stripped_prefix_len @ 0x2ef5c

long stripped_prefix_len(char *param_1,long param_2)

{
  char cVar1;
  long local_28;
  char *local_10;
  
  for (local_10 = param_1; local_28 = param_2, *local_10 == '/'; local_10 = local_10 + 1) {
  }
  while( true ) {
    do {
      if (*local_10 == '\0') {
        return -1;
      }
      cVar1 = *local_10;
      local_10 = local_10 + 1;
    } while (cVar1 != '/');
    local_28 = local_28 + -1;
    if (local_28 == 0) break;
    for (; *local_10 == '/'; local_10 = local_10 + 1) {
    }
  }
  return (long)local_10 - (long)param_1;
}




// Function: contains_dot_dot @ 0x2efdc

undefined8 contains_dot_dot(char *param_1)

{
  char cVar1;
  char *local_10;
  
  for (local_10 = param_1;
      ((*local_10 != '.' || (local_10[1] != '.')) || ((local_10[2] != '/' && (local_10[2] != '\0')))
      ); local_10 = local_10 + 1) {
    while (*local_10 != '/') {
      cVar1 = *local_10;
      local_10 = local_10 + 1;
      if (cVar1 == '\0') {
        return 0;
      }
    }
  }
  return 1;
}




// Function: dump_zeros @ 0x2f05d

undefined8 dump_zeros(int *param_1,long param_2)

{
  int *piVar1;
  undefined8 uVar2;
  size_t __n;
  ssize_t sVar3;
  
  if (param_2 < *(long *)(param_1 + 2)) {
    piVar1 = __errno_location();
    *piVar1 = 0x16;
    uVar2 = 0;
  }
  else {
    while (*(long *)(param_1 + 2) < param_2) {
      __n = param_2 - *(long *)(param_1 + 2);
      if (0x200 < (long)__n) {
        __n = 0x200;
      }
      sVar3 = write(*param_1,&DAT_00183d00,__n);
      if (sVar3 < 1) {
        if (sVar3 == 0) {
          piVar1 = __errno_location();
          *piVar1 = 0x16;
        }
        return 0;
      }
      *(long *)(param_1 + 2) = *(long *)(param_1 + 2) + sVar3;
    }
    uVar2 = 1;
  }
  return uVar2;
}




// Function: tar_sparse_member_p @ 0x2f126

undefined8 tar_sparse_member_p(long param_1)

{
  undefined8 uVar1;
  
  if (*(long *)(*(long *)(param_1 + 0x20) + 0x10) == 0) {
    uVar1 = 0;
  }
  else {
    uVar1 = (**(code **)(*(long *)(param_1 + 0x20) + 0x10))(param_1);
  }
  return uVar1;
}




// Function: tar_sparse_init @ 0x2f165

undefined8 tar_sparse_init(void *param_1)

{
  char cVar1;
  undefined8 uVar2;
  
  memset(param_1,0,0x30);
  cVar1 = FUN_0012fa5c(param_1);
  if (cVar1 == '\x01') {
    if (**(long **)((long)param_1 + 0x20) == 0) {
      uVar2 = 1;
    }
    else {
      uVar2 = (*(code *)**(undefined8 **)((long)param_1 + 0x20))(param_1);
    }
  }
  else {
    uVar2 = 0;
  }
  return uVar2;
}




// Function: tar_sparse_done @ 0x2f1d2

undefined8 tar_sparse_done(long param_1)

{
  undefined8 uVar1;
  
  if (*(long *)(*(long *)(param_1 + 0x20) + 8) == 0) {
    uVar1 = 1;
  }
  else {
    uVar1 = (**(code **)(*(long *)(param_1 + 0x20) + 8))(param_1);
  }
  return uVar1;
}




// Function: tar_sparse_scan @ 0x2f211

undefined8 tar_sparse_scan(long param_1,undefined4 param_2,undefined8 param_3)

{
  undefined8 uVar1;
  
  if (*(long *)(*(long *)(param_1 + 0x20) + 0x30) == 0) {
    uVar1 = 1;
  }
  else {
    uVar1 = (**(code **)(*(long *)(param_1 + 0x20) + 0x30))(param_1,param_2,param_3);
  }
  return uVar1;
}




// Function: tar_sparse_dump_region @ 0x2f261

undefined8 tar_sparse_dump_region(long param_1,undefined8 param_2)

{
  undefined8 uVar1;
  
  if (*(long *)(*(long *)(param_1 + 0x20) + 0x38) == 0) {
    uVar1 = 0;
  }
  else {
    uVar1 = (**(code **)(*(long *)(param_1 + 0x20) + 0x38))(param_1,param_2);
  }
  return uVar1;
}




// Function: tar_sparse_extract_region @ 0x2f2ab

undefined8 tar_sparse_extract_region(long param_1,undefined8 param_2)

{
  undefined8 uVar1;
  
  if (*(long *)(*(long *)(param_1 + 0x20) + 0x40) == 0) {
    uVar1 = 0;
  }
  else {
    uVar1 = (**(code **)(*(long *)(param_1 + 0x20) + 0x40))(param_1,param_2);
  }
  return uVar1;
}




// Function: tar_sparse_dump_header @ 0x2f2f5

undefined8 tar_sparse_dump_header(long param_1)

{
  undefined8 uVar1;
  
  if (*(long *)(*(long *)(param_1 + 0x20) + 0x18) == 0) {
    uVar1 = 0;
  }
  else {
    uVar1 = (**(code **)(*(long *)(param_1 + 0x20) + 0x18))(param_1);
  }
  return uVar1;
}




// Function: tar_sparse_decode_header @ 0x2f334

undefined8 tar_sparse_decode_header(long param_1)

{
  undefined8 uVar1;
  
  if (*(long *)(*(long *)(param_1 + 0x20) + 0x28) == 0) {
    uVar1 = 1;
  }
  else {
    uVar1 = (**(code **)(*(long *)(param_1 + 0x20) + 0x28))(param_1);
  }
  return uVar1;
}




// Function: tar_sparse_fixup_header @ 0x2f373

undefined8 tar_sparse_fixup_header(long param_1)

{
  undefined8 uVar1;
  
  if (*(long *)(*(long *)(param_1 + 0x20) + 0x20) == 0) {
    uVar1 = 1;
  }
  else {
    uVar1 = (**(code **)(*(long *)(param_1 + 0x20) + 0x20))(param_1);
  }
  return uVar1;
}




// Function: lseek_or_error @ 0x2f3b2

bool lseek_or_error(int *param_1,__off_t param_2)

{
  byte bVar1;
  __off_t _Var2;
  
  if ((char)param_1[1] == '\0') {
    bVar1 = FUN_0012f05d(param_1,param_2);
    bVar1 = bVar1 ^ 1;
  }
  else {
    _Var2 = lseek(*param_1,param_2,0);
    bVar1 = (byte)((ulong)_Var2 >> 0x3f);
  }
  if (bVar1 != 0) {
    FUN_0012b3eb(**(undefined8 **)(param_1 + 6),param_2);
  }
  return bVar1 == 0;
}




// Function: zero_block_p @ 0x2f433

undefined8 zero_block_p(char *param_1,long param_2)

{
  char cVar1;
  long local_18;
  char *local_10;
  
  local_18 = param_2;
  local_10 = param_1;
  do {
    if (local_18 == 0) {
      return 1;
    }
    cVar1 = *local_10;
    local_18 = local_18 + -1;
    local_10 = local_10 + 1;
  } while (cVar1 == '\0');
  return 0;
}




// Function: sparse_add_map @ 0x2f477

void sparse_add_map(long param_1,undefined8 *param_2)

{
  undefined8 *puVar1;
  long lVar2;
  undefined8 uVar3;
  long local_18;
  
  local_18 = *(long *)(param_1 + 0x140);
  lVar2 = *(long *)(param_1 + 0x130);
  if (lVar2 == *(long *)(param_1 + 0x138)) {
    local_18 = FUN_00160a5f(local_18,param_1 + 0x138,0x10);
    *(long *)(param_1 + 0x140) = local_18;
  }
  puVar1 = (undefined8 *)(lVar2 * 0x10 + local_18);
  uVar3 = param_2[1];
  *puVar1 = *param_2;
  puVar1[1] = uVar3;
  *(long *)(param_1 + 0x130) = lVar2 + 1;
  return;
}




// Function: sparse_scan_file_raw @ 0x2f527

undefined8 sparse_scan_file_raw(undefined4 *param_1)

{
  undefined4 uVar1;
  long lVar2;
  char cVar3;
  undefined8 uVar4;
  long lVar5;
  long in_FS_OFFSET;
  long local_240;
  long local_228;
  long local_220;
  undefined1 local_218 [520];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  lVar2 = *(long *)(param_1 + 6);
  uVar1 = *param_1;
  local_240 = 0;
  local_228 = 0;
  local_220 = 0;
  *(undefined8 *)(lVar2 + 0x118) = 0;
  cVar3 = FUN_0012f211(param_1,0,0);
  if (cVar3 == '\x01') {
    while( true ) {
      lVar5 = FUN_0012a9e9(uVar1,local_218,0x200);
      if ((lVar5 == 0) || (lVar5 == -1)) break;
      cVar3 = FUN_0012f433(local_218,lVar5);
      if (cVar3 == '\0') {
        if (local_220 == 0) {
          local_228 = local_240;
        }
        local_220 = lVar5 + local_220;
        *(long *)(lVar2 + 0x118) = lVar5 + *(long *)(lVar2 + 0x118);
        cVar3 = FUN_0012f211(param_1,1,local_218);
        if (cVar3 != '\x01') {
          uVar4 = 0;
          goto LAB_0012f7b0;
        }
      }
      else if (local_220 != 0) {
        FUN_0012f477(lVar2,&local_228);
        local_220 = 0;
        cVar3 = FUN_0012f211(param_1,1,0);
        if (cVar3 != '\x01') {
          uVar4 = 0;
          goto LAB_0012f7b0;
        }
      }
      local_240 = lVar5 + local_240;
    }
    if (local_220 == 0) {
      local_228 = local_240;
    }
    FUN_0012f477(lVar2,&local_228);
    *(long *)(lVar2 + 0x118) = lVar5 + *(long *)(lVar2 + 0x118);
    uVar4 = FUN_0012f211(param_1,2,0);
  }
  else {
    uVar4 = 0;
  }
LAB_0012f7b0:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar4;
}




// Function: sparse_scan_file_wholesparse @ 0x2f7c6

bool sparse_scan_file_wholesparse(long param_1)

{
  long lVar1;
  long in_FS_OFFSET;
  bool bVar2;
  undefined8 local_28;
  undefined8 local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  lVar1 = *(long *)(param_1 + 0x18);
  local_28 = 0;
  local_20 = 0;
  bVar2 = *(long *)(lVar1 + 0x98) == 0;
  if (bVar2) {
    *(undefined8 *)(lVar1 + 0x118) = 0;
    local_28 = *(undefined8 *)(lVar1 + 0x88);
    FUN_0012f477(lVar1,&local_28);
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return bVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sparse_scan_file_seek @ 0x2f864

undefined8 sparse_scan_file_seek(int *param_1)

{
  int __fd;
  long lVar1;
  __off_t __offset;
  int *piVar2;
  undefined8 uVar3;
  __off_t _Var4;
  long in_FS_OFFSET;
  long local_48;
  __off_t local_28;
  long local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  lVar1 = *(long *)(param_1 + 6);
  __fd = *param_1;
  local_28 = 0;
  local_20 = 0;
  local_48 = 0;
  *(undefined8 *)(lVar1 + 0x118) = 0;
  do {
    __offset = lseek(__fd,local_48,3);
    if (__offset == -1) {
      piVar2 = __errno_location();
      if (*piVar2 == 6) {
        local_20 = 0;
        local_28 = *(__off_t *)(lVar1 + 0x88);
        FUN_0012f477(lVar1,&local_28);
        uVar3 = 1;
      }
      else {
        uVar3 = 0;
      }
LAB_0012f9cf:
      if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
        __stack_chk_fail();
      }
      return uVar3;
    }
    _Var4 = lseek(__fd,__offset,4);
    if (((local_48 == 0) && (__offset == 0)) && (_Var4 == *(long *)(lVar1 + 0x88))) {
      lseek(__fd,0,0);
      uVar3 = 0;
      goto LAB_0012f9cf;
    }
    local_20 = _Var4 - __offset;
    local_28 = __offset;
    FUN_0012f477(lVar1,&local_28);
    *(long *)(lVar1 + 0x118) = *(long *)(lVar1 + 0x118) + local_20;
    local_48 = _Var4;
  } while( true );
}




// Function: sparse_scan_file @ 0x2f9e5

undefined8 sparse_scan_file(undefined8 param_1)

{
  char cVar1;
  
  cVar1 = FUN_0012f7c6(param_1);
  if (cVar1 != '\0') {
    return 1;
  }
  if (DAT_0019f984 != 2) {
    if (2 < DAT_0019f984) {
      return 0;
    }
    if (DAT_0019f984 != 0) {
      if (DAT_0019f984 != 1) {
        return 0;
      }
      goto LAB_0012fa3e;
    }
  }
  cVar1 = FUN_0012f864(param_1);
  if (cVar1 != '\0') {
    return 1;
  }
LAB_0012fa3e:
  cVar1 = FUN_0012f527(param_1);
  if (cVar1 == '\0') {
    return 0;
  }
  return 1;
}




// Function: sparse_select_optab @ 0x2fa5c

undefined8 sparse_select_optab(long param_1)

{
  int iVar1;
  
  iVar1 = DAT_0019f468;
  if (DAT_0019f468 == 0) {
    iVar1 = DAT_0019f864;
  }
  switch(iVar1) {
  default:
    return 0;
  case 1:
  case 3:
    return 0;
  case 2:
  case 6:
    *(undefined **)(param_1 + 0x20) = &DAT_0019a7c0;
    break;
  case 4:
    *(undefined **)(param_1 + 0x20) = &DAT_0019a880;
    break;
  case 5:
    *(undefined **)(param_1 + 0x20) = &DAT_0019a820;
  }
  return 1;
}




// Function: sparse_dump_region @ 0x2faf0

undefined8 sparse_dump_region(int *param_1,long param_2)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  long lVar4;
  long lVar5;
  long lVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  long in_FS_OFFSET;
  long local_f0;
  long local_e8;
  stat local_c8;
  undefined1 local_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_f0 = *(long *)(param_2 * 0x10 + *(long *)(*(long *)(param_1 + 6) + 0x140) + 8);
  cVar1 = FUN_0012f3b2(param_1,*(undefined8 *)
                                (param_2 * 0x10 + *(long *)(*(long *)(param_1 + 6) + 0x140)));
  if (cVar1 == '\x01') {
    while (0 < local_f0) {
      lVar4 = local_f0;
      if (0x200 < local_f0) {
        lVar4 = 0x200;
      }
      lVar5 = FUN_0010bb4b();
      lVar6 = FUN_0015ec70(*param_1,lVar5,lVar4);
      if (lVar6 == -1) {
        FUN_0012b2f8(**(undefined8 **)(param_1 + 6),
                     (*(long *)(param_2 * 0x10 + *(long *)(*(long *)(param_1 + 6) + 0x140) + 8) +
                     *(long *)(param_2 * 0x10 + *(long *)(*(long *)(param_1 + 6) + 0x140))) -
                     local_f0,lVar4);
        uVar3 = 0;
        goto LAB_0012fe74;
      }
      if (lVar6 == 0) {
        iVar2 = fstat(*param_1,&local_c8);
        if (iVar2 == 0) {
          local_e8 = *(long *)(*(long *)(param_1 + 6) + 0x88) - local_c8.st_size;
        }
        else {
          local_e8 = *(long *)(*(long *)(param_1 + 6) + 0x88) -
                     ((*(long *)(param_2 * 0x10 + *(long *)(*(long *)(param_1 + 6) + 0x140) + 8) +
                      *(long *)(param_2 * 0x10 + *(long *)(*(long *)(param_1 + 6) + 0x140))) -
                     local_f0);
        }
        if ((DAT_0019ec84 & 0x80) != 0) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar3 = FUN_00154e5a(local_e8,local_38);
          uVar7 = FUN_0015e548(**(undefined8 **)(param_1 + 6));
          uVar8 = ngettext("%s: File shrank by %s byte; padding with zeros",
                           "%s: File shrank by %s bytes; padding with zeros",local_e8);
          error(0,0,uVar8,uVar7,uVar3);
        }
        if (DAT_0019f8bc != '\x01') {
          FUN_00139e98(1);
        }
        uVar3 = 0;
        goto LAB_0012fe74;
      }
      memset((void *)(lVar5 + lVar6),0,0x200 - lVar6);
      local_f0 = local_f0 - lVar6;
      *(long *)(param_1 + 4) = lVar6 + *(long *)(param_1 + 4);
      FUN_0010bba7(lVar5);
    }
    uVar3 = 1;
  }
  else {
    uVar3 = 0;
  }
LAB_0012fe74:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar3;
}




// Function: sparse_extract_region @ 0x2fe94

undefined8 sparse_extract_region(undefined4 *param_1,long param_2)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  long lVar4;
  long lVar5;
  long local_28;
  
  cVar1 = FUN_0012f3b2(param_1,*(undefined8 *)
                                (param_2 * 0x10 + *(long *)(*(long *)(param_1 + 6) + 0x140)));
  if (cVar1 == '\x01') {
    local_28 = *(long *)(param_2 * 0x10 + *(long *)(*(long *)(param_1 + 6) + 0x140) + 8);
    if (local_28 == 0) {
      if ((*(char *)(param_1 + 1) != '\0') && (iVar2 = FUN_00132f52(*param_1), iVar2 != 0)) {
        FUN_0013ea7e(**(undefined8 **)(param_1 + 6));
      }
LAB_00130089:
      uVar3 = 1;
    }
    else {
      do {
        if (local_28 < 1) goto LAB_00130089;
        lVar4 = local_28;
        if (0x200 < local_28) {
          lVar4 = 0x200;
        }
        lVar5 = FUN_0010bb4b();
        if (lVar5 == 0) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar3 = gettext("Unexpected EOF in archive");
          error(0,0,uVar3);
          DAT_0019fdd0 = 2;
          return 0;
        }
        FUN_0010bba7(lVar5);
        *(long *)(param_1 + 4) = *(long *)(param_1 + 4) + 0x200;
        lVar5 = FUN_0012aa95(*param_1,lVar5,lVar4);
        local_28 = local_28 - lVar5;
        FUN_0010ad86(*(long *)(*(long *)(param_1 + 6) + 0x118) - *(long *)(param_1 + 4));
        *(long *)(param_1 + 2) = lVar5 + *(long *)(param_1 + 2);
      } while (lVar5 == lVar4);
      FUN_0013eb4b(**(undefined8 **)(param_1 + 6),lVar5,lVar4);
      uVar3 = 0;
    }
  }
  else {
    uVar3 = 0;
  }
  return uVar3;
}




// Function: sparse_dump_file @ 0x30090

undefined8 sparse_dump_file(int param_1,long param_2)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  char local_51;
  ulong local_50;
  int local_48;
  undefined1 local_44;
  long local_38;
  long local_30;
  long local_28;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0012f165(&local_48);
  if (cVar1 == '\x01') {
    local_44 = 1;
    local_48 = param_1;
    local_30 = param_2;
    local_51 = FUN_0012f9e5(&local_48);
    if ((local_51 != '\0') && (*(long *)(local_28 + 0x38) != 0)) {
      FUN_0012f2f5(&local_48);
      if (-1 < param_1) {
        FUN_0010ab09(*(undefined8 *)(local_30 + 8),*(undefined8 *)(local_30 + 0x88),
                     *(long *)(local_30 + 0x118) - local_38);
        for (local_50 = 0; (local_51 != '\0' && (local_50 < *(ulong *)(local_30 + 0x130)));
            local_50 = local_50 + 1) {
          local_51 = FUN_0012f261(&local_48,local_50);
        }
      }
    }
    FUN_001142d7(*(long *)(local_30 + 0x118) - local_38);
    cVar1 = FUN_0012f1d2(&local_48);
    if ((cVar1 == '\0') || (local_51 == '\0')) {
      uVar2 = 1;
    }
    else {
      uVar2 = 0;
    }
  }
  else {
    uVar2 = 3;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: sparse_member_p @ 0x301de

undefined8 sparse_member_p(undefined8 param_1)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined1 local_48 [24];
  undefined8 local_30;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0012f165(local_48);
  if (cVar1 == '\x01') {
    local_30 = param_1;
    uVar2 = FUN_0012f126(local_48);
  }
  else {
    uVar2 = 0;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: sparse_fixup_header @ 0x30241

undefined8 sparse_fixup_header(undefined8 param_1)

{
  char cVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  undefined1 local_48 [24];
  undefined8 local_30;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0012f165(local_48);
  if (cVar1 == '\x01') {
    local_30 = param_1;
    uVar2 = FUN_0012f373(local_48);
  }
  else {
    uVar2 = 0;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: sparse_extract_file @ 0x302a4

undefined8 sparse_extract_file(int param_1,long param_2,long *param_3)

{
  char cVar1;
  undefined8 uVar2;
  __off_t _Var3;
  long in_FS_OFFSET;
  char local_51;
  ulong local_50;
  int local_48;
  undefined1 local_44;
  undefined8 local_40;
  long local_38;
  long local_30;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0012f165(&local_48);
  if (cVar1 == '\x01') {
    local_48 = param_1;
    local_30 = param_2;
    _Var3 = lseek(param_1,0,0);
    local_44 = _Var3 == 0;
    local_40 = 0;
    local_51 = FUN_0012f334(&local_48);
    for (local_50 = 0; (local_51 != '\0' && (local_50 < *(ulong *)(local_30 + 0x130)));
        local_50 = local_50 + 1) {
      local_51 = FUN_0012f2ab(&local_48,local_50);
    }
    *param_3 = *(long *)(local_30 + 0x118) - local_38;
    cVar1 = FUN_0012f1d2(&local_48);
    if ((cVar1 == '\0') || (local_51 == '\0')) {
      uVar2 = 1;
    }
    else {
      uVar2 = 0;
    }
  }
  else {
    uVar2 = 3;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: sparse_skip_file @ 0x303bd

undefined8 sparse_skip_file(long param_1)

{
  char cVar1;
  char cVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  undefined4 local_48 [4];
  long local_38;
  long local_30;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0012f165(local_48);
  if (cVar1 == '\x01') {
    local_48[0] = 0xffffffff;
    local_30 = param_1;
    cVar1 = FUN_0012f334(local_48);
    FUN_001287c1(*(long *)(local_30 + 0x118) - local_38);
    cVar2 = FUN_0012f1d2(local_48);
    if ((cVar2 == '\0') || (cVar1 == '\0')) {
      uVar3 = 1;
    }
    else {
      uVar3 = 0;
    }
  }
  else {
    uVar3 = 3;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar3;
}




// Function: check_sparse_region @ 0x3046a

undefined8 check_sparse_region(undefined4 *param_1,long param_2,long param_3)

{
  char cVar1;
  undefined8 uVar2;
  long lVar3;
  long lVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  long local_268;
  undefined1 local_248 [32];
  undefined1 local_228 [520];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0012f3b2(param_1,param_2);
  local_268 = param_2;
  if (cVar1 == '\x01') {
    for (; local_268 < param_3; local_268 = lVar4 + local_268) {
      lVar3 = param_3 - local_268;
      if (0x200 < lVar3) {
        lVar3 = 0x200;
      }
      lVar4 = FUN_0015ec70(*param_1,local_228,lVar3);
      if (lVar4 == -1) {
        FUN_0012b2f8(**(undefined8 **)(param_1 + 6),local_268,lVar3);
        uVar2 = 0;
        goto LAB_00130635;
      }
      if (lVar4 == 0) {
        uVar2 = gettext("Size differs");
        FUN_001107be(*(undefined8 *)(param_1 + 6),uVar2);
        uVar2 = 0;
        goto LAB_00130635;
      }
      cVar1 = FUN_0012f433(local_228,lVar4);
      if (cVar1 != '\x01') {
        uVar2 = FUN_00154d1c(local_268,local_248);
        uVar5 = gettext("File fragment at %s is not a hole");
        FUN_001107be(*(undefined8 *)(param_1 + 6),uVar5,uVar2);
        uVar2 = 0;
        goto LAB_00130635;
      }
    }
    uVar2 = 1;
  }
  else {
    uVar2 = 0;
  }
LAB_00130635:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar2;
}




// Function: check_data_region @ 0x3064f

undefined8 check_data_region(undefined4 *param_1,long param_2)

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  long lVar4;
  void *__s1;
  size_t __n;
  long in_FS_OFFSET;
  long local_238;
  undefined1 local_218 [520];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  cVar1 = FUN_0012f3b2(param_1,*(undefined8 *)
                                (param_2 * 0x10 + *(long *)(*(long *)(param_1 + 6) + 0x140)));
  if (cVar1 == '\x01') {
    local_238 = *(long *)(param_2 * 0x10 + *(long *)(*(long *)(param_1 + 6) + 0x140) + 8);
    FUN_0010ad86(*(long *)(*(long *)(param_1 + 6) + 0x118) - *(long *)(param_1 + 4));
    do {
      if (local_238 < 1) {
        uVar3 = 1;
        goto LAB_0013095c;
      }
      lVar4 = local_238;
      if (0x200 < local_238) {
        lVar4 = 0x200;
      }
      __s1 = (void *)FUN_0010bb4b();
      if (__s1 == (void *)0x0) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar3 = gettext("Unexpected EOF in archive");
        error(0,0,uVar3);
        DAT_0019fdd0 = 2;
        uVar3 = 0;
        goto LAB_0013095c;
      }
      FUN_0010bba7(__s1);
      *(long *)(param_1 + 4) = *(long *)(param_1 + 4) + 0x200;
      __n = FUN_0015ec70(*param_1,local_218,lVar4);
      if (__n == 0xffffffffffffffff) {
        FUN_0012b2f8(**(undefined8 **)(param_1 + 6),
                     (*(long *)(param_2 * 0x10 + *(long *)(*(long *)(param_1 + 6) + 0x140) + 8) +
                     *(long *)(param_2 * 0x10 + *(long *)(*(long *)(param_1 + 6) + 0x140))) -
                     local_238,lVar4);
        uVar3 = 0;
        goto LAB_0013095c;
      }
      if (__n == 0) {
        uVar3 = gettext("Size differs");
        FUN_001107be(&DAT_0019fa20,uVar3);
        uVar3 = 0;
        goto LAB_0013095c;
      }
      local_238 = local_238 - __n;
      FUN_0010ad86(*(long *)(*(long *)(param_1 + 6) + 0x118) - *(long *)(param_1 + 4));
      iVar2 = memcmp(__s1,local_218,__n);
    } while (iVar2 == 0);
    uVar3 = gettext("Contents differ");
    FUN_001107be(*(undefined8 *)(param_1 + 6),uVar3);
    uVar3 = 0;
  }
  else {
    uVar3 = 0;
  }
LAB_0013095c:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar3;
}




// Function: sparse_diff_file @ 0x30972

char sparse_diff_file(undefined4 param_1,long param_2)

{
  char cVar1;
  long in_FS_OFFSET;
  char local_59;
  ulong local_58;
  long local_50;
  undefined4 local_48;
  undefined1 local_44;
  long local_38;
  long local_30;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_50 = 0;
  cVar1 = FUN_0012f165(&local_48);
  if (cVar1 == '\x01') {
    local_44 = 1;
    local_48 = param_1;
    local_30 = param_2;
    local_59 = FUN_0012f334(&local_48);
    FUN_0010ad2b(param_2);
    for (local_58 = 0; (local_59 != '\0' && (local_58 < *(ulong *)(local_30 + 0x130)));
        local_58 = local_58 + 1) {
      cVar1 = FUN_0013046a(&local_48,local_50,
                           *(undefined8 *)(local_58 * 0x10 + *(long *)(local_30 + 0x140)));
      if (cVar1 == '\0') {
LAB_00130a45:
        local_59 = '\0';
      }
      else {
        cVar1 = FUN_0013064f(&local_48,local_58);
        if (cVar1 == '\0') goto LAB_00130a45;
        local_59 = '\x01';
      }
      local_50 = *(long *)(local_58 * 0x10 + *(long *)(local_30 + 0x140) + 8) +
                 *(long *)(local_58 * 0x10 + *(long *)(local_30 + 0x140));
    }
    if (local_59 != '\x01') {
      FUN_001287c1(*(long *)(local_30 + 0x118) - local_38);
    }
    FUN_0010ad66();
    FUN_0012f1d2(&local_48);
  }
  else {
    local_59 = '\0';
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_59;
}




// Function: oldgnu_sparse_member_p @ 0x30afb

bool oldgnu_sparse_member_p(void)

{
  return *(char *)(DAT_0019f460 + 0x9c) == 'S';
}




// Function: oldgnu_add_sparse @ 0x30b1c

undefined8 oldgnu_add_sparse(long param_1,long param_2)

{
  undefined8 uVar1;
  long in_FS_OFFSET;
  long local_28;
  long local_20;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (*(char *)(param_2 + 0xc) == '\0') {
    uVar1 = 1;
  }
  else {
    local_28 = FUN_0012798b(param_2,0xc);
    local_20 = FUN_0012798b(param_2 + 0xc,0xc);
    if ((((local_28 < 0) || (local_20 < 0)) || (SCARRY8(local_20,local_28))) ||
       ((*(long *)(*(long *)(param_1 + 0x18) + 0x88) < local_20 + local_28 ||
        (*(long *)(*(long *)(param_1 + 0x18) + 0x118) < 0)))) {
      uVar1 = 2;
    }
    else {
      FUN_0012f477(*(undefined8 *)(param_1 + 0x18),&local_28);
      uVar1 = 0;
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar1;
}




// Function: oldgnu_fixup_header @ 0x30c22

ulong oldgnu_fixup_header(long param_1)

{
  ulong uVar1;
  ulong uVar2;
  
  uVar1 = FUN_0012798b(DAT_0019f460 + 0x1e3,0xc);
  *(undefined8 *)(*(long *)(param_1 + 0x18) + 0x118) =
       *(undefined8 *)(*(long *)(param_1 + 0x18) + 0x88);
  uVar2 = uVar1;
  if ((long)uVar1 < 0) {
    uVar2 = 0;
  }
  *(ulong *)(*(long *)(param_1 + 0x18) + 0x88) = uVar2;
  return ~uVar1 >> 0x3f;
}




// Function: oldgnu_get_sparse_info @ 0x30c9a

undefined8 oldgnu_get_sparse_info(long param_1)

{
  char cVar1;
  long lVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  int local_2c;
  ulong local_28;
  
  lVar2 = DAT_0019f460;
  *(undefined8 *)(*(long *)(param_1 + 0x18) + 0x130) = 0;
  local_28 = 0;
  while ((local_28 < 4 &&
         (local_2c = FUN_00130b1c(param_1,lVar2 + local_28 * 0x18 + 0x182), local_2c == 0))) {
    local_28 = local_28 + 1;
  }
  cVar1 = *(char *)(lVar2 + 0x1e2);
  while ((local_2c == 0 && (cVar1 != '\0'))) {
    lVar2 = FUN_0010bb4b();
    if (lVar2 == 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar4 = gettext("Unexpected EOF in archive");
      error(0,0,uVar4);
      DAT_0019fdd0 = 2;
      return 0;
    }
    FUN_0010bba7(lVar2);
    local_28 = 0;
    while ((local_28 < 0x15 && (local_2c == 0))) {
      local_2c = FUN_00130b1c(param_1,lVar2 + local_28 * 0x18);
      local_28 = local_28 + 1;
    }
    cVar1 = *(char *)(lVar2 + 0x1f8);
  }
  if (local_2c == 2) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar4 = **(undefined8 **)(param_1 + 0x18);
    uVar3 = gettext("%s: invalid sparse archive member");
    error(0,0,uVar3,uVar4);
    DAT_0019fdd0 = 2;
    uVar4 = 0;
  }
  else {
    uVar4 = 1;
  }
  return uVar4;
}




// Function: oldgnu_store_sparse_info @ 0x30e70

void oldgnu_store_sparse_info(long param_1,ulong *param_2,long param_3,long param_4)

{
  undefined8 local_28;
  undefined8 local_20;
  
  local_28 = param_4;
  local_20 = param_3;
  while ((*param_2 < *(ulong *)(*(long *)(param_1 + 0x18) + 0x130) && (local_28 != 0))) {
    FUN_00112a9a(*(undefined8 *)(*param_2 * 0x10 + *(long *)(*(long *)(param_1 + 0x18) + 0x140)),
                 local_20,0xc);
    FUN_00112a9a(*(undefined8 *)(*param_2 * 0x10 + *(long *)(*(long *)(param_1 + 0x18) + 0x140) + 8)
                 ,local_20 + 0xc,0xc);
    local_28 = local_28 + -1;
    local_20 = local_20 + 0x18;
    *param_2 = *param_2 + 1;
  }
  return;
}




// Function: oldgnu_dump_header @ 0x30f43

undefined8 oldgnu_dump_header(long param_1)

{
  long in_FS_OFFSET;
  ulong local_28;
  undefined8 local_20;
  void *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_20 = FUN_0010bacd();
  local_18 = (void *)FUN_0011381d(*(undefined8 *)(param_1 + 0x18));
  *(undefined1 *)((long)local_18 + 0x9c) = 0x53;
  if (4 < *(ulong *)(*(long *)(param_1 + 0x18) + 0x130)) {
    *(undefined1 *)((long)local_18 + 0x1e2) = 1;
  }
  FUN_00112a9a(*(undefined8 *)(*(long *)(param_1 + 0x18) + 0x88),(long)local_18 + 0x1e3,0xc);
  FUN_00112a9a(*(undefined8 *)(*(long *)(param_1 + 0x18) + 0x118),(long)local_18 + 0x7c,0xc);
  local_28 = 0;
  FUN_00130e70(param_1,&local_28,(long)local_18 + 0x182,4);
  *(bool *)((long)local_18 + 0x1e2) = local_28 < *(ulong *)(*(long *)(param_1 + 0x18) + 0x130);
  FUN_0011422e(*(undefined8 *)(param_1 + 0x18),local_18,local_20);
  while (local_28 < *(ulong *)(*(long *)(param_1 + 0x18) + 0x130)) {
    local_18 = (void *)FUN_0010bb4b();
    memset(local_18,0,0x200);
    FUN_00130e70(param_1,&local_28,local_18,0x15);
    if (local_28 < *(ulong *)(*(long *)(param_1 + 0x18) + 0x130)) {
      *(undefined1 *)((long)local_18 + 0x1f8) = 1;
    }
    FUN_0010bba7(local_18);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return 1;
}




// Function: star_sparse_member_p @ 0x310ff

bool star_sparse_member_p(void)

{
  return *(char *)(DAT_0019f460 + 0x9c) == 'S';
}




// Function: star_fixup_header @ 0x31120

ulong star_fixup_header(long param_1)

{
  ulong uVar1;
  ulong uVar2;
  
  uVar1 = FUN_0012798b(DAT_0019f460 + 0x1c4,0xc);
  *(undefined8 *)(*(long *)(param_1 + 0x18) + 0x118) =
       *(undefined8 *)(*(long *)(param_1 + 0x18) + 0x88);
  uVar2 = uVar1;
  if ((long)uVar1 < 0) {
    uVar2 = 0;
  }
  *(ulong *)(*(long *)(param_1 + 0x18) + 0x88) = uVar2;
  return ~uVar1 >> 0x3f;
}




// Function: star_get_sparse_info @ 0x31198

undefined8 star_get_sparse_info(long param_1)

{
  char cVar1;
  long lVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  int local_2c;
  ulong local_28;
  
  lVar2 = DAT_0019f460;
  local_2c = 0;
  *(undefined8 *)(*(long *)(param_1 + 0x18) + 0x130) = 0;
  if ((*(char *)(lVar2 + 0x159) == '\0') && (*(char *)(lVar2 + 0x16e) != '\0')) {
    local_28 = 0;
    while ((local_28 < 4 &&
           (local_2c = FUN_00130b1c(param_1,lVar2 + local_28 * 0x18 + 0x164), local_2c == 0))) {
      local_28 = local_28 + 1;
    }
    cVar1 = *(char *)(lVar2 + 0x163);
  }
  else {
    cVar1 = '\x01';
  }
  while ((local_2c == 0 && (cVar1 != '\0'))) {
    lVar2 = FUN_0010bb4b();
    if (lVar2 == 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar4 = gettext("Unexpected EOF in archive");
      error(0,0,uVar4);
      DAT_0019fdd0 = 2;
      return 0;
    }
    FUN_0010bba7(lVar2);
    local_28 = 0;
    while ((local_28 < 0x15 && (local_2c == 0))) {
      local_2c = FUN_00130b1c(param_1,lVar2 + local_28 * 0x18);
      local_28 = local_28 + 1;
    }
    *(long *)(param_1 + 0x10) = *(long *)(param_1 + 0x10) + 0x200;
    cVar1 = *(char *)(lVar2 + 0x1f8);
  }
  if (local_2c == 2) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar4 = **(undefined8 **)(param_1 + 0x18);
    uVar3 = gettext("%s: invalid sparse archive member");
    error(0,0,uVar3,uVar4);
    DAT_0019fdd0 = 2;
    uVar4 = 0;
  }
  else {
    uVar4 = 1;
  }
  return uVar4;
}




// Function: pax_sparse_member_p @ 0x313b3

undefined4 pax_sparse_member_p(long param_1)

{
  undefined4 uVar1;
  
  if ((*(long *)(*(long *)(param_1 + 0x18) + 0x130) == 0) &&
     (*(int *)(*(long *)(param_1 + 0x18) + 0x124) == 0)) {
    uVar1 = 0;
  }
  else {
    uVar1 = 1;
  }
  return uVar1;
}




// Function: pax_start_header @ 0x313f6

undefined8 pax_start_header(long param_1)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  uVar1 = *(undefined8 *)(param_1 + 0x88);
  *(undefined8 *)(param_1 + 0x88) = *(undefined8 *)(param_1 + 0x118);
  uVar2 = FUN_0011381d(param_1);
  *(undefined8 *)(param_1 + 0x88) = uVar1;
  return uVar2;
}




// Function: pax_dump_header_0 @ 0x31450

undefined8 pax_dump_header_0(long param_1)

{
  long lVar1;
  char cVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  ulong local_60;
  long local_58;
  undefined8 local_50;
  long local_48;
  undefined8 local_40;
  undefined1 local_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_50 = FUN_0010bacd();
  local_48 = *(long *)(*(long *)(param_1 + 0x18) + 0x140);
  local_58 = 0;
  FUN_0011e1d3("GNU.sparse.size",*(undefined8 *)(param_1 + 0x18),0);
  FUN_0011e1d3("GNU.sparse.numblocks",*(undefined8 *)(param_1 + 0x18),0);
  cVar2 = FUN_0011c63f("GNU.sparse.map");
  if ((cVar2 == '\0') && (DAT_0019f980 != 0)) {
    FUN_0011e1d3("GNU.sparse.name",*(undefined8 *)(param_1 + 0x18),0);
    local_58 = *(long *)(*(long *)(param_1 + 0x18) + 8);
    lVar1 = *(long *)(param_1 + 0x18);
    uVar3 = FUN_0011cd02(*(undefined8 *)(param_1 + 0x18),"%d/GNUSparseFile.%p/%f",0);
    *(undefined8 *)(lVar1 + 8) = uVar3;
    FUN_0011e848(*(long *)(param_1 + 0x18) + 0x168);
    for (local_60 = 0; local_60 < *(ulong *)(*(long *)(param_1 + 0x18) + 0x130);
        local_60 = local_60 + 1) {
      if (local_60 != 0) {
        FUN_0011e863(*(long *)(param_1 + 0x18) + 0x168,&DAT_00183c63);
      }
      uVar3 = FUN_00154e5a(*(undefined8 *)(local_48 + local_60 * 0x10),local_38);
      FUN_0011e863(*(long *)(param_1 + 0x18) + 0x168,uVar3);
      FUN_0011e863(*(long *)(param_1 + 0x18) + 0x168,&DAT_00183c63);
      uVar3 = FUN_00154e5a(*(undefined8 *)(local_48 + local_60 * 0x10 + 8),local_38);
      FUN_0011e863(*(long *)(param_1 + 0x18) + 0x168,uVar3);
    }
    cVar2 = FUN_0011e8dc(*(long *)(param_1 + 0x18) + 0x168,"GNU.sparse.map");
    if (cVar2 != '\x01') {
      free(*(void **)(*(long *)(param_1 + 0x18) + 8));
      *(long *)(*(long *)(param_1 + 0x18) + 8) = local_58;
      uVar3 = 0;
      goto LAB_00131784;
    }
  }
  else {
    for (local_60 = 0; local_60 < *(ulong *)(*(long *)(param_1 + 0x18) + 0x130);
        local_60 = local_60 + 1) {
      FUN_0011e1d3("GNU.sparse.offset",*(undefined8 *)(param_1 + 0x18),&local_60);
      FUN_0011e1d3("GNU.sparse.numbytes",*(undefined8 *)(param_1 + 0x18),&local_60);
    }
  }
  local_40 = FUN_001313f6(*(undefined8 *)(param_1 + 0x18));
  FUN_0011422e(*(undefined8 *)(param_1 + 0x18),local_40,local_50);
  if (local_58 != 0) {
    free(*(void **)(*(long *)(param_1 + 0x18) + 8));
    *(long *)(*(long *)(param_1 + 0x18) + 8) = local_58;
  }
  uVar3 = 1;
LAB_00131784:
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar3;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: pax_dump_header_1 @ 0x3179e

undefined8 pax_dump_header_1(long param_1)

{
  long lVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  char *pcVar4;
  size_t sVar5;
  size_t sVar6;
  long lVar7;
  undefined8 uVar8;
  long in_FS_OFFSET;
  char *local_d8;
  char *local_d0;
  ulong local_c8;
  long local_c0;
  char *local_b8;
  char *local_b0;
  char *local_a8;
  char *local_a0;
  char *local_98;
  char *local_90;
  char *local_88;
  char *local_80;
  char *local_78;
  char *local_70;
  char *local_68;
  char *local_60;
  undefined1 local_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  uVar3 = FUN_0010bacd();
  lVar1 = *(long *)(*(long *)(param_1 + 0x18) + 0x140);
  uVar2 = *(undefined8 *)(*(long *)(param_1 + 0x18) + 8);
  pcVar4 = (char *)FUN_00154e5a(*(undefined8 *)(*(long *)(param_1 + 0x18) + 0x130),local_38);
  sVar5 = strlen(pcVar4);
  local_c0 = sVar5 + 1;
  for (local_c8 = 0; local_c8 < *(ulong *)(*(long *)(param_1 + 0x18) + 0x130);
      local_c8 = local_c8 + 1) {
    pcVar4 = (char *)FUN_00154e5a(*(undefined8 *)(lVar1 + local_c8 * 0x10),local_38);
    sVar5 = strlen(pcVar4);
    pcVar4 = (char *)FUN_00154e5a(*(undefined8 *)(lVar1 + local_c8 * 0x10 + 8),local_38);
    sVar6 = strlen(pcVar4);
    local_c0 = sVar6 + sVar5 + local_c0 + 2;
  }
  lVar7 = local_c0 + 0x1ff;
  if (local_c0 + 0x1ff < 0) {
    lVar7 = local_c0 + 0x3fe;
  }
  *(long *)(*(long *)(param_1 + 0x18) + 0x118) =
       *(long *)(*(long *)(param_1 + 0x18) + 0x118) + (lVar7 >> 9) * 0x200;
  *(long *)(param_1 + 0x10) = *(long *)(param_1 + 0x10) + (lVar7 >> 9) * 0x200;
  FUN_0011e1d3("GNU.sparse.major",*(undefined8 *)(param_1 + 0x18),0);
  FUN_0011e1d3("GNU.sparse.minor",*(undefined8 *)(param_1 + 0x18),0);
  FUN_0011e1d3("GNU.sparse.name",*(undefined8 *)(param_1 + 0x18),0);
  FUN_0011e1d3("GNU.sparse.realsize",*(undefined8 *)(param_1 + 0x18),0);
  lVar7 = *(long *)(param_1 + 0x18);
  uVar8 = FUN_0011cd02(*(undefined8 *)(param_1 + 0x18),"%d/GNUSparseFile.%p/%f",0);
  *(undefined8 *)(lVar7 + 8) = uVar8;
  sVar5 = strlen(*(char **)(*(long *)(param_1 + 0x18) + 8));
  if (100 < sVar5) {
    *(undefined1 *)(*(long *)(*(long *)(param_1 + 0x18) + 8) + 100) = 0;
  }
  uVar8 = FUN_001313f6(*(undefined8 *)(param_1 + 0x18));
  FUN_0011422e(*(undefined8 *)(param_1 + 0x18),uVar8,uVar3);
  free(*(void **)(*(long *)(param_1 + 0x18) + 8));
  *(undefined8 *)(*(long *)(param_1 + 0x18) + 8) = uVar2;
  local_d8 = (char *)FUN_0010bb4b();
  local_b0 = (char *)FUN_00154e5a(*(undefined8 *)(*(long *)(param_1 + 0x18) + 0x130),local_38);
  local_b8 = local_d8 + 0x200;
  local_d0 = local_d8;
  while (*local_b0 != '\0') {
    if (local_d0 == local_b8) {
      FUN_0010bba7(local_d8);
      local_d8 = (char *)FUN_0010bb4b();
      local_b8 = local_d8 + 0x200;
      local_d0 = local_d8;
    }
    *local_d0 = *local_b0;
    local_d0 = local_d0 + 1;
    local_b0 = local_b0 + 1;
  }
  local_a8 = local_d8 + 0x200;
  local_a0 = "\n";
  while (*local_a0 != '\0') {
    if (local_d0 == local_a8) {
      FUN_0010bba7(local_d8);
      local_d8 = (char *)FUN_0010bb4b();
      local_a8 = local_d8 + 0x200;
      local_d0 = local_d8;
    }
    *local_d0 = *local_a0;
    local_d0 = local_d0 + 1;
    local_a0 = local_a0 + 1;
  }
  for (local_c8 = 0; local_c8 < *(ulong *)(*(long *)(param_1 + 0x18) + 0x130);
      local_c8 = local_c8 + 1) {
    local_90 = (char *)FUN_00154e5a(*(undefined8 *)(lVar1 + local_c8 * 0x10),local_38);
    local_98 = local_d8 + 0x200;
    while (*local_90 != '\0') {
      if (local_d0 == local_98) {
        FUN_0010bba7(local_d8);
        local_d8 = (char *)FUN_0010bb4b();
        local_98 = local_d8 + 0x200;
        local_d0 = local_d8;
      }
      *local_d0 = *local_90;
      local_d0 = local_d0 + 1;
      local_90 = local_90 + 1;
    }
    local_88 = local_d8 + 0x200;
    local_80 = "\n";
    while (*local_80 != '\0') {
      if (local_d0 == local_88) {
        FUN_0010bba7(local_d8);
        local_d8 = (char *)FUN_0010bb4b();
        local_88 = local_d8 + 0x200;
        local_d0 = local_d8;
      }
      *local_d0 = *local_80;
      local_d0 = local_d0 + 1;
      local_80 = local_80 + 1;
    }
    local_70 = (char *)FUN_00154e5a(*(undefined8 *)(lVar1 + local_c8 * 0x10 + 8),local_38);
    local_78 = local_d8 + 0x200;
    while (*local_70 != '\0') {
      if (local_d0 == local_78) {
        FUN_0010bba7(local_d8);
        local_d8 = (char *)FUN_0010bb4b();
        local_78 = local_d8 + 0x200;
        local_d0 = local_d8;
      }
      *local_d0 = *local_70;
      local_d0 = local_d0 + 1;
      local_70 = local_70 + 1;
    }
    local_68 = local_d8 + 0x200;
    local_60 = "\n";
    while (*local_60 != '\0') {
      if (local_d0 == local_68) {
        FUN_0010bba7(local_d8);
        local_d8 = (char *)FUN_0010bb4b();
        local_68 = local_d8 + 0x200;
        local_d0 = local_d8;
      }
      *local_d0 = *local_60;
      local_d0 = local_d0 + 1;
      local_60 = local_60 + 1;
    }
  }
  memset(local_d0,0,0x200 - ((long)local_d0 - (long)local_d8));
  FUN_0010bba7(local_d8);
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return 1;
}




// Function: pax_dump_header @ 0x31fd5

bool pax_dump_header(long param_1)

{
  char cVar1;
  
  *(undefined4 *)(*(long *)(param_1 + 0x18) + 0x124) = DAT_0019f97c;
  *(undefined4 *)(*(long *)(param_1 + 0x18) + 0x128) = DAT_0019f980;
  if (*(int *)(*(long *)(param_1 + 0x18) + 0x124) == 0) {
    cVar1 = FUN_00131450(param_1);
  }
  else {
    cVar1 = FUN_0013179e(param_1);
  }
  return cVar1 != '\0';
}




// Function: decode_num @ 0x3204b

undefined8 decode_num(uintmax_t *param_1,char *param_2,ulong param_3)

{
  undefined8 uVar1;
  int *piVar2;
  long in_FS_OFFSET;
  char *local_20;
  uintmax_t local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if ((int)*param_2 - 0x30U < 10) {
    piVar2 = __errno_location();
    *piVar2 = 0;
    local_18 = strtoumax(param_2,&local_20,10);
    if (local_18 <= param_3) {
      piVar2 = __errno_location();
      if ((*piVar2 != 0x22) && (*local_20 == '\0')) {
        *param_1 = local_18;
        uVar1 = 1;
        goto LAB_001320ea;
      }
    }
    uVar1 = 0;
  }
  else {
    uVar1 = 0;
  }
LAB_001320ea:
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar1;
}




// Function: pax_decode_header @ 0x32100

undefined8 pax_decode_header(long param_1)

{
  char *pcVar1;
  char cVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long lVar5;
  long in_FS_OFFSET;
  undefined8 local_a0;
  char *local_98;
  char *local_90;
  ulong local_88;
  char *local_80;
  char *local_78;
  char *local_70;
  char *local_68;
  char *local_60;
  char *local_58;
  long local_50;
  undefined8 local_48;
  undefined8 local_40;
  char local_38 [20];
  char acStack_24 [4];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (*(int *)(*(long *)(param_1 + 0x18) + 0x124) != 0) {
    local_50 = FUN_0010bacd();
    FUN_0010bba7(DAT_0019f460);
    local_98 = (char *)FUN_0010bb4b();
    if (local_98 == (char *)0x0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar3 = gettext("Unexpected EOF in archive");
      error(0,0,uVar3);
                    /* WARNING: Subroutine does not return */
      FUN_001177f4();
    }
    local_80 = local_98 + 0x200;
    local_90 = local_98;
    pcVar1 = local_38;
    do {
      local_78 = pcVar1;
      if (local_78 == acStack_24) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar3 = **(undefined8 **)(param_1 + 0x18);
        uVar4 = gettext("%s: numeric overflow in sparse archive member");
        error(0,0,uVar4,uVar3);
        DAT_0019fdd0 = 2;
        uVar3 = 0;
        goto LAB_00132854;
      }
      if (local_90 == local_80) {
        FUN_0010bba7(local_98);
        local_98 = (char *)FUN_0010bb4b();
        if (local_98 == (char *)0x0) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar3 = gettext("Unexpected EOF in archive");
          error(0,0,uVar3);
                    /* WARNING: Subroutine does not return */
          FUN_001177f4();
        }
        local_80 = local_98 + 0x200;
        local_90 = local_98;
      }
      pcVar1 = local_90 + 1;
      *local_78 = *local_90;
      local_90 = pcVar1;
      pcVar1 = local_78 + 1;
    } while (*local_78 != '\n');
    *local_78 = '\0';
    local_78 = local_78 + 1;
    cVar2 = FUN_0013204b(&local_a0,local_38,0xffffffffffffffff);
    if (cVar2 != '\x01') {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar3 = **(undefined8 **)(param_1 + 0x18);
      uVar4 = gettext("%s: malformed sparse archive member");
      error(0,0,uVar4,uVar3);
      DAT_0019fdd0 = 2;
      uVar3 = 0;
      goto LAB_00132854;
    }
    *(undefined8 *)(*(long *)(param_1 + 0x18) + 0x138) = local_a0;
    lVar5 = *(long *)(param_1 + 0x18);
    uVar3 = FUN_00160c29(*(undefined8 *)(*(long *)(param_1 + 0x18) + 0x138),0x10);
    *(undefined8 *)(lVar5 + 0x140) = uVar3;
    *(undefined8 *)(*(long *)(param_1 + 0x18) + 0x130) = 0;
    for (local_88 = 0; local_88 < *(ulong *)(*(long *)(param_1 + 0x18) + 0x138);
        local_88 = local_88 + 1) {
      local_70 = local_98 + 0x200;
      pcVar1 = local_38;
      do {
        local_68 = pcVar1;
        if (local_68 == acStack_24) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar3 = **(undefined8 **)(param_1 + 0x18);
          uVar4 = gettext("%s: numeric overflow in sparse archive member");
          error(0,0,uVar4,uVar3);
          goto LAB_001327f9;
        }
        if (local_90 == local_70) {
          FUN_0010bba7(local_98);
          local_98 = (char *)FUN_0010bb4b();
          if (local_98 == (char *)0x0) {
            if (DAT_0019fdc8 != (code *)0x0) {
              (*DAT_0019fdc8)();
            }
            uVar3 = gettext("Unexpected EOF in archive");
            error(0,0,uVar3);
                    /* WARNING: Subroutine does not return */
            FUN_001177f4();
          }
          local_70 = local_98 + 0x200;
          local_90 = local_98;
        }
        pcVar1 = local_90 + 1;
        *local_68 = *local_90;
        local_90 = pcVar1;
        pcVar1 = local_68 + 1;
      } while (*local_68 != '\n');
      *local_68 = '\0';
      local_68 = local_68 + 1;
      cVar2 = FUN_0013204b(&local_a0,local_38,0x7fffffffffffffff);
      if (cVar2 != '\x01') {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar3 = **(undefined8 **)(param_1 + 0x18);
        uVar4 = gettext("%s: malformed sparse archive member");
        error(0,0,uVar4,uVar3);
LAB_001327f9:
        DAT_0019fdd0 = 2;
        uVar3 = 0;
        goto LAB_00132854;
      }
      local_48 = local_a0;
      local_60 = local_98 + 0x200;
      pcVar1 = local_38;
      do {
        local_58 = pcVar1;
        if (local_58 == acStack_24) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar3 = **(undefined8 **)(param_1 + 0x18);
          uVar4 = gettext("%s: numeric overflow in sparse archive member");
          error(0,0,uVar4,uVar3);
          goto LAB_001327f9;
        }
        if (local_90 == local_60) {
          FUN_0010bba7(local_98);
          local_98 = (char *)FUN_0010bb4b();
          if (local_98 == (char *)0x0) {
            if (DAT_0019fdc8 != (code *)0x0) {
              (*DAT_0019fdc8)();
            }
            uVar3 = gettext("Unexpected EOF in archive");
            error(0,0,uVar3);
                    /* WARNING: Subroutine does not return */
            FUN_001177f4();
          }
          local_60 = local_98 + 0x200;
          local_90 = local_98;
        }
        pcVar1 = local_90 + 1;
        *local_58 = *local_90;
        local_90 = pcVar1;
        pcVar1 = local_58 + 1;
      } while (*local_58 != '\n');
      *local_58 = '\0';
      local_58 = local_58 + 1;
      cVar2 = FUN_0013204b(&local_a0,local_38,0x7fffffffffffffff);
      if (cVar2 != '\x01') {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar3 = **(undefined8 **)(param_1 + 0x18);
        uVar4 = gettext("%s: malformed sparse archive member");
        error(0,0,uVar4,uVar3);
        goto LAB_001327f9;
      }
      local_40 = local_a0;
      FUN_0012f477(*(undefined8 *)(param_1 + 0x18),&local_48);
    }
    FUN_0010bba7(local_98);
    lVar5 = FUN_0010bacd();
    *(long *)(param_1 + 0x10) = (lVar5 - local_50) * 0x200 + *(long *)(param_1 + 0x10);
  }
  uVar3 = 1;
LAB_00132854:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar3;
}




// Function: find_compression_suffix @ 0x3286e

undefined ** find_compression_suffix(char *param_1,long *param_2)

{
  int iVar1;
  char *pcVar2;
  undefined *__n;
  size_t sVar3;
  undefined **local_20;
  
  pcVar2 = strrchr(param_1,0x2e);
  if (pcVar2 != (char *)0x0) {
    __n = (undefined *)strlen(pcVar2 + 1);
    for (local_20 = &PTR_DAT_0019c960; *local_20 != (undefined *)0x0; local_20 = local_20 + 3) {
      if ((__n == local_20[1]) && (iVar1 = memcmp(*local_20,pcVar2 + 1,(size_t)__n), iVar1 == 0)) {
        if (param_2 == (long *)0x0) {
          return local_20;
        }
        sVar3 = strlen(param_1);
        *param_2 = (sVar3 - (long)__n) + -1;
        return local_20;
      }
    }
  }
  return (undefined **)0x0;
}




// Function: find_compression_program @ 0x32930

undefined8 find_compression_program(undefined8 param_1,undefined8 param_2)

{
  long lVar1;
  
  lVar1 = FUN_0013286e(param_1,0);
  if (lVar1 != 0) {
    param_2 = *(undefined8 *)(lVar1 + 0x10);
  }
  return param_2;
}




// Function: set_compression_program_by_suffix @ 0x32970

void set_compression_program_by_suffix(undefined8 param_1,undefined8 param_2)

{
  long lVar1;
  
  lVar1 = FUN_00132930(param_1,param_2);
  if (lVar1 != 0) {
    DAT_0019f898 = lVar1;
  }
  return;
}




// Function: strip_compression_suffix @ 0x329b0

void * strip_compression_suffix(void *param_1)

{
  int iVar1;
  void *pvVar2;
  long in_FS_OFFSET;
  ulong local_28;
  void *local_20;
  undefined8 *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_20 = (void *)0x0;
  local_18 = (undefined8 *)FUN_0013286e(param_1,&local_28);
  pvVar2 = local_20;
  if (local_18 != (undefined8 *)0x0) {
    if (((4 < local_28) &&
        (iVar1 = strncmp((char *)((long)param_1 + (local_28 - 4)),".tar",4), iVar1 == 0)) &&
       (*(char *)*local_18 != 't')) {
      local_28 = local_28 - 4;
    }
    if (local_28 == 0) {
      pvVar2 = (void *)0x0;
    }
    else {
      local_20 = (void *)FUN_00160b5b(local_28 + 1);
      memcpy(local_20,param_1,local_28);
      *(undefined1 *)((long)local_20 + local_28) = 0;
      pvVar2 = local_20;
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return pvVar2;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: xexec @ 0x32aaa

undefined8 xexec(undefined8 param_1)

{
  int iVar1;
  undefined4 extraout_var;
  long in_FS_OFFSET;
  char *local_38;
  undefined *local_30;
  undefined8 local_28;
  undefined8 local_20;
  undefined8 local_10;
  
  local_10 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  local_38 = "/bin/sh";
  local_30 = &DAT_00183f78;
  local_20 = 0;
  local_28 = param_1;
  execv("/bin/sh",&local_38);
  FUN_0013e251(param_1);
  iVar1 = fstat(DAT_0019f9cc,(stat *)&DAT_0019f7a0);
  return CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),iVar1 == 0);
}




// Function: sys_get_archive_stat @ 0x32b11

undefined8 sys_get_archive_stat(void)

{
  int iVar1;
  undefined4 extraout_var;
  
  iVar1 = fstat(DAT_0019f9cc,(stat *)&DAT_0019f7a0);
  return CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),iVar1 == 0);
}




// Function: sys_file_is_archive @ 0x32b37

undefined4 sys_file_is_archive(long param_1)

{
  undefined4 uVar1;
  
  if (((DAT_0019fbf8 == 0) || (*(long *)(param_1 + 0x58) != DAT_0019fbf8)) ||
     (*(long *)(param_1 + 0x60) != DAT_0019fc00)) {
    uVar1 = 0;
  }
  else {
    uVar1 = 1;
  }
  return uVar1;
}




// Function: sys_save_archive_dev_ino @ 0x32b88

void sys_save_archive_dev_ino(void)

{
  if ((DAT_0019f9cc < 0x40000000) && ((DAT_0019f7b8 & 0xf000) == 0x8000)) {
    DAT_0019fbf8 = DAT_0019f7a0;
    DAT_0019fc00 = DAT_0019f7a8;
  }
  else {
    DAT_0019fbf8 = 0;
  }
  return;
}




// Function: sys_detect_dev_null_output @ 0x32bdc

void sys_detect_dev_null_output(void)

{
  int iVar1;
  long in_FS_OFFSET;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = strcmp((char *)*DAT_0019fbd8,"/dev/null");
  if (iVar1 == 0) {
LAB_00132c7c:
    DAT_0019f9d0 = 1;
  }
  else {
    if ((DAT_0019f9cc < 0x40000000) && ((DAT_0019f7b8 & 0xf000) == 0x2000)) {
      iVar1 = stat("/dev/null",&local_a8);
      if ((iVar1 == 0) && ((DAT_0019f7a0 == local_a8.st_dev && (DAT_0019f7a8 == local_a8.st_ino))))
      goto LAB_00132c7c;
    }
    DAT_0019f9d0 = 0;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: sys_wait_for_child @ 0x32ca8

void sys_wait_for_child(int param_1,char param_2)

{
  __pid_t _Var1;
  int *piVar2;
  undefined8 uVar3;
  long in_FS_OFFSET;
  uint local_28;
  uint local_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 != 0) {
    do {
      _Var1 = waitpid(param_1,(int *)&local_28,0);
      if (_Var1 != -1) goto LAB_00132d10;
      piVar2 = __errno_location();
    } while (*piVar2 == 4);
    FUN_0013eaf9(DAT_0019f898);
LAB_00132d10:
    if ((char)(((byte)local_28 & 0x7f) + 1) >> 1 < '\x01') {
      if (((int)local_28 >> 8 & 0xffU) != 0) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar3 = gettext("Child returned status %d");
        error(0,0,uVar3,(int)local_28 >> 8 & 0xff);
                    /* WARNING: Subroutine does not return */
        FUN_001177f4();
      }
    }
    else {
      local_24 = local_28 & 0x7f;
      if ((param_2 != '\0') || (local_24 != 0xd)) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar3 = gettext("Child died with signal %d");
        error(0,0,uVar3,local_24);
                    /* WARNING: Subroutine does not return */
        FUN_001177f4();
      }
    }
  }
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sys_spawn_shell @ 0x32df0

void sys_spawn_shell(void)

{
  __pid_t _Var1;
  int *piVar2;
  long in_FS_OFFSET;
  int local_20;
  int local_1c;
  char *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_18 = getenv("SHELL");
  if (local_18 == (char *)0x0) {
    local_18 = "/bin/sh";
  }
  local_1c = FUN_0012b537();
  if (local_1c != 0) goto LAB_00132e8d;
  FUN_0015c835();
  execlp(local_18,"-sh",&DAT_00183fb4,0);
  FUN_0013e251(local_18);
  do {
    piVar2 = __errno_location();
    if (*piVar2 != 4) {
      FUN_0013eaf9(local_18);
      break;
    }
LAB_00132e8d:
    _Var1 = waitpid(local_1c,&local_20,0);
  } while (_Var1 == -1);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: sys_compare_uid @ 0x32ebf

undefined4 sys_compare_uid(long param_1,long param_2)

{
  return CONCAT31((int3)((uint)*(int *)(param_2 + 0x1c) >> 8),
                  *(int *)(param_1 + 0x1c) == *(int *)(param_2 + 0x1c));
}




// Function: sys_compare_gid @ 0x32ee4

undefined4 sys_compare_gid(long param_1,long param_2)

{
  return CONCAT31((int3)((uint)*(int *)(param_2 + 0x20) >> 8),
                  *(int *)(param_1 + 0x20) == *(int *)(param_2 + 0x20));
}




// Function: sys_compare_links @ 0x32f09

undefined4 sys_compare_links(long *param_1,long *param_2)

{
  undefined4 uVar1;
  
  if ((*param_2 == *param_1) && (param_2[1] == param_1[1])) {
    uVar1 = 1;
  }
  else {
    uVar1 = 0;
  }
  return uVar1;
}




// Function: sys_truncate @ 0x32f52

int sys_truncate(int param_1)

{
  int iVar1;
  __off_t __length;
  
  __length = lseek(param_1,0,1);
  if (__length < 0) {
    iVar1 = -1;
  }
  else {
    iVar1 = ftruncate(param_1,__length);
  }
  return iVar1;
}




// Function: is_regular_file @ 0x32f9a

bool is_regular_file(char *param_1)

{
  int iVar1;
  int *piVar2;
  long in_FS_OFFSET;
  bool bVar3;
  stat local_a8;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = stat(param_1,&local_a8);
  if (iVar1 == 0) {
    bVar3 = (local_a8.st_mode & 0xf000) == 0x8000;
  }
  else {
    piVar2 = __errno_location();
    bVar3 = *piVar2 == 2;
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return bVar3;
}




// Function: sys_write_archive_buffer @ 0x3301a

void sys_write_archive_buffer(void)

{
  if (DAT_0019f9cc < 0x40000000) {
    FUN_00152175(DAT_0019f9cc,DAT_0019efa0,DAT_0019f870);
  }
  else {
    FUN_0013ff8c(DAT_0019f9cc + -0x40000000,DAT_0019efa0,DAT_0019f870);
  }
  return;
}




// Function: xdup2 @ 0x3307b

void xdup2(int param_1,int param_2)

{
  int iVar1;
  int *piVar2;
  undefined8 uVar3;
  
  if (param_1 != param_2) {
    iVar1 = close(param_2);
    if (iVar1 != 0) {
      piVar2 = __errno_location();
      if (*piVar2 != 9) {
        piVar2 = __errno_location();
        iVar1 = *piVar2;
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar3 = gettext("Cannot close");
        error(0,iVar1,uVar3);
                    /* WARNING: Subroutine does not return */
        FUN_001177f4();
      }
    }
    iVar1 = dup(param_1);
    if (iVar1 != param_2) {
      if (iVar1 < 0) {
        piVar2 = __errno_location();
        iVar1 = *piVar2;
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar3 = gettext("Cannot dup");
        error(0,iVar1,uVar3);
                    /* WARNING: Subroutine does not return */
        FUN_001177f4();
      }
                    /* WARNING: Subroutine does not return */
      abort();
    }
    FUN_0010bc11(param_1);
  }
  return;
}




// Function: wait_for_grandchild @ 0x33179

void wait_for_grandchild(__pid_t param_1)

{
  __pid_t _Var1;
  int *piVar2;
  long in_FS_OFFSET;
  uint local_18;
  uint local_14;
  undefined8 local_10;
  
  local_10 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  local_14 = 0;
  do {
    _Var1 = waitpid(param_1,(int *)&local_18,0);
    if (_Var1 != -1) goto LAB_001331d8;
    piVar2 = __errno_location();
  } while (*piVar2 == 4);
  FUN_0013eaf9(DAT_0019f898);
LAB_001331d8:
  if ((char)(((byte)local_18 & 0x7f) + 1) >> 1 < '\x01') {
    if (((int)local_18 >> 8 & 0xffU) != 0) {
      local_14 = (int)local_18 >> 8 & 0xff;
    }
  }
  else {
    raise(local_18 & 0x7f);
  }
                    /* WARNING: Subroutine does not return */
  exit(local_14);
}




// Function: sys_child_open_for_compress @ 0x3321b

int sys_child_open_for_compress(void)

{
  int iVar1;
  int iVar2;
  undefined8 uVar3;
  void *pvVar4;
  int *piVar5;
  ulong uVar6;
  long in_FS_OFFSET;
  long local_40;
  long local_38;
  ulong local_30;
  undefined4 local_20;
  int local_1c;
  undefined4 local_18;
  undefined4 local_14;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  signal(0xd,(__sighandler_t)0x1);
  FUN_0012b577(&local_20);
  iVar1 = FUN_0012b537();
  if (iVar1 < 1) {
    uVar3 = gettext("tar (child)");
    FUN_0015c844(uVar3);
    signal(0xd,(__sighandler_t)0x0);
    FUN_0013307b(local_20,0);
    FUN_0010bc11(local_1c);
    if (((((DAT_0019fdf8 != '\0') ||
          (DAT_0019fdf0 = strchr((char *)*DAT_0019fbd8,0x3a), DAT_0019fdf0 == (char *)0x0)) ||
         (DAT_0019fdf0 <= (char *)*DAT_0019fbd8)) ||
        (pvVar4 = memchr((void *)*DAT_0019fbd8,0x2f,(long)DAT_0019fdf0 - *DAT_0019fbd8),
        pvVar4 != (void *)0x0)) && (iVar1 = FUN_00132f9a(*DAT_0019fbd8), iVar1 != 0)) {
      if (DAT_0019f884 != '\0') {
        FUN_0012a571(*DAT_0019fbd8,1);
      }
      iVar1 = strcmp((char *)*DAT_0019fbd8,"-");
      if (iVar1 != 0) {
        DAT_0019f9cc = creat((char *)*DAT_0019fbd8,0x1b6);
        if (DAT_0019f9cc < 0) {
          piVar5 = __errno_location();
          iVar1 = *piVar5;
          if (DAT_0019f884 != '\0') {
            FUN_0012a874();
          }
          piVar5 = __errno_location();
          *piVar5 = iVar1;
          FUN_0013e3b4(*DAT_0019fbd8);
        }
        FUN_0013307b(DAT_0019f9cc,1);
      }
      FUN_0015c835();
      FUN_00132aaa(DAT_0019f898);
    }
    FUN_0012b577(&local_18);
    iVar1 = FUN_0012b537();
    if (iVar1 == 0) {
      uVar3 = gettext("tar (grandchild)");
      FUN_0015c844(uVar3);
      FUN_0013307b(local_14,1);
      FUN_0010bc11(local_18);
      FUN_0015c835();
      FUN_00132aaa(DAT_0019f898);
    }
    FUN_0013307b(local_18,0);
    FUN_0010bc11(local_14);
    iVar2 = strcmp((char *)*DAT_0019fbd8,"-");
    if (iVar2 == 0) {
      DAT_0019f9cc = 1;
    }
    else {
      if (((DAT_0019fdf8 == '\x01') ||
          (DAT_0019fdf0 = strchr((char *)*DAT_0019fbd8,0x3a), DAT_0019fdf0 == (char *)0x0)) ||
         ((DAT_0019fdf0 <= (char *)*DAT_0019fbd8 ||
          (pvVar4 = memchr((void *)*DAT_0019fbd8,0x2f,(long)DAT_0019fdf0 - *DAT_0019fbd8),
          pvVar4 != (void *)0x0)))) {
        DAT_0019f9cc = creat((char *)*DAT_0019fbd8,0x1b6);
      }
      else {
        DAT_0019f9cc = FUN_0013f860(*DAT_0019fbd8,0x41,0x40000000,DAT_0019f950);
      }
      if (DAT_0019f9cc < 0) {
        FUN_0013e3b4(*DAT_0019fbd8);
      }
    }
    do {
      local_40 = 0;
      local_38 = DAT_0019efa0;
      for (local_30 = 0; local_30 < DAT_0019f870; local_30 = local_30 + local_40) {
        local_40 = FUN_0015ec70(0,local_38,DAT_0019f870 - local_30);
        if (local_40 == -1) {
          FUN_0013e5d6(DAT_0019f898);
        }
        if (local_40 == 0) break;
        local_38 = local_38 + local_40;
      }
      if (local_40 == 0) {
        if (local_30 == 0) goto code_r0x001336c4;
        memset((void *)(DAT_0019efa0 + local_30),0,DAT_0019f870 - local_30);
        uVar6 = FUN_0013301a();
        if (uVar6 == DAT_0019f870) goto code_r0x001336c4;
        FUN_0010c6d5(uVar6);
      }
      uVar6 = FUN_0013301a();
    } while (uVar6 == DAT_0019f870);
    FUN_0010c6d5(uVar6);
code_r0x001336c4:
    FUN_00133179(iVar1);
  }
  else {
    DAT_0019f9cc = local_1c;
    FUN_0010bc11(local_20);
    if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
      return iVar1;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: run_decompress_program @ 0x336d5

int run_decompress_program(void)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  void *pvVar6;
  ulong uVar7;
  ulong uVar8;
  long in_FS_OFFSET;
  long lStack_168;
  ulong uStack_160;
  int iStack_148;
  undefined4 uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  long lStack_138;
  code *pcStack_130;
  undefined1 local_120 [4];
  uint local_11c;
  long local_118;
  long local_110;
  long local_108;
  char **local_100;
  long local_f8;
  undefined8 local_98;
  undefined8 local_20;
  
  local_20 = *(undefined8 *)(in_FS_OFFSET + 0x28);
  local_110 = 0;
  local_11c = 0x2080e06;
  local_98 = environ;
  local_f8 = 1;
  pcStack_130 = (code *)0x133731;
  local_118 = FUN_0010afa2(local_120);
  while( true ) {
    if (local_118 == 0) {
      if (local_110 == 0) {
        if (DAT_0019fdc8 != (code *)0x0) {
          pcStack_130 = (code *)0x133961;
          (*DAT_0019fdc8)();
        }
        pcStack_130 = (code *)0x133970;
        uVar5 = gettext("unable to run decompression program");
        pcStack_130 = (code *)0x133987;
        error(0,0,uVar5);
                    /* WARNING: Subroutine does not return */
        pcStack_130 = (code *)0x13398c;
        FUN_001177f4();
      }
      pcStack_130 = FUN_0013399b;
      FUN_0013e251(local_110);
      lStack_138 = *(long *)(in_FS_OFFSET + 0x28);
      pcStack_130 = (code *)&stack0xfffffffffffffff8;
      FUN_0012b577(&iStack_148);
      iVar1 = FUN_0012b537();
      if (iVar1 < 1) {
        uVar5 = gettext("tar (child)");
        FUN_0015c844(uVar5);
        signal(0xd,(__sighandler_t)0x0);
        FUN_0013307b(uStack_144,1);
        FUN_0010bc11(iStack_148);
        iVar1 = strcmp((char *)*DAT_0019fbd8,"-");
        if (((iVar1 != 0) &&
            ((((DAT_0019fdf8 != '\0' ||
               (DAT_0019fdf0 = strchr((char *)*DAT_0019fbd8,0x3a), DAT_0019fdf0 == (char *)0x0)) ||
              (DAT_0019fdf0 <= (char *)*DAT_0019fbd8)) ||
             (pvVar6 = memchr((void *)*DAT_0019fbd8,0x2f,(long)DAT_0019fdf0 - *DAT_0019fbd8),
             pvVar6 != (void *)0x0)))) && (iVar1 = FUN_00132f9a(*DAT_0019fbd8), iVar1 != 0)) {
          DAT_0019f9cc = open((char *)*DAT_0019fbd8,0,0x1b6);
          if (DAT_0019f9cc < 0) {
            FUN_0013e3b4(*DAT_0019fbd8);
          }
          FUN_0013307b(DAT_0019f9cc,0);
          FUN_0015c835();
          run_decompress_program();
        }
        FUN_0012b577(&uStack_140);
        iVar1 = FUN_0012b537();
        if (iVar1 == 0) {
          uVar5 = gettext("tar (grandchild)");
          FUN_0015c844(uVar5);
          FUN_0013307b(uStack_140,0);
          FUN_0010bc11(uStack_13c);
          FUN_0015c835();
          run_decompress_program();
        }
        FUN_0013307b(uStack_13c,1);
        FUN_0010bc11(uStack_140);
        iVar2 = strcmp((char *)*DAT_0019fbd8,"-");
        if (iVar2 == 0) {
          DAT_0019f9cc = 0;
        }
        else if (((DAT_0019fdf8 == '\x01') ||
                 (DAT_0019fdf0 = strchr((char *)*DAT_0019fbd8,0x3a), DAT_0019fdf0 == (char *)0x0))
                || ((DAT_0019fdf0 <= (char *)*DAT_0019fbd8 ||
                    (pvVar6 = memchr((void *)*DAT_0019fbd8,0x2f,(long)DAT_0019fdf0 - *DAT_0019fbd8),
                    pvVar6 != (void *)0x0)))) {
          DAT_0019f9cc = open((char *)*DAT_0019fbd8,0,0x1b6);
        }
        else {
          DAT_0019f9cc = FUN_0013f860(*DAT_0019fbd8,0,0x40000000,DAT_0019f950);
        }
        if (DAT_0019f9cc < 0) {
          FUN_0013e3b4(*DAT_0019fbd8);
        }
        while( true ) {
          FUN_0010adb0();
          while( true ) {
            if (DAT_0019f9cc < 0x40000000) {
              uStack_160 = FUN_0015ec70(DAT_0019f9cc,DAT_0019efa0,DAT_0019f870);
            }
            else {
              uStack_160 = FUN_0013fe62(DAT_0019f9cc + -0x40000000,DAT_0019efa0,DAT_0019f870);
            }
            if (uStack_160 != 0xffffffffffffffff) break;
            FUN_0010c729();
          }
          if (uStack_160 == 0) break;
          lStack_168 = DAT_0019efa0;
          for (; uStack_160 != 0; uStack_160 = uStack_160 - uVar7) {
            uVar7 = uStack_160;
            if (0x200 < uStack_160) {
              uVar7 = 0x200;
            }
            uVar8 = FUN_00152175(1,lStack_168,uVar7);
            if (uVar7 != uVar8) {
              FUN_0013eb22(DAT_0019f898);
            }
            lStack_168 = lStack_168 + uVar7;
          }
        }
        FUN_0010bc11(1);
        FUN_00133179(iVar1);
      }
      else {
        DAT_0019f9cc = iStack_148;
        FUN_0010bc11(uStack_144);
        if (lStack_138 == *(long *)(in_FS_OFFSET + 0x28)) {
          return iVar1;
        }
      }
                    /* WARNING: Subroutine does not return */
      __stack_chk_fail();
    }
    if (local_110 != 0) {
      if ((DAT_0019ec84 & 0x80000) != 0) {
        if (DAT_0019fdc8 != (code *)0x0) {
          pcStack_130 = (code *)0x13376f;
          (*DAT_0019fdc8)();
        }
        pcStack_130 = (code *)0x13377e;
        uVar5 = gettext("cannot run %s");
        pcStack_130 = (code *)0x133786;
        piVar3 = __errno_location();
        pcStack_130 = (code *)0x1337a6;
        error(0,*piVar3,uVar5,local_110);
      }
      if ((DAT_0019ec84 & 0x80000) != 0) {
        if (DAT_0019fdc8 != (code *)0x0) {
          pcStack_130 = (code *)0x1337ca;
          (*DAT_0019fdc8)();
        }
        pcStack_130 = (code *)0x1337d9;
        uVar5 = gettext("trying %s");
        pcStack_130 = (code *)0x1337fa;
        error(0,0,uVar5,local_118);
      }
    }
    pcStack_130 = (code *)0x133819;
    iVar1 = FUN_001461a6(local_118,&local_108,local_11c);
    if (iVar1 != 0) break;
    local_11c = local_11c | 8;
    pcStack_130 = (code *)0x1338b9;
    memmove(local_100,local_100 + local_f8,local_108 * 8);
    local_100[local_108] = "-d";
    local_110 = local_118;
    pcStack_130 = (code *)0x133902;
    execvp(*local_100,local_100);
    local_100[local_108] = (char *)0x0;
    pcStack_130 = (code *)0x13392d;
    local_118 = FUN_0010b012(local_120);
  }
  if (DAT_0019fdc8 != (code *)0x0) {
    pcStack_130 = (code *)0x133832;
    (*DAT_0019fdc8)();
  }
  pcStack_130 = (code *)0x133841;
  uVar5 = FUN_0014647f(&local_108);
  pcStack_130 = (code *)0x133853;
  uVar4 = gettext("cannot split string \'%s\': %s");
  pcStack_130 = (code *)0x133877;
  error(0,0,uVar4,local_118,uVar5);
                    /* WARNING: Subroutine does not return */
  pcStack_130 = (code *)0x13387c;
  FUN_001177f4();
}




// Function: sys_child_open_for_uncompress @ 0x3399b

int sys_child_open_for_uncompress(void)

{
  int iVar1;
  int iVar2;
  undefined8 uVar3;
  void *pvVar4;
  ulong uVar5;
  ulong uVar6;
  long in_FS_OFFSET;
  long local_40;
  ulong local_38;
  int local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0012b577(&local_20);
  iVar1 = FUN_0012b537();
  if (iVar1 < 1) {
    uVar3 = gettext("tar (child)");
    FUN_0015c844(uVar3);
    signal(0xd,(__sighandler_t)0x0);
    FUN_0013307b(local_1c,1);
    FUN_0010bc11(local_20);
    iVar1 = strcmp((char *)*DAT_0019fbd8,"-");
    if (((iVar1 != 0) &&
        ((((DAT_0019fdf8 != '\0' ||
           (DAT_0019fdf0 = strchr((char *)*DAT_0019fbd8,0x3a), DAT_0019fdf0 == (char *)0x0)) ||
          (DAT_0019fdf0 <= (char *)*DAT_0019fbd8)) ||
         (pvVar4 = memchr((void *)*DAT_0019fbd8,0x2f,(long)DAT_0019fdf0 - *DAT_0019fbd8),
         pvVar4 != (void *)0x0)))) && (iVar1 = FUN_00132f9a(*DAT_0019fbd8), iVar1 != 0)) {
      DAT_0019f9cc = open((char *)*DAT_0019fbd8,0,0x1b6);
      if (DAT_0019f9cc < 0) {
        FUN_0013e3b4(*DAT_0019fbd8);
      }
      FUN_0013307b(DAT_0019f9cc,0);
      FUN_0015c835();
      FUN_001336d5();
    }
    FUN_0012b577(&local_18);
    iVar1 = FUN_0012b537();
    if (iVar1 == 0) {
      uVar3 = gettext("tar (grandchild)");
      FUN_0015c844(uVar3);
      FUN_0013307b(local_18,0);
      FUN_0010bc11(local_14);
      FUN_0015c835();
      FUN_001336d5();
    }
    FUN_0013307b(local_14,1);
    FUN_0010bc11(local_18);
    iVar2 = strcmp((char *)*DAT_0019fbd8,"-");
    if (iVar2 == 0) {
      DAT_0019f9cc = 0;
    }
    else if (((DAT_0019fdf8 == '\x01') ||
             (DAT_0019fdf0 = strchr((char *)*DAT_0019fbd8,0x3a), DAT_0019fdf0 == (char *)0x0)) ||
            ((DAT_0019fdf0 <= (char *)*DAT_0019fbd8 ||
             (pvVar4 = memchr((void *)*DAT_0019fbd8,0x2f,(long)DAT_0019fdf0 - *DAT_0019fbd8),
             pvVar4 != (void *)0x0)))) {
      DAT_0019f9cc = open((char *)*DAT_0019fbd8,0,0x1b6);
    }
    else {
      DAT_0019f9cc = FUN_0013f860(*DAT_0019fbd8,0,0x40000000,DAT_0019f950);
    }
    if (DAT_0019f9cc < 0) {
      FUN_0013e3b4(*DAT_0019fbd8);
    }
    while( true ) {
      FUN_0010adb0();
      while( true ) {
        if (DAT_0019f9cc < 0x40000000) {
          local_38 = FUN_0015ec70(DAT_0019f9cc,DAT_0019efa0,DAT_0019f870);
        }
        else {
          local_38 = FUN_0013fe62(DAT_0019f9cc + -0x40000000,DAT_0019efa0,DAT_0019f870);
        }
        if (local_38 != 0xffffffffffffffff) break;
        FUN_0010c729();
      }
      if (local_38 == 0) break;
      local_40 = DAT_0019efa0;
      for (; local_38 != 0; local_38 = local_38 - uVar5) {
        uVar5 = local_38;
        if (0x200 < local_38) {
          uVar5 = 0x200;
        }
        uVar6 = FUN_00152175(1,local_40,uVar5);
        if (uVar5 != uVar6) {
          FUN_0013eb22(DAT_0019f898);
        }
        local_40 = local_40 + uVar5;
      }
    }
    FUN_0010bc11(1);
    FUN_00133179(iVar1);
  }
  else {
    DAT_0019f9cc = local_20;
    FUN_0010bc11(local_1c);
    if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
      return iVar1;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: dec_to_env @ 0x33de6

void dec_to_env(char *param_1,undefined8 param_2)

{
  int iVar1;
  char *__value;
  long in_FS_OFFSET;
  undefined1 local_28 [24];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  __value = (char *)FUN_00154e5a(param_2,local_28);
  iVar1 = setenv(param_1,__value,1);
  if (iVar1 != 0) {
    FUN_0011783c();
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: time_to_env @ 0x33e58

void time_to_env(char *param_1,undefined8 param_2,undefined8 param_3)

{
  int iVar1;
  char *__value;
  long in_FS_OFFSET;
  undefined1 local_38 [40];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  __value = (char *)FUN_00129f2e(param_2,param_3,local_38);
  iVar1 = setenv(param_1,__value,1);
  if (iVar1 != 0) {
    FUN_0011783c();
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: oct_to_env @ 0x33ed9

void oct_to_env(char *param_1,undefined8 param_2)

{
  int iVar1;
  long in_FS_OFFSET;
  char local_28 [24];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  snprintf(local_28,0x18,"0%lo",param_2);
  iVar1 = setenv(param_1,local_28,1);
  if (iVar1 != 0) {
    FUN_0011783c();
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: str_to_env @ 0x33f58

void str_to_env(char *param_1,char *param_2)

{
  int iVar1;
  
  if (param_2 != (char *)0x0) {
    iVar1 = setenv(param_1,param_2,1);
    if (iVar1 == 0) {
      return;
    }
    FUN_0011783c();
  }
  unsetenv(param_1);
  return;
}




// Function: chr_to_env @ 0x33fa3

void chr_to_env(char *param_1,char param_2)

{
  int iVar1;
  long in_FS_OFFSET;
  char local_12 [2];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_12[1] = 0;
  local_12[0] = param_2;
  iVar1 = setenv(param_1,local_12,1);
  if (iVar1 != 0) {
    FUN_0011783c();
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: stat_to_env @ 0x3400a

void stat_to_env(undefined8 param_1,char param_2,long param_3)

{
  int iVar1;
  uint uVar2;
  undefined8 uVar3;
  
  FUN_00133f58("TAR_VERSION",&DAT_00184059);
  FUN_00133f58("TAR_ARCHIVE",*DAT_0019fbe8);
  FUN_00133de6("TAR_VOLUME",((long)DAT_0019fbe8 - DAT_0019fbd8 >> 3) + 1);
  FUN_00133de6("TAR_BLOCKING_FACTOR",(long)DAT_0019f868);
  iVar1 = DAT_0019f468;
  if (DAT_0019f468 == 0) {
    iVar1 = DAT_0019f864;
  }
  uVar3 = FUN_00134d66(iVar1);
  FUN_00133f58("TAR_FORMAT",uVar3);
  FUN_00133fa3("TAR_FILETYPE",(int)param_2);
  FUN_00133ed9("TAR_MODE",*(undefined4 *)(param_3 + 0x70));
  FUN_00133f58("TAR_FILENAME",param_1);
  FUN_00133f58("TAR_REALNAME",*(undefined8 *)(param_3 + 8));
  FUN_00133f58("TAR_UNAME",*(undefined8 *)(param_3 + 0x20));
  FUN_00133f58("TAR_GNAME",*(undefined8 *)(param_3 + 0x28));
  FUN_00133e58("TAR_ATIME",*(undefined8 *)(param_3 + 0xe8),*(undefined8 *)(param_3 + 0xf0));
  FUN_00133e58("TAR_MTIME",*(undefined8 *)(param_3 + 0xf8),*(undefined8 *)(param_3 + 0x100));
  FUN_00133e58("TAR_CTIME",*(undefined8 *)(param_3 + 0x108),*(undefined8 *)(param_3 + 0x110));
  FUN_00133de6("TAR_SIZE",*(undefined8 *)(param_3 + 0x88));
  FUN_00133de6("TAR_UID",*(undefined4 *)(param_3 + 0x74));
  FUN_00133de6("TAR_GID",*(undefined4 *)(param_3 + 0x78));
  if (param_2 == 'l') {
LAB_001342b8:
    unsetenv("TAR_MINOR");
    unsetenv("TAR_MAJOR");
    FUN_00133f58("TAR_LINKNAME",*(undefined8 *)(param_3 + 0x18));
  }
  else {
    if (param_2 < 'm') {
      if (param_2 < 'd') {
        if ('a' < param_2) {
          uVar2 = gnu_dev_minor(*(ulonglong *)(param_3 + 0x80));
          FUN_00133de6("TAR_MINOR",uVar2);
          uVar2 = gnu_dev_major(*(ulonglong *)(param_3 + 0x80));
          FUN_00133de6("TAR_MAJOR",uVar2);
          unsetenv("TAR_LINKNAME");
          return;
        }
      }
      else if (param_2 == 'h') goto LAB_001342b8;
    }
    unsetenv("TAR_MINOR");
    unsetenv("TAR_MAJOR");
    unsetenv("TAR_LINKNAME");
  }
  return;
}




// Function: sys_exec_command @ 0x34323

undefined4 sys_exec_command(undefined8 param_1,char param_2,undefined8 param_3)

{
  long in_FS_OFFSET;
  undefined4 local_18;
  undefined4 local_14;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0012b577(&local_18);
  DAT_0019f838 = signal(0xd,(__sighandler_t)0x1);
  DAT_0019f830 = FUN_0012b537();
  if (DAT_0019f830 == 0) {
    FUN_0013307b(local_18,0);
    FUN_0010bc11(local_14);
    FUN_0013400a(param_1,(int)param_2,param_3);
    FUN_0015c835();
    FUN_00132aaa(DAT_0019f9a0);
  }
  else {
    FUN_0010bc11(local_18);
    if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
      return local_14;
    }
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sys_wait_command @ 0x343ea

void sys_wait_command(void)

{
  __pid_t _Var1;
  int *piVar2;
  undefined8 uVar3;
  long lVar4;
  long in_FS_OFFSET;
  uint local_24;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if (-1 < DAT_0019f830) {
    signal(0xd,DAT_0019f838);
    do {
      _Var1 = waitpid(DAT_0019f830,(int *)&local_24,0);
      if (_Var1 != -1) {
        if ((local_24 & 0x7f) == 0) {
          if ((DAT_0019f9a8 != '\x01') && (((int)local_24 >> 8 & 0xffU) != 0)) {
            if (DAT_0019fdc8 != (code *)0x0) {
              (*DAT_0019fdc8)();
            }
            lVar4 = (long)DAT_0019f830;
            uVar3 = gettext("%lu: Child returned status %d");
            error(0,0,uVar3,lVar4,(int)local_24 >> 8 & 0xff);
            DAT_0019fdd0 = 2;
          }
        }
        else if ((char)(((byte)local_24 & 0x7f) + 1) >> 1 < '\x01') {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          lVar4 = (long)DAT_0019f830;
          uVar3 = gettext("%lu: Child terminated on unknown reason");
          error(0,0,uVar3,lVar4);
          DAT_0019fdd0 = 2;
        }
        else {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          lVar4 = (long)DAT_0019f830;
          uVar3 = gettext("%lu: Child terminated on signal %d");
          error(0,0,uVar3,lVar4,local_24 & 0x7f);
        }
        DAT_0019f830 = -1;
        goto LAB_001345ca;
      }
      piVar2 = __errno_location();
    } while (*piVar2 == 4);
    DAT_0019f830 = -1;
    FUN_0013eaf9(DAT_0019f9a0);
  }
LAB_001345ca:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: sys_exec_info_script @ 0x345e7

ulong sys_exec_info_script(undefined8 *param_1,int param_2)

{
  __pid_t _Var1;
  int iVar2;
  __ssize_t _Var3;
  int *piVar4;
  ulong uVar5;
  char *pcVar6;
  long in_FS_OFFSET;
  uint local_54;
  int local_50;
  int local_4c;
  char *local_48;
  size_t local_40;
  FILE *local_38;
  int local_30;
  int local_2c;
  undefined1 local_28 [24];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  FUN_0012b577(&local_30);
  DAT_0019f840 = signal(0xd,(__sighandler_t)0x1);
  local_4c = FUN_0012b537();
  if (local_4c == 0) {
    setenv("TAR_VERSION","1.34",1);
    setenv("TAR_ARCHIVE",(char *)*param_1,1);
    pcVar6 = (char *)FUN_00154e5a((long)param_2,local_28);
    setenv("TAR_VOLUME",pcVar6,1);
    pcVar6 = (char *)FUN_00154e5a((long)DAT_0019f868,local_28);
    setenv("TAR_BLOCKING_FACTOR",pcVar6,1);
    pcVar6 = (char *)FUN_00134e1c(DAT_0019f860);
    setenv("TAR_SUBCOMMAND",pcVar6,1);
    iVar2 = DAT_0019f468;
    if (DAT_0019f468 == 0) {
      iVar2 = DAT_0019f864;
    }
    pcVar6 = (char *)FUN_00134d66(iVar2);
    setenv("TAR_FORMAT",pcVar6,1);
    pcVar6 = (char *)FUN_00154e5a((long)local_2c,local_28);
    setenv("TAR_FD",pcVar6,1);
    FUN_0010bc11(local_30);
    FUN_0015c835();
    uVar5 = FUN_00132aaa(DAT_0019f8c0);
  }
  else {
    local_48 = (char *)0x0;
    local_40 = 0;
    FUN_0010bc11(local_2c);
    local_38 = fdopen(local_30,"r");
    _Var3 = getline(&local_48,&local_40,local_38);
    local_50 = (int)_Var3;
    fclose(local_38);
    if ((0 < local_50) && (local_48[(long)local_50 + -1] == '\n')) {
      local_50 = local_50 + -1;
      local_48[local_50] = '\0';
    }
    do {
      _Var1 = waitpid(local_4c,(int *)&local_54,0);
      if (_Var1 != -1) {
        signal(0xd,DAT_0019f840);
        if ((local_54 & 0x7f) == 0) {
          if ((((int)local_54 >> 8 & 0xffU) == 0) && (0 < local_50)) {
            *param_1 = local_48;
          }
          else {
            free(local_48);
          }
          uVar5 = (ulong)((int)local_54 >> 8 & 0xff);
        }
        else {
          free(local_48);
          uVar5 = 0xffffffff;
        }
        goto LAB_001348c1;
      }
      piVar4 = __errno_location();
    } while (*piVar4 == 4);
    signal(0xd,DAT_0019f840);
    FUN_0013eaf9(DAT_0019f8c0);
    uVar5 = 0xffffffff;
  }
LAB_001348c1:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: sys_exec_checkpoint_script @ 0x348d7

void sys_exec_checkpoint_script(undefined8 param_1,char *param_2,int param_3)

{
  __pid_t _Var1;
  int iVar2;
  int *piVar3;
  char *pcVar4;
  long in_FS_OFFSET;
  int local_30;
  __pid_t local_2c;
  undefined1 local_28 [24];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_2c = FUN_0012b537();
  if (local_2c == 0) {
    setenv("TAR_VERSION","1.34",1);
    setenv("TAR_ARCHIVE",param_2,1);
    pcVar4 = (char *)FUN_00154e5a((long)param_3,local_28);
    setenv("TAR_CHECKPOINT",pcVar4,1);
    pcVar4 = (char *)FUN_00154e5a((long)DAT_0019f868,local_28);
    setenv("TAR_BLOCKING_FACTOR",pcVar4,1);
    pcVar4 = (char *)FUN_00134e1c(DAT_0019f860);
    setenv("TAR_SUBCOMMAND",pcVar4,1);
    iVar2 = DAT_0019f468;
    if (DAT_0019f468 == 0) {
      iVar2 = DAT_0019f864;
    }
    pcVar4 = (char *)FUN_00134d66(iVar2);
    setenv("TAR_FORMAT",pcVar4,1);
    FUN_0015c835();
    FUN_00132aaa(param_1);
  }
  else {
    do {
      _Var1 = waitpid(local_2c,&local_30,0);
      if (_Var1 != -1) goto LAB_00134a48;
      piVar3 = __errno_location();
    } while (*piVar3 == 4);
    FUN_0013eaf9(param_1);
  }
LAB_00134a48:
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: request_stdin @ 0x34a6f

void request_stdin(undefined8 param_1)

{
  long lVar1;
  undefined8 uVar2;
  
  if (DAT_0019fc40 != 0) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    lVar1 = DAT_0019fc40;
    uVar2 = gettext("Options \'%s\' and \'%s\' both want standard input");
    error(0,0,uVar2,lVar1,param_1);
                    /* WARNING: Subroutine does not return */
    FUN_0013814c(2);
  }
  DAT_0019fc40 = param_1;
  return;
}




// Function: confirm @ 0x34af4

bool confirm(undefined8 param_1,undefined8 param_2)

{
  int iVar1;
  undefined8 uVar2;
  __ssize_t _Var3;
  long in_FS_OFFSET;
  bool local_21;
  char *local_20;
  size_t local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_21 = false;
  if (DAT_0019fcb0 == (FILE *)0x0) {
    if ((DAT_0019f9cc == 0) || (DAT_0019fc40 != 0)) {
      DAT_0019fcb0 = fopen("/dev/tty","r");
      if (DAT_0019fcb0 != (FILE *)0x0) goto LAB_00134b95;
      FUN_0013e3b4("/dev/tty");
    }
    FUN_00134a6f(&DAT_0018423a);
    DAT_0019fcb0 = stdin;
  }
LAB_00134b95:
  uVar2 = FUN_0015e84e(param_2);
  fprintf(DAT_0019efd0,"%s %s?",param_1,uVar2);
  fflush_unlocked(DAT_0019efd0);
  if (DAT_0019fcb8 == 0) {
    local_20 = (char *)0x0;
    local_18 = 0;
    _Var3 = getline(&local_20,&local_18,DAT_0019fcb0);
    if (_Var3 < 0) {
      DAT_0019fcb8 = 1;
    }
    else {
      iVar1 = rpmatch(local_20);
      local_21 = 0 < iVar1;
    }
    free(local_20);
  }
  if (DAT_0019fcb8 != 0) {
    fputc_unlocked(10,DAT_0019efd0);
    fflush_unlocked(DAT_0019efd0);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_21;
}




// Function: set_archive_format @ 0x34c7e

void set_archive_format(char *param_1)

{
  int iVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined **local_20;
  
  local_20 = &PTR_DAT_0019a8e0;
  do {
    iVar1 = strcmp(*local_20,param_1);
    if (iVar1 == 0) {
      DAT_0019f864 = *(undefined4 *)(local_20 + 1);
      return;
    }
    local_20 = local_20 + 2;
  } while (*local_20 != (undefined *)0x0);
  if (DAT_0019fdc8 != (code *)0x0) {
    (*DAT_0019fdc8)();
  }
  uVar2 = FUN_0015e548(param_1);
  uVar3 = gettext("%s: Invalid archive format");
  error(0,0,uVar3,uVar2);
                    /* WARNING: Subroutine does not return */
  FUN_0013814c(2);
}




// Function: set_xattr_option @ 0x34d36

void set_xattr_option(int param_1)

{
  if (param_1 == 1) {
    FUN_00134c7e("posix");
  }
  DAT_0019f96c = param_1;
  return;
}




// Function: archive_format_string @ 0x34d66

char * archive_format_string(int param_1)

{
  undefined **local_10;
  
  local_10 = &PTR_DAT_0019a8e0;
  while( true ) {
    if (*local_10 == (undefined *)0x0) {
      return "unknown?";
    }
    if (param_1 == *(int *)(local_10 + 1)) break;
    local_10 = local_10 + 2;
  }
  return *local_10;
}




// Function: assert_format @ 0x34dad

void assert_format(uint param_1)

{
  undefined8 uVar1;
  
  if ((1 << ((byte)DAT_0019f864 & 0x1f) & param_1) == 0) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar1 = gettext("GNU features wanted on incompatible archive format");
    error(0,0,uVar1);
                    /* WARNING: Subroutine does not return */
    FUN_0013814c(2);
  }
  return;
}




// Function: subcommand_string @ 0x34e1c

char * subcommand_string(undefined4 param_1)

{
  char *pcVar1;
  
  switch(param_1) {
  case 0:
    pcVar1 = "unknown?";
    break;
  case 1:
    pcVar1 = "-r";
    break;
  case 2:
    pcVar1 = "-A";
    break;
  case 3:
    pcVar1 = "-c";
    break;
  case 4:
    pcVar1 = "-D";
    break;
  case 5:
    pcVar1 = "-d";
    break;
  case 6:
    pcVar1 = "-x";
    break;
  case 7:
    pcVar1 = "-t";
    break;
  case 8:
    pcVar1 = "-u";
    break;
  case 9:
    pcVar1 = "--test-label";
    break;
  default:
                    /* WARNING: Subroutine does not return */
    abort();
  }
  return pcVar1;
}




// Function: tar_list_quoting_styles @ 0x34eb6

void tar_list_quoting_styles(long param_1,char *param_2)

{
  undefined1 *puVar1;
  size_t __n;
  size_t __n_00;
  int local_54;
  
  __n = strlen(param_2);
  for (local_54 = 0; (&PTR_s_literal_0019b6c0)[local_54] != (undefined *)0x0;
      local_54 = local_54 + 1) {
    if ((ulong)(*(long *)(param_1 + 0x20) - *(long *)(param_1 + 0x18)) < __n) {
      _obstack_newchunk(param_1,__n);
    }
    memcpy(*(void **)(param_1 + 0x18),param_2,__n);
    *(size_t *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + __n;
    __n_00 = strlen((&PTR_s_literal_0019b6c0)[local_54]);
    if ((ulong)(*(long *)(param_1 + 0x20) - *(long *)(param_1 + 0x18)) < __n_00) {
      _obstack_newchunk(param_1,__n_00);
    }
    memcpy(*(void **)(param_1 + 0x18),(&PTR_s_literal_0019b6c0)[local_54],__n_00);
    *(size_t *)(param_1 + 0x18) = *(long *)(param_1 + 0x18) + __n_00;
    if (*(long *)(param_1 + 0x20) == *(long *)(param_1 + 0x18)) {
      _obstack_newchunk(param_1,1);
    }
    puVar1 = *(undefined1 **)(param_1 + 0x18);
    *(undefined1 **)(param_1 + 0x18) = puVar1 + 1;
    *puVar1 = 10;
  }
  return;
}




// Function: tar_set_quoting_style @ 0x35084

void tar_set_quoting_style(char *param_1)

{
  undefined8 uVar1;
  int iVar2;
  undefined8 uVar3;
  int local_1c;
  
  local_1c = 0;
  while( true ) {
    if ((&PTR_s_literal_0019b6c0)[local_1c] == (undefined *)0x0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar1 = DAT_0019fe48;
      uVar3 = gettext("Unknown quoting style \'%s\'. Try \'%s --quoting-style=help\' to get a list."
                     );
      error(0,0,uVar3,param_1,uVar1);
                    /* WARNING: Subroutine does not return */
      FUN_001177f4();
    }
    iVar2 = strcmp(param_1,(&PTR_s_literal_0019b6c0)[local_1c]);
    if (iVar2 == 0) break;
    local_1c = local_1c + 1;
  }
  FUN_0015c9a2(0,local_1c);
  return;
}




// Function: format_default_settings @ 0x35152

void format_default_settings(void)

{
  undefined *puVar1;
  undefined8 uVar2;
  
  puVar1 = PTR_s_escape_0019b6f8;
  uVar2 = FUN_00134d66(6);
  FUN_00161277("--format=%s -f%s -b%d --quoting-style=%s --rmt-command=%s --rsh-command=%s",uVar2,
               &DAT_001867e7,0x14,puVar1,"/usr/local/libexec/rmt","/usr/bin/rsh");
  return;
}




// Function: option_conflict_error @ 0x351b6

void option_conflict_error(undefined8 param_1,undefined8 param_2)

{
  undefined8 uVar1;
  
  if (DAT_0019fdc8 != (code *)0x0) {
    (*DAT_0019fdc8)();
  }
  uVar1 = gettext("\'%s\' cannot be used with \'%s\'");
  error(0,0,uVar1,param_1,param_2);
                    /* WARNING: Subroutine does not return */
  FUN_0013814c(2);
}




// Function: optloc_save @ 0x3521d

undefined8 optloc_save(uint param_1,undefined4 *param_2)

{
  size_t sVar1;
  undefined4 *puVar2;
  long local_20;
  
  if (9 < param_1) {
                    /* WARNING: Subroutine does not return */
    abort();
  }
  local_20 = 0x20;
  if (*(long *)(param_2 + 2) != 0) {
    sVar1 = strlen(*(char **)(param_2 + 2));
    local_20 = sVar1 + 0x21;
  }
  puVar2 = (undefined4 *)FUN_00160b5b(local_20);
  if (*(long *)(param_2 + 2) == 0) {
    *(undefined8 *)(puVar2 + 2) = 0;
  }
  else {
    strcpy((char *)(puVar2 + 8),*(char **)(param_2 + 2));
    *(undefined4 **)(puVar2 + 2) = puVar2 + 8;
  }
  *puVar2 = *param_2;
  *(undefined8 *)(puVar2 + 4) = *(undefined8 *)(param_2 + 4);
  *(undefined8 *)(puVar2 + 6) = *(undefined8 *)(&DAT_0019fc60 + (ulong)param_1 * 8);
  *(undefined4 **)(&DAT_0019fc60 + (ulong)param_1 * 8) = puVar2;
  return *(undefined8 *)(puVar2 + 6);
}




// Function: optloc_lookup @ 0x35327

undefined8 optloc_lookup(int param_1)

{
  return *(undefined8 *)(&DAT_0019fc60 + (long)param_1 * 8);
}




// Function: option_set_in_cl @ 0x3534c

bool option_set_in_cl(undefined4 param_1)

{
  int *piVar1;
  bool bVar2;
  
  piVar1 = (int *)FUN_00135327(param_1);
  if (piVar1 == (int *)0x0) {
    bVar2 = false;
  }
  else {
    bVar2 = *piVar1 == 1;
  }
  return bVar2;
}




// Function: optloc_eq @ 0x35388

bool optloc_eq(int *param_1,int *param_2)

{
  int iVar1;
  bool bVar2;
  
  if (*param_1 == *param_2) {
    if (*param_1 == 1) {
      bVar2 = true;
    }
    else {
      iVar1 = strcmp(*(char **)(param_1 + 2),*(char **)(param_2 + 2));
      bVar2 = iVar1 == 0;
    }
  }
  else {
    bVar2 = false;
  }
  return bVar2;
}




// Function: set_subcommand_option @ 0x353ea

void set_subcommand_option(int param_1)

{
  undefined8 uVar1;
  
  if ((DAT_0019f860 != 0) && (param_1 != DAT_0019f860)) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar1 = gettext(
                   "You may not specify more than one \'-Acdtrux\', \'--delete\' or  \'--test-label\' option"
                   );
    error(0,0,uVar1);
                    /* WARNING: Subroutine does not return */
    FUN_0013814c(2);
  }
  DAT_0019f860 = param_1;
  return;
}




// Function: set_use_compress_program_option @ 0x3545f

void set_use_compress_program_option(char *param_1,undefined8 param_2)

{
  int iVar1;
  int *piVar2;
  undefined8 uVar3;
  
  piVar2 = (int *)FUN_0013521d(0,param_2);
  if (DAT_0019f898 != (char *)0x0) {
    iVar1 = strcmp(DAT_0019f898,param_1);
    if ((iVar1 != 0) && (*piVar2 == 1)) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar3 = gettext("Conflicting compression options");
      error(0,0,uVar3);
                    /* WARNING: Subroutine does not return */
      FUN_0013814c(2);
    }
  }
  DAT_0019f898 = param_1;
  return;
}




// Function: sigstat @ 0x3550c

void sigstat(int param_1)

{
  FUN_0010ae4d();
  FUN_0010baa2();
  signal(param_1,sigstat);
  return;
}




// Function: stat_on_signal @ 0x3553c

void stat_on_signal(int param_1)

{
  signal(param_1,FUN_0013550c);
  return;
}




// Function: decode_signal @ 0x35562

undefined4 decode_signal(char *param_1)

{
  int iVar1;
  undefined8 uVar2;
  undefined **local_18;
  char *local_10;
  
  iVar1 = strncmp(param_1,"SIG",3);
  local_10 = param_1;
  if (iVar1 == 0) {
    local_10 = param_1 + 3;
  }
  local_18 = &PTR_DAT_0019a9c0;
  while( true ) {
    if (&UNK_0019aa0f < local_18) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar2 = gettext("Unknown signal name: %s");
      error(0,0,uVar2,param_1);
                    /* WARNING: Subroutine does not return */
      FUN_001177f4();
    }
    iVar1 = strcmp(*local_18,local_10);
    if (iVar1 == 0) break;
    local_18 = local_18 + 2;
  }
  return *(undefined4 *)(local_18 + 1);
}




// Function: set_stat_signal @ 0x35629

void set_stat_signal(undefined8 param_1)

{
  undefined4 uVar1;
  
  uVar1 = FUN_00135562(param_1);
  FUN_0013553c(uVar1);
  return;
}




// Function: get_date_or_file @ 0x3564f

undefined8 get_date_or_file(long param_1,undefined8 param_2,char *param_3,undefined1 (*param_4) [16])

{
  char cVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 *puVar6;
  long in_FS_OFFSET;
  undefined1 auVar7 [16];
  stat local_b8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  if ((*param_3 == '/') || (*param_3 == '.')) {
    iVar2 = stat(param_3,&local_b8);
    if (iVar2 != 0) {
      FUN_0013ea03(param_3);
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar3 = gettext("Date sample file not found");
      error(0,0,uVar3);
                    /* WARNING: Subroutine does not return */
      FUN_0013814c(2);
    }
    auVar7 = FUN_0015fd9e(&local_b8);
    *param_4 = auVar7;
  }
  else {
    cVar1 = FUN_0015a8ed(param_4,param_3,0);
    if (cVar1 != '\x01') {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar3 = FUN_0015e84e(param_3);
      uVar4 = FUN_00127abc(*(undefined8 *)*param_4,*(undefined8 *)(*param_4 + 8),0);
      uVar5 = gettext("Substituting %s for unknown date format %s");
      error(0,0,uVar5,uVar4,uVar3);
      *(undefined8 *)(*param_4 + 8) = 0;
      uVar3 = 1;
      goto LAB_0013587e;
    }
    puVar6 = (undefined8 *)FUN_00160b5b(0x28);
    uVar3 = *(undefined8 *)(*param_4 + 8);
    puVar6[1] = *(undefined8 *)*param_4;
    puVar6[2] = uVar3;
    puVar6[3] = param_2;
    uVar3 = FUN_00160cc6(param_3);
    puVar6[4] = uVar3;
    *puVar6 = *(undefined8 *)(param_1 + 8);
    *(undefined8 **)(param_1 + 8) = puVar6;
  }
  uVar3 = 0;
LAB_0013587e:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar3;
}




// Function: report_textual_dates @ 0x3589e

void report_textual_dates(long param_1)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  int iVar4;
  char *__s2;
  undefined8 uVar5;
  undefined8 *local_30;
  
  local_30 = *(undefined8 **)(param_1 + 8);
  while (local_30 != (undefined8 *)0x0) {
    puVar1 = (undefined8 *)*local_30;
    if (DAT_0019f9ac != 0) {
      __s2 = (char *)FUN_00127abc(local_30[1],local_30[2],1);
      iVar4 = strcmp((char *)local_30[4],__s2);
      if (iVar4 != 0) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar2 = local_30[4];
        uVar3 = local_30[3];
        uVar5 = gettext("Option %s: Treating date \'%s\' as %s");
        error(0,0,uVar5,uVar3,uVar2,__s2);
      }
    }
    free((void *)local_30[4]);
    free(local_30);
    local_30 = puVar1;
  }
  return;
}




// Function: tar_help_filter @ 0x359a8

/* WARNING: Removing unreachable block (ram,0x0013614e) */
/* WARNING: Removing unreachable block (ram,0x0013615c) */

undefined8 tar_help_filter(int param_1,undefined8 param_2)

{
  undefined1 *puVar1;
  undefined8 uVar2;
  char *pcVar3;
  size_t sVar4;
  long in_FS_OFFSET;
  undefined8 local_118;
  undefined1 local_68 [8];
  long local_60;
  undefined1 *local_58;
  undefined1 *local_50;
  undefined1 *local_48;
  ulong local_38;
  byte local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  if (param_1 == 0x2000004) {
    _obstack_begin(local_68,0,0,FUN_00160b5b,PTR_free_0019bfc0);
    pcVar3 = (char *)gettext("Valid arguments for the --quoting-style option are:");
    sVar4 = strlen(pcVar3);
    if ((ulong)((long)local_48 - (long)local_50) < sVar4) {
      _obstack_newchunk(local_68,sVar4);
    }
    memcpy(local_50,pcVar3,sVar4);
    local_50 = local_50 + sVar4;
    if ((ulong)((long)local_48 - (long)local_50) < 2) {
      _obstack_newchunk(local_68,2);
    }
    memcpy(local_50,&DAT_001869c4,2);
    local_50 = local_50 + 2;
    FUN_00134eb6(local_68,&DAT_001869c7);
    pcVar3 = (char *)gettext("\n*This* tar defaults to:\n");
    sVar4 = strlen(pcVar3);
    if ((ulong)((long)local_48 - (long)local_50) < sVar4) {
      _obstack_newchunk(local_68,sVar4);
    }
    memcpy(local_50,pcVar3,sVar4);
    local_50 = local_50 + sVar4;
    pcVar3 = (char *)FUN_00135152();
    sVar4 = strlen(pcVar3);
    if ((ulong)((long)local_48 - (long)local_50) < sVar4) {
      _obstack_newchunk(local_68,sVar4);
    }
    memcpy(local_50,pcVar3,sVar4);
    local_50 = local_50 + sVar4;
    if (local_48 == local_50) {
      _obstack_newchunk(local_68,1);
    }
    puVar1 = local_50 + 1;
    *local_50 = 10;
    local_50 = puVar1;
    if (local_48 == puVar1) {
      _obstack_newchunk(local_68,1);
    }
    puVar1 = local_58;
    *local_50 = 0;
    if (local_58 == local_50 + 1) {
      local_18 = local_18 | 2;
    }
    local_50 = (undefined1 *)(~local_38 & (ulong)(local_50 + 1 + local_38));
    if ((ulong)((long)local_48 - local_60) < (ulong)((long)local_50 - local_60)) {
      local_50 = local_48;
    }
    local_58 = local_50;
    local_118 = FUN_00160cc6(puVar1);
    _obstack_free(local_68,0);
  }
  else {
    local_118 = param_2;
    if (param_1 < 0x2000005) {
      if (param_1 == 0xcf) {
        uVar2 = gettext("filter the archive through %s");
        local_118 = FUN_00161277(uVar2,&DAT_00185e64);
      }
      else if (param_1 < 0xd0) {
        if (param_1 == 0x96) {
          uVar2 = gettext("filter the archive through %s");
          local_118 = FUN_00161277(uVar2,&DAT_00185e5c);
        }
        else if (param_1 < 0x97) {
          if (param_1 == 0x95) {
            uVar2 = gettext("filter the archive through %s");
            local_118 = FUN_00161277(uVar2,&DAT_00185e57);
          }
          else if (param_1 < 0x96) {
            if (param_1 == 0x94) {
              uVar2 = gettext("filter the archive through %s");
              local_118 = FUN_00161277(uVar2,&DAT_00185e52);
            }
            else if (param_1 < 0x95) {
              if (param_1 == 0x7a) {
                uVar2 = gettext("filter the archive through %s");
                local_118 = FUN_00161277(uVar2,&DAT_00185e2b);
              }
              else if (param_1 < 0x7b) {
                if (param_1 == 0x6a) {
                  uVar2 = gettext("filter the archive through %s");
                  local_118 = FUN_00161277(uVar2,"bzip2");
                }
                else if (param_1 < 0x6b) {
                  if (param_1 == 0x4a) {
                    uVar2 = gettext("filter the archive through %s");
                    local_118 = FUN_00161277(uVar2,&DAT_00185e61);
                  }
                  else if (param_1 == 0x5a) {
                    uVar2 = gettext("filter the archive through %s");
                    local_118 = FUN_00161277(uVar2,"compress");
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_118;
}




// Function: expand_pax_option @ 0x361aa

/* WARNING: Removing unreachable block (ram,0x0013697c) */
/* WARNING: Removing unreachable block (ram,0x00136990) */

undefined8 expand_pax_option(undefined8 param_1,char *param_2)

{
  int iVar1;
  size_t __n;
  byte *pbVar2;
  byte *__n_00;
  ushort **ppuVar3;
  void *__dest;
  char *pcVar4;
  size_t __n_01;
  undefined8 uVar5;
  long in_FS_OFFSET;
  char *local_1a8;
  byte *local_190;
  ulong local_188;
  undefined8 local_a8;
  undefined8 local_a0;
  undefined1 local_98 [8];
  long local_90;
  char *local_88;
  char *local_80;
  char *local_78;
  ulong local_68;
  byte local_48;
  undefined1 local_38 [40];
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  _obstack_begin(local_98,0,0,FUN_00160b5b,PTR_free_0019bfc0);
  local_1a8 = param_2;
  while (*local_1a8 != '\0') {
    __n = strcspn(local_1a8,",");
    pbVar2 = memchr(local_1a8,0x3d,__n);
    if (pbVar2 == (byte *)0x0) {
      if ((ulong)((long)local_78 - (long)local_80) < __n) {
        _obstack_newchunk(local_98,__n);
      }
      memcpy(local_80,local_1a8,__n);
      local_80 = local_80 + __n;
    }
    else {
      __n_00 = pbVar2 + (1 - (long)local_1a8);
      if (local_78 + -(long)local_80 < __n_00) {
        _obstack_newchunk(local_98,__n_00);
      }
      memcpy(local_80,local_1a8,(size_t)__n_00);
      local_80 = local_80 + (long)__n_00;
      local_188 = __n - (long)__n_00;
      while ((local_190 = pbVar2 + 1, *local_190 != 0 &&
             (ppuVar3 = __ctype_b_loc(), ((*ppuVar3)[*local_190] & 0x2000) != 0))) {
        local_188 = local_188 - 1;
        pbVar2 = local_190;
      }
      if ((*local_190 == 0x7b) && (local_190[local_188 - 1] == 0x7d)) {
        __dest = (void *)FUN_00160b5b(local_188);
        memcpy(__dest,pbVar2 + 2,local_188 - 2);
        *(undefined1 *)((long)__dest + (local_188 - 2)) = 0;
        iVar1 = FUN_0013564f(param_1,"--pax-option",__dest,&local_a8);
        if (iVar1 == 0) {
          pcVar4 = (char *)FUN_00129f2e(local_a8,local_a0,local_38);
          __n_01 = strlen(pcVar4);
          if ((ulong)((long)local_78 - (long)local_80) < __n_01) {
            _obstack_newchunk(local_98,__n_01);
          }
          memcpy(local_80,pcVar4,__n_01);
          local_80 = local_80 + __n_01;
        }
        else {
          if ((ulong)((long)local_78 - (long)local_80) < local_188) {
            _obstack_newchunk(local_98,local_188);
          }
          memcpy(local_80,local_190,local_188);
          local_80 = local_80 + local_188;
        }
        free(__dest);
      }
      else {
        if ((ulong)((long)local_78 - (long)local_80) < local_188) {
          _obstack_newchunk(local_98,local_188);
        }
        memcpy(local_80,local_190,local_188);
        local_80 = local_80 + local_188;
      }
    }
    local_1a8 = local_1a8 + __n;
    if (*local_1a8 != '\0') {
      if (local_78 == local_80) {
        _obstack_newchunk(local_98,1);
      }
      *local_80 = *local_1a8;
      local_1a8 = local_1a8 + 1;
      local_80 = local_80 + 1;
    }
  }
  if (local_78 == local_80) {
    _obstack_newchunk(local_98,1);
  }
  pcVar4 = local_88;
  *local_80 = '\0';
  if (local_88 == local_80 + 1) {
    local_48 = local_48 | 2;
  }
  local_80 = (char *)(~local_68 & (ulong)(local_80 + 1 + local_68));
  if ((ulong)((long)local_78 - local_90) < (ulong)((long)local_80 - local_90)) {
    local_80 = local_78;
  }
  local_88 = local_80;
  uVar5 = FUN_00160cc6(pcVar4);
  _obstack_free(local_98,0);
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return uVar5;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: parse_owner_group @ 0x369f0

ulong parse_owner_group(char *param_1,ulong param_2,undefined8 *param_3)

{
  char *pcVar1;
  int iVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  long in_FS_OFFSET;
  ulong local_58;
  undefined1 local_50 [8];
  ulong local_48;
  char *local_40;
  char *local_38;
  char *local_30;
  char *local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_58 = 0xffffffffffffffff;
  local_40 = (char *)0x0;
  local_38 = (char *)0x0;
  local_30 = strchr(param_1,0x3a);
  if (local_30 == (char *)0x0) {
    if ((*param_1 < '0') || ('9' < *param_1)) {
      iVar2 = 4;
    }
    else {
      iVar2 = FUN_00160e19(param_1,local_50,10,&local_48,&DAT_001869f3);
    }
    if (iVar2 == 0) {
      pcVar1 = param_1;
      if (local_48 <= param_2) {
        local_58 = local_48;
        pcVar1 = local_38;
      }
    }
    else {
      pcVar1 = param_1;
      if (iVar2 != 1) {
        local_40 = param_1;
        pcVar1 = local_38;
      }
    }
  }
  else {
    local_28 = local_30 + 1;
    *local_30 = '\0';
    if (*param_1 != '\0') {
      local_40 = param_1;
    }
    pcVar1 = local_38;
    if ((local_28 != (char *)0x0) &&
       ((iVar2 = FUN_00160e19(local_28,local_50,10,&local_58,&DAT_001869f3), iVar2 != 0 ||
        (pcVar1 = local_38, param_2 < local_58)))) {
      local_38 = local_28;
      pcVar1 = local_38;
    }
  }
  local_38 = pcVar1;
  if (local_38 == (char *)0x0) {
    if (local_40 != (char *)0x0) {
      *param_3 = local_40;
    }
    if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
      return local_58;
    }
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  if (DAT_0019fdc8 != (code *)0x0) {
    (*DAT_0019fdc8)();
  }
  uVar3 = gettext("Invalid owner or group ID");
  uVar4 = FUN_0015e548(local_38);
  error(0,0,"%s: %s",uVar4,uVar3);
                    /* WARNING: Subroutine does not return */
  FUN_001177f4();
}




// Function: set_old_files_option @ 0x36bbb

void set_old_files_option(uint param_1,undefined8 param_2)

{
  int iVar1;
  long lVar2;
  
  lVar2 = FUN_0013521d(9,param_2);
  if (lVar2 != 0) {
    iVar1 = FUN_00135388(param_2,lVar2);
    if ((iVar1 != 0) && (param_1 != DAT_0019f8d8)) {
      FUN_001351b6((&PTR_s___overwrite_dir_0019aa20)[(int)param_1],
                   (&PTR_s___overwrite_dir_0019aa20)[DAT_0019f8d8]);
    }
  }
  DAT_0019f8d8 = param_1;
  return;
}




// Function: parse_opt @ 0x36c56

undefined8 parse_opt(int param_1,char *param_2,long *param_3)

{
  int iVar1;
  char *pcVar2;
  long lVar3;
  ulong uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  long in_FS_OFFSET;
  char *local_78;
  int local_5c;
  ulong local_58;
  char *local_50;
  long *local_48;
  void *local_40;
  void *local_38;
  long local_30;
  long local_28;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_48 = (long *)param_3[5];
  if (param_1 == 0x1000005) {
    if (*(int *)*local_48 == 2) {
      uVar7 = *(undefined8 *)(*local_48 + 0x10);
      uVar6 = *(undefined8 *)(*local_48 + 8);
      uVar5 = gettext("%s:%lu: location of the error");
      error(0,0,uVar5,uVar6,uVar7);
    }
    else if (*(int *)*local_48 == 0) {
      uVar7 = *(undefined8 *)(*local_48 + 8);
      uVar6 = gettext("error parsing %s");
      error(0,0,uVar6,uVar7);
    }
                    /* WARNING: Subroutine does not return */
    exit(0x40);
  }
  if (param_1 < 0x1000006) {
    pcVar2 = DAT_0019f9a0;
    if (param_1 < 0xd0) {
      local_78 = param_2;
      switch(param_1) {
      case 0:
        FUN_0012c829(param_2);
        pcVar2 = DAT_0019f9a0;
        break;
      default:
        goto switchD_00136ce6_caseD_1;
      case 0x30:
      case 0x31:
      case 0x32:
      case 0x33:
      case 0x34:
      case 0x35:
      case 0x36:
      case 0x37:
        uVar7 = gettext("Options \'-[0-7][lmh]\' not supported by *this* tar");
        argp_error(param_3,uVar7);
                    /* WARNING: Subroutine does not return */
        exit(0x40);
      case 0x41:
        FUN_001353ea(2);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x42:
        DAT_0019f94d = 1;
        break;
      case 0x46:
        DAT_0019f8fc = 1;
        DAT_0019f8c0 = param_2;
        break;
      case 0x48:
        FUN_00134c7e(param_2);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x49:
        FUN_0013545f(param_2,*local_48);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x4a:
        FUN_0013545f(&DAT_00185e61,*local_48);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x4b:
        FUN_0013521d(5,*local_48);
        FUN_0012d6a2(param_2);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x4c:
        iVar1 = FUN_00160e19(param_2,&local_50,10,&local_58,"bBcGgkKMmPTtw");
        if (iVar1 != 0) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar7 = gettext("Invalid tape length");
          uVar6 = FUN_0015e548(param_2);
          error(0,0,"%s: %s",uVar6,uVar7);
                    /* WARNING: Subroutine does not return */
          FUN_0013814c(2);
        }
        if ((param_2 < local_50) &&
           (pcVar2 = strchr("bBcGgkKMmPTtw",(int)local_50[-1]), pcVar2 == (char *)0x0)) {
          DAT_0019f990 = (double)local_58 * DAT_00187670;
        }
        else {
          DAT_0019f990 = (double)local_58;
        }
        DAT_0019f8fc = 1;
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x4d:
        DAT_0019f8fc = 1;
        break;
      case 0x4e:
        DAT_0019f87c = 1;
      case 0x99:
        if (-1 < DAT_0019f908) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar7 = gettext("More than one threshold date");
          error(0,0,uVar7);
                    /* WARNING: Subroutine does not return */
          FUN_0013814c(2);
        }
        if (param_1 == 0x99) {
          pcVar2 = "--newer-mtime";
        }
        else {
          pcVar2 = "--after-date";
        }
        FUN_0013564f(local_48,pcVar2,param_2,&DAT_0019f900);
        FUN_0013521d(3,*local_48);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x4f:
        DAT_0019f998 = 1;
        break;
      case 0x50:
        FUN_0013521d(8,*local_48);
        DAT_0019f878 = 1;
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x52:
        DAT_0019f88c = 1;
        break;
      case 0x53:
        DAT_0019f979 = 1;
        break;
      case 0x55:
        FUN_00136bbb(3,*local_48);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x56:
        DAT_0019f9c0 = param_2;
        break;
      case 0x57:
        FUN_0013521d(4,*local_48);
        DAT_0019f9b0 = 1;
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x5a:
        FUN_0013545f("compress",*local_48);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x61:
        *(undefined1 *)((long)local_48 + 0x12) = 1;
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x62:
        iVar1 = FUN_00160e19(param_2,0,10,&local_50,&DAT_001869f3);
        if ((((iVar1 != 0) || (DAT_0019f868 = (int)local_50, (char *)(long)DAT_0019f868 != local_50)
             ) || (DAT_0019f868 < 1)) ||
           (DAT_0019f870 = (char *)((long)local_50 << 9), pcVar2 = DAT_0019f9a0,
           (char *)((ulong)local_50 & 0x7fffffffffffff) != local_50)) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar7 = gettext("Invalid blocking factor");
          uVar6 = FUN_0015e548(param_2);
          error(0,0,"%s: %s",uVar6,uVar7);
                    /* WARNING: Subroutine does not return */
          FUN_0013814c(2);
        }
        break;
      case 99:
        FUN_001353ea(3);
        pcVar2 = DAT_0019f9a0;
        break;
      case 100:
        FUN_001353ea(5);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x66:
        if (DAT_0019fbe0 == DAT_0019fc38) {
          DAT_0019fbd8 = FUN_00160a5f(DAT_0019fbd8,&DAT_0019fc38,8);
        }
        lVar3 = DAT_0019fbe0 * 8;
        DAT_0019fbe0 = DAT_0019fbe0 + 1;
        *(char **)(DAT_0019fbd8 + lVar3) = param_2;
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x67:
        FUN_0013521d(2,*local_48);
        DAT_0019f87c = 1;
        DAT_0019f8e0 = param_2;
      case 0x47:
        DAT_0019f8be = 1;
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x68:
        DAT_0019f8a0 = 1;
        break;
      case 0x69:
        DAT_0019f8bd = 1;
        break;
      case 0x6a:
        FUN_0013545f("bzip2",*local_48);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x6b:
        FUN_00136bbb(4,*local_48);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x6c:
        DAT_0019fc34 = 1;
        break;
      case 0x6d:
        DAT_0019f99a = 1;
        break;
      case 0x6e:
        DAT_0019fc14 = 1;
        break;
      case 0x6f:
        *(undefined1 *)(local_48 + 2) = 1;
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x70:
        DAT_0019f960 = 1;
        break;
      case 0x72:
        FUN_001353ea(1);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x73:
        FUN_0013521d(6,*local_48);
        DAT_0019f958 = 1;
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x74:
        FUN_001353ea(7);
        DAT_0019f9ac = DAT_0019f9ac + 1;
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x75:
        FUN_001353ea(8);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x76:
        DAT_0019f9ac = DAT_0019f9ac + 1;
        DAT_0019ec84 = DAT_0019ec84 | 0x583000;
        break;
      case 0x77:
        DAT_0019f8c8 = 1;
        break;
      case 0x78:
        FUN_001353ea(6);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x7a:
        FUN_0013545f(&DAT_00185e2b,*local_48);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x80:
        FUN_00134c7e("posix");
        DAT_0019f968 = 1;
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x81:
        if (param_2 == (char *)0x0) {
          DAT_0019f880 = 1;
        }
        else {
          lVar3 = FUN_00147c61("--atime-preserve",param_2,&PTR_s_replace_0019a950,&DAT_001867c8,4,
                               PTR_FUN_0019ee48);
          DAT_0019f880 = *(undefined4 *)(&DAT_001867c8 + lVar3 * 4);
          pcVar2 = DAT_0019f9a0;
        }
        break;
      case 0x82:
        DAT_0019f884 = 1;
        if (param_2 != (char *)0x0) {
          local_48[4] = (long)param_2;
          pcVar2 = DAT_0019f9a0;
        }
        break;
      case 0x83:
        DAT_0019f8ec = 1;
        break;
      case 0x84:
        if (param_2 == (char *)0x0) {
          DAT_0019f890 = 10;
        }
        else {
          if (*param_2 == '.') {
            FUN_0010f825(&DAT_00186b47);
            local_78 = param_2 + 1;
          }
          uVar4 = strtoul(local_78,&local_50,0);
          DAT_0019f890 = (undefined4)uVar4;
          pcVar2 = DAT_0019f9a0;
          if (*local_50 != '\0') {
            if (DAT_0019fdc8 != (code *)0x0) {
              (*DAT_0019fdc8)();
            }
            uVar7 = gettext("--checkpoint value is not an integer");
            error(0,0,uVar7);
                    /* WARNING: Subroutine does not return */
            FUN_001177f4();
          }
        }
        break;
      case 0x85:
        FUN_0010f825(param_2);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x86:
        DAT_0019f910 = 2;
        break;
      case 0x87:
        DAT_0019fc31 = 1;
        break;
      case 0x88:
        DAT_0019f8a1 = 1;
        break;
      case 0x89:
        FUN_001353ea(4);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x8a:
        DAT_0019fdf8 = 1;
        break;
      case 0x8b:
        DAT_0019f87a = 1;
        break;
      case 0x8c:
        local_28 = FUN_001369f0(param_2,0xffffffff,&DAT_0019f8b0);
        pcVar2 = DAT_0019f9a0;
        if (local_28 == -1) {
          DAT_0019f8b8 = 0xffffffff;
          if (DAT_0019f8b0 != 0) {
            FUN_0012c261(DAT_0019f8b0,&DAT_0019f8b8);
            pcVar2 = DAT_0019f9a0;
          }
        }
        else {
          DAT_0019f8b8 = (undefined4)local_28;
        }
        break;
      case 0x8d:
        FUN_0012938e(param_2);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x8e:
        DAT_0019f9a8 = 1;
        break;
      case 0x8f:
        DAT_0019f8bc = 1;
        break;
      case 0x90:
        DAT_0019fbf0 = param_2;
        break;
      case 0x91:
        DAT_0019f8dc = 1;
        break;
      case 0x92:
        FUN_00136bbb(6,*local_48);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x93:
        uVar4 = strtoul(param_2,&local_50,10);
        DAT_0019f8e8 = (undefined4)uVar4;
        pcVar2 = DAT_0019f9a0;
        if (*local_50 != '\0') {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar7 = gettext("Invalid incremental level value");
          error(0,0,uVar7);
                    /* WARNING: Subroutine does not return */
          FUN_0013814c(2);
        }
        break;
      case 0x94:
        FUN_0013545f(&DAT_00185e52,*local_48);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x95:
        FUN_0013545f(&DAT_00185e57,*local_48);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x96:
        FUN_0013545f(&DAT_00185e5c,*local_48);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x97:
        DAT_0019f8f0 = FUN_00155658(param_2);
        if (DAT_0019f8f0 == 0) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar7 = gettext("Invalid mode given on option");
          error(0,0,uVar7);
                    /* WARNING: Subroutine does not return */
          FUN_001177f4();
        }
        DAT_0019f8f8 = umask(0);
        umask(DAT_0019f8f8);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x98:
        FUN_0013564f(local_48,"--mtime",param_2,&DAT_0019f920);
        pcVar2 = DAT_0019f9a0;
        if (DAT_0019f910 == 0) {
          DAT_0019f910 = 1;
        }
        break;
      case 0x9a:
        DAT_0019f968 = 0xffffffff;
        break;
      case 0x9b:
        *(undefined1 *)((long)local_48 + 0x12) = 0;
        pcVar2 = DAT_0019f9a0;
        break;
      case 0x9c:
        DAT_0019f8ec = 0;
        break;
      case 0x9d:
        DAT_0019fc31 = 0;
        break;
      case 0x9e:
        DAT_0019f9a8 = 0;
        break;
      case 0x9f:
        FUN_00136bbb(1,*local_48);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xa0:
        for (; pcVar2 = DAT_0019f9a0, *local_78 != '\0'; local_78 = local_78 + 1) {
          FUN_0015c9cd(0,(int)*local_78,0);
        }
        break;
      case 0xa1:
        DAT_0019f95c = 0xffffffff;
        break;
      case 0xa2:
        DAT_0019f960 = 0xffffffff;
        break;
      case 0xa3:
        DAT_0019fc14 = 0;
        break;
      case 0xa4:
        DAT_0019f964 = 0xffffffff;
        break;
      case 0xa5:
        FUN_00134d36(0xffffffff);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xa6:
        DAT_0019f934 = 1;
        break;
      case 0xa7:
        FUN_0013521d(1,*local_48);
        if (param_2 == (char *)0x0) {
          DAT_0019f8d0 = (char *)0x1;
          pcVar2 = DAT_0019f9a0;
        }
        else {
          iVar1 = FUN_00160e19(param_2,0,10,&local_50,&DAT_001869f3);
          if (iVar1 != 0) {
            if (DAT_0019fdc8 != (code *)0x0) {
              (*DAT_0019fdc8)();
            }
            uVar7 = gettext("Invalid number");
            uVar6 = FUN_0015e548(param_2);
            error(0,0,"%s: %s",uVar6,uVar7);
                    /* WARNING: Subroutine does not return */
            FUN_001177f4();
          }
          DAT_0019f8d0 = local_50;
          pcVar2 = DAT_0019f9a0;
        }
        break;
      case 0xa8:
        FUN_00134c7e(&DAT_00184244);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xa9:
        DAT_0019f935 = 1;
        break;
      case 0xaa:
        FUN_0013521d(7,*local_48);
        DAT_0019f936 = 1;
        DAT_0019f938 = param_2;
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xab:
        FUN_00136bbb(0,*local_48);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xac:
        FUN_00136bbb(2,*local_48);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xad:
        local_30 = FUN_001369f0(param_2,0xffffffff,&DAT_0019f940);
        pcVar2 = DAT_0019f9a0;
        if (local_30 == -1) {
          DAT_0019f948 = 0xffffffff;
          if (DAT_0019f940 != 0) {
            FUN_0012c173(DAT_0019f940,&DAT_0019f948);
            pcVar2 = DAT_0019f9a0;
          }
        }
        else {
          DAT_0019f948 = (undefined4)local_30;
        }
        break;
      case 0xae:
        FUN_00129234(param_2);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xaf:
        local_38 = (void *)FUN_001361aa(local_48,param_2);
        *(undefined1 *)((long)local_48 + 0x11) = 1;
        FUN_0011cc78(local_38);
        free(local_38);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xb0:
        FUN_00134c7e("posix");
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xb1:
        for (; pcVar2 = DAT_0019f9a0, *local_78 != '\0'; local_78 = local_78 + 1) {
          FUN_0015c9cd(0,(int)*local_78,1);
        }
        break;
      case 0xb2:
        FUN_00135084(param_2);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xb3:
        iVar1 = FUN_00160e19(param_2,0,10,&local_50,"bBcGgkKMmPTtw");
        if (iVar1 != 0) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar7 = gettext("Invalid record size");
          uVar6 = FUN_0015e548(param_2);
          error(0,0,"%s: %s",uVar6,uVar7);
                    /* WARNING: Subroutine does not return */
          FUN_0013814c(2);
        }
        DAT_0019f870 = local_50;
        if (((ulong)local_50 & 0x1ff) != 0) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar7 = gettext("Record size must be a multiple of %d.");
          error(0,0,uVar7,0x200);
                    /* WARNING: Subroutine does not return */
          FUN_0013814c(2);
        }
        DAT_0019f868 = (int)((ulong)local_50 >> 9);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xb4:
        DAT_0019f94c = 1;
        break;
      case 0xb5:
        DAT_0019f94e = 1;
        break;
      case 0xb6:
        DAT_0019f9a9 = 1;
        break;
      case 0xb7:
        PTR_s__usr_local_libexec_rmt_0019ece0 = param_2;
        break;
      case 0xb8:
        DAT_0019f950 = param_2;
        break;
      case 0xb9:
        DAT_0019f95c = 1;
        break;
      case 0xba:
        FUN_00134c7e("posix");
        DAT_0019f964 = 1;
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xbb:
        local_40 = (void *)FUN_00135152();
        printf("%s\n",local_40);
        FUN_0014e014();
        free(local_40);
                    /* WARNING: Subroutine does not return */
        exit(0);
      case 0xbc:
        DAT_0019f978 = 1;
        break;
      case 0xbd:
        FUN_00123f6b();
        FUN_0014e014();
                    /* WARNING: Subroutine does not return */
        exit(0);
      case 0xbe:
        DAT_0019fc30 = 1;
        break;
      case 0xbf:
        FUN_00136bbb(5,*local_48);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xc0:
        lVar3 = FUN_00147c61("--sort",param_2,&PTR_DAT_0019a980,&DAT_0019cb40,4,PTR_FUN_0019ee48);
        DAT_0019fc2c = *(undefined4 *)(&DAT_0019cb40 + lVar3 * 4);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xc1:
        lVar3 = FUN_00147c61("--hole-detection",param_2,&PTR_DAT_0019a9a0,&DAT_00186a30,4,
                             PTR_FUN_0019ee48);
        DAT_0019f984 = *(undefined4 *)(&DAT_00186a30 + lVar3 * 4);
        DAT_0019f979 = 1;
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xc2:
        DAT_0019f979 = 1;
        uVar4 = strtoul(param_2,&local_50,10);
        DAT_0019f97c = (undefined4)uVar4;
        pcVar2 = DAT_0019f9a0;
        if (*local_50 != '\0') {
          if (*local_50 != '.') {
            if (DAT_0019fdc8 != (code *)0x0) {
              (*DAT_0019fdc8)();
            }
            uVar7 = gettext("Invalid sparse version value");
            error(0,0,uVar7);
                    /* WARNING: Subroutine does not return */
            FUN_0013814c(2);
          }
          uVar4 = strtoul(local_50 + 1,&local_50,10);
          DAT_0019f980 = (undefined4)uVar4;
          pcVar2 = DAT_0019f9a0;
          if (*local_50 != '\0') {
            if (DAT_0019fdc8 != (code *)0x0) {
              (*DAT_0019fdc8)();
            }
            uVar7 = gettext("Invalid sparse version value");
            error(0,0,uVar7);
                    /* WARNING: Subroutine does not return */
            FUN_0013814c(2);
          }
        }
        break;
      case 0xc3:
        iVar1 = FUN_00160e19(param_2,0,10,&local_50,&DAT_001869f3);
        if (iVar1 != 0) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar7 = gettext("Invalid number of elements");
          uVar6 = FUN_0015e548(param_2);
          error(0,0,"%s: %s",uVar6,uVar7);
                    /* WARNING: Subroutine does not return */
          FUN_0013814c(2);
        }
        DAT_0019f970 = local_50;
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xc4:
        DAT_0019f884 = 1;
        local_48[3] = (long)param_2;
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xc5:
        FUN_001353ea(9);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xc6:
        if (param_2 == (char *)0x0) {
          DAT_0019f999 = 1;
        }
        else {
          FUN_00135629(param_2);
          pcVar2 = DAT_0019f9a0;
        }
        break;
      case 199:
        pcVar2 = param_2;
        if (DAT_0019f9a0 != (char *)0x0) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar7 = gettext("Only one --to-command option allowed");
          error(0,0,uVar7);
                    /* WARNING: Subroutine does not return */
          FUN_0013814c(2);
        }
        break;
      case 200:
        FUN_0013ae0b(param_2);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xc9:
        DAT_0019f879 = 1;
        break;
      case 0xca:
        DAT_0019f9b8 = param_2;
        break;
      case 0xcb:
        FUN_0013c9ee(param_2);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xcc:
        FUN_00134d36(1);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xcd:
      case 0xce:
        FUN_00134d36(1);
        FUN_0013d0e5(param_2,param_1 == 0xce);
        pcVar2 = DAT_0019f9a0;
        break;
      case 0xcf:
        FUN_0013545f(&DAT_00185e64,*local_48);
        pcVar2 = DAT_0019f9a0;
      }
LAB_0013812a:
      DAT_0019f9a0 = pcVar2;
      uVar7 = 0;
      goto LAB_0013812f;
    }
    if (param_1 == 0x1000003) {
      if (*(long *)(*param_3 + 0x20) != 0) {
        local_5c = 0;
        while (pcVar2 = DAT_0019f9a0,
              *(long *)((long)local_5c * 0x20 + *(long *)(*param_3 + 0x20)) != 0) {
          *(long *)(param_3[6] + (long)local_5c * 8) = param_3[5];
          local_5c = local_5c + 1;
        }
      }
      goto LAB_0013812a;
    }
  }
switchD_00136ce6_caseD_1:
  uVar7 = 7;
LAB_0013812f:
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar7;
}




// Function: usage @ 0x3814c

void usage(int param_1)

{
  argp_help(&PTR_DAT_0019eb60,stderr,4,DAT_0019fe48);
  FUN_0014e014();
                    /* WARNING: Subroutine does not return */
  exit(param_1);
}




// Function: find_argp_option_key @ 0x38192

long * find_argp_option_key(long *param_1,int param_2)

{
  long *local_10;
  
  for (local_10 = param_1;
      (((*local_10 != 0 || ((int)local_10[1] != 0)) || (local_10[2] != 0)) ||
      (((int)local_10[3] != 0 || (local_10[4] != 0)))); local_10 = local_10 + 6) {
    if (param_2 == (int)local_10[1]) {
      return local_10;
    }
  }
  return (long *)0x0;
}




// Function: find_argp_option @ 0x381fd

long find_argp_option(undefined8 *param_1,undefined4 param_2)

{
  long local_18;
  long *local_10;
  
  local_18 = FUN_00138192(*param_1,param_2);
  if ((local_18 == 0) && (param_1[4] != 0)) {
    local_10 = (long *)param_1[4];
    while ((*local_10 != 0 &&
           (local_18 = FUN_00138192(*(undefined8 *)*local_10,param_2), local_18 == 0))) {
      local_10 = local_10 + 4;
    }
  }
  return local_18;
}




// Function: more_options @ 0x3828e

void more_options(undefined4 param_1,undefined8 param_2,undefined8 param_3)

{
  long in_FS_OFFSET;
  undefined8 local_38;
  undefined8 local_30;
  undefined1 local_28;
  undefined1 local_27;
  undefined1 local_26;
  undefined8 local_20;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_30 = 0;
  local_28 = 0;
  local_27 = 0;
  local_26 = 0;
  local_20 = 0;
  local_18 = 0;
  local_38 = param_3;
  argp_parse(&PTR_DAT_0019c920,param_1,param_2,0x2a,0,&local_38);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: parse_default_options @ 0x38321

void parse_default_options(long *param_1)

{
  char *pcVar1;
  char cVar2;
  int iVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  long in_FS_OFFSET;
  undefined1 local_13c [4];
  char *local_138;
  long local_130;
  undefined4 local_128 [2];
  char *local_120;
  undefined8 local_118;
  undefined8 local_110;
  long local_108;
  undefined8 *local_100;
  undefined8 local_f8;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_138 = getenv("TAR_OPTIONS");
  local_128[0] = 0;
  local_120 = "TAR_OPTIONS";
  local_118 = 0;
  local_110 = 0;
  if (local_138 != (char *)0x0) {
    local_f8 = 1;
    iVar3 = FUN_001461a6(local_138,&local_108,0x2000e46);
    if (iVar3 != 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar4 = FUN_0014647f(&local_108);
      uVar5 = gettext("cannot split TAR_OPTIONS: %s");
      error(0,0,uVar5,uVar4);
                    /* WARNING: Subroutine does not return */
      FUN_001177f4();
    }
    if (local_108 != 0) {
      *local_100 = DAT_0019fe48;
      local_130 = *param_1;
      *param_1 = (long)local_128;
      iVar3 = argp_parse(&PTR_DAT_0019eb60,(int)local_108 + (int)local_f8,local_100,0x28,local_13c,
                         param_1);
      if (iVar3 != 0) {
                    /* WARNING: Subroutine does not return */
        abort();
      }
      *param_1 = local_130;
      cVar2 = FUN_00134a5e();
      if (cVar2 != '\0') {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        pcVar1 = local_120;
        uVar4 = gettext("non-option arguments in %s");
        error(0,0,uVar4,pcVar1);
                    /* WARNING: Subroutine does not return */
        FUN_0013814c(2);
      }
      local_108 = 0;
    }
    FUN_00146399(&local_108);
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: decode_options @ 0x3854d

void decode_options(int param_1,undefined8 *param_2)

{
  long *plVar1;
  undefined8 *puVar2;
  char cVar3;
  int iVar4;
  uint uVar5;
  char *pcVar6;
  size_t sVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  uint uVar10;
  long in_FS_OFFSET;
  undefined8 *local_108;
  int local_fc;
  int local_ec;
  int local_e8;
  uint local_e4;
  undefined8 *local_e0;
  undefined8 *local_d8;
  char *local_d0;
  undefined8 *local_c8;
  long local_c0;
  int *local_b8;
  undefined8 local_b0;
  ulong local_a8;
  int *local_a0;
  undefined8 local_98;
  void *local_90;
  int *local_88;
  undefined8 local_80;
  undefined4 local_78 [2];
  undefined8 local_70;
  undefined8 local_68;
  undefined8 local_60;
  undefined4 *local_58;
  undefined8 local_50;
  char local_48;
  char local_47;
  char local_46;
  char *local_40;
  undefined8 local_38;
  undefined1 local_23;
  char local_22;
  undefined1 local_21;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_78[0] = 1;
  local_70 = 0;
  local_68 = 0;
  local_60 = 0;
  local_58 = local_78;
  local_50 = 0;
  local_48 = '\0';
  local_47 = '\0';
  local_46 = '\0';
  local_40 = (char *)0x0;
  local_38 = 0;
  FUN_0014d302(&DAT_00187004,&PTR_s_John_Gilmore_0019eba0);
  local_40 = getenv("SIMPLE_BACKUP_SUFFIX");
  pcVar6 = getenv("POSIXLY_CORRECT");
  DAT_0019f9c8 = pcVar6 != (char *)0x0;
  DAT_0019f860 = 0;
  DAT_0019f864 = 0;
  DAT_0019f868 = 0x14;
  DAT_0019f870 = 0x2800;
  DAT_0019f8a8 = FUN_0014e53e();
  DAT_0019f984 = 0;
  DAT_0019f900 = 0x8000000000000000;
  DAT_0019f908 = -1;
  DAT_0019f920 = 0x8000000000000000;
  DAT_0019f928 = -1;
  DAT_0019f930 = 8;
  DAT_0019fc28 = 1;
  DAT_0019f97c = 1;
  DAT_0019f980 = 0;
  DAT_0019fc2c = 0;
  DAT_0019f948 = 0xffffffff;
  DAT_0019f940 = 0;
  DAT_0019f8b8 = 0xffffffff;
  DAT_0019f8b0 = 0;
  DAT_0019f8ec = 1;
  DAT_0019f8e8 = -1;
  DAT_0019fc14 = 0xffffffff;
  local_108 = param_2;
  local_fc = param_1;
  if ((1 < param_1) && (*(char *)param_2[1] != '-')) {
    local_23 = 0x2d;
    local_21 = 0;
    sVar7 = strlen((char *)param_2[1]);
    iVar4 = param_1 + (int)sVar7;
    local_e8 = iVar4 + -1;
    local_c8 = (undefined8 *)FUN_00160b5b((long)iVar4 << 3);
    local_d8 = local_c8 + 1;
    *local_c8 = *param_2;
    local_e0 = param_2 + 2;
    for (local_d0 = (char *)param_2[1]; puVar2 = local_d8, *local_d0 != '\0';
        local_d0 = local_d0 + 1) {
      local_22 = *local_d0;
      local_d8 = local_d8 + 1;
      uVar8 = FUN_00160cc6(&local_23);
      *puVar2 = uVar8;
      local_c0 = FUN_001381fd(&PTR_DAT_0019eb60,(int)*local_d0);
      if ((local_c0 != 0) && (*(long *)(local_c0 + 0x10) != 0)) {
        if (param_2 + param_1 <= local_e0) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          cVar3 = *local_d0;
          uVar8 = gettext("Old option \'%c\' requires an argument.");
          error(0,0,uVar8,(int)cVar3);
                    /* WARNING: Subroutine does not return */
          FUN_0013814c(2);
        }
        *local_d8 = *local_e0;
        local_e0 = local_e0 + 1;
        local_d8 = local_d8 + 1;
      }
    }
    while (local_e0 < param_2 + param_1) {
      *local_d8 = *local_e0;
      local_e0 = local_e0 + 1;
      local_d8 = local_d8 + 1;
    }
    *local_d8 = 0;
    local_fc = local_e8;
    local_108 = local_c8;
  }
  FUN_00138321(&local_58);
  iVar4 = argp_parse(&PTR_DAT_0019eb60,local_fc,local_108,8,&local_ec,&local_58);
  if (iVar4 != 0) {
                    /* WARNING: Subroutine does not return */
    exit(2);
  }
  if (local_48 != '\0') {
    if (DAT_0019f860 == 3) {
      FUN_00134c7e(&DAT_00184244);
    }
    else {
      DAT_0019f95c = 0xffffffff;
    }
  }
  for (; local_ec < local_fc; local_ec = local_ec + 1) {
    FUN_0012c829(local_108[local_ec]);
  }
  if (DAT_0019f864 == 0) {
    if (local_47 == '\0') {
      DAT_0019f864 = 6;
    }
    else {
      DAT_0019f864 = 4;
    }
  }
  if ((((DAT_0019f9c0 != (char *)0x0) && (DAT_0019f860 == 3)) || (DAT_0019f8be != '\0')) ||
     ((DAT_0019f8fc != '\0' || (DAT_0019f979 != '\0')))) {
    FUN_00134dad(0x54);
  }
  if (DAT_0019f8d0 != 0) {
    cVar3 = FUN_00134a5e();
    if (cVar3 != '\x01') {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar8 = gettext("--occurrence is meaningless without a file list");
      error(0,0,uVar8);
                    /* WARNING: Subroutine does not return */
      FUN_0013814c(2);
    }
    if (((&DAT_0019cb60)[DAT_0019f860] & 0x10) == 0) {
      iVar4 = FUN_0013534c(1);
      if (iVar4 == 0) {
        DAT_0019f8d0 = 0;
      }
      else {
        uVar8 = FUN_00134e1c(DAT_0019f860);
        FUN_001351b6("--occurrence",uVar8);
      }
    }
  }
  plVar1 = DAT_0019fbd8;
  if (DAT_0019fbe0 == 0) {
    DAT_0019fbe0 = 1;
    pcVar6 = getenv("TAPE");
    *plVar1 = (long)pcVar6;
    if (*DAT_0019fbd8 == 0) {
      *DAT_0019fbd8 = (long)&DAT_001867e7;
    }
  }
  if ((1 < DAT_0019fbe0) && (DAT_0019f8fc != '\x01')) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar8 = gettext("Multiple archive files require \'-M\' option");
    error(0,0,uVar8);
                    /* WARNING: Subroutine does not return */
    FUN_0013814c(2);
  }
  if ((DAT_0019f8e0 != 0) && (-1 < DAT_0019f908)) {
    local_b8 = (int *)FUN_00135327(2);
    local_b0 = FUN_00135327(3);
    iVar4 = FUN_00135388(local_b8,local_b0);
    if (iVar4 == 0) {
      if (*local_b8 == 1) {
        DAT_0019f8e0 = 0;
      }
      else {
        memset(&DAT_0019f900,0,0x10);
      }
    }
    else {
      FUN_001351b6("--listed-incremental","--newer");
    }
  }
  if ((DAT_0019f8e8 != -1) && (DAT_0019f8e0 == 0)) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar8 = gettext("--level is meaningless without --listed-incremental");
    error(0,0,uVar8);
  }
  if ((DAT_0019f9c0 != (char *)0x0) && ((DAT_0019f864 == 6 || (DAT_0019f864 == 2)))) {
    if (DAT_0019f8fc == '\0') {
      local_a8 = 99;
    }
    else {
      local_a8 = 0x51;
    }
    sVar7 = strlen(DAT_0019f9c0);
    if (local_a8 < sVar7) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar8 = FUN_0015e548(DAT_0019f9c0);
      uVar9 = ngettext("%s: Volume label is too long (limit is %lu byte)",
                       "%s: Volume label is too long (limit is %lu bytes)",local_a8);
      error(0,0,uVar9,uVar8,local_a8);
                    /* WARNING: Subroutine does not return */
      FUN_0013814c(2);
    }
  }
  if (DAT_0019f9b0 != '\0') {
    if (DAT_0019f8fc != '\0') {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar8 = gettext("Cannot verify multi-volume archives");
      error(0,0,uVar8);
                    /* WARNING: Subroutine does not return */
      FUN_0013814c(2);
    }
    if (DAT_0019f898 != 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar8 = gettext("Cannot verify compressed archives");
      error(0,0,uVar8);
                    /* WARNING: Subroutine does not return */
      FUN_0013814c(2);
    }
    if (((&DAT_0019cb60)[DAT_0019f860] & 2) == 0) {
      iVar4 = FUN_0013534c(4);
      if (iVar4 == 0) {
        DAT_0019f9b0 = '\0';
      }
      else {
        uVar8 = FUN_00134e1c(DAT_0019f860);
        FUN_001351b6("--verify",uVar8);
      }
    }
  }
  if (DAT_0019f898 != 0) {
    if (DAT_0019f8fc != '\0') {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar8 = gettext("Cannot use multi-volume compressed archives");
      error(0,0,uVar8);
                    /* WARNING: Subroutine does not return */
      FUN_0013814c(2);
    }
    if (((&DAT_0019cb60)[DAT_0019f860] & 4) != 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar8 = gettext("Cannot update compressed archives");
      error(0,0,uVar8);
                    /* WARNING: Subroutine does not return */
      FUN_0013814c(2);
    }
    if (DAT_0019f860 == 2) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar8 = gettext("Cannot concatenate compressed archives");
      error(0,0,uVar8);
                    /* WARNING: Subroutine does not return */
      FUN_0013814c(2);
    }
  }
  if ((DAT_0019f910 == 2) && (DAT_0019f928 < 0)) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar8 = gettext("--clamp-mtime needs a date specified using --mtime");
    error(0,0,uVar8);
                    /* WARNING: Subroutine does not return */
    FUN_0013814c(2);
  }
  if (((local_47 != '\0') && (DAT_0019f864 != 4)) && (((&DAT_0019cb60)[DAT_0019f860] & 1) == 0)) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar8 = gettext("--pax-option can be used only on POSIX archives");
    error(0,0,uVar8);
                    /* WARNING: Subroutine does not return */
    FUN_0013814c(2);
  }
  if (((0 < DAT_0019f968) && (DAT_0019f864 != 4)) && (((&DAT_0019cb60)[DAT_0019f860] & 1) == 0)) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar8 = gettext("--acls can be used only on POSIX archives");
    error(0,0,uVar8);
                    /* WARNING: Subroutine does not return */
    FUN_0013814c(2);
  }
  if (((0 < DAT_0019f964) && (DAT_0019f864 != 4)) && (((&DAT_0019cb60)[DAT_0019f860] & 1) == 0)) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar8 = gettext("--selinux can be used only on POSIX archives");
    error(0,0,uVar8);
                    /* WARNING: Subroutine does not return */
    FUN_0013814c(2);
  }
  if (((0 < DAT_0019f96c) && (DAT_0019f864 != 4)) && (((&DAT_0019cb60)[DAT_0019f860] & 1) == 0)) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar8 = gettext("--xattrs can be used only on POSIX archives");
    error(0,0,uVar8);
                    /* WARNING: Subroutine does not return */
    FUN_0013814c(2);
  }
  if ((DAT_0019f988 != '\0') && (((&DAT_0019cb60)[DAT_0019f860] & 1) == 0)) {
    iVar4 = FUN_0013534c(5);
    if (iVar4 == 0) {
      DAT_0019f988 = '\0';
    }
    else {
      uVar8 = FUN_00134e1c(DAT_0019f860);
      FUN_001351b6("--starting-file",uVar8);
    }
  }
  if ((DAT_0019f958 != '\0') && (((&DAT_0019cb60)[DAT_0019f860] & 1) == 0)) {
    iVar4 = FUN_0013534c(6);
    if (iVar4 == 0) {
      DAT_0019f958 = '\0';
    }
    else {
      uVar8 = FUN_00134e1c(DAT_0019f860);
      FUN_001351b6("--same-order",uVar8);
    }
  }
  if (DAT_0019f936 != '\0') {
    if (DAT_0019f878 != '\0') {
      local_a0 = (int *)FUN_00135327(7);
      local_98 = FUN_00135327(8);
      iVar4 = FUN_00135388(local_a0,local_98);
      if (iVar4 == 0) {
        if (*local_a0 == 1) {
          DAT_0019f878 = '\0';
        }
        else {
          DAT_0019f936 = '\0';
        }
      }
      else {
        FUN_001351b6("--one-top-level","--absolute-names");
      }
    }
    if ((DAT_0019f936 != '\0') && (DAT_0019f938 == 0)) {
      local_90 = (void *)FUN_0014e13e(*DAT_0019fbd8);
      DAT_0019f938 = FUN_001329b0(local_90);
      free(local_90);
      if (DAT_0019f938 == 0) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar8 = gettext(
                       "Cannot deduce top-level directory name; please set it explicitly with --one-top-level=DIR"
                       );
        error(0,0,uVar8);
                    /* WARNING: Subroutine does not return */
        FUN_0013814c(2);
      }
    }
  }
  if (DAT_0019f94c != '\0') {
    DAT_0019f8d8 = 3;
  }
  if (DAT_0019f8a0 == '\0') {
    uVar10 = 0xa0900;
  }
  else {
    uVar10 = 0x80900;
  }
  if (DAT_0019f880 == 2) {
    uVar5 = 0x40000;
  }
  else {
    uVar5 = 0;
  }
  DAT_0019fc08 = uVar5 | uVar10;
  DAT_0019fc0c = DAT_0019fc08 | 0x10000;
  if (DAT_0019f8a0 == '\0') {
    DAT_0019fc10 = 0x100;
  }
  else {
    DAT_0019fc10 = 0;
  }
  local_e4 = DAT_0019fc08;
  if (DAT_0019f860 == 9) {
    cVar3 = FUN_00134a5e();
    if (cVar3 != '\x01') {
      DAT_0019f9ac = DAT_0019f9ac + 1;
    }
  }
  else if (DAT_0019f879 != '\0') {
    DAT_0019f9ac = 2;
  }
  if ((DAT_0019f990 != 0.0) && (DAT_0019f990 < (double)DAT_0019f870)) {
    if (DAT_0019fdc8 != (code *)0x0) {
      (*DAT_0019fdc8)();
    }
    uVar8 = gettext("Volume length cannot be less than record size");
    error(0,0,uVar8);
                    /* WARNING: Subroutine does not return */
    FUN_0013814c(2);
  }
  if ((DAT_0019f958 != '\0') && (DAT_0019f8e0 != 0)) {
    local_88 = (int *)FUN_00135327(6);
    local_80 = FUN_00135327(2);
    iVar4 = FUN_00135388(local_88,local_80);
    if (iVar4 == 0) {
      if (*local_88 == 1) {
        DAT_0019f8e0 = 0;
      }
      else {
        DAT_0019f958 = '\0';
      }
    }
    else {
      FUN_001351b6("--preserve-order","--listed-incremental");
    }
  }
  switch(DAT_0019f860) {
  case 1:
  case 2:
  case 8:
    for (DAT_0019fbe8 = DAT_0019fbd8; DAT_0019fbe8 < DAT_0019fbd8 + DAT_0019fbe0;
        DAT_0019fbe8 = DAT_0019fbe8 + 1) {
      iVar4 = strcmp((char *)*DAT_0019fbe8,"-");
      if (iVar4 == 0) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar8 = gettext("Options \'-Aru\' are incompatible with \'-f -\'");
        error(0,0,uVar8);
                    /* WARNING: Subroutine does not return */
        FUN_0013814c(2);
      }
    }
  default:
    break;
  case 3:
    cVar3 = FUN_00134a5e();
    if (cVar3 != '\x01') {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar8 = gettext("Cowardly refusing to create an empty archive");
      error(0,0,uVar8);
                    /* WARNING: Subroutine does not return */
      FUN_0013814c(2);
    }
    if (((local_46 != '\0') && (DAT_0019fbe0 != 0)) &&
       (iVar4 = strcmp((char *)*DAT_0019fbd8,"-"), iVar4 != 0)) {
      FUN_00132970(*DAT_0019fbd8,DAT_0019f898);
    }
    break;
  case 5:
  case 6:
  case 7:
  case 9:
    for (DAT_0019fbe8 = DAT_0019fbd8; DAT_0019fbe8 < DAT_0019fbd8 + DAT_0019fbe0;
        DAT_0019fbe8 = DAT_0019fbe8 + 1) {
      iVar4 = strcmp((char *)*DAT_0019fbe8,"-");
      if (iVar4 == 0) {
        FUN_00134a6f(&DAT_0018746d);
      }
    }
  }
  if (DAT_0019fbf0 != (char *)0x0) {
    DAT_0019efd0 = fopen(DAT_0019fbf0,"w");
    if (DAT_0019efd0 != (FILE *)0x0) goto LAB_001398a3;
    FUN_0013e3b4(DAT_0019fbf0);
  }
  DAT_0019efd0 = stdout;
  if (DAT_0019f998 != '\0') {
    DAT_0019efd0 = stderr;
  }
LAB_001398a3:
  DAT_0019fbe8 = DAT_0019fbd8;
  if (local_40 != (char *)0x0) {
    DAT_0019fe30 = FUN_00160cc6(local_40);
  }
  if ((DAT_0019f884 != '\0') &&
     (((DAT_0019f888 = FUN_0014dec2("--backup",local_38), DAT_0019f888 == 0 ||
       (DAT_0019f998 != '\0')) || (DAT_0019f9a0 != 0)))) {
    DAT_0019f884 = '\0';
  }
  FUN_0010fb72();
  FUN_0013589e(&local_58);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: main @ 0x39948

int main(undefined4 param_1,undefined8 *param_2)

{
  int iVar1;
  undefined8 uVar2;
  
  FUN_0010adc5();
  FUN_0015c844(*param_2);
  setlocale(6,"");
  bindtextdomain(&DAT_00187004,"/usr/local/share/locale");
  textdomain(&DAT_00187004);
  DAT_0019ee88 = 2;
  DAT_0019fdd0 = 0;
  DAT_0019fdc8 = FUN_00110622;
  FUN_0015c9a2(0,7);
  uVar2 = gettext("stdout");
  FUN_0014dfe0(uVar2);
  iVar1 = FUN_0015fe52();
  if (iVar1 == 0) {
    DAT_0019fc38 = 10;
    DAT_0019fbd8 = (void *)FUN_00160b5b(0x50);
    DAT_0019fbe0 = 0;
    signal(0x11,(__sighandler_t)0x0);
    FUN_0015c826();
    FUN_0013854d(param_1,param_2);
    FUN_0012c94c();
    if (DAT_0019f9b8 != 0) {
      FUN_0010d1c6();
    }
    switch(DAT_0019f860) {
    case 0:
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar2 = gettext(
                     "You must specify one of the \'-Acdtrux\', \'--delete\' or \'--test-label\' options"
                     );
      error(0,0,uVar2);
                    /* WARNING: Subroutine does not return */
      FUN_0013814c(2);
    case 1:
    case 2:
    case 8:
      FUN_0013c325();
      break;
    case 3:
      FUN_00114e23();
      break;
    case 4:
      FUN_00117069();
      break;
    case 5:
      FUN_0011075e();
      FUN_001257b8(FUN_00111897);
      break;
    case 6:
      FUN_001182fd();
      FUN_001257b8(FUN_0011bbcb);
      FUN_0011c2ee();
      break;
    case 7:
      FUN_001257b8(FUN_00125cf3);
      break;
    case 9:
      FUN_00128913();
    }
    FUN_00110729();
    if (DAT_0019f999 != '\0') {
      FUN_0010baa2();
    }
    if (DAT_0019fc34 != 0) {
      FUN_0011573f();
    }
    if (DAT_0019f9b8 != 0) {
      FUN_0010d2e3();
    }
    free(DAT_0019fbd8);
    FUN_0013d17c();
    FUN_0012c978();
    if (DAT_0019fdd0 == 2) {
      uVar2 = gettext("Exiting with failure status due to previous errors");
      error(0,0,uVar2);
    }
    if (DAT_0019efd0 == stdout) {
      FUN_0014e014();
    }
    else {
      iVar1 = ferror_unlocked(stderr);
      if ((iVar1 != 0) || (iVar1 = fclose(stderr), iVar1 != 0)) {
        FUN_00139e98(2);
      }
    }
    return DAT_0019fdd0;
  }
  if (DAT_0019fdc8 != (code *)0x0) {
    (*DAT_0019fdc8)();
  }
  uVar2 = gettext("failed to assert availability of the standard file descriptors");
  error(0,0,&DAT_0018752f,uVar2);
                    /* WARNING: Subroutine does not return */
  FUN_001177f4();
}




// Function: tar_stat_init @ 0x39c55

void tar_stat_init(void *param_1)

{
  memset(param_1,0,0x1b8);
  return;
}




// Function: tar_stat_close @ 0x39c7e

bool tar_stat_close(undefined8 *param_1)

{
  int iVar1;
  
  if (param_1[0x34] == 0) {
    if (*(int *)(param_1 + 0x35) < 1) {
      iVar1 = 0;
    }
    else {
      iVar1 = close(*(int *)(param_1 + 0x35));
    }
  }
  else {
    iVar1 = closedir((DIR *)param_1[0x34]);
  }
  param_1[0x34] = 0;
  *(undefined4 *)(param_1 + 0x35) = 0;
  if (iVar1 != 0) {
    FUN_0012b26a(*param_1);
  }
  return iVar1 == 0;
}




// Function: tar_stat_destroy @ 0x39d1c

void tar_stat_destroy(undefined8 *param_1)

{
  FUN_00139c7e(param_1);
  FUN_0011d55d(param_1[0x2c],param_1[0x2b]);
  free((void *)*param_1);
  free((void *)param_1[1]);
  free((void *)param_1[3]);
  free((void *)param_1[4]);
  free((void *)param_1[5]);
  free((void *)param_1[6]);
  free((void *)param_1[7]);
  free((void *)param_1[9]);
  free((void *)param_1[0x28]);
  free((void *)param_1[0x32]);
  FUN_0011e785(param_1 + 0x2d);
  FUN_00117bda(param_1);
  memset(param_1,0,0x1b8);
  return;
}




// Function: tar_timespec_cmp @ 0x39e35

void tar_timespec_cmp(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  undefined8 local_20;
  undefined8 local_10;
  
  local_20 = param_4;
  local_10 = param_2;
  if (DAT_0019f468 != 4) {
    local_20 = 0;
    local_10 = 0;
  }
  FUN_0015ff90(param_1,local_10,param_3,local_20);
  return;
}




// Function: set_exit_status @ 0x39e98

void set_exit_status(int param_1)

{
  if (DAT_0019fdd0 < param_1) {
    DAT_0019fdd0 = param_1;
  }
  return;
}




// Function: new_transform @ 0x39eba

undefined8 new_transform(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  
  uVar2 = FUN_00160c06(0x70);
  uVar1 = uVar2;
  if (DAT_0019fcc8 != (undefined8 *)0x0) {
    *DAT_0019fcc8 = uVar2;
    uVar1 = DAT_0019fcc0;
  }
  DAT_0019fcc0 = uVar1;
  DAT_0019fcc8 = (undefined8 *)uVar2;
  return uVar2;
}




// Function: add_segment @ 0x39f0c

undefined8 * add_segment(long param_1)

{
  undefined8 *puVar1;
  
  puVar1 = (undefined8 *)FUN_00160b5b(0x20);
  *puVar1 = 0;
  if (*(long *)(param_1 + 0x60) == 0) {
    *(undefined8 **)(param_1 + 0x58) = puVar1;
  }
  else {
    **(undefined8 **)(param_1 + 0x60) = puVar1;
  }
  *(undefined8 **)(param_1 + 0x60) = puVar1;
  *(long *)(param_1 + 0x68) = *(long *)(param_1 + 0x68) + 1;
  return puVar1;
}




// Function: add_literal_segment @ 0x39f85

void add_literal_segment(undefined8 param_1,void *param_2,long param_3)

{
  size_t __n;
  long lVar1;
  undefined8 uVar2;
  
  __n = param_3 - (long)param_2;
  if (__n != 0) {
    lVar1 = FUN_00139f0c(param_1);
    *(undefined4 *)(lVar1 + 8) = 0;
    uVar2 = FUN_00160b5b(__n + 1);
    *(undefined8 *)(lVar1 + 0x10) = uVar2;
    memcpy(*(void **)(lVar1 + 0x10),param_2,__n);
    *(undefined1 *)(__n + *(long *)(lVar1 + 0x10)) = 0;
    *(size_t *)(lVar1 + 0x18) = __n;
  }
  return;
}




// Function: add_char_segment @ 0x3a022

void add_char_segment(undefined8 param_1,undefined1 param_2)

{
  long lVar1;
  undefined8 uVar2;
  
  lVar1 = FUN_00139f0c(param_1);
  *(undefined4 *)(lVar1 + 8) = 0;
  uVar2 = FUN_00160b5b(2);
  *(undefined8 *)(lVar1 + 0x10) = uVar2;
  **(undefined1 **)(lVar1 + 0x10) = param_2;
  *(undefined1 *)(*(long *)(lVar1 + 0x10) + 1) = 0;
  *(undefined8 *)(lVar1 + 0x18) = 1;
  return;
}




// Function: add_backref_segment @ 0x3a090

void add_backref_segment(undefined8 param_1,undefined8 param_2)

{
  long lVar1;
  
  lVar1 = FUN_00139f0c(param_1);
  *(undefined4 *)(lVar1 + 8) = 1;
  *(undefined8 *)(lVar1 + 0x10) = param_2;
  return;
}




// Function: parse_xform_flags @ 0x3a0ce

undefined8 parse_xform_flags(uint *param_1,undefined4 param_2)

{
  switch(param_2) {
  case 0x48:
    *param_1 = *param_1 & 0xfffffffd;
    break;
  default:
    return 1;
  case 0x52:
    *param_1 = *param_1 & 0xfffffffe;
    break;
  case 0x53:
    *param_1 = *param_1 & 0xfffffffb;
    break;
  case 0x68:
    *param_1 = *param_1 | 2;
    break;
  case 0x72:
    *param_1 = *param_1 | 1;
    break;
  case 0x73:
    *param_1 = *param_1 | 4;
  }
  return 0;
}




// Function: add_case_ctl_segment @ 0x3a18f

void add_case_ctl_segment(undefined8 param_1,undefined4 param_2)

{
  long lVar1;
  
  lVar1 = FUN_00139f0c(param_1);
  *(undefined4 *)(lVar1 + 8) = 2;
  *(undefined4 *)(lVar1 + 0x10) = param_2;
  return;
}




// Function: parse_transform_expr @ 0x3a1ca

char * parse_transform_expr(char *param_1)

{
  char cVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  undefined8 uVar5;
  ulong uVar6;
  long in_FS_OFFSET;
  char *local_280;
  int local_26c;
  int local_268;
  uint local_264;
  char *local_258;
  char *local_250;
  char *local_248;
  long local_240;
  char *local_238;
  ulong local_230;
  undefined1 local_228;
  char local_227;
  undefined1 auStack_226 [518];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_264 = 0;
  local_240 = FUN_00139eba();
  if (*param_1 == 's') {
    iVar3 = (int)param_1[1];
    if (iVar3 == 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar5 = gettext("Invalid transform expression");
      error(0,0,uVar5);
                    /* WARNING: Subroutine does not return */
      FUN_0013814c(2);
    }
    local_26c = 2;
    while ((param_1[local_26c] != '\0' && (iVar3 != param_1[local_26c]))) {
      if ((param_1[local_26c] == '\\') && (param_1[(long)local_26c + 1] != '\0')) {
        local_26c = local_26c + 1;
      }
      local_26c = local_26c + 1;
    }
    iVar4 = local_26c;
    if (iVar3 != param_1[local_26c]) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar5 = gettext("Invalid transform expression");
      error(0,0,uVar5);
                    /* WARNING: Subroutine does not return */
      FUN_0013814c(2);
    }
    while ((iVar2 = iVar4, local_268 = iVar2 + 1, param_1[local_268] != '\0' &&
           (iVar3 != param_1[local_268]))) {
      iVar4 = local_268;
      if ((param_1[local_268] == '\\') && (iVar4 = local_268, param_1[(long)local_268 + 1] != '\0'))
      {
        iVar4 = iVar2 + 2;
      }
    }
    if (iVar3 != param_1[local_268]) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar5 = gettext("Invalid transform expression");
      error(0,0,uVar5);
                    /* WARNING: Subroutine does not return */
      FUN_0013814c(2);
    }
    *(undefined4 *)(local_240 + 8) = 0;
    *(undefined4 *)(local_240 + 0xc) = DAT_0019ec00;
    for (local_250 = param_1 + (long)local_268 + 1; (*local_250 != '\0' && (*local_250 != ';'));
        local_250 = local_250 + 1) {
      cVar1 = *local_250;
      if (cVar1 == 'x') {
        local_264 = local_264 | 1;
      }
      else if (cVar1 < 'y') {
        if (cVar1 == 'i') {
          local_264 = local_264 | 2;
        }
        else {
          if ('i' < cVar1) goto LAB_0013a67e;
          if (cVar1 < ':') {
            if (cVar1 < '0') goto LAB_0013a67e;
            uVar6 = strtoul(local_250,&local_250,0);
            *(int *)(local_240 + 0x10) = (int)uVar6;
            local_250 = local_250 + -1;
          }
          else {
            if (cVar1 != 'g') goto LAB_0013a67e;
            *(undefined4 *)(local_240 + 8) = 1;
          }
        }
      }
      else {
LAB_0013a67e:
        iVar4 = FUN_0013a0ce(local_240 + 0xc,(int)*local_250);
        if (iVar4 != 0) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          cVar1 = *local_250;
          uVar5 = gettext("Unknown flag in transform expression: %c");
          error(0,0,uVar5,(int)cVar1);
                    /* WARNING: Subroutine does not return */
          FUN_0013814c(2);
        }
      }
    }
    if (*local_250 == ';') {
      local_250 = local_250 + 1;
    }
    local_238 = (char *)FUN_00160b5b((long)(local_26c + -1));
    memcpy(local_238,param_1 + 2,(long)(local_26c + -2));
    local_238[(long)local_26c + -2] = '\0';
    iVar4 = FUN_00167ccd(local_240 + 0x18,local_238,local_264);
    if (iVar4 != 0) {
      FUN_00167e43(iVar4,local_240 + 0x18,&local_228,0x200);
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar5 = gettext("Invalid transform expression: %s");
      error(0,0,uVar5,&local_228);
                    /* WARNING: Subroutine does not return */
      FUN_0013814c(2);
    }
    if ((*local_238 == '^') || ((2 < local_26c && (local_238[(long)local_26c + -3] == '$')))) {
      *(undefined4 *)(local_240 + 8) = 0;
    }
    free(local_238);
    local_26c = local_26c + 1;
    local_238 = (char *)FUN_00160b5b((long)((local_268 - local_26c) + 1));
    memcpy(local_238,param_1 + local_26c,(long)(local_268 - local_26c));
    local_238[local_268 - local_26c] = '\0';
    local_248 = local_238;
    local_258 = local_238;
    while (*local_258 != '\0') {
      if (*local_258 == '\\') {
        FUN_00139f85(local_240,local_248,local_258);
        local_258 = local_258 + 1;
        switch(*local_258) {
        case '&':
          FUN_0013a022(local_240,0x26);
          local_258 = local_258 + 1;
          break;
        default:
          if (iVar3 == *local_258) {
            FUN_0013a022(local_240,iVar3);
          }
          else {
            local_228 = 0x5c;
            local_227 = *local_258;
            FUN_00139f85(local_240,&local_228,auStack_226);
          }
          local_258 = local_258 + 1;
          break;
        case '0':
        case '1':
        case '2':
        case '3':
        case '4':
        case '5':
        case '6':
        case '7':
        case '8':
        case '9':
          local_230 = strtoul(local_258,&local_258,10);
          if (*(ulong *)(local_240 + 0x48) < local_230) {
            if (DAT_0019fdc8 != (code *)0x0) {
              (*DAT_0019fdc8)();
            }
            uVar5 = gettext("Invalid transform replacement: back reference out of range");
            error(0,0,uVar5);
                    /* WARNING: Subroutine does not return */
            FUN_0013814c(2);
          }
          FUN_0013a090(local_240,local_230);
          break;
        case 'E':
          FUN_0013a18f(local_240,0);
          local_258 = local_258 + 1;
          break;
        case 'L':
          FUN_0013a18f(local_240,4);
          local_258 = local_258 + 1;
          break;
        case 'U':
          FUN_0013a18f(local_240,3);
          local_258 = local_258 + 1;
          break;
        case '\\':
          FUN_0013a022(local_240,0x5c);
          local_258 = local_258 + 1;
          break;
        case 'a':
          FUN_0013a022(local_240,7);
          local_258 = local_258 + 1;
          break;
        case 'b':
          FUN_0013a022(local_240,8);
          local_258 = local_258 + 1;
          break;
        case 'f':
          FUN_0013a022(local_240,0xc);
          local_258 = local_258 + 1;
          break;
        case 'l':
          FUN_0013a18f(local_240,2);
          local_258 = local_258 + 1;
          break;
        case 'n':
          FUN_0013a022(local_240,10);
          local_258 = local_258 + 1;
          break;
        case 'r':
          FUN_0013a022(local_240,0xd);
          local_258 = local_258 + 1;
          break;
        case 't':
          FUN_0013a022(local_240,9);
          local_258 = local_258 + 1;
          break;
        case 'u':
          FUN_0013a18f(local_240,1);
          local_258 = local_258 + 1;
          break;
        case 'v':
          FUN_0013a022(local_240,0xb);
          local_258 = local_258 + 1;
        }
        local_248 = local_258;
      }
      else if (*local_258 == '&') {
        FUN_00139f85(local_240,local_248,local_258);
        FUN_0013a090(local_240,0);
        local_258 = local_258 + 1;
        local_248 = local_258;
      }
      else {
        local_258 = local_258 + 1;
      }
    }
    FUN_00139f85(local_240,local_248,local_258);
    free(local_238);
    local_280 = local_250;
  }
  else {
    iVar3 = strncmp(param_1,"flags=",6);
    if (iVar3 != 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar5 = gettext("Invalid transform expression");
      error(0,0,uVar5);
                    /* WARNING: Subroutine does not return */
      FUN_0013814c(2);
    }
    DAT_0019ec00 = 0;
    for (local_280 = param_1 + 6; *local_280 != '\0'; local_280 = local_280 + 1) {
      if (*local_280 == ';') {
        local_280 = local_280 + 1;
        break;
      }
      iVar3 = FUN_0013a0ce(&DAT_0019ec00,(int)*local_280);
      if (iVar3 != 0) {
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        cVar1 = *local_280;
        uVar5 = gettext("Unknown transform flag: %c");
        error(0,0,uVar5,(int)cVar1);
                    /* WARNING: Subroutine does not return */
        FUN_0013814c(2);
      }
    }
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return local_280;
}




// Function: set_transform_expr @ 0x3ae0b

void set_transform_expr(char *param_1)

{
  undefined8 local_10;
  
  for (local_10 = param_1; *local_10 != '\0'; local_10 = (char *)FUN_0013a1ca(local_10)) {
  }
  return;
}




// Function: run_case_conv @ 0x3ae3c

byte * run_case_conv(undefined4 param_1,void *param_2,ulong param_3)

{
  int iVar1;
  byte *local_10;
  
  if (DAT_0019fd40 < param_3) {
    DAT_0019fd40 = param_3;
    DAT_0019fcd0 = (byte *)FUN_00160b8d(DAT_0019fcd0,param_3);
  }
  memcpy(DAT_0019fcd0,param_2,param_3);
  switch(param_1) {
  case 1:
    iVar1 = toupper((uint)*DAT_0019fcd0);
    *DAT_0019fcd0 = (byte)iVar1;
    break;
  case 2:
    iVar1 = tolower((uint)*DAT_0019fcd0);
    *DAT_0019fcd0 = (byte)iVar1;
    break;
  case 3:
    for (local_10 = DAT_0019fcd0; local_10 < DAT_0019fcd0 + param_3; local_10 = local_10 + 1) {
      iVar1 = toupper((uint)*local_10);
      *local_10 = (byte)iVar1;
    }
    break;
  case 4:
    for (local_10 = DAT_0019fcd0; local_10 < DAT_0019fcd0 + param_3; local_10 = local_10 + 1) {
      iVar1 = tolower((uint)*local_10);
      *local_10 = (byte)iVar1;
    }
  }
  return DAT_0019fcd0;
}




// Function: _single_transform_name_to_obstack @ 0x3afa7

void _single_transform_name_to_obstack(long param_1,char *param_2)

{
  uint uVar1;
  undefined1 *puVar2;
  int iVar3;
  ulong *__ptr;
  ulong uVar4;
  size_t __n;
  char *local_f8;
  int local_e4;
  int local_e0;
  ulong local_d8;
  ulong local_d0;
  char *local_c8;
  undefined8 *local_c0;
  
  local_d8 = 0;
  local_e4 = 0;
  local_e0 = 0;
  __ptr = (ulong *)FUN_00160b5b((*(long *)(param_1 + 0x48) + 1) * 0x10);
  local_f8 = param_2;
LAB_0013b6ba:
  do {
    if (*local_f8 == '\0') goto LAB_0013b6cc;
    iVar3 = FUN_0016f294(param_1 + 0x18,local_f8,*(long *)(param_1 + 0x48) + 1,__ptr,0);
    if (iVar3 == 0) {
      local_d0 = __ptr[1];
      if (*__ptr != 0) {
        uVar4 = *__ptr;
        if ((ulong)((long)DAT_0019fd00 - (long)DAT_0019fcf8) < uVar4) {
          _obstack_newchunk(&DAT_0019fce0,uVar4);
        }
        memcpy(DAT_0019fcf8,local_f8,uVar4);
        DAT_0019fcf8 = DAT_0019fcf8 + uVar4;
      }
      local_d8 = local_d8 + 1;
      if ((*(int *)(param_1 + 0x10) != 0) && (local_d8 < *(uint *)(param_1 + 0x10))) {
        if ((ulong)((long)DAT_0019fd00 - (long)DAT_0019fcf8) < local_d0) {
          _obstack_newchunk(&DAT_0019fce0,local_d0);
        }
        memcpy(DAT_0019fcf8,local_f8,local_d0);
        DAT_0019fcf8 = DAT_0019fcf8 + local_d0;
        local_f8 = local_f8 + local_d0;
        goto LAB_0013b6ba;
      }
      for (local_c0 = *(undefined8 **)(param_1 + 0x58); local_c0 != (undefined8 *)0x0;
          local_c0 = (undefined8 *)*local_c0) {
        uVar1 = *(uint *)(local_c0 + 1);
        if (uVar1 == 2) {
          uVar1 = *(uint *)(local_c0 + 2);
          if (uVar1 < 5) {
            if ((uVar1 < 3) && (uVar1 != 0)) {
              if (1 < uVar1 - 1) goto LAB_0013b528;
              if ((local_e0 == 0) || (local_e0 - 3U < 2)) {
                local_e0 = local_e4;
              }
            }
            local_e4 = *(int *)(local_c0 + 2);
          }
        }
        else if (uVar1 < 3) {
          if (uVar1 == 0) {
            if (local_e4 == 0) {
              local_c8 = (char *)local_c0[2];
            }
            else {
              local_c8 = (char *)FUN_0013ae3c(local_e4,local_c0[2],local_c0[3]);
              if ((local_e4 == 1) || (local_e4 == 2)) {
                local_e4 = local_e0;
                local_e0 = 0;
              }
            }
            uVar4 = local_c0[3];
            if ((ulong)((long)DAT_0019fd00 - (long)DAT_0019fcf8) < uVar4) {
              _obstack_newchunk(&DAT_0019fce0,uVar4);
            }
            memcpy(DAT_0019fcf8,local_c8,uVar4);
            DAT_0019fcf8 = DAT_0019fcf8 + uVar4;
          }
          else if (((uVar1 == 1) && (__ptr[local_c0[2] * 2] != 0xffffffffffffffff)) &&
                  (__ptr[local_c0[2] * 2 + 1] != 0xffffffffffffffff)) {
            uVar4 = __ptr[local_c0[2] * 2 + 1] - __ptr[local_c0[2] * 2];
            local_c8 = local_f8 + __ptr[local_c0[2] * 2];
            if ((local_e4 != 0) &&
               ((local_c8 = (char *)FUN_0013ae3c(local_e4,local_c8,uVar4), local_e4 == 1 ||
                (local_e4 == 2)))) {
              local_e4 = local_e0;
              local_e0 = 0;
            }
            if ((ulong)((long)DAT_0019fd00 - (long)DAT_0019fcf8) < uVar4) {
              _obstack_newchunk(&DAT_0019fce0,uVar4);
            }
            memcpy(DAT_0019fcf8,local_c8,uVar4);
            DAT_0019fcf8 = DAT_0019fcf8 + uVar4;
          }
        }
LAB_0013b528:
      }
    }
    else {
      local_d0 = strlen(local_f8);
      if ((ulong)((long)DAT_0019fd00 - (long)DAT_0019fcf8) < local_d0) {
        _obstack_newchunk(&DAT_0019fce0,local_d0);
      }
      memcpy(DAT_0019fcf8,local_f8,local_d0);
      DAT_0019fcf8 = DAT_0019fcf8 + local_d0;
    }
    local_f8 = local_f8 + local_d0;
    if (*(int *)(param_1 + 8) == 0) {
      __n = strlen(local_f8);
      if ((ulong)((long)DAT_0019fd00 - (long)DAT_0019fcf8) < __n) {
        _obstack_newchunk(&DAT_0019fce0,__n);
      }
      memcpy(DAT_0019fcf8,local_f8,__n);
      DAT_0019fcf8 = DAT_0019fcf8 + __n;
LAB_0013b6cc:
      if (DAT_0019fd00 == DAT_0019fcf8) {
        _obstack_newchunk(&DAT_0019fce0,1);
      }
      puVar2 = DAT_0019fcf8 + 1;
      *DAT_0019fcf8 = 0;
      DAT_0019fcf8 = puVar2;
      free(__ptr);
      return;
    }
  } while( true );
}




// Function: _transform_name_to_obstack @ 0x3b734

undefined1 _transform_name_to_obstack(uint param_1,ulong param_2,ulong *param_3)

{
  ulong local_38;
  undefined1 local_21;
  undefined8 *local_20;
  
  local_21 = 0;
  if (DAT_0019fd38 != '\x01') {
    _obstack_begin(&DAT_0019fce0,0,0,FUN_00160b5b,PTR_free_0019bfc0);
    DAT_0019fd38 = '\x01';
  }
  local_38 = param_2;
  for (local_20 = DAT_0019fcc0; local_20 != (undefined8 *)0x0; local_20 = (undefined8 *)*local_20) {
    if ((*(uint *)((long)local_20 + 0xc) & param_1) != 0) {
      FUN_0013afa7(local_20,local_38);
      local_38 = DAT_0019fcf0;
      if (DAT_0019fcf0 == DAT_0019fcf8) {
        DAT_0019fd30 = DAT_0019fd30 | 2;
      }
      DAT_0019fcf8 = ~DAT_0019fd10 & DAT_0019fcf8 + DAT_0019fd10;
      if (DAT_0019fd00 - DAT_0019fce8 < DAT_0019fcf8 - DAT_0019fce8) {
        DAT_0019fcf8 = DAT_0019fd00;
      }
      DAT_0019fcf0 = DAT_0019fcf8;
      local_21 = 1;
    }
  }
  *param_3 = local_38;
  return local_21;
}




// Function: transform_name_fp @ 0x3b8af

char transform_name_fp(undefined8 *param_1,undefined4 param_2,code *param_3,undefined8 param_4)

{
  void *pvVar1;
  undefined8 uVar2;
  long in_FS_OFFSET;
  char local_29;
  void *local_28;
  undefined *local_20;
  void *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_29 = FUN_0013b734(param_2,*param_1,&local_28);
  if (local_29 == '\0') {
    if (param_3 != (code *)0x0) {
      *param_1 = 0;
      uVar2 = (*param_3)(local_28,param_4);
      FUN_00129547(param_1,uVar2);
      free(local_28);
      local_29 = '\x01';
    }
  }
  else {
    pvVar1 = local_28;
    if (param_3 != (code *)0x0) {
      pvVar1 = (void *)(*param_3)(local_28,param_4);
    }
    FUN_00129547(param_1,pvVar1);
    local_20 = &DAT_0019fce0;
    local_18 = local_28;
    if ((DAT_0019fce8 < local_28) && (local_28 < DAT_0019fd00)) {
      DAT_0019fcf0 = local_28;
      DAT_0019fcf8 = local_28;
    }
    else {
      _obstack_free(&DAT_0019fce0,local_28);
    }
  }
  if (local_10 == *(long *)(in_FS_OFFSET + 0x28)) {
    return local_29;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}




// Function: transform_name @ 0x3b9f2

void transform_name(undefined8 param_1,undefined4 param_2)

{
  FUN_0013b8af(param_1,param_2,0,0);
  return;
}




// Function: transform_program_p @ 0x3ba20

undefined8 transform_program_p(void)

{
  return CONCAT71((int7)((ulong)DAT_0019fcc0 >> 8),DAT_0019fcc0 != 0);
}




// Function: dunlink_alloc @ 0x3ba37

undefined8 * dunlink_alloc(void)

{
  undefined8 *puVar1;
  undefined8 local_10;
  
  if (DAT_0019fd60 == (undefined8 *)0x0) {
    local_10 = (undefined8 *)FUN_00160b5b(0x28);
  }
  else {
    local_10 = DAT_0019fd60;
    puVar1 = (undefined8 *)*DAT_0019fd60;
    *DAT_0019fd60 = 0;
    DAT_0019fd60 = puVar1;
  }
  return local_10;
}




// Function: dunlink_insert @ 0x3ba89

void dunlink_insert(long *param_1,long *param_2)

{
  if (param_1 == (long *)0x0) {
    *param_2 = (long)DAT_0019fd48;
    DAT_0019fd48 = param_2;
  }
  else {
    *param_2 = *param_1;
    *param_1 = (long)param_2;
  }
  if (*param_2 == 0) {
    DAT_0019fd50 = param_2;
  }
  DAT_0019fd58 = DAT_0019fd58 + 1;
  return;
}




// Function: dunlink_reclaim @ 0x3bb00

void dunlink_reclaim(undefined8 *param_1)

{
  free((void *)param_1[2]);
  *param_1 = DAT_0019fd60;
  DAT_0019fd60 = param_1;
  return;
}




// Function: flush_deferred_unlinks @ 0x3bb3c

void flush_deferred_unlinks(char param_1)

{
  undefined8 *puVar1;
  char *__name;
  undefined8 *puVar2;
  undefined4 uVar3;
  int iVar4;
  int *piVar5;
  undefined8 *local_38;
  undefined8 *local_30;
  char *local_28;
  
  uVar3 = DAT_0019f600;
  local_30 = (undefined8 *)0x0;
  local_38 = DAT_0019fd48;
  puVar2 = DAT_0019fd48;
LAB_0013bd19:
  while( true ) {
    DAT_0019fd48 = puVar2;
    if (local_38 == (undefined8 *)0x0) {
      if (DAT_0019fd48 == (undefined8 *)0x0) {
        DAT_0019fd50 = 0;
      }
      else if (param_1 != '\0') {
        local_38 = DAT_0019fd48;
        while (local_38 != (undefined8 *)0x0) {
          puVar2 = (undefined8 *)*local_38;
          FUN_0012adf1(*(undefined4 *)(local_38 + 1));
          if (((*(int *)(local_38 + 1) == 0) || (*(char *)(local_38 + 3) == '\0')) ||
             ((*(char *)local_38[2] != '\0' && (iVar4 = strcmp((char *)local_38[2],"."), iVar4 != 0)
              ))) {
            local_28 = (char *)local_38[2];
          }
          else {
            local_28 = (char *)FUN_0012afc0();
            FUN_0012adf1(*(int *)(local_38 + 1) + -1);
          }
          iVar4 = unlinkat(DAT_0019c268,local_28,0x200);
          if ((iVar4 != 0) && (piVar5 = __errno_location(), *piVar5 != 2)) {
            FUN_0013e705(local_28);
          }
          FUN_0013bb00(local_38);
          DAT_0019fd58 = DAT_0019fd58 + -1;
          local_38 = puVar2;
        }
        DAT_0019fd50 = 0;
        DAT_0019fd48 = (undefined8 *)0x0;
      }
      FUN_0012adf1(uVar3);
      return;
    }
    puVar1 = (undefined8 *)*local_38;
    if ((param_1 != '\0') || ((ulong)(local_38[4] + DAT_0019fd68) < DAT_0019efc8)) break;
    local_30 = local_38;
    puVar2 = DAT_0019fd48;
    local_38 = puVar1;
  }
  FUN_0012adf1(*(undefined4 *)(local_38 + 1));
  if (*(char *)(local_38 + 3) == '\0') {
    iVar4 = unlinkat(DAT_0019c268,(char *)local_38[2],0);
    if ((iVar4 != 0) && (piVar5 = __errno_location(), *piVar5 != 2)) {
      FUN_0013eaa7(local_38[2]);
    }
  }
  else {
    if (((*(int *)(local_38 + 1) != 0) && (*(char *)(local_38 + 3) != '\0')) &&
       ((*(char *)local_38[2] == '\0' || (iVar4 = strcmp((char *)local_38[2],"."), iVar4 == 0)))) {
      local_30 = local_38;
      puVar2 = DAT_0019fd48;
      local_38 = puVar1;
      goto LAB_0013bd19;
    }
    __name = (char *)local_38[2];
    iVar4 = unlinkat(DAT_0019c268,__name,0x200);
    if (iVar4 == 0) goto LAB_0013bcc2;
    piVar5 = __errno_location();
    iVar4 = *piVar5;
    if (iVar4 == 0x27) {
LAB_0013bc5f:
      local_30 = local_38;
      puVar2 = DAT_0019fd48;
      local_38 = puVar1;
      goto LAB_0013bd19;
    }
    if (iVar4 < 0x28) {
      if (iVar4 == 2) goto LAB_0013bcc2;
      if (iVar4 == 0x11) goto LAB_0013bc5f;
    }
    FUN_0013e705(__name);
  }
LAB_0013bcc2:
  FUN_0013bb00(local_38);
  DAT_0019fd58 = DAT_0019fd58 + -1;
  puVar2 = puVar1;
  local_38 = puVar1;
  if (local_30 != (undefined8 *)0x0) {
    *local_30 = puVar1;
    puVar2 = DAT_0019fd48;
  }
  goto LAB_0013bd19;
}




// Function: finish_deferred_unlinks @ 0x3be6b

void finish_deferred_unlinks(void)

{
  undefined8 *puVar1;
  
  FUN_0013bb3c(1);
  while (DAT_0019fd60 != (undefined8 *)0x0) {
    puVar1 = (undefined8 *)*DAT_0019fd60;
    free(DAT_0019fd60);
    DAT_0019fd60 = puVar1;
  }
  return;
}




// Function: queue_deferred_unlink @ 0x3bebb

void queue_deferred_unlink(undefined8 param_1,undefined1 param_2)

{
  int iVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  undefined8 *local_20;
  undefined8 *local_18;
  
  if ((DAT_0019fd48 != (undefined8 *)0x0) &&
     ((ulong)(DAT_0019fd48[4] + DAT_0019fd68) < DAT_0019efc8)) {
    FUN_0013bb3c(0);
  }
  puVar2 = (undefined8 *)FUN_0013ba37();
  *puVar2 = 0;
  *(undefined4 *)(puVar2 + 1) = DAT_0019f600;
  uVar3 = FUN_00160cc6(param_1);
  puVar2[2] = uVar3;
  FUN_00129946(puVar2[2]);
  *(undefined1 *)(puVar2 + 3) = param_2;
  puVar2[4] = DAT_0019efc8;
  if ((*(char *)(puVar2 + 3) == '\0') ||
     ((*(char *)puVar2[2] != '\0' && (iVar1 = strcmp((char *)puVar2[2],"."), iVar1 != 0)))) {
    FUN_0013ba89(DAT_0019fd50,puVar2);
  }
  else {
    local_18 = (undefined8 *)0x0;
    for (local_20 = DAT_0019fd48;
        (local_20 != (undefined8 *)0x0 &&
        ((*(char *)(local_20 + 3) == '\0' ||
         (((*(char *)local_20[2] != '\0' && (iVar1 = strcmp((char *)local_20[2],"."), iVar1 != 0))
          || (*(int *)(puVar2 + 1) <= *(int *)(local_20 + 1)))))));
        local_20 = (undefined8 *)*local_20) {
      local_18 = local_20;
    }
    if (local_20 == (undefined8 *)0x0) {
      FUN_0013ba89(DAT_0019fd50,puVar2);
    }
    else {
      FUN_0013ba89(local_18,puVar2);
    }
  }
  return;
}




// Function: append_file @ 0x3c078

void append_file(char *param_1)

{
  int iVar1;
  int iVar2;
  long lVar3;
  ulong uVar4;
  long lVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  long in_FS_OFFSET;
  ulong local_e8;
  ulong local_e0;
  stat local_c8;
  undefined1 local_38 [24];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  iVar1 = openat(DAT_0019c268,param_1,0);
  if (iVar1 < 0) {
    FUN_0013e38b(param_1);
  }
  else {
    iVar2 = fstat(iVar1,&local_c8);
    if (iVar2 == 0) {
      local_e8 = local_c8.st_size;
      while (0 < (long)local_e8) {
        lVar3 = FUN_0010bb4b();
        local_e0 = FUN_0010bbf5(lVar3);
        if (local_e8 < local_e0) {
          local_e0 = local_e8;
          uVar4 = (ulong)((uint)local_e8 & 0x1ff);
          if (uVar4 != 0) {
            memset((void *)(lVar3 + local_e8),0,0x200 - uVar4);
          }
        }
        lVar5 = FUN_0015ec70(iVar1,lVar3,local_e0);
        if (lVar5 == -1) {
          FUN_0013e5fc(param_1,local_c8.st_size - local_e8,local_e0);
        }
        if (lVar5 == 0) {
          if (DAT_0019fdc8 != (code *)0x0) {
            (*DAT_0019fdc8)();
          }
          uVar6 = FUN_00154e5a(local_e8,local_38);
          uVar7 = FUN_0015e548(param_1);
          uVar8 = ngettext("%s: File shrank by %s byte","%s: File shrank by %s bytes",local_e8);
          error(0,0,uVar8,uVar7,uVar6);
                    /* WARNING: Subroutine does not return */
          FUN_001177f4();
        }
        local_e8 = local_e8 - lVar5;
        FUN_0010bba7(lVar3 + (lVar5 - 1U & 0xfffffffffffffe00));
      }
    }
    else {
      FUN_0013ea03(param_1);
    }
    iVar1 = close(iVar1);
    if (iVar1 != 0) {
      FUN_0013e1ff(param_1);
    }
  }
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: update_archive @ 0x3c325

void update_archive(void)

{
  undefined4 uVar1;
  bool bVar2;
  char cVar3;
  undefined4 uVar4;
  int iVar5;
  char *__ptr;
  undefined8 uVar6;
  undefined8 uVar7;
  size_t sVar8;
  long lVar9;
  long in_FS_OFFSET;
  undefined1 auVar10 [16];
  undefined4 local_f0;
  char *local_e8;
  undefined1 local_b8 [24];
  uint local_a0;
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  local_f0 = 0;
  bVar2 = false;
  FUN_0012d3b1();
  FUN_0010f694(2);
  FUN_0011d498();
  while (!bVar2) {
    uVar4 = FUN_00125ecf(&DAT_0019f460,&DAT_0019fa20,0);
    switch(uVar4) {
    case 0:
    case 2:
                    /* WARNING: Subroutine does not return */
      abort();
    case 1:
      FUN_00126747(DAT_0019f460,&DAT_0019fa20,&DAT_0019f468,0);
      FUN_00125733((int)*(char *)(DAT_0019f460 + 0x9c),&DAT_0019fa20);
      DAT_0019f864 = DAT_0019f468;
      if ((DAT_0019f860 == 8) && (lVar9 = FUN_0012ed0e(DAT_0019fa28), lVar9 != 0)) {
        FUN_0012adf1(*(undefined4 *)(lVar9 + 0x28));
        iVar5 = FUN_0012a9b8(DAT_0019fa28,local_b8);
        if (iVar5 == 0) {
          if ((local_a0 & 0xf000) == 0x4000) {
            __ptr = (char *)FUN_0012b8e1(*(undefined8 *)(lVar9 + 0x10),1);
            if (__ptr != (char *)0x0) {
              uVar6 = FUN_0012b66c(*(undefined8 *)(lVar9 + 0x10));
              for (local_e8 = __ptr; *local_e8 != '\0'; local_e8 = local_e8 + sVar8 + 1) {
                uVar1 = *(undefined4 *)(lVar9 + 0x28);
                uVar7 = FUN_0012b75b(uVar6,local_e8);
                FUN_0012d5b5(uVar7,uVar1,0,0);
                sVar8 = strlen(local_e8);
              }
              FUN_0012b72d(uVar6);
              free(__ptr);
              FUN_0012d817(lVar9);
            }
          }
          else {
            auVar10 = FUN_0015fd9e(local_b8);
            iVar5 = FUN_00139e35(auVar10._0_8_,auVar10._8_8_,DAT_0019fb18,DAT_0019fb20);
            if (iVar5 < 1) {
              FUN_0012d817(lVar9);
            }
          }
        }
      }
      FUN_00128891();
      break;
    case 3:
      DAT_0019efb0 = DAT_0019f460;
      bVar2 = true;
      break;
    case 4:
      bVar2 = true;
      break;
    case 5:
      FUN_0010bba7(DAT_0019f460);
      switch(local_f0) {
      case 0:
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar6 = gettext("This does not look like a tar archive");
        error(0,0,uVar6);
      case 1:
      case 3:
        if (DAT_0019fdc8 != (code *)0x0) {
          (*DAT_0019fdc8)();
        }
        uVar6 = gettext("Skipping to next header");
        error(0,0,uVar6);
        DAT_0019fdd0 = 2;
        break;
      case 2:
      case 4:
                    /* WARNING: Subroutine does not return */
        abort();
      }
    }
    FUN_00139d1c(&DAT_0019fa20);
    local_f0 = uVar4;
  }
  FUN_0010baf9();
  DAT_0019fd70 = 1;
  DAT_0019fd78 = DAT_0019efb0;
  while (lVar9 = FUN_0012eda0(), lVar9 != 0) {
    uVar6 = *(undefined8 *)(lVar9 + 0x10);
    cVar3 = FUN_00117c43(uVar6,0);
    if ((cVar3 == '\0') &&
       ((DAT_0019f8c8 == '\0' || (iVar5 = FUN_00134af4(&DAT_001879ce,uVar6), iVar5 != 0)))) {
      if (DAT_0019f860 == 2) {
        FUN_0013c078(uVar6);
      }
      else {
        FUN_00116a25(0,uVar6,uVar6);
      }
    }
  }
  FUN_00112da0();
  FUN_0010d046();
  FUN_0013be6b();
  FUN_0012dbba();
  if (local_20 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: utf8_init @ 0x3c7e3

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 utf8_init(byte param_1)

{
  char *pcVar1;
  iconv_t pvVar2;
  
  if (*(long *)(&DAT_0019ec10 + (long)(int)(uint)param_1 * 8) == -1) {
    if (param_1 == 0) {
      pcVar1 = (char *)FUN_00154ee3();
      _DAT_0019ec10 = iconv_open(pcVar1,"UTF-8");
    }
    else {
      pcVar1 = (char *)FUN_00154ee3();
      pvVar2 = iconv_open("UTF-8",pcVar1);
      *(iconv_t *)(&DAT_0019ec10 + (long)(int)(uint)param_1 * 8) = pvVar2;
    }
  }
  return *(undefined8 *)(&DAT_0019ec10 + (long)(int)(uint)param_1 * 8);
}




// Function: utf8_convert @ 0x3c8a0

undefined8 utf8_convert(undefined1 param_1,char *param_2,undefined8 *param_3)

{
  undefined8 uVar1;
  size_t sVar2;
  long in_FS_OFFSET;
  char *local_40;
  char *local_38;
  size_t local_30;
  size_t local_28;
  iconv_t local_20;
  char *local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_20 = (iconv_t)FUN_0013c7e3(param_1);
  if (local_20 == (iconv_t)0x0) {
    uVar1 = FUN_00160cc6(param_2);
    *param_3 = uVar1;
    uVar1 = 1;
  }
  else if (local_20 == (iconv_t)0xffffffffffffffff) {
    uVar1 = 0;
  }
  else {
    local_30 = strlen(param_2);
    local_30 = local_30 + 1;
    local_28 = local_30 * 0x10 + 1;
    local_38 = (char *)FUN_00160b5b(local_28);
    local_40 = param_2;
    local_18 = local_38;
    sVar2 = iconv(local_20,&local_40,&local_30,&local_38,&local_28);
    if (sVar2 == 0) {
      *local_38 = '\0';
      *param_3 = local_18;
      uVar1 = 1;
    }
    else {
      free(local_18);
      uVar1 = 0;
    }
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return uVar1;
}




// Function: string_ascii_p @ 0x3c9b7

undefined8 string_ascii_p(char *param_1)

{
  char *local_10;
  
  local_10 = param_1;
  while( true ) {
    if (*local_10 == '\0') {
      return 1;
    }
    if (*local_10 < '\0') break;
    local_10 = local_10 + 1;
  }
  return 0;
}




// Function: set_warning_option @ 0x3c9ee

void set_warning_option(char *param_1)

{
  bool bVar1;
  int iVar2;
  size_t sVar3;
  long lVar4;
  char *local_20;
  
  bVar1 = false;
  iVar2 = strcmp(param_1,"none");
  if (iVar2 == 0) {
    DAT_0019ec84 = 0;
  }
  else {
    sVar3 = strlen(param_1);
    local_20 = param_1;
    if (2 < sVar3) {
      iVar2 = memcmp(param_1,&DAT_00187b57,3);
      if (iVar2 == 0) {
        bVar1 = true;
        local_20 = param_1 + 3;
      }
    }
    lVar4 = FUN_00147c61("--warning",local_20,&PTR_DAT_0019aa60,&DAT_0019ec20,4,PTR_FUN_0019ee48);
    if (bVar1) {
      DAT_0019ec84 = DAT_0019ec84 & ~*(uint *)(&DAT_0019ec20 + lVar4 * 4);
    }
    else {
      DAT_0019ec84 = DAT_0019ec84 | *(uint *)(&DAT_0019ec20 + lVar4 * 4);
    }
  }
  return;
}




// Function: acls_one_line @ 0x3cae5

/* WARNING: Removing unreachable block (ram,0x0013cf31) */
/* WARNING: Removing unreachable block (ram,0x0013cf3f) */

void acls_one_line(char *param_1,undefined1 param_2,char *param_3,ulong param_4)

{
  undefined1 *puVar1;
  int iVar2;
  size_t sVar3;
  size_t sVar4;
  ulong uVar5;
  long in_FS_OFFSET;
  char *local_110;
  int local_ec;
  undefined1 local_68 [8];
  long local_60;
  undefined1 *local_58;
  undefined1 *local_50;
  undefined1 *local_48;
  ulong local_38;
  byte local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  sVar3 = strlen(param_1);
  local_ec = 0;
  if ((param_3 != (char *)0x0) && (param_4 != 0)) {
    _obstack_begin(local_68,0,0,FUN_00160b5b,PTR_free_0019bfc0);
    local_110 = param_3;
    for (; (ulong)(long)local_ec <= param_4; local_ec = local_ec + iVar2 + 1) {
      sVar4 = strcspn(local_110,",\n");
      iVar2 = (int)sVar4;
      if (iVar2 == 0) break;
      if (param_3 != local_110) {
        if (local_48 == local_50) {
          _obstack_newchunk(local_68,1);
        }
        *local_50 = param_2;
        local_50 = local_50 + 1;
      }
      uVar5 = (ulong)(int)sVar3;
      if ((ulong)((long)local_48 - (long)local_50) < uVar5) {
        _obstack_newchunk(local_68,uVar5);
      }
      memcpy(local_50,param_1,uVar5);
      local_50 = local_50 + uVar5;
      uVar5 = (ulong)iVar2;
      if ((ulong)((long)local_48 - (long)local_50) < uVar5) {
        _obstack_newchunk(local_68,uVar5);
      }
      memcpy(local_50,local_110,uVar5);
      local_50 = local_50 + uVar5;
      local_110 = local_110 + (long)iVar2 + 1;
    }
    if (local_48 == local_50) {
      _obstack_newchunk(local_68,1);
    }
    puVar1 = local_58;
    *local_50 = 0;
    if (local_58 == local_50 + 1) {
      local_18 = local_18 | 2;
    }
    local_50 = (undefined1 *)(~local_38 & (ulong)(local_50 + 1 + local_38));
    if ((ulong)((long)local_48 - local_60) < (ulong)((long)local_50 - local_60)) {
      local_50 = local_48;
    }
    local_58 = local_50;
    fprintf(DAT_0019efd0,"%s",puVar1);
    _obstack_free(local_68,0);
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}




// Function: xattrs_acls_get @ 0x3cf89

void xattrs_acls_get(void)

{
  undefined8 uVar1;
  
  if (0 < DAT_0019f968) {
    if (DAT_0019fdb0 == 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar1 = gettext("POSIX ACL support is not available");
      error(0,0,uVar1);
    }
    DAT_0019fdb0 = 1;
  }
  return;
}




// Function: xattrs_acls_set @ 0x3d003

void xattrs_acls_set(undefined8 param_1,undefined8 param_2,char param_3)

{
  undefined8 uVar1;
  
  if ((0 < DAT_0019f968) && (param_3 != '2')) {
    if (DAT_0019fdb4 == 0) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar1 = gettext("POSIX ACL support is not available");
      error(0,0,uVar1);
    }
    DAT_0019fdb4 = 1;
  }
  return;
}




// Function: mask_map_realloc @ 0x3d07e

void mask_map_realloc(undefined8 *param_1)

{
  undefined8 uVar1;
  
  if (param_1[2] == param_1[1]) {
    if (param_1[1] == 0) {
      param_1[1] = 4;
    }
    uVar1 = FUN_00160a5f(*param_1,param_1 + 1,8);
    *param_1 = uVar1;
  }
  return;
}




// Function: xattrs_mask_add @ 0x3d0e5

void xattrs_mask_add(undefined8 param_1,char param_2)

{
  long lVar1;
  long *plVar2;
  
  if (param_2 == '\0') {
    plVar2 = (long *)&DAT_0019fd98;
  }
  else {
    plVar2 = &DAT_0019fd80;
  }
  FUN_0013d07e(plVar2);
  lVar1 = plVar2[2];
  plVar2[2] = lVar1 + 1;
  *(undefined8 *)(*plVar2 + lVar1 * 8) = param_1;
  return;
}




// Function: clear_mask_map @ 0x3d14d

void clear_mask_map(undefined8 *param_1)

{
  if (param_1[1] != 0) {
    free((void *)*param_1);
  }
  return;
}




// Function: xattrs_clear_setup @ 0x3d17c

void xattrs_clear_setup(void)

{
  FUN_0013d14d(&DAT_0019fd80);
  FUN_0013d14d(&DAT_0019fd98);
  return;
}




// Function: xattrs_xattrs_get @ 0x3d1a5

void xattrs_xattrs_get(undefined4 param_1,undefined8 param_2,undefined8 param_3,int param_4)

{
  char cVar1;
  char *pcVar2;
  size_t sVar3;
  long lVar4;
  int *piVar5;
  long local_28;
  char *local_20;
  
  if (0 < DAT_0019f96c) {
    if (DAT_0019fdb8 == (char *)0x0) {
      DAT_0019fdb8 = (char *)FUN_00160a5f(0,&DAT_0019ec88,1);
    }
    while( true ) {
      if (param_4 == 0) {
        local_28 = FUN_00147394(param_1,param_2,DAT_0019fdb8,DAT_0019ec88);
      }
      else {
        local_28 = flistxattr(param_4,DAT_0019fdb8,DAT_0019ec88);
      }
      if ((local_28 != -1) || (piVar5 = __errno_location(), *piVar5 != 0x22)) break;
      DAT_0019fdb8 = (char *)FUN_00160a5f(DAT_0019fdb8,&DAT_0019ec88,1);
    }
    if (local_28 == -1) {
      if (param_4 == 0) {
        pcVar2 = "llistxattrat";
      }
      else {
        pcVar2 = "flistxattr";
      }
      FUN_0013e038(pcVar2,param_2);
    }
    else {
      local_20 = DAT_0019fdb8;
      if (DAT_0019fdc0 == (void *)0x0) {
        DAT_0019fdc0 = (void *)FUN_00160a5f(0,&DAT_0019ec90,1);
      }
      for (; 0 < local_28; local_28 = (local_28 - sVar3) + -1) {
        sVar3 = strlen(local_20);
        while( true ) {
          if (param_4 == 0) {
            lVar4 = FUN_00146e17(param_1,param_2,local_20,DAT_0019fdc0,DAT_0019ec90);
          }
          else {
            lVar4 = fgetxattr(param_4,local_20,DAT_0019fdc0,DAT_0019ec90);
          }
          if ((lVar4 != -1) || (piVar5 = __errno_location(), *piVar5 != 0x22)) break;
          DAT_0019fdc0 = (void *)FUN_00160a5f(DAT_0019fdc0,&DAT_0019ec90,1);
        }
        if (lVar4 == -1) {
          piVar5 = __errno_location();
          if (*piVar5 != 0x3d) {
            if (param_4 == 0) {
              pcVar2 = "lgetxattrat";
            }
            else {
              pcVar2 = "fgetxattr";
            }
            FUN_0013e038(pcVar2,param_2);
          }
        }
        else {
          cVar1 = FUN_0013d84f(local_20,1);
          if (cVar1 != '\x01') {
            FUN_0011d78e(param_3,local_20,DAT_0019fdc0,lVar4);
          }
        }
        local_20 = local_20 + sVar3 + 1;
      }
    }
  }
  return;
}




// Function: xattrs__fd_set @ 0x3d44b

void xattrs__fd_set(undefined8 param_1,undefined8 param_2,char param_3,undefined8 param_4,long param_5
                 ,undefined8 param_6)

{
  undefined8 uVar1;
  int *piVar2;
  int local_24;
  char *local_20;
  
  if (param_5 != 0) {
    local_20 = "setxattrat";
    if (param_3 == '2') {
      local_20 = "lsetxattr";
      local_24 = FUN_00146870(DAT_0019c268,param_2,param_4,param_5,param_6,0);
    }
    else {
      local_24 = FUN_00146591(DAT_0019c268,param_2,param_4,param_5,param_6,0);
    }
    if ((local_24 == -1) && ((DAT_0019ec84 & 0x200000) != 0)) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar1 = gettext("%s: Cannot set \'%s\' extended attribute for file \'%s\'");
      piVar2 = __errno_location();
      error(0,*piVar2,uVar1,local_20,param_4,param_2);
    }
  }
  return;
}




// Function: xattrs_selinux_get @ 0x3d566

void xattrs_selinux_get(undefined4 param_1,undefined8 param_2,long param_3,int param_4)

{
  int iVar1;
  int *piVar2;
  char *pcVar3;
  
  if (0 < DAT_0019f964) {
    if (param_4 == 0) {
      iVar1 = FUN_0015f56b(param_1,param_2,param_3 + 0x30);
    }
    else {
      iVar1 = FUN_00161d8a(param_4,param_3 + 0x30);
    }
    if (iVar1 == -1) {
      piVar2 = __errno_location();
      if (*piVar2 != 0x3d) {
        piVar2 = __errno_location();
        if (*piVar2 != 0x5f) {
          if (param_4 == 0) {
            pcVar3 = "lgetfileconat";
          }
          else {
            pcVar3 = "fgetfilecon";
          }
          FUN_0013e038(pcVar3,param_2);
        }
      }
    }
  }
  return;
}




// Function: xattrs_selinux_set @ 0x3d609

void xattrs_selinux_set(long param_1,undefined8 param_2,char param_3)

{
  undefined8 uVar1;
  int *piVar2;
  int local_24;
  char *local_20;
  
  if ((0 < DAT_0019f964) && (*(long *)(param_1 + 0x30) != 0)) {
    if (param_3 == '2') {
      local_24 = FUN_0015fa87(DAT_0019c268,param_2,*(undefined8 *)(param_1 + 0x30));
      local_20 = "lsetfileconat";
    }
    else {
      local_24 = FUN_0015f7f9(DAT_0019c268,param_2,*(undefined8 *)(param_1 + 0x30));
      local_20 = "setfileconat";
    }
    if ((local_24 == -1) && ((DAT_0019ec84 & 0x200000) != 0)) {
      if (DAT_0019fdc8 != (code *)0x0) {
        (*DAT_0019fdc8)();
      }
      uVar1 = gettext("%s: Cannot set SELinux context for file \'%s\'");
      piVar2 = __errno_location();
      error(0,*piVar2,uVar1,local_20,param_2);
    }
  }
  return;
}




// Function: xattrs_matches_mask @ 0x3d717

undefined8 xattrs_matches_mask(char *param_1,long *param_2)

{
  int iVar1;
  int local_c;
  
  if (param_2[1] != 0) {
    for (local_c = 0; (ulong)(long)local_c < (ulong)param_2[2]; local_c = local_c + 1) {
      iVar1 = fnmatch(*(char **)((long)local_c * 8 + *param_2),param_1,0);
      if (iVar1 == 0) {
        return 1;
      }
    }
  }
  return 0;
}




// Function: xattrs_kw_included @ 0x3d79b

undefined8 xattrs_kw_included(char *param_1,char param_2)

{
  int iVar1;
  undefined8 uVar2;
  undefined4 extraout_var;
  
  if (DAT_0019fd88 == 0) {
    if (param_2 == '\0') {
      iVar1 = strncmp(param_1,"user.",5);
      uVar2 = CONCAT71((int7)(CONCAT44(extraout_var,iVar1) >> 8),iVar1 == 0);
    }
    else {
      uVar2 = 1;
    }
  }
  else {
    uVar2 = FUN_0013d717(param_1,&DAT_0019fd80);
  }
  return uVar2;
}




// Function: xattrs_kw_excluded @ 0x3d803

undefined8 xattrs_kw_excluded(undefined8 param_1)

{
  char cVar1;
  
  if ((DAT_0019fda0 != 0) && (cVar1 = FUN_0013d717(param_1,&DAT_0019fd98), cVar1 != '\0')) {
    return 1;
  }
  return 0;
}




// Function: xattrs_masked_out @ 0x3d84f

undefined8 xattrs_masked_out(undefined8 param_1,undefined1 param_2)

{
  char cVar1;
  
  cVar1 = FUN_0013d79b(param_1,param_2);
  if ((cVar1 != '\0') && (cVar1 = FUN_0013d803(param_1,param_2), cVar1 == '\0')) {
    return 0;
  }
  return 1;
}




// Function: xattrs_xattrs_set @ 0x3d8a1

void xattrs_xattrs_set(long param_1,undefined8 param_2,char param_3,uint param_4)

{
  char *__s1;
  long lVar1;
  char cVar2;
  int iVar3;
  size_t sVar4;
  ulong local_18;
  
  if ((0 < DAT_0019f96c) && (local_18 = 0, *(long *)(param_1 + 0x158) != 0)) {
    for (; local_18 < *(ulong *)(param_1 + 0x158); local_18 = local_18 + 1) {
      lVar1 = *(long *)(local_18 * 0x18 + *(long *)(param_1 + 0x160));
      sVar4 = strlen("SCHILY.xattr.");
      __s1 = (char *)(lVar1 + sVar4);
      if (((param_3 != '0') || (iVar3 = strcmp(__s1,"security.capability"), param_4 != (iVar3 != 0))
          ) && (cVar2 = FUN_0013d84f(__s1,0), cVar2 == '\0')) {
        FUN_0013d44b(param_1,param_2,(int)param_3,__s1,
                     *(undefined8 *)(local_18 * 0x18 + *(long *)(param_1 + 0x160) + 8),
                     *(undefined8 *)(local_18 * 0x18 + *(long *)(param_1 + 0x160) + 0x10));
      }
    }
  }
  return;
}




// Function: xattrs_print_char @ 0x3d9ec

void xattrs_print_char(long param_1,undefined1 *param_2)

{
  long lVar1;
  char cVar2;
  size_t sVar3;
  int local_24;
  
  if (DAT_0019f9ac < 2) {
    *param_2 = 0;
  }
  else {
    if (((0 < DAT_0019f96c) || (0 < DAT_0019f964)) || (0 < DAT_0019f968)) {
      *param_2 = 0x20;
      param_2[1] = 0;
    }
    if ((0 < DAT_0019f96c) && (*(long *)(param_1 + 0x158) != 0)) {
      for (local_24 = 0; (ulong)(long)local_24 < *(ulong *)(param_1 + 0x158);
          local_24 = local_24 + 1) {
        lVar1 = *(long *)((long)local_24 * 0x18 + *(long *)(param_1 + 0x160));
        sVar3 = strlen("SCHILY.xattr.");
        cVar2 = FUN_0013d84f(sVar3 + lVar1,0);
        if (cVar2 != '\x01') {
          *param_2 = 0x2a;
          break;
        }
      }
    }
    if ((0 < DAT_0019f964) && (*(long *)(param_1 + 0x30) != 0)) {
      *param_2 = 0x2e;
    }
    if ((0 < DAT_0019f968) && ((*(long *)(param_1 + 0x40) != 0 || (*(long *)(param_1 + 0x50) != 0)))
       ) {
      *param_2 = 0x2b;
    }
  }
  return;
}




// Function: xattrs_print @ 0x3db33

void xattrs_print(long param_1)

{
  char cVar1;
  size_t sVar2;
  long lVar3;
  int local_24;
  
  if (2 < DAT_0019f9ac) {
    if ((0 < DAT_0019f964) && (*(long *)(param_1 + 0x30) != 0)) {
      fprintf(DAT_0019efd0,"  s: %s\n",*(undefined8 *)(param_1 + 0x30));
    }
    if ((0 < DAT_0019f968) && ((*(long *)(param_1 + 0x40) != 0 || (*(long *)(param_1 + 0x50) != 0)))
       ) {
      fprintf(DAT_0019efd0,"  a: ");
      FUN_0013cae5(&DAT_00187cb4,0x2c,*(undefined8 *)(param_1 + 0x38),
                   *(undefined8 *)(param_1 + 0x40));
      if ((*(long *)(param_1 + 0x40) != 0) && (*(long *)(param_1 + 0x50) != 0)) {
        fprintf(DAT_0019efd0,",");
      }
      FUN_0013cae5("default:",0x2c,*(undefined8 *)(param_1 + 0x48),*(undefined8 *)(param_1 + 0x50));
      fprintf(DAT_0019efd0,"\n");
    }
    if ((0 < DAT_0019f96c) && (*(long *)(param_1 + 0x158) != 0)) {
      for (local_24 = 0; (ulong)(long)local_24 < *(ulong *)(param_1 + 0x158);
          local_24 = local_24 + 1) {
        lVar3 = *(long *)((long)local_24 * 0x18 + *(long *)(param_1 + 0x160));
        sVar2 = strlen("SCHILY.xattr.");
        lVar3 = sVar2 + lVar3;
        cVar1 = FUN_0013d84f(lVar3,0);
        if (cVar1 != '\x01') {
          fprintf(DAT_0019efd0,"  x: %lu %s\n",
                  *(undefined8 *)((long)local_24 * 0x18 + *(long *)(param_1 + 0x160) + 0x10),lVar3);
        }
      }
    }
  }
  return;
}




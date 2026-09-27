# Cast snippets, round I (`b3878d32e`) -> final (`632437155`), with IDA

Printed by `castshow.py <round-I arm> <final arm> OPT PROJECT BIN ADDR` from the castbench arms; names from the unstripped twins.

## gzip `pqdownheap` O0 — 50 casts -> 8 (IDA 23)

```c
===== round I
// Function: sub_b0dd @ 0xb0dd
void sub_b0dd(long a0,int a1)
{
  int v1;
  int v2; // stack - 0x24
  int v3; // stack - 0x10
  
  v1 = *(int *)((long)a1 * 4 + 0xdf200);
  v2 = a1;
  for (v3 = a1 * 2; v3 <= dat_dfaf4; v3 = v3 << 1) {
    if ((v3 < dat_dfaf4) && ((*(unsigned short *)(a0 + (long)*(int *)((long)(v3 + 1) * 4 + 0xdf200) * 4) < *(unsigned short *)(a0 + (long)*(int *)((long)v3 * 4 + 0xdf200) * 4) || ((*(short *)(a0 + (long)*(int *)((long)(v3 + 1) * 4 + 0xdf200) * 4) == *(short *)(a0 + (long)*(int *)((long)v3 * 4 + 0xdf200) * 4) && (*(unsigned char *)((long)*(int *)((long)(v3 + 1) * 4 + 0xdf200) + 0xdfb00) <= *(unsigned char *)((long)*(int *)((long)v3 * 4 + 0xdf200) + 0xdfb00)))))))
      v3 += 1;
    if ((*(unsigned short *)(a0 + (long)v1 * 4) < *(unsigned short *)(a0 + (long)*(int *)((long)v3 * 4 + 0xdf200) * 4)) || ((*(short *)(a0 + (long)v1 * 4) == *(short *)(a0 + (long)*(int *)((long)v3 * 4 + 0xdf200) * 4) && (*(unsigned char *)((long)v1 + 0xdfb00) <= *(unsigned char *)((long)*(int *)((long)v3 * 4 + 0xdf200) + 0xdfb00))))) break;
    *(unsigned int *)((long)v2 * 4 + 0xdf200) = *(unsigned int *)((long)v3 * 4 + 0xdf200);
    v2 = v3;
  }
  *(int *)((long)v2 * 4 + 0xdf200) = v1;
}


===== final
// Function: sub_b0dd @ 0xb0dd
void sub_b0dd(long a0,int a1)
{
  int v1;
  int v2; // stack - 0x24
  int v3; // stack - 0x10
  
  v1 = dat_df200[a1];
  v2 = a1;
  for (v3 = a1 * 2; v3 <= dat_dfaf4; v3 = v3 << 1) {
    if ((v3 < dat_dfaf4) && ((*(unsigned short *)(a0 + dat_df200[v3 + 1] * 4L) < *(unsigned short *)(a0 + dat_df200[v3] * 4L) || ((*(short *)(a0 + dat_df200[v3 + 1] * 4L) == *(short *)(a0 + dat_df200[v3] * 4L) && (dat_dfb00[dat_df200[v3 + 1]] <= dat_dfb00[dat_df200[v3]]))))))
      v3 += 1;
    if ((*(unsigned short *)(a0 + v1 * 4L) < *(unsigned short *)(a0 + dat_df200[v3] * 4L)) || ((*(short *)(a0 + v1 * 4L) == *(short *)(a0 + dat_df200[v3] * 4L) && (dat_dfb00[v1] <= dat_dfb00[dat_df200[v3]])))) break;
    dat_df200[v2] = dat_df200[v3];
    v2 = v3;
  }
  dat_df200[v2] = v1;
}


===== ida
// Function: pqdownheap @ 0xb0dd
long long pqdownheap(long long a1, int a2)
{
  int v3; // [rsp+0h] [rbp-1Ch]
  int i; // [rsp+14h] [rbp-8h]
  unsigned int v5; // [rsp+18h] [rbp-4h]

  v3 = a2;
  v5 = *((int *)&unk_DF200 + a2);
  for ( i = 2 * a2; i <= dword_DFAF4; i *= 2 )
  {
    if ( i < dword_DFAF4
      && (*(short *)(4LL * *((int *)&unk_DF200 + i + 1) + a1) < *(short *)(4LL * *((int *)&unk_DF200 + i) + a1)
       || *(short *)(4LL * *((int *)&unk_DF200 + i + 1) + a1) == *(short *)(4LL * *((int *)&unk_DF200 + i) + a1)
       && byte_DFB00[*((int *)&unk_DF200 + i + 1)] <= byte_DFB00[*((int *)&unk_DF200 + i)]) )
    {
      ++i;
    }
    if ( *(short *)(4LL * (int)v5 + a1) < *(short *)(4LL * *((int *)&unk_DF200 + i) + a1)
      || *(short *)(4LL * (int)v5 + a1) == *(short *)(4LL * *((int *)&unk_DF200 + i) + a1)
      && byte_DFB00[v5] <= byte_DFB00[*((int *)&unk_DF200 + i)] )
    {
      break;
    }
    *((int *)&unk_DF200 + v3) = *((int *)&unk_DF200 + i);
    v3 = i;
  }
  *((int *)&unk_DF200 + v3) = v5;
  return v5;
}



```

## ls `getenv_quoting_style` O2-noinline — 5 casts -> 0 (IDA 0)

```c
===== round I
// Function: sub_64b0 @ 0x64b0
unsigned int sub_64b0(void) // early-return x2
{
  int v1; // eax
  char *v2; // rax
  
  v2 = getenv("QUOTING_STYLE");
  if (!v2)
    return 0xffffffff;
  v1 = sub_cf70(v2,(long *)0x259e0,(void *)0x1d9c0,4);
  if (0 <= v1)
    return *(unsigned int *)((long)v1 * 4 + 0x1d9c0);
  v2 = (char *)sub_158c0(v2);
  error(0,0,dcgettext(NULL,"ignoring invalid value of environment variable QUOTING_STYLE: %s",5),v2);
  return 0xffffffff;
}


===== final
// Function: sub_64b0 @ 0x64b0
unsigned int sub_64b0(void) // early-return x2
{
  int v1; // eax
  char *v2; // rax
  
  v2 = getenv("QUOTING_STYLE");
  if (!v2)
    return 0xffffffff;
  v1 = sub_cf70(v2,&dat_259e0,dat_1d9c0,4);
  if (0 <= v1)
    return dat_1d9c0[v1];
  v2 = sub_158c0(v2);
  error(0,0,dcgettext(NULL,"ignoring invalid value of environment variable QUOTING_STYLE: %s",5),v2);
  return 0xffffffff;
}


===== ida
// Function: getenv_quoting_style @ 0x64b0
long long getenv_quoting_style()
{
  char *v0; // rax
  char *v1; // rbp
  int v2; // eax
  long long v4; // r12
  char *v5; // rax

  v0 = getenv("QUOTING_STYLE");
  if ( !v0 )
    return 0xFFFFFFFFLL;
  v1 = v0;
  v2 = sub_CF70(v0);
  if ( v2 >= 0 )
    return dword_1D9C0[v2];
  v4 = sub_158C0(v1, off_259E0);
  v5 = dcgettext(0, "ignoring invalid value of environment variable QUOTING_STYLE: %s", 5);
  error(0, 0, v5, v4);
  return 0xFFFFFFFFLL;
}



```

## cmp `file_position` O0 — 12 casts -> 4 (IDA 0)

```c
===== round I
// Function: sub_447f @ 0x447f
unsigned long sub_447f(int a0)
{
  unsigned int v1;
  unsigned long v2;
  
  if (*(char *)((long)a0 + 0x10215) != '\x01') {
    *(char *)((long)a0 + 0x10215) = 1;
    v2 = *(unsigned long *)((long)a0 * 8 + 0x10200);
    v1 = *(unsigned int *)((long)a0 * 4 + 0x100b0);
    *(unsigned long *)((long)a0 * 8 + 0x10220) = lseek(v1,v2,1);
  }
  return *(unsigned long *)((long)a0 * 8 + 0x10220);
}


===== final
// Function: sub_447f @ 0x447f
unsigned long sub_447f(int a0)
{
  unsigned int v1;
  unsigned long v2;
  
  if (dat_10215[a0] != '\x01') {
    dat_10215[a0] = '\x01';
    v2 = *(unsigned long *)(a0 * 8L + 0x10200);
    v1 = *(unsigned int *)(a0 * 4L + 0x100b0);
    *(unsigned long *)(a0 * 8L + 0x10220) = lseek(v1,v2,1);
  }
  return *(unsigned long *)(a0 * 8L + 0x10220);
}


===== ida
// Function: file_position @ 0x447f
long long file_position(int a1)
{
  if ( byte_10215[a1] != 1 )
  {
    byte_10215[a1] = 1;
    qword_10220[a1] = lseek(dword_100B0[a1], qword_10200[a1], 1);
  }
  return qword_10220[a1];
}



```

## tar `base64_init` O0 — 5 casts -> 2 (IDA 2)

```c
===== round I
// Function: sub_254c9 @ 0x254c9
void sub_254c9(void)
{
  int4 v1; // stack - 0xc
  
  memset((void *)0x9f4a0,0x40,0x100);
  for (v1 = 0; v1 <= 0x3f; v1 = v1 + 1) {
    *(char *)((int8)(int4)"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/%s: Omitting"[v1] + 0x9f4a0) = (char)v1;
  }
  return;
}


===== final
// Function: sub_254c9 @ 0x254c9
void sub_254c9(void)
{
  int4 v1; // stack - 0xc
  
  memset(dat_9f4a0,0x40,0x100);
  for (v1 = 0; v1 <= 0x3f; v1 = v1 + 1) {
    dat_9f4a0[(int4)"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/%s: Omitting"[v1]] = (char)v1;
  }
  return;
}


===== ida
// Function: base64_init @ 0x254c9
void *base64_init()
{
  void *result; // rax
  int i; // [rsp+Ch] [rbp-4h]

  result = memset(byte_9F4A0, 64, sizeof(byte_9F4A0));
  for ( i = 0; i <= 63; ++i )
  {
    result = (void *)aAbcdefghijklmn[i];
    byte_9F4A0[(long long)result] = i;
  }
  return result;
}



```

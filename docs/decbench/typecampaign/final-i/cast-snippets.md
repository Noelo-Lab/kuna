# Three real functions, round G → final (castbench text, `kuna decompile-all --json`), with IDA's text for reference

## tail::record_open_fd -O0 (castarith)

```c
// ===== round G (10a0db235)
// Function: sub_4435 @ 0x4435
void sub_4435(void *a0,unsigned int a1,unsigned long a2,struct_1 *a3,unsigned int a4)
{
  char v1 [16];
  
  *(unsigned int *)((long)a0 + 0x38) = a1;
  *(unsigned long *)((long)a0 + 8) = a2;
  v1 = sub_ee31(a3);
  *(long *)((long)a0 + 0x10) = SUB168(v1,0);
  *(long *)((long)a0 + 0x18) = SUB168(v1,8);
  *(unsigned long *)((long)a0 + 0x20) = a3->field_0x0;
  *(unsigned long *)((long)a0 + 0x28) = a3->field_0x8;
  *(unsigned int *)((long)a0 + 0x30) = a3->field_0x18;
  *(unsigned int *)((long)a0 + 0x40) = a4;
  *(unsigned long *)((long)a0 + 0x58) = 0;
  *(char *)((long)a0 + 0x34) = 0;
}


// ===== final (b3878d32e)
// Function: sub_4435 @ 0x4435
void sub_4435(void *a0,unsigned int a1,unsigned long a2,struct_1 *a3,unsigned int a4)
{
  char v1 [16];
  
  ((unsigned int *)a0)[0xe] = a1;
  ((unsigned long *)a0)[1] = a2;
  v1 = sub_ee31(a3);
  ((unsigned long *)a0)[2] = SUB168(v1,0);
  ((unsigned long *)a0)[3] = SUB168(v1,8);
  ((unsigned long *)a0)[4] = a3->field_0x0;
  ((unsigned long *)a0)[5] = a3->field_0x8;
  ((unsigned int *)a0)[0xc] = a3->field_0x18;
  ((unsigned int *)a0)[0x10] = a4;
  ((unsigned long *)a0)[0xb] = 0;
  ((char *)a0)[0x34] = 0;
}


// ===== IDA (decbench tree)
// Function: record_open_fd @ 0x4435
long long record_open_fd(long long a1, int a2, long long a3, long long a4, int a5)
{
  long long v5; // rdx

  *(int *)(a1 + 56) = a2;
  *(long long *)(a1 + 8) = a3;
  *(long long *)(a1 + 16) = sub_EE31(a4);
  *(long long *)(a1 + 24) = v5;
  *(long long *)(a1 + 32) = *(long long *)a4;
  *(long long *)(a1 + 40) = *(long long *)(a4 + 8);
  *(int *)(a1 + 48) = *(int *)(a4 + 24);
  *(int *)(a1 + 64) = a5;
  *(long long *)(a1 + 88) = 0;
  *(char *)(a1 + 52) = 0;
  return a1;
}



```

## find::pred_size -O2 (castarith+castimplied)

```c
// ===== round G (10a0db235)
// Function: sub_9820 @ 0x9820
unsigned long sub_9820(unsigned long a0,void *a1,void *a2)
{
  int v1;
  unsigned long v2; // rax
  undefined7 v3; // r8
  
  v1 = *(int *)((long)a2 + 0x38);
  v2 = (*(long *)((long)a1 + 0x30) / (long)*(int *)((long)a2 + 0x3c) + 1) - (unsigned long)(*(long *)((long)a1 + 0x30) % (long)*(int *)((long)a2 + 0x3c) == 0);
  v3 = (undefined7)((unsigned long)a1 >> 8);
  if (v1 == 1)
    return CONCAT71(v3,v2 < *(unsigned long *)((long)a2 + 0x40)) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return (unsigned long)(*(unsigned long *)((long)a2 + 0x40) < v2);
  }
  return CONCAT71(v3,*(unsigned long *)((long)a2 + 0x40) == v2) & 0xffffffff;
}


// ===== final (b3878d32e)
// Function: sub_9820 @ 0x9820
unsigned long sub_9820(unsigned long a0,void *a1,void *a2)
{
  int v1;
  unsigned long v2; // rax
  undefined7 v3; // r8
  
  v1 = ((int *)a2)[0xe];
  v2 = (((long *)a1)[6] / (long)((int *)a2)[0xf] + 1) - (unsigned long)(((long *)a1)[6] % (long)((int *)a2)[0xf] == 0);
  v3 = (undefined7)((unsigned long)a1 >> 8);
  if (v1 == 1)
    return CONCAT71(v3,v2 < ((unsigned long *)a2)[8]) & 0xffffffff;
  if (v1 != 2) {
    if (v1)
      return 0;
    return ((unsigned long *)a2)[8] < v2;
  }
  return CONCAT71(v3,((unsigned long *)a2)[8] == v2) & 0xffffffff;
}


// ===== IDA (decbench tree)
// Function: pred_size @ 0x9820
long long pred_size(long long a1, long long a2, long long a3)
{
  unsigned int v3; // r8d
  long long v5; // rax
  bool v6; // cf
  int v7; // edx
  unsigned long long v8; // rax
  unsigned int v9; // r8d

  v3 = a2;
  v5 = *(long long *)(a2 + 48) / *(int *)(a3 + 60);
  v6 = *(long long *)(a2 + 48) % *(int *)(a3 + 60) == 0;
  v7 = *(int *)(a3 + 56);
  v8 = v5 - (v6 - 1LL);
  if ( v7 == 1 )
  {
    LOBYTE(v3) = *(long long *)(a3 + 64) > v8;
    return v3;
  }
  else if ( v7 == 2 )
  {
    LOBYTE(v3) = *(long long *)(a3 + 64) == v8;
    return v3;
  }
  else
  {
    v9 = 0;
    if ( !v7 )
      LOBYTE(v9) = *(long long *)(a3 + 64) < v8;
    return v9;
  }
}



```

## grep::drain_input -O0 (castsign+castimplied)

```c
// ===== round G (10a0db235)
// Function: sub_9272 @ 0x9272
unsigned long sub_9272(int a0,struct_12 *a1) // early-return x2
{
  unsigned int v1;
  unsigned long v2; // rax
  long v3; // rax
  unsigned long v4; // stack - 0x10
  
  if (((a1->field_0x18 & 0xf000) == 0x1000) && (dat_338b2)) {
    v2 = splice(a0,NULL,1,NULL,0x18000,1);
    v4 = v2;
    if ((long)v2 <= -1) {
      v1 = *__errno_location();
      v2 = (unsigned long)v1;
      if (v1 == 0x16) goto label_9331;
    }
    while (0 < (long)v4) {
      v2 = splice(a0,NULL,1,NULL,0x18000,1);
      v4 = v2;
    }
    return CONCAT71((undefined7)(v2 >> 8),v4 == 0);
  }
label_9331:
  do {
    v3 = sub_24c3e(a0,dat_337f0,dat_337f8);
    if (!v3)
      return 1;
  } while (v3 != -1);
  return 0;
}


// ===== final (b3878d32e)
// Function: sub_9272 @ 0x9272
unsigned long sub_9272(int a0,struct_12 *a1) // early-return x2
{
  unsigned int v1;
  unsigned long v2; // rax
  long v3; // rax
  long v4; // stack - 0x10
  
  if (((a1->field_0x18 & 0xf000) == 0x1000) && (dat_338b2)) {
    v2 = splice(a0,NULL,1,NULL,0x18000,1);
    v4 = v2;
    if ((long)v2 <= -1) {
      v1 = *__errno_location();
      v2 = v1;
      if (v1 == 0x16) goto label_9331;
    }
    while (0 < v4) {
      v2 = splice(a0,NULL,1,NULL,0x18000,1);
      v4 = v2;
    }
    return CONCAT71((undefined7)(v2 >> 8),v4 == 0);
  }
label_9331:
  do {
    v3 = sub_24c3e(a0,dat_337f0,dat_337f8);
    if (!v3)
      return 1;
  } while (v3 != -1);
  return 0;
}


// ===== IDA (decbench tree)
// Function: drain_input @ 0x9272
bool drain_input(unsigned int a1, long long a2)
{
  ssize_t v3; // [rsp+18h] [rbp-8h]
  long long v4; // [rsp+18h] [rbp-8h]

  if ( (*(int *)(a2 + 24) & 0xF000) == 0x1000
    && byte_338B2
    && ((v3 = splice(a1, 0, 1, 0, 0x18000u, 1u), v3 >= 0) || *__errno_location() != 22) )
  {
    while ( v3 > 0 )
      v3 = splice(a1, 0, 1, 0, 0x18000u, 1u);
    return v3 == 0;
  }
  else
  {
    while ( 1 )
    {
      v4 = sub_24C3E(a1, ptr, qword_337F8);
      if ( !v4 )
        break;
      if ( v4 == -1 )
        return 0;
    }
    return 1;
  }
}



```

## grep::comsubs -O0 (castsign+castimplied)

```c
// ===== round G (10a0db235)
// Function: sub_1aa71 @ 0x1aa71
void * sub_1aa71(char *a0,char *a1)
{
  char v1;
  void *v2; // stack - 0x30
  char *v3; // stack - 0x28
  unsigned long v4; // stack - 0x20
  char *v5; // stack - 0x18
  unsigned long v6; // stack - 0x10
  
  v2 = (void *)sub_26f82(8);
  for (v3 = a0; *v3; v3 = &v3[1]) {
    v4 = 0;
    v1 = *v3;
    for (v5 = strchr(a1,(int)v1); v5; v5 = strchr(&v5[1],(int)v1)) {
      for (v6 = 1; (v3[v6] && (v3[v6] == v5[v6])); v6 = v6 + 1) {
      }
      if ((long)v4 < (long)v6)
        v4 = v6;
      v1 = *v3;
    }
    if (v4)
      v2 = (void *)sub_1aa32(v2,v3,v4);
  }
  return v2;
}


// ===== final (b3878d32e)
// Function: sub_1aa71 @ 0x1aa71
void * sub_1aa71(char *a0,char *a1)
{
  char v1;
  void *v2; // stack - 0x30
  char *v3; // stack - 0x28
  long v4; // stack - 0x20
  char *v5; // stack - 0x18
  unsigned long v6; // stack - 0x10
  
  v2 = (void *)sub_26f82(8);
  for (v3 = a0; *v3; v3 = &v3[1]) {
    v4 = 0;
    v1 = *v3;
    for (v5 = strchr(a1,v1); v5; v5 = strchr(&v5[1],v1)) {
      for (v6 = 1; (v3[v6] && (v3[v6] == v5[v6])); v6 = v6 + 1) {
      }
      if (v4 < (long)v6)
        v4 = v6;
      v1 = *v3;
    }
    if (v4)
      v2 = (void *)sub_1aa32(v2,v3,v4);
  }
  return v2;
}


// ===== IDA (decbench tree)
None
```


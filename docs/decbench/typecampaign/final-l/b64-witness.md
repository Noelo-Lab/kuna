# The base64 witness shape, on the public fixture

The user's witness for the cast campaign was a textbook base64 decoder. Its shape is reproduced here only through
the repo's public fixture `decompiler/crates/kuna-analysis/tests/fixtures/elemptr_x86_64.c` (`w_build` fills a `malloc`'d decoding table from a constant
alphabet; `w_decode` indexes it by input bytes and writes a `malloc`'d buffer). The checked-in stripped builds
are decompiled whole-binary (`kuna decompile-all <bin>`) with four builds of main; casts are castcount's.

| build | `w_decode` gcc -O0 / clang -O0 / gcc -O2 | `w_build` gcc -O0 / clang -O0 / gcc -O2 | whole fixture gcc -O0 / clang -O0 / gcc -O2 |
|---|---|---|---|
| round I `b3878d32e` | 33 / 29 / 28 | 9 / 9 / 2 | 146 / 134 / 189 |
| round J `c960fb18d` (witness reported) | 33 / 29 / 28 | 9 / 9 / 2 | 144 / 132 / 184 |
| round K `0096e984d` | 25 / 25 / 24 | 5 / 5 / 2 | 126 / 116 / 178 |
| **final `632437155`** | 7 / 7 / 9 | 1 / 1 / 1 | 46 / 40 / 120 |

`w_decode`, gcc -O0, round J (when the witness was reported) and final:

```c
// Function: w_decode @ 0x30001250
void * w_decode(long a0,unsigned long a1,unsigned long *a2) // early-return x2, ternary x4
{
  unsigned long v1;
  int v2; // eax
  int v3; // eax
  int v4; // eax
  unsigned int v5; // eax
  void *v6; // rax
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  if (!dat_300053e0)
    w_build();
  if (a1 & 3)
    return NULL;
  *a2 = (a1 >> 2) * 3;
  if (*(char *)(a0 + (a1 - 1)) == '=')
    *a2 = *a2 - 1;
  if (*(char *)(a0 + (a1 - 2)) == '=')
    *a2 = *a2 - 1;
  v6 = malloc(*a2 + 1);
  if (!v6)
    return NULL;
  v7 = 0;
  v8 = 0;
  while (v1 = v7, v7 < a1) {
    v2 = (*(char *)(v7 + a0) != '=') ? (int)*(char *)((unsigned long)*(unsigned char *)(v7 + a0) + dat_300053e0) : 0; // branch-flip
    v7 += 1;
    v3 = (*(char *)(v7 + a0) != '=') ? (int)*(char *)((unsigned long)*(unsigned char *)(v7 + a0) + dat_300053e0) : 0; // branch-flip
    v7 = v1 + 2;
    v4 = (*(char *)(v7 + a0) != '=') ? (int)*(char *)((unsigned long)*(unsigned char *)(v7 + a0) + dat_300053e0) : 0; // branch-flip
    v7 = v1 + 3;
    v5 = (*(char *)(v7 + a0) != '=') ? (int)*(char *)((unsigned long)*(unsigned char *)(v7 + a0) + dat_300053e0) : 0; // branch-flip
    v7 = v1 + 4;
    v5 += v2 * 0x40000 + v3 * 0x1000 + v4 * 0x40;
    if (v8 < *a2) {
      v1 = v8 + 1;
      *(char *)(v8 + (long)v6) = (char)(v5 >> 0x10);
      v8 = v1;
    }
    if (v8 < *a2) {
      v1 = v8 + 1;
      *(char *)(v8 + (long)v6) = (char)(v5 >> 8);
      v8 = v1;
    }
    if (v8 < *a2) {
      v1 = v8 + 1;
      *(char *)(v8 + (long)v6) = (char)v5;
      v8 = v1;
    }
  }
  *(char *)((long)v6 + *a2) = 0;
  return v6;
}
```

```c
// Function: w_decode @ 0x30001250
char * w_decode(char *a0,unsigned long a1,unsigned long *a2) // early-return x2, ternary x4
{
  unsigned long v1;
  int v2; // eax
  int v3; // eax
  int v4; // eax
  unsigned int v5; // eax
  char *v6; // rax
  unsigned long v7; // stack - 0x20
  unsigned long v8; // stack - 0x18
  
  if (!dat_300053e0)
    w_build();
  if (a1 & 3)
    return NULL;
  *a2 = (a1 >> 2) * 3;
  if (a0[a1 - 1] == '=')
    *a2 = *a2 - 1;
  if (a0[a1 - 2] == '=')
    *a2 = *a2 - 1;
  v6 = malloc(*a2 + 1);
  if (!v6)
    return NULL;
  v7 = 0;
  v8 = 0;
  while (v1 = v7, v7 < a1) {
    v2 = (a0[v7] != '=') ? dat_300053e0[(unsigned char)a0[v7]] : 0; // branch-flip
    v7 += 1;
    v3 = (a0[v7] != '=') ? dat_300053e0[(unsigned char)a0[v7]] : 0; // branch-flip
    v7 = v1 + 2;
    v4 = (a0[v7] != '=') ? dat_300053e0[(unsigned char)a0[v7]] : 0; // branch-flip
    v7 = v1 + 3;
    v5 = (a0[v7] != '=') ? dat_300053e0[(unsigned char)a0[v7]] : 0; // branch-flip
    v7 = v1 + 4;
    v5 += v2 * 0x40000 + v3 * 0x1000 + v4 * 0x40;
    if (v8 < *a2) {
      v1 = v8 + 1;
      v6[v8] = (char)(v5 >> 0x10);
      v8 = v1;
    }
    if (v8 < *a2) {
      v1 = v8 + 1;
      v6[v8] = (char)(v5 >> 8);
      v8 = v1;
    }
    if (v8 < *a2) {
      v1 = v8 + 1;
      v6[v8] = (char)v5;
      v8 = v1;
    }
  }
  v6[*a2] = '\0';
  return v6;
}
```

`w_build`, gcc -O0, round J and final:

```c
// Function: w_build @ 0x300011d6
void w_build(void)
{
  int v1; // stack - 0x10
  int v2; // stack - 0xc
  
  dat_300053e0 = malloc(0x100);
  for (v1 = 0; v1 <= 0xff; v1 = v1 + 1) {
    *(char *)((long)v1 + (long)dat_300053e0) = 0x80;
  }
  for (v2 = 0; v2 <= 0x3f; v2 = v2 + 1) {
    *(char *)((unsigned long)*(unsigned char *)((long)v2 + 0x30005080) + (long)dat_300053e0) = (char)v2;
  }
}
```

```c
// Function: w_build @ 0x300011d6
void w_build(void)
{
  int v1; // stack - 0x10
  int v2; // stack - 0xc
  
  dat_300053e0 = malloc(0x100);
  for (v1 = 0; v1 <= 0xff; v1 = v1 + 1) {
    dat_300053e0[v1] = '\x80';
  }
  for (v2 = 0; v2 <= 0x3f; v2 = v2 + 1) {
    dat_300053e0[dat_30005080[v2]] = (char)v2;
  }
}
```

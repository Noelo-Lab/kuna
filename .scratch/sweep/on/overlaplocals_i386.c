// Function: overlap_bytes @ 0x8049000
void overlap_bytes(void)
{
  char v1 [2]; // al
  char v2 [2]; // al
  char v3 [2]; // al
  char v4 [8]; // stack - 0x100
  char v5 [54];
  char v6 [58];
  char v7 [114];
  char v8 [8]; // stack - 0xbc
  char v9 [8]; // stack - 0x7c
  
  do {
    v1[0] = s_804a000[8];
    v1[1] = s_804a000[9];
    v8[0] = s_804a000[0];
    v8[1] = s_804a000[1];
    v8[2] = s_804a000[2];
    v8[3] = s_804a000[3];
    v8[4] = s_804a000[4];
    v8[5] = s_804a000[5];
    v8[6] = s_804a000[6];
    v8[7] = s_804a000[7];
    memset_fixture(v5,0,0x32);
    v2[0] = s_804a00b[8];
    v2[1] = s_804a00b[9];
    v4[0] = s_804a00b[0];
    v4[1] = s_804a00b[1];
    v4[2] = s_804a00b[2];
    v4[3] = s_804a00b[3];
    v4[4] = s_804a00b[4];
    v4[5] = s_804a00b[5];
    v4[6] = s_804a00b[6];
    v4[7] = s_804a00b[7];
    memset_fixture(v6,0,0x32);
    v3[0] = s_804a016[8];
    v3[1] = s_804a016[9];
    v9[0] = s_804a016[0];
    v9[1] = s_804a016[1];
    v9[2] = s_804a016[2];
    v9[3] = s_804a016[3];
    v9[4] = s_804a016[4];
    v9[5] = s_804a016[5];
    v9[6] = s_804a016[6];
    v9[7] = s_804a016[7];
    memset_fixture(v7,0,0x32);
    consume_bytes(v8);
    consume_bytes(v4);
    consume_bytes(v9);
  } while (dat_804b021);
}

// Function: memset_fixture @ 0x80490ca
void memset_fixture(void)
{
  return;
}

// Function: consume_bytes @ 0x80490cb
void consume_bytes(void)
{
  return;
}

// Function: _start @ 0x80490cc
void _start(void)
{
  overlap_bytes();
}


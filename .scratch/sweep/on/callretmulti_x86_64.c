// Function: sub_401000 @ 0x401000
void sub_401000(void)
{
  unsigned short v1;
  long v2; // rcx
  long v3; // rbx
  unsigned short v4 [16];
  
  v3 = 0;
label_40100d:
  do {
    if (!initialized) {
      init_one();
      init_two();
      initialized = '\x01';
    }
    v1 = (unsigned short)helper();
    v2 = 0;
    do {
      if (v4[v2] == v1) goto label_40100d;
      v2 += 1;
    } while (v2 < 0x10);
    v4[v3] = v1;
    v3 += 1;
  } while (v3 <= 0xf);
}

// Function: helper @ 0x401067
unsigned long helper(void)
{
  return 7;
}

// Function: init_one @ 0x40106d
unsigned long init_one(void)
{
  return 0;
}

// Function: init_two @ 0x401070
void init_two(void)
{
  return;
}


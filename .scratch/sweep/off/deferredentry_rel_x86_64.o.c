// Function: sub_400000 @ 0x400000
int sub_400000(int a0,int a1)
{
  int v1; // stack - 0x10
  int v2; // stack - 0xc
  
  v1 = a0 * a1;
  for (v2 = 0; v2 < a0; v2 = v2 + 1) {
    v1 += v2;
  }
  return v1;
}

// Function: sub_400038 @ 0x400038
int sub_400038(unsigned int a0)
{
  char v1 [72]; // stack - 0x58
  
  snprintf(v1,0x40,"%d",a0);
  return (int)v1[0];
}

// Function: sub_400095 @ 0x400095
int sub_400095(unsigned int a0,unsigned int a1,unsigned int a2)
{
  int v1; // eax
  
  v1 = sub_400000(a0,a1);
  return sub_400038(a2) + v1;
}

// Function: pub1 @ 0x4000ce
void pub1(int a0)
{
  sub_400095(a0,a0 + 1,a0 + 2);
}

// Function: pub2 @ 0x4000f7
int pub2(unsigned int a0,int a1)
{
  int v1; // stack - 0x10
  int v2; // stack - 0xc
  
  v1 = 0;
  for (v2 = 0; v2 < a1; v2 = v2 + 1) {
    v1 += sub_400000(v2,a0);
  }
  return v1;
}

// Function: pub3 @ 0x40013c
int pub3(void)
{
  int v1; // eax
  
  v1 = pub1(3);
  return pub2(4,5) + v1;
}

// Function: main @ 0x40016c
void main(void)
{
  pub3();
}


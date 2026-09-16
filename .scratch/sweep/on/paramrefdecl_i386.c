// Function: sub_8049000 @ 0x8049000
void sub_8049000(void)
{
  derive_key(1,2);
  do {
  } while( true );
}

// Function: derive_key @ 0x804900d
void derive_key(unsigned int a0,unsigned int a1)
{
  consume(&a0,a1);
}

// Function: consume @ 0x8049021
int consume(int *a0,int a1)
{
  return *a0 + a1;
}


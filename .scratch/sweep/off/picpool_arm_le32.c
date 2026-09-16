// Function: sub_400 @ 0x400
void sub_400(void)
{
  void *v1; // lr
  
  sub_420();
  sub_430();
  sub_448();
  (*v1)(); // jump-as-call
}

// Function: sub_420 @ 0x420
unsigned int sub_420(void)
{
  return 0x300;
}

// Function: sub_430 @ 0x430
unsigned long long sub_430(void)
{
  return 0x100000320;
}

// Function: sub_448 @ 0x448
unsigned int sub_448(void)
{
  return 0x340;
}


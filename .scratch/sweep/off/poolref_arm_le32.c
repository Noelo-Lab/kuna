// Function: sub_10000 @ 0x10000
void sub_10000(void)
{
  void *v1; // lr
  
  sub_10020();
  sub_1002c();
  sub_10034();
  sub_10040();
  (*v1)(); // jump-as-call
}

// Function: sub_10020 @ 0x10020
char * sub_10020(void)
{
  return "kuna poolref prompt";
}

// Function: sub_1002c @ 0x1002c
unsigned int sub_1002c(void)
{
  return dat_10180;
}

// Function: sub_10034 @ 0x10034
unsigned short sub_10034(void)
{
  return s_10120._0_2_;
}

// Function: sub_10040 @ 0x10040
unsigned int sub_10040(void)
{
  return 0x2a;
}


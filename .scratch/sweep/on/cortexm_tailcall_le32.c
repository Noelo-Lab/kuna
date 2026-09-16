// Function: sub_8008000 @ 0x8008000
void sub_8008000(void)
{
  sub_8008010();
  sub_8008020(); // tail-call
}

// Function: sub_8008010 @ 0x8008010
void sub_8008010(void)
{
  sub_8008030();
  do {
  } while( true );
}

// Function: sub_8008020 @ 0x8008020
unsigned int sub_8008020(void)
{
  return 0x2a;
}

// Function: sub_8008030 @ 0x8008030
unsigned int sub_8008030(void)
{
  sub_8008040();
  return 99;
}

// Function: sub_8008040 @ 0x8008040
void sub_8008040(void)
{
  sub_8008050();
}

// Function: sub_8008050 @ 0x8008050
unsigned int sub_8008050(void)
{
  return 9;
}


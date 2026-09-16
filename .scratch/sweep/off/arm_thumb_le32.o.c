// Function: thumb_add @ 0x400000
int thumb_add(int a0,int a1)
{
  return a0 + a1 * 3;
}

// Function: _start @ 0x400014
void _start(void)
{
  thumb_add(7,0xb);
  do {
  } while( true );
}


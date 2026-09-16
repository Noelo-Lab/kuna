// Function: sub_8049000 @ 0x8049000
void sub_8049000(void)
{
  unsigned int v1; // ebx
  
  v1 = 0;
  do {
    sub_8049027(2,v1,0,0x804a004);
    v1 += 1;
  } while (v1 <= 3);
  (*(void *)swi(0x80))(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: sub_8049027 @ 0x8049027
unsigned int sub_8049027(unsigned int a0)
{
  dat_804a000 = (char *)&a0;
  return a0;
}


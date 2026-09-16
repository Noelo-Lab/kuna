// Function: prompt_user @ 0x100000
void prompt_user(void) // warn: Function flows into unmapped memory
{
  syscall(1,1,0x101000,0x37);
  syscall(1,1,0x101038,0x1b);
  syscall(0x3c,0);
  halt_missing(); // warn: Function flow reaches unmapped memory: r0x00100037 flows to r0x00100039
}


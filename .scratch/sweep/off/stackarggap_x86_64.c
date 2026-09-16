// Function: sub_401000 @ 0x401000
void sub_401000(void)
{
  hash_final(0x402008);
  syscall(0x3c,0); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: hash_final @ 0x401015
void hash_final(unsigned long *a0)
{
  (*dat_402000)(a0[4],a0[3],a0[2],a0[1],*a0);
}


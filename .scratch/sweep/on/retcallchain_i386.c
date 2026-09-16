// Function: _start @ 0x8049000
void _start(void)
{
  chain_entry();
  (*(void *)swi(0x80))(); // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: chain_entry @ 0x804900e
void chain_entry(void)
{
  (*dat_804a000)();
  (*dat_804a004)();
  (*dat_804a008)();
}

// Function: first_link @ 0x8049033
unsigned int first_link(void)
{
  return 0x1111;
}

// Function: second_link @ 0x8049039
void second_link(void)
{
  return;
}

// Function: third_link @ 0x804903f
void third_link(void)
{
  return;
}


// Function: entry_dispatch @ 0x8049000
void entry_dispatch(void)
{
  (*dat_804a000)();
  (*dat_804a004)();
  (*dat_804a008)();
}

// Function: ordinary_ret @ 0x8049025
unsigned int ordinary_ret(void)
{
  return 0x1111;
}

// Function: immediate_ret @ 0x804902b
unsigned int immediate_ret(void)
{
  return 0x2222;
}

// Function: incoming_return_ret @ 0x8049033
void incoming_return_ret(void)
{
  return;
}

// Function: computed_ret @ 0x8049036
void computed_ret(void)
{
  return;
}

// Function: constant_ret @ 0x804903d
unsigned int constant_ret(void)
{
  switch(0x8049025) {
    case 0x8049025:
      return 0x1111;
    
  }
}

// Function: adjusted_fallthrough_ret @ 0x8049043
void adjusted_fallthrough_ret(void)
{
  return;
}

// Function: unrelated_fallthrough_store_ret @ 0x804904d
void unrelated_fallthrough_store_ret(void)
{
  dat_804b00c = 0x8049058;
}

// Function: negative_displacement_ret @ 0x8049059
void negative_displacement_ret(void)
{
  return;
}

// Function: conditional_bypass_ret @ 0x804906f
void conditional_bypass_ret(void)
{
  return;
}

// Function: partial_sp_ret @ 0x8049080
void partial_sp_ret(void)
{
  return;
}

// Function: first_link @ 0x8049091
unsigned int first_link(void)
{
  return 0x3333;
}

// Function: second_link @ 0x8049097
void second_link(void)
{
  return;
}

// Function: third_link @ 0x804909d
void third_link(void)
{
  return;
}


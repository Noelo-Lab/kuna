// Function: push_immediate_ret @ 0x8049000
void push_immediate_ret(void) // warn: Function flows into unmapped memory
{
  int v1; // eax
  
  v1 = unpacker();
  switch(0x804b000) {
    case 0x804b000:
      *(int *)(v1 + -0x74) = *(int *)(v1 + -0x74) - (int)&Stack00000000;
      halt_missing(); // warn: Function flow reaches unmapped memory: the instruction at r0x0804b00e runs past the mapped bytes
    
  }
}

// Function: unpacker @ 0x804900b
unsigned int unpacker(void)
{
  return 0x1111;
}

// Function: ordinary_ret @ 0x8049011
void ordinary_ret(void)
{
  return;
}

// Function: argument_push_later_ret @ 0x8049012
void argument_push_later_ret(void)
{
  argument_user(0x804b000);
}

// Function: argument_user @ 0x8049020
unsigned int argument_user(unsigned int a0)
{
  return a0;
}

// Function: stack_adjust_ret @ 0x8049025
void stack_adjust_ret(void)
{
  return;
}

// Function: stack_overwrite_ret @ 0x804902e
void stack_overwrite_ret(void)
{
  return;
}

// Function: computed_target_ret @ 0x8049037
void computed_target_ret(void)
{
  return;
}

// Function: conditional_bypass_ret @ 0x8049039
void conditional_bypass_ret(void)
{
  return;
}

// Function: two_push_dispatch @ 0x8049043
void two_push_dispatch(void)
{
  dispatch_helper();
}

// Function: dispatch_helper @ 0x804904f
void dispatch_helper(void)
{
  return;
}


// Function: shared_wrapper @ 0x401000
unsigned long shared_wrapper(int a0) // return-dupe
{
  if (!a0)
    return 7;
  return 0xffffffff;
}

// Function: callable_ret @ 0x40100f
void callable_ret(void)
{
  return;
}

// Function: after_ret @ 0x401010
unsigned long after_ret(void)
{
  return 0x21;
}

// Function: ordinary_first @ 0x401016
void ordinary_first(void)
{ // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: ordinary_next @ 0x40101b
unsigned long ordinary_next(void)
{
  return 0x16;
}

// Function: before_direct_branch @ 0x401021
void before_direct_branch(void)
{ // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: direct_branch_entry @ 0x401026
void direct_branch_entry(void)
{
  after_ret(); // tail-call
}

// Function: before_indirect_branch @ 0x401028
void before_indirect_branch(void)
{ // warn: funcboundflow: fall-through reached the next function entry; truncating flow here
}

// Function: indirect_branch_entry @ 0x40102d
void indirect_branch_entry(void)
{
  void *v1; // rax
  
  (*v1)(); // jump-as-call
}

// Function: ret_caller @ 0x40102f
unsigned long ret_caller(void)
{
  callable_ret();
  return 0x2c;
}

// Function: _start @ 0x40103a
void _start(void) // warn: Function flows into unmapped memory
{
  shared_wrapper(0);
  ordinary_first();
  before_direct_branch();
  before_indirect_branch();
  syscall(0x3c,(unsigned long)(unsigned int)ret_caller());
  halt_missing(); // warn: Function flow reaches unmapped memory: r0x0040105c flows to r0x0040105e
}


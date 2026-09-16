// Function: _DT_INIT @ 0x1000
void _DT_INIT(void) // return-dupe
{
  if (!dat_3ff8)
    return;
  (*dat_3ff8)();
}

// Function: sub_1020 @ 0x1020
void sub_1020(void)
{
  (*dat_4010)(); // jump-as-call
}

// Function: __cxa_finalize @ 0x1030
void __cxa_finalize(void)
{
  (*dat_3fd8)(); // jump-as-call
}

// Function: sub_1040 @ 0x1040
void sub_1040(void)
{
  return;
}

// Function: _FINI_0 @ 0x10b0
void _FINI_0(void)
{
  if (!dat_4020) {
    if (dat_3fd8)
      __cxa_finalize(dat_4018);
    sub_1040();
    dat_4020 = 1;
    return;
  }
}

// Function: _INIT_0 @ 0x10f0
void _INIT_0(void)
{
  return;
}

// Function: sig::Account::Account @ 0x10fa
void sig::Account::Account(Account *this,int a1)
{
  *(int *)this = a1;
  *(unsigned int *)((long)this + 4) = 0;
  _ZN3sig4sinkE = a1;
}

// Function: sig::Account::~Account @ 0x112c
void sig::Account::~Account(Account *this)
{
  _ZN3sig4sinkE = *(unsigned int *)this;
}

// Function: sig::Account::balance @ 0x114a
unsigned int sig::Account::balance(Account *this)
{
  return *(unsigned int *)this;
}

// Function: sig::Account::deposit @ 0x115e
int sig::Account::deposit(int *a0,unsigned int a1,int a2)
{
  _ZN3sig4sinkE = a1;
  *a0 = *a0 + a2;
  a0[1] = a0[1] + 1;
  return *a0;
}

// Function: sig::Account::rate @ 0x11aa
int sig::Account::rate(int a0)
{
  return a0 * 3 + 1;
}

// Function: sig::combine @ 0x11c3
int sig::combine(unsigned int a0,int a1)
{
  _ZN3sig4sinkE = a0;
  return a1 * 7;
}

// Function: sig_global @ 0x11ed
int sig_global(char *a0,int a1)
{
  _ZN3sig4sinkE = a1;
  return a1 + *a0;
}

// Function: _DT_FINI @ 0x121c
void _DT_FINI(void)
{
  return;
}


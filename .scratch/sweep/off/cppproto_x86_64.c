// Function: _init @ 0x401000
void _init(void) // return-dupe
{
  if (!dat_403ff0)
    return;
  (*dat_403ff0)();
}

// Function: sub_401020 @ 0x401020
void sub_401020(void)
{
  (*dat_404010)(); // jump-as-call
}

// Function: printf @ 0x401050
int printf(char *a0,...)
{
  int v1; // eax
  
  v1 = (*dat_404018)(); // jump-as-call
  return v1;
}

// Function: __stack_chk_fail @ 0x401060
void __stack_chk_fail(void)
{
  (*dat_404020)(); // jump-as-call
}

// Function: _start @ 0x401070
void _start(unsigned long a0,unsigned long a1,unsigned long a2)
{
  char v1 [8];
  unsigned long v2; // stack + 0x0
  
  (*dat_403fe8)(main,v2,&Stack0000000000000008,0,0,a2,v1);
  do {
  } while( true );
}

// Function: _dl_relocate_static_pie @ 0x4010a0
void _dl_relocate_static_pie(void)
{
  return;
}

// Function: deregister_tm_clones @ 0x4010b0
void deregister_tm_clones(void)
{
  return;
}

// Function: register_tm_clones @ 0x4010e0
void register_tm_clones(void)
{
  return;
}

// Function: __do_global_dtors_aux @ 0x401120
void __do_global_dtors_aux(void)
{
  if (!completed.0) {
    deregister_tm_clones();
    completed.0 = 1;
    return;
  }
}

// Function: frame_dummy @ 0x401150
void frame_dummy(void)
{
  register_tm_clones(); // tail-call
}

// Function: db::inner::scaled_add @ 0x401156
int db::inner::scaled_add(int a,int b)
{
  return b * 5 + a * 3;
}

// Function: Account::Account @ 0x40117c
void Account::Account(Account *this,int a1,int a2)
{
  *(int *)this = a1;
  *(int *)((long)this + 4) = a2;
  *(int *)((long)this + 8) = a2 * 2;
}

// Function: Account::deposit @ 0x4011b2
int Account::deposit(Account *this,int amount)
{
  this->balance = this->balance + amount;
  if (this->limit < this->balance)
    this->balance = this->limit;
  this->id = this->id + 1;
  return this->balance;
}

// Function: Account::available @ 0x40120c
int Account::available(Account *this)
{
  return this->id + (this->limit - this->balance);
}

// Function: Account::bump @ 0x401232
void Account::bump(Account *this,int *out)
{
  *out = *out + this->balance + this->limit;
}

// Function: Account::make_id @ 0x401264
int Account::make_id(int seed)
{
  return seed * 7 + 0xd;
}

// Function: probe_virtual_call @ 0x40127e
int probe_virtual_call(Shape *s,int scale)
{
  int v1; // eax
  
  v1 = (**s->_vptr.Shape)(s,scale);
  return s->tag + v1;
}

// Function: probe_template_int @ 0x4012b6
int probe_template_int(int a,int b)
{
  int v1; // eax
  
  v1 = maxof(a,b);
  return maxof(b,a) + v1;
}

// Function: probe_template_double @ 0x4012f1
double probe_template_double(double a,double b)
{
  double v1; // xmm0_qa
  
  v1 = maxof(a,b);
  return v1 + v1;
}

// Function: main @ 0x40132e
int main(int argc,char **argv)
{
  int o; // stack - 0x4c
  int t; // stack - 0x48
  int v1; // eax
  int v2; // eax
  Shape v3;
  Account v4;
  double v5; // xmm0_qa
  
  Shape::Shape((Shape *)&v3,argc);
  Account::Account((Account *)&v4,3,4);
  o = 5;
  Account::bump(&v4,&o);
  t = db::inner::scaled_add(argc,2);
  t += probe_virtual_call(&v3,argc);
  v1 = Account::deposit(&v4,argc);
  v2 = Account::available(&v4);
  t += o + v1 + v2 + Account::make_id(argc);
  t += probe_template_int(argc,4);
  v5 = probe_template_double((double)argc,dat_402010);
  t += (int)v5;
  printf("%d %s\n",(unsigned long)t,*argv);
  return t & 0xff;
}

// Function: Shape::Shape @ 0x401466
void Shape::Shape(Shape *this,int a1)
{
  *(void **)this = Shape_vptr;
  *(int *)((long)this + 8) = a1;
}

// Function: Shape::area @ 0x40148e
int Shape::area(Shape *this,int scale)
{
  return this->tag * scale;
}

// Function: maxof @ 0x4014aa
int maxof(int a,int b)
{
  if (b < a)
    b = a;
  return b;
}

// Function: maxof @ 0x4014ca
double maxof(double a,double b)
{
  if (a > b)
    b = a;
  return b;
}

// Function: _fini @ 0x401500
void _fini(void)
{
  return;
}


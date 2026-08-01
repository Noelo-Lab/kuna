__attribute__((noinline)) long callee(long a,long b,long c,long d,long e,long f,long h,long i)
{ return a+b+c+d+e+f+h*3+i*5; }
__attribute__((noinline)) long caller(long x,long y){ return callee(1,2,3,4,5,6,x,y); }
int main(int argc,char**argv){ return (int)caller(argc,(long)argv); }

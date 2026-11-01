#include <stdint.h>
#include <stddef.h>
#define KEEP __attribute__((noinline,used))
KEEP int64_t add(int64_t a,int64_t b) { return a+b; }
KEEP uint64_t tiers(uint64_t a,uint64_t b,uint64_t c) { return (a|b)^(c+(a&(b>>3))); }
KEEP int64_t choose(int64_t a,int64_t b) { if(a>10)return b+1; else if(a<0)return b-1; else return b*2; }
KEEP int64_t sum_for(int64_t n) { int64_t s=0; for(int64_t i=0;i<n;i++){if((i&1)==0)continue;s+=i;}return s; }
KEEP int64_t bottom(int64_t n) { int64_t s=0; do {n--;if((n&1)==0)continue;s+=n;}while(n>0);return s; }
KEEP int64_t bottom_call(int64_t n, int64_t *p) { int64_t s=0; do {s+=n;if(n&1){n--;continue;}n-=2;}while(add(n,*p)>0);return s; }
KEEP int64_t short_calls(int64_t a,int64_t b) { if(a>0 && add(a,b)>4)return 7;return 2; }
KEEP int64_t loop_calls(int64_t a,int64_t b) { int64_t s=0;while(a>0 && add(a,b)>4){s+=a;a--;}return s; }
KEEP int64_t dispatch(int64_t n,int64_t a) { switch(n){case 0:a+=3; __attribute__((fallthrough));case 1:a*=2;break;case 2:case 3:a-=4;break;case 4:a^=7;break;case 5:a+=8;break;default:a=-a;}return a; }
KEEP int32_t read_at(int32_t *p,int64_t i) {return p[i];}
KEEP void write_at(int32_t *p,int64_t i,int32_t v) {p[i]=v;}
KEEP int32_t *from_word(uintptr_t p) {return (int32_t*)p;}
KEEP uintptr_t to_word(int32_t *p) {return (uintptr_t)p;}
KEEP int32_t *retype(double *p) {return (int32_t*)p;}
KEEP uint32_t narrow(uint64_t a) {return (uint32_t)a;}
KEEP int64_t extend(int8_t a) {return a;}
KEEP double number(int64_t a) {return (double)a;}
KEEP double bits(uint64_t a) {union{uint64_t u;double f;} x={.u=a};return x.f;}
KEEP uint64_t unbits(double a) {union{uint64_t u;double f;} x={.f=a};return x.u;}
KEEP const char *text(void) {return "quote\" tab\t utf8\xc3\xa9";}
KEEP int rune(void) {return 0x1f600;}
KEEP double negative_zero(void) {return -0.0;}
KEEP double positive_infinity(void) {return __builtin_inf();}
KEEP double quiet_nan(void) {return __builtin_nan("0x123");}
int main(void) {return (int)add(1,2);}
KEEP void record(int64_t *p,int64_t tag) {*p=*p*10+tag;}
KEEP int64_t bottom_continue(int64_t n,int64_t *p) {do {record(p,1);n--;if(n&1){record(p,2);continue;}record(p,3);}while(n>0);return n;}
KEEP int64_t probe(int64_t *p,int64_t v) {*p+=1;return v;}
KEEP int64_t condition_calls(int64_t n,int64_t *p) {if(n>0 && probe(p,n)>2)return 7;return 3;}
KEEP int64_t condition_loop(int64_t n,int64_t *p) {int64_t s=0;while(n>0 && probe(p,n)>2){s+=n;n--;}return s;}
KEEP int64_t fallthroughs(int64_t n,int64_t a,int64_t *p) {switch(n){case 0:record(p,1);__attribute__((fallthrough));case 1:record(p,2);return a+1;case 2:record(p,3);__attribute__((fallthrough));case 3:record(p,4);return a+2;case 4:return a-1;case 5:return a-2;default:return a;}}
KEEP int64_t condition_value(int64_t n,int64_t *p) {int64_t v;if(n>0 && (v=probe(p,n))>2)return v*2;return 3;}
KEEP int64_t condition_for(int64_t n,int64_t *p) {int64_t s=0,v;for(int64_t i=0;(v=probe(p,n-i))>2;i++){if(i&1)continue;s+=v;}return s;}
KEEP int64_t name_collision(int64_t _kuna_first_1) {int64_t v=0;do{v+=_kuna_first_1;_kuna_first_1--;}while(_kuna_first_1>0);return v;}
KEEP int64_t go(int64_t range,int64_t _kuna_ident_72616e6765) {return range+_kuna_ident_72616e6765;}
KEEP int64_t callback(int64_t x) {return x*3;}
KEEP int64_t call_callback(int64_t (*fn)(int64_t),int64_t x) {return fn(x)+1;}
KEEP int32_t shadow(int32_t *unsafe,int64_t uint64) {return unsafe[uint64];}
KEEP _Bool bool_value(int64_t x) {return x!=0;}
KEEP int64_t bool_number(_Bool value) {return value?7:3;}
KEEP _Bool bool_true(void) {return 1;}
KEEP _Bool bool_false(void) {return 0;}
KEEP int32_t *null_pointer(void) {return 0;}
KEEP _Bool is_null(int32_t *p) {return p==0;}
KEEP int64_t shadow_null(int32_t *p,int64_t nil) {return p==0?nil:0;}
KEEP _Bool nested_calls(int64_t n,int64_t *p) {return n>0 && (probe(p,n)>2 || probe(p,n+2)<1);}

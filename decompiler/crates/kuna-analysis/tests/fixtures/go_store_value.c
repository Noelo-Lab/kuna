#include <stdint.h>
#define KEEP __attribute__((noinline,noclone))
struct Fields { signed char s8; unsigned char u8; short s16; unsigned short u16; int s32; unsigned int u32; long s64; unsigned long u64; };
KEEP void signed8(struct Fields *p,unsigned char a,unsigned char b){p->s8=(signed char)(a+b);}
KEEP void unsigned8(struct Fields *p,signed char a,signed char b){p->u8=(unsigned char)a+(unsigned char)b;}
KEEP void signed_plain8(signed char *p,unsigned char a,unsigned char b){*p=(signed char)(a+b);}
KEEP void unsigned_plain8(unsigned char *p,signed char a,signed char b){*p=(unsigned char)a+(unsigned char)b;}
KEEP void signed_xor8(struct Fields *p,signed char a){p->s8=a^1;}
KEEP void signed_constant8(struct Fields *p){p->s8=-1;}
KEEP void unsigned_constant8(struct Fields *p){p->u8=(unsigned char)-1;}
KEEP void signed16(struct Fields *p,unsigned short a,unsigned short b){p->s16=(short)(a+b);}
KEEP void unsigned16(struct Fields *p,short a,short b){p->u16=(unsigned short)a+(unsigned short)b;}
KEEP void signed_plain16(short *p,unsigned short a,unsigned short b){*p=(short)(a+b);}
KEEP void unsigned_plain16(unsigned short *p,short a,short b){*p=(unsigned short)a+(unsigned short)b;}
KEEP void signed_xor16(struct Fields *p,short a){p->s16=a^1;}
KEEP void signed_constant16(struct Fields *p){p->s16=-1;}
KEEP void unsigned_constant16(struct Fields *p){p->u16=(unsigned short)-1;}
KEEP void signed32(struct Fields *p,unsigned int a,unsigned int b){p->s32=(int)(a+b);}
KEEP void unsigned32(struct Fields *p,int a,int b){p->u32=(unsigned int)a+(unsigned int)b;}
KEEP void signed_plain32(int *p,unsigned int a,unsigned int b){*p=(int)(a+b);}
KEEP void unsigned_plain32(unsigned int *p,int a,int b){*p=(unsigned int)a+(unsigned int)b;}
KEEP void signed_xor32(struct Fields *p,int a){p->s32=a^1;}
KEEP void signed_constant32(struct Fields *p){p->s32=-1;}
KEEP void unsigned_constant32(struct Fields *p){p->u32=(unsigned int)-1;}
KEEP void signed64(struct Fields *p,unsigned long a,unsigned long b){p->s64=(long)(a+b);}
KEEP void unsigned64(struct Fields *p,long a,long b){p->u64=(unsigned long)a+(unsigned long)b;}
KEEP void signed_plain64(long *p,unsigned long a,unsigned long b){*p=(long)(a+b);}
KEEP void unsigned_plain64(unsigned long *p,long a,long b){*p=(unsigned long)a+(unsigned long)b;}
KEEP void signed_xor64(struct Fields *p,long a){p->s64=a^1;}
KEEP void signed_constant64(struct Fields *p){p->s64=-1;}
KEEP void unsigned_constant64(struct Fields *p){p->u64=(unsigned long)-1;}
extern struct Fields *counted_destination(struct Fields *p);
extern unsigned long counted_value(unsigned long a,unsigned long b);
KEEP void side_effect(struct Fields *p,unsigned long a,unsigned long b){counted_destination(p)->s64=counted_value(a,b);}
KEEP void array_store(long *p,unsigned long i,unsigned long a){p[i]=a;}
struct Nested { struct Fields values[2]; };
KEEP void nested_store(struct Nested *p,unsigned long a){p->values[1].s32=(int)a;}
KEEP void float_store(double *p,double a){*p=a;}
KEEP void pointer_store(long **p,long *a){*p=a;}
KEEP void bool_store(_Bool *p,_Bool a){*p=a;}
int main(void){return 0;}

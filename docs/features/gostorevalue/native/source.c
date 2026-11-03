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

unsigned int pointer_calls, value_calls;
__attribute__((noinline,noclone)) struct Fields *counted_destination(struct Fields *p){pointer_calls++;return p;}
__attribute__((noinline,noclone)) unsigned long counted_value(unsigned long a,unsigned long b){value_calls++;return a+b;}
#include <stdio.h>
#include <string.h>
int main(void){unsigned long values[]={0UL,1UL,2UL,127UL,128UL,255UL,256UL,32767UL,32768UL,65535UL,65536UL,2147483647UL,2147483648UL,4294967295UL,4294967296UL,9223372036854775807UL,9223372036854775808UL,18446744073709551615UL};for(unsigned i=0;i<sizeof(values)/sizeof(*values);i++){for(unsigned j=0;j<sizeof(values)/sizeof(*values);j++){unsigned long a=values[i],b=values[j];
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed8(&p,(unsigned char)a,(unsigned char)b);printf("signed8 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned8(&p,(signed char)a,(signed char)b);printf("unsigned8 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_plain8(&p.s8,(unsigned char)a,(unsigned char)b);printf("signed_plain8 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned_plain8(&p.u8,(signed char)a,(signed char)b);printf("unsigned_plain8 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_xor8(&p,(signed char)a);printf("signed_xor8 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_constant8(&p);printf("signed_constant8 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned_constant8(&p);printf("unsigned_constant8 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed16(&p,(unsigned short)a,(unsigned short)b);printf("signed16 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned16(&p,(short)a,(short)b);printf("unsigned16 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_plain16(&p.s16,(unsigned short)a,(unsigned short)b);printf("signed_plain16 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned_plain16(&p.u16,(short)a,(short)b);printf("unsigned_plain16 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_xor16(&p,(short)a);printf("signed_xor16 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_constant16(&p);printf("signed_constant16 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned_constant16(&p);printf("unsigned_constant16 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed32(&p,(unsigned int)a,(unsigned int)b);printf("signed32 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned32(&p,(int)a,(int)b);printf("unsigned32 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_plain32(&p.s32,(unsigned int)a,(unsigned int)b);printf("signed_plain32 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned_plain32(&p.u32,(int)a,(int)b);printf("unsigned_plain32 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_xor32(&p,(int)a);printf("signed_xor32 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_constant32(&p);printf("signed_constant32 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned_constant32(&p);printf("unsigned_constant32 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed64(&p,(unsigned long)a,(unsigned long)b);printf("signed64 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned64(&p,(long)a,(long)b);printf("unsigned64 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_plain64(&p.s64,(unsigned long)a,(unsigned long)b);printf("signed_plain64 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned_plain64(&p.u64,(long)a,(long)b);printf("unsigned_plain64 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_xor64(&p,(long)a);printf("signed_xor64 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;signed_constant64(&p);printf("signed_constant64 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;unsigned_constant64(&p);printf("unsigned_constant64 %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{struct Fields p={-3,5,-7,11,-13,17,-19,23};pointer_calls=0;value_calls=0;side_effect(&p,(unsigned long)a,(unsigned long)b);printf("side_effect %lu %lu %lu %lu %lu %lu %lu %lu %u %u\n",(unsigned long)p.s8,(unsigned long)p.u8,(unsigned long)p.s16,(unsigned long)p.u16,(unsigned long)p.s32,(unsigned long)p.u32,(unsigned long)p.s64,(unsigned long)p.u64,pointer_calls,value_calls);}
{long arr[3]={9,10,11};array_store(arr,a%3,b);printf("array_store %lu %lu %lu\n",(unsigned long)arr[0],(unsigned long)arr[1],(unsigned long)arr[2]);}
{double dest,input;memcpy(&input,&a,8);float_store(&dest,input);unsigned long bits;memcpy(&bits,&dest,8);printf("float_store %lu\n",bits);}
{long value=(long)a,*dest;pointer_store(&dest,&value);printf("pointer_store %lu\n",(unsigned long)*dest);}
{_Bool dest;bool_store(&dest,(a&1)!=0);printf("bool_store %u\n",(unsigned int)dest);}
}}return 0;}

#include <stdio.h>
#include <string.h>
#define main fixture_main
#include "fixture.c"
#undef main
void dump(const char*n,void*v,size_t z){printf("%s ",n);for(size_t k=0;k<z;k++)printf("%02x",((unsigned char*)v)[k]);puts("");}
int main(void){unsigned long values[]={0UL,1UL,2UL,127UL,128UL,255UL,256UL,32767UL,32768UL,65535UL,65536UL,2147483647UL,2147483648UL,4294967295UL,4294967296UL,9223372036854775807UL,9223372036854775808UL,18446744073709551615UL};for(unsigned i=0;i<18;i++){unsigned long a=values[i];
{struct {unsigned long before[2];short value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i8_to_i16(&s.value,(signed char)a);dump("i8_to_i16",&s,sizeof s);}
{struct {unsigned long before[2];unsigned short value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i8_to_u16(&s.value,(signed char)a);dump("i8_to_u16",&s,sizeof s);}
{struct {unsigned long before[2];int value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i8_to_i32(&s.value,(signed char)a);dump("i8_to_i32",&s,sizeof s);}
{struct {unsigned long before[2];unsigned int value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i8_to_u32(&s.value,(signed char)a);dump("i8_to_u32",&s,sizeof s);}
{struct {unsigned long before[2];long value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i8_to_i64(&s.value,(signed char)a);dump("i8_to_i64",&s,sizeof s);}
{struct {unsigned long before[2];unsigned long value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i8_to_u64(&s.value,(signed char)a);dump("i8_to_u64",&s,sizeof s);}
{struct {unsigned long before[2];short value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u8_to_i16(&s.value,(unsigned char)a);dump("u8_to_i16",&s,sizeof s);}
{struct {unsigned long before[2];unsigned short value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u8_to_u16(&s.value,(unsigned char)a);dump("u8_to_u16",&s,sizeof s);}
{struct {unsigned long before[2];int value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u8_to_i32(&s.value,(unsigned char)a);dump("u8_to_i32",&s,sizeof s);}
{struct {unsigned long before[2];unsigned int value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u8_to_u32(&s.value,(unsigned char)a);dump("u8_to_u32",&s,sizeof s);}
{struct {unsigned long before[2];long value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u8_to_i64(&s.value,(unsigned char)a);dump("u8_to_i64",&s,sizeof s);}
{struct {unsigned long before[2];unsigned long value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u8_to_u64(&s.value,(unsigned char)a);dump("u8_to_u64",&s,sizeof s);}
{struct {unsigned long before[2];signed char value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i16_to_i8(&s.value,(short)a);dump("i16_to_i8",&s,sizeof s);}
{struct {unsigned long before[2];unsigned char value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i16_to_u8(&s.value,(short)a);dump("i16_to_u8",&s,sizeof s);}
{struct {unsigned long before[2];int value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i16_to_i32(&s.value,(short)a);dump("i16_to_i32",&s,sizeof s);}
{struct {unsigned long before[2];unsigned int value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i16_to_u32(&s.value,(short)a);dump("i16_to_u32",&s,sizeof s);}
{struct {unsigned long before[2];long value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i16_to_i64(&s.value,(short)a);dump("i16_to_i64",&s,sizeof s);}
{struct {unsigned long before[2];unsigned long value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i16_to_u64(&s.value,(short)a);dump("i16_to_u64",&s,sizeof s);}
{struct {unsigned long before[2];signed char value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u16_to_i8(&s.value,(unsigned short)a);dump("u16_to_i8",&s,sizeof s);}
{struct {unsigned long before[2];unsigned char value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u16_to_u8(&s.value,(unsigned short)a);dump("u16_to_u8",&s,sizeof s);}
{struct {unsigned long before[2];int value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u16_to_i32(&s.value,(unsigned short)a);dump("u16_to_i32",&s,sizeof s);}
{struct {unsigned long before[2];unsigned int value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u16_to_u32(&s.value,(unsigned short)a);dump("u16_to_u32",&s,sizeof s);}
{struct {unsigned long before[2];long value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u16_to_i64(&s.value,(unsigned short)a);dump("u16_to_i64",&s,sizeof s);}
{struct {unsigned long before[2];unsigned long value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u16_to_u64(&s.value,(unsigned short)a);dump("u16_to_u64",&s,sizeof s);}
{struct {unsigned long before[2];signed char value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i32_to_i8(&s.value,(int)a);dump("i32_to_i8",&s,sizeof s);}
{struct {unsigned long before[2];unsigned char value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i32_to_u8(&s.value,(int)a);dump("i32_to_u8",&s,sizeof s);}
{struct {unsigned long before[2];short value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i32_to_i16(&s.value,(int)a);dump("i32_to_i16",&s,sizeof s);}
{struct {unsigned long before[2];unsigned short value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i32_to_u16(&s.value,(int)a);dump("i32_to_u16",&s,sizeof s);}
{struct {unsigned long before[2];long value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i32_to_i64(&s.value,(int)a);dump("i32_to_i64",&s,sizeof s);}
{struct {unsigned long before[2];unsigned long value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i32_to_u64(&s.value,(int)a);dump("i32_to_u64",&s,sizeof s);}
{struct {unsigned long before[2];signed char value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u32_to_i8(&s.value,(unsigned int)a);dump("u32_to_i8",&s,sizeof s);}
{struct {unsigned long before[2];unsigned char value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u32_to_u8(&s.value,(unsigned int)a);dump("u32_to_u8",&s,sizeof s);}
{struct {unsigned long before[2];short value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u32_to_i16(&s.value,(unsigned int)a);dump("u32_to_i16",&s,sizeof s);}
{struct {unsigned long before[2];unsigned short value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u32_to_u16(&s.value,(unsigned int)a);dump("u32_to_u16",&s,sizeof s);}
{struct {unsigned long before[2];long value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u32_to_i64(&s.value,(unsigned int)a);dump("u32_to_i64",&s,sizeof s);}
{struct {unsigned long before[2];unsigned long value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u32_to_u64(&s.value,(unsigned int)a);dump("u32_to_u64",&s,sizeof s);}
{struct {unsigned long before[2];signed char value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i64_to_i8(&s.value,(long)a);dump("i64_to_i8",&s,sizeof s);}
{struct {unsigned long before[2];unsigned char value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i64_to_u8(&s.value,(long)a);dump("i64_to_u8",&s,sizeof s);}
{struct {unsigned long before[2];short value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i64_to_i16(&s.value,(long)a);dump("i64_to_i16",&s,sizeof s);}
{struct {unsigned long before[2];unsigned short value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i64_to_u16(&s.value,(long)a);dump("i64_to_u16",&s,sizeof s);}
{struct {unsigned long before[2];int value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i64_to_i32(&s.value,(long)a);dump("i64_to_i32",&s,sizeof s);}
{struct {unsigned long before[2];unsigned int value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);i64_to_u32(&s.value,(long)a);dump("i64_to_u32",&s,sizeof s);}
{struct {unsigned long before[2];signed char value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u64_to_i8(&s.value,(unsigned long)a);dump("u64_to_i8",&s,sizeof s);}
{struct {unsigned long before[2];unsigned char value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u64_to_u8(&s.value,(unsigned long)a);dump("u64_to_u8",&s,sizeof s);}
{struct {unsigned long before[2];short value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u64_to_i16(&s.value,(unsigned long)a);dump("u64_to_i16",&s,sizeof s);}
{struct {unsigned long before[2];unsigned short value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u64_to_u16(&s.value,(unsigned long)a);dump("u64_to_u16",&s,sizeof s);}
{struct {unsigned long before[2];int value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u64_to_i32(&s.value,(unsigned long)a);dump("u64_to_i32",&s,sizeof s);}
{struct {unsigned long before[2];unsigned int value;unsigned long after[2];}s;memset(&s,0xa5,sizeof s);u64_to_u32(&s.value,(unsigned long)a);dump("u64_to_u32",&s,sizeof s);}
}return 0;}

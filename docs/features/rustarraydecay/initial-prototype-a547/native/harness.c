#define _GNU_SOURCE
#include <stdio.h>
#include <inttypes.h>
#include <string.h>
#include <sys/mman.h>
#define OMIT_MAIN
#include "array_decay.c"
extern uint64_t trace;
static void report(const char*name,void*result,void*base,uint64_t count,uint64_t before,uint64_t after){printf("%s %016" PRIx64 " %" PRIu64 " %016" PRIx64 " %016" PRIx64 "\n",name,(uint64_t)((uintptr_t)result-(uintptr_t)base),count,before,after);}
int main(void){for(int mode=0;mode<2;mode++)for(int position=0;position<2;position++){void*memory=mmap(0,4096,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_ANONYMOUS,-1,0);if(memory==MAP_FAILED)return 2;memset(memory,0xa5,4096);struct DecayFields*p=(struct DecayFields*)((char*)memory+128*position);void*array=(char*)memory+2048+128*position;if(mode&&mprotect(memory,4096,PROT_NONE))return 3;printf("mode %d base %d\n",mode,position);
{trace=0;void*result=field_i8(p);report("field_i8",result,p,trace,0,0);}
{trace=0;void*result=whole_i8((int8_t(*)[9])array);report("whole_i8",result,array,trace,0,0);}
{trace=0;void*result=end_i8((int8_t(*)[9])array);report("end_i8",result,array,trace,0,0);}
{trace=0;void*result=erased_i8(p);report("erased_i8",result,p,trace,0,0);}
{trace=0;void*result=retyped_i8(p);report("retyped_i8",result,p,trace,0,0);}
{trace=0;void*result=array_i8(p);report("array_i8",result,p,trace,0,0);}
{struct {uint64_t before;int8_t*value;uint64_t after;}slot={UINT64_C(0x123456789abcdef0),0,UINT64_C(0xfedcba9876543210)};stored_i8(&slot.value,p);report("stored_i8",slot.value,p,0,slot.before,slot.after);}
{trace=0;void*result=passed_i8(p);report("passed_i8",result,p,trace,0,0);}
{trace=0;void*result=field_u8(p);report("field_u8",result,p,trace,0,0);}
{trace=0;void*result=whole_u8((uint8_t(*)[9])array);report("whole_u8",result,array,trace,0,0);}
{trace=0;void*result=end_u8((uint8_t(*)[9])array);report("end_u8",result,array,trace,0,0);}
{trace=0;void*result=erased_u8(p);report("erased_u8",result,p,trace,0,0);}
{trace=0;void*result=retyped_u8(p);report("retyped_u8",result,p,trace,0,0);}
{trace=0;void*result=array_u8(p);report("array_u8",result,p,trace,0,0);}
{struct {uint64_t before;uint8_t*value;uint64_t after;}slot={UINT64_C(0x123456789abcdef0),0,UINT64_C(0xfedcba9876543210)};stored_u8(&slot.value,p);report("stored_u8",slot.value,p,0,slot.before,slot.after);}
{trace=0;void*result=passed_u8(p);report("passed_u8",result,p,trace,0,0);}
{trace=0;void*result=field_i16(p);report("field_i16",result,p,trace,0,0);}
{trace=0;void*result=whole_i16((int16_t(*)[9])array);report("whole_i16",result,array,trace,0,0);}
{trace=0;void*result=end_i16((int16_t(*)[9])array);report("end_i16",result,array,trace,0,0);}
{trace=0;void*result=erased_i16(p);report("erased_i16",result,p,trace,0,0);}
{trace=0;void*result=retyped_i16(p);report("retyped_i16",result,p,trace,0,0);}
{trace=0;void*result=array_i16(p);report("array_i16",result,p,trace,0,0);}
{struct {uint64_t before;int16_t*value;uint64_t after;}slot={UINT64_C(0x123456789abcdef0),0,UINT64_C(0xfedcba9876543210)};stored_i16(&slot.value,p);report("stored_i16",slot.value,p,0,slot.before,slot.after);}
{trace=0;void*result=passed_i16(p);report("passed_i16",result,p,trace,0,0);}
{trace=0;void*result=field_u16(p);report("field_u16",result,p,trace,0,0);}
{trace=0;void*result=whole_u16((uint16_t(*)[9])array);report("whole_u16",result,array,trace,0,0);}
{trace=0;void*result=end_u16((uint16_t(*)[9])array);report("end_u16",result,array,trace,0,0);}
{trace=0;void*result=erased_u16(p);report("erased_u16",result,p,trace,0,0);}
{trace=0;void*result=retyped_u16(p);report("retyped_u16",result,p,trace,0,0);}
{trace=0;void*result=array_u16(p);report("array_u16",result,p,trace,0,0);}
{struct {uint64_t before;uint16_t*value;uint64_t after;}slot={UINT64_C(0x123456789abcdef0),0,UINT64_C(0xfedcba9876543210)};stored_u16(&slot.value,p);report("stored_u16",slot.value,p,0,slot.before,slot.after);}
{trace=0;void*result=passed_u16(p);report("passed_u16",result,p,trace,0,0);}
{trace=0;void*result=field_i32(p);report("field_i32",result,p,trace,0,0);}
{trace=0;void*result=whole_i32((int32_t(*)[9])array);report("whole_i32",result,array,trace,0,0);}
{trace=0;void*result=end_i32((int32_t(*)[9])array);report("end_i32",result,array,trace,0,0);}
{trace=0;void*result=erased_i32(p);report("erased_i32",result,p,trace,0,0);}
{trace=0;void*result=retyped_i32(p);report("retyped_i32",result,p,trace,0,0);}
{trace=0;void*result=array_i32(p);report("array_i32",result,p,trace,0,0);}
{struct {uint64_t before;int32_t*value;uint64_t after;}slot={UINT64_C(0x123456789abcdef0),0,UINT64_C(0xfedcba9876543210)};stored_i32(&slot.value,p);report("stored_i32",slot.value,p,0,slot.before,slot.after);}
{trace=0;void*result=passed_i32(p);report("passed_i32",result,p,trace,0,0);}
{trace=0;void*result=field_u32(p);report("field_u32",result,p,trace,0,0);}
{trace=0;void*result=whole_u32((uint32_t(*)[9])array);report("whole_u32",result,array,trace,0,0);}
{trace=0;void*result=end_u32((uint32_t(*)[9])array);report("end_u32",result,array,trace,0,0);}
{trace=0;void*result=erased_u32(p);report("erased_u32",result,p,trace,0,0);}
{trace=0;void*result=retyped_u32(p);report("retyped_u32",result,p,trace,0,0);}
{trace=0;void*result=array_u32(p);report("array_u32",result,p,trace,0,0);}
{struct {uint64_t before;uint32_t*value;uint64_t after;}slot={UINT64_C(0x123456789abcdef0),0,UINT64_C(0xfedcba9876543210)};stored_u32(&slot.value,p);report("stored_u32",slot.value,p,0,slot.before,slot.after);}
{trace=0;void*result=passed_u32(p);report("passed_u32",result,p,trace,0,0);}
{trace=0;void*result=field_i64(p);report("field_i64",result,p,trace,0,0);}
{trace=0;void*result=whole_i64((int64_t(*)[9])array);report("whole_i64",result,array,trace,0,0);}
{trace=0;void*result=end_i64((int64_t(*)[9])array);report("end_i64",result,array,trace,0,0);}
{trace=0;void*result=erased_i64(p);report("erased_i64",result,p,trace,0,0);}
{trace=0;void*result=retyped_i64(p);report("retyped_i64",result,p,trace,0,0);}
{trace=0;void*result=array_i64(p);report("array_i64",result,p,trace,0,0);}
{struct {uint64_t before;int64_t*value;uint64_t after;}slot={UINT64_C(0x123456789abcdef0),0,UINT64_C(0xfedcba9876543210)};stored_i64(&slot.value,p);report("stored_i64",slot.value,p,0,slot.before,slot.after);}
{trace=0;void*result=passed_i64(p);report("passed_i64",result,p,trace,0,0);}
{trace=0;void*result=field_u64(p);report("field_u64",result,p,trace,0,0);}
{trace=0;void*result=whole_u64((uint64_t(*)[9])array);report("whole_u64",result,array,trace,0,0);}
{trace=0;void*result=end_u64((uint64_t(*)[9])array);report("end_u64",result,array,trace,0,0);}
{trace=0;void*result=erased_u64(p);report("erased_u64",result,p,trace,0,0);}
{trace=0;void*result=retyped_u64(p);report("retyped_u64",result,p,trace,0,0);}
{trace=0;void*result=array_u64(p);report("array_u64",result,p,trace,0,0);}
{struct {uint64_t before;uint64_t*value;uint64_t after;}slot={UINT64_C(0x123456789abcdef0),0,UINT64_C(0xfedcba9876543210)};stored_u64(&slot.value,p);report("stored_u64",slot.value,p,0,slot.before,slot.after);}
{trace=0;void*result=passed_u64(p);report("passed_u64",result,p,trace,0,0);}
{trace=0;void*result=field_float32(p);report("field_float32",result,p,trace,0,0);}
{trace=0;void*result=whole_float32((float(*)[9])array);report("whole_float32",result,array,trace,0,0);}
{trace=0;void*result=end_float32((float(*)[9])array);report("end_float32",result,array,trace,0,0);}
{trace=0;void*result=erased_float32(p);report("erased_float32",result,p,trace,0,0);}
{trace=0;void*result=retyped_float32(p);report("retyped_float32",result,p,trace,0,0);}
{trace=0;void*result=array_float32(p);report("array_float32",result,p,trace,0,0);}
{struct {uint64_t before;float*value;uint64_t after;}slot={UINT64_C(0x123456789abcdef0),0,UINT64_C(0xfedcba9876543210)};stored_float32(&slot.value,p);report("stored_float32",slot.value,p,0,slot.before,slot.after);}
{trace=0;void*result=passed_float32(p);report("passed_float32",result,p,trace,0,0);}
{trace=0;void*result=field_float64(p);report("field_float64",result,p,trace,0,0);}
{trace=0;void*result=whole_float64((double(*)[9])array);report("whole_float64",result,array,trace,0,0);}
{trace=0;void*result=end_float64((double(*)[9])array);report("end_float64",result,array,trace,0,0);}
{trace=0;void*result=erased_float64(p);report("erased_float64",result,p,trace,0,0);}
{trace=0;void*result=retyped_float64(p);report("retyped_float64",result,p,trace,0,0);}
{trace=0;void*result=array_float64(p);report("array_float64",result,p,trace,0,0);}
{struct {uint64_t before;double*value;uint64_t after;}slot={UINT64_C(0x123456789abcdef0),0,UINT64_C(0xfedcba9876543210)};stored_float64(&slot.value,p);report("stored_float64",slot.value,p,0,slot.before,slot.after);}
{trace=0;void*result=passed_float64(p);report("passed_float64",result,p,trace,0,0);}
{trace=0;void*result=field_pointers(p);report("field_pointers",result,p,trace,0,0);}
{trace=0;void*result=whole_pointers((uint8_t*(*)[9])array);report("whole_pointers",result,array,trace,0,0);}
{trace=0;void*result=end_pointers((uint8_t*(*)[9])array);report("end_pointers",result,array,trace,0,0);}
{trace=0;void*result=erased_pointers(p);report("erased_pointers",result,p,trace,0,0);}
{trace=0;void*result=retyped_pointers(p);report("retyped_pointers",result,p,trace,0,0);}
{trace=0;void*result=array_pointers(p);report("array_pointers",result,p,trace,0,0);}
{struct {uint64_t before;uint8_t**value;uint64_t after;}slot={UINT64_C(0x123456789abcdef0),0,UINT64_C(0xfedcba9876543210)};stored_pointers(&slot.value,p);report("stored_pointers",slot.value,p,0,slot.before,slot.after);}
{trace=0;void*result=passed_pointers(p);report("passed_pointers",result,p,trace,0,0);}
{trace=0;void*result=field_flags(p);report("field_flags",result,p,trace,0,0);}
{trace=0;void*result=whole_flags((_Bool(*)[9])array);report("whole_flags",result,array,trace,0,0);}
{trace=0;void*result=end_flags((_Bool(*)[9])array);report("end_flags",result,array,trace,0,0);}
{trace=0;void*result=erased_flags(p);report("erased_flags",result,p,trace,0,0);}
{trace=0;void*result=retyped_flags(p);report("retyped_flags",result,p,trace,0,0);}
{trace=0;void*result=array_flags(p);report("array_flags",result,p,trace,0,0);}
{struct {uint64_t before;_Bool*value;uint64_t after;}slot={UINT64_C(0x123456789abcdef0),0,UINT64_C(0xfedcba9876543210)};stored_flags(&slot.value,p);report("stored_flags",slot.value,p,0,slot.before,slot.after);}
{trace=0;void*result=passed_flags(p);report("passed_flags",result,p,trace,0,0);}
{trace=0;void*result=passed_counted(p);report("passed_counted",result,p,trace,0,0);}
if(munmap(memory,4096))return 4;}return 0;}

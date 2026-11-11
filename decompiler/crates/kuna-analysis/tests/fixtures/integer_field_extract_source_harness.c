#include <stdio.h>
#include <inttypes.h>
#include <assert.h>
int main(void) { int64_t values[]={INT64_MIN,-9223372036854775807LL,-4294967297LL,-4294967296LL,-65537LL,-65536LL,-32769LL,-32768LL,-257LL,-256LL,-129LL,-128LL,-1LL,0LL,1LL,127LL,128LL,255LL,256LL,32767LL,32768LL,65535LL,65536LL,4294967295LL,4294967296LL,4294967297LL,9223372036854775806LL,9223372036854775807LL}; for (unsigned i=0;i<sizeof(values)/sizeof(*values);++i) { for (unsigned j=0;j<sizeof(values)/sizeof(*values);++j) { int64_t a=values[i],b=values[j];
calls=0; printf("%016" PRIx64 "\n",(uint64_t)signed_int8(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)signed_int8_wide(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)signed_uint8(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)signed_uint8_wide(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)signed_int16(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)signed_int16_wide(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)signed_uint16(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)signed_uint16_wide(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)signed_int32(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)signed_int32_wide(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)signed_uint32(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)signed_uint32_wide(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)unsigned_int8(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)unsigned_uint8(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)unsigned_int16(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)unsigned_uint16(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)unsigned_int32(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)unsigned_uint32(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)first_narrow(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)shift8(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)shift32(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)signed_high(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)signed_as_unsigned(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)unsigned_as_signed(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)exact_signed(a,b)); assert(calls==1);
calls=0; printf("%016" PRIx64 "\n",(uint64_t)exact_unsigned(a,b)); assert(calls==1);
}}}

#include <dlfcn.h>
#include <float.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include "sample.so.h"
_Static_assert(LDBL_MANT_DIG == 64, "x87 extended input is required");
static unsigned short status(void) { unsigned short v; __asm__ volatile("fnstsw %0":"=am"(v)); return v&0x3f; }
static unsigned short control(void) { unsigned short v; __asm__ volatile("fnstcw %0":"=m"(v)); return v; }
static void clear(void) { __asm__ volatile("fnclex":::"memory"); }
int main(int argc,char **argv) {
    if(argc!=2)return 90;
    void *image=dlopen(argv[1],RTLD_NOW|RTLD_LOCAL); if(!image){puts(dlerror());return 91;}
    short (*ref16)(long double*,unsigned short)=dlsym(image,"sample_fistp16");
    int (*ref32)(long double*,unsigned short)=dlsym(image,"sample_fistp32");
    long (*ref64)(long double*,unsigned short)=dlsym(image,"sample_fistp64");
    short (*keep16)(long double*,unsigned short,long double*)=dlsym(image,"sample_fist16");
    int (*keep32)(long double*,unsigned short,long double*)=dlsym(image,"sample_fist32");
    if(!ref16||!ref32||!ref64||!keep16||!keep32)return 92;
    long double values[1100]={0.0L,-0.0L,0.25L,-0.25L,0.5L,-0.5L,1.5L,-1.5L,2.5L,-2.5L,
        32767.0L,32767.5L,32768.0L,-32768.0L,-32768.5L,-32769.0L,
        2147483647.0L,2147483647.5L,2147483648.0L,-2147483648.0L,-2147483648.5L,-2147483649.0L,
        9223372036854775807.0L,9223372036854775807.5L,9223372036854775808.0L,
        -9223372036854775808.0L,-9223372036854775809.0L,0x1.000000000000001p0L,
        -0x1.000000000000001p0L,0x1p-16445L,INFINITY,-INFINITY,NAN};
    unsigned count=33;uint64_t random=0x625e9ac13710d4b7ULL;
    for(unsigned i=0;i<1024;i++){random=random*6364136223846793005ULL+1;values[count++]=(long double)(int64_t)(random>>30)/8.0L;}
    unsigned short saved=control(),baseline=0x37f;
    __asm__ volatile("fldcw %0"::"m"(baseline):"memory");
    unsigned comparisons=0;
    for(unsigned mode=0;mode<4;mode++)for(unsigned i=0;i<count;i++)for(unsigned which=0;which<5;which++){
        unsigned short cw=(unsigned short)(0x37f|(mode<<10));
        long double expected_payload,actual_payload;memset(&expected_payload,0xa5,sizeof(expected_payload));memset(&actual_payload,0xa5,sizeof(actual_payload));
        clear();long expected;
        switch(which){case 0:expected=ref16(&values[i],cw);break;case 1:expected=ref32(&values[i],cw);break;case 2:expected=ref64(&values[i],cw);break;case 3:expected=keep16(&values[i],cw,&expected_payload);break;default:expected=keep32(&values[i],cw,&expected_payload);}
        unsigned short expected_status=status();if(control()!=baseline)return 93;
        clear();long actual;
        switch(which){case 0:actual=sample_fistp16(&values[i],cw);break;case 1:actual=sample_fistp32(&values[i],cw);break;case 2:actual=sample_fistp64(&values[i],cw);break;case 3:actual=sample_fist16(&values[i],cw,&actual_payload);break;default:actual=sample_fist32(&values[i],cw,&actual_payload);}
        unsigned short actual_status=status();if(control()!=baseline)return 94;
        if(expected!=actual||expected_status!=actual_status||(which>=3&&memcmp(&expected_payload,&actual_payload,10))){fprintf(stderr,"mode=%u input=%u form=%u result=%ld/%ld status=%x/%x\n",mode,i,which,expected,actual,expected_status,actual_status);return 1;}
        comparisons++;
    }
    __asm__ volatile("fldcw %0"::"m"(saved):"memory");
    printf("%u native comparisons passed\n",comparisons);return 0;
}

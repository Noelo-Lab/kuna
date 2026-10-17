#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <sys/mman.h>
#define cursor ((volatile unsigned char *)0x601000)
#define SUB161(x,n) ((uint8_t)((unsigned __int128)(x) >> (8*(n))))
#define CONCAT11(a,b) (((uint16_t)(uint8_t)(a)<<8)|(uint8_t)(b))
#define CONCAT12(a,b) (((uint32_t)(uint8_t)(a)<<16)|(uint16_t)(b))
#define CONCAT13(a,b) (((uint32_t)(uint8_t)(a)<<24)|((uint32_t)(b)&0xffffff))
GENERATED_FUNCTION

int main(void) {
    void *data=mmap((void*)0x601000,4096,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_ANONYMOUS|MAP_FIXED_NOREPLACE,-1,0);
    if(data==MAP_FAILED){perror("mmap data");return 2;}
    unsigned char bytes[]={ORIGINAL_BYTES};
    void *code=mmap(0,4096,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_ANONYMOUS,-1,0);
    if(code==MAP_FAILED)return 3;
    memcpy(code,bytes,sizeof(bytes));
    if(mprotect(code,4096,PROT_READ|PROT_EXEC))return 4;
    for(unsigned n=0;n<256;n++) {
        for(unsigned i=0;i<32;i++)cursor[i]=(unsigned char)(n+17*i);
        unsigned expected=((unsigned (*)(void))code)();
        unsigned got=sub_400000();
        if(expected!=got){fprintf(stderr,"%u: %u != %u\n",n,expected,got);return 1;}
    }
    return 0;
}

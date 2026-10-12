#include <stdint.h>
#include <stdio.h>
#include <string.h>
extern unsigned char bytes[32], whole[8];
void write1(unsigned char); void write2(unsigned short); void write4(unsigned int);
void write8(unsigned long long); void whole_write(unsigned long long);
unsigned char read1(void); unsigned short read2(void); unsigned int read4(void);
unsigned long long read8(void), whole_read(void);
int read_signed4(void); void overlap_copy(unsigned short);
unsigned int read_const4(void);
void whole_write1(unsigned char); unsigned char whole_read1(void);
void whole_write2(unsigned short); unsigned short whole_read2(void);
void whole_write4(unsigned int); unsigned int whole_read4(void);
static uint64_t load(const unsigned char *p, unsigned size) {
    uint64_t value = 0;
    for (unsigned i = 0; i < size; ++i) value |= (uint64_t)p[i] << (8*i);
    return value;
}
static void store(unsigned char *p, unsigned size, uint64_t value) {
    for (unsigned i = 0; i < size; ++i) p[i] = value >> (8*i);
}
int main(void) {
    _Static_assert(sizeof(unsigned int) == 4 && sizeof(unsigned long long) == 8, "fixture ABI");
    if (read_const4() != UINT32_C(0x55443322)) return 7;
    const uint64_t cases[] = {0, 1, UINT64_MAX, UINT64_C(0x8077665544332211)};
    for (unsigned c = 0; c < sizeof(cases)/sizeof(cases[0]); ++c) {
        unsigned char expected[32];
        for (unsigned i = 0; i < 32; ++i) expected[i] = bytes[i] = 0x80 + i;
        uint64_t value = cases[c];
        write1(value); store(expected+3,1,value);
        if (memcmp(bytes,expected,32) || read1()!=load(expected+3,1)) return 1;
        write2(value); store(expected+1,2,value);
        if (memcmp(bytes,expected,32) || read2()!=load(expected+1,2)) return 2;
        write4(value); store(expected+3,4,value);
        int32_t signed_value; memcpy(&signed_value,expected+3,4);
        if (memcmp(bytes,expected,32) || read4()!=load(expected+3,4) || read_signed4()!=signed_value) return 3;
        write8(value); store(expected+7,8,value);
        if (memcmp(bytes,expected,32) || read8()!=load(expected+7,8)) return 4;
        whole_write(value);
        if (whole_read()!=value || load(whole,8)!=value) return 5;
        whole_write1(value); if (whole_read1() != (uint8_t)value) return 8;
        whole_write2(value); if (whole_read2() != (uint16_t)value) return 9;
        whole_write4(value); if (whole_read4() != (uint32_t)value) return 10;
        uint64_t snapshot = load(expected,8);
        store(expected+4,8,snapshot); store(expected+6,2,value);
        overlap_copy(value);
        if (memcmp(bytes,expected,32)) return 6;
    }
    puts("bytes, widths, overlap and signed reads: OK");
    return 0;
}

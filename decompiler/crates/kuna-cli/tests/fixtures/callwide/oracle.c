typedef unsigned int u32;
typedef unsigned long long u64;

#ifdef CALLWIDE_EMITTED_HEADER
#include CALLWIDE_EMITTED_HEADER
#endif
#if !defined(CALLWIDE_EMITTED_HEADER) || defined(CALLWIDE_NATIVE_PRODUCE_SUM)
extern u64 produce_sum(u32 left_high, u32 left_low, u32 right_high, u32 right_low);
#endif
#if !defined(CALLWIDE_EMITTED_HEADER) || defined(CALLWIDE_NATIVE_CALLER64)
extern u64 caller64(u64 left, u64 right);
#endif
#if !defined(CALLWIDE_EMITTED_HEADER) || defined(CALLWIDE_NATIVE_CALLER_HIGH32)
extern u32 caller_high32(u64 left, u64 right);
#endif
#if !defined(CALLWIDE_EMITTED_HEADER) || defined(CALLWIDE_NATIVE_CALLER_HIGH16)
extern unsigned short caller_high16(u64 left, u64 right);
#endif
#if !defined(CALLWIDE_EMITTED_HEADER) || defined(CALLWIDE_NATIVE_CALLER_HIGH8)
extern unsigned char caller_high8(u64 left, u64 right);
#endif
#if !defined(CALLWIDE_EMITTED_HEADER) || defined(CALLWIDE_NATIVE_CALLER_BOOL)
extern _Bool caller_bool(u64 left, u64 right);
#endif
#if !defined(CALLWIDE_EMITTED_HEADER) || defined(CALLWIDE_NATIVE_CALLER_IGNORED)
extern u32 caller_ignored(u64 left, u64 right);
#endif
#if CALLWIDE_DWARF_CALLER_CONTRACTS
#if !defined(CALLWIDE_EMITTED_HEADER) || defined(CALLWIDE_NATIVE_CONTRACT_WIDE)
extern u64 contract_wide(u64 left, u64 right);
#endif
#if !defined(CALLWIDE_EMITTED_HEADER) || defined(CALLWIDE_NATIVE_CONTRACT_NARROW)
extern u32 contract_narrow(u64 left, u64 right);
#endif
#endif

static const u64 vectors[][2] __attribute__((section(".rodata.harness"))) = {
    { 0x0000000100000002ULL, 0x0000000300000004ULL },
    { 0x11223344ffffffffULL, 0x0102030400000001ULL },
    { 0x00000000ffffffffULL, 0x0000000000000001ULL },
    { 0xffffffffffffffffULL, 0x0000000000000001ULL },
    { 0x0000010000000001ULL, 0x0000000000000002ULL },
    { 0x00ff0100aabbccddULL, 0x0000000011223344ULL },
    { 0x7fffffff00000000ULL, 0x0000000100000000ULL },
    { 0x1234567800000000ULL, 0x1111111100000000ULL },
};

__attribute__((noinline, section(".text.harness"))) static void poison_stack(u32 seed)
{
    volatile u32 scratch[128];
    u32 i;
    for (i = 0; i < 128; ++i)
        scratch[i] = 0xa5c30000U + seed + i;
    __asm__ volatile("" : : "r"(scratch) : "memory");
}

__attribute__((section(".text.harness"))) int callwide_run(void)
{
    unsigned int i;
    for (i = 0; i < sizeof(vectors) / sizeof(vectors[0]); ++i) {
        u64 left = vectors[i][0];
        u64 right = vectors[i][1];
        u64 sum = left + right;
        u32 high = (u32)(sum >> 32);
        poison_stack(i);
        if (produce_sum((u32)(left >> 32), (u32)left, (u32)(right >> 32), (u32)right) != sum)
            return 9;
#ifdef CALLWIDE_IDENTITY_SINK
        if (caller64(left, right) != sum)
#else
        if (caller64(left, right) != (sum ^ 0x0000001100000022ULL))
#endif
            return 1;
        if (caller_high32(left, right) != high)
            return 2;
        if (caller_high16(left, right) != (unsigned short)high)
            return 3;
        if (caller_high8(left, right) != (unsigned char)high)
            return 4;
        if (caller_bool(left, right) != (high != 0))
            return 5;
        if (caller_ignored(left, right) != (u32)(left >> 32))
            return 6;
#if CALLWIDE_DWARF_CALLER_CONTRACTS
#ifdef CALLWIDE_IDENTITY_SINK
        if (contract_wide(left, right) != sum)
#else
        if (contract_wide(left, right) != (sum ^ 0x0000001100000022ULL))
#endif
            return 7;
        poison_stack(i);
        if (contract_narrow(left, right) != (high ^ (unsigned short)high
                ^ (unsigned char)high ^ (high != 0) ^ (u32)(left >> 32)))
            return 8;
#endif
    }
    return 0;
}

#ifdef CALLWIDE_HOST_REPLAY
int main(void)
{
    return callwide_run();
}
#endif

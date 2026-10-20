typedef unsigned int u32;
typedef unsigned long long u64;

#define SINK(name) __attribute__((noinline, used, visibility("hidden"), section(".text.callwide_" #name)))

SINK(consume64) u64 consume64(u64 value)
{
#ifdef CALLWIDE_IDENTITY_SINK
    return value;
#else
    return value ^ 0x0000001100000022ULL;
#endif
}

SINK(consume_two32) u32 consume_two32(u32 high, u32 low)
{
    return high ^ low;
}

SINK(consume_slot) u64 consume_slot(u32 tag, u64 value)
{
    return value ^ tag;
}

#if CALLWIDE_DWARF_CALLER_CONTRACTS
extern u64 caller64(u64 left, u64 right);
extern u32 caller_high32(u64 left, u64 right);
extern unsigned short caller_high16(u64 left, u64 right);
extern unsigned char caller_high8(u64 left, u64 right);
extern _Bool caller_bool(u64 left, u64 right);
extern u32 caller_ignored(u64 left, u64 right);

SINK(contract_wide) u64 contract_wide(u64 left, u64 right)
{
    return caller64(left, right);
}

SINK(contract_narrow) u32 contract_narrow(u64 left, u64 right)
{
    return caller_high32(left, right) ^ caller_high16(left, right)
        ^ caller_high8(left, right) ^ caller_bool(left, right)
        ^ caller_ignored(left, right);
}

#ifdef CALLWIDE_MIXED
extern u64 caller_rejected(u64 left, u64 right);

__attribute__((noinline, used, visibility("hidden"), section(".text.callwide.mixed")))
u64 contract_rejected(u64 left, u64 right)
{
    return caller_rejected(left, right);
}
#endif
#endif

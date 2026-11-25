typedef long long i64;
typedef unsigned long long u64;
i64 shift31(i64 value, i64 *echo) {
    *echo = value;
    return (i64)((u64)value << 31);
}
i64 shift63(i64 value) { return (i64)((u64)value << 63); }
i64 widened_shift31(int value) { return (i64)((u64)(i64)value << 31); }
i64 word_wrap(int value) { return (int)((unsigned int)value + 1u); }
#ifdef TEST_CALL
extern i64 source_value(void);
i64 shifted_call(void) { return (i64)((u64)source_value() << 31); }
#endif

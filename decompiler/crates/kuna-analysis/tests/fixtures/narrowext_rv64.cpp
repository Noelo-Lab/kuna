/* narrowext fixture: RV64 narrow values whose type states their sign.
 *
 * clang++ --target=riscv64-linux-gnu -march=rv64gc -mno-relax -O2 -g \
 *         -c narrowext_rv64.cpp -o narrowext_rv64.o
 * clang++ --target=riscv64-linux-gnu -march=rv64gc -mno-relax -O2 \
 *         -c narrowext_rv64.cpp -o narrowext_rv64_nodebug.o
 *
 * The RISC-V calling convention widens a value narrower than 32 bits by the
 * sign of its type.  char16_t is unsigned (DWARF DW_ATE_UTF, mangled `Ds`), E2
 * is an anonymous unsigned 16-bit enum and neg1 a signed 8-bit one, whose sign
 * clang states only through DW_AT_type.  The callees are static and the
 * object is built without relaxation, so its calls need no relocation. */
#define NI __attribute__((noinline))
typedef enum : unsigned short { EA = 0, EB = 40000, EC = 50000 } E2;
enum neg1 : signed char { NEGA = -100, NEGB = 100 };
NI static char16_t r16(int k) { return (char16_t)(k * 300 + 5); }
NI static E2 re2(int k) { return (E2)(k * 1000); }
NI static neg1 rn(int k) { return (neg1)(k - 100); }
NI long c16(int k) { return r16(k) + 1L; }
NI long ce2(int k) { return re2(k) + 1L; }
NI long cn(int k) { return rn(k) / 3L; }
NI long widen16(char16_t c) { return c; }
NI bool hi_sur(char16_t c) { return c >= 0xD800 && c <= 0xDBFF; }
NI long pe2(E2 x) { return x + 1L; }
NI long pn(neg1 x) { return x / 3L; }

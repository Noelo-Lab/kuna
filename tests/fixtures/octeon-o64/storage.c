/* GCC 7.3, big-endian MIPS GNU binutils 2.30.
   ${CC} -EB -O2 -fno-pic -mno-abicalls -march=octeon -mabi=o64 -c storage.c -o storage.o
   ${OBJDUMP} -dr storage.o
   SHA-256 of GCC 7.3: 5d26f5ed013593e77d0237dac6f3179f68cdbb681f8c20d7948444b7ff3e0b49
   SHA-256 of GNU as 2.30: 96b3d7b45a26a3b53aba77e56a372d30c8fa92f06092a2ca78a39d1e97ad465a */
typedef struct { int v; } Word;
typedef struct { unsigned long long first; int last; } Pair;
typedef union { unsigned int u; float f; } Value;
typedef union { unsigned long long bits; double d; } Bits;

unsigned long long dbits(double x) { return (Bits){.d = x}.bits; }
double frombits(unsigned long long x) { return (Bits){.bits = x}.d; }
unsigned int fbits(float x) { return (Value){.f = x}.u; }
float fromword(unsigned x) { return (Value){.u = x}.f; }
float fifthfloat(int a, int b, int c, int d, float x) { return x; }

int field(Word s) { return s.v; }
int fifthfield(int a, int b, int c, int d, Word s) { return s.v; }
Word build(int x) { return (Word){x}; }
Value makeunion(unsigned x) { return (Value){.u = x}; }
int tail(int a, int b, int c, Pair p) { return p.last; }

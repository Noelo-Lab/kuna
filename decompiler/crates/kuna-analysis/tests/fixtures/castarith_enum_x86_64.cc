// castarith C++ enum fixture: `enum class` elements with an 8- and a 16-bit
// underlying type, read through a pointer plus whole elements.  Built with
// g++ -g -O2.  kuna prints each as a plain `enum`, which C sizes as an `int`,
// so the subscript form would scale by 4 and the integer form must stay.
#include <cstdio>
#include <cstdint>
enum class Kind : uint8_t { A = 1, B = 7 };
enum class Op : uint16_t { X = 1, Y = 9 };
__attribute__((noipa)) void use_kind(Kind k) { std::printf("%d\n", (int)k); }
__attribute__((noipa)) void use_op(Op k) { std::printf("%d\n", (int)k); }
extern "C" __attribute__((noipa)) void rd_enum_class(void *p)
{
    use_kind(*(Kind *)((char *)p + 5));
    use_op(*(Op *)((char *)p + 6));
}
int main()
{
    static unsigned char b[64];
    for (int i = 0; i < 64; i++)
        b[i] = (unsigned char)(i * 7 + 1);
    rd_enum_class(b);
}

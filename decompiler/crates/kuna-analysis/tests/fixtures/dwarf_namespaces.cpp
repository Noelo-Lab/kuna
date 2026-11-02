#include <cstdint>
#include <cstdio>
#include <cstdlib>

namespace left {
__attribute__((noinline)) uint32_t target(uint32_t x) { return x + 10; }
__attribute__((noinline)) uint32_t same(uint32_t x) { return target(x); }
namespace deep {
__attribute__((noinline)) uint32_t target(uint32_t x) { return x + 20; }
}
}
namespace right {
__attribute__((noinline)) uint32_t target(uint32_t x) { return x + 30; }
__attribute__((noinline)) uint32_t cross(uint32_t x) { return left::target(x); }
}
__attribute__((noinline)) uint32_t target(uint32_t x) { return x + 40; }
extern "C" __attribute__((noinline)) uint32_t direct_left(uint32_t x) { return left::target(x); }
extern "C" __attribute__((noinline)) uint32_t direct_nested(uint32_t x) { return left::deep::target(x); }
extern "C" __attribute__((noinline)) uint32_t direct_right(uint32_t x) { return right::target(x); }
extern "C" __attribute__((noinline)) uint32_t direct_global(uint32_t x) { return target(x); }
extern "C" __attribute__((noinline)) uint32_t direct_shadow(uint32_t left) { return ::left::target(left); }

int main(int argc, char **argv) {
    uint32_t x = argc > 1 ? static_cast<uint32_t>(std::strtoul(argv[1], nullptr, 0)) : 0;
    std::printf("%u %u %u %u %u %u %u\n", direct_left(x), direct_nested(x), direct_right(x),
                direct_global(x), direct_shadow(x), left::same(x), right::cross(x));
}

#include <cstdint>
#include <cstdio>
#include <cstdlib>

using Callback = unsigned (*)(unsigned);
namespace left {
unsigned data = 77;
__attribute__((noinline)) unsigned target(unsigned x) { return x + 10; }
__attribute__((noinline)) uintptr_t same_address() { return reinterpret_cast<uintptr_t>(&target); }
namespace deep {
__attribute__((noinline)) unsigned target(unsigned x) { return x + 20; }
}
}
namespace right {
__attribute__((noinline)) unsigned target(unsigned x) { return x + 30; }
__attribute__((noinline)) uintptr_t cross_address() { return reinterpret_cast<uintptr_t>(&left::target); }
}
__attribute__((noinline)) unsigned target(unsigned x) { return x + 40; }
extern "C" __attribute__((noinline)) uintptr_t foreign_address() { return reinterpret_cast<uintptr_t>(&left::target); }
extern "C" __attribute__((noinline)) uintptr_t nested_address() { return reinterpret_cast<uintptr_t>(&left::deep::target); }
extern "C" __attribute__((noinline)) uintptr_t right_address() { return reinterpret_cast<uintptr_t>(&right::target); }
extern "C" __attribute__((noinline)) uintptr_t global_shadow(uintptr_t target) { return reinterpret_cast<uintptr_t>(&::target); }
extern "C" __attribute__((noinline)) uintptr_t scope_shadow(uintptr_t left) { return reinterpret_cast<uintptr_t>(&::left::target); }
extern "C" __attribute__((noinline)) uintptr_t interior_address() { return reinterpret_cast<uintptr_t>(&left::target) + 1; }
extern "C" __attribute__((noinline)) uintptr_t data_address() { return reinterpret_cast<uintptr_t>(&left::data); }

int main(int argc, char **argv) {
    unsigned x = argc > 1 ? static_cast<unsigned>(std::strtoul(argv[1], nullptr, 0)) : 0;
    std::printf("%u %u %u %u %u %u %u %u %u\n",
                reinterpret_cast<Callback>(foreign_address())(x),
                reinterpret_cast<Callback>(nested_address())(x),
                reinterpret_cast<Callback>(right_address())(x),
                reinterpret_cast<Callback>(global_shadow(x))(x),
                reinterpret_cast<Callback>(scope_shadow(x))(x),
                reinterpret_cast<Callback>(left::same_address())(x),
                reinterpret_cast<Callback>(right::cross_address())(x),
                reinterpret_cast<Callback>(interior_address() - 1)(x),
                *reinterpret_cast<unsigned *>(data_address()));
}

#include <cstdio>
#include <cstdlib>
#include <cstdint>
using Callback = unsigned (*)(unsigned);
namespace left { __attribute__((noinline)) unsigned target(unsigned x) { return x + 10; } }
extern "C" __attribute__((noinline)) uintptr_t get_target() { return reinterpret_cast<uintptr_t>(&left::target); }
int main(int argc, char **argv) { unsigned x = argc > 1 ? std::strtoul(argv[1], nullptr, 0) : 0; std::printf("%u\n", reinterpret_cast<Callback>(get_target())(x)); }

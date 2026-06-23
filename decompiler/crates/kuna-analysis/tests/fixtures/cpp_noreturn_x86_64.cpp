// Tiny C++ fixture for the no-return x demangle cross-pass seam.
// fail() tail-calls std::terminate() (mangled _ZSt9terminatev, demangled
// std::terminate, scope std) — a no-return symbol whose funcsym is renamed by
// the demangle pass before install, so it must be resolved by ADDRESS.
#include <exception>
namespace app {
struct Guard {
  // throw_it() raises, which lowers to __cxa_throw (also in the no-return list).
  void throw_it(int x) { if (x) throw x; }
};
}
__attribute__((noinline)) void fail() {
  std::terminate();   // -> _ZSt9terminatev
}
int main(int argc, char** argv) {
  app::Guard g;
  g.throw_it(argc);
  if (argc > 7) fail();
  return argc;
}

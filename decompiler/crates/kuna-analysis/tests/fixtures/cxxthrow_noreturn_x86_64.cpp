// Fixture source for the libstdc++ `std::__throw_*` no-return family (GH-273,
// kuna DIV-78). Build (checked in as `cxxthrow_noreturn_x86_64`):
//
//   clang++ -O1 -no-pie -fno-pic -fno-asynchronous-unwind-tables \
//           -o cxxthrow_noreturn_x86_64 cxxthrow_noreturn_x86_64.cpp
//
// The two helpers are declared here WITHOUT `__attribute__((__noreturn__))` (the
// real `<bits/functexcept.h>` declares them noreturn), on purpose: the compiler
// then believes the call returns and emits the code after it. That is exactly the
// shape a real C++ binary has from the *decompiler's* point of view, where no
// attribute survives — the callee is an undefined `.dynsym` import
// (`_ZSt20__throw_length_errorPKc`) reached through a PLT stub, and the only thing
// that can prove the call cannot return is the shipped no-return name list.
//
// No libstdc++ header is included (declaring the helpers ourselves would clash
// with the real noreturn declarations); the symbols still resolve to the real
// libstdc++ definitions at link time, so the binary carries genuine UND imports.
namespace std {
void __throw_length_error(const char *);
void __throw_out_of_range(const char *);
} // namespace std

// The witness shape: the code after the throw call is reachable ONLY by falling
// through the call, so it is dead in fact. Without the list entry the decompiler
// follows that fall-through and emits `n * 7` as if it ran.
extern "C" long append_bound(long n) {
  if (n <= 100)
    return n * 3;
  std::__throw_length_error("basic_string::append");
  return n * 7;
}

// A second family member, so the fixture is not a single-symbol proof.
extern "C" long at_bound(long n) {
  if (n < 64)
    return n + 1;
  std::__throw_out_of_range("vector::at");
  return n * 11;
}

int main() { return (int)(append_bound(3) + at_bound(2)); }

// Fixture for `itaniumrtti` constructor naming: optimized helpers that a
// constructor or destructor was inlined into are not constructors, and a name
// the symbol table gives a function is never joined by a recovered one.
//
//   clang++ -O2 -g0 -fPIC -shared -fvisibility=hidden -fvisibility-inlines-hidden \
//       -o itaniumctors_wrap_O2_x86_64.so itaniumctors_wrap_x86_64.cpp
//   strip --strip-all itaniumctors_wrap_O2_x86_64.so
//   g++ -O0 -g0 -fPIC -shared -fvisibility=hidden -fvisibility-inlines-hidden \
//       -o itaniumctors_wrap_O0sym_x86_64.so itaniumctors_wrap_x86_64.cpp
//
// (clang 14.0.0 and g++ 11.4.0, Ubuntu 22.04; the -O0 build is NOT stripped.)
// At -O2 `Gadget`'s destructor keeps its vptr store (it makes a virtual call),
// so the class is known to have a virtual destructor, and each helper below
// stores a vptr through its first parameter exactly as a constructor does:
//   make_gadget    returns a Gadget by value (the store is to the return slot)
//   place_gadget   placement new into caller storage
//   Holder::Holder a non-polymorphic class whose first member is a Gadget
//   drop_widget    `delete p` of a final class, its destructors inlined
// At -O0 the always_inline constructor is still inlined into Holder::Holder,
// whose own symbol must stay its only name.
#include <new>

#define API extern "C" __attribute__((visibility("default")))
#define NOINL __attribute__((noinline))

API void note(int);

namespace kit {
struct Gadget {
    int v;
    __attribute__((always_inline)) Gadget(int x) : v(x) {}
    virtual ~Gadget() { note(get()); }
    virtual int get() const { return v; }
};

struct Widget final : Gadget {
    Widget(int x) : Gadget(x) {}
    int get() const override { return v + 1; }
};

NOINL Gadget make_gadget(int x) { return Gadget(x); }
NOINL void place_gadget(void *p, int x) { new (p) Gadget(x); }

struct Holder {
    Gadget g;
    int extra;
    Holder(int x);
};
NOINL Holder::Holder(int x) : g(x), extra(x + 1) {}

NOINL void drop_widget(Widget *p) { delete p; }
} // namespace kit

using namespace kit;

API int use_gadgets(int x) {
    Gadget a = make_gadget(x);
    alignas(Gadget) unsigned char mem[sizeof(Gadget)];
    place_gadget(mem, x);
    Holder h(x);
    Widget *p = new Widget(x);
    int r = a.get() + reinterpret_cast<Gadget *>(mem)->get() + h.g.get() + p->get();
    drop_widget(p);
    return r;
}

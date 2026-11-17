// Fixture for `itaniumrtti` constructor/destructor naming: functions that store
// a class's vptr through `this` exactly as a constructor or destructor does, but
// are neither.
//
//   g++ -O0 -g0 -fPIC -shared -fvisibility=hidden -fvisibility-inlines-hidden \
//       -o itaniumctors_impostor_O0_x86_64.so itaniumctors_impostor_x86_64.cpp
//   g++ -O2 -g0 -fPIC -shared -fvisibility=hidden -fvisibility-inlines-hidden \
//       -o itaniumctors_impostor_O2_x86_64.so itaniumctors_impostor_x86_64.cpp
//   strip --strip-all itaniumctors_impostor_O0_x86_64.so itaniumctors_impostor_O2_x86_64.so
//
// (g++ 11.4.0, binutils 2.38, Ubuntu 22.04.)
// At -O0:
//   Mid::~Mid    a pure virtual destructor: no vtable slot holds it, and it
//                stores Mid's vptr before it calls Base::~Base, where Mid::Mid
//                stores it after calling Base::Base
//   Holder::Holder  the always_inline Gadget constructor inlined into it, its
//                `this` read back from the inlined parameter's own frame slot
// At -O2 each of these sits in its class's vtable and stores the class's vptr:
//   Obj::Release   `delete this` behind the reference-count test
//   Once::Release  `delete this` in a final class: the deleting destructor's code
//   Cell::reset    destroys and rebuilds `*this`
#include <new>

#define API extern "C" __attribute__((visibility("default")))

API void note(const void *);

namespace imp {
struct Base {
    virtual ~Base();
    virtual int f();
    int b = 1;
};
Base::~Base() { note(this); }
int Base::f() { return b; }

struct Mid : Base {
    ~Mid() override = 0;
    int f() override;
    int m = 2;
};
Mid::~Mid() { note(this); }
int Mid::f() { return m; }

struct Leaf final : Mid {
    ~Leaf() override;
    int f() override;
    int l = 3;
};
Leaf::~Leaf() { note(this); }
int Leaf::f() { return l; }

struct Gadget {
    __attribute__((always_inline)) Gadget(int x) : v(x) {}
    virtual ~Gadget();
    virtual int get() const;
    int v;
};
Gadget::~Gadget() { note(this); }
int Gadget::get() const { return v; }

struct Holder {
    Gadget g;
    int extra;
    Holder(int x);
};
Holder::Holder(int x) : g(x), extra(x + 1) {}

struct IUnk {
    virtual void AddRef() = 0;
    virtual void Release() = 0;

  protected:
    virtual ~IUnk();
};
IUnk::~IUnk() { note(this); }

struct Obj : IUnk {
    void AddRef() override;
    void Release() override;
    ~Obj() override;
    int rc = 1;
};
void Obj::AddRef() { ++rc; }
void Obj::Release() {
    if (--rc == 0)
        delete this;
}
Obj::~Obj() { note(this); }

struct Once final : IUnk {
    void AddRef() override;
    void Release() override;
    ~Once() override;
};
void Once::AddRef() {}
void Once::Release() { delete this; }
Once::~Once() { note(this); }

struct Cell {
    Cell();
    virtual ~Cell();
    virtual void reset();
    int v;
};
Cell::Cell() : v(0) {}
Cell::~Cell() { note(this); }
void Cell::reset() {
    this->~Cell();
    new (this) Cell();
}
} // namespace imp

using namespace imp;

API void *make(int k) {
    switch (k) {
    case 0:
        return new Leaf;
    case 1:
        return new Holder(k);
    case 2:
        return new Obj;
    case 3:
        return new Once;
    default:
        return new Cell;
    }
}

// Fixture for `itaniumrtti` destructor naming: virtual methods that store their
// own class's vptr through `this` before any call or branch, as a destructor
// does, because they rebuild `*this` with placement new or delete it.
//
//   g++ -O2 -g0 -fPIC -shared -fvisibility=hidden -fvisibility-inlines-hidden \
//       -o itaniumctors_rebuild_O2_x86_64.so itaniumctors_rebuild_x86_64.cpp
//   clang++ -O2 -g0 -fPIC -shared -fvisibility=hidden -fvisibility-inlines-hidden \
//       -o itaniumctors_rebuild_clang_O2_x86_64.so itaniumctors_rebuild_x86_64.cpp
//   strip --strip-all itaniumctors_rebuild_O2_x86_64.so itaniumctors_rebuild_clang_O2_x86_64.so
//
// (g++ 11.4.0, clang 14.0.0, binutils 2.38, Ubuntu 22.04.)
// Each of these sits in its class's vtable and stores the class's vptr first:
//   Again::reinit  placement-new rebuild, no virtual destructor
//   Fresh::reinit  placement-new rebuild beside a trivial virtual destructor
//   Redo::reset    a trivial destroy and a rebuild whose constructor calls out
//   Self::destroy  `delete this` with a non-virtual destructor
//   Grow::reset    destroy and rebuild; a polymorphic member's vptr follows a call
//   Fill::reinit   rebuild; a polymorphic member's vptr follows a call
//   Base::morph    destroys a Base and builds a Derived in its place
// Base's own destructors are named. Iface is abstract: clang's deleting
// destructor for it is a trap, and its complete destructor is still named.
#include <new>

#define API extern "C" __attribute__((visibility("default")))

API void note(const void *);
API void note2(const void *);

namespace rb {
struct Again {
    Again();
    virtual void reinit();
    virtual int get();
    int v;
};
Again::Again() : v(7) {}
void Again::reinit() { new (this) Again(); }
int Again::get() { return v; }

struct Fresh {
    Fresh();
    virtual ~Fresh() = default;
    virtual void reinit();
    virtual int get();
    int v;
};
Fresh::Fresh() : v(7) { note(this); }
void Fresh::reinit() { new (this) Fresh(); }
int Fresh::get() { return v; }

struct Redo {
    Redo();
    virtual void reset();
    virtual int get();
    int v;
};
Redo::Redo() : v(0) { note(this); }
void Redo::reset() {
    this->~Redo();
    new (this) Redo();
}
int Redo::get() { return v; }

struct IDestroy {
    virtual void destroy() = 0;
    virtual int id() = 0;

  protected:
    ~IDestroy() = default;
};
struct Self : IDestroy {
    void destroy() override;
    int id() override;
    ~Self();
    int x = 0;
};
Self::~Self() { note(this); }
void Self::destroy() { delete this; }
int Self::id() { return x; }

struct Member {
    Member() : x(1) {}
    virtual ~Member() { note2(this); }
    virtual int m() { return x; }
    int x;
};
struct Grow {
    Grow();
    ~Grow();
    virtual void reset();
    virtual int get();
    Member m;
    int v;
};
Grow::Grow() : v(0) {}
Grow::~Grow() { note(this); }
void Grow::reset() {
    this->Grow::~Grow();
    new (this) Grow();
}
int Grow::get() { return v + m.m(); }

struct Logged {
    Logged();
    int z;
};
Logged::Logged() : z(3) { note2(this); }
struct Plain {
    Plain() : x(1) {}
    virtual ~Plain() {}
    virtual int m() { return x; }
    int x;
};
struct Fill {
    Fill();
    virtual void reinit();
    virtual int get();
    Logged l;
    Plain p;
    int v;
};
Fill::Fill() : v(0) {}
void Fill::reinit() { new (this) Fill(); }
int Fill::get() { return v + p.m() + l.z; }

struct Base {
    Base();
    virtual void morph();
    virtual ~Base();
    virtual int get();
    int b;
};
struct Derived : Base {
    Derived();
    int get() override;
    int d;
};
Base::Base() : b(1) {}
Base::~Base() { note(this); }
Derived::Derived() : d(2) { note2(this); }
void Base::morph() {
    this->Base::~Base();
    new (this) Derived();
}
int Base::get() { return b; }
int Derived::get() { return d; }

struct Iface {
    virtual ~Iface();
    virtual int run() = 0;
};
Iface::~Iface() { note(this); }
struct Impl : Iface {
    int run() override;
    int r = 4;
};
int Impl::run() { return r; }
} // namespace rb

using namespace rb;

API void *make(int k) {
    switch (k) {
    case 0: return new Again;
    case 1: return new Fresh;
    case 2: return new Redo;
    case 3: return new Self;
    case 4: return new Grow;
    case 5: return new Fill;
    case 6: return new Derived;
    default: return new Impl;
    }
}

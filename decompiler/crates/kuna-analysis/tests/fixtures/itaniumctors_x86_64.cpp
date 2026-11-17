// Fixture for `itaniumrtti` constructor/destructor recovery from vptr stores.
//
// Built twice as a STRIPPED, hidden-visibility shared object, so the only type
// information is the Itanium RTTI graph and nothing names a constructor:
//
//   g++ -O0 -g0 -fPIC -shared -fvisibility=hidden -fvisibility-inlines-hidden \
//       -o itaniumctors_O0_x86_64.so itaniumctors_x86_64.cpp
//   g++ -O2 -g0 -fPIC -shared -fvisibility=hidden -fvisibility-inlines-hidden \
//       -o itaniumctors_O2_x86_64.so itaniumctors_x86_64.cpp
//   strip --strip-all itaniumctors_O0_x86_64.so itaniumctors_O2_x86_64.so
//
// (g++ 13.3.0, Ubuntu 24.04.) The shapes the vptr-store rule must get right or
// refuse:
//   Animal   virtual dtor declared AFTER a virtual method (D1/D0 at slots 1/2),
//            and a second, copy, constructor
//   Dog      a vtable slot (`clone`) that constructs a Dog
//   Counter  polymorphic WITHOUT a virtual destructor: its constructor and
//            destructor store the same vptr the same way, so neither is named
//   Cat      multiple inheritance (Named at +16)
//   make_dog / make_cat / on_stack / use_all: callers that construct objects;
//            at -O2 the constructors are inlined into them, and they store the
//            vptr through operator new's result or their own frame, never `this`
#include <new>

namespace zoo {

// Virtual dtor declared AFTER a virtual method: the D1/D0 pair is at slots 1/2.
class Animal {
public:
    int legs;
    explicit Animal(int l) : legs(l) {}
    Animal(const Animal &o) : legs(o.legs + 1) {}   // a second (copy) constructor
    virtual int speak() const { return legs; }
    virtual ~Animal() {}
    virtual int weight() const { return legs * 2; }
};

class Dog : public Animal {
public:
    int bark;
    Dog(int l, int b) : Animal(l), bark(b) {}
    virtual int speak() const { return bark; }
    virtual ~Dog() { bark = 0; }
    virtual Animal *clone() const { return new Dog(*this); }  // a vtable slot that constructs
};

// Polymorphic class WITHOUT a virtual destructor.
class Counter {
public:
    int n;
    explicit Counter(int x) : n(x) {}
    ~Counter() { n = -1; }
    virtual int next() { return ++n; }
};

// Abstract interface + multiple inheritance.
class Named {
public:
    virtual ~Named() {}
    virtual const char *name() const = 0;
};

class Cat : public Animal, public Named {
public:
    int lives;
    explicit Cat(int l) : Animal(4), lives(l) {}
    virtual ~Cat() {}
    virtual const char *name() const { return "cat"; }
};

} // namespace zoo

using namespace zoo;

// Factories: a free function that constructs (inlined at -O2) is NOT a constructor.
extern "C" __attribute__((visibility("default"))) Animal *make_dog(int b) {
    return new Dog(4, b);
}

extern "C" __attribute__((visibility("default"))) Animal *make_cat(int l) {
    return new Cat(l);
}

extern "C" __attribute__((visibility("default"))) int on_stack(int x) {
    Dog d(3, x);
    Counter c(x);
    Animal *a = &d;
    int r = a->speak() + c.next();
    return r;
}

extern "C" __attribute__((visibility("default"))) int use_all(Animal *a, Named *n) {
    Animal *b = static_cast<Dog *>(a)->clone();
    int r = b->weight() + (n->name()[0]);
    delete b;
    return r;
}

extern "C" __attribute__((visibility("default"))) void kill(Animal *a) { delete a; }

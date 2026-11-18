// Fixture for `itaniumrtti` constructor and destructor naming: a function that
// returns a polymorphic class by value builds it in a return slot passed as its
// first parameter, so it stores the class's vptr where a constructor or a
// destructor stores `this`'s.
//
//   g++ -O2 -g0 -fPIC -shared -fvisibility=hidden -fvisibility-inlines-hidden \
//       -o itaniumctors_byvalue_O2_x86_64.so itaniumctors_byvalue.cpp
//   clang++ -target i386-linux-gnu -O2 -g0 -fno-pic -fno-exceptions \
//       -fno-asynchronous-unwind-tables -nostdinc++ -fvisibility=hidden \
//       -fvisibility-inlines-hidden -c -o itaniumctors_byvalue_i386.o itaniumctors_byvalue.cpp
//   ld -m elf_i386 -shared -z notext -o itaniumctors_byvalue_O2_i386.so itaniumctors_byvalue_i386.o
//   strip --strip-all itaniumctors_byvalue_O2_x86_64.so itaniumctors_byvalue_O2_i386.so
//
// (g++ 11.4.0, clang 14.0.0, binutils 2.38, Ubuntu 22.04.)
// Virtual methods returning their own class by value, whose `this` is the second
// parameter: Vec::scaled and Vec::shifted (neighbours, beside a trivial virtual
// destructor), Num::twice and Num::half (no virtual destructor), Color::darker
// and Color::brighter (Color implements an interface), Copy::copy (beside a
// method that only traps), and Zero::zero and Zero::unit, which never read
// `*this` (on x86-64 they return the slot's address; on i386 nothing tells them
// apart). Pair's destructors are named; its by-value methods are not.
// Pager::collect returns a polymorphic List by value through a slot spilled
// across calls it was not passed to; it is not a constructor.

#define API __attribute__((visibility("default")))

extern "C" void note(const void *);
extern "C" int ext(int);
extern "C" void *xalloc(unsigned long);

namespace bv {
struct Vec {
  virtual ~Vec() = default;
  virtual Vec scaled(int) const;
  virtual Vec shifted(int) const;
  virtual int len() const;
  int x = 0, y = 0;
  Vec() = default;
  Vec(int a, int b) : x(a), y(b) {}
};
Vec Vec::scaled(int k) const { return Vec(x * k, y * k); }
Vec Vec::shifted(int k) const { return Vec(x + k, y + k); }
int Vec::len() const { return x + y; }

struct Num {
  virtual Num twice() const;
  virtual Num half() const;
  virtual int get() const;
  int x = 0;
  Num() = default;
  explicit Num(int a) : x(a) {}
};
Num Num::twice() const { return Num(x * 2); }
Num Num::half() const { return Num(x / 2); }
int Num::get() const { return x; }

struct Printable {
  virtual void print() const = 0;
};
struct Color : Printable {
  void print() const override;
  virtual Color darker() const;
  virtual Color brighter() const;
  int r = 0, g = 0, b = 0;
  Color() = default;
  Color(int a, int b_, int c) : r(a), g(b_), b(c) {}
};
void Color::print() const { note(this); }
Color Color::darker() const { return Color(r / 2, g / 2, b / 2); }
Color Color::brighter() const { return Color(r * 2, g * 2, b * 2); }

struct Copy {
  virtual ~Copy() = default;
  virtual Copy copy() const;
  virtual int unsupported() const;
  int x = 0;
  Copy() = default;
  explicit Copy(int q) : x(q) {}
};
Copy Copy::copy() const { return Copy(x); }
int Copy::unsupported() const { __builtin_trap(); }

struct Zero {
  virtual ~Zero() = default;
  virtual Zero zero() const;
  virtual Zero unit() const;
  virtual int len() const;
  int x = 0;
  Zero() = default;
  explicit Zero(int q) : x(q) {}
};
Zero Zero::zero() const { return Zero(); }
Zero Zero::unit() const { return Zero(1); }
int Zero::len() const { return x; }

struct Pair {
  virtual ~Pair();
  virtual Pair clone() const;
  virtual Pair reversed() const;
  int x = 0, y = 0;
  Pair() = default;
  Pair(int a, int b) : x(a), y(b) {}
};
Pair::~Pair() { note(this); }
Pair Pair::clone() const { return *this; }
Pair Pair::reversed() const { return Pair(y, x); }

template <class T> struct List {
  List() : d(nullptr), n(0) {}
  virtual ~List();
  void append(T t) {
    if (!d)
      d = (T *)xalloc(64);
    d[n++] = t;
  }
  T *d;
  int n;
};
template <class T> List<T>::~List() { note(d); }

struct Page {
  int a;
};
struct Pager {
  static List<Page *> collect(int a, int b, int c, int d, int e, int f);
};
List<Page *> Pager::collect(int a, int b, int c, int d, int e, int f) {
  int t = ext(a);
  t += ext(b) * c;
  t ^= ext(d) + e;
  t -= ext(f) * a * b;
  int u = ext(t) + a + b + c + d + e + f;
  List<Page *> l;
  l.n = t + u;
  return l;
}
} // namespace bv

extern "C" API void *use_all(int k) {
  switch (k) {
  case 0: return new bv::Vec(1, 2);
  case 1: return new bv::Num(3);
  case 2: return new bv::Color(1, 2, 3);
  case 3: return new bv::Copy(4);
  case 4: return new bv::Zero(5);
  case 5: return new bv::Pair(1, 2);
  }
  auto y = bv::Pager::collect(k, k + 1, k + 2, k + 3, k + 4, k + 5);
  note(&y);
  return nullptr;
}

// Fixture for kuna-vtabletypes.xml. Built with
//   g++ -O1 -fno-pic -no-pie -fno-exceptions -fno-stack-protector
//       -fno-asynchronous-unwind-tables -ffreestanding -nostdlib -static
//       -Wl,-z,noseparate-code -Wl,--build-id=none -o vtabletypes vtabletypes.cpp
// The C++ ABI objects the compiler refers to are defined here, so nothing
// else is linked in.
namespace __cxxabiv1 {
struct __class_type_info { virtual void f(); };
struct __si_class_type_info { virtual void f(); };
void __class_type_info::f() {}
void __si_class_type_info::f() {}
}
extern "C" void __cxa_pure_virtual() { for (;;) {} }

struct Shape {
    virtual int area(int s) const = 0;
    virtual int scaled(int s) const { return area(s) * 2 + id; }
    int id = 7;
};
struct Sq : Shape { int area(int s) const override { return s * s + id; } };
struct Tr : Shape { int area(int s) const override { return s * s / 2; } };

__attribute__((noinline)) int use(const Shape *p, int n) { return p->area(n) + p->scaled(n); }
__attribute__((noinline, optimize("O0"))) int local(int n) { Tr t; const Shape &r = t; return r.area(n); }

Shape *volatile pick;
extern "C" void _start() {
    Sq a;
    Tr b;
    pick = &a;
    use(pick, 3);
    pick = &b;
    for (;;) use(pick, local(4));
}

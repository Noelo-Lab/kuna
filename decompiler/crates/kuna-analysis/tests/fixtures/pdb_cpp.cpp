// pdb_cpp.cpp — a freestanding MSVC-ABI C++ PE whose function names live only in
// its `.pdb`, as decorated `S_PUB32` publics (`??0Box@app@@QEAA@H@Z`, ...).
//
// A class with a virtual destructor makes the compiler emit the `scalar deleting
// destructor` (`??_G`), which `delete` reaches through the vftable; `operator
// new`/`operator delete` are defined here so nothing needs the CRT.
//
//   clang++ --target=x86_64-pc-windows-msvc -O1 -fno-rtti -fno-exceptions \
//       -ffreestanding -gcodeview -g -c pdb_cpp.cpp -o pdb_cpp.obj
//   lld-link /entry:mainCRTStartup /subsystem:console /nodefaultlib /debug \
//       /pdbaltpath:pdb_cpp.pdb /out:pdb_cpp.exe /pdb:pdb_cpp.pdb pdb_cpp.obj

typedef unsigned long long size_t;

static char heap[256];
static size_t heap_used;

void *operator new(size_t n) {
    void *p = heap + heap_used;
    heap_used += (n + 15) & ~(size_t)15;
    return p;
}

void operator delete(void *) {}
void operator delete(void *, size_t) {}

namespace app {
namespace util {
__attribute__((noinline)) int scale(int x) { return x * 3 + 1; }
}

struct Shape {
    virtual ~Shape();
    virtual int area() const;
};

struct Box : Shape {
    int side;
    explicit Box(int s);
    ~Box() override;
    int area() const override;
};

Shape::~Shape() {}
int Shape::area() const { return 0; }

__attribute__((noinline)) Box::Box(int s) : side(util::scale(s)) {}
Box::~Box() { side = 0; }
int Box::area() const { return side * side; }
}

extern "C" int mainCRTStartup() {
    app::Shape *volatile s = new app::Box(2);
    int a = s->area();
    delete s;
    return a;
}

// Polymorphic C++ exercising MSVC RTTI / vftable recovery (s1_rtti).
//
// One base class (Shape) + one derived class (Box), each with a virtual method,
// so the MSVC C++ ABI emits in .rdata: per class a CompleteObjectLocator (RTTI4)
// immediately before the vftable, the RTTI0 TypeDescriptors carrying the
// `.?AVShape@@` / `.?AVBox@@` mangled class names (clang emits `.?AU...` for a
// `struct`), and the RTTI1/2/3 hierarchy descriptors.  Freestanding (no CRT): the
// only CRT symbol the RTTI Type Descriptors reference is the `type_info` vftable
// (??_7type_info@@6B@), which we supply (in inline asm, so the symbol name is
// identical on x86 and x64) as a single pointer cell so the image links standalone
// while keeping the *real* MSVC C++ RTTI ABI bytes.  No dynamic_cast / new /
// static-guard / virtual dtor, so no other CRT helper is pulled in.
struct Shape {
    virtual int area() const { return 0; }
};
struct Box : Shape {
    int side;
    int area() const override { return side * side; }
};

// A global Box: construction stores Box's vftable into the object, anchoring Box's
// vftable + COL + RTTI graph (the linker keeps them, .rdata carries them).
Box g_box;

// Virtual dispatch through a base pointer the optimizer cannot devirtualize, so
// Shape's vftable slot is live too.
extern "C" int boxArea(Shape *s) {
    return s->area();
}

extern "C" int mainCRTStartup() {
    g_box.side = 7;
    Shape *volatile p = &g_box;
    return boxArea(p);
}

// Stand in for the CRT `const type_info::`vftable'` (mangled ??_7type_info@@6B@):
// a single pointer cell, defined in inline asm so the mangled label is byte-for-byte
// the same on x86 and x64 (a C array would be `___vftable...`/`__vftable...`,
// underscore-decorated differently per arch).  The RTTI0 TypeDescriptors only POINT
// at it (so they all agree -- exactly how the analyzer finds the common type_info
// vftable).
__asm__(
    ".section .rdata,\"dr\"\n"
    ".globl \"\x3f\x3f_7type_info@@6B@\"\n"
    "\"\x3f\x3f_7type_info@@6B@\":\n"
    "  .zero 8\n");

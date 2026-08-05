// Fixture for `itaniumrtti` (Itanium GCC/Clang RTTI + vtable recovery).
//
// Built as a STRIPPED shared object, so `.symtab` is gone and the only surviving
// type information is the Itanium ABI graph itself: the `.rela.dyn` relocations
// naming `__cxxabiv1::__{,si_,vmi_}class_type_info`'s vtables, the `_ZTS...` type
// name strings in `.rodata`, and the `_ZTV...` vtable objects in `.data.rel.ro`.
// Every class below is deliberately NOT exported by name (they are only reachable
// through the two exported factory functions), so no `.dynsym` FUNC symbol names a
// virtual method — the recovery has to come from the RTTI graph alone.
//
//   g++ -O0 -g0 -fPIC -shared -fvisibility=hidden -fvisibility-inlines-hidden \
//       -o itaniumrtti_x86_64.so itaniumrtti_x86_64.cpp
//   strip --strip-all itaniumrtti_x86_64.so
//
// Hidden visibility is what makes this a real test: without it every implicit
// class method is emitted WEAK and exported, so `.dynsym` alone would already name
// them and the recovery would have nothing to prove. Hidden, the only defined
// dynamic symbols are the two `probe_*` entry points.
//
// Covers all three `__cxxabiv1` typeinfo flavours:
//   Shape     -> __class_type_info      (no bases)
//   Circle    -> __si_class_type_info   (one base at offset 0)
//   Widget    -> __vmi_class_type_info  (two bases; Drawable at a nonzero offset,
//                                        so its sub-vtable needs `this`-adjusting
//                                        thunks and offset-to-top = -16)
// ...plus the two naming hazards that cost real recovery when mishandled:
//   Vec<int> / Vec<double>  -> distinct classes whose NAME-ONLY demangling collides
//   (anonymous ns)::Hidden  -> a TU-local type, whose ABI name string carries the
//                              leading `*` marker

namespace shapes {

class Shape {
public:
    int tag;
    explicit Shape(int t) : tag(t) {}
    virtual ~Shape() {}
    virtual int area(int scale) const { return scale * tag; }
    virtual int perimeter() const { return tag; }
};

class Circle : public Shape {
public:
    int radius;
    Circle(int t, int r) : Shape(t), radius(r) {}
    virtual ~Circle() {}
    virtual int area(int scale) const { return scale * radius * radius * 3; }
};

class Loggable {
public:
    int level;
    explicit Loggable(int l) : level(l) {}
    virtual ~Loggable() {}
    virtual int emit(int code) { return code + level; }
};

class Drawable {
public:
    int pen;
    explicit Drawable(int p) : pen(p) {}
    virtual ~Drawable() {}
    virtual int render(int code) { return code * pen; }
};

// Two polymorphic bases: the Drawable subobject sits at a nonzero offset, so the
// vtable object carries a SECOND sub-vtable (offset-to-top -16) of thunks.
class Widget : public Loggable, public Drawable {
public:
    int width;
    Widget(int l, int p, int w) : Loggable(l), Drawable(p), width(w) {}
    virtual ~Widget() {}
    virtual int emit(int code) { return code + level + width; }
    virtual int render(int code) { return code * pen - width; }
};

// Two INSTANTIATIONS of one template: distinct classes with distinct vtables whose
// name-only demangling would collide (`Vec<int>` and `Vec<double>` both reduce to
// `Vec`), which would silently cost one of them its whole recovery.
template <typename T>
class Vec {
public:
    T v;
    explicit Vec(T x) : v(x) {}
    virtual ~Vec() {}
    virtual T get() const { return v; }
};

} // namespace shapes

// A translation-unit-local class. The Itanium ABI marks its type-name string with a
// leading `*` (compare by pointer, not by string), which is NOT part of the mangled
// name — the case that covers every anonymous-namespace implementation class.
namespace {
class Hidden {
public:
    int h;
    explicit Hidden(int x) : h(x) {}
    virtual ~Hidden() {}
    virtual int scaled() { return h * 9; }
};
} // namespace

using namespace shapes;

// The only exported entry points; every class above is internal to them.
extern "C" __attribute__((visibility("default"))) int probe_shapes(int scale) {
    Circle c(2, 5);
    Shape *s = &c;
    return s->area(scale) + s->perimeter();
}

extern "C" __attribute__((visibility("default"))) int probe_widget(int code) {
    Widget w(1, 2, 3);
    Drawable *d = &w;
    return d->render(code) + w.emit(code);
}

extern "C" __attribute__((visibility("default"))) int probe_generic(int n) {
    Vec<int> vi(n);
    Vec<double> vd(n * 1.5);
    Hidden hidden(n);
    return vi.get() + (int)vd.get() + hidden.scaled();
}

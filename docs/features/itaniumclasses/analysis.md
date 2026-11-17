# itaniumclasses: analysis

## The construct

On a stripped GCC/Clang C++ ELF, `itaniumrtti` recovers every polymorphic class's
name, its base list and its vtables, and names the slot functions. It recovers no
*type*. The object a method is called on stays `void *` or a per-function
`struct_N`, and a virtual call prints as arithmetic on the vptr.

`decompiler/crates/kuna-analysis/tests/fixtures/itaniumrtti_x86_64.so`, source:

```cpp
extern "C" int probe_shapes(int scale) {
    Circle c(2, 5);
    Shape *s = &c;
    return s->area(scale) + s->perimeter();
}
```

`kuna decompile itaniumrtti_x86_64.so probe_shapes`, main:

```c
  long v3 [3]; // rax | stack - 0x38

  sub_14ea(v3,2,5);
  v1 = (**(int (**)(void *,unsigned int))(v3[0] + 0x10))(v3,a0);
  v2 = (**(int (**)(void *))(v3[0] + 0x18))(v3);
  shapes::Circle::vfunc_0(v3);
```

and the method `Shape::area`:

```c
int shapes::Shape::vfunc_2(void *a0,int a1)
{
  return ((int *)a0)[2] * a1;
}
```

The reader has to work out that `v3[0] + 0x10` is slot 2 of some vtable, find which
class `v3` is, and look the slot up by hand. Everything needed to print it is
already recovered: the class is `Circle`, its vtable has four slots, slot 2 is
`vfunc_2`.

## The engine already renders it

Nothing in type propagation or the printer is missing. Declaring the types by hand
on the same binary gives the target output:

```text
typedef struct Shape_vtbl { code *vfunc_0; code *vfunc_1; code *vfunc_2; code *vfunc_3; } Shape_vtbl;
typedef struct Shape { Shape_vtbl *vptr; int field_0x8; int field_0xc; } Shape;
typedef struct Circle { Shape base; } Circle;
prototype 0x14ea void Circle(Circle *this,int a1,int a2)
```

passed as `--assert @file`, plus `--assert 'type v3 Circle'`:

```c
  Circle v3; // rax | stack - 0x38

  shapes::Circle::Circle(&v3,2,5);
  v1 = (*(int (*)(void *,unsigned int))(v3.base.vptr)->vfunc_2)(&v3,a0);
  v2 = (*(int (*)(void *))(v3.base.vptr)->vfunc_3)(&v3);
```

So the gap is one decision in P1 external-refinement: `itaniumrtti` stops at names
where it could also emit the class and vtable types it has the evidence for.

## What the experiments say about how to declare them

All on the fixture, with hand-written `--assert` files standing in for the pass.

| Slot field type | Virtual call prints as | Verdict |
|---|---|---|
| `void *` | `(**(int (**)(void *,unsigned int))((long)v3.base + 0x10))(&v3,a0)` | no gain: the field is not used |
| `code *` | `(*(int (*)(void *,unsigned int))(v3.base.vptr)->vfunc_2)(&v3,a0)` | named slot, call-site signature kept |
| `long (*)(Shape *this)` (arity guessed) | `(*(v3.base.vptr)->vfunc_2)(&v3.base)` | **wrong**: the `a0` argument is dropped |
| `long (*)(Shape *this,...)` | `(*(v3.base.vptr)->vfunc_2)(&v3.base,(unsigned long)a0)` | return type forced to `long`, argument widened |
| the true signature | `(*(v3.vptr)->vfunc_2)((Shape *)&v3,a0)` | best, but needs the callee's prototype |

A declared slot signature is taken as fact at the call site, so a guessed one
produces false output. `code *` is the only form that is both useful and safe
without knowing the callee's prototype.

| Class struct declared as | `Shape::area` prints as | Verdict |
|---|---|---|
| nothing (main) | `((int *)a0)[2] * a1` | baseline |
| `{ Shape_vtbl *vptr; }` (too small) | `*(int *)&this[1].vptr * v1` | **worse than baseline** |
| `{ Shape_vtbl *vptr; int field_0x8; }` | `this->field_0x8 * a1` | the target |

A class struct that does not cover the bytes a method touches is worse than no
struct. The layout therefore has to be at least as large as every access through
`this` that the pass can see, or it must not be emitted.

Locking `this` alone as a parameter (`param 0x14b8::0 RDI Shape *this`) fixes the
arity at one and unbinds the method's other parameters. The unlocked input hint
(`kuna_inputhints`, used by `swiftcall` for `self`) names and types one input
without that effect.

## Where the layout evidence is

RTTI gives the class name, the bases, and for a `__vmi_class_type_info` class the
offset of every non-primary base. It gives no size and no data members. The binary
does:

- The deleting destructor ends in `operator delete(this, sizeof(C))` wherever sized
  deallocation is compiled in (the GCC and Clang default from C++14). On the
  fixture: `operator delete(this,0x10)` for `Shape`. In `libapt-pkg.so.6.0.0`:
  `pkgAcquire::Item::~Item` ends in `operator delete(this,0x198)`.
- `operator new(N)` whose result is handed to a constructor.
- The constant offsets of the loads and stores through `this` in the class's own
  slot functions, constructors and destructors.

## How much output this touches

`kuna decompile-all libapt-pkg.so.6.0.0` (Ubuntu 24.04, 4497 functions, main plus
#1606), counted with regular expressions over the emitted C, so approximate:

- about 495 call sites of the shape `(**(sig)(*obj + 0xNN))(obj, ...)`, in 278
  functions;
- 359 functions named only by slot index (`<C>::vfunc_<i>`), 290 of them with a
  `void *`, integer-pointer or `struct_N *` receiver;
- 173 classes with at least one named slot function.

## Reference

The reference here is the source: `s->area(scale)` on a `Shape *`. Ghidra reaches
class structures only through `RecoverClassesFromRTTIScript` (script tier, not
auto-analysis) and IDA through its GCC RTTI reconstruction; neither was run for
this analysis, so no side-by-side is claimed.

## Owning phase

P1 external-refinement (`itaniumrtti`). The types are facts produced at load and
committed like `swifttypes`' layouts; P5 type propagation and P9 emission are used
as they are.

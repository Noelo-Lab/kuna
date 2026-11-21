# itaniumclasses: size, fields and `this` for Itanium classes on ELF

`itaniumrtti` names classes, vtables and slot functions on a stripped GCC/Clang
C++ ELF. #1756 adds the vtable types under `itaniumrtti on`: a `C_vtbl` per
class, a class structure `C` whose only field is `C_vtbl *vptr` (variable-length,
no size), and slot prototypes where the targets' known prototypes agree. #1810
builds on that for IOKit: a class of known size, an unlocked `C *this` hint on
its members, constructors and destructors, and a typed vtable object.

What neither does is type anything on a stripped ELF. #1756 types no `this`
there, so its vtable types reach a call site only where a symbol already gave
the receiver a class type. #1810 gets the size and the fields from IOKit's
`OSMetaClass` registration and `kalloc_type` signature, which an ELF does not
have. This proposal is the ELF half: find the size and the fields of an Itanium
class from the code, and commit the `this` hint and the vtable object that #1810
commits for IOKit.

It is a draft because it depends on #1756 and #1810, which are not merged, and
because a typed receiver changes every C++ function's output.

## Before and after

`kuna decompile itaniumrtti_x86_64.so probe_shapes` on main:

```c
  long v3 [3]; // rax | stack - 0x38

  sub_14ea(v3,2,5);
  v1 = (**(int (**)(void *,unsigned int))(v3[0] + 0x10))(v3,a0);
  v2 = (**(int (**)(void *))(v3[0] + 0x18))(v3);
```

and the slot function:

```c
int shapes::Shape::vfunc_2(void *a0,int a1)
{
  return ((int *)a0)[2] * a1;
}
```

Target, reproduced on main by declaring the types by hand (`analysis.md`). The
field and slot names will be the ones #1756 gives (`vptr`, `vfn_<i>`):

```c
  Circle v3; // rax | stack - 0x38

  shapes::Circle::Circle(&v3,2,5);
  v1 = (*(int (*)(void *,unsigned int))(v3.base.vptr)->vfunc_2)(&v3,a0);
  v2 = (*(int (*)(void *))(v3.base.vptr)->vfunc_3)(&v3);
```

```c
int shapes::Shape::vfunc_2(Shape *this,int a1)
{
  return this->field_0x8 * a1;
}
```

Not yet measured: the output of `probe_shapes` on the #1756 branch itself. The
fixture is a shared object, and #1756 leaves out shared objects whose vtable
slots are symbolic relocations; whether that applies here decides how much of
the target #1756 already reaches.

## Option surface

None. This refines `itaniumrtti on`, so it is part of `on` (`docs/agents.md`: a
refinement extends the option it refines). `names` and `off` are unchanged.

## What is added

1. **The size of a class**, from the code:
   - the constant in `operator delete(this, N)` at the end of the class's
     deleting destructor (sized deallocation, the GCC and Clang default from
     C++14);
   - failing that, `operator new(N)` whose result reaches one of the class's
     constructors;
   - in both cases `N` must cover every constant offset loaded or stored through
     `this` in the class's own slot functions, constructors and destructors. A
     conflict means the evidence is not about this class, and the size stays
     unknown.

   The size goes into the instance-size field #1810 adds to `VtableClass`, so
   the install lays the class out to that size as it does for an IOKit class.
2. **The fields**, from the same accesses: offset and width, one field per
   distinct access, gaps padded, laid out around the vptrs and the embedded
   bases #1756 places. An access that straddles a base's bytes or a vptr gives
   no field.
3. **The receiver**: an unlocked `this` hint of type `C *` on each slot function,
   for the class whose vtable holds it first (the defining class), and on
   constructors and destructors where #1606 names them. The register is the
   ABI's first integer parameter (`RDI` on x86-64, `x0` on AArch64), not a fixed
   `x0` as in #1810's IOKit commit. Only for a class with a size; see question 1.
4. **The vtable objects**: the slots from a constructor's vptr store become a data
   object `C_vtable` of type `C_vtbl`, as #1810 does for IOKit, so the store
   prints `this->vptr = &C_vtable` instead of a cast.

Items 3 and 4 are what #1810's `kuna-console/src/kuna_iokittypes.rs` commits for
IOKit. The proposal is to move that commit into one place that both producers
feed, not to write a second copy.

## Refusals

- A function that sits in the vtables of classes with no common base (identical
  code folding, `__cxa_pure_virtual`, a shared trivial destructor) gets no hint.
- A thunk in a secondary sub-vtable receives a pointer to the base subobject, not
  to `C`; it gets no hint.
- A class with a virtual base keeps what #1756 gives it (#1756 already leaves the
  virtual base out); it gets no size and no hint.
- A function a symbol table or DWARF already types keeps that, as does a class
  DWARF already laid out (#1756 leaves those alone).

## Not in this feature

- Attributing non-virtual methods by following `this` through call sites.
- The MSVC `rtti` pass. The same evidence applies; it is a separate PR.

## Module and docs

New module `kuna-analysis/src/analyzers/rtti/kuna_itaniumclasses.rs`: walks the
functions #1756's recovery already attributes to a class, collects the
`operator delete`/`operator new` sizes and the accesses through `this`, and fills
`VtableClass`'s size and fields. The walk is the one #1606 uses to find vptr
stores (p-code along the fall-through path, first parameter as `this`), bounded
per function. Spec prose in `docs/spec/01-program-prep.md` beside `itaniumrtti`,
the `itaniumrtti` row in `phases.toml`.

## Acceptance

- `tests/stages/kuna-itaniumclasses.xml`, two passes on the existing fixture:
  `names` shows the vptr arithmetic and the `void *` receiver, `on` shows the
  named slot and `this->field_0x8`. A class without size evidence and a folded
  slot function are the negatives.
- An oracle against unstripped, DWARF-carrying twins of the same sources: every
  emitted size equals DWARF's `DW_AT_byte_size`, every field offset is a member
  offset or lies inside one, and no hint names a class the function is not a
  member of. Zero wrong is the bar; unknown is acceptable.
- Every function whose output changes is reviewed, on the in-repo C++ fixtures and
  on `libapt-pkg` and `libicuuc`, because a typed receiver propagates into
  callers' locals.
- The four gates, with no existing stage assertion moved (the work is under `on`,
  which is opt-in).
- Load time and `decompile-all` time on the same two libraries against the parent
  commit.

## Questions for review

1. A class without size evidence keeps #1756's variable-length structure. Should
   its functions still get the `this` hint? `analysis.md` shows a too-small fixed
   struct printing worse than none; whether a variable-length one avoids that has
   not been measured, so the proposal withholds the hint until it is.
2. Is moving #1810's hint and vtable-object commit into a shared place acceptable,
   or should the ELF commit stand on its own?
3. Fields from the observed accesses, or fixed 8-byte fields as the IOKit layout
   uses where no signature names the bytes?

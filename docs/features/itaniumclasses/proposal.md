# itaniumclasses: class and vtable types from the Itanium RTTI graph

`itaniumrtti` names classes, vtables and slot functions on a stripped GCC/Clang
C++ ELF but emits no type, so a virtual call prints as vptr arithmetic and a
method's receiver is `void *`. The engine already renders both once the types
exist (`analysis.md`). This proposes that the pass emit them.

This is a draft because it spans a producer (the RTTI pass), a new kind of evidence
(layout from `this` accesses), the commit boundary and every C++ function's output.

## Before and after

`kuna decompile itaniumrtti_x86_64.so probe_shapes`, main:

```c
  long v3 [3]; // rax | stack - 0x38

  sub_14ea(v3,2,5);
  v1 = (**(int (**)(void *,unsigned int))(v3[0] + 0x10))(v3,a0);
  v2 = (**(int (**)(void *))(v3[0] + 0x18))(v3);
```

Target, reproduced today by declaring the types by hand:

```c
  Circle v3; // rax | stack - 0x38

  shapes::Circle::Circle(&v3,2,5);
  v1 = (*(int (*)(void *,unsigned int))(v3.vptr)->vfunc_2)(&v3,a0);
  v2 = (*(int (*)(void *))(v3.vptr)->vfunc_3)(&v3);
```

and `int shapes::Shape::vfunc_2(void *a0,int a1) { return ((int *)a0)[2] * a1; }`
becomes `int shapes::Shape::vfunc_2(Shape *this,int a1) { return this->field_0x8 * a1; }`.

## Option surface

No new option. `itaniumrtti` gains a value: `on|names|off`. `names` is today's
behaviour; `on` adds the types. Whether `on` stays the default is decided by the
evidence in *Acceptance*; until then the types ship under an explicit value and the
default keeps meaning `names`.

## What is emitted

1. **A vtable type per class**, `<C>_vtbl`: one field per slot of the primary
   sub-vtable, named as the slot function is (`vfunc_<i>`), typed `code *`. A
   secondary sub-vtable gets `<C>_vtbl_for_<Base>`. The slot signature is left
   open on purpose: a declared signature is taken as fact at the call site, and a
   guessed arity drops arguments (`analysis.md`, first table). With `code *` the
   call goes through the named field and keeps the signature the call site
   recovers.
2. **A class type**, `<C>`: `vptr` at offset 0 typed `<C>_vtbl *`,
   `vptr_for_<Base>` at each non-primary base offset the `__vmi_class_type_info`
   base list states, and `field_0x<off>` for the other bytes. The layout is flat:
   a derived class repeats its bases' fields at their offsets instead of embedding
   a base struct, so a missing base size cannot shift anything.
3. **The receiver**: `this` of type `<C> *` on each slot function, for the class
   that defines the slot, and on constructors and destructors where #1606 names
   them. It is an unlocked input hint (`kuna_inputhints`), not a prototype: the
   arity and the other parameters stay recovered.
4. **The vtable objects** become typed data of their `<C>_vtbl` type. With only
   the pointer typed, a constructor's store prints
   `this->vptr = (Circle_vtbl *)Circle_vptr`; typing the object is meant to remove
   that cast, which the implementation has to confirm.

## Layout evidence, and when no class type is emitted

RTTI states no size and no data members. A too-small struct is worse than none
(`*(int *)&this[1].vptr` instead of `((int *)a0)[2]`), so a class type is emitted
only when its size is known:

- the constant in `operator delete(this, N)` at the end of the deleting destructor;
- failing that, `operator new(N)` whose result reaches one of the class's
  constructors;
- and in both cases `N` must cover every constant offset loaded or stored through
  `this` in the class's own slot functions, constructors and destructors. A
  conflict means the evidence is not about this class, and it gets no class type.

Fields come from those same accesses: offset and width, one field per distinct
access, gaps padded. A class with no size evidence keeps its `<C>_vtbl` (which
needs none) and its functions keep today's receivers.

The walk that collects the accesses is the one #1606 uses to find vptr stores:
p-code along the fall-through path, first parameter as `this`. It is bounded per
function and runs only on functions already attributed to a class.

## Refusals

- A function that sits in the vtables of classes with no common base (identical
  code folding, `__cxa_pure_virtual`, a shared trivial destructor) gets no hint.
- A thunk in a secondary sub-vtable receives a pointer to the base subobject, not
  to `C`; it gets no hint.
- A class with a virtual base: its vtable carries vbase offsets and its layout is
  not fixed by the static type. It gets `<C>_vtbl` only.
- A function a symbol table or DWARF already types keeps that.

## Not in this feature

- Slot signatures from the slot targets' own prototypes. This is the obvious next
  step and needs cross-function information; `code *` is its safe floor.
- Attributing non-virtual methods by following `this` through call sites.
- Embedding base structs.
- The MSVC `rtti` pass. The same design applies; it is a separate PR.

## Module and docs

New module `kuna-analysis/src/analyzers/rtti/kuna_itaniumclasses.rs`, fed by the
graph `kuna_itaniumrtti` already recovers; facts added to `AnalysisOutput` beside
`swift_types` and committed the same way. Spec prose in
`docs/spec/01-program-prep.md` next to the `itaniumrtti` section, the option row
in `phases.toml`, generated catalog and `docs/options.md`.

## Acceptance

- `tests/stages/kuna-itaniumclasses.xml`, two passes on the existing fixture:
  `names` shows the vptr arithmetic and the `void *` receiver, the typed value
  shows `vptr->vfunc_2` and `this->field_0x8`. A class without size evidence and a
  folded slot function are the negatives.
- An oracle against unstripped, DWARF-carrying twins of the same sources: every
  emitted class size equals DWARF's `DW_AT_byte_size`, every emitted field offset
  is a member offset or lies inside one, and no receiver hint names a class the
  function is not a member of. Zero wrong is the bar; unknown is acceptable.
- Every function whose output changes is reviewed, on the in-repo C++ fixtures and
  on two distribution libraries (`libapt-pkg`, `libicuuc`), because a typed
  receiver propagates into callers' locals.
- The four gates, with no existing stage assertion moved while the default means
  `names`.
- Load time and `decompile-all` time on the same two libraries against the parent
  commit.

## Questions for review

1. Is a third value on `itaniumrtti` the right surface, or should the types be
   unconditional once the oracle shows zero wrong?
2. `code *` slots now and signatures in a follow-up: acceptable as a first step?
3. Flat class layouts, or is base embedding wanted from the start?

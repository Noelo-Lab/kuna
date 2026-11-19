#!/usr/bin/env python3
"""Build iokitcxx_kext.macho: a tiny arm64 MH_KEXT_BUNDLE for the typed
classes of `iokitclasses on`.

One __mod_init_func initializer registers Base (0x18 bytes) through an
unnamed OSMetaClass constructor and Derived (0x30 bytes, super Base) through
a second unnamed overload (the one a class with its own zone uses), and
stores each X::MetaClass vtable. Vtables (in __DATA_CONST,__const, behind
two zero words):

    Base     [Base_D1, Base_D0, Base_getMetaClass, Base::release, Base::m2, Base::twice]
    Derived  [Derived_D1, Derived_D0, Derived_getMetaClass, Base::release, Derived_m2, Base::twice]

A __kalloc_type view for site.Derived (0x30 bytes, signature 121112).

    Derived_m2       ldr x0, [x0, #0x10]; ret          (unnamed; the override)
    Derived::Derived adrp/add x8, &Derived vtable + 16; str x8, [x0];
                     mov x9, #1; str x9, [x0, #8]; ret
    Base::twice      ldr x8, [x0]; ldr x8, [x8, #0x20]; mov w1, #7; blr x8  (a call of slot 4)
    make_derived     adrp/add x0, &Derived::gMetaClass; bl Derived::MetaClass::alloc;
                     mov x9, #1; str x9, [x0, #8]
    leaf             mov w0, #1; ret

Symbols: Base::release, Base::m2 (const, so its `this` is certain), Base::twice, Derived::Derived, make_derived
and leaf. Pointers are plain absolute words.

Run from this directory: python3 gen_iokitcxx.py
"""
import struct

B = 0xFFFFFFF007B00000
TEXT, EXEC, CONST, DATA, LINKEDIT = 0x0, 0x4000, 0x5000, 0x6000, 0x7000
CSTRING = 0x3000
KALLOC_TYPE = CONST + 0x200

def u32(*v): return struct.pack("<" + "I" * len(v), *v)
def u64(*v): return struct.pack("<" + "Q" * len(v), *v)

def adrp(rd, pc, target):
    delta = ((B + target) >> 12) - ((B + pc) >> 12)
    return 0x90000000 | ((delta & 3) << 29) | (((delta >> 2) & 0x7FFFF) << 5) | rd
def add_lo(rd, rn, target): return 0x91000000 | ((target & 0xFFF) << 10) | (rn << 5) | rd
def bl(pc, target): return 0x94000000 | (((target - pc) // 4) & 0x3FFFFFF)
def movw(rd, imm): return 0x52800000 | (imm << 5) | rd

RET = 0xD65F03C0
names = {"Base": CSTRING, "Derived": CSTRING + 5, "site": CSTRING + 13, "sig": CSTRING + 26}
BASE_META, DERIVED_META = DATA, DATA + 0x40

code, blob = {}, {}
pc = EXEC
def func(name, words):
    global pc
    code[name] = pc
    out = []
    for w in words:
        out.append(w(pc + 4 * len(out)) if callable(w) else w)
    blob[name] = out
    pc += 4 * len(out)
    pc = (pc + 15) & ~15

def addr_of(rd, target):
    return [lambda p, rd=rd, t=target: adrp(rd, p, t), add_lo(rd, rd, target)]

CONST_BASE = CONST + 0x100
vt = {"Base": CONST_BASE, "Derived": CONST_BASE + 0x40, "BaseMeta": CONST_BASE + 0x80, "DerivedMeta": CONST_BASE + 0xB8}

func("ctor", [RET])
func("ctor_zone", [RET])
init = [0xA9BF7BFD, 0xA9BF4FF4]
init += addr_of(19, BASE_META) + addr_of(1, names["Base"]) + [0xAA1F03E2, 0xAA1303E0, movw(3, 0x18)]
init += [lambda p: bl(p, code["ctor"])]
init += addr_of(16, vt["BaseMeta"] + 16) + [0xF9000270]
init += addr_of(19, DERIVED_META) + addr_of(1, names["Derived"]) + addr_of(2, BASE_META) + [0xAA1303E0, movw(3, 0x30)]
init += [0xAA1F03E4, lambda p: bl(p, code["ctor_zone"])]
init += addr_of(16, vt["DerivedMeta"] + 16) + [0xF9000270]
init += [0xA8C14FF4, 0xA8C17BFD, RET]
func("init", init)
func("Base_getMetaClass", addr_of(0, BASE_META) + [RET])
func("Derived_getMetaClass", addr_of(0, DERIVED_META) + [RET])
for n, v in [("Base_D1", 0), ("Base_D0", 0), ("Derived_D1", 0), ("Derived_D0", 0), ("Base_release", 1), ("Base_m2", 2),
             ("BaseMeta_D1", 0), ("BaseMeta_D0", 0), ("BaseMeta_alloc", 5),
             ("DerivedMeta_D1", 0), ("DerivedMeta_D0", 0), ("DerivedMeta_alloc", 6), ("Meta_shared", 7)]:
    func(n, [movw(0, v), RET])
func("Derived_m2", [0xF9400800, RET])
func("Derived_ctor", addr_of(8, vt["Derived"] + 16) + [0xF9000008, 0xD2800029, 0xF9000409, RET])
func("Base_twice", [0xA9BF7BFD, 0xF9400008, 0xF9401108, movw(1, 7), 0xD63F0100, 0xA8C17BFD, RET])
func("make_derived", [0xA9BF7BFD] + addr_of(0, DERIVED_META) + [lambda p: bl(p, code["DerivedMeta_alloc"])]
     + [0xD2800029, 0xF9000409, 0xA8C17BFD, RET])
func("leaf", [movw(0, 1), RET])
text_end = pc

c = code
vtables = {
    "Base": ["Base_D1", "Base_D0", "Base_getMetaClass", "Base_release", "Base_m2", "Base_twice"],
    "Derived": ["Derived_D1", "Derived_D0", "Derived_getMetaClass", "Base_release", "Derived_m2", "Base_twice"],
    "BaseMeta": ["BaseMeta_D1", "BaseMeta_D0", "Meta_shared", "BaseMeta_alloc"],
    "DerivedMeta": ["DerivedMeta_D1", "DerivedMeta_D0", "Meta_shared", "DerivedMeta_alloc"],
}

symbols = [(b"__ZN4Base7releaseEv", "Base_release", 0x0F, 2), (b"__ZNK4Base2m2Ev", "Base_m2", 0x0F, 2),
           (b"__ZN4Base5twiceEv", "Base_twice", 0x0F, 2), (b"__ZN7DerivedC2Ev", "Derived_ctor", 0x0F, 2),
           (b"_make_derived", "make_derived", 0x0F, 2), (b"_leaf", "leaf", 0x0F, 2)]
strtab = b"\0"
stroff = {}
for n, *_ in symbols:
    stroff[n] = len(strtab)
    strtab += n + b"\0"
strtab = strtab.ljust((len(strtab) + 7) & ~7, b"\0")
syms = b"".join(u32(stroff[n]) + bytes([t, sect]) + struct.pack("<H", 0) + u64(B + c[f])
                for n, f, t, sect in symbols)
symoff, str_off = LINKEDIT, LINKEDIT + len(syms)
linkedit = syms + strtab

def section(seg, name, addr, size, flags=0):
    return name.ljust(16, b"\0") + seg.ljust(16, b"\0") + u64(B + addr, size) + u32(addr, 0, 0, 0, flags, 0, 0, 0)
def segment(name, addr, size, sections=(), prot=5):
    body = name.ljust(16, b"\0") + u64(B + addr, size, addr, size) + u32(prot, prot, len(sections), 0) + b"".join(sections)
    return u32(0x19, 8 + len(body)) + body

cmds = [
    segment(b"__TEXT", TEXT, 0x4000, [section(b"__TEXT", b"__cstring", CSTRING, 33, 0x2)]),
    segment(b"__TEXT_EXEC", EXEC, 0x1000, [section(b"__TEXT_EXEC", b"__text", EXEC, text_end - EXEC, 0x80000400)]),
    segment(b"__DATA_CONST", CONST, 0x1000, [section(b"__DATA_CONST", b"__mod_init_func", CONST, 8, 0x9),
                                              section(b"__DATA_CONST", b"__const", CONST_BASE, 0x100),
                                              section(b"__DATA_CONST", b"__kalloc_type", KALLOC_TYPE, 0x40)], prot=1),
    segment(b"__DATA", DATA, 0x1000, [section(b"__DATA", b"__data", DATA, 0x80)], prot=3),
    segment(b"__LINKEDIT", LINKEDIT, len(linkedit), prot=1),
    u32(0x2, 24, symoff, len(symbols), str_off, len(strtab)),
    u32(0xB, 80, 0, 0, 0, len(symbols), len(symbols), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
]
image = bytearray(LINKEDIT + len(linkedit))
def place(off, data): image[off:off + len(data)] = data
place(0, u32(0xFEEDFACF, 0x0100000C, 0, 0xB, len(cmds), sum(map(len, cmds)), 0x85, 0) + b"".join(cmds))
place(CSTRING, b"Base\0Derived\0site.Derived\0" + b"121112\0")
place(KALLOC_TYPE + 0x10, u64(B + names["site"]))
place(KALLOC_TYPE + 0x20, u64(B + names["sig"]))
place(KALLOC_TYPE + 0x28, u32(0x61, 0x30))
for name, words in blob.items():
    place(c[name], u32(*words))
place(CONST, u64(B + c["init"]))
for k, slots in vtables.items():
    place(vt[k], u64(0, 0) + b"".join(u64(B + c[s]) for s in slots))
place(LINKEDIT, linkedit)
open("iokitcxx_kext.macho", "wb").write(image)
print({k: hex(B + v) for k, v in code.items()})

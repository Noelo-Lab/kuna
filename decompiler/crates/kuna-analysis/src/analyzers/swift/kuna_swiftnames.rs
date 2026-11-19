//! (kuna `swiftmetadata`) The names the Swift metadata pass derives from code
//! shape where a record alone gives only a position.
//!
//! - A class vtable getter/setter/modify triple whose bodies all address the
//!   same `self` field (`add x0, x20, #off` for the access check, or a
//!   direct load/store) is the accessor set of that stored property, named
//!   from the reflection field record at that offset
//!   (`Module.Class.width.getter`; a lazy property's `$__lazy_storage_$_`
//!   storage names the property itself). A computed property's body
//!   dispatches through the isa or calls first, so it never matches.
//! - A local protocol requirement takes the member name of what a class
//!   conformance's witness thunk dispatches to (a vtable slot named above,
//!   or a surviving symbol); otherwise its witnesses are named by kind
//!   (`requirement#2.getter`).
//! - Value witness functions are named from each type's value witness table
//!   (`Module.Token.destroy_value_witness`), a bitwise copy as the shared
//!   `__swift_memcpy<size>_<align>`, an enum's single callee of `destroy` /
//!   `initializeWithCopy` as its `outlined_consume` / `outlined_copy`.
//! - The compiler-emitted runtime helpers are matched by their fixed shape
//!   and the runtime function they call
//!   (`__swift_instantiateConcreteTypeFromMangledName`, the boxed opaque
//!   existential helpers).
//!
//! The code-shape readers decode A64 (every Apple platform Swift ships on);
//! on another architecture only the requirement kinds and surviving symbols
//! apply.

use std::collections::{BTreeMap, HashMap, HashSet};

use object::read::{Object, ObjectSection, ObjectSymbol};

use super::Namer;
use super::descriptors::{ContextKind, Metadata, MethodKind, Target};
use super::kuna_swifttypes::Types;
use crate::demangle::kuna_swift::demangle_swift_name;
use crate::objc::sections::MachoImage;

/// The `swiftself` register.
const SELF: u8 = 20;
/// The instructions a body is read for.
const SCAN: usize = 48;
/// The `$__lazy_storage_$_` prefix of a lazy property's storage field.
const LAZY: &str = "$__lazy_storage_$_";

/// The names derived for one image.
#[derive(Default, Debug)]
pub struct Names {
    /// `(class descriptor, vtable index)` → member (`width.getter`).
    pub slots: HashMap<(u64, usize), String>,
    /// `(protocol descriptor, requirement index)` → member.
    pub requirements: HashMap<(u64, usize), String>,
    /// Further functions: `(address, name, owner)`.
    pub functions: Vec<(u64, String, String)>,
}

/// The tag of a protocol requirement kind (`ProtocolRequirementFlags`).
pub fn requirement_tag(kind: u32) -> Option<&'static str> {
    Some(match kind {
        1 => "method",
        2 => "init",
        3 => "getter",
        4 => "setter",
        5 => "read",
        6 => "modify",
        8 => "associated_conformance",
        _ => return None,
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Insn {
    AddImm {
        rd: u8,
        rn: u8,
        imm: u64,
    },
    Mem {
        load: bool,
        vector: bool,
        rt: u8,
        rt2: Option<u8>,
        rn: u8,
        off: i64,
        width: u64,
        writeback: bool,
    },
    /// A register-offset load/store (`ldr xt, [xn, xm]`).
    MemReg {
        load: bool,
        vector: bool,
        rt: u8,
        rn: u8,
        rm: u8,
    },
    AddReg {
        rd: u8,
        rn: u8,
        rm: u8,
    },
    Adrp {
        rd: u8,
        page: u64,
    },
    Mov {
        rd: u8,
    },
    Bl(u64),
    B(u64),
    Br(u8),
    Blr(u8),
    Ret,
    Other,
}

fn sext(v: u32, bits: u32) -> i64 {
    let shift = 64 - bits;
    ((v as i64) << shift) >> shift
}

/// The few A64 forms the matchers read; everything else is `Other`.
fn decode(w: u32, pc: u64) -> Insn {
    let rt = (w & 31) as u8;
    let rn = ((w >> 5) & 31) as u8;
    if w & 0xff80_0000 == 0x9100_0000 {
        let imm = ((w >> 10) & 0xfff) as u64;
        let imm = if w & (1 << 22) != 0 { imm << 12 } else { imm };
        return Insn::AddImm { rd: rt, rn, imm };
    }
    if w & 0xffe0_ffe0 == 0xaa00_03e0 {
        return Insn::Mov { rd: rt };
    }
    if w & 0x9f00_0000 == 0x9000_0000 {
        let imm = (((w >> 5) & 0x7_ffff) << 2) | ((w >> 29) & 3);
        let page = (pc & !0xfff).wrapping_add((sext(imm, 21) * 4096) as u64);
        return Insn::Adrp { rd: rt, page };
    }
    if w & 0xff20_fc00 == 0x8b00_0000 {
        return Insn::AddReg {
            rd: rt,
            rn,
            rm: ((w >> 16) & 31) as u8,
        };
    }
    // Load/store register, register offset (unscaled `lsl #0`).
    if (w >> 27) & 7 == 7 && (w >> 24) & 3 == 0 && w & (1 << 21) != 0 && (w >> 10) & 3 == 2 {
        if (w >> 12) & 0xf != 0x6 {
            return Insn::Other;
        }
        let vector = w & (1 << 26) != 0;
        let opc = (w >> 22) & 3;
        let load = if vector { opc & 1 == 1 } else { opc != 0 };
        return Insn::MemReg {
            load,
            vector,
            rt,
            rn,
            rm: ((w >> 16) & 31) as u8,
        };
    }
    match w & 0xfc00_0000 {
        0x9400_0000 => return Insn::Bl(pc.wrapping_add((sext(w & 0x03ff_ffff, 26) * 4) as u64)),
        0x1400_0000 => return Insn::B(pc.wrapping_add((sext(w & 0x03ff_ffff, 26) * 4) as u64)),
        _ => {}
    }
    if w == 0xd65f_03c0 || w == 0xd65f_0bff || w == 0xd65f_0fff {
        return Insn::Ret;
    }
    match w & 0xffff_fc1f {
        0xd61f_0000 => return Insn::Br(rn),
        0xd63f_0000 => return Insn::Blr(rn),
        _ => {}
    }
    match w & 0xffff_f800 {
        0xd61f_0800 | 0xd71f_0800 => return Insn::Br(rn),
        0xd63f_0800 | 0xd73f_0800 => return Insn::Blr(rn),
        _ => {}
    }
    let size = w >> 30;
    let vector = w & (1 << 26) != 0;
    let opc = (w >> 22) & 3;
    // Load/store register, unsigned immediate offset.
    if (w >> 27) & 7 == 7 && (w >> 24) & 3 == 1 {
        let (load, width) = match (vector, opc) {
            (true, 2) if size == 0 => (false, 16),
            (true, 3) if size == 0 => (true, 16),
            (true, 0) => (false, 1u64 << size),
            (true, 1) => (true, 1u64 << size),
            (false, 0) => (false, 1u64 << size),
            (false, 1) => (true, 1u64 << size),
            (false, 2) | (false, 3) if size != 3 => (true, 1u64 << size),
            _ => return Insn::Other,
        };
        let off = ((w >> 10) & 0xfff) as i64 * width as i64;
        return Insn::Mem {
            load,
            vector,
            rt,
            rt2: None,
            rn,
            off,
            width,
            writeback: false,
        };
    }
    // Load/store register, unscaled / pre-index / post-index.
    if (w >> 27) & 7 == 7 && (w >> 24) & 3 == 0 && w & (1 << 21) == 0 {
        let idx = (w >> 10) & 3;
        if idx == 2 {
            return Insn::Other;
        }
        let (load, width) = match (vector, opc) {
            (true, 2) if size == 0 => (false, 16),
            (true, 3) if size == 0 => (true, 16),
            (true, 0) => (false, 1u64 << size),
            (true, 1) => (true, 1u64 << size),
            (false, 0) => (false, 1u64 << size),
            (false, 1) => (true, 1u64 << size),
            (false, 2) | (false, 3) if size != 3 => (true, 1u64 << size),
            _ => return Insn::Other,
        };
        let imm = sext((w >> 12) & 0x1ff, 9);
        return Insn::Mem {
            load,
            vector,
            rt,
            rt2: None,
            rn,
            off: if idx == 1 { 0 } else { imm },
            width,
            writeback: idx != 0,
        };
    }
    // Load/store pair.
    if (w >> 27) & 7 == 5 && w & (1 << 25) == 0 {
        let idx = (w >> 23) & 3;
        if idx == 0 {
            return Insn::Other;
        }
        let width = match (vector, size) {
            (false, 0) | (false, 1) => 4,
            (false, 2) => 8,
            (true, 0) => 4,
            (true, 1) => 8,
            (true, 2) => 16,
            _ => return Insn::Other,
        };
        let imm = sext((w >> 15) & 0x7f, 7) * width as i64;
        return Insn::Mem {
            load: w & (1 << 22) != 0,
            vector,
            rt,
            rt2: Some(((w >> 10) & 31) as u8),
            rn,
            off: if idx == 1 { 0 } else { imm },
            width: width * 2,
            writeback: idx != 2,
        };
    }
    Insn::Other
}

/// A branch or test-and-branch word with its displacement cleared, so a
/// fixed template matches wherever the code sits.
fn normalize(w: u32) -> u32 {
    if matches!(w & 0xfc00_0000, 0x9400_0000 | 0x1400_0000) {
        return w & 0xfc00_0000;
    }
    if w & 0x7e00_0000 == 0x3600_0000 {
        return w & 0xfff8_001f;
    }
    if w & 0x7e00_0000 == 0x3400_0000 || w & 0xff00_0010 == 0x5400_0000 {
        return w & 0xff00_001f;
    }
    w
}

/// A runtime helper's fixed arm64 body (branch displacements cleared), the
/// import each of its calls must reach, and its C name.
struct Template {
    words: &'static [u32],
    calls: &'static [&'static [&'static str]],
    name: &'static str,
}

const IN_CONTEXT: &[&str] = &[
    "swift_getTypeByMangledNameInContext",
    "swift_getTypeByMangledNameInContext2",
];

const TEMPLATES: &[Template] = &[
    // swiftc 6.x.
    Template {
        words: &[
            0xaa00_03e8, 0xf940_0000, 0xb7f8_0000, 0xd65f_03c0, 0xa9be_4ff4, 0xa901_7bfd,
            0x9100_43fd, 0xcb80_83e1, 0x8b20_c100, 0xaa08_03f3, 0xd280_0002, 0xd280_0003,
            0x9400_0000, 0xf900_0260, 0xa941_7bfd, 0xa8c2_4ff4, 0xd65f_03c0,
        ],
        calls: &[IN_CONTEXT],
        name: "__swift_instantiateConcreteTypeFromMangledName",
    },
    // swiftc 5.x.
    Template {
        words: &[
            0xa9be_4ff4, 0xa901_7bfd, 0x9100_43fd, 0xaa00_03f3, 0xf940_0000, 0xb7f8_0000,
            0xa941_7bfd, 0xa8c2_4ff4, 0xd65f_03c0, 0xcb80_83e1, 0x8b20_c260, 0xd280_0002,
            0xd280_0003, 0x9400_0000, 0xf900_0260, 0x1400_0000,
        ],
        calls: &[IN_CONTEXT],
        name: "__swift_instantiateConcreteTypeFromMangledName",
    },
    // The split form (cache in `x0`, mangled-name reference in `x1`).
    Template {
        words: &[
            0xa9be_4ff4, 0xa901_7bfd, 0x9100_43fd, 0xaa00_03f3, 0xf940_0000, 0x9240_0008,
            0xf100_001f, 0xfa40_1900, 0x5400_0000, 0xf940_0029, 0x9360_fd28, 0x8b29_c020,
            0xaa08_03e1, 0xd280_0002, 0xd280_0003, 0x9400_0000, 0xf900_0260, 0xa941_7bfd,
            0xa8c2_4ff4, 0xd65f_03c0,
        ],
        calls: &[IN_CONTEXT],
        name: "__swift_instantiateConcreteTypeFromMangledName",
    },
    Template {
        words: &[
            0xf85f_8028, 0xb940_5108, 0x3688_0008, 0xf940_0009, 0x9240_1d08, 0x9100_410a,
            0x8a28_0148, 0x8b08_0120, 0xd65f_03c0,
        ],
        calls: &[],
        name: "__swift_project_boxed_opaque_existential_1",
    },
    Template {
        words: &[
            0xa9bf_7bfd, 0x9100_03fd, 0xf85f_8028, 0xb940_5108, 0x3688_0008, 0x9240_1d02,
            0x9400_0000, 0xaa01_03e0, 0xa8c1_7bfd, 0xd65f_03c0,
        ],
        calls: &[&["swift_makeBoxUnique"]],
        name: "__swift_mutable_project_boxed_opaque_existential_1",
    },
    Template {
        words: &[
            0xf940_0c01, 0xf85f_8028, 0x3941_4909, 0x3708_0009, 0xf940_0502, 0xd61f_0040,
            0xf940_0000, 0x1400_0000,
        ],
        calls: &[&["swift_release"]],
        name: "__swift_destroy_boxed_opaque_existential_1",
    },
];

/// The value witness functions in table order, then an enum's three.
const VALUE_WITNESSES: [&str; 8] = [
    "initializeBufferWithCopyOfBuffer",
    "destroy",
    "initializeWithCopy",
    "assignWithCopy",
    "initializeWithTake",
    "assignWithTake",
    "getEnumTagSinglePayload",
    "storeEnumTagSinglePayload",
];
const ENUM_WITNESSES: [&str; 3] = ["getEnumTag", "destructiveProjectEnumData", "destructiveInjectEnumTag"];

pub struct Deriver<'a> {
    pub md: &'a Metadata,
    pub img: &'a MachoImage<'a>,
    pub namer: &'a Namer<'a>,
    pub types: &'a Types<'a>,
    pub imports: &'a HashMap<u64, String>,
    pub bind_slots: &'a HashSet<u64>,
    /// Real symbols the image keeps, by address.
    pub symbols: HashMap<u64, String>,
    /// The C-name prefix of the format (`_` on Mach-O).
    pub prefix: &'static str,
    /// Field descriptor address → its index in `md.fields`.
    fields: HashMap<u64, usize>,
}

impl<'a> Deriver<'a> {
    pub fn new(
        file: &object::File,
        md: &'a Metadata,
        img: &'a MachoImage<'a>,
        namer: &'a Namer<'a>,
        types: &'a Types<'a>,
        imports: &'a HashMap<u64, String>,
        bind_slots: &'a HashSet<u64>,
    ) -> Deriver<'a> {
        let mut symbols = HashMap::new();
        for s in file.symbols() {
            if s.kind() == object::SymbolKind::Text && s.is_definition() {
                if let Ok(n) = s.name() {
                    symbols.entry(s.address()).or_insert_with(|| n.to_string());
                }
            }
        }
        Deriver {
            md,
            img,
            namer,
            types,
            imports,
            bind_slots,
            symbols,
            prefix: if file.format() == object::BinaryFormat::MachO {
                "_"
            } else {
                ""
            },
            fields: md.fields.iter().enumerate().map(|(i, f)| (f.addr, i)).collect(),
        }
    }

    pub fn derive(&self, file: &object::File) -> Names {
        let mut names = Names::default();
        let mut classes: Vec<_> = self
            .md
            .contexts
            .values()
            .filter(|c| c.kind == ContextKind::Class)
            .collect();
        classes.sort_by_key(|c| c.addr);
        // The code-shape readers decode A64; another architecture keeps only
        // what the records and surviving symbols say.
        let arm64 = file.architecture() == object::Architecture::Aarch64;
        if arm64 {
            for ctx in &classes {
                self.properties(ctx, &mut names);
            }
        }
        self.requirements(&mut names, arm64);
        if arm64 {
            self.value_witnesses(&mut names);
            for sec in file.sections() {
                if matches!(sec.name(), Ok("__text") | Ok(".text")) {
                    if let Ok(data) = sec.data() {
                        self.helpers(sec.address(), data, &mut names);
                    }
                }
            }
        }
        names
    }

    fn word(&self, at: u64) -> Option<u32> {
        self.img.read_u32(at)
    }

    fn insns(&self, addr: u64, n: usize) -> Vec<(u64, Insn)> {
        let mut out = Vec::new();
        for k in 0..n as u64 {
            let pc = addr.wrapping_add(4 * k);
            let Some(w) = self.word(pc) else { break };
            out.push((pc, decode(w, pc)));
        }
        out
    }

    /// The import a call target's stub jumps through (`adrp x16; ldr x16;
    /// br x16`, or the arm64e `adrp x17; add x17; ldr x16, [x17]; braa`).
    fn stub_import(&self, at: u64) -> Option<&'a str> {
        let w0 = self.word(at)?;
        let page = |w: u32, pc: u64| -> u64 {
            let imm = (((w >> 5) & 0x7_ffff) << 2) | ((w >> 29) & 3);
            (pc & !0xfff).wrapping_add((sext(imm, 21) * 4096) as u64)
        };
        let w1 = self.word(at.wrapping_add(4))?;
        let slot = if w0 & 0x9f00_001f == 0x9000_0010 && w1 & 0xffc0_03ff == 0xf940_0210 {
            page(w0, at).wrapping_add(((w1 >> 10) & 0xfff) as u64 * 8)
        } else if w0 & 0x9f00_001f == 0x9000_0011 && w1 & 0xff80_03ff == 0x9100_0231 {
            page(w0, at).wrapping_add(((w1 >> 10) & 0xfff) as u64)
        } else {
            return None;
        };
        self.imports
            .get(&slot)
            .map(|n| n.strip_prefix('_').unwrap_or(n))
    }

    /// The `self` field accesses of the body at `f`, in order, until it
    /// returns, branches away (read straight through a `b` when
    /// `through_branches`) or overwrites `self`. An offset is an
    /// immediate, or the static value of a field offset global the body
    /// loads through `adrp; ldr` (a class whose layout another module
    /// fixes).
    fn self_uses(&self, f: u64, n: usize, through_branches: bool) -> Vec<Use> {
        let mut pages: [Option<u64>; 32] = [None; 32];
        let mut vals: [Option<u64>; 32] = [None; 32];
        let mut out = Vec::new();
        for (_, i) in self.insns(f, n) {
            match i {
                Insn::Mem {
                    rn: SELF,
                    off,
                    load,
                    writeback,
                    ..
                } => {
                    if off == 0 && load {
                        out.push(Use::Isa);
                    } else if off < 16 || writeback {
                        out.push(Use::End);
                        return out;
                    } else {
                        out.push(Use::Field { store: !load, off: off as u64 });
                    }
                }
                Insn::MemReg { rn: SELF, rm, load, .. } => match vals[rm as usize] {
                    Some(off) => out.push(Use::Field { store: !load, off }),
                    None => {
                        out.push(Use::End);
                        return out;
                    }
                },
                Insn::AddImm { rd, rn: SELF, imm } if rd != SELF && imm >= 16 => {
                    out.push(Use::Field { store: false, off: imm });
                }
                Insn::AddReg { rd, rn: SELF, rm } if rd != SELF => match vals[rm as usize] {
                    Some(off) => out.push(Use::Field { store: false, off }),
                    None => {
                        out.push(Use::End);
                        return out;
                    }
                },
                Insn::Bl(_) | Insn::Blr(_) => {
                    out.push(Use::Call);
                    pages[..19].fill(None);
                    vals[..19].fill(None);
                    continue;
                }
                Insn::B(_) if through_branches => {
                    out.push(Use::Call);
                    continue;
                }
                Insn::B(_) | Insn::Br(_) | Insn::Ret => {
                    out.push(Use::End);
                    return out;
                }
                _ => {}
            }
            // The registers the instruction writes.
            let (written, page, val): ([Option<u8>; 2], Option<u64>, Option<u64>) = match i {
                Insn::Adrp { rd, page } => ([Some(rd), None], Some(page), None),
                Insn::Mem {
                    load: true,
                    vector: false,
                    rt,
                    rt2,
                    rn,
                    off,
                    writeback,
                    ..
                } => {
                    let v = match (pages[rn as usize], rt2, writeback) {
                        (Some(p), None, false) if off >= 0 => self
                            .img
                            .read_ptr(p.wrapping_add(off as u64))
                            .filter(|v| (16..1 << 24).contains(v)),
                        _ => None,
                    };
                    ([Some(rt), rt2], None, v)
                }
                Insn::MemReg { load: true, vector: false, rt, .. } => ([Some(rt), None], None, None),
                Insn::AddImm { rd, .. } | Insn::AddReg { rd, .. } | Insn::Mov { rd } => {
                    ([Some(rd), None], None, None)
                }
                _ => ([None, None], None, None),
            };
            for r in written.into_iter().flatten() {
                if r == SELF {
                    out.push(Use::End);
                    return out;
                }
                if r < 32 {
                    pages[r as usize] = None;
                    vals[r as usize] = None;
                }
            }
            if let Some(r) = written[0] {
                if r < 31 {
                    pages[r as usize] = page;
                    vals[r as usize] = val;
                }
            }
        }
        out
    }

    /// The `self` field offset the body at `f` addresses first, before any
    /// call, branch or isa load.
    fn self_field(&self, f: u64) -> Option<u64> {
        match self.self_uses(f, SCAN, false).first()? {
            Use::Field { off, .. } => Some(*off),
            _ => None,
        }
    }

    /// Whether the body at `f` calls `callee` within its first instructions.
    fn calls(&self, f: u64, callee: u64) -> bool {
        for (_, i) in self.insns(f, SCAN) {
            match i {
                Insn::Bl(t) | Insn::B(t) if t == callee => return true,
                Insn::Ret | Insn::Br(_) => return false,
                _ => {}
            }
        }
        false
    }

    /// A class's own stored fields: `(offset, name, is var)`, in record
    /// order, read from its static metadata's field offset vector.
    fn class_fields(&self, ctx: &super::descriptors::Context) -> Option<Vec<(u64, String, bool)>> {
        let meta = *self.types.metadata.get(&ctx.addr)?;
        let fd = &self.md.fields[*self.fields.get(&ctx.fields?)?];
        if fd.records.len() != ctx.num_fields as usize {
            return None;
        }
        let vector = meta.checked_add(ctx.field_offset_vector as u64 * self.md.ptr_size)?;
        let mut out = Vec::new();
        for (i, r) in fd.records.iter().enumerate() {
            let off = self.img.read_ptr(vector.wrapping_add(self.md.ptr_size * i as u64))?;
            out.push((off, r.name.clone()?, r.flags & 2 != 0));
        }
        Some(out)
    }

    /// Whether the body at `f` stores to `self` + `off` before it returns
    /// (a lazy getter keeps the value it computed).
    fn stores_self(&self, f: u64, off: u64) -> bool {
        self.self_uses(f, 4 * SCAN, true)
            .contains(&Use::Field { store: true, off })
    }

    /// Name a class's stored-property accessors. A vtable getter/setter/modify
    /// run is one property's; the getter's first `self` access names the field.
    /// The setter and modify, when the optimizer kept them, must address the
    /// same field (a stored modify yields its address; a lazy one calls the
    /// getter), a lazy getter must store its storage back, each field is named
    /// once, and the matched getters must follow the field order.
    fn properties(&self, ctx: &super::descriptors::Context, names: &mut Names) {
        if ctx.vtable.len() < 3 {
            return;
        }
        let Some(fields) = self.class_fields(ctx) else {
            return;
        };
        let mut by_field: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for w in 0..ctx.vtable.len() - 2 {
            let (g, s, m) = (&ctx.vtable[w], &ctx.vtable[w + 1], &ctx.vtable[w + 2]);
            if (g.kind, s.kind, m.kind) != (MethodKind::Getter, MethodKind::Setter, MethodKind::Modify)
                || !(g.instance && s.instance && m.instance)
            {
                continue;
            }
            let Some(gi) = g.impl_addr else { continue };
            let Some(off) = self.self_field(gi) else {
                continue;
            };
            let at: Vec<usize> = (0..fields.len()).filter(|&k| fields[k].0 == off).collect();
            let [k] = at[..] else { continue };
            if !fields[k].2 {
                continue;
            }
            let lazy = fields[k].1.starts_with(LAZY);
            if lazy && !self.stores_self(gi, off) {
                continue;
            }
            if s.impl_addr.is_some_and(|si| self.self_field(si) != Some(off)) {
                continue;
            }
            if let Some(mi) = m.impl_addr {
                let yields = self.self_field(mi) == Some(off);
                if !(yields || lazy && self.calls(mi, gi)) {
                    continue;
                }
            }
            by_field.entry(k).or_default().push(w);
        }
        let matched: Vec<(usize, usize)> = by_field
            .into_iter()
            .filter_map(|(k, triples)| match triples[..] {
                [w] => Some((w, k)),
                _ => None,
            })
            .collect();
        let mut order = matched.clone();
        order.sort_unstable();
        if order.windows(2).any(|p| p[1].1 <= p[0].1) {
            return;
        }
        for (w, k) in matched {
            let name = &fields[k].1;
            let name = name.strip_prefix(LAZY).unwrap_or(name);
            if name.is_empty() {
                continue;
            }
            for (d, tag) in ["getter", "setter", "modify"].iter().enumerate() {
                names.slots.insert((ctx.addr, w + d), format!("{name}.{tag}"));
            }
        }
    }

    /// The vtable slot (or direct target) a class witness thunk forwards to.
    fn dispatch(&self, f: u64) -> Option<Dispatch> {
        let mut isa: Option<u8> = None;
        let mut slot: Option<(u8, u64)> = None;
        for (_, i) in self.insns(f, 24) {
            match i {
                Insn::Mem {
                    load: true,
                    vector: false,
                    rt: SELF,
                    rn: SELF,
                    off: 0,
                    rt2: None,
                    writeback: false,
                    ..
                } => {}
                Insn::Mem {
                    load: true,
                    vector: false,
                    rt,
                    rn: SELF,
                    off: 0,
                    rt2: None,
                    writeback: false,
                    ..
                } => isa = Some(rt),
                Insn::Mem {
                    load: true,
                    vector: false,
                    rt,
                    rn,
                    off,
                    rt2: None,
                    writeback: false,
                    ..
                } if Some(rn) == isa && off > 0 => slot = Some((rt, off as u64)),
                Insn::Blr(r) | Insn::Br(r) => {
                    return match slot {
                        Some((s, off)) if s == r => Some(Dispatch::Offset(off)),
                        _ => None,
                    };
                }
                Insn::Bl(t) | Insn::B(t) => {
                    return (isa.is_none() && self.stub_import(t).is_none()).then_some(Dispatch::Direct(t));
                }
                Insn::Ret => return None,
                Insn::AddImm { rd: SELF, .. } | Insn::Mov { rd: SELF } => return None,
                _ => {}
            }
        }
        None
    }

    /// The class `(descriptor, vtable index)` a metadata offset of the class
    /// at `ctx` names, through its locally laid-out superclasses.
    fn slot_at(&self, class: u64, off: u64, by_meta: &HashMap<u64, u64>) -> Option<(u64, usize)> {
        if self.md.ptr_size == 0 || !off.is_multiple_of(self.md.ptr_size) {
            return None;
        }
        let word = off / self.md.ptr_size;
        let mut at = class;
        for _ in 0..32 {
            let ctx = self.md.contexts.get(&at)?;
            let lo = ctx.vtable_offset as u64;
            if word >= lo && word - lo < ctx.vtable.len() as u64 {
                return Some((at, (word - lo) as usize));
            }
            let meta = *self.types.metadata.get(&at)?;
            let sup = self.img.read_ptr(meta.wrapping_add(self.md.ptr_size))?;
            at = *by_meta.get(&sup)?;
        }
        None
    }

    /// Name local protocol requirements from what their witnesses are: a
    /// surviving witness symbol's member, or the named vtable slot a class
    /// witness thunk forwards to. Every witness of a requirement must agree
    /// and match its kind; a settable property's modify follows its getter
    /// and setter.
    fn requirements(&self, names: &mut Names, arm64: bool) {
        let by_meta: HashMap<u64, u64> = self.types.metadata.iter().map(|(c, m)| (*m, *c)).collect();
        let mut impl_slot: HashMap<u64, Option<(u64, usize)>> = HashMap::new();
        for ctx in self.md.contexts.values() {
            for e in &ctx.vtable {
                if let Some(f) = e.impl_addr {
                    impl_slot
                        .entry(f)
                        .and_modify(|v| *v = None)
                        .or_insert(Some((ctx.addr, e.index)));
                }
            }
        }
        let mut found: BTreeMap<(u64, usize), HashSet<String>> = BTreeMap::new();
        for c in &self.md.conformances {
            let Target::Direct(p) = c.protocol else { continue };
            let Some(proto) = self.md.contexts.get(&p) else {
                continue;
            };
            let class = match c.type_ref {
                super::descriptors::TypeRef::Descriptor(Target::Direct(t)) => Some(t),
                _ => None,
            };
            for (k, f) in self.witnesses(c, proto) {
                let kind = self.md.requirement_kind(proto.requirements[k].0);
                let member = self.symbol_member(f).or_else(|| {
                    if !arm64 {
                        return None;
                    }
                    let class = class.filter(|t| {
                        self.md.contexts.get(t).map(|x| x.kind) == Some(ContextKind::Class)
                    })?;
                    let slot = match self.dispatch(f)? {
                        Dispatch::Offset(off) => self.slot_at(class, off, &by_meta)?,
                        Dispatch::Direct(t) => (*impl_slot.get(&t)?)?,
                    };
                    names.slots.get(&slot).cloned()
                });
                let Some(member) = member else { continue };
                if member_kind_agrees(&member, kind) {
                    found.entry((p, k)).or_default().insert(member);
                }
            }
        }
        for (key, members) in found {
            if members.len() == 1 {
                names.requirements.insert(key, members.into_iter().next().unwrap_or_default());
            }
        }
        // A settable property requirement is a getter, setter, modify run.
        let mut inferred = Vec::new();
        for (&(p, k), member) in &names.requirements {
            let Some(prop) = member.strip_suffix(".setter") else { continue };
            let Some(proto) = self.md.contexts.get(&p) else { continue };
            let Some(first) = k.checked_sub(1) else { continue };
            let kinds: Vec<u32> = (first..=k + 1)
                .filter_map(|i| proto.requirements.get(i))
                .map(|(r, _)| self.md.requirement_kind(*r))
                .collect();
            let getter = names.requirements.get(&(p, first));
            if kinds == [3, 4, 6] && getter.is_some_and(|g| g.strip_suffix(".getter") == Some(prop)) {
                inferred.push(((p, k + 1), format!("{prop}.modify")));
            }
        }
        for (key, member) in inferred {
            names.requirements.entry(key).or_insert(member);
        }
    }

    /// `(requirement index, witness function)` of a conformance to a local
    /// protocol: its resilient witnesses, or its plain witness table's words.
    fn witnesses(
        &self,
        c: &super::descriptors::Conformance,
        proto: &super::descriptors::Context,
    ) -> Vec<(usize, u64)> {
        let mut out = Vec::new();
        for (req, f) in &c.witnesses {
            if let (Target::Direct(r), Some(f)) = (req, f) {
                if let Some(k) = proto.requirements.iter().position(|(a, _)| a == r) {
                    out.push((k, *f));
                }
            }
        }
        if c.witnesses.is_empty() && c.flags & (1 << 17) == 0 {
            if let Some(w) = c.witness_table {
                for k in 0..proto.requirements.len() {
                    let slot = w.wrapping_add(self.md.ptr_size * (1 + k as u64));
                    if let Some(f) = self.md.words.get(&slot) {
                        out.push((k, *f));
                    }
                }
            }
        }
        out.retain(|(_, f)| self.namer.is_code(*f));
        out
    }

    /// The protocol member a surviving witness symbol names
    /// (`<T:P>.member` → `member`).
    fn symbol_member(&self, f: u64) -> Option<String> {
        let raw = self.symbols.get(&f)?;
        let name = demangle_swift_name(raw).or_else(|| demangle_swift_name(&format!("_{raw}")))?;
        let rest = name.strip_prefix('<')?;
        let mut depth = 1i32;
        for (i, ch) in rest.char_indices() {
            match ch {
                '<' => depth += 1,
                '>' => {
                    depth -= 1;
                    if depth == 0 {
                        return rest[i + 1..].strip_prefix('.').map(str::to_string);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn value_witnesses(&self, names: &mut Names) {
        let mut types: Vec<(&u64, &u64)> = self.types.metadata.iter().collect();
        types.sort();
        for (&addr, &meta) in types {
            let Some(ctx) = self.md.contexts.get(&addr) else {
                continue;
            };
            if !matches!(ctx.kind, ContextKind::Struct | ContextKind::Enum) {
                continue;
            }
            let Some(path) = self.namer.path(addr, 0) else {
                continue;
            };
            let Some(vwt) = self.types.local_witness_table(meta) else {
                continue;
            };
            let ps = self.md.ptr_size;
            let Some(size) = self.img.read_ptr(vwt.wrapping_add(8 * ps)) else {
                continue;
            };
            let Some(flags) = self.img.read_u32(vwt.wrapping_add(10 * ps)) else {
                continue;
            };
            let align = (flags & 0xff) as u64 + 1;
            let mut slots: Vec<(u64, &str)> = VALUE_WITNESSES
                .iter()
                .enumerate()
                .map(|(i, n)| (vwt.wrapping_add(ps * i as u64), *n))
                .collect();
            if ctx.kind == ContextKind::Enum {
                let base = vwt.wrapping_add(11 * ps);
                slots.extend(ENUM_WITNESSES.iter().enumerate().map(|(i, n)| (base.wrapping_add(ps * i as u64), *n)));
            }
            for (slot, witness) in slots {
                if self.bind_slots.contains(&slot) {
                    continue;
                }
                let Some(f) = self.img.read_ptr(slot) else { continue };
                if !self.namer.is_code(f) || self.stub_import(f).is_some() {
                    continue;
                }
                if matches!(witness, "initializeBufferWithCopyOfBuffer" | "initializeWithCopy" | "assignWithCopy" | "initializeWithTake" | "assignWithTake")
                    && self.pure_copy(f) == Some(size)
                {
                    let name = format!("{}__swift_memcpy{size}_{align}", self.prefix);
                    names.functions.push((f, name.clone(), name));
                    continue;
                }
                if witness == "destroy" && self.word(f) == Some(0xd65f_03c0) {
                    let name = format!("{}__swift_noop_void_return", self.prefix);
                    names.functions.push((f, name.clone(), name));
                    continue;
                }
                names.functions.push((f, format!("{path}.{witness}_value_witness"), path.clone()));
                if ctx.kind == ContextKind::Enum {
                    let outlined = match witness {
                        "destroy" => "outlined_consume",
                        "initializeWithCopy" => "outlined_copy",
                        _ => continue,
                    };
                    if let Some(callee) = self.single_local_callee(f) {
                        names.functions.push((callee, format!("{path}.{outlined}"), path.clone()));
                    }
                }
            }
        }
    }

    /// The byte count of a bitwise copy from `x1` to `x0` that returns `x0`:
    /// every instruction a load off `x1` or a store off `x0` of a register
    /// so loaded, the stores covering `[0, n)` without a gap.
    fn pure_copy(&self, f: u64) -> Option<u64> {
        let mut loaded: HashSet<(bool, u8)> = HashSet::new();
        let mut spans: Vec<(u64, u64)> = Vec::new();
        for (_, i) in self.insns(f, 64) {
            match i {
                Insn::Ret => {
                    spans.sort_unstable();
                    let mut end = 0u64;
                    for (lo, hi) in spans.iter().copied() {
                        if lo > end {
                            return None;
                        }
                        end = end.max(hi);
                    }
                    return (end > 0).then_some(end);
                }
                Insn::Mem {
                    load: true,
                    rn: 1,
                    rt,
                    rt2,
                    vector,
                    writeback: false,
                    ..
                } if vector || (rt > 1 && rt2.is_none_or(|r| r > 1)) => {
                    loaded.insert((vector, rt));
                    if let Some(r) = rt2 {
                        loaded.insert((vector, r));
                    }
                }
                Insn::Mem {
                    load: false,
                    rn: 0,
                    rt,
                    rt2,
                    vector,
                    off,
                    width,
                    writeback: false,
                } if off >= 0
                    && loaded.contains(&(vector, rt))
                    && rt2.is_none_or(|r| loaded.contains(&(vector, r))) =>
                {
                    spans.push((off as u64, (off as u64).saturating_add(width)));
                }
                _ => return None,
            }
        }
        None
    }

    /// The one local function the body at `f` calls or tail-calls, when it
    /// makes no other call.
    fn single_local_callee(&self, f: u64) -> Option<u64> {
        let mut callee = None;
        for (_, i) in self.insns(f, SCAN) {
            match i {
                Insn::Bl(t) | Insn::B(t) => {
                    if self.stub_import(t).is_some() || !self.namer.is_code(t) || callee.is_some() {
                        return None;
                    }
                    callee = Some(t);
                    if matches!(i, Insn::B(_)) {
                        return callee;
                    }
                }
                Insn::Blr(_) | Insn::Br(_) => return None,
                Insn::Ret => return callee,
                _ => {}
            }
        }
        None
    }

    /// Every place in a code section a runtime helper's template matches,
    /// with each call reaching the import the template names.
    fn helpers(&self, base: u64, data: &[u8], names: &mut Names) {
        let n = data.len() / 4;
        let word = |i: usize| u32::from_le_bytes([data[4 * i], data[4 * i + 1], data[4 * i + 2], data[4 * i + 3]]);
        for i in 0..n {
            let w = word(i);
            for t in TEMPLATES {
                if w != t.words[0] || i + t.words.len() > n {
                    continue;
                }
                if !t.words.iter().enumerate().all(|(k, tw)| *tw == normalize(word(i + k))) {
                    continue;
                }
                let at = base.wrapping_add(4 * i as u64);
                let mut calls = t.calls.iter();
                let ok = t.words.iter().enumerate().all(|(k, tw)| {
                    if !matches!(*tw, 0x9400_0000 | 0x1400_0000) {
                        return true;
                    }
                    let pc = at.wrapping_add(4 * k as u64);
                    let target = match decode(word(i + k), pc) {
                        Insn::Bl(t) | Insn::B(t) => t,
                        _ => return false,
                    };
                    match self.stub_import(target) {
                        Some(imp) => calls.next().is_some_and(|allowed| allowed.contains(&imp)),
                        // A branch inside the helper itself.
                        None => target.wrapping_sub(at) < 4 * t.words.len() as u64,
                    }
                }) && calls.next().is_none();
                if ok {
                    let name = format!("{}{}", self.prefix, t.name);
                    names.functions.push((at, name.clone(), name));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Use {
    Field { store: bool, off: u64 },
    Isa,
    Call,
    End,
}

enum Dispatch {
    Offset(u64),
    Direct(u64),
}

/// Whether a member name (`label.getter`, `scaled(by:)`) is of the
/// requirement kind.
fn member_kind_agrees(member: &str, kind: u32) -> bool {
    let tag = member.rsplit('.').next().unwrap_or("");
    match kind {
        1 => !matches!(tag, "getter" | "setter" | "modify" | "read") && member.contains('('),
        3 => tag == "getter",
        4 => tag == "setter",
        5 => tag == "read",
        6 => tag == "modify",
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_the_forms_the_matchers_read() {
        assert_eq!(decode(0x9100_4280, 0), Insn::AddImm { rd: 0, rn: 20, imm: 0x10 });
        assert!(matches!(
            decode(0xf940_e280, 0),
            Insn::Mem { load: true, rn: 20, off: 0x1c0, width: 8, .. }
        ));
        assert!(matches!(
            decode(0xa941_5293, 0),
            Insn::Mem { load: true, rn: 20, off: 0x10, rt: 19, rt2: Some(20), .. }
        ));
        assert!(matches!(decode(0x3cc0_9021, 0), Insn::Mem { load: true, rn: 1, off: 9, width: 16, .. }));
        assert_eq!(decode(0x9400_0010, 0x1000), Insn::Bl(0x1040));
        assert_eq!(decode(0x17ff_fff7, 0x1f90), Insn::B(0x1f6c));
        assert_eq!(decode(0xd63f_0100, 0), Insn::Blr(8));
        assert_eq!(decode(0xd65f_03c0, 0), Insn::Ret);
        assert_eq!(normalize(0x3688_00c8), 0x3688_0008);
        assert_eq!(normalize(0x9400_018e), 0x9400_0000);
    }

    #[test]
    fn member_kinds() {
        assert!(member_kind_agrees("width.getter", 3));
        assert!(!member_kind_agrees("width.getter", 4));
        assert!(member_kind_agrees("scaled(by:)", 1));
        assert!(!member_kind_agrees("width.getter", 1));
        assert_eq!(requirement_tag(3), Some("getter"));
        assert_eq!(requirement_tag(7), None);
    }
}

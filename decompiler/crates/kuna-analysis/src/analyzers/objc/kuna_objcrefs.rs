//! The ObjC reference slots and selector stubs: `__objc_selrefs`,
//! `__objc_classrefs`, `__objc_superrefs` and `__objc_stubs`.
//!
//! Code never touches a selector string or a `class_t` directly. A message send
//! loads the selector from a `__objc_selrefs` slot (`adrp/ldr x1`), a class
//! method call loads its receiver from a `__objc_classrefs` slot, `[super …]`
//! fills `objc_super.class` from a `__objc_superrefs` slot, and a modern link
//! (`-objc_stubs_fast`) routes most sends through a `__objc_stubs` entry that
//! loads the selref itself and jumps to `objc_msgSend`. Without names for those
//! four, every send reads `objc_msgSend(obj, dat_1045b76f0)` or `sub_10349a5c0(obj)`.
//!
//! Per slot this emits a pointer-sized data object: `selRef_<sel>` (colons
//! sanitized, the `sel_` convention of the selector string itself),
//! `classRef_<Class>` / `superRef_<Class>` (the class a rebase slot points at,
//! or the import a bind slot names, `_OBJC_CLASS_$_` stripped). Each stub
//! becomes the function `objc_msgSend$<sel>`, the name the linker gives the
//! symbol it synthesizes the stub for (IDA renders the same).
//!
//! The rebase slots are also reported as **constant** (the ranges [`emit`] returns):
//! dyld fills them before any code runs and nothing rewrites them afterwards,
//! which is why Ghidra's `ObjectiveC2_ClassAnalyzer` marks the three ref blocks
//! read-only. The commit paints those ranges foldable under `objcrefconst`, so a
//! send renders `objc_msgSend(obj, sel_init)` and `[super init]` shows its class.
//! A bind slot (an imported class) has no static value and is named only.
//!
//! Section-metadata reads only, no instruction decode beyond the two-instruction
//! stub shape (arm64 `adrp x1; ldr x1,[x1,#off]`, x86-64 `mov rsi,[rip+off]`).

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::fspec::{parameter_pieces_flags, ParameterPieces, PrototypePieces};
use object::read::{Object, ObjectSection};

use super::classt;
use super::sanitize_selector;
use super::sections::MachoImage;
use super::{encoding, TypeCtx};
use crate::pass::{AnalysisOutput, DataObjectFact, FidMatch, SymFact, SymKind};

const SEC_SELREFS: &str = "__objc_selrefs";
const SEC_CLASSREFS: &str = "__objc_classrefs";
const SEC_SUPERREFS: &str = "__objc_superrefs";
const SEC_STUBS: &str = "__objc_stubs";

/// The import symbol prefix of an Objective-C class (`_OBJC_CLASS_$_NSObject`).
const CLASS_SYMBOL_PREFIX: &str = "_OBJC_CLASS_$_";

/// What the method walk learned about this image's selectors, for the stubs.
#[derive(Default)]
pub struct Selectors {
    /// Per selector, the one type encoding every method with that selector
    /// agrees on; `None` once two methods disagree.
    encodings: HashMap<String, Option<String>>,
    /// Selector strings that already carry a `sel_` symbol.
    named: HashSet<u64>,
}

impl Selectors {
    /// Record one method's selector and type encoding.
    pub fn record(&mut self, selector: &str, types: Option<&str>) {
        self.encodings
            .entry(selector.to_string())
            .and_modify(|e| {
                if e.as_deref() != types {
                    *e = None;
                }
            })
            .or_insert_with(|| types.map(str::to_string));
    }

    /// The `sel_<sel>` data symbol on the selector string, once per string. The
    /// method walk names the selectors of this image's own methods; a sent-only
    /// selector (`alloc`, `init`) is named from its slot so a folded selref reads.
    pub fn name(&mut self, out: &mut AnalysisOutput, sel_va: u64, sel: &str) {
        if self.named.insert(sel_va) {
            out.symbols.push(SymFact {
                addr: sel_va,
                name: format!("sel_{}", sanitize_selector(sel)),
                kind: SymKind::Data,
            });
        }
    }
}

/// Name the reference slots and stubs of `file` into `out`, returning the
/// `[first, last_open)` ranges of the rebase slots (adjacent slots merged), the
/// ones whose contents are constant. A stub whose selector has one agreed
/// encoding in `selectors` gets the matching prototype.
pub(super) fn emit(
    img: &MachoImage,
    file: &object::File,
    tctx: &TypeCtx,
    selectors: &mut Selectors,
    out: &mut AnalysisOutput,
) -> Vec<(u64, u64)> {
    let mut const_slots = Vec::new();
    let ps = img.ptr_size();

    for va in section_slots(file, SEC_SELREFS, ps) {
        let Some(sel_va) = img.rebase_ptr(va) else { continue };
        let Some(sel) = img.read_cstr(sel_va).filter(|s| !s.is_empty()) else { continue };
        selectors.name(out, sel_va, &sel);
        out.data_objects.push(DataObjectFact {
            addr: va,
            name: format!("selRef_{}", sanitize_selector(&sel)),
            size: ps as u32,
        });
        push_slot(&mut const_slots, va, ps);
    }

    for (section, prefix) in [(SEC_CLASSREFS, "classRef_"), (SEC_SUPERREFS, "superRef_")] {
        for va in section_slots(file, section, ps) {
            let class = match img.rebase_ptr(va) {
                Some(class_va) => {
                    let name = classt::read_class_t(img, class_va)
                        .and_then(|c| classt::class_name(img, c.ro));
                    let Some(name) = name else { continue };
                    push_slot(&mut const_slots, va, ps);
                    name
                }
                None => match img.bind_name(va).and_then(|n| n.strip_prefix(CLASS_SYMBOL_PREFIX)) {
                    Some(name) => name.to_string(),
                    None => continue,
                },
            };
            out.data_objects.push(DataObjectFact {
                addr: va,
                name: format!("{prefix}{class}"),
                size: ps as u32,
            });
        }
    }

    if let Some(sec) = file.section_by_name(SEC_STUBS) {
        if let Ok(data) = sec.data() {
            for (stub_va, selref_va) in stubs(file.architecture(), sec.address(), data) {
                let Some(sel_va) = img.rebase_ptr(selref_va) else { continue };
                let Some(sel) = img.read_cstr(sel_va).filter(|s| !s.is_empty()) else { continue };
                selectors.name(out, sel_va, &sel);
                let name = format!("objc_msgSend${sel}");
                out.symbols.push(SymFact { addr: stub_va, name: name.clone(), kind: SymKind::Function });
                if let Some(enc) = selectors.encodings.get(&sel).and_then(|e| e.as_deref()) {
                    if let Some(pieces) = stub_prototype(&name, enc, file.architecture(), tctx) {
                        out.prototypes_at.push((stub_va, pieces));
                    }
                }
                out.fid_names.push(FidMatch { addr: stub_va, name });
            }
        }
    }
    const_slots
}

/// The address of every pointer-sized slot of the named section.
fn section_slots(file: &object::File, name: &str, ps: u64) -> Vec<u64> {
    let Some(sec) = file.section_by_name(name) else { return Vec::new() };
    (0..sec.size() / ps).map(|i| sec.address() + i * ps).collect()
}

/// Append the slot `[va, va + ps)`, merging with the previous range when adjacent.
fn push_slot(ranges: &mut Vec<(u64, u64)>, va: u64, ps: u64) {
    match ranges.last_mut() {
        Some((_, end)) if *end == va => *end = va + ps,
        _ => ranges.push((va, va + ps)),
    }
}

/// The prototype of a selector stub from its method's type encoding: the
/// method's signature without `_cmd`, every parameter pinned to the register
/// the full `(self, _cmd, …)` call places it in, so a caller renders
/// `objc_msgSend$greet:(obj, 5)` rather than passing a dead `x1` through. Only
/// register-class integer and pointer arguments (and on arm64 floating-point
/// ones) are pinned; a signature with a stack or aggregate argument gets no
/// prototype and keeps the untyped inference.
fn stub_prototype(
    name: &str,
    enc: &str,
    arch: object::Architecture,
    tctx: &TypeCtx,
) -> Option<PrototypePieces> {
    let (int_regs, float_regs): (&[&str], &[&str]) = match arch {
        object::Architecture::Aarch64 => (
            &["x0", "x2", "x3", "x4", "x5", "x6", "x7"],
            &["d0", "d1", "d2", "d3", "d4", "d5", "d6", "d7"],
        ),
        object::Architecture::X86_64 => (&["RDI", "RDX", "RCX", "R8", "R9"], &[]),
        _ => return None,
    };
    let method = encoding::decode_method(name, enc, tctx.types, tctx.ptr_size, tctx.word_size)?;
    if method.intypes.len() < 2 {
        return None;
    }
    let mut pieces = PrototypePieces {
        name: name.to_string(),
        outtype: method.outtype,
        first_var_arg_slot: -1,
        ..Default::default()
    };
    let (mut next_int, mut next_float) = (0usize, 0usize);
    for (i, (ty, pname)) in method.intypes.iter().zip(&method.innames).enumerate() {
        if i == 1 {
            continue;
        }
        let reg = match ty.get_metatype() {
            type_metatype::TYPE_FLOAT => {
                next_float += 1;
                float_regs.get(next_float - 1)?
            }
            _ if is_register_scalar(ty, tctx.ptr_size) => {
                next_int += 1;
                int_regs.get(next_int - 1)?
            }
            _ => return None,
        };
        let vn = tctx.arch.get_register_varnode(reg.as_bytes()).ok()?;
        let slot = pieces.intypes.len();
        pieces.intypes.push(Rc::clone(ty));
        pieces.innames.push(pname.clone());
        pieces.input_storage.push((
            slot as i32,
            ParameterPieces {
                addr: Address::new(vn.space?, vn.offset),
                type_: Some(Rc::clone(ty)),
                flags: parameter_pieces_flags::TYPELOCK | parameter_pieces_flags::NAMELOCK,
            },
        ));
    }
    Some(pieces)
}

/// Whether `ty` travels in one integer register: an integer, pointer, boolean or
/// code pointer of at most pointer size.
fn is_register_scalar(ty: &Datatype, ptr_size: i32) -> bool {
    ty.get_size() <= ptr_size
        && matches!(
            ty.get_metatype(),
            type_metatype::TYPE_INT
                | type_metatype::TYPE_UINT
                | type_metatype::TYPE_BOOL
                | type_metatype::TYPE_PTR
                | type_metatype::TYPE_CODE
                | type_metatype::TYPE_UNKNOWN
        )
}

/// Every `(stub_va, selref_slot_va)` in a `__objc_stubs` section, by scanning
/// for the selector-loading instruction pair at each possible stub start. Both
/// the 32-byte `-objc_stubs_fast` and the 12-byte `-objc_stubs_small` layouts
/// start with the same pair, so the stride need not be known.
pub fn stubs(arch: object::Architecture, base: u64, data: &[u8]) -> Vec<(u64, u64)> {
    match arch {
        object::Architecture::Aarch64 => stubs_arm64(base, data),
        object::Architecture::X86_64 => stubs_x86_64(base, data),
        _ => Vec::new(),
    }
}

/// arm64: `adrp x1, page` then `ldr x1, [x1, #off]`.
fn stubs_arm64(base: u64, data: &[u8]) -> Vec<(u64, u64)> {
    let mut out = Vec::new();
    let words: Vec<u32> = data.chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect();
    for i in 0..words.len().saturating_sub(1) {
        let (adrp, ldr) = (words[i], words[i + 1]);
        if adrp & 0x9F00_001F != 0x9000_0001 || ldr & 0xFFC0_03FF != 0xF940_0021 {
            continue;
        }
        let pc = base + 4 * i as u64;
        let immlo = (adrp >> 29) & 0x3;
        let immhi = (adrp >> 5) & 0x7FFFF;
        let imm = (((immhi << 2) | immlo) as i32) << 11 >> 11;
        let page = (pc & !0xFFF).wrapping_add((imm as i64 as u64) << 12);
        let off = ((ldr >> 10) & 0xFFF) as u64 * 8;
        out.push((pc, page.wrapping_add(off)));
    }
    out
}

/// x86-64: `mov rsi, [rip + off]` (`48 8b 35 rel32`) followed by a `jmp`.
fn stubs_x86_64(base: u64, data: &[u8]) -> Vec<(u64, u64)> {
    let mut out = Vec::new();
    let mut i = 0;
    while i + 8 <= data.len() {
        if data[i..i + 3] == [0x48, 0x8b, 0x35] && matches!(data[i + 7], 0xff | 0xe9) {
            let rel = i32::from_le_bytes([data[i + 3], data[i + 4], data[i + 5], data[i + 6]]);
            let next_ip = base + i as u64 + 7;
            out.push((base + i as u64, next_ip.wrapping_add(rel as i64 as u64)));
            i += 7;
        } else {
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The two-instruction pair of an `-objc_stubs_fast` entry at `0x1000008a0`
    /// (`adrp x1, 0x100008000; ldr x1, [x1, #0x210]`), followed by the
    /// `objc_msgSend` jump words that must not match, then a second stub.
    #[test]
    fn arm64_stub_pairs_resolve_their_selref_slot() {
        let words: [u32; 16] = [
            0x9000_0041, 0xF941_0821, 0x9000_0030, 0xF940_0210, 0xD61F_0200, 0xD420_0020, 0xD420_0020, 0xD420_0020,
            0x9000_0041, 0xF941_0C21, 0x9000_0030, 0xF940_0210, 0xD61F_0200, 0xD420_0020, 0xD420_0020, 0xD420_0020,
        ];
        let data: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
        assert_eq!(
            stubs(object::Architecture::Aarch64, 0x1000008a0, &data),
            vec![(0x1000008a0, 0x100008210), (0x1000008c0, 0x100008218)]
        );
    }

    #[test]
    fn x86_64_stub_pairs_resolve_their_selref_slot() {
        let mut data = vec![0x48, 0x8b, 0x35, 0x10, 0x00, 0x00, 0x00, 0xff, 0x25, 0, 0, 0, 0, 0xcc, 0xcc, 0xcc];
        data.extend_from_slice(&[0x48, 0x8b, 0x35, 0xf0, 0xff, 0xff, 0xff, 0xe9, 0, 0, 0, 0, 0xcc, 0xcc, 0xcc, 0xcc]);
        assert_eq!(
            stubs(object::Architecture::X86_64, 0x1000, &data),
            vec![(0x1000, 0x1017), (0x1010, 0x1007)]
        );
    }

    /// The whole walk over the `macho_objc_refs_arm64` fixture: six selref slots,
    /// one superref, two classrefs (one a bind), three stubs; the rebase slots of
    /// the three sections are the constant ranges, the bind slot is not.
    #[test]
    fn walks_macho_objc_refs_fixture() {
        use crate::loader::format::macho::resolve_chained_fixups;
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/macho_objc_refs_arm64");
        let bytes = std::fs::read(path).expect("read fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse Mach-O");
        let img = MachoImage::with_fixups(&file, resolve_chained_fixups(&file, &bytes));

        assert_eq!(img.rebase_ptr(0x1000081f0), Some(0x100008050), "superref → class_t Derived");
        assert_eq!(img.rebase_ptr(0x100008230), None, "a bind slot has no pointer");
        assert_eq!(img.bind_name(0x100008230), Some("_OBJC_CLASS_$_Remote"));

        let sec = file.section_by_name(SEC_STUBS).expect("__objc_stubs");
        let found = stubs(file.architecture(), sec.address(), sec.data().unwrap());
        assert_eq!(found.len(), 3);
        assert_eq!(found[0], (0x1000008c0, 0x100008210));
        let sel = img.read_cstr(img.rebase_ptr(found[0].1).unwrap()).unwrap();
        assert_eq!(sel, "count");

        let mut slots = Vec::new();
        for va in section_slots(&file, SEC_SELREFS, 8) {
            push_slot(&mut slots, va, 8);
        }
        assert_eq!(slots, vec![(0x1000081f8, 0x100008228)], "six adjacent selref slots merge");
    }

    #[test]
    fn adjacent_slots_merge() {
        let mut r = Vec::new();
        push_slot(&mut r, 0x100, 8);
        push_slot(&mut r, 0x108, 8);
        push_slot(&mut r, 0x118, 8);
        assert_eq!(r, vec![(0x100, 0x110), (0x118, 0x120)]);
    }
}

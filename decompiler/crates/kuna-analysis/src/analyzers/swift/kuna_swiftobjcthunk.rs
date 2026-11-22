//! P1 Swift vtable slot names from Objective-C entry points (`swiftobjcthunk`,
//! an arm of `swiftmetadata`).
//!
//! A Swift method descriptor carries no name, so a stripped class's vtable
//! slots print by position (`Module.Class.vtable#20.method`). An `@objc`
//! member also has an Objective-C entry point: the class's method list names
//! it by selector, and its body is a thunk that bridges the arguments and
//! calls the member's native implementation directly. Where the thunk calls
//! exactly one implementation the class's vtable or override table lists,
//! of a kind its selector can name, that slot takes a member name:
//!
//! - a getter or setter whose selector is an `@objc` property's accessor in
//!   the class's property list: `scrollView.getter`, `scrollView.setter`;
//! - a method or initializer: the selector itself,
//!   `addFrameWithTimestamp:label:` (Swift spells the method
//!   `addFrame(timestamp:label:)`, which the selector alone does not fix).
//!
//! An instance slot takes an instance method's selector, a class slot a class
//! method's, and an initializer an instance `init…` selector. A thunk that
//! sends an Objective-C message itself names nothing: it is an override of an
//! inherited Objective-C method with its body inlined (`super.layoutSubviews()`
//! and then a call), and such an override has no vtable slot, so the slot it
//! calls is some other method.
//!
//! An optimized thunk can hold its member's body inlined, and that body can
//! call another member of the class directly, so the slot a thunk calls is
//! only taken as its own where nothing points elsewhere: no own slot of a kind
//! the selector names lost its implementation (a member inlined everywhere,
//! its thunk included, leaves its slot empty), and no other implementation of the class
//! calls the hit directly (a member whose thunk inlined it but whose own body
//! survives calls what the thunk calls). An implementation that carries a
//! Swift symbol keeps the name the symbol gives it.
//!
//! The name belongs to the method descriptor, so a subclass's override of
//! the slot takes it too. A slot two thunks name differently stays unnamed.
//! arm64 only: the scan reads `bl`/`b` targets up to the next stated function
//! start (at most [`MAX_THUNK_INSNS`] instructions in a thunk).

use std::collections::{BTreeMap, HashMap, HashSet};

use object::{Object, ObjectSection, ObjectSymbol};

use super::descriptors::{Context, ContextKind, Metadata, MethodKind, Target};
use crate::objc::sections::MachoImage;
use crate::objc::{classt, kuna_objcprops, methods};

/// The longest thunk body scanned. A bridging thunk retains, converts and
/// releases each argument around one call; none comes near this.
pub const MAX_THUNK_INSNS: u64 = 128;

/// The longest implementation body scanned for its direct calls.
const MAX_BODY_INSNS: u64 = 1 << 16;

/// The Swift class metadata's `Description` word, after the five `class_t`
/// words and the Swift class fields (64-bit `ClassMetadata`).
const DESCRIPTION: u64 = 0x40;

/// `class_t.data` low bits that mark a Swift class (`FAST_IS_SWIFT_LEGACY`,
/// `FAST_IS_SWIFT_STABLE`).
const SWIFT_CLASS_BITS: u64 = 0x3;

/// One slot an implementation fills: its method descriptor, kind and
/// whether `self` is an instance.
#[derive(Clone, Copy)]
struct Slot {
    descriptor: u64,
    kind: MethodKind,
    instance: bool,
}

/// The member names [`members`] finds: by method descriptor, and the
/// implementations that keep their positional name because a Swift symbol
/// names them.
#[derive(Default)]
pub struct Members {
    names: HashMap<u64, String>,
    symbolized: HashSet<u64>,
}

impl Members {
    /// The member name of `descriptor` for its implementation at `imp`.
    pub fn get(&self, descriptor: u64, imp: u64) -> Option<&String> {
        if self.symbolized.contains(&imp) {
            return None;
        }
        self.names.get(&descriptor)
    }
}

/// The member name of every method descriptor an `@objc` thunk reaches,
/// keyed by the descriptor's address.
pub fn members(
    file: &object::File,
    bytes: &[u8],
    img: &MachoImage,
    md: &Metadata,
    imports: &HashMap<u64, String>,
) -> Members {
    if file.architecture() != object::Architecture::Aarch64 || img.ptr_size() != 8 {
        return Members::default();
    }
    let classes: Vec<Class> = img
        .classlists(file)
        .into_iter()
        .flat_map(|(addr, data)| (0..data.len() as u64 / 8).map(move |i| addr.wrapping_add(i * 8)))
        .filter_map(|slot| img.read_ptr(slot))
        .filter_map(|class_ptr| swift_class(file, img, md, class_ptr))
        .collect();
    if classes.is_empty() {
        return Members::default();
    }
    let mut starts = crate::analyzers::entry::macho_function_starts(file, bytes);
    for c in &classes {
        starts.extend(c.thunks.iter().map(|t| t.imp));
        starts.extend(c.slots.keys());
    }
    starts.sort_unstable();
    starts.dedup();
    let sends = Sends {
        img,
        imports,
        objc_stubs: section_range(file, "__objc_stubs"),
    };
    let mut found: BTreeMap<u64, Option<String>> = BTreeMap::new();
    for c in &classes {
        let called = called_by_slots(img, &starts, c);
        for t in &c.thunks {
            let Some((slot, member)) = thunk_member(&sends, &starts, c, t, &called) else {
                continue;
            };
            found
                .entry(slot)
                .and_modify(|m| {
                    if m.as_deref() != Some(member.as_str()) {
                        *m = None;
                    }
                })
                .or_insert(Some(member));
        }
    }
    let symbolized = file
        .symbols()
        .filter(|s| s.kind() == object::SymbolKind::Text && s.is_definition())
        .filter(|s| {
            s.name().is_ok_and(|n| {
                ["_$s", "$s", "_$S", "$S", "_T0"]
                    .iter()
                    .any(|p| n.starts_with(p))
            })
        })
        .map(|s| s.address())
        .collect();
    Members {
        names: found
            .into_iter()
            .filter_map(|(d, m)| Some((d, m?)))
            .collect(),
        symbolized,
    }
}

/// Every implementation another implementation of `c` calls or branches to
/// directly.
fn called_by_slots(img: &MachoImage, starts: &[u64], c: &Class) -> HashSet<u64> {
    let mut out = HashSet::new();
    for &f in c.slots.keys() {
        out.extend(
            call_targets(img, starts, f, MAX_BODY_INSNS)
                .into_iter()
                .filter(|&t| t != f && c.slots.contains_key(&t)),
        );
    }
    out
}

/// The slot `t` calls and the member its selector names there, when the
/// thunk reaches exactly one of the class's slots and sends no Objective-C
/// message itself. A thunk that sends one (`super.layoutSubviews()`) has the
/// body of an override of an inherited Objective-C method inlined, which has
/// no vtable slot: the slot it calls is some other method. Where an own slot
/// of a kind the selector names has no implementation, or another
/// implementation of the class calls the hit (`called`), the hit may be what
/// an inlined body calls, and names nothing.
fn thunk_member(
    sends: &Sends,
    starts: &[u64],
    c: &Class,
    t: &Thunk,
    called: &HashSet<u64>,
) -> Option<(u64, String)> {
    let (kinds, member) = own_member(t, &c.props);
    if c.empty.iter().any(|&(k, instance)| {
        kinds.contains(&k) && (k == MethodKind::Init || instance != t.class_method)
    }) {
        return None;
    }
    let mut hit: Option<Slot> = None;
    for target in call_targets(sends.img, starts, t.imp, MAX_THUNK_INSNS) {
        if sends.is_message_send(target) {
            return None;
        }
        let Some(s) = c.slots.get(&target) else {
            continue;
        };
        if called.contains(&target) || hit.is_some_and(|h| h.descriptor != s.descriptor) {
            return None;
        }
        hit = Some(*s);
    }
    let slot = hit?;
    let instance = slot.instance || slot.kind == MethodKind::Init;
    if instance == t.class_method || !kinds.contains(&slot.kind) {
        return None;
    }
    Some((slot.descriptor, member))
}

/// Where an image's calls send an Objective-C message: a selector stub in
/// `__objc_stubs`, or a stub that jumps through a slot bound to
/// `objc_msgSend*`.
struct Sends<'a> {
    img: &'a MachoImage<'a>,
    imports: &'a HashMap<u64, String>,
    objc_stubs: Option<(u64, u64)>,
}

impl Sends<'_> {
    fn is_message_send(&self, target: u64) -> bool {
        if self
            .objc_stubs
            .is_some_and(|(lo, hi)| (lo..hi).contains(&target))
        {
            return true;
        }
        stub_slot(self.img, target)
            .and_then(|slot| self.imports.get(&slot))
            .is_some_and(|name| name.trim_start_matches('_').starts_with("objc_msgSend"))
    }
}

/// The `[start, end)` of the section `name`.
fn section_range(file: &object::File, name: &str) -> Option<(u64, u64)> {
    let s = file.section_by_name(name)?;
    Some((s.address(), s.address().saturating_add(s.size())))
}

/// The pointer slot an arm64 stub at `addr` jumps through: `adrp x16` then
/// `ldr x16, [x16, #off]`, or arm64e's `adrp x17`, `add x17, x17, #off`,
/// `ldr x16, [x17]`.
fn stub_slot(img: &MachoImage, addr: u64) -> Option<u64> {
    let code = img.read(addr, 12)?;
    let word = |k: usize| u32::from_le_bytes([code[k], code[k + 1], code[k + 2], code[k + 3]]);
    let adrp = word(0);
    if adrp & 0x9f00_0000 != 0x9000_0000 {
        return None;
    }
    let imm = (((adrp >> 5) & 0x7_ffff) << 2) | ((adrp >> 29) & 3);
    let page = (addr & !0xfff).wrapping_add_signed((((imm << 11) as i32) >> 11) as i64 * 0x1000);
    let next = word(4);
    let off = (u64::from(next >> 10) & 0xfff)
        * if next & 0xffc0_0000 == 0xf940_0000 {
            8
        } else {
            1
        };
    match next & 0xffc0_0000 {
        0xf940_0000 | 0x9100_0000 => Some(page.wrapping_add(off)),
        _ => None,
    }
}

/// One Objective-C method of a class: its selector, IMP and whether it is a
/// class (`+`) method.
struct Thunk {
    selector: String,
    imp: u64,
    class_method: bool,
}

/// An `@objc` property: whether it is an instance property, its getter and
/// setter selectors and its name.
struct Property {
    instance: bool,
    getter: String,
    setter: Option<String>,
    name: String,
}

/// A Swift class with Objective-C metadata: its properties, methods and the
/// slot each of its implementations fills.
struct Class {
    props: Vec<Property>,
    thunks: Vec<Thunk>,
    slots: HashMap<u64, Slot>,
    /// The kind and instance-ness of each own slot with no implementation.
    empty: Vec<(MethodKind, bool)>,
}

/// The class at `class_ptr` when it is a Swift class whose metadata names a
/// local class descriptor.
fn swift_class(
    file: &object::File,
    img: &MachoImage,
    md: &Metadata,
    class_ptr: u64,
) -> Option<Class> {
    let data = img.read_ptr(class_ptr.wrapping_add(0x20))?;
    if data & SWIFT_CLASS_BITS == 0 {
        return None;
    }
    let ctx = img
        .read_ptr(class_ptr.wrapping_add(DESCRIPTION))
        .and_then(|d| md.contexts.get(&d))
        .filter(|c| c.kind == ContextKind::Class)?;
    let class = classt::read_class_t(img, class_ptr)?;
    let mut out = Class {
        props: Vec::new(),
        thunks: Vec::new(),
        slots: slots(md, ctx),
        empty: ctx
            .vtable
            .iter()
            .filter(|e| e.impl_addr.is_none())
            .map(|e| (e.kind, e.instance))
            .collect(),
    };
    let meta = classt::metaclass_ro(img, class.isa);
    for (ro, class_method) in [(Some(class.ro), false), (meta, true)] {
        let Some(ro) = ro else { continue };
        for (name, attrs) in classt::base_properties(img, ro)
            .map(|l| kuna_objcprops::properties(img, l))
            .unwrap_or_default()
        {
            let mut sels = kuna_objcprops::from_attributes(&name, &attrs)
                .into_iter()
                .map(|(s, _)| s);
            if let Some(getter) = sels.next() {
                out.props.push(Property {
                    instance: !class_method,
                    getter,
                    setter: sels.next(),
                    name,
                });
            }
        }
        for list in classt::method_lists(img, ro) {
            for m in methods::walk_method_list(img, list, file.architecture()) {
                if m.imp != 0 {
                    out.thunks.push(Thunk {
                        selector: m.selector,
                        imp: m.imp,
                        class_method,
                    });
                }
            }
        }
    }
    Some(out)
}

/// The slot each implementation of `ctx` fills: its own vtable entries and
/// the overrides of a local base class's entries.
fn slots(md: &Metadata, ctx: &Context) -> HashMap<u64, Slot> {
    let mut out = HashMap::new();
    for e in &ctx.vtable {
        if let Some(f) = e.impl_addr {
            out.insert(
                f,
                Slot {
                    descriptor: e.addr,
                    kind: e.kind,
                    instance: e.instance,
                },
            );
        }
    }
    for o in &ctx.overrides {
        let (Some(f), Target::Direct(m), Target::Direct(b)) = (o.impl_addr, o.method, o.base_class)
        else {
            continue;
        };
        let Some(e) = md
            .contexts
            .get(&b)
            .and_then(|base| base.vtable.iter().find(|e| e.addr == m))
        else {
            continue;
        };
        out.insert(
            f,
            Slot {
                descriptor: m,
                kind: e.kind,
                instance: e.instance,
            },
        );
    }
    out
}

/// The `bl` and `b` targets of the arm64 code at `imp`, up to the next
/// function start in `starts` or `max` instructions.
fn call_targets(img: &MachoImage, starts: &[u64], imp: u64, max: u64) -> Vec<u64> {
    let next = starts.partition_point(|&s| s <= imp);
    let end = starts.get(next).copied().unwrap_or(u64::MAX);
    let n = (end.saturating_sub(imp) / 4).min(max);
    let Some(code) = img.read(imp, (n * 4) as usize) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for (k, w) in code.chunks_exact(4).enumerate() {
        let insn = u32::from_le_bytes([w[0], w[1], w[2], w[3]]);
        if matches!(insn & 0xfc00_0000, 0x9400_0000 | 0x1400_0000) {
            let imm = (((insn & 0x03ff_ffff) << 6) as i32 >> 4) as i64;
            out.push(imp.wrapping_add(k as u64 * 4).wrapping_add_signed(imm));
        }
    }
    out
}

/// The slot kinds `thunk` can be the entry point of and the member name it
/// gives one: an accessor of a listed property, else a method or (an
/// instance `init…` selector) an initializer, by selector.
fn own_member(thunk: &Thunk, props: &[Property]) -> (&'static [MethodKind], String) {
    let sel = thunk.selector.as_str();
    let mine = || props.iter().filter(|p| p.instance != thunk.class_method);
    if let Some(p) = mine().find(|p| p.getter == sel) {
        return (&[MethodKind::Getter], format!("{}.getter", p.name));
    }
    if let Some(p) = mine().find(|p| p.setter.as_deref() == Some(sel)) {
        return (&[MethodKind::Setter], format!("{}.setter", p.name));
    }
    if sel.starts_with("init") && !thunk.class_method {
        return (&[MethodKind::Init, MethodKind::Method], sel.to_string());
    }
    (&[MethodKind::Method], sel.to_string())
}

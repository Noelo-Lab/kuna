//! **gotypes** — recover NAMED types from a Go binary's own runtime metadata
//! (`docs/multilang.md` W3, the Go provider, phase 1).
//!
//! A stripped Go binary carries no DWARF, but reflection and interface
//! dispatch need the type graph at runtime, so the linker bakes it in:
//! `runtime.firstmoduledata` holds `types`/`etypes` (the type blob's bounds)
//! and `typelinks` (offsets of the reachable `_type` records). This pass walks
//! that metadata and interns named struct shells — `main.Point { X int; ... }`
//! — into the type factory, plus the well-known header shells (`go.string`,
//! `go.slice`, `go.iface`, `go.eface`) with their `LangFacet` records, so the
//! W3 plane starts filling from the binary's own knowledge.
//!
//! ## Phase-1 scope (the rest is later consumers, not this pass)
//!
//! Recovered: package-qualified NAMED struct types (fields, verified offsets,
//! total size), with field types mapped to base widths, pointers, and the
//! header shells. Deliberately NOT phase 1: interface/itab method resolution,
//! generic instantiations beyond their raw name string, map/chan internals
//! beyond an opaque named shell, typed GLOBALS (no variable binding happens
//! here), and any caller-signature recovery. Named non-struct types (a
//! `type MyInt int`) are also skipped — a bare alias adds no layout knowledge.
//! 64-bit little-endian images only (every supported Go target in the corpus;
//! a 32-bit or BE image returns empty).
//!
//! ## Locating moduledata without symbols
//!
//! `moduledata.pcHeader` is its FIRST field and points at the pclntab header —
//! which the `pclntab` pass already locates on a stripped binary. So the
//! anchor scan reads each data section word-by-word for a pointer equal to the
//! header VMA, then validates the candidate in two steps:
//! 1. the next three words parse as the `funcnametab` slice (in-image pointer,
//!    `len == cap`, sane length) — true of every moduledata since go1.16;
//! 2. within the following window, the type metadata VALIDATES AGAINST
//!    ITSELF, in whichever of its two historical forms is present. Through
//!    go1.25: a `(types, etypes)` pair plus a `typelinks` slice
//!    (`len == cap`, sampled entries decode as plausible `_type` records
//!    inside `[types, etypes)`). From go1.26: the slice is gone — moduledata
//!    carries `types, typedesclen, etypes` and the records sit consecutively
//!    in `[types + 8, types + typedesclen)`, enumerated by advancing each
//!    record's own descriptor size (`internal/abi.Type::DescriptorSize`),
//!    accepted only when the walk decodes back to back. Field positions are
//!    NOT hardcoded — runtime field order drifts release to release — the
//!    window scan + cross-validation is what makes one implementation hold
//!    across go1.18..go1.27. go1.26+ also emits `firstmoduledata` into its
//!    own `.go.module` section, which the anchor scan covers.
//!
//! ## The `_type` record (`internal/abi.Type`, stable since go1.18)
//!
//! 48 bytes on 64-bit: `size @0`, `ptrbytes @8`, `hash @16`, `tflag @20`,
//! `align @21`, `fieldalign @22`, `kind @23`, `equal @24`, `gcdata @32`,
//! `nameOff(i32) @40`, `ptrToThis(i32) @44`. Names resolve at
//! `types + nameOff`: a flags byte, a varint length, then the bytes
//! (`tflagExtraStar` prefixes a `*` to strip). A struct type continues with
//! `pkgPath (ptr) @48`, `fields slice @56`: each field is
//! `{name (ptr), typ (ptr), offset (uintptr)}`, 24 bytes, since go1.19.
//!
//! ## Gating
//!
//! Registered only for a Go-detected image (the `gopclntab` precedent) and
//! env-bridged (`KUNA_GOTYPES`, default on) because interning happens at
//! `load file`, upstream of every `option` line — the `dwarfstructs`
//! situation exactly. Phase 1 is OUTPUT-NEUTRAL by construction: it adds
//! factory knowledge that nothing binds to a variable yet, so emitted C does
//! not change until a consumer (typed globals, interface dispatch) lands.

use std::rc::Rc;

use object::read::Object;

use kuna_base::types::int4;
use kuna_decomp::dtype::{flags as tflags, type_metatype, Datatype, TypeFactory, TypeField};
use kuna_decomp::kuna_gotypes::{gotypes_enabled, GOTYPES_MAX_TYPELINKS};
use kuna_decomp::kuna_langfacet::LangFacet;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};
use crate::pclntab::{locate_header, ImageBytes};

/// Go `kind` byte values (`internal/abi.Kind`), masked by [`KIND_MASK`].
const KIND_MASK: u8 = 0x1f;
const KIND_BOOL: u8 = 1;
const KIND_INT: u8 = 2;
const KIND_INT8: u8 = 3;
const KIND_INT16: u8 = 4;
const KIND_INT32: u8 = 5;
const KIND_INT64: u8 = 6;
const KIND_UINT: u8 = 7;
const KIND_UINT8: u8 = 8;
const KIND_UINT16: u8 = 9;
const KIND_UINT32: u8 = 10;
const KIND_UINT64: u8 = 11;
const KIND_UINTPTR: u8 = 12;
const KIND_FLOAT32: u8 = 13;
const KIND_FLOAT64: u8 = 14;
const KIND_ARRAY: u8 = 17;
const KIND_CHAN: u8 = 18;
const KIND_FUNC: u8 = 19;
const KIND_INTERFACE: u8 = 20;
const KIND_MAP: u8 = 21;
const KIND_POINTER: u8 = 22;
const KIND_SLICE: u8 = 23;
const KIND_STRING: u8 = 24;
const KIND_STRUCT: u8 = 25;
const KIND_UNSAFEPTR: u8 = 26;

/// `tflag` bits (`internal/abi.TFlag`).
const TFLAG_EXTRA_STAR: u8 = 0x2;
const TFLAG_NAMED: u8 = 0x4;

/// The `_type` base record size on a 64-bit target.
const TYPE_BASE: u64 = 48;

/// How far past the moduledata anchor the window scan looks, in 8-byte words.
/// The fields phase 1 needs (`types`/`etypes`/`typelinks`) have sat within the
/// first ~50 words of moduledata on every release since go1.16; 96 leaves room
/// for two more releases of growth without scanning unrelated globals.
const MODULEDATA_WINDOW_WORDS: u64 = 96;

/// One decoded `_type` record header.
#[derive(Clone, Debug)]
struct GoType {
    vma: u64,
    size: u64,
    kind: u8,
    tflag: u8,
    name: String,
}

/// The located metadata: the type blob bounds and the reachable `_type`
/// record offsets — from the `typelinks` slice (go1.18..go1.25) or from the
/// descriptor-region walk that replaced it (go1.26+).
struct TypeMeta {
    types: u64,
    etypes: u64,
    offsets: Vec<u32>,
}

/// Read a u64 word at `vma`.
fn word(img: &ImageBytes, vma: u64) -> Option<u64> {
    img.read(vma, 8).map(|b| u64::from_le_bytes(b.try_into().unwrap()))
}

/// `true` if `vma` reads as mapped image bytes.
fn mapped(img: &ImageBytes, vma: u64) -> bool {
    img.read(vma, 1).is_some()
}

/// Decode a Go name record at `vma` (go1.17+ encoding): a flags byte, a varint
/// length, then the UTF-8 bytes. Returns `None` on any malformation.
fn read_go_name(img: &ImageBytes, vma: u64) -> Option<String> {
    let _flags = img.read(vma, 1)?[0];
    let mut len: usize = 0;
    let mut shift = 0u32;
    let mut cur = vma + 1;
    for _ in 0..4 {
        let b = img.read(cur, 1)?[0];
        cur += 1;
        len |= ((b & 0x7f) as usize) << shift;
        if b & 0x80 == 0 {
            break;
        }
        shift += 7;
    }
    if len == 0 || len > 512 {
        return None;
    }
    let bytes = img.read(cur, len)?;
    std::str::from_utf8(bytes).ok().map(str::to_string)
}

/// Decode the `_type` base record at `types + off`. The name comes from the
/// `nameOff` at `+40`, resolved against the blob base, `tflagExtraStar`'s
/// leading `*` stripped.
fn read_type(img: &ImageBytes, types: u64, etypes: u64, off: u32) -> Option<GoType> {
    let vma = types.checked_add(off as u64)?;
    if vma + TYPE_BASE > etypes {
        return None;
    }
    let rec = img.read(vma, TYPE_BASE as usize)?;
    let size = u64::from_le_bytes(rec[0..8].try_into().unwrap());
    let tflag = rec[20];
    let kind = rec[23] & KIND_MASK;
    if kind == 0 || kind > KIND_UNSAFEPTR {
        return None;
    }
    // A type cannot be larger than the image; 1 GiB bounds a corrupt size word.
    if size > 0x4000_0000 {
        return None;
    }
    let name_off = i32::from_le_bytes(rec[40..44].try_into().unwrap());
    if name_off <= 0 {
        return None;
    }
    let name_vma = types.checked_add(name_off as u64)?;
    let mut name = read_go_name(img, name_vma)?;
    if tflag & TFLAG_EXTRA_STAR != 0 {
        name = name.strip_prefix('*').unwrap_or(&name).to_string();
    }
    Some(GoType { vma, size, kind, tflag, name })
}

/// `true` if the three words at `vma` parse as a plausible Go slice header
/// (`ptr` mapped, `len == cap`, `len <= max`).
fn plausible_slice(img: &ImageBytes, vma: u64, max_len: u64) -> Option<(u64, u64)> {
    let ptr = word(img, vma)?;
    let len = word(img, vma + 8)?;
    let cap = word(img, vma + 16)?;
    if len == 0 || len != cap || len > max_len || !mapped(img, ptr) {
        return None;
    }
    Some((ptr, len))
}

/// Locate `types`/`etypes` and the `typelinks` slice inside the moduledata
/// window at `anchor`, by cross-validation (see the module header). Samples up
/// to 16 typelink entries and requires every sampled one to decode.
fn locate_type_meta(img: &ImageBytes, anchor: u64) -> Option<TypeMeta> {
    // Candidate (types, etypes) pairs: adjacent words, both mapped, ordered,
    // blob no larger than 256 MiB.
    let mut pairs: Vec<(u64, u64, u64)> = Vec::new(); // (word_idx, types, etypes)
    for i in 0..MODULEDATA_WINDOW_WORDS {
        let a = match word(img, anchor + i * 8) {
            Some(v) => v,
            None => break,
        };
        let Some(b) = word(img, anchor + (i + 1) * 8) else { break };
        if a != 0 && a < b && b - a < 0x1000_0000 && mapped(img, a) && mapped(img, b - 1) {
            pairs.push((i, a, b));
        }
    }
    for &(i, types, etypes) in &pairs {
        // The typelinks slice follows (types, etypes) within the window.
        for j in (i + 2)..MODULEDATA_WINDOW_WORDS {
            let Some((tl_ptr, tl_len)) = plausible_slice(img, anchor + j * 8, 1_000_000) else {
                continue;
            };
            // Validate: sampled entries are int32 offsets into the blob whose
            // records decode.
            let n = tl_len as usize;
            let step = (n / 16).max(1);
            let mut ok = 0usize;
            let mut tried = 0usize;
            for k in (0..n).step_by(step).take(16) {
                tried += 1;
                let Some(entb) = img.read(tl_ptr + (k as u64) * 4, 4) else { break };
                let off = i32::from_le_bytes(entb.try_into().unwrap());
                if off >= 0 && read_type(img, types, etypes, off as u32).is_some() {
                    ok += 1;
                }
            }
            if tried >= 1 && ok == tried {
                let n = n.min(GOTYPES_MAX_TYPELINKS);
                let mut offsets = Vec::with_capacity(n);
                for k in 0..n {
                    let Some(entb) = img.read(tl_ptr + (k as u64) * 4, 4) else { break };
                    let off = i32::from_le_bytes(entb.try_into().unwrap());
                    if off >= 0 {
                        offsets.push(off as u32);
                    }
                }
                return Some(TypeMeta { types, etypes, offsets });
            }
        }
    }
    None
}

/// The go1.26+ `_type` descriptor stride: records sit consecutively in
/// `[types + 8, types + typedesclen)` and a walker advances by each record's
/// own descriptor size (`internal/abi.Type::DescriptorSize`, go1.27.2) — the
/// per-kind wrapper, an optional `UncommonType` (+ its methods), and the
/// kind's trailing arrays, in that memory order. `None` for a stride this
/// phase cannot trust; the caller stops the walk there (conservative: keep
/// what already decoded).
fn descriptor_stride(img: &ImageBytes, t: &GoType) -> Option<u64> {
    const UNCOMMON_SIZE: u64 = 16; // {PkgPath i32, Mcount u16, Xcount u16, Moff u32, _ u32}
    const METHOD_SIZE: u64 = 16; // 4 x i32
    const TFLAG_UNCOMMON: u8 = 0x1;
    let slice_len = |at: u64| -> Option<u64> {
        let len = word(img, at + 8)?;
        let cap = word(img, at + 16)?;
        (len == cap && len <= 1_000_000).then_some(len)
    };
    let (base, add): (u64, u64) = match t.kind {
        KIND_ARRAY => (TYPE_BASE + 24, 0),
        KIND_CHAN => (TYPE_BASE + 16, 0),
        KIND_FUNC => {
            // InCount u16 @ +48, OutCount u16 @ +50 (top bit = variadic).
            let b = img.read(t.vma + TYPE_BASE, 4)?;
            let inc = u16::from_le_bytes(b[0..2].try_into().unwrap()) as u64;
            let outc = (u16::from_le_bytes(b[2..4].try_into().unwrap()) & 0x7fff) as u64;
            if inc + outc > 512 {
                return None;
            }
            (TYPE_BASE + 8, (inc + outc) * 8)
        }
        KIND_INTERFACE => {
            // PkgPath Name (ptr) @ +48, Methods slice @ +56; Imethod = 8.
            (TYPE_BASE + 8 + 24, slice_len(t.vma + TYPE_BASE + 8)? * 8)
        }
        KIND_MAP => (TYPE_BASE + 88, 0), // Key,Elem,Group,Hasher,GroupSize,KeysOff,KeyStride,ElemsOff,ElemStride,ElemOff,Flags+pad
        KIND_POINTER | KIND_SLICE => (TYPE_BASE + 8, 0),
        KIND_STRUCT => {
            // PkgPath Name (ptr) @ +48, Fields slice @ +56; StructField = 24.
            (TYPE_BASE + 8 + 24, slice_len(t.vma + TYPE_BASE + 8)? * 24)
        }
        _ => (TYPE_BASE, 0), // plain kinds (bool..complex128, string, unsafeptr)
    };
    let uncommon = if t.tflag & TFLAG_UNCOMMON != 0 {
        // Mcount u16 sits at uncommon + 4.
        let b = img.read(t.vma + base + 4, 2)?;
        let mcount = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        if mcount > 4096 {
            return None;
        }
        UNCOMMON_SIZE + mcount * METHOD_SIZE
    } else {
        0
    };
    let stride = base + uncommon + add;
    (stride <= 1_048_576).then_some(stride)
}

/// Locate the go1.26+ type metadata: the moduledata triple
/// `types, typedesclen, etypes` (the middle word bounds the consecutive
/// descriptor region that replaced the `typelinks` slice), then enumerate the
/// records by walking that region with [`descriptor_stride`]. Cross-validated
/// the same way the slice form is: the triple's arithmetic must hold
/// (`types + typedesclen <= etypes`), both bounds must map, and the walk must
/// decode records back to back — a divergence stops it conservatively.
fn locate_type_meta_descriptors(img: &ImageBytes, anchor: u64) -> Option<TypeMeta> {
    for i in 0..MODULEDATA_WINDOW_WORDS {
        let Some(types) = word(img, anchor + i * 8) else { break };
        let Some(typedesclen) = word(img, anchor + (i + 1) * 8) else { break };
        let Some(etypes) = word(img, anchor + (i + 2) * 8) else { break };
        if types == 0
            || typedesclen == 0
            || typedesclen >= 0x1000_0000
            || types + typedesclen > etypes
            || etypes - types >= 0x1000_0000
            || !mapped(img, types)
            || !mapped(img, etypes - 1)
        {
            continue;
        }
        // Walk the descriptor region; require a run of decoding records.
        let end = types + typedesclen;
        let mut td = types + 8;
        let mut offsets: Vec<u32> = Vec::new();
        while td < end && offsets.len() < GOTYPES_MAX_TYPELINKS {
            td = (td + 7) & !7;
            if td >= end {
                break;
            }
            let off = (td - types) as u32;
            let Some(t) = read_type(img, types, etypes, off) else { break };
            let Some(stride) = descriptor_stride(img, &t) else { break };
            offsets.push(off);
            td += stride;
        }
        // Accept only a walk that looks like the real region: several records
        // and no early divergence (reached the end or the cap).
        if offsets.len() >= 8 && (td >= end || offsets.len() == GOTYPES_MAX_TYPELINKS) {
            return Some(TypeMeta { types, etypes, offsets });
        }
    }
    None
}

/// Find the moduledata anchor: a data-section word holding the pclntab header
/// VMA whose following three words parse as the `funcnametab` slice.
fn locate_moduledata(file: &object::File, img: &ImageBytes, header_vma: u64) -> Option<u64> {
    use object::ObjectSection;
    for sec in file.sections() {
        let name = sec.name().unwrap_or("");
        if !matches!(
            name,
            // go1.26+ linkers emit firstmoduledata into its own `.go.module`
            // section; older releases keep it in the data family.
            ".go.module" | "go.module" | "__go_module"
                | ".noptrdata" | ".data" | ".data.rel.ro" | "__noptrdata" | "__data" | "__DATA"
        ) {
            continue;
        }
        let base = sec.address();
        let Ok(data) = sec.data() else { continue };
        let words = data.len() / 8;
        for w in 0..words {
            let v = u64::from_le_bytes(data[w * 8..w * 8 + 8].try_into().unwrap());
            if v != header_vma {
                continue;
            }
            let anchor = base + (w as u64) * 8;
            // funcnametab slice directly follows pcHeader*.
            if plausible_slice(img, anchor + 8, 0x1000_0000).is_some() {
                return Some(anchor);
            }
        }
    }
    None
}

/// Map a field's `_type` to an interned kuna datatype. Named structs reference
/// their (possibly still incomplete) shells; scalars map to base widths; the
/// string/slice/interface kinds map to the well-known header shells; anything
/// else degrades to `undefined<size>` so the field's extent stays honest.
fn field_type(
    tf: &dyn TypeFactory,
    img: &ImageBytes,
    meta: &TypeMeta,
    type_vma: u64,
) -> Option<Rc<Datatype>> {
    if type_vma < meta.types || type_vma + TYPE_BASE > meta.etypes {
        return None;
    }
    let off = (type_vma - meta.types) as u32;
    let t = read_type(img, meta.types, meta.etypes, off)?;
    let m = |mt| tf.get_base(t.size as int4, mt).ok();
    match t.kind {
        KIND_BOOL => tf.get_base(1, type_metatype::TYPE_BOOL).ok(),
        KIND_INT | KIND_INT8 | KIND_INT16 | KIND_INT32 | KIND_INT64 => {
            m(type_metatype::TYPE_INT)
        }
        KIND_UINT | KIND_UINT8 | KIND_UINT16 | KIND_UINT32 | KIND_UINT64 | KIND_UINTPTR => {
            m(type_metatype::TYPE_UINT)
        }
        KIND_FLOAT32 | KIND_FLOAT64 => m(type_metatype::TYPE_FLOAT),
        KIND_POINTER => {
            // elem *Type sits at +48; a named struct pointee references its
            // shell, anything else is a void*.
            let pointee = word(img, t.vma + TYPE_BASE)
                .and_then(|elem| field_type(tf, img, meta, elem))
                .unwrap_or_else(|| tf.get_type_void().expect("void type"));
            tf.get_type_pointer(8, pointee, 1).ok()
        }
        KIND_STRING => shell(tf, "go.string").ok(),
        KIND_SLICE => shell(tf, "go.slice").ok(),
        KIND_INTERFACE => {
            // `any` (no methods) is an eface; everything else an iface. The
            // method count lives past the base record; phase 1 keys off the
            // canonical names instead: `interface {}` is the eface.
            if t.name == "interface {}" {
                shell(tf, "go.eface").ok()
            } else {
                shell(tf, "go.iface").ok()
            }
        }
        KIND_STRUCT if t.tflag & TFLAG_NAMED != 0 && t.name.contains('.') => {
            shell(tf, &t.name).ok()
        }
        _ => m(type_metatype::TYPE_UNKNOWN),
    }
}

/// Get-or-create one of the well-known Go header shells: an already-interned
/// (possibly completed) struct is returned as-is — `get_type_struct` alone
/// refuses a name it has already completed.
fn shell(tf: &dyn TypeFactory, name: &str) -> kuna_base::error::KunaResult<Rc<Datatype>> {
    if let Some(held) = tf.find_by_name(name)? {
        if held.get_metatype() == type_metatype::TYPE_STRUCT {
            return Ok(held);
        }
        return Err(kuna_base::error::KunaError::lowlevel(format!(
            "gotypes: `{name}` is already held by a non-struct"
        )));
    }
    tf.get_type_struct(name)
}

/// Intern the four well-known header shells and their facets. Idempotent: an
/// already-complete shell is left alone.
fn intern_header_shells(tf: &dyn TypeFactory) {
    let ptr8 = |tf: &dyn TypeFactory| {
        let v = tf.get_type_void().expect("void");
        tf.get_type_pointer(8, v, 1).expect("ptr")
    };
    let u8w = |tf: &dyn TypeFactory, w: int4| tf.get_base(w, type_metatype::TYPE_UINT).expect("uint");
    let rows: [(&str, Vec<(&str, Rc<Datatype>)>, LangFacet); 4] = [
        ("go.string", vec![("data", ptr8(tf)), ("len", u8w(tf, 8))], LangFacet::GoString),
        (
            "go.slice",
            vec![("data", ptr8(tf)), ("len", u8w(tf, 8)), ("cap", u8w(tf, 8))],
            LangFacet::GoSlice { elem: "undefined8".into() },
        ),
        ("go.iface", vec![("tab", ptr8(tf)), ("data", ptr8(tf))], LangFacet::GoIface { eface: false }),
        ("go.eface", vec![("type", ptr8(tf)), ("data", ptr8(tf))], LangFacet::GoIface { eface: true }),
    ];
    for (name, fields, facet) in rows {
        let Ok(ct) = shell(tf, name) else { continue };
        if ct.get_size() == 0 {
            let size = (fields.len() * 8) as int4;
            let fd: Vec<TypeField> = fields
                .into_iter()
                .enumerate()
                .map(|(i, (n, t))| TypeField {
                    ident: i as int4,
                    offset: (i * 8) as int4,
                    name: n.to_string(),
                    field_type: t,
                })
                .collect();
            let _ = tf.set_fields_struct_raw(&ct, fd, Vec::new(), size, 8, 0);
        }
        let _ = tf.kuna_register_lang_facet(name, facet);
    }
}

/// `true` when `name` is a package-qualified type name phase 1 should intern:
/// printable, dotted (`main.Point`), and neither a generic instantiation
/// (`main.Box[int]`) nor an anonymous struct literal — both are later-phase
/// naming questions, not layout ones.
fn name_ok(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 256
        && name.contains('.')
        && name.chars().all(|c| c.is_ascii_graphic() || c == ' ')
        && !name.contains('[')
        && !name.contains('{')
}

/// Walk the typelinks and intern named struct shells. Returns
/// `(walked, interned)` for the note/tests.
fn intern_named_structs(
    tf: &dyn TypeFactory,
    img: &ImageBytes,
    meta: &TypeMeta,
) -> (usize, usize) {
    let mut walked = 0usize;
    let mut interned = 0usize;
    // Pass 1: declare every named struct so cross-references resolve by name.
    let mut structs: Vec<GoType> = Vec::new();
    for &off in &meta.offsets {
        let Some(t) = read_type(img, meta.types, meta.etypes, off) else { continue };
        walked += 1;
        // Typelinks hold the types interface conversion reaches, which for a
        // struct is usually its POINTER form (`*main.Point`); the named struct
        // itself is that pointer's pointee. Consider both the record and, for
        // a pointer, the record it points at.
        let mut candidates: Vec<GoType> = Vec::new();
        if t.kind == KIND_POINTER {
            if let Some(elem) = word(img, t.vma + TYPE_BASE) {
                if elem >= meta.types && elem + TYPE_BASE <= meta.etypes {
                    if let Some(p) = read_type(img, meta.types, meta.etypes, (elem - meta.types) as u32) {
                        candidates.push(p);
                    }
                }
            }
        }
        candidates.push(t);
        for c in candidates {
            if c.kind == KIND_STRUCT
                && c.tflag & TFLAG_NAMED != 0
                && name_ok(&c.name)
                && !structs.iter().any(|s| s.vma == c.vma)
                && shell(tf, &c.name).is_ok()
            {
                structs.push(c);
            }
        }
    }
    // Pass 2: fill the layouts.
    for t in &structs {
        let Ok(ct) = shell(tf, &t.name) else { continue };
        if ct.get_size() != 0 {
            // Already completed (DWARF on an unstripped -g build, or an earlier
            // module) — leave the existing layout alone.
            continue;
        }
        // structtype: pkgPath (ptr) @ +48, fields slice @ +56.
        let Some((f_ptr, f_len)) = plausible_slice(img, t.vma + TYPE_BASE + 8, 4096)
            .or_else(|| {
                // A zero-field struct has len == cap == 0; plausible_slice
                // rejects len 0, so probe that case explicitly.
                let len = word(img, t.vma + TYPE_BASE + 16)?;
                let cap = word(img, t.vma + TYPE_BASE + 24)?;
                (len == 0 && cap == 0).then_some((0, 0))
            })
        else {
            continue;
        };
        let mut fields: Vec<TypeField> = Vec::new();
        let mut good = true;
        for i in 0..f_len {
            let fv = f_ptr + i * 24;
            let (Some(name_ptr), Some(typ_ptr), Some(offset)) =
                (word(img, fv), word(img, fv + 8), word(img, fv + 16))
            else {
                good = false;
                break;
            };
            let Some(fname) = read_go_name(img, name_ptr) else {
                good = false;
                break;
            };
            let Some(fty) = field_type(tf, img, meta, typ_ptr) else {
                good = false;
                break;
            };
            if offset > t.size || offset > i32::MAX as u64 {
                good = false;
                break;
            }
            // A zero-size field (an embedded noCopy / zero-size marker) carries
            // no bytes — drop it rather than refuse the layout (the DWARF
            // importer makes the same call).
            if fty.get_size() == 0 {
                continue;
            }
            fields.push(TypeField {
                ident: i as int4,
                offset: offset as int4,
                name: fname,
                field_type: fty,
            });
        }
        if !good {
            continue;
        }
        fields.sort_by_key(|f| f.offset);
        // Overlapping or over-extent layouts are refused whole (the factory's
        // sorted-disjoint invariant; a wrong layout is worse than a shell).
        let mut prev_end: i64 = 0;
        let mut disjoint = true;
        for f in &fields {
            let sz = f.field_type.get_size().max(1) as i64;
            if (f.offset as i64) < prev_end || f.offset as i64 + sz > t.size as i64 {
                disjoint = false;
                break;
            }
            prev_end = f.offset as i64 + sz;
        }
        if !disjoint {
            continue;
        }
        if tf
            .set_fields_struct_raw(&ct, fields, Vec::new(), t.size as int4, 8, 0)
            .is_ok()
        {
            interned += 1;
        }
    }
    (walked, interned)
}

/// (kuna) `GoTypesPass` — the Go runtime-type harvest (option `gotypes`,
/// default on, Go-detected images only).
pub struct GoTypesPass;

impl AnalysisPass for GoTypesPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }
    fn id(&self) -> &'static str {
        "gotypes"
    }
    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        if !gotypes_enabled() {
            return AnalysisOutput::default();
        }
        // 64-bit little-endian only (module header).
        if !ctx.file.is_little_endian() || !ctx.file.is_64() {
            return AnalysisOutput::default();
        }
        let img = ImageBytes::new(ctx.file);
        let Some(header_vma) = locate_header(ctx.file, &img) else {
            return AnalysisOutput::default();
        };
        let Some(anchor) = locate_moduledata(ctx.file, &img, header_vma) else {
            return AnalysisOutput::default();
        };
        let Some(meta) = locate_type_meta(&img, anchor)
            .or_else(|| locate_type_meta_descriptors(&img, anchor))
        else {
            return AnalysisOutput::default();
        };
        let tf = ctx.arch.types();
        intern_header_shells(tf);
        let (walked, interned) = intern_named_structs(tf, &img, &meta);
        let _ = (walked, interned);
        AnalysisOutput::default()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

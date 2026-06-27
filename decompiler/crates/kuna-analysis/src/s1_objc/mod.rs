//! S1 Mach-O Objective-C metadata recovery (`ObjcMetadataPass`).
//!
//! The kuna analog of Ghidra's `ObjcTypeMetadataAnalyzer` (the name-recovery
//! half): parse the `__objc_*` ObjC 2.0 metadata web entirely out of the loaded
//! Mach-O image and recover **method names**, so a stripped IMP renders
//! `-[Greeter greet:]` / `+[Greeter classMethod]` instead of `sub_<addr>`.
//!
//! # The walk (design §3.2)
//!
//! ```text
//!   __objc_classlist  (array of class_t*)
//!     └─ class_t        isa=metaclass, data=class_ro_t* (& FAST_DATA_MASK)
//!          ├─ class_ro_t.name         → class name ("Greeter")
//!          ├─ class_ro_t.baseMethods  → method_list_t (the `-` instance methods)
//!          └─ class_t.isa → metaclass class_ro_t.baseMethods (the `+` methods)
//!               method_list_t → per method_t: selector, types, IMP (function VA)
//! ```
//!
//! Both `method_list_t` forms are handled (`methods.rs`): the **large absolute**
//! 64-bit form (§3.2's MVP target) and the **small/relative** form a modern clang
//! emits — handling the latter is what makes the bare-toolchain fixture name end
//! to end.
//!
//! # What it emits
//!
//! - **`out.fid_names`** — the IMP function **rename** `-[Class sel]` / `+[Class
//!   sel]`, the label-gated placeholder rename (the FID precedent: the commit seam
//!   only overwrites an engine `sub_*`/`FUN_*` placeholder, never a real symbol).
//! - **`out.symbols`** — an `_OBJC_CLASS_$_<name>` Data symbol per class + the IMP
//!   as a Function symbol (so the function exists for the rename to bind to even
//!   when it is reachable *only* through the metadata, not a call edge or
//!   `LC_FUNCTION_STARTS`) + a selector Data symbol per distinct selector.
//!
//! # Gating
//!
//! Default-OFF (`--option objc`, the `fid` precedent), Mach-O-format gated (the
//! pass is registered only for a Mach-O target in `passes.rs`), real-binary-path
//! only. Selectors are plain ASCII — **no demangler needed**. A non-Mach-O / no-
//! `__objc_*` image yields an empty output, so every parity gate is byte-identical
//! by construction.
//!
//! # Scope (this PR — design §5 PR-O1)
//!
//! x86-64, no chained fixups. The arm64 + `LC_DYLD_CHAINED_FIXUPS` resolver
//! (PR-O0/O2) is deferred: on a chained-fixup image the raw `__objc_*` slots are
//! packed fixup entries, not addresses, so the walk reads garbage and emits
//! nothing (still safe — additive, never wrong-renames). ObjC v1 / dyld-shared-
//! cache extraction / method-type prototypes / `objc_msgSend` overrides are also
//! deferred (design §3.2 LOSS).

pub mod classt;
pub mod encoding;
pub mod methods;
pub mod sections;

use kuna_decomp::dtype::TypeFactory;

use crate::pass::{
    AnalysisCtx, AnalysisOutput, AnalysisPass, FidMatch, Stage, SymFact, SymKind,
};
use sections::MachoImage;

/// The type-mapping context an IMP/ivar prototype is built against: the
/// architecture's [`TypeFactory`] + its pointer size / word size (`data_org`).
/// Threaded from `ctx.arch` into the method walk so the ObjC type-encoding
/// decoder can map `i20@0:8i16` → `int (id, SEL, int)` (design §5 PR-O2).
struct TypeCtx<'a> {
    types: &'a dyn TypeFactory,
    ptr_size: i32,
    word_size: u32,
}

/// The Mach-O Objective-C metadata pass (gate id `objc`, default-off).
///
/// Pure over `ctx` (reads `ctx.file` sections only), additive, never failing —
/// the standard analysis-pass contract. Inert on every non-Mach-O / no-ObjC
/// image.
#[derive(Default)]
pub struct ObjcMetadataPass;

impl AnalysisPass for ObjcMetadataPass {
    fn stage(&self) -> Stage {
        Stage::S1
    }

    fn id(&self) -> &'static str {
        "objc"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        // Mach-O only (belt-and-suspenders on top of the format gate in passes.rs).
        if ctx.file.format() != object::BinaryFormat::MachO {
            return out;
        }
        // Resolve LC_DYLD_CHAINED_FIXUPS (PR-O0) so an arm64(e) image's __objc_*
        // pointer slots dereference to real addresses, not packed fixup entries.
        // Empty (no-op) on a classic x86-64 LC_DYLD_INFO Mach-O — the walk then
        // reads raw section words exactly as the x86-64 PR-O1 path did.
        let fixups = crate::s1_loader::format::macho::resolve_chained_fixups(ctx.file, ctx.bytes);
        let img = MachoImage::with_fixups(ctx.file, fixups);
        let Some((cl_addr, cl_data)) = img.classlist(ctx.file) else {
            return out;
        };

        // The type-mapping context for IMP/ivar prototypes (PR-O2).
        let (ptr_size, word_size) = ctx.arch.data_org();
        let tctx = TypeCtx { types: ctx.arch.types(), ptr_size, word_size };

        let ps = img.ptr_size() as usize;
        // Walk `__objc_classlist`: an array of `class_t*` (stride = ptr size).
        for (i, _) in cl_data.chunks_exact(ps).enumerate() {
            let entry_va = cl_addr.wrapping_add((i * ps) as u64);
            let Some(class_ptr) = img.read_ptr(entry_va) else {
                continue;
            };
            if class_ptr == 0 {
                continue;
            }
            self.walk_class(&img, &tctx, class_ptr, &mut out);
        }
        out
    }
}

impl ObjcMetadataPass {
    /// Recover one class: its name, instance (`-`)/class (`+`) methods, and ivars.
    fn walk_class(
        &self,
        img: &MachoImage,
        tctx: &TypeCtx,
        class_ptr: u64,
        out: &mut AnalysisOutput,
    ) {
        let Some(class) = classt::read_class_t(img, class_ptr) else {
            return;
        };
        let Some(name) = classt::class_name(img, class.ro) else {
            return;
        };

        // The class symbol (`_OBJC_CLASS_$_Greeter`), a Data fact at the class_t.
        out.symbols.push(SymFact {
            addr: class.addr,
            name: format!("_OBJC_CLASS_$_{name}"),
            kind: SymKind::Data,
        });

        // Instance (`-`) methods: class_ro_t.baseMethods.
        if let Some(list) = classt::base_methods(img, class.ro) {
            self.emit_methods(img, tctx, &name, '-', list, out);
        }
        // Class (`+`) methods: metaclass (isa) class_ro_t.baseMethods.
        if let Some(meta_ro) = classt::metaclass_ro(img, class.isa) {
            if let Some(list) = classt::base_methods(img, meta_ro) {
                self.emit_methods(img, tctx, &name, '+', list, out);
            }
        }
        // Instance variables: `<Class>::ivar` Data labels (PR-O2).
        if let Some(ivar_list) = classt::base_ivars(img, class.ro) {
            self.emit_ivars(img, tctx, &name, ivar_list, out);
        }
    }

    /// Emit the rename + function/selector symbols + (PR-O2) the typed prototype
    /// for one method list. `kind` is `-` (instance) or `+` (class).
    fn emit_methods(
        &self,
        img: &MachoImage,
        tctx: &TypeCtx,
        class: &str,
        kind: char,
        list_va: u64,
        out: &mut AnalysisOutput,
    ) {
        for m in methods::walk_method_list(img, list_va) {
            // `-[Greeter greet:]` / `+[Greeter classMethod]` — the ObjC method name
            // the printer renders. Selectors are ASCII, so no demangling.
            let label = format!("{kind}[{class} {sel}]", sel = m.selector);

            // The IMP must exist as a function for the rename to bind, even when it
            // is reachable ONLY through the metadata (no call edge / no
            // LC_FUNCTION_STARTS). The idempotent SymFact add is a no-op if the
            // function already exists, and the placeholder name it installs is what
            // the fid_names rename then overwrites.
            out.symbols.push(SymFact {
                addr: m.imp,
                name: label.clone(),
                kind: SymKind::Function,
            });
            // The label-gated rename (the FID precedent): overwrite the engine's
            // own sub_*/FUN_* placeholder with the ObjC method name, never a real
            // symbol.
            out.fid_names.push(FidMatch { addr: m.imp, name: label.clone() });

            // The typed prototype from the ObjC type encoding (PR-O2): `i20@0:8i16`
            // → `int -[Greeter greet:](id self, SEL _cmd, int n)`. Keyed by the
            // IMP label name (the commit seam matches `pieces.name` against the
            // FunctionSymbol installed above with that same `-[Class sel]` name).
            // Best-effort — an unparseable / absent encoding leaves the IMP *named*
            // (above) but untyped, never a failure.
            if let Some(enc) = m.types.as_deref() {
                if let Some(pieces) = encoding::decode_method(
                    &label, enc, tctx.types, tctx.ptr_size, tctx.word_size,
                ) {
                    out.prototypes.push(pieces);
                }
            }

            // A Data symbol on the selector string itself, so the selector text is
            // named in the listing (`sel_greet_` etc.). Best-effort: only when the
            // selector VMA is separately recoverable.
            if m.selector_va != 0 {
                out.symbols.push(SymFact {
                    addr: m.selector_va,
                    name: format!("sel_{}", sanitize_selector(&m.selector)),
                    kind: SymKind::Data,
                });
            }
        }
    }

    /// Emit the `<Class>::ivar` Data labels for one `ivar_list_t` (PR-O2). Each
    /// ivar's `*offset` slot is labeled with the `<Class>::<ivar>` name so the
    /// listing names the field-offset constant; the type encoding is decoded for
    /// completeness (the offset slot itself is a `long`, the field type rides the
    /// label name). Best-effort, additive — a malformed list emits nothing.
    fn emit_ivars(
        &self,
        img: &MachoImage,
        tctx: &TypeCtx,
        class: &str,
        list_va: u64,
        out: &mut AnalysisOutput,
    ) {
        for iv in classt::walk_ivar_list(img, list_va) {
            // The `<Class>::<ivar>` label on the ivar's offset variable
            // (`_OBJC_IVAR_$_Class.ivar`), a Data symbol at the offset slot VMA.
            out.symbols.push(SymFact {
                addr: iv.offset_va,
                name: format!("{class}::{name}", name = iv.name),
                kind: SymKind::Data,
            });
            // Decode the ivar element type for completeness (not committed as a
            // typed datatype here — the offset slot is a scalar; this just
            // validates the encoding path and keeps the decoder exercised).
            if let Some(enc) = iv.types.as_deref() {
                let _ = encoding::decode_ivar_type(
                    enc, tctx.types, tctx.ptr_size, tctx.word_size,
                );
            }
        }
    }
}

/// Turn a selector (`greet:`, `setFoo:bar:`) into a symbol-name-safe token for a
/// `sel_<...>` Data symbol: colons → underscores, every other non-alphanumeric →
/// underscore. The selector text itself is preserved in the IMP rename label.
fn sanitize_selector(sel: &str) -> String {
    sel.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_identity() {
        let p = ObjcMetadataPass;
        assert_eq!(p.id(), "objc");
        assert_eq!(p.stage(), Stage::S1);
    }

    #[test]
    fn sanitize_selector_replaces_colons() {
        assert_eq!(sanitize_selector("greet:"), "greet_");
        assert_eq!(sanitize_selector("setFoo:bar:"), "setFoo_bar_");
        assert_eq!(sanitize_selector("length"), "length");
    }

    /// The full metadata walk over the vendored `macho_objc` fixture (spec-less:
    /// parses the Mach-O directly, no SLEIGH `.sla` needed) recovers the class
    /// `Greeter`, the selector `greet:`, and the IMP at the pinned VMA — the same
    /// chain `verify_objc.rs` proves end-to-end through the decompiler. Locks the
    /// `MachoImage` / `class_t` / `method_list_t` (small/relative form) readers.
    #[test]
    fn walks_macho_objc_fixture_metadata() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/macho_objc");
        let bytes = std::fs::read(path).expect("read macho_objc fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse Mach-O");
        assert_eq!(file.format(), object::BinaryFormat::MachO, "fixture must be a Mach-O");

        let img = MachoImage::new(&file);
        let (cl_addr, cl_data) = img.classlist(&file).expect("__objc_classlist present");
        let ps = img.ptr_size() as usize;
        assert_eq!(ps, 8, "the fixture is a 64-bit Mach-O");

        // First (only) class: class_t -> class_ro_t -> name "Greeter".
        let class_ptr = img.read_ptr(cl_addr).expect("class_t pointer");
        let class = classt::read_class_t(&img, class_ptr).expect("class_t reads");
        assert_eq!(classt::class_name(&img, class.ro).as_deref(), Some("Greeter"));

        // Instance method list: the single `-(int)greet:` method.
        let list = classt::base_methods(&img, class.ro).expect("baseMethods present");
        let methods = methods::walk_method_list(&img, list);
        assert_eq!(methods.len(), 1, "exactly one instance method");
        let m = &methods[0];
        assert_eq!(m.selector, "greet:");
        assert_eq!(m.imp, 0x100000640, "the pinned IMP VMA");
        assert_eq!(m.types.as_deref(), Some("i20@0:8i16"));

        // The metaclass has no `+` methods.
        let meta_ro = classt::metaclass_ro(&img, class.isa).expect("metaclass ro");
        assert!(classt::base_methods(&img, meta_ro).is_none(), "no class (+) methods");

        // The classlist holds exactly one class_t* entry (8 bytes).
        assert_eq!(cl_data.len(), ps, "one class in the classlist");
    }

    /// The arm64 chained-fixup path (design §5 PR-O0 + PR-O2): the vendored
    /// `macho_objc_arm64` fixture is a **real `LC_DYLD_CHAINED_FIXUPS`** binary
    /// (`-fixup_chains` lld), so its `__objc_*` pointer slots are packed fixup
    /// entries. This proves the chained-fixup resolver + the arm64 ObjC walk
    /// together recover `-[Greeter greet:]`:
    ///
    ///   1. The resolver builds a non-empty overlay; the `__objc_classlist[0]`
    ///      slot **resolves** to the real `class_t` VMA `0x100008000` (a slot the
    ///      raw word also happens to equal, with `next=0`), and the `class_t.isa`
    ///      slot — which carries a non-zero `next` in its high bits — resolves to
    ///      the real `0x100008028` (the raw word `0x0020000100008028` would be
    ///      garbage without the resolver).
    ///   2. With the overlay threaded through `MachoImage::read_ptr`, the walk
    ///      reaches the class name `Greeter`, the selector `greet:`, the type
    ///      encoding `i20@0:8i16`, and the IMP at the pinned VMA `0x1000005f0`.
    #[test]
    fn walks_macho_objc_arm64_chained_fixups() {
        use crate::s1_loader::format::macho::resolve_chained_fixups;
        use object::read::Object;

        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/macho_objc_arm64");
        let bytes = std::fs::read(path).expect("read macho_objc_arm64 fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse Mach-O");
        assert_eq!(file.format(), object::BinaryFormat::MachO);
        assert_eq!(file.architecture(), object::Architecture::Aarch64, "arm64 fixture");

        // (1) The chained-fixup resolver: a real LC_DYLD_CHAINED_FIXUPS binary
        // yields a NON-empty overlay, and the classlist/class_t slots resolve to
        // the real class_t VMAs.
        let fixups = resolve_chained_fixups(&file, &bytes);
        assert!(!fixups.is_empty(), "the fixture carries real chained fixups");
        assert_eq!(
            fixups.resolved_ptr(0x100004000),
            Some(0x100008000),
            "__objc_classlist[0] resolves to the class_t VMA"
        );
        assert_eq!(
            fixups.resolved_ptr(0x100008000),
            Some(0x100008028),
            "class_t.isa slot resolves to the metaclass (raw word would be garbage)"
        );

        // (2) The arm64 walk THROUGH the resolved overlay recovers the method name.
        let img = MachoImage::with_fixups(&file, fixups);
        let (cl_addr, _cl_data) = img.classlist(&file).expect("__objc_classlist present");
        let class_ptr = img.read_ptr(cl_addr).expect("class_t pointer (resolved)");
        assert_eq!(class_ptr, 0x100008000, "the resolved class_t VMA");

        let class = classt::read_class_t(&img, class_ptr).expect("class_t reads");
        assert_eq!(classt::class_name(&img, class.ro).as_deref(), Some("Greeter"));

        let list = classt::base_methods(&img, class.ro).expect("baseMethods present");
        let methods = methods::walk_method_list(&img, list);
        assert_eq!(methods.len(), 1, "exactly one instance method");
        let m = &methods[0];
        assert_eq!(m.selector, "greet:");
        assert_eq!(m.imp, 0x1000005f0, "the pinned arm64 IMP VMA");
        assert_eq!(m.types.as_deref(), Some("i20@0:8i16"));
    }

    /// The no-op guarantee (CAUTION in the brief): the chained-fixup resolver
    /// yields an **empty** overlay on the classic x86-64 `macho_objc` fixture
    /// (which uses `LC_DYLD_INFO_ONLY`, not `LC_DYLD_CHAINED_FIXUPS`), so
    /// `read_ptr` reads raw section words exactly as before — the existing x86-64
    /// fixtures stay byte-identical.
    #[test]
    fn resolver_is_noop_on_non_chained_fixup_macho() {
        use crate::s1_loader::format::macho::resolve_chained_fixups;
        for name in ["macho_objc", "macho_imports"] {
            let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
            let bytes = std::fs::read(&path).expect("read fixture");
            let file = object::File::parse(bytes.as_slice()).expect("parse Mach-O");
            let fixups = resolve_chained_fixups(&file, &bytes);
            assert!(
                fixups.is_empty(),
                "{name} has no LC_DYLD_CHAINED_FIXUPS → the resolver is a strict no-op"
            );
            // With an empty overlay, read_ptr falls through to the raw section word.
            let img = MachoImage::with_fixups(&file, fixups);
            let img_raw = MachoImage::new(&file);
            if let Some((cl_addr, _)) = img.classlist(&file) {
                assert_eq!(
                    img.read_ptr(cl_addr),
                    img_raw.read_ptr(cl_addr),
                    "{name}: with/without the (empty) overlay reads identically"
                );
            }
        }
    }
}

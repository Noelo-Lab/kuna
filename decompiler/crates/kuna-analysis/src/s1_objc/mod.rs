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
pub mod methods;
pub mod sections;

use crate::pass::{
    AnalysisCtx, AnalysisOutput, AnalysisPass, FidMatch, Stage, SymFact, SymKind,
};
use sections::MachoImage;

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
        let img = MachoImage::new(ctx.file);
        let Some((cl_addr, cl_data)) = img.classlist(ctx.file) else {
            return out;
        };

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
            self.walk_class(&img, class_ptr, &mut out);
        }
        out
    }
}

impl ObjcMetadataPass {
    /// Recover one class: its name + instance (`-`) and class (`+`) methods.
    fn walk_class(&self, img: &MachoImage, class_ptr: u64, out: &mut AnalysisOutput) {
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
            self.emit_methods(img, &name, '-', list, out);
        }
        // Class (`+`) methods: metaclass (isa) class_ro_t.baseMethods.
        if let Some(meta_ro) = classt::metaclass_ro(img, class.isa) {
            if let Some(list) = classt::base_methods(img, meta_ro) {
                self.emit_methods(img, &name, '+', list, out);
            }
        }
    }

    /// Emit the rename + function/selector symbols for one method list. `kind` is
    /// `-` (instance) or `+` (class).
    fn emit_methods(
        &self,
        img: &MachoImage,
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
            out.fid_names.push(FidMatch { addr: m.imp, name: label });

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
}

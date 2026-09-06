//! Per-function `kind` classification (`"func"` | `"plt"` | `"thunk"`) — the
//! annotation the browser inventory folds import stubs and trampolines out of,
//! and the one `kuna decompile-graph` labels its function rows with.
//!
//! Shared here (rather than in either front-end) for the same reason
//! [`crate::project`] is: two surfaces answer "what kind of function is this"
//! and they must not answer it differently.
//!
//! A [`Classifier`] is built once per run from (a) an `object`-crate re-parse
//! of the binary bytes — the import-stub section ranges (`.plt` family /
//! Mach-O symbol stubs) and the imported-symbol name set — and (b) the deduped
//! function-entry list the engine reported. Per entry:
//!
//! * `"plt"`   — the entry VMA falls inside a stub section, or the name is an
//!   imported symbol. (kuna's function names are demangled; an import that
//!   only matches after demangling is caught by the range test instead.)
//! * `"thunk"` — the entry's single instruction is a *lone jump*
//!   ([`ConsoleProgram::lone_jump_target`]): direct to another known function
//!   entry, or indirect (`jmp [mem]`) anywhere.
//! * `"func"`  — everything else (including any probe/parse failure —
//!   conservative).
//!
//! On ARM-family specs a Thumb entry VMA carries the mode bit; every range /
//! entry-set test masks it (`vma & !1`), the same normalization
//! `kuna_console::project::build_asm` applies to its labels.

use std::collections::BTreeSet;

use crate::engine::ConsoleProgram;
use object::{Object, ObjectSection, ObjectSymbol};

/// ELF sections that hold import/linkage stubs (procedure-linkage tables).
const ELF_STUB_SECTIONS: &[&str] = &[".plt", ".plt.got", ".plt.sec", ".iplt", ".MIPS.stubs"];

/// Mach-O stub-section name fallbacks (used alongside the authoritative
/// `S_SYMBOL_STUBS` section-type flag).
const MACHO_STUB_SECTIONS: &[&str] = &[
    "__stubs",
    "__auth_stubs",
    "__symbol_stub",
    "__symbol_stub1",
    "__picsymbolstub4",
    "__stub_helper",
];

/// The per-run classification context (see the module doc).
pub struct Classifier {
    /// ARM-family spec ⇒ Thumb-bit normalization applies (`vma & !1`).
    arm: bool,
    /// `[start, end)` VMA ranges of the import-stub sections.
    stub_ranges: Vec<(u64, u64)>,
    /// Imported symbol names (ELF undefined dynamic symbols; Mach-O undefined
    /// symbols with one leading `_` stripped; PE `imports()`).
    imports: BTreeSet<String>,
    /// Every known function-entry VMA, Thumb-masked (the "ANOTHER entry"
    /// test for a direct lone jump).
    entries: BTreeSet<u64>,
}

impl Classifier {
    /// Build the context: re-parse `binary`'s bytes with the `object` crate
    /// (already the LoadImage backend's parser — no new dependency) and
    /// normalize the engine's deduped entry list. A missing/unparsable file
    /// degrades to empty ranges/imports (every entry then probes as
    /// `"thunk"`/`"func"`).
    pub fn new(prog: &ConsoleProgram, binary: &str, entries: impl Iterator<Item = u64>) -> Self {
        let bytes = std::fs::read(binary).unwrap_or_default();
        match kuna_analysis::loadimage_object::parse_object(&*bytes) {
            Ok(file) => Classifier::from_object(prog, Some(&file), entries),
            Err(_) => Classifier::from_object(prog, None, entries),
        }
    }

    /// [`Self::new`] off an already-parsed image, for a caller that holds one.
    pub fn from_object(
        prog: &ConsoleProgram,
        file: Option<&object::File>,
        entries: impl Iterator<Item = u64>,
    ) -> Self {
        let arm = prog.description().starts_with("ARM");
        let normalize = |vma: u64| if arm { vma & !1 } else { vma };
        let entries: BTreeSet<u64> = entries.map(normalize).collect();

        let mut stub_ranges = Vec::new();
        let mut imports = BTreeSet::new();
        if let Some(f) = file {
            collect_stub_ranges(f, &mut stub_ranges);
            collect_imports(f, &mut imports);
        }
        Classifier { arm, stub_ranges, imports, entries }
    }

    /// Thumb-mask an ARM VMA (identity elsewhere).
    fn normalize(&self, vma: u64) -> u64 {
        if self.arm {
            vma & !1
        } else {
            vma
        }
    }

    /// Classify one function entry (see the module doc for the rules).
    pub fn kind(&self, prog: &ConsoleProgram, name: &str, vma: u64) -> &'static str {
        let norm = self.normalize(vma);
        // An entry with no mapped bytes is an EXTERNAL — a relocatable object's
        // undefined symbol, bound to a synthetic address purely so a call to it
        // renders by name. It is a call that leaves this module and has no body
        // to show, which is what the `plt` group means here; the name test below
        // cannot catch it, since kuna's names are demangled
        // (`CellClass::Cell_Coord`) while the symbol table's are not.
        if !prog.vma_bytes_mapped(norm) {
            return "plt";
        }
        if self.stub_ranges.iter().any(|&(s, e)| norm >= s && norm < e)
            || self.imports.contains(name)
        {
            return "plt";
        }
        match prog.lone_jump_target(norm) {
            // Direct lone jump: a thunk only if it lands on ANOTHER known
            // function entry (a jump-to-self / jump-into-body is not one).
            Some(Some(target)) => {
                let t = self.normalize(target);
                if t != norm && self.entries.contains(&t) {
                    "thunk"
                } else {
                    "func"
                }
            }
            // Indirect lone jump (`jmp [mem]`): a stub-shaped trampoline even
            // outside a recognized stub section.
            Some(None) => "thunk",
            None => "func",
        }
    }
}

/// The `[start, end)` VMA ranges of every import-stub section (nonzero size).
fn collect_stub_ranges(f: &object::File, out: &mut Vec<(u64, u64)>) {
    for s in f.sections() {
        let name = s.name().unwrap_or("");
        let is_stub = match f.format() {
            object::BinaryFormat::Elf => ELF_STUB_SECTIONS.contains(&name),
            object::BinaryFormat::MachO => {
                // The section-type flag is authoritative; the well-known stub
                // names are kept as a fallback.
                let flagged = matches!(
                    s.flags(),
                    object::SectionFlags::MachO { flags }
                        if flags & object::macho::SECTION_TYPE == object::macho::S_SYMBOL_STUBS
                );
                flagged || MACHO_STUB_SECTIONS.contains(&name)
            }
            _ => false, // PE: imports are reached via the IAT, not stub sections
        };
        if is_stub && s.size() > 0 {
            out.push((s.address(), s.address().saturating_add(s.size())));
        }
    }
}

/// The imported-symbol name set (per-format, see [`Classifier::imports`]).
fn collect_imports(f: &object::File, out: &mut BTreeSet<String>) {
    match f.format() {
        object::BinaryFormat::Elf => {
            for sym in f.dynamic_symbols() {
                if sym.is_undefined() {
                    if let Ok(name) = sym.name() {
                        if !name.is_empty() {
                            out.insert(name.to_string());
                        }
                    }
                }
            }
        }
        object::BinaryFormat::MachO => {
            for sym in f.symbols() {
                if sym.is_undefined() {
                    if let Ok(name) = sym.name() {
                        // Mach-O C symbols carry one leading underscore.
                        let name = name.strip_prefix('_').unwrap_or(name);
                        if !name.is_empty() {
                            out.insert(name.to_string());
                        }
                    }
                }
            }
        }
        object::BinaryFormat::Pe => {
            if let Ok(imports) = f.imports() {
                for imp in imports {
                    let name = String::from_utf8_lossy(imp.name()).into_owned();
                    if !name.is_empty() {
                        out.insert(name);
                    }
                }
            }
        }
        _ => {}
    }
}

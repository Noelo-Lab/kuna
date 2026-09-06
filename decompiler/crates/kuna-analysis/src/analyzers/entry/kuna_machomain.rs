//! (kuna) `machomain` — name the Mach-O `LC_MAIN` entry routine `main` and give
//! it the conventional `int main(int, char **)` prototype (P1 program prep).
//!
//! On a stripped Mach-O executable every function comes out `sub_<addr>`, and
//! the one an agent needs first — the program's entry — is indistinguishable
//! from the rest of the inventory: `kuna functions` lists 24 `sub_*` and no
//! `main`, so finding where the program starts means reading bodies. The image
//! already says which one it is. `LC_MAIN` is a load command whose `entryoff`
//! field is documented as the file offset of `main()`; `ld64` emits it for every
//! normally-linked executable, `dyld` calls that address as
//! `main(argc, argv, envp, apple)`, and — unlike the symbol table — the load
//! command survives `strip` intact. So the name is a fact the container states,
//! not an inference: this pass reads it and applies it.
//!
//! Two things are applied together, because they come from the same fact:
//!
//! - the **name** `main`, through the `entry_names` overlay the commit boundary
//!   already consults for the ELF `_INIT_<i>`/`_DT_INIT` names;
//! - the **prototype** `int main(int argc, char **argv)`, parked by that name.
//!
//! The prototype is the Mach-O analog of what [`super::kuna_entrymainproto`]
//! recovers on a PE, and it exists for the same reason: kuna reads a callee's
//! parameters out of the callee's OWN body, so a `main` that ignores its
//! arguments never reads `rdi`/`rsi` and is declared `void(void)`. On a PE the
//! arguments are recoverable only by reading the in-image CRT startup's call
//! site, which is why that pass is a byte scan for a named-accessor cluster.
//! Mach-O needs none of that: the C runtime that calls `main` lives in
//! `libdyld.dylib`, outside the image, and the ABI it uses is fixed. The two
//! passes are deliberately typed differently for that reason — `entrymainproto`
//! reports the widths its call site establishes and refuses to assert the C
//! library's declaration (the same shape carries `wmain`'s `wchar_t **`),
//! whereas `LC_MAIN` *is* the POSIX `main` by definition, so the real
//! `int` / `char **` spelling is the honest one and lets a string literal render
//! through `argv[i]`.
//!
//! `envp` is deliberately NOT declared. `dyld` does pass it (and a fourth
//! `apple` pointer), but a three- or four-parameter `main` is unconventional
//! enough that the extra unused slots cost more in noise than they buy, and a
//! `main` that really reads `envp` still shows the third argument register in
//! its body.
//!
//! ## What it refuses
//!
//! - a non-Mach-O image, and any Mach-O that is not `MH_EXECUTE` (a dylib or
//!   bundle carries no `LC_MAIN`);
//! - an `LC_UNIXTHREAD`-only image (the pre-10.8 entry shape) — that entry is
//!   the crt's `start`, not `main`, so nothing is claimed;
//! - an entry VMA outside every executable section;
//! - an entry that ALREADY carries a function symbol. A non-stripped Mach-O
//!   names it `_main` from its own `.symtab` and that name wins, exactly as the
//!   commit boundary's idempotent `find_function_across_scopes` probe arranges;
//! - an image that already defines a symbol spelled `main`, so the by-name
//!   prototype park can never land on a different function.
//!
//! Like every other analysis pass the facts are computed at LOAD and COMMITTED
//! only when the gate is on, so `--option machomain off` restores the
//! `sub_<addr>` / `void(void)` form exactly.

use kuna_decomp::dtype::type_metatype;
use kuna_decomp::fspec::PrototypePieces;
use object::read::{Object, ObjectSymbol};

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

use super::{executable_sections, existing_function_addrs, in_executable_section};

/// The name the load command licenses. `LC_MAIN`'s `entryoff` is documented as
/// the offset of `main()`, so this is a restatement of the container, not a
/// guess — and it is spelled without the Mach-O `_` prefix on purpose: the
/// prefix is the assembler's C-symbol decoration, and `kuna functions` is asked
/// for the C name.
const MAIN: &str = "main";

/// (kuna) The Mach-O `LC_MAIN` entry naming + prototype pass (`machomain`).
///
/// Registered like every other analysis pass and computed at LOAD; the commit
/// boundary applies the name and the one prototype only when
/// `--option machomain on` (the default) lets this pass's output through the
/// gate.
pub struct MachoMainPass;

impl AnalysisPass for MachoMainPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "machomain"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        let Some(vma) = entry_main_vma(ctx.file, ctx.bytes) else {
            return out;
        };
        // The name rides the `entry_names` overlay, which the commit consults for
        // VMAs in `entries`. The entry oracle already discovers this address on
        // every image that carries `LC_MAIN`, but emitting it here too keeps the
        // name from silently evaporating if that oracle is off or filters it.
        out.entries.push(vma);
        out.entry_names.push((vma, MAIN.to_string()));
        if let Some(pieces) = main_prototype(ctx) {
            out.prototypes.push(pieces);
        }
        out
    }
}

/// The VMA `LC_MAIN` names, once every refusal in the module header has been
/// applied. `None` means this pass contributes nothing and the commit is
/// byte-identical to before.
fn entry_main_vma(file: &object::File, bytes: &[u8]) -> Option<u64> {
    if file.format() != object::BinaryFormat::MachO {
        return None;
    }
    let vma = super::macho_entry::macho_main_entry_vma(bytes)?;
    if !in_executable_section(&executable_sections(file), vma) {
        return None;
    }
    // A named entry has a better name coming from whatever named it, and an image
    // that already spells a symbol `main` would make the by-name prototype park
    // ambiguous.
    if existing_function_addrs(file, bytes).binary_search(&vma).is_ok() {
        return None;
    }
    if file.symbols().chain(file.dynamic_symbols()).any(|s| s.name() == Ok(MAIN)) {
        return None;
    }
    Some(vma)
}

/// `int main(int argc, char **argv)` — the declaration `LC_MAIN` licenses.
fn main_prototype(ctx: &AnalysisCtx) -> Option<PrototypePieces> {
    let (_addr_size, word_size) = ctx.arch.data_org();
    let types = ctx.arch.types();
    let ptr = types.get_size_of_pointer();
    let int4 = types.get_base(4, type_metatype::TYPE_INT).ok()?;
    let ch = types.get_type_char(types.get_size_of_char()).ok()?;
    let charp = types.get_type_pointer(ptr, ch, word_size).ok()?;
    let charpp = types.get_type_pointer(ptr, charp, word_size).ok()?;
    Some(PrototypePieces {
        name: MAIN.to_string(),
        outtype: Some(std::rc::Rc::clone(&int4)),
        intypes: vec![int4, charpp],
        innames: vec!["argc".to_string(), "argv".to_string()],
        first_var_arg_slot: -1,
        output_storage: None,
        input_storage: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> Vec<u8> {
        let path = format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name);
        std::fs::read(&path).unwrap_or_else(|_| panic!("read fixture {path}"))
    }

    fn claim(name: &str) -> Option<u64> {
        let bytes = fixture(name);
        let file = object::File::parse(bytes.as_slice()).unwrap_or_else(|e| panic!("{name}: {e}"));
        entry_main_vma(&file, bytes.as_slice())
    }

    /// The headline: a stripped Mach-O executable's `LC_MAIN` entry is claimed,
    /// at the address the load command states.
    #[test]
    fn claims_the_lc_main_entry_of_a_stripped_executable() {
        assert_eq!(claim("macho_stripped_main"), Some(0x1000005b0));
    }

    /// The guard that keeps this from overwriting better knowledge: the very same
    /// image with its symbol table intact names that address `_main` itself, and
    /// a named entry is left alone.
    #[test]
    fn defers_to_an_entry_that_already_carries_a_symbol() {
        assert_eq!(claim("macho_imports"), None, "the named twin must be refused");
        assert_eq!(claim("macho_imports_arm64"), None, "arch-independent refusal");
    }

    /// Structurally inert off Mach-O: an ELF executable and a PE both contribute
    /// nothing, which is why no ELF/PE parity assertion can move.
    #[test]
    fn contributes_nothing_off_mach_o() {
        assert_eq!(claim("cet_pie_x86_64"), None, "ELF");
        assert_eq!(claim("msvc_rtti_x64.exe"), None, "PE");
    }
}

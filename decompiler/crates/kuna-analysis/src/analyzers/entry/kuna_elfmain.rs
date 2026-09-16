//! (kuna) `elfmain` — name the ELF routine crt1 hands to `__libc_start_main`
//! `main`, and declare it `int main(int argc, char **argv)` (P1 program prep).
//!
//! kuna already finds this address. Entry-discovery oracle 4 decodes the
//! `_start` → `__libc_start_main(main, …)` idiom on x86-64, AArch64, ARM and
//! RISC-V and seeds the recovered VMA as a function entry — but address-only, so
//! on a stripped ELF the program's own starting point arrives in the inventory
//! as one more `sub_<addr>`, with whatever prototype reading its body alone
//! produces. On `fmt` built `-O2` and stripped that is
//! `unsigned long sub_26a0(int a0,char **a1)`: the two slots exist because this
//! `main` happens to read them, their widths are right, and everything an agent
//! actually wants — which function this is, that `a0` counts the strings `a1`
//! points at, that the value handed back is the process exit status — is absent.
//!
//! The container states all of it. The C runtime's contract is that the first
//! argument to `__libc_start_main` is `main`, and the POSIX declaration of
//! `main` is `int main(int, char **)`. So this pass applies two things that come
//! from the same fact, exactly as its Mach-O counterpart
//! [`super::kuna_machomain`] applies them from `LC_MAIN`:
//!
//! - the **name** `main`, through the `entry_names` overlay the commit boundary
//!   already consults for the `_INIT_<i>`/`_DT_INIT` names;
//! - the **prototype** `int main(int argc, char **argv)`, parked by that name.
//!
//! The return type is the half body-driven recovery can never supply: `main`'s
//! value is consumed by `__libc_start_main`, outside the image, so kuna types it
//! from the widest register write it can see and prints `unsigned long`. The
//! parameters are the half it supplies only by accident — a `main` that ignores
//! its arguments reads neither argument register and is declared `void(void)`,
//! and one that reads only `argc` declares one anonymous slot.
//!
//! ## Why three parameters and not two
//!
//! glibc's `__libc_start_main` calls `main(argc, argv, envp)`, and the third
//! argument is declared here even though most programs ignore it. The two-
//! argument form is not a smaller version of the same claim — because the
//! prototype is applied LOCKED, declaring two parameters asserts that there is
//! no third one, and on a `main` that does read `envp` that assertion deletes a
//! live parameter: the entry value stops being an input, the read becomes an
//! uninitialised local, and the emitted C passes that undefined local on. The
//! in-repo ARM fixture `armlibcmain_le32` @0x103dc is one — it forwards its
//! third argument register straight to `__printf_chk` — and the two-argument
//! form rendered `unsigned int v4; // r2` with no assignment anywhere, still
//! passed as `__printf_chk(2,"…",v4,0)`.
//!
//! Nothing readable at load time separates the two cases. That fixture's `main`
//! never touches `r2`: it sets up `r0`/`r1`, branches to `__printf_chk`, and the
//! only evidence that `r2` carries a value is the CALLEE's signature. A body
//! walk looking for a read of the third argument register sees nothing there and
//! nothing in a `main` that truly ignores `envp`, so it cannot tell them apart —
//! and the two mistakes are not symmetric. An `envp` the program ignores is one
//! unused parameter in a declaration that is true of every hosted C program;
//! an `envp` dropped from a program that uses it is wrong output. So the
//! declaration the C runtime actually makes is the one applied, which is also
//! what IDA Pro reports for the same address
//! (`int __cdecl main(int argc, const char **argv, const char **envp)`).
//!
//! ## Where the address comes from
//!
//! [`super::libc_start_main_target`] — oracle 4 itself, unchanged, so this pass
//! never decodes an instruction of its own and can never disagree with the entry
//! the discovery set already contains. Its ARM arm is written around the
//! `R_ARM_RELATIVE` that relocates crt1's GOT slot, which a non-PIE ARM32
//! executable does not carry; [`super::kuna_armlibcmain`] is the arm that reads
//! that value out of the image instead, and it already names its result `main`.
//! This pass therefore consults it as a second source **for the prototype only**
//! — the entry and the name stay that pass's own, so `--option armlibcmain off`
//! still restores the previous inventory exactly and the by-name park simply
//! finds no `main` to land on.
//!
//! ## What it refuses
//!
//! - a non-ELF image (Mach-O is `machomain`'s, PE is
//!   [`super::kuna_entrymainproto`]'s);
//! - an image with no symbol spelled `__libc_start_main` at all. Oracle 4's
//!   x86-64 arm matches an argument-setup encoding and a following `call`, which
//!   is evidence enough to add a function entry but not to assert that the
//!   function is the C `main`; the C runtime's own name is what distinguishes a
//!   glibc crt1 from a hand-rolled entry point, and a fully stripped static
//!   image states neither;
//! - a recovered address equal to `_start` itself, or outside every executable
//!   section;
//! - an address that ALREADY carries a function symbol. A non-stripped ELF names
//!   it `main` from its own `.symtab` and that name wins, exactly as the commit
//!   boundary's idempotent `find_function_across_scopes` probe arranges — which
//!   is also why no unstripped fixture, and no parity assertion, can move;
//! - an image that already defines a symbol spelled `main`, so the by-name
//!   prototype park can never land on a different function (the ambiguity
//!   `retain_unambiguous_names` guards against in `analyzers/protos`).
//!
//! Like every other analysis pass the facts are computed at LOAD and COMMITTED
//! only when the gate is on, so `--option elfmain off` restores the
//! `sub_<addr>` / width-only form exactly.

use kuna_decomp::dtype::type_metatype;
use kuna_decomp::fspec::PrototypePieces;
use object::read::{Object, ObjectSymbol};

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

use super::{executable_sections, existing_function_addrs, in_executable_section};

/// The name the C runtime's contract licenses for `__libc_start_main`'s first
/// argument.
const MAIN: &str = "main";

/// The C runtime entry whose presence says the image's `_start` is a crt1.
const LIBC_START_MAIN: &[u8] = b"__libc_start_main";

/// (kuna) The ELF libc-start `main` naming + prototype pass (`elfmain`).
///
/// Registered like every other analysis pass and computed at LOAD; the commit
/// boundary applies the name and the one prototype only when
/// `--option elfmain on` (the default) lets this pass's output through the gate.
pub struct ElfMainPass;

impl AnalysisPass for ElfMainPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "elfmain"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        let Some((vma, named_here)) = main_claim(ctx.file, ctx.bytes) else {
            return out;
        };
        if named_here {
            // The name rides the `entry_names` overlay, which the commit consults
            // for VMAs in `entries`. Oracle 4 already discovers this address, but
            // emitting it here too keeps the name from silently evaporating if
            // that oracle is off or filters it.
            out.entries.push(vma);
            out.entry_names.push((vma, MAIN.to_string()));
        }
        if let Some(pieces) = main_prototype(ctx) {
            out.prototypes.push(pieces);
        }
        out
    }
}

/// The VMA crt1 hands `__libc_start_main`, once every refusal in the module
/// header has been applied, paired with whether THIS pass is the one that names
/// it (false when the address came from [`super::kuna_armlibcmain`], which owns
/// the entry and the name). `None` means this pass contributes nothing and the
/// commit is byte-identical to before.
fn main_claim(file: &object::File, bytes: &[u8]) -> Option<(u64, bool)> {
    if file.format() != object::BinaryFormat::Elf {
        return None;
    }
    // Both refusals come off one pass over the symbol names, and both are
    // cheaper than the decode and the section scans below -- on a large image
    // that already names its own `main` this is the whole cost of the pass.
    let (names_runtime, names_main) = scan_runtime_names(file);
    if !names_runtime || names_main {
        return None;
    }
    let entry = file.entry();
    let (vma, named_here) = match super::libc_start_main_target(file, entry) {
        Some(vma) => (vma, true),
        // The non-PIE ARM32 shape oracle 4's relocation cross-check cannot see.
        // That pass emits the entry and the name; only the prototype is added
        // here, and it is inert unless `armlibcmain` is on to install the `main`
        // it parks on.
        None => (super::kuna_armlibcmain::main_vma(file, bytes)?, false),
    };
    if vma == 0 || vma == entry & !1 {
        return None;
    }
    if !in_executable_section(&executable_sections(file), vma) {
        return None;
    }
    // A named entry has a better name coming from whatever named it, and an image
    // that already spells a symbol `main` would make the by-name prototype park
    // ambiguous.
    if existing_function_addrs(file, bytes).binary_search(&vma).is_ok() {
        return None;
    }
    Some((vma, named_here))
}

/// `(the image names __libc_start_main, the image names main)`, from one pass
/// over the static and dynamic symbol names.
///
/// The first is the evidence that `_start` really is a glibc crt1, and so that
/// its first argument is `main` rather than whatever a hand-rolled entry point
/// puts in that register. The second is what would make the by-name prototype
/// park ambiguous, and it is also how a non-stripped ELF says it has a better
/// name for this address already. The GNU version suffix is stripped because a
/// versioned import spells the name `__libc_start_main@GLIBC_2.34`.
fn scan_runtime_names(file: &object::File) -> (bool, bool) {
    let mut runtime = false;
    let mut main = false;
    for sym in file.symbols().chain(file.dynamic_symbols()) {
        let Ok(name) = sym.name_bytes() else { continue };
        if name == MAIN.as_bytes() {
            main = true;
        } else if !runtime && crate::loader::elf_plt::strip_version(name) == LIBC_START_MAIN {
            runtime = true;
        }
        if runtime && main {
            break;
        }
    }
    (runtime, main)
}

/// `int main(int argc, char **argv, char **envp)` — the declaration the C
/// runtime's call licenses, all three arguments of it (see the module header for
/// why the third one is not optional).
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
        intypes: vec![int4, std::rc::Rc::clone(&charpp), charpp],
        innames: vec!["argc".to_string(), "argv".to_string(), "envp".to_string()],
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

    fn claim(name: &str) -> Option<(u64, bool)> {
        let bytes = fixture(name);
        let file = object::File::parse(bytes.as_slice()).unwrap_or_else(|e| panic!("{name}: {e}"));
        main_claim(&file, bytes.as_slice())
    }

    /// The headline, in both link models: crt1 hands `main` over as a
    /// PC-relative `lea` in a PIE and as a bare immediate in an `ET_EXEC`, and
    /// the claim is the same either way.
    #[test]
    fn claims_the_libc_start_main_argument_of_a_stripped_elf() {
        // PIE: `lea rdi,[rip+0x286]` at 0x1178.
        assert_eq!(claim("stripped_dynamic_x86_64"), Some((0x1405, true)));
        // ET_EXEC: `mov rdi,0x40137a` at 0x401188.
        assert_eq!(claim("eh_lsda_x86_64"), Some((0x40137a, true)));
    }

    /// The same decode on the other three architectures oracle 4 covers, so the
    /// pass is not an x86-64 feature with three dead arms.
    #[test]
    fn claims_it_on_every_architecture_oracle_4_decodes() {
        assert_eq!(claim("entrymain_aarch64"), Some((0x714, true)));
        assert_eq!(claim("entrymain_riscv64"), Some((0x608, true)));
        assert_eq!(claim("entrymain_arm"), Some((0x4d8, true)));
    }

    /// The non-PIE ARM32 shape oracle 4 cannot see: `armlibcmain` owns the entry
    /// and the name, so only the prototype is contributed here.
    #[test]
    fn contributes_only_the_prototype_on_the_non_pie_arm_shape() {
        assert_eq!(claim("armlibcmain_le32"), Some((0x103dc, false)));
        assert_eq!(claim("armlibcmain_got_le32"), Some((0x10518, false)));
    }

    /// The guard that keeps this from overwriting better knowledge: the
    /// un-stripped image names that address `main` itself, and the pass refuses
    /// — which is why no unstripped-ELF expectation anywhere can move.
    #[test]
    fn defers_to_an_image_that_names_its_own_main() {
        assert_eq!(claim("fmt_x86_64"), None, "ET_EXEC, not stripped");
        assert_eq!(claim("cet_pie_x86_64"), None, "PIE, not stripped");
    }

    /// Structurally inert off ELF, and on an ELF that never names the C runtime
    /// entry the claim rests on.
    #[test]
    fn contributes_nothing_without_the_libc_runtime() {
        assert_eq!(claim("macho_stripped_main"), None, "Mach-O is machomain's");
        assert_eq!(claim("msvc_rtti_x64.exe"), None, "PE is entrymainproto's");
        assert_eq!(claim("unmapped_call_x86_64"), None, "static, stripped, no crt1 name");
        assert_eq!(claim("poolref_arm_le32"), None, "no __libc_start_main anywhere");
    }
}

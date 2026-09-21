//! (kuna `libctypes`) The stdio stream slots — `stdin`, `stdout`, `stderr` — as
//! typed `FILE *` data symbols.
//!
//! The rest of this table types a stream where a CALL says what it is. That
//! leaves the streams themselves untyped storage: the dynamic linker's `stdout`
//! is a `FILE *` in every image that imports it, and kuna learned that only
//! where the same function also called something like `__overflow(FILE *, int)`.
//! A function that merely hands `stdout` to one of the image's own helpers
//! learned nothing, and the two renderings of the same global disagreed inside
//! one binary.
//!
//! The claim here is a declaration, not an inference, so it is keyed on the
//! evidence that the slot really is the dynamic linker's and not a global the
//! program happens to spell `stdout`: the DYNAMIC RELOCATION that binds it.
//! Two shapes carry a stream, and they hold different things.
//!
//! ## `COPY` — the slot IS the stream pointer
//!
//! An executable that references `stdout` gets a `.bss` word of `sizeof(FILE *)`
//! and an `R_*_COPY` relocation naming it; the loader copies libc's own
//! `FILE *stdout` into it at start-up. The slot's type is `FILE *`.
//!
//! ```text
//! 000000000000c088  R_X86_64_COPY  stdout@GLIBC_2.2.5 + 0   (coreutils fmt, .bss)
//! ```
//!
//! ## `GLOB_DAT` on an UNDEFINED symbol — the slot holds the stream's ADDRESS
//!
//! A shared object cannot copy-relocate, so it reads the stream through its GOT:
//! the slot holds `&stdout`, and its type is `FILE **`. The name reflects that —
//! `stdout_ptr`, not `stdout` — because the address is not the stream and
//! calling it one would make the emitted `*stdout_ptr` read as a double
//! indirection that is not in the program.
//!
//! ```text
//! 000000000002bf38  R_X86_64_GLOB_DAT  stdout@GLIBC_2.2.5 + 0   (libselinux.so.1, .got)
//! ```
//!
//! ## What is declined
//!
//! A name the image DEFINES itself never reaches either arm: the `COPY` arm is
//! reached only through a copy relocation, which is by construction a claim
//! about a definition in another image, and the `GLOB_DAT` arm requires the
//! symbol to be undefined. A spelling that occurs more than once in `.dynsym` is
//! declined outright, and so is a `COPY` slot whose declared size is not the
//! image's pointer width.
//!
//! `stdout_ptr` is kuna's own coinage, so it gets the same treatment one step
//! further out: the `GLOB_DAT` arm declines when either symbol table already
//! spells the name it would mint. An image with its own `stdout_ptr` global
//! would otherwise print two different addresses as one identifier. `.symtab`
//! counts as much as `.dynsym` — a `static long stdout_ptr` reaches only the
//! former, and the loader's data symbols are named from it. The `FILE` shell
//! itself comes from
//! [`super::named_aggregate`], which declines when the name is already held by
//! something else — so an image whose own debug info defines a different `FILE`
//! contributes no stream symbol at all rather than a contradictory one.
//!
//! The slots mint that `FILE` at glibc's x86-64 width, so the pass calls in here
//! only on a target that width is true of (`glibc::target_takes_the_widths`).
//! The relocation numbering below still lists four architectures because it is
//! [`crate::loader::kuna_dynrelocs`]'s; the other three are never reached.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use kuna_base::types::uint4;
use kuna_decomp::dtype::TypeFactory;
use object::{elf, Object, ObjectSymbol, RelocationFlags, RelocationTarget};

use super::Layout;
use crate::pass::TypedDataFact;

/// The three stream names, in `.dynsym` spelling.
pub(super) const STREAM_NAMES: [&str; 3] = ["stdin", "stdout", "stderr"];

/// The `COPY` / `GLOB_DAT` `r_type` pair for `machine`, or `None` where kuna does
/// not know this architecture's dynamic relocation numbering. The same four
/// architectures [`crate::loader::kuna_dynrelocs`] handles.
fn copy_globdat(arch: object::Architecture) -> Option<(u32, u32)> {
    match arch {
        object::Architecture::X86_64 => Some((elf::R_X86_64_COPY, elf::R_X86_64_GLOB_DAT)),
        object::Architecture::Aarch64 => Some((elf::R_AARCH64_COPY, elf::R_AARCH64_GLOB_DAT)),
        object::Architecture::I386 => Some((elf::R_386_COPY, elf::R_386_GLOB_DAT)),
        object::Architecture::Arm => Some((elf::R_ARM_COPY, elf::R_ARM_GLOB_DAT)),
        _ => None,
    }
}

/// The minted `<stream>_ptr` spellings this image already answers to.
///
/// The `COPY` arm re-uses a name the image itself carries, but the `GLOB_DAT`
/// arm invents one, and an invented name is only a name while nothing else
/// holds it. Both tables are read: a `static long stdout_ptr` appears in
/// `.symtab` alone, and that is the table the loader's data symbols are named
/// from.
fn minted_names_taken(file: &object::File) -> HashSet<String> {
    let wanted: Vec<String> = STREAM_NAMES.iter().map(|n| format!("{n}_ptr")).collect();
    let mut taken = HashSet::new();
    let mut note = |sym: &object::Symbol| {
        let Ok(raw) = sym.name_bytes() else { return };
        let stripped = crate::loader::elf_plt::strip_version(raw);
        if let Some(hit) = wanted.iter().find(|w| w.as_bytes() == stripped.as_slice()) {
            taken.insert(hit.clone());
        }
    };
    for sym in file.symbols() {
        note(&sym);
    }
    for sym in file.dynamic_symbols() {
        note(&sym);
    }
    taken
}

/// The stream data symbols this image carries, typed.
///
/// Empty for anything but a linked ELF on one of the four handled
/// architectures, and empty when the `FILE` shell cannot be minted.
pub(super) fn stream_data_symbols(
    file: &object::File,
    types: &dyn TypeFactory,
    word_size: uint4,
    layout: Layout,
) -> Vec<TypedDataFact> {
    let mut out = Vec::new();
    if file.format() != object::BinaryFormat::Elf
        || file.kind() == object::ObjectKind::Relocatable
    {
        return out;
    }
    let Some((r_copy, r_glob_dat)) = copy_globdat(file.architecture()) else {
        return out;
    };
    let Some(relocs) = file.dynamic_relocations() else {
        return out;
    };

    // `.dynsym` by index, restricted to the three spellings. A spelling that
    // occurs more than once is dropped: the relocation would still say which
    // slot it binds, but a second entry means the image is not using the name
    // the way the C library does, and the cheap answer is to say nothing.
    let mut by_index: HashMap<usize, (&'static str, u64, bool)> = HashMap::new();
    let mut seen: HashMap<&'static str, u32> = HashMap::new();
    for sym in file.dynamic_symbols() {
        let Ok(raw) = sym.name_bytes() else { continue };
        let stripped = crate::loader::elf_plt::strip_version(raw);
        let Some(name) = STREAM_NAMES.iter().find(|n| n.as_bytes() == stripped.as_slice())
        else {
            continue;
        };
        *seen.entry(name).or_insert(0) += 1;
        by_index.insert(sym.index().0, (name, sym.size(), sym.is_undefined()));
    }
    by_index.retain(|_, (name, _, _)| seen.get(name) == Some(&1));
    if by_index.is_empty() {
        return out;
    }

    let Ok(stream) = super::named_aggregate("FILE", types, word_size, layout) else {
        return out;
    };
    let ptr = types.get_size_of_pointer();
    let Ok(stream_ptr) = types.get_type_pointer(ptr, stream, word_size) else {
        return out;
    };
    let ptr_width = ptr.max(0) as u64;

    let taken = minted_names_taken(file);
    let mut claimed: Vec<u64> = Vec::new();
    for (offset, reloc) in relocs {
        let RelocationFlags::Elf { r_type } = reloc.flags() else { continue };
        let RelocationTarget::Symbol(idx) = reloc.target() else { continue };
        let Some(&(name, size, undefined)) = by_index.get(&idx.0) else { continue };
        let (sym_name, ct) = if r_type == r_copy && !undefined && size == ptr_width {
            // The copy destination holds what libc's own `stdout` holds.
            (name.to_string(), Rc::clone(&stream_ptr))
        } else if r_type == r_glob_dat && undefined {
            // The GOT slot holds the ADDRESS of the stream pointer. The name is
            // minted here, so it is only usable while the image does not
            // already answer to it.
            let minted = format!("{name}_ptr");
            if taken.contains(&minted) {
                continue;
            }
            let Ok(pp) = types.get_type_pointer(ptr, Rc::clone(&stream_ptr), word_size) else {
                continue;
            };
            (minted, pp)
        } else {
            continue;
        };
        if claimed.contains(&offset) {
            continue;
        }
        claimed.push(offset);
        out.push(TypedDataFact { addr: offset, name: sym_name, type_: ct });
    }
    out
}

//! The note for a PPC32 secure-PLT PIC image whose import calls this run leaves
//! as `sub_`: only `ppc32picimports abi` proves the `r30` each stub reads, and it
//! is off unless the option, the mode or the environment sets it. Printed on
//! stderr with the runtime notes and carried in `functions --summary`.

use crate::runtime_hints::RuntimeHint;
use kuna_decomp::kuna_ppc32picimports::Mode;

/// Each noted subcommand that prints import names, with the mode it loads
/// under when `--mode` is absent (`None` is `auto`). `strings` and `read`
/// print none.
const SUBCOMMAND_MODES: &[(&str, Option<&str>)] = &[
    ("decompile", None),
    ("decompile-all", None),
    ("decompile-project", None),
    ("decompile-graph", None),
    ("functions", None),
    ("disassemble", None),
    ("xrefs", Some(crate::xrefs::DEFAULT_MODE)),
];

/// The note for `binary` under the run's final `options`, mode overrides included.
pub fn hint(binary: &str, options: &[(String, String)]) -> Option<RuntimeHint> {
    is_ppc32_elf(binary).then(|| stub_hint(binary, options)).flatten()
}

/// Print the note for one `kuna <sub> <rest..>` invocation on `binary`.
pub fn note_for_invocation(sub: &str, binary: &str, rest: &[String]) {
    if let Some(note) = run_options(sub, binary, rest).and_then(|options| stub_hint(binary, &options)) {
        eprintln!("note: {binary}: {}", note.hint);
    }
}

/// The options `kuna <sub>` runs `binary` with: its mode's overrides, then the
/// explicit `--option`s. `None` when the subcommand prints no import names or
/// `binary` is not a 32-bit PowerPC ELF, which is decided before the mode is.
fn run_options(sub: &str, binary: &str, rest: &[String]) -> Option<Vec<(String, String)>> {
    let &(_, default_mode) = SUBCOMMAND_MODES.iter().find(|(name, _)| *name == sub)?;
    if !is_ppc32_elf(binary) {
        return None;
    }
    let (mut mode, mut explicit) = (default_mode, Vec::new());
    let mut i = 0;
    while i < rest.len() {
        match rest[i].as_str() {
            "--mode" if i + 1 < rest.len() => {
                mode = Some(rest[i + 1].as_str());
                i += 1;
            }
            "--option" if i + 2 < rest.len() => {
                explicit.push((rest[i + 1].clone(), rest[i + 2].clone()));
                i += 2;
            }
            _ => {}
        }
        i += 1;
    }
    crate::decompile_all::mode_options_for_binary(mode, binary, explicit).ok()
}

/// Whether `binary` starts with an ELFCLASS32 `EM_PPC` header, from its first 20 bytes.
fn is_ppc32_elf(binary: &str) -> bool {
    use std::io::Read;
    let mut header = [0u8; 20];
    if std::fs::File::open(binary).and_then(|mut file| file.read_exact(&mut header)).is_err()
        || header[..4] != *b"\x7fELF"
        || header[4] != 1
    {
        return false;
    }
    let machine = [header[18], header[19]];
    let machine = if header[5] == 2 { u16::from_be_bytes(machine) } else { u16::from_le_bytes(machine) };
    machine == object::elf::EM_PPC
}

/// The note for a 32-bit PowerPC ELF `binary` under `options`.
fn stub_hint(binary: &str, options: &[(String, String)]) -> Option<RuntimeHint> {
    let mode = crate::loadtime::last_option_value(options, "ppc32picimports").map_or_else(Mode::from_env, Mode::from_env_value);
    if mode == Mode::Abi {
        return None;
    }
    let stubs = crate::image::inspect(binary, |bytes| {
        let file = object::File::parse(bytes).ok()?;
        Some(kuna_analysis::loader::kuna_ppc32_imports::pic_stub_count(&file))
    })?;
    (stubs > 0).then(|| RuntimeHint {
        id: "ppc32-pic-imports",
        version: None,
        hint: format!(
            "{stubs} PPC32 PIC import {}; --option ppc32picimports abi may name {} by proving r30 at every call (a whole-image walk at load: about 1.05-1.35x a function listing, up to 2.9x a single-function decompile)",
            if stubs == 1 { "stub prints as sub_" } else { "stubs print as sub_" },
            if stubs == 1 { "it" } else { "them" }
        ),
        actionable: true,
        crates: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> String {
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/ppc32-pic-imports.elf").to_owned()
    }

    #[test]
    fn every_noted_subcommand_is_resolved_or_prints_no_import_names() {
        for sub in crate::runtime_hints::NOTED_SUBCOMMANDS {
            let listed = SUBCOMMAND_MODES.iter().any(|(name, _)| name == sub);
            assert!(listed != matches!(*sub, "strings" | "read"), "{sub}");
        }
    }

    #[test]
    fn each_subcommand_resolves_the_mode_it_loads_under() {
        let image = fixture();
        let (option, value) = kuna_decomp::modes::mode_overrides("aggressive").unwrap()[0];
        let aggressive = |options: Option<Vec<(String, String)>>| {
            options.unwrap().iter().any(|(o, v)| o == option && v == value)
        };
        for (sub, _) in SUBCOMMAND_MODES {
            assert_eq!(aggressive(run_options(sub, &image, &[])), *sub != "xrefs", "{sub} under its default mode");
        }
        let explicit = ["--mode".to_owned(), "aggressive".to_owned()];
        assert!(aggressive(run_options("xrefs", &image, &explicit)));
        assert_eq!(run_options("strings", &image, &[]), None);
        let x86 = concat!(env!("CARGO_MANIFEST_DIR"), "/../kuna-analysis/tests/fixtures/fauxware");
        assert_eq!(run_options("functions", x86, &[]), None);
    }
}

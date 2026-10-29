//! Translate CLI options into the environment read during program loading.
//!
//! The console subprocess and in-process loader share these conversions. Runtime
//! option validation still runs after loading; these preserve the loader's legacy
//! fallback values, including its different boolean vocabularies.

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::process::Command;

use kuna_decomp::{
    kuna_calltargettype, kuna_dwarfstructs, kuna_dwarfvariants, kuna_dynrelocs, kuna_i386_pie_plt,
    kuna_ifuncfpret, kuna_libctypes, kuna_msvcfpconst, kuna_pdatachained, kuna_peordinal,
    kuna_goabi, kuna_goreg, kuna_gotypes, kuna_relocrebase, kuna_rexthunk, kuna_sourcelang, kuna_swiftdemangle,
    kuna_symbolnamebound, kuna_symbolnamechars,
    kuna_variantnames,
    kuna_symbolnamerepair, kuna_typedepth, options::RELOC_OBJECTS_ENV,
};

#[derive(Clone, Copy)]
enum Encoding {
    DefaultOn,
    DefaultOff,
    RelocObjects,
    Arm64e,
    SymbolNameChars,
    Trimmed,
    LibcTypes,
    SourceLang,
}

fn binding(name: &str) -> Option<(&'static str, Encoding)> {
    use Encoding::*;
    Some(match name {
        "relocobjects" => (RELOC_OBJECTS_ENV, RelocObjects),
        "i386_pie_plt" => (kuna_i386_pie_plt::I386_PIE_PLT_ENV, DefaultOn),
        "ifuncfpret" => (kuna_ifuncfpret::IFUNCFPRET_ENV, DefaultOff),
        "relocrebase" => (kuna_relocrebase::RELOCREBASE_ENV, DefaultOn),
        "dynrelocs" => (kuna_dynrelocs::DYNRELOCS_ENV, DefaultOn),
        "pdatachained" => (kuna_pdatachained::PDATACHAINED_ENV, DefaultOn),
        "rexthunk" => (kuna_rexthunk::REXTHUNK_ENV, DefaultOn),
        "goabi" => (kuna_goabi::GOABI_ENV, DefaultOn),
        "goreg" => (kuna_goreg::GOREG_ENV, DefaultOn),
        "peordinal" => (kuna_peordinal::PEORDINAL_ENV, DefaultOn),
        "msvcfpconst" => (kuna_msvcfpconst::MSVCFPCONST_ENV, DefaultOn),
        "symbolnamerepair" => (kuna_symbolnamerepair::SYMBOLNAMEREPAIR_ENV, DefaultOn),
        "symbolnamechars" => (kuna_symbolnamechars::SYMBOLNAMECHARS_ENV, SymbolNameChars),
        "symbolnamebound" => (kuna_symbolnamebound::SYMBOLNAMEBOUND_ENV, Trimmed),
        "typedepth" => (kuna_typedepth::TYPEDEPTH_ENV, DefaultOn),
        "libctypes" => (kuna_libctypes::LIBCTYPES_ENV, LibcTypes),
        "dwarfstructs" => (kuna_dwarfstructs::DWARFSTRUCTS_ENV, DefaultOn),
        "gotypes" => (kuna_gotypes::GOTYPES_ENV, DefaultOn),
        "dwarfvariants" => (kuna_dwarfvariants::DWARFVARIANTS_ENV, DefaultOn),
        "variantnames" => (kuna_variantnames::VARIANTNAMES_ENV, DefaultOn),
        "calltargettype" => (kuna_calltargettype::CALLTARGETTYPE_ENV, DefaultOff),
        "macho-arm64e" => ("KUNA_MACHO_ARM64E", Arm64e),
        "sourcelang" => (kuna_sourcelang::SOURCELANG_ENV, SourceLang),
        "swiftdemangle" => (kuna_swiftdemangle::SWIFTDEMANGLE_ENV, DefaultOn),
        _ => return None,
    })
}

impl Encoding {
    fn encode(self, value: &str) -> Option<String> {
        let token = value.trim().to_ascii_lowercase();
        let off = matches!(token.as_str(), "off" | "0" | "false");
        Some(match self {
            Self::DefaultOn => if off { "off" } else { "on" }.into(),
            Self::DefaultOff => if matches!(token.as_str(), "on" | "1" | "true" | "") {
                "on"
            } else {
                "off"
            }
            .into(),
            Self::RelocObjects => if off || token == "no" { "0" } else { "1" }.into(),
            Self::Arm64e => {
                return matches!(token.as_str(), "on" | "true" | "1" | "yes").then(|| "1".into());
            }
            Self::SymbolNameChars => kuna_symbolnamechars::NameChars::parse(value)
                .unwrap_or_default()
                .as_str()
                .into(),
            Self::Trimmed => value.trim().into(),
            Self::LibcTypes => {
                if off {
                    "off".into()
                } else {
                    token
                }
            }
            // A forced language travels as its token; `auto` (and any value the
            // console `option` arm will refuse anyway) clears the variable so
            // detection decides.
            Self::SourceLang => {
                return kuna_sourcelang::SourceLang::parse_forced(value)
                    .map(|lang| lang.as_str().into());
            }
        })
    }
}

pub(crate) fn is_loadtime_gate(name: &str) -> bool {
    binding(name).is_some()
}

pub(crate) fn last_option_value<'a>(
    options: &'a [(String, String)],
    name: &str,
) -> Option<&'a str> {
    options
        .iter()
        .rev()
        .find(|(key, _)| key == name)
        .map(|(_, value)| value.as_str())
}

fn settings(
    options: &[(String, String)],
    slice: Option<&str>,
    decode_lanes: usize,
) -> BTreeMap<&'static str, Option<String>> {
    let mut values = BTreeMap::new();
    for (name, value) in options {
        if let Some((key, encoding)) = binding(name) {
            values.insert(key, encoding.encode(value));
        }
    }
    if last_option_value(options, "setlanguage").is_some_and(|l| l.starts_with("rust")) {
        values.insert(kuna_calltargettype::CALLTARGETTYPE_ENV, Some("off".into()));
    }
    if let Some(slice) = slice.filter(|s| !s.trim().is_empty()) {
        values.insert("KUNA_MACHO_SLICE", Some(slice.into()));
    }
    if decode_lanes > 1 {
        values.insert(
            kuna_analysis::listing::kuna_pdecode::DECODE_JOBS_ENV,
            Some(decode_lanes.to_string()),
        );
    }
    values
}

pub(crate) fn apply_to_command(
    cmd: &mut Command,
    options: &[(String, String)],
    slice: Option<&str>,
) {
    for (name, value) in settings(options, slice, 1) {
        match value {
            Some(value) => cmd.env(name, value),
            None => cmd.env_remove(name),
        };
    }
}

pub(crate) fn apply_to_process(
    options: &[(String, String)],
    slice: Option<&str>,
    decode_lanes: usize,
) -> LoadtimeEnv {
    let mut env = LoadtimeEnv::default();
    for (name, value) in settings(options, slice, decode_lanes) {
        env.set(name, value.as_deref());
    }
    env
}

/// Restores inherited values when an in-process load returns or unwinds.
#[derive(Default)]
pub(crate) struct LoadtimeEnv {
    previous: Vec<(&'static str, Option<OsString>)>,
}

impl LoadtimeEnv {
    fn set(&mut self, name: &'static str, value: Option<&str>) {
        self.previous.push((name, std::env::var_os(name)));
        match value {
            Some(value) => std::env::set_var(name, value),
            None => std::env::remove_var(name),
        }
    }
}

impl Drop for LoadtimeEnv {
    fn drop(&mut self) {
        for (name, previous) in self.previous.drain(..).rev() {
            match previous {
                Some(value) => std::env::set_var(name, value),
                None => std::env::remove_var(name),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loader_value_vocabularies_are_preserved() {
        for (name, cases) in [
            (
                "typedepth",
                vec![
                    (" OFF ", Some("off")),
                    ("false", Some("off")),
                    ("no", Some("on")),
                    ("invalid", Some("on")),
                    ("", Some("on")),
                ],
            ),
            (
                "ifuncfpret",
                vec![
                    (" TRUE ", Some("on")),
                    ("", Some("on")),
                    ("yes", Some("off")),
                    ("invalid", Some("off")),
                ],
            ),
            (
                "relocobjects",
                vec![
                    (" NO ", Some("0")),
                    ("false", Some("0")),
                    ("invalid", Some("1")),
                ],
            ),
            (
                "macho-arm64e",
                vec![
                    (" YES ", Some("1")),
                    ("1", Some("1")),
                    ("", None),
                    ("off", None),
                    ("invalid", None),
                ],
            ),
            (
                "libctypes",
                vec![
                    (" GLIBC ", Some("glibc")),
                    ("0", Some("off")),
                    ("future", Some("future")),
                ],
            ),
            (
                "symbolnamebound",
                vec![(" 23 ", Some("23")), ("BAD", Some("BAD"))],
            ),
            (
                "symbolnamechars",
                vec![("off", Some("off")), ("invalid", Some("safe"))],
            ),
        ] {
            let (_, encoding) = binding(name).unwrap();
            for (input, expected) in cases {
                assert_eq!(
                    encoding.encode(input).as_deref(),
                    expected,
                    "{name} {input:?}"
                );
            }
        }
    }

    #[test]
    fn command_overrides_keep_last_values_and_explicit_removals() {
        let options = [
            ("typedepth".into(), "on".into()),
            ("typedepth".into(), "off".into()),
            ("macho-arm64e".into(), "off".into()),
            ("dwarfvariants".into(), "off".into()),
            ("unrelated".into(), "on".into()),
        ];
        let mut cmd = Command::new("unused");
        apply_to_command(&mut cmd, &options, Some("arm64"));
        let actual: BTreeMap<_, _> = cmd
            .get_envs()
            .map(|(k, v)| (k.to_str().unwrap(), v.map(|v| v.to_str().unwrap())))
            .collect();
        assert_eq!(
            actual,
            BTreeMap::from([
                ("KUNA_TYPEDEPTH", Some("off")),
                ("KUNA_MACHO_ARM64E", None),
                ("KUNA_MACHO_SLICE", Some("arm64")),
                ("KUNA_DWARFVARIANTS", Some("off")),
            ])
        );
        assert!(settings(&[], Some("  "), 1).is_empty());
        assert_eq!(
            settings(&[], None, 4)[kuna_analysis::listing::kuna_pdecode::DECODE_JOBS_ENV],
            Some("4".into())
        );
    }

    #[test]
    fn process_guard_restores_overwrites_and_removals_after_unwind() {
        const KEY: &str = "KUNA_CLI_LOADTIME_GUARD_TEST";
        let initial = std::env::var_os(KEY);
        let mut outer = LoadtimeEnv::default();
        outer.set(KEY, Some("inherited"));
        let result = std::panic::catch_unwind(|| {
            let mut inner = LoadtimeEnv::default();
            inner.set(KEY, Some("temporary"));
            inner.set(KEY, None);
            assert_eq!(std::env::var_os(KEY), None);
            panic!("load failed");
        });
        assert!(result.is_err());
        assert_eq!(std::env::var_os(KEY), Some("inherited".into()));
        drop(outer);
        assert_eq!(std::env::var_os(KEY), initial);
    }
}

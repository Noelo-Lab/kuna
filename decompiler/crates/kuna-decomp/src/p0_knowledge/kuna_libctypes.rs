//! (kuna `libctypes`) The named-libc-aggregate gate — spell a libc/POSIX
//! aggregate pointer in the built-in prototype tables (`FILE *`, `stat *`,
//! `DIR *`, `option *`, …) instead of the width-stable `void *` those slots
//! carry today.
//!
//! The prototype tables' [`Ty`](../../../kuna_analysis/protos) vocabulary is
//! deliberately width-stable, so every aggregate pointer in them is a `void *`.
//! That is honest about the width and silent about the pointee: a `fopen`
//! result, a `fclose` argument and an `__uflow` parameter are all `void *`, and
//! nothing downstream can tell them apart. The table entry is the only place the
//! pointee is known — `int fclose(FILE *)` is not an inference, it is the
//! declaration — so this gate teaches the table to say so.
//!
//! ## Why this is an env-var bridge, not an `Architecture` flag
//!
//! The named shells are interned into the type factory by the analysis pass at
//! `load file`, which runs **upstream of every `option` command** and, in
//! `kuna decompile-all`, upstream of `apply_runtime_options`: an
//! `Architecture` bool read inside the pass would see the constructor default
//! whatever the user asked for. So the gate is a process environment variable
//! ([`LIBCTYPES_ENV`]), consulted by the pass and by the declared-name lookup,
//! exported by the `kuna decompile`/`decompile-all` CLI before the load and set
//! by the console's `option libctypes …` arm, exactly as the sibling
//! `dwarfstructs`/`typedepth` gates do. The companion
//! `Architecture::analysis_libctypes` bool keeps the option catalog-visible and
//! makes `phase catalog`'s live `current` field reflect it.
//!
//! With the gate off the pass does not run at all, so not one named shell is
//! interned and the output is byte-identical to the `void *` tables alone.
//!
//! ## The third value
//!
//! `glibc` additionally installs the PUBLISHED x86-64 field layouts of the nine
//! aggregates whose members are part of the platform's API (`FILE`, `stat`,
//! `timespec`, `timeval`, `tm`, `passwd`, `group`, `option`, `dirent`), so a
//! field access renders under its own name and the loaded value carries the
//! field's type. The pass installs them only on a target the layouts are true of
//! — an x86-64 ELF whose `.dynstr` names glibc — and keeps the `opaque` shells
//! everywhere else.
//!
//! Default **opaque**: the table's own declaration is the strongest evidence
//! about a libc pointee that exists, and `void *` discards it. The 675
//! datatest assertions are structurally untouched (no datatest loads a file);
//! the evidence for the default is the corpus sweep recorded in
//! `docs/features/libctypes/record.json`. `--option libctypes off` restores the
//! byte-identical `void *` tables.

/// Environment variable that gates the named libc aggregate types (read by
/// `kuna-analysis::analyzers::protos::kuna_libctypes`). Absent or any value
/// other than the off-tokens ⇒ **enabled**; set to `off`/`0`/`false` ⇒ disabled
/// (the `void *` tables); set to `glibc` ⇒ enabled with the published field
/// layouts (see [`LibcTypesLayout`]).
pub const LIBCTYPES_ENV: &str = "KUNA_LIBCTYPES";

/// How much of an aggregate the table says.
///
/// The two enabled values differ only in what is inside the shell, never in
/// which slots are named: `opaque` interns a sized, fieldless, incomplete
/// struct, so `f->_IO_read_ptr` renders as `f->field_0x8`; `glibc` fills the
/// nine aggregates whose layout is PUBLIC in the installed headers with their
/// real x86-64 fields, so the same access renders under its own name and the
/// loaded value carries the field's type.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LibcTypesLayout {
    /// The `void *` tables, untouched — the pass does not run.
    Off,
    /// Named, sized, fieldless shells.
    Opaque,
    /// Named shells plus the published glibc x86-64 field layouts, on a target
    /// the layouts are true of (the decision is the analysis pass's:
    /// `kuna_libctypes::glibc::target_is_glibc_x86_64`).
    Glibc,
}

/// The layout this process asks for, from [`LIBCTYPES_ENV`]. Unset ⇒
/// [`LibcTypesLayout::Opaque`] (the shipped default); an unrecognised token is
/// read as `Opaque` rather than refused, because this is a bridge, not the
/// validator — `Architecture::set_kuna_option` rejects a bad value at the
/// `option` line, where the operator can see it.
pub fn libctypes_layout() -> LibcTypesLayout {
    match std::env::var(LIBCTYPES_ENV) {
        Ok(v) => match v.trim().to_ascii_lowercase().as_str() {
            "off" | "0" | "false" => LibcTypesLayout::Off,
            "glibc" => LibcTypesLayout::Glibc,
            _ => LibcTypesLayout::Opaque,
        },
        Err(_) => LibcTypesLayout::Opaque, // unset => default-on, opaque
    }
}

/// Whether the named libc aggregate types are enabled for this process.
/// Default **on**: only an explicit off-token in [`LIBCTYPES_ENV`] disables it.
pub fn libctypes_enabled() -> bool {
    libctypes_layout() != LibcTypesLayout::Off
}

/// Bridge a `set_kuna_option("libctypes", val)` choice to [`LIBCTYPES_ENV`] so a
/// later `load file` in the *same* process sees it (the interactive
/// `option libctypes glibc` before `load file` path; the CLI sets the env var on
/// the subprocess directly).
pub fn set_libctypes_env_layout(layout: LibcTypesLayout) {
    std::env::set_var(
        LIBCTYPES_ENV,
        match layout {
            LibcTypesLayout::Off => "off",
            LibcTypesLayout::Opaque => "opaque",
            LibcTypesLayout::Glibc => "glibc",
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The gate defaults on (unset env), only the off-tokens disable it, and
    /// `glibc` is the one token that asks for the field layouts. Serialized by
    /// being a single test (env is process-global).
    #[test]
    fn libctypes_env_gate() {
        std::env::remove_var(LIBCTYPES_ENV);
        assert!(libctypes_enabled(), "unset => default-on");
        assert_eq!(libctypes_layout(), LibcTypesLayout::Opaque, "unset => opaque");
        for off in ["off", "0", "false", "OFF", " off "] {
            std::env::set_var(LIBCTYPES_ENV, off);
            assert!(!libctypes_enabled(), "`{off}` should disable");
            assert_eq!(libctypes_layout(), LibcTypesLayout::Off);
        }
        for on in ["opaque", "on", "1", "true", "OPAQUE", " opaque ", ""] {
            std::env::set_var(LIBCTYPES_ENV, on);
            assert!(libctypes_enabled(), "`{on}` should enable");
            assert_eq!(libctypes_layout(), LibcTypesLayout::Opaque, "`{on}` is not a layout");
        }
        for g in ["glibc", "GLIBC", " glibc "] {
            std::env::set_var(LIBCTYPES_ENV, g);
            assert!(libctypes_enabled(), "`{g}` should enable");
            assert_eq!(libctypes_layout(), LibcTypesLayout::Glibc, "`{g}` asks for layouts");
        }
        for (layout, want) in [
            (LibcTypesLayout::Off, LibcTypesLayout::Off),
            (LibcTypesLayout::Opaque, LibcTypesLayout::Opaque),
            (LibcTypesLayout::Glibc, LibcTypesLayout::Glibc),
        ] {
            set_libctypes_env_layout(layout);
            assert_eq!(libctypes_layout(), want);
        }
        std::env::remove_var(LIBCTYPES_ENV);
    }
}

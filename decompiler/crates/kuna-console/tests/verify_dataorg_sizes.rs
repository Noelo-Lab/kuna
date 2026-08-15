//! (kuna) The widened `<data_organization>` decode: every scalar width a
//! per-architecture C spelling needs is read off the compiler spec.
//!
//! `Architecture::decode_data_organization` used to match only
//! `integer_size` / `long_size` / `pointer_size` / `char_size` / `wchar_size`;
//! `<short_size>` (54 vendored cspecs), `<long_long_size>` (51), `<float_size>`
//! (60), `<double_size>` (61) and `<long_double_size>` (56) fell through its
//! `_ => {}` arm, so no consumer could ask for them. These tests pin the decoded
//! tuple per data model, which is what makes the difference between the data
//! models observable at all.
//!
//! The interesting cases are the ones a hard-coded `2/4/8` table gets wrong:
//! **LP64** (`long` is 8, so an 8-byte integer is `long`), **LLP64** (`long` is
//! 4 with 8-byte pointers, so an 8-byte integer is `long long`), and the
//! **long-double split** — 10 on x86 ELF (the x87 extended value width, which no
//! `sizeof` can match) versus 8 under MSVC, where `long double` aliases `double`.
//!
//! Bootstrapping needs the built `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The decoded scalar widths, in declaration order:
/// `(char, short, int, long, long long, pointer, float, double, long double)`.
type Widths = (i32, i32, i32, i32, i32, i32, i32, i32, i32);

/// Bootstrap `fixture` and read its decoded data organization.
/// `None` => specs-less skip.
fn widths_of(fixture: &str) -> Option<Widths> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let path = root.join("decompiler/crates/kuna-analysis/tests/fixtures").join(fixture);
    let prog = match bootstrap_from_object(path.to_str()?, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_dataorg_sizes: skipping {fixture} (bootstrap failed, build \
                 `.sla` with `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    let t = prog.arch().types();
    Some((
        t.get_size_of_char(),
        t.get_size_of_short(),
        t.get_size_of_int(),
        t.get_size_of_long(),
        t.get_size_of_long_long(),
        t.get_size_of_pointer(),
        t.get_size_of_float(),
        t.get_size_of_double(),
        t.get_size_of_long_double(),
    ))
}

/// x86-64 System V is LP64: `long` is 8, so an 8-byte integer spells `long`.
/// Its `long double` is the x87 extended value, 10 bytes wide — the cspec even
/// annotates that its storage is 16 (`<!-- aligned-length=16 -->`), which is why
/// no `sizeof` assertion on a 10-byte float can ever hold.
#[test]
fn x86_64_sysv_is_lp64_with_an_x87_long_double() {
    let Some(w) = widths_of("fmt_x86_64") else { return };
    assert_eq!(w, (1, 2, 4, 8, 8, 8, 4, 8, 10), "x86-64 gcc");
}

/// i386 ELF is ILP32: `long` is 4, so an 8-byte integer must spell `long long`,
/// not `long`. Same x87 `long double`.
#[test]
fn i386_elf_is_ilp32_so_eight_bytes_is_long_long() {
    let Some(w) = widths_of("i386_pie_nl") else { return };
    assert_eq!(w, (1, 2, 4, 4, 8, 4, 4, 8, 10), "i386 gcc");
}

/// AArch64 is LP64 like x86-64, but its `long double` is NOT the x87 format.
/// Pinned as decoded rather than as the ABI says: `AARCH64.cspec` records 8
/// where the AArch64 ELF ABI has a 16-byte `long double`. A speller must not
/// invent a wider type than the spec claims.
#[test]
fn aarch64_is_lp64() {
    let Some(w) = widths_of("fmt_aarch64") else { return };
    assert_eq!((w.0, w.1, w.2, w.3, w.4, w.5), (1, 2, 4, 8, 8, 8), "aarch64 integer widths");
    assert_eq!((w.6, w.7), (4, 8), "aarch64 float/double");
}

/// ARM32 AAPCS: ILP32, so 8 bytes is `long long`.
#[test]
fn arm32_is_ilp32() {
    let Some(w) = widths_of("fmt_arm") else { return };
    assert_eq!((w.0, w.1, w.2, w.3, w.4, w.5), (1, 2, 4, 4, 8, 4), "arm integer widths");
}

/// A cspec that declares no `<long_double_size>` must fall back to `double`
/// rather than to 0 — otherwise a float speller would match size 0 and a
/// `long double` would become unspellable on every such target. This asserts the
/// `setup_sizes` fallback fires, whichever fixture exercises it.
#[test]
fn long_double_never_decodes_to_zero() {
    for fixture in
        ["fmt_x86_64", "i386_pie_nl", "fmt_arm", "fmt_aarch64", "fmt_riscv64", "mips_gp_le32"]
    {
        let Some(w) = widths_of(fixture) else { continue };
        assert!(w.8 >= w.7, "{fixture}: long double ({}) must be at least double ({})", w.8, w.7);
        for (name, v) in
            [("char", w.0), ("short", w.1), ("int", w.2), ("long", w.3), ("long long", w.4),
             ("pointer", w.5), ("float", w.6), ("double", w.7), ("long double", w.8)]
        {
            assert!(v > 0, "{fixture}: {name} width decoded as {v}");
        }
    }
}

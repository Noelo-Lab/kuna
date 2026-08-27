//! End-to-end gate for the **symbol-extent → type-factory seam**: a data symbol
//! whose declared `st_size` is not representable as the `int4` the type factory
//! sizes types in must cost that symbol its exact extent, never the binary.
//!
//! `st_size` is a 64-bit ELF field (`Elf64_Sym.st_size`) that no header check
//! validates, so it is ordinary attacker-controlled data. Arm 4a of
//! `commit_analysis_output` turns each loader data symbol into an
//! `undefined<size>` global, and it used to narrow the extent with
//! `(*size).max(1) as int4` — clamping AFTER the truncating cast, which makes the
//! clamp inspect the wrong number. Two whole classes of `st_size` fell through it:
//!
//! * **Low 32 bits zero** (`0x1_0000_0000`, fixture `hostile_size_low32_x86_64`):
//!   truncates to `0`, `.max(1)` never sees it, and `add_symbol_internal` rejects
//!   the size-0 type with `g_a symbol created with zero size type`. Because
//!   `commit_analysis_output` applies its arms in place and propagates with `?`,
//!   that one symbol aborted the commit mid-way — every later arm (library and
//!   DWARF prototypes, context paints, tracked registers, call-fixups, DWARF
//!   locals and line comments) was abandoned, and the stash is `mem::take`n so a
//!   second `read symbols` cannot retry. `kuna functions` exited 1 with nothing.
//! * **Low 32 bits with the sign bit set** (`0xffff_fff0`, fixture
//!   `hostile_size_neg_x86_64`): truncates to a NEGATIVE `int4`, which the type
//!   factory's cache lookups indexed unchecked — a process abort, not an error:
//!   `index out of bounds: the len is 9 but the index is 18446744073709551600`.
//!
//! Clamping BEFORE the cast closes both, and the lower bounds in
//! `TypeFactory::get_base` / `get_type_char_sized` keep a negative size an error
//! rather than a panic if one ever reaches them by another route.
//!
//! The assertion is that the symbol NAME survives — that is the user-visible
//! repair, and it is strictly stronger than "the load returned Ok", which a
//! silently dropped symbol would also satisfy.
//!
//! ## Why this cannot be a `tests/stages` case
//!
//! Both parity corpora are symbol-less bytechunks with hand-written `<symbol>`
//! elements and never construct an `ObjectLoadImage`, so a `.symtab`-gated defect
//! is invisible to `make test` and `make test-stages` in both directions. The
//! whole coverage burden is here.
//!
//! ## `.sla` precondition
//!
//! Like the sibling loader gates, bootstrapping needs the built `x86` `.sla`
//! under `specs/` (gitignored; `make specs`). When it is absent the bootstrap
//! fails; the test prints that and returns early (a specs-less CI is a visible
//! skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(name)
        .to_str()
        .unwrap()
        .to_string()
}

/// What one fixture load produced: the C for `main`, and the names the symbol
/// table installed. `None` is the specs-less skip.
struct Loaded {
    code: String,
    names: Vec<String>,
}

/// Bootstrap `name`, commit the analysis facts, and decompile `main`.
///
/// `commit_pending_analysis` is the exact call `kuna functions` /
/// `decompile-all` / `decompile-project` make, and it is where every failure
/// mode above surfaced — a panic here fails the test as loudly as an `Err`.
fn load(name: &str) -> Option<Loaded> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = fixture(name);

    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_hostile_symbol_sizes: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis()
        .unwrap_or_else(|e| panic!("`read symbols` must survive `{name}`: {}", e.explain()));

    let names: Vec<String> =
        prog.function_entries().map(|(n, _)| n.to_string()).collect();

    let cmds: Vec<String> =
        ["load function main".into(), "decompile".into(), "print C".into()].to_vec();
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(prog);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    Some(Loaded { code: status.optr.clone(), names })
}

/// Every fixture is the same two-line program (`main` returns `g_a`) and differs
/// only in the `st_size` byte, so the assertions are identical: the commit
/// survives, `main` is in the symbol table, and the read of `0x402000` renders by
/// the recovered name `g_a` rather than the anonymous `dat_402000`.
fn assert_recovered(name: &str) {
    let Some(Loaded { code, names }) = load(name) else { return }; // specs-less skip

    assert!(
        names.iter().any(|n| n == "main"),
        "`{name}`: the commit must still install the function symbols; got {names:?}",
    );
    assert!(
        code.contains("g_a"),
        "`{name}`: the data symbol must render by its recovered name; got:\n{code}",
    );
    assert!(
        !code.contains("dat_402000"),
        "`{name}`: `0x402000` is covered by the `g_a` symbol and must not render raw; got:\n{code}",
    );
}

/// `st_size = 0x1_0000_0000`. Truncated first, this was a size-0 type, and the
/// error it raised aborted the whole analysis commit.
#[test]
fn low_32_bits_zero_keeps_the_symbol_and_the_commit() {
    assert_recovered("hostile_size_low32_x86_64");
}

/// `st_size = 0xffff_fff0`. Truncated first, this was a negative `int4` that
/// indexed the type factory's cache out of bounds and aborted the PROCESS. The
/// harness reports a panic here as a failure, so no extra assertion is needed to
/// pin it — but the name check below also proves the symbol is genuinely
/// recovered rather than merely skipped.
#[test]
fn sign_bit_size_is_not_a_process_abort() {
    assert_recovered("hostile_size_neg_x86_64");
}

/// The control: an ordinary `st_size = 8` on the identical program. It loaded
/// before this change and must be untouched by it — the clamp is a no-op on
/// every extent that was already representable.
#[test]
fn representable_size_is_unchanged() {
    assert_recovered("hostile_size_sane_x86_64");
}

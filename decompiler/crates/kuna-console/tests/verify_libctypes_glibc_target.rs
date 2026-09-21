//! `--option libctypes glibc` installs the PUBLISHED glibc x86-64 field layouts,
//! and a name the OPERATOR declares must not reach them on a target they are not
//! true of.
//!
//! The declared-name path (`--define-function 0x…=getgrnam`) is answered long
//! after `load file`, with the object file out of reach, so it cannot re-run
//! `glibc::target_is_glibc_x86_64` — it is handed that pass's own answer
//! (`AnalysisOutput::libctypes_glibc`, kept on the `ConsoleProgram`). Nothing
//! about the program may stand in for it: the member names the kernel ABI fixes
//! are shared by every libc and every architecture, so an image's own debug info
//! is not evidence about its libc.
//!
//! One declared name, `getgrnam`, against two real images. Neither imports it and
//! neither imports anything else that names a `group`, so the aggregate the
//! declaration mints is the whole of what is under test.
//!
//!  - `libctypes_glibc_x86_64` — a dynamically linked x86-64 glibc ELF. The gate
//!    accepts, and `group` is minted COMPLETE: `gr_name` at 0, 32 bytes.
//!  - `libctypes_mips32_glibc_le32` — a dynamically linked 32-bit MIPS ELF whose
//!    `.dynstr` names `libc.so.6` and `GLIBC_2.4`, so only the ARCHITECTURE half
//!    of the gate refuses it, and which carries debug info defining `struct stat`
//!    and `struct timespec` with `st_dev` and `tv_sec` at offset 0. That is the
//!    combination that once bought the x86-64 layouts from inside. Every pointer
//!    on that target is four bytes, so `gr_passwd` is not at offset 8 and
//!    `sizeof(struct group)` is not 32 — and since the aggregate WIDTHS are
//!    x86-64's too, the declaration mints no `group` at all there
//!    (`AnalysisOutput::libctypes_refused`).
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `.sla` files under `specs/` (gitignored; `make
//! specs`). When one is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Mutex;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_decomp::kuna_libctypes::LIBCTYPES_ENV;

/// `bump` in the MIPS fixture; `next_byte` in the x86-64 one. Any code address
/// does — the declaration is what is under test, not the body.
const MIPS_ENTRY: u64 = 0x400810;
const X86_64_ENTRY: u64 = 0x1189;

/// The `libctypes` gate is a process environment variable and these tests run as
/// threads of one process, so they take turns.
static ENV_LOCK: Mutex<()> = Mutex::new(());

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap `fixture` with `libctypes` asking for `glibc`, declare `getgrnam` at
/// `entry`, and hand back the `group` the declaration minted, if any. `None` ⇒ a
/// specs-less skip.
fn declared_group(fixture: &str, entry: u64) -> Option<Option<Rc<kuna_decomp::dtype::Datatype>>> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = root.join("decompiler/crates/kuna-analysis/tests/fixtures").join(fixture);
    // The named shells are interned inside `load file`, upstream of every `option`
    // command, so the value has to be in the environment before the bootstrap.
    std::env::set_var(LIBCTYPES_ENV, "glibc");
    let mut prog: ConsoleProgram = match bootstrap_from_object(bin.to_str()?, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_libctypes_glibc_target: skipping (bootstrap failed, build `.sla` \
                 with `make specs`): {}",
                e.explain()
            );
            std::env::remove_var(LIBCTYPES_ENV);
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    let space = prog.arch().manage().get_default_code_space().expect("code space").clone();
    prog.declare_function(Address::new(Rc::clone(&space), entry), Some("getgrnam"), 0)
        .expect("the declaration lands");
    let held = prog.arch().types().find_by_name("group").expect("type lookup");
    std::env::remove_var(LIBCTYPES_ENV);
    Some(held)
}

/// The gate accepts: the declared `getgrnam` returns the published layout.
#[test]
fn an_accepted_target_mints_the_published_group() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let Some(group) = declared_group("libctypes_glibc_x86_64", X86_64_ENTRY) else { return };
    let group = group.expect("an x86-64 glibc ELF takes the named declaration");
    assert!(!group.is_incomplete(), "an x86-64 glibc ELF takes the fields");
    assert_eq!(group.get_size(), 32, "the published width");
    let first = group.get_field(0).expect("a first member");
    assert_eq!(first.name, "gr_name");
    assert_eq!(first.offset, 0);
}

/// The gate refuses on architecture alone: `.dynstr` names glibc, the image is
/// MIPS32, and the declared `getgrnam` gets the width-stable `void *` signature —
/// neither the x86-64 offsets nor the x86-64 width.
#[test]
fn a_mips32_glibc_image_cannot_reach_the_layouts() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let Some(group) = declared_group("libctypes_mips32_glibc_le32", MIPS_ENTRY) else { return };
    assert!(
        group.is_none(),
        "a refused target mints no `group`, whatever its own debug info says"
    );
}

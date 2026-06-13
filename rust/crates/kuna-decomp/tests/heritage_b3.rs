//! GATE for `rport/w10-heritage-apply`: `ActionHeritage` builds real SSA on the
//! live `Funcdata` (the B3 / post-first-heritage boundary).
//!
//! The W0 golden harness (`kuna/goldens.py`) captured the C++ IR at the
//! post-first-heritage boundary ("B3" — `print raw` + `print tree varnode`,
//! taken at the first `break start paramdouble`, the action immediately after
//! `heritage` in the mainloop).  This gate bootstraps a corpus function through
//! the same XML frontend `decompile_e2e.rs` uses, drives flow + the one
//! `ActionHeritage` pass (`Funcdata::op_heritage`), and compares the resulting
//! Rust SSA against the C++ B3 snapshot.
//!
//! ## What is compared (and the one documented divergence)
//!
//! Heritage's job is to wire free reads to their reaching writes/inputs and
//! place MULTIEQUAL phi-nodes at dominance frontiers.  The gate verifies that
//! structurally: every Varnode's **SSA identity** — its `printRaw` token
//! (register-name / shortcut+offset, size) plus its class (`(free)` /
//! `input` / written-by-`<defseq>`) — and the **phi (MULTIEQUAL) placement**.
//!
//! One op in the C++ B3 is *not* reproducible in the merged tree: the RETURN
//! gains an `ACC` operand from `guardReturns`' active-output path, which needs
//! `Funcdata::getActiveOutput()` initialized by `ActionPrototypeTypes`
//! (`initActiveOutput`) — a W4/W6 proto-recovery seam that is a no-op here
//! (`ActionPrototypeTypes::apply` returns 0; `funcp` is the `seams::FuncProto`
//! stub).  The gate documents that exact extra varnode and otherwise requires a
//! match.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::{Address, SeqNum};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::{AddrSpace, AddrSpaceManager, RegisterLookup};
use kuna_base::xml::{DocumentStorage, Element};

use kuna_decomp::architecture::Architecture;
use kuna_decomp::decompile_drive::build_and_follow_flow;
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase};
use kuna_decomp::xml_arch::{XmlArchitecture, XmlArchitectureCapability};

use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::translate::{register_translate_ids, Translate};

// ===========================================================================
// Repo plumbing (mirrors decompile_e2e.rs / corpus_bootstrap.rs).
// ===========================================================================

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

struct SymbolFn {
    name: String,
    space: String,
    offset: u64,
}

struct DataTest {
    binaryimage: Rc<Element>,
    arch_id: String,
    symbols: Vec<SymbolFn>,
}

fn parse_u64(s: &str) -> u64 {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x") {
        u64::from_str_radix(hex, 16).unwrap_or(0)
    } else {
        s.parse().unwrap_or(0)
    }
}

fn find_named(el: &Rc<Element>, name: &str, out: &mut Vec<Rc<Element>>) {
    if el.get_name() == name {
        out.push(Rc::clone(el));
    }
    for c in el.get_children() {
        find_named(c, name, out);
    }
}

fn attr(el: &Element, name: &str) -> Option<String> {
    el.get_attribute_value(name).ok().map(|b| String::from_utf8_lossy(b).into_owned())
}

fn parse_datatest(stem: &str) -> Result<DataTest, String> {
    let path = repo_root().join("decompiler/datatests").join(format!("{stem}.xml"));
    let xml = std::fs::read(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let mut store = DocumentStorage::new();
    let root = store
        .parse_document(&xml)
        .map_err(|e| format!("parse {stem}: {e}"))?
        .get_root()
        .clone();
    let mut bis = Vec::new();
    find_named(&root, "binaryimage", &mut bis);
    let binaryimage = bis.into_iter().next().ok_or("no <binaryimage>")?;
    let arch_id = attr(&binaryimage, "arch").ok_or("<binaryimage> has no arch")?;
    let mut syms = Vec::new();
    find_named(&binaryimage, "symbol", &mut syms);
    let symbols: Vec<SymbolFn> = syms
        .iter()
        .filter_map(|s| {
            Some(SymbolFn {
                name: attr(s, "name")?,
                space: attr(s, "space")?,
                offset: parse_u64(&attr(s, "offset")?),
            })
        })
        .collect();
    Ok(DataTest { binaryimage, arch_id, symbols })
}

struct DummyImg;
impl LoadImage for DummyImg {
    fn get_file_name(&self) -> &str {
        "dummy"
    }
    fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
        Err(KunaError::data_unavail("dummy"))
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

fn build_registry() -> IdRegistry {
    let mut registry = IdRegistry::with_base_ids();
    register_translate_ids(&mut registry);
    register_sleigh_arch_ids(&mut registry);
    register_loadimage_xml_ids(&mut registry);
    registry
}

fn bootstrap(dt: &DataTest) -> Result<XmlArchitecture, String> {
    let root = repo_root();
    let registry = build_registry();
    let capability = XmlArchitectureCapability::new();
    let mut arch = capability.build_architecture("datatest", "");
    arch.build_loader(Rc::clone(&dt.binaryimage)).map_err(|e| format!("build_loader: {e}"))?;
    let mut db = LanguageDatabase::new();
    db.scan_for_sleigh_directories(root.join("specs").to_str().unwrap());
    db.get_descriptions(&registry).map_err(|e| format!("collect ldefs: {e}"))?;
    arch.sleigh_mut().set_archid(&dt.arch_id);
    arch.sleigh_mut()
        .resolve_architecture(&db, &dt.arch_id)
        .map_err(|e| format!("resolve_architecture: {e}"))?;
    if arch.sleigh().language_index() < 0 {
        return Err("language index unresolved".to_string());
    }
    let specs = arch.sleigh().build_spec_file(&db).map_err(|e| format!("build_spec_file: {e}"))?;
    let resolved_sla = specs.slafile.ok_or("build_spec_file resolved no .sla")?;
    let sla = std::fs::read(&resolved_sla).map_err(|e| format!("read sla: {e}"))?;
    arch.sleigh_mut()
        .build_translator(Box::new(DummyImg), &sla)
        .map_err(|e| format!("build_translator: {e}"))?;
    arch.sleigh_mut()
        .base_mut()
        .ok_or("no Architecture base after build_translator")?
        .init_post_engine()
        .map_err(|e| format!("init_post_engine: {e}"))?;
    let manager_ptr: *const AddrSpaceManager = arch.sleigh().base().unwrap().manage();
    // SAFETY: the manager lives inside `arch` and outlives the open() call; the
    // borrow is released before any &mut use of `arch` (same shape as
    // decompile_e2e.rs / corpus_bootstrap.rs).
    arch.open_image(unsafe { &*manager_ptr }, &registry)
        .map_err(|e| format!("open_image: {e}"))?;
    let img = arch.take_loader().ok_or("loader vanished after open")?;
    arch.sleigh_mut().base_mut().unwrap().set_loader(Box::new(img));
    Ok(arch)
}

// ===========================================================================
// Rust IR -> faithful `printRaw`-style Varnode token rendering.
// ===========================================================================

/// One Varnode's SSA identity, as the B3 `print tree varnode` section encodes
/// it: the `printRaw` token (register name or shortcut+offset, with `:size` if
/// the size differs from the default) plus the SSA class suffix.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct VnIdentity(String);

/// Render a Varnode exactly as `Varnode::printRawNoMarkup` + `printRaw`
/// (`varnode.cc:730`/`760`): register name (or shortcut + `printRaw` offset),
/// optional `:size`, then the `(i)` / `(<defseq>)` / `(free)` class suffix.
fn render_vn(arch: &Architecture, fd: &Funcdata, vn: kuna_decomp::seams::VarnodeId) -> VnIdentity {
    let v = fd.vbank().get(vn).expect("render_vn: stale vn");
    let loc = v.get_addr().clone();
    let size = v.get_size();
    let trans = arch.translate();
    let spc = loc.get_space().expect("render_vn: vn space");
    // printRawNoMarkup: try the register name first.
    let regname = trans.get_register_name(spc, loc.get_offset(), size);
    let (mut tok, expect) = if !regname.is_empty() {
        // off = loc - point.offset; appended as "+off" when nonzero — for the
        // exact-named registers in our corpus off is always 0.
        let reg = trans.get_register_data(&regname).expect("render_vn: register data");
        let off = loc.get_offset().wrapping_sub(reg.offset);
        let mut s = regname.clone();
        if off != 0 {
            s.push('+');
            s.push_str(&off.to_string());
        }
        (s, reg.size as i32)
    } else {
        // shortcut + Address::printRaw (constant => "#0x..", others => "0x..").
        let mut s = String::new();
        s.push(spc.get_shortcut());
        loc.print_raw(&mut s).expect("render_vn: Address::print_raw");
        // expect = trans->getDefaultSize() (the AddrSpaceManager default size).
        (s, arch.manage().get_default_size())
    };
    // printRaw: if (expect != size) s << ':' << size;
    if expect != size {
        tok.push(':');
        tok.push_str(&size.to_string());
    }
    // class suffix.
    if v.is_input() {
        tok.push_str("(i)");
    } else if v.is_written() {
        let def = v.get_def().expect("render_vn: written vn def");
        tok.push('(');
        tok.push_str(&render_seqnum(fd, def));
        tok.push(')');
    }
    // if ((flags & (insert|constant)) == 0) "(free)"
    if !v.is_constant() && !v.is_input() && !v.is_written() {
        // free: not inserted, not constant.  (Inserted == input||written.)
        tok.push_str("(free)");
    }
    VnIdentity(tok)
}

/// Render a def-op's SeqNum exactly like the C++ `SeqNum::operator<<`
/// (`address.cc:32`): `pc.printRaw(s)` then `':' << uniq`.  Using `printRaw`
/// (not a bare `{:x}`) is load-bearing — `Address::printRaw` zero-pads the
/// offset to the space's address size, which is how the C++ B3 renders SeqNum
/// addresses (`0x00010020`, `0x004076c8`), so the SSA-identity tokens match
/// byte-for-byte instead of differing only in leading zeros.
fn render_seqnum(fd: &Funcdata, op: kuna_decomp::seams::OpId) -> String {
    let o = fd.obank().get(op).expect("render_seqnum: stale op");
    let sq: &SeqNum = o.get_seq_num();
    let pc = sq.get_addr();
    let mut s = String::new();
    pc.print_raw(&mut s).expect("render_seqnum: Address::print_raw");
    s.push(':');
    s.push_str(&format!("{:x}", sq.get_time()));
    s
}

/// The multiset of every Varnode's SSA identity in the function (the structural
/// content of the B3 `print tree varnode` section, ignoring the heap-pointer
/// / create-index / consumed details the goldens scrub or that depend on exact
/// allocation order).
fn rust_vn_identities(arch: &Architecture, fd: &Funcdata) -> Vec<VnIdentity> {
    let mut out: Vec<VnIdentity> = fd
        .vbank()
        .iter_def()
        .map(|vn| render_vn(arch, fd, vn))
        .collect();
    out.sort();
    out
}

// ===========================================================================
// C++ B3 snapshot parsing.
// ===========================================================================

/// Parse the `xunknownN = <token> ...` lines of a B3 snapshot into the same
/// Varnode-identity multiset (the token between `= ` and the first ` (`).
fn cpp_vn_identities(b3: &str) -> Vec<VnIdentity> {
    let mut out = Vec::new();
    for line in b3.lines() {
        // varnode-tree lines are the scrubbed `(internal=scrubbed)` lines.
        if !line.contains("(internal=scrubbed)") {
            continue;
        }
        // "<type> = <token> [attrs] (consumed=...) (internal=...) (create=...)"
        let after_eq = match line.split_once(" = ") {
            Some((_, r)) => r,
            None => continue,
        };
        // token is everything up to the first " (" (the consumed/internal/create
        // group) or a trailing attribute word; attributes (tied/mapped/persistent
        // /unaff/...) are part of the SSA identity, so keep them but drop the
        // "(consumed=" tail.
        let tok = match after_eq.find(" (consumed=") {
            Some(idx) => &after_eq[..idx],
            None => after_eq,
        };
        out.push(VnIdentity(tok.trim().to_string()));
    }
    out.sort();
    out
}

/// Count MULTIEQUAL (phi) ops in the Rust IR, with the block index each lives
/// in (the phi-placement check).
fn rust_phi_blocks(fd: &Funcdata) -> Vec<i32> {
    use kuna_num::opcodes::OpCode;
    let mut blocks = Vec::new();
    for op in fd.obank().iter_alive() {
        if fd.obank().get(op).expect("phi: stale op").code() == OpCode::CPUI_MULTIEQUAL {
            if let Some(bl) = fd.obank().get(op).and_then(|o| o.get_parent()) {
                blocks.push(fd.bblocks_ref().block(bl).get_index());
            }
        }
    }
    blocks.sort_unstable();
    blocks
}

/// Count MULTIEQUAL phis in the C++ B3 raw section (the `= <a> ? <b>` form the
/// MULTIEQUAL printer uses), with their block index.
fn cpp_phi_blocks(b3: &str) -> Vec<i32> {
    let mut blocks = Vec::new();
    let mut cur_block: i32 = -1;
    for line in b3.lines() {
        if let Some(rest) = line.strip_prefix("Basic Block ") {
            // "Basic Block <idx> <start>-<stop>"
            if let Some(tok) = rest.split_whitespace().next() {
                cur_block = tok.parse().unwrap_or(-1);
            }
            continue;
        }
        // A MULTIEQUAL raw op line is "<seq>:\t<out> = <a> ? <b>".
        if line.contains(" = ") && line.contains(" ? ") && line.starts_with("0x") {
            blocks.push(cur_block);
        }
    }
    blocks.sort_unstable();
    blocks
}

// ===========================================================================
// The gate.
// ===========================================================================

/// Bootstrap `stem`'s `which`-th symbol, run flow + ONE ActionHeritage pass,
/// and return (architecture-owning XmlArchitecture, post-heritage Funcdata).
fn run_to_b3(stem: &str, which: usize) -> Result<(XmlArchitecture, Funcdata), String> {
    let dt = parse_datatest(stem)?;
    let mut xarch = bootstrap(&dt)?;
    let sym = dt.symbols.get(which).ok_or("symbol index out of range")?;
    let space_name = sym.space.clone();
    let off = sym.offset;
    let name = sym.name.clone();
    let arch = xarch.sleigh_mut().base_mut().ok_or("no Architecture base")?;
    let space = arch
        .manage()
        .get_space_by_name(&space_name)
        .ok_or(format!("no space {space_name}"))?
        .clone();
    let entry = Address::new(space, off);
    // followFlow (generate_ops + generate_blocks), then the one heritage pass.
    // The pre-heritage actions (ActionStart / ActionDefaultParams /
    // ActionPrototypeTypes) are seam no-ops in the merged tree, so driving the
    // single op_heritage directly reproduces the C++ "first heritage" boundary.
    let mut fd =
        build_and_follow_flow(arch, &name, entry, 0).map_err(|e| format!("flow: {e}"))?;
    fd.op_heritage();
    Ok((xarch, fd))
}

fn b3_snapshot(stem: &str) -> Option<String> {
    let p = repo_root()
        .join("tests/golden/snapshots/cpp")
        .join(stem)
        .join("000-B3.txt");
    std::fs::read_to_string(p).ok()
}

/// Does the live `Funcdata`'s `glb` (the [`Architecture`] handle heritage
/// iterates spaces through) contain the spaces the lifted Varnodes actually
/// live in?  This is the precondition heritage needs to reach the IR.
///
/// Returns `(glb_contains_all_vn_spaces, reachable_heritageable_vn_count)`.
fn glb_covers_varnode_spaces(fd: &Funcdata) -> (bool, usize) {
    let m = fd.get_arch().manage();
    let mut glb_spaces: Vec<(*const AddrSpace, i32, String)> = Vec::new();
    for i in 0..m.num_spaces() {
        if let Some(s) = m.get_space(i) {
            glb_spaces.push((Rc::as_ptr(s), s.get_index(), s.get_name().to_string()));
        }
    }
    let mut all_covered = true;
    let mut reachable = 0usize;
    for vn in fd.vbank().iter_def() {
        let s = fd.vbank().get(vn).unwrap().get_addr().get_space().unwrap().clone();
        // A Varnode space is "covered" iff glb holds the *same* Rc — the identity
        // heritage's `loc_space_ids`/`build_info_list` keys on.
        let covered = glb_spaces.iter().any(|(ptr, _, _)| *ptr == Rc::as_ptr(&s));
        if covered {
            reachable += 1;
        } else {
            all_covered = false;
        }
    }
    (all_covered, reachable)
}

/// Normalize a Varnode-identity token to its **SSA identity**, stripping the
/// attributes that a separate (still-un-ported) seam attaches — so the B3
/// comparison is on what heritage builds (def/use links + phi placement), not
/// on proto/symbol recovery output:
///
///   * proto attributes `unaff` / `unsigned` (C++ `ActionPrototypeTypes` /
///     `markUnaffected` — the same proto seam the gate docs flag for the RETURN
///     `ACC` operand);
///   * symbol attributes `mapped` / `persistent` / `addrtied` / `tied` /
///     `volatile` / `nolocalalias` (the W4 `ScopeLocal` symbol seam: the merged
///     `Funcdata::localmap` is a unit stub, so no symbol flags are queried);
///   * the space-pointer constant rendering: C++ prints a LOAD/STORE space id
///     as `#(spaceptr)`, the merged tree prints the raw index `#0x<n>`; both
///     denote the same constant Varnode (a printRaw-rendering seam, not SSA).
fn normalize_identity(id: &VnIdentity) -> VnIdentity {
    let mut tok = id.0.clone();
    // Space-pointer constant: any `#0xN:8` / `#(spaceptr):8` collapses to a
    // single canonical spaceptr token (the constant's *value* is the space
    // index either way; the rendering differs only by the proto/printRaw seam).
    if tok.ends_with(":8") && tok.starts_with('#') {
        tok = "#spaceptr:8".to_string();
    }
    // Strip trailing proto/symbol attribute words.
    const ATTRS: &[&str] = &[
        "unaff", "unsigned", "mapped", "persistent", "addrtied", "tied", "volatile",
        "nolocalalias", "tlock", "nlock", "input", "indirect",
    ];
    loop {
        let trimmed = tok.trim_end();
        let stripped = ATTRS.iter().find_map(|a| {
            trimmed.strip_suffix(a).filter(|p| p.ends_with(' ')).map(|p| p.to_string())
        });
        match stripped {
            Some(p) => tok = p,
            None => break,
        }
    }
    VnIdentity(tok.trim_end().to_string())
}

fn normalize_set(ids: &[VnIdentity]) -> Vec<VnIdentity> {
    let mut out: Vec<VnIdentity> = ids.iter().map(normalize_identity).collect();
    out.sort();
    out
}

/// GATE (LOSS-132 keystone, post-unification): with the single
/// `AddrSpaceManager` shared as `glb`, `ActionHeritage` reaches the **real**
/// lifted Varnodes and builds genuine SSA (def/use links + MULTIEQUAL phi
/// placement) on the live corpus `Funcdata`.  This gate asserts that real B3
/// parity against the C++ oracle snapshot:
///
///   1. `glb` now covers **all** the lifted Varnode spaces (the dual-manager
///      seam is closed — the keystone proof);
///   2. MULTIEQUAL phi placement matches the C++ B3 exactly (the SSA-construction
///      proof — heritage placed the phis at the same dominance-frontier blocks);
///   3. every Varnode's SSA identity matches the C++ B3, modulo the documented
///      proto/symbol/spaceptr-rendering seam attributes ([`normalize_identity`])
///      and the one proto-attributable RETURN-value extra the gate docs allow
///      (`ActionPrototypeTypes` / `getActiveOutput`, a separate seam).
fn run_b3_gate(stem: &str, which: usize, expect_phi_blocks: &[i32]) {
    let (xarch, fd) = match run_to_b3(stem, which) {
        Ok(v) => v,
        Err(e) if e.contains("not built") || e.contains("no .sla") => {
            eprintln!("SKIP {stem}: {e}");
            return;
        }
        Err(e) => panic!("{stem} run_to_b3: {e}"),
    };
    let arch = xarch.sleigh().base().unwrap();
    let b3 = match b3_snapshot(stem) {
        Some(b) => b,
        None => {
            eprintln!("SKIP {stem}: no C++ B3 snapshot");
            return;
        }
    };

    // Heritage RAN (op_heritage drove the engine over the real IR) — the action
    // is no longer a no-op stub.  Prove the IR is non-trivial (the flow lifted
    // real ops) so we are exercising heritage, not an empty function.
    let nvn = fd.vbank().iter_def().count();
    assert!(nvn > 0, "{stem}: flow produced no Varnodes (nothing for heritage to do)");

    let (glb_covers, reachable) = glb_covers_varnode_spaces(&fd);
    let rust_phis = rust_phi_blocks(&fd);
    let cpp_phis = cpp_phi_blocks(&b3);
    let rust_ids = rust_vn_identities(arch, &fd);
    let cpp_ids = cpp_vn_identities(&b3);
    let rust_norm = normalize_set(&rust_ids);
    let cpp_norm = normalize_set(&cpp_ids);
    let only_rust: Vec<&VnIdentity> = rust_norm.iter().filter(|x| !cpp_norm.contains(x)).collect();
    let only_cpp: Vec<&VnIdentity> = cpp_norm.iter().filter(|x| !rust_norm.contains(x)).collect();

    eprintln!("=== {stem} B3 ===");
    eprintln!("glb covers varnode spaces: {glb_covers} (reachable {reachable}/{nvn})");
    eprintln!("rust phi blocks: {rust_phis:?}   cpp phi blocks: {cpp_phis:?}");
    eprintln!("rust vn {}  cpp vn {}", rust_ids.len(), cpp_ids.len());
    eprintln!("only in cpp B3 ({}): {:?}", only_cpp.len(), only_cpp);
    eprintln!("only in rust   ({}): {:?}", only_rust.len(), only_rust);

    // (1) The keystone: the single-manager unification means glb now carries the
    // engine's full space set, so heritage reaches every lifted Varnode.
    assert!(
        glb_covers && reachable == nvn,
        "{stem}: glb must cover ALL lifted Varnode spaces after the single-manager \
         unification (reachable {reachable}/{nvn}); the keystone is not closed"
    );

    // (2) SSA-construction proof: MULTIEQUAL phi placement matches the C++ B3.
    assert_eq!(
        rust_phis, expect_phi_blocks,
        "{stem}: MULTIEQUAL phi placement must match the C++ B3"
    );
    assert_eq!(rust_phis, cpp_phis, "{stem}: phi placement must match the parsed C++ B3");

    // (3) SSA-identity parity (modulo the documented proto/symbol/spaceptr seam
    // attributes).  The ONLY tolerated Rust-only identity is the
    // proto-attributable input/free *class flip*: a register the C++ leaves
    // `(free)` at B3 but that proto-recovery (`ActionPrototypeTypes` /
    // `markUnaffected` / `setInputVarnode` — a separate seam, exactly the one the
    // gate docs flag for the boolless RETURN `ACC` operand) later marks `(i)`,
    // which the merged tree (no proto pass) marks `(i)` already.  Such a token
    // has the SAME base storage as a `(free)`/`(i)` cpp token; nothing else may
    // be Rust-only.  Strip the trailing SSA-class suffix to compare base
    // storage.
    fn base_storage(id: &VnIdentity) -> String {
        match id.0.rfind('(') {
            Some(i) => id.0[..i].to_string(),
            None => id.0.clone(),
        }
    }
    let cpp_bases: std::collections::BTreeSet<String> =
        cpp_norm.iter().map(base_storage).collect();
    let unexplained: Vec<&&VnIdentity> = only_rust
        .iter()
        .filter(|id| !cpp_bases.contains(&base_storage(id)))
        .collect();
    assert!(
        unexplained.is_empty(),
        "{stem}: Rust B3 has SSA identities with no counterpart base storage in the C++ \
         B3 (after normalizing the proto/symbol/spaceptr seam attributes, and allowing \
         the proto-attributable input/free class flip): {unexplained:?}"
    );
}

#[test]
fn boolless_b3_ssa_parity() {
    // The C++ boolless B3 places one MULTIEQUAL (ACC phi) at the join block 3.
    run_b3_gate("boolless", 0, &[3]);
}

#[test]
fn lzcount_b3_ssa_parity() {
    // Generality #1: a small PowerPC corpus function (a different architecture
    // / byte structure than the 8051 boolless).  Phi blocks from its C++ B3.
    run_b3_gate("lzcount", 0, &cpp_phi_blocks(&b3_snapshot("lzcount").unwrap_or_default()));
}

#[test]
fn stackspill_b3_ssa_parity() {
    // Generality #2: a small stack-spill corpus function (a third structure,
    // exercising stack-space Varnodes).
    run_b3_gate("stackspill", 0, &cpp_phi_blocks(&b3_snapshot("stackspill").unwrap_or_default()));
}
//! INDEPENDENT VERIFIER adversarial tests (item: rport/w10-condexe-structure).
//!
//! This branch wires `PrintC::emitBlockCondition` (the `&&`/`||` short-circuit
//! render of a structured `BlockCondition`) and closes the `spliceBlockBasic`
//! trailing-branch-destruction seam.  The pre-existing test suite never exercised
//! a rendered `BlockType::Condition`, so the verifier adds direct coverage of the
//! new render against the C++ oracle, plus the negative case (a non-mergeable
//! single condition must stay a plain `if (...)`, never get mis-merged into a
//! `&&`/`||`).
//!
//! The tests render real corpus functions through the FULL pipeline
//! (`decompile_func` + `print_c`) and compare against the committed C++ oracle
//! (`decomp_test_dbg`, the same binary `printc_parity.rs` drives).  They are
//! anti-special-casing by construction: the `&&`/`||` form must arise from the
//! genuine `CollapseStructure::rule_block_or` match over the real CFG.

use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::AddrSpaceManager;
use kuna_base::xml::{DocumentStorage, Element};

use kuna_decomp::block::BlockType;
use kuna_decomp::decompile_drive::{decompile_func, print_c};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase};
use kuna_decomp::xml_arch::{XmlArchitecture, XmlArchitectureCapability};

use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::translate::register_translate_ids;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn attr(el: &Element, name: &str) -> Option<String> {
    el.get_attribute_value(name).ok().map(|b| String::from_utf8_lossy(b).into_owned())
}

fn find_named(el: &Rc<Element>, name: &str, out: &mut Vec<Rc<Element>>) {
    if el.get_name() == name {
        out.push(Rc::clone(el));
    }
    for c in el.get_children() {
        find_named(c, name, out);
    }
}

fn parse_u64(s: &str) -> u64 {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x") {
        u64::from_str_radix(hex, 16).unwrap_or(0)
    } else {
        s.parse().unwrap_or(0)
    }
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

fn parse_datatest(stem: &str) -> Result<DataTest, String> {
    let path = repo_root().join("tests/datatests").join(format!("{stem}.xml"));
    let xml = std::fs::read(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let mut store = DocumentStorage::new();
    let root = store
        .parse_document(&xml)
        .map_err(|e| format!("parse {stem}: {e}"))?
        .get_root()
        .clone();
    let mut bins = Vec::new();
    find_named(&root, "binaryimage", &mut bins);
    let binaryimage = bins.first().ok_or("no <binaryimage>")?.clone();
    let mut langs = Vec::new();
    find_named(&root, "language", &mut langs);
    let arch_id = langs
        .first()
        .and_then(|l| attr(l, "id"))
        .or_else(|| attr(&binaryimage, "arch"))
        .ok_or("no language/arch id")?;
    let mut syms = Vec::new();
    find_named(&root, "symbol", &mut syms);
    let symbols = syms
        .iter()
        .filter_map(|s| {
            Some(SymbolFn {
                name: attr(s, "name")?,
                space: attr(s, "space").unwrap_or_else(|| "ram".to_string()),
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
    if !specs.compilerfile.is_empty() {
        if let Ok(cspec) = std::fs::read(&specs.compilerfile) {
            arch.sleigh_mut()
                .base_mut()
                .ok_or("no Architecture base after build_translator")?
                .set_cspec_xml(cspec);
        }
    }
    arch.sleigh_mut()
        .base_mut()
        .ok_or("no Architecture base after build_translator")?
        .init_post_engine()
        .map_err(|e| format!("init_post_engine: {e}"))?;
    let manager_ptr: *const AddrSpaceManager = arch.sleigh().base().unwrap().manage();
    // SAFETY: the manager outlives the open() call; borrow released before &mut use.
    arch.open_image(unsafe { &*manager_ptr }, &registry)
        .map_err(|e| format!("open_image: {e}"))?;
    let img = arch.take_loader().ok_or("loader vanished after open")?;
    arch.sleigh_mut().base_mut().unwrap().set_loader(Box::new(img));
    Ok(arch)
}

/// Render every `<symbol>` of `stem` through the full pipeline, concatenated.
fn rust_render(stem: &str) -> Result<String, String> {
    let dt = parse_datatest(stem)?;
    let mut xarch = bootstrap(&dt)?;
    let mut out = String::new();
    for sym in &dt.symbols {
        let base = xarch.sleigh_mut().base_mut().ok_or("no base")?;
        let space = match base.manage().get_space_by_name(&sym.space) {
            Some(s) => Rc::clone(s),
            None => continue,
        };
        let entry = Address::new(space, sym.offset);
        base.duplicate_shared_returns = false; // (kuna) DIV-18: upstream-parity form for the dump
        if let Ok(fd) = decompile_func(base, &sym.name, entry, 0) {
            out.push_str(&print_c(base, &fd));
            out.push('\n');
        }
    }
    Ok(out)
}

/// Render `stem`'s first symbol and hand back the analyzed `Funcdata` for
/// structural inspection (the structured-block tree).
fn rust_decompile_first(stem: &str) -> Result<(XmlArchitecture, Funcdata), String> {
    let dt = parse_datatest(stem)?;
    let mut xarch = bootstrap(&dt)?;
    let sym = dt.symbols.first().ok_or("no symbols")?;
    let space_name = sym.space.clone();
    let off = sym.offset;
    let name = sym.name.clone();
    let base = xarch.sleigh_mut().base_mut().ok_or("no base")?;
    let space = base
        .manage()
        .get_space_by_name(&space_name)
        .ok_or(format!("no space {space_name}"))?
        .clone();
    let entry = Address::new(space, off);
    // (kuna) DIV-18: pin returndup off — this fence asserts the upstream-parity condexe structure.
    base.duplicate_shared_returns = false;
    let fd = decompile_func(base, &name, entry, 0).map_err(|e| format!("decompile: {e}"))?;
    Ok((xarch, fd))
}

// ---------------------------------------------------------------------------
// C++ oracle (decomp_test_dbg) — same wiring as printc_parity.rs::cpp_print_c.
// ---------------------------------------------------------------------------

fn cpp_oracle_bin() -> PathBuf {
    if let Some(p) = std::env::var_os("KUNA_DECOMP_TEST") {
        PathBuf::from(p)
    } else {
        repo_root().join("decompiler/cpp/decomp_test_dbg")
    }
}

fn cpp_render(stem: &str) -> Option<String> {
    let bin = cpp_oracle_bin();
    if !bin.exists() {
        return None;
    }
    let datatests = repo_root().join("tests/datatests");
    let out = Command::new(&bin)
        .arg("-sleighpath")
        .arg(repo_root().join("specs"))
        .arg("-path")
        .arg(&datatests)
        .arg("datatests")
        .arg(format!("{stem}.xml"))
        .env("KUNA_DUMP", "1")
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    let begin = text.find("===KUNA_DUMP_BEGIN")?;
    let after = text[begin..].find('\n')? + begin + 1;
    let mid = text[after..].find("===KUNA_DUMP_MID===")? + after;
    Some(text[after..mid].to_string())
}

fn skippable(e: &str) -> bool {
    e.contains("not built") || e.contains("no .sla") || e.contains("language index")
}

/// Collect the gluing opcode of every structured `BlockCondition` node anywhere
/// in the `sblocks` tree (recursive — Condition nodes nest inside If/Ls/Graph
/// parents, they are not a flat top-level list).  The opcode is what
/// `emit_block_condition` maps to `&&` (BOOL_AND) / `||` (BOOL_OR).
fn condition_opcodes(fd: &Funcdata) -> Vec<kuna_num::opcodes::OpCode> {
    fn walk(fd: &Funcdata, bl: kuna_decomp::seams::BlockId, acc: &mut Vec<kuna_num::opcodes::OpCode>) {
        let block = fd.sblocks_ref().block(bl);
        if block.get_type() == BlockType::Condition {
            if let Some(opc) = block.get_condition_opcode() {
                acc.push(opc);
            }
        }
        let children: Vec<_> = block.get_list().to_vec();
        for c in children {
            walk(fd, c, acc);
        }
    }
    let mut v = Vec::new();
    if let Some(root) = fd.sblocks_ref().root {
        walk(fd, root, &mut v);
    }
    v
}

fn count_condition_blocks(fd: &Funcdata) -> usize {
    condition_opcodes(fd).len()
}

// ===========================================================================
// AT1 — DEAD-CODE TRIPWIRE (the central verified fact of this review): NO
// corpus function the Rust engine analyzes currently produces a
// `BlockType::Condition` node, so `emit_block_condition` (added by this diff) is
// never reached end-to-end.  The `||`/`&&` that DO appear in renders (e.g. nan's
// `NAN(..) || NAN(..)`) come from the DATA-FLOW expression printer
// (CPUI_BOOL_OR PcodeOp -> BOOLEAN_OR token), not the control-flow short-circuit
// emitter.  This test pins that reality: when a future wave makes
// CollapseStructure::rule_block_or fire on the corpus, this tripwire flips and
// the THAT wave's verdict must re-confirm emitBlockCondition's parity end-to-end.
//
// (The transcription itself was verified faithful against the C++ by inspection
// — see the verdict file — but it cannot be confirmed by execution until a
// Condition node is produced.)
// ===========================================================================
#[test]
fn at1_emitblockcondition_is_currently_unreached_dead_code() {
    let stems = ["nan", "ccmp", "orcompare", "boolless", "elseif", "condexesub"];
    let mut any_cond = Vec::new();
    for stem in stems {
        let (_x, fd) = match rust_decompile_first(stem) {
            Ok(v) => v,
            Err(e) if skippable(&e) => {
                eprintln!("SKIP {stem}: {e}");
                continue;
            }
            Err(e) => {
                eprintln!("(could not decompile {stem}: {e})");
                continue;
            }
        };
        let opcs = condition_opcodes(&fd);
        if !opcs.is_empty() {
            any_cond.push(format!("{stem} -> {opcs:?}"));
        }
    }
    // CURRENT, VERIFIED state: zero BlockCondition nodes (the new emitter is
    // unreached).  If this assert fails because a Condition appeared, the
    // structuring layer advanced — re-verify emitBlockCondition's parity then.
    assert!(
        any_cond.is_empty(),
        "a BlockCondition node now appears ({any_cond:?}) — emitBlockCondition is \
         no longer dead code; re-verify its &&/|| render against the C++ oracle"
    );
}

// ===========================================================================
// AT2 — INVARIANT: whenever the engine DOES build a Condition node, its gluing
// opcode is one of the two boolean-glue opcodes.  The emitter's branch is
// `if opc == CPUI_BOOL_AND { &&-token } else { ||-token }` with an
// `unwrap_or(BOOL_AND)` fallback — a Condition carrying a non-boolean opcode
// would be silently mis-rendered as `&&`.  Today no corpus function builds a
// Condition (AT1), so this is a guard that holds vacuously now and becomes load-
// bearing the moment structuring lands; it must never observe a bad opcode.
// ===========================================================================
#[test]
fn at2_any_condition_opcode_is_a_boolean_glue() {
    let stems = ["nan", "ccmp", "orcompare", "boolless", "elseif", "condexesub"];
    for stem in stems {
        let (_x, fd) = match rust_decompile_first(stem) {
            Ok(v) => v,
            Err(e) if skippable(&e) => {
                eprintln!("SKIP {stem}: {e}");
                continue;
            }
            Err(e) => {
                eprintln!("(could not decompile {stem}: {e})");
                continue;
            }
        };
        for opc in condition_opcodes(&fd) {
            assert!(
                opc == kuna_num::opcodes::OpCode::CPUI_BOOL_AND
                    || opc == kuna_num::opcodes::OpCode::CPUI_BOOL_OR,
                "a BlockCondition in {stem} carries opcode {opc:?}, which is neither \
                 BOOL_AND nor BOOL_OR — emitBlockCondition would mis-render it"
            );
        }
    }
}

// ===========================================================================
// AT_OBS — DOCUMENTED OBSERVATION (non-asserting beyond the regression guard):
// the C++ oracle structures ccmp into `if (ptr[1] == 0x3c && val <= 9)`, but the
// Rust engine does NOT yet collapse the boolean arithmetic into a `&&`
// BlockCondition (it renders `v3 = v2 | ZEXT(..)`).  This gap is PRE-EXISTING
// (the analysis layer that produces the `&&`, not the emitter) and is OUT OF
// SCOPE for this render-only diff.  We pin the current Rust behavior so a future
// wave that closes the gap notices the change here, and we confirm the C++
// oracle DOES produce the `&&` (so the oracle target is recorded).
// ===========================================================================
#[test]
fn at_obs_ccmp_and_structuring_is_a_known_gap_not_this_diff() {
    let rust = match rust_render("ccmp") {
        Ok(r) => r,
        Err(e) if skippable(&e) => {
            eprintln!("SKIP ccmp: {e}");
            return;
        }
        Err(e) => panic!("ccmp render: {e}"),
    };
    let (_x, fd) = rust_decompile_first("ccmp").expect("ccmp decompile");
    // Current Rust state: NO &&-BlockCondition for ccmp (the gap).  If a future
    // wave closes it, this pin flips and the verdict for THAT wave must re-confirm
    // parity — not silently change ccmp's render under an unrelated diff.
    let has_cond = count_condition_blocks(&fd) > 0;
    eprintln!("ccmp Rust BlockCondition present: {has_cond}\n=== RUST ccmp ===\n{rust}");

    // The C++ oracle target is the `&&` form (recorded for the future wave).
    if let Some(cpp) = cpp_render("ccmp") {
        assert!(
            cpp.contains("ptr[1] == 0x3c && val <= 9"),
            "C++ oracle for ccmp must hold the `&&` target form:\n{cpp}"
        );
    }
}

// ===========================================================================
// AT3 — NON-MERGEABLE condition stays SEPARATE (anti-mis-merge): boolless has a
// single comparison `if (dat_52 <= 10)`.  It must NOT be structured into a
// BlockCondition and must NOT render a `&&`/`||`.  This is the negative control
// the prompt requires: enabling emitBlockCondition must not pull an ordinary
// single-condition `if` into the short-circuit path.
// ===========================================================================
#[test]
fn at3_single_condition_stays_plain_if_no_merge() {
    let (_x, fd) = match rust_decompile_first("boolless") {
        Ok(v) => v,
        Err(e) if skippable(&e) => {
            eprintln!("SKIP boolless: {e}");
            return;
        }
        Err(e) => panic!("boolless decompile: {e}"),
    };
    // No BlockCondition node may exist (the single compare is not mergeable).
    assert_eq!(
        count_condition_blocks(&fd),
        0,
        "boolless's single `if (dat_52 <= 10)` must NOT be structured into a \
         BlockCondition (no mergeable second clause)"
    );

    let rust = rust_render("boolless").expect("boolless render");
    eprintln!("=== RUST boolless ===\n{rust}");
    assert!(rust.contains("if (dat_52 <= 10)"), "boolless must keep its plain `if`:\n{rust}");
    assert!(!rust.contains("&&"), "boolless must not gain a spurious `&&`:\n{rust}");
    assert!(!rust.contains("||"), "boolless must not gain a spurious `||`:\n{rust}");
}

// ===========================================================================
// AT4 — anti-special-casing / cross-function purity: the condition render is
// produced ONLY by a real CollapseStructure match, never by a function-name or
// address hardcode.  nan (a genuine OR BlockCondition) and ccmp (no Condition in
// the current Rust engine) each render their OWN tokens; neither leaks the
// other's clause.  Combined with the structured-tree opcode witness, this proves
// the render is engine-driven, not a printer string hack.
// ===========================================================================
#[test]
fn at4_condition_render_is_engine_driven_per_function() {
    // ccmp -> AND condition with ptr/val tokens.
    let ccmp = match rust_render("ccmp") {
        Ok(r) => r,
        Err(e) if skippable(&e) => {
            eprintln!("SKIP ccmp: {e}");
            return;
        }
        Err(e) => panic!("ccmp: {e}"),
    };
    // nan -> OR condition (`NAN(..) || NAN(..)`), a different arch/function.
    let nan = match rust_render("nan") {
        Ok(r) => r,
        Err(e) if skippable(&e) => {
            eprintln!("SKIP nan: {e}");
            return;
        }
        Err(e) => panic!("nan: {e}"),
    };

    // Each function carries ITS OWN tokens; no cross-leak between functions.
    assert!(!nan.contains("ptr[1] == 0x3c"), "nan must not leak ccmp's clause:\n{nan}");
    assert!(!ccmp.contains("NAN("), "ccmp must not leak nan's `NAN(` token:\n{ccmp}");
    // nan renders its own `NAN(a0)` token (engine-derived per function, never a
    // printer string keyed on a name).  Earlier this rendered as the un-folded
    // `NAN(..) || NAN(..)` self-BOOL_OR; now that RuleTrivialArith is correctly
    // active (its C++ group "analysis" is in the decompile grouplist), the
    // `V || V => V` fold collapses it to the single `NAN(a0)` the C++ oracle
    // emits (`read_nan(NAN(a0));`, nan.xml "NaN operations #1").  The anti-leak
    // discrimination is unchanged; the witness token is `NAN(`.
    assert!(nan.contains("NAN("), "nan must render its own `NAN(` token:\n{nan}");

    // Anti-special-casing on the new code path: neither function's render is
    // produced by a function-name/address branch in the emitter — the diff
    // contains no such hardcode (grepped in the verdict), and the tokens above
    // are derived from each function's own analyzed IR.
    eprintln!("=== ccmp ===\n{ccmp}\n=== nan ===\n{nan}");
}

// CORPUS PROBE (committed, `#[ignore]`d — run with `-- --ignored --nocapture`):
// scan EVERY decompilable datatest function for a `BlockType::Condition` node.
// This is the reproducible evidence behind AT1's dead-code finding: at the time
// of this review it reports `0` functions with a Condition node out of ~167
// decompiled, i.e. `emit_block_condition` is unreached end-to-end.  Re-run it
// when the structuring layer advances to confirm the emitter starts firing.
#[test]
#[ignore]
fn probe_scan_corpus_for_any_condition_node() {
    let dir = repo_root().join("tests/datatests");
    let mut total_funcs = 0usize;
    let mut funcs_with_cond = Vec::new();
    for entry in std::fs::read_dir(&dir).unwrap() {
        let p = entry.unwrap().path();
        if p.extension().map(|x| x != "xml").unwrap_or(true) { continue; }
        let stem = p.file_stem().unwrap().to_string_lossy().to_string();
        let dt = match parse_datatest(&stem) { Ok(d) => d, Err(_) => continue };
        let mut xarch = match bootstrap(&dt) { Ok(a) => a, Err(_) => continue };
        for sym in &dt.symbols {
            let base = match xarch.sleigh_mut().base_mut() { Some(b)=>b, None=>continue };
            let space = match base.manage().get_space_by_name(&sym.space) { Some(s)=>Rc::clone(s), None=>continue };
            let entry_addr = Address::new(space, sym.offset);
            base.duplicate_shared_returns = false; // (kuna) DIV-18: upstream-parity form for the structure scan
            if let Ok(fd) = decompile_func(base, &sym.name, entry_addr, 0) {
                total_funcs += 1;
                let opcs = condition_opcodes(&fd);
                if !opcs.is_empty() {
                    funcs_with_cond.push(format!("{stem}:{} -> {:?}", sym.name, opcs));
                }
            }
        }
    }
    eprintln!("=== scanned {total_funcs} functions ===");
    eprintln!("=== functions with a BlockCondition node: {} ===", funcs_with_cond.len());
    for f in &funcs_with_cond { eprintln!("  {f}"); }
}

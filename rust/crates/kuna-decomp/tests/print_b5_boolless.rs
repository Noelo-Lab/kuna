//! GATE for `rport/w10-structure-printbody`: drive `boolless` print C (the B5
//! boundary) through the now-wired block-structuring, explicit/implied marking,
//! and IR-coupled PrintC body driver, and show the structured body it emits
//! beside the C++ B5 oracle.
//!
//! Before this wave `print C` emitted only the signature shell plus a `WARNING`
//! comment (`sblocks` empty, body emission seamed).  This gate asserts that the
//! structured body now appears (an `if` statement, not just the shell), that the
//! comparison condition renders the global as `dat_52` (the explicit
//! unnamed-location naming) inside `(...)`, and that the function is still
//! brace-matched and carries the function name.
//!
//! Full byte-parity with the C++ B5 oracle (`if (dat_52 <= 10)` + the recovered
//! `v1` local + the `<= ` join) is gated on the NEXT layer (ConditionalJoin /
//! RuleLessEqual collapsing the two-compare boolean pattern into one
//! `INT_LESSEQUAL`, and Merge/naming binding the `v1` local) — reported in the
//! item's seams_remaining, not asserted here.
use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::{Address, SeqNum};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::{AddrSpaceManager, RegisterLookup};
use kuna_base::xml::{DocumentStorage, Element};

use kuna_decomp::architecture::Architecture;
use kuna_decomp::decompile_drive::decompile_func;
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase};
use kuna_decomp::xml_arch::{XmlArchitecture, XmlArchitectureCapability};

use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::translate::register_translate_ids;

// ===========================================================================
// Repo plumbing (mirrors deadcode_b4.rs, plus the cspec read for proto recovery).
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
    // Hand the resolved cspec XML to the architecture so build_default_proto
    // decodes the real <default_proto> input/output param lists (proto recovery).
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

// ===========================================================================
// Op-listing render (coarse "print raw"-ish op line; mirrors deadcode_b4.rs).
// ===========================================================================

fn render_vn(arch: &Architecture, fd: &Funcdata, vn: kuna_decomp::seams::VarnodeId) -> String {
    let v = match fd.vbank().get(vn) {
        Some(v) => v,
        None => return "<stale>".to_string(),
    };
    let loc = v.get_addr().clone();
    let size = v.get_size();
    let trans = arch.translate();
    let spc = match loc.get_space() {
        Some(s) => s,
        None => return "<nospace>".to_string(),
    };
    let regname = trans.get_register_name(spc, loc.get_offset(), size);
    let mut tok = if !regname.is_empty() {
        regname
    } else {
        let mut s = String::new();
        s.push(spc.get_shortcut());
        let _ = loc.print_raw(&mut s);
        s
    };
    let expect = arch.manage().get_default_size();
    if expect != size {
        tok.push(':');
        tok.push_str(&size.to_string());
    }
    tok
}

fn render_op(arch: &Architecture, fd: &Funcdata, op: kuna_decomp::seams::OpId) -> String {
    let o = fd.obank().get(op).expect("render_op: stale op");
    let sq: &SeqNum = o.get_seq_num();
    let mut s = String::new();
    let _ = sq.get_addr().print_raw(&mut s);
    s.push(':');
    s.push_str(&format!("{:x}", sq.get_time()));
    s.push_str(":\t");
    if let Some(out) = o.get_out() {
        s.push_str(&render_vn(arch, fd, out));
        s.push_str(" = ");
    }
    s.push_str(&format!("{:?}", o.code()));
    s.push('(');
    let n = o.num_input();
    for i in 0..n {
        if i != 0 {
            s.push(',');
        }
        if let Some(inv) = o.get_in(i) {
            s.push_str(&render_vn(arch, fd, inv));
        }
    }
    s.push(')');
    s
}

fn run_full(stem: &str, which: usize) -> Result<(XmlArchitecture, Funcdata), String> {
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
    let fd = decompile_func(arch, &name, entry, 0).map_err(|e| format!("decompile: {e}"))?;
    Ok((xarch, fd))
}

use kuna_decomp::decompile_drive::print_c;

#[test]
#[ignore]
fn zz_dump_boolless_cfg() {
    let (mut xarch, fd) = match run_full("boolless", 0) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("SKIP: {e}");
            return;
        }
    };
    let arch = xarch.sleigh_mut().base_mut().unwrap();
    let nbb = fd.bblocks_get_size();
    eprintln!("=== basic blocks ({nbb}) ===");
    for i in 0..nbb {
        let bl = fd.bblocks_get_block(i);
        let block = fd.bblocks_ref().block(bl);
        let so = block.size_out();
        let mut outs = String::new();
        for j in 0..so {
            let o = fd.bblocks_ref().block(bl).get_out(j);
            outs.push_str(&format!(" out{j}={:?}", o));
        }
        eprintln!("BB {i} (id={bl:?}) sizeOut={so}{outs}");
        for op in fd.bb_ops(bl) {
            let o = fd.obank().get(op).unwrap();
            let flip = o.is_boolean_flip();
            let ft = o.is_fallthru_true();
            eprintln!("    {} flip={flip} fallthru_true={ft}", render_op(arch, &fd, op));
        }
    }
    eprintln!("=== sblocks size: {} ===", fd.sblocks_get_size());
    let rust = print_c(arch, &fd);
    eprintln!("=== RUST print C ===\n{rust}");
}

/// The committed C++ B5 oracle for boolless (tests/golden/snapshots/cpp/...).
const CPP_B5_ORACLE: &str = "\nuint1 boolless(void)\n\n{\n  uint1 v1; // acc\n  \n  v1 = dat_52;\n  if (dat_52 <= 10) {\n    v1 = 1;\n  }\n  return v1;\n}\n";

#[test]
fn boolless_print_c_emits_structured_body() {
    let (mut xarch, fd) = match run_full("boolless", 0) {
        Ok(v) => v,
        Err(e) if e.contains("not built") || e.contains("no .sla") => {
            eprintln!("SKIP boolless print C: {e}");
            return;
        }
        Err(e) => panic!("boolless run_full: {e}"),
    };
    let arch = xarch.sleigh_mut().base_mut().unwrap();

    // Dump the analyzed IR the printer renders from (the B4 boundary).
    eprintln!("=== boolless analyzed IR (the printer input) ===");
    let mut alive: Vec<kuna_decomp::seams::OpId> = fd.obank().iter_alive().collect();
    alive.sort_by_key(|&op| fd.obank().get(op).unwrap().get_seq_num().get_time());
    for &op in &alive {
        eprintln!("  {}", render_op(arch, &fd, op));
    }

    eprintln!("=== boolless sblocks size: {} ===", fd.sblocks_get_size());
    let rust = print_c(arch, &fd);

    eprintln!("\n=== RUST print C (boolless) ===\n{rust}");
    eprintln!("=== C++ B5 oracle ===\n{CPP_B5_ORACLE}");

    // (1) Block structuring produced a non-empty structured hierarchy.
    assert!(
        fd.sblocks_get_size() != 0,
        "ActionBlockStructure must seed sblocks (got empty structure)"
    );

    // (2) The body is no longer the empty shell: an `if` statement appears.
    assert!(
        rust.contains("if ("),
        "print C must now emit a structured if-statement, got:\n{rust}"
    );

    // (3) The global data location renders as the explicit `dat_52` name
    //     (the faithful pushUnnamedLocation / kuna angr naming), inside the
    //     comparison condition.
    assert!(
        rust.contains("dat_52"),
        "the explicit global must render as dat_52, got:\n{rust}"
    );

    // (4) Still a complete, brace-matched function carrying its name.
    assert!(
        rust.contains("boolless"),
        "print C must carry the function name, got:\n{rust}"
    );
    assert_eq!(
        rust.matches('{').count(),
        rust.matches('}').count(),
        "braces must balance, got:\n{rust}"
    );
    assert!(rust.matches('{').count() >= 2, "if-body + function body braces, got:\n{rust}");

    // The comparison renders inside parentheses (the opCbranch ( ) wrap).
    assert!(
        rust.contains("(") && rust.contains(")"),
        "the condition must be parenthesized, got:\n{rust}"
    );
}

// ===========================================================================
// REAL-PARITY tests (item: w10-structure-printbody / Round 2 — REPAIR).
//
// The Round-1 verifier left two tripwires that FIRE when boolless reaches real
// positive-assertion parity (`if (dat_52 <= 10)`).  Round 2 closes the analysis
// layer that produces that form — ActionRedundBranch::removeBranch (drops the
// degenerate same-target CBRANCH), ActionBlockStructure's negate-condition op
// flip + RuleCondNegate::opNormalizeFlip (INT_LESS(#0xa,i0x52) -> swapped
// INT_LESSEQUAL), and ActionPresentCompareForm gated on present_lessequal
// (`< 0xb` -> `<= 10`).  These rewrites happen in the ANALYZED IR; the printer
// still renders only what it is handed.  The two tests below now assert the
// achieved parity AND keep the anti-special-casing guard: the collapse must come
// from the IR carrying a real `INT_LESSEQUAL`, never from a hardcoded printer.
// ===========================================================================

/// ANTI-SPECIAL-CASING (preserved): the `dat_52 <= 10` form is produced by a
/// real IR rewrite — the analyzed IR carries an `INT_LESSEQUAL` whose operands
/// are `(i0x52, #0xa)` — NOT by the printer hardcoding the oracle string.  A
/// printer special-cased to the oracle would emit `<= 10` over a raw
/// `INT_LESS(#0xa, i0x52)`; this test fails if the IR does not actually hold the
/// collapsed comparison.
#[test]
fn w10_boolless_lessequal_comes_from_real_ir_not_printer() {
    let (mut xarch, fd) = match run_full("boolless", 0) {
        Ok(v) => v,
        Err(e) if e.contains("not built") || e.contains("no .sla") => {
            eprintln!("SKIP: {e}");
            return;
        }
        Err(e) => panic!("boolless run_full: {e}"),
    };
    let arch = xarch.sleigh_mut().base_mut().unwrap();

    // The analyzed IR must carry a real INT_LESSEQUAL over (i0x52, #0xa) — the
    // collapse is in the IR, not the printer.  (The degenerate INT_NOTEQUAL
    // CBRANCH was removed by ActionRedundBranch, so no INT_LESS(#0xa,..) survives.)
    let mut saw_lesseq = false;
    let mut saw_raw_less = false;
    for op in fd.obank().iter_alive() {
        match fd.obank().get(op).unwrap().code() {
            kuna_num::opcodes::OpCode::CPUI_INT_LESSEQUAL => saw_lesseq = true,
            kuna_num::opcodes::OpCode::CPUI_INT_LESS => saw_raw_less = true,
            _ => {}
        }
    }
    assert!(
        saw_lesseq,
        "the analyzed IR must hold a real INT_LESSEQUAL (the collapse is in the IR, not the printer)"
    );
    assert!(
        !saw_raw_less,
        "the degenerate INT_LESS path must be gone (ActionRedundBranch + opNormalizeFlip collapsed it)"
    );

    let rust = print_c(arch, &fd);
    eprintln!("=== RUST print C (boolless) ===\n{rust}");
    // The printer renders that IR as the oracle's `dat_52 <= 10`.
    assert!(
        rust.contains("dat_52 <= 10"),
        "the collapsed IR must render as `dat_52 <= 10`, got:\n{rust}"
    );
}

/// THE REAL WIN: boolless's min=1/max=1 positive datatest assertion
/// `if (dat_52 <= 10)` is now satisfied by genuine parity (the first real
/// positive assertion from the un-seam chain).  This replaces the Round-1
/// "not yet real" guard, which was designed to fire exactly here.
#[test]
fn w10_boolless_positive_datatest_assertion_now_real() {
    let (mut xarch, fd) = match run_full("boolless", 0) {
        Ok(v) => v,
        Err(e) if e.contains("not built") || e.contains("no .sla") => {
            eprintln!("SKIP: {e}");
            return;
        }
        Err(e) => panic!("boolless run_full: {e}"),
    };
    let arch = xarch.sleigh_mut().base_mut().unwrap();
    let rust = print_c(arch, &fd);
    // The datatest's min=1 positive assertion `if (dat_52 <= 10)` is now MET.
    assert!(
        rust.contains("if (dat_52 <= 10)"),
        "the boolless POSITIVE (min=1) datatest assertion `if (dat_52 <= 10)` must now hold, got:\n{rust}"
    );
}

/// THE MERGE/NAMING/OUTPUT-TYPE WIN: boolless `print C` now byte-matches the C++
/// B5 oracle, modulo the single documented residual `undefined1` vs `uint1` (the
/// recovered output/local *type name* — the W8 `ActionInferTypes` surface; the
/// output STORAGE, the merged `v1` local, the `v1 = dat_52` trim COPY, the decl,
/// the `// acc` storage comment, and the return statement all recover exactly).
///
/// The assertion substitutes the W8 type name (`undefined1` -> `uint1`) and then
/// requires an EXACT byte match.  When `ActionInferTypes` lands, the substitution
/// becomes a no-op and boolless is the first fully-byte-parity function.
#[test]
fn w10_boolless_full_byte_parity_modulo_type_inference() {
    let (mut xarch, fd) = match run_full("boolless", 0) {
        Ok(v) => v,
        Err(e) if e.contains("not built") || e.contains("no .sla") => {
            eprintln!("SKIP: {e}");
            return;
        }
        Err(e) => panic!("boolless run_full: {e}"),
    };
    let arch = xarch.sleigh_mut().base_mut().unwrap();
    let rust = print_c(arch, &fd);

    // The merge/naming/output-storage layer is real: a named `v1` local, the
    // `v1 = dat_52;` trim COPY, the `undefined1 v1; // acc` decl, and `return v1`.
    assert!(rust.contains("v1 = dat_52;"), "trim-COPY initial assignment missing:\n{rust}");
    assert!(rust.contains("v1; // acc"), "decl + storage comment missing:\n{rust}");
    assert!(rust.contains("    v1 = 1;"), "if-body assignment must use the merged name:\n{rust}");
    assert!(rust.contains("return v1;"), "return must use the merged name:\n{rust}");
    // i0x52 (the global) must STILL render as `dat_52` (NOT absorbed into v1).
    assert!(rust.contains("dat_52"), "the global must stay dat_52 (no over-merge):\n{rust}");
    assert!(!rust.contains("dat_52 = "), "dat_52 must not be assigned (it is read-only here):\n{rust}");

    // Full byte-parity modulo the W8 type-name residual.
    let normalized = rust.replace("undefined1", "uint1");
    assert_eq!(
        normalized, CPP_B5_ORACLE,
        "boolless print C must byte-match the C++ B5 oracle after the documented \
         W8 type-name substitution (undefined1 -> uint1).\n--- rust (raw) ---\n{rust}\n\
         --- normalized ---\n{normalized}\n--- oracle ---\n{CPP_B5_ORACLE}"
    );
}

/// The emitted body is a real, structurally-complete function (begin/return/
/// brace-matched), with the assignment INSIDE the if-body — i.e. the structured
/// hierarchy is real, not a flat dump.  Guards against the if-collapse silently
/// degrading to a flat statement list.
#[test]
fn w10_boolless_if_body_contains_assignment() {
    let (mut xarch, fd) = match run_full("boolless", 0) {
        Ok(v) => v,
        Err(e) if e.contains("not built") || e.contains("no .sla") => {
            eprintln!("SKIP: {e}");
            return;
        }
        Err(e) => panic!("boolless run_full: {e}"),
    };
    let arch = xarch.sleigh_mut().base_mut().unwrap();
    let rust = print_c(arch, &fd);

    // The if introduces a brace; the assignment (`= 1`) sits between that open
    // brace and its matching close brace (real nesting, not a flat list).
    let if_pos = rust.find("if (").expect("structured if present");
    let open = rust[if_pos..].find('{').map(|o| if_pos + o).expect("if open brace");
    let close = rust[open..].find('}').map(|c| open + c).expect("if close brace");
    let body = &rust[open..close];
    assert!(
        body.contains("= 1"),
        "the `= 1` assignment must be nested inside the if-body braces, got body:\n{body}\nfull:\n{rust}"
    );
    // And a return after the if (real function tail).
    assert!(rust.contains("return"), "function must emit a return, got:\n{rust}");
}





// ===========================================================================
// VERIFIER adversarial tests (item: w10-unseam-merge-types — Round 1).
//
// These target the spots the hunt list flagged as most fragile for the
// merge/naming/output-type un-seam: (1) the trim COPY must be a REAL artifact of
// the Merge engine over live IR (anti-special-casing for `v1 = dat_52`), (2) the
// merged HighVariable must actually carry >1 instance (a real merge, not a
// rename), and the `v1` name + `// acc` comment must derive from the addr-tied
// storage member, NOT the trim-COPY unique, and (3) the angr `vN` naming must be
// CONDITIONAL on the merge/addr-tied chain — a function whose pipeline produced
// no nameable local must NOT get a `vN` name or an `undefined<N>` recovered
// return type (proving the naming is engine logic, not a boolless hardcode).
// ===========================================================================

/// (1) ANTI-SPECIAL-CASING: the `v1 = dat_52` trim COPY is a genuine Merge-engine
/// artifact — a `COPY(i0x52)` whose output is a FRESH internal/unique varnode
/// (not the global, not ACC) that feeds the `ACC = MULTIEQUAL(...)`.  A printer
/// hardcoded to emit `v1 = dat_52` would leave the IR without this op.  This is
/// the structural witness that `ActionMergeRequired` ran `mergeAddrTied ->
/// groupPartials -> mergeMarker` over the real IR and `trimOpInput`/
/// `allocateCopyTrim` inserted the trim.
#[test]
fn w10_merge_trim_copy_is_real_ir_artifact() {
    let (mut xarch, fd) = match run_full("boolless", 0) {
        Ok(v) => v,
        Err(e) if e.contains("not built") || e.contains("no .sla") => {
            eprintln!("SKIP: {e}");
            return;
        }
        Err(e) => panic!("boolless run_full: {e}"),
    };
    let arch = xarch.sleigh_mut().base_mut().unwrap();

    // Find a COPY whose output feeds a MULTIEQUAL (the merge trim that breaks the
    // global<->ACC cover intersection).  Its output must be a fresh unique
    // (IPTR_INTERNAL space) — created by the merge, not present pre-merge.
    let mut trim_copy_out: Option<kuna_decomp::seams::VarnodeId> = None;
    let mut multiequal_inputs: Vec<kuna_decomp::seams::VarnodeId> = Vec::new();
    for op in fd.obank().iter_alive() {
        let o = fd.obank().get(op).unwrap();
        match o.code() {
            kuna_num::opcodes::OpCode::CPUI_MULTIEQUAL => {
                for i in 0..o.num_input() {
                    if let Some(v) = o.get_in(i) {
                        multiequal_inputs.push(v);
                    }
                }
            }
            kuna_num::opcodes::OpCode::CPUI_COPY => {
                // candidate trim if its single output is an internal unique
                if let Some(out) = o.get_out() {
                    let internal = fd
                        .vbank()
                        .get(out)
                        .and_then(|v| v.get_addr().get_space())
                        .map(|s| s.get_type() == kuna_base::space::spacetype::IPTR_INTERNAL)
                        .unwrap_or(false);
                    if internal {
                        trim_copy_out = Some(out);
                    }
                }
            }
            _ => {}
        }
    }
    let trim_out = trim_copy_out
        .expect("merge must have inserted a COPY into a fresh unique (the trim COPY)");
    assert!(
        multiequal_inputs.contains(&trim_out),
        "the trim COPY's unique output must feed the ACC MULTIEQUAL (the real merge join)"
    );

    // And the printer renders exactly that op as `v1 = dat_52;` — the unique is
    // ACC's merged high (named v1), its COPY input is the global (dat_52).
    let rust = print_c(arch, &fd);
    assert!(rust.contains("v1 = dat_52;"), "trim COPY must render as `v1 = dat_52;`, got:\n{rust}");
}

/// (2) REAL MERGE + STORAGE-REPRESENTATIVE: ACC's HighVariable must carry MORE
/// THAN ONE instance (a genuine merge of the trim-unique + the ACC members),
/// and the bound `v1` name plus the `// acc` storage comment must come from the
/// addr-tied ACC member, not the trim unique.  Guards against (a) the "merge"
/// being a no-op rename of a single varnode and (b) the storage comment being
/// read off the wrong (unique) representative.
#[test]
fn w10_merged_high_has_multiple_instances_and_acc_storage() {
    let (mut xarch, fd) = match run_full("boolless", 0) {
        Ok(v) => v,
        Err(e) if e.contains("not built") || e.contains("no .sla") => {
            eprintln!("SKIP: {e}");
            return;
        }
        Err(e) => panic!("boolless run_full: {e}"),
    };
    let arch = xarch.sleigh_mut().base_mut().unwrap();

    // Locate the high named "v1".
    let mut v1_high: Option<kuna_decomp::seams::HighVariableId> = None;
    for vn in fd.vbank().iter_loc() {
        if let Some(h) = fd.vbank().get(vn).and_then(|v| v.get_high()) {
            if fd.high_bank().get(h).and_then(|hh| hh.kuna_name()) == Some("v1") {
                v1_high = Some(h);
                break;
            }
        }
    }
    let h = v1_high.expect("a high named v1 must exist after ActionNameVars");
    let n = fd.high_bank().get(h).unwrap().num_instances();
    assert!(n > 1, "the merged v1 high must hold >1 instance (real merge), got {n}");

    // At least one instance is the addr-tied ACC register (an INTERNAL unique is
    // never addr-tied); the storage comment derives from it.
    let mut has_addr_tied = false;
    let mut has_unique = false;
    for i in 0..n {
        let vn = fd.high_bank().get(h).unwrap().get_instance(i);
        let v = fd.vbank().get(vn).unwrap();
        if v.is_addr_tied() {
            has_addr_tied = true;
        }
        if v.get_addr().get_space().map(|s| s.get_type() == kuna_base::space::spacetype::IPTR_INTERNAL).unwrap_or(false) {
            has_unique = true;
        }
    }
    assert!(has_addr_tied, "v1 high must contain the addr-tied ACC storage member");
    assert!(has_unique, "v1 high must contain the merged trim-COPY unique member");

    // The decl + storage comment render from the ACC register (lowercased) — not
    // from the unique (which has no register name).
    let rust = print_c(arch, &fd);
    assert!(rust.contains("v1; // acc"), "storage comment must be the ACC reg name lowercased, got:\n{rust}");
}

/// (3) NAMING IS CONDITIONAL (anti-hardcode): a different function whose pipeline
/// produces no nameable addr-tied local must NOT get a `vN` name nor an
/// `undefined<N>` recovered return type — it stays the un-recovered `void` shell
/// with raw register tokens.  If the engine special-cased boolless's strings, an
/// unrelated function would leak `v1`/`undefined1`; it must not.
#[test]
fn w10_naming_conditional_other_function_gets_no_vn_name() {
    // condconst is x86-16; in this slice its analysis does not recover a nameable
    // addr-tied local, so naming/output-type must NOT fire (no `vN`, void return).
    let (mut xarch, fd) = match run_full("condconst", 0) {
        Ok(v) => v,
        Err(e) if e.contains("not built") || e.contains("no .sla") => {
            eprintln!("SKIP: {e}");
            return;
        }
        Err(e) => panic!("condconst run_full: {e}"),
    };
    let arch = xarch.sleigh_mut().base_mut().unwrap();
    let rust = print_c(arch, &fd);

    // No HighVariable should carry a kuna name here (naming is gated on the
    // addr-tied/merge chain that this function's slice does not reach).
    let mut named = 0usize;
    for vn in fd.vbank().iter_loc() {
        if let Some(h) = fd.vbank().get(vn).and_then(|v| v.get_high()) {
            if fd.high_bank().get(h).and_then(|hh| hh.kuna_name()).is_some() {
                named += 1;
            }
        }
    }
    assert_eq!(named, 0, "no nameable local in this slice -> no `vN` names, got {named}; print:\n{rust}");
    // And critically: no boolless-specific string leaked across functions.
    assert!(!rust.contains("dat_52"), "boolless's `dat_52` must not appear in condconst:\n{rust}");
    assert!(!rust.contains("// acc"), "boolless's `// acc` must not appear in condconst:\n{rust}");
}

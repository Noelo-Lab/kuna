//! END-TO-END GATE for the `Funcdata::encode` port
//! (`crates/kuna-decomp/src/substrate/funcdata_encode.rs`).
//!
//! Decompiles real corpus functions (reusing the `decompile_e2e.rs` bootstrap
//! mechanics), encodes each resulting [`Funcdata`] as the `<function>`/`<ast>`
//! document, round-trips the bytes back through kuna's `PackedDecode`, and
//! asserts:
//!
//!   * the header (`<function name size>`) round-trips,
//!   * the `<ast>` has `<varnodes>` + `<block>`(s), each `<op>` a `<seqnum uniq>`
//!     and a `code`, and
//!   * the **load-bearing invariant**: every `<addr ref>` under `<varnodes>`
//!     equals a live Varnode's `get_create_index()`, and every `<op>`'s
//!     `<seqnum uniq>` equals that op's `get_time()` — the same accessors the
//!     markup `EmitMarkup` path reads, so the `<ast>` ref-ids match the markup
//!     ref-ids by construction.
//!
//! Like `decompile_e2e.rs`, this needs the built `.sla` artifacts (`make specs`);
//! a fixture whose bootstrap/decompile fails is reported (not silently skipped),
//! and the gate requires at least a few functions to validate.

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::{Address, ELEM_ADDR, ATTRIB_UNIQ, ELEM_RANGELIST, ELEM_SEQNUM};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{
    Decoder, IdRegistry, PackedDecode, PackedEncode, ATTRIB_NAME, ATTRIB_REF, ATTRIB_SIZE,
    ELEM_VOID,
};
use kuna_base::space::{spacetype, AddrSpaceManager};
use kuna_base::types::int4;
use kuna_base::xml::{DocumentStorage, Element};

use kuna_decomp::architecture::Architecture;
use kuna_decomp::block::ELEM_BLOCK;
use kuna_decomp::decompile_drive::decompile_func;
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::funcdata_encode::{ELEM_AST, ELEM_FUNCTION, ELEM_IOP, ELEM_VARNODES};
use kuna_decomp::options::{register_option_elements, OptionDatabase};
use kuna_decomp::prettyprint::ids::{ATTRIB_OPREF, ATTRIB_VARREF};
use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase};
use kuna_decomp::xml_arch::{XmlArchitecture, XmlArchitectureCapability};

use kuna_num::pcoderaw::{ATTRIB_CODE, ELEM_SPACEID};
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::translate::{register_translate_ids, ELEM_OP};

// ===========================================================================
// Fixture selection (same as decompile_e2e.rs).
// ===========================================================================

const FIXTURES: &[&str] = &["boolless", "ccmp", "gp", "lzcount", "condexesub"];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

// ===========================================================================
// Datatest XML parsing (copied from decompile_e2e.rs).
// ===========================================================================

struct SymbolFn {
    name: String,
    space: String,
    offset: u64,
}

struct DataTest {
    binaryimage: Rc<Element>,
    arch_id: String,
    symbols: Vec<SymbolFn>,
    options: Vec<(String, String, String, String)>,
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

fn parse_option_com(text: &str) -> Option<(String, String, String, String)> {
    let mut it = text.split_whitespace();
    if it.next()? != "option" {
        return None;
    }
    let name = it.next()?.to_string();
    let p1 = it.next().unwrap_or("").to_string();
    let p2 = it.next().unwrap_or("").to_string();
    let p3 = it.next().unwrap_or("").to_string();
    Some((name, p1, p2, p3))
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

    let mut coms = Vec::new();
    find_named(&root, "com", &mut coms);
    let options: Vec<_> = coms
        .iter()
        .filter_map(|c| parse_option_com(&String::from_utf8_lossy(c.get_content())))
        .collect();

    Ok(DataTest { binaryimage, arch_id, symbols, options })
}

// ===========================================================================
// Bootstrap a full Architecture (copied from decompile_e2e.rs).
// ===========================================================================

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
    register_option_elements(&mut registry);
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
    // decompile_e2e.rs).
    arch.open_image(unsafe { &*manager_ptr }, &registry)
        .map_err(|e| format!("open_image: {e}"))?;
    let img = arch.take_loader().ok_or("loader vanished after open")?;
    arch.sleigh_mut().base_mut().unwrap().set_loader(Box::new(img));

    Ok(arch)
}

fn apply_options(arch: &mut Architecture, dt: &DataTest, registry: &IdRegistry) {
    let options = OptionDatabase::new();
    for (name, p1, p2, p3) in &dt.options {
        let id = registry.find_element(name, 0);
        if id == 0 {
            continue;
        }
        let _ = options.set(arch, id, p1, p2, p3);
    }
}

// ===========================================================================
// Round-trip structural decode of a <function> document.
// ===========================================================================

#[derive(Default)]
struct Observed {
    name: Vec<u8>,
    size: i64,
    varnode_refs: BTreeSet<u64>,
    op_uniqs: BTreeSet<u64>,
    saw_iop: bool,
    saw_spaceid: bool,
    saw_void: bool,
    saw_bare_ref: bool,
    block_count: usize,
    op_count: usize,
}

fn decode_function(manage: &AddrSpaceManager, bytes: &[u8]) -> KunaResult<Observed> {
    let mut obs = Observed::default();
    let mut dec = PackedDecode::new(manage);
    dec.ingest_stream(bytes)?;

    let fid = dec.open_element()?;
    assert_eq!(fid, ELEM_FUNCTION.get_id(), "root is not <function>");
    obs.name = dec.read_string_id(&ATTRIB_NAME)?;
    obs.size = dec.read_signed_integer_id(&ATTRIB_SIZE)?;

    loop {
        let c = dec.peek_element()?;
        if c == 0 {
            break;
        }
        if c == ELEM_AST.get_id() {
            let ast = dec.open_element()?;
            decode_ast(&mut dec, &mut obs)?;
            dec.close_element(ast)?;
        } else {
            dec.skip_element()?; // baseaddr <addr>, or anything unexpected.
        }
    }
    dec.close_element(fid)?;
    Ok(obs)
}

fn decode_ast(dec: &mut PackedDecode, obs: &mut Observed) -> KunaResult<()> {
    loop {
        let c = dec.peek_element()?;
        if c == 0 {
            break;
        }
        if c == ELEM_VARNODES.get_id() {
            let vid = dec.open_element()?;
            loop {
                let a = dec.peek_element()?;
                if a == 0 {
                    break;
                }
                assert_eq!(a, ELEM_ADDR.get_id(), "<varnodes> child is not <addr>");
                let aid = dec.open_element()?;
                obs.varnode_refs.insert(dec.read_unsigned_integer_id(&ATTRIB_REF)?);
                dec.close_element(aid)?;
            }
            dec.close_element(vid)?;
        } else if c == ELEM_BLOCK.get_id() {
            obs.block_count += 1;
            let bid = dec.open_element()?;
            decode_block(dec, obs)?;
            dec.close_element(bid)?;
        } else {
            // <blockedge> (or any later-phase element): skip the subtree.
            dec.skip_element()?;
        }
    }
    Ok(())
}

fn decode_block(dec: &mut PackedDecode, obs: &mut Observed) -> KunaResult<()> {
    loop {
        let c = dec.peek_element()?;
        if c == 0 {
            break;
        }
        if c == ELEM_OP.get_id() {
            obs.op_count += 1;
            let oid = dec.open_element()?;
            let _code = dec.read_signed_integer_id(&ATTRIB_CODE)?;
            let mut first = true;
            loop {
                let oc = dec.peek_element()?;
                if oc == 0 {
                    break;
                }
                if first {
                    assert_eq!(oc, ELEM_SEQNUM.get_id(), "first <op> child is not <seqnum>");
                    let sid = dec.open_element()?;
                    obs.op_uniqs.insert(dec.read_unsigned_integer_id(&ATTRIB_UNIQ)?);
                    dec.close_element(sid)?;
                    first = false;
                } else {
                    if oc == ELEM_IOP.get_id() {
                        obs.saw_iop = true;
                    } else if oc == ELEM_SPACEID.get_id() {
                        obs.saw_spaceid = true;
                    } else if oc == ELEM_VOID.get_id() {
                        obs.saw_void = true;
                    } else if oc == ELEM_ADDR.get_id() {
                        obs.saw_bare_ref = true;
                    }
                    dec.skip_element()?;
                }
            }
            dec.close_element(oid)?;
        } else {
            assert_eq!(c, ELEM_RANGELIST.get_id(), "unexpected <block> child");
            dec.skip_element()?;
        }
    }
    Ok(())
}

// ===========================================================================
// The live-Funcdata expectations for the invariant cross-check.
// ===========================================================================

fn expected_refs(fd: &Funcdata) -> BTreeSet<u64> {
    fd.vbank()
        .iter_loc()
        .filter(|&vn| fd.vbank().get(vn).unwrap().get_space().get_type() != spacetype::IPTR_IOP)
        .map(|vn| fd.vbank().get(vn).unwrap().get_create_index() as u64)
        .collect()
}

fn expected_uniqs(fd: &Funcdata) -> BTreeSet<u64> {
    let mut out = BTreeSet::new();
    let n: int4 = fd.bblocks_get_size();
    for i in 0..n {
        let bl = fd.bblocks_get_block(i);
        for op in fd.bb_ops(bl) {
            out.insert(fd.obank().get(op).unwrap().get_time() as u64);
        }
    }
    out
}

/// Encode `fd`, round-trip, and assert the structure + the load-bearing
/// invariant.  Returns the `Observed` for the aggregate coverage report.
fn encode_and_verify(fd: &mut Funcdata) -> Observed {
    let mut bytes: Vec<u8> = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut bytes);
        fd.encode(&mut enc, 1, true).expect("encode");
    }
    assert!(!bytes.is_empty(), "encode produced no bytes");

    let obs = decode_function(fd.get_arch().manage(), &bytes).expect("decode round-trip");

    assert_eq!(obs.name, fd.get_name().as_bytes(), "<function name> mismatch");
    assert_eq!(obs.size, fd.get_size() as i64, "<function size> mismatch");
    assert!(obs.block_count >= 1, "no <block> encoded");
    assert!(obs.op_count >= 1, "no <op> encoded");

    // The load-bearing invariant.
    assert_eq!(obs.varnode_refs, expected_refs(fd), "<varnodes> refs != create indices");
    assert_eq!(obs.op_uniqs, expected_uniqs(fd), "<op> uniqs != op times");

    obs
}

// ===========================================================================
// The gate.
// ===========================================================================

#[test]
fn corpus_functions_encode_and_roundtrip() {
    let mut verified = 0usize;
    let mut errors: Vec<String> = Vec::new();
    let mut agg = Observed::default();

    for stem in FIXTURES {
        let dt = match parse_datatest(stem) {
            Ok(dt) => dt,
            Err(e) => {
                errors.push(format!("{stem} (parse): {e}"));
                continue;
            }
        };
        let registry = build_registry();
        let mut xarch = match bootstrap(&dt) {
            Ok(x) => x,
            Err(e) => {
                errors.push(format!("{stem} (bootstrap): {e}"));
                continue;
            }
        };
        if let Some(base) = xarch.sleigh_mut().base_mut() {
            apply_options(base, &dt, &registry);
        }

        for sym in &dt.symbols {
            let base = match xarch.sleigh_mut().base_mut() {
                Some(b) => b,
                None => continue,
            };
            let space = match base.manage().get_space_by_name(&sym.space) {
                Some(s) => Rc::clone(s),
                None => continue,
            };
            let entry = Address::new(space, sym.offset);
            match decompile_func(base, &sym.name, entry, 0) {
                Ok(mut fd) => {
                    let obs = encode_and_verify(&mut fd);
                    verified += 1;
                    agg.saw_iop |= obs.saw_iop;
                    agg.saw_spaceid |= obs.saw_spaceid;
                    agg.saw_void |= obs.saw_void;
                    agg.saw_bare_ref |= obs.saw_bare_ref;
                }
                Err(e) => errors.push(format!("{stem}:{} decompile: {e}", sym.name)),
            }
        }
    }

    eprintln!("=== funcdata_encode_e2e report ===");
    eprintln!("verified functions: {verified}");
    eprintln!(
        "aggregate branch coverage: bare_ref={} void={} spaceid={} iop={}",
        agg.saw_bare_ref, agg.saw_void, agg.saw_spaceid, agg.saw_iop
    );
    for e in &errors {
        eprintln!("  note: {e}");
    }

    // The gate's teeth: several real functions must encode + round-trip with the
    // invariant intact, and the always-present branches (bare ref + void) must
    // appear across real code.  (The <iop>/<spaceid> branches are guaranteed by
    // the synthetic unit test; they are only reported here.)
    assert!(
        verified >= 3,
        "expected >= 3 corpus functions to encode + round-trip, got {verified} (notes: {errors:?})"
    );
    assert!(agg.saw_bare_ref, "no bare <addr ref> observed across real functions");
    assert!(agg.saw_void, "no <void> slot observed across real functions");
}

// ===========================================================================
// Step 5 GATE: the click-to-address contract — the clang `<function>` markup's
// token refs resolve against the `<ast>`.
//
// This ties the step-4 encoder (`Funcdata::encode` -> `<ast>`) and the step-5
// markup emitter (`PrintC::doc_function_markup` -> clang `<function>`) together
// over the SAME decompiled `Funcdata`: every `opref`/`varref` a marked-up token
// carries MUST appear as an `<op>`'s `<seqnum uniq>` / a `<varnodes>` `<addr ref>`
// in the `<ast>` — because both sides read the exact same `get_time()` /
// `get_create_index()` accessors, the tie holds by construction.
// ===========================================================================

/// Recursively walk a packed clang `<function>` document, collecting every
/// `ATTRIB_OPREF` / `ATTRIB_VARREF` value (regardless of the element it hangs
/// on).  Unmatched attributes are auto-skipped by `get_next_attribute_id`.
fn walk_markup(
    dec: &mut PackedDecode,
    oprefs: &mut BTreeSet<u64>,
    varrefs: &mut BTreeSet<u64>,
) -> KunaResult<()> {
    let id = dec.open_element()?;
    loop {
        let a = dec.get_next_attribute_id()?;
        if a == 0 {
            break;
        }
        if a == ATTRIB_OPREF.get_id() {
            oprefs.insert(dec.read_unsigned_integer()?);
        } else if a == ATTRIB_VARREF.get_id() {
            varrefs.insert(dec.read_unsigned_integer()?);
        }
    }
    loop {
        let c = dec.peek_element()?;
        if c == 0 {
            break;
        }
        walk_markup(dec, oprefs, varrefs)?;
    }
    dec.close_element(id)?;
    Ok(())
}

/// Structurally decode the clang `<function>` markup document and return its
/// `(oprefs, varrefs)` sets.
fn decode_markup_refs(
    manage: &AddrSpaceManager,
    bytes: &[u8],
) -> KunaResult<(BTreeSet<u64>, BTreeSet<u64>)> {
    let mut dec = PackedDecode::new(manage);
    dec.ingest_stream(bytes)?;
    // The markup root is the clang <function> (EmitMarkup::begin_function opens
    // ELEM_FUNCTION, the same numeric id as the encoder's <function>).
    let root = dec.peek_element()?;
    assert_eq!(root, ELEM_FUNCTION.get_id(), "markup root is not <function>");
    let mut oprefs = BTreeSet::new();
    let mut varrefs = BTreeSet::new();
    walk_markup(&mut dec, &mut oprefs, &mut varrefs)?;
    Ok((oprefs, varrefs))
}

/// Drive `PrintC::doc_function_markup` over `fd` (moving the printer out of the
/// architecture exactly as `decompile_drive::print_c` does) and decode the
/// resulting clang `<function>` into its `(oprefs, varrefs)` sets.
fn markup_refs_for(base: &mut Architecture, fd: &Funcdata) -> (BTreeSet<u64>, BTreeSet<u64>) {
    let mut printer = base.take_print();
    let bytes = printer.doc_function_markup(fd, base);
    base.put_print(printer);
    assert!(!bytes.is_empty(), "doc_function_markup produced no bytes");
    decode_markup_refs(fd.get_arch().manage(), &bytes).expect("decode markup <function>")
}

#[test]
fn corpus_functions_markup_refs_resolve_against_ast() {
    let mut verified = 0usize;
    let mut errors: Vec<String> = Vec::new();
    let mut total_oprefs = 0usize;
    let mut total_varrefs = 0usize;

    for stem in FIXTURES {
        let dt = match parse_datatest(stem) {
            Ok(dt) => dt,
            Err(e) => {
                errors.push(format!("{stem} (parse): {e}"));
                continue;
            }
        };
        let registry = build_registry();
        let mut xarch = match bootstrap(&dt) {
            Ok(x) => x,
            Err(e) => {
                errors.push(format!("{stem} (bootstrap): {e}"));
                continue;
            }
        };
        if let Some(base) = xarch.sleigh_mut().base_mut() {
            apply_options(base, &dt, &registry);
        }

        for sym in &dt.symbols {
            let base = match xarch.sleigh_mut().base_mut() {
                Some(b) => b,
                None => continue,
            };
            let space = match base.manage().get_space_by_name(&sym.space) {
                Some(s) => Rc::clone(s),
                None => continue,
            };
            let entry = Address::new(space, sym.offset);
            let fd = match decompile_func(base, &sym.name, entry, 0) {
                Ok(fd) => fd,
                Err(e) => {
                    errors.push(format!("{stem}:{} decompile: {e}", sym.name));
                    continue;
                }
            };

            // (1)+(2) the <ast> ids (step 4) and the clang markup refs (step 5).
            let ast_refs = expected_refs(&fd);
            let ast_uniqs = expected_uniqs(&fd);
            let (oprefs, varrefs) = markup_refs_for(base, &fd);

            // (3)+(4) the click-to-address contract: every markup opref/varref
            // resolves to an <ast> op time / varnode create index.  Because the
            // markup and the <ast> read the SAME get_time()/get_create_index(),
            // this is a subset relation that must hold exactly.
            for o in &oprefs {
                assert!(
                    ast_uniqs.contains(o),
                    "{stem}:{} markup opref {o} has no matching <ast> <op> <seqnum uniq>",
                    sym.name
                );
            }
            for v in &varrefs {
                assert!(
                    ast_refs.contains(v),
                    "{stem}:{} markup varref {v} has no matching <ast> <varnodes> <addr ref>",
                    sym.name
                );
            }
            total_oprefs += oprefs.len();
            total_varrefs += varrefs.len();
            verified += 1;
        }
    }

    eprintln!("=== markup-refs-resolve-against-ast report ===");
    eprintln!(
        "verified functions: {verified}, distinct oprefs {total_oprefs}, distinct varrefs {total_varrefs}"
    );
    for e in &errors {
        eprintln!("  note: {e}");
    }

    // The gate's teeth: several real functions verify, and — crucially — the
    // markup actually CARRIES refs (guards against a MarkupRef-still-none
    // regression) that ALL resolve against the <ast> (asserted per function above).
    assert!(
        verified >= 3,
        "expected >= 3 corpus functions to markup + resolve, got {verified} (notes: {errors:?})"
    );
    assert!(
        total_oprefs > 0,
        "no <opref> emitted across the corpus markup — MarkupRef still none()?"
    );
    assert!(
        total_varrefs > 0,
        "no <varref> emitted across the corpus markup — MarkupRef still none()?"
    );
}

#[test]
fn markup_emits_nonempty_refs() {
    // A focused tripwire for the MarkupRef-still-none() regression: SOME real
    // decompiled function's clang markup must carry BOTH a resolvable opref AND a
    // varref.  (Not every function has named variable leaves — a purely boolean
    // body may emit only oprefs — so scan the corpus for one fully-marked
    // function rather than pinning a single fixture.)
    let mut saw_op = false;
    let mut saw_var = false;
    let mut found_both = false;

    'outer: for stem in FIXTURES {
        let dt = match parse_datatest(stem) {
            Ok(dt) => dt,
            Err(_) => continue,
        };
        let registry = build_registry();
        let mut xarch = match bootstrap(&dt) {
            Ok(x) => x,
            Err(_) => continue,
        };
        if let Some(base) = xarch.sleigh_mut().base_mut() {
            apply_options(base, &dt, &registry);
        }
        for sym in &dt.symbols {
            let base = match xarch.sleigh_mut().base_mut() {
                Some(b) => b,
                None => continue,
            };
            let space = match base.manage().get_space_by_name(&sym.space) {
                Some(s) => Rc::clone(s),
                None => continue,
            };
            let entry = Address::new(space, sym.offset);
            let fd = match decompile_func(base, &sym.name, entry, 0) {
                Ok(fd) => fd,
                Err(_) => continue,
            };
            let (oprefs, varrefs) = markup_refs_for(base, &fd);
            saw_op |= !oprefs.is_empty();
            saw_var |= !varrefs.is_empty();
            if !oprefs.is_empty() && !varrefs.is_empty() {
                found_both = true;
                break 'outer;
            }
        }
    }

    assert!(saw_op, "doc_function_markup emitted no <opref> — MarkupRef never populated");
    assert!(saw_var, "doc_function_markup emitted no <varref> — MarkupRef never populated");
    assert!(found_both, "no single function's markup carried both an opref and a varref");
}

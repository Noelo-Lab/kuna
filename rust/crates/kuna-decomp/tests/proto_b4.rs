//! GATE for `rport/w10-proto-recovery`: the real `fspec::FuncProto` is wired onto
//! `Funcdata` and the proto-recovery actions (`ActionPrototypeTypes` ->
//! `Heritage::guardReturns` -> `ActionReturnRecovery`) run on the live heritaged
//! SSA, converging the `boolless` RETURN toward the C++ B4 `return(#0x0) ACC`.
//!
//! Before this wave the `boolless` RETURN read the raw 8051 stack-pop chain
//! (`return(u0x0000e400)` over LOAD/INT_ADD/LOAD/PIECE), which stayed alive
//! because RETURN-in0 was fully consumed (9 alive ops).  This gate asserts:
//!   * RETURN-in0 is now the stripped indirect ref (`#0x0` constant);
//!   * RETURN recovers the accumulator (ACC) as a second input;
//!   * the dead stack-pop chain is swept (alive-op count drops below 9).
//!
//! The cspec `<default_proto>` decode (the general, spec-driven path) gives the
//! 8051 default model its real ACC output `<pentry>`, so the recovery fires for
//! every 8051 function — no boolless special-casing.

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

use kuna_num::opcodes::OpCode;
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

#[test]
fn boolless_return_recovers_accumulator() {
    let (xarch, fd) = match run_full("boolless", 0) {
        Ok(v) => v,
        Err(e) if e.contains("not built") || e.contains("no .sla") => {
            eprintln!("SKIP boolless: {e}");
            return;
        }
        Err(e) => panic!("boolless run_full: {e}"),
    };
    let arch = xarch.sleigh().base().unwrap();

    let mut alive: Vec<kuna_decomp::seams::OpId> = fd.obank().iter_alive().collect();
    alive.sort_by_key(|&op| {
        let o = fd.obank().get(op).unwrap();
        let blk = o.get_parent().map(|b| fd.bblocks_ref().block(b).get_index()).unwrap_or(-1);
        (blk, o.get_seq_num().get_time())
    });

    let n_alive = alive.len();
    eprintln!("=== boolless B4 (post-pipeline, proto recovery on) ===");
    eprintln!("alive ops: {n_alive}");
    for &op in &alive {
        eprintln!("  {}", render_op(arch, &fd, op));
    }

    // Locate the RETURN and inspect its inputs.
    let ret = alive
        .iter()
        .copied()
        .find(|&op| fd.obank().get(op).unwrap().code() == OpCode::CPUI_RETURN)
        .expect("a RETURN op survives");
    let ro = fd.obank().get(ret).unwrap();
    let n_in = ro.num_input();
    eprintln!("RETURN num_input = {n_in}");
    let in0 = ro.get_in(0).expect("RETURN in0");
    let in0_is_const = fd.vbank().get(in0).map(|v| v.is_constant()).unwrap_or(false);
    let in0_const_val = fd.vbank().get(in0).map(|v| v.get_offset()).unwrap_or(0);
    eprintln!(
        "RETURN in0: const={in0_is_const} val={in0_const_val} ; form = {}",
        render_op(arch, &fd, ret)
    );

    // (1) ActionPrototypeTypes stripped the indirect register: in0 is now #0x0.
    assert!(
        in0_is_const && in0_const_val == 0,
        "ActionPrototypeTypes must strip RETURN-in0 to the #0x0 constant (got const={in0_is_const}, val={in0_const_val})"
    );

    // (2) The recovered return value (ACC) is now a second input to the RETURN.
    assert!(
        n_in >= 2,
        "ActionReturnRecovery must append the recovered output (ACC) as RETURN in1 (got num_input={n_in})"
    );
    let in1 = ro.get_in(1).expect("RETURN in1");
    let in1_name = render_vn(arch, &fd, in1);
    eprintln!("RETURN in1 (recovered output) = {in1_name}");
    // The recovered output is the 8051 accumulator (ACC), the cspec output reg.
    // (render_vn appends a `:<size>` suffix when the size differs from the
    // default; ACC is 1 byte, rendered "ACC:1".)
    assert!(
        in1_name == "ACC" || in1_name.starts_with("ACC:"),
        "the recovered return value must be the 8051 accumulator ACC (got {in1_name})"
    );

    // (3) The dead stack-pop chain (LOAD/INT_ADD/LOAD/PIECE) is swept once the
    // RETURN no longer consumes it: the alive-op count drops below the pre-wave 9.
    use std::collections::BTreeMap;
    let mut by_code: BTreeMap<String, usize> = BTreeMap::new();
    for &op in &alive {
        *by_code.entry(format!("{:?}", fd.obank().get(op).unwrap().code())).or_default() += 1;
    }
    eprintln!("by opcode: {by_code:?}");
    assert!(
        n_alive < 9,
        "the dead stack-pop chain must be swept once RETURN reads the recovered output (expected < 9 alive ops, got {n_alive})"
    );
    // The raw 8051 RET stack-pop chain is gone: no PIECE concatenation survives
    // (it only existed to assemble the popped return address the RETURN consumed).
    assert!(
        !by_code.contains_key("CPUI_PIECE"),
        "the stack-pop PIECE must be swept (got {by_code:?})"
    );
}

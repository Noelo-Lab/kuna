//! GOLDEN GATE for item `w2-sleigh-core`: replay every per-instruction raw
//! p-code lift fixture in `tests/golden/vectors/lift/` (emitted by the C++
//! oracle's `golden lift` / `golden liftctx` console commands,
//! `decompiler/cpp/kuna_goldengen.cc`).
//!
//! Each fixture pins one corpus binary against one `.sla` language: for every
//! lift point, the C++ decoded up to 80 instructions and dumped each one as a
//! header line `insn <space>:<offset> <len>` followed by its emitted p-code
//! ops, ending with a deterministic `lifterror <class> <space>:<off> <msg>`.
//! The Rust SLEIGH runtime must reproduce each dump byte-for-byte: load the
//! binaryimage via `loadimage_xml`, decode the `.sla`, apply the recorded
//! context, and lift each instruction at the same byte address.
//!
//! The fixture body is reproduced exactly, including the LOSS-015 rule that a
//! LOAD/STORE space-pointer constant prints the space NAME (the C++ heap
//! pointer / Rust manager index normalized away).  16 fixtures, 16 distinct
//! SLEIGH languages, 1171 instructions — 100% required.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::spacetype;
use kuna_base::xml::{DocumentStorage, Element};
use kuna_num::opcodes::{get_opname, OpCode};
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::{register_loadimage_xml_ids, LoadImageXml};
use kuna_sleigh::sleigh::Sleigh;
use kuna_sleigh::translate::PcodeEmit;

/// Repository root (the test reads corpus XML + built `.sla` files from it).
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// goldenOpName: the canonical enum names (kuna_goldengen.cc's ZPULL/SPULL
/// fix is already the kuna `get_opname` behavior).
fn golden_op_name(opc: OpCode) -> &'static str {
    get_opname(opc)
}

/// PcodeEmit that renders raw lifted ops into a line buffer, exactly like the
/// C++ `GoldenLiftEmit` (two-space indent, opcode name, output or `-`, then
/// inputs).  Holds the manager to resolve spaceid-constant names.
struct LiftEmit<'a> {
    buf: Vec<String>,
    manager: &'a kuna_base::space::AddrSpaceManager,
}

impl PcodeEmit for LiftEmit<'_> {
    fn dump(
        &mut self,
        _addr: &Address,
        opc: OpCode,
        outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        let mut s = format!("  {} ", golden_op_name(opc));
        match outvar {
            None => s.push('-'),
            Some(v) => s.push_str(&self.render_varnode(v, false)),
        }
        for (i, v) in vars.iter().enumerate() {
            s.push(' ');
            let spaceid = i == 0 && (opc == OpCode::CPUI_LOAD || opc == OpCode::CPUI_STORE);
            s.push_str(&self.render_varnode(v, spaceid));
        }
        self.buf.push(s);
    }
}

impl LiftEmit<'_> {
    fn render_varnode(&self, vn: &VarnodeData, spaceid_const: bool) -> String {
        let spc = vn.space.as_ref().expect("varnode has a space");
        if spaceid_const && spc.get_type() == spacetype::IPTR_CONSTANT {
            let pointed = vn.get_space_from_const(self.manager).expect("spaceid resolves");
            format!("({},{},{})", spc.get_name(), pointed.get_name(), vn.size)
        } else {
            format!("({},0x{:x},{})", spc.get_name(), vn.offset, vn.size)
        }
    }
}

/// Map a lift error to its fixture class (kuna_goldengen.cc's catch ladder).
fn lifterror_class(e: &KunaError) -> &'static str {
    match e {
        KunaError::DataUnavail { .. } => "dataunavail",
        KunaError::Unimpl { .. } => "unimpl",
        KunaError::BadData { .. } => "baddata",
        // SleighError / LowlevelError and everything else in the Lowlevel
        // hierarchy.
        _ => "lowlevel",
    }
}

/// Recursively find the `<binaryimage>` element in a corpus document.
fn find_binaryimage(el: &Rc<Element>) -> Option<Rc<Element>> {
    if el.get_name() == "binaryimage" {
        return Some(Rc::clone(el));
    }
    for c in el.get_children() {
        if let Some(found) = find_binaryimage(c) {
            return Some(found);
        }
    }
    None
}

/// A parsed fixture: the metadata needed to set up the engine + the verbatim
/// expected body (everything from the first `insn`/`lifterror` line onward).
struct Fixture {
    corpus: String,
    sla_rel: String,
    /// `(space_name, byte_offset)` lift points, in document order.
    lift_points: Vec<(String, u64)>,
    /// `(name, value)` context variables in effect at the first lift point.
    context: Vec<(String, u32)>,
    /// The expected lift-dump body lines (insn/op/lifterror), in order.
    expected: Vec<String>,
}

fn parse_fixture(text: &str) -> Fixture {
    let mut corpus = String::new();
    let mut sla_rel = String::new();
    let mut lift_points = Vec::new();
    let mut context = Vec::new();
    let mut expected = Vec::new();
    let mut in_body = false;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("# corpus:") {
            corpus = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("# sla:") {
            // "# sla: <path> (relative to specs/)"
            let rest = rest.trim();
            sla_rel = rest.split(" (relative").next().unwrap().trim().to_string();
        } else if line.starts_with("# lift-points") {
            // "# lift-points (space:byte-offset name): ram:0x10020=cntlzwtest ..."
            let after = line.split("): ").nth(1).unwrap_or("");
            for tok in after.split_whitespace() {
                // "ram:0x10020=cntlzwtest" or "ram:0x10020"
                let sp = tok.split('=').next().unwrap();
                let mut it = sp.splitn(2, ':');
                let space = it.next().unwrap().to_string();
                let off_str = it.next().unwrap();
                let off = parse_u64(off_str);
                lift_points.push((space, off));
            }
        } else if let Some(kv) = line.strip_prefix("context ") {
            // "context name=value"
            let mut it = kv.splitn(2, '=');
            let name = it.next().unwrap().to_string();
            let val: u32 = it.next().unwrap().trim().parse().unwrap();
            context.push((name, val));
        } else if line.starts_with("insn ") || line.starts_with("lifterror ") {
            in_body = true;
            expected.push(line.to_string());
        } else if in_body && (line.starts_with("  ") || line.starts_with('\t')) {
            // an op line (two-space indent)
            expected.push(line.to_string());
        } else if line.starts_with("# golden lift ") {
            // a per-lift-point banner; not part of the comparable body
            in_body = false;
        }
    }
    Fixture { corpus, sla_rel, lift_points, context, expected }
}

fn parse_u64(s: &str) -> u64 {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x") {
        u64::from_str_radix(hex, 16).unwrap()
    } else {
        s.parse().unwrap()
    }
}

/// Load image stub that errors on read (replaced by the corpus image after
/// the `.sla` is decoded).
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

/// Run one fixture and return the produced body lines.
fn run_fixture(fix: &Fixture) -> Vec<String> {
    let root = repo_root();
    let xml = std::fs::read(root.join(&fix.corpus)).expect("read corpus xml");
    let mut store = DocumentStorage::new();
    let doc_root = store.parse_document(&xml).expect("parse corpus xml").get_root().clone();
    let bi = find_binaryimage(&doc_root).expect("corpus has a binaryimage");

    let sla = std::fs::read(root.join("specs").join(&fix.sla_rel)).expect("read .sla");

    let mut registry = IdRegistry::with_base_ids();
    register_loadimage_xml_ids(&mut registry);

    let mut img = LoadImageXml::new(&fix.corpus, bi).expect("LoadImageXml::new");
    let ctx = Box::new(ContextInternal::new());
    let mut sleigh = Sleigh::new(Box::new(DummyImg), ctx);
    sleigh.initialize_from_sla(&sla).expect("initialize from sla");

    // Apply the recorded context (registered by initialize via build_xrefs;
    // set the per-fixture default values).
    for (name, val) in &fix.context {
        // best-effort: variables not registered by the language are skipped
        // (the fixture lists every registered variable, so all should exist).
        sleigh.set_context_default(name, *val);
    }
    // Re-painting through defaults requires the values be applied before any
    // decode; ContextInternal::setVariableDefault writes the default blob,
    // which getContext returns for unset addresses.

    img.open(sleigh.base().manager(), &registry).expect("open corpus image");
    sleigh.set_loader(Box::new(img));

    let mut out: Vec<String> = Vec::new();
    for (space, off) in &fix.lift_points {
        let spc = Rc::clone(
            sleigh.base().manager().get_space_by_name(space).expect("lift-point space"),
        );
        let mut addr = Address::new(spc, *off);
        for _ in 0..80 {
            // Resolve the manager pointer for the emitter (no host pointers).
            let manager_ptr: *const kuna_base::space::AddrSpaceManager =
                sleigh.base().manager();
            let mut emit = LiftEmit { buf: Vec::new(), manager: unsafe { &*manager_ptr } };
            match sleigh.one_instruction(&mut emit, &addr) {
                Ok(len) => {
                    out.push(format!("insn {}:0x{:x} {}", space, addr.get_offset(), len));
                    out.extend(emit.buf);
                    addr = &addr + i64::from(len);
                }
                Err(e) => {
                    out.push(format!(
                        "lifterror {} {}:0x{:x} {}",
                        lifterror_class(&e),
                        space,
                        addr.get_offset(),
                        e
                    ));
                    break;
                }
            }
        }
    }
    out
}

/// Set::set_context_default needs trait import in scope.
use kuna_sleigh::translate::Translate;

macro_rules! fixture_test {
    ($name:ident, $file:literal) => {
        #[test]
        fn $name() {
            let root = repo_root();
            let path = root.join("tests/golden/vectors/lift").join($file);
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read fixture {}: {e}", path.display()));
            let fix = parse_fixture(&text);
            let got = run_fixture(&fix);
            if got != fix.expected {
                // Report the first divergence for a readable failure.
                let n = got.len().min(fix.expected.len());
                let mut first = None;
                for i in 0..n {
                    if got[i] != fix.expected[i] {
                        first = Some(i);
                        break;
                    }
                }
                let idx = first.unwrap_or(n);
                let ctx_lo = idx.saturating_sub(3);
                eprintln!("--- fixture {} diverges at body line {} ---", $file, idx);
                for i in ctx_lo..(idx + 4).min(got.len().max(fix.expected.len())) {
                    let g = got.get(i).map(String::as_str).unwrap_or("<missing>");
                    let e = fix.expected.get(i).map(String::as_str).unwrap_or("<missing>");
                    let mark = if g == e { " " } else { ">" };
                    eprintln!("{mark} [{i}] got : {g}");
                    eprintln!("{mark} [{i}] want: {e}");
                }
                panic!(
                    "fixture {} mismatch (got {} lines, want {} lines)",
                    $file,
                    got.len(),
                    fix.expected.len()
                );
            }
        }
    };
}

fixture_test!(golden_lift_boolless, "boolless.txt");
fixture_test!(golden_lift_ccmp, "ccmp.txt");
fixture_test!(golden_lift_condexesub, "condexesub.txt");
fixture_test!(golden_lift_floatprint, "floatprint.txt");
fixture_test!(golden_lift_gh6990_returnpair, "gh6990-returnpair.txt");
fixture_test!(golden_lift_gh7139_x86_16, "gh7139-x86-16-disp16-signed.txt");
fixture_test!(golden_lift_gh8817_v850, "gh8817-v850indbranch.txt");
fixture_test!(golden_lift_gh8844_avr, "gh8844-avr-rjmpwrap.txt");
fixture_test!(golden_lift_gh8913_6502, "gh8913-addcarrychain.txt");
fixture_test!(golden_lift_gh9001_hcs12, "gh9001-hcs12brn.txt");
fixture_test!(golden_lift_gh9203_riscv, "gh9203-condexe-loopcopy.txt");
fixture_test!(golden_lift_gp, "gp.txt");
fixture_test!(golden_lift_lzcount, "lzcount.txt");
fixture_test!(golden_lift_promotecompare, "promotecompare.txt");
fixture_test!(golden_lift_readvolatile, "readvolatile.txt");
fixture_test!(golden_lift_skipnext2, "skipnext2.txt");

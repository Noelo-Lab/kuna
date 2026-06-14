//! INDEPENDENT VERIFIER harness (item: w10-structure-printbody, Round 2).
//!
//! Reproduces the Round-1 verifier's datatest-string-match measurement to
//! confirm the Round-2 gate claim: the rust-engine datatest assertion count
//! rose AND the new passes include REAL positive (min>=1) content assertions
//! (not just vacuous min=0/max=0 "must-not-appear" matches).
//!
//! For every `decompiler/datatests/*.xml`, bootstrap the full XML frontend,
//! `decompile_func` + `print_c` every `<symbol>`, then evaluate every
//! `<stringmatch min max>` regex over the concatenated rendered C, exactly the
//! way the C++ datatest harness scores `min<=hits<=max`.  Classify each PASS as
//! positive (min>=1) or negative (min=0,max=0) and print the tally.
//!
//! This test never asserts a count (the corpus moves); it is a measurement
//! probe driven with `--nocapture`.  It is committed with the verdict so the
//! measurement is reproducible.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::AddrSpaceManager;
use kuna_base::xml::{DocumentStorage, Element};

use kuna_decomp::decompile_drive::{decompile_func, print_c};
use kuna_decomp::options::{register_option_elements, OptionDatabase};
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

struct StringMatch {
    name: String,
    min: i64,
    max: i64,
    /// The (XML-unescaped) regex text.
    pattern: String,
}

struct DataTest {
    binaryimage: Rc<Element>,
    arch_id: String,
    symbols: Vec<SymbolFn>,
    options: Vec<(String, String, String, String)>,
    matches: Vec<StringMatch>,
}

fn xml_unescape(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
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

fn parse_datatest(path: &std::path::Path) -> Result<DataTest, String> {
    let xml = std::fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let mut store = DocumentStorage::new();
    let root = store
        .parse_document(&xml)
        .map_err(|e| format!("parse {}: {e}", path.display()))?
        .get_root()
        .clone();

    let mut bins = Vec::new();
    find_named(&root, "binaryimage", &mut bins);
    let binaryimage = bins.first().ok_or("no <binaryimage>")?.clone();

    let mut langs = Vec::new();
    find_named(&root, "language", &mut langs);
    // The arch id lives on <decompilertest>/<language id=..> or on <binaryimage arch=..>.
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

    let mut coms = Vec::new();
    find_named(&root, "com", &mut coms);
    let options = coms
        .iter()
        .filter_map(|c| parse_option_com(&String::from_utf8_lossy(c.get_content())))
        .collect();

    let mut sms = Vec::new();
    find_named(&root, "stringmatch", &mut sms);
    let matches = sms
        .iter()
        .map(|m| StringMatch {
            name: attr(m, "name").unwrap_or_default(),
            min: attr(m, "min").map(|s| parse_u64(&s) as i64).unwrap_or(1),
            max: attr(m, "max").map(|s| parse_u64(&s) as i64).unwrap_or(1),
            pattern: xml_unescape(&String::from_utf8_lossy(m.get_content())),
        })
        .collect();

    Ok(DataTest { binaryimage, arch_id, symbols, options, matches })
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

    // Hand the resolved cspec/pspec to the architecture (the C++
    // parseCompilerConfig/parseProcessorConfig inputs) — the pspec
    // `<context_data>` is what steers the SLEIGH disassembly mode (e.g. x86-64
    // lifts as 64-bit, not 16-bit real mode).  Mirrors `bootstrap_program`.
    if !specs.compilerfile.is_empty() {
        if let Ok(cspec) = std::fs::read(&specs.compilerfile) {
            arch.sleigh_mut().base_mut().unwrap().set_cspec_xml(cspec);
        }
    }
    if !specs.processorfile.is_empty() {
        if let Ok(pspec) = std::fs::read(&specs.processorfile) {
            arch.sleigh_mut().base_mut().unwrap().set_pspec_xml(pspec);
        }
    }

    // Install the register-name lookup on the engine manager before
    // init_post_engine resolves the pspec `<tracked_set>` register names.
    arch.sleigh_mut()
        .base_mut()
        .unwrap()
        .translate_mut()
        .install_register_lookup()
        .map_err(|e| format!("install_register_lookup: {e}"))?;

    arch.sleigh_mut()
        .base_mut()
        .ok_or("no Architecture base after build_translator")?
        .init_post_engine()
        .map_err(|e| format!("init_post_engine: {e}"))?;

    let manager_ptr: *const AddrSpaceManager = arch.sleigh().base().unwrap().manage();
    arch.open_image(unsafe { &*manager_ptr }, &registry).map_err(|e| format!("open_image: {e}"))?;
    let img = arch.take_loader().ok_or("loader vanished after open")?;
    arch.sleigh_mut().base_mut().unwrap().set_loader(Box::new(img));
    Ok(arch)
}

/// Bootstrap + decompile every symbol; return the concatenated rendered C (the
/// C++ harness matches over the whole printed output of the test).
fn render_corpus(dt: &DataTest) -> Result<String, String> {
    let registry = build_registry();
    let mut xarch = bootstrap(dt)?;

    // Apply option commands (best-effort, like the e2e harness).
    {
        let options = OptionDatabase::new();
        if let Some(base) = xarch.sleigh_mut().base_mut() {
            for (name, p1, p2, p3) in &dt.options {
                let id = registry.find_element(name, 0);
                if id != 0 {
                    let _ = options.set(base, id, p1, p2, p3);
                }
            }
        }
    }

    let mut out = String::new();
    for sym in &dt.symbols {
        let base = xarch.sleigh_mut().base_mut().ok_or("no base")?;
        let space = match base.manage().get_space_by_name(&sym.space) {
            Some(s) => Rc::clone(s),
            None => continue,
        };
        let entry = Address::new(space, sym.offset);
        if let Ok(fd) = decompile_func(base, &sym.name, entry, 0) {
            out.push_str(&print_c(base, &fd));
            out.push('\n');
        }
    }
    Ok(out)
}

/// Evaluate a regex over `text`, counting NON-OVERLAPPING matches (regex crate
/// `find_iter` semantics — same family as the C++ harness's `std::regex_search`
/// loop).
fn count_matches(pattern: &str, text: &str) -> Result<usize, String> {
    let re = regex::Regex::new(pattern).map_err(|e| format!("bad regex `{pattern}`: {e}"))?;
    Ok(re.find_iter(text).count())
}

#[test]
fn verify_w10_corpus_stringmatch_tally() {
    let dir = repo_root().join("decompiler/datatests");
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .expect("read datatests dir")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "xml").unwrap_or(false))
        .collect();
    files.sort();

    let mut total_assertions = 0usize;
    let mut pass_positive = 0usize; // min>=1 satisfied
    let mut pass_negative = 0usize; // min==0 && max==0 satisfied (absence)
    let mut fail_positive = 0usize;
    let mut fail_negative = 0usize;
    let mut bootstrap_skips = 0usize;
    let mut positive_pass_samples: Vec<String> = Vec::new();

    for path in &files {
        let dt = match parse_datatest(path) {
            Ok(d) => d,
            Err(_) => continue,
        };
        if dt.matches.is_empty() {
            continue;
        }
        let rendered = match render_corpus(&dt) {
            Ok(r) => r,
            Err(_) => {
                bootstrap_skips += 1;
                continue;
            }
        };
        let stem = path.file_stem().unwrap().to_string_lossy().into_owned();
        for sm in &dt.matches {
            total_assertions += 1;
            let hits = count_matches(&sm.pattern, &rendered).unwrap_or(usize::MAX) as i64;
            let pass = hits >= sm.min && (sm.max < 0 || hits <= sm.max);
            let is_positive = sm.min >= 1;
            match (pass, is_positive) {
                (true, true) => {
                    pass_positive += 1;
                    if positive_pass_samples.len() < 30 {
                        positive_pass_samples
                            .push(format!("{stem}::{}  ~  `{}`", sm.name, sm.pattern));
                    }
                }
                (true, false) => pass_negative += 1,
                (false, true) => fail_positive += 1,
                (false, false) => fail_negative += 1,
            }
        }
    }

    eprintln!("=== w10 corpus stringmatch tally (rust engine) ===");
    eprintln!("datatest files: {}", files.len());
    eprintln!("bootstrap/arch skips: {bootstrap_skips}");
    eprintln!("total assertions evaluated: {total_assertions}");
    eprintln!("PASS positive (min>=1, REAL content): {pass_positive}");
    eprintln!("PASS negative (min=0 max=0, absence):  {pass_negative}");
    eprintln!("FAIL positive: {fail_positive}");
    eprintln!("FAIL negative: {fail_negative}");
    eprintln!("--- sample of REAL positive passes ---");
    for s in &positive_pass_samples {
        eprintln!("  + {s}");
    }

    // The verdict-relevant claim: at least one REAL positive (min>=1) assertion
    // passes by genuine parity (the Round-1 REJECT condition was 0 such passes).
    assert!(
        pass_positive >= 1,
        "expected >= 1 real positive (min>=1) datatest assertion to pass; got {pass_positive}"
    );
}

// ===========================================================================
// w10-refinement-loops verifier (this un-seam)
//
// The blocker the round-1 verifier found was a MIS-LIFT: the x86:LE:64 corpus
// decompiled as 16-bit real-mode garbage because the engine never applied the
// processor-spec `<context_data>` `<context_set>` paints (addrsize/opsize/
// longMode) that steer SLEIGH's disassembly mode.  The fix wires
// `Architecture::parse_processor_config` (the C++ `parseProcessorConfig`
// `ELEM_CONTEXT_DATA` branch) so the engine context is correct.  Only with the
// CORRECT 64-bit lift does the (already-faithful) refinement + loop structurer
// produce oracle-direction output.
//
// These two tests assert against the C++ ORACLE's direction (taken from the
// datatest XML and the C++ decomp_test_dbg, which scores divopt 34/34 and
// forloop1 1/1), NOT against a substring of a mis-lift:
//   (1) divopt: its oracle is 100% STRAIGHT-LINE division (no loop), all 34
//       assertions Division/Modulo.  The test pins the 64-bit lift (RDI/RSP,
//       NOT 16-bit AX/SI/DI) and that NO bogus loop keyword appears — the
//       round-1 `do { } while` was an artifact of the garbage lift.
//   (2) forloop1: its oracle asserts a real `for (... )` loop.  The test pins
//       that the loop structurer + emitter render a real C loop KEYWORD on a
//       function whose oracle is itself a loop (the keyword comes from the
//       structurer collapse, not a hand-built tree).
// ===========================================================================

/// `divopt` (x86:LE:64) is, in the C++ oracle, entirely straight-line integer
/// division (`*divu = *divu / 81;` …) — 34 Division/Modulo assertions, no loop.
/// Round 1 mis-lifted it as 16-bit real-mode garbage (AX/SI/DI, 0xffff), and the
/// then-`do { } while` it celebrated was an artifact of that garbage.  With the
/// pspec `<context_data>` paints applied the lift is correct 64-bit: this test
/// pins the lift DIRECTION against the oracle — 64-bit registers present, the
/// 16-bit garbage registers ABSENT, and NO bogus loop keyword (the oracle is
/// loop-free here).  It does NOT pin the full division text (the reciprocal-
/// multiply -> `/` recognition + pointer typing is the documented NEXT blocker).
#[test]
fn verify_w10_refinement_divopt_lifts_64bit_not_16bit_garbage() {
    let path = repo_root().join("decompiler/datatests/divopt.xml");
    let dt = parse_datatest(&path).expect("parse divopt.xml");
    let rendered = render_corpus(&dt).expect("divopt must decompile");

    assert!(
        rendered.contains("divoptu") && rendered.contains("modoptu"),
        "divopt must render its functions:\n{rendered}"
    );

    // A correct lift uses 64-bit arithmetic; the round-1 garbage lift used 16-bit
    // real mode.  With the W10 symbol-naming un-seam the 64-bit first-arg pointer
    // (RDI) is now bound to its recovered parameter `divu`, so the body no longer
    // shows the bare register — the 64-bit-ness is pinned by the wide arithmetic
    // mask (a 16-bit lift could at most produce 0xffff) and the named pointer.
    let wide_mask = count_matches(r"0xffffffffffffffff", &rendered).unwrap_or(0);
    let sixteen = count_matches(r"\b(AX|SI|DI|BX|CX|DX|SP|BP)\b", &rendered).unwrap_or(0);
    assert!(
        wide_mask >= 1 || rendered.contains("divu"),
        "divopt must lift with 64-bit arithmetic (wide mask / named 64-bit pointer); \
         got neither:\n{rendered}"
    );
    assert_eq!(
        sixteen, 0,
        "divopt must NOT lift as 16-bit real mode (no bare AX/SI/DI — the round-1 \
         garbage signature); found {sixteen}:\n{rendered}"
    );

    // The oracle for divopt is straight-line: there must be NO loop keyword.
    // (The round-1 `do {{ }} while` was a wrong-direction artifact of the
    // garbage lift; it must be gone.)
    let loops = count_matches(r"\bwhile *\(| while *\( *true *\)|\bfor *\(|\bdo \{", &rendered)
        .unwrap_or(0);
    assert_eq!(
        loops, 0,
        "divopt's oracle is loop-free (straight-line division); a loop keyword here \
         is a wrong-direction structuring artifact:\n{rendered}"
    );
}

/// The loop structurer + emitters are validated against a function whose C++
/// ORACLE is itself a loop: `forloop1` asserts `for (.* v1 = 0; v1 < max; v1 =
/// v1 + 1)` in `decompiler/datatests/forloop1.xml` (the C++ decomp_test_dbg
/// scores it 1/1).  With the correct 64-bit lift the loop CFG collapses and the
/// emitter renders a real C loop KEYWORD (`while`/`for`/`do`).  This pins the
/// loop emitters (emitBlockWhileDo/InfLoop/…) on an oracle-loop function — the
/// keyword comes from the structurer collapse driving the printer, NOT from a
/// hand-built tree or a mis-lift substring.  (The exact `for (v1=0; …)`
/// normalization + variable typing is the documented NEXT blocker; the engine
/// currently renders a `while( true ) { … break; }` of the same loop.)
#[test]
fn verify_w10_loop_emitter_renders_real_loop_on_oracle_loop_fn() {
    let path = repo_root().join("decompiler/datatests/forloop1.xml");
    let dt = parse_datatest(&path).expect("parse forloop1.xml");
    let rendered = render_corpus(&dt).expect("forloop1 must decompile");

    assert!(
        rendered.contains("forloop1"),
        "forloop1 must render its function:\n{rendered}"
    );
    // A real C loop keyword from the structurer collapse: `while (`/`for (`/`do {`.
    let loop_kw =
        count_matches(r"\bwhile *\(|\bwhile\( *true *\)|\bfor *\(|\bdo \{", &rendered).unwrap_or(0);
    assert!(
        loop_kw >= 1,
        "forloop1's oracle is a `for` loop; the structurer + emitter must render a \
         real C loop keyword (while/for/do), got none:\n{rendered}"
    );
}

// ===========================================================================
// VERIFIER ADVERSARIAL TESTS (w10-refinement-loops, ROUND 2)
//
// The Round-1 blocker (BLOCKER-1) was that divopt mis-lifted as 16-bit garbage,
// so the "applied count rise" was vacuous (the celebrated `do-while` was a
// wrong-direction artifact).  These tests pin the FIX itself in a way a 16-bit
// garbage lift cannot pass — stronger than the porter's substring checks:
//   - the 64-bit reciprocal-multiply division CONSTANTS must be present (a
//     16-bit real-mode lift cannot materialize a 64-bit `* 0x948b…` multiply);
//   - the straight-line body must STORE through the RDI pointer for the array
//     elements (the oracle's `*divu`/`divu[N]` direction), not loop;
//   - the loop function's collapse must produce a bounded body (a `break`, the
//     loop counter referenced), not a bare keyword.
// They are committed with the verdict so the un-faked rise stays pinned.
// ===========================================================================

/// ADVERSARIAL: divopt's 64-bit reciprocal-multiply division CONSTANTS must
/// appear in the lift.  Compiler-emitted unsigned `/ const` lowers to a wide
/// `value * magic >> shift`; the magic constants (e.g. `0x948b0fcd6e9e0653`
/// for `/81`) are 64-bit and *cannot* be produced by a 16-bit real-mode lift
/// (the Round-1 garbage signature).  This is a much harder gate than "an RDI
/// appears": it proves the wide multiply was actually decoded.
#[test]
fn verify_w10_r2_divopt_reciprocal_multiply_is_64bit() {
    let path = repo_root().join("decompiler/datatests/divopt.xml");
    let dt = parse_datatest(&path).expect("parse divopt.xml");
    let rendered = render_corpus(&dt).expect("divopt must decompile");

    // The reciprocal magic for the first divisor (/81) in divoptu, and the
    // 64-bit sign/shift mask the lowering uses.  Both are 64-bit literals.
    let magic = count_matches(r"0x948b0fcd6e9e0653", &rendered).unwrap_or(0);
    assert!(
        magic >= 1,
        "divopt must lift the 64-bit reciprocal-multiply magic constant \
         (impossible under a 16-bit real-mode lift):\n{rendered}"
    );
    // A 64-bit-wide shift/negate mask (0xffffffffffffffff) — a 16-bit lift
    // would at most produce 0xffff.  Its presence proves 64-bit arithmetic.
    let wide_mask = count_matches(r"0xffffffffffffffff", &rendered).unwrap_or(0);
    let narrow_mask = count_matches(r"\b0xffff\b", &rendered).unwrap_or(0);
    assert!(
        wide_mask >= 1,
        "divopt must use 64-bit-wide arithmetic masks:\n{rendered}"
    );
    assert_eq!(
        narrow_mask, 0,
        "divopt must NOT contain the 16-bit real-mode 0xffff signature \
         (found {narrow_mask}):\n{rendered}"
    );
}

/// ADVERSARIAL: divopt's body must STORE through the RDI pointer for the array
/// elements in STRAIGHT-LINE form (the oracle direction: `*divu = …; divu[N] =
/// …;`).  The lift renders these as `STORE(…,RDI + 0xNN, …)` one per element
/// with NO loop — a 16-bit garbage lift produced neither a coherent pointer
/// nor straight-line stores.  Pins both the pointer recovery direction and the
/// absence of a spurious loop on a known loop-free oracle function.
#[test]
fn verify_w10_r2_divopt_stores_through_rdi_straightline() {
    let path = repo_root().join("decompiler/datatests/divopt.xml");
    let dt = parse_datatest(&path).expect("parse divopt.xml");
    let rendered = render_corpus(&dt).expect("divopt must decompile");

    // Multiple distinct array-element stores through the recovered pointer
    // parameter (the oracle has 17 elements per function); require several to
    // prove the straight-line body.  The W10 symbol-naming un-seam binds the RDI
    // first-arg pointer to its recovered parameter, so the stores now render
    // through that NAME (the oracle direction `*p = …; p[N] = …`), not the raw
    // register.  The store target is the bound parameter identifier followed by the
    // element offset (`a0`/`a0 + 0xNN` when unlocked, the declared name when locked)
    // — a single-token name, never the raw `RDI` register and never `$$undef`.
    let ptr_stores = count_matches(r"STORE\([0-9]+,[A-Za-z_][A-Za-z0-9_]*(?: \+ 0x)?", &rendered)
        .unwrap_or(0);
    assert!(
        ptr_stores >= 8,
        "divopt must STORE through the recovered pointer for the array elements in \
         straight-line form (oracle: `*p = …; p[N] = …`); got {ptr_stores}:\n{rendered}"
    );
    // The store target must NOT be the raw first-arg register (the un-seam binds it
    // to the parameter) nor the `$$undef` placeholder (a coherent name was assigned).
    assert_eq!(
        count_matches(r"STORE\([0-9]+,RDI\b", &rendered).unwrap_or(0),
        0,
        "divopt's stores must render through the bound parameter, not the raw RDI \
         register:\n{rendered}"
    );
    assert_eq!(
        count_matches(r"\$\$undef", &rendered).unwrap_or(0),
        0,
        "divopt must not leak the `$$undef` placeholder name:\n{rendered}"
    );
    // And NO loop keyword — the oracle for divopt is entirely loop-free, so any
    // loop here is the Round-1 wrong-direction structuring artifact.
    let loops =
        count_matches(r"\bwhile *\(|\bfor *\(|\bdo \{", &rendered).unwrap_or(0);
    assert_eq!(
        loops, 0,
        "divopt is loop-free in the oracle; a loop keyword is a wrong-direction \
         artifact:\n{rendered}"
    );
}

/// ADVERSARIAL: forloop1's structurer collapse must be a REAL bounded loop, not
/// a bare keyword.  The oracle is `for(v1=0; v1<max; v1=v1+1)`; the engine
/// renders `while( true ) { … break; }`.  Require BOTH the loop keyword AND a
/// `break` inside it (the structurer recognized the exit edge) AND the
/// induction step (`+ 1`) — a substring like `while` in a comment could not
/// satisfy all three.  This pins the loop EMITTER on a genuine collapse.
#[test]
fn verify_w10_r2_forloop1_is_bounded_loop_not_bare_keyword() {
    let path = repo_root().join("decompiler/datatests/forloop1.xml");
    let dt = parse_datatest(&path).expect("parse forloop1.xml");
    let rendered = render_corpus(&dt).expect("forloop1 must decompile");

    let loop_kw =
        count_matches(r"\bwhile *\(|\bfor *\(|\bdo \{", &rendered).unwrap_or(0);
    assert!(loop_kw >= 1, "forloop1 must render a loop keyword:\n{rendered}");
    // The structurer found the loop's exit edge -> a `break;` inside the body.
    let breaks = count_matches(r"\bbreak;", &rendered).unwrap_or(0);
    assert!(
        breaks >= 1,
        "forloop1's loop collapse must emit a `break;` for the recovered exit \
         edge (proves a real CFG collapse, not a bare keyword):\n{rendered}"
    );
    // The induction step (the counter increment) survives in the body — proves
    // the loop body, not an empty/garbage loop, was structured.
    let step = count_matches(r"\+ 1", &rendered).unwrap_or(0);
    assert!(
        step >= 1,
        "forloop1's loop body must carry the induction step (`+ 1`):\n{rendered}"
    );
}

// ===========================================================================
// w10-symbol-naming verifier (this un-seam)
//
// The blocker the prior reviews named: ActionInputPrototype + typing recover the
// parameters and ScopeLocal::restructure promotes the stack locals, so the
// SIGNATURE renders typed named params — but the BODY still showed the raw
// registers / stack addresses (`RDI`/`RSI`/`RSP + off`) because the recovered
// Symbols were never bound to the body HighVariables.  This un-seam materializes
// the recovered parameters as ScopeLocal Symbols (`ProtoStoreSymbol::setInput`)
// and routes `ActionNameVars::linkSymbols` through the symbol query so the body
// HighVariables carry the recovered names.  These tests pin the binding on the
// integer-pointer corpus the un-seam targets.
// ===========================================================================

/// `readstruct` (nestedoffset.xml) takes `(twostruct *ptr,int8 a,int8 b)` in the
/// signature.  Before the un-seam the body referenced the raw argument registers
/// `RDI`/`RSI`/`RDX`; after binding the recovered parameter Symbols the body must
/// reference the recovered parameter NAMES (`a0`/`a1`/`a2` on the unlocked corpus
/// path, the declared names on the locked datatest path) — never the raw
/// registers, and the parameters must NOT be re-declared as body locals.
#[test]
fn verify_w10_symbol_naming_readstruct_body_uses_param_names_not_registers() {
    let path = repo_root().join("decompiler/datatests/nestedoffset.xml");
    let dt = parse_datatest(&path).expect("parse nestedoffset.xml");
    let rendered = render_corpus(&dt).expect("readstruct must decompile");

    assert!(
        rendered.contains("readstruct"),
        "readstruct must render its function:\n{rendered}"
    );
    // The first-arg pointer (RDI) must be bound to its recovered parameter and used
    // in the LOAD address; the raw argument registers must be gone from the body.
    let raw_args = count_matches(r"\b(RDI|RSI|RDX|EDI|ESI|EDX)\b", &rendered).unwrap_or(0);
    assert_eq!(
        raw_args, 0,
        "readstruct's body must reference the bound parameter names, not the raw \
         argument registers (RDI/RSI/RDX); found {raw_args}:\n{rendered}"
    );
    // The recovered first parameter (`a0`) is the LOAD base — proves the body
    // HighVariable carries the parameter Symbol, the un-seam's core effect.
    let param_in_load = count_matches(r"LOAD\([0-9]+,a0", &rendered).unwrap_or(0);
    assert!(
        param_in_load >= 1,
        "readstruct must LOAD through the bound first parameter (`a0`):\n{rendered}"
    );
    // A parameter must never leak the `$$undef` placeholder name.
    assert_eq!(
        count_matches(r"\$\$undef", &rendered).unwrap_or(0),
        0,
        "no `$$undef` placeholder may leak into the body:\n{rendered}"
    );
    // Parameters render in the signature, not the body decl block: there must be no
    // local declaration of the parameter name (`int8 a0;` etc.).
    assert_eq!(
        count_matches(r"(?m)^\s+\w[\w ]*\ba0 *;", &rendered).unwrap_or(0),
        0,
        "a recovered parameter must not be re-declared as a body local:\n{rendered}"
    );
}

// ===========================================================================
// PSPEC-CONTEXT LOAD-PATH REGRESSION (item: w10-pspec-context)
//
// The processor-spec `<context_data><context_set>` defaults (x86-64's
// addrsize=2/opsize=1/longMode=1) must be applied to the engine's ContextDatabase
// *before* SLEIGH disassembles, on the real datatest console load path
// (bootstrap_program -> Architecture::init_post_engine -> parse_processor_config,
// the slice of C++ Architecture::parseProcessorConfig that dispatches
// ELEM_CONTEXT_DATA to context->decodeFromSpec).  Without it x86:LE:64 decodes as
// 16-bit real mode (SP+0xfffe, BX+SI, BP, DS/SS segment CALLOTHER) and the
// loop-NAMED datatests form no back-edges and never structure.
//
// These tests pin that property on the exact loop-named files this item is
// about (loopcomment, elseif) — files a 16-bit real-mode lift could not pass:
// the 64-bit registers must be present, the 16-bit real-mode garbage signature
// absent, and the structurer must produce real keywords.  They are NOT a
// substring of a mis-lift; the same render path the C++ harness scores.
// ===========================================================================

/// `loopcomment` (x86:LE:64) carries three real loops (for/while/do-while in the
/// source).  With the pspec `<context_data>` paints applied the lift is 64-bit
/// (RSP/RBP/RDI/RSI, 8-byte frame offsets like `0xfffffffffffffff8`) and the
/// loops collapse to real C loop keywords.  A 16-bit real-mode lift (the bug
/// this item fixes) shows `SP + 0xfffe`/`BX + SI`/`CALLOTHER(0,DS,…)` and forms
/// no back-edges.  This pins the pspec-context load path on a named loop file.
#[test]
fn verify_w10_pspec_context_loopcomment_lifts_64bit_and_structures() {
    let path = repo_root().join("decompiler/datatests/loopcomment.xml");
    let dt = parse_datatest(&path).expect("parse loopcomment.xml");
    let rendered = render_corpus(&dt).expect("loopcomment must decompile");

    assert!(
        rendered.contains("loopcomment"),
        "loopcomment must render its function:\n{rendered}"
    );

    // 64-bit registers present (RSP/RBP/RDI/…); the 16-bit real-mode garbage
    // signature absent.  `\bSP \+ 0xfffe\b`-class offsets and `BX + SI` and the
    // `CALLOTHER(0,DS|SS` segment ops are the unambiguous 16-bit-real-mode marks.
    let sixtyfour = count_matches(r"\bR(SP|BP|DI|SI|AX|BX|CX|DX)\b", &rendered).unwrap_or(0);
    assert!(
        sixtyfour >= 1,
        "loopcomment must lift with 64-bit registers (RSP/RBP/…); got none \
         (the pspec <context_data> paints were not applied):\n{rendered}"
    );
    let realmode =
        count_matches(r"\bBX \+ SI\b|CALLOTHER\(0,DS|CALLOTHER\(0,SS|\b0xfffe\b", &rendered)
            .unwrap_or(0);
    assert_eq!(
        realmode, 0,
        "loopcomment must NOT lift as 16-bit real mode (BX+SI / segment CALLOTHER \
         / 0xfffe — the bug this item fixes); found {realmode}:\n{rendered}"
    );

    // The loops collapse to real C loop keywords (proves the back-edges formed,
    // which they cannot under a real-mode lift that never reaches the targets).
    let loop_kw = count_matches(r"\bwhile *\(|\bfor *\(|\bdo \{", &rendered).unwrap_or(0);
    assert!(
        loop_kw >= 1,
        "loopcomment's loops must structure to real C loop keywords once the \
         64-bit lift forms the back-edges:\n{rendered}"
    );
}

/// `forloop_varused` (x86:LE:64, function `forloop_loopvarused`) is a second
/// loop-NAMED file: a `for` loop whose induction variable is used after the
/// loop.  With the pspec context paints the lift is 64-bit (RSP frame) and the
/// loop collapses to a real C loop keyword; a 16-bit real-mode lift forms no
/// back-edge and never structures.  (The `elseif` file's function is introduced
/// by `map fun` *script* commands, not a `<symbol>` tag, so it is exercised by
/// the live datatest runner — see the gate report — rather than this
/// `<symbol>`-iterating render path; `forloop_varused` carries a `<symbol>` and
/// so renders here.)
#[test]
fn verify_w10_pspec_context_forloop_varused_lifts_64bit_and_structures() {
    let path = repo_root().join("decompiler/datatests/forloop_varused.xml");
    let dt = parse_datatest(&path).expect("parse forloop_varused.xml");
    let rendered = render_corpus(&dt).expect("forloop_varused must decompile");

    assert!(
        rendered.contains("forloop_loopvarused"),
        "forloop_varused must render its function:\n{rendered}"
    );

    let sixtyfour = count_matches(r"\bR(SP|BP|DI|SI|AX|BX|CX|DX)\b", &rendered).unwrap_or(0);
    assert!(
        sixtyfour >= 1,
        "forloop_varused must lift with 64-bit registers (RSP/…); got none \
         (the pspec <context_data> paints were not applied):\n{rendered}"
    );
    let realmode =
        count_matches(r"\bBX \+ SI\b|CALLOTHER\(0,DS|CALLOTHER\(0,SS|\b0xfffe\b", &rendered)
            .unwrap_or(0);
    assert_eq!(
        realmode, 0,
        "forloop_varused must NOT lift as 16-bit real mode; found {realmode}:\n{rendered}"
    );
    // The loop collapses to a real C loop keyword once the 64-bit back-edge forms.
    let loop_kw = count_matches(r"\bwhile *\(|\bfor *\(|\bdo \{", &rendered).unwrap_or(0);
    assert!(
        loop_kw >= 1,
        "forloop_varused's loop must structure to a real C loop keyword:\n{rendered}"
    );
}

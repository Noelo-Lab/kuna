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

    // A correct 64-bit lift now RECONSTRUCTS the reciprocal-multiply into clean
    // `/ N` division (the w10-divmod un-seam), so the proof of 64-bit-ness is the
    // reconstructed decimal divisors themselves — a 16-bit real-mode garbage lift
    // could never produce them.  The wide `* -1`/magic correction is consumed by
    // the reconstruction (so the 0xffffffffffffffff mask is gone, not present).
    let div_const = count_matches(r"/ 81\b", &rendered).unwrap_or(0)
        + count_matches(r"/ 89\b", &rendered).unwrap_or(0)
        + count_matches(r"/ 91\b", &rendered).unwrap_or(0);
    let sixteen = count_matches(r"\b(AX|SI|DI|BX|CX|DX|SP|BP)\b", &rendered).unwrap_or(0);
    assert!(
        div_const >= 3,
        "divopt must lift with 64-bit arithmetic and reconstruct clean `/ N` \
         division (e.g. `/ 81`); got none:\n{rendered}"
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

/// ADVERSARIAL: divopt's 64-bit reciprocal-multiply must be DECODED AND
/// RECONSTRUCTED into clean `/ const` division (the RuleDivOpt family — the
/// w10-divmod un-seam).  Compiler-emitted unsigned `/ const` lowers to a wide
/// `value * magic >> shift`; the magic constants (e.g. `0x948b0fcd6e9e0653`
/// for `/81`) are 64-bit and *cannot* be produced by a 16-bit real-mode lift.
/// Now that the reconstruction rules fire, the magic and the `* -1` wide-mask
/// correction are CONSUMED — the body must show `… / 81` (decimal divisor), and
/// the raw magic must NOT survive.  This is the strongest gate: it proves the
/// wide multiply was decoded AND collapsed back to division.
#[test]
fn verify_w10_r2_divopt_reciprocal_multiply_is_64bit() {
    let path = repo_root().join("decompiler/datatests/divopt.xml");
    let dt = parse_datatest(&path).expect("parse divopt.xml");
    let rendered = render_corpus(&dt).expect("divopt must decompile");

    // The reconstruction CONSUMES the reciprocal magic: it must NOT survive in
    // the body (its presence would mean RuleDivOpt declined — the pre-un-seam
    // garbage signature).
    let magic = count_matches(r"0x948b0fcd6e9e0653", &rendered).unwrap_or(0);
    assert_eq!(
        magic, 0,
        "divopt's reciprocal magic must be reconstructed away into `/ 81`, not \
         left as a raw `* 0x948b…` multiply:\n{rendered}"
    );
    // The wide `* -1` sign/shift correction is likewise consumed by the
    // reconstruction, so the 64-bit-wide negate mask must be gone too.
    let wide_mask = count_matches(r"0xffffffffffffffff", &rendered).unwrap_or(0);
    assert_eq!(
        wide_mask, 0,
        "divopt's `* -1` correction must collapse into the reconstructed \
         division (no surviving 0xffffffffffffffff mask):\n{rendered}"
    );
    // The reconstruction target: at least the first few divisors render as clean
    // decimal `/ N` (a 16-bit garbage lift could never produce these, and a
    // declined RuleDivOpt would show the magic instead).
    let div_const = count_matches(r"/ 81\b", &rendered).unwrap_or(0)
        + count_matches(r"/ 89\b", &rendered).unwrap_or(0)
        + count_matches(r"/ 91\b", &rendered).unwrap_or(0);
    assert!(
        div_const >= 3,
        "divopt must reconstruct the unsigned/signed divisions to clean decimal \
         `/ N` divisors (e.g. `/ 81`, `/ 89`, `/ 91`):\n{rendered}"
    );
    // And NO 16-bit real-mode signature.
    let narrow_mask = count_matches(r"\b0xffff\b", &rendered).unwrap_or(0);
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
    // first-arg pointer to its recovered parameter, and the pointer-flow un-seam
    // routes the STORE through the typed pointer into the oracle's array notation
    // (`divu[N] = divu[N] / k;`).  So the stores now render `a0[N] = <expr>;`
    // (pointer/array element on the LHS of `=`), the C++ oracle direction — NOT a
    // functional `STORE(spaceid, ptr, value)` and NOT through the raw register.
    let ptr_stores = count_matches(r"(?m)^\s*a0\[[0-9a-fx]+\]\s*=", &rendered).unwrap_or(0);
    assert!(
        ptr_stores >= 8,
        "divopt must STORE through the recovered pointer for the array elements in \
         straight-line array-notation form (oracle: `divu[N] = …;`); got \
         {ptr_stores}:\n{rendered}"
    );
    // No functional STORE form may survive (the un-seam absorbs it into `[]`), and
    // the store target must NOT be the raw first-arg register (the un-seam binds it
    // to the parameter).
    assert_eq!(
        count_matches(r"STORE\([0-9]+,", &rendered).unwrap_or(0),
        0,
        "divopt's array-element stores must be absorbed into `a0[N] = …` notation, \
         not left as a functional `STORE(spaceid, ptr, value)`:\n{rendered}"
    );
    assert_eq!(
        count_matches(r"\bRDI\b", &rendered).unwrap_or(0),
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
    // The recovered first parameter (`a0`) is the access base — proves the body
    // HighVariable carries the parameter Symbol, the un-seam's core effect.  With
    // the pointer-flow un-seam the LOAD through `a0` is no longer a functional
    // `LOAD(..,a0)`: it is absorbed into the C-faithful member/array/deref form
    // (`a0[..]` / `*(a0 + ..)` / `a0->field`), exactly as the C++ opLoad renders a
    // PTRADD/PTRSUB or plain pointer address.  Require the parameter base to drive
    // an access of one of those forms (the un-seam's effect is the binding of `a0`
    // as the access base, not the obsolete functional-LOAD spelling).
    let param_access = count_matches(r"\ba0(\[|->|\b\s*\+|\))", &rendered).unwrap_or(0);
    let param_in_star = count_matches(r"\*\([^)]*\ba0\b", &rendered).unwrap_or(0);
    assert!(
        param_access >= 1 || param_in_star >= 1,
        "readstruct must access through the bound first parameter (`a0`) in \
         member/array/deref form (`a0[..]`/`*(a0 + ..)`/`a0->..`):\n{rendered}"
    );
    // The functional `LOAD(..,a0)` spelling must be GONE — the un-seam routes the
    // load through the typed pointer into structured access.
    assert_eq!(
        count_matches(r"LOAD\([0-9]+,\s*a0\b", &rendered).unwrap_or(0),
        0,
        "the LOAD through `a0` must be absorbed into deref/array/member notation, \
         not a functional `LOAD(spaceid, a0)`:\n{rendered}"
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

// ===========================================================================
// w10-symbol-naming INDEPENDENT VERIFIER adversarial tests (Round 1)
//
// Hunt-list targets for this item: (1) the `vN` base counter increments in the
// `beginLoc..endLoc` location-order walk — a HashSet/non-deterministic `seen`
// would scramble the numbering or the binding; (2) the recovered parameters
// must bind to the angr default `aN` names and appear in the body, never the raw
// argument registers and never the `$$undef` placeholder; (3) parameters must
// NOT be re-declared as body locals.  These pin the un-seam against the C++
// oracle (`decomp_dbg` with `namestyle angr`, the kuna default) on the
// integer/pointer corpus, the same scheme as `kunaArgName`/`buildDefaultName`.
// ===========================================================================

/// DETERMINISM: rendering the same function twice must be byte-identical.  The
/// `seen` dedup is a `BTreeSet` and the walk is `iter_loc` (a BTreeMap), so the
/// `vN` numbering and the symbol binding are order-stable; a `HashSet`/`HashMap`
/// regression in the naming walk would surface here as a flaky diff.
#[test]
fn verify_w10_symbol_naming_render_is_deterministic() {
    let path = repo_root().join("decompiler/datatests/nestedoffset.xml");
    let dt = parse_datatest(&path).expect("parse nestedoffset.xml");
    let a = render_corpus(&dt).expect("readstruct must decompile (run 1)");
    let b = render_corpus(&dt).expect("readstruct must decompile (run 2)");
    assert_eq!(
        a, b,
        "the symbol-naming walk must be deterministic across runs (no HashSet/HashMap \
         in the naming path):\nrun1:\n{a}\nrun2:\n{b}"
    );
    // And the render must be the REAL body (not the W9-emit stub warning) — the
    // whole point of binding names is moot if the body is a stub.
    assert!(
        !a.contains("WARNING: body emission"),
        "the body must be real C (not the W9-emit stub) for naming to be observable:\n{a}"
    );
}

/// `vN` NUMBERING: the unnamed locals get sequential `v1`, `v2`, … in location
/// order (the `base++` counter starts at 1).  readstruct's body has two distinct
/// unnamed result locals — they must render as `v1` and `v2` (not `v0`, not a
/// gap, not duplicated), proving the counter and the per-high dedup both work.
#[test]
fn verify_w10_symbol_naming_local_vn_counter_is_sequential_from_v1() {
    let path = repo_root().join("decompiler/datatests/nestedoffset.xml");
    let dt = parse_datatest(&path).expect("parse nestedoffset.xml");
    let rendered = render_corpus(&dt).expect("readstruct must decompile");
    // The angr local default is `v<base>` with base starting at 1 (C++
    // `buildDefaultName`: `s << 'v' << dec << base++`, and apply() seeds base=1).
    // There must be NO `v0` (off-by-one would start the counter at 0).
    assert_eq!(
        count_matches(r"\bv0\b", &rendered).unwrap_or(0),
        0,
        "the angr local counter starts at v1, never v0 (C++ apply() seeds base=1):\n{rendered}"
    );
    // At least `v1` must be present (the function has unnamed result locals).
    assert!(
        count_matches(r"\bv1\b", &rendered).unwrap_or(0) >= 1,
        "the first unnamed local must be named v1:\n{rendered}"
    );
}

/// PARAM BINDING on a SECOND corpus (divopt.xml, a different x86-64 pointer
/// function): the recovered first-arg pointer must bind to its parameter name
/// and be used in the body; the raw argument registers and the `$$undef`
/// placeholder must both be absent.  This guards against the binding being a
/// nestedoffset-specific accident.
#[test]
fn verify_w10_symbol_naming_divopt_binds_param_no_raw_reg_no_undef() {
    let path = repo_root().join("decompiler/datatests/divopt.xml");
    let dt = parse_datatest(&path).expect("parse divopt.xml");
    let rendered = render_corpus(&dt).expect("divopt must decompile");
    // No `$$undef` placeholder may leak (an unnamed param Symbol left undefined
    // would render `$$undefXXXX`; the un-seam materializes a real `aN`/declared
    // name instead).
    assert_eq!(
        count_matches(r"\$\$undef", &rendered).unwrap_or(0),
        0,
        "no `$$undef` placeholder may leak into the body:\n{rendered}"
    );
    // The first-arg pointer (RDI) must be GONE from the body — it is bound to its
    // recovered parameter.  A residual `RDI` token means the high was not bound.
    assert_eq!(
        count_matches(r"\bRDI\b", &rendered).unwrap_or(0),
        0,
        "divopt's body must bind the RDI first-arg pointer to its parameter, not \
         render the raw register:\n{rendered}"
    );
}

/// NO PARAM RE-DECLARATION: a recovered parameter renders in the signature, never
/// in the body decl block (C++ `emitLocalVarDecls` -> `emitScopeVarDecls(scope,
/// no_category)` skips `function_parameter` symbols).  The body must not contain a
/// local declaration line `<type> a0;` for the first parameter.
#[test]
fn verify_w10_symbol_naming_param_not_redeclared_as_local() {
    let path = repo_root().join("decompiler/datatests/nestedoffset.xml");
    let dt = parse_datatest(&path).expect("parse nestedoffset.xml");
    let rendered = render_corpus(&dt).expect("readstruct must decompile");
    // A body-local declaration is an indented `<type ...> <name>;` line.  The
    // parameter `a0` must appear in the signature parentheses but NOT as such a
    // standalone declaration statement in the body.
    assert_eq!(
        count_matches(r"(?m)^\s+[A-Za-z_][\w ]*\ba0\s*;\s*$", &rendered).unwrap_or(0),
        0,
        "a recovered parameter (a0) must render in the signature, not be re-declared \
         as a body local:\n{rendered}"
    );
    // Sanity: the parameter name is actually present somewhere (the binding fired).
    assert!(
        rendered.contains("a0"),
        "the recovered first parameter (a0) must be bound and rendered:\n{rendered}"
    );
}

// ===========================================================================
// VERIFIER ADVERSARIAL TESTS (item: w10-pointer-flow)
//
// These pin the most fragile spot the hunt list flagged for this item: the
// PrintC LOAD/STORE/PTRADD/PTRSUB operand ORDER under the direct-recursion RPN
// engine.  The engine renders in PUSH order (first push = leftmost operand;
// see `op_binary_ir` / `emit_expression_ir`), which is the INVERSE of the C++
// LIFO `nodepend` push order.  A faithful `op_store_ir` must therefore push the
// POINTER first (LHS) then the VALUE (RHS) so a STORE renders `ptr = value` —
// the C++ `opStore` (printc.cc:520) pushes in2(value) then in1(pointer), and
// the LIFO reversal makes the pointer the LHS.
// ===========================================================================

/// F1 (REJECT trigger): a STORE absorbed into an array/member access must render
/// with the array/member access on the LEFT of `=` (the C++ oracle for divopt is
/// straight-line `p[N] = <reciprocal-multiply expr>`, scored 34/34 by
/// decomp_test_dbg).  The branch's `op_store_ir` pushes value-then-pointer in the
/// SAME order as the C++ source, but the direct-recursion engine renders in push
/// order, so the assignment comes out INVERTED (`<expr> = p[N]`).
#[test]
fn w10_ptr_flow_store_renders_pointer_on_lhs_of_assignment() {
    let path = repo_root().join("decompiler/datatests/divopt.xml");
    let dt = parse_datatest(&path).expect("parse divopt.xml");
    let rendered = render_corpus(&dt).expect("divopt must decompile");

    // The function recovers a pointer parameter; its STOREs must put the array
    // element (`a0[N]`) on the LHS.  Count well-formed `a0[..] = ` assignments vs
    // the inverted store form.  NOTE: a legitimate LOAD-into-temp renders
    // `<ident> = a0[N];` (a read of the element into a single-identifier lvalue),
    // so the bare `= a0[N];` shape is NOT itself the bug.  The op_store_ir
    // inversion put a *non-lvalue expression* on the LHS — its tell is a
    // multi-token expression containing a closing `)` (every reciprocal-multiply
    // expr opens `SUB(`/`ZEXT(`) before the ` = a0[N];`.  A single-identifier read
    // LHS has no `)`, so this distinguishes the bug from a legitimate read.
    let lhs_form = count_matches(r"(?m)^\s*a0\[[0-9a-fx]+\]\s*=", &rendered).unwrap_or(0);
    let inverted_form =
        count_matches(r"(?m)^[^=\n]*\)[^=\n]*=\s*a0\[[0-9a-fx]+\]\s*;", &rendered).unwrap_or(0);

    assert_eq!(
        inverted_form, 0,
        "STORE assignments must NOT render with a non-lvalue expression on the \
         LHS and the pointer/array access on the RIGHT of `=` \
         (`<expr> = a0[N];`); that is the inverted-operand bug in op_store_ir. \
         Found {inverted_form} inverted store(s):\n{rendered}"
    );
    assert!(
        lhs_form >= 1,
        "at least one STORE must render the array element on the LHS \
         (`a0[N] = <value>;`), matching the C++ oracle's straight-line stores:\n{rendered}"
    );
}

/// The member/array access on the LOAD side is REAL: a LOAD whose pointer is a
/// genuine PTRADD/PTRSUB (from real pointer-type propagation) renders as
/// `a0[index]` / `*(a0 + ..)`, NOT as a functional `LOAD(spaceid, ptr)`.  This
/// guards against a "fix" to the store bug that breaks the load rendering, and
/// confirms the access is produced by the real transform (PTRADD/PTRSUB created
/// by RulePtrArith), not fabricated by a print-time special case.
#[test]
fn w10_ptr_flow_load_renders_array_member_access_not_functional_load() {
    let path = repo_root().join("decompiler/datatests/divopt.xml");
    let dt = parse_datatest(&path).expect("parse divopt.xml");
    let rendered = render_corpus(&dt).expect("divopt must decompile");

    // Real array access through the recovered pointer parameter.
    let array_access = count_matches(r"a0\[[0-9a-fx]+\]", &rendered).unwrap_or(0);
    assert!(
        array_access >= 1,
        "a LOAD through the recovered pointer must render as a real array access \
         `a0[N]` (genuine PTRADD/PTRSUB from RulePtrArith), not a functional \
         `LOAD(..)`:\n{rendered}"
    );
    // The functional LOAD form must be gone for the absorbed (array-deref) reads.
    let functional_load_on_param = count_matches(r"LOAD\([0-9]+,a0\[", &rendered).unwrap_or(0);
    assert_eq!(
        functional_load_on_param, 0,
        "an array-deref LOAD must be absorbed into `[]` notation, not left as a \
         functional `LOAD(spaceid, a0[..])`:\n{rendered}"
    );
}

/// Independent corroboration on a SECOND corpus function (readstruct in
/// nestedoffset.xml): a LOAD through a non-PTRADD/PTRSUB pointer (plain INT_ADD
/// address) renders as an explicit `*(...)` dereference with the pointer base
/// (`a0`) inside — never as a functional `LOAD(..)` and never with the operands
/// transposed.  This pins the opLoad single-operand path (which the store bug
/// does NOT affect) so a store-only fix is shown to leave loads correct.
#[test]
fn w10_ptr_flow_load_explicit_deref_keeps_base_inside_star() {
    let path = repo_root().join("decompiler/datatests/nestedoffset.xml");
    let dt = parse_datatest(&path).expect("parse nestedoffset.xml");
    let rendered = render_corpus(&dt).expect("readstruct must decompile");

    // The body must dereference through the bound pointer parameter `a0`.  With
    // the W10 `ActionSetCasts` cast plane now active, a type-less LOAD pointer
    // (this `render_corpus` path does not apply the `parse line` struct types, so
    // the address arrives as a generic integer) faithfully receives an explicit
    // pointer cast — `*(uint4 *)(a0 + ..)` — exactly as the C++ `TypeOpLoad::getInputCast`
    // builds a `getTypePointer(reqtype)` for a non-pointer LOAD address
    // (typeop.cc:454).  Since w10-printc-cast-render the cast renders in C
    // cast-notation `(type *)` (printc `opTypeCast`), not the older functional
    // `CAST(..)`.  The intent of this fence is unchanged: `a0` is *dereferenced*
    // (a `*` deref or an `a0[..]` access), never left as a functional `LOAD(..)`.
    // The deref regex therefore tolerates an optional cast — either the C-notation
    // `(type *)` group or the legacy functional `CAST` — between `*` and the base;
    // the typed datatest (`nestedoffset` with `parse line`) renders the clean
    // `ptr->array[b + a]` and is checked by the engine parity run.
    let star_deref_with_base =
        count_matches(r"\*(CAST|\([^()]*\*\s*\))?\([^)]*\ba0\b", &rendered).unwrap_or(0);
    let plain_array = count_matches(r"\ba0\[", &rendered).unwrap_or(0);
    assert!(
        star_deref_with_base >= 1 || plain_array >= 1,
        "a LOAD through the recovered pointer must render the pointer base `a0` \
         inside a `*(...)` deref or an `a0[..]` access (real pointer flow), not as \
         a functional LOAD:\n{rendered}"
    );
    // No functional LOAD over the bound parameter should remain.
    assert_eq!(
        count_matches(r"LOAD\([0-9]+,\s*a0\b", &rendered).unwrap_or(0),
        0,
        "the LOAD through `a0` must be absorbed into deref/array notation, not a \
         functional `LOAD(spaceid, a0)`:\n{rendered}"
    );
}

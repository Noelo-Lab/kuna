//! END-TO-END GATE for item `w9x-arch-engine-glue`.
//!
//! Proves the god-object assembly produces C end-to-end.  For 5 corpus
//! datatests spanning architectures (8051 / AARCH64 / MIPS / PowerPC / ARM),
//! this gate:
//!
//!   1. extracts the `<binaryimage>` + `<symbol>` functions + `<script>`
//!      `option` commands from `tests/datatests/*.xml` (reusing the
//!      `corpus_bootstrap.rs` bootstrap mechanics),
//!   2. bootstraps a full [`Architecture`] (the XML frontend through
//!      `build_translator`, then [`Architecture::init_post_engine`] — the
//!      `buildTypegrp`/`buildCoreTypes`/`buildAction`/… tail of
//!      `Architecture::init`),
//!   3. applies any `<script>` `option NAME VALUE` commands against the **real**
//!      `Architecture` via `OptionDatabase::set` (proving item #2:
//!      `Architecture: ArchOptionContext`),
//!   4. decompiles each `<symbol>` function to C
//!      ([`decompile_drive::decompile_func`] -> [`decompile_drive::print_c`]),
//!      and
//!   5. asserts the C output is non-empty + structurally sane (a function
//!      signature + matched braces).
//!
//! This proves the full path RUNS and emits plausible C — NOT byte-parity (the
//! W10 grind); the printer body is the documented W9-emit seam (see
//! `printc.rs` / `decompile_drive.rs`), so the asserted C is a complete,
//! brace-matched function *shell*.
//!
//! ## `.sla` precondition
//!
//! Like `corpus_bootstrap.rs`, this gate needs the built `.sla` artifacts under
//! `specs/` (gitignored; `make specs`).  A fixture whose `.sla` is absent is
//! reported as `errored` (with its first error) rather than silently skipped,
//! so the gate's pass/fail count is honest.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::AddrSpaceManager;
use kuna_base::xml::{DocumentStorage, Element};

use kuna_decomp::architecture::Architecture;
use kuna_decomp::decompile_drive::{decompile_func, print_c};
use kuna_decomp::options::{register_option_elements, ArchOptionContext, OptionDatabase};
use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase};
use kuna_decomp::xml_arch::{XmlArchitecture, XmlArchitectureCapability};

use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::translate::register_translate_ids;

// ===========================================================================
// Fixture selection: 5 datatests spanning architectures.
// ===========================================================================

/// `(datatest stem, expected arch prefix)` — one per distinct architecture,
/// chosen as simple corpus functions (the same straight-line / small functions
/// the lift-fixture pin set uses).  >= 5 required by the gate.
const FIXTURES: &[&str] = &["boolless", "ccmp", "gp", "lzcount", "condexesub"];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

// ===========================================================================
// Datatest XML parsing.
// ===========================================================================

/// One `<symbol>` function from a `<binaryimage>`.
struct SymbolFn {
    name: String,
    space: String,
    offset: u64,
}

/// The parsed datatest: the `<binaryimage>` element, the arch id, the symbol
/// functions, and the `<script>` `option` commands (name + up to 3 params).
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

/// Parse a `<com>option NAME P1 P2 P3</com>` script line into its tokens.
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

    // <symbol space=".." offset=".." name=".."/> children of binaryimage.
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

    // <script><com>option ...</com>...</script> commands.
    let mut coms = Vec::new();
    find_named(&root, "com", &mut coms);
    let options: Vec<_> = coms
        .iter()
        .filter_map(|c| parse_option_com(&String::from_utf8_lossy(c.get_content())))
        .collect();

    Ok(DataTest { binaryimage, arch_id, symbols, options })
}

// ===========================================================================
// Bootstrap a full Architecture from the datatest (corpus_bootstrap mechanics).
// ===========================================================================

/// A dummy load image (errors on read) — replaced by the opened corpus image.
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

/// Bootstrap the XML frontend through `build_translator`, then run
/// `Architecture::init_post_engine` so the returned `XmlArchitecture` owns a
/// decompilation-ready `Architecture` (`arch.sleigh().base()`).
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

    // The tail of Architecture::init (buildTypegrp/buildCoreTypes/buildAction/…).
    arch.sleigh_mut()
        .base_mut()
        .ok_or("no Architecture base after build_translator")?
        .init_post_engine()
        .map_err(|e| format!("init_post_engine: {e}"))?;

    // Open the corpus image against the engine and hand it to the engine.
    let manager_ptr: *const AddrSpaceManager = arch.sleigh().base().unwrap().manage();
    // SAFETY: the manager lives inside `arch` and outlives the open() call; the
    // borrow is released before any &mut use of `arch` (same shape as
    // corpus_bootstrap.rs).
    arch.open_image(unsafe { &*manager_ptr }, &registry)
        .map_err(|e| format!("open_image: {e}"))?;
    let img = arch.take_loader().ok_or("loader vanished after open")?;
    arch.sleigh_mut().base_mut().unwrap().set_loader(Box::new(img));

    Ok(arch)
}

/// Apply the datatest's `<script>` `option` commands against the real
/// `Architecture` via `OptionDatabase::set` (item #2: `Architecture:
/// ArchOptionContext`).  Returns the number of options applied successfully.
fn apply_options(arch: &mut Architecture, dt: &DataTest, registry: &IdRegistry) -> usize {
    let options = OptionDatabase::new();
    let mut applied = 0;
    for (name, p1, p2, p3) in &dt.options {
        let id = registry.find_element(name, 0);
        if id == 0 {
            continue; // unknown option element name (not registered)
        }
        if options.set(arch, id, p1, p2, p3).is_ok() {
            applied += 1;
        }
    }
    applied
}

// ===========================================================================
// Structural C-sanity check.
// ===========================================================================

/// A C function shell is structurally sane iff it has a signature (the function
/// name followed by a parenthesized parameter list) and matched braces.
fn is_structurally_sane(c: &str, func_name: &str) -> bool {
    if c.trim().is_empty() {
        return false;
    }
    let has_sig = c.contains(func_name) && c.contains('(') && c.contains(')');
    let open = c.matches('{').count();
    let close = c.matches('}').count();
    has_sig && open >= 1 && open == close
}

/// The outcome of decompiling one symbol.
enum Outcome {
    /// Produced structurally-sane C (the rendered text).
    Ok(String),
    /// Errored at some stage (the first error message).
    Err(String),
}

/// Bootstrap + decompile every symbol of one datatest, returning per-symbol
/// outcomes.
fn run_fixture(stem: &str) -> Result<Vec<(String, Outcome)>, String> {
    let dt = parse_datatest(stem)?;
    let registry = build_registry();
    let mut xarch = bootstrap(&dt)?;

    // Apply the script's `option` commands against the real Architecture.
    {
        let base = xarch.sleigh_mut().base_mut().ok_or("no base for options")?;
        let _applied = apply_options(base, &dt, &registry);
    }

    let mut outcomes = Vec::new();
    for sym in &dt.symbols {
        let base = match xarch.sleigh_mut().base_mut() {
            Some(b) => b,
            None => {
                outcomes.push((sym.name.clone(), Outcome::Err("no Architecture base".into())));
                continue;
            }
        };
        // Resolve the symbol's entry address against the engine space.
        let space = match base.manage().get_space_by_name(&sym.space) {
            Some(s) => Rc::clone(s),
            None => {
                outcomes
                    .push((sym.name.clone(), Outcome::Err(format!("no space {}", sym.space))));
                continue;
            }
        };
        let entry = Address::new(space, sym.offset);
        match decompile_func(base, &sym.name, entry, 0) {
            Ok(fd) => {
                let c = print_c(base, &fd);
                outcomes.push((sym.name.clone(), Outcome::Ok(c)));
            }
            Err(e) => {
                outcomes.push((sym.name.clone(), Outcome::Err(format!("decompile: {e}"))));
            }
        }
    }
    Ok(outcomes)
}

// ===========================================================================
// The gate.
// ===========================================================================

#[test]
fn five_corpus_functions_decompile_to_structurally_sane_c() {
    let mut produced = 0usize;
    let mut errored = 0usize;
    let mut first_errors: Vec<String> = Vec::new();
    let mut demo: Option<String> = None;

    for stem in FIXTURES {
        match run_fixture(stem) {
            Ok(outcomes) => {
                for (name, outcome) in outcomes {
                    match outcome {
                        Outcome::Ok(c) => {
                            assert!(
                                is_structurally_sane(&c, &name),
                                "{stem}:{name} produced non-sane C:\n{c}"
                            );
                            produced += 1;
                            if demo.is_none() {
                                demo = Some(format!("// {stem}:{name}\n{c}"));
                            }
                        }
                        Outcome::Err(e) => {
                            errored += 1;
                            first_errors.push(format!("{stem}:{name}: {e}"));
                        }
                    }
                }
            }
            Err(e) => {
                errored += 1;
                first_errors.push(format!("{stem} (bootstrap): {e}"));
            }
        }
    }

    // Print the report (cargo shows it on failure; `--nocapture` on success).
    eprintln!("=== decompile_e2e report ===");
    eprintln!("produced compilable-looking C: {produced}");
    eprintln!("errored: {errored}");
    for e in &first_errors {
        eprintln!("  first-error: {e}");
    }
    if let Some(d) = &demo {
        eprintln!("--- demo C function ---\n{d}\n--- end demo ---");
    }

    // The gate's teeth: at least 5 corpus functions must produce structurally
    // sane C through the full bootstrap -> decompile -> print path.
    assert!(
        produced >= 5,
        "expected >= 5 corpus functions to produce structurally-sane C, got {produced} \
         (errors: {first_errors:?})"
    );
}

/// Item #2 proof: `option NAME VALUE` (the most-used datatest command) mutates
/// the **real** `Architecture` / printer / proto-model registry through
/// `Architecture: ArchOptionContext`.  Drives `OptionDatabase::set` against a
/// bootstrapped `Architecture` exactly as the console `IfcOption` would, and
/// asserts the owned subsystem actually changed.
#[test]
fn option_command_mutates_the_real_architecture() {
    // gp is a MIPS datatest with `option readonly on` in its script.
    let dt = parse_datatest("gp").expect("parse gp");
    let registry = build_registry();
    let mut xarch = bootstrap(&dt).expect("bootstrap gp");
    let arch = xarch.sleigh_mut().base_mut().expect("Architecture base");

    let options = OptionDatabase::new();
    let set = |arch: &mut Architecture, name: &str, p1: &str| -> KunaResult<String> {
        let id = registry.find_element(name, 0);
        assert!(id != 0, "option element `{name}` is not registered");
        options.set(arch, id, p1, "", "")
    };

    // (1) a plain config field: `option readonly on` -> readonlypropagate=true.
    assert!(!arch.readonlypropagate, "precondition: readonly defaults off");
    set(arch, "readonly", "on").expect("option readonly on");
    assert!(arch.readonlypropagate, "option readonly on did not flip readonlypropagate");
    set(arch, "readonly", "off").expect("option readonly off");
    assert!(!arch.readonlypropagate, "option readonly off did not clear readonlypropagate");

    // (2) the printer: `option nullprinting on` -> PrintC option flips.
    assert!(!arch.print().options.null, "precondition: null printing off");
    set(arch, "nullprinting", "on").expect("option nullprinting on");
    assert!(arch.print().options.null, "option nullprinting on did not flip the printer");

    // (3) the proto-model registry: `option extrapop 8` -> defaultfp extrapop.
    //     The seeded default model exists, so the extrapop write resolves.
    set(arch, "extrapop", "8").expect("option extrapop 8");
    assert_eq!(
        arch.default_fp().expect("a default proto model is seeded").get_extra_pop(),
        8,
        "option extrapop did not reach the default prototype model"
    );

    // (4) an unknown proto model surfaces the faithful error (not a panic).
    let err = set(arch, "defaultprototype", "nonexistent-model").unwrap_err();
    assert!(
        format!("{err}").contains("Unknown prototype model"),
        "unknown model gave the wrong error: {err}"
    );

    // (5) the action database: `option setaction decompile` sets the current
    //     root (proving `ArchOptionContext::set_current_action` reaches the real
    //     `allacts`, which `init_post_engine` populated with the universal tree).
    set(arch, "setaction", "decompile").expect("option setaction decompile");
    assert_eq!(arch.current_action_name(), "decompile");
}

/// The `maxinstruction` option is LIVE: flow-following honors
/// `glb->max_instructions` / `glb->flowoptions` exactly as C++
/// `Funcdata::followFlow` wires them into the `FlowInfo`
/// (`decompiler/cpp/funcdata_op.cc:765`: `flow.setFlags(glb->flowoptions);
/// flow.setMaximumInstructions(glb->max_instructions);`).
///
/// Drives the bound end-to-end through `OptionDatabase::set` +
/// `decompile_func` (the same path the console `option maxinstruction N`
/// takes), exercising both halves of the `FlowInfo::processInstruction`
/// too-many-instructions policy (`flow.cc:409`):
///
///   * with the default `error_toomanyinstructions` flow option, exceeding the
///     bound is the faithful fatal error `"Flow exceeded maximum allowable
///     instructions"`;
///   * with `option errortoomanyinstructions off`, the same bound TRUNCATES
///     flow (artificial halt) and the function still decompiles.
#[test]
fn maxinstruction_option_bounds_flow_following() {
    use kuna_decomp::flow::flow_flags;

    // gp is a MIPS datatest whose function body is far more than 2 instructions.
    let dt = parse_datatest("gp").expect("parse gp");
    let registry = build_registry();
    let mut xarch = bootstrap(&dt).expect("bootstrap gp");
    let arch = xarch.sleigh_mut().base_mut().expect("Architecture base");

    // The constructor defaults (C++ `resetDefaultsInternal`, architecture.cc:1420).
    assert_eq!(arch.flowoptions, flow_flags::error_toomanyinstructions);
    assert_eq!(arch.max_instructions, 100000);

    // test_gp is the datatest's real (byte-backed) function; the other symbols
    // (printf/populate) are call targets without loaded bytes.
    let sym = dt
        .symbols
        .iter()
        .find(|s| s.name == "test_gp")
        .expect("gp has the test_gp <symbol>");
    let space =
        Rc::clone(arch.manage().get_space_by_name(&sym.space).expect("symbol space exists"));
    let entry = Address::new(space, sym.offset);

    let options = OptionDatabase::new();
    let set = |arch: &mut Architecture, name: &str, p1: &str| -> KunaResult<String> {
        let id = registry.find_element(name, 0);
        assert!(id != 0, "option element `{name}` is not registered");
        options.set(arch, id, p1, "", "")
    };

    // (1) `option maxinstruction 2` + the default error_toomanyinstructions
    //     flow option: following flow past the bound must abort with the
    //     faithful C++ error (flow.cc:411).
    set(arch, "maxinstruction", "2").expect("option maxinstruction 2");
    let err = match decompile_func(arch, &sym.name, entry.clone(), 0) {
        Ok(_) => panic!("decompile under `option maxinstruction 2` must fail"),
        Err(e) => e,
    };
    assert!(
        format!("{err}").contains("Flow exceeded maximum allowable instructions"),
        "wrong error under the instruction bound: {err}"
    );

    // (2) `option errortoomanyinstructions off`: the same tiny bound now
    //     truncates flow with an artificial halt (flow.cc:414) instead of
    //     erroring, and the truncated function still prints sane C.
    set(arch, "errortoomanyinstructions", "off")
        .expect("option errortoomanyinstructions off");
    let fd = decompile_func(arch, &sym.name, entry.clone(), 0)
        .expect("truncated flow should still decompile");
    let c = print_c(arch, &fd);
    assert!(is_structurally_sane(&c, &sym.name), "truncated-flow C is not sane:\n{c}");

    // (3) Restore the error flag and the default bound: the full function
    //     decompiles again, proving the failure in (1) was the bound itself.
    set(arch, "errortoomanyinstructions", "on")
        .expect("option errortoomanyinstructions on");
    set(arch, "maxinstruction", "100000").expect("option maxinstruction 100000");
    let fd = decompile_func(arch, &sym.name, entry, 0)
        .expect("decompile under the default bound");
    let c = print_c(arch, &fd);
    assert!(is_structurally_sane(&c, &sym.name), "default-bound C is not sane:\n{c}");
}

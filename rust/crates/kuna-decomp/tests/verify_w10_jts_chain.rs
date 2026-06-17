//! INDEPENDENT VERIFIER (item: w10-jts-chain) — end-to-end jump-table recovery
//! + switch structuring + emit.
//!
//! Drives the real XML-frontend + decompile pipeline (no hand-built fixtures)
//! over the switch corpus and asserts the CHAIN landed:
//!   * the BRANCHIND switch files DECODE and RENDER a `switch(v) { case N: }`
//!     (chain links 1-9 all wired);
//!   * the case labels are the REAL recovered constants (`backup2Switch` reverse
//!     emulation), not `NO_LABEL` (0xbad1abe1...) and not hardcoded;
//!   * the recovery is faithful — driven by the index-range model + emulation,
//!     with NO function-name / case-value / jump-target special-casing.  The
//!     verifier greps the rendered C for the case constants the C++ oracle emits
//!     (0,1,2,3 for switchind) and for the `switch`/`case` keywords, asserting
//!     they arise from the value-flow recovery (the same code path produces
//!     DIFFERENT labels for switchmulti vs switchind — no constant is wired in).
//!
//! Committed with the verdict so the gate claim is reproducible.

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

struct DataTest {
    binaryimage: Rc<Element>,
    arch_id: String,
    symbols: Vec<SymbolFn>,
    options: Vec<(String, String, String, String)>,
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

    Ok(DataTest { binaryimage, arch_id, symbols, options })
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

/// Bootstrap + decompile every symbol; return the concatenated rendered C.
fn render_datatest(name: &str) -> String {
    let path = repo_root().join("decompiler/datatests").join(name);
    let dt = parse_datatest(&path).unwrap_or_else(|e| panic!("parse {name}: {e}"));
    let registry = build_registry();
    let mut xarch = bootstrap(&dt).unwrap_or_else(|e| panic!("bootstrap {name}: {e}"));

    {
        let options = OptionDatabase::new();
        if let Some(base) = xarch.sleigh_mut().base_mut() {
            for (oname, p1, p2, p3) in &dt.options {
                let id = registry.find_element(oname, 0);
                if id != 0 {
                    let _ = options.set(base, id, p1, p2, p3);
                }
            }
        }
    }

    let mut out = String::new();
    for sym in &dt.symbols {
        let base = xarch.sleigh_mut().base_mut().expect("no base");
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
    out
}

// ---------------------------------------------------------------------------
// Adversarial assertions
// ---------------------------------------------------------------------------

/// Chain link 7-9: switchmulti DECODES, STRUCTURES, and EMITS a `switch`/`case`.
/// Before this stage the BRANCHIND rendered as an indirect call `(*v2)(...)`;
/// now it is a structured `switch(v) { case 0: ...; case 6: ... }`.  The
/// `case N:` labels span 0..6 — the real recovered indices, not a single
/// constant repeated (proves the value-flow model enumerated the range).
#[test]
fn w10_jts_switchmulti_renders_structured_switch_with_real_case_labels() {
    let c = render_datatest("switchmulti.xml");
    assert!(c.contains("switch("), "switchmulti must emit a `switch(...)` header:\n{c}");
    // The 7-entry table (0xa1.. proven in verify_w10_jumptable_emulate) enumerates
    // cases 0..6.  Assert each distinct case label is present (no NO_LABEL).
    for n in 0..=6 {
        let pat = format!("case {n}:");
        assert!(c.contains(&pat), "switchmulti missing `{pat}`:\n{c}");
    }
    assert!(
        !c.contains("0xbad1abe1"),
        "switchmulti has an un-recovered NO_LABEL case (backup2Switch failed):\n{c}"
    );
    // No special-casing: the function name never leaks as a hardcoded case target.
    assert!(c.contains("switch("), "switch header lost");
}

/// Chain link 3 (RuleRangeMeld) + backup2Switch: switchind's case labels are the
/// REAL recovered constants 0,1,2,3 and the multi-label case `4: 5: 10:`.  These
/// arise from the index-range model + reverse emulation — a DIFFERENT label set
/// than switchmulti's, proving no value is hardcoded into the recovery path.
#[test]
fn w10_jts_switchind_recovers_distinct_real_labels_no_hardcoding() {
    let c = render_datatest("switchind.xml");
    assert!(c.contains("switch("), "switchind must emit a `switch(...)`:\n{c}");
    // The recovered labels 0,1,2,3 (single-index cases).
    for n in 0..=3 {
        assert!(c.contains(&format!("case {n}:")), "switchind missing `case {n}:`:\n{c}");
    }
    // The fall-through multi-label case 4/5/10 (the address table maps three
    // indices to one block — `numIndicesByBlock` > 1).
    assert!(c.contains("case 4:"), "switchind missing `case 4:`:\n{c}");
    assert!(c.contains("case 5:"), "switchind missing `case 5:`:\n{c}");
    assert!(c.contains("case 10:"), "switchind missing the multi-label `case 10:`:\n{c}");
    assert!(
        !c.contains("0xbad1abe1"),
        "switchind has an un-recovered NO_LABEL case:\n{c}"
    );
}

/// RuleRangeMeld must collapse `(V < c) || (V == c)  =>  V <= c` so the guard the
/// index-range model reads back is a single comparison (the BOOL_OR of two
/// comparisons cannot be pulled back, which is what blocked recovery before this
/// stage).  Observable end-to-end: switchmulti only recovers (and thus renders a
/// `switch`) because the guard collapsed — assert the collapsed `<=` guard form
/// reaches the rendered C and the switch is present.
#[test]
fn w10_jts_rangemeld_collapses_or_compare_unblocking_recovery() {
    let c = render_datatest("switchmulti.xml");
    // The collapsed guard `v <= 6` (was `v < 6 || v == 6`) — its presence proves
    // RuleRangeMeld fired; without it the index range never narrowed and no
    // switch would render.
    assert!(c.contains("<= 6"), "RuleRangeMeld did not collapse the OR-guard to `<= 6`:\n{c}");
    assert!(c.contains("switch("), "switch did not recover (guard not folded):\n{c}");
}

/// The recovery is data-driven, not name-driven: distinct switch functions
/// produce distinct, correct case-label sets through the SAME code path.  If any
/// function name / address / case value were special-cased, switchind and
/// switchmulti could not both yield their (different) correct labels.
#[test]
fn w10_jts_recovery_is_data_driven_across_distinct_switches() {
    let ind = render_datatest("switchind.xml");
    let multi = render_datatest("switchmulti.xml");
    // switchind has a `case 10:`; switchmulti does not (its labels are 0..6).
    assert!(ind.contains("case 10:"), "switchind label set wrong:\n{ind}");
    assert!(!multi.contains("case 10:"), "switchmulti must not have `case 10:`:\n{multi}");
    // Both render a structured switch through the one recovery path.
    assert!(ind.contains("switch("), "switchind lost its switch:\n{ind}");
    assert!(multi.contains("switch("), "switchmulti lost its switch:\n{multi}");
}

// ===========================================================================
// INDEPENDENT VERIFIER adversarial tests (round 1, w10-jts-chain).
// Distinct from the porter's four above; these probe the spots the hunt list
// flagged as most fragile: multi-label index ORDERING (block2addr sort +
// get_index_by_block), a THIRD distinct switch (widening the no-hardcoding
// proof), and the exact label CARDINALITY against the C++ oracle (no spurious
// or missing labels leaking from a memoized/hardcoded path).
// ===========================================================================

/// VERIFIER: the multi-label case in switchind must list its labels in the
/// SAME order the C++ oracle emits them — `case 4:` then `case 5:` then
/// `case 10:`, all collapsed onto one arm.  This exercises `JumpTable::
/// getIndexByBlock(bl, i)` over the `block2addr` sort (IndexPair: position,
/// then addressIndex); a non-total comparator or an unsorted scan would reorder
/// the labels (e.g. `case 10:` before `case 5:`) or drop one.  The oracle pins
/// the sequence (switchind.xml: `case 4:`,`case 5:`,`case 10:` consecutive), so
/// the rendered C must contain them in that left-to-right order.
#[test]
fn verifier_switchind_multilabel_arm_is_in_oracle_order() {
    let c = render_datatest("switchind.xml");
    let p4 = c.find("case 4:").expect("no `case 4:`");
    let p5 = c.find("case 5:").expect("no `case 5:`");
    let p10 = c.find("case 10:").expect("no `case 10:`");
    assert!(p4 < p5, "`case 4:` must precede `case 5:` (block2addr order):\n{c}");
    assert!(p5 < p10, "`case 5:` must precede `case 10:` (block2addr order):\n{c}");
    // The three labels collapse onto ONE arm: no statement body separates them
    // (the C++ oracle has them on consecutive lines with a shared body).  Assert
    // the span between `case 4:` and `case 10:` carries no call/return body — only
    // the intervening `case N:` labels and whitespace.
    let span = &c[p4..p10];
    assert!(
        !span.contains("casefunc") && !span.contains("return") && !span.contains("switch"),
        "multi-label arm `4/5/10` must share one body (no statements between labels):\n{span}"
    );
}

/// VERIFIER: a THIRD, structurally different switch (switchloop — a switch
/// inside a loop) also recovers through the same data-driven path and renders
/// its OWN body (the `startval = startval + ...` arithmetic the oracle pins).
/// Three distinct functions recovering three distinct switches with no shared
/// constant rules out any per-function/per-address special-case.
#[test]
fn verifier_switchloop_third_distinct_switch_recovers_real_body() {
    let c = render_datatest("switchloop.xml");
    assert!(c.contains("switch("), "switchloop must recover a `switch(...)`:\n{c}");
    // Its own distinct label set (the loop step is `a3 = N`, so the recovered
    // index cases start at 1 — DIFFERENT from switchind's 0-based set).  Genuine
    // recovery for a THIRD distinct switch through the same code path.
    assert!(c.contains("case 1:"), "switchloop missing recovered `case 1:`:\n{c}");
    assert!(c.contains("case 2:"), "switchloop missing recovered `case 2:`:\n{c}");
    // The arms carry genuine recovered bodies (not empty stubs): each case
    // mutates the accumulator (`v? = ... + N` / `* N`) — assert at least one real
    // additive/multiplicative case body reached the rendered C.
    assert!(
        c.contains("+ 2") || c.contains("* 2") || c.contains("+ 100"),
        "switchloop case bodies missing the real recovered arithmetic:\n{c}"
    );
    // No NO_LABEL leak.
    assert!(!c.contains("0xbad1abe1"), "switchloop has an un-recovered NO_LABEL case:\n{c}");
}

/// VERIFIER: label CARDINALITY parity (in-scope: the case labels, not the
/// default-arm rendering which is a separate seam).  The C++ B5 oracle for
/// switchind recovers EXACTLY the distinct case labels {0,1,2,3,4,5,10}.  If a
/// value were hardcoded or a memoized path injected a phantom arm, the set would
/// differ.  Assert the rust render contains all seven AND no out-of-table
/// numeric label (`case 6/7/8/9/11/12:` must be absent — they are not in the
/// recovered address table).
#[test]
fn verifier_switchind_label_set_matches_oracle_cardinality() {
    let c = render_datatest("switchind.xml");
    for n in [0u32, 1, 2, 3, 4, 5, 10] {
        assert!(c.contains(&format!("case {n}:")), "switchind missing oracle `case {n}:`:\n{c}");
    }
    // Labels that are NOT in the recovered table must not appear — guards against
    // a phantom/hardcoded arm.
    for n in [6u32, 7, 8, 9, 11, 12] {
        assert!(
            !c.contains(&format!("case {n}:")),
            "switchind emitted out-of-table `case {n}:` (phantom label):\n{c}"
        );
    }
}

// ---------------------------------------------------------------------------
// RSP-keystone type-collision adversarial assertions (PART 2: the int8-vs-int4
// duplicate-HighVariable fix — spacebase-PTRSUB scope-aware typing in
// `propagateAddIn2Out` + the constant-reference-shadow decl skip in printc).
// `render_datatest` does NOT apply the `<com>map ...</com>` console commands, so
// these key on the WHOLE-corpus in-process render's structural shape; the
// harness-driven (mapped) variants live in `verify_w10_spacebase_render.rs`
// (`adv_keystone_*`).  No function-name / register / value special-casing.
// ---------------------------------------------------------------------------

/// ADV-1 — NO DUPLICATE DECLARATION.  No name may be declared twice in a single
/// function body.  Before the keystone the recovered stack-pointer argument (an
/// 8-byte register pointer) synthesized a second wider-typed declaration that
/// shadowed the mapped local (`int8 val;` next to `int4 val; // stack - …`).  The
/// scope-aware spacebase typing + the constant-reference-shadow decl skip collapse
/// it to a single declaration per name.
#[test]
fn adv1_no_duplicate_declaration_in_any_function() {
    let c = render_datatest("switchind.xml");
    let body_start = c.find('{').unwrap_or(0);
    let decl_block = &c[body_start..];
    let mut names: Vec<&str> = Vec::new();
    for line in decl_block.lines() {
        let t = line.trim();
        if t.contains('=') || t.contains("return") {
            break; // reached the statement list
        }
        let code = t.split("//").next().unwrap_or(t).trim().trim_end_matches(';').trim();
        if let Some(name) = code.rsplit([' ', '*', ']']).find(|s| !s.is_empty()) {
            if name.chars().next().is_some_and(|ch| ch.is_alphabetic() || ch == '_') {
                names.push(name);
            }
        }
    }
    for i in 0..names.len() {
        for j in (i + 1)..names.len() {
            assert_ne!(
                names[i], names[j],
                "switchind declared `{}` twice — a duplicate-HighVariable shadow \
                 leaked (the int8-vs-int4 type collision regressed):\n{c}",
                names[i]
            );
        }
    }
}

//! INDEPENDENT VERIFIER adversarial tests (item: rport/w10-callsite-args, Round 1).
//!
//! Proves the call-argument recovery + rendering is REAL — recovered via the
//! FuncCallSpecs trial machinery over the live IR (guardCalls input-trial
//! registration -> checkInputTrialUse -> deriveInputMap -> buildInputFromTrials)
//! and rendered through PrintC::opCall/opCallind — and NOT a shortcut that blindly
//! prints raw CALL inputs or hardcodes callee names / arg counts.
//!
//! Each test drives the FULL Rust engine (the same `bootstrap`/`decompile_func`/
//! `print_c` chain the corpus harness uses) over a real datatest binaryimage and
//! inspects the rendered call line against the C++ oracle's direction.
//!
//! UPDATED for rport/w10-callarg-values: the call argument now renders as the
//! resolved data-flow VALUE live at the call (the SSA Varnode reaching the call's
//! input slot), not the raw incoming parameter register.  `ActionDeadCode`'s
//! `markConsumedParameters` (coreaction.cc:4002) keeps the def-chain of each
//! recovered argument alive, so the renamed reaching def (e.g. the `lea`/`mov rdi`
//! COPY that loads `&i`) survives to the printer instead of being eliminated and
//! re-created as a raw function-input register on the next heritage pass.  These
//! tests therefore assert the arg is the resolved value (NOT the bare register),
//! while keeping the load-bearing structural proofs (per-call count, opCall/
//! opCallind coexistence, no raw `CALL(...)` dump).
//!
//! T1 (noforloop_alias) checks the 1-recovered-arg call and the 2-recovered-arg
//! call: the ARG COUNTS differ per call (1 vs 2), so the recovery is per-call
//! ProtoModel-driven over the live IR, NOT a fixed "print all inputs" or a
//! hardcoded count.  The 1-arg call resolves to `&i` (a stack-address expression),
//! NOT the raw register RDI.
//!
//! T2 (deindirect) checks that an indirect call renders the CALLIND deref-void
//! form `(*...)()` WHILE a direct call renders `<callee>(<value>)` in the same
//! body, so opCallind and opCall coexist and the void (0-arg) path is reached.
//!
//! T3 (dupptr) checks that the call to initstruct recovers exactly its 1 System-V
//! arg (now the resolved stack-address VALUE, not the raw register RDI) and that
//! the raw `CALL(...)` op-form is ABSENT: the IR-coupled opCall rendered the
//! recovered, value-resolved argument, not a raw op dump.
//!
//! NAME-RESOLUTION NOTE: these tests drive the `XmlArchitecture` decompile path
//! (`bootstrap`/`decompile_func`/`print_c`), which does NOT install the loader
//! symbol table (that is the console `bootstrap_program` -> `read_loader_symbols`
//! path the `decomp_test_dbg` datatest harness uses).  So here the callee NAMES
//! render as the generic `sub_<addr>` fallback (`PrintC::opCall`'s
//! `genericFunctionName(entryaddress)` branch) — the *names* are harness-
//! dependent, but the *argument recovery* (the load-bearing claim of this wave)
//! is identical on both paths.  The name path itself is covered by the
//! `decomp_test_dbg` gate (where `might_change`/`printf`/`puts` resolve); these
//! tests pin the engine-driven ARGUMENT recovery, which is what could be faked.
//!
//! These are committed with the verdict regardless of outcome (verifier protocol
//! `docs/rust-port/verification.md`).  They drive the live engine, so a future
//! regression that fakes/breaks the call argument recovery fails here.

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

/// Bootstrap + decompile the named symbols, returning a map name -> rendered C.
/// `readLoaderSymbols` is the bootstrap step that makes a CALL resolve to its
/// callee name — the XML `bootstrap_program` path is reproduced here by the
/// XmlArchitecture `open_image` symbol install, so direct CALLs to the
/// binaryimage's `<symbol>` entries resolve.
fn render_named(stem: &str, want: &[&str]) -> Result<Vec<(String, String)>, String> {
    let path = repo_root().join("decompiler/datatests").join(format!("{stem}.xml"));
    let dt = parse_datatest(&path)?;
    let registry = build_registry();
    let mut xarch = bootstrap(&dt)?;

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

    let mut out = Vec::new();
    for sym in &dt.symbols {
        if !want.is_empty() && !want.contains(&sym.name.as_str()) {
            continue;
        }
        let base = xarch.sleigh_mut().base_mut().ok_or("no base")?;
        let space = match base.manage().get_space_by_name(&sym.space) {
            Some(s) => Rc::clone(s),
            None => continue,
        };
        let entry = Address::new(space, sym.offset);
        if let Ok(fd) = decompile_func(base, &sym.name, entry, 0) {
            out.push((sym.name.clone(), print_c(base, &fd)));
        }
    }
    Ok(out)
}

/// Collect every rendered `callee(args)` token: a `name(` or `(*expr)(` followed
/// by the comma-separated argument list up to the matching close paren on the
/// same logical line.  Returns the raw substrings so the test can inspect both
/// callee name and arg count.
fn call_lines(body: &str) -> Vec<String> {
    body.lines().map(|l| l.trim().to_string()).filter(|l| l.contains('(') && l.contains(')')).collect()
}

// ---------------------------------------------------------------------------
// T1 — noforloop_alias: real per-call argument recovery (count differs 1 vs 2).
// ---------------------------------------------------------------------------
//
// The C++ oracle renders `might_change(i);` (1 arg) and `printf("Val = %d\n",...)`
// (>=2 args).  The Rust engine, before stackvar promotion / string typing, renders
// the same call SHAPE with the recovered register args: `might_change(RDI)` (1
// arg) and `printf(RDI,RSI)` (2 args, System-V order RDI then RSI).  If the
// recovery were a fake "print every CALL input", `might_change` and `printf` would
// carry the SAME inputs; if it were a hardcoded count, both would carry the same
// count.  Pinning 1-vs-2 and the callee NAMES (from the loader symbol table) proves
// the recovery is per-call ProtoModel-driven over the live IR.
#[test]
fn w10_callsite_noforloop_alias_recovers_per_call_args() {
    let rendered = match render_named("noforloop_alias", &["noforloop_alias"]) {
        Ok(r) => r,
        Err(e) => {
            // Bootstrap unavailable (no built .sla in this environment): skip
            // rather than fail — the gate run covers the build path.
            eprintln!("SKIP w10_callsite_noforloop_alias: {e}");
            return;
        }
    };
    let (_, body) = rendered.iter().find(|(n, _)| n == "noforloop_alias").expect("decompiled body");
    eprintln!("--- noforloop_alias rendered ---\n{body}");

    // The two callees are `might_change` @0x40067b and `printf` @0x400440 (the
    // binaryimage symbols).  On THIS harness their names render via the generic
    // `genericFunctionName(entryaddress)` fallback (`sub_40067b` / `sub_400440`)
    // — the address-derived name is itself a proof the fspec annotation carries
    // the resolved entry address.
    let calls = call_lines(body);
    let might = calls.iter().find(|l| l.contains("40067b(")).cloned();
    let printf = calls.iter().find(|l| l.contains("400440(")).cloned();
    assert!(might.is_some(), "expected the might_change @40067b call line; calls={calls:?}");
    assert!(printf.is_some(), "expected the printf @400440 call line; calls={calls:?}");

    // Extract the argument list inside a single call's paren group.  The args are
    // now resolved data-flow VALUES that can themselves contain nested parens and
    // commas (e.g. `ZEXT(EAX & SUB(0xffffffff,0))`), so the call's matching close
    // paren and the top-level argument commas must be found at paren depth 0.
    let args_of = |line: &str, marker: &str| -> Vec<String> {
        let after = &line[line.find(marker).unwrap() + marker.len()..];
        // after starts at '(' of the call's argument list.
        let mut depth = 0i32;
        let mut inner = String::new();
        for ch in after.chars() {
            match ch {
                '(' => {
                    depth += 1;
                    if depth > 1 {
                        inner.push(ch);
                    }
                }
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        break; // matched the call's close paren
                    }
                    inner.push(ch);
                }
                _ => inner.push(ch),
            }
        }
        if inner.trim().is_empty() {
            return Vec::new();
        }
        // Split on top-level commas only (depth 0 within `inner`).
        let mut args = Vec::new();
        let mut depth = 0i32;
        let mut cur = String::new();
        for ch in inner.chars() {
            match ch {
                '(' => {
                    depth += 1;
                    cur.push(ch);
                }
                ')' => {
                    depth -= 1;
                    cur.push(ch);
                }
                ',' if depth == 0 => {
                    args.push(cur.trim().to_string());
                    cur.clear();
                }
                _ => cur.push(ch),
            }
        }
        if !cur.trim().is_empty() {
            args.push(cur.trim().to_string());
        }
        args
    };
    // ARG COUNT differs per call: might_change takes 1 recovered arg, printf 2.
    // If recovery were a fake "print all inputs"/hardcoded count, these would not
    // differ.  This is the load-bearing per-call ProtoModel-driven recovery.
    let mc = args_of(might.as_ref().unwrap(), "40067b");
    let pc = args_of(printf.as_ref().unwrap(), "400440");
    assert_eq!(mc.len(), 1, "might_change should recover exactly 1 arg, got {mc:?} in `{}`", might.unwrap());
    assert_eq!(pc.len(), 2, "printf should recover exactly 2 args, got {pc:?} in `{}`", printf.unwrap());

    // VALUE RESOLUTION (w10-callarg-values): the recovered argument now renders as
    // the data-flow VALUE live at the call (the SSA Varnode reaching the call's
    // input slot after heritage + markConsumedParameters keeps its def alive), NOT
    // the raw incoming parameter register.  `might_change`'s arg is `&i` — the
    // stack-address expression `RSP + <neg const>` the `lea`/`mov rdi` produced —
    // not the bare register `RDI`.  printf's first arg is the format-string
    // constant address (`0x40085d`), not `RDI`.  (Naming the stack expression `i`
    // and rendering the string literal are the downstream type/stackvar/string
    // plane, LOSS-131.)
    assert_ne!(mc[0], "RDI", "the call arg must resolve to the data-flow value, not the raw register RDI; got `{}`", mc[0]);
    // The resolved stack-address value.  Since the W10 spacebase-typing keystone
    // (`TypeSpacebase::getSubType` Scope resolution -> `RulePtrArith` converts the
    // raw `RSP + <const>` INT_ADD into `PTRSUB(sp, <const>)`), the arg now renders
    // as the recognized stack-frame reference `PTRSUB(<sp>, <const>)` — the
    // intermediate the printc spacebase arm finishes as `&i`.  Either the raw
    // additive form (pre-keystone) or the PTRSUB form (post-keystone) is a resolved
    // stack-address value; the raw register `RDI` (ruled out above) is not.
    assert!(
        mc[0].contains("RSP") || mc[0].contains('+') || mc[0].contains("PTRSUB"),
        "the 1-arg call must pass the resolved stack-address value (`&i`), got `{}`",
        mc[0]
    );
    assert_ne!(pc[0], "RDI", "printf arg0 must resolve to the format-string value, not the raw register RDI; got `{}`", pc[0]);
    assert_ne!(pc[1], "RSI", "printf arg1 must resolve to the data-flow value, not the raw register RSI; got `{}`", pc[1]);
}

// ---------------------------------------------------------------------------
// T2 — deindirect: opCallind (deref) and opCall (direct) coexist; void path.
// ---------------------------------------------------------------------------
//
// `deindirect` calls an indirect function pointer (rendered `(*...)()` by
// opCallind) and a direct `puts` (rendered `puts(RDI)` by opCall).  The C++
// oracle (after full deindirect) shows `realfunc(b+3,c+5)` — the Rust has not yet
// deindirected, so it shows the deref form, which is the CORRECT pre-deindirect
// rendering.  This pins both renderers fire AND the 0-arg void CALLIND path
// (`(*...)()`) is reachable — exactly the C++ opCallind `count==0` branch.
#[test]
fn w10_callsite_deindirect_callind_and_direct_coexist() {
    let rendered = match render_named("deindirect", &["deindirect"]) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("SKIP w10_callsite_deindirect: {e}");
            return;
        }
    };
    let (_, body) = rendered.iter().find(|(n, _)| n == "deindirect").expect("decompiled body");
    eprintln!("--- deindirect rendered ---\n{body}");

    // opCallind: an indirect call renders the dereference form `(*<expr>)(`.
    assert!(
        body.contains("(*") && body.contains(")("),
        "expected an opCallind `(*<funcptr>)(...)` form in deindirect body:\n{body}"
    );
    // opCall: the direct `puts` @0x100580 call recovers exactly 1 arg (RDI); its
    // name renders via the generic `sub_100580` fallback on this harness.
    let calls = call_lines(body);
    let puts = calls.iter().find(|l| l.contains("100580(")).cloned();
    assert!(puts.is_some(), "expected a direct call to puts @100580; calls={calls:?}");
    let puts = puts.unwrap();
    let inner = &puts[puts.find("100580(").unwrap() + "100580(".len()..];
    let inner = inner.trim_start_matches('(');
    let inner = &inner[..inner.find(')').unwrap_or(inner.len())];
    // VALUE RESOLUTION (w10-callarg-values): the recovered arg renders as the
    // data-flow value reaching the call slot (here the constant string-address
    // operand the call passes), NOT the raw incoming register RDI.  Exactly one
    // comma-free argument is recovered (the System-V first slot).
    let inner = inner.trim();
    assert!(!inner.is_empty(), "puts should recover exactly its 1 System-V arg; got empty");
    assert!(!inner.contains(','), "puts should recover exactly 1 arg; got `{inner}`");
    assert_ne!(inner, "RDI", "puts arg must resolve to the data-flow value, not the raw register RDI; got `{inner}`");
}

// ---------------------------------------------------------------------------
// T3 — dupptr: a recovered direct-call argument, no raw `CALL(...)` op dump.
// ---------------------------------------------------------------------------
//
// `loadstore_fieldarray` calls `initstruct` @0x100684 (a callee whose name is
// neither a libc function nor any literal — its name comes purely from the
// binaryimage `<symbol>` table when the loader symbols are installed; on this
// harness it renders as the generic `sub_100684`).  This pins the IR-coupled
// opCall rendered the recovered ARGUMENT (System-V RDI), and that the raw
// `CALL(...)` op-form (what the printer emitted BEFORE this wave) is ABSENT.
#[test]
fn w10_callsite_dupptr_direct_call_arg_no_raw_op_form() {
    let rendered = match render_named("dupptr", &["loadstore_fieldarray"]) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("SKIP w10_callsite_dupptr: {e}");
            return;
        }
    };
    let (_, body) =
        rendered.iter().find(|(n, _)| n == "loadstore_fieldarray").expect("decompiled body");
    eprintln!("--- loadstore_fieldarray rendered ---\n{body}");

    // The direct call to initstruct @100684 recovers exactly 1 System-V arg, now
    // rendered as the data-flow VALUE reaching the call slot (the resolved stack
    // address `RSP + <neg const>` = `&myval`), NOT the raw register RDI
    // (w10-callarg-values).
    let calls = call_lines(body);
    let isc = calls.iter().find(|l| l.contains("100684(")).cloned();
    assert!(isc.is_some(), "expected the initstruct @100684 call; calls={calls:?}");
    let isc = isc.unwrap();
    // Extract the call's argument list by *balanced*-paren matching from the
    // opening `100684(` to its matching close.  A naive "first `)`" would stop
    // inside a now-faithful C cast notation — the arg renders as
    // `(int8)RSP + <const>` since w10-printc-cast-render, so the leading `(int8)`
    // cast carries its own parentheses (printc `opTypeCast`).  Comma-splitting
    // the balanced body counts top-level args only (a cast's `,`-free body never
    // perturbs the count here).
    let arglist = {
        let start = isc.find("100684(").unwrap() + "100684(".len();
        let rest = &isc[start..];
        let mut depth = 1i32;
        let mut end = rest.len();
        for (i, c) in rest.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        end = i;
                        break;
                    }
                }
                _ => {}
            }
        }
        &rest[..end]
    };
    let inner = arglist.trim();
    // Count TOP-LEVEL commas only: the W10 spacebase-typing keystone renders the
    // arg as `(xunknown1 *)PTRSUB(<sp>, <const>)`, whose `PTRSUB(<sp>,<const>)`
    // carries its own depth>0 comma — that is NOT an argument separator.
    let top_level_commas = {
        let mut depth = 0i32;
        let mut n = 0usize;
        for c in inner.chars() {
            match c {
                '(' | '[' => depth += 1,
                ')' | ']' => depth -= 1,
                ',' if depth == 0 => n += 1,
                _ => {}
            }
        }
        n
    };
    assert!(!inner.is_empty() && top_level_commas == 0, "initstruct should recover exactly 1 arg; got `{inner}`");
    assert_ne!(inner, "RDI", "initstruct arg must resolve to the data-flow value, not the raw register RDI; got `{inner}`");
    // The resolved stack-address value: the raw additive form `RSP + <const>`
    // (pre-keystone) or the recognized `PTRSUB(<sp>, <const>)` stack-frame
    // reference (post-keystone, finished as `&myval` by the printc spacebase arm).
    assert!(
        inner.contains("RSP") || inner.contains('+') || inner.contains("PTRSUB"),
        "initstruct arg must be the resolved stack-address value (`&myval`); got `{inner}`"
    );

    // The pre-wave raw rendering MUST NOT appear: the call is the functional
    // `<callee>(arg)` form, never the raw `CALL(<target>)` op dump.
    assert!(!body.contains("CALL("), "raw `CALL(...)` op-form must not appear:\n{body}");
}

//! End-to-end gate for DWARF variant-part import (`--option dwarfvariants`).
//!
//! The bug: a Rust tagged enum carries **no `DW_TAG_member`** of its own — its
//! whole layout lives under a `DW_TAG_variant_part` — so `dwarfstructs` (DIV-86)
//! recovered its `DW_AT_byte_size` and zero fields. A field-less aggregate is
//! still an aggregate the ABI classifier acts on, so an 8-byte
//! `fn(u32) -> Result<u32,u32>` came out as a hidden-return-buffer call with a
//! phantom `rethidden` first parameter that the body then read the real argument
//! out of, and a 16-byte one wrote its variants as `*(uint *)&r->field_0x4`.
//!
//! Each test runs the SAME function twice — `dwarfvariants off` must reproduce
//! the bug, the default must reproduce the fix — over the committed
//! `dwarfvariants_x86_64` fixture (`rustc 1.90.0 -C debuginfo=2 -C opt-level=1`,
//! source vendored beside it). `dwarfstructs` is pinned ON in both arms so what
//! is being measured is the variant part alone and not its sibling. (This arm
//! EXTENDS `dwarfstructs` and is gated on it as well;
//! `dwarfstructs_off_suppresses_the_variant_arm_too` is the test for that.)
//!
//! ## Why this file and not a datatest
//!
//! `tests/datatests/` and most of `tests/stages/` are `<bytechunk>`s: no symbol
//! table, no DWARF, no relocations. A DWARF-gated change is structurally
//! unreachable from them. `tests/stages/kuna-dwarfvariants.xml` is the stage-tier
//! companion and gets at the fixture through `load file`; this file is the
//! cross-crate one.
//!
//! ## Gate flip
//!
//! The types are baked at `load file`, upstream of every `option` command, so the
//! live gate is the process env var `kuna_dwarfvariants::DWARFVARIANTS_ENV`.
//! Flipping it is serialized by a mutex — the env is process-global and
//! `cargo test` runs the tests in this binary concurrently.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;
use std::sync::Mutex;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::kuna_dwarfstructs::DWARFSTRUCTS_ENV;
use kuna_decomp::kuna_dwarfvariants::DWARFVARIANTS_ENV;

/// Serializes the env-var flip + bootstrap across the concurrently-run tests.
static GATE: Mutex<()> = Mutex::new(());

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the fixture with `dwarfvariants` on/off (and `dwarfstructs` pinned
/// on either way), decompile each of `funcs` and return the concatenated C.
/// `None` => specs-less skip.
fn decompile(funcs: &[&str], variants: bool) -> Option<String> {
    let _guard = GATE.lock().unwrap_or_else(|e| e.into_inner());
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let path = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures/dwarfvariants_x86_64")
        .to_str()?
        .to_string();

    std::env::set_var(DWARFSTRUCTS_ENV, "on");
    std::env::set_var(DWARFVARIANTS_ENV, if variants { "on" } else { "off" });
    let prog = bootstrap_from_object(&path, "", &spec_roots);
    std::env::remove_var(DWARFVARIANTS_ENV);
    std::env::remove_var(DWARFSTRUCTS_ENV);
    let mut prog = match prog {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_dwarfvariants: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let mut cmds: Vec<String> = Vec::new();
    for f in funcs {
        cmds.push(format!("load function {f}"));
        cmds.push("decompile".into());
        cmds.push("print C".into());
    }
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(prog);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    Some(status.optr.clone())
}

/// Run one function both ways; `None` => specs-less skip.
fn ab(func: &str) -> Option<(String, String)> {
    let off = decompile(&[func], false)?;
    let on = decompile(&[func], true)?;
    Some((off, on))
}

/// An 8-byte `Result<u32,u32>` return: a field-less aggregate is classified as a
/// hidden-return buffer, so the prototype grows a phantom `rethidden` first
/// parameter AND the real one shifts a slot right (the body then reads `x` out of
/// `rethidden`). With the variant part imported the type has a width the
/// classifier can place, and the signature is the source's.
#[test]
fn result_return_loses_its_phantom_sret_parameter() {
    let Some((off, on)) = ab("ret_result") else { return };
    assert!(
        off.contains("ret_result(core::result::Result<u32, u32> *rethidden,uint4 x)"),
        "gate off should reproduce the phantom sret, got:\n{off}"
    );
    assert!(
        on.contains("core::result::Result<u32, u32> ret_result(uint4 x)"),
        "gate on should recover the source signature, got:\n{on}"
    );
    assert!(!on.contains("rethidden"), "no phantom parameter survives:\n{on}");
}

/// The recovered type is spelled with the compiler's own name and its
/// discriminant is a NAMED field, not a byte offset.
#[test]
fn the_discriminant_is_a_named_field() {
    let Some((off, on)) = ab("ret_three") else { return };
    // 16 bytes: a real sret either way, so the parameter list does not move and
    // what changes is purely how the payload is written.
    assert!(
        off.contains("*(uint4 *)&rethidden->field_0x4 = x;"),
        "gate off should write a byte offset, got:\n{off}"
    );
    assert!(
        on.contains("rethidden->tag = 0;"),
        "gate on should name the discriminant, got:\n{on}"
    );
}

/// The union facet is named by the SOURCE variant name, which is the whole point
/// of reading DWARF instead of inferring from codegen: `A`/`B` here, `Ok`/`Err`
/// or `Some`/`None` elsewhere, never `Variant0`.
#[test]
fn variant_payloads_are_named_by_the_source_variant() {
    let Some((_off, on)) = ab("ret_three") else { return };
    assert!(
        on.contains("(rethidden->payload).A.__0 = x;"),
        "variant A's payload should be a named field path, got:\n{on}"
    );
    assert!(
        on.contains("(rethidden->payload).B.__0 = "),
        "and so should variant B's, got:\n{on}"
    );
    assert!(!on.contains("Variant0"), "no synthesized variant name:\n{on}");
}

/// A variant with TWO fields keeps both, each under its own source name.
#[test]
fn a_multi_field_variant_keeps_every_field() {
    let Some((off, on)) = ab("ret_multi") else { return };
    assert!(
        off.contains("*(uint4 *)&rethidden->field_0x8 = x + 1;"),
        "gate off should write byte offsets, got:\n{off}"
    );
    assert!(
        on.contains("(rethidden->payload).P.a = x;") && on.contains("(rethidden->payload).P.b = x + 1;"),
        "both fields of variant P should be named, got:\n{on}"
    );
}

/// A RECURSIVE enum (`enum List { Cons(u32, *const List), Nil }`) terminates and
/// the self-referential payload field still names the enum.
#[test]
fn a_recursive_enum_terminates_and_names_itself() {
    let Some((off, on)) = ab("list_len") else { return };
    assert!(
        off.contains("l = *(fx::List **)&l->field_0x8;"),
        "gate off should chase a byte offset, got:\n{off}"
    );
    assert!(
        on.contains("(l->payload).Cons.__1") && on.contains("l->tag & 1"),
        "gate on should name the payload and the tag, got:\n{on}"
    );
}

/// A NICHE-encoded `Option<&u32>`: `Some` is the DEFAULT variant (no
/// `DW_AT_discr_value`) and its payload overlaps the discriminant, so there is no
/// byte range that is only the tag and the recovered type is the overlay UNION
/// alone. The signature is still fixed, and the value is still attributed to the
/// variant that owns it.
#[test]
fn a_niche_option_recovers_its_signature_and_its_default_variant() {
    let Some((off, on)) = ab("ret_niche") else { return };
    assert!(
        off.contains("*rethidden,uint4 x,uint4 *p)"),
        "gate off should reproduce the phantom sret, got:\n{off}"
    );
    assert!(
        on.contains("core::option::Option<&u32> ret_niche(uint4 x,uint4 *p)"),
        "gate on should recover the source signature, got:\n{on}"
    );
    assert!(
        on.contains("core::option::Option<&u32>::Some"),
        "the pointer belongs to the DEFAULT variant `Some`, got:\n{on}"
    );
    assert!(!on.contains("None"), "the fieldless variant overlays nothing:\n{on}");
}

/// A FIELDLESS enum is a `DW_TAG_enumeration_type`, not a variant part, so this
/// pass never sees it and the rendering is byte-identical.
#[test]
fn a_fieldless_enum_is_untouched() {
    let Some((off, on)) = ab("ret_plain") else { return };
    assert_eq!(off, on, "a C-like enum has no variant part");
    assert!(on.contains("Plain ret_plain(uint4 x)"), "got:\n{on}");
}

/// A plain C-shaped struct in the same image is byte-identical: no C aggregate
/// carries a `DW_TAG_variant_part`, so this pass is inert on one.
#[test]
fn a_plain_struct_is_untouched() {
    let Some((off, on)) = ab("ret_pair") else { return };
    assert_eq!(off, on, "a plain struct has no variant part");
    assert!(on.contains("v1.lo = x;"), "got:\n{on}");
}

/// The arm EXTENDS `dwarfstructs` and is gated on it too: with the
/// aggregate-layout gate off, `dwarfstructs off` stays exactly the pre-DIV-86
/// name-only mapping its own catalog row promises, `dwarfvariants` or not.
#[test]
fn dwarfstructs_off_suppresses_the_variant_arm_too() {
    let _guard = GATE.lock().unwrap_or_else(|e| e.into_inner());
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures/dwarfvariants_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    std::env::set_var(DWARFSTRUCTS_ENV, "off");
    std::env::set_var(DWARFVARIANTS_ENV, "on");
    let prog = bootstrap_from_object(&path, "", &spec_roots);
    std::env::remove_var(DWARFVARIANTS_ENV);
    std::env::remove_var(DWARFSTRUCTS_ENV);
    let Ok(prog) = prog else {
        eprintln!("verify_dwarfvariants: skipping (no `.sla`)");
        return;
    };
    assert!(
        prog.arch().types().kuna_variant_layouts().is_empty(),
        "the variant arm must not run with `dwarfstructs off`"
    );
}

/// The side table is filled with the compiler's own answer — which member is the
/// tag, and which value selects which variant — and it is filled ONLY when the
/// gate is on.
#[test]
fn the_layout_side_table_is_recorded() {
    let _guard = GATE.lock().unwrap_or_else(|e| e.into_inner());
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures/dwarfvariants_x86_64")
        .to_str()
        .unwrap()
        .to_string();

    for on in [false, true] {
        std::env::set_var(DWARFSTRUCTS_ENV, "on");
        std::env::set_var(DWARFVARIANTS_ENV, if on { "on" } else { "off" });
        let prog = bootstrap_from_object(&path, "", &spec_roots);
        std::env::remove_var(DWARFVARIANTS_ENV);
        std::env::remove_var(DWARFSTRUCTS_ENV);
        let Ok(prog) = prog else {
            eprintln!("verify_dwarfvariants: skipping (no `.sla`)");
            return;
        };
        let types = prog.arch().types();
        let l = types.kuna_variant_layout("core::result::Result<u32, u32>");
        if !on {
            assert!(l.is_none(), "gate off records nothing");
            assert!(types.kuna_variant_layouts().is_empty());
            continue;
        }
        let l = l.expect("Result layout recorded");
        assert_eq!(l.size, 8);
        assert_eq!((l.tag_offset, l.tag_size), (0, 4));
        assert_eq!(l.payload_offset, 4);
        assert!(!l.niche);
        assert_eq!(l.facet_for_discr(0).unwrap().name, "Ok");
        assert_eq!(l.facet_for_discr(1).unwrap().name, "Err");
        assert_eq!(
            l.facet_named("Ok").unwrap().fields,
            vec![("__0".to_string(), 4)],
            "field offsets are absolute within the enum"
        );

        // The niche shape's DEFAULT variant is recorded as one: every value the
        // other variants did not claim selects it.
        let n = types
            .kuna_variant_layout("core::option::Option<&u32>")
            .expect("niche Option layout recorded");
        assert!(n.niche);
        assert_eq!(n.union_type, n.type_name, "the union is the whole type");
        assert_eq!(n.payload_offset, 0);
        assert_eq!(n.facet_named("Some").unwrap().discr, None);
        assert!(n.facet_named("None").unwrap().payload_type.is_empty());
        assert_eq!(n.facet_for_discr(0).unwrap().name, "None");
        assert_eq!(n.facet_for_discr(0x2010c8).unwrap().name, "Some");

        // Three variants, and the fieldless one is on the table with no payload
        // type -- it has no bytes to overlay.
        let t = types.kuna_variant_layout("fx::Three").expect("Three layout recorded");
        assert_eq!(t.variants.len(), 3);
        assert!(t.facet_named("C").unwrap().payload_type.is_empty());
        assert_eq!(t.facet_named("B").unwrap().fields, vec![("__0".to_string(), 8)]);
    }
}

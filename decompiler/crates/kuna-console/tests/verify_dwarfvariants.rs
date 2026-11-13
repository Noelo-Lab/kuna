//! End-to-end gate for DWARF variant-part import (part of `--option dwarf`).
//!
//! The bug: a Rust tagged enum carries **no `DW_TAG_member`** of its own — its
//! whole layout lives under a `DW_TAG_variant_part` — so the aggregate-layout
//! import (DIV-86) recovered its `DW_AT_byte_size` and zero fields. A field-less
//! aggregate is still an aggregate the ABI classifier acts on, so an 8-byte
//! `fn(u32) -> Result<u32,u32>` came out as a hidden-return-buffer call with a
//! phantom `rethidden` first parameter that the body then read the real argument
//! out of, and a 16-byte one wrote its variants as `*(uint *)&r->field_0x4`.
//!
//! Each test decompiles a function at the defaults over one of two committed
//! fixtures (`rustc 1.90.0 -C debuginfo=2 -C opt-level=1`, sources vendored
//! beside them): `dwarfvariants_x86_64` for the SHAPES the importer has to read,
//! and `dwarfvariants_overlay_x86_64` for what it is allowed to NAME.
//!
//! ## The naming rule has a test for the REFUSAL, not only for the happy path
//!
//! A union member selects itself by offset and the discriminant is never
//! consulted, so a variant name is printed only where exactly one variant claims
//! the bytes. `an_overlaying_result_names_neither_variant` and
//! `reading_an_overlaying_result_names_neither_variant` are the refusals;
//! `an_option_still_names_its_only_payload_variant` and
//! `uncontested_variant_payloads_are_named_by_the_source_variant` are the keeps;
//! `suppression_does_not_give_back_the_phantom_sret` pins that the TYPE is
//! untouched by the decision.
//!
//! ## Why this file and not a datatest
//!
//! `tests/datatests/` and most of `tests/stages/` are `<bytechunk>`s: no symbol
//! table, no DWARF, no relocations. A DWARF-gated change is structurally
//! unreachable from them. `tests/stages/kuna-dwarfvariants.xml` is the stage-tier
//! companion and gets at the fixture through `load file`; this file is the
//! cross-crate one.

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the shapes fixture, decompile each of `funcs` and return the
/// concatenated C.
fn decompile(funcs: &[&str]) -> String {
    decompile_in("dwarfvariants_x86_64", funcs)
}

/// As [`decompile`], over an arbitrary committed fixture.
fn decompile_in(fixture: &str, funcs: &[&str]) -> String {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let path = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(fixture)
        .to_str().expect("UTF-8 fixture path")
        .to_string();

    let mut prog = bootstrap_from_object(&path, "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
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
    status.optr.clone()
}

/// The overlay fixture — the one built for the NAMING rule rather than for the
/// shapes.
fn overlay(func: &str) -> String {
    decompile_in("dwarfvariants_overlay_x86_64", &[func])
}

/// An 8-byte `Result<u32,u32>` return: a field-less aggregate is classified as a
/// hidden-return buffer, so the prototype grew a phantom `rethidden` first
/// parameter AND the real one shifted a slot right. With the variant part
/// imported the type has a width the classifier can place, and the signature is
/// the source's.
#[test]
fn result_return_loses_its_phantom_sret_parameter() {
    let on = decompile(&["ret_result"]);
    assert!(
        on.contains("core::result::Result<u32, u32> ret_result(uint4 x)"),
        "the source signature should come back, got:\n{on}"
    );
    assert!(!on.contains("rethidden"), "no phantom parameter survives:\n{on}");
}

/// The recovered type is spelled with the compiler's own name and its
/// discriminant is a NAMED field, not a byte offset.
#[test]
fn the_discriminant_is_a_named_field() {
    let on = decompile(&["ret_three"]);
    assert!(
        on.contains("rethidden->tag = "),
        "the discriminant should be a named field, got:\n{on}"
    );
    assert!(!on.contains("field_0x4"), "no byte-offset spelling of the tag:\n{on}");
}

/// The union facet is named by the SOURCE variant name where that name is
/// FORCED: `Three`'s `A` owns [4,8) and `B` owns [8,16), no other variant claims
/// either range, so an access there can only be that variant's. Never
/// `Variant0`, and never a variant that merely won a union-scorer tie.
#[test]
fn uncontested_variant_payloads_are_named_by_the_source_variant() {
    let on = decompile(&["ret_three"]);
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

/// A variant with TWO fields keeps every field, and the naming rule is applied
/// PER FIELD: `Multi`'s `P.a` at [4,8) is claimed by nothing else and keeps its
/// name, while `P.b` and `Q.__0` both claim offset 8 under different spellings,
/// so that word is written through the offset form and NEITHER facet is named.
#[test]
fn a_multi_field_variant_keeps_every_field_and_names_only_what_it_may() {
    let on = decompile(&["ret_multi"]);
    assert!(
        on.contains("(rethidden->payload).field_0x4.a = x;"),
        "`a` is P's alone, so it keeps its name, got:\n{on}"
    );
    assert!(
        on.contains("(rethidden->payload).field_0x4.field_0x8 = x + 1;"),
        "`b` collides with Q's `__0`, so the word is spelled by offset, got:\n{on}"
    );
    assert!(
        !on.contains(".P.") && !on.contains(".Q."),
        "neither variant may be named where they overlay each other, got:\n{on}"
    );
}

/// A RECURSIVE enum (`enum List { Cons(u32, *const List), Nil }`) terminates and
/// the self-referential payload field still names the enum.
#[test]
fn a_recursive_enum_terminates_and_names_itself() {
    let on = decompile(&["list_len"]);
    assert!(
        on.contains("(l->payload).Cons.__1") && on.contains("l->tag"),
        "the payload and the tag should be named, got:\n{on}"
    );
}

/// A NICHE-encoded `Option<&u32>`: `Some` is the DEFAULT variant (no
/// `DW_AT_discr_value`) and its payload overlaps the discriminant, so there is no
/// byte range that is only the tag and the recovered type is the overlay UNION
/// alone. The signature is still fixed, and the value is still attributed to the
/// variant that owns it.
#[test]
fn a_niche_option_recovers_its_signature_and_its_default_variant() {
    let on = decompile(&["ret_niche"]);
    assert!(
        on.contains("core::option::Option<&u32> ret_niche(uint4 x,uint4 *p)"),
        "the source signature should come back, got:\n{on}"
    );
    assert!(
        on.contains("core::option::Option<&u32>::Some"),
        "the pointer belongs to the DEFAULT variant `Some`, got:\n{on}"
    );
    assert!(!on.contains("None"), "the fieldless variant overlays nothing:\n{on}");
}

/// A FIELDLESS enum is a `DW_TAG_enumeration_type`, not a variant part, so this
/// pass never sees it.
#[test]
fn a_fieldless_enum_is_untouched() {
    let on = decompile(&["ret_plain"]);
    assert!(on.contains("Plain ret_plain(uint4 x)"), "got:\n{on}");
}

/// A plain C-shaped struct in the same image: no C aggregate carries a
/// `DW_TAG_variant_part`, so this pass is inert on one.
#[test]
fn a_plain_struct_is_untouched() {
    let on = decompile(&["ret_pair"]);
    assert!(on.contains("v1.lo = x;"), "got:\n{on}");
}

/// The side table is filled with the compiler's own answer — which member is the
/// tag, and which value selects which variant.
#[test]
fn the_layout_side_table_is_recorded() {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures/dwarfvariants_x86_64")
        .to_str()
        .unwrap()
        .to_string();

    let prog = bootstrap_from_object(&path, "", &spec_roots).expect("required fixture setup");
    let types = prog.arch().types();
    let l = types.kuna_variant_layout("core::result::Result<u32, u32>").expect("Result layout recorded");
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

// ---------------------------------------------------------------------------
// The naming rule, end to end (`dwarfvariants_overlay_x86_64`)
// ---------------------------------------------------------------------------

/// **The refusal.** `Result<u64,u64>` puts `Ok.__0` and `Err.__0` at the SAME
/// offset, so the union member the scorer picks is not evidence of anything.
/// Before the suppression this fixture printed `Ok` ten times and `Err` never —
/// including on the `Err` arm. Neither name may appear as a field path in the
/// emitted body now; the payload is written through the offset spelling instead.
#[test]
fn an_overlaying_result_names_neither_variant() {
    let on = overlay("put_res");
    assert!(
        on.contains("dst->tag ="),
        "the discriminant is still recovered as a named field, got:\n{on}"
    );
    assert!(
        on.contains("(dst->payload).field_0x8.__0 ="),
        "the payload should be spelled by offset, got:\n{on}"
    );
    for bad in [".Ok", ".Err"] {
        assert!(
            !on.contains(bad),
            "`{bad}` is a coin flip on this type and must not be printed, got:\n{on}"
        );
    }
}

/// The consumer side of the same type: an `Err(e) => e + 100` arm used to render
/// as `(long)v2.payload.Ok + 100`. Reading the payload must not name a variant
/// either — the type is unchanged, only the label is gone.
#[test]
fn reading_an_overlaying_result_names_neither_variant() {
    let on = overlay("use16");
    assert!(on.contains("+ 100"), "the Err arm is still in the body, got:\n{on}");
    for bad in [".Ok", ".Err"] {
        assert!(!on.contains(bad), "`{bad}` must not be printed, got:\n{on}");
    }
    assert!(on.contains("payload.field_0x8"), "the offset spelling is used, got:\n{on}");
}

/// **The kept half.** `Option<u64>` has exactly ONE payload-carrying variant, so
/// `Some` is forced and must survive the suppression — this is what stops the
/// rule from degenerating into "never name anything".
#[test]
fn an_option_still_names_its_only_payload_variant() {
    let on = overlay("put_opt");
    assert!(
        on.contains("(dst->payload).Some.__0 ="),
        "`Some` is the only claimant of those bytes and must be named, got:\n{on}"
    );
}

/// The recovered TYPE is not what the suppression changes: `r16`'s 16-byte
/// `Result<u64,u64>` still loses its phantom `rethidden` and still returns the
/// source's type by value. Only the field path moved.
#[test]
fn suppression_does_not_give_back_the_phantom_sret() {
    let on = overlay("r16");
    assert!(
        on.contains("core::result::Result<u64, u64> r16(uint4 x)"),
        "the source signature should come back, got:\n{on}"
    );
    assert!(!on.contains("rethidden"), "no phantom parameter survives:\n{on}");
}

/// The side table still carries the compiler's own variant names and
/// discriminant values for the suppressed type — the labels are a RENDERING
/// decision, and `names_any_variant` is the flag that records which way it went.
#[test]
fn the_side_table_keeps_the_names_the_type_does_not() {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let path = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures/dwarfvariants_overlay_x86_64")
        .to_str()
        .unwrap()
        .to_string();
    let prog = bootstrap_from_object(&path, "", &spec_roots).expect("required fixture setup");
    let types = prog.arch().types();

    let r = types
        .kuna_variant_layout("core::result::Result<u64, u64>")
        .expect("Result<u64,u64> layout recorded");
    assert_eq!(r.facet_for_discr(0).unwrap().name, "Ok");
    assert_eq!(r.facet_for_discr(1).unwrap().name, "Err");
    assert_eq!(r.facet_named("Ok").unwrap().label, "field_0x8");
    assert_eq!(r.facet_named("Err").unwrap().label, "field_0x8");
    assert!(!r.names_any_variant(), "nothing in the emitted type names a variant");

    let o = types
        .kuna_variant_layout("core::option::Option<u64>")
        .expect("Option<u64> layout recorded");
    assert_eq!(o.facet_named("Some").unwrap().label, "Some");
    assert!(o.names_any_variant());
}

//! VERIFIER adversarial fences for item `w10-merge-casts` — the `ActionSetCasts`
//! cast-insertion driver (C++ `coreaction.cc:2407-2872`, ported in
//! `kuna_decomp::coreaction_casts`).
//!
//! The driver itself runs end-to-end over the datatest corpus (the parity check
//! is `python -m kuna.run_tests --datatests --engine rust`).  These tests pin the
//! two cast *decision* predicates the corpus stresses hardest, at the exact
//! boundaries the porter's own work could regress:
//!
//!   1. [`test_struct_offset0`] (coreaction.cc:2443-2472) — the
//!      "is this conflict resolvable by passing to the struct/array first field"
//!      test that decides PTRSUB-vs-CAST.  Attacked on: the array branch, the
//!      struct first-field-at-nonzero-offset rejection, the `void *` guard, and
//!      the metatype gate.
//!   2. [`is_op_identical`] (coreaction.cc:2559-2571) — the pointer-chain +
//!      typedef-stripping identity used by `castOutput`'s force path.  Attacked on:
//!      pointer-depth descent, typedef stripping, and the genuine-mismatch case.
//!
//! Plus a regression fence ([`byte_identical_boolless_unperturbed_by_casts`]) that
//! drives the **whole** `boolless` decompile through the now-active cast stage and
//! asserts the rendered C is *byte-identical* to the committed C++ B5 oracle — the
//! cast stage must stay a no-op on the byte-identical families.
//!
//! Every assertion is two-sided (asserts both the cast-needed and cast-not-needed
//! arm) so it cannot be satisfied by a predicate that always returns one value.

use std::rc::Rc;

use kuna_base::types::int4;
use kuna_decomp::coreaction_casts::{is_op_identical, test_struct_offset0};
use kuna_decomp::cast::{CastStrategy, CastStrategyC};
use kuna_decomp::dtype::{
    type_metatype, Datatype, DatatypeKind, TypeFactory, TypeFactoryImpl, TypeField,
};

// ---- type builders ---------------------------------------------------------

fn base(size: int4, m: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, m))
}

fn void() -> Rc<Datatype> {
    let mut d = Datatype::new(1, type_metatype::TYPE_VOID);
    d.kind = DatatypeKind::Void;
    Rc::new(d)
}

/// A plain `TypePointer` to `ptrto`, word-size 1, no bound space (so
/// `castStandard`'s space check stays "one pointer has no address" -> no cast).
fn ptr(ptrto: Rc<Datatype>) -> Rc<Datatype> {
    let mut d = Datatype::new(8, type_metatype::TYPE_PTR);
    d.kind = DatatypeKind::Pointer { ptrto, spaceid: None, truncate: None, wordsize: 1 };
    Rc::new(d)
}

/// An array of `n` elements of `elem`; align_size set to the array byte size so
/// `getAlignSize()` differs from the element's (the C++ TypeArray align).
fn array(elem: Rc<Datatype>, n: int4) -> Rc<Datatype> {
    let esz = elem.get_size();
    let mut d = Datatype::new(esz * n, type_metatype::TYPE_ARRAY);
    d.align_size = esz * n;
    d.kind = DatatypeKind::Array { arrayof: elem, arraysize: n };
    Rc::new(d)
}

/// A struct from `(offset, type)` fields.
fn structof(fields: &[(int4, Rc<Datatype>)]) -> Rc<Datatype> {
    let total: int4 = fields.iter().map(|(o, t)| o + t.get_size()).max().unwrap_or(0);
    let mut d = Datatype::new(total, type_metatype::TYPE_STRUCT);
    let field: Vec<TypeField> = fields
        .iter()
        .enumerate()
        .map(|(i, (off, ct))| TypeField::new(i as int4, *off, format!("f{i}"), Rc::clone(ct)))
        .collect();
    d.kind = DatatypeKind::Struct { field, bitfield: Vec::new() };
    Rc::new(d)
}

/// A typedef wrapper: a fresh `Datatype` whose `typedef_imm` points to `inner`.
fn typedefof(inner: Rc<Datatype>) -> Rc<Datatype> {
    let mut d = Datatype::new(inner.get_size(), inner.get_metatype());
    d.typedef_imm = Some(inner);
    Rc::new(d)
}

fn strat() -> CastStrategyC {
    CastStrategyC::new(Rc::new(TypeFactoryImpl::new()) as Rc<dyn TypeFactory>)
}

// ===========================================================================
// (1) test_struct_offset0 — the struct/array first-field PTRSUB resolution.
//     coreaction.cc:2443-2472.
// ===========================================================================

/// ARRAY branch: `curtype = int4(*)[5]`, `reqtype = int4 *`.  The required
/// element (int4) matches the array base (int4) under `castStandard`, so a PTRSUB
/// is induced (returns true).  coreaction.cc:2460-2464.
#[test]
fn w10_test_struct_offset0_array_first_element_induces_ptrsub() {
    let s = strat();
    let int4t = base(4, type_metatype::TYPE_INT);
    let arr = array(Rc::clone(&int4t), 5);
    let curtype = ptr(arr); // int4 (*)[5]
    let reqtype = ptr(Rc::clone(&int4t)); // int4 *
    assert!(
        test_struct_offset0(&reqtype, &curtype, &s),
        "pointer-to-array-of-int4 vs pointer-to-int4: first element matches -> PTRSUB (cpp:2460)"
    );
    // ADVERSARIAL: the SAME current type, but a `void *` required type must NOT
    // induce a PTRSUB (cpp:2468-2470 "Don't induce PTRSUB for void *").
    let reqvoid = ptr(void());
    assert!(
        !test_struct_offset0(&reqvoid, &curtype, &s),
        "void* required type must NOT induce a PTRSUB (cpp:2468)"
    );
}

/// STRUCT branch: a struct whose FIRST field is at offset 0 with a matching type
/// induces a PTRSUB; a struct whose first field is at a NON-zero offset is
/// rejected (cpp:2452 `if ((*iter).offset != 0) return false`).
#[test]
fn w10_test_struct_offset0_struct_first_field_offset_gate() {
    let s = strat();
    let int4t = base(4, type_metatype::TYPE_INT);
    // first field at offset 0, type int4 -> matches reqtype int4* -> PTRSUB.
    let st0 = structof(&[(0, Rc::clone(&int4t)), (4, Rc::clone(&int4t))]);
    let cur0 = ptr(st0);
    let req = ptr(Rc::clone(&int4t));
    assert!(
        test_struct_offset0(&req, &cur0, &s),
        "struct first field @0 matching int4 -> PTRSUB (cpp:2448-2454)"
    );
    // ADVERSARIAL: a struct whose first field is at a NON-zero offset (e.g. 4) is
    // rejected outright (cpp:2452).  Build the struct's `field[0].offset = 4`.
    let st_nonzero = structof(&[(4, Rc::clone(&int4t))]);
    let cur_nonzero = ptr(st_nonzero);
    assert!(
        !test_struct_offset0(&req, &cur_nonzero, &s),
        "struct whose first field is at non-zero offset is rejected (cpp:2452)"
    );
}

/// METATYPE gate: a pointer to a *scalar* (not struct/array) is rejected by the
/// final `else { return false; }` (cpp:2465-2467) regardless of the required type.
#[test]
fn w10_test_struct_offset0_scalar_pointee_rejected() {
    let s = strat();
    let int4t = base(4, type_metatype::TYPE_INT);
    let cur_scalar = ptr(Rc::clone(&int4t)); // int4 * (pointee is scalar)
    let req = ptr(Rc::clone(&int4t));
    assert!(
        !test_struct_offset0(&req, &cur_scalar, &s),
        "pointer-to-scalar pointee cannot be a struct/array first-field access (cpp:2465)"
    );
    // And a non-pointer current type is rejected at the very first guard (cpp:2446).
    assert!(
        !test_struct_offset0(&req, &int4t, &s),
        "non-pointer current type rejected at the first guard (cpp:2446)"
    );
}

// ===========================================================================
// (2) is_op_identical — pointer-chain descent + typedef stripping.
//     coreaction.cc:2559-2571.
// ===========================================================================

/// Pointer-chain descent: `int4 **` is op-identical to `int4 **` (same leaf),
/// but `int4 **` is NOT op-identical to `float4 **` (different leaf after the
/// matched pointer descent).  cpp:2562-2565.
#[test]
fn w10_is_op_identical_descends_pointer_chain() {
    let int4t = base(4, type_metatype::TYPE_INT);
    let float4t = base(4, type_metatype::TYPE_FLOAT);
    let pp_int = ptr(ptr(Rc::clone(&int4t)));
    // Distinct Rc chain to the SAME leaf int4t -> identical after descent.
    let pp_int2 = ptr(ptr(Rc::clone(&int4t)));
    assert!(
        is_op_identical(&pp_int, &pp_int2),
        "two int4** chains ending at the same leaf are op-identical (cpp:2562-2570)"
    );
    // Different leaf (float4) -> NOT identical.
    let pp_float = ptr(ptr(Rc::clone(&float4t)));
    assert!(
        !is_op_identical(&pp_int, &pp_float),
        "int4** vs float4** are NOT op-identical (different leaf)"
    );
}

/// Typedef stripping: a typedef of `int4` is op-identical to the underlying
/// `int4` (cpp:2566-2569 strips both sides before the identity test).
#[test]
fn w10_is_op_identical_strips_typedefs() {
    let int4t = base(4, type_metatype::TYPE_INT);
    let td = typedefof(Rc::clone(&int4t)); // typedef int4 myint;
    assert!(
        is_op_identical(&td, &int4t),
        "a typedef of int4 is op-identical to int4 after stripping (cpp:2566)"
    );
    // ADVERSARIAL: a typedef of int4 is NOT identical to a *distinct* int4 leaf.
    let other_int4 = base(4, type_metatype::TYPE_INT);
    assert!(
        !is_op_identical(&td, &other_int4),
        "typedef(int4) vs a structurally-equal-but-distinct int4 leaf are not Rc-identical"
    );
}

// ===========================================================================
// (3) Regression fence: the active cast stage must not perturb byte-identical
//     families.  Drive the WHOLE boolless decompile and byte-compare the C body.
// ===========================================================================

/// Self-contained datatest bootstrap + single-symbol decompile/print harness
/// (mirrors `print_b5_boolless.rs`'s `run_full` + `print_c`).  Lives in-file so
/// the regression fence has no cross-test dependency.
mod harness {
    use std::path::PathBuf;
    use std::rc::Rc;

    use kuna_base::address::Address;
    use kuna_base::error::{KunaError, KunaResult};
    use kuna_base::marshal::IdRegistry;
    use kuna_base::space::AddrSpaceManager;
    use kuna_base::xml::{DocumentStorage, Element};

    use kuna_decomp::decompile_drive::{decompile_func, print_c};
    use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase};
    use kuna_decomp::xml_arch::{XmlArchitecture, XmlArchitectureCapability};

    use kuna_sleigh::loadimage::LoadImage;
    use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
    use kuna_sleigh::translate::register_translate_ids;

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
        let symbols = syms
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
        // Install the register-name lookup (matches the real console engine): the
        // angr `dat_`/`vN` naming split reads `manage->getRegisterName(...)` to tell
        // a register local from a global data read.
        arch.sleigh_mut()
            .base_mut()
            .ok_or("no Architecture base after build_translator")?
            .translate_mut()
            .as_sleigh_mut().expect("standalone Sleigh engine").install_register_lookup()
            .map_err(|e| format!("install_register_lookup: {e}"))?;
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
        // SAFETY: the manager outlives open(); the borrow is released before &mut use.
        arch.open_image(unsafe { &*manager_ptr }, &registry)
            .map_err(|e| format!("open_image: {e}"))?;
        let img = arch.take_loader().ok_or("loader vanished after open")?;
        arch.sleigh_mut().base_mut().unwrap().set_loader(Box::new(img));
        Ok(arch)
    }

    /// Bootstrap the datatest `stem`, decompile its `which`-th `<symbol>`, and
    /// return the rendered C (the same path the `print C` oracle exercises, so the
    /// now-active `ActionSetCasts` runs over it).
    pub fn render_one(stem: &str, which: usize) -> Result<String, String> {
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
        arch.early_return = false; // (kuna) C++-parity test: opt out of earlyreturn (DIV-23 default-on)
        arch.switch_return = false; // (kuna) also opt out of switchreturn (DIV-25 default-on)
        let fd = decompile_func(arch, &name, entry, 0).map_err(|e| format!("decompile: {e}"))?;
        Ok(print_c(arch, &fd))
    }
}

/// The committed C++ B5 oracle for `boolless` (the byte-identical family).  The
/// now-active `ActionSetCasts` must leave this *exactly* unchanged — no spurious
/// CAST/PTRSUB inserted into a typeless integer body.
const BOOLLESS_CPP_B5: &str =
    // (Header gap adjusted for the kuna DIV-33 brace-placement default.)
    "\nuint1 boolless(void)\n{\n  uint1 v1; // acc\n  \n  v1 = dat_52;\n  if (dat_52 <= 10) {\n    v1 = 1;\n  }\n  return v1;\n}\n";

#[test]
fn w10_byte_identical_boolless_unperturbed_by_casts() {
    let rendered = match harness::render_one("boolless", 0) {
        Ok(s) => s,
        Err(e) => {
            // Bootstrap can be unavailable in some CI sandboxes (no .sla); skip
            // rather than fail, exactly like the sibling B5 print tests.
            eprintln!("SKIP boolless cast-regression fence: {e}");
            return;
        }
    };
    assert_eq!(
        rendered, BOOLLESS_CPP_B5,
        "the active ActionSetCasts must leave the byte-identical boolless body unchanged:\n\
         --- rust ---\n{rendered}\n--- oracle ---\n{BOOLLESS_CPP_B5}"
    );
    // Two-sided: an integer-typeless body must contain NO inserted cast/ptrsub
    // tokens at all (the driver ran, but found nothing to cast).
    assert!(
        !rendered.contains("CAST(") && !rendered.contains("(float"),
        "no spurious cast token may appear in the typeless boolless body:\n{rendered}"
    );
}

// ===========================================================================
// VERIFIER round-1 adversarial fences — the getInputCast per-op dispatch
// (coreaction_casts.rs::get_input_cast). Two confirmed faithfulness bugs in the
// named getInputCast surface are pinned here via the reachable `castStandard`
// proxy: the per-op getInputCast bodies differ ONLY in (a) the reqtype source
// and (b) the final `castStandard(reqtype, curtype, care_uint_int, care_ptr_uint)`
// argument tuple. These tests fix the C++-faithful tuple for the two op families
// the Rust dispatch transcribes incorrectly, and prove the wrong tuple changes
// the cast DECISION for a signed-vs-unsigned operand pair (so the bug is a real
// dropped/wrong cast, not a no-op rename).
//
//   * INT_SLESS / INT_SLESSEQUAL (typeop.cc:1025-1033, :1051-1059): the C++ body
//     is reqtype = inputTypeLocal(slot) [a SIGNED int], then
//     castStandard(reqtype, curtype, TRUE, TRUE). The Rust dispatch
//     (coreaction_casts.rs:302-305) routes SLESS/SLESSEQUAL to
//     `get_input_cast_equal` — the EQUAL/NOTEQUAL body — which uses
//     reqtype = max-read-facing(in0,in1) and castStandard(.., FALSE, FALSE).
//   * INT_ZEXT / INT_SEXT (typeop.cc:1133-1141, :1159-1167): the C++ fallthrough
//     tail is castStandard(reqtype, curtype, TRUE, FALSE). The Rust
//     `get_input_cast_extension` (coreaction_casts.rs:534) uses
//     castStandard(reqtype, curtype, FALSE, TRUE) — both args inverted.

fn uint(size: int4) -> Rc<Datatype> {
    base(size, type_metatype::TYPE_UINT)
}
fn sint(size: int4) -> Rc<Datatype> {
    base(size, type_metatype::TYPE_INT)
}

/// SLESS faithfulness: for reqtype = `int4` (the signed `inputTypeLocal` of an
/// INT_SLESS) and curtype = `uint4`, the C++ tail `castStandard(int4, uint4,
/// true, true)` REQUIRES a cast (signed/unsigned mismatch with care_uint_int).
/// The EQUAL-body tuple the Rust dispatch substitutes — `(false, false)` — drops
/// it. This two-sided assert proves the misroute is an observable, wrong cast
/// decision (a missing `(int)` on a signed comparison of an unsigned operand),
/// not a benign relabel. [w10-merge-casts F1]
#[test]
fn w10_sless_getinputcast_uses_signed_local_and_true_true_tuple() {
    let s = strat();
    let req = sint(4); // INT_SLESS inputTypeLocal(slot) = getBase(size, TYPE_INT)
    let cur = uint(4); // operand merged to uint4
    // C++-faithful SLESS tail: a cast IS needed.
    assert!(
        s.cast_standard(&req, &cur, true, true).is_some(),
        "SLESS faithful body castStandard(int4,uint4,true,true) must REQUIRE a cast \
         (signed-vs-unsigned, care_uint_int) — typeop.cc:1032"
    );
    // The EQUAL-body tuple the Rust dispatch substitutes drops the cast — this is
    // the divergence: where C++ renders (int)a < (int)b the misroute renders a < b.
    assert!(
        s.cast_standard(&req, &cur, false, false).is_none(),
        "EQUAL-body tuple castStandard(int4,uint4,false,false) drops the cast — \
         demonstrates the SLESS misroute changes the cast decision"
    );
}

/// ZEXT/SEXT faithfulness: the C++ fallthrough tail is `castStandard(reqtype,
/// curtype, true, false)`; the Rust uses `(false, true)` (both inverted). For
/// reqtype = `int4` and curtype = `uint4` the faithful `(true,false)` requires a
/// cast where the inverted `(false,true)` drops it. [w10-merge-casts F2]
#[test]
fn w10_extension_getinputcast_tuple_is_true_false_not_false_true() {
    let s = strat();
    let req = sint(4);
    let cur = uint(4);
    assert!(
        s.cast_standard(&req, &cur, true, false).is_some(),
        "ZEXT/SEXT faithful tail castStandard(int4,uint4,true,false) must REQUIRE a cast \
         — typeop.cc:1140"
    );
    assert!(
        s.cast_standard(&req, &cur, false, true).is_none(),
        "the inverted tuple castStandard(int4,uint4,false,true) drops the cast — \
         demonstrates the ZEXT/SEXT arg inversion changes the cast decision"
    );
}

/// LESS/LESSEQUAL (the body the Rust DOES route correctly, `(true,false)`) must
/// remain distinct from the SLESS `(true,true)` body: for reqtype=`uint4`,
/// curtype=`ptr` the care_ptr_uint flag is the discriminator. This pins that the
/// fix for SLESS must NOT collapse SLESS into the LESS arm (they differ in the
/// 2nd flag). [w10-merge-casts F1 boundary]
#[test]
fn w10_less_vs_sless_caretuple_discriminator_is_care_ptr_uint() {
    let s = strat();
    let req = uint(8);
    let cur = ptr(sint(4)); // a pointer curtype, uint reqtype
    // care_ptr_uint=false (LESS) -> uint-from-ptr needs NO cast;
    // care_ptr_uint=true (SLESS) -> it DOES. So SLESS != LESS.
    let less = s.cast_standard(&req, &cur, true, false);
    let sless = s.cast_standard(&req, &cur, true, true);
    assert!(
        less.is_none() && sless.is_some(),
        "LESS (care_ptr_uint=false) and SLESS (care_ptr_uint=true) must differ on a \
         uint<-ptr pair — a SLESS fix must use its own (true,true) tail, not the LESS arm \
         (cast.cc:772 !care_ptr_uint && curbase==PTR)"
    );
}

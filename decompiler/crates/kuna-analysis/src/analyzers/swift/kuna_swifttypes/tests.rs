//! (kuna `swifttypes`) The nesting bound on layouts built through other
//! layouts; the end-to-end layouts are `verify_swifttypes`.

use super::*;
use crate::loader::format::macho::ChainedFixups;
use kuna_decomp::dtype::TypeFactoryImpl;
use object::read::{Object, ObjectSection, ObjectSymbol, ObjectSymbolTable};

use super::super::descriptors::ELF_SECTIONS;

fn factory() -> TypeFactoryImpl {
    let types = TypeFactoryImpl::new();
    types.set_default_alignment_map();
    types.set_max_basetype_size(8);
    types.setup_sizes(Some(8), 8, 4);
    types
        .set_core_type("char", 1, type_metatype::TYPE_INT, true)
        .expect("char core type");
    types.cache_core_types().expect("cache core types");
    types
}

/// `(name, type)` of each field of `swift_types_elf`'s `typefix.Sprite`,
/// built with `outer` layouts already in progress, or `None` when Sprite
/// itself is out of reach.
fn sprite_fields(outer: usize) -> Option<Vec<(String, String)>> {
    let bytes = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/swift_types_elf"
    ))
    .unwrap();
    let file = object::File::parse(&*bytes).unwrap();
    let mut imports = HashMap::new();
    let mut bind_slots = HashSet::new();
    for (addr, reloc) in file.dynamic_relocations().into_iter().flatten() {
        if let object::RelocationTarget::Symbol(idx) = reloc.target() {
            let sym = file
                .dynamic_symbol_table()
                .and_then(|t| t.symbol_by_index(idx).ok());
            if let Some(name) = sym.and_then(|s| s.name().ok()) {
                imports.insert(addr, name.to_string());
                bind_slots.insert(addr);
            }
        }
    }
    let fixups = ChainedFixups::from_rebases(super::super::elf_relative_relocs(&file));
    let img = MachoImage::with_fixups(&file, fixups);
    let reader = Reader {
        img: &img,
        bind_slots: &bind_slots,
    };
    let md = reader.read_all(&file, &ELF_SECTIONS, bytes.len());
    let namer = Namer {
        md: &md,
        imports: &imports,
        memo: Default::default(),
        code: file
            .sections()
            .filter(|s| s.kind() == object::SectionKind::Text)
            .map(|s| (s.address(), s.address().saturating_add(s.size())))
            .collect(),
        budget: std::cell::Cell::new(usize::MAX),
    };
    let tf = factory();
    let mut types = Types::new(
        &md,
        &reader,
        &imports,
        &namer,
        &tf,
        file.architecture(),
        false,
    );
    types.locate_all();
    let sprite = *types.by_path.get("typefix.Sprite").unwrap();
    types.in_progress.borrow_mut().extend(1..=outer as u64);
    let sprite = types.struct_type(sprite)?;
    Some(
        (0..)
            .map_while(|i| sprite.get_field(i))
            .map(|f| (f.name.clone(), f.field_type.get_name().to_string()))
            .collect(),
    )
}

#[test]
fn nested_layouts_stop_at_the_nesting_bound() {
    let type_of = |fields: &[(String, String)], name: &str| {
        fields
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, t)| t.clone())
            .unwrap()
    };
    let top = sprite_fields(0).unwrap();
    assert_eq!(type_of(&top, "origin"), "typefix.Point");
    assert_eq!(type_of(&top, "pixel"), "typefix.Pixel");
    // Sprite is the last layout under the bound: its struct fields, one
    // deeper, keep their bytes untyped, and every field keeps its name.
    let cut = sprite_fields(MAX_NESTING - 1).unwrap();
    assert_ne!(type_of(&cut, "origin"), "typefix.Point");
    assert_ne!(type_of(&cut, "pixel"), "typefix.Pixel");
    let names = |f: &[(String, String)]| f.iter().map(|(n, _)| n.clone()).collect::<Vec<_>>();
    assert_eq!(names(&cut), names(&top));
    assert!(sprite_fields(MAX_NESTING).is_none());
}

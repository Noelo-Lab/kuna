//! Complex float/double values use pair layouts on explicitly supported ABIs.
use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory, TypeField};
use std::rc::Rc;

/// Widest complex value, in bytes, this target ABI passes like its pair
/// structure; 0 when no complex value uses the pair layout.
pub(crate) fn max_size(
    format: object::BinaryFormat,
    architecture: object::Architecture,
    archid: &str,
) -> i32 {
    use object::{Architecture as A, BinaryFormat as B};
    match (format, architecture) {
        (B::Elf, A::X86_64) if archid == "x86:LE:64:default:gcc" => 16,
        (B::Pe, A::I386) if archid == "x86:LE:32:default:windows" => 8,
        (B::Pe, A::X86_64) if archid == "x86:LE:64:default:windows" => 16,
        _ => 0,
    }
}

pub(crate) fn complex_type(types: &dyn TypeFactory, size: i32) -> Option<Rc<Datatype>> {
    let name = match size {
        8 => "kuna_complex_float",
        16 => "kuna_complex_double",
        _ => return None,
    };
    let scalar = types.get_base(size / 2, type_metatype::TYPE_FLOAT).ok()?;
    let mut shell = None;
    for suffix in 0..32 {
        let candidate = if suffix == 0 {
            name.to_owned()
        } else {
            format!("{name}_{suffix}")
        };
        match types.find_by_name(&candidate).ok()? {
            Some(existing) if matches_layout(&existing, size) => return Some(existing),
            Some(_) => continue,
            None => {
                shell = Some(types.get_type_struct(&candidate).ok()?);
                break;
            }
        }
    }
    let shell = shell?;
    let fields = vec![
        TypeField::new(0, 0, "real", scalar.clone()),
        TypeField::new(size / 2, size / 2, "imag", scalar),
    ];
    types
        .set_fields_struct_raw(&shell, fields, Vec::new(), size, size / 2, 0)
        .ok()
}

fn matches_layout(ty: &Datatype, size: i32) -> bool {
    ty.get_metatype() == type_metatype::TYPE_STRUCT
        && !ty.is_incomplete()
        && ty.get_size() == size
        && ty.get_alignment() == size / 2
        && ty.get_field(2).is_none()
        && ["real", "imag"].iter().enumerate().all(|(index, name)| {
            ty.get_field(index as i32).is_some_and(|field| {
                field.name == *name
                    && field.offset == index as i32 * (size / 2)
                    && field.field_type.get_size() == size / 2
                    && field.field_type.get_metatype() == type_metatype::TYPE_FLOAT
            })
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_decomp::dtype::TypeFactoryImpl;
    fn factory() -> TypeFactoryImpl {
        let types = TypeFactoryImpl::new();
        types.set_default_alignment_map();
        types.set_max_basetype_size(8);
        types.setup_sizes(Some(8), 8, 4);
        types.cache_core_types().unwrap();
        types
    }
    #[test]
    fn only_verified_target_abis_use_pair_layouts() {
        use object::{Architecture as A, BinaryFormat as B};
        assert_eq!(max_size(B::Elf, A::X86_64, "x86:LE:64:default:gcc"), 16);
        assert_eq!(max_size(B::Pe, A::I386, "x86:LE:32:default:windows"), 8);
        assert_eq!(max_size(B::Pe, A::X86_64, "x86:LE:64:default:windows"), 16);
        assert_eq!(max_size(B::MachO, A::X86_64, "x86:LE:64:default:gcc"), 0);
        assert_eq!(max_size(B::Pe, A::X86_64, "x86:LE:64:default:gcc"), 0);
        assert_eq!(max_size(B::Elf, A::I386, "x86:LE:32:default:gcc"), 0);
        assert_eq!(max_size(B::Elf, A::Aarch64, "AARCH64:LE:64:v8A:default"), 0);
    }
    #[test]
    fn repeated_components_reuse_complete_layouts() {
        let types = factory();
        for size in [8, 16] {
            let first = complex_type(&types, size).unwrap();
            let second = complex_type(&types, size).unwrap();
            assert!(matches_layout(&first, size));
            assert!(Rc::ptr_eq(&first, &second));
        }
        for size in [4, 20, 24, 32] {
            assert!(complex_type(&types, size).is_none());
        }
    }
    #[test]
    fn user_names_are_not_reused_or_completed() {
        let types = factory();
        let user = types.get_type_struct("kuna_complex_float").unwrap();
        let generated = complex_type(&types, 8).unwrap();
        assert_ne!(generated.get_name(), user.get_name());
        assert!(user.is_incomplete());
        let shell = types.get_type_struct("kuna_complex_double").unwrap();
        let integer = types.get_base(8, type_metatype::TYPE_UINT).unwrap();
        let user = types
            .set_fields_struct_raw(
                &shell,
                vec![
                    TypeField::new(0, 0, "real", integer.clone()),
                    TypeField::new(8, 8, "imag", integer),
                ],
                Vec::new(),
                16,
                8,
                0,
            )
            .unwrap();
        let generated = complex_type(&types, 16).unwrap();
        assert_ne!(generated.get_name(), user.get_name());
        assert!(matches_layout(&generated, 16));
    }
}

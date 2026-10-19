//! Overlapping debug locals describe views of one physical frame object.

use kuna_base::error::KunaResult;
use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory, TypeField};
use std::collections::BTreeSet;
use std::rc::Rc;

pub(crate) type Local = (String, Rc<Datatype>, i64);

pub(crate) fn local_name(
    stem: &str,
    mapped: &BTreeSet<String>,
    reserved: &BTreeSet<String>,
) -> String {
    if !mapped.contains(stem) {
        return stem.to_owned();
    }
    for variant in 1u64.. {
        let name = format!("{stem}_{variant}");
        if !mapped.contains(&name) && !reserved.contains(&name) {
            return name;
        }
    }
    unreachable!()
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a.max(1)
}

fn composite(
    factory: &dyn TypeFactory,
    stem: &str,
    fields: Vec<TypeField>,
    size: i32,
    alignment: i32,
    union: bool,
) -> KunaResult<Rc<Datatype>> {
    for variant in 0u64.. {
        let name = format!("{stem}_{variant}");
        if let Some(existing) = factory.find_by_name(&name)? {
            if existing.get_size() == size
                && !existing.is_incomplete()
                && !matches!(&existing.kind, kuna_decomp::dtype::DatatypeKind::Struct { bitfield, .. } if !bitfield.is_empty())
                && existing.get_alignment() == alignment
                && existing.num_depend() as usize == fields.len()
                && fields.iter().enumerate().all(|(i, field)| {
                    existing.get_field(i as i32).is_some_and(|old| {
                        old.offset == field.offset
                            && old.name == field.name
                            && Rc::ptr_eq(&old.field_type, &field.field_type)
                    })
                })
                && existing.get_metatype()
                    == if union {
                        type_metatype::TYPE_UNION
                    } else {
                        type_metatype::TYPE_STRUCT
                    }
            {
                return Ok(existing);
            }
            continue;
        }
        return if union {
            factory.set_fields_union_raw(&factory.get_type_union(&name)?, fields, size, alignment)
        } else {
            factory.set_fields_struct_raw(
                &factory.get_type_struct(&name)?,
                fields,
                Vec::new(),
                size,
                alignment,
                0,
            )
        };
    }
    unreachable!()
}

fn backing(factory: &dyn TypeFactory, function: u64, group: &[Local]) -> KunaResult<Local> {
    let start = group[0].2;
    let size = group
        .iter()
        .map(|(_, ty, off)| i128::from(*off) + i128::from(ty.get_size()))
        .max()
        .unwrap()
        - i128::from(start);
    let size = i32::try_from(size).map_err(|_| {
        kuna_base::error::KunaError::lowlevel("Debug stack overlap exceeds supported object size")
    })?;
    let stem = format!("dwarf_stack_{function:x}_{:x}", start as u64);
    let raw_name = "storage_bytes";
    let mut members = BTreeSet::from([raw_name.to_owned()]);
    let mut fields = Vec::new();
    for (index, (name, ty, off)) in group.iter().enumerate() {
        let offset = i32::try_from(i128::from(*off) - i128::from(start)).map_err(|_| {
            kuna_base::error::KunaError::lowlevel(
                "Debug stack view offset exceeds supported object size",
            )
        })?;
        let view = if offset == 0 {
            Rc::clone(ty)
        } else {
            let extent = offset.checked_add(ty.get_size()).ok_or_else(|| {
                kuna_base::error::KunaError::lowlevel("Debug stack view extent overflow")
            })?;
            composite(
                factory,
                &format!("{stem}_slice_{index}"),
                vec![TypeField::new(0, offset, "value", Rc::clone(ty))],
                extent,
                gcd(gcd(ty.get_alignment(), offset), extent),
                false,
            )?
        };
        let label = if name.is_empty() { "view" } else { name.as_str() };
        let mut member = label.to_owned();
        let mut variant = 0;
        while !members.insert(member.clone()) {
            variant += 1;
            member = format!("{label}_{variant}");
        }
        fields.push(TypeField::new(index as i32, 0, member, view));
    }
    let bytes = factory.get_type_array(size, factory.get_base(1, type_metatype::TYPE_UINT)?)?;
    fields.push(TypeField::new(fields.len() as i32, 0, raw_name, bytes));
    let alignment = gcd(
        fields
            .iter()
            .map(|f| f.field_type.get_alignment())
            .max()
            .unwrap_or(1),
        size,
    );
    let ty = composite(factory, &stem, fields, size, alignment, true)?;
    Ok((format!("shared_{:x}", start as u64), ty, start))
}

/// Views of one type at one offset stay a single local; distinct views become
/// union members named after their locals. Conflicting optional debug locks are
/// omitted if their common extent cannot be represented; the machine's storage
/// recovery remains authoritative.
pub(crate) fn coalesce(
    factory: &dyn TypeFactory,
    function: u64,
    mut locals: Vec<Local>,
    mask: u64,
) -> Vec<Local> {
    locals.retain(|(_, ty, _)| ty.get_size() > 0);
    for (_, _, offset) in &mut locals {
        let raw = *offset as u64 & mask;
        *offset = if raw & ((mask >> 1) + 1) != 0 {
            (raw | !mask) as i64
        } else {
            raw as i64
        };
    }
    let mut wraps: Vec<_> = locals
        .iter()
        .filter(|(_, ty, off)| {
            (*off as u64 & mask)
                .checked_add(ty.get_size() as u64 - 1)
                .is_none_or(|end| end > mask)
                || i128::from(*off) + i128::from(ty.get_size()) - 1 > i128::from((mask >> 1) as i64)
        })
        .map(|(_, ty, off)| (*off as u64 & mask, ty.get_size()))
        .collect();
    loop {
        let before = wraps.len();
        for (_, ty, off) in &locals {
            let range = (*off as u64 & mask, ty.get_size());
            if !wraps.contains(&range)
                && wraps
                    .iter()
                    .any(|(start, size)| byte_overlap(range.0, range.1, *start, *size, mask))
            {
                wraps.push(range);
            }
        }
        if wraps.len() == before {
            break;
        }
    }
    locals.retain(|(_, ty, off)| {
        !wraps.iter().any(|(start, size)| {
            byte_overlap(*off as u64 & mask, ty.get_size(), *start, *size, mask)
        })
    });
    let mut names: BTreeSet<_> = locals.iter().map(|local| local.0.clone()).collect();
    locals.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| a.0.cmp(&b.0)));
    let mut result = Vec::new();
    let mut first = 0;
    while first < locals.len() {
        let mut end = i128::from(locals[first].2) + i128::from(locals[first].1.get_size());
        let mut next = first + 1;
        while next < locals.len() && i128::from(locals[next].2) < end {
            end = end.max(i128::from(locals[next].2) + i128::from(locals[next].1.get_size()));
            next += 1;
        }
        let mut group: Vec<Local> = Vec::new();
        for local in &locals[first..next] {
            if !group
                .iter()
                .any(|(_, ty, off)| *off == local.2 && Rc::ptr_eq(ty, &local.1))
            {
                group.push(local.clone());
            }
        }
        if group.len() == 1 {
            result.extend(group);
        } else if let Ok(mut shared) = backing(factory, function, &group) {
            let stem = shared.0.clone();
            let mut variant = 0;
            while !names.insert(shared.0.clone()) {
                variant += 1;
                shared.0 = format!("{stem}_{variant}");
            }
            result.push(shared);
        }
        first = next;
    }
    result
}

fn byte_overlap(left: u64, left_size: i32, right: u64, right_size: i32, mask: u64) -> bool {
    left_size > 0
        && right_size > 0
        && ((left.wrapping_sub(right) & mask) < right_size as u64
            || (right.wrapping_sub(left) & mask) < left_size as u64)
}

pub(crate) fn overlaps(
    left: &kuna_base::address::Address,
    left_size: i32,
    right: &kuna_base::address::Address,
    right_size: i32,
) -> bool {
    left_size > 0
        && right_size > 0
        && left
            .get_space()
            .zip(right.get_space())
            .is_some_and(|(a, b)| a.get_index() == b.get_index())
        && left.get_space().is_some_and(|space| {
            byte_overlap(
                left.get_offset(),
                left_size,
                right.get_offset(),
                right_size,
                space.wrap_offset(u64::MAX),
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_decomp::dtype::TypeFactoryImpl;

    fn factory() -> TypeFactoryImpl {
        let factory = TypeFactoryImpl::new();
        factory.set_default_alignment_map();
        factory.set_max_basetype_size(8);
        factory
    }

    #[test]
    fn carried_names_cannot_rename_backing_over_a_future_debug_local() {
        let mapped = BTreeSet::from(["dwarf_stack_1000".to_owned()]);
        let reserved = BTreeSet::from(["dwarf_stack_1000_1".to_owned()]);
        assert_eq!(
            local_name("dwarf_stack_1000", &mapped, &reserved),
            "dwarf_stack_1000_2"
        );
        assert_eq!(
            local_name("dwarf_stack_2000", &mapped, &reserved),
            "dwarf_stack_2000"
        );
    }

    #[test]
    fn overlapping_views_share_one_exact_extent_and_reuse_identity() {
        let f = factory();
        let integer = f.get_base(8, type_metatype::TYPE_INT).unwrap();
        let real = f.get_base(4, type_metatype::TYPE_FLOAT).unwrap();
        let locals = vec![("bits".into(), integer, -32), ("alias".into(), real, -32)];
        let first = coalesce(&f, 0x1000, locals.clone(), u64::MAX);
        assert_eq!(first.len(), 1);
        assert_eq!(first[0].2, -32);
        assert_eq!(first[0].1.get_size(), 8);
        assert_eq!(first[0].1.get_metatype(), type_metatype::TYPE_UNION);
        assert!(Rc::ptr_eq(
            &first[0].1,
            &coalesce(&f, 0x1000, locals, u64::MAX)[0].1
        ));
    }

    #[test]
    fn reused_slots_of_one_type_stay_one_local_and_views_keep_debug_names() {
        let f = factory();
        let word = f.get_base(4, type_metatype::TYPE_UINT).unwrap();
        let real = f.get_base(4, type_metatype::TYPE_FLOAT).unwrap();
        let mapped = coalesce(
            &f,
            0x1000,
            vec![
                ("rsa_n".into(), Rc::clone(&word), -32),
                ("dsa_p".into(), Rc::clone(&word), -32),
                ("kx".into(), Rc::clone(&word), -16),
                ("storage_bytes".into(), real, -16),
                ("err".into(), word, -16),
            ],
            u64::MAX,
        );
        assert_eq!(mapped.len(), 2);
        assert_eq!(mapped[0].0, "dsa_p");
        assert_eq!(mapped[0].1.get_metatype(), type_metatype::TYPE_UINT);
        let union = &mapped[1].1;
        assert_eq!(union.get_metatype(), type_metatype::TYPE_UNION);
        let members: Vec<_> = (0..union.num_depend() as i32)
            .map(|index| union.get_field(index).unwrap().name.clone())
            .collect();
        assert_eq!(members, ["err", "storage_bytes_1", "storage_bytes"]);
    }

    #[test]
    fn transitive_interior_overlap_keeps_adjacent_objects_separate() {
        let f = factory();
        let word = f.get_base(4, type_metatype::TYPE_UINT).unwrap();
        let locals = vec![
            ("a".into(), Rc::clone(&word), -32),
            ("b".into(), Rc::clone(&word), -30),
            ("c".into(), Rc::clone(&word), -28),
            ("next".into(), word, -24),
        ];
        let mapped = coalesce(&f, 0x1000, locals, u64::MAX);
        assert_eq!(mapped.len(), 2);
        assert_eq!(mapped[0].1.get_size(), 8);
        assert_eq!(mapped[1].0, "next");
        let slice = &mapped[0].1.get_field(1).unwrap().field_type;
        assert_eq!(slice.get_field(0).unwrap().offset, 2);
        assert_eq!(slice.get_size(), 6);
        assert_eq!(slice.get_alignment(), 2);
    }

    #[test]
    fn a_user_type_with_the_synthetic_name_is_never_overwritten() {
        let f = factory();
        let held = f
            .get_type_struct("dwarf_stack_1000_ffffffffffffffe0_0")
            .unwrap();
        let word = f.get_base(4, type_metatype::TYPE_UINT).unwrap();
        let real = f.get_base(4, type_metatype::TYPE_FLOAT).unwrap();
        let mapped = coalesce(
            &f,
            0x1000,
            vec![("a".into(), word, -32), ("b".into(), real, -32)],
            u64::MAX,
        );
        assert_eq!(held.get_size(), 0);
        assert_eq!(
            mapped[0].1.get_name(),
            "dwarf_stack_1000_ffffffffffffffe0_1"
        );
    }

    #[test]
    fn generated_local_names_do_not_shadow_neighboring_debug_locals() {
        let f = factory();
        let word = f.get_base(4, type_metatype::TYPE_UINT).unwrap();
        let real = f.get_base(4, type_metatype::TYPE_FLOAT).unwrap();
        let mapped = coalesce(
            &f,
            0x1000,
            vec![
                ("a".into(), Rc::clone(&word), -32),
                ("b".into(), real, -32),
                ("shared_ffffffffffffffe0".into(), word, -16),
            ],
            u64::MAX,
        );
        assert_eq!(mapped[0].0, "shared_ffffffffffffffe0_1");
        assert_eq!(mapped[1].0, "shared_ffffffffffffffe0");
    }

    #[test]
    fn stack_width_normalizes_aliases_and_declines_wrapping_components() {
        let f = factory();
        let word = f.get_base(4, type_metatype::TYPE_UINT).unwrap();
        let real = f.get_base(4, type_metatype::TYPE_FLOAT).unwrap();
        let mapped = coalesce(
            &f,
            0x1000,
            vec![
                ("a".into(), Rc::clone(&word), -32),
                ("b".into(), real, 0xffffffe0),
                ("wrap".into(), Rc::clone(&word), -2),
                ("zero".into(), Rc::clone(&word), 0),
                ("chain".into(), word, 2),
            ],
            0xffffffff,
        );
        assert_eq!(mapped.len(), 1);
        assert_eq!(mapped[0].2, -32);
        assert_eq!(mapped[0].1.get_metatype(), type_metatype::TYPE_UNION);
        assert!(byte_overlap(0xffffffff, 2, 0, 4, 0xffffffff));
        assert!(!byte_overlap(0xfffffffc, 4, 0, 4, 0xffffffff));
    }

    #[test]
    fn a_slice_with_extra_bitfields_is_not_the_same_layout() {
        use kuna_decomp::dtype::TypeBitField;
        let f = factory();
        let word = f.get_base(4, type_metatype::TYPE_UINT).unwrap();
        let name = "dwarf_stack_1000_ffffffffffffffe0_slice_1_0";
        let user = f
            .set_fields_struct_raw(
                &f.get_type_struct(name).unwrap(),
                vec![TypeField::new(0, 2, "value", Rc::clone(&word))],
                vec![TypeBitField::new(1, 1, false, "extra", Rc::clone(&word))],
                6,
                2,
                0,
            )
            .unwrap();
        let mapped = coalesce(
            &f,
            0x1000,
            vec![("a".into(), Rc::clone(&word), -32), ("b".into(), word, -30)],
            u64::MAX,
        );
        let slice = &mapped[0].1.get_field(1).unwrap().field_type;
        assert!(!Rc::ptr_eq(slice, &user));
        assert!(slice.get_name().ends_with("slice_1_1"));
    }
}

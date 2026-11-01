//! Map Go field types with bounded pointer expansion, including recursive aliases.

use super::*;

const MAX_POINTER_DEPTH: usize = 64;

pub(super) fn field_type(
    tf: &dyn TypeFactory,
    img: &GoImage,
    meta: &TypeMeta,
    mut vma: u64,
) -> Option<Rc<Datatype>> {
    let mut pointers = Vec::new();
    let mut result = loop {
        let offset = vma
            .checked_sub(meta.types)
            .and_then(|n| u32::try_from(n).ok());
        let Some(t) = offset.and_then(|off| read_type(img, meta.types, meta.etypes, off)) else {
            break None;
        };
        if t.kind != KIND_POINTER {
            break leaf_type(tf, &t);
        }
        if pointers.len() == MAX_POINTER_DEPTH || pointers.contains(&vma) {
            break None;
        }
        pointers.push(vma);
        let Some(next) = word(img, vma + img.type_base()) else {
            break None;
        };
        vma = next;
    };
    for _ in pointers {
        let pointee = result.unwrap_or_else(|| tf.get_type_void().expect("void type"));
        result = tf.get_type_pointer(img.word_size as int4, pointee, 1).ok();
    }
    result
}

fn leaf_type(tf: &dyn TypeFactory, t: &GoType) -> Option<Rc<Datatype>> {
    let base = |mt| tf.get_base(t.size as int4, mt).ok();
    match t.kind {
        KIND_BOOL => tf.get_base(1, type_metatype::TYPE_BOOL).ok(),
        KIND_INT | KIND_INT8 | KIND_INT16 | KIND_INT32 | KIND_INT64 => {
            base(type_metatype::TYPE_INT)
        }
        KIND_UINT | KIND_UINT8 | KIND_UINT16 | KIND_UINT32 | KIND_UINT64 | KIND_UINTPTR => {
            base(type_metatype::TYPE_UINT)
        }
        KIND_FLOAT32 | KIND_FLOAT64 => base(type_metatype::TYPE_FLOAT),
        KIND_STRING => shell(tf, "go.string").ok(),
        KIND_SLICE => shell(tf, "go.slice").ok(),
        KIND_INTERFACE => shell(
            tf,
            if t.name == "interface {}" {
                "go.eface"
            } else {
                "go.iface"
            },
        )
        .ok(),
        KIND_STRUCT if t.tflag & TFLAG_NAMED != 0 && t.name.contains('.') => {
            shell(tf, &t.name).ok()
        }
        _ => base(type_metatype::TYPE_UNKNOWN),
    }
}

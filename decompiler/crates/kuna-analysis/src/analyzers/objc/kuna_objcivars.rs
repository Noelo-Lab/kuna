//! Objective-C instance layouts: a struct per class from its `ivar_list_t`, so a
//! method's `self` is a typed pointer and an ivar access renders as a field.
//!
//! `class_ro_t.ivars` names every ivar with its type encoding, its size and a
//! pointer to its **offset variable** in `__objc_ivar` — the word the compiled
//! method loads and adds to `self` (`ldrsw x8, [x8, #off]; ldr d0, [x19, x8]`),
//! because under the non-fragile ABI the runtime may slide those offsets when
//! a superclass from another image grows. Statically the word holds the
//! compile-time layout, the one this struct describes, so with the slot folded
//! (`objcivars`) `*(double *)((long)self + Foo::ratio)` becomes `self->ratio`.
//!
//! The struct named after the class carries the class's own ivars at their
//! offsets, with a local superclass embedded as `super` at 0 so a parent's field
//! keeps its name (`self->super.count`); an imported superclass's bytes stay an
//! anonymous prefix. Field types come from the encoding (`@"NSString"` is an
//! opaque `NSString *`), falling back to an `undefined<size>` when it does not
//! decode. A class whose name already denotes a complete type (a DWARF struct, a
//! user `typedef`) is left as it is.

use std::collections::HashMap;
use std::rc::Rc;

use kuna_decomp::dtype::{type_metatype, Datatype, TypeField};

use super::classt::{self, Ivar};
use super::encoding::{self, TypeCtx};
use super::sections::MachoImage;

/// One class's instance layout as read off the metadata.
pub struct Layout {
    pub name: String,
    pub superclass: Option<String>,
    pub instance_start: u32,
    pub instance_size: u32,
    pub ivars: Vec<Ivar>,
}

/// Read the layout of the `class_t` at `class_addr` named `name`; `None` when the
/// `class_ro_t` extent is unreadable.
pub fn layout(img: &MachoImage, class: &classt::ClassT, name: &str) -> Option<Layout> {
    let (instance_start, instance_size) = classt::instance_extent(img, class.ro)?;
    let ivars = classt::base_ivars(img, class.ro).map(|l| classt::walk_ivar_list(img, l)).unwrap_or_default();
    Some(Layout {
        name: name.to_string(),
        superclass: classt::superclass_name(img, class.addr),
        instance_start,
        instance_size,
        ivars,
    })
}

/// The struct types of every layout, keyed by class name, built superclass
/// first so a local parent embeds as `super`.
pub struct Structs {
    built: HashMap<String, Rc<Datatype>>,
}

impl Structs {
    pub(super) fn build(tctx: &TypeCtx, layouts: &HashMap<String, Layout>) -> Structs {
        let mut structs = Structs { built: HashMap::new() };
        for name in layouts.keys() {
            structs.build_one(tctx, layouts, name, 0);
        }
        structs
    }

    /// `<Class> *`, the type of `self` in the class's methods, if the class got a struct.
    pub(super) fn self_type(&self, tctx: &TypeCtx, class: &str) -> Option<Rc<Datatype>> {
        let st = self.built.get(class)?;
        tctx.types.get_type_pointer(tctx.ptr_size, Rc::clone(st), tctx.word_size).ok()
    }

    fn build_one(
        &mut self,
        tctx: &TypeCtx,
        layouts: &HashMap<String, Layout>,
        name: &str,
        depth: usize,
    ) -> Option<Rc<Datatype>> {
        if let Some(t) = self.built.get(name) {
            return Some(Rc::clone(t));
        }
        let layout = layouts.get(name)?;
        if depth > 64 || layout.instance_size == 0 || layout.instance_size > 1 << 20 {
            return None;
        }
        let types = tctx.types;
        let existing = types.find_by_name(name).ok().flatten();
        let shell = match existing {
            Some(t) if t.get_metatype() == type_metatype::TYPE_STRUCT && t.is_incomplete() => t,
            Some(_) => return None,
            None => types.get_type_struct(name).ok()?,
        };
        // Interned before the fields so a self-referential ivar (`Node *next`) can
        // name it; an incomplete shell is still a usable pointee.
        self.built.insert(name.to_string(), Rc::clone(&shell));

        let mut fields: Vec<TypeField> = Vec::new();
        let sup = layout
            .superclass
            .as_deref()
            .and_then(|s| self.build_one(tctx, layouts, s, depth + 1));
        if let Some(sup) = sup {
            if sup.get_size() > 0 && sup.get_size() as u32 <= layout.instance_start.max(sup.get_size() as u32) {
                fields.push(TypeField::new(0, 0, "super".to_string(), sup));
            }
        }
        let mut end = fields.first().map(|f| f.field_type.get_size() as u32).unwrap_or(0);
        let mut ivars: Vec<&Ivar> = layout.ivars.iter().collect();
        ivars.sort_by_key(|iv| iv.offset);
        for iv in ivars {
            if iv.offset < end || iv.offset >= layout.instance_size {
                continue;
            }
            let ty = iv
                .types
                .as_deref()
                .and_then(|e| encoding::decode_ivar_type(e, tctx))
                .filter(|t| t.get_size() > 0 && t.get_metatype() != type_metatype::TYPE_VOID)
                .or_else(|| types.get_base(iv.size.clamp(1, 8) as i32, type_metatype::TYPE_UNKNOWN).ok())?;
            if iv.offset as u64 + ty.get_size() as u64 > layout.instance_size as u64 {
                continue;
            }
            end = iv.offset + ty.get_size() as u32;
            fields.push(TypeField::new(iv.offset as i32, iv.offset as i32, iv.name.clone(), ty));
        }
        if fields.is_empty() {
            self.built.remove(name);
            return None;
        }
        let size = layout.instance_size as i32;
        let align = (tctx.ptr_size).min(size).max(1);
        match types.set_fields_struct_raw(&shell, fields, Vec::new(), size, align, 0) {
            Ok(done) => {
                self.built.insert(name.to_string(), Rc::clone(&done));
                Some(done)
            }
            Err(_) => {
                self.built.remove(name);
                None
            }
        }
    }
}

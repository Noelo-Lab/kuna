//! Objective-C categories: `__objc_catlist` / `__objc_nlcatlist` and the
//! `category_t` records they point at.
//!
//! A category adds methods to a class defined elsewhere, often in another image
//! (`UIView(YYAdd)`), so its IMPs reach no `class_t` of this image and the class
//! walk never sees them: on an ObjC-heavy iOS app they are the bulk of the
//! `sub_<addr>` functions left after `objc`. Each record is parsed to its name,
//! class (a rebase slot's `class_t`, or an imported class's bind name) and the
//! two method lists; the caller renames the IMPs `-[Class(Category) sel]` through
//! the same path as a class's own methods. The property lists type the
//! getters and setters (`kuna_objcprops`); the protocol list is not read, since
//! nothing in the emitted C would show it.

use object::read::{Object, ObjectSection};

use super::classt;
use super::sections::MachoImage;

const SEC_CATLIST: &str = "__objc_catlist";
const SEC_NLCATLIST: &str = "__objc_nlcatlist";
const CLASS_SYMBOL_PREFIX: &str = "_OBJC_CLASS_$_";

/// `category_t` field offsets (64-bit): `name`, `cls`, `instanceMethods`,
/// `classMethods`, `protocols`, `instanceProperties`.
mod category_t {
    pub const NAME: u64 = 0x00;
    pub const CLS: u64 = 0x08;
    pub const INSTANCE_METHODS: u64 = 0x10;
    pub const CLASS_METHODS: u64 = 0x18;
    pub const INSTANCE_PROPERTIES: u64 = 0x28;
    pub const CLASS_PROPERTIES: u64 = 0x30;
}

/// One category: the label stem `Class(Category)` and its method lists.
#[derive(Debug)]
pub struct Category {
    /// `Class(Category)`, the text between the brackets of `-[Class(Category) sel]`;
    /// `(Category)` alone when the class slot resolves to nothing.
    pub label: String,
    /// The extended class, when the slot resolved.
    pub class: Option<String>,
    /// `method_list_t*` of the instance methods, `None` when absent.
    pub instance_methods: Option<u64>,
    /// `method_list_t*` of the class methods, `None` when absent.
    pub class_methods: Option<u64>,
    /// `property_list_t*` of the instance and class properties, `None` when absent.
    pub instance_properties: Option<u64>,
    pub class_properties: Option<u64>,
}

/// Every category of `file`, in `__objc_catlist` then `__objc_nlcatlist` order.
pub fn categories(img: &MachoImage, file: &object::File) -> Vec<Category> {
    let ps = img.ptr_size();
    let mut out = Vec::new();
    for section in [SEC_CATLIST, SEC_NLCATLIST] {
        let Some(sec) = file.section_by_name(section) else { continue };
        for i in 0..sec.size() / ps {
            let Some(cat) = img.rebase_ptr(sec.address().wrapping_add(i * ps)) else { continue };
            let Some(name) = img.rebase_ptr(cat.wrapping_add(category_t::NAME)).and_then(|p| img.read_cstr(p)) else {
                continue;
            };
            if name.is_empty() {
                continue;
            }
            let class = match img.rebase_ptr(cat.wrapping_add(category_t::CLS)) {
                Some(class_va) => {
                    classt::read_class_t(img, class_va).and_then(|c| classt::class_name(img, c.ro))
                }
                None => img
                    .bind_name(cat.wrapping_add(category_t::CLS))
                    .and_then(|n| n.strip_prefix(CLASS_SYMBOL_PREFIX))
                    .map(str::to_string),
            };
            let label = match &class {
                Some(class) => format!("{class}({name})"),
                None => format!("({name})"),
            };
            out.push(Category {
                label,
                class,
                instance_methods: img.rebase_ptr(cat.wrapping_add(category_t::INSTANCE_METHODS)),
                class_methods: img.rebase_ptr(cat.wrapping_add(category_t::CLASS_METHODS)),
                instance_properties: img.rebase_ptr(cat.wrapping_add(category_t::INSTANCE_PROPERTIES)),
                class_properties: img.rebase_ptr(cat.wrapping_add(category_t::CLASS_PROPERTIES)),
            });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader::format::macho::{resolve_chained_fixups, resolve_dyld_info_binds};

    /// `macho_objc_meta_arm64`: `Base(Extra)` on a local class, `Remote(Extra)` on
    /// an imported (bind) class, each with one instance and one class method.
    #[test]
    fn walks_macho_objc_cats_fixture() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/macho_objc_meta_arm64");
        let bytes = std::fs::read(path).expect("read fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse Mach-O");
        let fixups = resolve_chained_fixups(&file, &bytes);
        let img = MachoImage::with_fixups(&file, fixups).with_binds(resolve_dyld_info_binds(&file, &bytes));
        let cats = categories(&img, &file);
        let labels: Vec<&str> = cats.iter().map(|c| c.label.as_str()).collect();
        assert_eq!(labels, ["Base(Extra)", "Remote(Extra)"]);
        assert!(cats.iter().all(|c| c.instance_methods.is_some() && c.class_methods.is_some()));
    }
}

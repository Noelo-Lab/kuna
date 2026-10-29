//! Objective-C protocols: `__objc_protolist`, the `protocol_t` records and the
//! `__objc_protorefs` slots.
//!
//! A protocol declares selectors with their type encodings but no IMP, which is
//! exactly the knowledge the selector stubs lack for a method this image only
//! *calls*: `UITableViewDelegate` redeclared here spells the signature of every
//! `tableView:didSelectRowAtIndexPath:` send. Each protocol is parsed to its
//! name and four method lists; the caller records the encodings and names the
//! `protocol_t` `_OBJC_PROTOCOL_$_<name>`, the `__objc_protorefs` slot
//! `protoRef_<name>`.

use object::read::{Object, ObjectSection};

use super::methods::{self, Method};
use super::sections::MachoImage;

const SEC_PROTOLIST: &str = "__objc_protolist";

/// `protocol_t` field offsets (64-bit): `isa`, `mangledName`, `protocols`,
/// `instanceMethods`, `classMethods`, `optionalInstanceMethods`,
/// `optionalClassMethods`, `instanceProperties`, ….
mod protocol_t {
    pub const NAME: u64 = 0x08;
    pub const METHOD_LISTS: [u64; 4] = [0x18, 0x20, 0x28, 0x30];
    pub const INSTANCE_PROPERTIES: u64 = 0x38;
}

/// One protocol: where it lives, what it is called, what it declares.
#[derive(Debug)]
pub struct Protocol {
    pub addr: u64,
    pub name: String,
    /// Every declared method (required and optional, instance and class), IMP 0.
    pub methods: Vec<Method>,
    /// The declared properties' getters and setters with their encodings.
    pub properties: Vec<(String, String)>,
}

/// Every protocol of `file`, in `__objc_protolist` order.
pub fn protocols(img: &MachoImage, file: &object::File) -> Vec<Protocol> {
    let ps = img.ptr_size();
    let Some(sec) = file.section_by_name(SEC_PROTOLIST) else { return Vec::new() };
    let mut out = Vec::new();
    for i in 0..sec.size() / ps {
        let Some(addr) = img.rebase_ptr(sec.address().wrapping_add(i * ps)) else { continue };
        let Some(name) = protocol_name(img, addr) else { continue };
        let mut methods = Vec::new();
        for off in protocol_t::METHOD_LISTS {
            if let Some(list) = img.rebase_ptr(addr.wrapping_add(off)) {
                methods.extend(methods::walk_method_list(img, list, file.architecture()));
            }
        }
        let properties = img
            .rebase_ptr(addr + protocol_t::INSTANCE_PROPERTIES)
            .map(|list| super::kuna_objcprops::declarations(img, list))
            .unwrap_or_default();
        out.push(Protocol { addr, name, methods, properties });
    }
    out
}

/// The name of the `protocol_t` at `addr`, if readable and non-empty.
pub fn protocol_name(img: &MachoImage, addr: u64) -> Option<String> {
    img.rebase_ptr(addr.wrapping_add(protocol_t::NAME)).and_then(|p| img.read_cstr(p)).filter(|n| !n.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader::format::macho::resolve_chained_fixups;

    /// `macho_objc_meta_arm64` declares `Greeter` with a required `greet:` and an
    /// optional `wave`, both IMP-less and typed.
    #[test]
    fn walks_macho_objc_meta_protocols() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/macho_objc_meta_arm64");
        let bytes = std::fs::read(path).expect("read fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse Mach-O");
        let img = MachoImage::with_fixups(&file, resolve_chained_fixups(&file, &bytes));
        let protos = protocols(&img, &file);
        assert_eq!(protos.len(), 1);
        assert_eq!(protos[0].name, "Greeter");
        let decls: Vec<(&str, Option<&str>, u64)> =
            protos[0].methods.iter().map(|m| (m.selector.as_str(), m.types.as_deref(), m.imp)).collect();
        assert_eq!(decls, [("greet:", Some("i20@0:8i16"), 0), ("wave", Some("v16@0:8"), 0)]);
    }
}

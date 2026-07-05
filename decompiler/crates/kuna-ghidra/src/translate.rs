//! The tspec \<sleigh> element decode — the address-space subset of
//! `decompiler/cpp/ghidra_translate.cc` (`GhidraTranslate::initialize` /
//! `decode`, ghidra_translate.cc:35-43,161-176).
//!
//! In ghidra mode no `.sla` file exists: the entire "language" the native
//! side knows is the stripped-down `<sleigh>` document the Java client
//! sends at registerProgram (produced by `SleighLanguage.encodeTranslator`,
//! SleighLanguage.java) — endianness, the unique base, and the address
//! space list whose per-space `index` attributes are the Java
//! `AddressSpace.getUnique()` values.  Those indices are the deepest
//! interop invariant of the packed protocol: `PackedDecode::read_space` /
//! `PackedEncode::write_space` exchange spaces by exactly these numbers, so
//! parsing the tspec into a real [`AddrSpaceManager`] (sparse
//! `insert_space`, kuna-base/src/space.rs) is what gives decompileAt a
//! working \<addr> decoder.
//!
//! The C++ `decodeSpaces` loop cannot be called directly in Rust (the
//! decoder borrows the manager the loop mutates); the sanctioned stepwise
//! pattern — a fresh decoder per child element around each
//! `decode_space`/`insert_space` pair — comes from the kuna-sleigh space
//! manager test (kuna-sleigh/src/translate.rs:1086-1124).
//!
//! Phase-1 scope: the query-side register/user-op caches and
//! `oneInstruction` (the getPcode path) of C++ GhidraTranslate are the
//! phase-2 engine bridge; see `docs/ghidra-integration.md`.

use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{Decoder, IdRegistry, XmlDecode, ATTRIB_BIGENDIAN};
use kuna_base::space::{AddrSpaceManager, ConstantSpace, ATTRIB_DEFAULTSPACE};
use kuna_base::xml::xml_tree;
use kuna_sleigh::translate::{register_translate_ids, TruncationTag, ATTRIB_UNIQBASE, ELEM_SLEIGH};

use crate::ids::register_ghidra_ids;

/// Build the marshaling [`IdRegistry`] a ghidra-mode session needs: the
/// kuna-base tables, the translate.cc ids (the tspec vocabulary), and the
/// ghidra protocol ids (the same per-module registration pattern as
/// `kuna-console`'s `build_registry`).
pub fn build_registry() -> IdRegistry {
    let mut registry = IdRegistry::with_base_ids();
    register_translate_ids(&mut registry);
    register_ghidra_ids(&mut registry);
    registry
}

/// The decoded tspec: what C++ `GhidraTranslate` caches after `initialize`
/// (ghidra_translate.hh:36-57) — minus the query-backed register/user-op
/// caches (phase 2).
#[derive(Debug)]
pub struct GhidraTranslate {
    /// The address-space model, indices bound to the Java
    /// `AddressSpace.getUnique()` numbering.
    pub manager: AddrSpaceManager,
    /// The processor endianness (C++ `Translate::setBigEndian`).
    pub big_endian: bool,
    /// The starting offset for temporaries in the unique space (C++
    /// `Translate::setUniqueBase`; Java sends
    /// `UniqueLayout.SLEIGH_BASE.getOffset(language)`).
    pub unique_base: u64,
}

impl GhidraTranslate {
    /// Parse the \<sleigh> tspec document (C++ `GhidraTranslate::decode`,
    /// ghidra_translate.cc:161-176):
    ///
    /// ```text
    ///   <sleigh bigendian=... uniqbase=...>
    ///     <spaces defaultspace=...>
    ///       <space|space_unique|space_other|space_overlay .../> ...
    ///     </spaces>
    ///     <truncate_space space=... size=.../> ...
    ///   </sleigh>
    /// ```
    pub fn decode(tspec: &[u8], registry: &IdRegistry) -> KunaResult<GhidraTranslate> {
        let document = xml_tree(tspec)?;
        let root = Rc::clone(document.get_root());
        if root.get_name() != "sleigh" {
            return Err(KunaError::lowlevel(format!(
                "Could not find sleigh tag (got <{}>)",
                root.get_name()
            )));
        }

        let mut manager = AddrSpaceManager::new();

        // openElement(ELEM_SLEIGH); readBool(ATTRIB_BIGENDIAN);
        // readUnsignedInteger(ATTRIB_UNIQBASE)   (ghidra_translate.cc:164-166)
        let (big_endian, unique_base) = {
            let mut dec = XmlDecode::new_with_root(&manager, registry, &root, 0);
            dec.open_element_id(&ELEM_SLEIGH)?;
            let be = dec.read_bool_id(&ATTRIB_BIGENDIAN)?;
            let ub = dec.read_unsigned_integer_id(&ATTRIB_UNIQBASE)?;
            (be, ub)
        };

        // decodeSpaces (translate.cc): constant space first, then one child
        // element per space, then the defaultspace resolution.  Driven
        // stepwise (fresh decoder per child) per the kuna decode_spaces
        // aliasing note.
        manager.insert_space(Rc::new(ConstantSpace::new()))?;
        let spaces_el = root
            .get_children()
            .iter()
            .find(|c| c.get_name() == "spaces")
            .cloned()
            .ok_or_else(|| KunaError::lowlevel("Missing <spaces> in <sleigh> tag"))?;
        let defname = {
            let mut dec = XmlDecode::new_with_root(&manager, registry, &spaces_el, 0);
            dec.open_element()?;
            String::from_utf8_lossy(&dec.read_string_id(&ATTRIB_DEFAULTSPACE)?).into_owned()
        };
        for child in spaces_el.get_children() {
            let spc = {
                let mut dec = XmlDecode::new_with_root(&manager, registry, child, 0);
                manager.decode_space(&mut dec)?
            };
            manager.insert_space(spc)?;
        }
        let def_index = match manager.get_space_by_name(&defname) {
            Some(spc) => spc.get_index(),
            None => {
                return Err(KunaError::lowlevel(format!(
                    "Bad 'defaultspace' attribute: {defname}"
                )))
            }
        };
        manager.set_default_code_space(def_index)?;

        // for(;;) { peek ELEM_TRUNCATE_SPACE; TruncationTag::decode;
        // truncateSpace(tag); }  (ghidra_translate.cc:168-174)
        for child in root.get_children() {
            if child.get_name() != "truncate_space" {
                continue;
            }
            let mut tag = TruncationTag::default();
            {
                let mut dec = XmlDecode::new_with_root(&manager, registry, child, 0);
                tag.decode(&mut dec)?;
            }
            manager.truncate_space(tag.get_name(), tag.get_size())?;
        }

        Ok(GhidraTranslate {
            manager,
            big_endian,
            unique_base,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A realistic tspec, shaped exactly like SleighLanguage.encodeTranslator
    /// output (single line, XmlEncode(false)): OTHER at unique index 1
    /// (required by insertSpace), then unique/ram/register.
    pub const TEST_TSPEC: &[u8] = b"<sleigh bigendian=\"false\" uniqbase=\"0x10000000\">\
<spaces defaultspace=\"ram\">\
<space_other name=\"OTHER\" index=\"1\" size=\"8\" bigendian=\"false\" delay=\"0\" physical=\"true\"/>\
<space_unique name=\"unique\" index=\"2\" size=\"4\" bigendian=\"false\" delay=\"0\" physical=\"true\"/>\
<space name=\"ram\" index=\"3\" size=\"8\" bigendian=\"false\" delay=\"1\" physical=\"true\"/>\
<space name=\"register\" index=\"4\" size=\"4\" bigendian=\"false\" delay=\"0\" physical=\"true\"/>\
</spaces></sleigh>";

    #[test]
    fn test_decode_tspec() {
        let registry = build_registry();
        let tr = GhidraTranslate::decode(TEST_TSPEC, &registry).unwrap();
        assert!(!tr.big_endian);
        assert_eq!(tr.unique_base, 0x10000000);
        let ram = tr.manager.get_space_by_name("ram").expect("ram space");
        assert_eq!(ram.get_index(), 3);
        assert_eq!(ram.get_addr_size(), 8);
        let reg = tr
            .manager
            .get_space_by_name("register")
            .expect("register space");
        assert_eq!(reg.get_index(), 4);
        let unique = tr
            .manager
            .get_space_by_name("unique")
            .expect("unique space");
        assert_eq!(unique.get_index(), 2);
        // defaultspace resolution
        let def = tr.manager.get_default_code_space().expect("default space");
        assert_eq!(def.get_name(), "ram");
        // constant space auto-inserted at index 0
        let cspace = tr.manager.get_constant_space().expect("constant space");
        assert_eq!(cspace.get_index(), 0);
    }

    #[test]
    fn test_decode_tspec_truncate_space() {
        let registry = build_registry();
        let tspec = b"<sleigh bigendian=\"true\" uniqbase=\"0x100\">\
<spaces defaultspace=\"ram\">\
<space name=\"ram\" index=\"1\" size=\"8\" bigendian=\"true\" delay=\"1\" physical=\"true\"/>\
</spaces><truncate_space space=\"ram\" size=\"4\"/></sleigh>";
        let tr = GhidraTranslate::decode(tspec, &registry).unwrap();
        assert!(tr.big_endian);
        let ram = tr.manager.get_space_by_name("ram").unwrap();
        assert_eq!(ram.get_addr_size(), 4);
    }

    #[test]
    fn test_decode_tspec_rejects_non_sleigh_root() {
        let registry = build_registry();
        assert!(GhidraTranslate::decode(b"<x/>", &registry).is_err());
    }
}

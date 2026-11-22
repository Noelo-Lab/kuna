//! Mach-O `__objc_*` section location + a VMA→bytes reader.
//!
//! The Objective-C 2.0 metadata graph is a web of absolute pointers between the
//! `__objc_*` sections (`__objc_classlist` → `__objc_data` `class_t` →
//! `__objc_const` `class_ro_t` → method/selector strings in `__TEXT`). Walking it
//! means dereferencing a VMA into whichever section covers it, so this module
//! builds a flat `[ (lo, hi, &bytes) ]` map of every *mapped* section and reads
//! little/big-endian scalars + C strings off it (the kuna analog of Ghidra's
//! `Memory`/`MemBuffer` reads in `Objc2*`).
//!
//! It is the port of the section-locating half of Ghidra's `Objc2Constants` /
//! `ObjcTypeMetadataAnalyzer.checkExecutableFormat` (the `getSection("__objc_*")`
//! lookups). Pure & total: a non-Mach-O / no-`__objc_*` input yields an empty
//! [`MachoImage::classlist`], so the pass is an additive no-op.

use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

use object::read::{Object, ObjectSection};
use object::{Endianness, SectionKind};

use crate::loader::format::macho::ChainedFixups;

/// The `__objc_classlist` section name (the array of `class_t*` the walk starts
/// from). Lives in `__DATA_CONST,__objc_classlist` on a modern toolchain,
/// `__DATA,__objc_classlist` historically — `object` exposes the bare section
/// short-name, so we match either.
pub const SEC_CLASSLIST: &str = "__objc_classlist";

/// What one method/ivar/property record costs against the walk budget: a
/// record's own reads plus the symbols and strings the pass keeps for it.
pub const RECORD_WORK: u64 = 64;

/// A read-only view over a Mach-O image's mapped sections, keyed by VMA range, so
/// an absolute pointer in the ObjC metadata can be dereferenced to bytes.
pub struct MachoImage<'a> {
    /// `(vma_lo, vma_hi, data)` for every mapped, non-zero-length section, sorted
    /// by `vma_lo`. `data` is the section's file bytes (a zero-fill/`.bss` section
    /// is skipped — it carries no metadata pointers).
    spans: Vec<(u64, u64, &'a [u8])>,
    /// The image endianness (`__objc_*` scalars are read with it).
    endian: Endianness,
    /// Pointer size in bytes (8 for the 64-bit Mach-O this pass targets; 4 for a
    /// 32-bit image).
    ptr_size: u64,
    /// The resolved `LC_DYLD_CHAINED_FIXUPS` overlay (PR-O0). On a chained-fixup
    /// arm64(e) Mach-O the raw section word at a pointer slot is a packed fixup
    /// entry, not the real address, so [`Self::read_ptr`] consults this overlay
    /// **first** and only falls back to the raw section word when the slot was not
    /// a rebase fixup. Empty (the no-op overlay) on a classic x86-64
    /// `LC_DYLD_INFO` Mach-O — `read_ptr` then reads raw section bytes exactly as
    /// before this resolver existed.
    fixups: ChainedFixups,
    /// `slot_vma → import symbol` from a classic `LC_DYLD_INFO` bind program, the
    /// pre-chained-fixup twin of [`ChainedFixups::bind_name`]. Empty on a chained
    /// image.
    binds: HashMap<u64, Rc<str>>,
    /// Up to `n` bytes at an address outside the image's own sections: the dyld
    /// shared cache regions an image's selector references, class references
    /// and method names point into. `None` on a standalone image.
    outside: Option<Rc<dyn Fn(u64, usize) -> Option<Vec<u8>> + 'a>>,
    /// The selector base a direct-selector method list (`0x40000000`) offsets
    /// its selectors from: the dyld shared cache's.
    pub selector_base: Option<u64>,
    /// The symbols of the image that spell a method, `-[…]` or `+[…]`, by address.
    method_symbols: HashMap<u64, &'a str>,
    /// Work left for the metadata walks; see [`Self::charge`].
    work: Cell<u64>,
}

/// The walk budget, in bytes of metadata visited: eight times the mapped bytes,
/// with a floor for a small image. An honest walk visits each record and string
/// a handful of times; a crafted image whose lists all repeat the same records
/// (or one long string) stops here instead of costing unbounded time and memory.
fn work_budget(spans: &[(u64, u64, &[u8])]) -> u64 {
    let mapped: u64 = spans.iter().map(|&(_, _, d)| d.len() as u64).sum();
    mapped.saturating_mul(8).max(16 << 20)
}

impl<'a> MachoImage<'a> {
    /// Read addresses outside the image through `outside` (a dyld shared cache),
    /// and resolve direct selectors from `selector_base`.
    pub fn with_cache(
        mut self,
        outside: Rc<dyn Fn(u64, usize) -> Option<Vec<u8>> + 'a>,
        selector_base: Option<u64>,
    ) -> MachoImage<'a> {
        self.outside = Some(outside);
        self.selector_base = selector_base;
        self
    }

    /// Up to `n` bytes at `va` from the cache, charged to the walk budget.
    fn read_outside(&self, va: u64, n: usize) -> Option<Vec<u8>> {
        let outside = self.outside.as_ref()?;
        self.charge(n as u64).then(|| outside(va, n)).flatten()
    }

    /// Keep the method names `file`'s own symbol table gives, for
    /// [`Self::method_symbol`].
    pub fn with_method_symbols(mut self, file: &'a object::File<'a>) -> MachoImage<'a> {
        use object::{Object, ObjectSymbol};
        self.method_symbols = file
            .symbols()
            .filter_map(|s| Some((s.address(), s.name().ok()?)))
            .filter(|(_, n)| n.starts_with("-[") || n.starts_with("+["))
            .collect();
        self
    }

    /// The symbol at `imp` when it spells the `kind` method `selector` of
    /// `class`, under any category name. The linker merges the categories of a
    /// class into one and the metadata names only the merged one, so the symbol
    /// is the spelling to keep.
    pub fn method_symbol(&self, imp: u64, kind: char, class: &str, selector: &str) -> Option<&'a str> {
        let name = *self.method_symbols.get(&imp)?;
        let stem = name.strip_prefix(kind)?.strip_prefix('[')?.strip_suffix(']')?.strip_suffix(selector)?.strip_suffix(' ')?;
        let owner = stem.split_once('(').map_or(stem, |(owner, _)| owner);
        (owner == class).then_some(name)
    }

    /// Every method symbol of the image with its address.
    pub fn method_symbols(&self) -> impl Iterator<Item = (u64, &'a str)> + '_ {
        self.method_symbols.iter().map(|(&addr, &name)| (addr, name))
    }

    /// Whether the image was loaded from a dyld shared cache.
    pub fn in_cache(&self) -> bool {
        self.outside.is_some()
    }

    /// Spend `units` of the walk budget; `false` once it is exhausted.
    pub fn charge(&self, units: u64) -> bool {
        let left = self.work.get();
        if left < units {
            self.work.set(0);
            return false;
        }
        self.work.set(left - units);
        true
    }

    /// Build the VMA→bytes map from a parsed Mach-O `file`. Skips zero-fill
    /// sections (no file content) and zero-length sections. `None` is never
    /// returned — an image with no usable sections yields an empty map (every read
    /// then misses, the inert path).
    pub fn new(file: &'a object::File<'a>) -> MachoImage<'a> {
        Self::with_fixups(file, ChainedFixups::default())
    }

    /// Like [`Self::new`] but with a resolved [`ChainedFixups`] overlay (PR-O0):
    /// [`Self::read_ptr`] consults the overlay first, so an arm64(e)
    /// chained-fixup image's `__objc_*` pointer slots dereference to **real**
    /// addresses instead of packed fixup entries. The live ObjC pass builds the
    /// overlay from `ctx.bytes` and threads it here; passing
    /// [`ChainedFixups::default`] (the no-op overlay) reproduces the classic
    /// raw-section read path for a non-chained-fixup Mach-O.
    pub fn with_fixups(file: &'a object::File<'a>, fixups: ChainedFixups) -> MachoImage<'a> {
        let mut spans: Vec<(u64, u64, &'a [u8])> = Vec::new();
        for sec in file.sections() {
            if matches!(sec.kind(), SectionKind::UninitializedData) {
                continue;
            }
            // An ELF's unmapped sections (`.symtab`, `.strtab`, debug info)
            // sit at address 0 and would shadow the mapped ones there.
            if let object::SectionFlags::Elf { sh_flags } = sec.flags() {
                if sh_flags & u64::from(object::elf::SHF_ALLOC) == 0 {
                    continue;
                }
            }
            let addr = sec.address();
            let size = sec.size();
            if size == 0 {
                continue;
            }
            if let Ok(data) = sec.data() {
                spans.push((addr, addr.saturating_add(size), data));
            }
        }
        spans.sort_unstable_by_key(|&(lo, _, _)| lo);
        let ptr_size = if file.is_64() { 8 } else { 4 };
        let work = Cell::new(work_budget(&spans));
        MachoImage {
            spans,
            endian: file.endianness(),
            ptr_size,
            fixups,
            binds: HashMap::new(),
            outside: None,
            selector_base: None,
            method_symbols: HashMap::new(),
            work,
        }
    }

    /// Attach the classic `LC_DYLD_INFO` bind slots (`slot_vma → import`), so
    /// [`Self::bind_name`] answers on an image without chained fixups.
    pub fn with_binds(mut self, binds: HashMap<u64, Rc<str>>) -> MachoImage<'a> {
        self.binds = binds;
        self
    }

    /// Pointer size in bytes for this image.
    pub fn ptr_size(&self) -> u64 {
        self.ptr_size
    }

    /// `n` bytes starting at virtual address `va`, if `[va, va+n)` lies wholly
    /// inside one mapped section. `None` for an unmapped / split read.
    pub fn read(&self, va: u64, n: usize) -> Option<&'a [u8]> {
        let end = va.checked_add(n as u64)?;
        for &(lo, hi, data) in &self.spans {
            if va >= lo && end <= hi {
                let off = (va - lo) as usize;
                return data.get(off..off + n);
            }
        }
        None
    }

    /// A pointer-sized word at `va` (the absolute `class_t*` / `class_ro_t*` /
    /// string-pointer form the large/64-bit ObjC layout uses), honoring endianness
    /// + pointer width.
    ///
    /// On a chained-fixup image the raw section word at a pointer slot is a packed
    /// fixup entry, **not** the real address, so a resolved rebase from the
    /// [`ChainedFixups`] overlay wins when one exists at `va`. A slot the overlay
    /// does not cover (a non-fixup constant, a bind/import slot, or any non-chained
    /// Mach-O — where the overlay is empty) falls through to the raw section read,
    /// which is byte-identical to the pre-resolver behavior.
    pub fn read_ptr(&self, va: u64) -> Option<u64> {
        if let Some(resolved) = self.fixups.resolved_ptr(va) {
            return Some(resolved);
        }
        let n = self.ptr_size as usize;
        let owned;
        let bytes: &[u8] = match self.read(va, n) {
            Some(b) => b,
            None => {
                owned = self.read_outside(va, n)?;
                &owned
            }
        };
        Some(match (self.endian, self.ptr_size) {
            (Endianness::Little, 8) => u64::from_le_bytes(bytes.try_into().ok()?),
            (Endianness::Big, 8) => u64::from_be_bytes(bytes.try_into().ok()?),
            (Endianness::Little, _) => u32::from_le_bytes(bytes.try_into().ok()?) as u64,
            (Endianness::Big, _) => u32::from_be_bytes(bytes.try_into().ok()?) as u64,
        })
    }

    /// The import symbol a **bind** slot at `va` names (`_OBJC_CLASS_$_NSObject`),
    /// from the chained-fixup imports or the classic bind program; `None` for any
    /// other slot.
    pub fn bind_name(&self, va: u64) -> Option<&str> {
        self.fixups.bind_name(va).or_else(|| self.binds.get(&va).map(Rc::as_ref))
    }

    /// Whether the image carries chained fixups (`LC_DYLD_CHAINED_FIXUPS`), a
    /// bind-only image included: its slots are packed words, not pointers.
    pub fn chained(&self) -> bool {
        self.fixups.is_chained()
    }

    /// The pointer a **rebase** slot at `va` holds: the resolved chained-fixup
    /// target, or on a classic `LC_DYLD_INFO` image the non-zero raw word (a
    /// bind slot there reads `0`). `None` for a bind slot or an unmapped `va`;
    /// on a chained image a slot the overlay does not cover is never a pointer.
    pub fn rebase_ptr(&self, va: u64) -> Option<u64> {
        if let Some(resolved) = self.fixups.resolved_ptr(va) {
            return Some(resolved);
        }
        if self.chained() {
            return None;
        }
        self.read_ptr(va).filter(|&p| p != 0)
    }

    /// A little/big-endian `u32` at `va` (the `method_list_t` header fields +
    /// the relative-method-list `i32` offsets are read through this).
    pub fn read_u32(&self, va: u64) -> Option<u32> {
        let owned;
        let bytes: &[u8] = match self.read(va, 4) {
            Some(b) => b,
            None => {
                owned = self.read_outside(va, 4)?;
                &owned
            }
        };
        Some(match self.endian {
            Endianness::Little => u32::from_le_bytes(bytes.try_into().ok()?),
            Endianness::Big => u32::from_be_bytes(bytes.try_into().ok()?),
        })
    }

    /// A signed little/big-endian `i32` at `va` (a relative-method-list field is a
    /// signed self-relative displacement).
    pub fn read_i32(&self, va: u64) -> Option<i32> {
        self.read_u32(va).map(|v| v as i32)
    }

    /// The NUL-terminated ASCII string at `va` (a selector / class name / type
    /// encoding). `None` for an unmapped address, a string with no terminator in
    /// its section, or one longer than the walk budget left. Reads byte-by-byte
    /// across the covering section only, and never more bytes than the budget
    /// can pay for. A string outside the image is looked for in a 64-byte
    /// window of the cache, then a 4096-byte one.
    pub fn read_cstr(&self, va: u64) -> Option<String> {
        let left = self.work.get();
        if left == 0 {
            return None;
        }
        let Some(&(lo, hi, data)) = self.spans.iter().find(|&&(lo, hi, _)| va >= lo && va < hi) else {
            return [64usize, 4096].into_iter().find_map(|n| {
                let bytes = self.read_outside(va, n)?;
                let s = &bytes[..bytes.iter().position(|&b| b == 0)?];
                s.iter().all(|b| (0x20..0x7f).contains(b)).then(|| String::from_utf8_lossy(s).into_owned())
            });
        };
        let start = (va - lo) as usize;
        let max = usize::try_from(hi - lo).unwrap_or(usize::MAX).min(data.len());
        let max = max.min(start.saturating_add(usize::try_from(left).unwrap_or(usize::MAX)));
        let mut s = String::new();
        let mut i = start;
        let mut terminated = false;
        while i < max {
            let b = data[i];
            if b == 0 {
                terminated = true;
                break;
            }
            // ObjC selectors / class names / type encodings are printable ASCII.
            if !(0x20..0x7f).contains(&b) {
                break;
            }
            s.push(b as char);
            i += 1;
        }
        let paid = self.charge((i - start) as u64 + 1);
        (terminated && paid).then_some(s)
    }

    /// The `(vma_lo, bytes)` of every `__objc_classlist` section — the arrays of
    /// `class_t*` entries the walk iterates (stride = [`Self::ptr_size`]). Empty
    /// when the image carries no ObjC class list (the inert path).
    pub fn classlists(&self, file: &'a object::File<'a>) -> Vec<(u64, &'a [u8])> {
        sections_named(file, SEC_CLASSLIST)
            .filter_map(|sec| Some((sec.address(), sec.data().ok()?)))
            .filter(|(_, data)| !data.is_empty())
            .collect()
    }
}

/// Every section named `name`, in load-command order. An ordinary image has at
/// most one; a Mach-O joining several dyld shared cache images (`dyldimage
/// A+B`) has one per image.
pub fn sections_named<'data, 'file>(
    file: &'file object::File<'data>,
    name: &'file str,
) -> impl Iterator<Item = object::Section<'data, 'file>> + 'file {
    file.sections().filter(move |sec| sec.name_bytes() == Ok(name.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn image(data: &[u8], work: u64) -> MachoImage<'_> {
        MachoImage {
            spans: vec![(0x1000, 0x1000 + data.len() as u64, data)],
            endian: Endianness::Little,
            ptr_size: 8,
            fixups: ChainedFixups::default(),
            binds: HashMap::new(),
            outside: None,
            selector_base: None,
            method_symbols: HashMap::new(),
            work: Cell::new(work),
        }
    }

    /// A string is read only as far as the budget left pays for: one that
    /// fits is read and charged its bytes, one longer than what is left is
    /// refused and spends the rest.
    #[test]
    fn read_cstr_scans_no_further_than_the_budget() {
        let mut data = vec![b'a'; 1 << 16];
        data[3] = 0;
        let last = data.len() - 1;
        data[last] = 0;
        let img = image(&data, 100);
        assert_eq!(img.read_cstr(0x1000).as_deref(), Some("aaa"));
        assert_eq!(img.work.get(), 96);
        assert_eq!(img.read_cstr(0x1004), None);
        assert_eq!(img.work.get(), 0);
        let img = image(&data, 1 << 20);
        assert_eq!(img.read_cstr(0x1004).map(|s| s.len()), Some((1 << 16) - 5));
    }

    /// A symbol names the method under the category the source gave it; the
    /// metadata may name another. Only the same kind, class and selector match.
    #[test]
    fn method_symbol_matches_class_and_selector_under_any_category() {
        let mut img = image(&[0; 8], 0);
        img.method_symbols.insert(0x1000, "-[NSObject(NSKeyValueCoding) valueForKey:]");
        img.method_symbols.insert(0x1004, "+[Greeter make]");
        assert_eq!(img.method_symbol(0x1000, '-', "NSObject", "valueForKey:"), Some("-[NSObject(NSKeyValueCoding) valueForKey:]"));
        assert_eq!(img.method_symbol(0x1004, '+', "Greeter", "make"), Some("+[Greeter make]"));
        assert_eq!(img.method_symbol(0x1000, '+', "NSObject", "valueForKey:"), None);
        assert_eq!(img.method_symbol(0x1000, '-', "NSObj", "valueForKey:"), None);
        assert_eq!(img.method_symbol(0x1000, '-', "NSObject", "Key:"), None);
        assert_eq!(img.method_symbol(0x1008, '-', "NSObject", "valueForKey:"), None);
    }

    /// Bytes outside the image come from the cache and are charged to the
    /// walk budget; a string there is found in the window that holds its end.
    #[test]
    fn reads_outside_the_image_go_through_the_cache_and_the_budget() {
        let mut far = vec![b's'; 200];
        far[3] = 0;
        far.push(0);
        far.extend([0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11]);
        let outside = move |va: u64, n: usize| {
            let at = usize::try_from(va.checked_sub(0x9000)?).ok()?;
            far.get(at..).map(|b| b[..n.min(b.len())].to_vec())
        };
        let img = image(&[0; 8], 5000).with_cache(Rc::new(outside), Some(0x9000));
        assert!(img.in_cache());
        assert_eq!(img.read_cstr(0x9000).as_deref(), Some("sss"));
        assert_eq!(img.read_cstr(0x9004).map(|s| s.len()), Some(196));
        assert_eq!(img.read_ptr(0x9000 + 201), Some(0x1122_3344_5566_7788));
        assert_eq!(img.read_ptr(0x9000 + 205), None, "a short read is no pointer");
        assert_eq!(img.read_cstr(0x8000), None);
        let spent = image(&[0; 8], 10).with_cache(Rc::new(|_, n| Some(vec![0; n])), None);
        assert_eq!(spent.read_u32(0x9000), Some(0));
        assert_eq!(spent.read_ptr(0x9000), None, "the budget is spent");
    }
}

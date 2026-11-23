//! (kuna) The dyld shared cache: open a cache and its sub-caches, list its images,
//! and rebuild one image, or several joined, as a standalone thin Mach-O at its
//! cache addresses.
//!
//! A cache is a main file named `dyld_shared_cache_<arch>` plus, on iOS 15+ and
//! macOS 12+, sub-cache files beside it (`.01`, `.27.dylddata`, `.44.dyldlinkedit`,
//! ...) and a `.symbols` file holding the local symbols. Each file carries a
//! header and a mapping table placing its bytes at virtual addresses. Images are
//! named by an image table in the main file. The record layouts are Apple's public
//! `dyld_cache_format.h`; a header field exists when it lies below `mappingOffset`,
//! which is how the format grows without a version number.
//!
//! The cache is never read whole: a 3 GB cache costs only the pages of the image
//! being rebuilt, read through file handles. [`read_selected_image`] is the one
//! entry point the loader paths call, before they parse anything; after it, the
//! input is an ordinary Mach-O. See [`extract`] for the rebuild, `merge` for
//! how several images become one program, and [`xrefs`] for how calls into
//! other images are named. [`follow_images`] finds the image an address outside
//! the selection lies in.

use std::cell::{Cell, OnceCell, RefCell};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::rc::Rc;

pub mod extract;
mod merge;
pub mod slide;
pub mod xrefs;

/// Joins the image names of a `dyldimage` value that loads several images.
pub const IMAGE_SEPARATOR: char = '+';

/// Every dyld shared cache starts with this, followed by padding and the arch.
pub const MAGIC_PREFIX: &[u8] = b"dyld_v1";

pub const MAPPING_TEXT_STUBS: u64 = 1 << 3;

const HDR_MAPPING_OFFSET: usize = 16;
const HDR_IMAGES_OFFSET_OLD: usize = 24;
const HDR_SLIDE_INFO_OLD: usize = 56;
const HDR_LOCAL_SYMBOLS: usize = 72;
const HDR_UUID: usize = 88;
const HDR_MAPPING_WITH_SLIDE: usize = 312;
const HDR_SUBCACHES: usize = 392;
const HDR_SYMBOL_FILE_UUID: usize = 400;
const HDR_IMAGES_OFFSET: usize = 448;
const HDR_CACHE_SUBTYPE: usize = 456;
const HDR_OBJC_OPTS: usize = 464;
/// `ObjCOptimizationHeader.relativeMethodSelectorBaseAddressOffset`.
const OBJC_OPTS_SELECTOR_BASE: usize = 0x30;
/// `objc_opt_t.relativeMethodSelectorBaseAddressOffset`, present from version 16.
const OBJC_OPT_T_SELECTOR_BASE: usize = 0x28;

/// Does `prefix` (the first bytes of a file) open a dyld shared cache?
pub fn is_dyld_cache(prefix: &[u8]) -> bool {
    prefix.starts_with(MAGIC_PREFIX)
}

/// Does the file at `path` open a dyld shared cache? Reads 16 bytes.
pub fn is_dyld_cache_file(path: &Path) -> bool {
    let mut prefix = [0u8; 16];
    File::open(path)
        .and_then(|mut f| f.read_exact(&mut prefix))
        .is_ok_and(|()| is_dyld_cache(&prefix))
}

/// The selected images of the dyld shared cache at `path`, rebuilt as one thin
/// Mach-O; `None` when `path` is not a dyld shared cache. The selection is the
/// `dyldimage` option, carried in [`kuna_decomp::kuna_dyldcache::DYLDIMAGE_ENV`].
pub fn read_selected_image(path: &Path) -> Option<Result<Vec<u8>, String>> {
    if !is_dyld_cache_file(path) {
        return None;
    }
    Some(DyldCache::open(path).and_then(|cache| {
        cache.extract_merged(&cache.select_all(kuna_decomp::kuna_dyldcache::selected_image().as_deref())?)
    }))
}

/// The byte size the images `name` selects occupy, `__LINKEDIT` aside (the cache shares
/// one across all images): the cache counterpart of a file's length for
/// size-based policy. `None` when `path` is not a cache.
pub fn image_size(path: &Path, name: Option<&str>) -> Option<Result<u64, String>> {
    if !is_dyld_cache_file(path) {
        return None;
    }
    Some(DyldCache::open(path).and_then(|cache| {
        let mut size = 0u64;
        for index in cache.select_all(name)? {
            let image = cache.image_layout(index)?;
            size += image.segments.iter().filter(|s| s.name != "__LINKEDIT").map(|s| s.filesize).sum::<u64>();
        }
        Ok(size)
    }))
}

/// One address [`follow_images`] found outside the loaded images.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FollowStep {
    pub requested: u64,
    /// `requested`, or what the stub at `requested` jumps to.
    pub target: u64,
    /// The install name of the image holding `target`.
    pub image: String,
    /// Is that image part of [`Followed::images`]? `false` past the limit.
    pub loaded: bool,
}

/// What [`follow_images`] found: the `dyldimage` value to load and one step per
/// address the selection did not hold.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Followed {
    pub images: String,
    pub steps: Vec<FollowStep>,
}

/// Widen the `dyldimage` value `selected` of the cache at `path` until the
/// images it names hold `addrs`, adding at most `limit` images. Each address
/// carries whether a stub there may be followed to its target: an address in a
/// stub island belongs to no image, and what a request for it means is the
/// function the stub jumps to. An address the cache does not map is left alone.
pub fn follow_images(path: &Path, selected: &str, addrs: &[(u64, bool)], limit: usize) -> Result<Followed, String> {
    let cache = DyldCache::open(path)?;
    let mut loaded = Vec::new();
    for index in cache.select_all(Some(selected))? {
        loaded.push(cache.image_layout(index)?);
    }
    let mut followed = Followed { images: selected.to_string(), steps: Vec::new() };
    let mut names = None;
    let mut added = 0;
    for &(requested, through_stubs) in addrs {
        if loaded.iter().any(|layout| layout.holds(requested)) {
            continue;
        }
        let names = names.get_or_insert_with(|| xrefs::Names::new(&cache));
        let Some((target, index)) = names.owner(requested, through_stubs) else {
            continue;
        };
        let image = cache.images[index].path.clone();
        let mut held = loaded.iter().any(|layout| layout.holds(target));
        if !held && added < limit {
            loaded.push(cache.image_layout(index)?);
            followed.images.push(IMAGE_SEPARATOR);
            followed.images.push_str(&image);
            added += 1;
            held = true;
        }
        followed.steps.push(FollowStep { requested, target, image, loaded: held });
    }
    Ok(followed)
}

/// One file of a split cache: the main file, a sub-cache, or `.symbols`.
struct CacheFile {
    file: File,
    name: String,
    len: u64,
}

impl CacheFile {
    /// An error for a `len`-byte range at `offset` the file does not hold.
    fn check_range(&self, offset: u64, len: u64) -> Result<(), String> {
        if offset.checked_add(len).is_none_or(|end| end > self.len) {
            return Err(format!(
                "{}: {len} bytes at {offset:#x} lie past the end of the {}-byte file",
                self.name, self.len
            ));
        }
        Ok(())
    }

    /// The `len` bytes at `offset`; an error, before anything is allocated, for
    /// a range the file does not hold.
    fn read_at(&self, offset: u64, len: usize) -> Result<Vec<u8>, String> {
        self.check_range(offset, len as u64)?;
        let mut buf = vec![0u8; len];
        let mut f = &self.file;
        f.seek(SeekFrom::Start(offset))
            .and_then(|_| f.read_exact(&mut buf))
            .map_err(|e| format!("{}: cannot read {len} bytes at {offset:#x}: {e}", self.name))?;
        Ok(buf)
    }
}

/// One `dyld_cache_mapping[_and_slide]_info`: bytes of `file` at `address`.
#[derive(Clone, Debug)]
pub struct Mapping {
    pub address: u64,
    pub size: u64,
    pub file_offset: u64,
    pub file: usize,
    pub slide_offset: u64,
    pub slide_size: u64,
    pub flags: u64,
    pub init_prot: u32,
}

impl Mapping {
    fn contains(&self, va: u64, len: u64) -> bool {
        match (va.checked_add(len), self.address.checked_add(self.size)) {
            (Some(end), Some(limit)) => va >= self.address && end <= limit,
            _ => false,
        }
    }
}

/// One image of the cache: its Mach-O header address and install name.
#[derive(Clone, Debug)]
pub struct CacheImage {
    pub address: u64,
    pub path: String,
}

/// Where an image's local symbols live in the local-symbols file.
struct LocalSymbols {
    file: usize,
    nlist_offset: u64,
    strings_offset: u64,
    strings_size: u64,
    /// image key -> (first nlist index, count); the key is the image's offset
    /// from the cache base (64-bit entries) or its file offset (32-bit entries).
    entries: HashMap<u64, (u32, u32)>,
    keyed_by_vm_offset: bool,
}

/// An opened dyld shared cache.
pub struct DyldCache {
    files: Vec<CacheFile>,
    mappings: Vec<Mapping>,
    images: Vec<CacheImage>,
    base: u64,
    locals: Option<LocalSymbols>,
    slides: RefCell<HashMap<usize, Option<Rc<slide::SlideInfo>>>>,
    /// Image layouts by header address, so aliases of one image share one.
    layouts: RefCell<HashMap<u64, Option<std::rc::Rc<extract::ImageLayout>>>>,
    /// The bytes of every cache file: no image's segments or load commands
    /// can legitimately add up to more.
    total_len: u64,
    /// The load-command bytes read into `layouts` so far, bounded by `total_len`.
    layout_bytes: Cell<u64>,
    /// Bytes the slide infos and symbol tables built so far may still retain;
    /// see [`DyldCache::charge`].
    retained: Cell<u64>,
    /// `objcOptsOffset` of the header: the `ObjCOptimizationHeader`, as an
    /// offset from the cache base. Zero or absent before dyld-1042.
    objc_opts: Option<u64>,
    objc_selector_base: OnceCell<Option<u64>>,
}

fn u32_at(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes(b[off..off + 4].try_into().unwrap())
}

fn u64_at(b: &[u8], off: usize) -> u64 {
    u64::from_le_bytes(b[off..off + 8].try_into().unwrap())
}

/// A cache header: the bytes below `mappingOffset`, which bound its fields.
struct Header(Vec<u8>);

impl Header {
    fn read(file: &CacheFile) -> Result<Header, String> {
        let head = file.read_at(0, 24)?;
        if !is_dyld_cache(&head) {
            return Err(format!("{}: not a dyld shared cache", file.name));
        }
        let len = (u32_at(&head, HDR_MAPPING_OFFSET) as usize).clamp(24, 0x10000);
        Ok(Header(file.read_at(0, len)?))
    }

    fn has(&self, offset: usize, width: usize) -> bool {
        offset + width <= self.0.len()
    }

    fn u32(&self, offset: usize) -> Option<u32> {
        self.has(offset, 4).then(|| u32_at(&self.0, offset))
    }

    fn u64(&self, offset: usize) -> Option<u64> {
        self.has(offset, 8).then(|| u64_at(&self.0, offset))
    }

    fn uuid(&self, offset: usize) -> Option<[u8; 16]> {
        self.has(offset, 16).then(|| self.0[offset..offset + 16].try_into().unwrap())
    }

    fn arch(&self) -> String {
        String::from_utf8_lossy(&self.0[7..16]).trim_matches(|c: char| c == ' ' || c == '\0').to_string()
    }
}

fn read_mappings(file: &CacheFile, index: usize, h: &Header) -> Result<Vec<Mapping>, String> {
    let mut out = Vec::new();
    if let (Some(off), Some(count)) = (h.u32(HDR_MAPPING_WITH_SLIDE), h.u32(HDR_MAPPING_WITH_SLIDE + 4)) {
        if count > 0 {
            let table = file.read_at(off as u64, count as usize * 56)?;
            for r in table.chunks_exact(56) {
                out.push(Mapping {
                    address: u64_at(r, 0),
                    size: u64_at(r, 8),
                    file_offset: u64_at(r, 16),
                    file: index,
                    slide_offset: u64_at(r, 24),
                    slide_size: u64_at(r, 32),
                    flags: u64_at(r, 40),
                    init_prot: u32_at(r, 52),
                });
            }
            return Ok(out);
        }
    }
    let off = h.u32(HDR_MAPPING_OFFSET).unwrap_or(0);
    let count = h.u32(HDR_MAPPING_OFFSET + 4).unwrap_or(0);
    let table = file.read_at(off as u64, count as usize * 32)?;
    let (old_slide_offset, old_slide_size) =
        (h.u64(HDR_SLIDE_INFO_OLD).unwrap_or(0), h.u64(HDR_SLIDE_INFO_OLD + 8).unwrap_or(0));
    for (i, r) in table.chunks_exact(32).enumerate() {
        let (slide_offset, slide_size) =
            if i == 1 { (old_slide_offset, old_slide_size) } else { (0, 0) };
        out.push(Mapping {
            address: u64_at(r, 0),
            size: u64_at(r, 8),
            file_offset: u64_at(r, 16),
            file: index,
            slide_offset,
            slide_size,
            flags: 0,
            init_prot: u32_at(r, 28),
        });
    }
    Ok(out)
}

/// The file names of a main cache's sub-caches, with the UUID each must carry.
fn subcache_names(main: &CacheFile, path: &Path, h: &Header) -> Result<Vec<(PathBuf, [u8; 16])>, String> {
    let (Some(off), Some(count)) = (h.u32(HDR_SUBCACHES), h.u32(HDR_SUBCACHES + 4)) else {
        return Ok(Vec::new());
    };
    let named = h.has(HDR_CACHE_SUBTYPE, 4);
    let stride = if named { 56 } else { 24 };
    let table = main.read_at(off as u64, count as usize * stride)?;
    table
        .chunks_exact(stride)
        .enumerate()
        .map(|(i, entry)| {
            let suffix = if named {
                let raw = &entry[24..56];
                String::from_utf8_lossy(&raw[..raw.iter().position(|&b| b == 0).unwrap_or(32)]).into_owned()
            } else {
                format!(".{}", i + 1)
            };
            if suffix.contains(['/', '\\']) || suffix.contains("..") {
                return Err(format!("{}: sub-cache suffix {suffix:?} is not a file-name suffix", main.name));
            }
            let mut name = path.as_os_str().to_owned();
            name.push(&suffix);
            Ok((PathBuf::from(name), entry[..16].try_into().unwrap()))
        })
        .collect()
}

impl DyldCache {
    /// Open the cache whose main file is `path`, with every sub-cache and the
    /// `.symbols` file the header names.
    pub fn open(path: &Path) -> Result<DyldCache, String> {
        let open = |p: &Path| -> Result<CacheFile, String> {
            let file = File::open(p).map_err(|e| format!("{}: {e}", p.display()))?;
            let len = file.metadata().map_err(|e| format!("{}: {e}", p.display()))?.len();
            Ok(CacheFile { file, name: p.display().to_string(), len })
        };
        let main = open(path)?;
        let h = Header::read(&main)?;
        let mut mappings = read_mappings(&main, 0, &h)?;
        let base = mappings.first().map(|m| m.address).ok_or("dyld shared cache has no mappings")?;
        let mut files = vec![main];
        for (sub_path, uuid) in subcache_names(&files[0], path, &h)? {
            let sub = open(&sub_path).map_err(|e| format!("missing sub-cache: {e}"))?;
            let sh = Header::read(&sub)?;
            if sh.uuid(HDR_UUID) != Some(uuid) {
                return Err(format!("{}: sub-cache UUID does not match the main cache", sub.name));
            }
            mappings.extend(read_mappings(&sub, files.len(), &sh)?);
            files.push(sub);
        }
        let locals = Self::open_local_symbols(path, &h, &mut files, &open)?;
        let images = Self::read_images(&files[0], &h)?;
        let total_len: u64 = files.iter().map(|f| f.len).sum();
        mappings.retain(|m| m.size != 0);
        mappings.sort_by_key(|m| m.address);
        Ok(DyldCache {
            files,
            mappings,
            images,
            base,
            locals,
            slides: RefCell::default(),
            layouts: RefCell::default(),
            total_len,
            layout_bytes: Cell::new(0),
            retained: Cell::new(total_len.saturating_mul(2).max(MIN_RETAINED)),
            objc_opts: h.u64(HDR_OBJC_OPTS).filter(|&o| o != 0),
            objc_selector_base: OnceCell::new(),
        })
    }

    /// The address every direct-selector method list of the cache offsets its
    /// selectors from. The `ObjCOptimizationHeader` the cache header names
    /// carries it as an offset from the cache base; an older cache carries it in
    /// libobjc's `objc_opt_t` (`__objc_opt_ro`, version 16 on) as a signed offset
    /// from that record. `None` on a cache with neither.
    pub fn objc_selector_base(&self) -> Option<u64> {
        *self.objc_selector_base.get_or_init(|| self.header_selector_base().or_else(|| self.libobjc_selector_base()))
    }

    fn header_selector_base(&self) -> Option<u64> {
        let opts = self.read_va(self.base.checked_add(self.objc_opts?)?, OBJC_OPTS_SELECTOR_BASE + 8).ok()?;
        if u32_at(&opts, 0) == 0 {
            return None;
        }
        let rel = u64_at(&opts, OBJC_OPTS_SELECTOR_BASE);
        if rel == 0 { None } else { self.base.checked_add(rel) }
    }

    fn libobjc_selector_base(&self) -> Option<u64> {
        let index = self.images.iter().position(|i| i.path.ends_with("/libobjc.A.dylib"))?;
        let layout = self.image_layout(index).ok()?;
        let opt = layout.segments.iter().flat_map(|s| s.sections.iter()).find(|s| s.sectname == "__objc_opt_ro")?;
        let record = self.read_va(opt.addr, OBJC_OPT_T_SELECTOR_BASE + 8).ok()?;
        if u32_at(&record, 0) < 16 {
            return None;
        }
        let rel = u64_at(&record, OBJC_OPT_T_SELECTOR_BASE) as i64;
        if rel == 0 { None } else { opt.addr.checked_add_signed(rel) }
    }

    fn open_local_symbols(
        path: &Path,
        h: &Header,
        files: &mut Vec<CacheFile>,
        open: &dyn Fn(&Path) -> Result<CacheFile, String>,
    ) -> Result<Option<LocalSymbols>, String> {
        let symbol_uuid = h.uuid(HDR_SYMBOL_FILE_UUID).filter(|u| u.iter().any(|&b| b != 0));
        let (file, sh_offset, sh_size) = match symbol_uuid {
            Some(uuid) => {
                let mut name = path.as_os_str().to_owned();
                name.push(".symbols");
                let name = PathBuf::from(name);
                if !name.exists() {
                    return Ok(None);
                }
                let sym = open(&name)?;
                let sh = Header::read(&sym)?;
                if sh.uuid(HDR_UUID) != Some(uuid) {
                    return Err(format!("{}: UUID does not match the main cache", sym.name));
                }
                let (off, size) = (sh.u64(HDR_LOCAL_SYMBOLS), sh.u64(HDR_LOCAL_SYMBOLS + 8));
                files.push(sym);
                (files.len() - 1, off.unwrap_or(0), size.unwrap_or(0))
            }
            None => (0, h.u64(HDR_LOCAL_SYMBOLS).unwrap_or(0), h.u64(HDR_LOCAL_SYMBOLS + 8).unwrap_or(0)),
        };
        if sh_offset == 0 || sh_size < 24 {
            return Ok(None);
        }
        let info = files[file].read_at(sh_offset, 24)?;
        let wide = symbol_uuid.is_some();
        let entry_size = if wide { 16 } else { 12 };
        let count = u32_at(&info, 20) as usize;
        let table = files[file].read_at(sh_offset.saturating_add(u32_at(&info, 16) as u64), count * entry_size)?;
        let entries = table
            .chunks_exact(entry_size)
            .map(|e| {
                if wide {
                    (u64_at(e, 0), (u32_at(e, 8), u32_at(e, 12)))
                } else {
                    (u32_at(e, 0) as u64, (u32_at(e, 4), u32_at(e, 8)))
                }
            })
            .collect();
        Ok(Some(LocalSymbols {
            file,
            nlist_offset: sh_offset.saturating_add(u32_at(&info, 0) as u64),
            strings_offset: sh_offset.saturating_add(u32_at(&info, 8) as u64),
            strings_size: u32_at(&info, 12) as u64,
            entries,
            keyed_by_vm_offset: wide,
        }))
    }

    fn read_images(main: &CacheFile, h: &Header) -> Result<Vec<CacheImage>, String> {
        let (off, count) = match (h.u32(HDR_IMAGES_OFFSET), h.u32(HDR_IMAGES_OFFSET + 4)) {
            (Some(off), Some(count)) if count > 0 => (off, count),
            _ => (h.u32(HDR_IMAGES_OFFSET_OLD).unwrap_or(0), h.u32(HDR_IMAGES_OFFSET_OLD + 4).unwrap_or(0)),
        };
        let table = main.read_at(off as u64, count as usize * 32)?;
        let rows: Vec<(u64, u64)> =
            table.chunks_exact(32).map(|r| (u64_at(r, 0), u32_at(r, 24) as u64)).collect();
        let extended = Cell::new(0u64);
        let charge = |n: u64| {
            extended.set(extended.get() + n);
            if extended.get() > main.len { Err("image paths run past the cache file".to_string()) } else { Ok(()) }
        };
        let pool = StringPool::read_at(&|o, len| main.read_at(o, len), main.len, rows.iter().map(|r| r.1), &charge)?;
        let mut allowance = main.len as usize;
        Ok(rows
            .into_iter()
            .map(|(address, name)| {
                let path = pool.get_at_max(name, allowance);
                allowance -= path.len();
                CacheImage { address, path: String::from_utf8_lossy(path).into_owned() }
            })
            .collect())
    }

    /// The cache's architecture name, from the magic (`arm64e`, `x86_64h`, ...).
    pub fn arch(&self) -> String {
        Header::read(&self.files[0]).map(|h| h.arch()).unwrap_or_default()
    }

    pub fn images(&self) -> &[CacheImage] {
        &self.images
    }

    pub fn mappings(&self) -> &[Mapping] {
        &self.mappings
    }

    /// The unslid address of the first mapping, which offsets in the cache count from.
    pub fn base(&self) -> u64 {
        self.base
    }

    /// Resolve an image name to its index: a full install name, else a unique
    /// basename, else a unique basename without its last extension. With no
    /// name, a cache of one image selects it.
    pub fn select(&self, name: Option<&str>) -> Result<usize, String> {
        select_image(&self.images, name)
    }

    /// Resolve a `dyldimage` value: one name, or several joined by `+`
    /// (`Foundation+CoreFoundation`), each as [`DyldCache::select`] reads it.
    /// A `+` inside an image's own name (`libc++.1.dylib+Foundation`) stays
    /// part of that name.
    pub fn select_all(&self, names: Option<&str>) -> Result<Vec<usize>, String> {
        match names {
            Some(names) if names.contains(IMAGE_SEPARATOR) => select_images(&self.images, names),
            name => Ok(vec![self.select(name)?]),
        }
    }

    /// The mapping holding `[va, va + len)`; the mappings are sorted by address.
    fn mapping_index(&self, va: u64, len: u64) -> Option<usize> {
        let i = self.mappings.partition_point(|m| m.address <= va).checked_sub(1)?;
        self.mappings[i].contains(va, len).then_some(i)
    }

    fn mapping_of(&self, va: u64, len: u64) -> Option<&Mapping> {
        self.mapping_index(va, len).map(|i| &self.mappings[i])
    }

    /// Charge `n` retained bytes against the cache's allowance: every slide info
    /// and symbol table kept in memory counts, and an honest image needs a
    /// fraction of twice the cache's size. An `Err` once it is spent stops the
    /// work that would have retained them.
    fn charge(&self, n: u64) -> Result<(), String> {
        let left = self.retained.get();
        if n > left {
            self.retained.set(0);
            return Err(format!("{}: slide info and symbol data exceed the cache's size", self.files[0].name));
        }
        self.retained.set(left - n);
        Ok(())
    }

    /// The raw bytes at `[va, va + len)`, which must lie in one mapping.
    pub fn read_va(&self, va: u64, len: usize) -> Result<Vec<u8>, String> {
        let m = self
            .mapping_of(va, len as u64)
            .ok_or_else(|| format!("address range {va:#x}+{len:#x} is not mapped by the cache"))?;
        let offset = m
            .file_offset
            .checked_add(va - m.address)
            .ok_or_else(|| format!("address {va:#x} maps past the end of its file"))?;
        self.files[m.file].read_at(offset, len)
    }

    /// The bytes at `[va, va + len)` with the slide info of their mapping applied
    /// at slide zero, so every rebased pointer reads as its unslid target.
    pub fn read_slid(&self, va: u64, len: usize) -> Result<Vec<u8>, String> {
        let index = self
            .mapping_index(va, len as u64)
            .ok_or_else(|| format!("address range {va:#x}+{len:#x} is not mapped by the cache"))?;
        let m = self.mappings[index].clone();
        let Some(info) = self.slide_info(index)? else {
            return self.read_va(va, len);
        };
        let page = info.page_size as u64;
        let lo = m.address + (va - m.address) / page * page;
        let hi = m
            .address
            .saturating_add((va + len as u64 - m.address).div_ceil(page).saturating_mul(page))
            .min(m.address + m.size);
        let mut pages = self.read_va(lo, (hi - lo) as usize)?;
        info.apply(&mut pages, ((lo - m.address) / page) as usize);
        let start = (va - lo) as usize;
        Ok(pages[start..start + len].to_vec())
    }

    fn slide_info(&self, mapping: usize) -> Result<Option<Rc<slide::SlideInfo>>, String> {
        if let Some(cached) = self.slides.borrow().get(&mapping) {
            return Ok(cached.clone());
        }
        let m = &self.mappings[mapping];
        let info = if m.slide_size == 0 {
            None
        } else {
            self.files[m.file].check_range(m.slide_offset, m.slide_size)?;
            self.charge(m.slide_size)?;
            let raw = self.files[m.file].read_at(m.slide_offset, m.slide_size as usize)?;
            slide::SlideInfo::parse(&raw)?.map(Rc::new)
        };
        self.slides.borrow_mut().insert(mapping, info.clone());
        Ok(info)
    }

    /// The local symbols the cache moved out of image `index`; empty when the
    /// cache has none.
    fn local_symbols(&self, index: usize) -> Result<Vec<extract::Nlist>, String> {
        let Some(locals) = &self.locals else {
            return Ok(Vec::new());
        };
        let address = self.images[index].address;
        let key = if locals.keyed_by_vm_offset {
            address.wrapping_sub(self.base)
        } else {
            match self.mapping_of(address, 1) {
                Some(m) => m.file_offset.wrapping_add(address - m.address),
                None => return Ok(Vec::new()),
            }
        };
        let Some(&(start, count)) = locals.entries.get(&key) else {
            return Ok(Vec::new());
        };
        self.charge(count as u64 * NLIST_RETAINED)?;
        let file = &self.files[locals.file];
        let raw = file.read_at(locals.nlist_offset.saturating_add(start as u64 * 16), count as usize * 16)?;
        let pool = StringPool::read(
            &|off, len| file.read_at(locals.strings_offset.saturating_add(off), len),
            locals.strings_size,
            &raw,
            &|n| self.charge(n),
        )?;
        extract::Nlist::parse_all(&raw, &pool, &|n| self.charge(n))
    }
}

/// [`DyldCache::select`] over an image list.
fn select_image(images: &[CacheImage], name: Option<&str>) -> Result<usize, String> {
    let Some(name) = name else {
        return match images.len() {
            1 => Ok(0),
            n => Err(format!(
                "dyld shared cache holds {n} images; choose one with --image NAME \
                 (list them with `kuna unpack <cache> --list`)"
            )),
        };
    };
    match_image(images, name)?.ok_or_else(|| no_image(images, name))
}

/// [`DyldCache::select_all`] over an image list, for a value holding a `+`.
/// From each piece the longest run of `+`-joined pieces that names an image is
/// taken, so a `+` of an image's own name (`libc++.1.dylib`) does not split it.
/// A run ends at the end of the value or before a non-empty piece (`libc++`
/// never reads as `libc`), and holds at most as many `+` as the image name
/// holding the most.
fn select_images(images: &[CacheImage], names: &str) -> Result<Vec<usize>, String> {
    let pieces: Vec<&str> = names.split(IMAGE_SEPARATOR).collect();
    let widest = images.iter().map(|i| i.path.matches(IMAGE_SEPARATOR).count()).max().unwrap_or(0);
    let joined = |from: usize, to: usize| pieces[from..to].join("+").trim().to_string();
    let ends_a_name = |to: &usize| *to == pieces.len() || !pieces[*to].trim().is_empty();
    let longest = |from: usize| -> Result<Option<(usize, usize)>, String> {
        for to in (from + 1..=pieces.len().min(from + 1 + widest)).rev().filter(ends_a_name) {
            if let Some(index) = match_image(images, &joined(from, to))? {
                return Ok(Some((to, index)));
            }
        }
        Ok(None)
    };
    let mut out = Vec::new();
    let mut from = 0;
    while from < pieces.len() {
        if pieces[from].trim().is_empty() {
            from += 1;
            continue;
        }
        let Some((to, index)) = longest(from)? else {
            let named = |k: &usize| !pieces[*k].trim().is_empty() && !matches!(longest(*k), Ok(None));
            let to = (from + 1..pieces.len()).find(named).unwrap_or(pieces.len());
            return Err(no_image(images, &joined(from, to)));
        };
        out.push(index);
        from = to;
    }
    Ok(out)
}

/// The image `name` names: a full install name, else a unique basename, else
/// a unique basename without its last extension. `None` when no image matches;
/// an error when several share the first tier that matches.
fn match_image(images: &[CacheImage], name: &str) -> Result<Option<usize>, String> {
    let base = |p: &str| p.rsplit('/').next().unwrap_or(p).to_string();
    let stem = |p: &str| {
        let b = base(p);
        b.rsplit_once('.').map(|(s, _)| s.to_string()).unwrap_or(b)
    };
    let tiers: [&dyn Fn(&CacheImage) -> bool; 3] = [
        &|i| i.path == name,
        &|i| base(&i.path) == name,
        &|i| stem(&i.path) == name,
    ];
    for matches in tiers {
        let hits: Vec<usize> =
            images.iter().enumerate().filter(|(_, i)| matches(i)).map(|(n, _)| n).collect();
        match hits.len() {
            0 => continue,
            1 => return Ok(Some(hits[0])),
            _ => {
                let names: Vec<&str> = hits.iter().map(|&n| images[n].path.as_str()).collect();
                return Err(format!("image name {name:?} is ambiguous: {}", names.join(", ")));
            }
        }
    }
    Ok(None)
}

/// The error for a `name` no image matches, with up to eight images whose
/// install names contain it.
fn no_image(images: &[CacheImage], name: &str) -> String {
    let lower = name.to_ascii_lowercase();
    let near: Vec<&str> = images
        .iter()
        .filter(|i| i.path.to_ascii_lowercase().contains(&lower))
        .take(8)
        .map(|i| i.path.as_str())
        .collect();
    if near.is_empty() {
        format!("no image named {name:?} in the dyld shared cache")
    } else {
        format!("no image named {name:?} in the dyld shared cache; close: {}", near.join(", "))
    }
}

/// The retained-byte allowance's floor (see [`DyldCache::charge`]), so a small
/// cache is not limited by its own size.
const MIN_RETAINED: u64 = 1 << 28;

/// The retained size of one parsed [`extract::Nlist`], its name aside.
const NLIST_RETAINED: u64 = std::mem::size_of::<extract::Nlist>() as u64;

/// Bytes read past the last string of a group, extended until that string ends.
const NAME_TAIL: u64 = 4096;

/// Two names closer than this are read in one span.
const GROUP_GAP: u64 = 16 * 1024;

/// The pieces of a large string area that a set of offsets name. Cache images
/// share one symbol-string pool of tens of megabytes with their names
/// scattered through it, and image paths lie inside the images themselves, so
/// the area is read in spans around the strings actually used.
pub(crate) struct StringPool {
    spans: Vec<(u64, Vec<u8>)>,
}

impl StringPool {
    /// Read, through `read(offset, len)`, the spans of a `size`-byte pool that
    /// `nlists` name.
    pub(crate) fn read(
        read: &dyn Fn(u64, usize) -> Result<Vec<u8>, String>,
        size: u64,
        nlists: &[u8],
        charge: &dyn Fn(u64) -> Result<(), String>,
    ) -> Result<StringPool, String> {
        Self::read_at(read, size, nlists.chunks_exact(16).map(|n| u32_at(n, 0) as u64), charge)
    }

    /// Read the spans of a `size`-byte pool around the strings at `offsets`.
    /// A string that runs past its group's tail is extended through `charge`, so
    /// unterminated strings cannot multiply the pool across calls.
    pub(crate) fn read_at(
        read: &dyn Fn(u64, usize) -> Result<Vec<u8>, String>,
        size: u64,
        offsets: impl Iterator<Item = u64>,
        charge: &dyn Fn(u64) -> Result<(), String>,
    ) -> Result<StringPool, String> {
        let mut strx: Vec<u64> = offsets.filter(|&s| s > 0 && s < size).collect();
        strx.sort_unstable();
        strx.dedup();
        let mut spans = Vec::new();
        let mut i = 0;
        while i < strx.len() {
            let lo = strx[i];
            let mut last = lo;
            while i < strx.len() && strx[i] - last <= GROUP_GAP {
                last = strx[i];
                i += 1;
            }
            let mut end = (last + NAME_TAIL).min(size);
            let mut bytes = read(lo, (end - lo) as usize)?;
            let mut scanned = (last - lo) as usize;
            while end < size && !bytes[scanned..].contains(&0) {
                scanned = bytes.len();
                let more = (end + NAME_TAIL).min(size);
                charge(more - end)?;
                bytes.extend(read(end, (more - end) as usize)?);
                end = more;
            }
            spans.push((lo, bytes));
        }
        Ok(StringPool { spans })
    }

    pub(crate) fn get_at(&self, offset: u64) -> &[u8] {
        self.get_at_max(offset, usize::MAX)
    }

    /// The string at `offset`, looking at most `max` bytes for its end.
    pub(crate) fn get_at_max(&self, offset: u64, max: usize) -> &[u8] {
        let at = self.spans.partition_point(|(lo, _)| *lo <= offset);
        let Some((lo, bytes)) = at.checked_sub(1).map(|i| &self.spans[i]) else {
            return b"";
        };
        let Some(s) = bytes.get((offset - lo) as usize..) else {
            return b"";
        };
        let s = &s[..s.len().min(max)];
        &s[..s.iter().position(|&b| b == 0).unwrap_or(s.len())]
    }

    pub(crate) fn get(&self, strx: u32) -> &[u8] {
        self.get_at(strx as u64)
    }
}

#[cfg(test)]
mod tests;

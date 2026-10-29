//! (kuna) The dyld shared cache: open a cache and its sub-caches, list its images,
//! and rebuild one image as a standalone thin Mach-O at its cache addresses.
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
//! input is an ordinary Mach-O. See [`extract`] for the rebuild and [`xrefs`] for
//! how calls into other images are named.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

pub mod extract;
pub mod slide;
pub mod xrefs;

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

/// The image the run selected: the `dyldimage` option, carried in
/// [`kuna_decomp::kuna_dyldcache::DYLDIMAGE_ENV`].
fn selected_image_name() -> Option<String> {
    std::env::var(kuna_decomp::kuna_dyldcache::DYLDIMAGE_ENV)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

/// The selected image of the dyld shared cache at `path`, rebuilt as a thin
/// Mach-O; `None` when `path` is not a dyld shared cache.
pub fn read_selected_image(path: &Path) -> Option<Result<Vec<u8>, String>> {
    if !is_dyld_cache_file(path) {
        return None;
    }
    Some(DyldCache::open(path).and_then(|cache| {
        let index = cache.select(selected_image_name().as_deref())?;
        cache.extract(index)
    }))
}

/// The byte size the selected image's segments occupy, `__LINKEDIT` aside
/// (the cache shares one across all images): the cache counterpart of a
/// file's length for size-based policy. `None` when `path` is not a cache.
pub fn selected_image_size(path: &Path) -> Option<Result<u64, String>> {
    if !is_dyld_cache_file(path) {
        return None;
    }
    Some(DyldCache::open(path).and_then(|cache| {
        let index = cache.select(selected_image_name().as_deref())?;
        let image = cache.image_layout(index)?;
        Ok(image.segments.iter().filter(|s| s.name != "__LINKEDIT").map(|s| s.filesize).sum())
    }))
}

/// One file of a split cache: the main file, a sub-cache, or `.symbols`.
struct CacheFile {
    file: File,
    name: String,
}

impl CacheFile {
    fn read_at(&self, offset: u64, len: usize) -> Result<Vec<u8>, String> {
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
        va >= self.address && va.saturating_add(len) <= self.address + self.size
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
    slides: RefCell<HashMap<usize, Option<slide::SlideInfo>>>,
    layouts: RefCell<HashMap<usize, Option<std::rc::Rc<extract::ImageLayout>>>>,
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
    Ok(table
        .chunks_exact(stride)
        .enumerate()
        .map(|(i, entry)| {
            let suffix = if named {
                let raw = &entry[24..56];
                String::from_utf8_lossy(&raw[..raw.iter().position(|&b| b == 0).unwrap_or(32)]).into_owned()
            } else {
                format!(".{}", i + 1)
            };
            let mut name = path.as_os_str().to_owned();
            name.push(&suffix);
            (PathBuf::from(name), entry[..16].try_into().unwrap())
        })
        .collect())
}

impl DyldCache {
    /// Open the cache whose main file is `path`, with every sub-cache and the
    /// `.symbols` file the header names.
    pub fn open(path: &Path) -> Result<DyldCache, String> {
        let open = |p: &Path| -> Result<CacheFile, String> {
            let file = File::open(p).map_err(|e| format!("{}: {e}", p.display()))?;
            Ok(CacheFile { file, name: p.display().to_string() })
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
        Ok(DyldCache {
            files,
            mappings,
            images,
            base,
            locals,
            slides: RefCell::default(),
            layouts: RefCell::default(),
        })
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
                let Ok(sym) = open(Path::new(&name)) else {
                    return Ok(None);
                };
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
        let table = files[file].read_at(sh_offset + u32_at(&info, 16) as u64, count * entry_size)?;
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
            nlist_offset: sh_offset + u32_at(&info, 0) as u64,
            strings_offset: sh_offset + u32_at(&info, 8) as u64,
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
        let (Some(lo), Some(hi)) =
            (rows.iter().map(|r| r.1).min(), rows.iter().map(|r| r.1).max())
        else {
            return Ok(Vec::new());
        };
        let pool = main.read_at(lo, (hi - lo) as usize + 1024)?;
        Ok(rows
            .into_iter()
            .map(|(address, name_off)| {
                let s = &pool[(name_off - lo) as usize..];
                let end = s.iter().position(|&b| b == 0).unwrap_or(s.len());
                CacheImage { address, path: String::from_utf8_lossy(&s[..end]).into_owned() }
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
    /// basename, else a unique basename without its extension. With no name, a
    /// cache of one image selects it.
    pub fn select(&self, name: Option<&str>) -> Result<usize, String> {
        let Some(name) = name else {
            return match self.images.len() {
                1 => Ok(0),
                n => Err(format!(
                    "dyld shared cache holds {n} images; choose one with --image NAME \
                     (list them with `kuna unpack <cache> --list`)"
                )),
            };
        };
        let base = |p: &str| p.rsplit('/').next().unwrap_or(p).to_string();
        let stem = |p: &str| {
            let b = base(p);
            b.split_once('.').map(|(s, _)| s.to_string()).unwrap_or(b)
        };
        let tiers: [&dyn Fn(&CacheImage) -> bool; 3] = [
            &|i| i.path == name,
            &|i| base(&i.path) == name,
            &|i| stem(&i.path) == name,
        ];
        for matches in tiers {
            let hits: Vec<usize> =
                self.images.iter().enumerate().filter(|(_, i)| matches(i)).map(|(n, _)| n).collect();
            match hits.len() {
                0 => continue,
                1 => return Ok(hits[0]),
                _ => {
                    let names: Vec<&str> = hits.iter().map(|&n| self.images[n].path.as_str()).collect();
                    return Err(format!("image name {name:?} is ambiguous: {}", names.join(", ")));
                }
            }
        }
        let lower = name.to_ascii_lowercase();
        let near: Vec<&str> = self
            .images
            .iter()
            .filter(|i| i.path.to_ascii_lowercase().contains(&lower))
            .take(8)
            .map(|i| i.path.as_str())
            .collect();
        Err(if near.is_empty() {
            format!("no image named {name:?} in the dyld shared cache")
        } else {
            format!("no image named {name:?} in the dyld shared cache; close: {}", near.join(", "))
        })
    }

    fn mapping_of(&self, va: u64, len: u64) -> Option<&Mapping> {
        self.mappings.iter().find(|m| m.contains(va, len))
    }

    /// The raw bytes at `[va, va + len)`, which must lie in one mapping.
    pub fn read_va(&self, va: u64, len: usize) -> Result<Vec<u8>, String> {
        let m = self
            .mapping_of(va, len as u64)
            .ok_or_else(|| format!("address range {va:#x}+{len:#x} is not mapped by the cache"))?;
        self.files[m.file].read_at(m.file_offset + (va - m.address), len)
    }

    /// The bytes at `[va, va + len)` with the slide info of their mapping applied
    /// at slide zero, so every rebased pointer reads as its unslid target.
    pub fn read_slid(&self, va: u64, len: usize) -> Result<Vec<u8>, String> {
        let m = self
            .mapping_of(va, len as u64)
            .ok_or_else(|| format!("address range {va:#x}+{len:#x} is not mapped by the cache"))?
            .clone();
        let index = self.mappings.iter().position(|x| x.address == m.address).unwrap_or(0);
        let Some(info) = self.slide_info(index)? else {
            return self.read_va(va, len);
        };
        let page = info.page_size as u64;
        let lo = m.address + (va - m.address) / page * page;
        let hi = (m.address + (va + len as u64 - m.address).div_ceil(page) * page).min(m.address + m.size);
        let mut pages = self.read_va(lo, (hi - lo) as usize)?;
        info.apply(&mut pages, ((lo - m.address) / page) as usize);
        let start = (va - lo) as usize;
        Ok(pages[start..start + len].to_vec())
    }

    fn slide_info(&self, mapping: usize) -> Result<Option<slide::SlideInfo>, String> {
        if let Some(cached) = self.slides.borrow().get(&mapping) {
            return Ok(cached.clone());
        }
        let m = &self.mappings[mapping];
        let info = if m.slide_size == 0 {
            None
        } else {
            let raw = self.files[m.file].read_at(m.slide_offset, m.slide_size as usize)?;
            slide::SlideInfo::parse(&raw)?
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
            address - self.base
        } else {
            match self.mapping_of(address, 1) {
                Some(m) => m.file_offset + (address - m.address),
                None => return Ok(Vec::new()),
            }
        };
        let Some(&(start, count)) = locals.entries.get(&key) else {
            return Ok(Vec::new());
        };
        let file = &self.files[locals.file];
        let raw = file.read_at(locals.nlist_offset + start as u64 * 16, count as usize * 16)?;
        let pool = StringPool::read(
            &|off, len| file.read_at(locals.strings_offset + off, len),
            locals.strings_size,
            &raw,
        )?;
        Ok(extract::Nlist::parse_all(&raw, &pool))
    }
}

/// Bytes read past each string a group of nlists names, extended until the
/// last name in the group ends.
const NAME_TAIL: u64 = 4096;

/// Two names closer than this are read in one span.
const GROUP_GAP: u64 = 16 * 1024;

/// The pieces of a shared string pool that one nlist array names. Cache images
/// share one pool of tens of megabytes, and an image's names are scattered
/// through it, so the pool is read in spans around the names actually used.
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
    ) -> Result<StringPool, String> {
        let mut strx: Vec<u64> =
            nlists.chunks_exact(16).map(|n| u32_at(n, 0) as u64).filter(|&s| s > 0 && s < size).collect();
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
            while end < size && !bytes[(last - lo) as usize..].contains(&0) {
                let more = (end + NAME_TAIL).min(size);
                bytes.extend(read(end, (more - end) as usize)?);
                end = more;
            }
            spans.push((lo, bytes));
        }
        Ok(StringPool { spans })
    }

    pub(crate) fn get(&self, strx: u32) -> &[u8] {
        let strx = strx as u64;
        let at = self.spans.partition_point(|(lo, _)| *lo <= strx);
        let Some((lo, bytes)) = at.checked_sub(1).map(|i| &self.spans[i]) else {
            return b"";
        };
        let Some(s) = bytes.get((strx - lo) as usize..) else {
            return b"";
        };
        &s[..s.iter().position(|&b| b == 0).unwrap_or(s.len())]
    }
}

#[cfg(test)]
mod tests;

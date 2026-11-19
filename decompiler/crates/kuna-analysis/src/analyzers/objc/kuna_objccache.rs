//! The dyld shared cache behind an image the Objective-C pass reads.
//!
//! An image rebuilt out of a cache keeps its own sections, but its selector
//! strings, its `__objc_selrefs` targets and the classes of other images lie in
//! the cache's shared regions, and its method lists offset their selectors from
//! the cache's selector base. [`CacheReader`] gives the pass those bytes, one
//! slid page at a time, each page read once.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use crate::loader::kuna_dyldcache::{is_dyld_cache_file, DyldCache};

const PAGE: u64 = 0x4000;

/// The pages kept at once; past it the reader starts over.
const MAX_PAGES: usize = 4096;

pub struct CacheReader {
    cache: DyldCache,
    pages: RefCell<HashMap<u64, Option<Rc<[u8]>>>>,
    /// Pages still to be read from the cache files: twice the cache's own.
    left: Cell<u64>,
}

impl CacheReader {
    /// The cache `path` opens, when the image under analysis was loaded from one.
    pub fn open(path: &Path) -> Option<CacheReader> {
        let cache = is_dyld_cache_file(path).then(|| DyldCache::open(path).ok()).flatten()?;
        let mapped = cache.mappings().iter().fold(0u64, |n, m| n.saturating_add(m.size));
        Some(CacheReader { cache, pages: RefCell::default(), left: Cell::new((mapped / PAGE).saturating_mul(2).max(64)) })
    }

    pub fn selector_base(&self) -> Option<u64> {
        self.cache.objc_selector_base()
    }

    /// Up to `n` bytes at `va`, pointers unslid: fewer where the mapping holding
    /// `va` ends first, `None` where nothing is mapped.
    pub fn read(&self, va: u64, n: usize) -> Option<Vec<u8>> {
        let mut out = Vec::new();
        let mut at = va;
        while out.len() < n {
            let Some((start, page)) = self.page(at) else { break };
            let from = (at - start) as usize;
            let take = (n - out.len()).min(page.len() - from);
            out.extend_from_slice(&page[from..from + take]);
            let Some(next) = at.checked_add(take as u64) else { break };
            at = next;
        }
        (!out.is_empty()).then_some(out)
    }

    /// The slid page holding `va` and its address. Pages are counted from the
    /// start of their mapping, as the slide info counts them.
    fn page(&self, va: u64) -> Option<(u64, Rc<[u8]>)> {
        let m = self.cache.mappings().iter().find(|m| va >= m.address && va - m.address < m.size)?;
        let start = m.address + (va - m.address) / PAGE * PAGE;
        if let Some(page) = self.pages.borrow().get(&start) {
            return page.clone().map(|p| (start, p));
        }
        let len = PAGE.min(m.size - (start - m.address)) as usize;
        let page = self.spend().then(|| self.cache.read_slid(start, len).ok()).flatten().map(Rc::from);
        let mut pages = self.pages.borrow_mut();
        if pages.len() >= MAX_PAGES {
            pages.clear();
        }
        pages.insert(start, page.clone());
        page.map(|p| (start, p))
    }

    fn spend(&self) -> bool {
        let left = self.left.get();
        self.left.set(left.saturating_sub(1));
        left != 0
    }
}

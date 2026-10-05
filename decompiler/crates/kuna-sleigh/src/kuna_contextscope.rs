//! Context transactions for speculative decoding, preserving values and write boundaries.

use std::cell::RefCell;

use kuna_base::error::KunaResult;

use crate::globalcontext::{ContextCache, ContextDatabase};

/// Decode against a private copy; restore the original database and cache on
/// drop, unless the caller commits the scope. Nested scopes restore their parent.
pub struct ContextScope<'a> {
    database: &'a RefCell<Box<dyn ContextDatabase>>,
    cache: &'a RefCell<ContextCache>,
    saved: Option<(Box<dyn ContextDatabase>, ContextCache)>,
}

impl<'a> ContextScope<'a> {
    pub(crate) fn new(
        database: &'a RefCell<Box<dyn ContextDatabase>>,
        cache: &'a RefCell<ContextCache>,
    ) -> Self {
        let private = database.borrow().clone_for_speculation();
        let saved_cache = cache.borrow().clone();
        let original = database.replace(private);
        Self {
            database,
            cache,
            saved: Some((original, saved_cache)),
        }
    }

    /// Prevent translation from changing one variable inside this scope. Other
    /// commits, such as Thumb IT state needed by subsequent instructions, remain enabled.
    pub fn protect_variable(&self, name: &[u8]) -> KunaResult<()> {
        let var = self.database.borrow().get_variable(name)?;
        let word = var.get_word() as usize;
        let mut cache = self.cache.borrow_mut();
        let mask = cache.set_write_mask(word, u32::MAX);
        cache.set_write_mask(word, mask & !(var.get_mask() << var.get_shift()));
        Ok(())
    }

    /// Temporarily probe the context from before this scope's walk. Dropping
    /// the returned scope restores the current speculative database and cache.
    pub fn probe_original(&self) -> ContextScope<'_> {
        let (database, cache) = self.saved.as_ref().unwrap();
        let saved = (
            self.database.replace(database.clone_for_speculation()),
            self.cache.replace(cache.clone()),
        );
        ContextScope {
            database: self.database,
            cache: self.cache,
            saved: Some(saved),
        }
    }

    /// Keep this scope's database and cache changes.
    pub fn commit(mut self) {
        self.saved = None;
    }
}

impl Drop for ContextScope<'_> {
    fn drop(&mut self) {
        if let Some((database, cache)) = self.saved.take() {
            self.database.replace(database);
            self.cache.replace(cache);
        }
    }
}

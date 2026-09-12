//! (kuna) Rebuilding a decode-equivalent SLEIGH engine away from the
//! `Architecture` that owns the original.
//!
//! A `Sleigh` cannot be shared or cloned: it holds `Rc`s throughout, and an
//! `AddrSpace` is nine `Cell`s. What CAN be carried is the instructions for
//! building another one — the `.sla` bytes, the `.ldefs` space truncations, the
//! context database's values, and the image bytes behind an `Arc` — all of them
//! plain data. [`EngineRecipe`] is the first two;
//! [`kuna_sleigh::kuna_ctxsnapshot::ContextValueSnapshot`] is the third;
//! [`ImageBytes`] is the fourth.
//!
//! The engine this builds is a bare [`Sleigh`], not an `Architecture`: a decode
//! needs `&dyn Translate` and a code-space handle and nothing else, so the type
//! factory, the action pool and the cspec layer are deliberately not
//! reproduced. It is built with `allow_context_set(false)` — belt and braces on
//! a language where `SleighBase::has_context_commits()` is false, since there
//! is then nothing for a decode to write.

use std::rc::Rc;
use std::sync::Arc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::AddrSpace;
use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::kuna_ctxsnapshot::{restore_context, ContextValueSnapshot};
use kuna_sleigh::kuna_sharedbytes::SharedBytesImage;
use kuna_sleigh::loadimage::{ImageBytes, LoadImage};
use kuna_sleigh::sleigh::Sleigh;

/// Everything needed to rebuild a decode-equivalent SLEIGH engine. Plain data
/// throughout, so it is `Send + Sync`.
#[derive(Clone, Debug)]
pub struct EngineRecipe {
    /// The architecture id the original was built for.
    pub archid: Arc<str>,
    /// The resolved `.sla` bytes, exactly as `build_translator` consumed them.
    pub sla: Arc<[u8]>,
    /// The active language's `.ldefs` `<truncate_space>` records, in ldefs order
    /// (C++ `SleighArchitecture::modifySpaces`).
    pub truncations: Arc<[(String, u32)]>,
}

// The recipe exists to cross a thread boundary; assert that at compile time
// rather than trusting the field list to stay plain data.
const _: fn() = || {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<EngineRecipe>();
};

/// The loader a [`Sleigh`] is constructed over before its real image is
/// attached (the `NullLoad` placeholder shape the console bootstrap uses).
#[derive(Debug)]
struct UnattachedImage;

impl LoadImage for UnattachedImage {
    fn get_file_name(&self) -> &str {
        ""
    }
    fn load_fill(&mut self, _ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
        Err(KunaError::data_unavail(format!(
            "decode engine: no image attached (read at {:#x})",
            addr.get_offset()
        )))
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

/// Build a decode-equivalent engine from `recipe` over `bytes`, with `ctx`'s
/// context values.
///
/// Returns the engine and its OWN default code space: space identity is
/// `Rc::ptr_eq` everywhere it matters (`load_fill`'s space check, the context
/// cache), so every `Address` handed to this engine must be built in the space
/// it returns here, never in the parent's.
pub fn build_decode_engine(
    recipe: &EngineRecipe,
    bytes: Arc<dyn ImageBytes>,
    ctx: &ContextValueSnapshot,
) -> KunaResult<(Sleigh, Rc<AddrSpace>)> {
    let mut sleigh = Sleigh::new(Box::new(UnattachedImage), Box::new(ContextInternal::new()));
    sleigh.initialize_from_sla(&recipe.sla)?;

    // The `.ldefs` `<truncate_space>` records, as `modify_spaces` applies them
    // to the parent: the address size a space reports is part of how it decodes.
    for (name, size) in recipe.truncations.iter() {
        sleigh.base().manager().truncate_space(name, *size)?;
    }

    sleigh.install_register_lookup()?;

    let code_space = sleigh
        .base()
        .manager()
        .get_default_code_space()
        .map(Rc::clone)
        .ok_or_else(|| KunaError::lowlevel("decode engine: the .sla declares no code space"))?;

    let mut image = SharedBytesImage::new(&recipe.archid, bytes);
    image.attach_to_space(Rc::clone(&code_space));
    sleigh.set_loader(Box::new(image));

    sleigh.with_context_db_mut(|db| restore_context(db, ctx, &code_space));
    sleigh.allow_context_set(false);

    Ok((sleigh, code_space))
}

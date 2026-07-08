//! Port of `decompiler/cpp/raw_arch.{cc,hh}` (item `w4-fw-arch-frontends`) — the
//! bare-bones capability for treating a file as a raw executable image.
//!
//! [`RawBinaryArchitecture`] is a [`SleighArchitecture`](crate::sleigh_arch)
//! leaf: it overrides `buildLoader` (open the raw file as a
//! [`RawLoadImage`](kuna_sleigh::loadimage::RawLoadImage) + apply `adjustvma`),
//! `resolveArchitecture` (`archid = getTarget()` — nothing is derivable from a
//! raw image), and `postSpecFile` (attach the default code space to the loader).
//!
//! [`RawBinaryArchitectureCapability`] is the catch-all extension point: it
//! matches *any* file (`isFileMatch` always returns `true`, which is why
//! `sortCapabilities` pushes "raw" last) and an XML document whose root element
//! is named `raw_savefile`.
//!
//! ## Wired vs. seamed
//!
//! - **Wired**: the capability (`isFileMatch`/`isXmlMatch`/`buildArchitecture`),
//!   the raw-image `buildLoader` (open + `adjustvma`), the `resolveArchitecture`
//!   override (`archid = target` then the base resolve), and the `restoreXml`
//!   document-walk (`adjustvma` + the `coretypes` child registration).
//! - **Seamed**: `init(store)` and `encode` (the W6/W8 type-factory/print/
//!   spec-decode + `Encoder` flow), and the `postSpecFile`
//!   `Architecture::postSpecFile` chain.  The `attachToSpace(getDefaultCodeSpace())`
//!   tail of `postSpecFile` is wired (it needs only the built translator's
//!   default code space).

use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::ElementId;
use kuna_base::space::AddrSpace;
use kuna_base::xml::Element;

use kuna_sleigh::loadimage::{LoadImage, RawLoadImage};

use crate::capability::ArchitectureCapability;
use crate::sleigh_arch::SleighArchitecture;
use crate::xml_arch::parse_auto_base;

/// Marshaling element \<raw_savefile> (C++ `ELEM_RAW_SAVEFILE`,
/// `raw_arch.cc:20`).
pub const ELEM_RAW_SAVEFILE: ElementId = ElementId::new("raw_savefile", 237);

// ---------------------------------------------------------------------------
// RawBinaryArchitectureCapability (raw_arch.hh:29)
// ---------------------------------------------------------------------------

/// \brief Extension point for building an Architecture that reads in raw images
/// (C++ `class RawBinaryArchitectureCapability : public ArchitectureCapability`,
/// `raw_arch.hh:29`).
///
/// In C++ a single static singleton registers itself; per the kuna no-static
/// convention the bootstrap constructs and `register`s this with the
/// [`CapabilityRegistry`](crate::capability::CapabilityRegistry).  Because
/// `isFileMatch` always succeeds, `sortCapabilities` moves "raw" to the end of
/// the search order.
#[derive(Debug, Clone, Default)]
pub struct RawBinaryArchitectureCapability;

impl RawBinaryArchitectureCapability {
    /// Construct the capability (C++ singleton ctor sets `name = "raw"`).
    pub fn new() -> RawBinaryArchitectureCapability {
        RawBinaryArchitectureCapability
    }

    /// Build a fresh [`RawBinaryArchitecture`] over the given executable (C++
    /// `buildArchitecture`, `raw_arch.cc:37`).
    pub fn build_architecture(&self, filename: &str, target: &str) -> RawBinaryArchitecture {
        RawBinaryArchitecture::new(filename, target)
    }
}

impl ArchitectureCapability for RawBinaryArchitectureCapability {
    /// The capability identifier (C++ `name = "raw"`).
    fn get_name(&self) -> &str {
        "raw"
    }

    /// Determine if this extension can handle the file at `filename` (C++
    /// `isFileMatch`, `raw_arch.cc:43` — always `true`: a file can always be
    /// opened as a raw binary).
    fn is_file_match(&self, _filename: &str) -> bool {
        true // File can always be opened as raw binary
    }

    /// Determine if this extension can handle a parsed XML document whose root
    /// element is named `root_name` (C++ `isXmlMatch`, `raw_arch.cc:49` —
    /// `doc->getRoot()->getName() == "raw_savefile"`).
    fn is_xml_match(&self, root_name: &str) -> bool {
        root_name == "raw_savefile"
    }
}

// ---------------------------------------------------------------------------
// RawBinaryArchitecture (raw_arch.hh:42)
// ---------------------------------------------------------------------------

/// \brief Architecture that reads its binary as a raw file (C++ `class
/// RawBinaryArchitecture : public SleighArchitecture`, `raw_arch.hh:42`).
///
/// Models the C++ leaf-of-`SleighArchitecture` by composition: owns a
/// [`SleighArchitecture`] (`sleigh`) plus the raw-specific `adjustvma` and the
/// loaded [`RawLoadImage`].
pub struct RawBinaryArchitecture {
    /// The composed `SleighArchitecture` base (the C++ base subobject).
    sleigh: SleighArchitecture,
    /// What address byte 0 of the raw file gets treated as (C++ `adjustvma`).
    adjustvma: i64,
    /// The image loader, once [`Self::build_loader`] has opened the raw file
    /// (C++ `loader`, a `RawLoadImage *` held by the base).
    loader: Option<RawLoadImage>,
}

impl RawBinaryArchitecture {
    /// \brief Constructor (C++ `RawBinaryArchitecture::RawBinaryArchitecture`,
    /// `raw_arch.cc:80`).  Wraps the base constructor and zeroes `adjustvma`.
    pub fn new(fname: &str, targ: &str) -> RawBinaryArchitecture {
        RawBinaryArchitecture {
            sleigh: SleighArchitecture::new(fname, targ),
            adjustvma: 0,
            loader: None,
        }
    }

    /// Borrow the composed `SleighArchitecture` base.
    pub fn sleigh(&self) -> &SleighArchitecture {
        &self.sleigh
    }

    /// Mutably borrow the composed `SleighArchitecture` base.
    pub fn sleigh_mut(&mut self) -> &mut SleighArchitecture {
        &mut self.sleigh
    }

    /// The VMA adjustment (C++ `adjustvma`).
    pub fn adjustvma(&self) -> i64 {
        self.adjustvma
    }

    /// Borrow the loaded image, if [`Self::build_loader`] has run.
    pub fn loader(&self) -> Option<&RawLoadImage> {
        self.loader.as_ref()
    }

    /// Take ownership of the loaded image (so it can be handed to the engine via
    /// `Sleigh::set_loader`).  Leaves [`Self::loader`] empty.
    pub fn take_loader(&mut self) -> Option<RawLoadImage> {
        self.loader.take()
    }

    /// \brief Build the image loader (C++ `RawBinaryArchitecture::buildLoader`,
    /// `raw_arch.cc:55`).
    ///
    /// Opens the executable file as a [`RawLoadImage`] and applies `adjustvma`.
    /// The C++ also calls `collectSpecFiles(*errorstream)` first; that `.ldefs`
    /// scan is the caller's job (it owns the
    /// [`LanguageDatabase`](crate::sleigh_arch::LanguageDatabase)).
    ///
    /// NOTE (C++ UB corner, faithfully preserved): the C++ calls
    /// `ldr->adjustVma(adjustvma)` HERE — before `attachToSpace` (which
    /// `postSpecFile` does) — yet `RawLoadImage::adjustVma` dereferences
    /// `spaceid->getWordSize()`, which is null until the attach
    /// (`loadimage.cc:79`).  A raw build with a nonzero `adjustvma` is therefore
    /// a null-deref in C++; the kuna-sleigh port encodes that UB as a panic
    /// (ADR 0004).  This transcription calls `adjust_vma` in the same place, so
    /// the C++ semantics (including the UB on the `adjustvma != 0` save-file
    /// path) are preserved exactly.  See `docs/rust-port/losses.md`.
    pub fn build_loader(&mut self) -> KunaResult<()> {
        // unique_ptr<RawLoadImage> ldr(new RawLoadImage(getFilename())); ldr->open();
        let mut ldr = RawLoadImage::new(self.sleigh.get_filename());
        ldr.open()?;
        // if (adjustvma != 0) ldr->adjustVma(adjustvma);  (C++ UB if no space yet)
        if self.adjustvma != 0 {
            ldr.adjust_vma(self.adjustvma);
        }
        self.loader = Some(ldr);
        Ok(())
    }

    /// \brief Resolve the architecture (C++
    /// `RawBinaryArchitecture::resolveArchitecture`, `raw_arch.cc:66`).
    ///
    /// `archid = getTarget()` — nothing can be derived from a raw image, so the
    /// passed-in target is copied in, then the base
    /// [`SleighArchitecture::resolve_architecture`] runs.  Because `archid` is
    /// already set, the base resolve skips the loader-`getArchType` branch (so
    /// `arch_type` is unused for a raw image — passed `""`).
    pub fn resolve_architecture(
        &mut self,
        db: &crate::sleigh_arch::LanguageDatabase,
    ) -> KunaResult<()> {
        // archid = getTarget(); -- set archid so the base resolve copies it
        // verbatim (its `if (archid.empty())` branch is skipped).  We route the
        // target into archid via the base by setting target as the source; the
        // base resolve's empty-archid path uses target when target != "default".
        // To match C++ exactly (archid = target, NOT via the loader), pre-seed
        // archid through a dedicated setter on the base.
        let target = self.sleigh.get_target().to_string();
        self.sleigh.set_archid(&target);
        // SleighArchitecture::resolveArchitecture();
        self.sleigh.resolve_architecture(db, "")
    }

    /// \brief Attach the default code space to the loader (the wired tail of C++
    /// `RawBinaryArchitecture::postSpecFile`, `raw_arch.cc:73`).
    ///
    /// `((RawLoadImage *)loader)->attachToSpace(getDefaultCodeSpace());`
    ///
    /// STUB(W4): the C++ `postSpecFile` first calls `Architecture::postSpecFile`
    /// (the segmented-pointer + read-only chain); only the attach tail is wired
    /// here.  (The C++ `postSpecFile` does NOT re-apply `adjustvma` — that is done
    /// once in `buildLoader`; see [`Self::build_loader`].)
    pub fn attach_default_space(&mut self, default_code_space: Rc<AddrSpace>) -> KunaResult<()> {
        let loader = self.loader.as_mut().ok_or_else(|| {
            KunaError::lowlevel("RawBinaryArchitecture::postSpecFile: loader not built")
        })?;
        // ((RawLoadImage *)loader)->attachToSpace(getDefaultCodeSpace());
        loader.attach_to_space(default_code_space);
        Ok(())
    }

    /// \brief Restore the raw save-file state, walking the `<raw_savefile>`
    /// children (C++ `RawBinaryArchitecture::restoreXml`, `raw_arch.cc:97`).
    ///
    /// Reads the header (`name`/`target`), parses `adjustvma`, and registers the
    /// optional `coretypes` child tag (the only child a raw save-file carries
    /// before the base state).  Returns the recognized child tags (in C++ order)
    /// plus the trailing base-state element.
    pub fn restore_xml(
        &mut self,
        name: &str,
        target: &str,
        adjustvma_str: &str,
        children: &[Rc<Element>],
    ) -> KunaResult<RawRestore> {
        // restoreXmlHeader(el);
        self.sleigh.restore_xml_header(name, target);
        // istringstream s(adjustvma); s.unsetf(dec|hex|oct); s >> adjustvma;
        self.adjustvma = parse_auto_base(adjustvma_str);

        let mut iter = children.iter();
        let mut registered: Vec<Rc<Element>> = Vec::new();
        let mut next = iter.next();
        // if ((*iter)->getName() == "coretypes") { register; ++iter; }
        if let Some(el) = next {
            if el.get_name() == "coretypes" {
                registered.push(Rc::clone(el));
                next = iter.next();
            }
        }
        // init(store);  -- STUB(W6/W8)
        // if (iter != list.end()) { register *iter; SleighArchitecture::restoreXml(store); }
        let rest = next.map(Rc::clone);
        Ok(RawRestore { to_register: registered, rest })
    }
}

/// The result of [`RawBinaryArchitecture::restore_xml`]: the child tags to
/// register into the document store (in C++ order) plus the trailing base-state
/// element.
#[derive(Debug, Clone, Default)]
pub struct RawRestore {
    /// The recognized `coretypes` child tag, in document order, for the caller
    /// to register into its `DocumentStorage`.
    pub to_register: Vec<Rc<Element>>,
    /// The trailing element carrying the base `SleighArchitecture::restoreXml`
    /// state, if present.
    pub rest: Option<Rc<Element>>,
}

#[cfg(test)]
mod tests;

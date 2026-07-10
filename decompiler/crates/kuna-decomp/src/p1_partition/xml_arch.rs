//! Port of `decompiler/cpp/xml_arch.{cc,hh}` (item `w4-fw-arch-frontends`) —
//! the extension that reads executables stored in the decompiler's XML
//! `<binaryimage>` format.  **This is how the datatest corpus loads.**
//!
//! [`XmlArchitecture`] is a [`SleighArchitecture`](crate::sleigh_arch) leaf: it
//! overrides `buildLoader` (find the `<binaryimage>` tag and wrap it in a
//! [`LoadImageXml`]), `postSpecFile` (open the image against the built
//! translator + apply `adjustvma`), and adds the `<xml_savefile>`
//! `encode`/`restoreXml` marshaling around the base header.
//!
//! [`XmlArchitectureCapability`] is the registered extension point: it matches a
//! file beginning with `"<bi"` (a `<binaryimage>` tag) and an XML document whose
//! root element is named `xml_savefile`.
//!
//! ## Wired vs. stubbed
//!
//! - **Wired**: the capability (`isFileMatch`/`isXmlMatch`/`buildArchitecture`),
//!   the `<binaryimage>` extraction (`buildLoader`'s tag lookup + image
//!   construction), `adjustvma` decode, and the `restoreXml` document-walk that
//!   registers the `binaryimage`/`specextensions`/`coretypes` child tags into
//!   the `DocumentStorage`.
//! - **Stubbed**: `init(store)` (the W6/W8 `Architecture::init` flow — type
//!   factory, print language, spec decode), `encode` (the full `Encoder` +
//!   `types->encodeCoreTypes` marshaling), and the `postSpecFile` chain's
//!   `Architecture::postSpecFile` (which reaches the segmented-pointer +
//!   read-only stubs).  The image OPEN itself (`LoadImageXml::open` against the
//!   translator's manager) is wired and is exactly what the corpus bootstrap
//!   gate drives.

use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{ElementId, IdRegistry};
use kuna_base::space::AddrSpaceManager;
use kuna_base::xml::Element;

use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::LoadImageXml;

use crate::capability::ArchitectureCapability;
use crate::sleigh_arch::SleighArchitecture;

/// Marshaling element \<xml_savefile> (C++ `ELEM_XML_SAVEFILE`,
/// `xml_arch.cc:20`).
pub const ELEM_XML_SAVEFILE: ElementId = ElementId::new("xml_savefile", 236);

// ---------------------------------------------------------------------------
// XmlArchitectureCapability
// ---------------------------------------------------------------------------

/// \brief Extension for building an XML format capable Architecture (C++ `class
/// XmlArchitectureCapability : public ArchitectureCapability`, `xml_arch.hh:29`).
///
/// In C++ a single static singleton registers itself; per the kuna no-static
/// convention the bootstrap constructs and `register`s this with the
/// [`CapabilityRegistry`](crate::capability::CapabilityRegistry).
#[derive(Debug, Clone, Default)]
pub struct XmlArchitectureCapability;

impl XmlArchitectureCapability {
    /// Construct the capability (C++ singleton ctor sets `name = "xml"`).
    pub fn new() -> XmlArchitectureCapability {
        XmlArchitectureCapability
    }

    /// Build a fresh [`XmlArchitecture`] over the given executable (C++
    /// `buildArchitecture`, `xml_arch.cc:37`).
    pub fn build_architecture(&self, filename: &str, target: &str) -> XmlArchitecture {
        XmlArchitecture::new(filename, target)
    }
}

impl ArchitectureCapability for XmlArchitectureCapability {
    /// The capability identifier (C++ `name = "xml"`).
    fn get_name(&self) -> &str {
        "xml"
    }

    /// Determine if this extension can handle the file at `filename` (C++
    /// `isFileMatch`, `xml_arch.cc:43`).
    ///
    /// The C++ skips leading whitespace and reads three characters, matching
    /// `'<' 'b' 'i'` (the start of a `<binaryimage>` tag).
    fn is_file_match(&self, filename: &str) -> bool {
        let bytes = match std::fs::read(filename) {
            Ok(b) => b,
            Err(_) => return false,
        };
        // Skip leading whitespace, then read the next three chars (EOF yields the
        // C++ `istream::get()` sentinel -1, which never equals a tag char —
        // modeled here by a None that fails the match).
        let mut it = bytes.iter().copied().skip_while(|c| c.is_ascii_whitespace());
        let val1 = it.next();
        let val2 = it.next();
        let val3 = it.next();
        // Probably <binaryimage> tag
        val1 == Some(b'<') && val2 == Some(b'b') && val3 == Some(b'i')
    }

    /// Determine if this extension can handle a parsed XML document whose root
    /// element is named `root_name` (C++ `isXmlMatch`, `xml_arch.cc:60` —
    /// `doc->getRoot()->getName() == "xml_savefile"`).
    fn is_xml_match(&self, root_name: &str) -> bool {
        root_name == "xml_savefile"
    }
}

// ---------------------------------------------------------------------------
// XmlArchitecture
// ---------------------------------------------------------------------------

/// \brief An [`Architecture`](crate::architecture::Architecture) that loads
/// executables using an XML format (C++ `class XmlArchitecture : public
/// SleighArchitecture`, `xml_arch.hh:42`).
///
/// Models the C++ leaf-of-`SleighArchitecture` by composition: owns a
/// [`SleighArchitecture`] (`sleigh`) plus the XML-specific `adjustvma` and the
/// loaded [`LoadImageXml`].
pub struct XmlArchitecture {
    /// The composed `SleighArchitecture` base (the C++ base subobject).
    sleigh: SleighArchitecture,
    /// The amount to adjust the virtual memory address (C++ `adjustvma`).
    adjustvma: i64,
    /// The image loader, once [`Self::build_loader`] has wrapped the
    /// `<binaryimage>` tag (C++ `loader`, a `LoadImageXml *` held by the base).
    loader: Option<LoadImageXml>,
}

impl XmlArchitecture {
    /// \brief Constructor (C++ `XmlArchitecture::XmlArchitecture`,
    /// `xml_arch.cc:95`).
    ///
    /// Wraps the base constructor and zeroes `adjustvma`.
    /// \param fname is the path to the executable file (containing XML)
    /// \param targ is the (optional) language id
    pub fn new(fname: &str, targ: &str) -> XmlArchitecture {
        XmlArchitecture {
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

    /// Consume the wrapper, returning the composed `SleighArchitecture` base
    /// (the C++ slicing of the leaf back to its `SleighArchitecture` subobject
    /// — used once the XML-specific `loader`/`adjustvma` machinery is spent and
    /// only the built `Architecture` god object is still needed).
    pub fn into_sleigh(self) -> SleighArchitecture {
        self.sleigh
    }

    /// The VMA adjustment (C++ `adjustvma`).
    pub fn adjustvma(&self) -> i64 {
        self.adjustvma
    }

    /// Borrow the loaded image, if [`Self::build_loader`] has run.
    pub fn loader(&self) -> Option<&LoadImageXml> {
        self.loader.as_ref()
    }

    /// Take ownership of the loaded image (so it can be handed to the engine via
    /// `Sleigh::set_loader`).  Leaves [`Self::loader`] empty.
    pub fn take_loader(&mut self) -> Option<LoadImageXml> {
        self.loader.take()
    }

    /// \brief Build the image loader (C++ `XmlArchitecture::buildLoader`,
    /// `xml_arch.cc:66`).
    ///
    /// Finds the `<binaryimage>` tag (in the document store, or by opening the
    /// executable file), and wraps it in a [`LoadImageXml`].  The C++ also calls
    /// `collectSpecFiles(*errorstream)` first (so the `.ldefs` are scanned before
    /// any spec lookup); that scan is the caller's job (it owns the
    /// [`LanguageDatabase`](crate::sleigh_arch::LanguageDatabase) — see the
    /// corpus bootstrap), so this takes the already-extracted `<binaryimage>`
    /// element.
    ///
    /// `registry` validates that the element is a well-formed `<binaryimage>`
    /// (`LoadImageXml::new` checks the tag name) — the same check the C++
    /// `getTag("binaryimage")` lookup performs.  Errors if the element is not a
    /// `<binaryimage>` (C++ `throw LowlevelError("Could not find binaryimage
    /// tag")`).
    pub fn build_loader(&mut self, binaryimage: Rc<Element>) -> KunaResult<()> {
        // The document open/extract is the caller's (it owns the store); the
        // resolved <binaryimage> element is passed in.
        if binaryimage.get_name() != "binaryimage" {
            return Err(KunaError::lowlevel("Could not find binaryimage tag"));
        }
        let filename = self.sleigh.get_filename().to_string();
        self.loader = Some(LoadImageXml::new(&filename, binaryimage)?);
        Ok(())
    }

    /// \brief Open the loaded image against the built translator (the wired core
    /// of C++ `XmlArchitecture::postSpecFile`, `xml_arch.cc:82`).
    ///
    /// STUB(W4/W6/W8): the C++ `postSpecFile` first calls
    /// `Architecture::postSpecFile` (the segmented-pointer + read-only-from-loader
    /// chain); only the image-open + vma-adjust tail is wired here (it needs just
    /// the translator's address-space manager, which the built base supplies).
    /// The opened [`LoadImageXml`] is taken out so the caller can hand it to the
    /// engine via `Sleigh::set_loader` (the C++ `loader` back-pointer the engine
    /// reads on decode).
    pub fn open_image(&mut self, manager: &AddrSpaceManager, registry: &IdRegistry) -> KunaResult<()> {
        let loader = self
            .loader
            .as_mut()
            .ok_or_else(|| KunaError::lowlevel("XmlArchitecture::postSpecFile: loader not built"))?;
        loader.open(manager, registry)?;
        if self.adjustvma != 0 {
            loader.adjust_vma(self.adjustvma);
        }
        Ok(())
    }

    /// \brief Restore the XML save-file state, walking the `<xml_savefile>`
    /// children (C++ `XmlArchitecture::restoreXml`, `xml_arch.cc:116`).
    ///
    /// The C++ reads the header (`name`/`target`), parses `adjustvma`, and
    /// registers the `binaryimage`/`specextensions`/`coretypes` child tags into
    /// the document store (each consumed later by `init`/the spec decode).  This
    /// port performs the header restore + `adjustvma` parse and reports which of
    /// the three optional child tags are present (so the caller can register
    /// them into its store), in the C++ document order.
    ///
    /// `root` is the `<xml_savefile>` element; `name`/`target`/`adjustvma_str`
    /// are its attributes (the caller reads them off the element via the XML DOM,
    /// matching the C++ `el->getAttributeValue`).  Returns the ordered list of
    /// recognized child tags to register, plus the trailing "rest" element (the
    /// base `SleighArchitecture::restoreXml` state), per the C++ iterator walk.
    pub fn restore_xml(
        &mut self,
        name: &str,
        target: &str,
        adjustvma_str: &str,
        children: &[Rc<Element>],
    ) -> KunaResult<XmlRestore> {
        self.sleigh.restore_xml_header(name, target);
        self.adjustvma = parse_auto_base(adjustvma_str);

        // The C++ walks children in order: binaryimage, then specextensions, then
        // coretypes (each conditional on the name); a trailing element is the
        // base restore.
        let mut iter = children.iter();
        let mut registered: Vec<Rc<Element>> = Vec::new();
        let mut next = iter.next();
        if let Some(el) = next {
            if el.get_name() == "binaryimage" {
                registered.push(Rc::clone(el));
                next = iter.next();
            }
        }
        if let Some(el) = next {
            if el.get_name() == "specextensions" {
                registered.push(Rc::clone(el));
                next = iter.next();
            }
        }
        if let Some(el) = next {
            if el.get_name() == "coretypes" {
                registered.push(Rc::clone(el));
                next = iter.next();
            }
        }
        // init(store);  -- STUB(W6/W8): the type-factory/print/spec-decode flow.
        let rest = next.map(Rc::clone);
        Ok(XmlRestore { to_register: registered, rest })
    }
}

/// The result of [`XmlArchitecture::restore_xml`]: the child tags to register
/// into the document store (in C++ order) plus the trailing base-state element.
#[derive(Debug, Clone, Default)]
pub struct XmlRestore {
    /// The recognized `binaryimage`/`specextensions`/`coretypes` child tags, in
    /// document order, for the caller to register into its `DocumentStorage`.
    pub to_register: Vec<Rc<Element>>,
    /// The trailing element carrying the base `SleighArchitecture::restoreXml`
    /// state, if present (the C++ `if (iter != list.end())` tail).
    pub rest: Option<Rc<Element>>,
}

/// Parse an integer with auto-detected base (the C++ `istringstream` with
/// `unsetf(dec|hex|oct)` — `0x`/`0X` prefix is hex, a leading `0` is octal,
/// otherwise decimal; an optional leading sign is honored), matching
/// `strtoll(s, 0, 0)`.  Returns 0 on a fully-unparseable string, as the C++
/// `>>` leaves the target unchanged from its default 0.
pub(crate) fn parse_auto_base(s: &str) -> i64 {
    let t = s.trim();
    let (neg, body) = match t.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, t.strip_prefix('+').unwrap_or(t)),
    };
    let val: i64 = if let Some(hex) = body.strip_prefix("0x").or_else(|| body.strip_prefix("0X")) {
        i64::from_str_radix(hex, 16).unwrap_or(0)
    } else if body.len() > 1 && body.starts_with('0') {
        i64::from_str_radix(&body[1..], 8).unwrap_or(0)
    } else {
        body.parse::<i64>().unwrap_or(0)
    };
    if neg {
        val.wrapping_neg()
    } else {
        val
    }
}

#[cfg(test)]
mod tests;

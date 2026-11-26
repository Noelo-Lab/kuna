//! One builder for every bootstrap. A [`Bootstrap`] names where the languages
//! come from, then takes each setting a load can have — the target language,
//! the ARM decode mode, the load-time options — and loads one input with it.
//! Each `bootstrap_*` free function calls it with fixed settings.

use std::rc::Rc;

use kuna_base::error::KunaResult;
use kuna_base::xml::Element;
use kuna_decomp::kuna_loadoptions::{self, LoadOptions, LoadScope};

use crate::engine::{
    arm_isa_from_env, bootstrap_image, bootstrap_mz_bytes, bootstrap_object,
    bootstrap_raw_image, bootstrap_te, bootstrap_te_bytes, bootstrap_xml_image, classify_file,
    xml_root_image, ArmIsa, ConsoleProgram, FileInput, Languages,
};

/// Where a [`Bootstrap`] finds its languages: spec roots (a slice, array or
/// `Vec` of anything [`IntoSpecRoot`](kuna_decomp::kuna_memoryspecs::IntoSpecRoot)
/// takes) it scans at each load, or a
/// [`LanguageDatabase`](kuna_decomp::sleigh_arch::LanguageDatabase) a caller
/// scanned once with [`language_database`](crate::engine::language_database).
/// Kuna implements it; callers cannot.
pub trait LanguageSource: Languages {}

impl<L: Languages + ?Sized> LanguageSource for L {}

/// The settings for loading a binary into a [`ConsoleProgram`].
///
/// ```no_run
/// # use kuna_console::engine::{language_database, ArmIsa, Bootstrap};
/// # use kuna_decomp::kuna_loadoptions::LoadOptions;
/// let db = language_database(&["specs"])?;
/// let options = LoadOptions::default();
/// let load = Bootstrap::new(&db).arm_isa(Some(ArmIsa::Thumb)).options(&options);
/// let first = load.object("a.elf")?;
/// let second = load.object("b.elf")?;
/// # Ok::<(), kuna_base::error::KunaError>(())
/// ```
///
/// Each setting has a default:
///
/// - **target**: empty, so the input chooses its language. A raw image has no
///   header to choose from and needs one.
/// - **options**: none, so the load reads the process environment. Every
///   input puts the environment in force on the calling thread for the load,
///   even when another program's options are in force there, and the program
///   it builds keeps reading the environment live; an object load (or an image
///   load that reaches the object loader) also writes the image's Go version
///   to [`GOVERSION_ENV`](kuna_decomp::kuna_goreg::GOVERSION_ENV), or clears
///   it. With [`Bootstrap::options`], nothing on the load reads or writes the
///   environment, and the program keeps the options for its analysis commit
///   and decompiles.
/// - **arm_isa**: unset. Without options, an unset ISA comes from
///   [`ARM_ISA_ENV`](crate::engine::ARM_ISA_ENV); with options, it is `None`
///   (the image's own evidence decides).
///
/// The builder only borrows its settings and holds no state of its own, so
/// one builder can load many inputs, on one thread or on several. Each load
/// enters its options on the thread that runs it and leaves them when it
/// returns.
pub struct Bootstrap<'a, L: LanguageSource + ?Sized> {
    languages: &'a L,
    target: &'a str,
    arm_isa: Option<Option<ArmIsa>>,
    options: Option<&'a LoadOptions>,
}

impl<L: LanguageSource + ?Sized> Clone for Bootstrap<'_, L> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<L: LanguageSource + ?Sized> Copy for Bootstrap<'_, L> {}

impl<'a, L: LanguageSource + ?Sized> Bootstrap<'a, L> {
    /// A bootstrap over `languages`, with every other setting at its default.
    pub fn new(languages: &'a L) -> Self {
        Self {
            languages,
            target: "",
            arm_isa: None,
            options: None,
        }
    }

    /// The SLEIGH language id to load as. Empty or `default` lets the input
    /// choose; any other id overrides the language the input names. An XML
    /// input carries its own and ignores this.
    pub fn target(mut self, target: &'a str) -> Self {
        self.target = target;
        self
    }

    /// The ARM/Thumb decode mode for a 32-bit ARM target: `Some` paints the
    /// executable ranges with it, `None` leaves the image's own evidence to
    /// decide. A load whose language is not 32-bit ARM fails when this is
    /// `Some`. Not read for an XML input.
    pub fn arm_isa(mut self, isa: Option<ArmIsa>) -> Self {
        self.arm_isa = Some(isa);
        self
    }

    /// The load-time options, in place of the environment.
    pub fn options(mut self, options: &'a LoadOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// Load an object-format binary: ELF, PE, Mach-O or COFF, as
    /// [`bootstrap_from_object`](crate::engine::bootstrap_from_object) does.
    pub fn object(&self, path: &str) -> KunaResult<ConsoleProgram> {
        let isa = self.resolved_isa()?;
        match self.options {
            Some(options) => {
                bootstrap_object(path, self.target, self.languages, isa, options, false)
            }
            None => bootstrap_object(
                path,
                self.target,
                self.languages,
                isa,
                &LoadOptions::from_env(),
                true,
            ),
        }
    }

    /// Load a real image on disk, choosing the loader by its bytes: a UEFI
    /// TE, a DOS `MZ`, or an object-format binary, as
    /// [`bootstrap_from_image`](crate::engine::bootstrap_from_image) does.
    pub fn image(&self, path: &str) -> KunaResult<ConsoleProgram> {
        let isa = self.resolved_isa()?;
        match self.options {
            Some(options) => {
                bootstrap_image(path, self.target, self.languages, isa, options, false)
            }
            None => bootstrap_image(
                path,
                self.target,
                self.languages,
                isa,
                &LoadOptions::from_env(),
                true,
            ),
        }
    }

    /// Load any input `load file` accepts: a real image as [`Bootstrap::image`]
    /// takes it, or an XML document holding a `<binaryimage>`, as
    /// [`bootstrap_from_file`](crate::engine::bootstrap_from_file) does.
    pub fn file(&self, path: &str) -> KunaResult<ConsoleProgram> {
        let input = {
            let _load = self.enter();
            classify_file(path)?
        };
        match input {
            FileInput::Mz(bytes) => self.mz_bytes(path, &bytes),
            FileInput::Te(bytes) => self.te_bytes(path, &bytes),
            FileInput::Object => self.object(path),
            FileInput::Xml(root) => self.xml_root(&root),
        }
    }

    /// Load a headerless byte image mapped at `base`, its functions found from
    /// `entries`, as [`bootstrap_from_raw`](crate::engine::bootstrap_from_raw)
    /// does. Needs a target.
    pub fn raw(&self, path: &str, base: u64, entries: &[u64]) -> KunaResult<ConsoleProgram> {
        let isa = self.resolved_isa()?;
        let _load = self.enter();
        bootstrap_raw_image(path, self.target, base, Some(entries), isa, self.languages, None)
    }

    /// Load a headerless image for byte and instruction queries, without
    /// declaring a function entry or triggering function discovery.
    pub fn raw_view(&self, path: &str, base: u64) -> KunaResult<ConsoleProgram> {
        let isa = self.resolved_isa()?;
        let _load = self.enter();
        bootstrap_raw_image(path, self.target, base, None, isa, self.languages, None)
    }

    /// Load a UEFI TE image, as [`bootstrap_from_te`](crate::engine::bootstrap_from_te) does.
    pub fn te(&self, path: &str) -> KunaResult<ConsoleProgram> {
        let isa = self.resolved_isa()?;
        let _load = self.enter();
        bootstrap_te(path, self.target, self.languages, isa)
    }

    /// Load a UEFI TE image from bytes already read from `path`, as
    /// [`bootstrap_from_te_bytes`](crate::engine::bootstrap_from_te_bytes) does.
    pub fn te_bytes(&self, path: &str, bytes: &[u8]) -> KunaResult<ConsoleProgram> {
        let isa = self.resolved_isa()?;
        let _load = self.enter();
        bootstrap_te_bytes(path, bytes, self.target, self.languages, isa)
    }

    /// Load a DOS `MZ` executable from bytes already read from `path`, as
    /// [`bootstrap_from_mz_bytes`](crate::engine::bootstrap_from_mz_bytes) does.
    pub fn mz_bytes(&self, path: &str, bytes: &[u8]) -> KunaResult<ConsoleProgram> {
        let isa = self.resolved_isa()?;
        let _load = self.enter();
        bootstrap_mz_bytes(path, bytes, self.target, self.languages, isa)
    }

    /// Load a `<binaryimage>` element as language `arch_id`, as
    /// [`bootstrap_program`](crate::engine::bootstrap_program) does.
    pub fn xml_image(&self, binaryimage: Rc<Element>, arch_id: &str) -> KunaResult<ConsoleProgram> {
        let _load = self.enter();
        bootstrap_xml_image(binaryimage, arch_id, self.languages)
    }

    /// Load the `<binaryimage>` inside a parsed XML document, as language its
    /// `arch` attribute names, as
    /// [`bootstrap_from_root`](crate::engine::bootstrap_from_root) does.
    pub fn xml_root(&self, root: &Rc<Element>) -> KunaResult<ConsoleProgram> {
        let (binaryimage, arch_id) = xml_root_image(root)?;
        self.xml_image(binaryimage, &arch_id)
    }

    fn enter(&self) -> LoadScope {
        match self.options {
            Some(options) => options.enter(),
            None => kuna_loadoptions::enter_environment(),
        }
    }

    fn resolved_isa(&self) -> KunaResult<Option<ArmIsa>> {
        match (self.arm_isa, self.options) {
            (Some(isa), _) => Ok(isa),
            (None, Some(_)) => Ok(None),
            (None, None) => arm_isa_from_env(),
        }
    }
}

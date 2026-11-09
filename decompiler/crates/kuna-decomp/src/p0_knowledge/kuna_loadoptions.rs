//! (kuna) Load-time options: the settings the loader, the analysis tier and the
//! architecture build read before any `option` command can run.
//!
//! A caller fills a [`LoadOptions`] and passes it to a bootstrap that
//! takes one (`kuna_console::engine::bootstrap_from_object_with_options`). The
//! bootstrap enters a [`LoadScope`] on the loading thread, the gates read the
//! options through [`scoped`], and the new `Architecture` keeps a copy, so the
//! analysis commit and every later decompile of that program read the same
//! values. Nothing on that path reads or writes the process environment, so two
//! images can load at once with different settings.
//!
//! The environment is the CLI's channel: the `kuna` CLI exports each
//! `--option` gate onto the `decomp_dbg` subprocess, and the console's
//! `load file` turns it back into options with [`LoadOptions::from_env`]. A
//! gate read outside any scope falls back to its environment variable.

use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;

use crate::kuna_kernelcache::KernelcacheChoice;
use crate::kuna_libctypes::LibcTypesLayout;
use crate::kuna_ppc32picimports::Mode as Ppc32PicMode;
use crate::kuna_sourcelang::SourceLang;
use crate::kuna_symbolnamechars::NameChars;
use crate::options::env_toggle;

/// The `--slice` fat Mach-O slice override.
pub const MACHO_SLICE_ENV: &str = "KUNA_MACHO_SLICE";
/// The external `.pdb` file to apply.
pub const PDB_PATH_ENV: &str = "kuna_pdb_path";
/// The external `.fid` function-ID database to apply.
pub const FID_DB_ENV: &str = "kuna_fid_db";
/// The FLIRT `.sig` files, or directories of them, to match.
pub const FLIRT_SIGS_ENV: &str = "KUNA_FLIRT_SIGS";
/// `off` skips the Objective-C metadata pass at load.
pub const LOAD_OBJC_ENV: &str = "KUNA_LOAD_OBJC";
/// `off` skips the string-literal pass at load.
pub const LOAD_STRINGS_ENV: &str = "KUNA_LOAD_STRINGS";
/// `off` copies a thin Mach-O into memory instead of mapping it.
pub const LOAD_MMAP_ENV: &str = "KUNA_LOAD_MMAP";

/// Every load-time setting, one field per option. [`Default`] is the shipped
/// default of each option, the value an unset environment variable means.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadOptions {
    /// `ppc32picimports`; only `abi` acts at load time.
    pub ppc32picimports: Ppc32PicMode,
    /// `ifuncfpret`: name x86-64 IFUNC PLT stubs.
    pub ifuncfpret: bool,
    /// `ppcapu`: select the PowerPC APU language for an image that needs it.
    pub ppcapu: bool,
    /// `kernelcache`: unwrap Apple firmware containers and pick a fileset entry.
    pub kernelcache: KernelcacheChoice,
    /// `goabi`: select the golang compiler spec for a Go image.
    pub goabi: bool,
    /// `symbolnamechars`: how symbol-name characters are sanitized.
    pub symbolnamechars: NameChars,
    /// `libctypes`: the named libc aggregate types.
    pub libctypes: LibcTypesLayout,
    /// `calltargettype`: give DWARF function pointers their prototypes.
    pub calltargettype: bool,
    /// `macho-arm64e`: select the Apple-Silicon spec for an arm64e Mach-O.
    pub macho_arm64e: bool,
    /// `sourcelang`: force the source language; `None` trusts detection.
    pub sourcelang: Option<SourceLang>,
    /// `objcivars`: build Objective-C instance layouts.
    pub objcivars: bool,
    /// `objcsdk`: the SDK or header directory Objective-C signatures come from.
    pub objcsdk: Option<PathBuf>,
    /// `dyldimage`: the dyld shared cache image to load; `None` takes the default.
    pub dyldimage: Option<String>,
    /// `dyldcrossrefs`: name references into other dyld cache images.
    pub dyldcrossrefs: bool,
    /// The fat Mach-O slice to load (`--slice`); `None` takes the default.
    pub macho_slice: Option<String>,
    /// An external `.pdb` file to apply.
    pub pdb_path: Option<PathBuf>,
    /// An external `.fid` function-ID database to apply.
    pub fid_db: Option<PathBuf>,
    /// FLIRT `.sig` files, or directories whose `*.sig` files are all taken.
    pub flirt_sigs: Vec<PathBuf>,
    /// The Go toolchain release, which picks the `runtime.g` layout. The
    /// bootstrap takes it from the image's build info, else from here.
    pub go_version: Option<(u16, u16)>,
    /// Run the Objective-C metadata pass at load, so `objc` has facts to commit.
    pub load_objc: bool,
    /// Run the string-literal pass at load, so `strings` has facts to commit.
    pub load_strings: bool,
    /// Map a thin Mach-O read-only rather than copy it into memory.
    pub load_mmap: bool,
}

impl Default for LoadOptions {
    fn default() -> Self {
        LoadOptions {
            ppc32picimports: Ppc32PicMode::Off,
            ifuncfpret: true,
            ppcapu: false,
            kernelcache: KernelcacheChoice::default(),
            goabi: true,
            symbolnamechars: NameChars::default(),
            libctypes: LibcTypesLayout::Opaque,
            calltargettype: true,
            macho_arm64e: true,
            sourcelang: None,
            objcivars: true,
            objcsdk: None,
            dyldimage: None,
            dyldcrossrefs: true,
            macho_slice: None,
            pdb_path: None,
            fid_db: None,
            flirt_sigs: Vec::new(),
            go_version: None,
            load_objc: true,
            load_strings: true,
            load_mmap: true,
        }
    }
}

impl LoadOptions {
    /// The options the environment states, in the vocabulary the CLI exports.
    /// An unset or unrecognized variable leaves that option at its default.
    pub fn from_env() -> Self {
        LoadOptions {
            ppc32picimports: env::ppc32picimports(),
            ifuncfpret: env::ifuncfpret(),
            ppcapu: env::ppcapu(),
            kernelcache: env::kernelcache(),
            goabi: env::goabi(),
            symbolnamechars: env::symbolnamechars(),
            libctypes: env::libctypes(),
            calltargettype: env::calltargettype(),
            macho_arm64e: env::macho_arm64e(),
            sourcelang: env::sourcelang(),
            objcivars: env::objcivars(),
            objcsdk: env::objcsdk(),
            dyldimage: env::dyldimage(),
            dyldcrossrefs: env::dyldcrossrefs(),
            macho_slice: env::macho_slice(),
            pdb_path: env::pdb_path(),
            fid_db: env::fid_db(),
            flirt_sigs: env::flirt_sigs(),
            go_version: env::go_version(),
            load_objc: env::load_objc(),
            load_strings: env::load_strings(),
            load_mmap: env::load_mmap(),
        }
    }

    /// The options in force on this thread: the innermost scope's, else the
    /// environment's.
    pub fn in_force() -> Rc<LoadOptions> {
        CURRENT
            .with(|current| current.borrow().clone())
            .unwrap_or_else(|| {
                FALLBACKS.with(|count| count.set(count.get() + 1));
                Rc::new(LoadOptions::from_env())
            })
    }

    /// Make these the options in force on this thread until the scope drops.
    pub fn enter(&self) -> LoadScope {
        enter(Rc::new(self.clone()))
    }

    /// Write the named option's value to the environment variable a later
    /// `load file` in this process reads. `Architecture::set_kuna_option`
    /// already does this for the option it sets; this serves a caller that
    /// changed the options through `Architecture::load_options_mut`. A load
    /// with explicit options never needs it. A name that is not a load-time
    /// option is ignored.
    pub fn export_env(&self, option: &str) {
        use crate::*;
        match option {
            "ppc32picimports" => self.ppc32picimports.set_env(),
            "ifuncfpret" => kuna_ifuncfpret::set_ifuncfpret_env(self.ifuncfpret),
            "ppcapu" => kuna_ppcapu::set_ppcapu_env(self.ppcapu),
            "kernelcache" => kuna_kernelcache::set_kernelcache_env(&self.kernelcache),
            "goabi" => kuna_goabi::set_goabi_env(self.goabi),
            "symbolnamechars" => {
                kuna_symbolnamechars::set_symbolnamechars_env(self.symbolnamechars)
            }
            "libctypes" => kuna_libctypes::set_libctypes_env_layout(self.libctypes),
            "calltargettype" => std::env::set_var(
                kuna_calltargettype::CALLTARGETTYPE_ENV,
                if self.calltargettype { "on" } else { "off" },
            ),
            "macho-arm64e" => kuna_macho_arm64e::set_macho_arm64e_env(self.macho_arm64e),
            "sourcelang" => kuna_sourcelang::set_sourcelang_env(self.sourcelang),
            "objcivars" => kuna_objcivars::set_objcivars_env(self.objcivars),
            "objcsdk" => kuna_objcsdk::set_objcsdk_env(
                self.objcsdk.as_deref().and_then(|p| p.to_str()),
            ),
            "dyldimage" => {
                kuna_dyldcache::set_dyldimage_env(self.dyldimage.as_deref().unwrap_or("none"))
            }
            "dyldcrossrefs" => kuna_dyldcache::set_dyldcrossrefs_env(self.dyldcrossrefs),
            _ => {}
        }
    }
}

thread_local! {
    static CURRENT: RefCell<Option<Rc<LoadOptions>>> = const { RefCell::new(None) };
    static FALLBACKS: Cell<u64> = const { Cell::new(0) };
}

/// Make `options` the options in force on this thread until the scope drops.
pub fn enter(options: Rc<LoadOptions>) -> LoadScope {
    let previous = CURRENT.with(|current| current.replace(Some(options)));
    LoadScope { previous }
}

/// Hand this thread back to the environment until the scope drops: every gate
/// reads its environment variable when it runs. A program built under this
/// scope keeps reading the environment through
/// [`crate::infra::architecture::Architecture::load_scope`].
pub fn enter_environment() -> LoadScope {
    let previous = CURRENT.with(|current| current.replace(None));
    LoadScope { previous }
}

/// Whether some options are in force on this thread. Unlike [`scoped`], asking
/// does not count as an environment read.
pub(crate) fn options_in_force() -> bool {
    CURRENT.with(|current| current.borrow().is_some())
}

/// `read` applied to the options in force on this thread, or `None` outside
/// every scope (the caller then reads its environment variable).
pub fn scoped<T>(read: impl FnOnce(&LoadOptions) -> T) -> Option<T> {
    let read = CURRENT.with(|current| current.borrow().as_deref().map(read));
    if read.is_none() {
        FALLBACKS.with(|count| count.set(count.get() + 1));
    }
    read
}

/// How many reads of the options on this thread found no scope in force and
/// fell back to the environment. A caller that loads with explicit options can
/// check that its calls leave this unchanged.
pub fn environment_fallbacks() -> u64 {
    FALLBACKS.with(Cell::get)
}

/// Restores the enclosing options when dropped. Scopes nest, and each belongs
/// to the thread that entered it.
#[must_use = "the options leave force when the scope drops"]
pub struct LoadScope {
    previous: Option<Rc<LoadOptions>>,
}

impl Drop for LoadScope {
    fn drop(&mut self) {
        let previous = self.previous.take();
        CURRENT.with(|current| *current.borrow_mut() = previous);
    }
}

/// Each option read from its environment variable, in the vocabulary the CLI
/// exports: the fallback a gate takes outside every scope.
pub mod env {
    use super::*;

    fn path(name: &str) -> Option<PathBuf> {
        std::env::var_os(name)
            .filter(|p| !p.is_empty())
            .map(PathBuf::from)
    }

    pub fn ppc32picimports() -> Ppc32PicMode {
        std::env::var(crate::kuna_ppc32picimports::ENV).map_or(Ppc32PicMode::Off, |value| {
            Ppc32PicMode::from_env_value(&value)
        })
    }

    pub fn ifuncfpret() -> bool {
        env_toggle(crate::kuna_ifuncfpret::IFUNCFPRET_ENV, true)
    }

    pub fn ppcapu() -> bool {
        env_toggle(crate::kuna_ppcapu::PPCAPU_ENV, false)
    }

    pub fn kernelcache() -> KernelcacheChoice {
        KernelcacheChoice::from_env()
    }

    pub fn goabi() -> bool {
        env_toggle(crate::kuna_goabi::GOABI_ENV, true)
    }

    pub fn symbolnamechars() -> NameChars {
        match std::env::var(crate::kuna_symbolnamechars::SYMBOLNAMECHARS_ENV) {
            Ok(v) => NameChars::parse(&v).unwrap_or_default(),
            Err(_) => NameChars::default(),
        }
    }

    pub fn libctypes() -> LibcTypesLayout {
        match std::env::var(crate::kuna_libctypes::LIBCTYPES_ENV) {
            Ok(v) => match v.trim().to_ascii_lowercase().as_str() {
                "off" | "0" | "false" => LibcTypesLayout::Off,
                "glibc" => LibcTypesLayout::Glibc,
                _ => LibcTypesLayout::Opaque,
            },
            Err(_) => LibcTypesLayout::Opaque,
        }
    }

    pub fn calltargettype() -> bool {
        env_toggle(crate::kuna_calltargettype::CALLTARGETTYPE_ENV, true)
    }

    pub fn macho_arm64e() -> bool {
        env_toggle(crate::kuna_macho_arm64e::MACHO_ARM64E_ENV, true)
    }

    pub fn sourcelang() -> Option<SourceLang> {
        SourceLang::parse_forced(&std::env::var(crate::kuna_sourcelang::SOURCELANG_ENV).ok()?)
    }

    pub fn objcivars() -> bool {
        match std::env::var(crate::kuna_objcivars::OBJCIVARS_ENV) {
            Ok(v) => {
                !matches!(v.trim().to_ascii_lowercase().as_str(), "off" | "0" | "false" | "no")
            }
            Err(_) => true,
        }
    }

    pub fn objcsdk() -> Option<PathBuf> {
        crate::kuna_objcsdk::sdk_path_from_env()
    }

    pub fn dyldimage() -> Option<String> {
        std::env::var(crate::kuna_dyldcache::DYLDIMAGE_ENV)
            .ok()
            .and_then(|v| crate::kuna_dyldcache::image_name(&v).map(str::to_string))
    }

    pub fn dyldcrossrefs() -> bool {
        env_toggle(crate::kuna_dyldcache::DYLDCROSSREFS_ENV, true)
    }

    pub fn macho_slice() -> Option<String> {
        std::env::var(MACHO_SLICE_ENV)
            .ok()
            .filter(|t| !t.trim().is_empty())
    }

    pub fn pdb_path() -> Option<PathBuf> {
        path(PDB_PATH_ENV)
    }

    pub fn fid_db() -> Option<PathBuf> {
        path(FID_DB_ENV)
    }

    pub fn flirt_sigs() -> Vec<PathBuf> {
        std::env::var_os(FLIRT_SIGS_ENV)
            .map(|list| std::env::split_paths(&list).collect())
            .unwrap_or_default()
    }

    fn load_pass(name: &str) -> bool {
        !std::env::var(name).is_ok_and(|value| {
            matches!(value.trim().to_ascii_lowercase().as_str(), "off" | "0" | "false")
        })
    }

    pub fn load_objc() -> bool {
        load_pass(LOAD_OBJC_ENV)
    }

    pub fn load_strings() -> bool {
        load_pass(LOAD_STRINGS_ENV)
    }

    pub fn load_mmap() -> bool {
        !std::env::var(LOAD_MMAP_ENV).is_ok_and(|value| matches!(value.as_str(), "off" | "0" | "false"))
    }

    /// The `"<maj>.<min>"` release in `KUNA_GOVERSION`; `None` when it is
    /// unset or unparsable.
    pub fn go_version() -> Option<(u16, u16)> {
        let value = std::env::var(crate::kuna_goreg::GOVERSION_ENV).ok()?;
        let mut parts = value.trim().split('.');
        Some((parts.next()?.parse().ok()?, parts.next()?.parse().ok()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scopes_nest_and_restore_per_thread() {
        assert_eq!(scoped(|o| o.goabi), None);
        let outer = LoadOptions {
            goabi: false,
            ..LoadOptions::default()
        };
        let _outer = outer.enter();
        assert_eq!(scoped(|o| o.goabi), Some(false));
        {
            let _inner = LoadOptions::default().enter();
            assert_eq!(scoped(|o| o.goabi), Some(true));
            std::thread::spawn(|| assert_eq!(scoped(|o| o.goabi), None))
                .join()
                .unwrap();
        }
        assert_eq!(scoped(|o| o.goabi), Some(false));
        assert!(!LoadOptions::in_force().goabi);
    }

    #[test]
    fn a_scope_restores_the_enclosing_options_after_a_panic() {
        let _outer = LoadOptions {
            dyldcrossrefs: false,
            ..LoadOptions::default()
        }
        .enter();
        let unwound = std::panic::catch_unwind(|| {
            let _inner = LoadOptions::default().enter();
            panic!("load failed");
        });
        assert!(unwound.is_err());
        assert_eq!(scoped(|o| o.dyldcrossrefs), Some(false));
    }

    #[test]
    fn gates_read_the_scope_not_the_environment() {
        let options = LoadOptions {
            ifuncfpret: false,
            ppcapu: true,
            kernelcache: KernelcacheChoice::Off,
            symbolnamechars: NameChars::Ident,
            libctypes: LibcTypesLayout::Glibc,
            calltargettype: false,
            macho_arm64e: false,
            sourcelang: Some(SourceLang::Rust),
            ppc32picimports: Ppc32PicMode::Abi,
            objcivars: false,
            objcsdk: Some(PathBuf::from("/sdk")),
            dyldimage: Some("libfoo.dylib".into()),
            dyldcrossrefs: false,
            ..LoadOptions::default()
        };
        let _scope = options.enter();
        assert!(!crate::kuna_ifuncfpret::ifuncfpret_enabled());
        assert!(crate::kuna_ppcapu::ppcapu_enabled());
        assert_eq!(KernelcacheChoice::in_force(), KernelcacheChoice::Off);
        assert_eq!(
            crate::kuna_symbolnamechars::symbolnamechars_mode(),
            NameChars::Ident
        );
        assert_eq!(
            crate::kuna_libctypes::libctypes_layout(),
            LibcTypesLayout::Glibc
        );
        assert!(!crate::kuna_calltargettype::dwarf_function_types());
        assert!(!crate::kuna_macho_arm64e::macho_arm64e_enabled());
        assert_eq!(crate::kuna_sourcelang::forced(), Some(SourceLang::Rust));
        assert!(crate::kuna_ppc32picimports::abi_enabled());
        assert!(!crate::kuna_objcivars::objcivars_enabled());
        assert_eq!(crate::kuna_objcsdk::sdk_path(), Some(PathBuf::from("/sdk")));
        assert_eq!(crate::kuna_dyldcache::selected_image().as_deref(), Some("libfoo.dylib"));
        assert!(!crate::kuna_dyldcache::dyldcrossrefs_enabled());
    }
}

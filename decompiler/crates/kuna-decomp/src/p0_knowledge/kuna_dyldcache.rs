//! (kuna) The dyld shared cache options: which image of a cache to load
//! (`dyldimage`) and whether its references into other images are named
//! (`dyldcrossrefs`).
//!
//! Both act inside `load file`, while the loader rebuilds the image from the
//! cache, upstream of every per-function `option` command, so both travel as
//! process environment variables set by the `kuna` CLI before loading (the
//! `dynrelocs` loadtime pattern). The CLI's `--image NAME` sets
//! [`DYLDIMAGE_ENV`] for every subcommand; the console's `option dyldimage NAME`
//! before `load file` sets the same variable, and is accepted before any image
//! is loaded, since choosing the first image is its purpose.

use kuna_base::error::KunaResult;

/// The install name, basename or stem of the cache image to load.
pub const DYLDIMAGE_ENV: &str = "KUNA_DYLD_IMAGE";

/// Gates naming calls and GOT slots that reach into other cache images.
/// Absent or any value other than the off-tokens means enabled.
pub const DYLDCROSSREFS_ENV: &str = "KUNA_DYLDCROSSREFS";

pub fn dyldcrossrefs_enabled() -> bool {
    crate::options::env_toggle(DYLDCROSSREFS_ENV, true)
}

pub fn set_dyldcrossrefs_env(on: bool) {
    std::env::set_var(DYLDCROSSREFS_ENV, if on { "on" } else { "off" });
}

pub fn set_dyldimage_env(name: &str) {
    std::env::set_var(DYLDIMAGE_ENV, name);
}

/// The image a `dyldimage` value names; `none` and the empty value name none.
pub fn image_name(value: &str) -> Option<&str> {
    Some(value.trim()).filter(|v| !v.is_empty() && *v != "none")
}

/// The image the run selected through [`DYLDIMAGE_ENV`].
pub fn selected_image() -> Option<String> {
    std::env::var(DYLDIMAGE_ENV).ok().and_then(|v| image_name(&v).map(str::to_string))
}

/// Apply `option dyldimage|dyldcrossrefs VALUE`, or `None` for another option.
/// Neither needs a loaded image: the next `load file` reads both.
pub fn set_option(name: &str, value: &str) -> Option<KunaResult<String>> {
    Some(match name {
        "dyldimage" => Ok(match image_name(value) {
            Some(image) => {
                set_dyldimage_env(image);
                format!("dyld shared cache image set to {image}")
            }
            None => {
                std::env::remove_var(DYLDIMAGE_ENV);
                "dyld shared cache image selection cleared".to_string()
            }
        }),
        "dyldcrossrefs" => crate::options::on_or_off(value).map(|on| {
            set_dyldcrossrefs_env(on);
            format!("dyld shared cache cross-image reference naming turned {}", if on { "on" } else { "off" })
        }),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crossrefs_default_on_and_off_tokens() {
        std::env::remove_var(DYLDCROSSREFS_ENV);
        assert!(dyldcrossrefs_enabled());
        for off in ["off", "0", "false"] {
            std::env::set_var(DYLDCROSSREFS_ENV, off);
            assert!(!dyldcrossrefs_enabled(), "`{off}` disables");
        }
        set_dyldcrossrefs_env(true);
        assert!(dyldcrossrefs_enabled());
        std::env::remove_var(DYLDCROSSREFS_ENV);
    }

    #[test]
    fn none_and_empty_name_no_image() {
        assert_eq!(image_name(" CoreFoundation "), Some("CoreFoundation"));
        assert_eq!(image_name("none"), None);
        assert_eq!(image_name("  "), None);
        assert!(set_option("dyldcrossrefs", "maybe").unwrap().is_err());
        assert!(set_option("dynrelocs", "on").is_none());
    }
}

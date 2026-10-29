//! (kuna) The dyld shared cache options: which image of a cache to load
//! (`dyldimage`) and whether its references into other images are named
//! (`dyldcrossrefs`).
//!
//! Both act inside `load file`, while the loader rebuilds the image from the
//! cache, upstream of every per-function `option` command, so both travel as
//! process environment variables set by the `kuna` CLI before loading (the
//! `dynrelocs` loadtime pattern). The CLI's `--image NAME` sets
//! [`DYLDIMAGE_ENV`] for every subcommand; the console's `option dyldimage NAME`
//! before `load file` sets the same variable. The `Architecture` fields exist
//! only for catalog visibility and the live `current` value.

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
}

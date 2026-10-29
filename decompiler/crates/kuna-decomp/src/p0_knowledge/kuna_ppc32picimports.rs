//! Loader-time gate for PPC32 secure-PLT identities proved from callers.
//! The CLI bridges the option before opening the image; an interactive console
//! must set it before `load file`, as with `i386_pie_plt`.
pub const ENV: &str = "KUNA_PPC32PICIMPORTS";

pub fn enabled() -> bool {
    crate::options::env_toggle(ENV, false)
}

pub fn set_env(on: bool) {
    std::env::set_var(ENV, if on { "on" } else { "off" });
}

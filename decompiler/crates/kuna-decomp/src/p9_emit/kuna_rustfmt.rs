//! (kuna `rustfmt`) Which Rust formatting-macro recoveries run. `on` recovers
//! print!/eprint!, format! and panic! (literal and piece-array layouts, primitive
//! and string arguments, static integer specifications); `all` adds the String
//! and Stdout write!/writeln! writers; `off` keeps every runtime formatting call.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RustFmtMode {
    Off,
    #[default]
    On,
    All,
}

impl RustFmtMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::On => "on",
            Self::All => "all",
        }
    }

    /// print!, format! and panic! recovery with every argument form.
    pub fn recovers(self) -> bool {
        self != Self::Off
    }

    /// The String and Stdout write! recoveries.
    pub fn writers(self) -> bool {
        self == Self::All
    }
}

pub fn parse_rustfmt_mode(value: &str) -> Result<(RustFmtMode, String), String> {
    let mode = match value.trim().to_ascii_lowercase().as_str() {
        "off" => RustFmtMode::Off,
        "on" | "" => RustFmtMode::On,
        "all" => RustFmtMode::All,
        other => {
            return Err(format!(
                "unknown rustfmt value {other:?} (expected off, on or all)"
            ))
        }
    };
    Ok((mode, format!("Rust formatting-macro recovery set to {}", mode.as_str())))
}

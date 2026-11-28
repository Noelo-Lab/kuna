//! Native TASKING Carmel containers normalized to the engine's byte offsets.

use kuna_base::error::{KunaError, KunaResult};
use kuna_decomp::kuna_loadoptions;

mod ieee;
mod loader;
pub(super) use loader::NativeImage;
mod text;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum Space {
    Code,
    Data,
    Cliw,
    Io,
    Ext,
}

impl Space {
    pub(super) const fn name(self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Data => "data",
            Self::Cliw => "cliw",
            Self::Io => "io",
            Self::Ext => "ext",
        }
    }
}

#[derive(Debug)]
pub(super) struct Region {
    pub(super) space: Space,
    pub(super) offset: u64,
    pub(super) bytes: Vec<u8>,
    pub(super) readonly: bool,
    pub(super) executable: bool,
}

#[derive(Debug)]
pub(super) struct Symbol {
    pub(super) space: Space,
    pub(super) offset: u64,
    pub(super) name: Vec<u8>,
}

#[derive(Debug, Default)]
pub(super) struct Image {
    pub(super) regions: Vec<Region>,
    pub(super) symbols: Vec<Symbol>,
    pub(super) imports: Vec<Symbol>,
    pub(super) entry: Option<u64>,
    pub(super) normalized_cliw: bool,
    pub(super) relocatable: bool,
}

pub(super) fn recognizes(bytes: &[u8], target: &str) -> bool {
    ieee::recognizes(bytes) || text::recognizes(bytes)
        && (target.starts_with("Carmel:") || text_profile_selected())
}

fn text_profile_selected() -> bool {
    kuna_loadoptions::scoped(|options| options.tasking_carmel_code)
        .unwrap_or_else(kuna_loadoptions::env::tasking_carmel_code)
}

pub(super) fn parse(bytes: &[u8]) -> KunaResult<Image> {
    if ieee::recognizes(bytes) {
        return ieee::parse(bytes);
    }
    if !text_profile_selected() {
        return Err(KunaError::lowlevel(
            "text records do not identify the Carmel memory bank; set KUNA_TASKING_CARMEL_CODE=on \
             only for a TASKING export containing instructions in CODE and no auxiliary data",
        ));
    }
    text::parse(bytes)
}

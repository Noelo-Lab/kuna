//! Shared immutable input storage for object views and load-image segments.

use std::fs::File;
use std::ops::Deref;
use std::sync::Arc;

use kuna_base::error::{KunaError, KunaResult};
use memmap2::{Mmap, MmapOptions};

#[derive(Clone, Debug)]
pub enum ImageBytes {
    Owned(Arc<[u8]>),
    Mapped(Arc<Mmap>),
}

impl ImageBytes {
    /// Mapping requires the input file to remain unchanged while it is analyzed.
    /// A kernel collection (`MH_FILESET`) declines: its entry is selected into an
    /// owned copy.
    pub fn map_thin_macho(path: &str) -> KunaResult<Option<Self>> {
        if std::env::var("KUNA_LOAD_MMAP")
            .is_ok_and(|value| matches!(value.as_str(), "off" | "0" | "false"))
        {
            return Ok(None);
        }
        let file = File::open(path).map_err(|error| {
            KunaError::lowlevel(format!("Unable to open image file: {path}: {error}"))
        })?;
        if !file.metadata().is_ok_and(|metadata| metadata.is_file()) {
            return Ok(None);
        }
        // The analysis treats its input as immutable; writes use owned segment overlays.
        let mapping = unsafe { MmapOptions::new().map_copy_read_only(&file) };
        let Ok(mapping) = mapping else {
            return Ok(None);
        };
        let filetype = |word: [u8; 4], little: bool| {
            if little { u32::from_le_bytes(word) } else { u32::from_be_bytes(word) }
        };
        let header = mapping.get(..16).and_then(|header| {
            let word: [u8; 4] = header[12..16].try_into().ok()?;
            match header[..4] {
                [0xcf, 0xfa, 0xed, 0xfe] | [0xce, 0xfa, 0xed, 0xfe] => Some(filetype(word, true)),
                [0xfe, 0xed, 0xfa, 0xcf] | [0xfe, 0xed, 0xfa, 0xce] => Some(filetype(word, false)),
                _ => None,
            }
        });
        if !header.is_some_and(|filetype| filetype != object::macho::MH_FILESET) {
            return Ok(None);
        }
        Ok(Some(Self::Mapped(Arc::new(mapping))))
    }
}

impl Deref for ImageBytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        match self {
            Self::Owned(bytes) => bytes,
            Self::Mapped(bytes) => bytes,
        }
    }
}

impl From<Arc<[u8]>> for ImageBytes {
    fn from(bytes: Arc<[u8]>) -> Self {
        Self::Owned(bytes)
    }
}

impl From<Vec<u8>> for ImageBytes {
    fn from(bytes: Vec<u8>) -> Self {
        Self::Owned(bytes.into())
    }
}

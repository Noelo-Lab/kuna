//! Function selectors and object-coordinate provenance shared by front-ends.

use std::fmt;

use kuna_base::address::Address;

/// A stable coordinate in a relocatable input object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectLocation {
    pub section_index: usize,
    pub section: String,
    pub offset: u64,
}

/// Why an entry address exists in the loaded program.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryProvenance {
    /// Code or a symbol from a normal linked/XML image.
    Mapped,
    /// A definition backed by a section in a relocatable object.
    DefinedObject,
    /// An undefined/import symbol assigned a synthetic address for call naming.
    UndefinedExternal,
}

/// A user-facing function selector shared by the console, native CLI and WASM
/// front-ends.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntrySelector {
    Name(String),
    Numeric(u64),
    SectionOffset { section: String, offset: u64 },
    SectionIndexOffset { section_index: usize, offset: u64 },
}

impl EntrySelector {
    /// Parse `.section+0xOFFSET`, `SECTION_INDEX:0xOFFSET`, a `0xVMA`, or a
    /// function name. Bare numeric tokens are intentionally left as names here;
    /// callers implementing an explicit `--addr` surface construct
    /// [`EntrySelector::Numeric`] after applying that surface's number grammar.
    pub fn parse(token: &str) -> Self {
        if let Some((section, offset)) = token.split_once('+') {
            if section.starts_with('.') {
                if let Some(offset) = parse_number(offset) {
                    return Self::SectionOffset {
                        section: section.to_string(),
                        offset,
                    };
                }
            }
        }
        if let Some((section_index, offset)) = token.split_once(':') {
            if let (Ok(section_index), Some(offset)) =
                (section_index.parse::<usize>(), parse_number(offset))
            {
                return Self::SectionIndexOffset {
                    section_index,
                    offset,
                };
            }
        }
        if token.starts_with("0x") || token.starts_with("0X") {
            if let Some(value) = parse_number(token) {
                return Self::Numeric(value);
            }
        }
        Self::Name(token.to_string())
    }

    pub(crate) fn display(&self) -> String {
        match self {
            Self::Name(name) => name.clone(),
            Self::Numeric(vma) => format!("0x{vma:x}"),
            Self::SectionOffset { section, offset } => format!("{section}+0x{offset:x}"),
            Self::SectionIndexOffset {
                section_index,
                offset,
            } => format!("{section_index}:0x{offset:x}"),
        }
    }
}

fn parse_number(token: &str) -> Option<u64> {
    let token = token.trim();
    let digits = token
        .strip_prefix("0x")
        .or_else(|| token.strip_prefix("0X"))
        .unwrap_or(token);
    (!digits.is_empty())
        .then(|| u64::from_str_radix(digits, 16).ok())
        .flatten()
}

/// A selector did not identify exactly one usable entry.
#[derive(Debug, Clone)]
pub enum EntryLookupError {
    NotFound {
        selector: String,
    },
    Unmapped {
        selector: String,
        relocatable: bool,
    },
    Ambiguous {
        selector: String,
        candidates: Vec<FunctionEntry>,
    },
}

impl fmt::Display for EntryLookupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound { selector } => write!(f, "no function matches {selector:?}"),
            Self::Unmapped {
                selector,
                relocatable,
            } => {
                write!(f, "address {selector} is not mapped in this input")?;
                if *relocatable {
                    write!(
                        f,
                        "; relocatable objects use synthetic load VMAs—select a definition with `.section+0xOFFSET` or `SECTION_INDEX:0xOFFSET`"
                    )?;
                }
                Ok(())
            }
            Self::Ambiguous {
                selector,
                candidates,
            } => {
                writeln!(f, "selector {selector:?} is ambiguous; candidates:")?;
                for candidate in candidates {
                    match &candidate.object_location {
                        Some(location) => writeln!(
                            f,
                            "  {} at {}+0x{:x} (section {}, synthetic 0x{:x}, {})",
                            candidate.name,
                            location.section,
                            location.offset,
                            location.section_index,
                            candidate.addr.get_offset(),
                            candidate.binding.as_deref().unwrap_or("unknown binding")
                        )?,
                        None => writeln!(
                            f,
                            "  {} at synthetic 0x{:x}",
                            candidate.name,
                            candidate.addr.get_offset()
                        )?,
                    }
                }
                write!(
                    f,
                    "use a section-qualified selector to choose one candidate"
                )
            }
        }
    }
}

impl std::error::Error for EntryLookupError {}

#[derive(Debug, Clone)]
pub(crate) struct ObjectSectionLocation {
    pub(crate) index: usize,
    pub(crate) name: String,
    pub(crate) vma: u64,
    pub(crate) size: u64,
}

/// One canonical function entry for whole-program and explicit selection.
#[derive(Debug, Clone)]
pub struct FunctionEntry {
    /// The most informative name at this entry.
    pub name: String,
    /// The entry address, Thumb-bit normalized on an ARM-family spec.
    pub addr: Address,
    /// Every other name this same entry carries, in preference order.
    pub aliases: Vec<String>,
    /// The entry's byte extent: an upper-bound clip to the next entry or the
    /// end of its containing code section. Zero when it cannot be measured.
    pub size: u64,
    /// Original object-file coordinate for an `ET_REL`/relocatable definition.
    pub object_location: Option<ObjectLocation>,
    /// Defined/mapped versus genuinely undefined/import provenance.
    pub provenance: EntryProvenance,
    /// Object symbol binding (`local`, `global`, `weak`, or `unknown`).
    pub binding: Option<String>,
}

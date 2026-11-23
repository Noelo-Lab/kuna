//! PowerPC APU requirements do not generally identify an exclusive processor.
//! Shared ISEL keeps the generic register and calling-convention model; SPE
//! selects e500 only under `ppcapu on`.
use kuna_decomp::kuna_ppcapu::Mode;
use object::{Architecture, FileFlags, Object, ObjectSection};

fn word(bytes: &[u8], little: bool) -> Option<u32> {
    let bytes = bytes.get(..4)?.try_into().ok()?;
    Some(if little {
        u32::from_le_bytes(bytes)
    } else {
        u32::from_be_bytes(bytes)
    })
}

#[derive(Default)]
struct Requirements {
    spe: bool,
    isel: bool,
}

fn read_requirements(bytes: &[u8], little: bool, apus: &mut Requirements) -> Option<()> {
    if bytes.len() > 4096 {
        return None;
    }
    let mut cursor = 0usize;
    while cursor < bytes.len() {
        let tail = &bytes[cursor..];
        if tail.len() < 12 && tail.iter().all(|&b| b == 0) {
            break;
        }
        let namesz = word(tail, little)? as usize;
        let descsz = word(tail.get(4..)?, little)? as usize;
        let kind = word(tail.get(8..)?, little)?;
        let descstart = 12usize.checked_add(namesz.checked_add(3)? & !3)?;
        let descend = descstart.checked_add(descsz)?;
        let name = tail.get(12..12usize.checked_add(namesz)?)?;
        if !name.is_empty() {
            std::ffi::CStr::from_bytes_with_nul(name).ok()?;
        }
        let desc = tail.get(descstart..descend)?;
        cursor = cursor.checked_add(descend.checked_add(3)? & !3)?;
        if kind != 2 || name != b"APUinfo\0" {
            continue;
        }
        if desc.len() % 4 != 0 {
            return None;
        }
        for entry in desc.chunks_exact(4) {
            let apu = word(entry, little)?;
            if apu & 0xffff != 1 {
                return None;
            }
            match apu >> 16 {
                0x40 => apus.isel = true,
                0x41..=0x43 => {}
                0x100 | 0x101 => apus.spe = true,
                _ => return None,
            }
        }
    }
    Some(())
}

/// `Err` when a note is malformed or names an APU this reader cannot classify.
fn powerpc_variant(file: &object::File<'_>) -> Result<Option<&'static str>, ()> {
    let mut apus = Requirements::default();
    for section in file
        .sections()
        .filter(|s| s.name() == Ok(".PPC.EMB.apuinfo"))
    {
        let data = section.data().map_err(|_| ())?;
        read_requirements(data, file.is_little_endian(), &mut apus).ok_or(())?;
    }
    let embedded = matches!(file.flags(), FileFlags::Elf { e_flags, .. } if e_flags & object::elf::EF_PPC_EMB != 0);
    Ok(if apus.spe {
        Some("e500")
    } else if apus.isel || embedded {
        Some("isel")
    } else {
        None
    })
}

fn applies(file: &object::File<'_>) -> bool {
    file.format() == object::BinaryFormat::Elf && file.architecture() == Architecture::PowerPc
}

pub(crate) fn language_ids() -> impl Iterator<Item = String> {
    ["BE", "LE"].into_iter().flat_map(|endian| {
        ["e500", "isel"].map(|variant| format!("PowerPC:{endian}:32:{variant}:default"))
    })
}

pub(crate) fn language_id(file: &object::File<'_>) -> Option<String> {
    language_id_when(file, kuna_decomp::kuna_ppcapu::ppcapu_mode())
}

/// An unclassifiable note keeps the generic decoder; [`notes`] reports it.
pub(super) fn language_id_when(file: &object::File<'_>, mode: Mode) -> Option<String> {
    if mode == Mode::Off || !applies(file) {
        return None;
    }
    let endian = if file.is_little_endian() { "LE" } else { "BE" };
    let variant = match powerpc_variant(file).ok().flatten()? {
        "e500" if mode != Mode::On => "isel",
        variant => variant,
    };
    Some(format!("PowerPC:{endian}:32:{variant}:default"))
}

pub(crate) fn notes(file: &object::File<'_>, language: &str, explicit: bool) -> Vec<String> {
    notes_when(
        file,
        language,
        explicit,
        kuna_decomp::kuna_ppcapu::ppcapu_mode() != Mode::Off,
    )
}

pub(super) fn notes_when(
    file: &object::File<'_>,
    language: &str,
    explicit: bool,
    enabled: bool,
) -> Vec<String> {
    if explicit || !enabled || !applies(file) || powerpc_variant(file).is_ok() {
        return Vec::new();
    }
    vec![format!("PowerPC APU note is malformed or names an APU kuna does not classify; using {language}; use --target to specify another language")]
}

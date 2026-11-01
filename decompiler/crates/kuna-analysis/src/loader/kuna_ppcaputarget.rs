//! PowerPC APU requirements do not generally identify an exclusive processor.
use kuna_base::error::{KunaError, KunaResult};
use object::{Architecture, Object, ObjectSection};

fn unsupported(message: &str) -> KunaError {
    KunaError::lowlevel(format!("{message}; pass an explicit --target language id"))
}

fn word(bytes: &[u8], little: bool) -> Option<u32> {
    let bytes = bytes.get(..4)?.try_into().ok()?;
    Some(if little {
        u32::from_le_bytes(bytes)
    } else {
        u32::from_be_bytes(bytes)
    })
}

fn apu_requirements(bytes: &[u8], little: bool) -> Option<bool> {
    if bytes.len() > 4096 {
        return None;
    }
    let mut cursor = 0usize;
    let mut spe = false;
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
                0x40..=0x43 => {}
                0x100 | 0x101 => spe = true,
                _ => return None,
            }
        }
    }
    Some(spe)
}

fn powerpc_variant(file: &object::File<'_>) -> KunaResult<Option<&'static str>> {
    let mut spe = false;
    for section in file
        .sections()
        .filter(|s| s.name() == Ok(".PPC.EMB.apuinfo"))
    {
        let requirements = section
            .data()
            .ok()
            .and_then(|data| apu_requirements(data, file.is_little_endian()))
            .ok_or_else(|| unsupported("unsupported or malformed PowerPC APU requirements"))?;
        spe |= requirements;
    }
    Ok(if spe { Some("e500") } else { None })
}

pub(crate) fn language_ids() -> impl Iterator<Item = String> {
    ["BE", "LE"]
        .into_iter()
        .map(|endian| format!("PowerPC:{endian}:32:e500:default"))
}

pub(crate) fn language_id(file: &object::File<'_>) -> KunaResult<Option<String>> {
    if file.format() != object::BinaryFormat::Elf || file.architecture() != Architecture::PowerPc {
        return Ok(None);
    }
    let endian = if file.is_little_endian() { "LE" } else { "BE" };
    Ok(powerpc_variant(file)?.map(|variant| format!("PowerPC:{endian}:32:{variant}:default")))
}

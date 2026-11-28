//! TASKING locator `-f2`/`-f3` output for instruction-only CODE images.
//!
//! The caller must know that all exported words are instructions in CODE:
//! neither format preserves bank or per-region execution metadata.

use kuna_analysis::loader::{ihex, srecord};
use kuna_base::error::{KunaError, KunaResult};

use super::{Image, Region, Space};

const WORD: u64 = 3;
const FETCH: u64 = 2 * WORD;
const BYTE_LIMIT: u64 = (1 << 24) * WORD;

pub(super) fn recognizes(bytes: &[u8]) -> bool {
    srecord::recognizes(bytes) || ihex::recognizes(bytes)
}

pub(super) fn parse(bytes: &[u8]) -> KunaResult<Image> {
    let mut image = Image::default();
    if srecord::recognizes(bytes) {
        srecord::read(bytes, |line, record| match record {
            srecord::Record::Data { address, bytes, address_size: 3 } =>
                add_words(&mut image, line, address, bytes),
            srecord::Record::Data { .. } =>
                Err(error(line, "expected Motorola S2 data records for 24-bit CODE")),
            srecord::Record::End { address: 0, address_size: 3 } => Ok(()),
            srecord::Record::End { address, address_size: 3 } => {
                image.entry = Some(u64::from(address) * WORD);
                Ok(())
            }
            srecord::Record::End { .. } =>
                Err(error(line, "expected a Motorola S8 end record for 24-bit CODE")),
        })?;
    } else {
        ihex::read(bytes, |line, record| match record {
            ihex::Record::Data { address, bytes } => add_words(&mut image, line, address, bytes),
            ihex::Record::StartLinear(address) if u64::from(address) * WORD < BYTE_LIMIT => {
                image.entry = Some(u64::from(address) * WORD);
                Ok(())
            }
            ihex::Record::StartLinear(_) =>
                Err(error(line, "entry overflows the 24-bit CODE word range")),
            ihex::Record::StartSegment { .. } =>
                Err(error(line, "expected an Intel HEX linear start address")),
            ihex::Record::AddressBase(base) if u64::from(base) * WORD >= BYTE_LIMIT =>
                Err(error(line, "address extension overflows the 24-bit CODE word range")),
            ihex::Record::AddressBase(_) | ihex::Record::End => Ok(()),
        })?;
    }

    if image.regions.is_empty() {
        return Err(KunaError::lowlevel("TASKING text output contains no Carmel CODE data"));
    }
    order_and_coalesce(&mut image.regions)?;
    if let Some(entry) = image.entry {
        let fetch = entry / FETCH * FETCH;
        let i = image.regions.partition_point(|region| region.offset <= fetch);
        if i == 0 || fetch + FETCH > end(&image.regions[i - 1]) {
            return Err(KunaError::lowlevel(format!(
                "Carmel CODE start word 0x{:x} does not have a mapped fetch",
                entry / WORD,
            )));
        }
    } else {
        // Some exports have no start address; lccarm emits S8 address zero
        // even for those. Use the first mapped complete fetch without filling gaps.
        image.entry = image.regions.iter().find_map(|region| {
            let start = region.offset.next_multiple_of(FETCH);
            (start + FETCH <= end(region)).then_some(start)
        });
    }
    Ok(image)
}

fn error(line: usize, message: &str) -> KunaError {
    KunaError::lowlevel(format!("Carmel CODE text line {line}: {message}"))
}

fn add_words(image: &mut Image, line: usize, address: u32, bytes: &[u8]) -> KunaResult<()> {
    let offset = u64::from(address) * WORD;
    if bytes.is_empty() || !(bytes.len() as u64).is_multiple_of(WORD) {
        return Err(error(line, "data record must contain whole three-byte CODE words"));
    }
    if offset + bytes.len() as u64 > BYTE_LIMIT {
        return Err(error(line, "data record overflows the 24-bit CODE word range"));
    }
    if let Some(last) = image.regions.last_mut() {
        if merge(last, offset, bytes)? {
            return Ok(());
        }
    }
    image.regions.push(Region {
        space: Space::Code, offset, bytes: bytes.to_vec(), readonly: true, executable: true,
    });
    Ok(())
}

fn end(region: &Region) -> u64 {
    region.offset + region.bytes.len() as u64
}

/// Merge contiguous records and allow repeated bytes only when they agree.
fn merge(region: &mut Region, offset: u64, bytes: &[u8]) -> KunaResult<bool> {
    if offset < region.offset || offset > end(region) {
        return Ok(false);
    }
    let index = (offset - region.offset) as usize;
    let overlap = bytes.len().min(region.bytes.len() - index);
    if region.bytes[index..index + overlap] != bytes[..overlap] {
        return Err(KunaError::lowlevel(format!(
            "Conflicting overlapping Carmel CODE text records at word 0x{:x}",
            offset / WORD,
        )));
    }
    region.bytes.extend_from_slice(&bytes[overlap..]);
    Ok(true)
}

fn order_and_coalesce(regions: &mut Vec<Region>) -> KunaResult<()> {
    regions.sort_unstable_by_key(|region| region.offset);
    let mut last = 0;
    for i in 1..regions.len() {
        let (previous, remaining) = regions.split_at_mut(i);
        let next = &remaining[0];
        if !merge(&mut previous[last], next.offset, &next.bytes)? {
            last += 1;
            regions.swap(last, i);
        }
    }
    regions.truncate(last + 1);
    Ok(())
}

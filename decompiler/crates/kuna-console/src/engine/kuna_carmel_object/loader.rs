use std::{cell::Cell, rc::Rc};

use kuna_base::{
    address::{Address, RangeList},
    error::{KunaError, KunaResult},
    space::{AddrSpace, AddrSpaceManager},
};
use kuna_sleigh::loadimage::{section_flags, LoadImage, LoadImageSection};

use super::{Region, Space};

const SPACES: [Space; 5] = [Space::Code, Space::Data, Space::Cliw, Space::Io, Space::Ext];

pub(in crate::engine) struct NativeImage {
    filename: String,
    regions: Vec<Region>,
    spaces: [Rc<AddrSpace>; SPACES.len()],
    adjustment: i128,
    section: Cell<usize>,
}

fn end(region: &Region) -> u64 {
    region.offset + region.bytes.len() as u64
}

impl NativeImage {
    pub(in crate::engine) fn new(path: &str, mut regions: Vec<Region>, manager: &AddrSpaceManager) -> KunaResult<Self> {
        let [Some(code), Some(data), Some(cliw), Some(io), Some(ext)] =
            SPACES.map(|space| manager.get_space_by_name(space.name()).cloned())
        else {
            return Err(KunaError::lowlevel("incomplete Carmel address spaces"));
        };
        let spaces = [code, data, cliw, io, ext];
        regions.sort_unstable_by_key(|region| (region.space, region.offset));
        for region in &regions {
            let space = &spaces[region.space as usize];
            let width = u64::from(space.get_word_size());
            let valid = (!region.executable || region.space == Space::Code)
                && !region.bytes.is_empty() && region.offset.is_multiple_of(width)
                && (region.space == Space::Code && !region.executable
                    || (region.bytes.len() as u64).is_multiple_of(width))
                && region.offset.checked_add(region.bytes.len() as u64)
                    .is_some_and(|limit| limit - 1 <= space.get_highest());
            if !valid {
                return Err(KunaError::lowlevel(format!("invalid extent in TASKING {} space", region.space.name())));
            }
        }
        if regions.windows(2).any(|pair| pair[0].space == pair[1].space && end(&pair[0]) > pair[1].offset) {
            return Err(KunaError::lowlevel("overlapping TASKING regions"));
        }
        Ok(Self { filename: path.to_owned(), regions, spaces, adjustment: 0, section: Cell::new(0) })
    }

    pub(in crate::engine) fn address(&self, space: Space, offset: u64) -> KunaResult<Address> {
        let handle = &self.spaces[space as usize];
        if offset > handle.get_highest() || !offset.is_multiple_of(u64::from(handle.get_word_size())) {
            return Err(KunaError::lowlevel(format!("invalid TASKING {} symbol byte offset {offset:#x}", space.name())));
        }
        Ok(Address::new(Rc::clone(handle), offset))
    }

    pub(in crate::engine) fn executable_ranges(&self) -> Vec<(u64, u64)> {
        let mut ranges: Vec<(u64, u64)> = Vec::new();
        for region in self.regions.iter().filter(|r| r.executable) {
            let Some((start, end)) = self.range(region) else { continue; };
            if let Some(last) = ranges.last_mut().filter(|last| last.1 == start) {
                last.1 = end;
            } else {
                ranges.push((start, end));
            }
        }
        ranges
    }

    pub(in crate::engine) fn contains(&self, space: Space, offset: u64) -> bool {
        let after = self.regions.partition_point(|r| (r.space, r.offset) <= (space, offset));
        after.checked_sub(1).is_some_and(|i| self.regions[i].space == space && offset < end(&self.regions[i]))
    }

    fn delta(&self, space: Space) -> i128 {
        self.adjustment * i128::from(self.spaces[space as usize].get_word_size())
    }

    fn range(&self, region: &Region) -> Option<(u64, u64)> {
        let start = u64::try_from(i128::from(region.offset) + self.delta(region.space)).ok()?;
        let end = start.checked_add(region.bytes.len() as u64)?;
        (end - 1 <= self.spaces[region.space as usize].get_highest()).then_some((start, end))
    }
}

impl LoadImage for NativeImage {
    fn get_file_name(&self) -> &str { &self.filename }

    fn get_arch_type(&self) -> Vec<u8> { super::super::kuna_carmel_image::TARGET.as_bytes().to_vec() }

    fn adjust_vma(&mut self, adjust: i64) { self.adjustment += i128::from(adjust); }

    fn load_fill(&mut self, dst: &mut [u8], address: &Address) -> KunaResult<()> {
        if dst.is_empty() { return Ok(()); }
        let unavailable = || KunaError::data_unavail("outside mapped TASKING image");
        let handle = address.get_space().ok_or_else(unavailable)?;
        let space = SPACES.iter().copied().find(|&space| handle.get_index() == self.spaces[space as usize].get_index())
            .ok_or_else(unavailable)?;
        let mut offset = u64::try_from(i128::from(address.get_offset()) - self.delta(space)).map_err(|_| unavailable())?;
        let after = self.regions.partition_point(|r| (r.space, r.offset) <= (space, offset));
        let first = after.checked_sub(1).ok_or_else(unavailable)?;
        if self.regions[first].space != space || offset >= end(&self.regions[first]) {
            return Err(unavailable());
        }
        let mut dst = dst;
        for region in &self.regions[first..] {
            if region.space != space || region.offset > offset { break; }
            let within = (offset - region.offset) as usize;
            let count = dst.len().min(region.bytes.len() - within);
            dst[..count].copy_from_slice(&region.bytes[within..within + count]);
            dst = &mut dst[count..];
            if dst.is_empty() { return Ok(()); }
            offset += count as u64;
        }
        if !self.regions[first].executable { return Err(unavailable()); }
        // Decoder lookahead can cross a mapped CODE tail. Issue context is
        // painted only over actual complete fetches, never onto this padding.
        dst.fill(0);
        Ok(())
    }

    fn get_readonly(&self, ranges: &mut RangeList) {
        for region in self.regions.iter().filter(|r| r.readonly) {
            if let Some((start, end)) = self.range(region) {
                ranges.insert_range(Rc::clone(&self.spaces[region.space as usize]), start, end - 1);
            }
        }
    }

    fn open_section_info(&self) { self.section.set(0); }

    fn get_next_section(&self, out: &mut LoadImageSection) -> bool {
        for index in self.section.get()..self.regions.len() {
            let region = &self.regions[index];
            let Some((start, end)) = self.range(region) else { continue; };
            let kind = if region.executable { section_flags::CODE } else { section_flags::DATA };
            *out = LoadImageSection {
                address: Address::new(Rc::clone(&self.spaces[region.space as usize]), start),
                size: end - start,
                flags: kind | if region.readonly { section_flags::READONLY } else { 0 },
            };
            self.section.set(index + 1);
            return index + 1 < self.regions.len();
        }
        self.section.set(self.regions.len());
        *out = LoadImageSection::default();
        false
    }
}

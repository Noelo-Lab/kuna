//! EFI 1.1 ("UEFI") and Tiano decompression: the LZ77 + Huffman format of the
//! UEFI specification's standard compression section and of EDK2's Tiano
//! custom-decompress GUID section.
//!
//! A port of EDK2 `BaseTools/Source/C/Common/Decompress.c`. The two variants
//! differ only in the width of the position-set size field: 4 bits for the
//! EFI 1.1 format, 5 for Tiano. The stream is an 8-byte header (compressed
//! size, original size, little-endian) followed by blocks.

const BITBUFSIZ: u32 = 32;
const MAXMATCH: u16 = 256;
const THRESHOLD: u16 = 3;
const CODE_BIT: u16 = 16;
const NC: usize = 0xff + MAXMATCH as usize + 2 - THRESHOLD as usize;
const CBIT: u16 = 9;
const MAXPBIT: u16 = 5;
const TBIT: u16 = 5;
const MAXNP: usize = (1 << MAXPBIT) - 1;
const NT: usize = CODE_BIT as usize + 3;
const NPT: usize = if NT > MAXNP { NT } else { MAXNP };

/// Which format: the position-set width.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Variant {
    /// EFI 1.1 / UEFI standard compression (4-bit position set).
    Efi,
    /// EDK2 Tiano compression (5-bit position set).
    Tiano,
}

struct Sd<'a> {
    src: &'a [u8],
    dst: Vec<u8>,
    in_buf: usize,
    bit_count: u16,
    bit_buf: u32,
    sub_bit_buf: u32,
    block_size: u16,
    comp_size: u32,
    orig_size: usize,
    bad: bool,
    pbit: u16,
    left: Vec<u16>,
    right: Vec<u16>,
    c_len: [u8; NC],
    pt_len: [u8; NPT],
    c_table: Vec<u16>,
    pt_table: [u16; 256],
}

impl Sd<'_> {
    fn fill_buf(&mut self, mut n: u16) {
        self.bit_buf = ((u64::from(self.bit_buf)) << n) as u32;
        while n > self.bit_count {
            n -= self.bit_count;
            self.bit_buf |= ((u64::from(self.sub_bit_buf)) << n) as u32;
            if self.comp_size > 0 {
                self.comp_size -= 1;
                self.sub_bit_buf = u32::from(*self.src.get(self.in_buf).unwrap_or(&0));
                self.in_buf += 1;
            } else {
                self.sub_bit_buf = 0;
            }
            self.bit_count = 8;
        }
        self.bit_count -= n;
        self.bit_buf |= self.sub_bit_buf >> self.bit_count;
    }

    fn get_bits(&mut self, n: u16) -> u32 {
        let out = if n == 0 { 0 } else { self.bit_buf >> (BITBUFSIZ - u32::from(n)) };
        self.fill_buf(n);
        out
    }

    /// `MakeTable`; false on a corrupt table.
    fn make_table(&mut self, num_char: usize, bit_len: &[u8], table_bits: u16, which: TableKind) -> bool {
        let mut count = [0u16; 17];
        let mut weight = [0u16; 17];
        let mut start = [0u16; 18];
        for &l in &bit_len[..num_char] {
            if l > 16 {
                return false;
            }
            count[l as usize] += 1;
        }
        for i in 1..=16usize {
            start[i + 1] = start[i].wrapping_add(count[i].wrapping_shl(16 - i as u32));
        }
        if start[17] != 0 {
            return false;
        }
        let ju_bits = 16 - table_bits;
        for i in 1..=table_bits as usize {
            start[i] >>= ju_bits;
            weight[i] = 1 << (table_bits as usize - i);
        }
        for (i, w) in weight.iter_mut().enumerate().skip(table_bits as usize + 1) {
            *w = 1 << (16 - i);
        }
        let mut index = (start[table_bits as usize + 1] >> ju_bits) as usize;
        let max_len = 1usize << table_bits;
        if index != 0 {
            while index != max_len {
                self.table_mut(which)[index] = 0;
                index += 1;
            }
        }
        let mut avail = num_char as u16;
        let mask = 1u16 << (15 - table_bits);
        for ch in 0..num_char {
            let len = bit_len[ch] as usize;
            if len == 0 || len >= 17 {
                continue;
            }
            let next = start[len].wrapping_add(weight[len]);
            if len <= table_bits as usize {
                if start[len] >= next || next as usize > max_len {
                    return false;
                }
                for i in start[len]..next {
                    self.table_mut(which)[i as usize] = ch as u16;
                }
            } else {
                let mut index3 = start[len];
                let mut slot = Slot::Table((index3 >> ju_bits) as usize);
                let mut rest = len - table_bits as usize;
                while rest != 0 {
                    if self.read_slot(slot, which) == 0 {
                        if avail as usize >= self.left.len() {
                            return false;
                        }
                        self.right[avail as usize] = 0;
                        self.left[avail as usize] = 0;
                        self.write_slot(slot, which, avail);
                        avail += 1;
                    }
                    let node = self.read_slot(slot, which) as usize;
                    if node >= self.left.len() {
                        return false;
                    }
                    slot = if index3 & mask != 0 { Slot::Right(node) } else { Slot::Left(node) };
                    index3 <<= 1;
                    rest -= 1;
                }
                self.write_slot(slot, which, ch as u16);
            }
            start[len] = next;
        }
        true
    }

    fn table_mut(&mut self, which: TableKind) -> &mut [u16] {
        match which {
            TableKind::C => &mut self.c_table,
            TableKind::Pt => &mut self.pt_table,
        }
    }

    fn read_slot(&mut self, slot: Slot, which: TableKind) -> u16 {
        match slot {
            Slot::Table(i) => self.table_mut(which)[i],
            Slot::Left(i) => self.left[i],
            Slot::Right(i) => self.right[i],
        }
    }

    fn write_slot(&mut self, slot: Slot, which: TableKind, v: u16) {
        match slot {
            Slot::Table(i) => self.table_mut(which)[i] = v,
            Slot::Left(i) => self.left[i] = v,
            Slot::Right(i) => self.right[i] = v,
        }
    }

    fn walk(&self, mut val: u16, limit: usize, first_bit: u32) -> u16 {
        let mut mask = 1u32 << (BITBUFSIZ - 1 - first_bit);
        while val as usize >= limit {
            let i = val as usize;
            if i >= self.left.len() || mask == 0 {
                return 0;
            }
            val = if self.bit_buf & mask != 0 { self.right[i] } else { self.left[i] };
            mask >>= 1;
        }
        val
    }

    fn decode_p(&mut self) -> u32 {
        let val = self.walk(self.pt_table[(self.bit_buf >> (BITBUFSIZ - 8)) as usize], MAXNP, 8);
        self.fill_buf(u16::from(self.pt_len[val as usize]));
        if val > 1 {
            (1u32 << (val - 1)) + self.get_bits(val - 1)
        } else {
            u32::from(val)
        }
    }

    fn read_pt_len(&mut self, nn: usize, nbit: u16, special: i32) -> bool {
        let number = self.get_bits(nbit) as usize;
        if number == 0 {
            let c = self.get_bits(nbit) as u16;
            self.pt_table = [c; 256];
            self.pt_len[..nn].fill(0);
            return true;
        }
        let mut index = 0usize;
        while index < number && index < NPT {
            let mut c = (self.bit_buf >> (BITBUFSIZ - 3)) as u16;
            if c == 7 {
                let mut mask = 1u32 << (BITBUFSIZ - 1 - 3);
                while mask & self.bit_buf != 0 {
                    mask >>= 1;
                    c += 1;
                }
            }
            self.fill_buf(if c < 7 { 3 } else { c - 3 });
            self.pt_len[index] = c as u8;
            index += 1;
            if index as i32 == special {
                let mut zeros = self.get_bits(2) as i32 - 1;
                while zeros >= 0 && index < NPT {
                    self.pt_len[index] = 0;
                    index += 1;
                    zeros -= 1;
                }
            }
        }
        while index < nn && index < NPT {
            self.pt_len[index] = 0;
            index += 1;
        }
        let lens = self.pt_len;
        self.make_table(nn, &lens, 8, TableKind::Pt)
    }

    fn read_c_len(&mut self) {
        let number = self.get_bits(CBIT) as usize;
        if number == 0 {
            let c = self.get_bits(CBIT) as u16;
            self.c_len = [0; NC];
            self.c_table.fill(c);
            return;
        }
        let mut index = 0usize;
        while index < number && index < NC {
            let c = self.walk(self.pt_table[(self.bit_buf >> (BITBUFSIZ - 8)) as usize], NT, 8);
            self.fill_buf(u16::from(self.pt_len[c as usize]));
            if c <= 2 {
                let run = match c {
                    0 => 1,
                    1 => self.get_bits(4) as usize + 3,
                    _ => self.get_bits(CBIT) as usize + 20,
                };
                for _ in 0..run {
                    if index < NC {
                        self.c_len[index] = 0;
                        index += 1;
                    }
                }
            } else {
                self.c_len[index] = (c - 2) as u8;
                index += 1;
            }
        }
        self.c_len[index..].fill(0);
        let lens = self.c_len;
        if !self.make_table(NC, &lens, 12, TableKind::C) {
            self.bad = true;
        }
    }

    fn decode_c(&mut self) -> u16 {
        if self.block_size == 0 {
            self.block_size = self.get_bits(16) as u16;
            if !self.read_pt_len(NT, TBIT, 3) {
                self.bad = true;
                return 0;
            }
            self.read_c_len();
            if self.bad || !self.read_pt_len(MAXNP, self.pbit, -1) {
                self.bad = true;
                return 0;
            }
        }
        self.block_size = self.block_size.wrapping_sub(1);
        let c = self.walk(self.c_table[(self.bit_buf >> (BITBUFSIZ - 12)) as usize], NC, 12);
        self.fill_buf(u16::from(self.c_len[c as usize]));
        c
    }

    fn decode(&mut self) {
        loop {
            let c = self.decode_c();
            if self.bad {
                return;
            }
            if c < 256 {
                self.dst.push(c as u8);
                if self.dst.len() >= self.orig_size {
                    return;
                }
            } else {
                let len = c - (0x100 - THRESHOLD);
                let back = self.decode_p() as usize + 1;
                let Some(mut from) = self.dst.len().checked_sub(back) else {
                    self.bad = true;
                    return;
                };
                for _ in 0..len {
                    if self.dst.len() >= self.orig_size {
                        return;
                    }
                    let b = self.dst[from];
                    self.dst.push(b);
                    from += 1;
                }
                if self.dst.len() >= self.orig_size {
                    return;
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum TableKind {
    C,
    Pt,
}

#[derive(Clone, Copy)]
enum Slot {
    Table(usize),
    Left(usize),
    Right(usize),
}

/// Decompress an EFI 1.1 or Tiano stream (8-byte size header first).
pub fn decompress(data: &[u8], variant: Variant) -> Result<Vec<u8>, String> {
    let word = |at: usize| data.get(at..at + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    let (Some(comp), Some(orig)) = (word(0), word(4)) else { return Err("short EFI/Tiano header".into()) };
    if comp as usize > data.len() - 8 {
        return Err(format!("EFI/Tiano compressed size {comp} past the {} bytes given", data.len() - 8));
    }
    if orig > 256 << 20 {
        return Err(format!("EFI/Tiano original size {orig} is too large"));
    }
    let mut sd = Sd {
        src: &data[8..],
        dst: Vec::with_capacity(orig as usize),
        in_buf: 0,
        bit_count: 0,
        bit_buf: 0,
        sub_bit_buf: 0,
        block_size: 0,
        comp_size: comp,
        orig_size: orig as usize,
        bad: false,
        pbit: if variant == Variant::Efi { 4 } else { MAXPBIT },
        left: vec![0; 2 * NC - 1],
        right: vec![0; 2 * NC - 1],
        c_len: [0; NC],
        pt_len: [0; NPT],
        c_table: vec![0; 4096],
        pt_table: [0; 256],
    };
    if orig == 0 {
        return Ok(Vec::new());
    }
    sd.fill_buf(BITBUFSIZ as u16);
    sd.decode();
    if sd.bad || sd.dst.len() != sd.orig_size {
        return Err(format!("{variant:?} stream is corrupt"));
    }
    Ok(sd.dst)
}

/// Decompress trying `first`, then the other variant: the standard
/// compression section names EFI 1.1, but some firmware stores Tiano there.
pub fn decompress_either(data: &[u8], first: Variant) -> Result<Vec<u8>, String> {
    let other = if first == Variant::Efi { Variant::Tiano } else { Variant::Efi };
    decompress(data, first).or_else(|e| decompress(data, other).map_err(|_| e))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> Vec<u8> {
        std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)).unwrap()
    }

    #[test]
    fn both_variants_round_trip_edk2_tianocompress_output() {
        let plain = fixture("uefi_diskio_x64.efi");
        assert_eq!(decompress(&fixture("uefi_diskio_x64.tiano"), Variant::Tiano).unwrap(), plain);
        assert_eq!(decompress(&fixture("uefi_diskio_x64.eficomp"), Variant::Efi).unwrap(), plain);
        assert_eq!(decompress_either(&fixture("uefi_diskio_x64.tiano"), Variant::Efi).unwrap(), plain);
    }

    #[test]
    fn a_truncated_stream_is_an_error() {
        let data = fixture("uefi_diskio_x64.tiano");
        assert!(decompress(&data[..data.len() / 2], Variant::Tiano).is_err());
    }
}

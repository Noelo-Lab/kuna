//! Replaces java.util.zip.
//!
//! A read-only zip archive reader with the semantics of JDK 21's
//! `java.util.zip.ZipFile`, which decides what Vineflower sees in a jar: the end
//! record is located the same way, entries are listed in central-directory
//! order, a stub in front of the archive (the `JM\1\0` header of a `.jmod`, a
//! launcher script) is absorbed through the central-directory offset, and
//! [`ZipReader::get_entry`] resolves names like `ZipFile.getEntry`. Sizes come
//! from the central directory, so data-descriptor entries need nothing special.
//! Stored and deflated entries are supported; encrypted entries and other
//! compression methods make the archive fail to open, as in the JDK.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use flate2::{Decompress, FlushDecompress, Status};

pub const STORED: u16 = 0;
pub const DEFLATED: u16 = 8;

const LOCSIG: u32 = 0x0403_4b50;
const CENSIG: u32 = 0x0201_4b50;
const ENDSIG: u32 = 0x0605_4b50;
const ZIP64_ENDSIG: u32 = 0x0606_4b50;
const ZIP64_LOCSIG: u32 = 0x0706_4b50;
const LOCHDR: usize = 30;
const CENHDR: usize = 46;
const ENDHDR: usize = 22;
const ZIP64_ENDHDR: usize = 56;
const ZIP64_LOCHDR: usize = 20;
const END_MAXLEN: i64 = 0xFFFF + ENDHDR as i64;
const READBLOCKSZ: i64 = 128;
const ZIP64_MAGICVAL: u64 = 0xFFFF_FFFF;
const ZIP64_MAGICCOUNT: u64 = 0xFFFF;
const EXTID_ZIP64: u16 = 0x0001;

/// The largest array a JVM can allocate; no entry is inflated past it.
pub const MAX_ARRAY_SIZE: u64 = i32::MAX as u64 - 8;
const PREALLOC_LIMIT: u64 = 1 << 24;

/// One central-directory record.
#[derive(Clone, Debug)]
pub struct ZipEntry {
    pub name: String,
    pub method: u16,
    pub flags: u16,
    pub crc: u32,
    pub size: u64,
    pub compressed_size: u64,
    /// Offset of the local header as recorded in the archive (before the stub shift).
    pub local_header_offset: u64,
}

impl ZipEntry {
    /// `ZipEntry.isDirectory()`.
    pub fn is_directory(&self) -> bool {
        self.name.ends_with('/')
    }
}

/// An open zip archive.
pub struct ZipReader {
    path: PathBuf,
    file: RefCell<Option<File>>,
    file_len: u64,
    locpos: u64,
    entries: Vec<ZipEntry>,
    index: BTreeMap<String, usize>,
}

struct End {
    centot: u64,
    cenlen: u64,
    cenoff: u64,
    endpos: u64,
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn get16(b: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([b[off], b[off + 1]])
}

fn get32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

fn get64(b: &[u8], off: usize) -> u64 {
    let mut a = [0u8; 8];
    a.copy_from_slice(&b[off..off + 8]);
    u64::from_le_bytes(a)
}

fn read_fully_at(file: &File, buf: &mut [u8], pos: u64) -> io::Result<usize> {
    let mut f = file;
    f.seek(SeekFrom::Start(pos))?;
    let mut done = 0;
    while done < buf.len() {
        match f.read(&mut buf[done..]) {
            Ok(0) => break,
            Ok(n) => done += n,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    Ok(done)
}

impl ZipReader {
    /// `new ZipFile(file)`: reads and validates the central directory.
    pub fn open(path: &Path) -> io::Result<ZipReader> {
        let file = File::open(path)?;
        let file_len = file.metadata()?.len();
        let (locpos, entries) = Self::init_cen(&file, file_len)?;
        let mut index = BTreeMap::new();
        for (i, e) in entries.iter().enumerate() {
            index.insert(e.name.clone(), i);
        }
        Ok(ZipReader {
            path: path.to_path_buf(),
            file: RefCell::new(Some(file)),
            file_len,
            locpos,
            entries,
            index,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The entries in central-directory order (`ZipFile.entries()`).
    pub fn entries(&self) -> &[ZipEntry] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Bytes in front of the archive proper (4 for a `.jmod`).
    pub fn prefix_len(&self) -> u64 {
        self.locpos
    }

    /// `ZipFile.getEntry(name)`: the last record named `name` or `name + "/"`.
    pub fn get_entry(&self, name: &str) -> Option<&ZipEntry> {
        let exact = self.index.get(name).copied();
        let slashed = self.index.get(&format!("{name}/")).copied();
        let pick = match (exact, slashed) {
            (Some(a), Some(b)) => Some(a.max(b)),
            (a, b) => a.or(b),
        };
        pick.map(|i| &self.entries[i])
    }

    /// The last record named exactly `name`.
    pub fn get_entry_exact(&self, name: &str) -> Option<&ZipEntry> {
        self.index.get(name).map(|&i| &self.entries[i])
    }

    /// Read the entry `get_entry(name)` finds; `None` when there is none.
    pub fn read(&self, name: &str) -> io::Result<Option<Vec<u8>>> {
        match self.get_entry(name) {
            Some(e) => self.read_entry(e).map(Some),
            None => Ok(None),
        }
    }

    /// The uncompressed bytes of `entry`.
    pub fn read_entry(&self, entry: &ZipEntry) -> io::Result<Vec<u8>> {
        let guard = self.file.borrow();
        let Some(file) = guard.as_ref() else {
            return Err(io::Error::other("zip file closed"));
        };
        let loc_pos = self
            .locpos
            .checked_add(entry.local_header_offset)
            .ok_or_else(|| invalid("invalid LOC header offset"))?;
        let mut loc = [0u8; LOCHDR];
        if loc_pos > self.file_len || read_fully_at(file, &mut loc, loc_pos)? != LOCHDR {
            return Err(invalid("ZipFile error reading zip file"));
        }
        if get32(&loc, 0) != LOCSIG {
            return Err(invalid("ZipFile invalid LOC header (bad signature)"));
        }
        let data_pos = loc_pos + LOCHDR as u64 + get16(&loc, 26) as u64 + get16(&loc, 28) as u64;
        let csize = entry.compressed_size;
        if data_pos > self.file_len || csize > self.file_len - data_pos {
            return Err(invalid(format!("truncated zip entry {}", entry.name)));
        }
        let mut raw = vec![0u8; csize as usize];
        if read_fully_at(file, &mut raw, data_pos)? != raw.len() {
            return Err(invalid(format!("truncated zip entry {}", entry.name)));
        }
        drop(guard);
        match entry.method {
            STORED => Ok(raw),
            DEFLATED => inflate(&raw, false, entry.size.min(MAX_ARRAY_SIZE), false)
                .map_err(|e| invalid(format!("{} ({})", e, entry.name))),
            m => Err(invalid(format!("invalid compression method {m}"))),
        }
    }

    pub fn is_closed(&self) -> bool {
        self.file.borrow().is_none()
    }

    /// `ZipFile.close()`; later reads fail.
    pub fn close(&self) {
        self.file.borrow_mut().take();
    }

    fn find_end(file: &File, ziplen: u64) -> io::Result<End> {
        if ziplen == 0 {
            return Err(invalid("zip file is empty"));
        }
        let ziplen = i64::try_from(ziplen).map_err(|_| invalid("zip file too large"))?;
        let min_hdr = (ziplen - END_MAXLEN).max(0);
        let min_pos = min_hdr - (READBLOCKSZ - ENDHDR as i64);
        let tail_start = min_pos.max(0);
        let mut tail = vec![0u8; (ziplen - tail_start) as usize];
        if read_fully_at(file, &mut tail, tail_start as u64)? != tail.len() {
            return Err(invalid("zip END header not found"));
        }
        let mut pos = ziplen - READBLOCKSZ;
        while pos >= min_pos {
            for i in (0..=(READBLOCKSZ - ENDHDR as i64)).rev() {
                let abs = pos + i;
                if abs < tail_start {
                    continue;
                }
                let off = (abs - tail_start) as usize;
                if get32(&tail, off) != ENDSIG {
                    continue;
                }
                let endbuf = &tail[off..off + ENDHDR];
                let mut end = End {
                    centot: get16(endbuf, 10) as u64,
                    cenlen: get32(endbuf, 12) as u64,
                    cenoff: get32(endbuf, 16) as u64,
                    endpos: abs as u64,
                };
                let comlen = get16(endbuf, 20) as i64;
                if abs + ENDHDR as i64 + comlen != ziplen {
                    let cenpos = abs - end.cenlen as i64;
                    let locpos = cenpos - end.cenoff as i64;
                    let mut sbuf = [0u8; 4];
                    let sig_at = |p: i64, sbuf: &mut [u8; 4]| -> io::Result<Option<u32>> {
                        Ok((read_fully_at(file, sbuf, p as u64)? == 4).then(|| get32(sbuf, 0)))
                    };
                    if cenpos < 0
                        || locpos < 0
                        || sig_at(cenpos, &mut sbuf)? != Some(CENSIG)
                        || sig_at(locpos, &mut sbuf)? != Some(LOCSIG)
                    {
                        continue;
                    }
                }
                Self::apply_zip64_end(file, ziplen as u64, &mut end).ok();
                return Ok(end);
            }
            pos -= READBLOCKSZ - ENDHDR as i64;
        }
        Err(invalid("zip END header not found"))
    }

    fn apply_zip64_end(file: &File, ziplen: u64, end: &mut End) -> io::Result<()> {
        if end.endpos < ZIP64_LOCHDR as u64 {
            return Ok(());
        }
        let mut loc64 = [0u8; ZIP64_LOCHDR];
        if read_fully_at(file, &mut loc64, end.endpos - ZIP64_LOCHDR as u64)? != ZIP64_LOCHDR
            || get32(&loc64, 0) != ZIP64_LOCSIG
        {
            return Ok(());
        }
        let end64pos = get64(&loc64, 8);
        if end64pos > ziplen {
            return Ok(());
        }
        let mut end64 = [0u8; ZIP64_ENDHDR];
        if read_fully_at(file, &mut end64, end64pos)? != ZIP64_ENDHDR
            || get32(&end64, 0) != ZIP64_ENDSIG
        {
            return Ok(());
        }
        let cenlen64 = get64(&end64, 40);
        let cenoff64 = get64(&end64, 48);
        let centot64 = get64(&end64, 32);
        if cenlen64 != end.cenlen && end.cenlen != ZIP64_MAGICVAL
            || cenoff64 != end.cenoff && end.cenoff != ZIP64_MAGICVAL
            || centot64 != end.centot && end.centot != ZIP64_MAGICCOUNT
        {
            return Ok(());
        }
        end.cenlen = cenlen64;
        end.cenoff = cenoff64;
        end.centot = centot64;
        end.endpos = end64pos;
        Ok(())
    }

    fn init_cen(file: &File, file_len: u64) -> io::Result<(u64, Vec<ZipEntry>)> {
        let end = Self::find_end(file, file_len)?;
        if end.endpos == 0 {
            return Ok((0, Vec::new()));
        }
        if end.cenlen > end.endpos {
            return Err(invalid("invalid END header (bad central directory size)"));
        }
        let cenpos = end.endpos - end.cenlen;
        let locpos = cenpos
            .checked_sub(end.cenoff)
            .ok_or_else(|| invalid("invalid END header (bad central directory offset)"))?;
        if end.cenlen + ENDHDR as u64 >= i32::MAX as u64 {
            return Err(invalid(
                "invalid END header (central directory size too large)",
            ));
        }
        let mut cen = vec![0u8; end.cenlen as usize + ENDHDR];
        if read_fully_at(file, &mut cen, cenpos)? != cen.len() {
            return Err(invalid("read CEN tables failed"));
        }
        let mut entries = Vec::with_capacity(end.centot.min(1 << 16) as usize);
        let limit = cen.len() - ENDHDR;
        let mut pos = 0usize;
        while pos + CENHDR <= limit {
            let (entry, next) = Self::check_and_add_entry(&cen, pos)?;
            entries.push(entry);
            pos = next;
        }
        if pos + ENDHDR != cen.len() {
            return Err(invalid("invalid CEN header (bad header size)"));
        }
        Ok((locpos, entries))
    }

    fn check_and_add_entry(cen: &[u8], pos: usize) -> io::Result<(ZipEntry, usize)> {
        if get32(cen, pos) != CENSIG {
            return Err(invalid("invalid CEN header (bad signature)"));
        }
        let flags = get16(cen, pos + 8);
        let method = get16(cen, pos + 10);
        if flags & 1 != 0 {
            return Err(invalid("invalid CEN header (encrypted entry)"));
        }
        if method != STORED && method != DEFLATED {
            return Err(invalid(format!(
                "invalid CEN header (bad compression method: {method})"
            )));
        }
        let nlen = get16(cen, pos + 28) as usize;
        let elen = get16(cen, pos + 30) as usize;
        let clen = get16(cen, pos + 32) as usize;
        let header_size = CENHDR + nlen + clen + elen;
        if header_size > 0xFFFF || pos + header_size > cen.len() - ENDHDR {
            return Err(invalid("invalid CEN header (bad header size)"));
        }
        let mut csize = get32(cen, pos + 20) as u64;
        let mut size = get32(cen, pos + 24) as u64;
        let mut offset = get32(cen, pos + 42) as u64;
        let disk = get16(cen, pos + 34) as u64;
        let name_pos = pos + CENHDR;
        let extra = &cen[name_pos + nlen..name_pos + nlen + elen];
        if elen > 0 {
            Self::read_extra(extra, &mut size, &mut csize, &mut offset)?;
        } else if csize == ZIP64_MAGICVAL
            || size == ZIP64_MAGICVAL
            || offset == ZIP64_MAGICVAL
            || disk == ZIP64_MAGICCOUNT
        {
            return Err(invalid("Invalid CEN header (invalid zip64 extra len size)"));
        }
        let bad_name = || invalid("invalid CEN header (bad entry name or comment)");
        let name = std::str::from_utf8(&cen[name_pos..name_pos + nlen])
            .map_err(|_| bad_name())?
            .to_string();
        if clen > 0 {
            let start = name_pos + nlen + elen;
            std::str::from_utf8(&cen[start..start + clen]).map_err(|_| bad_name())?;
        }
        let entry = ZipEntry {
            name,
            method,
            flags,
            crc: get32(cen, pos + 16),
            size,
            compressed_size: csize,
            local_header_offset: offset,
        };
        Ok((entry, pos + header_size))
    }

    fn read_extra(
        extra: &[u8],
        size: &mut u64,
        csize: &mut u64,
        offset: &mut u64,
    ) -> io::Result<()> {
        let mut off = 0usize;
        while off + 4 <= extra.len() {
            let tag = get16(extra, off);
            let block = get16(extra, off + 2) as usize;
            off += 4;
            if off + block > extra.len() {
                return Err(invalid(format!(
                    "Invalid CEN header (invalid extra data field size for tag: 0x{tag:04x})"
                )));
            }
            if tag == EXTID_ZIP64 {
                let wants = [*size, *csize, *offset].contains(&ZIP64_MAGICVAL);
                if block == 0 {
                    if wants {
                        return Err(invalid(
                            "Invalid CEN header (invalid zip64 extra data field size)",
                        ));
                    }
                } else {
                    if !matches!(block, 8 | 16 | 24 | 28) {
                        return Err(invalid(
                            "Invalid CEN header (invalid zip64 extra data field size)",
                        ));
                    }
                    let mut p = off;
                    for field in [&mut *size, &mut *csize, &mut *offset] {
                        if *field != ZIP64_MAGICVAL {
                            continue;
                        }
                        if p + 8 > off + block {
                            return Err(invalid("Invalid zip64 extra block size value"));
                        }
                        let v = get64(extra, p);
                        if v > i64::MAX as u64 {
                            return Err(invalid("Invalid zip64 extra block size value"));
                        }
                        *field = v;
                        p += 8;
                    }
                }
            }
            off += block;
        }
        Ok(())
    }
}

impl std::fmt::Debug for ZipReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ZipReader({}, {} entries)",
            self.path.display(),
            self.entries.len()
        )
    }
}

/// Inflate a raw (`zlib == false`) or zlib-wrapped deflate stream that must end
/// within `input`. At most `limit` bytes are produced: more is an error, or with
/// `stop_at_limit` the output simply ends there (`Inflater` into a fixed array).
pub fn inflate(input: &[u8], zlib: bool, limit: u64, stop_at_limit: bool) -> io::Result<Vec<u8>> {
    let mut out: Vec<u8> = Vec::with_capacity(limit.min(PREALLOC_LIMIT) as usize);
    let mut d = Decompress::new(zlib);
    loop {
        if out.len() == out.capacity() && (out.len() as u64) < limit {
            let room = (limit - out.len() as u64).min(out.capacity().max(8192) as u64);
            out.reserve_exact(room as usize);
        }
        let consumed = d.total_in() as usize;
        let produced = out.len();
        let status = d
            .decompress_vec(&input[consumed..], &mut out, FlushDecompress::None)
            .map_err(|e| invalid(format!("invalid deflate stream: {e}")))?;
        if status == Status::StreamEnd || (stop_at_limit && out.len() as u64 >= limit) {
            return Ok(out);
        }
        if d.total_in() as usize == consumed && out.len() == produced {
            if out.len() as u64 >= limit {
                return Err(invalid("inflated data exceeds the recorded size"));
            }
            return Err(invalid("Unexpected end of ZLIB input stream"));
        }
    }
}

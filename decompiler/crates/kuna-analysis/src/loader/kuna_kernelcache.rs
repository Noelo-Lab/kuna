//! (kuna) Apple firmware containers and kernel collections, peeled at load.
//!
//! An iOS kernelcache ships as an IMG4 (or bare IM4P) DER container whose payload
//! is an LZFSE or `complzss` stream, and the decoded image is an `MH_FILESET`
//! kernel collection: one Mach-O header per `LC_FILESET_ENTRY` (the kernel and
//! every kext), all sharing one file and one `LC_DYLD_CHAINED_FIXUPS` table.
//! [`peel`] turns such an input into one thin Mach-O the rest of the loader reads
//! unchanged: it unwraps the container, applies the collection's chained-fixup
//! rebases to the pointer slots, and moves the chosen entry's header and load
//! commands to file offset 0. Every entry's file offsets are absolute in the
//! collection, so no load command needs rewriting. Gated by the `kernelcache`
//! option ([`KernelcacheChoice`]).

use kuna_decomp::kuna_kernelcache::KernelcacheChoice;

mod fileset;
mod lzfse;
mod lzss;

#[cfg(test)]
mod tests;

pub use fileset::FilesetEntry;

/// The largest payload the decompressors will produce.
const DECODED_LIMIT: usize = 1 << 31;

/// The loadable image after [`peel`], plus a one-line note on what was done.
pub struct Peeled {
    pub bytes: Vec<u8>,
    pub note: Option<String>,
}

/// Whether `bytes` is an IMG4 / IM4P container or a bare compressed payload.
pub fn is_container(bytes: &[u8]) -> bool {
    der_magic(bytes).is_some() || lzss::is_complzss(bytes) || lzfse::is_lzfse(bytes)
}

/// Unwrap a container and select one fileset entry, per `choice`. Input that is
/// neither a container nor an `MH_FILESET` comes back unchanged.
pub fn peel(bytes: Vec<u8>, choice: &KernelcacheChoice) -> Result<Peeled, String> {
    if *choice == KernelcacheChoice::Off {
        return Ok(Peeled { bytes, note: None });
    }
    let mut notes = Vec::new();
    let bytes = if is_container(&bytes) {
        let (payload, note) = unwrap_payload(&bytes)?;
        notes.push(note);
        payload
    } else {
        bytes
    };
    let bytes = if fileset::is_fileset(&bytes) {
        let (image, note) = fileset::extract(bytes, choice)?;
        notes.push(note);
        image
    } else {
        bytes
    };
    let note = (!notes.is_empty()).then(|| notes.join("; "));
    Ok(Peeled { bytes, note })
}

/// The entries of an `MH_FILESET` (after unwrapping any container), in load
/// command order; empty for any other image.
pub fn fileset_entries(bytes: &[u8]) -> Result<Vec<FilesetEntry>, String> {
    if is_container(bytes) {
        let (payload, _) = unwrap_payload(bytes)?;
        return Ok(fileset::entries(&payload).unwrap_or_default());
    }
    Ok(fileset::entries(bytes).unwrap_or_default())
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum DerMagic {
    Img4,
    Im4p,
}

fn der_magic(bytes: &[u8]) -> Option<DerMagic> {
    if bytes.first() != Some(&TAG_SEQUENCE) {
        return None;
    }
    let window = &bytes[..bytes.len().min(16)];
    let has = |magic: &[u8; 4]| {
        window.windows(6).any(|w| w[..2] == [TAG_IA5STRING, 4] && w[2..] == magic[..])
    };
    if has(b"IMG4") {
        Some(DerMagic::Img4)
    } else if has(b"IM4P") {
        Some(DerMagic::Im4p)
    } else {
        None
    }
}

fn unwrap_payload(bytes: &[u8]) -> Result<(Vec<u8>, String), String> {
    match der_magic(bytes) {
        Some(magic) => {
            let im4p = Im4p::parse(bytes, magic)?;
            if im4p.encrypted {
                return Err(format!(
                    "IM4P payload '{}' is encrypted (it carries a KBAG); decrypt it first, \
                     then load the decrypted IM4P or its raw payload",
                    im4p.kind
                ));
            }
            let hint = im4p.decoded_size.unwrap_or(0);
            let (payload, codec) = decompress(im4p.data, hint)?;
            if let Some(size) = im4p.decoded_size.filter(|&s| s != payload.len()) {
                return Err(format!(
                    "IM4P '{}' declares {size} decoded bytes but decoded to {}",
                    im4p.kind,
                    payload.len()
                ));
            }
            let note = format!(
                "unwrapped {} payload '{}' ({codec}, {} -> {} bytes)",
                if magic == DerMagic::Img4 { "IMG4" } else { "IM4P" },
                im4p.kind,
                im4p.data.len(),
                payload.len()
            );
            Ok((payload, note))
        }
        None => {
            let (payload, codec) = decompress(bytes, 0)?;
            let note = format!("decoded {codec} payload ({} -> {} bytes)", bytes.len(), payload.len());
            Ok((payload, note))
        }
    }
}

fn decompress(data: &[u8], size_hint: usize) -> Result<(Vec<u8>, &'static str), String> {
    if lzss::is_complzss(data) {
        return Ok((lzss::decode(data, DECODED_LIMIT)?, "complzss"));
    }
    if lzfse::is_lzfse(data) {
        let out = lzfse::decode(data, size_hint, DECODED_LIMIT).map_err(|e| e.to_string())?;
        return Ok((out, "LZFSE"));
    }
    Ok((data.to_vec(), "uncompressed"))
}

const TAG_INTEGER: u8 = 0x02;
const TAG_OCTET_STRING: u8 = 0x04;
const TAG_IA5STRING: u8 = 0x16;
const TAG_SEQUENCE: u8 = 0x30;

/// The fields of an IM4P the loader needs.
struct Im4p<'a> {
    kind: String,
    data: &'a [u8],
    encrypted: bool,
    decoded_size: Option<usize>,
}

impl<'a> Im4p<'a> {
    /// `IMG4 ::= SEQUENCE { "IMG4", IM4P, [0] IM4M OPTIONAL, ... }` and
    /// `IM4P ::= SEQUENCE { "IM4P", type, description, OCTET STRING payload,
    /// OCTET STRING kbag OPTIONAL, SEQUENCE { INTEGER algorithm, INTEGER size }
    /// OPTIONAL, [0] properties OPTIONAL }`.
    fn parse(bytes: &'a [u8], magic: DerMagic) -> Result<Self, String> {
        let mut outer = Der::new(bytes);
        let body = outer.expect(TAG_SEQUENCE, "container")?;
        let mut seq = Der::new(body);
        let im4p_body = if magic == DerMagic::Img4 {
            expect_magic(&mut seq, b"IMG4")?;
            seq.expect(TAG_SEQUENCE, "IM4P")?
        } else {
            body
        };
        let mut im4p = Der::new(im4p_body);
        expect_magic(&mut im4p, b"IM4P")?;
        let kind = String::from_utf8_lossy(im4p.expect(TAG_IA5STRING, "IM4P type")?).into_owned();
        let mut data = None;
        let mut encrypted = false;
        let mut decoded_size = None;
        while let Some((tag, value)) = im4p.next()? {
            match tag {
                TAG_IA5STRING if data.is_none() => {}
                TAG_OCTET_STRING if data.is_none() => data = Some(value),
                TAG_OCTET_STRING => encrypted = true,
                TAG_SEQUENCE => {
                    let mut info = Der::new(value);
                    let _algorithm = info.expect(TAG_INTEGER, "compression algorithm")?;
                    let size = info.expect(TAG_INTEGER, "decoded size")?;
                    if size.len() > 8 {
                        return Err("IM4P decoded size does not fit 64 bits".into());
                    }
                    decoded_size =
                        Some(size.iter().fold(0u64, |acc, &b| (acc << 8) | b as u64) as usize);
                }
                _ => {}
            }
        }
        let data = data.ok_or("IM4P has no payload")?;
        Ok(Self { kind, data, encrypted, decoded_size })
    }
}

fn expect_magic(der: &mut Der, magic: &[u8; 4]) -> Result<(), String> {
    let found = der.expect(TAG_IA5STRING, "magic")?;
    if found != magic {
        return Err(format!(
            "expected {} magic, found {:?}",
            String::from_utf8_lossy(magic),
            String::from_utf8_lossy(found)
        ));
    }
    Ok(())
}

/// A DER reader over one constructed value's contents.
struct Der<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Der<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    /// The next `(first tag byte, contents)`; multi-byte tags are skipped over
    /// and reported by their first byte.
    fn next(&mut self) -> Result<Option<(u8, &'a [u8])>, String> {
        const TRUNC: &str = "truncated DER value in IMG4/IM4P";
        let Some(&tag) = self.buf.get(self.pos) else { return Ok(None) };
        let mut p = self.pos + 1;
        if tag & 0x1f == 0x1f {
            while self.buf.get(p).ok_or(TRUNC)? & 0x80 != 0 {
                p += 1;
            }
            p += 1;
        }
        let first = *self.buf.get(p).ok_or(TRUNC)?;
        p += 1;
        let len = if first < 0x80 {
            first as usize
        } else {
            let n = (first & 0x7f) as usize;
            if n == 0 || n > 8 {
                return Err("unsupported DER length in IMG4/IM4P".into());
            }
            let bytes = self.buf.get(p..p + n).ok_or(TRUNC)?;
            p += n;
            bytes.iter().fold(0usize, |acc, &b| (acc << 8) | b as usize)
        };
        let end = p.checked_add(len).ok_or(TRUNC)?;
        let value = self.buf.get(p..end).ok_or(TRUNC)?;
        self.pos = end;
        Ok(Some((tag, value)))
    }

    fn expect(&mut self, tag: u8, what: &str) -> Result<&'a [u8], String> {
        match self.next()? {
            Some((found, value)) if found == tag => Ok(value),
            Some((found, _)) => Err(format!("IMG4/IM4P {what}: expected DER tag {tag:#04x}, found {found:#04x}")),
            None => Err(format!("IMG4/IM4P {what}: missing")),
        }
    }
}

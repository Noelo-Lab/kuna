//! Encode a compiled [`SleighBase`] as a binary `.sla` image or debug XML.
//!
//! The runtime types emit the element stream; [`FormatEncode`] supplies the
//! header, packed encoding and deflate compression used by Ghidra's compiler.

use std::io::Write;

use kuna_base::error::KunaResult;
use kuna_base::marshal::XmlEncode;
use kuna_num::opcodes::OpcodeEncoder;

use kuna_sleigh::slaformat::FormatEncode;
use kuna_sleigh::sleighbase::SleighBase;

/// Emit the compiled spec's element stream through the supplied encoder.
pub fn encode_sleigh(base: &SleighBase, encoder: &mut dyn OpcodeEncoder) -> KunaResult<()> {
    base.encode(encoder)
}

/// Write the `sla\x04` header and compressed packed element stream.
pub fn encode_to_sla_writer<W: Write>(base: &SleighBase, w: W) -> KunaResult<()> {
    if let Ok(path) = std::env::var("KUNA_DUMP_XML") {
        let _ = std::fs::write(path, encode_to_xml_bytes(base)?);
    }
    let mut encoder = FormatEncode::new(w, -1);
    {
        let mut packed = encoder.packed();
        base.encode(&mut packed)?;
    }
    encoder.flush()
}

/// Produce the complete binary `.sla` image in memory.
pub fn encode_to_sla_bytes(base: &SleighBase) -> KunaResult<Vec<u8>> {
    let mut out: Vec<u8> = Vec::new();
    encode_to_sla_writer(base, &mut out)?;
    Ok(out)
}

pub(crate) fn encode_to_xml_bytes(base: &SleighBase) -> KunaResult<Vec<u8>> {
    let mut out = Vec::new();
    base.encode(&mut XmlEncode::new(&mut out))?;
    Ok(out)
}

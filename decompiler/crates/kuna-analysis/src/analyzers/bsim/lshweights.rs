//! Vector weighting: `WeightFactory` and `IDFLookup`.
//!
//! A BSim feature is a 32-bit hash; its weight comes from two tables. The **TF**
//! table ranks how often a feature appears in one function (a repeated hash is
//! evidence), and the **IDF** table ranks how common that hash is across the
//! whole corpus (a hash almost every function has is evidence of nothing). Ghidra
//! ships both tables as XML, per processor size, in
//! `Ghidra/Features/BSim/data/lshweights_*.xml`.
//!
//! # Origin (upstream Ghidra)
//!
//! - `Generic/src/main/java/generic/lsh/vector/WeightFactory.java`
//! - `.../vector/IDFLookup.java`
//!
//! # Faithfulness
//!
//! `IDFLookup` is Ghidra's **open-addressing** table with linear probing, an
//! `0xffffffff` empty marker, and a capacity that is the smallest power of two
//! `>= size`, doubled. That layout is observable: `toArray`/`set` iterate the
//! probe table in slot order, so a different table shape would serialize and
//! restore different orderings. The probe is reproduced slot for slot.
//!
//! `WeightFactory` stores `idfweight[512]` and `tfweight[64]`, applies
//! `scale`/`addend`, and pre-multiplies the four probability penalties by
//! `scale` so the scorer can use them directly. All arithmetic is `f64` exactly
//! as Ghidra's `double`.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::xml::{DocumentStorage, Element};

/// The empty-slot marker in Ghidra's probe table (`0xffffffff` as `int`).
const IDF_EMPTY: i32 = -1;

/// Feature weighting tables, loaded from a `lshweights_*.xml`.
///
/// Field-for-field the Java class: the two weight tables are fixed-size, and the
/// four `*_norm` values are the probability penalties already multiplied by
/// `scale`, which is what the significance formula consumes.
#[derive(Clone, Debug)]
pub struct WeightFactory {
    /// Weights by normalised IDF position (512 entries).
    idf_weight: Vec<f64>,
    /// Weights by term frequency (64 entries).
    tf_weight: Vec<f64>,
    /// Scale to which IDF weights are normalised.
    weight_norm: f64,
    /// Hash-flip probability, parameter 0.
    prob_flip0: f64,
    /// Hash-flip probability, parameter 1.
    prob_flip1: f64,
    /// Hash addition/removal probability, parameter 0.
    prob_diff0: f64,
    /// Hash addition/removal probability, parameter 1.
    prob_diff1: f64,
    /// Final scaling of all weights.
    scale: f64,
    /// Final correction to a score.
    addend: f64,
    /// `prob_flip0 * scale`, precomputed as Ghidra does.
    prob_flip0_norm: f64,
    /// `prob_flip1 * scale`.
    prob_flip1_norm: f64,
    /// `prob_diff0 * scale`.
    prob_diff0_norm: f64,
    /// `prob_diff1 * scale`.
    prob_diff1_norm: f64,
}

/// The number of entries in the IDF weight table (`idfweight.length`).
pub const IDF_WEIGHT_SIZE: usize = 512;
/// The number of entries in the TF weight table (`tfweight.length`).
pub const TF_WEIGHT_SIZE: usize = 64;

impl Default for WeightFactory {
    fn default() -> Self {
        WeightFactory {
            idf_weight: vec![0.0; IDF_WEIGHT_SIZE],
            tf_weight: vec![0.0; TF_WEIGHT_SIZE],
            weight_norm: 0.0,
            prob_flip0: 0.0,
            prob_flip1: 0.0,
            prob_diff0: 0.0,
            prob_diff1: 0.0,
            scale: 1.0,
            addend: 0.0,
            prob_flip0_norm: 0.0,
            prob_flip1_norm: 0.0,
            prob_diff0_norm: 0.0,
            prob_diff1_norm: 0.0,
        }
    }
}

impl WeightFactory {
    /// The coefficient for a feature: `idfweight[i] * tfweight[t]`
    /// (`WeightFactory.getCoeff`).
    pub fn get_coeff(&self, i: short_t, t: short_t) -> f64 {
        let i = (i as usize).min(self.idf_weight.len() - 1);
        let t = (t as usize).min(self.tf_weight.len() - 1);
        self.idf_weight[i] * self.tf_weight[t]
    }

    /// `getFlipNorm0`.
    pub fn flip_norm0(&self) -> f64 {
        self.prob_flip0_norm
    }
    /// `getDiffNorm0`.
    pub fn diff_norm0(&self) -> f64 {
        self.prob_diff0_norm
    }
    /// `getFlipNorm1`.
    pub fn flip_norm1(&self) -> f64 {
        self.prob_flip1_norm
    }
    /// `getDiffNorm1`.
    pub fn diff_norm1(&self) -> f64 {
        self.prob_diff1_norm
    }
    /// `getScale`.
    pub fn scale(&self) -> f64 {
        self.scale
    }
    /// `getAddend`.
    pub fn addend(&self) -> f64 {
        self.addend
    }
    /// `getWeightNorm`.
    pub fn weight_norm(&self) -> f64 {
        self.weight_norm
    }

    /// Recompute the four `*_norm` fields from `scale` (`updateNorms`).
    fn update_norms(&mut self) {
        self.prob_flip0_norm = self.prob_flip0 * self.scale;
        self.prob_flip1_norm = self.prob_flip1 * self.scale;
        self.prob_diff0_norm = self.prob_diff0 * self.scale;
        self.prob_diff1_norm = self.prob_diff1 * self.scale;
    }

    /// A factory whose coefficients are the raw term-frequency index and whose
    /// penalty terms are zero — the arithmetic baseline unit tests compare
    /// against. Test-only; a real run loads Ghidra's XML instead.
    #[doc(hidden)]
    pub fn with_unit_weights_for_tests() -> Self {
        let mut out = WeightFactory::default();
        for (i, slot) in out.idf_weight.iter_mut().enumerate() {
            *slot = (i + 1) as f64;
        }
        for (i, slot) in out.tf_weight.iter_mut().enumerate() {
            *slot = (i + 1) as f64;
        }
        out.update_norms();
        out
    }

    /// Force the four penalty parameters and the addend, then recompute the
    /// pre-scaled norms. Test-only.
    #[doc(hidden)]
    pub fn set_penalties_for_tests(
        &mut self,
        flip0: f64,
        flip1: f64,
        diff0: f64,
        diff1: f64,
        addend: f64,
    ) {
        self.prob_flip0 = flip0;
        self.prob_flip1 = flip1;
        self.prob_diff0 = diff0;
        self.prob_diff1 = diff1;
        self.addend = addend;
        self.update_norms();
    }

    /// Parse the `<weightfactory>` element (`WeightFactory.restoreXml`).
    pub fn restore_xml(el: &Element) -> KunaResult<Self> {
        let mut out = WeightFactory::default();
        out.scale = parse_attr_f64(el, "scale")?;
        out.addend = parse_attr_f64(el, "addend")?;
        let scale_sqrt = out.scale.sqrt();
        let mut idf = Vec::with_capacity(IDF_WEIGHT_SIZE);
        let mut tf = Vec::with_capacity(TF_WEIGHT_SIZE);
        for child in el.get_children() {
            let text = std::str::from_utf8(child.get_content())
                .map_err(|_| KunaError::bad_data("lsh weights: non-UTF-8 weight value"))?;
            let value: f64 = text
                .trim()
                .parse()
                .map_err(|_| KunaError::bad_data(format!("lsh weights: bad number {text:?}")))?;
            match child.get_name() {
                // Ghidra stores the idf values pre-divided by sqrt(scale).
                "idf" => idf.push(value * scale_sqrt),
                "tf" => tf.push(value),
                "weightnorm" => {
                    out.weight_norm =
                        value / out.scale;
                }
                "probflip0" => out.prob_flip0 = value,
                "probflip1" => out.prob_flip1 = value,
                "probdiff0" => out.prob_diff0 = value,
                "probdiff1" => out.prob_diff1 = value,
                _ => {}
            }
        }
        if idf.len() != IDF_WEIGHT_SIZE {
            return Err(KunaError::bad_data(format!(
                "lsh weights: {IDF_WEIGHT_SIZE} <idf> entries expected, found {}",
                idf.len()
            )));
        }
        if tf.len() != TF_WEIGHT_SIZE {
            return Err(KunaError::bad_data(format!(
                "lsh weights: {TF_WEIGHT_SIZE} <tf> entries expected, found {}",
                tf.len()
            )));
        }
        out.idf_weight = idf;
        out.tf_weight = tf;
        out.update_norms();
        Ok(out)
    }
}

/// A `short`-width index into the weight tables (`short` in Java).
pub type short_t = i16;

/// Parse a required `f64` attribute.
fn parse_attr_f64(el: &Element, name: &str) -> KunaResult<f64> {
    let raw = el.get_attribute_value(name)?;
    let text = std::str::from_utf8(raw)
        .map_err(|_| KunaError::bad_data(format!("lsh weights: non-UTF-8 {name}")))?;
    text.trim()
        .parse()
        .map_err(|_| KunaError::bad_data(format!("lsh weights: bad {name} {text:?}")))
}

/// Inverse-document-frequency lookup: hash → how many corpus functions carry it.
///
/// Ghidra's table is open addressing with linear probing over a power-of-two
/// slot count; the slot order is what `toArray` serializes, so it is reproduced
/// rather than replaced by an ordered map.
#[derive(Clone, Debug, Default)]
pub struct IDFLookup {
    /// Occupied entries in slot order (`None` is an empty slot).
    slots: Vec<Option<(i32, i32)>>,
    /// The probe mask (`mask` in Java; `slots.len() - 1`).
    mask: usize,
    /// The declared entry count.
    size: usize,
}

impl IDFLookup {
    /// True when no table was loaded (`empty()`).
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    /// The capacity (`getCapacity`, the Java mask).
    pub fn capacity(&self) -> usize {
        self.mask
    }

    /// The corpus frequency of `hash`, or 0 when absent (`getCount`).
    pub fn get_count(&self, hash: i32) -> i32 {
        if self.mask == 0 && self.slots.is_empty() {
            return 0;
        }
        let mut val = (hash as usize) & self.mask;
        loop {
            match self.slots[val] {
                None => return 0,
                Some((h, c)) if h == hash => return c,
                Some(_) => val = (val + 1) & self.mask,
            }
        }
    }

    /// Grow to a table of `size` entries (`initializeTable`).
    fn initialize_table(&mut self, size: usize) {
        let mut mask = 1usize;
        while mask < size {
            mask <<= 1;
        }
        mask <<= 1;
        self.slots = vec![None; mask];
        self.mask = mask - 1;
        self.size = size;
    }

    fn insert_hash(&mut self, hash: i32, count: i32) {
        let mut val = (hash as usize) & self.mask;
        while self.slots[val].is_some() {
            val = (val + 1) & self.mask;
        }
        self.slots[val] = Some((hash, count));
    }

    /// Parse the `<idflookup>` element (`IDFLookup.restoreXml`).
    pub fn restore_xml(el: &Element) -> KunaResult<Self> {
        let mut out = IDFLookup::default();
        // No `size` attribute means an empty table.
        let Ok(raw_size) = el.get_attribute_value("size") else {
            return Ok(out);
        };
        let size_text = std::str::from_utf8(raw_size)
            .map_err(|_| KunaError::bad_data("lsh idflookup: non-UTF-8 size"))?;
        let size: usize = size_text
            .trim()
            .parse()
            .map_err(|_| KunaError::bad_data("lsh idflookup: bad size"))?;
        if size == 0 {
            return Ok(out);
        }
        out.initialize_table(size);
        for child in el.get_children() {
            if child.get_name() != "hash" {
                continue;
            }
            let count_raw = child.get_attribute_value("count")?;
            let count_text = std::str::from_utf8(count_raw)
                .map_err(|_| KunaError::bad_data("lsh idflookup: non-UTF-8 count"))?;
            let count: i32 = count_text
                .trim()
                .parse()
                .map_err(|_| KunaError::bad_data("lsh idflookup: bad count"))?;
            let text = std::str::from_utf8(child.get_content())
                .map_err(|_| KunaError::bad_data("lsh idflookup: non-UTF-8 hash"))?;
            let hash = parse_java_hex(text.trim())?;
            out.insert_hash(hash, count);
        }
        Ok(out)
    }

    /// Build from `(hash, count)` pairs (`IDFLookup.set`).
    pub fn from_pairs(pairs: &[(i32, i32)]) -> Self {
        let mut out = IDFLookup::default();
        if pairs.is_empty() {
            return out;
        }
        out.initialize_table(pairs.len());
        for &(hash, count) in pairs {
            out.insert_hash(hash, count);
        }
        out
    }

    /// Collapse to `(hash, count)` pairs in slot order (`IDFLookup.toArray`).
    pub fn to_pairs(&self) -> Vec<(i32, i32)> {
        self.slots.iter().flatten().copied().collect()
    }

    /// The declared entry count.
    pub fn declared_size(&self) -> usize {
        self.size
    }
}

/// Parse a signed 32-bit value written as `0x…` (Java's `decodeInt` accepts the
/// hex spelling, and the weights files use it).
fn parse_java_hex(text: &str) -> KunaResult<i32> {
    let t = text.trim();
    let (neg, body) = match t.strip_prefix('-') {
        Some(rest) => (true, rest.trim()),
        None => (false, t),
    };
    let value = if let Some(hex) = body.strip_prefix("0x").or_else(|| body.strip_prefix("0X")) {
        u32::from_str_radix(hex, 16)
            .map_err(|_| KunaError::bad_data(format!("lsh: bad hex number {t:?}")))?
    } else {
        body.parse::<u32>()
            .map_err(|_| KunaError::bad_data(format!("lsh: bad number {t:?}")))?
    };
    let value = value as i32;
    Ok(if neg { value.wrapping_neg() } else { value })
}

/// A parsed `lshweights_*.xml`: the settings id plus both tables.
#[derive(Clone, Debug)]
pub struct LshWeights {
    /// The weighting-scheme id (`settings` on `<weights>`).
    pub settings: i32,
    /// The feature weight tables.
    pub factory: Rc<WeightFactory>,
    /// The corpus frequency lookup.
    pub idf: Rc<IDFLookup>,
}

impl LshWeights {
    /// Parse a weights document (`LSHVectorFactory.readWeights`).
    ///
    /// Requires both `<weightfactory>` and `<idflookup>`; their absence is an
    /// error in Ghidra and here.
    pub fn parse(bytes: &[u8]) -> KunaResult<Self> {
        let mut storage = DocumentStorage::new();
        let doc = storage.parse_document(bytes)?;
        let root = doc.get_root();
        if root.get_name() != "weights" {
            return Err(KunaError::bad_data(format!(
                "lsh weights: root element is {:?}, expected \"weights\"",
                root.get_name()
            )));
        }
        let settings_raw = root.get_attribute_value("settings")?;
        let settings_text = std::str::from_utf8(settings_raw)
            .map_err(|_| KunaError::bad_data("lsh weights: non-UTF-8 settings"))?;
        let settings = parse_java_hex(settings_text)?;

        let mut factory = None;
        let mut idf = None;
        for child in root.get_children() {
            match child.get_name() {
                "weightfactory" => factory = Some(WeightFactory::restore_xml(child)?),
                "idflookup" => idf = Some(IDFLookup::restore_xml(child)?),
                _ => {}
            }
        }
        let factory = factory.ok_or_else(|| {
            KunaError::bad_data("lsh weights: no <weightfactory> element")
        })?;
        let idf =
            idf.ok_or_else(|| KunaError::bad_data("lsh weights: no <idflookup> element"))?;
        Ok(LshWeights { settings, factory: Rc::new(factory), idf: Rc::new(idf) })
    }
}

/// The processor-size weights file name (`GenSignatures.getWeightsFile`).
///
/// Two languages compare with the size-symmetric file when their address sizes
/// differ; otherwise each size has its own table. A language id is
/// `processor:endian:size:variant`, so the third field is the address size.
pub fn weights_file_for(language1: &str, language2: &str) -> Option<&'static str> {
    let split1: Vec<&str> = language1.split(':').collect();
    let split2: Vec<&str> = language2.split(':').collect();
    if split1.len() < 3 || split2.len() < 3 {
        return None;
    }
    if split1[0] == "Dalvik" || split1[0] == "JVM" {
        return (split2[0] == split1[0]).then_some("lshweights_cpool.xml");
    }
    let size1 = split1[2];
    let size2 = split2[2];
    if size1 != size2 {
        if !matches!(size1, "64" | "32") || !matches!(size2, "64" | "32") {
            return None;
        }
        return Some("lshweights_nosize.xml");
    }
    match size1 {
        "32" => Some("lshweights_32.xml"),
        "64" => {
            // A 64-bit language whose variant names a 32-bit ABI (x32) compares
            // against the 32-bit weights.
            if split1.len() > 3 && split1[3].contains("-32") {
                Some("lshweights_64_32.xml")
            } else {
                Some("lshweights_64.xml")
            }
        }
        _ => Some("lshweights_nosize.xml"),
    }
}

/// The corpus frequency table as an ordered map, for callers that want to embed
/// a fixed table rather than ship the XML.
pub fn idf_as_map(idf: &IDFLookup) -> BTreeMap<i32, i32> {
    idf.to_pairs().into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A complete document: 512 `<idf>`, 64 `<tf>`, and a two-entry lookup.
    fn mini() -> String {
        let idf: String = (0..IDF_WEIGHT_SIZE)
            .map(|i| format!("<idf>{}</idf>\n", 0.5 + i as f64))
            .collect();
        let tf: String =
            (0..TF_WEIGHT_SIZE).map(|i| format!("<tf>{}</tf>\n", 1.0 + i as f64)).collect();
        format!(
            "<weights settings=\"0x49\">\n\
             <weightfactory scale=\"2\" addend=\"1\">\n\
             {idf}{tf}</weightfactory>\n\
             <idflookup size=\"4\">\n\
             <hash count=\"7\">0x10</hash>\n\
             <hash count=\"9\">0x20</hash>\n\
             </idflookup>\n</weights>\n"
        )
    }

    #[test]
    fn a_short_idf_table_is_refused() {
        // The size check is the contract: Ghidra's `restoreXml` reads exactly
        // `idfweight.length` entries, so a short table must not load silently.
        let short = "<weights settings=\"0x49\"><weightfactory scale=\"2\" addend=\"1\">\
                     <idf>0.5</idf></weightfactory><idflookup/></weights>";
        let err = LshWeights::parse(short.as_bytes()).expect_err("short table refused");
        assert!(err.explain().contains("512"), "{}", err.explain());
    }

    #[test]
    fn a_complete_document_loads_both_tables() {
        let w = LshWeights::parse(mini().as_bytes()).expect("parse");
        assert_eq!(w.settings, 0x49);
        assert_eq!(w.idf.get_count(0x10), 7);
        assert_eq!(w.idf.get_count(0x20), 9);
        assert_eq!(w.idf.get_count(0x99), 0);
        assert!(!w.idf.is_empty());
        // Ghidra saves each <idf> as `weight / sqrt(scale)` and multiplies by
        // `sqrt(scale)` on load, so the stored 0.5 restores as 0.5*sqrt(2).
        let sqrt2 = 2.0f64.sqrt();
        assert!((w.factory.get_coeff(0, 0) - 0.5 * sqrt2 * 1.0).abs() < 1e-12);
    }

    #[test]
    fn the_table_is_open_addressed_with_linear_probing() {
        // Capacity is the smallest power of two >= size, doubled, then masked.
        let look = IDFLookup::from_pairs(&[(1, 5), (2, 6), (3, 7)]);
        assert_eq!(look.declared_size(), 3);
        assert_eq!(look.capacity() + 1, look.slots.len());
        assert_eq!(look.get_count(1), 5);
        assert_eq!(look.get_count(2), 6);
        assert_eq!(look.get_count(3), 7);
        assert_eq!(look.get_count(4), 0);
        // Round-tripping through the slot array preserves slot order.
        let pairs = look.to_pairs();
        assert_eq!(pairs.len(), 3);
        let back = IDFLookup::from_pairs(&pairs);
        assert_eq!(back.get_count(1), 5);
    }

    #[test]
    fn an_empty_table_answers_zero_and_round_trips() {
        let empty = IDFLookup::default();
        assert!(empty.is_empty());
        assert_eq!(empty.get_count(1234), 0);
        assert!(empty.to_pairs().is_empty());
        assert!(IDFLookup::from_pairs(&[]).is_empty());
    }

    #[test]
    fn a_missing_size_attribute_is_an_empty_table() {
        let mut storage = DocumentStorage::new();
        let doc = storage
            .parse_document(b"<idflookup/>".as_slice())
            .expect("parse");
        let root = doc.get_root();
        let look = IDFLookup::restore_xml(root).expect("restore");
        assert!(look.is_empty());
    }

    #[test]
    fn the_weights_file_follows_the_address_size() {
        assert_eq!(
            weights_file_for("x86:LE:32:default", "x86:LE:32:default"),
            Some("lshweights_32.xml")
        );
        assert_eq!(
            weights_file_for("x86:LE:64:default", "x86:LE:64:default"),
            Some("lshweights_64.xml")
        );
        // Differing sizes fall back to the size-symmetric table.
        assert_eq!(
            weights_file_for("x86:LE:64:default", "x86:LE:32:default"),
            Some("lshweights_nosize.xml")
        );
        // The x32 variant compares against 32-bit weights.
        assert_eq!(
            weights_file_for("x86:LE:64:x86-32", "x86:LE:64:default"),
            Some("lshweights_64_32.xml")
        );
        // An unparsable id has no table.
        assert_eq!(weights_file_for("x86:LE", "x86:LE"), None);
        assert_eq!(weights_file_for("JVM:BE:32:default", "JVM:BE:32:default"), Some("lshweights_cpool.xml"));
        assert_eq!(weights_file_for("JVM:BE:32:default", "Dalvik:LE:32:default"), None);
    }

    #[test]
    fn hex_and_decimal_spellings_both_parse() {
        assert_eq!(parse_java_hex("0x10").unwrap(), 16);
        assert_eq!(parse_java_hex("16").unwrap(), 16);
        assert_eq!(parse_java_hex("-0x10").unwrap(), -16);
        // A hash above i32::MAX keeps its bit pattern.
        assert_eq!(parse_java_hex("0xffffffff").unwrap(), -1);
        assert!(parse_java_hex("nope").is_err());
    }
}

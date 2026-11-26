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

/// Generous ceiling relative to the stock 1,000-entry tables.
const MAX_IDF_HASHES: usize = 1 << 16;

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
    pub fn get_coeff(&self, i: i16, t: i16) -> f64 {
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
        if !out.scale.is_finite() || out.scale <= 0.0 || !out.addend.is_finite() {
            return Err(KunaError::bad_data("lsh weights: scale must be positive and addend finite"));
        }
        let scale_sqrt = out.scale.sqrt();
        let (mut idf, mut tf) = (0, 0);
        let mut seen = 0u8;
        for child in el.get_children() {
            let name = child.get_name();
            let text = std::str::from_utf8(child.get_content())
                .map_err(|_| KunaError::bad_data("lsh weights: non-UTF-8 weight value"))?;
            let value: f64 = text
                .trim()
                .parse()
                .map_err(|_| KunaError::bad_data(format!("lsh weights: bad number {text:?}")))?;
            if !value.is_finite() || value < 0.0 {
                return Err(KunaError::bad_data(format!("lsh weights: invalid <{name}> value")));
            }
            let (slot, bit, value) = match name {
                // Ghidra stores the idf values pre-divided by sqrt(scale).
                "idf" if idf < IDF_WEIGHT_SIZE => {
                    let slot = &mut out.idf_weight[idf];
                    idf += 1;
                    (slot, 0, value * scale_sqrt)
                }
                "tf" if tf < TF_WEIGHT_SIZE => {
                    let slot = &mut out.tf_weight[tf];
                    tf += 1;
                    (slot, 0, value)
                }
                "idf" | "tf" => return Err(KunaError::bad_data(format!("lsh weights: too many <{name}> entries"))),
                "weightnorm" => (&mut out.weight_norm, 1, value / out.scale),
                "probflip0" => (&mut out.prob_flip0, 2, value),
                "probflip1" => (&mut out.prob_flip1, 4, value),
                "probdiff0" => (&mut out.prob_diff0, 8, value),
                "probdiff1" => (&mut out.prob_diff1, 16, value),
                _ => continue,
            };
            if seen & bit != 0 {
                return Err(KunaError::bad_data(format!("lsh weights: duplicate <{name}>")));
            }
            if !value.is_finite() || (bit > 1 && !(value * out.scale).is_finite()) {
                return Err(KunaError::bad_data(format!("lsh weights: <{name}> overflows after scaling")));
            }
            seen |= bit;
            *slot = value;
        }
        if idf != IDF_WEIGHT_SIZE || tf != TF_WEIGHT_SIZE {
            return Err(KunaError::bad_data(format!(
                "lsh weights: expected {IDF_WEIGHT_SIZE} <idf> and {TF_WEIGHT_SIZE} <tf> entries, found {idf} and {tf}"
            )));
        }
        for (index, name) in ["weightnorm", "probflip0", "probflip1", "probdiff0", "probdiff1"].into_iter().enumerate() {
            if seen & (1 << index) == 0 {
                return Err(KunaError::bad_data(format!("lsh weights: missing <{name}>")));
            }
        }
        let max_idf = out.idf_weight.iter().copied().fold(0.0, f64::max);
        let max_tf = out.tf_weight.iter().copied().fold(0.0, f64::max);
        if (max_idf * max_tf).powi(2) > f64::MAX / f64::from(u32::MAX) {
            return Err(KunaError::bad_data("lsh weights: feature coefficients overflow the vector norm"));
        }
        out.update_norms();
        Ok(out)
    }
}

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
    fn initialize_table(&mut self, size: usize) -> KunaResult<()> {
        let capacity = size
            .checked_next_power_of_two()
            .and_then(|n| n.checked_mul(2))
            .filter(|_| size <= MAX_IDF_HASHES)
            .ok_or_else(|| KunaError::bad_data("lsh idflookup: capacity exceeds the limit"))?;
        self.slots = vec![None; capacity];
        self.mask = capacity - 1;
        self.size = size;
        Ok(())
    }

    fn insert_hash(&mut self, hash: i32, count: i32) -> KunaResult<()> {
        if hash == -1 || count < 0 {
            return Err(KunaError::bad_data("lsh idflookup: invalid hash or negative count"));
        }
        let mut val = (hash as usize) & self.mask;
        for _ in 0..self.slots.len() {
            match self.slots[val] {
                None => {
                    self.slots[val] = Some((hash, count));
                    return Ok(());
                }
                Some((existing, _)) if existing == hash => {
                    return Err(KunaError::bad_data(format!("lsh idflookup: duplicate hash {hash:#x}")));
                }
                Some(_) => val = (val + 1) & self.mask,
            }
        }
        Err(KunaError::bad_data("lsh idflookup: no free slot"))
    }

    /// Parse the `<idflookup>` element (`IDFLookup.restoreXml`).
    pub fn restore_xml(el: &Element) -> KunaResult<Self> {
        let mut out = IDFLookup::default();
        let size = match el.get_attribute_value("size") {
            Ok(raw) => std::str::from_utf8(raw)
                .ok()
                .and_then(|text| text.trim().parse::<usize>().ok())
                .ok_or_else(|| KunaError::bad_data("lsh idflookup: invalid size"))?,
            Err(_) => 0,
        };
        let children = el.get_children();
        if size != children.len() || size > MAX_IDF_HASHES {
            return Err(KunaError::bad_data(format!(
                "lsh idflookup: declared size {size}, found {} entries (maximum {MAX_IDF_HASHES})",
                children.len(),
            )));
        }
        if size == 0 {
            return Ok(out);
        }
        out.initialize_table(size)?;
        for child in children {
            if child.get_name() != "hash" {
                return Err(KunaError::bad_data("lsh idflookup: expected <hash>"));
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
            out.insert_hash(hash, count)?;
        }
        Ok(out)
    }

    /// Build from `(hash, count)` pairs (`IDFLookup.set`).
    pub fn from_pairs(pairs: &[(i32, i32)]) -> KunaResult<Self> {
        let mut out = IDFLookup::default();
        if !pairs.is_empty() {
            out.initialize_table(pairs.len())?;
            for &(hash, count) in pairs {
                out.insert_hash(hash, count)?;
            }
        }
        Ok(out)
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
                "weightfactory" if factory.is_none() => factory = Some(WeightFactory::restore_xml(child)?),
                "idflookup" if idf.is_none() => idf = Some(IDFLookup::restore_xml(child)?),
                "weightfactory" | "idflookup" => {
                    return Err(KunaError::bad_data(format!("lsh weights: duplicate <{}>", child.get_name())));
                }
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

/// A single scoring profile pinned when a local corpus is created.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LshProfile {
    Size32,
    Size64,
    Size64With32Abi,
    NoSize,
    Jvm,
    Dalvik,
}

impl LshProfile {
    /// The stable corpus profile name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Size32 => "32",
            Self::Size64 => "64",
            Self::Size64With32Abi => "64_32",
            Self::NoSize => "nosize",
            Self::Jvm => "jvm",
            Self::Dalvik => "dalvik",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "32" => Some(Self::Size32),
            "64" => Some(Self::Size64),
            "64_32" => Some(Self::Size64With32Abi),
            "nosize" => Some(Self::NoSize),
            "jvm" => Some(Self::Jvm),
            "dalvik" => Some(Self::Dalvik),
            _ => None,
        }
    }

    pub fn weights_file(self) -> &'static str {
        match self {
            Self::Size32 => "lshweights_32.xml",
            Self::Size64 => "lshweights_64.xml",
            Self::Size64With32Abi => "lshweights_64_32.xml",
            Self::NoSize => "lshweights_nosize.xml",
            Self::Jvm | Self::Dalvik => "lshweights_cpool.xml",
        }
    }

    /// Load the stock Ghidra 12.1.2 weight data embedded in Kuna.
    pub fn bundled_weights(self) -> KunaResult<LshWeights> {
        let xml: &[u8] = match self {
            Self::Size32 => include_bytes!("weights/lshweights_32.xml"),
            Self::Size64 => include_bytes!("weights/lshweights_64.xml"),
            Self::Size64With32Abi => include_bytes!("weights/lshweights_64_32.xml"),
            Self::NoSize => include_bytes!("weights/lshweights_nosize.xml"),
            Self::Jvm | Self::Dalvik => include_bytes!("weights/lshweights_cpool.xml"),
        };
        LshWeights::parse(xml)
    }

    /// Default profile for a full Ghidra language ID.
    pub fn for_language(language: &str) -> Option<Self> {
        let mut fields = language.split(':');
        let processor = fields.next()?;
        let endian = fields.next()?;
        let size = fields.next()?.parse::<u32>().ok()?;
        let variant = fields.next()?;
        if processor.is_empty() || endian.is_empty() || size == 0 || variant.is_empty() || fields.next().is_some() {
            return None;
        }
        Some(match (processor, size) {
            ("JVM", _) => Self::Jvm,
            ("Dalvik", _) => Self::Dalvik,
            (_, 32) => Self::Size32,
            (_, 64) if variant.contains("-32") => Self::Size64With32Abi,
            (_, 64) => Self::Size64,
            _ => Self::NoSize,
        })
    }

    /// A size-independent native corpus can mix sizes and processors, but
    /// constant-pool languages have separate profiles.
    pub fn admits(self, language: &str) -> bool {
        match Self::for_language(language) {
            Some(default) if self == Self::NoSize => !matches!(default, Self::Jvm | Self::Dalvik),
            Some(default) => default == self,
            None => false,
        }
    }

    pub fn admits_settings(self, settings: u32) -> bool {
        use kuna_decomp::signature::sig_mods::SIG_COLLAPSE_SIZE;
        let collapses_size = (settings >> 2) & SIG_COLLAPSE_SIZE != 0;
        super::signature::settings_are_valid(settings) && collapses_size == (self == Self::NoSize)
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
             <weightnorm>2</weightnorm><probflip0>1</probflip0><probflip1>1</probflip1>\
             <probdiff0>1</probdiff0><probdiff1>1</probdiff1>{idf}{tf}</weightfactory>\n\
             <idflookup size=\"2\">\n\
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
    fn loaded_penalties_match_an_independent_significance_calculation() {
        use super::super::lsh::{score_candidate, LshCosineVector};

        let w = LshWeights::parse(mini().as_bytes()).unwrap();
        let a = LshCosineVector::from_features(&[1, 2], &w.factory, &w.idf);
        let b = LshCosineVector::from_features(&[1, 3, 4], &w.factory, &w.idf);
        let (similarity, significance) = score_candidate(&w.factory, &a, &b, 0.0).unwrap();
        // Equal coefficients; one shared hash, one flip, and one size difference.
        assert!((similarity - 1.0 / 6.0f64.sqrt()).abs() < 1e-12);
        let expected = 0.5 - 2.0 * (2.0 + 2.0 / 3.0) + 1.0;
        assert!((significance - expected).abs() < 1e-12);
    }

    #[test]
    fn the_table_is_open_addressed_with_linear_probing() {
        // Capacity is the smallest power of two >= size, doubled, then masked.
        let look = IDFLookup::from_pairs(&[(1, 5), (2, 6), (3, 7)]).unwrap();
        assert_eq!(look.declared_size(), 3);
        assert_eq!(look.capacity() + 1, look.slots.len());
        assert_eq!(look.get_count(1), 5);
        assert_eq!(look.get_count(2), 6);
        assert_eq!(look.get_count(3), 7);
        assert_eq!(look.get_count(4), 0);
        // Round-tripping through the slot array preserves slot order.
        let pairs = look.to_pairs();
        assert_eq!(pairs.len(), 3);
        let back = IDFLookup::from_pairs(&pairs).unwrap();
        assert_eq!(back.get_count(1), 5);
    }

    #[test]
    fn an_empty_table_answers_zero_and_round_trips() {
        let empty = IDFLookup::default();
        assert!(empty.is_empty());
        assert_eq!(empty.get_count(1234), 0);
        assert!(empty.to_pairs().is_empty());
        assert!(IDFLookup::from_pairs(&[]).unwrap().is_empty());
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
    fn a_profile_is_shared_by_every_input_or_refuses_it() {
        let x64 = "x86:LE:64:default";
        let x32 = "x86:LE:32:default";
        assert_eq!(LshProfile::for_language(x64), Some(LshProfile::Size64));
        assert_eq!(LshProfile::for_language("x86:LE:64:x86-32"), Some(LshProfile::Size64With32Abi));
        assert!(LshProfile::Size64.admits(x64));
        assert!(LshProfile::Size64.admits("AARCH64:LE:64:v8A"));
        assert!(!LshProfile::Size64.admits(x32));
        assert!(LshProfile::NoSize.admits(x64));
        assert!(LshProfile::NoSize.admits(x32));
        assert!(LshProfile::NoSize.admits("z80:LE:16:default"));
        assert!(!LshProfile::NoSize.admits("JVM:BE:32:default"));
        assert!(!LshProfile::Jvm.admits("Dalvik:LE:32:default"));
        assert!(!LshProfile::NoSize.admits("x86:LE"));
        assert!(LshProfile::NoSize.admits_settings(0x4d));
        assert!(!LshProfile::NoSize.admits_settings(0x49));
        assert!(!LshProfile::Size64.admits_settings(0x4d));
    }

    #[test]
    fn each_bundled_profile_loads_its_stock_weights() {
        for (profile, settings, hash) in [
            (LshProfile::Size32, 0x49, 0x0a12_0351),
            (LshProfile::Size64, 0x49, 0x1523_1688),
            (LshProfile::Size64With32Abi, 0x49, 0x0e97_eee9),
            (LshProfile::NoSize, 0x4d, 0x2611_1c79),
            (LshProfile::Jvm, 0x49, 0x056b_3354),
            (LshProfile::Dalvik, 0x49, 0x056b_3354),
        ] {
            let w = profile.bundled_weights().unwrap();
            assert_eq!(w.settings, settings, "{profile:?}");
            assert!(profile.admits_settings(settings as u32), "{profile:?}");
            assert_eq!(w.idf.declared_size(), 1000, "{profile:?}");
            assert_eq!(w.idf.get_count(hash), 1, "{profile:?}");
        }
    }

    #[test]
    fn malformed_weight_documents_are_rejected() {
        for (from, to, reason) in [
            ("<probdiff0>1</probdiff0>", "", "missing <probdiff0>"),
            ("<probdiff0>1</probdiff0>", "<probdiff0>NaN</probdiff0>", "invalid <probdiff0>"),
            ("<probflip0>1</probflip0>", "<probflip0>-1</probflip0>", "invalid <probflip0>"),
            ("<probflip0>1</probflip0>", "<probflip0>1e308</probflip0>", "overflows"),
            ("<probflip1>1</probflip1>", "<probflip0>1</probflip0>", "duplicate <probflip0>"),
            ("scale=\"2\"", "scale=\"0\"", "scale must be positive"),
            ("size=\"2\"", "size=\"1\"", "declared size"),
            ("size=\"2\"", "size=\"1000000000\"", "declared size"),
            (" size=\"2\"", "", "declared size"),
            ("0x20</hash>", "0x10</hash>", "duplicate hash"),
            ("count=\"9\"", "count=\"-1\"", "negative count"),
        ] {
            let xml = mini().replace(from, to);
            let err = LshWeights::parse(xml.as_bytes()).expect_err(reason);
            assert!(err.explain().contains(reason), "{}", err.explain());
        }
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

//! Generating a BSim signature for a decompiled function.
//!
//! A BSim record is what Ghidra's `GenSignatures.SignatureTask` produces for one
//! function: the decompiler's feature list (32-bit hashes), the flags that say the
//! body has unimplemented instructions or ran into bad data, and the function's
//! identity. The feature list itself comes from the decompiler's signature module,
//! already ported as [`kuna_decomp::signature`] — this module is the thin seam
//! that turns a [`Funcdata`] into a record and reports why a function could not
//! be signed.
//!
//! # Origin (upstream Ghidra)
//!
//! - `Ghidra/Features/BSim/src/main/java/ghidra/features/bsim/query/GenSignatures.java`
//!   — `SignatureTask.decompile` decides what is signable: a thunk is skipped, a
//!   function without a body is skipped, and a result with **no features** is
//!   reported rather than stored.
//! - `Features/Decompiler/src/main/java/ghidra/app/decompiler/signature/SignatureResult.java`
//!   — the `features` / `hasunimplemented` / `hasbaddata` shape.
//!
//! # What "no body" means here
//!
//! Ghidra asks the *program* whether a function has a body (`hasBody`). kuna
//! reaches this module only after a function has been decompiled, so a function
//! with no body never arrives; the equivalent refusal is an empty feature list,
//! which this module reports as [`SignatureOutcome::NoFeatures`] instead of
//! storing an unmatchable record.

use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::signature::{get_settings, set_settings, sig_mods, GraphSigManager};

/// The decompiler's signature settings BSim uses.
///
/// Ghidra takes the settings from the database's vector factory
/// (`vectorFactory.getSettings()`), and the shipped `lshweights_*.xml` files all
/// declare `settings="0x49"`. The engine reads the mode bits from `settings >> 2`
/// (`GraphSigManager::sigmods`), so `0x49 >> 2 == 0x12`, which is
/// `SIG_COLLAPSE_INDNOISE(0x2) | SIG_DONOTUSE_CONST(0x10)` — **not**
/// `SIG_COLLAPSE_SIZE`, whose bit is clear. A run built against these weights
/// therefore collapses indirect-copy noise and ignores constant values, and
/// keeps size distinctions.
pub const BSIM_SIG_SETTINGS: u32 = 0x49;

/// One signed function, ready to be stored in a corpus.
#[derive(Clone, Debug, PartialEq)]
pub struct BsimSignature {
    /// The function's name, as the corpus records it.
    pub name: String,
    /// The function's entry address in its own program.
    pub address: u64,
    /// The decompiler's feature hashes, sorted ascending (as `u32`).
    pub features: Vec<u32>,
    /// The body contains unimplemented instructions.
    pub has_unimplemented: bool,
    /// The body's flow ran into bad data.
    pub has_bad_data: bool,
}

impl BsimSignature {
    /// A stable identity for this signature: name plus address. Two functions
    /// with the same name at different addresses are different records, exactly
    /// as BSim's `(executable, name, address)` triple keys them.
    pub fn key(&self) -> (&str, u64) {
        (&self.name, self.address)
    }

    /// The feature count. A signature with none is refused by
    /// [`sign_function`], so a stored record is never empty.
    pub fn num_features(&self) -> usize {
        self.features.len()
    }
}

/// Why a function produced no signature.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SignatureOutcome {
    /// The decompiler found no features (Ghidra's "No features in signature").
    /// The function is not signable and never enters the corpus.
    NoFeatures,
    /// The signature generator refused the function, e.g. it exceeds the varnode
    /// threshold or the settings are invalid. Carries the engine's reason.
    Refused(String),
}

impl SignatureOutcome {
    /// One sentence for a report line.
    pub fn describe(&self) -> String {
        match self {
            SignatureOutcome::NoFeatures => "the decompiler found no features".into(),
            SignatureOutcome::Refused(reason) => format!("the signature generator refused it ({reason})"),
        }
    }
}

/// Which of BSim's signature settings the run uses.
///
/// A corpus is only comparable within one settings value, so the value travels
/// with every record: two vectors built under different settings measure
/// different things.
pub fn settings_vector() -> u32 {
    BSIM_SIG_SETTINGS
}

/// Generate the signature for a decompiled function.
///
/// Returns the record, or the reason the function cannot be signed. `features` is
/// sorted ascending and de-duplicated only in the sense that repeats are kept —
/// a repeated hash is that feature's term frequency, which the vector model
/// consumes.
pub fn sign_function(fd: &Funcdata, name: &str, address: u64) -> Result<BsimSignature, SignatureOutcome> {
    // The settings are process-global and `GraphSigManager::new` reads them, so a
    // BSim run must establish its own before the first function. Ghidra does the
    // same through the vector factory (`newdecompiler.setSignatureSettings`).
    if get_settings() != BSIM_SIG_SETTINGS {
        set_settings(BSIM_SIG_SETTINGS);
    }
    let mut manager =
        GraphSigManager::new().map_err(|e| SignatureOutcome::Refused(e.explain().to_string()))?;
    manager
        .set_current_function(fd)
        .map_err(|e| SignatureOutcome::Refused(e.explain().to_string()))?;
    manager.generate(fd);
    let features = manager.get_signature_vector();
    if features.is_empty() {
        return Err(SignatureOutcome::NoFeatures);
    }
    Ok(BsimSignature {
        name: name.to_string(),
        address,
        features,
        has_unimplemented: fd.has_unimplemented(),
        has_bad_data: fd.has_bad_data(),
    })
}

/// The engine setting in the form [`GraphSigManager::test_settings`] accepts: the
/// mode bits shifted over the check bit, which is what `set_settings` wants.
///
/// Ghidra's database stores the whole `settings` integer (`0x49`); the engine's
/// manager reads the mode bits from `settings >> 2` and validates the rest.
pub fn engine_settings_value() -> u32 {
    settings_vector()
}

/// True when `settings` is a value the engine accepts.
pub fn settings_are_valid(settings: u32) -> bool {
    GraphSigManager::test_settings(settings)
}

/// The signature-mode bits a BSim run asks for, named for a report.
pub fn signature_modes() -> Vec<&'static str> {
    let modes = BSIM_SIG_SETTINGS >> 2;
    let mut out = Vec::new();
    if modes & sig_mods::SIG_COLLAPSE_SIZE != 0 {
        out.push("collapse-size");
    }
    if modes & sig_mods::SIG_COLLAPSE_INDNOISE != 0 {
        out.push("collapse-indnoise");
    }
    if modes & sig_mods::SIG_DONOTUSE_CONST != 0 {
        out.push("donotuse-const");
    }
    if modes & sig_mods::SIG_DONOTUSE_INPUT != 0 {
        out.push("donotuse-input");
    }
    if modes & sig_mods::SIG_DONOTUSE_PERSIST != 0 {
        out.push("donotuse-persist");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bsim_settings_match_the_shipped_weights_files() {
        // Every `lshweights_*.xml` declares this, and the engine must accept it.
        assert_eq!(settings_vector(), 0x49);
        assert!(settings_are_valid(BSIM_SIG_SETTINGS));
        // The mode bits `0x49` actually selects.
        let modes = BSIM_SIG_SETTINGS >> 2;
        assert_eq!(modes, 0x12);
        assert_ne!(modes & sig_mods::SIG_COLLAPSE_INDNOISE, 0);
        assert_ne!(modes & sig_mods::SIG_DONOTUSE_CONST, 0);
        // `SIG_COLLAPSE_SIZE` is NOT among them; asserting it was a doc error.
        assert_eq!(modes & sig_mods::SIG_COLLAPSE_SIZE, 0);
    }

    #[test]
    fn a_zero_setting_is_invalid() {
        assert!(!settings_are_valid(0));
    }

    #[test]
    fn the_mode_names_describe_what_the_run_asks_for() {
        let modes = signature_modes();
        assert!(modes.contains(&"collapse-indnoise"));
        assert!(modes.contains(&"donotuse-const"));
        // Settings `0x49` does NOT set: size collision, input and persist facts.
        assert!(!modes.contains(&"collapse-size"));
        assert!(!modes.contains(&"donotuse-input"));
        assert!(!modes.contains(&"donotuse-persist"));
    }

    #[test]
    fn a_record_keys_on_name_and_address() {
        let sig = BsimSignature {
            name: "decode_packet".into(),
            address: 0x401000,
            features: vec![1, 2, 3],
            has_unimplemented: false,
            has_bad_data: false,
        };
        assert_eq!(sig.key(), ("decode_packet", 0x401000));
        assert_eq!(sig.num_features(), 3);
    }

    #[test]
    fn an_outcome_says_why_in_one_sentence() {
        assert!(SignatureOutcome::NoFeatures.describe().contains("no features"));
        assert!(SignatureOutcome::Refused("too big".into())
            .describe()
            .contains("too big"));
    }
}

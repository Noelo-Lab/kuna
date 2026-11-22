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

use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::signature::GraphSigManager;

/// One signed function, ready to be stored in a corpus.
#[derive(Clone, Debug, PartialEq)]
pub struct BsimSignature {
    /// The function's name, as the corpus records it.
    pub name: String,
    /// The function's entry address in its own program.
    pub address: u64,
    /// The settings used to generate these feature hashes.
    pub settings: u32,
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

    /// The feature count. Signing refuses an empty signature.
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

/// Generate features under the settings supplied by a BSim profile or database.
pub fn sign_function_with_settings(
    fd: &Funcdata,
    name: &str,
    address: u64,
    settings: u32,
) -> Result<BsimSignature, SignatureOutcome> {
    let mut manager = GraphSigManager::with_settings(settings)
        .map_err(|e| SignatureOutcome::Refused(e.explain().to_string()))?;
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
        settings,
        features,
        has_unimplemented: fd.has_unimplemented(),
        has_bad_data: fd.has_bad_data(),
    })
}

/// True when `settings` is a value the engine accepts.
pub fn settings_are_valid(settings: u32) -> bool {
    GraphSigManager::test_settings(settings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_stock_profiles_select_different_size_modes() {
        use kuna_decomp::signature::sig_mods::SIG_COLLAPSE_SIZE;
        for settings in [0x49, 0x4d] {
            assert!(settings_are_valid(settings));
        }
        assert_eq!((0x49 >> 2) & SIG_COLLAPSE_SIZE, 0);
        assert_ne!((0x4d >> 2) & SIG_COLLAPSE_SIZE, 0);
    }

    #[test]
    fn a_zero_setting_is_invalid() {
        assert!(!settings_are_valid(0));
    }

    #[test]
    fn a_record_keys_on_name_and_address() {
        let sig = BsimSignature {
            name: "decode_packet".into(),
            address: 0x401000,
            settings: 0x49,
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

use super::*;

fn object_of(sig: &Signature, fill: u8) -> Vec<u8> {
    let mut v = vec![fill; sig.len];
    v[sig.anchor_at..sig.anchor_at + sig.anchor.len()].copy_from_slice(sig.anchor);
    v
}

#[test]
fn every_signature_has_a_distinct_hash() {
    let mut hashes: Vec<u64> = SIGNATURES.iter().map(|s| s.hash).collect();
    hashes.sort_unstable();
    hashes.dedup();
    assert_eq!(hashes.len(), SIGNATURES.len());
}

#[test]
fn an_anchor_without_the_object_is_no_match() {
    for sig in SIGNATURES {
        let code = object_of(sig, 0xcc);
        assert!(scan_bytes(&code, 0x400000).is_empty(), "{}", sig.intrinsic);
    }
}

#[test]
fn the_hash_ignores_relocated_bytes_only() {
    let sig = &SIGNATURES[1];
    let mut a = object_of(sig, 0x90);
    let base = masked_hash(&a, sig.wild);
    let r = sig.wild[0].clone();
    a[r.start] ^= 0xff;
    assert_eq!(masked_hash(&a, sig.wild), base, "a relocated byte does not count");
    let fixed = (0..sig.len).find(|i| !sig.wild.iter().any(|r| r.contains(i))).unwrap();
    a[fixed] ^= 0xff;
    assert_ne!(masked_hash(&a, sig.wild), base, "any other byte does");
}

#[test]
fn every_named_intrinsic_has_the_model() {
    for sig in SIGNATURES {
        assert!(kuna_decomp::kuna_x87intrinsic::is_intrinsic(sig.intrinsic), "{}", sig.intrinsic);
    }
}

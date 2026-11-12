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
        assert!(scan_bytes(&code, 0x400000, SIGNATURES).is_empty(), "{}", sig.intrinsic);
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

#[test]
fn only_a_stub_jumping_to_the_dispatcher_is_found() {
    // mov edx, 0x11223344; jmp <0x1000 + 10 + rel>
    let stub = |rel: i32| {
        let mut v = vec![0xba, 0x44, 0x33, 0x22, 0x11, 0xe9];
        v.extend_from_slice(&rel.to_le_bytes());
        v
    };
    let mut code = stub(0x100);
    code.extend(stub(0x200 - 10));
    assert_eq!(table_stubs(&code, 0x1000, 0x1000 + 10 + 0x100), vec![(0x1000, 0x1122_3344)]);
    assert_eq!(table_stubs(&code, 0x1000, 0x1000 + 20 + 0x200 - 10), vec![(0x100a, 0x1122_3344)]);
    assert!(table_stubs(&code, 0x1000, 0x9999).is_empty());
}

#[test]
fn the_dispatcher_is_not_an_intrinsic() {
    assert!(!kuna_decomp::kuna_x87intrinsic::is_intrinsic(DISPATCHER.intrinsic));
    let binary = SIGNATURES
        .iter()
        .filter(|s| kuna_decomp::kuna_x87intrinsic::model_for(s.intrinsic).is_some_and(|m| m.1.len() == 2))
        .count();
    assert_eq!(binary, 2, "pow and fmod");
}

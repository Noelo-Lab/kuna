//! Logic-level tests for `kuna_preserve_thumb_funcptr` (GH-8471), exercising the
//! mode-bit function-pointer decision on hand-built type-system facts.

use crate::dtype::type_metatype;

use super::*;

/// The canonical Thumb function-pointer facts: `*spacebase` pointing at a
/// code-typed symbol, the constant landing exactly on the symbol (residual 0).
/// `word_size = 1` so byte-scaling is identity.
fn good_facts() -> ThumbPtrTypeFacts {
    ThumbPtrTypeFacts {
        base_metatype: type_metatype::TYPE_PTR,
        ptr_to_metatype: type_metatype::TYPE_SPACEBASE,
        word_size: 1,
        sub: Some(ThumbSubTypeFact {
            metatype: type_metatype::TYPE_CODE,
            residual_offset: 0,
        }),
    }
}

// funcptr_align = 1 (the Thumb LSB): extra == 1 is the only valid encoding.
const ALIGN: i32 = 1;

#[test]
fn canonical_thumb_funcptr_fires() {
    // val lands on the symbol; extra == 1 == the Thumb LSB.
    assert!(kuna_preserve_thumb_funcptr(true, ALIGN, 0x1000, 1, 1, &good_facts()));
}

#[test]
fn gate_off_returns_false() {
    assert!(!kuna_preserve_thumb_funcptr(false, ALIGN, 0x1000, 1, 1, &good_facts()));
}

#[test]
fn no_funcptr_align_returns_false() {
    // funcptr_align == 0: the architecture has no mode bit, so never fires.
    assert!(!kuna_preserve_thumb_funcptr(true, 0, 0x1000, 1, 1, &good_facts()));
}

#[test]
fn multiplier_gt_one_returns_false() {
    // multiplier > 1 is not the byte-grain PTRADD form.
    assert!(!kuna_preserve_thumb_funcptr(true, ALIGN, 0x1000, 1, 2, &good_facts()));
}

#[test]
fn base_not_pointer_returns_false() {
    let mut f = good_facts();
    f.base_metatype = type_metatype::TYPE_INT;
    assert!(!kuna_preserve_thumb_funcptr(true, ALIGN, 0x1000, 1, 1, &f));
}

#[test]
fn ptr_to_not_spacebase_returns_false() {
    let mut f = good_facts();
    f.ptr_to_metatype = type_metatype::TYPE_STRUCT;
    assert!(!kuna_preserve_thumb_funcptr(true, ALIGN, 0x1000, 1, 1, &f));
}

#[test]
fn null_subtype_returns_false() {
    let mut f = good_facts();
    f.sub = None; // getSubType returned null
    assert!(!kuna_preserve_thumb_funcptr(true, ALIGN, 0x1000, 1, 1, &f));
}

#[test]
fn nonzero_residual_offset_returns_false() {
    let mut f = good_facts();
    f.sub = Some(ThumbSubTypeFact {
        metatype: type_metatype::TYPE_CODE,
        residual_offset: 4, // val did not land exactly on the symbol
    });
    assert!(!kuna_preserve_thumb_funcptr(true, ALIGN, 0x1000, 1, 1, &f));
}

#[test]
fn subtype_not_code_returns_false() {
    let mut f = good_facts();
    f.sub = Some(ThumbSubTypeFact {
        metatype: type_metatype::TYPE_INT, // not a function/code symbol
        residual_offset: 0,
    });
    assert!(!kuna_preserve_thumb_funcptr(true, ALIGN, 0x1000, 1, 1, &f));
}

#[test]
fn extra_zero_or_negative_returns_false() {
    // bextra <= 0: not an alignment-encoding offset.
    assert!(!kuna_preserve_thumb_funcptr(true, ALIGN, 0x1000, 0, 1, &good_facts()));
    assert!(!kuna_preserve_thumb_funcptr(true, ALIGN, 0x1000, -1, 1, &good_facts()));
}

#[test]
fn extra_at_or_above_align_returns_false() {
    // bextra >= (1 << funcptr_align): with align=1 that bound is 2.
    assert!(!kuna_preserve_thumb_funcptr(true, ALIGN, 0x1000, 2, 1, &good_facts()));
    // With align=2 the bound is 4; extra==3 fits (in band), extra==4 does not.
    assert!(kuna_preserve_thumb_funcptr(true, 2, 0x1000, 3, 1, &good_facts()));
    assert!(!kuna_preserve_thumb_funcptr(true, 2, 0x1000, 4, 1, &good_facts()));
}

#[test]
fn word_size_scales_extra() {
    // word_size = 2: addressToByteInt(extra=1) = 2, which is >= (1<<1)=2 -> reject.
    let mut f = good_facts();
    f.word_size = 2;
    assert!(!kuna_preserve_thumb_funcptr(true, ALIGN, 0x1000, 1, 1, &f));
    // ... but with align=2 (bound 4), bextra=2 is in band.
    assert!(kuna_preserve_thumb_funcptr(true, 2, 0x1000, 1, 1, &f));
}

#[test]
fn option_default_is_on_and_apply_flips() {
    // Shipped default: option thumbfuncptr on (DIV-2 default-on).
    let mut opt = ThumbFuncPtrOption::default();
    assert!(opt.is_enabled());
    let msg = opt.apply(false);
    assert!(!opt.is_enabled());
    assert!(msg.contains("off"));
    let msg = opt.apply(true);
    assert!(opt.is_enabled());
    assert!(msg.contains("on"));
}

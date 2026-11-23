use super::{Helper, Source, recognise};

/// `outlined destroy` whose metadata is a local type's static metadata
/// (`adrp`/`add x1`).
const STATIC_DESTROY: [u32; 13] = [
    0xa9be4ff4, 0xa9017bfd, 0x910043fd, 0xaa0003f3, 0xf001f141, 0x91136021, 0xf85f8028, 0xf9400508,
    0xd63f0100, 0xaa1303e0, 0xa9417bfd, 0xa8c24ff4, 0xd65f03c0,
];

/// The same with the metadata loaded from a pointer slot (`adrp`/`ldr x1`).
const SLOT_DESTROY: [u32; 13] = [
    0xa9be4ff4, 0xa9017bfd, 0x910043fd, 0xaa0003f3, 0xd001fae1, 0xf940ec21, 0xf85f8028, 0xf9400508,
    0xd63f0100, 0xaa1303e0, 0xa9417bfd, 0xa8c24ff4, 0xd65f03c0,
];

/// A class's ivar destroyer: it destroys a field at an offset off `self`
/// through the field type's witness, then tail-calls the runtime.
const IVAR_DESTROYER: [u32; 19] = [
    0xa9be4ff4, 0xa9017bfd, 0x910043fd, 0xb00200e8, 0xf9439113, 0xd2800000, 0x94c5b2db, 0xaa0003e1,
    0xf85f8008, 0xf9400508, 0x8b130280, 0xd63f0100, 0xf9400288, 0xb9403101, 0x79406902, 0xaa1403e0,
    0xa9417bfd, 0xa8c24ff4, 0x14c5df87,
];

#[test]
fn a_static_metadata_destroy_is_recognised() {
    assert_eq!(
        recognise(&STATIC_DESTROY, 0x1_0001_659c),
        Some(Helper {
            kind: 0,
            source: Source::Static(0x1_03e4_14d8),
            values: vec![0],
            returns: Some(0),
        })
    );
}

#[test]
fn a_slot_metadata_destroy_is_recognised() {
    assert_eq!(
        recognise(&SLOT_DESTROY, 0x1_0001_a768),
        Some(Helper {
            kind: 0,
            source: Source::Slot(0x1_03f7_81d8),
            values: vec![0],
            returns: Some(0),
        })
    );
}

#[test]
fn a_body_that_does_more_than_one_witness_call_is_not_a_helper() {
    assert_eq!(recognise(&IVAR_DESTROYER, 0x1_0003_6fa4), None);
}

/// A merged `outlined destroy` whose metadata comes from
/// `__swift_instantiateConcreteTypeFromMangledName` at a cache it is passed
/// (`mov x0, x1; bl`); x1 still holds the same argument at the call.
const FORWARDED_DESTROY: [u32; 15] = [
    0xa9be4ff4, 0xa9017bfd, 0x910043fd, 0xaa0003f3, 0xaa0103e0, 0x97ffdefe, 0xaa0003e1, 0xf85f8008,
    0xf9400508, 0xaa1303e0, 0xd63f0100, 0xaa1303e0, 0xa9417bfd, 0xa8c24ff4, 0xd65f03c0,
];

/// `destroy value witness` of a struct whose one non-trivial field is a
/// resilient type: the field's accessor and destroy witness, tail-called.
const WITNESS_TAIL_DESTROY: [u32; 10] = [
    0xa9bf4ffe, 0xaa0003f3, 0xd2800000, 0x9400082f, 0xaa0003e1, 0xaa1303e0, 0xf85f8028, 0xf9400502,
    0xa8c14ffe, 0xd61f0040,
];

/// The same called, then returning without handing the value back.
const WITNESS_CALL_DESTROY: [u32; 14] = [
    0xa9be4ff4, 0xa9017bfd, 0x910043fd, 0xaa0003f3, 0xd2800000, 0x97ffdefe, 0xaa0003e1, 0xf85f8008,
    0xf9400508, 0xaa1303e0, 0xd63f0100, 0xa9417bfd, 0xa8c24ff4, 0xd65f03c0,
];

#[test]
fn a_forwarded_cache_is_one_argument() {
    assert_eq!(
        recognise(&FORWARDED_DESTROY, 0xfcdc),
        Some(Helper {
            kind: 0,
            source: Source::Forwarded(0x78e8, vec![1]),
            values: vec![0],
            returns: Some(0),
        })
    );
}

#[test]
fn a_body_that_does_not_return_the_value_is_not_a_helper() {
    assert_eq!(recognise(&WITNESS_TAIL_DESTROY, 0x13bb8), None);
    assert_eq!(recognise(&WITNESS_CALL_DESTROY, 0xfcdc), None);
}

#[test]
fn a_body_cut_off_before_its_return_is_not_a_helper() {
    assert_eq!(recognise(&FORWARDED_DESTROY[..12], 0xfcdc), None);
}

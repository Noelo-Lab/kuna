//! Several images of a dyld shared cache in one run (GH #1622): `--image A+B`
//! loads both as one program, and an address outside the loaded images loads
//! the image that holds it (`--follow-images N`), a stub island first followed
//! to the function it jumps to.
//!
//! Fixture: `dyld_cache_arm64e/` (`dyld_cache_arm64e.py`), whose `libcaller`
//! calls `callee_add` (0x180002800, in `libcallee`) through the island at
//! 0x180008800. Integration tests require the built processor specs under
//! `specs/`.

use crate::common;

const CALLEE_BODY: &str = "return a0 + a1;";

fn kuna(sub: &str, args: &[&str]) -> (String, String, i32) {
    let cache = common::fixture("dyld_cache_arm64e/dyld_shared_cache_arm64e");
    let mut argv = vec![sub, cache.as_str()];
    argv.extend_from_slice(args);
    common::run_kuna(&argv)
}

#[test]
fn two_images_load_as_one_program() {
    let (stdout, stderr, rc) = kuna("decompile", &["0x180002800", "--addr", "--image", "libcaller+libcallee"]);
    assert_eq!(rc, 0, "{stderr}");
    assert!(stdout.contains("_callee_add(int a0,int a1)") && stdout.contains(CALLEE_BODY), "{stdout}");
    assert!(!stderr.contains("[kuna]"), "nothing was followed: {stderr}");

    let (stdout, stderr, rc) = kuna("functions", &["--option", "dyldimage", "libcallee+libcaller"]);
    assert_eq!(rc, 0, "{stderr}");
    for function in ["0x180001800\t_caller_entry", "0x180001828\t_helper_local", "0x180002800\t_callee_add"] {
        assert!(stdout.contains(function), "{function}: {stdout}");
    }
}

#[test]
fn an_address_in_another_image_loads_that_image() {
    let (stdout, stderr, rc) = kuna("decompile", &["0x180002800", "--addr", "--image", "libcaller"]);
    assert_eq!(rc, 0, "{stderr}");
    assert!(stdout.contains(CALLEE_BODY), "{stdout}");
    assert!(stderr.contains("0x180002800 lies in /usr/lib/libcallee.dylib; loading that image too"), "{stderr}");

    let (stdout, stderr, rc) = kuna("xrefs", &["--from", "0x180002800", "--option", "dyldimage", "libcaller"]);
    assert_eq!(rc, 0, "{stderr}");
    assert!(stdout.contains("_callee_add @ 0x180002800"), "{stdout}");
}

#[test]
fn a_stub_island_is_followed_to_its_function() {
    for sub in ["decompile", "disassemble"] {
        let (stdout, stderr, rc) = kuna(sub, &["0x180008800", "--addr", "--image", "libcaller"]);
        assert_eq!(rc, 0, "{sub}: {stderr}");
        assert!(stdout.contains("_callee_add"), "{sub}: {stdout}");
        assert!(stderr.contains("0x180008800 is a stub for 0x180002800"), "{sub}: {stderr}");
    }
    let (stdout, stderr, rc) = kuna("xrefs", &["--to", "0x180008800", "--image", "libcaller"]);
    assert_eq!(rc, 0, "{stderr}");
    assert!(stdout.contains("1 reference to callee_add @ 0x180008800"), "the island is what the call names: {stdout}");
    assert!(!stderr.contains("[kuna]"), "{stderr}");
}

#[test]
fn follow_images_zero_keeps_the_selection() {
    let (_, stderr, rc) =
        kuna("decompile", &["0x180002800", "--addr", "--image", "libcaller", "--follow-images", "0"]);
    assert_ne!(rc, 0);
    assert!(stderr.contains("address 0x180002800 is not mapped in this input"), "{stderr}");
    assert!(stderr.contains("lies in /usr/lib/libcallee.dylib, which is not loaded"), "{stderr}");
}

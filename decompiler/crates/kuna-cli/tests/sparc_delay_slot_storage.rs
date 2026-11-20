use crate::common;
use serde_json::Value;

const WORDS: [u32; 12] = [
    0x9de3_bf90, // save %sp, -112, %sp
    0xf027_bffc, // st %i0, [%fp-4]  (independent saved value)
    0xc027_bff4, // st %g0, [%fp-12]
    0x9007_bff4, // add %fp, -12, %o0
    0x4000_0006, // call 0x1028
    0xc027_bff8, // st %g0, [%fp-8]  (delay slot)
    0xf007_bffc, // ld [%fp-4], %i0
    0xb01a_0018, // xor %o0, %i0, %i0
    0x81c7_e008, // ret
    0x81e8_0000, // restore
    0x81c3_e008, // retl
    0xd002_2004, // ld [%o0+4], %o0
];

fn decompile(words: [u32; 12], proto: &str, extent: &str) -> Value {
    let path = common::scratch_file("sparc-delay-slot-storage", "bin");
    std::fs::write(&path, words.into_iter().flat_map(u32::to_be_bytes).collect::<Vec<_>>())
        .unwrap();
    let (out, err, status) = common::run_kuna(&[
        "decompile", path.to_str().unwrap(), "0x1000", "--json", "--raw-image",
        "--target", "sparc:BE:32:default:gcc", "--base", "0x1000",
        "--option", "arrayextent", extent,
        "--define-function", "0x1000-0x1028=fixture",
        "--define-function", "0x1028-0x1030=inspect_pair", "--assert", proto,
    ]);
    std::fs::remove_file(path).unwrap();
    assert_eq!(status, 0, "{err}");
    let doc: Value = serde_json::from_str(&out).unwrap();
    let record = &doc["functions"][0];
    assert!(record["error"].is_null(), "{record}");
    record.clone()
}

fn at(record: &Value, offset: i64, size: u64) -> &Value {
    record["variables"].as_array().unwrap().iter().find(|var| {
        var["stack_offset"].as_i64() == Some(offset) && var["size"].as_u64() == Some(size)
    }).unwrap_or_else(|| panic!("missing {size}-byte local at {offset}: {}", record["code"]))
}

#[test]
fn name_independent_sparc_call_delay_slot_keeps_both_words() {
    let proto = "prototype inspect_pair unsigned int inspect_pair(void *data)";
    let record = decompile(WORDS, proto, "on");
    let array = at(&record, -12, 8)["name"].as_str().unwrap();
    let separate = at(&record, -4, 4)["name"].as_str().unwrap();
    let code = record["code"].as_str().unwrap();
    assert!(code.contains(&format!("{array}[0] = 0;")), "{code}");
    assert!(code.contains(&format!("{array}[1] = 0;")), "{code}");
    assert!(code.contains(&format!("inspect_pair({array}) ^ {separate}")), "{code}");
    let off = decompile(WORDS, proto, "off");
    at(&off, -12, 4);
    at(&off, -8, 4);
}

#[test]
fn adjacent_zero_without_delay_slot_evidence_stays_separate() {
    let mut words = WORDS;
    words.swap(3, 5); // Put the tail store before the call, not in its bundle.
    let record = decompile(words, "prototype inspect_pair unsigned int inspect_pair(void *data)", "on");
    at(&record, -12, 4);
    at(&record, -8, 4);
    at(&record, -4, 4);
}

#[test]
fn separately_passed_delay_slot_word_stays_separate() {
    let mut words = WORDS;
    words[1] = 0x9207_bff8; // add %fp, -8, %o1
    words[6] = 0x0100_0000; // nop (the independent slot is not used)
    let record = decompile(words,
        "prototype inspect_pair unsigned int inspect_pair(void *head, void *tail)", "on");
    let head = at(&record, -12, 4)["name"].as_str().unwrap();
    let tail = record["variables"].as_array().unwrap().iter()
        .find(|v| v["stack_offset"].as_i64() == Some(-8))
        .and_then(|v| v["name"].as_str()).expect("no separate local at -8");
    let c = record["code"].as_str().unwrap();
    assert!(c.contains(&format!("inspect_pair(&{head},{tail})"))
        || c.contains(&format!("inspect_pair(&{head},&{tail})")), "{c}");
}

#[test]
fn directly_read_stack_word_keeps_the_baseline_layout() {
    let mut words = WORDS;
    words[6] = 0xf007_bff4; // ld [%fp-12], %i0
    let proto = "prototype inspect_pair unsigned int inspect_pair(void *data)";
    let on = decompile(words, proto, "on");
    let off = decompile(words, proto, "off");
    assert_eq!(on["code"], off["code"]);
}

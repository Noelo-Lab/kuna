#[path = "common/arm_images.rs"]
#[allow(dead_code)]
mod arm_images;

use crate::common;

const FIRMWARE: &[u8] = &[
    0x41, 0xf2, 0x00, 0x01, // movw r1, #0x1000
    0xc4, 0xf2, 0x02, 0x01, // movt r1, #0x4002
    0x08, 0x68, // ldr r0, [r1]
    0x0a, 0x68, // ldr r2, [r1] (otherwise dead)
    0x88, 0x80, // strh r0, [r1, #4]
    0x70, 0x47, // bx lr
];

const DEVICE: &str = r#"
<device schemaVersion="1.3">
  <name>SvdFixture</name><version>1.0</version><description>MMIO fixture</description>
  <addressUnitBits>8</addressUnitBits><width>32</width><size>32</size>
  <peripherals>
    <peripheral derivedFrom="TEMPLATE">
      <name>RCC</name><baseAddress>0x40020000</baseAddress>
    </peripheral>
    <peripheral>
      <name>TEMPLATE</name><baseAddress>0x50000000</baseAddress>
      <registers>
        <register><name>WORD</name><addressOffset>0x3000</addressOffset><size>32</size></register>
        <cluster>
          <dim>2</dim><dimIncrement>0x1000</dimIncrement>
          <name>BANK[%s]</name><addressOffset>0</addressOffset>
          <cluster>
            <name>CFG</name><addressOffset>0</addressOffset><size>16</size>
            <register>
              <name>ALIAS</name><alternateRegister>CR</alternateRegister><addressOffset>0</addressOffset>
            </register>
            <register derivedFrom="TEMPLATE.WORD">
              <name>CR</name><addressOffset>0</addressOffset><access>read-only</access>
            </register>
            <register>
              <dim>2</dim><dimIncrement>2</dimIncrement><dimIndex>A-B</dimIndex>
              <name>SLOT%s</name><addressOffset>4</addressOffset>
            </register>
          </cluster>
        </cluster>
      </registers>
    </peripheral>
  </peripherals>
</device>
"#;

const RAW: &[&str] = &[
    "--raw-image",
    "--target",
    "ARM:LE:32:v8:default",
    "--base",
    "0x8000000",
    "--isa",
    "thumb",
];

#[test]
fn bare_metal_registers_are_named_and_both_volatile_reads_survive() {
    let dir = tempfile::tempdir().unwrap();
    let fw = dir.path().join("example.bin");
    let svd = dir.path().join("device map.svd");
    std::fs::write(&fw, FIRMWARE).unwrap();
    std::fs::write(&svd, DEVICE).unwrap();
    let fw = fw.to_str().unwrap();
    let svd = svd.to_str().unwrap();
    let flags = [RAW, &["--svd", svd, "--assert-strict"]].concat();
    let (text, err, status) =
        common::run_kuna(&[["decompile", fw, "0x8000000"].as_slice(), &flags].concat());
    assert_eq!(status, 0, "{err}\n{text}");
    check_register_accesses(&text, "RCC_BANK1_CFG_CR");

    let (json, err, status) = common::run_kuna(
        &[
            &["decompile", fw, "0x8000000", "--json"],
            flags.as_slice(),
            &["--assert", "data 0x40021000 volatile uint4 CONTROL"],
        ]
        .concat(),
    );
    assert_eq!(status, 0, "{err}\n{json}");
    let json: serde_json::Value = serde_json::from_str(&json).unwrap();
    check_register_accesses(json["functions"][0]["code"].as_str().unwrap(), "CONTROL");

    let (text, err, status) = common::run_kuna(
        &[
            &["decompile-all", fw, "--addr", "0x8000000"],
            flags.as_slice(),
        ]
        .concat(),
    );
    assert_eq!(status, 0, "{err}\n{text}");
    check_register_accesses(&text, "RCC_BANK1_CFG_CR");

    let export = dir.path().join("export");
    let (out, err, status) = common::run_kuna(
        &[
            &[
                "decompile-project",
                fw,
                "--addr",
                "0x8000000",
                "-o",
                export.to_str().unwrap(),
            ],
            flags.as_slice(),
        ]
        .concat(),
    );
    assert_eq!(status, 0, "{err}\n{out}");
    let code = std::fs::read_to_string(export.join("example.bin.c")).unwrap();
    check_register_accesses(&code, "RCC_BANK1_CFG_CR");
    let header = std::fs::read_to_string(export.join("example.bin.h")).unwrap();
    assert!(
        header.contains("extern volatile unsigned int RCC_BANK1_CFG_CR;"),
        "{header}"
    );
    assert!(
        header.contains("extern volatile unsigned short RCC_BANK1_CFG_SLOTA;"),
        "{header}"
    );
}

fn check_register_accesses(text: &str, control: &str) {
    assert_eq!(text.matches(&format!("= {control};")).count(), 2, "{text}");
    assert!(text.contains("RCC_BANK1_CFG_SLOTA ="), "{text}");
}

#[test]
fn a_broken_layout_fails_with_its_source_instead_of_decompiling_without_the_map() {
    let dir = tempfile::tempdir().unwrap();
    let fw = dir.path().join("example.bin");
    let svd = dir.path().join("broken.svd");
    std::fs::write(&fw, FIRMWARE).unwrap();
    let broken = DEVICE.replace("TEMPLATE.WORD", "TEMPLATE.MISSING");
    std::fs::write(&svd, broken).unwrap();
    let svd = svd.to_str().unwrap();
    let args = [
        ["decompile", fw.to_str().unwrap(), "0x8000000"].as_slice(),
        RAW,
        &["--svd", svd],
    ]
    .concat();
    let (text, err, status) = common::run_kuna(&args);
    assert_eq!(status, 2, "{err}\n{text}");
    assert!(text.is_empty(), "{text}");
    assert!(
        err.contains(svd) && err.contains("TEMPLATE.MISSING"),
        "{err}"
    );
}

const MASK_FIRMWARE: &[u8] = &[
    0x41, 0xf2, 0x00, 0x01, // movw r1, #0x1000
    0xc4, 0xf2, 0x02, 0x01, // movt r1, #0x4002
    0x08, 0x68, // ldr r0, [r1]
    0x4a, 0x68, // ldr r2, [r1, #4]
    0x00, 0xf4, 0x00, 0x30, // and r0, r0, #0x20000
    0x02, 0xf4, 0x00, 0x32, // and r2, r2, #0x20000
    0x10, 0x44, // add r0, r2
    0x70, 0x47, // bx lr
];

const MASK_DEVICE: &str = r#"
<device schemaVersion="1.3">
  <name>SvdMaskFixture</name><addressUnitBits>8</addressUnitBits><size>32</size>
  <peripherals><peripheral>
    <name>RCC</name><baseAddress>0x40021000</baseAddress>
    <registers>
      <register><name>CR</name><addressOffset>0</addressOffset>
        <fields>
          <field derivedFrom="RCC.TEMPLATE.READY1"><name>HSERDY</name></field>
          <field><name>OTHER</name><lsb>2</lsb><msb>4</msb></field>
        </fields>
      </register>
      <register><name>CFGR</name><addressOffset>4</addressOffset></register>
      <register><name>TEMPLATE</name><addressOffset>8</addressOffset>
        <fields><field><dim>2</dim><dimIncrement>1</dimIncrement><name>READY%s</name><bitRange>[16:16]</bitRange></field></fields>
      </register>
    </registers>
  </peripheral></peripherals>
</device>
"#;

#[test]
fn a_mask_names_only_the_field_of_the_register_that_was_read() {
    let dir = tempfile::tempdir().unwrap();
    let fw = dir.path().join("mask.bin");
    let svd = dir.path().join("mask.svd");
    std::fs::write(&fw, MASK_FIRMWARE).unwrap();
    std::fs::write(&svd, MASK_DEVICE).unwrap();
    let args = [
        ["decompile", fw.to_str().unwrap(), "0x8000000"].as_slice(),
        RAW,
        &["--svd", svd.to_str().unwrap(), "--assert-strict"],
    ]
    .concat();
    let (text, err, status) = common::run_kuna(&args);
    assert_eq!(status, 0, "{err}\n{text}");
    check_mask(&text);

    let (json, err, status) = common::run_kuna(&[args.as_slice(), &["--json"]].concat());
    assert_eq!(status, 0, "{err}\n{json}");
    let json: serde_json::Value = serde_json::from_str(&json).unwrap();
    check_mask(json["functions"][0]["code"].as_str().unwrap());

    let ambiguous = MASK_DEVICE.replace(
        "<field><name>OTHER</name><lsb>2</lsb><msb>4</msb></field>",
        "<field><name>ALIAS</name><bitOffset>17</bitOffset><bitWidth>1</bitWidth></field>",
    );
    std::fs::write(&svd, ambiguous).unwrap();
    let (text, err, status) = common::run_kuna(&args);
    assert_eq!(status, 0, "{err}\n{text}");
    assert!(!text.contains("_Msk"), "{text}");
    assert_eq!(text.matches("& 0x20000").count(), 2, "{text}");

    let alternate = MASK_DEVICE.replace(
        "</registers>",
        r#"<register><name>CR_INPUT</name><addressOffset>0</addressOffset>
           <alternateRegister>CR</alternateRegister>
           <fields><field><name>FILTER</name><bitRange>[18:17]</bitRange></field></fields>
           </register></registers>"#,
    );
    std::fs::write(&svd, alternate).unwrap();
    let (text, err, status) = common::run_kuna(&args);
    assert_eq!(status, 0, "{err}\n{text}");
    assert!(text.contains("= RCC_CR;"), "{text}");
    assert!(!text.contains("_Msk"), "{text}");
    assert_eq!(text.matches("& 0x20000").count(), 2, "{text}");
}

fn check_mask(text: &str) {
    assert!(
        text.contains("#define RCC_CR_HSERDY_Msk (0x20000)"),
        "{text}"
    );
    assert!(text.contains("#undef RCC_CR_HSERDY_Msk"), "{text}");
    assert_eq!(text.matches("& RCC_CR_HSERDY_Msk").count(), 1, "{text}");
    assert!(text.contains("RCC_CFGR & 0x20000"), "{text}");
    assert_eq!(text.matches("= RCC_CR;").count(), 1, "{text}");
}

#[test]
fn a_field_array_mask_does_not_rename_a_narrow_read_of_its_register() {
    let dir = tempfile::tempdir().unwrap();
    let fw = dir.path().join("width.bin");
    let svd = dir.path().join("width.svd");
    let mut firmware = MASK_FIRMWARE.to_vec();
    firmware[10..12].copy_from_slice(&[0x0a, 0x88]); // ldrh r2, [r1]
    firmware[12..20].copy_from_slice(&[0x00, 0xf0, 0x60, 0x00, 0x02, 0xf0, 0x60, 0x02]);
    std::fs::write(&fw, firmware).unwrap();
    let device = MASK_DEVICE.replace(
        "<field><name>OTHER</name><lsb>2</lsb><msb>4</msb></field>",
        "<field><dim>2</dim><dimIncrement>4</dimIncrement><name>SEL%s</name><lsb>1</lsb><msb>2</msb></field>",
    );
    std::fs::write(&svd, device).unwrap();
    let args = [
        ["decompile", fw.to_str().unwrap(), "0x8000000"].as_slice(),
        RAW,
        &["--svd", svd.to_str().unwrap(), "--assert-strict"],
    ]
    .concat();
    let (text, err, status) = common::run_kuna(&args);
    assert_eq!(status, 0, "{err}\n{text}");
    assert!(text.contains("#define RCC_CR_SEL1_Msk (0x60)"), "{text}");
    assert_eq!(text.matches("& RCC_CR_SEL1_Msk").count(), 1, "{text}");
    assert_eq!(text.matches("& 0x60").count(), 1, "{text}");
}

#[test]
fn oversized_svd_array_names_are_rejected_before_expansion() {
    let dir = tempfile::tempdir().unwrap();
    let fw = dir.path().join("mask.bin");
    let svd = dir.path().join("oversized.svd");
    std::fs::write(&fw, MASK_FIRMWARE).unwrap();
    let device = MASK_DEVICE.replace(
        "<name>CFGR</name>",
        &format!(
            "<dim>2</dim><dimIncrement>4</dimIncrement><name>{}%s</name>",
            "r".repeat(257)
        ),
    );
    std::fs::write(&svd, device).unwrap();
    let svd = svd.to_str().unwrap();
    let (text, err, status) = common::run_kuna(
        &[
            ["decompile", fw.to_str().unwrap(), "0x8000000"].as_slice(),
            RAW,
            &["--svd", svd],
        ]
        .concat(),
    );
    assert_eq!(status, 2, "{err}\n{text}");
    assert!(
        err.contains(svd) && err.contains("name exceeds 256 bytes"),
        "{err}"
    );
}

#[test]
fn graph_strict_mode_rejects_a_partial_device_map() {
    let dir = tempfile::tempdir().unwrap();
    let fw = dir.path().join("graph.elf");
    let svd = dir.path().join("graph.svd");
    std::fs::write(
        &fw,
        arm_images::elf(
            MASK_FIRMWARE,
            &[(0, "$t")],
            &[(1, "poll", MASK_FIRMWARE.len() as u64)],
        ),
    )
    .unwrap();
    std::fs::write(&svd, MASK_DEVICE).unwrap();
    let (text, err, status) = common::run_kuna(&[
        "decompile-graph",
        fw.to_str().unwrap(),
        "--functions",
        "poll",
        "--svd",
        svd.to_str().unwrap(),
        "--assert",
        "data 0x40021004 no_such_type CONTROL",
        "--assert-strict",
    ]);
    assert_eq!(status, 1, "{err}\n{text}");
    assert!(text.is_empty(), "{text}");
    assert!(
        err.contains("no_such_type CONTROL") && err.contains("--assert-strict"),
        "{err}"
    );
}

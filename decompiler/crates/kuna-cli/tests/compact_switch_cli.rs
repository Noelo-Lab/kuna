//! Native switches must keep the selector and its case labels in one domain.
mod common;
use common::process;

use std::path::PathBuf;
use std::process::Command;

/// An x86-64 executable whose single segment maps `body` at 0x401000.
fn elf(body: &[u8]) -> Vec<u8> {
    let mut image = vec![0; 0x1000];
    image[..7].copy_from_slice(b"\x7fELF\x02\x01\x01");
    for (at, value) in [(16, 2u16), (18, 62), (52, 64), (54, 56), (56, 1)] {
        image[at..at + 2].copy_from_slice(&value.to_le_bytes());
    }
    for (at, value) in [(20, 1u32), (64, 1), (68, 5)] {
        image[at..at + 4].copy_from_slice(&value.to_le_bytes());
    }
    let size = (0x1000 + body.len()) as u64;
    for (at, value) in [
        (24, 0x401000u64),
        (32, 64),
        (80, 0x400000),
        (88, 0x400000),
        (96, size),
        (104, size),
        (112, 0x1000),
    ] {
        image[at..at + 8].copy_from_slice(&value.to_le_bytes());
    }
    image.extend(body);
    image
}

/// MSABI dispatch: bounded `mode - 5`, a byte map, a relative target table,
/// and a stored-mode retry backedge that re-dispatches `store->mode - 5` when
/// it is at most `retry_bound`.
fn compact_fixture(map: &[u8], targets: &[u32], retry_bound: usize) -> Vec<u8> {
    let mut code = vec![
        0x83, 0xc2, 0xfb, 0x83, 0xfa, 0x03, 0x0f, 0x87, 0x43, 0, 0, 0, 0x4c, 0x8d, 0x05, 0xed,
        0xef, 0xff, 0xff, 0x48, 0x63, 0xc2, 0x41, 0x0f, 0xb6, 0x84, 0, 0x55, 0x10, 0, 0, 0x41,
        0x8b, 0x94, 0x80, 0x5c, 0x10, 0, 0, 0x4c, 0x01, 0xc2, 0xff, 0xe2, 0xb8, 0x2c, 0x01, 0, 0,
        0xc3, 0xb8, 0xf0, 0, 0, 0, 0xc3, 0x8b, 0x11, 0x83, 0xfa, 0x08, 0x0f, 0x84, 0x0c, 0, 0, 0,
        0x83, 0xc2, 0xfb, 0x83, 0xfa, 0x03, 0x0f, 0x86, 0xc4, 0xff, 0xff, 0xff, 0xb8, 0x0e, 0x01,
        0, 0, 0xc3,
    ];
    assert_eq!(code.len(), 85);
    code[5] = (map.len() - 1) as u8;
    code[72] = retry_bound as u8;
    let table_at = (0x1000 + code.len() + map.len() + 3) & !3;
    code[35..39].copy_from_slice(&(table_at as u32).to_le_bytes());
    code.extend(map);
    code.resize(table_at - 0x1000, 0);
    for target in targets {
        code.extend(target.to_le_bytes());
    }
    elf(&code)
}

/// The same dispatch in GCC's non-PIC shape: a zero-extended index loads the
/// map byte at an absolute address and jumps through an absolute 8-byte
/// table, and the retry backedge re-enters at the zero extension.
fn absolute_fixture(map: &[u8], targets: &[u32], retry_bound: usize) -> Vec<u8> {
    let mut code = vec![
        0x83, 0xea, 0x05, 0xb8, 0x0e, 0x01, 0, 0, 0x83, 0xfa, 0x03, 0x0f, 0x87, 0x35, 0, 0, 0,
        0x89, 0xd2, 0x0f, 0xb6, 0x82, 0, 0, 0, 0, 0x3e, 0xff, 0x24, 0xc5, 0, 0, 0, 0, 0xb8, 0x2c,
        0x01, 0, 0, 0xc3, 0xb8, 0xf0, 0, 0, 0, 0xc3, 0x8b, 0x11, 0x83, 0xfa, 0x08, 0x74, 0x0c,
        0x83, 0xea, 0x05, 0x83, 0xfa, 0x03, 0x0f, 0x86, 0xd0, 0xff, 0xff, 0xff, 0xb8, 0x0e, 0x01,
        0, 0, 0xc3,
    ];
    assert_eq!(code.len(), 71);
    code[10] = (map.len() - 1) as u8;
    code[58] = retry_bound as u8;
    let map_at = 0x401000 + code.len();
    let table_at = (map_at + map.len() + 7) & !7;
    code[22..26].copy_from_slice(&(map_at as u32).to_le_bytes());
    code[30..34].copy_from_slice(&(table_at as u32).to_le_bytes());
    code.extend(map);
    code.resize(table_at - 0x401000, 0);
    for &target in targets {
        code.extend(u64::from(target).to_le_bytes());
    }
    elf(&code)
}

/// GCC's shape for a byte index in front of a short two-byte map:
/// `(unsigned char)(mode - 9)` is bounded to 0..9, the map value is checked
/// against the 21-entry absolute table, reloaded and dispatched, and the retry
/// backedge re-enters at the zero extension.  `past` fills the memory after
/// the map, which no bounded index reads.
fn short_map_fixture(past: &[u16]) -> Vec<u8> {
    let mut code = vec![
        0x83, 0xea, 0x09, 0xb8, 0x0e, 0x01, 0, 0, 0x80, 0xfa, 0x09, 0x77, 0x32, 0x0f, 0xb6, 0xd2,
        0x66, 0x83, 0xbc, 0x12, 0, 0, 0, 0, 0x14, 0x77, 0x43, 0x0f, 0xb7, 0x84, 0x12, 0, 0, 0, 0,
        0x3e, 0xff, 0x24, 0xc5, 0, 0, 0, 0, 0x8b, 0x11, 0x83, 0xfa, 0x08, 0x74, 0x08, 0x83, 0xea,
        0x09, 0x80, 0xfa, 0x09, 0x76, 0xd3, 0xb8, 0x0e, 0x01, 0, 0, 0xc3,
    ];
    for value in [1600u32, 1601, 1602, 1603, 1604, 77] {
        code.push(0xb8);
        code.extend(value.to_le_bytes());
        code.push(0xc3);
    }
    assert_eq!(code.len(), 100);
    let map_at = 0x401000 + code.len() as u32;
    for entry in [4u16, 15, 6, 11, 20, 20, 9, 11, 6, 1].iter().chain(past) {
        code.extend(entry.to_le_bytes());
    }
    let table_at = (0x401000 + code.len() as u32 + 7) & !7;
    code.resize((table_at - 0x401000) as usize, 0);
    for value in 0..21u64 {
        let target = match value {
            1 => 0x401040,
            4 | 11 => 0x401046,
            14 => 0x40104c,
            9 => 0x401052,
            6 | 20 => 0x401058,
            15 => 0x40102b,
            _ => 0x40105e,
        };
        code.extend(u64::to_le_bytes(target));
    }
    for at in [20, 31] {
        code[at..at + 4].copy_from_slice(&map_at.to_le_bytes());
    }
    code[39..43].copy_from_slice(&table_at.to_le_bytes());
    elf(&code)
}

/// SysV `pick(p, a, b)`: `k = *p - 'A'` is bounded to 0..3 three branches
/// before a relative table dispatch on `k`, out of reach of the guard search,
/// so the selector range is the full byte and the table ends at its first
/// far entry.  `past` overrides the entries after that far entry.
fn far_guard_fixture(past: &[i32]) -> Vec<u8> {
    let mut code = vec![
        0x0f, 0xb6, 0x07, 0x83, 0xe8, 0x41, 0x3c, 0x03, 0x77, 0x38, 0x85, 0xf6, 0x74, 0x3a, 0x85,
        0xd2, 0x74, 0x3c, 0x83, 0xfe, 0x05, 0x74, 0x3d, 0x0f, 0xb6, 0xc0, 0x48, 0x8d, 0x0d, 0x3b,
        0, 0, 0, 0x48, 0x63, 0x04, 0x81, 0x48, 0x01, 0xc8, 0xff, 0xe0,
    ];
    for value in [100u32, 101, 102, 103, 0xffff_ffff, 7, 8, 9] {
        code.push(0xb8);
        code.extend(value.to_le_bytes());
        code.push(0xc3);
    }
    assert_eq!(code.len(), 0x5a);
    code.resize(0x5c, 0);
    for entry in [-0x32i32, -0x2c, -0x26, -0x20] {
        code.extend(entry.to_le_bytes());
    }
    code.extend(0x4000_0000i32.to_le_bytes());
    for entry in (5..256).map(|i| past.get(i - 5).copied().unwrap_or(0x4000_0000)) {
        code.extend(entry.to_le_bytes());
    }
    elf(&code)
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn specs() -> PathBuf {
    std::env::var_os("SLEIGHHOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| root().join("specs"))
}

fn kuna() -> Command {
    Command::new(
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into()),
    )
}

fn printed(mut command: Command) -> String {
    let out = command.output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}

fn decompile(image: &[u8], asserts: &[&str], options: &[&str]) -> String {
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("fixture.elf");
    std::fs::write(&binary, image).unwrap();
    let mut command = kuna();
    command.args(["decompile-all"]).arg(&binary);
    for option in options {
        command.args(["--option", option, "off"]);
    }
    command
        .args(["--addr", "0x401000", "--mode", "reliable", "--assert-strict", "--sleighpath"])
        .arg(specs());
    for assertion in asserts {
        command.args(["--assert", assertion]);
    }
    printed(command)
}

fn run(image: &[u8], end: u32, guarded: bool) -> String {
    let options: &[&str] = if guarded { &[] } else { &["switchmultipred"] };
    let function = format!("function 0x401000-{end:#x}=compact_dispatch");
    decompile(
        image,
        &[
            "typedef struct ModeStore { unsigned int mode; };",
            &function,
            "prototype compact_dispatch int MSABI compact_dispatch(struct ModeStore *store, unsigned int mode)",
        ],
        options,
    )
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn compilers() -> Vec<&'static str> {
    let found: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!found.is_empty(), "the round trip requires a C compiler");
    found
}

/// Compile the printed `compact_dispatch` with every available compiler at
/// -O0 and -O2, map the fixture at its link address, and compare the printed
/// function with the fixture's own code for every mode against each stored
/// mode in `stores`.  An alarm turns a printed loop that never exits into a
/// failure.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn round_trip(image: &[u8], printed: &str, stores: &[u32]) {
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("fixture.elf");
    std::fs::write(&binary, image).unwrap();
    let list = |values: &mut dyn Iterator<Item = u32>| {
        values
            .map(|v| format!("{v}u"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let probes = || (0..48).chain([0x7f, 0x80, 0xff, 0x100, 0xffff_fffb, 0xffff_ffff]);
    let src = scratch.path().join("round_trip.c");
    std::fs::write(
        &src,
        format!(
            r#"#define _GNU_SOURCE
#include <fcntl.h>
#include <stdbool.h>
#include <stdio.h>
#include <sys/mman.h>
#include <unistd.h>
typedef int int4;
typedef unsigned int uint4;
typedef unsigned char uint1;
typedef unsigned short uint2;
typedef unsigned long uint8;
typedef struct ModeStore {{ unsigned int mode; }} ModeStore;
{printed}
static const unsigned modes[] = {{{modes}}};
static const unsigned stores[] = {{{stores}}};
int main(int argc, char **argv) {{
  int fd = open(argv[1], O_RDONLY);
  off_t size = lseek(fd, 0, SEEK_END);
  if (mmap((void *)0x400000, size, PROT_READ | PROT_EXEC, MAP_PRIVATE | MAP_FIXED_NOREPLACE, fd, 0) != (void *)0x400000)
    return 2;
  int (__attribute__((ms_abi)) *native)(ModeStore *, unsigned) = (void *)0x401000;
  int bad = 0;
  alarm(20);
  for (unsigned i = 0; i < sizeof modes / sizeof *modes; i++)
    for (unsigned j = 0; j < sizeof stores / sizeof *stores; j++) {{
      ModeStore a = {{stores[j]}}, b = {{stores[j]}};
      int want = native(&a, modes[i]), got = compact_dispatch(&b, modes[i]);
      if (want != got) {{
        printf("mode=%u stored=%u native=%d printed=%d\n", modes[i], stores[j], want, got);
        bad = 1;
      }}
    }}
  return bad;
}}
"#,
            modes = list(&mut probes()),
            stores = list(&mut stores.iter().copied()),
        ),
    )
    .unwrap();
    let exe = scratch.path().join("round_trip");
    for cc in compilers() {
        for level in ["-O0", "-O2"] {
            let compile = Command::new(cc)
                .args(["-w", "-fPIE", "-pie", level])
                .args(common::CC_GCC15_DEMOTE)
                .arg(&src)
                .arg("-o")
                .arg(&exe)
                .output()
                .unwrap();
            assert!(
                compile.status.success(),
                "{cc} {level}: {}\n{printed}",
                String::from_utf8_lossy(&compile.stderr)
            );
            let run = Command::new(&exe).arg(&binary).output().unwrap();
            assert!(
                run.status.success(),
                "{cc} {level}: {} {}\n{printed}",
                run.status,
                String::from_utf8_lossy(&run.stdout)
            );
        }
    }
}

/// The stored modes for which the fixture's retry backedge terminates: every
/// value except those that re-dispatch to the retry target itself.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn terminating_stores(map: &[u8], targets: &[u32], retry: u32, retry_bound: usize) -> Vec<u32> {
    (0..48u32)
        .chain([0x7f, 0x80, 0xff, 0x100, 0xffff_fffb, 0xffff_ffff])
        .filter(|&stored| {
            let index = stored.wrapping_sub(5) as usize;
            stored == 8 || index > retry_bound || targets[map[index] as usize] != retry
        })
        .collect()
}

fn checked(map: &[u8], targets: &[u32], retry_bound: usize, guarded: bool) -> String {
    let image = compact_fixture(map, targets, retry_bound);
    let c = run(&image, 0x401055, guarded);
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    round_trip(&image, &c, &terminating_stores(map, targets, 0x1038, retry_bound));
    c
}

fn checked_absolute(map: &[u8], targets: &[u32], retry_bound: usize) -> String {
    let image = absolute_fixture(map, targets, retry_bound);
    let c = run(&image, 0x401047, true);
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    round_trip(&image, &c, &terminating_stores(map, targets, 0x40102e, retry_bound));
    c
}

/// Index 2 reads map byte 1 (return 240); index 3 reads map byte 2 (retry).
#[test]
fn permuted_byte_map_keeps_original_index_labels() {
    let c = checked(&[0, 3, 1, 2], &[0x102c, 0x1032, 0x1038, 0x104f], 3, true);
    assert!(
        c.contains("switch(mode)"),
        "selector is in a different domain: {c}"
    );
    assert!(
        c.contains("case 2:\n        return 0xf0;"),
        "lost index2/240: {c}"
    );
    assert!(
        !c.contains("switch(*(char *)"),
        "kept mapped byte with raw-index labels: {c}"
    );
}

#[test]
fn compressed_map_with_duplicate_default_targets_keeps_all_index_labels() {
    let c = checked(
        &[
            0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 1, 4, 4, 0, 4, 4, 4, 2, 2, 4, 0, 4, 4, 4, 4, 4, 4, 3,
            4, 4, 4, 4, 4, 4, 4, 0,
        ],
        &[0x102c, 0x1032, 0x1038, 0x104f, 0x104f],
        36,
        true,
    );
    assert!(
        c.contains("switch(mode)"),
        "selector is in a different domain: {c}"
    );
    assert!(c.contains("case 0xb:"), "lost index11/240: {c}");
    assert!(
        c.contains("case 0x12:") && c.contains("case 0x13:"),
        "lost retry cases: {c}"
    );
    assert!(
        c.contains("case 0xe:") && c.contains("case 0x24:"),
        "lost duplicate flight cases: {c}"
    );
}

#[test]
fn missing_guard_recovery_retains_raw_target_labels() {
    let c = checked(&[0, 3, 1, 2], &[0x102c, 0x1032, 0x1038, 0x104f], 3, false);
    assert!(
        c.contains("case 0x401032:"),
        "lost raw return240 target: {c}"
    );
    assert!(c.contains("case 0x40104f:"), "lost raw default target: {c}");
    assert!(
        c.contains("if (4 <= mode)"),
        "folded an unverified range guard: {c}"
    );
    assert!(
        !c.contains("case 2:"),
        "reused original-index labels on a raw target: {c}"
    );
}

/// The retry backedge re-dispatches only index 0 (`store->mode == 5`), so the
/// loop must reset the index to 0 before it dispatches again.  A late model that
/// cannot label the table's rows leaves the cases labelled by address instead
/// of reading that constant as a default value and losing the reset.
#[test]
fn constant_retry_index_survives_a_rejected_late_model() {
    let c = checked(&[0, 1, 1, 1, 1], &[0x1032, 0x1038, 0x102c, 0x104f], 0, true);
    assert!(c.contains("mode = 0"), "lost the retry index reset: {c}");
    assert!(
        c.contains("case 0x401032:\n          return 0xf0;"),
        "lost the raw return240 target: {c}"
    );
    assert!(
        c.contains("cases are labelled by address"),
        "dropped model without a warning: {c}"
    );
}

/// Index 0 reads map byte 4, whose row repeats row 0's target, so the map
/// byte agrees with the index labels on every row; map byte 4 itself has no
/// index label.
#[test]
fn repeated_table_row_does_not_label_a_map_byte() {
    let c = checked(&[4, 1, 2, 3], &[0x102c, 0x1032, 0x1038, 0x104f, 0x102c], 3, true);
    assert_index_selector(&c);
}

/// The same repeated row through GCC's absolute byte map and 8-byte table.
#[test]
fn repeated_absolute_table_row_does_not_label_a_map_byte() {
    let c = checked_absolute(
        &[4, 1, 2, 3],
        &[0x401022, 0x401028, 0x40102e, 0x401041, 0x401022],
        3,
    );
    assert_index_selector(&c);
}

fn assert_index_selector(c: &str) {
    assert!(
        c.contains("switch(mode)"),
        "selector is in a different domain: {c}"
    );
    assert!(
        c.contains("case 0:\n        return 300;"),
        "lost index0/300: {c}"
    );
    assert!(
        !c.contains("switch(*(char *)"),
        "kept mapped byte with raw-index labels: {c}"
    );
}

/// The index reads past the map only where its guard was lost late; those
/// map entries send it to table rows that hold row targets, but no bounded
/// index reads them.
#[test]
fn byte_index_past_a_short_map_keeps_index_labels() {
    let image = short_map_fixture(&[6, 6, 6, 6, 4, 4, 4, 4, 6, 6, 6, 6, 4, 4, 4, 4]);
    let c = run(&image, 0x401064, true);
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        let stores: Vec<u32> = (0..48u32)
            .chain([0x7f, 0x80, 0xff, 0x100, 0xffff_fffb, 0xffff_ffff])
            .filter(|&stored| stored != 10)
            .collect();
        round_trip(&image, &c, &stores);
    }
    assert!(
        c.contains("switch(mode & 0xff)"),
        "selector is in a different domain: {c}"
    );
    assert!(
        c.contains("case 9:\n      return 0x640;"),
        "lost index9/1600: {c}"
    );
    assert!(
        !c.contains("labelled by address") && !c.contains("case 0x40"),
        "labelled cases by address: {c}"
    );
}

/// A fixture built from C whose `pick` the round trip compares with its
/// source: where `pick` starts, the address the file maps at, where its
/// `map` lies, and how many modes to compare.
struct SourceFixture {
    name: &'static str,
    pick: &'static str,
    base: u32,
    map: (u32, usize),
    modes: u32,
}

const MIPS_CUT_ROW: SourceFixture = SourceFixture {
    name: "switch_cutrow_mipsel",
    pick: "0x400110",
    base: 0x400000,
    map: (0x4001c4, 12),
    modes: 12,
};

/// `pick` bounds `mode` to 0..5 and switches on `map[mode] - 9`.  Mode 3's map
/// value falls outside the table, so the flow-time rows stop at mode 2, yet
/// modes 4 and 5 still dispatch to mode 0's target.
#[test]
fn cut_row_index_is_still_dispatched() {
    cut_row(&MIPS_CUT_ROW);
}

/// As above, but modes 4 and 5 read a table entry that no kept row read.
#[test]
fn cut_row_reading_an_unread_entry_is_still_dispatched() {
    cut_row(&SourceFixture { name: "switch_cutrow_unread_mipsel", ..MIPS_CUT_ROW });
}

/// The same bounded `map[mode] - 9` switch, where `map[mode] - 9 < 9` sends
/// modes 3, 4 and 5 away before the table.  Modes 4 and 5 would read the
/// next switch's table, whose first entry is mode 2's target, so counting
/// them as dispatched would reject the correct index labels.
#[test]
fn cut_row_a_later_guard_sends_away_keeps_index_labels() {
    let fixture = SourceFixture {
        name: "switch_cutrow_guarded_x86_64",
        pick: "0x43410a",
        base: 0x433000,
        map: (0x435078, 12),
        modes: 40,
    };
    let c = source_decompile(&fixture);
    assert_eq!(c.matches("switch(a0)").count(), 2, "lost the index selector: {c}");
    assert!(!c.contains("labelled by address"), "labelled cases by address: {c}");
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    source_round_trip(&fixture, &c);
}

/// `pick` bounds `mode` to 0..6 and switches on `map[mode] - 9`.  Mode 5's map
/// value falls outside the table, so the flow-time rows stop at mode 4, yet
/// mode 6 still dispatches, to the default block that modes 0 and 4 reach.  A
/// value printed under `default:` does not contradict the index labels.
#[test]
fn cut_row_reaching_the_default_keeps_index_labels() {
    let fixture = SourceFixture {
        name: "switch_cutrow_default_x86_64",
        pick: "0x40410e",
        base: 0x403000,
        map: (0x405048, 7),
        modes: 40,
    };
    let c = source_decompile(&fixture);
    assert!(c.contains("switch(a0)"), "lost the index selector: {c}");
    assert!(!c.contains("labelled by address"), "labelled cases by address: {c}");
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    source_round_trip(&fixture, &c);
}

/// `pick` bounds `mode` to 0..4 and switches on `map[mode]` through a 5-entry
/// table.  Mode 2's map value falls outside the table, so the flow-time rows
/// stop at mode 1, while mode 3 still dispatches to mode 1's labelled case.
/// The index labels would leave mode 3 out, so the cases are labelled by
/// address, and the table, whose first entry's bytes spell `7A@`, must print
/// as the table rather than as a string literal.
#[test]
fn cut_row_reaching_a_labelled_case_falls_back_to_the_table() {
    let fixture = SourceFixture {
        name: "switch_cutrow_labelled_x86_64",
        pick: "0x40410e",
        base: 0x403000,
        map: (0x405030, 20),
        modes: 40,
    };
    let c = source_decompile(&fixture);
    assert!(c.contains("labelled by address"), "kept index labels that miss mode 3: {c}");
    assert!(!c.contains('"'), "printed the table as a string literal: {c}");
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    source_round_trip(&fixture, &c);
}

/// `pick` switches on `map[mode]`, whose modes 0 and 5 both reach the table
/// entry 0x4001a8, and every case body ends in the shared epilogue at
/// 0x40017c.  Mode 1's map value is sent away by the table guard, so the
/// table is built over the map value.  The case body, once it takes in a copy
/// of the epilogue, starts at 0x40017c, which must never label a case.
#[test]
fn address_labels_are_the_table_values_not_the_block_starts() {
    let fixture = SourceFixture {
        name: "switch_shared_epilogue_mipsel",
        pick: "0x400124",
        base: 0x400000,
        map: (0x400214, 7),
        modes: 40,
    };
    let c = source_decompile(&fixture);
    assert!(c.contains("switch("), "lost the switch: {c}");
    assert!(!c.contains("case 0x40017c:"), "labelled a case by its block start: {c}");
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    source_round_trip(&fixture, &c);
}

/// The `switch_cutrow_labelled_x86_64` switch, assembled with a `nop` at the
/// start of the two dispatched cases.  A `nop` has no p-code, so each case's
/// block starts one byte past its table value, and the address labels must
/// still be the table values 0x404153 and 0x404162.
#[test]
fn address_labels_hold_at_a_case_starting_with_a_nop() {
    let fixture = SourceFixture {
        name: "switch_nop_case_x86_64",
        pick: "0x40410e",
        base: 0x403000,
        map: (0x405030, 20),
        modes: 40,
    };
    let c = source_decompile(&fixture);
    assert!(c.contains("labelled by address"), "kept index labels that miss mode 3: {c}");
    for label in ["case 0x404153:", "case 0x404162:"] {
        assert!(c.contains(label), "lost the table value {label}: {c}");
    }
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    source_round_trip(&fixture, &c);
}

/// The `switch_shared_epilogue_mipsel` code with map `{7, 0, 5, 19, 10, 1,
/// 0}`, assembled with a `nop` at the start of the case at the table value
/// 0x4001a8, which modes 0 and 5 reach.  Mode 3's map value cuts the table,
/// and modes 5 and 6 reach only cases earlier modes reach, so the switch stays
/// over `mode` and its cases are labelled by address: the label must be
/// 0x4001a8, not the shared epilogue's 0x40017c.
#[test]
fn address_labels_hold_at_a_mips_case_starting_with_a_nop() {
    let fixture = SourceFixture {
        name: "switch_nop_case_mipsel",
        pick: "0x400124",
        base: 0x400000,
        map: (0x400214, 7),
        modes: 40,
    };
    let c = source_decompile(&fixture);
    assert!(c.contains("labelled by address"), "kept index labels that miss modes 5 and 6: {c}");
    assert!(c.contains("case 0x4001a8:"), "lost the table value 0x4001a8: {c}");
    assert!(!c.contains("case 0x40017c:"), "labelled a case by its block start: {c}");
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    source_round_trip(&fixture, &c);
}

/// `pick` bounds `mode` to 0..4, and a second guard bounds `map[mode] - 8`
/// to the 7-entry table.  Mode 1's map value is sent away by that guard, so
/// a table over `mode` stops at mode 1 and loses the cases modes 2 and 3
/// reach: the table is built over the map value instead.
#[test]
fn map_value_a_guard_sends_away_keeps_later_rows() {
    let fixture = SourceFixture {
        name: "switch_maprow_mipsel",
        pick: "0x400110",
        base: 0x400000,
        map: (0x400200, 10),
        modes: 40,
    };
    let c = source_decompile(&fixture);
    assert!(!c.contains("labelled by address"), "labelled cases by address: {c}");
    for body in ["return a0 * 5 + 0x3ef;", "return a0 * 3 + 1000;"] {
        assert!(c.contains(body), "lost the case body {body}: {c}");
    }
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    source_round_trip(&fixture, &c);
}

/// GCC checks `map[mode]` in memory and reloads it to index the table, so
/// the check bounds the reloaded value only through an equal address.  Modes
/// 3 and 5 are sent away by it, and mode 6 reaches a case no earlier mode
/// does.
#[test]
fn reloaded_map_value_bounds_the_table() {
    let fixture = SourceFixture {
        name: "switch_maprow_reload_x86_64",
        pick: "0x401000",
        base: 0x400000,
        map: (0x402058, 14),
        modes: 40,
    };
    let c = source_decompile(&fixture);
    assert!(!c.contains("labelled by address"), "labelled cases by address: {c}");
    assert!(c.contains("case 10:"), "lost the case mode 6 reaches: {c}");
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    source_round_trip(&fixture, &c);
}

/// Mode 1's map value is sent away by the table guard, and its row would read
/// far outside the image, so a table over `mode` cannot be built at all.
#[test]
fn map_value_reading_outside_the_image_keeps_the_switch() {
    let fixture = SourceFixture {
        name: "switch_maprow_far_x86_64",
        pick: "0x401000",
        base: 0x400000,
        map: (0x402048, 12),
        modes: 40,
    };
    let c = source_decompile(&fixture);
    assert!(c.contains("switch("), "lost the switch: {c}");
    assert!(!c.contains("jump-as-call"), "printed the dispatch as a call: {c}");
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    source_round_trip(&fixture, &c);
}

/// `pick` reads `map[mode]` from `{240, 1, 241, 241, 241}`, and a second
/// guard sends mode 1's value away from the table.  A switch over the byte
/// map value would print labels such as 0xf0, which a signed `char` never
/// matches, so the switch stays over `mode`.
#[test]
fn map_value_at_the_sign_bit_keeps_the_index_switch() {
    let fixture = SourceFixture {
        name: "switch_maprow_signbit_x86_64",
        pick: "0x401000",
        base: 0x400000,
        map: (0x402038, 5),
        modes: 40,
    };
    let c = source_decompile(&fixture);
    assert!(c.contains("switch(a0)"), "moved the switch off the index: {c}");
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    source_round_trip(&fixture, &c);
}

/// GCC's shape for a byte map `{208, 204, 203, 213, 217, 206}`: mode 4's
/// value is sent away and cuts the table, which still loses mode 5's row.
/// The switch stays over `mode`, so modes 0 to 4 keep their cases rather than
/// going through labels a signed `char` never matches.
#[test]
fn reloaded_map_value_at_the_sign_bit_keeps_the_index_switch() {
    let fixture = SourceFixture {
        name: "switch_maprow_signbit_gcc_x86_64",
        pick: "0x401000",
        base: 0x400000,
        map: (0x402070, 6),
        modes: 5,
    };
    let c = source_decompile(&fixture);
    assert!(c.contains("switch(a0)"), "moved the switch off the index: {c}");
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    source_round_trip(&fixture, &c);
}

fn cut_row(fixture: &SourceFixture) {
    let c = source_decompile(fixture);
    assert!(c.contains("switch("), "lost the switch: {c}");
    assert!(
        !c.contains("switch(a0)") || (c.contains("case 4:") && c.contains("case 5:")),
        "labelled the mode switch without modes 4 and 5: {c}"
    );
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    source_round_trip(fixture, &c);
}

fn fixtures() -> PathBuf {
    root().join("decompiler/crates/kuna-analysis/tests/fixtures")
}

fn source_decompile(fixture: &SourceFixture) -> String {
    let mut command = kuna();
    command
        .arg("decompile-all")
        .arg(fixtures().join(fixture.name))
        .args(["--addr", fixture.pick, "--sleighpath"])
        .arg(specs());
    printed(command)
}

/// Compile the printed `pick` for the host with every available compiler at
/// -O0 and -O2, map the fixture's file at its base address for the data it
/// reads, and compare it with the fixture source's `pick` for each mode.  A
/// printed store to `sink` through `gp`, which a MIPS fixture never sets, is
/// dropped.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn source_round_trip(fixture: &SourceFixture, printed: &str) {
    let binary = fixtures().join(fixture.name);
    let source = fixtures().join(format!("{}.c", fixture.name));
    let mut globals: Vec<&str> = printed
        .match_indices("dat_")
        .filter(|&(at, _)| {
            !printed[..at].ends_with(|ch: char| ch.is_ascii_alphanumeric() || ch == '_')
        })
        .map(|(at, _)| {
            let end = printed[at + 4..]
                .find(|ch: char| !ch.is_ascii_hexdigit())
                .map_or(printed.len(), |len| at + 4 + len);
            &printed[at..end]
        })
        .filter(|name| name.len() > 4)
        .collect();
    globals.sort_unstable();
    globals.dedup();
    let globals: String = globals
        .iter()
        .map(|name| format!("#define {name} (*(unsigned int (*)[64])0x{})\n", &name[4..]))
        .collect();
    let function: String = printed
        .lines()
        .filter(|line| !line.contains(" + -0x7ff0) = "))
        .map(|line| format!("{}\n", line.replace("pick(", "printed_pick(")))
        .collect();
    let scratch = tempfile::tempdir().unwrap();
    let src = scratch.path().join("source_round_trip.c");
    std::fs::write(
        &src,
        format!(
            r#"#define _GNU_SOURCE
#include <fcntl.h>
#include <stdio.h>
#include <sys/mman.h>
#include <unistd.h>
#define pick native_pick
#define map native_map
#include "{source}"
#undef pick
#undef map
#define map (*(unsigned char (*)[{map_len}]){map_at:#x})
{globals}{function}
int main(int argc, char **argv) {{
  int fd = open(argv[1], O_RDONLY);
  off_t size = lseek(fd, 0, SEEK_END);
  if (mmap((void *){base:#x}, size, PROT_READ, MAP_PRIVATE | MAP_FIXED_NOREPLACE, fd, 0) != (void *){base:#x})
    return 2;
  int bad = 0;
  for (unsigned mode = 0; mode < {modes}; mode++) {{
    int want = native_pick(mode), got = printed_pick(mode);
    if (want != got) {{
      printf("mode=%u native=%d printed=%d\n", mode, want, got);
      bad = 1;
    }}
  }}
  return bad;
}}
"#,
            source = source.display(),
            base = fixture.base,
            map_at = fixture.map.0,
            map_len = fixture.map.1,
            modes = fixture.modes,
        ),
    )
    .unwrap();
    let exe = scratch.path().join("source_round_trip");
    for cc in compilers() {
        for level in ["-O0", "-O2"] {
            let compile = Command::new(cc)
                .args(["-w", "-fPIE", "-pie", level])
                .args(common::CC_GCC15_DEMOTE)
                .arg(&src)
                .arg("-o")
                .arg(&exe)
                .output()
                .unwrap();
            assert!(
                compile.status.success(),
                "{cc} {level}: {}\n{printed}",
                String::from_utf8_lossy(&compile.stderr)
            );
            let run = Command::new(&exe).arg(&binary).output().unwrap();
            assert!(
                run.status.success(),
                "{cc} {level}: {} {}\n{printed}",
                run.status,
                String::from_utf8_lossy(&run.stdout)
            );
        }
    }
}

#[test]
fn wider_late_selector_range_keeps_index_labels() {
    wider_late_selector_range(&[]);
}

/// Entry 5 lies past the far entry that ends the table and repeats row 0's
/// target; the guard keeps `k` from ever reading it.
#[test]
fn entry_past_the_table_end_keeps_index_labels() {
    wider_late_selector_range(&[-0x32]);
}

fn wider_late_selector_range(past: &[i32]) {
    let c = decompile(
        &far_guard_fixture(past),
        &[
            "function 0x401000-0x40105a=pick",
            "prototype pick int pick(unsigned char *p, int a, int b)",
        ],
        &[],
    );
    assert!(
        c.contains("case 0:\n        return 100;") && c.contains("case 3:\n        return 0x67;"),
        "lost index labels: {c}"
    );
    assert!(!c.contains("case 0x40"), "labelled cases by address: {c}");
}

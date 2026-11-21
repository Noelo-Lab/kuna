//! The zip reader against `unzip` and against malformed archives.

use std::path::Path;

use kuna_jvm::zip_reader::{inflate, ZipReader, DEFLATED, STORED};

use crate::f6_support::{data, run, skip, tmp_dir, tool, vineflower_jar};

struct Listed {
    size: u64,
    csize: u64,
    method: String,
    name: String,
}

fn unzip_list(unzip: &Path, archive: &Path) -> Vec<Listed> {
    let out = run(unzip, &["-Zl", &archive.to_string_lossy()]);
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    let mut listed = Vec::new();
    for line in text.lines() {
        let mut rest = line;
        let mut fields = Vec::new();
        for _ in 0..9 {
            rest = rest.trim_start();
            let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
            fields.push(&rest[..end]);
            rest = &rest[end..];
        }
        let name = rest.trim_start();
        let is_entry = fields.len() == 9
            && fields[1].contains('.')
            && fields[3].parse::<u64>().is_ok()
            && fields[5].parse::<u64>().is_ok()
            && !name.is_empty();
        if is_entry {
            listed.push(Listed {
                size: fields[3].parse().unwrap(),
                csize: fields[5].parse().unwrap(),
                method: fields[6].to_string(),
                name: name.to_string(),
            });
        }
    }
    listed
}

/// Entry list (order, names, sizes, methods) and every entry's bytes against `unzip`.
fn compare_with_unzip(unzip: &Path, archive: &Path) -> (usize, usize) {
    let zip = ZipReader::open(archive).unwrap_or_else(|e| panic!("{}: {e}", archive.display()));
    let listed = unzip_list(unzip, archive);
    assert_eq!(zip.len(), listed.len(), "{}", archive.display());
    for (mine, theirs) in zip.entries().iter().zip(&listed) {
        assert_eq!(mine.name, theirs.name, "{}", archive.display());
        assert_eq!(
            mine.size,
            theirs.size,
            "{}: {}",
            archive.display(),
            mine.name
        );
        assert_eq!(mine.compressed_size, theirs.csize, "{}", mine.name);
        let method = if theirs.method == "stor" {
            STORED
        } else {
            DEFLATED
        };
        assert_eq!(mine.method, method, "{}", mine.name);
    }
    let all = run(unzip, &["-p", &archive.to_string_lossy()]).stdout;
    let mut mine = Vec::new();
    for entry in zip.entries().iter().filter(|e| !e.is_directory()) {
        mine.extend(zip.read_entry(entry).unwrap());
    }
    assert!(mine == all, "{}: unzip -p bytes differ", archive.display());
    (zip.len(), all.len())
}

const FIXTURES: [&str; 11] = [
    "mixed.jar",
    "mr.jar",
    "mrtool.jar",
    "badver.jar",
    "dup.zip",
    "prefixed.zip",
    "trailing.zip",
    "comment.zip",
    "zip64.zip",
    "inner.jar",
    "tiny.jmod",
];

#[test]
fn fixtures_match_unzip() {
    let Some(unzip) = tool(None, "unzip") else {
        return skip("fixtures_match_unzip", "no unzip");
    };
    let (mut entries, mut bytes) = (0, 0);
    for name in FIXTURES {
        let (e, b) = compare_with_unzip(&unzip, &data(name));
        entries += e;
        bytes += b;
    }
    eprintln!("zip fixtures: {entries} entries, {bytes} bytes identical to unzip -p");
}

#[test]
fn vineflower_jar_matches_unzip() {
    let (Some(unzip), Some(jar)) = (tool(None, "unzip"), vineflower_jar()) else {
        return skip("vineflower_jar_matches_unzip", "no unzip or vineflower jar");
    };
    let (e, b) = compare_with_unzip(&unzip, &jar);
    eprintln!("vineflower-1.12.0.jar: {e} entries, {b} bytes identical to unzip -p");
}

#[test]
fn infozip_archive_with_stored_and_deflated_entries_matches_unzip() {
    let (Some(unzip), Some(zip)) = (tool(None, "unzip"), tool(None, "zip")) else {
        return skip("infozip_archive", "no zip/unzip");
    };
    let out = tmp_dir("infozip").join("tree.zip");
    let status = std::process::Command::new(&zip)
        .current_dir(data("tree"))
        .args([
            "-q",
            "-r",
            "-FS",
            "-n",
            ".class",
            &out.to_string_lossy(),
            ".",
        ])
        .status()
        .expect("zip");
    assert!(status.success());
    let reader = ZipReader::open(&out).unwrap();
    let methods: Vec<u16> = reader.entries().iter().map(|e| e.method).collect();
    assert!(methods.contains(&STORED) && methods.contains(&DEFLATED));
    compare_with_unzip(&unzip, &out);
}

#[test]
fn get_entry_follows_zipfile_lookup() {
    let dup = ZipReader::open(&data("dup.zip")).unwrap();
    let names: Vec<&str> = dup.entries().iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, ["x.txt", "y/", "x.txt", "y"]);
    assert_eq!(
        dup.read("x.txt").unwrap().unwrap(),
        b"second, deflated\n".to_vec()
    );
    assert_eq!(dup.get_entry("y").unwrap().name, "y");
    assert_eq!(dup.get_entry("y/").unwrap().name, "y/");

    let mixed = ZipReader::open(&data("mixed.jar")).unwrap();
    assert_eq!(mixed.get_entry("pkg").unwrap().name, "pkg/");
    assert!(mixed.get_entry("res").is_none());
    assert!(mixed.get_entry_exact("pkg").is_none());
    assert_eq!(
        mixed.read("res/empty.bin").unwrap().unwrap(),
        Vec::<u8>::new()
    );
    assert!(mixed.read("missing").unwrap().is_none());
    assert!(mixed.get_entry("pkg/").unwrap().is_directory());
    mixed.close();
    assert!(mixed.is_closed());
    assert!(mixed.read("res/data.txt").is_err());
}

#[test]
fn prefixes_descriptors_and_zip64() {
    let jmod = ZipReader::open(&data("tiny.jmod")).unwrap();
    assert_eq!(jmod.prefix_len(), 4);
    assert!(jmod.entries().iter().all(|e| e.flags & 8 != 0));
    assert!(jmod.read("classes/module-info.class").unwrap().is_some());

    let prefixed = ZipReader::open(&data("prefixed.zip")).unwrap();
    assert_eq!(prefixed.prefix_len(), 43);
    assert_eq!(
        prefixed.read("a.txt").unwrap().unwrap(),
        b"after the stub\n"
    );

    let zip64 = ZipReader::open(&data("zip64.zip")).unwrap();
    let sizes: Vec<(u64, u64)> = zip64
        .entries()
        .iter()
        .map(|e| (e.size, e.compressed_size))
        .collect();
    assert_eq!(sizes[0], (12, 12));
    assert_eq!(sizes[1].0, 270);
    assert_eq!(
        zip64.read("b/c.txt").unwrap().unwrap(),
        b"deflated ".repeat(30)
    );

    let trailing = ZipReader::open(&data("trailing.zip")).unwrap();
    assert_eq!(
        trailing.read("t.txt").unwrap().unwrap(),
        b"trailing junk follows\n"
    );
}

struct XorShift(u64);

impl XorShift {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

fn exercise(path: &Path) -> bool {
    match ZipReader::open(path) {
        Ok(zip) => {
            for entry in zip.entries() {
                let _ = zip.read_entry(entry);
                let _ = zip.read(&entry.name);
            }
            true
        }
        Err(_) => false,
    }
}

#[test]
fn malformed_archives_fail_without_panicking() {
    let dir = tmp_dir("garbage");
    let scratch = dir.join("case.zip");
    let write = |bytes: &[u8]| std::fs::write(&scratch, bytes).unwrap();

    write(&[]);
    let err = ZipReader::open(&scratch).unwrap_err();
    assert!(err.to_string().contains("empty"), "{err}");

    let mut rng = XorShift(0x9E37_79B9_7F4A_7C15);
    for len in [1usize, 21, 22, 100, 4096, 70_000] {
        let mut bytes: Vec<u8> = (0..len).map(|_| rng.next() as u8).collect();
        write(&bytes);
        exercise(&scratch);
        if len >= 22 {
            let at = len - 22;
            bytes[at..at + 4].copy_from_slice(&0x0605_4b50u32.to_le_bytes());
            write(&bytes);
            exercise(&scratch);
        }
    }

    let original = std::fs::read(data("mixed.jar")).unwrap();
    let mut opened = 0;
    for cut in 0..original.len() {
        write(&original[..cut]);
        if exercise(&scratch) {
            let zip = ZipReader::open(&scratch).unwrap();
            let names: Vec<&str> = zip.entries().iter().map(|e| e.name.as_str()).collect();
            assert_eq!(names, ["q/Inner.txt"], "cut at {cut}");
            opened += 1;
        }
    }
    eprintln!("{opened} truncations of mixed.jar open as the stored inner.jar, as with ZipFile");
    for at in 0..original.len() {
        let mut bytes = original.clone();
        bytes[at] ^= 0x5A;
        write(&bytes);
        exercise(&scratch);
    }

    let end = original.len() - 22;
    let cen = u32::from_le_bytes(original[end + 16..end + 20].try_into().unwrap()) as usize;
    assert_eq!(original[cen..cen + 4], 0x0201_4b50u32.to_le_bytes());
    let mut huge = original.clone();
    huge[cen + 20..cen + 24].copy_from_slice(&0xFFFF_FFF0u32.to_le_bytes());
    write(&huge);
    let zip = ZipReader::open(&scratch).unwrap();
    assert!(zip.read_entry(&zip.entries()[0]).is_err());

    let mut encrypted = original.clone();
    encrypted[cen + 8] |= 1;
    write(&encrypted);
    assert!(ZipReader::open(&scratch)
        .unwrap_err()
        .to_string()
        .contains("encrypted"));

    let mut bzip2 = original;
    bzip2[cen + 10..cen + 12].copy_from_slice(&12u16.to_le_bytes());
    write(&bzip2);
    assert!(ZipReader::open(&scratch)
        .unwrap_err()
        .to_string()
        .contains("compression method"));
}

#[test]
fn inflate_bounds_its_output() {
    let zip = ZipReader::open(&data("zip64.zip")).unwrap();
    let entry = zip.entries()[1].clone();
    let mut lying = entry.clone();
    lying.size = 10;
    assert!(zip.read_entry(&lying).is_err());

    let data = b"a stream that is much longer than ten bytes".repeat(4);
    let mut z = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::best());
    std::io::Write::write_all(&mut z, &data).unwrap();
    let packed = z.finish().unwrap();
    assert_eq!(
        inflate(&packed, true, data.len() as u64, false).unwrap(),
        data
    );
    assert_eq!(
        inflate(&packed, true, 10, true).unwrap(),
        data[..10].to_vec()
    );
    assert!(inflate(&packed, true, 10, false).is_err());
    assert!(inflate(&packed[..packed.len() / 2], true, data.len() as u64, false).is_err());
}

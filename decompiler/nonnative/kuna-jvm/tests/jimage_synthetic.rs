//! A hand-built jimage (both byte orders) exercising the reader without a JDK:
//! the perfect-hash redirect (seeded and direct buckets), directory nodes, and
//! the `zip`, `compact-cp`, stacked and unknown compression plugins.

use std::collections::BTreeMap;
use std::path::PathBuf;

use flate2::write::ZlibEncoder;
use kuna_jvm::context_source::IContextSource;
use kuna_jvm::jdk_runtime::{JImage, JavaRuntimeContextSource};
use kuna_jvm::rt::try_catch;
use kuna_jvm::zip_reader::ZipReader;

use crate::f6_support::{data, tmp_dir};

fn hash(name: &str, seed: i32) -> i32 {
    let mut h = seed;
    for b in name.bytes() {
        h = h.wrapping_mul(0x0100_0193) ^ b as i32;
    }
    h & 0x7FFF_FFFF
}

struct Writer {
    big_endian: bool,
    strings: Vec<u8>,
    string_offsets: BTreeMap<String, u32>,
    locations: Vec<u8>,
    named: Vec<(String, u32)>,
    resources: Vec<u8>,
}

impl Writer {
    fn new(big_endian: bool) -> Writer {
        let mut w = Writer {
            big_endian,
            strings: Vec::new(),
            string_offsets: BTreeMap::new(),
            locations: Vec::new(),
            named: Vec::new(),
            resources: Vec::new(),
        };
        w.string("");
        w
    }

    fn u32(&self, v: u32) -> [u8; 4] {
        if self.big_endian {
            v.to_be_bytes()
        } else {
            v.to_le_bytes()
        }
    }

    fn string(&mut self, s: &str) -> u32 {
        if let Some(&o) = self.string_offsets.get(s) {
            return o;
        }
        let o = self.strings.len() as u32;
        self.strings.extend_from_slice(s.as_bytes());
        self.strings.push(0);
        self.string_offsets.insert(s.to_string(), o);
        o
    }

    /// A location named `/module/base.ext` (or `base` alone when `module` is empty).
    fn location(
        &mut self,
        module: &str,
        base: &str,
        ext: &str,
        content: &[u8],
        uncompressed: Option<usize>,
    ) -> u32 {
        let attrs = [
            (
                1u8,
                if module.is_empty() {
                    0
                } else {
                    self.string(module) as u64
                },
            ),
            (3, self.string(base) as u64),
            (
                4,
                if ext.is_empty() {
                    0
                } else {
                    self.string(ext) as u64
                },
            ),
            (5, self.resources.len() as u64),
            (
                6,
                if uncompressed.is_some() {
                    content.len() as u64
                } else {
                    0
                },
            ),
            (7, uncompressed.unwrap_or(content.len()) as u64),
        ];
        self.resources.extend_from_slice(content);
        let offset = self.locations.len() as u32;
        for (kind, value) in attrs {
            if value == 0 {
                continue;
            }
            let bytes = value.to_be_bytes();
            let skip = bytes.iter().take_while(|&&b| b == 0).count().min(7);
            self.locations.push((kind << 3) | (7 - skip as u8));
            self.locations.extend_from_slice(&bytes[skip..]);
        }
        self.locations.push(0);
        let mut name = String::new();
        if !module.is_empty() {
            name = format!("/{module}/");
        }
        name.push_str(base);
        if !ext.is_empty() {
            name = format!("{name}.{ext}");
        }
        self.named.push((name, offset));
        offset
    }

    fn resource(
        &mut self,
        module: &str,
        path: &str,
        content: &[u8],
        uncompressed: Option<usize>,
    ) -> u32 {
        let (base, ext) = path.rsplit_once('.').unwrap_or((path, ""));
        self.location(module, base, ext, content, uncompressed)
    }

    fn directory(&mut self, path: &str, children: &[u32]) -> u32 {
        let content: Vec<u8> = children.iter().flat_map(|&c| self.u32(c)).collect();
        match path {
            "/modules" => self.location("", "/modules", "", &content, None),
            _ => self.location(
                "modules",
                path.trim_start_matches("/modules/"),
                "",
                &content,
                None,
            ),
        }
    }

    fn compressed(&mut self, plugin: &str, payload: &[u8], uncompressed: usize) -> Vec<u8> {
        let name = self.string(plugin);
        let mut out = self.u32(0xCAFE_FAFA).to_vec();
        let wide = |v: u64| {
            if self.big_endian {
                v.to_be_bytes()
            } else {
                v.to_le_bytes()
            }
        };
        out.extend_from_slice(&wide(payload.len() as u64));
        out.extend_from_slice(&wide(uncompressed as u64));
        out.extend_from_slice(&self.u32(name));
        out.extend_from_slice(&self.u32(u32::MAX));
        out.push(1);
        out.extend_from_slice(payload);
        out
    }

    fn finish(&self) -> (Vec<u8>, bool) {
        let n = self.named.len();
        let mut buckets: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (i, (name, _)) in self.named.iter().enumerate() {
            buckets[hash(name, 0x0100_0193) as usize % n].push(i);
        }
        let mut redirect = vec![0i32; n];
        let mut slots: Vec<Option<usize>> = vec![None; n];
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by_key(|&b| std::cmp::Reverse(buckets[b].len()));
        let mut seeded = false;
        for &b in order.iter().filter(|&&b| buckets[b].len() > 1) {
            'seed: for seed in 1..100_000 {
                let mut taken = Vec::new();
                for &i in &buckets[b] {
                    let s = hash(&self.named[i].0, seed) as usize % n;
                    if slots[s].is_some() || taken.contains(&s) {
                        continue 'seed;
                    }
                    taken.push(s);
                }
                for (&i, &s) in buckets[b].iter().zip(&taken) {
                    slots[s] = Some(i);
                }
                redirect[b] = seed;
                seeded = true;
                break;
            }
        }
        for &b in order.iter().filter(|&&b| buckets[b].len() == 1) {
            let s = slots.iter().position(Option::is_none).unwrap();
            slots[s] = Some(buckets[b][0]);
            redirect[b] = -(s as i32) - 1;
        }
        let mut out = Vec::new();
        for v in [0xCAFE_DADA, 0x0001_0000, 0, n as u32, n as u32] {
            out.extend_from_slice(&self.u32(v));
        }
        out.extend_from_slice(&self.u32(self.locations.len() as u32));
        out.extend_from_slice(&self.u32(self.strings.len() as u32));
        for r in &redirect {
            out.extend_from_slice(&self.u32(*r as u32));
        }
        for s in &slots {
            out.extend_from_slice(&self.u32(self.named[s.unwrap()].1));
        }
        out.extend_from_slice(&self.locations);
        out.extend_from_slice(&self.strings);
        out.extend_from_slice(&self.resources);
        (out, seeded)
    }
}

fn cint(v: u32) -> Vec<u8> {
    match v {
        0..=0x1F => vec![0xA0 | v as u8],
        0x20..=0x1FFF => vec![0xC0 | (v >> 8) as u8, v as u8],
        _ => v.to_be_bytes().to_vec(),
    }
}

fn utf8_entry(s: &str) -> Vec<u8> {
    let mut out = vec![1];
    out.extend_from_slice(&(s.len() as u16).to_be_bytes());
    out.extend_from_slice(s.as_bytes());
    out
}

fn zlib(bytes: &[u8]) -> Vec<u8> {
    let mut z = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    std::io::Write::write_all(&mut z, bytes).unwrap();
    z.finish().unwrap()
}

const MODULE: &str = "f6.tiny";

struct Built {
    path: PathBuf,
    expected: Vec<(&'static str, Vec<u8>)>,
}

fn build(big_endian: bool) -> Built {
    let mut w = Writer::new(big_endian);
    let jmod = ZipReader::open(&data("tiny.jmod")).unwrap();
    let module_info = jmod.read("classes/module-info.class").unwrap().unwrap();
    let plain = b"\xCA\xFE\xBA\xBEplain class".to_vec();
    let zipped = b"zip compressed class body ".repeat(20);

    let shared = w.string("Shared");
    let desc = w.string("(L;)V");
    let pkg = w.string("java/lang");
    let class = w.string("String");
    let mut flow = cint(pkg);
    flow.extend(cint(class));
    let mut transformed = b"\xCA\xFE\xBA\xBE\x00\x00\x00\x34".to_vec();
    transformed.extend_from_slice(&7u16.to_be_bytes());
    transformed.extend(utf8_entry("Plain"));
    transformed.push(23);
    transformed.extend(cint(shared));
    transformed.push(25);
    transformed.extend(cint(desc));
    transformed.extend(cint(flow.len() as u32));
    transformed.extend(&flow);
    transformed.extend([7, 0, 1]);
    transformed.extend([5, 1, 2, 3, 4, 5, 6, 7, 8]);
    transformed.extend(b"TAIL");
    let mut shared_class = b"\xCA\xFE\xBA\xBE\x00\x00\x00\x34".to_vec();
    shared_class.extend_from_slice(&7u16.to_be_bytes());
    shared_class.extend(utf8_entry("Plain"));
    shared_class.extend(utf8_entry("Shared"));
    shared_class.extend(utf8_entry("(Ljava/lang/String;)V"));
    shared_class.extend([7, 0, 1]);
    shared_class.extend([5, 1, 2, 3, 4, 5, 6, 7, 8]);
    shared_class.extend(b"TAIL");

    let mi = w.resource(MODULE, "module-info.class", &module_info, None);
    let a = w.resource(MODULE, "p/A.class", &plain, None);
    let zip_payload = w.compressed("zip", &zlib(&zipped), zipped.len());
    let b = w.resource(MODULE, "p/B.class", &zip_payload, Some(zipped.len()));
    let cp_payload = w.compressed("compact-cp", &transformed, shared_class.len());
    let c = w.resource(MODULE, "p/C.class", &cp_payload, Some(shared_class.len()));
    let stacked = w.compressed("zip", &zlib(&cp_payload), cp_payload.len());
    let d = w.resource(MODULE, "p/D.class", &stacked, Some(shared_class.len()));
    let unknown = w.compressed("lzma", b"????", 4);
    let e = w.resource(MODULE, "p/E.class", &unknown, Some(4));
    let res = w.resource(MODULE, "p/q/res.txt", b"resource\n", None);
    let q = w.directory("/modules/f6.tiny/p/q", &[res]);
    let p = w.directory("/modules/f6.tiny/p", &[a, b, c, d, e, q]);
    let m = w.directory("/modules/f6.tiny", &[mi, p]);
    w.directory("/modules", &[m]);

    let (bytes, seeded) = w.finish();
    assert!(
        seeded,
        "the test image should exercise a seeded hash bucket"
    );
    let home = tmp_dir(if big_endian {
        "synthetic-be"
    } else {
        "synthetic-le"
    });
    std::fs::create_dir_all(home.join("lib")).unwrap();
    let path = home.join("lib/modules");
    std::fs::write(&path, bytes).unwrap();
    Built {
        path,
        expected: vec![
            ("module-info.class", module_info),
            ("p/A.class", plain),
            ("p/B.class", zipped),
            ("p/C.class", shared_class.clone()),
            ("p/D.class", shared_class),
            ("p/q/res.txt", b"resource\n".to_vec()),
        ],
    }
}

#[test]
fn synthetic_images_read_in_both_byte_orders() {
    for big_endian in [false, true] {
        let built = build(big_endian);
        let image = JImage::open(&built.path).unwrap();
        assert_eq!(image.is_big_endian(), big_endian);
        assert_eq!(image.module_names().unwrap(), [MODULE]);
        let walk: Vec<(String, bool)> = image.walk_module(MODULE).unwrap();
        let names: Vec<&str> = walk.iter().map(|(n, _)| n.as_str()).collect();
        assert_eq!(
            names,
            [
                "",
                "module-info.class",
                "p",
                "p/A.class",
                "p/B.class",
                "p/C.class",
                "p/D.class",
                "p/E.class",
                "p/q",
                "p/q/res.txt"
            ]
        );
        for (name, bytes) in &built.expected {
            let read = image.read(&format!("/{MODULE}/{name}")).unwrap().unwrap();
            assert_eq!(&read, bytes, "{name}");
        }
        let err = image.read(&format!("/{MODULE}/p/E.class")).unwrap_err();
        assert!(err.to_string().contains("'lzma'"), "{err}");
        assert!(image.read("/f6.tiny/p/Missing.class").unwrap().is_none());
        assert!(image.read("/modules/f6.tiny/p").unwrap().is_some());

        let home = built.path.parent().unwrap().parent().unwrap();
        let rt = JavaRuntimeContextSource::open(home, "synthetic").unwrap();
        assert!(rt.is_jimage());
        let children = rt.get_entries().child_contexts().to_vec();
        assert_eq!(children.len(), 1);
        let module = &children[0];
        assert_eq!(module.get_name(), "module f6.tiny@1.0");
        let entries = module.get_entries();
        let classes: Vec<&str> = entries.classes().iter().map(|c| c.base_path()).collect();
        assert_eq!(classes, ["module-info", "p/A", "p/B", "p/C", "p/D", "p/E"]);
        assert_eq!(module.get_class_bytes("p/C").unwrap(), built.expected[3].1);
        assert!(try_catch(|| module.get_class_bytes("p/E"))
            .unwrap_err()
            .is("IOException"));
        assert_eq!(
            module.get_input_stream("p/./q/../q/res.txt").unwrap(),
            b"resource\n"
        );
    }
}

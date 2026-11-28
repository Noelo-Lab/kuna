//! Fixture access and a minimal class-file reader for structural checks.

use std::path::PathBuf;

pub fn fixture(name: &str) -> Vec<u8> {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../decompiler/managed/kuna-jvm/fixtures/dex").join(name);
    std::fs::read(&p).unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}

pub struct Method {
    pub name: String,
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub attrs: Vec<String>,
}

pub struct Class {
    pub version: u16,
    pub utf8: Vec<String>,
    /// `(name, descriptor)` of each `NameAndType` constant.
    pub name_types: Vec<(String, String)>,
    /// `(inner, outer, simple name)` class names of the `InnerClasses` entries.
    pub inner: Vec<(String, String, String)>,
    pub methods: Vec<Method>,
    pub attrs: Vec<String>,
}

struct R<'a>(&'a [u8], usize);

impl R<'_> {
    fn u8(&mut self) -> u8 {
        self.1 += 1;
        self.0[self.1 - 1]
    }
    fn u16(&mut self) -> u16 {
        (u16::from(self.u8()) << 8) | u16::from(self.u8())
    }
    fn u32(&mut self) -> u32 {
        (u32::from(self.u16()) << 16) | u32::from(self.u16())
    }
    fn take(&mut self, n: usize) -> &[u8] {
        self.1 += n;
        &self.0[self.1 - n..self.1]
    }
}

pub fn read_class(b: &[u8]) -> Class {
    let mut r = R(b, 0);
    assert_eq!(r.u32(), 0xCAFE_BABE);
    r.u16();
    let version = r.u16();
    let n = r.u16();
    let mut utf8 = vec![String::new(); n as usize];
    let mut class_ref = vec![0u16; n as usize];
    let mut nat = Vec::new();
    let mut i = 1;
    while i < n {
        let tag = r.u8();
        match tag {
            1 => {
                let len = r.u16() as usize;
                utf8[i as usize] = String::from_utf8_lossy(r.take(len)).into_owned();
            }
            12 => nat.push((r.u16(), r.u16())),
            3 | 4 | 9..=11 | 18 => {
                r.take(4);
            }
            5 | 6 => {
                r.take(8);
                i += 1;
            }
            7 => class_ref[i as usize] = r.u16(),
            8 | 16 => {
                r.take(2);
            }
            15 => {
                r.take(3);
            }
            t => panic!("tag {t}"),
        }
        i += 1;
    }
    r.take(6);
    let ni = r.u16();
    r.take(2 * ni as usize);
    let attrs = |r: &mut R, utf8: &[String]| -> Vec<(String, Vec<u8>)> {
        let n = r.u16();
        (0..n)
            .map(|_| {
                let name = utf8[r.u16() as usize].clone();
                let len = r.u32() as usize;
                (name, r.take(len).to_vec())
            })
            .collect()
    };
    let nf = r.u16();
    for _ in 0..nf {
        r.take(6);
        attrs(&mut r, &utf8);
    }
    let nm = r.u16();
    let mut methods = Vec::new();
    for _ in 0..nm {
        r.u16();
        let name = utf8[r.u16() as usize].clone();
        r.u16();
        let mut m = Method {
            name,
            max_stack: 0,
            max_locals: 0,
            code: Vec::new(),
            attrs: Vec::new(),
        };
        for (an, body) in attrs(&mut r, &utf8) {
            if an == "Code" {
                let mut c = R(&body, 0);
                m.max_stack = c.u16();
                m.max_locals = c.u16();
                let len = c.u32() as usize;
                m.code = c.take(len).to_vec();
                let ne = c.u16();
                c.take(8 * ne as usize);
                for (sub, _) in attrs(&mut c, &utf8) {
                    m.attrs.push(sub);
                }
            }
            m.attrs.push(an);
        }
        methods.push(m);
    }
    let class_attrs = attrs(&mut r, &utf8);
    assert_eq!(r.1, b.len(), "trailing bytes");
    let name = |i: u16| if i == 0 { String::new() } else { utf8[class_ref[i as usize] as usize].clone() };
    let mut inner = Vec::new();
    for (an, body) in &class_attrs {
        if an == "InnerClasses" {
            let mut c = R(body, 0);
            for _ in 0..c.u16() {
                let (i, o, nm) = (c.u16(), c.u16(), c.u16());
                c.u16();
                inner.push((name(i), name(o), utf8[nm as usize].clone()));
            }
        }
    }
    let attrs = class_attrs.into_iter().map(|a| a.0).collect();
    let name_types = nat.iter().map(|&(a, b)| (utf8[a as usize].clone(), utf8[b as usize].clone())).collect();
    Class {
        version,
        name_types,
        utf8,
        inner,
        methods,
        attrs,
    }
}

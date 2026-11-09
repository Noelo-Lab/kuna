//! The classes and methods of an input, read from the class files alone
//! (nothing is decompiled): names, nesting, access flags, source files, sizes.

use crate::kuna_input::InputSet;
use crate::rt::jstring::decode_modified_utf8;

/// One class file.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ClassInfo {
    /// Internal name (`a/b/C$D`).
    pub internal: String,
    /// The enclosing class: the `InnerClasses` outer class of a member class,
    /// the `EnclosingMethod` class of a local or anonymous one.
    pub outer: Option<String>,
    /// Access flags of the class file header.
    pub access: u16,
    /// Access flags of the class's own `InnerClasses` entry (as declared in source).
    pub inner_access: Option<u16>,
    pub super_class: Option<String>,
    pub source_file: Option<String>,
    /// Size of the class file in bytes.
    pub size: usize,
    /// Every method, in class-file order.
    pub methods: Vec<MethodInfo>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MethodInfo {
    pub name: String,
    pub descriptor: String,
    pub access: u16,
    /// Bytecode length of the `Code` attribute; `None` for abstract and native methods.
    pub code_length: Option<u32>,
}

/// What a class file references, beyond [`ClassInfo`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ClassRefs {
    /// Every `CONSTANT_Class` name.
    pub classes: Vec<String>,
    /// Owners of `<init>` method references (constructor calls, `super(..)`, `this(..)`).
    pub constructors: Vec<String>,
}

/// Every class of `input`, sorted by internal name (the first of duplicate names).
/// Class files that do not parse are left out.
pub fn list(input: &InputSet) -> Vec<ClassInfo> {
    let mut classes: Vec<ClassInfo> = parse_all(input)
        .into_iter()
        .filter_map(|parsed| parsed.map(|(info, _)| info))
        .collect();
    classes.sort_by(|a, b| a.internal.cmp(&b.internal));
    classes.dedup_by(|b, a| a.internal == b.internal);
    classes
}

/// [`parse`] of every class file of `input`, in [`InputSet::classes`] order
/// (`None` where it does not parse), on up to 8 threads.
pub fn parse_all(input: &InputSet) -> Vec<Option<(ClassInfo, ClassRefs)>> {
    let classes = input.classes();
    let threads = std::thread::available_parallelism()
        .map_or(1, |n| n.get())
        .clamp(1, 8)
        .min(classes.len().div_ceil(256).max(1));
    let chunk = classes.len().div_ceil(threads).max(1);
    std::thread::scope(|scope| {
        let parts: Vec<_> = classes
            .chunks(chunk)
            .map(|part| {
                let parsed =
                    scope.spawn(move || part.iter().map(|c| parse(c.bytes).ok()).collect());
                (part.len(), parsed)
            })
            .collect();
        parts
            .into_iter()
            .flat_map(|(len, parsed)| parsed.join().unwrap_or_else(|_| vec![None; len]))
            .collect()
    })
}

/// `public`, `static`, … of class (`InnerClasses`) access flags, in Java modifier order.
pub fn class_access_names(flags: u16) -> Vec<&'static str> {
    names(
        flags,
        &[
            (0x0001, "public"),
            (0x0002, "private"),
            (0x0004, "protected"),
            (0x0400, "abstract"),
            (0x0008, "static"),
            (0x0010, "final"),
            (0x0200, "interface"),
            (0x2000, "annotation"),
            (0x4000, "enum"),
            (0x1000, "synthetic"),
            (0x8000, "module"),
        ],
    )
}

/// `public`, `static`, … of method access flags, in Java modifier order.
pub fn method_access_names(flags: u16) -> Vec<&'static str> {
    names(
        flags,
        &[
            (0x0001, "public"),
            (0x0002, "private"),
            (0x0004, "protected"),
            (0x0400, "abstract"),
            (0x0008, "static"),
            (0x0010, "final"),
            (0x0020, "synchronized"),
            (0x0100, "native"),
            (0x0800, "strictfp"),
            (0x0040, "bridge"),
            (0x0080, "varargs"),
            (0x1000, "synthetic"),
        ],
    )
}

fn names(flags: u16, table: &[(u16, &'static str)]) -> Vec<&'static str> {
    table
        .iter()
        .filter(|(bit, _)| flags & bit != 0)
        .map(|&(_, name)| name)
        .collect()
}

#[derive(Clone)]
enum Constant {
    None,
    Utf8(String),
    Class(u16),
    MemberRef { class: u16, name_and_type: u16 },
    NameAndType { name: u16 },
}

struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], String> {
        if self.bytes.len() - self.pos < n {
            return Err(format!("truncated class file at byte {}", self.pos));
        }
        let s = &self.bytes[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }

    fn u1(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }

    fn u2(&mut self) -> Result<u16, String> {
        let b = self.take(2)?;
        Ok(u16::from_be_bytes([b[0], b[1]]))
    }

    fn u4(&mut self) -> Result<u32, String> {
        let b = self.take(4)?;
        Ok(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }
}

struct Pool(Vec<Constant>);

impl Pool {
    fn utf8(&self, index: u16) -> Option<String> {
        match self.0.get(index as usize) {
            Some(Constant::Utf8(s)) => Some(s.clone()),
            _ => None,
        }
    }

    fn class(&self, index: u16) -> Option<String> {
        match self.0.get(index as usize) {
            Some(Constant::Class(name)) => self.utf8(*name),
            _ => None,
        }
    }
}

/// Parses one class file.
pub fn parse(bytes: &[u8]) -> Result<(ClassInfo, ClassRefs), String> {
    let mut r = Reader { bytes, pos: 0 };
    if r.u4()? != 0xCAFE_BABE {
        return Err("not a class file".to_string());
    }
    r.take(4)?;
    let count = r.u2()? as usize;
    let mut pool = vec![Constant::None; count.max(1)];
    let mut i = 1;
    while i < count {
        let tag = r.u1()?;
        pool[i] = match tag {
            1 => {
                let len = r.u2()? as usize;
                Constant::Utf8(decode_modified_utf8(r.take(len)?))
            }
            7 => Constant::Class(r.u2()?),
            9..=11 => Constant::MemberRef {
                class: r.u2()?,
                name_and_type: r.u2()?,
            },
            12 => {
                let name = r.u2()?;
                r.u2()?;
                Constant::NameAndType { name }
            }
            3 | 4 | 17 | 18 => {
                r.take(4)?;
                Constant::None
            }
            5 | 6 => {
                r.take(8)?;
                i += 1;
                Constant::None
            }
            8 | 16 | 19 | 20 => {
                r.take(2)?;
                Constant::None
            }
            15 => {
                r.take(3)?;
                Constant::None
            }
            _ => return Err(format!("invalid constant pool tag {tag} at #{i}")),
        };
        i += 1;
    }
    let pool = Pool(pool);
    let access = r.u2()?;
    let internal = pool
        .class(r.u2()?)
        .ok_or_else(|| "bad this_class".to_string())?;
    let super_index = r.u2()?;
    let super_class = (super_index != 0)
        .then(|| pool.class(super_index))
        .flatten();
    let interfaces = r.u2()? as usize;
    r.take(interfaces * 2)?;
    let fields = r.u2()?;
    for _ in 0..fields {
        r.take(6)?;
        skip_attributes(&mut r)?;
    }
    let method_count = r.u2()?;
    let mut methods = Vec::with_capacity(method_count as usize);
    for _ in 0..method_count {
        let access = r.u2()?;
        let name = pool.utf8(r.u2()?).unwrap_or_default();
        let descriptor = pool.utf8(r.u2()?).unwrap_or_default();
        let mut code_length = None;
        for _ in 0..r.u2()? {
            let attribute = pool.utf8(r.u2()?);
            let len = r.u4()? as usize;
            let data = r.take(len)?;
            if attribute.as_deref() == Some("Code") && data.len() >= 8 {
                code_length = Some(u32::from_be_bytes([data[4], data[5], data[6], data[7]]));
            }
        }
        methods.push(MethodInfo {
            name,
            descriptor,
            access,
            code_length,
        });
    }
    let mut source_file = None;
    let mut outer = None;
    let mut inner_access = None;
    let mut enclosing = None;
    for _ in 0..r.u2()? {
        let attribute = pool.utf8(r.u2()?);
        let len = r.u4()? as usize;
        let data = r.take(len)?;
        let mut a = Reader {
            bytes: data,
            pos: 0,
        };
        match attribute.as_deref() {
            Some("SourceFile") => source_file = pool.utf8(a.u2()?),
            Some("EnclosingMethod") => enclosing = pool.class(a.u2()?),
            Some("InnerClasses") => {
                for _ in 0..a.u2()? {
                    let inner = a.u2()?;
                    let outer_index = a.u2()?;
                    a.u2()?;
                    let flags = a.u2()?;
                    if pool.class(inner).as_deref() == Some(internal.as_str()) {
                        inner_access = Some(flags);
                        if outer_index != 0 {
                            outer = pool.class(outer_index);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    if outer.is_none() {
        outer = enclosing;
    }

    let mut refs = ClassRefs::default();
    for constant in &pool.0 {
        match constant {
            Constant::Class(name) => {
                if let Some(name) = pool.utf8(*name) {
                    refs.classes.push(name);
                }
            }
            Constant::MemberRef {
                class,
                name_and_type,
            } => {
                let is_init = match pool.0.get(*name_and_type as usize) {
                    Some(Constant::NameAndType { name }) => {
                        pool.utf8(*name).as_deref() == Some("<init>")
                    }
                    _ => false,
                };
                if is_init {
                    if let Some(owner) = pool.class(*class) {
                        refs.constructors.push(owner);
                    }
                }
            }
            _ => {}
        }
    }
    refs.classes.sort();
    refs.classes.dedup();
    refs.constructors.sort();
    refs.constructors.dedup();

    Ok((
        ClassInfo {
            internal,
            outer,
            access,
            inner_access,
            super_class,
            source_file,
            size: bytes.len(),
            methods,
        },
        refs,
    ))
}

fn skip_attributes(r: &mut Reader<'_>) -> Result<(), String> {
    for _ in 0..r.u2()? {
        r.take(2)?;
        let len = r.u4()? as usize;
        r.take(len)?;
    }
    Ok(())
}

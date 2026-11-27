//! CMSIS-SVD register layouts as program-scoped volatile data declarations.

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;

use kuna_console::assertions::{Body, Directive};
use roxmltree::{Document, Node, ParsingOptions};

const MAX_BYTES: u64 = 64 * 1024 * 1024;
const MAX_NODES: u32 = 2_000_000;
const MAX_INSTANCES: usize = 100_000;
const MAX_DEPTH: usize = 64;
const MAX_NAME: usize = 256;
const MAX_DIRECTIVE_BYTES: usize = 64 * 1024 * 1024;

type Xml<'a> = Node<'a, 'a>;
type Result<T, E = String> = std::result::Result<T, E>;

pub(crate) fn load(path: &str) -> Result<Vec<Directive>> {
    let read = || -> Result<Vec<Directive>> {
        let mut xml = String::new();
        std::fs::File::open(path)
            .and_then(|file| file.take(MAX_BYTES + 1).read_to_string(&mut xml))
            .map_err(|e| e.to_string())?;
        if xml.len() as u64 > MAX_BYTES {
            return Err("file exceeds 64 MiB".into());
        }
        let document = Document::parse_with_options(
            &xml,
            ParsingOptions {
                nodes_limit: MAX_NODES,
                ..ParsingOptions::default()
            },
        )
        .map_err(|e| e.to_string())?;
        directives(document.root_element(), path)
    };
    read().map_err(|e| format!("--svd {path}: {e}"))
}

#[derive(Clone, Copy)]
struct Layout<'a> {
    size: Option<u64>,
    children: Option<Xml<'a>>,
    dimension: Option<Xml<'a>>,
    bits: Option<(u64, u64)>,
}

struct Register {
    name: String,
    bytes: u64,
    alternate: bool,
    fields: BTreeMap<u64, Option<String>>,
}

struct Import<'a> {
    peripherals: Xml<'a>,
    layouts: BTreeMap<u32, Layout<'a>>,
    registers: BTreeMap<u64, Register>,
    instances: usize,
}

fn directives(device: Xml<'_>, path: &str) -> Result<Vec<Directive>> {
    let peripherals = child(device, "peripherals")
        .filter(|_| device.has_tag_name("device"))
        .ok_or("expected a CMSIS-SVD device with peripherals")?;
    if number(device, "addressUnitBits")?.is_some_and(|bits| bits != 8) {
        return Err("only byte-addressed devices (addressUnitBits=8) are supported".into());
    }
    let mut import = Import {
        peripherals,
        layouts: BTreeMap::new(),
        registers: BTreeMap::new(),
        instances: 0,
    };
    let size = number(device, "size")?;
    for item in peripherals
        .children()
        .filter(|n| n.has_tag_name("peripheral"))
    {
        import.expand(item, 0, size, "", false, 0)?;
    }
    if import.registers.is_empty() {
        return Err("device has no registers".into());
    }
    let mut names = BTreeSet::new();
    let mut directives = Vec::with_capacity(import.registers.len());
    let mut field_names = BTreeSet::new();
    let mut bytes = 0;
    for (addr, mut register) in import.registers {
        if !names.insert(register.name.clone()) {
            while !names.insert(format!("{}_{addr:x}", register.name)) {
                register.name.push('_');
            }
            register.name = format!("{}_{addr:x}", register.name);
        }
        let decl = format!("volatile uint{} {}", register.bytes, register.name);
        let raw = format!("--svd {path}: data {addr:#x} {decl}");
        count_directive(&mut bytes, raw.len() + decl.len())?;
        directives.push(Directive {
            raw,
            body: Body::Data { addr, decl },
        });
        let size = register.bytes as i32;
        for (mask, field) in register.fields {
            let Some(field) = field else { continue };
            let mut name = format!("{}_{field}_Msk", register.name);
            if !field_names.insert(name.clone()) {
                name = format!("{}_{field}_{addr:x}_{mask:x}_Msk", register.name);
                while !field_names.insert(name.clone()) {
                    name.insert(0, '_');
                }
            }
            let raw = format!("--svd {path}: mmio-mask {addr:#x} {size} {mask:#x} {name}");
            count_directive(&mut bytes, raw.len() + name.len())?;
            directives.push(Directive {
                raw,
                body: Body::MmioMask {
                    addr,
                    size,
                    mask,
                    name,
                },
            });
        }
    }
    Ok(directives)
}

fn count_directive(total: &mut usize, bytes: usize) -> Result<()> {
    *total += bytes;
    if *total > MAX_DIRECTIVE_BYTES {
        return Err("expanded register and field declarations exceed 64 MiB".into());
    }
    Ok(())
}

impl<'a> Import<'a> {
    /// Resolve explicit inheritance before applying defaults from the instance's parent.
    fn layout(&mut self, node: Xml<'a>, depth: usize) -> Result<Layout<'a>> {
        if let Some(&layout) = self.layouts.get(&node.id().get()) {
            return Ok(layout);
        }
        check_depth(depth)?;
        let inherited = node
            .attribute("derivedFrom")
            .map(|reference| {
                let (base, member) = self.reference(node, reference, depth + 1)?;
                let mut layout = self.layout(base, depth + 1)?;
                if let Some(index) = member {
                    if let Some((offset, width)) = layout.bits {
                        let step = required_number(
                            layout.dimension.ok_or("missing array dimension")?,
                            "dimIncrement",
                        )?;
                        let offset = index
                            .checked_mul(step)
                            .and_then(|delta| offset.checked_add(delta))
                            .ok_or("field array offset overflow")?;
                        layout.bits = Some((offset, width));
                    }
                    layout.dimension = None;
                }
                Ok::<_, String>(layout)
            })
            .transpose()?;
        let own_children = if node.has_tag_name("peripheral") {
            child(node, "registers")
        } else if node.has_tag_name("register") {
            child(node, "fields")
        } else if node.has_tag_name("cluster") && node.children().any(|n| is_register_layout(&n)) {
            Some(node)
        } else {
            None
        };
        let layout = Layout {
            size: number(node, "size")?.or(inherited.and_then(|layout| layout.size)),
            children: own_children.or(inherited.and_then(|layout| layout.children)),
            dimension: if child(node, "dim").is_some() {
                Some(node)
            } else if text(node, "name").is_some_and(|name| name.contains("%s")) {
                inherited.and_then(|layout| layout.dimension)
            } else {
                None
            },
            bits: if node.has_tag_name("field") {
                field_bits(node)?.or(inherited.and_then(|layout| layout.bits))
            } else {
                None
            },
        };
        self.layouts.insert(node.id().get(), layout);
        Ok(layout)
    }

    fn reference(
        &mut self,
        node: Xml<'a>,
        reference: &str,
        depth: usize,
    ) -> Result<(Xml<'a>, Option<u64>)> {
        let missing = || format!("{}: unknown derivedFrom target {reference:?}", label(node));
        let expected = node.tag_name().name();
        let mut parts = reference.split('.').peekable();
        let mut scope = node.parent().ok_or_else(missing)?;
        if reference.contains('.') {
            scope = self.peripherals;
        }
        while let Some(part) = parts.next() {
            let kind = if parts.peek().is_none() {
                expected
            } else if scope == self.peripherals {
                "peripheral"
            } else {
                ""
            };
            let candidates = || {
                scope.children().filter(|&item| {
                    if kind.is_empty() {
                        is_register_layout(&item)
                    } else {
                        item.has_tag_name(kind)
                    }
                })
            };
            let exact = candidates().find(|&item| text(item, "name") == Some(part));
            let (found, member) = if let Some(exact) = exact {
                (exact, None)
            } else {
                let mut matching = None;
                for item in candidates() {
                    let Some(name) = text(item, "name").filter(|name| name.contains("%s")) else {
                        continue;
                    };
                    if item == node || !Array::matches_template(name, part) {
                        continue;
                    }
                    let dimension = self.layout(item, depth)?.dimension;
                    if let Some(index) = Array::read(dimension, name)?.find_index(name, part) {
                        if matching.replace((item, Some(index))).is_some() {
                            return Err(format!(
                                "{}: ambiguous derivedFrom target {reference:?}",
                                label(node)
                            ));
                        }
                    }
                }
                matching.ok_or_else(missing)?
            };
            if parts.peek().is_none() {
                return Ok((found, member));
            }
            scope = self.layout(found, depth)?.children.ok_or_else(missing)?;
        }
        Err(missing())
    }

    fn expand(
        &mut self,
        node: Xml<'a>,
        base: u64,
        default_size: Option<u64>,
        prefix: &str,
        alternate: bool,
        depth: usize,
    ) -> Result<()> {
        check_depth(depth)?;
        let name = text(node, "name")
            .filter(|name| !name.is_empty())
            .ok_or_else(|| format!("{} has no name", label(node)))?;
        let addr = if node.has_tag_name("peripheral") {
            "baseAddress"
        } else {
            "addressOffset"
        };
        let base = base
            .checked_add(required_number(node, addr)?)
            .ok_or_else(|| format!("{name}: address overflow"))?;
        let layout = self.layout(node, 0)?;
        let size = layout.size.or(default_size);
        let alternate = alternate
            || [
                "alternateRegister",
                "alternatePeripheral",
                "alternateCluster",
            ]
            .into_iter()
            .any(|tag| child(node, tag).is_some());
        let array = Array::read(layout.dimension, name)?;
        for i in 0..array.count {
            self.instances += 1;
            if self.instances > MAX_INSTANCES {
                return Err(
                    "device exceeds 100000 expanded peripherals, clusters and registers".into(),
                );
            }
            let addr = i
                .checked_mul(array.stride)
                .and_then(|delta| base.checked_add(delta))
                .ok_or_else(|| format!("{name}: array address overflow"))?;
            let name = array.name_at(name, i);
            if prefix.len() + name.len() >= MAX_NAME {
                return Err("qualified SVD register name exceeds 255 bytes".into());
            }
            let mut qualified = String::with_capacity(prefix.len() + name.len() + 1);
            qualified.push_str(prefix);
            if prefix.is_empty() && name.starts_with(|c: char| c.is_ascii_digit()) {
                qualified.push('_');
            }
            qualified.extend(name.chars().map(identifier_char));
            if node.has_tag_name("register") {
                let bytes = match size {
                    Some(bits @ (8 | 16 | 32 | 64)) => bits / 8,
                    Some(bits) => {
                        return Err(format!("{qualified}: unsupported {bits}-bit register"))
                    }
                    None => return Err(format!("{qualified}: missing register size")),
                };
                addr.checked_add(bytes - 1)
                    .ok_or_else(|| format!("{qualified}: address overflow"))?;
                let fields = self.field_masks(layout.children, bytes * 8)?;
                use std::collections::btree_map::Entry;
                let register = Register {
                    name: qualified,
                    bytes,
                    alternate,
                    fields,
                };
                match self.registers.entry(addr) {
                    Entry::Vacant(slot) => {
                        slot.insert(register);
                    }
                    Entry::Occupied(mut slot) => {
                        let old = slot.get_mut();
                        // The read proves an address, not which hardware view is active.
                        if old.bytes == bytes {
                            old.fields.retain(|mask, name| {
                                name.is_some() && register.fields.get(mask) == Some(&*name)
                            });
                        } else {
                            old.fields.clear();
                        }
                        if old.alternate && !alternate {
                            old.name = register.name;
                            old.bytes = bytes;
                            old.alternate = false;
                        }
                    }
                }
            } else if let Some(children) = layout.children {
                qualified.push('_');
                for item in children.children().filter(is_register_layout) {
                    self.expand(item, addr, size, &qualified, alternate, depth + 1)?;
                }
            }
        }
        Ok(())
    }

    fn field_masks(
        &mut self,
        fields: Option<Xml<'a>>,
        size: u64,
    ) -> Result<BTreeMap<u64, Option<String>>> {
        let mut masks = BTreeMap::new();
        let Some(fields) = fields else {
            return Ok(masks);
        };
        for field in fields.children().filter(|n| n.has_tag_name("field")) {
            let name = text(field, "name")
                .filter(|name| !name.is_empty())
                .ok_or("field has no name")?;
            let layout = self.layout(field, 0)?;
            let (lsb, width) = layout
                .bits
                .ok_or_else(|| format!("{name}: missing field range"))?;
            let array = Array::read(layout.dimension, name)?;
            for i in 0..array.count {
                self.instances += 1;
                if self.instances > MAX_INSTANCES {
                    return Err("device exceeds 100000 expanded peripheral layout entries".into());
                }
                let offset = i
                    .checked_mul(array.stride)
                    .and_then(|delta| lsb.checked_add(delta));
                let offset = offset
                    .filter(|offset| width != 0 && width <= size && *offset <= size - width)
                    .ok_or_else(|| {
                        format!("{name}: field range exceeds its {size}-bit register")
                    })?;
                let mask = (u64::MAX >> (64 - width)) << offset;
                let name: String = array
                    .name_at(name, i)
                    .chars()
                    .map(identifier_char)
                    .collect();
                masks
                    .entry(mask)
                    .and_modify(|existing| {
                        if existing.as_ref() != Some(&name) {
                            *existing = None;
                        }
                    })
                    .or_insert(Some(name));
            }
        }
        Ok(masks)
    }
}

fn identifier_char(c: char) -> char {
    if c.is_ascii_alphanumeric() {
        c
    } else {
        '_'
    }
}

fn field_bits(field: Xml<'_>) -> Result<Option<(u64, u64)>> {
    let invalid = || format!("{}: invalid bit range", label(field));
    let offset = number(field, "bitOffset")?;
    let width = number(field, "bitWidth")?;
    let lsb = number(field, "lsb")?;
    let msb = number(field, "msb")?;
    let range = text(field, "bitRange");
    let (lsb, msb) = match (offset, width, lsb, msb, range) {
        (None, None, None, None, None) => return Ok(None),
        (Some(offset), width, None, None, None) => return Ok(Some((offset, width.unwrap_or(1)))),
        (None, None, Some(lsb), Some(msb), None) => (lsb, msb),
        (None, None, None, None, Some(range)) => {
            let (msb, lsb) = range
                .strip_prefix('[')
                .and_then(|r| r.strip_suffix(']'))
                .and_then(|r| r.split_once(':'))
                .ok_or_else(invalid)?;
            (
                lsb.trim().parse().map_err(|_| invalid())?,
                msb.trim().parse().map_err(|_| invalid())?,
            )
        }
        _ => return Err(invalid()),
    };
    let width = msb
        .checked_sub(lsb)
        .and_then(|delta| delta.checked_add(1))
        .ok_or_else(invalid)?;
    Ok(Some((lsb, width)))
}

struct Array {
    count: u64,
    stride: u64,
    indexes: Option<Vec<String>>,
}

impl Array {
    fn read(dimension: Option<Xml<'_>>, name: &str) -> Result<Self> {
        if name.len() > MAX_NAME || name.matches("%s").count() > 1 {
            return Err("SVD name exceeds 256 bytes or has multiple array placeholders".into());
        }
        if let Some(dim) = dimension {
            let count = required_number(dim, "dim")?;
            let stride = required_number(dim, "dimIncrement")?;
            if count == 0 || count > MAX_INSTANCES as u64 || !name.contains("%s") {
                return Err(format!("{name}: invalid or oversized array"));
            }
            let indexes = text(dim, "dimIndex")
                .map(|s| dim_indexes(s, count as usize))
                .transpose()?;
            Ok(Self {
                count,
                stride,
                indexes,
            })
        } else if name.contains("%s") {
            Err(format!("{name}: array name has no dimensions"))
        } else {
            Ok(Self {
                count: 1,
                stride: 0,
                indexes: None,
            })
        }
    }

    fn template_index<'b>(template: &str, name: &'b str) -> Option<&'b str> {
        let (prefix, suffix) = template
            .split_once("[%s]")
            .or_else(|| template.split_once("%s"))?;
        Some(name.strip_prefix(prefix)?.strip_suffix(suffix)?)
    }

    fn matches_template(template: &str, name: &str) -> bool {
        Self::template_index(template, name).is_some_and(|index| !index.is_empty())
    }

    fn find_index(&self, template: &str, name: &str) -> Option<u64> {
        let index = Self::template_index(template, name)?;
        if let Some(indexes) = &self.indexes {
            return indexes
                .iter()
                .position(|candidate| candidate == index)
                .map(|i| i as u64);
        }
        let value = index.parse::<u64>().ok()?;
        (value < self.count && value.to_string() == index).then_some(value)
    }

    fn name_at<'a>(&self, name: &'a str, i: u64) -> Cow<'a, str> {
        if !name.contains("%s") {
            return Cow::Borrowed(name);
        }
        let decimal;
        let index = if let Some(labels) = &self.indexes {
            &labels[i as usize]
        } else {
            decimal = i.to_string();
            &decimal
        };
        Cow::Owned(name.replace("[%s]", index).replace("%s", index))
    }
}

fn child<'a>(node: Xml<'a>, tag: &str) -> Option<Xml<'a>> {
    node.children().find(|n| n.has_tag_name(tag))
}

fn text<'a>(node: Xml<'a>, tag: &str) -> Option<&'a str> {
    child(node, tag).and_then(|n| n.text()).map(str::trim)
}

fn is_register_layout(node: &Xml<'_>) -> bool {
    node.has_tag_name("register") || node.has_tag_name("cluster")
}

fn label<'a>(node: Xml<'a>) -> &'a str {
    text(node, "name").unwrap_or_else(|| node.tag_name().name())
}

fn number(node: Xml<'_>, tag: &str) -> Result<Option<u64>> {
    let Some(raw) = text(node, tag) else {
        return Ok(None);
    };
    let value = raw.strip_prefix('+').unwrap_or(raw);
    let (value, shift) = match value.as_bytes().last().map(u8::to_ascii_lowercase) {
        Some(b'k') => (&value[..value.len() - 1], 10),
        Some(b'm') => (&value[..value.len() - 1], 20),
        Some(b'g') => (&value[..value.len() - 1], 30),
        Some(b't') => (&value[..value.len() - 1], 40),
        _ => (value, 0),
    };
    let (digits, radix) = if let Some(hex) = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
    {
        (hex, 16)
    } else if let Some(binary) = value.strip_prefix('#') {
        (binary, 2)
    } else {
        (value, 10)
    };
    u64::from_str_radix(digits, radix)
        .ok()
        .and_then(|n| n.checked_mul(1 << shift))
        .map(Some)
        .ok_or_else(|| format!("{}: invalid {tag} {raw:?}", label(node)))
}

fn required_number(node: Xml<'_>, tag: &str) -> Result<u64> {
    number(node, tag)?.ok_or_else(|| format!("{}: missing {tag}", label(node)))
}

fn check_depth(depth: usize) -> Result<()> {
    if depth >= MAX_DEPTH {
        return Err(
            "SVD layout nesting or derivedFrom chain exceeds 64 levels (possible cycle)".into(),
        );
    }
    Ok(())
}

fn dim_indexes(text: &str, count: usize) -> Result<Vec<String>> {
    if text.split(',').any(|label| label.trim().len() > MAX_NAME) {
        return Err("SVD array index exceeds 256 bytes".into());
    }
    let indexes = if let Some((first, last)) = text.split_once('-').filter(|_| !text.contains(','))
    {
        let first = first.trim();
        let last = last.trim();
        if let (Ok(first), Ok(last)) = (first.parse::<u32>(), last.parse::<u32>()) {
            (first..=last)
                .take(count + 1)
                .map(|i| i.to_string())
                .collect()
        } else if let ([first], [last]) = (first.as_bytes(), last.as_bytes()) {
            (*first..=*last)
                .take(count + 1)
                .map(|i| char::from(i).to_string())
                .collect()
        } else {
            Vec::new()
        }
    } else {
        text.split(',')
            .map(str::trim)
            .take(count + 1)
            .map(str::to_owned)
            .collect::<Vec<_>>()
    };
    if indexes.len() != count || indexes.iter().any(String::is_empty) {
        return Err(format!(
            "dimIndex {text:?} does not describe {count} array entries"
        ));
    }
    Ok(indexes)
}

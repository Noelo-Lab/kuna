//! CMSIS-SVD device descriptions: peripherals as C structures at their base
//! addresses, and interrupt names.
//!
//! `derivedFrom` peripherals reuse their base's registers (and structure);
//! clusters flatten into `<cluster>_<register>` members; `dim` arrays expand
//! `%s` (from `dimIndex` or the index). Registers that overlap an earlier one
//! (SVD alternates) are dropped from the structure; gaps become reserved bytes.

use kuna_base::xml::{xml_tree, Element};
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Register {
    pub name: String,
    pub offset: u64,
    pub size: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Peripheral {
    pub name: String,
    pub base: u64,
    /// The structure name (the base peripheral's for a derived one).
    pub type_name: String,
    pub registers: Vec<Register>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Device {
    pub name: String,
    pub peripherals: Vec<Peripheral>,
    /// IRQ number to name.
    pub interrupts: BTreeMap<u32, String>,
}

fn child<'a>(e: &'a Element, name: &str) -> Option<&'a Rc<Element>> {
    e.get_children().iter().find(|c| c.get_name() == name)
}

fn text(e: &Element, name: &str) -> Option<String> {
    child(e, name).map(|c| String::from_utf8_lossy(c.get_content()).trim().to_string())
}

fn number(s: &str) -> Option<u64> {
    let s = s.trim();
    if let Some(h) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u64::from_str_radix(h, 16).ok()
    } else if let Some(b) = s.strip_prefix('#') {
        u64::from_str_radix(&b.replace(['x', 'X'], "0"), 2).ok()
    } else {
        s.parse().ok()
    }
}

fn c_name(s: &str) -> String {
    let mut out: String = s.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
    if out.chars().next().is_none_or(|c| c.is_ascii_digit()) {
        out.insert(0, '_');
    }
    out
}

/// `(name, offset)` for each element of a `dim` array, or the one element.
fn expand(e: &Element, name: &str, offset: u64) -> Vec<(String, u64)> {
    let Some(dim) = text(e, "dim").and_then(|d| number(&d)) else {
        return vec![(name.to_string(), offset)];
    };
    let step = text(e, "dimIncrement").and_then(|d| number(&d)).unwrap_or(0);
    let index: Vec<String> = match text(e, "dimIndex") {
        Some(list) if list.contains('-') && !list.contains(',') => {
            let (a, b) = list.split_once('-').unwrap_or_default();
            match (a.trim().parse::<u64>(), b.trim().parse::<u64>()) {
                (Ok(a), Ok(b)) => (a..=b).map(|i| i.to_string()).collect(),
                _ => Vec::new(),
            }
        }
        Some(list) => list.split(',').map(|s| s.trim().to_string()).collect(),
        None => Vec::new(),
    };
    (0..dim.min(1024))
        .map(|i| {
            let tag = index.get(i as usize).cloned().unwrap_or_else(|| i.to_string());
            (name.replace("[%s]", &tag).replace("%s", &tag), offset + i * step)
        })
        .collect()
}

fn registers(parent: &Element, prefix: &str, base: u64, size_bits: u32, out: &mut Vec<Register>) {
    for e in parent.get_children() {
        let is_cluster = e.get_name() == "cluster";
        if e.get_name() != "register" && !is_cluster {
            continue;
        }
        let Some(name) = text(e, "name") else { continue };
        let Some(offset) = text(e, "addressOffset").and_then(|o| number(&o)) else { continue };
        let size = text(e, "size").and_then(|s| number(&s)).map_or(size_bits, |s| s as u32);
        for (n, off) in expand(e, &name, base + offset) {
            let full = format!("{prefix}{}", c_name(&n));
            if is_cluster {
                registers(e, &format!("{full}_"), off, size, out);
            } else {
                out.push(Register { name: full, offset: off, size: (size / 8).max(1) });
            }
        }
    }
}

/// Parse an SVD file.
pub fn parse(bytes: &[u8]) -> Result<Device, String> {
    let doc = xml_tree(bytes).map_err(|e| format!("SVD: {}", e.explain()))?;
    let root = doc.get_root();
    if root.get_name() != "device" {
        return Err(format!("SVD: root element is <{}>, not <device>", root.get_name()));
    }
    let mut dev = Device { name: text(root, "name").unwrap_or_default(), ..Device::default() };
    let dev_size = text(root, "size").and_then(|s| number(&s)).unwrap_or(32) as u32;
    let Some(list) = child(root, "peripherals") else { return Ok(dev) };
    let mut by_name: BTreeMap<String, (String, Vec<Register>)> = BTreeMap::new();
    for p in list.get_children().iter().filter(|p| p.get_name() == "peripheral") {
        let (Some(name), Some(base)) = (text(p, "name"), text(p, "baseAddress").and_then(|b| number(&b))) else { continue };
        let size = text(p, "size").and_then(|s| number(&s)).map_or(dev_size, |s| s as u32);
        let mut regs = Vec::new();
        if let Some(r) = child(p, "registers") {
            registers(r, "", 0, size, &mut regs);
        }
        let derived = p.get_attribute_value("derivedFrom").ok().map(|v| String::from_utf8_lossy(v).to_string());
        let type_name = match derived.as_ref().and_then(|d| by_name.get(d)) {
            Some((t, base_regs)) if regs.is_empty() => {
                regs = base_regs.clone();
                t.clone()
            }
            _ => format!("{}_Type", c_name(&name)),
        };
        for irq in p.get_children().iter().filter(|c| c.get_name() == "interrupt") {
            if let (Some(n), Some(v)) = (text(irq, "name"), text(irq, "value").and_then(|v| number(&v))) {
                dev.interrupts.entry(v as u32).or_insert(c_name(&n));
            }
        }
        regs.sort_by_key(|r| r.offset);
        by_name.insert(name.clone(), (type_name.clone(), regs.clone()));
        dev.peripherals.push(Peripheral { name: c_name(&name), base, type_name, registers: regs });
    }
    Ok(dev)
}

impl Peripheral {
    /// Bytes the registers span.
    pub fn extent(&self) -> u64 {
        self.registers.iter().map(|r| r.offset + u64::from(r.size)).max().unwrap_or(0)
    }

    /// `struct <type> { ... };` with reserved padding, one member per
    /// non-overlapping register.
    pub fn c_struct(&self) -> String {
        let mut body = String::new();
        let mut at = 0u64;
        let mut pad = 0;
        for r in &self.registers {
            if r.offset < at {
                continue;
            }
            if r.offset > at {
                body.push_str(&format!(" unsigned char _reserved{pad}[{}];", r.offset - at));
                pad += 1;
            }
            let ty = match r.size {
                1 => "unsigned char",
                2 => "unsigned short",
                8 => "unsigned long long",
                _ => "unsigned int",
            };
            body.push_str(&format!(" {ty} {};", r.name));
            at = r.offset + u64::from(r.size);
        }
        format!("struct {} {{{body} }};", self.type_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SVD: &str = r#"<?xml version="1.0"?>
<device><name>MINI</name><size>32</size><peripherals>
<peripheral><name>USART1</name><baseAddress>0x40011000</baseAddress><registers>
<register><name>SR</name><addressOffset>0x0</addressOffset></register>
<register><name>DR</name><addressOffset>0x4</addressOffset></register>
<register><name>CR%s</name><addressOffset>0xC</addressOffset><dim>2</dim><dimIncrement>4</dimIncrement><dimIndex>1,2</dimIndex></register>
</registers><interrupt><name>USART1</name><value>37</value></interrupt></peripheral>
<peripheral derivedFrom="USART1"><name>USART2</name><baseAddress>0x40004400</baseAddress></peripheral>
</peripherals></device>"#;

    #[test]
    fn peripherals_become_structures_and_interrupts_names() {
        let d = parse(SVD.as_bytes()).unwrap();
        assert_eq!(d.peripherals.len(), 2);
        let u1 = &d.peripherals[0];
        assert_eq!(u1.c_struct(), "struct USART1_Type { unsigned int SR; unsigned int DR; unsigned char _reserved0[4]; unsigned int CR1; unsigned int CR2; };");
        assert_eq!(u1.extent(), 0x14);
        assert_eq!(d.peripherals[1].type_name, "USART1_Type");
        assert_eq!(d.interrupts.get(&37).map(String::as_str), Some("USART1"));
    }
}

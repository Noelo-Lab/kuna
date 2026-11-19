//! Raw Cortex-M images: the vector table at offset 0 gives the stack, the
//! handlers and, by voting, the load address.
//!
//! Word 0 is the initial main stack pointer and must lie in SRAM (or a CCM/TCM
//! alias below it); words 1..15 are
//! the system exceptions, each zero or an odd (Thumb) pointer into the Code
//! region, with Reset non-zero; IRQ words follow until the first word that is
//! neither zero nor such a pointer. The base is the 0x1000-aligned
//! `handler - offset` that the most (handler, dummy-handler instruction) pairs
//! vote for, with at least [`MIN_VOTES`] votes; otherwise the first common
//! flash base that holds every handler; otherwise the handler page below the
//! lowest handler.

/// One handler slot: its name and target (Thumb bit cleared).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Handler {
    pub name: String,
    pub addr: u64,
}

/// What the vector table says about a raw image.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VectorTable {
    pub base: u64,
    pub initial_sp: u32,
    /// Unique handler targets, first name wins; a target several IRQs share is
    /// `Default_Handler`.
    pub handlers: Vec<Handler>,
    /// Number of words the table spans, the stack pointer included.
    pub words: usize,
}

impl VectorTable {
    pub fn reset(&self) -> u64 {
        self.handlers[0].addr
    }
}

const SYSTEM: [&str; 15] = [
    "Reset", "NMI", "HardFault", "MemManage", "BusFault", "UsageFault", "SecureFault", "", "", "", "SVC",
    "DebugMon", "", "PendSV", "SysTick",
];

const MAX_IRQS: usize = 496;
const MIN_VOTES: usize = 15;
const COMMON_BASES: [u64; 6] = [0x0800_0000, 0x0000_0000, 0x0040_0000, 0x0001_0000, 0x0000_8000, 0x1000_0000];

/// Dummy-handler encodings: `bx lr`, `b .`, `b.w .`, `nop; b .`.
const DUMMY: [&[u8]; 4] = [&[0x70, 0x47], &[0xfe, 0xe7], &[0xff, 0xf7, 0xfe, 0xbf], &[0x00, 0xbf, 0xfe, 0xe7]];

fn word(bytes: &[u8], i: usize) -> Option<u32> {
    Some(u32::from_le_bytes(bytes.get(4 * i..4 * i + 4)?.try_into().ok()?))
}

fn code_pointer(w: u32) -> bool {
    w & 1 == 1 && w < 0x2000_0000
}

/// Probe `bytes` as a Cortex-M image; `base` overrides the vote.
pub fn probe(bytes: &[u8], base: Option<u64>) -> Option<VectorTable> {
    let sp = word(bytes, 0)?;
    if sp % 4 != 0 || !(0x1000_0000..=0x4000_0000).contains(&sp) {
        return None;
    }
    let mut slots: Vec<(String, u32)> = Vec::new();
    for (i, name) in SYSTEM.iter().enumerate() {
        let w = word(bytes, i + 1)?;
        if name.is_empty() {
            continue;
        }
        if i == 0 && !code_pointer(w) {
            return None;
        }
        if w != 0 && !code_pointer(w) {
            return None;
        }
        if w != 0 {
            slots.push((format!("{name}_Handler"), w));
        }
    }
    let mut words = 16;
    for irq in 0..MAX_IRQS {
        let Some(w) = word(bytes, 16 + irq) else { break };
        if w != 0 && !code_pointer(w) {
            break;
        }
        words = 17 + irq;
        if w != 0 {
            slots.push((format!("IRQ{irq}_Handler"), w));
        }
    }
    while words > 16 && word(bytes, words - 1) == Some(0) {
        words -= 1;
    }
    let targets: Vec<u64> = slots.iter().map(|(_, w)| u64::from(w & !1)).collect();
    let len = bytes.len() as u64;
    let holds = |b: u64| targets.iter().all(|&t| t >= b && t < b + len);
    let base = match base {
        Some(b) => b,
        None => vote(bytes, &targets, words)
            .or_else(|| COMMON_BASES.iter().copied().find(|&b| holds(b)))
            .unwrap_or_else(|| targets.iter().min().copied().unwrap_or(0) & !0xfff),
    };
    if !holds(base) {
        return None;
    }
    let mut handlers: Vec<Handler> = Vec::new();
    for ((name, _), &addr) in slots.iter().zip(&targets) {
        match handlers.iter_mut().find(|h| h.addr == addr) {
            Some(h) if name.starts_with("IRQ") => h.name = "Default_Handler".into(),
            Some(_) => {}
            None => handlers.push(Handler { name: name.clone(), addr }),
        }
    }
    Some(VectorTable { base, initial_sp: sp, handlers, words })
}

fn vote(bytes: &[u8], targets: &[u64], words: usize) -> Option<u64> {
    let mut hits = Vec::new();
    let mut off = 4 * words;
    while off + 2 <= bytes.len() {
        if DUMMY.iter().any(|p| bytes[off..].starts_with(p)) {
            hits.push(off as u64);
        }
        off += 2;
    }
    let mut votes: std::collections::BTreeMap<u64, usize> = std::collections::BTreeMap::new();
    for &t in targets {
        for &h in &hits {
            if t >= h && (t - h) % 0x1000 == 0 {
                *votes.entry(t - h).or_default() += 1;
            }
        }
    }
    votes.into_iter().filter(|&(_, n)| n >= MIN_VOTES).max_by_key(|&(b, n)| (n, std::cmp::Reverse(b))).map(|(b, _)| b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn image(base: u32) -> Vec<u8> {
        let mut b = vec![0u8; 0x400];
        let mut put = |i: usize, w: u32| b[4 * i..4 * i + 4].copy_from_slice(&w.to_le_bytes());
        put(0, 0x2000_8000);
        put(1, base + 0x201);
        for i in [2, 3, 11, 14, 15] {
            put(i, base + 0x301);
        }
        for irq in 0..20 {
            put(16 + irq, base + 0x301);
        }
        put(16 + 20, base + 0x281);
        b[0x300..0x302].copy_from_slice(&[0xfe, 0xe7]);
        b
    }

    #[test]
    fn votes_the_flash_base_and_names_handlers() {
        let t = probe(&image(0x0800_0000), None).unwrap();
        assert_eq!(t.base, 0x0800_0000);
        assert_eq!(t.reset(), 0x0800_0200);
        assert_eq!(t.words, 37);
        let names: Vec<&str> = t.handlers.iter().map(|h| h.name.as_str()).collect();
        assert_eq!(names, ["Reset_Handler", "Default_Handler", "IRQ20_Handler"]);
    }

    #[test]
    fn rejects_a_non_vector_table() {
        assert!(probe(&[0x7f, b'E', b'L', b'F', 0, 0, 0, 0], None).is_none());
        let mut b = image(0);
        b[4..8].copy_from_slice(&0x200u32.to_le_bytes());
        assert!(probe(&b, None).is_none());
    }
}

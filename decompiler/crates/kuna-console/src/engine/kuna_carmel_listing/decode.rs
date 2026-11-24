//! Typed field bindings used for the native listing. No executable semantics.

mod tables;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct Operand {
    pub kind: u8,
    pub value: u32,
    pub lsb: u8,
    pub width: u8,
}

#[derive(Clone, Copy)]
pub(super) struct Operation {
    pub family: u16,
    pub semantic: u16,
    pub left: u32,
    pub right: Option<u32>,
    pub payload: u64,
    pub extra: u32,
    operands: [Operand; 10],
    operand_count: usize,
}

impl Operation {
    pub(super) fn narrow(&self) -> bool {
        self.right.is_none()
    }

    pub(super) fn operands(&self) -> &[Operand] {
        &self.operands[..self.operand_count]
    }

    fn new(
        family: u16,
        semantic: u16,
        left: u32,
        right: Option<u32>,
        payload: u64,
        extra: u32,
        projections: &[OperandField],
    ) -> Self {
        let mut operands = [Operand::default(); 10];
        for (out, projection) in operands.iter_mut().zip(projections) {
            let field = projection.field;
            *out = Operand {
                kind: projection.kind,
                value: field.read(payload),
                lsb: field.lsb,
                width: field.width,
            };
        }
        Self { family, semantic, left, right, payload, extra, operands, operand_count: projections.len() }
    }
}

#[derive(Clone, Copy)]
struct Field {
    lsb: u8,
    width: u8,
    mask: u64,
}

const fn field(lsb: u8, width: u8) -> Field {
    Field { lsb, width, mask: (1u64 << width) - 1 }
}

impl Field {
    fn read(self, value: u64) -> u32 {
        ((value >> self.lsb) & self.mask) as u32
    }
}

struct Pattern {
    mask: u64,
    value: u64,
}

const fn pattern(lsb: u8, width: u8, key: u32) -> Pattern {
    Pattern { mask: field(lsb, width).mask << lsb, value: (key as u64) << lsb }
}

impl Pattern {
    fn matches(&self, value: u64) -> bool {
        value & self.mask == self.value
    }
}

struct OperandField {
    kind: u8,
    field: Field,
}

const fn operand(kind: u8, field: Field) -> OperandField {
    OperandField { kind, field }
}

struct Rule {
    gate: Pattern,
    operands: &'static [OperandField],
}

struct Variant24 {
    key: u32,
    semantic: u16,
    rules: &'static [Rule],
}

struct Family24 {
    id: u16,
    gate: Pattern,
    variant: Field,
    variants: &'static [Variant24],
}

struct Variant48 {
    key: u32,
    semantic: u16,
}

struct Family48 {
    id: u16,
    cliw: bool,
    variant: Field,
    extra: Field,
    variants: &'static [Variant48],
    operands: &'static [OperandField],
}

fn compact_family(word: u32) -> Option<&'static Family24> {
    tables::COMPACT_INDEX[((word >> 15) & 0xff) as usize]
        .iter()
        .map(|&index| &tables::COMPACT[usize::from(index)])
        .find(|f| f.gate.matches(u64::from(word)))
}

/// Native operand admission, distinct from the CrossView rendering.
/// `None` means that no compact family matches the word.
pub(super) fn compact_rejection(word: u32) -> Option<bool> {
    let family = compact_family(word)?;
    if !matches!(family.id, 16 | 35 | 40 | 41 | 45 | 49) {
        return Some(false);
    }
    let payload = u64::from(word);
    Some(compact_rule(family, payload).is_some_and(|(_, rule)| {
        rule.operands.iter().any(|op| match op.kind {
            107 => matches!(op.field.read(payload), 21 | 28 | 29),
            141 => matches!(op.field.read(payload), 0 | 1 | 3 | 6),
            _ => false,
        })
    }))
}

fn compact_rule(family: &Family24, payload: u64) -> Option<(u16, &Rule)> {
    let key = family.variant.read(payload);
    let variant = family.variants.iter().find(|v| v.key == key)?;
    let rule = variant.rules.iter().find(|r| r.gate.matches(payload))?;
    Some((variant.semantic, rule))
}

pub(super) fn compact(word: u32) -> Option<Operation> {
    let payload = u64::from(word);
    let family = compact_family(word)?;
    let (semantic, rule) = compact_rule(family, payload)?;
    Some(Operation::new(family.id, semantic, word, None, payload, 0, rule.operands))
}

fn wide_family(left: u32, right: u32) -> Option<(&'static Family48, u64)> {
    let right_payload = u64::from(right & 0x7fffff);
    let payload = right_payload << 22 | u64::from(left & 0x3fffff);
    let family = tables::WIDE.get(usize::from(tables::WIDE_INDEX[(payload >> 35) as usize]))?;
    let payload = if family.cliw { right_payload << 23 | u64::from(left & 0x7fffff) } else { payload };
    Some((family, payload))
}

pub(super) fn wide_rejected(left: u32, right: u32) -> bool {
    let Some((family, payload)) = wide_family(left, right) else { return false; };
    let variant = family.variant.read(payload);
    match family.id {
        43 => family.variants.iter().any(|v| matches!(v.semantic, 94 | 95) && v.key == variant)
            && family.operands.iter().any(|op| op.kind == 125 && matches!(op.field.read(payload), 0x45..=0x47 | 0x7d | 0x7f)),
        44 if left >> 22 == 1 => {
            let [destination, source, count] = family.operands else { return false; };
            // This CE2 word shifter route rejects its unconnected
            // configuration input before either operand can issue.
            destination.field.read(payload) == 0xc0 && source.field.read(payload) == 0xf8
                && count.field.read(payload) == 0x4e
                && family.variants.iter().any(|v| v.semantic == 41 && v.key == variant)
        }
        _ => false,
    }
}

pub(super) fn wide(left: u32, right: u32) -> Option<Operation> {
    let (family, payload) = wide_family(left, right)?;
    let key = family.variant.read(payload);
    let variant = family.variants.iter().find(|v| v.key == key)?;
    Some(Operation::new(
        family.id, variant.semantic, left, Some(right), payload, family.extra.read(payload), family.operands,
    ))
}

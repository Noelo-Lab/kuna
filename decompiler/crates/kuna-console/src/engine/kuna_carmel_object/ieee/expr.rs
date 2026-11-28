//! TASKING evaluates an LR expression before writing its result in section MAUs.

use kuna_analysis::loader::ieee695::{self as wire, Cursor, Id, err};
use kuna_base::error::KunaResult;

#[derive(Clone, Copy)]
pub(super) enum Variable {
    Base(Id),
    Relative(Id),
    Public(Id),
    External(Id),
}

#[derive(Clone, Copy)]
enum Token {
    Number(i64),
    Variable(Variable),
    Op(u8),
}

pub(super) struct Expression {
    pub(super) at: usize,
    tokens: Vec<Token>,
}

impl Expression {
    pub(super) fn read(cursor: &mut Cursor<'_>, parenthesized: bool) -> KunaResult<Self> {
        let at = cursor.offset;
        let mut tokens = Vec::new();
        while let Some(atom) = wire::expression_atom(cursor, parenthesized)? {
            let token = match atom {
                wire::Atom::Number(n) => Token::Number(n),
                wire::Atom::Letter(letter) => Token::Variable(match letter {
                    0xc9 => Variable::Public(cursor.id()?),
                    0xcc => Variable::Base(cursor.id()?),
                    0xd2 => Variable::Relative(cursor.id()?),
                    0xd8 => Variable::External(cursor.id()?),
                    _ => return Err(err(cursor.offset - 1, format_args!("unsupported expression variable {letter:#04x}"))),
                }),
                wire::Atom::Function(op @ (0x90 | 0x91 | 0x93 | 0x94 | 0xa0..=0xb5)) => Token::Op(op),
                wire::Atom::Function(op) => return Err(err(cursor.offset - 1, format_args!("unsupported expression function {op:#04x}"))),
            };
            tokens.push(token);
        }
        if tokens.is_empty() { return Err(err(at, "empty expression")); }
        Ok(Self { at, tokens })
    }

    pub(super) fn take_word_count(&mut self) -> KunaResult<usize> {
        // The parenthesized form defaults to one MAU. A trailing number
        // after the expression, rather than inside it, overrides that width.
        if let [_, .., Token::Number(n)] = &self.tokens[..] {
            let count = usize::try_from(*n).ok().filter(|n| (1..=4).contains(n))
                .ok_or_else(|| err(self.at, "unsupported relocation MAU count"))?;
            self.tokens.pop();
            Ok(count)
        } else {
            Ok(1)
        }
    }

    pub(super) fn section(&self) -> Option<Id> {
        match self.tokens.first()? {
            Token::Variable(Variable::Base(id) | Variable::Relative(id)) => Some(*id),
            _ => None,
        }
    }

    pub(super) fn bind(&mut self, mut resolve: impl FnMut(Variable) -> KunaResult<Option<i64>>) -> KunaResult<()> {
        for token in &mut self.tokens {
            let Token::Variable(var) = *token else { continue; };
            if let Some(value) = resolve(var)? {
                *token = Token::Number(value);
            }
        }
        Ok(())
    }

    pub(super) fn externals(&self) -> impl Iterator<Item = Id> + '_ {
        self.tokens.iter().filter_map(|token| match *token {
            Token::Variable(Variable::External(id)) => Some(id),
            _ => None,
        })
    }

    pub(super) fn has_external(&self) -> bool {
        self.externals().next().is_some()
    }

    /// The original long CALL check compares target-PC to signed 24-bit
    /// bounds before scattering its bits over the two instruction words.
    /// Keep synthetic import addresses inside the intersection of those bounds.
    pub(super) fn displacement(&self) -> KunaResult<Option<(Id, i64, i64)>> {
        use Token::{Number, Op};
        let [Number(opcode), Token::Variable(Variable::External(id)), rest @ ..] = &self.tokens[..] else { return Ok(None); };
        if (opcode >> 24) & 0xff0000 != 0xcf0000 { return Ok(None); }
        let Some((i, _)) = rest.iter().enumerate().find(|(_, token)| matches!(token, Op(0xa6))) else { return Ok(None); };
        let [Op(0xa6), Op(0x93), Op(0x93), Number(max), Op(0xad), Op(0x94), Number(min), Op(0xac), Op(0xb1), Number(3), Op(0xb5), ..] = &rest[i..] else { return Ok(None); };
        if *min > *max { return Err(err(self.at, "invalid relocation displacement bounds")); }
        let pc = eval(&rest[..i], self.at, |_| Err(err(self.at, "unsupported symbolic call origin")))?;
        let low = pc.checked_add(*min).ok_or_else(|| err(self.at, "relocation displacement overflow"))?;
        let high = pc.checked_add(*max).ok_or_else(|| err(self.at, "relocation displacement overflow"))?;
        Ok(Some((*id, low, high)))
    }

    pub(super) fn eval(&self, resolve: impl FnMut(Variable) -> KunaResult<i64>) -> KunaResult<i64> {
        eval(&self.tokens, self.at, resolve)
    }

    pub(super) fn encoded(&self, n: usize, swap: bool, mut external: impl FnMut(Id, usize) -> KunaResult<i64>) -> KunaResult<[u8; 8]> {
        let value = self.eval(|var| match var {
            Variable::External(id) => external(id, self.at),
            _ => Err(err(self.at, "unbound relocation variable")),
        })?;
        if n < 8 && (i128::from(value) < -(1_i128 << (8 * n - 1)) || i128::from(value) >= (1_i128 << (8 * n))) {
            return Err(err(self.at, "relocation value does not fit its field"));
        }
        let mut bytes = value.to_be_bytes();
        bytes.copy_within(8 - n.., 0);
        if swap {
            bytes[..6].rotate_left(3);
        }
        Ok(bytes)
    }
}

fn eval(tokens: &[Token], at: usize, mut resolve: impl FnMut(Variable) -> KunaResult<i64>) -> KunaResult<i64> {
    // All standard producer expressions seen here fit in eight stack slots.
    // Bound the stack explicitly instead of allocating once per expression.
    let mut stack = [0; 8];
    let mut len = 0_usize;
    let underflow = || err(at, "relocation expression stack underflow");
    for token in tokens {
        let value = match *token {
            Token::Number(n) => n,
            Token::Variable(var) => resolve(var)?,
            Token::Op(op) => {
                if op == 0x93 { // TASKING duplicate
                    *stack.get(len.checked_sub(1).ok_or_else(underflow)?).ok_or_else(underflow)?
                } else if op == 0x94 { // TASKING exchange
                    let prev = len.checked_sub(2).ok_or_else(underflow)?;
                    stack.swap(prev, prev + 1);
                    continue;
                } else if op == 0xb5 { // @ERR: value, condition, diagnostic
                    let first = len.checked_sub(3).ok_or_else(underflow)?;
                    if stack[first + 1] != 0 {
                        return Err(err(at, format_args!("relocation range check failed (TASKING diagnostic {})", stack[first + 2])));
                    }
                    len = first;
                    stack[first]
                } else {
                    let count = match op { 0xa0 | 0xa1 => 0, 0xa2..=0xa4 => 1, 0xb3 => 3, 0xb4 => 4, _ => 2 };
                    let first = len.checked_sub(count).ok_or_else(underflow)?;
                    let args = &stack[first..len];
                    len = first;
                    operation(op, args).ok_or_else(|| err(at, format_args!("invalid operands or overflow for expression function {op:#04x}")))?
                }
            }
        };
        *stack.get_mut(len).ok_or_else(|| err(at, "unsupported relocation expression stack depth"))? = value;
        len += 1;
    }
    if len != 1 { return Err(err(at, "expression does not produce exactly one value")); }
    Ok(stack[0])
}

fn operation(op: u8, args: &[i64]) -> Option<i64> {
    let unary = || args[0];
    let binary = || (args[0], args[1]);
    Some(match op {
        0xa0 => 0,
        0xa1 => 1,
        0xa2 => unary().checked_abs()?,
        0xa3 => unary().checked_neg()?,
        0xa4 => !unary(),
        0xb3 | 0xb4 => {
            let lo = u32::try_from(args[args.len() - 2]).ok()?;
            let hi = u32::try_from(args[args.len() - 1]).ok()?;
            if lo > hi || hi >= 64 { return None; }
            let mask = u64::MAX >> (63 - (hi - lo));
            if op == 0xb3 { ((args[0] as u64 >> lo) & mask) as i64 }
            else { ((args[0] as u64 & !(mask << lo)) | ((args[1] as u64 & mask) << lo)) as i64 }
        }
        _ => {
            let (a, b) = binary();
            match op {
                0x90 => {
                    let shift = u32::try_from(b).ok().filter(|&b| b < 64)?;
                    i64::try_from(i128::from(a) << shift).ok()?
                }
                0x91 => a.checked_shr(u32::try_from(b).ok()?)?,
                0xa5 => a.checked_add(b)?,
                0xa6 => a.checked_sub(b)?,
                0xa7 => a.checked_div(b)?,
                0xa8 => a.checked_mul(b)?,
                0xa9 => a.max(b),
                0xaa => a.min(b),
                0xab if a >= 0 && b > 0 => a % b,
                0xac => i64::from(a < b),
                0xad => i64::from(a > b),
                0xae => i64::from(a == b),
                0xaf => i64::from(a != b),
                0xb0 => a & b,
                0xb1 => a | b,
                0xb2 => a ^ b,
                _ => return None,
            }
        }
    })
}

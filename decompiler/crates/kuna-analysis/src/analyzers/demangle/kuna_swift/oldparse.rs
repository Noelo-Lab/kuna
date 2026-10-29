//! (kuna `swiftdemangle`) The Swift 3 (`_Tt`) type mangling, which the
//! Objective-C runtime still uses for Swift class and protocol names
//! (`_TtC6Toyota24AddMileageViewController`). Only the type subset that
//! can appear in a runtime name is read: nominal types with nested and
//! private (`P33_<hash>`) contexts, protocol compositions, bound generics,
//! the known stdlib types and back-reference substitutions. The result is
//! the same tree the `$s` parser builds, so both printers apply.

use std::rc::Rc;

use super::node::{Kind, Node, NodeRef};
use super::parse::{mk_type, swift_type};

struct OldParser<'a> {
    text: &'a [u8],
    pos: usize,
    subs: Vec<NodeRef>,
}

/// Parse the body after `_Tt` into a `Type` node; `None` when anything
/// outside the subset appears or input remains.
pub fn parse_old_type(body: &str) -> Option<NodeRef> {
    let mut p = OldParser {
        text: body.as_bytes(),
        pos: 0,
        subs: Vec::new(),
    };
    let ty = p.ty()?;
    if p.pos != p.text.len() {
        return None;
    }
    Some(ty)
}

impl<'a> OldParser<'a> {
    fn peek(&self) -> u8 {
        self.text.get(self.pos).copied().unwrap_or(0)
    }

    fn next(&mut self) -> u8 {
        let c = self.peek();
        if c != 0 {
            self.pos += 1;
        }
        c
    }

    fn next_if(&mut self, c: u8) -> bool {
        if c != 0 && self.peek() == c {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn natural(&mut self) -> Option<usize> {
        if !self.peek().is_ascii_digit() {
            return None;
        }
        let mut n: usize = 0;
        while self.peek().is_ascii_digit() {
            n = n
                .checked_mul(10)?
                .checked_add((self.next() - b'0') as usize)?;
        }
        Some(n)
    }

    /// `NATURAL IDENTIFIER-STRING`; punycode and operator names refuse.
    fn identifier(&mut self) -> Option<String> {
        let len = self.natural()?;
        if len == 0 || self.pos + len > self.text.len() {
            return None;
        }
        let s = std::str::from_utf8(&self.text[self.pos..self.pos + len]).ok()?;
        if !s
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'$')
            || s.as_bytes()[0].is_ascii_digit()
        {
            return None;
        }
        self.pos += len;
        Some(s.to_string())
    }

    fn substitution(&mut self) -> Option<NodeRef> {
        // After the `S`: `_` is index 0, `N_` is index N+1, a letter is a
        // known module or type.
        if self.next_if(b'_') {
            return self.subs.first().cloned();
        }
        if self.peek().is_ascii_digit() {
            let n = self.natural()?;
            if !self.next_if(b'_') {
                return None;
            }
            return self.subs.get(n.checked_add(1)?).cloned();
        }
        Some(match self.next() {
            b's' => Node::text(Kind::Module, "Swift"),
            b'o' => Node::text(Kind::Module, "__C"),
            b'C' => Node::text(Kind::Module, "__C_Synthesized"),
            b'a' => swift_type(Kind::Structure, "Array"),
            b'b' => swift_type(Kind::Structure, "Bool"),
            b'c' => swift_type(Kind::Structure, "UnicodeScalar"),
            b'd' => swift_type(Kind::Structure, "Double"),
            b'f' => swift_type(Kind::Structure, "Float"),
            b'i' => swift_type(Kind::Structure, "Int"),
            b'P' => swift_type(Kind::Structure, "UnsafePointer"),
            b'p' => swift_type(Kind::Structure, "UnsafeMutablePointer"),
            b'Q' => swift_type(Kind::Enum, "ImplicitlyUnwrappedOptional"),
            b'q' => swift_type(Kind::Enum, "Optional"),
            b'R' => swift_type(Kind::Structure, "UnsafeBufferPointer"),
            b'r' => swift_type(Kind::Structure, "UnsafeMutableBufferPointer"),
            b'S' => swift_type(Kind::Structure, "String"),
            b'u' => swift_type(Kind::Structure, "UInt"),
            _ => return None,
        })
    }

    /// `context ::= module | nominal-type | 'E' module context`.
    fn context(&mut self) -> Option<NodeRef> {
        match self.peek() {
            b'E' => {
                self.next();
                let module = match self.peek() {
                    b's' => {
                        self.next();
                        Node::text(Kind::Module, "Swift")
                    }
                    b'S' => {
                        self.next();
                        let s = self.substitution()?;
                        if s.kind != Kind::Module {
                            return None;
                        }
                        s
                    }
                    _ => {
                        let m = Node::text(Kind::Module, self.identifier()?);
                        self.subs.push(Rc::clone(&m));
                        m
                    }
                };
                let ty = self.context()?;
                Some(Node::with(Kind::Extension, vec![module, ty]))
            }
            b'S' => {
                self.next();
                let s = self.substitution()?;
                if s.kind == Kind::Type {
                    return Some(Rc::clone(s.first()?));
                }
                Some(s)
            }
            b's' => {
                self.next();
                Some(Node::text(Kind::Module, "Swift"))
            }
            b'C' | b'V' | b'O' | b'P' | b'G' => {
                let t = self.ty()?;
                Some(Rc::clone(t.first()?))
            }
            c if c.is_ascii_digit() => {
                let m = Node::text(Kind::Module, self.identifier()?);
                self.subs.push(Rc::clone(&m));
                Some(m)
            }
            _ => None,
        }
    }

    /// A protocol name, where a substitution may stand for the whole
    /// protocol rather than its context.
    fn protocol_name(&mut self) -> Option<NodeRef> {
        if self.peek() == b'S' {
            self.next();
            let s = self.substitution()?;
            if s.kind == Kind::Type {
                if s.first()?.kind != Kind::Protocol {
                    return None;
                }
                return Some(s);
            }
            if s.kind != Kind::Module {
                return None;
            }
            return self.nominal_in(Kind::Protocol, s);
        }
        self.nominal(Kind::Protocol)
    }

    /// `decl-name ::= context identifier | context 'P' identifier identifier`.
    fn nominal(&mut self, kind: Kind) -> Option<NodeRef> {
        let ctx = self.context()?;
        self.nominal_in(kind, ctx)
    }

    fn nominal_in(&mut self, kind: Kind, ctx: NodeRef) -> Option<NodeRef> {
        let name = if self.next_if(b'P') {
            let discriminator = Node::text(Kind::Identifier, self.identifier()?);
            let name = Node::text(Kind::Identifier, self.identifier()?);
            Node::with(Kind::PrivateDeclName, vec![discriminator, name])
        } else {
            Node::text(Kind::Identifier, self.identifier()?)
        };
        let ty = mk_type(Node::with(kind, vec![ctx, name]));
        self.subs.push(Rc::clone(&ty));
        Some(ty)
    }

    fn ty(&mut self) -> Option<NodeRef> {
        match self.next() {
            b'C' => self.nominal(Kind::Class),
            b'V' => self.nominal(Kind::Structure),
            b'O' => self.nominal(Kind::Enum),
            b'P' => {
                // A protocol composition: `P` protocol* `_`; a single
                // protocol prints as the bare protocol.
                let mut protos = Vec::new();
                while !self.next_if(b'_') {
                    if self.peek() == 0 {
                        return None;
                    }
                    protos.push(self.protocol_name()?);
                }
                if protos.len() == 1 {
                    return protos.pop();
                }
                Some(mk_type(Node::with(
                    Kind::ProtocolList,
                    vec![Node::with(Kind::TypeList, protos)],
                )))
            }
            b'G' => {
                let base = self.ty()?;
                let nominal = Rc::clone(base.first()?);
                let mut args = Vec::new();
                while !self.next_if(b'_') {
                    if self.peek() == 0 {
                        return None;
                    }
                    args.push(self.ty()?);
                }
                let kind = match nominal.kind {
                    Kind::Class => Kind::BoundGenericClass,
                    Kind::Structure => Kind::BoundGenericStructure,
                    Kind::Enum => Kind::BoundGenericEnum,
                    _ => return None,
                };
                Some(mk_type(Node::with(
                    kind,
                    vec![base, Node::with(Kind::TypeList, args)],
                )))
            }
            b'S' => {
                let s = self.substitution()?;
                if s.kind != Kind::Type {
                    return None;
                }
                Some(s)
            }
            _ => None,
        }
    }
}

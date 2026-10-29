//! (kuna `swiftdemangle`) The Swift 5 (`$s`) stable-mangling parser: a
//! postfix stack machine over the operators of `Mangling.rst`. Each
//! operator pops its operands from the node stack and pushes one node; the
//! stack left at the end becomes the `Global` node. Any operator or shape
//! outside the implemented grammar fails the whole parse (`None`), which
//! the callers turn into passthrough of the mangled spelling.

use std::rc::Rc;

use super::node::{Kind, Node, NodeRef};
use super::punycode;

const MAX_REPEAT: u64 = 2048;
const MAX_WORDS: usize = 26;
const MAX_NODES: usize = 1 << 16;

pub struct Parser<'a> {
    text: &'a [u8],
    pos: usize,
    stack: Vec<NodeRef>,
    subs: Vec<NodeRef>,
    words: Vec<String>,
    old_fn_types: bool,
    budget: usize,
    failed: bool,
}

pub(super) fn swift_type(kind: Kind, name: &str) -> NodeRef {
    mk_type(Node::with(
        kind,
        vec![
            Node::text(Kind::Module, "Swift"),
            Node::text(Kind::Identifier, name),
        ],
    ))
}

pub(super) fn mk_type(child: NodeRef) -> NodeRef {
    Node::with(Kind::Type, vec![child])
}

fn is_protocol_type(n: &Node) -> bool {
    n.kind == Kind::Type && n.first().is_some_and(|c| c.kind == Kind::Protocol)
}

impl<'a> Parser<'a> {
    /// Parse `body` (the text after the mangling prefix). `old_fn_types` is
    /// the Swift 4.0 `_T0` convention (labels inside the function type).
    pub fn new(body: &'a str, old_fn_types: bool) -> Self {
        Parser {
            text: body.as_bytes(),
            pos: 0,
            stack: Vec::new(),
            subs: Vec::new(),
            words: Vec::new(),
            old_fn_types,
            budget: MAX_NODES,
            failed: false,
        }
    }

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
        if self.peek() == c && c != 0 {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn push_back(&mut self) {
        self.pos = self.pos.saturating_sub(1);
    }

    fn at_end(&self) -> bool {
        self.pos >= self.text.len()
    }

    fn tick(&mut self) -> Option<()> {
        self.budget = self.budget.checked_sub(1)?;
        Some(())
    }

    fn push(&mut self, n: NodeRef) {
        if self.budget == 0 {
            self.stack.clear();
            self.pos = self.text.len();
            self.failed = true;
            return;
        }
        self.budget -= 1;
        self.stack.push(n);
    }

    fn pop(&mut self) -> Option<NodeRef> {
        self.stack.pop()
    }

    fn pop_kind(&mut self, kind: Kind) -> Option<NodeRef> {
        if self.stack.last()?.kind == kind {
            self.stack.pop()
        } else {
            None
        }
    }

    fn pop_if(&mut self, pred: impl Fn(Kind) -> bool) -> Option<NodeRef> {
        if pred(self.stack.last()?.kind) {
            self.stack.pop()
        } else {
            None
        }
    }

    fn add_sub(&mut self, n: &NodeRef) {
        self.subs.push(Rc::clone(n));
    }

    // ---- numbers -------------------------------------------------------

    /// `NATURAL ::= [1-9][0-9]*`; `None` when no digit follows (a leading
    /// `0` is not a natural).
    fn natural(&mut self) -> Option<u64> {
        if !self.peek().is_ascii_digit() {
            return None;
        }
        let mut n: u64 = 0;
        while self.peek().is_ascii_digit() {
            n = n
                .checked_mul(10)?
                .checked_add((self.next() - b'0') as u64)?;
        }
        Some(n)
    }

    /// `INDEX ::= '_' | NATURAL '_'`, decoded to 0 / N+1.
    fn index(&mut self) -> Option<u64> {
        if self.next_if(b'_') {
            return Some(0);
        }
        let n = self.natural()?;
        if self.next_if(b'_') {
            n.checked_add(1)
        } else {
            None
        }
    }

    fn index_node(&mut self) -> Option<NodeRef> {
        Some(Node::index(Kind::Number, self.index()?))
    }

    // ---- identifiers ---------------------------------------------------

    fn identifier(&mut self) -> Option<NodeRef> {
        let mut has_word_substs = false;
        let mut is_punycoded = false;
        if !self.peek().is_ascii_digit() {
            return None;
        }
        if self.peek() == b'0' {
            self.next();
            if self.peek() == b'0' {
                self.next();
                is_punycoded = true;
            } else {
                has_word_substs = true;
            }
        }
        let mut ident = String::new();
        loop {
            while has_word_substs && self.peek().is_ascii_alphabetic() {
                let c = self.next();
                let idx = if c.is_ascii_lowercase() {
                    (c - b'a') as usize
                } else {
                    has_word_substs = false;
                    (c - b'A') as usize
                };
                if idx >= self.words.len() {
                    return None;
                }
                ident.push_str(&self.words[idx]);
            }
            if self.next_if(b'0') {
                break;
            }
            let num = self.natural()? as usize;
            if num == 0 {
                return None;
            }
            if is_punycoded {
                self.next_if(b'_');
            }
            if self.pos + num > self.text.len() {
                return None;
            }
            let slice = &self.text[self.pos..self.pos + num];
            if is_punycoded {
                let decoded = punycode::decode_swift(std::str::from_utf8(slice).ok()?)?;
                ident.push_str(&decoded);
            } else {
                let s = std::str::from_utf8(slice).ok()?;
                if !s
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'$')
                {
                    return None;
                }
                ident.push_str(s);
                self.collect_words(s);
            }
            self.pos += num;
            if !has_word_substs {
                break;
            }
        }
        if ident.is_empty() {
            return None;
        }
        let n = Node::text(Kind::Identifier, ident);
        self.add_sub(&n);
        Some(n)
    }

    /// Record the words of a literal identifier slice for later `0…`
    /// word substitutions: letter/digit runs split at `_` and at an
    /// uppercase letter that follows a non-uppercase character.
    fn collect_words(&mut self, s: &str) {
        let b = s.as_bytes();
        let mut start: Option<usize> = None;
        for i in 0..=b.len() {
            let c = if i < b.len() { b[i] } else { 0 };
            if let Some(st) = start {
                let prev = b[i - 1];
                let word_end =
                    c == b'_' || c == 0 || (!prev.is_ascii_uppercase() && c.is_ascii_uppercase());
                if word_end {
                    if i - st >= 2 && self.words.len() < MAX_WORDS {
                        self.words.push(s[st..i].to_string());
                    }
                    start = None;
                }
            }
            if start.is_none() && c != 0 && c != b'_' && !c.is_ascii_digit() {
                start = Some(i);
            }
        }
    }

    fn operator_identifier(&mut self) -> Option<NodeRef> {
        let ident = self.pop_kind(Kind::Identifier)?;
        let mut op = String::new();
        for c in ident.get_text().chars() {
            if !c.is_ascii() {
                op.push(c);
                continue;
            }
            let o = match c {
                'a' => '&',
                'c' => '@',
                'd' => '/',
                'e' => '=',
                'g' => '>',
                'l' => '<',
                'm' => '*',
                'n' => '!',
                'o' => '|',
                'p' => '+',
                'q' => '?',
                'r' => '%',
                's' => '-',
                't' => '~',
                'x' => '^',
                'z' => '.',
                _ => return None,
            };
            op.push(o);
        }
        let kind = match self.next() {
            b'i' => Kind::InfixOperator,
            b'p' => Kind::PrefixOperator,
            b'P' => Kind::PostfixOperator,
            _ => return None,
        };
        Some(Node::text(kind, op))
    }

    fn local_identifier(&mut self) -> Option<NodeRef> {
        if self.next_if(b'L') {
            let discriminator = self.pop_kind(Kind::Identifier)?;
            let name = self.pop_if(Kind::is_decl_name)?;
            return Some(Node::with(Kind::PrivateDeclName, vec![discriminator, name]));
        }
        if self.next_if(b'l') {
            let discriminator = self.pop_kind(Kind::Identifier)?;
            return Some(Node::with(Kind::PrivateDeclName, vec![discriminator]));
        }
        let c = self.peek();
        if (b'a'..=b'j').contains(&c) || (b'A'..=b'J').contains(&c) {
            self.next();
            let kind = Node::text(Kind::Identifier, (c as char).to_string());
            let name = self.pop()?;
            return Some(Node::with(Kind::RelatedEntityDeclName, vec![kind, name]));
        }
        let discriminator = self.index_node()?;
        let name = self.pop_if(Kind::is_decl_name)?;
        Some(Node::with(Kind::LocalDeclName, vec![discriminator, name]))
    }

    // ---- substitutions -------------------------------------------------

    fn multi_substitutions(&mut self) -> Option<NodeRef> {
        let mut repeat: Option<u64> = None;
        loop {
            let c = self.next();
            if c == 0 {
                return None;
            }
            if c.is_ascii_lowercase() {
                let n = self.push_multi_substitutions(repeat, (c - b'a') as usize)?;
                self.push(n);
                repeat = None;
                continue;
            }
            if c.is_ascii_uppercase() {
                return self.push_multi_substitutions(repeat, (c - b'A') as usize);
            }
            if c == b'_' {
                let idx = repeat.map_or(26, |r| r as usize + 27);
                return self.subs.get(idx).cloned();
            }
            self.push_back();
            repeat = Some(self.natural()?);
        }
    }

    fn push_multi_substitutions(&mut self, repeat: Option<u64>, idx: usize) -> Option<NodeRef> {
        let n = self.subs.get(idx).cloned()?;
        let mut count = repeat.unwrap_or(1);
        if count > MAX_REPEAT {
            return None;
        }
        while count > 1 {
            self.push(Rc::clone(&n));
            count -= 1;
        }
        Some(n)
    }

    fn standard_substitution(&mut self) -> Option<NodeRef> {
        match self.next() {
            b'o' => Some(Node::text(Kind::Module, "__C")),
            b'C' => Some(Node::text(Kind::Module, "__C_Synthesized")),
            b'g' => {
                let arg = self.pop_kind(Kind::Type)?;
                let opt = mk_type(Node::with(
                    Kind::BoundGenericEnum,
                    vec![
                        swift_type(Kind::Enum, "Optional"),
                        Node::with(Kind::TypeList, vec![arg]),
                    ],
                ));
                self.add_sub(&opt);
                Some(opt)
            }
            _ => {
                self.push_back();
                let repeat = self.natural();
                if repeat.is_some_and(|r| r > MAX_REPEAT) {
                    return None;
                }
                let second = self.next_if(b'c');
                let c = self.next();
                let (kind, name) = known_type(c, second)?;
                let n = swift_type(kind, name);
                let mut count = repeat.unwrap_or(1);
                while count > 1 {
                    self.push(Rc::clone(&n));
                    count -= 1;
                }
                Some(n)
            }
        }
    }

    // ---- contexts ------------------------------------------------------

    fn pop_context(&mut self) -> Option<NodeRef> {
        if let Some(m) = self.pop_module() {
            return Some(m);
        }
        if let Some(ty) = self.pop_kind(Kind::Type) {
            if ty.len() != 1 {
                return None;
            }
            let child = Rc::clone(ty.first()?);
            if !child.kind.is_context() {
                return None;
            }
            return Some(child);
        }
        self.pop_if(Kind::is_context)
    }

    fn pop_module(&mut self) -> Option<NodeRef> {
        if let Some(ident) = self.pop_kind(Kind::Identifier) {
            return Some(ident.rekind(Kind::Module));
        }
        self.pop_kind(Kind::Module)
    }

    fn pop_type_and_get_child(&mut self) -> Option<NodeRef> {
        let ty = self.pop_kind(Kind::Type)?;
        if ty.len() != 1 {
            return None;
        }
        Some(Rc::clone(ty.first()?))
    }

    fn pop_type_and_get_any_generic(&mut self) -> Option<NodeRef> {
        let child = self.pop_type_and_get_child()?;
        if child.kind.is_any_generic() {
            Some(child)
        } else {
            None
        }
    }

    fn any_generic_type(&mut self, kind: Kind) -> Option<NodeRef> {
        let name = self.pop_if(Kind::is_decl_name)?;
        let ctx = self.pop_context()?;
        let ty = mk_type(Node::with(kind, vec![ctx, name]));
        self.add_sub(&ty);
        Some(ty)
    }

    fn extension_context(&mut self) -> Option<NodeRef> {
        let gen_sig = self.pop_kind(Kind::DependentGenericSignature);
        let module = self.pop_module()?;
        let ty = self.pop_type_and_get_any_generic()?;
        let mut children = vec![module, ty];
        children.extend(gen_sig);
        Some(Node::with(Kind::Extension, children))
    }

    // ---- function types ------------------------------------------------

    fn clang_type(&mut self) -> Option<NodeRef> {
        let num = self.natural()? as usize;
        if num == 0 || self.pos + num > self.text.len() {
            return None;
        }
        let s = std::str::from_utf8(&self.text[self.pos..self.pos + num])
            .ok()?
            .to_string();
        self.pos += num;
        Some(Node::text(Kind::ClangType, s))
    }

    fn pop_function_type(&mut self, kind: Kind, has_clang_type: bool) -> Option<NodeRef> {
        let mut children = Vec::new();
        if has_clang_type {
            children.push(self.clang_type()?);
        }
        for k in [
            Kind::SendingResultFunctionType,
            Kind::GlobalActorFunctionType,
            Kind::IsolatedAnyFunctionType,
            Kind::NonIsolatedCallerFunctionType,
            Kind::DifferentiableFunctionType,
            Kind::ThrowsAnnotation,
            Kind::TypedThrowsAnnotation,
            Kind::ConcurrentFunctionType,
            Kind::AsyncAnnotation,
        ] {
            if let Some(n) = self.pop_kind(k) {
                children.push(n);
            }
        }
        children.push(self.pop_function_params(Kind::ArgumentTuple)?);
        children.push(self.pop_function_params(Kind::ReturnType)?);
        Some(mk_type(Node::with(kind, children)))
    }

    fn pop_function_params(&mut self, kind: Kind) -> Option<NodeRef> {
        let params = if self.pop_kind(Kind::EmptyList).is_some() {
            mk_type(Node::new(Kind::Tuple))
        } else {
            self.pop_kind(Kind::Type)?
        };
        Some(Node::with(kind, vec![params]))
    }

    /// The label list of an entity whose type is `ty` (a function type):
    /// `Some(LabelList)` when labels (possibly an explicit empty list) were
    /// mangled, `None` when the entity carries none. A label-run that does
    /// not consist of identifiers / `_` markers is an `Err`-like failure
    /// reported through the returned `Result`.
    fn pop_function_param_labels(&mut self, ty: Option<&NodeRef>) -> Result<Option<NodeRef>, ()> {
        if !self.old_fn_types && self.pop_kind(Kind::EmptyList).is_some() {
            return Ok(Some(Node::new(Kind::LabelList)));
        }
        let Some(ty) = ty else { return Ok(None) };
        if ty.kind != Kind::Type {
            return Ok(None);
        }
        let Some(mut func) = ty.first().cloned() else {
            return Ok(None);
        };
        if func.kind == Kind::DependentGenericType {
            let Some(inner) = func.child(1).and_then(|c| c.first()).cloned() else {
                return Ok(None);
            };
            func = inner;
        }
        if func.kind != Kind::FunctionType && func.kind != Kind::NoEscapeFunctionType {
            return Ok(None);
        }
        let Some(arg_tuple) = func.child_of_kind(Kind::ArgumentTuple) else {
            return Err(());
        };
        let Some(params_ty) = arg_tuple.first() else {
            return Err(());
        };
        let Some(params) = params_ty.first() else {
            return Err(());
        };
        let num_params = if params.kind == Kind::Tuple {
            params.len()
        } else {
            1
        };
        if num_params == 0 {
            return Ok(None);
        }
        let mut labels = Vec::new();
        let mut has_labels = false;
        if self.old_fn_types {
            if params.kind != Kind::Tuple {
                return Ok(Some(Node::new(Kind::LabelList)));
            }
            for p in &params.children {
                match p.child_of_kind(Kind::TupleElementName) {
                    Some(l) => {
                        has_labels = true;
                        labels.push(Node::text(Kind::Identifier, l.get_text()));
                    }
                    None => labels.push(Node::new(Kind::FirstElementMarker)),
                }
            }
        } else {
            for _ in 0..num_params {
                let Some(label) = self.pop() else {
                    return Err(());
                };
                if label.kind != Kind::Identifier && label.kind != Kind::FirstElementMarker {
                    return Err(());
                }
                has_labels |= label.kind != Kind::FirstElementMarker;
                labels.push(label);
            }
            labels.reverse();
        }
        if !has_labels {
            return Ok(Some(Node::new(Kind::LabelList)));
        }
        Ok(Some(Node::with(Kind::LabelList, labels)))
    }

    fn plain_function(&mut self, kind: Kind) -> Option<NodeRef> {
        let gen_sig = self.pop_kind(Kind::DependentGenericSignature);
        let mut ty = self.pop_function_type(Kind::FunctionType, false)?;
        let labels = self.pop_function_param_labels(Some(&ty)).ok()?;
        if let Some(sig) = gen_sig {
            ty = mk_type(Node::with(Kind::DependentGenericType, vec![sig, ty]));
        }
        let name = self.pop_if(Kind::is_decl_name)?;
        let ctx = self.pop_context()?;
        let mut children = vec![ctx, name];
        children.extend(labels);
        children.push(ty);
        Some(Node::with(kind, children))
    }

    fn entity(&mut self, kind: Kind) -> Option<NodeRef> {
        let ty = self.pop_kind(Kind::Type)?;
        let labels = self.pop_function_param_labels(Some(&ty)).ok()?;
        let name = self.pop_if(Kind::is_decl_name)?;
        let ctx = self.pop_context()?;
        let mut children = vec![ctx, name];
        children.extend(labels);
        children.push(ty);
        Some(Node::with(kind, children))
    }

    fn variable(&mut self) -> Option<NodeRef> {
        let var = self.entity(Kind::Variable)?;
        self.accessor(var)
    }

    fn subscript(&mut self) -> Option<NodeRef> {
        let private_name = self.pop_kind(Kind::PrivateDeclName);
        let ty = self.pop_kind(Kind::Type)?;
        let labels = self.pop_function_param_labels(Some(&ty)).ok()?;
        let ctx = self.pop_context()?;
        let mut children = vec![ctx];
        children.extend(labels);
        children.push(ty);
        children.extend(private_name);
        let sub = Node::with(Kind::Subscript, children);
        self.accessor(sub)
    }

    fn accessor(&mut self, storage: NodeRef) -> Option<NodeRef> {
        let kind = match self.next() {
            b'm' => Kind::MaterializeForSet,
            b's' => Kind::Setter,
            b'g' => Kind::Getter,
            b'G' => Kind::GlobalGetter,
            b'w' => Kind::WillSet,
            b'W' => Kind::DidSet,
            b'r' => Kind::ReadAccessor,
            b'y' => Kind::Read2Accessor,
            b'M' => Kind::ModifyAccessor,
            b'x' => Kind::Modify2Accessor,
            b'i' => Kind::InitAccessor,
            b'a' => match self.next() {
                b'O' => Kind::OwningMutableAddressor,
                b'o' => Kind::NativeOwningMutableAddressor,
                b'P' => Kind::NativePinningMutableAddressor,
                b'u' => Kind::UnsafeMutableAddressor,
                _ => return None,
            },
            b'l' => match self.next() {
                b'O' => Kind::OwningAddressor,
                b'o' => Kind::NativeOwningAddressor,
                b'p' => Kind::NativePinningAddressor,
                b'u' => Kind::UnsafeAddressor,
                _ => return None,
            },
            b'p' => return Some(storage),
            _ => return None,
        };
        Some(Node::with(kind, vec![storage]))
    }

    fn function_entity(&mut self) -> Option<NodeRef> {
        enum Args {
            None,
            TypeAndMaybePrivateName,
            TypeAndIndex,
            Index,
            ContextArg,
        }
        let (args, kind) = match self.next() {
            b'D' => (Args::None, Kind::Deallocator),
            b'd' => (Args::None, Kind::Destructor),
            b'E' => (Args::None, Kind::IVarDestroyer),
            b'e' => (Args::None, Kind::IVarInitializer),
            b'i' => (Args::None, Kind::Initializer),
            b'C' => (Args::TypeAndMaybePrivateName, Kind::Allocator),
            b'c' => (Args::TypeAndMaybePrivateName, Kind::Constructor),
            b'U' => (Args::TypeAndIndex, Kind::ExplicitClosure),
            b'u' => (Args::TypeAndIndex, Kind::ImplicitClosure),
            b'A' => (Args::Index, Kind::DefaultArgumentInitializer),
            b'p' => return self.entity(Kind::GenericTypeParamDecl),
            b'P' => (Args::None, Kind::PropertyWrapperBackingInitializer),
            b'W' => (Args::None, Kind::PropertyWrapperInitFromProjectedValue),
            b'Z' => (Args::None, Kind::IsolatedDeallocator),
            b'a' => (Args::ContextArg, Kind::RuntimeAttributeGenerator),
            _ => return None,
        };
        let mut name_or_index = None;
        let mut param_type = None;
        let mut labels = None;
        let mut context_arg = None;
        match args {
            Args::None => {}
            Args::TypeAndMaybePrivateName => {
                name_or_index = self.pop_kind(Kind::PrivateDeclName);
                param_type = Some(self.pop_kind(Kind::Type)?);
                labels = self.pop_function_param_labels(param_type.as_ref()).ok()?;
            }
            Args::TypeAndIndex => {
                name_or_index = Some(self.index_node()?);
                param_type = Some(self.pop_kind(Kind::Type)?);
            }
            Args::Index => {
                name_or_index = Some(self.index_node()?);
            }
            Args::ContextArg => {
                context_arg = Some(self.pop()?);
            }
        }
        let ctx = self.pop_context()?;
        let mut children = vec![ctx];
        match args {
            Args::None => {}
            Args::Index => children.push(name_or_index?),
            Args::TypeAndMaybePrivateName => {
                children.extend(labels);
                children.push(param_type?);
                children.extend(name_or_index);
            }
            Args::TypeAndIndex => {
                children.push(name_or_index?);
                children.push(param_type?);
            }
            Args::ContextArg => children.push(context_arg?),
        }
        Some(Node::with(kind, children))
    }

    // ---- types ---------------------------------------------------------

    fn bound_generic_type(&mut self) -> Option<NodeRef> {
        let mut retro: Vec<NodeRef> = Vec::new();
        while let Some(rc) = self.pop_kind(Kind::RetroactiveConformance) {
            retro.push(rc);
        }
        retro.reverse();
        let type_lists = self.bound_generics()?;
        let nominal = self.pop_type_and_get_any_generic()?;
        let mut bound = self.bound_generic_args(&nominal, &type_lists, 0)?;
        if !retro.is_empty() {
            bound = bound.plus(Node::with(Kind::TypeList, retro));
        }
        let ty = mk_type(bound);
        self.add_sub(&ty);
        Some(ty)
    }

    /// Pops `bound-generic-args` into one type list per nesting level,
    /// innermost first.
    fn bound_generics(&mut self) -> Option<Vec<NodeRef>> {
        let mut lists = Vec::new();
        loop {
            let mut tys = Vec::new();
            while let Some(ty) = self.pop_kind(Kind::Type) {
                tys.push(ty);
            }
            tys.reverse();
            lists.push(Node::with(Kind::TypeList, tys));
            if self.pop_kind(Kind::EmptyList).is_some() {
                break;
            }
            self.pop_kind(Kind::FirstElementMarker)?;
        }
        Some(lists)
    }

    fn bound_generic_args(
        &mut self,
        nominal: &NodeRef,
        lists: &[NodeRef],
        mut idx: usize,
    ) -> Option<NodeRef> {
        let consumes = !matches!(
            nominal.kind,
            Kind::Variable
                | Kind::Subscript
                | Kind::ImplicitClosure
                | Kind::ExplicitClosure
                | Kind::DefaultArgumentInitializer
                | Kind::Initializer
                | Kind::PropertyWrapperBackingInitializer
                | Kind::PropertyWrapperInitFromProjectedValue
                | Kind::Static
        );
        let context = Rc::clone(nominal.first()?);
        let args = lists.get(idx)?;
        if consumes {
            idx += 1;
        }
        let mut nominal = Rc::clone(nominal);
        if idx < lists.len() {
            let bound_parent = if context.kind == Kind::Extension {
                let inner = self.bound_generic_args(context.child(1)?, lists, idx)?;
                let mut ext = vec![Rc::clone(context.first()?), inner];
                ext.extend(context.children.iter().skip(2).cloned());
                Node::with(Kind::Extension, ext)
            } else {
                self.bound_generic_args(&context, lists, idx)?
            };
            let mut children = vec![bound_parent];
            children.extend(nominal.children.iter().skip(1).cloned());
            nominal = Node::with(nominal.kind, children);
        }
        if !consumes || args.is_empty() {
            return Some(nominal);
        }
        let kind = match nominal.kind {
            Kind::Class => Kind::BoundGenericClass,
            Kind::Structure => Kind::BoundGenericStructure,
            Kind::Enum => Kind::BoundGenericEnum,
            Kind::Protocol => Kind::BoundGenericProtocol,
            Kind::OtherNominalType => Kind::BoundGenericOtherNominalType,
            Kind::TypeAlias => Kind::BoundGenericTypeAlias,
            Kind::Function | Kind::Constructor => Kind::BoundGenericFunction,
            _ => return None,
        };
        Some(Node::with(kind, vec![mk_type(nominal), Rc::clone(args)]))
    }

    fn tuple(&mut self) -> Option<NodeRef> {
        let mut elems = Vec::new();
        if self.pop_kind(Kind::EmptyList).is_none() {
            loop {
                let first = self.pop_kind(Kind::FirstElementMarker).is_some();
                let mut elem = Vec::new();
                if let Some(v) = self.pop_kind(Kind::VariadicMarker) {
                    elem.push(v);
                }
                if let Some(ident) = self.pop_kind(Kind::Identifier) {
                    elem.push(Node::text(Kind::TupleElementName, ident.get_text()));
                }
                elem.push(self.pop_kind(Kind::Type)?);
                elems.push(Node::with(Kind::TupleElement, elem));
                if first {
                    break;
                }
            }
            elems.reverse();
        }
        Some(mk_type(Node::with(Kind::Tuple, elems)))
    }

    fn pop_protocol(&mut self) -> Option<NodeRef> {
        if let Some(ty) = self.pop_kind(Kind::Type) {
            if !is_protocol_type(&ty) {
                return None;
            }
            return Some(ty);
        }
        let name = self.pop_if(Kind::is_decl_name)?;
        let ctx = self.pop_context()?;
        Some(mk_type(Node::with(Kind::Protocol, vec![ctx, name])))
    }

    fn protocol_list(&mut self) -> Option<NodeRef> {
        let mut protos = Vec::new();
        if self.pop_kind(Kind::EmptyList).is_none() {
            loop {
                let first = self.pop_kind(Kind::FirstElementMarker).is_some();
                protos.push(self.pop_protocol()?);
                if first {
                    break;
                }
            }
            protos.reverse();
        }
        Some(Node::with(
            Kind::ProtocolList,
            vec![Node::with(Kind::TypeList, protos)],
        ))
    }

    fn pop_type_list(&mut self) -> Option<NodeRef> {
        let mut tys = Vec::new();
        if self.pop_kind(Kind::EmptyList).is_none() {
            loop {
                let first = self.pop_kind(Kind::FirstElementMarker).is_some();
                tys.push(self.pop_kind(Kind::Type)?);
                if first {
                    break;
                }
            }
            tys.reverse();
        }
        Some(Node::with(Kind::TypeList, tys))
    }

    fn builtin_type(&mut self) -> Option<NodeRef> {
        let name = match self.next() {
            b'b' => "Builtin.BridgeObject".to_string(),
            b'B' => "Builtin.UnsafeValueBuffer".to_string(),
            b'c' => "Builtin.RawUnsafeContinuation".to_string(),
            b'D' => "Builtin.DefaultActorStorage".to_string(),
            b'd' => "Builtin.NonDefaultDistributedActorStorage".to_string(),
            b'e' => "Builtin.Executor".to_string(),
            b'f' => {
                let size = self.index()?.checked_sub(1)?;
                if size == 0 || size > 4096 {
                    return None;
                }
                format!("Builtin.FPIEEE{size}")
            }
            b'i' => {
                let size = self.index()?.checked_sub(1)?;
                if size == 0 || size > 4096 {
                    return None;
                }
                format!("Builtin.Int{size}")
            }
            b'I' => "Builtin.IntLiteral".to_string(),
            b'j' => "Builtin.Job".to_string(),
            b'P' => "Builtin.PackIndex".to_string(),
            b'O' => "Builtin.UnknownObject".to_string(),
            b'o' => "Builtin.NativeObject".to_string(),
            b'p' => "Builtin.RawPointer".to_string(),
            b't' => "Builtin.SILToken".to_string(),
            b'w' => "Builtin.Word".to_string(),
            b'A' => "Builtin.ImplicitActor".to_string(),
            b'v' => {
                let elts = self.index()?.checked_sub(1)?;
                if elts == 0 || elts > 4096 {
                    return None;
                }
                let elt = self.pop_type_and_get_child()?;
                if elt.kind != Kind::BuiltinTypeName {
                    return None;
                }
                let inner = elt.get_text().strip_prefix("Builtin.")?;
                format!("Builtin.Vec{elts}x{inner}")
            }
            b'V' => {
                let element = self.pop_kind(Kind::Type)?;
                let size = self.pop_kind(Kind::Type)?;
                return Some(mk_type(Node::with(
                    Kind::BuiltinFixedArray,
                    vec![size, element],
                )));
            }
            _ => return None,
        };
        Some(mk_type(Node::text(Kind::BuiltinTypeName, name)))
    }

    fn metatype_representation(&mut self) -> Option<NodeRef> {
        let text = match self.next() {
            b't' => "@thin",
            b'T' => "@thick",
            b'o' => "@objc_metatype",
            _ => return None,
        };
        Some(Node::text(Kind::MetatypeRepresentation, text))
    }

    fn special_type(&mut self) -> Option<NodeRef> {
        Some(match self.next() {
            b'E' => return self.pop_function_type(Kind::NoEscapeFunctionType, false),
            b'A' => return self.pop_function_type(Kind::EscapingAutoClosureType, false),
            b'K' => return self.pop_function_type(Kind::AutoClosureType, false),
            b'f' => return self.pop_function_type(Kind::ThinFunctionType, false),
            b'U' => return self.pop_function_type(Kind::UncurriedFunctionType, false),
            b'C' => return self.pop_function_type(Kind::CFunctionPointer, false),
            b'B' => return self.pop_function_type(Kind::ObjCBlock, false),
            b'L' => return self.pop_function_type(Kind::EscapingObjCBlock, false),
            b'z' => match self.next() {
                b'B' => return self.pop_function_type(Kind::ObjCBlock, true),
                b'C' => return self.pop_function_type(Kind::CFunctionPointer, true),
                _ => return None,
            },
            b'o' => mk_type(Node::with(Kind::Unowned, vec![self.pop_kind(Kind::Type)?])),
            b'u' => mk_type(Node::with(
                Kind::Unmanaged,
                vec![self.pop_kind(Kind::Type)?],
            )),
            b'w' => mk_type(Node::with(Kind::Weak, vec![self.pop_kind(Kind::Type)?])),
            b'b' => mk_type(Node::with(
                Kind::SILBoxType,
                vec![self.pop_kind(Kind::Type)?],
            )),
            b'D' => mk_type(Node::with(
                Kind::DynamicSelf,
                vec![self.pop_kind(Kind::Type)?],
            )),
            b'M' => {
                let repr = self.metatype_representation()?;
                let ty = self.pop_kind(Kind::Type)?;
                mk_type(Node::with(Kind::Metatype, vec![repr, ty]))
            }
            b'm' => {
                let repr = self.metatype_representation()?;
                let ty = self.pop_kind(Kind::Type)?;
                mk_type(Node::with(Kind::ExistentialMetatype, vec![repr, ty]))
            }
            b'P' => {
                let reqs = self.constrained_existential_requirements()?;
                let base = self.pop_kind(Kind::Type)?;
                mk_type(Node::with(Kind::ConstrainedExistential, vec![base, reqs]))
            }
            b'p' => mk_type(Node::with(
                Kind::ExistentialMetatype,
                vec![self.pop_kind(Kind::Type)?],
            )),
            b'c' => {
                let superclass = self.pop_kind(Kind::Type)?;
                let protos = self.protocol_list()?;
                mk_type(Node::with(
                    Kind::ProtocolListWithClass,
                    vec![protos, superclass],
                ))
            }
            b'l' => {
                let protos = self.protocol_list()?;
                mk_type(Node::with(Kind::ProtocolListWithAnyObject, vec![protos]))
            }
            b'e' => mk_type(Node::new(Kind::ErrorType)),
            b'x' | b'X' => {
                let c = self.text[self.pos - 1];
                let mut sig_args = Vec::new();
                if c == b'X' {
                    let sig = self.pop_kind(Kind::DependentGenericSignature)?;
                    let args = self.pop_type_list()?;
                    sig_args.push(sig);
                    sig_args.push(args);
                }
                let fields = self.pop_type_list()?;
                let mut layout = Vec::new();
                for f in &fields.children {
                    let inner = f.first()?;
                    if inner.kind == Kind::InOut {
                        layout.push(Node::with(
                            Kind::SILBoxMutableField,
                            vec![mk_type(Rc::clone(inner.first()?))],
                        ));
                    } else {
                        layout.push(Node::with(Kind::SILBoxImmutableField, vec![Rc::clone(f)]));
                    }
                }
                let mut children = vec![Node::with(Kind::SILBoxLayout, layout)];
                children.extend(sig_args);
                mk_type(Node::with(Kind::SILBoxTypeWithLayout, children))
            }
            b'S' => match self.next() {
                b'q' => mk_type(Node::with(
                    Kind::SugaredOptional,
                    vec![self.pop_kind(Kind::Type)?],
                )),
                b'a' => mk_type(Node::with(
                    Kind::SugaredArray,
                    vec![self.pop_kind(Kind::Type)?],
                )),
                b'D' => {
                    let value = self.pop_kind(Kind::Type)?;
                    let key = self.pop_kind(Kind::Type)?;
                    mk_type(Node::with(Kind::SugaredDictionary, vec![key, value]))
                }
                b'p' => mk_type(Node::with(
                    Kind::SugaredParen,
                    vec![self.pop_kind(Kind::Type)?],
                )),
                _ => return None,
            },
            _ => return None,
        })
    }

    fn constrained_existential_requirements(&mut self) -> Option<NodeRef> {
        let mut reqs = Vec::new();
        loop {
            let first = self.pop_kind(Kind::FirstElementMarker).is_some();
            reqs.push(self.pop_if(Kind::is_requirement)?);
            if first {
                break;
            }
        }
        reqs.reverse();
        Some(Node::with(
            Kind::ConstrainedExistentialRequirementList,
            reqs,
        ))
    }

    fn type_annotation(&mut self) -> Option<NodeRef> {
        Some(match self.next() {
            b'a' => Node::new(Kind::AsyncAnnotation),
            b'A' => Node::new(Kind::IsolatedAnyFunctionType),
            b'b' => Node::new(Kind::ConcurrentFunctionType),
            b'c' => Node::with(
                Kind::GlobalActorFunctionType,
                vec![self.pop_type_and_get_child()?],
            ),
            b'C' => Node::new(Kind::NonIsolatedCallerFunctionType),
            b'i' => mk_type(Node::with(
                Kind::Isolated,
                vec![self.pop_type_and_get_child()?],
            )),
            b'j' => {
                let text = match self.next() {
                    b'f' => "_forward",
                    b'r' => "reverse",
                    b'd' => "",
                    b'l' => "_linear",
                    _ => return None,
                };
                Node::text(Kind::DifferentiableFunctionType, text)
            }
            b'k' => mk_type(Node::with(
                Kind::NoDerivative,
                vec![self.pop_type_and_get_child()?],
            )),
            b'K' => Node::with(
                Kind::TypedThrowsAnnotation,
                vec![self.pop_type_and_get_child()?],
            ),
            b't' => mk_type(Node::with(
                Kind::CompileTimeLiteral,
                vec![self.pop_type_and_get_child()?],
            )),
            b'T' => Node::new(Kind::SendingResultFunctionType),
            b'u' => mk_type(Node::with(
                Kind::Sending,
                vec![self.pop_type_and_get_child()?],
            )),
            _ => return None,
        })
    }

    // ---- generics ------------------------------------------------------

    fn generic_param_type(depth: u64, index: u64) -> NodeRef {
        Node::with_text(
            Kind::DependentGenericParamType,
            generic_parameter_name(depth, index),
            vec![
                Node::index(Kind::Index, depth),
                Node::index(Kind::Index, index),
            ],
        )
    }

    fn generic_param_index(&mut self) -> Option<NodeRef> {
        if self.next_if(b'd') {
            let depth = self.index()?.checked_add(1)?;
            let index = self.index()?;
            return Some(Self::generic_param_type(depth, index));
        }
        if self.next_if(b'z') {
            return Some(Self::generic_param_type(0, 0));
        }
        if self.next_if(b's') {
            return Some(Node::new(Kind::ConstrainedExistentialSelf));
        }
        let index = self.index()?.checked_add(1)?;
        Some(Self::generic_param_type(0, index))
    }

    fn pop_assoc_type_name(&mut self) -> Option<NodeRef> {
        let proto = self.pop_kind(Kind::Type);
        if let Some(p) = &proto {
            if !is_protocol_type(p) {
                return None;
            }
        }
        let id = self.pop_kind(Kind::Identifier)?;
        let mut children = vec![id];
        children.extend(proto);
        Some(Node::with(Kind::DependentAssociatedTypeRef, children))
    }

    fn pop_assoc_type_path(&mut self) -> Option<NodeRef> {
        let mut path = Vec::new();
        loop {
            let first = self.pop_kind(Kind::FirstElementMarker).is_some();
            path.push(self.pop_assoc_type_name()?);
            if first {
                break;
            }
        }
        path.reverse();
        Some(Node::with(Kind::AssocTypePath, path))
    }

    fn associated_type_simple(&mut self, base: Option<NodeRef>) -> Option<NodeRef> {
        let name = self.pop_assoc_type_name()?;
        let base_ty = match base {
            Some(b) => mk_type(b),
            None => self.pop_kind(Kind::Type)?,
        };
        Some(mk_type(Node::with(
            Kind::DependentMemberType,
            vec![base_ty, name],
        )))
    }

    fn associated_type_compound(&mut self, base: Option<NodeRef>) -> Option<NodeRef> {
        let mut names = Vec::new();
        loop {
            let first = self.pop_kind(Kind::FirstElementMarker).is_some();
            names.push(self.pop_assoc_type_name()?);
            if first {
                break;
            }
        }
        let mut base_ty = match base {
            Some(b) => mk_type(b),
            None => self.pop_kind(Kind::Type)?,
        };
        while let Some(name) = names.pop() {
            base_ty = mk_type(Node::with(Kind::DependentMemberType, vec![base_ty, name]));
        }
        Some(base_ty)
    }

    fn archetype(&mut self) -> Option<NodeRef> {
        Some(match self.next() {
            b'a' => {
                let ident = self.pop_kind(Kind::Identifier)?;
                let arche = self.pop_type_and_get_child()?;
                let ty = mk_type(Node::with(Kind::AssociatedTypeRef, vec![arche, ident]));
                self.add_sub(&ty);
                ty
            }
            b'O' => {
                let ctx = self.pop_context()?;
                Node::with(Kind::OpaqueReturnTypeOf, vec![ctx])
            }
            b'o' => {
                let index = self.index()?;
                let lists = self.bound_generics()?;
                let name = self.pop()?;
                let mut children = vec![name, Node::index(Kind::Index, index)];
                let mut all = Vec::new();
                for l in lists.iter().rev() {
                    all.push(Rc::clone(l));
                }
                children.push(Node::with(Kind::TypeList, all));
                let ty = mk_type(Node::with(Kind::OpaqueType, children));
                self.add_sub(&ty);
                ty
            }
            b'r' => mk_type(Node::new(Kind::OpaqueReturnType)),
            b'R' => {
                let ordinal = self.index()?;
                mk_type(Node::with(
                    Kind::OpaqueReturnType,
                    vec![Node::index(Kind::OpaqueReturnTypeIndex, ordinal)],
                ))
            }
            b'x' => {
                let t = self.associated_type_simple(None)?;
                self.add_sub(&t);
                t
            }
            b'X' => {
                let t = self.associated_type_compound(None)?;
                self.add_sub(&t);
                t
            }
            b'y' => {
                let gp = self.generic_param_index()?;
                let t = self.associated_type_simple(Some(gp))?;
                self.add_sub(&t);
                t
            }
            b'Y' => {
                let gp = self.generic_param_index()?;
                let t = self.associated_type_compound(Some(gp))?;
                self.add_sub(&t);
                t
            }
            b'z' => {
                let t = self.associated_type_simple(Some(Self::generic_param_type(0, 0)))?;
                self.add_sub(&t);
                t
            }
            b'Z' => {
                let t = self.associated_type_compound(Some(Self::generic_param_type(0, 0)))?;
                self.add_sub(&t);
                t
            }
            b'p' => {
                let count = self.pop_type_and_get_child()?;
                let pattern = self.pop_type_and_get_child()?;
                mk_type(Node::with(Kind::PackExpansion, vec![pattern, count]))
            }
            b'e' => {
                let pack = self.pop_type_and_get_child()?;
                let level = self.index()?;
                mk_type(Node::with(
                    Kind::PackElement,
                    vec![pack, Node::index(Kind::PackElementLevel, level)],
                ))
            }
            b'P' => {
                let list = self.pop_type_list()?;
                mk_type(Node::with(Kind::Pack, list.children.clone()))
            }
            _ => return None,
        })
    }

    fn generic_signature(&mut self, has_param_counts: bool) -> Option<NodeRef> {
        let mut counts = Vec::new();
        if has_param_counts {
            while !self.next_if(b'l') {
                let count = if self.next_if(b'z') {
                    0
                } else {
                    self.index()?.checked_add(1)?
                };
                counts.push(Node::index(Kind::DependentGenericParamCount, count));
                if self.at_end() {
                    return None;
                }
            }
        } else {
            counts.push(Node::index(Kind::DependentGenericParamCount, 1));
        }
        let mut reqs = Vec::new();
        while let Some(r) = self.pop_if(Kind::is_requirement) {
            reqs.push(r);
        }
        reqs.reverse();
        counts.extend(reqs);
        Some(Node::with(Kind::DependentGenericSignature, counts))
    }

    fn generic_type(&mut self) -> Option<NodeRef> {
        let sig = self.pop_kind(Kind::DependentGenericSignature)?;
        let ty = self.pop_kind(Kind::Type)?;
        Some(mk_type(Node::with(
            Kind::DependentGenericType,
            vec![sig, ty],
        )))
    }

    fn generic_requirement(&mut self) -> Option<NodeRef> {
        #[derive(PartialEq)]
        enum Constraint {
            Generic,
            Assoc,
            CompoundAssoc,
            Substitution,
        }
        #[derive(PartialEq)]
        enum Ty {
            Protocol,
            BaseClass,
            SameType,
            SameShape,
            Layout,
            PackMarker,
            Inverse,
        }
        use Constraint::*;
        let (ck, tk) = match self.next() {
            b'v' => (Generic, Ty::PackMarker),
            b'c' => (Assoc, Ty::BaseClass),
            b'C' => (CompoundAssoc, Ty::BaseClass),
            b'b' => (Generic, Ty::BaseClass),
            b'B' => (Substitution, Ty::BaseClass),
            b't' => (Assoc, Ty::SameType),
            b'T' => (CompoundAssoc, Ty::SameType),
            b's' => (Generic, Ty::SameType),
            b'S' => (Substitution, Ty::SameType),
            b'h' => (Generic, Ty::SameShape),
            b'm' => (Assoc, Ty::Layout),
            b'M' => (CompoundAssoc, Ty::Layout),
            b'l' => (Generic, Ty::Layout),
            b'L' => (Substitution, Ty::Layout),
            b'p' => (Assoc, Ty::Protocol),
            b'P' => (CompoundAssoc, Ty::Protocol),
            b'Q' => (Substitution, Ty::Protocol),
            b'i' => (Generic, Ty::Inverse),
            b'I' => (Substitution, Ty::Inverse),
            b'j' => (Assoc, Ty::Inverse),
            b'J' => (CompoundAssoc, Ty::Inverse),
            _ => {
                self.push_back();
                (Generic, Ty::Protocol)
            }
        };
        let inverse_kind = if tk == Ty::Inverse {
            Some(self.index()?)
        } else {
            None
        };
        let constr_ty = match ck {
            Generic => mk_type(self.generic_param_index()?),
            Assoc => {
                let gp = self.generic_param_index()?;
                let t = self.associated_type_simple(Some(gp))?;
                self.add_sub(&t);
                t
            }
            CompoundAssoc => {
                let gp = self.generic_param_index()?;
                let t = self.associated_type_compound(Some(gp))?;
                self.add_sub(&t);
                t
            }
            Substitution => self.pop_kind(Kind::Type)?,
        };
        Some(match tk {
            Ty::PackMarker => Node::with(Kind::DependentGenericParamPackMarker, vec![constr_ty]),
            Ty::Inverse => Node::with(
                Kind::DependentGenericInverseConformanceRequirement,
                vec![constr_ty, Node::index(Kind::Index, inverse_kind?)],
            ),
            Ty::Protocol => Node::with(
                Kind::DependentGenericConformanceRequirement,
                vec![constr_ty, self.pop_protocol()?],
            ),
            Ty::BaseClass => Node::with(
                Kind::DependentGenericConformanceRequirement,
                vec![constr_ty, self.pop_kind(Kind::Type)?],
            ),
            Ty::SameType => Node::with(
                Kind::DependentGenericSameTypeRequirement,
                vec![constr_ty, self.pop_kind(Kind::Type)?],
            ),
            Ty::SameShape => Node::with(
                Kind::DependentGenericSameShapeRequirement,
                vec![constr_ty, self.pop_kind(Kind::Type)?],
            ),
            Ty::Layout => {
                let c = self.next();
                let mut children = vec![
                    constr_ty,
                    Node::text(Kind::Identifier, (c as char).to_string()),
                ];
                match c {
                    b'U' | b'R' | b'N' | b'C' | b'D' | b'T' | b'B' => {}
                    b'E' | b'M' => {
                        let size = self.index()?;
                        let align = self.index()?;
                        children.push(Node::index(Kind::Number, size));
                        children.push(Node::index(Kind::Number, align));
                    }
                    b'e' | b'm' | b'S' => {
                        let size = self.index()?;
                        children.push(Node::index(Kind::Number, size));
                    }
                    _ => return None,
                }
                Node::with(Kind::DependentGenericLayoutRequirement, children)
            }
        })
    }

    // ---- global markers --------------------------------------------------

    fn metatype(&mut self) -> Option<NodeRef> {
        let c = self.next();
        let kind = match c {
            b'a' => Kind::TypeMetadataAccessFunction,
            b'A' => {
                return Some(Node::with(
                    Kind::ReflectionMetadataAssocTypeDescriptor,
                    vec![self.pop_protocol_conformance()?],
                ));
            }
            b'b' => Kind::CanonicalSpecializedGenericTypeMetadataAccessFunction,
            b'B' => Kind::ReflectionMetadataBuiltinDescriptor,
            b'C' => {
                let ty = self.pop_kind(Kind::Type)?;
                let inner = Rc::clone(ty.first()?);
                if !inner.kind.is_any_generic() {
                    return None;
                }
                return Some(Node::with(
                    Kind::ReflectionMetadataSuperclassDescriptor,
                    vec![inner],
                ));
            }
            b'D' => Kind::TypeMetadataDemanglingCache,
            b'F' => Kind::ReflectionMetadataFieldDescriptor,
            b'c' => {
                return Some(Node::with(
                    Kind::ProtocolConformanceDescriptor,
                    vec![self.pop_protocol_conformance()?],
                ));
            }
            b'f' => Kind::FullTypeMetadata,
            b'g' => {
                return Some(Node::with(
                    Kind::OpaqueTypeDescriptorAccessor,
                    vec![self.pop()?],
                ));
            }
            b'h' => {
                return Some(Node::with(
                    Kind::OpaqueTypeDescriptorAccessorImpl,
                    vec![self.pop()?],
                ));
            }
            b'i' => Kind::TypeMetadataInstantiationFunction,
            b'I' => Kind::TypeMetadataInstantiationCache,
            b'j' => {
                return Some(Node::with(
                    Kind::OpaqueTypeDescriptorAccessorKey,
                    vec![self.pop()?],
                ));
            }
            b'J' => {
                return Some(Node::with(
                    Kind::NoncanonicalSpecializedGenericTypeMetadataCache,
                    vec![self.pop()?],
                ));
            }
            b'k' => {
                return Some(Node::with(
                    Kind::OpaqueTypeDescriptorAccessorVar,
                    vec![self.pop()?],
                ));
            }
            b'K' => {
                return Some(Node::with(
                    Kind::MetadataInstantiationCache,
                    vec![self.pop()?],
                ));
            }
            b'l' => Kind::TypeMetadataSingletonInitializationCache,
            b'L' => Kind::TypeMetadataLazyCache,
            b'm' => Kind::Metaclass,
            b'M' => Kind::CanonicalSpecializedGenericMetaclass,
            b'n' => Kind::NominalTypeDescriptor,
            b'N' => Kind::NoncanonicalSpecializedGenericTypeMetadata,
            b'o' => Kind::ClassMetadataBaseOffset,
            b'p' => {
                return Some(Node::with(
                    Kind::ProtocolDescriptor,
                    vec![self.pop_protocol()?],
                ));
            }
            b'P' => Kind::GenericTypeMetadataPattern,
            b'Q' => return Some(Node::with(Kind::OpaqueTypeDescriptor, vec![self.pop()?])),
            b'r' => Kind::TypeMetadataCompletionFunction,
            b's' => Kind::ObjCResilientClassStub,
            b'S' => {
                return Some(Node::with(
                    Kind::ProtocolSelfConformanceDescriptor,
                    vec![self.pop_protocol()?],
                ));
            }
            b't' => Kind::FullObjCResilientClassStub,
            b'u' => Kind::MethodLookupFunction,
            b'U' => Kind::ObjCMetadataUpdateFunction,
            b'V' => {
                return Some(Node::with(
                    Kind::PropertyDescriptor,
                    vec![self.pop_if(Kind::is_entity)?],
                ));
            }
            b'X' => return self.private_context_descriptor(),
            b'z' => Kind::CanonicalPrespecializedGenericTypeCachingOnceToken,
            _ => return None,
        };
        Some(Node::with(kind, vec![self.pop_kind(Kind::Type)?]))
    }

    fn private_context_descriptor(&mut self) -> Option<NodeRef> {
        Some(match self.next() {
            b'E' => Node::with(Kind::ExtensionDescriptor, vec![self.pop_context()?]),
            b'M' => Node::with(Kind::ModuleDescriptor, vec![self.pop_module()?]),
            b'Y' => {
                let discriminator = self.pop()?;
                let ctx = self.pop_context()?;
                Node::with(
                    Kind::AnonymousDescriptor,
                    vec![Node::with(Kind::AnonymousContext, vec![discriminator, ctx])],
                )
            }
            b'X' => Node::with(Kind::AnonymousDescriptor, vec![self.pop_context()?]),
            _ => return None,
        })
    }

    fn pop_protocol_conformance(&mut self) -> Option<NodeRef> {
        let gen_sig = self.pop_kind(Kind::DependentGenericSignature);
        let module = self.pop_module()?;
        let proto = self.pop_protocol()?;
        let mut ty = self.pop_kind(Kind::Type);
        let mut ident = None;
        if ty.is_none() {
            // Property behavior conformance: the type is an identifier.
            ident = self.pop_kind(Kind::Identifier);
            ty = self.pop_kind(Kind::Type);
        }
        let ty = match (ty, gen_sig.as_ref()) {
            (Some(t), Some(sig)) => mk_type(Node::with(
                Kind::DependentGenericType,
                vec![Rc::clone(sig), t],
            )),
            (Some(t), None) => t,
            (None, _) => return None,
        };
        let mut children = vec![ty, proto, module];
        children.extend(ident);
        Some(Node::with(Kind::ProtocolConformance, children))
    }

    fn witness(&mut self) -> Option<NodeRef> {
        let c = self.next();
        Some(match c {
            b'C' => Node::with(Kind::EnumCase, vec![self.pop_if(Kind::is_entity)?]),
            b'V' => Node::with(Kind::ValueWitnessTable, vec![self.pop_kind(Kind::Type)?]),
            b'v' => {
                let directness = match self.next() {
                    b'd' => "direct",
                    b'i' => "indirect",
                    _ => return None,
                };
                let entity = self.pop_if(Kind::is_entity)?;
                Node::with(
                    Kind::FieldOffset,
                    vec![Node::text(Kind::Directness, directness), entity],
                )
            }
            b'S' => Node::with(
                Kind::ProtocolSelfConformanceWitnessTable,
                vec![self.pop_protocol()?],
            ),
            b'P' => Node::with(
                Kind::ProtocolWitnessTable,
                vec![self.pop_protocol_conformance()?],
            ),
            b'p' => Node::with(
                Kind::ProtocolWitnessTablePattern,
                vec![self.pop_protocol_conformance()?],
            ),
            b'G' => Node::with(
                Kind::GenericProtocolWitnessTable,
                vec![self.pop_protocol_conformance()?],
            ),
            b'I' => Node::with(
                Kind::GenericProtocolWitnessTableInstantiationFunction,
                vec![self.pop_protocol_conformance()?],
            ),
            b'r' => Node::with(
                Kind::ResilientProtocolWitnessTable,
                vec![self.pop_protocol_conformance()?],
            ),
            b'l' | b'L' => {
                let conf = self.pop_protocol_conformance()?;
                let ty = self.pop_kind(Kind::Type)?;
                let kind = if c == b'l' {
                    Kind::LazyProtocolWitnessTableAccessor
                } else {
                    Kind::LazyProtocolWitnessTableCacheVariable
                };
                Node::with(kind, vec![ty, conf])
            }
            b'a' => Node::with(
                Kind::ProtocolWitnessTableAccessor,
                vec![self.pop_protocol_conformance()?],
            ),
            b't' => {
                let name = self.pop_if(Kind::is_decl_name)?;
                let conf = self.pop_protocol_conformance()?;
                Node::with(Kind::AssociatedTypeMetadataAccessor, vec![conf, name])
            }
            b'T' => {
                let proto_ty = self.pop_kind(Kind::Type)?;
                let path = self.pop_assoc_type_path()?;
                let conf = self.pop_protocol_conformance()?;
                Node::with(
                    Kind::AssociatedTypeWitnessTableAccessor,
                    vec![conf, path, proto_ty],
                )
            }
            b'b' => {
                let proto_ty = self.pop_kind(Kind::Type)?;
                let conf = self.pop_protocol_conformance()?;
                Node::with(Kind::BaseWitnessTableAccessor, vec![conf, proto_ty])
            }
            b'O' => {
                let kind = match self.next() {
                    b'y' => Kind::OutlinedCopy,
                    b'e' => Kind::OutlinedConsume,
                    b'r' => Kind::OutlinedRetain,
                    b's' => Kind::OutlinedRelease,
                    b'b' => Kind::OutlinedInitializeWithTake,
                    b'c' => Kind::OutlinedInitializeWithCopy,
                    b'd' => Kind::OutlinedAssignWithTake,
                    b'f' => Kind::OutlinedAssignWithCopy,
                    b'h' => Kind::OutlinedDestroy,
                    b'B' => Kind::OutlinedInitializeWithTakeNoValueWitness,
                    b'C' => Kind::OutlinedInitializeWithCopyNoValueWitness,
                    b'D' => Kind::OutlinedAssignWithTakeNoValueWitness,
                    b'F' => Kind::OutlinedAssignWithCopyNoValueWitness,
                    b'H' => Kind::OutlinedDestroyNoValueWitness,
                    b'i' => Kind::OutlinedEnumTagStore,
                    b'j' => Kind::OutlinedEnumProjectDataForLoad,
                    b'g' => Kind::OutlinedEnumGetTag,
                    _ => return None,
                };
                let case_index = if matches!(
                    kind,
                    Kind::OutlinedEnumTagStore | Kind::OutlinedEnumProjectDataForLoad
                ) {
                    Some(self.index()?)
                } else {
                    None
                };
                let sig = self.pop_kind(Kind::DependentGenericSignature);
                let ty = self.pop_kind(Kind::Type)?;
                let mut children = vec![ty];
                children.extend(sig);
                children.extend(case_index.map(|i| Node::index(Kind::Index, i)));
                Node::with(kind, children)
            }
            b'Z' | b'z' => {
                let mut vars = Vec::new();
                while self.pop_kind(Kind::FirstElementMarker).is_some() {
                    vars.push(self.pop_if(Kind::is_decl_name)?);
                }
                vars.reverse();
                let ctx = self.pop_context()?;
                let kind = if c == b'Z' {
                    Kind::GlobalVariableOnceFunction
                } else {
                    Kind::GlobalVariableOnceToken
                };
                Node::with(
                    kind,
                    vec![ctx, Node::with(Kind::GlobalVariableOnceDeclList, vars)],
                )
            }
            _ => return None,
        })
    }

    fn value_witness(&mut self) -> Option<NodeRef> {
        let code = [self.next(), self.next()];
        let name = match &code {
            b"al" => "allocateBuffer",
            b"ca" => "assignWithCopy",
            b"ta" => "assignWithTake",
            b"de" => "deallocateBuffer",
            b"xx" => "destroy",
            b"XX" => "destroyBuffer",
            b"Xx" => "destroyArray",
            b"CP" => "initializeBufferWithCopyOfBuffer",
            b"Cp" => "initializeBufferWithCopy",
            b"cp" => "initializeWithCopy",
            b"TK" => "initializeBufferWithTakeOfBuffer",
            b"Tk" => "initializeBufferWithTake",
            b"tk" => "initializeWithTake",
            b"pr" => "projectBuffer",
            b"xs" => "storeExtraInhabitant",
            b"xg" => "getExtraInhabitantIndex",
            b"Cc" => "initializeArrayWithCopy",
            b"Tt" => "initializeArrayWithTakeFrontToBack",
            b"tT" => "initializeArrayWithTakeBackToFront",
            b"ug" => "getEnumTag",
            b"up" => "destructiveProjectEnumData",
            b"ui" => "destructiveInjectEnumTag",
            b"et" => "getEnumTagSinglePayload",
            b"st" => "storeEnumTagSinglePayload",
            _ => return None,
        };
        let ty = self.pop_kind(Kind::Type)?;
        Some(Node::with_text(Kind::ValueWitness, name, vec![ty]))
    }

    fn spec_attributes(&mut self, kind: Kind) -> Option<NodeRef> {
        let serialized = self.next_if(b'q');
        let async_removed = self.next_if(b'a');
        let pass = self.next();
        if !pass.is_ascii_digit() {
            return None;
        }
        let mut children = Vec::new();
        if serialized {
            children.push(Node::new(Kind::IsSerialized));
        }
        if async_removed {
            children.push(Node::new(Kind::AsyncRemoved));
        }
        children.push(Node::index(
            Kind::SpecializationPassID,
            (pass - b'0') as u64,
        ));
        Some(Node::with(kind, children))
    }

    fn dropped_arguments(&mut self) -> Vec<NodeRef> {
        let mut out = Vec::new();
        while self.next_if(b't') {
            let n = self.natural().map_or(0, |n| n + 1);
            out.push(Node::index(Kind::DroppedArgument, n));
        }
        out
    }

    fn generic_specialization(&mut self, kind: Kind, dropped: Vec<NodeRef>) -> Option<NodeRef> {
        let spec = self.spec_attributes(kind)?;
        let list = self.pop_type_list()?;
        let mut children = spec.children.clone();
        for ty in &list.children {
            children.push(Node::with(
                Kind::GenericSpecializationParam,
                vec![Rc::clone(ty)],
            ));
        }
        children.extend(dropped);
        Some(Node::with(kind, children))
    }

    fn function_specialization(&mut self) -> Option<NodeRef> {
        let spec = self.spec_attributes(Kind::FunctionSignatureSpecialization)?;
        let mut children = spec.children.clone();
        let mut idx = 0u64;
        while !self.next_if(b'_') {
            if self.at_end() {
                return None;
            }
            children.push(self.func_spec_param(Kind::FunctionSignatureSpecializationParam, idx)?);
            idx += 1;
        }
        if !self.next_if(b'n') {
            children.push(self.func_spec_param(Kind::FunctionSignatureSpecializationReturn, 0)?);
        }
        // The constant / closure payloads are popped in reverse order.
        for i in (0..children.len()).rev() {
            let param = Rc::clone(&children[i]);
            if param.kind != Kind::FunctionSignatureSpecializationParam
                && param.kind != Kind::FunctionSignatureSpecializationReturn
            {
                continue;
            }
            if param.len() < 1 {
                continue;
            }
            let pk = param.first()?.get_index();
            match pk {
                CP_FUNCTION | CP_GLOBAL | CP_STRING | CP_KEYPATH | CLOSURE_PROP => {
                    let mut extra = Vec::new();
                    while let Some(ty) = self.pop_kind(Kind::Type) {
                        if pk != CLOSURE_PROP && pk != CP_KEYPATH {
                            return None;
                        }
                        extra.push(ty);
                    }
                    let name = self.pop_kind(Kind::Identifier)?;
                    let mut text = name.get_text().to_string();
                    if pk == CP_STRING {
                        if let Some(t) = text.strip_prefix('_') {
                            text = t.to_string();
                        }
                    }
                    extra.push(Node::text(
                        Kind::FunctionSignatureSpecializationParamPayload,
                        text,
                    ));
                    extra.reverse();
                    let mut nc = param.children.clone();
                    nc.extend(extra);
                    children[i] = Node::with_index(param.kind, param.get_index(), nc);
                }
                _ => {}
            }
        }
        Some(Node::with(Kind::FunctionSignatureSpecialization, children))
    }

    fn func_spec_param(&mut self, kind: Kind, idx: u64) -> Option<NodeRef> {
        let mut children = Vec::new();
        let pk = |v: u64| Node::index(Kind::FunctionSignatureSpecializationParamKind, v);
        match self.next() {
            b'n' => {}
            b'c' => children.push(pk(CLOSURE_PROP)),
            b'p' => match self.next() {
                b'f' => children.push(pk(CP_FUNCTION)),
                b'g' => children.push(pk(CP_GLOBAL)),
                b'i' | b'd' => {
                    let c = self.text[self.pos - 1];
                    children.push(pk(if c == b'i' { CP_INTEGER } else { CP_FLOAT }));
                    let start = self.pos;
                    while self.peek().is_ascii_digit() {
                        self.next();
                    }
                    if start == self.pos {
                        return None;
                    }
                    let digits = std::str::from_utf8(&self.text[start..self.pos])
                        .ok()?
                        .to_string();
                    children.push(Node::text(
                        Kind::FunctionSignatureSpecializationParamPayload,
                        digits,
                    ));
                }
                b's' => {
                    let enc = match self.next() {
                        b'b' => "u8",
                        b'w' => "u16",
                        b'c' => "objc",
                        _ => return None,
                    };
                    children.push(pk(CP_STRING));
                    children.push(Node::text(
                        Kind::FunctionSignatureSpecializationParamPayload,
                        enc,
                    ));
                }
                b'k' => children.push(pk(CP_KEYPATH)),
                _ => return None,
            },
            b'e' => {
                let mut v = EXISTENTIAL_TO_GENERIC;
                if self.next_if(b'D') {
                    v |= DEAD;
                }
                if self.next_if(b'G') {
                    v |= OWNED_TO_GUARANTEED;
                }
                if self.next_if(b'O') {
                    v |= GUARANTEED_TO_OWNED;
                }
                if self.next_if(b'X') {
                    v |= SROA;
                }
                children.push(pk(v));
            }
            b'd' => {
                let mut v = DEAD;
                if self.next_if(b'G') {
                    v |= OWNED_TO_GUARANTEED;
                }
                if self.next_if(b'O') {
                    v |= GUARANTEED_TO_OWNED;
                }
                if self.next_if(b'X') {
                    v |= SROA;
                }
                children.push(pk(v));
            }
            b'g' => {
                let mut v = OWNED_TO_GUARANTEED;
                if self.next_if(b'X') {
                    v |= SROA;
                }
                children.push(pk(v));
            }
            b'o' => {
                let mut v = GUARANTEED_TO_OWNED;
                if self.next_if(b'X') {
                    v |= SROA;
                }
                children.push(pk(v));
            }
            b'x' => children.push(pk(SROA)),
            b'i' => children.push(pk(BOX_TO_VALUE)),
            b's' => children.push(pk(BOX_TO_STACK)),
            b'r' => children.push(pk(INOUT_TO_OUT)),
            _ => return None,
        }
        Some(Node::with_index(kind, idx, children))
    }

    fn thunk_or_specialization(&mut self) -> Option<NodeRef> {
        let c = self.next();
        Some(match c {
            b'c' => Node::with(Kind::CurryThunk, vec![self.pop_if(Kind::is_entity)?]),
            b'j' => Node::with(Kind::DispatchThunk, vec![self.pop_if(Kind::is_entity)?]),
            b'q' => Node::with(Kind::MethodDescriptor, vec![self.pop_if(Kind::is_entity)?]),
            b'o' => Node::new(Kind::ObjCAttribute),
            b'O' => Node::new(Kind::NonObjCAttribute),
            b'D' => Node::new(Kind::DynamicAttribute),
            b'd' => Node::new(Kind::DirectMethodReferenceAttribute),
            b'E' => Node::new(Kind::DistributedThunk),
            b'F' => Node::new(Kind::DistributedAccessor),
            b'a' => Node::new(Kind::PartialApplyObjCForwarder),
            b'A' => Node::new(Kind::PartialApplyForwarder),
            b'm' => Node::new(Kind::MergedFunction),
            b'X' => Node::new(Kind::DynamicallyReplaceableFunctionVar),
            b'x' => Node::new(Kind::DynamicallyReplaceableFunctionKey),
            b'I' => Node::new(Kind::DynamicallyReplaceableFunctionImpl),
            b'u' => Node::new(Kind::AsyncFunctionPointer),
            b'Y' => Node::with(
                Kind::AsyncSuspendResumePartialFunction,
                vec![self.index_node()?],
            ),
            b'Q' => Node::with(
                Kind::AsyncAwaitResumePartialFunction,
                vec![self.index_node()?],
            ),
            b'V' => {
                let base = self.pop_if(Kind::is_entity)?;
                let derived = self.pop_if(Kind::is_entity)?;
                Node::with(Kind::VTableThunk, vec![base, derived])
            }
            b'W' => {
                let entity = self.pop_if(Kind::is_entity)?;
                let conf = self.pop_protocol_conformance()?;
                Node::with(Kind::ProtocolWitness, vec![conf, entity])
            }
            b'S' => Node::with(
                Kind::ProtocolSelfConformanceWitness,
                vec![self.pop_if(Kind::is_entity)?],
            ),
            b'R' | b'r' | b'y' => {
                let kind = match c {
                    b'y' => Kind::ReabstractionThunkHelperWithSelf,
                    b'R' => Kind::ReabstractionThunkHelper,
                    _ => Kind::ReabstractionThunk,
                };
                let mut children = Vec::new();
                children.extend(self.pop_kind(Kind::DependentGenericSignature));
                if c == b'y' {
                    children.push(self.pop_kind(Kind::Type)?);
                }
                let ty2 = self.pop_kind(Kind::Type)?;
                let ty1 = self.pop_kind(Kind::Type)?;
                children.push(ty1);
                children.push(ty2);
                Node::with(kind, children)
            }
            b'g' => self.generic_specialization(Kind::GenericSpecialization, Vec::new())?,
            b'G' => {
                self.generic_specialization(Kind::GenericSpecializationNotReAbstracted, Vec::new())?
            }
            b'B' => self.generic_specialization(
                Kind::GenericSpecializationInResilienceDomain,
                Vec::new(),
            )?,
            b't' => {
                self.push_back();
                let dropped = self.dropped_arguments();
                let kind = match self.next() {
                    b'g' => Kind::GenericSpecialization,
                    b'G' => Kind::GenericSpecializationNotReAbstracted,
                    b'B' => Kind::GenericSpecializationInResilienceDomain,
                    _ => return None,
                };
                self.generic_specialization(kind, dropped)?
            }
            b's' => {
                self.generic_specialization(Kind::GenericSpecializationPrespecialized, Vec::new())?
            }
            b'i' => self.generic_specialization(Kind::InlinedGenericFunction, Vec::new())?,
            b'P' | b'p' => {
                let kind = if c == b'P' {
                    Kind::GenericPartialSpecializationNotReAbstracted
                } else {
                    Kind::GenericPartialSpecialization
                };
                let spec = self.spec_attributes(kind)?;
                let param = Node::with(
                    Kind::GenericSpecializationParam,
                    vec![self.pop_kind(Kind::Type)?],
                );
                spec.plus(param)
            }
            b'f' => self.function_specialization()?,
            b'K' | b'k' => {
                let kind = if c == b'K' {
                    Kind::KeyPathGetterThunkHelper
                } else {
                    Kind::KeyPathSetterThunkHelper
                };
                let serialized = self.next_if(b'q');
                let mut types = Vec::new();
                let mut node = self.pop()?;
                if node.kind != Kind::Type {
                    return None;
                }
                loop {
                    types.push(node);
                    match self.pop() {
                        Some(n) if n.kind == Kind::Type => node = n,
                        Some(n) => {
                            node = n;
                            break;
                        }
                        None => return None,
                    }
                }
                let mut children = if node.kind == Kind::DependentGenericSignature {
                    let decl = self.pop()?;
                    vec![decl, node]
                } else {
                    vec![node]
                };
                types.reverse();
                children.extend(types);
                if serialized {
                    children.push(Node::new(Kind::IsSerialized));
                }
                Node::with(kind, children)
            }
            b'l' => Node::with(
                Kind::AssociatedTypeDescriptor,
                vec![self.pop_assoc_type_name()?],
            ),
            b'L' => Node::with(
                Kind::ProtocolRequirementsBaseDescriptor,
                vec![self.pop_protocol()?],
            ),
            b'M' => Node::with(
                Kind::DefaultAssociatedTypeMetadataAccessor,
                vec![self.pop_assoc_type_name()?],
            ),
            b'n' | b'N' => {
                let req = self.pop_protocol()?;
                let path = self.pop_assoc_type_path()?;
                let conforming = self.pop_kind(Kind::Type)?;
                let kind = if c == b'n' {
                    Kind::AssociatedConformanceDescriptor
                } else {
                    Kind::DefaultAssociatedConformanceAccessor
                };
                Node::with(kind, vec![conforming, path, req])
            }
            b'b' => {
                let req = self.pop_protocol()?;
                let conforming = self.pop_kind(Kind::Type)?;
                Node::with(Kind::BaseConformanceDescriptor, vec![conforming, req])
            }
            b'H' | b'h' => {
                let kind = if c == b'H' {
                    Kind::KeyPathEqualsThunkHelper
                } else {
                    Kind::KeyPathHashThunkHelper
                };
                let serialized = self.next_if(b'q');
                let sig = self.pop_kind(Kind::DependentGenericSignature);
                let mut types = Vec::new();
                let mut node = self.pop()?;
                if node.kind != Kind::Type {
                    return None;
                }
                loop {
                    types.push(node);
                    match self.pop_kind(Kind::Type) {
                        Some(n) => node = n,
                        None => break,
                    }
                }
                types.reverse();
                let mut children = types;
                children.extend(sig);
                if serialized {
                    children.push(Node::new(Kind::IsSerialized));
                }
                Node::with(kind, children)
            }
            b'v' => {
                let idx = self.index()?;
                if self.next_if(b'r') {
                    Node::index(Kind::OutlinedReadOnlyObject, idx)
                } else {
                    Node::index(Kind::OutlinedVariable, idx)
                }
            }
            b'w' => match self.next() {
                b'b' => Node::new(Kind::BackDeploymentThunk),
                b'B' => Node::new(Kind::BackDeploymentFallback),
                b'S' => Node::new(Kind::HasSymbolQuery),
                b'c' => Node::new(Kind::CoroFunctionPointer),
                b'd' => Node::new(Kind::DefaultOverride),
                _ => return None,
            },
            _ => return None,
        })
    }

    fn conformance(&mut self) -> Option<NodeRef> {
        Some(match self.next() {
            b'P' => Node::with(
                Kind::ProtocolConformanceRefInTypeModule,
                vec![self.pop_protocol()?],
            ),
            b'p' => Node::with(
                Kind::ProtocolConformanceRefInProtocolModule,
                vec![self.pop_protocol()?],
            ),
            b'C' => {
                let list = self.pop_any_protocol_conformance_list()?;
                let conf_ref = self.pop_protocol_conformance_ref()?;
                let ty = self.pop_kind(Kind::Type)?;
                Node::with(Kind::ConcreteProtocolConformance, vec![ty, conf_ref, list])
            }
            b'D' => {
                let index = self.dependent_conformance_index()?;
                let proto = self.pop_protocol()?;
                let ty = self.pop_kind(Kind::Type)?;
                Node::with(
                    Kind::DependentProtocolConformanceRoot,
                    vec![ty, proto, index],
                )
            }
            b'I' => {
                let index = self.dependent_conformance_index()?;
                let proto = self.pop_protocol()?;
                let nested = self.pop_dependent_protocol_conformance()?;
                Node::with(
                    Kind::DependentProtocolConformanceInherited,
                    vec![nested, proto, index],
                )
            }
            b'A' => {
                let index = self.dependent_conformance_index()?;
                let assoc = self.pop_dependent_associated_conformance()?;
                let nested = self.pop_dependent_protocol_conformance()?;
                Node::with(
                    Kind::DependentProtocolConformanceAssociated,
                    vec![nested, assoc, index],
                )
            }
            b'O' => {
                let ty = self.pop_kind(Kind::Type)?;
                let nested = self.pop_dependent_protocol_conformance()?;
                Node::with(Kind::DependentProtocolConformanceOpaque, vec![nested, ty])
            }
            b'X' => Node::with(
                Kind::PackProtocolConformance,
                vec![self.pop_any_protocol_conformance_list()?],
            ),
            _ => return None,
        })
    }

    fn dependent_conformance_index(&mut self) -> Option<NodeRef> {
        let index = self.index()?;
        if index == 1 {
            return Some(Node::new(Kind::UnknownIndex));
        }
        Some(Node::index(Kind::Index, index.checked_sub(2)?))
    }

    fn pop_protocol_conformance_ref(&mut self) -> Option<NodeRef> {
        if let Some(n) = self.pop_kind(Kind::ProtocolConformanceRefInTypeModule) {
            return Some(n);
        }
        if let Some(n) = self.pop_kind(Kind::ProtocolConformanceRefInProtocolModule) {
            return Some(n);
        }
        let module = self.pop_module()?;
        let proto = self.pop_protocol()?;
        Some(Node::with(
            Kind::ProtocolConformanceRefInOtherModule,
            vec![proto, module],
        ))
    }

    fn pop_any_protocol_conformance(&mut self) -> Option<NodeRef> {
        self.pop_if(|k| {
            matches!(
                k,
                Kind::ConcreteProtocolConformance
                    | Kind::PackProtocolConformance
                    | Kind::DependentProtocolConformanceRoot
                    | Kind::DependentProtocolConformanceInherited
                    | Kind::DependentProtocolConformanceAssociated
                    | Kind::DependentProtocolConformanceOpaque
            )
        })
    }

    fn pop_dependent_protocol_conformance(&mut self) -> Option<NodeRef> {
        self.pop_if(|k| {
            matches!(
                k,
                Kind::DependentProtocolConformanceRoot
                    | Kind::DependentProtocolConformanceInherited
                    | Kind::DependentProtocolConformanceAssociated
                    | Kind::DependentProtocolConformanceOpaque
            )
        })
    }

    fn pop_dependent_associated_conformance(&mut self) -> Option<NodeRef> {
        let proto = self.pop_protocol()?;
        let ty = self.pop_kind(Kind::Type)?;
        Some(Node::with(
            Kind::DependentAssociatedConformance,
            vec![ty, proto],
        ))
    }

    fn pop_any_protocol_conformance_list(&mut self) -> Option<NodeRef> {
        let mut list = Vec::new();
        if self.pop_kind(Kind::EmptyList).is_none() {
            loop {
                let first = self.pop_kind(Kind::FirstElementMarker).is_some();
                list.push(self.pop_any_protocol_conformance()?);
                if first {
                    break;
                }
            }
            list.reverse();
        }
        Some(Node::with(Kind::AnyProtocolConformanceList, list))
    }

    fn retroactive_conformance(&mut self) -> Option<NodeRef> {
        let index = self.index_node()?;
        let conf = self.pop_any_protocol_conformance()?;
        Some(Node::with(Kind::RetroactiveConformance, vec![index, conf]))
    }

    fn impl_function_type(&mut self) -> Option<NodeRef> {
        let mut children = Vec::new();
        if self.next_if(b's') {
            let lists = self.bound_generics()?;
            let sig = self.pop_kind(Kind::DependentGenericSignature)?;
            if lists.len() != 1 {
                return None;
            }
            children.push(Node::with(
                Kind::ImplPatternSubstitutions,
                vec![sig, lists[0].clone()],
            ));
        }
        if self.next_if(b'I') {
            let lists = self.bound_generics()?;
            if lists.len() != 1 {
                return None;
            }
            children.push(Node::with(
                Kind::ImplInvocationSubstitutions,
                vec![lists[0].clone()],
            ));
        }
        let gen_sig = self.pop_kind(Kind::DependentGenericSignature);
        if gen_sig.is_some() {
            self.next_if(b'P');
        }
        if self.next_if(b'e') {
            children.push(Node::new(Kind::ImplEscaping));
        }
        if self.next_if(b'A') {
            children.push(Node::new(Kind::ImplErasedIsolation));
        }
        if self.next_if(b'N') {
            children.push(Node::text(
                Kind::ImplFunctionAttribute,
                "nonisolated(nonsending)",
            ));
        }
        let diff = match self.peek() {
            b'd' => Some(""),
            b'l' => Some("(_linear)"),
            b'f' => Some("(_forward)"),
            b'r' => Some("(reverse)"),
            _ => None,
        };
        if let Some(d) = diff {
            self.next();
            children.push(Node::text(Kind::ImplDifferentiabilityKind, d));
        }
        let callee = match self.next() {
            b'y' => "@callee_unowned",
            b'g' => "@callee_guaranteed",
            b'x' => "@callee_owned",
            b't' => "@convention(thin)",
            _ => return None,
        };
        children.push(Node::text(Kind::ImplConvention, callee));
        let mut clang = false;
        let fconv = match self.next() {
            b'B' => Some("block"),
            b'C' => Some("c"),
            b'z' => match self.next() {
                b'B' => {
                    clang = true;
                    Some("block")
                }
                b'C' => {
                    clang = true;
                    Some("c")
                }
                _ => {
                    self.push_back();
                    self.push_back();
                    None
                }
            },
            b'M' => Some("method"),
            b'J' => Some("objc_method"),
            b'K' => Some("closure"),
            b'W' => Some("witness_method"),
            _ => {
                self.push_back();
                None
            }
        };
        if let Some(f) = fconv {
            let mut fc = vec![Node::text(Kind::ImplFunctionConventionName, f)];
            if clang {
                fc.push(self.clang_type()?);
            }
            children.push(Node::with(Kind::ImplFunctionConvention, fc));
        }
        let coro = if self.next_if(b'A') {
            Some("yield_once")
        } else if self.next_if(b'I') {
            Some("yield_once_2")
        } else if self.next_if(b'G') {
            Some("yield_many")
        } else {
            None
        };
        if let Some(c) = coro {
            children.push(Node::text(Kind::ImplCoroutineKind, c));
        }
        if self.next_if(b'h') {
            children.push(Node::text(Kind::ImplFunctionAttribute, "@Sendable"));
        }
        if self.next_if(b'H') {
            children.push(Node::text(Kind::ImplFunctionAttribute, "@async"));
        }
        if self.next_if(b'T') {
            children.push(Node::new(Kind::ImplSendingResult));
        }
        children.extend(gen_sig);
        let mut conv_count = 0usize;
        while let Some(attr) = self.impl_param_convention() {
            let mut param = vec![Node::text(Kind::ImplConvention, attr)];
            if self.next_if(b'w') {
                param.push(Node::text(
                    Kind::ImplParameterResultDifferentiability,
                    "@noDerivative",
                ));
            }
            if self.next_if(b'T') {
                param.push(Node::text(Kind::ImplParameterSending, "sending"));
            }
            if self.next_if(b'I') {
                param.push(Node::text(Kind::ImplParameterIsolated, "isolated"));
            }
            if self.next_if(b'L') {
                param.push(Node::text(
                    Kind::ImplParameterImplicitLeading,
                    "@implicit_leading",
                ));
            }
            children.push(Node::with(Kind::ImplParameter, param));
            conv_count += 1;
        }
        while let Some(attr) = self.impl_result_convention() {
            let mut res = vec![Node::text(Kind::ImplConvention, attr)];
            if self.next_if(b'w') {
                res.push(Node::text(
                    Kind::ImplParameterResultDifferentiability,
                    "@noDerivative",
                ));
            }
            children.push(Node::with(Kind::ImplResult, res));
            conv_count += 1;
        }
        while self.next_if(b'Y') {
            let attr = self.impl_param_convention()?;
            children.push(Node::with(
                Kind::ImplYield,
                vec![Node::text(Kind::ImplConvention, attr)],
            ));
            conv_count += 1;
        }
        if self.next_if(b'z') {
            let attr = self.impl_result_convention()?;
            children.push(Node::with(
                Kind::ImplErrorResult,
                vec![Node::text(Kind::ImplConvention, attr)],
            ));
            conv_count += 1;
        }
        if !self.next_if(b'_') {
            return None;
        }
        let n = children.len();
        for i in 0..conv_count {
            let ty = self.pop_kind(Kind::Type)?;
            let slot = n - i - 1;
            children[slot] = children[slot].plus(ty);
        }
        Some(mk_type(Node::with(Kind::ImplFunctionType, children)))
    }

    fn impl_param_convention(&mut self) -> Option<&'static str> {
        let attr = match self.next() {
            b'i' => "@in",
            b'c' => "@in_constant",
            b'l' => "@inout",
            b'b' => "@inout_aliasable",
            b'n' => "@in_guaranteed",
            b'X' => "@in_cxx",
            b'x' => "@owned",
            b'g' => "@guaranteed",
            b'e' => "@deallocating",
            b'y' => "@unowned",
            b'v' => "@pack_owned",
            b'p' => "@pack_guaranteed",
            b'm' => "@pack_inout",
            0 => return None,
            _ => {
                self.push_back();
                return None;
            }
        };
        Some(attr)
    }

    fn impl_result_convention(&mut self) -> Option<&'static str> {
        let attr = match self.next() {
            b'r' => "@out",
            b'o' => "@owned",
            b'd' => "@unowned",
            b'u' => "@unowned_inner_pointer",
            b'a' => "@autoreleased",
            b'k' => "@pack_out",
            b'l' => "@guaranteed_address",
            b'g' => "@guaranteed",
            b'm' => "@inout",
            0 => return None,
            _ => {
                self.push_back();
                return None;
            }
        };
        Some(attr)
    }

    // ---- the operator loop ----------------------------------------------

    fn operator(&mut self) -> Option<NodeRef> {
        self.tick()?;
        let c = self.next();
        match c {
            b'A' => self.multi_substitutions(),
            b'B' => self.builtin_type(),
            b'C' => self.any_generic_type(Kind::Class),
            b'D' => Some(Node::with(
                Kind::TypeMangling,
                vec![self.pop_kind(Kind::Type)?],
            )),
            b'E' => self.extension_context(),
            b'F' => self.plain_function(Kind::Function),
            b'G' => self.bound_generic_type(),
            b'H' => self.conformance(),
            b'I' => self.impl_function_type(),
            b'K' => Some(Node::new(Kind::ThrowsAnnotation)),
            b'L' => self.local_identifier(),
            b'M' => self.metatype(),
            b'N' => Some(Node::with(
                Kind::TypeMetadata,
                vec![self.pop_kind(Kind::Type)?],
            )),
            b'O' => self.any_generic_type(Kind::Enum),
            b'P' => self.any_generic_type(Kind::Protocol),
            b'Q' => self.archetype(),
            b'R' => self.generic_requirement(),
            b'S' => self.standard_substitution(),
            b'T' => self.thunk_or_specialization(),
            b'V' => self.any_generic_type(Kind::Structure),
            b'W' => self.witness(),
            b'X' => self.special_type(),
            b'Y' => self.type_annotation(),
            b'Z' => Some(Node::with(
                Kind::Static,
                vec![self.pop_if(Kind::is_entity)?],
            )),
            b'a' => self.any_generic_type(Kind::TypeAlias),
            b'c' => self.pop_function_type(Kind::FunctionType, false),
            b'd' => Some(Node::new(Kind::VariadicMarker)),
            b'f' => self.function_entity(),
            b'g' => self.retroactive_conformance(),
            b'h' => Some(mk_type(Node::with(
                Kind::Shared,
                vec![self.pop_type_and_get_child()?],
            ))),
            b'i' => self.subscript(),
            b'l' => self.generic_signature(false),
            b'm' => Some(mk_type(Node::with(
                Kind::Metatype,
                vec![self.pop_kind(Kind::Type)?],
            ))),
            b'n' => Some(mk_type(Node::with(
                Kind::Owned,
                vec![self.pop_type_and_get_child()?],
            ))),
            b'o' => self.operator_identifier(),
            b'p' => Some(mk_type(self.protocol_list()?)),
            b'q' => Some(mk_type(self.generic_param_index()?)),
            b'r' => self.generic_signature(true),
            b's' => Some(Node::text(Kind::Module, "Swift")),
            b't' => self.tuple(),
            b'u' => self.generic_type(),
            b'v' => self.variable(),
            b'w' => self.value_witness(),
            b'x' => Some(mk_type(Self::generic_param_type(0, 0))),
            b'y' => Some(Node::new(Kind::EmptyList)),
            b'z' => Some(mk_type(Node::with(
                Kind::InOut,
                vec![self.pop_type_and_get_child()?],
            ))),
            b'_' => Some(Node::new(Kind::FirstElementMarker)),
            b'.' => {
                // IRGen's `.<n>` disambiguation suffix: the rest of the text,
                // kept verbatim.
                self.push_back();
                let rest = std::str::from_utf8(&self.text[self.pos..])
                    .ok()?
                    .to_string();
                self.pos = self.text.len();
                Some(Node::text(Kind::Suffix, rest))
            }
            b'$' => {
                let negative = self.next_if(b'n');
                let value = self.index()?;
                Some(mk_type(if negative {
                    Node::index(Kind::NegativeInteger, value)
                } else {
                    Node::index(Kind::Integer, value)
                }))
            }
            _ => {
                self.push_back();
                self.identifier()
            }
        }
    }

    /// Parse the whole body into the `Global` node.
    pub fn parse_global(mut self) -> Option<NodeRef> {
        while !self.at_end() {
            let n = self.operator()?;
            self.push(n);
        }
        if self.failed {
            return None;
        }
        // The function attributes (specializations, thunk markers) sit on
        // top of the stack and wrap the entity below them; a partial-apply
        // forwarder additionally owns everything popped after it.
        let suffix = self.pop_kind(Kind::Suffix);
        let mut groups: Vec<(Option<NodeRef>, Vec<NodeRef>)> = vec![(None, Vec::new())];
        while let Some(attr) = self.pop_if(Kind::is_function_attr) {
            if attr.kind == Kind::PartialApplyForwarder
                || attr.kind == Kind::PartialApplyObjCForwarder
            {
                groups.push((Some(attr), Vec::new()));
            } else {
                groups.last_mut()?.1.push(attr);
            }
        }
        let mut tail: Vec<NodeRef> = Vec::new();
        for n in self.stack.drain(..) {
            if n.kind == Kind::Type {
                tail.push(Rc::clone(n.first()?));
            } else {
                tail.push(n);
            }
        }
        tail.extend(suffix);
        let mut children = Vec::new();
        while let Some((pa, mut attrs)) = groups.pop() {
            attrs.extend(std::mem::take(&mut tail));
            match pa {
                Some(pa) => tail = vec![Node::with(pa.kind, attrs)],
                None => children = attrs,
            }
        }
        if children.is_empty() {
            return None;
        }
        Some(Node::with(Kind::Global, children))
    }
}

/// `KNOWN-TYPE-KIND` / `KNOWN-TYPE-KIND-2`: the stdlib types with a
/// one-letter spelling.
fn known_type(c: u8, second: bool) -> Option<(Kind, &'static str)> {
    use Kind::{Class, Enum, Protocol, Structure};
    Some(if second {
        match c {
            b'A' => (Protocol, "Actor"),
            b'C' => (Structure, "CheckedContinuation"),
            b'c' => (Structure, "UnsafeContinuation"),
            b'E' => (Structure, "CancellationError"),
            b'e' => (Structure, "UnownedSerialExecutor"),
            b'F' => (Protocol, "Executor"),
            b'f' => (Protocol, "SerialExecutor"),
            b'G' => (Structure, "TaskGroup"),
            b'g' => (Structure, "ThrowingTaskGroup"),
            b'h' => (Protocol, "TaskExecutor"),
            b'I' => (Protocol, "AsyncIteratorProtocol"),
            b'i' => (Protocol, "AsyncSequence"),
            b'J' => (Structure, "UnownedJob"),
            b'M' => (Class, "MainActor"),
            b'P' => (Structure, "TaskPriority"),
            b'S' => (Structure, "AsyncStream"),
            b's' => (Structure, "AsyncThrowingStream"),
            b'T' => (Structure, "Task"),
            b't' => (Structure, "UnsafeCurrentTask"),
            _ => return None,
        }
    } else {
        match c {
            b'A' => (Structure, "AutoreleasingUnsafeMutablePointer"),
            b'a' => (Structure, "Array"),
            b'B' => (Protocol, "BinaryFloatingPoint"),
            b'b' => (Structure, "Bool"),
            b'D' => (Structure, "Dictionary"),
            b'd' => (Structure, "Double"),
            b'E' => (Protocol, "Encodable"),
            b'e' => (Protocol, "Decodable"),
            b'F' => (Protocol, "FloatingPoint"),
            b'f' => (Structure, "Float"),
            b'G' => (Protocol, "RandomNumberGenerator"),
            b'H' => (Protocol, "Hashable"),
            b'h' => (Structure, "Set"),
            b'I' => (Structure, "DefaultIndices"),
            b'i' => (Structure, "Int"),
            b'J' => (Structure, "Character"),
            b'j' => (Protocol, "Numeric"),
            b'K' => (Protocol, "BidirectionalCollection"),
            b'k' => (Protocol, "RandomAccessCollection"),
            b'L' => (Protocol, "Comparable"),
            b'l' => (Protocol, "Collection"),
            b'M' => (Protocol, "MutableCollection"),
            b'm' => (Protocol, "RangeReplaceableCollection"),
            b'N' => (Structure, "ClosedRange"),
            b'n' => (Structure, "Range"),
            b'O' => (Structure, "ObjectIdentifier"),
            b'P' => (Structure, "UnsafePointer"),
            b'p' => (Structure, "UnsafeMutablePointer"),
            b'Q' => (Protocol, "Equatable"),
            b'q' => (Enum, "Optional"),
            b'R' => (Structure, "UnsafeBufferPointer"),
            b'r' => (Structure, "UnsafeMutableBufferPointer"),
            b'S' => (Structure, "String"),
            b's' => (Structure, "Substring"),
            b'T' => (Protocol, "Sequence"),
            b't' => (Protocol, "IteratorProtocol"),
            b'U' => (Protocol, "UnsignedInteger"),
            b'u' => (Structure, "UInt"),
            b'V' => (Structure, "UnsafeRawPointer"),
            b'v' => (Structure, "UnsafeMutableRawPointer"),
            b'W' => (Structure, "UnsafeRawBufferPointer"),
            b'w' => (Structure, "UnsafeMutableRawBufferPointer"),
            b'X' => (Protocol, "RangeExpression"),
            b'x' => (Protocol, "Strideable"),
            b'Y' => (Protocol, "RawRepresentable"),
            b'y' => (Protocol, "StringProtocol"),
            b'Z' => (Protocol, "SignedInteger"),
            b'z' => (Protocol, "BinaryInteger"),
            _ => return None,
        }
    })
}

/// `A`, `B`, …, `Z`, `AA`, … for depth 0; a depth suffix otherwise
/// (`A1` is depth 1, index 0).
pub(super) fn generic_parameter_name(depth: u64, mut index: u64) -> String {
    let mut name = String::new();
    loop {
        name.push((b'A' + (index % 26) as u8) as char);
        index /= 26;
        if index == 0 {
            break;
        }
    }
    if depth != 0 {
        name.push_str(&depth.to_string());
    }
    name
}

pub(super) const CP_FUNCTION: u64 = 0;
pub(super) const CP_GLOBAL: u64 = 1;
pub(super) const CP_INTEGER: u64 = 2;
pub(super) const CP_FLOAT: u64 = 3;
pub(super) const CP_STRING: u64 = 4;
pub(super) const CLOSURE_PROP: u64 = 5;
pub(super) const BOX_TO_VALUE: u64 = 6;
pub(super) const BOX_TO_STACK: u64 = 7;
pub(super) const INOUT_TO_OUT: u64 = 8;
pub(super) const CP_KEYPATH: u64 = 9;
pub(super) const DEAD: u64 = 1 << 6;
pub(super) const OWNED_TO_GUARANTEED: u64 = 1 << 7;
pub(super) const SROA: u64 = 1 << 8;
pub(super) const GUARANTEED_TO_OWNED: u64 = 1 << 9;
pub(super) const EXISTENTIAL_TO_GENERIC: u64 = 1 << 10;

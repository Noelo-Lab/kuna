//! Port of `org/jetbrains/java/decompiler/util/TextBuffer.java`.
//!
//! The text is kept as UTF-16 units, so every offset, length and line width is
//! the Java one. The tree of newline groups lives in a per-buffer arena (index 0
//! is the root). As in Java, appending a buffer shares its replacement records
//! with the copy, so a later shift of either buffer moves both (`Clone` shares
//! them the same way). The GC-driven `DebugTrace` leak checker is not ported,
//! and Java `assert`s (disabled at run time) are not either.

use std::cell::{Cell, RefCell};
use std::collections::{BTreeSet, VecDeque};
use std::rc::Rc;

use crate::annotation_exprent::AnnotationExprent;
use crate::bytecode_mapping_tracer::BytecodeMappingTracer;
use crate::class_text_token::ClassTextToken;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::field_descriptor::FieldDescriptor;
use crate::field_text_token::FieldTextToken;
use crate::generic_main::GenericMain;
use crate::generic_type::GenericType;
use crate::i_fernflower_logger::Severity;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::jthrow;
use crate::method_descriptor::MethodDescriptor;
use crate::method_text_token::MethodTextToken;
use crate::pair::Pair;
use crate::rt::jstring::from_utf16;
use crate::rt::{
    exception, jchar, objects_hash, BitSet, JHashMap, JHashSet, JLinkedHashMap, JavaEq, JavaHash,
    JavaStr, Shared,
};
use crate::text_token::TextToken;
use crate::text_token_visitor::TextTokenVisitor;
use crate::type_annotation::{PathValue, TypeAnnotation};
use crate::var_type::VarType;
use crate::variable_text_token::VariableTextToken;

/// `Boolean.getBoolean("decompiler.allow.text.buffer.to.string")`: never set.
const ALLOW_TO_STRING: bool = false;

fn units(s: &str) -> Vec<u16> {
    if s.is_ascii() {
        s.bytes().map(u16::from).collect()
    } else {
        s.utf16()
    }
}

fn push_units(sb: &mut Vec<u16>, s: &str) {
    if s.is_ascii() {
        sb.extend(s.bytes().map(u16::from));
    } else {
        sb.extend(s.utf16());
    }
}

fn text_of(u: &[u16]) -> String {
    if u.iter().all(|&c| c < 0x80) {
        u.iter().map(|&c| c as u8 as char).collect()
    } else {
        from_utf16(u)
    }
}

/// `String.indexOf(str, fromIndex)` over UTF-16 units.
fn index_of(hay: &[u16], pat: &[u16], from: i32) -> i32 {
    let n = hay.len() as i32;
    let from = from.max(0);
    if from >= n {
        return if pat.is_empty() { n } else { -1 };
    }
    if pat.is_empty() {
        return from;
    }
    let last = n - pat.len() as i32;
    let mut i = from;
    while i <= last {
        if hay[i as usize..i as usize + pat.len()] == *pat {
            return i;
        }
        i += 1;
    }
    -1
}

/// `String.lastIndexOf(str, fromIndex)` over UTF-16 units.
fn last_index_of(hay: &[u16], pat: &[u16], from: i32) -> i32 {
    let right = hay.len() as i32 - pat.len() as i32;
    let mut i = from.min(right);
    if i < 0 {
        return -1;
    }
    if pat.is_empty() {
        return i;
    }
    while i >= 0 {
        if hay[i as usize..i as usize + pat.len()] == *pat {
            return i;
        }
        i -= 1;
    }
    -1
}

/// `String.split(separator)` for a literal separator: trailing empty strings dropped.
fn split(s: &[u16], sep: &[u16]) -> Vec<Vec<u16>> {
    let mut out = Vec::new();
    let mut start = 0usize;
    loop {
        let at = index_of(s, sep, start as i32);
        if at < 0 || sep.is_empty() {
            break;
        }
        out.push(s[start..at as usize].to_vec());
        start = at as usize + sep.len();
    }
    if out.is_empty() {
        return vec![s.to_vec()];
    }
    out.push(s[start..].to_vec());
    while out.last().is_some_and(|l| l.is_empty()) {
        out.pop();
    }
    out
}

/// `String.trim()`.
fn trim(s: &[u16]) -> &[u16] {
    let start = s.iter().position(|&c| c > 0x20).unwrap_or(s.len());
    let end = s.iter().rposition(|&c| c > 0x20).map_or(start, |e| e + 1);
    &s[start..end.max(start)]
}

/// `String.isBlank()`.
fn is_blank(s: &str) -> bool {
    s.utf16().iter().all(|&c| jchar::is_whitespace(c))
}

/// The UTF-16 index of the first `c` in `s`, or -1.
fn jindex_of_char(s: &str, c: char) -> i32 {
    match s.find(c) {
        Some(b) => s[..b].jlen() as i32,
        None => -1,
    }
}

type FormatCache = Option<(String, String, Rc<[u16]>, Rc<[u16]>)>;

thread_local! {
    static FORMAT_CACHE: RefCell<FormatCache> = const { RefCell::new(None) };
}

fn format_units(line_separator: &str, indent: &str) -> (Rc<[u16]>, Rc<[u16]>) {
    FORMAT_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some((sep, ind, sep_units, ind_units)) = cache.as_ref() {
            if sep == line_separator && ind == indent {
                return (sep_units.clone(), ind_units.clone());
            }
        }
        let sep_units: Rc<[u16]> = units(line_separator).into();
        let ind_units: Rc<[u16]> = units(indent).into();
        *cache = Some((
            line_separator.to_string(),
            indent.to_string(),
            sep_units.clone(),
            ind_units.clone(),
        ));
        (sep_units, ind_units)
    })
}

struct Replacement {
    my_start: Cell<i32>,
    my_length: i32,
    my_dedent: bool,
}

#[derive(Clone)]
struct NewlineGroup {
    my_parent: Option<usize>,
    my_start: i32,
    my_length: i32,
    my_base_indent: i32,
    my_extra_indent: i32,
    my_children: Vec<usize>,
    my_replacements: Vec<Rc<Replacement>>,
    my_tokens: Vec<TextToken>,
}

impl NewlineGroup {
    fn new(parent: Option<usize>, start: i32, base_indent: i32, extra_indent: i32) -> NewlineGroup {
        NewlineGroup {
            my_parent: parent,
            my_start: start,
            my_length: 0,
            my_base_indent: base_indent,
            my_extra_indent: extra_indent,
            my_children: Vec::new(),
            my_replacements: Vec::new(),
            my_tokens: Vec::new(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct BytecodeMappingKey {
    my_bytecode_offset: i32,
    my_class: Option<String>,
    my_method: Option<String>,
}

impl BytecodeMappingKey {
    fn new(bytecode_offset: i32, class_name: Option<String>, method_key: Option<String>) -> Self {
        BytecodeMappingKey {
            my_bytecode_offset: bytecode_offset,
            my_class: class_name,
            my_method: method_key,
        }
    }
}

impl JavaEq for BytecodeMappingKey {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl JavaHash for BytecodeMappingKey {
    fn java_hash(&self) -> i32 {
        objects_hash(&[
            self.my_bytecode_offset,
            self.my_class.java_hash(),
            self.my_method.java_hash(),
        ])
    }
}

impl std::fmt::Display for BytecodeMappingKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.my_class.as_deref().unwrap_or("null"),
            self.my_method.as_deref().unwrap_or("null"),
            self.my_bytecode_offset
        )
    }
}

/// Java source text under construction, with its line, token and bytecode mappings.
#[derive(Clone)]
pub struct TextBuffer {
    my_line_separator: Rc<[u16]>,
    my_indent: Rc<[u16]>,
    my_preferred_line_length: i32,
    my_groups: Vec<NewlineGroup>,
    my_current_group: usize,
    my_has_reformatted: bool,
    my_string_builder: Vec<u16>,
    my_line_to_offset_mapping: Option<JHashMap<i32, i32>>,
    my_bytecode_offset_mapping: JLinkedHashMap<BytecodeMappingKey, i32>,
    my_line_mapping: Option<JHashMap<i32, BTreeSet<i32>>>,
}

impl Default for TextBuffer {
    fn default() -> Self {
        TextBuffer::new()
    }
}

const ROOT: usize = 0;

impl TextBuffer {
    fn assemble(
        line_separator: &str,
        indent: &str,
        preferred_line_length: i32,
        string_builder: Vec<u16>,
    ) -> TextBuffer {
        let (my_line_separator, my_indent) = format_units(line_separator, indent);
        TextBuffer {
            my_line_separator,
            my_indent,
            my_preferred_line_length: preferred_line_length,
            my_groups: vec![NewlineGroup::new(None, 0, 0, 0)],
            my_current_group: ROOT,
            my_has_reformatted: false,
            my_string_builder: string_builder,
            my_line_to_offset_mapping: None,
            my_bytecode_offset_mapping: JLinkedHashMap::new(),
            my_line_mapping: None,
        }
    }

    fn from_context(string_builder: Vec<u16>) -> TextBuffer {
        let line_separator = DecompilerContext::get_new_line_separator().to_string();
        let indent = DecompilerContext::get_property(IFernflowerPreferences::INDENT_STRING)
            .unwrap_or_else(|| "null".to_string());
        let preferred =
            DecompilerContext::get_int_option(IFernflowerPreferences::PREFERRED_LINE_LENGTH);
        TextBuffer::assemble(&line_separator, &indent, preferred, string_builder)
    }

    pub fn new() -> TextBuffer {
        TextBuffer::from_context(Vec::new())
    }

    /// Java: `TextBuffer(int size)`.
    pub fn with_capacity(size: i32) -> TextBuffer {
        TextBuffer::from_context(Vec::with_capacity(size.max(0) as usize))
    }

    /// Java: `TextBuffer(String text)`.
    pub fn from_text(text: &str) -> TextBuffer {
        TextBuffer::from_context(units(text))
    }

    /// A buffer with explicit formatting settings instead of the current
    /// context's (not in Java; for tests and tools).
    pub fn with_format(
        line_separator: &str,
        indent: &str,
        preferred_line_length: i32,
    ) -> TextBuffer {
        TextBuffer::assemble(line_separator, indent, preferred_line_length, Vec::new())
    }

    /// Java: `append(String)`.
    pub fn append(&mut self, str: &str) -> &mut Self {
        push_units(&mut self.my_string_builder, str);
        self
    }

    /// Java: `append(char)`.
    pub fn append_char(&mut self, ch: char) -> &mut Self {
        if (ch as u32) < 0x80 {
            self.my_string_builder.push(ch as u16);
        } else {
            push_units(&mut self.my_string_builder, ch.encode_utf8(&mut [0; 4]));
        }
        self
    }

    /// Java: `append(int)`.
    pub fn append_int(&mut self, i: i32) -> &mut Self {
        self.append(&i.to_string())
    }

    pub fn append_line_separator(&mut self) -> &mut Self {
        let sep = self.my_line_separator.clone();
        self.my_string_builder.extend_from_slice(&sep);
        self
    }

    pub fn append_indent(&mut self, length: i32) -> &mut Self {
        let indent = self.my_indent.clone();
        for _ in 0..length.max(0) {
            self.my_string_builder.extend_from_slice(&indent);
        }
        self
    }

    /// Java: `appendCastTypeName(VarType, List, Set)`.
    pub fn append_cast_type_name_annotations(
        &mut self,
        type_: &VarType,
        type_annotations: &mut Vec<Pair<VecDeque<PathValue>, AnnotationExprent>>,
        filter: &JHashSet<String>,
    ) -> &mut Self {
        self.append_cast_type_name_annotations_vararg(type_, type_annotations, filter, false, true)
    }

    /// Java: `appendCastTypeName(VarType, List, Set, boolean vararg, boolean flush)`.
    pub fn append_cast_type_name_annotations_vararg(
        &mut self,
        type_: &VarType,
        type_annotations: &mut Vec<Pair<VecDeque<PathValue>, AnnotationExprent>>,
        filter: &JHashSet<String>,
        vararg: bool,
        flush: bool,
    ) -> &mut Self {
        TypeAnnotation::append_cast_type_name(self, type_, type_annotations, filter, vararg, flush);
        self
    }

    /// Java: `appendCastTypeName(VarType)`.
    pub fn append_cast_type_name(&mut self, type_: &VarType) -> &mut Self {
        let cast_name = ExprProcessor::get_cast_type_name(type_);
        self.append_cast_type_name_str(&cast_name, type_)
    }

    /// Java: `appendCastTypeName(String castName, VarType)`.
    pub fn append_cast_type_name_str(&mut self, cast_name: &str, type_: &VarType) -> &mut Self {
        if type_.type_ == CodeType::Object {
            if type_.array_dim > 0 {
                let cut = cast_name.jlen() - (type_.array_dim * 2) as usize;
                let name = cast_name.jsubstring(0, cut);
                self.append_all_classes(&name, &type_.value);
                let at = self.length() - name.jlen() as i32;
                self.add_generic_type_tokens(at, &name, type_);
                self.append(&cast_name.jsubstring_from(cut));
            } else {
                self.append_all_classes(cast_name, &type_.value);
                let at = self.length() - cast_name.jlen() as i32;
                self.add_generic_type_tokens(at, cast_name, type_);
            }
            return self;
        }
        self.append(cast_name)
    }

    /// Java: `appendTypeName(VarType)`.
    pub fn append_type_name(&mut self, type_: &VarType) -> &mut Self {
        let name = ExprProcessor::get_type_name(type_);
        self.append_type_name_str(&name, type_)
    }

    /// Java: `appendTypeName(String name, VarType)`.
    pub fn append_type_name_str(&mut self, name: &str, type_: &VarType) -> &mut Self {
        if type_.type_ == CodeType::Object {
            self.append_all_classes(name, &type_.value);
            let at = self.length() - name.jlen() as i32;
            self.add_generic_type_tokens(at, name, type_);
            return self;
        }
        self.append(name)
    }

    pub fn add_type_name_token(&mut self, type_: &VarType, index: i32) -> &mut Self {
        if type_.type_ == CodeType::Object {
            let name = ExprProcessor::get_type_name(type_);
            self.add_all_class_tokens(index, &name, &type_.value);
            self.add_generic_type_tokens(index, &name, type_);
        }
        self
    }

    pub fn append_all_classes(&mut self, text: &str, qualified_name: &str) -> &mut Self {
        let at = self.length();
        self.add_all_class_tokens(at, text, qualified_name);
        self.append(text)
    }

    pub fn add_all_class_tokens(
        &mut self,
        index: i32,
        text: &str,
        qualified_name: &str,
    ) -> &mut Self {
        let mut index = index;
        let mut text = text;
        if let Some(lt) = text.find('<') {
            text = &text[..lt];
        }
        let mut pos = 0usize;
        while text.contains('.') && qualified_name[pos..].contains('$') {
            let dot = text.find('.').expect("contains '.'");
            let length = text[..dot].jlen() as i32;
            let dollar = pos + qualified_name[pos..].find('$').expect("contains '$'");
            self.add_class_token(index, length, &qualified_name[..dollar]);
            pos = dollar + 1;
            text = &text[dot + 1..];
            index += length + 1;
        }
        if !is_blank(text) {
            self.add_class_token(index, text.jlen() as i32, qualified_name);
        }
        self
    }

    pub fn add_generic_type_tokens(
        &mut self,
        index: i32,
        text: &str,
        type_: &VarType,
    ) -> &mut Self {
        if !type_.is_generic() {
            return self;
        }
        let gen_type: GenericType = type_.as_generic();
        let index = index + jindex_of_char(text, '<') + 1;
        let mut offset = 0;
        let arguments = gen_type.get_arguments();
        for argument in arguments.iter() {
            match argument.clone() {
                None => offset += 1,
                Some(t) if t.is_generic() => {
                    let gt: GenericType = t.as_generic();
                    let wildcard = gt.get_wildcard();
                    if wildcard == GenericType::WILDCARD_EXTENDS {
                        offset += 10;
                    } else if wildcard == GenericType::WILDCARD_SUPER {
                        offset += 8;
                    }
                    let name = GenericMain::get_generic_cast_type_name(&gt);
                    if gt.type_ == CodeType::Object {
                        self.add_all_class_tokens(index + offset, &name, &gt.value);
                    }
                    self.add_generic_type_tokens(index + offset, &name, &gt);
                    offset += name.jlen() as i32;
                }
                Some(t) => {
                    if t.type_ == CodeType::Object {
                        self.add_type_name_token(&t, index + offset);
                    }
                    offset += ExprProcessor::get_cast_type_name(&t).jlen() as i32;
                }
            }
            offset += 2;
        }
        self
    }

    pub fn add_class_token(&mut self, index: i32, length: i32, name: &str) -> &mut Self {
        self.add_token(ClassTextToken::new(index, length, false, name.to_string()));
        self
    }

    pub fn append_class(
        &mut self,
        value: &str,
        declaration: bool,
        qualified_name: &str,
    ) -> &mut Self {
        let at = self.length();
        self.add_token(ClassTextToken::new(
            at,
            value.jlen() as i32,
            declaration,
            qualified_name.to_string(),
        ));
        self.append(value)
    }

    /// Java: `appendField(String, boolean, String, String, String descriptor)`.
    pub fn append_field(
        &mut self,
        value: &str,
        declaration: bool,
        class_name: &str,
        name: &str,
        descriptor: &str,
    ) -> &mut Self {
        let descriptor = FieldDescriptor::parse_descriptor(descriptor);
        self.append_field_descriptor(value, declaration, class_name, name, &descriptor)
    }

    /// Java: `appendField(String, boolean, String, String, FieldDescriptor)`.
    pub fn append_field_descriptor(
        &mut self,
        value: &str,
        declaration: bool,
        class_name: &str,
        name: &str,
        descriptor: &FieldDescriptor,
    ) -> &mut Self {
        let at = self.length();
        self.add_token(FieldTextToken::new(
            at,
            value.jlen() as i32,
            declaration,
            class_name.to_string(),
            name.to_string(),
            descriptor.clone(),
        ));
        self.append(value)
    }

    /// Java: `appendMethod(String, boolean, String, String, String descriptor)`.
    pub fn append_method(
        &mut self,
        value: &str,
        declaration: bool,
        class_name: &str,
        name: &str,
        descriptor: &str,
    ) -> &mut Self {
        let descriptor = MethodDescriptor::parse_descriptor(descriptor);
        self.append_method_descriptor(value, declaration, class_name, name, &descriptor)
    }

    /// Java: `appendMethod(String, boolean, String, String, MethodDescriptor)`.
    pub fn append_method_descriptor(
        &mut self,
        value: &str,
        declaration: bool,
        class_name: &str,
        name: &str,
        descriptor: &MethodDescriptor,
    ) -> &mut Self {
        let at = self.length();
        self.add_token(MethodTextToken::new(
            at,
            value.jlen() as i32,
            declaration,
            class_name.to_string(),
            name.to_string(),
            descriptor.clone(),
        ));
        self.append(value)
    }

    /// Java: `appendVariable(String, boolean, boolean, String, String, String methodDescriptor, int, String)`.
    #[allow(clippy::too_many_arguments)]
    pub fn append_variable(
        &mut self,
        value: &str,
        declaration: bool,
        parameter: bool,
        class_name: &str,
        method_name: &str,
        method_descriptor: &str,
        index: i32,
        name: &str,
    ) -> &mut Self {
        let descriptor = MethodDescriptor::parse_descriptor(method_descriptor);
        self.append_variable_descriptor(
            value,
            declaration,
            parameter,
            class_name,
            method_name,
            &descriptor,
            index,
            name,
        )
    }

    /// Java: `appendVariable(String, boolean, boolean, String, String, MethodDescriptor, int, String)`.
    #[allow(clippy::too_many_arguments)]
    pub fn append_variable_descriptor(
        &mut self,
        value: &str,
        declaration: bool,
        parameter: bool,
        class_name: &str,
        method_name: &str,
        method_descriptor: &MethodDescriptor,
        index: i32,
        name: &str,
    ) -> &mut Self {
        let at = self.length();
        self.add_token(VariableTextToken::new(
            at,
            value.jlen() as i32,
            declaration,
            parameter,
            class_name.to_string(),
            method_name.to_string(),
            method_descriptor.clone(),
            index,
            name.to_string(),
        ));
        self.append(value)
    }

    fn add_token(&mut self, token: TextToken) {
        self.my_groups[self.my_current_group].my_tokens.push(token);
    }

    /// Opens a group of possible newlines that are either all taken or none.
    pub fn push_newline_group(&mut self, base_indent: i32, extra_indent: i32) -> &mut Self {
        let group = NewlineGroup::new(
            Some(self.my_current_group),
            self.my_string_builder.len() as i32,
            base_indent,
            extra_indent,
        );
        let index = self.my_groups.len();
        self.my_groups.push(group);
        self.my_groups[self.my_current_group]
            .my_children
            .push(index);
        self.my_current_group = index;
        self
    }

    /// Java: `appendPossibleNewline()`.
    pub fn append_possible_newline(&mut self) -> &mut Self {
        self.append_possible_newline_alt("")
    }

    /// Java: `appendPossibleNewline(String alternative)`.
    pub fn append_possible_newline_alt(&mut self, alternative: &str) -> &mut Self {
        self.append_possible_newline_dedent(alternative, false)
    }

    /// Java: `appendPossibleNewline(String alternative, boolean dedent)`.
    pub fn append_possible_newline_dedent(&mut self, alternative: &str, dedent: bool) -> &mut Self {
        let replacement = Rc::new(Replacement {
            my_start: Cell::new(self.my_string_builder.len() as i32),
            my_length: alternative.jlen() as i32,
            my_dedent: dedent,
        });
        self.my_groups[self.my_current_group]
            .my_replacements
            .push(replacement);
        self.append(alternative)
    }

    pub fn pop_newline_group(&mut self) -> &mut Self {
        if self.my_current_group == ROOT {
            jthrow!(IllegalStateException; "Cannot pop root group: {}", self.convert_to_string_and_allow_data_discard());
        }
        let length = self.my_string_builder.len() as i32;
        let group = &mut self.my_groups[self.my_current_group];
        group.my_length = length - group.my_start;
        self.my_current_group = group.my_parent.expect("a non-root group has a parent");
        self
    }

    pub fn prepend(&mut self, s: &str) -> &mut Self {
        let u = units(s);
        let shift = u.len() as i32;
        self.my_string_builder.splice(0..0, u);
        self.shift_mapping(shift);
        self
    }

    pub fn enclose(&mut self, left: &str, right: &str) -> &mut Self {
        self.prepend(left);
        self.append(right);
        self
    }

    pub fn enclose_with_parens(&mut self) -> &mut Self {
        self.enclose("(", ")")
    }

    pub fn contains_only_whitespaces(&self) -> bool {
        self.my_string_builder.iter().all(|&c| c == b' ' as u16)
    }

    /// Java: `addBytecodeMapping(int)`.
    pub fn add_bytecode_mapping_offset(&mut self, bytecode_offset: i32) {
        let at = self.my_string_builder.len() as i32;
        self.my_bytecode_offset_mapping
            .put_if_absent(BytecodeMappingKey::new(bytecode_offset, None, None), at);
    }

    /// Java: `addStartBytecodeMapping(int)`.
    pub fn add_start_bytecode_mapping_offset(&mut self, bytecode_offset: i32) {
        self.my_bytecode_offset_mapping
            .put_if_absent(BytecodeMappingKey::new(bytecode_offset, None, None), 0);
    }

    /// Java: `addBytecodeMapping(BitSet)`.
    pub fn add_bytecode_mapping(&mut self, bytecode_offsets: Option<&BitSet>) {
        let Some(offsets) = bytecode_offsets else {
            return;
        };
        for i in offsets.iter() {
            self.add_bytecode_mapping_offset(i as i32);
        }
    }

    /// Java: `addStartBytecodeMapping(BitSet)`.
    pub fn add_start_bytecode_mapping(&mut self, bytecode_offsets: Option<&BitSet>) {
        let Some(offsets) = bytecode_offsets else {
            return;
        };
        for i in offsets.iter() {
            self.add_start_bytecode_mapping_offset(i as i32);
        }
    }

    pub fn clear_unassigned_bytecode_mapping_data(&mut self) {
        self.my_bytecode_offset_mapping
            .retain(|key, _| key.my_class.is_some());
    }

    /// One tracer per `(class, method key)`, in mapping order, each fed the
    /// source line of every bytecode offset.
    pub fn get_tracers(
        &self,
    ) -> JLinkedHashMap<Pair<String, Option<String>>, BytecodeMappingTracer> {
        let sb = &self.my_string_builder;
        let sep = &self.my_line_separator;
        let mut newline_offsets: Vec<i32> = Vec::new();
        let mut i = index_of(sb, sep, 0);
        while i != -1 {
            newline_offsets.push(i);
            i = index_of(sb, sep, i + 1);
        }
        let mut tracers: JLinkedHashMap<Pair<String, Option<String>>, BytecodeMappingTracer> =
            JLinkedHashMap::new();
        for (key, &text_offset) in self.my_bytecode_offset_mapping.iter() {
            let Some(class_name) = &key.my_class else {
                jthrow!(IllegalStateException;
                    "getTracers called when not all bytecode offsets have a valid class and method");
            };
            let tracer = *tracers.compute_if_absent(
                Pair::of(class_name.clone(), key.my_method.clone()),
                BytecodeMappingTracer::new,
            );
            let line_no = match newline_offsets.binary_search(&text_offset) {
                Ok(at) | Err(at) => at as i32,
            };
            tracer.set_current_source_line(line_no);
            tracer.add_mapping(key.my_bytecode_offset);
        }
        tracers
    }

    fn reformat_group(&mut self, group: usize, offset_mapping: &mut Vec<i32>, extra_indent: i32) {
        let mut extra_indent = extra_indent;
        let sep = self.my_line_separator.clone();
        let indent = self.my_indent.clone();
        let sep_len = sep.len() as i32;
        let indent_len = indent.len() as i32;
        let (g_start, g_length, g_base_indent, g_extra_indent) = {
            let g = &self.my_groups[group];
            (g.my_start, g.my_length, g.my_base_indent, g.my_extra_indent)
        };
        let children = self.my_groups[group].my_children.clone();
        let replacements = self.my_groups[group].my_replacements.clone();

        let mut offset = offset_mapping[g_start as usize];
        let actual_start = g_start + offset;
        let last_newline = last_index_of(&self.my_string_builder, &sep, actual_start);
        let mut next_newline = index_of(&self.my_string_builder, &sep, actual_start);
        let first_part_end = if next_newline == -1 {
            actual_start + g_length
        } else {
            next_newline.min(actual_start + g_length)
        };
        let mut group_end_without_newlines = if last_newline == -1 {
            first_part_end
        } else {
            first_part_end - last_newline
        };
        while next_newline != -1 && next_newline <= actual_start + g_length {
            let line_start = next_newline;
            next_newline = index_of(&self.my_string_builder, &sep, next_newline + 1);
            let mut line_end = next_newline;
            if line_end == -1 || line_end > actual_start + g_length {
                line_end = actual_start + g_length;
            }
            let line_length = extra_indent + line_end - line_start - sep_len;
            if line_length > group_end_without_newlines {
                group_end_without_newlines = line_length;
            }
        }
        let add_new_lines = group_end_without_newlines > self.my_preferred_line_length;

        let original_extra_indent = extra_indent;
        if add_new_lines && !replacements.is_empty() {
            extra_indent += g_extra_indent;
        }

        let mut children_index = 0;
        let mut replacement_index = 0;
        let mut pos = g_start;
        while pos <= g_start + g_length {
            if pos != g_start {
                offset_mapping.push(offset);
            }

            let len = self.my_string_builder.len() as i32;
            let at = pos + offset;
            if at + sep_len < len
                && self.my_string_builder[at as usize..(at + sep_len) as usize] == *sep
                && (at + sep_len * 2 >= len
                    || self.my_string_builder[(at + sep_len) as usize..(at + sep_len * 2) as usize]
                        != *sep)
            {
                let insert_at = (at + sep_len) as usize;
                let repeated: Vec<u16> = indent.repeat(extra_indent.max(0) as usize);
                self.my_string_builder
                    .splice(insert_at..insert_at, repeated);
                offset += indent_len * extra_indent;
            }

            let mut another_pass = true;
            while another_pass {
                another_pass = false;

                if add_new_lines
                    && replacement_index < replacements.len()
                    && pos == replacements[replacement_index].my_start.get()
                {
                    let replacement = &replacements[replacement_index];
                    let from = (pos + offset) as usize;
                    let to = ((pos + offset + replacement.my_length) as usize)
                        .min(self.my_string_builder.len());
                    self.my_string_builder.splice(from..to, sep.iter().copied());
                    if replacement.my_dedent {
                        extra_indent = original_extra_indent;
                    }
                    let insert_at = (pos + offset + sep_len) as usize;
                    let count = (g_base_indent + extra_indent).max(0) as usize;
                    self.my_string_builder
                        .splice(insert_at..insert_at, indent.repeat(count));
                    offset += indent_len * (g_base_indent + extra_indent) + sep_len
                        - replacement.my_length;
                    replacement_index += 1;
                    another_pass = true;
                }
                *offset_mapping.last_mut().expect("offset mapping") = offset;

                let current_pos = pos;
                if children_index < children.len()
                    && self.my_groups[children[children_index]].my_start == current_pos
                {
                    let child = children[children_index];
                    self.reformat_group(child, offset_mapping, extra_indent);
                    offset = *offset_mapping.last().expect("offset mapping");
                    pos += self.my_groups[child].my_length;
                    children_index += 1;
                    another_pass = true;
                }
            }
            pos += 1;
        }

        *offset_mapping.last_mut().expect("offset mapping") = offset;

        for token in &mut self.my_groups[group].my_tokens {
            let start = token.get_start();
            if start < offset_mapping.len() as i32 {
                let at = usize::try_from(start).expect("token before the start of the text");
                token.shift(offset_mapping[at]);
            }
        }
    }

    /// Takes the possible newlines of every group whose text runs past the
    /// preferred line length, keeping the mappings in step.
    pub fn reformat(&mut self) {
        if self.my_current_group != ROOT {
            jthrow!(IllegalStateException; "Cannot reformat while in a group");
        }
        if self.my_has_reformatted {
            jthrow!(IllegalStateException; "Cannot reformat twice");
        }
        self.my_has_reformatted = true;
        self.my_groups[ROOT].my_length = self.my_string_builder.len() as i32;
        let mut offset_mapping: Vec<i32> = Vec::with_capacity(self.my_string_builder.len() + 1);
        offset_mapping.push(0);
        self.reformat_group(ROOT, &mut offset_mapping, 0);
        self.my_bytecode_offset_mapping
            .for_each_value_mut(|_, value| *value += offset_mapping[*value as usize]);
    }

    pub fn content_equals(&self, string: &str) -> bool {
        self.my_string_builder == units(string)
    }

    pub fn convert_to_string_and_allow_data_discard(&self) -> String {
        let original = &self.my_string_builder;
        let line_to_offset = match &self.my_line_to_offset_mapping {
            Some(map) if !map.is_empty() => map,
            _ => {
                if self.my_line_mapping.is_some() {
                    return self.add_original_line_numbers();
                }
                return text_of(original);
            }
        };
        let sep = &self.my_line_separator;
        let mut res: Vec<u16> = Vec::new();
        let src_lines = split(original, sep);
        let mut current_line_start_offset = 0i32;
        let mut current_line = 0usize;
        let mut previous_mark_line = 0usize;
        let mut dumped_lines = 0i32;
        let mut lines_with_marks = line_to_offset.key_vec();
        lines_with_marks.sort();
        for mark_line in lines_with_marks {
            let mark_offset = *line_to_offset.get(&mark_line).expect("mark line");
            while current_line < src_lines.len() {
                let line = &src_lines[current_line];
                let line_end = current_line_start_offset + line.len() as i32 + sep.len() as i32;
                if mark_offset <= line_end {
                    let required_line = mark_line - 1;
                    let lines_to_add = required_line - dumped_lines;
                    dumped_lines = required_line;
                    self.append_lines(
                        &mut res,
                        &src_lines,
                        previous_mark_line,
                        current_line,
                        lines_to_add,
                    );
                    previous_mark_line = current_line;
                    break;
                }
                current_line_start_offset = line_end;
                current_line += 1;
            }
        }
        if previous_mark_line < src_lines.len() {
            let n = (src_lines.len() - previous_mark_line) as i32;
            self.append_lines(&mut res, &src_lines, previous_mark_line, src_lines.len(), n);
        }
        text_of(&res)
    }

    fn add_original_line_numbers(&self) -> String {
        let sb = &self.my_string_builder;
        let sep = &self.my_line_separator;
        let line_mapping = self.my_line_mapping.as_ref().expect("line mapping");
        let mut out: Vec<u16> = Vec::new();
        let mut line_start = 0i32;
        let mut count = 0;
        let length = sep.len() as i32;
        loop {
            let line_end = index_of(sb, sep, line_start);
            if line_end <= 0 {
                break;
            }
            count += 1;
            out.extend_from_slice(&sb[line_start as usize..line_end as usize]);
            if let Some(integers) = line_mapping.get(&count) {
                push_units(&mut out, "//");
                for integer in integers {
                    out.push(b' ' as u16);
                    push_units(&mut out, &integer.to_string());
                }
            }
            out.extend_from_slice(sep);
            line_start = line_end + length;
        }
        if (line_start as usize) < sb.len() {
            out.extend_from_slice(&sb[line_start as usize..]);
        }
        text_of(&out)
    }

    fn append_lines(
        &self,
        res: &mut Vec<u16>,
        src_lines: &[Vec<u16>],
        from: usize,
        to: usize,
        required_line_number: i32,
    ) {
        let sep = &self.my_line_separator;
        let span = to as i32 - from as i32;
        if span > required_line_number {
            let strings =
                TextBuffer::compact_lines(src_lines[from..to].to_vec(), required_line_number);
            let mut separators_required = required_line_number - 1;
            for s in strings {
                res.extend_from_slice(&s);
                let more = separators_required > 0;
                separators_required -= 1;
                if more {
                    res.extend_from_slice(sep);
                }
            }
            res.extend_from_slice(sep);
        } else {
            for line in &src_lines[from..to] {
                res.extend_from_slice(line);
                res.extend_from_slice(sep);
            }
            for _ in 0..(required_line_number - span).max(0) {
                res.extend_from_slice(sep);
            }
        }
    }

    pub fn length(&self) -> i32 {
        self.my_string_builder.len() as i32
    }

    /// Drops the first `position` units (`StringBuilder.delete(0, position)`).
    pub fn set_start(&mut self, position: i32) {
        if position < 0 {
            exception::throw(
                "StringIndexOutOfBoundsException",
                format!("start 0, end {position}"),
            );
        }
        let end = (position as usize).min(self.my_string_builder.len());
        self.my_string_builder.drain(..end);
        self.shift_mapping(-position);
    }

    pub fn set_length(&mut self, position: i32) {
        if position < 0 {
            exception::throw(
                "StringIndexOutOfBoundsException",
                format!("String index out of range: {position}"),
            );
        }
        self.my_string_builder.resize(position as usize, 0);
        if let Some(map) = &self.my_line_to_offset_mapping {
            let mut new_map = JHashMap::new();
            for (&key, &value) in map.iter() {
                if value <= position {
                    new_map.put(key, value);
                }
            }
            self.my_line_to_offset_mapping = Some(new_map);
        }
        self.truncate_group(ROOT, position);
    }

    fn truncate_group(&mut self, group: usize, string_length: i32) {
        {
            let g = &mut self.my_groups[group];
            if g.my_start + g.my_length > string_length {
                g.my_length = string_length - g.my_start;
            }
        }
        let children = std::mem::take(&mut self.my_groups[group].my_children);
        let mut kept = Vec::with_capacity(children.len());
        for child in children {
            if self.my_groups[child].my_start <= string_length {
                self.truncate_group(child, string_length);
                kept.push(child);
            }
        }
        let g = &mut self.my_groups[group];
        g.my_children = kept;
        g.my_replacements
            .retain(|r| r.my_start.get() <= string_length);
        g.my_tokens.retain(|t| t.get_start() <= string_length);
    }

    /// Java: `append(TextBuffer buffer, String className, String methodKey)`;
    /// bytecode offsets of `buffer` not yet bound to a method are bound to this one.
    pub fn append_buffer_mapped(
        &mut self,
        buffer: &TextBuffer,
        class_name: Option<&str>,
        method_key: Option<&str>,
    ) -> &mut Self {
        if buffer.my_current_group != ROOT {
            jthrow!(IllegalArgumentException; "Can't append buffer with non-root group");
        }
        let len = self.my_string_builder.len() as i32;
        if let Some(other) = &buffer.my_line_to_offset_mapping {
            if !other.is_empty() {
                self.check_map_created();
                let map = self.my_line_to_offset_mapping.as_mut().expect("created");
                for (&key, &value) in other.iter() {
                    map.put(key, value + len);
                }
            }
        }
        for (key, &value) in buffer.my_bytecode_offset_mapping.iter() {
            let key = if key.my_class.is_none() {
                BytecodeMappingKey::new(
                    key.my_bytecode_offset,
                    class_name.map(str::to_string),
                    method_key.map(str::to_string),
                )
            } else {
                key.clone()
            };
            self.my_bytecode_offset_mapping
                .put_if_absent(key, value + len);
        }
        let other_root = &buffer.my_groups[ROOT];
        let replacements = other_root.my_replacements.clone();
        let mut tokens = other_root.my_tokens.clone();
        let current = self.my_current_group;
        let children: Vec<usize> = other_root
            .my_children
            .iter()
            .map(|&child| self.copy_group_from(buffer, child, current))
            .collect();
        for r in &replacements {
            r.my_start.set(r.my_start.get() + len);
        }
        for token in &mut tokens {
            token.shift(len);
        }
        for &child in &children {
            self.shift_group(child, len);
        }
        let g = &mut self.my_groups[current];
        g.my_replacements.extend(replacements);
        g.my_tokens.extend(tokens);
        g.my_children.extend(children);
        self.my_string_builder
            .extend_from_slice(&buffer.my_string_builder);
        self
    }

    /// Java: `append(TextBuffer)`.
    pub fn append_buffer(&mut self, buffer: &TextBuffer) -> &mut Self {
        self.append_buffer_mapped(buffer, None, None)
    }

    fn copy_group_from(&mut self, from: &TextBuffer, group: usize, parent: usize) -> usize {
        let source = &from.my_groups[group];
        let index = self.my_groups.len();
        self.my_groups.push(NewlineGroup {
            my_parent: Some(parent),
            my_start: source.my_start,
            my_length: source.my_length,
            my_base_indent: source.my_base_indent,
            my_extra_indent: source.my_extra_indent,
            my_children: Vec::new(),
            my_replacements: source.my_replacements.clone(),
            my_tokens: source.my_tokens.clone(),
        });
        let children: Vec<usize> = source
            .my_children
            .iter()
            .map(|&child| self.copy_group_from(from, child, index))
            .collect();
        self.my_groups[index].my_children = children;
        index
    }

    fn shift_group(&mut self, group: usize, amount: i32) {
        let g = &mut self.my_groups[group];
        g.my_start += amount;
        for r in &g.my_replacements {
            r.my_start.set(r.my_start.get() + amount);
        }
        for token in &mut g.my_tokens {
            token.shift(amount);
        }
        let children = g.my_children.clone();
        for child in children {
            self.shift_group(child, amount);
        }
    }

    fn shift_mapping(&mut self, shift_offset: i32) {
        if let Some(map) = &self.my_line_to_offset_mapping {
            let mut new_map = JHashMap::new();
            for (&key, &value) in map.iter() {
                let mut new_value = value;
                if new_value >= 0 {
                    new_value += shift_offset;
                }
                if new_value >= 0 {
                    new_map.put(key, new_value);
                }
            }
            self.my_line_to_offset_mapping = Some(new_map);
        }
        self.my_bytecode_offset_mapping
            .for_each_value_mut(|_, value| *value += shift_offset);
        self.shift_group(ROOT, shift_offset);
    }

    fn check_map_created(&mut self) {
        if self.my_line_to_offset_mapping.is_none() {
            self.my_line_to_offset_mapping = Some(JHashMap::new());
        }
    }

    /// Java: `countLines()`.
    pub fn count_lines(&self) -> i32 {
        self.count_lines_from(0)
    }

    /// Java: `countLines(int from)`.
    pub fn count_lines_from(&self, from: i32) -> i32 {
        let sep = text_of(&self.my_line_separator);
        self.count(&sep, from)
    }

    /// Occurrences of `substring` found at positions above 0 from `from` on.
    pub fn count(&self, substring: &str, from: i32) -> i32 {
        let pat = units(substring);
        let mut count = 0;
        let length = pat.len() as i32;
        let mut p = from;
        loop {
            p = index_of(&self.my_string_builder, &pat, p);
            if p <= 0 {
                break;
            }
            count += 1;
            p += length;
        }
        count
    }

    pub fn count_chars(&self, c: char) -> i32 {
        let unit = c as u32;
        self.my_string_builder
            .iter()
            .filter(|&&u| u as u32 == unit)
            .count() as i32
    }

    fn compact_lines(src_lines: Vec<Vec<u16>>, required_line_number: i32) -> Vec<Vec<u16>> {
        if src_lines.len() < 2 || src_lines.len() as i32 <= required_line_number {
            return src_lines;
        }
        let mut res = src_lines;
        let open = [b'{' as u16];
        let close = [b'}' as u16];
        let mut i = res.len() - 1;
        while i > 0 {
            let s = res[i].clone();
            let trimmed = trim(&s);
            if trimmed == open || trimmed == close {
                res[i - 1].extend_from_slice(&s);
                res.remove(i);
            }
            if res.len() as i32 <= required_line_number {
                return res;
            }
            i -= 1;
        }
        let mut i = res.len() - 1;
        while i > 0 {
            let s = res[i].clone();
            if trim(&s).is_empty() {
                res[i - 1].extend_from_slice(&s);
                res.remove(i);
            }
            if res.len() as i32 <= required_line_number {
                return res;
            }
            i -= 1;
        }
        res
    }

    /// `line_mapping` holds `(original line, new line)` pairs.
    pub fn dump_original_line_numbers(&mut self, line_mapping: &[i32]) {
        if !line_mapping.is_empty() {
            let mut map: JHashMap<i32, BTreeSet<i32>> = JHashMap::new();
            let mut i = 0;
            while i < line_mapping.len() {
                let key = line_mapping[i + 1];
                map.compute_if_absent(key, BTreeSet::new)
                    .insert(line_mapping[i]);
                i += 2;
            }
            self.my_line_mapping = Some(map);
        }
    }

    /// `line:column` of a text offset, as the token dump prints it.
    pub fn get_pos(&self, index: i32) -> String {
        let sep = &self.my_line_separator;
        let before = &self.my_string_builder[..index as usize];
        let line = split(before, sep).len();
        let mut line_start = last_index_of(before, sep, before.len() as i32);
        line_start += if line_start != -1 {
            sep.len() as i32
        } else {
            1
        };
        format!("{}:{}", line, index + 1 - line_start)
    }

    pub fn get_tokens(&self) -> Vec<TextToken> {
        self.flatten_tokens(ROOT)
    }

    fn flatten_tokens(&self, group: usize) -> Vec<TextToken> {
        let g = &self.my_groups[group];
        let mut result = g.my_tokens.clone();
        for &child in &g.my_children {
            result.extend(self.flatten_tokens(child));
        }
        result.sort_by_key(|t| t.get_start());
        result
    }

    pub fn visit_tokens(&self, visitor: Option<&mut dyn TextTokenVisitor>) {
        let Some(visitor) = visitor else {
            return;
        };
        visitor.start(&text_of(&self.my_string_builder));
        for token in self.get_tokens() {
            token.visit(visitor);
        }
        visitor.end();
    }

    /// `visitTokens` for a visitor that reads or writes the visited buffer
    /// itself (`TextTokenDumpVisitor`): no borrow is held while it runs.
    pub fn visit_tokens_shared(
        buffer: &Shared<TextBuffer>,
        visitor: Option<&mut dyn TextTokenVisitor>,
    ) {
        let Some(visitor) = visitor else {
            return;
        };
        let (content, tokens) = {
            let b = buffer.borrow();
            (text_of(&b.my_string_builder), b.get_tokens())
        };
        visitor.start(&content);
        for token in tokens {
            token.visit(visitor);
        }
        visitor.end();
    }

    /// The leak check of the unported `DebugTrace`: nothing to check.
    pub fn check_leaks() {}
}

/// `toString()`: discouraged, so it warns (or fails in unit-test mode) first.
impl std::fmt::Display for TextBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !ALLOW_TO_STRING {
            if DecompilerContext::get_option(IFernflowerPreferences::UNIT_TEST_MODE) {
                exception::throw("AssertionError", "Usage of TextBuffer.toString");
            } else {
                DecompilerContext::get_logger()
                    .write_message("Usage of TextBuffer.toString", Severity::Warn);
            }
        }
        f.write_str(&self.convert_to_string_and_allow_data_discard())
    }
}

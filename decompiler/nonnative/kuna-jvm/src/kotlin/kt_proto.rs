//! Port of the protobuf-lite runtime (`CodedInputStream`, `AbstractParser`,
//! `GeneratedMessageLite`) and the `ProtoBuf`/`JvmProtoBuf` messages that the
//! Kotlin plugin shades as `org/vineflower/kt/{protobuf,metadata}`.
//!
//! One schema-driven loop stands in for the generated parse constructors: the
//! field tables list every field of every message reachable from `Class`,
//! `Package`, `Function` and `StringTableTypes` (with the JVM extensions
//! registered), so unknown fields, merges, required fields and malformed input
//! behave as in the jar. The typed views expose the fields the plugin reads.
//! The shading left every runtime string constant as `"Code"`: exception
//! messages are `"Code"` and `Record.getString()` always throws.

use std::rc::Rc;

use crate::rt::{rethrow, throw, try_catch};

fn invalid_protocol_buffer() -> ! {
    throw("InvalidProtocolBufferException", "Code")
}

fn uninitialized_message() -> ! {
    throw("UninitializedMessageException", "Code")
}

fn array_index_out_of_bounds(index: i32, length: usize) -> ! {
    throw(
        "ArrayIndexOutOfBoundsException",
        format!("Index {} out of bounds for length {}", index, length),
    )
}

/// `System.arraycopy` on byte arrays, with the JVM's checks.
fn arraycopy(src: &[u8], src_pos: i32, dst: &mut [u8], dst_pos: i32, length: i32) {
    check_arraycopy(src.len(), src_pos, dst.len(), dst_pos, length);
    let (s, d, n) = (src_pos as usize, dst_pos as usize, length as usize);
    dst[d..d + n].copy_from_slice(&src[s..s + n]);
}

fn check_arraycopy(src_len: usize, src_pos: i32, dst_len: usize, dst_pos: i32, length: i32) {
    let fail = |m: String| -> ! { throw("ArrayIndexOutOfBoundsException", m) };
    if src_pos < 0 {
        fail(format!(
            "arraycopy: source index {} out of bounds for byte[{}]",
            src_pos, src_len
        ));
    }
    if dst_pos < 0 {
        fail(format!(
            "arraycopy: destination index {} out of bounds for byte[{}]",
            dst_pos, dst_len
        ));
    }
    if length < 0 {
        fail(format!("arraycopy: length {} is negative", length));
    }
    if length as u64 + src_pos as u64 > src_len as u64 {
        fail(format!(
            "arraycopy: last source index {} out of bounds for byte[{}]",
            length as u64 + src_pos as u64,
            src_len
        ));
    }
    if length as u64 + dst_pos as u64 > dst_len as u64 {
        fail(format!(
            "arraycopy: last destination index {} out of bounds for byte[{}]",
            length as u64 + dst_pos as u64,
            dst_len
        ));
    }
}

/// `java.io.InputStream`, as far as the parser uses it.
pub trait JavaInputStream {
    /// `read()`: the next byte, or -1 at the end.
    fn read(&mut self) -> i32;
    /// `read(byte[], int, int)`.
    fn read_into(&mut self, b: &mut [u8], off: i32, len: i32) -> i32;
}

/// `java.io.ByteArrayInputStream`.
pub struct ByteArrayInputStream {
    buf: Rc<[u8]>,
    pos: i32,
    count: i32,
}

impl ByteArrayInputStream {
    pub fn new(buf: Rc<[u8]>) -> ByteArrayInputStream {
        let count = buf.len() as i32;
        ByteArrayInputStream { buf, pos: 0, count }
    }

    pub fn available(&self) -> i32 {
        self.count - self.pos
    }
}

impl JavaInputStream for ByteArrayInputStream {
    fn read(&mut self) -> i32 {
        if self.pos < self.count {
            let b = self.buf[self.pos as usize];
            self.pos += 1;
            b as i32
        } else {
            -1
        }
    }

    fn read_into(&mut self, b: &mut [u8], off: i32, len: i32) -> i32 {
        let length = b.len() as i32;
        if (length | off | len) < 0 || len > length - off {
            throw(
                "IndexOutOfBoundsException",
                format!(
                    "Range [{}, {} + {}) out of bounds for length {}",
                    off, off, len, length
                ),
            );
        }
        if self.pos >= self.count {
            return -1;
        }
        let avail = self.count - self.pos;
        let len = if len > avail { avail } else { len };
        if len <= 0 {
            return 0;
        }
        let (p, o, n) = (self.pos as usize, off as usize, len as usize);
        b[o..o + n].copy_from_slice(&self.buf[p..p + n]);
        self.pos += len;
        len
    }
}

/// `AbstractMessageLite.Builder.LimitedInputStream`.
pub struct LimitedInputStream<'a> {
    input: &'a mut dyn JavaInputStream,
    limit: i32,
}

impl<'a> LimitedInputStream<'a> {
    pub fn new(input: &'a mut dyn JavaInputStream, limit: i32) -> LimitedInputStream<'a> {
        LimitedInputStream { input, limit }
    }
}

impl JavaInputStream for LimitedInputStream<'_> {
    fn read(&mut self) -> i32 {
        if self.limit <= 0 {
            return -1;
        }
        let result = self.input.read();
        if result >= 0 {
            self.limit -= 1;
        }
        result
    }

    fn read_into(&mut self, b: &mut [u8], off: i32, len: i32) -> i32 {
        if self.limit <= 0 {
            return -1;
        }
        let len = len.min(self.limit);
        let result = self.input.read_into(b, off, len);
        if result >= 0 {
            self.limit -= result;
        }
        result
    }
}

/// `WireFormat.getTagWireType`.
pub fn get_tag_wire_type(tag: i32) -> i32 {
    tag & 7
}

/// `WireFormat.getTagFieldNumber`.
pub fn get_tag_field_number(tag: i32) -> i32 {
    ((tag as u32) >> 3) as i32
}

/// `WireFormat.makeTag`.
pub fn make_tag(field_number: i32, wire_type: i32) -> i32 {
    field_number.wrapping_shl(3) | wire_type
}

const BUFFER_SIZE: i32 = 4096;

/// `CodedInputStream` over an `InputStream`, with its 4096-byte buffer.
pub struct CodedInputStream<'a> {
    buffer: Vec<u8>,
    buffer_size: i32,
    buffer_size_after_limit: i32,
    buffer_pos: i32,
    input: &'a mut dyn JavaInputStream,
    last_tag: i32,
    total_bytes_retired: i32,
    current_limit: i32,
    recursion_depth: i32,
    recursion_limit: i32,
    size_limit: i32,
}

impl<'a> CodedInputStream<'a> {
    pub fn new_instance(input: &'a mut dyn JavaInputStream) -> CodedInputStream<'a> {
        CodedInputStream {
            buffer: vec![0; BUFFER_SIZE as usize],
            buffer_size: 0,
            buffer_size_after_limit: 0,
            buffer_pos: 0,
            input,
            last_tag: 0,
            total_bytes_retired: 0,
            current_limit: i32::MAX,
            recursion_depth: 0,
            recursion_limit: 64,
            size_limit: 67108864,
        }
    }

    fn byte_at(&self, index: i32) -> i8 {
        if index < 0 || index as usize >= self.buffer.len() {
            array_index_out_of_bounds(index, self.buffer.len());
        }
        self.buffer[index as usize] as i8
    }

    pub fn check_recursion_limit(&self) {
        if self.recursion_depth >= self.recursion_limit {
            invalid_protocol_buffer();
        }
    }

    pub fn read_tag(&mut self) -> i32 {
        if self.is_at_end() {
            self.last_tag = 0;
            return 0;
        }
        self.last_tag = self.read_raw_varint32();
        if get_tag_field_number(self.last_tag) == 0 {
            invalid_protocol_buffer();
        }
        self.last_tag
    }

    pub fn check_last_tag_was(&self, value: i32) {
        if self.last_tag != value {
            invalid_protocol_buffer();
        }
    }

    /// Skips one field; false for an end-group tag.
    pub fn skip_field(&mut self, tag: i32) -> bool {
        match get_tag_wire_type(tag) {
            0 => {
                self.read_int64();
                true
            }
            1 => {
                self.read_raw_little_endian64();
                true
            }
            2 => {
                self.read_bytes();
                true
            }
            3 => {
                self.skip_message();
                let end_tag = make_tag(get_tag_field_number(tag), 4);
                self.check_last_tag_was(end_tag);
                true
            }
            4 => false,
            5 => {
                self.read_raw_little_endian32();
                true
            }
            _ => invalid_protocol_buffer(),
        }
    }

    pub fn skip_message(&mut self) {
        loop {
            let tag = self.read_tag();
            if tag == 0 {
                return;
            }
            self.check_recursion_limit();
            self.recursion_depth += 1;
            let done = !self.skip_field(tag);
            self.recursion_depth -= 1;
            if done {
                return;
            }
        }
    }

    pub fn read_double(&mut self) -> f64 {
        f64::from_bits(self.read_raw_little_endian64() as u64)
    }

    pub fn read_float(&mut self) -> f32 {
        f32::from_bits(self.read_raw_little_endian32() as u32)
    }

    pub fn read_int64(&mut self) -> i64 {
        self.read_raw_varint64()
    }

    pub fn read_int32(&mut self) -> i32 {
        self.read_raw_varint32()
    }

    pub fn read_bool(&mut self) -> bool {
        self.read_raw_varint64() != 0
    }

    pub fn read_bytes(&mut self) -> Rc<[u8]> {
        let size = self.read_raw_varint32();
        if size <= self.buffer_size.wrapping_sub(self.buffer_pos) && size > 0 {
            let mut copy = vec![0u8; size as usize];
            arraycopy(&self.buffer, self.buffer_pos, &mut copy, 0, size);
            self.buffer_pos = self.buffer_pos.wrapping_add(size);
            Rc::from(copy)
        } else if size == 0 {
            Rc::from(Vec::new())
        } else {
            Rc::from(self.read_raw_bytes_slow_path(size))
        }
    }

    pub fn read_enum(&mut self) -> i32 {
        self.read_raw_varint32()
    }

    pub fn read_sint64(&mut self) -> i64 {
        decode_zig_zag64(self.read_raw_varint64())
    }

    pub fn read_raw_varint32(&mut self) -> i32 {
        'fastpath: {
            let mut pos = self.buffer_pos;
            if self.buffer_size == pos {
                break 'fastpath;
            }
            let mut x = self.byte_at(pos) as i32;
            pos += 1;
            if x >= 0 {
                self.buffer_pos = pos;
                return x;
            }
            if self.buffer_size.wrapping_sub(pos) < 9 {
                break 'fastpath;
            }
            x ^= (self.byte_at(pos) as i32) << 7;
            pos += 1;
            if x < 0 {
                x ^= -128;
            } else {
                x ^= (self.byte_at(pos) as i32) << 14;
                pos += 1;
                if x >= 0 {
                    x ^= 16256;
                } else {
                    x ^= (self.byte_at(pos) as i32) << 21;
                    pos += 1;
                    if x < 0 {
                        x ^= -2080896;
                    } else {
                        let y = self.byte_at(pos) as i32;
                        pos += 1;
                        x ^= y << 28;
                        x ^= 266354560;
                        if y < 0 {
                            let mut terminated = false;
                            for _ in 0..5 {
                                let b = self.byte_at(pos);
                                pos += 1;
                                if b >= 0 {
                                    terminated = true;
                                    break;
                                }
                            }
                            if !terminated {
                                break 'fastpath;
                            }
                        }
                    }
                }
            }
            self.buffer_pos = pos;
            return x;
        }
        self.read_raw_varint64_slow_path() as i32
    }

    /// The static `readRawVarint32(int, InputStream)`.
    pub fn read_raw_varint32_from(first_byte: i32, input: &mut dyn JavaInputStream) -> i32 {
        if first_byte & 128 == 0 {
            return first_byte;
        }
        let mut result = first_byte & 127;
        let mut offset = 7;
        while offset < 32 {
            let b = input.read();
            if b == -1 {
                invalid_protocol_buffer();
            }
            result |= (b & 127) << offset;
            if b & 128 == 0 {
                return result;
            }
            offset += 7;
        }
        while offset < 64 {
            let b = input.read();
            if b == -1 {
                invalid_protocol_buffer();
            }
            if b & 128 == 0 {
                return result;
            }
            offset += 7;
        }
        invalid_protocol_buffer()
    }

    pub fn read_raw_varint64(&mut self) -> i64 {
        'fastpath: {
            let mut pos = self.buffer_pos;
            if self.buffer_size == pos {
                break 'fastpath;
            }
            let y = self.byte_at(pos) as i32;
            pos += 1;
            if y >= 0 {
                self.buffer_pos = pos;
                return y as i64;
            }
            if self.buffer_size.wrapping_sub(pos) < 9 {
                break 'fastpath;
            }
            let mut x = (y ^ ((self.byte_at(pos) as i32) << 7)) as i64;
            pos += 1;
            if x < 0 {
                x ^= -128;
            } else {
                x ^= ((self.byte_at(pos) as i32) << 14) as i64;
                pos += 1;
                if x >= 0 {
                    x ^= 16256;
                } else {
                    x ^= ((self.byte_at(pos) as i32) << 21) as i64;
                    pos += 1;
                    if x < 0 {
                        x ^= -2080896;
                    } else {
                        x ^= (self.byte_at(pos) as i64) << 28;
                        pos += 1;
                        if x >= 0 {
                            x ^= 266354560;
                        } else {
                            x ^= (self.byte_at(pos) as i64) << 35;
                            pos += 1;
                            if x < 0 {
                                x ^= -34093383808;
                            } else {
                                x ^= (self.byte_at(pos) as i64) << 42;
                                pos += 1;
                                if x >= 0 {
                                    x ^= 4363953127296;
                                } else {
                                    x ^= (self.byte_at(pos) as i64) << 49;
                                    pos += 1;
                                    if x < 0 {
                                        x ^= -558586000294016;
                                    } else {
                                        x ^= (self.byte_at(pos) as i64) << 56;
                                        pos += 1;
                                        x ^= 71499008037633920;
                                        if x < 0 {
                                            let b = self.byte_at(pos);
                                            pos += 1;
                                            if b < 0 {
                                                break 'fastpath;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            self.buffer_pos = pos;
            return x;
        }
        self.read_raw_varint64_slow_path()
    }

    fn read_raw_varint64_slow_path(&mut self) -> i64 {
        let mut result: i64 = 0;
        let mut shift = 0;
        while shift < 64 {
            let b = self.read_raw_byte();
            result |= ((b as i64) & 127) << shift;
            if (b as i32) & 128 == 0 {
                return result;
            }
            shift += 7;
        }
        invalid_protocol_buffer()
    }

    pub fn read_raw_little_endian32(&mut self) -> i32 {
        let mut pos = self.buffer_pos;
        if self.buffer_size.wrapping_sub(pos) < 4 {
            self.refill_buffer(4);
            pos = self.buffer_pos;
        }
        self.buffer_pos = pos.wrapping_add(4);
        let b = |i: i32| self.byte_at(pos.wrapping_add(i)) as u8 as i32;
        b(0) | (b(1) << 8) | (b(2) << 16) | (b(3) << 24)
    }

    pub fn read_raw_little_endian64(&mut self) -> i64 {
        let mut pos = self.buffer_pos;
        if self.buffer_size.wrapping_sub(pos) < 8 {
            self.refill_buffer(8);
            pos = self.buffer_pos;
        }
        self.buffer_pos = pos.wrapping_add(8);
        let mut result: i64 = 0;
        for i in 0..8 {
            result |= (self.byte_at(pos.wrapping_add(i)) as u8 as i64) << (8 * i);
        }
        result
    }

    pub fn push_limit(&mut self, byte_limit: i32) -> i32 {
        if byte_limit < 0 {
            invalid_protocol_buffer();
        }
        let byte_limit =
            byte_limit.wrapping_add(self.total_bytes_retired.wrapping_add(self.buffer_pos));
        let old_limit = self.current_limit;
        if byte_limit > old_limit {
            invalid_protocol_buffer();
        }
        self.current_limit = byte_limit;
        self.recompute_buffer_size_after_limit();
        old_limit
    }

    fn recompute_buffer_size_after_limit(&mut self) {
        self.buffer_size = self.buffer_size.wrapping_add(self.buffer_size_after_limit);
        let buffer_end = self.total_bytes_retired.wrapping_add(self.buffer_size);
        if buffer_end > self.current_limit {
            self.buffer_size_after_limit = buffer_end.wrapping_sub(self.current_limit);
            self.buffer_size = self.buffer_size.wrapping_sub(self.buffer_size_after_limit);
        } else {
            self.buffer_size_after_limit = 0;
        }
    }

    pub fn pop_limit(&mut self, old_limit: i32) {
        self.current_limit = old_limit;
        self.recompute_buffer_size_after_limit();
    }

    pub fn get_bytes_until_limit(&self) -> i32 {
        if self.current_limit == i32::MAX {
            return -1;
        }
        let current_absolute_position = self.total_bytes_retired.wrapping_add(self.buffer_pos);
        self.current_limit.wrapping_sub(current_absolute_position)
    }

    pub fn is_at_end(&mut self) -> bool {
        self.buffer_pos == self.buffer_size && !self.try_refill_buffer(1)
    }

    fn ensure_available(&mut self, n: i32) {
        if self.buffer_size.wrapping_sub(self.buffer_pos) < n {
            self.refill_buffer(n);
        }
    }

    fn refill_buffer(&mut self, n: i32) {
        if !self.try_refill_buffer(n) {
            invalid_protocol_buffer();
        }
    }

    fn try_refill_buffer(&mut self, n: i32) -> bool {
        loop {
            if self.buffer_pos.wrapping_add(n) <= self.buffer_size {
                throw("IllegalStateException", format!("Code{}Code", n));
            }
            if self
                .total_bytes_retired
                .wrapping_add(self.buffer_pos)
                .wrapping_add(n)
                > self.current_limit
            {
                return false;
            }
            let pos = self.buffer_pos;
            if pos > 0 {
                if self.buffer_size > pos {
                    let len = self.buffer_size.wrapping_sub(pos);
                    check_arraycopy(self.buffer.len(), pos, self.buffer.len(), 0, len);
                    self.buffer
                        .copy_within(pos as usize..(pos + len) as usize, 0);
                }
                self.total_bytes_retired = self.total_bytes_retired.wrapping_add(pos);
                self.buffer_size = self.buffer_size.wrapping_sub(pos);
                self.buffer_pos = 0;
            }
            let bytes_read = self.input.read_into(
                &mut self.buffer,
                self.buffer_size,
                BUFFER_SIZE.wrapping_sub(self.buffer_size),
            );
            if bytes_read == 0 || !(-1..=BUFFER_SIZE).contains(&bytes_read) {
                throw("IllegalStateException", format!("Code{}Code", bytes_read));
            }
            if bytes_read > 0 {
                self.buffer_size = self.buffer_size.wrapping_add(bytes_read);
                if self
                    .total_bytes_retired
                    .wrapping_add(n)
                    .wrapping_sub(self.size_limit)
                    > 0
                {
                    invalid_protocol_buffer();
                }
                self.recompute_buffer_size_after_limit();
                if self.buffer_size >= n {
                    return true;
                }
                continue;
            }
            return false;
        }
    }

    pub fn read_raw_byte(&mut self) -> i8 {
        if self.buffer_pos == self.buffer_size {
            self.refill_buffer(1);
        }
        let pos = self.buffer_pos;
        self.buffer_pos = pos.wrapping_add(1);
        self.byte_at(pos)
    }

    fn read_raw_bytes_slow_path(&mut self, size: i32) -> Vec<u8> {
        if size <= 0 {
            if size == 0 {
                return Vec::new();
            }
            invalid_protocol_buffer();
        }
        if self
            .total_bytes_retired
            .wrapping_add(self.buffer_pos)
            .wrapping_add(size)
            > self.current_limit
        {
            let n = self
                .current_limit
                .wrapping_sub(self.total_bytes_retired)
                .wrapping_sub(self.buffer_pos);
            self.skip_raw_bytes(n);
            invalid_protocol_buffer();
        }
        if size < BUFFER_SIZE {
            let mut bytes = vec![0u8; size as usize];
            let pos = self.buffer_pos;
            let buffered = self.buffer_size.wrapping_sub(pos);
            arraycopy(&self.buffer, pos, &mut bytes, 0, buffered);
            self.buffer_pos = self.buffer_size;
            self.ensure_available(size.wrapping_sub(buffered));
            arraycopy(
                &self.buffer,
                0,
                &mut bytes,
                buffered,
                size.wrapping_sub(buffered),
            );
            self.buffer_pos = size.wrapping_sub(buffered);
            return bytes;
        }
        let original_buffer_pos = self.buffer_pos;
        let original_buffer_size = self.buffer_size;
        self.total_bytes_retired = self.total_bytes_retired.wrapping_add(self.buffer_size);
        self.buffer_pos = 0;
        self.buffer_size = 0;
        let mut size_left =
            size.wrapping_sub(original_buffer_size.wrapping_sub(original_buffer_pos));
        let mut chunks: Vec<Vec<u8>> = Vec::new();
        while size_left > 0 {
            let mut chunk = vec![0u8; size_left.min(BUFFER_SIZE) as usize];
            let chunk_len = chunk.len() as i32;
            let mut pos = 0;
            while pos < chunk_len {
                let n = self.input.read_into(&mut chunk, pos, chunk_len - pos);
                if n == -1 {
                    invalid_protocol_buffer();
                }
                self.total_bytes_retired = self.total_bytes_retired.wrapping_add(n);
                pos += n;
            }
            size_left -= chunk_len;
            chunks.push(chunk);
        }
        let mut bytes = vec![0u8; size as usize];
        let mut pos = original_buffer_size.wrapping_sub(original_buffer_pos);
        arraycopy(&self.buffer, original_buffer_pos, &mut bytes, 0, pos);
        for chunk in chunks {
            arraycopy(&chunk, 0, &mut bytes, pos, chunk.len() as i32);
            pos += chunk.len() as i32;
        }
        bytes
    }

    pub fn skip_raw_bytes(&mut self, size: i32) {
        if size <= self.buffer_size.wrapping_sub(self.buffer_pos) && size >= 0 {
            self.buffer_pos = self.buffer_pos.wrapping_add(size);
        } else {
            self.skip_raw_bytes_slow_path(size);
        }
    }

    fn skip_raw_bytes_slow_path(&mut self, size: i32) {
        if size < 0 {
            invalid_protocol_buffer();
        }
        if self
            .total_bytes_retired
            .wrapping_add(self.buffer_pos)
            .wrapping_add(size)
            > self.current_limit
        {
            let n = self
                .current_limit
                .wrapping_sub(self.total_bytes_retired)
                .wrapping_sub(self.buffer_pos);
            self.skip_raw_bytes(n);
            invalid_protocol_buffer();
        }
        let mut pos = self.buffer_size.wrapping_sub(self.buffer_pos);
        self.buffer_pos = self.buffer_size;
        self.refill_buffer(1);
        while size.wrapping_sub(pos) > self.buffer_size {
            pos = pos.wrapping_add(self.buffer_size);
            self.buffer_pos = self.buffer_size;
            self.refill_buffer(1);
        }
        self.buffer_pos = size.wrapping_sub(pos);
    }
}

pub fn decode_zig_zag64(n: i64) -> i64 {
    (((n as u64) >> 1) as i64) ^ -(n & 1)
}

/// Every message type reachable from the parsed roots.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum MsgType {
    Class,
    Package,
    Function,
    Property,
    Constructor,
    ValueParameter,
    Type,
    TypeArgument,
    TypeParameter,
    TypeTable,
    TypeAlias,
    EnumEntry,
    Annotation,
    AnnotationArgument,
    AnnotationArgumentValue,
    Contract,
    Effect,
    Expression,
    VersionRequirementTable,
    VersionRequirement,
    CompilerPluginData,
    JvmMethodSignature,
    JvmFieldSignature,
    JvmPropertySignature,
    StringTableTypes,
    Record,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FieldKind {
    Int32,
    SInt64,
    Bool,
    Float,
    Double,
    Bytes,
    /// An enum with values `0..n` (every enum here numbers its constants by ordinal).
    Enum(i32),
    Msg(MsgType),
}

use FieldKind::{Bool, Bytes, Double, Enum, Float, Int32, Msg, SInt64};

impl FieldKind {
    fn wire_type(self) -> i32 {
        match self {
            Int32 | SInt64 | Bool | Enum(_) => 0,
            Float => 5,
            Double => 1,
            Bytes | Msg(_) => 2,
        }
    }

    fn is_packable(self) -> bool {
        !matches!(self, Bytes | Msg(_))
    }
}

/// One field of a generated message (or a registered `JvmProtoBuf` extension).
#[derive(Debug)]
pub struct FieldSpec {
    pub number: i32,
    kind: FieldKind,
    pub repeated: bool,
    pub required: bool,
    pub extension: bool,
    pub name: &'static str,
}

const fn fs(
    number: i32,
    kind: FieldKind,
    repeated: bool,
    required: bool,
    extension: bool,
    name: &'static str,
) -> FieldSpec {
    FieldSpec {
        number,
        kind,
        repeated,
        required,
        extension,
        name,
    }
}

#[rustfmt::skip]
const CLASS_FIELDS: &[FieldSpec] = &[
    fs(1, Int32, false, false, false, "flags"),
    fs(2, Int32, true, false, false, "supertype_id"),
    fs(3, Int32, false, true, false, "fq_name"),
    fs(4, Int32, false, false, false, "companion_object_name"),
    fs(5, Msg(MsgType::TypeParameter), true, false, false, "type_parameter"),
    fs(6, Msg(MsgType::Type), true, false, false, "supertype"),
    fs(7, Int32, true, false, false, "nested_class_name"),
    fs(8, Msg(MsgType::Constructor), true, false, false, "constructor"),
    fs(9, Msg(MsgType::Function), true, false, false, "function"),
    fs(10, Msg(MsgType::Property), true, false, false, "property"),
    fs(11, Msg(MsgType::TypeAlias), true, false, false, "type_alias"),
    fs(13, Msg(MsgType::EnumEntry), true, false, false, "enum_entry"),
    fs(16, Int32, true, false, false, "sealed_subclass_fq_name"),
    fs(17, Int32, false, false, false, "inline_class_underlying_property_name"),
    fs(18, Msg(MsgType::Type), false, false, false, "inline_class_underlying_type"),
    fs(19, Int32, false, false, false, "inline_class_underlying_type_id"),
    fs(20, Msg(MsgType::Type), true, false, false, "context_receiver_type"),
    fs(21, Int32, true, false, false, "context_receiver_type_id"),
    fs(25, Msg(MsgType::Annotation), true, false, false, "annotation"),
    fs(30, Msg(MsgType::TypeTable), false, false, false, "type_table"),
    fs(31, Int32, true, false, false, "version_requirement"),
    fs(32, Msg(MsgType::VersionRequirementTable), false, false, false, "version_requirement_table"),
    fs(33, Msg(MsgType::CompilerPluginData), true, false, false, "compiler_plugin_data"),
    fs(101, Int32, false, false, true, "class_module_name"),
    fs(102, Msg(MsgType::Property), true, false, true, "class_local_variable"),
    fs(103, Int32, false, false, true, "anonymous_object_origin_name"),
    fs(104, Int32, false, false, true, "jvm_class_flags"),
];

#[rustfmt::skip]
const PACKAGE_FIELDS: &[FieldSpec] = &[
    fs(3, Msg(MsgType::Function), true, false, false, "function"),
    fs(4, Msg(MsgType::Property), true, false, false, "property"),
    fs(5, Msg(MsgType::TypeAlias), true, false, false, "type_alias"),
    fs(30, Msg(MsgType::TypeTable), false, false, false, "type_table"),
    fs(32, Msg(MsgType::VersionRequirementTable), false, false, false, "version_requirement_table"),
    fs(101, Int32, false, false, true, "package_module_name"),
    fs(102, Msg(MsgType::Property), true, false, true, "package_local_variable"),
];

#[rustfmt::skip]
const FUNCTION_FIELDS: &[FieldSpec] = &[
    fs(1, Int32, false, false, false, "old_flags"),
    fs(2, Int32, false, true, false, "name"),
    fs(3, Msg(MsgType::Type), false, false, false, "return_type"),
    fs(4, Msg(MsgType::TypeParameter), true, false, false, "type_parameter"),
    fs(5, Msg(MsgType::Type), false, false, false, "receiver_type"),
    fs(6, Msg(MsgType::ValueParameter), true, false, false, "value_parameter"),
    fs(7, Int32, false, false, false, "return_type_id"),
    fs(8, Int32, false, false, false, "receiver_type_id"),
    fs(9, Int32, false, false, false, "flags"),
    fs(10, Msg(MsgType::Type), true, false, false, "context_receiver_type"),
    fs(11, Int32, true, false, false, "context_receiver_type_id"),
    fs(12, Msg(MsgType::Annotation), true, false, false, "annotation"),
    fs(13, Msg(MsgType::ValueParameter), true, false, false, "context_parameter"),
    fs(30, Msg(MsgType::TypeTable), false, false, false, "type_table"),
    fs(31, Int32, true, false, false, "version_requirement"),
    fs(32, Msg(MsgType::Contract), false, false, false, "contract"),
    fs(33, Msg(MsgType::CompilerPluginData), true, false, false, "compiler_plugin_data"),
    fs(34, Msg(MsgType::Annotation), true, false, false, "extension_receiver_annotation"),
    fs(100, Msg(MsgType::JvmMethodSignature), false, false, true, "method_signature"),
    fs(101, Int32, false, false, true, "lambda_class_origin_name"),
];

#[rustfmt::skip]
const PROPERTY_FIELDS: &[FieldSpec] = &[
    fs(1, Int32, false, false, false, "old_flags"),
    fs(2, Int32, false, true, false, "name"),
    fs(3, Msg(MsgType::Type), false, false, false, "return_type"),
    fs(4, Msg(MsgType::TypeParameter), true, false, false, "type_parameter"),
    fs(5, Msg(MsgType::Type), false, false, false, "receiver_type"),
    fs(6, Msg(MsgType::ValueParameter), false, false, false, "setter_value_parameter"),
    fs(7, Int32, false, false, false, "getter_flags"),
    fs(8, Int32, false, false, false, "setter_flags"),
    fs(9, Int32, false, false, false, "return_type_id"),
    fs(10, Int32, false, false, false, "receiver_type_id"),
    fs(11, Int32, false, false, false, "flags"),
    fs(12, Msg(MsgType::Type), true, false, false, "context_receiver_type"),
    fs(13, Int32, true, false, false, "context_receiver_type_id"),
    fs(14, Msg(MsgType::Annotation), true, false, false, "annotation"),
    fs(15, Msg(MsgType::Annotation), true, false, false, "getter_annotation"),
    fs(16, Msg(MsgType::Annotation), true, false, false, "setter_annotation"),
    fs(17, Msg(MsgType::ValueParameter), true, false, false, "context_parameter"),
    fs(31, Int32, true, false, false, "version_requirement"),
    fs(32, Msg(MsgType::CompilerPluginData), true, false, false, "compiler_plugin_data"),
    fs(33, Msg(MsgType::Annotation), true, false, false, "extension_receiver_annotation"),
    fs(34, Msg(MsgType::Annotation), true, false, false, "backing_field_annotation"),
    fs(35, Msg(MsgType::Annotation), true, false, false, "delegate_field_annotation"),
    fs(40, Msg(MsgType::Contract), false, false, false, "getter_contract"),
    fs(41, Msg(MsgType::Contract), false, false, false, "setter_contract"),
    fs(100, Msg(MsgType::JvmPropertySignature), false, false, true, "property_signature"),
    fs(101, Int32, false, false, true, "jvm_flags"),
];

#[rustfmt::skip]
const CONSTRUCTOR_FIELDS: &[FieldSpec] = &[
    fs(1, Int32, false, false, false, "flags"),
    fs(2, Msg(MsgType::ValueParameter), true, false, false, "value_parameter"),
    fs(3, Msg(MsgType::Annotation), true, false, false, "annotation"),
    fs(31, Int32, true, false, false, "version_requirement"),
    fs(32, Msg(MsgType::CompilerPluginData), true, false, false, "compiler_plugin_data"),
    fs(100, Msg(MsgType::JvmMethodSignature), false, false, true, "constructor_signature"),
];

#[rustfmt::skip]
const VALUE_PARAMETER_FIELDS: &[FieldSpec] = &[
    fs(1, Int32, false, false, false, "flags"),
    fs(2, Int32, false, true, false, "name"),
    fs(3, Msg(MsgType::Type), false, false, false, "type"),
    fs(4, Msg(MsgType::Type), false, false, false, "vararg_element_type"),
    fs(5, Int32, false, false, false, "type_id"),
    fs(6, Int32, false, false, false, "vararg_element_type_id"),
    fs(7, Msg(MsgType::Annotation), true, false, false, "annotation"),
    fs(8, Msg(MsgType::AnnotationArgumentValue), false, false, false, "annotation_parameter_default_value"),
];

#[rustfmt::skip]
const TYPE_FIELDS: &[FieldSpec] = &[
    fs(1, Int32, false, false, false, "flags"),
    fs(2, Msg(MsgType::TypeArgument), true, false, false, "argument"),
    fs(3, Bool, false, false, false, "nullable"),
    fs(4, Int32, false, false, false, "flexible_type_capabilities_id"),
    fs(5, Msg(MsgType::Type), false, false, false, "flexible_upper_bound"),
    fs(6, Int32, false, false, false, "class_name"),
    fs(7, Int32, false, false, false, "type_parameter"),
    fs(8, Int32, false, false, false, "flexible_upper_bound_id"),
    fs(9, Int32, false, false, false, "type_parameter_name"),
    fs(10, Msg(MsgType::Type), false, false, false, "outer_type"),
    fs(11, Int32, false, false, false, "outer_type_id"),
    fs(12, Int32, false, false, false, "type_alias_name"),
    fs(13, Msg(MsgType::Type), false, false, false, "abbreviated_type"),
    fs(14, Int32, false, false, false, "abbreviated_type_id"),
    fs(100, Msg(MsgType::Annotation), true, false, false, "annotation"),
    fs(101, Bool, false, false, true, "is_raw"),
];

#[rustfmt::skip]
const TYPE_ARGUMENT_FIELDS: &[FieldSpec] = &[
    fs(1, Enum(4), false, false, false, "projection"),
    fs(2, Msg(MsgType::Type), false, false, false, "type"),
    fs(3, Int32, false, false, false, "type_id"),
];

#[rustfmt::skip]
const TYPE_PARAMETER_FIELDS: &[FieldSpec] = &[
    fs(1, Int32, false, true, false, "id"),
    fs(2, Int32, false, true, false, "name"),
    fs(3, Bool, false, false, false, "reified"),
    fs(4, Enum(3), false, false, false, "variance"),
    fs(5, Msg(MsgType::Type), true, false, false, "upper_bound"),
    fs(6, Int32, true, false, false, "upper_bound_id"),
    fs(100, Msg(MsgType::Annotation), true, false, false, "annotation"),
];

#[rustfmt::skip]
const TYPE_TABLE_FIELDS: &[FieldSpec] = &[
    fs(1, Msg(MsgType::Type), true, false, false, "type"),
    fs(2, Int32, false, false, false, "first_nullable"),
];

#[rustfmt::skip]
const TYPE_ALIAS_FIELDS: &[FieldSpec] = &[
    fs(1, Int32, false, false, false, "flags"),
    fs(2, Int32, false, true, false, "name"),
    fs(3, Msg(MsgType::TypeParameter), true, false, false, "type_parameter"),
    fs(4, Msg(MsgType::Type), false, false, false, "underlying_type"),
    fs(5, Int32, false, false, false, "underlying_type_id"),
    fs(6, Msg(MsgType::Type), false, false, false, "expanded_type"),
    fs(7, Int32, false, false, false, "expanded_type_id"),
    fs(8, Msg(MsgType::Annotation), true, false, false, "annotation"),
    fs(31, Int32, true, false, false, "version_requirement"),
    fs(32, Msg(MsgType::CompilerPluginData), true, false, false, "compiler_plugin_data"),
];

#[rustfmt::skip]
const ENUM_ENTRY_FIELDS: &[FieldSpec] = &[
    fs(1, Int32, false, false, false, "name"),
    fs(2, Msg(MsgType::Annotation), true, false, false, "annotation"),
];

#[rustfmt::skip]
const ANNOTATION_FIELDS: &[FieldSpec] = &[
    fs(1, Int32, false, true, false, "id"),
    fs(2, Msg(MsgType::AnnotationArgument), true, false, false, "argument"),
];

#[rustfmt::skip]
const ANNOTATION_ARGUMENT_FIELDS: &[FieldSpec] = &[
    fs(1, Int32, false, true, false, "name_id"),
    fs(2, Msg(MsgType::AnnotationArgumentValue), false, true, false, "value"),
];

#[rustfmt::skip]
const ANNOTATION_ARGUMENT_VALUE_FIELDS: &[FieldSpec] = &[
    fs(1, Enum(13), false, false, false, "type"),
    fs(2, SInt64, false, false, false, "int_value"),
    fs(3, Float, false, false, false, "float_value"),
    fs(4, Double, false, false, false, "double_value"),
    fs(5, Int32, false, false, false, "string_value"),
    fs(6, Int32, false, false, false, "class_id"),
    fs(7, Int32, false, false, false, "enum_value_id"),
    fs(8, Msg(MsgType::Annotation), false, false, false, "annotation"),
    fs(9, Msg(MsgType::AnnotationArgumentValue), true, false, false, "array_element"),
    fs(10, Int32, false, false, false, "flags"),
    fs(11, Int32, false, false, false, "array_dimension_count"),
];

#[rustfmt::skip]
const CONTRACT_FIELDS: &[FieldSpec] = &[
    fs(1, Msg(MsgType::Effect), true, false, false, "effect"),
];

#[rustfmt::skip]
const EFFECT_FIELDS: &[FieldSpec] = &[
    fs(1, Enum(3), false, false, false, "effect_type"),
    fs(2, Msg(MsgType::Expression), true, false, false, "effect_constructor_argument"),
    fs(3, Msg(MsgType::Expression), false, false, false, "conclusion_of_conditional_effect"),
    fs(4, Enum(3), false, false, false, "kind"),
    fs(5, Enum(3), false, false, false, "condition_kind"),
];

#[rustfmt::skip]
const EXPRESSION_FIELDS: &[FieldSpec] = &[
    fs(1, Int32, false, false, false, "flags"),
    fs(2, Int32, false, false, false, "value_parameter_reference"),
    fs(3, Enum(3), false, false, false, "constant_value"),
    fs(4, Msg(MsgType::Type), false, false, false, "is_instance_type"),
    fs(5, Int32, false, false, false, "is_instance_type_id"),
    fs(6, Msg(MsgType::Expression), true, false, false, "and_argument"),
    fs(7, Msg(MsgType::Expression), true, false, false, "or_argument"),
];

#[rustfmt::skip]
const VERSION_REQUIREMENT_TABLE_FIELDS: &[FieldSpec] = &[
    fs(1, Msg(MsgType::VersionRequirement), true, false, false, "requirement"),
];

#[rustfmt::skip]
const VERSION_REQUIREMENT_FIELDS: &[FieldSpec] = &[
    fs(1, Int32, false, false, false, "version"),
    fs(2, Int32, false, false, false, "version_full"),
    fs(3, Enum(3), false, false, false, "level"),
    fs(4, Int32, false, false, false, "error_code"),
    fs(5, Int32, false, false, false, "message"),
    fs(6, Enum(3), false, false, false, "version_kind"),
];

#[rustfmt::skip]
const COMPILER_PLUGIN_DATA_FIELDS: &[FieldSpec] = &[
    fs(1, Int32, false, true, false, "plugin_id"),
    fs(2, Bytes, false, true, false, "data"),
];

#[rustfmt::skip]
const JVM_METHOD_SIGNATURE_FIELDS: &[FieldSpec] = &[
    fs(1, Int32, false, false, false, "name"),
    fs(2, Int32, false, false, false, "desc"),
];

#[rustfmt::skip]
const JVM_FIELD_SIGNATURE_FIELDS: &[FieldSpec] = &[
    fs(1, Int32, false, false, false, "name"),
    fs(2, Int32, false, false, false, "desc"),
];

#[rustfmt::skip]
const JVM_PROPERTY_SIGNATURE_FIELDS: &[FieldSpec] = &[
    fs(1, Msg(MsgType::JvmFieldSignature), false, false, false, "field"),
    fs(2, Msg(MsgType::JvmMethodSignature), false, false, false, "synthetic_method"),
    fs(3, Msg(MsgType::JvmMethodSignature), false, false, false, "getter"),
    fs(4, Msg(MsgType::JvmMethodSignature), false, false, false, "setter"),
    fs(5, Msg(MsgType::JvmMethodSignature), false, false, false, "delegate_method"),
];

#[rustfmt::skip]
const STRING_TABLE_TYPES_FIELDS: &[FieldSpec] = &[
    fs(1, Msg(MsgType::Record), true, false, false, "record"),
    fs(5, Int32, true, false, false, "local_name"),
];

#[rustfmt::skip]
const RECORD_FIELDS: &[FieldSpec] = &[
    fs(1, Int32, false, false, false, "range"),
    fs(2, Int32, false, false, false, "predefined_index"),
    fs(3, Enum(3), false, false, false, "operation"),
    fs(4, Int32, true, false, false, "substring_index"),
    fs(5, Int32, true, false, false, "replace_char"),
    fs(6, Bytes, false, false, false, "string"),
];

impl MsgType {
    /// The Java binary name of the generated class.
    pub fn java_name(self) -> &'static str {
        match self {
            MsgType::Class => "org.vineflower.kt.metadata.ProtoBuf$Class",
            MsgType::Package => "org.vineflower.kt.metadata.ProtoBuf$Package",
            MsgType::Function => "org.vineflower.kt.metadata.ProtoBuf$Function",
            MsgType::Property => "org.vineflower.kt.metadata.ProtoBuf$Property",
            MsgType::Constructor => "org.vineflower.kt.metadata.ProtoBuf$Constructor",
            MsgType::ValueParameter => "org.vineflower.kt.metadata.ProtoBuf$ValueParameter",
            MsgType::Type => "org.vineflower.kt.metadata.ProtoBuf$Type",
            MsgType::TypeArgument => "org.vineflower.kt.metadata.ProtoBuf$Type$Argument",
            MsgType::TypeParameter => "org.vineflower.kt.metadata.ProtoBuf$TypeParameter",
            MsgType::TypeTable => "org.vineflower.kt.metadata.ProtoBuf$TypeTable",
            MsgType::TypeAlias => "org.vineflower.kt.metadata.ProtoBuf$TypeAlias",
            MsgType::EnumEntry => "org.vineflower.kt.metadata.ProtoBuf$EnumEntry",
            MsgType::Annotation => "org.vineflower.kt.metadata.ProtoBuf$Annotation",
            MsgType::AnnotationArgument => {
                "org.vineflower.kt.metadata.ProtoBuf$Annotation$Argument"
            }
            MsgType::AnnotationArgumentValue => {
                "org.vineflower.kt.metadata.ProtoBuf$Annotation$Argument$Value"
            }
            MsgType::Contract => "org.vineflower.kt.metadata.ProtoBuf$Contract",
            MsgType::Effect => "org.vineflower.kt.metadata.ProtoBuf$Effect",
            MsgType::Expression => "org.vineflower.kt.metadata.ProtoBuf$Expression",
            MsgType::VersionRequirementTable => {
                "org.vineflower.kt.metadata.ProtoBuf$VersionRequirementTable"
            }
            MsgType::VersionRequirement => "org.vineflower.kt.metadata.ProtoBuf$VersionRequirement",
            MsgType::CompilerPluginData => "org.vineflower.kt.metadata.ProtoBuf$CompilerPluginData",
            MsgType::JvmMethodSignature => {
                "org.vineflower.kt.metadata.jvm.JvmProtoBuf$JvmMethodSignature"
            }
            MsgType::JvmFieldSignature => {
                "org.vineflower.kt.metadata.jvm.JvmProtoBuf$JvmFieldSignature"
            }
            MsgType::JvmPropertySignature => {
                "org.vineflower.kt.metadata.jvm.JvmProtoBuf$JvmPropertySignature"
            }
            MsgType::StringTableTypes => {
                "org.vineflower.kt.metadata.jvm.JvmProtoBuf$StringTableTypes"
            }
            MsgType::Record => "org.vineflower.kt.metadata.jvm.JvmProtoBuf$StringTableTypes$Record",
        }
    }

    /// Whether the shaded jar kept the message's `Builder` (with `toBuilder()`
    /// and `newBuilder(X)`); merging into one of the others is a `NoSuchMethodError`.
    pub fn has_builder(self) -> bool {
        !matches!(
            self,
            MsgType::Contract
                | MsgType::Effect
                | MsgType::Expression
                | MsgType::VersionRequirementTable
                | MsgType::VersionRequirement
                | MsgType::CompilerPluginData
                | MsgType::JvmFieldSignature
                | MsgType::StringTableTypes
                | MsgType::Record
        )
    }

    /// `x.toBuilder()` on a message whose builder the shading stripped.
    fn to_builder(self) {
        if !self.has_builder() {
            let n = self.java_name();
            throw(
                "NoSuchMethodError",
                format!("'{}$Builder {}.toBuilder()'", n, n),
            );
        }
    }

    /// `X.newBuilder(x)` on a message whose builder the shading stripped.
    fn new_builder(self) {
        if !self.has_builder() {
            let n = self.java_name();
            throw(
                "NoSuchMethodError",
                format!("'{}$Builder {}.newBuilder({})'", n, n, n),
            );
        }
    }

    /// The message's fields and registered extensions, in field-number order
    /// (a number is never both a field and an extension of one message).
    pub fn fields(self) -> &'static [FieldSpec] {
        match self {
            MsgType::Class => CLASS_FIELDS,
            MsgType::Package => PACKAGE_FIELDS,
            MsgType::Function => FUNCTION_FIELDS,
            MsgType::Property => PROPERTY_FIELDS,
            MsgType::Constructor => CONSTRUCTOR_FIELDS,
            MsgType::ValueParameter => VALUE_PARAMETER_FIELDS,
            MsgType::Type => TYPE_FIELDS,
            MsgType::TypeArgument => TYPE_ARGUMENT_FIELDS,
            MsgType::TypeParameter => TYPE_PARAMETER_FIELDS,
            MsgType::TypeTable => TYPE_TABLE_FIELDS,
            MsgType::TypeAlias => TYPE_ALIAS_FIELDS,
            MsgType::EnumEntry => ENUM_ENTRY_FIELDS,
            MsgType::Annotation => ANNOTATION_FIELDS,
            MsgType::AnnotationArgument => ANNOTATION_ARGUMENT_FIELDS,
            MsgType::AnnotationArgumentValue => ANNOTATION_ARGUMENT_VALUE_FIELDS,
            MsgType::Contract => CONTRACT_FIELDS,
            MsgType::Effect => EFFECT_FIELDS,
            MsgType::Expression => EXPRESSION_FIELDS,
            MsgType::VersionRequirementTable => VERSION_REQUIREMENT_TABLE_FIELDS,
            MsgType::VersionRequirement => VERSION_REQUIREMENT_FIELDS,
            MsgType::CompilerPluginData => COMPILER_PLUGIN_DATA_FIELDS,
            MsgType::JvmMethodSignature => JVM_METHOD_SIGNATURE_FIELDS,
            MsgType::JvmFieldSignature => JVM_FIELD_SIGNATURE_FIELDS,
            MsgType::JvmPropertySignature => JVM_PROPERTY_SIGNATURE_FIELDS,
            MsgType::StringTableTypes => STRING_TABLE_TYPES_FIELDS,
            MsgType::Record => RECORD_FIELDS,
        }
    }
}

#[derive(Clone, Debug)]
enum FieldValue {
    /// Int32, SInt64, Bool (0/1), Enum (number), Float/Double (raw bits).
    Scalar(i64),
    Bytes(Rc<[u8]>),
    Message(Rc<Message>),
    Scalars(Vec<i64>),
    Messages(Vec<Rc<Message>>),
}

/// A parsed message of any type: one slot per field of its schema.
#[derive(Clone, Debug)]
pub struct Message {
    message_type: MsgType,
    values: Vec<Option<FieldValue>>,
}

impl Message {
    pub fn new(message_type: MsgType) -> Message {
        Message {
            message_type,
            values: vec![None; message_type.fields().len()],
        }
    }

    pub fn get_message_type(&self) -> MsgType {
        self.message_type
    }

    fn slot(&self, number: i32) -> usize {
        self.message_type
            .fields()
            .iter()
            .position(|f| f.number == number)
            .expect("field number in schema")
    }

    /// `hasX()` for a singular field.
    pub fn has(&self, number: i32) -> bool {
        self.values[self.slot(number)].is_some()
    }

    pub fn scalar(&self, number: i32) -> Option<i64> {
        match &self.values[self.slot(number)] {
            Some(FieldValue::Scalar(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn bytes(&self, number: i32) -> Option<Rc<[u8]>> {
        match &self.values[self.slot(number)] {
            Some(FieldValue::Bytes(v)) => Some(v.clone()),
            _ => None,
        }
    }

    pub fn message(&self, number: i32) -> Option<&Rc<Message>> {
        match &self.values[self.slot(number)] {
            Some(FieldValue::Message(v)) => Some(v),
            _ => None,
        }
    }

    pub fn scalars(&self, number: i32) -> &[i64] {
        match &self.values[self.slot(number)] {
            Some(FieldValue::Scalars(v)) => v,
            _ => &[],
        }
    }

    pub fn messages(&self, number: i32) -> &[Rc<Message>] {
        match &self.values[self.slot(number)] {
            Some(FieldValue::Messages(v)) => v,
            _ => &[],
        }
    }

    /// `isInitialized()`: required fields set, recursively (extensions included).
    pub fn is_initialized(&self) -> bool {
        for (spec, value) in self.message_type.fields().iter().zip(&self.values) {
            if spec.required && value.is_none() {
                return false;
            }
            match value {
                Some(FieldValue::Message(m)) if !m.is_initialized() => return false,
                Some(FieldValue::Messages(ms)) if ms.iter().any(|m| !m.is_initialized()) => {
                    return false
                }
                _ => {}
            }
        }
        true
    }

    /// `Builder.mergeFrom(other)`: set scalars overwrite, singular messages merge,
    /// repeated fields concatenate. Extension messages are rebuilt with `build()`.
    /// (Generated `mergeFrom` visits fields in declaration order; the only
    /// builder-less nested merge reachable from a parse is `JvmPropertySignature.field`,
    /// its first field, so number order throws at the same point.)
    pub fn merge_from(&mut self, other: &Message) {
        for (slot, spec) in self.message_type.fields().iter().enumerate() {
            let Some(theirs) = &other.values[slot] else {
                continue;
            };
            let merged = match (self.values[slot].take(), theirs) {
                (Some(FieldValue::Message(mine)), FieldValue::Message(theirs)) => {
                    if !spec.extension {
                        mine.message_type.new_builder();
                    }
                    let mut m = (*mine).clone();
                    m.merge_from(theirs);
                    if spec.extension && !m.is_initialized() {
                        uninitialized_message();
                    }
                    FieldValue::Message(Rc::new(m))
                }
                (Some(FieldValue::Scalars(mut mine)), FieldValue::Scalars(theirs)) => {
                    mine.extend_from_slice(theirs);
                    FieldValue::Scalars(mine)
                }
                (Some(FieldValue::Messages(mut mine)), FieldValue::Messages(theirs)) => {
                    mine.extend_from_slice(theirs);
                    FieldValue::Messages(mine)
                }
                (_, theirs) => theirs.clone(),
            };
            self.values[slot] = Some(merged);
        }
    }

    fn push_scalar(&mut self, slot: usize, v: i64) {
        match &mut self.values[slot] {
            Some(FieldValue::Scalars(list)) => list.push(v),
            other => *other = Some(FieldValue::Scalars(vec![v])),
        }
    }

    fn push_message(&mut self, slot: usize, m: Message) {
        match &mut self.values[slot] {
            Some(FieldValue::Messages(list)) => list.push(Rc::new(m)),
            other => *other = Some(FieldValue::Messages(vec![Rc::new(m)])),
        }
    }

    fn set_or_merge_message(&mut self, slot: usize, m: Message) {
        let value = match self.values[slot].take() {
            Some(FieldValue::Message(existing)) => {
                let mut merged = (*existing).clone();
                merged.merge_from(&m);
                merged
            }
            _ => m,
        };
        self.values[slot] = Some(FieldValue::Message(Rc::new(value)));
    }

    /// The generated `X(CodedInputStream, ExtensionRegistryLite)` constructor
    /// (`Parser.parsePartialFrom`), with every `JvmProtoBuf` extension registered.
    pub fn parse_partial_from(input: &mut CodedInputStream, message_type: MsgType) -> Message {
        let mut message = Message::new(message_type);
        message.parse_into(input);
        message
    }

    /// The parse loop, building `self` in place (what Java's unfinished message holds on a throw).
    fn parse_into(&mut self, input: &mut CodedInputStream) {
        let fields = self.message_type.fields();
        let message = self;
        loop {
            let tag = input.read_tag();
            if tag == 0 {
                break;
            }
            let number = get_tag_field_number(tag);
            let wire_type = get_tag_wire_type(tag);
            let known = fields
                .iter()
                .position(|f| f.number == number && !f.extension);
            match known {
                Some(slot) if wire_type == fields[slot].kind.wire_type() => {
                    message.read_field(input, slot);
                }
                Some(slot)
                    if wire_type == 2
                        && fields[slot].repeated
                        && fields[slot].kind.is_packable() =>
                {
                    message.read_packed(input, slot);
                }
                _ => {
                    if !message.parse_unknown_field(input, tag) {
                        break;
                    }
                }
            }
        }
    }

    fn read_field(&mut self, input: &mut CodedInputStream, slot: usize) {
        let spec = &self.message_type.fields()[slot];
        let value = match spec.kind {
            Int32 => input.read_int32() as i64,
            SInt64 => input.read_sint64(),
            Bool => input.read_bool() as i64,
            Float => input.read_raw_little_endian32() as u32 as i64,
            Double => input.read_raw_little_endian64(),
            Enum(count) => {
                let raw = input.read_enum();
                if raw < 0 || raw >= count {
                    return;
                }
                raw as i64
            }
            Bytes => {
                let bytes = input.read_bytes();
                self.values[slot] = Some(FieldValue::Bytes(bytes));
                return;
            }
            Msg(t) => {
                if !spec.repeated && self.values[slot].is_some() {
                    t.to_builder();
                }
                let m = read_message(input, t);
                if spec.repeated {
                    self.push_message(slot, m);
                } else {
                    self.set_or_merge_message(slot, m);
                }
                return;
            }
        };
        if spec.repeated {
            self.push_scalar(slot, value);
        } else {
            self.values[slot] = Some(FieldValue::Scalar(value));
        }
    }

    fn read_packed(&mut self, input: &mut CodedInputStream, slot: usize) {
        let length = input.read_raw_varint32();
        let limit = input.push_limit(length);
        while input.get_bytes_until_limit() > 0 {
            let v = input.read_int32() as i64;
            self.push_scalar(slot, v);
        }
        input.pop_limit(limit);
    }

    /// `GeneratedMessageLite.parseUnknownField`: a registered extension, or a skip.
    fn parse_unknown_field(&mut self, input: &mut CodedInputStream, tag: i32) -> bool {
        let wire_type = get_tag_wire_type(tag);
        let number = get_tag_field_number(tag);
        let fields = self.message_type.fields();
        let Some(slot) = fields
            .iter()
            .position(|f| f.number == number && f.extension)
        else {
            return input.skip_field(tag);
        };
        let spec = &fields[slot];
        if wire_type != spec.kind.wire_type() {
            return input.skip_field(tag);
        }
        match spec.kind {
            Msg(t) => {
                let mut built = match (&self.values[slot], spec.repeated) {
                    (Some(FieldValue::Message(existing)), false) => {
                        existing.message_type.to_builder();
                        (**existing).clone()
                    }
                    _ => Message::new(t),
                };
                read_message_into_builder(input, &mut built);
                if !built.is_initialized() {
                    uninitialized_message();
                }
                if spec.repeated {
                    self.push_message(slot, built);
                } else {
                    self.values[slot] = Some(FieldValue::Message(Rc::new(built)));
                }
            }
            Int32 => {
                let v = input.read_int32() as i64;
                self.values[slot] = Some(FieldValue::Scalar(v));
            }
            Bool => {
                let v = input.read_bool() as i64;
                self.values[slot] = Some(FieldValue::Scalar(v));
            }
            _ => unreachable!("extension kinds are int32, bool and messages"),
        }
        true
    }

    /// `AbstractParser.parseFrom(InputStream, ExtensionRegistryLite)`.
    pub fn parse_from(input: &mut dyn JavaInputStream, message_type: MsgType) -> Message {
        let message = Message::parse_partial_from_stream(input, message_type);
        if !message.is_initialized() {
            invalid_protocol_buffer();
        }
        message
    }

    /// `AbstractParser.parsePartialFrom(InputStream, ExtensionRegistryLite)`.
    pub fn parse_partial_from_stream(
        input: &mut dyn JavaInputStream,
        message_type: MsgType,
    ) -> Message {
        let mut coded = CodedInputStream::new_instance(input);
        let message = Message::parse_partial_from(&mut coded, message_type);
        coded.check_last_tag_was(0);
        message
    }

    /// `AbstractParser.parseDelimitedFrom`: `None` when the stream is already at its end.
    pub fn parse_delimited_from(
        input: &mut dyn JavaInputStream,
        message_type: MsgType,
    ) -> Option<Message> {
        let first_byte = input.read();
        if first_byte == -1 {
            return None;
        }
        let size = CodedInputStream::read_raw_varint32_from(first_byte, input);
        let mut limited = LimitedInputStream::new(input, size);
        let message = Message::parse_partial_from_stream(&mut limited, message_type);
        if !message.is_initialized() {
            invalid_protocol_buffer();
        }
        Some(message)
    }
}

/// `CodedInputStream.readMessage(Parser, ExtensionRegistryLite)`.
fn read_message(input: &mut CodedInputStream, message_type: MsgType) -> Message {
    let length = input.read_raw_varint32();
    input.check_recursion_limit();
    let old_limit = input.push_limit(length);
    input.recursion_depth += 1;
    let message = Message::parse_partial_from(input, message_type);
    input.check_last_tag_was(0);
    input.recursion_depth -= 1;
    input.pop_limit(old_limit);
    message
}

/// `CodedInputStream.readMessage(Builder, ExtensionRegistryLite)`: the builder's
/// `mergeFrom(CodedInputStream)` merges what was parsed, even the unfinished
/// message of an `InvalidProtocolBufferException`, before anything else is checked.
fn read_message_into_builder(input: &mut CodedInputStream, builder: &mut Message) {
    let length = input.read_raw_varint32();
    input.check_recursion_limit();
    let old_limit = input.push_limit(length);
    input.recursion_depth += 1;
    let mut parsed = Message::new(builder.message_type);
    let outcome = try_catch(|| parsed.parse_into(input));
    if let Err(caught) = outcome {
        if caught.is("InvalidProtocolBufferException") {
            builder.merge_from(&parsed);
        }
        rethrow(caught);
    }
    builder.merge_from(&parsed);
    input.check_last_tag_was(0);
    input.recursion_depth -= 1;
    input.pop_limit(old_limit);
}

/// `List.get(int)` on a generated message's list (`Collections.emptyList()` when never set).
fn list_get<T>(list: &[T], index: i32) -> &T {
    if list.is_empty() {
        throw("IndexOutOfBoundsException", format!("Index: {}", index));
    }
    if index < 0 || index as usize >= list.len() {
        throw(
            "IndexOutOfBoundsException",
            format!("Index {} out of bounds for length {}", index, list.len()),
        );
    }
    &list[index as usize]
}

macro_rules! proto_enum {
    ($(#[$meta:meta])* $name:ident { $($variant:ident = $number:literal => $java:literal),* $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
        pub enum $name {
            $($variant),*
        }

        impl $name {
            pub const VALUES: &'static [$name] = &[$($name::$variant),*];

            /// `valueOf(int)`.
            pub fn value_of(value: i32) -> Option<$name> {
                match value {
                    $($number => Some($name::$variant),)*
                    _ => None,
                }
            }

            pub fn get_number(self) -> i32 {
                match self {
                    $($name::$variant => $number),*
                }
            }

            /// `name()`: the Java constant.
            pub fn name(self) -> &'static str {
                match self {
                    $($name::$variant => $java),*
                }
            }
        }
    };
}

proto_enum!(
    /// `ProtoBuf.Visibility`.
    Visibility {
        Internal = 0 => "INTERNAL",
        Private = 1 => "PRIVATE",
        Protected = 2 => "PROTECTED",
        Public = 3 => "PUBLIC",
        PrivateToThis = 4 => "PRIVATE_TO_THIS",
        Local = 5 => "LOCAL",
    }
);

proto_enum!(
    /// `ProtoBuf.Modality`.
    Modality {
        Final = 0 => "FINAL",
        Open = 1 => "OPEN",
        Abstract = 2 => "ABSTRACT",
        Sealed = 3 => "SEALED",
    }
);

proto_enum!(
    /// `ProtoBuf.Class.Kind`.
    ClassKind {
        Class = 0 => "CLASS",
        Interface = 1 => "INTERFACE",
        EnumClass = 2 => "ENUM_CLASS",
        EnumEntry = 3 => "ENUM_ENTRY",
        AnnotationClass = 4 => "ANNOTATION_CLASS",
        Object = 5 => "OBJECT",
        CompanionObject = 6 => "COMPANION_OBJECT",
    }
);

proto_enum!(
    /// `ProtoBuf.MemberKind`.
    MemberKind {
        Declaration = 0 => "DECLARATION",
        FakeOverride = 1 => "FAKE_OVERRIDE",
        Delegation = 2 => "DELEGATION",
        Synthesized = 3 => "SYNTHESIZED",
    }
);

proto_enum!(
    /// `ProtoBuf.ReturnValueStatus`.
    ReturnValueStatus {
        Unspecified = 0 => "UNSPECIFIED",
        MustUse = 1 => "MUST_USE",
        ExplicitlyIgnorable = 2 => "EXPLICITLY_IGNORABLE",
    }
);

proto_enum!(
    /// `ProtoBuf.Type.Argument.Projection`.
    Projection {
        In = 0 => "IN",
        Out = 1 => "OUT",
        Inv = 2 => "INV",
        Star = 3 => "STAR",
    }
);

proto_enum!(
    /// `ProtoBuf.TypeParameter.Variance`.
    Variance {
        In = 0 => "IN",
        Out = 1 => "OUT",
        Inv = 2 => "INV",
    }
);

proto_enum!(
    /// `ProtoBuf.Effect.EffectType`.
    EffectType {
        ReturnsConstant = 0 => "RETURNS_CONSTANT",
        Calls = 1 => "CALLS",
        ReturnsNotNull = 2 => "RETURNS_NOT_NULL",
    }
);

proto_enum!(
    /// `ProtoBuf.Effect.InvocationKind`.
    InvocationKind {
        AtMostOnce = 0 => "AT_MOST_ONCE",
        ExactlyOnce = 1 => "EXACTLY_ONCE",
        AtLeastOnce = 2 => "AT_LEAST_ONCE",
    }
);

proto_enum!(
    /// `ProtoBuf.Expression.ConstantValue`.
    ConstantValue {
        True = 0 => "TRUE",
        False = 1 => "FALSE",
        Null = 2 => "NULL",
    }
);

proto_enum!(
    /// `JvmProtoBuf.StringTableTypes.Record.Operation` (`NONE`, `INTERNAL_TO_CLASS_ID`,
    /// `DESC_TO_CLASS_ID`); the shading turned all three constant names into `"Code"`.
    Operation {
        None = 0 => "Code",
        InternalToClassId = 1 => "Code",
        DescToClassId = 2 => "Code",
    }
);

macro_rules! message_view {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Debug)]
        pub struct $name(Rc<Message>);

        impl $name {
            pub fn get_default_instance() -> $name {
                $name(Rc::new(Message::new(MsgType::$name)))
            }

            pub fn from_message(message: Rc<Message>) -> $name {
                assert_eq!(message.get_message_type(), MsgType::$name);
                $name(message)
            }

            pub fn message(&self) -> &Rc<Message> {
                &self.0
            }

            /// Java `==`.
            pub fn ptr_eq(&self, other: &$name) -> bool {
                Rc::ptr_eq(&self.0, &other.0)
            }

            pub fn is_initialized(&self) -> bool {
                self.0.is_initialized()
            }
        }
    };
}

macro_rules! int_field {
    ($has:ident, $get:ident, $number:literal, $default:expr) => {
        pub fn $has(&self) -> bool {
            self.0.has($number)
        }

        pub fn $get(&self) -> i32 {
            self.0.scalar($number).map_or($default, |v| v as i32)
        }
    };
}

macro_rules! bool_field {
    ($has:ident, $get:ident, $number:literal) => {
        pub fn $has(&self) -> bool {
            self.0.has($number)
        }

        pub fn $get(&self) -> bool {
            self.0.scalar($number).is_some_and(|v| v != 0)
        }
    };
}

macro_rules! enum_field {
    ($has:ident, $get:ident, $number:literal, $enum:ident :: $default:ident) => {
        pub fn $has(&self) -> bool {
            self.0.has($number)
        }

        pub fn $get(&self) -> $enum {
            self.0.scalar($number).map_or($enum::$default, |v| {
                $enum::value_of(v as i32).expect("stored enum numbers are valid")
            })
        }
    };
}

macro_rules! message_field {
    ($has:ident, $get:ident, $number:literal, $view:ident) => {
        pub fn $has(&self) -> bool {
            self.0.has($number)
        }

        pub fn $get(&self) -> $view {
            match self.0.message($number) {
                Some(m) => $view(m.clone()),
                None => $view::get_default_instance(),
            }
        }
    };
}

macro_rules! message_list {
    ($list:ident, $count:ident, $get:ident, $number:literal, $view:ident) => {
        pub fn $list(&self) -> Vec<$view> {
            self.0
                .messages($number)
                .iter()
                .map(|m| $view(m.clone()))
                .collect()
        }

        pub fn $count(&self) -> i32 {
            self.0.messages($number).len() as i32
        }

        pub fn $get(&self, index: i32) -> $view {
            $view(list_get(self.0.messages($number), index).clone())
        }
    };
}

macro_rules! int_list {
    ($list:ident, $count:ident, $number:literal) => {
        pub fn $list(&self) -> Vec<i32> {
            self.0.scalars($number).iter().map(|v| *v as i32).collect()
        }

        pub fn $count(&self) -> i32 {
            self.0.scalars($number).len() as i32
        }
    };
}

message_view!(
    /// `ProtoBuf.Class`.
    Class
);
message_view!(
    /// `ProtoBuf.Package`.
    Package
);
message_view!(
    /// `ProtoBuf.Function`.
    Function
);
message_view!(
    /// `ProtoBuf.Property`.
    Property
);
message_view!(
    /// `ProtoBuf.Constructor`.
    Constructor
);
message_view!(
    /// `ProtoBuf.ValueParameter`.
    ValueParameter
);
message_view!(
    /// `ProtoBuf.Type`.
    Type
);
message_view!(
    /// `ProtoBuf.Type.Argument`.
    TypeArgument
);
message_view!(
    /// `ProtoBuf.TypeParameter`.
    TypeParameter
);
message_view!(
    /// `ProtoBuf.TypeTable`.
    TypeTable
);
message_view!(
    /// `ProtoBuf.TypeAlias`.
    TypeAlias
);
message_view!(
    /// `ProtoBuf.Contract`.
    Contract
);
message_view!(
    /// `ProtoBuf.Effect`.
    Effect
);
message_view!(
    /// `ProtoBuf.Expression`.
    Expression
);
message_view!(
    /// `JvmProtoBuf.JvmMethodSignature`.
    JvmMethodSignature
);
message_view!(
    /// `JvmProtoBuf.JvmFieldSignature`.
    JvmFieldSignature
);
message_view!(
    /// `JvmProtoBuf.JvmPropertySignature`.
    JvmPropertySignature
);
message_view!(
    /// `JvmProtoBuf.StringTableTypes`.
    StringTableTypes
);
message_view!(
    /// `JvmProtoBuf.StringTableTypes.Record`.
    Record
);

impl Class {
    /// Java: `parseFrom(InputStream, ExtensionRegistryLite)`
    pub fn parse_from(input: &mut dyn JavaInputStream) -> Class {
        Class(Rc::new(Message::parse_from(input, MsgType::Class)))
    }

    int_field!(has_flags, get_flags, 1, 6);
    int_field!(has_fq_name, get_fq_name, 3, 0);
    int_field!(has_companion_object_name, get_companion_object_name, 4, 0);
    message_list!(
        get_supertype_list,
        get_supertype_count,
        get_supertype,
        6,
        Type
    );
    message_list!(
        get_constructor_list,
        get_constructor_count,
        get_constructor,
        8,
        Constructor
    );
    message_list!(
        get_function_list,
        get_function_count,
        get_function,
        9,
        Function
    );
    message_list!(
        get_property_list,
        get_property_count,
        get_property,
        10,
        Property
    );
    message_list!(
        get_type_alias_list,
        get_type_alias_count,
        get_type_alias,
        11,
        TypeAlias
    );
    message_field!(has_type_table, get_type_table, 30, TypeTable);
    int_field!(
        has_extension_class_module_name,
        get_extension_class_module_name,
        101,
        0
    );
    message_list!(
        get_extension_class_local_variable_list,
        get_extension_class_local_variable_count,
        get_extension_class_local_variable,
        102,
        Property
    );
    int_field!(
        has_extension_anonymous_object_origin_name,
        get_extension_anonymous_object_origin_name,
        103,
        0
    );
    int_field!(
        has_extension_jvm_class_flags,
        get_extension_jvm_class_flags,
        104,
        0
    );
}

impl Package {
    /// Java: `parseFrom(InputStream, ExtensionRegistryLite)`
    pub fn parse_from(input: &mut dyn JavaInputStream) -> Package {
        Package(Rc::new(Message::parse_from(input, MsgType::Package)))
    }

    message_list!(
        get_function_list,
        get_function_count,
        get_function,
        3,
        Function
    );
    message_list!(
        get_property_list,
        get_property_count,
        get_property,
        4,
        Property
    );
    message_list!(
        get_type_alias_list,
        get_type_alias_count,
        get_type_alias,
        5,
        TypeAlias
    );
    message_field!(has_type_table, get_type_table, 30, TypeTable);
    int_field!(
        has_extension_package_module_name,
        get_extension_package_module_name,
        101,
        0
    );
    message_list!(
        get_extension_package_local_variable_list,
        get_extension_package_local_variable_count,
        get_extension_package_local_variable,
        102,
        Property
    );
}

impl Function {
    /// Java: `parseFrom(InputStream, ExtensionRegistryLite)`
    pub fn parse_from(input: &mut dyn JavaInputStream) -> Function {
        Function(Rc::new(Message::parse_from(input, MsgType::Function)))
    }

    int_field!(has_flags, get_flags, 9, 6);
    int_field!(has_name, get_name, 2, 0);
    message_field!(has_return_type, get_return_type, 3, Type);
    message_list!(
        get_type_parameter_list,
        get_type_parameter_count,
        get_type_parameter,
        4,
        TypeParameter
    );
    message_field!(has_receiver_type, get_receiver_type, 5, Type);
    message_list!(
        get_value_parameter_list,
        get_value_parameter_count,
        get_value_parameter,
        6,
        ValueParameter
    );
    message_list!(
        get_context_receiver_type_list,
        get_context_receiver_type_count,
        get_context_receiver_type,
        10,
        Type
    );
    message_field!(has_type_table, get_type_table, 30, TypeTable);
    message_field!(has_contract, get_contract, 32, Contract);
    message_field!(
        has_extension_method_signature,
        get_extension_method_signature,
        100,
        JvmMethodSignature
    );
    int_field!(
        has_extension_lambda_class_origin_name,
        get_extension_lambda_class_origin_name,
        101,
        0
    );
}

impl Property {
    int_field!(has_flags, get_flags, 11, 518);
    int_field!(has_name, get_name, 2, 0);
    message_field!(has_return_type, get_return_type, 3, Type);
    message_field!(
        has_setter_value_parameter,
        get_setter_value_parameter,
        6,
        ValueParameter
    );
    int_field!(has_getter_flags, get_getter_flags, 7, 0);
    int_field!(has_setter_flags, get_setter_flags, 8, 0);
    message_field!(
        has_extension_property_signature,
        get_extension_property_signature,
        100,
        JvmPropertySignature
    );
    int_field!(has_extension_flags, get_extension_flags, 101, 0);
}

impl Constructor {
    int_field!(has_flags, get_flags, 1, 6);
    message_list!(
        get_value_parameter_list,
        get_value_parameter_count,
        get_value_parameter,
        2,
        ValueParameter
    );
    message_field!(
        has_extension_constructor_signature,
        get_extension_constructor_signature,
        100,
        JvmMethodSignature
    );
}

impl ValueParameter {
    int_field!(has_flags, get_flags, 1, 0);
    int_field!(has_name, get_name, 2, 0);
    message_field!(has_type, get_type, 3, Type);
    message_field!(has_vararg_element_type, get_vararg_element_type, 4, Type);
    int_field!(has_type_id, get_type_id, 5, 0);
}

impl Type {
    message_list!(
        get_argument_list,
        get_argument_count,
        get_argument,
        2,
        TypeArgument
    );
    bool_field!(has_nullable, get_nullable, 3);
    int_field!(has_class_name, get_class_name, 6, 0);
    int_field!(has_type_parameter_name, get_type_parameter_name, 9, 0);
    int_field!(has_type_alias_name, get_type_alias_name, 12, 0);
    bool_field!(has_extension_is_raw, get_extension_is_raw, 101);
}

impl TypeArgument {
    enum_field!(has_projection, get_projection, 1, Projection::Inv);
    message_field!(has_type, get_type, 2, Type);
    int_field!(has_type_id, get_type_id, 3, 0);
}

impl TypeParameter {
    int_field!(has_id, get_id, 1, 0);
    int_field!(has_name, get_name, 2, 0);
    bool_field!(has_reified, get_reified, 3);
    enum_field!(has_variance, get_variance, 4, Variance::Inv);
    message_list!(
        get_upper_bound_list,
        get_upper_bound_count,
        get_upper_bound,
        5,
        Type
    );
}

impl TypeTable {
    message_list!(get_type_list, get_type_count, get_type, 1, Type);
    int_field!(has_first_nullable, get_first_nullable, 2, -1);
}

impl TypeAlias {
    int_field!(has_name, get_name, 2, 0);
    message_field!(has_underlying_type, get_underlying_type, 4, Type);
}

impl Contract {
    message_list!(get_effect_list, get_effect_count, get_effect, 1, Effect);
}

impl Effect {
    enum_field!(
        has_effect_type,
        get_effect_type,
        1,
        EffectType::ReturnsConstant
    );
    message_list!(
        get_effect_constructor_argument_list,
        get_effect_constructor_argument_count,
        get_effect_constructor_argument,
        2,
        Expression
    );
    message_field!(
        has_conclusion_of_conditional_effect,
        get_conclusion_of_conditional_effect,
        3,
        Expression
    );
    enum_field!(has_kind, get_kind, 4, InvocationKind::AtMostOnce);
}

impl Expression {
    int_field!(has_flags, get_flags, 1, 0);
    int_field!(
        has_value_parameter_reference,
        get_value_parameter_reference,
        2,
        0
    );
    enum_field!(
        has_constant_value,
        get_constant_value,
        3,
        ConstantValue::True
    );
    message_field!(has_is_instance_type, get_is_instance_type, 4, Type);
    int_field!(has_is_instance_type_id, get_is_instance_type_id, 5, 0);
    message_list!(
        get_and_argument_list,
        get_and_argument_count,
        get_and_argument,
        6,
        Expression
    );
    message_list!(
        get_or_argument_list,
        get_or_argument_count,
        get_or_argument,
        7,
        Expression
    );
}

impl JvmMethodSignature {
    int_field!(has_name, get_name, 1, 0);
    int_field!(has_desc, get_desc, 2, 0);
}

impl JvmFieldSignature {
    int_field!(has_name, get_name, 1, 0);
    int_field!(has_desc, get_desc, 2, 0);
}

impl JvmPropertySignature {
    message_field!(has_field, get_field, 1, JvmFieldSignature);
    message_field!(
        has_synthetic_method,
        get_synthetic_method,
        2,
        JvmMethodSignature
    );
    message_field!(has_getter, get_getter, 3, JvmMethodSignature);
    message_field!(has_setter, get_setter, 4, JvmMethodSignature);
}

impl StringTableTypes {
    /// Java: `parseDelimitedFrom(InputStream, ExtensionRegistryLite)`; `None` is Java's `null`.
    pub fn parse_delimited_from(input: &mut dyn JavaInputStream) -> Option<StringTableTypes> {
        Message::parse_delimited_from(input, MsgType::StringTableTypes)
            .map(|m| StringTableTypes(Rc::new(m)))
    }

    message_list!(get_record_list, get_record_count, get_record, 1, Record);
}

impl Record {
    int_field!(has_range, get_range, 1, 1);
    int_field!(has_predefined_index, get_predefined_index, 2, 0);
    enum_field!(has_operation, get_operation, 3, Operation::None);
    int_list!(get_substring_index_list, get_substring_index_count, 4);
    int_list!(get_replace_char_list, get_replace_char_count, 5);

    pub fn has_string(&self) -> bool {
        self.0.has(6)
    }

    /// The shaded `ByteString.toStringUtf8()` decodes with the charset `"Code"`,
    /// so a set string throws; an unset one is the default `""`.
    pub fn get_string(&self) -> String {
        if self.has_string() {
            throw("RuntimeException", "Code");
        }
        String::new()
    }
}

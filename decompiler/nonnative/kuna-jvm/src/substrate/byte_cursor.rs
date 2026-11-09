//! Replaces util/DataInputFullStream: a big-endian cursor over class-file bytes
//! with `DataInputStream`'s method names. Reading past the end throws
//! `IOException`, as `EOFException` does in Java.

use std::rc::Rc;
use std::sync::Arc;

use crate::jthrow;
use crate::rt::jstring::decode_modified_utf8;

pub struct ByteCursor {
    data: Bytes,
    pos: usize,
}

#[derive(Clone)]
enum Bytes {
    Local(Rc<[u8]>),
    Shared(Arc<[u8]>),
}

impl std::ops::Deref for Bytes {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        match self {
            Bytes::Local(b) => b,
            Bytes::Shared(b) => b,
        }
    }
}

impl ByteCursor {
    pub fn new(bytes: Vec<u8>) -> ByteCursor {
        ByteCursor {
            data: Bytes::Local(Rc::from(bytes)),
            pos: 0,
        }
    }

    /// A cursor over bytes shared with their owner, without copying them.
    pub fn from_shared(bytes: Rc<[u8]>) -> ByteCursor {
        ByteCursor {
            data: Bytes::Local(bytes),
            pos: 0,
        }
    }

    /// A cursor over bytes other threads share, without copying them.
    pub fn from_arc(bytes: Arc<[u8]>) -> ByteCursor {
        ByteCursor {
            data: Bytes::Shared(bytes),
            pos: 0,
        }
    }

    /// Another cursor over the same bytes, at `pos`.
    pub fn share_at(&self, pos: usize) -> ByteCursor {
        ByteCursor {
            data: self.data.clone(),
            pos,
        }
    }

    /// Every byte, wherever the cursor is.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    fn take(&mut self, n: usize) -> &[u8] {
        if self.remaining() < n {
            jthrow!(IOException; "EOF");
        }
        let start = self.pos;
        self.pos += n;
        &self.data[start..self.pos]
    }

    fn take_array<const N: usize>(&mut self) -> [u8; N] {
        let mut out = [0u8; N];
        out.copy_from_slice(self.take(N));
        out
    }

    pub fn read_unsigned_byte(&mut self) -> i32 {
        self.take_array::<1>()[0] as i32
    }

    pub fn read_byte(&mut self) -> i8 {
        self.take_array::<1>()[0] as i8
    }

    pub fn read_boolean(&mut self) -> bool {
        self.read_unsigned_byte() != 0
    }

    pub fn read_unsigned_short(&mut self) -> i32 {
        u16::from_be_bytes(self.take_array()) as i32
    }

    pub fn read_short(&mut self) -> i16 {
        i16::from_be_bytes(self.take_array())
    }

    pub fn read_char(&mut self) -> u16 {
        u16::from_be_bytes(self.take_array())
    }

    pub fn read_int(&mut self) -> i32 {
        i32::from_be_bytes(self.take_array())
    }

    pub fn read_long(&mut self) -> i64 {
        i64::from_be_bytes(self.take_array())
    }

    pub fn read_float(&mut self) -> f32 {
        f32::from_bits(u32::from_be_bytes(self.take_array()))
    }

    pub fn read_double(&mut self) -> f64 {
        f64::from_bits(u64::from_be_bytes(self.take_array()))
    }

    /// `readUTF()`: a u2 length followed by that many bytes of modified UTF-8.
    pub fn read_utf(&mut self) -> String {
        let len = self.read_unsigned_short() as usize;
        decode_modified_utf8(self.take(len))
    }

    /// `readFully(byte[])`.
    pub fn read_fully(&mut self, buf: &mut [u8]) {
        let n = buf.len();
        buf.copy_from_slice(self.take(n));
    }

    /// `read(n)`: exactly `n` bytes, or `IOException`.
    pub fn read(&mut self, n: usize) -> Vec<u8> {
        if self.remaining() < n {
            jthrow!(IOException; "premature end of stream");
        }
        self.take(n).to_vec()
    }

    /// `discard(n)`: skip exactly `n` bytes, or `IOException`.
    pub fn discard(&mut self, n: usize) {
        if self.remaining() < n {
            jthrow!(IOException; "premature end of stream");
        }
        self.pos += n;
    }

    /// `skipBytes(n)`: skips up to `n` bytes without failing at the end; returns the count.
    pub fn skip_bytes(&mut self, n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }
        let k = (n as usize).min(self.remaining());
        self.pos += k;
        k as i32
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }
}

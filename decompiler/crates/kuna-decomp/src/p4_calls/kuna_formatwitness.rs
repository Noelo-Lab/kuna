//! P4 matching-format evidence for partial call argument recovery.
//!
//! [`kuna_calleearitylive`](crate::p4_calls::kuna_calleearitylive) normally
//! requires the callee body to prove a sibling's longer argument list.  That
//! proof deliberately declines variadic callees, whose register-save prologue
//! reads every argument register.  A stripped `printf` therefore leaves this
//! common shape unresolved:
//!
//! ```text
//! printf("position=(%d,%d)\n", old_x, old_y);
//! state->x = new_x;
//! state->y = new_y;
//! printf("position=(%d,%d)\n", new_x, new_y);
//! ```
//!
//! The stores make `onlyOpUse` reject `new_x` and `new_y`, even though the
//! assembly passes them.  The matching nonzero constant format pointer and its
//! parsed conversion count give the missing discriminator: another finalized
//! call to the same entry with exactly that many arguments is an arity witness.
//! The parser accepts a conservative printf subset and declines positional or
//! malformed formats.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::OpId;
use crate::funcdata::Funcdata;
use crate::p4_calls::kuna_calleearity::witness_storage;

const MAX_FORMAT: usize = 256;

fn constant_pointer(data: &Funcdata, op: OpId) -> Option<Address> {
    let call = data.obank().get(op)?;
    let vn = data.vbank().get(call.get_in(1)?)?;
    if !vn.is_constant() || vn.get_offset() == 0 {
        return None;
    }
    let glb = data.get_arch();
    let space = Rc::clone(glb.manage().get_default_data_space()?);
    let point = call.get_addr();
    let mut full_encoding = 0;
    let resolved = glb
        .resolve_constant(
            &space,
            vn.get_offset(),
            vn.get_size(),
            point,
            &mut full_encoding,
        )
        .ok()?;
    (!resolved.is_invalid()).then_some(resolved)
}

fn read_format(data: &Funcdata, addr: &Address) -> Option<Vec<u8>> {
    let mut bytes = Vec::new();
    for offset in 0..MAX_FORMAT {
        let mut byte = [0u8; 1];
        let at_offset = addr.get_offset().checked_add(offset as u64)?;
        let at = Address::new(Rc::clone(addr.get_space()?), at_offset);
        data.get_arch().loader_fill(&mut byte, &at).ok()?;
        if byte[0] == 0 {
            return (!bytes.is_empty()).then_some(bytes);
        }
        if !(byte[0].is_ascii_graphic() || byte[0].is_ascii_whitespace()) {
            return None;
        }
        bytes.push(byte[0]);
    }
    None
}

fn take_digits(bytes: &[u8], mut i: usize) -> usize {
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    i
}

pub(crate) fn printf_argument_count(bytes: &[u8]) -> Option<usize> {
    let mut count = 0usize;
    let mut i = 0usize;
    let mut saw_conversion = false;
    while i < bytes.len() {
        if bytes[i] != b'%' {
            i += 1;
            continue;
        }
        i += 1;
        if i >= bytes.len() {
            return None;
        }
        if bytes[i] == b'%' {
            i += 1;
            continue;
        }
        saw_conversion = true;
        while i < bytes.len() && b"#0- +'I".contains(&bytes[i]) {
            i += 1;
        }
        if i < bytes.len() && bytes[i] == b'*' {
            count += 1;
            i += 1;
            let end = take_digits(bytes, i);
            if end < bytes.len() && bytes[end] == b'$' {
                return None;
            }
        } else {
            let end = take_digits(bytes, i);
            if end < bytes.len() && bytes[end] == b'$' {
                return None;
            }
            i = end;
        }
        if i < bytes.len() && bytes[i] == b'.' {
            i += 1;
            if i < bytes.len() && bytes[i] == b'*' {
                count += 1;
                i += 1;
                let end = take_digits(bytes, i);
                if end < bytes.len() && bytes[end] == b'$' {
                    return None;
                }
            } else {
                i = take_digits(bytes, i);
            }
        }
        if i + 1 < bytes.len()
            && ((bytes[i] == b'h' && bytes[i + 1] == b'h')
                || (bytes[i] == b'l' && bytes[i + 1] == b'l'))
        {
            i += 2;
        } else if i < bytes.len() && b"hljztL".contains(&bytes[i]) {
            i += 1;
        }
        let conversion = *bytes.get(i)?;
        i += 1;
        match conversion {
            b'd' | b'i' | b'o' | b'u' | b'x' | b'X' | b'f' | b'F' | b'e' | b'E' | b'g' | b'G'
            | b'a' | b'A' | b'c' | b's' | b'p' | b'n' => count += 1,
            b'm' => {}
            _ => return None,
        }
    }
    saw_conversion.then_some(count)
}

pub(crate) fn best_witness(entry: &Address, op: OpId, data: &Funcdata) -> Vec<(Address, int4)> {
    let Some(format_addr) = constant_pointer(data, op) else {
        return Vec::new();
    };
    let Some(format) = read_format(data, &format_addr) else {
        return Vec::new();
    };
    let Some(varargs) = printf_argument_count(&format) else {
        return Vec::new();
    };
    let expected = varargs + 1;
    let mut best = Vec::new();
    for i in 0..data.num_calls() {
        let sibling = data.get_call_specs(i);
        if sibling.is_input_active()
            || sibling.get_op() == op
            || sibling.get_entry_address() != entry
        {
            continue;
        }
        let sibling_op = sibling.get_op();
        if data.obank().get(sibling_op).map(|o| o.code()) != Some(OpCode::CPUI_CALL)
            || constant_pointer(data, sibling_op).as_ref() != Some(&format_addr)
        {
            continue;
        }
        let Some(storage) = witness_storage(data, sibling) else {
            continue;
        };
        if storage.len() == expected && storage.len() > best.len() {
            best = storage;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::printf_argument_count;

    #[test]
    fn counts_conversions_and_star_operands() {
        assert_eq!(printf_argument_count(b"position=(%d,%d)\\n"), Some(2));
        assert_eq!(printf_argument_count(b"%*.*s %% %lld"), Some(4));
        assert_eq!(printf_argument_count(b"errno: %m"), Some(0));
    }

    #[test]
    fn declines_positional_and_malformed_formats() {
        assert_eq!(printf_argument_count(b"%2$d"), None);
        assert_eq!(printf_argument_count(b"%q"), None);
        assert_eq!(printf_argument_count(b"tail %"), None);
        assert_eq!(printf_argument_count(b"plain text"), None);
    }
}

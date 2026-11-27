//! kuna: whether the Eclipse compiler (ECJ) wrote a class, read from the order of its constant pool.
//!
//! ECJ's `ClassFile` enters the class's own name first and its superclass's second, each
//! `CONSTANT_Class` right before the `CONSTANT_Utf8` it names, then the names and descriptors of the
//! fields and methods in member order, a method's `Code` attribute name before the constants its
//! code uses. javac generates all method code before it writes the class, so the member references
//! of the code come before `Code` (javac 8 to 12 put them first; javac 13 and later as the code
//! uses them, its own class too); ASM and `kuna-dex` enter a name's `CONSTANT_Utf8` before the
//! `CONSTANT_Class` that names it. A class whose pool has no `Code` or no member reference decides
//! nothing. Docs: `docs/spec/11-jvm.md` section 11.3.

const UTF8: u8 = 1;
const CLASS: u8 = 7;

/// Whether the class file `data`, whose constant pool starts at `pool` and names its class at
/// `this` and its superclass at `sup`, is ECJ's (see the module header).
pub fn written_by_ecj(data: &[u8], pool: usize, this: i32, sup: i32) -> bool {
    if this != 1 || (sup != 0 && sup != 3) {
        return false;
    }
    let u2 = |o: usize| -> Option<usize> {
        Some(((*data.get(o)? as usize) << 8) | *data.get(o + 1)? as usize)
    };
    let Some(count) = u2(pool) else {
        return false;
    };
    let mut at = pool + 2;
    let mut code = None;
    let mut index = 1;
    while index < count {
        let Some(&tag) = data.get(at) else {
            return false;
        };
        let operand = u2(at + 1);
        let header = match (index, sup) {
            (1, _) | (3, 3) => tag == CLASS && operand == Some(index + 1),
            (2, _) | (4, 3) => tag == UTF8,
            _ => true,
        };
        if !header {
            return false;
        }
        let size = match tag {
            UTF8 => {
                let Some(len) = operand else {
                    return false;
                };
                if code.is_none() && data.get(at + 3..at + 3 + len) == Some(b"Code") {
                    code = Some(index);
                }
                3 + len
            }
            9..=11 => return code.is_some(),
            3 | 4 | 12 | 17 | 18 => 5,
            5 | 6 => {
                index += 1;
                9
            }
            15 => 4,
            CLASS | 8 | 16 | 19 | 20 => 3,
            _ => return false,
        };
        at += size;
        index += 1;
    }
    false
}

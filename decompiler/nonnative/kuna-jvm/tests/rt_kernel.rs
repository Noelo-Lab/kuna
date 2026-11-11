//! The object arenas, shared lists, bit sets, strings and exceptions of `rt`.

use kuna_jvm::rt::jstring::decode_modified_utf8;
use kuna_jvm::rt::{reset_class_scope, throw, try_catch, BitSet, JList, JavaStr};
use kuna_jvm::{arena_identity, arena_object, java_subclass, unported};

struct NodeData {
    value: i32,
    next: Option<Node>,
}
arena_object!(Node => NodeData, Class);
arena_identity!(Node);

#[test]
fn arena_handles_have_identity_and_allow_cross_object_access() {
    let a = Node::alloc(NodeData {
        value: 1,
        next: None,
    });
    let b = Node::alloc(NodeData {
        value: 2,
        next: Some(a),
    });
    a.with_mut(|d| d.next = Some(b));
    assert_ne!(a, b);
    let sum = a.with(|d| d.value + d.next.unwrap().with(|n| n.value));
    assert_eq!(sum, 3);
    let made = b.with_mut(|d| {
        d.value = 5;
        Node::alloc(NodeData {
            value: d.value + 1,
            next: None,
        })
    });
    assert_eq!(made.with(|d| d.value), 6);
    let list = JList::from(vec![a, b]);
    assert_eq!(list.index_of(&b), 1);
}

#[test]
fn same_object_reentry_is_reported() {
    let a = Node::alloc(NodeData {
        value: 1,
        next: None,
    });
    let caught = try_catch(|| a.with_mut(|_| a.with(|d| d.value))).unwrap_err();
    assert!(caught.message.contains("re-entered"), "{}", caught.message);
}

#[test]
fn stale_handles_are_reported_after_reset() {
    let a = Node::alloc(NodeData {
        value: 1,
        next: None,
    });
    reset_class_scope();
    let caught = try_catch(|| a.with(|d| d.value)).unwrap_err();
    assert!(caught.message.contains("stale"), "{}", caught.message);
    let b = Node::alloc(NodeData {
        value: 9,
        next: None,
    });
    assert_eq!(b.with(|d| d.value), 9);
}

#[test]
fn shared_lists_alias() {
    let a: JList<i32> = JList::new();
    let b = a.clone();
    a.add(1);
    b.add(2);
    assert_eq!(a.to_vec(), vec![1, 2]);
    assert!(a.ptr_eq(&b));
    let c = a.copy();
    c.add(3);
    assert_eq!(a.len(), 2);
    assert!(a.remove_item(&1));
    assert_eq!(b.to_vec(), vec![2]);
    for x in b.iter() {
        b.add(x + 10);
    }
    assert_eq!(b.to_vec(), vec![2, 12]);
}

#[test]
fn bitset_matches_java() {
    let mut a = BitSet::new();
    for i in [1, 5, 64, 130] {
        a.set(i);
    }
    assert_eq!(a.to_string(), "{1, 5, 64, 130}");
    assert_eq!(a.length(), 131);
    assert_eq!(a.cardinality(), 4);
    assert_eq!(a.next_set_bit(6), 64);
    assert_eq!(a.next_set_bit(131), -1);
    assert_eq!(a.next_clear_bit(64), 65);
    let mut b = BitSet::new();
    b.set(5);
    b.set(200);
    let mut c = a.clone();
    c.and(&b);
    assert_eq!(c.to_string(), "{5}");
    c.or(&b);
    c.and_not(&a);
    assert_eq!(c.to_string(), "{200}");
    c.clear(200);
    assert!(c.is_empty());
    assert_eq!(c, BitSet::new());
}

#[test]
fn strings_keep_utf16_semantics() {
    assert_eq!("abc".jlen(), 3);
    let smile = decode_modified_utf8(&[0xED, 0xA0, 0xBD, 0xED, 0xB8, 0x80, b'x']);
    assert_eq!(smile, "\u{1F600}x");
    assert_eq!(smile.jlen(), 3);
    assert_eq!(smile.jchar_at(2), b'x' as u16);
    let lone = decode_modified_utf8(&[b'a', 0xED, 0xA0, 0xBD, b'b']);
    assert_eq!(lone.utf16(), vec![b'a' as u16, 0xD83D, b'b' as u16]);
    assert_eq!(lone.jsubstring(1, 2).utf16(), vec![0xD83D]);
    assert_eq!(decode_modified_utf8(&[0xC0, 0x80]), "\0");
}

#[test]
fn exceptions_unwind_to_the_catch() {
    let caught = try_catch(|| -> i32 { throw("IllegalStateException", "bad graph") }).unwrap_err();
    assert!(caught.is("IllegalStateException"));
    assert_eq!(caught.message, "IllegalStateException: bad graph");
    let v: Vec<i32> = Vec::new();
    let caught = try_catch(|| v[3]).unwrap_err();
    assert!(caught.exception.is_none());
    assert!(
        caught.message.contains("index out of bounds"),
        "{}",
        caught.message
    );
    assert_eq!(try_catch(|| 7).unwrap(), 7);
}

struct ShapeData {
    name: String,
    kind: ShapeKind,
}
enum ShapeKind {
    Circle(CircleData),
    Square(SquareData),
}
struct CircleData {
    radius: i32,
}
struct SquareData {
    side: i32,
}
arena_object!(Shape => ShapeData, Class);
java_subclass!(Circle: Shape, ShapeKind::Circle(CircleData), is_circle, as_circle, try_as_circle);
java_subclass!(Square: Shape, ShapeKind::Square(SquareData), is_square, as_square, try_as_square);

impl Shape {
    fn get_name(self) -> String {
        self.with(|d| d.name.clone())
    }

    fn area(self) -> i32 {
        if let Some(c) = self.try_as_circle() {
            return Circle::area(c);
        }
        Square::area(self.as_square())
    }
}

impl Circle {
    fn area(self) -> i32 {
        self.d(|d| 3 * d.radius * d.radius)
    }
}

impl Square {
    fn area(self) -> i32 {
        self.d(|d| d.side * d.side)
    }
}

#[test]
fn subclass_views_cast_and_dispatch() {
    let c = Shape::alloc(ShapeData {
        name: "c".into(),
        kind: ShapeKind::Circle(CircleData { radius: 2 }),
    });
    let s = Shape::alloc(ShapeData {
        name: "s".into(),
        kind: ShapeKind::Square(SquareData { side: 3 }),
    });
    assert!(c.is_circle() && !c.is_square());
    assert_eq!((c.area(), s.area()), (12, 9));
    let sq = s.as_square();
    sq.d_mut(|d| d.side = 4);
    assert_eq!(sq.get_name(), "s");
    assert_eq!(Shape::from(sq).area(), 16);
    assert!(c.is_live());
    let caught = try_catch(|| c.as_square()).unwrap_err();
    assert!(caught.is("ClassCastException"));
    let caught = try_catch(|| -> i32 { unported!("IfHelper.mergeAllIfs") }).unwrap_err();
    assert!(caught.is("UnportedError"));
}
